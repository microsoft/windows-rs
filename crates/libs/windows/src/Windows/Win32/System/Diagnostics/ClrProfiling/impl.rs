#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`, `\"Win32_System_WinRT_Metadata\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_WinRT_Metadata")]
pub trait ICorProfilerAssemblyReferenceProvider_Impl: Sized {
    fn AddAssemblyReference(&self, passemblyrefinfo: *const COR_PRF_ASSEMBLY_REFERENCE_INFO) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl ::windows::core::RuntimeName for ICorProfilerAssemblyReferenceProvider {}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl ICorProfilerAssemblyReferenceProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerAssemblyReferenceProvider_Impl, const OFFSET: isize>() -> ICorProfilerAssemblyReferenceProvider_Vtbl {
        unsafe extern "system" fn AddAssemblyReference<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerAssemblyReferenceProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, passemblyrefinfo: *const COR_PRF_ASSEMBLY_REFERENCE_INFO) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddAssemblyReference(::core::mem::transmute_copy(&passemblyrefinfo)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AddAssemblyReference: AddAssemblyReference::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICorProfilerAssemblyReferenceProvider as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICorProfilerCallback_Impl: Sized {
    fn Initialize(&self, picorprofilerinfounk: ::core::option::Option<&::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn Shutdown(&self) -> ::windows::core::Result<()>;
    fn AppDomainCreationStarted(&self, appdomainid: usize) -> ::windows::core::Result<()>;
    fn AppDomainCreationFinished(&self, appdomainid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn AppDomainShutdownStarted(&self, appdomainid: usize) -> ::windows::core::Result<()>;
    fn AppDomainShutdownFinished(&self, appdomainid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn AssemblyLoadStarted(&self, assemblyid: usize) -> ::windows::core::Result<()>;
    fn AssemblyLoadFinished(&self, assemblyid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn AssemblyUnloadStarted(&self, assemblyid: usize) -> ::windows::core::Result<()>;
    fn AssemblyUnloadFinished(&self, assemblyid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn ModuleLoadStarted(&self, moduleid: usize) -> ::windows::core::Result<()>;
    fn ModuleLoadFinished(&self, moduleid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn ModuleUnloadStarted(&self, moduleid: usize) -> ::windows::core::Result<()>;
    fn ModuleUnloadFinished(&self, moduleid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn ModuleAttachedToAssembly(&self, moduleid: usize, assemblyid: usize) -> ::windows::core::Result<()>;
    fn ClassLoadStarted(&self, classid: usize) -> ::windows::core::Result<()>;
    fn ClassLoadFinished(&self, classid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn ClassUnloadStarted(&self, classid: usize) -> ::windows::core::Result<()>;
    fn ClassUnloadFinished(&self, classid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn FunctionUnloadStarted(&self, functionid: usize) -> ::windows::core::Result<()>;
    fn JITCompilationStarted(&self, functionid: usize, fissafetoblock: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn JITCompilationFinished(&self, functionid: usize, hrstatus: ::windows::core::HRESULT, fissafetoblock: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn JITCachedFunctionSearchStarted(&self, functionid: usize, pbusecachedfunction: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn JITCachedFunctionSearchFinished(&self, functionid: usize, result: COR_PRF_JIT_CACHE) -> ::windows::core::Result<()>;
    fn JITFunctionPitched(&self, functionid: usize) -> ::windows::core::Result<()>;
    fn JITInlining(&self, callerid: usize, calleeid: usize, pfshouldinline: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn ThreadCreated(&self, threadid: usize) -> ::windows::core::Result<()>;
    fn ThreadDestroyed(&self, threadid: usize) -> ::windows::core::Result<()>;
    fn ThreadAssignedToOSThread(&self, managedthreadid: usize, osthreadid: u32) -> ::windows::core::Result<()>;
    fn RemotingClientInvocationStarted(&self) -> ::windows::core::Result<()>;
    fn RemotingClientSendingMessage(&self, pcookie: *mut ::windows::core::GUID, fisasync: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn RemotingClientReceivingReply(&self, pcookie: *mut ::windows::core::GUID, fisasync: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn RemotingClientInvocationFinished(&self) -> ::windows::core::Result<()>;
    fn RemotingServerReceivingMessage(&self, pcookie: *mut ::windows::core::GUID, fisasync: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn RemotingServerInvocationStarted(&self) -> ::windows::core::Result<()>;
    fn RemotingServerInvocationReturned(&self) -> ::windows::core::Result<()>;
    fn RemotingServerSendingReply(&self, pcookie: *mut ::windows::core::GUID, fisasync: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn UnmanagedToManagedTransition(&self, functionid: usize, reason: COR_PRF_TRANSITION_REASON) -> ::windows::core::Result<()>;
    fn ManagedToUnmanagedTransition(&self, functionid: usize, reason: COR_PRF_TRANSITION_REASON) -> ::windows::core::Result<()>;
    fn RuntimeSuspendStarted(&self, suspendreason: COR_PRF_SUSPEND_REASON) -> ::windows::core::Result<()>;
    fn RuntimeSuspendFinished(&self) -> ::windows::core::Result<()>;
    fn RuntimeSuspendAborted(&self) -> ::windows::core::Result<()>;
    fn RuntimeResumeStarted(&self) -> ::windows::core::Result<()>;
    fn RuntimeResumeFinished(&self) -> ::windows::core::Result<()>;
    fn RuntimeThreadSuspended(&self, threadid: usize) -> ::windows::core::Result<()>;
    fn RuntimeThreadResumed(&self, threadid: usize) -> ::windows::core::Result<()>;
    fn MovedReferences(&self, cmovedobjectidranges: u32, oldobjectidrangestart: *mut usize, newobjectidrangestart: *mut usize, cobjectidrangelength: *mut u32) -> ::windows::core::Result<()>;
    fn ObjectAllocated(&self, objectid: usize, classid: usize) -> ::windows::core::Result<()>;
    fn ObjectsAllocatedByClass(&self, cclasscount: u32, classids: *mut usize, cobjects: *mut u32) -> ::windows::core::Result<()>;
    fn ObjectReferences(&self, objectid: usize, classid: usize, cobjectrefs: u32, objectrefids: *mut usize) -> ::windows::core::Result<()>;
    fn RootReferences(&self, crootrefs: u32, rootrefids: *mut usize) -> ::windows::core::Result<()>;
    fn ExceptionThrown(&self, thrownobjectid: usize) -> ::windows::core::Result<()>;
    fn ExceptionSearchFunctionEnter(&self, functionid: usize) -> ::windows::core::Result<()>;
    fn ExceptionSearchFunctionLeave(&self) -> ::windows::core::Result<()>;
    fn ExceptionSearchFilterEnter(&self, functionid: usize) -> ::windows::core::Result<()>;
    fn ExceptionSearchFilterLeave(&self) -> ::windows::core::Result<()>;
    fn ExceptionSearchCatcherFound(&self, functionid: usize) -> ::windows::core::Result<()>;
    fn ExceptionOSHandlerEnter(&self, __unused: usize) -> ::windows::core::Result<()>;
    fn ExceptionOSHandlerLeave(&self, __unused: usize) -> ::windows::core::Result<()>;
    fn ExceptionUnwindFunctionEnter(&self, functionid: usize) -> ::windows::core::Result<()>;
    fn ExceptionUnwindFunctionLeave(&self) -> ::windows::core::Result<()>;
    fn ExceptionUnwindFinallyEnter(&self, functionid: usize) -> ::windows::core::Result<()>;
    fn ExceptionUnwindFinallyLeave(&self) -> ::windows::core::Result<()>;
    fn ExceptionCatcherEnter(&self, functionid: usize, objectid: usize) -> ::windows::core::Result<()>;
    fn ExceptionCatcherLeave(&self) -> ::windows::core::Result<()>;
    fn COMClassicVTableCreated(&self, wrappedclassid: usize, implementediid: *const ::windows::core::GUID, pvtable: *mut ::core::ffi::c_void, cslots: u32) -> ::windows::core::Result<()>;
    fn COMClassicVTableDestroyed(&self, wrappedclassid: usize, implementediid: *const ::windows::core::GUID, pvtable: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn ExceptionCLRCatcherFound(&self) -> ::windows::core::Result<()>;
    fn ExceptionCLRCatcherExecute(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ICorProfilerCallback {}
#[cfg(feature = "Win32_Foundation")]
impl ICorProfilerCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: isize>() -> ICorProfilerCallback_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, picorprofilerinfounk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::windows::core::from_raw_borrowed(&picorprofilerinfounk)).into()
        }
        unsafe extern "system" fn Shutdown<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Shutdown().into()
        }
        unsafe extern "system" fn AppDomainCreationStarted<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appdomainid: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AppDomainCreationStarted(::core::mem::transmute_copy(&appdomainid)).into()
        }
        unsafe extern "system" fn AppDomainCreationFinished<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appdomainid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AppDomainCreationFinished(::core::mem::transmute_copy(&appdomainid), ::core::mem::transmute_copy(&hrstatus)).into()
        }
        unsafe extern "system" fn AppDomainShutdownStarted<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appdomainid: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AppDomainShutdownStarted(::core::mem::transmute_copy(&appdomainid)).into()
        }
        unsafe extern "system" fn AppDomainShutdownFinished<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appdomainid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AppDomainShutdownFinished(::core::mem::transmute_copy(&appdomainid), ::core::mem::transmute_copy(&hrstatus)).into()
        }
        unsafe extern "system" fn AssemblyLoadStarted<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, assemblyid: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AssemblyLoadStarted(::core::mem::transmute_copy(&assemblyid)).into()
        }
        unsafe extern "system" fn AssemblyLoadFinished<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, assemblyid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AssemblyLoadFinished(::core::mem::transmute_copy(&assemblyid), ::core::mem::transmute_copy(&hrstatus)).into()
        }
        unsafe extern "system" fn AssemblyUnloadStarted<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, assemblyid: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AssemblyUnloadStarted(::core::mem::transmute_copy(&assemblyid)).into()
        }
        unsafe extern "system" fn AssemblyUnloadFinished<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, assemblyid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AssemblyUnloadFinished(::core::mem::transmute_copy(&assemblyid), ::core::mem::transmute_copy(&hrstatus)).into()
        }
        unsafe extern "system" fn ModuleLoadStarted<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, moduleid: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ModuleLoadStarted(::core::mem::transmute_copy(&moduleid)).into()
        }
        unsafe extern "system" fn ModuleLoadFinished<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, moduleid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ModuleLoadFinished(::core::mem::transmute_copy(&moduleid), ::core::mem::transmute_copy(&hrstatus)).into()
        }
        unsafe extern "system" fn ModuleUnloadStarted<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, moduleid: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ModuleUnloadStarted(::core::mem::transmute_copy(&moduleid)).into()
        }
        unsafe extern "system" fn ModuleUnloadFinished<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, moduleid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ModuleUnloadFinished(::core::mem::transmute_copy(&moduleid), ::core::mem::transmute_copy(&hrstatus)).into()
        }
        unsafe extern "system" fn ModuleAttachedToAssembly<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, moduleid: usize, assemblyid: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ModuleAttachedToAssembly(::core::mem::transmute_copy(&moduleid), ::core::mem::transmute_copy(&assemblyid)).into()
        }
        unsafe extern "system" fn ClassLoadStarted<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, classid: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ClassLoadStarted(::core::mem::transmute_copy(&classid)).into()
        }
        unsafe extern "system" fn ClassLoadFinished<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, classid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ClassLoadFinished(::core::mem::transmute_copy(&classid), ::core::mem::transmute_copy(&hrstatus)).into()
        }
        unsafe extern "system" fn ClassUnloadStarted<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, classid: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ClassUnloadStarted(::core::mem::transmute_copy(&classid)).into()
        }
        unsafe extern "system" fn ClassUnloadFinished<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, classid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ClassUnloadFinished(::core::mem::transmute_copy(&classid), ::core::mem::transmute_copy(&hrstatus)).into()
        }
        unsafe extern "system" fn FunctionUnloadStarted<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, functionid: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FunctionUnloadStarted(::core::mem::transmute_copy(&functionid)).into()
        }
        unsafe extern "system" fn JITCompilationStarted<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, functionid: usize, fissafetoblock: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.JITCompilationStarted(::core::mem::transmute_copy(&functionid), ::core::mem::transmute_copy(&fissafetoblock)).into()
        }
        unsafe extern "system" fn JITCompilationFinished<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, functionid: usize, hrstatus: ::windows::core::HRESULT, fissafetoblock: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.JITCompilationFinished(::core::mem::transmute_copy(&functionid), ::core::mem::transmute_copy(&hrstatus), ::core::mem::transmute_copy(&fissafetoblock)).into()
        }
        unsafe extern "system" fn JITCachedFunctionSearchStarted<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, functionid: usize, pbusecachedfunction: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.JITCachedFunctionSearchStarted(::core::mem::transmute_copy(&functionid), ::core::mem::transmute_copy(&pbusecachedfunction)).into()
        }
        unsafe extern "system" fn JITCachedFunctionSearchFinished<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, functionid: usize, result: COR_PRF_JIT_CACHE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.JITCachedFunctionSearchFinished(::core::mem::transmute_copy(&functionid), ::core::mem::transmute_copy(&result)).into()
        }
        unsafe extern "system" fn JITFunctionPitched<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, functionid: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.JITFunctionPitched(::core::mem::transmute_copy(&functionid)).into()
        }
        unsafe extern "system" fn JITInlining<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callerid: usize, calleeid: usize, pfshouldinline: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.JITInlining(::core::mem::transmute_copy(&callerid), ::core::mem::transmute_copy(&calleeid), ::core::mem::transmute_copy(&pfshouldinline)).into()
        }
        unsafe extern "system" fn ThreadCreated<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, threadid: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ThreadCreated(::core::mem::transmute_copy(&threadid)).into()
        }
        unsafe extern "system" fn ThreadDestroyed<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, threadid: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ThreadDestroyed(::core::mem::transmute_copy(&threadid)).into()
        }
        unsafe extern "system" fn ThreadAssignedToOSThread<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, managedthreadid: usize, osthreadid: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ThreadAssignedToOSThread(::core::mem::transmute_copy(&managedthreadid), ::core::mem::transmute_copy(&osthreadid)).into()
        }
        unsafe extern "system" fn RemotingClientInvocationStarted<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemotingClientInvocationStarted().into()
        }
        unsafe extern "system" fn RemotingClientSendingMessage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcookie: *mut ::windows::core::GUID, fisasync: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemotingClientSendingMessage(::core::mem::transmute_copy(&pcookie), ::core::mem::transmute_copy(&fisasync)).into()
        }
        unsafe extern "system" fn RemotingClientReceivingReply<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcookie: *mut ::windows::core::GUID, fisasync: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemotingClientReceivingReply(::core::mem::transmute_copy(&pcookie), ::core::mem::transmute_copy(&fisasync)).into()
        }
        unsafe extern "system" fn RemotingClientInvocationFinished<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemotingClientInvocationFinished().into()
        }
        unsafe extern "system" fn RemotingServerReceivingMessage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcookie: *mut ::windows::core::GUID, fisasync: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemotingServerReceivingMessage(::core::mem::transmute_copy(&pcookie), ::core::mem::transmute_copy(&fisasync)).into()
        }
        unsafe extern "system" fn RemotingServerInvocationStarted<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemotingServerInvocationStarted().into()
        }
        unsafe extern "system" fn RemotingServerInvocationReturned<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemotingServerInvocationReturned().into()
        }
        unsafe extern "system" fn RemotingServerSendingReply<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcookie: *mut ::windows::core::GUID, fisasync: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemotingServerSendingReply(::core::mem::transmute_copy(&pcookie), ::core::mem::transmute_copy(&fisasync)).into()
        }
        unsafe extern "system" fn UnmanagedToManagedTransition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, functionid: usize, reason: COR_PRF_TRANSITION_REASON) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnmanagedToManagedTransition(::core::mem::transmute_copy(&functionid), ::core::mem::transmute_copy(&reason)).into()
        }
        unsafe extern "system" fn ManagedToUnmanagedTransition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, functionid: usize, reason: COR_PRF_TRANSITION_REASON) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ManagedToUnmanagedTransition(::core::mem::transmute_copy(&functionid), ::core::mem::transmute_copy(&reason)).into()
        }
        unsafe extern "system" fn RuntimeSuspendStarted<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, suspendreason: COR_PRF_SUSPEND_REASON) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RuntimeSuspendStarted(::core::mem::transmute_copy(&suspendreason)).into()
        }
        unsafe extern "system" fn RuntimeSuspendFinished<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RuntimeSuspendFinished().into()
        }
        unsafe extern "system" fn RuntimeSuspendAborted<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RuntimeSuspendAborted().into()
        }
        unsafe extern "system" fn RuntimeResumeStarted<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RuntimeResumeStarted().into()
        }
        unsafe extern "system" fn RuntimeResumeFinished<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RuntimeResumeFinished().into()
        }
        unsafe extern "system" fn RuntimeThreadSuspended<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, threadid: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RuntimeThreadSuspended(::core::mem::transmute_copy(&threadid)).into()
        }
        unsafe extern "system" fn RuntimeThreadResumed<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, threadid: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RuntimeThreadResumed(::core::mem::transmute_copy(&threadid)).into()
        }
        unsafe extern "system" fn MovedReferences<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cmovedobjectidranges: u32, oldobjectidrangestart: *mut usize, newobjectidrangestart: *mut usize, cobjectidrangelength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MovedReferences(::core::mem::transmute_copy(&cmovedobjectidranges), ::core::mem::transmute_copy(&oldobjectidrangestart), ::core::mem::transmute_copy(&newobjectidrangestart), ::core::mem::transmute_copy(&cobjectidrangelength)).into()
        }
        unsafe extern "system" fn ObjectAllocated<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objectid: usize, classid: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ObjectAllocated(::core::mem::transmute_copy(&objectid), ::core::mem::transmute_copy(&classid)).into()
        }
        unsafe extern "system" fn ObjectsAllocatedByClass<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cclasscount: u32, classids: *mut usize, cobjects: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ObjectsAllocatedByClass(::core::mem::transmute_copy(&cclasscount), ::core::mem::transmute_copy(&classids), ::core::mem::transmute_copy(&cobjects)).into()
        }
        unsafe extern "system" fn ObjectReferences<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objectid: usize, classid: usize, cobjectrefs: u32, objectrefids: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ObjectReferences(::core::mem::transmute_copy(&objectid), ::core::mem::transmute_copy(&classid), ::core::mem::transmute_copy(&cobjectrefs), ::core::mem::transmute_copy(&objectrefids)).into()
        }
        unsafe extern "system" fn RootReferences<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, crootrefs: u32, rootrefids: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RootReferences(::core::mem::transmute_copy(&crootrefs), ::core::mem::transmute_copy(&rootrefids)).into()
        }
        unsafe extern "system" fn ExceptionThrown<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, thrownobjectid: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ExceptionThrown(::core::mem::transmute_copy(&thrownobjectid)).into()
        }
        unsafe extern "system" fn ExceptionSearchFunctionEnter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, functionid: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ExceptionSearchFunctionEnter(::core::mem::transmute_copy(&functionid)).into()
        }
        unsafe extern "system" fn ExceptionSearchFunctionLeave<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ExceptionSearchFunctionLeave().into()
        }
        unsafe extern "system" fn ExceptionSearchFilterEnter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, functionid: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ExceptionSearchFilterEnter(::core::mem::transmute_copy(&functionid)).into()
        }
        unsafe extern "system" fn ExceptionSearchFilterLeave<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ExceptionSearchFilterLeave().into()
        }
        unsafe extern "system" fn ExceptionSearchCatcherFound<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, functionid: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ExceptionSearchCatcherFound(::core::mem::transmute_copy(&functionid)).into()
        }
        unsafe extern "system" fn ExceptionOSHandlerEnter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __unused: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ExceptionOSHandlerEnter(::core::mem::transmute_copy(&__unused)).into()
        }
        unsafe extern "system" fn ExceptionOSHandlerLeave<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __unused: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ExceptionOSHandlerLeave(::core::mem::transmute_copy(&__unused)).into()
        }
        unsafe extern "system" fn ExceptionUnwindFunctionEnter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, functionid: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ExceptionUnwindFunctionEnter(::core::mem::transmute_copy(&functionid)).into()
        }
        unsafe extern "system" fn ExceptionUnwindFunctionLeave<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ExceptionUnwindFunctionLeave().into()
        }
        unsafe extern "system" fn ExceptionUnwindFinallyEnter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, functionid: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ExceptionUnwindFinallyEnter(::core::mem::transmute_copy(&functionid)).into()
        }
        unsafe extern "system" fn ExceptionUnwindFinallyLeave<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ExceptionUnwindFinallyLeave().into()
        }
        unsafe extern "system" fn ExceptionCatcherEnter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, functionid: usize, objectid: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ExceptionCatcherEnter(::core::mem::transmute_copy(&functionid), ::core::mem::transmute_copy(&objectid)).into()
        }
        unsafe extern "system" fn ExceptionCatcherLeave<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ExceptionCatcherLeave().into()
        }
        unsafe extern "system" fn COMClassicVTableCreated<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wrappedclassid: usize, implementediid: *const ::windows::core::GUID, pvtable: *mut ::core::ffi::c_void, cslots: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.COMClassicVTableCreated(::core::mem::transmute_copy(&wrappedclassid), ::core::mem::transmute_copy(&implementediid), ::core::mem::transmute_copy(&pvtable), ::core::mem::transmute_copy(&cslots)).into()
        }
        unsafe extern "system" fn COMClassicVTableDestroyed<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wrappedclassid: usize, implementediid: *const ::windows::core::GUID, pvtable: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.COMClassicVTableDestroyed(::core::mem::transmute_copy(&wrappedclassid), ::core::mem::transmute_copy(&implementediid), ::core::mem::transmute_copy(&pvtable)).into()
        }
        unsafe extern "system" fn ExceptionCLRCatcherFound<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ExceptionCLRCatcherFound().into()
        }
        unsafe extern "system" fn ExceptionCLRCatcherExecute<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ExceptionCLRCatcherExecute().into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            Shutdown: Shutdown::<Identity, Impl, OFFSET>,
            AppDomainCreationStarted: AppDomainCreationStarted::<Identity, Impl, OFFSET>,
            AppDomainCreationFinished: AppDomainCreationFinished::<Identity, Impl, OFFSET>,
            AppDomainShutdownStarted: AppDomainShutdownStarted::<Identity, Impl, OFFSET>,
            AppDomainShutdownFinished: AppDomainShutdownFinished::<Identity, Impl, OFFSET>,
            AssemblyLoadStarted: AssemblyLoadStarted::<Identity, Impl, OFFSET>,
            AssemblyLoadFinished: AssemblyLoadFinished::<Identity, Impl, OFFSET>,
            AssemblyUnloadStarted: AssemblyUnloadStarted::<Identity, Impl, OFFSET>,
            AssemblyUnloadFinished: AssemblyUnloadFinished::<Identity, Impl, OFFSET>,
            ModuleLoadStarted: ModuleLoadStarted::<Identity, Impl, OFFSET>,
            ModuleLoadFinished: ModuleLoadFinished::<Identity, Impl, OFFSET>,
            ModuleUnloadStarted: ModuleUnloadStarted::<Identity, Impl, OFFSET>,
            ModuleUnloadFinished: ModuleUnloadFinished::<Identity, Impl, OFFSET>,
            ModuleAttachedToAssembly: ModuleAttachedToAssembly::<Identity, Impl, OFFSET>,
            ClassLoadStarted: ClassLoadStarted::<Identity, Impl, OFFSET>,
            ClassLoadFinished: ClassLoadFinished::<Identity, Impl, OFFSET>,
            ClassUnloadStarted: ClassUnloadStarted::<Identity, Impl, OFFSET>,
            ClassUnloadFinished: ClassUnloadFinished::<Identity, Impl, OFFSET>,
            FunctionUnloadStarted: FunctionUnloadStarted::<Identity, Impl, OFFSET>,
            JITCompilationStarted: JITCompilationStarted::<Identity, Impl, OFFSET>,
            JITCompilationFinished: JITCompilationFinished::<Identity, Impl, OFFSET>,
            JITCachedFunctionSearchStarted: JITCachedFunctionSearchStarted::<Identity, Impl, OFFSET>,
            JITCachedFunctionSearchFinished: JITCachedFunctionSearchFinished::<Identity, Impl, OFFSET>,
            JITFunctionPitched: JITFunctionPitched::<Identity, Impl, OFFSET>,
            JITInlining: JITInlining::<Identity, Impl, OFFSET>,
            ThreadCreated: ThreadCreated::<Identity, Impl, OFFSET>,
            ThreadDestroyed: ThreadDestroyed::<Identity, Impl, OFFSET>,
            ThreadAssignedToOSThread: ThreadAssignedToOSThread::<Identity, Impl, OFFSET>,
            RemotingClientInvocationStarted: RemotingClientInvocationStarted::<Identity, Impl, OFFSET>,
            RemotingClientSendingMessage: RemotingClientSendingMessage::<Identity, Impl, OFFSET>,
            RemotingClientReceivingReply: RemotingClientReceivingReply::<Identity, Impl, OFFSET>,
            RemotingClientInvocationFinished: RemotingClientInvocationFinished::<Identity, Impl, OFFSET>,
            RemotingServerReceivingMessage: RemotingServerReceivingMessage::<Identity, Impl, OFFSET>,
            RemotingServerInvocationStarted: RemotingServerInvocationStarted::<Identity, Impl, OFFSET>,
            RemotingServerInvocationReturned: RemotingServerInvocationReturned::<Identity, Impl, OFFSET>,
            RemotingServerSendingReply: RemotingServerSendingReply::<Identity, Impl, OFFSET>,
            UnmanagedToManagedTransition: UnmanagedToManagedTransition::<Identity, Impl, OFFSET>,
            ManagedToUnmanagedTransition: ManagedToUnmanagedTransition::<Identity, Impl, OFFSET>,
            RuntimeSuspendStarted: RuntimeSuspendStarted::<Identity, Impl, OFFSET>,
            RuntimeSuspendFinished: RuntimeSuspendFinished::<Identity, Impl, OFFSET>,
            RuntimeSuspendAborted: RuntimeSuspendAborted::<Identity, Impl, OFFSET>,
            RuntimeResumeStarted: RuntimeResumeStarted::<Identity, Impl, OFFSET>,
            RuntimeResumeFinished: RuntimeResumeFinished::<Identity, Impl, OFFSET>,
            RuntimeThreadSuspended: RuntimeThreadSuspended::<Identity, Impl, OFFSET>,
            RuntimeThreadResumed: RuntimeThreadResumed::<Identity, Impl, OFFSET>,
            MovedReferences: MovedReferences::<Identity, Impl, OFFSET>,
            ObjectAllocated: ObjectAllocated::<Identity, Impl, OFFSET>,
            ObjectsAllocatedByClass: ObjectsAllocatedByClass::<Identity, Impl, OFFSET>,
            ObjectReferences: ObjectReferences::<Identity, Impl, OFFSET>,
            RootReferences: RootReferences::<Identity, Impl, OFFSET>,
            ExceptionThrown: ExceptionThrown::<Identity, Impl, OFFSET>,
            ExceptionSearchFunctionEnter: ExceptionSearchFunctionEnter::<Identity, Impl, OFFSET>,
            ExceptionSearchFunctionLeave: ExceptionSearchFunctionLeave::<Identity, Impl, OFFSET>,
            ExceptionSearchFilterEnter: ExceptionSearchFilterEnter::<Identity, Impl, OFFSET>,
            ExceptionSearchFilterLeave: ExceptionSearchFilterLeave::<Identity, Impl, OFFSET>,
            ExceptionSearchCatcherFound: ExceptionSearchCatcherFound::<Identity, Impl, OFFSET>,
            ExceptionOSHandlerEnter: ExceptionOSHandlerEnter::<Identity, Impl, OFFSET>,
            ExceptionOSHandlerLeave: ExceptionOSHandlerLeave::<Identity, Impl, OFFSET>,
            ExceptionUnwindFunctionEnter: ExceptionUnwindFunctionEnter::<Identity, Impl, OFFSET>,
            ExceptionUnwindFunctionLeave: ExceptionUnwindFunctionLeave::<Identity, Impl, OFFSET>,
            ExceptionUnwindFinallyEnter: ExceptionUnwindFinallyEnter::<Identity, Impl, OFFSET>,
            ExceptionUnwindFinallyLeave: ExceptionUnwindFinallyLeave::<Identity, Impl, OFFSET>,
            ExceptionCatcherEnter: ExceptionCatcherEnter::<Identity, Impl, OFFSET>,
            ExceptionCatcherLeave: ExceptionCatcherLeave::<Identity, Impl, OFFSET>,
            COMClassicVTableCreated: COMClassicVTableCreated::<Identity, Impl, OFFSET>,
            COMClassicVTableDestroyed: COMClassicVTableDestroyed::<Identity, Impl, OFFSET>,
            ExceptionCLRCatcherFound: ExceptionCLRCatcherFound::<Identity, Impl, OFFSET>,
            ExceptionCLRCatcherExecute: ExceptionCLRCatcherExecute::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICorProfilerCallback as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICorProfilerCallback2_Impl: Sized + ICorProfilerCallback_Impl {
    fn ThreadNameChanged(&self, threadid: usize, cchname: u32, name: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn GarbageCollectionStarted(&self, cgenerations: i32, generationcollected: *mut super::super::super::Foundation::BOOL, reason: COR_PRF_GC_REASON) -> ::windows::core::Result<()>;
    fn SurvivingReferences(&self, csurvivingobjectidranges: u32, objectidrangestart: *mut usize, cobjectidrangelength: *mut u32) -> ::windows::core::Result<()>;
    fn GarbageCollectionFinished(&self) -> ::windows::core::Result<()>;
    fn FinalizeableObjectQueued(&self, finalizerflags: u32, objectid: usize) -> ::windows::core::Result<()>;
    fn RootReferences2(&self, crootrefs: u32, rootrefids: *mut usize, rootkinds: *mut COR_PRF_GC_ROOT_KIND, rootflags: *mut COR_PRF_GC_ROOT_FLAGS, rootids: *mut usize) -> ::windows::core::Result<()>;
    fn HandleCreated(&self, handleid: usize, initialobjectid: usize) -> ::windows::core::Result<()>;
    fn HandleDestroyed(&self, handleid: usize) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ICorProfilerCallback2 {}
#[cfg(feature = "Win32_Foundation")]
impl ICorProfilerCallback2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback2_Impl, const OFFSET: isize>() -> ICorProfilerCallback2_Vtbl {
        unsafe extern "system" fn ThreadNameChanged<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, threadid: usize, cchname: u32, name: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ThreadNameChanged(::core::mem::transmute_copy(&threadid), ::core::mem::transmute_copy(&cchname), ::core::mem::transmute(&name)).into()
        }
        unsafe extern "system" fn GarbageCollectionStarted<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cgenerations: i32, generationcollected: *mut super::super::super::Foundation::BOOL, reason: COR_PRF_GC_REASON) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GarbageCollectionStarted(::core::mem::transmute_copy(&cgenerations), ::core::mem::transmute_copy(&generationcollected), ::core::mem::transmute_copy(&reason)).into()
        }
        unsafe extern "system" fn SurvivingReferences<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, csurvivingobjectidranges: u32, objectidrangestart: *mut usize, cobjectidrangelength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SurvivingReferences(::core::mem::transmute_copy(&csurvivingobjectidranges), ::core::mem::transmute_copy(&objectidrangestart), ::core::mem::transmute_copy(&cobjectidrangelength)).into()
        }
        unsafe extern "system" fn GarbageCollectionFinished<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GarbageCollectionFinished().into()
        }
        unsafe extern "system" fn FinalizeableObjectQueued<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, finalizerflags: u32, objectid: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FinalizeableObjectQueued(::core::mem::transmute_copy(&finalizerflags), ::core::mem::transmute_copy(&objectid)).into()
        }
        unsafe extern "system" fn RootReferences2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, crootrefs: u32, rootrefids: *mut usize, rootkinds: *mut COR_PRF_GC_ROOT_KIND, rootflags: *mut COR_PRF_GC_ROOT_FLAGS, rootids: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RootReferences2(::core::mem::transmute_copy(&crootrefs), ::core::mem::transmute_copy(&rootrefids), ::core::mem::transmute_copy(&rootkinds), ::core::mem::transmute_copy(&rootflags), ::core::mem::transmute_copy(&rootids)).into()
        }
        unsafe extern "system" fn HandleCreated<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handleid: usize, initialobjectid: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.HandleCreated(::core::mem::transmute_copy(&handleid), ::core::mem::transmute_copy(&initialobjectid)).into()
        }
        unsafe extern "system" fn HandleDestroyed<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handleid: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.HandleDestroyed(::core::mem::transmute_copy(&handleid)).into()
        }
        Self {
            base__: ICorProfilerCallback_Vtbl::new::<Identity, Impl, OFFSET>(),
            ThreadNameChanged: ThreadNameChanged::<Identity, Impl, OFFSET>,
            GarbageCollectionStarted: GarbageCollectionStarted::<Identity, Impl, OFFSET>,
            SurvivingReferences: SurvivingReferences::<Identity, Impl, OFFSET>,
            GarbageCollectionFinished: GarbageCollectionFinished::<Identity, Impl, OFFSET>,
            FinalizeableObjectQueued: FinalizeableObjectQueued::<Identity, Impl, OFFSET>,
            RootReferences2: RootReferences2::<Identity, Impl, OFFSET>,
            HandleCreated: HandleCreated::<Identity, Impl, OFFSET>,
            HandleDestroyed: HandleDestroyed::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICorProfilerCallback2 as ::windows::core::ComInterface>::IID || iid == &<ICorProfilerCallback as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICorProfilerCallback3_Impl: Sized + ICorProfilerCallback2_Impl {
    fn InitializeForAttach(&self, pcorprofilerinfounk: ::core::option::Option<&::windows::core::IUnknown>, pvclientdata: *mut ::core::ffi::c_void, cbclientdata: u32) -> ::windows::core::Result<()>;
    fn ProfilerAttachComplete(&self) -> ::windows::core::Result<()>;
    fn ProfilerDetachSucceeded(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ICorProfilerCallback3 {}
#[cfg(feature = "Win32_Foundation")]
impl ICorProfilerCallback3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback3_Impl, const OFFSET: isize>() -> ICorProfilerCallback3_Vtbl {
        unsafe extern "system" fn InitializeForAttach<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcorprofilerinfounk: *mut ::core::ffi::c_void, pvclientdata: *mut ::core::ffi::c_void, cbclientdata: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InitializeForAttach(::windows::core::from_raw_borrowed(&pcorprofilerinfounk), ::core::mem::transmute_copy(&pvclientdata), ::core::mem::transmute_copy(&cbclientdata)).into()
        }
        unsafe extern "system" fn ProfilerAttachComplete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ProfilerAttachComplete().into()
        }
        unsafe extern "system" fn ProfilerDetachSucceeded<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ProfilerDetachSucceeded().into()
        }
        Self {
            base__: ICorProfilerCallback2_Vtbl::new::<Identity, Impl, OFFSET>(),
            InitializeForAttach: InitializeForAttach::<Identity, Impl, OFFSET>,
            ProfilerAttachComplete: ProfilerAttachComplete::<Identity, Impl, OFFSET>,
            ProfilerDetachSucceeded: ProfilerDetachSucceeded::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICorProfilerCallback3 as ::windows::core::ComInterface>::IID || iid == &<ICorProfilerCallback as ::windows::core::ComInterface>::IID || iid == &<ICorProfilerCallback2 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICorProfilerCallback4_Impl: Sized + ICorProfilerCallback3_Impl {
    fn ReJITCompilationStarted(&self, functionid: usize, rejitid: usize, fissafetoblock: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetReJITParameters(&self, moduleid: usize, methodid: u32, pfunctioncontrol: ::core::option::Option<&ICorProfilerFunctionControl>) -> ::windows::core::Result<()>;
    fn ReJITCompilationFinished(&self, functionid: usize, rejitid: usize, hrstatus: ::windows::core::HRESULT, fissafetoblock: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn ReJITError(&self, moduleid: usize, methodid: u32, functionid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn MovedReferences2(&self, cmovedobjectidranges: u32, oldobjectidrangestart: *mut usize, newobjectidrangestart: *mut usize, cobjectidrangelength: *mut usize) -> ::windows::core::Result<()>;
    fn SurvivingReferences2(&self, csurvivingobjectidranges: u32, objectidrangestart: *mut usize, cobjectidrangelength: *mut usize) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ICorProfilerCallback4 {}
#[cfg(feature = "Win32_Foundation")]
impl ICorProfilerCallback4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback4_Impl, const OFFSET: isize>() -> ICorProfilerCallback4_Vtbl {
        unsafe extern "system" fn ReJITCompilationStarted<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, functionid: usize, rejitid: usize, fissafetoblock: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReJITCompilationStarted(::core::mem::transmute_copy(&functionid), ::core::mem::transmute_copy(&rejitid), ::core::mem::transmute_copy(&fissafetoblock)).into()
        }
        unsafe extern "system" fn GetReJITParameters<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, moduleid: usize, methodid: u32, pfunctioncontrol: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetReJITParameters(::core::mem::transmute_copy(&moduleid), ::core::mem::transmute_copy(&methodid), ::windows::core::from_raw_borrowed(&pfunctioncontrol)).into()
        }
        unsafe extern "system" fn ReJITCompilationFinished<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, functionid: usize, rejitid: usize, hrstatus: ::windows::core::HRESULT, fissafetoblock: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReJITCompilationFinished(::core::mem::transmute_copy(&functionid), ::core::mem::transmute_copy(&rejitid), ::core::mem::transmute_copy(&hrstatus), ::core::mem::transmute_copy(&fissafetoblock)).into()
        }
        unsafe extern "system" fn ReJITError<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, moduleid: usize, methodid: u32, functionid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReJITError(::core::mem::transmute_copy(&moduleid), ::core::mem::transmute_copy(&methodid), ::core::mem::transmute_copy(&functionid), ::core::mem::transmute_copy(&hrstatus)).into()
        }
        unsafe extern "system" fn MovedReferences2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cmovedobjectidranges: u32, oldobjectidrangestart: *mut usize, newobjectidrangestart: *mut usize, cobjectidrangelength: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MovedReferences2(::core::mem::transmute_copy(&cmovedobjectidranges), ::core::mem::transmute_copy(&oldobjectidrangestart), ::core::mem::transmute_copy(&newobjectidrangestart), ::core::mem::transmute_copy(&cobjectidrangelength)).into()
        }
        unsafe extern "system" fn SurvivingReferences2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, csurvivingobjectidranges: u32, objectidrangestart: *mut usize, cobjectidrangelength: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SurvivingReferences2(::core::mem::transmute_copy(&csurvivingobjectidranges), ::core::mem::transmute_copy(&objectidrangestart), ::core::mem::transmute_copy(&cobjectidrangelength)).into()
        }
        Self {
            base__: ICorProfilerCallback3_Vtbl::new::<Identity, Impl, OFFSET>(),
            ReJITCompilationStarted: ReJITCompilationStarted::<Identity, Impl, OFFSET>,
            GetReJITParameters: GetReJITParameters::<Identity, Impl, OFFSET>,
            ReJITCompilationFinished: ReJITCompilationFinished::<Identity, Impl, OFFSET>,
            ReJITError: ReJITError::<Identity, Impl, OFFSET>,
            MovedReferences2: MovedReferences2::<Identity, Impl, OFFSET>,
            SurvivingReferences2: SurvivingReferences2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICorProfilerCallback4 as ::windows::core::ComInterface>::IID || iid == &<ICorProfilerCallback as ::windows::core::ComInterface>::IID || iid == &<ICorProfilerCallback2 as ::windows::core::ComInterface>::IID || iid == &<ICorProfilerCallback3 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICorProfilerCallback5_Impl: Sized + ICorProfilerCallback4_Impl {
    fn ConditionalWeakTableElementReferences(&self, crootrefs: u32, keyrefids: *mut usize, valuerefids: *mut usize, rootids: *mut usize) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ICorProfilerCallback5 {}
#[cfg(feature = "Win32_Foundation")]
impl ICorProfilerCallback5_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback5_Impl, const OFFSET: isize>() -> ICorProfilerCallback5_Vtbl {
        unsafe extern "system" fn ConditionalWeakTableElementReferences<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, crootrefs: u32, keyrefids: *mut usize, valuerefids: *mut usize, rootids: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ConditionalWeakTableElementReferences(::core::mem::transmute_copy(&crootrefs), ::core::mem::transmute_copy(&keyrefids), ::core::mem::transmute_copy(&valuerefids), ::core::mem::transmute_copy(&rootids)).into()
        }
        Self {
            base__: ICorProfilerCallback4_Vtbl::new::<Identity, Impl, OFFSET>(),
            ConditionalWeakTableElementReferences: ConditionalWeakTableElementReferences::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICorProfilerCallback5 as ::windows::core::ComInterface>::IID || iid == &<ICorProfilerCallback as ::windows::core::ComInterface>::IID || iid == &<ICorProfilerCallback2 as ::windows::core::ComInterface>::IID || iid == &<ICorProfilerCallback3 as ::windows::core::ComInterface>::IID || iid == &<ICorProfilerCallback4 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICorProfilerCallback6_Impl: Sized + ICorProfilerCallback5_Impl {
    fn GetAssemblyReferences(&self, wszassemblypath: &::windows::core::PCWSTR, pasmrefprovider: ::core::option::Option<&ICorProfilerAssemblyReferenceProvider>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ICorProfilerCallback6 {}
#[cfg(feature = "Win32_Foundation")]
impl ICorProfilerCallback6_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback6_Impl, const OFFSET: isize>() -> ICorProfilerCallback6_Vtbl {
        unsafe extern "system" fn GetAssemblyReferences<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszassemblypath: ::windows::core::PCWSTR, pasmrefprovider: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAssemblyReferences(::core::mem::transmute(&wszassemblypath), ::windows::core::from_raw_borrowed(&pasmrefprovider)).into()
        }
        Self { base__: ICorProfilerCallback5_Vtbl::new::<Identity, Impl, OFFSET>(), GetAssemblyReferences: GetAssemblyReferences::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICorProfilerCallback6 as ::windows::core::ComInterface>::IID || iid == &<ICorProfilerCallback as ::windows::core::ComInterface>::IID || iid == &<ICorProfilerCallback2 as ::windows::core::ComInterface>::IID || iid == &<ICorProfilerCallback3 as ::windows::core::ComInterface>::IID || iid == &<ICorProfilerCallback4 as ::windows::core::ComInterface>::IID || iid == &<ICorProfilerCallback5 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICorProfilerCallback7_Impl: Sized + ICorProfilerCallback6_Impl {
    fn ModuleInMemorySymbolsUpdated(&self, moduleid: usize) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ICorProfilerCallback7 {}
#[cfg(feature = "Win32_Foundation")]
impl ICorProfilerCallback7_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback7_Impl, const OFFSET: isize>() -> ICorProfilerCallback7_Vtbl {
        unsafe extern "system" fn ModuleInMemorySymbolsUpdated<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, moduleid: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ModuleInMemorySymbolsUpdated(::core::mem::transmute_copy(&moduleid)).into()
        }
        Self {
            base__: ICorProfilerCallback6_Vtbl::new::<Identity, Impl, OFFSET>(),
            ModuleInMemorySymbolsUpdated: ModuleInMemorySymbolsUpdated::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICorProfilerCallback7 as ::windows::core::ComInterface>::IID || iid == &<ICorProfilerCallback as ::windows::core::ComInterface>::IID || iid == &<ICorProfilerCallback2 as ::windows::core::ComInterface>::IID || iid == &<ICorProfilerCallback3 as ::windows::core::ComInterface>::IID || iid == &<ICorProfilerCallback4 as ::windows::core::ComInterface>::IID || iid == &<ICorProfilerCallback5 as ::windows::core::ComInterface>::IID || iid == &<ICorProfilerCallback6 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICorProfilerCallback8_Impl: Sized + ICorProfilerCallback7_Impl {
    fn DynamicMethodJITCompilationStarted(&self, functionid: usize, fissafetoblock: super::super::super::Foundation::BOOL, pilheader: *mut u8, cbilheader: u32) -> ::windows::core::Result<()>;
    fn DynamicMethodJITCompilationFinished(&self, functionid: usize, hrstatus: ::windows::core::HRESULT, fissafetoblock: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ICorProfilerCallback8 {}
#[cfg(feature = "Win32_Foundation")]
impl ICorProfilerCallback8_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback8_Impl, const OFFSET: isize>() -> ICorProfilerCallback8_Vtbl {
        unsafe extern "system" fn DynamicMethodJITCompilationStarted<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, functionid: usize, fissafetoblock: super::super::super::Foundation::BOOL, pilheader: *mut u8, cbilheader: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DynamicMethodJITCompilationStarted(::core::mem::transmute_copy(&functionid), ::core::mem::transmute_copy(&fissafetoblock), ::core::mem::transmute_copy(&pilheader), ::core::mem::transmute_copy(&cbilheader)).into()
        }
        unsafe extern "system" fn DynamicMethodJITCompilationFinished<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, functionid: usize, hrstatus: ::windows::core::HRESULT, fissafetoblock: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DynamicMethodJITCompilationFinished(::core::mem::transmute_copy(&functionid), ::core::mem::transmute_copy(&hrstatus), ::core::mem::transmute_copy(&fissafetoblock)).into()
        }
        Self {
            base__: ICorProfilerCallback7_Vtbl::new::<Identity, Impl, OFFSET>(),
            DynamicMethodJITCompilationStarted: DynamicMethodJITCompilationStarted::<Identity, Impl, OFFSET>,
            DynamicMethodJITCompilationFinished: DynamicMethodJITCompilationFinished::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICorProfilerCallback8 as ::windows::core::ComInterface>::IID || iid == &<ICorProfilerCallback as ::windows::core::ComInterface>::IID || iid == &<ICorProfilerCallback2 as ::windows::core::ComInterface>::IID || iid == &<ICorProfilerCallback3 as ::windows::core::ComInterface>::IID || iid == &<ICorProfilerCallback4 as ::windows::core::ComInterface>::IID || iid == &<ICorProfilerCallback5 as ::windows::core::ComInterface>::IID || iid == &<ICorProfilerCallback6 as ::windows::core::ComInterface>::IID || iid == &<ICorProfilerCallback7 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICorProfilerCallback9_Impl: Sized + ICorProfilerCallback8_Impl {
    fn DynamicMethodUnloaded(&self, functionid: usize) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ICorProfilerCallback9 {}
#[cfg(feature = "Win32_Foundation")]
impl ICorProfilerCallback9_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback9_Impl, const OFFSET: isize>() -> ICorProfilerCallback9_Vtbl {
        unsafe extern "system" fn DynamicMethodUnloaded<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerCallback9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, functionid: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DynamicMethodUnloaded(::core::mem::transmute_copy(&functionid)).into()
        }
        Self { base__: ICorProfilerCallback8_Vtbl::new::<Identity, Impl, OFFSET>(), DynamicMethodUnloaded: DynamicMethodUnloaded::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICorProfilerCallback9 as ::windows::core::ComInterface>::IID || iid == &<ICorProfilerCallback as ::windows::core::ComInterface>::IID || iid == &<ICorProfilerCallback2 as ::windows::core::ComInterface>::IID || iid == &<ICorProfilerCallback3 as ::windows::core::ComInterface>::IID || iid == &<ICorProfilerCallback4 as ::windows::core::ComInterface>::IID || iid == &<ICorProfilerCallback5 as ::windows::core::ComInterface>::IID || iid == &<ICorProfilerCallback6 as ::windows::core::ComInterface>::IID || iid == &<ICorProfilerCallback7 as ::windows::core::ComInterface>::IID || iid == &<ICorProfilerCallback8 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICorProfilerFunctionControl_Impl: Sized {
    fn SetCodegenFlags(&self, flags: u32) -> ::windows::core::Result<()>;
    fn SetILFunctionBody(&self, cbnewilmethodheader: u32, pbnewilmethodheader: *mut u8) -> ::windows::core::Result<()>;
    fn SetILInstrumentedCodeMap(&self, cilmapentries: u32, rgilmapentries: *mut COR_IL_MAP) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ICorProfilerFunctionControl {}
#[cfg(feature = "Win32_Foundation")]
impl ICorProfilerFunctionControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerFunctionControl_Impl, const OFFSET: isize>() -> ICorProfilerFunctionControl_Vtbl {
        unsafe extern "system" fn SetCodegenFlags<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerFunctionControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCodegenFlags(::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn SetILFunctionBody<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerFunctionControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbnewilmethodheader: u32, pbnewilmethodheader: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetILFunctionBody(::core::mem::transmute_copy(&cbnewilmethodheader), ::core::mem::transmute_copy(&pbnewilmethodheader)).into()
        }
        unsafe extern "system" fn SetILInstrumentedCodeMap<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerFunctionControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cilmapentries: u32, rgilmapentries: *mut COR_IL_MAP) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetILInstrumentedCodeMap(::core::mem::transmute_copy(&cilmapentries), ::core::mem::transmute_copy(&rgilmapentries)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetCodegenFlags: SetCodegenFlags::<Identity, Impl, OFFSET>,
            SetILFunctionBody: SetILFunctionBody::<Identity, Impl, OFFSET>,
            SetILInstrumentedCodeMap: SetILInstrumentedCodeMap::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICorProfilerFunctionControl as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`, `\"implement\"`*"]
pub trait ICorProfilerFunctionEnum_Impl: Sized {
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<ICorProfilerFunctionEnum>;
    fn GetCount(&self, pcelt: *mut u32) -> ::windows::core::Result<()>;
    fn Next(&self, celt: u32, ids: *mut COR_PRF_FUNCTION, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ICorProfilerFunctionEnum {}
impl ICorProfilerFunctionEnum_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerFunctionEnum_Impl, const OFFSET: isize>() -> ICorProfilerFunctionEnum_Vtbl {
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerFunctionEnum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerFunctionEnum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerFunctionEnum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerFunctionEnum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcelt: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCount(::core::mem::transmute_copy(&pcelt)).into()
        }
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerFunctionEnum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ids: *mut COR_PRF_FUNCTION, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ids), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICorProfilerFunctionEnum as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`, `\"Win32_Foundation\"`, `\"Win32_System_WinRT_Metadata\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WinRT_Metadata"))]
pub trait ICorProfilerInfo_Impl: Sized {
    fn GetClassFromObject(&self, objectid: usize, pclassid: *mut usize) -> ::windows::core::Result<()>;
    fn GetClassFromToken(&self, moduleid: usize, typedef: u32, pclassid: *mut usize) -> ::windows::core::Result<()>;
    fn GetCodeInfo(&self, functionid: usize, pstart: *mut *mut u8, pcsize: *mut u32) -> ::windows::core::Result<()>;
    fn GetEventMask(&self, pdwevents: *mut u32) -> ::windows::core::Result<()>;
    fn GetFunctionFromIP(&self, ip: *mut u8, pfunctionid: *mut usize) -> ::windows::core::Result<()>;
    fn GetFunctionFromToken(&self, moduleid: usize, token: u32, pfunctionid: *mut usize) -> ::windows::core::Result<()>;
    fn GetHandleFromThread(&self, threadid: usize, phthread: *mut super::super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
    fn GetObjectSize(&self, objectid: usize, pcsize: *mut u32) -> ::windows::core::Result<()>;
    fn IsArrayClass(&self, classid: usize, pbaseelemtype: *mut super::super::WinRT::Metadata::CorElementType, pbaseclassid: *mut usize, pcrank: *mut u32) -> ::windows::core::Result<()>;
    fn GetThreadInfo(&self, threadid: usize, pdwwin32threadid: *mut u32) -> ::windows::core::Result<()>;
    fn GetCurrentThreadID(&self, pthreadid: *mut usize) -> ::windows::core::Result<()>;
    fn GetClassIDInfo(&self, classid: usize, pmoduleid: *mut usize, ptypedeftoken: *mut u32) -> ::windows::core::Result<()>;
    fn GetFunctionInfo(&self, functionid: usize, pclassid: *mut usize, pmoduleid: *mut usize, ptoken: *mut u32) -> ::windows::core::Result<()>;
    fn SetEventMask(&self, dwevents: u32) -> ::windows::core::Result<()>;
    fn SetEnterLeaveFunctionHooks(&self, pfuncenter: *mut FunctionEnter, pfuncleave: *mut FunctionLeave, pfunctailcall: *mut FunctionTailcall) -> ::windows::core::Result<()>;
    fn SetFunctionIDMapper(&self, pfunc: *mut FunctionIDMapper) -> ::windows::core::Result<()>;
    fn GetTokenAndMetaDataFromFunction(&self, functionid: usize, riid: *const ::windows::core::GUID, ppimport: *mut ::core::option::Option<::windows::core::IUnknown>, ptoken: *mut u32) -> ::windows::core::Result<()>;
    fn GetModuleInfo(&self, moduleid: usize, ppbaseloadaddress: *mut *mut u8, cchname: u32, pcchname: *mut u32, szname: ::windows::core::PWSTR, passemblyid: *mut usize) -> ::windows::core::Result<()>;
    fn GetModuleMetaData(&self, moduleid: usize, dwopenflags: u32, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn GetILFunctionBody(&self, moduleid: usize, methodid: u32, ppmethodheader: *mut *mut u8, pcbmethodsize: *mut u32) -> ::windows::core::Result<()>;
    fn GetILFunctionBodyAllocator(&self, moduleid: usize) -> ::windows::core::Result<IMethodMalloc>;
    fn SetILFunctionBody(&self, moduleid: usize, methodid: u32, pbnewilmethodheader: *mut u8) -> ::windows::core::Result<()>;
    fn GetAppDomainInfo(&self, appdomainid: usize, cchname: u32, pcchname: *mut u32, szname: ::windows::core::PWSTR, pprocessid: *mut usize) -> ::windows::core::Result<()>;
    fn GetAssemblyInfo(&self, assemblyid: usize, cchname: u32, pcchname: *mut u32, szname: ::windows::core::PWSTR, pappdomainid: *mut usize, pmoduleid: *mut usize) -> ::windows::core::Result<()>;
    fn SetFunctionReJIT(&self, functionid: usize) -> ::windows::core::Result<()>;
    fn ForceGC(&self) -> ::windows::core::Result<()>;
    fn SetILInstrumentedCodeMap(&self, functionid: usize, fstartjit: super::super::super::Foundation::BOOL, cilmapentries: u32, rgilmapentries: *mut COR_IL_MAP) -> ::windows::core::Result<()>;
    fn GetInprocInspectionInterface(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn GetInprocInspectionIThisThread(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn GetThreadContext(&self, threadid: usize, pcontextid: *mut usize) -> ::windows::core::Result<()>;
    fn BeginInprocDebugging(&self, fthisthreadonly: super::super::super::Foundation::BOOL, pdwprofilercontext: *mut u32) -> ::windows::core::Result<()>;
    fn EndInprocDebugging(&self, dwprofilercontext: u32) -> ::windows::core::Result<()>;
    fn GetILToNativeMapping(&self, functionid: usize, cmap: u32, pcmap: *mut u32, map: *mut COR_DEBUG_IL_TO_NATIVE_MAP) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WinRT_Metadata"))]
impl ::windows::core::RuntimeName for ICorProfilerInfo {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WinRT_Metadata"))]
impl ICorProfilerInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo_Impl, const OFFSET: isize>() -> ICorProfilerInfo_Vtbl {
        unsafe extern "system" fn GetClassFromObject<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objectid: usize, pclassid: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetClassFromObject(::core::mem::transmute_copy(&objectid), ::core::mem::transmute_copy(&pclassid)).into()
        }
        unsafe extern "system" fn GetClassFromToken<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, moduleid: usize, typedef: u32, pclassid: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetClassFromToken(::core::mem::transmute_copy(&moduleid), ::core::mem::transmute_copy(&typedef), ::core::mem::transmute_copy(&pclassid)).into()
        }
        unsafe extern "system" fn GetCodeInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, functionid: usize, pstart: *mut *mut u8, pcsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCodeInfo(::core::mem::transmute_copy(&functionid), ::core::mem::transmute_copy(&pstart), ::core::mem::transmute_copy(&pcsize)).into()
        }
        unsafe extern "system" fn GetEventMask<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwevents: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetEventMask(::core::mem::transmute_copy(&pdwevents)).into()
        }
        unsafe extern "system" fn GetFunctionFromIP<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ip: *mut u8, pfunctionid: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFunctionFromIP(::core::mem::transmute_copy(&ip), ::core::mem::transmute_copy(&pfunctionid)).into()
        }
        unsafe extern "system" fn GetFunctionFromToken<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, moduleid: usize, token: u32, pfunctionid: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFunctionFromToken(::core::mem::transmute_copy(&moduleid), ::core::mem::transmute_copy(&token), ::core::mem::transmute_copy(&pfunctionid)).into()
        }
        unsafe extern "system" fn GetHandleFromThread<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, threadid: usize, phthread: *mut super::super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetHandleFromThread(::core::mem::transmute_copy(&threadid), ::core::mem::transmute_copy(&phthread)).into()
        }
        unsafe extern "system" fn GetObjectSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objectid: usize, pcsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetObjectSize(::core::mem::transmute_copy(&objectid), ::core::mem::transmute_copy(&pcsize)).into()
        }
        unsafe extern "system" fn IsArrayClass<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, classid: usize, pbaseelemtype: *mut super::super::WinRT::Metadata::CorElementType, pbaseclassid: *mut usize, pcrank: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsArrayClass(::core::mem::transmute_copy(&classid), ::core::mem::transmute_copy(&pbaseelemtype), ::core::mem::transmute_copy(&pbaseclassid), ::core::mem::transmute_copy(&pcrank)).into()
        }
        unsafe extern "system" fn GetThreadInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, threadid: usize, pdwwin32threadid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetThreadInfo(::core::mem::transmute_copy(&threadid), ::core::mem::transmute_copy(&pdwwin32threadid)).into()
        }
        unsafe extern "system" fn GetCurrentThreadID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pthreadid: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCurrentThreadID(::core::mem::transmute_copy(&pthreadid)).into()
        }
        unsafe extern "system" fn GetClassIDInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, classid: usize, pmoduleid: *mut usize, ptypedeftoken: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetClassIDInfo(::core::mem::transmute_copy(&classid), ::core::mem::transmute_copy(&pmoduleid), ::core::mem::transmute_copy(&ptypedeftoken)).into()
        }
        unsafe extern "system" fn GetFunctionInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, functionid: usize, pclassid: *mut usize, pmoduleid: *mut usize, ptoken: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFunctionInfo(::core::mem::transmute_copy(&functionid), ::core::mem::transmute_copy(&pclassid), ::core::mem::transmute_copy(&pmoduleid), ::core::mem::transmute_copy(&ptoken)).into()
        }
        unsafe extern "system" fn SetEventMask<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwevents: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEventMask(::core::mem::transmute_copy(&dwevents)).into()
        }
        unsafe extern "system" fn SetEnterLeaveFunctionHooks<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfuncenter: *mut FunctionEnter, pfuncleave: *mut FunctionLeave, pfunctailcall: *mut FunctionTailcall) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEnterLeaveFunctionHooks(::core::mem::transmute_copy(&pfuncenter), ::core::mem::transmute_copy(&pfuncleave), ::core::mem::transmute_copy(&pfunctailcall)).into()
        }
        unsafe extern "system" fn SetFunctionIDMapper<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfunc: *mut FunctionIDMapper) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFunctionIDMapper(::core::mem::transmute_copy(&pfunc)).into()
        }
        unsafe extern "system" fn GetTokenAndMetaDataFromFunction<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, functionid: usize, riid: *const ::windows::core::GUID, ppimport: *mut *mut ::core::ffi::c_void, ptoken: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetTokenAndMetaDataFromFunction(::core::mem::transmute_copy(&functionid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppimport), ::core::mem::transmute_copy(&ptoken)).into()
        }
        unsafe extern "system" fn GetModuleInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, moduleid: usize, ppbaseloadaddress: *mut *mut u8, cchname: u32, pcchname: *mut u32, szname: ::windows::core::PWSTR, passemblyid: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetModuleInfo(::core::mem::transmute_copy(&moduleid), ::core::mem::transmute_copy(&ppbaseloadaddress), ::core::mem::transmute_copy(&cchname), ::core::mem::transmute_copy(&pcchname), ::core::mem::transmute_copy(&szname), ::core::mem::transmute_copy(&passemblyid)).into()
        }
        unsafe extern "system" fn GetModuleMetaData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, moduleid: usize, dwopenflags: u32, riid: *const ::windows::core::GUID, ppout: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetModuleMetaData(::core::mem::transmute_copy(&moduleid), ::core::mem::transmute_copy(&dwopenflags), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppout, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetILFunctionBody<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, moduleid: usize, methodid: u32, ppmethodheader: *mut *mut u8, pcbmethodsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetILFunctionBody(::core::mem::transmute_copy(&moduleid), ::core::mem::transmute_copy(&methodid), ::core::mem::transmute_copy(&ppmethodheader), ::core::mem::transmute_copy(&pcbmethodsize)).into()
        }
        unsafe extern "system" fn GetILFunctionBodyAllocator<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, moduleid: usize, ppmalloc: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetILFunctionBodyAllocator(::core::mem::transmute_copy(&moduleid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmalloc, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetILFunctionBody<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, moduleid: usize, methodid: u32, pbnewilmethodheader: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetILFunctionBody(::core::mem::transmute_copy(&moduleid), ::core::mem::transmute_copy(&methodid), ::core::mem::transmute_copy(&pbnewilmethodheader)).into()
        }
        unsafe extern "system" fn GetAppDomainInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appdomainid: usize, cchname: u32, pcchname: *mut u32, szname: ::windows::core::PWSTR, pprocessid: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAppDomainInfo(::core::mem::transmute_copy(&appdomainid), ::core::mem::transmute_copy(&cchname), ::core::mem::transmute_copy(&pcchname), ::core::mem::transmute_copy(&szname), ::core::mem::transmute_copy(&pprocessid)).into()
        }
        unsafe extern "system" fn GetAssemblyInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, assemblyid: usize, cchname: u32, pcchname: *mut u32, szname: ::windows::core::PWSTR, pappdomainid: *mut usize, pmoduleid: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAssemblyInfo(::core::mem::transmute_copy(&assemblyid), ::core::mem::transmute_copy(&cchname), ::core::mem::transmute_copy(&pcchname), ::core::mem::transmute_copy(&szname), ::core::mem::transmute_copy(&pappdomainid), ::core::mem::transmute_copy(&pmoduleid)).into()
        }
        unsafe extern "system" fn SetFunctionReJIT<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, functionid: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFunctionReJIT(::core::mem::transmute_copy(&functionid)).into()
        }
        unsafe extern "system" fn ForceGC<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ForceGC().into()
        }
        unsafe extern "system" fn SetILInstrumentedCodeMap<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, functionid: usize, fstartjit: super::super::super::Foundation::BOOL, cilmapentries: u32, rgilmapentries: *mut COR_IL_MAP) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetILInstrumentedCodeMap(::core::mem::transmute_copy(&functionid), ::core::mem::transmute_copy(&fstartjit), ::core::mem::transmute_copy(&cilmapentries), ::core::mem::transmute_copy(&rgilmapentries)).into()
        }
        unsafe extern "system" fn GetInprocInspectionInterface<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppicd: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetInprocInspectionInterface() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppicd, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInprocInspectionIThisThread<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppicd: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetInprocInspectionIThisThread() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppicd, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetThreadContext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, threadid: usize, pcontextid: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetThreadContext(::core::mem::transmute_copy(&threadid), ::core::mem::transmute_copy(&pcontextid)).into()
        }
        unsafe extern "system" fn BeginInprocDebugging<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fthisthreadonly: super::super::super::Foundation::BOOL, pdwprofilercontext: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BeginInprocDebugging(::core::mem::transmute_copy(&fthisthreadonly), ::core::mem::transmute_copy(&pdwprofilercontext)).into()
        }
        unsafe extern "system" fn EndInprocDebugging<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwprofilercontext: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndInprocDebugging(::core::mem::transmute_copy(&dwprofilercontext)).into()
        }
        unsafe extern "system" fn GetILToNativeMapping<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, functionid: usize, cmap: u32, pcmap: *mut u32, map: *mut COR_DEBUG_IL_TO_NATIVE_MAP) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetILToNativeMapping(::core::mem::transmute_copy(&functionid), ::core::mem::transmute_copy(&cmap), ::core::mem::transmute_copy(&pcmap), ::core::mem::transmute_copy(&map)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetClassFromObject: GetClassFromObject::<Identity, Impl, OFFSET>,
            GetClassFromToken: GetClassFromToken::<Identity, Impl, OFFSET>,
            GetCodeInfo: GetCodeInfo::<Identity, Impl, OFFSET>,
            GetEventMask: GetEventMask::<Identity, Impl, OFFSET>,
            GetFunctionFromIP: GetFunctionFromIP::<Identity, Impl, OFFSET>,
            GetFunctionFromToken: GetFunctionFromToken::<Identity, Impl, OFFSET>,
            GetHandleFromThread: GetHandleFromThread::<Identity, Impl, OFFSET>,
            GetObjectSize: GetObjectSize::<Identity, Impl, OFFSET>,
            IsArrayClass: IsArrayClass::<Identity, Impl, OFFSET>,
            GetThreadInfo: GetThreadInfo::<Identity, Impl, OFFSET>,
            GetCurrentThreadID: GetCurrentThreadID::<Identity, Impl, OFFSET>,
            GetClassIDInfo: GetClassIDInfo::<Identity, Impl, OFFSET>,
            GetFunctionInfo: GetFunctionInfo::<Identity, Impl, OFFSET>,
            SetEventMask: SetEventMask::<Identity, Impl, OFFSET>,
            SetEnterLeaveFunctionHooks: SetEnterLeaveFunctionHooks::<Identity, Impl, OFFSET>,
            SetFunctionIDMapper: SetFunctionIDMapper::<Identity, Impl, OFFSET>,
            GetTokenAndMetaDataFromFunction: GetTokenAndMetaDataFromFunction::<Identity, Impl, OFFSET>,
            GetModuleInfo: GetModuleInfo::<Identity, Impl, OFFSET>,
            GetModuleMetaData: GetModuleMetaData::<Identity, Impl, OFFSET>,
            GetILFunctionBody: GetILFunctionBody::<Identity, Impl, OFFSET>,
            GetILFunctionBodyAllocator: GetILFunctionBodyAllocator::<Identity, Impl, OFFSET>,
            SetILFunctionBody: SetILFunctionBody::<Identity, Impl, OFFSET>,
            GetAppDomainInfo: GetAppDomainInfo::<Identity, Impl, OFFSET>,
            GetAssemblyInfo: GetAssemblyInfo::<Identity, Impl, OFFSET>,
            SetFunctionReJIT: SetFunctionReJIT::<Identity, Impl, OFFSET>,
            ForceGC: ForceGC::<Identity, Impl, OFFSET>,
            SetILInstrumentedCodeMap: SetILInstrumentedCodeMap::<Identity, Impl, OFFSET>,
            GetInprocInspectionInterface: GetInprocInspectionInterface::<Identity, Impl, OFFSET>,
            GetInprocInspectionIThisThread: GetInprocInspectionIThisThread::<Identity, Impl, OFFSET>,
            GetThreadContext: GetThreadContext::<Identity, Impl, OFFSET>,
            BeginInprocDebugging: BeginInprocDebugging::<Identity, Impl, OFFSET>,
            EndInprocDebugging: EndInprocDebugging::<Identity, Impl, OFFSET>,
            GetILToNativeMapping: GetILToNativeMapping::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICorProfilerInfo as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`, `\"Win32_Foundation\"`, `\"Win32_System_WinRT_Metadata\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WinRT_Metadata"))]
