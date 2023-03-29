#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait IActionOnCLREvent_Impl: Sized {
    fn OnEvent(&self, event: EClrEvent, data: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IActionOnCLREvent {}
impl IActionOnCLREvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IActionOnCLREvent_Impl, const OFFSET: isize>() -> IActionOnCLREvent_Vtbl {
        unsafe extern "system" fn OnEvent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IActionOnCLREvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, event: EClrEvent, data: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnEvent(::core::mem::transmute_copy(&event), ::core::mem::transmute_copy(&data)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnEvent: OnEvent::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IActionOnCLREvent as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait IApartmentCallback_Impl: Sized {
    fn DoCallback(&self, pfunc: usize, pdata: usize) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IApartmentCallback {}
impl IApartmentCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IApartmentCallback_Impl, const OFFSET: isize>() -> IApartmentCallback_Vtbl {
        unsafe extern "system" fn DoCallback<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IApartmentCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfunc: usize, pdata: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DoCallback(::core::mem::transmute_copy(&pfunc), ::core::mem::transmute_copy(&pdata)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), DoCallback: DoCallback::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IApartmentCallback as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait IAppDomainBinding_Impl: Sized {
    fn OnAppDomain(&self, pappdomain: ::core::option::Option<&::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IAppDomainBinding {}
impl IAppDomainBinding_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppDomainBinding_Impl, const OFFSET: isize>() -> IAppDomainBinding_Vtbl {
        unsafe extern "system" fn OnAppDomain<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppDomainBinding_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pappdomain: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnAppDomain(::windows::core::from_raw_borrowed(&pappdomain)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnAppDomain: OnAppDomain::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppDomainBinding as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait ICLRAppDomainResourceMonitor_Impl: Sized {
    fn GetCurrentAllocated(&self, dwappdomainid: u32, pbytesallocated: *mut u64) -> ::windows::core::Result<()>;
    fn GetCurrentSurvived(&self, dwappdomainid: u32, pappdomainbytessurvived: *mut u64, ptotalbytessurvived: *mut u64) -> ::windows::core::Result<()>;
    fn GetCurrentCpuTime(&self, dwappdomainid: u32, pmilliseconds: *mut u64) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ICLRAppDomainResourceMonitor {}
impl ICLRAppDomainResourceMonitor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRAppDomainResourceMonitor_Impl, const OFFSET: isize>() -> ICLRAppDomainResourceMonitor_Vtbl {
        unsafe extern "system" fn GetCurrentAllocated<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRAppDomainResourceMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwappdomainid: u32, pbytesallocated: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCurrentAllocated(::core::mem::transmute_copy(&dwappdomainid), ::core::mem::transmute_copy(&pbytesallocated)).into()
        }
        unsafe extern "system" fn GetCurrentSurvived<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRAppDomainResourceMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwappdomainid: u32, pappdomainbytessurvived: *mut u64, ptotalbytessurvived: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCurrentSurvived(::core::mem::transmute_copy(&dwappdomainid), ::core::mem::transmute_copy(&pappdomainbytessurvived), ::core::mem::transmute_copy(&ptotalbytessurvived)).into()
        }
        unsafe extern "system" fn GetCurrentCpuTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRAppDomainResourceMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwappdomainid: u32, pmilliseconds: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCurrentCpuTime(::core::mem::transmute_copy(&dwappdomainid), ::core::mem::transmute_copy(&pmilliseconds)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrentAllocated: GetCurrentAllocated::<Identity, Impl, OFFSET>,
            GetCurrentSurvived: GetCurrentSurvived::<Identity, Impl, OFFSET>,
            GetCurrentCpuTime: GetCurrentCpuTime::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICLRAppDomainResourceMonitor as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ICLRAssemblyIdentityManager_Impl: Sized {
    fn GetCLRAssemblyReferenceList(&self, ppwzassemblyreferences: *const ::windows::core::PCWSTR, dwnumofreferences: u32) -> ::windows::core::Result<ICLRAssemblyReferenceList>;
    fn GetBindingIdentityFromFile(&self, pwzfilepath: &::windows::core::PCWSTR, dwflags: u32, pwzbuffer: ::windows::core::PWSTR, pcchbuffersize: *mut u32) -> ::windows::core::Result<()>;
    fn GetBindingIdentityFromStream(&self, pstream: ::core::option::Option<&super::Com::IStream>, dwflags: u32, pwzbuffer: ::windows::core::PWSTR, pcchbuffersize: *mut u32) -> ::windows::core::Result<()>;
    fn GetReferencedAssembliesFromFile(&self, pwzfilepath: &::windows::core::PCWSTR, dwflags: u32, pexcludeassemblieslist: ::core::option::Option<&ICLRAssemblyReferenceList>) -> ::windows::core::Result<ICLRReferenceAssemblyEnum>;
    fn GetReferencedAssembliesFromStream(&self, pstream: ::core::option::Option<&super::Com::IStream>, dwflags: u32, pexcludeassemblieslist: ::core::option::Option<&ICLRAssemblyReferenceList>) -> ::windows::core::Result<ICLRReferenceAssemblyEnum>;
    fn GetProbingAssembliesFromReference(&self, dwmachinetype: u32, dwflags: u32, pwzreferenceidentity: &::windows::core::PCWSTR) -> ::windows::core::Result<ICLRProbingAssemblyEnum>;
    fn IsStronglyNamed(&self, pwzassemblyidentity: &::windows::core::PCWSTR, pbisstronglynamed: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for ICLRAssemblyIdentityManager {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ICLRAssemblyIdentityManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRAssemblyIdentityManager_Impl, const OFFSET: isize>() -> ICLRAssemblyIdentityManager_Vtbl {
        unsafe extern "system" fn GetCLRAssemblyReferenceList<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRAssemblyIdentityManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwzassemblyreferences: *const ::windows::core::PCWSTR, dwnumofreferences: u32, ppreferencelist: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCLRAssemblyReferenceList(::core::mem::transmute_copy(&ppwzassemblyreferences), ::core::mem::transmute_copy(&dwnumofreferences)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppreferencelist, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBindingIdentityFromFile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRAssemblyIdentityManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzfilepath: ::windows::core::PCWSTR, dwflags: u32, pwzbuffer: ::windows::core::PWSTR, pcchbuffersize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetBindingIdentityFromFile(::core::mem::transmute(&pwzfilepath), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pwzbuffer), ::core::mem::transmute_copy(&pcchbuffersize)).into()
        }
        unsafe extern "system" fn GetBindingIdentityFromStream<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRAssemblyIdentityManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void, dwflags: u32, pwzbuffer: ::windows::core::PWSTR, pcchbuffersize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetBindingIdentityFromStream(::windows::core::from_raw_borrowed(&pstream), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pwzbuffer), ::core::mem::transmute_copy(&pcchbuffersize)).into()
        }
        unsafe extern "system" fn GetReferencedAssembliesFromFile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRAssemblyIdentityManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzfilepath: ::windows::core::PCWSTR, dwflags: u32, pexcludeassemblieslist: *mut ::core::ffi::c_void, ppreferenceenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetReferencedAssembliesFromFile(::core::mem::transmute(&pwzfilepath), ::core::mem::transmute_copy(&dwflags), ::windows::core::from_raw_borrowed(&pexcludeassemblieslist)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppreferenceenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetReferencedAssembliesFromStream<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRAssemblyIdentityManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void, dwflags: u32, pexcludeassemblieslist: *mut ::core::ffi::c_void, ppreferenceenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetReferencedAssembliesFromStream(::windows::core::from_raw_borrowed(&pstream), ::core::mem::transmute_copy(&dwflags), ::windows::core::from_raw_borrowed(&pexcludeassemblieslist)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppreferenceenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProbingAssembliesFromReference<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRAssemblyIdentityManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmachinetype: u32, dwflags: u32, pwzreferenceidentity: ::windows::core::PCWSTR, ppprobingassemblyenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProbingAssembliesFromReference(::core::mem::transmute_copy(&dwmachinetype), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pwzreferenceidentity)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppprobingassemblyenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsStronglyNamed<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRAssemblyIdentityManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzassemblyidentity: ::windows::core::PCWSTR, pbisstronglynamed: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsStronglyNamed(::core::mem::transmute(&pwzassemblyidentity), ::core::mem::transmute_copy(&pbisstronglynamed)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCLRAssemblyReferenceList: GetCLRAssemblyReferenceList::<Identity, Impl, OFFSET>,
            GetBindingIdentityFromFile: GetBindingIdentityFromFile::<Identity, Impl, OFFSET>,
            GetBindingIdentityFromStream: GetBindingIdentityFromStream::<Identity, Impl, OFFSET>,
            GetReferencedAssembliesFromFile: GetReferencedAssembliesFromFile::<Identity, Impl, OFFSET>,
            GetReferencedAssembliesFromStream: GetReferencedAssembliesFromStream::<Identity, Impl, OFFSET>,
            GetProbingAssembliesFromReference: GetProbingAssembliesFromReference::<Identity, Impl, OFFSET>,
            IsStronglyNamed: IsStronglyNamed::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICLRAssemblyIdentityManager as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait ICLRAssemblyReferenceList_Impl: Sized {
    fn IsStringAssemblyReferenceInList(&self, pwzassemblyname: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn IsAssemblyReferenceInList(&self, pname: ::core::option::Option<&::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ICLRAssemblyReferenceList {}
impl ICLRAssemblyReferenceList_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRAssemblyReferenceList_Impl, const OFFSET: isize>() -> ICLRAssemblyReferenceList_Vtbl {
        unsafe extern "system" fn IsStringAssemblyReferenceInList<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRAssemblyReferenceList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzassemblyname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsStringAssemblyReferenceInList(::core::mem::transmute(&pwzassemblyname)).into()
        }
        unsafe extern "system" fn IsAssemblyReferenceInList<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRAssemblyReferenceList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsAssemblyReferenceInList(::windows::core::from_raw_borrowed(&pname)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsStringAssemblyReferenceInList: IsStringAssemblyReferenceInList::<Identity, Impl, OFFSET>,
            IsAssemblyReferenceInList: IsAssemblyReferenceInList::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICLRAssemblyReferenceList as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait ICLRControl_Impl: Sized {
    fn GetCLRManager(&self, riid: *const ::windows::core::GUID, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn SetAppDomainManagerType(&self, pwzappdomainmanagerassembly: &::windows::core::PCWSTR, pwzappdomainmanagertype: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ICLRControl {}
impl ICLRControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRControl_Impl, const OFFSET: isize>() -> ICLRControl_Vtbl {
        unsafe extern "system" fn GetCLRManager<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCLRManager(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppobject)).into()
        }
        unsafe extern "system" fn SetAppDomainManagerType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzappdomainmanagerassembly: ::windows::core::PCWSTR, pwzappdomainmanagertype: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAppDomainManagerType(::core::mem::transmute(&pwzappdomainmanagerassembly), ::core::mem::transmute(&pwzappdomainmanagertype)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCLRManager: GetCLRManager::<Identity, Impl, OFFSET>,
            SetAppDomainManagerType: SetAppDomainManagerType::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICLRControl as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub trait ICLRDebugManager_Impl: Sized {
    fn BeginConnection(&self, dwconnectionid: u32, szconnectionname: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn SetConnectionTasks(&self, id: u32, dwcount: u32) -> ::windows::core::Result<ICLRTask>;
    fn EndConnection(&self, dwconnectionid: u32) -> ::windows::core::Result<()>;
    fn SetDacl(&self, pacl: *mut super::super::Security::ACL) -> ::windows::core::Result<()>;
    fn GetDacl(&self, pacl: *mut *mut super::super::Security::ACL) -> ::windows::core::Result<()>;
    fn IsDebuggerAttached(&self, pbattached: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetSymbolReadingPolicy(&self, policy: ESymbolReadingPolicy) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::windows::core::RuntimeName for ICLRDebugManager {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ICLRDebugManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRDebugManager_Impl, const OFFSET: isize>() -> ICLRDebugManager_Vtbl {
        unsafe extern "system" fn BeginConnection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRDebugManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwconnectionid: u32, szconnectionname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BeginConnection(::core::mem::transmute_copy(&dwconnectionid), ::core::mem::transmute(&szconnectionname)).into()
        }
        unsafe extern "system" fn SetConnectionTasks<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRDebugManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: u32, dwcount: u32, ppclrtask: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SetConnectionTasks(::core::mem::transmute_copy(&id), ::core::mem::transmute_copy(&dwcount)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclrtask, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndConnection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRDebugManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwconnectionid: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndConnection(::core::mem::transmute_copy(&dwconnectionid)).into()
        }
        unsafe extern "system" fn SetDacl<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRDebugManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pacl: *mut super::super::Security::ACL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDacl(::core::mem::transmute_copy(&pacl)).into()
        }
        unsafe extern "system" fn GetDacl<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRDebugManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pacl: *mut *mut super::super::Security::ACL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDacl(::core::mem::transmute_copy(&pacl)).into()
        }
        unsafe extern "system" fn IsDebuggerAttached<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRDebugManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbattached: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsDebuggerAttached(::core::mem::transmute_copy(&pbattached)).into()
        }
        unsafe extern "system" fn SetSymbolReadingPolicy<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRDebugManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, policy: ESymbolReadingPolicy) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSymbolReadingPolicy(::core::mem::transmute_copy(&policy)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginConnection: BeginConnection::<Identity, Impl, OFFSET>,
            SetConnectionTasks: SetConnectionTasks::<Identity, Impl, OFFSET>,
            EndConnection: EndConnection::<Identity, Impl, OFFSET>,
            SetDacl: SetDacl::<Identity, Impl, OFFSET>,
            GetDacl: GetDacl::<Identity, Impl, OFFSET>,
            IsDebuggerAttached: IsDebuggerAttached::<Identity, Impl, OFFSET>,
            SetSymbolReadingPolicy: SetSymbolReadingPolicy::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICLRDebugManager as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICLRDebugging_Impl: Sized {
    fn OpenVirtualProcess(&self, modulebaseaddress: u64, pdatatarget: ::core::option::Option<&::windows::core::IUnknown>, plibraryprovider: ::core::option::Option<&ICLRDebuggingLibraryProvider>, pmaxdebuggersupportedversion: *mut CLR_DEBUGGING_VERSION, riidprocess: *const ::windows::core::GUID, ppprocess: *mut ::core::option::Option<::windows::core::IUnknown>, pversion: *mut CLR_DEBUGGING_VERSION, pdwflags: *mut CLR_DEBUGGING_PROCESS_FLAGS) -> ::windows::core::Result<()>;
    fn CanUnloadNow(&self, hmodule: super::super::Foundation::HMODULE) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ICLRDebugging {}
#[cfg(feature = "Win32_Foundation")]
impl ICLRDebugging_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRDebugging_Impl, const OFFSET: isize>() -> ICLRDebugging_Vtbl {
        unsafe extern "system" fn OpenVirtualProcess<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRDebugging_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, modulebaseaddress: u64, pdatatarget: *mut ::core::ffi::c_void, plibraryprovider: *mut ::core::ffi::c_void, pmaxdebuggersupportedversion: *mut CLR_DEBUGGING_VERSION, riidprocess: *const ::windows::core::GUID, ppprocess: *mut *mut ::core::ffi::c_void, pversion: *mut CLR_DEBUGGING_VERSION, pdwflags: *mut CLR_DEBUGGING_PROCESS_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OpenVirtualProcess(::core::mem::transmute_copy(&modulebaseaddress), ::windows::core::from_raw_borrowed(&pdatatarget), ::windows::core::from_raw_borrowed(&plibraryprovider), ::core::mem::transmute_copy(&pmaxdebuggersupportedversion), ::core::mem::transmute_copy(&riidprocess), ::core::mem::transmute_copy(&ppprocess), ::core::mem::transmute_copy(&pversion), ::core::mem::transmute_copy(&pdwflags)).into()
        }
        unsafe extern "system" fn CanUnloadNow<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRDebugging_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmodule: super::super::Foundation::HMODULE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CanUnloadNow(::core::mem::transmute_copy(&hmodule)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OpenVirtualProcess: OpenVirtualProcess::<Identity, Impl, OFFSET>,
            CanUnloadNow: CanUnloadNow::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICLRDebugging as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICLRDebuggingLibraryProvider_Impl: Sized {
    fn ProvideLibrary(&self, pwszfilename: &::windows::core::PCWSTR, dwtimestamp: u32, dwsizeofimage: u32, phmodule: *mut super::super::Foundation::HMODULE) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ICLRDebuggingLibraryProvider {}
#[cfg(feature = "Win32_Foundation")]
impl ICLRDebuggingLibraryProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRDebuggingLibraryProvider_Impl, const OFFSET: isize>() -> ICLRDebuggingLibraryProvider_Vtbl {
        unsafe extern "system" fn ProvideLibrary<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRDebuggingLibraryProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszfilename: ::windows::core::PCWSTR, dwtimestamp: u32, dwsizeofimage: u32, phmodule: *mut super::super::Foundation::HMODULE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ProvideLibrary(::core::mem::transmute(&pwszfilename), ::core::mem::transmute_copy(&dwtimestamp), ::core::mem::transmute_copy(&dwsizeofimage), ::core::mem::transmute_copy(&phmodule)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ProvideLibrary: ProvideLibrary::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICLRDebuggingLibraryProvider as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait ICLRDomainManager_Impl: Sized {
    fn SetAppDomainManagerType(&self, wszappdomainmanagerassembly: &::windows::core::PCWSTR, wszappdomainmanagertype: &::windows::core::PCWSTR, dwinitializedomainflags: EInitializeNewDomainFlags) -> ::windows::core::Result<()>;
    fn SetPropertiesForDefaultAppDomain(&self, nproperties: u32, pwszpropertynames: *const ::windows::core::PCWSTR, pwszpropertyvalues: *const ::windows::core::PCWSTR) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ICLRDomainManager {}
impl ICLRDomainManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRDomainManager_Impl, const OFFSET: isize>() -> ICLRDomainManager_Vtbl {
        unsafe extern "system" fn SetAppDomainManagerType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRDomainManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszappdomainmanagerassembly: ::windows::core::PCWSTR, wszappdomainmanagertype: ::windows::core::PCWSTR, dwinitializedomainflags: EInitializeNewDomainFlags) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAppDomainManagerType(::core::mem::transmute(&wszappdomainmanagerassembly), ::core::mem::transmute(&wszappdomainmanagertype), ::core::mem::transmute_copy(&dwinitializedomainflags)).into()
        }
        unsafe extern "system" fn SetPropertiesForDefaultAppDomain<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRDomainManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nproperties: u32, pwszpropertynames: *const ::windows::core::PCWSTR, pwszpropertyvalues: *const ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPropertiesForDefaultAppDomain(::core::mem::transmute_copy(&nproperties), ::core::mem::transmute_copy(&pwszpropertynames), ::core::mem::transmute_copy(&pwszpropertyvalues)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetAppDomainManagerType: SetAppDomainManagerType::<Identity, Impl, OFFSET>,
            SetPropertiesForDefaultAppDomain: SetPropertiesForDefaultAppDomain::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICLRDomainManager as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICLRErrorReportingManager_Impl: Sized {
    fn GetBucketParametersForCurrentException(&self, pparams: *mut BucketParameters) -> ::windows::core::Result<()>;
    fn BeginCustomDump(&self, dwflavor: ECustomDumpFlavor, dwnumitems: u32, items: *mut CustomDumpItem, dwreserved: u32) -> ::windows::core::Result<()>;
    fn EndCustomDump(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ICLRErrorReportingManager {}
#[cfg(feature = "Win32_Foundation")]
impl ICLRErrorReportingManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRErrorReportingManager_Impl, const OFFSET: isize>() -> ICLRErrorReportingManager_Vtbl {
        unsafe extern "system" fn GetBucketParametersForCurrentException<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRErrorReportingManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparams: *mut BucketParameters) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetBucketParametersForCurrentException(::core::mem::transmute_copy(&pparams)).into()
        }
        unsafe extern "system" fn BeginCustomDump<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRErrorReportingManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflavor: ECustomDumpFlavor, dwnumitems: u32, items: *mut CustomDumpItem, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BeginCustomDump(::core::mem::transmute_copy(&dwflavor), ::core::mem::transmute_copy(&dwnumitems), ::core::mem::transmute_copy(&items), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn EndCustomDump<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRErrorReportingManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndCustomDump().into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetBucketParametersForCurrentException: GetBucketParametersForCurrentException::<Identity, Impl, OFFSET>,
            BeginCustomDump: BeginCustomDump::<Identity, Impl, OFFSET>,
            EndCustomDump: EndCustomDump::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICLRErrorReportingManager as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait ICLRGCManager_Impl: Sized {
    fn Collect(&self, generation: i32) -> ::windows::core::Result<()>;
    fn GetStats(&self, pstats: *mut COR_GC_STATS) -> ::windows::core::Result<()>;
    fn SetGCStartupLimits(&self, segmentsize: u32, maxgen0size: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ICLRGCManager {}
impl ICLRGCManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRGCManager_Impl, const OFFSET: isize>() -> ICLRGCManager_Vtbl {
        unsafe extern "system" fn Collect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRGCManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, generation: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Collect(::core::mem::transmute_copy(&generation)).into()
        }
        unsafe extern "system" fn GetStats<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRGCManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstats: *mut COR_GC_STATS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetStats(::core::mem::transmute_copy(&pstats)).into()
        }
        unsafe extern "system" fn SetGCStartupLimits<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRGCManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, segmentsize: u32, maxgen0size: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetGCStartupLimits(::core::mem::transmute_copy(&segmentsize), ::core::mem::transmute_copy(&maxgen0size)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Collect: Collect::<Identity, Impl, OFFSET>,
            GetStats: GetStats::<Identity, Impl, OFFSET>,
            SetGCStartupLimits: SetGCStartupLimits::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICLRGCManager as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait ICLRGCManager2_Impl: Sized + ICLRGCManager_Impl {
    fn SetGCStartupLimitsEx(&self, segmentsize: usize, maxgen0size: usize) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ICLRGCManager2 {}
impl ICLRGCManager2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRGCManager2_Impl, const OFFSET: isize>() -> ICLRGCManager2_Vtbl {
        unsafe extern "system" fn SetGCStartupLimitsEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRGCManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, segmentsize: usize, maxgen0size: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetGCStartupLimitsEx(::core::mem::transmute_copy(&segmentsize), ::core::mem::transmute_copy(&maxgen0size)).into()
        }
        Self { base__: ICLRGCManager_Vtbl::new::<Identity, Impl, OFFSET>(), SetGCStartupLimitsEx: SetGCStartupLimitsEx::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICLRGCManager2 as ::windows::core::ComInterface>::IID || iid == &<ICLRGCManager as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait ICLRHostBindingPolicyManager_Impl: Sized {
    fn ModifyApplicationPolicy(&self, pwzsourceassemblyidentity: &::windows::core::PCWSTR, pwztargetassemblyidentity: &::windows::core::PCWSTR, pbapplicationpolicy: *mut u8, cbapppolicysize: u32, dwpolicymodifyflags: u32, pbnewapplicationpolicy: *mut u8, pcbnewapppolicysize: *mut u32) -> ::windows::core::Result<()>;
    fn EvaluatePolicy(&self, pwzreferenceidentity: &::windows::core::PCWSTR, pbapplicationpolicy: *mut u8, cbapppolicysize: u32, pwzpostpolicyreferenceidentity: ::windows::core::PWSTR, pcchpostpolicyreferenceidentity: *mut u32, pdwpoliciesapplied: *mut u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ICLRHostBindingPolicyManager {}
impl ICLRHostBindingPolicyManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRHostBindingPolicyManager_Impl, const OFFSET: isize>() -> ICLRHostBindingPolicyManager_Vtbl {
        unsafe extern "system" fn ModifyApplicationPolicy<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRHostBindingPolicyManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzsourceassemblyidentity: ::windows::core::PCWSTR, pwztargetassemblyidentity: ::windows::core::PCWSTR, pbapplicationpolicy: *mut u8, cbapppolicysize: u32, dwpolicymodifyflags: u32, pbnewapplicationpolicy: *mut u8, pcbnewapppolicysize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ModifyApplicationPolicy(::core::mem::transmute(&pwzsourceassemblyidentity), ::core::mem::transmute(&pwztargetassemblyidentity), ::core::mem::transmute_copy(&pbapplicationpolicy), ::core::mem::transmute_copy(&cbapppolicysize), ::core::mem::transmute_copy(&dwpolicymodifyflags), ::core::mem::transmute_copy(&pbnewapplicationpolicy), ::core::mem::transmute_copy(&pcbnewapppolicysize)).into()
        }
        unsafe extern "system" fn EvaluatePolicy<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRHostBindingPolicyManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzreferenceidentity: ::windows::core::PCWSTR, pbapplicationpolicy: *mut u8, cbapppolicysize: u32, pwzpostpolicyreferenceidentity: ::windows::core::PWSTR, pcchpostpolicyreferenceidentity: *mut u32, pdwpoliciesapplied: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EvaluatePolicy(::core::mem::transmute(&pwzreferenceidentity), ::core::mem::transmute_copy(&pbapplicationpolicy), ::core::mem::transmute_copy(&cbapppolicysize), ::core::mem::transmute_copy(&pwzpostpolicyreferenceidentity), ::core::mem::transmute_copy(&pcchpostpolicyreferenceidentity), ::core::mem::transmute_copy(&pdwpoliciesapplied)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ModifyApplicationPolicy: ModifyApplicationPolicy::<Identity, Impl, OFFSET>,
            EvaluatePolicy: EvaluatePolicy::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICLRHostBindingPolicyManager as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait ICLRHostProtectionManager_Impl: Sized {
    fn SetProtectedCategories(&self, categories: EApiCategories) -> ::windows::core::Result<()>;
    fn SetEagerSerializeGrantSets(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ICLRHostProtectionManager {}
impl ICLRHostProtectionManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRHostProtectionManager_Impl, const OFFSET: isize>() -> ICLRHostProtectionManager_Vtbl {
        unsafe extern "system" fn SetProtectedCategories<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRHostProtectionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, categories: EApiCategories) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProtectedCategories(::core::mem::transmute_copy(&categories)).into()
        }
        unsafe extern "system" fn SetEagerSerializeGrantSets<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRHostProtectionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEagerSerializeGrantSets().into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetProtectedCategories: SetProtectedCategories::<Identity, Impl, OFFSET>,
            SetEagerSerializeGrantSets: SetEagerSerializeGrantSets::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICLRHostProtectionManager as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait ICLRIoCompletionManager_Impl: Sized {
    fn OnComplete(&self, dwerrorcode: u32, numberofbytestransferred: u32, pvoverlapped: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ICLRIoCompletionManager {}
impl ICLRIoCompletionManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRIoCompletionManager_Impl, const OFFSET: isize>() -> ICLRIoCompletionManager_Vtbl {
        unsafe extern "system" fn OnComplete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRIoCompletionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwerrorcode: u32, numberofbytestransferred: u32, pvoverlapped: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnComplete(::core::mem::transmute_copy(&dwerrorcode), ::core::mem::transmute_copy(&numberofbytestransferred), ::core::mem::transmute_copy(&pvoverlapped)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnComplete: OnComplete::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICLRIoCompletionManager as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait ICLRMemoryNotificationCallback_Impl: Sized {
    fn OnMemoryNotification(&self, ememoryavailable: EMemoryAvailable) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ICLRMemoryNotificationCallback {}
impl ICLRMemoryNotificationCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRMemoryNotificationCallback_Impl, const OFFSET: isize>() -> ICLRMemoryNotificationCallback_Vtbl {
        unsafe extern "system" fn OnMemoryNotification<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRMemoryNotificationCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ememoryavailable: EMemoryAvailable) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnMemoryNotification(::core::mem::transmute_copy(&ememoryavailable)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnMemoryNotification: OnMemoryNotification::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICLRMemoryNotificationCallback as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ICLRMetaHost_Impl: Sized {
    fn GetRuntime(&self, pwzversion: &::windows::core::PCWSTR, riid: *const ::windows::core::GUID, ppruntime: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetVersionFromFile(&self, pwzfilepath: &::windows::core::PCWSTR, pwzbuffer: ::windows::core::PWSTR, pcchbuffer: *mut u32) -> ::windows::core::Result<()>;
    fn EnumerateInstalledRuntimes(&self) -> ::windows::core::Result<super::Com::IEnumUnknown>;
    fn EnumerateLoadedRuntimes(&self, hndprocess: super::super::Foundation::HANDLE) -> ::windows::core::Result<super::Com::IEnumUnknown>;
    fn RequestRuntimeLoadedNotification(&self, pcallbackfunction: RuntimeLoadedCallbackFnPtr) -> ::windows::core::Result<()>;
    fn QueryLegacyV2RuntimeBinding(&self, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn ExitProcess(&self, iexitcode: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for ICLRMetaHost {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ICLRMetaHost_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRMetaHost_Impl, const OFFSET: isize>() -> ICLRMetaHost_Vtbl {
        unsafe extern "system" fn GetRuntime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRMetaHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzversion: ::windows::core::PCWSTR, riid: *const ::windows::core::GUID, ppruntime: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetRuntime(::core::mem::transmute(&pwzversion), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppruntime)).into()
        }
        unsafe extern "system" fn GetVersionFromFile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRMetaHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzfilepath: ::windows::core::PCWSTR, pwzbuffer: ::windows::core::PWSTR, pcchbuffer: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetVersionFromFile(::core::mem::transmute(&pwzfilepath), ::core::mem::transmute_copy(&pwzbuffer), ::core::mem::transmute_copy(&pcchbuffer)).into()
        }
        unsafe extern "system" fn EnumerateInstalledRuntimes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRMetaHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumerateInstalledRuntimes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumerator, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateLoadedRuntimes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRMetaHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hndprocess: super::super::Foundation::HANDLE, ppenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumerateLoadedRuntimes(::core::mem::transmute_copy(&hndprocess)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumerator, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestRuntimeLoadedNotification<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRMetaHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallbackfunction: RuntimeLoadedCallbackFnPtr) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RequestRuntimeLoadedNotification(::core::mem::transmute_copy(&pcallbackfunction)).into()
        }
        unsafe extern "system" fn QueryLegacyV2RuntimeBinding<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRMetaHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.QueryLegacyV2RuntimeBinding(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppunk)).into()
        }
        unsafe extern "system" fn ExitProcess<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRMetaHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iexitcode: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ExitProcess(::core::mem::transmute_copy(&iexitcode)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetRuntime: GetRuntime::<Identity, Impl, OFFSET>,
            GetVersionFromFile: GetVersionFromFile::<Identity, Impl, OFFSET>,
            EnumerateInstalledRuntimes: EnumerateInstalledRuntimes::<Identity, Impl, OFFSET>,
            EnumerateLoadedRuntimes: EnumerateLoadedRuntimes::<Identity, Impl, OFFSET>,
            RequestRuntimeLoadedNotification: RequestRuntimeLoadedNotification::<Identity, Impl, OFFSET>,
            QueryLegacyV2RuntimeBinding: QueryLegacyV2RuntimeBinding::<Identity, Impl, OFFSET>,
            ExitProcess: ExitProcess::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICLRMetaHost as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait ICLRMetaHostPolicy_Impl: Sized {
    fn GetRequestedRuntime(&self, dwpolicyflags: METAHOST_POLICY_FLAGS, pwzbinary: &::windows::core::PCWSTR, pcfgstream: ::core::option::Option<&super::Com::IStream>, pwzversion: &::windows::core::PWSTR, pcchversion: *mut u32, pwzimageversion: ::windows::core::PWSTR, pcchimageversion: *mut u32, pdwconfigflags: *mut u32, riid: *const ::windows::core::GUID, ppruntime: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ICLRMetaHostPolicy {}
#[cfg(feature = "Win32_System_Com")]
impl ICLRMetaHostPolicy_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRMetaHostPolicy_Impl, const OFFSET: isize>() -> ICLRMetaHostPolicy_Vtbl {
        unsafe extern "system" fn GetRequestedRuntime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRMetaHostPolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwpolicyflags: METAHOST_POLICY_FLAGS, pwzbinary: ::windows::core::PCWSTR, pcfgstream: *mut ::core::ffi::c_void, pwzversion: ::windows::core::PWSTR, pcchversion: *mut u32, pwzimageversion: ::windows::core::PWSTR, pcchimageversion: *mut u32, pdwconfigflags: *mut u32, riid: *const ::windows::core::GUID, ppruntime: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetRequestedRuntime(::core::mem::transmute_copy(&dwpolicyflags), ::core::mem::transmute(&pwzbinary), ::windows::core::from_raw_borrowed(&pcfgstream), ::core::mem::transmute(&pwzversion), ::core::mem::transmute_copy(&pcchversion), ::core::mem::transmute_copy(&pwzimageversion), ::core::mem::transmute_copy(&pcchimageversion), ::core::mem::transmute_copy(&pdwconfigflags), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppruntime)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetRequestedRuntime: GetRequestedRuntime::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICLRMetaHostPolicy as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait ICLROnEventManager_Impl: Sized {
    fn RegisterActionOnEvent(&self, event: EClrEvent, paction: ::core::option::Option<&IActionOnCLREvent>) -> ::windows::core::Result<()>;
    fn UnregisterActionOnEvent(&self, event: EClrEvent, paction: ::core::option::Option<&IActionOnCLREvent>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ICLROnEventManager {}
impl ICLROnEventManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLROnEventManager_Impl, const OFFSET: isize>() -> ICLROnEventManager_Vtbl {
        unsafe extern "system" fn RegisterActionOnEvent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLROnEventManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, event: EClrEvent, paction: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RegisterActionOnEvent(::core::mem::transmute_copy(&event), ::windows::core::from_raw_borrowed(&paction)).into()
        }
        unsafe extern "system" fn UnregisterActionOnEvent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLROnEventManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, event: EClrEvent, paction: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnregisterActionOnEvent(::core::mem::transmute_copy(&event), ::windows::core::from_raw_borrowed(&paction)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterActionOnEvent: RegisterActionOnEvent::<Identity, Impl, OFFSET>,
            UnregisterActionOnEvent: UnregisterActionOnEvent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICLROnEventManager as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait ICLRPolicyManager_Impl: Sized {
    fn SetDefaultAction(&self, operation: EClrOperation, action: EPolicyAction) -> ::windows::core::Result<()>;
    fn SetTimeout(&self, operation: EClrOperation, dwmilliseconds: u32) -> ::windows::core::Result<()>;
    fn SetActionOnTimeout(&self, operation: EClrOperation, action: EPolicyAction) -> ::windows::core::Result<()>;
    fn SetTimeoutAndAction(&self, operation: EClrOperation, dwmilliseconds: u32, action: EPolicyAction) -> ::windows::core::Result<()>;
    fn SetActionOnFailure(&self, failure: EClrFailure, action: EPolicyAction) -> ::windows::core::Result<()>;
    fn SetUnhandledExceptionPolicy(&self, policy: EClrUnhandledException) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ICLRPolicyManager {}
impl ICLRPolicyManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRPolicyManager_Impl, const OFFSET: isize>() -> ICLRPolicyManager_Vtbl {
        unsafe extern "system" fn SetDefaultAction<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRPolicyManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operation: EClrOperation, action: EPolicyAction) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDefaultAction(::core::mem::transmute_copy(&operation), ::core::mem::transmute_copy(&action)).into()
        }
        unsafe extern "system" fn SetTimeout<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRPolicyManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operation: EClrOperation, dwmilliseconds: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTimeout(::core::mem::transmute_copy(&operation), ::core::mem::transmute_copy(&dwmilliseconds)).into()
        }
        unsafe extern "system" fn SetActionOnTimeout<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRPolicyManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operation: EClrOperation, action: EPolicyAction) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetActionOnTimeout(::core::mem::transmute_copy(&operation), ::core::mem::transmute_copy(&action)).into()
        }
        unsafe extern "system" fn SetTimeoutAndAction<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRPolicyManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operation: EClrOperation, dwmilliseconds: u32, action: EPolicyAction) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTimeoutAndAction(::core::mem::transmute_copy(&operation), ::core::mem::transmute_copy(&dwmilliseconds), ::core::mem::transmute_copy(&action)).into()
        }
        unsafe extern "system" fn SetActionOnFailure<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRPolicyManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, failure: EClrFailure, action: EPolicyAction) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetActionOnFailure(::core::mem::transmute_copy(&failure), ::core::mem::transmute_copy(&action)).into()
        }
        unsafe extern "system" fn SetUnhandledExceptionPolicy<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRPolicyManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, policy: EClrUnhandledException) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetUnhandledExceptionPolicy(::core::mem::transmute_copy(&policy)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetDefaultAction: SetDefaultAction::<Identity, Impl, OFFSET>,
            SetTimeout: SetTimeout::<Identity, Impl, OFFSET>,
            SetActionOnTimeout: SetActionOnTimeout::<Identity, Impl, OFFSET>,
            SetTimeoutAndAction: SetTimeoutAndAction::<Identity, Impl, OFFSET>,
            SetActionOnFailure: SetActionOnFailure::<Identity, Impl, OFFSET>,
            SetUnhandledExceptionPolicy: SetUnhandledExceptionPolicy::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICLRPolicyManager as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait ICLRProbingAssemblyEnum_Impl: Sized {
    fn Get(&self, dwindex: u32, pwzbuffer: ::windows::core::PWSTR, pcchbuffersize: *mut u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ICLRProbingAssemblyEnum {}
impl ICLRProbingAssemblyEnum_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRProbingAssemblyEnum_Impl, const OFFSET: isize>() -> ICLRProbingAssemblyEnum_Vtbl {
        unsafe extern "system" fn Get<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRProbingAssemblyEnum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, pwzbuffer: ::windows::core::PWSTR, pcchbuffersize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Get(::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&pwzbuffer), ::core::mem::transmute_copy(&pcchbuffersize)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Get: Get::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICLRProbingAssemblyEnum as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait ICLRProfiling_Impl: Sized {
    fn AttachProfiler(&self, dwprofileeprocessid: u32, dwmillisecondsmax: u32, pclsidprofiler: *const ::windows::core::GUID, wszprofilerpath: &::windows::core::PCWSTR, pvclientdata: *mut ::core::ffi::c_void, cbclientdata: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ICLRProfiling {}
impl ICLRProfiling_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRProfiling_Impl, const OFFSET: isize>() -> ICLRProfiling_Vtbl {
        unsafe extern "system" fn AttachProfiler<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRProfiling_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwprofileeprocessid: u32, dwmillisecondsmax: u32, pclsidprofiler: *const ::windows::core::GUID, wszprofilerpath: ::windows::core::PCWSTR, pvclientdata: *mut ::core::ffi::c_void, cbclientdata: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AttachProfiler(::core::mem::transmute_copy(&dwprofileeprocessid), ::core::mem::transmute_copy(&dwmillisecondsmax), ::core::mem::transmute_copy(&pclsidprofiler), ::core::mem::transmute(&wszprofilerpath), ::core::mem::transmute_copy(&pvclientdata), ::core::mem::transmute_copy(&cbclientdata)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AttachProfiler: AttachProfiler::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICLRProfiling as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait ICLRReferenceAssemblyEnum_Impl: Sized {
    fn Get(&self, dwindex: u32, pwzbuffer: ::windows::core::PWSTR, pcchbuffersize: *mut u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ICLRReferenceAssemblyEnum {}
impl ICLRReferenceAssemblyEnum_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRReferenceAssemblyEnum_Impl, const OFFSET: isize>() -> ICLRReferenceAssemblyEnum_Vtbl {
        unsafe extern "system" fn Get<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRReferenceAssemblyEnum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, pwzbuffer: ::windows::core::PWSTR, pcchbuffersize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Get(::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&pwzbuffer), ::core::mem::transmute_copy(&pcchbuffersize)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Get: Get::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICLRReferenceAssemblyEnum as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICLRRuntimeHost_Impl: Sized {
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
    fn SetHostControl(&self, phostcontrol: ::core::option::Option<&IHostControl>) -> ::windows::core::Result<()>;
    fn GetCLRControl(&self) -> ::windows::core::Result<ICLRControl>;
    fn UnloadAppDomain(&self, dwappdomainid: u32, fwaituntildone: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn ExecuteInAppDomain(&self, dwappdomainid: u32, pcallback: FExecuteInAppDomainCallback, cookie: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetCurrentAppDomainId(&self, pdwappdomainid: *mut u32) -> ::windows::core::Result<()>;
    fn ExecuteApplication(&self, pwzappfullname: &::windows::core::PCWSTR, dwmanifestpaths: u32, ppwzmanifestpaths: *const ::windows::core::PCWSTR, dwactivationdata: u32, ppwzactivationdata: *const ::windows::core::PCWSTR, preturnvalue: *mut i32) -> ::windows::core::Result<()>;
    fn ExecuteInDefaultAppDomain(&self, pwzassemblypath: &::windows::core::PCWSTR, pwztypename: &::windows::core::PCWSTR, pwzmethodname: &::windows::core::PCWSTR, pwzargument: &::windows::core::PCWSTR, preturnvalue: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ICLRRuntimeHost {}
#[cfg(feature = "Win32_Foundation")]
impl ICLRRuntimeHost_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRRuntimeHost_Impl, const OFFSET: isize>() -> ICLRRuntimeHost_Vtbl {
        unsafe extern "system" fn Start<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRRuntimeHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Start().into()
        }
        unsafe extern "system" fn Stop<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRRuntimeHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Stop().into()
        }
        unsafe extern "system" fn SetHostControl<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRRuntimeHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phostcontrol: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetHostControl(::windows::core::from_raw_borrowed(&phostcontrol)).into()
        }
        unsafe extern "system" fn GetCLRControl<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRRuntimeHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclrcontrol: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCLRControl() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pclrcontrol, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnloadAppDomain<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRRuntimeHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwappdomainid: u32, fwaituntildone: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnloadAppDomain(::core::mem::transmute_copy(&dwappdomainid), ::core::mem::transmute_copy(&fwaituntildone)).into()
        }
        unsafe extern "system" fn ExecuteInAppDomain<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRRuntimeHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwappdomainid: u32, pcallback: FExecuteInAppDomainCallback, cookie: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ExecuteInAppDomain(::core::mem::transmute_copy(&dwappdomainid), ::core::mem::transmute_copy(&pcallback), ::core::mem::transmute_copy(&cookie)).into()
        }
        unsafe extern "system" fn GetCurrentAppDomainId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRRuntimeHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwappdomainid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCurrentAppDomainId(::core::mem::transmute_copy(&pdwappdomainid)).into()
        }
        unsafe extern "system" fn ExecuteApplication<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRRuntimeHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzappfullname: ::windows::core::PCWSTR, dwmanifestpaths: u32, ppwzmanifestpaths: *const ::windows::core::PCWSTR, dwactivationdata: u32, ppwzactivationdata: *const ::windows::core::PCWSTR, preturnvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ExecuteApplication(::core::mem::transmute(&pwzappfullname), ::core::mem::transmute_copy(&dwmanifestpaths), ::core::mem::transmute_copy(&ppwzmanifestpaths), ::core::mem::transmute_copy(&dwactivationdata), ::core::mem::transmute_copy(&ppwzactivationdata), ::core::mem::transmute_copy(&preturnvalue)).into()
        }
        unsafe extern "system" fn ExecuteInDefaultAppDomain<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRRuntimeHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzassemblypath: ::windows::core::PCWSTR, pwztypename: ::windows::core::PCWSTR, pwzmethodname: ::windows::core::PCWSTR, pwzargument: ::windows::core::PCWSTR, preturnvalue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ExecuteInDefaultAppDomain(::core::mem::transmute(&pwzassemblypath), ::core::mem::transmute(&pwztypename), ::core::mem::transmute(&pwzmethodname), ::core::mem::transmute(&pwzargument), ::core::mem::transmute_copy(&preturnvalue)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICLRRuntimeHost as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICLRRuntimeInfo_Impl: Sized {
    fn GetVersionString(&self, pwzbuffer: ::windows::core::PWSTR, pcchbuffer: *mut u32) -> ::windows::core::Result<()>;
    fn GetRuntimeDirectory(&self, pwzbuffer: ::windows::core::PWSTR, pcchbuffer: *mut u32) -> ::windows::core::Result<()>;
    fn IsLoaded(&self, hndprocess: super::super::Foundation::HANDLE, pbloaded: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn LoadErrorString(&self, iresourceid: u32, pwzbuffer: ::windows::core::PWSTR, pcchbuffer: *mut u32, ilocaleid: i32) -> ::windows::core::Result<()>;
    fn LoadLibraryA(&self, pwzdllname: &::windows::core::PCWSTR, phndmodule: *mut super::super::Foundation::HMODULE) -> ::windows::core::Result<()>;
    fn GetProcAddress(&self, pszprocname: &::windows::core::PCSTR, ppproc: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetInterface(&self, rclsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn IsLoadable(&self, pbloadable: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetDefaultStartupFlags(&self, dwstartupflags: u32, pwzhostconfigfile: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn GetDefaultStartupFlags(&self, pdwstartupflags: *mut u32, pwzhostconfigfile: ::windows::core::PWSTR, pcchhostconfigfile: *mut u32) -> ::windows::core::Result<()>;
    fn BindAsLegacyV2Runtime(&self) -> ::windows::core::Result<()>;
    fn IsStarted(&self, pbstarted: *mut super::super::Foundation::BOOL, pdwstartupflags: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ICLRRuntimeInfo {}
#[cfg(feature = "Win32_Foundation")]
impl ICLRRuntimeInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRRuntimeInfo_Impl, const OFFSET: isize>() -> ICLRRuntimeInfo_Vtbl {
        unsafe extern "system" fn GetVersionString<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRRuntimeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzbuffer: ::windows::core::PWSTR, pcchbuffer: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetVersionString(::core::mem::transmute_copy(&pwzbuffer), ::core::mem::transmute_copy(&pcchbuffer)).into()
        }
        unsafe extern "system" fn GetRuntimeDirectory<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRRuntimeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzbuffer: ::windows::core::PWSTR, pcchbuffer: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetRuntimeDirectory(::core::mem::transmute_copy(&pwzbuffer), ::core::mem::transmute_copy(&pcchbuffer)).into()
        }
        unsafe extern "system" fn IsLoaded<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRRuntimeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hndprocess: super::super::Foundation::HANDLE, pbloaded: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsLoaded(::core::mem::transmute_copy(&hndprocess), ::core::mem::transmute_copy(&pbloaded)).into()
        }
        unsafe extern "system" fn LoadErrorString<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRRuntimeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iresourceid: u32, pwzbuffer: ::windows::core::PWSTR, pcchbuffer: *mut u32, ilocaleid: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LoadErrorString(::core::mem::transmute_copy(&iresourceid), ::core::mem::transmute_copy(&pwzbuffer), ::core::mem::transmute_copy(&pcchbuffer), ::core::mem::transmute_copy(&ilocaleid)).into()
        }
        unsafe extern "system" fn LoadLibraryA<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRRuntimeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzdllname: ::windows::core::PCWSTR, phndmodule: *mut super::super::Foundation::HMODULE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LoadLibraryA(::core::mem::transmute(&pwzdllname), ::core::mem::transmute_copy(&phndmodule)).into()
        }
        unsafe extern "system" fn GetProcAddress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRRuntimeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszprocname: ::windows::core::PCSTR, ppproc: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetProcAddress(::core::mem::transmute(&pszprocname), ::core::mem::transmute_copy(&ppproc)).into()
        }
        unsafe extern "system" fn GetInterface<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRRuntimeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetInterface(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppunk)).into()
        }
        unsafe extern "system" fn IsLoadable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRRuntimeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbloadable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsLoadable(::core::mem::transmute_copy(&pbloadable)).into()
        }
        unsafe extern "system" fn SetDefaultStartupFlags<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRRuntimeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstartupflags: u32, pwzhostconfigfile: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDefaultStartupFlags(::core::mem::transmute_copy(&dwstartupflags), ::core::mem::transmute(&pwzhostconfigfile)).into()
        }
        unsafe extern "system" fn GetDefaultStartupFlags<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRRuntimeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstartupflags: *mut u32, pwzhostconfigfile: ::windows::core::PWSTR, pcchhostconfigfile: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDefaultStartupFlags(::core::mem::transmute_copy(&pdwstartupflags), ::core::mem::transmute_copy(&pwzhostconfigfile), ::core::mem::transmute_copy(&pcchhostconfigfile)).into()
        }
        unsafe extern "system" fn BindAsLegacyV2Runtime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRRuntimeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BindAsLegacyV2Runtime().into()
        }
        unsafe extern "system" fn IsStarted<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRRuntimeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstarted: *mut super::super::Foundation::BOOL, pdwstartupflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsStarted(::core::mem::transmute_copy(&pbstarted), ::core::mem::transmute_copy(&pdwstartupflags)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICLRRuntimeInfo as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICLRStrongName_Impl: Sized {
    fn GetHashFromAssemblyFile(&self, pszfilepath: &::windows::core::PCSTR, pihashalg: *mut u32, pbhash: *mut u8, cchhash: u32, pchhash: *mut u32) -> ::windows::core::Result<()>;
    fn GetHashFromAssemblyFileW(&self, pwzfilepath: &::windows::core::PCWSTR, pihashalg: *mut u32, pbhash: *mut u8, cchhash: u32, pchhash: *mut u32) -> ::windows::core::Result<()>;
    fn GetHashFromBlob(&self, pbblob: *mut u8, cchblob: u32, pihashalg: *mut u32, pbhash: *mut u8, cchhash: u32, pchhash: *mut u32) -> ::windows::core::Result<()>;
    fn GetHashFromFile(&self, pszfilepath: &::windows::core::PCSTR, pihashalg: *mut u32, pbhash: *mut u8, cchhash: u32, pchhash: *mut u32) -> ::windows::core::Result<()>;
    fn GetHashFromFileW(&self, pwzfilepath: &::windows::core::PCWSTR, pihashalg: *mut u32, pbhash: *mut u8, cchhash: u32, pchhash: *mut u32) -> ::windows::core::Result<()>;
    fn GetHashFromHandle(&self, hfile: super::super::Foundation::HANDLE, pihashalg: *mut u32, pbhash: *mut u8, cchhash: u32, pchhash: *mut u32) -> ::windows::core::Result<()>;
    fn StrongNameCompareAssemblies(&self, pwzassembly1: &::windows::core::PCWSTR, pwzassembly2: &::windows::core::PCWSTR, pdwresult: *mut u32) -> ::windows::core::Result<()>;
    fn StrongNameFreeBuffer(&self, pbmemory: *mut u8) -> ::windows::core::Result<()>;
    fn StrongNameGetBlob(&self, pwzfilepath: &::windows::core::PCWSTR, pbblob: *mut u8, pcbblob: *mut u32) -> ::windows::core::Result<()>;
    fn StrongNameGetBlobFromImage(&self, pbbase: *mut u8, dwlength: u32, pbblob: *mut u8, pcbblob: *mut u32) -> ::windows::core::Result<()>;
    fn StrongNameGetPublicKey(&self, pwzkeycontainer: &::windows::core::PCWSTR, pbkeyblob: *mut u8, cbkeyblob: u32, ppbpublickeyblob: *mut *mut u8, pcbpublickeyblob: *mut u32) -> ::windows::core::Result<()>;
    fn StrongNameHashSize(&self, ulhashalg: u32, pcbsize: *mut u32) -> ::windows::core::Result<()>;
    fn StrongNameKeyDelete(&self, pwzkeycontainer: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn StrongNameKeyGen(&self, pwzkeycontainer: &::windows::core::PCWSTR, dwflags: u32, ppbkeyblob: *mut *mut u8, pcbkeyblob: *mut u32) -> ::windows::core::Result<()>;
    fn StrongNameKeyGenEx(&self, pwzkeycontainer: &::windows::core::PCWSTR, dwflags: u32, dwkeysize: u32, ppbkeyblob: *mut *mut u8, pcbkeyblob: *mut u32) -> ::windows::core::Result<()>;
    fn StrongNameKeyInstall(&self, pwzkeycontainer: &::windows::core::PCWSTR, pbkeyblob: *mut u8, cbkeyblob: u32) -> ::windows::core::Result<()>;
    fn StrongNameSignatureGeneration(&self, pwzfilepath: &::windows::core::PCWSTR, pwzkeycontainer: &::windows::core::PCWSTR, pbkeyblob: *mut u8, cbkeyblob: u32, ppbsignatureblob: *mut *mut u8, pcbsignatureblob: *mut u32) -> ::windows::core::Result<()>;
    fn StrongNameSignatureGenerationEx(&self, wszfilepath: &::windows::core::PCWSTR, wszkeycontainer: &::windows::core::PCWSTR, pbkeyblob: *mut u8, cbkeyblob: u32, ppbsignatureblob: *mut *mut u8, pcbsignatureblob: *mut u32, dwflags: u32) -> ::windows::core::Result<()>;
    fn StrongNameSignatureSize(&self, pbpublickeyblob: *mut u8, cbpublickeyblob: u32, pcbsize: *mut u32) -> ::windows::core::Result<()>;
    fn StrongNameSignatureVerification(&self, pwzfilepath: &::windows::core::PCWSTR, dwinflags: u32, pdwoutflags: *mut u32) -> ::windows::core::Result<()>;
    fn StrongNameSignatureVerificationEx(&self, pwzfilepath: &::windows::core::PCWSTR, fforceverification: super::super::Foundation::BOOLEAN, pfwasverified: *mut u8) -> ::windows::core::Result<()>;
    fn StrongNameSignatureVerificationFromImage(&self, pbbase: *mut u8, dwlength: u32, dwinflags: u32, pdwoutflags: *mut u32) -> ::windows::core::Result<()>;
    fn StrongNameTokenFromAssembly(&self, pwzfilepath: &::windows::core::PCWSTR, ppbstrongnametoken: *mut *mut u8, pcbstrongnametoken: *mut u32) -> ::windows::core::Result<()>;
    fn StrongNameTokenFromAssemblyEx(&self, pwzfilepath: &::windows::core::PCWSTR, ppbstrongnametoken: *mut *mut u8, pcbstrongnametoken: *mut u32, ppbpublickeyblob: *mut *mut u8, pcbpublickeyblob: *mut u32) -> ::windows::core::Result<()>;
    fn StrongNameTokenFromPublicKey(&self, pbpublickeyblob: *mut u8, cbpublickeyblob: u32, ppbstrongnametoken: *mut *mut u8, pcbstrongnametoken: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ICLRStrongName {}
#[cfg(feature = "Win32_Foundation")]
impl ICLRStrongName_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: isize>() -> ICLRStrongName_Vtbl {
        unsafe extern "system" fn GetHashFromAssemblyFile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfilepath: ::windows::core::PCSTR, pihashalg: *mut u32, pbhash: *mut u8, cchhash: u32, pchhash: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetHashFromAssemblyFile(::core::mem::transmute(&pszfilepath), ::core::mem::transmute_copy(&pihashalg), ::core::mem::transmute_copy(&pbhash), ::core::mem::transmute_copy(&cchhash), ::core::mem::transmute_copy(&pchhash)).into()
        }
        unsafe extern "system" fn GetHashFromAssemblyFileW<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzfilepath: ::windows::core::PCWSTR, pihashalg: *mut u32, pbhash: *mut u8, cchhash: u32, pchhash: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetHashFromAssemblyFileW(::core::mem::transmute(&pwzfilepath), ::core::mem::transmute_copy(&pihashalg), ::core::mem::transmute_copy(&pbhash), ::core::mem::transmute_copy(&cchhash), ::core::mem::transmute_copy(&pchhash)).into()
        }
        unsafe extern "system" fn GetHashFromBlob<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbblob: *mut u8, cchblob: u32, pihashalg: *mut u32, pbhash: *mut u8, cchhash: u32, pchhash: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetHashFromBlob(::core::mem::transmute_copy(&pbblob), ::core::mem::transmute_copy(&cchblob), ::core::mem::transmute_copy(&pihashalg), ::core::mem::transmute_copy(&pbhash), ::core::mem::transmute_copy(&cchhash), ::core::mem::transmute_copy(&pchhash)).into()
        }
        unsafe extern "system" fn GetHashFromFile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfilepath: ::windows::core::PCSTR, pihashalg: *mut u32, pbhash: *mut u8, cchhash: u32, pchhash: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetHashFromFile(::core::mem::transmute(&pszfilepath), ::core::mem::transmute_copy(&pihashalg), ::core::mem::transmute_copy(&pbhash), ::core::mem::transmute_copy(&cchhash), ::core::mem::transmute_copy(&pchhash)).into()
        }
        unsafe extern "system" fn GetHashFromFileW<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzfilepath: ::windows::core::PCWSTR, pihashalg: *mut u32, pbhash: *mut u8, cchhash: u32, pchhash: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetHashFromFileW(::core::mem::transmute(&pwzfilepath), ::core::mem::transmute_copy(&pihashalg), ::core::mem::transmute_copy(&pbhash), ::core::mem::transmute_copy(&cchhash), ::core::mem::transmute_copy(&pchhash)).into()
        }
        unsafe extern "system" fn GetHashFromHandle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hfile: super::super::Foundation::HANDLE, pihashalg: *mut u32, pbhash: *mut u8, cchhash: u32, pchhash: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetHashFromHandle(::core::mem::transmute_copy(&hfile), ::core::mem::transmute_copy(&pihashalg), ::core::mem::transmute_copy(&pbhash), ::core::mem::transmute_copy(&cchhash), ::core::mem::transmute_copy(&pchhash)).into()
        }
        unsafe extern "system" fn StrongNameCompareAssemblies<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzassembly1: ::windows::core::PCWSTR, pwzassembly2: ::windows::core::PCWSTR, pdwresult: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StrongNameCompareAssemblies(::core::mem::transmute(&pwzassembly1), ::core::mem::transmute(&pwzassembly2), ::core::mem::transmute_copy(&pdwresult)).into()
        }
        unsafe extern "system" fn StrongNameFreeBuffer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbmemory: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StrongNameFreeBuffer(::core::mem::transmute_copy(&pbmemory)).into()
        }
        unsafe extern "system" fn StrongNameGetBlob<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzfilepath: ::windows::core::PCWSTR, pbblob: *mut u8, pcbblob: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StrongNameGetBlob(::core::mem::transmute(&pwzfilepath), ::core::mem::transmute_copy(&pbblob), ::core::mem::transmute_copy(&pcbblob)).into()
        }
        unsafe extern "system" fn StrongNameGetBlobFromImage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbbase: *mut u8, dwlength: u32, pbblob: *mut u8, pcbblob: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StrongNameGetBlobFromImage(::core::mem::transmute_copy(&pbbase), ::core::mem::transmute_copy(&dwlength), ::core::mem::transmute_copy(&pbblob), ::core::mem::transmute_copy(&pcbblob)).into()
        }
        unsafe extern "system" fn StrongNameGetPublicKey<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzkeycontainer: ::windows::core::PCWSTR, pbkeyblob: *mut u8, cbkeyblob: u32, ppbpublickeyblob: *mut *mut u8, pcbpublickeyblob: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StrongNameGetPublicKey(::core::mem::transmute(&pwzkeycontainer), ::core::mem::transmute_copy(&pbkeyblob), ::core::mem::transmute_copy(&cbkeyblob), ::core::mem::transmute_copy(&ppbpublickeyblob), ::core::mem::transmute_copy(&pcbpublickeyblob)).into()
        }
        unsafe extern "system" fn StrongNameHashSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulhashalg: u32, pcbsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StrongNameHashSize(::core::mem::transmute_copy(&ulhashalg), ::core::mem::transmute_copy(&pcbsize)).into()
        }
        unsafe extern "system" fn StrongNameKeyDelete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzkeycontainer: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StrongNameKeyDelete(::core::mem::transmute(&pwzkeycontainer)).into()
        }
        unsafe extern "system" fn StrongNameKeyGen<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzkeycontainer: ::windows::core::PCWSTR, dwflags: u32, ppbkeyblob: *mut *mut u8, pcbkeyblob: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StrongNameKeyGen(::core::mem::transmute(&pwzkeycontainer), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&ppbkeyblob), ::core::mem::transmute_copy(&pcbkeyblob)).into()
        }
        unsafe extern "system" fn StrongNameKeyGenEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzkeycontainer: ::windows::core::PCWSTR, dwflags: u32, dwkeysize: u32, ppbkeyblob: *mut *mut u8, pcbkeyblob: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StrongNameKeyGenEx(::core::mem::transmute(&pwzkeycontainer), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&dwkeysize), ::core::mem::transmute_copy(&ppbkeyblob), ::core::mem::transmute_copy(&pcbkeyblob)).into()
        }
        unsafe extern "system" fn StrongNameKeyInstall<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzkeycontainer: ::windows::core::PCWSTR, pbkeyblob: *mut u8, cbkeyblob: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StrongNameKeyInstall(::core::mem::transmute(&pwzkeycontainer), ::core::mem::transmute_copy(&pbkeyblob), ::core::mem::transmute_copy(&cbkeyblob)).into()
        }
        unsafe extern "system" fn StrongNameSignatureGeneration<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzfilepath: ::windows::core::PCWSTR, pwzkeycontainer: ::windows::core::PCWSTR, pbkeyblob: *mut u8, cbkeyblob: u32, ppbsignatureblob: *mut *mut u8, pcbsignatureblob: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StrongNameSignatureGeneration(::core::mem::transmute(&pwzfilepath), ::core::mem::transmute(&pwzkeycontainer), ::core::mem::transmute_copy(&pbkeyblob), ::core::mem::transmute_copy(&cbkeyblob), ::core::mem::transmute_copy(&ppbsignatureblob), ::core::mem::transmute_copy(&pcbsignatureblob)).into()
        }
        unsafe extern "system" fn StrongNameSignatureGenerationEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszfilepath: ::windows::core::PCWSTR, wszkeycontainer: ::windows::core::PCWSTR, pbkeyblob: *mut u8, cbkeyblob: u32, ppbsignatureblob: *mut *mut u8, pcbsignatureblob: *mut u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StrongNameSignatureGenerationEx(::core::mem::transmute(&wszfilepath), ::core::mem::transmute(&wszkeycontainer), ::core::mem::transmute_copy(&pbkeyblob), ::core::mem::transmute_copy(&cbkeyblob), ::core::mem::transmute_copy(&ppbsignatureblob), ::core::mem::transmute_copy(&pcbsignatureblob), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn StrongNameSignatureSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbpublickeyblob: *mut u8, cbpublickeyblob: u32, pcbsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StrongNameSignatureSize(::core::mem::transmute_copy(&pbpublickeyblob), ::core::mem::transmute_copy(&cbpublickeyblob), ::core::mem::transmute_copy(&pcbsize)).into()
        }
        unsafe extern "system" fn StrongNameSignatureVerification<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzfilepath: ::windows::core::PCWSTR, dwinflags: u32, pdwoutflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StrongNameSignatureVerification(::core::mem::transmute(&pwzfilepath), ::core::mem::transmute_copy(&dwinflags), ::core::mem::transmute_copy(&pdwoutflags)).into()
        }
        unsafe extern "system" fn StrongNameSignatureVerificationEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzfilepath: ::windows::core::PCWSTR, fforceverification: super::super::Foundation::BOOLEAN, pfwasverified: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StrongNameSignatureVerificationEx(::core::mem::transmute(&pwzfilepath), ::core::mem::transmute_copy(&fforceverification), ::core::mem::transmute_copy(&pfwasverified)).into()
        }
        unsafe extern "system" fn StrongNameSignatureVerificationFromImage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbbase: *mut u8, dwlength: u32, dwinflags: u32, pdwoutflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StrongNameSignatureVerificationFromImage(::core::mem::transmute_copy(&pbbase), ::core::mem::transmute_copy(&dwlength), ::core::mem::transmute_copy(&dwinflags), ::core::mem::transmute_copy(&pdwoutflags)).into()
        }
        unsafe extern "system" fn StrongNameTokenFromAssembly<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzfilepath: ::windows::core::PCWSTR, ppbstrongnametoken: *mut *mut u8, pcbstrongnametoken: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StrongNameTokenFromAssembly(::core::mem::transmute(&pwzfilepath), ::core::mem::transmute_copy(&ppbstrongnametoken), ::core::mem::transmute_copy(&pcbstrongnametoken)).into()
        }
        unsafe extern "system" fn StrongNameTokenFromAssemblyEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzfilepath: ::windows::core::PCWSTR, ppbstrongnametoken: *mut *mut u8, pcbstrongnametoken: *mut u32, ppbpublickeyblob: *mut *mut u8, pcbpublickeyblob: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StrongNameTokenFromAssemblyEx(::core::mem::transmute(&pwzfilepath), ::core::mem::transmute_copy(&ppbstrongnametoken), ::core::mem::transmute_copy(&pcbstrongnametoken), ::core::mem::transmute_copy(&ppbpublickeyblob), ::core::mem::transmute_copy(&pcbpublickeyblob)).into()
        }
        unsafe extern "system" fn StrongNameTokenFromPublicKey<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbpublickeyblob: *mut u8, cbpublickeyblob: u32, ppbstrongnametoken: *mut *mut u8, pcbstrongnametoken: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StrongNameTokenFromPublicKey(::core::mem::transmute_copy(&pbpublickeyblob), ::core::mem::transmute_copy(&cbpublickeyblob), ::core::mem::transmute_copy(&ppbstrongnametoken), ::core::mem::transmute_copy(&pcbstrongnametoken)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICLRStrongName as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICLRStrongName2_Impl: Sized {
    fn StrongNameGetPublicKeyEx(&self, pwzkeycontainer: &::windows::core::PCWSTR, pbkeyblob: *mut u8, cbkeyblob: u32, ppbpublickeyblob: *mut *mut u8, pcbpublickeyblob: *mut u32, uhashalgid: u32, ureserved: u32) -> ::windows::core::Result<()>;
    fn StrongNameSignatureVerificationEx2(&self, wszfilepath: &::windows::core::PCWSTR, fforceverification: super::super::Foundation::BOOLEAN, pbecmapublickey: *mut u8, cbecmapublickey: u32, pfwasverified: *mut u8) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ICLRStrongName2 {}
#[cfg(feature = "Win32_Foundation")]
impl ICLRStrongName2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRStrongName2_Impl, const OFFSET: isize>() -> ICLRStrongName2_Vtbl {
        unsafe extern "system" fn StrongNameGetPublicKeyEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRStrongName2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzkeycontainer: ::windows::core::PCWSTR, pbkeyblob: *mut u8, cbkeyblob: u32, ppbpublickeyblob: *mut *mut u8, pcbpublickeyblob: *mut u32, uhashalgid: u32, ureserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StrongNameGetPublicKeyEx(::core::mem::transmute(&pwzkeycontainer), ::core::mem::transmute_copy(&pbkeyblob), ::core::mem::transmute_copy(&cbkeyblob), ::core::mem::transmute_copy(&ppbpublickeyblob), ::core::mem::transmute_copy(&pcbpublickeyblob), ::core::mem::transmute_copy(&uhashalgid), ::core::mem::transmute_copy(&ureserved)).into()
        }
        unsafe extern "system" fn StrongNameSignatureVerificationEx2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRStrongName2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszfilepath: ::windows::core::PCWSTR, fforceverification: super::super::Foundation::BOOLEAN, pbecmapublickey: *mut u8, cbecmapublickey: u32, pfwasverified: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StrongNameSignatureVerificationEx2(::core::mem::transmute(&wszfilepath), ::core::mem::transmute_copy(&fforceverification), ::core::mem::transmute_copy(&pbecmapublickey), ::core::mem::transmute_copy(&cbecmapublickey), ::core::mem::transmute_copy(&pfwasverified)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            StrongNameGetPublicKeyEx: StrongNameGetPublicKeyEx::<Identity, Impl, OFFSET>,
            StrongNameSignatureVerificationEx2: StrongNameSignatureVerificationEx2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICLRStrongName2 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait ICLRStrongName3_Impl: Sized {
    fn StrongNameDigestGenerate(&self, wszfilepath: &::windows::core::PCWSTR, ppbdigestblob: *mut *mut u8, pcbdigestblob: *mut u32, dwflags: u32) -> ::windows::core::Result<()>;
    fn StrongNameDigestSign(&self, wszkeycontainer: &::windows::core::PCWSTR, pbkeyblob: *mut u8, cbkeyblob: u32, pbdigestblob: *mut u8, cbdigestblob: u32, hashalgid: u32, ppbsignatureblob: *mut *mut u8, pcbsignatureblob: *mut u32, dwflags: u32) -> ::windows::core::Result<()>;
    fn StrongNameDigestEmbed(&self, wszfilepath: &::windows::core::PCWSTR, pbsignatureblob: *mut u8, cbsignatureblob: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ICLRStrongName3 {}
impl ICLRStrongName3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRStrongName3_Impl, const OFFSET: isize>() -> ICLRStrongName3_Vtbl {
        unsafe extern "system" fn StrongNameDigestGenerate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRStrongName3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszfilepath: ::windows::core::PCWSTR, ppbdigestblob: *mut *mut u8, pcbdigestblob: *mut u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StrongNameDigestGenerate(::core::mem::transmute(&wszfilepath), ::core::mem::transmute_copy(&ppbdigestblob), ::core::mem::transmute_copy(&pcbdigestblob), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn StrongNameDigestSign<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRStrongName3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszkeycontainer: ::windows::core::PCWSTR, pbkeyblob: *mut u8, cbkeyblob: u32, pbdigestblob: *mut u8, cbdigestblob: u32, hashalgid: u32, ppbsignatureblob: *mut *mut u8, pcbsignatureblob: *mut u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StrongNameDigestSign(::core::mem::transmute(&wszkeycontainer), ::core::mem::transmute_copy(&pbkeyblob), ::core::mem::transmute_copy(&cbkeyblob), ::core::mem::transmute_copy(&pbdigestblob), ::core::mem::transmute_copy(&cbdigestblob), ::core::mem::transmute_copy(&hashalgid), ::core::mem::transmute_copy(&ppbsignatureblob), ::core::mem::transmute_copy(&pcbsignatureblob), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn StrongNameDigestEmbed<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRStrongName3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszfilepath: ::windows::core::PCWSTR, pbsignatureblob: *mut u8, cbsignatureblob: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StrongNameDigestEmbed(::core::mem::transmute(&wszfilepath), ::core::mem::transmute_copy(&pbsignatureblob), ::core::mem::transmute_copy(&cbsignatureblob)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            StrongNameDigestGenerate: StrongNameDigestGenerate::<Identity, Impl, OFFSET>,
            StrongNameDigestSign: StrongNameDigestSign::<Identity, Impl, OFFSET>,
            StrongNameDigestEmbed: StrongNameDigestEmbed::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICLRStrongName3 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait ICLRSyncManager_Impl: Sized {
    fn GetMonitorOwner(&self, cookie: usize) -> ::windows::core::Result<IHostTask>;
    fn CreateRWLockOwnerIterator(&self, cookie: usize, piterator: *mut usize) -> ::windows::core::Result<()>;
    fn GetRWLockOwnerNext(&self, iterator: usize) -> ::windows::core::Result<IHostTask>;
    fn DeleteRWLockOwnerIterator(&self, iterator: usize) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ICLRSyncManager {}
impl ICLRSyncManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRSyncManager_Impl, const OFFSET: isize>() -> ICLRSyncManager_Vtbl {
        unsafe extern "system" fn GetMonitorOwner<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRSyncManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: usize, ppownerhosttask: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMonitorOwner(::core::mem::transmute_copy(&cookie)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppownerhosttask, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRWLockOwnerIterator<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRSyncManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: usize, piterator: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateRWLockOwnerIterator(::core::mem::transmute_copy(&cookie), ::core::mem::transmute_copy(&piterator)).into()
        }
        unsafe extern "system" fn GetRWLockOwnerNext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRSyncManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iterator: usize, ppownerhosttask: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRWLockOwnerNext(::core::mem::transmute_copy(&iterator)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppownerhosttask, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteRWLockOwnerIterator<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRSyncManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iterator: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteRWLockOwnerIterator(::core::mem::transmute_copy(&iterator)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetMonitorOwner: GetMonitorOwner::<Identity, Impl, OFFSET>,
            CreateRWLockOwnerIterator: CreateRWLockOwnerIterator::<Identity, Impl, OFFSET>,
            GetRWLockOwnerNext: GetRWLockOwnerNext::<Identity, Impl, OFFSET>,
            DeleteRWLockOwnerIterator: DeleteRWLockOwnerIterator::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICLRSyncManager as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICLRTask_Impl: Sized {
    fn SwitchIn(&self, threadhandle: super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
    fn SwitchOut(&self) -> ::windows::core::Result<()>;
    fn GetMemStats(&self, memusage: *mut COR_GC_THREAD_STATS) -> ::windows::core::Result<()>;
    fn Reset(&self, ffull: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn ExitTask(&self) -> ::windows::core::Result<()>;
    fn Abort(&self) -> ::windows::core::Result<()>;
    fn RudeAbort(&self) -> ::windows::core::Result<()>;
    fn NeedsPriorityScheduling(&self, pbneedspriorityscheduling: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn YieldTask(&self) -> ::windows::core::Result<()>;
    fn LocksHeld(&self, plockcount: *mut usize) -> ::windows::core::Result<()>;
    fn SetTaskIdentifier(&self, asked: u64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ICLRTask {}
#[cfg(feature = "Win32_Foundation")]
impl ICLRTask_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRTask_Impl, const OFFSET: isize>() -> ICLRTask_Vtbl {
        unsafe extern "system" fn SwitchIn<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, threadhandle: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SwitchIn(::core::mem::transmute_copy(&threadhandle)).into()
        }
        unsafe extern "system" fn SwitchOut<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SwitchOut().into()
        }
        unsafe extern "system" fn GetMemStats<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, memusage: *mut COR_GC_THREAD_STATS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMemStats(::core::mem::transmute_copy(&memusage)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ffull: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset(::core::mem::transmute_copy(&ffull)).into()
        }
        unsafe extern "system" fn ExitTask<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ExitTask().into()
        }
        unsafe extern "system" fn Abort<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Abort().into()
        }
        unsafe extern "system" fn RudeAbort<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RudeAbort().into()
        }
        unsafe extern "system" fn NeedsPriorityScheduling<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbneedspriorityscheduling: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.NeedsPriorityScheduling(::core::mem::transmute_copy(&pbneedspriorityscheduling)).into()
        }
        unsafe extern "system" fn YieldTask<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.YieldTask().into()
        }
        unsafe extern "system" fn LocksHeld<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plockcount: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LocksHeld(::core::mem::transmute_copy(&plockcount)).into()
        }
        unsafe extern "system" fn SetTaskIdentifier<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, asked: u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTaskIdentifier(::core::mem::transmute_copy(&asked)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICLRTask as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICLRTask2_Impl: Sized + ICLRTask_Impl {
    fn BeginPreventAsyncAbort(&self) -> ::windows::core::Result<()>;
    fn EndPreventAsyncAbort(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ICLRTask2 {}
#[cfg(feature = "Win32_Foundation")]
impl ICLRTask2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRTask2_Impl, const OFFSET: isize>() -> ICLRTask2_Vtbl {
        unsafe extern "system" fn BeginPreventAsyncAbort<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRTask2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BeginPreventAsyncAbort().into()
        }
        unsafe extern "system" fn EndPreventAsyncAbort<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRTask2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICLRTask2 as ::windows::core::ComInterface>::IID || iid == &<ICLRTask as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait ICLRTaskManager_Impl: Sized {
    fn CreateTask(&self) -> ::windows::core::Result<ICLRTask>;
    fn GetCurrentTask(&self) -> ::windows::core::Result<ICLRTask>;
    fn SetUILocale(&self, lcid: u32) -> ::windows::core::Result<()>;
    fn SetLocale(&self, lcid: u32) -> ::windows::core::Result<()>;
    fn GetCurrentTaskType(&self, ptasktype: *mut ETaskType) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ICLRTaskManager {}
impl ICLRTaskManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRTaskManager_Impl, const OFFSET: isize>() -> ICLRTaskManager_Vtbl {
        unsafe extern "system" fn CreateTask<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRTaskManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptask: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateTask() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptask, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentTask<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRTaskManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptask: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurrentTask() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptask, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUILocale<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRTaskManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcid: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetUILocale(::core::mem::transmute_copy(&lcid)).into()
        }
        unsafe extern "system" fn SetLocale<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRTaskManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcid: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLocale(::core::mem::transmute_copy(&lcid)).into()
        }
        unsafe extern "system" fn GetCurrentTaskType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICLRTaskManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptasktype: *mut ETaskType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCurrentTaskType(::core::mem::transmute_copy(&ptasktype)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateTask: CreateTask::<Identity, Impl, OFFSET>,
            GetCurrentTask: GetCurrentTask::<Identity, Impl, OFFSET>,
            SetUILocale: SetUILocale::<Identity, Impl, OFFSET>,
            SetLocale: SetLocale::<Identity, Impl, OFFSET>,
            GetCurrentTaskType: GetCurrentTaskType::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICLRTaskManager as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait ICatalogServices_Impl: Sized {
    fn Autodone(&self) -> ::windows::core::Result<()>;
    fn NotAutodone(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ICatalogServices {}
impl ICatalogServices_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICatalogServices_Impl, const OFFSET: isize>() -> ICatalogServices_Vtbl {
        unsafe extern "system" fn Autodone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICatalogServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Autodone().into()
        }
        unsafe extern "system" fn NotAutodone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICatalogServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.NotAutodone().into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Autodone: Autodone::<Identity, Impl, OFFSET>,
            NotAutodone: NotAutodone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICatalogServices as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait ICorConfiguration_Impl: Sized {
    fn SetGCThreadControl(&self, pgcthreadcontrol: ::core::option::Option<&IGCThreadControl>) -> ::windows::core::Result<()>;
    fn SetGCHostControl(&self, pgchostcontrol: ::core::option::Option<&IGCHostControl>) -> ::windows::core::Result<()>;
    fn SetDebuggerThreadControl(&self, pdebuggerthreadcontrol: ::core::option::Option<&IDebuggerThreadControl>) -> ::windows::core::Result<()>;
    fn AddDebuggerSpecialThread(&self, dwspecialthreadid: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ICorConfiguration {}
impl ICorConfiguration_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorConfiguration_Impl, const OFFSET: isize>() -> ICorConfiguration_Vtbl {
        unsafe extern "system" fn SetGCThreadControl<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgcthreadcontrol: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetGCThreadControl(::windows::core::from_raw_borrowed(&pgcthreadcontrol)).into()
        }
        unsafe extern "system" fn SetGCHostControl<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgchostcontrol: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetGCHostControl(::windows::core::from_raw_borrowed(&pgchostcontrol)).into()
        }
        unsafe extern "system" fn SetDebuggerThreadControl<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdebuggerthreadcontrol: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDebuggerThreadControl(::windows::core::from_raw_borrowed(&pdebuggerthreadcontrol)).into()
        }
        unsafe extern "system" fn AddDebuggerSpecialThread<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwspecialthreadid: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddDebuggerSpecialThread(::core::mem::transmute_copy(&dwspecialthreadid)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetGCThreadControl: SetGCThreadControl::<Identity, Impl, OFFSET>,
            SetGCHostControl: SetGCHostControl::<Identity, Impl, OFFSET>,
            SetDebuggerThreadControl: SetDebuggerThreadControl::<Identity, Impl, OFFSET>,
            AddDebuggerSpecialThread: AddDebuggerSpecialThread::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICorConfiguration as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICorRuntimeHost_Impl: Sized {
    fn CreateLogicalThreadState(&self) -> ::windows::core::Result<()>;
    fn DeleteLogicalThreadState(&self) -> ::windows::core::Result<()>;
    fn SwitchInLogicalThreadState(&self, pfibercookie: *mut u32) -> ::windows::core::Result<()>;
    fn SwitchOutLogicalThreadState(&self, pfibercookie: *mut *mut u32) -> ::windows::core::Result<()>;
    fn LocksHeldByLogicalThread(&self, pcount: *mut u32) -> ::windows::core::Result<()>;
    fn MapFile(&self, hfile: super::super::Foundation::HANDLE, hmapaddress: *mut super::super::Foundation::HMODULE) -> ::windows::core::Result<()>;
    fn GetConfiguration(&self) -> ::windows::core::Result<ICorConfiguration>;
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
    fn CreateDomain(&self, pwzfriendlyname: &::windows::core::PCWSTR, pidentityarray: ::core::option::Option<&::windows::core::IUnknown>) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn GetDefaultDomain(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn EnumDomains(&self, henum: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn NextDomain(&self, henum: *mut ::core::ffi::c_void, pappdomain: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn CloseEnum(&self, henum: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn CreateDomainEx(&self, pwzfriendlyname: &::windows::core::PCWSTR, psetup: ::core::option::Option<&::windows::core::IUnknown>, pevidence: ::core::option::Option<&::windows::core::IUnknown>) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn CreateDomainSetup(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn CreateEvidence(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn UnloadDomain(&self, pappdomain: ::core::option::Option<&::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn CurrentDomain(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ICorRuntimeHost {}
#[cfg(feature = "Win32_Foundation")]
impl ICorRuntimeHost_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorRuntimeHost_Impl, const OFFSET: isize>() -> ICorRuntimeHost_Vtbl {
        unsafe extern "system" fn CreateLogicalThreadState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorRuntimeHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateLogicalThreadState().into()
        }
        unsafe extern "system" fn DeleteLogicalThreadState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorRuntimeHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteLogicalThreadState().into()
        }
        unsafe extern "system" fn SwitchInLogicalThreadState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorRuntimeHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfibercookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SwitchInLogicalThreadState(::core::mem::transmute_copy(&pfibercookie)).into()
        }
        unsafe extern "system" fn SwitchOutLogicalThreadState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorRuntimeHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfibercookie: *mut *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SwitchOutLogicalThreadState(::core::mem::transmute_copy(&pfibercookie)).into()
        }
        unsafe extern "system" fn LocksHeldByLogicalThread<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorRuntimeHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LocksHeldByLogicalThread(::core::mem::transmute_copy(&pcount)).into()
        }
        unsafe extern "system" fn MapFile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorRuntimeHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hfile: super::super::Foundation::HANDLE, hmapaddress: *mut super::super::Foundation::HMODULE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MapFile(::core::mem::transmute_copy(&hfile), ::core::mem::transmute_copy(&hmapaddress)).into()
        }
        unsafe extern "system" fn GetConfiguration<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorRuntimeHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconfiguration: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetConfiguration() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pconfiguration, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Start<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorRuntimeHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Start().into()
        }
        unsafe extern "system" fn Stop<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorRuntimeHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Stop().into()
        }
        unsafe extern "system" fn CreateDomain<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorRuntimeHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzfriendlyname: ::windows::core::PCWSTR, pidentityarray: *mut ::core::ffi::c_void, pappdomain: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateDomain(::core::mem::transmute(&pwzfriendlyname), ::windows::core::from_raw_borrowed(&pidentityarray)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pappdomain, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultDomain<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorRuntimeHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pappdomain: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDefaultDomain() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pappdomain, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumDomains<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorRuntimeHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, henum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumDomains(::core::mem::transmute_copy(&henum)).into()
        }
        unsafe extern "system" fn NextDomain<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorRuntimeHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, henum: *mut ::core::ffi::c_void, pappdomain: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.NextDomain(::core::mem::transmute_copy(&henum), ::core::mem::transmute_copy(&pappdomain)).into()
        }
        unsafe extern "system" fn CloseEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorRuntimeHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, henum: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CloseEnum(::core::mem::transmute_copy(&henum)).into()
        }
        unsafe extern "system" fn CreateDomainEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorRuntimeHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzfriendlyname: ::windows::core::PCWSTR, psetup: *mut ::core::ffi::c_void, pevidence: *mut ::core::ffi::c_void, pappdomain: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateDomainEx(::core::mem::transmute(&pwzfriendlyname), ::windows::core::from_raw_borrowed(&psetup), ::windows::core::from_raw_borrowed(&pevidence)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pappdomain, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDomainSetup<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorRuntimeHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pappdomainsetup: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateDomainSetup() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pappdomainsetup, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEvidence<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorRuntimeHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pevidence: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateEvidence() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pevidence, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnloadDomain<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorRuntimeHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pappdomain: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnloadDomain(::windows::core::from_raw_borrowed(&pappdomain)).into()
        }
        unsafe extern "system" fn CurrentDomain<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorRuntimeHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pappdomain: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentDomain() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pappdomain, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICorRuntimeHost as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`, `\"Win32_System_Threading\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO", feature = "Win32_System_Threading"))]
pub trait ICorThreadpool_Impl: Sized {
    fn CorRegisterWaitForSingleObject(&self, phnewwaitobject: *mut super::super::Foundation::HANDLE, hwaitobject: super::super::Foundation::HANDLE, callback: super::Threading::WAITORTIMERCALLBACK, context: *mut ::core::ffi::c_void, timeout: u32, executeonlyonce: super::super::Foundation::BOOL, result: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn CorUnregisterWait(&self, hwaitobject: super::super::Foundation::HANDLE, completionevent: super::super::Foundation::HANDLE, result: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn CorQueueUserWorkItem(&self, function: super::Threading::LPTHREAD_START_ROUTINE, context: *mut ::core::ffi::c_void, executeonlyonce: super::super::Foundation::BOOL, result: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn CorCreateTimer(&self, phnewtimer: *mut super::super::Foundation::HANDLE, callback: super::Threading::WAITORTIMERCALLBACK, parameter: *mut ::core::ffi::c_void, duetime: u32, period: u32, result: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn CorChangeTimer(&self, timer: super::super::Foundation::HANDLE, duetime: u32, period: u32, result: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn CorDeleteTimer(&self, timer: super::super::Foundation::HANDLE, completionevent: super::super::Foundation::HANDLE, result: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn CorBindIoCompletionCallback(&self, filehandle: super::super::Foundation::HANDLE, callback: super::IO::LPOVERLAPPED_COMPLETION_ROUTINE) -> ::windows::core::Result<()>;
    fn CorCallOrQueueUserWorkItem(&self, function: super::Threading::LPTHREAD_START_ROUTINE, context: *mut ::core::ffi::c_void, result: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn CorSetMaxThreads(&self, maxworkerthreads: u32, maxiocompletionthreads: u32) -> ::windows::core::Result<()>;
    fn CorGetMaxThreads(&self, maxworkerthreads: *mut u32, maxiocompletionthreads: *mut u32) -> ::windows::core::Result<()>;
    fn CorGetAvailableThreads(&self, availableworkerthreads: *mut u32, availableiocompletionthreads: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO", feature = "Win32_System_Threading"))]
impl ::windows::core::RuntimeName for ICorThreadpool {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO", feature = "Win32_System_Threading"))]
impl ICorThreadpool_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorThreadpool_Impl, const OFFSET: isize>() -> ICorThreadpool_Vtbl {
        unsafe extern "system" fn CorRegisterWaitForSingleObject<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorThreadpool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phnewwaitobject: *mut super::super::Foundation::HANDLE, hwaitobject: super::super::Foundation::HANDLE, callback: super::Threading::WAITORTIMERCALLBACK, context: *mut ::core::ffi::c_void, timeout: u32, executeonlyonce: super::super::Foundation::BOOL, result: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CorRegisterWaitForSingleObject(::core::mem::transmute_copy(&phnewwaitobject), ::core::mem::transmute_copy(&hwaitobject), ::core::mem::transmute_copy(&callback), ::core::mem::transmute_copy(&context), ::core::mem::transmute_copy(&timeout), ::core::mem::transmute_copy(&executeonlyonce), ::core::mem::transmute_copy(&result)).into()
        }
        unsafe extern "system" fn CorUnregisterWait<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorThreadpool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwaitobject: super::super::Foundation::HANDLE, completionevent: super::super::Foundation::HANDLE, result: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CorUnregisterWait(::core::mem::transmute_copy(&hwaitobject), ::core::mem::transmute_copy(&completionevent), ::core::mem::transmute_copy(&result)).into()
        }
        unsafe extern "system" fn CorQueueUserWorkItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorThreadpool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, function: super::Threading::LPTHREAD_START_ROUTINE, context: *mut ::core::ffi::c_void, executeonlyonce: super::super::Foundation::BOOL, result: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CorQueueUserWorkItem(::core::mem::transmute_copy(&function), ::core::mem::transmute_copy(&context), ::core::mem::transmute_copy(&executeonlyonce), ::core::mem::transmute_copy(&result)).into()
        }
        unsafe extern "system" fn CorCreateTimer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorThreadpool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phnewtimer: *mut super::super::Foundation::HANDLE, callback: super::Threading::WAITORTIMERCALLBACK, parameter: *mut ::core::ffi::c_void, duetime: u32, period: u32, result: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CorCreateTimer(::core::mem::transmute_copy(&phnewtimer), ::core::mem::transmute_copy(&callback), ::core::mem::transmute_copy(&parameter), ::core::mem::transmute_copy(&duetime), ::core::mem::transmute_copy(&period), ::core::mem::transmute_copy(&result)).into()
        }
        unsafe extern "system" fn CorChangeTimer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorThreadpool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timer: super::super::Foundation::HANDLE, duetime: u32, period: u32, result: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CorChangeTimer(::core::mem::transmute_copy(&timer), ::core::mem::transmute_copy(&duetime), ::core::mem::transmute_copy(&period), ::core::mem::transmute_copy(&result)).into()
        }
        unsafe extern "system" fn CorDeleteTimer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorThreadpool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timer: super::super::Foundation::HANDLE, completionevent: super::super::Foundation::HANDLE, result: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CorDeleteTimer(::core::mem::transmute_copy(&timer), ::core::mem::transmute_copy(&completionevent), ::core::mem::transmute_copy(&result)).into()
        }
        unsafe extern "system" fn CorBindIoCompletionCallback<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorThreadpool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filehandle: super::super::Foundation::HANDLE, callback: super::IO::LPOVERLAPPED_COMPLETION_ROUTINE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CorBindIoCompletionCallback(::core::mem::transmute_copy(&filehandle), ::core::mem::transmute_copy(&callback)).into()
        }
        unsafe extern "system" fn CorCallOrQueueUserWorkItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorThreadpool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, function: super::Threading::LPTHREAD_START_ROUTINE, context: *mut ::core::ffi::c_void, result: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CorCallOrQueueUserWorkItem(::core::mem::transmute_copy(&function), ::core::mem::transmute_copy(&context), ::core::mem::transmute_copy(&result)).into()
        }
        unsafe extern "system" fn CorSetMaxThreads<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorThreadpool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxworkerthreads: u32, maxiocompletionthreads: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CorSetMaxThreads(::core::mem::transmute_copy(&maxworkerthreads), ::core::mem::transmute_copy(&maxiocompletionthreads)).into()
        }
        unsafe extern "system" fn CorGetMaxThreads<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorThreadpool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxworkerthreads: *mut u32, maxiocompletionthreads: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CorGetMaxThreads(::core::mem::transmute_copy(&maxworkerthreads), ::core::mem::transmute_copy(&maxiocompletionthreads)).into()
        }
        unsafe extern "system" fn CorGetAvailableThreads<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorThreadpool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, availableworkerthreads: *mut u32, availableiocompletionthreads: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CorGetAvailableThreads(::core::mem::transmute_copy(&availableworkerthreads), ::core::mem::transmute_copy(&availableiocompletionthreads)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICorThreadpool as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDebuggerInfo_Impl: Sized {
    fn IsDebuggerAttached(&self, pbattached: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDebuggerInfo {}
#[cfg(feature = "Win32_Foundation")]
impl IDebuggerInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDebuggerInfo_Impl, const OFFSET: isize>() -> IDebuggerInfo_Vtbl {
        unsafe extern "system" fn IsDebuggerAttached<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDebuggerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbattached: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsDebuggerAttached(::core::mem::transmute_copy(&pbattached)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), IsDebuggerAttached: IsDebuggerAttached::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDebuggerInfo as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait IDebuggerThreadControl_Impl: Sized {
    fn ThreadIsBlockingForDebugger(&self) -> ::windows::core::Result<()>;
    fn ReleaseAllRuntimeThreads(&self) -> ::windows::core::Result<()>;
    fn StartBlockingForDebugger(&self, dwunused: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDebuggerThreadControl {}
impl IDebuggerThreadControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDebuggerThreadControl_Impl, const OFFSET: isize>() -> IDebuggerThreadControl_Vtbl {
        unsafe extern "system" fn ThreadIsBlockingForDebugger<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDebuggerThreadControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ThreadIsBlockingForDebugger().into()
        }
        unsafe extern "system" fn ReleaseAllRuntimeThreads<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDebuggerThreadControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleaseAllRuntimeThreads().into()
        }
        unsafe extern "system" fn StartBlockingForDebugger<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDebuggerThreadControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwunused: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StartBlockingForDebugger(::core::mem::transmute_copy(&dwunused)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ThreadIsBlockingForDebugger: ThreadIsBlockingForDebugger::<Identity, Impl, OFFSET>,
            ReleaseAllRuntimeThreads: ReleaseAllRuntimeThreads::<Identity, Impl, OFFSET>,
            StartBlockingForDebugger: StartBlockingForDebugger::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDebuggerThreadControl as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait IGCHost_Impl: Sized {
    fn SetGCStartupLimits(&self, segmentsize: u32, maxgen0size: u32) -> ::windows::core::Result<()>;
    fn Collect(&self, generation: i32) -> ::windows::core::Result<()>;
    fn GetStats(&self, pstats: *mut COR_GC_STATS) -> ::windows::core::Result<()>;
    fn GetThreadStats(&self, pfibercookie: *mut u32, pstats: *mut COR_GC_THREAD_STATS) -> ::windows::core::Result<()>;
    fn SetVirtualMemLimit(&self, sztmaxvirtualmemmb: usize) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IGCHost {}
impl IGCHost_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGCHost_Impl, const OFFSET: isize>() -> IGCHost_Vtbl {
        unsafe extern "system" fn SetGCStartupLimits<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGCHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, segmentsize: u32, maxgen0size: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetGCStartupLimits(::core::mem::transmute_copy(&segmentsize), ::core::mem::transmute_copy(&maxgen0size)).into()
        }
        unsafe extern "system" fn Collect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGCHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, generation: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Collect(::core::mem::transmute_copy(&generation)).into()
        }
        unsafe extern "system" fn GetStats<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGCHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstats: *mut COR_GC_STATS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetStats(::core::mem::transmute_copy(&pstats)).into()
        }
        unsafe extern "system" fn GetThreadStats<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGCHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfibercookie: *mut u32, pstats: *mut COR_GC_THREAD_STATS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetThreadStats(::core::mem::transmute_copy(&pfibercookie), ::core::mem::transmute_copy(&pstats)).into()
        }
        unsafe extern "system" fn SetVirtualMemLimit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGCHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sztmaxvirtualmemmb: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetVirtualMemLimit(::core::mem::transmute_copy(&sztmaxvirtualmemmb)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetGCStartupLimits: SetGCStartupLimits::<Identity, Impl, OFFSET>,
            Collect: Collect::<Identity, Impl, OFFSET>,
            GetStats: GetStats::<Identity, Impl, OFFSET>,
            GetThreadStats: GetThreadStats::<Identity, Impl, OFFSET>,
            SetVirtualMemLimit: SetVirtualMemLimit::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGCHost as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait IGCHost2_Impl: Sized + IGCHost_Impl {
    fn SetGCStartupLimitsEx(&self, segmentsize: usize, maxgen0size: usize) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IGCHost2 {}
impl IGCHost2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGCHost2_Impl, const OFFSET: isize>() -> IGCHost2_Vtbl {
        unsafe extern "system" fn SetGCStartupLimitsEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGCHost2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, segmentsize: usize, maxgen0size: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetGCStartupLimitsEx(::core::mem::transmute_copy(&segmentsize), ::core::mem::transmute_copy(&maxgen0size)).into()
        }
        Self { base__: IGCHost_Vtbl::new::<Identity, Impl, OFFSET>(), SetGCStartupLimitsEx: SetGCStartupLimitsEx::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGCHost2 as ::windows::core::ComInterface>::IID || iid == &<IGCHost as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait IGCHostControl_Impl: Sized {
    fn RequestVirtualMemLimit(&self, sztmaxvirtualmemmb: usize, psztnewmaxvirtualmemmb: *mut usize) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IGCHostControl {}
impl IGCHostControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGCHostControl_Impl, const OFFSET: isize>() -> IGCHostControl_Vtbl {
        unsafe extern "system" fn RequestVirtualMemLimit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGCHostControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sztmaxvirtualmemmb: usize, psztnewmaxvirtualmemmb: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RequestVirtualMemLimit(::core::mem::transmute_copy(&sztmaxvirtualmemmb), ::core::mem::transmute_copy(&psztnewmaxvirtualmemmb)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), RequestVirtualMemLimit: RequestVirtualMemLimit::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGCHostControl as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait IGCThreadControl_Impl: Sized {
    fn ThreadIsBlockingForSuspension(&self) -> ::windows::core::Result<()>;
    fn SuspensionStarting(&self) -> ::windows::core::Result<()>;
    fn SuspensionEnding(&self, generation: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IGCThreadControl {}
impl IGCThreadControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGCThreadControl_Impl, const OFFSET: isize>() -> IGCThreadControl_Vtbl {
        unsafe extern "system" fn ThreadIsBlockingForSuspension<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGCThreadControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ThreadIsBlockingForSuspension().into()
        }
        unsafe extern "system" fn SuspensionStarting<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGCThreadControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SuspensionStarting().into()
        }
        unsafe extern "system" fn SuspensionEnding<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGCThreadControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, generation: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SuspensionEnding(::core::mem::transmute_copy(&generation)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ThreadIsBlockingForSuspension: ThreadIsBlockingForSuspension::<Identity, Impl, OFFSET>,
            SuspensionStarting: SuspensionStarting::<Identity, Impl, OFFSET>,
            SuspensionEnding: SuspensionEnding::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGCThreadControl as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait IHostAssemblyManager_Impl: Sized {
    fn GetNonHostStoreAssemblies(&self) -> ::windows::core::Result<ICLRAssemblyReferenceList>;
    fn GetAssemblyStore(&self) -> ::windows::core::Result<IHostAssemblyStore>;
}
impl ::windows::core::RuntimeName for IHostAssemblyManager {}
impl IHostAssemblyManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostAssemblyManager_Impl, const OFFSET: isize>() -> IHostAssemblyManager_Vtbl {
        unsafe extern "system" fn GetNonHostStoreAssemblies<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostAssemblyManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppreferencelist: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetNonHostStoreAssemblies() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppreferencelist, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAssemblyStore<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostAssemblyManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppassemblystore: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAssemblyStore() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppassemblystore, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetNonHostStoreAssemblies: GetNonHostStoreAssemblies::<Identity, Impl, OFFSET>,
            GetAssemblyStore: GetAssemblyStore::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHostAssemblyManager as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IHostAssemblyStore_Impl: Sized {
    fn ProvideAssembly(&self, pbindinfo: *mut AssemblyBindInfo, passemblyid: *mut u64, pcontext: *mut u64, ppstmassemblyimage: *mut ::core::option::Option<super::Com::IStream>, ppstmpdb: *mut ::core::option::Option<super::Com::IStream>) -> ::windows::core::Result<()>;
    fn ProvideModule(&self, pbindinfo: *mut ModuleBindInfo, pdwmoduleid: *mut u32, ppstmmoduleimage: *mut ::core::option::Option<super::Com::IStream>, ppstmpdb: *mut ::core::option::Option<super::Com::IStream>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IHostAssemblyStore {}
#[cfg(feature = "Win32_System_Com")]
impl IHostAssemblyStore_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostAssemblyStore_Impl, const OFFSET: isize>() -> IHostAssemblyStore_Vtbl {
        unsafe extern "system" fn ProvideAssembly<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostAssemblyStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbindinfo: *mut AssemblyBindInfo, passemblyid: *mut u64, pcontext: *mut u64, ppstmassemblyimage: *mut *mut ::core::ffi::c_void, ppstmpdb: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ProvideAssembly(::core::mem::transmute_copy(&pbindinfo), ::core::mem::transmute_copy(&passemblyid), ::core::mem::transmute_copy(&pcontext), ::core::mem::transmute_copy(&ppstmassemblyimage), ::core::mem::transmute_copy(&ppstmpdb)).into()
        }
        unsafe extern "system" fn ProvideModule<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostAssemblyStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbindinfo: *mut ModuleBindInfo, pdwmoduleid: *mut u32, ppstmmoduleimage: *mut *mut ::core::ffi::c_void, ppstmpdb: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ProvideModule(::core::mem::transmute_copy(&pbindinfo), ::core::mem::transmute_copy(&pdwmoduleid), ::core::mem::transmute_copy(&ppstmmoduleimage), ::core::mem::transmute_copy(&ppstmpdb)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ProvideAssembly: ProvideAssembly::<Identity, Impl, OFFSET>,
            ProvideModule: ProvideModule::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHostAssemblyStore as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait IHostAutoEvent_Impl: Sized {
    fn Wait(&self, dwmilliseconds: u32, option: u32) -> ::windows::core::Result<()>;
    fn Set(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IHostAutoEvent {}
impl IHostAutoEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostAutoEvent_Impl, const OFFSET: isize>() -> IHostAutoEvent_Vtbl {
        unsafe extern "system" fn Wait<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostAutoEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmilliseconds: u32, option: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Wait(::core::mem::transmute_copy(&dwmilliseconds), ::core::mem::transmute_copy(&option)).into()
        }
        unsafe extern "system" fn Set<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostAutoEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Set().into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Wait: Wait::<Identity, Impl, OFFSET>, Set: Set::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHostAutoEvent as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait IHostControl_Impl: Sized {
    fn GetHostManager(&self, riid: *const ::windows::core::GUID, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn SetAppDomainManager(&self, dwappdomainid: u32, punkappdomainmanager: ::core::option::Option<&::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IHostControl {}
impl IHostControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostControl_Impl, const OFFSET: isize>() -> IHostControl_Vtbl {
        unsafe extern "system" fn GetHostManager<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetHostManager(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppobject)).into()
        }
        unsafe extern "system" fn SetAppDomainManager<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwappdomainid: u32, punkappdomainmanager: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAppDomainManager(::core::mem::transmute_copy(&dwappdomainid), ::windows::core::from_raw_borrowed(&punkappdomainmanager)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetHostManager: GetHostManager::<Identity, Impl, OFFSET>,
            SetAppDomainManager: SetAppDomainManager::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHostControl as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IHostCrst_Impl: Sized {
    fn Enter(&self, option: u32) -> ::windows::core::Result<()>;
    fn Leave(&self) -> ::windows::core::Result<()>;
    fn TryEnter(&self, option: u32, pbsucceeded: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetSpinCount(&self, dwspincount: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IHostCrst {}
#[cfg(feature = "Win32_Foundation")]
impl IHostCrst_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostCrst_Impl, const OFFSET: isize>() -> IHostCrst_Vtbl {
        unsafe extern "system" fn Enter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostCrst_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, option: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Enter(::core::mem::transmute_copy(&option)).into()
        }
        unsafe extern "system" fn Leave<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostCrst_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Leave().into()
        }
        unsafe extern "system" fn TryEnter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostCrst_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, option: u32, pbsucceeded: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.TryEnter(::core::mem::transmute_copy(&option), ::core::mem::transmute_copy(&pbsucceeded)).into()
        }
        unsafe extern "system" fn SetSpinCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostCrst_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwspincount: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSpinCount(::core::mem::transmute_copy(&dwspincount)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Enter: Enter::<Identity, Impl, OFFSET>,
            Leave: Leave::<Identity, Impl, OFFSET>,
            TryEnter: TryEnter::<Identity, Impl, OFFSET>,
            SetSpinCount: SetSpinCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHostCrst as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait IHostGCManager_Impl: Sized {
    fn ThreadIsBlockingForSuspension(&self) -> ::windows::core::Result<()>;
    fn SuspensionStarting(&self) -> ::windows::core::Result<()>;
    fn SuspensionEnding(&self, generation: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IHostGCManager {}
impl IHostGCManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostGCManager_Impl, const OFFSET: isize>() -> IHostGCManager_Vtbl {
        unsafe extern "system" fn ThreadIsBlockingForSuspension<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostGCManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ThreadIsBlockingForSuspension().into()
        }
        unsafe extern "system" fn SuspensionStarting<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostGCManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SuspensionStarting().into()
        }
        unsafe extern "system" fn SuspensionEnding<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostGCManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, generation: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SuspensionEnding(::core::mem::transmute_copy(&generation)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ThreadIsBlockingForSuspension: ThreadIsBlockingForSuspension::<Identity, Impl, OFFSET>,
            SuspensionStarting: SuspensionStarting::<Identity, Impl, OFFSET>,
            SuspensionEnding: SuspensionEnding::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHostGCManager as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IHostIoCompletionManager_Impl: Sized {
    fn CreateIoCompletionPort(&self, phport: *mut super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
    fn CloseIoCompletionPort(&self, hport: super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
    fn SetMaxThreads(&self, dwmaxiocompletionthreads: u32) -> ::windows::core::Result<()>;
    fn GetMaxThreads(&self, pdwmaxiocompletionthreads: *mut u32) -> ::windows::core::Result<()>;
    fn GetAvailableThreads(&self, pdwavailableiocompletionthreads: *mut u32) -> ::windows::core::Result<()>;
    fn GetHostOverlappedSize(&self, pcbsize: *mut u32) -> ::windows::core::Result<()>;
    fn SetCLRIoCompletionManager(&self, pmanager: ::core::option::Option<&ICLRIoCompletionManager>) -> ::windows::core::Result<()>;
    fn InitializeHostOverlapped(&self, pvoverlapped: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn Bind(&self, hport: super::super::Foundation::HANDLE, hhandle: super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
    fn SetMinThreads(&self, dwminiocompletionthreads: u32) -> ::windows::core::Result<()>;
    fn GetMinThreads(&self, pdwminiocompletionthreads: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IHostIoCompletionManager {}
#[cfg(feature = "Win32_Foundation")]
impl IHostIoCompletionManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostIoCompletionManager_Impl, const OFFSET: isize>() -> IHostIoCompletionManager_Vtbl {
        unsafe extern "system" fn CreateIoCompletionPort<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostIoCompletionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phport: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateIoCompletionPort(::core::mem::transmute_copy(&phport)).into()
        }
        unsafe extern "system" fn CloseIoCompletionPort<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostIoCompletionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hport: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CloseIoCompletionPort(::core::mem::transmute_copy(&hport)).into()
        }
        unsafe extern "system" fn SetMaxThreads<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostIoCompletionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmaxiocompletionthreads: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMaxThreads(::core::mem::transmute_copy(&dwmaxiocompletionthreads)).into()
        }
        unsafe extern "system" fn GetMaxThreads<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostIoCompletionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmaxiocompletionthreads: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMaxThreads(::core::mem::transmute_copy(&pdwmaxiocompletionthreads)).into()
        }
        unsafe extern "system" fn GetAvailableThreads<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostIoCompletionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwavailableiocompletionthreads: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAvailableThreads(::core::mem::transmute_copy(&pdwavailableiocompletionthreads)).into()
        }
        unsafe extern "system" fn GetHostOverlappedSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostIoCompletionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetHostOverlappedSize(::core::mem::transmute_copy(&pcbsize)).into()
        }
        unsafe extern "system" fn SetCLRIoCompletionManager<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostIoCompletionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmanager: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCLRIoCompletionManager(::windows::core::from_raw_borrowed(&pmanager)).into()
        }
        unsafe extern "system" fn InitializeHostOverlapped<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostIoCompletionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvoverlapped: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InitializeHostOverlapped(::core::mem::transmute_copy(&pvoverlapped)).into()
        }
        unsafe extern "system" fn Bind<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostIoCompletionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hport: super::super::Foundation::HANDLE, hhandle: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Bind(::core::mem::transmute_copy(&hport), ::core::mem::transmute_copy(&hhandle)).into()
        }
        unsafe extern "system" fn SetMinThreads<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostIoCompletionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwminiocompletionthreads: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMinThreads(::core::mem::transmute_copy(&dwminiocompletionthreads)).into()
        }
        unsafe extern "system" fn GetMinThreads<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostIoCompletionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwminiocompletionthreads: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMinThreads(::core::mem::transmute_copy(&pdwminiocompletionthreads)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHostIoCompletionManager as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait IHostMalloc_Impl: Sized {
    fn Alloc(&self, cbsize: usize, ecriticallevel: EMemoryCriticalLevel, ppmem: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn DebugAlloc(&self, cbsize: usize, ecriticallevel: EMemoryCriticalLevel, pszfilename: &::windows::core::PCSTR, ilineno: i32, ppmem: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn Free(&self, pmem: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IHostMalloc {}
impl IHostMalloc_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostMalloc_Impl, const OFFSET: isize>() -> IHostMalloc_Vtbl {
        unsafe extern "system" fn Alloc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostMalloc_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbsize: usize, ecriticallevel: EMemoryCriticalLevel, ppmem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Alloc(::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&ecriticallevel), ::core::mem::transmute_copy(&ppmem)).into()
        }
        unsafe extern "system" fn DebugAlloc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostMalloc_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbsize: usize, ecriticallevel: EMemoryCriticalLevel, pszfilename: ::windows::core::PCSTR, ilineno: i32, ppmem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DebugAlloc(::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&ecriticallevel), ::core::mem::transmute(&pszfilename), ::core::mem::transmute_copy(&ilineno), ::core::mem::transmute_copy(&ppmem)).into()
        }
        unsafe extern "system" fn Free<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostMalloc_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmem: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Free(::core::mem::transmute_copy(&pmem)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Alloc: Alloc::<Identity, Impl, OFFSET>,
            DebugAlloc: DebugAlloc::<Identity, Impl, OFFSET>,
            Free: Free::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHostMalloc as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait IHostManualEvent_Impl: Sized {
    fn Wait(&self, dwmilliseconds: u32, option: u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Set(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IHostManualEvent {}
impl IHostManualEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostManualEvent_Impl, const OFFSET: isize>() -> IHostManualEvent_Vtbl {
        unsafe extern "system" fn Wait<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostManualEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmilliseconds: u32, option: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Wait(::core::mem::transmute_copy(&dwmilliseconds), ::core::mem::transmute_copy(&option)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostManualEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Set<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostManualEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Set().into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Wait: Wait::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Set: Set::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHostManualEvent as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait IHostMemoryManager_Impl: Sized {
    fn CreateMalloc(&self, dwmalloctype: u32) -> ::windows::core::Result<IHostMalloc>;
    fn VirtualAlloc(&self, paddress: *mut ::core::ffi::c_void, dwsize: usize, flallocationtype: u32, flprotect: u32, ecriticallevel: EMemoryCriticalLevel, ppmem: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn VirtualFree(&self, lpaddress: *mut ::core::ffi::c_void, dwsize: usize, dwfreetype: u32) -> ::windows::core::Result<()>;
    fn VirtualQuery(&self, lpaddress: *mut ::core::ffi::c_void, lpbuffer: *mut ::core::ffi::c_void, dwlength: usize, presult: *mut usize) -> ::windows::core::Result<()>;
    fn VirtualProtect(&self, lpaddress: *mut ::core::ffi::c_void, dwsize: usize, flnewprotect: u32, pfloldprotect: *mut u32) -> ::windows::core::Result<()>;
    fn GetMemoryLoad(&self, pmemoryload: *mut u32, pavailablebytes: *mut usize) -> ::windows::core::Result<()>;
    fn RegisterMemoryNotificationCallback(&self, pcallback: ::core::option::Option<&ICLRMemoryNotificationCallback>) -> ::windows::core::Result<()>;
    fn NeedsVirtualAddressSpace(&self, startaddress: *mut ::core::ffi::c_void, size: usize) -> ::windows::core::Result<()>;
    fn AcquiredVirtualAddressSpace(&self, startaddress: *mut ::core::ffi::c_void, size: usize) -> ::windows::core::Result<()>;
    fn ReleasedVirtualAddressSpace(&self, startaddress: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IHostMemoryManager {}
impl IHostMemoryManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostMemoryManager_Impl, const OFFSET: isize>() -> IHostMemoryManager_Vtbl {
        unsafe extern "system" fn CreateMalloc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostMemoryManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmalloctype: u32, ppmalloc: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateMalloc(::core::mem::transmute_copy(&dwmalloctype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmalloc, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VirtualAlloc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostMemoryManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paddress: *mut ::core::ffi::c_void, dwsize: usize, flallocationtype: u32, flprotect: u32, ecriticallevel: EMemoryCriticalLevel, ppmem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.VirtualAlloc(::core::mem::transmute_copy(&paddress), ::core::mem::transmute_copy(&dwsize), ::core::mem::transmute_copy(&flallocationtype), ::core::mem::transmute_copy(&flprotect), ::core::mem::transmute_copy(&ecriticallevel), ::core::mem::transmute_copy(&ppmem)).into()
        }
        unsafe extern "system" fn VirtualFree<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostMemoryManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpaddress: *mut ::core::ffi::c_void, dwsize: usize, dwfreetype: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.VirtualFree(::core::mem::transmute_copy(&lpaddress), ::core::mem::transmute_copy(&dwsize), ::core::mem::transmute_copy(&dwfreetype)).into()
        }
        unsafe extern "system" fn VirtualQuery<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostMemoryManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpaddress: *mut ::core::ffi::c_void, lpbuffer: *mut ::core::ffi::c_void, dwlength: usize, presult: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.VirtualQuery(::core::mem::transmute_copy(&lpaddress), ::core::mem::transmute_copy(&lpbuffer), ::core::mem::transmute_copy(&dwlength), ::core::mem::transmute_copy(&presult)).into()
        }
        unsafe extern "system" fn VirtualProtect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostMemoryManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpaddress: *mut ::core::ffi::c_void, dwsize: usize, flnewprotect: u32, pfloldprotect: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.VirtualProtect(::core::mem::transmute_copy(&lpaddress), ::core::mem::transmute_copy(&dwsize), ::core::mem::transmute_copy(&flnewprotect), ::core::mem::transmute_copy(&pfloldprotect)).into()
        }
        unsafe extern "system" fn GetMemoryLoad<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostMemoryManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmemoryload: *mut u32, pavailablebytes: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMemoryLoad(::core::mem::transmute_copy(&pmemoryload), ::core::mem::transmute_copy(&pavailablebytes)).into()
        }
        unsafe extern "system" fn RegisterMemoryNotificationCallback<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostMemoryManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RegisterMemoryNotificationCallback(::windows::core::from_raw_borrowed(&pcallback)).into()
        }
        unsafe extern "system" fn NeedsVirtualAddressSpace<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostMemoryManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startaddress: *mut ::core::ffi::c_void, size: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.NeedsVirtualAddressSpace(::core::mem::transmute_copy(&startaddress), ::core::mem::transmute_copy(&size)).into()
        }
        unsafe extern "system" fn AcquiredVirtualAddressSpace<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostMemoryManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startaddress: *mut ::core::ffi::c_void, size: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AcquiredVirtualAddressSpace(::core::mem::transmute_copy(&startaddress), ::core::mem::transmute_copy(&size)).into()
        }
        unsafe extern "system" fn ReleasedVirtualAddressSpace<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostMemoryManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startaddress: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleasedVirtualAddressSpace(::core::mem::transmute_copy(&startaddress)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHostMemoryManager as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait IHostPolicyManager_Impl: Sized {
    fn OnDefaultAction(&self, operation: EClrOperation, action: EPolicyAction) -> ::windows::core::Result<()>;
    fn OnTimeout(&self, operation: EClrOperation, action: EPolicyAction) -> ::windows::core::Result<()>;
    fn OnFailure(&self, failure: EClrFailure, action: EPolicyAction) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IHostPolicyManager {}
impl IHostPolicyManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostPolicyManager_Impl, const OFFSET: isize>() -> IHostPolicyManager_Vtbl {
        unsafe extern "system" fn OnDefaultAction<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostPolicyManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operation: EClrOperation, action: EPolicyAction) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnDefaultAction(::core::mem::transmute_copy(&operation), ::core::mem::transmute_copy(&action)).into()
        }
        unsafe extern "system" fn OnTimeout<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostPolicyManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operation: EClrOperation, action: EPolicyAction) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnTimeout(::core::mem::transmute_copy(&operation), ::core::mem::transmute_copy(&action)).into()
        }
        unsafe extern "system" fn OnFailure<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostPolicyManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, failure: EClrFailure, action: EPolicyAction) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnFailure(::core::mem::transmute_copy(&failure), ::core::mem::transmute_copy(&action)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnDefaultAction: OnDefaultAction::<Identity, Impl, OFFSET>,
            OnTimeout: OnTimeout::<Identity, Impl, OFFSET>,
            OnFailure: OnFailure::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHostPolicyManager as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait IHostSecurityContext_Impl: Sized {
    fn Capture(&self) -> ::windows::core::Result<IHostSecurityContext>;
}
impl ::windows::core::RuntimeName for IHostSecurityContext {}
impl IHostSecurityContext_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostSecurityContext_Impl, const OFFSET: isize>() -> IHostSecurityContext_Vtbl {
        unsafe extern "system" fn Capture<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostSecurityContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppclonedcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Capture() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclonedcontext, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Capture: Capture::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHostSecurityContext as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IHostSecurityManager_Impl: Sized {
    fn ImpersonateLoggedOnUser(&self, htoken: super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
    fn RevertToSelf(&self) -> ::windows::core::Result<()>;
    fn OpenThreadToken(&self, dwdesiredaccess: u32, bopenasself: super::super::Foundation::BOOL, phthreadtoken: *mut super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
    fn SetThreadToken(&self, htoken: super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
    fn GetSecurityContext(&self, econtexttype: EContextType) -> ::windows::core::Result<IHostSecurityContext>;
    fn SetSecurityContext(&self, econtexttype: EContextType, psecuritycontext: ::core::option::Option<&IHostSecurityContext>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IHostSecurityManager {}
#[cfg(feature = "Win32_Foundation")]
impl IHostSecurityManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostSecurityManager_Impl, const OFFSET: isize>() -> IHostSecurityManager_Vtbl {
        unsafe extern "system" fn ImpersonateLoggedOnUser<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostSecurityManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, htoken: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ImpersonateLoggedOnUser(::core::mem::transmute_copy(&htoken)).into()
        }
        unsafe extern "system" fn RevertToSelf<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostSecurityManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RevertToSelf().into()
        }
        unsafe extern "system" fn OpenThreadToken<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostSecurityManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdesiredaccess: u32, bopenasself: super::super::Foundation::BOOL, phthreadtoken: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OpenThreadToken(::core::mem::transmute_copy(&dwdesiredaccess), ::core::mem::transmute_copy(&bopenasself), ::core::mem::transmute_copy(&phthreadtoken)).into()
        }
        unsafe extern "system" fn SetThreadToken<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostSecurityManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, htoken: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetThreadToken(::core::mem::transmute_copy(&htoken)).into()
        }
        unsafe extern "system" fn GetSecurityContext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostSecurityManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, econtexttype: EContextType, ppsecuritycontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSecurityContext(::core::mem::transmute_copy(&econtexttype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsecuritycontext, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurityContext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostSecurityManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, econtexttype: EContextType, psecuritycontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSecurityContext(::core::mem::transmute_copy(&econtexttype), ::windows::core::from_raw_borrowed(&psecuritycontext)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ImpersonateLoggedOnUser: ImpersonateLoggedOnUser::<Identity, Impl, OFFSET>,
            RevertToSelf: RevertToSelf::<Identity, Impl, OFFSET>,
            OpenThreadToken: OpenThreadToken::<Identity, Impl, OFFSET>,
            SetThreadToken: SetThreadToken::<Identity, Impl, OFFSET>,
            GetSecurityContext: GetSecurityContext::<Identity, Impl, OFFSET>,
            SetSecurityContext: SetSecurityContext::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHostSecurityManager as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait IHostSemaphore_Impl: Sized {
    fn Wait(&self, dwmilliseconds: u32, option: u32) -> ::windows::core::Result<()>;
    fn ReleaseSemaphore(&self, lreleasecount: i32, lppreviouscount: *mut i32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IHostSemaphore {}
impl IHostSemaphore_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostSemaphore_Impl, const OFFSET: isize>() -> IHostSemaphore_Vtbl {
        unsafe extern "system" fn Wait<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostSemaphore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmilliseconds: u32, option: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Wait(::core::mem::transmute_copy(&dwmilliseconds), ::core::mem::transmute_copy(&option)).into()
        }
        unsafe extern "system" fn ReleaseSemaphore<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostSemaphore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lreleasecount: i32, lppreviouscount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleaseSemaphore(::core::mem::transmute_copy(&lreleasecount), ::core::mem::transmute_copy(&lppreviouscount)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Wait: Wait::<Identity, Impl, OFFSET>,
            ReleaseSemaphore: ReleaseSemaphore::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHostSemaphore as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IHostSyncManager_Impl: Sized {
    fn SetCLRSyncManager(&self, pmanager: ::core::option::Option<&ICLRSyncManager>) -> ::windows::core::Result<()>;
    fn CreateCrst(&self) -> ::windows::core::Result<IHostCrst>;
    fn CreateCrstWithSpinCount(&self, dwspincount: u32) -> ::windows::core::Result<IHostCrst>;
    fn CreateAutoEvent(&self) -> ::windows::core::Result<IHostAutoEvent>;
    fn CreateManualEvent(&self, binitialstate: super::super::Foundation::BOOL) -> ::windows::core::Result<IHostManualEvent>;
    fn CreateMonitorEvent(&self, cookie: usize) -> ::windows::core::Result<IHostAutoEvent>;
    fn CreateRWLockWriterEvent(&self, cookie: usize) -> ::windows::core::Result<IHostAutoEvent>;
    fn CreateRWLockReaderEvent(&self, binitialstate: super::super::Foundation::BOOL, cookie: usize) -> ::windows::core::Result<IHostManualEvent>;
    fn CreateSemaphoreA(&self, dwinitial: u32, dwmax: u32) -> ::windows::core::Result<IHostSemaphore>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IHostSyncManager {}
#[cfg(feature = "Win32_Foundation")]
impl IHostSyncManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostSyncManager_Impl, const OFFSET: isize>() -> IHostSyncManager_Vtbl {
        unsafe extern "system" fn SetCLRSyncManager<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostSyncManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmanager: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCLRSyncManager(::windows::core::from_raw_borrowed(&pmanager)).into()
        }
        unsafe extern "system" fn CreateCrst<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostSyncManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcrst: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateCrst() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcrst, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCrstWithSpinCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostSyncManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwspincount: u32, ppcrst: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateCrstWithSpinCount(::core::mem::transmute_copy(&dwspincount)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcrst, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAutoEvent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostSyncManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppevent: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateAutoEvent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppevent, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateManualEvent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostSyncManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, binitialstate: super::super::Foundation::BOOL, ppevent: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateManualEvent(::core::mem::transmute_copy(&binitialstate)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppevent, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMonitorEvent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostSyncManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: usize, ppevent: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateMonitorEvent(::core::mem::transmute_copy(&cookie)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppevent, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRWLockWriterEvent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostSyncManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: usize, ppevent: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateRWLockWriterEvent(::core::mem::transmute_copy(&cookie)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppevent, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRWLockReaderEvent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostSyncManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, binitialstate: super::super::Foundation::BOOL, cookie: usize, ppevent: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateRWLockReaderEvent(::core::mem::transmute_copy(&binitialstate), ::core::mem::transmute_copy(&cookie)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppevent, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSemaphoreA<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostSyncManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinitial: u32, dwmax: u32, ppsemaphore: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateSemaphoreA(::core::mem::transmute_copy(&dwinitial), ::core::mem::transmute_copy(&dwmax)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsemaphore, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHostSyncManager as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait IHostTask_Impl: Sized {
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Alert(&self) -> ::windows::core::Result<()>;
    fn Join(&self, dwmilliseconds: u32, option: u32) -> ::windows::core::Result<()>;
    fn SetPriority(&self, newpriority: i32) -> ::windows::core::Result<()>;
    fn GetPriority(&self, ppriority: *mut i32) -> ::windows::core::Result<()>;
    fn SetCLRTask(&self, pclrtask: ::core::option::Option<&ICLRTask>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IHostTask {}
impl IHostTask_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostTask_Impl, const OFFSET: isize>() -> IHostTask_Vtbl {
        unsafe extern "system" fn Start<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Start().into()
        }
        unsafe extern "system" fn Alert<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Alert().into()
        }
        unsafe extern "system" fn Join<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmilliseconds: u32, option: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Join(::core::mem::transmute_copy(&dwmilliseconds), ::core::mem::transmute_copy(&option)).into()
        }
        unsafe extern "system" fn SetPriority<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newpriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPriority(::core::mem::transmute_copy(&newpriority)).into()
        }
        unsafe extern "system" fn GetPriority<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPriority(::core::mem::transmute_copy(&ppriority)).into()
        }
        unsafe extern "system" fn SetCLRTask<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclrtask: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCLRTask(::windows::core::from_raw_borrowed(&pclrtask)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Start: Start::<Identity, Impl, OFFSET>,
            Alert: Alert::<Identity, Impl, OFFSET>,
            Join: Join::<Identity, Impl, OFFSET>,
            SetPriority: SetPriority::<Identity, Impl, OFFSET>,
            GetPriority: GetPriority::<Identity, Impl, OFFSET>,
            SetCLRTask: SetCLRTask::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHostTask as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"Win32_Foundation\"`, `\"Win32_System_Threading\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
pub trait IHostTaskManager_Impl: Sized {
    fn GetCurrentTask(&self) -> ::windows::core::Result<IHostTask>;
    fn CreateTask(&self, dwstacksize: u32, pstartaddress: super::Threading::LPTHREAD_START_ROUTINE, pparameter: *mut ::core::ffi::c_void, pptask: *mut ::core::option::Option<IHostTask>) -> ::windows::core::Result<()>;
    fn Sleep(&self, dwmilliseconds: u32, option: u32) -> ::windows::core::Result<()>;
    fn SwitchToTask(&self, option: u32) -> ::windows::core::Result<()>;
    fn SetUILocale(&self, lcid: u32) -> ::windows::core::Result<()>;
    fn SetLocale(&self, lcid: u32) -> ::windows::core::Result<()>;
    fn CallNeedsHostHook(&self, target: usize, pbcallneedshosthook: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn LeaveRuntime(&self, target: usize) -> ::windows::core::Result<()>;
    fn EnterRuntime(&self) -> ::windows::core::Result<()>;
    fn ReverseLeaveRuntime(&self) -> ::windows::core::Result<()>;
    fn ReverseEnterRuntime(&self) -> ::windows::core::Result<()>;
    fn BeginDelayAbort(&self) -> ::windows::core::Result<()>;
    fn EndDelayAbort(&self) -> ::windows::core::Result<()>;
    fn BeginThreadAffinity(&self) -> ::windows::core::Result<()>;
    fn EndThreadAffinity(&self) -> ::windows::core::Result<()>;
    fn SetStackGuarantee(&self, guarantee: u32) -> ::windows::core::Result<()>;
    fn GetStackGuarantee(&self, pguarantee: *mut u32) -> ::windows::core::Result<()>;
    fn SetCLRTaskManager(&self, ppmanager: ::core::option::Option<&ICLRTaskManager>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
impl ::windows::core::RuntimeName for IHostTaskManager {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
impl IHostTaskManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostTaskManager_Impl, const OFFSET: isize>() -> IHostTaskManager_Vtbl {
        unsafe extern "system" fn GetCurrentTask<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostTaskManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptask: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurrentTask() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptask, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTask<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostTaskManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstacksize: u32, pstartaddress: super::Threading::LPTHREAD_START_ROUTINE, pparameter: *mut ::core::ffi::c_void, pptask: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateTask(::core::mem::transmute_copy(&dwstacksize), ::core::mem::transmute_copy(&pstartaddress), ::core::mem::transmute_copy(&pparameter), ::core::mem::transmute_copy(&pptask)).into()
        }
        unsafe extern "system" fn Sleep<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostTaskManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmilliseconds: u32, option: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Sleep(::core::mem::transmute_copy(&dwmilliseconds), ::core::mem::transmute_copy(&option)).into()
        }
        unsafe extern "system" fn SwitchToTask<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostTaskManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, option: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SwitchToTask(::core::mem::transmute_copy(&option)).into()
        }
        unsafe extern "system" fn SetUILocale<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostTaskManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcid: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetUILocale(::core::mem::transmute_copy(&lcid)).into()
        }
        unsafe extern "system" fn SetLocale<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostTaskManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcid: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLocale(::core::mem::transmute_copy(&lcid)).into()
        }
        unsafe extern "system" fn CallNeedsHostHook<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostTaskManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: usize, pbcallneedshosthook: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CallNeedsHostHook(::core::mem::transmute_copy(&target), ::core::mem::transmute_copy(&pbcallneedshosthook)).into()
        }
        unsafe extern "system" fn LeaveRuntime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostTaskManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LeaveRuntime(::core::mem::transmute_copy(&target)).into()
        }
        unsafe extern "system" fn EnterRuntime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostTaskManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnterRuntime().into()
        }
        unsafe extern "system" fn ReverseLeaveRuntime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostTaskManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReverseLeaveRuntime().into()
        }
        unsafe extern "system" fn ReverseEnterRuntime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostTaskManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReverseEnterRuntime().into()
        }
        unsafe extern "system" fn BeginDelayAbort<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostTaskManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BeginDelayAbort().into()
        }
        unsafe extern "system" fn EndDelayAbort<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostTaskManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndDelayAbort().into()
        }
        unsafe extern "system" fn BeginThreadAffinity<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostTaskManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BeginThreadAffinity().into()
        }
        unsafe extern "system" fn EndThreadAffinity<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostTaskManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndThreadAffinity().into()
        }
        unsafe extern "system" fn SetStackGuarantee<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostTaskManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guarantee: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStackGuarantee(::core::mem::transmute_copy(&guarantee)).into()
        }
        unsafe extern "system" fn GetStackGuarantee<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostTaskManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguarantee: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetStackGuarantee(::core::mem::transmute_copy(&pguarantee)).into()
        }
        unsafe extern "system" fn SetCLRTaskManager<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostTaskManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmanager: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCLRTaskManager(::windows::core::from_raw_borrowed(&ppmanager)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHostTaskManager as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"Win32_System_Threading\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Threading")]
pub trait IHostThreadpoolManager_Impl: Sized {
    fn QueueUserWorkItem(&self, function: super::Threading::LPTHREAD_START_ROUTINE, context: *mut ::core::ffi::c_void, flags: u32) -> ::windows::core::Result<()>;
    fn SetMaxThreads(&self, dwmaxworkerthreads: u32) -> ::windows::core::Result<()>;
    fn GetMaxThreads(&self, pdwmaxworkerthreads: *mut u32) -> ::windows::core::Result<()>;
    fn GetAvailableThreads(&self, pdwavailableworkerthreads: *mut u32) -> ::windows::core::Result<()>;
    fn SetMinThreads(&self, dwminiocompletionthreads: u32) -> ::windows::core::Result<()>;
    fn GetMinThreads(&self, pdwminiocompletionthreads: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Threading")]
impl ::windows::core::RuntimeName for IHostThreadpoolManager {}
#[cfg(feature = "Win32_System_Threading")]
impl IHostThreadpoolManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostThreadpoolManager_Impl, const OFFSET: isize>() -> IHostThreadpoolManager_Vtbl {
        unsafe extern "system" fn QueueUserWorkItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostThreadpoolManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, function: super::Threading::LPTHREAD_START_ROUTINE, context: *mut ::core::ffi::c_void, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.QueueUserWorkItem(::core::mem::transmute_copy(&function), ::core::mem::transmute_copy(&context), ::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn SetMaxThreads<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostThreadpoolManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmaxworkerthreads: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMaxThreads(::core::mem::transmute_copy(&dwmaxworkerthreads)).into()
        }
        unsafe extern "system" fn GetMaxThreads<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostThreadpoolManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmaxworkerthreads: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMaxThreads(::core::mem::transmute_copy(&pdwmaxworkerthreads)).into()
        }
        unsafe extern "system" fn GetAvailableThreads<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostThreadpoolManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwavailableworkerthreads: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAvailableThreads(::core::mem::transmute_copy(&pdwavailableworkerthreads)).into()
        }
        unsafe extern "system" fn SetMinThreads<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostThreadpoolManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwminiocompletionthreads: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMinThreads(::core::mem::transmute_copy(&dwminiocompletionthreads)).into()
        }
        unsafe extern "system" fn GetMinThreads<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHostThreadpoolManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwminiocompletionthreads: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMinThreads(::core::mem::transmute_copy(&pdwminiocompletionthreads)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueueUserWorkItem: QueueUserWorkItem::<Identity, Impl, OFFSET>,
            SetMaxThreads: SetMaxThreads::<Identity, Impl, OFFSET>,
            GetMaxThreads: GetMaxThreads::<Identity, Impl, OFFSET>,
            GetAvailableThreads: GetAvailableThreads::<Identity, Impl, OFFSET>,
            SetMinThreads: SetMinThreads::<Identity, Impl, OFFSET>,
            GetMinThreads: GetMinThreads::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHostThreadpoolManager as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait IManagedObject_Impl: Sized {
    fn GetSerializedBuffer(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn GetObjectIdentity(&self, pbstrguid: *mut ::windows::core::BSTR, appdomainid: *mut i32, pccw: *mut i32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IManagedObject {}
impl IManagedObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IManagedObject_Impl, const OFFSET: isize>() -> IManagedObject_Vtbl {
        unsafe extern "system" fn GetSerializedBuffer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IManagedObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSerializedBuffer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObjectIdentity<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IManagedObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, appdomainid: *mut i32, pccw: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetObjectIdentity(::core::mem::transmute_copy(&pbstrguid), ::core::mem::transmute_copy(&appdomainid), ::core::mem::transmute_copy(&pccw)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSerializedBuffer: GetSerializedBuffer::<Identity, Impl, OFFSET>,
            GetObjectIdentity: GetObjectIdentity::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IManagedObject as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IObjectHandle_Impl: Sized {
    fn Unwrap(&self) -> ::windows::core::Result<super::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IObjectHandle {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IObjectHandle_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IObjectHandle_Impl, const OFFSET: isize>() -> IObjectHandle_Vtbl {
        unsafe extern "system" fn Unwrap<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IObjectHandle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppv: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Unwrap() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppv, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Unwrap: Unwrap::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IObjectHandle as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait ITypeName_Impl: Sized {
    fn GetNameCount(&self) -> ::windows::core::Result<u32>;
    fn GetNames(&self, count: u32, rgbsznames: *mut ::windows::core::BSTR, pcount: *mut u32) -> ::windows::core::Result<()>;
    fn GetTypeArgumentCount(&self) -> ::windows::core::Result<u32>;
    fn GetTypeArguments(&self, count: u32, rgparguments: *mut ::core::option::Option<ITypeName>, pcount: *mut u32) -> ::windows::core::Result<()>;
    fn GetModifierLength(&self) -> ::windows::core::Result<u32>;
    fn GetModifiers(&self, count: u32, rgmodifiers: *mut u32, pcount: *mut u32) -> ::windows::core::Result<()>;
    fn GetAssemblyName(&self) -> ::windows::core::Result<::windows::core::BSTR>;
}
impl ::windows::core::RuntimeName for ITypeName {}
impl ITypeName_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeName_Impl, const OFFSET: isize>() -> ITypeName_Vtbl {
        unsafe extern "system" fn GetNameCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetNameCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNames<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: u32, rgbsznames: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetNames(::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&rgbsznames), ::core::mem::transmute_copy(&pcount)).into()
        }
        unsafe extern "system" fn GetTypeArgumentCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTypeArgumentCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTypeArguments<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: u32, rgparguments: *mut *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetTypeArguments(::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&rgparguments), ::core::mem::transmute_copy(&pcount)).into()
        }
        unsafe extern "system" fn GetModifierLength<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetModifierLength() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetModifiers<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: u32, rgmodifiers: *mut u32, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetModifiers(::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&rgmodifiers), ::core::mem::transmute_copy(&pcount)).into()
        }
        unsafe extern "system" fn GetAssemblyName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rgbszassemblynames: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAssemblyName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(rgbszassemblynames, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetNameCount: GetNameCount::<Identity, Impl, OFFSET>,
            GetNames: GetNames::<Identity, Impl, OFFSET>,
            GetTypeArgumentCount: GetTypeArgumentCount::<Identity, Impl, OFFSET>,
            GetTypeArguments: GetTypeArguments::<Identity, Impl, OFFSET>,
            GetModifierLength: GetModifierLength::<Identity, Impl, OFFSET>,
            GetModifiers: GetModifiers::<Identity, Impl, OFFSET>,
            GetAssemblyName: GetAssemblyName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITypeName as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait ITypeNameBuilder_Impl: Sized {
    fn OpenGenericArguments(&self) -> ::windows::core::Result<()>;
    fn CloseGenericArguments(&self) -> ::windows::core::Result<()>;
    fn OpenGenericArgument(&self) -> ::windows::core::Result<()>;
    fn CloseGenericArgument(&self) -> ::windows::core::Result<()>;
    fn AddName(&self, szname: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn AddPointer(&self) -> ::windows::core::Result<()>;
    fn AddByRef(&self) -> ::windows::core::Result<()>;
    fn AddSzArray(&self) -> ::windows::core::Result<()>;
    fn AddArray(&self, rank: u32) -> ::windows::core::Result<()>;
    fn AddAssemblySpec(&self, szassemblyspec: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn ToString(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn Clear(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ITypeNameBuilder {}
impl ITypeNameBuilder_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeNameBuilder_Impl, const OFFSET: isize>() -> ITypeNameBuilder_Vtbl {
        unsafe extern "system" fn OpenGenericArguments<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeNameBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OpenGenericArguments().into()
        }
        unsafe extern "system" fn CloseGenericArguments<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeNameBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CloseGenericArguments().into()
        }
        unsafe extern "system" fn OpenGenericArgument<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeNameBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OpenGenericArgument().into()
        }
        unsafe extern "system" fn CloseGenericArgument<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeNameBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CloseGenericArgument().into()
        }
        unsafe extern "system" fn AddName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeNameBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddName(::core::mem::transmute(&szname)).into()
        }
        unsafe extern "system" fn AddPointer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeNameBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddPointer().into()
        }
        unsafe extern "system" fn AddByRef<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeNameBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddByRef().into()
        }
        unsafe extern "system" fn AddSzArray<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeNameBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddSzArray().into()
        }
        unsafe extern "system" fn AddArray<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeNameBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rank: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddArray(::core::mem::transmute_copy(&rank)).into()
        }
        unsafe extern "system" fn AddAssemblySpec<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeNameBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szassemblyspec: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddAssemblySpec(::core::mem::transmute(&szassemblyspec)).into()
        }
        unsafe extern "system" fn ToString<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeNameBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszstringrepresentation: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ToString() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszstringrepresentation, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clear<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeNameBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Clear().into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITypeNameBuilder as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ClrHosting\"`, `\"implement\"`*"]
pub trait ITypeNameFactory_Impl: Sized {
    fn ParseTypeName(&self, szname: &::windows::core::PCWSTR, perror: *mut u32, pptypename: *mut ::core::option::Option<ITypeName>) -> ::windows::core::Result<()>;
    fn GetTypeNameBuilder(&self) -> ::windows::core::Result<ITypeNameBuilder>;
}
impl ::windows::core::RuntimeName for ITypeNameFactory {}
impl ITypeNameFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeNameFactory_Impl, const OFFSET: isize>() -> ITypeNameFactory_Vtbl {
        unsafe extern "system" fn ParseTypeName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeNameFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szname: ::windows::core::PCWSTR, perror: *mut u32, pptypename: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ParseTypeName(::core::mem::transmute(&szname), ::core::mem::transmute_copy(&perror), ::core::mem::transmute_copy(&pptypename)).into()
        }
        unsafe extern "system" fn GetTypeNameBuilder<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeNameFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptypebuilder: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTypeNameBuilder() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptypebuilder, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ParseTypeName: ParseTypeName::<Identity, Impl, OFFSET>,
            GetTypeNameBuilder: GetTypeNameBuilder::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITypeNameFactory as ::windows::core::ComInterface>::IID
    }
}
