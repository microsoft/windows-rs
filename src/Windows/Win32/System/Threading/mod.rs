#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct APP_MEMORY_INFORMATION {
    pub AvailableCommit: u64,
    pub PrivateCommitUsage: u64,
    pub PeakPrivateCommitUsage: u64,
    pub TotalCommitUsage: u64,
}
impl APP_MEMORY_INFORMATION {}
impl ::std::default::Default for APP_MEMORY_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for APP_MEMORY_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("APP_MEMORY_INFORMATION")
            .field("AvailableCommit", &self.AvailableCommit)
            .field("PrivateCommitUsage", &self.PrivateCommitUsage)
            .field("PeakPrivateCommitUsage", &self.PeakPrivateCommitUsage)
            .field("TotalCommitUsage", &self.TotalCommitUsage)
            .finish()
    }
}
impl ::std::cmp::PartialEq for APP_MEMORY_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.AvailableCommit == other.AvailableCommit
            && self.PrivateCommitUsage == other.PrivateCommitUsage
            && self.PeakPrivateCommitUsage == other.PeakPrivateCommitUsage
            && self.TotalCommitUsage == other.TotalCommitUsage
    }
}
impl ::std::cmp::Eq for APP_MEMORY_INFORMATION {}
unsafe impl ::windows::runtime::Abi for APP_MEMORY_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn AcquireSRWLockExclusive(srwlock: *mut super::SystemServices::RTL_SRWLOCK) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AcquireSRWLockExclusive(srwlock: *mut super::SystemServices::RTL_SRWLOCK);
        }
        ::std::mem::transmute(AcquireSRWLockExclusive(::std::mem::transmute(srwlock)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn AcquireSRWLockShared(srwlock: *mut super::SystemServices::RTL_SRWLOCK) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AcquireSRWLockShared(srwlock: *mut super::SystemServices::RTL_SRWLOCK);
        }
        ::std::mem::transmute(AcquireSRWLockShared(::std::mem::transmute(srwlock)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn AddIntegrityLabelToBoundaryDescriptor<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSID>,
>(
    boundarydescriptor: *mut super::super::Foundation::HANDLE,
    integritylabel: Param1,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AddIntegrityLabelToBoundaryDescriptor(
                boundarydescriptor: *mut super::super::Foundation::HANDLE,
                integritylabel: super::super::Foundation::PSID,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(AddIntegrityLabelToBoundaryDescriptor(
            ::std::mem::transmute(boundarydescriptor),
            integritylabel.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn AddSIDToBoundaryDescriptor<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSID>,
>(
    boundarydescriptor: *mut super::super::Foundation::HANDLE,
    requiredsid: Param1,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AddSIDToBoundaryDescriptor(
                boundarydescriptor: *mut super::super::Foundation::HANDLE,
                requiredsid: super::super::Foundation::PSID,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(AddSIDToBoundaryDescriptor(
            ::std::mem::transmute(boundarydescriptor),
            requiredsid.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn AttachThreadInput<
    'a,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    idattach: u32,
    idattachto: u32,
    fattach: Param2,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AttachThreadInput(
                idattach: u32,
                idattachto: u32,
                fattach: super::super::Foundation::BOOL,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(AttachThreadInput(
            ::std::mem::transmute(idattach),
            ::std::mem::transmute(idattachto),
            fattach.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(
    :: std :: clone :: Clone,
    :: std :: marker :: Copy,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
)]
#[repr(transparent)]
pub struct BoundaryDescriptorHandle(pub isize);
impl ::std::default::Default for BoundaryDescriptorHandle {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for BoundaryDescriptorHandle {}
unsafe impl ::windows::runtime::Abi for BoundaryDescriptorHandle {
    type Abi = Self;
    type DefaultType = Self;
}
pub const CONDITION_VARIABLE_LOCKMODE_SHARED: u32 = 1u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct CREATE_EVENT(pub u32);
pub const CREATE_EVENT_INITIAL_SET: CREATE_EVENT = CREATE_EVENT(2u32);
pub const CREATE_EVENT_MANUAL_RESET: CREATE_EVENT = CREATE_EVENT(1u32);
impl ::std::convert::From<u32> for CREATE_EVENT {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CREATE_EVENT {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for CREATE_EVENT {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for CREATE_EVENT {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for CREATE_EVENT {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for CREATE_EVENT {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for CREATE_EVENT {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CREATE_MUTEX_INITIAL_OWNER: u32 = 1u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct CREATE_PROCESS_LOGON_FLAGS(pub u32);
pub const LOGON_WITH_PROFILE: CREATE_PROCESS_LOGON_FLAGS = CREATE_PROCESS_LOGON_FLAGS(1u32);
pub const LOGON_NETCREDENTIALS_ONLY: CREATE_PROCESS_LOGON_FLAGS = CREATE_PROCESS_LOGON_FLAGS(2u32);
impl ::std::convert::From<u32> for CREATE_PROCESS_LOGON_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CREATE_PROCESS_LOGON_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for CREATE_PROCESS_LOGON_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for CREATE_PROCESS_LOGON_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for CREATE_PROCESS_LOGON_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for CREATE_PROCESS_LOGON_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for CREATE_PROCESS_LOGON_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CREATE_WAITABLE_TIMER_HIGH_RESOLUTION: u32 = 2u32;
pub const CREATE_WAITABLE_TIMER_MANUAL_RESET: u32 = 1u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn CallbackMayRunLong(
    pci: *mut super::SystemServices::TP_CALLBACK_INSTANCE,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CallbackMayRunLong(
                pci: *mut super::SystemServices::TP_CALLBACK_INSTANCE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CallbackMayRunLong(::std::mem::transmute(pci)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn CancelThreadpoolIo(pio: *mut super::SystemServices::TP_IO) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CancelThreadpoolIo(pio: *mut super::SystemServices::TP_IO);
        }
        ::std::mem::transmute(CancelThreadpoolIo(::std::mem::transmute(pio)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn CancelWaitableTimer<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    htimer: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CancelWaitableTimer(
                htimer: super::super::Foundation::HANDLE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CancelWaitableTimer(htimer.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn ChangeTimerQueueTimer<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    timerqueue: Param0,
    timer: Param1,
    duetime: u32,
    period: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ChangeTimerQueueTimer(
                timerqueue: super::super::Foundation::HANDLE,
                timer: super::super::Foundation::HANDLE,
                duetime: u32,
                period: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ChangeTimerQueueTimer(
            timerqueue.into_param().abi(),
            timer.into_param().abi(),
            ::std::mem::transmute(duetime),
            ::std::mem::transmute(period),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn ClosePrivateNamespace<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, NamespaceHandle>,
>(
    handle: Param0,
    flags: u32,
) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ClosePrivateNamespace(
                handle: NamespaceHandle,
                flags: u32,
            ) -> super::super::Foundation::BOOLEAN;
        }
        ::std::mem::transmute(ClosePrivateNamespace(
            handle.into_param().abi(),
            ::std::mem::transmute(flags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn CloseThreadpool<'a, Param0: ::windows::runtime::IntoParam<'a, PTP_POOL>>(
    ptpp: Param0,
) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CloseThreadpool(ptpp: PTP_POOL);
        }
        ::std::mem::transmute(CloseThreadpool(ptpp.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn CloseThreadpoolCleanupGroup(ptpcg: isize) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CloseThreadpoolCleanupGroup(ptpcg: isize);
        }
        ::std::mem::transmute(CloseThreadpoolCleanupGroup(::std::mem::transmute(ptpcg)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn CloseThreadpoolCleanupGroupMembers<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    ptpcg: isize,
    fcancelpendingcallbacks: Param1,
    pvcleanupcontext: *mut ::std::ffi::c_void,
) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CloseThreadpoolCleanupGroupMembers(
                ptpcg: isize,
                fcancelpendingcallbacks: super::super::Foundation::BOOL,
                pvcleanupcontext: *mut ::std::ffi::c_void,
            );
        }
        ::std::mem::transmute(CloseThreadpoolCleanupGroupMembers(
            ::std::mem::transmute(ptpcg),
            fcancelpendingcallbacks.into_param().abi(),
            ::std::mem::transmute(pvcleanupcontext),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn CloseThreadpoolIo(pio: *mut super::SystemServices::TP_IO) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CloseThreadpoolIo(pio: *mut super::SystemServices::TP_IO);
        }
        ::std::mem::transmute(CloseThreadpoolIo(::std::mem::transmute(pio)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn CloseThreadpoolTimer(pti: *mut super::SystemServices::TP_TIMER) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CloseThreadpoolTimer(pti: *mut super::SystemServices::TP_TIMER);
        }
        ::std::mem::transmute(CloseThreadpoolTimer(::std::mem::transmute(pti)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn CloseThreadpoolWait(pwa: *mut super::SystemServices::TP_WAIT) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CloseThreadpoolWait(pwa: *mut super::SystemServices::TP_WAIT);
        }
        ::std::mem::transmute(CloseThreadpoolWait(::std::mem::transmute(pwa)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn CloseThreadpoolWork(pwk: *mut super::SystemServices::TP_WORK) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CloseThreadpoolWork(pwk: *mut super::SystemServices::TP_WORK);
        }
        ::std::mem::transmute(CloseThreadpoolWork(::std::mem::transmute(pwk)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn ConvertFiberToThread() -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ConvertFiberToThread() -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ConvertFiberToThread())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn ConvertThreadToFiber(
    lpparameter: *const ::std::ffi::c_void,
) -> *mut ::std::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ConvertThreadToFiber(
                lpparameter: *const ::std::ffi::c_void,
            ) -> *mut ::std::ffi::c_void;
        }
        ::std::mem::transmute(ConvertThreadToFiber(::std::mem::transmute(lpparameter)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn ConvertThreadToFiberEx(
    lpparameter: *const ::std::ffi::c_void,
    dwflags: u32,
) -> *mut ::std::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ConvertThreadToFiberEx(
                lpparameter: *const ::std::ffi::c_void,
                dwflags: u32,
            ) -> *mut ::std::ffi::c_void;
        }
        ::std::mem::transmute(ConvertThreadToFiberEx(
            ::std::mem::transmute(lpparameter),
            ::std::mem::transmute(dwflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn CreateBoundaryDescriptorA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    name: Param0,
    flags: u32,
) -> BoundaryDescriptorHandle {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateBoundaryDescriptorA(
                name: super::super::Foundation::PSTR,
                flags: u32,
            ) -> BoundaryDescriptorHandle;
        }
        ::std::mem::transmute(CreateBoundaryDescriptorA(
            name.into_param().abi(),
            ::std::mem::transmute(flags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn CreateBoundaryDescriptorW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    name: Param0,
    flags: u32,
) -> BoundaryDescriptorHandle {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateBoundaryDescriptorW(
                name: super::super::Foundation::PWSTR,
                flags: u32,
            ) -> BoundaryDescriptorHandle;
        }
        ::std::mem::transmute(CreateBoundaryDescriptorW(
            name.into_param().abi(),
            ::std::mem::transmute(flags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub unsafe fn CreateEventA<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lpeventattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
    bmanualreset: Param1,
    binitialstate: Param2,
    lpname: Param3,
) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateEventA(
                lpeventattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
                bmanualreset: super::super::Foundation::BOOL,
                binitialstate: super::super::Foundation::BOOL,
                lpname: super::super::Foundation::PSTR,
            ) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(CreateEventA(
            ::std::mem::transmute(lpeventattributes),
            bmanualreset.into_param().abi(),
            binitialstate.into_param().abi(),
            lpname.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub unsafe fn CreateEventExA<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lpeventattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
    lpname: Param1,
    dwflags: CREATE_EVENT,
    dwdesiredaccess: u32,
) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateEventExA(
                lpeventattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
                lpname: super::super::Foundation::PSTR,
                dwflags: CREATE_EVENT,
                dwdesiredaccess: u32,
            ) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(CreateEventExA(
            ::std::mem::transmute(lpeventattributes),
            lpname.into_param().abi(),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(dwdesiredaccess),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub unsafe fn CreateEventExW<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpeventattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
    lpname: Param1,
    dwflags: CREATE_EVENT,
    dwdesiredaccess: u32,
) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateEventExW(
                lpeventattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
                lpname: super::super::Foundation::PWSTR,
                dwflags: CREATE_EVENT,
                dwdesiredaccess: u32,
            ) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(CreateEventExW(
            ::std::mem::transmute(lpeventattributes),
            lpname.into_param().abi(),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(dwdesiredaccess),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub unsafe fn CreateEventW<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpeventattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
    bmanualreset: Param1,
    binitialstate: Param2,
    lpname: Param3,
) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateEventW(
                lpeventattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
                bmanualreset: super::super::Foundation::BOOL,
                binitialstate: super::super::Foundation::BOOL,
                lpname: super::super::Foundation::PWSTR,
            ) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(CreateEventW(
            ::std::mem::transmute(lpeventattributes),
            bmanualreset.into_param().abi(),
            binitialstate.into_param().abi(),
            lpname.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_WindowsProgramming")]
pub unsafe fn CreateFiber(
    dwstacksize: usize,
    lpstartaddress: ::std::option::Option<super::WindowsProgramming::LPFIBER_START_ROUTINE>,
    lpparameter: *const ::std::ffi::c_void,
) -> *mut ::std::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateFiber(
                dwstacksize: usize,
                lpstartaddress: ::windows::runtime::RawPtr,
                lpparameter: *const ::std::ffi::c_void,
            ) -> *mut ::std::ffi::c_void;
        }
        ::std::mem::transmute(CreateFiber(
            ::std::mem::transmute(dwstacksize),
            ::std::mem::transmute(lpstartaddress),
            ::std::mem::transmute(lpparameter),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_WindowsProgramming")]
pub unsafe fn CreateFiberEx(
    dwstackcommitsize: usize,
    dwstackreservesize: usize,
    dwflags: u32,
    lpstartaddress: ::std::option::Option<super::WindowsProgramming::LPFIBER_START_ROUTINE>,
    lpparameter: *const ::std::ffi::c_void,
) -> *mut ::std::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateFiberEx(
                dwstackcommitsize: usize,
                dwstackreservesize: usize,
                dwflags: u32,
                lpstartaddress: ::windows::runtime::RawPtr,
                lpparameter: *const ::std::ffi::c_void,
            ) -> *mut ::std::ffi::c_void;
        }
        ::std::mem::transmute(CreateFiberEx(
            ::std::mem::transmute(dwstackcommitsize),
            ::std::mem::transmute(dwstackreservesize),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(lpstartaddress),
            ::std::mem::transmute(lpparameter),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub unsafe fn CreateMutexA<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lpmutexattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
    binitialowner: Param1,
    lpname: Param2,
) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateMutexA(
                lpmutexattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
                binitialowner: super::super::Foundation::BOOL,
                lpname: super::super::Foundation::PSTR,
            ) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(CreateMutexA(
            ::std::mem::transmute(lpmutexattributes),
            binitialowner.into_param().abi(),
            lpname.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub unsafe fn CreateMutexExA<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lpmutexattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
    lpname: Param1,
    dwflags: u32,
    dwdesiredaccess: u32,
) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateMutexExA(
                lpmutexattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
                lpname: super::super::Foundation::PSTR,
                dwflags: u32,
                dwdesiredaccess: u32,
            ) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(CreateMutexExA(
            ::std::mem::transmute(lpmutexattributes),
            lpname.into_param().abi(),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(dwdesiredaccess),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub unsafe fn CreateMutexExW<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpmutexattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
    lpname: Param1,
    dwflags: u32,
    dwdesiredaccess: u32,
) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateMutexExW(
                lpmutexattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
                lpname: super::super::Foundation::PWSTR,
                dwflags: u32,
                dwdesiredaccess: u32,
            ) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(CreateMutexExW(
            ::std::mem::transmute(lpmutexattributes),
            lpname.into_param().abi(),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(dwdesiredaccess),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub unsafe fn CreateMutexW<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpmutexattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
    binitialowner: Param1,
    lpname: Param2,
) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateMutexW(
                lpmutexattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
                binitialowner: super::super::Foundation::BOOL,
                lpname: super::super::Foundation::PWSTR,
            ) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(CreateMutexW(
            ::std::mem::transmute(lpmutexattributes),
            binitialowner.into_param().abi(),
            lpname.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub unsafe fn CreatePrivateNamespaceA<
    'a,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lpprivatenamespaceattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
    lpboundarydescriptor: *const ::std::ffi::c_void,
    lpaliasprefix: Param2,
) -> NamespaceHandle {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreatePrivateNamespaceA(
                lpprivatenamespaceattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
                lpboundarydescriptor: *const ::std::ffi::c_void,
                lpaliasprefix: super::super::Foundation::PSTR,
            ) -> NamespaceHandle;
        }
        ::std::mem::transmute(CreatePrivateNamespaceA(
            ::std::mem::transmute(lpprivatenamespaceattributes),
            ::std::mem::transmute(lpboundarydescriptor),
            lpaliasprefix.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub unsafe fn CreatePrivateNamespaceW<
    'a,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpprivatenamespaceattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
    lpboundarydescriptor: *const ::std::ffi::c_void,
    lpaliasprefix: Param2,
) -> NamespaceHandle {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreatePrivateNamespaceW(
                lpprivatenamespaceattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
                lpboundarydescriptor: *const ::std::ffi::c_void,
                lpaliasprefix: super::super::Foundation::PWSTR,
            ) -> NamespaceHandle;
        }
        ::std::mem::transmute(CreatePrivateNamespaceW(
            ::std::mem::transmute(lpprivatenamespaceattributes),
            ::std::mem::transmute(lpboundarydescriptor),
            lpaliasprefix.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub unsafe fn CreateProcessA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    Param7: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lpapplicationname: Param0,
    lpcommandline: Param1,
    lpprocessattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
    lpthreadattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
    binherithandles: Param4,
    dwcreationflags: PROCESS_CREATION_FLAGS,
    lpenvironment: *const ::std::ffi::c_void,
    lpcurrentdirectory: Param7,
    lpstartupinfo: *const STARTUPINFOA,
    lpprocessinformation: *mut PROCESS_INFORMATION,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateProcessA(
                lpapplicationname: super::super::Foundation::PSTR,
                lpcommandline: super::super::Foundation::PSTR,
                lpprocessattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
                lpthreadattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
                binherithandles: super::super::Foundation::BOOL,
                dwcreationflags: PROCESS_CREATION_FLAGS,
                lpenvironment: *const ::std::ffi::c_void,
                lpcurrentdirectory: super::super::Foundation::PSTR,
                lpstartupinfo: *const STARTUPINFOA,
                lpprocessinformation: *mut PROCESS_INFORMATION,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CreateProcessA(
            lpapplicationname.into_param().abi(),
            lpcommandline.into_param().abi(),
            ::std::mem::transmute(lpprocessattributes),
            ::std::mem::transmute(lpthreadattributes),
            binherithandles.into_param().abi(),
            ::std::mem::transmute(dwcreationflags),
            ::std::mem::transmute(lpenvironment),
            lpcurrentdirectory.into_param().abi(),
            ::std::mem::transmute(lpstartupinfo),
            ::std::mem::transmute(lpprocessinformation),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub unsafe fn CreateProcessAsUserA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    Param8: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    htoken: Param0,
    lpapplicationname: Param1,
    lpcommandline: Param2,
    lpprocessattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
    lpthreadattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
    binherithandles: Param5,
    dwcreationflags: u32,
    lpenvironment: *const ::std::ffi::c_void,
    lpcurrentdirectory: Param8,
    lpstartupinfo: *const STARTUPINFOA,
    lpprocessinformation: *mut PROCESS_INFORMATION,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateProcessAsUserA(
                htoken: super::super::Foundation::HANDLE,
                lpapplicationname: super::super::Foundation::PSTR,
                lpcommandline: super::super::Foundation::PSTR,
                lpprocessattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
                lpthreadattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
                binherithandles: super::super::Foundation::BOOL,
                dwcreationflags: u32,
                lpenvironment: *const ::std::ffi::c_void,
                lpcurrentdirectory: super::super::Foundation::PSTR,
                lpstartupinfo: *const STARTUPINFOA,
                lpprocessinformation: *mut PROCESS_INFORMATION,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CreateProcessAsUserA(
            htoken.into_param().abi(),
            lpapplicationname.into_param().abi(),
            lpcommandline.into_param().abi(),
            ::std::mem::transmute(lpprocessattributes),
            ::std::mem::transmute(lpthreadattributes),
            binherithandles.into_param().abi(),
            ::std::mem::transmute(dwcreationflags),
            ::std::mem::transmute(lpenvironment),
            lpcurrentdirectory.into_param().abi(),
            ::std::mem::transmute(lpstartupinfo),
            ::std::mem::transmute(lpprocessinformation),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub unsafe fn CreateProcessAsUserW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    Param8: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    htoken: Param0,
    lpapplicationname: Param1,
    lpcommandline: Param2,
    lpprocessattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
    lpthreadattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
    binherithandles: Param5,
    dwcreationflags: u32,
    lpenvironment: *const ::std::ffi::c_void,
    lpcurrentdirectory: Param8,
    lpstartupinfo: *const STARTUPINFOW,
    lpprocessinformation: *mut PROCESS_INFORMATION,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateProcessAsUserW(
                htoken: super::super::Foundation::HANDLE,
                lpapplicationname: super::super::Foundation::PWSTR,
                lpcommandline: super::super::Foundation::PWSTR,
                lpprocessattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
                lpthreadattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
                binherithandles: super::super::Foundation::BOOL,
                dwcreationflags: u32,
                lpenvironment: *const ::std::ffi::c_void,
                lpcurrentdirectory: super::super::Foundation::PWSTR,
                lpstartupinfo: *const STARTUPINFOW,
                lpprocessinformation: *mut PROCESS_INFORMATION,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CreateProcessAsUserW(
            htoken.into_param().abi(),
            lpapplicationname.into_param().abi(),
            lpcommandline.into_param().abi(),
            ::std::mem::transmute(lpprocessattributes),
            ::std::mem::transmute(lpthreadattributes),
            binherithandles.into_param().abi(),
            ::std::mem::transmute(dwcreationflags),
            ::std::mem::transmute(lpenvironment),
            lpcurrentdirectory.into_param().abi(),
            ::std::mem::transmute(lpstartupinfo),
            ::std::mem::transmute(lpprocessinformation),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub unsafe fn CreateProcessW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    Param7: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpapplicationname: Param0,
    lpcommandline: Param1,
    lpprocessattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
    lpthreadattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
    binherithandles: Param4,
    dwcreationflags: PROCESS_CREATION_FLAGS,
    lpenvironment: *const ::std::ffi::c_void,
    lpcurrentdirectory: Param7,
    lpstartupinfo: *const STARTUPINFOW,
    lpprocessinformation: *mut PROCESS_INFORMATION,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateProcessW(
                lpapplicationname: super::super::Foundation::PWSTR,
                lpcommandline: super::super::Foundation::PWSTR,
                lpprocessattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
                lpthreadattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
                binherithandles: super::super::Foundation::BOOL,
                dwcreationflags: PROCESS_CREATION_FLAGS,
                lpenvironment: *const ::std::ffi::c_void,
                lpcurrentdirectory: super::super::Foundation::PWSTR,
                lpstartupinfo: *const STARTUPINFOW,
                lpprocessinformation: *mut PROCESS_INFORMATION,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CreateProcessW(
            lpapplicationname.into_param().abi(),
            lpcommandline.into_param().abi(),
            ::std::mem::transmute(lpprocessattributes),
            ::std::mem::transmute(lpthreadattributes),
            binherithandles.into_param().abi(),
            ::std::mem::transmute(dwcreationflags),
            ::std::mem::transmute(lpenvironment),
            lpcurrentdirectory.into_param().abi(),
            ::std::mem::transmute(lpstartupinfo),
            ::std::mem::transmute(lpprocessinformation),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn CreateProcessWithLogonW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param8: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpusername: Param0,
    lpdomain: Param1,
    lppassword: Param2,
    dwlogonflags: CREATE_PROCESS_LOGON_FLAGS,
    lpapplicationname: Param4,
    lpcommandline: Param5,
    dwcreationflags: u32,
    lpenvironment: *const ::std::ffi::c_void,
    lpcurrentdirectory: Param8,
    lpstartupinfo: *const STARTUPINFOW,
    lpprocessinformation: *mut PROCESS_INFORMATION,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateProcessWithLogonW(
                lpusername: super::super::Foundation::PWSTR,
                lpdomain: super::super::Foundation::PWSTR,
                lppassword: super::super::Foundation::PWSTR,
                dwlogonflags: CREATE_PROCESS_LOGON_FLAGS,
                lpapplicationname: super::super::Foundation::PWSTR,
                lpcommandline: super::super::Foundation::PWSTR,
                dwcreationflags: u32,
                lpenvironment: *const ::std::ffi::c_void,
                lpcurrentdirectory: super::super::Foundation::PWSTR,
                lpstartupinfo: *const STARTUPINFOW,
                lpprocessinformation: *mut PROCESS_INFORMATION,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CreateProcessWithLogonW(
            lpusername.into_param().abi(),
            lpdomain.into_param().abi(),
            lppassword.into_param().abi(),
            ::std::mem::transmute(dwlogonflags),
            lpapplicationname.into_param().abi(),
            lpcommandline.into_param().abi(),
            ::std::mem::transmute(dwcreationflags),
            ::std::mem::transmute(lpenvironment),
            lpcurrentdirectory.into_param().abi(),
            ::std::mem::transmute(lpstartupinfo),
            ::std::mem::transmute(lpprocessinformation),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn CreateProcessWithTokenW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param6: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    htoken: Param0,
    dwlogonflags: CREATE_PROCESS_LOGON_FLAGS,
    lpapplicationname: Param2,
    lpcommandline: Param3,
    dwcreationflags: u32,
    lpenvironment: *const ::std::ffi::c_void,
    lpcurrentdirectory: Param6,
    lpstartupinfo: *const STARTUPINFOW,
    lpprocessinformation: *mut PROCESS_INFORMATION,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateProcessWithTokenW(
                htoken: super::super::Foundation::HANDLE,
                dwlogonflags: CREATE_PROCESS_LOGON_FLAGS,
                lpapplicationname: super::super::Foundation::PWSTR,
                lpcommandline: super::super::Foundation::PWSTR,
                dwcreationflags: u32,
                lpenvironment: *const ::std::ffi::c_void,
                lpcurrentdirectory: super::super::Foundation::PWSTR,
                lpstartupinfo: *const STARTUPINFOW,
                lpprocessinformation: *mut PROCESS_INFORMATION,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CreateProcessWithTokenW(
            htoken.into_param().abi(),
            ::std::mem::transmute(dwlogonflags),
            lpapplicationname.into_param().abi(),
            lpcommandline.into_param().abi(),
            ::std::mem::transmute(dwcreationflags),
            ::std::mem::transmute(lpenvironment),
            lpcurrentdirectory.into_param().abi(),
            ::std::mem::transmute(lpstartupinfo),
            ::std::mem::transmute(lpprocessinformation),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security",
    feature = "Win32_System_SystemServices"
))]
pub unsafe fn CreateRemoteThread<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hprocess: Param0,
    lpthreadattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
    dwstacksize: usize,
    lpstartaddress: ::std::option::Option<super::SystemServices::LPTHREAD_START_ROUTINE>,
    lpparameter: *const ::std::ffi::c_void,
    dwcreationflags: u32,
    lpthreadid: *mut u32,
) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateRemoteThread(
                hprocess: super::super::Foundation::HANDLE,
                lpthreadattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
                dwstacksize: usize,
                lpstartaddress: ::windows::runtime::RawPtr,
                lpparameter: *const ::std::ffi::c_void,
                dwcreationflags: u32,
                lpthreadid: *mut u32,
            ) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(CreateRemoteThread(
            hprocess.into_param().abi(),
            ::std::mem::transmute(lpthreadattributes),
            ::std::mem::transmute(dwstacksize),
            ::std::mem::transmute(lpstartaddress),
            ::std::mem::transmute(lpparameter),
            ::std::mem::transmute(dwcreationflags),
            ::std::mem::transmute(lpthreadid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security",
    feature = "Win32_System_SystemServices"
))]
pub unsafe fn CreateRemoteThreadEx<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param6: ::windows::runtime::IntoParam<'a, LPPROC_THREAD_ATTRIBUTE_LIST>,
>(
    hprocess: Param0,
    lpthreadattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
    dwstacksize: usize,
    lpstartaddress: ::std::option::Option<super::SystemServices::LPTHREAD_START_ROUTINE>,
    lpparameter: *const ::std::ffi::c_void,
    dwcreationflags: u32,
    lpattributelist: Param6,
    lpthreadid: *mut u32,
) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateRemoteThreadEx(
                hprocess: super::super::Foundation::HANDLE,
                lpthreadattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
                dwstacksize: usize,
                lpstartaddress: ::windows::runtime::RawPtr,
                lpparameter: *const ::std::ffi::c_void,
                dwcreationflags: u32,
                lpattributelist: LPPROC_THREAD_ATTRIBUTE_LIST,
                lpthreadid: *mut u32,
            ) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(CreateRemoteThreadEx(
            hprocess.into_param().abi(),
            ::std::mem::transmute(lpthreadattributes),
            ::std::mem::transmute(dwstacksize),
            ::std::mem::transmute(lpstartaddress),
            ::std::mem::transmute(lpparameter),
            ::std::mem::transmute(dwcreationflags),
            lpattributelist.into_param().abi(),
            ::std::mem::transmute(lpthreadid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub unsafe fn CreateSemaphoreA<
    'a,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lpsemaphoreattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
    linitialcount: i32,
    lmaximumcount: i32,
    lpname: Param3,
) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateSemaphoreA(
                lpsemaphoreattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
                linitialcount: i32,
                lmaximumcount: i32,
                lpname: super::super::Foundation::PSTR,
            ) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(CreateSemaphoreA(
            ::std::mem::transmute(lpsemaphoreattributes),
            ::std::mem::transmute(linitialcount),
            ::std::mem::transmute(lmaximumcount),
            lpname.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub unsafe fn CreateSemaphoreExA<
    'a,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lpsemaphoreattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
    linitialcount: i32,
    lmaximumcount: i32,
    lpname: Param3,
    dwflags: u32,
    dwdesiredaccess: u32,
) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateSemaphoreExA(
                lpsemaphoreattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
                linitialcount: i32,
                lmaximumcount: i32,
                lpname: super::super::Foundation::PSTR,
                dwflags: u32,
                dwdesiredaccess: u32,
            ) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(CreateSemaphoreExA(
            ::std::mem::transmute(lpsemaphoreattributes),
            ::std::mem::transmute(linitialcount),
            ::std::mem::transmute(lmaximumcount),
            lpname.into_param().abi(),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(dwdesiredaccess),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub unsafe fn CreateSemaphoreExW<
    'a,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpsemaphoreattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
    linitialcount: i32,
    lmaximumcount: i32,
    lpname: Param3,
    dwflags: u32,
    dwdesiredaccess: u32,
) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateSemaphoreExW(
                lpsemaphoreattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
                linitialcount: i32,
                lmaximumcount: i32,
                lpname: super::super::Foundation::PWSTR,
                dwflags: u32,
                dwdesiredaccess: u32,
            ) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(CreateSemaphoreExW(
            ::std::mem::transmute(lpsemaphoreattributes),
            ::std::mem::transmute(linitialcount),
            ::std::mem::transmute(lmaximumcount),
            lpname.into_param().abi(),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(dwdesiredaccess),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub unsafe fn CreateSemaphoreW<
    'a,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpsemaphoreattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
    linitialcount: i32,
    lmaximumcount: i32,
    lpname: Param3,
) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateSemaphoreW(
                lpsemaphoreattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
                linitialcount: i32,
                lmaximumcount: i32,
                lpname: super::super::Foundation::PWSTR,
            ) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(CreateSemaphoreW(
            ::std::mem::transmute(lpsemaphoreattributes),
            ::std::mem::transmute(linitialcount),
            ::std::mem::transmute(lmaximumcount),
            lpname.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security",
    feature = "Win32_System_SystemServices"
))]
pub unsafe fn CreateThread(
    lpthreadattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
    dwstacksize: usize,
    lpstartaddress: ::std::option::Option<super::SystemServices::LPTHREAD_START_ROUTINE>,
    lpparameter: *const ::std::ffi::c_void,
    dwcreationflags: THREAD_CREATION_FLAGS,
    lpthreadid: *mut u32,
) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateThread(
                lpthreadattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
                dwstacksize: usize,
                lpstartaddress: ::windows::runtime::RawPtr,
                lpparameter: *const ::std::ffi::c_void,
                dwcreationflags: THREAD_CREATION_FLAGS,
                lpthreadid: *mut u32,
            ) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(CreateThread(
            ::std::mem::transmute(lpthreadattributes),
            ::std::mem::transmute(dwstacksize),
            ::std::mem::transmute(lpstartaddress),
            ::std::mem::transmute(lpparameter),
            ::std::mem::transmute(dwcreationflags),
            ::std::mem::transmute(lpthreadid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn CreateThreadpool(reserved: *mut ::std::ffi::c_void) -> PTP_POOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateThreadpool(reserved: *mut ::std::ffi::c_void) -> PTP_POOL;
        }
        ::std::mem::transmute(CreateThreadpool(::std::mem::transmute(reserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn CreateThreadpoolCleanupGroup() -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateThreadpoolCleanupGroup() -> isize;
        }
        ::std::mem::transmute(CreateThreadpoolCleanupGroup())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn CreateThreadpoolIo<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    fl: Param0,
    pfnio: ::std::option::Option<PTP_WIN32_IO_CALLBACK>,
    pv: *mut ::std::ffi::c_void,
    pcbe: *const super::SystemServices::TP_CALLBACK_ENVIRON_V3,
) -> *mut super::SystemServices::TP_IO {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateThreadpoolIo(
                fl: super::super::Foundation::HANDLE,
                pfnio: ::windows::runtime::RawPtr,
                pv: *mut ::std::ffi::c_void,
                pcbe: *const ::std::mem::ManuallyDrop<
                    super::SystemServices::TP_CALLBACK_ENVIRON_V3,
                >,
            ) -> *mut super::SystemServices::TP_IO;
        }
        ::std::mem::transmute(CreateThreadpoolIo(
            fl.into_param().abi(),
            ::std::mem::transmute(pfnio),
            ::std::mem::transmute(pv),
            ::std::mem::transmute(pcbe),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn CreateThreadpoolTimer(
    pfnti: ::std::option::Option<super::SystemServices::PTP_TIMER_CALLBACK>,
    pv: *mut ::std::ffi::c_void,
    pcbe: *const super::SystemServices::TP_CALLBACK_ENVIRON_V3,
) -> *mut super::SystemServices::TP_TIMER {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateThreadpoolTimer(
                pfnti: ::windows::runtime::RawPtr,
                pv: *mut ::std::ffi::c_void,
                pcbe: *const ::std::mem::ManuallyDrop<
                    super::SystemServices::TP_CALLBACK_ENVIRON_V3,
                >,
            ) -> *mut super::SystemServices::TP_TIMER;
        }
        ::std::mem::transmute(CreateThreadpoolTimer(
            ::std::mem::transmute(pfnti),
            ::std::mem::transmute(pv),
            ::std::mem::transmute(pcbe),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn CreateThreadpoolWait(
    pfnwa: ::std::option::Option<super::SystemServices::PTP_WAIT_CALLBACK>,
    pv: *mut ::std::ffi::c_void,
    pcbe: *const super::SystemServices::TP_CALLBACK_ENVIRON_V3,
) -> *mut super::SystemServices::TP_WAIT {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateThreadpoolWait(
                pfnwa: ::windows::runtime::RawPtr,
                pv: *mut ::std::ffi::c_void,
                pcbe: *const ::std::mem::ManuallyDrop<
                    super::SystemServices::TP_CALLBACK_ENVIRON_V3,
                >,
            ) -> *mut super::SystemServices::TP_WAIT;
        }
        ::std::mem::transmute(CreateThreadpoolWait(
            ::std::mem::transmute(pfnwa),
            ::std::mem::transmute(pv),
            ::std::mem::transmute(pcbe),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn CreateThreadpoolWork(
    pfnwk: ::std::option::Option<super::SystemServices::PTP_WORK_CALLBACK>,
    pv: *mut ::std::ffi::c_void,
    pcbe: *const super::SystemServices::TP_CALLBACK_ENVIRON_V3,
) -> *mut super::SystemServices::TP_WORK {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateThreadpoolWork(
                pfnwk: ::windows::runtime::RawPtr,
                pv: *mut ::std::ffi::c_void,
                pcbe: *const ::std::mem::ManuallyDrop<
                    super::SystemServices::TP_CALLBACK_ENVIRON_V3,
                >,
            ) -> *mut super::SystemServices::TP_WORK;
        }
        ::std::mem::transmute(CreateThreadpoolWork(
            ::std::mem::transmute(pfnwk),
            ::std::mem::transmute(pv),
            ::std::mem::transmute(pcbe),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn CreateTimerQueue() -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateTimerQueue() -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(CreateTimerQueue())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn CreateTimerQueueTimer<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    phnewtimer: *mut super::super::Foundation::HANDLE,
    timerqueue: Param1,
    callback: ::std::option::Option<super::SystemServices::WAITORTIMERCALLBACK>,
    parameter: *const ::std::ffi::c_void,
    duetime: u32,
    period: u32,
    flags: WORKER_THREAD_FLAGS,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateTimerQueueTimer(
                phnewtimer: *mut super::super::Foundation::HANDLE,
                timerqueue: super::super::Foundation::HANDLE,
                callback: ::windows::runtime::RawPtr,
                parameter: *const ::std::ffi::c_void,
                duetime: u32,
                period: u32,
                flags: WORKER_THREAD_FLAGS,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CreateTimerQueueTimer(
            ::std::mem::transmute(phnewtimer),
            timerqueue.into_param().abi(),
            ::std::mem::transmute(callback),
            ::std::mem::transmute(parameter),
            ::std::mem::transmute(duetime),
            ::std::mem::transmute(period),
            ::std::mem::transmute(flags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn CreateUmsCompletionList(
    umscompletionlist: *mut *mut ::std::ffi::c_void,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateUmsCompletionList(
                umscompletionlist: *mut *mut ::std::ffi::c_void,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CreateUmsCompletionList(::std::mem::transmute(
            umscompletionlist,
        )))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn CreateUmsThreadContext(
    lpumsthread: *mut *mut ::std::ffi::c_void,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateUmsThreadContext(
                lpumsthread: *mut *mut ::std::ffi::c_void,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CreateUmsThreadContext(::std::mem::transmute(lpumsthread)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub unsafe fn CreateWaitableTimerExW<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lptimerattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
    lptimername: Param1,
    dwflags: u32,
    dwdesiredaccess: u32,
) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateWaitableTimerExW(
                lptimerattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
                lptimername: super::super::Foundation::PWSTR,
                dwflags: u32,
                dwdesiredaccess: u32,
            ) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(CreateWaitableTimerExW(
            ::std::mem::transmute(lptimerattributes),
            lptimername.into_param().abi(),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(dwdesiredaccess),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub unsafe fn CreateWaitableTimerW<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lptimerattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
    bmanualreset: Param1,
    lptimername: Param2,
) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateWaitableTimerW(
                lptimerattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
                bmanualreset: super::super::Foundation::BOOL,
                lptimername: super::super::Foundation::PWSTR,
            ) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(CreateWaitableTimerW(
            ::std::mem::transmute(lptimerattributes),
            bmanualreset.into_param().abi(),
            lptimername.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn DeleteBoundaryDescriptor<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, BoundaryDescriptorHandle>,
>(
    boundarydescriptor: Param0,
) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeleteBoundaryDescriptor(boundarydescriptor: BoundaryDescriptorHandle);
        }
        ::std::mem::transmute(DeleteBoundaryDescriptor(
            boundarydescriptor.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Kernel",
    feature = "Win32_System_SystemServices"
))]
pub unsafe fn DeleteCriticalSection(
    lpcriticalsection: *mut super::SystemServices::RTL_CRITICAL_SECTION,
) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeleteCriticalSection(
                lpcriticalsection: *mut super::SystemServices::RTL_CRITICAL_SECTION,
            );
        }
        ::std::mem::transmute(DeleteCriticalSection(::std::mem::transmute(
            lpcriticalsection,
        )))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn DeleteFiber(lpfiber: *const ::std::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeleteFiber(lpfiber: *const ::std::ffi::c_void);
        }
        ::std::mem::transmute(DeleteFiber(::std::mem::transmute(lpfiber)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn DeleteProcThreadAttributeList<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, LPPROC_THREAD_ATTRIBUTE_LIST>,
>(
    lpattributelist: Param0,
) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeleteProcThreadAttributeList(lpattributelist: LPPROC_THREAD_ATTRIBUTE_LIST);
        }
        ::std::mem::transmute(DeleteProcThreadAttributeList(
            lpattributelist.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn DeleteSynchronizationBarrier(
    lpbarrier: *mut super::SystemServices::RTL_BARRIER,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeleteSynchronizationBarrier(
                lpbarrier: *mut super::SystemServices::RTL_BARRIER,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(DeleteSynchronizationBarrier(::std::mem::transmute(
            lpbarrier,
        )))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn DeleteTimerQueue<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    timerqueue: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeleteTimerQueue(
                timerqueue: super::super::Foundation::HANDLE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(DeleteTimerQueue(timerqueue.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn DeleteTimerQueueEx<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    timerqueue: Param0,
    completionevent: Param1,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeleteTimerQueueEx(
                timerqueue: super::super::Foundation::HANDLE,
                completionevent: super::super::Foundation::HANDLE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(DeleteTimerQueueEx(
            timerqueue.into_param().abi(),
            completionevent.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn DeleteTimerQueueTimer<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    timerqueue: Param0,
    timer: Param1,
    completionevent: Param2,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeleteTimerQueueTimer(
                timerqueue: super::super::Foundation::HANDLE,
                timer: super::super::Foundation::HANDLE,
                completionevent: super::super::Foundation::HANDLE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(DeleteTimerQueueTimer(
            timerqueue.into_param().abi(),
            timer.into_param().abi(),
            completionevent.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn DeleteUmsCompletionList(
    umscompletionlist: *const ::std::ffi::c_void,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeleteUmsCompletionList(
                umscompletionlist: *const ::std::ffi::c_void,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(DeleteUmsCompletionList(::std::mem::transmute(
            umscompletionlist,
        )))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn DeleteUmsThreadContext(
    umsthread: *const ::std::ffi::c_void,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeleteUmsThreadContext(
                umsthread: *const ::std::ffi::c_void,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(DeleteUmsThreadContext(::std::mem::transmute(umsthread)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn DequeueUmsCompletionListItems(
    umscompletionlist: *const ::std::ffi::c_void,
    waittimeout: u32,
    umsthreadlist: *mut *mut ::std::ffi::c_void,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DequeueUmsCompletionListItems(
                umscompletionlist: *const ::std::ffi::c_void,
                waittimeout: u32,
                umsthreadlist: *mut *mut ::std::ffi::c_void,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(DequeueUmsCompletionListItems(
            ::std::mem::transmute(umscompletionlist),
            ::std::mem::transmute(waittimeout),
            ::std::mem::transmute(umsthreadlist),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn DisassociateCurrentThreadFromCallback(
    pci: *mut super::SystemServices::TP_CALLBACK_INSTANCE,
) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DisassociateCurrentThreadFromCallback(
                pci: *mut super::SystemServices::TP_CALLBACK_INSTANCE,
            );
        }
        ::std::mem::transmute(DisassociateCurrentThreadFromCallback(
            ::std::mem::transmute(pci),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Kernel",
    feature = "Win32_System_SystemServices"
))]
pub unsafe fn EnterCriticalSection(
    lpcriticalsection: *mut super::SystemServices::RTL_CRITICAL_SECTION,
) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnterCriticalSection(
                lpcriticalsection: *mut super::SystemServices::RTL_CRITICAL_SECTION,
            );
        }
        ::std::mem::transmute(EnterCriticalSection(::std::mem::transmute(
            lpcriticalsection,
        )))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn EnterSynchronizationBarrier(
    lpbarrier: *mut super::SystemServices::RTL_BARRIER,
    dwflags: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnterSynchronizationBarrier(
                lpbarrier: *mut super::SystemServices::RTL_BARRIER,
                dwflags: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(EnterSynchronizationBarrier(
            ::std::mem::transmute(lpbarrier),
            ::std::mem::transmute(dwflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn EnterUmsSchedulingMode(
    schedulerstartupinfo: *const UMS_SCHEDULER_STARTUP_INFO,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnterUmsSchedulingMode(
                schedulerstartupinfo: *const ::std::mem::ManuallyDrop<UMS_SCHEDULER_STARTUP_INFO>,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(EnterUmsSchedulingMode(::std::mem::transmute(
            schedulerstartupinfo,
        )))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn ExecuteUmsThread(
    umsthread: *mut ::std::ffi::c_void,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ExecuteUmsThread(
                umsthread: *mut ::std::ffi::c_void,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ExecuteUmsThread(::std::mem::transmute(umsthread)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn ExitProcess(uexitcode: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ExitProcess(uexitcode: u32);
        }
        ::std::mem::transmute(ExitProcess(::std::mem::transmute(uexitcode)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn ExitThread(dwexitcode: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ExitThread(dwexitcode: u32);
        }
        ::std::mem::transmute(ExitThread(::std::mem::transmute(dwexitcode)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn FlsAlloc(
    lpcallback: ::std::option::Option<super::SystemServices::PFLS_CALLBACK_FUNCTION>,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FlsAlloc(lpcallback: ::windows::runtime::RawPtr) -> u32;
        }
        ::std::mem::transmute(FlsAlloc(::std::mem::transmute(lpcallback)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn FlsFree(dwflsindex: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FlsFree(dwflsindex: u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FlsFree(::std::mem::transmute(dwflsindex)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn FlsGetValue(dwflsindex: u32) -> *mut ::std::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FlsGetValue(dwflsindex: u32) -> *mut ::std::ffi::c_void;
        }
        ::std::mem::transmute(FlsGetValue(::std::mem::transmute(dwflsindex)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn FlsSetValue(
    dwflsindex: u32,
    lpflsdata: *const ::std::ffi::c_void,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FlsSetValue(
                dwflsindex: u32,
                lpflsdata: *const ::std::ffi::c_void,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FlsSetValue(
            ::std::mem::transmute(dwflsindex),
            ::std::mem::transmute(lpflsdata),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn FlushProcessWriteBuffers() {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FlushProcessWriteBuffers();
        }
        ::std::mem::transmute(FlushProcessWriteBuffers())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn FreeLibraryWhenCallbackReturns<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>,
>(
    pci: *mut super::SystemServices::TP_CALLBACK_INSTANCE,
    r#mod: Param1,
) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FreeLibraryWhenCallbackReturns(
                pci: *mut super::SystemServices::TP_CALLBACK_INSTANCE,
                r#mod: super::super::Foundation::HINSTANCE,
            );
        }
        ::std::mem::transmute(FreeLibraryWhenCallbackReturns(
            ::std::mem::transmute(pci),
            r#mod.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct GET_GUI_RESOURCES_FLAGS(pub u32);
pub const GR_GDIOBJECTS: GET_GUI_RESOURCES_FLAGS = GET_GUI_RESOURCES_FLAGS(0u32);
pub const GR_GDIOBJECTS_PEAK: GET_GUI_RESOURCES_FLAGS = GET_GUI_RESOURCES_FLAGS(2u32);
pub const GR_USEROBJECTS: GET_GUI_RESOURCES_FLAGS = GET_GUI_RESOURCES_FLAGS(1u32);
pub const GR_USEROBJECTS_PEAK: GET_GUI_RESOURCES_FLAGS = GET_GUI_RESOURCES_FLAGS(4u32);
impl ::std::convert::From<u32> for GET_GUI_RESOURCES_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GET_GUI_RESOURCES_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for GET_GUI_RESOURCES_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for GET_GUI_RESOURCES_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for GET_GUI_RESOURCES_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for GET_GUI_RESOURCES_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for GET_GUI_RESOURCES_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub unsafe fn GetActiveProcessorCount(groupnumber: u16) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetActiveProcessorCount(groupnumber: u16) -> u32;
        }
        ::std::mem::transmute(GetActiveProcessorCount(::std::mem::transmute(groupnumber)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn GetActiveProcessorGroupCount() -> u16 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetActiveProcessorGroupCount() -> u16;
        }
        ::std::mem::transmute(GetActiveProcessorGroupCount())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetCurrentProcess() -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCurrentProcess() -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(GetCurrentProcess())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn GetCurrentProcessId() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCurrentProcessId() -> u32;
        }
        ::std::mem::transmute(GetCurrentProcessId())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn GetCurrentProcessorNumber() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCurrentProcessorNumber() -> u32;
        }
        ::std::mem::transmute(GetCurrentProcessorNumber())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Kernel")]
pub unsafe fn GetCurrentProcessorNumberEx(procnumber: *mut super::Kernel::PROCESSOR_NUMBER) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCurrentProcessorNumberEx(procnumber: *mut super::Kernel::PROCESSOR_NUMBER);
        }
        ::std::mem::transmute(GetCurrentProcessorNumberEx(::std::mem::transmute(
            procnumber,
        )))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetCurrentThread() -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCurrentThread() -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(GetCurrentThread())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn GetCurrentThreadId() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCurrentThreadId() -> u32;
        }
        ::std::mem::transmute(GetCurrentThreadId())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn GetCurrentThreadStackLimits(lowlimit: *mut usize, highlimit: *mut usize) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCurrentThreadStackLimits(lowlimit: *mut usize, highlimit: *mut usize);
        }
        ::std::mem::transmute(GetCurrentThreadStackLimits(
            ::std::mem::transmute(lowlimit),
            ::std::mem::transmute(highlimit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn GetCurrentUmsThread() -> *mut ::std::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCurrentUmsThread() -> *mut ::std::ffi::c_void;
        }
        ::std::mem::transmute(GetCurrentUmsThread())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetExitCodeProcess<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hprocess: Param0,
    lpexitcode: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetExitCodeProcess(
                hprocess: super::super::Foundation::HANDLE,
                lpexitcode: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetExitCodeProcess(
            hprocess.into_param().abi(),
            ::std::mem::transmute(lpexitcode),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetExitCodeThread<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hthread: Param0,
    lpexitcode: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetExitCodeThread(
                hthread: super::super::Foundation::HANDLE,
                lpexitcode: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetExitCodeThread(
            hthread.into_param().abi(),
            ::std::mem::transmute(lpexitcode),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetGuiResources<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hprocess: Param0,
    uiflags: GET_GUI_RESOURCES_FLAGS,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetGuiResources(
                hprocess: super::super::Foundation::HANDLE,
                uiflags: GET_GUI_RESOURCES_FLAGS,
            ) -> u32;
        }
        ::std::mem::transmute(GetGuiResources(
            hprocess.into_param().abi(),
            ::std::mem::transmute(uiflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn GetMachineTypeAttributes(
    machine: u16,
) -> ::windows::runtime::Result<MACHINE_ATTRIBUTES> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetMachineTypeAttributes(
                machine: u16,
                machinetypeattributes: *mut MACHINE_ATTRIBUTES,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <MACHINE_ATTRIBUTES as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        GetMachineTypeAttributes(::std::mem::transmute(machine), &mut result__)
            .from_abi::<MACHINE_ATTRIBUTES>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn GetMaximumProcessorCount(groupnumber: u16) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetMaximumProcessorCount(groupnumber: u16) -> u32;
        }
        ::std::mem::transmute(GetMaximumProcessorCount(::std::mem::transmute(groupnumber)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn GetMaximumProcessorGroupCount() -> u16 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetMaximumProcessorGroupCount() -> u16;
        }
        ::std::mem::transmute(GetMaximumProcessorGroupCount())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn GetNextUmsListItem(umscontext: *mut ::std::ffi::c_void) -> *mut ::std::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetNextUmsListItem(umscontext: *mut ::std::ffi::c_void) -> *mut ::std::ffi::c_void;
        }
        ::std::mem::transmute(GetNextUmsListItem(::std::mem::transmute(umscontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetNumaAvailableMemoryNode(
    node: u8,
    availablebytes: *mut u64,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetNumaAvailableMemoryNode(
                node: u8,
                availablebytes: *mut u64,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetNumaAvailableMemoryNode(
            ::std::mem::transmute(node),
            ::std::mem::transmute(availablebytes),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetNumaAvailableMemoryNodeEx(
    node: u16,
    availablebytes: *mut u64,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetNumaAvailableMemoryNodeEx(
                node: u16,
                availablebytes: *mut u64,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetNumaAvailableMemoryNodeEx(
            ::std::mem::transmute(node),
            ::std::mem::transmute(availablebytes),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetNumaHighestNodeNumber(
    highestnodenumber: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetNumaHighestNodeNumber(
                highestnodenumber: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetNumaHighestNodeNumber(::std::mem::transmute(
            highestnodenumber,
        )))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetNumaNodeNumberFromHandle<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hfile: Param0,
    nodenumber: *mut u16,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetNumaNodeNumberFromHandle(
                hfile: super::super::Foundation::HANDLE,
                nodenumber: *mut u16,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetNumaNodeNumberFromHandle(
            hfile.into_param().abi(),
            ::std::mem::transmute(nodenumber),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetNumaNodeProcessorMask(
    node: u8,
    processormask: *mut u64,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetNumaNodeProcessorMask(
                node: u8,
                processormask: *mut u64,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetNumaNodeProcessorMask(
            ::std::mem::transmute(node),
            ::std::mem::transmute(processormask),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub unsafe fn GetNumaNodeProcessorMask2(
    nodenumber: u16,
    processormasks: *mut super::Kernel::GROUP_AFFINITY,
    processormaskcount: u16,
    requiredmaskcount: *mut u16,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetNumaNodeProcessorMask2(
                nodenumber: u16,
                processormasks: *mut super::Kernel::GROUP_AFFINITY,
                processormaskcount: u16,
                requiredmaskcount: *mut u16,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetNumaNodeProcessorMask2(
            ::std::mem::transmute(nodenumber),
            ::std::mem::transmute(processormasks),
            ::std::mem::transmute(processormaskcount),
            ::std::mem::transmute(requiredmaskcount),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub unsafe fn GetNumaNodeProcessorMaskEx(
    node: u16,
    processormask: *mut super::Kernel::GROUP_AFFINITY,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetNumaNodeProcessorMaskEx(
                node: u16,
                processormask: *mut super::Kernel::GROUP_AFFINITY,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetNumaNodeProcessorMaskEx(
            ::std::mem::transmute(node),
            ::std::mem::transmute(processormask),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetNumaProcessorNode(
    processor: u8,
    nodenumber: *mut u8,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetNumaProcessorNode(
                processor: u8,
                nodenumber: *mut u8,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetNumaProcessorNode(
            ::std::mem::transmute(processor),
            ::std::mem::transmute(nodenumber),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub unsafe fn GetNumaProcessorNodeEx(
    processor: *const super::Kernel::PROCESSOR_NUMBER,
    nodenumber: *mut u16,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetNumaProcessorNodeEx(
                processor: *const super::Kernel::PROCESSOR_NUMBER,
                nodenumber: *mut u16,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetNumaProcessorNodeEx(
            ::std::mem::transmute(processor),
            ::std::mem::transmute(nodenumber),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetNumaProximityNode(
    proximityid: u32,
    nodenumber: *mut u8,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetNumaProximityNode(
                proximityid: u32,
                nodenumber: *mut u8,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetNumaProximityNode(
            ::std::mem::transmute(proximityid),
            ::std::mem::transmute(nodenumber),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetNumaProximityNodeEx(
    proximityid: u32,
    nodenumber: *mut u16,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetNumaProximityNodeEx(
                proximityid: u32,
                nodenumber: *mut u16,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetNumaProximityNodeEx(
            ::std::mem::transmute(proximityid),
            ::std::mem::transmute(nodenumber),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetPriorityClass<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hprocess: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPriorityClass(hprocess: super::super::Foundation::HANDLE) -> u32;
        }
        ::std::mem::transmute(GetPriorityClass(hprocess.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetProcessAffinityMask<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hprocess: Param0,
    lpprocessaffinitymask: *mut usize,
    lpsystemaffinitymask: *mut usize,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetProcessAffinityMask(
                hprocess: super::super::Foundation::HANDLE,
                lpprocessaffinitymask: *mut usize,
                lpsystemaffinitymask: *mut usize,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetProcessAffinityMask(
            hprocess.into_param().abi(),
            ::std::mem::transmute(lpprocessaffinitymask),
            ::std::mem::transmute(lpsystemaffinitymask),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetProcessDEPPolicy<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hprocess: Param0,
    lpflags: *mut u32,
    lppermanent: *mut super::super::Foundation::BOOL,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetProcessDEPPolicy(
                hprocess: super::super::Foundation::HANDLE,
                lpflags: *mut u32,
                lppermanent: *mut super::super::Foundation::BOOL,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetProcessDEPPolicy(
            hprocess.into_param().abi(),
            ::std::mem::transmute(lpflags),
            ::std::mem::transmute(lppermanent),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub unsafe fn GetProcessDefaultCpuSetMasks<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    process: Param0,
    cpusetmasks: *mut super::Kernel::GROUP_AFFINITY,
    cpusetmaskcount: u16,
    requiredmaskcount: *mut u16,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetProcessDefaultCpuSetMasks(
                process: super::super::Foundation::HANDLE,
                cpusetmasks: *mut super::Kernel::GROUP_AFFINITY,
                cpusetmaskcount: u16,
                requiredmaskcount: *mut u16,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetProcessDefaultCpuSetMasks(
            process.into_param().abi(),
            ::std::mem::transmute(cpusetmasks),
            ::std::mem::transmute(cpusetmaskcount),
            ::std::mem::transmute(requiredmaskcount),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetProcessDefaultCpuSets<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    process: Param0,
    cpusetids: *mut u32,
    cpusetidcount: u32,
    requiredidcount: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetProcessDefaultCpuSets(
                process: super::super::Foundation::HANDLE,
                cpusetids: *mut u32,
                cpusetidcount: u32,
                requiredidcount: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetProcessDefaultCpuSets(
            process.into_param().abi(),
            ::std::mem::transmute(cpusetids),
            ::std::mem::transmute(cpusetidcount),
            ::std::mem::transmute(requiredidcount),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetProcessGroupAffinity<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hprocess: Param0,
    groupcount: *mut u16,
    grouparray: *mut u16,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetProcessGroupAffinity(
                hprocess: super::super::Foundation::HANDLE,
                groupcount: *mut u16,
                grouparray: *mut u16,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetProcessGroupAffinity(
            hprocess.into_param().abi(),
            ::std::mem::transmute(groupcount),
            ::std::mem::transmute(grouparray),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetProcessHandleCount<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hprocess: Param0,
    pdwhandlecount: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetProcessHandleCount(
                hprocess: super::super::Foundation::HANDLE,
                pdwhandlecount: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetProcessHandleCount(
            hprocess.into_param().abi(),
            ::std::mem::transmute(pdwhandlecount),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetProcessId<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    process: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetProcessId(process: super::super::Foundation::HANDLE) -> u32;
        }
        ::std::mem::transmute(GetProcessId(process.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetProcessIdOfThread<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    thread: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetProcessIdOfThread(thread: super::super::Foundation::HANDLE) -> u32;
        }
        ::std::mem::transmute(GetProcessIdOfThread(thread.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetProcessInformation<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hprocess: Param0,
    processinformationclass: PROCESS_INFORMATION_CLASS,
    processinformation: *mut ::std::ffi::c_void,
    processinformationsize: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetProcessInformation(
                hprocess: super::super::Foundation::HANDLE,
                processinformationclass: PROCESS_INFORMATION_CLASS,
                processinformation: *mut ::std::ffi::c_void,
                processinformationsize: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetProcessInformation(
            hprocess.into_param().abi(),
            ::std::mem::transmute(processinformationclass),
            ::std::mem::transmute(processinformation),
            ::std::mem::transmute(processinformationsize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn GetProcessIoCounters<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hprocess: Param0,
    lpiocounters: *mut super::SystemServices::IO_COUNTERS,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetProcessIoCounters(
                hprocess: super::super::Foundation::HANDLE,
                lpiocounters: *mut super::SystemServices::IO_COUNTERS,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetProcessIoCounters(
            hprocess.into_param().abi(),
            ::std::mem::transmute(lpiocounters),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn GetProcessMitigationPolicy<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hprocess: Param0,
    mitigationpolicy: super::SystemServices::PROCESS_MITIGATION_POLICY,
    lpbuffer: *mut ::std::ffi::c_void,
    dwlength: usize,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetProcessMitigationPolicy(
                hprocess: super::super::Foundation::HANDLE,
                mitigationpolicy: super::SystemServices::PROCESS_MITIGATION_POLICY,
                lpbuffer: *mut ::std::ffi::c_void,
                dwlength: usize,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetProcessMitigationPolicy(
            hprocess.into_param().abi(),
            ::std::mem::transmute(mitigationpolicy),
            ::std::mem::transmute(lpbuffer),
            ::std::mem::transmute(dwlength),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetProcessPriorityBoost<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hprocess: Param0,
    pdisablepriorityboost: *mut super::super::Foundation::BOOL,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetProcessPriorityBoost(
                hprocess: super::super::Foundation::HANDLE,
                pdisablepriorityboost: *mut super::super::Foundation::BOOL,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetProcessPriorityBoost(
            hprocess.into_param().abi(),
            ::std::mem::transmute(pdisablepriorityboost),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetProcessShutdownParameters(
    lpdwlevel: *mut u32,
    lpdwflags: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetProcessShutdownParameters(
                lpdwlevel: *mut u32,
                lpdwflags: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetProcessShutdownParameters(
            ::std::mem::transmute(lpdwlevel),
            ::std::mem::transmute(lpdwflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetProcessTimes<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hprocess: Param0,
    lpcreationtime: *mut super::super::Foundation::FILETIME,
    lpexittime: *mut super::super::Foundation::FILETIME,
    lpkerneltime: *mut super::super::Foundation::FILETIME,
    lpusertime: *mut super::super::Foundation::FILETIME,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetProcessTimes(
                hprocess: super::super::Foundation::HANDLE,
                lpcreationtime: *mut super::super::Foundation::FILETIME,
                lpexittime: *mut super::super::Foundation::FILETIME,
                lpkerneltime: *mut super::super::Foundation::FILETIME,
                lpusertime: *mut super::super::Foundation::FILETIME,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetProcessTimes(
            hprocess.into_param().abi(),
            ::std::mem::transmute(lpcreationtime),
            ::std::mem::transmute(lpexittime),
            ::std::mem::transmute(lpkerneltime),
            ::std::mem::transmute(lpusertime),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn GetProcessVersion(processid: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetProcessVersion(processid: u32) -> u32;
        }
        ::std::mem::transmute(GetProcessVersion(::std::mem::transmute(processid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetProcessWorkingSetSize<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hprocess: Param0,
    lpminimumworkingsetsize: *mut usize,
    lpmaximumworkingsetsize: *mut usize,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetProcessWorkingSetSize(
                hprocess: super::super::Foundation::HANDLE,
                lpminimumworkingsetsize: *mut usize,
                lpmaximumworkingsetsize: *mut usize,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetProcessWorkingSetSize(
            hprocess.into_param().abi(),
            ::std::mem::transmute(lpminimumworkingsetsize),
            ::std::mem::transmute(lpmaximumworkingsetsize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetStartupInfoW(lpstartupinfo: *mut STARTUPINFOW) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetStartupInfoW(lpstartupinfo: *mut STARTUPINFOW);
        }
        ::std::mem::transmute(GetStartupInfoW(::std::mem::transmute(lpstartupinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn GetSystemCpuSetInformation<
    'a,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    information: *mut super::SystemServices::SYSTEM_CPU_SET_INFORMATION,
    bufferlength: u32,
    returnedlength: *mut u32,
    process: Param3,
    flags: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSystemCpuSetInformation(
                information: *mut super::SystemServices::SYSTEM_CPU_SET_INFORMATION,
                bufferlength: u32,
                returnedlength: *mut u32,
                process: super::super::Foundation::HANDLE,
                flags: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetSystemCpuSetInformation(
            ::std::mem::transmute(information),
            ::std::mem::transmute(bufferlength),
            ::std::mem::transmute(returnedlength),
            process.into_param().abi(),
            ::std::mem::transmute(flags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetSystemTimes(
    lpidletime: *mut super::super::Foundation::FILETIME,
    lpkerneltime: *mut super::super::Foundation::FILETIME,
    lpusertime: *mut super::super::Foundation::FILETIME,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSystemTimes(
                lpidletime: *mut super::super::Foundation::FILETIME,
                lpkerneltime: *mut super::super::Foundation::FILETIME,
                lpusertime: *mut super::super::Foundation::FILETIME,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetSystemTimes(
            ::std::mem::transmute(lpidletime),
            ::std::mem::transmute(lpkerneltime),
            ::std::mem::transmute(lpusertime),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetThreadDescription<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hthread: Param0,
) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThreadDescription(
                hthread: super::super::Foundation::HANDLE,
                ppszthreaddescription: *mut super::super::Foundation::PWSTR,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        GetThreadDescription(hthread.into_param().abi(), &mut result__)
            .from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub unsafe fn GetThreadGroupAffinity<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hthread: Param0,
    groupaffinity: *mut super::Kernel::GROUP_AFFINITY,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThreadGroupAffinity(
                hthread: super::super::Foundation::HANDLE,
                groupaffinity: *mut super::Kernel::GROUP_AFFINITY,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetThreadGroupAffinity(
            hthread.into_param().abi(),
            ::std::mem::transmute(groupaffinity),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetThreadIOPendingFlag<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hthread: Param0,
    lpioispending: *mut super::super::Foundation::BOOL,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThreadIOPendingFlag(
                hthread: super::super::Foundation::HANDLE,
                lpioispending: *mut super::super::Foundation::BOOL,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetThreadIOPendingFlag(
            hthread.into_param().abi(),
            ::std::mem::transmute(lpioispending),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetThreadId<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    thread: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThreadId(thread: super::super::Foundation::HANDLE) -> u32;
        }
        ::std::mem::transmute(GetThreadId(thread.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub unsafe fn GetThreadIdealProcessorEx<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hthread: Param0,
    lpidealprocessor: *mut super::Kernel::PROCESSOR_NUMBER,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThreadIdealProcessorEx(
                hthread: super::super::Foundation::HANDLE,
                lpidealprocessor: *mut super::Kernel::PROCESSOR_NUMBER,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetThreadIdealProcessorEx(
            hthread.into_param().abi(),
            ::std::mem::transmute(lpidealprocessor),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetThreadInformation<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hthread: Param0,
    threadinformationclass: THREAD_INFORMATION_CLASS,
    threadinformation: *mut ::std::ffi::c_void,
    threadinformationsize: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThreadInformation(
                hthread: super::super::Foundation::HANDLE,
                threadinformationclass: THREAD_INFORMATION_CLASS,
                threadinformation: *mut ::std::ffi::c_void,
                threadinformationsize: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetThreadInformation(
            hthread.into_param().abi(),
            ::std::mem::transmute(threadinformationclass),
            ::std::mem::transmute(threadinformation),
            ::std::mem::transmute(threadinformationsize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetThreadPriority<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hthread: Param0,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThreadPriority(hthread: super::super::Foundation::HANDLE) -> i32;
        }
        ::std::mem::transmute(GetThreadPriority(hthread.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetThreadPriorityBoost<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hthread: Param0,
    pdisablepriorityboost: *mut super::super::Foundation::BOOL,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThreadPriorityBoost(
                hthread: super::super::Foundation::HANDLE,
                pdisablepriorityboost: *mut super::super::Foundation::BOOL,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetThreadPriorityBoost(
            hthread.into_param().abi(),
            ::std::mem::transmute(pdisablepriorityboost),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub unsafe fn GetThreadSelectedCpuSetMasks<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    thread: Param0,
    cpusetmasks: *mut super::Kernel::GROUP_AFFINITY,
    cpusetmaskcount: u16,
    requiredmaskcount: *mut u16,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThreadSelectedCpuSetMasks(
                thread: super::super::Foundation::HANDLE,
                cpusetmasks: *mut super::Kernel::GROUP_AFFINITY,
                cpusetmaskcount: u16,
                requiredmaskcount: *mut u16,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetThreadSelectedCpuSetMasks(
            thread.into_param().abi(),
            ::std::mem::transmute(cpusetmasks),
            ::std::mem::transmute(cpusetmaskcount),
            ::std::mem::transmute(requiredmaskcount),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetThreadSelectedCpuSets<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    thread: Param0,
    cpusetids: *mut u32,
    cpusetidcount: u32,
    requiredidcount: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThreadSelectedCpuSets(
                thread: super::super::Foundation::HANDLE,
                cpusetids: *mut u32,
                cpusetidcount: u32,
                requiredidcount: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetThreadSelectedCpuSets(
            thread.into_param().abi(),
            ::std::mem::transmute(cpusetids),
            ::std::mem::transmute(cpusetidcount),
            ::std::mem::transmute(requiredidcount),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetThreadTimes<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hthread: Param0,
    lpcreationtime: *mut super::super::Foundation::FILETIME,
    lpexittime: *mut super::super::Foundation::FILETIME,
    lpkerneltime: *mut super::super::Foundation::FILETIME,
    lpusertime: *mut super::super::Foundation::FILETIME,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThreadTimes(
                hthread: super::super::Foundation::HANDLE,
                lpcreationtime: *mut super::super::Foundation::FILETIME,
                lpexittime: *mut super::super::Foundation::FILETIME,
                lpkerneltime: *mut super::super::Foundation::FILETIME,
                lpusertime: *mut super::super::Foundation::FILETIME,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetThreadTimes(
            hthread.into_param().abi(),
            ::std::mem::transmute(lpcreationtime),
            ::std::mem::transmute(lpexittime),
            ::std::mem::transmute(lpkerneltime),
            ::std::mem::transmute(lpusertime),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetUmsCompletionListEvent(
    umscompletionlist: *const ::std::ffi::c_void,
    umscompletionevent: *mut super::super::Foundation::HANDLE,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetUmsCompletionListEvent(
                umscompletionlist: *const ::std::ffi::c_void,
                umscompletionevent: *mut super::super::Foundation::HANDLE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetUmsCompletionListEvent(
            ::std::mem::transmute(umscompletionlist),
            ::std::mem::transmute(umscompletionevent),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetUmsSystemThreadInformation<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    threadhandle: Param0,
    systemthreadinfo: *mut UMS_SYSTEM_THREAD_INFORMATION,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetUmsSystemThreadInformation(
                threadhandle: super::super::Foundation::HANDLE,
                systemthreadinfo: *mut UMS_SYSTEM_THREAD_INFORMATION,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetUmsSystemThreadInformation(
            threadhandle.into_param().abi(),
            ::std::mem::transmute(systemthreadinfo),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const INIT_ONCE_ASYNC: u32 = 2u32;
pub const INIT_ONCE_CHECK_ONLY: u32 = 1u32;
pub const INIT_ONCE_CTX_RESERVED_BITS: u32 = 2u32;
pub const INIT_ONCE_INIT_FAILED: u32 = 4u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn InitOnceBeginInitialize(
    lpinitonce: *mut super::SystemServices::RTL_RUN_ONCE,
    dwflags: u32,
    fpending: *mut super::super::Foundation::BOOL,
    lpcontext: *mut *mut ::std::ffi::c_void,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitOnceBeginInitialize(
                lpinitonce: *mut super::SystemServices::RTL_RUN_ONCE,
                dwflags: u32,
                fpending: *mut super::super::Foundation::BOOL,
                lpcontext: *mut *mut ::std::ffi::c_void,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(InitOnceBeginInitialize(
            ::std::mem::transmute(lpinitonce),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(fpending),
            ::std::mem::transmute(lpcontext),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn InitOnceComplete(
    lpinitonce: *mut super::SystemServices::RTL_RUN_ONCE,
    dwflags: u32,
    lpcontext: *const ::std::ffi::c_void,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitOnceComplete(
                lpinitonce: *mut super::SystemServices::RTL_RUN_ONCE,
                dwflags: u32,
                lpcontext: *const ::std::ffi::c_void,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(InitOnceComplete(
            ::std::mem::transmute(lpinitonce),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(lpcontext),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn InitOnceExecuteOnce(
    initonce: *mut super::SystemServices::RTL_RUN_ONCE,
    initfn: ::std::option::Option<PINIT_ONCE_FN>,
    parameter: *mut ::std::ffi::c_void,
    context: *mut *mut ::std::ffi::c_void,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitOnceExecuteOnce(
                initonce: *mut super::SystemServices::RTL_RUN_ONCE,
                initfn: ::windows::runtime::RawPtr,
                parameter: *mut ::std::ffi::c_void,
                context: *mut *mut ::std::ffi::c_void,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(InitOnceExecuteOnce(
            ::std::mem::transmute(initonce),
            ::std::mem::transmute(initfn),
            ::std::mem::transmute(parameter),
            ::std::mem::transmute(context),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn InitOnceInitialize(initonce: *mut super::SystemServices::RTL_RUN_ONCE) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitOnceInitialize(initonce: *mut super::SystemServices::RTL_RUN_ONCE);
        }
        ::std::mem::transmute(InitOnceInitialize(::std::mem::transmute(initonce)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn InitializeConditionVariable(
    conditionvariable: *mut super::SystemServices::RTL_CONDITION_VARIABLE,
) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitializeConditionVariable(
                conditionvariable: *mut super::SystemServices::RTL_CONDITION_VARIABLE,
            );
        }
        ::std::mem::transmute(InitializeConditionVariable(::std::mem::transmute(
            conditionvariable,
        )))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Kernel",
    feature = "Win32_System_SystemServices"
))]
pub unsafe fn InitializeCriticalSection(
    lpcriticalsection: *mut super::SystemServices::RTL_CRITICAL_SECTION,
) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitializeCriticalSection(
                lpcriticalsection: *mut super::SystemServices::RTL_CRITICAL_SECTION,
            );
        }
        ::std::mem::transmute(InitializeCriticalSection(::std::mem::transmute(
            lpcriticalsection,
        )))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Kernel",
    feature = "Win32_System_SystemServices"
))]
pub unsafe fn InitializeCriticalSectionAndSpinCount(
    lpcriticalsection: *mut super::SystemServices::RTL_CRITICAL_SECTION,
    dwspincount: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitializeCriticalSectionAndSpinCount(
                lpcriticalsection: *mut super::SystemServices::RTL_CRITICAL_SECTION,
                dwspincount: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(InitializeCriticalSectionAndSpinCount(
            ::std::mem::transmute(lpcriticalsection),
            ::std::mem::transmute(dwspincount),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Kernel",
    feature = "Win32_System_SystemServices"
))]
pub unsafe fn InitializeCriticalSectionEx(
    lpcriticalsection: *mut super::SystemServices::RTL_CRITICAL_SECTION,
    dwspincount: u32,
    flags: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitializeCriticalSectionEx(
                lpcriticalsection: *mut super::SystemServices::RTL_CRITICAL_SECTION,
                dwspincount: u32,
                flags: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(InitializeCriticalSectionEx(
            ::std::mem::transmute(lpcriticalsection),
            ::std::mem::transmute(dwspincount),
            ::std::mem::transmute(flags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn InitializeProcThreadAttributeList(
    lpattributelist: LPPROC_THREAD_ATTRIBUTE_LIST,
    dwattributecount: u32,
    dwflags: u32,
    lpsize: *mut usize,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitializeProcThreadAttributeList(
                lpattributelist: LPPROC_THREAD_ATTRIBUTE_LIST,
                dwattributecount: u32,
                dwflags: u32,
                lpsize: *mut usize,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(InitializeProcThreadAttributeList(
            ::std::mem::transmute(lpattributelist),
            ::std::mem::transmute(dwattributecount),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(lpsize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Kernel")]
pub unsafe fn InitializeSListHead(listhead: *mut super::Kernel::SLIST_HEADER) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitializeSListHead(listhead: *mut super::Kernel::SLIST_HEADER);
        }
        ::std::mem::transmute(InitializeSListHead(::std::mem::transmute(listhead)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn InitializeSRWLock(srwlock: *mut super::SystemServices::RTL_SRWLOCK) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitializeSRWLock(srwlock: *mut super::SystemServices::RTL_SRWLOCK);
        }
        ::std::mem::transmute(InitializeSRWLock(::std::mem::transmute(srwlock)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn InitializeSynchronizationBarrier(
    lpbarrier: *mut super::SystemServices::RTL_BARRIER,
    ltotalthreads: i32,
    lspincount: i32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitializeSynchronizationBarrier(
                lpbarrier: *mut super::SystemServices::RTL_BARRIER,
                ltotalthreads: i32,
                lspincount: i32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(InitializeSynchronizationBarrier(
            ::std::mem::transmute(lpbarrier),
            ::std::mem::transmute(ltotalthreads),
            ::std::mem::transmute(lspincount),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Kernel")]
pub unsafe fn InterlockedFlushSList(
    listhead: *mut super::Kernel::SLIST_HEADER,
) -> *mut super::Kernel::SLIST_ENTRY {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InterlockedFlushSList(
                listhead: *mut super::Kernel::SLIST_HEADER,
            ) -> *mut super::Kernel::SLIST_ENTRY;
        }
        ::std::mem::transmute(InterlockedFlushSList(::std::mem::transmute(listhead)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Kernel")]
pub unsafe fn InterlockedPopEntrySList(
    listhead: *mut super::Kernel::SLIST_HEADER,
) -> *mut super::Kernel::SLIST_ENTRY {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InterlockedPopEntrySList(
                listhead: *mut super::Kernel::SLIST_HEADER,
            ) -> *mut super::Kernel::SLIST_ENTRY;
        }
        ::std::mem::transmute(InterlockedPopEntrySList(::std::mem::transmute(listhead)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Kernel")]
pub unsafe fn InterlockedPushEntrySList(
    listhead: *mut super::Kernel::SLIST_HEADER,
    listentry: *mut super::Kernel::SLIST_ENTRY,
) -> *mut super::Kernel::SLIST_ENTRY {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InterlockedPushEntrySList(
                listhead: *mut super::Kernel::SLIST_HEADER,
                listentry: *mut super::Kernel::SLIST_ENTRY,
            ) -> *mut super::Kernel::SLIST_ENTRY;
        }
        ::std::mem::transmute(InterlockedPushEntrySList(
            ::std::mem::transmute(listhead),
            ::std::mem::transmute(listentry),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Kernel")]
pub unsafe fn InterlockedPushListSListEx(
    listhead: *mut super::Kernel::SLIST_HEADER,
    list: *mut super::Kernel::SLIST_ENTRY,
    listend: *mut super::Kernel::SLIST_ENTRY,
    count: u32,
) -> *mut super::Kernel::SLIST_ENTRY {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InterlockedPushListSListEx(
                listhead: *mut super::Kernel::SLIST_HEADER,
                list: *mut super::Kernel::SLIST_ENTRY,
                listend: *mut super::Kernel::SLIST_ENTRY,
                count: u32,
            ) -> *mut super::Kernel::SLIST_ENTRY;
        }
        ::std::mem::transmute(InterlockedPushListSListEx(
            ::std::mem::transmute(listhead),
            ::std::mem::transmute(list),
            ::std::mem::transmute(listend),
            ::std::mem::transmute(count),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn IsImmersiveProcess<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hprocess: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsImmersiveProcess(
                hprocess: super::super::Foundation::HANDLE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(IsImmersiveProcess(hprocess.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn IsProcessCritical<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hprocess: Param0,
    critical: *mut super::super::Foundation::BOOL,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsProcessCritical(
                hprocess: super::super::Foundation::HANDLE,
                critical: *mut super::super::Foundation::BOOL,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(IsProcessCritical(
            hprocess.into_param().abi(),
            ::std::mem::transmute(critical),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn IsProcessorFeaturePresent(
    processorfeature: PROCESSOR_FEATURE_ID,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsProcessorFeaturePresent(
                processorfeature: PROCESSOR_FEATURE_ID,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(IsProcessorFeaturePresent(::std::mem::transmute(
            processorfeature,
        )))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn IsThreadAFiber() -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsThreadAFiber() -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(IsThreadAFiber())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn IsThreadpoolTimerSet(
    pti: *mut super::SystemServices::TP_TIMER,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsThreadpoolTimerSet(
                pti: *mut super::SystemServices::TP_TIMER,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(IsThreadpoolTimerSet(::std::mem::transmute(pti)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn IsWow64Process<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hprocess: Param0,
    wow64process: *mut super::super::Foundation::BOOL,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsWow64Process(
                hprocess: super::super::Foundation::HANDLE,
                wow64process: *mut super::super::Foundation::BOOL,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(IsWow64Process(
            hprocess.into_param().abi(),
            ::std::mem::transmute(wow64process),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn IsWow64Process2<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hprocess: Param0,
    pprocessmachine: *mut u16,
    pnativemachine: *mut u16,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsWow64Process2(
                hprocess: super::super::Foundation::HANDLE,
                pprocessmachine: *mut u16,
                pnativemachine: *mut u16,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(IsWow64Process2(
            hprocess.into_param().abi(),
            ::std::mem::transmute(pprocessmachine),
            ::std::mem::transmute(pnativemachine),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(
    :: std :: clone :: Clone,
    :: std :: marker :: Copy,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
)]
#[repr(transparent)]
pub struct LPPROC_THREAD_ATTRIBUTE_LIST(pub *mut ::std::ffi::c_void);
impl ::std::default::Default for LPPROC_THREAD_ATTRIBUTE_LIST {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for LPPROC_THREAD_ATTRIBUTE_LIST {}
unsafe impl ::windows::runtime::Abi for LPPROC_THREAD_ATTRIBUTE_LIST {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Kernel",
    feature = "Win32_System_SystemServices"
))]
pub unsafe fn LeaveCriticalSection(
    lpcriticalsection: *mut super::SystemServices::RTL_CRITICAL_SECTION,
) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LeaveCriticalSection(
                lpcriticalsection: *mut super::SystemServices::RTL_CRITICAL_SECTION,
            );
        }
        ::std::mem::transmute(LeaveCriticalSection(::std::mem::transmute(
            lpcriticalsection,
        )))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Kernel",
    feature = "Win32_System_SystemServices"
))]
pub unsafe fn LeaveCriticalSectionWhenCallbackReturns(
    pci: *mut super::SystemServices::TP_CALLBACK_INSTANCE,
    pcs: *mut super::SystemServices::RTL_CRITICAL_SECTION,
) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LeaveCriticalSectionWhenCallbackReturns(
                pci: *mut super::SystemServices::TP_CALLBACK_INSTANCE,
                pcs: *mut super::SystemServices::RTL_CRITICAL_SECTION,
            );
        }
        ::std::mem::transmute(LeaveCriticalSectionWhenCallbackReturns(
            ::std::mem::transmute(pci),
            ::std::mem::transmute(pcs),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct MACHINE_ATTRIBUTES(pub u32);
pub const UserEnabled: MACHINE_ATTRIBUTES = MACHINE_ATTRIBUTES(1u32);
pub const KernelEnabled: MACHINE_ATTRIBUTES = MACHINE_ATTRIBUTES(2u32);
pub const Wow64Container: MACHINE_ATTRIBUTES = MACHINE_ATTRIBUTES(4u32);
impl ::std::convert::From<u32> for MACHINE_ATTRIBUTES {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MACHINE_ATTRIBUTES {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for MACHINE_ATTRIBUTES {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for MACHINE_ATTRIBUTES {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for MACHINE_ATTRIBUTES {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for MACHINE_ATTRIBUTES {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for MACHINE_ATTRIBUTES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct MEMORY_PRIORITY(pub u32);
pub const MEMORY_PRIORITY_VERY_LOW: MEMORY_PRIORITY = MEMORY_PRIORITY(1u32);
pub const MEMORY_PRIORITY_LOW: MEMORY_PRIORITY = MEMORY_PRIORITY(2u32);
pub const MEMORY_PRIORITY_MEDIUM: MEMORY_PRIORITY = MEMORY_PRIORITY(3u32);
pub const MEMORY_PRIORITY_BELOW_NORMAL: MEMORY_PRIORITY = MEMORY_PRIORITY(4u32);
pub const MEMORY_PRIORITY_NORMAL: MEMORY_PRIORITY = MEMORY_PRIORITY(5u32);
impl ::std::convert::From<u32> for MEMORY_PRIORITY {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MEMORY_PRIORITY {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for MEMORY_PRIORITY {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for MEMORY_PRIORITY {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for MEMORY_PRIORITY {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for MEMORY_PRIORITY {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for MEMORY_PRIORITY {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct MEMORY_PRIORITY_INFORMATION {
    pub MemoryPriority: MEMORY_PRIORITY,
}
impl MEMORY_PRIORITY_INFORMATION {}
impl ::std::default::Default for MEMORY_PRIORITY_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MEMORY_PRIORITY_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MEMORY_PRIORITY_INFORMATION")
            .field("MemoryPriority", &self.MemoryPriority)
            .finish()
    }
}
impl ::std::cmp::PartialEq for MEMORY_PRIORITY_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.MemoryPriority == other.MemoryPriority
    }
}
impl ::std::cmp::Eq for MEMORY_PRIORITY_INFORMATION {}
unsafe impl ::windows::runtime::Abi for MEMORY_PRIORITY_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS(pub u32);
pub const MWMO_NONE: MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS =
    MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS(0u32);
pub const MWMO_ALERTABLE: MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS =
    MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS(2u32);
pub const MWMO_INPUTAVAILABLE: MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS =
    MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS(4u32);
pub const MWMO_WAITALL: MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS =
    MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS(1u32);
impl ::std::convert::From<u32> for MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const MUTEX_MODIFY_STATE: u32 = 1u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub unsafe fn MsgWaitForMultipleObjects<
    'a,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    ncount: u32,
    phandles: *const super::super::Foundation::HANDLE,
    fwaitall: Param2,
    dwmilliseconds: u32,
    dwwakemask: super::super::UI::WindowsAndMessaging::QUEUE_STATUS_FLAGS,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsgWaitForMultipleObjects(
                ncount: u32,
                phandles: *const super::super::Foundation::HANDLE,
                fwaitall: super::super::Foundation::BOOL,
                dwmilliseconds: u32,
                dwwakemask: super::super::UI::WindowsAndMessaging::QUEUE_STATUS_FLAGS,
            ) -> u32;
        }
        ::std::mem::transmute(MsgWaitForMultipleObjects(
            ::std::mem::transmute(ncount),
            ::std::mem::transmute(phandles),
            fwaitall.into_param().abi(),
            ::std::mem::transmute(dwmilliseconds),
            ::std::mem::transmute(dwwakemask),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub unsafe fn MsgWaitForMultipleObjectsEx(
    ncount: u32,
    phandles: *const super::super::Foundation::HANDLE,
    dwmilliseconds: u32,
    dwwakemask: super::super::UI::WindowsAndMessaging::QUEUE_STATUS_FLAGS,
    dwflags: MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsgWaitForMultipleObjectsEx(
                ncount: u32,
                phandles: *const super::super::Foundation::HANDLE,
                dwmilliseconds: u32,
                dwwakemask: super::super::UI::WindowsAndMessaging::QUEUE_STATUS_FLAGS,
                dwflags: MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS,
            ) -> u32;
        }
        ::std::mem::transmute(MsgWaitForMultipleObjectsEx(
            ::std::mem::transmute(ncount),
            ::std::mem::transmute(phandles),
            ::std::mem::transmute(dwmilliseconds),
            ::std::mem::transmute(dwwakemask),
            ::std::mem::transmute(dwflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(
    :: std :: clone :: Clone,
    :: std :: marker :: Copy,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
)]
#[repr(transparent)]
pub struct NamespaceHandle(pub isize);
impl ::std::default::Default for NamespaceHandle {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for NamespaceHandle {}
unsafe impl ::windows::runtime::Abi for NamespaceHandle {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn NtQueryInformationProcess<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    processhandle: Param0,
    processinformationclass: PROCESSINFOCLASS,
    processinformation: *mut ::std::ffi::c_void,
    processinformationlength: u32,
    returnlength: *mut u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NtQueryInformationProcess(
                processhandle: super::super::Foundation::HANDLE,
                processinformationclass: PROCESSINFOCLASS,
                processinformation: *mut ::std::ffi::c_void,
                processinformationlength: u32,
                returnlength: *mut u32,
            ) -> super::super::Foundation::NTSTATUS;
        }
        NtQueryInformationProcess(
            processhandle.into_param().abi(),
            ::std::mem::transmute(processinformationclass),
            ::std::mem::transmute(processinformation),
            ::std::mem::transmute(processinformationlength),
            ::std::mem::transmute(returnlength),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn NtQueryInformationThread<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    threadhandle: Param0,
    threadinformationclass: THREADINFOCLASS,
    threadinformation: *mut ::std::ffi::c_void,
    threadinformationlength: u32,
    returnlength: *mut u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NtQueryInformationThread(
                threadhandle: super::super::Foundation::HANDLE,
                threadinformationclass: THREADINFOCLASS,
                threadinformation: *mut ::std::ffi::c_void,
                threadinformationlength: u32,
                returnlength: *mut u32,
            ) -> super::super::Foundation::NTSTATUS;
        }
        NtQueryInformationThread(
            threadhandle.into_param().abi(),
            ::std::mem::transmute(threadinformationclass),
            ::std::mem::transmute(threadinformation),
            ::std::mem::transmute(threadinformationlength),
            ::std::mem::transmute(returnlength),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn OpenEventA<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    dwdesiredaccess: u32,
    binherithandle: Param1,
    lpname: Param2,
) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenEventA(
                dwdesiredaccess: u32,
                binherithandle: super::super::Foundation::BOOL,
                lpname: super::super::Foundation::PSTR,
            ) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(OpenEventA(
            ::std::mem::transmute(dwdesiredaccess),
            binherithandle.into_param().abi(),
            lpname.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn OpenEventW<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    dwdesiredaccess: u32,
    binherithandle: Param1,
    lpname: Param2,
) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenEventW(
                dwdesiredaccess: u32,
                binherithandle: super::super::Foundation::BOOL,
                lpname: super::super::Foundation::PWSTR,
            ) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(OpenEventW(
            ::std::mem::transmute(dwdesiredaccess),
            binherithandle.into_param().abi(),
            lpname.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn OpenMutexW<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    dwdesiredaccess: u32,
    binherithandle: Param1,
    lpname: Param2,
) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenMutexW(
                dwdesiredaccess: u32,
                binherithandle: super::super::Foundation::BOOL,
                lpname: super::super::Foundation::PWSTR,
            ) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(OpenMutexW(
            ::std::mem::transmute(dwdesiredaccess),
            binherithandle.into_param().abi(),
            lpname.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn OpenPrivateNamespaceA<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lpboundarydescriptor: *const ::std::ffi::c_void,
    lpaliasprefix: Param1,
) -> NamespaceHandle {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenPrivateNamespaceA(
                lpboundarydescriptor: *const ::std::ffi::c_void,
                lpaliasprefix: super::super::Foundation::PSTR,
            ) -> NamespaceHandle;
        }
        ::std::mem::transmute(OpenPrivateNamespaceA(
            ::std::mem::transmute(lpboundarydescriptor),
            lpaliasprefix.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn OpenPrivateNamespaceW<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpboundarydescriptor: *const ::std::ffi::c_void,
    lpaliasprefix: Param1,
) -> NamespaceHandle {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenPrivateNamespaceW(
                lpboundarydescriptor: *const ::std::ffi::c_void,
                lpaliasprefix: super::super::Foundation::PWSTR,
            ) -> NamespaceHandle;
        }
        ::std::mem::transmute(OpenPrivateNamespaceW(
            ::std::mem::transmute(lpboundarydescriptor),
            lpaliasprefix.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn OpenProcess<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    dwdesiredaccess: PROCESS_ACCESS_RIGHTS,
    binherithandle: Param1,
    dwprocessid: u32,
) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenProcess(
                dwdesiredaccess: PROCESS_ACCESS_RIGHTS,
                binherithandle: super::super::Foundation::BOOL,
                dwprocessid: u32,
            ) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(OpenProcess(
            ::std::mem::transmute(dwdesiredaccess),
            binherithandle.into_param().abi(),
            ::std::mem::transmute(dwprocessid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub unsafe fn OpenProcessToken<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    processhandle: Param0,
    desiredaccess: super::super::Security::TOKEN_ACCESS_MASK,
    tokenhandle: *mut super::super::Foundation::HANDLE,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenProcessToken(
                processhandle: super::super::Foundation::HANDLE,
                desiredaccess: super::super::Security::TOKEN_ACCESS_MASK,
                tokenhandle: *mut super::super::Foundation::HANDLE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(OpenProcessToken(
            processhandle.into_param().abi(),
            ::std::mem::transmute(desiredaccess),
            ::std::mem::transmute(tokenhandle),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn OpenSemaphoreW<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    dwdesiredaccess: u32,
    binherithandle: Param1,
    lpname: Param2,
) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenSemaphoreW(
                dwdesiredaccess: u32,
                binherithandle: super::super::Foundation::BOOL,
                lpname: super::super::Foundation::PWSTR,
            ) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(OpenSemaphoreW(
            ::std::mem::transmute(dwdesiredaccess),
            binherithandle.into_param().abi(),
            lpname.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn OpenThread<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    dwdesiredaccess: THREAD_ACCESS_RIGHTS,
    binherithandle: Param1,
    dwthreadid: u32,
) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenThread(
                dwdesiredaccess: THREAD_ACCESS_RIGHTS,
                binherithandle: super::super::Foundation::BOOL,
                dwthreadid: u32,
            ) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(OpenThread(
            ::std::mem::transmute(dwdesiredaccess),
            binherithandle.into_param().abi(),
            ::std::mem::transmute(dwthreadid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub unsafe fn OpenThreadToken<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    threadhandle: Param0,
    desiredaccess: super::super::Security::TOKEN_ACCESS_MASK,
    openasself: Param2,
    tokenhandle: *mut super::super::Foundation::HANDLE,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenThreadToken(
                threadhandle: super::super::Foundation::HANDLE,
                desiredaccess: super::super::Security::TOKEN_ACCESS_MASK,
                openasself: super::super::Foundation::BOOL,
                tokenhandle: *mut super::super::Foundation::HANDLE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(OpenThreadToken(
            threadhandle.into_param().abi(),
            ::std::mem::transmute(desiredaccess),
            openasself.into_param().abi(),
            ::std::mem::transmute(tokenhandle),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn OpenWaitableTimerW<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    dwdesiredaccess: u32,
    binherithandle: Param1,
    lptimername: Param2,
) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenWaitableTimerW(
                dwdesiredaccess: u32,
                binherithandle: super::super::Foundation::BOOL,
                lptimername: super::super::Foundation::PWSTR,
            ) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(OpenWaitableTimerW(
            ::std::mem::transmute(dwdesiredaccess),
            binherithandle.into_param().abi(),
            lptimername.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Kernel",
    feature = "Win32_System_WindowsProgramming"
))]
pub struct PEB {
    pub Reserved1: [u8; 2],
    pub BeingDebugged: u8,
    pub Reserved2: [u8; 1],
    pub Reserved3: [*mut ::std::ffi::c_void; 2],
    pub Ldr: *mut PEB_LDR_DATA,
    pub ProcessParameters: *mut RTL_USER_PROCESS_PARAMETERS,
    pub Reserved4: [*mut ::std::ffi::c_void; 3],
    pub AtlThunkSListPtr: *mut ::std::ffi::c_void,
    pub Reserved5: *mut ::std::ffi::c_void,
    pub Reserved6: u32,
    pub Reserved7: *mut ::std::ffi::c_void,
    pub Reserved8: u32,
    pub AtlThunkSListPtr32: u32,
    pub Reserved9: [*mut ::std::ffi::c_void; 45],
    pub Reserved10: [u8; 96],
    pub PostProcessInitRoutine:
        ::std::option::Option<super::WindowsProgramming::PPS_POST_PROCESS_INIT_ROUTINE>,
    pub Reserved11: [u8; 128],
    pub Reserved12: [*mut ::std::ffi::c_void; 1],
    pub SessionId: u32,
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Kernel",
    feature = "Win32_System_WindowsProgramming"
))]
impl PEB {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Kernel",
    feature = "Win32_System_WindowsProgramming"
))]
impl ::std::default::Default for PEB {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Kernel",
    feature = "Win32_System_WindowsProgramming"
))]
impl ::std::fmt::Debug for PEB {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PEB")
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
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Kernel",
    feature = "Win32_System_WindowsProgramming"
))]
impl ::std::cmp::PartialEq for PEB {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved1 == other.Reserved1
            && self.BeingDebugged == other.BeingDebugged
            && self.Reserved2 == other.Reserved2
            && self.Reserved3 == other.Reserved3
            && self.Ldr == other.Ldr
            && self.ProcessParameters == other.ProcessParameters
            && self.Reserved4 == other.Reserved4
            && self.AtlThunkSListPtr == other.AtlThunkSListPtr
            && self.Reserved5 == other.Reserved5
            && self.Reserved6 == other.Reserved6
            && self.Reserved7 == other.Reserved7
            && self.Reserved8 == other.Reserved8
            && self.AtlThunkSListPtr32 == other.AtlThunkSListPtr32
            && self.Reserved9 == other.Reserved9
            && self.Reserved10 == other.Reserved10
            && self.PostProcessInitRoutine.map(|f| f as usize)
                == other.PostProcessInitRoutine.map(|f| f as usize)
            && self.Reserved11 == other.Reserved11
            && self.Reserved12 == other.Reserved12
            && self.SessionId == other.SessionId
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Kernel",
    feature = "Win32_System_WindowsProgramming"
))]
impl ::std::cmp::Eq for PEB {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Kernel",
    feature = "Win32_System_WindowsProgramming"
))]
unsafe impl ::windows::runtime::Abi for PEB {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
pub struct PEB_LDR_DATA {
    pub Reserved1: [u8; 8],
    pub Reserved2: [*mut ::std::ffi::c_void; 3],
    pub InMemoryOrderModuleList: super::Kernel::LIST_ENTRY,
}
#[cfg(feature = "Win32_System_Kernel")]
impl PEB_LDR_DATA {}
#[cfg(feature = "Win32_System_Kernel")]
impl ::std::default::Default for PEB_LDR_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl ::std::fmt::Debug for PEB_LDR_DATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PEB_LDR_DATA")
            .field("Reserved1", &self.Reserved1)
            .field("Reserved2", &self.Reserved2)
            .field("InMemoryOrderModuleList", &self.InMemoryOrderModuleList)
            .finish()
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl ::std::cmp::PartialEq for PEB_LDR_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved1 == other.Reserved1
            && self.Reserved2 == other.Reserved2
            && self.InMemoryOrderModuleList == other.InMemoryOrderModuleList
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl ::std::cmp::Eq for PEB_LDR_DATA {}
#[cfg(feature = "Win32_System_Kernel")]
unsafe impl ::windows::runtime::Abi for PEB_LDR_DATA {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub type PINIT_ONCE_FN = unsafe extern "system" fn(
    initonce: *mut super::SystemServices::RTL_RUN_ONCE,
    parameter: *mut ::std::ffi::c_void,
    context: *mut *mut ::std::ffi::c_void,
) -> super::super::Foundation::BOOL;
pub const PME_CURRENT_VERSION: u32 = 1u32;
pub const PME_FAILFAST_ON_COMMIT_FAIL_DISABLE: u32 = 0u32;
pub const PME_FAILFAST_ON_COMMIT_FAIL_ENABLE: u32 = 1u32;
pub const PRIVATE_NAMESPACE_FLAG_DESTROY: u32 = 1u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct PROCESSINFOCLASS(pub i32);
pub const ProcessBasicInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(0i32);
pub const ProcessDebugPort: PROCESSINFOCLASS = PROCESSINFOCLASS(7i32);
pub const ProcessWow64Information: PROCESSINFOCLASS = PROCESSINFOCLASS(26i32);
pub const ProcessImageFileName: PROCESSINFOCLASS = PROCESSINFOCLASS(27i32);
pub const ProcessBreakOnTermination: PROCESSINFOCLASS = PROCESSINFOCLASS(29i32);
impl ::std::convert::From<i32> for PROCESSINFOCLASS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PROCESSINFOCLASS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct PROCESSOR_FEATURE_ID(pub u32);
pub const PF_ARM_64BIT_LOADSTORE_ATOMIC: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(25u32);
pub const PF_ARM_DIVIDE_INSTRUCTION_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(24u32);
pub const PF_ARM_EXTERNAL_CACHE_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(26u32);
pub const PF_ARM_FMAC_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(27u32);
pub const PF_ARM_VFP_32_REGISTERS_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(18u32);
pub const PF_3DNOW_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(7u32);
pub const PF_CHANNELS_ENABLED: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(16u32);
pub const PF_COMPARE_EXCHANGE_DOUBLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(2u32);
pub const PF_COMPARE_EXCHANGE128: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(14u32);
pub const PF_COMPARE64_EXCHANGE128: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(15u32);
pub const PF_FASTFAIL_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(23u32);
pub const PF_FLOATING_POINT_EMULATED: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(1u32);
pub const PF_FLOATING_POINT_PRECISION_ERRATA: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(0u32);
pub const PF_MMX_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(3u32);
pub const PF_NX_ENABLED: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(12u32);
pub const PF_PAE_ENABLED: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(9u32);
pub const PF_RDTSC_INSTRUCTION_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(8u32);
pub const PF_RDWRFSGSBASE_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(22u32);
pub const PF_SECOND_LEVEL_ADDRESS_TRANSLATION: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(20u32);
pub const PF_SSE3_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(13u32);
pub const PF_VIRT_FIRMWARE_ENABLED: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(21u32);
pub const PF_XMMI_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(6u32);
pub const PF_XMMI64_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(10u32);
pub const PF_XSAVE_ENABLED: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(17u32);
pub const PF_ARM_V8_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(29u32);
pub const PF_ARM_V8_CRYPTO_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID =
    PROCESSOR_FEATURE_ID(30u32);
pub const PF_ARM_V8_CRC32_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID =
    PROCESSOR_FEATURE_ID(31u32);
pub const PF_ARM_V81_ATOMIC_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID =
    PROCESSOR_FEATURE_ID(34u32);
impl ::std::convert::From<u32> for PROCESSOR_FEATURE_ID {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PROCESSOR_FEATURE_ID {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for PROCESSOR_FEATURE_ID {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for PROCESSOR_FEATURE_ID {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for PROCESSOR_FEATURE_ID {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for PROCESSOR_FEATURE_ID {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for PROCESSOR_FEATURE_ID {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct PROCESS_ACCESS_RIGHTS(pub u32);
pub const PROCESS_TERMINATE: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(1u32);
pub const PROCESS_CREATE_THREAD: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(2u32);
pub const PROCESS_SET_SESSIONID: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(4u32);
pub const PROCESS_VM_OPERATION: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(8u32);
pub const PROCESS_VM_READ: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(16u32);
pub const PROCESS_VM_WRITE: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(32u32);
pub const PROCESS_DUP_HANDLE: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(64u32);
pub const PROCESS_CREATE_PROCESS: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(128u32);
pub const PROCESS_SET_QUOTA: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(256u32);
pub const PROCESS_SET_INFORMATION: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(512u32);
pub const PROCESS_QUERY_INFORMATION: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(1024u32);
pub const PROCESS_SUSPEND_RESUME: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(2048u32);
pub const PROCESS_QUERY_LIMITED_INFORMATION: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(4096u32);
pub const PROCESS_SET_LIMITED_INFORMATION: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(8192u32);
pub const PROCESS_ALL_ACCESS: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(2097151u32);
pub const PROCESS_DELETE: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(65536u32);
pub const PROCESS_READ_CONTROL: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(131072u32);
pub const PROCESS_WRITE_DAC: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(262144u32);
pub const PROCESS_WRITE_OWNER: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(524288u32);
pub const PROCESS_SYNCHRONIZE: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(1048576u32);
pub const PROCESS_STANDARD_RIGHTS_REQUIRED: PROCESS_ACCESS_RIGHTS =
    PROCESS_ACCESS_RIGHTS(983040u32);
impl ::std::convert::From<u32> for PROCESS_ACCESS_RIGHTS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PROCESS_ACCESS_RIGHTS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for PROCESS_ACCESS_RIGHTS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for PROCESS_ACCESS_RIGHTS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for PROCESS_ACCESS_RIGHTS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for PROCESS_ACCESS_RIGHTS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for PROCESS_ACCESS_RIGHTS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct PROCESS_AFFINITY_AUTO_UPDATE_FLAGS(pub u32);
pub const PROCESS_AFFINITY_DISABLE_AUTO_UPDATE: PROCESS_AFFINITY_AUTO_UPDATE_FLAGS =
    PROCESS_AFFINITY_AUTO_UPDATE_FLAGS(0u32);
pub const PROCESS_AFFINITY_ENABLE_AUTO_UPDATE: PROCESS_AFFINITY_AUTO_UPDATE_FLAGS =
    PROCESS_AFFINITY_AUTO_UPDATE_FLAGS(1u32);
impl ::std::convert::From<u32> for PROCESS_AFFINITY_AUTO_UPDATE_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PROCESS_AFFINITY_AUTO_UPDATE_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for PROCESS_AFFINITY_AUTO_UPDATE_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for PROCESS_AFFINITY_AUTO_UPDATE_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for PROCESS_AFFINITY_AUTO_UPDATE_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for PROCESS_AFFINITY_AUTO_UPDATE_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for PROCESS_AFFINITY_AUTO_UPDATE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Kernel",
    feature = "Win32_System_WindowsProgramming"
))]
pub struct PROCESS_BASIC_INFORMATION {
    pub Reserved1: *mut ::std::ffi::c_void,
    pub PebBaseAddress: *mut PEB,
    pub Reserved2: [*mut ::std::ffi::c_void; 2],
    pub UniqueProcessId: usize,
    pub Reserved3: *mut ::std::ffi::c_void,
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Kernel",
    feature = "Win32_System_WindowsProgramming"
))]
impl PROCESS_BASIC_INFORMATION {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Kernel",
    feature = "Win32_System_WindowsProgramming"
))]
impl ::std::default::Default for PROCESS_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Kernel",
    feature = "Win32_System_WindowsProgramming"
))]
impl ::std::fmt::Debug for PROCESS_BASIC_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PROCESS_BASIC_INFORMATION")
            .field("Reserved1", &self.Reserved1)
            .field("PebBaseAddress", &self.PebBaseAddress)
            .field("Reserved2", &self.Reserved2)
            .field("UniqueProcessId", &self.UniqueProcessId)
            .field("Reserved3", &self.Reserved3)
            .finish()
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Kernel",
    feature = "Win32_System_WindowsProgramming"
))]
impl ::std::cmp::PartialEq for PROCESS_BASIC_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved1 == other.Reserved1
            && self.PebBaseAddress == other.PebBaseAddress
            && self.Reserved2 == other.Reserved2
            && self.UniqueProcessId == other.UniqueProcessId
            && self.Reserved3 == other.Reserved3
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Kernel",
    feature = "Win32_System_WindowsProgramming"
))]
impl ::std::cmp::Eq for PROCESS_BASIC_INFORMATION {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Kernel",
    feature = "Win32_System_WindowsProgramming"
))]
unsafe impl ::windows::runtime::Abi for PROCESS_BASIC_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct PROCESS_CREATION_FLAGS(pub u32);
pub const DEBUG_PROCESS: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(1u32);
pub const DEBUG_ONLY_THIS_PROCESS: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(2u32);
pub const CREATE_SUSPENDED: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(4u32);
pub const DETACHED_PROCESS: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(8u32);
pub const CREATE_NEW_CONSOLE: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(16u32);
pub const NORMAL_PRIORITY_CLASS: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(32u32);
pub const IDLE_PRIORITY_CLASS: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(64u32);
pub const HIGH_PRIORITY_CLASS: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(128u32);
pub const REALTIME_PRIORITY_CLASS: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(256u32);
pub const CREATE_NEW_PROCESS_GROUP: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(512u32);
pub const CREATE_UNICODE_ENVIRONMENT: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(1024u32);
pub const CREATE_SEPARATE_WOW_VDM: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(2048u32);
pub const CREATE_SHARED_WOW_VDM: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(4096u32);
pub const CREATE_FORCEDOS: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(8192u32);
pub const BELOW_NORMAL_PRIORITY_CLASS: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(16384u32);
pub const ABOVE_NORMAL_PRIORITY_CLASS: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(32768u32);
pub const INHERIT_PARENT_AFFINITY: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(65536u32);
pub const INHERIT_CALLER_PRIORITY: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(131072u32);
pub const CREATE_PROTECTED_PROCESS: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(262144u32);
pub const EXTENDED_STARTUPINFO_PRESENT: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(524288u32);
pub const PROCESS_MODE_BACKGROUND_BEGIN: PROCESS_CREATION_FLAGS =
    PROCESS_CREATION_FLAGS(1048576u32);
pub const PROCESS_MODE_BACKGROUND_END: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(2097152u32);
pub const CREATE_SECURE_PROCESS: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(4194304u32);
pub const CREATE_BREAKAWAY_FROM_JOB: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(16777216u32);
pub const CREATE_PRESERVE_CODE_AUTHZ_LEVEL: PROCESS_CREATION_FLAGS =
    PROCESS_CREATION_FLAGS(33554432u32);
pub const CREATE_DEFAULT_ERROR_MODE: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(67108864u32);
pub const CREATE_NO_WINDOW: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(134217728u32);
pub const PROFILE_USER: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(268435456u32);
pub const PROFILE_KERNEL: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(536870912u32);
pub const PROFILE_SERVER: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(1073741824u32);
pub const CREATE_IGNORE_SYSTEM_DEFAULT: PROCESS_CREATION_FLAGS =
    PROCESS_CREATION_FLAGS(2147483648u32);
impl ::std::convert::From<u32> for PROCESS_CREATION_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PROCESS_CREATION_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for PROCESS_CREATION_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for PROCESS_CREATION_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for PROCESS_CREATION_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for PROCESS_CREATION_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for PROCESS_CREATION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct PROCESS_DEP_FLAGS(pub u32);
pub const PROCESS_DEP_ENABLE: PROCESS_DEP_FLAGS = PROCESS_DEP_FLAGS(1u32);
pub const PROCESS_DEP_DISABLE_ATL_THUNK_EMULATION: PROCESS_DEP_FLAGS = PROCESS_DEP_FLAGS(2u32);
pub const PROCESS_DEP_NONE: PROCESS_DEP_FLAGS = PROCESS_DEP_FLAGS(0u32);
impl ::std::convert::From<u32> for PROCESS_DEP_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PROCESS_DEP_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for PROCESS_DEP_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for PROCESS_DEP_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for PROCESS_DEP_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for PROCESS_DEP_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for PROCESS_DEP_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PROCESS_INFORMATION {
    pub hProcess: super::super::Foundation::HANDLE,
    pub hThread: super::super::Foundation::HANDLE,
    pub dwProcessId: u32,
    pub dwThreadId: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl PROCESS_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for PROCESS_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for PROCESS_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PROCESS_INFORMATION")
            .field("hProcess", &self.hProcess)
            .field("hThread", &self.hThread)
            .field("dwProcessId", &self.dwProcessId)
            .field("dwThreadId", &self.dwThreadId)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for PROCESS_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.hProcess == other.hProcess
            && self.hThread == other.hThread
            && self.dwProcessId == other.dwProcessId
            && self.dwThreadId == other.dwThreadId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for PROCESS_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PROCESS_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct PROCESS_INFORMATION_CLASS(pub i32);
pub const ProcessMemoryPriority: PROCESS_INFORMATION_CLASS = PROCESS_INFORMATION_CLASS(0i32);
pub const ProcessMemoryExhaustionInfo: PROCESS_INFORMATION_CLASS = PROCESS_INFORMATION_CLASS(1i32);
pub const ProcessAppMemoryInfo: PROCESS_INFORMATION_CLASS = PROCESS_INFORMATION_CLASS(2i32);
pub const ProcessInPrivateInfo: PROCESS_INFORMATION_CLASS = PROCESS_INFORMATION_CLASS(3i32);
pub const ProcessPowerThrottling: PROCESS_INFORMATION_CLASS = PROCESS_INFORMATION_CLASS(4i32);
pub const ProcessReservedValue1: PROCESS_INFORMATION_CLASS = PROCESS_INFORMATION_CLASS(5i32);
pub const ProcessTelemetryCoverageInfo: PROCESS_INFORMATION_CLASS = PROCESS_INFORMATION_CLASS(6i32);
pub const ProcessProtectionLevelInfo: PROCESS_INFORMATION_CLASS = PROCESS_INFORMATION_CLASS(7i32);
pub const ProcessLeapSecondInfo: PROCESS_INFORMATION_CLASS = PROCESS_INFORMATION_CLASS(8i32);
pub const ProcessMachineTypeInfo: PROCESS_INFORMATION_CLASS = PROCESS_INFORMATION_CLASS(9i32);
pub const ProcessInformationClassMax: PROCESS_INFORMATION_CLASS = PROCESS_INFORMATION_CLASS(10i32);
impl ::std::convert::From<i32> for PROCESS_INFORMATION_CLASS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PROCESS_INFORMATION_CLASS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PROCESS_LEAP_SECOND_INFO {
    pub Flags: u32,
    pub Reserved: u32,
}
impl PROCESS_LEAP_SECOND_INFO {}
impl ::std::default::Default for PROCESS_LEAP_SECOND_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PROCESS_LEAP_SECOND_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PROCESS_LEAP_SECOND_INFO")
            .field("Flags", &self.Flags)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
impl ::std::cmp::PartialEq for PROCESS_LEAP_SECOND_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.Reserved == other.Reserved
    }
}
impl ::std::cmp::Eq for PROCESS_LEAP_SECOND_INFO {}
unsafe impl ::windows::runtime::Abi for PROCESS_LEAP_SECOND_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
pub const PROCESS_LEAP_SECOND_INFO_FLAG_ENABLE_SIXTY_SECOND: u32 = 1u32;
pub const PROCESS_LEAP_SECOND_INFO_VALID_FLAGS: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PROCESS_MACHINE_INFORMATION {
    pub ProcessMachine: u16,
    pub Res0: u16,
    pub MachineAttributes: MACHINE_ATTRIBUTES,
}
impl PROCESS_MACHINE_INFORMATION {}
impl ::std::default::Default for PROCESS_MACHINE_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PROCESS_MACHINE_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PROCESS_MACHINE_INFORMATION")
            .field("ProcessMachine", &self.ProcessMachine)
            .field("Res0", &self.Res0)
            .field("MachineAttributes", &self.MachineAttributes)
            .finish()
    }
}
impl ::std::cmp::PartialEq for PROCESS_MACHINE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.ProcessMachine == other.ProcessMachine
            && self.Res0 == other.Res0
            && self.MachineAttributes == other.MachineAttributes
    }
}
impl ::std::cmp::Eq for PROCESS_MACHINE_INFORMATION {}
unsafe impl ::windows::runtime::Abi for PROCESS_MACHINE_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PROCESS_MEMORY_EXHAUSTION_INFO {
    pub Version: u16,
    pub Reserved: u16,
    pub Type: PROCESS_MEMORY_EXHAUSTION_TYPE,
    pub Value: usize,
}
impl PROCESS_MEMORY_EXHAUSTION_INFO {}
impl ::std::default::Default for PROCESS_MEMORY_EXHAUSTION_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PROCESS_MEMORY_EXHAUSTION_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PROCESS_MEMORY_EXHAUSTION_INFO")
            .field("Version", &self.Version)
            .field("Reserved", &self.Reserved)
            .field("Type", &self.Type)
            .field("Value", &self.Value)
            .finish()
    }
}
impl ::std::cmp::PartialEq for PROCESS_MEMORY_EXHAUSTION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version
            && self.Reserved == other.Reserved
            && self.Type == other.Type
            && self.Value == other.Value
    }
}
impl ::std::cmp::Eq for PROCESS_MEMORY_EXHAUSTION_INFO {}
unsafe impl ::windows::runtime::Abi for PROCESS_MEMORY_EXHAUSTION_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct PROCESS_MEMORY_EXHAUSTION_TYPE(pub i32);
pub const PMETypeFailFastOnCommitFailure: PROCESS_MEMORY_EXHAUSTION_TYPE =
    PROCESS_MEMORY_EXHAUSTION_TYPE(0i32);
pub const PMETypeMax: PROCESS_MEMORY_EXHAUSTION_TYPE = PROCESS_MEMORY_EXHAUSTION_TYPE(1i32);
impl ::std::convert::From<i32> for PROCESS_MEMORY_EXHAUSTION_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PROCESS_MEMORY_EXHAUSTION_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct PROCESS_NAME_FORMAT(pub u32);
pub const PROCESS_NAME_WIN32: PROCESS_NAME_FORMAT = PROCESS_NAME_FORMAT(0u32);
pub const PROCESS_NAME_NATIVE: PROCESS_NAME_FORMAT = PROCESS_NAME_FORMAT(1u32);
impl ::std::convert::From<u32> for PROCESS_NAME_FORMAT {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PROCESS_NAME_FORMAT {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for PROCESS_NAME_FORMAT {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for PROCESS_NAME_FORMAT {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for PROCESS_NAME_FORMAT {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for PROCESS_NAME_FORMAT {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for PROCESS_NAME_FORMAT {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const PROCESS_POWER_THROTTLING_CURRENT_VERSION: u32 = 1u32;
pub const PROCESS_POWER_THROTTLING_EXECUTION_SPEED: u32 = 1u32;
pub const PROCESS_POWER_THROTTLING_IGNORE_TIMER_RESOLUTION: u32 = 4u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PROCESS_POWER_THROTTLING_STATE {
    pub Version: u32,
    pub ControlMask: u32,
    pub StateMask: u32,
}
impl PROCESS_POWER_THROTTLING_STATE {}
impl ::std::default::Default for PROCESS_POWER_THROTTLING_STATE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PROCESS_POWER_THROTTLING_STATE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PROCESS_POWER_THROTTLING_STATE")
            .field("Version", &self.Version)
            .field("ControlMask", &self.ControlMask)
            .field("StateMask", &self.StateMask)
            .finish()
    }
}
impl ::std::cmp::PartialEq for PROCESS_POWER_THROTTLING_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version
            && self.ControlMask == other.ControlMask
            && self.StateMask == other.StateMask
    }
}
impl ::std::cmp::Eq for PROCESS_POWER_THROTTLING_STATE {}
unsafe impl ::windows::runtime::Abi for PROCESS_POWER_THROTTLING_STATE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct PROCESS_PROTECTION_LEVEL(pub u32);
pub const PROTECTION_LEVEL_WINTCB_LIGHT: PROCESS_PROTECTION_LEVEL = PROCESS_PROTECTION_LEVEL(0u32);
pub const PROTECTION_LEVEL_WINDOWS: PROCESS_PROTECTION_LEVEL = PROCESS_PROTECTION_LEVEL(1u32);
pub const PROTECTION_LEVEL_WINDOWS_LIGHT: PROCESS_PROTECTION_LEVEL = PROCESS_PROTECTION_LEVEL(2u32);
pub const PROTECTION_LEVEL_ANTIMALWARE_LIGHT: PROCESS_PROTECTION_LEVEL =
    PROCESS_PROTECTION_LEVEL(3u32);
pub const PROTECTION_LEVEL_LSA_LIGHT: PROCESS_PROTECTION_LEVEL = PROCESS_PROTECTION_LEVEL(4u32);
pub const PROTECTION_LEVEL_WINTCB: PROCESS_PROTECTION_LEVEL = PROCESS_PROTECTION_LEVEL(5u32);
pub const PROTECTION_LEVEL_CODEGEN_LIGHT: PROCESS_PROTECTION_LEVEL = PROCESS_PROTECTION_LEVEL(6u32);
pub const PROTECTION_LEVEL_AUTHENTICODE: PROCESS_PROTECTION_LEVEL = PROCESS_PROTECTION_LEVEL(7u32);
pub const PROTECTION_LEVEL_PPL_APP: PROCESS_PROTECTION_LEVEL = PROCESS_PROTECTION_LEVEL(8u32);
pub const PROTECTION_LEVEL_NONE: PROCESS_PROTECTION_LEVEL = PROCESS_PROTECTION_LEVEL(4294967294u32);
impl ::std::convert::From<u32> for PROCESS_PROTECTION_LEVEL {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PROCESS_PROTECTION_LEVEL {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for PROCESS_PROTECTION_LEVEL {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for PROCESS_PROTECTION_LEVEL {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for PROCESS_PROTECTION_LEVEL {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for PROCESS_PROTECTION_LEVEL {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for PROCESS_PROTECTION_LEVEL {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PROCESS_PROTECTION_LEVEL_INFORMATION {
    pub ProtectionLevel: PROCESS_PROTECTION_LEVEL,
}
impl PROCESS_PROTECTION_LEVEL_INFORMATION {}
impl ::std::default::Default for PROCESS_PROTECTION_LEVEL_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PROCESS_PROTECTION_LEVEL_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PROCESS_PROTECTION_LEVEL_INFORMATION")
            .field("ProtectionLevel", &self.ProtectionLevel)
            .finish()
    }
}
impl ::std::cmp::PartialEq for PROCESS_PROTECTION_LEVEL_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.ProtectionLevel == other.ProtectionLevel
    }
}
impl ::std::cmp::Eq for PROCESS_PROTECTION_LEVEL_INFORMATION {}
unsafe impl ::windows::runtime::Abi for PROCESS_PROTECTION_LEVEL_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
pub const PROC_THREAD_ATTRIBUTE_REPLACE_VALUE: u32 = 1u32;
pub type PTIMERAPCROUTINE = unsafe extern "system" fn(
    lpargtocompletionroutine: *const ::std::ffi::c_void,
    dwtimerlowvalue: u32,
    dwtimerhighvalue: u32,
);
#[derive(
    :: std :: clone :: Clone,
    :: std :: marker :: Copy,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
)]
#[repr(transparent)]
pub struct PTP_POOL(pub isize);
impl ::std::default::Default for PTP_POOL {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for PTP_POOL {}
unsafe impl ::windows::runtime::Abi for PTP_POOL {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_System_SystemServices")]
pub type PTP_WIN32_IO_CALLBACK = unsafe extern "system" fn(
    instance: *mut super::SystemServices::TP_CALLBACK_INSTANCE,
    context: *mut ::std::ffi::c_void,
    overlapped: *mut ::std::ffi::c_void,
    ioresult: u32,
    numberofbytestransferred: usize,
    io: *mut super::SystemServices::TP_IO,
);
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn PulseEvent<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hevent: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PulseEvent(
                hevent: super::super::Foundation::HANDLE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(PulseEvent(hevent.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct QUEUE_USER_APC_FLAGS(pub i32);
pub const QUEUE_USER_APC_FLAGS_NONE: QUEUE_USER_APC_FLAGS = QUEUE_USER_APC_FLAGS(0i32);
pub const QUEUE_USER_APC_FLAGS_SPECIAL_USER_APC: QUEUE_USER_APC_FLAGS = QUEUE_USER_APC_FLAGS(1i32);
impl ::std::convert::From<i32> for QUEUE_USER_APC_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for QUEUE_USER_APC_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_System_Kernel")]
pub unsafe fn QueryDepthSList(listhead: *const super::Kernel::SLIST_HEADER) -> u16 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryDepthSList(listhead: *const super::Kernel::SLIST_HEADER) -> u16;
        }
        ::std::mem::transmute(QueryDepthSList(::std::mem::transmute(listhead)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn QueryFullProcessImageNameA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hprocess: Param0,
    dwflags: PROCESS_NAME_FORMAT,
    lpexename: super::super::Foundation::PSTR,
    lpdwsize: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryFullProcessImageNameA(
                hprocess: super::super::Foundation::HANDLE,
                dwflags: PROCESS_NAME_FORMAT,
                lpexename: super::super::Foundation::PSTR,
                lpdwsize: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(QueryFullProcessImageNameA(
            hprocess.into_param().abi(),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(lpexename),
            ::std::mem::transmute(lpdwsize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn QueryFullProcessImageNameW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hprocess: Param0,
    dwflags: PROCESS_NAME_FORMAT,
    lpexename: super::super::Foundation::PWSTR,
    lpdwsize: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryFullProcessImageNameW(
                hprocess: super::super::Foundation::HANDLE,
                dwflags: PROCESS_NAME_FORMAT,
                lpexename: super::super::Foundation::PWSTR,
                lpdwsize: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(QueryFullProcessImageNameW(
            hprocess.into_param().abi(),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(lpexename),
            ::std::mem::transmute(lpdwsize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn QueryProcessAffinityUpdateMode<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hprocess: Param0,
    lpdwflags: *mut PROCESS_AFFINITY_AUTO_UPDATE_FLAGS,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryProcessAffinityUpdateMode(
                hprocess: super::super::Foundation::HANDLE,
                lpdwflags: *mut PROCESS_AFFINITY_AUTO_UPDATE_FLAGS,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(QueryProcessAffinityUpdateMode(
            hprocess.into_param().abi(),
            ::std::mem::transmute(lpdwflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn QueryProtectedPolicy(
    policyguid: *const ::windows::runtime::GUID,
    policyvalue: *mut usize,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryProtectedPolicy(
                policyguid: *const ::windows::runtime::GUID,
                policyvalue: *mut usize,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(QueryProtectedPolicy(
            ::std::mem::transmute(policyguid),
            ::std::mem::transmute(policyvalue),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn QueryThreadpoolStackInformation<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, PTP_POOL>,
>(
    ptpp: Param0,
    ptpsi: *mut super::SystemServices::TP_POOL_STACK_INFORMATION,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryThreadpoolStackInformation(
                ptpp: PTP_POOL,
                ptpsi: *mut super::SystemServices::TP_POOL_STACK_INFORMATION,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(QueryThreadpoolStackInformation(
            ptpp.into_param().abi(),
            ::std::mem::transmute(ptpsi),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn QueryUmsThreadInformation(
    umsthread: *const ::std::ffi::c_void,
    umsthreadinfoclass: super::SystemServices::RTL_UMS_THREAD_INFO_CLASS,
    umsthreadinformation: *mut ::std::ffi::c_void,
    umsthreadinformationlength: u32,
    returnlength: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryUmsThreadInformation(
                umsthread: *const ::std::ffi::c_void,
                umsthreadinfoclass: super::SystemServices::RTL_UMS_THREAD_INFO_CLASS,
                umsthreadinformation: *mut ::std::ffi::c_void,
                umsthreadinformationlength: u32,
                returnlength: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(QueryUmsThreadInformation(
            ::std::mem::transmute(umsthread),
            ::std::mem::transmute(umsthreadinfoclass),
            ::std::mem::transmute(umsthreadinformation),
            ::std::mem::transmute(umsthreadinformationlength),
            ::std::mem::transmute(returnlength),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn QueueUserAPC<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    pfnapc: ::std::option::Option<super::SystemServices::PAPCFUNC>,
    hthread: Param1,
    dwdata: usize,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueueUserAPC(
                pfnapc: ::windows::runtime::RawPtr,
                hthread: super::super::Foundation::HANDLE,
                dwdata: usize,
            ) -> u32;
        }
        ::std::mem::transmute(QueueUserAPC(
            ::std::mem::transmute(pfnapc),
            hthread.into_param().abi(),
            ::std::mem::transmute(dwdata),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn QueueUserAPC2<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    apcroutine: ::std::option::Option<super::SystemServices::PAPCFUNC>,
    thread: Param1,
    data: usize,
    flags: QUEUE_USER_APC_FLAGS,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueueUserAPC2(
                apcroutine: ::windows::runtime::RawPtr,
                thread: super::super::Foundation::HANDLE,
                data: usize,
                flags: QUEUE_USER_APC_FLAGS,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(QueueUserAPC2(
            ::std::mem::transmute(apcroutine),
            thread.into_param().abi(),
            ::std::mem::transmute(data),
            ::std::mem::transmute(flags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn QueueUserWorkItem(
    function: ::std::option::Option<super::SystemServices::LPTHREAD_START_ROUTINE>,
    context: *const ::std::ffi::c_void,
    flags: WORKER_THREAD_FLAGS,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueueUserWorkItem(
                function: ::windows::runtime::RawPtr,
                context: *const ::std::ffi::c_void,
                flags: WORKER_THREAD_FLAGS,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(QueueUserWorkItem(
            ::std::mem::transmute(function),
            ::std::mem::transmute(context),
            ::std::mem::transmute(flags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub struct RTL_USER_PROCESS_PARAMETERS {
    pub Reserved1: [u8; 16],
    pub Reserved2: [*mut ::std::ffi::c_void; 10],
    pub ImagePathName: super::Kernel::UNICODE_STRING,
    pub CommandLine: super::Kernel::UNICODE_STRING,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl RTL_USER_PROCESS_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::std::default::Default for RTL_USER_PROCESS_PARAMETERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::std::fmt::Debug for RTL_USER_PROCESS_PARAMETERS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RTL_USER_PROCESS_PARAMETERS")
            .field("Reserved1", &self.Reserved1)
            .field("Reserved2", &self.Reserved2)
            .field("ImagePathName", &self.ImagePathName)
            .field("CommandLine", &self.CommandLine)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::std::cmp::PartialEq for RTL_USER_PROCESS_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved1 == other.Reserved1
            && self.Reserved2 == other.Reserved2
            && self.ImagePathName == other.ImagePathName
            && self.CommandLine == other.CommandLine
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::std::cmp::Eq for RTL_USER_PROCESS_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
unsafe impl ::windows::runtime::Abi for RTL_USER_PROCESS_PARAMETERS {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn RegisterWaitForSingleObject<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    phnewwaitobject: *mut super::super::Foundation::HANDLE,
    hobject: Param1,
    callback: ::std::option::Option<super::SystemServices::WAITORTIMERCALLBACK>,
    context: *const ::std::ffi::c_void,
    dwmilliseconds: u32,
    dwflags: WORKER_THREAD_FLAGS,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterWaitForSingleObject(
                phnewwaitobject: *mut super::super::Foundation::HANDLE,
                hobject: super::super::Foundation::HANDLE,
                callback: ::windows::runtime::RawPtr,
                context: *const ::std::ffi::c_void,
                dwmilliseconds: u32,
                dwflags: WORKER_THREAD_FLAGS,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(RegisterWaitForSingleObject(
            ::std::mem::transmute(phnewwaitobject),
            hobject.into_param().abi(),
            ::std::mem::transmute(callback),
            ::std::mem::transmute(context),
            ::std::mem::transmute(dwmilliseconds),
            ::std::mem::transmute(dwflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn ReleaseMutex<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hmutex: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReleaseMutex(
                hmutex: super::super::Foundation::HANDLE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ReleaseMutex(hmutex.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn ReleaseMutexWhenCallbackReturns<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    pci: *mut super::SystemServices::TP_CALLBACK_INSTANCE,
    r#mut: Param1,
) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReleaseMutexWhenCallbackReturns(
                pci: *mut super::SystemServices::TP_CALLBACK_INSTANCE,
                r#mut: super::super::Foundation::HANDLE,
            );
        }
        ::std::mem::transmute(ReleaseMutexWhenCallbackReturns(
            ::std::mem::transmute(pci),
            r#mut.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn ReleaseSRWLockExclusive(srwlock: *mut super::SystemServices::RTL_SRWLOCK) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReleaseSRWLockExclusive(srwlock: *mut super::SystemServices::RTL_SRWLOCK);
        }
        ::std::mem::transmute(ReleaseSRWLockExclusive(::std::mem::transmute(srwlock)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn ReleaseSRWLockShared(srwlock: *mut super::SystemServices::RTL_SRWLOCK) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReleaseSRWLockShared(srwlock: *mut super::SystemServices::RTL_SRWLOCK);
        }
        ::std::mem::transmute(ReleaseSRWLockShared(::std::mem::transmute(srwlock)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn ReleaseSemaphore<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hsemaphore: Param0,
    lreleasecount: i32,
    lppreviouscount: *mut i32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReleaseSemaphore(
                hsemaphore: super::super::Foundation::HANDLE,
                lreleasecount: i32,
                lppreviouscount: *mut i32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ReleaseSemaphore(
            hsemaphore.into_param().abi(),
            ::std::mem::transmute(lreleasecount),
            ::std::mem::transmute(lppreviouscount),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn ReleaseSemaphoreWhenCallbackReturns<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    pci: *mut super::SystemServices::TP_CALLBACK_INSTANCE,
    sem: Param1,
    crel: u32,
) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReleaseSemaphoreWhenCallbackReturns(
                pci: *mut super::SystemServices::TP_CALLBACK_INSTANCE,
                sem: super::super::Foundation::HANDLE,
                crel: u32,
            );
        }
        ::std::mem::transmute(ReleaseSemaphoreWhenCallbackReturns(
            ::std::mem::transmute(pci),
            sem.into_param().abi(),
            ::std::mem::transmute(crel),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn ResetEvent<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hevent: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ResetEvent(
                hevent: super::super::Foundation::HANDLE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ResetEvent(hevent.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn ResumeThread<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hthread: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ResumeThread(hthread: super::super::Foundation::HANDLE) -> u32;
        }
        ::std::mem::transmute(ResumeThread(hthread.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct STARTUPINFOA {
    pub cb: u32,
    pub lpReserved: super::super::Foundation::PSTR,
    pub lpDesktop: super::super::Foundation::PSTR,
    pub lpTitle: super::super::Foundation::PSTR,
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
impl STARTUPINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for STARTUPINFOA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for STARTUPINFOA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("STARTUPINFOA")
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
impl ::std::cmp::PartialEq for STARTUPINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb
            && self.lpReserved == other.lpReserved
            && self.lpDesktop == other.lpDesktop
            && self.lpTitle == other.lpTitle
            && self.dwX == other.dwX
            && self.dwY == other.dwY
            && self.dwXSize == other.dwXSize
            && self.dwYSize == other.dwYSize
            && self.dwXCountChars == other.dwXCountChars
            && self.dwYCountChars == other.dwYCountChars
            && self.dwFillAttribute == other.dwFillAttribute
            && self.dwFlags == other.dwFlags
            && self.wShowWindow == other.wShowWindow
            && self.cbReserved2 == other.cbReserved2
            && self.lpReserved2 == other.lpReserved2
            && self.hStdInput == other.hStdInput
            && self.hStdOutput == other.hStdOutput
            && self.hStdError == other.hStdError
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for STARTUPINFOA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for STARTUPINFOA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct STARTUPINFOEXA {
    pub StartupInfo: STARTUPINFOA,
    pub lpAttributeList: LPPROC_THREAD_ATTRIBUTE_LIST,
}
#[cfg(feature = "Win32_Foundation")]
impl STARTUPINFOEXA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for STARTUPINFOEXA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for STARTUPINFOEXA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("STARTUPINFOEXA")
            .field("StartupInfo", &self.StartupInfo)
            .field("lpAttributeList", &self.lpAttributeList)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for STARTUPINFOEXA {
    fn eq(&self, other: &Self) -> bool {
        self.StartupInfo == other.StartupInfo && self.lpAttributeList == other.lpAttributeList
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for STARTUPINFOEXA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for STARTUPINFOEXA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct STARTUPINFOEXW {
    pub StartupInfo: STARTUPINFOW,
    pub lpAttributeList: LPPROC_THREAD_ATTRIBUTE_LIST,
}
#[cfg(feature = "Win32_Foundation")]
impl STARTUPINFOEXW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for STARTUPINFOEXW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for STARTUPINFOEXW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("STARTUPINFOEXW")
            .field("StartupInfo", &self.StartupInfo)
            .field("lpAttributeList", &self.lpAttributeList)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for STARTUPINFOEXW {
    fn eq(&self, other: &Self) -> bool {
        self.StartupInfo == other.StartupInfo && self.lpAttributeList == other.lpAttributeList
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for STARTUPINFOEXW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for STARTUPINFOEXW {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct STARTUPINFOW {
    pub cb: u32,
    pub lpReserved: super::super::Foundation::PWSTR,
    pub lpDesktop: super::super::Foundation::PWSTR,
    pub lpTitle: super::super::Foundation::PWSTR,
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
impl STARTUPINFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for STARTUPINFOW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for STARTUPINFOW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("STARTUPINFOW")
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
impl ::std::cmp::PartialEq for STARTUPINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb
            && self.lpReserved == other.lpReserved
            && self.lpDesktop == other.lpDesktop
            && self.lpTitle == other.lpTitle
            && self.dwX == other.dwX
            && self.dwY == other.dwY
            && self.dwXSize == other.dwXSize
            && self.dwYSize == other.dwYSize
            && self.dwXCountChars == other.dwXCountChars
            && self.dwYCountChars == other.dwYCountChars
            && self.dwFillAttribute == other.dwFillAttribute
            && self.dwFlags == other.dwFlags
            && self.wShowWindow == other.wShowWindow
            && self.cbReserved2 == other.cbReserved2
            && self.lpReserved2 == other.lpReserved2
            && self.hStdInput == other.hStdInput
            && self.hStdOutput == other.hStdOutput
            && self.hStdError == other.hStdError
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for STARTUPINFOW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for STARTUPINFOW {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct STARTUPINFOW_FLAGS(pub u32);
pub const STARTF_FORCEONFEEDBACK: STARTUPINFOW_FLAGS = STARTUPINFOW_FLAGS(64u32);
pub const STARTF_FORCEOFFFEEDBACK: STARTUPINFOW_FLAGS = STARTUPINFOW_FLAGS(128u32);
pub const STARTF_PREVENTPINNING: STARTUPINFOW_FLAGS = STARTUPINFOW_FLAGS(8192u32);
pub const STARTF_RUNFULLSCREEN: STARTUPINFOW_FLAGS = STARTUPINFOW_FLAGS(32u32);
pub const STARTF_TITLEISAPPID: STARTUPINFOW_FLAGS = STARTUPINFOW_FLAGS(4096u32);
pub const STARTF_TITLEISLINKNAME: STARTUPINFOW_FLAGS = STARTUPINFOW_FLAGS(2048u32);
pub const STARTF_UNTRUSTEDSOURCE: STARTUPINFOW_FLAGS = STARTUPINFOW_FLAGS(32768u32);
pub const STARTF_USECOUNTCHARS: STARTUPINFOW_FLAGS = STARTUPINFOW_FLAGS(8u32);
pub const STARTF_USEFILLATTRIBUTE: STARTUPINFOW_FLAGS = STARTUPINFOW_FLAGS(16u32);
pub const STARTF_USEHOTKEY: STARTUPINFOW_FLAGS = STARTUPINFOW_FLAGS(512u32);
pub const STARTF_USEPOSITION: STARTUPINFOW_FLAGS = STARTUPINFOW_FLAGS(4u32);
pub const STARTF_USESHOWWINDOW: STARTUPINFOW_FLAGS = STARTUPINFOW_FLAGS(1u32);
pub const STARTF_USESIZE: STARTUPINFOW_FLAGS = STARTUPINFOW_FLAGS(2u32);
pub const STARTF_USESTDHANDLES: STARTUPINFOW_FLAGS = STARTUPINFOW_FLAGS(256u32);
impl ::std::convert::From<u32> for STARTUPINFOW_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for STARTUPINFOW_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for STARTUPINFOW_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for STARTUPINFOW_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for STARTUPINFOW_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for STARTUPINFOW_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for STARTUPINFOW_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const SYNCHRONIZATION_BARRIER_FLAGS_BLOCK_ONLY: u32 = 2u32;
pub const SYNCHRONIZATION_BARRIER_FLAGS_NO_DELETE: u32 = 4u32;
pub const SYNCHRONIZATION_BARRIER_FLAGS_SPIN_ONLY: u32 = 1u32;
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Kernel",
    feature = "Win32_System_SystemServices"
))]
pub unsafe fn SetCriticalSectionSpinCount(
    lpcriticalsection: *mut super::SystemServices::RTL_CRITICAL_SECTION,
    dwspincount: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetCriticalSectionSpinCount(
                lpcriticalsection: *mut super::SystemServices::RTL_CRITICAL_SECTION,
                dwspincount: u32,
            ) -> u32;
        }
        ::std::mem::transmute(SetCriticalSectionSpinCount(
            ::std::mem::transmute(lpcriticalsection),
            ::std::mem::transmute(dwspincount),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SetEvent<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hevent: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetEvent(hevent: super::super::Foundation::HANDLE)
                -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetEvent(hevent.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn SetEventWhenCallbackReturns<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    pci: *mut super::SystemServices::TP_CALLBACK_INSTANCE,
    evt: Param1,
) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetEventWhenCallbackReturns(
                pci: *mut super::SystemServices::TP_CALLBACK_INSTANCE,
                evt: super::super::Foundation::HANDLE,
            );
        }
        ::std::mem::transmute(SetEventWhenCallbackReturns(
            ::std::mem::transmute(pci),
            evt.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SetPriorityClass<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hprocess: Param0,
    dwpriorityclass: PROCESS_CREATION_FLAGS,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetPriorityClass(
                hprocess: super::super::Foundation::HANDLE,
                dwpriorityclass: PROCESS_CREATION_FLAGS,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetPriorityClass(
            hprocess.into_param().abi(),
            ::std::mem::transmute(dwpriorityclass),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SetProcessAffinityMask<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hprocess: Param0,
    dwprocessaffinitymask: usize,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetProcessAffinityMask(
                hprocess: super::super::Foundation::HANDLE,
                dwprocessaffinitymask: usize,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetProcessAffinityMask(
            hprocess.into_param().abi(),
            ::std::mem::transmute(dwprocessaffinitymask),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SetProcessAffinityUpdateMode<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hprocess: Param0,
    dwflags: PROCESS_AFFINITY_AUTO_UPDATE_FLAGS,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetProcessAffinityUpdateMode(
                hprocess: super::super::Foundation::HANDLE,
                dwflags: PROCESS_AFFINITY_AUTO_UPDATE_FLAGS,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetProcessAffinityUpdateMode(
            hprocess.into_param().abi(),
            ::std::mem::transmute(dwflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SetProcessDEPPolicy(dwflags: PROCESS_DEP_FLAGS) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetProcessDEPPolicy(dwflags: PROCESS_DEP_FLAGS) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetProcessDEPPolicy(::std::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub unsafe fn SetProcessDefaultCpuSetMasks<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    process: Param0,
    cpusetmasks: *const super::Kernel::GROUP_AFFINITY,
    cpusetmaskcount: u16,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetProcessDefaultCpuSetMasks(
                process: super::super::Foundation::HANDLE,
                cpusetmasks: *const super::Kernel::GROUP_AFFINITY,
                cpusetmaskcount: u16,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetProcessDefaultCpuSetMasks(
            process.into_param().abi(),
            ::std::mem::transmute(cpusetmasks),
            ::std::mem::transmute(cpusetmaskcount),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SetProcessDefaultCpuSets<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    process: Param0,
    cpusetids: *const u32,
    cpusetidcount: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetProcessDefaultCpuSets(
                process: super::super::Foundation::HANDLE,
                cpusetids: *const u32,
                cpusetidcount: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetProcessDefaultCpuSets(
            process.into_param().abi(),
            ::std::mem::transmute(cpusetids),
            ::std::mem::transmute(cpusetidcount),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn SetProcessDynamicEHContinuationTargets<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    process: Param0,
    numberoftargets: u16,
    targets: *mut super::SystemServices::PROCESS_DYNAMIC_EH_CONTINUATION_TARGET,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetProcessDynamicEHContinuationTargets(
                process: super::super::Foundation::HANDLE,
                numberoftargets: u16,
                targets: *mut super::SystemServices::PROCESS_DYNAMIC_EH_CONTINUATION_TARGET,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetProcessDynamicEHContinuationTargets(
            process.into_param().abi(),
            ::std::mem::transmute(numberoftargets),
            ::std::mem::transmute(targets),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn SetProcessDynamicEnforcedCetCompatibleRanges<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    process: Param0,
    numberofranges: u16,
    ranges: *mut super::SystemServices::PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetProcessDynamicEnforcedCetCompatibleRanges(
                process: super::super::Foundation::HANDLE,
                numberofranges: u16,
                ranges: *mut super::SystemServices::PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetProcessDynamicEnforcedCetCompatibleRanges(
            process.into_param().abi(),
            ::std::mem::transmute(numberofranges),
            ::std::mem::transmute(ranges),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SetProcessInformation<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hprocess: Param0,
    processinformationclass: PROCESS_INFORMATION_CLASS,
    processinformation: *const ::std::ffi::c_void,
    processinformationsize: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetProcessInformation(
                hprocess: super::super::Foundation::HANDLE,
                processinformationclass: PROCESS_INFORMATION_CLASS,
                processinformation: *const ::std::ffi::c_void,
                processinformationsize: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetProcessInformation(
            hprocess.into_param().abi(),
            ::std::mem::transmute(processinformationclass),
            ::std::mem::transmute(processinformation),
            ::std::mem::transmute(processinformationsize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn SetProcessMitigationPolicy(
    mitigationpolicy: super::SystemServices::PROCESS_MITIGATION_POLICY,
    lpbuffer: *const ::std::ffi::c_void,
    dwlength: usize,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetProcessMitigationPolicy(
                mitigationpolicy: super::SystemServices::PROCESS_MITIGATION_POLICY,
                lpbuffer: *const ::std::ffi::c_void,
                dwlength: usize,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetProcessMitigationPolicy(
            ::std::mem::transmute(mitigationpolicy),
            ::std::mem::transmute(lpbuffer),
            ::std::mem::transmute(dwlength),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SetProcessPriorityBoost<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    hprocess: Param0,
    bdisablepriorityboost: Param1,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetProcessPriorityBoost(
                hprocess: super::super::Foundation::HANDLE,
                bdisablepriorityboost: super::super::Foundation::BOOL,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetProcessPriorityBoost(
            hprocess.into_param().abi(),
            bdisablepriorityboost.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SetProcessRestrictionExemption<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    fenableexemption: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetProcessRestrictionExemption(
                fenableexemption: super::super::Foundation::BOOL,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetProcessRestrictionExemption(
            fenableexemption.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SetProcessShutdownParameters(
    dwlevel: u32,
    dwflags: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetProcessShutdownParameters(
                dwlevel: u32,
                dwflags: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetProcessShutdownParameters(
            ::std::mem::transmute(dwlevel),
            ::std::mem::transmute(dwflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SetProcessWorkingSetSize<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hprocess: Param0,
    dwminimumworkingsetsize: usize,
    dwmaximumworkingsetsize: usize,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetProcessWorkingSetSize(
                hprocess: super::super::Foundation::HANDLE,
                dwminimumworkingsetsize: usize,
                dwmaximumworkingsetsize: usize,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetProcessWorkingSetSize(
            hprocess.into_param().abi(),
            ::std::mem::transmute(dwminimumworkingsetsize),
            ::std::mem::transmute(dwmaximumworkingsetsize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SetProtectedPolicy(
    policyguid: *const ::windows::runtime::GUID,
    policyvalue: usize,
    oldpolicyvalue: *mut usize,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetProtectedPolicy(
                policyguid: *const ::windows::runtime::GUID,
                policyvalue: usize,
                oldpolicyvalue: *mut usize,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetProtectedPolicy(
            ::std::mem::transmute(policyguid),
            ::std::mem::transmute(policyvalue),
            ::std::mem::transmute(oldpolicyvalue),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SetThreadAffinityMask<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hthread: Param0,
    dwthreadaffinitymask: usize,
) -> usize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetThreadAffinityMask(
                hthread: super::super::Foundation::HANDLE,
                dwthreadaffinitymask: usize,
            ) -> usize;
        }
        ::std::mem::transmute(SetThreadAffinityMask(
            hthread.into_param().abi(),
            ::std::mem::transmute(dwthreadaffinitymask),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SetThreadDescription<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hthread: Param0,
    lpthreaddescription: Param1,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetThreadDescription(
                hthread: super::super::Foundation::HANDLE,
                lpthreaddescription: super::super::Foundation::PWSTR,
            ) -> ::windows::runtime::HRESULT;
        }
        SetThreadDescription(
            hthread.into_param().abi(),
            lpthreaddescription.into_param().abi(),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub unsafe fn SetThreadGroupAffinity<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hthread: Param0,
    groupaffinity: *const super::Kernel::GROUP_AFFINITY,
    previousgroupaffinity: *mut super::Kernel::GROUP_AFFINITY,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetThreadGroupAffinity(
                hthread: super::super::Foundation::HANDLE,
                groupaffinity: *const super::Kernel::GROUP_AFFINITY,
                previousgroupaffinity: *mut super::Kernel::GROUP_AFFINITY,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetThreadGroupAffinity(
            hthread.into_param().abi(),
            ::std::mem::transmute(groupaffinity),
            ::std::mem::transmute(previousgroupaffinity),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SetThreadIdealProcessor<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hthread: Param0,
    dwidealprocessor: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetThreadIdealProcessor(
                hthread: super::super::Foundation::HANDLE,
                dwidealprocessor: u32,
            ) -> u32;
        }
        ::std::mem::transmute(SetThreadIdealProcessor(
            hthread.into_param().abi(),
            ::std::mem::transmute(dwidealprocessor),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub unsafe fn SetThreadIdealProcessorEx<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hthread: Param0,
    lpidealprocessor: *const super::Kernel::PROCESSOR_NUMBER,
    lppreviousidealprocessor: *mut super::Kernel::PROCESSOR_NUMBER,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetThreadIdealProcessorEx(
                hthread: super::super::Foundation::HANDLE,
                lpidealprocessor: *const super::Kernel::PROCESSOR_NUMBER,
                lppreviousidealprocessor: *mut super::Kernel::PROCESSOR_NUMBER,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetThreadIdealProcessorEx(
            hthread.into_param().abi(),
            ::std::mem::transmute(lpidealprocessor),
            ::std::mem::transmute(lppreviousidealprocessor),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SetThreadInformation<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hthread: Param0,
    threadinformationclass: THREAD_INFORMATION_CLASS,
    threadinformation: *const ::std::ffi::c_void,
    threadinformationsize: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetThreadInformation(
                hthread: super::super::Foundation::HANDLE,
                threadinformationclass: THREAD_INFORMATION_CLASS,
                threadinformation: *const ::std::ffi::c_void,
                threadinformationsize: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetThreadInformation(
            hthread.into_param().abi(),
            ::std::mem::transmute(threadinformationclass),
            ::std::mem::transmute(threadinformation),
            ::std::mem::transmute(threadinformationsize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SetThreadPriority<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hthread: Param0,
    npriority: THREAD_PRIORITY,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetThreadPriority(
                hthread: super::super::Foundation::HANDLE,
                npriority: THREAD_PRIORITY,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetThreadPriority(
            hthread.into_param().abi(),
            ::std::mem::transmute(npriority),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SetThreadPriorityBoost<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    hthread: Param0,
    bdisablepriorityboost: Param1,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetThreadPriorityBoost(
                hthread: super::super::Foundation::HANDLE,
                bdisablepriorityboost: super::super::Foundation::BOOL,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetThreadPriorityBoost(
            hthread.into_param().abi(),
            bdisablepriorityboost.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub unsafe fn SetThreadSelectedCpuSetMasks<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    thread: Param0,
    cpusetmasks: *const super::Kernel::GROUP_AFFINITY,
    cpusetmaskcount: u16,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetThreadSelectedCpuSetMasks(
                thread: super::super::Foundation::HANDLE,
                cpusetmasks: *const super::Kernel::GROUP_AFFINITY,
                cpusetmaskcount: u16,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetThreadSelectedCpuSetMasks(
            thread.into_param().abi(),
            ::std::mem::transmute(cpusetmasks),
            ::std::mem::transmute(cpusetmaskcount),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SetThreadSelectedCpuSets<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    thread: Param0,
    cpusetids: *const u32,
    cpusetidcount: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetThreadSelectedCpuSets(
                thread: super::super::Foundation::HANDLE,
                cpusetids: *const u32,
                cpusetidcount: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetThreadSelectedCpuSets(
            thread.into_param().abi(),
            ::std::mem::transmute(cpusetids),
            ::std::mem::transmute(cpusetidcount),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SetThreadStackGuarantee(
    stacksizeinbytes: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetThreadStackGuarantee(
                stacksizeinbytes: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetThreadStackGuarantee(::std::mem::transmute(
            stacksizeinbytes,
        )))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SetThreadToken<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    thread: *const super::super::Foundation::HANDLE,
    token: Param1,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetThreadToken(
                thread: *const super::super::Foundation::HANDLE,
                token: super::super::Foundation::HANDLE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetThreadToken(
            ::std::mem::transmute(thread),
            token.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn SetThreadpoolStackInformation<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, PTP_POOL>,
>(
    ptpp: Param0,
    ptpsi: *const super::SystemServices::TP_POOL_STACK_INFORMATION,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetThreadpoolStackInformation(
                ptpp: PTP_POOL,
                ptpsi: *const super::SystemServices::TP_POOL_STACK_INFORMATION,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetThreadpoolStackInformation(
            ptpp.into_param().abi(),
            ::std::mem::transmute(ptpsi),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn SetThreadpoolThreadMaximum<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, PTP_POOL>,
>(
    ptpp: Param0,
    cthrdmost: u32,
) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetThreadpoolThreadMaximum(ptpp: PTP_POOL, cthrdmost: u32);
        }
        ::std::mem::transmute(SetThreadpoolThreadMaximum(
            ptpp.into_param().abi(),
            ::std::mem::transmute(cthrdmost),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SetThreadpoolThreadMinimum<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, PTP_POOL>,
>(
    ptpp: Param0,
    cthrdmic: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetThreadpoolThreadMinimum(
                ptpp: PTP_POOL,
                cthrdmic: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetThreadpoolThreadMinimum(
            ptpp.into_param().abi(),
            ::std::mem::transmute(cthrdmic),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn SetThreadpoolTimer(
    pti: *mut super::SystemServices::TP_TIMER,
    pftduetime: *const super::super::Foundation::FILETIME,
    msperiod: u32,
    mswindowlength: u32,
) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetThreadpoolTimer(
                pti: *mut super::SystemServices::TP_TIMER,
                pftduetime: *const super::super::Foundation::FILETIME,
                msperiod: u32,
                mswindowlength: u32,
            );
        }
        ::std::mem::transmute(SetThreadpoolTimer(
            ::std::mem::transmute(pti),
            ::std::mem::transmute(pftduetime),
            ::std::mem::transmute(msperiod),
            ::std::mem::transmute(mswindowlength),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn SetThreadpoolTimerEx(
    pti: *mut super::SystemServices::TP_TIMER,
    pftduetime: *const super::super::Foundation::FILETIME,
    msperiod: u32,
    mswindowlength: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetThreadpoolTimerEx(
                pti: *mut super::SystemServices::TP_TIMER,
                pftduetime: *const super::super::Foundation::FILETIME,
                msperiod: u32,
                mswindowlength: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetThreadpoolTimerEx(
            ::std::mem::transmute(pti),
            ::std::mem::transmute(pftduetime),
            ::std::mem::transmute(msperiod),
            ::std::mem::transmute(mswindowlength),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn SetThreadpoolWait<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    pwa: *mut super::SystemServices::TP_WAIT,
    h: Param1,
    pfttimeout: *const super::super::Foundation::FILETIME,
) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetThreadpoolWait(
                pwa: *mut super::SystemServices::TP_WAIT,
                h: super::super::Foundation::HANDLE,
                pfttimeout: *const super::super::Foundation::FILETIME,
            );
        }
        ::std::mem::transmute(SetThreadpoolWait(
            ::std::mem::transmute(pwa),
            h.into_param().abi(),
            ::std::mem::transmute(pfttimeout),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn SetThreadpoolWaitEx<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    pwa: *mut super::SystemServices::TP_WAIT,
    h: Param1,
    pfttimeout: *const super::super::Foundation::FILETIME,
    reserved: *mut ::std::ffi::c_void,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetThreadpoolWaitEx(
                pwa: *mut super::SystemServices::TP_WAIT,
                h: super::super::Foundation::HANDLE,
                pfttimeout: *const super::super::Foundation::FILETIME,
                reserved: *mut ::std::ffi::c_void,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetThreadpoolWaitEx(
            ::std::mem::transmute(pwa),
            h.into_param().abi(),
            ::std::mem::transmute(pfttimeout),
            ::std::mem::transmute(reserved),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn SetUmsThreadInformation(
    umsthread: *const ::std::ffi::c_void,
    umsthreadinfoclass: super::SystemServices::RTL_UMS_THREAD_INFO_CLASS,
    umsthreadinformation: *const ::std::ffi::c_void,
    umsthreadinformationlength: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetUmsThreadInformation(
                umsthread: *const ::std::ffi::c_void,
                umsthreadinfoclass: super::SystemServices::RTL_UMS_THREAD_INFO_CLASS,
                umsthreadinformation: *const ::std::ffi::c_void,
                umsthreadinformationlength: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetUmsThreadInformation(
            ::std::mem::transmute(umsthread),
            ::std::mem::transmute(umsthreadinfoclass),
            ::std::mem::transmute(umsthreadinformation),
            ::std::mem::transmute(umsthreadinformationlength),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SetWaitableTimer<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    htimer: Param0,
    lpduetime: *const i64,
    lperiod: i32,
    pfncompletionroutine: ::std::option::Option<PTIMERAPCROUTINE>,
    lpargtocompletionroutine: *const ::std::ffi::c_void,
    fresume: Param5,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetWaitableTimer(
                htimer: super::super::Foundation::HANDLE,
                lpduetime: *const i64,
                lperiod: i32,
                pfncompletionroutine: ::windows::runtime::RawPtr,
                lpargtocompletionroutine: *const ::std::ffi::c_void,
                fresume: super::super::Foundation::BOOL,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetWaitableTimer(
            htimer.into_param().abi(),
            ::std::mem::transmute(lpduetime),
            ::std::mem::transmute(lperiod),
            ::std::mem::transmute(pfncompletionroutine),
            ::std::mem::transmute(lpargtocompletionroutine),
            fresume.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn SetWaitableTimerEx<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    htimer: Param0,
    lpduetime: *const i64,
    lperiod: i32,
    pfncompletionroutine: ::std::option::Option<PTIMERAPCROUTINE>,
    lpargtocompletionroutine: *const ::std::ffi::c_void,
    wakecontext: *const super::SystemServices::REASON_CONTEXT,
    tolerabledelay: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetWaitableTimerEx(
                htimer: super::super::Foundation::HANDLE,
                lpduetime: *const i64,
                lperiod: i32,
                pfncompletionroutine: ::windows::runtime::RawPtr,
                lpargtocompletionroutine: *const ::std::ffi::c_void,
                wakecontext: *const super::SystemServices::REASON_CONTEXT,
                tolerabledelay: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetWaitableTimerEx(
            htimer.into_param().abi(),
            ::std::mem::transmute(lpduetime),
            ::std::mem::transmute(lperiod),
            ::std::mem::transmute(pfncompletionroutine),
            ::std::mem::transmute(lpargtocompletionroutine),
            ::std::mem::transmute(wakecontext),
            ::std::mem::transmute(tolerabledelay),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn Sleep(dwmilliseconds: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Sleep(dwmilliseconds: u32);
        }
        ::std::mem::transmute(Sleep(::std::mem::transmute(dwmilliseconds)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Kernel",
    feature = "Win32_System_SystemServices"
))]
pub unsafe fn SleepConditionVariableCS(
    conditionvariable: *mut super::SystemServices::RTL_CONDITION_VARIABLE,
    criticalsection: *mut super::SystemServices::RTL_CRITICAL_SECTION,
    dwmilliseconds: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SleepConditionVariableCS(
                conditionvariable: *mut super::SystemServices::RTL_CONDITION_VARIABLE,
                criticalsection: *mut super::SystemServices::RTL_CRITICAL_SECTION,
                dwmilliseconds: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SleepConditionVariableCS(
            ::std::mem::transmute(conditionvariable),
            ::std::mem::transmute(criticalsection),
            ::std::mem::transmute(dwmilliseconds),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn SleepConditionVariableSRW(
    conditionvariable: *mut super::SystemServices::RTL_CONDITION_VARIABLE,
    srwlock: *mut super::SystemServices::RTL_SRWLOCK,
    dwmilliseconds: u32,
    flags: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SleepConditionVariableSRW(
                conditionvariable: *mut super::SystemServices::RTL_CONDITION_VARIABLE,
                srwlock: *mut super::SystemServices::RTL_SRWLOCK,
                dwmilliseconds: u32,
                flags: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SleepConditionVariableSRW(
            ::std::mem::transmute(conditionvariable),
            ::std::mem::transmute(srwlock),
            ::std::mem::transmute(dwmilliseconds),
            ::std::mem::transmute(flags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SleepEx<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    dwmilliseconds: u32,
    balertable: Param1,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SleepEx(dwmilliseconds: u32, balertable: super::super::Foundation::BOOL) -> u32;
        }
        ::std::mem::transmute(SleepEx(
            ::std::mem::transmute(dwmilliseconds),
            balertable.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn StartThreadpoolIo(pio: *mut super::SystemServices::TP_IO) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StartThreadpoolIo(pio: *mut super::SystemServices::TP_IO);
        }
        ::std::mem::transmute(StartThreadpoolIo(::std::mem::transmute(pio)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn SubmitThreadpoolWork(pwk: *mut super::SystemServices::TP_WORK) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SubmitThreadpoolWork(pwk: *mut super::SystemServices::TP_WORK);
        }
        ::std::mem::transmute(SubmitThreadpoolWork(::std::mem::transmute(pwk)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SuspendThread<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hthread: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SuspendThread(hthread: super::super::Foundation::HANDLE) -> u32;
        }
        ::std::mem::transmute(SuspendThread(hthread.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn SwitchToFiber(lpfiber: *const ::std::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SwitchToFiber(lpfiber: *const ::std::ffi::c_void);
        }
        ::std::mem::transmute(SwitchToFiber(::std::mem::transmute(lpfiber)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SwitchToThread() -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SwitchToThread() -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SwitchToThread())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct THREADINFOCLASS(pub i32);
pub const ThreadIsIoPending: THREADINFOCLASS = THREADINFOCLASS(16i32);
pub const ThreadNameInformation: THREADINFOCLASS = THREADINFOCLASS(38i32);
impl ::std::convert::From<i32> for THREADINFOCLASS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for THREADINFOCLASS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct THREAD_ACCESS_RIGHTS(pub u32);
pub const THREAD_TERMINATE: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(1u32);
pub const THREAD_SUSPEND_RESUME: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(2u32);
pub const THREAD_GET_CONTEXT: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(8u32);
pub const THREAD_SET_CONTEXT: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(16u32);
pub const THREAD_SET_INFORMATION: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(32u32);
pub const THREAD_QUERY_INFORMATION: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(64u32);
pub const THREAD_SET_THREAD_TOKEN: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(128u32);
pub const THREAD_IMPERSONATE: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(256u32);
pub const THREAD_DIRECT_IMPERSONATION: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(512u32);
pub const THREAD_SET_LIMITED_INFORMATION: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(1024u32);
pub const THREAD_QUERY_LIMITED_INFORMATION: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(2048u32);
pub const THREAD_RESUME: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(4096u32);
pub const THREAD_ALL_ACCESS: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(2097151u32);
pub const THREAD_DELETE: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(65536u32);
pub const THREAD_READ_CONTROL: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(131072u32);
pub const THREAD_WRITE_DAC: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(262144u32);
pub const THREAD_WRITE_OWNER: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(524288u32);
pub const THREAD_SYNCHRONIZE: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(1048576u32);
pub const THREAD_STANDARD_RIGHTS_REQUIRED: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(983040u32);
impl ::std::convert::From<u32> for THREAD_ACCESS_RIGHTS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for THREAD_ACCESS_RIGHTS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for THREAD_ACCESS_RIGHTS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for THREAD_ACCESS_RIGHTS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for THREAD_ACCESS_RIGHTS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for THREAD_ACCESS_RIGHTS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for THREAD_ACCESS_RIGHTS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct THREAD_CREATION_FLAGS(pub u32);
pub const THREAD_CREATE_RUN_IMMEDIATELY: THREAD_CREATION_FLAGS = THREAD_CREATION_FLAGS(0u32);
pub const THREAD_CREATE_SUSPENDED: THREAD_CREATION_FLAGS = THREAD_CREATION_FLAGS(4u32);
pub const STACK_SIZE_PARAM_IS_A_RESERVATION: THREAD_CREATION_FLAGS =
    THREAD_CREATION_FLAGS(65536u32);
impl ::std::convert::From<u32> for THREAD_CREATION_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for THREAD_CREATION_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for THREAD_CREATION_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for THREAD_CREATION_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for THREAD_CREATION_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for THREAD_CREATION_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for THREAD_CREATION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct THREAD_INFORMATION_CLASS(pub i32);
pub const ThreadMemoryPriority: THREAD_INFORMATION_CLASS = THREAD_INFORMATION_CLASS(0i32);
pub const ThreadAbsoluteCpuPriority: THREAD_INFORMATION_CLASS = THREAD_INFORMATION_CLASS(1i32);
pub const ThreadDynamicCodePolicy: THREAD_INFORMATION_CLASS = THREAD_INFORMATION_CLASS(2i32);
pub const ThreadPowerThrottling: THREAD_INFORMATION_CLASS = THREAD_INFORMATION_CLASS(3i32);
pub const ThreadInformationClassMax: THREAD_INFORMATION_CLASS = THREAD_INFORMATION_CLASS(4i32);
impl ::std::convert::From<i32> for THREAD_INFORMATION_CLASS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for THREAD_INFORMATION_CLASS {
    type Abi = Self;
    type DefaultType = Self;
}
pub const THREAD_POWER_THROTTLING_CURRENT_VERSION: u32 = 1u32;
pub const THREAD_POWER_THROTTLING_EXECUTION_SPEED: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct THREAD_POWER_THROTTLING_STATE {
    pub Version: u32,
    pub ControlMask: u32,
    pub StateMask: u32,
}
impl THREAD_POWER_THROTTLING_STATE {}
impl ::std::default::Default for THREAD_POWER_THROTTLING_STATE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for THREAD_POWER_THROTTLING_STATE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("THREAD_POWER_THROTTLING_STATE")
            .field("Version", &self.Version)
            .field("ControlMask", &self.ControlMask)
            .field("StateMask", &self.StateMask)
            .finish()
    }
}
impl ::std::cmp::PartialEq for THREAD_POWER_THROTTLING_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version
            && self.ControlMask == other.ControlMask
            && self.StateMask == other.StateMask
    }
}
impl ::std::cmp::Eq for THREAD_POWER_THROTTLING_STATE {}
unsafe impl ::windows::runtime::Abi for THREAD_POWER_THROTTLING_STATE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const THREAD_POWER_THROTTLING_VALID_FLAGS: u32 = 1u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct THREAD_PRIORITY(pub i32);
pub const THREAD_MODE_BACKGROUND_BEGIN: THREAD_PRIORITY = THREAD_PRIORITY(65536i32);
pub const THREAD_MODE_BACKGROUND_END: THREAD_PRIORITY = THREAD_PRIORITY(131072i32);
pub const THREAD_PRIORITY_ABOVE_NORMAL: THREAD_PRIORITY = THREAD_PRIORITY(1i32);
pub const THREAD_PRIORITY_BELOW_NORMAL: THREAD_PRIORITY = THREAD_PRIORITY(-1i32);
pub const THREAD_PRIORITY_HIGHEST: THREAD_PRIORITY = THREAD_PRIORITY(2i32);
pub const THREAD_PRIORITY_IDLE: THREAD_PRIORITY = THREAD_PRIORITY(-15i32);
pub const THREAD_PRIORITY_MIN: THREAD_PRIORITY = THREAD_PRIORITY(-2i32);
pub const THREAD_PRIORITY_LOWEST: THREAD_PRIORITY = THREAD_PRIORITY(-2i32);
pub const THREAD_PRIORITY_NORMAL: THREAD_PRIORITY = THREAD_PRIORITY(0i32);
pub const THREAD_PRIORITY_TIME_CRITICAL: THREAD_PRIORITY = THREAD_PRIORITY(15i32);
impl ::std::convert::From<i32> for THREAD_PRIORITY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for THREAD_PRIORITY {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn TerminateProcess<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hprocess: Param0,
    uexitcode: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TerminateProcess(
                hprocess: super::super::Foundation::HANDLE,
                uexitcode: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(TerminateProcess(
            hprocess.into_param().abi(),
            ::std::mem::transmute(uexitcode),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn TerminateThread<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hthread: Param0,
    dwexitcode: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TerminateThread(
                hthread: super::super::Foundation::HANDLE,
                dwexitcode: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(TerminateThread(
            hthread.into_param().abi(),
            ::std::mem::transmute(dwexitcode),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(
    :: std :: clone :: Clone,
    :: std :: marker :: Copy,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
)]
#[repr(transparent)]
pub struct TimerQueueHandle(pub isize);
impl ::std::default::Default for TimerQueueHandle {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for TimerQueueHandle {}
unsafe impl ::windows::runtime::Abi for TimerQueueHandle {
    type Abi = Self;
    type DefaultType = Self;
}
pub unsafe fn TlsAlloc() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TlsAlloc() -> u32;
        }
        ::std::mem::transmute(TlsAlloc())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn TlsFree(dwtlsindex: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TlsFree(dwtlsindex: u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(TlsFree(::std::mem::transmute(dwtlsindex)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn TlsGetValue(dwtlsindex: u32) -> *mut ::std::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TlsGetValue(dwtlsindex: u32) -> *mut ::std::ffi::c_void;
        }
        ::std::mem::transmute(TlsGetValue(::std::mem::transmute(dwtlsindex)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn TlsSetValue(
    dwtlsindex: u32,
    lptlsvalue: *const ::std::ffi::c_void,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TlsSetValue(
                dwtlsindex: u32,
                lptlsvalue: *const ::std::ffi::c_void,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(TlsSetValue(
            ::std::mem::transmute(dwtlsindex),
            ::std::mem::transmute(lptlsvalue),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn TryAcquireSRWLockExclusive(
    srwlock: *mut super::SystemServices::RTL_SRWLOCK,
) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TryAcquireSRWLockExclusive(
                srwlock: *mut super::SystemServices::RTL_SRWLOCK,
            ) -> super::super::Foundation::BOOLEAN;
        }
        ::std::mem::transmute(TryAcquireSRWLockExclusive(::std::mem::transmute(srwlock)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn TryAcquireSRWLockShared(
    srwlock: *mut super::SystemServices::RTL_SRWLOCK,
) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TryAcquireSRWLockShared(
                srwlock: *mut super::SystemServices::RTL_SRWLOCK,
            ) -> super::super::Foundation::BOOLEAN;
        }
        ::std::mem::transmute(TryAcquireSRWLockShared(::std::mem::transmute(srwlock)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Kernel",
    feature = "Win32_System_SystemServices"
))]
pub unsafe fn TryEnterCriticalSection(
    lpcriticalsection: *mut super::SystemServices::RTL_CRITICAL_SECTION,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TryEnterCriticalSection(
                lpcriticalsection: *mut super::SystemServices::RTL_CRITICAL_SECTION,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(TryEnterCriticalSection(::std::mem::transmute(
            lpcriticalsection,
        )))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn TrySubmitThreadpoolCallback(
    pfns: ::std::option::Option<super::SystemServices::PTP_SIMPLE_CALLBACK>,
    pv: *mut ::std::ffi::c_void,
    pcbe: *const super::SystemServices::TP_CALLBACK_ENVIRON_V3,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TrySubmitThreadpoolCallback(
                pfns: ::windows::runtime::RawPtr,
                pv: *mut ::std::ffi::c_void,
                pcbe: *const ::std::mem::ManuallyDrop<
                    super::SystemServices::TP_CALLBACK_ENVIRON_V3,
                >,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(TrySubmitThreadpoolCallback(
            ::std::mem::transmute(pfns),
            ::std::mem::transmute(pv),
            ::std::mem::transmute(pcbe),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_System_SystemServices")]
pub struct UMS_SCHEDULER_STARTUP_INFO {
    pub UmsVersion: u32,
    pub CompletionList: *mut ::std::ffi::c_void,
    pub SchedulerProc: ::std::option::Option<super::SystemServices::PRTL_UMS_SCHEDULER_ENTRY_POINT>,
    pub SchedulerParam: *mut ::std::ffi::c_void,
}
#[cfg(feature = "Win32_System_SystemServices")]
impl UMS_SCHEDULER_STARTUP_INFO {}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::default::Default for UMS_SCHEDULER_STARTUP_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::fmt::Debug for UMS_SCHEDULER_STARTUP_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("UMS_SCHEDULER_STARTUP_INFO")
            .field("UmsVersion", &self.UmsVersion)
            .field("CompletionList", &self.CompletionList)
            .field("SchedulerParam", &self.SchedulerParam)
            .finish()
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::PartialEq for UMS_SCHEDULER_STARTUP_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.UmsVersion == other.UmsVersion
            && self.CompletionList == other.CompletionList
            && self.SchedulerProc.map(|f| f as usize) == other.SchedulerProc.map(|f| f as usize)
            && self.SchedulerParam == other.SchedulerParam
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::Eq for UMS_SCHEDULER_STARTUP_INFO {}
#[cfg(feature = "Win32_System_SystemServices")]
unsafe impl ::windows::runtime::Abi for UMS_SCHEDULER_STARTUP_INFO {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct UMS_SYSTEM_THREAD_INFORMATION {
    pub UmsVersion: u32,
    pub Anonymous: UMS_SYSTEM_THREAD_INFORMATION_0,
}
impl UMS_SYSTEM_THREAD_INFORMATION {}
impl ::std::default::Default for UMS_SYSTEM_THREAD_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for UMS_SYSTEM_THREAD_INFORMATION {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for UMS_SYSTEM_THREAD_INFORMATION {}
unsafe impl ::windows::runtime::Abi for UMS_SYSTEM_THREAD_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union UMS_SYSTEM_THREAD_INFORMATION_0 {
    pub Anonymous: UMS_SYSTEM_THREAD_INFORMATION_0_0,
    pub ThreadUmsFlags: u32,
}
impl UMS_SYSTEM_THREAD_INFORMATION_0 {}
impl ::std::default::Default for UMS_SYSTEM_THREAD_INFORMATION_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for UMS_SYSTEM_THREAD_INFORMATION_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for UMS_SYSTEM_THREAD_INFORMATION_0 {}
unsafe impl ::windows::runtime::Abi for UMS_SYSTEM_THREAD_INFORMATION_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct UMS_SYSTEM_THREAD_INFORMATION_0_0 {
    pub _bitfield: u32,
}
impl UMS_SYSTEM_THREAD_INFORMATION_0_0 {}
impl ::std::default::Default for UMS_SYSTEM_THREAD_INFORMATION_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for UMS_SYSTEM_THREAD_INFORMATION_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::std::cmp::PartialEq for UMS_SYSTEM_THREAD_INFORMATION_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for UMS_SYSTEM_THREAD_INFORMATION_0_0 {}
unsafe impl ::windows::runtime::Abi for UMS_SYSTEM_THREAD_INFORMATION_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn UmsThreadYield(
    schedulerparam: *const ::std::ffi::c_void,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UmsThreadYield(
                schedulerparam: *const ::std::ffi::c_void,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(UmsThreadYield(::std::mem::transmute(schedulerparam)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn UnregisterWait<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    waithandle: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UnregisterWait(
                waithandle: super::super::Foundation::HANDLE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(UnregisterWait(waithandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn UnregisterWaitEx<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    waithandle: Param0,
    completionevent: Param1,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UnregisterWaitEx(
                waithandle: super::super::Foundation::HANDLE,
                completionevent: super::super::Foundation::HANDLE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(UnregisterWaitEx(
            waithandle.into_param().abi(),
            completionevent.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn UpdateProcThreadAttribute<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, LPPROC_THREAD_ATTRIBUTE_LIST>,
>(
    lpattributelist: Param0,
    dwflags: u32,
    attribute: usize,
    lpvalue: *const ::std::ffi::c_void,
    cbsize: usize,
    lppreviousvalue: *mut ::std::ffi::c_void,
    lpreturnsize: *const usize,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UpdateProcThreadAttribute(
                lpattributelist: LPPROC_THREAD_ATTRIBUTE_LIST,
                dwflags: u32,
                attribute: usize,
                lpvalue: *const ::std::ffi::c_void,
                cbsize: usize,
                lppreviousvalue: *mut ::std::ffi::c_void,
                lpreturnsize: *const usize,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(UpdateProcThreadAttribute(
            lpattributelist.into_param().abi(),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(attribute),
            ::std::mem::transmute(lpvalue),
            ::std::mem::transmute(cbsize),
            ::std::mem::transmute(lppreviousvalue),
            ::std::mem::transmute(lpreturnsize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const WAIT_ABANDONED: u32 = 128u32;
pub const WAIT_ABANDONED_0: u32 = 128u32;
pub const WAIT_IO_COMPLETION: u32 = 192u32;
pub const WAIT_OBJECT_0: u32 = 0u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct WORKER_THREAD_FLAGS(pub u32);
pub const WT_EXECUTEDEFAULT: WORKER_THREAD_FLAGS = WORKER_THREAD_FLAGS(0u32);
pub const WT_EXECUTEINIOTHREAD: WORKER_THREAD_FLAGS = WORKER_THREAD_FLAGS(1u32);
pub const WT_EXECUTEINPERSISTENTTHREAD: WORKER_THREAD_FLAGS = WORKER_THREAD_FLAGS(128u32);
pub const WT_EXECUTEINWAITTHREAD: WORKER_THREAD_FLAGS = WORKER_THREAD_FLAGS(4u32);
pub const WT_EXECUTELONGFUNCTION: WORKER_THREAD_FLAGS = WORKER_THREAD_FLAGS(16u32);
pub const WT_EXECUTEONLYONCE: WORKER_THREAD_FLAGS = WORKER_THREAD_FLAGS(8u32);
pub const WT_TRANSFER_IMPERSONATION: WORKER_THREAD_FLAGS = WORKER_THREAD_FLAGS(256u32);
pub const WT_EXECUTEINTIMERTHREAD: WORKER_THREAD_FLAGS = WORKER_THREAD_FLAGS(32u32);
impl ::std::convert::From<u32> for WORKER_THREAD_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WORKER_THREAD_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for WORKER_THREAD_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for WORKER_THREAD_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for WORKER_THREAD_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for WORKER_THREAD_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for WORKER_THREAD_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn WaitForInputIdle<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hprocess: Param0,
    dwmilliseconds: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WaitForInputIdle(
                hprocess: super::super::Foundation::HANDLE,
                dwmilliseconds: u32,
            ) -> u32;
        }
        ::std::mem::transmute(WaitForInputIdle(
            hprocess.into_param().abi(),
            ::std::mem::transmute(dwmilliseconds),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn WaitForMultipleObjects<
    'a,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    ncount: u32,
    lphandles: *const super::super::Foundation::HANDLE,
    bwaitall: Param2,
    dwmilliseconds: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WaitForMultipleObjects(
                ncount: u32,
                lphandles: *const super::super::Foundation::HANDLE,
                bwaitall: super::super::Foundation::BOOL,
                dwmilliseconds: u32,
            ) -> u32;
        }
        ::std::mem::transmute(WaitForMultipleObjects(
            ::std::mem::transmute(ncount),
            ::std::mem::transmute(lphandles),
            bwaitall.into_param().abi(),
            ::std::mem::transmute(dwmilliseconds),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn WaitForMultipleObjectsEx<
    'a,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    ncount: u32,
    lphandles: *const super::super::Foundation::HANDLE,
    bwaitall: Param2,
    dwmilliseconds: u32,
    balertable: Param4,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WaitForMultipleObjectsEx(
                ncount: u32,
                lphandles: *const super::super::Foundation::HANDLE,
                bwaitall: super::super::Foundation::BOOL,
                dwmilliseconds: u32,
                balertable: super::super::Foundation::BOOL,
            ) -> u32;
        }
        ::std::mem::transmute(WaitForMultipleObjectsEx(
            ::std::mem::transmute(ncount),
            ::std::mem::transmute(lphandles),
            bwaitall.into_param().abi(),
            ::std::mem::transmute(dwmilliseconds),
            balertable.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn WaitForSingleObject<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hhandle: Param0,
    dwmilliseconds: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WaitForSingleObject(
                hhandle: super::super::Foundation::HANDLE,
                dwmilliseconds: u32,
            ) -> u32;
        }
        ::std::mem::transmute(WaitForSingleObject(
            hhandle.into_param().abi(),
            ::std::mem::transmute(dwmilliseconds),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn WaitForSingleObjectEx<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    hhandle: Param0,
    dwmilliseconds: u32,
    balertable: Param2,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WaitForSingleObjectEx(
                hhandle: super::super::Foundation::HANDLE,
                dwmilliseconds: u32,
                balertable: super::super::Foundation::BOOL,
            ) -> u32;
        }
        ::std::mem::transmute(WaitForSingleObjectEx(
            hhandle.into_param().abi(),
            ::std::mem::transmute(dwmilliseconds),
            balertable.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn WaitForThreadpoolIoCallbacks<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    pio: *mut super::SystemServices::TP_IO,
    fcancelpendingcallbacks: Param1,
) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WaitForThreadpoolIoCallbacks(
                pio: *mut super::SystemServices::TP_IO,
                fcancelpendingcallbacks: super::super::Foundation::BOOL,
            );
        }
        ::std::mem::transmute(WaitForThreadpoolIoCallbacks(
            ::std::mem::transmute(pio),
            fcancelpendingcallbacks.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn WaitForThreadpoolTimerCallbacks<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    pti: *mut super::SystemServices::TP_TIMER,
    fcancelpendingcallbacks: Param1,
) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WaitForThreadpoolTimerCallbacks(
                pti: *mut super::SystemServices::TP_TIMER,
                fcancelpendingcallbacks: super::super::Foundation::BOOL,
            );
        }
        ::std::mem::transmute(WaitForThreadpoolTimerCallbacks(
            ::std::mem::transmute(pti),
            fcancelpendingcallbacks.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn WaitForThreadpoolWaitCallbacks<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    pwa: *mut super::SystemServices::TP_WAIT,
    fcancelpendingcallbacks: Param1,
) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WaitForThreadpoolWaitCallbacks(
                pwa: *mut super::SystemServices::TP_WAIT,
                fcancelpendingcallbacks: super::super::Foundation::BOOL,
            );
        }
        ::std::mem::transmute(WaitForThreadpoolWaitCallbacks(
            ::std::mem::transmute(pwa),
            fcancelpendingcallbacks.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn WaitForThreadpoolWorkCallbacks<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    pwk: *mut super::SystemServices::TP_WORK,
    fcancelpendingcallbacks: Param1,
) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WaitForThreadpoolWorkCallbacks(
                pwk: *mut super::SystemServices::TP_WORK,
                fcancelpendingcallbacks: super::super::Foundation::BOOL,
            );
        }
        ::std::mem::transmute(WaitForThreadpoolWorkCallbacks(
            ::std::mem::transmute(pwk),
            fcancelpendingcallbacks.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn WaitOnAddress(
    address: *const ::std::ffi::c_void,
    compareaddress: *const ::std::ffi::c_void,
    addresssize: usize,
    dwmilliseconds: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WaitOnAddress(
                address: *const ::std::ffi::c_void,
                compareaddress: *const ::std::ffi::c_void,
                addresssize: usize,
                dwmilliseconds: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(WaitOnAddress(
            ::std::mem::transmute(address),
            ::std::mem::transmute(compareaddress),
            ::std::mem::transmute(addresssize),
            ::std::mem::transmute(dwmilliseconds),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn WakeAllConditionVariable(
    conditionvariable: *mut super::SystemServices::RTL_CONDITION_VARIABLE,
) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WakeAllConditionVariable(
                conditionvariable: *mut super::SystemServices::RTL_CONDITION_VARIABLE,
            );
        }
        ::std::mem::transmute(WakeAllConditionVariable(::std::mem::transmute(
            conditionvariable,
        )))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn WakeByAddressAll(address: *const ::std::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WakeByAddressAll(address: *const ::std::ffi::c_void);
        }
        ::std::mem::transmute(WakeByAddressAll(::std::mem::transmute(address)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn WakeByAddressSingle(address: *const ::std::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WakeByAddressSingle(address: *const ::std::ffi::c_void);
        }
        ::std::mem::transmute(WakeByAddressSingle(::std::mem::transmute(address)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn WakeConditionVariable(
    conditionvariable: *mut super::SystemServices::RTL_CONDITION_VARIABLE,
) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WakeConditionVariable(
                conditionvariable: *mut super::SystemServices::RTL_CONDITION_VARIABLE,
            );
        }
        ::std::mem::transmute(WakeConditionVariable(::std::mem::transmute(
            conditionvariable,
        )))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn WinExec<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lpcmdline: Param0,
    ucmdshow: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinExec(lpcmdline: super::super::Foundation::PSTR, ucmdshow: u32) -> u32;
        }
        ::std::mem::transmute(WinExec(
            lpcmdline.into_param().abi(),
            ::std::mem::transmute(ucmdshow),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn Wow64SetThreadDefaultGuestMachine(machine: u16) -> u16 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Wow64SetThreadDefaultGuestMachine(machine: u16) -> u16;
        }
        ::std::mem::transmute(Wow64SetThreadDefaultGuestMachine(::std::mem::transmute(
            machine,
        )))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn Wow64SuspendThread<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hthread: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Wow64SuspendThread(hthread: super::super::Foundation::HANDLE) -> u32;
        }
        ::std::mem::transmute(Wow64SuspendThread(hthread.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