pub trait ICorProfilerInfo2_Impl: Sized + ICorProfilerInfo_Impl {
    fn DoStackSnapshot(&self, thread: usize, callback: *mut StackSnapshotCallback, infoflags: u32, clientdata: *mut ::core::ffi::c_void, context: *mut u8, contextsize: u32) -> ::windows::core::Result<()>;
    fn SetEnterLeaveFunctionHooks2(&self, pfuncenter: *mut FunctionEnter2, pfuncleave: *mut FunctionLeave2, pfunctailcall: *mut FunctionTailcall2) -> ::windows::core::Result<()>;
    fn GetFunctionInfo2(&self, funcid: usize, frameinfo: usize, pclassid: *mut usize, pmoduleid: *mut usize, ptoken: *mut u32, ctypeargs: u32, pctypeargs: *mut u32, typeargs: *mut usize) -> ::windows::core::Result<()>;
    fn GetStringLayout(&self, pbufferlengthoffset: *mut u32, pstringlengthoffset: *mut u32, pbufferoffset: *mut u32) -> ::windows::core::Result<()>;
    fn GetClassLayout(&self, classid: usize, rfieldoffset: *mut super::super::WinRT::Metadata::COR_FIELD_OFFSET, cfieldoffset: u32, pcfieldoffset: *mut u32, pulclasssize: *mut u32) -> ::windows::core::Result<()>;
    fn GetClassIDInfo2(&self, classid: usize, pmoduleid: *mut usize, ptypedeftoken: *mut u32, pparentclassid: *mut usize, cnumtypeargs: u32, pcnumtypeargs: *mut u32, typeargs: *mut usize) -> ::windows::core::Result<()>;
    fn GetCodeInfo2(&self, functionid: usize, ccodeinfos: u32, pccodeinfos: *mut u32, codeinfos: *mut COR_PRF_CODE_INFO) -> ::windows::core::Result<()>;
    fn GetClassFromTokenAndTypeArgs(&self, moduleid: usize, typedef: u32, ctypeargs: u32, typeargs: *mut usize, pclassid: *mut usize) -> ::windows::core::Result<()>;
    fn GetFunctionFromTokenAndTypeArgs(&self, moduleid: usize, funcdef: u32, classid: usize, ctypeargs: u32, typeargs: *mut usize, pfunctionid: *mut usize) -> ::windows::core::Result<()>;
    fn EnumModuleFrozenObjects(&self, moduleid: usize) -> ::windows::core::Result<ICorProfilerObjectEnum>;
    fn GetArrayObjectInfo(&self, objectid: usize, cdimensions: u32, pdimensionsizes: *mut u32, pdimensionlowerbounds: *mut i32, ppdata: *mut *mut u8) -> ::windows::core::Result<()>;
    fn GetBoxClassLayout(&self, classid: usize, pbufferoffset: *mut u32) -> ::windows::core::Result<()>;
    fn GetThreadAppDomain(&self, threadid: usize, pappdomainid: *mut usize) -> ::windows::core::Result<()>;
    fn GetRVAStaticAddress(&self, classid: usize, fieldtoken: u32, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetAppDomainStaticAddress(&self, classid: usize, fieldtoken: u32, appdomainid: usize, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetThreadStaticAddress(&self, classid: usize, fieldtoken: u32, threadid: usize, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetContextStaticAddress(&self, classid: usize, fieldtoken: u32, contextid: usize, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetStaticFieldInfo(&self, classid: usize, fieldtoken: u32, pfieldinfo: *mut COR_PRF_STATIC_TYPE) -> ::windows::core::Result<()>;
    fn GetGenerationBounds(&self, cobjectranges: u32, pcobjectranges: *mut u32, ranges: *mut COR_PRF_GC_GENERATION_RANGE) -> ::windows::core::Result<()>;
    fn GetObjectGeneration(&self, objectid: usize, range: *mut COR_PRF_GC_GENERATION_RANGE) -> ::windows::core::Result<()>;
    fn GetNotifiedExceptionClauseInfo(&self, pinfo: *mut COR_PRF_EX_CLAUSE_INFO) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WinRT_Metadata"))]
impl ::windows::core::RuntimeName for ICorProfilerInfo2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WinRT_Metadata"))]
impl ICorProfilerInfo2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo2_Impl, const OFFSET: isize>() -> ICorProfilerInfo2_Vtbl {
        unsafe extern "system" fn DoStackSnapshot<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, thread: usize, callback: *mut StackSnapshotCallback, infoflags: u32, clientdata: *mut ::core::ffi::c_void, context: *mut u8, contextsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DoStackSnapshot(::core::mem::transmute_copy(&thread), ::core::mem::transmute_copy(&callback), ::core::mem::transmute_copy(&infoflags), ::core::mem::transmute_copy(&clientdata), ::core::mem::transmute_copy(&context), ::core::mem::transmute_copy(&contextsize)).into()
        }
        unsafe extern "system" fn SetEnterLeaveFunctionHooks2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfuncenter: *mut FunctionEnter2, pfuncleave: *mut FunctionLeave2, pfunctailcall: *mut FunctionTailcall2) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEnterLeaveFunctionHooks2(::core::mem::transmute_copy(&pfuncenter), ::core::mem::transmute_copy(&pfuncleave), ::core::mem::transmute_copy(&pfunctailcall)).into()
        }
        unsafe extern "system" fn GetFunctionInfo2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, funcid: usize, frameinfo: usize, pclassid: *mut usize, pmoduleid: *mut usize, ptoken: *mut u32, ctypeargs: u32, pctypeargs: *mut u32, typeargs: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFunctionInfo2(::core::mem::transmute_copy(&funcid), ::core::mem::transmute_copy(&frameinfo), ::core::mem::transmute_copy(&pclassid), ::core::mem::transmute_copy(&pmoduleid), ::core::mem::transmute_copy(&ptoken), ::core::mem::transmute_copy(&ctypeargs), ::core::mem::transmute_copy(&pctypeargs), ::core::mem::transmute_copy(&typeargs)).into()
        }
        unsafe extern "system" fn GetStringLayout<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbufferlengthoffset: *mut u32, pstringlengthoffset: *mut u32, pbufferoffset: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetStringLayout(::core::mem::transmute_copy(&pbufferlengthoffset), ::core::mem::transmute_copy(&pstringlengthoffset), ::core::mem::transmute_copy(&pbufferoffset)).into()
        }
        unsafe extern "system" fn GetClassLayout<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, classid: usize, rfieldoffset: *mut super::super::WinRT::Metadata::COR_FIELD_OFFSET, cfieldoffset: u32, pcfieldoffset: *mut u32, pulclasssize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetClassLayout(::core::mem::transmute_copy(&classid), ::core::mem::transmute_copy(&rfieldoffset), ::core::mem::transmute_copy(&cfieldoffset), ::core::mem::transmute_copy(&pcfieldoffset), ::core::mem::transmute_copy(&pulclasssize)).into()
        }
        unsafe extern "system" fn GetClassIDInfo2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, classid: usize, pmoduleid: *mut usize, ptypedeftoken: *mut u32, pparentclassid: *mut usize, cnumtypeargs: u32, pcnumtypeargs: *mut u32, typeargs: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetClassIDInfo2(::core::mem::transmute_copy(&classid), ::core::mem::transmute_copy(&pmoduleid), ::core::mem::transmute_copy(&ptypedeftoken), ::core::mem::transmute_copy(&pparentclassid), ::core::mem::transmute_copy(&cnumtypeargs), ::core::mem::transmute_copy(&pcnumtypeargs), ::core::mem::transmute_copy(&typeargs)).into()
        }
        unsafe extern "system" fn GetCodeInfo2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, functionid: usize, ccodeinfos: u32, pccodeinfos: *mut u32, codeinfos: *mut COR_PRF_CODE_INFO) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCodeInfo2(::core::mem::transmute_copy(&functionid), ::core::mem::transmute_copy(&ccodeinfos), ::core::mem::transmute_copy(&pccodeinfos), ::core::mem::transmute_copy(&codeinfos)).into()
        }
        unsafe extern "system" fn GetClassFromTokenAndTypeArgs<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, moduleid: usize, typedef: u32, ctypeargs: u32, typeargs: *mut usize, pclassid: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetClassFromTokenAndTypeArgs(::core::mem::transmute_copy(&moduleid), ::core::mem::transmute_copy(&typedef), ::core::mem::transmute_copy(&ctypeargs), ::core::mem::transmute_copy(&typeargs), ::core::mem::transmute_copy(&pclassid)).into()
        }
        unsafe extern "system" fn GetFunctionFromTokenAndTypeArgs<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, moduleid: usize, funcdef: u32, classid: usize, ctypeargs: u32, typeargs: *mut usize, pfunctionid: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFunctionFromTokenAndTypeArgs(::core::mem::transmute_copy(&moduleid), ::core::mem::transmute_copy(&funcdef), ::core::mem::transmute_copy(&classid), ::core::mem::transmute_copy(&ctypeargs), ::core::mem::transmute_copy(&typeargs), ::core::mem::transmute_copy(&pfunctionid)).into()
        }
        unsafe extern "system" fn EnumModuleFrozenObjects<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, moduleid: usize, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumModuleFrozenObjects(::core::mem::transmute_copy(&moduleid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetArrayObjectInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objectid: usize, cdimensions: u32, pdimensionsizes: *mut u32, pdimensionlowerbounds: *mut i32, ppdata: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetArrayObjectInfo(::core::mem::transmute_copy(&objectid), ::core::mem::transmute_copy(&cdimensions), ::core::mem::transmute_copy(&pdimensionsizes), ::core::mem::transmute_copy(&pdimensionlowerbounds), ::core::mem::transmute_copy(&ppdata)).into()
        }
        unsafe extern "system" fn GetBoxClassLayout<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, classid: usize, pbufferoffset: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetBoxClassLayout(::core::mem::transmute_copy(&classid), ::core::mem::transmute_copy(&pbufferoffset)).into()
        }
        unsafe extern "system" fn GetThreadAppDomain<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, threadid: usize, pappdomainid: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetThreadAppDomain(::core::mem::transmute_copy(&threadid), ::core::mem::transmute_copy(&pappdomainid)).into()
        }
        unsafe extern "system" fn GetRVAStaticAddress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, classid: usize, fieldtoken: u32, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetRVAStaticAddress(::core::mem::transmute_copy(&classid), ::core::mem::transmute_copy(&fieldtoken), ::core::mem::transmute_copy(&ppaddress)).into()
        }
        unsafe extern "system" fn GetAppDomainStaticAddress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, classid: usize, fieldtoken: u32, appdomainid: usize, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAppDomainStaticAddress(::core::mem::transmute_copy(&classid), ::core::mem::transmute_copy(&fieldtoken), ::core::mem::transmute_copy(&appdomainid), ::core::mem::transmute_copy(&ppaddress)).into()
        }
        unsafe extern "system" fn GetThreadStaticAddress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, classid: usize, fieldtoken: u32, threadid: usize, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetThreadStaticAddress(::core::mem::transmute_copy(&classid), ::core::mem::transmute_copy(&fieldtoken), ::core::mem::transmute_copy(&threadid), ::core::mem::transmute_copy(&ppaddress)).into()
        }
        unsafe extern "system" fn GetContextStaticAddress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, classid: usize, fieldtoken: u32, contextid: usize, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetContextStaticAddress(::core::mem::transmute_copy(&classid), ::core::mem::transmute_copy(&fieldtoken), ::core::mem::transmute_copy(&contextid), ::core::mem::transmute_copy(&ppaddress)).into()
        }
        unsafe extern "system" fn GetStaticFieldInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, classid: usize, fieldtoken: u32, pfieldinfo: *mut COR_PRF_STATIC_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetStaticFieldInfo(::core::mem::transmute_copy(&classid), ::core::mem::transmute_copy(&fieldtoken), ::core::mem::transmute_copy(&pfieldinfo)).into()
        }
        unsafe extern "system" fn GetGenerationBounds<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cobjectranges: u32, pcobjectranges: *mut u32, ranges: *mut COR_PRF_GC_GENERATION_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetGenerationBounds(::core::mem::transmute_copy(&cobjectranges), ::core::mem::transmute_copy(&pcobjectranges), ::core::mem::transmute_copy(&ranges)).into()
        }
        unsafe extern "system" fn GetObjectGeneration<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objectid: usize, range: *mut COR_PRF_GC_GENERATION_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetObjectGeneration(::core::mem::transmute_copy(&objectid), ::core::mem::transmute_copy(&range)).into()
        }
        unsafe extern "system" fn GetNotifiedExceptionClauseInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *mut COR_PRF_EX_CLAUSE_INFO) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetNotifiedExceptionClauseInfo(::core::mem::transmute_copy(&pinfo)).into()
        }
        Self {
            base__: ICorProfilerInfo_Vtbl::new::<Identity, Impl, OFFSET>(),
            DoStackSnapshot: DoStackSnapshot::<Identity, Impl, OFFSET>,
            SetEnterLeaveFunctionHooks2: SetEnterLeaveFunctionHooks2::<Identity, Impl, OFFSET>,
            GetFunctionInfo2: GetFunctionInfo2::<Identity, Impl, OFFSET>,
            GetStringLayout: GetStringLayout::<Identity, Impl, OFFSET>,
            GetClassLayout: GetClassLayout::<Identity, Impl, OFFSET>,
            GetClassIDInfo2: GetClassIDInfo2::<Identity, Impl, OFFSET>,
            GetCodeInfo2: GetCodeInfo2::<Identity, Impl, OFFSET>,
            GetClassFromTokenAndTypeArgs: GetClassFromTokenAndTypeArgs::<Identity, Impl, OFFSET>,
            GetFunctionFromTokenAndTypeArgs: GetFunctionFromTokenAndTypeArgs::<Identity, Impl, OFFSET>,
            EnumModuleFrozenObjects: EnumModuleFrozenObjects::<Identity, Impl, OFFSET>,
            GetArrayObjectInfo: GetArrayObjectInfo::<Identity, Impl, OFFSET>,
            GetBoxClassLayout: GetBoxClassLayout::<Identity, Impl, OFFSET>,
            GetThreadAppDomain: GetThreadAppDomain::<Identity, Impl, OFFSET>,
            GetRVAStaticAddress: GetRVAStaticAddress::<Identity, Impl, OFFSET>,
            GetAppDomainStaticAddress: GetAppDomainStaticAddress::<Identity, Impl, OFFSET>,
            GetThreadStaticAddress: GetThreadStaticAddress::<Identity, Impl, OFFSET>,
            GetContextStaticAddress: GetContextStaticAddress::<Identity, Impl, OFFSET>,
            GetStaticFieldInfo: GetStaticFieldInfo::<Identity, Impl, OFFSET>,
            GetGenerationBounds: GetGenerationBounds::<Identity, Impl, OFFSET>,
            GetObjectGeneration: GetObjectGeneration::<Identity, Impl, OFFSET>,
            GetNotifiedExceptionClauseInfo: GetNotifiedExceptionClauseInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICorProfilerInfo2 as ::windows::core::ComInterface>::IID || iid == &<ICorProfilerInfo as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`, `\"Win32_Foundation\"`, `\"Win32_System_WinRT_Metadata\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WinRT_Metadata"))]
