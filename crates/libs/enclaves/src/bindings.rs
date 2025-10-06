windows_link::link!("vertdll.dll" "system" fn AcquireSRWLockExclusive(srwlock : *mut SRWLOCK));
windows_link::link!("vertdll.dll" "system" fn AcquireSRWLockShared(srwlock : *mut SRWLOCK));
windows_link::link!("vertdll.dll" "system" fn CallEnclave(lproutine : isize, lpparameter : *const core::ffi::c_void, fwaitforthread : BOOL, lpreturnvalue : *mut *mut core::ffi::c_void) -> BOOL);
windows_link::link!("vertdll.dll" "system" fn DeleteCriticalSection(lpcriticalsection : *mut CRITICAL_SECTION));
windows_link::link!("vertdll.dll" "system" fn DeleteSynchronizationBarrier(lpbarrier : *mut SYNCHRONIZATION_BARRIER) -> BOOL);
windows_link::link!("vertdll.dll" "system" fn DisableThreadLibraryCalls(hlibmodule : HMODULE) -> BOOL);
windows_link::link!("vertdll.dll" "system" fn EnclaveGetAttestationReport(enclavedata : *const u8, report : *mut core::ffi::c_void, buffersize : u32, outputsize : *mut u32) -> HRESULT);
windows_link::link!("vertdll.dll" "system" fn EnclaveGetEnclaveInformation(informationsize : u32, enclaveinformation : *mut ENCLAVE_INFORMATION) -> HRESULT);
windows_link::link!("vertdll.dll" "system" fn EnclaveSealData(datatoencrypt : *const core::ffi::c_void, datatoencryptsize : u32, identitypolicy : ENCLAVE_SEALING_IDENTITY_POLICY, runtimepolicy : u32, protectedblob : *mut core::ffi::c_void, buffersize : u32, protectedblobsize : *mut u32) -> HRESULT);
windows_link::link!("vertdll.dll" "system" fn EnclaveUnsealData(protectedblob : *const core::ffi::c_void, protectedblobsize : u32, decrypteddata : *mut core::ffi::c_void, buffersize : u32, decrypteddatasize : *mut u32, sealingidentity : *mut ENCLAVE_IDENTITY, unsealingflags : *mut u32) -> HRESULT);
windows_link::link!("vertdll.dll" "system" fn EnclaveVerifyAttestationReport(enclavetype : u32, report : *const core::ffi::c_void, reportsize : u32) -> HRESULT);
windows_link::link!("vertdll.dll" "system" fn EnterCriticalSection(lpcriticalsection : *mut CRITICAL_SECTION));
windows_link::link!("vertdll.dll" "system" fn EnterSynchronizationBarrier(lpbarrier : *mut SYNCHRONIZATION_BARRIER, dwflags : u32) -> BOOL);
windows_link::link!("vertdll.dll" "system" fn GetCurrentProcess() -> HANDLE);
windows_link::link!("vertdll.dll" "system" fn GetCurrentThread() -> HANDLE);
windows_link::link!("vertdll.dll" "system" fn GetCurrentThreadId() -> u32);
windows_link::link!("vertdll.dll" "system" fn GetLastError() -> WIN32_ERROR);
windows_link::link!("vertdll.dll" "system" fn GetModuleHandleExW(dwflags : u32, lpmodulename : PCWSTR, phmodule : *mut HMODULE) -> BOOL);
windows_link::link!("vertdll.dll" "system" fn GetProcAddress(hmodule : HMODULE, lpprocname : PCSTR) -> FARPROC);
windows_link::link!("vertdll.dll" "system" fn GetProcessHeap() -> HANDLE);
windows_link::link!("vertdll.dll" "system" fn GetProcessHeaps(numberofheaps : u32, processheaps : *mut HANDLE) -> u32);
windows_link::link!("vertdll.dll" "system" fn HeapAlloc(hheap : HANDLE, dwflags : HEAP_FLAGS, dwbytes : usize) -> *mut core::ffi::c_void);
windows_link::link!("vertdll.dll" "system" fn HeapCompact(hheap : HANDLE, dwflags : HEAP_FLAGS) -> usize);
windows_link::link!("vertdll.dll" "system" fn HeapCreate(floptions : HEAP_FLAGS, dwinitialsize : usize, dwmaximumsize : usize) -> HANDLE);
windows_link::link!("vertdll.dll" "system" fn HeapDestroy(hheap : HANDLE) -> BOOL);
windows_link::link!("vertdll.dll" "system" fn HeapFree(hheap : HANDLE, dwflags : HEAP_FLAGS, lpmem : *const core::ffi::c_void) -> BOOL);
windows_link::link!("vertdll.dll" "system" fn HeapLock(hheap : HANDLE) -> BOOL);
windows_link::link!("vertdll.dll" "system" fn HeapReAlloc(hheap : HANDLE, dwflags : HEAP_FLAGS, lpmem : *const core::ffi::c_void, dwbytes : usize) -> *mut core::ffi::c_void);
windows_link::link!("vertdll.dll" "system" fn HeapSize(hheap : HANDLE, dwflags : HEAP_FLAGS, lpmem : *const core::ffi::c_void) -> usize);
windows_link::link!("vertdll.dll" "system" fn HeapUnlock(hheap : HANDLE) -> BOOL);
windows_link::link!("vertdll.dll" "system" fn InitializeConditionVariable(conditionvariable : *mut CONDITION_VARIABLE));
windows_link::link!("vertdll.dll" "system" fn InitializeCriticalSection(lpcriticalsection : *mut CRITICAL_SECTION));
windows_link::link!("vertdll.dll" "system" fn InitializeCriticalSectionAndSpinCount(lpcriticalsection : *mut CRITICAL_SECTION, dwspincount : u32) -> BOOL);
windows_link::link!("vertdll.dll" "system" fn InitializeCriticalSectionEx(lpcriticalsection : *mut CRITICAL_SECTION, dwspincount : u32, flags : u32) -> BOOL);
windows_link::link!("vertdll.dll" "system" fn InitializeSListHead(listhead : *mut SLIST_HEADER));
windows_link::link!("vertdll.dll" "system" fn InitializeSRWLock(srwlock : *mut SRWLOCK));
windows_link::link!("vertdll.dll" "system" fn InitializeSynchronizationBarrier(lpbarrier : *mut SYNCHRONIZATION_BARRIER, ltotalthreads : i32, lspincount : i32) -> BOOL);
windows_link::link!("vertdll.dll" "system" fn InterlockedFlushSList(listhead : *mut SLIST_HEADER) -> *mut SLIST_ENTRY);
windows_link::link!("vertdll.dll" "system" fn InterlockedPopEntrySList(listhead : *mut SLIST_HEADER) -> *mut SLIST_ENTRY);
windows_link::link!("vertdll.dll" "system" fn InterlockedPushEntrySList(listhead : *mut SLIST_HEADER, listentry : *mut SLIST_ENTRY) -> *mut SLIST_ENTRY);
windows_link::link!("vertdll.dll" "system" fn InterlockedPushListSListEx(listhead : *mut SLIST_HEADER, list : *mut SLIST_ENTRY, listend : *mut SLIST_ENTRY, count : u32) -> *mut SLIST_ENTRY);
windows_link::link!("vertdll.dll" "system" fn IsProcessorFeaturePresent(processorfeature : PROCESSOR_FEATURE_ID) -> BOOL);
windows_link::link!("vertdll.dll" "system" fn LeaveCriticalSection(lpcriticalsection : *mut CRITICAL_SECTION));
windows_link::link!("vertdll.dll" "system" fn MultiByteToWideChar(codepage : u32, dwflags : MULTI_BYTE_TO_WIDE_CHAR_FLAGS, lpmultibytestr : PCSTR, cbmultibyte : i32, lpwidecharstr : PWSTR, cchwidechar : i32) -> i32);
windows_link::link!("vertdll.dll" "system" fn NtTerminateProcess(processhandle : HANDLE, exitstatus : NTSTATUS) -> NTSTATUS);
windows_link::link!("vertdll.dll" "system" fn OutputDebugStringW(lpoutputstring : PCWSTR));
windows_link::link!("vertdll.dll" "system" fn QueryDepthSList(listhead : *const SLIST_HEADER) -> u16);
windows_link::link!("vertdll.dll" "system" fn RaiseException(dwexceptioncode : u32, dwexceptionflags : u32, nnumberofarguments : u32, lparguments : *const usize));
windows_link::link!("vertdll.dll" "system" fn ReleaseSRWLockExclusive(srwlock : *mut SRWLOCK));
windows_link::link!("vertdll.dll" "system" fn ReleaseSRWLockShared(srwlock : *mut SRWLOCK));
windows_link::link!("vertdll.dll" "system" fn RtlCaptureContext(contextrecord : *mut CONTEXT));
windows_link::link!("vertdll.dll" "system" fn RtlGetSystemGlobalData(dataid : RTL_SYSTEM_GLOBAL_DATA_ID, buffer : *mut core::ffi::c_void, size : u32) -> u32);
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
windows_link::link!("vertdll.dll" "system" fn RtlLookupFunctionEntry(controlpc : u64, imagebase : *mut u64, historytable : *mut UNWIND_HISTORY_TABLE) -> *mut IMAGE_RUNTIME_FUNCTION_ENTRY);
#[cfg(target_arch = "aarch64")]
windows_link::link!("vertdll.dll" "system" fn RtlLookupFunctionEntry(controlpc : usize, imagebase : *mut usize, historytable : *mut UNWIND_HISTORY_TABLE) -> *mut IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY);
windows_link::link!("vertdll.dll" "system" fn RtlPcToFileHeader(pcvalue : *const core::ffi::c_void, baseofimage : *mut *mut core::ffi::c_void) -> *mut core::ffi::c_void);
windows_link::link!("vertdll.dll" "system" fn RtlTimeFieldsToTime(timefields : *const TIME_FIELDS, time : *mut i64) -> bool);
windows_link::link!("vertdll.dll" "system" fn RtlUnwind(targetframe : *const core::ffi::c_void, targetip : *const core::ffi::c_void, exceptionrecord : *const EXCEPTION_RECORD, returnvalue : *const core::ffi::c_void));
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm64ec",
    target_arch = "x86_64"
))]
windows_link::link!("vertdll.dll" "system" fn RtlUnwindEx(targetframe : *const core::ffi::c_void, targetip : *const core::ffi::c_void, exceptionrecord : *const EXCEPTION_RECORD, returnvalue : *const core::ffi::c_void, contextrecord : *const CONTEXT, historytable : *const UNWIND_HISTORY_TABLE));
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
windows_link::link!("vertdll.dll" "system" fn RtlVirtualUnwind(handlertype : RTL_VIRTUAL_UNWIND_HANDLER_TYPE, imagebase : u64, controlpc : u64, functionentry : *const IMAGE_RUNTIME_FUNCTION_ENTRY, contextrecord : *mut CONTEXT, handlerdata : *mut *mut core::ffi::c_void, establisherframe : *mut u64, contextpointers : *mut KNONVOLATILE_CONTEXT_POINTERS) -> EXCEPTION_ROUTINE);
#[cfg(target_arch = "aarch64")]
windows_link::link!("vertdll.dll" "system" fn RtlVirtualUnwind(handlertype : RTL_VIRTUAL_UNWIND_HANDLER_TYPE, imagebase : usize, controlpc : usize, functionentry : *const IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY, contextrecord : *mut CONTEXT, handlerdata : *mut *mut core::ffi::c_void, establisherframe : *mut usize, contextpointers : *mut KNONVOLATILE_CONTEXT_POINTERS) -> EXCEPTION_ROUTINE);
windows_link::link!("vertdll.dll" "system" fn SetCriticalSectionSpinCount(lpcriticalsection : *mut CRITICAL_SECTION, dwspincount : u32) -> u32);
windows_link::link!("vertdll.dll" "system" fn SetLastError(dwerrcode : WIN32_ERROR));
windows_link::link!("vertdll.dll" "system" fn SetThreadStackGuarantee(stacksizeinbytes : *mut u32) -> BOOL);
windows_link::link!("vertdll.dll" "system" fn SetUnhandledExceptionFilter(lptoplevelexceptionfilter : LPTOP_LEVEL_EXCEPTION_FILTER) -> LPTOP_LEVEL_EXCEPTION_FILTER);
windows_link::link!("vertdll.dll" "system" fn SleepConditionVariableCS(conditionvariable : *mut CONDITION_VARIABLE, criticalsection : *mut CRITICAL_SECTION, dwmilliseconds : u32) -> BOOL);
windows_link::link!("vertdll.dll" "system" fn SleepConditionVariableSRW(conditionvariable : *mut CONDITION_VARIABLE, srwlock : *mut SRWLOCK, dwmilliseconds : u32, flags : u32) -> BOOL);
windows_link::link!("vertdll.dll" "system" fn TerminateEnclave(lpaddress : *const core::ffi::c_void, fwait : BOOL) -> BOOL);
windows_link::link!("vertdll.dll" "system" fn TerminateProcess(hprocess : HANDLE, uexitcode : u32) -> BOOL);
windows_link::link!("vertdll.dll" "system" fn TlsAlloc() -> u32);
windows_link::link!("vertdll.dll" "system" fn TlsFree(dwtlsindex : u32) -> BOOL);
windows_link::link!("vertdll.dll" "system" fn TlsGetValue(dwtlsindex : u32) -> *mut core::ffi::c_void);
windows_link::link!("vertdll.dll" "system" fn TlsSetValue(dwtlsindex : u32, lptlsvalue : *const core::ffi::c_void) -> BOOL);
windows_link::link!("vertdll.dll" "system" fn TryAcquireSRWLockExclusive(srwlock : *mut SRWLOCK) -> bool);
windows_link::link!("vertdll.dll" "system" fn TryAcquireSRWLockShared(srwlock : *mut SRWLOCK) -> bool);
windows_link::link!("vertdll.dll" "system" fn TryEnterCriticalSection(lpcriticalsection : *mut CRITICAL_SECTION) -> BOOL);
windows_link::link!("vertdll.dll" "system" fn VirtualAlloc(lpaddress : *const core::ffi::c_void, dwsize : usize, flallocationtype : VIRTUAL_ALLOCATION_TYPE, flprotect : PAGE_PROTECTION_FLAGS) -> *mut core::ffi::c_void);
windows_link::link!("vertdll.dll" "system" fn VirtualFree(lpaddress : *mut core::ffi::c_void, dwsize : usize, dwfreetype : VIRTUAL_FREE_TYPE) -> BOOL);
windows_link::link!("vertdll.dll" "system" fn VirtualProtect(lpaddress : *const core::ffi::c_void, dwsize : usize, flnewprotect : PAGE_PROTECTION_FLAGS, lpfloldprotect : *mut PAGE_PROTECTION_FLAGS) -> BOOL);
windows_link::link!("vertdll.dll" "system" fn VirtualQuery(lpaddress : *const core::ffi::c_void, lpbuffer : *mut MEMORY_BASIC_INFORMATION, dwlength : usize) -> usize);
windows_link::link!("vertdll.dll" "system" fn WaitOnAddress(address : *const core::ffi::c_void, compareaddress : *const core::ffi::c_void, addresssize : usize, dwmilliseconds : u32) -> BOOL);
windows_link::link!("vertdll.dll" "system" fn WakeAllConditionVariable(conditionvariable : *mut CONDITION_VARIABLE));
windows_link::link!("vertdll.dll" "system" fn WakeByAddressAll(address : *const core::ffi::c_void));
windows_link::link!("vertdll.dll" "system" fn WakeByAddressSingle(address : *const core::ffi::c_void));
windows_link::link!("vertdll.dll" "system" fn WakeConditionVariable(conditionvariable : *mut CONDITION_VARIABLE));
windows_link::link!("vertdll.dll" "system" fn WideCharToMultiByte(codepage : u32, dwflags : u32, lpwidecharstr : PCWSTR, cchwidechar : i32, lpmultibytestr : PSTR, cbmultibyte : i32, lpdefaultchar : PCSTR, lpuseddefaultchar : *mut BOOL) -> i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub union ARM64_NT_NEON128 {
    pub Anonymous: ARM64_NT_NEON128_0,
    pub D: [f64; 2],
    pub S: [f32; 4],
    pub H: [u16; 8],
    pub B: [u8; 16],
}
impl Default for ARM64_NT_NEON128 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ARM64_NT_NEON128_0 {
    pub Low: u64,
    pub High: i64,
}
pub type BOOL = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CONDITION_VARIABLE {
    pub Ptr: *mut core::ffi::c_void,
}
impl Default for CONDITION_VARIABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct CONTEXT {
    pub ContextFlags: CONTEXT_FLAGS,
    pub Dr0: u32,
    pub Dr1: u32,
    pub Dr2: u32,
    pub Dr3: u32,
    pub Dr6: u32,
    pub Dr7: u32,
    pub FloatSave: FLOATING_SAVE_AREA,
    pub SegGs: u32,
    pub SegFs: u32,
    pub SegEs: u32,
    pub SegDs: u32,
    pub Edi: u32,
    pub Esi: u32,
    pub Ebx: u32,
    pub Edx: u32,
    pub Ecx: u32,
    pub Eax: u32,
    pub Ebp: u32,
    pub Eip: u32,
    pub SegCs: u32,
    pub EFlags: u32,
    pub Esp: u32,
    pub SegSs: u32,
    pub ExtendedRegisters: [u8; 512],
}
#[cfg(target_arch = "x86")]
impl Default for CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct CONTEXT {
    pub P1Home: u64,
    pub P2Home: u64,
    pub P3Home: u64,
    pub P4Home: u64,
    pub P5Home: u64,
    pub P6Home: u64,
    pub ContextFlags: CONTEXT_FLAGS,
    pub MxCsr: u32,
    pub SegCs: u16,
    pub SegDs: u16,
    pub SegEs: u16,
    pub SegFs: u16,
    pub SegGs: u16,
    pub SegSs: u16,
    pub EFlags: u32,
    pub Dr0: u64,
    pub Dr1: u64,
    pub Dr2: u64,
    pub Dr3: u64,
    pub Dr6: u64,
    pub Dr7: u64,
    pub Rax: u64,
    pub Rcx: u64,
    pub Rdx: u64,
    pub Rbx: u64,
    pub Rsp: u64,
    pub Rbp: u64,
    pub Rsi: u64,
    pub Rdi: u64,
    pub R8: u64,
    pub R9: u64,
    pub R10: u64,
    pub R11: u64,
    pub R12: u64,
    pub R13: u64,
    pub R14: u64,
    pub R15: u64,
    pub Rip: u64,
    pub Anonymous: CONTEXT_0,
    pub VectorRegister: [M128A; 26],
    pub VectorControl: u64,
    pub DebugControl: u64,
    pub LastBranchToRip: u64,
    pub LastBranchFromRip: u64,
    pub LastExceptionToRip: u64,
    pub LastExceptionFromRip: u64,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union CONTEXT_0 {
    pub FltSave: XSAVE_FORMAT,
    pub Anonymous: CONTEXT_0_0,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for CONTEXT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct CONTEXT_0_0 {
    pub Header: [M128A; 2],
    pub Legacy: [M128A; 8],
    pub Xmm0: M128A,
    pub Xmm1: M128A,
    pub Xmm2: M128A,
    pub Xmm3: M128A,
    pub Xmm4: M128A,
    pub Xmm5: M128A,
    pub Xmm6: M128A,
    pub Xmm7: M128A,
    pub Xmm8: M128A,
    pub Xmm9: M128A,
    pub Xmm10: M128A,
    pub Xmm11: M128A,
    pub Xmm12: M128A,
    pub Xmm13: M128A,
    pub Xmm14: M128A,
    pub Xmm15: M128A,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for CONTEXT_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy)]
pub struct CONTEXT {
    pub ContextFlags: CONTEXT_FLAGS,
    pub Cpsr: u32,
    pub Anonymous: CONTEXT_0,
    pub Sp: u64,
    pub Pc: u64,
    pub V: [ARM64_NT_NEON128; 32],
    pub Fpcr: u32,
    pub Fpsr: u32,
    pub Bcr: [u32; 8],
    pub Bvr: [u64; 8],
    pub Wcr: [u32; 2],
    pub Wvr: [u64; 2],
}
#[cfg(target_arch = "aarch64")]
impl Default for CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy)]
pub union CONTEXT_0 {
    pub Anonymous: CONTEXT_0_0,
    pub X: [u64; 31],
}
#[cfg(target_arch = "aarch64")]
impl Default for CONTEXT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy, Default)]
pub struct CONTEXT_0_0 {
    pub X0: u64,
    pub X1: u64,
    pub X2: u64,
    pub X3: u64,
    pub X4: u64,
    pub X5: u64,
    pub X6: u64,
    pub X7: u64,
    pub X8: u64,
    pub X9: u64,
    pub X10: u64,
    pub X11: u64,
    pub X12: u64,
    pub X13: u64,
    pub X14: u64,
    pub X15: u64,
    pub X16: u64,
    pub X17: u64,
    pub X18: u64,
    pub X19: u64,
    pub X20: u64,
    pub X21: u64,
    pub X22: u64,
    pub X23: u64,
    pub X24: u64,
    pub X25: u64,
    pub X26: u64,
    pub X27: u64,
    pub X28: u64,
    pub Fp: u64,
    pub Lr: u64,
}
pub type CONTEXT_FLAGS = u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRITICAL_SECTION {
    pub DebugInfo: *mut CRITICAL_SECTION_DEBUG,
    pub LockCount: i32,
    pub RecursionCount: i32,
    pub OwningThread: HANDLE,
    pub LockSemaphore: HANDLE,
    pub SpinCount: usize,
}
impl Default for CRITICAL_SECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRITICAL_SECTION_DEBUG {
    pub Type: u16,
    pub CreatorBackTraceIndex: u16,
    pub CriticalSection: *mut CRITICAL_SECTION,
    pub ProcessLocksList: LIST_ENTRY,
    pub EntryCount: u32,
    pub ContentionCount: u32,
    pub Flags: u32,
    pub CreatorBackTraceIndexHigh: u16,
    pub Identifier: u16,
}
impl Default for CRITICAL_SECTION_DEBUG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct DELAYLOAD_INFO {
    pub Size: u32,
    pub DelayloadDescriptor: *mut IMAGE_DELAYLOAD_DESCRIPTOR,
    pub ThunkAddress: *mut IMAGE_THUNK_DATA32,
    pub TargetDllName: PCSTR,
    pub TargetApiDescriptor: DELAYLOAD_PROC_DESCRIPTOR,
    pub TargetModuleBase: *mut core::ffi::c_void,
    pub Unused: *mut core::ffi::c_void,
    pub LastError: u32,
}
#[cfg(target_arch = "x86")]
impl Default for DELAYLOAD_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm64ec",
    target_arch = "x86_64"
))]
#[derive(Clone, Copy)]
pub struct DELAYLOAD_INFO {
    pub Size: u32,
    pub DelayloadDescriptor: *mut IMAGE_DELAYLOAD_DESCRIPTOR,
    pub ThunkAddress: *mut IMAGE_THUNK_DATA64,
    pub TargetDllName: PCSTR,
    pub TargetApiDescriptor: DELAYLOAD_PROC_DESCRIPTOR,
    pub TargetModuleBase: *mut core::ffi::c_void,
    pub Unused: *mut core::ffi::c_void,
    pub LastError: u32,
}
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm64ec",
    target_arch = "x86_64"
))]
impl Default for DELAYLOAD_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DELAYLOAD_PROC_DESCRIPTOR {
    pub ImportDescribedByName: u32,
    pub Description: DELAYLOAD_PROC_DESCRIPTOR_0,
}
impl Default for DELAYLOAD_PROC_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DELAYLOAD_PROC_DESCRIPTOR_0 {
    pub Name: PCSTR,
    pub Ordinal: u32,
}
impl Default for DELAYLOAD_PROC_DESCRIPTOR_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct ENCLAVE_IDENTITY {
    pub OwnerId: [u8; 32],
    pub UniqueId: [u8; 32],
    pub AuthorId: [u8; 32],
    pub FamilyId: [u8; 16],
    pub ImageId: [u8; 16],
    pub EnclaveSvn: u32,
    pub SecureKernelSvn: u32,
    pub PlatformSvn: u32,
    pub Flags: u32,
    pub SigningLevel: u32,
    pub EnclaveType: u32,
}
impl Default for ENCLAVE_IDENTITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ENCLAVE_INFORMATION {
    pub EnclaveType: u32,
    pub Reserved: u32,
    pub BaseAddress: *mut core::ffi::c_void,
    pub Size: usize,
    pub Identity: ENCLAVE_IDENTITY,
}
impl Default for ENCLAVE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type ENCLAVE_SEALING_IDENTITY_POLICY = i32;
pub type EXCEPTION_DISPOSITION = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EXCEPTION_POINTERS {
    pub ExceptionRecord: *mut EXCEPTION_RECORD,
    pub ContextRecord: *mut CONTEXT,
}
impl Default for EXCEPTION_POINTERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EXCEPTION_RECORD {
    pub ExceptionCode: NTSTATUS,
    pub ExceptionFlags: u32,
    pub ExceptionRecord: *mut EXCEPTION_RECORD,
    pub ExceptionAddress: *mut core::ffi::c_void,
    pub NumberParameters: u32,
    pub ExceptionInformation: [usize; 15],
}
impl Default for EXCEPTION_RECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type EXCEPTION_ROUTINE = Option<
    unsafe extern "system" fn(
        exceptionrecord: *mut EXCEPTION_RECORD,
        establisherframe: *const core::ffi::c_void,
        contextrecord: *mut CONTEXT,
        dispatchercontext: *const core::ffi::c_void,
    ) -> EXCEPTION_DISPOSITION,
