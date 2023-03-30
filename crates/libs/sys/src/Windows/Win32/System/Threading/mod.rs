::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`*"] fn AcquireSRWLockExclusive ( srwlock : *mut RTL_SRWLOCK ) -> ( ) );
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`*"] fn AcquireSRWLockShared ( srwlock : *mut RTL_SRWLOCK ) -> ( ) );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn AddIntegrityLabelToBoundaryDescriptor ( boundarydescriptor : *mut super::super::Foundation:: HANDLE , integritylabel : super::super::Foundation:: PSID ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn AddSIDToBoundaryDescriptor ( boundarydescriptor : *mut super::super::Foundation:: HANDLE , requiredsid : super::super::Foundation:: PSID ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "user32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn AttachThreadInput ( idattach : u32 , idattachto : u32 , fattach : super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "avrt.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn AvQuerySystemResponsiveness ( avrthandle : super::super::Foundation:: HANDLE , systemresponsivenessvalue : *mut u32 ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "avrt.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn AvRevertMmThreadCharacteristics ( avrthandle : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "avrt.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn AvRtCreateThreadOrderingGroup ( context : *mut super::super::Foundation:: HANDLE , period : *const i64 , threadorderingguid : *mut ::windows_sys::core::GUID , timeout : *const i64 ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "avrt.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn AvRtCreateThreadOrderingGroupExA ( context : *mut super::super::Foundation:: HANDLE , period : *const i64 , threadorderingguid : *mut ::windows_sys::core::GUID , timeout : *const i64 , taskname : ::windows_sys::core::PCSTR ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "avrt.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn AvRtCreateThreadOrderingGroupExW ( context : *mut super::super::Foundation:: HANDLE , period : *const i64 , threadorderingguid : *mut ::windows_sys::core::GUID , timeout : *const i64 , taskname : ::windows_sys::core::PCWSTR ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "avrt.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn AvRtDeleteThreadOrderingGroup ( context : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "avrt.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn AvRtJoinThreadOrderingGroup ( context : *mut super::super::Foundation:: HANDLE , threadorderingguid : *const ::windows_sys::core::GUID , before : super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "avrt.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn AvRtLeaveThreadOrderingGroup ( context : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "avrt.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn AvRtWaitOnThreadOrderingGroup ( context : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "avrt.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn AvSetMmMaxThreadCharacteristicsA ( firsttask : ::windows_sys::core::PCSTR , secondtask : ::windows_sys::core::PCSTR , taskindex : *mut u32 ) -> super::super::Foundation:: HANDLE );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "avrt.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn AvSetMmMaxThreadCharacteristicsW ( firsttask : ::windows_sys::core::PCWSTR , secondtask : ::windows_sys::core::PCWSTR , taskindex : *mut u32 ) -> super::super::Foundation:: HANDLE );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "avrt.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn AvSetMmThreadCharacteristicsA ( taskname : ::windows_sys::core::PCSTR , taskindex : *mut u32 ) -> super::super::Foundation:: HANDLE );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "avrt.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn AvSetMmThreadCharacteristicsW ( taskname : ::windows_sys::core::PCWSTR , taskindex : *mut u32 ) -> super::super::Foundation:: HANDLE );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "avrt.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn AvSetMmThreadPriority ( avrthandle : super::super::Foundation:: HANDLE , priority : AVRT_PRIORITY ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn CallbackMayRunLong ( pci : PTP_CALLBACK_INSTANCE ) -> super::super::Foundation:: BOOL );
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`*"] fn CancelThreadpoolIo ( pio : PTP_IO ) -> ( ) );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn CancelWaitableTimer ( htimer : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn ChangeTimerQueueTimer ( timerqueue : super::super::Foundation:: HANDLE , timer : super::super::Foundation:: HANDLE , duetime : u32 , period : u32 ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn ClosePrivateNamespace ( handle : NamespaceHandle , flags : u32 ) -> super::super::Foundation:: BOOLEAN );
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`*"] fn CloseThreadpool ( ptpp : PTP_POOL ) -> ( ) );
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`*"] fn CloseThreadpoolCleanupGroup ( ptpcg : isize ) -> ( ) );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn CloseThreadpoolCleanupGroupMembers ( ptpcg : isize , fcancelpendingcallbacks : super::super::Foundation:: BOOL , pvcleanupcontext : *mut ::core::ffi::c_void ) -> ( ) );
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`*"] fn CloseThreadpoolIo ( pio : PTP_IO ) -> ( ) );
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`*"] fn CloseThreadpoolTimer ( pti : PTP_TIMER ) -> ( ) );
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`*"] fn CloseThreadpoolWait ( pwa : PTP_WAIT ) -> ( ) );
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`*"] fn CloseThreadpoolWork ( pwk : PTP_WORK ) -> ( ) );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn ConvertFiberToThread ( ) -> super::super::Foundation:: BOOL );
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`*"] fn ConvertThreadToFiber ( lpparameter : *const ::core::ffi::c_void ) -> *mut ::core::ffi::c_void );
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`*"] fn ConvertThreadToFiberEx ( lpparameter : *const ::core::ffi::c_void , dwflags : u32 ) -> *mut ::core::ffi::c_void );
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`*"] fn CreateBoundaryDescriptorA ( name : ::windows_sys::core::PCSTR , flags : u32 ) -> BoundaryDescriptorHandle );
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`*"] fn CreateBoundaryDescriptorW ( name : ::windows_sys::core::PCWSTR , flags : u32 ) -> BoundaryDescriptorHandle );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn CreateEventA ( lpeventattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , bmanualreset : super::super::Foundation:: BOOL , binitialstate : super::super::Foundation:: BOOL , lpname : ::windows_sys::core::PCSTR ) -> super::super::Foundation:: HANDLE );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn CreateEventExA ( lpeventattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , lpname : ::windows_sys::core::PCSTR , dwflags : CREATE_EVENT , dwdesiredaccess : u32 ) -> super::super::Foundation:: HANDLE );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn CreateEventExW ( lpeventattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , lpname : ::windows_sys::core::PCWSTR , dwflags : CREATE_EVENT , dwdesiredaccess : u32 ) -> super::super::Foundation:: HANDLE );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn CreateEventW ( lpeventattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , bmanualreset : super::super::Foundation:: BOOL , binitialstate : super::super::Foundation:: BOOL , lpname : ::windows_sys::core::PCWSTR ) -> super::super::Foundation:: HANDLE );
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`*"] fn CreateFiber ( dwstacksize : usize , lpstartaddress : LPFIBER_START_ROUTINE , lpparameter : *const ::core::ffi::c_void ) -> *mut ::core::ffi::c_void );
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`*"] fn CreateFiberEx ( dwstackcommitsize : usize , dwstackreservesize : usize , dwflags : u32 , lpstartaddress : LPFIBER_START_ROUTINE , lpparameter : *const ::core::ffi::c_void ) -> *mut ::core::ffi::c_void );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn CreateMutexA ( lpmutexattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , binitialowner : super::super::Foundation:: BOOL , lpname : ::windows_sys::core::PCSTR ) -> super::super::Foundation:: HANDLE );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn CreateMutexExA ( lpmutexattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , lpname : ::windows_sys::core::PCSTR , dwflags : u32 , dwdesiredaccess : u32 ) -> super::super::Foundation:: HANDLE );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn CreateMutexExW ( lpmutexattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , lpname : ::windows_sys::core::PCWSTR , dwflags : u32 , dwdesiredaccess : u32 ) -> super::super::Foundation:: HANDLE );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn CreateMutexW ( lpmutexattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , binitialowner : super::super::Foundation:: BOOL , lpname : ::windows_sys::core::PCWSTR ) -> super::super::Foundation:: HANDLE );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn CreatePrivateNamespaceA ( lpprivatenamespaceattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , lpboundarydescriptor : *const ::core::ffi::c_void , lpaliasprefix : ::windows_sys::core::PCSTR ) -> NamespaceHandle );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn CreatePrivateNamespaceW ( lpprivatenamespaceattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , lpboundarydescriptor : *const ::core::ffi::c_void , lpaliasprefix : ::windows_sys::core::PCWSTR ) -> NamespaceHandle );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn CreateProcessA ( lpapplicationname : ::windows_sys::core::PCSTR , lpcommandline : ::windows_sys::core::PSTR , lpprocessattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , lpthreadattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , binherithandles : super::super::Foundation:: BOOL , dwcreationflags : PROCESS_CREATION_FLAGS , lpenvironment : *const ::core::ffi::c_void , lpcurrentdirectory : ::windows_sys::core::PCSTR , lpstartupinfo : *const STARTUPINFOA , lpprocessinformation : *mut PROCESS_INFORMATION ) -> super::super::Foundation:: BOOL );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link ! ( "advapi32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn CreateProcessAsUserA ( htoken : super::super::Foundation:: HANDLE , lpapplicationname : ::windows_sys::core::PCSTR , lpcommandline : ::windows_sys::core::PSTR , lpprocessattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , lpthreadattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , binherithandles : super::super::Foundation:: BOOL , dwcreationflags : PROCESS_CREATION_FLAGS , lpenvironment : *const ::core::ffi::c_void , lpcurrentdirectory : ::windows_sys::core::PCSTR , lpstartupinfo : *const STARTUPINFOA , lpprocessinformation : *mut PROCESS_INFORMATION ) -> super::super::Foundation:: BOOL );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link ! ( "advapi32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn CreateProcessAsUserW ( htoken : super::super::Foundation:: HANDLE , lpapplicationname : ::windows_sys::core::PCWSTR , lpcommandline : ::windows_sys::core::PWSTR , lpprocessattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , lpthreadattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , binherithandles : super::super::Foundation:: BOOL , dwcreationflags : PROCESS_CREATION_FLAGS , lpenvironment : *const ::core::ffi::c_void , lpcurrentdirectory : ::windows_sys::core::PCWSTR , lpstartupinfo : *const STARTUPINFOW , lpprocessinformation : *mut PROCESS_INFORMATION ) -> super::super::Foundation:: BOOL );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn CreateProcessW ( lpapplicationname : ::windows_sys::core::PCWSTR , lpcommandline : ::windows_sys::core::PWSTR , lpprocessattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , lpthreadattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , binherithandles : super::super::Foundation:: BOOL , dwcreationflags : PROCESS_CREATION_FLAGS , lpenvironment : *const ::core::ffi::c_void , lpcurrentdirectory : ::windows_sys::core::PCWSTR , lpstartupinfo : *const STARTUPINFOW , lpprocessinformation : *mut PROCESS_INFORMATION ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "advapi32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn CreateProcessWithLogonW ( lpusername : ::windows_sys::core::PCWSTR , lpdomain : ::windows_sys::core::PCWSTR , lppassword : ::windows_sys::core::PCWSTR , dwlogonflags : CREATE_PROCESS_LOGON_FLAGS , lpapplicationname : ::windows_sys::core::PCWSTR , lpcommandline : ::windows_sys::core::PWSTR , dwcreationflags : PROCESS_CREATION_FLAGS , lpenvironment : *const ::core::ffi::c_void , lpcurrentdirectory : ::windows_sys::core::PCWSTR , lpstartupinfo : *const STARTUPINFOW , lpprocessinformation : *mut PROCESS_INFORMATION ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "advapi32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn CreateProcessWithTokenW ( htoken : super::super::Foundation:: HANDLE , dwlogonflags : CREATE_PROCESS_LOGON_FLAGS , lpapplicationname : ::windows_sys::core::PCWSTR , lpcommandline : ::windows_sys::core::PWSTR , dwcreationflags : PROCESS_CREATION_FLAGS , lpenvironment : *const ::core::ffi::c_void , lpcurrentdirectory : ::windows_sys::core::PCWSTR , lpstartupinfo : *const STARTUPINFOW , lpprocessinformation : *mut PROCESS_INFORMATION ) -> super::super::Foundation:: BOOL );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn CreateRemoteThread ( hprocess : super::super::Foundation:: HANDLE , lpthreadattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , dwstacksize : usize , lpstartaddress : LPTHREAD_START_ROUTINE , lpparameter : *const ::core::ffi::c_void , dwcreationflags : u32 , lpthreadid : *mut u32 ) -> super::super::Foundation:: HANDLE );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn CreateRemoteThreadEx ( hprocess : super::super::Foundation:: HANDLE , lpthreadattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , dwstacksize : usize , lpstartaddress : LPTHREAD_START_ROUTINE , lpparameter : *const ::core::ffi::c_void , dwcreationflags : u32 , lpattributelist : LPPROC_THREAD_ATTRIBUTE_LIST , lpthreadid : *mut u32 ) -> super::super::Foundation:: HANDLE );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn CreateSemaphoreA ( lpsemaphoreattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , linitialcount : i32 , lmaximumcount : i32 , lpname : ::windows_sys::core::PCSTR ) -> super::super::Foundation:: HANDLE );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn CreateSemaphoreExA ( lpsemaphoreattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , linitialcount : i32 , lmaximumcount : i32 , lpname : ::windows_sys::core::PCSTR , dwflags : u32 , dwdesiredaccess : u32 ) -> super::super::Foundation:: HANDLE );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn CreateSemaphoreExW ( lpsemaphoreattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , linitialcount : i32 , lmaximumcount : i32 , lpname : ::windows_sys::core::PCWSTR , dwflags : u32 , dwdesiredaccess : u32 ) -> super::super::Foundation:: HANDLE );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn CreateSemaphoreW ( lpsemaphoreattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , linitialcount : i32 , lmaximumcount : i32 , lpname : ::windows_sys::core::PCWSTR ) -> super::super::Foundation:: HANDLE );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn CreateThread ( lpthreadattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , dwstacksize : usize , lpstartaddress : LPTHREAD_START_ROUTINE , lpparameter : *const ::core::ffi::c_void , dwcreationflags : THREAD_CREATION_FLAGS , lpthreadid : *mut u32 ) -> super::super::Foundation:: HANDLE );
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`*"] fn CreateThreadpool ( reserved : *const ::core::ffi::c_void ) -> PTP_POOL );
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`*"] fn CreateThreadpoolCleanupGroup ( ) -> isize );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn CreateThreadpoolIo ( fl : super::super::Foundation:: HANDLE , pfnio : PTP_WIN32_IO_CALLBACK , pv : *mut ::core::ffi::c_void , pcbe : *const TP_CALLBACK_ENVIRON_V3 ) -> PTP_IO );
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`*"] fn CreateThreadpoolTimer ( pfnti : PTP_TIMER_CALLBACK , pv : *mut ::core::ffi::c_void , pcbe : *const TP_CALLBACK_ENVIRON_V3 ) -> PTP_TIMER );
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`*"] fn CreateThreadpoolWait ( pfnwa : PTP_WAIT_CALLBACK , pv : *mut ::core::ffi::c_void , pcbe : *const TP_CALLBACK_ENVIRON_V3 ) -> PTP_WAIT );
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`*"] fn CreateThreadpoolWork ( pfnwk : PTP_WORK_CALLBACK , pv : *mut ::core::ffi::c_void , pcbe : *const TP_CALLBACK_ENVIRON_V3 ) -> PTP_WORK );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn CreateTimerQueue ( ) -> super::super::Foundation:: HANDLE );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn CreateTimerQueueTimer ( phnewtimer : *mut super::super::Foundation:: HANDLE , timerqueue : super::super::Foundation:: HANDLE , callback : WAITORTIMERCALLBACK , parameter : *const ::core::ffi::c_void , duetime : u32 , period : u32 , flags : WORKER_THREAD_FLAGS ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn CreateUmsCompletionList ( umscompletionlist : *mut *mut ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn CreateUmsThreadContext ( lpumsthread : *mut *mut ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn CreateWaitableTimerExW ( lptimerattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , lptimername : ::windows_sys::core::PCWSTR , dwflags : u32 , dwdesiredaccess : u32 ) -> super::super::Foundation:: HANDLE );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn CreateWaitableTimerW ( lptimerattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , bmanualreset : super::super::Foundation:: BOOL , lptimername : ::windows_sys::core::PCWSTR ) -> super::super::Foundation:: HANDLE );
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`*"] fn DeleteBoundaryDescriptor ( boundarydescriptor : BoundaryDescriptorHandle ) -> ( ) );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn DeleteCriticalSection ( lpcriticalsection : *mut RTL_CRITICAL_SECTION ) -> ( ) );
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`*"] fn DeleteFiber ( lpfiber : *const ::core::ffi::c_void ) -> ( ) );
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`*"] fn DeleteProcThreadAttributeList ( lpattributelist : LPPROC_THREAD_ATTRIBUTE_LIST ) -> ( ) );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn DeleteSynchronizationBarrier ( lpbarrier : *mut RTL_BARRIER ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn DeleteTimerQueue ( timerqueue : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn DeleteTimerQueueEx ( timerqueue : super::super::Foundation:: HANDLE , completionevent : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn DeleteTimerQueueTimer ( timerqueue : super::super::Foundation:: HANDLE , timer : super::super::Foundation:: HANDLE , completionevent : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn DeleteUmsCompletionList ( umscompletionlist : *const ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn DeleteUmsThreadContext ( umsthread : *const ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn DequeueUmsCompletionListItems ( umscompletionlist : *const ::core::ffi::c_void , waittimeout : u32 , umsthreadlist : *mut *mut ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`*"] fn DisassociateCurrentThreadFromCallback ( pci : PTP_CALLBACK_INSTANCE ) -> ( ) );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn EnterCriticalSection ( lpcriticalsection : *mut RTL_CRITICAL_SECTION ) -> ( ) );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn EnterSynchronizationBarrier ( lpbarrier : *mut RTL_BARRIER , dwflags : u32 ) -> super::super::Foundation:: BOOL );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_SystemServices\"`*"] fn EnterUmsSchedulingMode ( schedulerstartupinfo : *const UMS_SCHEDULER_STARTUP_INFO ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn ExecuteUmsThread ( umsthread : *mut ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`*"] fn ExitProcess ( uexitcode : u32 ) -> ! );
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`*"] fn ExitThread ( dwexitcode : u32 ) -> ! );
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`*"] fn FlsAlloc ( lpcallback : PFLS_CALLBACK_FUNCTION ) -> u32 );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn FlsFree ( dwflsindex : u32 ) -> super::super::Foundation:: BOOL );
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`*"] fn FlsGetValue ( dwflsindex : u32 ) -> *mut ::core::ffi::c_void );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn FlsSetValue ( dwflsindex : u32 , lpflsdata : *const ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`*"] fn FlushProcessWriteBuffers ( ) -> ( ) );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn FreeLibraryWhenCallbackReturns ( pci : PTP_CALLBACK_INSTANCE , r#mod : super::super::Foundation:: HMODULE ) -> ( ) );
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`*"] fn GetActiveProcessorCount ( groupnumber : u16 ) -> u32 );
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`*"] fn GetActiveProcessorGroupCount ( ) -> u16 );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn GetCurrentProcess ( ) -> super::super::Foundation:: HANDLE );
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`*"] fn GetCurrentProcessId ( ) -> u32 );
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`*"] fn GetCurrentProcessorNumber ( ) -> u32 );
#[cfg(feature = "Win32_System_Kernel")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_System_Kernel\"`*"] fn GetCurrentProcessorNumberEx ( procnumber : *mut super::Kernel:: PROCESSOR_NUMBER ) -> ( ) );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn GetCurrentThread ( ) -> super::super::Foundation:: HANDLE );
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`*"] fn GetCurrentThreadId ( ) -> u32 );
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`*"] fn GetCurrentThreadStackLimits ( lowlimit : *mut usize , highlimit : *mut usize ) -> ( ) );
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`*"] fn GetCurrentUmsThread ( ) -> *mut ::core::ffi::c_void );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn GetExitCodeProcess ( hprocess : super::super::Foundation:: HANDLE , lpexitcode : *mut u32 ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn GetExitCodeThread ( hthread : super::super::Foundation:: HANDLE , lpexitcode : *mut u32 ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "user32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn GetGuiResources ( hprocess : super::super::Foundation:: HANDLE , uiflags : GET_GUI_RESOURCES_FLAGS ) -> u32 );
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`*"] fn GetMachineTypeAttributes ( machine : u16 , machinetypeattributes : *mut MACHINE_ATTRIBUTES ) -> ::windows_sys::core::HRESULT );
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`*"] fn GetMaximumProcessorCount ( groupnumber : u16 ) -> u32 );
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`*"] fn GetMaximumProcessorGroupCount ( ) -> u16 );
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`*"] fn GetNextUmsListItem ( umscontext : *mut ::core::ffi::c_void ) -> *mut ::core::ffi::c_void );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn GetNumaAvailableMemoryNode ( node : u8 , availablebytes : *mut u64 ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn GetNumaAvailableMemoryNodeEx ( node : u16 , availablebytes : *mut u64 ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn GetNumaHighestNodeNumber ( highestnodenumber : *mut u32 ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn GetNumaNodeNumberFromHandle ( hfile : super::super::Foundation:: HANDLE , nodenumber : *mut u16 ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn GetNumaNodeProcessorMask ( node : u8 , processormask : *mut u64 ) -> super::super::Foundation:: BOOL );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemInformation"))]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_SystemInformation\"`*"] fn GetNumaNodeProcessorMask2 ( nodenumber : u16 , processormasks : *mut super::SystemInformation:: GROUP_AFFINITY , processormaskcount : u16 , requiredmaskcount : *mut u16 ) -> super::super::Foundation:: BOOL );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemInformation"))]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_SystemInformation\"`*"] fn GetNumaNodeProcessorMaskEx ( node : u16 , processormask : *mut super::SystemInformation:: GROUP_AFFINITY ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn GetNumaProcessorNode ( processor : u8 , nodenumber : *mut u8 ) -> super::super::Foundation:: BOOL );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn GetNumaProcessorNodeEx ( processor : *const super::Kernel:: PROCESSOR_NUMBER , nodenumber : *mut u16 ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn GetNumaProximityNode ( proximityid : u32 , nodenumber : *mut u8 ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn GetNumaProximityNodeEx ( proximityid : u32 , nodenumber : *mut u16 ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn GetPriorityClass ( hprocess : super::super::Foundation:: HANDLE ) -> u32 );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn GetProcessAffinityMask ( hprocess : super::super::Foundation:: HANDLE , lpprocessaffinitymask : *mut usize , lpsystemaffinitymask : *mut usize ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn GetProcessDEPPolicy ( hprocess : super::super::Foundation:: HANDLE , lpflags : *mut u32 , lppermanent : *mut super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemInformation"))]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_SystemInformation\"`*"] fn GetProcessDefaultCpuSetMasks ( process : super::super::Foundation:: HANDLE , cpusetmasks : *mut super::SystemInformation:: GROUP_AFFINITY , cpusetmaskcount : u16 , requiredmaskcount : *mut u16 ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn GetProcessDefaultCpuSets ( process : super::super::Foundation:: HANDLE , cpusetids : *mut u32 , cpusetidcount : u32 , requiredidcount : *mut u32 ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn GetProcessGroupAffinity ( hprocess : super::super::Foundation:: HANDLE , groupcount : *mut u16 , grouparray : *mut u16 ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn GetProcessHandleCount ( hprocess : super::super::Foundation:: HANDLE , pdwhandlecount : *mut u32 ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn GetProcessId ( process : super::super::Foundation:: HANDLE ) -> u32 );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn GetProcessIdOfThread ( thread : super::super::Foundation:: HANDLE ) -> u32 );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn GetProcessInformation ( hprocess : super::super::Foundation:: HANDLE , processinformationclass : PROCESS_INFORMATION_CLASS , processinformation : *mut ::core::ffi::c_void , processinformationsize : u32 ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn GetProcessIoCounters ( hprocess : super::super::Foundation:: HANDLE , lpiocounters : *mut IO_COUNTERS ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn GetProcessMitigationPolicy ( hprocess : super::super::Foundation:: HANDLE , mitigationpolicy : PROCESS_MITIGATION_POLICY , lpbuffer : *mut ::core::ffi::c_void , dwlength : usize ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn GetProcessPriorityBoost ( hprocess : super::super::Foundation:: HANDLE , pdisablepriorityboost : *mut super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn GetProcessShutdownParameters ( lpdwlevel : *mut u32 , lpdwflags : *mut u32 ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn GetProcessTimes ( hprocess : super::super::Foundation:: HANDLE , lpcreationtime : *mut super::super::Foundation:: FILETIME , lpexittime : *mut super::super::Foundation:: FILETIME , lpkerneltime : *mut super::super::Foundation:: FILETIME , lpusertime : *mut super::super::Foundation:: FILETIME ) -> super::super::Foundation:: BOOL );
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`*"] fn GetProcessVersion ( processid : u32 ) -> u32 );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn GetProcessWorkingSetSize ( hprocess : super::super::Foundation:: HANDLE , lpminimumworkingsetsize : *mut usize , lpmaximumworkingsetsize : *mut usize ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn GetStartupInfoA ( lpstartupinfo : *mut STARTUPINFOA ) -> ( ) );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn GetStartupInfoW ( lpstartupinfo : *mut STARTUPINFOW ) -> ( ) );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn GetSystemTimes ( lpidletime : *mut super::super::Foundation:: FILETIME , lpkerneltime : *mut super::super::Foundation:: FILETIME , lpusertime : *mut super::super::Foundation:: FILETIME ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn GetThreadDescription ( hthread : super::super::Foundation:: HANDLE , ppszthreaddescription : *mut ::windows_sys::core::PWSTR ) -> ::windows_sys::core::HRESULT );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemInformation"))]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_SystemInformation\"`*"] fn GetThreadGroupAffinity ( hthread : super::super::Foundation:: HANDLE , groupaffinity : *mut super::SystemInformation:: GROUP_AFFINITY ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn GetThreadIOPendingFlag ( hthread : super::super::Foundation:: HANDLE , lpioispending : *mut super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn GetThreadId ( thread : super::super::Foundation:: HANDLE ) -> u32 );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn GetThreadIdealProcessorEx ( hthread : super::super::Foundation:: HANDLE , lpidealprocessor : *mut super::Kernel:: PROCESSOR_NUMBER ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn GetThreadInformation ( hthread : super::super::Foundation:: HANDLE , threadinformationclass : THREAD_INFORMATION_CLASS , threadinformation : *mut ::core::ffi::c_void , threadinformationsize : u32 ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn GetThreadPriority ( hthread : super::super::Foundation:: HANDLE ) -> i32 );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn GetThreadPriorityBoost ( hthread : super::super::Foundation:: HANDLE , pdisablepriorityboost : *mut super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemInformation"))]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_SystemInformation\"`*"] fn GetThreadSelectedCpuSetMasks ( thread : super::super::Foundation:: HANDLE , cpusetmasks : *mut super::SystemInformation:: GROUP_AFFINITY , cpusetmaskcount : u16 , requiredmaskcount : *mut u16 ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn GetThreadSelectedCpuSets ( thread : super::super::Foundation:: HANDLE , cpusetids : *mut u32 , cpusetidcount : u32 , requiredidcount : *mut u32 ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn GetThreadTimes ( hthread : super::super::Foundation:: HANDLE , lpcreationtime : *mut super::super::Foundation:: FILETIME , lpexittime : *mut super::super::Foundation:: FILETIME , lpkerneltime : *mut super::super::Foundation:: FILETIME , lpusertime : *mut super::super::Foundation:: FILETIME ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn GetUmsCompletionListEvent ( umscompletionlist : *const ::core::ffi::c_void , umscompletionevent : *mut super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn GetUmsSystemThreadInformation ( threadhandle : super::super::Foundation:: HANDLE , systemthreadinfo : *mut UMS_SYSTEM_THREAD_INFORMATION ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn InitOnceBeginInitialize ( lpinitonce : *mut RTL_RUN_ONCE , dwflags : u32 , fpending : *mut super::super::Foundation:: BOOL , lpcontext : *mut *mut ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn InitOnceComplete ( lpinitonce : *mut RTL_RUN_ONCE , dwflags : u32 , lpcontext : *const ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn InitOnceExecuteOnce ( initonce : *mut RTL_RUN_ONCE , initfn : PINIT_ONCE_FN , parameter : *mut ::core::ffi::c_void , context : *mut *mut ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`*"] fn InitOnceInitialize ( initonce : *mut RTL_RUN_ONCE ) -> ( ) );
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`*"] fn InitializeConditionVariable ( conditionvariable : *mut RTL_CONDITION_VARIABLE ) -> ( ) );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn InitializeCriticalSection ( lpcriticalsection : *mut RTL_CRITICAL_SECTION ) -> ( ) );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn InitializeCriticalSectionAndSpinCount ( lpcriticalsection : *mut RTL_CRITICAL_SECTION , dwspincount : u32 ) -> super::super::Foundation:: BOOL );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn InitializeCriticalSectionEx ( lpcriticalsection : *mut RTL_CRITICAL_SECTION , dwspincount : u32 , flags : u32 ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn InitializeProcThreadAttributeList ( lpattributelist : LPPROC_THREAD_ATTRIBUTE_LIST , dwattributecount : u32 , dwflags : u32 , lpsize : *mut usize ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_System_Kernel")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_System_Kernel\"`*"] fn InitializeSListHead ( listhead : *mut super::Kernel:: SLIST_HEADER ) -> ( ) );
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`*"] fn InitializeSRWLock ( srwlock : *mut RTL_SRWLOCK ) -> ( ) );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn InitializeSynchronizationBarrier ( lpbarrier : *mut RTL_BARRIER , ltotalthreads : i32 , lspincount : i32 ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_System_Kernel")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_System_Kernel\"`*"] fn InterlockedFlushSList ( listhead : *mut super::Kernel:: SLIST_HEADER ) -> *mut super::Kernel:: SLIST_ENTRY );
#[cfg(feature = "Win32_System_Kernel")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_System_Kernel\"`*"] fn InterlockedPopEntrySList ( listhead : *mut super::Kernel:: SLIST_HEADER ) -> *mut super::Kernel:: SLIST_ENTRY );
#[cfg(feature = "Win32_System_Kernel")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_System_Kernel\"`*"] fn InterlockedPushEntrySList ( listhead : *mut super::Kernel:: SLIST_HEADER , listentry : *mut super::Kernel:: SLIST_ENTRY ) -> *mut super::Kernel:: SLIST_ENTRY );
#[cfg(feature = "Win32_System_Kernel")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_System_Kernel\"`*"] fn InterlockedPushListSListEx ( listhead : *mut super::Kernel:: SLIST_HEADER , list : *mut super::Kernel:: SLIST_ENTRY , listend : *mut super::Kernel:: SLIST_ENTRY , count : u32 ) -> *mut super::Kernel:: SLIST_ENTRY );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "user32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn IsImmersiveProcess ( hprocess : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn IsProcessCritical ( hprocess : super::super::Foundation:: HANDLE , critical : *mut super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn IsProcessorFeaturePresent ( processorfeature : PROCESSOR_FEATURE_ID ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn IsThreadAFiber ( ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn IsThreadpoolTimerSet ( pti : PTP_TIMER ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn IsWow64Process ( hprocess : super::super::Foundation:: HANDLE , wow64process : *mut super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemInformation"))]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_SystemInformation\"`*"] fn IsWow64Process2 ( hprocess : super::super::Foundation:: HANDLE , pprocessmachine : *mut super::SystemInformation:: IMAGE_FILE_MACHINE , pnativemachine : *mut super::SystemInformation:: IMAGE_FILE_MACHINE ) -> super::super::Foundation:: BOOL );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn LeaveCriticalSection ( lpcriticalsection : *mut RTL_CRITICAL_SECTION ) -> ( ) );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn LeaveCriticalSectionWhenCallbackReturns ( pci : PTP_CALLBACK_INSTANCE , pcs : *mut RTL_CRITICAL_SECTION ) -> ( ) );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "ntdll.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn NtQueryInformationProcess ( processhandle : super::super::Foundation:: HANDLE , processinformationclass : PROCESSINFOCLASS , processinformation : *mut ::core::ffi::c_void , processinformationlength : u32 , returnlength : *mut u32 ) -> super::super::Foundation:: NTSTATUS );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "ntdll.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn NtQueryInformationThread ( threadhandle : super::super::Foundation:: HANDLE , threadinformationclass : THREADINFOCLASS , threadinformation : *mut ::core::ffi::c_void , threadinformationlength : u32 , returnlength : *mut u32 ) -> super::super::Foundation:: NTSTATUS );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "ntdll.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn NtSetInformationThread ( threadhandle : super::super::Foundation:: HANDLE , threadinformationclass : THREADINFOCLASS , threadinformation : *const ::core::ffi::c_void , threadinformationlength : u32 ) -> super::super::Foundation:: NTSTATUS );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn OpenEventA ( dwdesiredaccess : SYNCHRONIZATION_ACCESS_RIGHTS , binherithandle : super::super::Foundation:: BOOL , lpname : ::windows_sys::core::PCSTR ) -> super::super::Foundation:: HANDLE );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn OpenEventW ( dwdesiredaccess : SYNCHRONIZATION_ACCESS_RIGHTS , binherithandle : super::super::Foundation:: BOOL , lpname : ::windows_sys::core::PCWSTR ) -> super::super::Foundation:: HANDLE );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn OpenMutexW ( dwdesiredaccess : SYNCHRONIZATION_ACCESS_RIGHTS , binherithandle : super::super::Foundation:: BOOL , lpname : ::windows_sys::core::PCWSTR ) -> super::super::Foundation:: HANDLE );
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`*"] fn OpenPrivateNamespaceA ( lpboundarydescriptor : *const ::core::ffi::c_void , lpaliasprefix : ::windows_sys::core::PCSTR ) -> NamespaceHandle );
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`*"] fn OpenPrivateNamespaceW ( lpboundarydescriptor : *const ::core::ffi::c_void , lpaliasprefix : ::windows_sys::core::PCWSTR ) -> NamespaceHandle );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn OpenProcess ( dwdesiredaccess : PROCESS_ACCESS_RIGHTS , binherithandle : super::super::Foundation:: BOOL , dwprocessid : u32 ) -> super::super::Foundation:: HANDLE );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link ! ( "advapi32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn OpenProcessToken ( processhandle : super::super::Foundation:: HANDLE , desiredaccess : super::super::Security:: TOKEN_ACCESS_MASK , tokenhandle : *mut super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn OpenSemaphoreW ( dwdesiredaccess : SYNCHRONIZATION_ACCESS_RIGHTS , binherithandle : super::super::Foundation:: BOOL , lpname : ::windows_sys::core::PCWSTR ) -> super::super::Foundation:: HANDLE );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn OpenThread ( dwdesiredaccess : THREAD_ACCESS_RIGHTS , binherithandle : super::super::Foundation:: BOOL , dwthreadid : u32 ) -> super::super::Foundation:: HANDLE );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
::windows_targets::link ! ( "advapi32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"] fn OpenThreadToken ( threadhandle : super::super::Foundation:: HANDLE , desiredaccess : super::super::Security:: TOKEN_ACCESS_MASK , openasself : super::super::Foundation:: BOOL , tokenhandle : *mut super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn OpenWaitableTimerW ( dwdesiredaccess : SYNCHRONIZATION_ACCESS_RIGHTS , binherithandle : super::super::Foundation:: BOOL , lptimername : ::windows_sys::core::PCWSTR ) -> super::super::Foundation:: HANDLE );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn PulseEvent ( hevent : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_System_Kernel")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_System_Kernel\"`*"] fn QueryDepthSList ( listhead : *const super::Kernel:: SLIST_HEADER ) -> u16 );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn QueryFullProcessImageNameA ( hprocess : super::super::Foundation:: HANDLE , dwflags : PROCESS_NAME_FORMAT , lpexename : ::windows_sys::core::PSTR , lpdwsize : *mut u32 ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn QueryFullProcessImageNameW ( hprocess : super::super::Foundation:: HANDLE , dwflags : PROCESS_NAME_FORMAT , lpexename : ::windows_sys::core::PWSTR , lpdwsize : *mut u32 ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn QueryProcessAffinityUpdateMode ( hprocess : super::super::Foundation:: HANDLE , lpdwflags : *mut PROCESS_AFFINITY_AUTO_UPDATE_FLAGS ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn QueryProtectedPolicy ( policyguid : *const ::windows_sys::core::GUID , policyvalue : *mut usize ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn QueryThreadpoolStackInformation ( ptpp : PTP_POOL , ptpsi : *mut TP_POOL_STACK_INFORMATION ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn QueryUmsThreadInformation ( umsthread : *const ::core::ffi::c_void , umsthreadinfoclass : RTL_UMS_THREAD_INFO_CLASS , umsthreadinformation : *mut ::core::ffi::c_void , umsthreadinformationlength : u32 , returnlength : *mut u32 ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn QueueUserAPC ( pfnapc : super::super::Foundation:: PAPCFUNC , hthread : super::super::Foundation:: HANDLE , dwdata : usize ) -> u32 );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn QueueUserAPC2 ( apcroutine : super::super::Foundation:: PAPCFUNC , thread : super::super::Foundation:: HANDLE , data : usize , flags : QUEUE_USER_APC_FLAGS ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn QueueUserWorkItem ( function : LPTHREAD_START_ROUTINE , context : *const ::core::ffi::c_void , flags : WORKER_THREAD_FLAGS ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn RegisterWaitForSingleObject ( phnewwaitobject : *mut super::super::Foundation:: HANDLE , hobject : super::super::Foundation:: HANDLE , callback : WAITORTIMERCALLBACK , context : *const ::core::ffi::c_void , dwmilliseconds : u32 , dwflags : WORKER_THREAD_FLAGS ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn ReleaseMutex ( hmutex : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn ReleaseMutexWhenCallbackReturns ( pci : PTP_CALLBACK_INSTANCE , r#mut : super::super::Foundation:: HANDLE ) -> ( ) );
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`*"] fn ReleaseSRWLockExclusive ( srwlock : *mut RTL_SRWLOCK ) -> ( ) );
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`*"] fn ReleaseSRWLockShared ( srwlock : *mut RTL_SRWLOCK ) -> ( ) );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn ReleaseSemaphore ( hsemaphore : super::super::Foundation:: HANDLE , lreleasecount : i32 , lppreviouscount : *mut i32 ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn ReleaseSemaphoreWhenCallbackReturns ( pci : PTP_CALLBACK_INSTANCE , sem : super::super::Foundation:: HANDLE , crel : u32 ) -> ( ) );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn ResetEvent ( hevent : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn ResumeThread ( hthread : super::super::Foundation:: HANDLE ) -> u32 );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn SetCriticalSectionSpinCount ( lpcriticalsection : *mut RTL_CRITICAL_SECTION , dwspincount : u32 ) -> u32 );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn SetEvent ( hevent : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn SetEventWhenCallbackReturns ( pci : PTP_CALLBACK_INSTANCE , evt : super::super::Foundation:: HANDLE ) -> ( ) );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn SetPriorityClass ( hprocess : super::super::Foundation:: HANDLE , dwpriorityclass : PROCESS_CREATION_FLAGS ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn SetProcessAffinityMask ( hprocess : super::super::Foundation:: HANDLE , dwprocessaffinitymask : usize ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn SetProcessAffinityUpdateMode ( hprocess : super::super::Foundation:: HANDLE , dwflags : PROCESS_AFFINITY_AUTO_UPDATE_FLAGS ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn SetProcessDEPPolicy ( dwflags : PROCESS_DEP_FLAGS ) -> super::super::Foundation:: BOOL );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemInformation"))]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_SystemInformation\"`*"] fn SetProcessDefaultCpuSetMasks ( process : super::super::Foundation:: HANDLE , cpusetmasks : *const super::SystemInformation:: GROUP_AFFINITY , cpusetmaskcount : u16 ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn SetProcessDefaultCpuSets ( process : super::super::Foundation:: HANDLE , cpusetids : *const u32 , cpusetidcount : u32 ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn SetProcessDynamicEHContinuationTargets ( process : super::super::Foundation:: HANDLE , numberoftargets : u16 , targets : *mut PROCESS_DYNAMIC_EH_CONTINUATION_TARGET ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn SetProcessDynamicEnforcedCetCompatibleRanges ( process : super::super::Foundation:: HANDLE , numberofranges : u16 , ranges : *mut PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn SetProcessInformation ( hprocess : super::super::Foundation:: HANDLE , processinformationclass : PROCESS_INFORMATION_CLASS , processinformation : *const ::core::ffi::c_void , processinformationsize : u32 ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn SetProcessMitigationPolicy ( mitigationpolicy : PROCESS_MITIGATION_POLICY , lpbuffer : *const ::core::ffi::c_void , dwlength : usize ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn SetProcessPriorityBoost ( hprocess : super::super::Foundation:: HANDLE , bdisablepriorityboost : super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "user32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn SetProcessRestrictionExemption ( fenableexemption : super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn SetProcessShutdownParameters ( dwlevel : u32 , dwflags : u32 ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn SetProcessWorkingSetSize ( hprocess : super::super::Foundation:: HANDLE , dwminimumworkingsetsize : usize , dwmaximumworkingsetsize : usize ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn SetProtectedPolicy ( policyguid : *const ::windows_sys::core::GUID , policyvalue : usize , oldpolicyvalue : *mut usize ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn SetThreadAffinityMask ( hthread : super::super::Foundation:: HANDLE , dwthreadaffinitymask : usize ) -> usize );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn SetThreadDescription ( hthread : super::super::Foundation:: HANDLE , lpthreaddescription : ::windows_sys::core::PCWSTR ) -> ::windows_sys::core::HRESULT );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemInformation"))]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_SystemInformation\"`*"] fn SetThreadGroupAffinity ( hthread : super::super::Foundation:: HANDLE , groupaffinity : *const super::SystemInformation:: GROUP_AFFINITY , previousgroupaffinity : *mut super::SystemInformation:: GROUP_AFFINITY ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn SetThreadIdealProcessor ( hthread : super::super::Foundation:: HANDLE , dwidealprocessor : u32 ) -> u32 );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn SetThreadIdealProcessorEx ( hthread : super::super::Foundation:: HANDLE , lpidealprocessor : *const super::Kernel:: PROCESSOR_NUMBER , lppreviousidealprocessor : *mut super::Kernel:: PROCESSOR_NUMBER ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn SetThreadInformation ( hthread : super::super::Foundation:: HANDLE , threadinformationclass : THREAD_INFORMATION_CLASS , threadinformation : *const ::core::ffi::c_void , threadinformationsize : u32 ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn SetThreadPriority ( hthread : super::super::Foundation:: HANDLE , npriority : THREAD_PRIORITY ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn SetThreadPriorityBoost ( hthread : super::super::Foundation:: HANDLE , bdisablepriorityboost : super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemInformation"))]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_SystemInformation\"`*"] fn SetThreadSelectedCpuSetMasks ( thread : super::super::Foundation:: HANDLE , cpusetmasks : *const super::SystemInformation:: GROUP_AFFINITY , cpusetmaskcount : u16 ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn SetThreadSelectedCpuSets ( thread : super::super::Foundation:: HANDLE , cpusetids : *const u32 , cpusetidcount : u32 ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn SetThreadStackGuarantee ( stacksizeinbytes : *mut u32 ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "advapi32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn SetThreadToken ( thread : *const super::super::Foundation:: HANDLE , token : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn SetThreadpoolStackInformation ( ptpp : PTP_POOL , ptpsi : *const TP_POOL_STACK_INFORMATION ) -> super::super::Foundation:: BOOL );
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`*"] fn SetThreadpoolThreadMaximum ( ptpp : PTP_POOL , cthrdmost : u32 ) -> ( ) );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn SetThreadpoolThreadMinimum ( ptpp : PTP_POOL , cthrdmic : u32 ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn SetThreadpoolTimer ( pti : PTP_TIMER , pftduetime : *const super::super::Foundation:: FILETIME , msperiod : u32 , mswindowlength : u32 ) -> ( ) );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn SetThreadpoolTimerEx ( pti : PTP_TIMER , pftduetime : *const super::super::Foundation:: FILETIME , msperiod : u32 , mswindowlength : u32 ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn SetThreadpoolWait ( pwa : PTP_WAIT , h : super::super::Foundation:: HANDLE , pfttimeout : *const super::super::Foundation:: FILETIME ) -> ( ) );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn SetThreadpoolWaitEx ( pwa : PTP_WAIT , h : super::super::Foundation:: HANDLE , pfttimeout : *const super::super::Foundation:: FILETIME , reserved : *const ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn SetTimerQueueTimer ( timerqueue : super::super::Foundation:: HANDLE , callback : WAITORTIMERCALLBACK , parameter : *const ::core::ffi::c_void , duetime : u32 , period : u32 , preferio : super::super::Foundation:: BOOL ) -> super::super::Foundation:: HANDLE );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn SetUmsThreadInformation ( umsthread : *const ::core::ffi::c_void , umsthreadinfoclass : RTL_UMS_THREAD_INFO_CLASS , umsthreadinformation : *const ::core::ffi::c_void , umsthreadinformationlength : u32 ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn SetWaitableTimer ( htimer : super::super::Foundation:: HANDLE , lpduetime : *const i64 , lperiod : i32 , pfncompletionroutine : PTIMERAPCROUTINE , lpargtocompletionroutine : *const ::core::ffi::c_void , fresume : super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn SetWaitableTimerEx ( htimer : super::super::Foundation:: HANDLE , lpduetime : *const i64 , lperiod : i32 , pfncompletionroutine : PTIMERAPCROUTINE , lpargtocompletionroutine : *const ::core::ffi::c_void , wakecontext : *const REASON_CONTEXT , tolerabledelay : u32 ) -> super::super::Foundation:: BOOL );
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`*"] fn Sleep ( dwmilliseconds : u32 ) -> ( ) );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn SleepConditionVariableCS ( conditionvariable : *mut RTL_CONDITION_VARIABLE , criticalsection : *mut RTL_CRITICAL_SECTION , dwmilliseconds : u32 ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn SleepConditionVariableSRW ( conditionvariable : *mut RTL_CONDITION_VARIABLE , srwlock : *mut RTL_SRWLOCK , dwmilliseconds : u32 , flags : u32 ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn SleepEx ( dwmilliseconds : u32 , balertable : super::super::Foundation:: BOOL ) -> u32 );
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`*"] fn StartThreadpoolIo ( pio : PTP_IO ) -> ( ) );
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`*"] fn SubmitThreadpoolWork ( pwk : PTP_WORK ) -> ( ) );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn SuspendThread ( hthread : super::super::Foundation:: HANDLE ) -> u32 );
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`*"] fn SwitchToFiber ( lpfiber : *const ::core::ffi::c_void ) -> ( ) );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn SwitchToThread ( ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn TerminateProcess ( hprocess : super::super::Foundation:: HANDLE , uexitcode : u32 ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn TerminateThread ( hthread : super::super::Foundation:: HANDLE , dwexitcode : u32 ) -> super::super::Foundation:: BOOL );
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`*"] fn TlsAlloc ( ) -> u32 );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn TlsFree ( dwtlsindex : u32 ) -> super::super::Foundation:: BOOL );
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`*"] fn TlsGetValue ( dwtlsindex : u32 ) -> *mut ::core::ffi::c_void );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn TlsSetValue ( dwtlsindex : u32 , lptlsvalue : *const ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn TryAcquireSRWLockExclusive ( srwlock : *mut RTL_SRWLOCK ) -> super::super::Foundation:: BOOLEAN );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn TryAcquireSRWLockShared ( srwlock : *mut RTL_SRWLOCK ) -> super::super::Foundation:: BOOLEAN );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"] fn TryEnterCriticalSection ( lpcriticalsection : *mut RTL_CRITICAL_SECTION ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn TrySubmitThreadpoolCallback ( pfns : PTP_SIMPLE_CALLBACK , pv : *mut ::core::ffi::c_void , pcbe : *const TP_CALLBACK_ENVIRON_V3 ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn UmsThreadYield ( schedulerparam : *const ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn UnregisterWait ( waithandle : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn UnregisterWaitEx ( waithandle : super::super::Foundation:: HANDLE , completionevent : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn UpdateProcThreadAttribute ( lpattributelist : LPPROC_THREAD_ATTRIBUTE_LIST , dwflags : u32 , attribute : usize , lpvalue : *const ::core::ffi::c_void , cbsize : usize , lppreviousvalue : *mut ::core::ffi::c_void , lpreturnsize : *const usize ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "user32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn WaitForInputIdle ( hprocess : super::super::Foundation:: HANDLE , dwmilliseconds : u32 ) -> u32 );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn WaitForMultipleObjects ( ncount : u32 , lphandles : *const super::super::Foundation:: HANDLE , bwaitall : super::super::Foundation:: BOOL , dwmilliseconds : u32 ) -> super::super::Foundation:: WIN32_ERROR );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn WaitForMultipleObjectsEx ( ncount : u32 , lphandles : *const super::super::Foundation:: HANDLE , bwaitall : super::super::Foundation:: BOOL , dwmilliseconds : u32 , balertable : super::super::Foundation:: BOOL ) -> super::super::Foundation:: WIN32_ERROR );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn WaitForSingleObject ( hhandle : super::super::Foundation:: HANDLE , dwmilliseconds : u32 ) -> super::super::Foundation:: WIN32_ERROR );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn WaitForSingleObjectEx ( hhandle : super::super::Foundation:: HANDLE , dwmilliseconds : u32 , balertable : super::super::Foundation:: BOOL ) -> super::super::Foundation:: WIN32_ERROR );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn WaitForThreadpoolIoCallbacks ( pio : PTP_IO , fcancelpendingcallbacks : super::super::Foundation:: BOOL ) -> ( ) );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn WaitForThreadpoolTimerCallbacks ( pti : PTP_TIMER , fcancelpendingcallbacks : super::super::Foundation:: BOOL ) -> ( ) );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn WaitForThreadpoolWaitCallbacks ( pwa : PTP_WAIT , fcancelpendingcallbacks : super::super::Foundation:: BOOL ) -> ( ) );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn WaitForThreadpoolWorkCallbacks ( pwk : PTP_WORK , fcancelpendingcallbacks : super::super::Foundation:: BOOL ) -> ( ) );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "api-ms-win-core-synch-l1-2-0.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn WaitOnAddress ( address : *const ::core::ffi::c_void , compareaddress : *const ::core::ffi::c_void , addresssize : usize , dwmilliseconds : u32 ) -> super::super::Foundation:: BOOL );
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`*"] fn WakeAllConditionVariable ( conditionvariable : *mut RTL_CONDITION_VARIABLE ) -> ( ) );
::windows_targets::link ! ( "api-ms-win-core-synch-l1-2-0.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`*"] fn WakeByAddressAll ( address : *const ::core::ffi::c_void ) -> ( ) );
::windows_targets::link ! ( "api-ms-win-core-synch-l1-2-0.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`*"] fn WakeByAddressSingle ( address : *const ::core::ffi::c_void ) -> ( ) );
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`*"] fn WakeConditionVariable ( conditionvariable : *mut RTL_CONDITION_VARIABLE ) -> ( ) );
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`*"] fn WinExec ( lpcmdline : ::windows_sys::core::PCSTR , ucmdshow : u32 ) -> u32 );
::windows_targets::link ! ( "api-ms-win-core-wow64-l1-1-1.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`*"] fn Wow64SetThreadDefaultGuestMachine ( machine : u16 ) -> u16 );
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"] fn Wow64SuspendThread ( hthread : super::super::Foundation:: HANDLE ) -> u32 );
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
pub const INFINITE: u32 = 4294967295u32;
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
pub type AVRT_PRIORITY = i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const AVRT_PRIORITY_VERYLOW: AVRT_PRIORITY = -2i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const AVRT_PRIORITY_LOW: AVRT_PRIORITY = -1i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const AVRT_PRIORITY_NORMAL: AVRT_PRIORITY = 0i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const AVRT_PRIORITY_HIGH: AVRT_PRIORITY = 1i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const AVRT_PRIORITY_CRITICAL: AVRT_PRIORITY = 2i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub type CREATE_EVENT = u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const CREATE_EVENT_INITIAL_SET: CREATE_EVENT = 2u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const CREATE_EVENT_MANUAL_RESET: CREATE_EVENT = 1u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub type CREATE_PROCESS_LOGON_FLAGS = u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const LOGON_WITH_PROFILE: CREATE_PROCESS_LOGON_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const LOGON_NETCREDENTIALS_ONLY: CREATE_PROCESS_LOGON_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub type GET_GUI_RESOURCES_FLAGS = u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const GR_GDIOBJECTS: GET_GUI_RESOURCES_FLAGS = 0u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const GR_GDIOBJECTS_PEAK: GET_GUI_RESOURCES_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const GR_USEROBJECTS: GET_GUI_RESOURCES_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const GR_USEROBJECTS_PEAK: GET_GUI_RESOURCES_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub type MACHINE_ATTRIBUTES = i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const UserEnabled: MACHINE_ATTRIBUTES = 1i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const KernelEnabled: MACHINE_ATTRIBUTES = 2i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const Wow64Container: MACHINE_ATTRIBUTES = 4i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub type MEMORY_PRIORITY = u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const MEMORY_PRIORITY_VERY_LOW: MEMORY_PRIORITY = 1u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const MEMORY_PRIORITY_LOW: MEMORY_PRIORITY = 2u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const MEMORY_PRIORITY_MEDIUM: MEMORY_PRIORITY = 3u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const MEMORY_PRIORITY_BELOW_NORMAL: MEMORY_PRIORITY = 4u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const MEMORY_PRIORITY_NORMAL: MEMORY_PRIORITY = 5u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub type POWER_REQUEST_CONTEXT_FLAGS = u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const POWER_REQUEST_CONTEXT_DETAILED_STRING: POWER_REQUEST_CONTEXT_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const POWER_REQUEST_CONTEXT_SIMPLE_STRING: POWER_REQUEST_CONTEXT_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub type PROCESSINFOCLASS = i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessBasicInformation: PROCESSINFOCLASS = 0i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessQuotaLimits: PROCESSINFOCLASS = 1i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessIoCounters: PROCESSINFOCLASS = 2i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessVmCounters: PROCESSINFOCLASS = 3i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessTimes: PROCESSINFOCLASS = 4i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessBasePriority: PROCESSINFOCLASS = 5i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessRaisePriority: PROCESSINFOCLASS = 6i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessDebugPort: PROCESSINFOCLASS = 7i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessExceptionPort: PROCESSINFOCLASS = 8i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessAccessToken: PROCESSINFOCLASS = 9i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessLdtInformation: PROCESSINFOCLASS = 10i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessLdtSize: PROCESSINFOCLASS = 11i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessDefaultHardErrorMode: PROCESSINFOCLASS = 12i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessIoPortHandlers: PROCESSINFOCLASS = 13i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessPooledUsageAndLimits: PROCESSINFOCLASS = 14i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessWorkingSetWatch: PROCESSINFOCLASS = 15i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessUserModeIOPL: PROCESSINFOCLASS = 16i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessEnableAlignmentFaultFixup: PROCESSINFOCLASS = 17i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessPriorityClass: PROCESSINFOCLASS = 18i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessWx86Information: PROCESSINFOCLASS = 19i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessHandleCount: PROCESSINFOCLASS = 20i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessAffinityMask: PROCESSINFOCLASS = 21i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessPriorityBoost: PROCESSINFOCLASS = 22i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessDeviceMap: PROCESSINFOCLASS = 23i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessSessionInformation: PROCESSINFOCLASS = 24i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessForegroundInformation: PROCESSINFOCLASS = 25i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessWow64Information: PROCESSINFOCLASS = 26i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessImageFileName: PROCESSINFOCLASS = 27i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessLUIDDeviceMapsEnabled: PROCESSINFOCLASS = 28i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessBreakOnTermination: PROCESSINFOCLASS = 29i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessDebugObjectHandle: PROCESSINFOCLASS = 30i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessDebugFlags: PROCESSINFOCLASS = 31i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessHandleTracing: PROCESSINFOCLASS = 32i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessIoPriority: PROCESSINFOCLASS = 33i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessExecuteFlags: PROCESSINFOCLASS = 34i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessTlsInformation: PROCESSINFOCLASS = 35i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessCookie: PROCESSINFOCLASS = 36i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessImageInformation: PROCESSINFOCLASS = 37i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessCycleTime: PROCESSINFOCLASS = 38i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessPagePriority: PROCESSINFOCLASS = 39i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessInstrumentationCallback: PROCESSINFOCLASS = 40i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessThreadStackAllocation: PROCESSINFOCLASS = 41i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessWorkingSetWatchEx: PROCESSINFOCLASS = 42i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessImageFileNameWin32: PROCESSINFOCLASS = 43i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessImageFileMapping: PROCESSINFOCLASS = 44i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessAffinityUpdateMode: PROCESSINFOCLASS = 45i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessMemoryAllocationMode: PROCESSINFOCLASS = 46i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessGroupInformation: PROCESSINFOCLASS = 47i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessTokenVirtualizationEnabled: PROCESSINFOCLASS = 48i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessOwnerInformation: PROCESSINFOCLASS = 49i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessWindowInformation: PROCESSINFOCLASS = 50i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessHandleInformation: PROCESSINFOCLASS = 51i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessMitigationPolicy: PROCESSINFOCLASS = 52i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessDynamicFunctionTableInformation: PROCESSINFOCLASS = 53i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessHandleCheckingMode: PROCESSINFOCLASS = 54i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessKeepAliveCount: PROCESSINFOCLASS = 55i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessRevokeFileHandles: PROCESSINFOCLASS = 56i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessWorkingSetControl: PROCESSINFOCLASS = 57i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessHandleTable: PROCESSINFOCLASS = 58i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessCheckStackExtentsMode: PROCESSINFOCLASS = 59i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessCommandLineInformation: PROCESSINFOCLASS = 60i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessProtectionInformation: PROCESSINFOCLASS = 61i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessMemoryExhaustion: PROCESSINFOCLASS = 62i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessFaultInformation: PROCESSINFOCLASS = 63i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessTelemetryIdInformation: PROCESSINFOCLASS = 64i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessCommitReleaseInformation: PROCESSINFOCLASS = 65i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessReserved1Information: PROCESSINFOCLASS = 66i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessReserved2Information: PROCESSINFOCLASS = 67i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessSubsystemProcess: PROCESSINFOCLASS = 68i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessInPrivate: PROCESSINFOCLASS = 70i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessRaiseUMExceptionOnInvalidHandleClose: PROCESSINFOCLASS = 71i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessSubsystemInformation: PROCESSINFOCLASS = 75i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessWin32kSyscallFilterInformation: PROCESSINFOCLASS = 79i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessEnergyTrackingState: PROCESSINFOCLASS = 82i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const MaxProcessInfoClass: PROCESSINFOCLASS = 83i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub type PROCESSOR_FEATURE_ID = u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_ARM_64BIT_LOADSTORE_ATOMIC: PROCESSOR_FEATURE_ID = 25u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_ARM_DIVIDE_INSTRUCTION_AVAILABLE: PROCESSOR_FEATURE_ID = 24u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_ARM_EXTERNAL_CACHE_AVAILABLE: PROCESSOR_FEATURE_ID = 26u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_ARM_FMAC_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = 27u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_ARM_VFP_32_REGISTERS_AVAILABLE: PROCESSOR_FEATURE_ID = 18u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_3DNOW_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = 7u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_CHANNELS_ENABLED: PROCESSOR_FEATURE_ID = 16u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_COMPARE_EXCHANGE_DOUBLE: PROCESSOR_FEATURE_ID = 2u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_COMPARE_EXCHANGE128: PROCESSOR_FEATURE_ID = 14u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_COMPARE64_EXCHANGE128: PROCESSOR_FEATURE_ID = 15u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_FASTFAIL_AVAILABLE: PROCESSOR_FEATURE_ID = 23u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_FLOATING_POINT_EMULATED: PROCESSOR_FEATURE_ID = 1u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_FLOATING_POINT_PRECISION_ERRATA: PROCESSOR_FEATURE_ID = 0u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_MMX_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = 3u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_NX_ENABLED: PROCESSOR_FEATURE_ID = 12u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_PAE_ENABLED: PROCESSOR_FEATURE_ID = 9u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_RDTSC_INSTRUCTION_AVAILABLE: PROCESSOR_FEATURE_ID = 8u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_RDWRFSGSBASE_AVAILABLE: PROCESSOR_FEATURE_ID = 22u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_SECOND_LEVEL_ADDRESS_TRANSLATION: PROCESSOR_FEATURE_ID = 20u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_SSE3_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = 13u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_VIRT_FIRMWARE_ENABLED: PROCESSOR_FEATURE_ID = 21u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_XMMI_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = 6u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_XMMI64_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = 10u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_XSAVE_ENABLED: PROCESSOR_FEATURE_ID = 17u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_ARM_V8_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = 29u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_ARM_V8_CRYPTO_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = 30u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_ARM_V8_CRC32_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = 31u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_ARM_V81_ATOMIC_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = 34u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub type PROCESS_ACCESS_RIGHTS = u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_TERMINATE: PROCESS_ACCESS_RIGHTS = 1u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_CREATE_THREAD: PROCESS_ACCESS_RIGHTS = 2u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_SET_SESSIONID: PROCESS_ACCESS_RIGHTS = 4u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_VM_OPERATION: PROCESS_ACCESS_RIGHTS = 8u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_VM_READ: PROCESS_ACCESS_RIGHTS = 16u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_VM_WRITE: PROCESS_ACCESS_RIGHTS = 32u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_DUP_HANDLE: PROCESS_ACCESS_RIGHTS = 64u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_CREATE_PROCESS: PROCESS_ACCESS_RIGHTS = 128u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_SET_QUOTA: PROCESS_ACCESS_RIGHTS = 256u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_SET_INFORMATION: PROCESS_ACCESS_RIGHTS = 512u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_QUERY_INFORMATION: PROCESS_ACCESS_RIGHTS = 1024u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_SUSPEND_RESUME: PROCESS_ACCESS_RIGHTS = 2048u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_QUERY_LIMITED_INFORMATION: PROCESS_ACCESS_RIGHTS = 4096u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_SET_LIMITED_INFORMATION: PROCESS_ACCESS_RIGHTS = 8192u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_ALL_ACCESS: PROCESS_ACCESS_RIGHTS = 2097151u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_DELETE: PROCESS_ACCESS_RIGHTS = 65536u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_READ_CONTROL: PROCESS_ACCESS_RIGHTS = 131072u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_WRITE_DAC: PROCESS_ACCESS_RIGHTS = 262144u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_WRITE_OWNER: PROCESS_ACCESS_RIGHTS = 524288u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_SYNCHRONIZE: PROCESS_ACCESS_RIGHTS = 1048576u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_STANDARD_RIGHTS_REQUIRED: PROCESS_ACCESS_RIGHTS = 983040u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub type PROCESS_AFFINITY_AUTO_UPDATE_FLAGS = u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_AFFINITY_DISABLE_AUTO_UPDATE: PROCESS_AFFINITY_AUTO_UPDATE_FLAGS = 0u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_AFFINITY_ENABLE_AUTO_UPDATE: PROCESS_AFFINITY_AUTO_UPDATE_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub type PROCESS_CREATION_FLAGS = u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const DEBUG_PROCESS: PROCESS_CREATION_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const DEBUG_ONLY_THIS_PROCESS: PROCESS_CREATION_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const CREATE_SUSPENDED: PROCESS_CREATION_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const DETACHED_PROCESS: PROCESS_CREATION_FLAGS = 8u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const CREATE_NEW_CONSOLE: PROCESS_CREATION_FLAGS = 16u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const NORMAL_PRIORITY_CLASS: PROCESS_CREATION_FLAGS = 32u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const IDLE_PRIORITY_CLASS: PROCESS_CREATION_FLAGS = 64u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const HIGH_PRIORITY_CLASS: PROCESS_CREATION_FLAGS = 128u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const REALTIME_PRIORITY_CLASS: PROCESS_CREATION_FLAGS = 256u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const CREATE_NEW_PROCESS_GROUP: PROCESS_CREATION_FLAGS = 512u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const CREATE_UNICODE_ENVIRONMENT: PROCESS_CREATION_FLAGS = 1024u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const CREATE_SEPARATE_WOW_VDM: PROCESS_CREATION_FLAGS = 2048u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const CREATE_SHARED_WOW_VDM: PROCESS_CREATION_FLAGS = 4096u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const CREATE_FORCEDOS: PROCESS_CREATION_FLAGS = 8192u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const BELOW_NORMAL_PRIORITY_CLASS: PROCESS_CREATION_FLAGS = 16384u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ABOVE_NORMAL_PRIORITY_CLASS: PROCESS_CREATION_FLAGS = 32768u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const INHERIT_PARENT_AFFINITY: PROCESS_CREATION_FLAGS = 65536u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const INHERIT_CALLER_PRIORITY: PROCESS_CREATION_FLAGS = 131072u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const CREATE_PROTECTED_PROCESS: PROCESS_CREATION_FLAGS = 262144u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const EXTENDED_STARTUPINFO_PRESENT: PROCESS_CREATION_FLAGS = 524288u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_MODE_BACKGROUND_BEGIN: PROCESS_CREATION_FLAGS = 1048576u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_MODE_BACKGROUND_END: PROCESS_CREATION_FLAGS = 2097152u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const CREATE_SECURE_PROCESS: PROCESS_CREATION_FLAGS = 4194304u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const CREATE_BREAKAWAY_FROM_JOB: PROCESS_CREATION_FLAGS = 16777216u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const CREATE_PRESERVE_CODE_AUTHZ_LEVEL: PROCESS_CREATION_FLAGS = 33554432u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const CREATE_DEFAULT_ERROR_MODE: PROCESS_CREATION_FLAGS = 67108864u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const CREATE_NO_WINDOW: PROCESS_CREATION_FLAGS = 134217728u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROFILE_USER: PROCESS_CREATION_FLAGS = 268435456u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROFILE_KERNEL: PROCESS_CREATION_FLAGS = 536870912u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROFILE_SERVER: PROCESS_CREATION_FLAGS = 1073741824u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const CREATE_IGNORE_SYSTEM_DEFAULT: PROCESS_CREATION_FLAGS = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub type PROCESS_DEP_FLAGS = u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_DEP_ENABLE: PROCESS_DEP_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_DEP_DISABLE_ATL_THUNK_EMULATION: PROCESS_DEP_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_DEP_NONE: PROCESS_DEP_FLAGS = 0u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub type PROCESS_INFORMATION_CLASS = i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessMemoryPriority: PROCESS_INFORMATION_CLASS = 0i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessMemoryExhaustionInfo: PROCESS_INFORMATION_CLASS = 1i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessAppMemoryInfo: PROCESS_INFORMATION_CLASS = 2i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessInPrivateInfo: PROCESS_INFORMATION_CLASS = 3i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessPowerThrottling: PROCESS_INFORMATION_CLASS = 4i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessReservedValue1: PROCESS_INFORMATION_CLASS = 5i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessTelemetryCoverageInfo: PROCESS_INFORMATION_CLASS = 6i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessProtectionLevelInfo: PROCESS_INFORMATION_CLASS = 7i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessLeapSecondInfo: PROCESS_INFORMATION_CLASS = 8i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessMachineTypeInfo: PROCESS_INFORMATION_CLASS = 9i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessInformationClassMax: PROCESS_INFORMATION_CLASS = 10i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub type PROCESS_MEMORY_EXHAUSTION_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PMETypeFailFastOnCommitFailure: PROCESS_MEMORY_EXHAUSTION_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PMETypeMax: PROCESS_MEMORY_EXHAUSTION_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub type PROCESS_MITIGATION_POLICY = i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessDEPPolicy: PROCESS_MITIGATION_POLICY = 0i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessASLRPolicy: PROCESS_MITIGATION_POLICY = 1i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessDynamicCodePolicy: PROCESS_MITIGATION_POLICY = 2i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessStrictHandleCheckPolicy: PROCESS_MITIGATION_POLICY = 3i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessSystemCallDisablePolicy: PROCESS_MITIGATION_POLICY = 4i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessMitigationOptionsMask: PROCESS_MITIGATION_POLICY = 5i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessExtensionPointDisablePolicy: PROCESS_MITIGATION_POLICY = 6i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessControlFlowGuardPolicy: PROCESS_MITIGATION_POLICY = 7i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessSignaturePolicy: PROCESS_MITIGATION_POLICY = 8i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessFontDisablePolicy: PROCESS_MITIGATION_POLICY = 9i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessImageLoadPolicy: PROCESS_MITIGATION_POLICY = 10i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessSystemCallFilterPolicy: PROCESS_MITIGATION_POLICY = 11i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessPayloadRestrictionPolicy: PROCESS_MITIGATION_POLICY = 12i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessChildProcessPolicy: PROCESS_MITIGATION_POLICY = 13i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessSideChannelIsolationPolicy: PROCESS_MITIGATION_POLICY = 14i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessUserShadowStackPolicy: PROCESS_MITIGATION_POLICY = 15i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessRedirectionTrustPolicy: PROCESS_MITIGATION_POLICY = 16i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessUserPointerAuthPolicy: PROCESS_MITIGATION_POLICY = 17i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessSEHOPPolicy: PROCESS_MITIGATION_POLICY = 18i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const MaxProcessMitigationPolicy: PROCESS_MITIGATION_POLICY = 19i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub type PROCESS_NAME_FORMAT = u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_NAME_WIN32: PROCESS_NAME_FORMAT = 0u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_NAME_NATIVE: PROCESS_NAME_FORMAT = 1u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub type PROCESS_PROTECTION_LEVEL = u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROTECTION_LEVEL_WINTCB_LIGHT: PROCESS_PROTECTION_LEVEL = 0u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROTECTION_LEVEL_WINDOWS: PROCESS_PROTECTION_LEVEL = 1u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROTECTION_LEVEL_WINDOWS_LIGHT: PROCESS_PROTECTION_LEVEL = 2u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROTECTION_LEVEL_ANTIMALWARE_LIGHT: PROCESS_PROTECTION_LEVEL = 3u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROTECTION_LEVEL_LSA_LIGHT: PROCESS_PROTECTION_LEVEL = 4u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROTECTION_LEVEL_WINTCB: PROCESS_PROTECTION_LEVEL = 5u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROTECTION_LEVEL_CODEGEN_LIGHT: PROCESS_PROTECTION_LEVEL = 6u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROTECTION_LEVEL_AUTHENTICODE: PROCESS_PROTECTION_LEVEL = 7u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROTECTION_LEVEL_PPL_APP: PROCESS_PROTECTION_LEVEL = 8u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROTECTION_LEVEL_NONE: PROCESS_PROTECTION_LEVEL = 4294967294u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub type PROC_THREAD_ATTRIBUTE_NUM = u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcThreadAttributeParentProcess: PROC_THREAD_ATTRIBUTE_NUM = 0u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcThreadAttributeHandleList: PROC_THREAD_ATTRIBUTE_NUM = 2u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcThreadAttributeGroupAffinity: PROC_THREAD_ATTRIBUTE_NUM = 3u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcThreadAttributePreferredNode: PROC_THREAD_ATTRIBUTE_NUM = 4u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcThreadAttributeIdealProcessor: PROC_THREAD_ATTRIBUTE_NUM = 5u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcThreadAttributeUmsThread: PROC_THREAD_ATTRIBUTE_NUM = 6u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcThreadAttributeMitigationPolicy: PROC_THREAD_ATTRIBUTE_NUM = 7u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcThreadAttributeSecurityCapabilities: PROC_THREAD_ATTRIBUTE_NUM = 9u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcThreadAttributeProtectionLevel: PROC_THREAD_ATTRIBUTE_NUM = 11u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcThreadAttributeJobList: PROC_THREAD_ATTRIBUTE_NUM = 13u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcThreadAttributeChildProcessPolicy: PROC_THREAD_ATTRIBUTE_NUM = 14u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcThreadAttributeAllApplicationPackagesPolicy: PROC_THREAD_ATTRIBUTE_NUM = 15u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcThreadAttributeWin32kFilter: PROC_THREAD_ATTRIBUTE_NUM = 16u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcThreadAttributeSafeOpenPromptOriginClaim: PROC_THREAD_ATTRIBUTE_NUM = 17u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcThreadAttributeDesktopAppPolicy: PROC_THREAD_ATTRIBUTE_NUM = 18u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcThreadAttributePseudoConsole: PROC_THREAD_ATTRIBUTE_NUM = 22u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcThreadAttributeMitigationAuditPolicy: PROC_THREAD_ATTRIBUTE_NUM = 24u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcThreadAttributeMachineType: PROC_THREAD_ATTRIBUTE_NUM = 25u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcThreadAttributeComponentFilter: PROC_THREAD_ATTRIBUTE_NUM = 26u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcThreadAttributeEnableOptionalXStateFeatures: PROC_THREAD_ATTRIBUTE_NUM = 27u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcThreadAttributeTrustedApp: PROC_THREAD_ATTRIBUTE_NUM = 29u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub type QUEUE_USER_APC_FLAGS = i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const QUEUE_USER_APC_FLAGS_NONE: QUEUE_USER_APC_FLAGS = 0i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const QUEUE_USER_APC_FLAGS_SPECIAL_USER_APC: QUEUE_USER_APC_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const QUEUE_USER_APC_CALLBACK_DATA_CONTEXT: QUEUE_USER_APC_FLAGS = 65536i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub type RTL_UMS_THREAD_INFO_CLASS = i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const UmsThreadInvalidInfoClass: RTL_UMS_THREAD_INFO_CLASS = 0i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const UmsThreadUserContext: RTL_UMS_THREAD_INFO_CLASS = 1i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const UmsThreadPriority: RTL_UMS_THREAD_INFO_CLASS = 2i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const UmsThreadAffinity: RTL_UMS_THREAD_INFO_CLASS = 3i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const UmsThreadTeb: RTL_UMS_THREAD_INFO_CLASS = 4i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const UmsThreadIsSuspended: RTL_UMS_THREAD_INFO_CLASS = 5i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const UmsThreadIsTerminated: RTL_UMS_THREAD_INFO_CLASS = 6i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const UmsThreadMaxInfoClass: RTL_UMS_THREAD_INFO_CLASS = 7i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub type STARTUPINFOW_FLAGS = u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const STARTF_FORCEONFEEDBACK: STARTUPINFOW_FLAGS = 64u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const STARTF_FORCEOFFFEEDBACK: STARTUPINFOW_FLAGS = 128u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const STARTF_PREVENTPINNING: STARTUPINFOW_FLAGS = 8192u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const STARTF_RUNFULLSCREEN: STARTUPINFOW_FLAGS = 32u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const STARTF_TITLEISAPPID: STARTUPINFOW_FLAGS = 4096u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const STARTF_TITLEISLINKNAME: STARTUPINFOW_FLAGS = 2048u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const STARTF_UNTRUSTEDSOURCE: STARTUPINFOW_FLAGS = 32768u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const STARTF_USECOUNTCHARS: STARTUPINFOW_FLAGS = 8u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const STARTF_USEFILLATTRIBUTE: STARTUPINFOW_FLAGS = 16u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const STARTF_USEHOTKEY: STARTUPINFOW_FLAGS = 512u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const STARTF_USEPOSITION: STARTUPINFOW_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const STARTF_USESHOWWINDOW: STARTUPINFOW_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const STARTF_USESIZE: STARTUPINFOW_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const STARTF_USESTDHANDLES: STARTUPINFOW_FLAGS = 256u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub type SYNCHRONIZATION_ACCESS_RIGHTS = u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const EVENT_ALL_ACCESS: SYNCHRONIZATION_ACCESS_RIGHTS = 2031619u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const EVENT_MODIFY_STATE: SYNCHRONIZATION_ACCESS_RIGHTS = 2u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const MUTEX_ALL_ACCESS: SYNCHRONIZATION_ACCESS_RIGHTS = 2031617u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const MUTEX_MODIFY_STATE: SYNCHRONIZATION_ACCESS_RIGHTS = 1u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const SEMAPHORE_ALL_ACCESS: SYNCHRONIZATION_ACCESS_RIGHTS = 2031619u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const SEMAPHORE_MODIFY_STATE: SYNCHRONIZATION_ACCESS_RIGHTS = 2u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const TIMER_ALL_ACCESS: SYNCHRONIZATION_ACCESS_RIGHTS = 2031619u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const TIMER_MODIFY_STATE: SYNCHRONIZATION_ACCESS_RIGHTS = 2u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const TIMER_QUERY_STATE: SYNCHRONIZATION_ACCESS_RIGHTS = 1u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const SYNCHRONIZATION_DELETE: SYNCHRONIZATION_ACCESS_RIGHTS = 65536u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const SYNCHRONIZATION_READ_CONTROL: SYNCHRONIZATION_ACCESS_RIGHTS = 131072u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const SYNCHRONIZATION_WRITE_DAC: SYNCHRONIZATION_ACCESS_RIGHTS = 262144u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const SYNCHRONIZATION_WRITE_OWNER: SYNCHRONIZATION_ACCESS_RIGHTS = 524288u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const SYNCHRONIZATION_SYNCHRONIZE: SYNCHRONIZATION_ACCESS_RIGHTS = 1048576u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub type THREADINFOCLASS = i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadBasicInformation: THREADINFOCLASS = 0i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadTimes: THREADINFOCLASS = 1i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadPriority: THREADINFOCLASS = 2i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadBasePriority: THREADINFOCLASS = 3i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadAffinityMask: THREADINFOCLASS = 4i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadImpersonationToken: THREADINFOCLASS = 5i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadDescriptorTableEntry: THREADINFOCLASS = 6i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadEnableAlignmentFaultFixup: THREADINFOCLASS = 7i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadEventPair_Reusable: THREADINFOCLASS = 8i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadQuerySetWin32StartAddress: THREADINFOCLASS = 9i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadZeroTlsCell: THREADINFOCLASS = 10i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadPerformanceCount: THREADINFOCLASS = 11i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadAmILastThread: THREADINFOCLASS = 12i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadIdealProcessor: THREADINFOCLASS = 13i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadPriorityBoost: THREADINFOCLASS = 14i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadSetTlsArrayAddress: THREADINFOCLASS = 15i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadIsIoPending: THREADINFOCLASS = 16i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadHideFromDebugger: THREADINFOCLASS = 17i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadBreakOnTermination: THREADINFOCLASS = 18i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadSwitchLegacyState: THREADINFOCLASS = 19i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadIsTerminated: THREADINFOCLASS = 20i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadLastSystemCall: THREADINFOCLASS = 21i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadIoPriority: THREADINFOCLASS = 22i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadCycleTime: THREADINFOCLASS = 23i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadPagePriority: THREADINFOCLASS = 24i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadActualBasePriority: THREADINFOCLASS = 25i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadTebInformation: THREADINFOCLASS = 26i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadCSwitchMon: THREADINFOCLASS = 27i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadCSwitchPmu: THREADINFOCLASS = 28i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadWow64Context: THREADINFOCLASS = 29i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadGroupInformation: THREADINFOCLASS = 30i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadUmsInformation: THREADINFOCLASS = 31i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadCounterProfiling: THREADINFOCLASS = 32i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadIdealProcessorEx: THREADINFOCLASS = 33i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadCpuAccountingInformation: THREADINFOCLASS = 34i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadSuspendCount: THREADINFOCLASS = 35i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadActualGroupAffinity: THREADINFOCLASS = 41i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadDynamicCodePolicyInfo: THREADINFOCLASS = 42i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadSubsystemInformation: THREADINFOCLASS = 45i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const MaxThreadInfoClass: THREADINFOCLASS = 53i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub type THREAD_ACCESS_RIGHTS = u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_TERMINATE: THREAD_ACCESS_RIGHTS = 1u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_SUSPEND_RESUME: THREAD_ACCESS_RIGHTS = 2u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_GET_CONTEXT: THREAD_ACCESS_RIGHTS = 8u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_SET_CONTEXT: THREAD_ACCESS_RIGHTS = 16u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_SET_INFORMATION: THREAD_ACCESS_RIGHTS = 32u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_QUERY_INFORMATION: THREAD_ACCESS_RIGHTS = 64u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_SET_THREAD_TOKEN: THREAD_ACCESS_RIGHTS = 128u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_IMPERSONATE: THREAD_ACCESS_RIGHTS = 256u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_DIRECT_IMPERSONATION: THREAD_ACCESS_RIGHTS = 512u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_SET_LIMITED_INFORMATION: THREAD_ACCESS_RIGHTS = 1024u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_QUERY_LIMITED_INFORMATION: THREAD_ACCESS_RIGHTS = 2048u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_RESUME: THREAD_ACCESS_RIGHTS = 4096u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_ALL_ACCESS: THREAD_ACCESS_RIGHTS = 2097151u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_DELETE: THREAD_ACCESS_RIGHTS = 65536u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_READ_CONTROL: THREAD_ACCESS_RIGHTS = 131072u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_WRITE_DAC: THREAD_ACCESS_RIGHTS = 262144u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_WRITE_OWNER: THREAD_ACCESS_RIGHTS = 524288u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_SYNCHRONIZE: THREAD_ACCESS_RIGHTS = 1048576u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_STANDARD_RIGHTS_REQUIRED: THREAD_ACCESS_RIGHTS = 983040u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub type THREAD_CREATION_FLAGS = u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_CREATE_RUN_IMMEDIATELY: THREAD_CREATION_FLAGS = 0u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_CREATE_SUSPENDED: THREAD_CREATION_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const STACK_SIZE_PARAM_IS_A_RESERVATION: THREAD_CREATION_FLAGS = 65536u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub type THREAD_INFORMATION_CLASS = i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadMemoryPriority: THREAD_INFORMATION_CLASS = 0i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadAbsoluteCpuPriority: THREAD_INFORMATION_CLASS = 1i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadDynamicCodePolicy: THREAD_INFORMATION_CLASS = 2i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadPowerThrottling: THREAD_INFORMATION_CLASS = 3i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadInformationClassMax: THREAD_INFORMATION_CLASS = 4i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub type THREAD_PRIORITY = i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_MODE_BACKGROUND_BEGIN: THREAD_PRIORITY = 65536i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_MODE_BACKGROUND_END: THREAD_PRIORITY = 131072i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_PRIORITY_ABOVE_NORMAL: THREAD_PRIORITY = 1i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_PRIORITY_BELOW_NORMAL: THREAD_PRIORITY = -1i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_PRIORITY_HIGHEST: THREAD_PRIORITY = 2i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_PRIORITY_IDLE: THREAD_PRIORITY = -15i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_PRIORITY_MIN: THREAD_PRIORITY = -2i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_PRIORITY_LOWEST: THREAD_PRIORITY = -2i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_PRIORITY_NORMAL: THREAD_PRIORITY = 0i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_PRIORITY_TIME_CRITICAL: THREAD_PRIORITY = 15i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub type TP_CALLBACK_PRIORITY = i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const TP_CALLBACK_PRIORITY_HIGH: TP_CALLBACK_PRIORITY = 0i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const TP_CALLBACK_PRIORITY_NORMAL: TP_CALLBACK_PRIORITY = 1i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const TP_CALLBACK_PRIORITY_LOW: TP_CALLBACK_PRIORITY = 2i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const TP_CALLBACK_PRIORITY_INVALID: TP_CALLBACK_PRIORITY = 3i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const TP_CALLBACK_PRIORITY_COUNT: TP_CALLBACK_PRIORITY = 3i32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub type WORKER_THREAD_FLAGS = u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const WT_EXECUTEDEFAULT: WORKER_THREAD_FLAGS = 0u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const WT_EXECUTEINIOTHREAD: WORKER_THREAD_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const WT_EXECUTEINPERSISTENTTHREAD: WORKER_THREAD_FLAGS = 128u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const WT_EXECUTEINWAITTHREAD: WORKER_THREAD_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const WT_EXECUTELONGFUNCTION: WORKER_THREAD_FLAGS = 16u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const WT_EXECUTEONLYONCE: WORKER_THREAD_FLAGS = 8u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const WT_TRANSFER_IMPERSONATION: WORKER_THREAD_FLAGS = 256u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const WT_EXECUTEINTIMERTHREAD: WORKER_THREAD_FLAGS = 32u32;
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
pub type BoundaryDescriptorHandle = isize;
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
pub type LPPROC_THREAD_ATTRIBUTE_LIST = *mut ::core::ffi::c_void;
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
pub type NamespaceHandle = isize;
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
pub type PTP_CALLBACK_INSTANCE = isize;
pub type PTP_IO = isize;
pub type PTP_POOL = isize;
pub type PTP_TIMER = isize;
pub type PTP_WAIT = isize;
pub type PTP_WORK = isize;
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
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union REASON_CONTEXT_0 {
    pub Detailed: REASON_CONTEXT_0_0,
    pub SimpleReasonString: ::windows_sys::core::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for REASON_CONTEXT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for REASON_CONTEXT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct REASON_CONTEXT_0_0 {
    pub LocalizedReasonModule: super::super::Foundation::HMODULE,
    pub LocalizedReasonId: u32,
    pub ReasonStringCount: u32,
    pub ReasonStrings: *mut ::windows_sys::core::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for REASON_CONTEXT_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for REASON_CONTEXT_0_0 {
    fn clone(&self) -> Self {
        *self
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
    pub Identifier: u16,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::marker::Copy for RTL_CRITICAL_SECTION_DEBUG {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::clone::Clone for RTL_CRITICAL_SECTION_DEBUG {
    fn clone(&self) -> Self {
        *self
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
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct STARTUPINFOA {
    pub cb: u32,
    pub lpReserved: ::windows_sys::core::PSTR,
    pub lpDesktop: ::windows_sys::core::PSTR,
    pub lpTitle: ::windows_sys::core::PSTR,
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
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct STARTUPINFOW {
    pub cb: u32,
    pub lpReserved: ::windows_sys::core::PWSTR,
    pub lpDesktop: ::windows_sys::core::PWSTR,
    pub lpTitle: ::windows_sys::core::PWSTR,
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
pub type TimerQueueHandle = isize;
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
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub type APC_CALLBACK_FUNCTION = ::core::option::Option<unsafe extern "system" fn(param0: u32, param1: *mut ::core::ffi::c_void, param2: *mut ::core::ffi::c_void) -> ()>;
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
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub type WORKERCALLBACKFUNC = ::core::option::Option<unsafe extern "system" fn(param0: *mut ::core::ffi::c_void) -> ()>;