pub trait ICorProfilerInfo3_Impl: Sized + ICorProfilerInfo2_Impl {
    fn EnumJITedFunctions(&self) -> ::windows::core::Result<ICorProfilerFunctionEnum>;
    fn RequestProfilerDetach(&self, dwexpectedcompletionmilliseconds: u32) -> ::windows::core::Result<()>;
    fn SetFunctionIDMapper2(&self, pfunc: *mut FunctionIDMapper2, clientdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetStringLayout2(&self, pstringlengthoffset: *mut u32, pbufferoffset: *mut u32) -> ::windows::core::Result<()>;
    fn SetEnterLeaveFunctionHooks3(&self, pfuncenter3: *mut FunctionEnter3, pfuncleave3: *mut FunctionLeave3, pfunctailcall3: *mut FunctionTailcall3) -> ::windows::core::Result<()>;
    fn SetEnterLeaveFunctionHooks3WithInfo(&self, pfuncenter3withinfo: *mut FunctionEnter3WithInfo, pfuncleave3withinfo: *mut FunctionLeave3WithInfo, pfunctailcall3withinfo: *mut FunctionTailcall3WithInfo) -> ::windows::core::Result<()>;
    fn GetFunctionEnter3Info(&self, functionid: usize, eltinfo: usize, pframeinfo: *mut usize, pcbargumentinfo: *mut u32, pargumentinfo: *mut COR_PRF_FUNCTION_ARGUMENT_INFO) -> ::windows::core::Result<()>;
    fn GetFunctionLeave3Info(&self, functionid: usize, eltinfo: usize, pframeinfo: *mut usize, pretvalrange: *mut COR_PRF_FUNCTION_ARGUMENT_RANGE) -> ::windows::core::Result<()>;
    fn GetFunctionTailcall3Info(&self, functionid: usize, eltinfo: usize, pframeinfo: *mut usize) -> ::windows::core::Result<()>;
    fn EnumModules(&self) -> ::windows::core::Result<ICorProfilerModuleEnum>;
    fn GetRuntimeInformation(&self, pclrinstanceid: *mut u16, pruntimetype: *mut COR_PRF_RUNTIME_TYPE, pmajorversion: *mut u16, pminorversion: *mut u16, pbuildnumber: *mut u16, pqfeversion: *mut u16, cchversionstring: u32, pcchversionstring: *mut u32, szversionstring: ::windows::core::PWSTR) -> ::windows::core::Result<()>;
    fn GetThreadStaticAddress2(&self, classid: usize, fieldtoken: u32, appdomainid: usize, threadid: usize, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetAppDomainsContainingModule(&self, moduleid: usize, cappdomainids: u32, pcappdomainids: *mut u32, appdomainids: *mut usize) -> ::windows::core::Result<()>;
    fn GetModuleInfo2(&self, moduleid: usize, ppbaseloadaddress: *mut *mut u8, cchname: u32, pcchname: *mut u32, szname: ::windows::core::PWSTR, passemblyid: *mut usize, pdwmoduleflags: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WinRT_Metadata"))]
impl ::windows::core::RuntimeName for ICorProfilerInfo3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WinRT_Metadata"))]
impl ICorProfilerInfo3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo3_Impl, const OFFSET: isize>() -> ICorProfilerInfo3_Vtbl {
        unsafe extern "system" fn EnumJITedFunctions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumJITedFunctions() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestProfilerDetach<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwexpectedcompletionmilliseconds: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RequestProfilerDetach(::core::mem::transmute_copy(&dwexpectedcompletionmilliseconds)).into()
        }
        unsafe extern "system" fn SetFunctionIDMapper2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfunc: *mut FunctionIDMapper2, clientdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFunctionIDMapper2(::core::mem::transmute_copy(&pfunc), ::core::mem::transmute_copy(&clientdata)).into()
        }
        unsafe extern "system" fn GetStringLayout2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstringlengthoffset: *mut u32, pbufferoffset: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetStringLayout2(::core::mem::transmute_copy(&pstringlengthoffset), ::core::mem::transmute_copy(&pbufferoffset)).into()
        }
        unsafe extern "system" fn SetEnterLeaveFunctionHooks3<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfuncenter3: *mut FunctionEnter3, pfuncleave3: *mut FunctionLeave3, pfunctailcall3: *mut FunctionTailcall3) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEnterLeaveFunctionHooks3(::core::mem::transmute_copy(&pfuncenter3), ::core::mem::transmute_copy(&pfuncleave3), ::core::mem::transmute_copy(&pfunctailcall3)).into()
        }
        unsafe extern "system" fn SetEnterLeaveFunctionHooks3WithInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfuncenter3withinfo: *mut FunctionEnter3WithInfo, pfuncleave3withinfo: *mut FunctionLeave3WithInfo, pfunctailcall3withinfo: *mut FunctionTailcall3WithInfo) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEnterLeaveFunctionHooks3WithInfo(::core::mem::transmute_copy(&pfuncenter3withinfo), ::core::mem::transmute_copy(&pfuncleave3withinfo), ::core::mem::transmute_copy(&pfunctailcall3withinfo)).into()
        }
        unsafe extern "system" fn GetFunctionEnter3Info<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, functionid: usize, eltinfo: usize, pframeinfo: *mut usize, pcbargumentinfo: *mut u32, pargumentinfo: *mut COR_PRF_FUNCTION_ARGUMENT_INFO) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFunctionEnter3Info(::core::mem::transmute_copy(&functionid), ::core::mem::transmute_copy(&eltinfo), ::core::mem::transmute_copy(&pframeinfo), ::core::mem::transmute_copy(&pcbargumentinfo), ::core::mem::transmute_copy(&pargumentinfo)).into()
        }
        unsafe extern "system" fn GetFunctionLeave3Info<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, functionid: usize, eltinfo: usize, pframeinfo: *mut usize, pretvalrange: *mut COR_PRF_FUNCTION_ARGUMENT_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFunctionLeave3Info(::core::mem::transmute_copy(&functionid), ::core::mem::transmute_copy(&eltinfo), ::core::mem::transmute_copy(&pframeinfo), ::core::mem::transmute_copy(&pretvalrange)).into()
        }
        unsafe extern "system" fn GetFunctionTailcall3Info<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, functionid: usize, eltinfo: usize, pframeinfo: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFunctionTailcall3Info(::core::mem::transmute_copy(&functionid), ::core::mem::transmute_copy(&eltinfo), ::core::mem::transmute_copy(&pframeinfo)).into()
        }
        unsafe extern "system" fn EnumModules<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumModules() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRuntimeInformation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclrinstanceid: *mut u16, pruntimetype: *mut COR_PRF_RUNTIME_TYPE, pmajorversion: *mut u16, pminorversion: *mut u16, pbuildnumber: *mut u16, pqfeversion: *mut u16, cchversionstring: u32, pcchversionstring: *mut u32, szversionstring: ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetRuntimeInformation(::core::mem::transmute_copy(&pclrinstanceid), ::core::mem::transmute_copy(&pruntimetype), ::core::mem::transmute_copy(&pmajorversion), ::core::mem::transmute_copy(&pminorversion), ::core::mem::transmute_copy(&pbuildnumber), ::core::mem::transmute_copy(&pqfeversion), ::core::mem::transmute_copy(&cchversionstring), ::core::mem::transmute_copy(&pcchversionstring), ::core::mem::transmute_copy(&szversionstring)).into()
        }
        unsafe extern "system" fn GetThreadStaticAddress2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, classid: usize, fieldtoken: u32, appdomainid: usize, threadid: usize, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetThreadStaticAddress2(::core::mem::transmute_copy(&classid), ::core::mem::transmute_copy(&fieldtoken), ::core::mem::transmute_copy(&appdomainid), ::core::mem::transmute_copy(&threadid), ::core::mem::transmute_copy(&ppaddress)).into()
        }
        unsafe extern "system" fn GetAppDomainsContainingModule<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, moduleid: usize, cappdomainids: u32, pcappdomainids: *mut u32, appdomainids: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAppDomainsContainingModule(::core::mem::transmute_copy(&moduleid), ::core::mem::transmute_copy(&cappdomainids), ::core::mem::transmute_copy(&pcappdomainids), ::core::mem::transmute_copy(&appdomainids)).into()
        }
        unsafe extern "system" fn GetModuleInfo2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, moduleid: usize, ppbaseloadaddress: *mut *mut u8, cchname: u32, pcchname: *mut u32, szname: ::windows::core::PWSTR, passemblyid: *mut usize, pdwmoduleflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetModuleInfo2(::core::mem::transmute_copy(&moduleid), ::core::mem::transmute_copy(&ppbaseloadaddress), ::core::mem::transmute_copy(&cchname), ::core::mem::transmute_copy(&pcchname), ::core::mem::transmute_copy(&szname), ::core::mem::transmute_copy(&passemblyid), ::core::mem::transmute_copy(&pdwmoduleflags)).into()
        }
        Self {
            base__: ICorProfilerInfo2_Vtbl::new::<Identity, Impl, OFFSET>(),
            EnumJITedFunctions: EnumJITedFunctions::<Identity, Impl, OFFSET>,
            RequestProfilerDetach: RequestProfilerDetach::<Identity, Impl, OFFSET>,
            SetFunctionIDMapper2: SetFunctionIDMapper2::<Identity, Impl, OFFSET>,
            GetStringLayout2: GetStringLayout2::<Identity, Impl, OFFSET>,
            SetEnterLeaveFunctionHooks3: SetEnterLeaveFunctionHooks3::<Identity, Impl, OFFSET>,
            SetEnterLeaveFunctionHooks3WithInfo: SetEnterLeaveFunctionHooks3WithInfo::<Identity, Impl, OFFSET>,
            GetFunctionEnter3Info: GetFunctionEnter3Info::<Identity, Impl, OFFSET>,
            GetFunctionLeave3Info: GetFunctionLeave3Info::<Identity, Impl, OFFSET>,
            GetFunctionTailcall3Info: GetFunctionTailcall3Info::<Identity, Impl, OFFSET>,
            EnumModules: EnumModules::<Identity, Impl, OFFSET>,
            GetRuntimeInformation: GetRuntimeInformation::<Identity, Impl, OFFSET>,
            GetThreadStaticAddress2: GetThreadStaticAddress2::<Identity, Impl, OFFSET>,
            GetAppDomainsContainingModule: GetAppDomainsContainingModule::<Identity, Impl, OFFSET>,
            GetModuleInfo2: GetModuleInfo2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICorProfilerInfo3 as ::windows::core::ComInterface>::IID || iid == &<ICorProfilerInfo as ::windows::core::ComInterface>::IID || iid == &<ICorProfilerInfo2 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`, `\"Win32_Foundation\"`, `\"Win32_System_WinRT_Metadata\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WinRT_Metadata"))]