>;
pub type FARPROC = Option<unsafe extern "system" fn() -> isize>;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct FLOATING_SAVE_AREA {
    pub ControlWord: u32,
    pub StatusWord: u32,
    pub TagWord: u32,
    pub ErrorOffset: u32,
    pub ErrorSelector: u32,
    pub DataOffset: u32,
    pub DataSelector: u32,
    pub RegisterArea: [u8; 80],
    pub Spare0: u32,
}
#[cfg(target_arch = "x86")]
impl Default for FLOATING_SAVE_AREA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm64ec",
    target_arch = "x86_64"
))]
#[derive(Clone, Copy)]
pub struct FLOATING_SAVE_AREA {
    pub ControlWord: u32,
    pub StatusWord: u32,
    pub TagWord: u32,
    pub ErrorOffset: u32,
    pub ErrorSelector: u32,
    pub DataOffset: u32,
    pub DataSelector: u32,
    pub RegisterArea: [u8; 80],
    pub Cr0NpxState: u32,
}
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm64ec",
    target_arch = "x86_64"
))]
impl Default for FLOATING_SAVE_AREA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type HANDLE = *mut core::ffi::c_void;
pub type HEAP_FLAGS = u32;
pub type HINSTANCE = *mut core::ffi::c_void;
pub type HMODULE = *mut core::ffi::c_void;
pub type HRESULT = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY {
    pub BeginAddress: u32,
    pub Anonymous: IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY_0,
}
impl Default for IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY_0 {
    pub UnwindData: u32,
    pub Anonymous: IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY_0_0,
}
impl Default for IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IMAGE_DELAYLOAD_DESCRIPTOR {
    pub Attributes: IMAGE_DELAYLOAD_DESCRIPTOR_0,
    pub DllNameRVA: u32,
    pub ModuleHandleRVA: u32,
    pub ImportAddressTableRVA: u32,
    pub ImportNameTableRVA: u32,
    pub BoundImportAddressTableRVA: u32,
    pub UnloadInformationTableRVA: u32,
    pub TimeDateStamp: u32,
}
impl Default for IMAGE_DELAYLOAD_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union IMAGE_DELAYLOAD_DESCRIPTOR_0 {
    pub AllAttributes: u32,
    pub Anonymous: IMAGE_DELAYLOAD_DESCRIPTOR_0_0,
}
impl Default for IMAGE_DELAYLOAD_DESCRIPTOR_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct IMAGE_DELAYLOAD_DESCRIPTOR_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IMAGE_RUNTIME_FUNCTION_ENTRY {
    pub BeginAddress: u32,
    pub EndAddress: u32,
    pub Anonymous: IMAGE_RUNTIME_FUNCTION_ENTRY_0,
}
impl Default for IMAGE_RUNTIME_FUNCTION_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union IMAGE_RUNTIME_FUNCTION_ENTRY_0 {
    pub UnwindInfoAddress: u32,
    pub UnwindData: u32,
}
impl Default for IMAGE_RUNTIME_FUNCTION_ENTRY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IMAGE_THUNK_DATA32 {
    pub u1: IMAGE_THUNK_DATA32_0,
}
impl Default for IMAGE_THUNK_DATA32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union IMAGE_THUNK_DATA32_0 {
    pub ForwarderString: u32,
    pub Function: u32,
    pub Ordinal: u32,
    pub AddressOfData: u32,
}
impl Default for IMAGE_THUNK_DATA32_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IMAGE_THUNK_DATA64 {
    pub u1: IMAGE_THUNK_DATA64_0,
}
impl Default for IMAGE_THUNK_DATA64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union IMAGE_THUNK_DATA64_0 {
    pub ForwarderString: u64,
    pub Function: u64,
    pub Ordinal: u64,
    pub AddressOfData: u64,
}
impl Default for IMAGE_THUNK_DATA64_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct KNONVOLATILE_CONTEXT_POINTERS {
    pub Dummy: u32,
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct KNONVOLATILE_CONTEXT_POINTERS {
    pub Anonymous1: KNONVOLATILE_CONTEXT_POINTERS_0,
    pub Anonymous2: KNONVOLATILE_CONTEXT_POINTERS_1,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for KNONVOLATILE_CONTEXT_POINTERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union KNONVOLATILE_CONTEXT_POINTERS_0 {
    pub FloatingContext: [*mut M128A; 16],
    pub Anonymous: KNONVOLATILE_CONTEXT_POINTERS_0_0,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for KNONVOLATILE_CONTEXT_POINTERS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct KNONVOLATILE_CONTEXT_POINTERS_0_0 {
    pub Xmm0: *mut M128A,
    pub Xmm1: *mut M128A,
    pub Xmm2: *mut M128A,
    pub Xmm3: *mut M128A,
    pub Xmm4: *mut M128A,
    pub Xmm5: *mut M128A,
    pub Xmm6: *mut M128A,
    pub Xmm7: *mut M128A,
    pub Xmm8: *mut M128A,
    pub Xmm9: *mut M128A,
    pub Xmm10: *mut M128A,
    pub Xmm11: *mut M128A,
    pub Xmm12: *mut M128A,
    pub Xmm13: *mut M128A,
    pub Xmm14: *mut M128A,
    pub Xmm15: *mut M128A,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for KNONVOLATILE_CONTEXT_POINTERS_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union KNONVOLATILE_CONTEXT_POINTERS_1 {
    pub IntegerContext: [*mut u64; 16],
    pub Anonymous: KNONVOLATILE_CONTEXT_POINTERS_1_0,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for KNONVOLATILE_CONTEXT_POINTERS_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct KNONVOLATILE_CONTEXT_POINTERS_1_0 {
    pub Rax: *mut u64,
    pub Rcx: *mut u64,
    pub Rdx: *mut u64,
    pub Rbx: *mut u64,
    pub Rsp: *mut u64,
    pub Rbp: *mut u64,
    pub Rsi: *mut u64,
    pub Rdi: *mut u64,
    pub R8: *mut u64,
    pub R9: *mut u64,
    pub R10: *mut u64,
    pub R11: *mut u64,
    pub R12: *mut u64,
    pub R13: *mut u64,
    pub R14: *mut u64,
    pub R15: *mut u64,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for KNONVOLATILE_CONTEXT_POINTERS_1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy)]
pub struct KNONVOLATILE_CONTEXT_POINTERS {
    pub X19: *mut u64,
    pub X20: *mut u64,
    pub X21: *mut u64,
    pub X22: *mut u64,
    pub X23: *mut u64,
    pub X24: *mut u64,
    pub X25: *mut u64,
    pub X26: *mut u64,
    pub X27: *mut u64,
    pub X28: *mut u64,
    pub Fp: *mut u64,
    pub Lr: *mut u64,
    pub D8: *mut u64,
    pub D9: *mut u64,
    pub D10: *mut u64,
    pub D11: *mut u64,
    pub D12: *mut u64,
    pub D13: *mut u64,
    pub D14: *mut u64,
    pub D15: *mut u64,
}
#[cfg(target_arch = "aarch64")]
impl Default for KNONVOLATILE_CONTEXT_POINTERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct LIST_ENTRY {
    pub Flink: *mut LIST_ENTRY,
    pub Blink: *mut LIST_ENTRY,
}
impl Default for LIST_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type LPTOP_LEVEL_EXCEPTION_FILTER =
    Option<unsafe extern "system" fn(exceptioninfo: *const EXCEPTION_POINTERS) -> i32>;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct M128A {
    pub Low: u64,
    pub High: i64,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct MEMORY_BASIC_INFORMATION {
    pub BaseAddress: *mut core::ffi::c_void,
    pub AllocationBase: *mut core::ffi::c_void,
    pub AllocationProtect: PAGE_PROTECTION_FLAGS,
    pub RegionSize: usize,
    pub State: VIRTUAL_ALLOCATION_TYPE,
    pub Protect: PAGE_PROTECTION_FLAGS,
    pub Type: PAGE_TYPE,
}
#[cfg(target_arch = "x86")]
impl Default for MEMORY_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm64ec",
    target_arch = "x86_64"
))]
#[derive(Clone, Copy)]
pub struct MEMORY_BASIC_INFORMATION {
    pub BaseAddress: *mut core::ffi::c_void,
    pub AllocationBase: *mut core::ffi::c_void,
    pub AllocationProtect: PAGE_PROTECTION_FLAGS,
    pub PartitionId: u16,
    pub RegionSize: usize,
    pub State: VIRTUAL_ALLOCATION_TYPE,
    pub Protect: PAGE_PROTECTION_FLAGS,
    pub Type: PAGE_TYPE,
}
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm64ec",
    target_arch = "x86_64"
))]
impl Default for MEMORY_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type MULTI_BYTE_TO_WIDE_CHAR_FLAGS = u32;
pub type NTSTATUS = i32;
pub type PAGE_PROTECTION_FLAGS = u32;
pub type PAGE_TYPE = u32;
pub type PCSTR = *const u8;
pub type PCWSTR = *const u16;
pub type PDELAYLOAD_FAILURE_DLL_CALLBACK = Option<
    unsafe extern "system" fn(
        notificationreason: u32,
        delayloadinfo: *const DELAYLOAD_INFO,
    ) -> *mut core::ffi::c_void,
