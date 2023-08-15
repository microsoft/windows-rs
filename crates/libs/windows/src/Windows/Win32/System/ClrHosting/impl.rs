#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait IActionOnCLREvent_Impl: Sized {
    fn OnEvent(&self, event: EClrEvent, data: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IActionOnCLREvent {}
impl IActionOnCLREvent_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActionOnCLREvent_Impl, const OFFSET: isize>() -> IActionOnCLREvent_Vtbl {
        unsafe extern "system" fn OnEvent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActionOnCLREvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, event: EClrEvent, data: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnEvent(::core::mem::transmute_copy(&event), ::core::mem::transmute_copy(&data)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnEvent: OnEvent::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IActionOnCLREvent as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait IApartmentCallback_Impl: Sized {
    fn DoCallback(&self, pfunc: usize, pdata: usize) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IApartmentCallback {}
impl IApartmentCallback_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IApartmentCallback_Impl, const OFFSET: isize>() -> IApartmentCallback_Vtbl {
        unsafe extern "system" fn DoCallback<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IApartmentCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfunc: usize, pdata: usize) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DoCallback(::core::mem::transmute_copy(&pfunc), ::core::mem::transmute_copy(&pdata)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), DoCallback: DoCallback::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IApartmentCallback as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait IAppDomainBinding_Impl: Sized {
    fn OnAppDomain(&self, pappdomain: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IAppDomainBinding {}
impl IAppDomainBinding_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAppDomainBinding_Impl, const OFFSET: isize>() -> IAppDomainBinding_Vtbl {
        unsafe extern "system" fn OnAppDomain<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAppDomainBinding_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pappdomain: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnAppDomain(::windows_core::from_raw_borrowed(&pappdomain)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnAppDomain: OnAppDomain::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IAppDomainBinding as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait ICLRAppDomainResourceMonitor_Impl: Sized {
    fn GetCurrentAllocated(&self, dwappdomainid: u32, pbytesallocated: *mut u64) -> ::windows_core::Result<()>;
    fn GetCurrentSurvived(&self, dwappdomainid: u32, pappdomainbytessurvived: *mut u64, ptotalbytessurvived: *mut u64) -> ::windows_core::Result<()>;
    fn GetCurrentCpuTime(&self, dwappdomainid: u32, pmilliseconds: *mut u64) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ICLRAppDomainResourceMonitor {}
impl ICLRAppDomainResourceMonitor_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRAppDomainResourceMonitor_Impl, const OFFSET: isize>() -> ICLRAppDomainResourceMonitor_Vtbl {
        unsafe extern "system" fn GetCurrentAllocated<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRAppDomainResourceMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwappdomainid: u32, pbytesallocated: *mut u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCurrentAllocated(::core::mem::transmute_copy(&dwappdomainid), ::core::mem::transmute_copy(&pbytesallocated)).into()
        }
        unsafe extern "system" fn GetCurrentSurvived<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRAppDomainResourceMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwappdomainid: u32, pappdomainbytessurvived: *mut u64, ptotalbytessurvived: *mut u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCurrentSurvived(::core::mem::transmute_copy(&dwappdomainid), ::core::mem::transmute_copy(&pappdomainbytessurvived), ::core::mem::transmute_copy(&ptotalbytessurvived)).into()
        }
        unsafe extern "system" fn GetCurrentCpuTime<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRAppDomainResourceMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwappdomainid: u32, pmilliseconds: *mut u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCurrentCpuTime(::core::mem::transmute_copy(&dwappdomainid), ::core::mem::transmute_copy(&pmilliseconds)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrentAllocated: GetCurrentAllocated::<Identity, Impl, OFFSET>,
            GetCurrentSurvived: GetCurrentSurvived::<Identity, Impl, OFFSET>,
            GetCurrentCpuTime: GetCurrentCpuTime::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ICLRAppDomainResourceMonitor as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ICLRAssemblyIdentityManager_Impl: Sized {
    fn GetCLRAssemblyReferenceList(&self, ppwzassemblyreferences: *const ::windows_core::PCWSTR, dwnumofreferences: u32) -> ::windows_core::Result<ICLRAssemblyReferenceList>;
    fn GetBindingIdentityFromFile(&self, pwzfilepath: &::windows_core::PCWSTR, dwflags: u32, pwzbuffer: ::windows_core::PWSTR, pcchbuffersize: *mut u32) -> ::windows_core::Result<()>;
    fn GetBindingIdentityFromStream(&self, pstream: ::core::option::Option<&super::Com::IStream>, dwflags: u32, pwzbuffer: ::windows_core::PWSTR, pcchbuffersize: *mut u32) -> ::windows_core::Result<()>;
    fn GetReferencedAssembliesFromFile(&self, pwzfilepath: &::windows_core::PCWSTR, dwflags: u32, pexcludeassemblieslist: ::core::option::Option<&ICLRAssemblyReferenceList>) -> ::windows_core::Result<ICLRReferenceAssemblyEnum>;
    fn GetReferencedAssembliesFromStream(&self, pstream: ::core::option::Option<&super::Com::IStream>, dwflags: u32, pexcludeassemblieslist: ::core::option::Option<&ICLRAssemblyReferenceList>) -> ::windows_core::Result<ICLRReferenceAssemblyEnum>;
    fn GetProbingAssembliesFromReference(&self, dwmachinetype: u32, dwflags: u32, pwzreferenceidentity: &::windows_core::PCWSTR) -> ::windows_core::Result<ICLRProbingAssemblyEnum>;
    fn IsStronglyNamed(&self, pwzassemblyidentity: &::windows_core::PCWSTR) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for ICLRAssemblyIdentityManager {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ICLRAssemblyIdentityManager_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRAssemblyIdentityManager_Impl, const OFFSET: isize>() -> ICLRAssemblyIdentityManager_Vtbl {
        unsafe extern "system" fn GetCLRAssemblyReferenceList<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRAssemblyIdentityManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwzassemblyreferences: *const ::windows_core::PCWSTR, dwnumofreferences: u32, ppreferencelist: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCLRAssemblyReferenceList(::core::mem::transmute_copy(&ppwzassemblyreferences), ::core::mem::transmute_copy(&dwnumofreferences)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppreferencelist, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBindingIdentityFromFile<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRAssemblyIdentityManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzfilepath: ::windows_core::PCWSTR, dwflags: u32, pwzbuffer: ::windows_core::PWSTR, pcchbuffersize: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetBindingIdentityFromFile(::core::mem::transmute(&pwzfilepath), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pwzbuffer), ::core::mem::transmute_copy(&pcchbuffersize)).into()
        }
        unsafe extern "system" fn GetBindingIdentityFromStream<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRAssemblyIdentityManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void, dwflags: u32, pwzbuffer: ::windows_core::PWSTR, pcchbuffersize: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetBindingIdentityFromStream(::windows_core::from_raw_borrowed(&pstream), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pwzbuffer), ::core::mem::transmute_copy(&pcchbuffersize)).into()
        }
        unsafe extern "system" fn GetReferencedAssembliesFromFile<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRAssemblyIdentityManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzfilepath: ::windows_core::PCWSTR, dwflags: u32, pexcludeassemblieslist: *mut ::core::ffi::c_void, ppreferenceenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetReferencedAssembliesFromFile(::core::mem::transmute(&pwzfilepath), ::core::mem::transmute_copy(&dwflags), ::windows_core::from_raw_borrowed(&pexcludeassemblieslist)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppreferenceenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetReferencedAssembliesFromStream<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRAssemblyIdentityManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void, dwflags: u32, pexcludeassemblieslist: *mut ::core::ffi::c_void, ppreferenceenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetReferencedAssembliesFromStream(::windows_core::from_raw_borrowed(&pstream), ::core::mem::transmute_copy(&dwflags), ::windows_core::from_raw_borrowed(&pexcludeassemblieslist)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppreferenceenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProbingAssembliesFromReference<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRAssemblyIdentityManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmachinetype: u32, dwflags: u32, pwzreferenceidentity: ::windows_core::PCWSTR, ppprobingassemblyenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProbingAssembliesFromReference(::core::mem::transmute_copy(&dwmachinetype), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pwzreferenceidentity)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppprobingassemblyenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsStronglyNamed<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRAssemblyIdentityManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzassemblyidentity: ::windows_core::PCWSTR, pbisstronglynamed: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsStronglyNamed(::core::mem::transmute(&pwzassemblyidentity)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbisstronglynamed, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCLRAssemblyReferenceList: GetCLRAssemblyReferenceList::<Identity, Impl, OFFSET>,
            GetBindingIdentityFromFile: GetBindingIdentityFromFile::<Identity, Impl, OFFSET>,
            GetBindingIdentityFromStream: GetBindingIdentityFromStream::<Identity, Impl, OFFSET>,
            GetReferencedAssembliesFromFile: GetReferencedAssembliesFromFile::<Identity, Impl, OFFSET>,
            GetReferencedAssembliesFromStream: GetReferencedAssembliesFromStream::<Identity, Impl, OFFSET>,
            GetProbingAssembliesFromReference: GetProbingAssembliesFromReference::<Identity, Impl, OFFSET>,
            IsStronglyNamed: IsStronglyNamed::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ICLRAssemblyIdentityManager as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait ICLRAssemblyReferenceList_Impl: Sized {
    fn IsStringAssemblyReferenceInList(&self, pwzassemblyname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn IsAssemblyReferenceInList(&self, pname: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ICLRAssemblyReferenceList {}
impl ICLRAssemblyReferenceList_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRAssemblyReferenceList_Impl, const OFFSET: isize>() -> ICLRAssemblyReferenceList_Vtbl {
        unsafe extern "system" fn IsStringAssemblyReferenceInList<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRAssemblyReferenceList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzassemblyname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsStringAssemblyReferenceInList(::core::mem::transmute(&pwzassemblyname)).into()
        }
        unsafe extern "system" fn IsAssemblyReferenceInList<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRAssemblyReferenceList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsAssemblyReferenceInList(::windows_core::from_raw_borrowed(&pname)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsStringAssemblyReferenceInList: IsStringAssemblyReferenceInList::<Identity, Impl, OFFSET>,
            IsAssemblyReferenceInList: IsAssemblyReferenceInList::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ICLRAssemblyReferenceList as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait ICLRControl_Impl: Sized {
    fn GetCLRManager(&self, riid: *const ::windows_core::GUID, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn SetAppDomainManagerType(&self, pwzappdomainmanagerassembly: &::windows_core::PCWSTR, pwzappdomainmanagertype: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ICLRControl {}
impl ICLRControl_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRControl_Impl, const OFFSET: isize>() -> ICLRControl_Vtbl {
        unsafe extern "system" fn GetCLRManager<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCLRManager(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppobject)).into()
        }
        unsafe extern "system" fn SetAppDomainManagerType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzappdomainmanagerassembly: ::windows_core::PCWSTR, pwzappdomainmanagertype: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAppDomainManagerType(::core::mem::transmute(&pwzappdomainmanagerassembly), ::core::mem::transmute(&pwzappdomainmanagertype)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCLRManager: GetCLRManager::<Identity, Impl, OFFSET>,
            SetAppDomainManagerType: SetAppDomainManagerType::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ICLRControl as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub trait ICLRDebugManager_Impl: Sized {
    fn BeginConnection(&self, dwconnectionid: u32, szconnectionname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetConnectionTasks(&self, id: u32, dwcount: u32, ppclrtask: *const ::core::option::Option<ICLRTask>) -> ::windows_core::Result<()>;
    fn EndConnection(&self, dwconnectionid: u32) -> ::windows_core::Result<()>;
    fn SetDacl(&self, pacl: *const super::super::Security::ACL) -> ::windows_core::Result<()>;
    fn GetDacl(&self) -> ::windows_core::Result<*mut super::super::Security::ACL>;
    fn IsDebuggerAttached(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetSymbolReadingPolicy(&self, policy: ESymbolReadingPolicy) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::windows_core::RuntimeName for ICLRDebugManager {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ICLRDebugManager_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRDebugManager_Impl, const OFFSET: isize>() -> ICLRDebugManager_Vtbl {
        unsafe extern "system" fn BeginConnection<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRDebugManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwconnectionid: u32, szconnectionname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BeginConnection(::core::mem::transmute_copy(&dwconnectionid), ::core::mem::transmute(&szconnectionname)).into()
        }
        unsafe extern "system" fn SetConnectionTasks<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRDebugManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: u32, dwcount: u32, ppclrtask: *const *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetConnectionTasks(::core::mem::transmute_copy(&id), ::core::mem::transmute_copy(&dwcount), ::core::mem::transmute_copy(&ppclrtask)).into()
        }
        unsafe extern "system" fn EndConnection<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRDebugManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwconnectionid: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndConnection(::core::mem::transmute_copy(&dwconnectionid)).into()
        }
        unsafe extern "system" fn SetDacl<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRDebugManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pacl: *const super::super::Security::ACL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDacl(::core::mem::transmute_copy(&pacl)).into()
        }
        unsafe extern "system" fn GetDacl<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRDebugManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pacl: *mut *mut super::super::Security::ACL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDacl() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pacl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDebuggerAttached<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRDebugManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbattached: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsDebuggerAttached() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbattached, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSymbolReadingPolicy<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRDebugManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, policy: ESymbolReadingPolicy) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSymbolReadingPolicy(::core::mem::transmute_copy(&policy)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginConnection: BeginConnection::<Identity, Impl, OFFSET>,
            SetConnectionTasks: SetConnectionTasks::<Identity, Impl, OFFSET>,
            EndConnection: EndConnection::<Identity, Impl, OFFSET>,
            SetDacl: SetDacl::<Identity, Impl, OFFSET>,
            GetDacl: GetDacl::<Identity, Impl, OFFSET>,
            IsDebuggerAttached: IsDebuggerAttached::<Identity, Impl, OFFSET>,
            SetSymbolReadingPolicy: SetSymbolReadingPolicy::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ICLRDebugManager as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICLRDebugging_Impl: Sized {
    fn OpenVirtualProcess(&self, modulebaseaddress: u64, pdatatarget: ::core::option::Option<&::windows_core::IUnknown>, plibraryprovider: ::core::option::Option<&ICLRDebuggingLibraryProvider>, pmaxdebuggersupportedversion: *const CLR_DEBUGGING_VERSION, riidprocess: *const ::windows_core::GUID, ppprocess: *mut ::core::option::Option<::windows_core::IUnknown>, pversion: *mut CLR_DEBUGGING_VERSION, pdwflags: *mut CLR_DEBUGGING_PROCESS_FLAGS) -> ::windows_core::Result<()>;
    fn CanUnloadNow(&self, hmodule: super::super::Foundation::HMODULE) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ICLRDebugging {}
#[cfg(feature = "Win32_Foundation")]
impl ICLRDebugging_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRDebugging_Impl, const OFFSET: isize>() -> ICLRDebugging_Vtbl {
        unsafe extern "system" fn OpenVirtualProcess<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRDebugging_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, modulebaseaddress: u64, pdatatarget: *mut ::core::ffi::c_void, plibraryprovider: *mut ::core::ffi::c_void, pmaxdebuggersupportedversion: *const CLR_DEBUGGING_VERSION, riidprocess: *const ::windows_core::GUID, ppprocess: *mut *mut ::core::ffi::c_void, pversion: *mut CLR_DEBUGGING_VERSION, pdwflags: *mut CLR_DEBUGGING_PROCESS_FLAGS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OpenVirtualProcess(::core::mem::transmute_copy(&modulebaseaddress), ::windows_core::from_raw_borrowed(&pdatatarget), ::windows_core::from_raw_borrowed(&plibraryprovider), ::core::mem::transmute_copy(&pmaxdebuggersupportedversion), ::core::mem::transmute_copy(&riidprocess), ::core::mem::transmute_copy(&ppprocess), ::core::mem::transmute_copy(&pversion), ::core::mem::transmute_copy(&pdwflags)).into()
        }
        unsafe extern "system" fn CanUnloadNow<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRDebugging_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmodule: super::super::Foundation::HMODULE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CanUnloadNow(::core::mem::transmute_copy(&hmodule)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OpenVirtualProcess: OpenVirtualProcess::<Identity, Impl, OFFSET>,
            CanUnloadNow: CanUnloadNow::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ICLRDebugging as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICLRDebuggingLibraryProvider_Impl: Sized {
    fn ProvideLibrary(&self, pwszfilename: &::windows_core::PCWSTR, dwtimestamp: u32, dwsizeofimage: u32) -> ::windows_core::Result<super::super::Foundation::HMODULE>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ICLRDebuggingLibraryProvider {}
#[cfg(feature = "Win32_Foundation")]
impl ICLRDebuggingLibraryProvider_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRDebuggingLibraryProvider_Impl, const OFFSET: isize>() -> ICLRDebuggingLibraryProvider_Vtbl {
        unsafe extern "system" fn ProvideLibrary<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRDebuggingLibraryProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszfilename: ::windows_core::PCWSTR, dwtimestamp: u32, dwsizeofimage: u32, phmodule: *mut super::super::Foundation::HMODULE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ProvideLibrary(::core::mem::transmute(&pwszfilename), ::core::mem::transmute_copy(&dwtimestamp), ::core::mem::transmute_copy(&dwsizeofimage)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phmodule, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ProvideLibrary: ProvideLibrary::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ICLRDebuggingLibraryProvider as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait ICLRDomainManager_Impl: Sized {
    fn SetAppDomainManagerType(&self, wszappdomainmanagerassembly: &::windows_core::PCWSTR, wszappdomainmanagertype: &::windows_core::PCWSTR, dwinitializedomainflags: EInitializeNewDomainFlags) -> ::windows_core::Result<()>;
    fn SetPropertiesForDefaultAppDomain(&self, nproperties: u32, pwszpropertynames: *const ::windows_core::PCWSTR, pwszpropertyvalues: *const ::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ICLRDomainManager {}
impl ICLRDomainManager_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRDomainManager_Impl, const OFFSET: isize>() -> ICLRDomainManager_Vtbl {
        unsafe extern "system" fn SetAppDomainManagerType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRDomainManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszappdomainmanagerassembly: ::windows_core::PCWSTR, wszappdomainmanagertype: ::windows_core::PCWSTR, dwinitializedomainflags: EInitializeNewDomainFlags) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAppDomainManagerType(::core::mem::transmute(&wszappdomainmanagerassembly), ::core::mem::transmute(&wszappdomainmanagertype), ::core::mem::transmute_copy(&dwinitializedomainflags)).into()
        }
        unsafe extern "system" fn SetPropertiesForDefaultAppDomain<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRDomainManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nproperties: u32, pwszpropertynames: *const ::windows_core::PCWSTR, pwszpropertyvalues: *const ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPropertiesForDefaultAppDomain(::core::mem::transmute_copy(&nproperties), ::core::mem::transmute_copy(&pwszpropertynames), ::core::mem::transmute_copy(&pwszpropertyvalues)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetAppDomainManagerType: SetAppDomainManagerType::<Identity, Impl, OFFSET>,
            SetPropertiesForDefaultAppDomain: SetPropertiesForDefaultAppDomain::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ICLRDomainManager as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICLRErrorReportingManager_Impl: Sized {
    fn GetBucketParametersForCurrentException(&self, pparams: *mut BucketParameters) -> ::windows_core::Result<()>;
    fn BeginCustomDump(&self, dwflavor: ECustomDumpFlavor, dwnumitems: u32, items: *const CustomDumpItem, dwreserved: u32) -> ::windows_core::Result<()>;
    fn EndCustomDump(&self) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ICLRErrorReportingManager {}
#[cfg(feature = "Win32_Foundation")]
impl ICLRErrorReportingManager_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRErrorReportingManager_Impl, const OFFSET: isize>() -> ICLRErrorReportingManager_Vtbl {
        unsafe extern "system" fn GetBucketParametersForCurrentException<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRErrorReportingManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparams: *mut BucketParameters) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetBucketParametersForCurrentException(::core::mem::transmute_copy(&pparams)).into()
        }
        unsafe extern "system" fn BeginCustomDump<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRErrorReportingManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflavor: ECustomDumpFlavor, dwnumitems: u32, items: *const CustomDumpItem, dwreserved: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BeginCustomDump(::core::mem::transmute_copy(&dwflavor), ::core::mem::transmute_copy(&dwnumitems), ::core::mem::transmute_copy(&items), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn EndCustomDump<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRErrorReportingManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndCustomDump().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetBucketParametersForCurrentException: GetBucketParametersForCurrentException::<Identity, Impl, OFFSET>,
            BeginCustomDump: BeginCustomDump::<Identity, Impl, OFFSET>,
            EndCustomDump: EndCustomDump::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ICLRErrorReportingManager as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait ICLRGCManager_Impl: Sized {
    fn Collect(&self, generation: i32) -> ::windows_core::Result<()>;
    fn GetStats(&self, pstats: *mut COR_GC_STATS) -> ::windows_core::Result<()>;
    fn SetGCStartupLimits(&self, segmentsize: u32, maxgen0size: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ICLRGCManager {}
impl ICLRGCManager_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRGCManager_Impl, const OFFSET: isize>() -> ICLRGCManager_Vtbl {
        unsafe extern "system" fn Collect<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRGCManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, generation: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Collect(::core::mem::transmute_copy(&generation)).into()
        }
        unsafe extern "system" fn GetStats<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRGCManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstats: *mut COR_GC_STATS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetStats(::core::mem::transmute_copy(&pstats)).into()
        }
        unsafe extern "system" fn SetGCStartupLimits<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRGCManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, segmentsize: u32, maxgen0size: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetGCStartupLimits(::core::mem::transmute_copy(&segmentsize), ::core::mem::transmute_copy(&maxgen0size)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Collect: Collect::<Identity, Impl, OFFSET>,
            GetStats: GetStats::<Identity, Impl, OFFSET>,
            SetGCStartupLimits: SetGCStartupLimits::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ICLRGCManager as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait ICLRGCManager2_Impl: Sized + ICLRGCManager_Impl {
    fn SetGCStartupLimitsEx(&self, segmentsize: usize, maxgen0size: usize) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ICLRGCManager2 {}
impl ICLRGCManager2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRGCManager2_Impl, const OFFSET: isize>() -> ICLRGCManager2_Vtbl {
        unsafe extern "system" fn SetGCStartupLimitsEx<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRGCManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, segmentsize: usize, maxgen0size: usize) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetGCStartupLimitsEx(::core::mem::transmute_copy(&segmentsize), ::core::mem::transmute_copy(&maxgen0size)).into()
        }
        Self { base__: ICLRGCManager_Vtbl::new::<Identity, Impl, OFFSET>(), SetGCStartupLimitsEx: SetGCStartupLimitsEx::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ICLRGCManager2 as ::windows_core::ComInterface>::IID || iid == &<ICLRGCManager as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait ICLRHostBindingPolicyManager_Impl: Sized {
    fn ModifyApplicationPolicy(&self, pwzsourceassemblyidentity: &::windows_core::PCWSTR, pwztargetassemblyidentity: &::windows_core::PCWSTR, pbapplicationpolicy: *const u8, cbapppolicysize: u32, dwpolicymodifyflags: u32, pbnewapplicationpolicy: *mut u8, pcbnewapppolicysize: *mut u32) -> ::windows_core::Result<()>;
    fn EvaluatePolicy(&self, pwzreferenceidentity: &::windows_core::PCWSTR, pbapplicationpolicy: *const u8, cbapppolicysize: u32, pwzpostpolicyreferenceidentity: ::windows_core::PWSTR, pcchpostpolicyreferenceidentity: *mut u32, pdwpoliciesapplied: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ICLRHostBindingPolicyManager {}
impl ICLRHostBindingPolicyManager_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRHostBindingPolicyManager_Impl, const OFFSET: isize>() -> ICLRHostBindingPolicyManager_Vtbl {
        unsafe extern "system" fn ModifyApplicationPolicy<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRHostBindingPolicyManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzsourceassemblyidentity: ::windows_core::PCWSTR, pwztargetassemblyidentity: ::windows_core::PCWSTR, pbapplicationpolicy: *const u8, cbapppolicysize: u32, dwpolicymodifyflags: u32, pbnewapplicationpolicy: *mut u8, pcbnewapppolicysize: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ModifyApplicationPolicy(::core::mem::transmute(&pwzsourceassemblyidentity), ::core::mem::transmute(&pwztargetassemblyidentity), ::core::mem::transmute_copy(&pbapplicationpolicy), ::core::mem::transmute_copy(&cbapppolicysize), ::core::mem::transmute_copy(&dwpolicymodifyflags), ::core::mem::transmute_copy(&pbnewapplicationpolicy), ::core::mem::transmute_copy(&pcbnewapppolicysize)).into()
        }
        unsafe extern "system" fn EvaluatePolicy<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRHostBindingPolicyManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzreferenceidentity: ::windows_core::PCWSTR, pbapplicationpolicy: *const u8, cbapppolicysize: u32, pwzpostpolicyreferenceidentity: ::windows_core::PWSTR, pcchpostpolicyreferenceidentity: *mut u32, pdwpoliciesapplied: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EvaluatePolicy(::core::mem::transmute(&pwzreferenceidentity), ::core::mem::transmute_copy(&pbapplicationpolicy), ::core::mem::transmute_copy(&cbapppolicysize), ::core::mem::transmute_copy(&pwzpostpolicyreferenceidentity), ::core::mem::transmute_copy(&pcchpostpolicyreferenceidentity), ::core::mem::transmute_copy(&pdwpoliciesapplied)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ModifyApplicationPolicy: ModifyApplicationPolicy::<Identity, Impl, OFFSET>,
            EvaluatePolicy: EvaluatePolicy::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ICLRHostBindingPolicyManager as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait ICLRHostProtectionManager_Impl: Sized {
    fn SetProtectedCategories(&self, categories: EApiCategories) -> ::windows_core::Result<()>;
    fn SetEagerSerializeGrantSets(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ICLRHostProtectionManager {}
impl ICLRHostProtectionManager_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRHostProtectionManager_Impl, const OFFSET: isize>() -> ICLRHostProtectionManager_Vtbl {
        unsafe extern "system" fn SetProtectedCategories<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRHostProtectionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, categories: EApiCategories) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProtectedCategories(::core::mem::transmute_copy(&categories)).into()
        }
        unsafe extern "system" fn SetEagerSerializeGrantSets<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRHostProtectionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEagerSerializeGrantSets().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetProtectedCategories: SetProtectedCategories::<Identity, Impl, OFFSET>,
            SetEagerSerializeGrantSets: SetEagerSerializeGrantSets::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ICLRHostProtectionManager as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait ICLRIoCompletionManager_Impl: Sized {
    fn OnComplete(&self, dwerrorcode: u32, numberofbytestransferred: u32, pvoverlapped: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ICLRIoCompletionManager {}
impl ICLRIoCompletionManager_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRIoCompletionManager_Impl, const OFFSET: isize>() -> ICLRIoCompletionManager_Vtbl {
        unsafe extern "system" fn OnComplete<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRIoCompletionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwerrorcode: u32, numberofbytestransferred: u32, pvoverlapped: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnComplete(::core::mem::transmute_copy(&dwerrorcode), ::core::mem::transmute_copy(&numberofbytestransferred), ::core::mem::transmute_copy(&pvoverlapped)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnComplete: OnComplete::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ICLRIoCompletionManager as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait ICLRMemoryNotificationCallback_Impl: Sized {
    fn OnMemoryNotification(&self, ememoryavailable: EMemoryAvailable) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ICLRMemoryNotificationCallback {}
impl ICLRMemoryNotificationCallback_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRMemoryNotificationCallback_Impl, const OFFSET: isize>() -> ICLRMemoryNotificationCallback_Vtbl {
        unsafe extern "system" fn OnMemoryNotification<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRMemoryNotificationCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ememoryavailable: EMemoryAvailable) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnMemoryNotification(::core::mem::transmute_copy(&ememoryavailable)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnMemoryNotification: OnMemoryNotification::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ICLRMemoryNotificationCallback as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ICLRMetaHost_Impl: Sized {
    fn GetRuntime(&self, pwzversion: &::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppruntime: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetVersionFromFile(&self, pwzfilepath: &::windows_core::PCWSTR, pwzbuffer: ::windows_core::PWSTR, pcchbuffer: *mut u32) -> ::windows_core::Result<()>;
    fn EnumerateInstalledRuntimes(&self) -> ::windows_core::Result<super::Com::IEnumUnknown>;
    fn EnumerateLoadedRuntimes(&self, hndprocess: super::super::Foundation::HANDLE) -> ::windows_core::Result<super::Com::IEnumUnknown>;
    fn RequestRuntimeLoadedNotification(&self, pcallbackfunction: RuntimeLoadedCallbackFnPtr) -> ::windows_core::Result<()>;
    fn QueryLegacyV2RuntimeBinding(&self, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn ExitProcess(&self, iexitcode: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for ICLRMetaHost {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ICLRMetaHost_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRMetaHost_Impl, const OFFSET: isize>() -> ICLRMetaHost_Vtbl {
        unsafe extern "system" fn GetRuntime<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRMetaHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzversion: ::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppruntime: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetRuntime(::core::mem::transmute(&pwzversion), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppruntime)).into()
        }
        unsafe extern "system" fn GetVersionFromFile<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRMetaHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzfilepath: ::windows_core::PCWSTR, pwzbuffer: ::windows_core::PWSTR, pcchbuffer: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetVersionFromFile(::core::mem::transmute(&pwzfilepath), ::core::mem::transmute_copy(&pwzbuffer), ::core::mem::transmute_copy(&pcchbuffer)).into()
        }
        unsafe extern "system" fn EnumerateInstalledRuntimes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRMetaHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumerateInstalledRuntimes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumerator, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateLoadedRuntimes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRMetaHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hndprocess: super::super::Foundation::HANDLE, ppenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumerateLoadedRuntimes(::core::mem::transmute_copy(&hndprocess)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumerator, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestRuntimeLoadedNotification<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRMetaHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallbackfunction: RuntimeLoadedCallbackFnPtr) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RequestRuntimeLoadedNotification(::core::mem::transmute_copy(&pcallbackfunction)).into()
        }
        unsafe extern "system" fn QueryLegacyV2RuntimeBinding<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRMetaHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.QueryLegacyV2RuntimeBinding(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppunk)).into()
        }
        unsafe extern "system" fn ExitProcess<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRMetaHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iexitcode: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ExitProcess(::core::mem::transmute_copy(&iexitcode)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetRuntime: GetRuntime::<Identity, Impl, OFFSET>,
            GetVersionFromFile: GetVersionFromFile::<Identity, Impl, OFFSET>,
            EnumerateInstalledRuntimes: EnumerateInstalledRuntimes::<Identity, Impl, OFFSET>,
            EnumerateLoadedRuntimes: EnumerateLoadedRuntimes::<Identity, Impl, OFFSET>,
            RequestRuntimeLoadedNotification: RequestRuntimeLoadedNotification::<Identity, Impl, OFFSET>,
            QueryLegacyV2RuntimeBinding: QueryLegacyV2RuntimeBinding::<Identity, Impl, OFFSET>,
            ExitProcess: ExitProcess::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ICLRMetaHost as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait ICLRMetaHostPolicy_Impl: Sized {
    fn GetRequestedRuntime(&self, dwpolicyflags: METAHOST_POLICY_FLAGS, pwzbinary: &::windows_core::PCWSTR, pcfgstream: ::core::option::Option<&super::Com::IStream>, pwzversion: &::windows_core::PWSTR, pcchversion: *mut u32, pwzimageversion: ::windows_core::PWSTR, pcchimageversion: *mut u32, pdwconfigflags: *mut u32, riid: *const ::windows_core::GUID, ppruntime: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for ICLRMetaHostPolicy {}
#[cfg(feature = "Win32_System_Com")]
impl ICLRMetaHostPolicy_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRMetaHostPolicy_Impl, const OFFSET: isize>() -> ICLRMetaHostPolicy_Vtbl {
        unsafe extern "system" fn GetRequestedRuntime<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRMetaHostPolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwpolicyflags: METAHOST_POLICY_FLAGS, pwzbinary: ::windows_core::PCWSTR, pcfgstream: *mut ::core::ffi::c_void, pwzversion: ::windows_core::PWSTR, pcchversion: *mut u32, pwzimageversion: ::windows_core::PWSTR, pcchimageversion: *mut u32, pdwconfigflags: *mut u32, riid: *const ::windows_core::GUID, ppruntime: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetRequestedRuntime(::core::mem::transmute_copy(&dwpolicyflags), ::core::mem::transmute(&pwzbinary), ::windows_core::from_raw_borrowed(&pcfgstream), ::core::mem::transmute(&pwzversion), ::core::mem::transmute_copy(&pcchversion), ::core::mem::transmute_copy(&pwzimageversion), ::core::mem::transmute_copy(&pcchimageversion), ::core::mem::transmute_copy(&pdwconfigflags), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppruntime)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetRequestedRuntime: GetRequestedRuntime::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ICLRMetaHostPolicy as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait ICLROnEventManager_Impl: Sized {
    fn RegisterActionOnEvent(&self, event: EClrEvent, paction: ::core::option::Option<&IActionOnCLREvent>) -> ::windows_core::Result<()>;
    fn UnregisterActionOnEvent(&self, event: EClrEvent, paction: ::core::option::Option<&IActionOnCLREvent>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ICLROnEventManager {}
impl ICLROnEventManager_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLROnEventManager_Impl, const OFFSET: isize>() -> ICLROnEventManager_Vtbl {
        unsafe extern "system" fn RegisterActionOnEvent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLROnEventManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, event: EClrEvent, paction: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RegisterActionOnEvent(::core::mem::transmute_copy(&event), ::windows_core::from_raw_borrowed(&paction)).into()
        }
        unsafe extern "system" fn UnregisterActionOnEvent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLROnEventManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, event: EClrEvent, paction: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnregisterActionOnEvent(::core::mem::transmute_copy(&event), ::windows_core::from_raw_borrowed(&paction)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterActionOnEvent: RegisterActionOnEvent::<Identity, Impl, OFFSET>,
            UnregisterActionOnEvent: UnregisterActionOnEvent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ICLROnEventManager as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait ICLRPolicyManager_Impl: Sized {
    fn SetDefaultAction(&self, operation: EClrOperation, action: EPolicyAction) -> ::windows_core::Result<()>;
    fn SetTimeout(&self, operation: EClrOperation, dwmilliseconds: u32) -> ::windows_core::Result<()>;
    fn SetActionOnTimeout(&self, operation: EClrOperation, action: EPolicyAction) -> ::windows_core::Result<()>;
    fn SetTimeoutAndAction(&self, operation: EClrOperation, dwmilliseconds: u32, action: EPolicyAction) -> ::windows_core::Result<()>;
    fn SetActionOnFailure(&self, failure: EClrFailure, action: EPolicyAction) -> ::windows_core::Result<()>;
    fn SetUnhandledExceptionPolicy(&self, policy: EClrUnhandledException) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ICLRPolicyManager {}
impl ICLRPolicyManager_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRPolicyManager_Impl, const OFFSET: isize>() -> ICLRPolicyManager_Vtbl {
        unsafe extern "system" fn SetDefaultAction<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRPolicyManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operation: EClrOperation, action: EPolicyAction) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDefaultAction(::core::mem::transmute_copy(&operation), ::core::mem::transmute_copy(&action)).into()
        }
        unsafe extern "system" fn SetTimeout<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRPolicyManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operation: EClrOperation, dwmilliseconds: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTimeout(::core::mem::transmute_copy(&operation), ::core::mem::transmute_copy(&dwmilliseconds)).into()
        }
        unsafe extern "system" fn SetActionOnTimeout<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRPolicyManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operation: EClrOperation, action: EPolicyAction) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetActionOnTimeout(::core::mem::transmute_copy(&operation), ::core::mem::transmute_copy(&action)).into()
        }
        unsafe extern "system" fn SetTimeoutAndAction<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRPolicyManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operation: EClrOperation, dwmilliseconds: u32, action: EPolicyAction) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTimeoutAndAction(::core::mem::transmute_copy(&operation), ::core::mem::transmute_copy(&dwmilliseconds), ::core::mem::transmute_copy(&action)).into()
        }
        unsafe extern "system" fn SetActionOnFailure<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRPolicyManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, failure: EClrFailure, action: EPolicyAction) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetActionOnFailure(::core::mem::transmute_copy(&failure), ::core::mem::transmute_copy(&action)).into()
        }
        unsafe extern "system" fn SetUnhandledExceptionPolicy<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRPolicyManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, policy: EClrUnhandledException) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetUnhandledExceptionPolicy(::core::mem::transmute_copy(&policy)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetDefaultAction: SetDefaultAction::<Identity, Impl, OFFSET>,
            SetTimeout: SetTimeout::<Identity, Impl, OFFSET>,
            SetActionOnTimeout: SetActionOnTimeout::<Identity, Impl, OFFSET>,
            SetTimeoutAndAction: SetTimeoutAndAction::<Identity, Impl, OFFSET>,
            SetActionOnFailure: SetActionOnFailure::<Identity, Impl, OFFSET>,
            SetUnhandledExceptionPolicy: SetUnhandledExceptionPolicy::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ICLRPolicyManager as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait ICLRProbingAssemblyEnum_Impl: Sized {
    fn Get(&self, dwindex: u32, pwzbuffer: ::windows_core::PWSTR, pcchbuffersize: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ICLRProbingAssemblyEnum {}
impl ICLRProbingAssemblyEnum_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRProbingAssemblyEnum_Impl, const OFFSET: isize>() -> ICLRProbingAssemblyEnum_Vtbl {
        unsafe extern "system" fn Get<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRProbingAssemblyEnum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, pwzbuffer: ::windows_core::PWSTR, pcchbuffersize: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Get(::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&pwzbuffer), ::core::mem::transmute_copy(&pcchbuffersize)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Get: Get::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ICLRProbingAssemblyEnum as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait ICLRProfiling_Impl: Sized {
    fn AttachProfiler(&self, dwprofileeprocessid: u32, dwmillisecondsmax: u32, pclsidprofiler: *const ::windows_core::GUID, wszprofilerpath: &::windows_core::PCWSTR, pvclientdata: *const ::core::ffi::c_void, cbclientdata: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ICLRProfiling {}
impl ICLRProfiling_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRProfiling_Impl, const OFFSET: isize>() -> ICLRProfiling_Vtbl {
        unsafe extern "system" fn AttachProfiler<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRProfiling_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwprofileeprocessid: u32, dwmillisecondsmax: u32, pclsidprofiler: *const ::windows_core::GUID, wszprofilerpath: ::windows_core::PCWSTR, pvclientdata: *const ::core::ffi::c_void, cbclientdata: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AttachProfiler(::core::mem::transmute_copy(&dwprofileeprocessid), ::core::mem::transmute_copy(&dwmillisecondsmax), ::core::mem::transmute_copy(&pclsidprofiler), ::core::mem::transmute(&wszprofilerpath), ::core::mem::transmute_copy(&pvclientdata), ::core::mem::transmute_copy(&cbclientdata)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AttachProfiler: AttachProfiler::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ICLRProfiling as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait ICLRReferenceAssemblyEnum_Impl: Sized {
    fn Get(&self, dwindex: u32, pwzbuffer: ::windows_core::PWSTR, pcchbuffersize: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ICLRReferenceAssemblyEnum {}
impl ICLRReferenceAssemblyEnum_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRReferenceAssemblyEnum_Impl, const OFFSET: isize>() -> ICLRReferenceAssemblyEnum_Vtbl {
        unsafe extern "system" fn Get<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRReferenceAssemblyEnum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, pwzbuffer: ::windows_core::PWSTR, pcchbuffersize: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Get(::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&pwzbuffer), ::core::mem::transmute_copy(&pcchbuffersize)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Get: Get::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ICLRReferenceAssemblyEnum as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICLRRuntimeHost_Impl: Sized {
    fn Start(&self) -> ::windows_core::Result<()>;
    fn Stop(&self) -> ::windows_core::Result<()>;
    fn SetHostControl(&self, phostcontrol: ::core::option::Option<&IHostControl>) -> ::windows_core::Result<()>;
    fn GetCLRControl(&self) -> ::windows_core::Result<ICLRControl>;
    fn UnloadAppDomain(&self, dwappdomainid: u32, fwaituntildone: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn ExecuteInAppDomain(&self, dwappdomainid: u32, pcallback: FExecuteInAppDomainCallback, cookie: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetCurrentAppDomainId(&self) -> ::windows_core::Result<u32>;
    fn ExecuteApplication(&self, pwzappfullname: &::windows_core::PCWSTR, dwmanifestpaths: u32, ppwzmanifestpaths: *const ::windows_core::PCWSTR, dwactivationdata: u32, ppwzactivationdata: *const ::windows_core::PCWSTR) -> ::windows_core::Result<i32>;
    fn ExecuteInDefaultAppDomain(&self, pwzassemblypath: &::windows_core::PCWSTR, pwztypename: &::windows_core::PCWSTR, pwzmethodname: &::windows_core::PCWSTR, pwzargument: &::windows_core::PCWSTR) -> ::windows_core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ICLRRuntimeHost {}
#[cfg(feature = "Win32_Foundation")]
impl ICLRRuntimeHost_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRRuntimeHost_Impl, const OFFSET: isize>() -> ICLRRuntimeHost_Vtbl {
        unsafe extern "system" fn Start<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRRuntimeHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Start().into()
        }
        unsafe extern "system" fn Stop<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRRuntimeHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Stop().into()
        }
        unsafe extern "system" fn SetHostControl<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRRuntimeHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phostcontrol: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetHostControl(::windows_core::from_raw_borrowed(&phostcontrol)).into()
        }
        unsafe extern "system" fn GetCLRControl<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRRuntimeHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclrcontrol: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCLRControl() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pclrcontrol, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnloadAppDomain<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRRuntimeHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwappdomainid: u32, fwaituntildone: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnloadAppDomain(::core::mem::transmute_copy(&dwappdomainid), ::core::mem::transmute_copy(&fwaituntildone)).into()
        }
        unsafe extern "system" fn ExecuteInAppDomain<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRRuntimeHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwappdomainid: u32, pcallback: FExecuteInAppDomainCallback, cookie: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ExecuteInAppDomain(::core::mem::transmute_copy(&dwappdomainid), ::core::mem::transmute_copy(&pcallback), ::core::mem::transmute_copy(&cookie)).into()
        }
        unsafe extern "system" fn GetCurrentAppDomainId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRRuntimeHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwappdomainid: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurrentAppDomainId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwappdomainid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExecuteApplication<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRRuntimeHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzappfullname: ::windows_core::PCWSTR, dwmanifestpaths: u32, ppwzmanifestpaths: *const ::windows_core::PCWSTR, dwactivationdata: u32, ppwzactivationdata: *const ::windows_core::PCWSTR, preturnvalue: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ExecuteApplication(::core::mem::transmute(&pwzappfullname), ::core::mem::transmute_copy(&dwmanifestpaths), ::core::mem::transmute_copy(&ppwzmanifestpaths), ::core::mem::transmute_copy(&dwactivationdata), ::core::mem::transmute_copy(&ppwzactivationdata)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(preturnvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExecuteInDefaultAppDomain<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRRuntimeHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzassemblypath: ::windows_core::PCWSTR, pwztypename: ::windows_core::PCWSTR, pwzmethodname: ::windows_core::PCWSTR, pwzargument: ::windows_core::PCWSTR, preturnvalue: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ExecuteInDefaultAppDomain(::core::mem::transmute(&pwzassemblypath), ::core::mem::transmute(&pwztypename), ::core::mem::transmute(&pwzmethodname), ::core::mem::transmute(&pwzargument)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(preturnvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Start: Start::<Identity, Impl, OFFSET>,
            Stop: Stop::<Identity, Impl, OFFSET>,
            SetHostControl: SetHostControl::<Identity, Impl, OFFSET>,
            GetCLRControl: GetCLRControl::<Identity, Impl, OFFSET>,
            UnloadAppDomain: UnloadAppDomain::<Identity, Impl, OFFSET>,
            ExecuteInAppDomain: ExecuteInAppDomain::<Identity, Impl, OFFSET>,
            GetCurrentAppDomainId: GetCurrentAppDomainId::<Identity, Impl, OFFSET>,
            ExecuteApplication: ExecuteApplication::<Identity, Impl, OFFSET>,
            ExecuteInDefaultAppDomain: ExecuteInDefaultAppDomain::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ICLRRuntimeHost as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICLRRuntimeInfo_Impl: Sized {
    fn GetVersionString(&self, pwzbuffer: ::windows_core::PWSTR, pcchbuffer: *mut u32) -> ::windows_core::Result<()>;
    fn GetRuntimeDirectory(&self, pwzbuffer: ::windows_core::PWSTR, pcchbuffer: *mut u32) -> ::windows_core::Result<()>;
    fn IsLoaded(&self, hndprocess: super::super::Foundation::HANDLE) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn LoadErrorString(&self, iresourceid: u32, pwzbuffer: ::windows_core::PWSTR, pcchbuffer: *mut u32, ilocaleid: i32) -> ::windows_core::Result<()>;
    fn LoadLibraryA(&self, pwzdllname: &::windows_core::PCWSTR) -> ::windows_core::Result<super::super::Foundation::HMODULE>;
    fn GetProcAddress(&self, pszprocname: &::windows_core::PCSTR) -> ::windows_core::Result<*mut ::core::ffi::c_void>;
    fn GetInterface(&self, rclsid: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn IsLoadable(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetDefaultStartupFlags(&self, dwstartupflags: u32, pwzhostconfigfile: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetDefaultStartupFlags(&self, pdwstartupflags: *mut u32, pwzhostconfigfile: ::windows_core::PWSTR, pcchhostconfigfile: *mut u32) -> ::windows_core::Result<()>;
    fn BindAsLegacyV2Runtime(&self) -> ::windows_core::Result<()>;
    fn IsStarted(&self, pbstarted: *mut super::super::Foundation::BOOL, pdwstartupflags: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ICLRRuntimeInfo {}
#[cfg(feature = "Win32_Foundation")]
impl ICLRRuntimeInfo_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRRuntimeInfo_Impl, const OFFSET: isize>() -> ICLRRuntimeInfo_Vtbl {
        unsafe extern "system" fn GetVersionString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRRuntimeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzbuffer: ::windows_core::PWSTR, pcchbuffer: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetVersionString(::core::mem::transmute_copy(&pwzbuffer), ::core::mem::transmute_copy(&pcchbuffer)).into()
        }
        unsafe extern "system" fn GetRuntimeDirectory<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRRuntimeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzbuffer: ::windows_core::PWSTR, pcchbuffer: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetRuntimeDirectory(::core::mem::transmute_copy(&pwzbuffer), ::core::mem::transmute_copy(&pcchbuffer)).into()
        }
        unsafe extern "system" fn IsLoaded<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRRuntimeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hndprocess: super::super::Foundation::HANDLE, pbloaded: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsLoaded(::core::mem::transmute_copy(&hndprocess)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbloaded, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadErrorString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRRuntimeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iresourceid: u32, pwzbuffer: ::windows_core::PWSTR, pcchbuffer: *mut u32, ilocaleid: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LoadErrorString(::core::mem::transmute_copy(&iresourceid), ::core::mem::transmute_copy(&pwzbuffer), ::core::mem::transmute_copy(&pcchbuffer), ::core::mem::transmute_copy(&ilocaleid)).into()
        }
        unsafe extern "system" fn LoadLibraryA<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRRuntimeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzdllname: ::windows_core::PCWSTR, phndmodule: *mut super::super::Foundation::HMODULE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LoadLibraryA(::core::mem::transmute(&pwzdllname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phndmodule, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProcAddress<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRRuntimeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszprocname: ::windows_core::PCSTR, ppproc: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProcAddress(::core::mem::transmute(&pszprocname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInterface<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRRuntimeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetInterface(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppunk)).into()
        }
        unsafe extern "system" fn IsLoadable<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRRuntimeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbloadable: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsLoadable() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbloadable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultStartupFlags<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRRuntimeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstartupflags: u32, pwzhostconfigfile: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDefaultStartupFlags(::core::mem::transmute_copy(&dwstartupflags), ::core::mem::transmute(&pwzhostconfigfile)).into()
        }
        unsafe extern "system" fn GetDefaultStartupFlags<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRRuntimeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstartupflags: *mut u32, pwzhostconfigfile: ::windows_core::PWSTR, pcchhostconfigfile: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDefaultStartupFlags(::core::mem::transmute_copy(&pdwstartupflags), ::core::mem::transmute_copy(&pwzhostconfigfile), ::core::mem::transmute_copy(&pcchhostconfigfile)).into()
        }
        unsafe extern "system" fn BindAsLegacyV2Runtime<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRRuntimeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BindAsLegacyV2Runtime().into()
        }
        unsafe extern "system" fn IsStarted<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRRuntimeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstarted: *mut super::super::Foundation::BOOL, pdwstartupflags: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsStarted(::core::mem::transmute_copy(&pbstarted), ::core::mem::transmute_copy(&pdwstartupflags)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetVersionString: GetVersionString::<Identity, Impl, OFFSET>,
            GetRuntimeDirectory: GetRuntimeDirectory::<Identity, Impl, OFFSET>,
            IsLoaded: IsLoaded::<Identity, Impl, OFFSET>,
            LoadErrorString: LoadErrorString::<Identity, Impl, OFFSET>,
            LoadLibraryA: LoadLibraryA::<Identity, Impl, OFFSET>,
            GetProcAddress: GetProcAddress::<Identity, Impl, OFFSET>,
            GetInterface: GetInterface::<Identity, Impl, OFFSET>,
            IsLoadable: IsLoadable::<Identity, Impl, OFFSET>,
            SetDefaultStartupFlags: SetDefaultStartupFlags::<Identity, Impl, OFFSET>,
            GetDefaultStartupFlags: GetDefaultStartupFlags::<Identity, Impl, OFFSET>,
            BindAsLegacyV2Runtime: BindAsLegacyV2Runtime::<Identity, Impl, OFFSET>,
            IsStarted: IsStarted::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ICLRRuntimeInfo as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICLRStrongName_Impl: Sized {
    fn GetHashFromAssemblyFile(&self, pszfilepath: &::windows_core::PCSTR, pihashalg: *mut u32, pbhash: *mut u8, cchhash: u32, pchhash: *mut u32) -> ::windows_core::Result<()>;
    fn GetHashFromAssemblyFileW(&self, pwzfilepath: &::windows_core::PCWSTR, pihashalg: *mut u32, pbhash: *mut u8, cchhash: u32, pchhash: *mut u32) -> ::windows_core::Result<()>;
    fn GetHashFromBlob(&self, pbblob: *const u8, cchblob: u32, pihashalg: *mut u32, pbhash: *mut u8, cchhash: u32, pchhash: *mut u32) -> ::windows_core::Result<()>;
    fn GetHashFromFile(&self, pszfilepath: &::windows_core::PCSTR, pihashalg: *mut u32, pbhash: *mut u8, cchhash: u32, pchhash: *mut u32) -> ::windows_core::Result<()>;
    fn GetHashFromFileW(&self, pwzfilepath: &::windows_core::PCWSTR, pihashalg: *mut u32, pbhash: *mut u8, cchhash: u32, pchhash: *mut u32) -> ::windows_core::Result<()>;
    fn GetHashFromHandle(&self, hfile: super::super::Foundation::HANDLE, pihashalg: *mut u32, pbhash: *mut u8, cchhash: u32, pchhash: *mut u32) -> ::windows_core::Result<()>;
    fn StrongNameCompareAssemblies(&self, pwzassembly1: &::windows_core::PCWSTR, pwzassembly2: &::windows_core::PCWSTR) -> ::windows_core::Result<u32>;
    fn StrongNameFreeBuffer(&self, pbmemory: *const u8) -> ::windows_core::Result<()>;
    fn StrongNameGetBlob(&self, pwzfilepath: &::windows_core::PCWSTR, pbblob: *mut u8, pcbblob: *mut u32) -> ::windows_core::Result<()>;
    fn StrongNameGetBlobFromImage(&self, pbbase: *const u8, dwlength: u32, pbblob: *mut u8, pcbblob: *mut u32) -> ::windows_core::Result<()>;
    fn StrongNameGetPublicKey(&self, pwzkeycontainer: &::windows_core::PCWSTR, pbkeyblob: *const u8, cbkeyblob: u32, ppbpublickeyblob: *mut *mut u8, pcbpublickeyblob: *mut u32) -> ::windows_core::Result<()>;
    fn StrongNameHashSize(&self, ulhashalg: u32) -> ::windows_core::Result<u32>;
    fn StrongNameKeyDelete(&self, pwzkeycontainer: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn StrongNameKeyGen(&self, pwzkeycontainer: &::windows_core::PCWSTR, dwflags: u32, ppbkeyblob: *mut *mut u8, pcbkeyblob: *mut u32) -> ::windows_core::Result<()>;
    fn StrongNameKeyGenEx(&self, pwzkeycontainer: &::windows_core::PCWSTR, dwflags: u32, dwkeysize: u32, ppbkeyblob: *mut *mut u8, pcbkeyblob: *mut u32) -> ::windows_core::Result<()>;
    fn StrongNameKeyInstall(&self, pwzkeycontainer: &::windows_core::PCWSTR, pbkeyblob: *const u8, cbkeyblob: u32) -> ::windows_core::Result<()>;
    fn StrongNameSignatureGeneration(&self, pwzfilepath: &::windows_core::PCWSTR, pwzkeycontainer: &::windows_core::PCWSTR, pbkeyblob: *const u8, cbkeyblob: u32, ppbsignatureblob: *mut *mut u8, pcbsignatureblob: *mut u32) -> ::windows_core::Result<()>;
    fn StrongNameSignatureGenerationEx(&self, wszfilepath: &::windows_core::PCWSTR, wszkeycontainer: &::windows_core::PCWSTR, pbkeyblob: *const u8, cbkeyblob: u32, ppbsignatureblob: *mut *mut u8, pcbsignatureblob: *mut u32, dwflags: u32) -> ::windows_core::Result<()>;
    fn StrongNameSignatureSize(&self, pbpublickeyblob: *const u8, cbpublickeyblob: u32, pcbsize: *const u32) -> ::windows_core::Result<()>;
    fn StrongNameSignatureVerification(&self, pwzfilepath: &::windows_core::PCWSTR, dwinflags: u32) -> ::windows_core::Result<u32>;
    fn StrongNameSignatureVerificationEx(&self, pwzfilepath: &::windows_core::PCWSTR, fforceverification: super::super::Foundation::BOOLEAN) -> ::windows_core::Result<u8>;
    fn StrongNameSignatureVerificationFromImage(&self, pbbase: *const u8, dwlength: u32, dwinflags: u32) -> ::windows_core::Result<u32>;
    fn StrongNameTokenFromAssembly(&self, pwzfilepath: &::windows_core::PCWSTR, ppbstrongnametoken: *mut *mut u8, pcbstrongnametoken: *mut u32) -> ::windows_core::Result<()>;
    fn StrongNameTokenFromAssemblyEx(&self, pwzfilepath: &::windows_core::PCWSTR, ppbstrongnametoken: *mut *mut u8, pcbstrongnametoken: *mut u32, ppbpublickeyblob: *mut *mut u8, pcbpublickeyblob: *mut u32) -> ::windows_core::Result<()>;
    fn StrongNameTokenFromPublicKey(&self, pbpublickeyblob: *const u8, cbpublickeyblob: u32, ppbstrongnametoken: *mut *mut u8, pcbstrongnametoken: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ICLRStrongName {}
#[cfg(feature = "Win32_Foundation")]
impl ICLRStrongName_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: isize>() -> ICLRStrongName_Vtbl {
        unsafe extern "system" fn GetHashFromAssemblyFile<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfilepath: ::windows_core::PCSTR, pihashalg: *mut u32, pbhash: *mut u8, cchhash: u32, pchhash: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetHashFromAssemblyFile(::core::mem::transmute(&pszfilepath), ::core::mem::transmute_copy(&pihashalg), ::core::mem::transmute_copy(&pbhash), ::core::mem::transmute_copy(&cchhash), ::core::mem::transmute_copy(&pchhash)).into()
        }
        unsafe extern "system" fn GetHashFromAssemblyFileW<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzfilepath: ::windows_core::PCWSTR, pihashalg: *mut u32, pbhash: *mut u8, cchhash: u32, pchhash: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetHashFromAssemblyFileW(::core::mem::transmute(&pwzfilepath), ::core::mem::transmute_copy(&pihashalg), ::core::mem::transmute_copy(&pbhash), ::core::mem::transmute_copy(&cchhash), ::core::mem::transmute_copy(&pchhash)).into()
        }
        unsafe extern "system" fn GetHashFromBlob<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbblob: *const u8, cchblob: u32, pihashalg: *mut u32, pbhash: *mut u8, cchhash: u32, pchhash: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetHashFromBlob(::core::mem::transmute_copy(&pbblob), ::core::mem::transmute_copy(&cchblob), ::core::mem::transmute_copy(&pihashalg), ::core::mem::transmute_copy(&pbhash), ::core::mem::transmute_copy(&cchhash), ::core::mem::transmute_copy(&pchhash)).into()
        }
        unsafe extern "system" fn GetHashFromFile<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfilepath: ::windows_core::PCSTR, pihashalg: *mut u32, pbhash: *mut u8, cchhash: u32, pchhash: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetHashFromFile(::core::mem::transmute(&pszfilepath), ::core::mem::transmute_copy(&pihashalg), ::core::mem::transmute_copy(&pbhash), ::core::mem::transmute_copy(&cchhash), ::core::mem::transmute_copy(&pchhash)).into()
        }
        unsafe extern "system" fn GetHashFromFileW<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzfilepath: ::windows_core::PCWSTR, pihashalg: *mut u32, pbhash: *mut u8, cchhash: u32, pchhash: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetHashFromFileW(::core::mem::transmute(&pwzfilepath), ::core::mem::transmute_copy(&pihashalg), ::core::mem::transmute_copy(&pbhash), ::core::mem::transmute_copy(&cchhash), ::core::mem::transmute_copy(&pchhash)).into()
        }
        unsafe extern "system" fn GetHashFromHandle<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hfile: super::super::Foundation::HANDLE, pihashalg: *mut u32, pbhash: *mut u8, cchhash: u32, pchhash: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetHashFromHandle(::core::mem::transmute_copy(&hfile), ::core::mem::transmute_copy(&pihashalg), ::core::mem::transmute_copy(&pbhash), ::core::mem::transmute_copy(&cchhash), ::core::mem::transmute_copy(&pchhash)).into()
        }
        unsafe extern "system" fn StrongNameCompareAssemblies<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzassembly1: ::windows_core::PCWSTR, pwzassembly2: ::windows_core::PCWSTR, pdwresult: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StrongNameCompareAssemblies(::core::mem::transmute(&pwzassembly1), ::core::mem::transmute(&pwzassembly2)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StrongNameFreeBuffer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbmemory: *const u8) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StrongNameFreeBuffer(::core::mem::transmute_copy(&pbmemory)).into()
        }
        unsafe extern "system" fn StrongNameGetBlob<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzfilepath: ::windows_core::PCWSTR, pbblob: *mut u8, pcbblob: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StrongNameGetBlob(::core::mem::transmute(&pwzfilepath), ::core::mem::transmute_copy(&pbblob), ::core::mem::transmute_copy(&pcbblob)).into()
        }
        unsafe extern "system" fn StrongNameGetBlobFromImage<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbbase: *const u8, dwlength: u32, pbblob: *mut u8, pcbblob: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StrongNameGetBlobFromImage(::core::mem::transmute_copy(&pbbase), ::core::mem::transmute_copy(&dwlength), ::core::mem::transmute_copy(&pbblob), ::core::mem::transmute_copy(&pcbblob)).into()
        }
        unsafe extern "system" fn StrongNameGetPublicKey<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzkeycontainer: ::windows_core::PCWSTR, pbkeyblob: *const u8, cbkeyblob: u32, ppbpublickeyblob: *mut *mut u8, pcbpublickeyblob: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StrongNameGetPublicKey(::core::mem::transmute(&pwzkeycontainer), ::core::mem::transmute_copy(&pbkeyblob), ::core::mem::transmute_copy(&cbkeyblob), ::core::mem::transmute_copy(&ppbpublickeyblob), ::core::mem::transmute_copy(&pcbpublickeyblob)).into()
        }
        unsafe extern "system" fn StrongNameHashSize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulhashalg: u32, pcbsize: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StrongNameHashSize(::core::mem::transmute_copy(&ulhashalg)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcbsize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StrongNameKeyDelete<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzkeycontainer: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StrongNameKeyDelete(::core::mem::transmute(&pwzkeycontainer)).into()
        }
        unsafe extern "system" fn StrongNameKeyGen<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzkeycontainer: ::windows_core::PCWSTR, dwflags: u32, ppbkeyblob: *mut *mut u8, pcbkeyblob: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StrongNameKeyGen(::core::mem::transmute(&pwzkeycontainer), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&ppbkeyblob), ::core::mem::transmute_copy(&pcbkeyblob)).into()
        }
        unsafe extern "system" fn StrongNameKeyGenEx<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzkeycontainer: ::windows_core::PCWSTR, dwflags: u32, dwkeysize: u32, ppbkeyblob: *mut *mut u8, pcbkeyblob: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StrongNameKeyGenEx(::core::mem::transmute(&pwzkeycontainer), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&dwkeysize), ::core::mem::transmute_copy(&ppbkeyblob), ::core::mem::transmute_copy(&pcbkeyblob)).into()
        }
        unsafe extern "system" fn StrongNameKeyInstall<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzkeycontainer: ::windows_core::PCWSTR, pbkeyblob: *const u8, cbkeyblob: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StrongNameKeyInstall(::core::mem::transmute(&pwzkeycontainer), ::core::mem::transmute_copy(&pbkeyblob), ::core::mem::transmute_copy(&cbkeyblob)).into()
        }
        unsafe extern "system" fn StrongNameSignatureGeneration<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzfilepath: ::windows_core::PCWSTR, pwzkeycontainer: ::windows_core::PCWSTR, pbkeyblob: *const u8, cbkeyblob: u32, ppbsignatureblob: *mut *mut u8, pcbsignatureblob: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StrongNameSignatureGeneration(::core::mem::transmute(&pwzfilepath), ::core::mem::transmute(&pwzkeycontainer), ::core::mem::transmute_copy(&pbkeyblob), ::core::mem::transmute_copy(&cbkeyblob), ::core::mem::transmute_copy(&ppbsignatureblob), ::core::mem::transmute_copy(&pcbsignatureblob)).into()
        }
        unsafe extern "system" fn StrongNameSignatureGenerationEx<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszfilepath: ::windows_core::PCWSTR, wszkeycontainer: ::windows_core::PCWSTR, pbkeyblob: *const u8, cbkeyblob: u32, ppbsignatureblob: *mut *mut u8, pcbsignatureblob: *mut u32, dwflags: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StrongNameSignatureGenerationEx(::core::mem::transmute(&wszfilepath), ::core::mem::transmute(&wszkeycontainer), ::core::mem::transmute_copy(&pbkeyblob), ::core::mem::transmute_copy(&cbkeyblob), ::core::mem::transmute_copy(&ppbsignatureblob), ::core::mem::transmute_copy(&pcbsignatureblob), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn StrongNameSignatureSize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbpublickeyblob: *const u8, cbpublickeyblob: u32, pcbsize: *const u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StrongNameSignatureSize(::core::mem::transmute_copy(&pbpublickeyblob), ::core::mem::transmute_copy(&cbpublickeyblob), ::core::mem::transmute_copy(&pcbsize)).into()
        }
        unsafe extern "system" fn StrongNameSignatureVerification<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzfilepath: ::windows_core::PCWSTR, dwinflags: u32, pdwoutflags: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StrongNameSignatureVerification(::core::mem::transmute(&pwzfilepath), ::core::mem::transmute_copy(&dwinflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwoutflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StrongNameSignatureVerificationEx<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzfilepath: ::windows_core::PCWSTR, fforceverification: super::super::Foundation::BOOLEAN, pfwasverified: *mut u8) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StrongNameSignatureVerificationEx(::core::mem::transmute(&pwzfilepath), ::core::mem::transmute_copy(&fforceverification)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfwasverified, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StrongNameSignatureVerificationFromImage<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbbase: *const u8, dwlength: u32, dwinflags: u32, pdwoutflags: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StrongNameSignatureVerificationFromImage(::core::mem::transmute_copy(&pbbase), ::core::mem::transmute_copy(&dwlength), ::core::mem::transmute_copy(&dwinflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwoutflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StrongNameTokenFromAssembly<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzfilepath: ::windows_core::PCWSTR, ppbstrongnametoken: *mut *mut u8, pcbstrongnametoken: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StrongNameTokenFromAssembly(::core::mem::transmute(&pwzfilepath), ::core::mem::transmute_copy(&ppbstrongnametoken), ::core::mem::transmute_copy(&pcbstrongnametoken)).into()
        }
        unsafe extern "system" fn StrongNameTokenFromAssemblyEx<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzfilepath: ::windows_core::PCWSTR, ppbstrongnametoken: *mut *mut u8, pcbstrongnametoken: *mut u32, ppbpublickeyblob: *mut *mut u8, pcbpublickeyblob: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StrongNameTokenFromAssemblyEx(::core::mem::transmute(&pwzfilepath), ::core::mem::transmute_copy(&ppbstrongnametoken), ::core::mem::transmute_copy(&pcbstrongnametoken), ::core::mem::transmute_copy(&ppbpublickeyblob), ::core::mem::transmute_copy(&pcbpublickeyblob)).into()
        }
        unsafe extern "system" fn StrongNameTokenFromPublicKey<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbpublickeyblob: *const u8, cbpublickeyblob: u32, ppbstrongnametoken: *mut *mut u8, pcbstrongnametoken: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StrongNameTokenFromPublicKey(::core::mem::transmute_copy(&pbpublickeyblob), ::core::mem::transmute_copy(&cbpublickeyblob), ::core::mem::transmute_copy(&ppbstrongnametoken), ::core::mem::transmute_copy(&pcbstrongnametoken)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetHashFromAssemblyFile: GetHashFromAssemblyFile::<Identity, Impl, OFFSET>,
            GetHashFromAssemblyFileW: GetHashFromAssemblyFileW::<Identity, Impl, OFFSET>,
            GetHashFromBlob: GetHashFromBlob::<Identity, Impl, OFFSET>,
            GetHashFromFile: GetHashFromFile::<Identity, Impl, OFFSET>,
            GetHashFromFileW: GetHashFromFileW::<Identity, Impl, OFFSET>,
            GetHashFromHandle: GetHashFromHandle::<Identity, Impl, OFFSET>,
            StrongNameCompareAssemblies: StrongNameCompareAssemblies::<Identity, Impl, OFFSET>,
            StrongNameFreeBuffer: StrongNameFreeBuffer::<Identity, Impl, OFFSET>,
            StrongNameGetBlob: StrongNameGetBlob::<Identity, Impl, OFFSET>,
            StrongNameGetBlobFromImage: StrongNameGetBlobFromImage::<Identity, Impl, OFFSET>,
            StrongNameGetPublicKey: StrongNameGetPublicKey::<Identity, Impl, OFFSET>,
            StrongNameHashSize: StrongNameHashSize::<Identity, Impl, OFFSET>,
            StrongNameKeyDelete: StrongNameKeyDelete::<Identity, Impl, OFFSET>,
            StrongNameKeyGen: StrongNameKeyGen::<Identity, Impl, OFFSET>,
            StrongNameKeyGenEx: StrongNameKeyGenEx::<Identity, Impl, OFFSET>,
            StrongNameKeyInstall: StrongNameKeyInstall::<Identity, Impl, OFFSET>,
            StrongNameSignatureGeneration: StrongNameSignatureGeneration::<Identity, Impl, OFFSET>,
            StrongNameSignatureGenerationEx: StrongNameSignatureGenerationEx::<Identity, Impl, OFFSET>,
            StrongNameSignatureSize: StrongNameSignatureSize::<Identity, Impl, OFFSET>,
            StrongNameSignatureVerification: StrongNameSignatureVerification::<Identity, Impl, OFFSET>,
            StrongNameSignatureVerificationEx: StrongNameSignatureVerificationEx::<Identity, Impl, OFFSET>,
            StrongNameSignatureVerificationFromImage: StrongNameSignatureVerificationFromImage::<Identity, Impl, OFFSET>,
            StrongNameTokenFromAssembly: StrongNameTokenFromAssembly::<Identity, Impl, OFFSET>,
            StrongNameTokenFromAssemblyEx: StrongNameTokenFromAssemblyEx::<Identity, Impl, OFFSET>,
            StrongNameTokenFromPublicKey: StrongNameTokenFromPublicKey::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ICLRStrongName as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICLRStrongName2_Impl: Sized {
    fn StrongNameGetPublicKeyEx(&self, pwzkeycontainer: &::windows_core::PCWSTR, pbkeyblob: *const u8, cbkeyblob: u32, ppbpublickeyblob: *mut *mut u8, pcbpublickeyblob: *mut u32, uhashalgid: u32, ureserved: u32) -> ::windows_core::Result<()>;
    fn StrongNameSignatureVerificationEx2(&self, wszfilepath: &::windows_core::PCWSTR, fforceverification: super::super::Foundation::BOOLEAN, pbecmapublickey: *const u8, cbecmapublickey: u32) -> ::windows_core::Result<u8>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ICLRStrongName2 {}
#[cfg(feature = "Win32_Foundation")]
impl ICLRStrongName2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRStrongName2_Impl, const OFFSET: isize>() -> ICLRStrongName2_Vtbl {
        unsafe extern "system" fn StrongNameGetPublicKeyEx<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRStrongName2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzkeycontainer: ::windows_core::PCWSTR, pbkeyblob: *const u8, cbkeyblob: u32, ppbpublickeyblob: *mut *mut u8, pcbpublickeyblob: *mut u32, uhashalgid: u32, ureserved: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StrongNameGetPublicKeyEx(::core::mem::transmute(&pwzkeycontainer), ::core::mem::transmute_copy(&pbkeyblob), ::core::mem::transmute_copy(&cbkeyblob), ::core::mem::transmute_copy(&ppbpublickeyblob), ::core::mem::transmute_copy(&pcbpublickeyblob), ::core::mem::transmute_copy(&uhashalgid), ::core::mem::transmute_copy(&ureserved)).into()
        }
        unsafe extern "system" fn StrongNameSignatureVerificationEx2<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRStrongName2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszfilepath: ::windows_core::PCWSTR, fforceverification: super::super::Foundation::BOOLEAN, pbecmapublickey: *const u8, cbecmapublickey: u32, pfwasverified: *mut u8) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StrongNameSignatureVerificationEx2(::core::mem::transmute(&wszfilepath), ::core::mem::transmute_copy(&fforceverification), ::core::mem::transmute_copy(&pbecmapublickey), ::core::mem::transmute_copy(&cbecmapublickey)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfwasverified, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            StrongNameGetPublicKeyEx: StrongNameGetPublicKeyEx::<Identity, Impl, OFFSET>,
            StrongNameSignatureVerificationEx2: StrongNameSignatureVerificationEx2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ICLRStrongName2 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait ICLRStrongName3_Impl: Sized {
    fn StrongNameDigestGenerate(&self, wszfilepath: &::windows_core::PCWSTR, ppbdigestblob: *mut *mut u8, pcbdigestblob: *mut u32, dwflags: u32) -> ::windows_core::Result<()>;
    fn StrongNameDigestSign(&self, wszkeycontainer: &::windows_core::PCWSTR, pbkeyblob: *const u8, cbkeyblob: u32, pbdigestblob: *const u8, cbdigestblob: u32, hashalgid: u32, ppbsignatureblob: *mut *mut u8, pcbsignatureblob: *mut u32, dwflags: u32) -> ::windows_core::Result<()>;
    fn StrongNameDigestEmbed(&self, wszfilepath: &::windows_core::PCWSTR, pbsignatureblob: *const u8, cbsignatureblob: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ICLRStrongName3 {}
impl ICLRStrongName3_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRStrongName3_Impl, const OFFSET: isize>() -> ICLRStrongName3_Vtbl {
        unsafe extern "system" fn StrongNameDigestGenerate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRStrongName3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszfilepath: ::windows_core::PCWSTR, ppbdigestblob: *mut *mut u8, pcbdigestblob: *mut u32, dwflags: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StrongNameDigestGenerate(::core::mem::transmute(&wszfilepath), ::core::mem::transmute_copy(&ppbdigestblob), ::core::mem::transmute_copy(&pcbdigestblob), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn StrongNameDigestSign<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRStrongName3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszkeycontainer: ::windows_core::PCWSTR, pbkeyblob: *const u8, cbkeyblob: u32, pbdigestblob: *const u8, cbdigestblob: u32, hashalgid: u32, ppbsignatureblob: *mut *mut u8, pcbsignatureblob: *mut u32, dwflags: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StrongNameDigestSign(::core::mem::transmute(&wszkeycontainer), ::core::mem::transmute_copy(&pbkeyblob), ::core::mem::transmute_copy(&cbkeyblob), ::core::mem::transmute_copy(&pbdigestblob), ::core::mem::transmute_copy(&cbdigestblob), ::core::mem::transmute_copy(&hashalgid), ::core::mem::transmute_copy(&ppbsignatureblob), ::core::mem::transmute_copy(&pcbsignatureblob), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn StrongNameDigestEmbed<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRStrongName3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszfilepath: ::windows_core::PCWSTR, pbsignatureblob: *const u8, cbsignatureblob: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StrongNameDigestEmbed(::core::mem::transmute(&wszfilepath), ::core::mem::transmute_copy(&pbsignatureblob), ::core::mem::transmute_copy(&cbsignatureblob)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            StrongNameDigestGenerate: StrongNameDigestGenerate::<Identity, Impl, OFFSET>,
            StrongNameDigestSign: StrongNameDigestSign::<Identity, Impl, OFFSET>,
            StrongNameDigestEmbed: StrongNameDigestEmbed::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ICLRStrongName3 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait ICLRSyncManager_Impl: Sized {
    fn GetMonitorOwner(&self, cookie: usize) -> ::windows_core::Result<IHostTask>;
    fn CreateRWLockOwnerIterator(&self, cookie: usize) -> ::windows_core::Result<usize>;
    fn GetRWLockOwnerNext(&self, iterator: usize) -> ::windows_core::Result<IHostTask>;
    fn DeleteRWLockOwnerIterator(&self, iterator: usize) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ICLRSyncManager {}
impl ICLRSyncManager_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRSyncManager_Impl, const OFFSET: isize>() -> ICLRSyncManager_Vtbl {
        unsafe extern "system" fn GetMonitorOwner<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRSyncManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: usize, ppownerhosttask: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMonitorOwner(::core::mem::transmute_copy(&cookie)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppownerhosttask, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRWLockOwnerIterator<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRSyncManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: usize, piterator: *mut usize) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateRWLockOwnerIterator(::core::mem::transmute_copy(&cookie)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(piterator, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRWLockOwnerNext<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRSyncManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iterator: usize, ppownerhosttask: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRWLockOwnerNext(::core::mem::transmute_copy(&iterator)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppownerhosttask, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteRWLockOwnerIterator<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRSyncManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iterator: usize) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteRWLockOwnerIterator(::core::mem::transmute_copy(&iterator)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetMonitorOwner: GetMonitorOwner::<Identity, Impl, OFFSET>,
            CreateRWLockOwnerIterator: CreateRWLockOwnerIterator::<Identity, Impl, OFFSET>,
            GetRWLockOwnerNext: GetRWLockOwnerNext::<Identity, Impl, OFFSET>,
            DeleteRWLockOwnerIterator: DeleteRWLockOwnerIterator::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ICLRSyncManager as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICLRTask_Impl: Sized {
    fn SwitchIn(&self, threadhandle: super::super::Foundation::HANDLE) -> ::windows_core::Result<()>;
    fn SwitchOut(&self) -> ::windows_core::Result<()>;
    fn GetMemStats(&self) -> ::windows_core::Result<COR_GC_THREAD_STATS>;
    fn Reset(&self, ffull: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn ExitTask(&self) -> ::windows_core::Result<()>;
    fn Abort(&self) -> ::windows_core::Result<()>;
    fn RudeAbort(&self) -> ::windows_core::Result<()>;
    fn NeedsPriorityScheduling(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn YieldTask(&self) -> ::windows_core::Result<()>;
    fn LocksHeld(&self) -> ::windows_core::Result<usize>;
    fn SetTaskIdentifier(&self, asked: u64) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ICLRTask {}
#[cfg(feature = "Win32_Foundation")]
impl ICLRTask_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRTask_Impl, const OFFSET: isize>() -> ICLRTask_Vtbl {
        unsafe extern "system" fn SwitchIn<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, threadhandle: super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SwitchIn(::core::mem::transmute_copy(&threadhandle)).into()
        }
        unsafe extern "system" fn SwitchOut<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SwitchOut().into()
        }
        unsafe extern "system" fn GetMemStats<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, memusage: *mut COR_GC_THREAD_STATS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMemStats() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(memusage, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ffull: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset(::core::mem::transmute_copy(&ffull)).into()
        }
        unsafe extern "system" fn ExitTask<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ExitTask().into()
        }
        unsafe extern "system" fn Abort<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Abort().into()
        }
        unsafe extern "system" fn RudeAbort<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RudeAbort().into()
        }
        unsafe extern "system" fn NeedsPriorityScheduling<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbneedspriorityscheduling: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NeedsPriorityScheduling() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbneedspriorityscheduling, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn YieldTask<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.YieldTask().into()
        }
        unsafe extern "system" fn LocksHeld<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plockcount: *mut usize) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LocksHeld() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plockcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTaskIdentifier<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, asked: u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTaskIdentifier(::core::mem::transmute_copy(&asked)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SwitchIn: SwitchIn::<Identity, Impl, OFFSET>,
            SwitchOut: SwitchOut::<Identity, Impl, OFFSET>,
            GetMemStats: GetMemStats::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            ExitTask: ExitTask::<Identity, Impl, OFFSET>,
            Abort: Abort::<Identity, Impl, OFFSET>,
            RudeAbort: RudeAbort::<Identity, Impl, OFFSET>,
            NeedsPriorityScheduling: NeedsPriorityScheduling::<Identity, Impl, OFFSET>,
            YieldTask: YieldTask::<Identity, Impl, OFFSET>,
            LocksHeld: LocksHeld::<Identity, Impl, OFFSET>,
            SetTaskIdentifier: SetTaskIdentifier::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ICLRTask as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICLRTask2_Impl: Sized + ICLRTask_Impl {
    fn BeginPreventAsyncAbort(&self) -> ::windows_core::Result<()>;
    fn EndPreventAsyncAbort(&self) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ICLRTask2 {}
#[cfg(feature = "Win32_Foundation")]
impl ICLRTask2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRTask2_Impl, const OFFSET: isize>() -> ICLRTask2_Vtbl {
        unsafe extern "system" fn BeginPreventAsyncAbort<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRTask2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BeginPreventAsyncAbort().into()
        }
        unsafe extern "system" fn EndPreventAsyncAbort<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRTask2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndPreventAsyncAbort().into()
        }
        Self {
            base__: ICLRTask_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginPreventAsyncAbort: BeginPreventAsyncAbort::<Identity, Impl, OFFSET>,
            EndPreventAsyncAbort: EndPreventAsyncAbort::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ICLRTask2 as ::windows_core::ComInterface>::IID || iid == &<ICLRTask as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait ICLRTaskManager_Impl: Sized {
    fn CreateTask(&self) -> ::windows_core::Result<ICLRTask>;
    fn GetCurrentTask(&self) -> ::windows_core::Result<ICLRTask>;
    fn SetUILocale(&self, lcid: u32) -> ::windows_core::Result<()>;
    fn SetLocale(&self, lcid: u32) -> ::windows_core::Result<()>;
    fn GetCurrentTaskType(&self) -> ::windows_core::Result<ETaskType>;
}
impl ::windows_core::RuntimeName for ICLRTaskManager {}
impl ICLRTaskManager_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRTaskManager_Impl, const OFFSET: isize>() -> ICLRTaskManager_Vtbl {
        unsafe extern "system" fn CreateTask<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRTaskManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptask: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateTask() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptask, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentTask<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRTaskManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptask: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurrentTask() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptask, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUILocale<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRTaskManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcid: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetUILocale(::core::mem::transmute_copy(&lcid)).into()
        }
        unsafe extern "system" fn SetLocale<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRTaskManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcid: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLocale(::core::mem::transmute_copy(&lcid)).into()
        }
        unsafe extern "system" fn GetCurrentTaskType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICLRTaskManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptasktype: *mut ETaskType) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurrentTaskType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptasktype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateTask: CreateTask::<Identity, Impl, OFFSET>,
            GetCurrentTask: GetCurrentTask::<Identity, Impl, OFFSET>,
            SetUILocale: SetUILocale::<Identity, Impl, OFFSET>,
            SetLocale: SetLocale::<Identity, Impl, OFFSET>,
            GetCurrentTaskType: GetCurrentTaskType::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ICLRTaskManager as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait ICatalogServices_Impl: Sized {
    fn Autodone(&self) -> ::windows_core::Result<()>;
    fn NotAutodone(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ICatalogServices {}
impl ICatalogServices_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICatalogServices_Impl, const OFFSET: isize>() -> ICatalogServices_Vtbl {
        unsafe extern "system" fn Autodone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICatalogServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Autodone().into()
        }
        unsafe extern "system" fn NotAutodone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICatalogServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.NotAutodone().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Autodone: Autodone::<Identity, Impl, OFFSET>,
            NotAutodone: NotAutodone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ICatalogServices as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait ICorConfiguration_Impl: Sized {
    fn SetGCThreadControl(&self, pgcthreadcontrol: ::core::option::Option<&IGCThreadControl>) -> ::windows_core::Result<()>;
    fn SetGCHostControl(&self, pgchostcontrol: ::core::option::Option<&IGCHostControl>) -> ::windows_core::Result<()>;
    fn SetDebuggerThreadControl(&self, pdebuggerthreadcontrol: ::core::option::Option<&IDebuggerThreadControl>) -> ::windows_core::Result<()>;
    fn AddDebuggerSpecialThread(&self, dwspecialthreadid: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ICorConfiguration {}
impl ICorConfiguration_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICorConfiguration_Impl, const OFFSET: isize>() -> ICorConfiguration_Vtbl {
        unsafe extern "system" fn SetGCThreadControl<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICorConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgcthreadcontrol: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetGCThreadControl(::windows_core::from_raw_borrowed(&pgcthreadcontrol)).into()
        }
        unsafe extern "system" fn SetGCHostControl<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICorConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgchostcontrol: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetGCHostControl(::windows_core::from_raw_borrowed(&pgchostcontrol)).into()
        }
        unsafe extern "system" fn SetDebuggerThreadControl<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICorConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdebuggerthreadcontrol: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDebuggerThreadControl(::windows_core::from_raw_borrowed(&pdebuggerthreadcontrol)).into()
        }
        unsafe extern "system" fn AddDebuggerSpecialThread<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICorConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwspecialthreadid: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddDebuggerSpecialThread(::core::mem::transmute_copy(&dwspecialthreadid)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetGCThreadControl: SetGCThreadControl::<Identity, Impl, OFFSET>,
            SetGCHostControl: SetGCHostControl::<Identity, Impl, OFFSET>,
            SetDebuggerThreadControl: SetDebuggerThreadControl::<Identity, Impl, OFFSET>,
            AddDebuggerSpecialThread: AddDebuggerSpecialThread::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ICorConfiguration as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICorRuntimeHost_Impl: Sized {
    fn CreateLogicalThreadState(&self) -> ::windows_core::Result<()>;
    fn DeleteLogicalThreadState(&self) -> ::windows_core::Result<()>;
    fn SwitchInLogicalThreadState(&self, pfibercookie: *const u32) -> ::windows_core::Result<()>;
    fn SwitchOutLogicalThreadState(&self) -> ::windows_core::Result<*mut u32>;
    fn LocksHeldByLogicalThread(&self) -> ::windows_core::Result<u32>;
    fn MapFile(&self, hfile: super::super::Foundation::HANDLE) -> ::windows_core::Result<super::super::Foundation::HMODULE>;
    fn GetConfiguration(&self) -> ::windows_core::Result<ICorConfiguration>;
    fn Start(&self) -> ::windows_core::Result<()>;
    fn Stop(&self) -> ::windows_core::Result<()>;
    fn CreateDomain(&self, pwzfriendlyname: &::windows_core::PCWSTR, pidentityarray: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn GetDefaultDomain(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn EnumDomains(&self, henum: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn NextDomain(&self, henum: *const ::core::ffi::c_void) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn CloseEnum(&self, henum: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn CreateDomainEx(&self, pwzfriendlyname: &::windows_core::PCWSTR, psetup: ::core::option::Option<&::windows_core::IUnknown>, pevidence: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn CreateDomainSetup(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn CreateEvidence(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn UnloadDomain(&self, pappdomain: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn CurrentDomain(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ICorRuntimeHost {}
#[cfg(feature = "Win32_Foundation")]
impl ICorRuntimeHost_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICorRuntimeHost_Impl, const OFFSET: isize>() -> ICorRuntimeHost_Vtbl {
        unsafe extern "system" fn CreateLogicalThreadState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICorRuntimeHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateLogicalThreadState().into()
        }
        unsafe extern "system" fn DeleteLogicalThreadState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICorRuntimeHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteLogicalThreadState().into()
        }
        unsafe extern "system" fn SwitchInLogicalThreadState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICorRuntimeHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfibercookie: *const u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SwitchInLogicalThreadState(::core::mem::transmute_copy(&pfibercookie)).into()
        }
        unsafe extern "system" fn SwitchOutLogicalThreadState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICorRuntimeHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfibercookie: *mut *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SwitchOutLogicalThreadState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfibercookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocksHeldByLogicalThread<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICorRuntimeHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LocksHeldByLogicalThread() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MapFile<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICorRuntimeHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hfile: super::super::Foundation::HANDLE, hmapaddress: *mut super::super::Foundation::HMODULE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MapFile(::core::mem::transmute_copy(&hfile)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hmapaddress, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConfiguration<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICorRuntimeHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconfiguration: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetConfiguration() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pconfiguration, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Start<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICorRuntimeHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Start().into()
        }
        unsafe extern "system" fn Stop<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICorRuntimeHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Stop().into()
        }
        unsafe extern "system" fn CreateDomain<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICorRuntimeHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzfriendlyname: ::windows_core::PCWSTR, pidentityarray: *mut ::core::ffi::c_void, pappdomain: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateDomain(::core::mem::transmute(&pwzfriendlyname), ::windows_core::from_raw_borrowed(&pidentityarray)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pappdomain, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultDomain<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICorRuntimeHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pappdomain: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDefaultDomain() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pappdomain, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumDomains<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICorRuntimeHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, henum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumDomains(::core::mem::transmute_copy(&henum)).into()
        }
        unsafe extern "system" fn NextDomain<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICorRuntimeHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, henum: *const ::core::ffi::c_void, pappdomain: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NextDomain(::core::mem::transmute_copy(&henum)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pappdomain, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CloseEnum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICorRuntimeHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, henum: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CloseEnum(::core::mem::transmute_copy(&henum)).into()
        }
        unsafe extern "system" fn CreateDomainEx<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICorRuntimeHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzfriendlyname: ::windows_core::PCWSTR, psetup: *mut ::core::ffi::c_void, pevidence: *mut ::core::ffi::c_void, pappdomain: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateDomainEx(::core::mem::transmute(&pwzfriendlyname), ::windows_core::from_raw_borrowed(&psetup), ::windows_core::from_raw_borrowed(&pevidence)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pappdomain, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDomainSetup<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICorRuntimeHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pappdomainsetup: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateDomainSetup() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pappdomainsetup, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEvidence<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICorRuntimeHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pevidence: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateEvidence() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pevidence, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnloadDomain<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICorRuntimeHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pappdomain: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnloadDomain(::windows_core::from_raw_borrowed(&pappdomain)).into()
        }
        unsafe extern "system" fn CurrentDomain<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICorRuntimeHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pappdomain: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentDomain() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pappdomain, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateLogicalThreadState: CreateLogicalThreadState::<Identity, Impl, OFFSET>,
            DeleteLogicalThreadState: DeleteLogicalThreadState::<Identity, Impl, OFFSET>,
            SwitchInLogicalThreadState: SwitchInLogicalThreadState::<Identity, Impl, OFFSET>,
            SwitchOutLogicalThreadState: SwitchOutLogicalThreadState::<Identity, Impl, OFFSET>,
            LocksHeldByLogicalThread: LocksHeldByLogicalThread::<Identity, Impl, OFFSET>,
            MapFile: MapFile::<Identity, Impl, OFFSET>,
            GetConfiguration: GetConfiguration::<Identity, Impl, OFFSET>,
            Start: Start::<Identity, Impl, OFFSET>,
            Stop: Stop::<Identity, Impl, OFFSET>,
            CreateDomain: CreateDomain::<Identity, Impl, OFFSET>,
            GetDefaultDomain: GetDefaultDomain::<Identity, Impl, OFFSET>,
            EnumDomains: EnumDomains::<Identity, Impl, OFFSET>,
            NextDomain: NextDomain::<Identity, Impl, OFFSET>,
            CloseEnum: CloseEnum::<Identity, Impl, OFFSET>,
            CreateDomainEx: CreateDomainEx::<Identity, Impl, OFFSET>,
            CreateDomainSetup: CreateDomainSetup::<Identity, Impl, OFFSET>,
            CreateEvidence: CreateEvidence::<Identity, Impl, OFFSET>,
            UnloadDomain: UnloadDomain::<Identity, Impl, OFFSET>,
            CurrentDomain: CurrentDomain::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ICorRuntimeHost as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`, `\"Win32_System_Threading\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO", feature = "Win32_System_Threading"))]
pub trait ICorThreadpool_Impl: Sized {
    fn CorRegisterWaitForSingleObject(&self, phnewwaitobject: *const super::super::Foundation::HANDLE, hwaitobject: super::super::Foundation::HANDLE, callback: super::Threading::WAITORTIMERCALLBACK, context: *const ::core::ffi::c_void, timeout: u32, executeonlyonce: super::super::Foundation::BOOL) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CorUnregisterWait(&self, hwaitobject: super::super::Foundation::HANDLE, completionevent: super::super::Foundation::HANDLE) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CorQueueUserWorkItem(&self, function: super::Threading::LPTHREAD_START_ROUTINE, context: *const ::core::ffi::c_void, executeonlyonce: super::super::Foundation::BOOL) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CorCreateTimer(&self, phnewtimer: *const super::super::Foundation::HANDLE, callback: super::Threading::WAITORTIMERCALLBACK, parameter: *const ::core::ffi::c_void, duetime: u32, period: u32) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CorChangeTimer(&self, timer: super::super::Foundation::HANDLE, duetime: u32, period: u32) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CorDeleteTimer(&self, timer: super::super::Foundation::HANDLE, completionevent: super::super::Foundation::HANDLE) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CorBindIoCompletionCallback(&self, filehandle: super::super::Foundation::HANDLE, callback: super::IO::LPOVERLAPPED_COMPLETION_ROUTINE) -> ::windows_core::Result<()>;
    fn CorCallOrQueueUserWorkItem(&self, function: super::Threading::LPTHREAD_START_ROUTINE, context: *const ::core::ffi::c_void) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CorSetMaxThreads(&self, maxworkerthreads: u32, maxiocompletionthreads: u32) -> ::windows_core::Result<()>;
    fn CorGetMaxThreads(&self, maxworkerthreads: *mut u32, maxiocompletionthreads: *mut u32) -> ::windows_core::Result<()>;
    fn CorGetAvailableThreads(&self, availableworkerthreads: *mut u32, availableiocompletionthreads: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO", feature = "Win32_System_Threading"))]
impl ::windows_core::RuntimeName for ICorThreadpool {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO", feature = "Win32_System_Threading"))]
impl ICorThreadpool_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICorThreadpool_Impl, const OFFSET: isize>() -> ICorThreadpool_Vtbl {
        unsafe extern "system" fn CorRegisterWaitForSingleObject<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICorThreadpool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phnewwaitobject: *const super::super::Foundation::HANDLE, hwaitobject: super::super::Foundation::HANDLE, callback: super::Threading::WAITORTIMERCALLBACK, context: *const ::core::ffi::c_void, timeout: u32, executeonlyonce: super::super::Foundation::BOOL, result: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CorRegisterWaitForSingleObject(::core::mem::transmute_copy(&phnewwaitobject), ::core::mem::transmute_copy(&hwaitobject), ::core::mem::transmute_copy(&callback), ::core::mem::transmute_copy(&context), ::core::mem::transmute_copy(&timeout), ::core::mem::transmute_copy(&executeonlyonce)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CorUnregisterWait<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICorThreadpool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwaitobject: super::super::Foundation::HANDLE, completionevent: super::super::Foundation::HANDLE, result: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CorUnregisterWait(::core::mem::transmute_copy(&hwaitobject), ::core::mem::transmute_copy(&completionevent)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CorQueueUserWorkItem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICorThreadpool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, function: super::Threading::LPTHREAD_START_ROUTINE, context: *const ::core::ffi::c_void, executeonlyonce: super::super::Foundation::BOOL, result: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CorQueueUserWorkItem(::core::mem::transmute_copy(&function), ::core::mem::transmute_copy(&context), ::core::mem::transmute_copy(&executeonlyonce)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CorCreateTimer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICorThreadpool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phnewtimer: *const super::super::Foundation::HANDLE, callback: super::Threading::WAITORTIMERCALLBACK, parameter: *const ::core::ffi::c_void, duetime: u32, period: u32, result: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CorCreateTimer(::core::mem::transmute_copy(&phnewtimer), ::core::mem::transmute_copy(&callback), ::core::mem::transmute_copy(&parameter), ::core::mem::transmute_copy(&duetime), ::core::mem::transmute_copy(&period)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CorChangeTimer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICorThreadpool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timer: super::super::Foundation::HANDLE, duetime: u32, period: u32, result: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CorChangeTimer(::core::mem::transmute_copy(&timer), ::core::mem::transmute_copy(&duetime), ::core::mem::transmute_copy(&period)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CorDeleteTimer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICorThreadpool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timer: super::super::Foundation::HANDLE, completionevent: super::super::Foundation::HANDLE, result: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CorDeleteTimer(::core::mem::transmute_copy(&timer), ::core::mem::transmute_copy(&completionevent)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CorBindIoCompletionCallback<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICorThreadpool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filehandle: super::super::Foundation::HANDLE, callback: super::IO::LPOVERLAPPED_COMPLETION_ROUTINE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CorBindIoCompletionCallback(::core::mem::transmute_copy(&filehandle), ::core::mem::transmute_copy(&callback)).into()
        }
        unsafe extern "system" fn CorCallOrQueueUserWorkItem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICorThreadpool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, function: super::Threading::LPTHREAD_START_ROUTINE, context: *const ::core::ffi::c_void, result: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CorCallOrQueueUserWorkItem(::core::mem::transmute_copy(&function), ::core::mem::transmute_copy(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CorSetMaxThreads<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICorThreadpool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxworkerthreads: u32, maxiocompletionthreads: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CorSetMaxThreads(::core::mem::transmute_copy(&maxworkerthreads), ::core::mem::transmute_copy(&maxiocompletionthreads)).into()
        }
        unsafe extern "system" fn CorGetMaxThreads<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICorThreadpool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxworkerthreads: *mut u32, maxiocompletionthreads: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CorGetMaxThreads(::core::mem::transmute_copy(&maxworkerthreads), ::core::mem::transmute_copy(&maxiocompletionthreads)).into()
        }
        unsafe extern "system" fn CorGetAvailableThreads<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICorThreadpool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, availableworkerthreads: *mut u32, availableiocompletionthreads: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CorGetAvailableThreads(::core::mem::transmute_copy(&availableworkerthreads), ::core::mem::transmute_copy(&availableiocompletionthreads)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CorRegisterWaitForSingleObject: CorRegisterWaitForSingleObject::<Identity, Impl, OFFSET>,
            CorUnregisterWait: CorUnregisterWait::<Identity, Impl, OFFSET>,
            CorQueueUserWorkItem: CorQueueUserWorkItem::<Identity, Impl, OFFSET>,
            CorCreateTimer: CorCreateTimer::<Identity, Impl, OFFSET>,
            CorChangeTimer: CorChangeTimer::<Identity, Impl, OFFSET>,
            CorDeleteTimer: CorDeleteTimer::<Identity, Impl, OFFSET>,
            CorBindIoCompletionCallback: CorBindIoCompletionCallback::<Identity, Impl, OFFSET>,
            CorCallOrQueueUserWorkItem: CorCallOrQueueUserWorkItem::<Identity, Impl, OFFSET>,
            CorSetMaxThreads: CorSetMaxThreads::<Identity, Impl, OFFSET>,
            CorGetMaxThreads: CorGetMaxThreads::<Identity, Impl, OFFSET>,
            CorGetAvailableThreads: CorGetAvailableThreads::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ICorThreadpool as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDebuggerInfo_Impl: Sized {
    fn IsDebuggerAttached(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IDebuggerInfo {}
#[cfg(feature = "Win32_Foundation")]
impl IDebuggerInfo_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebuggerInfo_Impl, const OFFSET: isize>() -> IDebuggerInfo_Vtbl {
        unsafe extern "system" fn IsDebuggerAttached<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebuggerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbattached: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsDebuggerAttached() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbattached, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), IsDebuggerAttached: IsDebuggerAttached::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDebuggerInfo as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait IDebuggerThreadControl_Impl: Sized {
    fn ThreadIsBlockingForDebugger(&self) -> ::windows_core::Result<()>;
    fn ReleaseAllRuntimeThreads(&self) -> ::windows_core::Result<()>;
    fn StartBlockingForDebugger(&self, dwunused: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IDebuggerThreadControl {}
impl IDebuggerThreadControl_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebuggerThreadControl_Impl, const OFFSET: isize>() -> IDebuggerThreadControl_Vtbl {
        unsafe extern "system" fn ThreadIsBlockingForDebugger<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebuggerThreadControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ThreadIsBlockingForDebugger().into()
        }
        unsafe extern "system" fn ReleaseAllRuntimeThreads<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebuggerThreadControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleaseAllRuntimeThreads().into()
        }
        unsafe extern "system" fn StartBlockingForDebugger<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebuggerThreadControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwunused: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StartBlockingForDebugger(::core::mem::transmute_copy(&dwunused)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ThreadIsBlockingForDebugger: ThreadIsBlockingForDebugger::<Identity, Impl, OFFSET>,
            ReleaseAllRuntimeThreads: ReleaseAllRuntimeThreads::<Identity, Impl, OFFSET>,
            StartBlockingForDebugger: StartBlockingForDebugger::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDebuggerThreadControl as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait IGCHost_Impl: Sized {
    fn SetGCStartupLimits(&self, segmentsize: u32, maxgen0size: u32) -> ::windows_core::Result<()>;
    fn Collect(&self, generation: i32) -> ::windows_core::Result<()>;
    fn GetStats(&self, pstats: *mut COR_GC_STATS) -> ::windows_core::Result<()>;
    fn GetThreadStats(&self, pfibercookie: *const u32, pstats: *mut COR_GC_THREAD_STATS) -> ::windows_core::Result<()>;
    fn SetVirtualMemLimit(&self, sztmaxvirtualmemmb: usize) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IGCHost {}
impl IGCHost_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IGCHost_Impl, const OFFSET: isize>() -> IGCHost_Vtbl {
        unsafe extern "system" fn SetGCStartupLimits<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IGCHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, segmentsize: u32, maxgen0size: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetGCStartupLimits(::core::mem::transmute_copy(&segmentsize), ::core::mem::transmute_copy(&maxgen0size)).into()
        }
        unsafe extern "system" fn Collect<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IGCHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, generation: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Collect(::core::mem::transmute_copy(&generation)).into()
        }
        unsafe extern "system" fn GetStats<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IGCHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstats: *mut COR_GC_STATS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetStats(::core::mem::transmute_copy(&pstats)).into()
        }
        unsafe extern "system" fn GetThreadStats<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IGCHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfibercookie: *const u32, pstats: *mut COR_GC_THREAD_STATS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetThreadStats(::core::mem::transmute_copy(&pfibercookie), ::core::mem::transmute_copy(&pstats)).into()
        }
        unsafe extern "system" fn SetVirtualMemLimit<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IGCHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sztmaxvirtualmemmb: usize) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetVirtualMemLimit(::core::mem::transmute_copy(&sztmaxvirtualmemmb)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetGCStartupLimits: SetGCStartupLimits::<Identity, Impl, OFFSET>,
            Collect: Collect::<Identity, Impl, OFFSET>,
            GetStats: GetStats::<Identity, Impl, OFFSET>,
            GetThreadStats: GetThreadStats::<Identity, Impl, OFFSET>,
            SetVirtualMemLimit: SetVirtualMemLimit::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IGCHost as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait IGCHost2_Impl: Sized + IGCHost_Impl {
    fn SetGCStartupLimitsEx(&self, segmentsize: usize, maxgen0size: usize) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IGCHost2 {}
impl IGCHost2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IGCHost2_Impl, const OFFSET: isize>() -> IGCHost2_Vtbl {
        unsafe extern "system" fn SetGCStartupLimitsEx<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IGCHost2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, segmentsize: usize, maxgen0size: usize) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetGCStartupLimitsEx(::core::mem::transmute_copy(&segmentsize), ::core::mem::transmute_copy(&maxgen0size)).into()
        }
        Self { base__: IGCHost_Vtbl::new::<Identity, Impl, OFFSET>(), SetGCStartupLimitsEx: SetGCStartupLimitsEx::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IGCHost2 as ::windows_core::ComInterface>::IID || iid == &<IGCHost as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait IGCHostControl_Impl: Sized {
    fn RequestVirtualMemLimit(&self, sztmaxvirtualmemmb: usize, psztnewmaxvirtualmemmb: *mut usize) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IGCHostControl {}
impl IGCHostControl_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IGCHostControl_Impl, const OFFSET: isize>() -> IGCHostControl_Vtbl {
        unsafe extern "system" fn RequestVirtualMemLimit<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IGCHostControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sztmaxvirtualmemmb: usize, psztnewmaxvirtualmemmb: *mut usize) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RequestVirtualMemLimit(::core::mem::transmute_copy(&sztmaxvirtualmemmb), ::core::mem::transmute_copy(&psztnewmaxvirtualmemmb)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), RequestVirtualMemLimit: RequestVirtualMemLimit::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IGCHostControl as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait IGCThreadControl_Impl: Sized {
    fn ThreadIsBlockingForSuspension(&self) -> ::windows_core::Result<()>;
    fn SuspensionStarting(&self) -> ::windows_core::Result<()>;
    fn SuspensionEnding(&self, generation: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IGCThreadControl {}
impl IGCThreadControl_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IGCThreadControl_Impl, const OFFSET: isize>() -> IGCThreadControl_Vtbl {
        unsafe extern "system" fn ThreadIsBlockingForSuspension<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IGCThreadControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ThreadIsBlockingForSuspension().into()
        }
        unsafe extern "system" fn SuspensionStarting<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IGCThreadControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SuspensionStarting().into()
        }
        unsafe extern "system" fn SuspensionEnding<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IGCThreadControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, generation: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SuspensionEnding(::core::mem::transmute_copy(&generation)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ThreadIsBlockingForSuspension: ThreadIsBlockingForSuspension::<Identity, Impl, OFFSET>,
            SuspensionStarting: SuspensionStarting::<Identity, Impl, OFFSET>,
            SuspensionEnding: SuspensionEnding::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IGCThreadControl as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait IHostAssemblyManager_Impl: Sized {
    fn GetNonHostStoreAssemblies(&self) -> ::windows_core::Result<ICLRAssemblyReferenceList>;
    fn GetAssemblyStore(&self) -> ::windows_core::Result<IHostAssemblyStore>;
}
impl ::windows_core::RuntimeName for IHostAssemblyManager {}
impl IHostAssemblyManager_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostAssemblyManager_Impl, const OFFSET: isize>() -> IHostAssemblyManager_Vtbl {
        unsafe extern "system" fn GetNonHostStoreAssemblies<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostAssemblyManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppreferencelist: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetNonHostStoreAssemblies() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppreferencelist, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAssemblyStore<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostAssemblyManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppassemblystore: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAssemblyStore() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppassemblystore, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetNonHostStoreAssemblies: GetNonHostStoreAssemblies::<Identity, Impl, OFFSET>,
            GetAssemblyStore: GetAssemblyStore::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IHostAssemblyManager as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IHostAssemblyStore_Impl: Sized {
    fn ProvideAssembly(&self, pbindinfo: *const AssemblyBindInfo, passemblyid: *mut u64, pcontext: *mut u64, ppstmassemblyimage: *mut ::core::option::Option<super::Com::IStream>, ppstmpdb: *mut ::core::option::Option<super::Com::IStream>) -> ::windows_core::Result<()>;
    fn ProvideModule(&self, pbindinfo: *const ModuleBindInfo, pdwmoduleid: *mut u32, ppstmmoduleimage: *mut ::core::option::Option<super::Com::IStream>, ppstmpdb: *mut ::core::option::Option<super::Com::IStream>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IHostAssemblyStore {}
#[cfg(feature = "Win32_System_Com")]
impl IHostAssemblyStore_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostAssemblyStore_Impl, const OFFSET: isize>() -> IHostAssemblyStore_Vtbl {
        unsafe extern "system" fn ProvideAssembly<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostAssemblyStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbindinfo: *const AssemblyBindInfo, passemblyid: *mut u64, pcontext: *mut u64, ppstmassemblyimage: *mut *mut ::core::ffi::c_void, ppstmpdb: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ProvideAssembly(::core::mem::transmute_copy(&pbindinfo), ::core::mem::transmute_copy(&passemblyid), ::core::mem::transmute_copy(&pcontext), ::core::mem::transmute_copy(&ppstmassemblyimage), ::core::mem::transmute_copy(&ppstmpdb)).into()
        }
        unsafe extern "system" fn ProvideModule<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostAssemblyStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbindinfo: *const ModuleBindInfo, pdwmoduleid: *mut u32, ppstmmoduleimage: *mut *mut ::core::ffi::c_void, ppstmpdb: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ProvideModule(::core::mem::transmute_copy(&pbindinfo), ::core::mem::transmute_copy(&pdwmoduleid), ::core::mem::transmute_copy(&ppstmmoduleimage), ::core::mem::transmute_copy(&ppstmpdb)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ProvideAssembly: ProvideAssembly::<Identity, Impl, OFFSET>,
            ProvideModule: ProvideModule::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IHostAssemblyStore as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait IHostAutoEvent_Impl: Sized {
    fn Wait(&self, dwmilliseconds: u32, option: u32) -> ::windows_core::Result<()>;
    fn Set(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IHostAutoEvent {}
impl IHostAutoEvent_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostAutoEvent_Impl, const OFFSET: isize>() -> IHostAutoEvent_Vtbl {
        unsafe extern "system" fn Wait<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostAutoEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmilliseconds: u32, option: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Wait(::core::mem::transmute_copy(&dwmilliseconds), ::core::mem::transmute_copy(&option)).into()
        }
        unsafe extern "system" fn Set<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostAutoEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Set().into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Wait: Wait::<Identity, Impl, OFFSET>, Set: Set::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IHostAutoEvent as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait IHostControl_Impl: Sized {
    fn GetHostManager(&self, riid: *const ::windows_core::GUID, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn SetAppDomainManager(&self, dwappdomainid: u32, punkappdomainmanager: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IHostControl {}
impl IHostControl_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostControl_Impl, const OFFSET: isize>() -> IHostControl_Vtbl {
        unsafe extern "system" fn GetHostManager<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetHostManager(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppobject)).into()
        }
        unsafe extern "system" fn SetAppDomainManager<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwappdomainid: u32, punkappdomainmanager: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAppDomainManager(::core::mem::transmute_copy(&dwappdomainid), ::windows_core::from_raw_borrowed(&punkappdomainmanager)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetHostManager: GetHostManager::<Identity, Impl, OFFSET>,
            SetAppDomainManager: SetAppDomainManager::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IHostControl as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IHostCrst_Impl: Sized {
    fn Enter(&self, option: u32) -> ::windows_core::Result<()>;
    fn Leave(&self) -> ::windows_core::Result<()>;
    fn TryEnter(&self, option: u32) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetSpinCount(&self, dwspincount: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IHostCrst {}
#[cfg(feature = "Win32_Foundation")]
impl IHostCrst_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostCrst_Impl, const OFFSET: isize>() -> IHostCrst_Vtbl {
        unsafe extern "system" fn Enter<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostCrst_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, option: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Enter(::core::mem::transmute_copy(&option)).into()
        }
        unsafe extern "system" fn Leave<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostCrst_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Leave().into()
        }
        unsafe extern "system" fn TryEnter<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostCrst_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, option: u32, pbsucceeded: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TryEnter(::core::mem::transmute_copy(&option)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbsucceeded, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSpinCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostCrst_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwspincount: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSpinCount(::core::mem::transmute_copy(&dwspincount)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Enter: Enter::<Identity, Impl, OFFSET>,
            Leave: Leave::<Identity, Impl, OFFSET>,
            TryEnter: TryEnter::<Identity, Impl, OFFSET>,
            SetSpinCount: SetSpinCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IHostCrst as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait IHostGCManager_Impl: Sized {
    fn ThreadIsBlockingForSuspension(&self) -> ::windows_core::Result<()>;
    fn SuspensionStarting(&self) -> ::windows_core::Result<()>;
    fn SuspensionEnding(&self, generation: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IHostGCManager {}
impl IHostGCManager_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostGCManager_Impl, const OFFSET: isize>() -> IHostGCManager_Vtbl {
        unsafe extern "system" fn ThreadIsBlockingForSuspension<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostGCManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ThreadIsBlockingForSuspension().into()
        }
        unsafe extern "system" fn SuspensionStarting<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostGCManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SuspensionStarting().into()
        }
        unsafe extern "system" fn SuspensionEnding<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostGCManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, generation: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SuspensionEnding(::core::mem::transmute_copy(&generation)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ThreadIsBlockingForSuspension: ThreadIsBlockingForSuspension::<Identity, Impl, OFFSET>,
            SuspensionStarting: SuspensionStarting::<Identity, Impl, OFFSET>,
            SuspensionEnding: SuspensionEnding::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IHostGCManager as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IHostIoCompletionManager_Impl: Sized {
    fn CreateIoCompletionPort(&self) -> ::windows_core::Result<super::super::Foundation::HANDLE>;
    fn CloseIoCompletionPort(&self, hport: super::super::Foundation::HANDLE) -> ::windows_core::Result<()>;
    fn SetMaxThreads(&self, dwmaxiocompletionthreads: u32) -> ::windows_core::Result<()>;
    fn GetMaxThreads(&self) -> ::windows_core::Result<u32>;
    fn GetAvailableThreads(&self) -> ::windows_core::Result<u32>;
    fn GetHostOverlappedSize(&self) -> ::windows_core::Result<u32>;
    fn SetCLRIoCompletionManager(&self, pmanager: ::core::option::Option<&ICLRIoCompletionManager>) -> ::windows_core::Result<()>;
    fn InitializeHostOverlapped(&self, pvoverlapped: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn Bind(&self, hport: super::super::Foundation::HANDLE, hhandle: super::super::Foundation::HANDLE) -> ::windows_core::Result<()>;
    fn SetMinThreads(&self, dwminiocompletionthreads: u32) -> ::windows_core::Result<()>;
    fn GetMinThreads(&self) -> ::windows_core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IHostIoCompletionManager {}
#[cfg(feature = "Win32_Foundation")]
impl IHostIoCompletionManager_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostIoCompletionManager_Impl, const OFFSET: isize>() -> IHostIoCompletionManager_Vtbl {
        unsafe extern "system" fn CreateIoCompletionPort<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostIoCompletionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phport: *mut super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateIoCompletionPort() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phport, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CloseIoCompletionPort<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostIoCompletionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hport: super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CloseIoCompletionPort(::core::mem::transmute_copy(&hport)).into()
        }
        unsafe extern "system" fn SetMaxThreads<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostIoCompletionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmaxiocompletionthreads: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMaxThreads(::core::mem::transmute_copy(&dwmaxiocompletionthreads)).into()
        }
        unsafe extern "system" fn GetMaxThreads<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostIoCompletionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmaxiocompletionthreads: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMaxThreads() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwmaxiocompletionthreads, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAvailableThreads<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostIoCompletionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwavailableiocompletionthreads: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAvailableThreads() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwavailableiocompletionthreads, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHostOverlappedSize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostIoCompletionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbsize: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetHostOverlappedSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcbsize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCLRIoCompletionManager<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostIoCompletionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmanager: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCLRIoCompletionManager(::windows_core::from_raw_borrowed(&pmanager)).into()
        }
        unsafe extern "system" fn InitializeHostOverlapped<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostIoCompletionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvoverlapped: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InitializeHostOverlapped(::core::mem::transmute_copy(&pvoverlapped)).into()
        }
        unsafe extern "system" fn Bind<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostIoCompletionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hport: super::super::Foundation::HANDLE, hhandle: super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Bind(::core::mem::transmute_copy(&hport), ::core::mem::transmute_copy(&hhandle)).into()
        }
        unsafe extern "system" fn SetMinThreads<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostIoCompletionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwminiocompletionthreads: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMinThreads(::core::mem::transmute_copy(&dwminiocompletionthreads)).into()
        }
        unsafe extern "system" fn GetMinThreads<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostIoCompletionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwminiocompletionthreads: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMinThreads() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwminiocompletionthreads, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateIoCompletionPort: CreateIoCompletionPort::<Identity, Impl, OFFSET>,
            CloseIoCompletionPort: CloseIoCompletionPort::<Identity, Impl, OFFSET>,
            SetMaxThreads: SetMaxThreads::<Identity, Impl, OFFSET>,
            GetMaxThreads: GetMaxThreads::<Identity, Impl, OFFSET>,
            GetAvailableThreads: GetAvailableThreads::<Identity, Impl, OFFSET>,
            GetHostOverlappedSize: GetHostOverlappedSize::<Identity, Impl, OFFSET>,
            SetCLRIoCompletionManager: SetCLRIoCompletionManager::<Identity, Impl, OFFSET>,
            InitializeHostOverlapped: InitializeHostOverlapped::<Identity, Impl, OFFSET>,
            Bind: Bind::<Identity, Impl, OFFSET>,
            SetMinThreads: SetMinThreads::<Identity, Impl, OFFSET>,
            GetMinThreads: GetMinThreads::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IHostIoCompletionManager as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait IHostMalloc_Impl: Sized {
    fn Alloc(&self, cbsize: usize, ecriticallevel: EMemoryCriticalLevel, ppmem: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn DebugAlloc(&self, cbsize: usize, ecriticallevel: EMemoryCriticalLevel, pszfilename: *const u8, ilineno: i32, ppmem: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn Free(&self, pmem: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IHostMalloc {}
impl IHostMalloc_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostMalloc_Impl, const OFFSET: isize>() -> IHostMalloc_Vtbl {
        unsafe extern "system" fn Alloc<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostMalloc_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbsize: usize, ecriticallevel: EMemoryCriticalLevel, ppmem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Alloc(::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&ecriticallevel), ::core::mem::transmute_copy(&ppmem)).into()
        }
        unsafe extern "system" fn DebugAlloc<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostMalloc_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbsize: usize, ecriticallevel: EMemoryCriticalLevel, pszfilename: *const u8, ilineno: i32, ppmem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DebugAlloc(::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&ecriticallevel), ::core::mem::transmute_copy(&pszfilename), ::core::mem::transmute_copy(&ilineno), ::core::mem::transmute_copy(&ppmem)).into()
        }
        unsafe extern "system" fn Free<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostMalloc_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmem: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Free(::core::mem::transmute_copy(&pmem)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Alloc: Alloc::<Identity, Impl, OFFSET>,
            DebugAlloc: DebugAlloc::<Identity, Impl, OFFSET>,
            Free: Free::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IHostMalloc as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait IHostManualEvent_Impl: Sized {
    fn Wait(&self, dwmilliseconds: u32, option: u32) -> ::windows_core::Result<()>;
    fn Reset(&self) -> ::windows_core::Result<()>;
    fn Set(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IHostManualEvent {}
impl IHostManualEvent_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostManualEvent_Impl, const OFFSET: isize>() -> IHostManualEvent_Vtbl {
        unsafe extern "system" fn Wait<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostManualEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmilliseconds: u32, option: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Wait(::core::mem::transmute_copy(&dwmilliseconds), ::core::mem::transmute_copy(&option)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostManualEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Set<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostManualEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Set().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Wait: Wait::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Set: Set::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IHostManualEvent as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait IHostMemoryManager_Impl: Sized {
    fn CreateMalloc(&self, dwmalloctype: u32) -> ::windows_core::Result<IHostMalloc>;
    fn VirtualAlloc(&self, paddress: *const ::core::ffi::c_void, dwsize: usize, flallocationtype: u32, flprotect: u32, ecriticallevel: EMemoryCriticalLevel, ppmem: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn VirtualFree(&self, lpaddress: *const ::core::ffi::c_void, dwsize: usize, dwfreetype: u32) -> ::windows_core::Result<()>;
    fn VirtualQuery(&self, lpaddress: *const ::core::ffi::c_void, lpbuffer: *mut ::core::ffi::c_void, dwlength: usize, presult: *mut usize) -> ::windows_core::Result<()>;
    fn VirtualProtect(&self, lpaddress: *const ::core::ffi::c_void, dwsize: usize, flnewprotect: u32) -> ::windows_core::Result<u32>;
    fn GetMemoryLoad(&self, pmemoryload: *mut u32, pavailablebytes: *mut usize) -> ::windows_core::Result<()>;
    fn RegisterMemoryNotificationCallback(&self, pcallback: ::core::option::Option<&ICLRMemoryNotificationCallback>) -> ::windows_core::Result<()>;
    fn NeedsVirtualAddressSpace(&self, startaddress: *const ::core::ffi::c_void, size: usize) -> ::windows_core::Result<()>;
    fn AcquiredVirtualAddressSpace(&self, startaddress: *const ::core::ffi::c_void, size: usize) -> ::windows_core::Result<()>;
    fn ReleasedVirtualAddressSpace(&self, startaddress: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IHostMemoryManager {}
impl IHostMemoryManager_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostMemoryManager_Impl, const OFFSET: isize>() -> IHostMemoryManager_Vtbl {
        unsafe extern "system" fn CreateMalloc<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostMemoryManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmalloctype: u32, ppmalloc: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateMalloc(::core::mem::transmute_copy(&dwmalloctype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmalloc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VirtualAlloc<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostMemoryManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paddress: *const ::core::ffi::c_void, dwsize: usize, flallocationtype: u32, flprotect: u32, ecriticallevel: EMemoryCriticalLevel, ppmem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.VirtualAlloc(::core::mem::transmute_copy(&paddress), ::core::mem::transmute_copy(&dwsize), ::core::mem::transmute_copy(&flallocationtype), ::core::mem::transmute_copy(&flprotect), ::core::mem::transmute_copy(&ecriticallevel), ::core::mem::transmute_copy(&ppmem)).into()
        }
        unsafe extern "system" fn VirtualFree<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostMemoryManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpaddress: *const ::core::ffi::c_void, dwsize: usize, dwfreetype: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.VirtualFree(::core::mem::transmute_copy(&lpaddress), ::core::mem::transmute_copy(&dwsize), ::core::mem::transmute_copy(&dwfreetype)).into()
        }
        unsafe extern "system" fn VirtualQuery<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostMemoryManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpaddress: *const ::core::ffi::c_void, lpbuffer: *mut ::core::ffi::c_void, dwlength: usize, presult: *mut usize) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.VirtualQuery(::core::mem::transmute_copy(&lpaddress), ::core::mem::transmute_copy(&lpbuffer), ::core::mem::transmute_copy(&dwlength), ::core::mem::transmute_copy(&presult)).into()
        }
        unsafe extern "system" fn VirtualProtect<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostMemoryManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpaddress: *const ::core::ffi::c_void, dwsize: usize, flnewprotect: u32, pfloldprotect: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.VirtualProtect(::core::mem::transmute_copy(&lpaddress), ::core::mem::transmute_copy(&dwsize), ::core::mem::transmute_copy(&flnewprotect)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfloldprotect, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMemoryLoad<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostMemoryManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmemoryload: *mut u32, pavailablebytes: *mut usize) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMemoryLoad(::core::mem::transmute_copy(&pmemoryload), ::core::mem::transmute_copy(&pavailablebytes)).into()
        }
        unsafe extern "system" fn RegisterMemoryNotificationCallback<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostMemoryManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RegisterMemoryNotificationCallback(::windows_core::from_raw_borrowed(&pcallback)).into()
        }
        unsafe extern "system" fn NeedsVirtualAddressSpace<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostMemoryManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startaddress: *const ::core::ffi::c_void, size: usize) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.NeedsVirtualAddressSpace(::core::mem::transmute_copy(&startaddress), ::core::mem::transmute_copy(&size)).into()
        }
        unsafe extern "system" fn AcquiredVirtualAddressSpace<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostMemoryManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startaddress: *const ::core::ffi::c_void, size: usize) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AcquiredVirtualAddressSpace(::core::mem::transmute_copy(&startaddress), ::core::mem::transmute_copy(&size)).into()
        }
        unsafe extern "system" fn ReleasedVirtualAddressSpace<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostMemoryManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startaddress: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleasedVirtualAddressSpace(::core::mem::transmute_copy(&startaddress)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateMalloc: CreateMalloc::<Identity, Impl, OFFSET>,
            VirtualAlloc: VirtualAlloc::<Identity, Impl, OFFSET>,
            VirtualFree: VirtualFree::<Identity, Impl, OFFSET>,
            VirtualQuery: VirtualQuery::<Identity, Impl, OFFSET>,
            VirtualProtect: VirtualProtect::<Identity, Impl, OFFSET>,
            GetMemoryLoad: GetMemoryLoad::<Identity, Impl, OFFSET>,
            RegisterMemoryNotificationCallback: RegisterMemoryNotificationCallback::<Identity, Impl, OFFSET>,
            NeedsVirtualAddressSpace: NeedsVirtualAddressSpace::<Identity, Impl, OFFSET>,
            AcquiredVirtualAddressSpace: AcquiredVirtualAddressSpace::<Identity, Impl, OFFSET>,
            ReleasedVirtualAddressSpace: ReleasedVirtualAddressSpace::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IHostMemoryManager as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait IHostPolicyManager_Impl: Sized {
    fn OnDefaultAction(&self, operation: EClrOperation, action: EPolicyAction) -> ::windows_core::Result<()>;
    fn OnTimeout(&self, operation: EClrOperation, action: EPolicyAction) -> ::windows_core::Result<()>;
    fn OnFailure(&self, failure: EClrFailure, action: EPolicyAction) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IHostPolicyManager {}
impl IHostPolicyManager_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostPolicyManager_Impl, const OFFSET: isize>() -> IHostPolicyManager_Vtbl {
        unsafe extern "system" fn OnDefaultAction<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostPolicyManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operation: EClrOperation, action: EPolicyAction) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnDefaultAction(::core::mem::transmute_copy(&operation), ::core::mem::transmute_copy(&action)).into()
        }
        unsafe extern "system" fn OnTimeout<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostPolicyManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operation: EClrOperation, action: EPolicyAction) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnTimeout(::core::mem::transmute_copy(&operation), ::core::mem::transmute_copy(&action)).into()
        }
        unsafe extern "system" fn OnFailure<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostPolicyManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, failure: EClrFailure, action: EPolicyAction) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnFailure(::core::mem::transmute_copy(&failure), ::core::mem::transmute_copy(&action)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnDefaultAction: OnDefaultAction::<Identity, Impl, OFFSET>,
            OnTimeout: OnTimeout::<Identity, Impl, OFFSET>,
            OnFailure: OnFailure::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IHostPolicyManager as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait IHostSecurityContext_Impl: Sized {
    fn Capture(&self) -> ::windows_core::Result<IHostSecurityContext>;
}
impl ::windows_core::RuntimeName for IHostSecurityContext {}
impl IHostSecurityContext_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostSecurityContext_Impl, const OFFSET: isize>() -> IHostSecurityContext_Vtbl {
        unsafe extern "system" fn Capture<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostSecurityContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppclonedcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Capture() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclonedcontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Capture: Capture::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IHostSecurityContext as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IHostSecurityManager_Impl: Sized {
    fn ImpersonateLoggedOnUser(&self, htoken: super::super::Foundation::HANDLE) -> ::windows_core::Result<()>;
    fn RevertToSelf(&self) -> ::windows_core::Result<()>;
    fn OpenThreadToken(&self, dwdesiredaccess: u32, bopenasself: super::super::Foundation::BOOL) -> ::windows_core::Result<super::super::Foundation::HANDLE>;
    fn SetThreadToken(&self, htoken: super::super::Foundation::HANDLE) -> ::windows_core::Result<()>;
    fn GetSecurityContext(&self, econtexttype: EContextType) -> ::windows_core::Result<IHostSecurityContext>;
    fn SetSecurityContext(&self, econtexttype: EContextType, psecuritycontext: ::core::option::Option<&IHostSecurityContext>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IHostSecurityManager {}
#[cfg(feature = "Win32_Foundation")]
impl IHostSecurityManager_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostSecurityManager_Impl, const OFFSET: isize>() -> IHostSecurityManager_Vtbl {
        unsafe extern "system" fn ImpersonateLoggedOnUser<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostSecurityManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, htoken: super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ImpersonateLoggedOnUser(::core::mem::transmute_copy(&htoken)).into()
        }
        unsafe extern "system" fn RevertToSelf<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostSecurityManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RevertToSelf().into()
        }
        unsafe extern "system" fn OpenThreadToken<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostSecurityManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdesiredaccess: u32, bopenasself: super::super::Foundation::BOOL, phthreadtoken: *mut super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OpenThreadToken(::core::mem::transmute_copy(&dwdesiredaccess), ::core::mem::transmute_copy(&bopenasself)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phthreadtoken, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetThreadToken<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostSecurityManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, htoken: super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetThreadToken(::core::mem::transmute_copy(&htoken)).into()
        }
        unsafe extern "system" fn GetSecurityContext<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostSecurityManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, econtexttype: EContextType, ppsecuritycontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSecurityContext(::core::mem::transmute_copy(&econtexttype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsecuritycontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurityContext<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostSecurityManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, econtexttype: EContextType, psecuritycontext: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSecurityContext(::core::mem::transmute_copy(&econtexttype), ::windows_core::from_raw_borrowed(&psecuritycontext)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ImpersonateLoggedOnUser: ImpersonateLoggedOnUser::<Identity, Impl, OFFSET>,
            RevertToSelf: RevertToSelf::<Identity, Impl, OFFSET>,
            OpenThreadToken: OpenThreadToken::<Identity, Impl, OFFSET>,
            SetThreadToken: SetThreadToken::<Identity, Impl, OFFSET>,
            GetSecurityContext: GetSecurityContext::<Identity, Impl, OFFSET>,
            SetSecurityContext: SetSecurityContext::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IHostSecurityManager as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait IHostSemaphore_Impl: Sized {
    fn Wait(&self, dwmilliseconds: u32, option: u32) -> ::windows_core::Result<()>;
    fn ReleaseSemaphore(&self, lreleasecount: i32) -> ::windows_core::Result<i32>;
}
impl ::windows_core::RuntimeName for IHostSemaphore {}
impl IHostSemaphore_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostSemaphore_Impl, const OFFSET: isize>() -> IHostSemaphore_Vtbl {
        unsafe extern "system" fn Wait<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostSemaphore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmilliseconds: u32, option: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Wait(::core::mem::transmute_copy(&dwmilliseconds), ::core::mem::transmute_copy(&option)).into()
        }
        unsafe extern "system" fn ReleaseSemaphore<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostSemaphore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lreleasecount: i32, lppreviouscount: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReleaseSemaphore(::core::mem::transmute_copy(&lreleasecount)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lppreviouscount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Wait: Wait::<Identity, Impl, OFFSET>,
            ReleaseSemaphore: ReleaseSemaphore::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IHostSemaphore as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IHostSyncManager_Impl: Sized {
    fn SetCLRSyncManager(&self, pmanager: ::core::option::Option<&ICLRSyncManager>) -> ::windows_core::Result<()>;
    fn CreateCrst(&self) -> ::windows_core::Result<IHostCrst>;
    fn CreateCrstWithSpinCount(&self, dwspincount: u32) -> ::windows_core::Result<IHostCrst>;
    fn CreateAutoEvent(&self) -> ::windows_core::Result<IHostAutoEvent>;
    fn CreateManualEvent(&self, binitialstate: super::super::Foundation::BOOL) -> ::windows_core::Result<IHostManualEvent>;
    fn CreateMonitorEvent(&self, cookie: usize) -> ::windows_core::Result<IHostAutoEvent>;
    fn CreateRWLockWriterEvent(&self, cookie: usize) -> ::windows_core::Result<IHostAutoEvent>;
    fn CreateRWLockReaderEvent(&self, binitialstate: super::super::Foundation::BOOL, cookie: usize) -> ::windows_core::Result<IHostManualEvent>;
    fn CreateSemaphoreA(&self, dwinitial: u32, dwmax: u32) -> ::windows_core::Result<IHostSemaphore>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IHostSyncManager {}
#[cfg(feature = "Win32_Foundation")]
impl IHostSyncManager_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostSyncManager_Impl, const OFFSET: isize>() -> IHostSyncManager_Vtbl {
        unsafe extern "system" fn SetCLRSyncManager<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostSyncManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmanager: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCLRSyncManager(::windows_core::from_raw_borrowed(&pmanager)).into()
        }
        unsafe extern "system" fn CreateCrst<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostSyncManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcrst: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateCrst() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcrst, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCrstWithSpinCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostSyncManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwspincount: u32, ppcrst: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateCrstWithSpinCount(::core::mem::transmute_copy(&dwspincount)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcrst, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAutoEvent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostSyncManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppevent: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateAutoEvent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppevent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateManualEvent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostSyncManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, binitialstate: super::super::Foundation::BOOL, ppevent: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateManualEvent(::core::mem::transmute_copy(&binitialstate)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppevent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMonitorEvent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostSyncManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: usize, ppevent: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateMonitorEvent(::core::mem::transmute_copy(&cookie)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppevent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRWLockWriterEvent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostSyncManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: usize, ppevent: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateRWLockWriterEvent(::core::mem::transmute_copy(&cookie)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppevent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRWLockReaderEvent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostSyncManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, binitialstate: super::super::Foundation::BOOL, cookie: usize, ppevent: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateRWLockReaderEvent(::core::mem::transmute_copy(&binitialstate), ::core::mem::transmute_copy(&cookie)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppevent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSemaphoreA<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostSyncManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinitial: u32, dwmax: u32, ppsemaphore: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateSemaphoreA(::core::mem::transmute_copy(&dwinitial), ::core::mem::transmute_copy(&dwmax)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsemaphore, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetCLRSyncManager: SetCLRSyncManager::<Identity, Impl, OFFSET>,
            CreateCrst: CreateCrst::<Identity, Impl, OFFSET>,
            CreateCrstWithSpinCount: CreateCrstWithSpinCount::<Identity, Impl, OFFSET>,
            CreateAutoEvent: CreateAutoEvent::<Identity, Impl, OFFSET>,
            CreateManualEvent: CreateManualEvent::<Identity, Impl, OFFSET>,
            CreateMonitorEvent: CreateMonitorEvent::<Identity, Impl, OFFSET>,
            CreateRWLockWriterEvent: CreateRWLockWriterEvent::<Identity, Impl, OFFSET>,
            CreateRWLockReaderEvent: CreateRWLockReaderEvent::<Identity, Impl, OFFSET>,
            CreateSemaphoreA: CreateSemaphoreA::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IHostSyncManager as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait IHostTask_Impl: Sized {
    fn Start(&self) -> ::windows_core::Result<()>;
    fn Alert(&self) -> ::windows_core::Result<()>;
    fn Join(&self, dwmilliseconds: u32, option: u32) -> ::windows_core::Result<()>;
    fn SetPriority(&self, newpriority: i32) -> ::windows_core::Result<()>;
    fn GetPriority(&self) -> ::windows_core::Result<i32>;
    fn SetCLRTask(&self, pclrtask: ::core::option::Option<&ICLRTask>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IHostTask {}
impl IHostTask_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostTask_Impl, const OFFSET: isize>() -> IHostTask_Vtbl {
        unsafe extern "system" fn Start<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Start().into()
        }
        unsafe extern "system" fn Alert<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Alert().into()
        }
        unsafe extern "system" fn Join<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmilliseconds: u32, option: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Join(::core::mem::transmute_copy(&dwmilliseconds), ::core::mem::transmute_copy(&option)).into()
        }
        unsafe extern "system" fn SetPriority<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newpriority: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPriority(::core::mem::transmute_copy(&newpriority)).into()
        }
        unsafe extern "system" fn GetPriority<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppriority: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPriority() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppriority, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCLRTask<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclrtask: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCLRTask(::windows_core::from_raw_borrowed(&pclrtask)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Start: Start::<Identity, Impl, OFFSET>,
            Alert: Alert::<Identity, Impl, OFFSET>,
            Join: Join::<Identity, Impl, OFFSET>,
            SetPriority: SetPriority::<Identity, Impl, OFFSET>,
            GetPriority: GetPriority::<Identity, Impl, OFFSET>,
            SetCLRTask: SetCLRTask::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IHostTask as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"Win32_Foundation\"`, `\"Win32_System_Threading\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
pub trait IHostTaskManager_Impl: Sized {
    fn GetCurrentTask(&self) -> ::windows_core::Result<IHostTask>;
    fn CreateTask(&self, dwstacksize: u32, pstartaddress: super::Threading::LPTHREAD_START_ROUTINE, pparameter: *const ::core::ffi::c_void) -> ::windows_core::Result<IHostTask>;
    fn Sleep(&self, dwmilliseconds: u32, option: u32) -> ::windows_core::Result<()>;
    fn SwitchToTask(&self, option: u32) -> ::windows_core::Result<()>;
    fn SetUILocale(&self, lcid: u32) -> ::windows_core::Result<()>;
    fn SetLocale(&self, lcid: u32) -> ::windows_core::Result<()>;
    fn CallNeedsHostHook(&self, target: usize) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn LeaveRuntime(&self, target: usize) -> ::windows_core::Result<()>;
    fn EnterRuntime(&self) -> ::windows_core::Result<()>;
    fn ReverseLeaveRuntime(&self) -> ::windows_core::Result<()>;
    fn ReverseEnterRuntime(&self) -> ::windows_core::Result<()>;
    fn BeginDelayAbort(&self) -> ::windows_core::Result<()>;
    fn EndDelayAbort(&self) -> ::windows_core::Result<()>;
    fn BeginThreadAffinity(&self) -> ::windows_core::Result<()>;
    fn EndThreadAffinity(&self) -> ::windows_core::Result<()>;
    fn SetStackGuarantee(&self, guarantee: u32) -> ::windows_core::Result<()>;
    fn GetStackGuarantee(&self) -> ::windows_core::Result<u32>;
    fn SetCLRTaskManager(&self, ppmanager: ::core::option::Option<&ICLRTaskManager>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
impl ::windows_core::RuntimeName for IHostTaskManager {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
impl IHostTaskManager_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostTaskManager_Impl, const OFFSET: isize>() -> IHostTaskManager_Vtbl {
        unsafe extern "system" fn GetCurrentTask<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostTaskManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptask: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurrentTask() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptask, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTask<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostTaskManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstacksize: u32, pstartaddress: super::Threading::LPTHREAD_START_ROUTINE, pparameter: *const ::core::ffi::c_void, pptask: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateTask(::core::mem::transmute_copy(&dwstacksize), ::core::mem::transmute_copy(&pstartaddress), ::core::mem::transmute_copy(&pparameter)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptask, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Sleep<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostTaskManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmilliseconds: u32, option: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Sleep(::core::mem::transmute_copy(&dwmilliseconds), ::core::mem::transmute_copy(&option)).into()
        }
        unsafe extern "system" fn SwitchToTask<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostTaskManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, option: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SwitchToTask(::core::mem::transmute_copy(&option)).into()
        }
        unsafe extern "system" fn SetUILocale<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostTaskManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcid: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetUILocale(::core::mem::transmute_copy(&lcid)).into()
        }
        unsafe extern "system" fn SetLocale<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostTaskManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcid: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLocale(::core::mem::transmute_copy(&lcid)).into()
        }
        unsafe extern "system" fn CallNeedsHostHook<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostTaskManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: usize, pbcallneedshosthook: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CallNeedsHostHook(::core::mem::transmute_copy(&target)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbcallneedshosthook, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LeaveRuntime<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostTaskManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: usize) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LeaveRuntime(::core::mem::transmute_copy(&target)).into()
        }
        unsafe extern "system" fn EnterRuntime<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostTaskManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnterRuntime().into()
        }
        unsafe extern "system" fn ReverseLeaveRuntime<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostTaskManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReverseLeaveRuntime().into()
        }
        unsafe extern "system" fn ReverseEnterRuntime<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostTaskManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReverseEnterRuntime().into()
        }
        unsafe extern "system" fn BeginDelayAbort<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostTaskManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BeginDelayAbort().into()
        }
        unsafe extern "system" fn EndDelayAbort<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostTaskManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndDelayAbort().into()
        }
        unsafe extern "system" fn BeginThreadAffinity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostTaskManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BeginThreadAffinity().into()
        }
        unsafe extern "system" fn EndThreadAffinity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostTaskManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndThreadAffinity().into()
        }
        unsafe extern "system" fn SetStackGuarantee<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostTaskManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guarantee: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStackGuarantee(::core::mem::transmute_copy(&guarantee)).into()
        }
        unsafe extern "system" fn GetStackGuarantee<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostTaskManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguarantee: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStackGuarantee() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguarantee, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCLRTaskManager<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostTaskManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmanager: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCLRTaskManager(::windows_core::from_raw_borrowed(&ppmanager)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrentTask: GetCurrentTask::<Identity, Impl, OFFSET>,
            CreateTask: CreateTask::<Identity, Impl, OFFSET>,
            Sleep: Sleep::<Identity, Impl, OFFSET>,
            SwitchToTask: SwitchToTask::<Identity, Impl, OFFSET>,
            SetUILocale: SetUILocale::<Identity, Impl, OFFSET>,
            SetLocale: SetLocale::<Identity, Impl, OFFSET>,
            CallNeedsHostHook: CallNeedsHostHook::<Identity, Impl, OFFSET>,
            LeaveRuntime: LeaveRuntime::<Identity, Impl, OFFSET>,
            EnterRuntime: EnterRuntime::<Identity, Impl, OFFSET>,
            ReverseLeaveRuntime: ReverseLeaveRuntime::<Identity, Impl, OFFSET>,
            ReverseEnterRuntime: ReverseEnterRuntime::<Identity, Impl, OFFSET>,
            BeginDelayAbort: BeginDelayAbort::<Identity, Impl, OFFSET>,
            EndDelayAbort: EndDelayAbort::<Identity, Impl, OFFSET>,
            BeginThreadAffinity: BeginThreadAffinity::<Identity, Impl, OFFSET>,
            EndThreadAffinity: EndThreadAffinity::<Identity, Impl, OFFSET>,
            SetStackGuarantee: SetStackGuarantee::<Identity, Impl, OFFSET>,
            GetStackGuarantee: GetStackGuarantee::<Identity, Impl, OFFSET>,
            SetCLRTaskManager: SetCLRTaskManager::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IHostTaskManager as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"Win32_System_Threading\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Threading")]
pub trait IHostThreadpoolManager_Impl: Sized {
    fn QueueUserWorkItem(&self, function: super::Threading::LPTHREAD_START_ROUTINE, context: *const ::core::ffi::c_void, flags: u32) -> ::windows_core::Result<()>;
    fn SetMaxThreads(&self, dwmaxworkerthreads: u32) -> ::windows_core::Result<()>;
    fn GetMaxThreads(&self) -> ::windows_core::Result<u32>;
    fn GetAvailableThreads(&self) -> ::windows_core::Result<u32>;
    fn SetMinThreads(&self, dwminiocompletionthreads: u32) -> ::windows_core::Result<()>;
    fn GetMinThreads(&self) -> ::windows_core::Result<u32>;
}
#[cfg(feature = "Win32_System_Threading")]
impl ::windows_core::RuntimeName for IHostThreadpoolManager {}
#[cfg(feature = "Win32_System_Threading")]
impl IHostThreadpoolManager_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostThreadpoolManager_Impl, const OFFSET: isize>() -> IHostThreadpoolManager_Vtbl {
        unsafe extern "system" fn QueueUserWorkItem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostThreadpoolManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, function: super::Threading::LPTHREAD_START_ROUTINE, context: *const ::core::ffi::c_void, flags: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.QueueUserWorkItem(::core::mem::transmute_copy(&function), ::core::mem::transmute_copy(&context), ::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn SetMaxThreads<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostThreadpoolManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmaxworkerthreads: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMaxThreads(::core::mem::transmute_copy(&dwmaxworkerthreads)).into()
        }
        unsafe extern "system" fn GetMaxThreads<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostThreadpoolManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmaxworkerthreads: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMaxThreads() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwmaxworkerthreads, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAvailableThreads<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostThreadpoolManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwavailableworkerthreads: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAvailableThreads() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwavailableworkerthreads, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinThreads<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostThreadpoolManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwminiocompletionthreads: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMinThreads(::core::mem::transmute_copy(&dwminiocompletionthreads)).into()
        }
        unsafe extern "system" fn GetMinThreads<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHostThreadpoolManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwminiocompletionthreads: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMinThreads() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwminiocompletionthreads, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueueUserWorkItem: QueueUserWorkItem::<Identity, Impl, OFFSET>,
            SetMaxThreads: SetMaxThreads::<Identity, Impl, OFFSET>,
            GetMaxThreads: GetMaxThreads::<Identity, Impl, OFFSET>,
            GetAvailableThreads: GetAvailableThreads::<Identity, Impl, OFFSET>,
            SetMinThreads: SetMinThreads::<Identity, Impl, OFFSET>,
            GetMinThreads: GetMinThreads::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IHostThreadpoolManager as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait IManagedObject_Impl: Sized {
    fn GetSerializedBuffer(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetObjectIdentity(&self, pbstrguid: *mut ::windows_core::BSTR, appdomainid: *mut i32, pccw: *mut i32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IManagedObject {}
impl IManagedObject_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IManagedObject_Impl, const OFFSET: isize>() -> IManagedObject_Vtbl {
        unsafe extern "system" fn GetSerializedBuffer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IManagedObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSerializedBuffer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObjectIdentity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IManagedObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, appdomainid: *mut i32, pccw: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetObjectIdentity(::core::mem::transmute_copy(&pbstrguid), ::core::mem::transmute_copy(&appdomainid), ::core::mem::transmute_copy(&pccw)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSerializedBuffer: GetSerializedBuffer::<Identity, Impl, OFFSET>,
            GetObjectIdentity: GetObjectIdentity::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IManagedObject as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IObjectHandle_Impl: Sized {
    fn Unwrap(&self) -> ::windows_core::Result<super::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IObjectHandle {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IObjectHandle_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectHandle_Impl, const OFFSET: isize>() -> IObjectHandle_Vtbl {
        unsafe extern "system" fn Unwrap<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectHandle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppv: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Unwrap() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppv, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Unwrap: Unwrap::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IObjectHandle as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait ITypeName_Impl: Sized {
    fn GetNameCount(&self) -> ::windows_core::Result<u32>;
    fn GetNames(&self, count: u32, rgbsznames: *mut ::windows_core::BSTR, pcount: *mut u32) -> ::windows_core::Result<()>;
    fn GetTypeArgumentCount(&self) -> ::windows_core::Result<u32>;
    fn GetTypeArguments(&self, count: u32, rgparguments: *mut ::core::option::Option<ITypeName>, pcount: *mut u32) -> ::windows_core::Result<()>;
    fn GetModifierLength(&self) -> ::windows_core::Result<u32>;
    fn GetModifiers(&self, count: u32, rgmodifiers: *mut u32, pcount: *mut u32) -> ::windows_core::Result<()>;
    fn GetAssemblyName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
}
impl ::windows_core::RuntimeName for ITypeName {}
impl ITypeName_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITypeName_Impl, const OFFSET: isize>() -> ITypeName_Vtbl {
        unsafe extern "system" fn GetNameCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITypeName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetNameCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNames<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITypeName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: u32, rgbsznames: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pcount: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetNames(::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&rgbsznames), ::core::mem::transmute_copy(&pcount)).into()
        }
        unsafe extern "system" fn GetTypeArgumentCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITypeName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTypeArgumentCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTypeArguments<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITypeName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: u32, rgparguments: *mut *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetTypeArguments(::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&rgparguments), ::core::mem::transmute_copy(&pcount)).into()
        }
        unsafe extern "system" fn GetModifierLength<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITypeName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetModifierLength() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetModifiers<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITypeName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: u32, rgmodifiers: *mut u32, pcount: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetModifiers(::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&rgmodifiers), ::core::mem::transmute_copy(&pcount)).into()
        }
        unsafe extern "system" fn GetAssemblyName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITypeName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rgbszassemblynames: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAssemblyName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(rgbszassemblynames, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetNameCount: GetNameCount::<Identity, Impl, OFFSET>,
            GetNames: GetNames::<Identity, Impl, OFFSET>,
            GetTypeArgumentCount: GetTypeArgumentCount::<Identity, Impl, OFFSET>,
            GetTypeArguments: GetTypeArguments::<Identity, Impl, OFFSET>,
            GetModifierLength: GetModifierLength::<Identity, Impl, OFFSET>,
            GetModifiers: GetModifiers::<Identity, Impl, OFFSET>,
            GetAssemblyName: GetAssemblyName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITypeName as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait ITypeNameBuilder_Impl: Sized {
    fn OpenGenericArguments(&self) -> ::windows_core::Result<()>;
    fn CloseGenericArguments(&self) -> ::windows_core::Result<()>;
    fn OpenGenericArgument(&self) -> ::windows_core::Result<()>;
    fn CloseGenericArgument(&self) -> ::windows_core::Result<()>;
    fn AddName(&self, szname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn AddPointer(&self) -> ::windows_core::Result<()>;
    fn AddByRef(&self) -> ::windows_core::Result<()>;
    fn AddSzArray(&self) -> ::windows_core::Result<()>;
    fn AddArray(&self, rank: u32) -> ::windows_core::Result<()>;
    fn AddAssemblySpec(&self, szassemblyspec: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn ToString(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Clear(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ITypeNameBuilder {}
impl ITypeNameBuilder_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITypeNameBuilder_Impl, const OFFSET: isize>() -> ITypeNameBuilder_Vtbl {
        unsafe extern "system" fn OpenGenericArguments<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITypeNameBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OpenGenericArguments().into()
        }
        unsafe extern "system" fn CloseGenericArguments<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITypeNameBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CloseGenericArguments().into()
        }
        unsafe extern "system" fn OpenGenericArgument<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITypeNameBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OpenGenericArgument().into()
        }
        unsafe extern "system" fn CloseGenericArgument<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITypeNameBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CloseGenericArgument().into()
        }
        unsafe extern "system" fn AddName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITypeNameBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddName(::core::mem::transmute(&szname)).into()
        }
        unsafe extern "system" fn AddPointer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITypeNameBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddPointer().into()
        }
        unsafe extern "system" fn AddByRef<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITypeNameBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddByRef().into()
        }
        unsafe extern "system" fn AddSzArray<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITypeNameBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddSzArray().into()
        }
        unsafe extern "system" fn AddArray<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITypeNameBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rank: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddArray(::core::mem::transmute_copy(&rank)).into()
        }
        unsafe extern "system" fn AddAssemblySpec<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITypeNameBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szassemblyspec: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddAssemblySpec(::core::mem::transmute(&szassemblyspec)).into()
        }
        unsafe extern "system" fn ToString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITypeNameBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszstringrepresentation: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ToString() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszstringrepresentation, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clear<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITypeNameBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Clear().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OpenGenericArguments: OpenGenericArguments::<Identity, Impl, OFFSET>,
            CloseGenericArguments: CloseGenericArguments::<Identity, Impl, OFFSET>,
            OpenGenericArgument: OpenGenericArgument::<Identity, Impl, OFFSET>,
            CloseGenericArgument: CloseGenericArgument::<Identity, Impl, OFFSET>,
            AddName: AddName::<Identity, Impl, OFFSET>,
            AddPointer: AddPointer::<Identity, Impl, OFFSET>,
            AddByRef: AddByRef::<Identity, Impl, OFFSET>,
            AddSzArray: AddSzArray::<Identity, Impl, OFFSET>,
            AddArray: AddArray::<Identity, Impl, OFFSET>,
            AddAssemblySpec: AddAssemblySpec::<Identity, Impl, OFFSET>,
            ToString: ToString::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITypeNameBuilder as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait ITypeNameFactory_Impl: Sized {
    fn ParseTypeName(&self, szname: &::windows_core::PCWSTR, perror: *mut u32, pptypename: *mut ::core::option::Option<ITypeName>) -> ::windows_core::Result<()>;
    fn GetTypeNameBuilder(&self) -> ::windows_core::Result<ITypeNameBuilder>;
}
impl ::windows_core::RuntimeName for ITypeNameFactory {}
impl ITypeNameFactory_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITypeNameFactory_Impl, const OFFSET: isize>() -> ITypeNameFactory_Vtbl {
        unsafe extern "system" fn ParseTypeName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITypeNameFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szname: ::windows_core::PCWSTR, perror: *mut u32, pptypename: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ParseTypeName(::core::mem::transmute(&szname), ::core::mem::transmute_copy(&perror), ::core::mem::transmute_copy(&pptypename)).into()
        }
        unsafe extern "system" fn GetTypeNameBuilder<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITypeNameFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptypebuilder: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTypeNameBuilder() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptypebuilder, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ParseTypeName: ParseTypeName::<Identity, Impl, OFFSET>,
            GetTypeNameBuilder: GetTypeNameBuilder::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITypeNameFactory as ::windows_core::ComInterface>::IID
    }
}