pub trait ICorProfilerInfo4_Impl: Sized + ICorProfilerInfo3_Impl {
    fn EnumThreads(&self) -> ::windows::core::Result<ICorProfilerThreadEnum>;
    fn InitializeCurrentThread(&self) -> ::windows::core::Result<()>;
    fn RequestReJIT(&self, cfunctions: u32, moduleids: *mut usize, methodids: *mut u32) -> ::windows::core::Result<()>;
    fn RequestRevert(&self, cfunctions: u32, moduleids: *mut usize, methodids: *mut u32, status: *mut ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn GetCodeInfo3(&self, functionid: usize, rejitid: usize, ccodeinfos: u32, pccodeinfos: *mut u32, codeinfos: *mut COR_PRF_CODE_INFO) -> ::windows::core::Result<()>;
    fn GetFunctionFromIP2(&self, ip: *mut u8, pfunctionid: *mut usize, prejitid: *mut usize) -> ::windows::core::Result<()>;
    fn GetReJITIDs(&self, functionid: usize, crejitids: u32, pcrejitids: *mut u32, rejitids: *mut usize) -> ::windows::core::Result<()>;
    fn GetILToNativeMapping2(&self, functionid: usize, rejitid: usize, cmap: u32, pcmap: *mut u32, map: *mut COR_DEBUG_IL_TO_NATIVE_MAP) -> ::windows::core::Result<()>;
    fn EnumJITedFunctions2(&self) -> ::windows::core::Result<ICorProfilerFunctionEnum>;
    fn GetObjectSize2(&self, objectid: usize, pcsize: *mut usize) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WinRT_Metadata"))]
impl ::windows::core::RuntimeName for ICorProfilerInfo4 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WinRT_Metadata"))]
impl ICorProfilerInfo4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo4_Impl, const OFFSET: isize>() -> ICorProfilerInfo4_Vtbl {
        unsafe extern "system" fn EnumThreads<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumThreads() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitializeCurrentThread<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InitializeCurrentThread().into()
        }
        unsafe extern "system" fn RequestReJIT<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cfunctions: u32, moduleids: *mut usize, methodids: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RequestReJIT(::core::mem::transmute_copy(&cfunctions), ::core::mem::transmute_copy(&moduleids), ::core::mem::transmute_copy(&methodids)).into()
        }
        unsafe extern "system" fn RequestRevert<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cfunctions: u32, moduleids: *mut usize, methodids: *mut u32, status: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RequestRevert(::core::mem::transmute_copy(&cfunctions), ::core::mem::transmute_copy(&moduleids), ::core::mem::transmute_copy(&methodids), ::core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn GetCodeInfo3<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, functionid: usize, rejitid: usize, ccodeinfos: u32, pccodeinfos: *mut u32, codeinfos: *mut COR_PRF_CODE_INFO) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCodeInfo3(::core::mem::transmute_copy(&functionid), ::core::mem::transmute_copy(&rejitid), ::core::mem::transmute_copy(&ccodeinfos), ::core::mem::transmute_copy(&pccodeinfos), ::core::mem::transmute_copy(&codeinfos)).into()
        }
        unsafe extern "system" fn GetFunctionFromIP2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ip: *mut u8, pfunctionid: *mut usize, prejitid: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFunctionFromIP2(::core::mem::transmute_copy(&ip), ::core::mem::transmute_copy(&pfunctionid), ::core::mem::transmute_copy(&prejitid)).into()
        }
        unsafe extern "system" fn GetReJITIDs<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, functionid: usize, crejitids: u32, pcrejitids: *mut u32, rejitids: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetReJITIDs(::core::mem::transmute_copy(&functionid), ::core::mem::transmute_copy(&crejitids), ::core::mem::transmute_copy(&pcrejitids), ::core::mem::transmute_copy(&rejitids)).into()
        }
        unsafe extern "system" fn GetILToNativeMapping2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, functionid: usize, rejitid: usize, cmap: u32, pcmap: *mut u32, map: *mut COR_DEBUG_IL_TO_NATIVE_MAP) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetILToNativeMapping2(::core::mem::transmute_copy(&functionid), ::core::mem::transmute_copy(&rejitid), ::core::mem::transmute_copy(&cmap), ::core::mem::transmute_copy(&pcmap), ::core::mem::transmute_copy(&map)).into()
        }
        unsafe extern "system" fn EnumJITedFunctions2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumJITedFunctions2() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObjectSize2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objectid: usize, pcsize: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetObjectSize2(::core::mem::transmute_copy(&objectid), ::core::mem::transmute_copy(&pcsize)).into()
        }
        Self {
            base__: ICorProfilerInfo3_Vtbl::new::<Identity, Impl, OFFSET>(),
            EnumThreads: EnumThreads::<Identity, Impl, OFFSET>,
            InitializeCurrentThread: InitializeCurrentThread::<Identity, Impl, OFFSET>,
            RequestReJIT: RequestReJIT::<Identity, Impl, OFFSET>,
            RequestRevert: RequestRevert::<Identity, Impl, OFFSET>,
            GetCodeInfo3: GetCodeInfo3::<Identity, Impl, OFFSET>,
            GetFunctionFromIP2: GetFunctionFromIP2::<Identity, Impl, OFFSET>,
            GetReJITIDs: GetReJITIDs::<Identity, Impl, OFFSET>,
            GetILToNativeMapping2: GetILToNativeMapping2::<Identity, Impl, OFFSET>,
            EnumJITedFunctions2: EnumJITedFunctions2::<Identity, Impl, OFFSET>,
            GetObjectSize2: GetObjectSize2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICorProfilerInfo4 as ::windows::core::ComInterface>::IID || iid == &<ICorProfilerInfo as ::windows::core::ComInterface>::IID || iid == &<ICorProfilerInfo2 as ::windows::core::ComInterface>::IID || iid == &<ICorProfilerInfo3 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`, `\"Win32_Foundation\"`, `\"Win32_System_WinRT_Metadata\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WinRT_Metadata"))]
