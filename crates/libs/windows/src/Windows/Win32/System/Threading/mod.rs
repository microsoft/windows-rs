#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn AcquireSRWLockExclusive(srwlock: *mut RTL_SRWLOCK) {
    ::windows::imp::link ! ( "kernel32.dll""system" fn AcquireSRWLockExclusive ( srwlock : *mut RTL_SRWLOCK ) -> ( ) );
    AcquireSRWLockExclusive(srwlock)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn AcquireSRWLockShared(srwlock: *mut RTL_SRWLOCK) {
    ::windows::imp::link ! ( "kernel32.dll""system" fn AcquireSRWLockShared ( srwlock : *mut RTL_SRWLOCK ) -> ( ) );
    AcquireSRWLockShared(srwlock)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddIntegrityLabelToBoundaryDescriptor<P0>(boundarydescriptor: *mut super::super::Foundation::HANDLE, integritylabel: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::PSID>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn AddIntegrityLabelToBoundaryDescriptor ( boundarydescriptor : *mut super::super::Foundation:: HANDLE , integritylabel : super::super::Foundation:: PSID ) -> super::super::Foundation:: BOOL );
    AddIntegrityLabelToBoundaryDescriptor(boundarydescriptor, integritylabel.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddSIDToBoundaryDescriptor<P0>(boundarydescriptor: *mut super::super::Foundation::HANDLE, requiredsid: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::PSID>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn AddSIDToBoundaryDescriptor ( boundarydescriptor : *mut super::super::Foundation:: HANDLE , requiredsid : super::super::Foundation:: PSID ) -> super::super::Foundation:: BOOL );
    AddSIDToBoundaryDescriptor(boundarydescriptor, requiredsid.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AttachThreadInput<P0>(idattach: u32, idattachto: u32, fattach: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "user32.dll""system" fn AttachThreadInput ( idattach : u32 , idattachto : u32 , fattach : super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
    AttachThreadInput(idattach, idattachto, fattach.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AvQuerySystemResponsiveness<P0>(avrthandle: P0, systemresponsivenessvalue: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "avrt.dll""system" fn AvQuerySystemResponsiveness ( avrthandle : super::super::Foundation:: HANDLE , systemresponsivenessvalue : *mut u32 ) -> super::super::Foundation:: BOOL );
    AvQuerySystemResponsiveness(avrthandle.into_param().abi(), systemresponsivenessvalue)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AvRevertMmThreadCharacteristics<P0>(avrthandle: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "avrt.dll""system" fn AvRevertMmThreadCharacteristics ( avrthandle : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    AvRevertMmThreadCharacteristics(avrthandle.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AvRtCreateThreadOrderingGroup(context: *mut super::super::Foundation::HANDLE, period: *const i64, threadorderingguid: *mut ::windows::core::GUID, timeout: ::core::option::Option<*const i64>) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "avrt.dll""system" fn AvRtCreateThreadOrderingGroup ( context : *mut super::super::Foundation:: HANDLE , period : *const i64 , threadorderingguid : *mut :: windows::core::GUID , timeout : *const i64 ) -> super::super::Foundation:: BOOL );
    AvRtCreateThreadOrderingGroup(context, period, threadorderingguid, ::core::mem::transmute(timeout.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AvRtCreateThreadOrderingGroupExA<P0>(context: *mut super::super::Foundation::HANDLE, period: *const i64, threadorderingguid: *mut ::windows::core::GUID, timeout: ::core::option::Option<*const i64>, taskname: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "avrt.dll""system" fn AvRtCreateThreadOrderingGroupExA ( context : *mut super::super::Foundation:: HANDLE , period : *const i64 , threadorderingguid : *mut :: windows::core::GUID , timeout : *const i64 , taskname : :: windows::core::PCSTR ) -> super::super::Foundation:: BOOL );
    AvRtCreateThreadOrderingGroupExA(context, period, threadorderingguid, ::core::mem::transmute(timeout.unwrap_or(::std::ptr::null())), taskname.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AvRtCreateThreadOrderingGroupExW<P0>(context: *mut super::super::Foundation::HANDLE, period: *const i64, threadorderingguid: *mut ::windows::core::GUID, timeout: ::core::option::Option<*const i64>, taskname: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "avrt.dll""system" fn AvRtCreateThreadOrderingGroupExW ( context : *mut super::super::Foundation:: HANDLE , period : *const i64 , threadorderingguid : *mut :: windows::core::GUID , timeout : *const i64 , taskname : :: windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    AvRtCreateThreadOrderingGroupExW(context, period, threadorderingguid, ::core::mem::transmute(timeout.unwrap_or(::std::ptr::null())), taskname.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AvRtDeleteThreadOrderingGroup<P0>(context: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "avrt.dll""system" fn AvRtDeleteThreadOrderingGroup ( context : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    AvRtDeleteThreadOrderingGroup(context.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AvRtJoinThreadOrderingGroup<P0>(context: *mut super::super::Foundation::HANDLE, threadorderingguid: *const ::windows::core::GUID, before: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "avrt.dll""system" fn AvRtJoinThreadOrderingGroup ( context : *mut super::super::Foundation:: HANDLE , threadorderingguid : *const :: windows::core::GUID , before : super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
    AvRtJoinThreadOrderingGroup(context, threadorderingguid, before.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AvRtLeaveThreadOrderingGroup<P0>(context: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "avrt.dll""system" fn AvRtLeaveThreadOrderingGroup ( context : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    AvRtLeaveThreadOrderingGroup(context.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AvRtWaitOnThreadOrderingGroup<P0>(context: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "avrt.dll""system" fn AvRtWaitOnThreadOrderingGroup ( context : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    AvRtWaitOnThreadOrderingGroup(context.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AvSetMmMaxThreadCharacteristicsA<P0, P1>(firsttask: P0, secondtask: P1, taskindex: *mut u32) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "avrt.dll""system" fn AvSetMmMaxThreadCharacteristicsA ( firsttask : :: windows::core::PCSTR , secondtask : :: windows::core::PCSTR , taskindex : *mut u32 ) -> super::super::Foundation:: HANDLE );
    let result__ = AvSetMmMaxThreadCharacteristicsA(firsttask.into_param().abi(), secondtask.into_param().abi(), taskindex);
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AvSetMmMaxThreadCharacteristicsW<P0, P1>(firsttask: P0, secondtask: P1, taskindex: *mut u32) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "avrt.dll""system" fn AvSetMmMaxThreadCharacteristicsW ( firsttask : :: windows::core::PCWSTR , secondtask : :: windows::core::PCWSTR , taskindex : *mut u32 ) -> super::super::Foundation:: HANDLE );
    let result__ = AvSetMmMaxThreadCharacteristicsW(firsttask.into_param().abi(), secondtask.into_param().abi(), taskindex);
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AvSetMmThreadCharacteristicsA<P0>(taskname: P0, taskindex: *mut u32) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "avrt.dll""system" fn AvSetMmThreadCharacteristicsA ( taskname : :: windows::core::PCSTR , taskindex : *mut u32 ) -> super::super::Foundation:: HANDLE );
    let result__ = AvSetMmThreadCharacteristicsA(taskname.into_param().abi(), taskindex);
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AvSetMmThreadCharacteristicsW<P0>(taskname: P0, taskindex: *mut u32) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "avrt.dll""system" fn AvSetMmThreadCharacteristicsW ( taskname : :: windows::core::PCWSTR , taskindex : *mut u32 ) -> super::super::Foundation:: HANDLE );
    let result__ = AvSetMmThreadCharacteristicsW(taskname.into_param().abi(), taskindex);
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AvSetMmThreadPriority<P0>(avrthandle: P0, priority: AVRT_PRIORITY) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "avrt.dll""system" fn AvSetMmThreadPriority ( avrthandle : super::super::Foundation:: HANDLE , priority : AVRT_PRIORITY ) -> super::super::Foundation:: BOOL );
    AvSetMmThreadPriority(avrthandle.into_param().abi(), priority)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CallbackMayRunLong<P0>(pci: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<PTP_CALLBACK_INSTANCE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn CallbackMayRunLong ( pci : PTP_CALLBACK_INSTANCE ) -> super::super::Foundation:: BOOL );
    CallbackMayRunLong(pci.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn CancelThreadpoolIo<P0>(pio: P0)
where
    P0: ::windows::core::IntoParam<PTP_IO>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn CancelThreadpoolIo ( pio : PTP_IO ) -> ( ) );
    CancelThreadpoolIo(pio.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CancelWaitableTimer<P0>(htimer: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn CancelWaitableTimer ( htimer : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    CancelWaitableTimer(htimer.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ChangeTimerQueueTimer<P0, P1>(timerqueue: P0, timer: P1, duetime: u32, period: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn ChangeTimerQueueTimer ( timerqueue : super::super::Foundation:: HANDLE , timer : super::super::Foundation:: HANDLE , duetime : u32 , period : u32 ) -> super::super::Foundation:: BOOL );
    ChangeTimerQueueTimer(timerqueue.into_param().abi(), timer.into_param().abi(), duetime, period)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ClosePrivateNamespace<P0>(handle: P0, flags: u32) -> super::super::Foundation::BOOLEAN
where
    P0: ::windows::core::IntoParam<NamespaceHandle>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn ClosePrivateNamespace ( handle : NamespaceHandle , flags : u32 ) -> super::super::Foundation:: BOOLEAN );
    ClosePrivateNamespace(handle.into_param().abi(), flags)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn CloseThreadpool<P0>(ptpp: P0)
where
    P0: ::windows::core::IntoParam<PTP_POOL>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn CloseThreadpool ( ptpp : PTP_POOL ) -> ( ) );
    CloseThreadpool(ptpp.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn CloseThreadpoolCleanupGroup(ptpcg: isize) {
    ::windows::imp::link ! ( "kernel32.dll""system" fn CloseThreadpoolCleanupGroup ( ptpcg : isize ) -> ( ) );
    CloseThreadpoolCleanupGroup(ptpcg)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CloseThreadpoolCleanupGroupMembers<P0>(ptpcg: isize, fcancelpendingcallbacks: P0, pvcleanupcontext: ::core::option::Option<*mut ::core::ffi::c_void>)
where
    P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn CloseThreadpoolCleanupGroupMembers ( ptpcg : isize , fcancelpendingcallbacks : super::super::Foundation:: BOOL , pvcleanupcontext : *mut ::core::ffi::c_void ) -> ( ) );
    CloseThreadpoolCleanupGroupMembers(ptpcg, fcancelpendingcallbacks.into_param().abi(), ::core::mem::transmute(pvcleanupcontext.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn CloseThreadpoolIo<P0>(pio: P0)
where
    P0: ::windows::core::IntoParam<PTP_IO>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn CloseThreadpoolIo ( pio : PTP_IO ) -> ( ) );
    CloseThreadpoolIo(pio.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn CloseThreadpoolTimer<P0>(pti: P0)
where
    P0: ::windows::core::IntoParam<PTP_TIMER>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn CloseThreadpoolTimer ( pti : PTP_TIMER ) -> ( ) );
    CloseThreadpoolTimer(pti.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn CloseThreadpoolWait<P0>(pwa: P0)
where
    P0: ::windows::core::IntoParam<PTP_WAIT>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn CloseThreadpoolWait ( pwa : PTP_WAIT ) -> ( ) );
    CloseThreadpoolWait(pwa.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn CloseThreadpoolWork<P0>(pwk: P0)
where
    P0: ::windows::core::IntoParam<PTP_WORK>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn CloseThreadpoolWork ( pwk : PTP_WORK ) -> ( ) );
    CloseThreadpoolWork(pwk.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ConvertFiberToThread() -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn ConvertFiberToThread ( ) -> super::super::Foundation:: BOOL );
    ConvertFiberToThread()
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn ConvertThreadToFiber(lpparameter: ::core::option::Option<*const ::core::ffi::c_void>) -> *mut ::core::ffi::c_void {
    ::windows::imp::link ! ( "kernel32.dll""system" fn ConvertThreadToFiber ( lpparameter : *const ::core::ffi::c_void ) -> *mut ::core::ffi::c_void );
    ConvertThreadToFiber(::core::mem::transmute(lpparameter.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn ConvertThreadToFiberEx(lpparameter: ::core::option::Option<*const ::core::ffi::c_void>, dwflags: u32) -> *mut ::core::ffi::c_void {
    ::windows::imp::link ! ( "kernel32.dll""system" fn ConvertThreadToFiberEx ( lpparameter : *const ::core::ffi::c_void , dwflags : u32 ) -> *mut ::core::ffi::c_void );
    ConvertThreadToFiberEx(::core::mem::transmute(lpparameter.unwrap_or(::std::ptr::null())), dwflags)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn CreateBoundaryDescriptorA<P0>(name: P0, flags: u32) -> ::windows::core::Result<BoundaryDescriptorHandle>
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn CreateBoundaryDescriptorA ( name : :: windows::core::PCSTR , flags : u32 ) -> BoundaryDescriptorHandle );
    let result__ = CreateBoundaryDescriptorA(name.into_param().abi(), flags);
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn CreateBoundaryDescriptorW<P0>(name: P0, flags: u32) -> BoundaryDescriptorHandle
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn CreateBoundaryDescriptorW ( name : :: windows::core::PCWSTR , flags : u32 ) -> BoundaryDescriptorHandle );
    CreateBoundaryDescriptorW(name.into_param().abi(), flags)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateEventA<P0, P1, P2>(lpeventattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>, bmanualreset: P0, binitialstate: P1, lpname: P2) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn CreateEventA ( lpeventattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , bmanualreset : super::super::Foundation:: BOOL , binitialstate : super::super::Foundation:: BOOL , lpname : :: windows::core::PCSTR ) -> super::super::Foundation:: HANDLE );
    let result__ = CreateEventA(::core::mem::transmute(lpeventattributes.unwrap_or(::std::ptr::null())), bmanualreset.into_param().abi(), binitialstate.into_param().abi(), lpname.into_param().abi());
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateEventExA<P0>(lpeventattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>, lpname: P0, dwflags: CREATE_EVENT, dwdesiredaccess: u32) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn CreateEventExA ( lpeventattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , lpname : :: windows::core::PCSTR , dwflags : CREATE_EVENT , dwdesiredaccess : u32 ) -> super::super::Foundation:: HANDLE );
    let result__ = CreateEventExA(::core::mem::transmute(lpeventattributes.unwrap_or(::std::ptr::null())), lpname.into_param().abi(), dwflags, dwdesiredaccess);
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateEventExW<P0>(lpeventattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>, lpname: P0, dwflags: CREATE_EVENT, dwdesiredaccess: u32) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn CreateEventExW ( lpeventattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , lpname : :: windows::core::PCWSTR , dwflags : CREATE_EVENT , dwdesiredaccess : u32 ) -> super::super::Foundation:: HANDLE );
    let result__ = CreateEventExW(::core::mem::transmute(lpeventattributes.unwrap_or(::std::ptr::null())), lpname.into_param().abi(), dwflags, dwdesiredaccess);
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateEventW<P0, P1, P2>(lpeventattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>, bmanualreset: P0, binitialstate: P1, lpname: P2) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn CreateEventW ( lpeventattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , bmanualreset : super::super::Foundation:: BOOL , binitialstate : super::super::Foundation:: BOOL , lpname : :: windows::core::PCWSTR ) -> super::super::Foundation:: HANDLE );
    let result__ = CreateEventW(::core::mem::transmute(lpeventattributes.unwrap_or(::std::ptr::null())), bmanualreset.into_param().abi(), binitialstate.into_param().abi(), lpname.into_param().abi());
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn CreateFiber(dwstacksize: usize, lpstartaddress: LPFIBER_START_ROUTINE, lpparameter: ::core::option::Option<*const ::core::ffi::c_void>) -> *mut ::core::ffi::c_void {
    ::windows::imp::link ! ( "kernel32.dll""system" fn CreateFiber ( dwstacksize : usize , lpstartaddress : LPFIBER_START_ROUTINE , lpparameter : *const ::core::ffi::c_void ) -> *mut ::core::ffi::c_void );
    CreateFiber(dwstacksize, lpstartaddress, ::core::mem::transmute(lpparameter.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn CreateFiberEx(dwstackcommitsize: usize, dwstackreservesize: usize, dwflags: u32, lpstartaddress: LPFIBER_START_ROUTINE, lpparameter: ::core::option::Option<*const ::core::ffi::c_void>) -> *mut ::core::ffi::c_void {
    ::windows::imp::link ! ( "kernel32.dll""system" fn CreateFiberEx ( dwstackcommitsize : usize , dwstackreservesize : usize , dwflags : u32 , lpstartaddress : LPFIBER_START_ROUTINE , lpparameter : *const ::core::ffi::c_void ) -> *mut ::core::ffi::c_void );
    CreateFiberEx(dwstackcommitsize, dwstackreservesize, dwflags, lpstartaddress, ::core::mem::transmute(lpparameter.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateMutexA<P0, P1>(lpmutexattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>, binitialowner: P0, lpname: P1) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn CreateMutexA ( lpmutexattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , binitialowner : super::super::Foundation:: BOOL , lpname : :: windows::core::PCSTR ) -> super::super::Foundation:: HANDLE );
    let result__ = CreateMutexA(::core::mem::transmute(lpmutexattributes.unwrap_or(::std::ptr::null())), binitialowner.into_param().abi(), lpname.into_param().abi());
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateMutexExA<P0>(lpmutexattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>, lpname: P0, dwflags: u32, dwdesiredaccess: u32) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn CreateMutexExA ( lpmutexattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , lpname : :: windows::core::PCSTR , dwflags : u32 , dwdesiredaccess : u32 ) -> super::super::Foundation:: HANDLE );
    let result__ = CreateMutexExA(::core::mem::transmute(lpmutexattributes.unwrap_or(::std::ptr::null())), lpname.into_param().abi(), dwflags, dwdesiredaccess);
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateMutexExW<P0>(lpmutexattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>, lpname: P0, dwflags: u32, dwdesiredaccess: u32) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn CreateMutexExW ( lpmutexattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , lpname : :: windows::core::PCWSTR , dwflags : u32 , dwdesiredaccess : u32 ) -> super::super::Foundation:: HANDLE );
    let result__ = CreateMutexExW(::core::mem::transmute(lpmutexattributes.unwrap_or(::std::ptr::null())), lpname.into_param().abi(), dwflags, dwdesiredaccess);
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateMutexW<P0, P1>(lpmutexattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>, binitialowner: P0, lpname: P1) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn CreateMutexW ( lpmutexattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , binitialowner : super::super::Foundation:: BOOL , lpname : :: windows::core::PCWSTR ) -> super::super::Foundation:: HANDLE );
    let result__ = CreateMutexW(::core::mem::transmute(lpmutexattributes.unwrap_or(::std::ptr::null())), binitialowner.into_param().abi(), lpname.into_param().abi());
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreatePrivateNamespaceA<P0>(lpprivatenamespaceattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>, lpboundarydescriptor: *const ::core::ffi::c_void, lpaliasprefix: P0) -> ::windows::core::Result<NamespaceHandle>
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn CreatePrivateNamespaceA ( lpprivatenamespaceattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , lpboundarydescriptor : *const ::core::ffi::c_void , lpaliasprefix : :: windows::core::PCSTR ) -> NamespaceHandle );
    let result__ = CreatePrivateNamespaceA(::core::mem::transmute(lpprivatenamespaceattributes.unwrap_or(::std::ptr::null())), lpboundarydescriptor, lpaliasprefix.into_param().abi());
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreatePrivateNamespaceW<P0>(lpprivatenamespaceattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>, lpboundarydescriptor: *const ::core::ffi::c_void, lpaliasprefix: P0) -> NamespaceHandle
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn CreatePrivateNamespaceW ( lpprivatenamespaceattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , lpboundarydescriptor : *const ::core::ffi::c_void , lpaliasprefix : :: windows::core::PCWSTR ) -> NamespaceHandle );
    CreatePrivateNamespaceW(::core::mem::transmute(lpprivatenamespaceattributes.unwrap_or(::std::ptr::null())), lpboundarydescriptor, lpaliasprefix.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateProcessA<P0, P1, P2>(lpapplicationname: P0, lpcommandline: ::windows::core::PSTR, lpprocessattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>, lpthreadattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>, binherithandles: P1, dwcreationflags: PROCESS_CREATION_FLAGS, lpenvironment: ::core::option::Option<*const ::core::ffi::c_void>, lpcurrentdirectory: P2, lpstartupinfo: *const STARTUPINFOA, lpprocessinformation: *mut PROCESS_INFORMATION) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn CreateProcessA ( lpapplicationname : :: windows::core::PCSTR , lpcommandline : :: windows::core::PSTR , lpprocessattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , lpthreadattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , binherithandles : super::super::Foundation:: BOOL , dwcreationflags : PROCESS_CREATION_FLAGS , lpenvironment : *const ::core::ffi::c_void , lpcurrentdirectory : :: windows::core::PCSTR , lpstartupinfo : *const STARTUPINFOA , lpprocessinformation : *mut PROCESS_INFORMATION ) -> super::super::Foundation:: BOOL );
    CreateProcessA(lpapplicationname.into_param().abi(), ::core::mem::transmute(lpcommandline), ::core::mem::transmute(lpprocessattributes.unwrap_or(::std::ptr::null())), ::core::mem::transmute(lpthreadattributes.unwrap_or(::std::ptr::null())), binherithandles.into_param().abi(), dwcreationflags, ::core::mem::transmute(lpenvironment.unwrap_or(::std::ptr::null())), lpcurrentdirectory.into_param().abi(), lpstartupinfo, lpprocessinformation)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateProcessAsUserA<P0, P1, P2, P3>(htoken: P0, lpapplicationname: P1, lpcommandline: ::windows::core::PSTR, lpprocessattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>, lpthreadattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>, binherithandles: P2, dwcreationflags: PROCESS_CREATION_FLAGS, lpenvironment: ::core::option::Option<*const ::core::ffi::c_void>, lpcurrentdirectory: P3, lpstartupinfo: *const STARTUPINFOA, lpprocessinformation: *mut PROCESS_INFORMATION) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    P3: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn CreateProcessAsUserA ( htoken : super::super::Foundation:: HANDLE , lpapplicationname : :: windows::core::PCSTR , lpcommandline : :: windows::core::PSTR , lpprocessattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , lpthreadattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , binherithandles : super::super::Foundation:: BOOL , dwcreationflags : PROCESS_CREATION_FLAGS , lpenvironment : *const ::core::ffi::c_void , lpcurrentdirectory : :: windows::core::PCSTR , lpstartupinfo : *const STARTUPINFOA , lpprocessinformation : *mut PROCESS_INFORMATION ) -> super::super::Foundation:: BOOL );
    CreateProcessAsUserA(htoken.into_param().abi(), lpapplicationname.into_param().abi(), ::core::mem::transmute(lpcommandline), ::core::mem::transmute(lpprocessattributes.unwrap_or(::std::ptr::null())), ::core::mem::transmute(lpthreadattributes.unwrap_or(::std::ptr::null())), binherithandles.into_param().abi(), dwcreationflags, ::core::mem::transmute(lpenvironment.unwrap_or(::std::ptr::null())), lpcurrentdirectory.into_param().abi(), lpstartupinfo, lpprocessinformation)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateProcessAsUserW<P0, P1, P2, P3>(htoken: P0, lpapplicationname: P1, lpcommandline: ::windows::core::PWSTR, lpprocessattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>, lpthreadattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>, binherithandles: P2, dwcreationflags: PROCESS_CREATION_FLAGS, lpenvironment: ::core::option::Option<*const ::core::ffi::c_void>, lpcurrentdirectory: P3, lpstartupinfo: *const STARTUPINFOW, lpprocessinformation: *mut PROCESS_INFORMATION) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn CreateProcessAsUserW ( htoken : super::super::Foundation:: HANDLE , lpapplicationname : :: windows::core::PCWSTR , lpcommandline : :: windows::core::PWSTR , lpprocessattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , lpthreadattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , binherithandles : super::super::Foundation:: BOOL , dwcreationflags : PROCESS_CREATION_FLAGS , lpenvironment : *const ::core::ffi::c_void , lpcurrentdirectory : :: windows::core::PCWSTR , lpstartupinfo : *const STARTUPINFOW , lpprocessinformation : *mut PROCESS_INFORMATION ) -> super::super::Foundation:: BOOL );
    CreateProcessAsUserW(htoken.into_param().abi(), lpapplicationname.into_param().abi(), ::core::mem::transmute(lpcommandline), ::core::mem::transmute(lpprocessattributes.unwrap_or(::std::ptr::null())), ::core::mem::transmute(lpthreadattributes.unwrap_or(::std::ptr::null())), binherithandles.into_param().abi(), dwcreationflags, ::core::mem::transmute(lpenvironment.unwrap_or(::std::ptr::null())), lpcurrentdirectory.into_param().abi(), lpstartupinfo, lpprocessinformation)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateProcessW<P0, P1, P2>(lpapplicationname: P0, lpcommandline: ::windows::core::PWSTR, lpprocessattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>, lpthreadattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>, binherithandles: P1, dwcreationflags: PROCESS_CREATION_FLAGS, lpenvironment: ::core::option::Option<*const ::core::ffi::c_void>, lpcurrentdirectory: P2, lpstartupinfo: *const STARTUPINFOW, lpprocessinformation: *mut PROCESS_INFORMATION) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn CreateProcessW ( lpapplicationname : :: windows::core::PCWSTR , lpcommandline : :: windows::core::PWSTR , lpprocessattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , lpthreadattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , binherithandles : super::super::Foundation:: BOOL , dwcreationflags : PROCESS_CREATION_FLAGS , lpenvironment : *const ::core::ffi::c_void , lpcurrentdirectory : :: windows::core::PCWSTR , lpstartupinfo : *const STARTUPINFOW , lpprocessinformation : *mut PROCESS_INFORMATION ) -> super::super::Foundation:: BOOL );
    CreateProcessW(lpapplicationname.into_param().abi(), ::core::mem::transmute(lpcommandline), ::core::mem::transmute(lpprocessattributes.unwrap_or(::std::ptr::null())), ::core::mem::transmute(lpthreadattributes.unwrap_or(::std::ptr::null())), binherithandles.into_param().abi(), dwcreationflags, ::core::mem::transmute(lpenvironment.unwrap_or(::std::ptr::null())), lpcurrentdirectory.into_param().abi(), lpstartupinfo, lpprocessinformation)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateProcessWithLogonW<P0, P1, P2, P3, P4>(lpusername: P0, lpdomain: P1, lppassword: P2, dwlogonflags: CREATE_PROCESS_LOGON_FLAGS, lpapplicationname: P3, lpcommandline: ::windows::core::PWSTR, dwcreationflags: PROCESS_CREATION_FLAGS, lpenvironment: ::core::option::Option<*const ::core::ffi::c_void>, lpcurrentdirectory: P4, lpstartupinfo: *const STARTUPINFOW, lpprocessinformation: *mut PROCESS_INFORMATION) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P4: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn CreateProcessWithLogonW ( lpusername : :: windows::core::PCWSTR , lpdomain : :: windows::core::PCWSTR , lppassword : :: windows::core::PCWSTR , dwlogonflags : CREATE_PROCESS_LOGON_FLAGS , lpapplicationname : :: windows::core::PCWSTR , lpcommandline : :: windows::core::PWSTR , dwcreationflags : PROCESS_CREATION_FLAGS , lpenvironment : *const ::core::ffi::c_void , lpcurrentdirectory : :: windows::core::PCWSTR , lpstartupinfo : *const STARTUPINFOW , lpprocessinformation : *mut PROCESS_INFORMATION ) -> super::super::Foundation:: BOOL );
    CreateProcessWithLogonW(lpusername.into_param().abi(), lpdomain.into_param().abi(), lppassword.into_param().abi(), dwlogonflags, lpapplicationname.into_param().abi(), ::core::mem::transmute(lpcommandline), dwcreationflags, ::core::mem::transmute(lpenvironment.unwrap_or(::std::ptr::null())), lpcurrentdirectory.into_param().abi(), lpstartupinfo, lpprocessinformation)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateProcessWithTokenW<P0, P1, P2>(htoken: P0, dwlogonflags: CREATE_PROCESS_LOGON_FLAGS, lpapplicationname: P1, lpcommandline: ::windows::core::PWSTR, dwcreationflags: PROCESS_CREATION_FLAGS, lpenvironment: ::core::option::Option<*const ::core::ffi::c_void>, lpcurrentdirectory: P2, lpstartupinfo: *const STARTUPINFOW, lpprocessinformation: *mut PROCESS_INFORMATION) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn CreateProcessWithTokenW ( htoken : super::super::Foundation:: HANDLE , dwlogonflags : CREATE_PROCESS_LOGON_FLAGS , lpapplicationname : :: windows::core::PCWSTR , lpcommandline : :: windows::core::PWSTR , dwcreationflags : PROCESS_CREATION_FLAGS , lpenvironment : *const ::core::ffi::c_void , lpcurrentdirectory : :: windows::core::PCWSTR , lpstartupinfo : *const STARTUPINFOW , lpprocessinformation : *mut PROCESS_INFORMATION ) -> super::super::Foundation:: BOOL );
    CreateProcessWithTokenW(htoken.into_param().abi(), dwlogonflags, lpapplicationname.into_param().abi(), ::core::mem::transmute(lpcommandline), dwcreationflags, ::core::mem::transmute(lpenvironment.unwrap_or(::std::ptr::null())), lpcurrentdirectory.into_param().abi(), lpstartupinfo, lpprocessinformation)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateRemoteThread<P0>(hprocess: P0, lpthreadattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>, dwstacksize: usize, lpstartaddress: LPTHREAD_START_ROUTINE, lpparameter: ::core::option::Option<*const ::core::ffi::c_void>, dwcreationflags: u32, lpthreadid: ::core::option::Option<*mut u32>) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn CreateRemoteThread ( hprocess : super::super::Foundation:: HANDLE , lpthreadattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , dwstacksize : usize , lpstartaddress : LPTHREAD_START_ROUTINE , lpparameter : *const ::core::ffi::c_void , dwcreationflags : u32 , lpthreadid : *mut u32 ) -> super::super::Foundation:: HANDLE );
    let result__ = CreateRemoteThread(hprocess.into_param().abi(), ::core::mem::transmute(lpthreadattributes.unwrap_or(::std::ptr::null())), dwstacksize, lpstartaddress, ::core::mem::transmute(lpparameter.unwrap_or(::std::ptr::null())), dwcreationflags, ::core::mem::transmute(lpthreadid.unwrap_or(::std::ptr::null_mut())));
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateRemoteThreadEx<P0, P1>(hprocess: P0, lpthreadattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>, dwstacksize: usize, lpstartaddress: LPTHREAD_START_ROUTINE, lpparameter: ::core::option::Option<*const ::core::ffi::c_void>, dwcreationflags: u32, lpattributelist: P1, lpthreadid: ::core::option::Option<*mut u32>) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<LPPROC_THREAD_ATTRIBUTE_LIST>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn CreateRemoteThreadEx ( hprocess : super::super::Foundation:: HANDLE , lpthreadattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , dwstacksize : usize , lpstartaddress : LPTHREAD_START_ROUTINE , lpparameter : *const ::core::ffi::c_void , dwcreationflags : u32 , lpattributelist : LPPROC_THREAD_ATTRIBUTE_LIST , lpthreadid : *mut u32 ) -> super::super::Foundation:: HANDLE );
    let result__ = CreateRemoteThreadEx(hprocess.into_param().abi(), ::core::mem::transmute(lpthreadattributes.unwrap_or(::std::ptr::null())), dwstacksize, lpstartaddress, ::core::mem::transmute(lpparameter.unwrap_or(::std::ptr::null())), dwcreationflags, lpattributelist.into_param().abi(), ::core::mem::transmute(lpthreadid.unwrap_or(::std::ptr::null_mut())));
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateSemaphoreA<P0>(lpsemaphoreattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>, linitialcount: i32, lmaximumcount: i32, lpname: P0) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn CreateSemaphoreA ( lpsemaphoreattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , linitialcount : i32 , lmaximumcount : i32 , lpname : :: windows::core::PCSTR ) -> super::super::Foundation:: HANDLE );
    let result__ = CreateSemaphoreA(::core::mem::transmute(lpsemaphoreattributes.unwrap_or(::std::ptr::null())), linitialcount, lmaximumcount, lpname.into_param().abi());
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateSemaphoreExA<P0>(lpsemaphoreattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>, linitialcount: i32, lmaximumcount: i32, lpname: P0, dwflags: u32, dwdesiredaccess: u32) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn CreateSemaphoreExA ( lpsemaphoreattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , linitialcount : i32 , lmaximumcount : i32 , lpname : :: windows::core::PCSTR , dwflags : u32 , dwdesiredaccess : u32 ) -> super::super::Foundation:: HANDLE );
    let result__ = CreateSemaphoreExA(::core::mem::transmute(lpsemaphoreattributes.unwrap_or(::std::ptr::null())), linitialcount, lmaximumcount, lpname.into_param().abi(), dwflags, dwdesiredaccess);
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateSemaphoreExW<P0>(lpsemaphoreattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>, linitialcount: i32, lmaximumcount: i32, lpname: P0, dwflags: u32, dwdesiredaccess: u32) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn CreateSemaphoreExW ( lpsemaphoreattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , linitialcount : i32 , lmaximumcount : i32 , lpname : :: windows::core::PCWSTR , dwflags : u32 , dwdesiredaccess : u32 ) -> super::super::Foundation:: HANDLE );
    let result__ = CreateSemaphoreExW(::core::mem::transmute(lpsemaphoreattributes.unwrap_or(::std::ptr::null())), linitialcount, lmaximumcount, lpname.into_param().abi(), dwflags, dwdesiredaccess);
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateSemaphoreW<P0>(lpsemaphoreattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>, linitialcount: i32, lmaximumcount: i32, lpname: P0) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn CreateSemaphoreW ( lpsemaphoreattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , linitialcount : i32 , lmaximumcount : i32 , lpname : :: windows::core::PCWSTR ) -> super::super::Foundation:: HANDLE );
    let result__ = CreateSemaphoreW(::core::mem::transmute(lpsemaphoreattributes.unwrap_or(::std::ptr::null())), linitialcount, lmaximumcount, lpname.into_param().abi());
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateThread(lpthreadattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>, dwstacksize: usize, lpstartaddress: LPTHREAD_START_ROUTINE, lpparameter: ::core::option::Option<*const ::core::ffi::c_void>, dwcreationflags: THREAD_CREATION_FLAGS, lpthreadid: ::core::option::Option<*mut u32>) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
    ::windows::imp::link ! ( "kernel32.dll""system" fn CreateThread ( lpthreadattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , dwstacksize : usize , lpstartaddress : LPTHREAD_START_ROUTINE , lpparameter : *const ::core::ffi::c_void , dwcreationflags : THREAD_CREATION_FLAGS , lpthreadid : *mut u32 ) -> super::super::Foundation:: HANDLE );
    let result__ = CreateThread(::core::mem::transmute(lpthreadattributes.unwrap_or(::std::ptr::null())), dwstacksize, lpstartaddress, ::core::mem::transmute(lpparameter.unwrap_or(::std::ptr::null())), dwcreationflags, ::core::mem::transmute(lpthreadid.unwrap_or(::std::ptr::null_mut())));
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn CreateThreadpool(reserved: ::core::option::Option<*const ::core::ffi::c_void>) -> PTP_POOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn CreateThreadpool ( reserved : *const ::core::ffi::c_void ) -> PTP_POOL );
    CreateThreadpool(::core::mem::transmute(reserved.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn CreateThreadpoolCleanupGroup() -> isize {
    ::windows::imp::link ! ( "kernel32.dll""system" fn CreateThreadpoolCleanupGroup ( ) -> isize );
    CreateThreadpoolCleanupGroup()
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateThreadpoolIo<P0>(fl: P0, pfnio: PTP_WIN32_IO_CALLBACK, pv: ::core::option::Option<*mut ::core::ffi::c_void>, pcbe: ::core::option::Option<*const TP_CALLBACK_ENVIRON_V3>) -> ::windows::core::Result<PTP_IO>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn CreateThreadpoolIo ( fl : super::super::Foundation:: HANDLE , pfnio : PTP_WIN32_IO_CALLBACK , pv : *mut ::core::ffi::c_void , pcbe : *const TP_CALLBACK_ENVIRON_V3 ) -> PTP_IO );
    let result__ = CreateThreadpoolIo(fl.into_param().abi(), pfnio, ::core::mem::transmute(pv.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pcbe.unwrap_or(::std::ptr::null())));
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn CreateThreadpoolTimer(pfnti: PTP_TIMER_CALLBACK, pv: ::core::option::Option<*mut ::core::ffi::c_void>, pcbe: ::core::option::Option<*const TP_CALLBACK_ENVIRON_V3>) -> ::windows::core::Result<PTP_TIMER> {
    ::windows::imp::link ! ( "kernel32.dll""system" fn CreateThreadpoolTimer ( pfnti : PTP_TIMER_CALLBACK , pv : *mut ::core::ffi::c_void , pcbe : *const TP_CALLBACK_ENVIRON_V3 ) -> PTP_TIMER );
    let result__ = CreateThreadpoolTimer(pfnti, ::core::mem::transmute(pv.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pcbe.unwrap_or(::std::ptr::null())));
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn CreateThreadpoolWait(pfnwa: PTP_WAIT_CALLBACK, pv: ::core::option::Option<*mut ::core::ffi::c_void>, pcbe: ::core::option::Option<*const TP_CALLBACK_ENVIRON_V3>) -> ::windows::core::Result<PTP_WAIT> {
    ::windows::imp::link ! ( "kernel32.dll""system" fn CreateThreadpoolWait ( pfnwa : PTP_WAIT_CALLBACK , pv : *mut ::core::ffi::c_void , pcbe : *const TP_CALLBACK_ENVIRON_V3 ) -> PTP_WAIT );
    let result__ = CreateThreadpoolWait(pfnwa, ::core::mem::transmute(pv.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pcbe.unwrap_or(::std::ptr::null())));
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn CreateThreadpoolWork(pfnwk: PTP_WORK_CALLBACK, pv: ::core::option::Option<*mut ::core::ffi::c_void>, pcbe: ::core::option::Option<*const TP_CALLBACK_ENVIRON_V3>) -> ::windows::core::Result<PTP_WORK> {
    ::windows::imp::link ! ( "kernel32.dll""system" fn CreateThreadpoolWork ( pfnwk : PTP_WORK_CALLBACK , pv : *mut ::core::ffi::c_void , pcbe : *const TP_CALLBACK_ENVIRON_V3 ) -> PTP_WORK );
    let result__ = CreateThreadpoolWork(pfnwk, ::core::mem::transmute(pv.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pcbe.unwrap_or(::std::ptr::null())));
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateTimerQueue() -> ::windows::core::Result<super::super::Foundation::HANDLE> {
    ::windows::imp::link ! ( "kernel32.dll""system" fn CreateTimerQueue ( ) -> super::super::Foundation:: HANDLE );
    let result__ = CreateTimerQueue();
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateTimerQueueTimer<P0>(phnewtimer: *mut super::super::Foundation::HANDLE, timerqueue: P0, callback: WAITORTIMERCALLBACK, parameter: ::core::option::Option<*const ::core::ffi::c_void>, duetime: u32, period: u32, flags: WORKER_THREAD_FLAGS) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn CreateTimerQueueTimer ( phnewtimer : *mut super::super::Foundation:: HANDLE , timerqueue : super::super::Foundation:: HANDLE , callback : WAITORTIMERCALLBACK , parameter : *const ::core::ffi::c_void , duetime : u32 , period : u32 , flags : WORKER_THREAD_FLAGS ) -> super::super::Foundation:: BOOL );
    CreateTimerQueueTimer(phnewtimer, timerqueue.into_param().abi(), callback, ::core::mem::transmute(parameter.unwrap_or(::std::ptr::null())), duetime, period, flags)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateUmsCompletionList(umscompletionlist: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn CreateUmsCompletionList ( umscompletionlist : *mut *mut ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    CreateUmsCompletionList(umscompletionlist)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateUmsThreadContext(lpumsthread: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn CreateUmsThreadContext ( lpumsthread : *mut *mut ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    CreateUmsThreadContext(lpumsthread)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateWaitableTimerExW<P0>(lptimerattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>, lptimername: P0, dwflags: u32, dwdesiredaccess: u32) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn CreateWaitableTimerExW ( lptimerattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , lptimername : :: windows::core::PCWSTR , dwflags : u32 , dwdesiredaccess : u32 ) -> super::super::Foundation:: HANDLE );
    let result__ = CreateWaitableTimerExW(::core::mem::transmute(lptimerattributes.unwrap_or(::std::ptr::null())), lptimername.into_param().abi(), dwflags, dwdesiredaccess);
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateWaitableTimerW<P0, P1>(lptimerattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>, bmanualreset: P0, lptimername: P1) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn CreateWaitableTimerW ( lptimerattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , bmanualreset : super::super::Foundation:: BOOL , lptimername : :: windows::core::PCWSTR ) -> super::super::Foundation:: HANDLE );
    let result__ = CreateWaitableTimerW(::core::mem::transmute(lptimerattributes.unwrap_or(::std::ptr::null())), bmanualreset.into_param().abi(), lptimername.into_param().abi());
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn DeleteBoundaryDescriptor<P0>(boundarydescriptor: P0)
where
    P0: ::windows::core::IntoParam<BoundaryDescriptorHandle>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn DeleteBoundaryDescriptor ( boundarydescriptor : BoundaryDescriptorHandle ) -> ( ) );
    DeleteBoundaryDescriptor(boundarydescriptor.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn DeleteCriticalSection(lpcriticalsection: *mut RTL_CRITICAL_SECTION) {
    ::windows::imp::link ! ( "kernel32.dll""system" fn DeleteCriticalSection ( lpcriticalsection : *mut RTL_CRITICAL_SECTION ) -> ( ) );
    DeleteCriticalSection(lpcriticalsection)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn DeleteFiber(lpfiber: *const ::core::ffi::c_void) {
    ::windows::imp::link ! ( "kernel32.dll""system" fn DeleteFiber ( lpfiber : *const ::core::ffi::c_void ) -> ( ) );
    DeleteFiber(lpfiber)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn DeleteProcThreadAttributeList(lpattributelist: LPPROC_THREAD_ATTRIBUTE_LIST) {
    ::windows::imp::link ! ( "kernel32.dll""system" fn DeleteProcThreadAttributeList ( lpattributelist : LPPROC_THREAD_ATTRIBUTE_LIST ) -> ( ) );
    DeleteProcThreadAttributeList(lpattributelist)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeleteSynchronizationBarrier(lpbarrier: *mut RTL_BARRIER) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn DeleteSynchronizationBarrier ( lpbarrier : *mut RTL_BARRIER ) -> super::super::Foundation:: BOOL );
    DeleteSynchronizationBarrier(lpbarrier)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeleteTimerQueue<P0>(timerqueue: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn DeleteTimerQueue ( timerqueue : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    DeleteTimerQueue(timerqueue.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeleteTimerQueueEx<P0, P1>(timerqueue: P0, completionevent: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn DeleteTimerQueueEx ( timerqueue : super::super::Foundation:: HANDLE , completionevent : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    DeleteTimerQueueEx(timerqueue.into_param().abi(), completionevent.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeleteTimerQueueTimer<P0, P1, P2>(timerqueue: P0, timer: P1, completionevent: P2) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P2: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn DeleteTimerQueueTimer ( timerqueue : super::super::Foundation:: HANDLE , timer : super::super::Foundation:: HANDLE , completionevent : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    DeleteTimerQueueTimer(timerqueue.into_param().abi(), timer.into_param().abi(), completionevent.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeleteUmsCompletionList(umscompletionlist: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn DeleteUmsCompletionList ( umscompletionlist : *const ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    DeleteUmsCompletionList(umscompletionlist)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeleteUmsThreadContext(umsthread: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn DeleteUmsThreadContext ( umsthread : *const ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    DeleteUmsThreadContext(umsthread)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DequeueUmsCompletionListItems(umscompletionlist: *const ::core::ffi::c_void, waittimeout: u32, umsthreadlist: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn DequeueUmsCompletionListItems ( umscompletionlist : *const ::core::ffi::c_void , waittimeout : u32 , umsthreadlist : *mut *mut ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    DequeueUmsCompletionListItems(umscompletionlist, waittimeout, umsthreadlist)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn DisassociateCurrentThreadFromCallback<P0>(pci: P0)
where
    P0: ::windows::core::IntoParam<PTP_CALLBACK_INSTANCE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn DisassociateCurrentThreadFromCallback ( pci : PTP_CALLBACK_INSTANCE ) -> ( ) );
    DisassociateCurrentThreadFromCallback(pci.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn EnterCriticalSection(lpcriticalsection: *mut RTL_CRITICAL_SECTION) {
    ::windows::imp::link ! ( "kernel32.dll""system" fn EnterCriticalSection ( lpcriticalsection : *mut RTL_CRITICAL_SECTION ) -> ( ) );
    EnterCriticalSection(lpcriticalsection)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnterSynchronizationBarrier(lpbarrier: *mut RTL_BARRIER, dwflags: u32) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn EnterSynchronizationBarrier ( lpbarrier : *mut RTL_BARRIER , dwflags : u32 ) -> super::super::Foundation:: BOOL );
    EnterSynchronizationBarrier(lpbarrier, dwflags)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_SystemServices\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn EnterUmsSchedulingMode(schedulerstartupinfo: *const UMS_SCHEDULER_STARTUP_INFO) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn EnterUmsSchedulingMode ( schedulerstartupinfo : *const UMS_SCHEDULER_STARTUP_INFO ) -> super::super::Foundation:: BOOL );
    EnterUmsSchedulingMode(schedulerstartupinfo)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ExecuteUmsThread(umsthread: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn ExecuteUmsThread ( umsthread : *mut ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    ExecuteUmsThread(umsthread)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn ExitProcess(uexitcode: u32) -> ! {
    ::windows::imp::link ! ( "kernel32.dll""system" fn ExitProcess ( uexitcode : u32 ) -> ! );
    ExitProcess(uexitcode)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn ExitThread(dwexitcode: u32) -> ! {
    ::windows::imp::link ! ( "kernel32.dll""system" fn ExitThread ( dwexitcode : u32 ) -> ! );
    ExitThread(dwexitcode)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn FlsAlloc(lpcallback: PFLS_CALLBACK_FUNCTION) -> u32 {
    ::windows::imp::link ! ( "kernel32.dll""system" fn FlsAlloc ( lpcallback : PFLS_CALLBACK_FUNCTION ) -> u32 );
    FlsAlloc(lpcallback)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FlsFree(dwflsindex: u32) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn FlsFree ( dwflsindex : u32 ) -> super::super::Foundation:: BOOL );
    FlsFree(dwflsindex)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn FlsGetValue(dwflsindex: u32) -> *mut ::core::ffi::c_void {
    ::windows::imp::link ! ( "kernel32.dll""system" fn FlsGetValue ( dwflsindex : u32 ) -> *mut ::core::ffi::c_void );
    FlsGetValue(dwflsindex)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FlsSetValue(dwflsindex: u32, lpflsdata: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn FlsSetValue ( dwflsindex : u32 , lpflsdata : *const ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    FlsSetValue(dwflsindex, ::core::mem::transmute(lpflsdata.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn FlushProcessWriteBuffers() {
    ::windows::imp::link ! ( "kernel32.dll""system" fn FlushProcessWriteBuffers ( ) -> ( ) );
    FlushProcessWriteBuffers()
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FreeLibraryWhenCallbackReturns<P0, P1>(pci: P0, r#mod: P1)
where
    P0: ::windows::core::IntoParam<PTP_CALLBACK_INSTANCE>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HINSTANCE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn FreeLibraryWhenCallbackReturns ( pci : PTP_CALLBACK_INSTANCE , r#mod : super::super::Foundation:: HINSTANCE ) -> ( ) );
    FreeLibraryWhenCallbackReturns(pci.into_param().abi(), r#mod.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn GetActiveProcessorCount(groupnumber: u16) -> u32 {
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetActiveProcessorCount ( groupnumber : u16 ) -> u32 );
    GetActiveProcessorCount(groupnumber)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn GetActiveProcessorGroupCount() -> u16 {
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetActiveProcessorGroupCount ( ) -> u16 );
    GetActiveProcessorGroupCount()
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCurrentProcess() -> super::super::Foundation::HANDLE {
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetCurrentProcess ( ) -> super::super::Foundation:: HANDLE );
    GetCurrentProcess()
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn GetCurrentProcessId() -> u32 {
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetCurrentProcessId ( ) -> u32 );
    GetCurrentProcessId()
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn GetCurrentProcessorNumber() -> u32 {
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetCurrentProcessorNumber ( ) -> u32 );
    GetCurrentProcessorNumber()
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(feature = "Win32_System_Kernel")]
#[inline]
pub unsafe fn GetCurrentProcessorNumberEx() -> super::Kernel::PROCESSOR_NUMBER {
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetCurrentProcessorNumberEx ( procnumber : *mut super::Kernel:: PROCESSOR_NUMBER ) -> ( ) );
    let mut result__ = ::windows::core::zeroed::<super::Kernel::PROCESSOR_NUMBER>();
    GetCurrentProcessorNumberEx(&mut result__);
    ::std::mem::transmute(result__)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCurrentThread() -> super::super::Foundation::HANDLE {
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetCurrentThread ( ) -> super::super::Foundation:: HANDLE );
    GetCurrentThread()
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn GetCurrentThreadId() -> u32 {
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetCurrentThreadId ( ) -> u32 );
    GetCurrentThreadId()
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn GetCurrentThreadStackLimits(lowlimit: *mut usize, highlimit: *mut usize) {
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetCurrentThreadStackLimits ( lowlimit : *mut usize , highlimit : *mut usize ) -> ( ) );
    GetCurrentThreadStackLimits(lowlimit, highlimit)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn GetCurrentUmsThread() -> *mut ::core::ffi::c_void {
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetCurrentUmsThread ( ) -> *mut ::core::ffi::c_void );
    GetCurrentUmsThread()
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetExitCodeProcess<P0>(hprocess: P0, lpexitcode: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetExitCodeProcess ( hprocess : super::super::Foundation:: HANDLE , lpexitcode : *mut u32 ) -> super::super::Foundation:: BOOL );
    GetExitCodeProcess(hprocess.into_param().abi(), lpexitcode)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetExitCodeThread<P0>(hthread: P0, lpexitcode: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetExitCodeThread ( hthread : super::super::Foundation:: HANDLE , lpexitcode : *mut u32 ) -> super::super::Foundation:: BOOL );
    GetExitCodeThread(hthread.into_param().abi(), lpexitcode)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetGuiResources<P0>(hprocess: P0, uiflags: GET_GUI_RESOURCES_FLAGS) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "user32.dll""system" fn GetGuiResources ( hprocess : super::super::Foundation:: HANDLE , uiflags : GET_GUI_RESOURCES_FLAGS ) -> u32 );
    GetGuiResources(hprocess.into_param().abi(), uiflags)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn GetMachineTypeAttributes(machine: u16) -> ::windows::core::Result<MACHINE_ATTRIBUTES> {
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetMachineTypeAttributes ( machine : u16 , machinetypeattributes : *mut MACHINE_ATTRIBUTES ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<MACHINE_ATTRIBUTES>();
    GetMachineTypeAttributes(machine, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn GetMaximumProcessorCount(groupnumber: u16) -> u32 {
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetMaximumProcessorCount ( groupnumber : u16 ) -> u32 );
    GetMaximumProcessorCount(groupnumber)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn GetMaximumProcessorGroupCount() -> u16 {
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetMaximumProcessorGroupCount ( ) -> u16 );
    GetMaximumProcessorGroupCount()
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn GetNextUmsListItem(umscontext: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetNextUmsListItem ( umscontext : *mut ::core::ffi::c_void ) -> *mut ::core::ffi::c_void );
    GetNextUmsListItem(umscontext)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetNumaAvailableMemoryNode(node: u8, availablebytes: *mut u64) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetNumaAvailableMemoryNode ( node : u8 , availablebytes : *mut u64 ) -> super::super::Foundation:: BOOL );
    GetNumaAvailableMemoryNode(node, availablebytes)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetNumaAvailableMemoryNodeEx(node: u16, availablebytes: *mut u64) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetNumaAvailableMemoryNodeEx ( node : u16 , availablebytes : *mut u64 ) -> super::super::Foundation:: BOOL );
    GetNumaAvailableMemoryNodeEx(node, availablebytes)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetNumaHighestNodeNumber(highestnodenumber: *mut u32) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetNumaHighestNodeNumber ( highestnodenumber : *mut u32 ) -> super::super::Foundation:: BOOL );
    GetNumaHighestNodeNumber(highestnodenumber)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetNumaNodeNumberFromHandle<P0>(hfile: P0, nodenumber: *mut u16) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetNumaNodeNumberFromHandle ( hfile : super::super::Foundation:: HANDLE , nodenumber : *mut u16 ) -> super::super::Foundation:: BOOL );
    GetNumaNodeNumberFromHandle(hfile.into_param().abi(), nodenumber)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetNumaNodeProcessorMask(node: u8, processormask: *mut u64) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetNumaNodeProcessorMask ( node : u8 , processormask : *mut u64 ) -> super::super::Foundation:: BOOL );
    GetNumaNodeProcessorMask(node, processormask)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_SystemInformation\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemInformation"))]
#[inline]
pub unsafe fn GetNumaNodeProcessorMask2(nodenumber: u16, processormasks: ::core::option::Option<&mut [super::SystemInformation::GROUP_AFFINITY]>, requiredmaskcount: *mut u16) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetNumaNodeProcessorMask2 ( nodenumber : u16 , processormasks : *mut super::SystemInformation:: GROUP_AFFINITY , processormaskcount : u16 , requiredmaskcount : *mut u16 ) -> super::super::Foundation:: BOOL );
    GetNumaNodeProcessorMask2(nodenumber, ::core::mem::transmute(processormasks.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), processormasks.as_deref().map_or(0, |slice| slice.len() as _), requiredmaskcount)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_SystemInformation\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemInformation"))]
#[inline]
pub unsafe fn GetNumaNodeProcessorMaskEx(node: u16, processormask: *mut super::SystemInformation::GROUP_AFFINITY) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetNumaNodeProcessorMaskEx ( node : u16 , processormask : *mut super::SystemInformation:: GROUP_AFFINITY ) -> super::super::Foundation:: BOOL );
    GetNumaNodeProcessorMaskEx(node, processormask)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetNumaProcessorNode(processor: u8, nodenumber: *mut u8) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetNumaProcessorNode ( processor : u8 , nodenumber : *mut u8 ) -> super::super::Foundation:: BOOL );
    GetNumaProcessorNode(processor, nodenumber)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn GetNumaProcessorNodeEx(processor: *const super::Kernel::PROCESSOR_NUMBER, nodenumber: *mut u16) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetNumaProcessorNodeEx ( processor : *const super::Kernel:: PROCESSOR_NUMBER , nodenumber : *mut u16 ) -> super::super::Foundation:: BOOL );
    GetNumaProcessorNodeEx(processor, nodenumber)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetNumaProximityNode(proximityid: u32, nodenumber: *mut u8) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetNumaProximityNode ( proximityid : u32 , nodenumber : *mut u8 ) -> super::super::Foundation:: BOOL );
    GetNumaProximityNode(proximityid, nodenumber)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetNumaProximityNodeEx(proximityid: u32, nodenumber: *mut u16) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetNumaProximityNodeEx ( proximityid : u32 , nodenumber : *mut u16 ) -> super::super::Foundation:: BOOL );
    GetNumaProximityNodeEx(proximityid, nodenumber)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPriorityClass<P0>(hprocess: P0) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetPriorityClass ( hprocess : super::super::Foundation:: HANDLE ) -> u32 );
    GetPriorityClass(hprocess.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetProcessAffinityMask<P0>(hprocess: P0, lpprocessaffinitymask: *mut usize, lpsystemaffinitymask: *mut usize) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetProcessAffinityMask ( hprocess : super::super::Foundation:: HANDLE , lpprocessaffinitymask : *mut usize , lpsystemaffinitymask : *mut usize ) -> super::super::Foundation:: BOOL );
    GetProcessAffinityMask(hprocess.into_param().abi(), lpprocessaffinitymask, lpsystemaffinitymask)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetProcessDEPPolicy<P0>(hprocess: P0, lpflags: *mut u32, lppermanent: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetProcessDEPPolicy ( hprocess : super::super::Foundation:: HANDLE , lpflags : *mut u32 , lppermanent : *mut super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
    GetProcessDEPPolicy(hprocess.into_param().abi(), lpflags, lppermanent)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_SystemInformation\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemInformation"))]
#[inline]
pub unsafe fn GetProcessDefaultCpuSetMasks<P0>(process: P0, cpusetmasks: ::core::option::Option<&mut [super::SystemInformation::GROUP_AFFINITY]>, requiredmaskcount: *mut u16) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetProcessDefaultCpuSetMasks ( process : super::super::Foundation:: HANDLE , cpusetmasks : *mut super::SystemInformation:: GROUP_AFFINITY , cpusetmaskcount : u16 , requiredmaskcount : *mut u16 ) -> super::super::Foundation:: BOOL );
    GetProcessDefaultCpuSetMasks(process.into_param().abi(), ::core::mem::transmute(cpusetmasks.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), cpusetmasks.as_deref().map_or(0, |slice| slice.len() as _), requiredmaskcount)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetProcessDefaultCpuSets<P0>(process: P0, cpusetids: ::core::option::Option<&mut [u32]>, requiredidcount: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetProcessDefaultCpuSets ( process : super::super::Foundation:: HANDLE , cpusetids : *mut u32 , cpusetidcount : u32 , requiredidcount : *mut u32 ) -> super::super::Foundation:: BOOL );
    GetProcessDefaultCpuSets(process.into_param().abi(), ::core::mem::transmute(cpusetids.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), cpusetids.as_deref().map_or(0, |slice| slice.len() as _), requiredidcount)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetProcessGroupAffinity<P0>(hprocess: P0, groupcount: *mut u16, grouparray: *mut u16) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetProcessGroupAffinity ( hprocess : super::super::Foundation:: HANDLE , groupcount : *mut u16 , grouparray : *mut u16 ) -> super::super::Foundation:: BOOL );
    GetProcessGroupAffinity(hprocess.into_param().abi(), groupcount, grouparray)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetProcessHandleCount<P0>(hprocess: P0, pdwhandlecount: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetProcessHandleCount ( hprocess : super::super::Foundation:: HANDLE , pdwhandlecount : *mut u32 ) -> super::super::Foundation:: BOOL );
    GetProcessHandleCount(hprocess.into_param().abi(), pdwhandlecount)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetProcessId<P0>(process: P0) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetProcessId ( process : super::super::Foundation:: HANDLE ) -> u32 );
    GetProcessId(process.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetProcessIdOfThread<P0>(thread: P0) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetProcessIdOfThread ( thread : super::super::Foundation:: HANDLE ) -> u32 );
    GetProcessIdOfThread(thread.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetProcessInformation<P0>(hprocess: P0, processinformationclass: PROCESS_INFORMATION_CLASS, processinformation: *mut ::core::ffi::c_void, processinformationsize: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetProcessInformation ( hprocess : super::super::Foundation:: HANDLE , processinformationclass : PROCESS_INFORMATION_CLASS , processinformation : *mut ::core::ffi::c_void , processinformationsize : u32 ) -> super::super::Foundation:: BOOL );
    GetProcessInformation(hprocess.into_param().abi(), processinformationclass, processinformation, processinformationsize)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetProcessIoCounters<P0>(hprocess: P0, lpiocounters: *mut IO_COUNTERS) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetProcessIoCounters ( hprocess : super::super::Foundation:: HANDLE , lpiocounters : *mut IO_COUNTERS ) -> super::super::Foundation:: BOOL );
    GetProcessIoCounters(hprocess.into_param().abi(), lpiocounters)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetProcessMitigationPolicy<P0>(hprocess: P0, mitigationpolicy: PROCESS_MITIGATION_POLICY, lpbuffer: *mut ::core::ffi::c_void, dwlength: usize) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetProcessMitigationPolicy ( hprocess : super::super::Foundation:: HANDLE , mitigationpolicy : PROCESS_MITIGATION_POLICY , lpbuffer : *mut ::core::ffi::c_void , dwlength : usize ) -> super::super::Foundation:: BOOL );
    GetProcessMitigationPolicy(hprocess.into_param().abi(), mitigationpolicy, lpbuffer, dwlength)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetProcessPriorityBoost<P0>(hprocess: P0, pdisablepriorityboost: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetProcessPriorityBoost ( hprocess : super::super::Foundation:: HANDLE , pdisablepriorityboost : *mut super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
    GetProcessPriorityBoost(hprocess.into_param().abi(), pdisablepriorityboost)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetProcessShutdownParameters(lpdwlevel: *mut u32, lpdwflags: *mut u32) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetProcessShutdownParameters ( lpdwlevel : *mut u32 , lpdwflags : *mut u32 ) -> super::super::Foundation:: BOOL );
    GetProcessShutdownParameters(lpdwlevel, lpdwflags)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetProcessTimes<P0>(hprocess: P0, lpcreationtime: *mut super::super::Foundation::FILETIME, lpexittime: *mut super::super::Foundation::FILETIME, lpkerneltime: *mut super::super::Foundation::FILETIME, lpusertime: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetProcessTimes ( hprocess : super::super::Foundation:: HANDLE , lpcreationtime : *mut super::super::Foundation:: FILETIME , lpexittime : *mut super::super::Foundation:: FILETIME , lpkerneltime : *mut super::super::Foundation:: FILETIME , lpusertime : *mut super::super::Foundation:: FILETIME ) -> super::super::Foundation:: BOOL );
    GetProcessTimes(hprocess.into_param().abi(), lpcreationtime, lpexittime, lpkerneltime, lpusertime)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn GetProcessVersion(processid: u32) -> u32 {
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetProcessVersion ( processid : u32 ) -> u32 );
    GetProcessVersion(processid)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetProcessWorkingSetSize<P0>(hprocess: P0, lpminimumworkingsetsize: *mut usize, lpmaximumworkingsetsize: *mut usize) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetProcessWorkingSetSize ( hprocess : super::super::Foundation:: HANDLE , lpminimumworkingsetsize : *mut usize , lpmaximumworkingsetsize : *mut usize ) -> super::super::Foundation:: BOOL );
    GetProcessWorkingSetSize(hprocess.into_param().abi(), lpminimumworkingsetsize, lpmaximumworkingsetsize)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetStartupInfoA(lpstartupinfo: *mut STARTUPINFOA) {
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetStartupInfoA ( lpstartupinfo : *mut STARTUPINFOA ) -> ( ) );
    GetStartupInfoA(lpstartupinfo)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetStartupInfoW(lpstartupinfo: *mut STARTUPINFOW) {
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetStartupInfoW ( lpstartupinfo : *mut STARTUPINFOW ) -> ( ) );
    GetStartupInfoW(lpstartupinfo)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSystemTimes(lpidletime: ::core::option::Option<*mut super::super::Foundation::FILETIME>, lpkerneltime: ::core::option::Option<*mut super::super::Foundation::FILETIME>, lpusertime: ::core::option::Option<*mut super::super::Foundation::FILETIME>) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetSystemTimes ( lpidletime : *mut super::super::Foundation:: FILETIME , lpkerneltime : *mut super::super::Foundation:: FILETIME , lpusertime : *mut super::super::Foundation:: FILETIME ) -> super::super::Foundation:: BOOL );
    GetSystemTimes(::core::mem::transmute(lpidletime.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lpkerneltime.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lpusertime.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetThreadDescription<P0>(hthread: P0) -> ::windows::core::Result<::windows::core::PWSTR>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetThreadDescription ( hthread : super::super::Foundation:: HANDLE , ppszthreaddescription : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
    GetThreadDescription(hthread.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_SystemInformation\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemInformation"))]
#[inline]
pub unsafe fn GetThreadGroupAffinity<P0>(hthread: P0, groupaffinity: *mut super::SystemInformation::GROUP_AFFINITY) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetThreadGroupAffinity ( hthread : super::super::Foundation:: HANDLE , groupaffinity : *mut super::SystemInformation:: GROUP_AFFINITY ) -> super::super::Foundation:: BOOL );
    GetThreadGroupAffinity(hthread.into_param().abi(), groupaffinity)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetThreadIOPendingFlag<P0>(hthread: P0, lpioispending: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetThreadIOPendingFlag ( hthread : super::super::Foundation:: HANDLE , lpioispending : *mut super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
    GetThreadIOPendingFlag(hthread.into_param().abi(), lpioispending)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetThreadId<P0>(thread: P0) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetThreadId ( thread : super::super::Foundation:: HANDLE ) -> u32 );
    GetThreadId(thread.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn GetThreadIdealProcessorEx<P0>(hthread: P0, lpidealprocessor: *mut super::Kernel::PROCESSOR_NUMBER) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetThreadIdealProcessorEx ( hthread : super::super::Foundation:: HANDLE , lpidealprocessor : *mut super::Kernel:: PROCESSOR_NUMBER ) -> super::super::Foundation:: BOOL );
    GetThreadIdealProcessorEx(hthread.into_param().abi(), lpidealprocessor)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetThreadInformation<P0>(hthread: P0, threadinformationclass: THREAD_INFORMATION_CLASS, threadinformation: *mut ::core::ffi::c_void, threadinformationsize: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetThreadInformation ( hthread : super::super::Foundation:: HANDLE , threadinformationclass : THREAD_INFORMATION_CLASS , threadinformation : *mut ::core::ffi::c_void , threadinformationsize : u32 ) -> super::super::Foundation:: BOOL );
    GetThreadInformation(hthread.into_param().abi(), threadinformationclass, threadinformation, threadinformationsize)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetThreadPriority<P0>(hthread: P0) -> i32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetThreadPriority ( hthread : super::super::Foundation:: HANDLE ) -> i32 );
    GetThreadPriority(hthread.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetThreadPriorityBoost<P0>(hthread: P0, pdisablepriorityboost: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetThreadPriorityBoost ( hthread : super::super::Foundation:: HANDLE , pdisablepriorityboost : *mut super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
    GetThreadPriorityBoost(hthread.into_param().abi(), pdisablepriorityboost)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_SystemInformation\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemInformation"))]
#[inline]
pub unsafe fn GetThreadSelectedCpuSetMasks<P0>(thread: P0, cpusetmasks: ::core::option::Option<&mut [super::SystemInformation::GROUP_AFFINITY]>, requiredmaskcount: *mut u16) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetThreadSelectedCpuSetMasks ( thread : super::super::Foundation:: HANDLE , cpusetmasks : *mut super::SystemInformation:: GROUP_AFFINITY , cpusetmaskcount : u16 , requiredmaskcount : *mut u16 ) -> super::super::Foundation:: BOOL );
    GetThreadSelectedCpuSetMasks(thread.into_param().abi(), ::core::mem::transmute(cpusetmasks.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), cpusetmasks.as_deref().map_or(0, |slice| slice.len() as _), requiredmaskcount)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetThreadSelectedCpuSets<P0>(thread: P0, cpusetids: ::core::option::Option<&mut [u32]>, requiredidcount: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetThreadSelectedCpuSets ( thread : super::super::Foundation:: HANDLE , cpusetids : *mut u32 , cpusetidcount : u32 , requiredidcount : *mut u32 ) -> super::super::Foundation:: BOOL );
    GetThreadSelectedCpuSets(thread.into_param().abi(), ::core::mem::transmute(cpusetids.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), cpusetids.as_deref().map_or(0, |slice| slice.len() as _), requiredidcount)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetThreadTimes<P0>(hthread: P0, lpcreationtime: *mut super::super::Foundation::FILETIME, lpexittime: *mut super::super::Foundation::FILETIME, lpkerneltime: *mut super::super::Foundation::FILETIME, lpusertime: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetThreadTimes ( hthread : super::super::Foundation:: HANDLE , lpcreationtime : *mut super::super::Foundation:: FILETIME , lpexittime : *mut super::super::Foundation:: FILETIME , lpkerneltime : *mut super::super::Foundation:: FILETIME , lpusertime : *mut super::super::Foundation:: FILETIME ) -> super::super::Foundation:: BOOL );
    GetThreadTimes(hthread.into_param().abi(), lpcreationtime, lpexittime, lpkerneltime, lpusertime)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetUmsCompletionListEvent(umscompletionlist: *const ::core::ffi::c_void, umscompletionevent: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetUmsCompletionListEvent ( umscompletionlist : *const ::core::ffi::c_void , umscompletionevent : *mut super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    GetUmsCompletionListEvent(umscompletionlist, umscompletionevent)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetUmsSystemThreadInformation<P0>(threadhandle: P0, systemthreadinfo: *mut UMS_SYSTEM_THREAD_INFORMATION) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetUmsSystemThreadInformation ( threadhandle : super::super::Foundation:: HANDLE , systemthreadinfo : *mut UMS_SYSTEM_THREAD_INFORMATION ) -> super::super::Foundation:: BOOL );
    GetUmsSystemThreadInformation(threadhandle.into_param().abi(), systemthreadinfo)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InitOnceBeginInitialize(lpinitonce: *mut RTL_RUN_ONCE, dwflags: u32, fpending: *mut super::super::Foundation::BOOL, lpcontext: ::core::option::Option<*mut *mut ::core::ffi::c_void>) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn InitOnceBeginInitialize ( lpinitonce : *mut RTL_RUN_ONCE , dwflags : u32 , fpending : *mut super::super::Foundation:: BOOL , lpcontext : *mut *mut ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    InitOnceBeginInitialize(lpinitonce, dwflags, fpending, ::core::mem::transmute(lpcontext.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InitOnceComplete(lpinitonce: *mut RTL_RUN_ONCE, dwflags: u32, lpcontext: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn InitOnceComplete ( lpinitonce : *mut RTL_RUN_ONCE , dwflags : u32 , lpcontext : *const ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    InitOnceComplete(lpinitonce, dwflags, ::core::mem::transmute(lpcontext.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InitOnceExecuteOnce(initonce: *mut RTL_RUN_ONCE, initfn: PINIT_ONCE_FN, parameter: ::core::option::Option<*mut ::core::ffi::c_void>, context: ::core::option::Option<*mut *mut ::core::ffi::c_void>) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn InitOnceExecuteOnce ( initonce : *mut RTL_RUN_ONCE , initfn : PINIT_ONCE_FN , parameter : *mut ::core::ffi::c_void , context : *mut *mut ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    InitOnceExecuteOnce(initonce, initfn, ::core::mem::transmute(parameter.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(context.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn InitOnceInitialize() -> RTL_RUN_ONCE {
    ::windows::imp::link ! ( "kernel32.dll""system" fn InitOnceInitialize ( initonce : *mut RTL_RUN_ONCE ) -> ( ) );
    let mut result__ = ::windows::core::zeroed::<RTL_RUN_ONCE>();
    InitOnceInitialize(&mut result__);
    ::std::mem::transmute(result__)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn InitializeConditionVariable() -> RTL_CONDITION_VARIABLE {
    ::windows::imp::link ! ( "kernel32.dll""system" fn InitializeConditionVariable ( conditionvariable : *mut RTL_CONDITION_VARIABLE ) -> ( ) );
    let mut result__ = ::windows::core::zeroed::<RTL_CONDITION_VARIABLE>();
    InitializeConditionVariable(&mut result__);
    ::std::mem::transmute(result__)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn InitializeCriticalSection(lpcriticalsection: *mut RTL_CRITICAL_SECTION) {
    ::windows::imp::link ! ( "kernel32.dll""system" fn InitializeCriticalSection ( lpcriticalsection : *mut RTL_CRITICAL_SECTION ) -> ( ) );
    InitializeCriticalSection(lpcriticalsection)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn InitializeCriticalSectionAndSpinCount(lpcriticalsection: *mut RTL_CRITICAL_SECTION, dwspincount: u32) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn InitializeCriticalSectionAndSpinCount ( lpcriticalsection : *mut RTL_CRITICAL_SECTION , dwspincount : u32 ) -> super::super::Foundation:: BOOL );
    InitializeCriticalSectionAndSpinCount(lpcriticalsection, dwspincount)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn InitializeCriticalSectionEx(lpcriticalsection: *mut RTL_CRITICAL_SECTION, dwspincount: u32, flags: u32) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn InitializeCriticalSectionEx ( lpcriticalsection : *mut RTL_CRITICAL_SECTION , dwspincount : u32 , flags : u32 ) -> super::super::Foundation:: BOOL );
    InitializeCriticalSectionEx(lpcriticalsection, dwspincount, flags)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InitializeProcThreadAttributeList(lpattributelist: LPPROC_THREAD_ATTRIBUTE_LIST, dwattributecount: u32, dwflags: u32, lpsize: *mut usize) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn InitializeProcThreadAttributeList ( lpattributelist : LPPROC_THREAD_ATTRIBUTE_LIST , dwattributecount : u32 , dwflags : u32 , lpsize : *mut usize ) -> super::super::Foundation:: BOOL );
    InitializeProcThreadAttributeList(lpattributelist, dwattributecount, dwflags, lpsize)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(feature = "Win32_System_Kernel")]
#[inline]
pub unsafe fn InitializeSListHead() -> super::Kernel::SLIST_HEADER {
    ::windows::imp::link ! ( "kernel32.dll""system" fn InitializeSListHead ( listhead : *mut super::Kernel:: SLIST_HEADER ) -> ( ) );
    let mut result__ = ::windows::core::zeroed::<super::Kernel::SLIST_HEADER>();
    InitializeSListHead(&mut result__);
    ::std::mem::transmute(result__)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn InitializeSRWLock() -> RTL_SRWLOCK {
    ::windows::imp::link ! ( "kernel32.dll""system" fn InitializeSRWLock ( srwlock : *mut RTL_SRWLOCK ) -> ( ) );
    let mut result__ = ::windows::core::zeroed::<RTL_SRWLOCK>();
    InitializeSRWLock(&mut result__);
    ::std::mem::transmute(result__)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InitializeSynchronizationBarrier(lpbarrier: *mut RTL_BARRIER, ltotalthreads: i32, lspincount: i32) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn InitializeSynchronizationBarrier ( lpbarrier : *mut RTL_BARRIER , ltotalthreads : i32 , lspincount : i32 ) -> super::super::Foundation:: BOOL );
    InitializeSynchronizationBarrier(lpbarrier, ltotalthreads, lspincount)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(feature = "Win32_System_Kernel")]
#[inline]
pub unsafe fn InterlockedFlushSList(listhead: *mut super::Kernel::SLIST_HEADER) -> *mut super::Kernel::SLIST_ENTRY {
    ::windows::imp::link ! ( "kernel32.dll""system" fn InterlockedFlushSList ( listhead : *mut super::Kernel:: SLIST_HEADER ) -> *mut super::Kernel:: SLIST_ENTRY );
    InterlockedFlushSList(listhead)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(feature = "Win32_System_Kernel")]
#[inline]
pub unsafe fn InterlockedPopEntrySList(listhead: *mut super::Kernel::SLIST_HEADER) -> *mut super::Kernel::SLIST_ENTRY {
    ::windows::imp::link ! ( "kernel32.dll""system" fn InterlockedPopEntrySList ( listhead : *mut super::Kernel:: SLIST_HEADER ) -> *mut super::Kernel:: SLIST_ENTRY );
    InterlockedPopEntrySList(listhead)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(feature = "Win32_System_Kernel")]
#[inline]
pub unsafe fn InterlockedPushEntrySList(listhead: *mut super::Kernel::SLIST_HEADER, listentry: *mut super::Kernel::SLIST_ENTRY) -> *mut super::Kernel::SLIST_ENTRY {
    ::windows::imp::link ! ( "kernel32.dll""system" fn InterlockedPushEntrySList ( listhead : *mut super::Kernel:: SLIST_HEADER , listentry : *mut super::Kernel:: SLIST_ENTRY ) -> *mut super::Kernel:: SLIST_ENTRY );
    InterlockedPushEntrySList(listhead, listentry)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(feature = "Win32_System_Kernel")]
#[inline]
pub unsafe fn InterlockedPushListSListEx(listhead: *mut super::Kernel::SLIST_HEADER, list: *mut super::Kernel::SLIST_ENTRY, listend: *mut super::Kernel::SLIST_ENTRY, count: u32) -> *mut super::Kernel::SLIST_ENTRY {
    ::windows::imp::link ! ( "kernel32.dll""system" fn InterlockedPushListSListEx ( listhead : *mut super::Kernel:: SLIST_HEADER , list : *mut super::Kernel:: SLIST_ENTRY , listend : *mut super::Kernel:: SLIST_ENTRY , count : u32 ) -> *mut super::Kernel:: SLIST_ENTRY );
    InterlockedPushListSListEx(listhead, list, listend, count)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsImmersiveProcess<P0>(hprocess: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "user32.dll""system" fn IsImmersiveProcess ( hprocess : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    IsImmersiveProcess(hprocess.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsProcessCritical<P0>(hprocess: P0, critical: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn IsProcessCritical ( hprocess : super::super::Foundation:: HANDLE , critical : *mut super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
    IsProcessCritical(hprocess.into_param().abi(), critical)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsProcessorFeaturePresent(processorfeature: PROCESSOR_FEATURE_ID) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn IsProcessorFeaturePresent ( processorfeature : PROCESSOR_FEATURE_ID ) -> super::super::Foundation:: BOOL );
    IsProcessorFeaturePresent(processorfeature)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsThreadAFiber() -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn IsThreadAFiber ( ) -> super::super::Foundation:: BOOL );
    IsThreadAFiber()
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsThreadpoolTimerSet<P0>(pti: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<PTP_TIMER>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn IsThreadpoolTimerSet ( pti : PTP_TIMER ) -> super::super::Foundation:: BOOL );
    IsThreadpoolTimerSet(pti.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsWow64Process<P0>(hprocess: P0, wow64process: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn IsWow64Process ( hprocess : super::super::Foundation:: HANDLE , wow64process : *mut super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
    IsWow64Process(hprocess.into_param().abi(), wow64process)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_SystemInformation\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemInformation"))]
#[inline]
pub unsafe fn IsWow64Process2<P0>(hprocess: P0, pprocessmachine: *mut super::SystemInformation::IMAGE_FILE_MACHINE, pnativemachine: ::core::option::Option<*mut super::SystemInformation::IMAGE_FILE_MACHINE>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn IsWow64Process2 ( hprocess : super::super::Foundation:: HANDLE , pprocessmachine : *mut super::SystemInformation:: IMAGE_FILE_MACHINE , pnativemachine : *mut super::SystemInformation:: IMAGE_FILE_MACHINE ) -> super::super::Foundation:: BOOL );
    IsWow64Process2(hprocess.into_param().abi(), pprocessmachine, ::core::mem::transmute(pnativemachine.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn LeaveCriticalSection(lpcriticalsection: *mut RTL_CRITICAL_SECTION) {
    ::windows::imp::link ! ( "kernel32.dll""system" fn LeaveCriticalSection ( lpcriticalsection : *mut RTL_CRITICAL_SECTION ) -> ( ) );
    LeaveCriticalSection(lpcriticalsection)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn LeaveCriticalSectionWhenCallbackReturns<P0>(pci: P0, pcs: *mut RTL_CRITICAL_SECTION)
where
    P0: ::windows::core::IntoParam<PTP_CALLBACK_INSTANCE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn LeaveCriticalSectionWhenCallbackReturns ( pci : PTP_CALLBACK_INSTANCE , pcs : *mut RTL_CRITICAL_SECTION ) -> ( ) );
    LeaveCriticalSectionWhenCallbackReturns(pci.into_param().abi(), pcs)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NtQueryInformationProcess<P0>(processhandle: P0, processinformationclass: PROCESSINFOCLASS, processinformation: *mut ::core::ffi::c_void, processinformationlength: u32, returnlength: *mut u32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "ntdll.dll""system" fn NtQueryInformationProcess ( processhandle : super::super::Foundation:: HANDLE , processinformationclass : PROCESSINFOCLASS , processinformation : *mut ::core::ffi::c_void , processinformationlength : u32 , returnlength : *mut u32 ) -> super::super::Foundation:: NTSTATUS );
    NtQueryInformationProcess(processhandle.into_param().abi(), processinformationclass, processinformation, processinformationlength, returnlength).ok()
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NtQueryInformationThread<P0>(threadhandle: P0, threadinformationclass: THREADINFOCLASS, threadinformation: *mut ::core::ffi::c_void, threadinformationlength: u32, returnlength: *mut u32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "ntdll.dll""system" fn NtQueryInformationThread ( threadhandle : super::super::Foundation:: HANDLE , threadinformationclass : THREADINFOCLASS , threadinformation : *mut ::core::ffi::c_void , threadinformationlength : u32 , returnlength : *mut u32 ) -> super::super::Foundation:: NTSTATUS );
    NtQueryInformationThread(threadhandle.into_param().abi(), threadinformationclass, threadinformation, threadinformationlength, returnlength).ok()
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NtSetInformationThread<P0>(threadhandle: P0, threadinformationclass: THREADINFOCLASS, threadinformation: *const ::core::ffi::c_void, threadinformationlength: u32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "ntdll.dll""system" fn NtSetInformationThread ( threadhandle : super::super::Foundation:: HANDLE , threadinformationclass : THREADINFOCLASS , threadinformation : *const ::core::ffi::c_void , threadinformationlength : u32 ) -> super::super::Foundation:: NTSTATUS );
    NtSetInformationThread(threadhandle.into_param().abi(), threadinformationclass, threadinformation, threadinformationlength).ok()
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenEventA<P0, P1>(dwdesiredaccess: SYNCHRONIZATION_ACCESS_RIGHTS, binherithandle: P0, lpname: P1) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn OpenEventA ( dwdesiredaccess : SYNCHRONIZATION_ACCESS_RIGHTS , binherithandle : super::super::Foundation:: BOOL , lpname : :: windows::core::PCSTR ) -> super::super::Foundation:: HANDLE );
    let result__ = OpenEventA(dwdesiredaccess, binherithandle.into_param().abi(), lpname.into_param().abi());
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenEventW<P0, P1>(dwdesiredaccess: SYNCHRONIZATION_ACCESS_RIGHTS, binherithandle: P0, lpname: P1) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn OpenEventW ( dwdesiredaccess : SYNCHRONIZATION_ACCESS_RIGHTS , binherithandle : super::super::Foundation:: BOOL , lpname : :: windows::core::PCWSTR ) -> super::super::Foundation:: HANDLE );
    let result__ = OpenEventW(dwdesiredaccess, binherithandle.into_param().abi(), lpname.into_param().abi());
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenMutexW<P0, P1>(dwdesiredaccess: SYNCHRONIZATION_ACCESS_RIGHTS, binherithandle: P0, lpname: P1) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn OpenMutexW ( dwdesiredaccess : SYNCHRONIZATION_ACCESS_RIGHTS , binherithandle : super::super::Foundation:: BOOL , lpname : :: windows::core::PCWSTR ) -> super::super::Foundation:: HANDLE );
    let result__ = OpenMutexW(dwdesiredaccess, binherithandle.into_param().abi(), lpname.into_param().abi());
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn OpenPrivateNamespaceA<P0>(lpboundarydescriptor: *const ::core::ffi::c_void, lpaliasprefix: P0) -> NamespaceHandle
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn OpenPrivateNamespaceA ( lpboundarydescriptor : *const ::core::ffi::c_void , lpaliasprefix : :: windows::core::PCSTR ) -> NamespaceHandle );
    OpenPrivateNamespaceA(lpboundarydescriptor, lpaliasprefix.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn OpenPrivateNamespaceW<P0>(lpboundarydescriptor: *const ::core::ffi::c_void, lpaliasprefix: P0) -> NamespaceHandle
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn OpenPrivateNamespaceW ( lpboundarydescriptor : *const ::core::ffi::c_void , lpaliasprefix : :: windows::core::PCWSTR ) -> NamespaceHandle );
    OpenPrivateNamespaceW(lpboundarydescriptor, lpaliasprefix.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenProcess<P0>(dwdesiredaccess: PROCESS_ACCESS_RIGHTS, binherithandle: P0, dwprocessid: u32) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn OpenProcess ( dwdesiredaccess : PROCESS_ACCESS_RIGHTS , binherithandle : super::super::Foundation:: BOOL , dwprocessid : u32 ) -> super::super::Foundation:: HANDLE );
    let result__ = OpenProcess(dwdesiredaccess, binherithandle.into_param().abi(), dwprocessid);
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn OpenProcessToken<P0>(processhandle: P0, desiredaccess: super::super::Security::TOKEN_ACCESS_MASK, tokenhandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn OpenProcessToken ( processhandle : super::super::Foundation:: HANDLE , desiredaccess : super::super::Security:: TOKEN_ACCESS_MASK , tokenhandle : *mut super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    OpenProcessToken(processhandle.into_param().abi(), desiredaccess, tokenhandle)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenSemaphoreW<P0, P1>(dwdesiredaccess: SYNCHRONIZATION_ACCESS_RIGHTS, binherithandle: P0, lpname: P1) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn OpenSemaphoreW ( dwdesiredaccess : SYNCHRONIZATION_ACCESS_RIGHTS , binherithandle : super::super::Foundation:: BOOL , lpname : :: windows::core::PCWSTR ) -> super::super::Foundation:: HANDLE );
    let result__ = OpenSemaphoreW(dwdesiredaccess, binherithandle.into_param().abi(), lpname.into_param().abi());
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenThread<P0>(dwdesiredaccess: THREAD_ACCESS_RIGHTS, binherithandle: P0, dwthreadid: u32) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn OpenThread ( dwdesiredaccess : THREAD_ACCESS_RIGHTS , binherithandle : super::super::Foundation:: BOOL , dwthreadid : u32 ) -> super::super::Foundation:: HANDLE );
    let result__ = OpenThread(dwdesiredaccess, binherithandle.into_param().abi(), dwthreadid);
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn OpenThreadToken<P0, P1>(threadhandle: P0, desiredaccess: super::super::Security::TOKEN_ACCESS_MASK, openasself: P1, tokenhandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn OpenThreadToken ( threadhandle : super::super::Foundation:: HANDLE , desiredaccess : super::super::Security:: TOKEN_ACCESS_MASK , openasself : super::super::Foundation:: BOOL , tokenhandle : *mut super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    OpenThreadToken(threadhandle.into_param().abi(), desiredaccess, openasself.into_param().abi(), tokenhandle)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenWaitableTimerW<P0, P1>(dwdesiredaccess: SYNCHRONIZATION_ACCESS_RIGHTS, binherithandle: P0, lptimername: P1) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn OpenWaitableTimerW ( dwdesiredaccess : SYNCHRONIZATION_ACCESS_RIGHTS , binherithandle : super::super::Foundation:: BOOL , lptimername : :: windows::core::PCWSTR ) -> super::super::Foundation:: HANDLE );
    let result__ = OpenWaitableTimerW(dwdesiredaccess, binherithandle.into_param().abi(), lptimername.into_param().abi());
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PulseEvent<P0>(hevent: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn PulseEvent ( hevent : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    PulseEvent(hevent.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(feature = "Win32_System_Kernel")]
#[inline]
pub unsafe fn QueryDepthSList(listhead: *const super::Kernel::SLIST_HEADER) -> u16 {
    ::windows::imp::link ! ( "kernel32.dll""system" fn QueryDepthSList ( listhead : *const super::Kernel:: SLIST_HEADER ) -> u16 );
    QueryDepthSList(listhead)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryFullProcessImageNameA<P0>(hprocess: P0, dwflags: PROCESS_NAME_FORMAT, lpexename: ::windows::core::PSTR, lpdwsize: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn QueryFullProcessImageNameA ( hprocess : super::super::Foundation:: HANDLE , dwflags : PROCESS_NAME_FORMAT , lpexename : :: windows::core::PSTR , lpdwsize : *mut u32 ) -> super::super::Foundation:: BOOL );
    QueryFullProcessImageNameA(hprocess.into_param().abi(), dwflags, ::core::mem::transmute(lpexename), lpdwsize)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryFullProcessImageNameW<P0>(hprocess: P0, dwflags: PROCESS_NAME_FORMAT, lpexename: ::windows::core::PWSTR, lpdwsize: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn QueryFullProcessImageNameW ( hprocess : super::super::Foundation:: HANDLE , dwflags : PROCESS_NAME_FORMAT , lpexename : :: windows::core::PWSTR , lpdwsize : *mut u32 ) -> super::super::Foundation:: BOOL );
    QueryFullProcessImageNameW(hprocess.into_param().abi(), dwflags, ::core::mem::transmute(lpexename), lpdwsize)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryProcessAffinityUpdateMode<P0>(hprocess: P0, lpdwflags: ::core::option::Option<*mut PROCESS_AFFINITY_AUTO_UPDATE_FLAGS>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn QueryProcessAffinityUpdateMode ( hprocess : super::super::Foundation:: HANDLE , lpdwflags : *mut PROCESS_AFFINITY_AUTO_UPDATE_FLAGS ) -> super::super::Foundation:: BOOL );
    QueryProcessAffinityUpdateMode(hprocess.into_param().abi(), ::core::mem::transmute(lpdwflags.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryProtectedPolicy(policyguid: *const ::windows::core::GUID, policyvalue: *mut usize) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn QueryProtectedPolicy ( policyguid : *const :: windows::core::GUID , policyvalue : *mut usize ) -> super::super::Foundation:: BOOL );
    QueryProtectedPolicy(policyguid, policyvalue)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryThreadpoolStackInformation<P0>(ptpp: P0, ptpsi: *mut TP_POOL_STACK_INFORMATION) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<PTP_POOL>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn QueryThreadpoolStackInformation ( ptpp : PTP_POOL , ptpsi : *mut TP_POOL_STACK_INFORMATION ) -> super::super::Foundation:: BOOL );
    QueryThreadpoolStackInformation(ptpp.into_param().abi(), ptpsi)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryUmsThreadInformation(umsthread: *const ::core::ffi::c_void, umsthreadinfoclass: RTL_UMS_THREAD_INFO_CLASS, umsthreadinformation: *mut ::core::ffi::c_void, umsthreadinformationlength: u32, returnlength: ::core::option::Option<*mut u32>) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn QueryUmsThreadInformation ( umsthread : *const ::core::ffi::c_void , umsthreadinfoclass : RTL_UMS_THREAD_INFO_CLASS , umsthreadinformation : *mut ::core::ffi::c_void , umsthreadinformationlength : u32 , returnlength : *mut u32 ) -> super::super::Foundation:: BOOL );
    QueryUmsThreadInformation(umsthread, umsthreadinfoclass, umsthreadinformation, umsthreadinformationlength, ::core::mem::transmute(returnlength.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueueUserAPC<P0>(pfnapc: super::super::Foundation::PAPCFUNC, hthread: P0, dwdata: usize) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn QueueUserAPC ( pfnapc : super::super::Foundation:: PAPCFUNC , hthread : super::super::Foundation:: HANDLE , dwdata : usize ) -> u32 );
    QueueUserAPC(pfnapc, hthread.into_param().abi(), dwdata)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueueUserAPC2<P0>(apcroutine: super::super::Foundation::PAPCFUNC, thread: P0, data: usize, flags: QUEUE_USER_APC_FLAGS) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn QueueUserAPC2 ( apcroutine : super::super::Foundation:: PAPCFUNC , thread : super::super::Foundation:: HANDLE , data : usize , flags : QUEUE_USER_APC_FLAGS ) -> super::super::Foundation:: BOOL );
    QueueUserAPC2(apcroutine, thread.into_param().abi(), data, flags)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueueUserWorkItem(function: LPTHREAD_START_ROUTINE, context: ::core::option::Option<*const ::core::ffi::c_void>, flags: WORKER_THREAD_FLAGS) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn QueueUserWorkItem ( function : LPTHREAD_START_ROUTINE , context : *const ::core::ffi::c_void , flags : WORKER_THREAD_FLAGS ) -> super::super::Foundation:: BOOL );
    QueueUserWorkItem(function, ::core::mem::transmute(context.unwrap_or(::std::ptr::null())), flags)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegisterWaitForSingleObject<P0>(phnewwaitobject: *mut super::super::Foundation::HANDLE, hobject: P0, callback: WAITORTIMERCALLBACK, context: ::core::option::Option<*const ::core::ffi::c_void>, dwmilliseconds: u32, dwflags: WORKER_THREAD_FLAGS) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn RegisterWaitForSingleObject ( phnewwaitobject : *mut super::super::Foundation:: HANDLE , hobject : super::super::Foundation:: HANDLE , callback : WAITORTIMERCALLBACK , context : *const ::core::ffi::c_void , dwmilliseconds : u32 , dwflags : WORKER_THREAD_FLAGS ) -> super::super::Foundation:: BOOL );
    RegisterWaitForSingleObject(phnewwaitobject, hobject.into_param().abi(), callback, ::core::mem::transmute(context.unwrap_or(::std::ptr::null())), dwmilliseconds, dwflags)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReleaseMutex<P0>(hmutex: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn ReleaseMutex ( hmutex : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    ReleaseMutex(hmutex.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReleaseMutexWhenCallbackReturns<P0, P1>(pci: P0, r#mut: P1)
where
    P0: ::windows::core::IntoParam<PTP_CALLBACK_INSTANCE>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn ReleaseMutexWhenCallbackReturns ( pci : PTP_CALLBACK_INSTANCE , r#mut : super::super::Foundation:: HANDLE ) -> ( ) );
    ReleaseMutexWhenCallbackReturns(pci.into_param().abi(), r#mut.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn ReleaseSRWLockExclusive(srwlock: *mut RTL_SRWLOCK) {
    ::windows::imp::link ! ( "kernel32.dll""system" fn ReleaseSRWLockExclusive ( srwlock : *mut RTL_SRWLOCK ) -> ( ) );
    ReleaseSRWLockExclusive(srwlock)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn ReleaseSRWLockShared(srwlock: *mut RTL_SRWLOCK) {
    ::windows::imp::link ! ( "kernel32.dll""system" fn ReleaseSRWLockShared ( srwlock : *mut RTL_SRWLOCK ) -> ( ) );
    ReleaseSRWLockShared(srwlock)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReleaseSemaphore<P0>(hsemaphore: P0, lreleasecount: i32, lppreviouscount: ::core::option::Option<*mut i32>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn ReleaseSemaphore ( hsemaphore : super::super::Foundation:: HANDLE , lreleasecount : i32 , lppreviouscount : *mut i32 ) -> super::super::Foundation:: BOOL );
    ReleaseSemaphore(hsemaphore.into_param().abi(), lreleasecount, ::core::mem::transmute(lppreviouscount.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReleaseSemaphoreWhenCallbackReturns<P0, P1>(pci: P0, sem: P1, crel: u32)
where
    P0: ::windows::core::IntoParam<PTP_CALLBACK_INSTANCE>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn ReleaseSemaphoreWhenCallbackReturns ( pci : PTP_CALLBACK_INSTANCE , sem : super::super::Foundation:: HANDLE , crel : u32 ) -> ( ) );
    ReleaseSemaphoreWhenCallbackReturns(pci.into_param().abi(), sem.into_param().abi(), crel)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ResetEvent<P0>(hevent: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn ResetEvent ( hevent : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    ResetEvent(hevent.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ResumeThread<P0>(hthread: P0) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn ResumeThread ( hthread : super::super::Foundation:: HANDLE ) -> u32 );
    ResumeThread(hthread.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn SetCriticalSectionSpinCount(lpcriticalsection: *mut RTL_CRITICAL_SECTION, dwspincount: u32) -> u32 {
    ::windows::imp::link ! ( "kernel32.dll""system" fn SetCriticalSectionSpinCount ( lpcriticalsection : *mut RTL_CRITICAL_SECTION , dwspincount : u32 ) -> u32 );
    SetCriticalSectionSpinCount(lpcriticalsection, dwspincount)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetEvent<P0>(hevent: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn SetEvent ( hevent : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    SetEvent(hevent.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetEventWhenCallbackReturns<P0, P1>(pci: P0, evt: P1)
where
    P0: ::windows::core::IntoParam<PTP_CALLBACK_INSTANCE>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn SetEventWhenCallbackReturns ( pci : PTP_CALLBACK_INSTANCE , evt : super::super::Foundation:: HANDLE ) -> ( ) );
    SetEventWhenCallbackReturns(pci.into_param().abi(), evt.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetPriorityClass<P0>(hprocess: P0, dwpriorityclass: PROCESS_CREATION_FLAGS) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn SetPriorityClass ( hprocess : super::super::Foundation:: HANDLE , dwpriorityclass : PROCESS_CREATION_FLAGS ) -> super::super::Foundation:: BOOL );
    SetPriorityClass(hprocess.into_param().abi(), dwpriorityclass)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetProcessAffinityMask<P0>(hprocess: P0, dwprocessaffinitymask: usize) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn SetProcessAffinityMask ( hprocess : super::super::Foundation:: HANDLE , dwprocessaffinitymask : usize ) -> super::super::Foundation:: BOOL );
    SetProcessAffinityMask(hprocess.into_param().abi(), dwprocessaffinitymask)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetProcessAffinityUpdateMode<P0>(hprocess: P0, dwflags: PROCESS_AFFINITY_AUTO_UPDATE_FLAGS) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn SetProcessAffinityUpdateMode ( hprocess : super::super::Foundation:: HANDLE , dwflags : PROCESS_AFFINITY_AUTO_UPDATE_FLAGS ) -> super::super::Foundation:: BOOL );
    SetProcessAffinityUpdateMode(hprocess.into_param().abi(), dwflags)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetProcessDEPPolicy(dwflags: PROCESS_DEP_FLAGS) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn SetProcessDEPPolicy ( dwflags : PROCESS_DEP_FLAGS ) -> super::super::Foundation:: BOOL );
    SetProcessDEPPolicy(dwflags)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_SystemInformation\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemInformation"))]
#[inline]
pub unsafe fn SetProcessDefaultCpuSetMasks<P0>(process: P0, cpusetmasks: ::core::option::Option<&[super::SystemInformation::GROUP_AFFINITY]>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn SetProcessDefaultCpuSetMasks ( process : super::super::Foundation:: HANDLE , cpusetmasks : *const super::SystemInformation:: GROUP_AFFINITY , cpusetmaskcount : u16 ) -> super::super::Foundation:: BOOL );
    SetProcessDefaultCpuSetMasks(process.into_param().abi(), ::core::mem::transmute(cpusetmasks.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), cpusetmasks.as_deref().map_or(0, |slice| slice.len() as _))
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetProcessDefaultCpuSets<P0>(process: P0, cpusetids: ::core::option::Option<&[u32]>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn SetProcessDefaultCpuSets ( process : super::super::Foundation:: HANDLE , cpusetids : *const u32 , cpusetidcount : u32 ) -> super::super::Foundation:: BOOL );
    SetProcessDefaultCpuSets(process.into_param().abi(), ::core::mem::transmute(cpusetids.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), cpusetids.as_deref().map_or(0, |slice| slice.len() as _))
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetProcessDynamicEHContinuationTargets<P0>(process: P0, targets: &mut [PROCESS_DYNAMIC_EH_CONTINUATION_TARGET]) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn SetProcessDynamicEHContinuationTargets ( process : super::super::Foundation:: HANDLE , numberoftargets : u16 , targets : *mut PROCESS_DYNAMIC_EH_CONTINUATION_TARGET ) -> super::super::Foundation:: BOOL );
    SetProcessDynamicEHContinuationTargets(process.into_param().abi(), targets.len() as _, ::core::mem::transmute(targets.as_ptr()))
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetProcessDynamicEnforcedCetCompatibleRanges<P0>(process: P0, ranges: &mut [PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE]) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn SetProcessDynamicEnforcedCetCompatibleRanges ( process : super::super::Foundation:: HANDLE , numberofranges : u16 , ranges : *mut PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE ) -> super::super::Foundation:: BOOL );
    SetProcessDynamicEnforcedCetCompatibleRanges(process.into_param().abi(), ranges.len() as _, ::core::mem::transmute(ranges.as_ptr()))
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetProcessInformation<P0>(hprocess: P0, processinformationclass: PROCESS_INFORMATION_CLASS, processinformation: *const ::core::ffi::c_void, processinformationsize: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn SetProcessInformation ( hprocess : super::super::Foundation:: HANDLE , processinformationclass : PROCESS_INFORMATION_CLASS , processinformation : *const ::core::ffi::c_void , processinformationsize : u32 ) -> super::super::Foundation:: BOOL );
    SetProcessInformation(hprocess.into_param().abi(), processinformationclass, processinformation, processinformationsize)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetProcessMitigationPolicy(mitigationpolicy: PROCESS_MITIGATION_POLICY, lpbuffer: *const ::core::ffi::c_void, dwlength: usize) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn SetProcessMitigationPolicy ( mitigationpolicy : PROCESS_MITIGATION_POLICY , lpbuffer : *const ::core::ffi::c_void , dwlength : usize ) -> super::super::Foundation:: BOOL );
    SetProcessMitigationPolicy(mitigationpolicy, lpbuffer, dwlength)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetProcessPriorityBoost<P0, P1>(hprocess: P0, bdisablepriorityboost: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn SetProcessPriorityBoost ( hprocess : super::super::Foundation:: HANDLE , bdisablepriorityboost : super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
    SetProcessPriorityBoost(hprocess.into_param().abi(), bdisablepriorityboost.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetProcessRestrictionExemption<P0>(fenableexemption: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "user32.dll""system" fn SetProcessRestrictionExemption ( fenableexemption : super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
    SetProcessRestrictionExemption(fenableexemption.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetProcessShutdownParameters(dwlevel: u32, dwflags: u32) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn SetProcessShutdownParameters ( dwlevel : u32 , dwflags : u32 ) -> super::super::Foundation:: BOOL );
    SetProcessShutdownParameters(dwlevel, dwflags)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetProcessWorkingSetSize<P0>(hprocess: P0, dwminimumworkingsetsize: usize, dwmaximumworkingsetsize: usize) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn SetProcessWorkingSetSize ( hprocess : super::super::Foundation:: HANDLE , dwminimumworkingsetsize : usize , dwmaximumworkingsetsize : usize ) -> super::super::Foundation:: BOOL );
    SetProcessWorkingSetSize(hprocess.into_param().abi(), dwminimumworkingsetsize, dwmaximumworkingsetsize)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetProtectedPolicy(policyguid: *const ::windows::core::GUID, policyvalue: usize, oldpolicyvalue: ::core::option::Option<*mut usize>) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn SetProtectedPolicy ( policyguid : *const :: windows::core::GUID , policyvalue : usize , oldpolicyvalue : *mut usize ) -> super::super::Foundation:: BOOL );
    SetProtectedPolicy(policyguid, policyvalue, ::core::mem::transmute(oldpolicyvalue.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetThreadAffinityMask<P0>(hthread: P0, dwthreadaffinitymask: usize) -> usize
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn SetThreadAffinityMask ( hthread : super::super::Foundation:: HANDLE , dwthreadaffinitymask : usize ) -> usize );
    SetThreadAffinityMask(hthread.into_param().abi(), dwthreadaffinitymask)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetThreadDescription<P0, P1>(hthread: P0, lpthreaddescription: P1) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn SetThreadDescription ( hthread : super::super::Foundation:: HANDLE , lpthreaddescription : :: windows::core::PCWSTR ) -> :: windows::core::HRESULT );
    SetThreadDescription(hthread.into_param().abi(), lpthreaddescription.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_SystemInformation\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemInformation"))]
#[inline]
pub unsafe fn SetThreadGroupAffinity<P0>(hthread: P0, groupaffinity: *const super::SystemInformation::GROUP_AFFINITY, previousgroupaffinity: ::core::option::Option<*mut super::SystemInformation::GROUP_AFFINITY>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn SetThreadGroupAffinity ( hthread : super::super::Foundation:: HANDLE , groupaffinity : *const super::SystemInformation:: GROUP_AFFINITY , previousgroupaffinity : *mut super::SystemInformation:: GROUP_AFFINITY ) -> super::super::Foundation:: BOOL );
    SetThreadGroupAffinity(hthread.into_param().abi(), groupaffinity, ::core::mem::transmute(previousgroupaffinity.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetThreadIdealProcessor<P0>(hthread: P0, dwidealprocessor: u32) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn SetThreadIdealProcessor ( hthread : super::super::Foundation:: HANDLE , dwidealprocessor : u32 ) -> u32 );
    SetThreadIdealProcessor(hthread.into_param().abi(), dwidealprocessor)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn SetThreadIdealProcessorEx<P0>(hthread: P0, lpidealprocessor: *const super::Kernel::PROCESSOR_NUMBER, lppreviousidealprocessor: ::core::option::Option<*mut super::Kernel::PROCESSOR_NUMBER>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn SetThreadIdealProcessorEx ( hthread : super::super::Foundation:: HANDLE , lpidealprocessor : *const super::Kernel:: PROCESSOR_NUMBER , lppreviousidealprocessor : *mut super::Kernel:: PROCESSOR_NUMBER ) -> super::super::Foundation:: BOOL );
    SetThreadIdealProcessorEx(hthread.into_param().abi(), lpidealprocessor, ::core::mem::transmute(lppreviousidealprocessor.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetThreadInformation<P0>(hthread: P0, threadinformationclass: THREAD_INFORMATION_CLASS, threadinformation: *const ::core::ffi::c_void, threadinformationsize: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn SetThreadInformation ( hthread : super::super::Foundation:: HANDLE , threadinformationclass : THREAD_INFORMATION_CLASS , threadinformation : *const ::core::ffi::c_void , threadinformationsize : u32 ) -> super::super::Foundation:: BOOL );
    SetThreadInformation(hthread.into_param().abi(), threadinformationclass, threadinformation, threadinformationsize)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetThreadPriority<P0>(hthread: P0, npriority: THREAD_PRIORITY) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn SetThreadPriority ( hthread : super::super::Foundation:: HANDLE , npriority : THREAD_PRIORITY ) -> super::super::Foundation:: BOOL );
    SetThreadPriority(hthread.into_param().abi(), npriority)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetThreadPriorityBoost<P0, P1>(hthread: P0, bdisablepriorityboost: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn SetThreadPriorityBoost ( hthread : super::super::Foundation:: HANDLE , bdisablepriorityboost : super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
    SetThreadPriorityBoost(hthread.into_param().abi(), bdisablepriorityboost.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_SystemInformation\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemInformation"))]
#[inline]
pub unsafe fn SetThreadSelectedCpuSetMasks<P0>(thread: P0, cpusetmasks: ::core::option::Option<&[super::SystemInformation::GROUP_AFFINITY]>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn SetThreadSelectedCpuSetMasks ( thread : super::super::Foundation:: HANDLE , cpusetmasks : *const super::SystemInformation:: GROUP_AFFINITY , cpusetmaskcount : u16 ) -> super::super::Foundation:: BOOL );
    SetThreadSelectedCpuSetMasks(thread.into_param().abi(), ::core::mem::transmute(cpusetmasks.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), cpusetmasks.as_deref().map_or(0, |slice| slice.len() as _))
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetThreadSelectedCpuSets<P0>(thread: P0, cpusetids: &[u32]) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn SetThreadSelectedCpuSets ( thread : super::super::Foundation:: HANDLE , cpusetids : *const u32 , cpusetidcount : u32 ) -> super::super::Foundation:: BOOL );
    SetThreadSelectedCpuSets(thread.into_param().abi(), ::core::mem::transmute(cpusetids.as_ptr()), cpusetids.len() as _)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetThreadStackGuarantee(stacksizeinbytes: *mut u32) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn SetThreadStackGuarantee ( stacksizeinbytes : *mut u32 ) -> super::super::Foundation:: BOOL );
    SetThreadStackGuarantee(stacksizeinbytes)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetThreadToken<P0>(thread: ::core::option::Option<*const super::super::Foundation::HANDLE>, token: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn SetThreadToken ( thread : *const super::super::Foundation:: HANDLE , token : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    SetThreadToken(::core::mem::transmute(thread.unwrap_or(::std::ptr::null())), token.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetThreadpoolStackInformation<P0>(ptpp: P0, ptpsi: *const TP_POOL_STACK_INFORMATION) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<PTP_POOL>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn SetThreadpoolStackInformation ( ptpp : PTP_POOL , ptpsi : *const TP_POOL_STACK_INFORMATION ) -> super::super::Foundation:: BOOL );
    SetThreadpoolStackInformation(ptpp.into_param().abi(), ptpsi)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn SetThreadpoolThreadMaximum<P0>(ptpp: P0, cthrdmost: u32)
where
    P0: ::windows::core::IntoParam<PTP_POOL>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn SetThreadpoolThreadMaximum ( ptpp : PTP_POOL , cthrdmost : u32 ) -> ( ) );
    SetThreadpoolThreadMaximum(ptpp.into_param().abi(), cthrdmost)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetThreadpoolThreadMinimum<P0>(ptpp: P0, cthrdmic: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<PTP_POOL>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn SetThreadpoolThreadMinimum ( ptpp : PTP_POOL , cthrdmic : u32 ) -> super::super::Foundation:: BOOL );
    SetThreadpoolThreadMinimum(ptpp.into_param().abi(), cthrdmic)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetThreadpoolTimer<P0>(pti: P0, pftduetime: ::core::option::Option<*const super::super::Foundation::FILETIME>, msperiod: u32, mswindowlength: u32)
where
    P0: ::windows::core::IntoParam<PTP_TIMER>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn SetThreadpoolTimer ( pti : PTP_TIMER , pftduetime : *const super::super::Foundation:: FILETIME , msperiod : u32 , mswindowlength : u32 ) -> ( ) );
    SetThreadpoolTimer(pti.into_param().abi(), ::core::mem::transmute(pftduetime.unwrap_or(::std::ptr::null())), msperiod, mswindowlength)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetThreadpoolTimerEx<P0>(pti: P0, pftduetime: ::core::option::Option<*const super::super::Foundation::FILETIME>, msperiod: u32, mswindowlength: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<PTP_TIMER>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn SetThreadpoolTimerEx ( pti : PTP_TIMER , pftduetime : *const super::super::Foundation:: FILETIME , msperiod : u32 , mswindowlength : u32 ) -> super::super::Foundation:: BOOL );
    SetThreadpoolTimerEx(pti.into_param().abi(), ::core::mem::transmute(pftduetime.unwrap_or(::std::ptr::null())), msperiod, mswindowlength)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetThreadpoolWait<P0, P1>(pwa: P0, h: P1, pfttimeout: ::core::option::Option<*const super::super::Foundation::FILETIME>)
where
    P0: ::windows::core::IntoParam<PTP_WAIT>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn SetThreadpoolWait ( pwa : PTP_WAIT , h : super::super::Foundation:: HANDLE , pfttimeout : *const super::super::Foundation:: FILETIME ) -> ( ) );
    SetThreadpoolWait(pwa.into_param().abi(), h.into_param().abi(), ::core::mem::transmute(pfttimeout.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetThreadpoolWaitEx<P0, P1>(pwa: P0, h: P1, pfttimeout: ::core::option::Option<*const super::super::Foundation::FILETIME>, reserved: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<PTP_WAIT>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn SetThreadpoolWaitEx ( pwa : PTP_WAIT , h : super::super::Foundation:: HANDLE , pfttimeout : *const super::super::Foundation:: FILETIME , reserved : *const ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    SetThreadpoolWaitEx(pwa.into_param().abi(), h.into_param().abi(), ::core::mem::transmute(pfttimeout.unwrap_or(::std::ptr::null())), ::core::mem::transmute(reserved.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetTimerQueueTimer<P0, P1>(timerqueue: P0, callback: WAITORTIMERCALLBACK, parameter: ::core::option::Option<*const ::core::ffi::c_void>, duetime: u32, period: u32, preferio: P1) -> super::super::Foundation::HANDLE
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn SetTimerQueueTimer ( timerqueue : super::super::Foundation:: HANDLE , callback : WAITORTIMERCALLBACK , parameter : *const ::core::ffi::c_void , duetime : u32 , period : u32 , preferio : super::super::Foundation:: BOOL ) -> super::super::Foundation:: HANDLE );
    SetTimerQueueTimer(timerqueue.into_param().abi(), callback, ::core::mem::transmute(parameter.unwrap_or(::std::ptr::null())), duetime, period, preferio.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetUmsThreadInformation(umsthread: *const ::core::ffi::c_void, umsthreadinfoclass: RTL_UMS_THREAD_INFO_CLASS, umsthreadinformation: *const ::core::ffi::c_void, umsthreadinformationlength: u32) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn SetUmsThreadInformation ( umsthread : *const ::core::ffi::c_void , umsthreadinfoclass : RTL_UMS_THREAD_INFO_CLASS , umsthreadinformation : *const ::core::ffi::c_void , umsthreadinformationlength : u32 ) -> super::super::Foundation:: BOOL );
    SetUmsThreadInformation(umsthread, umsthreadinfoclass, umsthreadinformation, umsthreadinformationlength)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetWaitableTimer<P0, P1>(htimer: P0, lpduetime: *const i64, lperiod: i32, pfncompletionroutine: PTIMERAPCROUTINE, lpargtocompletionroutine: ::core::option::Option<*const ::core::ffi::c_void>, fresume: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn SetWaitableTimer ( htimer : super::super::Foundation:: HANDLE , lpduetime : *const i64 , lperiod : i32 , pfncompletionroutine : PTIMERAPCROUTINE , lpargtocompletionroutine : *const ::core::ffi::c_void , fresume : super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
    SetWaitableTimer(htimer.into_param().abi(), lpduetime, lperiod, pfncompletionroutine, ::core::mem::transmute(lpargtocompletionroutine.unwrap_or(::std::ptr::null())), fresume.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetWaitableTimerEx<P0>(htimer: P0, lpduetime: *const i64, lperiod: i32, pfncompletionroutine: PTIMERAPCROUTINE, lpargtocompletionroutine: ::core::option::Option<*const ::core::ffi::c_void>, wakecontext: ::core::option::Option<*const REASON_CONTEXT>, tolerabledelay: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn SetWaitableTimerEx ( htimer : super::super::Foundation:: HANDLE , lpduetime : *const i64 , lperiod : i32 , pfncompletionroutine : PTIMERAPCROUTINE , lpargtocompletionroutine : *const ::core::ffi::c_void , wakecontext : *const REASON_CONTEXT , tolerabledelay : u32 ) -> super::super::Foundation:: BOOL );
    SetWaitableTimerEx(htimer.into_param().abi(), lpduetime, lperiod, pfncompletionroutine, ::core::mem::transmute(lpargtocompletionroutine.unwrap_or(::std::ptr::null())), ::core::mem::transmute(wakecontext.unwrap_or(::std::ptr::null())), tolerabledelay)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn Sleep(dwmilliseconds: u32) {
    ::windows::imp::link ! ( "kernel32.dll""system" fn Sleep ( dwmilliseconds : u32 ) -> ( ) );
    Sleep(dwmilliseconds)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn SleepConditionVariableCS(conditionvariable: *mut RTL_CONDITION_VARIABLE, criticalsection: *mut RTL_CRITICAL_SECTION, dwmilliseconds: u32) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn SleepConditionVariableCS ( conditionvariable : *mut RTL_CONDITION_VARIABLE , criticalsection : *mut RTL_CRITICAL_SECTION , dwmilliseconds : u32 ) -> super::super::Foundation:: BOOL );
    SleepConditionVariableCS(conditionvariable, criticalsection, dwmilliseconds)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SleepConditionVariableSRW(conditionvariable: *mut RTL_CONDITION_VARIABLE, srwlock: *mut RTL_SRWLOCK, dwmilliseconds: u32, flags: u32) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn SleepConditionVariableSRW ( conditionvariable : *mut RTL_CONDITION_VARIABLE , srwlock : *mut RTL_SRWLOCK , dwmilliseconds : u32 , flags : u32 ) -> super::super::Foundation:: BOOL );
    SleepConditionVariableSRW(conditionvariable, srwlock, dwmilliseconds, flags)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SleepEx<P0>(dwmilliseconds: u32, balertable: P0) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn SleepEx ( dwmilliseconds : u32 , balertable : super::super::Foundation:: BOOL ) -> u32 );
    SleepEx(dwmilliseconds, balertable.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn StartThreadpoolIo<P0>(pio: P0)
where
    P0: ::windows::core::IntoParam<PTP_IO>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn StartThreadpoolIo ( pio : PTP_IO ) -> ( ) );
    StartThreadpoolIo(pio.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn SubmitThreadpoolWork<P0>(pwk: P0)
where
    P0: ::windows::core::IntoParam<PTP_WORK>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn SubmitThreadpoolWork ( pwk : PTP_WORK ) -> ( ) );
    SubmitThreadpoolWork(pwk.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SuspendThread<P0>(hthread: P0) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn SuspendThread ( hthread : super::super::Foundation:: HANDLE ) -> u32 );
    SuspendThread(hthread.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn SwitchToFiber(lpfiber: *const ::core::ffi::c_void) {
    ::windows::imp::link ! ( "kernel32.dll""system" fn SwitchToFiber ( lpfiber : *const ::core::ffi::c_void ) -> ( ) );
    SwitchToFiber(lpfiber)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SwitchToThread() -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn SwitchToThread ( ) -> super::super::Foundation:: BOOL );
    SwitchToThread()
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TerminateProcess<P0>(hprocess: P0, uexitcode: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn TerminateProcess ( hprocess : super::super::Foundation:: HANDLE , uexitcode : u32 ) -> super::super::Foundation:: BOOL );
    TerminateProcess(hprocess.into_param().abi(), uexitcode)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TerminateThread<P0>(hthread: P0, dwexitcode: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn TerminateThread ( hthread : super::super::Foundation:: HANDLE , dwexitcode : u32 ) -> super::super::Foundation:: BOOL );
    TerminateThread(hthread.into_param().abi(), dwexitcode)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn TlsAlloc() -> u32 {
    ::windows::imp::link ! ( "kernel32.dll""system" fn TlsAlloc ( ) -> u32 );
    TlsAlloc()
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TlsFree(dwtlsindex: u32) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn TlsFree ( dwtlsindex : u32 ) -> super::super::Foundation:: BOOL );
    TlsFree(dwtlsindex)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn TlsGetValue(dwtlsindex: u32) -> *mut ::core::ffi::c_void {
    ::windows::imp::link ! ( "kernel32.dll""system" fn TlsGetValue ( dwtlsindex : u32 ) -> *mut ::core::ffi::c_void );
    TlsGetValue(dwtlsindex)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TlsSetValue(dwtlsindex: u32, lptlsvalue: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn TlsSetValue ( dwtlsindex : u32 , lptlsvalue : *const ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    TlsSetValue(dwtlsindex, ::core::mem::transmute(lptlsvalue.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TryAcquireSRWLockExclusive(srwlock: *mut RTL_SRWLOCK) -> super::super::Foundation::BOOLEAN {
    ::windows::imp::link ! ( "kernel32.dll""system" fn TryAcquireSRWLockExclusive ( srwlock : *mut RTL_SRWLOCK ) -> super::super::Foundation:: BOOLEAN );
    TryAcquireSRWLockExclusive(srwlock)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TryAcquireSRWLockShared(srwlock: *mut RTL_SRWLOCK) -> super::super::Foundation::BOOLEAN {
    ::windows::imp::link ! ( "kernel32.dll""system" fn TryAcquireSRWLockShared ( srwlock : *mut RTL_SRWLOCK ) -> super::super::Foundation:: BOOLEAN );
    TryAcquireSRWLockShared(srwlock)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn TryEnterCriticalSection(lpcriticalsection: *mut RTL_CRITICAL_SECTION) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn TryEnterCriticalSection ( lpcriticalsection : *mut RTL_CRITICAL_SECTION ) -> super::super::Foundation:: BOOL );
    TryEnterCriticalSection(lpcriticalsection)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TrySubmitThreadpoolCallback(pfns: PTP_SIMPLE_CALLBACK, pv: ::core::option::Option<*mut ::core::ffi::c_void>, pcbe: ::core::option::Option<*const TP_CALLBACK_ENVIRON_V3>) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn TrySubmitThreadpoolCallback ( pfns : PTP_SIMPLE_CALLBACK , pv : *mut ::core::ffi::c_void , pcbe : *const TP_CALLBACK_ENVIRON_V3 ) -> super::super::Foundation:: BOOL );
    TrySubmitThreadpoolCallback(pfns, ::core::mem::transmute(pv.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pcbe.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UmsThreadYield(schedulerparam: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn UmsThreadYield ( schedulerparam : *const ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    UmsThreadYield(schedulerparam)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UnregisterWait<P0>(waithandle: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn UnregisterWait ( waithandle : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    UnregisterWait(waithandle.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UnregisterWaitEx<P0, P1>(waithandle: P0, completionevent: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn UnregisterWaitEx ( waithandle : super::super::Foundation:: HANDLE , completionevent : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    UnregisterWaitEx(waithandle.into_param().abi(), completionevent.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UpdateProcThreadAttribute(lpattributelist: LPPROC_THREAD_ATTRIBUTE_LIST, dwflags: u32, attribute: usize, lpvalue: ::core::option::Option<*const ::core::ffi::c_void>, cbsize: usize, lppreviousvalue: ::core::option::Option<*mut ::core::ffi::c_void>, lpreturnsize: ::core::option::Option<*const usize>) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn UpdateProcThreadAttribute ( lpattributelist : LPPROC_THREAD_ATTRIBUTE_LIST , dwflags : u32 , attribute : usize , lpvalue : *const ::core::ffi::c_void , cbsize : usize , lppreviousvalue : *mut ::core::ffi::c_void , lpreturnsize : *const usize ) -> super::super::Foundation:: BOOL );
    UpdateProcThreadAttribute(lpattributelist, dwflags, attribute, ::core::mem::transmute(lpvalue.unwrap_or(::std::ptr::null())), cbsize, ::core::mem::transmute(lppreviousvalue.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lpreturnsize.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WaitForInputIdle<P0>(hprocess: P0, dwmilliseconds: u32) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "user32.dll""system" fn WaitForInputIdle ( hprocess : super::super::Foundation:: HANDLE , dwmilliseconds : u32 ) -> u32 );
    WaitForInputIdle(hprocess.into_param().abi(), dwmilliseconds)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WaitForMultipleObjects<P0>(lphandles: &[super::super::Foundation::HANDLE], bwaitall: P0, dwmilliseconds: u32) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn WaitForMultipleObjects ( ncount : u32 , lphandles : *const super::super::Foundation:: HANDLE , bwaitall : super::super::Foundation:: BOOL , dwmilliseconds : u32 ) -> super::super::Foundation:: WIN32_ERROR );
    WaitForMultipleObjects(lphandles.len() as _, ::core::mem::transmute(lphandles.as_ptr()), bwaitall.into_param().abi(), dwmilliseconds)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WaitForMultipleObjectsEx<P0, P1>(lphandles: &[super::super::Foundation::HANDLE], bwaitall: P0, dwmilliseconds: u32, balertable: P1) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn WaitForMultipleObjectsEx ( ncount : u32 , lphandles : *const super::super::Foundation:: HANDLE , bwaitall : super::super::Foundation:: BOOL , dwmilliseconds : u32 , balertable : super::super::Foundation:: BOOL ) -> super::super::Foundation:: WIN32_ERROR );
    WaitForMultipleObjectsEx(lphandles.len() as _, ::core::mem::transmute(lphandles.as_ptr()), bwaitall.into_param().abi(), dwmilliseconds, balertable.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WaitForSingleObject<P0>(hhandle: P0, dwmilliseconds: u32) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn WaitForSingleObject ( hhandle : super::super::Foundation:: HANDLE , dwmilliseconds : u32 ) -> super::super::Foundation:: WIN32_ERROR );
    WaitForSingleObject(hhandle.into_param().abi(), dwmilliseconds)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WaitForSingleObjectEx<P0, P1>(hhandle: P0, dwmilliseconds: u32, balertable: P1) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn WaitForSingleObjectEx ( hhandle : super::super::Foundation:: HANDLE , dwmilliseconds : u32 , balertable : super::super::Foundation:: BOOL ) -> super::super::Foundation:: WIN32_ERROR );
    WaitForSingleObjectEx(hhandle.into_param().abi(), dwmilliseconds, balertable.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WaitForThreadpoolIoCallbacks<P0, P1>(pio: P0, fcancelpendingcallbacks: P1)
where
    P0: ::windows::core::IntoParam<PTP_IO>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn WaitForThreadpoolIoCallbacks ( pio : PTP_IO , fcancelpendingcallbacks : super::super::Foundation:: BOOL ) -> ( ) );
    WaitForThreadpoolIoCallbacks(pio.into_param().abi(), fcancelpendingcallbacks.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WaitForThreadpoolTimerCallbacks<P0, P1>(pti: P0, fcancelpendingcallbacks: P1)
where
    P0: ::windows::core::IntoParam<PTP_TIMER>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn WaitForThreadpoolTimerCallbacks ( pti : PTP_TIMER , fcancelpendingcallbacks : super::super::Foundation:: BOOL ) -> ( ) );
    WaitForThreadpoolTimerCallbacks(pti.into_param().abi(), fcancelpendingcallbacks.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WaitForThreadpoolWaitCallbacks<P0, P1>(pwa: P0, fcancelpendingcallbacks: P1)
where
    P0: ::windows::core::IntoParam<PTP_WAIT>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn WaitForThreadpoolWaitCallbacks ( pwa : PTP_WAIT , fcancelpendingcallbacks : super::super::Foundation:: BOOL ) -> ( ) );
    WaitForThreadpoolWaitCallbacks(pwa.into_param().abi(), fcancelpendingcallbacks.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WaitForThreadpoolWorkCallbacks<P0, P1>(pwk: P0, fcancelpendingcallbacks: P1)
where
    P0: ::windows::core::IntoParam<PTP_WORK>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn WaitForThreadpoolWorkCallbacks ( pwk : PTP_WORK , fcancelpendingcallbacks : super::super::Foundation:: BOOL ) -> ( ) );
    WaitForThreadpoolWorkCallbacks(pwk.into_param().abi(), fcancelpendingcallbacks.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WaitOnAddress(address: *const ::core::ffi::c_void, compareaddress: *const ::core::ffi::c_void, addresssize: usize, dwmilliseconds: u32) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "api-ms-win-core-synch-l1-2-0.dll""system" fn WaitOnAddress ( address : *const ::core::ffi::c_void , compareaddress : *const ::core::ffi::c_void , addresssize : usize , dwmilliseconds : u32 ) -> super::super::Foundation:: BOOL );
    WaitOnAddress(address, compareaddress, addresssize, dwmilliseconds)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn WakeAllConditionVariable(conditionvariable: *mut RTL_CONDITION_VARIABLE) {
    ::windows::imp::link ! ( "kernel32.dll""system" fn WakeAllConditionVariable ( conditionvariable : *mut RTL_CONDITION_VARIABLE ) -> ( ) );
    WakeAllConditionVariable(conditionvariable)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn WakeByAddressAll(address: *const ::core::ffi::c_void) {
    ::windows::imp::link ! ( "api-ms-win-core-synch-l1-2-0.dll""system" fn WakeByAddressAll ( address : *const ::core::ffi::c_void ) -> ( ) );
    WakeByAddressAll(address)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn WakeByAddressSingle(address: *const ::core::ffi::c_void) {
    ::windows::imp::link ! ( "api-ms-win-core-synch-l1-2-0.dll""system" fn WakeByAddressSingle ( address : *const ::core::ffi::c_void ) -> ( ) );
    WakeByAddressSingle(address)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn WakeConditionVariable(conditionvariable: *mut RTL_CONDITION_VARIABLE) {
    ::windows::imp::link ! ( "kernel32.dll""system" fn WakeConditionVariable ( conditionvariable : *mut RTL_CONDITION_VARIABLE ) -> ( ) );
    WakeConditionVariable(conditionvariable)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn WinExec<P0>(lpcmdline: P0, ucmdshow: u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn WinExec ( lpcmdline : :: windows::core::PCSTR , ucmdshow : u32 ) -> u32 );
    WinExec(lpcmdline.into_param().abi(), ucmdshow)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn Wow64SetThreadDefaultGuestMachine(machine: u16) -> u16 {
    ::windows::imp::link ! ( "api-ms-win-core-wow64-l1-1-1.dll""system" fn Wow64SetThreadDefaultGuestMachine ( machine : u16 ) -> u16 );
    Wow64SetThreadDefaultGuestMachine(machine)
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn Wow64SuspendThread<P0>(hthread: P0) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn Wow64SuspendThread ( hthread : super::super::Foundation:: HANDLE ) -> u32 );
    Wow64SuspendThread(hthread.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const CONDITION_VARIABLE_LOCKMODE_SHARED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const CREATE_MUTEX_INITIAL_OWNER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const CREATE_WAITABLE_TIMER_HIGH_RESOLUTION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const CREATE_WAITABLE_TIMER_MANUAL_RESET: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const FLS_OUT_OF_INDEXES: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const INIT_ONCE_ASYNC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const INIT_ONCE_CHECK_ONLY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const INIT_ONCE_CTX_RESERVED_BITS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const INIT_ONCE_INIT_FAILED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PME_CURRENT_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PME_FAILFAST_ON_COMMIT_FAIL_DISABLE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PME_FAILFAST_ON_COMMIT_FAIL_ENABLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PRIVATE_NAMESPACE_FLAG_DESTROY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_LEAP_SECOND_INFO_FLAG_ENABLE_SIXTY_SECOND: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_LEAP_SECOND_INFO_VALID_FLAGS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_POWER_THROTTLING_CURRENT_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_POWER_THROTTLING_EXECUTION_SPEED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_POWER_THROTTLING_IGNORE_TIMER_RESOLUTION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROC_THREAD_ATTRIBUTE_ALL_APPLICATION_PACKAGES_POLICY: u32 = 131087u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROC_THREAD_ATTRIBUTE_CHILD_PROCESS_POLICY: u32 = 131086u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROC_THREAD_ATTRIBUTE_COMPONENT_FILTER: u32 = 131098u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROC_THREAD_ATTRIBUTE_DESKTOP_APP_POLICY: u32 = 131090u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROC_THREAD_ATTRIBUTE_ENABLE_OPTIONAL_XSTATE_FEATURES: u32 = 196635u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROC_THREAD_ATTRIBUTE_GROUP_AFFINITY: u32 = 196611u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROC_THREAD_ATTRIBUTE_HANDLE_LIST: u32 = 131074u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROC_THREAD_ATTRIBUTE_IDEAL_PROCESSOR: u32 = 196613u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROC_THREAD_ATTRIBUTE_JOB_LIST: u32 = 131085u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROC_THREAD_ATTRIBUTE_MACHINE_TYPE: u32 = 131097u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROC_THREAD_ATTRIBUTE_MITIGATION_AUDIT_POLICY: u32 = 131096u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROC_THREAD_ATTRIBUTE_MITIGATION_POLICY: u32 = 131079u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROC_THREAD_ATTRIBUTE_PARENT_PROCESS: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROC_THREAD_ATTRIBUTE_PREFERRED_NODE: u32 = 131076u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROC_THREAD_ATTRIBUTE_PROTECTION_LEVEL: u32 = 131083u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROC_THREAD_ATTRIBUTE_PSEUDOCONSOLE: u32 = 131094u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROC_THREAD_ATTRIBUTE_REPLACE_VALUE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROC_THREAD_ATTRIBUTE_SECURITY_CAPABILITIES: u32 = 131081u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROC_THREAD_ATTRIBUTE_UMS_THREAD: u32 = 196614u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROC_THREAD_ATTRIBUTE_WIN32K_FILTER: u32 = 131088u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const SYNCHRONIZATION_BARRIER_FLAGS_BLOCK_ONLY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const SYNCHRONIZATION_BARRIER_FLAGS_NO_DELETE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const SYNCHRONIZATION_BARRIER_FLAGS_SPIN_ONLY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_POWER_THROTTLING_CURRENT_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_POWER_THROTTLING_EXECUTION_SPEED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_POWER_THROTTLING_VALID_FLAGS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const TLS_OUT_OF_INDEXES: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AVRT_PRIORITY(pub i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const AVRT_PRIORITY_VERYLOW: AVRT_PRIORITY = AVRT_PRIORITY(-2i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const AVRT_PRIORITY_LOW: AVRT_PRIORITY = AVRT_PRIORITY(-1i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const AVRT_PRIORITY_NORMAL: AVRT_PRIORITY = AVRT_PRIORITY(0i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const AVRT_PRIORITY_HIGH: AVRT_PRIORITY = AVRT_PRIORITY(1i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const AVRT_PRIORITY_CRITICAL: AVRT_PRIORITY = AVRT_PRIORITY(2i32);
impl ::core::marker::Copy for AVRT_PRIORITY {}
impl ::core::clone::Clone for AVRT_PRIORITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AVRT_PRIORITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for AVRT_PRIORITY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for AVRT_PRIORITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AVRT_PRIORITY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CREATE_EVENT(pub u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const CREATE_EVENT_INITIAL_SET: CREATE_EVENT = CREATE_EVENT(2u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const CREATE_EVENT_MANUAL_RESET: CREATE_EVENT = CREATE_EVENT(1u32);
impl ::core::marker::Copy for CREATE_EVENT {}
impl ::core::clone::Clone for CREATE_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CREATE_EVENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CREATE_EVENT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CREATE_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CREATE_EVENT").field(&self.0).finish()
    }
}
impl CREATE_EVENT {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for CREATE_EVENT {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CREATE_EVENT {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CREATE_EVENT {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CREATE_EVENT {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CREATE_EVENT {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CREATE_PROCESS_LOGON_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const LOGON_WITH_PROFILE: CREATE_PROCESS_LOGON_FLAGS = CREATE_PROCESS_LOGON_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const LOGON_NETCREDENTIALS_ONLY: CREATE_PROCESS_LOGON_FLAGS = CREATE_PROCESS_LOGON_FLAGS(2u32);
impl ::core::marker::Copy for CREATE_PROCESS_LOGON_FLAGS {}
impl ::core::clone::Clone for CREATE_PROCESS_LOGON_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CREATE_PROCESS_LOGON_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CREATE_PROCESS_LOGON_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CREATE_PROCESS_LOGON_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CREATE_PROCESS_LOGON_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GET_GUI_RESOURCES_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const GR_GDIOBJECTS: GET_GUI_RESOURCES_FLAGS = GET_GUI_RESOURCES_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const GR_GDIOBJECTS_PEAK: GET_GUI_RESOURCES_FLAGS = GET_GUI_RESOURCES_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const GR_USEROBJECTS: GET_GUI_RESOURCES_FLAGS = GET_GUI_RESOURCES_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const GR_USEROBJECTS_PEAK: GET_GUI_RESOURCES_FLAGS = GET_GUI_RESOURCES_FLAGS(4u32);
impl ::core::marker::Copy for GET_GUI_RESOURCES_FLAGS {}
impl ::core::clone::Clone for GET_GUI_RESOURCES_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GET_GUI_RESOURCES_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for GET_GUI_RESOURCES_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for GET_GUI_RESOURCES_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GET_GUI_RESOURCES_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MACHINE_ATTRIBUTES(pub i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const UserEnabled: MACHINE_ATTRIBUTES = MACHINE_ATTRIBUTES(1i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const KernelEnabled: MACHINE_ATTRIBUTES = MACHINE_ATTRIBUTES(2i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const Wow64Container: MACHINE_ATTRIBUTES = MACHINE_ATTRIBUTES(4i32);
impl ::core::marker::Copy for MACHINE_ATTRIBUTES {}
impl ::core::clone::Clone for MACHINE_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MACHINE_ATTRIBUTES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MACHINE_ATTRIBUTES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MACHINE_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MACHINE_ATTRIBUTES").field(&self.0).finish()
    }
}
impl MACHINE_ATTRIBUTES {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for MACHINE_ATTRIBUTES {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for MACHINE_ATTRIBUTES {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for MACHINE_ATTRIBUTES {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for MACHINE_ATTRIBUTES {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for MACHINE_ATTRIBUTES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MEMORY_PRIORITY(pub u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const MEMORY_PRIORITY_VERY_LOW: MEMORY_PRIORITY = MEMORY_PRIORITY(1u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const MEMORY_PRIORITY_LOW: MEMORY_PRIORITY = MEMORY_PRIORITY(2u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const MEMORY_PRIORITY_MEDIUM: MEMORY_PRIORITY = MEMORY_PRIORITY(3u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const MEMORY_PRIORITY_BELOW_NORMAL: MEMORY_PRIORITY = MEMORY_PRIORITY(4u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const MEMORY_PRIORITY_NORMAL: MEMORY_PRIORITY = MEMORY_PRIORITY(5u32);
impl ::core::marker::Copy for MEMORY_PRIORITY {}
impl ::core::clone::Clone for MEMORY_PRIORITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MEMORY_PRIORITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MEMORY_PRIORITY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MEMORY_PRIORITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MEMORY_PRIORITY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct POWER_REQUEST_CONTEXT_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const POWER_REQUEST_CONTEXT_DETAILED_STRING: POWER_REQUEST_CONTEXT_FLAGS = POWER_REQUEST_CONTEXT_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const POWER_REQUEST_CONTEXT_SIMPLE_STRING: POWER_REQUEST_CONTEXT_FLAGS = POWER_REQUEST_CONTEXT_FLAGS(1u32);
impl ::core::marker::Copy for POWER_REQUEST_CONTEXT_FLAGS {}
impl ::core::clone::Clone for POWER_REQUEST_CONTEXT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for POWER_REQUEST_CONTEXT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for POWER_REQUEST_CONTEXT_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for POWER_REQUEST_CONTEXT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("POWER_REQUEST_CONTEXT_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROCESSINFOCLASS(pub i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessBasicInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(0i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessQuotaLimits: PROCESSINFOCLASS = PROCESSINFOCLASS(1i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessIoCounters: PROCESSINFOCLASS = PROCESSINFOCLASS(2i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessVmCounters: PROCESSINFOCLASS = PROCESSINFOCLASS(3i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessTimes: PROCESSINFOCLASS = PROCESSINFOCLASS(4i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessBasePriority: PROCESSINFOCLASS = PROCESSINFOCLASS(5i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessRaisePriority: PROCESSINFOCLASS = PROCESSINFOCLASS(6i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessDebugPort: PROCESSINFOCLASS = PROCESSINFOCLASS(7i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessExceptionPort: PROCESSINFOCLASS = PROCESSINFOCLASS(8i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessAccessToken: PROCESSINFOCLASS = PROCESSINFOCLASS(9i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessLdtInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(10i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessLdtSize: PROCESSINFOCLASS = PROCESSINFOCLASS(11i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessDefaultHardErrorMode: PROCESSINFOCLASS = PROCESSINFOCLASS(12i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessIoPortHandlers: PROCESSINFOCLASS = PROCESSINFOCLASS(13i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessPooledUsageAndLimits: PROCESSINFOCLASS = PROCESSINFOCLASS(14i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessWorkingSetWatch: PROCESSINFOCLASS = PROCESSINFOCLASS(15i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessUserModeIOPL: PROCESSINFOCLASS = PROCESSINFOCLASS(16i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessEnableAlignmentFaultFixup: PROCESSINFOCLASS = PROCESSINFOCLASS(17i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessPriorityClass: PROCESSINFOCLASS = PROCESSINFOCLASS(18i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessWx86Information: PROCESSINFOCLASS = PROCESSINFOCLASS(19i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessHandleCount: PROCESSINFOCLASS = PROCESSINFOCLASS(20i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessAffinityMask: PROCESSINFOCLASS = PROCESSINFOCLASS(21i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessPriorityBoost: PROCESSINFOCLASS = PROCESSINFOCLASS(22i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessDeviceMap: PROCESSINFOCLASS = PROCESSINFOCLASS(23i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessSessionInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(24i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessForegroundInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(25i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessWow64Information: PROCESSINFOCLASS = PROCESSINFOCLASS(26i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessImageFileName: PROCESSINFOCLASS = PROCESSINFOCLASS(27i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessLUIDDeviceMapsEnabled: PROCESSINFOCLASS = PROCESSINFOCLASS(28i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessBreakOnTermination: PROCESSINFOCLASS = PROCESSINFOCLASS(29i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessDebugObjectHandle: PROCESSINFOCLASS = PROCESSINFOCLASS(30i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessDebugFlags: PROCESSINFOCLASS = PROCESSINFOCLASS(31i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessHandleTracing: PROCESSINFOCLASS = PROCESSINFOCLASS(32i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessIoPriority: PROCESSINFOCLASS = PROCESSINFOCLASS(33i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessExecuteFlags: PROCESSINFOCLASS = PROCESSINFOCLASS(34i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessTlsInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(35i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessCookie: PROCESSINFOCLASS = PROCESSINFOCLASS(36i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessImageInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(37i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessCycleTime: PROCESSINFOCLASS = PROCESSINFOCLASS(38i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessPagePriority: PROCESSINFOCLASS = PROCESSINFOCLASS(39i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessInstrumentationCallback: PROCESSINFOCLASS = PROCESSINFOCLASS(40i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessThreadStackAllocation: PROCESSINFOCLASS = PROCESSINFOCLASS(41i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessWorkingSetWatchEx: PROCESSINFOCLASS = PROCESSINFOCLASS(42i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessImageFileNameWin32: PROCESSINFOCLASS = PROCESSINFOCLASS(43i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessImageFileMapping: PROCESSINFOCLASS = PROCESSINFOCLASS(44i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessAffinityUpdateMode: PROCESSINFOCLASS = PROCESSINFOCLASS(45i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessMemoryAllocationMode: PROCESSINFOCLASS = PROCESSINFOCLASS(46i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessGroupInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(47i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessTokenVirtualizationEnabled: PROCESSINFOCLASS = PROCESSINFOCLASS(48i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessOwnerInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(49i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessWindowInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(50i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessHandleInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(51i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessMitigationPolicy: PROCESSINFOCLASS = PROCESSINFOCLASS(52i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessDynamicFunctionTableInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(53i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessHandleCheckingMode: PROCESSINFOCLASS = PROCESSINFOCLASS(54i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessKeepAliveCount: PROCESSINFOCLASS = PROCESSINFOCLASS(55i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessRevokeFileHandles: PROCESSINFOCLASS = PROCESSINFOCLASS(56i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessWorkingSetControl: PROCESSINFOCLASS = PROCESSINFOCLASS(57i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessHandleTable: PROCESSINFOCLASS = PROCESSINFOCLASS(58i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessCheckStackExtentsMode: PROCESSINFOCLASS = PROCESSINFOCLASS(59i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessCommandLineInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(60i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessProtectionInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(61i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessMemoryExhaustion: PROCESSINFOCLASS = PROCESSINFOCLASS(62i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessFaultInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(63i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessTelemetryIdInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(64i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessCommitReleaseInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(65i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessReserved1Information: PROCESSINFOCLASS = PROCESSINFOCLASS(66i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessReserved2Information: PROCESSINFOCLASS = PROCESSINFOCLASS(67i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessSubsystemProcess: PROCESSINFOCLASS = PROCESSINFOCLASS(68i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessInPrivate: PROCESSINFOCLASS = PROCESSINFOCLASS(70i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessRaiseUMExceptionOnInvalidHandleClose: PROCESSINFOCLASS = PROCESSINFOCLASS(71i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessSubsystemInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(75i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessWin32kSyscallFilterInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(79i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessEnergyTrackingState: PROCESSINFOCLASS = PROCESSINFOCLASS(82i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const MaxProcessInfoClass: PROCESSINFOCLASS = PROCESSINFOCLASS(83i32);
impl ::core::marker::Copy for PROCESSINFOCLASS {}
impl ::core::clone::Clone for PROCESSINFOCLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROCESSINFOCLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PROCESSINFOCLASS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PROCESSINFOCLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROCESSINFOCLASS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROCESSOR_FEATURE_ID(pub u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_ARM_64BIT_LOADSTORE_ATOMIC: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(25u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_ARM_DIVIDE_INSTRUCTION_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(24u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_ARM_EXTERNAL_CACHE_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(26u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_ARM_FMAC_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(27u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_ARM_VFP_32_REGISTERS_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(18u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_3DNOW_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(7u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_CHANNELS_ENABLED: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(16u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_COMPARE_EXCHANGE_DOUBLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(2u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_COMPARE_EXCHANGE128: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(14u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_COMPARE64_EXCHANGE128: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(15u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_FASTFAIL_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(23u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_FLOATING_POINT_EMULATED: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(1u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_FLOATING_POINT_PRECISION_ERRATA: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(0u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_MMX_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(3u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_NX_ENABLED: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(12u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_PAE_ENABLED: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(9u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_RDTSC_INSTRUCTION_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(8u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_RDWRFSGSBASE_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(22u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_SECOND_LEVEL_ADDRESS_TRANSLATION: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(20u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_SSE3_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(13u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_VIRT_FIRMWARE_ENABLED: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(21u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_XMMI_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(6u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_XMMI64_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(10u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_XSAVE_ENABLED: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(17u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_ARM_V8_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(29u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_ARM_V8_CRYPTO_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(30u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_ARM_V8_CRC32_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(31u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_ARM_V81_ATOMIC_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(34u32);
impl ::core::marker::Copy for PROCESSOR_FEATURE_ID {}
impl ::core::clone::Clone for PROCESSOR_FEATURE_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROCESSOR_FEATURE_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PROCESSOR_FEATURE_ID {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PROCESSOR_FEATURE_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROCESSOR_FEATURE_ID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROCESS_ACCESS_RIGHTS(pub u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_TERMINATE: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(1u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_CREATE_THREAD: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(2u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_SET_SESSIONID: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(4u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_VM_OPERATION: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(8u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_VM_READ: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(16u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_VM_WRITE: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(32u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_DUP_HANDLE: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(64u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_CREATE_PROCESS: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(128u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_SET_QUOTA: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(256u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_SET_INFORMATION: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(512u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_QUERY_INFORMATION: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(1024u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_SUSPEND_RESUME: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(2048u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_QUERY_LIMITED_INFORMATION: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(4096u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_SET_LIMITED_INFORMATION: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(8192u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_ALL_ACCESS: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(2097151u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_DELETE: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(65536u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_READ_CONTROL: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(131072u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_WRITE_DAC: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(262144u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_WRITE_OWNER: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(524288u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_SYNCHRONIZE: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(1048576u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_STANDARD_RIGHTS_REQUIRED: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(983040u32);
impl ::core::marker::Copy for PROCESS_ACCESS_RIGHTS {}
impl ::core::clone::Clone for PROCESS_ACCESS_RIGHTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROCESS_ACCESS_RIGHTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PROCESS_ACCESS_RIGHTS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PROCESS_ACCESS_RIGHTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROCESS_ACCESS_RIGHTS").field(&self.0).finish()
    }
}
impl PROCESS_ACCESS_RIGHTS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for PROCESS_ACCESS_RIGHTS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PROCESS_ACCESS_RIGHTS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PROCESS_ACCESS_RIGHTS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PROCESS_ACCESS_RIGHTS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PROCESS_ACCESS_RIGHTS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROCESS_AFFINITY_AUTO_UPDATE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_AFFINITY_DISABLE_AUTO_UPDATE: PROCESS_AFFINITY_AUTO_UPDATE_FLAGS = PROCESS_AFFINITY_AUTO_UPDATE_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_AFFINITY_ENABLE_AUTO_UPDATE: PROCESS_AFFINITY_AUTO_UPDATE_FLAGS = PROCESS_AFFINITY_AUTO_UPDATE_FLAGS(1u32);
impl ::core::marker::Copy for PROCESS_AFFINITY_AUTO_UPDATE_FLAGS {}
impl ::core::clone::Clone for PROCESS_AFFINITY_AUTO_UPDATE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROCESS_AFFINITY_AUTO_UPDATE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PROCESS_AFFINITY_AUTO_UPDATE_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PROCESS_AFFINITY_AUTO_UPDATE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROCESS_AFFINITY_AUTO_UPDATE_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROCESS_CREATION_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const DEBUG_PROCESS: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const DEBUG_ONLY_THIS_PROCESS: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const CREATE_SUSPENDED: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const DETACHED_PROCESS: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const CREATE_NEW_CONSOLE: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const NORMAL_PRIORITY_CLASS: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const IDLE_PRIORITY_CLASS: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const HIGH_PRIORITY_CLASS: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(128u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const REALTIME_PRIORITY_CLASS: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(256u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const CREATE_NEW_PROCESS_GROUP: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(512u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const CREATE_UNICODE_ENVIRONMENT: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(1024u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const CREATE_SEPARATE_WOW_VDM: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(2048u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const CREATE_SHARED_WOW_VDM: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(4096u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const CREATE_FORCEDOS: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(8192u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const BELOW_NORMAL_PRIORITY_CLASS: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(16384u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ABOVE_NORMAL_PRIORITY_CLASS: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(32768u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const INHERIT_PARENT_AFFINITY: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(65536u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const INHERIT_CALLER_PRIORITY: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(131072u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const CREATE_PROTECTED_PROCESS: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(262144u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const EXTENDED_STARTUPINFO_PRESENT: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(524288u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_MODE_BACKGROUND_BEGIN: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(1048576u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_MODE_BACKGROUND_END: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(2097152u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const CREATE_SECURE_PROCESS: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(4194304u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const CREATE_BREAKAWAY_FROM_JOB: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(16777216u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const CREATE_PRESERVE_CODE_AUTHZ_LEVEL: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(33554432u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const CREATE_DEFAULT_ERROR_MODE: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(67108864u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const CREATE_NO_WINDOW: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(134217728u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROFILE_USER: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(268435456u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROFILE_KERNEL: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(536870912u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROFILE_SERVER: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(1073741824u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const CREATE_IGNORE_SYSTEM_DEFAULT: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(2147483648u32);
impl ::core::marker::Copy for PROCESS_CREATION_FLAGS {}
impl ::core::clone::Clone for PROCESS_CREATION_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROCESS_CREATION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PROCESS_CREATION_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PROCESS_CREATION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROCESS_CREATION_FLAGS").field(&self.0).finish()
    }
}
impl PROCESS_CREATION_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for PROCESS_CREATION_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PROCESS_CREATION_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PROCESS_CREATION_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PROCESS_CREATION_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PROCESS_CREATION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROCESS_DEP_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_DEP_ENABLE: PROCESS_DEP_FLAGS = PROCESS_DEP_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_DEP_DISABLE_ATL_THUNK_EMULATION: PROCESS_DEP_FLAGS = PROCESS_DEP_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_DEP_NONE: PROCESS_DEP_FLAGS = PROCESS_DEP_FLAGS(0u32);
impl ::core::marker::Copy for PROCESS_DEP_FLAGS {}
impl ::core::clone::Clone for PROCESS_DEP_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROCESS_DEP_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PROCESS_DEP_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PROCESS_DEP_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROCESS_DEP_FLAGS").field(&self.0).finish()
    }
}
impl PROCESS_DEP_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for PROCESS_DEP_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PROCESS_DEP_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PROCESS_DEP_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PROCESS_DEP_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PROCESS_DEP_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROCESS_INFORMATION_CLASS(pub i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessMemoryPriority: PROCESS_INFORMATION_CLASS = PROCESS_INFORMATION_CLASS(0i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessMemoryExhaustionInfo: PROCESS_INFORMATION_CLASS = PROCESS_INFORMATION_CLASS(1i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessAppMemoryInfo: PROCESS_INFORMATION_CLASS = PROCESS_INFORMATION_CLASS(2i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessInPrivateInfo: PROCESS_INFORMATION_CLASS = PROCESS_INFORMATION_CLASS(3i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessPowerThrottling: PROCESS_INFORMATION_CLASS = PROCESS_INFORMATION_CLASS(4i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessReservedValue1: PROCESS_INFORMATION_CLASS = PROCESS_INFORMATION_CLASS(5i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessTelemetryCoverageInfo: PROCESS_INFORMATION_CLASS = PROCESS_INFORMATION_CLASS(6i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessProtectionLevelInfo: PROCESS_INFORMATION_CLASS = PROCESS_INFORMATION_CLASS(7i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessLeapSecondInfo: PROCESS_INFORMATION_CLASS = PROCESS_INFORMATION_CLASS(8i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessMachineTypeInfo: PROCESS_INFORMATION_CLASS = PROCESS_INFORMATION_CLASS(9i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessInformationClassMax: PROCESS_INFORMATION_CLASS = PROCESS_INFORMATION_CLASS(10i32);
impl ::core::marker::Copy for PROCESS_INFORMATION_CLASS {}
impl ::core::clone::Clone for PROCESS_INFORMATION_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROCESS_INFORMATION_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PROCESS_INFORMATION_CLASS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PROCESS_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROCESS_INFORMATION_CLASS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROCESS_MEMORY_EXHAUSTION_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PMETypeFailFastOnCommitFailure: PROCESS_MEMORY_EXHAUSTION_TYPE = PROCESS_MEMORY_EXHAUSTION_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PMETypeMax: PROCESS_MEMORY_EXHAUSTION_TYPE = PROCESS_MEMORY_EXHAUSTION_TYPE(1i32);
impl ::core::marker::Copy for PROCESS_MEMORY_EXHAUSTION_TYPE {}
impl ::core::clone::Clone for PROCESS_MEMORY_EXHAUSTION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROCESS_MEMORY_EXHAUSTION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PROCESS_MEMORY_EXHAUSTION_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PROCESS_MEMORY_EXHAUSTION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROCESS_MEMORY_EXHAUSTION_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROCESS_MITIGATION_POLICY(pub i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessDEPPolicy: PROCESS_MITIGATION_POLICY = PROCESS_MITIGATION_POLICY(0i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessASLRPolicy: PROCESS_MITIGATION_POLICY = PROCESS_MITIGATION_POLICY(1i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessDynamicCodePolicy: PROCESS_MITIGATION_POLICY = PROCESS_MITIGATION_POLICY(2i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessStrictHandleCheckPolicy: PROCESS_MITIGATION_POLICY = PROCESS_MITIGATION_POLICY(3i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessSystemCallDisablePolicy: PROCESS_MITIGATION_POLICY = PROCESS_MITIGATION_POLICY(4i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessMitigationOptionsMask: PROCESS_MITIGATION_POLICY = PROCESS_MITIGATION_POLICY(5i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessExtensionPointDisablePolicy: PROCESS_MITIGATION_POLICY = PROCESS_MITIGATION_POLICY(6i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessControlFlowGuardPolicy: PROCESS_MITIGATION_POLICY = PROCESS_MITIGATION_POLICY(7i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessSignaturePolicy: PROCESS_MITIGATION_POLICY = PROCESS_MITIGATION_POLICY(8i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessFontDisablePolicy: PROCESS_MITIGATION_POLICY = PROCESS_MITIGATION_POLICY(9i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessImageLoadPolicy: PROCESS_MITIGATION_POLICY = PROCESS_MITIGATION_POLICY(10i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessSystemCallFilterPolicy: PROCESS_MITIGATION_POLICY = PROCESS_MITIGATION_POLICY(11i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessPayloadRestrictionPolicy: PROCESS_MITIGATION_POLICY = PROCESS_MITIGATION_POLICY(12i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessChildProcessPolicy: PROCESS_MITIGATION_POLICY = PROCESS_MITIGATION_POLICY(13i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessSideChannelIsolationPolicy: PROCESS_MITIGATION_POLICY = PROCESS_MITIGATION_POLICY(14i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessUserShadowStackPolicy: PROCESS_MITIGATION_POLICY = PROCESS_MITIGATION_POLICY(15i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessRedirectionTrustPolicy: PROCESS_MITIGATION_POLICY = PROCESS_MITIGATION_POLICY(16i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const MaxProcessMitigationPolicy: PROCESS_MITIGATION_POLICY = PROCESS_MITIGATION_POLICY(17i32);
impl ::core::marker::Copy for PROCESS_MITIGATION_POLICY {}
impl ::core::clone::Clone for PROCESS_MITIGATION_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROCESS_MITIGATION_POLICY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PROCESS_MITIGATION_POLICY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PROCESS_MITIGATION_POLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROCESS_MITIGATION_POLICY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROCESS_NAME_FORMAT(pub u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_NAME_WIN32: PROCESS_NAME_FORMAT = PROCESS_NAME_FORMAT(0u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_NAME_NATIVE: PROCESS_NAME_FORMAT = PROCESS_NAME_FORMAT(1u32);
impl ::core::marker::Copy for PROCESS_NAME_FORMAT {}
impl ::core::clone::Clone for PROCESS_NAME_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROCESS_NAME_FORMAT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PROCESS_NAME_FORMAT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PROCESS_NAME_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROCESS_NAME_FORMAT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROCESS_PROTECTION_LEVEL(pub u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROTECTION_LEVEL_WINTCB_LIGHT: PROCESS_PROTECTION_LEVEL = PROCESS_PROTECTION_LEVEL(0u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROTECTION_LEVEL_WINDOWS: PROCESS_PROTECTION_LEVEL = PROCESS_PROTECTION_LEVEL(1u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROTECTION_LEVEL_WINDOWS_LIGHT: PROCESS_PROTECTION_LEVEL = PROCESS_PROTECTION_LEVEL(2u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROTECTION_LEVEL_ANTIMALWARE_LIGHT: PROCESS_PROTECTION_LEVEL = PROCESS_PROTECTION_LEVEL(3u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROTECTION_LEVEL_LSA_LIGHT: PROCESS_PROTECTION_LEVEL = PROCESS_PROTECTION_LEVEL(4u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROTECTION_LEVEL_WINTCB: PROCESS_PROTECTION_LEVEL = PROCESS_PROTECTION_LEVEL(5u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROTECTION_LEVEL_CODEGEN_LIGHT: PROCESS_PROTECTION_LEVEL = PROCESS_PROTECTION_LEVEL(6u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROTECTION_LEVEL_AUTHENTICODE: PROCESS_PROTECTION_LEVEL = PROCESS_PROTECTION_LEVEL(7u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROTECTION_LEVEL_PPL_APP: PROCESS_PROTECTION_LEVEL = PROCESS_PROTECTION_LEVEL(8u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROTECTION_LEVEL_NONE: PROCESS_PROTECTION_LEVEL = PROCESS_PROTECTION_LEVEL(4294967294u32);
impl ::core::marker::Copy for PROCESS_PROTECTION_LEVEL {}
impl ::core::clone::Clone for PROCESS_PROTECTION_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROCESS_PROTECTION_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PROCESS_PROTECTION_LEVEL {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PROCESS_PROTECTION_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROCESS_PROTECTION_LEVEL").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROC_THREAD_ATTRIBUTE_NUM(pub u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcThreadAttributeParentProcess: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(0u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcThreadAttributeHandleList: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(2u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcThreadAttributeGroupAffinity: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(3u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcThreadAttributePreferredNode: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(4u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcThreadAttributeIdealProcessor: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(5u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcThreadAttributeUmsThread: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(6u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcThreadAttributeMitigationPolicy: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(7u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcThreadAttributeSecurityCapabilities: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(9u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcThreadAttributeProtectionLevel: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(11u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcThreadAttributeJobList: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(13u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcThreadAttributeChildProcessPolicy: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(14u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcThreadAttributeAllApplicationPackagesPolicy: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(15u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcThreadAttributeWin32kFilter: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(16u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcThreadAttributeSafeOpenPromptOriginClaim: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(17u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcThreadAttributeDesktopAppPolicy: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(18u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcThreadAttributePseudoConsole: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(22u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcThreadAttributeMitigationAuditPolicy: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(24u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcThreadAttributeMachineType: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(25u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcThreadAttributeComponentFilter: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(26u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcThreadAttributeEnableOptionalXStateFeatures: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(27u32);
impl ::core::marker::Copy for PROC_THREAD_ATTRIBUTE_NUM {}
impl ::core::clone::Clone for PROC_THREAD_ATTRIBUTE_NUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROC_THREAD_ATTRIBUTE_NUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PROC_THREAD_ATTRIBUTE_NUM {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PROC_THREAD_ATTRIBUTE_NUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROC_THREAD_ATTRIBUTE_NUM").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct QUEUE_USER_APC_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const QUEUE_USER_APC_FLAGS_NONE: QUEUE_USER_APC_FLAGS = QUEUE_USER_APC_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const QUEUE_USER_APC_FLAGS_SPECIAL_USER_APC: QUEUE_USER_APC_FLAGS = QUEUE_USER_APC_FLAGS(1i32);
impl ::core::marker::Copy for QUEUE_USER_APC_FLAGS {}
impl ::core::clone::Clone for QUEUE_USER_APC_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for QUEUE_USER_APC_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for QUEUE_USER_APC_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for QUEUE_USER_APC_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QUEUE_USER_APC_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RTL_UMS_THREAD_INFO_CLASS(pub i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const UmsThreadInvalidInfoClass: RTL_UMS_THREAD_INFO_CLASS = RTL_UMS_THREAD_INFO_CLASS(0i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const UmsThreadUserContext: RTL_UMS_THREAD_INFO_CLASS = RTL_UMS_THREAD_INFO_CLASS(1i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const UmsThreadPriority: RTL_UMS_THREAD_INFO_CLASS = RTL_UMS_THREAD_INFO_CLASS(2i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const UmsThreadAffinity: RTL_UMS_THREAD_INFO_CLASS = RTL_UMS_THREAD_INFO_CLASS(3i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const UmsThreadTeb: RTL_UMS_THREAD_INFO_CLASS = RTL_UMS_THREAD_INFO_CLASS(4i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const UmsThreadIsSuspended: RTL_UMS_THREAD_INFO_CLASS = RTL_UMS_THREAD_INFO_CLASS(5i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const UmsThreadIsTerminated: RTL_UMS_THREAD_INFO_CLASS = RTL_UMS_THREAD_INFO_CLASS(6i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const UmsThreadMaxInfoClass: RTL_UMS_THREAD_INFO_CLASS = RTL_UMS_THREAD_INFO_CLASS(7i32);
impl ::core::marker::Copy for RTL_UMS_THREAD_INFO_CLASS {}
impl ::core::clone::Clone for RTL_UMS_THREAD_INFO_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTL_UMS_THREAD_INFO_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for RTL_UMS_THREAD_INFO_CLASS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for RTL_UMS_THREAD_INFO_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTL_UMS_THREAD_INFO_CLASS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct STARTUPINFOW_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const STARTF_FORCEONFEEDBACK: STARTUPINFOW_FLAGS = STARTUPINFOW_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const STARTF_FORCEOFFFEEDBACK: STARTUPINFOW_FLAGS = STARTUPINFOW_FLAGS(128u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const STARTF_PREVENTPINNING: STARTUPINFOW_FLAGS = STARTUPINFOW_FLAGS(8192u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const STARTF_RUNFULLSCREEN: STARTUPINFOW_FLAGS = STARTUPINFOW_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const STARTF_TITLEISAPPID: STARTUPINFOW_FLAGS = STARTUPINFOW_FLAGS(4096u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const STARTF_TITLEISLINKNAME: STARTUPINFOW_FLAGS = STARTUPINFOW_FLAGS(2048u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const STARTF_UNTRUSTEDSOURCE: STARTUPINFOW_FLAGS = STARTUPINFOW_FLAGS(32768u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const STARTF_USECOUNTCHARS: STARTUPINFOW_FLAGS = STARTUPINFOW_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const STARTF_USEFILLATTRIBUTE: STARTUPINFOW_FLAGS = STARTUPINFOW_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const STARTF_USEHOTKEY: STARTUPINFOW_FLAGS = STARTUPINFOW_FLAGS(512u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const STARTF_USEPOSITION: STARTUPINFOW_FLAGS = STARTUPINFOW_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const STARTF_USESHOWWINDOW: STARTUPINFOW_FLAGS = STARTUPINFOW_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const STARTF_USESIZE: STARTUPINFOW_FLAGS = STARTUPINFOW_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const STARTF_USESTDHANDLES: STARTUPINFOW_FLAGS = STARTUPINFOW_FLAGS(256u32);
impl ::core::marker::Copy for STARTUPINFOW_FLAGS {}
impl ::core::clone::Clone for STARTUPINFOW_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STARTUPINFOW_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for STARTUPINFOW_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for STARTUPINFOW_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STARTUPINFOW_FLAGS").field(&self.0).finish()
    }
}
impl STARTUPINFOW_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for STARTUPINFOW_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for STARTUPINFOW_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for STARTUPINFOW_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for STARTUPINFOW_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for STARTUPINFOW_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SYNCHRONIZATION_ACCESS_RIGHTS(pub u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const EVENT_ALL_ACCESS: SYNCHRONIZATION_ACCESS_RIGHTS = SYNCHRONIZATION_ACCESS_RIGHTS(2031619u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const EVENT_MODIFY_STATE: SYNCHRONIZATION_ACCESS_RIGHTS = SYNCHRONIZATION_ACCESS_RIGHTS(2u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const MUTEX_ALL_ACCESS: SYNCHRONIZATION_ACCESS_RIGHTS = SYNCHRONIZATION_ACCESS_RIGHTS(2031617u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const MUTEX_MODIFY_STATE: SYNCHRONIZATION_ACCESS_RIGHTS = SYNCHRONIZATION_ACCESS_RIGHTS(1u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const SEMAPHORE_ALL_ACCESS: SYNCHRONIZATION_ACCESS_RIGHTS = SYNCHRONIZATION_ACCESS_RIGHTS(2031619u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const SEMAPHORE_MODIFY_STATE: SYNCHRONIZATION_ACCESS_RIGHTS = SYNCHRONIZATION_ACCESS_RIGHTS(2u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const TIMER_ALL_ACCESS: SYNCHRONIZATION_ACCESS_RIGHTS = SYNCHRONIZATION_ACCESS_RIGHTS(2031619u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const TIMER_MODIFY_STATE: SYNCHRONIZATION_ACCESS_RIGHTS = SYNCHRONIZATION_ACCESS_RIGHTS(2u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const TIMER_QUERY_STATE: SYNCHRONIZATION_ACCESS_RIGHTS = SYNCHRONIZATION_ACCESS_RIGHTS(1u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const SYNCHRONIZATION_DELETE: SYNCHRONIZATION_ACCESS_RIGHTS = SYNCHRONIZATION_ACCESS_RIGHTS(65536u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const SYNCHRONIZATION_READ_CONTROL: SYNCHRONIZATION_ACCESS_RIGHTS = SYNCHRONIZATION_ACCESS_RIGHTS(131072u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const SYNCHRONIZATION_WRITE_DAC: SYNCHRONIZATION_ACCESS_RIGHTS = SYNCHRONIZATION_ACCESS_RIGHTS(262144u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const SYNCHRONIZATION_WRITE_OWNER: SYNCHRONIZATION_ACCESS_RIGHTS = SYNCHRONIZATION_ACCESS_RIGHTS(524288u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const SYNCHRONIZATION_SYNCHRONIZE: SYNCHRONIZATION_ACCESS_RIGHTS = SYNCHRONIZATION_ACCESS_RIGHTS(1048576u32);
impl ::core::marker::Copy for SYNCHRONIZATION_ACCESS_RIGHTS {}
impl ::core::clone::Clone for SYNCHRONIZATION_ACCESS_RIGHTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SYNCHRONIZATION_ACCESS_RIGHTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SYNCHRONIZATION_ACCESS_RIGHTS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SYNCHRONIZATION_ACCESS_RIGHTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYNCHRONIZATION_ACCESS_RIGHTS").field(&self.0).finish()
    }
}
impl SYNCHRONIZATION_ACCESS_RIGHTS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for SYNCHRONIZATION_ACCESS_RIGHTS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SYNCHRONIZATION_ACCESS_RIGHTS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SYNCHRONIZATION_ACCESS_RIGHTS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SYNCHRONIZATION_ACCESS_RIGHTS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SYNCHRONIZATION_ACCESS_RIGHTS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct THREADINFOCLASS(pub i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadBasicInformation: THREADINFOCLASS = THREADINFOCLASS(0i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadTimes: THREADINFOCLASS = THREADINFOCLASS(1i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadPriority: THREADINFOCLASS = THREADINFOCLASS(2i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadBasePriority: THREADINFOCLASS = THREADINFOCLASS(3i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadAffinityMask: THREADINFOCLASS = THREADINFOCLASS(4i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadImpersonationToken: THREADINFOCLASS = THREADINFOCLASS(5i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadDescriptorTableEntry: THREADINFOCLASS = THREADINFOCLASS(6i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadEnableAlignmentFaultFixup: THREADINFOCLASS = THREADINFOCLASS(7i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadEventPair_Reusable: THREADINFOCLASS = THREADINFOCLASS(8i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadQuerySetWin32StartAddress: THREADINFOCLASS = THREADINFOCLASS(9i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadZeroTlsCell: THREADINFOCLASS = THREADINFOCLASS(10i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadPerformanceCount: THREADINFOCLASS = THREADINFOCLASS(11i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadAmILastThread: THREADINFOCLASS = THREADINFOCLASS(12i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadIdealProcessor: THREADINFOCLASS = THREADINFOCLASS(13i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadPriorityBoost: THREADINFOCLASS = THREADINFOCLASS(14i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadSetTlsArrayAddress: THREADINFOCLASS = THREADINFOCLASS(15i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadIsIoPending: THREADINFOCLASS = THREADINFOCLASS(16i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadHideFromDebugger: THREADINFOCLASS = THREADINFOCLASS(17i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadBreakOnTermination: THREADINFOCLASS = THREADINFOCLASS(18i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadSwitchLegacyState: THREADINFOCLASS = THREADINFOCLASS(19i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadIsTerminated: THREADINFOCLASS = THREADINFOCLASS(20i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadLastSystemCall: THREADINFOCLASS = THREADINFOCLASS(21i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadIoPriority: THREADINFOCLASS = THREADINFOCLASS(22i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadCycleTime: THREADINFOCLASS = THREADINFOCLASS(23i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadPagePriority: THREADINFOCLASS = THREADINFOCLASS(24i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadActualBasePriority: THREADINFOCLASS = THREADINFOCLASS(25i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadTebInformation: THREADINFOCLASS = THREADINFOCLASS(26i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadCSwitchMon: THREADINFOCLASS = THREADINFOCLASS(27i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadCSwitchPmu: THREADINFOCLASS = THREADINFOCLASS(28i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadWow64Context: THREADINFOCLASS = THREADINFOCLASS(29i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadGroupInformation: THREADINFOCLASS = THREADINFOCLASS(30i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadUmsInformation: THREADINFOCLASS = THREADINFOCLASS(31i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadCounterProfiling: THREADINFOCLASS = THREADINFOCLASS(32i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadIdealProcessorEx: THREADINFOCLASS = THREADINFOCLASS(33i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadCpuAccountingInformation: THREADINFOCLASS = THREADINFOCLASS(34i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadSuspendCount: THREADINFOCLASS = THREADINFOCLASS(35i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadActualGroupAffinity: THREADINFOCLASS = THREADINFOCLASS(41i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadDynamicCodePolicyInfo: THREADINFOCLASS = THREADINFOCLASS(42i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadSubsystemInformation: THREADINFOCLASS = THREADINFOCLASS(45i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const MaxThreadInfoClass: THREADINFOCLASS = THREADINFOCLASS(53i32);
impl ::core::marker::Copy for THREADINFOCLASS {}
impl ::core::clone::Clone for THREADINFOCLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for THREADINFOCLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for THREADINFOCLASS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for THREADINFOCLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("THREADINFOCLASS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct THREAD_ACCESS_RIGHTS(pub u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_TERMINATE: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(1u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_SUSPEND_RESUME: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(2u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_GET_CONTEXT: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(8u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_SET_CONTEXT: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(16u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_SET_INFORMATION: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(32u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_QUERY_INFORMATION: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(64u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_SET_THREAD_TOKEN: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(128u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_IMPERSONATE: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(256u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_DIRECT_IMPERSONATION: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(512u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_SET_LIMITED_INFORMATION: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(1024u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_QUERY_LIMITED_INFORMATION: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(2048u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_RESUME: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(4096u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_ALL_ACCESS: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(2097151u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_DELETE: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(65536u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_READ_CONTROL: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(131072u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_WRITE_DAC: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(262144u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_WRITE_OWNER: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(524288u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_SYNCHRONIZE: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(1048576u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_STANDARD_RIGHTS_REQUIRED: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(983040u32);
impl ::core::marker::Copy for THREAD_ACCESS_RIGHTS {}
impl ::core::clone::Clone for THREAD_ACCESS_RIGHTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for THREAD_ACCESS_RIGHTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for THREAD_ACCESS_RIGHTS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for THREAD_ACCESS_RIGHTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("THREAD_ACCESS_RIGHTS").field(&self.0).finish()
    }
}
impl THREAD_ACCESS_RIGHTS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for THREAD_ACCESS_RIGHTS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for THREAD_ACCESS_RIGHTS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for THREAD_ACCESS_RIGHTS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for THREAD_ACCESS_RIGHTS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for THREAD_ACCESS_RIGHTS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct THREAD_CREATION_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_CREATE_RUN_IMMEDIATELY: THREAD_CREATION_FLAGS = THREAD_CREATION_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_CREATE_SUSPENDED: THREAD_CREATION_FLAGS = THREAD_CREATION_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const STACK_SIZE_PARAM_IS_A_RESERVATION: THREAD_CREATION_FLAGS = THREAD_CREATION_FLAGS(65536u32);
impl ::core::marker::Copy for THREAD_CREATION_FLAGS {}
impl ::core::clone::Clone for THREAD_CREATION_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for THREAD_CREATION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for THREAD_CREATION_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for THREAD_CREATION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("THREAD_CREATION_FLAGS").field(&self.0).finish()
    }
}
impl THREAD_CREATION_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for THREAD_CREATION_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for THREAD_CREATION_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for THREAD_CREATION_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for THREAD_CREATION_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for THREAD_CREATION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct THREAD_INFORMATION_CLASS(pub i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadMemoryPriority: THREAD_INFORMATION_CLASS = THREAD_INFORMATION_CLASS(0i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadAbsoluteCpuPriority: THREAD_INFORMATION_CLASS = THREAD_INFORMATION_CLASS(1i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadDynamicCodePolicy: THREAD_INFORMATION_CLASS = THREAD_INFORMATION_CLASS(2i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadPowerThrottling: THREAD_INFORMATION_CLASS = THREAD_INFORMATION_CLASS(3i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadInformationClassMax: THREAD_INFORMATION_CLASS = THREAD_INFORMATION_CLASS(4i32);
impl ::core::marker::Copy for THREAD_INFORMATION_CLASS {}
impl ::core::clone::Clone for THREAD_INFORMATION_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for THREAD_INFORMATION_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for THREAD_INFORMATION_CLASS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for THREAD_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("THREAD_INFORMATION_CLASS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct THREAD_PRIORITY(pub i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_MODE_BACKGROUND_BEGIN: THREAD_PRIORITY = THREAD_PRIORITY(65536i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_MODE_BACKGROUND_END: THREAD_PRIORITY = THREAD_PRIORITY(131072i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_PRIORITY_ABOVE_NORMAL: THREAD_PRIORITY = THREAD_PRIORITY(1i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_PRIORITY_BELOW_NORMAL: THREAD_PRIORITY = THREAD_PRIORITY(-1i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_PRIORITY_HIGHEST: THREAD_PRIORITY = THREAD_PRIORITY(2i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_PRIORITY_IDLE: THREAD_PRIORITY = THREAD_PRIORITY(-15i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_PRIORITY_MIN: THREAD_PRIORITY = THREAD_PRIORITY(-2i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_PRIORITY_LOWEST: THREAD_PRIORITY = THREAD_PRIORITY(-2i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_PRIORITY_NORMAL: THREAD_PRIORITY = THREAD_PRIORITY(0i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_PRIORITY_TIME_CRITICAL: THREAD_PRIORITY = THREAD_PRIORITY(15i32);
impl ::core::marker::Copy for THREAD_PRIORITY {}
impl ::core::clone::Clone for THREAD_PRIORITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for THREAD_PRIORITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for THREAD_PRIORITY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for THREAD_PRIORITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("THREAD_PRIORITY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TP_CALLBACK_PRIORITY(pub i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const TP_CALLBACK_PRIORITY_HIGH: TP_CALLBACK_PRIORITY = TP_CALLBACK_PRIORITY(0i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const TP_CALLBACK_PRIORITY_NORMAL: TP_CALLBACK_PRIORITY = TP_CALLBACK_PRIORITY(1i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const TP_CALLBACK_PRIORITY_LOW: TP_CALLBACK_PRIORITY = TP_CALLBACK_PRIORITY(2i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const TP_CALLBACK_PRIORITY_INVALID: TP_CALLBACK_PRIORITY = TP_CALLBACK_PRIORITY(3i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const TP_CALLBACK_PRIORITY_COUNT: TP_CALLBACK_PRIORITY = TP_CALLBACK_PRIORITY(3i32);
impl ::core::marker::Copy for TP_CALLBACK_PRIORITY {}
impl ::core::clone::Clone for TP_CALLBACK_PRIORITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TP_CALLBACK_PRIORITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for TP_CALLBACK_PRIORITY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for TP_CALLBACK_PRIORITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TP_CALLBACK_PRIORITY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WORKER_THREAD_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const WT_EXECUTEDEFAULT: WORKER_THREAD_FLAGS = WORKER_THREAD_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const WT_EXECUTEINIOTHREAD: WORKER_THREAD_FLAGS = WORKER_THREAD_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const WT_EXECUTEINPERSISTENTTHREAD: WORKER_THREAD_FLAGS = WORKER_THREAD_FLAGS(128u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const WT_EXECUTEINWAITTHREAD: WORKER_THREAD_FLAGS = WORKER_THREAD_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const WT_EXECUTELONGFUNCTION: WORKER_THREAD_FLAGS = WORKER_THREAD_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const WT_EXECUTEONLYONCE: WORKER_THREAD_FLAGS = WORKER_THREAD_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const WT_TRANSFER_IMPERSONATION: WORKER_THREAD_FLAGS = WORKER_THREAD_FLAGS(256u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const WT_EXECUTEINTIMERTHREAD: WORKER_THREAD_FLAGS = WORKER_THREAD_FLAGS(32u32);
impl ::core::marker::Copy for WORKER_THREAD_FLAGS {}
impl ::core::clone::Clone for WORKER_THREAD_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WORKER_THREAD_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WORKER_THREAD_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WORKER_THREAD_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WORKER_THREAD_FLAGS").field(&self.0).finish()
    }
}
impl WORKER_THREAD_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for WORKER_THREAD_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for WORKER_THREAD_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for WORKER_THREAD_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for WORKER_THREAD_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for WORKER_THREAD_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub struct APP_MEMORY_INFORMATION {
    pub AvailableCommit: u64,
    pub PrivateCommitUsage: u64,
    pub PeakPrivateCommitUsage: u64,
    pub TotalCommitUsage: u64,
}
impl ::core::marker::Copy for APP_MEMORY_INFORMATION {}
impl ::core::clone::Clone for APP_MEMORY_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for APP_MEMORY_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("APP_MEMORY_INFORMATION").field("AvailableCommit", &self.AvailableCommit).field("PrivateCommitUsage", &self.PrivateCommitUsage).field("PeakPrivateCommitUsage", &self.PeakPrivateCommitUsage).field("TotalCommitUsage", &self.TotalCommitUsage).finish()
    }
}
impl ::windows::core::TypeKind for APP_MEMORY_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for APP_MEMORY_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.AvailableCommit == other.AvailableCommit && self.PrivateCommitUsage == other.PrivateCommitUsage && self.PeakPrivateCommitUsage == other.PeakPrivateCommitUsage && self.TotalCommitUsage == other.TotalCommitUsage
    }
}
impl ::core::cmp::Eq for APP_MEMORY_INFORMATION {}
impl ::core::default::Default for APP_MEMORY_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BoundaryDescriptorHandle(pub isize);
impl BoundaryDescriptorHandle {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for BoundaryDescriptorHandle {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for BoundaryDescriptorHandle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for BoundaryDescriptorHandle {}
impl ::core::fmt::Debug for BoundaryDescriptorHandle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BoundaryDescriptorHandle").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for BoundaryDescriptorHandle {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub struct IO_COUNTERS {
    pub ReadOperationCount: u64,
    pub WriteOperationCount: u64,
    pub OtherOperationCount: u64,
    pub ReadTransferCount: u64,
    pub WriteTransferCount: u64,
    pub OtherTransferCount: u64,
}
impl ::core::marker::Copy for IO_COUNTERS {}
impl ::core::clone::Clone for IO_COUNTERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IO_COUNTERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IO_COUNTERS").field("ReadOperationCount", &self.ReadOperationCount).field("WriteOperationCount", &self.WriteOperationCount).field("OtherOperationCount", &self.OtherOperationCount).field("ReadTransferCount", &self.ReadTransferCount).field("WriteTransferCount", &self.WriteTransferCount).field("OtherTransferCount", &self.OtherTransferCount).finish()
    }
}
impl ::windows::core::TypeKind for IO_COUNTERS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for IO_COUNTERS {
    fn eq(&self, other: &Self) -> bool {
        self.ReadOperationCount == other.ReadOperationCount && self.WriteOperationCount == other.WriteOperationCount && self.OtherOperationCount == other.OtherOperationCount && self.ReadTransferCount == other.ReadTransferCount && self.WriteTransferCount == other.WriteTransferCount && self.OtherTransferCount == other.OtherTransferCount
    }
}
impl ::core::cmp::Eq for IO_COUNTERS {}
impl ::core::default::Default for IO_COUNTERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LPPROC_THREAD_ATTRIBUTE_LIST(pub *mut ::core::ffi::c_void);
impl LPPROC_THREAD_ATTRIBUTE_LIST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl ::core::default::Default for LPPROC_THREAD_ATTRIBUTE_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for LPPROC_THREAD_ATTRIBUTE_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for LPPROC_THREAD_ATTRIBUTE_LIST {}
impl ::core::fmt::Debug for LPPROC_THREAD_ATTRIBUTE_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LPPROC_THREAD_ATTRIBUTE_LIST").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for LPPROC_THREAD_ATTRIBUTE_LIST {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub struct MEMORY_PRIORITY_INFORMATION {
    pub MemoryPriority: MEMORY_PRIORITY,
}
impl ::core::marker::Copy for MEMORY_PRIORITY_INFORMATION {}
impl ::core::clone::Clone for MEMORY_PRIORITY_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MEMORY_PRIORITY_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MEMORY_PRIORITY_INFORMATION").field("MemoryPriority", &self.MemoryPriority).finish()
    }
}
impl ::windows::core::TypeKind for MEMORY_PRIORITY_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MEMORY_PRIORITY_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.MemoryPriority == other.MemoryPriority
    }
}
impl ::core::cmp::Eq for MEMORY_PRIORITY_INFORMATION {}
impl ::core::default::Default for MEMORY_PRIORITY_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NamespaceHandle(pub isize);
impl NamespaceHandle {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for NamespaceHandle {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for NamespaceHandle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for NamespaceHandle {}
impl ::core::fmt::Debug for NamespaceHandle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NamespaceHandle").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for NamespaceHandle {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub struct PEB {
    pub Reserved1: [u8; 2],
    pub BeingDebugged: u8,
    pub Reserved2: [u8; 1],
    pub Reserved3: [*mut ::core::ffi::c_void; 2],
    pub Ldr: *mut PEB_LDR_DATA,
    pub ProcessParameters: *mut RTL_USER_PROCESS_PARAMETERS,
    pub Reserved4: [*mut ::core::ffi::c_void; 3],
    pub AtlThunkSListPtr: *mut ::core::ffi::c_void,
    pub Reserved5: *mut ::core::ffi::c_void,
    pub Reserved6: u32,
    pub Reserved7: *mut ::core::ffi::c_void,
    pub Reserved8: u32,
    pub AtlThunkSListPtr32: u32,
    pub Reserved9: [*mut ::core::ffi::c_void; 45],
    pub Reserved10: [u8; 96],
    pub PostProcessInitRoutine: PPS_POST_PROCESS_INIT_ROUTINE,
    pub Reserved11: [u8; 128],
    pub Reserved12: [*mut ::core::ffi::c_void; 1],
    pub SessionId: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::marker::Copy for PEB {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::clone::Clone for PEB {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::fmt::Debug for PEB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEB")
            .field("Reserved1", &self.Reserved1)
            .field("BeingDebugged", &self.BeingDebugged)
            .field("Reserved2", &self.Reserved2)
            .field("Reserved3", &self.Reserved3)
            .field("Ldr", &self.Ldr)
            .field("ProcessParameters", &self.ProcessParameters)
            .field("Reserved4", &self.Reserved4)
            .field("AtlThunkSListPtr", &self.AtlThunkSListPtr)
            .field("Reserved5", &self.Reserved5)
            .field("Reserved6", &self.Reserved6)
            .field("Reserved7", &self.Reserved7)
            .field("Reserved8", &self.Reserved8)
            .field("AtlThunkSListPtr32", &self.AtlThunkSListPtr32)
            .field("Reserved9", &self.Reserved9)
            .field("Reserved10", &self.Reserved10)
            .field("Reserved11", &self.Reserved11)
            .field("Reserved12", &self.Reserved12)
            .field("SessionId", &self.SessionId)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::windows::core::TypeKind for PEB {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::default::Default for PEB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(feature = "Win32_System_Kernel")]
pub struct PEB_LDR_DATA {
    pub Reserved1: [u8; 8],
    pub Reserved2: [*mut ::core::ffi::c_void; 3],
    pub InMemoryOrderModuleList: super::Kernel::LIST_ENTRY,
}
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::marker::Copy for PEB_LDR_DATA {}
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::clone::Clone for PEB_LDR_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::fmt::Debug for PEB_LDR_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEB_LDR_DATA").field("Reserved1", &self.Reserved1).field("Reserved2", &self.Reserved2).field("InMemoryOrderModuleList", &self.InMemoryOrderModuleList).finish()
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl ::windows::core::TypeKind for PEB_LDR_DATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::cmp::PartialEq for PEB_LDR_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved1 == other.Reserved1 && self.Reserved2 == other.Reserved2 && self.InMemoryOrderModuleList == other.InMemoryOrderModuleList
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::cmp::Eq for PEB_LDR_DATA {}
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::default::Default for PEB_LDR_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub struct PROCESS_BASIC_INFORMATION {
    pub ExitStatus: super::super::Foundation::NTSTATUS,
    pub PebBaseAddress: *mut PEB,
    pub AffinityMask: usize,
    pub BasePriority: i32,
    pub UniqueProcessId: usize,
    pub InheritedFromUniqueProcessId: usize,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::marker::Copy for PROCESS_BASIC_INFORMATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::clone::Clone for PROCESS_BASIC_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::fmt::Debug for PROCESS_BASIC_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROCESS_BASIC_INFORMATION").field("ExitStatus", &self.ExitStatus).field("PebBaseAddress", &self.PebBaseAddress).field("AffinityMask", &self.AffinityMask).field("BasePriority", &self.BasePriority).field("UniqueProcessId", &self.UniqueProcessId).field("InheritedFromUniqueProcessId", &self.InheritedFromUniqueProcessId).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::windows::core::TypeKind for PROCESS_BASIC_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::cmp::PartialEq for PROCESS_BASIC_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.ExitStatus == other.ExitStatus && self.PebBaseAddress == other.PebBaseAddress && self.AffinityMask == other.AffinityMask && self.BasePriority == other.BasePriority && self.UniqueProcessId == other.UniqueProcessId && self.InheritedFromUniqueProcessId == other.InheritedFromUniqueProcessId
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::cmp::Eq for PROCESS_BASIC_INFORMATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::default::Default for PROCESS_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub struct PROCESS_DYNAMIC_EH_CONTINUATION_TARGET {
    pub TargetAddress: usize,
    pub Flags: usize,
}
impl ::core::marker::Copy for PROCESS_DYNAMIC_EH_CONTINUATION_TARGET {}
impl ::core::clone::Clone for PROCESS_DYNAMIC_EH_CONTINUATION_TARGET {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PROCESS_DYNAMIC_EH_CONTINUATION_TARGET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROCESS_DYNAMIC_EH_CONTINUATION_TARGET").field("TargetAddress", &self.TargetAddress).field("Flags", &self.Flags).finish()
    }
}
impl ::windows::core::TypeKind for PROCESS_DYNAMIC_EH_CONTINUATION_TARGET {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for PROCESS_DYNAMIC_EH_CONTINUATION_TARGET {
    fn eq(&self, other: &Self) -> bool {
        self.TargetAddress == other.TargetAddress && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for PROCESS_DYNAMIC_EH_CONTINUATION_TARGET {}
impl ::core::default::Default for PROCESS_DYNAMIC_EH_CONTINUATION_TARGET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub struct PROCESS_DYNAMIC_EH_CONTINUATION_TARGETS_INFORMATION {
    pub NumberOfTargets: u16,
    pub Reserved: u16,
    pub Reserved2: u32,
    pub Targets: *mut PROCESS_DYNAMIC_EH_CONTINUATION_TARGET,
}
impl ::core::marker::Copy for PROCESS_DYNAMIC_EH_CONTINUATION_TARGETS_INFORMATION {}
impl ::core::clone::Clone for PROCESS_DYNAMIC_EH_CONTINUATION_TARGETS_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PROCESS_DYNAMIC_EH_CONTINUATION_TARGETS_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROCESS_DYNAMIC_EH_CONTINUATION_TARGETS_INFORMATION").field("NumberOfTargets", &self.NumberOfTargets).field("Reserved", &self.Reserved).field("Reserved2", &self.Reserved2).field("Targets", &self.Targets).finish()
    }
}
impl ::windows::core::TypeKind for PROCESS_DYNAMIC_EH_CONTINUATION_TARGETS_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for PROCESS_DYNAMIC_EH_CONTINUATION_TARGETS_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.NumberOfTargets == other.NumberOfTargets && self.Reserved == other.Reserved && self.Reserved2 == other.Reserved2 && self.Targets == other.Targets
    }
}
impl ::core::cmp::Eq for PROCESS_DYNAMIC_EH_CONTINUATION_TARGETS_INFORMATION {}
impl ::core::default::Default for PROCESS_DYNAMIC_EH_CONTINUATION_TARGETS_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub struct PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE {
    pub BaseAddress: usize,
    pub Size: usize,
    pub Flags: u32,
}
impl ::core::marker::Copy for PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE {}
impl ::core::clone::Clone for PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE").field("BaseAddress", &self.BaseAddress).field("Size", &self.Size).field("Flags", &self.Flags).finish()
    }
}
impl ::windows::core::TypeKind for PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.BaseAddress == other.BaseAddress && self.Size == other.Size && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE {}
impl ::core::default::Default for PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub struct PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGES_INFORMATION {
    pub NumberOfRanges: u16,
    pub Reserved: u16,
    pub Reserved2: u32,
    pub Ranges: *mut PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE,
}
impl ::core::marker::Copy for PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGES_INFORMATION {}
impl ::core::clone::Clone for PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGES_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGES_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGES_INFORMATION").field("NumberOfRanges", &self.NumberOfRanges).field("Reserved", &self.Reserved).field("Reserved2", &self.Reserved2).field("Ranges", &self.Ranges).finish()
    }
}
impl ::windows::core::TypeKind for PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGES_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGES_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.NumberOfRanges == other.NumberOfRanges && self.Reserved == other.Reserved && self.Reserved2 == other.Reserved2 && self.Ranges == other.Ranges
    }
}
impl ::core::cmp::Eq for PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGES_INFORMATION {}
impl ::core::default::Default for PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGES_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PROCESS_INFORMATION {
    pub hProcess: super::super::Foundation::HANDLE,
    pub hThread: super::super::Foundation::HANDLE,
    pub dwProcessId: u32,
    pub dwThreadId: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PROCESS_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PROCESS_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PROCESS_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROCESS_INFORMATION").field("hProcess", &self.hProcess).field("hThread", &self.hThread).field("dwProcessId", &self.dwProcessId).field("dwThreadId", &self.dwThreadId).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for PROCESS_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PROCESS_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.hProcess == other.hProcess && self.hThread == other.hThread && self.dwProcessId == other.dwProcessId && self.dwThreadId == other.dwThreadId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PROCESS_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PROCESS_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub struct PROCESS_LEAP_SECOND_INFO {
    pub Flags: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for PROCESS_LEAP_SECOND_INFO {}
impl ::core::clone::Clone for PROCESS_LEAP_SECOND_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PROCESS_LEAP_SECOND_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROCESS_LEAP_SECOND_INFO").field("Flags", &self.Flags).field("Reserved", &self.Reserved).finish()
    }
}
impl ::windows::core::TypeKind for PROCESS_LEAP_SECOND_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for PROCESS_LEAP_SECOND_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for PROCESS_LEAP_SECOND_INFO {}
impl ::core::default::Default for PROCESS_LEAP_SECOND_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_System_SystemInformation\"`*"]
#[cfg(feature = "Win32_System_SystemInformation")]
pub struct PROCESS_MACHINE_INFORMATION {
    pub ProcessMachine: super::SystemInformation::IMAGE_FILE_MACHINE,
    pub Res0: u16,
    pub MachineAttributes: MACHINE_ATTRIBUTES,
}
#[cfg(feature = "Win32_System_SystemInformation")]
impl ::core::marker::Copy for PROCESS_MACHINE_INFORMATION {}
#[cfg(feature = "Win32_System_SystemInformation")]
impl ::core::clone::Clone for PROCESS_MACHINE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_SystemInformation")]
impl ::core::fmt::Debug for PROCESS_MACHINE_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROCESS_MACHINE_INFORMATION").field("ProcessMachine", &self.ProcessMachine).field("Res0", &self.Res0).field("MachineAttributes", &self.MachineAttributes).finish()
    }
}
#[cfg(feature = "Win32_System_SystemInformation")]
impl ::windows::core::TypeKind for PROCESS_MACHINE_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_System_SystemInformation")]
impl ::core::cmp::PartialEq for PROCESS_MACHINE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.ProcessMachine == other.ProcessMachine && self.Res0 == other.Res0 && self.MachineAttributes == other.MachineAttributes
    }
}
#[cfg(feature = "Win32_System_SystemInformation")]
impl ::core::cmp::Eq for PROCESS_MACHINE_INFORMATION {}
#[cfg(feature = "Win32_System_SystemInformation")]
impl ::core::default::Default for PROCESS_MACHINE_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub struct PROCESS_MEMORY_EXHAUSTION_INFO {
    pub Version: u16,
    pub Reserved: u16,
    pub Type: PROCESS_MEMORY_EXHAUSTION_TYPE,
    pub Value: usize,
}
impl ::core::marker::Copy for PROCESS_MEMORY_EXHAUSTION_INFO {}
impl ::core::clone::Clone for PROCESS_MEMORY_EXHAUSTION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PROCESS_MEMORY_EXHAUSTION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROCESS_MEMORY_EXHAUSTION_INFO").field("Version", &self.Version).field("Reserved", &self.Reserved).field("Type", &self.Type).field("Value", &self.Value).finish()
    }
}
impl ::windows::core::TypeKind for PROCESS_MEMORY_EXHAUSTION_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for PROCESS_MEMORY_EXHAUSTION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Reserved == other.Reserved && self.Type == other.Type && self.Value == other.Value
    }
}
impl ::core::cmp::Eq for PROCESS_MEMORY_EXHAUSTION_INFO {}
impl ::core::default::Default for PROCESS_MEMORY_EXHAUSTION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub struct PROCESS_POWER_THROTTLING_STATE {
    pub Version: u32,
    pub ControlMask: u32,
    pub StateMask: u32,
}
impl ::core::marker::Copy for PROCESS_POWER_THROTTLING_STATE {}
impl ::core::clone::Clone for PROCESS_POWER_THROTTLING_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PROCESS_POWER_THROTTLING_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROCESS_POWER_THROTTLING_STATE").field("Version", &self.Version).field("ControlMask", &self.ControlMask).field("StateMask", &self.StateMask).finish()
    }
}
impl ::windows::core::TypeKind for PROCESS_POWER_THROTTLING_STATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for PROCESS_POWER_THROTTLING_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.ControlMask == other.ControlMask && self.StateMask == other.StateMask
    }
}
impl ::core::cmp::Eq for PROCESS_POWER_THROTTLING_STATE {}
impl ::core::default::Default for PROCESS_POWER_THROTTLING_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub struct PROCESS_PROTECTION_LEVEL_INFORMATION {
    pub ProtectionLevel: PROCESS_PROTECTION_LEVEL,
}
impl ::core::marker::Copy for PROCESS_PROTECTION_LEVEL_INFORMATION {}
impl ::core::clone::Clone for PROCESS_PROTECTION_LEVEL_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PROCESS_PROTECTION_LEVEL_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROCESS_PROTECTION_LEVEL_INFORMATION").field("ProtectionLevel", &self.ProtectionLevel).finish()
    }
}
impl ::windows::core::TypeKind for PROCESS_PROTECTION_LEVEL_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for PROCESS_PROTECTION_LEVEL_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.ProtectionLevel == other.ProtectionLevel
    }
}
impl ::core::cmp::Eq for PROCESS_PROTECTION_LEVEL_INFORMATION {}
impl ::core::default::Default for PROCESS_PROTECTION_LEVEL_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PTP_CALLBACK_INSTANCE(pub isize);
impl PTP_CALLBACK_INSTANCE {
    pub fn is_invalid(&self) -> bool {
        self.0 == 0
    }
}
impl ::core::default::Default for PTP_CALLBACK_INSTANCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for PTP_CALLBACK_INSTANCE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for PTP_CALLBACK_INSTANCE {}
impl ::core::fmt::Debug for PTP_CALLBACK_INSTANCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PTP_CALLBACK_INSTANCE").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for PTP_CALLBACK_INSTANCE {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PTP_IO(pub isize);
impl PTP_IO {
    pub fn is_invalid(&self) -> bool {
        self.0 == 0
    }
}
impl ::core::default::Default for PTP_IO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for PTP_IO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for PTP_IO {}
impl ::core::fmt::Debug for PTP_IO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PTP_IO").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for PTP_IO {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PTP_POOL(pub isize);
impl ::core::default::Default for PTP_POOL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for PTP_POOL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for PTP_POOL {}
impl ::core::fmt::Debug for PTP_POOL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PTP_POOL").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for PTP_POOL {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PTP_TIMER(pub isize);
impl PTP_TIMER {
    pub fn is_invalid(&self) -> bool {
        self.0 == 0
    }
}
impl ::core::default::Default for PTP_TIMER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for PTP_TIMER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for PTP_TIMER {}
impl ::core::fmt::Debug for PTP_TIMER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PTP_TIMER").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for PTP_TIMER {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PTP_WAIT(pub isize);
impl PTP_WAIT {
    pub fn is_invalid(&self) -> bool {
        self.0 == 0
    }
}
impl ::core::default::Default for PTP_WAIT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for PTP_WAIT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for PTP_WAIT {}
impl ::core::fmt::Debug for PTP_WAIT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PTP_WAIT").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for PTP_WAIT {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PTP_WORK(pub isize);
impl PTP_WORK {
    pub fn is_invalid(&self) -> bool {
        self.0 == 0
    }
}
impl ::core::default::Default for PTP_WORK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for PTP_WORK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for PTP_WORK {}
impl ::core::fmt::Debug for PTP_WORK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PTP_WORK").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for PTP_WORK {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct REASON_CONTEXT {
    pub Version: u32,
    pub Flags: POWER_REQUEST_CONTEXT_FLAGS,
    pub Reason: REASON_CONTEXT_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for REASON_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for REASON_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for REASON_CONTEXT {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for REASON_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union REASON_CONTEXT_0 {
    pub Detailed: REASON_CONTEXT_0_0,
    pub SimpleReasonString: ::windows::core::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for REASON_CONTEXT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for REASON_CONTEXT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for REASON_CONTEXT_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for REASON_CONTEXT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct REASON_CONTEXT_0_0 {
    pub LocalizedReasonModule: super::super::Foundation::HINSTANCE,
    pub LocalizedReasonId: u32,
    pub ReasonStringCount: u32,
    pub ReasonStrings: *mut ::windows::core::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for REASON_CONTEXT_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for REASON_CONTEXT_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for REASON_CONTEXT_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REASON_CONTEXT_0_0").field("LocalizedReasonModule", &self.LocalizedReasonModule).field("LocalizedReasonId", &self.LocalizedReasonId).field("ReasonStringCount", &self.ReasonStringCount).field("ReasonStrings", &self.ReasonStrings).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for REASON_CONTEXT_0_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for REASON_CONTEXT_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.LocalizedReasonModule == other.LocalizedReasonModule && self.LocalizedReasonId == other.LocalizedReasonId && self.ReasonStringCount == other.ReasonStringCount && self.ReasonStrings == other.ReasonStrings
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for REASON_CONTEXT_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for REASON_CONTEXT_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub struct RTL_BARRIER {
    pub Reserved1: u32,
    pub Reserved2: u32,
    pub Reserved3: [usize; 2],
    pub Reserved4: u32,
    pub Reserved5: u32,
}
impl ::core::marker::Copy for RTL_BARRIER {}
impl ::core::clone::Clone for RTL_BARRIER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RTL_BARRIER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RTL_BARRIER").field("Reserved1", &self.Reserved1).field("Reserved2", &self.Reserved2).field("Reserved3", &self.Reserved3).field("Reserved4", &self.Reserved4).field("Reserved5", &self.Reserved5).finish()
    }
}
impl ::windows::core::TypeKind for RTL_BARRIER {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for RTL_BARRIER {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved1 == other.Reserved1 && self.Reserved2 == other.Reserved2 && self.Reserved3 == other.Reserved3 && self.Reserved4 == other.Reserved4 && self.Reserved5 == other.Reserved5
    }
}
impl ::core::cmp::Eq for RTL_BARRIER {}
impl ::core::default::Default for RTL_BARRIER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub struct RTL_CONDITION_VARIABLE {
    pub Ptr: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for RTL_CONDITION_VARIABLE {}
impl ::core::clone::Clone for RTL_CONDITION_VARIABLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RTL_CONDITION_VARIABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RTL_CONDITION_VARIABLE").field("Ptr", &self.Ptr).finish()
    }
}
impl ::windows::core::TypeKind for RTL_CONDITION_VARIABLE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for RTL_CONDITION_VARIABLE {
    fn eq(&self, other: &Self) -> bool {
        self.Ptr == other.Ptr
    }
}
impl ::core::cmp::Eq for RTL_CONDITION_VARIABLE {}
impl ::core::default::Default for RTL_CONDITION_VARIABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub struct RTL_CRITICAL_SECTION {
    pub DebugInfo: *mut RTL_CRITICAL_SECTION_DEBUG,
    pub LockCount: i32,
    pub RecursionCount: i32,
    pub OwningThread: super::super::Foundation::HANDLE,
    pub LockSemaphore: super::super::Foundation::HANDLE,
    pub SpinCount: usize,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::marker::Copy for RTL_CRITICAL_SECTION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::clone::Clone for RTL_CRITICAL_SECTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::fmt::Debug for RTL_CRITICAL_SECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RTL_CRITICAL_SECTION").field("DebugInfo", &self.DebugInfo).field("LockCount", &self.LockCount).field("RecursionCount", &self.RecursionCount).field("OwningThread", &self.OwningThread).field("LockSemaphore", &self.LockSemaphore).field("SpinCount", &self.SpinCount).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::windows::core::TypeKind for RTL_CRITICAL_SECTION {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::cmp::PartialEq for RTL_CRITICAL_SECTION {
    fn eq(&self, other: &Self) -> bool {
        self.DebugInfo == other.DebugInfo && self.LockCount == other.LockCount && self.RecursionCount == other.RecursionCount && self.OwningThread == other.OwningThread && self.LockSemaphore == other.LockSemaphore && self.SpinCount == other.SpinCount
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::cmp::Eq for RTL_CRITICAL_SECTION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::default::Default for RTL_CRITICAL_SECTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub struct RTL_CRITICAL_SECTION_DEBUG {
    pub Type: u16,
    pub CreatorBackTraceIndex: u16,
    pub CriticalSection: *mut RTL_CRITICAL_SECTION,
    pub ProcessLocksList: super::Kernel::LIST_ENTRY,
    pub EntryCount: u32,
    pub ContentionCount: u32,
    pub Flags: u32,
    pub CreatorBackTraceIndexHigh: u16,
    pub SpareWORD: u16,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::marker::Copy for RTL_CRITICAL_SECTION_DEBUG {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::clone::Clone for RTL_CRITICAL_SECTION_DEBUG {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::fmt::Debug for RTL_CRITICAL_SECTION_DEBUG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RTL_CRITICAL_SECTION_DEBUG").field("Type", &self.Type).field("CreatorBackTraceIndex", &self.CreatorBackTraceIndex).field("CriticalSection", &self.CriticalSection).field("ProcessLocksList", &self.ProcessLocksList).field("EntryCount", &self.EntryCount).field("ContentionCount", &self.ContentionCount).field("Flags", &self.Flags).field("CreatorBackTraceIndexHigh", &self.CreatorBackTraceIndexHigh).field("SpareWORD", &self.SpareWORD).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::windows::core::TypeKind for RTL_CRITICAL_SECTION_DEBUG {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::cmp::PartialEq for RTL_CRITICAL_SECTION_DEBUG {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.CreatorBackTraceIndex == other.CreatorBackTraceIndex && self.CriticalSection == other.CriticalSection && self.ProcessLocksList == other.ProcessLocksList && self.EntryCount == other.EntryCount && self.ContentionCount == other.ContentionCount && self.Flags == other.Flags && self.CreatorBackTraceIndexHigh == other.CreatorBackTraceIndexHigh && self.SpareWORD == other.SpareWORD
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::cmp::Eq for RTL_CRITICAL_SECTION_DEBUG {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::default::Default for RTL_CRITICAL_SECTION_DEBUG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub union RTL_RUN_ONCE {
    pub Ptr: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for RTL_RUN_ONCE {}
impl ::core::clone::Clone for RTL_RUN_ONCE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for RTL_RUN_ONCE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for RTL_RUN_ONCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub struct RTL_SRWLOCK {
    pub Ptr: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for RTL_SRWLOCK {}
impl ::core::clone::Clone for RTL_SRWLOCK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RTL_SRWLOCK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RTL_SRWLOCK").field("Ptr", &self.Ptr).finish()
    }
}
impl ::windows::core::TypeKind for RTL_SRWLOCK {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for RTL_SRWLOCK {
    fn eq(&self, other: &Self) -> bool {
        self.Ptr == other.Ptr
    }
}
impl ::core::cmp::Eq for RTL_SRWLOCK {}
impl ::core::default::Default for RTL_SRWLOCK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RTL_USER_PROCESS_PARAMETERS {
    pub Reserved1: [u8; 16],
    pub Reserved2: [*mut ::core::ffi::c_void; 10],
    pub ImagePathName: super::super::Foundation::UNICODE_STRING,
    pub CommandLine: super::super::Foundation::UNICODE_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RTL_USER_PROCESS_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RTL_USER_PROCESS_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RTL_USER_PROCESS_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RTL_USER_PROCESS_PARAMETERS").field("Reserved1", &self.Reserved1).field("Reserved2", &self.Reserved2).field("ImagePathName", &self.ImagePathName).field("CommandLine", &self.CommandLine).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for RTL_USER_PROCESS_PARAMETERS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RTL_USER_PROCESS_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved1 == other.Reserved1 && self.Reserved2 == other.Reserved2 && self.ImagePathName == other.ImagePathName && self.CommandLine == other.CommandLine
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RTL_USER_PROCESS_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RTL_USER_PROCESS_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct STARTUPINFOA {
    pub cb: u32,
    pub lpReserved: ::windows::core::PSTR,
    pub lpDesktop: ::windows::core::PSTR,
    pub lpTitle: ::windows::core::PSTR,
    pub dwX: u32,
    pub dwY: u32,
    pub dwXSize: u32,
    pub dwYSize: u32,
    pub dwXCountChars: u32,
    pub dwYCountChars: u32,
    pub dwFillAttribute: u32,
    pub dwFlags: STARTUPINFOW_FLAGS,
    pub wShowWindow: u16,
    pub cbReserved2: u16,
    pub lpReserved2: *mut u8,
    pub hStdInput: super::super::Foundation::HANDLE,
    pub hStdOutput: super::super::Foundation::HANDLE,
    pub hStdError: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STARTUPINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STARTUPINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for STARTUPINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STARTUPINFOA")
            .field("cb", &self.cb)
            .field("lpReserved", &self.lpReserved)
            .field("lpDesktop", &self.lpDesktop)
            .field("lpTitle", &self.lpTitle)
            .field("dwX", &self.dwX)
            .field("dwY", &self.dwY)
            .field("dwXSize", &self.dwXSize)
            .field("dwYSize", &self.dwYSize)
            .field("dwXCountChars", &self.dwXCountChars)
            .field("dwYCountChars", &self.dwYCountChars)
            .field("dwFillAttribute", &self.dwFillAttribute)
            .field("dwFlags", &self.dwFlags)
            .field("wShowWindow", &self.wShowWindow)
            .field("cbReserved2", &self.cbReserved2)
            .field("lpReserved2", &self.lpReserved2)
            .field("hStdInput", &self.hStdInput)
            .field("hStdOutput", &self.hStdOutput)
            .field("hStdError", &self.hStdError)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for STARTUPINFOA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for STARTUPINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb && self.lpReserved == other.lpReserved && self.lpDesktop == other.lpDesktop && self.lpTitle == other.lpTitle && self.dwX == other.dwX && self.dwY == other.dwY && self.dwXSize == other.dwXSize && self.dwYSize == other.dwYSize && self.dwXCountChars == other.dwXCountChars && self.dwYCountChars == other.dwYCountChars && self.dwFillAttribute == other.dwFillAttribute && self.dwFlags == other.dwFlags && self.wShowWindow == other.wShowWindow && self.cbReserved2 == other.cbReserved2 && self.lpReserved2 == other.lpReserved2 && self.hStdInput == other.hStdInput && self.hStdOutput == other.hStdOutput && self.hStdError == other.hStdError
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for STARTUPINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STARTUPINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct STARTUPINFOEXA {
    pub StartupInfo: STARTUPINFOA,
    pub lpAttributeList: LPPROC_THREAD_ATTRIBUTE_LIST,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STARTUPINFOEXA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STARTUPINFOEXA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for STARTUPINFOEXA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STARTUPINFOEXA").field("StartupInfo", &self.StartupInfo).field("lpAttributeList", &self.lpAttributeList).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for STARTUPINFOEXA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for STARTUPINFOEXA {
    fn eq(&self, other: &Self) -> bool {
        self.StartupInfo == other.StartupInfo && self.lpAttributeList == other.lpAttributeList
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for STARTUPINFOEXA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STARTUPINFOEXA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct STARTUPINFOEXW {
    pub StartupInfo: STARTUPINFOW,
    pub lpAttributeList: LPPROC_THREAD_ATTRIBUTE_LIST,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STARTUPINFOEXW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STARTUPINFOEXW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for STARTUPINFOEXW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STARTUPINFOEXW").field("StartupInfo", &self.StartupInfo).field("lpAttributeList", &self.lpAttributeList).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for STARTUPINFOEXW {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for STARTUPINFOEXW {
    fn eq(&self, other: &Self) -> bool {
        self.StartupInfo == other.StartupInfo && self.lpAttributeList == other.lpAttributeList
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for STARTUPINFOEXW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STARTUPINFOEXW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct STARTUPINFOW {
    pub cb: u32,
    pub lpReserved: ::windows::core::PWSTR,
    pub lpDesktop: ::windows::core::PWSTR,
    pub lpTitle: ::windows::core::PWSTR,
    pub dwX: u32,
    pub dwY: u32,
    pub dwXSize: u32,
    pub dwYSize: u32,
    pub dwXCountChars: u32,
    pub dwYCountChars: u32,
    pub dwFillAttribute: u32,
    pub dwFlags: STARTUPINFOW_FLAGS,
    pub wShowWindow: u16,
    pub cbReserved2: u16,
    pub lpReserved2: *mut u8,
    pub hStdInput: super::super::Foundation::HANDLE,
    pub hStdOutput: super::super::Foundation::HANDLE,
    pub hStdError: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STARTUPINFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STARTUPINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for STARTUPINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STARTUPINFOW")
            .field("cb", &self.cb)
            .field("lpReserved", &self.lpReserved)
            .field("lpDesktop", &self.lpDesktop)
            .field("lpTitle", &self.lpTitle)
            .field("dwX", &self.dwX)
            .field("dwY", &self.dwY)
            .field("dwXSize", &self.dwXSize)
            .field("dwYSize", &self.dwYSize)
            .field("dwXCountChars", &self.dwXCountChars)
            .field("dwYCountChars", &self.dwYCountChars)
            .field("dwFillAttribute", &self.dwFillAttribute)
            .field("dwFlags", &self.dwFlags)
            .field("wShowWindow", &self.wShowWindow)
            .field("cbReserved2", &self.cbReserved2)
            .field("lpReserved2", &self.lpReserved2)
            .field("hStdInput", &self.hStdInput)
            .field("hStdOutput", &self.hStdOutput)
            .field("hStdError", &self.hStdError)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for STARTUPINFOW {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for STARTUPINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb && self.lpReserved == other.lpReserved && self.lpDesktop == other.lpDesktop && self.lpTitle == other.lpTitle && self.dwX == other.dwX && self.dwY == other.dwY && self.dwXSize == other.dwXSize && self.dwYSize == other.dwYSize && self.dwXCountChars == other.dwXCountChars && self.dwYCountChars == other.dwYCountChars && self.dwFillAttribute == other.dwFillAttribute && self.dwFlags == other.dwFlags && self.wShowWindow == other.wShowWindow && self.cbReserved2 == other.cbReserved2 && self.lpReserved2 == other.lpReserved2 && self.hStdInput == other.hStdInput && self.hStdOutput == other.hStdOutput && self.hStdError == other.hStdError
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for STARTUPINFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STARTUPINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub struct THREAD_POWER_THROTTLING_STATE {
    pub Version: u32,
    pub ControlMask: u32,
    pub StateMask: u32,
}
impl ::core::marker::Copy for THREAD_POWER_THROTTLING_STATE {}
impl ::core::clone::Clone for THREAD_POWER_THROTTLING_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for THREAD_POWER_THROTTLING_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("THREAD_POWER_THROTTLING_STATE").field("Version", &self.Version).field("ControlMask", &self.ControlMask).field("StateMask", &self.StateMask).finish()
    }
}
impl ::windows::core::TypeKind for THREAD_POWER_THROTTLING_STATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for THREAD_POWER_THROTTLING_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.ControlMask == other.ControlMask && self.StateMask == other.StateMask
    }
}
impl ::core::cmp::Eq for THREAD_POWER_THROTTLING_STATE {}
impl ::core::default::Default for THREAD_POWER_THROTTLING_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub struct TP_CALLBACK_ENVIRON_V3 {
    pub Version: u32,
    pub Pool: PTP_POOL,
    pub CleanupGroup: isize,
    pub CleanupGroupCancelCallback: PTP_CLEANUP_GROUP_CANCEL_CALLBACK,
    pub RaceDll: *mut ::core::ffi::c_void,
    pub ActivationContext: isize,
    pub FinalizationCallback: PTP_SIMPLE_CALLBACK,
    pub u: TP_CALLBACK_ENVIRON_V3_1,
    pub CallbackPriority: TP_CALLBACK_PRIORITY,
    pub Size: u32,
}
impl ::core::marker::Copy for TP_CALLBACK_ENVIRON_V3 {}
impl ::core::clone::Clone for TP_CALLBACK_ENVIRON_V3 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for TP_CALLBACK_ENVIRON_V3 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for TP_CALLBACK_ENVIRON_V3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct TP_CALLBACK_ENVIRON_V3_0(pub u8);
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub union TP_CALLBACK_ENVIRON_V3_1 {
    pub Flags: u32,
    pub s: TP_CALLBACK_ENVIRON_V3_1_0,
}
impl ::core::marker::Copy for TP_CALLBACK_ENVIRON_V3_1 {}
impl ::core::clone::Clone for TP_CALLBACK_ENVIRON_V3_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for TP_CALLBACK_ENVIRON_V3_1 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for TP_CALLBACK_ENVIRON_V3_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub struct TP_CALLBACK_ENVIRON_V3_1_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for TP_CALLBACK_ENVIRON_V3_1_0 {}
impl ::core::clone::Clone for TP_CALLBACK_ENVIRON_V3_1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TP_CALLBACK_ENVIRON_V3_1_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TP_CALLBACK_ENVIRON_V3_1_0").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::windows::core::TypeKind for TP_CALLBACK_ENVIRON_V3_1_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for TP_CALLBACK_ENVIRON_V3_1_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for TP_CALLBACK_ENVIRON_V3_1_0 {}
impl ::core::default::Default for TP_CALLBACK_ENVIRON_V3_1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub struct TP_POOL_STACK_INFORMATION {
    pub StackReserve: usize,
    pub StackCommit: usize,
}
impl ::core::marker::Copy for TP_POOL_STACK_INFORMATION {}
impl ::core::clone::Clone for TP_POOL_STACK_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TP_POOL_STACK_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TP_POOL_STACK_INFORMATION").field("StackReserve", &self.StackReserve).field("StackCommit", &self.StackCommit).finish()
    }
}
impl ::windows::core::TypeKind for TP_POOL_STACK_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for TP_POOL_STACK_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.StackReserve == other.StackReserve && self.StackCommit == other.StackCommit
    }
}
impl ::core::cmp::Eq for TP_POOL_STACK_INFORMATION {}
impl ::core::default::Default for TP_POOL_STACK_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TimerQueueHandle(pub isize);
impl TimerQueueHandle {
    pub fn is_invalid(&self) -> bool {
        self.0 == 0
    }
}
impl ::core::default::Default for TimerQueueHandle {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for TimerQueueHandle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for TimerQueueHandle {}
impl ::core::fmt::Debug for TimerQueueHandle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimerQueueHandle").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for TimerQueueHandle {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_System_SystemServices\"`*"]
#[cfg(feature = "Win32_System_SystemServices")]
pub struct UMS_SCHEDULER_STARTUP_INFO {
    pub UmsVersion: u32,
    pub CompletionList: *mut ::core::ffi::c_void,
    pub SchedulerProc: PRTL_UMS_SCHEDULER_ENTRY_POINT,
    pub SchedulerParam: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::core::marker::Copy for UMS_SCHEDULER_STARTUP_INFO {}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::core::clone::Clone for UMS_SCHEDULER_STARTUP_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::core::fmt::Debug for UMS_SCHEDULER_STARTUP_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UMS_SCHEDULER_STARTUP_INFO").field("UmsVersion", &self.UmsVersion).field("CompletionList", &self.CompletionList).field("SchedulerParam", &self.SchedulerParam).finish()
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::windows::core::TypeKind for UMS_SCHEDULER_STARTUP_INFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::core::default::Default for UMS_SCHEDULER_STARTUP_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub struct UMS_SYSTEM_THREAD_INFORMATION {
    pub UmsVersion: u32,
    pub Anonymous: UMS_SYSTEM_THREAD_INFORMATION_0,
}
impl ::core::marker::Copy for UMS_SYSTEM_THREAD_INFORMATION {}
impl ::core::clone::Clone for UMS_SYSTEM_THREAD_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for UMS_SYSTEM_THREAD_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for UMS_SYSTEM_THREAD_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub union UMS_SYSTEM_THREAD_INFORMATION_0 {
    pub Anonymous: UMS_SYSTEM_THREAD_INFORMATION_0_0,
    pub ThreadUmsFlags: u32,
}
impl ::core::marker::Copy for UMS_SYSTEM_THREAD_INFORMATION_0 {}
impl ::core::clone::Clone for UMS_SYSTEM_THREAD_INFORMATION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for UMS_SYSTEM_THREAD_INFORMATION_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for UMS_SYSTEM_THREAD_INFORMATION_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub struct UMS_SYSTEM_THREAD_INFORMATION_0_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for UMS_SYSTEM_THREAD_INFORMATION_0_0 {}
impl ::core::clone::Clone for UMS_SYSTEM_THREAD_INFORMATION_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for UMS_SYSTEM_THREAD_INFORMATION_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UMS_SYSTEM_THREAD_INFORMATION_0_0").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::windows::core::TypeKind for UMS_SYSTEM_THREAD_INFORMATION_0_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for UMS_SYSTEM_THREAD_INFORMATION_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for UMS_SYSTEM_THREAD_INFORMATION_0_0 {}
impl ::core::default::Default for UMS_SYSTEM_THREAD_INFORMATION_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub type LPFIBER_START_ROUTINE = ::core::option::Option<unsafe extern "system" fn(lpfiberparameter: *mut ::core::ffi::c_void) -> ()>;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub type LPTHREAD_START_ROUTINE = ::core::option::Option<unsafe extern "system" fn(lpthreadparameter: *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub type PFLS_CALLBACK_FUNCTION = ::core::option::Option<unsafe extern "system" fn(lpflsdata: *const ::core::ffi::c_void) -> ()>;
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PINIT_ONCE_FN = ::core::option::Option<unsafe extern "system" fn(initonce: *mut RTL_RUN_ONCE, parameter: *mut ::core::ffi::c_void, context: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub type PPS_POST_PROCESS_INIT_ROUTINE = ::core::option::Option<unsafe extern "system" fn() -> ()>;
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_System_SystemServices\"`*"]
#[cfg(feature = "Win32_System_SystemServices")]
pub type PRTL_UMS_SCHEDULER_ENTRY_POINT = ::core::option::Option<unsafe extern "system" fn(reason: super::SystemServices::RTL_UMS_SCHEDULER_REASON, activationpayload: usize, schedulerparam: *const ::core::ffi::c_void) -> ()>;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub type PTIMERAPCROUTINE = ::core::option::Option<unsafe extern "system" fn(lpargtocompletionroutine: *const ::core::ffi::c_void, dwtimerlowvalue: u32, dwtimerhighvalue: u32) -> ()>;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub type PTP_CLEANUP_GROUP_CANCEL_CALLBACK = ::core::option::Option<unsafe extern "system" fn(objectcontext: *mut ::core::ffi::c_void, cleanupcontext: *mut ::core::ffi::c_void) -> ()>;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub type PTP_SIMPLE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(instance: PTP_CALLBACK_INSTANCE, context: *mut ::core::ffi::c_void) -> ()>;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub type PTP_TIMER_CALLBACK = ::core::option::Option<unsafe extern "system" fn(instance: PTP_CALLBACK_INSTANCE, context: *mut ::core::ffi::c_void, timer: PTP_TIMER) -> ()>;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub type PTP_WAIT_CALLBACK = ::core::option::Option<unsafe extern "system" fn(instance: PTP_CALLBACK_INSTANCE, context: *mut ::core::ffi::c_void, wait: PTP_WAIT, waitresult: u32) -> ()>;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub type PTP_WIN32_IO_CALLBACK = ::core::option::Option<unsafe extern "system" fn(instance: PTP_CALLBACK_INSTANCE, context: *mut ::core::ffi::c_void, overlapped: *mut ::core::ffi::c_void, ioresult: u32, numberofbytestransferred: usize, io: PTP_IO) -> ()>;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub type PTP_WORK_CALLBACK = ::core::option::Option<unsafe extern "system" fn(instance: PTP_CALLBACK_INSTANCE, context: *mut ::core::ffi::c_void, work: PTP_WORK) -> ()>;
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type WAITORTIMERCALLBACK = ::core::option::Option<unsafe extern "system" fn(param0: *mut ::core::ffi::c_void, param1: super::super::Foundation::BOOLEAN) -> ()>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