>;
pub type PROCESSOR_FEATURE_ID = u32;
pub type PSTR = *mut u8;
pub type PWSTR = *mut u16;
pub type RTL_SYSTEM_GLOBAL_DATA_ID = i32;
pub type RTL_VIRTUAL_UNWIND_HANDLER_TYPE = u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SINGLE_LIST_ENTRY {
    pub Next: *mut SINGLE_LIST_ENTRY,
}
impl Default for SINGLE_LIST_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SLIST_ENTRY {
    pub Next: *mut SLIST_ENTRY,
}
impl Default for SLIST_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub union SLIST_HEADER {
    pub Alignment: u64,
    pub Anonymous: SLIST_HEADER_0,
}
#[cfg(target_arch = "x86")]
impl Default for SLIST_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct SLIST_HEADER_0 {
    pub Next: SINGLE_LIST_ENTRY,
    pub Depth: u16,
    pub CpuId: u16,
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union SLIST_HEADER {
    pub Anonymous: SLIST_HEADER_0,
    pub HeaderX64: SLIST_HEADER_1,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SLIST_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct SLIST_HEADER_0 {
    pub Alignment: u64,
    pub Region: u64,
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct SLIST_HEADER_1 {
    pub _bitfield1: u64,
    pub _bitfield2: u64,
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy)]
pub union SLIST_HEADER {
    pub Anonymous: SLIST_HEADER_0,
    pub HeaderArm64: SLIST_HEADER_1,
}
#[cfg(target_arch = "aarch64")]
impl Default for SLIST_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy, Default)]
pub struct SLIST_HEADER_0 {
    pub Alignment: u64,
    pub Region: u64,
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy, Default)]
pub struct SLIST_HEADER_1 {
    pub _bitfield1: u64,
    pub _bitfield2: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SRWLOCK {
    pub Ptr: *mut core::ffi::c_void,
}
impl Default for SRWLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SYNCHRONIZATION_BARRIER {
    pub Reserved1: u32,
    pub Reserved2: u32,
    pub Reserved3: [usize; 2],
    pub Reserved4: u32,
    pub Reserved5: u32,
}
impl Default for SYNCHRONIZATION_BARRIER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TIME_FIELDS {
    pub Year: i16,
    pub Month: i16,
    pub Day: i16,
    pub Hour: i16,
    pub Minute: i16,
    pub Second: i16,
    pub Milliseconds: i16,
    pub Weekday: i16,
}
#[repr(C)]
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm64ec",
    target_arch = "x86_64"
))]
#[derive(Clone, Copy)]
pub struct UNWIND_HISTORY_TABLE {
    pub Count: u32,
    pub LocalHint: u8,
    pub GlobalHint: u8,
    pub Search: u8,
    pub Once: u8,
    pub LowAddress: usize,
    pub HighAddress: usize,
    pub Entry: [UNWIND_HISTORY_TABLE_ENTRY; 12],
}
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm64ec",
    target_arch = "x86_64"
))]
impl Default for UNWIND_HISTORY_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct UNWIND_HISTORY_TABLE_ENTRY {
    pub ImageBase: usize,
    pub FunctionEntry: *mut IMAGE_RUNTIME_FUNCTION_ENTRY,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for UNWIND_HISTORY_TABLE_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy)]