pub trait ICorProfilerInfo5_Impl: Sized + ICorProfilerInfo4_Impl {
    fn GetEventMask2(&self, pdweventslow: *mut u32, pdweventshigh: *mut u32) -> ::windows::core::Result<()>;
    fn SetEventMask2(&self, dweventslow: u32, dweventshigh: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WinRT_Metadata"))]
impl ::windows::core::RuntimeName for ICorProfilerInfo5 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WinRT_Metadata"))]
impl ICorProfilerInfo5_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo5_Impl, const OFFSET: isize>() -> ICorProfilerInfo5_Vtbl {
        unsafe extern "system" fn GetEventMask2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdweventslow: *mut u32, pdweventshigh: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetEventMask2(::core::mem::transmute_copy(&pdweventslow), ::core::mem::transmute_copy(&pdweventshigh)).into()
        }
        unsafe extern "system" fn SetEventMask2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dweventslow: u32, dweventshigh: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEventMask2(::core::mem::transmute_copy(&dweventslow), ::core::mem::transmute_copy(&dweventshigh)).into()
        }
        Self {
            base__: ICorProfilerInfo4_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetEventMask2: GetEventMask2::<Identity, Impl, OFFSET>,
            SetEventMask2: SetEventMask2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICorProfilerInfo5 as ::windows::core::ComInterface>::IID || iid == &<ICorProfilerInfo as ::windows::core::ComInterface>::IID || iid == &<ICorProfilerInfo2 as ::windows::core::ComInterface>::IID || iid == &<ICorProfilerInfo3 as ::windows::core::ComInterface>::IID || iid == &<ICorProfilerInfo4 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`, `\"Win32_Foundation\"`, `\"Win32_System_WinRT_Metadata\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WinRT_Metadata"))]
