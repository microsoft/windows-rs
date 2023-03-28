#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
#[repr(transparent)]
pub struct ICorProfilerAssemblyReferenceProvider(::windows::core::IUnknown);
impl ICorProfilerAssemblyReferenceProvider {
    #[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
    #[cfg(feature = "Win32_System_WinRT_Metadata")]
    pub unsafe fn AddAssemblyReference(&self, passemblyrefinfo: *const COR_PRF_ASSEMBLY_REFERENCE_INFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddAssemblyReference)(::windows::core::Interface::as_raw(self), passemblyrefinfo).ok()
    }
}
::windows::imp::interface_hierarchy!(ICorProfilerAssemblyReferenceProvider, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ICorProfilerAssemblyReferenceProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICorProfilerAssemblyReferenceProvider {}
impl ::core::fmt::Debug for ICorProfilerAssemblyReferenceProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICorProfilerAssemblyReferenceProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICorProfilerAssemblyReferenceProvider {
    type Vtable = ICorProfilerAssemblyReferenceProvider_Vtbl;
}
impl ::core::clone::Clone for ICorProfilerAssemblyReferenceProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICorProfilerAssemblyReferenceProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x66a78c24_2eef_4f65_b45f_dd1d8038bf3c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorProfilerAssemblyReferenceProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_WinRT_Metadata")]
    pub AddAssemblyReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, passemblyrefinfo: *const COR_PRF_ASSEMBLY_REFERENCE_INFO) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_WinRT_Metadata"))]
    AddAssemblyReference: usize,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
#[repr(transparent)]
pub struct ICorProfilerCallback(::windows::core::IUnknown);
impl ICorProfilerCallback {
    pub unsafe fn Initialize<P0>(&self, picorprofilerinfounk: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), picorprofilerinfounk.into_param().abi()).ok()
    }
    pub unsafe fn Shutdown(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Shutdown)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn AppDomainCreationStarted(&self, appdomainid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AppDomainCreationStarted)(::windows::core::Interface::as_raw(self), appdomainid).ok()
    }
    pub unsafe fn AppDomainCreationFinished(&self, appdomainid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AppDomainCreationFinished)(::windows::core::Interface::as_raw(self), appdomainid, hrstatus).ok()
    }
    pub unsafe fn AppDomainShutdownStarted(&self, appdomainid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AppDomainShutdownStarted)(::windows::core::Interface::as_raw(self), appdomainid).ok()
    }
    pub unsafe fn AppDomainShutdownFinished(&self, appdomainid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AppDomainShutdownFinished)(::windows::core::Interface::as_raw(self), appdomainid, hrstatus).ok()
    }
    pub unsafe fn AssemblyLoadStarted(&self, assemblyid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AssemblyLoadStarted)(::windows::core::Interface::as_raw(self), assemblyid).ok()
    }
    pub unsafe fn AssemblyLoadFinished(&self, assemblyid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AssemblyLoadFinished)(::windows::core::Interface::as_raw(self), assemblyid, hrstatus).ok()
    }
    pub unsafe fn AssemblyUnloadStarted(&self, assemblyid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AssemblyUnloadStarted)(::windows::core::Interface::as_raw(self), assemblyid).ok()
    }
    pub unsafe fn AssemblyUnloadFinished(&self, assemblyid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AssemblyUnloadFinished)(::windows::core::Interface::as_raw(self), assemblyid, hrstatus).ok()
    }
    pub unsafe fn ModuleLoadStarted(&self, moduleid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ModuleLoadStarted)(::windows::core::Interface::as_raw(self), moduleid).ok()
    }
    pub unsafe fn ModuleLoadFinished(&self, moduleid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ModuleLoadFinished)(::windows::core::Interface::as_raw(self), moduleid, hrstatus).ok()
    }
    pub unsafe fn ModuleUnloadStarted(&self, moduleid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ModuleUnloadStarted)(::windows::core::Interface::as_raw(self), moduleid).ok()
    }
    pub unsafe fn ModuleUnloadFinished(&self, moduleid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ModuleUnloadFinished)(::windows::core::Interface::as_raw(self), moduleid, hrstatus).ok()
    }
    pub unsafe fn ModuleAttachedToAssembly(&self, moduleid: usize, assemblyid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ModuleAttachedToAssembly)(::windows::core::Interface::as_raw(self), moduleid, assemblyid).ok()
    }
    pub unsafe fn ClassLoadStarted(&self, classid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ClassLoadStarted)(::windows::core::Interface::as_raw(self), classid).ok()
    }
    pub unsafe fn ClassLoadFinished(&self, classid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ClassLoadFinished)(::windows::core::Interface::as_raw(self), classid, hrstatus).ok()
    }
    pub unsafe fn ClassUnloadStarted(&self, classid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ClassUnloadStarted)(::windows::core::Interface::as_raw(self), classid).ok()
    }
    pub unsafe fn ClassUnloadFinished(&self, classid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ClassUnloadFinished)(::windows::core::Interface::as_raw(self), classid, hrstatus).ok()
    }
    pub unsafe fn FunctionUnloadStarted(&self, functionid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).FunctionUnloadStarted)(::windows::core::Interface::as_raw(self), functionid).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn JITCompilationStarted<P0>(&self, functionid: usize, fissafetoblock: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).JITCompilationStarted)(::windows::core::Interface::as_raw(self), functionid, fissafetoblock.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn JITCompilationFinished<P0>(&self, functionid: usize, hrstatus: ::windows::core::HRESULT, fissafetoblock: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).JITCompilationFinished)(::windows::core::Interface::as_raw(self), functionid, hrstatus, fissafetoblock.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn JITCachedFunctionSearchStarted(&self, functionid: usize, pbusecachedfunction: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).JITCachedFunctionSearchStarted)(::windows::core::Interface::as_raw(self), functionid, pbusecachedfunction).ok()
    }
    pub unsafe fn JITCachedFunctionSearchFinished(&self, functionid: usize, result: COR_PRF_JIT_CACHE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).JITCachedFunctionSearchFinished)(::windows::core::Interface::as_raw(self), functionid, result).ok()
    }
    pub unsafe fn JITFunctionPitched(&self, functionid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).JITFunctionPitched)(::windows::core::Interface::as_raw(self), functionid).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn JITInlining(&self, callerid: usize, calleeid: usize, pfshouldinline: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).JITInlining)(::windows::core::Interface::as_raw(self), callerid, calleeid, pfshouldinline).ok()
    }
    pub unsafe fn ThreadCreated(&self, threadid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ThreadCreated)(::windows::core::Interface::as_raw(self), threadid).ok()
    }
    pub unsafe fn ThreadDestroyed(&self, threadid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ThreadDestroyed)(::windows::core::Interface::as_raw(self), threadid).ok()
    }
    pub unsafe fn ThreadAssignedToOSThread(&self, managedthreadid: usize, osthreadid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ThreadAssignedToOSThread)(::windows::core::Interface::as_raw(self), managedthreadid, osthreadid).ok()
    }
    pub unsafe fn RemotingClientInvocationStarted(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemotingClientInvocationStarted)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemotingClientSendingMessage<P0>(&self, pcookie: *mut ::windows::core::GUID, fisasync: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).RemotingClientSendingMessage)(::windows::core::Interface::as_raw(self), pcookie, fisasync.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemotingClientReceivingReply<P0>(&self, pcookie: *mut ::windows::core::GUID, fisasync: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).RemotingClientReceivingReply)(::windows::core::Interface::as_raw(self), pcookie, fisasync.into_param().abi()).ok()
    }
    pub unsafe fn RemotingClientInvocationFinished(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemotingClientInvocationFinished)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemotingServerReceivingMessage<P0>(&self, pcookie: *mut ::windows::core::GUID, fisasync: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).RemotingServerReceivingMessage)(::windows::core::Interface::as_raw(self), pcookie, fisasync.into_param().abi()).ok()
    }
    pub unsafe fn RemotingServerInvocationStarted(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemotingServerInvocationStarted)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RemotingServerInvocationReturned(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemotingServerInvocationReturned)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemotingServerSendingReply<P0>(&self, pcookie: *mut ::windows::core::GUID, fisasync: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).RemotingServerSendingReply)(::windows::core::Interface::as_raw(self), pcookie, fisasync.into_param().abi()).ok()
    }
    pub unsafe fn UnmanagedToManagedTransition(&self, functionid: usize, reason: COR_PRF_TRANSITION_REASON) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UnmanagedToManagedTransition)(::windows::core::Interface::as_raw(self), functionid, reason).ok()
    }
    pub unsafe fn ManagedToUnmanagedTransition(&self, functionid: usize, reason: COR_PRF_TRANSITION_REASON) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ManagedToUnmanagedTransition)(::windows::core::Interface::as_raw(self), functionid, reason).ok()
    }
    pub unsafe fn RuntimeSuspendStarted(&self, suspendreason: COR_PRF_SUSPEND_REASON) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RuntimeSuspendStarted)(::windows::core::Interface::as_raw(self), suspendreason).ok()
    }
    pub unsafe fn RuntimeSuspendFinished(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RuntimeSuspendFinished)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RuntimeSuspendAborted(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RuntimeSuspendAborted)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RuntimeResumeStarted(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RuntimeResumeStarted)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RuntimeResumeFinished(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RuntimeResumeFinished)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RuntimeThreadSuspended(&self, threadid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RuntimeThreadSuspended)(::windows::core::Interface::as_raw(self), threadid).ok()
    }
    pub unsafe fn RuntimeThreadResumed(&self, threadid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RuntimeThreadResumed)(::windows::core::Interface::as_raw(self), threadid).ok()
    }
    pub unsafe fn MovedReferences(&self, cmovedobjectidranges: u32, oldobjectidrangestart: *mut usize, newobjectidrangestart: *mut usize, cobjectidrangelength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).MovedReferences)(::windows::core::Interface::as_raw(self), cmovedobjectidranges, oldobjectidrangestart, newobjectidrangestart, cobjectidrangelength).ok()
    }
    pub unsafe fn ObjectAllocated(&self, objectid: usize, classid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ObjectAllocated)(::windows::core::Interface::as_raw(self), objectid, classid).ok()
    }
    pub unsafe fn ObjectsAllocatedByClass(&self, cclasscount: u32, classids: *mut usize, cobjects: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ObjectsAllocatedByClass)(::windows::core::Interface::as_raw(self), cclasscount, classids, cobjects).ok()
    }
    pub unsafe fn ObjectReferences(&self, objectid: usize, classid: usize, cobjectrefs: u32, objectrefids: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ObjectReferences)(::windows::core::Interface::as_raw(self), objectid, classid, cobjectrefs, objectrefids).ok()
    }
    pub unsafe fn RootReferences(&self, crootrefs: u32, rootrefids: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RootReferences)(::windows::core::Interface::as_raw(self), crootrefs, rootrefids).ok()
    }
    pub unsafe fn ExceptionThrown(&self, thrownobjectid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ExceptionThrown)(::windows::core::Interface::as_raw(self), thrownobjectid).ok()
    }
    pub unsafe fn ExceptionSearchFunctionEnter(&self, functionid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ExceptionSearchFunctionEnter)(::windows::core::Interface::as_raw(self), functionid).ok()
    }
    pub unsafe fn ExceptionSearchFunctionLeave(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ExceptionSearchFunctionLeave)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ExceptionSearchFilterEnter(&self, functionid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ExceptionSearchFilterEnter)(::windows::core::Interface::as_raw(self), functionid).ok()
    }
    pub unsafe fn ExceptionSearchFilterLeave(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ExceptionSearchFilterLeave)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ExceptionSearchCatcherFound(&self, functionid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ExceptionSearchCatcherFound)(::windows::core::Interface::as_raw(self), functionid).ok()
    }
    pub unsafe fn ExceptionOSHandlerEnter(&self, __unused: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ExceptionOSHandlerEnter)(::windows::core::Interface::as_raw(self), __unused).ok()
    }
    pub unsafe fn ExceptionOSHandlerLeave(&self, __unused: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ExceptionOSHandlerLeave)(::windows::core::Interface::as_raw(self), __unused).ok()
    }
    pub unsafe fn ExceptionUnwindFunctionEnter(&self, functionid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ExceptionUnwindFunctionEnter)(::windows::core::Interface::as_raw(self), functionid).ok()
    }
    pub unsafe fn ExceptionUnwindFunctionLeave(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ExceptionUnwindFunctionLeave)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ExceptionUnwindFinallyEnter(&self, functionid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ExceptionUnwindFinallyEnter)(::windows::core::Interface::as_raw(self), functionid).ok()
    }
    pub unsafe fn ExceptionUnwindFinallyLeave(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ExceptionUnwindFinallyLeave)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ExceptionCatcherEnter(&self, functionid: usize, objectid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ExceptionCatcherEnter)(::windows::core::Interface::as_raw(self), functionid, objectid).ok()
    }
    pub unsafe fn ExceptionCatcherLeave(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ExceptionCatcherLeave)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn COMClassicVTableCreated(&self, wrappedclassid: usize, implementediid: *const ::windows::core::GUID, pvtable: *mut ::core::ffi::c_void, cslots: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).COMClassicVTableCreated)(::windows::core::Interface::as_raw(self), wrappedclassid, implementediid, pvtable, cslots).ok()
    }
    pub unsafe fn COMClassicVTableDestroyed(&self, wrappedclassid: usize, implementediid: *const ::windows::core::GUID, pvtable: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).COMClassicVTableDestroyed)(::windows::core::Interface::as_raw(self), wrappedclassid, implementediid, pvtable).ok()
    }
    pub unsafe fn ExceptionCLRCatcherFound(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ExceptionCLRCatcherFound)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ExceptionCLRCatcherExecute(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ExceptionCLRCatcherExecute)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(ICorProfilerCallback, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ICorProfilerCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICorProfilerCallback {}
impl ::core::fmt::Debug for ICorProfilerCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICorProfilerCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICorProfilerCallback {
    type Vtable = ICorProfilerCallback_Vtbl;
}
impl ::core::clone::Clone for ICorProfilerCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICorProfilerCallback {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x176fbed1_a55c_4796_98ca_a9da0ef883e7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorProfilerCallback_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, picorprofilerinfounk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Shutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AppDomainCreationStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appdomainid: usize) -> ::windows::core::HRESULT,
    pub AppDomainCreationFinished: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appdomainid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub AppDomainShutdownStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appdomainid: usize) -> ::windows::core::HRESULT,
    pub AppDomainShutdownFinished: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appdomainid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub AssemblyLoadStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, assemblyid: usize) -> ::windows::core::HRESULT,
    pub AssemblyLoadFinished: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, assemblyid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub AssemblyUnloadStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, assemblyid: usize) -> ::windows::core::HRESULT,
    pub AssemblyUnloadFinished: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, assemblyid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub ModuleLoadStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduleid: usize) -> ::windows::core::HRESULT,
    pub ModuleLoadFinished: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduleid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub ModuleUnloadStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduleid: usize) -> ::windows::core::HRESULT,
    pub ModuleUnloadFinished: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduleid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub ModuleAttachedToAssembly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduleid: usize, assemblyid: usize) -> ::windows::core::HRESULT,
    pub ClassLoadStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, classid: usize) -> ::windows::core::HRESULT,
    pub ClassLoadFinished: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, classid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub ClassUnloadStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, classid: usize) -> ::windows::core::HRESULT,
    pub ClassUnloadFinished: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, classid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub FunctionUnloadStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, functionid: usize) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub JITCompilationStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, functionid: usize, fissafetoblock: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    JITCompilationStarted: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub JITCompilationFinished: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, functionid: usize, hrstatus: ::windows::core::HRESULT, fissafetoblock: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    JITCompilationFinished: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub JITCachedFunctionSearchStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, functionid: usize, pbusecachedfunction: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    JITCachedFunctionSearchStarted: usize,
    pub JITCachedFunctionSearchFinished: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, functionid: usize, result: COR_PRF_JIT_CACHE) -> ::windows::core::HRESULT,
    pub JITFunctionPitched: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, functionid: usize) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub JITInlining: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, callerid: usize, calleeid: usize, pfshouldinline: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    JITInlining: usize,
    pub ThreadCreated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threadid: usize) -> ::windows::core::HRESULT,
    pub ThreadDestroyed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threadid: usize) -> ::windows::core::HRESULT,
    pub ThreadAssignedToOSThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, managedthreadid: usize, osthreadid: u32) -> ::windows::core::HRESULT,
    pub RemotingClientInvocationStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RemotingClientSendingMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcookie: *mut ::windows::core::GUID, fisasync: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RemotingClientSendingMessage: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RemotingClientReceivingReply: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcookie: *mut ::windows::core::GUID, fisasync: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RemotingClientReceivingReply: usize,
    pub RemotingClientInvocationFinished: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RemotingServerReceivingMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcookie: *mut ::windows::core::GUID, fisasync: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RemotingServerReceivingMessage: usize,
    pub RemotingServerInvocationStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemotingServerInvocationReturned: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RemotingServerSendingReply: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcookie: *mut ::windows::core::GUID, fisasync: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RemotingServerSendingReply: usize,
    pub UnmanagedToManagedTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, functionid: usize, reason: COR_PRF_TRANSITION_REASON) -> ::windows::core::HRESULT,
    pub ManagedToUnmanagedTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, functionid: usize, reason: COR_PRF_TRANSITION_REASON) -> ::windows::core::HRESULT,
    pub RuntimeSuspendStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, suspendreason: COR_PRF_SUSPEND_REASON) -> ::windows::core::HRESULT,
    pub RuntimeSuspendFinished: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RuntimeSuspendAborted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RuntimeResumeStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RuntimeResumeFinished: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RuntimeThreadSuspended: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threadid: usize) -> ::windows::core::HRESULT,
    pub RuntimeThreadResumed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threadid: usize) -> ::windows::core::HRESULT,
    pub MovedReferences: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cmovedobjectidranges: u32, oldobjectidrangestart: *mut usize, newobjectidrangestart: *mut usize, cobjectidrangelength: *mut u32) -> ::windows::core::HRESULT,
    pub ObjectAllocated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objectid: usize, classid: usize) -> ::windows::core::HRESULT,
    pub ObjectsAllocatedByClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cclasscount: u32, classids: *mut usize, cobjects: *mut u32) -> ::windows::core::HRESULT,
    pub ObjectReferences: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objectid: usize, classid: usize, cobjectrefs: u32, objectrefids: *mut usize) -> ::windows::core::HRESULT,
    pub RootReferences: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, crootrefs: u32, rootrefids: *mut usize) -> ::windows::core::HRESULT,
    pub ExceptionThrown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, thrownobjectid: usize) -> ::windows::core::HRESULT,
    pub ExceptionSearchFunctionEnter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, functionid: usize) -> ::windows::core::HRESULT,
    pub ExceptionSearchFunctionLeave: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ExceptionSearchFilterEnter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, functionid: usize) -> ::windows::core::HRESULT,
    pub ExceptionSearchFilterLeave: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ExceptionSearchCatcherFound: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, functionid: usize) -> ::windows::core::HRESULT,
    pub ExceptionOSHandlerEnter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __unused: usize) -> ::windows::core::HRESULT,
    pub ExceptionOSHandlerLeave: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __unused: usize) -> ::windows::core::HRESULT,
    pub ExceptionUnwindFunctionEnter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, functionid: usize) -> ::windows::core::HRESULT,
    pub ExceptionUnwindFunctionLeave: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ExceptionUnwindFinallyEnter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, functionid: usize) -> ::windows::core::HRESULT,
    pub ExceptionUnwindFinallyLeave: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ExceptionCatcherEnter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, functionid: usize, objectid: usize) -> ::windows::core::HRESULT,
    pub ExceptionCatcherLeave: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub COMClassicVTableCreated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wrappedclassid: usize, implementediid: *const ::windows::core::GUID, pvtable: *mut ::core::ffi::c_void, cslots: u32) -> ::windows::core::HRESULT,
    pub COMClassicVTableDestroyed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wrappedclassid: usize, implementediid: *const ::windows::core::GUID, pvtable: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ExceptionCLRCatcherFound: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ExceptionCLRCatcherExecute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
#[repr(transparent)]
pub struct ICorProfilerCallback2(::windows::core::IUnknown);
impl ICorProfilerCallback2 {
    pub unsafe fn Initialize<P0>(&self, picorprofilerinfounk: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.Initialize)(::windows::core::Interface::as_raw(self), picorprofilerinfounk.into_param().abi()).ok()
    }
    pub unsafe fn Shutdown(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Shutdown)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn AppDomainCreationStarted(&self, appdomainid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.AppDomainCreationStarted)(::windows::core::Interface::as_raw(self), appdomainid).ok()
    }
    pub unsafe fn AppDomainCreationFinished(&self, appdomainid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.AppDomainCreationFinished)(::windows::core::Interface::as_raw(self), appdomainid, hrstatus).ok()
    }
    pub unsafe fn AppDomainShutdownStarted(&self, appdomainid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.AppDomainShutdownStarted)(::windows::core::Interface::as_raw(self), appdomainid).ok()
    }
    pub unsafe fn AppDomainShutdownFinished(&self, appdomainid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.AppDomainShutdownFinished)(::windows::core::Interface::as_raw(self), appdomainid, hrstatus).ok()
    }
    pub unsafe fn AssemblyLoadStarted(&self, assemblyid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.AssemblyLoadStarted)(::windows::core::Interface::as_raw(self), assemblyid).ok()
    }
    pub unsafe fn AssemblyLoadFinished(&self, assemblyid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.AssemblyLoadFinished)(::windows::core::Interface::as_raw(self), assemblyid, hrstatus).ok()
    }
    pub unsafe fn AssemblyUnloadStarted(&self, assemblyid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.AssemblyUnloadStarted)(::windows::core::Interface::as_raw(self), assemblyid).ok()
    }
    pub unsafe fn AssemblyUnloadFinished(&self, assemblyid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.AssemblyUnloadFinished)(::windows::core::Interface::as_raw(self), assemblyid, hrstatus).ok()
    }
    pub unsafe fn ModuleLoadStarted(&self, moduleid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ModuleLoadStarted)(::windows::core::Interface::as_raw(self), moduleid).ok()
    }
    pub unsafe fn ModuleLoadFinished(&self, moduleid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ModuleLoadFinished)(::windows::core::Interface::as_raw(self), moduleid, hrstatus).ok()
    }
    pub unsafe fn ModuleUnloadStarted(&self, moduleid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ModuleUnloadStarted)(::windows::core::Interface::as_raw(self), moduleid).ok()
    }
    pub unsafe fn ModuleUnloadFinished(&self, moduleid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ModuleUnloadFinished)(::windows::core::Interface::as_raw(self), moduleid, hrstatus).ok()
    }
    pub unsafe fn ModuleAttachedToAssembly(&self, moduleid: usize, assemblyid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ModuleAttachedToAssembly)(::windows::core::Interface::as_raw(self), moduleid, assemblyid).ok()
    }
    pub unsafe fn ClassLoadStarted(&self, classid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ClassLoadStarted)(::windows::core::Interface::as_raw(self), classid).ok()
    }
    pub unsafe fn ClassLoadFinished(&self, classid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ClassLoadFinished)(::windows::core::Interface::as_raw(self), classid, hrstatus).ok()
    }
    pub unsafe fn ClassUnloadStarted(&self, classid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ClassUnloadStarted)(::windows::core::Interface::as_raw(self), classid).ok()
    }
    pub unsafe fn ClassUnloadFinished(&self, classid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ClassUnloadFinished)(::windows::core::Interface::as_raw(self), classid, hrstatus).ok()
    }
    pub unsafe fn FunctionUnloadStarted(&self, functionid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.FunctionUnloadStarted)(::windows::core::Interface::as_raw(self), functionid).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn JITCompilationStarted<P0>(&self, functionid: usize, fissafetoblock: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.JITCompilationStarted)(::windows::core::Interface::as_raw(self), functionid, fissafetoblock.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn JITCompilationFinished<P0>(&self, functionid: usize, hrstatus: ::windows::core::HRESULT, fissafetoblock: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.JITCompilationFinished)(::windows::core::Interface::as_raw(self), functionid, hrstatus, fissafetoblock.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn JITCachedFunctionSearchStarted(&self, functionid: usize, pbusecachedfunction: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.JITCachedFunctionSearchStarted)(::windows::core::Interface::as_raw(self), functionid, pbusecachedfunction).ok()
    }
    pub unsafe fn JITCachedFunctionSearchFinished(&self, functionid: usize, result: COR_PRF_JIT_CACHE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.JITCachedFunctionSearchFinished)(::windows::core::Interface::as_raw(self), functionid, result).ok()
    }
    pub unsafe fn JITFunctionPitched(&self, functionid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.JITFunctionPitched)(::windows::core::Interface::as_raw(self), functionid).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn JITInlining(&self, callerid: usize, calleeid: usize, pfshouldinline: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.JITInlining)(::windows::core::Interface::as_raw(self), callerid, calleeid, pfshouldinline).ok()
    }
    pub unsafe fn ThreadCreated(&self, threadid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ThreadCreated)(::windows::core::Interface::as_raw(self), threadid).ok()
    }
    pub unsafe fn ThreadDestroyed(&self, threadid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ThreadDestroyed)(::windows::core::Interface::as_raw(self), threadid).ok()
    }
    pub unsafe fn ThreadAssignedToOSThread(&self, managedthreadid: usize, osthreadid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ThreadAssignedToOSThread)(::windows::core::Interface::as_raw(self), managedthreadid, osthreadid).ok()
    }
    pub unsafe fn RemotingClientInvocationStarted(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.RemotingClientInvocationStarted)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemotingClientSendingMessage<P0>(&self, pcookie: *mut ::windows::core::GUID, fisasync: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.RemotingClientSendingMessage)(::windows::core::Interface::as_raw(self), pcookie, fisasync.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemotingClientReceivingReply<P0>(&self, pcookie: *mut ::windows::core::GUID, fisasync: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.RemotingClientReceivingReply)(::windows::core::Interface::as_raw(self), pcookie, fisasync.into_param().abi()).ok()
    }
    pub unsafe fn RemotingClientInvocationFinished(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.RemotingClientInvocationFinished)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemotingServerReceivingMessage<P0>(&self, pcookie: *mut ::windows::core::GUID, fisasync: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.RemotingServerReceivingMessage)(::windows::core::Interface::as_raw(self), pcookie, fisasync.into_param().abi()).ok()
    }
    pub unsafe fn RemotingServerInvocationStarted(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.RemotingServerInvocationStarted)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RemotingServerInvocationReturned(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.RemotingServerInvocationReturned)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemotingServerSendingReply<P0>(&self, pcookie: *mut ::windows::core::GUID, fisasync: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.RemotingServerSendingReply)(::windows::core::Interface::as_raw(self), pcookie, fisasync.into_param().abi()).ok()
    }
    pub unsafe fn UnmanagedToManagedTransition(&self, functionid: usize, reason: COR_PRF_TRANSITION_REASON) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.UnmanagedToManagedTransition)(::windows::core::Interface::as_raw(self), functionid, reason).ok()
    }
    pub unsafe fn ManagedToUnmanagedTransition(&self, functionid: usize, reason: COR_PRF_TRANSITION_REASON) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ManagedToUnmanagedTransition)(::windows::core::Interface::as_raw(self), functionid, reason).ok()
    }
    pub unsafe fn RuntimeSuspendStarted(&self, suspendreason: COR_PRF_SUSPEND_REASON) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.RuntimeSuspendStarted)(::windows::core::Interface::as_raw(self), suspendreason).ok()
    }
    pub unsafe fn RuntimeSuspendFinished(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.RuntimeSuspendFinished)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RuntimeSuspendAborted(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.RuntimeSuspendAborted)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RuntimeResumeStarted(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.RuntimeResumeStarted)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RuntimeResumeFinished(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.RuntimeResumeFinished)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RuntimeThreadSuspended(&self, threadid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.RuntimeThreadSuspended)(::windows::core::Interface::as_raw(self), threadid).ok()
    }
    pub unsafe fn RuntimeThreadResumed(&self, threadid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.RuntimeThreadResumed)(::windows::core::Interface::as_raw(self), threadid).ok()
    }
    pub unsafe fn MovedReferences(&self, cmovedobjectidranges: u32, oldobjectidrangestart: *mut usize, newobjectidrangestart: *mut usize, cobjectidrangelength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.MovedReferences)(::windows::core::Interface::as_raw(self), cmovedobjectidranges, oldobjectidrangestart, newobjectidrangestart, cobjectidrangelength).ok()
    }
    pub unsafe fn ObjectAllocated(&self, objectid: usize, classid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ObjectAllocated)(::windows::core::Interface::as_raw(self), objectid, classid).ok()
    }
    pub unsafe fn ObjectsAllocatedByClass(&self, cclasscount: u32, classids: *mut usize, cobjects: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ObjectsAllocatedByClass)(::windows::core::Interface::as_raw(self), cclasscount, classids, cobjects).ok()
    }
    pub unsafe fn ObjectReferences(&self, objectid: usize, classid: usize, cobjectrefs: u32, objectrefids: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ObjectReferences)(::windows::core::Interface::as_raw(self), objectid, classid, cobjectrefs, objectrefids).ok()
    }
    pub unsafe fn RootReferences(&self, crootrefs: u32, rootrefids: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.RootReferences)(::windows::core::Interface::as_raw(self), crootrefs, rootrefids).ok()
    }
    pub unsafe fn ExceptionThrown(&self, thrownobjectid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ExceptionThrown)(::windows::core::Interface::as_raw(self), thrownobjectid).ok()
    }
    pub unsafe fn ExceptionSearchFunctionEnter(&self, functionid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ExceptionSearchFunctionEnter)(::windows::core::Interface::as_raw(self), functionid).ok()
    }
    pub unsafe fn ExceptionSearchFunctionLeave(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ExceptionSearchFunctionLeave)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ExceptionSearchFilterEnter(&self, functionid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ExceptionSearchFilterEnter)(::windows::core::Interface::as_raw(self), functionid).ok()
    }
    pub unsafe fn ExceptionSearchFilterLeave(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ExceptionSearchFilterLeave)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ExceptionSearchCatcherFound(&self, functionid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ExceptionSearchCatcherFound)(::windows::core::Interface::as_raw(self), functionid).ok()
    }
    pub unsafe fn ExceptionOSHandlerEnter(&self, __unused: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ExceptionOSHandlerEnter)(::windows::core::Interface::as_raw(self), __unused).ok()
    }
    pub unsafe fn ExceptionOSHandlerLeave(&self, __unused: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ExceptionOSHandlerLeave)(::windows::core::Interface::as_raw(self), __unused).ok()
    }
    pub unsafe fn ExceptionUnwindFunctionEnter(&self, functionid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ExceptionUnwindFunctionEnter)(::windows::core::Interface::as_raw(self), functionid).ok()
    }
    pub unsafe fn ExceptionUnwindFunctionLeave(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ExceptionUnwindFunctionLeave)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ExceptionUnwindFinallyEnter(&self, functionid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ExceptionUnwindFinallyEnter)(::windows::core::Interface::as_raw(self), functionid).ok()
    }
    pub unsafe fn ExceptionUnwindFinallyLeave(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ExceptionUnwindFinallyLeave)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ExceptionCatcherEnter(&self, functionid: usize, objectid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ExceptionCatcherEnter)(::windows::core::Interface::as_raw(self), functionid, objectid).ok()
    }
    pub unsafe fn ExceptionCatcherLeave(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ExceptionCatcherLeave)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn COMClassicVTableCreated(&self, wrappedclassid: usize, implementediid: *const ::windows::core::GUID, pvtable: *mut ::core::ffi::c_void, cslots: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.COMClassicVTableCreated)(::windows::core::Interface::as_raw(self), wrappedclassid, implementediid, pvtable, cslots).ok()
    }
    pub unsafe fn COMClassicVTableDestroyed(&self, wrappedclassid: usize, implementediid: *const ::windows::core::GUID, pvtable: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.COMClassicVTableDestroyed)(::windows::core::Interface::as_raw(self), wrappedclassid, implementediid, pvtable).ok()
    }
    pub unsafe fn ExceptionCLRCatcherFound(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ExceptionCLRCatcherFound)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ExceptionCLRCatcherExecute(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ExceptionCLRCatcherExecute)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ThreadNameChanged(&self, threadid: usize, name: ::core::option::Option<&[u16]>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ThreadNameChanged)(::windows::core::Interface::as_raw(self), threadid, name.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(name.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GarbageCollectionStarted(&self, cgenerations: i32, generationcollected: *mut super::super::super::Foundation::BOOL, reason: COR_PRF_GC_REASON) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GarbageCollectionStarted)(::windows::core::Interface::as_raw(self), cgenerations, generationcollected, reason).ok()
    }
    pub unsafe fn SurvivingReferences(&self, csurvivingobjectidranges: u32, objectidrangestart: *mut usize, cobjectidrangelength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SurvivingReferences)(::windows::core::Interface::as_raw(self), csurvivingobjectidranges, objectidrangestart, cobjectidrangelength).ok()
    }
    pub unsafe fn GarbageCollectionFinished(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GarbageCollectionFinished)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn FinalizeableObjectQueued(&self, finalizerflags: u32, objectid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).FinalizeableObjectQueued)(::windows::core::Interface::as_raw(self), finalizerflags, objectid).ok()
    }
    pub unsafe fn RootReferences2(&self, crootrefs: u32, rootrefids: *mut usize, rootkinds: *mut COR_PRF_GC_ROOT_KIND, rootflags: *mut COR_PRF_GC_ROOT_FLAGS, rootids: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RootReferences2)(::windows::core::Interface::as_raw(self), crootrefs, rootrefids, rootkinds, rootflags, rootids).ok()
    }
    pub unsafe fn HandleCreated(&self, handleid: usize, initialobjectid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).HandleCreated)(::windows::core::Interface::as_raw(self), handleid, initialobjectid).ok()
    }
    pub unsafe fn HandleDestroyed(&self, handleid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).HandleDestroyed)(::windows::core::Interface::as_raw(self), handleid).ok()
    }
}
::windows::imp::interface_hierarchy!(ICorProfilerCallback2, ::windows::core::IUnknown, ICorProfilerCallback);
impl ::core::cmp::PartialEq for ICorProfilerCallback2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICorProfilerCallback2 {}
impl ::core::fmt::Debug for ICorProfilerCallback2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICorProfilerCallback2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICorProfilerCallback2 {
    type Vtable = ICorProfilerCallback2_Vtbl;
}
impl ::core::clone::Clone for ICorProfilerCallback2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICorProfilerCallback2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8a8cc829_ccf2_49fe_bbae_0f022228071a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorProfilerCallback2_Vtbl {
    pub base__: ICorProfilerCallback_Vtbl,
    pub ThreadNameChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threadid: usize, cchname: u32, name: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GarbageCollectionStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cgenerations: i32, generationcollected: *mut super::super::super::Foundation::BOOL, reason: COR_PRF_GC_REASON) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GarbageCollectionStarted: usize,
    pub SurvivingReferences: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, csurvivingobjectidranges: u32, objectidrangestart: *mut usize, cobjectidrangelength: *mut u32) -> ::windows::core::HRESULT,
    pub GarbageCollectionFinished: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FinalizeableObjectQueued: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, finalizerflags: u32, objectid: usize) -> ::windows::core::HRESULT,
    pub RootReferences2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, crootrefs: u32, rootrefids: *mut usize, rootkinds: *mut COR_PRF_GC_ROOT_KIND, rootflags: *mut COR_PRF_GC_ROOT_FLAGS, rootids: *mut usize) -> ::windows::core::HRESULT,
    pub HandleCreated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handleid: usize, initialobjectid: usize) -> ::windows::core::HRESULT,
    pub HandleDestroyed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handleid: usize) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
#[repr(transparent)]
pub struct ICorProfilerCallback3(::windows::core::IUnknown);
impl ICorProfilerCallback3 {
    pub unsafe fn Initialize<P0>(&self, picorprofilerinfounk: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.Initialize)(::windows::core::Interface::as_raw(self), picorprofilerinfounk.into_param().abi()).ok()
    }
    pub unsafe fn Shutdown(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.Shutdown)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn AppDomainCreationStarted(&self, appdomainid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.AppDomainCreationStarted)(::windows::core::Interface::as_raw(self), appdomainid).ok()
    }
    pub unsafe fn AppDomainCreationFinished(&self, appdomainid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.AppDomainCreationFinished)(::windows::core::Interface::as_raw(self), appdomainid, hrstatus).ok()
    }
    pub unsafe fn AppDomainShutdownStarted(&self, appdomainid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.AppDomainShutdownStarted)(::windows::core::Interface::as_raw(self), appdomainid).ok()
    }
    pub unsafe fn AppDomainShutdownFinished(&self, appdomainid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.AppDomainShutdownFinished)(::windows::core::Interface::as_raw(self), appdomainid, hrstatus).ok()
    }
    pub unsafe fn AssemblyLoadStarted(&self, assemblyid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.AssemblyLoadStarted)(::windows::core::Interface::as_raw(self), assemblyid).ok()
    }
    pub unsafe fn AssemblyLoadFinished(&self, assemblyid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.AssemblyLoadFinished)(::windows::core::Interface::as_raw(self), assemblyid, hrstatus).ok()
    }
    pub unsafe fn AssemblyUnloadStarted(&self, assemblyid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.AssemblyUnloadStarted)(::windows::core::Interface::as_raw(self), assemblyid).ok()
    }
    pub unsafe fn AssemblyUnloadFinished(&self, assemblyid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.AssemblyUnloadFinished)(::windows::core::Interface::as_raw(self), assemblyid, hrstatus).ok()
    }
    pub unsafe fn ModuleLoadStarted(&self, moduleid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.ModuleLoadStarted)(::windows::core::Interface::as_raw(self), moduleid).ok()
    }
    pub unsafe fn ModuleLoadFinished(&self, moduleid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.ModuleLoadFinished)(::windows::core::Interface::as_raw(self), moduleid, hrstatus).ok()
    }
    pub unsafe fn ModuleUnloadStarted(&self, moduleid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.ModuleUnloadStarted)(::windows::core::Interface::as_raw(self), moduleid).ok()
    }
    pub unsafe fn ModuleUnloadFinished(&self, moduleid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.ModuleUnloadFinished)(::windows::core::Interface::as_raw(self), moduleid, hrstatus).ok()
    }
    pub unsafe fn ModuleAttachedToAssembly(&self, moduleid: usize, assemblyid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.ModuleAttachedToAssembly)(::windows::core::Interface::as_raw(self), moduleid, assemblyid).ok()
    }
    pub unsafe fn ClassLoadStarted(&self, classid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.ClassLoadStarted)(::windows::core::Interface::as_raw(self), classid).ok()
    }
    pub unsafe fn ClassLoadFinished(&self, classid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.ClassLoadFinished)(::windows::core::Interface::as_raw(self), classid, hrstatus).ok()
    }
    pub unsafe fn ClassUnloadStarted(&self, classid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.ClassUnloadStarted)(::windows::core::Interface::as_raw(self), classid).ok()
    }
    pub unsafe fn ClassUnloadFinished(&self, classid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.ClassUnloadFinished)(::windows::core::Interface::as_raw(self), classid, hrstatus).ok()
    }
    pub unsafe fn FunctionUnloadStarted(&self, functionid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.FunctionUnloadStarted)(::windows::core::Interface::as_raw(self), functionid).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn JITCompilationStarted<P0>(&self, functionid: usize, fissafetoblock: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.JITCompilationStarted)(::windows::core::Interface::as_raw(self), functionid, fissafetoblock.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn JITCompilationFinished<P0>(&self, functionid: usize, hrstatus: ::windows::core::HRESULT, fissafetoblock: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.JITCompilationFinished)(::windows::core::Interface::as_raw(self), functionid, hrstatus, fissafetoblock.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn JITCachedFunctionSearchStarted(&self, functionid: usize, pbusecachedfunction: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.JITCachedFunctionSearchStarted)(::windows::core::Interface::as_raw(self), functionid, pbusecachedfunction).ok()
    }
    pub unsafe fn JITCachedFunctionSearchFinished(&self, functionid: usize, result: COR_PRF_JIT_CACHE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.JITCachedFunctionSearchFinished)(::windows::core::Interface::as_raw(self), functionid, result).ok()
    }
    pub unsafe fn JITFunctionPitched(&self, functionid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.JITFunctionPitched)(::windows::core::Interface::as_raw(self), functionid).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn JITInlining(&self, callerid: usize, calleeid: usize, pfshouldinline: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.JITInlining)(::windows::core::Interface::as_raw(self), callerid, calleeid, pfshouldinline).ok()
    }
    pub unsafe fn ThreadCreated(&self, threadid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.ThreadCreated)(::windows::core::Interface::as_raw(self), threadid).ok()
    }
    pub unsafe fn ThreadDestroyed(&self, threadid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.ThreadDestroyed)(::windows::core::Interface::as_raw(self), threadid).ok()
    }
    pub unsafe fn ThreadAssignedToOSThread(&self, managedthreadid: usize, osthreadid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.ThreadAssignedToOSThread)(::windows::core::Interface::as_raw(self), managedthreadid, osthreadid).ok()
    }
    pub unsafe fn RemotingClientInvocationStarted(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.RemotingClientInvocationStarted)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemotingClientSendingMessage<P0>(&self, pcookie: *mut ::windows::core::GUID, fisasync: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.RemotingClientSendingMessage)(::windows::core::Interface::as_raw(self), pcookie, fisasync.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemotingClientReceivingReply<P0>(&self, pcookie: *mut ::windows::core::GUID, fisasync: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.RemotingClientReceivingReply)(::windows::core::Interface::as_raw(self), pcookie, fisasync.into_param().abi()).ok()
    }
    pub unsafe fn RemotingClientInvocationFinished(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.RemotingClientInvocationFinished)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemotingServerReceivingMessage<P0>(&self, pcookie: *mut ::windows::core::GUID, fisasync: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.RemotingServerReceivingMessage)(::windows::core::Interface::as_raw(self), pcookie, fisasync.into_param().abi()).ok()
    }
    pub unsafe fn RemotingServerInvocationStarted(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.RemotingServerInvocationStarted)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RemotingServerInvocationReturned(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.RemotingServerInvocationReturned)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemotingServerSendingReply<P0>(&self, pcookie: *mut ::windows::core::GUID, fisasync: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.RemotingServerSendingReply)(::windows::core::Interface::as_raw(self), pcookie, fisasync.into_param().abi()).ok()
    }
    pub unsafe fn UnmanagedToManagedTransition(&self, functionid: usize, reason: COR_PRF_TRANSITION_REASON) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.UnmanagedToManagedTransition)(::windows::core::Interface::as_raw(self), functionid, reason).ok()
    }
    pub unsafe fn ManagedToUnmanagedTransition(&self, functionid: usize, reason: COR_PRF_TRANSITION_REASON) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.ManagedToUnmanagedTransition)(::windows::core::Interface::as_raw(self), functionid, reason).ok()
    }
    pub unsafe fn RuntimeSuspendStarted(&self, suspendreason: COR_PRF_SUSPEND_REASON) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.RuntimeSuspendStarted)(::windows::core::Interface::as_raw(self), suspendreason).ok()
    }
    pub unsafe fn RuntimeSuspendFinished(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.RuntimeSuspendFinished)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RuntimeSuspendAborted(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.RuntimeSuspendAborted)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RuntimeResumeStarted(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.RuntimeResumeStarted)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RuntimeResumeFinished(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.RuntimeResumeFinished)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RuntimeThreadSuspended(&self, threadid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.RuntimeThreadSuspended)(::windows::core::Interface::as_raw(self), threadid).ok()
    }
    pub unsafe fn RuntimeThreadResumed(&self, threadid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.RuntimeThreadResumed)(::windows::core::Interface::as_raw(self), threadid).ok()
    }
    pub unsafe fn MovedReferences(&self, cmovedobjectidranges: u32, oldobjectidrangestart: *mut usize, newobjectidrangestart: *mut usize, cobjectidrangelength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.MovedReferences)(::windows::core::Interface::as_raw(self), cmovedobjectidranges, oldobjectidrangestart, newobjectidrangestart, cobjectidrangelength).ok()
    }
    pub unsafe fn ObjectAllocated(&self, objectid: usize, classid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.ObjectAllocated)(::windows::core::Interface::as_raw(self), objectid, classid).ok()
    }
    pub unsafe fn ObjectsAllocatedByClass(&self, cclasscount: u32, classids: *mut usize, cobjects: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.ObjectsAllocatedByClass)(::windows::core::Interface::as_raw(self), cclasscount, classids, cobjects).ok()
    }
    pub unsafe fn ObjectReferences(&self, objectid: usize, classid: usize, cobjectrefs: u32, objectrefids: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.ObjectReferences)(::windows::core::Interface::as_raw(self), objectid, classid, cobjectrefs, objectrefids).ok()
    }
    pub unsafe fn RootReferences(&self, crootrefs: u32, rootrefids: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.RootReferences)(::windows::core::Interface::as_raw(self), crootrefs, rootrefids).ok()
    }
    pub unsafe fn ExceptionThrown(&self, thrownobjectid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.ExceptionThrown)(::windows::core::Interface::as_raw(self), thrownobjectid).ok()
    }
    pub unsafe fn ExceptionSearchFunctionEnter(&self, functionid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.ExceptionSearchFunctionEnter)(::windows::core::Interface::as_raw(self), functionid).ok()
    }
    pub unsafe fn ExceptionSearchFunctionLeave(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.ExceptionSearchFunctionLeave)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ExceptionSearchFilterEnter(&self, functionid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.ExceptionSearchFilterEnter)(::windows::core::Interface::as_raw(self), functionid).ok()
    }
    pub unsafe fn ExceptionSearchFilterLeave(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.ExceptionSearchFilterLeave)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ExceptionSearchCatcherFound(&self, functionid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.ExceptionSearchCatcherFound)(::windows::core::Interface::as_raw(self), functionid).ok()
    }
    pub unsafe fn ExceptionOSHandlerEnter(&self, __unused: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.ExceptionOSHandlerEnter)(::windows::core::Interface::as_raw(self), __unused).ok()
    }
    pub unsafe fn ExceptionOSHandlerLeave(&self, __unused: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.ExceptionOSHandlerLeave)(::windows::core::Interface::as_raw(self), __unused).ok()
    }
    pub unsafe fn ExceptionUnwindFunctionEnter(&self, functionid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.ExceptionUnwindFunctionEnter)(::windows::core::Interface::as_raw(self), functionid).ok()
    }
    pub unsafe fn ExceptionUnwindFunctionLeave(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.ExceptionUnwindFunctionLeave)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ExceptionUnwindFinallyEnter(&self, functionid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.ExceptionUnwindFinallyEnter)(::windows::core::Interface::as_raw(self), functionid).ok()
    }
    pub unsafe fn ExceptionUnwindFinallyLeave(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.ExceptionUnwindFinallyLeave)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ExceptionCatcherEnter(&self, functionid: usize, objectid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.ExceptionCatcherEnter)(::windows::core::Interface::as_raw(self), functionid, objectid).ok()
    }
    pub unsafe fn ExceptionCatcherLeave(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.ExceptionCatcherLeave)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn COMClassicVTableCreated(&self, wrappedclassid: usize, implementediid: *const ::windows::core::GUID, pvtable: *mut ::core::ffi::c_void, cslots: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.COMClassicVTableCreated)(::windows::core::Interface::as_raw(self), wrappedclassid, implementediid, pvtable, cslots).ok()
    }
    pub unsafe fn COMClassicVTableDestroyed(&self, wrappedclassid: usize, implementediid: *const ::windows::core::GUID, pvtable: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.COMClassicVTableDestroyed)(::windows::core::Interface::as_raw(self), wrappedclassid, implementediid, pvtable).ok()
    }
    pub unsafe fn ExceptionCLRCatcherFound(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.ExceptionCLRCatcherFound)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ExceptionCLRCatcherExecute(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.ExceptionCLRCatcherExecute)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ThreadNameChanged(&self, threadid: usize, name: ::core::option::Option<&[u16]>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ThreadNameChanged)(::windows::core::Interface::as_raw(self), threadid, name.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(name.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GarbageCollectionStarted(&self, cgenerations: i32, generationcollected: *mut super::super::super::Foundation::BOOL, reason: COR_PRF_GC_REASON) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GarbageCollectionStarted)(::windows::core::Interface::as_raw(self), cgenerations, generationcollected, reason).ok()
    }
    pub unsafe fn SurvivingReferences(&self, csurvivingobjectidranges: u32, objectidrangestart: *mut usize, cobjectidrangelength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SurvivingReferences)(::windows::core::Interface::as_raw(self), csurvivingobjectidranges, objectidrangestart, cobjectidrangelength).ok()
    }
    pub unsafe fn GarbageCollectionFinished(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GarbageCollectionFinished)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn FinalizeableObjectQueued(&self, finalizerflags: u32, objectid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.FinalizeableObjectQueued)(::windows::core::Interface::as_raw(self), finalizerflags, objectid).ok()
    }
    pub unsafe fn RootReferences2(&self, crootrefs: u32, rootrefids: *mut usize, rootkinds: *mut COR_PRF_GC_ROOT_KIND, rootflags: *mut COR_PRF_GC_ROOT_FLAGS, rootids: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.RootReferences2)(::windows::core::Interface::as_raw(self), crootrefs, rootrefids, rootkinds, rootflags, rootids).ok()
    }
    pub unsafe fn HandleCreated(&self, handleid: usize, initialobjectid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.HandleCreated)(::windows::core::Interface::as_raw(self), handleid, initialobjectid).ok()
    }
    pub unsafe fn HandleDestroyed(&self, handleid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.HandleDestroyed)(::windows::core::Interface::as_raw(self), handleid).ok()
    }
    pub unsafe fn InitializeForAttach<P0>(&self, pcorprofilerinfounk: P0, pvclientdata: *mut ::core::ffi::c_void, cbclientdata: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).InitializeForAttach)(::windows::core::Interface::as_raw(self), pcorprofilerinfounk.into_param().abi(), pvclientdata, cbclientdata).ok()
    }
    pub unsafe fn ProfilerAttachComplete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ProfilerAttachComplete)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ProfilerDetachSucceeded(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ProfilerDetachSucceeded)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(ICorProfilerCallback3, ::windows::core::IUnknown, ICorProfilerCallback, ICorProfilerCallback2);
impl ::core::cmp::PartialEq for ICorProfilerCallback3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICorProfilerCallback3 {}
impl ::core::fmt::Debug for ICorProfilerCallback3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICorProfilerCallback3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICorProfilerCallback3 {
    type Vtable = ICorProfilerCallback3_Vtbl;
}
impl ::core::clone::Clone for ICorProfilerCallback3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICorProfilerCallback3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4fd2ed52_7731_4b8d_9469_03d2cc3086c5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorProfilerCallback3_Vtbl {
    pub base__: ICorProfilerCallback2_Vtbl,
    pub InitializeForAttach: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcorprofilerinfounk: *mut ::core::ffi::c_void, pvclientdata: *mut ::core::ffi::c_void, cbclientdata: u32) -> ::windows::core::HRESULT,
    pub ProfilerAttachComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ProfilerDetachSucceeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
#[repr(transparent)]
pub struct ICorProfilerCallback4(::windows::core::IUnknown);
impl ICorProfilerCallback4 {
    pub unsafe fn Initialize<P0>(&self, picorprofilerinfounk: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.Initialize)(::windows::core::Interface::as_raw(self), picorprofilerinfounk.into_param().abi()).ok()
    }
    pub unsafe fn Shutdown(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.Shutdown)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn AppDomainCreationStarted(&self, appdomainid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.AppDomainCreationStarted)(::windows::core::Interface::as_raw(self), appdomainid).ok()
    }
    pub unsafe fn AppDomainCreationFinished(&self, appdomainid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.AppDomainCreationFinished)(::windows::core::Interface::as_raw(self), appdomainid, hrstatus).ok()
    }
    pub unsafe fn AppDomainShutdownStarted(&self, appdomainid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.AppDomainShutdownStarted)(::windows::core::Interface::as_raw(self), appdomainid).ok()
    }
    pub unsafe fn AppDomainShutdownFinished(&self, appdomainid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.AppDomainShutdownFinished)(::windows::core::Interface::as_raw(self), appdomainid, hrstatus).ok()
    }
    pub unsafe fn AssemblyLoadStarted(&self, assemblyid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.AssemblyLoadStarted)(::windows::core::Interface::as_raw(self), assemblyid).ok()
    }
    pub unsafe fn AssemblyLoadFinished(&self, assemblyid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.AssemblyLoadFinished)(::windows::core::Interface::as_raw(self), assemblyid, hrstatus).ok()
    }
    pub unsafe fn AssemblyUnloadStarted(&self, assemblyid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.AssemblyUnloadStarted)(::windows::core::Interface::as_raw(self), assemblyid).ok()
    }
    pub unsafe fn AssemblyUnloadFinished(&self, assemblyid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.AssemblyUnloadFinished)(::windows::core::Interface::as_raw(self), assemblyid, hrstatus).ok()
    }
    pub unsafe fn ModuleLoadStarted(&self, moduleid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.ModuleLoadStarted)(::windows::core::Interface::as_raw(self), moduleid).ok()
    }
    pub unsafe fn ModuleLoadFinished(&self, moduleid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.ModuleLoadFinished)(::windows::core::Interface::as_raw(self), moduleid, hrstatus).ok()
    }
    pub unsafe fn ModuleUnloadStarted(&self, moduleid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.ModuleUnloadStarted)(::windows::core::Interface::as_raw(self), moduleid).ok()
    }
    pub unsafe fn ModuleUnloadFinished(&self, moduleid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.ModuleUnloadFinished)(::windows::core::Interface::as_raw(self), moduleid, hrstatus).ok()
    }
    pub unsafe fn ModuleAttachedToAssembly(&self, moduleid: usize, assemblyid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.ModuleAttachedToAssembly)(::windows::core::Interface::as_raw(self), moduleid, assemblyid).ok()
    }
    pub unsafe fn ClassLoadStarted(&self, classid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.ClassLoadStarted)(::windows::core::Interface::as_raw(self), classid).ok()
    }
    pub unsafe fn ClassLoadFinished(&self, classid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.ClassLoadFinished)(::windows::core::Interface::as_raw(self), classid, hrstatus).ok()
    }
    pub unsafe fn ClassUnloadStarted(&self, classid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.ClassUnloadStarted)(::windows::core::Interface::as_raw(self), classid).ok()
    }
    pub unsafe fn ClassUnloadFinished(&self, classid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.ClassUnloadFinished)(::windows::core::Interface::as_raw(self), classid, hrstatus).ok()
    }
    pub unsafe fn FunctionUnloadStarted(&self, functionid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.FunctionUnloadStarted)(::windows::core::Interface::as_raw(self), functionid).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn JITCompilationStarted<P0>(&self, functionid: usize, fissafetoblock: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.JITCompilationStarted)(::windows::core::Interface::as_raw(self), functionid, fissafetoblock.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn JITCompilationFinished<P0>(&self, functionid: usize, hrstatus: ::windows::core::HRESULT, fissafetoblock: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.JITCompilationFinished)(::windows::core::Interface::as_raw(self), functionid, hrstatus, fissafetoblock.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn JITCachedFunctionSearchStarted(&self, functionid: usize, pbusecachedfunction: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.JITCachedFunctionSearchStarted)(::windows::core::Interface::as_raw(self), functionid, pbusecachedfunction).ok()
    }
    pub unsafe fn JITCachedFunctionSearchFinished(&self, functionid: usize, result: COR_PRF_JIT_CACHE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.JITCachedFunctionSearchFinished)(::windows::core::Interface::as_raw(self), functionid, result).ok()
    }
    pub unsafe fn JITFunctionPitched(&self, functionid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.JITFunctionPitched)(::windows::core::Interface::as_raw(self), functionid).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn JITInlining(&self, callerid: usize, calleeid: usize, pfshouldinline: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.JITInlining)(::windows::core::Interface::as_raw(self), callerid, calleeid, pfshouldinline).ok()
    }
    pub unsafe fn ThreadCreated(&self, threadid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.ThreadCreated)(::windows::core::Interface::as_raw(self), threadid).ok()
    }
    pub unsafe fn ThreadDestroyed(&self, threadid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.ThreadDestroyed)(::windows::core::Interface::as_raw(self), threadid).ok()
    }
    pub unsafe fn ThreadAssignedToOSThread(&self, managedthreadid: usize, osthreadid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.ThreadAssignedToOSThread)(::windows::core::Interface::as_raw(self), managedthreadid, osthreadid).ok()
    }
    pub unsafe fn RemotingClientInvocationStarted(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.RemotingClientInvocationStarted)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemotingClientSendingMessage<P0>(&self, pcookie: *mut ::windows::core::GUID, fisasync: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.RemotingClientSendingMessage)(::windows::core::Interface::as_raw(self), pcookie, fisasync.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemotingClientReceivingReply<P0>(&self, pcookie: *mut ::windows::core::GUID, fisasync: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.RemotingClientReceivingReply)(::windows::core::Interface::as_raw(self), pcookie, fisasync.into_param().abi()).ok()
    }
    pub unsafe fn RemotingClientInvocationFinished(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.RemotingClientInvocationFinished)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemotingServerReceivingMessage<P0>(&self, pcookie: *mut ::windows::core::GUID, fisasync: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.RemotingServerReceivingMessage)(::windows::core::Interface::as_raw(self), pcookie, fisasync.into_param().abi()).ok()
    }
    pub unsafe fn RemotingServerInvocationStarted(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.RemotingServerInvocationStarted)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RemotingServerInvocationReturned(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.RemotingServerInvocationReturned)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemotingServerSendingReply<P0>(&self, pcookie: *mut ::windows::core::GUID, fisasync: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.RemotingServerSendingReply)(::windows::core::Interface::as_raw(self), pcookie, fisasync.into_param().abi()).ok()
    }
    pub unsafe fn UnmanagedToManagedTransition(&self, functionid: usize, reason: COR_PRF_TRANSITION_REASON) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.UnmanagedToManagedTransition)(::windows::core::Interface::as_raw(self), functionid, reason).ok()
    }
    pub unsafe fn ManagedToUnmanagedTransition(&self, functionid: usize, reason: COR_PRF_TRANSITION_REASON) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.ManagedToUnmanagedTransition)(::windows::core::Interface::as_raw(self), functionid, reason).ok()
    }
    pub unsafe fn RuntimeSuspendStarted(&self, suspendreason: COR_PRF_SUSPEND_REASON) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.RuntimeSuspendStarted)(::windows::core::Interface::as_raw(self), suspendreason).ok()
    }
    pub unsafe fn RuntimeSuspendFinished(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.RuntimeSuspendFinished)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RuntimeSuspendAborted(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.RuntimeSuspendAborted)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RuntimeResumeStarted(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.RuntimeResumeStarted)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RuntimeResumeFinished(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.RuntimeResumeFinished)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RuntimeThreadSuspended(&self, threadid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.RuntimeThreadSuspended)(::windows::core::Interface::as_raw(self), threadid).ok()
    }
    pub unsafe fn RuntimeThreadResumed(&self, threadid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.RuntimeThreadResumed)(::windows::core::Interface::as_raw(self), threadid).ok()
    }
    pub unsafe fn MovedReferences(&self, cmovedobjectidranges: u32, oldobjectidrangestart: *mut usize, newobjectidrangestart: *mut usize, cobjectidrangelength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.MovedReferences)(::windows::core::Interface::as_raw(self), cmovedobjectidranges, oldobjectidrangestart, newobjectidrangestart, cobjectidrangelength).ok()
    }
    pub unsafe fn ObjectAllocated(&self, objectid: usize, classid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.ObjectAllocated)(::windows::core::Interface::as_raw(self), objectid, classid).ok()
    }
    pub unsafe fn ObjectsAllocatedByClass(&self, cclasscount: u32, classids: *mut usize, cobjects: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.ObjectsAllocatedByClass)(::windows::core::Interface::as_raw(self), cclasscount, classids, cobjects).ok()
    }
    pub unsafe fn ObjectReferences(&self, objectid: usize, classid: usize, cobjectrefs: u32, objectrefids: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.ObjectReferences)(::windows::core::Interface::as_raw(self), objectid, classid, cobjectrefs, objectrefids).ok()
    }
    pub unsafe fn RootReferences(&self, crootrefs: u32, rootrefids: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.RootReferences)(::windows::core::Interface::as_raw(self), crootrefs, rootrefids).ok()
    }
    pub unsafe fn ExceptionThrown(&self, thrownobjectid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.ExceptionThrown)(::windows::core::Interface::as_raw(self), thrownobjectid).ok()
    }
    pub unsafe fn ExceptionSearchFunctionEnter(&self, functionid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.ExceptionSearchFunctionEnter)(::windows::core::Interface::as_raw(self), functionid).ok()
    }
    pub unsafe fn ExceptionSearchFunctionLeave(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.ExceptionSearchFunctionLeave)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ExceptionSearchFilterEnter(&self, functionid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.ExceptionSearchFilterEnter)(::windows::core::Interface::as_raw(self), functionid).ok()
    }
    pub unsafe fn ExceptionSearchFilterLeave(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.ExceptionSearchFilterLeave)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ExceptionSearchCatcherFound(&self, functionid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.ExceptionSearchCatcherFound)(::windows::core::Interface::as_raw(self), functionid).ok()
    }
    pub unsafe fn ExceptionOSHandlerEnter(&self, __unused: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.ExceptionOSHandlerEnter)(::windows::core::Interface::as_raw(self), __unused).ok()
    }
    pub unsafe fn ExceptionOSHandlerLeave(&self, __unused: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.ExceptionOSHandlerLeave)(::windows::core::Interface::as_raw(self), __unused).ok()
    }
    pub unsafe fn ExceptionUnwindFunctionEnter(&self, functionid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.ExceptionUnwindFunctionEnter)(::windows::core::Interface::as_raw(self), functionid).ok()
    }
    pub unsafe fn ExceptionUnwindFunctionLeave(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.ExceptionUnwindFunctionLeave)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ExceptionUnwindFinallyEnter(&self, functionid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.ExceptionUnwindFinallyEnter)(::windows::core::Interface::as_raw(self), functionid).ok()
    }
    pub unsafe fn ExceptionUnwindFinallyLeave(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.ExceptionUnwindFinallyLeave)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ExceptionCatcherEnter(&self, functionid: usize, objectid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.ExceptionCatcherEnter)(::windows::core::Interface::as_raw(self), functionid, objectid).ok()
    }
    pub unsafe fn ExceptionCatcherLeave(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.ExceptionCatcherLeave)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn COMClassicVTableCreated(&self, wrappedclassid: usize, implementediid: *const ::windows::core::GUID, pvtable: *mut ::core::ffi::c_void, cslots: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.COMClassicVTableCreated)(::windows::core::Interface::as_raw(self), wrappedclassid, implementediid, pvtable, cslots).ok()
    }
    pub unsafe fn COMClassicVTableDestroyed(&self, wrappedclassid: usize, implementediid: *const ::windows::core::GUID, pvtable: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.COMClassicVTableDestroyed)(::windows::core::Interface::as_raw(self), wrappedclassid, implementediid, pvtable).ok()
    }
    pub unsafe fn ExceptionCLRCatcherFound(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.ExceptionCLRCatcherFound)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ExceptionCLRCatcherExecute(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.ExceptionCLRCatcherExecute)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ThreadNameChanged(&self, threadid: usize, name: ::core::option::Option<&[u16]>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.ThreadNameChanged)(::windows::core::Interface::as_raw(self), threadid, name.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(name.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GarbageCollectionStarted(&self, cgenerations: i32, generationcollected: *mut super::super::super::Foundation::BOOL, reason: COR_PRF_GC_REASON) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GarbageCollectionStarted)(::windows::core::Interface::as_raw(self), cgenerations, generationcollected, reason).ok()
    }
    pub unsafe fn SurvivingReferences(&self, csurvivingobjectidranges: u32, objectidrangestart: *mut usize, cobjectidrangelength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SurvivingReferences)(::windows::core::Interface::as_raw(self), csurvivingobjectidranges, objectidrangestart, cobjectidrangelength).ok()
    }
    pub unsafe fn GarbageCollectionFinished(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GarbageCollectionFinished)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn FinalizeableObjectQueued(&self, finalizerflags: u32, objectid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.FinalizeableObjectQueued)(::windows::core::Interface::as_raw(self), finalizerflags, objectid).ok()
    }
    pub unsafe fn RootReferences2(&self, crootrefs: u32, rootrefids: *mut usize, rootkinds: *mut COR_PRF_GC_ROOT_KIND, rootflags: *mut COR_PRF_GC_ROOT_FLAGS, rootids: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.RootReferences2)(::windows::core::Interface::as_raw(self), crootrefs, rootrefids, rootkinds, rootflags, rootids).ok()
    }
    pub unsafe fn HandleCreated(&self, handleid: usize, initialobjectid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.HandleCreated)(::windows::core::Interface::as_raw(self), handleid, initialobjectid).ok()
    }
    pub unsafe fn HandleDestroyed(&self, handleid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.HandleDestroyed)(::windows::core::Interface::as_raw(self), handleid).ok()
    }
    pub unsafe fn InitializeForAttach<P0>(&self, pcorprofilerinfounk: P0, pvclientdata: *mut ::core::ffi::c_void, cbclientdata: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.InitializeForAttach)(::windows::core::Interface::as_raw(self), pcorprofilerinfounk.into_param().abi(), pvclientdata, cbclientdata).ok()
    }
    pub unsafe fn ProfilerAttachComplete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ProfilerAttachComplete)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ProfilerDetachSucceeded(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ProfilerDetachSucceeded)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReJITCompilationStarted<P0>(&self, functionid: usize, rejitid: usize, fissafetoblock: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).ReJITCompilationStarted)(::windows::core::Interface::as_raw(self), functionid, rejitid, fissafetoblock.into_param().abi()).ok()
    }
    pub unsafe fn GetReJITParameters<P0>(&self, moduleid: usize, methodid: u32, pfunctioncontrol: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ICorProfilerFunctionControl>,
    {
        (::windows::core::Interface::vtable(self).GetReJITParameters)(::windows::core::Interface::as_raw(self), moduleid, methodid, pfunctioncontrol.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReJITCompilationFinished<P0>(&self, functionid: usize, rejitid: usize, hrstatus: ::windows::core::HRESULT, fissafetoblock: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).ReJITCompilationFinished)(::windows::core::Interface::as_raw(self), functionid, rejitid, hrstatus, fissafetoblock.into_param().abi()).ok()
    }
    pub unsafe fn ReJITError(&self, moduleid: usize, methodid: u32, functionid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReJITError)(::windows::core::Interface::as_raw(self), moduleid, methodid, functionid, hrstatus).ok()
    }
    pub unsafe fn MovedReferences2(&self, cmovedobjectidranges: u32, oldobjectidrangestart: *mut usize, newobjectidrangestart: *mut usize, cobjectidrangelength: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).MovedReferences2)(::windows::core::Interface::as_raw(self), cmovedobjectidranges, oldobjectidrangestart, newobjectidrangestart, cobjectidrangelength).ok()
    }
    pub unsafe fn SurvivingReferences2(&self, csurvivingobjectidranges: u32, objectidrangestart: *mut usize, cobjectidrangelength: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SurvivingReferences2)(::windows::core::Interface::as_raw(self), csurvivingobjectidranges, objectidrangestart, cobjectidrangelength).ok()
    }
}
::windows::imp::interface_hierarchy!(ICorProfilerCallback4, ::windows::core::IUnknown, ICorProfilerCallback, ICorProfilerCallback2, ICorProfilerCallback3);
impl ::core::cmp::PartialEq for ICorProfilerCallback4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICorProfilerCallback4 {}
impl ::core::fmt::Debug for ICorProfilerCallback4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICorProfilerCallback4").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICorProfilerCallback4 {
    type Vtable = ICorProfilerCallback4_Vtbl;
}
impl ::core::clone::Clone for ICorProfilerCallback4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICorProfilerCallback4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7b63b2e3_107d_4d48_b2f6_f61e229470d2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorProfilerCallback4_Vtbl {
    pub base__: ICorProfilerCallback3_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub ReJITCompilationStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, functionid: usize, rejitid: usize, fissafetoblock: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReJITCompilationStarted: usize,
    pub GetReJITParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduleid: usize, methodid: u32, pfunctioncontrol: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ReJITCompilationFinished: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, functionid: usize, rejitid: usize, hrstatus: ::windows::core::HRESULT, fissafetoblock: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReJITCompilationFinished: usize,
    pub ReJITError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduleid: usize, methodid: u32, functionid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub MovedReferences2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cmovedobjectidranges: u32, oldobjectidrangestart: *mut usize, newobjectidrangestart: *mut usize, cobjectidrangelength: *mut usize) -> ::windows::core::HRESULT,
    pub SurvivingReferences2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, csurvivingobjectidranges: u32, objectidrangestart: *mut usize, cobjectidrangelength: *mut usize) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
#[repr(transparent)]
pub struct ICorProfilerCallback5(::windows::core::IUnknown);
impl ICorProfilerCallback5 {
    pub unsafe fn Initialize<P0>(&self, picorprofilerinfounk: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.Initialize)(::windows::core::Interface::as_raw(self), picorprofilerinfounk.into_param().abi()).ok()
    }
    pub unsafe fn Shutdown(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.Shutdown)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn AppDomainCreationStarted(&self, appdomainid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.AppDomainCreationStarted)(::windows::core::Interface::as_raw(self), appdomainid).ok()
    }
    pub unsafe fn AppDomainCreationFinished(&self, appdomainid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.AppDomainCreationFinished)(::windows::core::Interface::as_raw(self), appdomainid, hrstatus).ok()
    }
    pub unsafe fn AppDomainShutdownStarted(&self, appdomainid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.AppDomainShutdownStarted)(::windows::core::Interface::as_raw(self), appdomainid).ok()
    }
    pub unsafe fn AppDomainShutdownFinished(&self, appdomainid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.AppDomainShutdownFinished)(::windows::core::Interface::as_raw(self), appdomainid, hrstatus).ok()
    }
    pub unsafe fn AssemblyLoadStarted(&self, assemblyid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.AssemblyLoadStarted)(::windows::core::Interface::as_raw(self), assemblyid).ok()
    }
    pub unsafe fn AssemblyLoadFinished(&self, assemblyid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.AssemblyLoadFinished)(::windows::core::Interface::as_raw(self), assemblyid, hrstatus).ok()
    }
    pub unsafe fn AssemblyUnloadStarted(&self, assemblyid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.AssemblyUnloadStarted)(::windows::core::Interface::as_raw(self), assemblyid).ok()
    }
    pub unsafe fn AssemblyUnloadFinished(&self, assemblyid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.AssemblyUnloadFinished)(::windows::core::Interface::as_raw(self), assemblyid, hrstatus).ok()
    }
    pub unsafe fn ModuleLoadStarted(&self, moduleid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.ModuleLoadStarted)(::windows::core::Interface::as_raw(self), moduleid).ok()
    }
    pub unsafe fn ModuleLoadFinished(&self, moduleid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.ModuleLoadFinished)(::windows::core::Interface::as_raw(self), moduleid, hrstatus).ok()
    }
    pub unsafe fn ModuleUnloadStarted(&self, moduleid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.ModuleUnloadStarted)(::windows::core::Interface::as_raw(self), moduleid).ok()
    }
    pub unsafe fn ModuleUnloadFinished(&self, moduleid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.ModuleUnloadFinished)(::windows::core::Interface::as_raw(self), moduleid, hrstatus).ok()
    }
    pub unsafe fn ModuleAttachedToAssembly(&self, moduleid: usize, assemblyid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.ModuleAttachedToAssembly)(::windows::core::Interface::as_raw(self), moduleid, assemblyid).ok()
    }
    pub unsafe fn ClassLoadStarted(&self, classid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.ClassLoadStarted)(::windows::core::Interface::as_raw(self), classid).ok()
    }
    pub unsafe fn ClassLoadFinished(&self, classid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.ClassLoadFinished)(::windows::core::Interface::as_raw(self), classid, hrstatus).ok()
    }
    pub unsafe fn ClassUnloadStarted(&self, classid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.ClassUnloadStarted)(::windows::core::Interface::as_raw(self), classid).ok()
    }
    pub unsafe fn ClassUnloadFinished(&self, classid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.ClassUnloadFinished)(::windows::core::Interface::as_raw(self), classid, hrstatus).ok()
    }
    pub unsafe fn FunctionUnloadStarted(&self, functionid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.FunctionUnloadStarted)(::windows::core::Interface::as_raw(self), functionid).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn JITCompilationStarted<P0>(&self, functionid: usize, fissafetoblock: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.JITCompilationStarted)(::windows::core::Interface::as_raw(self), functionid, fissafetoblock.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn JITCompilationFinished<P0>(&self, functionid: usize, hrstatus: ::windows::core::HRESULT, fissafetoblock: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.JITCompilationFinished)(::windows::core::Interface::as_raw(self), functionid, hrstatus, fissafetoblock.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn JITCachedFunctionSearchStarted(&self, functionid: usize, pbusecachedfunction: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.JITCachedFunctionSearchStarted)(::windows::core::Interface::as_raw(self), functionid, pbusecachedfunction).ok()
    }
    pub unsafe fn JITCachedFunctionSearchFinished(&self, functionid: usize, result: COR_PRF_JIT_CACHE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.JITCachedFunctionSearchFinished)(::windows::core::Interface::as_raw(self), functionid, result).ok()
    }
    pub unsafe fn JITFunctionPitched(&self, functionid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.JITFunctionPitched)(::windows::core::Interface::as_raw(self), functionid).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn JITInlining(&self, callerid: usize, calleeid: usize, pfshouldinline: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.JITInlining)(::windows::core::Interface::as_raw(self), callerid, calleeid, pfshouldinline).ok()
    }
    pub unsafe fn ThreadCreated(&self, threadid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.ThreadCreated)(::windows::core::Interface::as_raw(self), threadid).ok()
    }
    pub unsafe fn ThreadDestroyed(&self, threadid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.ThreadDestroyed)(::windows::core::Interface::as_raw(self), threadid).ok()
    }
    pub unsafe fn ThreadAssignedToOSThread(&self, managedthreadid: usize, osthreadid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.ThreadAssignedToOSThread)(::windows::core::Interface::as_raw(self), managedthreadid, osthreadid).ok()
    }
    pub unsafe fn RemotingClientInvocationStarted(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.RemotingClientInvocationStarted)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemotingClientSendingMessage<P0>(&self, pcookie: *mut ::windows::core::GUID, fisasync: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.RemotingClientSendingMessage)(::windows::core::Interface::as_raw(self), pcookie, fisasync.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemotingClientReceivingReply<P0>(&self, pcookie: *mut ::windows::core::GUID, fisasync: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.RemotingClientReceivingReply)(::windows::core::Interface::as_raw(self), pcookie, fisasync.into_param().abi()).ok()
    }
    pub unsafe fn RemotingClientInvocationFinished(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.RemotingClientInvocationFinished)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemotingServerReceivingMessage<P0>(&self, pcookie: *mut ::windows::core::GUID, fisasync: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.RemotingServerReceivingMessage)(::windows::core::Interface::as_raw(self), pcookie, fisasync.into_param().abi()).ok()
    }
    pub unsafe fn RemotingServerInvocationStarted(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.RemotingServerInvocationStarted)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RemotingServerInvocationReturned(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.RemotingServerInvocationReturned)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemotingServerSendingReply<P0>(&self, pcookie: *mut ::windows::core::GUID, fisasync: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.RemotingServerSendingReply)(::windows::core::Interface::as_raw(self), pcookie, fisasync.into_param().abi()).ok()
    }
    pub unsafe fn UnmanagedToManagedTransition(&self, functionid: usize, reason: COR_PRF_TRANSITION_REASON) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.UnmanagedToManagedTransition)(::windows::core::Interface::as_raw(self), functionid, reason).ok()
    }
    pub unsafe fn ManagedToUnmanagedTransition(&self, functionid: usize, reason: COR_PRF_TRANSITION_REASON) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.ManagedToUnmanagedTransition)(::windows::core::Interface::as_raw(self), functionid, reason).ok()
    }
    pub unsafe fn RuntimeSuspendStarted(&self, suspendreason: COR_PRF_SUSPEND_REASON) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.RuntimeSuspendStarted)(::windows::core::Interface::as_raw(self), suspendreason).ok()
    }
    pub unsafe fn RuntimeSuspendFinished(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.RuntimeSuspendFinished)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RuntimeSuspendAborted(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.RuntimeSuspendAborted)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RuntimeResumeStarted(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.RuntimeResumeStarted)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RuntimeResumeFinished(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.RuntimeResumeFinished)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RuntimeThreadSuspended(&self, threadid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.RuntimeThreadSuspended)(::windows::core::Interface::as_raw(self), threadid).ok()
    }
    pub unsafe fn RuntimeThreadResumed(&self, threadid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.RuntimeThreadResumed)(::windows::core::Interface::as_raw(self), threadid).ok()
    }
    pub unsafe fn MovedReferences(&self, cmovedobjectidranges: u32, oldobjectidrangestart: *mut usize, newobjectidrangestart: *mut usize, cobjectidrangelength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.MovedReferences)(::windows::core::Interface::as_raw(self), cmovedobjectidranges, oldobjectidrangestart, newobjectidrangestart, cobjectidrangelength).ok()
    }
    pub unsafe fn ObjectAllocated(&self, objectid: usize, classid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.ObjectAllocated)(::windows::core::Interface::as_raw(self), objectid, classid).ok()
    }
    pub unsafe fn ObjectsAllocatedByClass(&self, cclasscount: u32, classids: *mut usize, cobjects: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.ObjectsAllocatedByClass)(::windows::core::Interface::as_raw(self), cclasscount, classids, cobjects).ok()
    }
    pub unsafe fn ObjectReferences(&self, objectid: usize, classid: usize, cobjectrefs: u32, objectrefids: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.ObjectReferences)(::windows::core::Interface::as_raw(self), objectid, classid, cobjectrefs, objectrefids).ok()
    }
    pub unsafe fn RootReferences(&self, crootrefs: u32, rootrefids: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.RootReferences)(::windows::core::Interface::as_raw(self), crootrefs, rootrefids).ok()
    }
    pub unsafe fn ExceptionThrown(&self, thrownobjectid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.ExceptionThrown)(::windows::core::Interface::as_raw(self), thrownobjectid).ok()
    }
    pub unsafe fn ExceptionSearchFunctionEnter(&self, functionid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.ExceptionSearchFunctionEnter)(::windows::core::Interface::as_raw(self), functionid).ok()
    }
    pub unsafe fn ExceptionSearchFunctionLeave(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.ExceptionSearchFunctionLeave)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ExceptionSearchFilterEnter(&self, functionid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.ExceptionSearchFilterEnter)(::windows::core::Interface::as_raw(self), functionid).ok()
    }
    pub unsafe fn ExceptionSearchFilterLeave(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.ExceptionSearchFilterLeave)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ExceptionSearchCatcherFound(&self, functionid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.ExceptionSearchCatcherFound)(::windows::core::Interface::as_raw(self), functionid).ok()
    }
    pub unsafe fn ExceptionOSHandlerEnter(&self, __unused: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.ExceptionOSHandlerEnter)(::windows::core::Interface::as_raw(self), __unused).ok()
    }
    pub unsafe fn ExceptionOSHandlerLeave(&self, __unused: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.ExceptionOSHandlerLeave)(::windows::core::Interface::as_raw(self), __unused).ok()
    }
    pub unsafe fn ExceptionUnwindFunctionEnter(&self, functionid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.ExceptionUnwindFunctionEnter)(::windows::core::Interface::as_raw(self), functionid).ok()
    }
    pub unsafe fn ExceptionUnwindFunctionLeave(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.ExceptionUnwindFunctionLeave)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ExceptionUnwindFinallyEnter(&self, functionid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.ExceptionUnwindFinallyEnter)(::windows::core::Interface::as_raw(self), functionid).ok()
    }
    pub unsafe fn ExceptionUnwindFinallyLeave(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.ExceptionUnwindFinallyLeave)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ExceptionCatcherEnter(&self, functionid: usize, objectid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.ExceptionCatcherEnter)(::windows::core::Interface::as_raw(self), functionid, objectid).ok()
    }
    pub unsafe fn ExceptionCatcherLeave(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.ExceptionCatcherLeave)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn COMClassicVTableCreated(&self, wrappedclassid: usize, implementediid: *const ::windows::core::GUID, pvtable: *mut ::core::ffi::c_void, cslots: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.COMClassicVTableCreated)(::windows::core::Interface::as_raw(self), wrappedclassid, implementediid, pvtable, cslots).ok()
    }
    pub unsafe fn COMClassicVTableDestroyed(&self, wrappedclassid: usize, implementediid: *const ::windows::core::GUID, pvtable: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.COMClassicVTableDestroyed)(::windows::core::Interface::as_raw(self), wrappedclassid, implementediid, pvtable).ok()
    }
    pub unsafe fn ExceptionCLRCatcherFound(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.ExceptionCLRCatcherFound)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ExceptionCLRCatcherExecute(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.ExceptionCLRCatcherExecute)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ThreadNameChanged(&self, threadid: usize, name: ::core::option::Option<&[u16]>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.ThreadNameChanged)(::windows::core::Interface::as_raw(self), threadid, name.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(name.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GarbageCollectionStarted(&self, cgenerations: i32, generationcollected: *mut super::super::super::Foundation::BOOL, reason: COR_PRF_GC_REASON) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GarbageCollectionStarted)(::windows::core::Interface::as_raw(self), cgenerations, generationcollected, reason).ok()
    }
    pub unsafe fn SurvivingReferences(&self, csurvivingobjectidranges: u32, objectidrangestart: *mut usize, cobjectidrangelength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SurvivingReferences)(::windows::core::Interface::as_raw(self), csurvivingobjectidranges, objectidrangestart, cobjectidrangelength).ok()
    }
    pub unsafe fn GarbageCollectionFinished(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GarbageCollectionFinished)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn FinalizeableObjectQueued(&self, finalizerflags: u32, objectid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.FinalizeableObjectQueued)(::windows::core::Interface::as_raw(self), finalizerflags, objectid).ok()
    }
    pub unsafe fn RootReferences2(&self, crootrefs: u32, rootrefids: *mut usize, rootkinds: *mut COR_PRF_GC_ROOT_KIND, rootflags: *mut COR_PRF_GC_ROOT_FLAGS, rootids: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.RootReferences2)(::windows::core::Interface::as_raw(self), crootrefs, rootrefids, rootkinds, rootflags, rootids).ok()
    }
    pub unsafe fn HandleCreated(&self, handleid: usize, initialobjectid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.HandleCreated)(::windows::core::Interface::as_raw(self), handleid, initialobjectid).ok()
    }
    pub unsafe fn HandleDestroyed(&self, handleid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.HandleDestroyed)(::windows::core::Interface::as_raw(self), handleid).ok()
    }
    pub unsafe fn InitializeForAttach<P0>(&self, pcorprofilerinfounk: P0, pvclientdata: *mut ::core::ffi::c_void, cbclientdata: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.InitializeForAttach)(::windows::core::Interface::as_raw(self), pcorprofilerinfounk.into_param().abi(), pvclientdata, cbclientdata).ok()
    }
    pub unsafe fn ProfilerAttachComplete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.ProfilerAttachComplete)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ProfilerDetachSucceeded(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.ProfilerDetachSucceeded)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReJITCompilationStarted<P0>(&self, functionid: usize, rejitid: usize, fissafetoblock: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.ReJITCompilationStarted)(::windows::core::Interface::as_raw(self), functionid, rejitid, fissafetoblock.into_param().abi()).ok()
    }
    pub unsafe fn GetReJITParameters<P0>(&self, moduleid: usize, methodid: u32, pfunctioncontrol: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ICorProfilerFunctionControl>,
    {
        (::windows::core::Interface::vtable(self).base__.GetReJITParameters)(::windows::core::Interface::as_raw(self), moduleid, methodid, pfunctioncontrol.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReJITCompilationFinished<P0>(&self, functionid: usize, rejitid: usize, hrstatus: ::windows::core::HRESULT, fissafetoblock: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.ReJITCompilationFinished)(::windows::core::Interface::as_raw(self), functionid, rejitid, hrstatus, fissafetoblock.into_param().abi()).ok()
    }
    pub unsafe fn ReJITError(&self, moduleid: usize, methodid: u32, functionid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ReJITError)(::windows::core::Interface::as_raw(self), moduleid, methodid, functionid, hrstatus).ok()
    }
    pub unsafe fn MovedReferences2(&self, cmovedobjectidranges: u32, oldobjectidrangestart: *mut usize, newobjectidrangestart: *mut usize, cobjectidrangelength: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.MovedReferences2)(::windows::core::Interface::as_raw(self), cmovedobjectidranges, oldobjectidrangestart, newobjectidrangestart, cobjectidrangelength).ok()
    }
    pub unsafe fn SurvivingReferences2(&self, csurvivingobjectidranges: u32, objectidrangestart: *mut usize, cobjectidrangelength: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SurvivingReferences2)(::windows::core::Interface::as_raw(self), csurvivingobjectidranges, objectidrangestart, cobjectidrangelength).ok()
    }
    pub unsafe fn ConditionalWeakTableElementReferences(&self, crootrefs: u32, keyrefids: *mut usize, valuerefids: *mut usize, rootids: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ConditionalWeakTableElementReferences)(::windows::core::Interface::as_raw(self), crootrefs, keyrefids, valuerefids, rootids).ok()
    }
}
::windows::imp::interface_hierarchy!(ICorProfilerCallback5, ::windows::core::IUnknown, ICorProfilerCallback, ICorProfilerCallback2, ICorProfilerCallback3, ICorProfilerCallback4);
impl ::core::cmp::PartialEq for ICorProfilerCallback5 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICorProfilerCallback5 {}
impl ::core::fmt::Debug for ICorProfilerCallback5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICorProfilerCallback5").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICorProfilerCallback5 {
    type Vtable = ICorProfilerCallback5_Vtbl;
}
impl ::core::clone::Clone for ICorProfilerCallback5 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICorProfilerCallback5 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8dfba405_8c9f_45f8_bffa_83b14cef78b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorProfilerCallback5_Vtbl {
    pub base__: ICorProfilerCallback4_Vtbl,
    pub ConditionalWeakTableElementReferences: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, crootrefs: u32, keyrefids: *mut usize, valuerefids: *mut usize, rootids: *mut usize) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
#[repr(transparent)]
pub struct ICorProfilerCallback6(::windows::core::IUnknown);
impl ICorProfilerCallback6 {
    pub unsafe fn Initialize<P0>(&self, picorprofilerinfounk: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.Initialize)(::windows::core::Interface::as_raw(self), picorprofilerinfounk.into_param().abi()).ok()
    }
    pub unsafe fn Shutdown(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.Shutdown)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn AppDomainCreationStarted(&self, appdomainid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.AppDomainCreationStarted)(::windows::core::Interface::as_raw(self), appdomainid).ok()
    }
    pub unsafe fn AppDomainCreationFinished(&self, appdomainid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.AppDomainCreationFinished)(::windows::core::Interface::as_raw(self), appdomainid, hrstatus).ok()
    }
    pub unsafe fn AppDomainShutdownStarted(&self, appdomainid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.AppDomainShutdownStarted)(::windows::core::Interface::as_raw(self), appdomainid).ok()
    }
    pub unsafe fn AppDomainShutdownFinished(&self, appdomainid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.AppDomainShutdownFinished)(::windows::core::Interface::as_raw(self), appdomainid, hrstatus).ok()
    }
    pub unsafe fn AssemblyLoadStarted(&self, assemblyid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.AssemblyLoadStarted)(::windows::core::Interface::as_raw(self), assemblyid).ok()
    }
    pub unsafe fn AssemblyLoadFinished(&self, assemblyid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.AssemblyLoadFinished)(::windows::core::Interface::as_raw(self), assemblyid, hrstatus).ok()
    }
    pub unsafe fn AssemblyUnloadStarted(&self, assemblyid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.AssemblyUnloadStarted)(::windows::core::Interface::as_raw(self), assemblyid).ok()
    }
    pub unsafe fn AssemblyUnloadFinished(&self, assemblyid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.AssemblyUnloadFinished)(::windows::core::Interface::as_raw(self), assemblyid, hrstatus).ok()
    }
    pub unsafe fn ModuleLoadStarted(&self, moduleid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.ModuleLoadStarted)(::windows::core::Interface::as_raw(self), moduleid).ok()
    }
    pub unsafe fn ModuleLoadFinished(&self, moduleid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.ModuleLoadFinished)(::windows::core::Interface::as_raw(self), moduleid, hrstatus).ok()
    }
    pub unsafe fn ModuleUnloadStarted(&self, moduleid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.ModuleUnloadStarted)(::windows::core::Interface::as_raw(self), moduleid).ok()
    }
    pub unsafe fn ModuleUnloadFinished(&self, moduleid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.ModuleUnloadFinished)(::windows::core::Interface::as_raw(self), moduleid, hrstatus).ok()
    }
    pub unsafe fn ModuleAttachedToAssembly(&self, moduleid: usize, assemblyid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.ModuleAttachedToAssembly)(::windows::core::Interface::as_raw(self), moduleid, assemblyid).ok()
    }
    pub unsafe fn ClassLoadStarted(&self, classid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.ClassLoadStarted)(::windows::core::Interface::as_raw(self), classid).ok()
    }
    pub unsafe fn ClassLoadFinished(&self, classid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.ClassLoadFinished)(::windows::core::Interface::as_raw(self), classid, hrstatus).ok()
    }
    pub unsafe fn ClassUnloadStarted(&self, classid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.ClassUnloadStarted)(::windows::core::Interface::as_raw(self), classid).ok()
    }
    pub unsafe fn ClassUnloadFinished(&self, classid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.ClassUnloadFinished)(::windows::core::Interface::as_raw(self), classid, hrstatus).ok()
    }
    pub unsafe fn FunctionUnloadStarted(&self, functionid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.FunctionUnloadStarted)(::windows::core::Interface::as_raw(self), functionid).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn JITCompilationStarted<P0>(&self, functionid: usize, fissafetoblock: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.JITCompilationStarted)(::windows::core::Interface::as_raw(self), functionid, fissafetoblock.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn JITCompilationFinished<P0>(&self, functionid: usize, hrstatus: ::windows::core::HRESULT, fissafetoblock: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.JITCompilationFinished)(::windows::core::Interface::as_raw(self), functionid, hrstatus, fissafetoblock.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn JITCachedFunctionSearchStarted(&self, functionid: usize, pbusecachedfunction: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.JITCachedFunctionSearchStarted)(::windows::core::Interface::as_raw(self), functionid, pbusecachedfunction).ok()
    }
    pub unsafe fn JITCachedFunctionSearchFinished(&self, functionid: usize, result: COR_PRF_JIT_CACHE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.JITCachedFunctionSearchFinished)(::windows::core::Interface::as_raw(self), functionid, result).ok()
    }
    pub unsafe fn JITFunctionPitched(&self, functionid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.JITFunctionPitched)(::windows::core::Interface::as_raw(self), functionid).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn JITInlining(&self, callerid: usize, calleeid: usize, pfshouldinline: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.JITInlining)(::windows::core::Interface::as_raw(self), callerid, calleeid, pfshouldinline).ok()
    }
    pub unsafe fn ThreadCreated(&self, threadid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.ThreadCreated)(::windows::core::Interface::as_raw(self), threadid).ok()
    }
    pub unsafe fn ThreadDestroyed(&self, threadid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.ThreadDestroyed)(::windows::core::Interface::as_raw(self), threadid).ok()
    }
    pub unsafe fn ThreadAssignedToOSThread(&self, managedthreadid: usize, osthreadid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.ThreadAssignedToOSThread)(::windows::core::Interface::as_raw(self), managedthreadid, osthreadid).ok()
    }
    pub unsafe fn RemotingClientInvocationStarted(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.RemotingClientInvocationStarted)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemotingClientSendingMessage<P0>(&self, pcookie: *mut ::windows::core::GUID, fisasync: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.RemotingClientSendingMessage)(::windows::core::Interface::as_raw(self), pcookie, fisasync.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemotingClientReceivingReply<P0>(&self, pcookie: *mut ::windows::core::GUID, fisasync: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.RemotingClientReceivingReply)(::windows::core::Interface::as_raw(self), pcookie, fisasync.into_param().abi()).ok()
    }
    pub unsafe fn RemotingClientInvocationFinished(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.RemotingClientInvocationFinished)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemotingServerReceivingMessage<P0>(&self, pcookie: *mut ::windows::core::GUID, fisasync: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.RemotingServerReceivingMessage)(::windows::core::Interface::as_raw(self), pcookie, fisasync.into_param().abi()).ok()
    }
    pub unsafe fn RemotingServerInvocationStarted(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.RemotingServerInvocationStarted)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RemotingServerInvocationReturned(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.RemotingServerInvocationReturned)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemotingServerSendingReply<P0>(&self, pcookie: *mut ::windows::core::GUID, fisasync: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.RemotingServerSendingReply)(::windows::core::Interface::as_raw(self), pcookie, fisasync.into_param().abi()).ok()
    }
    pub unsafe fn UnmanagedToManagedTransition(&self, functionid: usize, reason: COR_PRF_TRANSITION_REASON) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.UnmanagedToManagedTransition)(::windows::core::Interface::as_raw(self), functionid, reason).ok()
    }
    pub unsafe fn ManagedToUnmanagedTransition(&self, functionid: usize, reason: COR_PRF_TRANSITION_REASON) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.ManagedToUnmanagedTransition)(::windows::core::Interface::as_raw(self), functionid, reason).ok()
    }
    pub unsafe fn RuntimeSuspendStarted(&self, suspendreason: COR_PRF_SUSPEND_REASON) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.RuntimeSuspendStarted)(::windows::core::Interface::as_raw(self), suspendreason).ok()
    }
    pub unsafe fn RuntimeSuspendFinished(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.RuntimeSuspendFinished)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RuntimeSuspendAborted(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.RuntimeSuspendAborted)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RuntimeResumeStarted(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.RuntimeResumeStarted)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RuntimeResumeFinished(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.RuntimeResumeFinished)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RuntimeThreadSuspended(&self, threadid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.RuntimeThreadSuspended)(::windows::core::Interface::as_raw(self), threadid).ok()
    }
    pub unsafe fn RuntimeThreadResumed(&self, threadid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.RuntimeThreadResumed)(::windows::core::Interface::as_raw(self), threadid).ok()
    }
    pub unsafe fn MovedReferences(&self, cmovedobjectidranges: u32, oldobjectidrangestart: *mut usize, newobjectidrangestart: *mut usize, cobjectidrangelength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.MovedReferences)(::windows::core::Interface::as_raw(self), cmovedobjectidranges, oldobjectidrangestart, newobjectidrangestart, cobjectidrangelength).ok()
    }
    pub unsafe fn ObjectAllocated(&self, objectid: usize, classid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.ObjectAllocated)(::windows::core::Interface::as_raw(self), objectid, classid).ok()
    }
    pub unsafe fn ObjectsAllocatedByClass(&self, cclasscount: u32, classids: *mut usize, cobjects: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.ObjectsAllocatedByClass)(::windows::core::Interface::as_raw(self), cclasscount, classids, cobjects).ok()
    }
    pub unsafe fn ObjectReferences(&self, objectid: usize, classid: usize, cobjectrefs: u32, objectrefids: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.ObjectReferences)(::windows::core::Interface::as_raw(self), objectid, classid, cobjectrefs, objectrefids).ok()
    }
    pub unsafe fn RootReferences(&self, crootrefs: u32, rootrefids: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.RootReferences)(::windows::core::Interface::as_raw(self), crootrefs, rootrefids).ok()
    }
    pub unsafe fn ExceptionThrown(&self, thrownobjectid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.ExceptionThrown)(::windows::core::Interface::as_raw(self), thrownobjectid).ok()
    }
    pub unsafe fn ExceptionSearchFunctionEnter(&self, functionid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.ExceptionSearchFunctionEnter)(::windows::core::Interface::as_raw(self), functionid).ok()
    }
    pub unsafe fn ExceptionSearchFunctionLeave(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.ExceptionSearchFunctionLeave)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ExceptionSearchFilterEnter(&self, functionid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.ExceptionSearchFilterEnter)(::windows::core::Interface::as_raw(self), functionid).ok()
    }
    pub unsafe fn ExceptionSearchFilterLeave(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.ExceptionSearchFilterLeave)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ExceptionSearchCatcherFound(&self, functionid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.ExceptionSearchCatcherFound)(::windows::core::Interface::as_raw(self), functionid).ok()
    }
    pub unsafe fn ExceptionOSHandlerEnter(&self, __unused: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.ExceptionOSHandlerEnter)(::windows::core::Interface::as_raw(self), __unused).ok()
    }
    pub unsafe fn ExceptionOSHandlerLeave(&self, __unused: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.ExceptionOSHandlerLeave)(::windows::core::Interface::as_raw(self), __unused).ok()
    }
    pub unsafe fn ExceptionUnwindFunctionEnter(&self, functionid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.ExceptionUnwindFunctionEnter)(::windows::core::Interface::as_raw(self), functionid).ok()
    }
    pub unsafe fn ExceptionUnwindFunctionLeave(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.ExceptionUnwindFunctionLeave)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ExceptionUnwindFinallyEnter(&self, functionid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.ExceptionUnwindFinallyEnter)(::windows::core::Interface::as_raw(self), functionid).ok()
    }
    pub unsafe fn ExceptionUnwindFinallyLeave(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.ExceptionUnwindFinallyLeave)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ExceptionCatcherEnter(&self, functionid: usize, objectid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.ExceptionCatcherEnter)(::windows::core::Interface::as_raw(self), functionid, objectid).ok()
    }
    pub unsafe fn ExceptionCatcherLeave(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.ExceptionCatcherLeave)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn COMClassicVTableCreated(&self, wrappedclassid: usize, implementediid: *const ::windows::core::GUID, pvtable: *mut ::core::ffi::c_void, cslots: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.COMClassicVTableCreated)(::windows::core::Interface::as_raw(self), wrappedclassid, implementediid, pvtable, cslots).ok()
    }
    pub unsafe fn COMClassicVTableDestroyed(&self, wrappedclassid: usize, implementediid: *const ::windows::core::GUID, pvtable: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.COMClassicVTableDestroyed)(::windows::core::Interface::as_raw(self), wrappedclassid, implementediid, pvtable).ok()
    }
    pub unsafe fn ExceptionCLRCatcherFound(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.ExceptionCLRCatcherFound)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ExceptionCLRCatcherExecute(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.ExceptionCLRCatcherExecute)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ThreadNameChanged(&self, threadid: usize, name: ::core::option::Option<&[u16]>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.ThreadNameChanged)(::windows::core::Interface::as_raw(self), threadid, name.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(name.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GarbageCollectionStarted(&self, cgenerations: i32, generationcollected: *mut super::super::super::Foundation::BOOL, reason: COR_PRF_GC_REASON) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GarbageCollectionStarted)(::windows::core::Interface::as_raw(self), cgenerations, generationcollected, reason).ok()
    }
    pub unsafe fn SurvivingReferences(&self, csurvivingobjectidranges: u32, objectidrangestart: *mut usize, cobjectidrangelength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.SurvivingReferences)(::windows::core::Interface::as_raw(self), csurvivingobjectidranges, objectidrangestart, cobjectidrangelength).ok()
    }
    pub unsafe fn GarbageCollectionFinished(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GarbageCollectionFinished)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn FinalizeableObjectQueued(&self, finalizerflags: u32, objectid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.FinalizeableObjectQueued)(::windows::core::Interface::as_raw(self), finalizerflags, objectid).ok()
    }
    pub unsafe fn RootReferences2(&self, crootrefs: u32, rootrefids: *mut usize, rootkinds: *mut COR_PRF_GC_ROOT_KIND, rootflags: *mut COR_PRF_GC_ROOT_FLAGS, rootids: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.RootReferences2)(::windows::core::Interface::as_raw(self), crootrefs, rootrefids, rootkinds, rootflags, rootids).ok()
    }
    pub unsafe fn HandleCreated(&self, handleid: usize, initialobjectid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.HandleCreated)(::windows::core::Interface::as_raw(self), handleid, initialobjectid).ok()
    }
    pub unsafe fn HandleDestroyed(&self, handleid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.HandleDestroyed)(::windows::core::Interface::as_raw(self), handleid).ok()
    }
    pub unsafe fn InitializeForAttach<P0>(&self, pcorprofilerinfounk: P0, pvclientdata: *mut ::core::ffi::c_void, cbclientdata: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.InitializeForAttach)(::windows::core::Interface::as_raw(self), pcorprofilerinfounk.into_param().abi(), pvclientdata, cbclientdata).ok()
    }
    pub unsafe fn ProfilerAttachComplete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.ProfilerAttachComplete)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ProfilerDetachSucceeded(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.ProfilerDetachSucceeded)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReJITCompilationStarted<P0>(&self, functionid: usize, rejitid: usize, fissafetoblock: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.ReJITCompilationStarted)(::windows::core::Interface::as_raw(self), functionid, rejitid, fissafetoblock.into_param().abi()).ok()
    }
    pub unsafe fn GetReJITParameters<P0>(&self, moduleid: usize, methodid: u32, pfunctioncontrol: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ICorProfilerFunctionControl>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.GetReJITParameters)(::windows::core::Interface::as_raw(self), moduleid, methodid, pfunctioncontrol.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReJITCompilationFinished<P0>(&self, functionid: usize, rejitid: usize, hrstatus: ::windows::core::HRESULT, fissafetoblock: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.ReJITCompilationFinished)(::windows::core::Interface::as_raw(self), functionid, rejitid, hrstatus, fissafetoblock.into_param().abi()).ok()
    }
    pub unsafe fn ReJITError(&self, moduleid: usize, methodid: u32, functionid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.ReJITError)(::windows::core::Interface::as_raw(self), moduleid, methodid, functionid, hrstatus).ok()
    }
    pub unsafe fn MovedReferences2(&self, cmovedobjectidranges: u32, oldobjectidrangestart: *mut usize, newobjectidrangestart: *mut usize, cobjectidrangelength: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.MovedReferences2)(::windows::core::Interface::as_raw(self), cmovedobjectidranges, oldobjectidrangestart, newobjectidrangestart, cobjectidrangelength).ok()
    }
    pub unsafe fn SurvivingReferences2(&self, csurvivingobjectidranges: u32, objectidrangestart: *mut usize, cobjectidrangelength: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SurvivingReferences2)(::windows::core::Interface::as_raw(self), csurvivingobjectidranges, objectidrangestart, cobjectidrangelength).ok()
    }
    pub unsafe fn ConditionalWeakTableElementReferences(&self, crootrefs: u32, keyrefids: *mut usize, valuerefids: *mut usize, rootids: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ConditionalWeakTableElementReferences)(::windows::core::Interface::as_raw(self), crootrefs, keyrefids, valuerefids, rootids).ok()
    }
    pub unsafe fn GetAssemblyReferences<P0, P1>(&self, wszassemblypath: P0, pasmrefprovider: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<ICorProfilerAssemblyReferenceProvider>,
    {
        (::windows::core::Interface::vtable(self).GetAssemblyReferences)(::windows::core::Interface::as_raw(self), wszassemblypath.into_param().abi(), pasmrefprovider.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(ICorProfilerCallback6, ::windows::core::IUnknown, ICorProfilerCallback, ICorProfilerCallback2, ICorProfilerCallback3, ICorProfilerCallback4, ICorProfilerCallback5);
impl ::core::cmp::PartialEq for ICorProfilerCallback6 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICorProfilerCallback6 {}
impl ::core::fmt::Debug for ICorProfilerCallback6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICorProfilerCallback6").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICorProfilerCallback6 {
    type Vtable = ICorProfilerCallback6_Vtbl;
}
impl ::core::clone::Clone for ICorProfilerCallback6 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICorProfilerCallback6 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfc13df4b_4448_4f4f_950c_ba8d19d00c36);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorProfilerCallback6_Vtbl {
    pub base__: ICorProfilerCallback5_Vtbl,
    pub GetAssemblyReferences: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszassemblypath: ::windows::core::PCWSTR, pasmrefprovider: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
#[repr(transparent)]
pub struct ICorProfilerCallback7(::windows::core::IUnknown);
impl ICorProfilerCallback7 {
    pub unsafe fn Initialize<P0>(&self, picorprofilerinfounk: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.Initialize)(::windows::core::Interface::as_raw(self), picorprofilerinfounk.into_param().abi()).ok()
    }
    pub unsafe fn Shutdown(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.Shutdown)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn AppDomainCreationStarted(&self, appdomainid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.AppDomainCreationStarted)(::windows::core::Interface::as_raw(self), appdomainid).ok()
    }
    pub unsafe fn AppDomainCreationFinished(&self, appdomainid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.AppDomainCreationFinished)(::windows::core::Interface::as_raw(self), appdomainid, hrstatus).ok()
    }
    pub unsafe fn AppDomainShutdownStarted(&self, appdomainid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.AppDomainShutdownStarted)(::windows::core::Interface::as_raw(self), appdomainid).ok()
    }
    pub unsafe fn AppDomainShutdownFinished(&self, appdomainid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.AppDomainShutdownFinished)(::windows::core::Interface::as_raw(self), appdomainid, hrstatus).ok()
    }
    pub unsafe fn AssemblyLoadStarted(&self, assemblyid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.AssemblyLoadStarted)(::windows::core::Interface::as_raw(self), assemblyid).ok()
    }
    pub unsafe fn AssemblyLoadFinished(&self, assemblyid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.AssemblyLoadFinished)(::windows::core::Interface::as_raw(self), assemblyid, hrstatus).ok()
    }
    pub unsafe fn AssemblyUnloadStarted(&self, assemblyid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.AssemblyUnloadStarted)(::windows::core::Interface::as_raw(self), assemblyid).ok()
    }
    pub unsafe fn AssemblyUnloadFinished(&self, assemblyid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.AssemblyUnloadFinished)(::windows::core::Interface::as_raw(self), assemblyid, hrstatus).ok()
    }
    pub unsafe fn ModuleLoadStarted(&self, moduleid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.ModuleLoadStarted)(::windows::core::Interface::as_raw(self), moduleid).ok()
    }
    pub unsafe fn ModuleLoadFinished(&self, moduleid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.ModuleLoadFinished)(::windows::core::Interface::as_raw(self), moduleid, hrstatus).ok()
    }
    pub unsafe fn ModuleUnloadStarted(&self, moduleid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.ModuleUnloadStarted)(::windows::core::Interface::as_raw(self), moduleid).ok()
    }
    pub unsafe fn ModuleUnloadFinished(&self, moduleid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.ModuleUnloadFinished)(::windows::core::Interface::as_raw(self), moduleid, hrstatus).ok()
    }
    pub unsafe fn ModuleAttachedToAssembly(&self, moduleid: usize, assemblyid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.ModuleAttachedToAssembly)(::windows::core::Interface::as_raw(self), moduleid, assemblyid).ok()
    }
    pub unsafe fn ClassLoadStarted(&self, classid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.ClassLoadStarted)(::windows::core::Interface::as_raw(self), classid).ok()
    }
    pub unsafe fn ClassLoadFinished(&self, classid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.ClassLoadFinished)(::windows::core::Interface::as_raw(self), classid, hrstatus).ok()
    }
    pub unsafe fn ClassUnloadStarted(&self, classid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.ClassUnloadStarted)(::windows::core::Interface::as_raw(self), classid).ok()
    }
    pub unsafe fn ClassUnloadFinished(&self, classid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.ClassUnloadFinished)(::windows::core::Interface::as_raw(self), classid, hrstatus).ok()
    }
    pub unsafe fn FunctionUnloadStarted(&self, functionid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.FunctionUnloadStarted)(::windows::core::Interface::as_raw(self), functionid).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn JITCompilationStarted<P0>(&self, functionid: usize, fissafetoblock: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.JITCompilationStarted)(::windows::core::Interface::as_raw(self), functionid, fissafetoblock.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn JITCompilationFinished<P0>(&self, functionid: usize, hrstatus: ::windows::core::HRESULT, fissafetoblock: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.JITCompilationFinished)(::windows::core::Interface::as_raw(self), functionid, hrstatus, fissafetoblock.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn JITCachedFunctionSearchStarted(&self, functionid: usize, pbusecachedfunction: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.JITCachedFunctionSearchStarted)(::windows::core::Interface::as_raw(self), functionid, pbusecachedfunction).ok()
    }
    pub unsafe fn JITCachedFunctionSearchFinished(&self, functionid: usize, result: COR_PRF_JIT_CACHE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.JITCachedFunctionSearchFinished)(::windows::core::Interface::as_raw(self), functionid, result).ok()
    }
    pub unsafe fn JITFunctionPitched(&self, functionid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.JITFunctionPitched)(::windows::core::Interface::as_raw(self), functionid).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn JITInlining(&self, callerid: usize, calleeid: usize, pfshouldinline: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.JITInlining)(::windows::core::Interface::as_raw(self), callerid, calleeid, pfshouldinline).ok()
    }
    pub unsafe fn ThreadCreated(&self, threadid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.ThreadCreated)(::windows::core::Interface::as_raw(self), threadid).ok()
    }
    pub unsafe fn ThreadDestroyed(&self, threadid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.ThreadDestroyed)(::windows::core::Interface::as_raw(self), threadid).ok()
    }
    pub unsafe fn ThreadAssignedToOSThread(&self, managedthreadid: usize, osthreadid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.ThreadAssignedToOSThread)(::windows::core::Interface::as_raw(self), managedthreadid, osthreadid).ok()
    }
    pub unsafe fn RemotingClientInvocationStarted(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.RemotingClientInvocationStarted)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemotingClientSendingMessage<P0>(&self, pcookie: *mut ::windows::core::GUID, fisasync: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.RemotingClientSendingMessage)(::windows::core::Interface::as_raw(self), pcookie, fisasync.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemotingClientReceivingReply<P0>(&self, pcookie: *mut ::windows::core::GUID, fisasync: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.RemotingClientReceivingReply)(::windows::core::Interface::as_raw(self), pcookie, fisasync.into_param().abi()).ok()
    }
    pub unsafe fn RemotingClientInvocationFinished(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.RemotingClientInvocationFinished)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemotingServerReceivingMessage<P0>(&self, pcookie: *mut ::windows::core::GUID, fisasync: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.RemotingServerReceivingMessage)(::windows::core::Interface::as_raw(self), pcookie, fisasync.into_param().abi()).ok()
    }
    pub unsafe fn RemotingServerInvocationStarted(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.RemotingServerInvocationStarted)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RemotingServerInvocationReturned(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.RemotingServerInvocationReturned)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemotingServerSendingReply<P0>(&self, pcookie: *mut ::windows::core::GUID, fisasync: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.RemotingServerSendingReply)(::windows::core::Interface::as_raw(self), pcookie, fisasync.into_param().abi()).ok()
    }
    pub unsafe fn UnmanagedToManagedTransition(&self, functionid: usize, reason: COR_PRF_TRANSITION_REASON) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.UnmanagedToManagedTransition)(::windows::core::Interface::as_raw(self), functionid, reason).ok()
    }
    pub unsafe fn ManagedToUnmanagedTransition(&self, functionid: usize, reason: COR_PRF_TRANSITION_REASON) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.ManagedToUnmanagedTransition)(::windows::core::Interface::as_raw(self), functionid, reason).ok()
    }
    pub unsafe fn RuntimeSuspendStarted(&self, suspendreason: COR_PRF_SUSPEND_REASON) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.RuntimeSuspendStarted)(::windows::core::Interface::as_raw(self), suspendreason).ok()
    }
    pub unsafe fn RuntimeSuspendFinished(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.RuntimeSuspendFinished)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RuntimeSuspendAborted(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.RuntimeSuspendAborted)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RuntimeResumeStarted(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.RuntimeResumeStarted)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RuntimeResumeFinished(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.RuntimeResumeFinished)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RuntimeThreadSuspended(&self, threadid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.RuntimeThreadSuspended)(::windows::core::Interface::as_raw(self), threadid).ok()
    }
    pub unsafe fn RuntimeThreadResumed(&self, threadid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.RuntimeThreadResumed)(::windows::core::Interface::as_raw(self), threadid).ok()
    }
    pub unsafe fn MovedReferences(&self, cmovedobjectidranges: u32, oldobjectidrangestart: *mut usize, newobjectidrangestart: *mut usize, cobjectidrangelength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.MovedReferences)(::windows::core::Interface::as_raw(self), cmovedobjectidranges, oldobjectidrangestart, newobjectidrangestart, cobjectidrangelength).ok()
    }
    pub unsafe fn ObjectAllocated(&self, objectid: usize, classid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.ObjectAllocated)(::windows::core::Interface::as_raw(self), objectid, classid).ok()
    }
    pub unsafe fn ObjectsAllocatedByClass(&self, cclasscount: u32, classids: *mut usize, cobjects: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.ObjectsAllocatedByClass)(::windows::core::Interface::as_raw(self), cclasscount, classids, cobjects).ok()
    }
    pub unsafe fn ObjectReferences(&self, objectid: usize, classid: usize, cobjectrefs: u32, objectrefids: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.ObjectReferences)(::windows::core::Interface::as_raw(self), objectid, classid, cobjectrefs, objectrefids).ok()
    }
    pub unsafe fn RootReferences(&self, crootrefs: u32, rootrefids: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.RootReferences)(::windows::core::Interface::as_raw(self), crootrefs, rootrefids).ok()
    }
    pub unsafe fn ExceptionThrown(&self, thrownobjectid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.ExceptionThrown)(::windows::core::Interface::as_raw(self), thrownobjectid).ok()
    }
    pub unsafe fn ExceptionSearchFunctionEnter(&self, functionid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.ExceptionSearchFunctionEnter)(::windows::core::Interface::as_raw(self), functionid).ok()
    }
    pub unsafe fn ExceptionSearchFunctionLeave(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.ExceptionSearchFunctionLeave)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ExceptionSearchFilterEnter(&self, functionid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.ExceptionSearchFilterEnter)(::windows::core::Interface::as_raw(self), functionid).ok()
    }
    pub unsafe fn ExceptionSearchFilterLeave(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.ExceptionSearchFilterLeave)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ExceptionSearchCatcherFound(&self, functionid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.ExceptionSearchCatcherFound)(::windows::core::Interface::as_raw(self), functionid).ok()
    }
    pub unsafe fn ExceptionOSHandlerEnter(&self, __unused: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.ExceptionOSHandlerEnter)(::windows::core::Interface::as_raw(self), __unused).ok()
    }
    pub unsafe fn ExceptionOSHandlerLeave(&self, __unused: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.ExceptionOSHandlerLeave)(::windows::core::Interface::as_raw(self), __unused).ok()
    }
    pub unsafe fn ExceptionUnwindFunctionEnter(&self, functionid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.ExceptionUnwindFunctionEnter)(::windows::core::Interface::as_raw(self), functionid).ok()
    }
    pub unsafe fn ExceptionUnwindFunctionLeave(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.ExceptionUnwindFunctionLeave)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ExceptionUnwindFinallyEnter(&self, functionid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.ExceptionUnwindFinallyEnter)(::windows::core::Interface::as_raw(self), functionid).ok()
    }
    pub unsafe fn ExceptionUnwindFinallyLeave(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.ExceptionUnwindFinallyLeave)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ExceptionCatcherEnter(&self, functionid: usize, objectid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.ExceptionCatcherEnter)(::windows::core::Interface::as_raw(self), functionid, objectid).ok()
    }
    pub unsafe fn ExceptionCatcherLeave(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.ExceptionCatcherLeave)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn COMClassicVTableCreated(&self, wrappedclassid: usize, implementediid: *const ::windows::core::GUID, pvtable: *mut ::core::ffi::c_void, cslots: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.COMClassicVTableCreated)(::windows::core::Interface::as_raw(self), wrappedclassid, implementediid, pvtable, cslots).ok()
    }
    pub unsafe fn COMClassicVTableDestroyed(&self, wrappedclassid: usize, implementediid: *const ::windows::core::GUID, pvtable: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.COMClassicVTableDestroyed)(::windows::core::Interface::as_raw(self), wrappedclassid, implementediid, pvtable).ok()
    }
    pub unsafe fn ExceptionCLRCatcherFound(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.ExceptionCLRCatcherFound)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ExceptionCLRCatcherExecute(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.ExceptionCLRCatcherExecute)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ThreadNameChanged(&self, threadid: usize, name: ::core::option::Option<&[u16]>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.ThreadNameChanged)(::windows::core::Interface::as_raw(self), threadid, name.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(name.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GarbageCollectionStarted(&self, cgenerations: i32, generationcollected: *mut super::super::super::Foundation::BOOL, reason: COR_PRF_GC_REASON) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GarbageCollectionStarted)(::windows::core::Interface::as_raw(self), cgenerations, generationcollected, reason).ok()
    }
    pub unsafe fn SurvivingReferences(&self, csurvivingobjectidranges: u32, objectidrangestart: *mut usize, cobjectidrangelength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.SurvivingReferences)(::windows::core::Interface::as_raw(self), csurvivingobjectidranges, objectidrangestart, cobjectidrangelength).ok()
    }
    pub unsafe fn GarbageCollectionFinished(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GarbageCollectionFinished)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn FinalizeableObjectQueued(&self, finalizerflags: u32, objectid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.FinalizeableObjectQueued)(::windows::core::Interface::as_raw(self), finalizerflags, objectid).ok()
    }
    pub unsafe fn RootReferences2(&self, crootrefs: u32, rootrefids: *mut usize, rootkinds: *mut COR_PRF_GC_ROOT_KIND, rootflags: *mut COR_PRF_GC_ROOT_FLAGS, rootids: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.RootReferences2)(::windows::core::Interface::as_raw(self), crootrefs, rootrefids, rootkinds, rootflags, rootids).ok()
    }
    pub unsafe fn HandleCreated(&self, handleid: usize, initialobjectid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.HandleCreated)(::windows::core::Interface::as_raw(self), handleid, initialobjectid).ok()
    }
    pub unsafe fn HandleDestroyed(&self, handleid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.HandleDestroyed)(::windows::core::Interface::as_raw(self), handleid).ok()
    }
    pub unsafe fn InitializeForAttach<P0>(&self, pcorprofilerinfounk: P0, pvclientdata: *mut ::core::ffi::c_void, cbclientdata: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.InitializeForAttach)(::windows::core::Interface::as_raw(self), pcorprofilerinfounk.into_param().abi(), pvclientdata, cbclientdata).ok()
    }
    pub unsafe fn ProfilerAttachComplete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.ProfilerAttachComplete)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ProfilerDetachSucceeded(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.ProfilerDetachSucceeded)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReJITCompilationStarted<P0>(&self, functionid: usize, rejitid: usize, fissafetoblock: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.ReJITCompilationStarted)(::windows::core::Interface::as_raw(self), functionid, rejitid, fissafetoblock.into_param().abi()).ok()
    }
    pub unsafe fn GetReJITParameters<P0>(&self, moduleid: usize, methodid: u32, pfunctioncontrol: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ICorProfilerFunctionControl>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetReJITParameters)(::windows::core::Interface::as_raw(self), moduleid, methodid, pfunctioncontrol.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReJITCompilationFinished<P0>(&self, functionid: usize, rejitid: usize, hrstatus: ::windows::core::HRESULT, fissafetoblock: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.ReJITCompilationFinished)(::windows::core::Interface::as_raw(self), functionid, rejitid, hrstatus, fissafetoblock.into_param().abi()).ok()
    }
    pub unsafe fn ReJITError(&self, moduleid: usize, methodid: u32, functionid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.ReJITError)(::windows::core::Interface::as_raw(self), moduleid, methodid, functionid, hrstatus).ok()
    }
    pub unsafe fn MovedReferences2(&self, cmovedobjectidranges: u32, oldobjectidrangestart: *mut usize, newobjectidrangestart: *mut usize, cobjectidrangelength: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.MovedReferences2)(::windows::core::Interface::as_raw(self), cmovedobjectidranges, oldobjectidrangestart, newobjectidrangestart, cobjectidrangelength).ok()
    }
    pub unsafe fn SurvivingReferences2(&self, csurvivingobjectidranges: u32, objectidrangestart: *mut usize, cobjectidrangelength: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SurvivingReferences2)(::windows::core::Interface::as_raw(self), csurvivingobjectidranges, objectidrangestart, cobjectidrangelength).ok()
    }
    pub unsafe fn ConditionalWeakTableElementReferences(&self, crootrefs: u32, keyrefids: *mut usize, valuerefids: *mut usize, rootids: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.ConditionalWeakTableElementReferences)(::windows::core::Interface::as_raw(self), crootrefs, keyrefids, valuerefids, rootids).ok()
    }
    pub unsafe fn GetAssemblyReferences<P0, P1>(&self, wszassemblypath: P0, pasmrefprovider: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<ICorProfilerAssemblyReferenceProvider>,
    {
        (::windows::core::Interface::vtable(self).base__.GetAssemblyReferences)(::windows::core::Interface::as_raw(self), wszassemblypath.into_param().abi(), pasmrefprovider.into_param().abi()).ok()
    }
    pub unsafe fn ModuleInMemorySymbolsUpdated(&self, moduleid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ModuleInMemorySymbolsUpdated)(::windows::core::Interface::as_raw(self), moduleid).ok()
    }
}
::windows::imp::interface_hierarchy!(ICorProfilerCallback7, ::windows::core::IUnknown, ICorProfilerCallback, ICorProfilerCallback2, ICorProfilerCallback3, ICorProfilerCallback4, ICorProfilerCallback5, ICorProfilerCallback6);
impl ::core::cmp::PartialEq for ICorProfilerCallback7 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICorProfilerCallback7 {}
impl ::core::fmt::Debug for ICorProfilerCallback7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICorProfilerCallback7").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICorProfilerCallback7 {
    type Vtable = ICorProfilerCallback7_Vtbl;
}
impl ::core::clone::Clone for ICorProfilerCallback7 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICorProfilerCallback7 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf76a2dba_1d52_4539_866c_2aa518f9efc3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorProfilerCallback7_Vtbl {
    pub base__: ICorProfilerCallback6_Vtbl,
    pub ModuleInMemorySymbolsUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduleid: usize) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
#[repr(transparent)]
pub struct ICorProfilerCallback8(::windows::core::IUnknown);
impl ICorProfilerCallback8 {
    pub unsafe fn Initialize<P0>(&self, picorprofilerinfounk: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.Initialize)(::windows::core::Interface::as_raw(self), picorprofilerinfounk.into_param().abi()).ok()
    }
    pub unsafe fn Shutdown(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.Shutdown)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn AppDomainCreationStarted(&self, appdomainid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.AppDomainCreationStarted)(::windows::core::Interface::as_raw(self), appdomainid).ok()
    }
    pub unsafe fn AppDomainCreationFinished(&self, appdomainid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.AppDomainCreationFinished)(::windows::core::Interface::as_raw(self), appdomainid, hrstatus).ok()
    }
    pub unsafe fn AppDomainShutdownStarted(&self, appdomainid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.AppDomainShutdownStarted)(::windows::core::Interface::as_raw(self), appdomainid).ok()
    }
    pub unsafe fn AppDomainShutdownFinished(&self, appdomainid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.AppDomainShutdownFinished)(::windows::core::Interface::as_raw(self), appdomainid, hrstatus).ok()
    }
    pub unsafe fn AssemblyLoadStarted(&self, assemblyid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.AssemblyLoadStarted)(::windows::core::Interface::as_raw(self), assemblyid).ok()
    }
    pub unsafe fn AssemblyLoadFinished(&self, assemblyid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.AssemblyLoadFinished)(::windows::core::Interface::as_raw(self), assemblyid, hrstatus).ok()
    }
    pub unsafe fn AssemblyUnloadStarted(&self, assemblyid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.AssemblyUnloadStarted)(::windows::core::Interface::as_raw(self), assemblyid).ok()
    }
    pub unsafe fn AssemblyUnloadFinished(&self, assemblyid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.AssemblyUnloadFinished)(::windows::core::Interface::as_raw(self), assemblyid, hrstatus).ok()
    }
    pub unsafe fn ModuleLoadStarted(&self, moduleid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.ModuleLoadStarted)(::windows::core::Interface::as_raw(self), moduleid).ok()
    }
    pub unsafe fn ModuleLoadFinished(&self, moduleid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.ModuleLoadFinished)(::windows::core::Interface::as_raw(self), moduleid, hrstatus).ok()
    }
    pub unsafe fn ModuleUnloadStarted(&self, moduleid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.ModuleUnloadStarted)(::windows::core::Interface::as_raw(self), moduleid).ok()
    }
    pub unsafe fn ModuleUnloadFinished(&self, moduleid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.ModuleUnloadFinished)(::windows::core::Interface::as_raw(self), moduleid, hrstatus).ok()
    }
    pub unsafe fn ModuleAttachedToAssembly(&self, moduleid: usize, assemblyid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.ModuleAttachedToAssembly)(::windows::core::Interface::as_raw(self), moduleid, assemblyid).ok()
    }
    pub unsafe fn ClassLoadStarted(&self, classid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.ClassLoadStarted)(::windows::core::Interface::as_raw(self), classid).ok()
    }
    pub unsafe fn ClassLoadFinished(&self, classid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.ClassLoadFinished)(::windows::core::Interface::as_raw(self), classid, hrstatus).ok()
    }
    pub unsafe fn ClassUnloadStarted(&self, classid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.ClassUnloadStarted)(::windows::core::Interface::as_raw(self), classid).ok()
    }
    pub unsafe fn ClassUnloadFinished(&self, classid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.ClassUnloadFinished)(::windows::core::Interface::as_raw(self), classid, hrstatus).ok()
    }
    pub unsafe fn FunctionUnloadStarted(&self, functionid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.FunctionUnloadStarted)(::windows::core::Interface::as_raw(self), functionid).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn JITCompilationStarted<P0>(&self, functionid: usize, fissafetoblock: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.JITCompilationStarted)(::windows::core::Interface::as_raw(self), functionid, fissafetoblock.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn JITCompilationFinished<P0>(&self, functionid: usize, hrstatus: ::windows::core::HRESULT, fissafetoblock: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.JITCompilationFinished)(::windows::core::Interface::as_raw(self), functionid, hrstatus, fissafetoblock.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn JITCachedFunctionSearchStarted(&self, functionid: usize, pbusecachedfunction: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.JITCachedFunctionSearchStarted)(::windows::core::Interface::as_raw(self), functionid, pbusecachedfunction).ok()
    }
    pub unsafe fn JITCachedFunctionSearchFinished(&self, functionid: usize, result: COR_PRF_JIT_CACHE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.JITCachedFunctionSearchFinished)(::windows::core::Interface::as_raw(self), functionid, result).ok()
    }
    pub unsafe fn JITFunctionPitched(&self, functionid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.JITFunctionPitched)(::windows::core::Interface::as_raw(self), functionid).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn JITInlining(&self, callerid: usize, calleeid: usize, pfshouldinline: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.JITInlining)(::windows::core::Interface::as_raw(self), callerid, calleeid, pfshouldinline).ok()
    }
    pub unsafe fn ThreadCreated(&self, threadid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.ThreadCreated)(::windows::core::Interface::as_raw(self), threadid).ok()
    }
    pub unsafe fn ThreadDestroyed(&self, threadid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.ThreadDestroyed)(::windows::core::Interface::as_raw(self), threadid).ok()
    }
    pub unsafe fn ThreadAssignedToOSThread(&self, managedthreadid: usize, osthreadid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.ThreadAssignedToOSThread)(::windows::core::Interface::as_raw(self), managedthreadid, osthreadid).ok()
    }
    pub unsafe fn RemotingClientInvocationStarted(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.RemotingClientInvocationStarted)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemotingClientSendingMessage<P0>(&self, pcookie: *mut ::windows::core::GUID, fisasync: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.RemotingClientSendingMessage)(::windows::core::Interface::as_raw(self), pcookie, fisasync.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemotingClientReceivingReply<P0>(&self, pcookie: *mut ::windows::core::GUID, fisasync: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.RemotingClientReceivingReply)(::windows::core::Interface::as_raw(self), pcookie, fisasync.into_param().abi()).ok()
    }
    pub unsafe fn RemotingClientInvocationFinished(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.RemotingClientInvocationFinished)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemotingServerReceivingMessage<P0>(&self, pcookie: *mut ::windows::core::GUID, fisasync: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.RemotingServerReceivingMessage)(::windows::core::Interface::as_raw(self), pcookie, fisasync.into_param().abi()).ok()
    }
    pub unsafe fn RemotingServerInvocationStarted(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.RemotingServerInvocationStarted)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RemotingServerInvocationReturned(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.RemotingServerInvocationReturned)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemotingServerSendingReply<P0>(&self, pcookie: *mut ::windows::core::GUID, fisasync: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.RemotingServerSendingReply)(::windows::core::Interface::as_raw(self), pcookie, fisasync.into_param().abi()).ok()
    }
    pub unsafe fn UnmanagedToManagedTransition(&self, functionid: usize, reason: COR_PRF_TRANSITION_REASON) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.UnmanagedToManagedTransition)(::windows::core::Interface::as_raw(self), functionid, reason).ok()
    }
    pub unsafe fn ManagedToUnmanagedTransition(&self, functionid: usize, reason: COR_PRF_TRANSITION_REASON) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.ManagedToUnmanagedTransition)(::windows::core::Interface::as_raw(self), functionid, reason).ok()
    }
    pub unsafe fn RuntimeSuspendStarted(&self, suspendreason: COR_PRF_SUSPEND_REASON) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.RuntimeSuspendStarted)(::windows::core::Interface::as_raw(self), suspendreason).ok()
    }
    pub unsafe fn RuntimeSuspendFinished(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.RuntimeSuspendFinished)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RuntimeSuspendAborted(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.RuntimeSuspendAborted)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RuntimeResumeStarted(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.RuntimeResumeStarted)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RuntimeResumeFinished(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.RuntimeResumeFinished)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RuntimeThreadSuspended(&self, threadid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.RuntimeThreadSuspended)(::windows::core::Interface::as_raw(self), threadid).ok()
    }
    pub unsafe fn RuntimeThreadResumed(&self, threadid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.RuntimeThreadResumed)(::windows::core::Interface::as_raw(self), threadid).ok()
    }
    pub unsafe fn MovedReferences(&self, cmovedobjectidranges: u32, oldobjectidrangestart: *mut usize, newobjectidrangestart: *mut usize, cobjectidrangelength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.MovedReferences)(::windows::core::Interface::as_raw(self), cmovedobjectidranges, oldobjectidrangestart, newobjectidrangestart, cobjectidrangelength).ok()
    }
    pub unsafe fn ObjectAllocated(&self, objectid: usize, classid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.ObjectAllocated)(::windows::core::Interface::as_raw(self), objectid, classid).ok()
    }
    pub unsafe fn ObjectsAllocatedByClass(&self, cclasscount: u32, classids: *mut usize, cobjects: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.ObjectsAllocatedByClass)(::windows::core::Interface::as_raw(self), cclasscount, classids, cobjects).ok()
    }
    pub unsafe fn ObjectReferences(&self, objectid: usize, classid: usize, cobjectrefs: u32, objectrefids: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.ObjectReferences)(::windows::core::Interface::as_raw(self), objectid, classid, cobjectrefs, objectrefids).ok()
    }
    pub unsafe fn RootReferences(&self, crootrefs: u32, rootrefids: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.RootReferences)(::windows::core::Interface::as_raw(self), crootrefs, rootrefids).ok()
    }
    pub unsafe fn ExceptionThrown(&self, thrownobjectid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.ExceptionThrown)(::windows::core::Interface::as_raw(self), thrownobjectid).ok()
    }
    pub unsafe fn ExceptionSearchFunctionEnter(&self, functionid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.ExceptionSearchFunctionEnter)(::windows::core::Interface::as_raw(self), functionid).ok()
    }
    pub unsafe fn ExceptionSearchFunctionLeave(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.ExceptionSearchFunctionLeave)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ExceptionSearchFilterEnter(&self, functionid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.ExceptionSearchFilterEnter)(::windows::core::Interface::as_raw(self), functionid).ok()
    }
    pub unsafe fn ExceptionSearchFilterLeave(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.ExceptionSearchFilterLeave)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ExceptionSearchCatcherFound(&self, functionid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.ExceptionSearchCatcherFound)(::windows::core::Interface::as_raw(self), functionid).ok()
    }
    pub unsafe fn ExceptionOSHandlerEnter(&self, __unused: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.ExceptionOSHandlerEnter)(::windows::core::Interface::as_raw(self), __unused).ok()
    }
    pub unsafe fn ExceptionOSHandlerLeave(&self, __unused: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.ExceptionOSHandlerLeave)(::windows::core::Interface::as_raw(self), __unused).ok()
    }
    pub unsafe fn ExceptionUnwindFunctionEnter(&self, functionid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.ExceptionUnwindFunctionEnter)(::windows::core::Interface::as_raw(self), functionid).ok()
    }
    pub unsafe fn ExceptionUnwindFunctionLeave(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.ExceptionUnwindFunctionLeave)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ExceptionUnwindFinallyEnter(&self, functionid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.ExceptionUnwindFinallyEnter)(::windows::core::Interface::as_raw(self), functionid).ok()
    }
    pub unsafe fn ExceptionUnwindFinallyLeave(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.ExceptionUnwindFinallyLeave)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ExceptionCatcherEnter(&self, functionid: usize, objectid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.ExceptionCatcherEnter)(::windows::core::Interface::as_raw(self), functionid, objectid).ok()
    }
    pub unsafe fn ExceptionCatcherLeave(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.ExceptionCatcherLeave)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn COMClassicVTableCreated(&self, wrappedclassid: usize, implementediid: *const ::windows::core::GUID, pvtable: *mut ::core::ffi::c_void, cslots: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.COMClassicVTableCreated)(::windows::core::Interface::as_raw(self), wrappedclassid, implementediid, pvtable, cslots).ok()
    }
    pub unsafe fn COMClassicVTableDestroyed(&self, wrappedclassid: usize, implementediid: *const ::windows::core::GUID, pvtable: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.COMClassicVTableDestroyed)(::windows::core::Interface::as_raw(self), wrappedclassid, implementediid, pvtable).ok()
    }
    pub unsafe fn ExceptionCLRCatcherFound(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.ExceptionCLRCatcherFound)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ExceptionCLRCatcherExecute(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.ExceptionCLRCatcherExecute)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ThreadNameChanged(&self, threadid: usize, name: ::core::option::Option<&[u16]>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.ThreadNameChanged)(::windows::core::Interface::as_raw(self), threadid, name.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(name.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GarbageCollectionStarted(&self, cgenerations: i32, generationcollected: *mut super::super::super::Foundation::BOOL, reason: COR_PRF_GC_REASON) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.GarbageCollectionStarted)(::windows::core::Interface::as_raw(self), cgenerations, generationcollected, reason).ok()
    }
    pub unsafe fn SurvivingReferences(&self, csurvivingobjectidranges: u32, objectidrangestart: *mut usize, cobjectidrangelength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.SurvivingReferences)(::windows::core::Interface::as_raw(self), csurvivingobjectidranges, objectidrangestart, cobjectidrangelength).ok()
    }
    pub unsafe fn GarbageCollectionFinished(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.GarbageCollectionFinished)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn FinalizeableObjectQueued(&self, finalizerflags: u32, objectid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.FinalizeableObjectQueued)(::windows::core::Interface::as_raw(self), finalizerflags, objectid).ok()
    }
    pub unsafe fn RootReferences2(&self, crootrefs: u32, rootrefids: *mut usize, rootkinds: *mut COR_PRF_GC_ROOT_KIND, rootflags: *mut COR_PRF_GC_ROOT_FLAGS, rootids: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.RootReferences2)(::windows::core::Interface::as_raw(self), crootrefs, rootrefids, rootkinds, rootflags, rootids).ok()
    }
    pub unsafe fn HandleCreated(&self, handleid: usize, initialobjectid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.HandleCreated)(::windows::core::Interface::as_raw(self), handleid, initialobjectid).ok()
    }
    pub unsafe fn HandleDestroyed(&self, handleid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.HandleDestroyed)(::windows::core::Interface::as_raw(self), handleid).ok()
    }
    pub unsafe fn InitializeForAttach<P0>(&self, pcorprofilerinfounk: P0, pvclientdata: *mut ::core::ffi::c_void, cbclientdata: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.InitializeForAttach)(::windows::core::Interface::as_raw(self), pcorprofilerinfounk.into_param().abi(), pvclientdata, cbclientdata).ok()
    }
    pub unsafe fn ProfilerAttachComplete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.ProfilerAttachComplete)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ProfilerDetachSucceeded(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.ProfilerDetachSucceeded)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReJITCompilationStarted<P0>(&self, functionid: usize, rejitid: usize, fissafetoblock: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.ReJITCompilationStarted)(::windows::core::Interface::as_raw(self), functionid, rejitid, fissafetoblock.into_param().abi()).ok()
    }
    pub unsafe fn GetReJITParameters<P0>(&self, moduleid: usize, methodid: u32, pfunctioncontrol: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ICorProfilerFunctionControl>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetReJITParameters)(::windows::core::Interface::as_raw(self), moduleid, methodid, pfunctioncontrol.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReJITCompilationFinished<P0>(&self, functionid: usize, rejitid: usize, hrstatus: ::windows::core::HRESULT, fissafetoblock: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.ReJITCompilationFinished)(::windows::core::Interface::as_raw(self), functionid, rejitid, hrstatus, fissafetoblock.into_param().abi()).ok()
    }
    pub unsafe fn ReJITError(&self, moduleid: usize, methodid: u32, functionid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.ReJITError)(::windows::core::Interface::as_raw(self), moduleid, methodid, functionid, hrstatus).ok()
    }
    pub unsafe fn MovedReferences2(&self, cmovedobjectidranges: u32, oldobjectidrangestart: *mut usize, newobjectidrangestart: *mut usize, cobjectidrangelength: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.MovedReferences2)(::windows::core::Interface::as_raw(self), cmovedobjectidranges, oldobjectidrangestart, newobjectidrangestart, cobjectidrangelength).ok()
    }
    pub unsafe fn SurvivingReferences2(&self, csurvivingobjectidranges: u32, objectidrangestart: *mut usize, cobjectidrangelength: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.SurvivingReferences2)(::windows::core::Interface::as_raw(self), csurvivingobjectidranges, objectidrangestart, cobjectidrangelength).ok()
    }
    pub unsafe fn ConditionalWeakTableElementReferences(&self, crootrefs: u32, keyrefids: *mut usize, valuerefids: *mut usize, rootids: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.ConditionalWeakTableElementReferences)(::windows::core::Interface::as_raw(self), crootrefs, keyrefids, valuerefids, rootids).ok()
    }
    pub unsafe fn GetAssemblyReferences<P0, P1>(&self, wszassemblypath: P0, pasmrefprovider: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<ICorProfilerAssemblyReferenceProvider>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.GetAssemblyReferences)(::windows::core::Interface::as_raw(self), wszassemblypath.into_param().abi(), pasmrefprovider.into_param().abi()).ok()
    }
    pub unsafe fn ModuleInMemorySymbolsUpdated(&self, moduleid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ModuleInMemorySymbolsUpdated)(::windows::core::Interface::as_raw(self), moduleid).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DynamicMethodJITCompilationStarted<P0>(&self, functionid: usize, fissafetoblock: P0, pilheader: *mut u8, cbilheader: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).DynamicMethodJITCompilationStarted)(::windows::core::Interface::as_raw(self), functionid, fissafetoblock.into_param().abi(), pilheader, cbilheader).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DynamicMethodJITCompilationFinished<P0>(&self, functionid: usize, hrstatus: ::windows::core::HRESULT, fissafetoblock: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).DynamicMethodJITCompilationFinished)(::windows::core::Interface::as_raw(self), functionid, hrstatus, fissafetoblock.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(ICorProfilerCallback8, ::windows::core::IUnknown, ICorProfilerCallback, ICorProfilerCallback2, ICorProfilerCallback3, ICorProfilerCallback4, ICorProfilerCallback5, ICorProfilerCallback6, ICorProfilerCallback7);
impl ::core::cmp::PartialEq for ICorProfilerCallback8 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICorProfilerCallback8 {}
impl ::core::fmt::Debug for ICorProfilerCallback8 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICorProfilerCallback8").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICorProfilerCallback8 {
    type Vtable = ICorProfilerCallback8_Vtbl;
}
impl ::core::clone::Clone for ICorProfilerCallback8 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICorProfilerCallback8 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5bed9b15_c079_4d47_bfe2_215a140c07e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorProfilerCallback8_Vtbl {
    pub base__: ICorProfilerCallback7_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub DynamicMethodJITCompilationStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, functionid: usize, fissafetoblock: super::super::super::Foundation::BOOL, pilheader: *mut u8, cbilheader: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DynamicMethodJITCompilationStarted: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DynamicMethodJITCompilationFinished: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, functionid: usize, hrstatus: ::windows::core::HRESULT, fissafetoblock: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DynamicMethodJITCompilationFinished: usize,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
#[repr(transparent)]
pub struct ICorProfilerCallback9(::windows::core::IUnknown);
impl ICorProfilerCallback9 {
    pub unsafe fn Initialize<P0>(&self, picorprofilerinfounk: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.Initialize)(::windows::core::Interface::as_raw(self), picorprofilerinfounk.into_param().abi()).ok()
    }
    pub unsafe fn Shutdown(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.Shutdown)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn AppDomainCreationStarted(&self, appdomainid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.AppDomainCreationStarted)(::windows::core::Interface::as_raw(self), appdomainid).ok()
    }
    pub unsafe fn AppDomainCreationFinished(&self, appdomainid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.AppDomainCreationFinished)(::windows::core::Interface::as_raw(self), appdomainid, hrstatus).ok()
    }
    pub unsafe fn AppDomainShutdownStarted(&self, appdomainid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.AppDomainShutdownStarted)(::windows::core::Interface::as_raw(self), appdomainid).ok()
    }
    pub unsafe fn AppDomainShutdownFinished(&self, appdomainid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.AppDomainShutdownFinished)(::windows::core::Interface::as_raw(self), appdomainid, hrstatus).ok()
    }
    pub unsafe fn AssemblyLoadStarted(&self, assemblyid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.AssemblyLoadStarted)(::windows::core::Interface::as_raw(self), assemblyid).ok()
    }
    pub unsafe fn AssemblyLoadFinished(&self, assemblyid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.AssemblyLoadFinished)(::windows::core::Interface::as_raw(self), assemblyid, hrstatus).ok()
    }
    pub unsafe fn AssemblyUnloadStarted(&self, assemblyid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.AssemblyUnloadStarted)(::windows::core::Interface::as_raw(self), assemblyid).ok()
    }
    pub unsafe fn AssemblyUnloadFinished(&self, assemblyid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.AssemblyUnloadFinished)(::windows::core::Interface::as_raw(self), assemblyid, hrstatus).ok()
    }
    pub unsafe fn ModuleLoadStarted(&self, moduleid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.ModuleLoadStarted)(::windows::core::Interface::as_raw(self), moduleid).ok()
    }
    pub unsafe fn ModuleLoadFinished(&self, moduleid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.ModuleLoadFinished)(::windows::core::Interface::as_raw(self), moduleid, hrstatus).ok()
    }
    pub unsafe fn ModuleUnloadStarted(&self, moduleid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.ModuleUnloadStarted)(::windows::core::Interface::as_raw(self), moduleid).ok()
    }
    pub unsafe fn ModuleUnloadFinished(&self, moduleid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.ModuleUnloadFinished)(::windows::core::Interface::as_raw(self), moduleid, hrstatus).ok()
    }
    pub unsafe fn ModuleAttachedToAssembly(&self, moduleid: usize, assemblyid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.ModuleAttachedToAssembly)(::windows::core::Interface::as_raw(self), moduleid, assemblyid).ok()
    }
    pub unsafe fn ClassLoadStarted(&self, classid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.ClassLoadStarted)(::windows::core::Interface::as_raw(self), classid).ok()
    }
    pub unsafe fn ClassLoadFinished(&self, classid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.ClassLoadFinished)(::windows::core::Interface::as_raw(self), classid, hrstatus).ok()
    }
    pub unsafe fn ClassUnloadStarted(&self, classid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.ClassUnloadStarted)(::windows::core::Interface::as_raw(self), classid).ok()
    }
    pub unsafe fn ClassUnloadFinished(&self, classid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.ClassUnloadFinished)(::windows::core::Interface::as_raw(self), classid, hrstatus).ok()
    }
    pub unsafe fn FunctionUnloadStarted(&self, functionid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.FunctionUnloadStarted)(::windows::core::Interface::as_raw(self), functionid).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn JITCompilationStarted<P0>(&self, functionid: usize, fissafetoblock: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.JITCompilationStarted)(::windows::core::Interface::as_raw(self), functionid, fissafetoblock.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn JITCompilationFinished<P0>(&self, functionid: usize, hrstatus: ::windows::core::HRESULT, fissafetoblock: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.JITCompilationFinished)(::windows::core::Interface::as_raw(self), functionid, hrstatus, fissafetoblock.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn JITCachedFunctionSearchStarted(&self, functionid: usize, pbusecachedfunction: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.JITCachedFunctionSearchStarted)(::windows::core::Interface::as_raw(self), functionid, pbusecachedfunction).ok()
    }
    pub unsafe fn JITCachedFunctionSearchFinished(&self, functionid: usize, result: COR_PRF_JIT_CACHE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.JITCachedFunctionSearchFinished)(::windows::core::Interface::as_raw(self), functionid, result).ok()
    }
    pub unsafe fn JITFunctionPitched(&self, functionid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.JITFunctionPitched)(::windows::core::Interface::as_raw(self), functionid).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn JITInlining(&self, callerid: usize, calleeid: usize, pfshouldinline: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.JITInlining)(::windows::core::Interface::as_raw(self), callerid, calleeid, pfshouldinline).ok()
    }
    pub unsafe fn ThreadCreated(&self, threadid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.ThreadCreated)(::windows::core::Interface::as_raw(self), threadid).ok()
    }
    pub unsafe fn ThreadDestroyed(&self, threadid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.ThreadDestroyed)(::windows::core::Interface::as_raw(self), threadid).ok()
    }
    pub unsafe fn ThreadAssignedToOSThread(&self, managedthreadid: usize, osthreadid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.ThreadAssignedToOSThread)(::windows::core::Interface::as_raw(self), managedthreadid, osthreadid).ok()
    }
    pub unsafe fn RemotingClientInvocationStarted(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.RemotingClientInvocationStarted)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemotingClientSendingMessage<P0>(&self, pcookie: *mut ::windows::core::GUID, fisasync: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.RemotingClientSendingMessage)(::windows::core::Interface::as_raw(self), pcookie, fisasync.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemotingClientReceivingReply<P0>(&self, pcookie: *mut ::windows::core::GUID, fisasync: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.RemotingClientReceivingReply)(::windows::core::Interface::as_raw(self), pcookie, fisasync.into_param().abi()).ok()
    }
    pub unsafe fn RemotingClientInvocationFinished(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.RemotingClientInvocationFinished)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemotingServerReceivingMessage<P0>(&self, pcookie: *mut ::windows::core::GUID, fisasync: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.RemotingServerReceivingMessage)(::windows::core::Interface::as_raw(self), pcookie, fisasync.into_param().abi()).ok()
    }
    pub unsafe fn RemotingServerInvocationStarted(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.RemotingServerInvocationStarted)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RemotingServerInvocationReturned(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.RemotingServerInvocationReturned)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemotingServerSendingReply<P0>(&self, pcookie: *mut ::windows::core::GUID, fisasync: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.RemotingServerSendingReply)(::windows::core::Interface::as_raw(self), pcookie, fisasync.into_param().abi()).ok()
    }
    pub unsafe fn UnmanagedToManagedTransition(&self, functionid: usize, reason: COR_PRF_TRANSITION_REASON) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.UnmanagedToManagedTransition)(::windows::core::Interface::as_raw(self), functionid, reason).ok()
    }
    pub unsafe fn ManagedToUnmanagedTransition(&self, functionid: usize, reason: COR_PRF_TRANSITION_REASON) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.ManagedToUnmanagedTransition)(::windows::core::Interface::as_raw(self), functionid, reason).ok()
    }
    pub unsafe fn RuntimeSuspendStarted(&self, suspendreason: COR_PRF_SUSPEND_REASON) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.RuntimeSuspendStarted)(::windows::core::Interface::as_raw(self), suspendreason).ok()
    }
    pub unsafe fn RuntimeSuspendFinished(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.RuntimeSuspendFinished)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RuntimeSuspendAborted(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.RuntimeSuspendAborted)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RuntimeResumeStarted(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.RuntimeResumeStarted)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RuntimeResumeFinished(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.RuntimeResumeFinished)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RuntimeThreadSuspended(&self, threadid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.RuntimeThreadSuspended)(::windows::core::Interface::as_raw(self), threadid).ok()
    }
    pub unsafe fn RuntimeThreadResumed(&self, threadid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.RuntimeThreadResumed)(::windows::core::Interface::as_raw(self), threadid).ok()
    }
    pub unsafe fn MovedReferences(&self, cmovedobjectidranges: u32, oldobjectidrangestart: *mut usize, newobjectidrangestart: *mut usize, cobjectidrangelength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.MovedReferences)(::windows::core::Interface::as_raw(self), cmovedobjectidranges, oldobjectidrangestart, newobjectidrangestart, cobjectidrangelength).ok()
    }
    pub unsafe fn ObjectAllocated(&self, objectid: usize, classid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.ObjectAllocated)(::windows::core::Interface::as_raw(self), objectid, classid).ok()
    }
    pub unsafe fn ObjectsAllocatedByClass(&self, cclasscount: u32, classids: *mut usize, cobjects: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.ObjectsAllocatedByClass)(::windows::core::Interface::as_raw(self), cclasscount, classids, cobjects).ok()
    }
    pub unsafe fn ObjectReferences(&self, objectid: usize, classid: usize, cobjectrefs: u32, objectrefids: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.ObjectReferences)(::windows::core::Interface::as_raw(self), objectid, classid, cobjectrefs, objectrefids).ok()
    }
    pub unsafe fn RootReferences(&self, crootrefs: u32, rootrefids: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.RootReferences)(::windows::core::Interface::as_raw(self), crootrefs, rootrefids).ok()
    }
    pub unsafe fn ExceptionThrown(&self, thrownobjectid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.ExceptionThrown)(::windows::core::Interface::as_raw(self), thrownobjectid).ok()
    }
    pub unsafe fn ExceptionSearchFunctionEnter(&self, functionid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.ExceptionSearchFunctionEnter)(::windows::core::Interface::as_raw(self), functionid).ok()
    }
    pub unsafe fn ExceptionSearchFunctionLeave(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.ExceptionSearchFunctionLeave)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ExceptionSearchFilterEnter(&self, functionid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.ExceptionSearchFilterEnter)(::windows::core::Interface::as_raw(self), functionid).ok()
    }
    pub unsafe fn ExceptionSearchFilterLeave(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.ExceptionSearchFilterLeave)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ExceptionSearchCatcherFound(&self, functionid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.ExceptionSearchCatcherFound)(::windows::core::Interface::as_raw(self), functionid).ok()
    }
    pub unsafe fn ExceptionOSHandlerEnter(&self, __unused: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.ExceptionOSHandlerEnter)(::windows::core::Interface::as_raw(self), __unused).ok()
    }
    pub unsafe fn ExceptionOSHandlerLeave(&self, __unused: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.ExceptionOSHandlerLeave)(::windows::core::Interface::as_raw(self), __unused).ok()
    }
    pub unsafe fn ExceptionUnwindFunctionEnter(&self, functionid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.ExceptionUnwindFunctionEnter)(::windows::core::Interface::as_raw(self), functionid).ok()
    }
    pub unsafe fn ExceptionUnwindFunctionLeave(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.ExceptionUnwindFunctionLeave)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ExceptionUnwindFinallyEnter(&self, functionid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.ExceptionUnwindFinallyEnter)(::windows::core::Interface::as_raw(self), functionid).ok()
    }
    pub unsafe fn ExceptionUnwindFinallyLeave(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.ExceptionUnwindFinallyLeave)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ExceptionCatcherEnter(&self, functionid: usize, objectid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.ExceptionCatcherEnter)(::windows::core::Interface::as_raw(self), functionid, objectid).ok()
    }
    pub unsafe fn ExceptionCatcherLeave(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.ExceptionCatcherLeave)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn COMClassicVTableCreated(&self, wrappedclassid: usize, implementediid: *const ::windows::core::GUID, pvtable: *mut ::core::ffi::c_void, cslots: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.COMClassicVTableCreated)(::windows::core::Interface::as_raw(self), wrappedclassid, implementediid, pvtable, cslots).ok()
    }
    pub unsafe fn COMClassicVTableDestroyed(&self, wrappedclassid: usize, implementediid: *const ::windows::core::GUID, pvtable: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.COMClassicVTableDestroyed)(::windows::core::Interface::as_raw(self), wrappedclassid, implementediid, pvtable).ok()
    }
    pub unsafe fn ExceptionCLRCatcherFound(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.ExceptionCLRCatcherFound)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ExceptionCLRCatcherExecute(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.ExceptionCLRCatcherExecute)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ThreadNameChanged(&self, threadid: usize, name: ::core::option::Option<&[u16]>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.ThreadNameChanged)(::windows::core::Interface::as_raw(self), threadid, name.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(name.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GarbageCollectionStarted(&self, cgenerations: i32, generationcollected: *mut super::super::super::Foundation::BOOL, reason: COR_PRF_GC_REASON) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.GarbageCollectionStarted)(::windows::core::Interface::as_raw(self), cgenerations, generationcollected, reason).ok()
    }
    pub unsafe fn SurvivingReferences(&self, csurvivingobjectidranges: u32, objectidrangestart: *mut usize, cobjectidrangelength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.SurvivingReferences)(::windows::core::Interface::as_raw(self), csurvivingobjectidranges, objectidrangestart, cobjectidrangelength).ok()
    }
    pub unsafe fn GarbageCollectionFinished(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.GarbageCollectionFinished)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn FinalizeableObjectQueued(&self, finalizerflags: u32, objectid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.FinalizeableObjectQueued)(::windows::core::Interface::as_raw(self), finalizerflags, objectid).ok()
    }
    pub unsafe fn RootReferences2(&self, crootrefs: u32, rootrefids: *mut usize, rootkinds: *mut COR_PRF_GC_ROOT_KIND, rootflags: *mut COR_PRF_GC_ROOT_FLAGS, rootids: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.RootReferences2)(::windows::core::Interface::as_raw(self), crootrefs, rootrefids, rootkinds, rootflags, rootids).ok()
    }
    pub unsafe fn HandleCreated(&self, handleid: usize, initialobjectid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.HandleCreated)(::windows::core::Interface::as_raw(self), handleid, initialobjectid).ok()
    }
    pub unsafe fn HandleDestroyed(&self, handleid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.HandleDestroyed)(::windows::core::Interface::as_raw(self), handleid).ok()
    }
    pub unsafe fn InitializeForAttach<P0>(&self, pcorprofilerinfounk: P0, pvclientdata: *mut ::core::ffi::c_void, cbclientdata: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.InitializeForAttach)(::windows::core::Interface::as_raw(self), pcorprofilerinfounk.into_param().abi(), pvclientdata, cbclientdata).ok()
    }
    pub unsafe fn ProfilerAttachComplete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.ProfilerAttachComplete)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ProfilerDetachSucceeded(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.ProfilerDetachSucceeded)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReJITCompilationStarted<P0>(&self, functionid: usize, rejitid: usize, fissafetoblock: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.ReJITCompilationStarted)(::windows::core::Interface::as_raw(self), functionid, rejitid, fissafetoblock.into_param().abi()).ok()
    }
    pub unsafe fn GetReJITParameters<P0>(&self, moduleid: usize, methodid: u32, pfunctioncontrol: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ICorProfilerFunctionControl>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetReJITParameters)(::windows::core::Interface::as_raw(self), moduleid, methodid, pfunctioncontrol.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReJITCompilationFinished<P0>(&self, functionid: usize, rejitid: usize, hrstatus: ::windows::core::HRESULT, fissafetoblock: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.ReJITCompilationFinished)(::windows::core::Interface::as_raw(self), functionid, rejitid, hrstatus, fissafetoblock.into_param().abi()).ok()
    }
    pub unsafe fn ReJITError(&self, moduleid: usize, methodid: u32, functionid: usize, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.ReJITError)(::windows::core::Interface::as_raw(self), moduleid, methodid, functionid, hrstatus).ok()
    }
    pub unsafe fn MovedReferences2(&self, cmovedobjectidranges: u32, oldobjectidrangestart: *mut usize, newobjectidrangestart: *mut usize, cobjectidrangelength: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.MovedReferences2)(::windows::core::Interface::as_raw(self), cmovedobjectidranges, oldobjectidrangestart, newobjectidrangestart, cobjectidrangelength).ok()
    }
    pub unsafe fn SurvivingReferences2(&self, csurvivingobjectidranges: u32, objectidrangestart: *mut usize, cobjectidrangelength: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.SurvivingReferences2)(::windows::core::Interface::as_raw(self), csurvivingobjectidranges, objectidrangestart, cobjectidrangelength).ok()
    }
    pub unsafe fn ConditionalWeakTableElementReferences(&self, crootrefs: u32, keyrefids: *mut usize, valuerefids: *mut usize, rootids: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.ConditionalWeakTableElementReferences)(::windows::core::Interface::as_raw(self), crootrefs, keyrefids, valuerefids, rootids).ok()
    }
    pub unsafe fn GetAssemblyReferences<P0, P1>(&self, wszassemblypath: P0, pasmrefprovider: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<ICorProfilerAssemblyReferenceProvider>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetAssemblyReferences)(::windows::core::Interface::as_raw(self), wszassemblypath.into_param().abi(), pasmrefprovider.into_param().abi()).ok()
    }
    pub unsafe fn ModuleInMemorySymbolsUpdated(&self, moduleid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.ModuleInMemorySymbolsUpdated)(::windows::core::Interface::as_raw(self), moduleid).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DynamicMethodJITCompilationStarted<P0>(&self, functionid: usize, fissafetoblock: P0, pilheader: *mut u8, cbilheader: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.DynamicMethodJITCompilationStarted)(::windows::core::Interface::as_raw(self), functionid, fissafetoblock.into_param().abi(), pilheader, cbilheader).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DynamicMethodJITCompilationFinished<P0>(&self, functionid: usize, hrstatus: ::windows::core::HRESULT, fissafetoblock: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.DynamicMethodJITCompilationFinished)(::windows::core::Interface::as_raw(self), functionid, hrstatus, fissafetoblock.into_param().abi()).ok()
    }
    pub unsafe fn DynamicMethodUnloaded(&self, functionid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DynamicMethodUnloaded)(::windows::core::Interface::as_raw(self), functionid).ok()
    }
}
::windows::imp::interface_hierarchy!(ICorProfilerCallback9, ::windows::core::IUnknown, ICorProfilerCallback, ICorProfilerCallback2, ICorProfilerCallback3, ICorProfilerCallback4, ICorProfilerCallback5, ICorProfilerCallback6, ICorProfilerCallback7, ICorProfilerCallback8);
impl ::core::cmp::PartialEq for ICorProfilerCallback9 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICorProfilerCallback9 {}
impl ::core::fmt::Debug for ICorProfilerCallback9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICorProfilerCallback9").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICorProfilerCallback9 {
    type Vtable = ICorProfilerCallback9_Vtbl;
}
impl ::core::clone::Clone for ICorProfilerCallback9 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICorProfilerCallback9 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27583ec3_c8f5_482f_8052_194b8ce4705a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorProfilerCallback9_Vtbl {
    pub base__: ICorProfilerCallback8_Vtbl,
    pub DynamicMethodUnloaded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, functionid: usize) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
#[repr(transparent)]
pub struct ICorProfilerFunctionControl(::windows::core::IUnknown);
impl ICorProfilerFunctionControl {
    pub unsafe fn SetCodegenFlags(&self, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCodegenFlags)(::windows::core::Interface::as_raw(self), flags).ok()
    }
    pub unsafe fn SetILFunctionBody(&self, cbnewilmethodheader: u32, pbnewilmethodheader: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetILFunctionBody)(::windows::core::Interface::as_raw(self), cbnewilmethodheader, pbnewilmethodheader).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetILInstrumentedCodeMap(&self, cilmapentries: u32, rgilmapentries: *mut COR_IL_MAP) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetILInstrumentedCodeMap)(::windows::core::Interface::as_raw(self), cilmapentries, rgilmapentries).ok()
    }
}
::windows::imp::interface_hierarchy!(ICorProfilerFunctionControl, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ICorProfilerFunctionControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICorProfilerFunctionControl {}
impl ::core::fmt::Debug for ICorProfilerFunctionControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICorProfilerFunctionControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICorProfilerFunctionControl {
    type Vtable = ICorProfilerFunctionControl_Vtbl;
}
impl ::core::clone::Clone for ICorProfilerFunctionControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICorProfilerFunctionControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf0963021_e1ea_4732_8581_e01b0bd3c0c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorProfilerFunctionControl_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetCodegenFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: u32) -> ::windows::core::HRESULT,
    pub SetILFunctionBody: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbnewilmethodheader: u32, pbnewilmethodheader: *mut u8) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetILInstrumentedCodeMap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cilmapentries: u32, rgilmapentries: *mut COR_IL_MAP) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetILInstrumentedCodeMap: usize,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
#[repr(transparent)]
pub struct ICorProfilerFunctionEnum(::windows::core::IUnknown);
impl ICorProfilerFunctionEnum {
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<ICorProfilerFunctionEnum> {
        let mut result__ = ::windows::core::zeroed::<ICorProfilerFunctionEnum>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCount(&self, pcelt: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCount)(::windows::core::Interface::as_raw(self), pcelt).ok()
    }
    pub unsafe fn Next(&self, celt: u32, ids: *mut COR_PRF_FUNCTION, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), celt, ids, pceltfetched).ok()
    }
}
::windows::imp::interface_hierarchy!(ICorProfilerFunctionEnum, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ICorProfilerFunctionEnum {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICorProfilerFunctionEnum {}
impl ::core::fmt::Debug for ICorProfilerFunctionEnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICorProfilerFunctionEnum").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICorProfilerFunctionEnum {
    type Vtable = ICorProfilerFunctionEnum_Vtbl;
}
impl ::core::clone::Clone for ICorProfilerFunctionEnum {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICorProfilerFunctionEnum {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xff71301a_b994_429d_a10b_b345a65280ef);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorProfilerFunctionEnum_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcelt: *mut u32) -> ::windows::core::HRESULT,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, ids: *mut COR_PRF_FUNCTION, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
#[repr(transparent)]
pub struct ICorProfilerInfo(::windows::core::IUnknown);
impl ICorProfilerInfo {
    pub unsafe fn GetClassFromObject(&self, objectid: usize, pclassid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetClassFromObject)(::windows::core::Interface::as_raw(self), objectid, pclassid).ok()
    }
    pub unsafe fn GetClassFromToken(&self, moduleid: usize, typedef: u32, pclassid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetClassFromToken)(::windows::core::Interface::as_raw(self), moduleid, typedef, pclassid).ok()
    }
    pub unsafe fn GetCodeInfo(&self, functionid: usize, pstart: *mut *mut u8, pcsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCodeInfo)(::windows::core::Interface::as_raw(self), functionid, pstart, pcsize).ok()
    }
    pub unsafe fn GetEventMask(&self, pdwevents: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetEventMask)(::windows::core::Interface::as_raw(self), pdwevents).ok()
    }
    pub unsafe fn GetFunctionFromIP(&self, ip: *mut u8, pfunctionid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFunctionFromIP)(::windows::core::Interface::as_raw(self), ip, pfunctionid).ok()
    }
    pub unsafe fn GetFunctionFromToken(&self, moduleid: usize, token: u32, pfunctionid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFunctionFromToken)(::windows::core::Interface::as_raw(self), moduleid, token, pfunctionid).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHandleFromThread(&self, threadid: usize, phthread: *mut super::super::super::Foundation::HANDLE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetHandleFromThread)(::windows::core::Interface::as_raw(self), threadid, phthread).ok()
    }
    pub unsafe fn GetObjectSize(&self, objectid: usize, pcsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetObjectSize)(::windows::core::Interface::as_raw(self), objectid, pcsize).ok()
    }
    #[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
    #[cfg(feature = "Win32_System_WinRT_Metadata")]
    pub unsafe fn IsArrayClass(&self, classid: usize, pbaseelemtype: *mut super::super::WinRT::Metadata::CorElementType, pbaseclassid: *mut usize, pcrank: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).IsArrayClass)(::windows::core::Interface::as_raw(self), classid, pbaseelemtype, pbaseclassid, pcrank).ok()
    }
    pub unsafe fn GetThreadInfo(&self, threadid: usize, pdwwin32threadid: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetThreadInfo)(::windows::core::Interface::as_raw(self), threadid, pdwwin32threadid).ok()
    }
    pub unsafe fn GetCurrentThreadID(&self, pthreadid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCurrentThreadID)(::windows::core::Interface::as_raw(self), pthreadid).ok()
    }
    pub unsafe fn GetClassIDInfo(&self, classid: usize, pmoduleid: *mut usize, ptypedeftoken: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetClassIDInfo)(::windows::core::Interface::as_raw(self), classid, pmoduleid, ptypedeftoken).ok()
    }
    pub unsafe fn GetFunctionInfo(&self, functionid: usize, pclassid: *mut usize, pmoduleid: *mut usize, ptoken: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFunctionInfo)(::windows::core::Interface::as_raw(self), functionid, pclassid, pmoduleid, ptoken).ok()
    }
    pub unsafe fn SetEventMask(&self, dwevents: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEventMask)(::windows::core::Interface::as_raw(self), dwevents).ok()
    }
    pub unsafe fn SetEnterLeaveFunctionHooks(&self, pfuncenter: *mut FunctionEnter, pfuncleave: *mut FunctionLeave, pfunctailcall: *mut FunctionTailcall) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEnterLeaveFunctionHooks)(::windows::core::Interface::as_raw(self), pfuncenter, pfuncleave, pfunctailcall).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFunctionIDMapper(&self, pfunc: *mut FunctionIDMapper) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFunctionIDMapper)(::windows::core::Interface::as_raw(self), pfunc).ok()
    }
    pub unsafe fn GetTokenAndMetaDataFromFunction(&self, functionid: usize, riid: *const ::windows::core::GUID, ppimport: *mut ::core::option::Option<::windows::core::IUnknown>, ptoken: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetTokenAndMetaDataFromFunction)(::windows::core::Interface::as_raw(self), functionid, riid, ::core::mem::transmute(ppimport), ptoken).ok()
    }
    pub unsafe fn GetModuleInfo(&self, moduleid: usize, ppbaseloadaddress: *mut *mut u8, pcchname: *mut u32, szname: &mut [u16], passemblyid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetModuleInfo)(::windows::core::Interface::as_raw(self), moduleid, ppbaseloadaddress, szname.len() as _, pcchname, ::core::mem::transmute(szname.as_ptr()), passemblyid).ok()
    }
    pub unsafe fn GetModuleMetaData(&self, moduleid: usize, dwopenflags: u32, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).GetModuleMetaData)(::windows::core::Interface::as_raw(self), moduleid, dwopenflags, riid, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetILFunctionBody(&self, moduleid: usize, methodid: u32, ppmethodheader: *mut *mut u8, pcbmethodsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetILFunctionBody)(::windows::core::Interface::as_raw(self), moduleid, methodid, ppmethodheader, pcbmethodsize).ok()
    }
    pub unsafe fn GetILFunctionBodyAllocator(&self, moduleid: usize) -> ::windows::core::Result<IMethodMalloc> {
        let mut result__ = ::windows::core::zeroed::<IMethodMalloc>();
        (::windows::core::Interface::vtable(self).GetILFunctionBodyAllocator)(::windows::core::Interface::as_raw(self), moduleid, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetILFunctionBody(&self, moduleid: usize, methodid: u32, pbnewilmethodheader: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetILFunctionBody)(::windows::core::Interface::as_raw(self), moduleid, methodid, pbnewilmethodheader).ok()
    }
    pub unsafe fn GetAppDomainInfo(&self, appdomainid: usize, pcchname: *mut u32, szname: &mut [u16], pprocessid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAppDomainInfo)(::windows::core::Interface::as_raw(self), appdomainid, szname.len() as _, pcchname, ::core::mem::transmute(szname.as_ptr()), pprocessid).ok()
    }
    pub unsafe fn GetAssemblyInfo(&self, assemblyid: usize, pcchname: *mut u32, szname: &mut [u16], pappdomainid: *mut usize, pmoduleid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAssemblyInfo)(::windows::core::Interface::as_raw(self), assemblyid, szname.len() as _, pcchname, ::core::mem::transmute(szname.as_ptr()), pappdomainid, pmoduleid).ok()
    }
    pub unsafe fn SetFunctionReJIT(&self, functionid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFunctionReJIT)(::windows::core::Interface::as_raw(self), functionid).ok()
    }
    pub unsafe fn ForceGC(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ForceGC)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetILInstrumentedCodeMap<P0>(&self, functionid: usize, fstartjit: P0, cilmapentries: u32, rgilmapentries: *mut COR_IL_MAP) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetILInstrumentedCodeMap)(::windows::core::Interface::as_raw(self), functionid, fstartjit.into_param().abi(), cilmapentries, rgilmapentries).ok()
    }
    pub unsafe fn GetInprocInspectionInterface(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).GetInprocInspectionInterface)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetInprocInspectionIThisThread(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).GetInprocInspectionIThisThread)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetThreadContext(&self, threadid: usize, pcontextid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetThreadContext)(::windows::core::Interface::as_raw(self), threadid, pcontextid).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BeginInprocDebugging<P0>(&self, fthisthreadonly: P0, pdwprofilercontext: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).BeginInprocDebugging)(::windows::core::Interface::as_raw(self), fthisthreadonly.into_param().abi(), pdwprofilercontext).ok()
    }
    pub unsafe fn EndInprocDebugging(&self, dwprofilercontext: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EndInprocDebugging)(::windows::core::Interface::as_raw(self), dwprofilercontext).ok()
    }
    pub unsafe fn GetILToNativeMapping(&self, functionid: usize, cmap: u32, pcmap: *mut u32, map: *mut COR_DEBUG_IL_TO_NATIVE_MAP) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetILToNativeMapping)(::windows::core::Interface::as_raw(self), functionid, cmap, pcmap, map).ok()
    }
}
::windows::imp::interface_hierarchy!(ICorProfilerInfo, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ICorProfilerInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICorProfilerInfo {}
impl ::core::fmt::Debug for ICorProfilerInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICorProfilerInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICorProfilerInfo {
    type Vtable = ICorProfilerInfo_Vtbl;
}
impl ::core::clone::Clone for ICorProfilerInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICorProfilerInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x28b5557d_3f3f_48b4_90b2_5f9eea2f6c48);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorProfilerInfo_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetClassFromObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objectid: usize, pclassid: *mut usize) -> ::windows::core::HRESULT,
    pub GetClassFromToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduleid: usize, typedef: u32, pclassid: *mut usize) -> ::windows::core::HRESULT,
    pub GetCodeInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, functionid: usize, pstart: *mut *mut u8, pcsize: *mut u32) -> ::windows::core::HRESULT,
    pub GetEventMask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwevents: *mut u32) -> ::windows::core::HRESULT,
    pub GetFunctionFromIP: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ip: *mut u8, pfunctionid: *mut usize) -> ::windows::core::HRESULT,
    pub GetFunctionFromToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduleid: usize, token: u32, pfunctionid: *mut usize) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetHandleFromThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threadid: usize, phthread: *mut super::super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetHandleFromThread: usize,
    pub GetObjectSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objectid: usize, pcsize: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_WinRT_Metadata")]
    pub IsArrayClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, classid: usize, pbaseelemtype: *mut super::super::WinRT::Metadata::CorElementType, pbaseclassid: *mut usize, pcrank: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_WinRT_Metadata"))]
    IsArrayClass: usize,
    pub GetThreadInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threadid: usize, pdwwin32threadid: *mut u32) -> ::windows::core::HRESULT,
    pub GetCurrentThreadID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pthreadid: *mut usize) -> ::windows::core::HRESULT,
    pub GetClassIDInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, classid: usize, pmoduleid: *mut usize, ptypedeftoken: *mut u32) -> ::windows::core::HRESULT,
    pub GetFunctionInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, functionid: usize, pclassid: *mut usize, pmoduleid: *mut usize, ptoken: *mut u32) -> ::windows::core::HRESULT,
    pub SetEventMask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwevents: u32) -> ::windows::core::HRESULT,
    pub SetEnterLeaveFunctionHooks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfuncenter: *mut FunctionEnter, pfuncleave: *mut FunctionLeave, pfunctailcall: *mut FunctionTailcall) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetFunctionIDMapper: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfunc: *mut FunctionIDMapper) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetFunctionIDMapper: usize,
    pub GetTokenAndMetaDataFromFunction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, functionid: usize, riid: *const ::windows::core::GUID, ppimport: *mut *mut ::core::ffi::c_void, ptoken: *mut u32) -> ::windows::core::HRESULT,
    pub GetModuleInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduleid: usize, ppbaseloadaddress: *mut *mut u8, cchname: u32, pcchname: *mut u32, szname: ::windows::core::PWSTR, passemblyid: *mut usize) -> ::windows::core::HRESULT,
    pub GetModuleMetaData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduleid: usize, dwopenflags: u32, riid: *const ::windows::core::GUID, ppout: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetILFunctionBody: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduleid: usize, methodid: u32, ppmethodheader: *mut *mut u8, pcbmethodsize: *mut u32) -> ::windows::core::HRESULT,
    pub GetILFunctionBodyAllocator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduleid: usize, ppmalloc: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetILFunctionBody: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduleid: usize, methodid: u32, pbnewilmethodheader: *mut u8) -> ::windows::core::HRESULT,
    pub GetAppDomainInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appdomainid: usize, cchname: u32, pcchname: *mut u32, szname: ::windows::core::PWSTR, pprocessid: *mut usize) -> ::windows::core::HRESULT,
    pub GetAssemblyInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, assemblyid: usize, cchname: u32, pcchname: *mut u32, szname: ::windows::core::PWSTR, pappdomainid: *mut usize, pmoduleid: *mut usize) -> ::windows::core::HRESULT,
    pub SetFunctionReJIT: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, functionid: usize) -> ::windows::core::HRESULT,
    pub ForceGC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetILInstrumentedCodeMap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, functionid: usize, fstartjit: super::super::super::Foundation::BOOL, cilmapentries: u32, rgilmapentries: *mut COR_IL_MAP) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetILInstrumentedCodeMap: usize,
    pub GetInprocInspectionInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppicd: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetInprocInspectionIThisThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppicd: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetThreadContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threadid: usize, pcontextid: *mut usize) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub BeginInprocDebugging: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fthisthreadonly: super::super::super::Foundation::BOOL, pdwprofilercontext: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BeginInprocDebugging: usize,
    pub EndInprocDebugging: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwprofilercontext: u32) -> ::windows::core::HRESULT,
    pub GetILToNativeMapping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, functionid: usize, cmap: u32, pcmap: *mut u32, map: *mut COR_DEBUG_IL_TO_NATIVE_MAP) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
#[repr(transparent)]
pub struct ICorProfilerInfo2(::windows::core::IUnknown);
impl ICorProfilerInfo2 {
    pub unsafe fn GetClassFromObject(&self, objectid: usize, pclassid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetClassFromObject)(::windows::core::Interface::as_raw(self), objectid, pclassid).ok()
    }
    pub unsafe fn GetClassFromToken(&self, moduleid: usize, typedef: u32, pclassid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetClassFromToken)(::windows::core::Interface::as_raw(self), moduleid, typedef, pclassid).ok()
    }
    pub unsafe fn GetCodeInfo(&self, functionid: usize, pstart: *mut *mut u8, pcsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetCodeInfo)(::windows::core::Interface::as_raw(self), functionid, pstart, pcsize).ok()
    }
    pub unsafe fn GetEventMask(&self, pdwevents: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetEventMask)(::windows::core::Interface::as_raw(self), pdwevents).ok()
    }
    pub unsafe fn GetFunctionFromIP(&self, ip: *mut u8, pfunctionid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetFunctionFromIP)(::windows::core::Interface::as_raw(self), ip, pfunctionid).ok()
    }
    pub unsafe fn GetFunctionFromToken(&self, moduleid: usize, token: u32, pfunctionid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetFunctionFromToken)(::windows::core::Interface::as_raw(self), moduleid, token, pfunctionid).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHandleFromThread(&self, threadid: usize, phthread: *mut super::super::super::Foundation::HANDLE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetHandleFromThread)(::windows::core::Interface::as_raw(self), threadid, phthread).ok()
    }
    pub unsafe fn GetObjectSize(&self, objectid: usize, pcsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetObjectSize)(::windows::core::Interface::as_raw(self), objectid, pcsize).ok()
    }
    #[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
    #[cfg(feature = "Win32_System_WinRT_Metadata")]
    pub unsafe fn IsArrayClass(&self, classid: usize, pbaseelemtype: *mut super::super::WinRT::Metadata::CorElementType, pbaseclassid: *mut usize, pcrank: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.IsArrayClass)(::windows::core::Interface::as_raw(self), classid, pbaseelemtype, pbaseclassid, pcrank).ok()
    }
    pub unsafe fn GetThreadInfo(&self, threadid: usize, pdwwin32threadid: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetThreadInfo)(::windows::core::Interface::as_raw(self), threadid, pdwwin32threadid).ok()
    }
    pub unsafe fn GetCurrentThreadID(&self, pthreadid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetCurrentThreadID)(::windows::core::Interface::as_raw(self), pthreadid).ok()
    }
    pub unsafe fn GetClassIDInfo(&self, classid: usize, pmoduleid: *mut usize, ptypedeftoken: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetClassIDInfo)(::windows::core::Interface::as_raw(self), classid, pmoduleid, ptypedeftoken).ok()
    }
    pub unsafe fn GetFunctionInfo(&self, functionid: usize, pclassid: *mut usize, pmoduleid: *mut usize, ptoken: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetFunctionInfo)(::windows::core::Interface::as_raw(self), functionid, pclassid, pmoduleid, ptoken).ok()
    }
    pub unsafe fn SetEventMask(&self, dwevents: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetEventMask)(::windows::core::Interface::as_raw(self), dwevents).ok()
    }
    pub unsafe fn SetEnterLeaveFunctionHooks(&self, pfuncenter: *mut FunctionEnter, pfuncleave: *mut FunctionLeave, pfunctailcall: *mut FunctionTailcall) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetEnterLeaveFunctionHooks)(::windows::core::Interface::as_raw(self), pfuncenter, pfuncleave, pfunctailcall).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFunctionIDMapper(&self, pfunc: *mut FunctionIDMapper) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetFunctionIDMapper)(::windows::core::Interface::as_raw(self), pfunc).ok()
    }
    pub unsafe fn GetTokenAndMetaDataFromFunction(&self, functionid: usize, riid: *const ::windows::core::GUID, ppimport: *mut ::core::option::Option<::windows::core::IUnknown>, ptoken: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetTokenAndMetaDataFromFunction)(::windows::core::Interface::as_raw(self), functionid, riid, ::core::mem::transmute(ppimport), ptoken).ok()
    }
    pub unsafe fn GetModuleInfo(&self, moduleid: usize, ppbaseloadaddress: *mut *mut u8, pcchname: *mut u32, szname: &mut [u16], passemblyid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetModuleInfo)(::windows::core::Interface::as_raw(self), moduleid, ppbaseloadaddress, szname.len() as _, pcchname, ::core::mem::transmute(szname.as_ptr()), passemblyid).ok()
    }
    pub unsafe fn GetModuleMetaData(&self, moduleid: usize, dwopenflags: u32, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).base__.GetModuleMetaData)(::windows::core::Interface::as_raw(self), moduleid, dwopenflags, riid, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetILFunctionBody(&self, moduleid: usize, methodid: u32, ppmethodheader: *mut *mut u8, pcbmethodsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetILFunctionBody)(::windows::core::Interface::as_raw(self), moduleid, methodid, ppmethodheader, pcbmethodsize).ok()
    }
    pub unsafe fn GetILFunctionBodyAllocator(&self, moduleid: usize) -> ::windows::core::Result<IMethodMalloc> {
        let mut result__ = ::windows::core::zeroed::<IMethodMalloc>();
        (::windows::core::Interface::vtable(self).base__.GetILFunctionBodyAllocator)(::windows::core::Interface::as_raw(self), moduleid, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetILFunctionBody(&self, moduleid: usize, methodid: u32, pbnewilmethodheader: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetILFunctionBody)(::windows::core::Interface::as_raw(self), moduleid, methodid, pbnewilmethodheader).ok()
    }
    pub unsafe fn GetAppDomainInfo(&self, appdomainid: usize, pcchname: *mut u32, szname: &mut [u16], pprocessid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetAppDomainInfo)(::windows::core::Interface::as_raw(self), appdomainid, szname.len() as _, pcchname, ::core::mem::transmute(szname.as_ptr()), pprocessid).ok()
    }
    pub unsafe fn GetAssemblyInfo(&self, assemblyid: usize, pcchname: *mut u32, szname: &mut [u16], pappdomainid: *mut usize, pmoduleid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetAssemblyInfo)(::windows::core::Interface::as_raw(self), assemblyid, szname.len() as _, pcchname, ::core::mem::transmute(szname.as_ptr()), pappdomainid, pmoduleid).ok()
    }
    pub unsafe fn SetFunctionReJIT(&self, functionid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetFunctionReJIT)(::windows::core::Interface::as_raw(self), functionid).ok()
    }
    pub unsafe fn ForceGC(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ForceGC)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetILInstrumentedCodeMap<P0>(&self, functionid: usize, fstartjit: P0, cilmapentries: u32, rgilmapentries: *mut COR_IL_MAP) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.SetILInstrumentedCodeMap)(::windows::core::Interface::as_raw(self), functionid, fstartjit.into_param().abi(), cilmapentries, rgilmapentries).ok()
    }
    pub unsafe fn GetInprocInspectionInterface(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).base__.GetInprocInspectionInterface)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetInprocInspectionIThisThread(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).base__.GetInprocInspectionIThisThread)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetThreadContext(&self, threadid: usize, pcontextid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetThreadContext)(::windows::core::Interface::as_raw(self), threadid, pcontextid).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BeginInprocDebugging<P0>(&self, fthisthreadonly: P0, pdwprofilercontext: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.BeginInprocDebugging)(::windows::core::Interface::as_raw(self), fthisthreadonly.into_param().abi(), pdwprofilercontext).ok()
    }
    pub unsafe fn EndInprocDebugging(&self, dwprofilercontext: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.EndInprocDebugging)(::windows::core::Interface::as_raw(self), dwprofilercontext).ok()
    }
    pub unsafe fn GetILToNativeMapping(&self, functionid: usize, cmap: u32, pcmap: *mut u32, map: *mut COR_DEBUG_IL_TO_NATIVE_MAP) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetILToNativeMapping)(::windows::core::Interface::as_raw(self), functionid, cmap, pcmap, map).ok()
    }
    pub unsafe fn DoStackSnapshot(&self, thread: usize, callback: *mut StackSnapshotCallback, infoflags: u32, clientdata: *mut ::core::ffi::c_void, context: *mut u8, contextsize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DoStackSnapshot)(::windows::core::Interface::as_raw(self), thread, callback, infoflags, clientdata, context, contextsize).ok()
    }
    pub unsafe fn SetEnterLeaveFunctionHooks2(&self, pfuncenter: *mut FunctionEnter2, pfuncleave: *mut FunctionLeave2, pfunctailcall: *mut FunctionTailcall2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEnterLeaveFunctionHooks2)(::windows::core::Interface::as_raw(self), pfuncenter, pfuncleave, pfunctailcall).ok()
    }
    pub unsafe fn GetFunctionInfo2(&self, funcid: usize, frameinfo: usize, pclassid: *mut usize, pmoduleid: *mut usize, ptoken: *mut u32, ctypeargs: u32, pctypeargs: *mut u32, typeargs: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFunctionInfo2)(::windows::core::Interface::as_raw(self), funcid, frameinfo, pclassid, pmoduleid, ptoken, ctypeargs, pctypeargs, typeargs).ok()
    }
    pub unsafe fn GetStringLayout(&self, pbufferlengthoffset: *mut u32, pstringlengthoffset: *mut u32, pbufferoffset: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetStringLayout)(::windows::core::Interface::as_raw(self), pbufferlengthoffset, pstringlengthoffset, pbufferoffset).ok()
    }
    #[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
    #[cfg(feature = "Win32_System_WinRT_Metadata")]
    pub unsafe fn GetClassLayout(&self, classid: usize, rfieldoffset: *mut super::super::WinRT::Metadata::COR_FIELD_OFFSET, cfieldoffset: u32, pcfieldoffset: *mut u32, pulclasssize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetClassLayout)(::windows::core::Interface::as_raw(self), classid, rfieldoffset, cfieldoffset, pcfieldoffset, pulclasssize).ok()
    }
    pub unsafe fn GetClassIDInfo2(&self, classid: usize, pmoduleid: *mut usize, ptypedeftoken: *mut u32, pparentclassid: *mut usize, cnumtypeargs: u32, pcnumtypeargs: *mut u32, typeargs: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetClassIDInfo2)(::windows::core::Interface::as_raw(self), classid, pmoduleid, ptypedeftoken, pparentclassid, cnumtypeargs, pcnumtypeargs, typeargs).ok()
    }
    pub unsafe fn GetCodeInfo2(&self, functionid: usize, ccodeinfos: u32, pccodeinfos: *mut u32, codeinfos: *mut COR_PRF_CODE_INFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCodeInfo2)(::windows::core::Interface::as_raw(self), functionid, ccodeinfos, pccodeinfos, codeinfos).ok()
    }
    pub unsafe fn GetClassFromTokenAndTypeArgs(&self, moduleid: usize, typedef: u32, ctypeargs: u32, typeargs: *mut usize, pclassid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetClassFromTokenAndTypeArgs)(::windows::core::Interface::as_raw(self), moduleid, typedef, ctypeargs, typeargs, pclassid).ok()
    }
    pub unsafe fn GetFunctionFromTokenAndTypeArgs(&self, moduleid: usize, funcdef: u32, classid: usize, ctypeargs: u32, typeargs: *mut usize, pfunctionid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFunctionFromTokenAndTypeArgs)(::windows::core::Interface::as_raw(self), moduleid, funcdef, classid, ctypeargs, typeargs, pfunctionid).ok()
    }
    pub unsafe fn EnumModuleFrozenObjects(&self, moduleid: usize) -> ::windows::core::Result<ICorProfilerObjectEnum> {
        let mut result__ = ::windows::core::zeroed::<ICorProfilerObjectEnum>();
        (::windows::core::Interface::vtable(self).EnumModuleFrozenObjects)(::windows::core::Interface::as_raw(self), moduleid, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetArrayObjectInfo(&self, objectid: usize, cdimensions: u32, pdimensionsizes: *mut u32, pdimensionlowerbounds: *mut i32, ppdata: *mut *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetArrayObjectInfo)(::windows::core::Interface::as_raw(self), objectid, cdimensions, pdimensionsizes, pdimensionlowerbounds, ppdata).ok()
    }
    pub unsafe fn GetBoxClassLayout(&self, classid: usize, pbufferoffset: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetBoxClassLayout)(::windows::core::Interface::as_raw(self), classid, pbufferoffset).ok()
    }
    pub unsafe fn GetThreadAppDomain(&self, threadid: usize, pappdomainid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetThreadAppDomain)(::windows::core::Interface::as_raw(self), threadid, pappdomainid).ok()
    }
    pub unsafe fn GetRVAStaticAddress(&self, classid: usize, fieldtoken: u32, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetRVAStaticAddress)(::windows::core::Interface::as_raw(self), classid, fieldtoken, ppaddress).ok()
    }
    pub unsafe fn GetAppDomainStaticAddress(&self, classid: usize, fieldtoken: u32, appdomainid: usize, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAppDomainStaticAddress)(::windows::core::Interface::as_raw(self), classid, fieldtoken, appdomainid, ppaddress).ok()
    }
    pub unsafe fn GetThreadStaticAddress(&self, classid: usize, fieldtoken: u32, threadid: usize, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetThreadStaticAddress)(::windows::core::Interface::as_raw(self), classid, fieldtoken, threadid, ppaddress).ok()
    }
    pub unsafe fn GetContextStaticAddress(&self, classid: usize, fieldtoken: u32, contextid: usize, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetContextStaticAddress)(::windows::core::Interface::as_raw(self), classid, fieldtoken, contextid, ppaddress).ok()
    }
    pub unsafe fn GetStaticFieldInfo(&self, classid: usize, fieldtoken: u32, pfieldinfo: *mut COR_PRF_STATIC_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetStaticFieldInfo)(::windows::core::Interface::as_raw(self), classid, fieldtoken, pfieldinfo).ok()
    }
    pub unsafe fn GetGenerationBounds(&self, cobjectranges: u32, pcobjectranges: *mut u32, ranges: *mut COR_PRF_GC_GENERATION_RANGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetGenerationBounds)(::windows::core::Interface::as_raw(self), cobjectranges, pcobjectranges, ranges).ok()
    }
    pub unsafe fn GetObjectGeneration(&self, objectid: usize, range: *mut COR_PRF_GC_GENERATION_RANGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetObjectGeneration)(::windows::core::Interface::as_raw(self), objectid, range).ok()
    }
    pub unsafe fn GetNotifiedExceptionClauseInfo(&self, pinfo: *mut COR_PRF_EX_CLAUSE_INFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetNotifiedExceptionClauseInfo)(::windows::core::Interface::as_raw(self), pinfo).ok()
    }
}
::windows::imp::interface_hierarchy!(ICorProfilerInfo2, ::windows::core::IUnknown, ICorProfilerInfo);
impl ::core::cmp::PartialEq for ICorProfilerInfo2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICorProfilerInfo2 {}
impl ::core::fmt::Debug for ICorProfilerInfo2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICorProfilerInfo2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICorProfilerInfo2 {
    type Vtable = ICorProfilerInfo2_Vtbl;
}
impl ::core::clone::Clone for ICorProfilerInfo2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICorProfilerInfo2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcc0935cd_a518_487d_b0bb_a93214e65478);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorProfilerInfo2_Vtbl {
    pub base__: ICorProfilerInfo_Vtbl,
    pub DoStackSnapshot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, thread: usize, callback: *mut StackSnapshotCallback, infoflags: u32, clientdata: *mut ::core::ffi::c_void, context: *mut u8, contextsize: u32) -> ::windows::core::HRESULT,
    pub SetEnterLeaveFunctionHooks2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfuncenter: *mut FunctionEnter2, pfuncleave: *mut FunctionLeave2, pfunctailcall: *mut FunctionTailcall2) -> ::windows::core::HRESULT,
    pub GetFunctionInfo2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, funcid: usize, frameinfo: usize, pclassid: *mut usize, pmoduleid: *mut usize, ptoken: *mut u32, ctypeargs: u32, pctypeargs: *mut u32, typeargs: *mut usize) -> ::windows::core::HRESULT,
    pub GetStringLayout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbufferlengthoffset: *mut u32, pstringlengthoffset: *mut u32, pbufferoffset: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_WinRT_Metadata")]
    pub GetClassLayout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, classid: usize, rfieldoffset: *mut super::super::WinRT::Metadata::COR_FIELD_OFFSET, cfieldoffset: u32, pcfieldoffset: *mut u32, pulclasssize: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_WinRT_Metadata"))]
    GetClassLayout: usize,
    pub GetClassIDInfo2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, classid: usize, pmoduleid: *mut usize, ptypedeftoken: *mut u32, pparentclassid: *mut usize, cnumtypeargs: u32, pcnumtypeargs: *mut u32, typeargs: *mut usize) -> ::windows::core::HRESULT,
    pub GetCodeInfo2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, functionid: usize, ccodeinfos: u32, pccodeinfos: *mut u32, codeinfos: *mut COR_PRF_CODE_INFO) -> ::windows::core::HRESULT,
    pub GetClassFromTokenAndTypeArgs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduleid: usize, typedef: u32, ctypeargs: u32, typeargs: *mut usize, pclassid: *mut usize) -> ::windows::core::HRESULT,
    pub GetFunctionFromTokenAndTypeArgs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduleid: usize, funcdef: u32, classid: usize, ctypeargs: u32, typeargs: *mut usize, pfunctionid: *mut usize) -> ::windows::core::HRESULT,
    pub EnumModuleFrozenObjects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduleid: usize, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetArrayObjectInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objectid: usize, cdimensions: u32, pdimensionsizes: *mut u32, pdimensionlowerbounds: *mut i32, ppdata: *mut *mut u8) -> ::windows::core::HRESULT,
    pub GetBoxClassLayout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, classid: usize, pbufferoffset: *mut u32) -> ::windows::core::HRESULT,
    pub GetThreadAppDomain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threadid: usize, pappdomainid: *mut usize) -> ::windows::core::HRESULT,
    pub GetRVAStaticAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, classid: usize, fieldtoken: u32, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetAppDomainStaticAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, classid: usize, fieldtoken: u32, appdomainid: usize, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetThreadStaticAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, classid: usize, fieldtoken: u32, threadid: usize, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetContextStaticAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, classid: usize, fieldtoken: u32, contextid: usize, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetStaticFieldInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, classid: usize, fieldtoken: u32, pfieldinfo: *mut COR_PRF_STATIC_TYPE) -> ::windows::core::HRESULT,
    pub GetGenerationBounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cobjectranges: u32, pcobjectranges: *mut u32, ranges: *mut COR_PRF_GC_GENERATION_RANGE) -> ::windows::core::HRESULT,
    pub GetObjectGeneration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objectid: usize, range: *mut COR_PRF_GC_GENERATION_RANGE) -> ::windows::core::HRESULT,
    pub GetNotifiedExceptionClauseInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *mut COR_PRF_EX_CLAUSE_INFO) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
#[repr(transparent)]
pub struct ICorProfilerInfo3(::windows::core::IUnknown);
impl ICorProfilerInfo3 {
    pub unsafe fn GetClassFromObject(&self, objectid: usize, pclassid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetClassFromObject)(::windows::core::Interface::as_raw(self), objectid, pclassid).ok()
    }
    pub unsafe fn GetClassFromToken(&self, moduleid: usize, typedef: u32, pclassid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetClassFromToken)(::windows::core::Interface::as_raw(self), moduleid, typedef, pclassid).ok()
    }
    pub unsafe fn GetCodeInfo(&self, functionid: usize, pstart: *mut *mut u8, pcsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetCodeInfo)(::windows::core::Interface::as_raw(self), functionid, pstart, pcsize).ok()
    }
    pub unsafe fn GetEventMask(&self, pdwevents: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetEventMask)(::windows::core::Interface::as_raw(self), pdwevents).ok()
    }
    pub unsafe fn GetFunctionFromIP(&self, ip: *mut u8, pfunctionid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetFunctionFromIP)(::windows::core::Interface::as_raw(self), ip, pfunctionid).ok()
    }
    pub unsafe fn GetFunctionFromToken(&self, moduleid: usize, token: u32, pfunctionid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetFunctionFromToken)(::windows::core::Interface::as_raw(self), moduleid, token, pfunctionid).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHandleFromThread(&self, threadid: usize, phthread: *mut super::super::super::Foundation::HANDLE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetHandleFromThread)(::windows::core::Interface::as_raw(self), threadid, phthread).ok()
    }
    pub unsafe fn GetObjectSize(&self, objectid: usize, pcsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetObjectSize)(::windows::core::Interface::as_raw(self), objectid, pcsize).ok()
    }
    #[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
    #[cfg(feature = "Win32_System_WinRT_Metadata")]
    pub unsafe fn IsArrayClass(&self, classid: usize, pbaseelemtype: *mut super::super::WinRT::Metadata::CorElementType, pbaseclassid: *mut usize, pcrank: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.IsArrayClass)(::windows::core::Interface::as_raw(self), classid, pbaseelemtype, pbaseclassid, pcrank).ok()
    }
    pub unsafe fn GetThreadInfo(&self, threadid: usize, pdwwin32threadid: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetThreadInfo)(::windows::core::Interface::as_raw(self), threadid, pdwwin32threadid).ok()
    }
    pub unsafe fn GetCurrentThreadID(&self, pthreadid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetCurrentThreadID)(::windows::core::Interface::as_raw(self), pthreadid).ok()
    }
    pub unsafe fn GetClassIDInfo(&self, classid: usize, pmoduleid: *mut usize, ptypedeftoken: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetClassIDInfo)(::windows::core::Interface::as_raw(self), classid, pmoduleid, ptypedeftoken).ok()
    }
    pub unsafe fn GetFunctionInfo(&self, functionid: usize, pclassid: *mut usize, pmoduleid: *mut usize, ptoken: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetFunctionInfo)(::windows::core::Interface::as_raw(self), functionid, pclassid, pmoduleid, ptoken).ok()
    }
    pub unsafe fn SetEventMask(&self, dwevents: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetEventMask)(::windows::core::Interface::as_raw(self), dwevents).ok()
    }
    pub unsafe fn SetEnterLeaveFunctionHooks(&self, pfuncenter: *mut FunctionEnter, pfuncleave: *mut FunctionLeave, pfunctailcall: *mut FunctionTailcall) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetEnterLeaveFunctionHooks)(::windows::core::Interface::as_raw(self), pfuncenter, pfuncleave, pfunctailcall).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFunctionIDMapper(&self, pfunc: *mut FunctionIDMapper) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetFunctionIDMapper)(::windows::core::Interface::as_raw(self), pfunc).ok()
    }
    pub unsafe fn GetTokenAndMetaDataFromFunction(&self, functionid: usize, riid: *const ::windows::core::GUID, ppimport: *mut ::core::option::Option<::windows::core::IUnknown>, ptoken: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetTokenAndMetaDataFromFunction)(::windows::core::Interface::as_raw(self), functionid, riid, ::core::mem::transmute(ppimport), ptoken).ok()
    }
    pub unsafe fn GetModuleInfo(&self, moduleid: usize, ppbaseloadaddress: *mut *mut u8, pcchname: *mut u32, szname: &mut [u16], passemblyid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetModuleInfo)(::windows::core::Interface::as_raw(self), moduleid, ppbaseloadaddress, szname.len() as _, pcchname, ::core::mem::transmute(szname.as_ptr()), passemblyid).ok()
    }
    pub unsafe fn GetModuleMetaData(&self, moduleid: usize, dwopenflags: u32, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).base__.base__.GetModuleMetaData)(::windows::core::Interface::as_raw(self), moduleid, dwopenflags, riid, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetILFunctionBody(&self, moduleid: usize, methodid: u32, ppmethodheader: *mut *mut u8, pcbmethodsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetILFunctionBody)(::windows::core::Interface::as_raw(self), moduleid, methodid, ppmethodheader, pcbmethodsize).ok()
    }
    pub unsafe fn GetILFunctionBodyAllocator(&self, moduleid: usize) -> ::windows::core::Result<IMethodMalloc> {
        let mut result__ = ::windows::core::zeroed::<IMethodMalloc>();
        (::windows::core::Interface::vtable(self).base__.base__.GetILFunctionBodyAllocator)(::windows::core::Interface::as_raw(self), moduleid, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetILFunctionBody(&self, moduleid: usize, methodid: u32, pbnewilmethodheader: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetILFunctionBody)(::windows::core::Interface::as_raw(self), moduleid, methodid, pbnewilmethodheader).ok()
    }
    pub unsafe fn GetAppDomainInfo(&self, appdomainid: usize, pcchname: *mut u32, szname: &mut [u16], pprocessid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetAppDomainInfo)(::windows::core::Interface::as_raw(self), appdomainid, szname.len() as _, pcchname, ::core::mem::transmute(szname.as_ptr()), pprocessid).ok()
    }
    pub unsafe fn GetAssemblyInfo(&self, assemblyid: usize, pcchname: *mut u32, szname: &mut [u16], pappdomainid: *mut usize, pmoduleid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetAssemblyInfo)(::windows::core::Interface::as_raw(self), assemblyid, szname.len() as _, pcchname, ::core::mem::transmute(szname.as_ptr()), pappdomainid, pmoduleid).ok()
    }
    pub unsafe fn SetFunctionReJIT(&self, functionid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetFunctionReJIT)(::windows::core::Interface::as_raw(self), functionid).ok()
    }
    pub unsafe fn ForceGC(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.ForceGC)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetILInstrumentedCodeMap<P0>(&self, functionid: usize, fstartjit: P0, cilmapentries: u32, rgilmapentries: *mut COR_IL_MAP) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetILInstrumentedCodeMap)(::windows::core::Interface::as_raw(self), functionid, fstartjit.into_param().abi(), cilmapentries, rgilmapentries).ok()
    }
    pub unsafe fn GetInprocInspectionInterface(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).base__.base__.GetInprocInspectionInterface)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetInprocInspectionIThisThread(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).base__.base__.GetInprocInspectionIThisThread)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetThreadContext(&self, threadid: usize, pcontextid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetThreadContext)(::windows::core::Interface::as_raw(self), threadid, pcontextid).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BeginInprocDebugging<P0>(&self, fthisthreadonly: P0, pdwprofilercontext: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.BeginInprocDebugging)(::windows::core::Interface::as_raw(self), fthisthreadonly.into_param().abi(), pdwprofilercontext).ok()
    }
    pub unsafe fn EndInprocDebugging(&self, dwprofilercontext: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.EndInprocDebugging)(::windows::core::Interface::as_raw(self), dwprofilercontext).ok()
    }
    pub unsafe fn GetILToNativeMapping(&self, functionid: usize, cmap: u32, pcmap: *mut u32, map: *mut COR_DEBUG_IL_TO_NATIVE_MAP) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetILToNativeMapping)(::windows::core::Interface::as_raw(self), functionid, cmap, pcmap, map).ok()
    }
    pub unsafe fn DoStackSnapshot(&self, thread: usize, callback: *mut StackSnapshotCallback, infoflags: u32, clientdata: *mut ::core::ffi::c_void, context: *mut u8, contextsize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.DoStackSnapshot)(::windows::core::Interface::as_raw(self), thread, callback, infoflags, clientdata, context, contextsize).ok()
    }
    pub unsafe fn SetEnterLeaveFunctionHooks2(&self, pfuncenter: *mut FunctionEnter2, pfuncleave: *mut FunctionLeave2, pfunctailcall: *mut FunctionTailcall2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetEnterLeaveFunctionHooks2)(::windows::core::Interface::as_raw(self), pfuncenter, pfuncleave, pfunctailcall).ok()
    }
    pub unsafe fn GetFunctionInfo2(&self, funcid: usize, frameinfo: usize, pclassid: *mut usize, pmoduleid: *mut usize, ptoken: *mut u32, ctypeargs: u32, pctypeargs: *mut u32, typeargs: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetFunctionInfo2)(::windows::core::Interface::as_raw(self), funcid, frameinfo, pclassid, pmoduleid, ptoken, ctypeargs, pctypeargs, typeargs).ok()
    }
    pub unsafe fn GetStringLayout(&self, pbufferlengthoffset: *mut u32, pstringlengthoffset: *mut u32, pbufferoffset: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetStringLayout)(::windows::core::Interface::as_raw(self), pbufferlengthoffset, pstringlengthoffset, pbufferoffset).ok()
    }
    #[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
    #[cfg(feature = "Win32_System_WinRT_Metadata")]
    pub unsafe fn GetClassLayout(&self, classid: usize, rfieldoffset: *mut super::super::WinRT::Metadata::COR_FIELD_OFFSET, cfieldoffset: u32, pcfieldoffset: *mut u32, pulclasssize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetClassLayout)(::windows::core::Interface::as_raw(self), classid, rfieldoffset, cfieldoffset, pcfieldoffset, pulclasssize).ok()
    }
    pub unsafe fn GetClassIDInfo2(&self, classid: usize, pmoduleid: *mut usize, ptypedeftoken: *mut u32, pparentclassid: *mut usize, cnumtypeargs: u32, pcnumtypeargs: *mut u32, typeargs: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetClassIDInfo2)(::windows::core::Interface::as_raw(self), classid, pmoduleid, ptypedeftoken, pparentclassid, cnumtypeargs, pcnumtypeargs, typeargs).ok()
    }
    pub unsafe fn GetCodeInfo2(&self, functionid: usize, ccodeinfos: u32, pccodeinfos: *mut u32, codeinfos: *mut COR_PRF_CODE_INFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetCodeInfo2)(::windows::core::Interface::as_raw(self), functionid, ccodeinfos, pccodeinfos, codeinfos).ok()
    }
    pub unsafe fn GetClassFromTokenAndTypeArgs(&self, moduleid: usize, typedef: u32, ctypeargs: u32, typeargs: *mut usize, pclassid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetClassFromTokenAndTypeArgs)(::windows::core::Interface::as_raw(self), moduleid, typedef, ctypeargs, typeargs, pclassid).ok()
    }
    pub unsafe fn GetFunctionFromTokenAndTypeArgs(&self, moduleid: usize, funcdef: u32, classid: usize, ctypeargs: u32, typeargs: *mut usize, pfunctionid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetFunctionFromTokenAndTypeArgs)(::windows::core::Interface::as_raw(self), moduleid, funcdef, classid, ctypeargs, typeargs, pfunctionid).ok()
    }
    pub unsafe fn EnumModuleFrozenObjects(&self, moduleid: usize) -> ::windows::core::Result<ICorProfilerObjectEnum> {
        let mut result__ = ::windows::core::zeroed::<ICorProfilerObjectEnum>();
        (::windows::core::Interface::vtable(self).base__.EnumModuleFrozenObjects)(::windows::core::Interface::as_raw(self), moduleid, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetArrayObjectInfo(&self, objectid: usize, cdimensions: u32, pdimensionsizes: *mut u32, pdimensionlowerbounds: *mut i32, ppdata: *mut *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetArrayObjectInfo)(::windows::core::Interface::as_raw(self), objectid, cdimensions, pdimensionsizes, pdimensionlowerbounds, ppdata).ok()
    }
    pub unsafe fn GetBoxClassLayout(&self, classid: usize, pbufferoffset: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetBoxClassLayout)(::windows::core::Interface::as_raw(self), classid, pbufferoffset).ok()
    }
    pub unsafe fn GetThreadAppDomain(&self, threadid: usize, pappdomainid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetThreadAppDomain)(::windows::core::Interface::as_raw(self), threadid, pappdomainid).ok()
    }
    pub unsafe fn GetRVAStaticAddress(&self, classid: usize, fieldtoken: u32, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetRVAStaticAddress)(::windows::core::Interface::as_raw(self), classid, fieldtoken, ppaddress).ok()
    }
    pub unsafe fn GetAppDomainStaticAddress(&self, classid: usize, fieldtoken: u32, appdomainid: usize, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetAppDomainStaticAddress)(::windows::core::Interface::as_raw(self), classid, fieldtoken, appdomainid, ppaddress).ok()
    }
    pub unsafe fn GetThreadStaticAddress(&self, classid: usize, fieldtoken: u32, threadid: usize, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetThreadStaticAddress)(::windows::core::Interface::as_raw(self), classid, fieldtoken, threadid, ppaddress).ok()
    }
    pub unsafe fn GetContextStaticAddress(&self, classid: usize, fieldtoken: u32, contextid: usize, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetContextStaticAddress)(::windows::core::Interface::as_raw(self), classid, fieldtoken, contextid, ppaddress).ok()
    }
    pub unsafe fn GetStaticFieldInfo(&self, classid: usize, fieldtoken: u32, pfieldinfo: *mut COR_PRF_STATIC_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetStaticFieldInfo)(::windows::core::Interface::as_raw(self), classid, fieldtoken, pfieldinfo).ok()
    }
    pub unsafe fn GetGenerationBounds(&self, cobjectranges: u32, pcobjectranges: *mut u32, ranges: *mut COR_PRF_GC_GENERATION_RANGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetGenerationBounds)(::windows::core::Interface::as_raw(self), cobjectranges, pcobjectranges, ranges).ok()
    }
    pub unsafe fn GetObjectGeneration(&self, objectid: usize, range: *mut COR_PRF_GC_GENERATION_RANGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetObjectGeneration)(::windows::core::Interface::as_raw(self), objectid, range).ok()
    }
    pub unsafe fn GetNotifiedExceptionClauseInfo(&self, pinfo: *mut COR_PRF_EX_CLAUSE_INFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetNotifiedExceptionClauseInfo)(::windows::core::Interface::as_raw(self), pinfo).ok()
    }
    pub unsafe fn EnumJITedFunctions(&self) -> ::windows::core::Result<ICorProfilerFunctionEnum> {
        let mut result__ = ::windows::core::zeroed::<ICorProfilerFunctionEnum>();
        (::windows::core::Interface::vtable(self).EnumJITedFunctions)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn RequestProfilerDetach(&self, dwexpectedcompletionmilliseconds: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RequestProfilerDetach)(::windows::core::Interface::as_raw(self), dwexpectedcompletionmilliseconds).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFunctionIDMapper2(&self, pfunc: *mut FunctionIDMapper2, clientdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFunctionIDMapper2)(::windows::core::Interface::as_raw(self), pfunc, clientdata).ok()
    }
    pub unsafe fn GetStringLayout2(&self, pstringlengthoffset: *mut u32, pbufferoffset: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetStringLayout2)(::windows::core::Interface::as_raw(self), pstringlengthoffset, pbufferoffset).ok()
    }
    pub unsafe fn SetEnterLeaveFunctionHooks3(&self, pfuncenter3: *mut FunctionEnter3, pfuncleave3: *mut FunctionLeave3, pfunctailcall3: *mut FunctionTailcall3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEnterLeaveFunctionHooks3)(::windows::core::Interface::as_raw(self), pfuncenter3, pfuncleave3, pfunctailcall3).ok()
    }
    pub unsafe fn SetEnterLeaveFunctionHooks3WithInfo(&self, pfuncenter3withinfo: *mut FunctionEnter3WithInfo, pfuncleave3withinfo: *mut FunctionLeave3WithInfo, pfunctailcall3withinfo: *mut FunctionTailcall3WithInfo) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEnterLeaveFunctionHooks3WithInfo)(::windows::core::Interface::as_raw(self), pfuncenter3withinfo, pfuncleave3withinfo, pfunctailcall3withinfo).ok()
    }
    pub unsafe fn GetFunctionEnter3Info(&self, functionid: usize, eltinfo: usize, pframeinfo: *mut usize, pcbargumentinfo: *mut u32, pargumentinfo: *mut COR_PRF_FUNCTION_ARGUMENT_INFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFunctionEnter3Info)(::windows::core::Interface::as_raw(self), functionid, eltinfo, pframeinfo, pcbargumentinfo, pargumentinfo).ok()
    }
    pub unsafe fn GetFunctionLeave3Info(&self, functionid: usize, eltinfo: usize, pframeinfo: *mut usize, pretvalrange: *mut COR_PRF_FUNCTION_ARGUMENT_RANGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFunctionLeave3Info)(::windows::core::Interface::as_raw(self), functionid, eltinfo, pframeinfo, pretvalrange).ok()
    }
    pub unsafe fn GetFunctionTailcall3Info(&self, functionid: usize, eltinfo: usize, pframeinfo: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFunctionTailcall3Info)(::windows::core::Interface::as_raw(self), functionid, eltinfo, pframeinfo).ok()
    }
    pub unsafe fn EnumModules(&self) -> ::windows::core::Result<ICorProfilerModuleEnum> {
        let mut result__ = ::windows::core::zeroed::<ICorProfilerModuleEnum>();
        (::windows::core::Interface::vtable(self).EnumModules)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetRuntimeInformation(&self, pclrinstanceid: *mut u16, pruntimetype: *mut COR_PRF_RUNTIME_TYPE, pmajorversion: *mut u16, pminorversion: *mut u16, pbuildnumber: *mut u16, pqfeversion: *mut u16, pcchversionstring: *mut u32, szversionstring: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetRuntimeInformation)(::windows::core::Interface::as_raw(self), pclrinstanceid, pruntimetype, pmajorversion, pminorversion, pbuildnumber, pqfeversion, szversionstring.len() as _, pcchversionstring, ::core::mem::transmute(szversionstring.as_ptr())).ok()
    }
    pub unsafe fn GetThreadStaticAddress2(&self, classid: usize, fieldtoken: u32, appdomainid: usize, threadid: usize, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetThreadStaticAddress2)(::windows::core::Interface::as_raw(self), classid, fieldtoken, appdomainid, threadid, ppaddress).ok()
    }
    pub unsafe fn GetAppDomainsContainingModule(&self, moduleid: usize, cappdomainids: u32, pcappdomainids: *mut u32, appdomainids: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAppDomainsContainingModule)(::windows::core::Interface::as_raw(self), moduleid, cappdomainids, pcappdomainids, appdomainids).ok()
    }
    pub unsafe fn GetModuleInfo2(&self, moduleid: usize, ppbaseloadaddress: *mut *mut u8, pcchname: *mut u32, szname: &mut [u16], passemblyid: *mut usize, pdwmoduleflags: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetModuleInfo2)(::windows::core::Interface::as_raw(self), moduleid, ppbaseloadaddress, szname.len() as _, pcchname, ::core::mem::transmute(szname.as_ptr()), passemblyid, pdwmoduleflags).ok()
    }
}
::windows::imp::interface_hierarchy!(ICorProfilerInfo3, ::windows::core::IUnknown, ICorProfilerInfo, ICorProfilerInfo2);
impl ::core::cmp::PartialEq for ICorProfilerInfo3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICorProfilerInfo3 {}
impl ::core::fmt::Debug for ICorProfilerInfo3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICorProfilerInfo3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICorProfilerInfo3 {
    type Vtable = ICorProfilerInfo3_Vtbl;
}
impl ::core::clone::Clone for ICorProfilerInfo3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICorProfilerInfo3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb555ed4f_452a_4e54_8b39_b5360bad32a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorProfilerInfo3_Vtbl {
    pub base__: ICorProfilerInfo2_Vtbl,
    pub EnumJITedFunctions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RequestProfilerDetach: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwexpectedcompletionmilliseconds: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetFunctionIDMapper2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfunc: *mut FunctionIDMapper2, clientdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetFunctionIDMapper2: usize,
    pub GetStringLayout2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstringlengthoffset: *mut u32, pbufferoffset: *mut u32) -> ::windows::core::HRESULT,
    pub SetEnterLeaveFunctionHooks3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfuncenter3: *mut FunctionEnter3, pfuncleave3: *mut FunctionLeave3, pfunctailcall3: *mut FunctionTailcall3) -> ::windows::core::HRESULT,
    pub SetEnterLeaveFunctionHooks3WithInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfuncenter3withinfo: *mut FunctionEnter3WithInfo, pfuncleave3withinfo: *mut FunctionLeave3WithInfo, pfunctailcall3withinfo: *mut FunctionTailcall3WithInfo) -> ::windows::core::HRESULT,
    pub GetFunctionEnter3Info: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, functionid: usize, eltinfo: usize, pframeinfo: *mut usize, pcbargumentinfo: *mut u32, pargumentinfo: *mut COR_PRF_FUNCTION_ARGUMENT_INFO) -> ::windows::core::HRESULT,
    pub GetFunctionLeave3Info: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, functionid: usize, eltinfo: usize, pframeinfo: *mut usize, pretvalrange: *mut COR_PRF_FUNCTION_ARGUMENT_RANGE) -> ::windows::core::HRESULT,
    pub GetFunctionTailcall3Info: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, functionid: usize, eltinfo: usize, pframeinfo: *mut usize) -> ::windows::core::HRESULT,
    pub EnumModules: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetRuntimeInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclrinstanceid: *mut u16, pruntimetype: *mut COR_PRF_RUNTIME_TYPE, pmajorversion: *mut u16, pminorversion: *mut u16, pbuildnumber: *mut u16, pqfeversion: *mut u16, cchversionstring: u32, pcchversionstring: *mut u32, szversionstring: ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetThreadStaticAddress2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, classid: usize, fieldtoken: u32, appdomainid: usize, threadid: usize, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetAppDomainsContainingModule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduleid: usize, cappdomainids: u32, pcappdomainids: *mut u32, appdomainids: *mut usize) -> ::windows::core::HRESULT,
    pub GetModuleInfo2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduleid: usize, ppbaseloadaddress: *mut *mut u8, cchname: u32, pcchname: *mut u32, szname: ::windows::core::PWSTR, passemblyid: *mut usize, pdwmoduleflags: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
#[repr(transparent)]
pub struct ICorProfilerInfo4(::windows::core::IUnknown);
impl ICorProfilerInfo4 {
    pub unsafe fn GetClassFromObject(&self, objectid: usize, pclassid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetClassFromObject)(::windows::core::Interface::as_raw(self), objectid, pclassid).ok()
    }
    pub unsafe fn GetClassFromToken(&self, moduleid: usize, typedef: u32, pclassid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetClassFromToken)(::windows::core::Interface::as_raw(self), moduleid, typedef, pclassid).ok()
    }
    pub unsafe fn GetCodeInfo(&self, functionid: usize, pstart: *mut *mut u8, pcsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetCodeInfo)(::windows::core::Interface::as_raw(self), functionid, pstart, pcsize).ok()
    }
    pub unsafe fn GetEventMask(&self, pdwevents: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetEventMask)(::windows::core::Interface::as_raw(self), pdwevents).ok()
    }
    pub unsafe fn GetFunctionFromIP(&self, ip: *mut u8, pfunctionid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetFunctionFromIP)(::windows::core::Interface::as_raw(self), ip, pfunctionid).ok()
    }
    pub unsafe fn GetFunctionFromToken(&self, moduleid: usize, token: u32, pfunctionid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetFunctionFromToken)(::windows::core::Interface::as_raw(self), moduleid, token, pfunctionid).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHandleFromThread(&self, threadid: usize, phthread: *mut super::super::super::Foundation::HANDLE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetHandleFromThread)(::windows::core::Interface::as_raw(self), threadid, phthread).ok()
    }
    pub unsafe fn GetObjectSize(&self, objectid: usize, pcsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetObjectSize)(::windows::core::Interface::as_raw(self), objectid, pcsize).ok()
    }
    #[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
    #[cfg(feature = "Win32_System_WinRT_Metadata")]
    pub unsafe fn IsArrayClass(&self, classid: usize, pbaseelemtype: *mut super::super::WinRT::Metadata::CorElementType, pbaseclassid: *mut usize, pcrank: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.IsArrayClass)(::windows::core::Interface::as_raw(self), classid, pbaseelemtype, pbaseclassid, pcrank).ok()
    }
    pub unsafe fn GetThreadInfo(&self, threadid: usize, pdwwin32threadid: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetThreadInfo)(::windows::core::Interface::as_raw(self), threadid, pdwwin32threadid).ok()
    }
    pub unsafe fn GetCurrentThreadID(&self, pthreadid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetCurrentThreadID)(::windows::core::Interface::as_raw(self), pthreadid).ok()
    }
    pub unsafe fn GetClassIDInfo(&self, classid: usize, pmoduleid: *mut usize, ptypedeftoken: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetClassIDInfo)(::windows::core::Interface::as_raw(self), classid, pmoduleid, ptypedeftoken).ok()
    }
    pub unsafe fn GetFunctionInfo(&self, functionid: usize, pclassid: *mut usize, pmoduleid: *mut usize, ptoken: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetFunctionInfo)(::windows::core::Interface::as_raw(self), functionid, pclassid, pmoduleid, ptoken).ok()
    }
    pub unsafe fn SetEventMask(&self, dwevents: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetEventMask)(::windows::core::Interface::as_raw(self), dwevents).ok()
    }
    pub unsafe fn SetEnterLeaveFunctionHooks(&self, pfuncenter: *mut FunctionEnter, pfuncleave: *mut FunctionLeave, pfunctailcall: *mut FunctionTailcall) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetEnterLeaveFunctionHooks)(::windows::core::Interface::as_raw(self), pfuncenter, pfuncleave, pfunctailcall).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFunctionIDMapper(&self, pfunc: *mut FunctionIDMapper) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetFunctionIDMapper)(::windows::core::Interface::as_raw(self), pfunc).ok()
    }
    pub unsafe fn GetTokenAndMetaDataFromFunction(&self, functionid: usize, riid: *const ::windows::core::GUID, ppimport: *mut ::core::option::Option<::windows::core::IUnknown>, ptoken: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetTokenAndMetaDataFromFunction)(::windows::core::Interface::as_raw(self), functionid, riid, ::core::mem::transmute(ppimport), ptoken).ok()
    }
    pub unsafe fn GetModuleInfo(&self, moduleid: usize, ppbaseloadaddress: *mut *mut u8, pcchname: *mut u32, szname: &mut [u16], passemblyid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetModuleInfo)(::windows::core::Interface::as_raw(self), moduleid, ppbaseloadaddress, szname.len() as _, pcchname, ::core::mem::transmute(szname.as_ptr()), passemblyid).ok()
    }
    pub unsafe fn GetModuleMetaData(&self, moduleid: usize, dwopenflags: u32, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetModuleMetaData)(::windows::core::Interface::as_raw(self), moduleid, dwopenflags, riid, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetILFunctionBody(&self, moduleid: usize, methodid: u32, ppmethodheader: *mut *mut u8, pcbmethodsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetILFunctionBody)(::windows::core::Interface::as_raw(self), moduleid, methodid, ppmethodheader, pcbmethodsize).ok()
    }
    pub unsafe fn GetILFunctionBodyAllocator(&self, moduleid: usize) -> ::windows::core::Result<IMethodMalloc> {
        let mut result__ = ::windows::core::zeroed::<IMethodMalloc>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetILFunctionBodyAllocator)(::windows::core::Interface::as_raw(self), moduleid, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetILFunctionBody(&self, moduleid: usize, methodid: u32, pbnewilmethodheader: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetILFunctionBody)(::windows::core::Interface::as_raw(self), moduleid, methodid, pbnewilmethodheader).ok()
    }
    pub unsafe fn GetAppDomainInfo(&self, appdomainid: usize, pcchname: *mut u32, szname: &mut [u16], pprocessid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetAppDomainInfo)(::windows::core::Interface::as_raw(self), appdomainid, szname.len() as _, pcchname, ::core::mem::transmute(szname.as_ptr()), pprocessid).ok()
    }
    pub unsafe fn GetAssemblyInfo(&self, assemblyid: usize, pcchname: *mut u32, szname: &mut [u16], pappdomainid: *mut usize, pmoduleid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetAssemblyInfo)(::windows::core::Interface::as_raw(self), assemblyid, szname.len() as _, pcchname, ::core::mem::transmute(szname.as_ptr()), pappdomainid, pmoduleid).ok()
    }
    pub unsafe fn SetFunctionReJIT(&self, functionid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetFunctionReJIT)(::windows::core::Interface::as_raw(self), functionid).ok()
    }
    pub unsafe fn ForceGC(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.ForceGC)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetILInstrumentedCodeMap<P0>(&self, functionid: usize, fstartjit: P0, cilmapentries: u32, rgilmapentries: *mut COR_IL_MAP) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetILInstrumentedCodeMap)(::windows::core::Interface::as_raw(self), functionid, fstartjit.into_param().abi(), cilmapentries, rgilmapentries).ok()
    }
    pub unsafe fn GetInprocInspectionInterface(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetInprocInspectionInterface)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetInprocInspectionIThisThread(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetInprocInspectionIThisThread)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetThreadContext(&self, threadid: usize, pcontextid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetThreadContext)(::windows::core::Interface::as_raw(self), threadid, pcontextid).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BeginInprocDebugging<P0>(&self, fthisthreadonly: P0, pdwprofilercontext: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.BeginInprocDebugging)(::windows::core::Interface::as_raw(self), fthisthreadonly.into_param().abi(), pdwprofilercontext).ok()
    }
    pub unsafe fn EndInprocDebugging(&self, dwprofilercontext: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.EndInprocDebugging)(::windows::core::Interface::as_raw(self), dwprofilercontext).ok()
    }
    pub unsafe fn GetILToNativeMapping(&self, functionid: usize, cmap: u32, pcmap: *mut u32, map: *mut COR_DEBUG_IL_TO_NATIVE_MAP) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetILToNativeMapping)(::windows::core::Interface::as_raw(self), functionid, cmap, pcmap, map).ok()
    }
    pub unsafe fn DoStackSnapshot(&self, thread: usize, callback: *mut StackSnapshotCallback, infoflags: u32, clientdata: *mut ::core::ffi::c_void, context: *mut u8, contextsize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.DoStackSnapshot)(::windows::core::Interface::as_raw(self), thread, callback, infoflags, clientdata, context, contextsize).ok()
    }
    pub unsafe fn SetEnterLeaveFunctionHooks2(&self, pfuncenter: *mut FunctionEnter2, pfuncleave: *mut FunctionLeave2, pfunctailcall: *mut FunctionTailcall2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetEnterLeaveFunctionHooks2)(::windows::core::Interface::as_raw(self), pfuncenter, pfuncleave, pfunctailcall).ok()
    }
    pub unsafe fn GetFunctionInfo2(&self, funcid: usize, frameinfo: usize, pclassid: *mut usize, pmoduleid: *mut usize, ptoken: *mut u32, ctypeargs: u32, pctypeargs: *mut u32, typeargs: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetFunctionInfo2)(::windows::core::Interface::as_raw(self), funcid, frameinfo, pclassid, pmoduleid, ptoken, ctypeargs, pctypeargs, typeargs).ok()
    }
    pub unsafe fn GetStringLayout(&self, pbufferlengthoffset: *mut u32, pstringlengthoffset: *mut u32, pbufferoffset: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetStringLayout)(::windows::core::Interface::as_raw(self), pbufferlengthoffset, pstringlengthoffset, pbufferoffset).ok()
    }
    #[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
    #[cfg(feature = "Win32_System_WinRT_Metadata")]
    pub unsafe fn GetClassLayout(&self, classid: usize, rfieldoffset: *mut super::super::WinRT::Metadata::COR_FIELD_OFFSET, cfieldoffset: u32, pcfieldoffset: *mut u32, pulclasssize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetClassLayout)(::windows::core::Interface::as_raw(self), classid, rfieldoffset, cfieldoffset, pcfieldoffset, pulclasssize).ok()
    }
    pub unsafe fn GetClassIDInfo2(&self, classid: usize, pmoduleid: *mut usize, ptypedeftoken: *mut u32, pparentclassid: *mut usize, cnumtypeargs: u32, pcnumtypeargs: *mut u32, typeargs: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetClassIDInfo2)(::windows::core::Interface::as_raw(self), classid, pmoduleid, ptypedeftoken, pparentclassid, cnumtypeargs, pcnumtypeargs, typeargs).ok()
    }
    pub unsafe fn GetCodeInfo2(&self, functionid: usize, ccodeinfos: u32, pccodeinfos: *mut u32, codeinfos: *mut COR_PRF_CODE_INFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetCodeInfo2)(::windows::core::Interface::as_raw(self), functionid, ccodeinfos, pccodeinfos, codeinfos).ok()
    }
    pub unsafe fn GetClassFromTokenAndTypeArgs(&self, moduleid: usize, typedef: u32, ctypeargs: u32, typeargs: *mut usize, pclassid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetClassFromTokenAndTypeArgs)(::windows::core::Interface::as_raw(self), moduleid, typedef, ctypeargs, typeargs, pclassid).ok()
    }
    pub unsafe fn GetFunctionFromTokenAndTypeArgs(&self, moduleid: usize, funcdef: u32, classid: usize, ctypeargs: u32, typeargs: *mut usize, pfunctionid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetFunctionFromTokenAndTypeArgs)(::windows::core::Interface::as_raw(self), moduleid, funcdef, classid, ctypeargs, typeargs, pfunctionid).ok()
    }
    pub unsafe fn EnumModuleFrozenObjects(&self, moduleid: usize) -> ::windows::core::Result<ICorProfilerObjectEnum> {
        let mut result__ = ::windows::core::zeroed::<ICorProfilerObjectEnum>();
        (::windows::core::Interface::vtable(self).base__.base__.EnumModuleFrozenObjects)(::windows::core::Interface::as_raw(self), moduleid, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetArrayObjectInfo(&self, objectid: usize, cdimensions: u32, pdimensionsizes: *mut u32, pdimensionlowerbounds: *mut i32, ppdata: *mut *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetArrayObjectInfo)(::windows::core::Interface::as_raw(self), objectid, cdimensions, pdimensionsizes, pdimensionlowerbounds, ppdata).ok()
    }
    pub unsafe fn GetBoxClassLayout(&self, classid: usize, pbufferoffset: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetBoxClassLayout)(::windows::core::Interface::as_raw(self), classid, pbufferoffset).ok()
    }
    pub unsafe fn GetThreadAppDomain(&self, threadid: usize, pappdomainid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetThreadAppDomain)(::windows::core::Interface::as_raw(self), threadid, pappdomainid).ok()
    }
    pub unsafe fn GetRVAStaticAddress(&self, classid: usize, fieldtoken: u32, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetRVAStaticAddress)(::windows::core::Interface::as_raw(self), classid, fieldtoken, ppaddress).ok()
    }
    pub unsafe fn GetAppDomainStaticAddress(&self, classid: usize, fieldtoken: u32, appdomainid: usize, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetAppDomainStaticAddress)(::windows::core::Interface::as_raw(self), classid, fieldtoken, appdomainid, ppaddress).ok()
    }
    pub unsafe fn GetThreadStaticAddress(&self, classid: usize, fieldtoken: u32, threadid: usize, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetThreadStaticAddress)(::windows::core::Interface::as_raw(self), classid, fieldtoken, threadid, ppaddress).ok()
    }
    pub unsafe fn GetContextStaticAddress(&self, classid: usize, fieldtoken: u32, contextid: usize, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetContextStaticAddress)(::windows::core::Interface::as_raw(self), classid, fieldtoken, contextid, ppaddress).ok()
    }
    pub unsafe fn GetStaticFieldInfo(&self, classid: usize, fieldtoken: u32, pfieldinfo: *mut COR_PRF_STATIC_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetStaticFieldInfo)(::windows::core::Interface::as_raw(self), classid, fieldtoken, pfieldinfo).ok()
    }
    pub unsafe fn GetGenerationBounds(&self, cobjectranges: u32, pcobjectranges: *mut u32, ranges: *mut COR_PRF_GC_GENERATION_RANGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetGenerationBounds)(::windows::core::Interface::as_raw(self), cobjectranges, pcobjectranges, ranges).ok()
    }
    pub unsafe fn GetObjectGeneration(&self, objectid: usize, range: *mut COR_PRF_GC_GENERATION_RANGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetObjectGeneration)(::windows::core::Interface::as_raw(self), objectid, range).ok()
    }
    pub unsafe fn GetNotifiedExceptionClauseInfo(&self, pinfo: *mut COR_PRF_EX_CLAUSE_INFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetNotifiedExceptionClauseInfo)(::windows::core::Interface::as_raw(self), pinfo).ok()
    }
    pub unsafe fn EnumJITedFunctions(&self) -> ::windows::core::Result<ICorProfilerFunctionEnum> {
        let mut result__ = ::windows::core::zeroed::<ICorProfilerFunctionEnum>();
        (::windows::core::Interface::vtable(self).base__.EnumJITedFunctions)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn RequestProfilerDetach(&self, dwexpectedcompletionmilliseconds: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.RequestProfilerDetach)(::windows::core::Interface::as_raw(self), dwexpectedcompletionmilliseconds).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFunctionIDMapper2(&self, pfunc: *mut FunctionIDMapper2, clientdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetFunctionIDMapper2)(::windows::core::Interface::as_raw(self), pfunc, clientdata).ok()
    }
    pub unsafe fn GetStringLayout2(&self, pstringlengthoffset: *mut u32, pbufferoffset: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetStringLayout2)(::windows::core::Interface::as_raw(self), pstringlengthoffset, pbufferoffset).ok()
    }
    pub unsafe fn SetEnterLeaveFunctionHooks3(&self, pfuncenter3: *mut FunctionEnter3, pfuncleave3: *mut FunctionLeave3, pfunctailcall3: *mut FunctionTailcall3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetEnterLeaveFunctionHooks3)(::windows::core::Interface::as_raw(self), pfuncenter3, pfuncleave3, pfunctailcall3).ok()
    }
    pub unsafe fn SetEnterLeaveFunctionHooks3WithInfo(&self, pfuncenter3withinfo: *mut FunctionEnter3WithInfo, pfuncleave3withinfo: *mut FunctionLeave3WithInfo, pfunctailcall3withinfo: *mut FunctionTailcall3WithInfo) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetEnterLeaveFunctionHooks3WithInfo)(::windows::core::Interface::as_raw(self), pfuncenter3withinfo, pfuncleave3withinfo, pfunctailcall3withinfo).ok()
    }
    pub unsafe fn GetFunctionEnter3Info(&self, functionid: usize, eltinfo: usize, pframeinfo: *mut usize, pcbargumentinfo: *mut u32, pargumentinfo: *mut COR_PRF_FUNCTION_ARGUMENT_INFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetFunctionEnter3Info)(::windows::core::Interface::as_raw(self), functionid, eltinfo, pframeinfo, pcbargumentinfo, pargumentinfo).ok()
    }
    pub unsafe fn GetFunctionLeave3Info(&self, functionid: usize, eltinfo: usize, pframeinfo: *mut usize, pretvalrange: *mut COR_PRF_FUNCTION_ARGUMENT_RANGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetFunctionLeave3Info)(::windows::core::Interface::as_raw(self), functionid, eltinfo, pframeinfo, pretvalrange).ok()
    }
    pub unsafe fn GetFunctionTailcall3Info(&self, functionid: usize, eltinfo: usize, pframeinfo: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetFunctionTailcall3Info)(::windows::core::Interface::as_raw(self), functionid, eltinfo, pframeinfo).ok()
    }
    pub unsafe fn EnumModules(&self) -> ::windows::core::Result<ICorProfilerModuleEnum> {
        let mut result__ = ::windows::core::zeroed::<ICorProfilerModuleEnum>();
        (::windows::core::Interface::vtable(self).base__.EnumModules)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetRuntimeInformation(&self, pclrinstanceid: *mut u16, pruntimetype: *mut COR_PRF_RUNTIME_TYPE, pmajorversion: *mut u16, pminorversion: *mut u16, pbuildnumber: *mut u16, pqfeversion: *mut u16, pcchversionstring: *mut u32, szversionstring: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetRuntimeInformation)(::windows::core::Interface::as_raw(self), pclrinstanceid, pruntimetype, pmajorversion, pminorversion, pbuildnumber, pqfeversion, szversionstring.len() as _, pcchversionstring, ::core::mem::transmute(szversionstring.as_ptr())).ok()
    }
    pub unsafe fn GetThreadStaticAddress2(&self, classid: usize, fieldtoken: u32, appdomainid: usize, threadid: usize, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetThreadStaticAddress2)(::windows::core::Interface::as_raw(self), classid, fieldtoken, appdomainid, threadid, ppaddress).ok()
    }
    pub unsafe fn GetAppDomainsContainingModule(&self, moduleid: usize, cappdomainids: u32, pcappdomainids: *mut u32, appdomainids: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetAppDomainsContainingModule)(::windows::core::Interface::as_raw(self), moduleid, cappdomainids, pcappdomainids, appdomainids).ok()
    }
    pub unsafe fn GetModuleInfo2(&self, moduleid: usize, ppbaseloadaddress: *mut *mut u8, pcchname: *mut u32, szname: &mut [u16], passemblyid: *mut usize, pdwmoduleflags: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetModuleInfo2)(::windows::core::Interface::as_raw(self), moduleid, ppbaseloadaddress, szname.len() as _, pcchname, ::core::mem::transmute(szname.as_ptr()), passemblyid, pdwmoduleflags).ok()
    }
    pub unsafe fn EnumThreads(&self) -> ::windows::core::Result<ICorProfilerThreadEnum> {
        let mut result__ = ::windows::core::zeroed::<ICorProfilerThreadEnum>();
        (::windows::core::Interface::vtable(self).EnumThreads)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn InitializeCurrentThread(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).InitializeCurrentThread)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RequestReJIT(&self, cfunctions: u32, moduleids: *mut usize, methodids: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RequestReJIT)(::windows::core::Interface::as_raw(self), cfunctions, moduleids, methodids).ok()
    }
    pub unsafe fn RequestRevert(&self, cfunctions: u32, moduleids: *mut usize, methodids: *mut u32, status: *mut ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RequestRevert)(::windows::core::Interface::as_raw(self), cfunctions, moduleids, methodids, status).ok()
    }
    pub unsafe fn GetCodeInfo3(&self, functionid: usize, rejitid: usize, ccodeinfos: u32, pccodeinfos: *mut u32, codeinfos: *mut COR_PRF_CODE_INFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCodeInfo3)(::windows::core::Interface::as_raw(self), functionid, rejitid, ccodeinfos, pccodeinfos, codeinfos).ok()
    }
    pub unsafe fn GetFunctionFromIP2(&self, ip: *mut u8, pfunctionid: *mut usize, prejitid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFunctionFromIP2)(::windows::core::Interface::as_raw(self), ip, pfunctionid, prejitid).ok()
    }
    pub unsafe fn GetReJITIDs(&self, functionid: usize, crejitids: u32, pcrejitids: *mut u32, rejitids: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetReJITIDs)(::windows::core::Interface::as_raw(self), functionid, crejitids, pcrejitids, rejitids).ok()
    }
    pub unsafe fn GetILToNativeMapping2(&self, functionid: usize, rejitid: usize, cmap: u32, pcmap: *mut u32, map: *mut COR_DEBUG_IL_TO_NATIVE_MAP) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetILToNativeMapping2)(::windows::core::Interface::as_raw(self), functionid, rejitid, cmap, pcmap, map).ok()
    }
    pub unsafe fn EnumJITedFunctions2(&self) -> ::windows::core::Result<ICorProfilerFunctionEnum> {
        let mut result__ = ::windows::core::zeroed::<ICorProfilerFunctionEnum>();
        (::windows::core::Interface::vtable(self).EnumJITedFunctions2)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetObjectSize2(&self, objectid: usize, pcsize: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetObjectSize2)(::windows::core::Interface::as_raw(self), objectid, pcsize).ok()
    }
}
::windows::imp::interface_hierarchy!(ICorProfilerInfo4, ::windows::core::IUnknown, ICorProfilerInfo, ICorProfilerInfo2, ICorProfilerInfo3);
impl ::core::cmp::PartialEq for ICorProfilerInfo4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICorProfilerInfo4 {}
impl ::core::fmt::Debug for ICorProfilerInfo4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICorProfilerInfo4").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICorProfilerInfo4 {
    type Vtable = ICorProfilerInfo4_Vtbl;
}
impl ::core::clone::Clone for ICorProfilerInfo4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICorProfilerInfo4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0d8fdcaa_6257_47bf_b1bf_94dac88466ee);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorProfilerInfo4_Vtbl {
    pub base__: ICorProfilerInfo3_Vtbl,
    pub EnumThreads: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub InitializeCurrentThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RequestReJIT: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cfunctions: u32, moduleids: *mut usize, methodids: *mut u32) -> ::windows::core::HRESULT,
    pub RequestRevert: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cfunctions: u32, moduleids: *mut usize, methodids: *mut u32, status: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub GetCodeInfo3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, functionid: usize, rejitid: usize, ccodeinfos: u32, pccodeinfos: *mut u32, codeinfos: *mut COR_PRF_CODE_INFO) -> ::windows::core::HRESULT,
    pub GetFunctionFromIP2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ip: *mut u8, pfunctionid: *mut usize, prejitid: *mut usize) -> ::windows::core::HRESULT,
    pub GetReJITIDs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, functionid: usize, crejitids: u32, pcrejitids: *mut u32, rejitids: *mut usize) -> ::windows::core::HRESULT,
    pub GetILToNativeMapping2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, functionid: usize, rejitid: usize, cmap: u32, pcmap: *mut u32, map: *mut COR_DEBUG_IL_TO_NATIVE_MAP) -> ::windows::core::HRESULT,
    pub EnumJITedFunctions2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetObjectSize2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objectid: usize, pcsize: *mut usize) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
#[repr(transparent)]
pub struct ICorProfilerInfo5(::windows::core::IUnknown);
impl ICorProfilerInfo5 {
    pub unsafe fn GetClassFromObject(&self, objectid: usize, pclassid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetClassFromObject)(::windows::core::Interface::as_raw(self), objectid, pclassid).ok()
    }
    pub unsafe fn GetClassFromToken(&self, moduleid: usize, typedef: u32, pclassid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetClassFromToken)(::windows::core::Interface::as_raw(self), moduleid, typedef, pclassid).ok()
    }
    pub unsafe fn GetCodeInfo(&self, functionid: usize, pstart: *mut *mut u8, pcsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetCodeInfo)(::windows::core::Interface::as_raw(self), functionid, pstart, pcsize).ok()
    }
    pub unsafe fn GetEventMask(&self, pdwevents: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetEventMask)(::windows::core::Interface::as_raw(self), pdwevents).ok()
    }
    pub unsafe fn GetFunctionFromIP(&self, ip: *mut u8, pfunctionid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetFunctionFromIP)(::windows::core::Interface::as_raw(self), ip, pfunctionid).ok()
    }
    pub unsafe fn GetFunctionFromToken(&self, moduleid: usize, token: u32, pfunctionid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetFunctionFromToken)(::windows::core::Interface::as_raw(self), moduleid, token, pfunctionid).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHandleFromThread(&self, threadid: usize, phthread: *mut super::super::super::Foundation::HANDLE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetHandleFromThread)(::windows::core::Interface::as_raw(self), threadid, phthread).ok()
    }
    pub unsafe fn GetObjectSize(&self, objectid: usize, pcsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetObjectSize)(::windows::core::Interface::as_raw(self), objectid, pcsize).ok()
    }
    #[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
    #[cfg(feature = "Win32_System_WinRT_Metadata")]
    pub unsafe fn IsArrayClass(&self, classid: usize, pbaseelemtype: *mut super::super::WinRT::Metadata::CorElementType, pbaseclassid: *mut usize, pcrank: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.IsArrayClass)(::windows::core::Interface::as_raw(self), classid, pbaseelemtype, pbaseclassid, pcrank).ok()
    }
    pub unsafe fn GetThreadInfo(&self, threadid: usize, pdwwin32threadid: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetThreadInfo)(::windows::core::Interface::as_raw(self), threadid, pdwwin32threadid).ok()
    }
    pub unsafe fn GetCurrentThreadID(&self, pthreadid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetCurrentThreadID)(::windows::core::Interface::as_raw(self), pthreadid).ok()
    }
    pub unsafe fn GetClassIDInfo(&self, classid: usize, pmoduleid: *mut usize, ptypedeftoken: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetClassIDInfo)(::windows::core::Interface::as_raw(self), classid, pmoduleid, ptypedeftoken).ok()
    }
    pub unsafe fn GetFunctionInfo(&self, functionid: usize, pclassid: *mut usize, pmoduleid: *mut usize, ptoken: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetFunctionInfo)(::windows::core::Interface::as_raw(self), functionid, pclassid, pmoduleid, ptoken).ok()
    }
    pub unsafe fn SetEventMask(&self, dwevents: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.SetEventMask)(::windows::core::Interface::as_raw(self), dwevents).ok()
    }
    pub unsafe fn SetEnterLeaveFunctionHooks(&self, pfuncenter: *mut FunctionEnter, pfuncleave: *mut FunctionLeave, pfunctailcall: *mut FunctionTailcall) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.SetEnterLeaveFunctionHooks)(::windows::core::Interface::as_raw(self), pfuncenter, pfuncleave, pfunctailcall).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFunctionIDMapper(&self, pfunc: *mut FunctionIDMapper) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.SetFunctionIDMapper)(::windows::core::Interface::as_raw(self), pfunc).ok()
    }
    pub unsafe fn GetTokenAndMetaDataFromFunction(&self, functionid: usize, riid: *const ::windows::core::GUID, ppimport: *mut ::core::option::Option<::windows::core::IUnknown>, ptoken: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetTokenAndMetaDataFromFunction)(::windows::core::Interface::as_raw(self), functionid, riid, ::core::mem::transmute(ppimport), ptoken).ok()
    }
    pub unsafe fn GetModuleInfo(&self, moduleid: usize, ppbaseloadaddress: *mut *mut u8, pcchname: *mut u32, szname: &mut [u16], passemblyid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetModuleInfo)(::windows::core::Interface::as_raw(self), moduleid, ppbaseloadaddress, szname.len() as _, pcchname, ::core::mem::transmute(szname.as_ptr()), passemblyid).ok()
    }
    pub unsafe fn GetModuleMetaData(&self, moduleid: usize, dwopenflags: u32, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetModuleMetaData)(::windows::core::Interface::as_raw(self), moduleid, dwopenflags, riid, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetILFunctionBody(&self, moduleid: usize, methodid: u32, ppmethodheader: *mut *mut u8, pcbmethodsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetILFunctionBody)(::windows::core::Interface::as_raw(self), moduleid, methodid, ppmethodheader, pcbmethodsize).ok()
    }
    pub unsafe fn GetILFunctionBodyAllocator(&self, moduleid: usize) -> ::windows::core::Result<IMethodMalloc> {
        let mut result__ = ::windows::core::zeroed::<IMethodMalloc>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetILFunctionBodyAllocator)(::windows::core::Interface::as_raw(self), moduleid, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetILFunctionBody(&self, moduleid: usize, methodid: u32, pbnewilmethodheader: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.SetILFunctionBody)(::windows::core::Interface::as_raw(self), moduleid, methodid, pbnewilmethodheader).ok()
    }
    pub unsafe fn GetAppDomainInfo(&self, appdomainid: usize, pcchname: *mut u32, szname: &mut [u16], pprocessid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetAppDomainInfo)(::windows::core::Interface::as_raw(self), appdomainid, szname.len() as _, pcchname, ::core::mem::transmute(szname.as_ptr()), pprocessid).ok()
    }
    pub unsafe fn GetAssemblyInfo(&self, assemblyid: usize, pcchname: *mut u32, szname: &mut [u16], pappdomainid: *mut usize, pmoduleid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetAssemblyInfo)(::windows::core::Interface::as_raw(self), assemblyid, szname.len() as _, pcchname, ::core::mem::transmute(szname.as_ptr()), pappdomainid, pmoduleid).ok()
    }
    pub unsafe fn SetFunctionReJIT(&self, functionid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.SetFunctionReJIT)(::windows::core::Interface::as_raw(self), functionid).ok()
    }
    pub unsafe fn ForceGC(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.ForceGC)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetILInstrumentedCodeMap<P0>(&self, functionid: usize, fstartjit: P0, cilmapentries: u32, rgilmapentries: *mut COR_IL_MAP) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.SetILInstrumentedCodeMap)(::windows::core::Interface::as_raw(self), functionid, fstartjit.into_param().abi(), cilmapentries, rgilmapentries).ok()
    }
    pub unsafe fn GetInprocInspectionInterface(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetInprocInspectionInterface)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetInprocInspectionIThisThread(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetInprocInspectionIThisThread)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetThreadContext(&self, threadid: usize, pcontextid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetThreadContext)(::windows::core::Interface::as_raw(self), threadid, pcontextid).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BeginInprocDebugging<P0>(&self, fthisthreadonly: P0, pdwprofilercontext: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.BeginInprocDebugging)(::windows::core::Interface::as_raw(self), fthisthreadonly.into_param().abi(), pdwprofilercontext).ok()
    }
    pub unsafe fn EndInprocDebugging(&self, dwprofilercontext: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.EndInprocDebugging)(::windows::core::Interface::as_raw(self), dwprofilercontext).ok()
    }
    pub unsafe fn GetILToNativeMapping(&self, functionid: usize, cmap: u32, pcmap: *mut u32, map: *mut COR_DEBUG_IL_TO_NATIVE_MAP) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetILToNativeMapping)(::windows::core::Interface::as_raw(self), functionid, cmap, pcmap, map).ok()
    }
    pub unsafe fn DoStackSnapshot(&self, thread: usize, callback: *mut StackSnapshotCallback, infoflags: u32, clientdata: *mut ::core::ffi::c_void, context: *mut u8, contextsize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.DoStackSnapshot)(::windows::core::Interface::as_raw(self), thread, callback, infoflags, clientdata, context, contextsize).ok()
    }
    pub unsafe fn SetEnterLeaveFunctionHooks2(&self, pfuncenter: *mut FunctionEnter2, pfuncleave: *mut FunctionLeave2, pfunctailcall: *mut FunctionTailcall2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetEnterLeaveFunctionHooks2)(::windows::core::Interface::as_raw(self), pfuncenter, pfuncleave, pfunctailcall).ok()
    }
    pub unsafe fn GetFunctionInfo2(&self, funcid: usize, frameinfo: usize, pclassid: *mut usize, pmoduleid: *mut usize, ptoken: *mut u32, ctypeargs: u32, pctypeargs: *mut u32, typeargs: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetFunctionInfo2)(::windows::core::Interface::as_raw(self), funcid, frameinfo, pclassid, pmoduleid, ptoken, ctypeargs, pctypeargs, typeargs).ok()
    }
    pub unsafe fn GetStringLayout(&self, pbufferlengthoffset: *mut u32, pstringlengthoffset: *mut u32, pbufferoffset: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetStringLayout)(::windows::core::Interface::as_raw(self), pbufferlengthoffset, pstringlengthoffset, pbufferoffset).ok()
    }
    #[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
    #[cfg(feature = "Win32_System_WinRT_Metadata")]
    pub unsafe fn GetClassLayout(&self, classid: usize, rfieldoffset: *mut super::super::WinRT::Metadata::COR_FIELD_OFFSET, cfieldoffset: u32, pcfieldoffset: *mut u32, pulclasssize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetClassLayout)(::windows::core::Interface::as_raw(self), classid, rfieldoffset, cfieldoffset, pcfieldoffset, pulclasssize).ok()
    }
    pub unsafe fn GetClassIDInfo2(&self, classid: usize, pmoduleid: *mut usize, ptypedeftoken: *mut u32, pparentclassid: *mut usize, cnumtypeargs: u32, pcnumtypeargs: *mut u32, typeargs: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetClassIDInfo2)(::windows::core::Interface::as_raw(self), classid, pmoduleid, ptypedeftoken, pparentclassid, cnumtypeargs, pcnumtypeargs, typeargs).ok()
    }
    pub unsafe fn GetCodeInfo2(&self, functionid: usize, ccodeinfos: u32, pccodeinfos: *mut u32, codeinfos: *mut COR_PRF_CODE_INFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetCodeInfo2)(::windows::core::Interface::as_raw(self), functionid, ccodeinfos, pccodeinfos, codeinfos).ok()
    }
    pub unsafe fn GetClassFromTokenAndTypeArgs(&self, moduleid: usize, typedef: u32, ctypeargs: u32, typeargs: *mut usize, pclassid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetClassFromTokenAndTypeArgs)(::windows::core::Interface::as_raw(self), moduleid, typedef, ctypeargs, typeargs, pclassid).ok()
    }
    pub unsafe fn GetFunctionFromTokenAndTypeArgs(&self, moduleid: usize, funcdef: u32, classid: usize, ctypeargs: u32, typeargs: *mut usize, pfunctionid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetFunctionFromTokenAndTypeArgs)(::windows::core::Interface::as_raw(self), moduleid, funcdef, classid, ctypeargs, typeargs, pfunctionid).ok()
    }
    pub unsafe fn EnumModuleFrozenObjects(&self, moduleid: usize) -> ::windows::core::Result<ICorProfilerObjectEnum> {
        let mut result__ = ::windows::core::zeroed::<ICorProfilerObjectEnum>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.EnumModuleFrozenObjects)(::windows::core::Interface::as_raw(self), moduleid, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetArrayObjectInfo(&self, objectid: usize, cdimensions: u32, pdimensionsizes: *mut u32, pdimensionlowerbounds: *mut i32, ppdata: *mut *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetArrayObjectInfo)(::windows::core::Interface::as_raw(self), objectid, cdimensions, pdimensionsizes, pdimensionlowerbounds, ppdata).ok()
    }
    pub unsafe fn GetBoxClassLayout(&self, classid: usize, pbufferoffset: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetBoxClassLayout)(::windows::core::Interface::as_raw(self), classid, pbufferoffset).ok()
    }
    pub unsafe fn GetThreadAppDomain(&self, threadid: usize, pappdomainid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetThreadAppDomain)(::windows::core::Interface::as_raw(self), threadid, pappdomainid).ok()
    }
    pub unsafe fn GetRVAStaticAddress(&self, classid: usize, fieldtoken: u32, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetRVAStaticAddress)(::windows::core::Interface::as_raw(self), classid, fieldtoken, ppaddress).ok()
    }
    pub unsafe fn GetAppDomainStaticAddress(&self, classid: usize, fieldtoken: u32, appdomainid: usize, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetAppDomainStaticAddress)(::windows::core::Interface::as_raw(self), classid, fieldtoken, appdomainid, ppaddress).ok()
    }
    pub unsafe fn GetThreadStaticAddress(&self, classid: usize, fieldtoken: u32, threadid: usize, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetThreadStaticAddress)(::windows::core::Interface::as_raw(self), classid, fieldtoken, threadid, ppaddress).ok()
    }
    pub unsafe fn GetContextStaticAddress(&self, classid: usize, fieldtoken: u32, contextid: usize, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetContextStaticAddress)(::windows::core::Interface::as_raw(self), classid, fieldtoken, contextid, ppaddress).ok()
    }
    pub unsafe fn GetStaticFieldInfo(&self, classid: usize, fieldtoken: u32, pfieldinfo: *mut COR_PRF_STATIC_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetStaticFieldInfo)(::windows::core::Interface::as_raw(self), classid, fieldtoken, pfieldinfo).ok()
    }
    pub unsafe fn GetGenerationBounds(&self, cobjectranges: u32, pcobjectranges: *mut u32, ranges: *mut COR_PRF_GC_GENERATION_RANGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetGenerationBounds)(::windows::core::Interface::as_raw(self), cobjectranges, pcobjectranges, ranges).ok()
    }
    pub unsafe fn GetObjectGeneration(&self, objectid: usize, range: *mut COR_PRF_GC_GENERATION_RANGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetObjectGeneration)(::windows::core::Interface::as_raw(self), objectid, range).ok()
    }
    pub unsafe fn GetNotifiedExceptionClauseInfo(&self, pinfo: *mut COR_PRF_EX_CLAUSE_INFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetNotifiedExceptionClauseInfo)(::windows::core::Interface::as_raw(self), pinfo).ok()
    }
    pub unsafe fn EnumJITedFunctions(&self) -> ::windows::core::Result<ICorProfilerFunctionEnum> {
        let mut result__ = ::windows::core::zeroed::<ICorProfilerFunctionEnum>();
        (::windows::core::Interface::vtable(self).base__.base__.EnumJITedFunctions)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn RequestProfilerDetach(&self, dwexpectedcompletionmilliseconds: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.RequestProfilerDetach)(::windows::core::Interface::as_raw(self), dwexpectedcompletionmilliseconds).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFunctionIDMapper2(&self, pfunc: *mut FunctionIDMapper2, clientdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetFunctionIDMapper2)(::windows::core::Interface::as_raw(self), pfunc, clientdata).ok()
    }
    pub unsafe fn GetStringLayout2(&self, pstringlengthoffset: *mut u32, pbufferoffset: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetStringLayout2)(::windows::core::Interface::as_raw(self), pstringlengthoffset, pbufferoffset).ok()
    }
    pub unsafe fn SetEnterLeaveFunctionHooks3(&self, pfuncenter3: *mut FunctionEnter3, pfuncleave3: *mut FunctionLeave3, pfunctailcall3: *mut FunctionTailcall3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetEnterLeaveFunctionHooks3)(::windows::core::Interface::as_raw(self), pfuncenter3, pfuncleave3, pfunctailcall3).ok()
    }
    pub unsafe fn SetEnterLeaveFunctionHooks3WithInfo(&self, pfuncenter3withinfo: *mut FunctionEnter3WithInfo, pfuncleave3withinfo: *mut FunctionLeave3WithInfo, pfunctailcall3withinfo: *mut FunctionTailcall3WithInfo) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetEnterLeaveFunctionHooks3WithInfo)(::windows::core::Interface::as_raw(self), pfuncenter3withinfo, pfuncleave3withinfo, pfunctailcall3withinfo).ok()
    }
    pub unsafe fn GetFunctionEnter3Info(&self, functionid: usize, eltinfo: usize, pframeinfo: *mut usize, pcbargumentinfo: *mut u32, pargumentinfo: *mut COR_PRF_FUNCTION_ARGUMENT_INFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetFunctionEnter3Info)(::windows::core::Interface::as_raw(self), functionid, eltinfo, pframeinfo, pcbargumentinfo, pargumentinfo).ok()
    }
    pub unsafe fn GetFunctionLeave3Info(&self, functionid: usize, eltinfo: usize, pframeinfo: *mut usize, pretvalrange: *mut COR_PRF_FUNCTION_ARGUMENT_RANGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetFunctionLeave3Info)(::windows::core::Interface::as_raw(self), functionid, eltinfo, pframeinfo, pretvalrange).ok()
    }
    pub unsafe fn GetFunctionTailcall3Info(&self, functionid: usize, eltinfo: usize, pframeinfo: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetFunctionTailcall3Info)(::windows::core::Interface::as_raw(self), functionid, eltinfo, pframeinfo).ok()
    }
    pub unsafe fn EnumModules(&self) -> ::windows::core::Result<ICorProfilerModuleEnum> {
        let mut result__ = ::windows::core::zeroed::<ICorProfilerModuleEnum>();
        (::windows::core::Interface::vtable(self).base__.base__.EnumModules)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetRuntimeInformation(&self, pclrinstanceid: *mut u16, pruntimetype: *mut COR_PRF_RUNTIME_TYPE, pmajorversion: *mut u16, pminorversion: *mut u16, pbuildnumber: *mut u16, pqfeversion: *mut u16, pcchversionstring: *mut u32, szversionstring: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetRuntimeInformation)(::windows::core::Interface::as_raw(self), pclrinstanceid, pruntimetype, pmajorversion, pminorversion, pbuildnumber, pqfeversion, szversionstring.len() as _, pcchversionstring, ::core::mem::transmute(szversionstring.as_ptr())).ok()
    }
    pub unsafe fn GetThreadStaticAddress2(&self, classid: usize, fieldtoken: u32, appdomainid: usize, threadid: usize, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetThreadStaticAddress2)(::windows::core::Interface::as_raw(self), classid, fieldtoken, appdomainid, threadid, ppaddress).ok()
    }
    pub unsafe fn GetAppDomainsContainingModule(&self, moduleid: usize, cappdomainids: u32, pcappdomainids: *mut u32, appdomainids: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetAppDomainsContainingModule)(::windows::core::Interface::as_raw(self), moduleid, cappdomainids, pcappdomainids, appdomainids).ok()
    }
    pub unsafe fn GetModuleInfo2(&self, moduleid: usize, ppbaseloadaddress: *mut *mut u8, pcchname: *mut u32, szname: &mut [u16], passemblyid: *mut usize, pdwmoduleflags: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetModuleInfo2)(::windows::core::Interface::as_raw(self), moduleid, ppbaseloadaddress, szname.len() as _, pcchname, ::core::mem::transmute(szname.as_ptr()), passemblyid, pdwmoduleflags).ok()
    }
    pub unsafe fn EnumThreads(&self) -> ::windows::core::Result<ICorProfilerThreadEnum> {
        let mut result__ = ::windows::core::zeroed::<ICorProfilerThreadEnum>();
        (::windows::core::Interface::vtable(self).base__.EnumThreads)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn InitializeCurrentThread(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.InitializeCurrentThread)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RequestReJIT(&self, cfunctions: u32, moduleids: *mut usize, methodids: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.RequestReJIT)(::windows::core::Interface::as_raw(self), cfunctions, moduleids, methodids).ok()
    }
    pub unsafe fn RequestRevert(&self, cfunctions: u32, moduleids: *mut usize, methodids: *mut u32, status: *mut ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.RequestRevert)(::windows::core::Interface::as_raw(self), cfunctions, moduleids, methodids, status).ok()
    }
    pub unsafe fn GetCodeInfo3(&self, functionid: usize, rejitid: usize, ccodeinfos: u32, pccodeinfos: *mut u32, codeinfos: *mut COR_PRF_CODE_INFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetCodeInfo3)(::windows::core::Interface::as_raw(self), functionid, rejitid, ccodeinfos, pccodeinfos, codeinfos).ok()
    }
    pub unsafe fn GetFunctionFromIP2(&self, ip: *mut u8, pfunctionid: *mut usize, prejitid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetFunctionFromIP2)(::windows::core::Interface::as_raw(self), ip, pfunctionid, prejitid).ok()
    }
    pub unsafe fn GetReJITIDs(&self, functionid: usize, crejitids: u32, pcrejitids: *mut u32, rejitids: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetReJITIDs)(::windows::core::Interface::as_raw(self), functionid, crejitids, pcrejitids, rejitids).ok()
    }
    pub unsafe fn GetILToNativeMapping2(&self, functionid: usize, rejitid: usize, cmap: u32, pcmap: *mut u32, map: *mut COR_DEBUG_IL_TO_NATIVE_MAP) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetILToNativeMapping2)(::windows::core::Interface::as_raw(self), functionid, rejitid, cmap, pcmap, map).ok()
    }
    pub unsafe fn EnumJITedFunctions2(&self) -> ::windows::core::Result<ICorProfilerFunctionEnum> {
        let mut result__ = ::windows::core::zeroed::<ICorProfilerFunctionEnum>();
        (::windows::core::Interface::vtable(self).base__.EnumJITedFunctions2)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetObjectSize2(&self, objectid: usize, pcsize: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetObjectSize2)(::windows::core::Interface::as_raw(self), objectid, pcsize).ok()
    }
    pub unsafe fn GetEventMask2(&self, pdweventslow: *mut u32, pdweventshigh: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetEventMask2)(::windows::core::Interface::as_raw(self), pdweventslow, pdweventshigh).ok()
    }
    pub unsafe fn SetEventMask2(&self, dweventslow: u32, dweventshigh: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEventMask2)(::windows::core::Interface::as_raw(self), dweventslow, dweventshigh).ok()
    }
}
::windows::imp::interface_hierarchy!(ICorProfilerInfo5, ::windows::core::IUnknown, ICorProfilerInfo, ICorProfilerInfo2, ICorProfilerInfo3, ICorProfilerInfo4);
impl ::core::cmp::PartialEq for ICorProfilerInfo5 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICorProfilerInfo5 {}
impl ::core::fmt::Debug for ICorProfilerInfo5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICorProfilerInfo5").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICorProfilerInfo5 {
    type Vtable = ICorProfilerInfo5_Vtbl;
}
impl ::core::clone::Clone for ICorProfilerInfo5 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICorProfilerInfo5 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x07602928_ce38_4b83_81e7_74adaf781214);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorProfilerInfo5_Vtbl {
    pub base__: ICorProfilerInfo4_Vtbl,
    pub GetEventMask2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdweventslow: *mut u32, pdweventshigh: *mut u32) -> ::windows::core::HRESULT,
    pub SetEventMask2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dweventslow: u32, dweventshigh: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
#[repr(transparent)]
pub struct ICorProfilerInfo6(::windows::core::IUnknown);
impl ICorProfilerInfo6 {
    pub unsafe fn GetClassFromObject(&self, objectid: usize, pclassid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetClassFromObject)(::windows::core::Interface::as_raw(self), objectid, pclassid).ok()
    }
    pub unsafe fn GetClassFromToken(&self, moduleid: usize, typedef: u32, pclassid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetClassFromToken)(::windows::core::Interface::as_raw(self), moduleid, typedef, pclassid).ok()
    }
    pub unsafe fn GetCodeInfo(&self, functionid: usize, pstart: *mut *mut u8, pcsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetCodeInfo)(::windows::core::Interface::as_raw(self), functionid, pstart, pcsize).ok()
    }
    pub unsafe fn GetEventMask(&self, pdwevents: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetEventMask)(::windows::core::Interface::as_raw(self), pdwevents).ok()
    }
    pub unsafe fn GetFunctionFromIP(&self, ip: *mut u8, pfunctionid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetFunctionFromIP)(::windows::core::Interface::as_raw(self), ip, pfunctionid).ok()
    }
    pub unsafe fn GetFunctionFromToken(&self, moduleid: usize, token: u32, pfunctionid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetFunctionFromToken)(::windows::core::Interface::as_raw(self), moduleid, token, pfunctionid).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHandleFromThread(&self, threadid: usize, phthread: *mut super::super::super::Foundation::HANDLE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetHandleFromThread)(::windows::core::Interface::as_raw(self), threadid, phthread).ok()
    }
    pub unsafe fn GetObjectSize(&self, objectid: usize, pcsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetObjectSize)(::windows::core::Interface::as_raw(self), objectid, pcsize).ok()
    }
    #[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
    #[cfg(feature = "Win32_System_WinRT_Metadata")]
    pub unsafe fn IsArrayClass(&self, classid: usize, pbaseelemtype: *mut super::super::WinRT::Metadata::CorElementType, pbaseclassid: *mut usize, pcrank: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.IsArrayClass)(::windows::core::Interface::as_raw(self), classid, pbaseelemtype, pbaseclassid, pcrank).ok()
    }
    pub unsafe fn GetThreadInfo(&self, threadid: usize, pdwwin32threadid: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetThreadInfo)(::windows::core::Interface::as_raw(self), threadid, pdwwin32threadid).ok()
    }
    pub unsafe fn GetCurrentThreadID(&self, pthreadid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetCurrentThreadID)(::windows::core::Interface::as_raw(self), pthreadid).ok()
    }
    pub unsafe fn GetClassIDInfo(&self, classid: usize, pmoduleid: *mut usize, ptypedeftoken: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetClassIDInfo)(::windows::core::Interface::as_raw(self), classid, pmoduleid, ptypedeftoken).ok()
    }
    pub unsafe fn GetFunctionInfo(&self, functionid: usize, pclassid: *mut usize, pmoduleid: *mut usize, ptoken: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetFunctionInfo)(::windows::core::Interface::as_raw(self), functionid, pclassid, pmoduleid, ptoken).ok()
    }
    pub unsafe fn SetEventMask(&self, dwevents: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.SetEventMask)(::windows::core::Interface::as_raw(self), dwevents).ok()
    }
    pub unsafe fn SetEnterLeaveFunctionHooks(&self, pfuncenter: *mut FunctionEnter, pfuncleave: *mut FunctionLeave, pfunctailcall: *mut FunctionTailcall) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.SetEnterLeaveFunctionHooks)(::windows::core::Interface::as_raw(self), pfuncenter, pfuncleave, pfunctailcall).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFunctionIDMapper(&self, pfunc: *mut FunctionIDMapper) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.SetFunctionIDMapper)(::windows::core::Interface::as_raw(self), pfunc).ok()
    }
    pub unsafe fn GetTokenAndMetaDataFromFunction(&self, functionid: usize, riid: *const ::windows::core::GUID, ppimport: *mut ::core::option::Option<::windows::core::IUnknown>, ptoken: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetTokenAndMetaDataFromFunction)(::windows::core::Interface::as_raw(self), functionid, riid, ::core::mem::transmute(ppimport), ptoken).ok()
    }
    pub unsafe fn GetModuleInfo(&self, moduleid: usize, ppbaseloadaddress: *mut *mut u8, pcchname: *mut u32, szname: &mut [u16], passemblyid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetModuleInfo)(::windows::core::Interface::as_raw(self), moduleid, ppbaseloadaddress, szname.len() as _, pcchname, ::core::mem::transmute(szname.as_ptr()), passemblyid).ok()
    }
    pub unsafe fn GetModuleMetaData(&self, moduleid: usize, dwopenflags: u32, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetModuleMetaData)(::windows::core::Interface::as_raw(self), moduleid, dwopenflags, riid, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetILFunctionBody(&self, moduleid: usize, methodid: u32, ppmethodheader: *mut *mut u8, pcbmethodsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetILFunctionBody)(::windows::core::Interface::as_raw(self), moduleid, methodid, ppmethodheader, pcbmethodsize).ok()
    }
    pub unsafe fn GetILFunctionBodyAllocator(&self, moduleid: usize) -> ::windows::core::Result<IMethodMalloc> {
        let mut result__ = ::windows::core::zeroed::<IMethodMalloc>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetILFunctionBodyAllocator)(::windows::core::Interface::as_raw(self), moduleid, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetILFunctionBody(&self, moduleid: usize, methodid: u32, pbnewilmethodheader: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.SetILFunctionBody)(::windows::core::Interface::as_raw(self), moduleid, methodid, pbnewilmethodheader).ok()
    }
    pub unsafe fn GetAppDomainInfo(&self, appdomainid: usize, pcchname: *mut u32, szname: &mut [u16], pprocessid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetAppDomainInfo)(::windows::core::Interface::as_raw(self), appdomainid, szname.len() as _, pcchname, ::core::mem::transmute(szname.as_ptr()), pprocessid).ok()
    }
    pub unsafe fn GetAssemblyInfo(&self, assemblyid: usize, pcchname: *mut u32, szname: &mut [u16], pappdomainid: *mut usize, pmoduleid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetAssemblyInfo)(::windows::core::Interface::as_raw(self), assemblyid, szname.len() as _, pcchname, ::core::mem::transmute(szname.as_ptr()), pappdomainid, pmoduleid).ok()
    }
    pub unsafe fn SetFunctionReJIT(&self, functionid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.SetFunctionReJIT)(::windows::core::Interface::as_raw(self), functionid).ok()
    }
    pub unsafe fn ForceGC(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.ForceGC)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetILInstrumentedCodeMap<P0>(&self, functionid: usize, fstartjit: P0, cilmapentries: u32, rgilmapentries: *mut COR_IL_MAP) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.SetILInstrumentedCodeMap)(::windows::core::Interface::as_raw(self), functionid, fstartjit.into_param().abi(), cilmapentries, rgilmapentries).ok()
    }
    pub unsafe fn GetInprocInspectionInterface(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetInprocInspectionInterface)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetInprocInspectionIThisThread(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetInprocInspectionIThisThread)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetThreadContext(&self, threadid: usize, pcontextid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetThreadContext)(::windows::core::Interface::as_raw(self), threadid, pcontextid).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BeginInprocDebugging<P0>(&self, fthisthreadonly: P0, pdwprofilercontext: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.BeginInprocDebugging)(::windows::core::Interface::as_raw(self), fthisthreadonly.into_param().abi(), pdwprofilercontext).ok()
    }
    pub unsafe fn EndInprocDebugging(&self, dwprofilercontext: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.EndInprocDebugging)(::windows::core::Interface::as_raw(self), dwprofilercontext).ok()
    }
    pub unsafe fn GetILToNativeMapping(&self, functionid: usize, cmap: u32, pcmap: *mut u32, map: *mut COR_DEBUG_IL_TO_NATIVE_MAP) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetILToNativeMapping)(::windows::core::Interface::as_raw(self), functionid, cmap, pcmap, map).ok()
    }
    pub unsafe fn DoStackSnapshot(&self, thread: usize, callback: *mut StackSnapshotCallback, infoflags: u32, clientdata: *mut ::core::ffi::c_void, context: *mut u8, contextsize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.DoStackSnapshot)(::windows::core::Interface::as_raw(self), thread, callback, infoflags, clientdata, context, contextsize).ok()
    }
    pub unsafe fn SetEnterLeaveFunctionHooks2(&self, pfuncenter: *mut FunctionEnter2, pfuncleave: *mut FunctionLeave2, pfunctailcall: *mut FunctionTailcall2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.SetEnterLeaveFunctionHooks2)(::windows::core::Interface::as_raw(self), pfuncenter, pfuncleave, pfunctailcall).ok()
    }
    pub unsafe fn GetFunctionInfo2(&self, funcid: usize, frameinfo: usize, pclassid: *mut usize, pmoduleid: *mut usize, ptoken: *mut u32, ctypeargs: u32, pctypeargs: *mut u32, typeargs: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetFunctionInfo2)(::windows::core::Interface::as_raw(self), funcid, frameinfo, pclassid, pmoduleid, ptoken, ctypeargs, pctypeargs, typeargs).ok()
    }
    pub unsafe fn GetStringLayout(&self, pbufferlengthoffset: *mut u32, pstringlengthoffset: *mut u32, pbufferoffset: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetStringLayout)(::windows::core::Interface::as_raw(self), pbufferlengthoffset, pstringlengthoffset, pbufferoffset).ok()
    }
    #[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
    #[cfg(feature = "Win32_System_WinRT_Metadata")]
    pub unsafe fn GetClassLayout(&self, classid: usize, rfieldoffset: *mut super::super::WinRT::Metadata::COR_FIELD_OFFSET, cfieldoffset: u32, pcfieldoffset: *mut u32, pulclasssize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetClassLayout)(::windows::core::Interface::as_raw(self), classid, rfieldoffset, cfieldoffset, pcfieldoffset, pulclasssize).ok()
    }
    pub unsafe fn GetClassIDInfo2(&self, classid: usize, pmoduleid: *mut usize, ptypedeftoken: *mut u32, pparentclassid: *mut usize, cnumtypeargs: u32, pcnumtypeargs: *mut u32, typeargs: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetClassIDInfo2)(::windows::core::Interface::as_raw(self), classid, pmoduleid, ptypedeftoken, pparentclassid, cnumtypeargs, pcnumtypeargs, typeargs).ok()
    }
    pub unsafe fn GetCodeInfo2(&self, functionid: usize, ccodeinfos: u32, pccodeinfos: *mut u32, codeinfos: *mut COR_PRF_CODE_INFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetCodeInfo2)(::windows::core::Interface::as_raw(self), functionid, ccodeinfos, pccodeinfos, codeinfos).ok()
    }
    pub unsafe fn GetClassFromTokenAndTypeArgs(&self, moduleid: usize, typedef: u32, ctypeargs: u32, typeargs: *mut usize, pclassid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetClassFromTokenAndTypeArgs)(::windows::core::Interface::as_raw(self), moduleid, typedef, ctypeargs, typeargs, pclassid).ok()
    }
    pub unsafe fn GetFunctionFromTokenAndTypeArgs(&self, moduleid: usize, funcdef: u32, classid: usize, ctypeargs: u32, typeargs: *mut usize, pfunctionid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetFunctionFromTokenAndTypeArgs)(::windows::core::Interface::as_raw(self), moduleid, funcdef, classid, ctypeargs, typeargs, pfunctionid).ok()
    }
    pub unsafe fn EnumModuleFrozenObjects(&self, moduleid: usize) -> ::windows::core::Result<ICorProfilerObjectEnum> {
        let mut result__ = ::windows::core::zeroed::<ICorProfilerObjectEnum>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.EnumModuleFrozenObjects)(::windows::core::Interface::as_raw(self), moduleid, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetArrayObjectInfo(&self, objectid: usize, cdimensions: u32, pdimensionsizes: *mut u32, pdimensionlowerbounds: *mut i32, ppdata: *mut *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetArrayObjectInfo)(::windows::core::Interface::as_raw(self), objectid, cdimensions, pdimensionsizes, pdimensionlowerbounds, ppdata).ok()
    }
    pub unsafe fn GetBoxClassLayout(&self, classid: usize, pbufferoffset: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetBoxClassLayout)(::windows::core::Interface::as_raw(self), classid, pbufferoffset).ok()
    }
    pub unsafe fn GetThreadAppDomain(&self, threadid: usize, pappdomainid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetThreadAppDomain)(::windows::core::Interface::as_raw(self), threadid, pappdomainid).ok()
    }
    pub unsafe fn GetRVAStaticAddress(&self, classid: usize, fieldtoken: u32, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetRVAStaticAddress)(::windows::core::Interface::as_raw(self), classid, fieldtoken, ppaddress).ok()
    }
    pub unsafe fn GetAppDomainStaticAddress(&self, classid: usize, fieldtoken: u32, appdomainid: usize, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetAppDomainStaticAddress)(::windows::core::Interface::as_raw(self), classid, fieldtoken, appdomainid, ppaddress).ok()
    }
    pub unsafe fn GetThreadStaticAddress(&self, classid: usize, fieldtoken: u32, threadid: usize, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetThreadStaticAddress)(::windows::core::Interface::as_raw(self), classid, fieldtoken, threadid, ppaddress).ok()
    }
    pub unsafe fn GetContextStaticAddress(&self, classid: usize, fieldtoken: u32, contextid: usize, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetContextStaticAddress)(::windows::core::Interface::as_raw(self), classid, fieldtoken, contextid, ppaddress).ok()
    }
    pub unsafe fn GetStaticFieldInfo(&self, classid: usize, fieldtoken: u32, pfieldinfo: *mut COR_PRF_STATIC_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetStaticFieldInfo)(::windows::core::Interface::as_raw(self), classid, fieldtoken, pfieldinfo).ok()
    }
    pub unsafe fn GetGenerationBounds(&self, cobjectranges: u32, pcobjectranges: *mut u32, ranges: *mut COR_PRF_GC_GENERATION_RANGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetGenerationBounds)(::windows::core::Interface::as_raw(self), cobjectranges, pcobjectranges, ranges).ok()
    }
    pub unsafe fn GetObjectGeneration(&self, objectid: usize, range: *mut COR_PRF_GC_GENERATION_RANGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetObjectGeneration)(::windows::core::Interface::as_raw(self), objectid, range).ok()
    }
    pub unsafe fn GetNotifiedExceptionClauseInfo(&self, pinfo: *mut COR_PRF_EX_CLAUSE_INFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetNotifiedExceptionClauseInfo)(::windows::core::Interface::as_raw(self), pinfo).ok()
    }
    pub unsafe fn EnumJITedFunctions(&self) -> ::windows::core::Result<ICorProfilerFunctionEnum> {
        let mut result__ = ::windows::core::zeroed::<ICorProfilerFunctionEnum>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.EnumJITedFunctions)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn RequestProfilerDetach(&self, dwexpectedcompletionmilliseconds: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.RequestProfilerDetach)(::windows::core::Interface::as_raw(self), dwexpectedcompletionmilliseconds).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFunctionIDMapper2(&self, pfunc: *mut FunctionIDMapper2, clientdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetFunctionIDMapper2)(::windows::core::Interface::as_raw(self), pfunc, clientdata).ok()
    }
    pub unsafe fn GetStringLayout2(&self, pstringlengthoffset: *mut u32, pbufferoffset: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetStringLayout2)(::windows::core::Interface::as_raw(self), pstringlengthoffset, pbufferoffset).ok()
    }
    pub unsafe fn SetEnterLeaveFunctionHooks3(&self, pfuncenter3: *mut FunctionEnter3, pfuncleave3: *mut FunctionLeave3, pfunctailcall3: *mut FunctionTailcall3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetEnterLeaveFunctionHooks3)(::windows::core::Interface::as_raw(self), pfuncenter3, pfuncleave3, pfunctailcall3).ok()
    }
    pub unsafe fn SetEnterLeaveFunctionHooks3WithInfo(&self, pfuncenter3withinfo: *mut FunctionEnter3WithInfo, pfuncleave3withinfo: *mut FunctionLeave3WithInfo, pfunctailcall3withinfo: *mut FunctionTailcall3WithInfo) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetEnterLeaveFunctionHooks3WithInfo)(::windows::core::Interface::as_raw(self), pfuncenter3withinfo, pfuncleave3withinfo, pfunctailcall3withinfo).ok()
    }
    pub unsafe fn GetFunctionEnter3Info(&self, functionid: usize, eltinfo: usize, pframeinfo: *mut usize, pcbargumentinfo: *mut u32, pargumentinfo: *mut COR_PRF_FUNCTION_ARGUMENT_INFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetFunctionEnter3Info)(::windows::core::Interface::as_raw(self), functionid, eltinfo, pframeinfo, pcbargumentinfo, pargumentinfo).ok()
    }
    pub unsafe fn GetFunctionLeave3Info(&self, functionid: usize, eltinfo: usize, pframeinfo: *mut usize, pretvalrange: *mut COR_PRF_FUNCTION_ARGUMENT_RANGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetFunctionLeave3Info)(::windows::core::Interface::as_raw(self), functionid, eltinfo, pframeinfo, pretvalrange).ok()
    }
    pub unsafe fn GetFunctionTailcall3Info(&self, functionid: usize, eltinfo: usize, pframeinfo: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetFunctionTailcall3Info)(::windows::core::Interface::as_raw(self), functionid, eltinfo, pframeinfo).ok()
    }
    pub unsafe fn EnumModules(&self) -> ::windows::core::Result<ICorProfilerModuleEnum> {
        let mut result__ = ::windows::core::zeroed::<ICorProfilerModuleEnum>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.EnumModules)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetRuntimeInformation(&self, pclrinstanceid: *mut u16, pruntimetype: *mut COR_PRF_RUNTIME_TYPE, pmajorversion: *mut u16, pminorversion: *mut u16, pbuildnumber: *mut u16, pqfeversion: *mut u16, pcchversionstring: *mut u32, szversionstring: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetRuntimeInformation)(::windows::core::Interface::as_raw(self), pclrinstanceid, pruntimetype, pmajorversion, pminorversion, pbuildnumber, pqfeversion, szversionstring.len() as _, pcchversionstring, ::core::mem::transmute(szversionstring.as_ptr())).ok()
    }
    pub unsafe fn GetThreadStaticAddress2(&self, classid: usize, fieldtoken: u32, appdomainid: usize, threadid: usize, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetThreadStaticAddress2)(::windows::core::Interface::as_raw(self), classid, fieldtoken, appdomainid, threadid, ppaddress).ok()
    }
    pub unsafe fn GetAppDomainsContainingModule(&self, moduleid: usize, cappdomainids: u32, pcappdomainids: *mut u32, appdomainids: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetAppDomainsContainingModule)(::windows::core::Interface::as_raw(self), moduleid, cappdomainids, pcappdomainids, appdomainids).ok()
    }
    pub unsafe fn GetModuleInfo2(&self, moduleid: usize, ppbaseloadaddress: *mut *mut u8, pcchname: *mut u32, szname: &mut [u16], passemblyid: *mut usize, pdwmoduleflags: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetModuleInfo2)(::windows::core::Interface::as_raw(self), moduleid, ppbaseloadaddress, szname.len() as _, pcchname, ::core::mem::transmute(szname.as_ptr()), passemblyid, pdwmoduleflags).ok()
    }
    pub unsafe fn EnumThreads(&self) -> ::windows::core::Result<ICorProfilerThreadEnum> {
        let mut result__ = ::windows::core::zeroed::<ICorProfilerThreadEnum>();
        (::windows::core::Interface::vtable(self).base__.base__.EnumThreads)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn InitializeCurrentThread(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.InitializeCurrentThread)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RequestReJIT(&self, cfunctions: u32, moduleids: *mut usize, methodids: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.RequestReJIT)(::windows::core::Interface::as_raw(self), cfunctions, moduleids, methodids).ok()
    }
    pub unsafe fn RequestRevert(&self, cfunctions: u32, moduleids: *mut usize, methodids: *mut u32, status: *mut ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.RequestRevert)(::windows::core::Interface::as_raw(self), cfunctions, moduleids, methodids, status).ok()
    }
    pub unsafe fn GetCodeInfo3(&self, functionid: usize, rejitid: usize, ccodeinfos: u32, pccodeinfos: *mut u32, codeinfos: *mut COR_PRF_CODE_INFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetCodeInfo3)(::windows::core::Interface::as_raw(self), functionid, rejitid, ccodeinfos, pccodeinfos, codeinfos).ok()
    }
    pub unsafe fn GetFunctionFromIP2(&self, ip: *mut u8, pfunctionid: *mut usize, prejitid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetFunctionFromIP2)(::windows::core::Interface::as_raw(self), ip, pfunctionid, prejitid).ok()
    }
    pub unsafe fn GetReJITIDs(&self, functionid: usize, crejitids: u32, pcrejitids: *mut u32, rejitids: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetReJITIDs)(::windows::core::Interface::as_raw(self), functionid, crejitids, pcrejitids, rejitids).ok()
    }
    pub unsafe fn GetILToNativeMapping2(&self, functionid: usize, rejitid: usize, cmap: u32, pcmap: *mut u32, map: *mut COR_DEBUG_IL_TO_NATIVE_MAP) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetILToNativeMapping2)(::windows::core::Interface::as_raw(self), functionid, rejitid, cmap, pcmap, map).ok()
    }
    pub unsafe fn EnumJITedFunctions2(&self) -> ::windows::core::Result<ICorProfilerFunctionEnum> {
        let mut result__ = ::windows::core::zeroed::<ICorProfilerFunctionEnum>();
        (::windows::core::Interface::vtable(self).base__.base__.EnumJITedFunctions2)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetObjectSize2(&self, objectid: usize, pcsize: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetObjectSize2)(::windows::core::Interface::as_raw(self), objectid, pcsize).ok()
    }
    pub unsafe fn GetEventMask2(&self, pdweventslow: *mut u32, pdweventshigh: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetEventMask2)(::windows::core::Interface::as_raw(self), pdweventslow, pdweventshigh).ok()
    }
    pub unsafe fn SetEventMask2(&self, dweventslow: u32, dweventshigh: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetEventMask2)(::windows::core::Interface::as_raw(self), dweventslow, dweventshigh).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumNgenModuleMethodsInliningThisMethod(&self, inlinersmoduleid: usize, inlineemoduleid: usize, inlineemethodid: u32, incompletedata: *mut super::super::super::Foundation::BOOL, ppenum: *mut ::core::option::Option<ICorProfilerMethodEnum>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumNgenModuleMethodsInliningThisMethod)(::windows::core::Interface::as_raw(self), inlinersmoduleid, inlineemoduleid, inlineemethodid, incompletedata, ::core::mem::transmute(ppenum)).ok()
    }
}
::windows::imp::interface_hierarchy!(ICorProfilerInfo6, ::windows::core::IUnknown, ICorProfilerInfo, ICorProfilerInfo2, ICorProfilerInfo3, ICorProfilerInfo4, ICorProfilerInfo5);
impl ::core::cmp::PartialEq for ICorProfilerInfo6 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICorProfilerInfo6 {}
impl ::core::fmt::Debug for ICorProfilerInfo6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICorProfilerInfo6").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICorProfilerInfo6 {
    type Vtable = ICorProfilerInfo6_Vtbl;
}
impl ::core::clone::Clone for ICorProfilerInfo6 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICorProfilerInfo6 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf30a070d_bffb_46a7_b1d8_8781ef7b698a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorProfilerInfo6_Vtbl {
    pub base__: ICorProfilerInfo5_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub EnumNgenModuleMethodsInliningThisMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inlinersmoduleid: usize, inlineemoduleid: usize, inlineemethodid: u32, incompletedata: *mut super::super::super::Foundation::BOOL, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnumNgenModuleMethodsInliningThisMethod: usize,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
#[repr(transparent)]
pub struct ICorProfilerInfo7(::windows::core::IUnknown);
impl ICorProfilerInfo7 {
    pub unsafe fn GetClassFromObject(&self, objectid: usize, pclassid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.GetClassFromObject)(::windows::core::Interface::as_raw(self), objectid, pclassid).ok()
    }
    pub unsafe fn GetClassFromToken(&self, moduleid: usize, typedef: u32, pclassid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.GetClassFromToken)(::windows::core::Interface::as_raw(self), moduleid, typedef, pclassid).ok()
    }
    pub unsafe fn GetCodeInfo(&self, functionid: usize, pstart: *mut *mut u8, pcsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.GetCodeInfo)(::windows::core::Interface::as_raw(self), functionid, pstart, pcsize).ok()
    }
    pub unsafe fn GetEventMask(&self, pdwevents: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.GetEventMask)(::windows::core::Interface::as_raw(self), pdwevents).ok()
    }
    pub unsafe fn GetFunctionFromIP(&self, ip: *mut u8, pfunctionid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.GetFunctionFromIP)(::windows::core::Interface::as_raw(self), ip, pfunctionid).ok()
    }
    pub unsafe fn GetFunctionFromToken(&self, moduleid: usize, token: u32, pfunctionid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.GetFunctionFromToken)(::windows::core::Interface::as_raw(self), moduleid, token, pfunctionid).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHandleFromThread(&self, threadid: usize, phthread: *mut super::super::super::Foundation::HANDLE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.GetHandleFromThread)(::windows::core::Interface::as_raw(self), threadid, phthread).ok()
    }
    pub unsafe fn GetObjectSize(&self, objectid: usize, pcsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.GetObjectSize)(::windows::core::Interface::as_raw(self), objectid, pcsize).ok()
    }
    #[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
    #[cfg(feature = "Win32_System_WinRT_Metadata")]
    pub unsafe fn IsArrayClass(&self, classid: usize, pbaseelemtype: *mut super::super::WinRT::Metadata::CorElementType, pbaseclassid: *mut usize, pcrank: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.IsArrayClass)(::windows::core::Interface::as_raw(self), classid, pbaseelemtype, pbaseclassid, pcrank).ok()
    }
    pub unsafe fn GetThreadInfo(&self, threadid: usize, pdwwin32threadid: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.GetThreadInfo)(::windows::core::Interface::as_raw(self), threadid, pdwwin32threadid).ok()
    }
    pub unsafe fn GetCurrentThreadID(&self, pthreadid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.GetCurrentThreadID)(::windows::core::Interface::as_raw(self), pthreadid).ok()
    }
    pub unsafe fn GetClassIDInfo(&self, classid: usize, pmoduleid: *mut usize, ptypedeftoken: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.GetClassIDInfo)(::windows::core::Interface::as_raw(self), classid, pmoduleid, ptypedeftoken).ok()
    }
    pub unsafe fn GetFunctionInfo(&self, functionid: usize, pclassid: *mut usize, pmoduleid: *mut usize, ptoken: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.GetFunctionInfo)(::windows::core::Interface::as_raw(self), functionid, pclassid, pmoduleid, ptoken).ok()
    }
    pub unsafe fn SetEventMask(&self, dwevents: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.SetEventMask)(::windows::core::Interface::as_raw(self), dwevents).ok()
    }
    pub unsafe fn SetEnterLeaveFunctionHooks(&self, pfuncenter: *mut FunctionEnter, pfuncleave: *mut FunctionLeave, pfunctailcall: *mut FunctionTailcall) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.SetEnterLeaveFunctionHooks)(::windows::core::Interface::as_raw(self), pfuncenter, pfuncleave, pfunctailcall).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFunctionIDMapper(&self, pfunc: *mut FunctionIDMapper) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.SetFunctionIDMapper)(::windows::core::Interface::as_raw(self), pfunc).ok()
    }
    pub unsafe fn GetTokenAndMetaDataFromFunction(&self, functionid: usize, riid: *const ::windows::core::GUID, ppimport: *mut ::core::option::Option<::windows::core::IUnknown>, ptoken: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.GetTokenAndMetaDataFromFunction)(::windows::core::Interface::as_raw(self), functionid, riid, ::core::mem::transmute(ppimport), ptoken).ok()
    }
    pub unsafe fn GetModuleInfo(&self, moduleid: usize, ppbaseloadaddress: *mut *mut u8, pcchname: *mut u32, szname: &mut [u16], passemblyid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.GetModuleInfo)(::windows::core::Interface::as_raw(self), moduleid, ppbaseloadaddress, szname.len() as _, pcchname, ::core::mem::transmute(szname.as_ptr()), passemblyid).ok()
    }
    pub unsafe fn GetModuleMetaData(&self, moduleid: usize, dwopenflags: u32, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.GetModuleMetaData)(::windows::core::Interface::as_raw(self), moduleid, dwopenflags, riid, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetILFunctionBody(&self, moduleid: usize, methodid: u32, ppmethodheader: *mut *mut u8, pcbmethodsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.GetILFunctionBody)(::windows::core::Interface::as_raw(self), moduleid, methodid, ppmethodheader, pcbmethodsize).ok()
    }
    pub unsafe fn GetILFunctionBodyAllocator(&self, moduleid: usize) -> ::windows::core::Result<IMethodMalloc> {
        let mut result__ = ::windows::core::zeroed::<IMethodMalloc>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.GetILFunctionBodyAllocator)(::windows::core::Interface::as_raw(self), moduleid, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetILFunctionBody(&self, moduleid: usize, methodid: u32, pbnewilmethodheader: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.SetILFunctionBody)(::windows::core::Interface::as_raw(self), moduleid, methodid, pbnewilmethodheader).ok()
    }
    pub unsafe fn GetAppDomainInfo(&self, appdomainid: usize, pcchname: *mut u32, szname: &mut [u16], pprocessid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.GetAppDomainInfo)(::windows::core::Interface::as_raw(self), appdomainid, szname.len() as _, pcchname, ::core::mem::transmute(szname.as_ptr()), pprocessid).ok()
    }
    pub unsafe fn GetAssemblyInfo(&self, assemblyid: usize, pcchname: *mut u32, szname: &mut [u16], pappdomainid: *mut usize, pmoduleid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.GetAssemblyInfo)(::windows::core::Interface::as_raw(self), assemblyid, szname.len() as _, pcchname, ::core::mem::transmute(szname.as_ptr()), pappdomainid, pmoduleid).ok()
    }
    pub unsafe fn SetFunctionReJIT(&self, functionid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.SetFunctionReJIT)(::windows::core::Interface::as_raw(self), functionid).ok()
    }
    pub unsafe fn ForceGC(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.ForceGC)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetILInstrumentedCodeMap<P0>(&self, functionid: usize, fstartjit: P0, cilmapentries: u32, rgilmapentries: *mut COR_IL_MAP) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.SetILInstrumentedCodeMap)(::windows::core::Interface::as_raw(self), functionid, fstartjit.into_param().abi(), cilmapentries, rgilmapentries).ok()
    }
    pub unsafe fn GetInprocInspectionInterface(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.GetInprocInspectionInterface)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetInprocInspectionIThisThread(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.GetInprocInspectionIThisThread)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetThreadContext(&self, threadid: usize, pcontextid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.GetThreadContext)(::windows::core::Interface::as_raw(self), threadid, pcontextid).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BeginInprocDebugging<P0>(&self, fthisthreadonly: P0, pdwprofilercontext: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.BeginInprocDebugging)(::windows::core::Interface::as_raw(self), fthisthreadonly.into_param().abi(), pdwprofilercontext).ok()
    }
    pub unsafe fn EndInprocDebugging(&self, dwprofilercontext: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.EndInprocDebugging)(::windows::core::Interface::as_raw(self), dwprofilercontext).ok()
    }
    pub unsafe fn GetILToNativeMapping(&self, functionid: usize, cmap: u32, pcmap: *mut u32, map: *mut COR_DEBUG_IL_TO_NATIVE_MAP) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.GetILToNativeMapping)(::windows::core::Interface::as_raw(self), functionid, cmap, pcmap, map).ok()
    }
    pub unsafe fn DoStackSnapshot(&self, thread: usize, callback: *mut StackSnapshotCallback, infoflags: u32, clientdata: *mut ::core::ffi::c_void, context: *mut u8, contextsize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.DoStackSnapshot)(::windows::core::Interface::as_raw(self), thread, callback, infoflags, clientdata, context, contextsize).ok()
    }
    pub unsafe fn SetEnterLeaveFunctionHooks2(&self, pfuncenter: *mut FunctionEnter2, pfuncleave: *mut FunctionLeave2, pfunctailcall: *mut FunctionTailcall2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.SetEnterLeaveFunctionHooks2)(::windows::core::Interface::as_raw(self), pfuncenter, pfuncleave, pfunctailcall).ok()
    }
    pub unsafe fn GetFunctionInfo2(&self, funcid: usize, frameinfo: usize, pclassid: *mut usize, pmoduleid: *mut usize, ptoken: *mut u32, ctypeargs: u32, pctypeargs: *mut u32, typeargs: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetFunctionInfo2)(::windows::core::Interface::as_raw(self), funcid, frameinfo, pclassid, pmoduleid, ptoken, ctypeargs, pctypeargs, typeargs).ok()
    }
    pub unsafe fn GetStringLayout(&self, pbufferlengthoffset: *mut u32, pstringlengthoffset: *mut u32, pbufferoffset: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetStringLayout)(::windows::core::Interface::as_raw(self), pbufferlengthoffset, pstringlengthoffset, pbufferoffset).ok()
    }
    #[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
    #[cfg(feature = "Win32_System_WinRT_Metadata")]
    pub unsafe fn GetClassLayout(&self, classid: usize, rfieldoffset: *mut super::super::WinRT::Metadata::COR_FIELD_OFFSET, cfieldoffset: u32, pcfieldoffset: *mut u32, pulclasssize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetClassLayout)(::windows::core::Interface::as_raw(self), classid, rfieldoffset, cfieldoffset, pcfieldoffset, pulclasssize).ok()
    }
    pub unsafe fn GetClassIDInfo2(&self, classid: usize, pmoduleid: *mut usize, ptypedeftoken: *mut u32, pparentclassid: *mut usize, cnumtypeargs: u32, pcnumtypeargs: *mut u32, typeargs: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetClassIDInfo2)(::windows::core::Interface::as_raw(self), classid, pmoduleid, ptypedeftoken, pparentclassid, cnumtypeargs, pcnumtypeargs, typeargs).ok()
    }
    pub unsafe fn GetCodeInfo2(&self, functionid: usize, ccodeinfos: u32, pccodeinfos: *mut u32, codeinfos: *mut COR_PRF_CODE_INFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetCodeInfo2)(::windows::core::Interface::as_raw(self), functionid, ccodeinfos, pccodeinfos, codeinfos).ok()
    }
    pub unsafe fn GetClassFromTokenAndTypeArgs(&self, moduleid: usize, typedef: u32, ctypeargs: u32, typeargs: *mut usize, pclassid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetClassFromTokenAndTypeArgs)(::windows::core::Interface::as_raw(self), moduleid, typedef, ctypeargs, typeargs, pclassid).ok()
    }
    pub unsafe fn GetFunctionFromTokenAndTypeArgs(&self, moduleid: usize, funcdef: u32, classid: usize, ctypeargs: u32, typeargs: *mut usize, pfunctionid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetFunctionFromTokenAndTypeArgs)(::windows::core::Interface::as_raw(self), moduleid, funcdef, classid, ctypeargs, typeargs, pfunctionid).ok()
    }
    pub unsafe fn EnumModuleFrozenObjects(&self, moduleid: usize) -> ::windows::core::Result<ICorProfilerObjectEnum> {
        let mut result__ = ::windows::core::zeroed::<ICorProfilerObjectEnum>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.EnumModuleFrozenObjects)(::windows::core::Interface::as_raw(self), moduleid, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetArrayObjectInfo(&self, objectid: usize, cdimensions: u32, pdimensionsizes: *mut u32, pdimensionlowerbounds: *mut i32, ppdata: *mut *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetArrayObjectInfo)(::windows::core::Interface::as_raw(self), objectid, cdimensions, pdimensionsizes, pdimensionlowerbounds, ppdata).ok()
    }
    pub unsafe fn GetBoxClassLayout(&self, classid: usize, pbufferoffset: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetBoxClassLayout)(::windows::core::Interface::as_raw(self), classid, pbufferoffset).ok()
    }
    pub unsafe fn GetThreadAppDomain(&self, threadid: usize, pappdomainid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetThreadAppDomain)(::windows::core::Interface::as_raw(self), threadid, pappdomainid).ok()
    }
    pub unsafe fn GetRVAStaticAddress(&self, classid: usize, fieldtoken: u32, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetRVAStaticAddress)(::windows::core::Interface::as_raw(self), classid, fieldtoken, ppaddress).ok()
    }
    pub unsafe fn GetAppDomainStaticAddress(&self, classid: usize, fieldtoken: u32, appdomainid: usize, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetAppDomainStaticAddress)(::windows::core::Interface::as_raw(self), classid, fieldtoken, appdomainid, ppaddress).ok()
    }
    pub unsafe fn GetThreadStaticAddress(&self, classid: usize, fieldtoken: u32, threadid: usize, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetThreadStaticAddress)(::windows::core::Interface::as_raw(self), classid, fieldtoken, threadid, ppaddress).ok()
    }
    pub unsafe fn GetContextStaticAddress(&self, classid: usize, fieldtoken: u32, contextid: usize, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetContextStaticAddress)(::windows::core::Interface::as_raw(self), classid, fieldtoken, contextid, ppaddress).ok()
    }
    pub unsafe fn GetStaticFieldInfo(&self, classid: usize, fieldtoken: u32, pfieldinfo: *mut COR_PRF_STATIC_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetStaticFieldInfo)(::windows::core::Interface::as_raw(self), classid, fieldtoken, pfieldinfo).ok()
    }
    pub unsafe fn GetGenerationBounds(&self, cobjectranges: u32, pcobjectranges: *mut u32, ranges: *mut COR_PRF_GC_GENERATION_RANGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetGenerationBounds)(::windows::core::Interface::as_raw(self), cobjectranges, pcobjectranges, ranges).ok()
    }
    pub unsafe fn GetObjectGeneration(&self, objectid: usize, range: *mut COR_PRF_GC_GENERATION_RANGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetObjectGeneration)(::windows::core::Interface::as_raw(self), objectid, range).ok()
    }
    pub unsafe fn GetNotifiedExceptionClauseInfo(&self, pinfo: *mut COR_PRF_EX_CLAUSE_INFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetNotifiedExceptionClauseInfo)(::windows::core::Interface::as_raw(self), pinfo).ok()
    }
    pub unsafe fn EnumJITedFunctions(&self) -> ::windows::core::Result<ICorProfilerFunctionEnum> {
        let mut result__ = ::windows::core::zeroed::<ICorProfilerFunctionEnum>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.EnumJITedFunctions)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn RequestProfilerDetach(&self, dwexpectedcompletionmilliseconds: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.RequestProfilerDetach)(::windows::core::Interface::as_raw(self), dwexpectedcompletionmilliseconds).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFunctionIDMapper2(&self, pfunc: *mut FunctionIDMapper2, clientdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.SetFunctionIDMapper2)(::windows::core::Interface::as_raw(self), pfunc, clientdata).ok()
    }
    pub unsafe fn GetStringLayout2(&self, pstringlengthoffset: *mut u32, pbufferoffset: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetStringLayout2)(::windows::core::Interface::as_raw(self), pstringlengthoffset, pbufferoffset).ok()
    }
    pub unsafe fn SetEnterLeaveFunctionHooks3(&self, pfuncenter3: *mut FunctionEnter3, pfuncleave3: *mut FunctionLeave3, pfunctailcall3: *mut FunctionTailcall3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.SetEnterLeaveFunctionHooks3)(::windows::core::Interface::as_raw(self), pfuncenter3, pfuncleave3, pfunctailcall3).ok()
    }
    pub unsafe fn SetEnterLeaveFunctionHooks3WithInfo(&self, pfuncenter3withinfo: *mut FunctionEnter3WithInfo, pfuncleave3withinfo: *mut FunctionLeave3WithInfo, pfunctailcall3withinfo: *mut FunctionTailcall3WithInfo) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.SetEnterLeaveFunctionHooks3WithInfo)(::windows::core::Interface::as_raw(self), pfuncenter3withinfo, pfuncleave3withinfo, pfunctailcall3withinfo).ok()
    }
    pub unsafe fn GetFunctionEnter3Info(&self, functionid: usize, eltinfo: usize, pframeinfo: *mut usize, pcbargumentinfo: *mut u32, pargumentinfo: *mut COR_PRF_FUNCTION_ARGUMENT_INFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetFunctionEnter3Info)(::windows::core::Interface::as_raw(self), functionid, eltinfo, pframeinfo, pcbargumentinfo, pargumentinfo).ok()
    }
    pub unsafe fn GetFunctionLeave3Info(&self, functionid: usize, eltinfo: usize, pframeinfo: *mut usize, pretvalrange: *mut COR_PRF_FUNCTION_ARGUMENT_RANGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetFunctionLeave3Info)(::windows::core::Interface::as_raw(self), functionid, eltinfo, pframeinfo, pretvalrange).ok()
    }
    pub unsafe fn GetFunctionTailcall3Info(&self, functionid: usize, eltinfo: usize, pframeinfo: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetFunctionTailcall3Info)(::windows::core::Interface::as_raw(self), functionid, eltinfo, pframeinfo).ok()
    }
    pub unsafe fn EnumModules(&self) -> ::windows::core::Result<ICorProfilerModuleEnum> {
        let mut result__ = ::windows::core::zeroed::<ICorProfilerModuleEnum>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.EnumModules)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetRuntimeInformation(&self, pclrinstanceid: *mut u16, pruntimetype: *mut COR_PRF_RUNTIME_TYPE, pmajorversion: *mut u16, pminorversion: *mut u16, pbuildnumber: *mut u16, pqfeversion: *mut u16, pcchversionstring: *mut u32, szversionstring: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetRuntimeInformation)(::windows::core::Interface::as_raw(self), pclrinstanceid, pruntimetype, pmajorversion, pminorversion, pbuildnumber, pqfeversion, szversionstring.len() as _, pcchversionstring, ::core::mem::transmute(szversionstring.as_ptr())).ok()
    }
    pub unsafe fn GetThreadStaticAddress2(&self, classid: usize, fieldtoken: u32, appdomainid: usize, threadid: usize, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetThreadStaticAddress2)(::windows::core::Interface::as_raw(self), classid, fieldtoken, appdomainid, threadid, ppaddress).ok()
    }
    pub unsafe fn GetAppDomainsContainingModule(&self, moduleid: usize, cappdomainids: u32, pcappdomainids: *mut u32, appdomainids: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetAppDomainsContainingModule)(::windows::core::Interface::as_raw(self), moduleid, cappdomainids, pcappdomainids, appdomainids).ok()
    }
    pub unsafe fn GetModuleInfo2(&self, moduleid: usize, ppbaseloadaddress: *mut *mut u8, pcchname: *mut u32, szname: &mut [u16], passemblyid: *mut usize, pdwmoduleflags: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetModuleInfo2)(::windows::core::Interface::as_raw(self), moduleid, ppbaseloadaddress, szname.len() as _, pcchname, ::core::mem::transmute(szname.as_ptr()), passemblyid, pdwmoduleflags).ok()
    }
    pub unsafe fn EnumThreads(&self) -> ::windows::core::Result<ICorProfilerThreadEnum> {
        let mut result__ = ::windows::core::zeroed::<ICorProfilerThreadEnum>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.EnumThreads)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn InitializeCurrentThread(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.InitializeCurrentThread)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RequestReJIT(&self, cfunctions: u32, moduleids: *mut usize, methodids: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.RequestReJIT)(::windows::core::Interface::as_raw(self), cfunctions, moduleids, methodids).ok()
    }
    pub unsafe fn RequestRevert(&self, cfunctions: u32, moduleids: *mut usize, methodids: *mut u32, status: *mut ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.RequestRevert)(::windows::core::Interface::as_raw(self), cfunctions, moduleids, methodids, status).ok()
    }
    pub unsafe fn GetCodeInfo3(&self, functionid: usize, rejitid: usize, ccodeinfos: u32, pccodeinfos: *mut u32, codeinfos: *mut COR_PRF_CODE_INFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetCodeInfo3)(::windows::core::Interface::as_raw(self), functionid, rejitid, ccodeinfos, pccodeinfos, codeinfos).ok()
    }
    pub unsafe fn GetFunctionFromIP2(&self, ip: *mut u8, pfunctionid: *mut usize, prejitid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetFunctionFromIP2)(::windows::core::Interface::as_raw(self), ip, pfunctionid, prejitid).ok()
    }
    pub unsafe fn GetReJITIDs(&self, functionid: usize, crejitids: u32, pcrejitids: *mut u32, rejitids: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetReJITIDs)(::windows::core::Interface::as_raw(self), functionid, crejitids, pcrejitids, rejitids).ok()
    }
    pub unsafe fn GetILToNativeMapping2(&self, functionid: usize, rejitid: usize, cmap: u32, pcmap: *mut u32, map: *mut COR_DEBUG_IL_TO_NATIVE_MAP) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetILToNativeMapping2)(::windows::core::Interface::as_raw(self), functionid, rejitid, cmap, pcmap, map).ok()
    }
    pub unsafe fn EnumJITedFunctions2(&self) -> ::windows::core::Result<ICorProfilerFunctionEnum> {
        let mut result__ = ::windows::core::zeroed::<ICorProfilerFunctionEnum>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.EnumJITedFunctions2)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetObjectSize2(&self, objectid: usize, pcsize: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetObjectSize2)(::windows::core::Interface::as_raw(self), objectid, pcsize).ok()
    }
    pub unsafe fn GetEventMask2(&self, pdweventslow: *mut u32, pdweventshigh: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetEventMask2)(::windows::core::Interface::as_raw(self), pdweventslow, pdweventshigh).ok()
    }
    pub unsafe fn SetEventMask2(&self, dweventslow: u32, dweventshigh: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetEventMask2)(::windows::core::Interface::as_raw(self), dweventslow, dweventshigh).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumNgenModuleMethodsInliningThisMethod(&self, inlinersmoduleid: usize, inlineemoduleid: usize, inlineemethodid: u32, incompletedata: *mut super::super::super::Foundation::BOOL, ppenum: *mut ::core::option::Option<ICorProfilerMethodEnum>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.EnumNgenModuleMethodsInliningThisMethod)(::windows::core::Interface::as_raw(self), inlinersmoduleid, inlineemoduleid, inlineemethodid, incompletedata, ::core::mem::transmute(ppenum)).ok()
    }
    pub unsafe fn ApplyMetaData(&self, moduleid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ApplyMetaData)(::windows::core::Interface::as_raw(self), moduleid).ok()
    }
    pub unsafe fn GetInMemorySymbolsLength(&self, moduleid: usize, pcountsymbolbytes: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetInMemorySymbolsLength)(::windows::core::Interface::as_raw(self), moduleid, pcountsymbolbytes).ok()
    }
    pub unsafe fn ReadInMemorySymbols(&self, moduleid: usize, symbolsreadoffset: u32, psymbolbytes: *mut u8, countsymbolbytes: u32, pcountsymbolbytesread: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReadInMemorySymbols)(::windows::core::Interface::as_raw(self), moduleid, symbolsreadoffset, psymbolbytes, countsymbolbytes, pcountsymbolbytesread).ok()
    }
}
::windows::imp::interface_hierarchy!(ICorProfilerInfo7, ::windows::core::IUnknown, ICorProfilerInfo, ICorProfilerInfo2, ICorProfilerInfo3, ICorProfilerInfo4, ICorProfilerInfo5, ICorProfilerInfo6);
impl ::core::cmp::PartialEq for ICorProfilerInfo7 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICorProfilerInfo7 {}
impl ::core::fmt::Debug for ICorProfilerInfo7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICorProfilerInfo7").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICorProfilerInfo7 {
    type Vtable = ICorProfilerInfo7_Vtbl;
}
impl ::core::clone::Clone for ICorProfilerInfo7 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICorProfilerInfo7 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9aeecc0d_63e0_4187_8c00_e312f503f663);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorProfilerInfo7_Vtbl {
    pub base__: ICorProfilerInfo6_Vtbl,
    pub ApplyMetaData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduleid: usize) -> ::windows::core::HRESULT,
    pub GetInMemorySymbolsLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduleid: usize, pcountsymbolbytes: *mut u32) -> ::windows::core::HRESULT,
    pub ReadInMemorySymbols: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduleid: usize, symbolsreadoffset: u32, psymbolbytes: *mut u8, countsymbolbytes: u32, pcountsymbolbytesread: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
#[repr(transparent)]
pub struct ICorProfilerInfo8(::windows::core::IUnknown);
impl ICorProfilerInfo8 {
    pub unsafe fn GetClassFromObject(&self, objectid: usize, pclassid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetClassFromObject)(::windows::core::Interface::as_raw(self), objectid, pclassid).ok()
    }
    pub unsafe fn GetClassFromToken(&self, moduleid: usize, typedef: u32, pclassid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetClassFromToken)(::windows::core::Interface::as_raw(self), moduleid, typedef, pclassid).ok()
    }
    pub unsafe fn GetCodeInfo(&self, functionid: usize, pstart: *mut *mut u8, pcsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetCodeInfo)(::windows::core::Interface::as_raw(self), functionid, pstart, pcsize).ok()
    }
    pub unsafe fn GetEventMask(&self, pdwevents: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetEventMask)(::windows::core::Interface::as_raw(self), pdwevents).ok()
    }
    pub unsafe fn GetFunctionFromIP(&self, ip: *mut u8, pfunctionid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetFunctionFromIP)(::windows::core::Interface::as_raw(self), ip, pfunctionid).ok()
    }
    pub unsafe fn GetFunctionFromToken(&self, moduleid: usize, token: u32, pfunctionid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetFunctionFromToken)(::windows::core::Interface::as_raw(self), moduleid, token, pfunctionid).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHandleFromThread(&self, threadid: usize, phthread: *mut super::super::super::Foundation::HANDLE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetHandleFromThread)(::windows::core::Interface::as_raw(self), threadid, phthread).ok()
    }
    pub unsafe fn GetObjectSize(&self, objectid: usize, pcsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetObjectSize)(::windows::core::Interface::as_raw(self), objectid, pcsize).ok()
    }
    #[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
    #[cfg(feature = "Win32_System_WinRT_Metadata")]
    pub unsafe fn IsArrayClass(&self, classid: usize, pbaseelemtype: *mut super::super::WinRT::Metadata::CorElementType, pbaseclassid: *mut usize, pcrank: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.IsArrayClass)(::windows::core::Interface::as_raw(self), classid, pbaseelemtype, pbaseclassid, pcrank).ok()
    }
    pub unsafe fn GetThreadInfo(&self, threadid: usize, pdwwin32threadid: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetThreadInfo)(::windows::core::Interface::as_raw(self), threadid, pdwwin32threadid).ok()
    }
    pub unsafe fn GetCurrentThreadID(&self, pthreadid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetCurrentThreadID)(::windows::core::Interface::as_raw(self), pthreadid).ok()
    }
    pub unsafe fn GetClassIDInfo(&self, classid: usize, pmoduleid: *mut usize, ptypedeftoken: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetClassIDInfo)(::windows::core::Interface::as_raw(self), classid, pmoduleid, ptypedeftoken).ok()
    }
    pub unsafe fn GetFunctionInfo(&self, functionid: usize, pclassid: *mut usize, pmoduleid: *mut usize, ptoken: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetFunctionInfo)(::windows::core::Interface::as_raw(self), functionid, pclassid, pmoduleid, ptoken).ok()
    }
    pub unsafe fn SetEventMask(&self, dwevents: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.SetEventMask)(::windows::core::Interface::as_raw(self), dwevents).ok()
    }
    pub unsafe fn SetEnterLeaveFunctionHooks(&self, pfuncenter: *mut FunctionEnter, pfuncleave: *mut FunctionLeave, pfunctailcall: *mut FunctionTailcall) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.SetEnterLeaveFunctionHooks)(::windows::core::Interface::as_raw(self), pfuncenter, pfuncleave, pfunctailcall).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFunctionIDMapper(&self, pfunc: *mut FunctionIDMapper) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.SetFunctionIDMapper)(::windows::core::Interface::as_raw(self), pfunc).ok()
    }
    pub unsafe fn GetTokenAndMetaDataFromFunction(&self, functionid: usize, riid: *const ::windows::core::GUID, ppimport: *mut ::core::option::Option<::windows::core::IUnknown>, ptoken: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetTokenAndMetaDataFromFunction)(::windows::core::Interface::as_raw(self), functionid, riid, ::core::mem::transmute(ppimport), ptoken).ok()
    }
    pub unsafe fn GetModuleInfo(&self, moduleid: usize, ppbaseloadaddress: *mut *mut u8, pcchname: *mut u32, szname: &mut [u16], passemblyid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetModuleInfo)(::windows::core::Interface::as_raw(self), moduleid, ppbaseloadaddress, szname.len() as _, pcchname, ::core::mem::transmute(szname.as_ptr()), passemblyid).ok()
    }
    pub unsafe fn GetModuleMetaData(&self, moduleid: usize, dwopenflags: u32, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetModuleMetaData)(::windows::core::Interface::as_raw(self), moduleid, dwopenflags, riid, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetILFunctionBody(&self, moduleid: usize, methodid: u32, ppmethodheader: *mut *mut u8, pcbmethodsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetILFunctionBody)(::windows::core::Interface::as_raw(self), moduleid, methodid, ppmethodheader, pcbmethodsize).ok()
    }
    pub unsafe fn GetILFunctionBodyAllocator(&self, moduleid: usize) -> ::windows::core::Result<IMethodMalloc> {
        let mut result__ = ::windows::core::zeroed::<IMethodMalloc>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetILFunctionBodyAllocator)(::windows::core::Interface::as_raw(self), moduleid, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetILFunctionBody(&self, moduleid: usize, methodid: u32, pbnewilmethodheader: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.SetILFunctionBody)(::windows::core::Interface::as_raw(self), moduleid, methodid, pbnewilmethodheader).ok()
    }
    pub unsafe fn GetAppDomainInfo(&self, appdomainid: usize, pcchname: *mut u32, szname: &mut [u16], pprocessid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetAppDomainInfo)(::windows::core::Interface::as_raw(self), appdomainid, szname.len() as _, pcchname, ::core::mem::transmute(szname.as_ptr()), pprocessid).ok()
    }
    pub unsafe fn GetAssemblyInfo(&self, assemblyid: usize, pcchname: *mut u32, szname: &mut [u16], pappdomainid: *mut usize, pmoduleid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetAssemblyInfo)(::windows::core::Interface::as_raw(self), assemblyid, szname.len() as _, pcchname, ::core::mem::transmute(szname.as_ptr()), pappdomainid, pmoduleid).ok()
    }
    pub unsafe fn SetFunctionReJIT(&self, functionid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.SetFunctionReJIT)(::windows::core::Interface::as_raw(self), functionid).ok()
    }
    pub unsafe fn ForceGC(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.ForceGC)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetILInstrumentedCodeMap<P0>(&self, functionid: usize, fstartjit: P0, cilmapentries: u32, rgilmapentries: *mut COR_IL_MAP) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.SetILInstrumentedCodeMap)(::windows::core::Interface::as_raw(self), functionid, fstartjit.into_param().abi(), cilmapentries, rgilmapentries).ok()
    }
    pub unsafe fn GetInprocInspectionInterface(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetInprocInspectionInterface)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetInprocInspectionIThisThread(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetInprocInspectionIThisThread)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetThreadContext(&self, threadid: usize, pcontextid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetThreadContext)(::windows::core::Interface::as_raw(self), threadid, pcontextid).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BeginInprocDebugging<P0>(&self, fthisthreadonly: P0, pdwprofilercontext: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.BeginInprocDebugging)(::windows::core::Interface::as_raw(self), fthisthreadonly.into_param().abi(), pdwprofilercontext).ok()
    }
    pub unsafe fn EndInprocDebugging(&self, dwprofilercontext: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.EndInprocDebugging)(::windows::core::Interface::as_raw(self), dwprofilercontext).ok()
    }
    pub unsafe fn GetILToNativeMapping(&self, functionid: usize, cmap: u32, pcmap: *mut u32, map: *mut COR_DEBUG_IL_TO_NATIVE_MAP) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetILToNativeMapping)(::windows::core::Interface::as_raw(self), functionid, cmap, pcmap, map).ok()
    }
    pub unsafe fn DoStackSnapshot(&self, thread: usize, callback: *mut StackSnapshotCallback, infoflags: u32, clientdata: *mut ::core::ffi::c_void, context: *mut u8, contextsize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.DoStackSnapshot)(::windows::core::Interface::as_raw(self), thread, callback, infoflags, clientdata, context, contextsize).ok()
    }
    pub unsafe fn SetEnterLeaveFunctionHooks2(&self, pfuncenter: *mut FunctionEnter2, pfuncleave: *mut FunctionLeave2, pfunctailcall: *mut FunctionTailcall2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.SetEnterLeaveFunctionHooks2)(::windows::core::Interface::as_raw(self), pfuncenter, pfuncleave, pfunctailcall).ok()
    }
    pub unsafe fn GetFunctionInfo2(&self, funcid: usize, frameinfo: usize, pclassid: *mut usize, pmoduleid: *mut usize, ptoken: *mut u32, ctypeargs: u32, pctypeargs: *mut u32, typeargs: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.GetFunctionInfo2)(::windows::core::Interface::as_raw(self), funcid, frameinfo, pclassid, pmoduleid, ptoken, ctypeargs, pctypeargs, typeargs).ok()
    }
    pub unsafe fn GetStringLayout(&self, pbufferlengthoffset: *mut u32, pstringlengthoffset: *mut u32, pbufferoffset: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.GetStringLayout)(::windows::core::Interface::as_raw(self), pbufferlengthoffset, pstringlengthoffset, pbufferoffset).ok()
    }
    #[doc = "*Required features: `\"Win32_System_WinRT_Metadata\"`*"]
    #[cfg(feature = "Win32_System_WinRT_Metadata")]
    pub unsafe fn GetClassLayout(&self, classid: usize, rfieldoffset: *mut super::super::WinRT::Metadata::COR_FIELD_OFFSET, cfieldoffset: u32, pcfieldoffset: *mut u32, pulclasssize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.GetClassLayout)(::windows::core::Interface::as_raw(self), classid, rfieldoffset, cfieldoffset, pcfieldoffset, pulclasssize).ok()
    }
    pub unsafe fn GetClassIDInfo2(&self, classid: usize, pmoduleid: *mut usize, ptypedeftoken: *mut u32, pparentclassid: *mut usize, cnumtypeargs: u32, pcnumtypeargs: *mut u32, typeargs: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.GetClassIDInfo2)(::windows::core::Interface::as_raw(self), classid, pmoduleid, ptypedeftoken, pparentclassid, cnumtypeargs, pcnumtypeargs, typeargs).ok()
    }
    pub unsafe fn GetCodeInfo2(&self, functionid: usize, ccodeinfos: u32, pccodeinfos: *mut u32, codeinfos: *mut COR_PRF_CODE_INFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.GetCodeInfo2)(::windows::core::Interface::as_raw(self), functionid, ccodeinfos, pccodeinfos, codeinfos).ok()
    }
    pub unsafe fn GetClassFromTokenAndTypeArgs(&self, moduleid: usize, typedef: u32, ctypeargs: u32, typeargs: *mut usize, pclassid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.GetClassFromTokenAndTypeArgs)(::windows::core::Interface::as_raw(self), moduleid, typedef, ctypeargs, typeargs, pclassid).ok()
    }
    pub unsafe fn GetFunctionFromTokenAndTypeArgs(&self, moduleid: usize, funcdef: u32, classid: usize, ctypeargs: u32, typeargs: *mut usize, pfunctionid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.GetFunctionFromTokenAndTypeArgs)(::windows::core::Interface::as_raw(self), moduleid, funcdef, classid, ctypeargs, typeargs, pfunctionid).ok()
    }
    pub unsafe fn EnumModuleFrozenObjects(&self, moduleid: usize) -> ::windows::core::Result<ICorProfilerObjectEnum> {
        let mut result__ = ::windows::core::zeroed::<ICorProfilerObjectEnum>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.EnumModuleFrozenObjects)(::windows::core::Interface::as_raw(self), moduleid, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetArrayObjectInfo(&self, objectid: usize, cdimensions: u32, pdimensionsizes: *mut u32, pdimensionlowerbounds: *mut i32, ppdata: *mut *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.GetArrayObjectInfo)(::windows::core::Interface::as_raw(self), objectid, cdimensions, pdimensionsizes, pdimensionlowerbounds, ppdata).ok()
    }
    pub unsafe fn GetBoxClassLayout(&self, classid: usize, pbufferoffset: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.GetBoxClassLayout)(::windows::core::Interface::as_raw(self), classid, pbufferoffset).ok()
    }
    pub unsafe fn GetThreadAppDomain(&self, threadid: usize, pappdomainid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.GetThreadAppDomain)(::windows::core::Interface::as_raw(self), threadid, pappdomainid).ok()
    }
    pub unsafe fn GetRVAStaticAddress(&self, classid: usize, fieldtoken: u32, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.GetRVAStaticAddress)(::windows::core::Interface::as_raw(self), classid, fieldtoken, ppaddress).ok()
    }
    pub unsafe fn GetAppDomainStaticAddress(&self, classid: usize, fieldtoken: u32, appdomainid: usize, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.GetAppDomainStaticAddress)(::windows::core::Interface::as_raw(self), classid, fieldtoken, appdomainid, ppaddress).ok()
    }
    pub unsafe fn GetThreadStaticAddress(&self, classid: usize, fieldtoken: u32, threadid: usize, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.GetThreadStaticAddress)(::windows::core::Interface::as_raw(self), classid, fieldtoken, threadid, ppaddress).ok()
    }
    pub unsafe fn GetContextStaticAddress(&self, classid: usize, fieldtoken: u32, contextid: usize, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.GetContextStaticAddress)(::windows::core::Interface::as_raw(self), classid, fieldtoken, contextid, ppaddress).ok()
    }
    pub unsafe fn GetStaticFieldInfo(&self, classid: usize, fieldtoken: u32, pfieldinfo: *mut COR_PRF_STATIC_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.GetStaticFieldInfo)(::windows::core::Interface::as_raw(self), classid, fieldtoken, pfieldinfo).ok()
    }
    pub unsafe fn GetGenerationBounds(&self, cobjectranges: u32, pcobjectranges: *mut u32, ranges: *mut COR_PRF_GC_GENERATION_RANGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.GetGenerationBounds)(::windows::core::Interface::as_raw(self), cobjectranges, pcobjectranges, ranges).ok()
    }
    pub unsafe fn GetObjectGeneration(&self, objectid: usize, range: *mut COR_PRF_GC_GENERATION_RANGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.GetObjectGeneration)(::windows::core::Interface::as_raw(self), objectid, range).ok()
    }
    pub unsafe fn GetNotifiedExceptionClauseInfo(&self, pinfo: *mut COR_PRF_EX_CLAUSE_INFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.base__.GetNotifiedExceptionClauseInfo)(::windows::core::Interface::as_raw(self), pinfo).ok()
    }
    pub unsafe fn EnumJITedFunctions(&self) -> ::windows::core::Result<ICorProfilerFunctionEnum> {
        let mut result__ = ::windows::core::zeroed::<ICorProfilerFunctionEnum>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.EnumJITedFunctions)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn RequestProfilerDetach(&self, dwexpectedcompletionmilliseconds: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.RequestProfilerDetach)(::windows::core::Interface::as_raw(self), dwexpectedcompletionmilliseconds).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFunctionIDMapper2(&self, pfunc: *mut FunctionIDMapper2, clientdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.SetFunctionIDMapper2)(::windows::core::Interface::as_raw(self), pfunc, clientdata).ok()
    }
    pub unsafe fn GetStringLayout2(&self, pstringlengthoffset: *mut u32, pbufferoffset: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetStringLayout2)(::windows::core::Interface::as_raw(self), pstringlengthoffset, pbufferoffset).ok()
    }
    pub unsafe fn SetEnterLeaveFunctionHooks3(&self, pfuncenter3: *mut FunctionEnter3, pfuncleave3: *mut FunctionLeave3, pfunctailcall3: *mut FunctionTailcall3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.SetEnterLeaveFunctionHooks3)(::windows::core::Interface::as_raw(self), pfuncenter3, pfuncleave3, pfunctailcall3).ok()
    }
    pub unsafe fn SetEnterLeaveFunctionHooks3WithInfo(&self, pfuncenter3withinfo: *mut FunctionEnter3WithInfo, pfuncleave3withinfo: *mut FunctionLeave3WithInfo, pfunctailcall3withinfo: *mut FunctionTailcall3WithInfo) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.SetEnterLeaveFunctionHooks3WithInfo)(::windows::core::Interface::as_raw(self), pfuncenter3withinfo, pfuncleave3withinfo, pfunctailcall3withinfo).ok()
    }
    pub unsafe fn GetFunctionEnter3Info(&self, functionid: usize, eltinfo: usize, pframeinfo: *mut usize, pcbargumentinfo: *mut u32, pargumentinfo: *mut COR_PRF_FUNCTION_ARGUMENT_INFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetFunctionEnter3Info)(::windows::core::Interface::as_raw(self), functionid, eltinfo, pframeinfo, pcbargumentinfo, pargumentinfo).ok()
    }
    pub unsafe fn GetFunctionLeave3Info(&self, functionid: usize, eltinfo: usize, pframeinfo: *mut usize, pretvalrange: *mut COR_PRF_FUNCTION_ARGUMENT_RANGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetFunctionLeave3Info)(::windows::core::Interface::as_raw(self), functionid, eltinfo, pframeinfo, pretvalrange).ok()
    }
    pub unsafe fn GetFunctionTailcall3Info(&self, functionid: usize, eltinfo: usize, pframeinfo: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetFunctionTailcall3Info)(::windows::core::Interface::as_raw(self), functionid, eltinfo, pframeinfo).ok()
    }
    pub unsafe fn EnumModules(&self) -> ::windows::core::Result<ICorProfilerModuleEnum> {
        let mut result__ = ::windows::core::zeroed::<ICorProfilerModuleEnum>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.EnumModules)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetRuntimeInformation(&self, pclrinstanceid: *mut u16, pruntimetype: *mut COR_PRF_RUNTIME_TYPE, pmajorversion: *mut u16, pminorversion: *mut u16, pbuildnumber: *mut u16, pqfeversion: *mut u16, pcchversionstring: *mut u32, szversionstring: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetRuntimeInformation)(::windows::core::Interface::as_raw(self), pclrinstanceid, pruntimetype, pmajorversion, pminorversion, pbuildnumber, pqfeversion, szversionstring.len() as _, pcchversionstring, ::core::mem::transmute(szversionstring.as_ptr())).ok()
    }
    pub unsafe fn GetThreadStaticAddress2(&self, classid: usize, fieldtoken: u32, appdomainid: usize, threadid: usize, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetThreadStaticAddress2)(::windows::core::Interface::as_raw(self), classid, fieldtoken, appdomainid, threadid, ppaddress).ok()
    }
    pub unsafe fn GetAppDomainsContainingModule(&self, moduleid: usize, cappdomainids: u32, pcappdomainids: *mut u32, appdomainids: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetAppDomainsContainingModule)(::windows::core::Interface::as_raw(self), moduleid, cappdomainids, pcappdomainids, appdomainids).ok()
    }
    pub unsafe fn GetModuleInfo2(&self, moduleid: usize, ppbaseloadaddress: *mut *mut u8, pcchname: *mut u32, szname: &mut [u16], passemblyid: *mut usize, pdwmoduleflags: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetModuleInfo2)(::windows::core::Interface::as_raw(self), moduleid, ppbaseloadaddress, szname.len() as _, pcchname, ::core::mem::transmute(szname.as_ptr()), passemblyid, pdwmoduleflags).ok()
    }
    pub unsafe fn EnumThreads(&self) -> ::windows::core::Result<ICorProfilerThreadEnum> {
        let mut result__ = ::windows::core::zeroed::<ICorProfilerThreadEnum>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.EnumThreads)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn InitializeCurrentThread(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.InitializeCurrentThread)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RequestReJIT(&self, cfunctions: u32, moduleids: *mut usize, methodids: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.RequestReJIT)(::windows::core::Interface::as_raw(self), cfunctions, moduleids, methodids).ok()
    }
    pub unsafe fn RequestRevert(&self, cfunctions: u32, moduleids: *mut usize, methodids: *mut u32, status: *mut ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.RequestRevert)(::windows::core::Interface::as_raw(self), cfunctions, moduleids, methodids, status).ok()
    }
    pub unsafe fn GetCodeInfo3(&self, functionid: usize, rejitid: usize, ccodeinfos: u32, pccodeinfos: *mut u32, codeinfos: *mut COR_PRF_CODE_INFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetCodeInfo3)(::windows::core::Interface::as_raw(self), functionid, rejitid, ccodeinfos, pccodeinfos, codeinfos).ok()
    }
    pub unsafe fn GetFunctionFromIP2(&self, ip: *mut u8, pfunctionid: *mut usize, prejitid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetFunctionFromIP2)(::windows::core::Interface::as_raw(self), ip, pfunctionid, prejitid).ok()
    }
    pub unsafe fn GetReJITIDs(&self, functionid: usize, crejitids: u32, pcrejitids: *mut u32, rejitids: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetReJITIDs)(::windows::core::Interface::as_raw(self), functionid, crejitids, pcrejitids, rejitids).ok()
    }
    pub unsafe fn GetILToNativeMapping2(&self, functionid: usize, rejitid: usize, cmap: u32, pcmap: *mut u32, map: *mut COR_DEBUG_IL_TO_NATIVE_MAP) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetILToNativeMapping2)(::windows::core::Interface::as_raw(self), functionid, rejitid, cmap, pcmap, map).ok()
    }
    pub unsafe fn EnumJITedFunctions2(&self) -> ::windows::core::Result<ICorProfilerFunctionEnum> {
        let mut result__ = ::windows::core::zeroed::<ICorProfilerFunctionEnum>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.EnumJITedFunctions2)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetObjectSize2(&self, objectid: usize, pcsize: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetObjectSize2)(::windows::core::Interface::as_raw(self), objectid, pcsize).ok()
    }
    pub unsafe fn GetEventMask2(&self, pdweventslow: *mut u32, pdweventshigh: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetEventMask2)(::windows::core::Interface::as_raw(self), pdweventslow, pdweventshigh).ok()
    }
    pub unsafe fn SetEventMask2(&self, dweventslow: u32, dweventshigh: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetEventMask2)(::windows::core::Interface::as_raw(self), dweventslow, dweventshigh).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumNgenModuleMethodsInliningThisMethod(&self, inlinersmoduleid: usize, inlineemoduleid: usize, inlineemethodid: u32, incompletedata: *mut super::super::super::Foundation::BOOL, ppenum: *mut ::core::option::Option<ICorProfilerMethodEnum>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.EnumNgenModuleMethodsInliningThisMethod)(::windows::core::Interface::as_raw(self), inlinersmoduleid, inlineemoduleid, inlineemethodid, incompletedata, ::core::mem::transmute(ppenum)).ok()
    }
    pub unsafe fn ApplyMetaData(&self, moduleid: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ApplyMetaData)(::windows::core::Interface::as_raw(self), moduleid).ok()
    }
    pub unsafe fn GetInMemorySymbolsLength(&self, moduleid: usize, pcountsymbolbytes: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetInMemorySymbolsLength)(::windows::core::Interface::as_raw(self), moduleid, pcountsymbolbytes).ok()
    }
    pub unsafe fn ReadInMemorySymbols(&self, moduleid: usize, symbolsreadoffset: u32, psymbolbytes: *mut u8, countsymbolbytes: u32, pcountsymbolbytesread: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ReadInMemorySymbols)(::windows::core::Interface::as_raw(self), moduleid, symbolsreadoffset, psymbolbytes, countsymbolbytes, pcountsymbolbytesread).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsFunctionDynamic(&self, functionid: usize, isdynamic: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).IsFunctionDynamic)(::windows::core::Interface::as_raw(self), functionid, isdynamic).ok()
    }
    pub unsafe fn GetFunctionFromIP3(&self, ip: *mut u8, functionid: *mut usize, prejitid: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFunctionFromIP3)(::windows::core::Interface::as_raw(self), ip, functionid, prejitid).ok()
    }
    pub unsafe fn GetDynamicFunctionInfo<P0>(&self, functionid: usize, moduleid: *mut usize, ppvsig: *mut *mut u8, pbsig: *mut u32, cchname: u32, pcchname: *mut u32, wszname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).GetDynamicFunctionInfo)(::windows::core::Interface::as_raw(self), functionid, moduleid, ppvsig, pbsig, cchname, pcchname, wszname.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(ICorProfilerInfo8, ::windows::core::IUnknown, ICorProfilerInfo, ICorProfilerInfo2, ICorProfilerInfo3, ICorProfilerInfo4, ICorProfilerInfo5, ICorProfilerInfo6, ICorProfilerInfo7);
impl ::core::cmp::PartialEq for ICorProfilerInfo8 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICorProfilerInfo8 {}
impl ::core::fmt::Debug for ICorProfilerInfo8 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICorProfilerInfo8").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICorProfilerInfo8 {
    type Vtable = ICorProfilerInfo8_Vtbl;
}
impl ::core::clone::Clone for ICorProfilerInfo8 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICorProfilerInfo8 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc5ac80a6_782e_4716_8044_39598c60cfbf);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorProfilerInfo8_Vtbl {
    pub base__: ICorProfilerInfo7_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub IsFunctionDynamic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, functionid: usize, isdynamic: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsFunctionDynamic: usize,
    pub GetFunctionFromIP3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ip: *mut u8, functionid: *mut usize, prejitid: *mut usize) -> ::windows::core::HRESULT,
    pub GetDynamicFunctionInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, functionid: usize, moduleid: *mut usize, ppvsig: *mut *mut u8, pbsig: *mut u32, cchname: u32, pcchname: *mut u32, wszname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
#[repr(transparent)]
pub struct ICorProfilerMethodEnum(::windows::core::IUnknown);
impl ICorProfilerMethodEnum {
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<ICorProfilerMethodEnum> {
        let mut result__ = ::windows::core::zeroed::<ICorProfilerMethodEnum>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCount(&self, pcelt: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCount)(::windows::core::Interface::as_raw(self), pcelt).ok()
    }
    pub unsafe fn Next(&self, celt: u32, elements: *mut COR_PRF_METHOD, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), celt, elements, pceltfetched).ok()
    }
}
::windows::imp::interface_hierarchy!(ICorProfilerMethodEnum, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ICorProfilerMethodEnum {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICorProfilerMethodEnum {}
impl ::core::fmt::Debug for ICorProfilerMethodEnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICorProfilerMethodEnum").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICorProfilerMethodEnum {
    type Vtable = ICorProfilerMethodEnum_Vtbl;
}
impl ::core::clone::Clone for ICorProfilerMethodEnum {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICorProfilerMethodEnum {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfccee788_0088_454b_a811_c99f298d1942);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorProfilerMethodEnum_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcelt: *mut u32) -> ::windows::core::HRESULT,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, elements: *mut COR_PRF_METHOD, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
#[repr(transparent)]
pub struct ICorProfilerModuleEnum(::windows::core::IUnknown);
impl ICorProfilerModuleEnum {
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<ICorProfilerModuleEnum> {
        let mut result__ = ::windows::core::zeroed::<ICorProfilerModuleEnum>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCount(&self, pcelt: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCount)(::windows::core::Interface::as_raw(self), pcelt).ok()
    }
    pub unsafe fn Next(&self, celt: u32, ids: *mut usize, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), celt, ids, pceltfetched).ok()
    }
}
::windows::imp::interface_hierarchy!(ICorProfilerModuleEnum, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ICorProfilerModuleEnum {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICorProfilerModuleEnum {}
impl ::core::fmt::Debug for ICorProfilerModuleEnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICorProfilerModuleEnum").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICorProfilerModuleEnum {
    type Vtable = ICorProfilerModuleEnum_Vtbl;
}
impl ::core::clone::Clone for ICorProfilerModuleEnum {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICorProfilerModuleEnum {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb0266d75_2081_4493_af7f_028ba34db891);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorProfilerModuleEnum_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcelt: *mut u32) -> ::windows::core::HRESULT,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, ids: *mut usize, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
#[repr(transparent)]
pub struct ICorProfilerObjectEnum(::windows::core::IUnknown);
impl ICorProfilerObjectEnum {
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<ICorProfilerObjectEnum> {
        let mut result__ = ::windows::core::zeroed::<ICorProfilerObjectEnum>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCount(&self, pcelt: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCount)(::windows::core::Interface::as_raw(self), pcelt).ok()
    }
    pub unsafe fn Next(&self, celt: u32, objects: *mut usize, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), celt, objects, pceltfetched).ok()
    }
}
::windows::imp::interface_hierarchy!(ICorProfilerObjectEnum, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ICorProfilerObjectEnum {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICorProfilerObjectEnum {}
impl ::core::fmt::Debug for ICorProfilerObjectEnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICorProfilerObjectEnum").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICorProfilerObjectEnum {
    type Vtable = ICorProfilerObjectEnum_Vtbl;
}
impl ::core::clone::Clone for ICorProfilerObjectEnum {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICorProfilerObjectEnum {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c6269bd_2d13_4321_ae12_6686365fd6af);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorProfilerObjectEnum_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcelt: *mut u32) -> ::windows::core::HRESULT,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, objects: *mut usize, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
#[repr(transparent)]
pub struct ICorProfilerThreadEnum(::windows::core::IUnknown);
impl ICorProfilerThreadEnum {
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<ICorProfilerThreadEnum> {
        let mut result__ = ::windows::core::zeroed::<ICorProfilerThreadEnum>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCount(&self, pcelt: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCount)(::windows::core::Interface::as_raw(self), pcelt).ok()
    }
    pub unsafe fn Next(&self, celt: u32, ids: *mut usize, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), celt, ids, pceltfetched).ok()
    }
}
::windows::imp::interface_hierarchy!(ICorProfilerThreadEnum, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ICorProfilerThreadEnum {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICorProfilerThreadEnum {}
impl ::core::fmt::Debug for ICorProfilerThreadEnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICorProfilerThreadEnum").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICorProfilerThreadEnum {
    type Vtable = ICorProfilerThreadEnum_Vtbl;
}
impl ::core::clone::Clone for ICorProfilerThreadEnum {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICorProfilerThreadEnum {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x571194f7_25ed_419f_aa8b_7016b3159701);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorProfilerThreadEnum_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcelt: *mut u32) -> ::windows::core::HRESULT,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, ids: *mut usize, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
#[repr(transparent)]
pub struct IMethodMalloc(::windows::core::IUnknown);
impl IMethodMalloc {
    pub unsafe fn Alloc(&self, cb: u32) -> *mut ::core::ffi::c_void {
        (::windows::core::Interface::vtable(self).Alloc)(::windows::core::Interface::as_raw(self), cb)
    }
}
::windows::imp::interface_hierarchy!(IMethodMalloc, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMethodMalloc {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMethodMalloc {}
impl ::core::fmt::Debug for IMethodMalloc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMethodMalloc").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMethodMalloc {
    type Vtable = IMethodMalloc_Vtbl;
}
impl ::core::clone::Clone for IMethodMalloc {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMethodMalloc {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa0efb28b_6ee2_4d7b_b983_a75ef7beedb8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMethodMalloc_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Alloc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cb: u32) -> *mut ::core::ffi::c_void,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const CorDB_CONTROL_Profiling: ::windows::core::PCSTR = ::windows::core::s!("Cor_Enable_Profiling");
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const CorDB_CONTROL_ProfilingL: ::windows::core::PCWSTR = ::windows::core::w!("Cor_Enable_Profiling");
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COR_PRF_CLAUSE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_CLAUSE_NONE: COR_PRF_CLAUSE_TYPE = COR_PRF_CLAUSE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_CLAUSE_FILTER: COR_PRF_CLAUSE_TYPE = COR_PRF_CLAUSE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_CLAUSE_CATCH: COR_PRF_CLAUSE_TYPE = COR_PRF_CLAUSE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_CLAUSE_FINALLY: COR_PRF_CLAUSE_TYPE = COR_PRF_CLAUSE_TYPE(3i32);
impl ::core::marker::Copy for COR_PRF_CLAUSE_TYPE {}
impl ::core::clone::Clone for COR_PRF_CLAUSE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COR_PRF_CLAUSE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for COR_PRF_CLAUSE_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for COR_PRF_CLAUSE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COR_PRF_CLAUSE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COR_PRF_CODEGEN_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_CODEGEN_DISABLE_INLINING: COR_PRF_CODEGEN_FLAGS = COR_PRF_CODEGEN_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_CODEGEN_DISABLE_ALL_OPTIMIZATIONS: COR_PRF_CODEGEN_FLAGS = COR_PRF_CODEGEN_FLAGS(2i32);
impl ::core::marker::Copy for COR_PRF_CODEGEN_FLAGS {}
impl ::core::clone::Clone for COR_PRF_CODEGEN_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COR_PRF_CODEGEN_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for COR_PRF_CODEGEN_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for COR_PRF_CODEGEN_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COR_PRF_CODEGEN_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COR_PRF_FINALIZER_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_FINALIZER_CRITICAL: COR_PRF_FINALIZER_FLAGS = COR_PRF_FINALIZER_FLAGS(1i32);
impl ::core::marker::Copy for COR_PRF_FINALIZER_FLAGS {}
impl ::core::clone::Clone for COR_PRF_FINALIZER_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COR_PRF_FINALIZER_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for COR_PRF_FINALIZER_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for COR_PRF_FINALIZER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COR_PRF_FINALIZER_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COR_PRF_GC_GENERATION(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_GC_GEN_0: COR_PRF_GC_GENERATION = COR_PRF_GC_GENERATION(0i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_GC_GEN_1: COR_PRF_GC_GENERATION = COR_PRF_GC_GENERATION(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_GC_GEN_2: COR_PRF_GC_GENERATION = COR_PRF_GC_GENERATION(2i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_GC_LARGE_OBJECT_HEAP: COR_PRF_GC_GENERATION = COR_PRF_GC_GENERATION(3i32);
impl ::core::marker::Copy for COR_PRF_GC_GENERATION {}
impl ::core::clone::Clone for COR_PRF_GC_GENERATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COR_PRF_GC_GENERATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for COR_PRF_GC_GENERATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for COR_PRF_GC_GENERATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COR_PRF_GC_GENERATION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COR_PRF_GC_REASON(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_GC_INDUCED: COR_PRF_GC_REASON = COR_PRF_GC_REASON(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_GC_OTHER: COR_PRF_GC_REASON = COR_PRF_GC_REASON(0i32);
impl ::core::marker::Copy for COR_PRF_GC_REASON {}
impl ::core::clone::Clone for COR_PRF_GC_REASON {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COR_PRF_GC_REASON {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for COR_PRF_GC_REASON {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for COR_PRF_GC_REASON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COR_PRF_GC_REASON").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COR_PRF_GC_ROOT_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_GC_ROOT_PINNING: COR_PRF_GC_ROOT_FLAGS = COR_PRF_GC_ROOT_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_GC_ROOT_WEAKREF: COR_PRF_GC_ROOT_FLAGS = COR_PRF_GC_ROOT_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_GC_ROOT_INTERIOR: COR_PRF_GC_ROOT_FLAGS = COR_PRF_GC_ROOT_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_GC_ROOT_REFCOUNTED: COR_PRF_GC_ROOT_FLAGS = COR_PRF_GC_ROOT_FLAGS(8i32);
impl ::core::marker::Copy for COR_PRF_GC_ROOT_FLAGS {}
impl ::core::clone::Clone for COR_PRF_GC_ROOT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COR_PRF_GC_ROOT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for COR_PRF_GC_ROOT_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for COR_PRF_GC_ROOT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COR_PRF_GC_ROOT_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COR_PRF_GC_ROOT_KIND(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_GC_ROOT_STACK: COR_PRF_GC_ROOT_KIND = COR_PRF_GC_ROOT_KIND(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_GC_ROOT_FINALIZER: COR_PRF_GC_ROOT_KIND = COR_PRF_GC_ROOT_KIND(2i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_GC_ROOT_HANDLE: COR_PRF_GC_ROOT_KIND = COR_PRF_GC_ROOT_KIND(3i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_GC_ROOT_OTHER: COR_PRF_GC_ROOT_KIND = COR_PRF_GC_ROOT_KIND(0i32);
impl ::core::marker::Copy for COR_PRF_GC_ROOT_KIND {}
impl ::core::clone::Clone for COR_PRF_GC_ROOT_KIND {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COR_PRF_GC_ROOT_KIND {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for COR_PRF_GC_ROOT_KIND {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for COR_PRF_GC_ROOT_KIND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COR_PRF_GC_ROOT_KIND").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COR_PRF_HIGH_MONITOR(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_HIGH_MONITOR_NONE: COR_PRF_HIGH_MONITOR = COR_PRF_HIGH_MONITOR(0i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_HIGH_ADD_ASSEMBLY_REFERENCES: COR_PRF_HIGH_MONITOR = COR_PRF_HIGH_MONITOR(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_HIGH_IN_MEMORY_SYMBOLS_UPDATED: COR_PRF_HIGH_MONITOR = COR_PRF_HIGH_MONITOR(2i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_HIGH_MONITOR_DYNAMIC_FUNCTION_UNLOADS: COR_PRF_HIGH_MONITOR = COR_PRF_HIGH_MONITOR(4i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_HIGH_REQUIRE_PROFILE_IMAGE: COR_PRF_HIGH_MONITOR = COR_PRF_HIGH_MONITOR(0i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_HIGH_ALLOWABLE_AFTER_ATTACH: COR_PRF_HIGH_MONITOR = COR_PRF_HIGH_MONITOR(6i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_HIGH_MONITOR_IMMUTABLE: COR_PRF_HIGH_MONITOR = COR_PRF_HIGH_MONITOR(0i32);
impl ::core::marker::Copy for COR_PRF_HIGH_MONITOR {}
impl ::core::clone::Clone for COR_PRF_HIGH_MONITOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COR_PRF_HIGH_MONITOR {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for COR_PRF_HIGH_MONITOR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for COR_PRF_HIGH_MONITOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COR_PRF_HIGH_MONITOR").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COR_PRF_JIT_CACHE(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_CACHED_FUNCTION_FOUND: COR_PRF_JIT_CACHE = COR_PRF_JIT_CACHE(0i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_CACHED_FUNCTION_NOT_FOUND: COR_PRF_JIT_CACHE = COR_PRF_JIT_CACHE(1i32);
impl ::core::marker::Copy for COR_PRF_JIT_CACHE {}
impl ::core::clone::Clone for COR_PRF_JIT_CACHE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COR_PRF_JIT_CACHE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for COR_PRF_JIT_CACHE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for COR_PRF_JIT_CACHE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COR_PRF_JIT_CACHE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COR_PRF_MISC(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const PROFILER_PARENT_UNKNOWN: COR_PRF_MISC = COR_PRF_MISC(-3i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const PROFILER_GLOBAL_CLASS: COR_PRF_MISC = COR_PRF_MISC(-2i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const PROFILER_GLOBAL_MODULE: COR_PRF_MISC = COR_PRF_MISC(-1i32);
impl ::core::marker::Copy for COR_PRF_MISC {}
impl ::core::clone::Clone for COR_PRF_MISC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COR_PRF_MISC {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for COR_PRF_MISC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for COR_PRF_MISC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COR_PRF_MISC").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COR_PRF_MODULE_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_MODULE_DISK: COR_PRF_MODULE_FLAGS = COR_PRF_MODULE_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_MODULE_NGEN: COR_PRF_MODULE_FLAGS = COR_PRF_MODULE_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_MODULE_DYNAMIC: COR_PRF_MODULE_FLAGS = COR_PRF_MODULE_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_MODULE_COLLECTIBLE: COR_PRF_MODULE_FLAGS = COR_PRF_MODULE_FLAGS(8i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_MODULE_RESOURCE: COR_PRF_MODULE_FLAGS = COR_PRF_MODULE_FLAGS(16i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_MODULE_FLAT_LAYOUT: COR_PRF_MODULE_FLAGS = COR_PRF_MODULE_FLAGS(32i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_MODULE_WINDOWS_RUNTIME: COR_PRF_MODULE_FLAGS = COR_PRF_MODULE_FLAGS(64i32);
impl ::core::marker::Copy for COR_PRF_MODULE_FLAGS {}
impl ::core::clone::Clone for COR_PRF_MODULE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COR_PRF_MODULE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for COR_PRF_MODULE_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for COR_PRF_MODULE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COR_PRF_MODULE_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COR_PRF_MONITOR(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_MONITOR_NONE: COR_PRF_MONITOR = COR_PRF_MONITOR(0i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_MONITOR_FUNCTION_UNLOADS: COR_PRF_MONITOR = COR_PRF_MONITOR(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_MONITOR_CLASS_LOADS: COR_PRF_MONITOR = COR_PRF_MONITOR(2i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_MONITOR_MODULE_LOADS: COR_PRF_MONITOR = COR_PRF_MONITOR(4i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_MONITOR_ASSEMBLY_LOADS: COR_PRF_MONITOR = COR_PRF_MONITOR(8i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_MONITOR_APPDOMAIN_LOADS: COR_PRF_MONITOR = COR_PRF_MONITOR(16i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_MONITOR_JIT_COMPILATION: COR_PRF_MONITOR = COR_PRF_MONITOR(32i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_MONITOR_EXCEPTIONS: COR_PRF_MONITOR = COR_PRF_MONITOR(64i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_MONITOR_GC: COR_PRF_MONITOR = COR_PRF_MONITOR(128i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_MONITOR_OBJECT_ALLOCATED: COR_PRF_MONITOR = COR_PRF_MONITOR(256i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_MONITOR_THREADS: COR_PRF_MONITOR = COR_PRF_MONITOR(512i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_MONITOR_REMOTING: COR_PRF_MONITOR = COR_PRF_MONITOR(1024i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_MONITOR_CODE_TRANSITIONS: COR_PRF_MONITOR = COR_PRF_MONITOR(2048i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_MONITOR_ENTERLEAVE: COR_PRF_MONITOR = COR_PRF_MONITOR(4096i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_MONITOR_CCW: COR_PRF_MONITOR = COR_PRF_MONITOR(8192i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_MONITOR_REMOTING_COOKIE: COR_PRF_MONITOR = COR_PRF_MONITOR(17408i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_MONITOR_REMOTING_ASYNC: COR_PRF_MONITOR = COR_PRF_MONITOR(33792i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_MONITOR_SUSPENDS: COR_PRF_MONITOR = COR_PRF_MONITOR(65536i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_MONITOR_CACHE_SEARCHES: COR_PRF_MONITOR = COR_PRF_MONITOR(131072i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_ENABLE_REJIT: COR_PRF_MONITOR = COR_PRF_MONITOR(262144i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_ENABLE_INPROC_DEBUGGING: COR_PRF_MONITOR = COR_PRF_MONITOR(524288i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_ENABLE_JIT_MAPS: COR_PRF_MONITOR = COR_PRF_MONITOR(1048576i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_DISABLE_INLINING: COR_PRF_MONITOR = COR_PRF_MONITOR(2097152i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_DISABLE_OPTIMIZATIONS: COR_PRF_MONITOR = COR_PRF_MONITOR(4194304i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_ENABLE_OBJECT_ALLOCATED: COR_PRF_MONITOR = COR_PRF_MONITOR(8388608i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_MONITOR_CLR_EXCEPTIONS: COR_PRF_MONITOR = COR_PRF_MONITOR(16777216i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_MONITOR_ALL: COR_PRF_MONITOR = COR_PRF_MONITOR(17301503i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_ENABLE_FUNCTION_ARGS: COR_PRF_MONITOR = COR_PRF_MONITOR(33554432i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_ENABLE_FUNCTION_RETVAL: COR_PRF_MONITOR = COR_PRF_MONITOR(67108864i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_ENABLE_FRAME_INFO: COR_PRF_MONITOR = COR_PRF_MONITOR(134217728i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_ENABLE_STACK_SNAPSHOT: COR_PRF_MONITOR = COR_PRF_MONITOR(268435456i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_USE_PROFILE_IMAGES: COR_PRF_MONITOR = COR_PRF_MONITOR(536870912i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_DISABLE_TRANSPARENCY_CHECKS_UNDER_FULL_TRUST: COR_PRF_MONITOR = COR_PRF_MONITOR(1073741824i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_DISABLE_ALL_NGEN_IMAGES: COR_PRF_MONITOR = COR_PRF_MONITOR(-2147483648i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_ALL: COR_PRF_MONITOR = COR_PRF_MONITOR(-1879048193i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_REQUIRE_PROFILE_IMAGE: COR_PRF_MONITOR = COR_PRF_MONITOR(536877056i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_ALLOWABLE_AFTER_ATTACH: COR_PRF_MONITOR = COR_PRF_MONITOR(268501758i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_MONITOR_IMMUTABLE: COR_PRF_MONITOR = COR_PRF_MONITOR(-285422592i32);
impl ::core::marker::Copy for COR_PRF_MONITOR {}
impl ::core::clone::Clone for COR_PRF_MONITOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COR_PRF_MONITOR {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for COR_PRF_MONITOR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for COR_PRF_MONITOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COR_PRF_MONITOR").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COR_PRF_RUNTIME_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_DESKTOP_CLR: COR_PRF_RUNTIME_TYPE = COR_PRF_RUNTIME_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_CORE_CLR: COR_PRF_RUNTIME_TYPE = COR_PRF_RUNTIME_TYPE(2i32);
impl ::core::marker::Copy for COR_PRF_RUNTIME_TYPE {}
impl ::core::clone::Clone for COR_PRF_RUNTIME_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COR_PRF_RUNTIME_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for COR_PRF_RUNTIME_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for COR_PRF_RUNTIME_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COR_PRF_RUNTIME_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COR_PRF_SNAPSHOT_INFO(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_SNAPSHOT_DEFAULT: COR_PRF_SNAPSHOT_INFO = COR_PRF_SNAPSHOT_INFO(0i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_SNAPSHOT_REGISTER_CONTEXT: COR_PRF_SNAPSHOT_INFO = COR_PRF_SNAPSHOT_INFO(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_SNAPSHOT_X86_OPTIMIZED: COR_PRF_SNAPSHOT_INFO = COR_PRF_SNAPSHOT_INFO(2i32);
impl ::core::marker::Copy for COR_PRF_SNAPSHOT_INFO {}
impl ::core::clone::Clone for COR_PRF_SNAPSHOT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COR_PRF_SNAPSHOT_INFO {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for COR_PRF_SNAPSHOT_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for COR_PRF_SNAPSHOT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COR_PRF_SNAPSHOT_INFO").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COR_PRF_STATIC_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_FIELD_NOT_A_STATIC: COR_PRF_STATIC_TYPE = COR_PRF_STATIC_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_FIELD_APP_DOMAIN_STATIC: COR_PRF_STATIC_TYPE = COR_PRF_STATIC_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_FIELD_THREAD_STATIC: COR_PRF_STATIC_TYPE = COR_PRF_STATIC_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_FIELD_CONTEXT_STATIC: COR_PRF_STATIC_TYPE = COR_PRF_STATIC_TYPE(4i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_FIELD_RVA_STATIC: COR_PRF_STATIC_TYPE = COR_PRF_STATIC_TYPE(8i32);
impl ::core::marker::Copy for COR_PRF_STATIC_TYPE {}
impl ::core::clone::Clone for COR_PRF_STATIC_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COR_PRF_STATIC_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for COR_PRF_STATIC_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for COR_PRF_STATIC_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COR_PRF_STATIC_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COR_PRF_SUSPEND_REASON(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_SUSPEND_OTHER: COR_PRF_SUSPEND_REASON = COR_PRF_SUSPEND_REASON(0i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_SUSPEND_FOR_GC: COR_PRF_SUSPEND_REASON = COR_PRF_SUSPEND_REASON(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_SUSPEND_FOR_APPDOMAIN_SHUTDOWN: COR_PRF_SUSPEND_REASON = COR_PRF_SUSPEND_REASON(2i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_SUSPEND_FOR_CODE_PITCHING: COR_PRF_SUSPEND_REASON = COR_PRF_SUSPEND_REASON(3i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_SUSPEND_FOR_SHUTDOWN: COR_PRF_SUSPEND_REASON = COR_PRF_SUSPEND_REASON(4i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_SUSPEND_FOR_INPROC_DEBUGGER: COR_PRF_SUSPEND_REASON = COR_PRF_SUSPEND_REASON(6i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_SUSPEND_FOR_GC_PREP: COR_PRF_SUSPEND_REASON = COR_PRF_SUSPEND_REASON(7i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_SUSPEND_FOR_REJIT: COR_PRF_SUSPEND_REASON = COR_PRF_SUSPEND_REASON(8i32);
impl ::core::marker::Copy for COR_PRF_SUSPEND_REASON {}
impl ::core::clone::Clone for COR_PRF_SUSPEND_REASON {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COR_PRF_SUSPEND_REASON {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for COR_PRF_SUSPEND_REASON {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for COR_PRF_SUSPEND_REASON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COR_PRF_SUSPEND_REASON").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COR_PRF_TRANSITION_REASON(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_TRANSITION_CALL: COR_PRF_TRANSITION_REASON = COR_PRF_TRANSITION_REASON(0i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const COR_PRF_TRANSITION_RETURN: COR_PRF_TRANSITION_REASON = COR_PRF_TRANSITION_REASON(1i32);
impl ::core::marker::Copy for COR_PRF_TRANSITION_REASON {}
impl ::core::clone::Clone for COR_PRF_TRANSITION_REASON {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COR_PRF_TRANSITION_REASON {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for COR_PRF_TRANSITION_REASON {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for COR_PRF_TRANSITION_REASON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COR_PRF_TRANSITION_REASON").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CorDebugIlToNativeMappingTypes(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const NO_MAPPING: CorDebugIlToNativeMappingTypes = CorDebugIlToNativeMappingTypes(-1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const PROLOG: CorDebugIlToNativeMappingTypes = CorDebugIlToNativeMappingTypes(-2i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub const EPILOG: CorDebugIlToNativeMappingTypes = CorDebugIlToNativeMappingTypes(-3i32);
impl ::core::marker::Copy for CorDebugIlToNativeMappingTypes {}
impl ::core::clone::Clone for CorDebugIlToNativeMappingTypes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CorDebugIlToNativeMappingTypes {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CorDebugIlToNativeMappingTypes {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CorDebugIlToNativeMappingTypes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CorDebugIlToNativeMappingTypes").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub struct COR_DEBUG_IL_TO_NATIVE_MAP {
    pub ilOffset: u32,
    pub nativeStartOffset: u32,
    pub nativeEndOffset: u32,
}
impl ::core::marker::Copy for COR_DEBUG_IL_TO_NATIVE_MAP {}
impl ::core::clone::Clone for COR_DEBUG_IL_TO_NATIVE_MAP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COR_DEBUG_IL_TO_NATIVE_MAP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COR_DEBUG_IL_TO_NATIVE_MAP").field("ilOffset", &self.ilOffset).field("nativeStartOffset", &self.nativeStartOffset).field("nativeEndOffset", &self.nativeEndOffset).finish()
    }
}
impl ::windows::core::TypeKind for COR_DEBUG_IL_TO_NATIVE_MAP {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for COR_DEBUG_IL_TO_NATIVE_MAP {
    fn eq(&self, other: &Self) -> bool {
        self.ilOffset == other.ilOffset && self.nativeStartOffset == other.nativeStartOffset && self.nativeEndOffset == other.nativeEndOffset
    }
}
impl ::core::cmp::Eq for COR_DEBUG_IL_TO_NATIVE_MAP {}
impl ::core::default::Default for COR_DEBUG_IL_TO_NATIVE_MAP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct COR_IL_MAP {
    pub oldOffset: u32,
    pub newOffset: u32,
    pub fAccurate: super::super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for COR_IL_MAP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for COR_IL_MAP {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for COR_IL_MAP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COR_IL_MAP").field("oldOffset", &self.oldOffset).field("newOffset", &self.newOffset).field("fAccurate", &self.fAccurate).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for COR_IL_MAP {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for COR_IL_MAP {
    fn eq(&self, other: &Self) -> bool {
        self.oldOffset == other.oldOffset && self.newOffset == other.newOffset && self.fAccurate == other.fAccurate
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for COR_IL_MAP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for COR_IL_MAP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`, `\"Win32_System_WinRT_Metadata\"`*"]
#[cfg(feature = "Win32_System_WinRT_Metadata")]
pub struct COR_PRF_ASSEMBLY_REFERENCE_INFO {
    pub pbPublicKeyOrToken: *mut ::core::ffi::c_void,
    pub cbPublicKeyOrToken: u32,
    pub szName: ::windows::core::PCWSTR,
    pub pMetaData: *mut super::super::WinRT::Metadata::ASSEMBLYMETADATA,
    pub pbHashValue: *mut ::core::ffi::c_void,
    pub cbHashValue: u32,
    pub dwAssemblyRefFlags: u32,
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl ::core::marker::Copy for COR_PRF_ASSEMBLY_REFERENCE_INFO {}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl ::core::clone::Clone for COR_PRF_ASSEMBLY_REFERENCE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl ::core::fmt::Debug for COR_PRF_ASSEMBLY_REFERENCE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COR_PRF_ASSEMBLY_REFERENCE_INFO").field("pbPublicKeyOrToken", &self.pbPublicKeyOrToken).field("cbPublicKeyOrToken", &self.cbPublicKeyOrToken).field("szName", &self.szName).field("pMetaData", &self.pMetaData).field("pbHashValue", &self.pbHashValue).field("cbHashValue", &self.cbHashValue).field("dwAssemblyRefFlags", &self.dwAssemblyRefFlags).finish()
    }
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl ::windows::core::TypeKind for COR_PRF_ASSEMBLY_REFERENCE_INFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl ::core::cmp::PartialEq for COR_PRF_ASSEMBLY_REFERENCE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pbPublicKeyOrToken == other.pbPublicKeyOrToken && self.cbPublicKeyOrToken == other.cbPublicKeyOrToken && self.szName == other.szName && self.pMetaData == other.pMetaData && self.pbHashValue == other.pbHashValue && self.cbHashValue == other.cbHashValue && self.dwAssemblyRefFlags == other.dwAssemblyRefFlags
    }
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl ::core::cmp::Eq for COR_PRF_ASSEMBLY_REFERENCE_INFO {}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl ::core::default::Default for COR_PRF_ASSEMBLY_REFERENCE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub struct COR_PRF_CODE_INFO {
    pub startAddress: usize,
    pub size: usize,
}
impl ::core::marker::Copy for COR_PRF_CODE_INFO {}
impl ::core::clone::Clone for COR_PRF_CODE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COR_PRF_CODE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COR_PRF_CODE_INFO").field("startAddress", &self.startAddress).field("size", &self.size).finish()
    }
}
impl ::windows::core::TypeKind for COR_PRF_CODE_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for COR_PRF_CODE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.startAddress == other.startAddress && self.size == other.size
    }
}
impl ::core::cmp::Eq for COR_PRF_CODE_INFO {}
impl ::core::default::Default for COR_PRF_CODE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub struct COR_PRF_EX_CLAUSE_INFO {
    pub clauseType: COR_PRF_CLAUSE_TYPE,
    pub programCounter: usize,
    pub framePointer: usize,
    pub shadowStackPointer: usize,
}
impl ::core::marker::Copy for COR_PRF_EX_CLAUSE_INFO {}
impl ::core::clone::Clone for COR_PRF_EX_CLAUSE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COR_PRF_EX_CLAUSE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COR_PRF_EX_CLAUSE_INFO").field("clauseType", &self.clauseType).field("programCounter", &self.programCounter).field("framePointer", &self.framePointer).field("shadowStackPointer", &self.shadowStackPointer).finish()
    }
}
impl ::windows::core::TypeKind for COR_PRF_EX_CLAUSE_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for COR_PRF_EX_CLAUSE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.clauseType == other.clauseType && self.programCounter == other.programCounter && self.framePointer == other.framePointer && self.shadowStackPointer == other.shadowStackPointer
    }
}
impl ::core::cmp::Eq for COR_PRF_EX_CLAUSE_INFO {}
impl ::core::default::Default for COR_PRF_EX_CLAUSE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub struct COR_PRF_FUNCTION {
    pub functionId: usize,
    pub reJitId: usize,
}
impl ::core::marker::Copy for COR_PRF_FUNCTION {}
impl ::core::clone::Clone for COR_PRF_FUNCTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COR_PRF_FUNCTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COR_PRF_FUNCTION").field("functionId", &self.functionId).field("reJitId", &self.reJitId).finish()
    }
}
impl ::windows::core::TypeKind for COR_PRF_FUNCTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for COR_PRF_FUNCTION {
    fn eq(&self, other: &Self) -> bool {
        self.functionId == other.functionId && self.reJitId == other.reJitId
    }
}
impl ::core::cmp::Eq for COR_PRF_FUNCTION {}
impl ::core::default::Default for COR_PRF_FUNCTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub struct COR_PRF_FUNCTION_ARGUMENT_INFO {
    pub numRanges: u32,
    pub totalArgumentSize: u32,
    pub ranges: [COR_PRF_FUNCTION_ARGUMENT_RANGE; 1],
}
impl ::core::marker::Copy for COR_PRF_FUNCTION_ARGUMENT_INFO {}
impl ::core::clone::Clone for COR_PRF_FUNCTION_ARGUMENT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COR_PRF_FUNCTION_ARGUMENT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COR_PRF_FUNCTION_ARGUMENT_INFO").field("numRanges", &self.numRanges).field("totalArgumentSize", &self.totalArgumentSize).field("ranges", &self.ranges).finish()
    }
}
impl ::windows::core::TypeKind for COR_PRF_FUNCTION_ARGUMENT_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for COR_PRF_FUNCTION_ARGUMENT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.numRanges == other.numRanges && self.totalArgumentSize == other.totalArgumentSize && self.ranges == other.ranges
    }
}
impl ::core::cmp::Eq for COR_PRF_FUNCTION_ARGUMENT_INFO {}
impl ::core::default::Default for COR_PRF_FUNCTION_ARGUMENT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub struct COR_PRF_FUNCTION_ARGUMENT_RANGE {
    pub startAddress: usize,
    pub length: u32,
}
impl ::core::marker::Copy for COR_PRF_FUNCTION_ARGUMENT_RANGE {}
impl ::core::clone::Clone for COR_PRF_FUNCTION_ARGUMENT_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COR_PRF_FUNCTION_ARGUMENT_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COR_PRF_FUNCTION_ARGUMENT_RANGE").field("startAddress", &self.startAddress).field("length", &self.length).finish()
    }
}
impl ::windows::core::TypeKind for COR_PRF_FUNCTION_ARGUMENT_RANGE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for COR_PRF_FUNCTION_ARGUMENT_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.startAddress == other.startAddress && self.length == other.length
    }
}
impl ::core::cmp::Eq for COR_PRF_FUNCTION_ARGUMENT_RANGE {}
impl ::core::default::Default for COR_PRF_FUNCTION_ARGUMENT_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub struct COR_PRF_GC_GENERATION_RANGE {
    pub generation: COR_PRF_GC_GENERATION,
    pub rangeStart: usize,
    pub rangeLength: usize,
    pub rangeLengthReserved: usize,
}
impl ::core::marker::Copy for COR_PRF_GC_GENERATION_RANGE {}
impl ::core::clone::Clone for COR_PRF_GC_GENERATION_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COR_PRF_GC_GENERATION_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COR_PRF_GC_GENERATION_RANGE").field("generation", &self.generation).field("rangeStart", &self.rangeStart).field("rangeLength", &self.rangeLength).field("rangeLengthReserved", &self.rangeLengthReserved).finish()
    }
}
impl ::windows::core::TypeKind for COR_PRF_GC_GENERATION_RANGE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for COR_PRF_GC_GENERATION_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.generation == other.generation && self.rangeStart == other.rangeStart && self.rangeLength == other.rangeLength && self.rangeLengthReserved == other.rangeLengthReserved
    }
}
impl ::core::cmp::Eq for COR_PRF_GC_GENERATION_RANGE {}
impl ::core::default::Default for COR_PRF_GC_GENERATION_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub struct COR_PRF_METHOD {
    pub moduleId: usize,
    pub methodId: u32,
}
impl ::core::marker::Copy for COR_PRF_METHOD {}
impl ::core::clone::Clone for COR_PRF_METHOD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COR_PRF_METHOD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COR_PRF_METHOD").field("moduleId", &self.moduleId).field("methodId", &self.methodId).finish()
    }
}
impl ::windows::core::TypeKind for COR_PRF_METHOD {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for COR_PRF_METHOD {
    fn eq(&self, other: &Self) -> bool {
        self.moduleId == other.moduleId && self.methodId == other.methodId
    }
}
impl ::core::cmp::Eq for COR_PRF_METHOD {}
impl ::core::default::Default for COR_PRF_METHOD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub union FunctionIDOrClientID {
    pub functionID: usize,
    pub clientID: usize,
}
impl ::core::marker::Copy for FunctionIDOrClientID {}
impl ::core::clone::Clone for FunctionIDOrClientID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for FunctionIDOrClientID {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for FunctionIDOrClientID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub type FunctionEnter = ::core::option::Option<unsafe extern "system" fn(funcid: usize) -> ()>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub type FunctionEnter2 = ::core::option::Option<unsafe extern "system" fn(funcid: usize, clientdata: usize, func: usize, argumentinfo: *mut COR_PRF_FUNCTION_ARGUMENT_INFO) -> ()>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub type FunctionEnter3 = ::core::option::Option<unsafe extern "system" fn(functionidorclientid: FunctionIDOrClientID) -> ()>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub type FunctionEnter3WithInfo = ::core::option::Option<unsafe extern "system" fn(functionidorclientid: FunctionIDOrClientID, eltinfo: usize) -> ()>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type FunctionIDMapper = ::core::option::Option<unsafe extern "system" fn(funcid: usize, pbhookfunction: *mut super::super::super::Foundation::BOOL) -> usize>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type FunctionIDMapper2 = ::core::option::Option<unsafe extern "system" fn(funcid: usize, clientdata: *mut ::core::ffi::c_void, pbhookfunction: *mut super::super::super::Foundation::BOOL) -> usize>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub type FunctionLeave = ::core::option::Option<unsafe extern "system" fn(funcid: usize) -> ()>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub type FunctionLeave2 = ::core::option::Option<unsafe extern "system" fn(funcid: usize, clientdata: usize, func: usize, retvalrange: *mut COR_PRF_FUNCTION_ARGUMENT_RANGE) -> ()>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub type FunctionLeave3 = ::core::option::Option<unsafe extern "system" fn(functionidorclientid: FunctionIDOrClientID) -> ()>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub type FunctionLeave3WithInfo = ::core::option::Option<unsafe extern "system" fn(functionidorclientid: FunctionIDOrClientID, eltinfo: usize) -> ()>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub type FunctionTailcall = ::core::option::Option<unsafe extern "system" fn(funcid: usize) -> ()>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub type FunctionTailcall2 = ::core::option::Option<unsafe extern "system" fn(funcid: usize, clientdata: usize, func: usize) -> ()>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub type FunctionTailcall3 = ::core::option::Option<unsafe extern "system" fn(functionidorclientid: FunctionIDOrClientID) -> ()>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub type FunctionTailcall3WithInfo = ::core::option::Option<unsafe extern "system" fn(functionidorclientid: FunctionIDOrClientID, eltinfo: usize) -> ()>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ClrProfiling\"`*"]
pub type StackSnapshotCallback = ::core::option::Option<unsafe extern "system" fn(funcid: usize, ip: usize, frameinfo: usize, contextsize: u32, context: *mut u8, clientdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