pub struct UNWIND_HISTORY_TABLE_ENTRY {
    pub ImageBase: usize,
    pub FunctionEntry: *mut IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY,
}
#[cfg(target_arch = "aarch64")]
impl Default for UNWIND_HISTORY_TABLE_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type VIRTUAL_ALLOCATION_TYPE = u32;
pub type VIRTUAL_FREE_TYPE = u32;
pub type WIN32_ERROR = u32;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct XSAVE_FORMAT {
    pub ControlWord: u16,
    pub StatusWord: u16,
    pub TagWord: u8,
    pub Reserved1: u8,
    pub ErrorOpcode: u16,
    pub ErrorOffset: u32,
    pub ErrorSelector: u16,
    pub Reserved2: u16,
    pub DataOffset: u32,
    pub DataSelector: u16,
    pub Reserved3: u16,
    pub MxCsr: u32,
    pub MxCsr_Mask: u32,
    pub FloatRegisters: [M128A; 8],
    pub XmmRegisters: [M128A; 8],
    pub Reserved4: [u8; 224],
}
#[cfg(target_arch = "x86")]
impl Default for XSAVE_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm64ec",
    target_arch = "x86_64"
))]
#[derive(Clone, Copy)]
pub struct XSAVE_FORMAT {
    pub ControlWord: u16,
    pub StatusWord: u16,
    pub TagWord: u8,
    pub Reserved1: u8,
    pub ErrorOpcode: u16,
    pub ErrorOffset: u32,
    pub ErrorSelector: u16,
    pub Reserved2: u16,
    pub DataOffset: u32,
    pub DataSelector: u16,
    pub Reserved3: u16,
    pub MxCsr: u32,
    pub MxCsr_Mask: u32,
    pub FloatRegisters: [M128A; 8],
    pub XmmRegisters: [M128A; 16],
    pub Reserved4: [u8; 96],
}
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm64ec",
    target_arch = "x86_64"
))]
impl Default for XSAVE_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