pub trait ICorProfilerInfo6_Impl: Sized + ICorProfilerInfo5_Impl {
    fn EnumNgenModuleMethodsInliningThisMethod(&self, inlinersmoduleid: usize, inlineemoduleid: usize, inlineemethodid: u32, incompletedata: *mut super::super::super::Foundation::BOOL, ppenum: *mut ::core::option::Option<ICorProfilerMethodEnum>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WinRT_Metadata"))]
impl ::windows::core::RuntimeName for ICorProfilerInfo6 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WinRT_Metadata"))]
impl ICorProfilerInfo6_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo6_Impl, const OFFSET: isize>() -> ICorProfilerInfo6_Vtbl {
        unsafe extern "system" fn EnumNgenModuleMethodsInliningThisMethod<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inlinersmoduleid: usize, inlineemoduleid: usize, inlineemethodid: u32, incompletedata: *mut super::super::super::Foundation::BOOL, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumNgenModuleMethodsInliningThisMethod(::core::mem::transmute_copy(&inlinersmoduleid), ::core::mem::transmute_copy(&inlineemoduleid), ::core::mem::transmute_copy(&inlineemethodid), ::core::mem::transmute_copy(&incompletedata), ::core::mem::transmute_copy(&ppenum)).into()
        }
        Self {
            base__: ICorProfilerInfo5_Vtbl::new::<Identity, Impl, OFFSET>(),
            EnumNgenModuleMethodsInliningThisMethod: EnumNgenModuleMethodsInliningThisMethod::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICorProfilerInfo6 as ::windows::core::ComInterface>::IID || iid == &<ICorProfilerInfo as ::windows::core::ComInterface>::IID || iid == &<ICorProfilerInfo2 as ::windows::core::ComInterface>::IID || iid == &<ICorProfilerInfo3 as ::windows::core::ComInterface>::IID || iid == &<ICorProfilerInfo4 as ::windows::core::ComInterface>::IID || iid == &<ICorProfilerInfo5 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`, `\"Win32_Foundation\"`, `\"Win32_System_WinRT_Metadata\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WinRT_Metadata"))]
