pub const ENCLAVE_FLAG_DYNAMIC_DEBUG_ACTIVE: i32 = 4;
pub const ENCLAVE_FLAG_DYNAMIC_DEBUG_ENABLED: i32 = 2;
pub const ENCLAVE_FLAG_FULL_DEBUG_ENABLED: i32 = 1;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct ENCLAVE_IDENTITY {
    pub OwnerId: [super::UINT8; 32],
    pub UniqueId: [super::UINT8; 32],
    pub AuthorId: [super::UINT8; 32],
    pub FamilyId: [super::UINT8; 16],
    pub ImageId: [super::UINT8; 16],
    pub EnclaveSvn: super::UINT32,
    pub SecureKernelSvn: super::UINT32,
    pub PlatformSvn: super::UINT32,
    pub Flags: super::UINT32,
    pub SigningLevel: super::UINT32,
    pub EnclaveType: super::UINT32,
}
#[cfg(feature = "shared")]
impl Default for ENCLAVE_IDENTITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ENCLAVE_IDENTITY_POLICY_SEAL_EXACT_CODE: ENCLAVE_SEALING_IDENTITY_POLICY = 1;
pub const ENCLAVE_IDENTITY_POLICY_SEAL_INVALID: ENCLAVE_SEALING_IDENTITY_POLICY = 0;
pub const ENCLAVE_IDENTITY_POLICY_SEAL_SAME_AUTHOR: ENCLAVE_SEALING_IDENTITY_POLICY = 5;
pub const ENCLAVE_IDENTITY_POLICY_SEAL_SAME_FAMILY: ENCLAVE_SEALING_IDENTITY_POLICY = 4;
pub const ENCLAVE_IDENTITY_POLICY_SEAL_SAME_IMAGE: ENCLAVE_SEALING_IDENTITY_POLICY = 3;
pub const ENCLAVE_IDENTITY_POLICY_SEAL_SAME_PRIMARY_CODE: ENCLAVE_SEALING_IDENTITY_POLICY = 2;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct ENCLAVE_INFORMATION {
    pub EnclaveType: super::ULONG,
    pub Reserved: super::ULONG,
    pub BaseAddress: super::PVOID,
    pub Size: super::SIZE_T,
    pub Identity: ENCLAVE_IDENTITY,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct ENCLAVE_PACKAGE_IDENTITY {
    pub PackageId: [super::UINT8; 32],
    pub PackageSvn: super::UINT64,
}
#[cfg(feature = "shared")]
impl Default for ENCLAVE_PACKAGE_IDENTITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ENCLAVE_REPORT_DATA_LENGTH: i32 = 64;
pub const ENCLAVE_RUNTIME_POLICY_ALLOW_DYNAMIC_DEBUG: i32 = 2;
pub const ENCLAVE_RUNTIME_POLICY_ALLOW_FULL_DEBUG: i32 = 1;
pub type ENCLAVE_SEALING_IDENTITY_POLICY = i32;
pub const ENCLAVE_UNSEAL_FLAG_STALE_KEY: i32 = 1;
pub const ENCLAVE_VBS_BASIC_KEY_FLAG_DEBUG_KEY: i32 = 8;
pub const ENCLAVE_VBS_BASIC_KEY_FLAG_FAMILY_ID: i32 = 2;
pub const ENCLAVE_VBS_BASIC_KEY_FLAG_IMAGE_ID: i32 = 4;
pub const ENCLAVE_VBS_BASIC_KEY_FLAG_MEASUREMENT: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ENCLAVE_VBS_BASIC_KEY_REQUEST {
    pub RequestSize: super::ULONG,
    pub Flags: super::ULONG,
    pub EnclaveSVN: super::ULONG,
    pub SystemKeyID: super::ULONG,
    pub CurrentSystemKeyID: super::ULONG,
}
#[cfg(feature = "shared")]
pub type PENCLAVE_VBS_BASIC_KEY_REQUEST = *mut ENCLAVE_VBS_BASIC_KEY_REQUEST;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
pub type PVBS_BASIC_ENCLAVE_EXCEPTION = *mut VBS_BASIC_ENCLAVE_EXCEPTION_AMD64;
#[cfg(any(target_arch = "aarch64", target_arch = "x86"))]
pub type PVBS_BASIC_ENCLAVE_EXCEPTION = *mut core::ffi::c_void;
#[cfg(feature = "shared")]
pub type PVBS_BASIC_ENCLAVE_EXCEPTION_AMD64 = *mut VBS_BASIC_ENCLAVE_EXCEPTION_AMD64;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PVBS_BASIC_ENCLAVE_SYSCALL_PAGE = *mut VBS_BASIC_ENCLAVE_SYSCALL_PAGE;
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
pub type PVBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR = *mut VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
pub type PVBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR = *mut VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR64;
#[cfg(feature = "shared")]
pub type PVBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR32 = *mut VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR32;
#[cfg(feature = "shared")]
pub type PVBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR64 = *mut VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR64;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_COMMIT_PAGES = Option<unsafe extern "C" fn(enclaveaddress: super::PVOID, numberofbytes: super::SIZE_T, sourceaddress: super::PVOID, pageprotection: super::ULONG) -> super::LONG>;
#[cfg(feature = "shared")]
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_CREATE_THREAD = Option<unsafe extern "C" fn(threaddescriptor: PVBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR) -> super::LONG>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_DECOMMIT_PAGES = Option<unsafe extern "C" fn(enclaveaddress: super::PVOID, numberofbytes: super::SIZE_T) -> super::LONG>;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_GENERATE_KEY = Option<unsafe extern "C" fn(keyrequest: PENCLAVE_VBS_BASIC_KEY_REQUEST, requestedkeysize: super::ULONG, returnedkey: super::PUCHAR) -> super::LONG>;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_GENERATE_RANDOM_DATA = Option<unsafe extern "C" fn(buffer: super::PUCHAR, numberofbytes: super::ULONG, generation: super::PULONGLONG) -> super::LONG>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_GENERATE_REPORT = Option<unsafe extern "C" fn(enclavedata: *const super::UINT8, report: super::PVOID, buffersize: super::UINT32, outputsize: *mut super::UINT32) -> super::LONG>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_GET_ENCLAVE_INFORMATION = Option<unsafe extern "C" fn(enclaveinfo: *mut ENCLAVE_INFORMATION) -> super::LONG>;
#[cfg(feature = "shared")]
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_INTERRUPT_THREAD = Option<unsafe extern "C" fn(threaddescriptor: PVBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR) -> super::LONG>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_PROTECT_PAGES = Option<unsafe extern "C" fn(enclaveaddress: super::PVOID, numberofytes: super::SIZE_T, pageprotection: super::ULONG) -> super::LONG>;
#[cfg(feature = "shared")]
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_RETURN_FROM_ENCLAVE = Option<unsafe extern "C" fn(returnvalue: super::ULONG_PTR)>;
#[cfg(feature = "shared")]
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_RETURN_FROM_EXCEPTION = Option<unsafe extern "C" fn(exceptionrecord: PVBS_BASIC_ENCLAVE_EXCEPTION) -> super::LONG>;
#[cfg(feature = "shared")]
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_TERMINATE_THREAD = Option<unsafe extern "C" fn(threaddescriptor: PVBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR) -> super::LONG>;
#[cfg(feature = "shared")]
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_VERIFY_REPORT = Option<unsafe extern "C" fn(report: *const core::ffi::c_void, reportsize: super::UINT32) -> super::LONG>;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VBS_BASIC_ENCLAVE_EXCEPTION_AMD64 {
    pub ExceptionCode: super::ULONG,
    pub NumberParameters: super::ULONG,
    pub ExceptionInformation: [super::ULONG_PTR; 3],
    pub ExceptionRAX: super::ULONG_PTR,
    pub ExceptionRCX: super::ULONG_PTR,
    pub ExceptionRIP: super::ULONG_PTR,
    pub ExceptionRFLAGS: super::ULONG_PTR,
    pub ExceptionRSP: super::ULONG_PTR,
}
#[cfg(feature = "shared")]
impl Default for VBS_BASIC_ENCLAVE_EXCEPTION_AMD64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct VBS_BASIC_ENCLAVE_SYSCALL_PAGE {
    pub ReturnFromEnclave: VBS_BASIC_ENCLAVE_BASIC_CALL_RETURN_FROM_ENCLAVE,
    pub ReturnFromException: VBS_BASIC_ENCLAVE_BASIC_CALL_RETURN_FROM_EXCEPTION,
    pub TerminateThread: VBS_BASIC_ENCLAVE_BASIC_CALL_TERMINATE_THREAD,
    pub InterruptThread: VBS_BASIC_ENCLAVE_BASIC_CALL_INTERRUPT_THREAD,
    pub CommitPages: VBS_BASIC_ENCLAVE_BASIC_CALL_COMMIT_PAGES,
    pub DecommitPages: VBS_BASIC_ENCLAVE_BASIC_CALL_DECOMMIT_PAGES,
    pub ProtectPages: VBS_BASIC_ENCLAVE_BASIC_CALL_PROTECT_PAGES,
    pub CreateThread: VBS_BASIC_ENCLAVE_BASIC_CALL_CREATE_THREAD,
    pub GetEnclaveInformation: VBS_BASIC_ENCLAVE_BASIC_CALL_GET_ENCLAVE_INFORMATION,
    pub GenerateKey: VBS_BASIC_ENCLAVE_BASIC_CALL_GENERATE_KEY,
    pub GenerateReport: VBS_BASIC_ENCLAVE_BASIC_CALL_GENERATE_REPORT,
    pub VerifyReport: VBS_BASIC_ENCLAVE_BASIC_CALL_VERIFY_REPORT,
    pub GenerateRandomData: VBS_BASIC_ENCLAVE_BASIC_CALL_GENERATE_RANDOM_DATA,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
pub type VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR = VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
pub type VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR = VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR64;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR32 {
    pub ThreadContext: [super::ULONG; 4],
    pub EntryPoint: super::ULONG,
    pub StackPointer: super::ULONG,
    pub ExceptionEntryPoint: super::ULONG,
    pub ExceptionStack: super::ULONG,
    pub ExceptionActive: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR64 {
    pub ThreadContext: [super::ULONGLONG; 4],
    pub EntryPoint: super::ULONGLONG,
    pub StackPointer: super::ULONGLONG,
    pub ExceptionEntryPoint: super::ULONGLONG,
    pub ExceptionStack: super::ULONGLONG,
    pub ExceptionActive: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct VBS_ENCLAVE_REPORT {
    pub ReportSize: super::UINT32,
    pub ReportVersion: super::UINT32,
    pub EnclaveData: [super::UINT8; 64],
    pub EnclaveIdentity: ENCLAVE_IDENTITY,
}
#[cfg(feature = "shared")]
impl Default for VBS_ENCLAVE_REPORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct VBS_ENCLAVE_REPORT_MODULE {
    pub Header: VBS_ENCLAVE_REPORT_VARDATA_HEADER,
    pub UniqueId: [super::UINT8; 32],
    pub AuthorId: [super::UINT8; 32],
    pub FamilyId: [super::UINT8; 16],
    pub ImageId: [super::UINT8; 16],
    pub Svn: super::UINT32,
    pub ModuleName: [super::WCHAR; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for VBS_ENCLAVE_REPORT_MODULE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct VBS_ENCLAVE_REPORT_PACKAGE_ID {
    pub Header: VBS_ENCLAVE_REPORT_VARDATA_HEADER,
    pub PackageIdentity: ENCLAVE_PACKAGE_IDENTITY,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct VBS_ENCLAVE_REPORT_PKG_HEADER {
    pub PackageSize: super::UINT32,
    pub Version: super::UINT32,
    pub SignatureScheme: super::UINT32,
    pub SignedStatementSize: super::UINT32,
    pub SignatureSize: super::UINT32,
    pub Reserved: super::UINT32,
}
pub const VBS_ENCLAVE_REPORT_PKG_HEADER_VERSION_CURRENT: i32 = 1;
pub const VBS_ENCLAVE_REPORT_SIGNATURE_SCHEME_SHA256_RSA_PSS_SHA256: i32 = 1;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct VBS_ENCLAVE_REPORT_VARDATA_HEADER {
    pub DataType: super::UINT32,
    pub Size: super::UINT32,
}
pub const VBS_ENCLAVE_REPORT_VERSION_CURRENT: i32 = 1;
pub const VBS_ENCLAVE_VARDATA_INVALID: i32 = 0;
pub const VBS_ENCLAVE_VARDATA_MODULE: i32 = 1;
pub const VBS_ENCLAVE_VARDATA_PACKAGE: i32 = 2;