pub trait ICorProfilerInfo7_Impl: Sized + ICorProfilerInfo6_Impl {
    fn ApplyMetaData(&self, moduleid: usize) -> ::windows::core::Result<()>;
    fn GetInMemorySymbolsLength(&self, moduleid: usize, pcountsymbolbytes: *mut u32) -> ::windows::core::Result<()>;
    fn ReadInMemorySymbols(&self, moduleid: usize, symbolsreadoffset: u32, psymbolbytes: *mut u8, countsymbolbytes: u32, pcountsymbolbytesread: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WinRT_Metadata"))]
impl ::windows::core::RuntimeName for ICorProfilerInfo7 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WinRT_Metadata"))]
impl ICorProfilerInfo7_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo7_Impl, const OFFSET: isize>() -> ICorProfilerInfo7_Vtbl {
        unsafe extern "system" fn ApplyMetaData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, moduleid: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ApplyMetaData(::core::mem::transmute_copy(&moduleid)).into()
        }
        unsafe extern "system" fn GetInMemorySymbolsLength<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, moduleid: usize, pcountsymbolbytes: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetInMemorySymbolsLength(::core::mem::transmute_copy(&moduleid), ::core::mem::transmute_copy(&pcountsymbolbytes)).into()
        }
        unsafe extern "system" fn ReadInMemorySymbols<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, moduleid: usize, symbolsreadoffset: u32, psymbolbytes: *mut u8, countsymbolbytes: u32, pcountsymbolbytesread: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReadInMemorySymbols(::core::mem::transmute_copy(&moduleid), ::core::mem::transmute_copy(&symbolsreadoffset), ::core::mem::transmute_copy(&psymbolbytes), ::core::mem::transmute_copy(&countsymbolbytes), ::core::mem::transmute_copy(&pcountsymbolbytesread)).into()
        }
        Self {
            base__: ICorProfilerInfo6_Vtbl::new::<Identity, Impl, OFFSET>(),
            ApplyMetaData: ApplyMetaData::<Identity, Impl, OFFSET>,
            GetInMemorySymbolsLength: GetInMemorySymbolsLength::<Identity, Impl, OFFSET>,
            ReadInMemorySymbols: ReadInMemorySymbols::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICorProfilerInfo7 as ::windows::core::ComInterface>::IID || iid == &<ICorProfilerInfo as ::windows::core::ComInterface>::IID || iid == &<ICorProfilerInfo2 as ::windows::core::ComInterface>::IID || iid == &<ICorProfilerInfo3 as ::windows::core::ComInterface>::IID || iid == &<ICorProfilerInfo4 as ::windows::core::ComInterface>::IID || iid == &<ICorProfilerInfo5 as ::windows::core::ComInterface>::IID || iid == &<ICorProfilerInfo6 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`, `\"Win32_Foundation\"`, `\"Win32_System_WinRT_Metadata\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WinRT_Metadata"))]
pub trait ICorProfilerInfo8_Impl: Sized + ICorProfilerInfo7_Impl {
    fn IsFunctionDynamic(&self, functionid: usize, isdynamic: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetFunctionFromIP3(&self, ip: *mut u8, functionid: *mut usize, prejitid: *mut usize) -> ::windows::core::Result<()>;
    fn GetDynamicFunctionInfo(&self, functionid: usize, moduleid: *mut usize, ppvsig: *mut *mut u8, pbsig: *mut u32, cchname: u32, pcchname: *mut u32, wszname: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WinRT_Metadata"))]
impl ::windows::core::RuntimeName for ICorProfilerInfo8 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WinRT_Metadata"))]
impl ICorProfilerInfo8_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo8_Impl, const OFFSET: isize>() -> ICorProfilerInfo8_Vtbl {
        unsafe extern "system" fn IsFunctionDynamic<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, functionid: usize, isdynamic: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsFunctionDynamic(::core::mem::transmute_copy(&functionid), ::core::mem::transmute_copy(&isdynamic)).into()
        }
        unsafe extern "system" fn GetFunctionFromIP3<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ip: *mut u8, functionid: *mut usize, prejitid: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFunctionFromIP3(::core::mem::transmute_copy(&ip), ::core::mem::transmute_copy(&functionid), ::core::mem::transmute_copy(&prejitid)).into()
        }
        unsafe extern "system" fn GetDynamicFunctionInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerInfo8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, functionid: usize, moduleid: *mut usize, ppvsig: *mut *mut u8, pbsig: *mut u32, cchname: u32, pcchname: *mut u32, wszname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDynamicFunctionInfo(::core::mem::transmute_copy(&functionid), ::core::mem::transmute_copy(&moduleid), ::core::mem::transmute_copy(&ppvsig), ::core::mem::transmute_copy(&pbsig), ::core::mem::transmute_copy(&cchname), ::core::mem::transmute_copy(&pcchname), ::core::mem::transmute(&wszname)).into()
        }
        Self {
            base__: ICorProfilerInfo7_Vtbl::new::<Identity, Impl, OFFSET>(),
            IsFunctionDynamic: IsFunctionDynamic::<Identity, Impl, OFFSET>,
            GetFunctionFromIP3: GetFunctionFromIP3::<Identity, Impl, OFFSET>,
            GetDynamicFunctionInfo: GetDynamicFunctionInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICorProfilerInfo8 as ::windows::core::ComInterface>::IID || iid == &<ICorProfilerInfo as ::windows::core::ComInterface>::IID || iid == &<ICorProfilerInfo2 as ::windows::core::ComInterface>::IID || iid == &<ICorProfilerInfo3 as ::windows::core::ComInterface>::IID || iid == &<ICorProfilerInfo4 as ::windows::core::ComInterface>::IID || iid == &<ICorProfilerInfo5 as ::windows::core::ComInterface>::IID || iid == &<ICorProfilerInfo6 as ::windows::core::ComInterface>::IID || iid == &<ICorProfilerInfo7 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`, `\"implement\"`*"]
pub trait ICorProfilerMethodEnum_Impl: Sized {
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<ICorProfilerMethodEnum>;
    fn GetCount(&self, pcelt: *mut u32) -> ::windows::core::Result<()>;
    fn Next(&self, celt: u32, elements: *mut COR_PRF_METHOD, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ICorProfilerMethodEnum {}
impl ICorProfilerMethodEnum_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerMethodEnum_Impl, const OFFSET: isize>() -> ICorProfilerMethodEnum_Vtbl {
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerMethodEnum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerMethodEnum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerMethodEnum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerMethodEnum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcelt: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCount(::core::mem::transmute_copy(&pcelt)).into()
        }
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerMethodEnum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, elements: *mut COR_PRF_METHOD, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&elements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICorProfilerMethodEnum as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`, `\"implement\"`*"]
pub trait ICorProfilerModuleEnum_Impl: Sized {
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<ICorProfilerModuleEnum>;
    fn GetCount(&self, pcelt: *mut u32) -> ::windows::core::Result<()>;
    fn Next(&self, celt: u32, ids: *mut usize, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ICorProfilerModuleEnum {}
impl ICorProfilerModuleEnum_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerModuleEnum_Impl, const OFFSET: isize>() -> ICorProfilerModuleEnum_Vtbl {
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerModuleEnum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerModuleEnum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerModuleEnum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerModuleEnum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcelt: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCount(::core::mem::transmute_copy(&pcelt)).into()
        }
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerModuleEnum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ids: *mut usize, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ids), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICorProfilerModuleEnum as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`, `\"implement\"`*"]
pub trait ICorProfilerObjectEnum_Impl: Sized {
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<ICorProfilerObjectEnum>;
    fn GetCount(&self, pcelt: *mut u32) -> ::windows::core::Result<()>;
    fn Next(&self, celt: u32, objects: *mut usize, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ICorProfilerObjectEnum {}
impl ICorProfilerObjectEnum_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerObjectEnum_Impl, const OFFSET: isize>() -> ICorProfilerObjectEnum_Vtbl {
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerObjectEnum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerObjectEnum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerObjectEnum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerObjectEnum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcelt: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCount(::core::mem::transmute_copy(&pcelt)).into()
        }
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerObjectEnum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, objects: *mut usize, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&objects), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICorProfilerObjectEnum as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`, `\"implement\"`*"]
pub trait ICorProfilerThreadEnum_Impl: Sized {
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<ICorProfilerThreadEnum>;
    fn GetCount(&self, pcelt: *mut u32) -> ::windows::core::Result<()>;
    fn Next(&self, celt: u32, ids: *mut usize, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ICorProfilerThreadEnum {}
impl ICorProfilerThreadEnum_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerThreadEnum_Impl, const OFFSET: isize>() -> ICorProfilerThreadEnum_Vtbl {
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerThreadEnum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerThreadEnum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerThreadEnum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerThreadEnum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcelt: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCount(::core::mem::transmute_copy(&pcelt)).into()
        }
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorProfilerThreadEnum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ids: *mut usize, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ids), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICorProfilerThreadEnum as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`, `\"implement\"`*"]
pub trait IMethodMalloc_Impl: Sized {
    fn Alloc(&self, cb: u32) -> *mut ::core::ffi::c_void;
}
impl ::windows::core::RuntimeName for IMethodMalloc {}
impl IMethodMalloc_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMethodMalloc_Impl, const OFFSET: isize>() -> IMethodMalloc_Vtbl {
        unsafe extern "system" fn Alloc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMethodMalloc_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cb: u32) -> *mut ::core::ffi::c_void {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Alloc(::core::mem::transmute_copy(&cb))
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Alloc: Alloc::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMethodMalloc as ::windows::core::ComInterface>::IID
    }
}
