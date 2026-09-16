#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn NtAccessCheckAndAuditAlarm(subsystemname: super::PUNICODE_STRING, handleid: Option<*const core::ffi::c_void>, objecttypename: super::PUNICODE_STRING, objectname: super::PUNICODE_STRING, securitydescriptor: super::PSECURITY_DESCRIPTOR, desiredaccess: super::ACCESS_MASK, genericmapping: super::PGENERIC_MAPPING, objectcreation: super::BOOLEAN, grantedaccess: super::PACCESS_MASK, accessstatus: super::PNTSTATUS, generateonclose: super::PBOOLEAN) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtAccessCheckAndAuditAlarm(subsystemname : super::PUNICODE_STRING, handleid : *const core::ffi::c_void, objecttypename : super::PUNICODE_STRING, objectname : super::PUNICODE_STRING, securitydescriptor : super::PSECURITY_DESCRIPTOR, desiredaccess : super::ACCESS_MASK, genericmapping : super::PGENERIC_MAPPING, objectcreation : super::BOOLEAN, grantedaccess : super::PACCESS_MASK, accessstatus : super::PNTSTATUS, generateonclose : super::PBOOLEAN) -> windows_core::NTSTATUS);
    unsafe { NtAccessCheckAndAuditAlarm(subsystemname, handleid.unwrap_or(core::mem::zeroed()) as _, objecttypename, objectname, securitydescriptor, desiredaccess, genericmapping, objectcreation, grantedaccess as _, accessstatus as _, generateonclose as _) }
}
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn NtAccessCheckByTypeAndAuditAlarm(subsystemname: super::PUNICODE_STRING, handleid: Option<*const core::ffi::c_void>, objecttypename: super::PUNICODE_STRING, objectname: super::PUNICODE_STRING, securitydescriptor: super::PSECURITY_DESCRIPTOR, principalselfsid: Option<super::PSID>, desiredaccess: super::ACCESS_MASK, audittype: super::AUDIT_EVENT_TYPE, flags: u32, objecttypelist: Option<&[super::OBJECT_TYPE_LIST]>, genericmapping: super::PGENERIC_MAPPING, objectcreation: super::BOOLEAN, grantedaccess: super::PACCESS_MASK, accessstatus: super::PNTSTATUS, generateonclose: super::PBOOLEAN) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtAccessCheckByTypeAndAuditAlarm(subsystemname : super::PUNICODE_STRING, handleid : *const core::ffi::c_void, objecttypename : super::PUNICODE_STRING, objectname : super::PUNICODE_STRING, securitydescriptor : super::PSECURITY_DESCRIPTOR, principalselfsid : super::PSID, desiredaccess : super::ACCESS_MASK, audittype : super::AUDIT_EVENT_TYPE, flags : u32, objecttypelist : super::POBJECT_TYPE_LIST, objecttypelistlength : u32, genericmapping : super::PGENERIC_MAPPING, objectcreation : super::BOOLEAN, grantedaccess : super::PACCESS_MASK, accessstatus : super::PNTSTATUS, generateonclose : super::PBOOLEAN) -> windows_core::NTSTATUS);
    unsafe { NtAccessCheckByTypeAndAuditAlarm(subsystemname, handleid.unwrap_or(core::mem::zeroed()) as _, objecttypename, objectname, securitydescriptor, principalselfsid.unwrap_or(core::mem::zeroed()) as _, desiredaccess, audittype, flags, core::mem::transmute(objecttypelist.map_or(core::ptr::null(), |slice| slice.as_ptr())), objecttypelist.map_or(0, |slice| slice.len().try_into().unwrap()), genericmapping, objectcreation, grantedaccess as _, accessstatus as _, generateonclose as _) }
}
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn NtAccessCheckByTypeResultListAndAuditAlarm(subsystemname: super::PUNICODE_STRING, handleid: Option<*const core::ffi::c_void>, objecttypename: super::PUNICODE_STRING, objectname: super::PUNICODE_STRING, securitydescriptor: super::PSECURITY_DESCRIPTOR, principalselfsid: Option<super::PSID>, desiredaccess: super::ACCESS_MASK, audittype: super::AUDIT_EVENT_TYPE, flags: u32, objecttypelist: Option<super::POBJECT_TYPE_LIST>, objecttypelistlength: u32, genericmapping: super::PGENERIC_MAPPING, objectcreation: super::BOOLEAN, grantedaccess: super::PACCESS_MASK, accessstatus: super::PNTSTATUS, generateonclose: super::PBOOLEAN) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtAccessCheckByTypeResultListAndAuditAlarm(subsystemname : super::PUNICODE_STRING, handleid : *const core::ffi::c_void, objecttypename : super::PUNICODE_STRING, objectname : super::PUNICODE_STRING, securitydescriptor : super::PSECURITY_DESCRIPTOR, principalselfsid : super::PSID, desiredaccess : super::ACCESS_MASK, audittype : super::AUDIT_EVENT_TYPE, flags : u32, objecttypelist : super::POBJECT_TYPE_LIST, objecttypelistlength : u32, genericmapping : super::PGENERIC_MAPPING, objectcreation : super::BOOLEAN, grantedaccess : super::PACCESS_MASK, accessstatus : super::PNTSTATUS, generateonclose : super::PBOOLEAN) -> windows_core::NTSTATUS);
    unsafe { NtAccessCheckByTypeResultListAndAuditAlarm(subsystemname, handleid.unwrap_or(core::mem::zeroed()) as _, objecttypename, objectname, securitydescriptor, principalselfsid.unwrap_or(core::mem::zeroed()) as _, desiredaccess, audittype, flags, objecttypelist.unwrap_or(core::mem::zeroed()) as _, objecttypelistlength, genericmapping, objectcreation, grantedaccess as _, accessstatus as _, generateonclose as _) }
}
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn NtAccessCheckByTypeResultListAndAuditAlarmByHandle(subsystemname: super::PUNICODE_STRING, handleid: Option<*const core::ffi::c_void>, clienttoken: super::HANDLE, objecttypename: super::PUNICODE_STRING, objectname: super::PUNICODE_STRING, securitydescriptor: super::PSECURITY_DESCRIPTOR, principalselfsid: Option<super::PSID>, desiredaccess: super::ACCESS_MASK, audittype: super::AUDIT_EVENT_TYPE, flags: u32, objecttypelist: Option<super::POBJECT_TYPE_LIST>, objecttypelistlength: u32, genericmapping: super::PGENERIC_MAPPING, objectcreation: super::BOOLEAN, grantedaccess: super::PACCESS_MASK, accessstatus: super::PNTSTATUS, generateonclose: super::PBOOLEAN) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtAccessCheckByTypeResultListAndAuditAlarmByHandle(subsystemname : super::PUNICODE_STRING, handleid : *const core::ffi::c_void, clienttoken : super::HANDLE, objecttypename : super::PUNICODE_STRING, objectname : super::PUNICODE_STRING, securitydescriptor : super::PSECURITY_DESCRIPTOR, principalselfsid : super::PSID, desiredaccess : super::ACCESS_MASK, audittype : super::AUDIT_EVENT_TYPE, flags : u32, objecttypelist : super::POBJECT_TYPE_LIST, objecttypelistlength : u32, genericmapping : super::PGENERIC_MAPPING, objectcreation : super::BOOLEAN, grantedaccess : super::PACCESS_MASK, accessstatus : super::PNTSTATUS, generateonclose : super::PBOOLEAN) -> windows_core::NTSTATUS);
    unsafe { NtAccessCheckByTypeResultListAndAuditAlarmByHandle(subsystemname, handleid.unwrap_or(core::mem::zeroed()) as _, clienttoken, objecttypename, objectname, securitydescriptor, principalselfsid.unwrap_or(core::mem::zeroed()) as _, desiredaccess, audittype, flags, objecttypelist.unwrap_or(core::mem::zeroed()) as _, objecttypelistlength, genericmapping, objectcreation, grantedaccess as _, accessstatus as _, generateonclose as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn NtAdjustGroupsToken(tokenhandle: super::HANDLE, resettodefault: super::BOOLEAN, newstate: Option<super::PTOKEN_GROUPS>, bufferlength: u32, previousstate: Option<super::PTOKEN_GROUPS>, returnlength: super::PULONG) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtAdjustGroupsToken(tokenhandle : super::HANDLE, resettodefault : super::BOOLEAN, newstate : super::PTOKEN_GROUPS, bufferlength : u32, previousstate : super::PTOKEN_GROUPS, returnlength : super::PULONG) -> windows_core::NTSTATUS);
    unsafe { NtAdjustGroupsToken(tokenhandle, resettodefault, newstate.unwrap_or(core::mem::zeroed()) as _, bufferlength, previousstate.unwrap_or(core::mem::zeroed()) as _, returnlength as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn NtAdjustPrivilegesToken(tokenhandle: super::HANDLE, disableallprivileges: super::BOOLEAN, newstate: Option<super::PTOKEN_PRIVILEGES>, bufferlength: u32, previousstate: Option<super::PTOKEN_PRIVILEGES>, returnlength: super::PULONG) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtAdjustPrivilegesToken(tokenhandle : super::HANDLE, disableallprivileges : super::BOOLEAN, newstate : super::PTOKEN_PRIVILEGES, bufferlength : u32, previousstate : super::PTOKEN_PRIVILEGES, returnlength : super::PULONG) -> windows_core::NTSTATUS);
    unsafe { NtAdjustPrivilegesToken(tokenhandle, disableallprivileges, newstate.unwrap_or(core::mem::zeroed()) as _, bufferlength, previousstate.unwrap_or(core::mem::zeroed()) as _, returnlength as _) }
}
#[cfg(all(feature = "basetsd", feature = "winnt"))]
#[inline]
pub unsafe fn NtAllocateVirtualMemory(processhandle: super::HANDLE, baseaddress: *mut *mut core::ffi::c_void, zerobits: usize, regionsize: super::PSIZE_T, allocationtype: u32, protect: u32) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtAllocateVirtualMemory(processhandle : super::HANDLE, baseaddress : *mut *mut core::ffi::c_void, zerobits : usize, regionsize : super::PSIZE_T, allocationtype : u32, protect : u32) -> windows_core::NTSTATUS);
    unsafe { NtAllocateVirtualMemory(processhandle, baseaddress as _, zerobits, regionsize as _, allocationtype, protect) }
}
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn NtCloseObjectAuditAlarm(subsystemname: super::PUNICODE_STRING, handleid: Option<*const core::ffi::c_void>, generateonclose: super::BOOLEAN) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtCloseObjectAuditAlarm(subsystemname : super::PUNICODE_STRING, handleid : *const core::ffi::c_void, generateonclose : super::BOOLEAN) -> windows_core::NTSTATUS);
    unsafe { NtCloseObjectAuditAlarm(subsystemname, handleid.unwrap_or(core::mem::zeroed()) as _, generateonclose) }
}
#[cfg(all(feature = "minwindef", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn NtCopyFileChunk(sourcehandle: super::HANDLE, desthandle: super::HANDLE, event: Option<super::HANDLE>, iostatusblock: super::PIO_STATUS_BLOCK, length: u32, sourceoffset: super::PLARGE_INTEGER, destoffset: super::PLARGE_INTEGER, sourcekey: Option<super::PULONG>, destkey: Option<super::PULONG>, flags: u32) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtCopyFileChunk(sourcehandle : super::HANDLE, desthandle : super::HANDLE, event : super::HANDLE, iostatusblock : super::PIO_STATUS_BLOCK, length : u32, sourceoffset : super::PLARGE_INTEGER, destoffset : super::PLARGE_INTEGER, sourcekey : super::PULONG, destkey : super::PULONG, flags : u32) -> windows_core::NTSTATUS);
    unsafe { NtCopyFileChunk(sourcehandle, desthandle, event.unwrap_or(core::mem::zeroed()) as _, iostatusblock as _, length, sourceoffset, destoffset, sourcekey.unwrap_or(core::mem::zeroed()) as _, destkey.unwrap_or(core::mem::zeroed()) as _, flags) }
}
#[cfg(all(feature = "d3dkmthk", feature = "lsalookup", feature = "ntsecapi", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn NtCreateSection(sectionhandle: super::PHANDLE, desiredaccess: super::ACCESS_MASK, objectattributes: Option<super::POBJECT_ATTRIBUTES>, maximumsize: Option<super::PLARGE_INTEGER>, sectionpageprotection: u32, allocationattributes: u32, filehandle: Option<super::HANDLE>) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtCreateSection(sectionhandle : super::PHANDLE, desiredaccess : super::ACCESS_MASK, objectattributes : super::POBJECT_ATTRIBUTES, maximumsize : super::PLARGE_INTEGER, sectionpageprotection : u32, allocationattributes : u32, filehandle : super::HANDLE) -> windows_core::NTSTATUS);
    unsafe { NtCreateSection(sectionhandle as _, desiredaccess, objectattributes.unwrap_or(core::mem::zeroed()) as _, maximumsize.unwrap_or(core::mem::zeroed()) as _, sectionpageprotection, allocationattributes, filehandle.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "d3dkmthk", feature = "lsalookup", feature = "ntsecapi", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn NtCreateSectionEx(sectionhandle: super::PHANDLE, desiredaccess: super::ACCESS_MASK, objectattributes: Option<super::POBJECT_ATTRIBUTES>, maximumsize: Option<super::PLARGE_INTEGER>, sectionpageprotection: u32, allocationattributes: u32, filehandle: Option<super::HANDLE>, extendedparameters: Option<&mut [super::MEM_EXTENDED_PARAMETER]>) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtCreateSectionEx(sectionhandle : super::PHANDLE, desiredaccess : super::ACCESS_MASK, objectattributes : super::POBJECT_ATTRIBUTES, maximumsize : super::PLARGE_INTEGER, sectionpageprotection : u32, allocationattributes : u32, filehandle : super::HANDLE, extendedparameters : super::PMEM_EXTENDED_PARAMETER, extendedparametercount : u32) -> windows_core::NTSTATUS);
    unsafe { NtCreateSectionEx(sectionhandle as _, desiredaccess, objectattributes.unwrap_or(core::mem::zeroed()) as _, maximumsize.unwrap_or(core::mem::zeroed()) as _, sectionpageprotection, allocationattributes, filehandle.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(extendedparameters.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut())), extendedparameters.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn NtDeleteObjectAuditAlarm(subsystemname: super::PUNICODE_STRING, handleid: Option<*const core::ffi::c_void>, generateonclose: super::BOOLEAN) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtDeleteObjectAuditAlarm(subsystemname : super::PUNICODE_STRING, handleid : *const core::ffi::c_void, generateonclose : super::BOOLEAN) -> windows_core::NTSTATUS);
    unsafe { NtDeleteObjectAuditAlarm(subsystemname, handleid.unwrap_or(core::mem::zeroed()) as _, generateonclose) }
}
#[cfg(all(feature = "d3dkmthk", feature = "lsalookup", feature = "ntsecapi", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn NtDuplicateToken(existingtokenhandle: super::HANDLE, desiredaccess: super::ACCESS_MASK, objectattributes: Option<super::POBJECT_ATTRIBUTES>, effectiveonly: super::BOOLEAN, tokentype: super::TOKEN_TYPE, newtokenhandle: super::PHANDLE) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtDuplicateToken(existingtokenhandle : super::HANDLE, desiredaccess : super::ACCESS_MASK, objectattributes : super::POBJECT_ATTRIBUTES, effectiveonly : super::BOOLEAN, tokentype : super::TOKEN_TYPE, newtokenhandle : super::PHANDLE) -> windows_core::NTSTATUS);
    unsafe { NtDuplicateToken(existingtokenhandle, desiredaccess, objectattributes.unwrap_or(core::mem::zeroed()) as _, effectiveonly, tokentype, newtokenhandle as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn NtFilterToken(existingtokenhandle: super::HANDLE, flags: u32, sidstodisable: Option<super::PTOKEN_GROUPS>, privilegestodelete: Option<super::PTOKEN_PRIVILEGES>, restrictedsids: Option<super::PTOKEN_GROUPS>, newtokenhandle: super::PHANDLE) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtFilterToken(existingtokenhandle : super::HANDLE, flags : u32, sidstodisable : super::PTOKEN_GROUPS, privilegestodelete : super::PTOKEN_PRIVILEGES, restrictedsids : super::PTOKEN_GROUPS, newtokenhandle : super::PHANDLE) -> windows_core::NTSTATUS);
    unsafe { NtFilterToken(existingtokenhandle, flags, sidstodisable.unwrap_or(core::mem::zeroed()) as _, privilegestodelete.unwrap_or(core::mem::zeroed()) as _, restrictedsids.unwrap_or(core::mem::zeroed()) as _, newtokenhandle as _) }
}
#[cfg(all(feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn NtFlushBuffersFileEx(filehandle: super::HANDLE, flags: u32, parameters: *const core::ffi::c_void, parameterssize: u32, iostatusblock: super::PIO_STATUS_BLOCK) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtFlushBuffersFileEx(filehandle : super::HANDLE, flags : u32, parameters : *const core::ffi::c_void, parameterssize : u32, iostatusblock : super::PIO_STATUS_BLOCK) -> windows_core::NTSTATUS);
    unsafe { NtFlushBuffersFileEx(filehandle, flags, parameters, parameterssize, iostatusblock as _) }
}
#[cfg(all(feature = "basetsd", feature = "winnt"))]
#[inline]
pub unsafe fn NtFreeVirtualMemory(processhandle: super::HANDLE, baseaddress: *mut *mut core::ffi::c_void, regionsize: super::PSIZE_T, freetype: u32) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtFreeVirtualMemory(processhandle : super::HANDLE, baseaddress : *mut *mut core::ffi::c_void, regionsize : super::PSIZE_T, freetype : u32) -> windows_core::NTSTATUS);
    unsafe { NtFreeVirtualMemory(processhandle, baseaddress as _, regionsize as _, freetype) }
}
#[cfg(all(feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn NtFsControlFile(filehandle: super::HANDLE, event: Option<super::HANDLE>, apcroutine: super::PIO_APC_ROUTINE, apccontext: Option<*const core::ffi::c_void>, iostatusblock: super::PIO_STATUS_BLOCK, fscontrolcode: u32, inputbuffer: Option<*const core::ffi::c_void>, inputbufferlength: u32, outputbuffer: Option<*mut core::ffi::c_void>, outputbufferlength: u32) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtFsControlFile(filehandle : super::HANDLE, event : super::HANDLE, apcroutine : super::PIO_APC_ROUTINE, apccontext : *const core::ffi::c_void, iostatusblock : super::PIO_STATUS_BLOCK, fscontrolcode : u32, inputbuffer : *const core::ffi::c_void, inputbufferlength : u32, outputbuffer : *mut core::ffi::c_void, outputbufferlength : u32) -> windows_core::NTSTATUS);
    unsafe { NtFsControlFile(filehandle, event.unwrap_or(core::mem::zeroed()) as _, apcroutine, apccontext.unwrap_or(core::mem::zeroed()) as _, iostatusblock as _, fscontrolcode, inputbuffer.unwrap_or(core::mem::zeroed()) as _, inputbufferlength, outputbuffer.unwrap_or(core::mem::zeroed()) as _, outputbufferlength) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn NtImpersonateAnonymousToken(threadhandle: super::HANDLE) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtImpersonateAnonymousToken(threadhandle : super::HANDLE) -> windows_core::NTSTATUS);
    unsafe { NtImpersonateAnonymousToken(threadhandle) }
}
#[cfg(all(feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn NtLockFile(filehandle: super::HANDLE, event: Option<super::HANDLE>, apcroutine: super::PIO_APC_ROUTINE, apccontext: Option<*const core::ffi::c_void>, iostatusblock: super::PIO_STATUS_BLOCK, byteoffset: super::PLARGE_INTEGER, length: super::PLARGE_INTEGER, key: u32, failimmediately: super::BOOLEAN, exclusivelock: super::BOOLEAN) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtLockFile(filehandle : super::HANDLE, event : super::HANDLE, apcroutine : super::PIO_APC_ROUTINE, apccontext : *const core::ffi::c_void, iostatusblock : super::PIO_STATUS_BLOCK, byteoffset : super::PLARGE_INTEGER, length : super::PLARGE_INTEGER, key : u32, failimmediately : super::BOOLEAN, exclusivelock : super::BOOLEAN) -> windows_core::NTSTATUS);
    unsafe { NtLockFile(filehandle, event.unwrap_or(core::mem::zeroed()) as _, apcroutine, apccontext.unwrap_or(core::mem::zeroed()) as _, iostatusblock as _, byteoffset, length, key, failimmediately, exclusivelock) }
}
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn NtOpenObjectAuditAlarm(subsystemname: super::PUNICODE_STRING, handleid: Option<*const core::ffi::c_void>, objecttypename: super::PUNICODE_STRING, objectname: super::PUNICODE_STRING, securitydescriptor: Option<super::PSECURITY_DESCRIPTOR>, clienttoken: super::HANDLE, desiredaccess: super::ACCESS_MASK, grantedaccess: super::ACCESS_MASK, privileges: Option<super::PPRIVILEGE_SET>, objectcreation: super::BOOLEAN, accessgranted: super::BOOLEAN, generateonclose: super::PBOOLEAN) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtOpenObjectAuditAlarm(subsystemname : super::PUNICODE_STRING, handleid : *const core::ffi::c_void, objecttypename : super::PUNICODE_STRING, objectname : super::PUNICODE_STRING, securitydescriptor : super::PSECURITY_DESCRIPTOR, clienttoken : super::HANDLE, desiredaccess : super::ACCESS_MASK, grantedaccess : super::ACCESS_MASK, privileges : super::PPRIVILEGE_SET, objectcreation : super::BOOLEAN, accessgranted : super::BOOLEAN, generateonclose : super::PBOOLEAN) -> windows_core::NTSTATUS);
    unsafe { NtOpenObjectAuditAlarm(subsystemname, handleid.unwrap_or(core::mem::zeroed()) as _, objecttypename, objectname, securitydescriptor.unwrap_or(core::mem::zeroed()) as _, clienttoken, desiredaccess, grantedaccess, privileges.unwrap_or(core::mem::zeroed()) as _, objectcreation, accessgranted, generateonclose as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn NtOpenProcessToken(processhandle: super::HANDLE, desiredaccess: super::ACCESS_MASK, tokenhandle: super::PHANDLE) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtOpenProcessToken(processhandle : super::HANDLE, desiredaccess : super::ACCESS_MASK, tokenhandle : super::PHANDLE) -> windows_core::NTSTATUS);
    unsafe { NtOpenProcessToken(processhandle, desiredaccess, tokenhandle as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn NtOpenProcessTokenEx(processhandle: super::HANDLE, desiredaccess: super::ACCESS_MASK, handleattributes: u32, tokenhandle: super::PHANDLE) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtOpenProcessTokenEx(processhandle : super::HANDLE, desiredaccess : super::ACCESS_MASK, handleattributes : u32, tokenhandle : super::PHANDLE) -> windows_core::NTSTATUS);
    unsafe { NtOpenProcessTokenEx(processhandle, desiredaccess, handleattributes, tokenhandle as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn NtOpenThreadToken(threadhandle: super::HANDLE, desiredaccess: super::ACCESS_MASK, openasself: super::BOOLEAN, tokenhandle: super::PHANDLE) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtOpenThreadToken(threadhandle : super::HANDLE, desiredaccess : super::ACCESS_MASK, openasself : super::BOOLEAN, tokenhandle : super::PHANDLE) -> windows_core::NTSTATUS);
    unsafe { NtOpenThreadToken(threadhandle, desiredaccess, openasself, tokenhandle as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn NtOpenThreadTokenEx(threadhandle: super::HANDLE, desiredaccess: super::ACCESS_MASK, openasself: super::BOOLEAN, handleattributes: u32, tokenhandle: super::PHANDLE) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtOpenThreadTokenEx(threadhandle : super::HANDLE, desiredaccess : super::ACCESS_MASK, openasself : super::BOOLEAN, handleattributes : u32, tokenhandle : super::PHANDLE) -> windows_core::NTSTATUS);
    unsafe { NtOpenThreadTokenEx(threadhandle, desiredaccess, openasself, handleattributes, tokenhandle as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn NtPrivilegeCheck(clienttoken: super::HANDLE, requiredprivileges: super::PPRIVILEGE_SET, result: super::PBOOLEAN) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtPrivilegeCheck(clienttoken : super::HANDLE, requiredprivileges : super::PPRIVILEGE_SET, result : super::PBOOLEAN) -> windows_core::NTSTATUS);
    unsafe { NtPrivilegeCheck(clienttoken, requiredprivileges as _, result as _) }
}
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn NtPrivilegeObjectAuditAlarm(subsystemname: super::PUNICODE_STRING, handleid: Option<*const core::ffi::c_void>, clienttoken: super::HANDLE, desiredaccess: super::ACCESS_MASK, privileges: super::PPRIVILEGE_SET, accessgranted: super::BOOLEAN) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtPrivilegeObjectAuditAlarm(subsystemname : super::PUNICODE_STRING, handleid : *const core::ffi::c_void, clienttoken : super::HANDLE, desiredaccess : super::ACCESS_MASK, privileges : super::PPRIVILEGE_SET, accessgranted : super::BOOLEAN) -> windows_core::NTSTATUS);
    unsafe { NtPrivilegeObjectAuditAlarm(subsystemname, handleid.unwrap_or(core::mem::zeroed()) as _, clienttoken, desiredaccess, privileges, accessgranted) }
}
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn NtPrivilegedServiceAuditAlarm(subsystemname: super::PUNICODE_STRING, servicename: super::PUNICODE_STRING, clienttoken: super::HANDLE, privileges: super::PPRIVILEGE_SET, accessgranted: super::BOOLEAN) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtPrivilegedServiceAuditAlarm(subsystemname : super::PUNICODE_STRING, servicename : super::PUNICODE_STRING, clienttoken : super::HANDLE, privileges : super::PPRIVILEGE_SET, accessgranted : super::BOOLEAN) -> windows_core::NTSTATUS);
    unsafe { NtPrivilegedServiceAuditAlarm(subsystemname, servicename, clienttoken, privileges, accessgranted) }
}
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "wdm", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn NtQueryDirectoryFile(filehandle: super::HANDLE, event: Option<super::HANDLE>, apcroutine: super::PIO_APC_ROUTINE, apccontext: Option<*const core::ffi::c_void>, iostatusblock: super::PIO_STATUS_BLOCK, fileinformation: *mut core::ffi::c_void, length: u32, fileinformationclass: super::FILE_INFORMATION_CLASS, returnsingleentry: super::BOOLEAN, filename: Option<super::PUNICODE_STRING>, restartscan: super::BOOLEAN) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtQueryDirectoryFile(filehandle : super::HANDLE, event : super::HANDLE, apcroutine : super::PIO_APC_ROUTINE, apccontext : *const core::ffi::c_void, iostatusblock : super::PIO_STATUS_BLOCK, fileinformation : *mut core::ffi::c_void, length : u32, fileinformationclass : super::FILE_INFORMATION_CLASS, returnsingleentry : super::BOOLEAN, filename : super::PUNICODE_STRING, restartscan : super::BOOLEAN) -> windows_core::NTSTATUS);
    unsafe { NtQueryDirectoryFile(filehandle, event.unwrap_or(core::mem::zeroed()) as _, apcroutine, apccontext.unwrap_or(core::mem::zeroed()) as _, iostatusblock as _, fileinformation as _, length, fileinformationclass, returnsingleentry, filename.unwrap_or(core::mem::zeroed()) as _, restartscan) }
}
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "wdm", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn NtQueryDirectoryFileEx(filehandle: super::HANDLE, event: Option<super::HANDLE>, apcroutine: super::PIO_APC_ROUTINE, apccontext: Option<*const core::ffi::c_void>, iostatusblock: super::PIO_STATUS_BLOCK, fileinformation: *mut core::ffi::c_void, length: u32, fileinformationclass: super::FILE_INFORMATION_CLASS, queryflags: u32, filename: Option<super::PUNICODE_STRING>) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtQueryDirectoryFileEx(filehandle : super::HANDLE, event : super::HANDLE, apcroutine : super::PIO_APC_ROUTINE, apccontext : *const core::ffi::c_void, iostatusblock : super::PIO_STATUS_BLOCK, fileinformation : *mut core::ffi::c_void, length : u32, fileinformationclass : super::FILE_INFORMATION_CLASS, queryflags : u32, filename : super::PUNICODE_STRING) -> windows_core::NTSTATUS);
    unsafe { NtQueryDirectoryFileEx(filehandle, event.unwrap_or(core::mem::zeroed()) as _, apcroutine, apccontext.unwrap_or(core::mem::zeroed()) as _, iostatusblock as _, fileinformation as _, length, fileinformationclass, queryflags, filename.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "d3dkmthk", feature = "lsalookup", feature = "ntsecapi", feature = "wdm", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn NtQueryInformationByName(objectattributes: super::POBJECT_ATTRIBUTES, iostatusblock: super::PIO_STATUS_BLOCK, fileinformation: *mut core::ffi::c_void, length: u32, fileinformationclass: super::FILE_INFORMATION_CLASS) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtQueryInformationByName(objectattributes : super::POBJECT_ATTRIBUTES, iostatusblock : super::PIO_STATUS_BLOCK, fileinformation : *mut core::ffi::c_void, length : u32, fileinformationclass : super::FILE_INFORMATION_CLASS) -> windows_core::NTSTATUS);
    unsafe { NtQueryInformationByName(objectattributes, iostatusblock as _, fileinformation as _, length, fileinformationclass) }
}
#[cfg(all(feature = "wdm", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn NtQueryInformationFile(filehandle: super::HANDLE, iostatusblock: super::PIO_STATUS_BLOCK, fileinformation: *mut core::ffi::c_void, length: u32, fileinformationclass: super::FILE_INFORMATION_CLASS) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtQueryInformationFile(filehandle : super::HANDLE, iostatusblock : super::PIO_STATUS_BLOCK, fileinformation : *mut core::ffi::c_void, length : u32, fileinformationclass : super::FILE_INFORMATION_CLASS) -> windows_core::NTSTATUS);
    unsafe { NtQueryInformationFile(filehandle, iostatusblock as _, fileinformation as _, length, fileinformationclass) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn NtQueryInformationToken(tokenhandle: super::HANDLE, tokeninformationclass: super::TOKEN_INFORMATION_CLASS, tokeninformation: Option<*mut core::ffi::c_void>, tokeninformationlength: u32, returnlength: super::PULONG) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtQueryInformationToken(tokenhandle : super::HANDLE, tokeninformationclass : super::TOKEN_INFORMATION_CLASS, tokeninformation : *mut core::ffi::c_void, tokeninformationlength : u32, returnlength : super::PULONG) -> windows_core::NTSTATUS);
    unsafe { NtQueryInformationToken(tokenhandle, tokeninformationclass, tokeninformation.unwrap_or(core::mem::zeroed()) as _, tokeninformationlength, returnlength as _) }
}
#[cfg(all(feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn NtQueryQuotaInformationFile(filehandle: super::HANDLE, iostatusblock: super::PIO_STATUS_BLOCK, buffer: *mut core::ffi::c_void, length: u32, returnsingleentry: super::BOOLEAN, sidlist: Option<*const core::ffi::c_void>, sidlistlength: u32, startsid: Option<super::PSID>, restartscan: super::BOOLEAN) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtQueryQuotaInformationFile(filehandle : super::HANDLE, iostatusblock : super::PIO_STATUS_BLOCK, buffer : *mut core::ffi::c_void, length : u32, returnsingleentry : super::BOOLEAN, sidlist : *const core::ffi::c_void, sidlistlength : u32, startsid : super::PSID, restartscan : super::BOOLEAN) -> windows_core::NTSTATUS);
    unsafe { NtQueryQuotaInformationFile(filehandle, iostatusblock as _, buffer as _, length, returnsingleentry, sidlist.unwrap_or(core::mem::zeroed()) as _, sidlistlength, startsid.unwrap_or(core::mem::zeroed()) as _, restartscan) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn NtQuerySecurityObject(handle: super::HANDLE, securityinformation: super::SECURITY_INFORMATION, securitydescriptor: Option<super::PSECURITY_DESCRIPTOR>, length: u32, lengthneeded: super::PULONG) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtQuerySecurityObject(handle : super::HANDLE, securityinformation : super::SECURITY_INFORMATION, securitydescriptor : super::PSECURITY_DESCRIPTOR, length : u32, lengthneeded : super::PULONG) -> windows_core::NTSTATUS);
    unsafe { NtQuerySecurityObject(handle, securityinformation, securitydescriptor.unwrap_or(core::mem::zeroed()) as _, length, lengthneeded as _) }
}
#[cfg(all(feature = "basetsd", feature = "winnt"))]
#[inline]
pub unsafe fn NtQueryVirtualMemory(processhandle: super::HANDLE, baseaddress: Option<*const core::ffi::c_void>, memoryinformationclass: MEMORY_INFORMATION_CLASS, memoryinformation: *mut core::ffi::c_void, memoryinformationlength: usize, returnlength: Option<super::PSIZE_T>) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtQueryVirtualMemory(processhandle : super::HANDLE, baseaddress : *const core::ffi::c_void, memoryinformationclass : MEMORY_INFORMATION_CLASS, memoryinformation : *mut core::ffi::c_void, memoryinformationlength : usize, returnlength : super::PSIZE_T) -> windows_core::NTSTATUS);
    unsafe { NtQueryVirtualMemory(processhandle, baseaddress.unwrap_or(core::mem::zeroed()) as _, memoryinformationclass, memoryinformation as _, memoryinformationlength, returnlength.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "wdm", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn NtQueryVolumeInformationFile(filehandle: super::HANDLE, iostatusblock: super::PIO_STATUS_BLOCK, fsinformation: *mut core::ffi::c_void, length: u32, fsinformationclass: super::FS_INFORMATION_CLASS) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtQueryVolumeInformationFile(filehandle : super::HANDLE, iostatusblock : super::PIO_STATUS_BLOCK, fsinformation : *mut core::ffi::c_void, length : u32, fsinformationclass : super::FS_INFORMATION_CLASS) -> windows_core::NTSTATUS);
    unsafe { NtQueryVolumeInformationFile(filehandle, iostatusblock as _, fsinformation as _, length, fsinformationclass) }
}
#[cfg(all(feature = "minwindef", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn NtReadFile(filehandle: super::HANDLE, event: Option<super::HANDLE>, apcroutine: super::PIO_APC_ROUTINE, apccontext: Option<*const core::ffi::c_void>, iostatusblock: super::PIO_STATUS_BLOCK, buffer: *mut core::ffi::c_void, length: u32, byteoffset: Option<super::PLARGE_INTEGER>, key: Option<super::PULONG>) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtReadFile(filehandle : super::HANDLE, event : super::HANDLE, apcroutine : super::PIO_APC_ROUTINE, apccontext : *const core::ffi::c_void, iostatusblock : super::PIO_STATUS_BLOCK, buffer : *mut core::ffi::c_void, length : u32, byteoffset : super::PLARGE_INTEGER, key : super::PULONG) -> windows_core::NTSTATUS);
    unsafe { NtReadFile(filehandle, event.unwrap_or(core::mem::zeroed()) as _, apcroutine, apccontext.unwrap_or(core::mem::zeroed()) as _, iostatusblock as _, buffer as _, length, byteoffset.unwrap_or(core::mem::zeroed()) as _, key.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "wdm", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn NtSetInformationFile(filehandle: super::HANDLE, iostatusblock: super::PIO_STATUS_BLOCK, fileinformation: *const core::ffi::c_void, length: u32, fileinformationclass: super::FILE_INFORMATION_CLASS) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtSetInformationFile(filehandle : super::HANDLE, iostatusblock : super::PIO_STATUS_BLOCK, fileinformation : *const core::ffi::c_void, length : u32, fileinformationclass : super::FILE_INFORMATION_CLASS) -> windows_core::NTSTATUS);
    unsafe { NtSetInformationFile(filehandle, iostatusblock as _, fileinformation, length, fileinformationclass) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn NtSetInformationToken(tokenhandle: super::HANDLE, tokeninformationclass: super::TOKEN_INFORMATION_CLASS, tokeninformation: *const core::ffi::c_void, tokeninformationlength: u32) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtSetInformationToken(tokenhandle : super::HANDLE, tokeninformationclass : super::TOKEN_INFORMATION_CLASS, tokeninformation : *const core::ffi::c_void, tokeninformationlength : u32) -> windows_core::NTSTATUS);
    unsafe { NtSetInformationToken(tokenhandle, tokeninformationclass, tokeninformation, tokeninformationlength) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn NtSetInformationVirtualMemory(processhandle: super::HANDLE, vminformationclass: VIRTUAL_MEMORY_INFORMATION_CLASS, virtualaddresses: &[MEMORY_RANGE_ENTRY], vminformation: *const core::ffi::c_void, vminformationlength: u32) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtSetInformationVirtualMemory(processhandle : super::HANDLE, vminformationclass : VIRTUAL_MEMORY_INFORMATION_CLASS, numberofentries : usize, virtualaddresses : PMEMORY_RANGE_ENTRY, vminformation : *const core::ffi::c_void, vminformationlength : u32) -> windows_core::NTSTATUS);
    unsafe { NtSetInformationVirtualMemory(processhandle, vminformationclass, virtualaddresses.len().try_into().unwrap(), core::mem::transmute(virtualaddresses.as_ptr()), vminformation, vminformationlength) }
}
#[cfg(all(feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn NtSetQuotaInformationFile(filehandle: super::HANDLE, iostatusblock: super::PIO_STATUS_BLOCK, buffer: *const core::ffi::c_void, length: u32) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtSetQuotaInformationFile(filehandle : super::HANDLE, iostatusblock : super::PIO_STATUS_BLOCK, buffer : *const core::ffi::c_void, length : u32) -> windows_core::NTSTATUS);
    unsafe { NtSetQuotaInformationFile(filehandle, iostatusblock as _, buffer, length) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn NtSetSecurityObject(handle: super::HANDLE, securityinformation: super::SECURITY_INFORMATION, securitydescriptor: super::PSECURITY_DESCRIPTOR) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtSetSecurityObject(handle : super::HANDLE, securityinformation : super::SECURITY_INFORMATION, securitydescriptor : super::PSECURITY_DESCRIPTOR) -> windows_core::NTSTATUS);
    unsafe { NtSetSecurityObject(handle, securityinformation, securitydescriptor) }
}
#[cfg(all(feature = "wdm", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn NtSetVolumeInformationFile(filehandle: super::HANDLE, iostatusblock: super::PIO_STATUS_BLOCK, fsinformation: *const core::ffi::c_void, length: u32, fsinformationclass: super::FS_INFORMATION_CLASS) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtSetVolumeInformationFile(filehandle : super::HANDLE, iostatusblock : super::PIO_STATUS_BLOCK, fsinformation : *const core::ffi::c_void, length : u32, fsinformationclass : super::FS_INFORMATION_CLASS) -> windows_core::NTSTATUS);
    unsafe { NtSetVolumeInformationFile(filehandle, iostatusblock as _, fsinformation, length, fsinformationclass) }
}
#[cfg(all(feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn NtUnlockFile(filehandle: super::HANDLE, iostatusblock: super::PIO_STATUS_BLOCK, byteoffset: super::PLARGE_INTEGER, length: super::PLARGE_INTEGER, key: u32) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtUnlockFile(filehandle : super::HANDLE, iostatusblock : super::PIO_STATUS_BLOCK, byteoffset : super::PLARGE_INTEGER, length : super::PLARGE_INTEGER, key : u32) -> windows_core::NTSTATUS);
    unsafe { NtUnlockFile(filehandle, iostatusblock as _, byteoffset, length, key) }
}
#[cfg(all(feature = "minwindef", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn NtWriteFile(filehandle: super::HANDLE, event: Option<super::HANDLE>, apcroutine: super::PIO_APC_ROUTINE, apccontext: Option<*const core::ffi::c_void>, iostatusblock: super::PIO_STATUS_BLOCK, buffer: *const core::ffi::c_void, length: u32, byteoffset: Option<super::PLARGE_INTEGER>, key: Option<super::PULONG>) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtWriteFile(filehandle : super::HANDLE, event : super::HANDLE, apcroutine : super::PIO_APC_ROUTINE, apccontext : *const core::ffi::c_void, iostatusblock : super::PIO_STATUS_BLOCK, buffer : *const core::ffi::c_void, length : u32, byteoffset : super::PLARGE_INTEGER, key : super::PULONG) -> windows_core::NTSTATUS);
    unsafe { NtWriteFile(filehandle, event.unwrap_or(core::mem::zeroed()) as _, apcroutine, apccontext.unwrap_or(core::mem::zeroed()) as _, iostatusblock as _, buffer, length, byteoffset.unwrap_or(core::mem::zeroed()) as _, key.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "lsalookup", feature = "ntddk", feature = "ntdef", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn PfxFindPrefix(prefixtable: PPREFIX_TABLE, fullname: super::PSTRING) -> PPREFIX_TABLE_ENTRY {
    windows_core::link!("ntdll.dll" "system" fn PfxFindPrefix(prefixtable : PPREFIX_TABLE, fullname : super::PSTRING) -> PPREFIX_TABLE_ENTRY);
    unsafe { PfxFindPrefix(prefixtable, fullname) }
}
#[cfg(all(feature = "lsalookup", feature = "ntddk", feature = "ntdef", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn PfxInitialize() -> PREFIX_TABLE {
    windows_core::link!("ntdll.dll" "system" fn PfxInitialize(prefixtable : PPREFIX_TABLE));
    unsafe {
        let mut result__ = core::mem::zeroed();
        PfxInitialize(&mut result__);
        result__
    }
}
#[cfg(all(feature = "lsalookup", feature = "ntddk", feature = "ntdef", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn PfxInsertPrefix(prefixtable: PPREFIX_TABLE, prefix: super::PSTRING, prefixtableentry: PPREFIX_TABLE_ENTRY) -> super::BOOLEAN {
    windows_core::link!("ntdll.dll" "system" fn PfxInsertPrefix(prefixtable : PPREFIX_TABLE, prefix : super::PSTRING, prefixtableentry : PPREFIX_TABLE_ENTRY) -> super::BOOLEAN);
    unsafe { PfxInsertPrefix(prefixtable, prefix, prefixtableentry as _) }
}
#[cfg(all(feature = "lsalookup", feature = "ntddk", feature = "ntdef", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn PfxRemovePrefix(prefixtable: PPREFIX_TABLE, prefixtableentry: PPREFIX_TABLE_ENTRY) {
    windows_core::link!("ntdll.dll" "system" fn PfxRemovePrefix(prefixtable : PPREFIX_TABLE, prefixtableentry : PPREFIX_TABLE_ENTRY));
    unsafe { PfxRemovePrefix(prefixtable, prefixtableentry) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn RtlAbsoluteToSelfRelativeSD(absolutesecuritydescriptor: super::PSECURITY_DESCRIPTOR, selfrelativesecuritydescriptor: Option<super::PSECURITY_DESCRIPTOR>, bufferlength: super::PULONG) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlAbsoluteToSelfRelativeSD(absolutesecuritydescriptor : super::PSECURITY_DESCRIPTOR, selfrelativesecuritydescriptor : super::PSECURITY_DESCRIPTOR, bufferlength : super::PULONG) -> windows_core::NTSTATUS);
    unsafe { RtlAbsoluteToSelfRelativeSD(absolutesecuritydescriptor, selfrelativesecuritydescriptor.unwrap_or(core::mem::zeroed()) as _, bufferlength as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RtlAddAccessAllowedAce(acl: super::PACL, acerevision: u32, accessmask: super::ACCESS_MASK, sid: super::PSID) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlAddAccessAllowedAce(acl : super::PACL, acerevision : u32, accessmask : super::ACCESS_MASK, sid : super::PSID) -> windows_core::NTSTATUS);
    unsafe { RtlAddAccessAllowedAce(acl as _, acerevision, accessmask, sid) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RtlAddAccessAllowedAceEx(acl: super::PACL, acerevision: u32, aceflags: u32, accessmask: super::ACCESS_MASK, sid: super::PSID) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlAddAccessAllowedAceEx(acl : super::PACL, acerevision : u32, aceflags : u32, accessmask : super::ACCESS_MASK, sid : super::PSID) -> windows_core::NTSTATUS);
    unsafe { RtlAddAccessAllowedAceEx(acl as _, acerevision, aceflags, accessmask, sid) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RtlAddAce(acl: super::PACL, acerevision: u32, startingaceindex: u32, acelist: *const core::ffi::c_void, acelistlength: u32) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlAddAce(acl : super::PACL, acerevision : u32, startingaceindex : u32, acelist : *const core::ffi::c_void, acelistlength : u32) -> windows_core::NTSTATUS);
    unsafe { RtlAddAce(acl as _, acerevision, startingaceindex, acelist, acelistlength) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RtlAllocateAndInitializeSid(identifierauthority: super::PSID_IDENTIFIER_AUTHORITY, subauthoritycount: u8, subauthority0: u32, subauthority1: u32, subauthority2: u32, subauthority3: u32, subauthority4: u32, subauthority5: u32, subauthority6: u32, subauthority7: u32, sid: *mut super::PSID) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlAllocateAndInitializeSid(identifierauthority : super::PSID_IDENTIFIER_AUTHORITY, subauthoritycount : u8, subauthority0 : u32, subauthority1 : u32, subauthority2 : u32, subauthority3 : u32, subauthority4 : u32, subauthority5 : u32, subauthority6 : u32, subauthority7 : u32, sid : *mut super::PSID) -> windows_core::NTSTATUS);
    unsafe { RtlAllocateAndInitializeSid(identifierauthority, subauthoritycount, subauthority0, subauthority1, subauthority2, subauthority3, subauthority4, subauthority5, subauthority6, subauthority7, sid as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn RtlAllocateAndInitializeSidEx(identifierauthority: super::PSID_IDENTIFIER_AUTHORITY, subauthorities: &[u32], sid: *mut super::PSID) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlAllocateAndInitializeSidEx(identifierauthority : super::PSID_IDENTIFIER_AUTHORITY, subauthoritycount : u8, subauthorities : super::PULONG, sid : *mut super::PSID) -> windows_core::NTSTATUS);
    unsafe { RtlAllocateAndInitializeSidEx(identifierauthority, subauthorities.len().try_into().unwrap(), core::mem::transmute(subauthorities.as_ptr()), sid as _) }
}
#[inline]
pub unsafe fn RtlAllocateHeap(heaphandle: *const core::ffi::c_void, flags: Option<u32>, size: usize) -> *mut core::ffi::c_void {
    windows_core::link!("ntdll.dll" "system" fn RtlAllocateHeap(heaphandle : *const core::ffi::c_void, flags : u32, size : usize) -> *mut core::ffi::c_void);
    unsafe { RtlAllocateHeap(heaphandle, flags.unwrap_or(core::mem::zeroed()) as _, size) }
}
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn RtlAppendStringToString(destination: super::PSTRING, source: *const super::STRING) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlAppendStringToString(destination : super::PSTRING, source : *const super::STRING) -> windows_core::NTSTATUS);
    unsafe { RtlAppendStringToString(destination as _, source) }
}
#[cfg(feature = "winternl")]
#[inline]
pub unsafe fn RtlCompareAltitudes(altitude1: super::PCUNICODE_STRING, altitude2: super::PCUNICODE_STRING) -> i32 {
    windows_core::link!("ntdll.dll" "system" fn RtlCompareAltitudes(altitude1 : super::PCUNICODE_STRING, altitude2 : super::PCUNICODE_STRING) -> i32);
    unsafe { RtlCompareAltitudes(altitude1, altitude2) }
}
#[inline]
pub unsafe fn RtlCompareMemoryUlong(source: *const core::ffi::c_void, length: usize, pattern: u32) -> usize {
    windows_core::link!("ntdll.dll" "system" fn RtlCompareMemoryUlong(source : *const core::ffi::c_void, length : usize, pattern : u32) -> usize);
    unsafe { RtlCompareMemoryUlong(source, length, pattern) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn RtlCompressBuffer(compressionformatandengine: u16, uncompressedbuffer: &[u8], compressedbuffer: super::PUCHAR, compressedbuffersize: u32, uncompressedchunksize: u32, finalcompressedsize: super::PULONG, workspace: *const core::ffi::c_void) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlCompressBuffer(compressionformatandengine : u16, uncompressedbuffer : super::PUCHAR, uncompressedbuffersize : u32, compressedbuffer : super::PUCHAR, compressedbuffersize : u32, uncompressedchunksize : u32, finalcompressedsize : super::PULONG, workspace : *const core::ffi::c_void) -> windows_core::NTSTATUS);
    unsafe { RtlCompressBuffer(compressionformatandengine, core::mem::transmute(uncompressedbuffer.as_ptr()), uncompressedbuffer.len().try_into().unwrap(), compressedbuffer as _, compressedbuffersize, uncompressedchunksize, finalcompressedsize as _, workspace) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RtlCopyLuid(destinationluid: super::PLUID, sourceluid: super::PLUID) {
    windows_core::link!("ntdll.dll" "system" fn RtlCopyLuid(destinationluid : super::PLUID, sourceluid : super::PLUID));
    unsafe { RtlCopyLuid(destinationluid as _, sourceluid) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RtlCopySid(destinationsidlength: u32, destinationsid: super::PSID, sourcesid: super::PSID) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlCopySid(destinationsidlength : u32, destinationsid : super::PSID, sourcesid : super::PSID) -> windows_core::NTSTATUS);
    unsafe { RtlCopySid(destinationsidlength, destinationsid as _, sourcesid) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RtlCreateAcl(acl: super::PACL, acllength: u32, aclrevision: u32) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlCreateAcl(acl : super::PACL, acllength : u32, aclrevision : u32) -> windows_core::NTSTATUS);
    unsafe { RtlCreateAcl(acl as _, acllength, aclrevision) }
}
#[cfg(feature = "basetsd")]
#[inline]
pub unsafe fn RtlCreateHeap(flags: u32, heapbase: Option<*const core::ffi::c_void>, reservesize: Option<usize>, commitsize: Option<usize>, lock: Option<*const core::ffi::c_void>, parameters: PRTL_HEAP_PARAMETERS) -> *mut core::ffi::c_void {
    windows_core::link!("ntdll.dll" "system" fn RtlCreateHeap(flags : u32, heapbase : *const core::ffi::c_void, reservesize : usize, commitsize : usize, lock : *const core::ffi::c_void, parameters : PRTL_HEAP_PARAMETERS) -> *mut core::ffi::c_void);
    unsafe { RtlCreateHeap(flags, heapbase.unwrap_or(core::mem::zeroed()) as _, reservesize.unwrap_or(core::mem::zeroed()) as _, commitsize.unwrap_or(core::mem::zeroed()) as _, lock.unwrap_or(core::mem::zeroed()) as _, parameters) }
}
#[cfg(all(feature = "lsalookup", feature = "minwindef", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn RtlCreateServiceSid(servicename: super::PUNICODE_STRING, servicesid: Option<super::PSID>, servicesidlength: super::PULONG) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlCreateServiceSid(servicename : super::PUNICODE_STRING, servicesid : super::PSID, servicesidlength : super::PULONG) -> windows_core::NTSTATUS);
    unsafe { RtlCreateServiceSid(servicename, servicesid.unwrap_or(core::mem::zeroed()) as _, servicesidlength as _) }
}
#[cfg(feature = "winternl")]
#[inline]
pub unsafe fn RtlCreateSystemVolumeInformationFolder(volumerootpath: super::PCUNICODE_STRING) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlCreateSystemVolumeInformationFolder(volumerootpath : super::PCUNICODE_STRING) -> windows_core::NTSTATUS);
    unsafe { RtlCreateSystemVolumeInformationFolder(volumerootpath) }
}
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn RtlCreateUnicodeString<P1>(destinationstring: super::PUNICODE_STRING, sourcestring: P1) -> super::BOOLEAN
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ntdll.dll" "system" fn RtlCreateUnicodeString(destinationstring : super::PUNICODE_STRING, sourcestring : windows_core::PCWSTR) -> super::BOOLEAN);
    unsafe { RtlCreateUnicodeString(destinationstring as _, sourcestring.param().abi()) }
}
#[cfg(all(feature = "minwindef", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn RtlCreateVirtualAccountSid(name: super::PCUNICODE_STRING, basesubauthority: u32, sid: super::PSID, sidlength: super::PULONG) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlCreateVirtualAccountSid(name : super::PCUNICODE_STRING, basesubauthority : u32, sid : super::PSID, sidlength : super::PULONG) -> windows_core::NTSTATUS);
    unsafe { RtlCreateVirtualAccountSid(name, basesubauthority, sid as _, sidlength as _) }
}
#[cfg(all(feature = "minwindef", feature = "ntnls", feature = "winnt"))]
#[inline]
pub unsafe fn RtlCustomCPToUnicodeN(customcp: super::PCPTABLEINFO, unicodestring: super::PWCH, maxbytesinunicodestring: u32, bytesinunicodestring: Option<super::PULONG>, customcpstring: &[u8]) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlCustomCPToUnicodeN(customcp : super::PCPTABLEINFO, unicodestring : super::PWCH, maxbytesinunicodestring : u32, bytesinunicodestring : super::PULONG, customcpstring : super::PCH, bytesincustomcpstring : u32) -> windows_core::NTSTATUS);
    unsafe { RtlCustomCPToUnicodeN(customcp, unicodestring as _, maxbytesinunicodestring, bytesinunicodestring.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(customcpstring.as_ptr()), customcpstring.len().try_into().unwrap()) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn RtlDecompressBuffer(compressionformat: u16, uncompressedbuffer: super::PUCHAR, uncompressedbuffersize: u32, compressedbuffer: &[u8], finaluncompressedsize: super::PULONG) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlDecompressBuffer(compressionformat : u16, uncompressedbuffer : super::PUCHAR, uncompressedbuffersize : u32, compressedbuffer : super::PUCHAR, compressedbuffersize : u32, finaluncompressedsize : super::PULONG) -> windows_core::NTSTATUS);
    unsafe { RtlDecompressBuffer(compressionformat, uncompressedbuffer as _, uncompressedbuffersize, core::mem::transmute(compressedbuffer.as_ptr()), compressedbuffer.len().try_into().unwrap(), finaluncompressedsize as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn RtlDecompressBufferEx(compressionformat: u16, uncompressedbuffer: super::PUCHAR, uncompressedbuffersize: u32, compressedbuffer: &[u8], finaluncompressedsize: super::PULONG, workspace: Option<*const core::ffi::c_void>) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlDecompressBufferEx(compressionformat : u16, uncompressedbuffer : super::PUCHAR, uncompressedbuffersize : u32, compressedbuffer : super::PUCHAR, compressedbuffersize : u32, finaluncompressedsize : super::PULONG, workspace : *const core::ffi::c_void) -> windows_core::NTSTATUS);
    unsafe { RtlDecompressBufferEx(compressionformat, uncompressedbuffer as _, uncompressedbuffersize, core::mem::transmute(compressedbuffer.as_ptr()), compressedbuffer.len().try_into().unwrap(), finaluncompressedsize as _, workspace.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn RtlDecompressFragment(compressionformat: u16, uncompressedfragment: super::PUCHAR, uncompressedfragmentsize: u32, compressedbuffer: &[u8], fragmentoffset: u32, finaluncompressedsize: super::PULONG, workspace: *const core::ffi::c_void) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlDecompressFragment(compressionformat : u16, uncompressedfragment : super::PUCHAR, uncompressedfragmentsize : u32, compressedbuffer : super::PUCHAR, compressedbuffersize : u32, fragmentoffset : u32, finaluncompressedsize : super::PULONG, workspace : *const core::ffi::c_void) -> windows_core::NTSTATUS);
    unsafe { RtlDecompressFragment(compressionformat, uncompressedfragment as _, uncompressedfragmentsize, core::mem::transmute(compressedbuffer.as_ptr()), compressedbuffer.len().try_into().unwrap(), fragmentoffset, finaluncompressedsize as _, workspace) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RtlDeleteAce(acl: super::PACL, aceindex: u32) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlDeleteAce(acl : super::PACL, aceindex : u32) -> windows_core::NTSTATUS);
    unsafe { RtlDeleteAce(acl as _, aceindex) }
}
#[inline]
pub unsafe fn RtlDestroyHeap(heaphandle: *const core::ffi::c_void) -> *mut core::ffi::c_void {
    windows_core::link!("ntdll.dll" "system" fn RtlDestroyHeap(heaphandle : *const core::ffi::c_void) -> *mut core::ffi::c_void);
    unsafe { RtlDestroyHeap(heaphandle) }
}
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn RtlDowncaseUnicodeString(destinationstring: super::PUNICODE_STRING, sourcestring: super::PCUNICODE_STRING, allocatedestinationstring: super::BOOLEAN) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlDowncaseUnicodeString(destinationstring : super::PUNICODE_STRING, sourcestring : super::PCUNICODE_STRING, allocatedestinationstring : super::BOOLEAN) -> windows_core::NTSTATUS);
    unsafe { RtlDowncaseUnicodeString(destinationstring, sourcestring, allocatedestinationstring) }
}
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winternl"))]
#[inline]
pub unsafe fn RtlDuplicateUnicodeString(flags: u32, stringin: super::PCUNICODE_STRING, stringout: super::PUNICODE_STRING) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlDuplicateUnicodeString(flags : u32, stringin : super::PCUNICODE_STRING, stringout : super::PUNICODE_STRING) -> windows_core::NTSTATUS);
    unsafe { RtlDuplicateUnicodeString(flags, stringin, stringout as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RtlEqualPrefixSid(sid1: super::PSID, sid2: super::PSID) -> super::BOOLEAN {
    windows_core::link!("ntdll.dll" "system" fn RtlEqualPrefixSid(sid1 : super::PSID, sid2 : super::PSID) -> super::BOOLEAN);
    unsafe { RtlEqualPrefixSid(sid1, sid2) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RtlEqualSid(sid1: super::PSID, sid2: super::PSID) -> super::BOOLEAN {
    windows_core::link!("ntdll.dll" "system" fn RtlEqualSid(sid1 : super::PSID, sid2 : super::PSID) -> super::BOOLEAN);
    unsafe { RtlEqualSid(sid1, sid2) }
}
#[cfg(feature = "ntdef")]
#[inline]
pub unsafe fn RtlFreeHeap(heaphandle: *const core::ffi::c_void, flags: Option<u32>, baseaddress: *mut core::ffi::c_void) -> super::LOGICAL {
    windows_core::link!("ntdll.dll" "system" fn RtlFreeHeap(heaphandle : *const core::ffi::c_void, flags : u32, baseaddress : *mut core::ffi::c_void) -> super::LOGICAL);
    unsafe { RtlFreeHeap(heaphandle, flags.unwrap_or(core::mem::zeroed()) as _, baseaddress as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RtlFreeSid(sid: super::PSID) -> *mut core::ffi::c_void {
    windows_core::link!("ntdll.dll" "system" fn RtlFreeSid(sid : super::PSID) -> *mut core::ffi::c_void);
    unsafe { RtlFreeSid(sid) }
}
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn RtlGenerate8dot3Name(name: super::PCUNICODE_STRING, allowextendedcharacters: super::BOOLEAN, context: PGENERATE_NAME_CONTEXT, name8dot3: super::PUNICODE_STRING) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlGenerate8dot3Name(name : super::PCUNICODE_STRING, allowextendedcharacters : super::BOOLEAN, context : PGENERATE_NAME_CONTEXT, name8dot3 : super::PUNICODE_STRING) -> windows_core::NTSTATUS);
    unsafe { RtlGenerate8dot3Name(name, allowextendedcharacters, context as _, name8dot3 as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RtlGetAce(acl: super::PACL, aceindex: u32, ace: *mut *mut core::ffi::c_void) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlGetAce(acl : super::PACL, aceindex : u32, ace : *mut *mut core::ffi::c_void) -> windows_core::NTSTATUS);
    unsafe { RtlGetAce(acl, aceindex, ace as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn RtlGetAcesBufferSize(acl: super::PACL, acesbuffersize: super::PULONG) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlGetAcesBufferSize(acl : super::PACL, acesbuffersize : super::PULONG) -> windows_core::NTSTATUS);
    unsafe { RtlGetAcesBufferSize(acl, acesbuffersize as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn RtlGetCompressionWorkSpaceSize(compressionformatandengine: u16, compressbufferworkspacesize: super::PULONG, compressfragmentworkspacesize: super::PULONG) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlGetCompressionWorkSpaceSize(compressionformatandengine : u16, compressbufferworkspacesize : super::PULONG, compressfragmentworkspacesize : super::PULONG) -> windows_core::NTSTATUS);
    unsafe { RtlGetCompressionWorkSpaceSize(compressionformatandengine, compressbufferworkspacesize as _, compressfragmentworkspacesize as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RtlGetDaclSecurityDescriptor(securitydescriptor: super::PSECURITY_DESCRIPTOR, daclpresent: super::PBOOLEAN, dacl: *mut super::PACL, dacldefaulted: super::PBOOLEAN) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlGetDaclSecurityDescriptor(securitydescriptor : super::PSECURITY_DESCRIPTOR, daclpresent : super::PBOOLEAN, dacl : *mut super::PACL, dacldefaulted : super::PBOOLEAN) -> windows_core::NTSTATUS);
    unsafe { RtlGetDaclSecurityDescriptor(securitydescriptor, daclpresent as _, dacl as _, dacldefaulted) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RtlGetGroupSecurityDescriptor(securitydescriptor: super::PSECURITY_DESCRIPTOR, group: *mut super::PSID, groupdefaulted: super::PBOOLEAN) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlGetGroupSecurityDescriptor(securitydescriptor : super::PSECURITY_DESCRIPTOR, group : *mut super::PSID, groupdefaulted : super::PBOOLEAN) -> windows_core::NTSTATUS);
    unsafe { RtlGetGroupSecurityDescriptor(securitydescriptor, group as _, groupdefaulted) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RtlGetOwnerSecurityDescriptor(securitydescriptor: super::PSECURITY_DESCRIPTOR, owner: *mut super::PSID, ownerdefaulted: super::PBOOLEAN) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlGetOwnerSecurityDescriptor(securitydescriptor : super::PSECURITY_DESCRIPTOR, owner : *mut super::PSID, ownerdefaulted : super::PBOOLEAN) -> windows_core::NTSTATUS);
    unsafe { RtlGetOwnerSecurityDescriptor(securitydescriptor, owner as _, ownerdefaulted) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RtlGetSaclSecurityDescriptor(securitydescriptor: super::PSECURITY_DESCRIPTOR, saclpresent: super::PBOOLEAN, sacl: *mut super::PACL, sacldefaulted: super::PBOOLEAN) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlGetSaclSecurityDescriptor(securitydescriptor : super::PSECURITY_DESCRIPTOR, saclpresent : super::PBOOLEAN, sacl : *mut super::PACL, sacldefaulted : super::PBOOLEAN) -> windows_core::NTSTATUS);
    unsafe { RtlGetSaclSecurityDescriptor(securitydescriptor, saclpresent as _, sacl as _, sacldefaulted as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RtlIdentifierAuthoritySid(sid: super::PSID) -> super::PSID_IDENTIFIER_AUTHORITY {
    windows_core::link!("ntdll.dll" "system" fn RtlIdentifierAuthoritySid(sid : super::PSID) -> super::PSID_IDENTIFIER_AUTHORITY);
    unsafe { RtlIdentifierAuthoritySid(sid) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RtlIdnToAscii<P1>(flags: u32, sourcestring: P1, sourcestringlength: i32, destinationstring: windows_core::PWSTR, destinationstringlength: super::PLONG) -> windows_core::NTSTATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ntdll.dll" "system" fn RtlIdnToAscii(flags : u32, sourcestring : windows_core::PCWSTR, sourcestringlength : i32, destinationstring : windows_core::PWSTR, destinationstringlength : super::PLONG) -> windows_core::NTSTATUS);
    unsafe { RtlIdnToAscii(flags, sourcestring.param().abi(), sourcestringlength, destinationstring, destinationstringlength as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RtlIdnToNameprepUnicode<P1>(flags: u32, sourcestring: P1, sourcestringlength: i32, destinationstring: windows_core::PWSTR, destinationstringlength: super::PLONG) -> windows_core::NTSTATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ntdll.dll" "system" fn RtlIdnToNameprepUnicode(flags : u32, sourcestring : windows_core::PCWSTR, sourcestringlength : i32, destinationstring : windows_core::PWSTR, destinationstringlength : super::PLONG) -> windows_core::NTSTATUS);
    unsafe { RtlIdnToNameprepUnicode(flags, sourcestring.param().abi(), sourcestringlength, destinationstring, destinationstringlength as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RtlIdnToUnicode<P1>(flags: u32, sourcestring: P1, sourcestringlength: i32, destinationstring: windows_core::PWSTR, destinationstringlength: super::PLONG) -> windows_core::NTSTATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ntdll.dll" "system" fn RtlIdnToUnicode(flags : u32, sourcestring : windows_core::PCWSTR, sourcestringlength : i32, destinationstring : windows_core::PWSTR, destinationstringlength : super::PLONG) -> windows_core::NTSTATUS);
    unsafe { RtlIdnToUnicode(flags, sourcestring.param().abi(), sourcestringlength, destinationstring, destinationstringlength as _) }
}
#[cfg(all(feature = "minwindef", feature = "ntnls"))]
#[inline]
pub unsafe fn RtlInitCodePageTable(tablebase: Option<&[u16; 2]>, codepagetable: super::PCPTABLEINFO) {
    windows_core::link!("ntdll.dll" "system" fn RtlInitCodePageTable(tablebase : super::PUSHORT, codepagetable : super::PCPTABLEINFO));
    unsafe { RtlInitCodePageTable(core::mem::transmute(tablebase.map_or(core::ptr::null(), |slice| slice.as_ptr())), codepagetable as _) }
}
#[cfg(all(feature = "lsalookup", feature = "ntdef", feature = "ntsecapi", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn RtlInitUTF8StringEx(destinationstring: super::PUTF8_STRING, sourcestring: Option<super::PCSZ>) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlInitUTF8StringEx(destinationstring : super::PUTF8_STRING, sourcestring : super::PCSZ) -> windows_core::NTSTATUS);
    unsafe { RtlInitUTF8StringEx(destinationstring as _, sourcestring.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
#[inline]
pub unsafe fn RtlInitUnicodeStringEx<P1>(destinationstring: super::PUNICODE_STRING, sourcestring: P1) -> windows_core::NTSTATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ntdll.dll" "system" fn RtlInitUnicodeStringEx(destinationstring : super::PUNICODE_STRING, sourcestring : windows_core::PCWSTR) -> windows_core::NTSTATUS);
    unsafe { RtlInitUnicodeStringEx(destinationstring as _, sourcestring.param().abi()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RtlInitializeSid(sid: super::PSID, identifierauthority: super::PSID_IDENTIFIER_AUTHORITY, subauthoritycount: u8) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlInitializeSid(sid : super::PSID, identifierauthority : super::PSID_IDENTIFIER_AUTHORITY, subauthoritycount : u8) -> windows_core::NTSTATUS);
    unsafe { RtlInitializeSid(sid as _, identifierauthority, subauthoritycount) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RtlIsCloudFilesPlaceholder(fileattributes: u32, reparsetag: u32) -> super::BOOLEAN {
    windows_core::link!("ntdll.dll" "system" fn RtlIsCloudFilesPlaceholder(fileattributes : u32, reparsetag : u32) -> super::BOOLEAN);
    unsafe { RtlIsCloudFilesPlaceholder(fileattributes, reparsetag) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RtlIsNonEmptyDirectoryReparsePointAllowed(reparsetag: u32) -> super::BOOLEAN {
    windows_core::link!("ntdll.dll" "system" fn RtlIsNonEmptyDirectoryReparsePointAllowed(reparsetag : u32) -> super::BOOLEAN);
    unsafe { RtlIsNonEmptyDirectoryReparsePointAllowed(reparsetag) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RtlIsNormalizedString<P1>(normform: u32, sourcestring: P1, sourcestringlength: i32, normalized: super::PBOOLEAN) -> windows_core::NTSTATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ntdll.dll" "system" fn RtlIsNormalizedString(normform : u32, sourcestring : windows_core::PCWSTR, sourcestringlength : i32, normalized : super::PBOOLEAN) -> windows_core::NTSTATUS);
    unsafe { RtlIsNormalizedString(normform, sourcestring.param().abi(), sourcestringlength, normalized as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RtlIsPartialPlaceholder(fileattributes: u32, reparsetag: u32) -> super::BOOLEAN {
    windows_core::link!("ntdll.dll" "system" fn RtlIsPartialPlaceholder(fileattributes : u32, reparsetag : u32) -> super::BOOLEAN);
    unsafe { RtlIsPartialPlaceholder(fileattributes, reparsetag) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RtlIsPartialPlaceholderFileHandle(filehandle: super::HANDLE, ispartialplaceholder: super::PBOOLEAN) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlIsPartialPlaceholderFileHandle(filehandle : super::HANDLE, ispartialplaceholder : super::PBOOLEAN) -> windows_core::NTSTATUS);
    unsafe { RtlIsPartialPlaceholderFileHandle(filehandle, ispartialplaceholder as _) }
}
#[cfg(all(feature = "wdm", feature = "winnt"))]
#[inline]
pub unsafe fn RtlIsPartialPlaceholderFileInfo(infobuffer: *const core::ffi::c_void, infoclass: super::FILE_INFORMATION_CLASS, ispartialplaceholder: super::PBOOLEAN) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlIsPartialPlaceholderFileInfo(infobuffer : *const core::ffi::c_void, infoclass : super::FILE_INFORMATION_CLASS, ispartialplaceholder : super::PBOOLEAN) -> windows_core::NTSTATUS);
    unsafe { RtlIsPartialPlaceholderFileInfo(infobuffer, infoclass, ispartialplaceholder as _) }
}
#[inline]
pub unsafe fn RtlLengthRequiredSid(subauthoritycount: u32) -> u32 {
    windows_core::link!("ntdll.dll" "system" fn RtlLengthRequiredSid(subauthoritycount : u32) -> u32);
    unsafe { RtlLengthRequiredSid(subauthoritycount) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RtlLengthSid(sid: super::PSID) -> u32 {
    windows_core::link!("ntdll.dll" "system" fn RtlLengthSid(sid : super::PSID) -> u32);
    unsafe { RtlLengthSid(sid) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn RtlMultiByteToUnicodeN(unicodestring: super::PWCH, maxbytesinunicodestring: u32, bytesinunicodestring: Option<super::PULONG>, multibytestring: &[u8]) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlMultiByteToUnicodeN(unicodestring : super::PWCH, maxbytesinunicodestring : u32, bytesinunicodestring : super::PULONG, multibytestring : *const i8, bytesinmultibytestring : u32) -> windows_core::NTSTATUS);
    unsafe { RtlMultiByteToUnicodeN(unicodestring as _, maxbytesinunicodestring, bytesinunicodestring.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(multibytestring.as_ptr()), multibytestring.len().try_into().unwrap()) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn RtlMultiByteToUnicodeSize(bytesinunicodestring: super::PULONG, multibytestring: &[u8]) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlMultiByteToUnicodeSize(bytesinunicodestring : super::PULONG, multibytestring : *const i8, bytesinmultibytestring : u32) -> windows_core::NTSTATUS);
    unsafe { RtlMultiByteToUnicodeSize(bytesinunicodestring as _, core::mem::transmute(multibytestring.as_ptr()), multibytestring.len().try_into().unwrap()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RtlNormalizeString<P1>(normform: u32, sourcestring: P1, sourcestringlength: i32, destinationstring: windows_core::PWSTR, destinationstringlength: super::PLONG) -> windows_core::NTSTATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ntdll.dll" "system" fn RtlNormalizeString(normform : u32, sourcestring : windows_core::PCWSTR, sourcestringlength : i32, destinationstring : windows_core::PWSTR, destinationstringlength : super::PLONG) -> windows_core::NTSTATUS);
    unsafe { RtlNormalizeString(normform, sourcestring.param().abi(), sourcestringlength, destinationstring, destinationstringlength as _) }
}
#[inline]
pub unsafe fn RtlNtStatusToDosErrorNoTeb(status: windows_core::NTSTATUS) -> u32 {
    windows_core::link!("ntdll.dll" "system" fn RtlNtStatusToDosErrorNoTeb(status : windows_core::NTSTATUS) -> u32);
    unsafe { RtlNtStatusToDosErrorNoTeb(status) }
}
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn RtlOemStringToUnicodeString(destinationstring: super::PUNICODE_STRING, sourcestring: super::PCOEM_STRING, allocatedestinationstring: super::BOOLEAN) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlOemStringToUnicodeString(destinationstring : super::PUNICODE_STRING, sourcestring : super::PCOEM_STRING, allocatedestinationstring : super::BOOLEAN) -> windows_core::NTSTATUS);
    unsafe { RtlOemStringToUnicodeString(destinationstring, sourcestring, allocatedestinationstring) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn RtlOemToUnicodeN(unicodestring: super::PWCH, maxbytesinunicodestring: u32, bytesinunicodestring: Option<super::PULONG>, oemstring: &[u8]) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlOemToUnicodeN(unicodestring : super::PWCH, maxbytesinunicodestring : u32, bytesinunicodestring : super::PULONG, oemstring : super::PCCH, bytesinoemstring : u32) -> windows_core::NTSTATUS);
    unsafe { RtlOemToUnicodeN(unicodestring as _, maxbytesinunicodestring, bytesinunicodestring.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(oemstring.as_ptr()), oemstring.len().try_into().unwrap()) }
}
#[cfg(all(feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn RtlPrefixString(string1: *const super::STRING, string2: *const super::STRING, caseinsensitive: super::BOOLEAN) -> super::BOOLEAN {
    windows_core::link!("ntdll.dll" "system" fn RtlPrefixString(string1 : *const super::STRING, string2 : *const super::STRING, caseinsensitive : super::BOOLEAN) -> super::BOOLEAN);
    unsafe { RtlPrefixString(string1, string2, caseinsensitive) }
}
#[cfg(all(feature = "basetsd", feature = "winnt"))]
#[inline]
pub unsafe fn RtlQueryPackageIdentity(tokenobject: *const core::ffi::c_void, packagefullname: windows_core::PWSTR, packagesize: super::PSIZE_T, appid: Option<windows_core::PWSTR>, appidsize: Option<super::PSIZE_T>, packaged: Option<super::PBOOLEAN>) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlQueryPackageIdentity(tokenobject : *const core::ffi::c_void, packagefullname : windows_core::PWSTR, packagesize : super::PSIZE_T, appid : windows_core::PWSTR, appidsize : super::PSIZE_T, packaged : super::PBOOLEAN) -> windows_core::NTSTATUS);
    unsafe { RtlQueryPackageIdentity(tokenobject, packagefullname, packagesize as _, appid.unwrap_or(core::mem::zeroed()) as _, appidsize.unwrap_or(core::mem::zeroed()) as _, packaged.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "basetsd", feature = "guiddef"))]
#[inline]
pub unsafe fn RtlQueryPackageIdentityEx(tokenobject: *const core::ffi::c_void, packagefullname: windows_core::PWSTR, packagesize: super::PSIZE_T, appid: Option<windows_core::PWSTR>, appidsize: Option<super::PSIZE_T>, dynamicid: Option<super::LPGUID>, flags: Option<super::PULONG64>) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlQueryPackageIdentityEx(tokenobject : *const core::ffi::c_void, packagefullname : windows_core::PWSTR, packagesize : super::PSIZE_T, appid : windows_core::PWSTR, appidsize : super::PSIZE_T, dynamicid : super::LPGUID, flags : super::PULONG64) -> windows_core::NTSTATUS);
    unsafe { RtlQueryPackageIdentityEx(tokenobject, packagefullname, packagesize as _, appid.unwrap_or(core::mem::zeroed()) as _, appidsize.unwrap_or(core::mem::zeroed()) as _, dynamicid.unwrap_or(core::mem::zeroed()) as _, flags.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RtlQueryProcessPlaceholderCompatibilityMode() -> i8 {
    windows_core::link!("ntdll.dll" "system" fn RtlQueryProcessPlaceholderCompatibilityMode() -> i8);
    unsafe { RtlQueryProcessPlaceholderCompatibilityMode() }
}
#[inline]
pub unsafe fn RtlQueryThreadPlaceholderCompatibilityMode() -> i8 {
    windows_core::link!("ntdll.dll" "system" fn RtlQueryThreadPlaceholderCompatibilityMode() -> i8);
    unsafe { RtlQueryThreadPlaceholderCompatibilityMode() }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn RtlRandom(seed: super::PULONG) -> u32 {
    windows_core::link!("ntdll.dll" "system" fn RtlRandom(seed : super::PULONG) -> u32);
    unsafe { RtlRandom(seed as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn RtlRandomEx(seed: super::PULONG) -> u32 {
    windows_core::link!("ntdll.dll" "system" fn RtlRandomEx(seed : super::PULONG) -> u32);
    unsafe { RtlRandomEx(seed as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RtlReplaceSidInSd(securitydescriptor: super::PSECURITY_DESCRIPTOR, oldsid: super::PSID, newsid: super::PSID, numchanges: *mut u32) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlReplaceSidInSd(securitydescriptor : super::PSECURITY_DESCRIPTOR, oldsid : super::PSID, newsid : super::PSID, numchanges : *mut u32) -> windows_core::NTSTATUS);
    unsafe { RtlReplaceSidInSd(securitydescriptor as _, oldsid, newsid, numchanges as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RtlSecondsSince1970ToTime(elapsedseconds: u32) -> super::LARGE_INTEGER {
    windows_core::link!("ntdll.dll" "system" fn RtlSecondsSince1970ToTime(elapsedseconds : u32, time : super::PLARGE_INTEGER));
    unsafe {
        let mut result__ = core::mem::zeroed();
        RtlSecondsSince1970ToTime(elapsedseconds, &mut result__);
        result__
    }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RtlSecondsSince1980ToTime(elapsedseconds: u32) -> super::LARGE_INTEGER {
    windows_core::link!("ntdll.dll" "system" fn RtlSecondsSince1980ToTime(elapsedseconds : u32, time : super::PLARGE_INTEGER));
    unsafe {
        let mut result__ = core::mem::zeroed();
        RtlSecondsSince1980ToTime(elapsedseconds, &mut result__);
        result__
    }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn RtlSelfRelativeToAbsoluteSD(selfrelativesecuritydescriptor: super::PSECURITY_DESCRIPTOR, absolutesecuritydescriptor: Option<super::PSECURITY_DESCRIPTOR>, absolutesecuritydescriptorsize: super::PULONG, dacl: Option<super::PACL>, daclsize: super::PULONG, sacl: Option<super::PACL>, saclsize: super::PULONG, owner: Option<super::PSID>, ownersize: super::PULONG, primarygroup: Option<super::PSID>, primarygroupsize: super::PULONG) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlSelfRelativeToAbsoluteSD(selfrelativesecuritydescriptor : super::PSECURITY_DESCRIPTOR, absolutesecuritydescriptor : super::PSECURITY_DESCRIPTOR, absolutesecuritydescriptorsize : super::PULONG, dacl : super::PACL, daclsize : super::PULONG, sacl : super::PACL, saclsize : super::PULONG, owner : super::PSID, ownersize : super::PULONG, primarygroup : super::PSID, primarygroupsize : super::PULONG) -> windows_core::NTSTATUS);
    unsafe { RtlSelfRelativeToAbsoluteSD(selfrelativesecuritydescriptor, absolutesecuritydescriptor.unwrap_or(core::mem::zeroed()) as _, absolutesecuritydescriptorsize as _, dacl.unwrap_or(core::mem::zeroed()) as _, daclsize as _, sacl.unwrap_or(core::mem::zeroed()) as _, saclsize as _, owner.unwrap_or(core::mem::zeroed()) as _, ownersize as _, primarygroup.unwrap_or(core::mem::zeroed()) as _, primarygroupsize as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RtlSetGroupSecurityDescriptor(securitydescriptor: super::PSECURITY_DESCRIPTOR, group: Option<super::PSID>, groupdefaulted: super::BOOLEAN) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlSetGroupSecurityDescriptor(securitydescriptor : super::PSECURITY_DESCRIPTOR, group : super::PSID, groupdefaulted : super::BOOLEAN) -> windows_core::NTSTATUS);
    unsafe { RtlSetGroupSecurityDescriptor(securitydescriptor as _, group.unwrap_or(core::mem::zeroed()) as _, groupdefaulted) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RtlSetOwnerSecurityDescriptor(securitydescriptor: super::PSECURITY_DESCRIPTOR, owner: Option<super::PSID>, ownerdefaulted: super::BOOLEAN) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlSetOwnerSecurityDescriptor(securitydescriptor : super::PSECURITY_DESCRIPTOR, owner : super::PSID, ownerdefaulted : super::BOOLEAN) -> windows_core::NTSTATUS);
    unsafe { RtlSetOwnerSecurityDescriptor(securitydescriptor as _, owner.unwrap_or(core::mem::zeroed()) as _, ownerdefaulted) }
}
#[inline]
pub unsafe fn RtlSetProcessPlaceholderCompatibilityMode(mode: i8) -> i8 {
    windows_core::link!("ntdll.dll" "system" fn RtlSetProcessPlaceholderCompatibilityMode(mode : i8) -> i8);
    unsafe { RtlSetProcessPlaceholderCompatibilityMode(mode) }
}
#[inline]
pub unsafe fn RtlSetThreadPlaceholderCompatibilityMode(mode: i8) -> i8 {
    windows_core::link!("ntdll.dll" "system" fn RtlSetThreadPlaceholderCompatibilityMode(mode : i8) -> i8);
    unsafe { RtlSetThreadPlaceholderCompatibilityMode(mode) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn RtlSubAuthorityCountSid(sid: super::PSID) -> super::PUCHAR {
    windows_core::link!("ntdll.dll" "system" fn RtlSubAuthorityCountSid(sid : super::PSID) -> super::PUCHAR);
    unsafe { RtlSubAuthorityCountSid(sid) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn RtlSubAuthoritySid(sid: super::PSID, subauthority: u32) -> super::PULONG {
    windows_core::link!("ntdll.dll" "system" fn RtlSubAuthoritySid(sid : super::PSID, subauthority : u32) -> super::PULONG);
    unsafe { RtlSubAuthoritySid(sid, subauthority) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn RtlTimeToSecondsSince1980(time: super::PLARGE_INTEGER, elapsedseconds: super::PULONG) -> super::BOOLEAN {
    windows_core::link!("ntdll.dll" "system" fn RtlTimeToSecondsSince1980(time : super::PLARGE_INTEGER, elapsedseconds : super::PULONG) -> super::BOOLEAN);
    unsafe { RtlTimeToSecondsSince1980(time, elapsedseconds as _) }
}
#[cfg(all(feature = "lsalookup", feature = "ntdef", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn RtlUTF8StringToUnicodeString(destinationstring: super::PUNICODE_STRING, sourcestring: super::PUTF8_STRING, allocatedestinationstring: super::BOOLEAN) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlUTF8StringToUnicodeString(destinationstring : super::PUNICODE_STRING, sourcestring : super::PUTF8_STRING, allocatedestinationstring : super::BOOLEAN) -> windows_core::NTSTATUS);
    unsafe { RtlUTF8StringToUnicodeString(destinationstring, sourcestring, allocatedestinationstring) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn RtlUTF8ToUnicodeN(unicodestringdestination: windows_core::PWSTR, unicodestringmaxbytecount: u32, unicodestringactualbytecount: super::PULONG, utf8stringsource: &[u8]) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlUTF8ToUnicodeN(unicodestringdestination : windows_core::PWSTR, unicodestringmaxbytecount : u32, unicodestringactualbytecount : super::PULONG, utf8stringsource : super::PCCH, utf8stringbytecount : u32) -> windows_core::NTSTATUS);
    unsafe { RtlUTF8ToUnicodeN(unicodestringdestination, unicodestringmaxbytecount, unicodestringactualbytecount as _, core::mem::transmute(utf8stringsource.as_ptr()), utf8stringsource.len().try_into().unwrap()) }
}
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn RtlUnicodeStringToCountedOemString(destinationstring: super::POEM_STRING, sourcestring: super::PCUNICODE_STRING, allocatedestinationstring: super::BOOLEAN) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlUnicodeStringToCountedOemString(destinationstring : super::POEM_STRING, sourcestring : super::PCUNICODE_STRING, allocatedestinationstring : super::BOOLEAN) -> windows_core::NTSTATUS);
    unsafe { RtlUnicodeStringToCountedOemString(destinationstring, sourcestring, allocatedestinationstring) }
}
#[cfg(all(feature = "lsalookup", feature = "ntdef", feature = "ntsecapi", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn RtlUnicodeStringToUTF8String(destinationstring: super::PUTF8_STRING, sourcestring: super::PCUNICODE_STRING, allocatedestinationstring: super::BOOLEAN) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlUnicodeStringToUTF8String(destinationstring : super::PUTF8_STRING, sourcestring : super::PCUNICODE_STRING, allocatedestinationstring : super::BOOLEAN) -> windows_core::NTSTATUS);
    unsafe { RtlUnicodeStringToUTF8String(destinationstring, sourcestring, allocatedestinationstring) }
}
#[cfg(all(feature = "minwindef", feature = "ntnls", feature = "winnt"))]
#[inline]
pub unsafe fn RtlUnicodeToCustomCPN(customcp: super::PCPTABLEINFO, customcpstring: super::PCH, maxbytesincustomcpstring: u32, bytesincustomcpstring: Option<super::PULONG>, unicodestring: super::PWCH, bytesinunicodestring: u32) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlUnicodeToCustomCPN(customcp : super::PCPTABLEINFO, customcpstring : super::PCH, maxbytesincustomcpstring : u32, bytesincustomcpstring : super::PULONG, unicodestring : super::PWCH, bytesinunicodestring : u32) -> windows_core::NTSTATUS);
    unsafe { RtlUnicodeToCustomCPN(customcp, customcpstring as _, maxbytesincustomcpstring, bytesincustomcpstring.unwrap_or(core::mem::zeroed()) as _, unicodestring, bytesinunicodestring) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn RtlUnicodeToMultiByteN(multibytestring: super::PCHAR, maxbytesinmultibytestring: u32, bytesinmultibytestring: Option<super::PULONG>, unicodestring: super::PCWCH, bytesinunicodestring: u32) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlUnicodeToMultiByteN(multibytestring : super::PCHAR, maxbytesinmultibytestring : u32, bytesinmultibytestring : super::PULONG, unicodestring : super::PCWCH, bytesinunicodestring : u32) -> windows_core::NTSTATUS);
    unsafe { RtlUnicodeToMultiByteN(multibytestring as _, maxbytesinmultibytestring, bytesinmultibytestring.unwrap_or(core::mem::zeroed()) as _, unicodestring, bytesinunicodestring) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn RtlUnicodeToOemN(oemstring: super::PCHAR, maxbytesinoemstring: u32, bytesinoemstring: Option<super::PULONG>, unicodestring: super::PCWCH, bytesinunicodestring: u32) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlUnicodeToOemN(oemstring : super::PCHAR, maxbytesinoemstring : u32, bytesinoemstring : super::PULONG, unicodestring : super::PCWCH, bytesinunicodestring : u32) -> windows_core::NTSTATUS);
    unsafe { RtlUnicodeToOemN(oemstring as _, maxbytesinoemstring, bytesinoemstring.unwrap_or(core::mem::zeroed()) as _, unicodestring, bytesinunicodestring) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn RtlUnicodeToUTF8N(utf8stringdestination: super::PCHAR, utf8stringmaxbytecount: u32, utf8stringactualbytecount: super::PULONG, unicodestringsource: super::PCWCH, unicodestringbytecount: u32) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlUnicodeToUTF8N(utf8stringdestination : super::PCHAR, utf8stringmaxbytecount : u32, utf8stringactualbytecount : super::PULONG, unicodestringsource : super::PCWCH, unicodestringbytecount : u32) -> windows_core::NTSTATUS);
    unsafe { RtlUnicodeToUTF8N(utf8stringdestination as _, utf8stringmaxbytecount, utf8stringactualbytecount as _, unicodestringsource, unicodestringbytecount) }
}
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn RtlUpcaseUnicodeStringToCountedOemString(destinationstring: super::POEM_STRING, sourcestring: super::PCUNICODE_STRING, allocatedestinationstring: super::BOOLEAN) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlUpcaseUnicodeStringToCountedOemString(destinationstring : super::POEM_STRING, sourcestring : super::PCUNICODE_STRING, allocatedestinationstring : super::BOOLEAN) -> windows_core::NTSTATUS);
    unsafe { RtlUpcaseUnicodeStringToCountedOemString(destinationstring, sourcestring, allocatedestinationstring) }
}
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn RtlUpcaseUnicodeStringToOemString(destinationstring: super::POEM_STRING, sourcestring: super::PCUNICODE_STRING, allocatedestinationstring: super::BOOLEAN) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlUpcaseUnicodeStringToOemString(destinationstring : super::POEM_STRING, sourcestring : super::PCUNICODE_STRING, allocatedestinationstring : super::BOOLEAN) -> windows_core::NTSTATUS);
    unsafe { RtlUpcaseUnicodeStringToOemString(destinationstring, sourcestring, allocatedestinationstring) }
}
#[cfg(all(feature = "minwindef", feature = "ntnls", feature = "winnt"))]
#[inline]
pub unsafe fn RtlUpcaseUnicodeToCustomCPN(customcp: super::PCPTABLEINFO, customcpstring: super::PCH, maxbytesincustomcpstring: u32, bytesincustomcpstring: Option<super::PULONG>, unicodestring: super::PWCH, bytesinunicodestring: u32) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlUpcaseUnicodeToCustomCPN(customcp : super::PCPTABLEINFO, customcpstring : super::PCH, maxbytesincustomcpstring : u32, bytesincustomcpstring : super::PULONG, unicodestring : super::PWCH, bytesinunicodestring : u32) -> windows_core::NTSTATUS);
    unsafe { RtlUpcaseUnicodeToCustomCPN(customcp, customcpstring as _, maxbytesincustomcpstring, bytesincustomcpstring.unwrap_or(core::mem::zeroed()) as _, unicodestring, bytesinunicodestring) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn RtlUpcaseUnicodeToMultiByteN(multibytestring: super::PCHAR, maxbytesinmultibytestring: u32, bytesinmultibytestring: Option<super::PULONG>, unicodestring: super::PCWCH, bytesinunicodestring: u32) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlUpcaseUnicodeToMultiByteN(multibytestring : super::PCHAR, maxbytesinmultibytestring : u32, bytesinmultibytestring : super::PULONG, unicodestring : super::PCWCH, bytesinunicodestring : u32) -> windows_core::NTSTATUS);
    unsafe { RtlUpcaseUnicodeToMultiByteN(multibytestring as _, maxbytesinmultibytestring, bytesinmultibytestring.unwrap_or(core::mem::zeroed()) as _, unicodestring, bytesinunicodestring) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn RtlUpcaseUnicodeToOemN(oemstring: super::PCHAR, maxbytesinoemstring: u32, bytesinoemstring: Option<super::PULONG>, unicodestring: super::PCWCH, bytesinunicodestring: u32) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlUpcaseUnicodeToOemN(oemstring : super::PCHAR, maxbytesinoemstring : u32, bytesinoemstring : super::PULONG, unicodestring : super::PCWCH, bytesinunicodestring : u32) -> windows_core::NTSTATUS);
    unsafe { RtlUpcaseUnicodeToOemN(oemstring as _, maxbytesinoemstring, bytesinoemstring.unwrap_or(core::mem::zeroed()) as _, unicodestring, bytesinunicodestring) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RtlValidSid(sid: super::PSID) -> super::BOOLEAN {
    windows_core::link!("ntdll.dll" "system" fn RtlValidSid(sid : super::PSID) -> super::BOOLEAN);
    unsafe { RtlValidSid(sid) }
}
#[cfg(feature = "winternl")]
#[inline]
pub unsafe fn RtlValidateUnicodeString(flags: Option<u32>, string: super::PCUNICODE_STRING) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlValidateUnicodeString(flags : u32, string : super::PCUNICODE_STRING) -> windows_core::NTSTATUS);
    unsafe { RtlValidateUnicodeString(flags.unwrap_or(core::mem::zeroed()) as _, string) }
}
#[cfg(all(feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn RtlxOemStringToUnicodeSize(oemstring: super::PCOEM_STRING) -> u32 {
    windows_core::link!("ntdll.dll" "system" fn RtlxOemStringToUnicodeSize(oemstring : super::PCOEM_STRING) -> u32);
    unsafe { RtlxOemStringToUnicodeSize(oemstring) }
}
#[cfg(feature = "winternl")]
#[inline]
pub unsafe fn RtlxUnicodeStringToOemSize(unicodestring: super::PCUNICODE_STRING) -> u32 {
    windows_core::link!("ntdll.dll" "system" fn RtlxUnicodeStringToOemSize(unicodestring : super::PCUNICODE_STRING) -> u32);
    unsafe { RtlxUnicodeStringToOemSize(unicodestring) }
}
#[cfg(all(feature = "basetsd", feature = "winnt"))]
#[inline]
pub unsafe fn ZwAllocateVirtualMemory(processhandle: super::HANDLE, baseaddress: *mut *mut core::ffi::c_void, zerobits: usize, regionsize: super::PSIZE_T, allocationtype: u32, protect: u32) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwAllocateVirtualMemory(processhandle : super::HANDLE, baseaddress : *mut *mut core::ffi::c_void, zerobits : usize, regionsize : super::PSIZE_T, allocationtype : u32, protect : u32) -> windows_core::NTSTATUS);
    unsafe { ZwAllocateVirtualMemory(processhandle, baseaddress as _, zerobits, regionsize as _, allocationtype, protect) }
}
#[cfg(all(feature = "basetsd", feature = "winnt"))]
#[inline]
pub unsafe fn ZwAllocateVirtualMemoryEx(processhandle: super::HANDLE, baseaddress: *mut *mut core::ffi::c_void, regionsize: super::PSIZE_T, allocationtype: u32, pageprotection: u32, extendedparameters: Option<&mut [super::MEM_EXTENDED_PARAMETER]>) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwAllocateVirtualMemoryEx(processhandle : super::HANDLE, baseaddress : *mut *mut core::ffi::c_void, regionsize : super::PSIZE_T, allocationtype : u32, pageprotection : u32, extendedparameters : super::PMEM_EXTENDED_PARAMETER, extendedparametercount : u32) -> windows_core::NTSTATUS);
    unsafe { ZwAllocateVirtualMemoryEx(processhandle, baseaddress as _, regionsize as _, allocationtype, pageprotection, core::mem::transmute(extendedparameters.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut())), extendedparameters.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[cfg(all(feature = "d3dkmthk", feature = "lsalookup", feature = "ntdef", feature = "ntsecapi", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn ZwCreateEvent(eventhandle: super::PHANDLE, desiredaccess: super::ACCESS_MASK, objectattributes: Option<super::POBJECT_ATTRIBUTES>, eventtype: super::EVENT_TYPE, initialstate: super::BOOLEAN) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwCreateEvent(eventhandle : super::PHANDLE, desiredaccess : super::ACCESS_MASK, objectattributes : super::POBJECT_ATTRIBUTES, eventtype : super::EVENT_TYPE, initialstate : super::BOOLEAN) -> windows_core::NTSTATUS);
    unsafe { ZwCreateEvent(eventhandle as _, desiredaccess, objectattributes.unwrap_or(core::mem::zeroed()) as _, eventtype, initialstate) }
}
#[cfg(all(feature = "d3dkmthk", feature = "lsalookup", feature = "ntsecapi", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn ZwDeleteFile(objectattributes: super::POBJECT_ATTRIBUTES) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwDeleteFile(objectattributes : super::POBJECT_ATTRIBUTES) -> windows_core::NTSTATUS);
    unsafe { ZwDeleteFile(objectattributes) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn ZwDuplicateObject(sourceprocesshandle: super::HANDLE, sourcehandle: super::HANDLE, targetprocesshandle: Option<super::HANDLE>, targethandle: Option<super::PHANDLE>, desiredaccess: super::ACCESS_MASK, handleattributes: u32, options: u32) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwDuplicateObject(sourceprocesshandle : super::HANDLE, sourcehandle : super::HANDLE, targetprocesshandle : super::HANDLE, targethandle : super::PHANDLE, desiredaccess : super::ACCESS_MASK, handleattributes : u32, options : u32) -> windows_core::NTSTATUS);
    unsafe { ZwDuplicateObject(sourceprocesshandle, sourcehandle, targetprocesshandle.unwrap_or(core::mem::zeroed()) as _, targethandle.unwrap_or(core::mem::zeroed()) as _, desiredaccess, handleattributes, options) }
}
#[cfg(all(feature = "d3dkmthk", feature = "lsalookup", feature = "ntsecapi", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn ZwDuplicateToken(existingtokenhandle: super::HANDLE, desiredaccess: super::ACCESS_MASK, objectattributes: Option<super::POBJECT_ATTRIBUTES>, effectiveonly: super::BOOLEAN, tokentype: super::TOKEN_TYPE, newtokenhandle: super::PHANDLE) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwDuplicateToken(existingtokenhandle : super::HANDLE, desiredaccess : super::ACCESS_MASK, objectattributes : super::POBJECT_ATTRIBUTES, effectiveonly : super::BOOLEAN, tokentype : super::TOKEN_TYPE, newtokenhandle : super::PHANDLE) -> windows_core::NTSTATUS);
    unsafe { ZwDuplicateToken(existingtokenhandle, desiredaccess, objectattributes.unwrap_or(core::mem::zeroed()) as _, effectiveonly, tokentype, newtokenhandle as _) }
}
#[cfg(all(feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn ZwFlushBuffersFile(filehandle: super::HANDLE, iostatusblock: super::PIO_STATUS_BLOCK) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "C" fn ZwFlushBuffersFile(filehandle : super::HANDLE, iostatusblock : super::PIO_STATUS_BLOCK) -> windows_core::NTSTATUS);
    unsafe { ZwFlushBuffersFile(filehandle, iostatusblock as _) }
}
#[cfg(all(feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn ZwFlushBuffersFileEx(filehandle: super::HANDLE, flags: u32, parameters: *const core::ffi::c_void, parameterssize: u32, iostatusblock: super::PIO_STATUS_BLOCK) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "C" fn ZwFlushBuffersFileEx(filehandle : super::HANDLE, flags : u32, parameters : *const core::ffi::c_void, parameterssize : u32, iostatusblock : super::PIO_STATUS_BLOCK) -> windows_core::NTSTATUS);
    unsafe { ZwFlushBuffersFileEx(filehandle, flags, parameters, parameterssize, iostatusblock as _) }
}
#[cfg(all(feature = "basetsd", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn ZwFlushVirtualMemory(processhandle: super::HANDLE, baseaddress: *mut *mut core::ffi::c_void, regionsize: super::PSIZE_T, iostatus: super::PIO_STATUS_BLOCK) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwFlushVirtualMemory(processhandle : super::HANDLE, baseaddress : *mut *mut core::ffi::c_void, regionsize : super::PSIZE_T, iostatus : super::PIO_STATUS_BLOCK) -> windows_core::NTSTATUS);
    unsafe { ZwFlushVirtualMemory(processhandle, baseaddress as _, regionsize as _, iostatus as _) }
}
#[cfg(all(feature = "basetsd", feature = "winnt"))]
#[inline]
pub unsafe fn ZwFreeVirtualMemory(processhandle: super::HANDLE, baseaddress: *mut *mut core::ffi::c_void, regionsize: super::PSIZE_T, freetype: u32) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwFreeVirtualMemory(processhandle : super::HANDLE, baseaddress : *mut *mut core::ffi::c_void, regionsize : super::PSIZE_T, freetype : u32) -> windows_core::NTSTATUS);
    unsafe { ZwFreeVirtualMemory(processhandle, baseaddress as _, regionsize as _, freetype) }
}
#[cfg(all(feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn ZwFsControlFile(filehandle: super::HANDLE, event: Option<super::HANDLE>, apcroutine: super::PIO_APC_ROUTINE, apccontext: Option<*const core::ffi::c_void>, iostatusblock: super::PIO_STATUS_BLOCK, fscontrolcode: u32, inputbuffer: Option<*const core::ffi::c_void>, inputbufferlength: u32, outputbuffer: Option<*mut core::ffi::c_void>, outputbufferlength: u32) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwFsControlFile(filehandle : super::HANDLE, event : super::HANDLE, apcroutine : super::PIO_APC_ROUTINE, apccontext : *const core::ffi::c_void, iostatusblock : super::PIO_STATUS_BLOCK, fscontrolcode : u32, inputbuffer : *const core::ffi::c_void, inputbufferlength : u32, outputbuffer : *mut core::ffi::c_void, outputbufferlength : u32) -> windows_core::NTSTATUS);
    unsafe { ZwFsControlFile(filehandle, event.unwrap_or(core::mem::zeroed()) as _, apcroutine, apccontext.unwrap_or(core::mem::zeroed()) as _, iostatusblock as _, fscontrolcode, inputbuffer.unwrap_or(core::mem::zeroed()) as _, inputbufferlength, outputbuffer.unwrap_or(core::mem::zeroed()) as _, outputbufferlength) }
}
#[cfg(all(feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn ZwLockFile(filehandle: super::HANDLE, event: Option<super::HANDLE>, apcroutine: super::PIO_APC_ROUTINE, apccontext: Option<*const core::ffi::c_void>, iostatusblock: super::PIO_STATUS_BLOCK, byteoffset: super::PLARGE_INTEGER, length: super::PLARGE_INTEGER, key: u32, failimmediately: super::BOOLEAN, exclusivelock: super::BOOLEAN) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwLockFile(filehandle : super::HANDLE, event : super::HANDLE, apcroutine : super::PIO_APC_ROUTINE, apccontext : *const core::ffi::c_void, iostatusblock : super::PIO_STATUS_BLOCK, byteoffset : super::PLARGE_INTEGER, length : super::PLARGE_INTEGER, key : u32, failimmediately : super::BOOLEAN, exclusivelock : super::BOOLEAN) -> windows_core::NTSTATUS);
    unsafe { ZwLockFile(filehandle, event.unwrap_or(core::mem::zeroed()) as _, apcroutine, apccontext.unwrap_or(core::mem::zeroed()) as _, iostatusblock as _, byteoffset, length, key, failimmediately, exclusivelock) }
}
#[cfg(all(feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn ZwNotifyChangeKey(keyhandle: super::HANDLE, event: Option<super::HANDLE>, apcroutine: super::PIO_APC_ROUTINE, apccontext: Option<*const core::ffi::c_void>, iostatusblock: super::PIO_STATUS_BLOCK, completionfilter: u32, watchtree: super::BOOLEAN, buffer: Option<*mut core::ffi::c_void>, buffersize: u32, asynchronous: super::BOOLEAN) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwNotifyChangeKey(keyhandle : super::HANDLE, event : super::HANDLE, apcroutine : super::PIO_APC_ROUTINE, apccontext : *const core::ffi::c_void, iostatusblock : super::PIO_STATUS_BLOCK, completionfilter : u32, watchtree : super::BOOLEAN, buffer : *mut core::ffi::c_void, buffersize : u32, asynchronous : super::BOOLEAN) -> windows_core::NTSTATUS);
    unsafe { ZwNotifyChangeKey(keyhandle, event.unwrap_or(core::mem::zeroed()) as _, apcroutine, apccontext.unwrap_or(core::mem::zeroed()) as _, iostatusblock as _, completionfilter, watchtree, buffer.unwrap_or(core::mem::zeroed()) as _, buffersize, asynchronous) }
}
#[cfg(all(feature = "d3dkmthk", feature = "lsalookup", feature = "ntsecapi", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn ZwOpenDirectoryObject(directoryhandle: super::PHANDLE, desiredaccess: super::ACCESS_MASK, objectattributes: super::POBJECT_ATTRIBUTES) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwOpenDirectoryObject(directoryhandle : super::PHANDLE, desiredaccess : super::ACCESS_MASK, objectattributes : super::POBJECT_ATTRIBUTES) -> windows_core::NTSTATUS);
    unsafe { ZwOpenDirectoryObject(directoryhandle as _, desiredaccess, objectattributes) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn ZwOpenProcessTokenEx(processhandle: super::HANDLE, desiredaccess: super::ACCESS_MASK, handleattributes: u32, tokenhandle: super::PHANDLE) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwOpenProcessTokenEx(processhandle : super::HANDLE, desiredaccess : super::ACCESS_MASK, handleattributes : u32, tokenhandle : super::PHANDLE) -> windows_core::NTSTATUS);
    unsafe { ZwOpenProcessTokenEx(processhandle, desiredaccess, handleattributes, tokenhandle as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn ZwOpenThreadTokenEx(threadhandle: super::HANDLE, desiredaccess: super::ACCESS_MASK, openasself: super::BOOLEAN, handleattributes: u32, tokenhandle: super::PHANDLE) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwOpenThreadTokenEx(threadhandle : super::HANDLE, desiredaccess : super::ACCESS_MASK, openasself : super::BOOLEAN, handleattributes : u32, tokenhandle : super::PHANDLE) -> windows_core::NTSTATUS);
    unsafe { ZwOpenThreadTokenEx(threadhandle, desiredaccess, openasself, handleattributes, tokenhandle as _) }
}
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "wdm", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn ZwQueryDirectoryFile(filehandle: super::HANDLE, event: Option<super::HANDLE>, apcroutine: super::PIO_APC_ROUTINE, apccontext: Option<*const core::ffi::c_void>, iostatusblock: super::PIO_STATUS_BLOCK, fileinformation: *mut core::ffi::c_void, length: u32, fileinformationclass: super::FILE_INFORMATION_CLASS, returnsingleentry: super::BOOLEAN, filename: Option<super::PUNICODE_STRING>, restartscan: super::BOOLEAN) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwQueryDirectoryFile(filehandle : super::HANDLE, event : super::HANDLE, apcroutine : super::PIO_APC_ROUTINE, apccontext : *const core::ffi::c_void, iostatusblock : super::PIO_STATUS_BLOCK, fileinformation : *mut core::ffi::c_void, length : u32, fileinformationclass : super::FILE_INFORMATION_CLASS, returnsingleentry : super::BOOLEAN, filename : super::PUNICODE_STRING, restartscan : super::BOOLEAN) -> windows_core::NTSTATUS);
    unsafe { ZwQueryDirectoryFile(filehandle, event.unwrap_or(core::mem::zeroed()) as _, apcroutine, apccontext.unwrap_or(core::mem::zeroed()) as _, iostatusblock as _, fileinformation as _, length, fileinformationclass, returnsingleentry, filename.unwrap_or(core::mem::zeroed()) as _, restartscan) }
}
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "wdm", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn ZwQueryDirectoryFileEx(filehandle: super::HANDLE, event: Option<super::HANDLE>, apcroutine: super::PIO_APC_ROUTINE, apccontext: Option<*const core::ffi::c_void>, iostatusblock: super::PIO_STATUS_BLOCK, fileinformation: *mut core::ffi::c_void, length: u32, fileinformationclass: super::FILE_INFORMATION_CLASS, queryflags: u32, filename: Option<super::PUNICODE_STRING>) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwQueryDirectoryFileEx(filehandle : super::HANDLE, event : super::HANDLE, apcroutine : super::PIO_APC_ROUTINE, apccontext : *const core::ffi::c_void, iostatusblock : super::PIO_STATUS_BLOCK, fileinformation : *mut core::ffi::c_void, length : u32, fileinformationclass : super::FILE_INFORMATION_CLASS, queryflags : u32, filename : super::PUNICODE_STRING) -> windows_core::NTSTATUS);
    unsafe { ZwQueryDirectoryFileEx(filehandle, event.unwrap_or(core::mem::zeroed()) as _, apcroutine, apccontext.unwrap_or(core::mem::zeroed()) as _, iostatusblock as _, fileinformation as _, length, fileinformationclass, queryflags, filename.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn ZwQueryEaFile(filehandle: super::HANDLE, iostatusblock: super::PIO_STATUS_BLOCK, buffer: *mut core::ffi::c_void, length: u32, returnsingleentry: super::BOOLEAN, ealist: Option<*const core::ffi::c_void>, ealistlength: u32, eaindex: Option<super::PULONG>, restartscan: super::BOOLEAN) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "C" fn ZwQueryEaFile(filehandle : super::HANDLE, iostatusblock : super::PIO_STATUS_BLOCK, buffer : *mut core::ffi::c_void, length : u32, returnsingleentry : super::BOOLEAN, ealist : *const core::ffi::c_void, ealistlength : u32, eaindex : super::PULONG, restartscan : super::BOOLEAN) -> windows_core::NTSTATUS);
    unsafe { ZwQueryEaFile(filehandle, iostatusblock as _, buffer as _, length, returnsingleentry, ealist.unwrap_or(core::mem::zeroed()) as _, ealistlength, eaindex.unwrap_or(core::mem::zeroed()) as _, restartscan) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn ZwQueryInformationToken(tokenhandle: super::HANDLE, tokeninformationclass: super::TOKEN_INFORMATION_CLASS, tokeninformation: Option<*mut core::ffi::c_void>, tokeninformationlength: u32, returnlength: super::PULONG) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwQueryInformationToken(tokenhandle : super::HANDLE, tokeninformationclass : super::TOKEN_INFORMATION_CLASS, tokeninformation : *mut core::ffi::c_void, tokeninformationlength : u32, returnlength : super::PULONG) -> windows_core::NTSTATUS);
    unsafe { ZwQueryInformationToken(tokenhandle, tokeninformationclass, tokeninformation.unwrap_or(core::mem::zeroed()) as _, tokeninformationlength, returnlength as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn ZwQueryObject(handle: Option<super::HANDLE>, objectinformationclass: super::OBJECT_INFORMATION_CLASS, objectinformation: Option<*mut core::ffi::c_void>, objectinformationlength: u32, returnlength: Option<super::PULONG>) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwQueryObject(handle : super::HANDLE, objectinformationclass : super::OBJECT_INFORMATION_CLASS, objectinformation : *mut core::ffi::c_void, objectinformationlength : u32, returnlength : super::PULONG) -> windows_core::NTSTATUS);
    unsafe { ZwQueryObject(handle.unwrap_or(core::mem::zeroed()) as _, objectinformationclass, objectinformation.unwrap_or(core::mem::zeroed()) as _, objectinformationlength, returnlength.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn ZwQueryQuotaInformationFile(filehandle: super::HANDLE, iostatusblock: super::PIO_STATUS_BLOCK, buffer: *mut core::ffi::c_void, length: u32, returnsingleentry: super::BOOLEAN, sidlist: Option<*const core::ffi::c_void>, sidlistlength: u32, startsid: Option<super::PSID>, restartscan: super::BOOLEAN) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwQueryQuotaInformationFile(filehandle : super::HANDLE, iostatusblock : super::PIO_STATUS_BLOCK, buffer : *mut core::ffi::c_void, length : u32, returnsingleentry : super::BOOLEAN, sidlist : *const core::ffi::c_void, sidlistlength : u32, startsid : super::PSID, restartscan : super::BOOLEAN) -> windows_core::NTSTATUS);
    unsafe { ZwQueryQuotaInformationFile(filehandle, iostatusblock as _, buffer as _, length, returnsingleentry, sidlist.unwrap_or(core::mem::zeroed()) as _, sidlistlength, startsid.unwrap_or(core::mem::zeroed()) as _, restartscan) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn ZwQuerySecurityObject(handle: super::HANDLE, securityinformation: super::SECURITY_INFORMATION, securitydescriptor: super::PSECURITY_DESCRIPTOR, length: u32, lengthneeded: super::PULONG) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwQuerySecurityObject(handle : super::HANDLE, securityinformation : super::SECURITY_INFORMATION, securitydescriptor : super::PSECURITY_DESCRIPTOR, length : u32, lengthneeded : super::PULONG) -> windows_core::NTSTATUS);
    unsafe { ZwQuerySecurityObject(handle, securityinformation, securitydescriptor as _, length, lengthneeded as _) }
}
#[cfg(all(feature = "basetsd", feature = "winnt"))]
#[inline]
pub unsafe fn ZwQueryVirtualMemory(processhandle: super::HANDLE, baseaddress: Option<*const core::ffi::c_void>, memoryinformationclass: MEMORY_INFORMATION_CLASS, memoryinformation: *mut core::ffi::c_void, memoryinformationlength: usize, returnlength: Option<super::PSIZE_T>) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwQueryVirtualMemory(processhandle : super::HANDLE, baseaddress : *const core::ffi::c_void, memoryinformationclass : MEMORY_INFORMATION_CLASS, memoryinformation : *mut core::ffi::c_void, memoryinformationlength : usize, returnlength : super::PSIZE_T) -> windows_core::NTSTATUS);
    unsafe { ZwQueryVirtualMemory(processhandle, baseaddress.unwrap_or(core::mem::zeroed()) as _, memoryinformationclass, memoryinformation as _, memoryinformationlength, returnlength.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn ZwSetEaFile(filehandle: super::HANDLE, iostatusblock: super::PIO_STATUS_BLOCK, buffer: *const core::ffi::c_void, length: u32) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "C" fn ZwSetEaFile(filehandle : super::HANDLE, iostatusblock : super::PIO_STATUS_BLOCK, buffer : *const core::ffi::c_void, length : u32) -> windows_core::NTSTATUS);
    unsafe { ZwSetEaFile(filehandle, iostatusblock as _, buffer, length) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn ZwSetEvent(eventhandle: super::HANDLE, previousstate: Option<super::PLONG>) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwSetEvent(eventhandle : super::HANDLE, previousstate : super::PLONG) -> windows_core::NTSTATUS);
    unsafe { ZwSetEvent(eventhandle, previousstate.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn ZwSetInformationToken(tokenhandle: super::HANDLE, tokeninformationclass: super::TOKEN_INFORMATION_CLASS, tokeninformation: *const core::ffi::c_void, tokeninformationlength: u32) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwSetInformationToken(tokenhandle : super::HANDLE, tokeninformationclass : super::TOKEN_INFORMATION_CLASS, tokeninformation : *const core::ffi::c_void, tokeninformationlength : u32) -> windows_core::NTSTATUS);
    unsafe { ZwSetInformationToken(tokenhandle, tokeninformationclass, tokeninformation, tokeninformationlength) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn ZwSetInformationVirtualMemory(processhandle: super::HANDLE, vminformationclass: VIRTUAL_MEMORY_INFORMATION_CLASS, virtualaddresses: &[MEMORY_RANGE_ENTRY], vminformation: *const core::ffi::c_void, vminformationlength: u32) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwSetInformationVirtualMemory(processhandle : super::HANDLE, vminformationclass : VIRTUAL_MEMORY_INFORMATION_CLASS, numberofentries : usize, virtualaddresses : PMEMORY_RANGE_ENTRY, vminformation : *const core::ffi::c_void, vminformationlength : u32) -> windows_core::NTSTATUS);
    unsafe { ZwSetInformationVirtualMemory(processhandle, vminformationclass, virtualaddresses.len().try_into().unwrap(), core::mem::transmute(virtualaddresses.as_ptr()), vminformation, vminformationlength) }
}
#[cfg(all(feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn ZwSetQuotaInformationFile(filehandle: super::HANDLE, iostatusblock: super::PIO_STATUS_BLOCK, buffer: *const core::ffi::c_void, length: u32) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwSetQuotaInformationFile(filehandle : super::HANDLE, iostatusblock : super::PIO_STATUS_BLOCK, buffer : *const core::ffi::c_void, length : u32) -> windows_core::NTSTATUS);
    unsafe { ZwSetQuotaInformationFile(filehandle, iostatusblock as _, buffer, length) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn ZwSetSecurityObject(handle: super::HANDLE, securityinformation: super::SECURITY_INFORMATION, securitydescriptor: super::PSECURITY_DESCRIPTOR) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwSetSecurityObject(handle : super::HANDLE, securityinformation : super::SECURITY_INFORMATION, securitydescriptor : super::PSECURITY_DESCRIPTOR) -> windows_core::NTSTATUS);
    unsafe { ZwSetSecurityObject(handle, securityinformation, securitydescriptor) }
}
#[cfg(all(feature = "wdm", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn ZwSetVolumeInformationFile(filehandle: super::HANDLE, iostatusblock: super::PIO_STATUS_BLOCK, fsinformation: *const core::ffi::c_void, length: u32, fsinformationclass: super::FS_INFORMATION_CLASS) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwSetVolumeInformationFile(filehandle : super::HANDLE, iostatusblock : super::PIO_STATUS_BLOCK, fsinformation : *const core::ffi::c_void, length : u32, fsinformationclass : super::FS_INFORMATION_CLASS) -> windows_core::NTSTATUS);
    unsafe { ZwSetVolumeInformationFile(filehandle, iostatusblock as _, fsinformation, length, fsinformationclass) }
}
#[cfg(all(feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn ZwUnlockFile(filehandle: super::HANDLE, iostatusblock: super::PIO_STATUS_BLOCK, byteoffset: super::PLARGE_INTEGER, length: super::PLARGE_INTEGER, key: u32) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwUnlockFile(filehandle : super::HANDLE, iostatusblock : super::PIO_STATUS_BLOCK, byteoffset : super::PLARGE_INTEGER, length : super::PLARGE_INTEGER, key : u32) -> windows_core::NTSTATUS);
    unsafe { ZwUnlockFile(filehandle, iostatusblock as _, byteoffset, length, key) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn ZwWaitForSingleObject(handle: super::HANDLE, alertable: super::BOOLEAN, timeout: Option<super::PLARGE_INTEGER>) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwWaitForSingleObject(handle : super::HANDLE, alertable : super::BOOLEAN, timeout : super::PLARGE_INTEGER) -> windows_core::NTSTATUS);
    unsafe { ZwWaitForSingleObject(handle, alertable, timeout.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "basetsd", feature = "winnt"))]
pub type ALLOCATE_VIRTUAL_MEMORY_EX_CALLBACK = Option<unsafe extern "C" fn(callbackcontext: super::HANDLE, processhandle: super::HANDLE, baseaddress: *mut *mut core::ffi::c_void, regionsize: super::PSIZE_T, allocationtype: u32, pageprotection: u32, extendedparameters: super::PMEM_EXTENDED_PARAMETER, extendedparametercount: u32) -> windows_core::NTSTATUS>;
pub const ANSI_DOS_DOT: i8 = 34;
pub const ANSI_DOS_QM: i8 = 62;
pub const ANSI_DOS_STAR: i8 = 60;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ATOMIC_CREATE_ECP_CONTEXT {
    pub Size: u16,
    pub InFlags: u16,
    pub OutFlags: u16,
    pub ReparseBufferLength: u16,
    pub ReparseBuffer: PREPARSE_DATA_BUFFER,
    pub FileSize: i64,
    pub ValidDataLength: i64,
    pub FileTimestamps: PFILE_TIMESTAMPS,
    pub FileAttributes: u32,
    pub UsnSourceInfo: u32,
    pub Usn: super::USN,
    pub SuppressFileAttributeInheritanceMask: u32,
    pub InOpFlags: u32,
    pub OutOpFlags: u32,
    pub InGenFlags: u32,
    pub OutGenFlags: u32,
    pub CaseSensitiveFlagsMask: u32,
    pub InCaseSensitiveFlags: u32,
    pub OutCaseSensitiveFlags: u32,
}
pub const ATOMIC_CREATE_ECP_IN_FLAG_BEST_EFFORT: i32 = 256;
pub const ATOMIC_CREATE_ECP_IN_FLAG_EOF_SPECIFIED: i32 = 4;
pub const ATOMIC_CREATE_ECP_IN_FLAG_FILE_ATTRIBUTES_SPECIFIED: i32 = 32;
pub const ATOMIC_CREATE_ECP_IN_FLAG_GEN_FLAGS_SPECIFIED: i32 = 32768;
pub const ATOMIC_CREATE_ECP_IN_FLAG_MARK_USN_SOURCE_INFO: i32 = 2048;
pub const ATOMIC_CREATE_ECP_IN_FLAG_OPERATION_MASK: i32 = 255;
pub const ATOMIC_CREATE_ECP_IN_FLAG_OP_FLAGS_SPECIFIED: i32 = 128;
pub const ATOMIC_CREATE_ECP_IN_FLAG_REPARSE_POINT_SPECIFIED: i32 = 2;
pub const ATOMIC_CREATE_ECP_IN_FLAG_SPARSE_SPECIFIED: i32 = 1;
pub const ATOMIC_CREATE_ECP_IN_FLAG_SUPPRESS_DIR_CHANGE_NOTIFY: i32 = 1024;
pub const ATOMIC_CREATE_ECP_IN_FLAG_SUPPRESS_FILE_ATTRIBUTE_INHERITANCE: i32 = 64;
pub const ATOMIC_CREATE_ECP_IN_FLAG_SUPPRESS_PARENT_TIMESTAMPS_UPDATE: i32 = 512;
pub const ATOMIC_CREATE_ECP_IN_FLAG_TIMESTAMPS_SPECIFIED: i32 = 16;
pub const ATOMIC_CREATE_ECP_IN_FLAG_VDL_SPECIFIED: i32 = 8;
pub const ATOMIC_CREATE_ECP_IN_FLAG_WRITE_USN_CLOSE_RECORD: i32 = 4096;
pub const ATOMIC_CREATE_ECP_IN_OP_FLAG_CASE_SENSITIVE_FLAGS_SPECIFIED: i32 = 1;
pub const ATOMIC_CREATE_ECP_OUT_FLAG_EOF_SET: i32 = 4;
pub const ATOMIC_CREATE_ECP_OUT_FLAG_FILE_ATTRIBUTES_RETURNED: i32 = 512;
pub const ATOMIC_CREATE_ECP_OUT_FLAG_FILE_ATTRIBUTES_SET: i32 = 32;
pub const ATOMIC_CREATE_ECP_OUT_FLAG_FILE_ATTRIBUTE_INHERITANCE_SUPPRESSED: i32 = 64;
pub const ATOMIC_CREATE_ECP_OUT_FLAG_OPERATION_MASK: i32 = 255;
pub const ATOMIC_CREATE_ECP_OUT_FLAG_OP_FLAGS_HONORED: i32 = 128;
pub const ATOMIC_CREATE_ECP_OUT_FLAG_REPARSE_POINT_SET: i32 = 2;
pub const ATOMIC_CREATE_ECP_OUT_FLAG_SPARSE_SET: i32 = 1;
pub const ATOMIC_CREATE_ECP_OUT_FLAG_TIMESTAMPS_RETURNED: i32 = 256;
pub const ATOMIC_CREATE_ECP_OUT_FLAG_TIMESTAMPS_SET: i32 = 16;
pub const ATOMIC_CREATE_ECP_OUT_FLAG_USN_CLOSE_RECORD_WRITTEN: i32 = 2048;
pub const ATOMIC_CREATE_ECP_OUT_FLAG_USN_RETURNED: i32 = 4096;
pub const ATOMIC_CREATE_ECP_OUT_FLAG_USN_SOURCE_INFO_MARKED: i32 = 1024;
pub const ATOMIC_CREATE_ECP_OUT_FLAG_VDL_SET: i32 = 8;
pub const ATOMIC_CREATE_ECP_OUT_OP_FLAG_CASE_SENSITIVE_FLAGS_SET: i32 = 1;
pub const AuditAccessCheck: SE_AUDIT_OPERATION = 2;
pub const AuditCloseNonObject: SE_AUDIT_OPERATION = 9;
pub const AuditCloseObject: SE_AUDIT_OPERATION = 5;
pub const AuditDeleteObject: SE_AUDIT_OPERATION = 6;
pub const AuditHandleCreation: SE_AUDIT_OPERATION = 12;
pub const AuditObjectReference: SE_AUDIT_OPERATION = 11;
pub const AuditOpenNonObject: SE_AUDIT_OPERATION = 10;
pub const AuditOpenObject: SE_AUDIT_OPERATION = 3;
pub const AuditOpenObjectForDelete: SE_AUDIT_OPERATION = 7;
pub const AuditOpenObjectForDeleteWithTransaction: SE_AUDIT_OPERATION = 8;
pub const AuditOpenObjectWithTransaction: SE_AUDIT_OPERATION = 4;
pub const AuditPrivilegeObject: SE_AUDIT_OPERATION = 0;
pub const AuditPrivilegeService: SE_AUDIT_OPERATION = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BASE_MCB {
    pub MaximumPairCount: u32,
    pub PairCount: u32,
    pub PoolType: u16,
    pub Flags: u16,
    pub Mapping: *mut core::ffi::c_void,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default)]
pub struct CACHE_MANAGER_CALLBACKS {
    pub AcquireForLazyWrite: PACQUIRE_FOR_LAZY_WRITE,
    pub ReleaseFromLazyWrite: PRELEASE_FROM_LAZY_WRITE,
    pub AcquireForReadAhead: PACQUIRE_FOR_READ_AHEAD,
    pub ReleaseFromReadAhead: PRELEASE_FROM_READ_AHEAD,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct CACHE_MANAGER_CALLBACKS_EX {
    pub Version: u16,
    pub Size: u16,
    pub Functions: CACHE_MANAGER_CALLBACK_FUNCTIONS,
}
pub const CACHE_MANAGER_CALLBACKS_EX_V1: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct CACHE_MANAGER_CALLBACK_FUNCTIONS {
    pub AcquireForLazyWriteEx: PACQUIRE_FOR_LAZY_WRITE_EX,
    pub ReleaseFromLazyWrite: PRELEASE_FROM_LAZY_WRITE,
    pub AcquireForReadAhead: PACQUIRE_FOR_READ_AHEAD,
    pub ReleaseFromReadAhead: PRELEASE_FROM_READ_AHEAD,
}
#[repr(C)]
#[cfg(all(feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct CACHE_UNINITIALIZE_EVENT {
    pub Next: *mut Self,
    pub Event: super::KEVENT,
}
#[cfg(all(feature = "wdm", feature = "winnt"))]
impl Default for CACHE_UNINITIALIZE_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CACHE_USE_DIRECT_ACCESS_MAPPING: i32 = 1;
pub const CACHE_VALID_FLAGS: i32 = 1;
pub const CC_ACQUIRE_DONT_WAIT: i32 = 1;
pub const CC_ACQUIRE_SUPPORTS_ASYNC_LAZYWRITE: i32 = 1;
pub const CC_AGGRESSIVE_UNMAP_BEHIND: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "usb", feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct CC_ASYNC_READ_CONTEXT {
    pub CompletionRoutine: PASYNC_READ_COMPLETION_CALLBACK,
    pub Context: *mut core::ffi::c_void,
    pub Mdl: super::PMDL,
    pub RequestorMode: super::KPROCESSOR_MODE,
    pub NestingLevel: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CC_DIRTY_PAGES_INFO {
    pub Version: u32,
    pub DirtyPageThreshold: i64,
    pub DirtyPageTarget: i64,
    pub CleanLockedCachedPagesThreshold: i64,
    pub CleanLockedCachedPagesTarget: i64,
    pub CurrentDirtyPages: i64,
    pub CurrentCleanLockedCachedPages: i64,
    pub CurrentPagesQueuedForWriting: i64,
}
pub const CC_DIRTY_PAGES_INFO_V1: i32 = 1;
pub const CC_DISABLE_DIRTY_PAGE_TRACKING: i32 = 8;
pub const CC_DISABLE_READ_AHEAD: i32 = 2;
pub const CC_DISABLE_UNMAP_BEHIND: i32 = 32;
pub const CC_DISABLE_WRITE_BEHIND: i32 = 4;
pub const CC_ENABLE_CPU_CACHE: i32 = 268435456;
pub const CC_ENABLE_DISK_IO_ACCOUNTING: i32 = 16;
#[repr(C)]
#[cfg(feature = "ntdef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CC_ERROR_CALLBACK_CONTEXT {
    pub NodeByteSize: super::CSHORT,
    pub ErrorCode: windows_core::NTSTATUS,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct CC_FILE_SIZES {
    pub AllocationSize: super::LARGE_INTEGER,
    pub FileSize: super::LARGE_INTEGER,
    pub ValidDataLength: super::LARGE_INTEGER,
}
#[cfg(feature = "winnt")]
impl Default for CC_FILE_SIZES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CC_FLUSH_AND_PURGE_GATHER_DIRTY_BITS: i32 = 2;
pub const CC_FLUSH_AND_PURGE_NO_PURGE: i32 = 1;
pub const CC_FLUSH_AND_PURGE_WRITEABLE_VIEWS_NOTSEEN: i32 = 4;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct COMPRESSED_DATA_INFO {
    pub CompressionFormatAndEngine: u16,
    pub CompressionUnitShift: u8,
    pub ChunkShift: u8,
    pub ClusterShift: u8,
    pub Reserved: u8,
    pub NumberOfChunks: u16,
    pub CompressedChunkSizes: [u32; 1],
}
impl Default for COMPRESSED_DATA_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const COMPRESSION_ENGINE_MASK: i32 = 65280;
pub const COMPRESSION_ENGINE_MAX: i32 = 512;
pub const COMPRESSION_FORMAT_ENGINE_MASK: i32 = 65535;
pub const COMPRESSION_FORMAT_MASK: i32 = 255;
pub const COMPRESSION_FORMAT_MAX: i32 = 8;
pub const COPY_FILE_CHUNK_DUPLICATE_EXTENTS: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "lsalookup", feature = "ntdef", feature = "ntsecapi", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COPY_INFORMATION {
    pub SourceFileObject: super::PFILE_OBJECT,
    pub SourceFileOffset: i64,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CREATE_REDIRECTION_ECP_CONTEXT {
    pub Size: u16,
    pub Flags: u16,
    pub FileId: super::FILE_ID_128,
    pub VolumeGuid: windows_core::GUID,
}
pub const CREATE_REDIRECTION_FLAGS_SERVICED_FROM_LAYER: i32 = 1;
pub const CREATE_REDIRECTION_FLAGS_SERVICED_FROM_REGISTERED_LAYER: i32 = 4;
pub const CREATE_REDIRECTION_FLAGS_SERVICED_FROM_REMOTE_LAYER: i32 = 8;
pub const CREATE_REDIRECTION_FLAGS_SERVICED_FROM_SCRATCH: i32 = 2;
pub const CREATE_REDIRECTION_FLAGS_SERVICED_FROM_USER_MODE: i32 = 16;
pub type CSV_DOWN_LEVEL_FILE_TYPE = i32;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CSV_DOWN_LEVEL_OPEN_ECP_CONTEXT {
    pub Version: u32,
    pub IsResume: super::BOOLEAN,
    pub FileType: CSV_DOWN_LEVEL_FILE_TYPE,
    pub SourceNodeId: u32,
    pub DestinationNodeId: u32,
}
pub const CSV_DOWN_LEVEL_OPEN_ECP_CONTEXT_V1: i32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CSV_QUERY_FILE_REVISION_ECP_CONTEXT {
    pub FileId: i64,
    pub FileRevision: [i64; 3],
}
impl Default for CSV_QUERY_FILE_REVISION_ECP_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CSV_QUERY_FILE_REVISION_ECP_CONTEXT_FILE_ID_128 {
    pub FileId: super::FILE_ID_128,
    pub FileRevision: [i64; 3],
}
#[cfg(feature = "winnt")]
impl Default for CSV_QUERY_FILE_REVISION_ECP_CONTEXT_FILE_ID_128 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CSV_SET_HANDLE_PROPERTIES_ECP_CONTEXT {
    pub Size: usize,
    pub PauseTimeoutInSeconds: u32,
    pub Flags: u32,
}
pub const CSV_SET_HANDLE_PROPERTIES_ECP_CONTEXT_FLAGS_VALID_ONLY_IF_CSV_COORDINATOR: i32 = 1;
pub const ChangeDataControlArea: FSRTL_CHANGE_BACKING_TYPE = 0;
pub const ChangeImageControlArea: FSRTL_CHANGE_BACKING_TYPE = 1;
pub const ChangeSharedCacheMap: FSRTL_CHANGE_BACKING_TYPE = 2;
pub const CsvCsvFsInternalFileObject: CSV_DOWN_LEVEL_FILE_TYPE = 1;
pub const CsvDownLevelFileObject: CSV_DOWN_LEVEL_FILE_TYPE = 0;
pub const DD_MUP_DEVICE_NAME: windows_core::PCWSTR = windows_core::w!("\\Device\\Mup");
pub const DEVICE_RESET_KEEP_STACK: i32 = 4;
pub const DEVICE_RESET_RESERVED_0: i32 = 1;
pub const DEVICE_RESET_RESERVED_1: i32 = 2;
pub const DOS_DOT: u16 = 34;
pub const DOS_QM: u16 = 62;
pub const DOS_STAR: u16 = 60;
pub const DO_NOT_PURGE_DIRTY_PAGES: i32 = 4;
pub const DO_NOT_RETRY_PURGE: i32 = 2;
pub const DO_SUPPORTS_PERSISTENT_ACLS: i32 = 131072;
#[cfg(all(feature = "basetsd", feature = "lsalookup", feature = "ntdef", feature = "ntsecapi", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
pub type DRIVER_FS_NOTIFICATION = Option<unsafe extern "C" fn(deviceobject: *const super::DEVICE_OBJECT, fsactive: super::BOOLEAN)>;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DUAL_OPLOCK_KEY_ECP_CONTEXT {
    pub ParentOplockKey: windows_core::GUID,
    pub TargetOplockKey: windows_core::GUID,
    pub ParentOplockKeySet: super::BOOLEAN,
    pub TargetOplockKeySet: super::BOOLEAN,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct DUPLICATE_CLUSTER_DATA {
    pub SourceLcn: i64,
    pub TargetLcn: i64,
    pub TargetFileOffset: super::LARGE_INTEGER,
    pub DuplicationLimit: u32,
    pub Reserved: u32,
}
#[cfg(feature = "winnt")]
impl Default for DUPLICATE_CLUSTER_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DfsLinkTrackingInformation: LINK_TRACKING_INFORMATION_TYPE = 1;
pub const EA_NAME_NETWORK_OPEN_ECP_INTEGRITY: windows_core::PCSTR = windows_core::s!("ECP{c584edbf-00df-4d28-00b8-8435baca8911e8}-INTEGRITY");
pub const EA_NAME_NETWORK_OPEN_ECP_INTEGRITY_U: windows_core::PCWSTR = windows_core::w!("ECP{c584edbf-00df-4d28-00b8-8435baca8911e8}-INTEGRITY");
pub const EA_NAME_NETWORK_OPEN_ECP_PRIVACY: windows_core::PCSTR = windows_core::s!("ECP{c584edbf-00df-4d28-00b8-8435baca8911e8}-PRIVACY");
pub const EA_NAME_NETWORK_OPEN_ECP_PRIVACY_U: windows_core::PCWSTR = windows_core::w!("ECP{c584edbf-00df-4d28-00b8-8435baca8911e8}-PRIVACY");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ECP_HEADER(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ECP_OPEN_PARAMETERS {
    pub Size: u16,
    pub Reserved: u16,
    pub Flags: u32,
}
pub const ECP_OPEN_PARAMETERS_FLAG_FAIL_ON_CASE_SENSITIVE_DIR: i32 = 16;
pub const ECP_OPEN_PARAMETERS_FLAG_IGNORE_DIR_CASE_SENSITIVITY: i32 = 8;
pub const ECP_OPEN_PARAMETERS_FLAG_OPEN_FOR_DELETE: i32 = 4;
pub const ECP_OPEN_PARAMETERS_FLAG_OPEN_FOR_READ: i32 = 1;
pub const ECP_OPEN_PARAMETERS_FLAG_OPEN_FOR_WRITE: i32 = 2;
pub const ECP_TYPE_CLFS_CREATE_CONTAINER: windows_core::GUID = windows_core::GUID::from_u128(0x8650c9fe_0cec_8bf6_bd1e_835956541090);
pub const ECP_TYPE_IO_STOP_ON_SYMLINK_FILTER_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x940e5d56_1646_4d3c_87b6_577ec36a1466);
pub const ECP_TYPE_OPEN_REPARSE_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x323eb6a8_affd_4d95_8230_863bce09d37a);
#[repr(C)]
#[cfg(all(feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct EOF_WAIT_BLOCK {
    pub EofWaitLinks: super::LIST_ENTRY,
    pub Event: super::KEVENT,
}
#[cfg(all(feature = "wdm", feature = "winnt"))]
impl Default for EOF_WAIT_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct EXTENT_READ_CACHE_INFO_BUFFER {
    pub AllocatedCache: super::LARGE_INTEGER,
    pub PopulatedCache: super::LARGE_INTEGER,
    pub InErrorCache: super::LARGE_INTEGER,
}
#[cfg(feature = "winnt")]
impl Default for EXTENT_READ_CACHE_INFO_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const EqualTo: FSRTL_COMPARISON_RESULT = 0;
pub type FAST_IO_POSSIBLE = i32;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILE_ACCESS_INFORMATION {
    pub AccessFlags: super::ACCESS_MASK,
}
pub const FILE_ACTION_ADDED_STREAM: i32 = 6;
pub const FILE_ACTION_ID_NOT_TUNNELLED: i32 = 10;
pub const FILE_ACTION_MODIFIED_STREAM: i32 = 8;
pub const FILE_ACTION_REMOVED_BY_DELETE: i32 = 9;
pub const FILE_ACTION_REMOVED_STREAM: i32 = 7;
pub const FILE_ACTION_TUNNELLED_ID_COLLISION: i32 = 11;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct FILE_ALLOCATION_INFORMATION {
    pub AllocationSize: super::LARGE_INTEGER,
}
#[cfg(feature = "winnt")]
impl Default for FILE_ALLOCATION_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ntddk", feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct FILE_ALL_INFORMATION {
    pub BasicInformation: super::FILE_BASIC_INFORMATION,
    pub StandardInformation: super::FILE_STANDARD_INFORMATION,
    pub InternalInformation: FILE_INTERNAL_INFORMATION,
    pub EaInformation: FILE_EA_INFORMATION,
    pub AccessInformation: FILE_ACCESS_INFORMATION,
    pub PositionInformation: super::FILE_POSITION_INFORMATION,
    pub ModeInformation: FILE_MODE_INFORMATION,
    pub AlignmentInformation: super::FILE_ALIGNMENT_INFORMATION,
    pub NameInformation: super::FILE_NAME_INFORMATION,
}
#[cfg(all(feature = "ntddk", feature = "wdm", feature = "winnt"))]
impl Default for FILE_ALL_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct FILE_BOTH_DIR_INFORMATION {
    pub NextEntryOffset: u32,
    pub FileIndex: u32,
    pub CreationTime: super::LARGE_INTEGER,
    pub LastAccessTime: super::LARGE_INTEGER,
    pub LastWriteTime: super::LARGE_INTEGER,
    pub ChangeTime: super::LARGE_INTEGER,
    pub EndOfFile: super::LARGE_INTEGER,
    pub AllocationSize: super::LARGE_INTEGER,
    pub FileAttributes: u32,
    pub FileNameLength: u32,
    pub EaSize: u32,
    pub ShortNameLength: super::CCHAR,
    pub ShortName: [u16; 12],
    pub FileName: [u16; 1],
}
#[cfg(feature = "winnt")]
impl Default for FILE_BOTH_DIR_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FILE_CLEANUP_FILE_DELETED: i32 = 4;
pub const FILE_CLEANUP_FILE_REMAINS: i32 = 2;
pub const FILE_CLEANUP_LINK_DELETED: i32 = 8;
pub const FILE_CLEANUP_POSIX_STYLE_DELETE: i32 = 32;
pub const FILE_CLEANUP_STREAM_DELETED: i32 = 16;
pub const FILE_CLEANUP_UNKNOWN: i32 = 0;
pub const FILE_CLEANUP_WRONG_DEVICE: i32 = 1;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILE_COMPLETION_INFORMATION {
    pub Port: super::HANDLE,
    pub Key: *mut core::ffi::c_void,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct FILE_COMPRESSION_INFORMATION {
    pub CompressedFileSize: super::LARGE_INTEGER,
    pub CompressionFormat: u16,
    pub CompressionUnitShift: u8,
    pub ChunkShift: u8,
    pub ClusterShift: u8,
    pub Reserved: [u8; 3],
}
#[cfg(feature = "winnt")]
impl Default for FILE_COMPRESSION_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct FILE_DIRECTORY_INFORMATION {
    pub NextEntryOffset: u32,
    pub FileIndex: u32,
    pub CreationTime: super::LARGE_INTEGER,
    pub LastAccessTime: super::LARGE_INTEGER,
    pub LastWriteTime: super::LARGE_INTEGER,
    pub ChangeTime: super::LARGE_INTEGER,
    pub EndOfFile: super::LARGE_INTEGER,
    pub AllocationSize: super::LARGE_INTEGER,
    pub FileAttributes: u32,
    pub FileNameLength: u32,
    pub FileName: [u16; 1],
}
#[cfg(feature = "winnt")]
impl Default for FILE_DIRECTORY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILE_EA_INFORMATION {
    pub EaSize: u32,
}
pub const FILE_EA_TYPE_ASCII: i32 = 65533;
pub const FILE_EA_TYPE_ASN1: i32 = 65501;
pub const FILE_EA_TYPE_BINARY: i32 = 65534;
pub const FILE_EA_TYPE_BITMAP: i32 = 65531;
pub const FILE_EA_TYPE_EA: i32 = 65518;
pub const FILE_EA_TYPE_FAMILY_IDS: i32 = 65281;
pub const FILE_EA_TYPE_ICON: i32 = 65529;
pub const FILE_EA_TYPE_METAFILE: i32 = 65530;
pub const FILE_EA_TYPE_MVMT: i32 = 65503;
pub const FILE_EA_TYPE_MVST: i32 = 65502;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct FILE_END_OF_FILE_INFORMATION_EX {
    pub EndOfFile: super::LARGE_INTEGER,
    pub PagingFileSizeInMM: super::LARGE_INTEGER,
    pub PagingFileMaxSize: super::LARGE_INTEGER,
    pub Flags: u32,
}
#[cfg(feature = "winnt")]
impl Default for FILE_END_OF_FILE_INFORMATION_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_FS_ATTRIBUTE_INFORMATION {
    pub FileSystemAttributes: u32,
    pub MaximumComponentNameLength: i32,
    pub FileSystemNameLength: u32,
    pub FileSystemName: [u16; 1],
}
impl Default for FILE_FS_ATTRIBUTE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct FILE_FS_CONTROL_INFORMATION {
    pub FreeSpaceStartFiltering: super::LARGE_INTEGER,
    pub FreeSpaceThreshold: super::LARGE_INTEGER,
    pub FreeSpaceStopFiltering: super::LARGE_INTEGER,
    pub DefaultQuotaThreshold: super::LARGE_INTEGER,
    pub DefaultQuotaLimit: super::LARGE_INTEGER,
    pub FileSystemControlFlags: u32,
}
#[cfg(feature = "winnt")]
impl Default for FILE_FS_CONTROL_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILE_FS_DATA_COPY_INFORMATION {
    pub NumberOfCopies: u32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_FS_DRIVER_PATH_INFORMATION {
    pub DriverInPath: super::BOOLEAN,
    pub DriverNameLength: u32,
    pub DriverName: [u16; 1],
}
#[cfg(feature = "winnt")]
impl Default for FILE_FS_DRIVER_PATH_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILE_FS_GUID_INFORMATION {
    pub FsGuid: windows_core::GUID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILE_FS_VOLUME_FLAGS_INFORMATION {
    pub Flags: u32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct FILE_FULL_DIR_INFORMATION {
    pub NextEntryOffset: u32,
    pub FileIndex: u32,
    pub CreationTime: super::LARGE_INTEGER,
    pub LastAccessTime: super::LARGE_INTEGER,
    pub LastWriteTime: super::LARGE_INTEGER,
    pub ChangeTime: super::LARGE_INTEGER,
    pub EndOfFile: super::LARGE_INTEGER,
    pub AllocationSize: super::LARGE_INTEGER,
    pub FileAttributes: u32,
    pub FileNameLength: u32,
    pub EaSize: u32,
    pub FileName: [u16; 1],
}
#[cfg(feature = "winnt")]
impl Default for FILE_FULL_DIR_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_GET_EA_INFORMATION {
    pub NextEntryOffset: u32,
    pub EaNameLength: u8,
    pub EaName: [i8; 1],
}
impl Default for FILE_GET_EA_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILE_GET_QUOTA_INFORMATION {
    pub NextEntryOffset: u32,
    pub SidLength: u32,
    pub Sid: super::SID,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct FILE_ID_64_EXTD_BOTH_DIR_INFORMATION {
    pub NextEntryOffset: u32,
    pub FileIndex: u32,
    pub CreationTime: super::LARGE_INTEGER,
    pub LastAccessTime: super::LARGE_INTEGER,
    pub LastWriteTime: super::LARGE_INTEGER,
    pub ChangeTime: super::LARGE_INTEGER,
    pub EndOfFile: super::LARGE_INTEGER,
    pub AllocationSize: super::LARGE_INTEGER,
    pub FileAttributes: u32,
    pub FileNameLength: u32,
    pub EaSize: u32,
    pub ReparsePointTag: u32,
    pub FileId: super::LARGE_INTEGER,
    pub ShortNameLength: super::CCHAR,
    pub ShortName: [u16; 12],
    pub FileName: [u16; 1],
}
#[cfg(feature = "winnt")]
impl Default for FILE_ID_64_EXTD_BOTH_DIR_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct FILE_ID_64_EXTD_DIR_INFORMATION {
    pub NextEntryOffset: u32,
    pub FileIndex: u32,
    pub CreationTime: super::LARGE_INTEGER,
    pub LastAccessTime: super::LARGE_INTEGER,
    pub LastWriteTime: super::LARGE_INTEGER,
    pub ChangeTime: super::LARGE_INTEGER,
    pub EndOfFile: super::LARGE_INTEGER,
    pub AllocationSize: super::LARGE_INTEGER,
    pub FileAttributes: u32,
    pub FileNameLength: u32,
    pub EaSize: u32,
    pub ReparsePointTag: u32,
    pub FileId: super::LARGE_INTEGER,
    pub FileName: [u16; 1],
}
#[cfg(feature = "winnt")]
impl Default for FILE_ID_64_EXTD_DIR_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct FILE_ID_ALL_EXTD_BOTH_DIR_INFORMATION {
    pub NextEntryOffset: u32,
    pub FileIndex: u32,
    pub CreationTime: super::LARGE_INTEGER,
    pub LastAccessTime: super::LARGE_INTEGER,
    pub LastWriteTime: super::LARGE_INTEGER,
    pub ChangeTime: super::LARGE_INTEGER,
    pub EndOfFile: super::LARGE_INTEGER,
    pub AllocationSize: super::LARGE_INTEGER,
    pub FileAttributes: u32,
    pub FileNameLength: u32,
    pub EaSize: u32,
    pub ReparsePointTag: u32,
    pub FileId: super::LARGE_INTEGER,
    pub FileId128: super::FILE_ID_128,
    pub ShortNameLength: super::CCHAR,
    pub ShortName: [u16; 12],
    pub FileName: [u16; 1],
}
#[cfg(feature = "winnt")]
impl Default for FILE_ID_ALL_EXTD_BOTH_DIR_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct FILE_ID_ALL_EXTD_DIR_INFORMATION {
    pub NextEntryOffset: u32,
    pub FileIndex: u32,
    pub CreationTime: super::LARGE_INTEGER,
    pub LastAccessTime: super::LARGE_INTEGER,
    pub LastWriteTime: super::LARGE_INTEGER,
    pub ChangeTime: super::LARGE_INTEGER,
    pub EndOfFile: super::LARGE_INTEGER,
    pub AllocationSize: super::LARGE_INTEGER,
    pub FileAttributes: u32,
    pub FileNameLength: u32,
    pub EaSize: u32,
    pub ReparsePointTag: u32,
    pub FileId: super::LARGE_INTEGER,
    pub FileId128: super::FILE_ID_128,
    pub FileName: [u16; 1],
}
#[cfg(feature = "winnt")]
impl Default for FILE_ID_ALL_EXTD_DIR_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct FILE_ID_BOTH_DIR_INFORMATION {
    pub NextEntryOffset: u32,
    pub FileIndex: u32,
    pub CreationTime: super::LARGE_INTEGER,
    pub LastAccessTime: super::LARGE_INTEGER,
    pub LastWriteTime: super::LARGE_INTEGER,
    pub ChangeTime: super::LARGE_INTEGER,
    pub EndOfFile: super::LARGE_INTEGER,
    pub AllocationSize: super::LARGE_INTEGER,
    pub FileAttributes: u32,
    pub FileNameLength: u32,
    pub EaSize: u32,
    pub ShortNameLength: super::CCHAR,
    pub ShortName: [u16; 12],
    pub FileId: super::LARGE_INTEGER,
    pub FileName: [u16; 1],
}
#[cfg(feature = "winnt")]
impl Default for FILE_ID_BOTH_DIR_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct FILE_ID_EXTD_BOTH_DIR_INFORMATION {
    pub NextEntryOffset: u32,
    pub FileIndex: u32,
    pub CreationTime: super::LARGE_INTEGER,
    pub LastAccessTime: super::LARGE_INTEGER,
    pub LastWriteTime: super::LARGE_INTEGER,
    pub ChangeTime: super::LARGE_INTEGER,
    pub EndOfFile: super::LARGE_INTEGER,
    pub AllocationSize: super::LARGE_INTEGER,
    pub FileAttributes: u32,
    pub FileNameLength: u32,
    pub EaSize: u32,
    pub ReparsePointTag: u32,
    pub FileId: super::FILE_ID_128,
    pub ShortNameLength: super::CCHAR,
    pub ShortName: [u16; 12],
    pub FileName: [u16; 1],
}
#[cfg(feature = "winnt")]
impl Default for FILE_ID_EXTD_BOTH_DIR_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct FILE_ID_EXTD_DIR_INFORMATION {
    pub NextEntryOffset: u32,
    pub FileIndex: u32,
    pub CreationTime: super::LARGE_INTEGER,
    pub LastAccessTime: super::LARGE_INTEGER,
    pub LastWriteTime: super::LARGE_INTEGER,
    pub ChangeTime: super::LARGE_INTEGER,
    pub EndOfFile: super::LARGE_INTEGER,
    pub AllocationSize: super::LARGE_INTEGER,
    pub FileAttributes: u32,
    pub FileNameLength: u32,
    pub EaSize: u32,
    pub ReparsePointTag: u32,
    pub FileId: super::FILE_ID_128,
    pub FileName: [u16; 1],
}
#[cfg(feature = "winnt")]
impl Default for FILE_ID_EXTD_DIR_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct FILE_ID_FULL_DIR_INFORMATION {
    pub NextEntryOffset: u32,
    pub FileIndex: u32,
    pub CreationTime: super::LARGE_INTEGER,
    pub LastAccessTime: super::LARGE_INTEGER,
    pub LastWriteTime: super::LARGE_INTEGER,
    pub ChangeTime: super::LARGE_INTEGER,
    pub EndOfFile: super::LARGE_INTEGER,
    pub AllocationSize: super::LARGE_INTEGER,
    pub FileAttributes: u32,
    pub FileNameLength: u32,
    pub EaSize: u32,
    pub FileId: super::LARGE_INTEGER,
    pub FileName: [u16; 1],
}
#[cfg(feature = "winnt")]
impl Default for FILE_ID_FULL_DIR_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct FILE_ID_GLOBAL_TX_DIR_INFORMATION {
    pub NextEntryOffset: u32,
    pub FileIndex: u32,
    pub CreationTime: super::LARGE_INTEGER,
    pub LastAccessTime: super::LARGE_INTEGER,
    pub LastWriteTime: super::LARGE_INTEGER,
    pub ChangeTime: super::LARGE_INTEGER,
    pub EndOfFile: super::LARGE_INTEGER,
    pub AllocationSize: super::LARGE_INTEGER,
    pub FileAttributes: u32,
    pub FileNameLength: u32,
    pub FileId: super::LARGE_INTEGER,
    pub LockingTransactionId: windows_core::GUID,
    pub TxInfoFlags: u32,
    pub FileName: [u16; 1],
}
#[cfg(feature = "winnt")]
impl Default for FILE_ID_GLOBAL_TX_DIR_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FILE_ID_GLOBAL_TX_DIR_INFO_FLAG_VISIBLE_OUTSIDE_TX: i32 = 4;
pub const FILE_ID_GLOBAL_TX_DIR_INFO_FLAG_VISIBLE_TO_TX: i32 = 2;
pub const FILE_ID_GLOBAL_TX_DIR_INFO_FLAG_WRITELOCKED: i32 = 1;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILE_ID_INFORMATION {
    pub VolumeSerialNumber: u64,
    pub FileId: super::FILE_ID_128,
}
#[repr(C)]
#[cfg(feature = "wdm")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILE_INFORMATION_DEFINITION {
    pub Class: super::FILE_INFORMATION_CLASS,
    pub NextEntryOffset: u32,
    pub FileNameOffset: u32,
    pub FileNameLengthOffset: u32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct FILE_INTERNAL_INFORMATION {
    pub IndexNumber: super::LARGE_INTEGER,
}
#[cfg(feature = "winnt")]
impl Default for FILE_INTERNAL_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILE_KNOWN_FOLDER_INFORMATION {
    pub Type: FILE_KNOWN_FOLDER_TYPE,
}
pub type FILE_KNOWN_FOLDER_TYPE = i32;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILE_LINKS_FULL_ID_INFORMATION {
    pub BytesNeeded: u32,
    pub EntriesReturned: u32,
    pub Entry: FILE_LINK_ENTRY_FULL_ID_INFORMATION,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILE_LINKS_INFORMATION {
    pub BytesNeeded: u32,
    pub EntriesReturned: u32,
    pub Entry: FILE_LINK_ENTRY_INFORMATION,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_LINK_ENTRY_FULL_ID_INFORMATION {
    pub NextEntryOffset: u32,
    pub ParentFileId: super::FILE_ID_128,
    pub FileNameLength: u32,
    pub FileName: [u16; 1],
}
#[cfg(feature = "winnt")]
impl Default for FILE_LINK_ENTRY_FULL_ID_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_LINK_ENTRY_INFORMATION {
    pub NextEntryOffset: u32,
    pub ParentFileId: i64,
    pub FileNameLength: u32,
    pub FileName: [u16; 1],
}
impl Default for FILE_LINK_ENTRY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FILE_LINK_FORCE_RESIZE_SOURCE_SR: i32 = 256;
pub const FILE_LINK_FORCE_RESIZE_SR: i32 = 384;
pub const FILE_LINK_FORCE_RESIZE_TARGET_SR: i32 = 128;
pub const FILE_LINK_IGNORE_READONLY_ATTRIBUTE: i32 = 64;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct FILE_LINK_INFORMATION {
    pub Anonymous: FILE_LINK_INFORMATION_0,
    pub RootDirectory: super::HANDLE,
    pub FileNameLength: u32,
    pub FileName: [u16; 1],
}
#[cfg(feature = "winnt")]
impl Default for FILE_LINK_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union FILE_LINK_INFORMATION_0 {
    pub ReplaceIfExists: super::BOOLEAN,
    pub Flags: u32,
}
#[cfg(feature = "winnt")]
impl Default for FILE_LINK_INFORMATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FILE_LINK_NO_DECREASE_AVAILABLE_SPACE: i32 = 32;
pub const FILE_LINK_NO_INCREASE_AVAILABLE_SPACE: i32 = 16;
pub const FILE_LINK_POSIX_SEMANTICS: i32 = 2;
pub const FILE_LINK_PRESERVE_AVAILABLE_SPACE: i32 = 48;
pub const FILE_LINK_REPLACE_IF_EXISTS: i32 = 1;
pub const FILE_LINK_SUPPRESS_STORAGE_RESERVE_INHERITANCE: i32 = 8;
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "lsalookup", feature = "ntdef", feature = "ntsecapi", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy)]
pub struct FILE_LOCK {
    pub CompleteLockIrpRoutine: PCOMPLETE_LOCK_IRP_ROUTINE,
    pub UnlockRoutine: PUNLOCK_ROUTINE,
    pub FastIoIsQuestionable: super::BOOLEAN,
    pub SpareC: [super::BOOLEAN; 3],
    pub LockInformation: *mut core::ffi::c_void,
    pub LastReturnedLockInfo: FILE_LOCK_INFO,
    pub LastReturnedLock: *mut core::ffi::c_void,
    pub LockRequestsInProgress: i32,
}
#[cfg(all(feature = "basetsd", feature = "lsalookup", feature = "ntdef", feature = "ntsecapi", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
impl Default for FILE_LOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "lsalookup", feature = "ntdef", feature = "ntsecapi", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy)]
pub struct FILE_LOCK_INFO {
    pub StartingByte: super::LARGE_INTEGER,
    pub Length: super::LARGE_INTEGER,
    pub ExclusiveLock: super::BOOLEAN,
    pub Key: u32,
    pub FileObject: super::PFILE_OBJECT,
    pub ProcessId: *mut core::ffi::c_void,
    pub EndingByte: super::LARGE_INTEGER,
}
#[cfg(all(feature = "basetsd", feature = "lsalookup", feature = "ntdef", feature = "ntsecapi", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
impl Default for FILE_LOCK_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct FILE_MAILSLOT_QUERY_INFORMATION {
    pub MaximumMessageSize: u32,
    pub MailslotQuota: u32,
    pub NextMessageSize: u32,
    pub MessagesAvailable: u32,
    pub ReadTimeout: super::LARGE_INTEGER,
}
#[cfg(feature = "winnt")]
impl Default for FILE_MAILSLOT_QUERY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILE_MAILSLOT_SET_INFORMATION {
    pub ReadTimeout: super::PLARGE_INTEGER,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILE_MODE_INFORMATION {
    pub Mode: u32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_MOVE_CLUSTER_INFORMATION {
    pub ClusterCount: u32,
    pub RootDirectory: super::HANDLE,
    pub FileNameLength: u32,
    pub FileName: [u16; 1],
}
#[cfg(feature = "winnt")]
impl Default for FILE_MOVE_CLUSTER_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_NAMES_INFORMATION {
    pub NextEntryOffset: u32,
    pub FileIndex: u32,
    pub FileNameLength: u32,
    pub FileName: [u16; 1],
}
impl Default for FILE_NAMES_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FILE_NEED_EA: i32 = 128;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_NETWORK_PHYSICAL_NAME_INFORMATION {
    pub FileNameLength: u32,
    pub FileName: [u16; 1],
}
impl Default for FILE_NETWORK_PHYSICAL_NAME_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FILE_NOTIFY_CHANGE_EA: i32 = 128;
pub const FILE_NOTIFY_CHANGE_NAME: i32 = 3;
pub const FILE_NOTIFY_CHANGE_STREAM_NAME: i32 = 512;
pub const FILE_NOTIFY_CHANGE_STREAM_SIZE: i32 = 1024;
pub const FILE_NOTIFY_CHANGE_STREAM_WRITE: i32 = 2048;
pub const FILE_NOTIFY_VALID_MASK: i32 = 4095;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FILE_OBJECTID_INFORMATION {
    pub FileReference: i64,
    pub ObjectId: [u8; 16],
    pub Anonymous: FILE_OBJECTID_INFORMATION_0,
}
impl Default for FILE_OBJECTID_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union FILE_OBJECTID_INFORMATION_0 {
    pub Anonymous: FILE_OBJECTID_INFORMATION_0_0,
    pub ExtendedInfo: [u8; 48],
}
impl Default for FILE_OBJECTID_INFORMATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_OBJECTID_INFORMATION_0_0 {
    pub BirthVolumeId: [u8; 16],
    pub BirthObjectId: [u8; 16],
    pub DomainId: [u8; 16],
}
impl Default for FILE_OBJECTID_INFORMATION_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FILE_OPBATCH_BREAK_UNDERWAY: i32 = 9;
pub const FILE_OPLOCK_BROKEN_TO_LEVEL_2: i32 = 7;
pub const FILE_OPLOCK_BROKEN_TO_NONE: i32 = 8;
pub const FILE_PIPE_ACCEPT_REMOTE_CLIENTS: i32 = 0;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILE_PIPE_ASSIGN_EVENT_BUFFER {
    pub EventHandle: super::HANDLE,
    pub KeyValue: u32,
}
pub const FILE_PIPE_BYTE_STREAM_MODE: i32 = 0;
pub const FILE_PIPE_BYTE_STREAM_TYPE: i32 = 0;
pub const FILE_PIPE_CLIENT_END: i32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILE_PIPE_CLIENT_PROCESS_BUFFER {
    pub ClientSession: *mut core::ffi::c_void,
    pub ClientProcess: *mut core::ffi::c_void,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_PIPE_CLIENT_PROCESS_BUFFER_EX {
    pub ClientSession: *mut core::ffi::c_void,
    pub ClientProcess: *mut core::ffi::c_void,
    pub ClientComputerNameLength: u16,
    pub ClientComputerBuffer: [u16; 16],
}
impl Default for FILE_PIPE_CLIENT_PROCESS_BUFFER_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILE_PIPE_CLIENT_PROCESS_BUFFER_V2 {
    pub ClientSession: u64,
    pub ClientProcess: *mut core::ffi::c_void,
}
pub const FILE_PIPE_CLOSING_STATE: i32 = 4;
pub const FILE_PIPE_COMPLETE_OPERATION: i32 = 1;
pub const FILE_PIPE_COMPUTER_NAME_LENGTH: i32 = 15;
pub const FILE_PIPE_CONNECTED_STATE: i32 = 3;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILE_PIPE_CREATE_SYMLINK_INPUT {
    pub NameOffset: u16,
    pub NameLength: u16,
    pub SubstituteNameOffset: u16,
    pub SubstituteNameLength: u16,
    pub Flags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILE_PIPE_DELETE_SYMLINK_INPUT {
    pub NameOffset: u16,
    pub NameLength: u16,
}
pub const FILE_PIPE_DISCONNECTED_STATE: i32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILE_PIPE_EVENT_BUFFER {
    pub NamedPipeState: u32,
    pub EntryType: u32,
    pub ByteCount: u32,
    pub KeyValue: u32,
    pub NumberRequests: u32,
}
pub const FILE_PIPE_FULL_DUPLEX: i32 = 2;
pub const FILE_PIPE_INBOUND: i32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILE_PIPE_INFORMATION {
    pub ReadMode: u32,
    pub CompletionMode: u32,
}
pub const FILE_PIPE_LISTENING_STATE: i32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILE_PIPE_LOCAL_INFORMATION {
    pub NamedPipeType: u32,
    pub NamedPipeConfiguration: u32,
    pub MaximumInstances: u32,
    pub CurrentInstances: u32,
    pub InboundQuota: u32,
    pub ReadDataAvailable: u32,
    pub OutboundQuota: u32,
    pub WriteQuotaAvailable: u32,
    pub NamedPipeState: u32,
    pub NamedPipeEnd: u32,
}
pub const FILE_PIPE_MESSAGE_MODE: i32 = 1;
pub const FILE_PIPE_MESSAGE_TYPE: i32 = 1;
pub const FILE_PIPE_OUTBOUND: i32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_PIPE_PEEK_BUFFER {
    pub NamedPipeState: u32,
    pub ReadDataAvailable: u32,
    pub NumberOfMessages: u32,
    pub MessageLength: u32,
    pub Data: [i8; 1],
}
impl Default for FILE_PIPE_PEEK_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FILE_PIPE_QUEUE_OPERATION: i32 = 0;
pub const FILE_PIPE_READ_DATA: i32 = 0;
pub const FILE_PIPE_REJECT_REMOTE_CLIENTS: i32 = 2;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct FILE_PIPE_REMOTE_INFORMATION {
    pub CollectDataTime: super::LARGE_INTEGER,
    pub MaximumCollectionCount: u32,
}
#[cfg(feature = "winnt")]
impl Default for FILE_PIPE_REMOTE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FILE_PIPE_SERVER_END: i32 = 1;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILE_PIPE_SILO_ARRIVAL_INPUT {
    pub JobHandle: super::HANDLE,
}
pub const FILE_PIPE_SYMLINK_FLAG_GLOBAL: i32 = 1;
pub const FILE_PIPE_SYMLINK_FLAG_RELATIVE: i32 = 2;
pub const FILE_PIPE_SYMLINK_VALID_FLAGS: i32 = 3;
pub const FILE_PIPE_TYPE_VALID_MASK: i32 = 3;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct FILE_PIPE_WAIT_FOR_BUFFER {
    pub Timeout: super::LARGE_INTEGER,
    pub NameLength: u32,
    pub TimeoutSpecified: super::BOOLEAN,
    pub Name: [u16; 1],
}
#[cfg(feature = "winnt")]
impl Default for FILE_PIPE_WAIT_FOR_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FILE_PIPE_WRITE_SPACE: i32 = 1;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct FILE_QUOTA_INFORMATION {
    pub NextEntryOffset: u32,
    pub SidLength: u32,
    pub ChangeTime: super::LARGE_INTEGER,
    pub QuotaUsed: super::LARGE_INTEGER,
    pub QuotaThreshold: super::LARGE_INTEGER,
    pub QuotaLimit: super::LARGE_INTEGER,
    pub Sid: super::SID,
}
#[cfg(feature = "winnt")]
impl Default for FILE_QUOTA_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FILE_REMOTE_PROTOCOL_INFORMATION {
    pub StructureVersion: u16,
    pub StructureSize: u16,
    pub Protocol: u32,
    pub ProtocolMajorVersion: u16,
    pub ProtocolMinorVersion: u16,
    pub ProtocolRevision: u16,
    pub Reserved: u16,
    pub Flags: u32,
    pub GenericReserved: FILE_REMOTE_PROTOCOL_INFORMATION_0,
    pub ProtocolSpecific: FILE_REMOTE_PROTOCOL_INFORMATION_1,
}
impl Default for FILE_REMOTE_PROTOCOL_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_REMOTE_PROTOCOL_INFORMATION_0 {
    pub Reserved: [u32; 8],
}
impl Default for FILE_REMOTE_PROTOCOL_INFORMATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union FILE_REMOTE_PROTOCOL_INFORMATION_1 {
    pub Smb2: FILE_REMOTE_PROTOCOL_INFORMATION_1_0,
    pub Reserved: [u32; 16],
}
impl Default for FILE_REMOTE_PROTOCOL_INFORMATION_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILE_REMOTE_PROTOCOL_INFORMATION_1_0 {
    pub Server: FILE_REMOTE_PROTOCOL_INFORMATION_1_0_0,
    pub Share: FILE_REMOTE_PROTOCOL_INFORMATION_1_0_1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILE_REMOTE_PROTOCOL_INFORMATION_1_0_0 {
    pub Capabilities: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_REMOTE_PROTOCOL_INFORMATION_1_0_1 {
    pub Capabilities: u32,
    pub ShareFlags: u32,
    pub ShareType: u8,
    pub Reserved0: [u8; 3],
    pub Reserved1: u32,
}
impl Default for FILE_REMOTE_PROTOCOL_INFORMATION_1_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FILE_RENAME_FORCE_RESIZE_SOURCE_SR: i32 = 256;
pub const FILE_RENAME_FORCE_RESIZE_SR: i32 = 384;
pub const FILE_RENAME_FORCE_RESIZE_TARGET_SR: i32 = 128;
pub const FILE_RENAME_IGNORE_READONLY_ATTRIBUTE: i32 = 64;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct FILE_RENAME_INFORMATION {
    pub Anonymous: FILE_RENAME_INFORMATION_0,
    pub RootDirectory: super::HANDLE,
    pub FileNameLength: u32,
    pub FileName: [u16; 1],
}
#[cfg(feature = "winnt")]
impl Default for FILE_RENAME_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union FILE_RENAME_INFORMATION_0 {
    pub ReplaceIfExists: super::BOOLEAN,
    pub Flags: u32,
}
#[cfg(feature = "winnt")]
impl Default for FILE_RENAME_INFORMATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FILE_RENAME_NO_DECREASE_AVAILABLE_SPACE: i32 = 32;
pub const FILE_RENAME_NO_INCREASE_AVAILABLE_SPACE: i32 = 16;
pub const FILE_RENAME_POSIX_SEMANTICS: i32 = 2;
pub const FILE_RENAME_PRESERVE_AVAILABLE_SPACE: i32 = 48;
pub const FILE_RENAME_REPLACE_IF_EXISTS: i32 = 1;
pub const FILE_RENAME_SUPPRESS_PIN_STATE_INHERITANCE: i32 = 4;
pub const FILE_RENAME_SUPPRESS_STORAGE_RESERVE_INHERITANCE: i32 = 8;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILE_REPARSE_POINT_INFORMATION {
    pub FileReference: i64,
    pub Tag: u32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILE_STANDARD_LINK_INFORMATION {
    pub NumberOfAccessibleLinks: u32,
    pub TotalNumberOfLinks: u32,
    pub DeletePending: super::BOOLEAN,
    pub Directory: super::BOOLEAN,
}
#[repr(C)]
#[cfg(feature = "winioctl")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILE_STORAGE_RESERVE_ID_INFORMATION {
    pub StorageReserveId: super::STORAGE_RESERVE_ID,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct FILE_STREAM_INFORMATION {
    pub NextEntryOffset: u32,
    pub StreamNameLength: u32,
    pub StreamSize: super::LARGE_INTEGER,
    pub StreamAllocationSize: super::LARGE_INTEGER,
    pub StreamName: [u16; 1],
}
#[cfg(feature = "winnt")]
impl Default for FILE_STREAM_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILE_STREAM_RESERVATION_INFORMATION {
    pub TrackedReservation: i64,
    pub EnforcedReservation: i64,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct FILE_TIMESTAMPS {
    pub CreationTime: super::LARGE_INTEGER,
    pub LastAccessTime: super::LARGE_INTEGER,
    pub LastWriteTime: super::LARGE_INTEGER,
    pub ChangeTime: super::LARGE_INTEGER,
}
#[cfg(feature = "winnt")]
impl Default for FILE_TIMESTAMPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_TRACKING_INFORMATION {
    pub DestinationFile: super::HANDLE,
    pub ObjectInformationLength: u32,
    pub ObjectInformation: [i8; 1],
}
#[cfg(feature = "winnt")]
impl Default for FILE_TRACKING_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FILE_TYPE_NOTIFICATION_GUID_CRASHDUMP_FILE: windows_core::GUID = windows_core::GUID::from_u128(0x9d453eb7_d2a6_4dbd_a2e3_fbd0ed9109a9);
pub const FILE_TYPE_NOTIFICATION_GUID_HIBERNATION_FILE: windows_core::GUID = windows_core::GUID::from_u128(0xb7624d64_b9a3_4cf8_8011_5b86c940e7b7);
pub const FILE_TYPE_NOTIFICATION_GUID_PAGE_FILE: windows_core::GUID = windows_core::GUID::from_u128(0x0d0a64a1_38fc_4db8_9fe7_3f4352cd7c5c);
pub const FILE_VC_CONTENT_INDEX_DISABLED: i32 = 8;
pub const FILE_VC_LOG_QUOTA_LIMIT: i32 = 32;
pub const FILE_VC_LOG_QUOTA_THRESHOLD: i32 = 16;
pub const FILE_VC_LOG_VOLUME_LIMIT: i32 = 128;
pub const FILE_VC_LOG_VOLUME_THRESHOLD: i32 = 64;
pub const FILE_VC_QUOTAS_INCOMPLETE: i32 = 256;
pub const FILE_VC_QUOTAS_REBUILDING: i32 = 512;
pub const FILE_VC_QUOTA_ENFORCE: i32 = 2;
pub const FILE_VC_QUOTA_MASK: i32 = 3;
pub const FILE_VC_QUOTA_NONE: i32 = 0;
pub const FILE_VC_QUOTA_TRACK: i32 = 1;
pub const FILE_VC_VALID_MASK: i32 = 1023;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_VOLUME_NAME_INFORMATION {
    pub DeviceNameLength: u32,
    pub DeviceName: [u16; 1],
}
impl Default for FILE_VOLUME_NAME_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FLAGS_DELAY_REASONS_BITMAP_SCANNED: i32 = 2;
pub const FLAGS_DELAY_REASONS_LOG_FILE_FULL: i32 = 1;
pub const FLAGS_END_OF_FILE_INFO_EX_EXTEND_PAGING: i32 = 1;
pub const FLAGS_END_OF_FILE_INFO_EX_NO_EXTRA_PAGING_EXTEND: i32 = 2;
pub const FLAGS_END_OF_FILE_INFO_EX_TIME_CONSTRAINED: i32 = 4;
#[cfg(all(feature = "basetsd", feature = "winnt"))]
pub type FREE_VIRTUAL_MEMORY_EX_CALLBACK = Option<unsafe extern "C" fn(callbackcontext: super::HANDLE, processhandle: super::HANDLE, baseaddress: *mut *mut core::ffi::c_void, regionsize: super::PSIZE_T, freetype: u32) -> windows_core::NTSTATUS>;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct FSCTL_GHOST_FILE_EXTENTS_INPUT_BUFFER {
    pub FileOffset: super::LARGE_INTEGER,
    pub ByteCount: super::LARGE_INTEGER,
    pub RecallOwnerGuid: windows_core::GUID,
    pub RecallMetadataBufferSize: u32,
    pub RecallMetadataBuffer: [u8; 1],
}
#[cfg(feature = "winnt")]
impl Default for FSCTL_GHOST_FILE_EXTENTS_INPUT_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FSCTL_LMR_GET_LINK_TRACKING_INFORMATION: i32 = 1310952;
pub const FSCTL_LMR_SET_LINK_TRACKING_INFORMATION: i32 = 1310956;
pub const FSCTL_MAILSLOT_PEEK: i32 = 802819;
pub const FSCTL_PIPE_ASSIGN_EVENT: i32 = 1114112;
pub const FSCTL_PIPE_CREATE_SYMLINK: i32 = 1114188;
pub const FSCTL_PIPE_DELETE_SYMLINK: i32 = 1114192;
pub const FSCTL_PIPE_DISABLE_IMPERSONATE: i32 = 1114180;
pub const FSCTL_PIPE_DISCONNECT: i32 = 1114116;
pub const FSCTL_PIPE_FLUSH: i32 = 1146944;
pub const FSCTL_PIPE_GET_CONNECTION_ATTRIBUTE: i32 = 1114160;
pub const FSCTL_PIPE_GET_HANDLE_ATTRIBUTE: i32 = 1114168;
pub const FSCTL_PIPE_GET_PIPE_ATTRIBUTE: i32 = 1114152;
pub const FSCTL_PIPE_IMPERSONATE: i32 = 1114140;
pub const FSCTL_PIPE_INTERNAL_READ: i32 = 1138676;
pub const FSCTL_PIPE_INTERNAL_READ_OVFLOW: i32 = 1138688;
pub const FSCTL_PIPE_INTERNAL_TRANSCEIVE: i32 = 1171455;
pub const FSCTL_PIPE_INTERNAL_WRITE: i32 = 1155064;
pub const FSCTL_PIPE_LISTEN: i32 = 1114120;
pub const FSCTL_PIPE_PEEK: i32 = 1130508;
pub const FSCTL_PIPE_QUERY_CLIENT_PROCESS: i32 = 1114148;
pub const FSCTL_PIPE_QUERY_CLIENT_PROCESS_V2: i32 = 1114196;
pub const FSCTL_PIPE_QUERY_EVENT: i32 = 1114128;
pub const FSCTL_PIPE_SET_CLIENT_PROCESS: i32 = 1114144;
pub const FSCTL_PIPE_SET_CONNECTION_ATTRIBUTE: i32 = 1114164;
pub const FSCTL_PIPE_SET_HANDLE_ATTRIBUTE: i32 = 1114172;
pub const FSCTL_PIPE_SET_PIPE_ATTRIBUTE: i32 = 1114156;
pub const FSCTL_PIPE_SILO_ARRIVAL: i32 = 1146952;
pub const FSCTL_PIPE_TRANSCEIVE: i32 = 1163287;
pub const FSCTL_PIPE_WAIT: i32 = 1114136;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct FSCTL_QUERY_GHOSTED_FILE_EXTENTS_INPUT_RANGE {
    pub FileOffset: super::LARGE_INTEGER,
    pub ByteCount: super::LARGE_INTEGER,
}
#[cfg(feature = "winnt")]
impl Default for FSCTL_QUERY_GHOSTED_FILE_EXTENTS_INPUT_RANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FSCTL_QUERY_GHOSTED_FILE_EXTENTS_OUTPUT {
    pub ExtentCount: u32,
    pub TotalExtentCount: u32,
    pub Extents: [u8; 1],
}
impl Default for FSCTL_QUERY_GHOSTED_FILE_EXTENTS_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FSCTL_QUERY_VOLUME_NUMA_INFO_OUTPUT {
    pub NumaNode: u32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct FSCTL_UNMAP_SPACE_INPUT_BUFFER {
    pub BytesToUnmap: super::LARGE_INTEGER,
}
#[cfg(feature = "winnt")]
impl Default for FSCTL_UNMAP_SPACE_INPUT_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct FSCTL_UNMAP_SPACE_OUTPUT {
    pub BytesUnmapped: super::LARGE_INTEGER,
}
#[cfg(feature = "winnt")]
impl Default for FSCTL_UNMAP_SPACE_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FSRTL_ADD_TC_CASE_SENSITIVE: i32 = 1;
pub const FSRTL_ADD_TC_KEY_BY_SHORT_NAME: i32 = 2;
#[repr(C)]
#[cfg(all(feature = "ntdef", feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct FSRTL_ADVANCED_FCB_HEADER {
    pub Base: FSRTL_COMMON_FCB_HEADER,
    pub FastMutex: super::PFAST_MUTEX,
    pub FilterContexts: super::LIST_ENTRY,
    pub PushLock: usize,
    pub FileContextSupportPointer: *mut *mut core::ffi::c_void,
    pub Anonymous: FSRTL_ADVANCED_FCB_HEADER_0,
    pub AePushLock: *mut core::ffi::c_void,
    pub BypassIoOpenCount: u32,
    pub ReservedContext: *mut FSRTL_PER_STREAM_CONTEXT,
}
#[cfg(all(feature = "ntdef", feature = "wdm", feature = "winnt"))]
impl Default for FSRTL_ADVANCED_FCB_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ntdef", feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union FSRTL_ADVANCED_FCB_HEADER_0 {
    pub Oplock: OPLOCK,
    pub ReservedForRemote: *mut core::ffi::c_void,
}
#[cfg(all(feature = "ntdef", feature = "wdm", feature = "winnt"))]
impl Default for FSRTL_ADVANCED_FCB_HEADER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type FSRTL_ALLOCATE_ECPLIST_FLAGS = u32;
pub const FSRTL_ALLOCATE_ECPLIST_FLAG_CHARGE_QUOTA: i32 = 1;
pub type FSRTL_ALLOCATE_ECP_FLAGS = u32;
pub const FSRTL_ALLOCATE_ECP_FLAG_CHARGE_QUOTA: i32 = 1;
pub const FSRTL_ALLOCATE_ECP_FLAG_NONPAGED_POOL: i32 = 2;
pub const FSRTL_ASYNC_CACHED_READ_TOP_LEVEL_IRP: isize = 7;
#[repr(C)]
#[cfg(feature = "usb")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FSRTL_AUXILIARY_BUFFER {
    pub Buffer: *mut core::ffi::c_void,
    pub Length: u32,
    pub Flags: u32,
    pub Mdl: super::PMDL,
}
pub const FSRTL_AUXILIARY_FLAG_DEALLOCATE: i32 = 1;
pub const FSRTL_CACHE_TOP_LEVEL_IRP: isize = 2;
pub const FSRTL_CC_FLUSH_ERROR_FLAG_NO_HARD_ERROR: i32 = 1;
pub const FSRTL_CC_FLUSH_ERROR_FLAG_NO_LOG_ENTRY: i32 = 2;
pub type FSRTL_CHANGE_BACKING_TYPE = i32;
#[repr(C)]
#[cfg(all(feature = "ntdef", feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct FSRTL_COMMON_FCB_HEADER {
    pub NodeTypeCode: super::CSHORT,
    pub NodeByteSize: super::CSHORT,
    pub Flags: u8,
    pub IsFastIoPossible: u8,
    pub Flags2: u8,
    pub _bitfield: u8,
    pub Resource: super::PERESOURCE,
    pub PagingIoResource: super::PERESOURCE,
    pub AllocationSize: super::LARGE_INTEGER,
    pub FileSize: super::LARGE_INTEGER,
    pub ValidDataLength: super::LARGE_INTEGER,
}
#[cfg(all(feature = "ntdef", feature = "wdm", feature = "winnt"))]
impl FSRTL_COMMON_FCB_HEADER {
    pub fn Reserved(&self) -> u8 {
        (self._bitfield << 4) >> 4
    }
    pub fn set_Reserved(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !15) | (value & 15);
    }
    pub fn Version(&self) -> u8 {
        self._bitfield >> 4
    }
    pub fn set_Version(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(15 << 4)) | ((value & 15) << 4);
    }
}
#[cfg(all(feature = "ntdef", feature = "wdm", feature = "winnt"))]
impl Default for FSRTL_COMMON_FCB_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type FSRTL_COMPARISON_RESULT = i32;
pub const FSRTL_DRIVER_BACKING_FLAG_USE_PAGE_FILE: i32 = 1;
pub type FSRTL_ECP_LOOKASIDE_FLAGS = u32;
pub const FSRTL_ECP_LOOKASIDE_FLAG_NONPAGED_POOL: i32 = 2;
pub const FSRTL_FAST_IO_TOP_LEVEL_IRP: isize = 4;
pub const FSRTL_FAT_LEGAL: i32 = 1;
pub const FSRTL_FCB_HEADER_V0: i32 = 0;
pub const FSRTL_FCB_HEADER_V1: i32 = 1;
pub const FSRTL_FCB_HEADER_V2: i32 = 2;
pub const FSRTL_FCB_HEADER_V3: i32 = 3;
pub const FSRTL_FCB_HEADER_V4: i32 = 4;
pub const FSRTL_FCB_HEADER_V5: i32 = 5;
pub const FSRTL_FIND_TC_CASE_SENSITIVE: i32 = 1;
pub const FSRTL_FLAG2_BYPASSIO_STREAM_PAUSED: i32 = 32;
pub const FSRTL_FLAG2_DO_MODIFIED_WRITE: i32 = 1;
pub const FSRTL_FLAG2_IS_PAGING_FILE: i32 = 8;
pub const FSRTL_FLAG2_PURGE_WHEN_MAPPED: i32 = 4;
pub const FSRTL_FLAG2_SUPPORTS_FILTER_CONTEXTS: i32 = 2;
pub const FSRTL_FLAG2_WRITABLE_USER_MAPPED_FILE: i32 = 16;
pub const FSRTL_FLAG_ACQUIRE_MAIN_RSRC_EX: i32 = 8;
pub const FSRTL_FLAG_ACQUIRE_MAIN_RSRC_SH: i32 = 16;
pub const FSRTL_FLAG_ADVANCED_HEADER: i32 = 64;
pub const FSRTL_FLAG_EOF_ADVANCE_ACTIVE: i32 = 128;
pub const FSRTL_FLAG_FILE_LENGTH_CHANGED: i32 = 2;
pub const FSRTL_FLAG_FILE_MODIFIED: i32 = 1;
pub const FSRTL_FLAG_LIMIT_MODIFIED_PAGES: i32 = 4;
pub const FSRTL_FLAG_USER_MAPPED_FILE: i32 = 32;
pub const FSRTL_FSP_TOP_LEVEL_IRP: isize = 1;
pub const FSRTL_HPFS_LEGAL: i32 = 2;
pub const FSRTL_MAX_TOP_LEVEL_IRP_FLAG: isize = 65535;
pub const FSRTL_MOD_WRITE_TOP_LEVEL_IRP: isize = 3;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FSRTL_MUP_PROVIDER_INFO_LEVEL_1 {
    pub ProviderId: u32,
}
#[repr(C)]
#[cfg(feature = "winternl")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FSRTL_MUP_PROVIDER_INFO_LEVEL_2 {
    pub ProviderId: u32,
    pub ProviderName: super::UNICODE_STRING,
}
pub const FSRTL_NETWORK1_TOP_LEVEL_IRP: isize = 5;
pub const FSRTL_NETWORK2_TOP_LEVEL_IRP: isize = 6;
pub const FSRTL_NTFS_LEGAL: i32 = 4;
pub const FSRTL_NTFS_STREAM_LEGAL: i32 = 20;
pub const FSRTL_OLE_LEGAL: i32 = 16;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FSRTL_PER_FILEOBJECT_CONTEXT {
    pub Links: super::LIST_ENTRY,
    pub OwnerId: *mut core::ffi::c_void,
    pub InstanceId: *mut core::ffi::c_void,
}
#[repr(C)]
#[cfg(all(feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FSRTL_PER_FILE_CONTEXT {
    pub Links: super::LIST_ENTRY,
    pub OwnerId: *mut core::ffi::c_void,
    pub InstanceId: *mut core::ffi::c_void,
    pub FreeCallback: super::PFREE_FUNCTION,
}
#[repr(C)]
#[cfg(all(feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FSRTL_PER_STREAM_CONTEXT {
    pub Links: super::LIST_ENTRY,
    pub OwnerId: *mut core::ffi::c_void,
    pub InstanceId: *mut core::ffi::c_void,
    pub FreeCallback: super::PFREE_FUNCTION,
}
pub type FSRTL_UNC_HARDENING_CAPABILITIES = u32;
pub const FSRTL_UNC_HARDENING_CAPABILITIES_INTEGRITY: i32 = 2;
pub const FSRTL_UNC_HARDENING_CAPABILITIES_MUTUAL_AUTH: i32 = 1;
pub const FSRTL_UNC_HARDENING_CAPABILITIES_PRIVACY: i32 = 4;
pub type FSRTL_UNC_PROVIDER_FLAGS = u32;
pub const FSRTL_UNC_PROVIDER_FLAGS_CONTAINER_AWARE: i32 = 8;
pub const FSRTL_UNC_PROVIDER_FLAGS_CSC_ENABLED: i32 = 2;
pub const FSRTL_UNC_PROVIDER_FLAGS_DOMAIN_SVC_AWARE: i32 = 4;
pub const FSRTL_UNC_PROVIDER_FLAGS_MAILSLOTS_SUPPORTED: i32 = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FSRTL_UNC_PROVIDER_REGISTRATION {
    pub Size: u16,
    pub Version: u16,
    pub Anonymous: FSRTL_UNC_PROVIDER_REGISTRATION_0,
    pub Anonymous2: FSRTL_UNC_PROVIDER_REGISTRATION_1,
}
impl Default for FSRTL_UNC_PROVIDER_REGISTRATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union FSRTL_UNC_PROVIDER_REGISTRATION_0 {
    pub ProviderFlags: FSRTL_UNC_PROVIDER_FLAGS,
    pub Anonymous: FSRTL_UNC_PROVIDER_REGISTRATION_0_0,
}
impl Default for FSRTL_UNC_PROVIDER_REGISTRATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FSRTL_UNC_PROVIDER_REGISTRATION_0_0 {
    pub _bitfield: u32,
}
impl FSRTL_UNC_PROVIDER_REGISTRATION_0_0 {
    pub fn MailslotsSupported(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_MailslotsSupported(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u32);
    }
    pub fn CscEnabled(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_CscEnabled(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u32) << 1);
    }
    pub fn DomainSvcAware(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_DomainSvcAware(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u32) << 2);
    }
    pub fn ContainersAware(&self) -> bool {
        (self._bitfield >> 3) & 1 != 0
    }
    pub fn set_ContainersAware(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value as u32) << 3);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union FSRTL_UNC_PROVIDER_REGISTRATION_1 {
    pub HardeningCapabilities: FSRTL_UNC_HARDENING_CAPABILITIES,
    pub Anonymous: FSRTL_UNC_PROVIDER_REGISTRATION_1_0,
}
impl Default for FSRTL_UNC_PROVIDER_REGISTRATION_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FSRTL_UNC_PROVIDER_REGISTRATION_1_0 {
    pub _bitfield: u32,
}
impl FSRTL_UNC_PROVIDER_REGISTRATION_1_0 {
    pub fn SupportsMutualAuth(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_SupportsMutualAuth(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u32);
    }
    pub fn SupportsIntegrity(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_SupportsIntegrity(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u32) << 1);
    }
    pub fn SupportsPrivacy(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_SupportsPrivacy(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u32) << 2);
    }
}
pub const FSRTL_UNC_REGISTRATION_CURRENT_VERSION: i32 = 513;
pub const FSRTL_UNC_REGISTRATION_VERSION_0200: i32 = 512;
pub const FSRTL_UNC_REGISTRATION_VERSION_0201: i32 = 513;
pub const FSRTL_VIRTDISK_FULLY_ALLOCATED: i32 = 1;
pub const FSRTL_VIRTDISK_NO_DRIVE_LETTER: i32 = 2;
pub const FSRTL_VOLSNAP_TOP_LEVEL_IRP: isize = 8;
pub const FSRTL_VOLUME_BACKGROUND_FORMAT: i32 = 14;
pub const FSRTL_VOLUME_CHANGE_SIZE: i32 = 13;
pub const FSRTL_VOLUME_DISMOUNT: i32 = 1;
pub const FSRTL_VOLUME_DISMOUNT_FAILED: i32 = 2;
pub const FSRTL_VOLUME_FORCED_CLOSED: i32 = 10;
pub const FSRTL_VOLUME_INFO_MAKE_COMPAT: i32 = 11;
pub const FSRTL_VOLUME_LOCK: i32 = 3;
pub const FSRTL_VOLUME_LOCK_FAILED: i32 = 4;
pub const FSRTL_VOLUME_MOUNT: i32 = 6;
pub const FSRTL_VOLUME_NEEDS_CHKDSK: i32 = 7;
pub const FSRTL_VOLUME_PREPARING_EJECT: i32 = 12;
pub const FSRTL_VOLUME_UNLOCK: i32 = 5;
pub const FSRTL_VOLUME_WEARING_OUT: i32 = 9;
pub const FSRTL_VOLUME_WORM_NEAR_FULL: i32 = 8;
pub const FSRTL_WILD_CHARACTER: i32 = 8;
pub const FS_FILTER_ACQUIRE_FOR_CC_FLUSH: u8 = 251;
pub const FS_FILTER_ACQUIRE_FOR_MOD_WRITE: u8 = 253;
pub const FS_FILTER_ACQUIRE_FOR_SECTION_SYNCHRONIZATION: u8 = 255;
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "lsalookup", feature = "minwindef", feature = "ntdef", feature = "ntsecapi", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct FS_FILTER_CALLBACKS {
    pub SizeOfFsFilterCallbacks: u32,
    pub Reserved: u32,
    pub PreAcquireForSectionSynchronization: PFS_FILTER_CALLBACK,
    pub PostAcquireForSectionSynchronization: PFS_FILTER_COMPLETION_CALLBACK,
    pub PreReleaseForSectionSynchronization: PFS_FILTER_CALLBACK,
    pub PostReleaseForSectionSynchronization: PFS_FILTER_COMPLETION_CALLBACK,
    pub PreAcquireForCcFlush: PFS_FILTER_CALLBACK,
    pub PostAcquireForCcFlush: PFS_FILTER_COMPLETION_CALLBACK,
    pub PreReleaseForCcFlush: PFS_FILTER_CALLBACK,
    pub PostReleaseForCcFlush: PFS_FILTER_COMPLETION_CALLBACK,
    pub PreAcquireForModifiedPageWriter: PFS_FILTER_CALLBACK,
    pub PostAcquireForModifiedPageWriter: PFS_FILTER_COMPLETION_CALLBACK,
    pub PreReleaseForModifiedPageWriter: PFS_FILTER_CALLBACK,
    pub PostReleaseForModifiedPageWriter: PFS_FILTER_COMPLETION_CALLBACK,
    pub PreQueryOpen: PFS_FILTER_CALLBACK,
    pub PostQueryOpen: PFS_FILTER_COMPLETION_CALLBACK,
}
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "lsalookup", feature = "minwindef", feature = "ntdef", feature = "ntsecapi", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy)]
pub struct FS_FILTER_CALLBACK_DATA {
    pub SizeOfFsFilterCallbackData: u32,
    pub Operation: u8,
    pub Reserved: u8,
    pub DeviceObject: *mut super::DEVICE_OBJECT,
    pub FileObject: *mut super::FILE_OBJECT,
    pub Parameters: FS_FILTER_PARAMETERS,
}
#[cfg(all(feature = "basetsd", feature = "lsalookup", feature = "minwindef", feature = "ntdef", feature = "ntsecapi", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
impl Default for FS_FILTER_CALLBACK_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "usb", feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union FS_FILTER_PARAMETERS {
    pub AcquireForModifiedPageWriter: FS_FILTER_PARAMETERS_0,
    pub ReleaseForModifiedPageWriter: FS_FILTER_PARAMETERS_1,
    pub AcquireForSectionSynchronization: FS_FILTER_PARAMETERS_2,
    pub QueryOpen: FS_FILTER_PARAMETERS_3,
    pub Others: FS_FILTER_PARAMETERS_4,
}
#[cfg(all(feature = "minwindef", feature = "usb", feature = "wdm", feature = "winnt"))]
impl Default for FS_FILTER_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "usb", feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FS_FILTER_PARAMETERS_0 {
    pub EndingOffset: super::PLARGE_INTEGER,
    pub ResourceToRelease: *mut super::PERESOURCE,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "usb", feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FS_FILTER_PARAMETERS_1 {
    pub ResourceToRelease: super::PERESOURCE,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "usb", feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FS_FILTER_PARAMETERS_2 {
    pub SyncType: FS_FILTER_SECTION_SYNC_TYPE,
    pub PageProtection: u32,
    pub OutputInformation: PFS_FILTER_SECTION_SYNC_OUTPUT,
    pub Flags: u32,
    pub AllocationAttributes: u32,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "usb", feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FS_FILTER_PARAMETERS_3 {
    pub Irp: super::PIRP,
    pub FileInformation: *mut core::ffi::c_void,
    pub Length: super::PULONG,
    pub FileInformationClass: super::FILE_INFORMATION_CLASS,
    pub CompletionStatus: windows_core::NTSTATUS,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "usb", feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FS_FILTER_PARAMETERS_4 {
    pub Argument1: *mut core::ffi::c_void,
    pub Argument2: *mut core::ffi::c_void,
    pub Argument3: *mut core::ffi::c_void,
    pub Argument4: *mut core::ffi::c_void,
    pub Argument5: *mut core::ffi::c_void,
}
pub const FS_FILTER_QUERY_OPEN: u8 = 249;
pub const FS_FILTER_RELEASE_FOR_CC_FLUSH: u8 = 250;
pub const FS_FILTER_RELEASE_FOR_MOD_WRITE: u8 = 252;
pub const FS_FILTER_RELEASE_FOR_SECTION_SYNCHRONIZATION: u8 = 254;
pub const FS_FILTER_SECTION_SYNC_IMAGE_EXTENTS_ARE_NOT_RVA: i32 = 8;
pub const FS_FILTER_SECTION_SYNC_IN_FLAG_DONT_UPDATE_LAST_ACCESS: i32 = 1;
pub const FS_FILTER_SECTION_SYNC_IN_FLAG_DONT_UPDATE_LAST_WRITE: i32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FS_FILTER_SECTION_SYNC_OUTPUT {
    pub StructureSize: u32,
    pub SizeReturned: u32,
    pub Flags: u32,
    pub DesiredReadAlignment: u32,
}
pub const FS_FILTER_SECTION_SYNC_SUPPORTS_ASYNC_PARALLEL_IO: i32 = 1;
pub const FS_FILTER_SECTION_SYNC_SUPPORTS_DIRECT_MAP_DATA: i32 = 2;
pub const FS_FILTER_SECTION_SYNC_SUPPORTS_DIRECT_MAP_IMAGE: i32 = 4;
pub type FS_FILTER_SECTION_SYNC_TYPE = i32;
pub type FS_FILTER_STREAM_FO_NOTIFICATION_TYPE = i32;
pub const FastIoIsNotPossible: FAST_IO_POSSIBLE = 0;
pub const FastIoIsPossible: FAST_IO_POSSIBLE = 1;
pub const FastIoIsQuestionable: FAST_IO_POSSIBLE = 2;
pub const GCR_ALLOW_LM: i32 = 4096;
pub const GCR_ALLOW_NO_TARGET: i32 = 8192;
pub const GCR_ALLOW_NTLM: i32 = 256;
pub const GCR_MACHINE_CREDENTIAL: i32 = 1024;
pub const GCR_NTLM3_PARMS: i32 = 32;
pub const GCR_TARGET_INFO: i32 = 64;
pub const GCR_USE_OEM_SET: i32 = 512;
pub const GCR_USE_OWF_PASSWORD: i32 = 2048;
pub const GCR_VSM_PROTECTED_PASSWORD: i32 = 16384;
pub const GENERATE_CLIENT_CHALLENGE: i32 = 16;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GENERATE_NAME_CONTEXT {
    pub Checksum: u16,
    pub ChecksumInserted: super::BOOLEAN,
    pub NameLength: u8,
    pub NameBuffer: [u16; 8],
    pub ExtensionLength: u32,
    pub ExtensionBuffer: [u16; 4],
    pub LastIndexValue: u32,
}
#[cfg(feature = "winnt")]
impl Default for GENERATE_NAME_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct GHOSTED_FILE_EXTENT {
    pub FileOffset: super::LARGE_INTEGER,
    pub ByteCount: super::LARGE_INTEGER,
    pub RecallOwnerGuid: windows_core::GUID,
    pub NextEntryOffset: u32,
    pub RecallMetadataBufferSize: u32,
    pub RecallMetadataBuffer: [u8; 1],
}
#[cfg(feature = "winnt")]
impl Default for GHOSTED_FILE_EXTENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const GUID_ECP_ATOMIC_CREATE: windows_core::GUID = windows_core::GUID::from_u128(0x4720bd83_52ac_4104_a130_d1ec6a8cc8e5);
pub const GUID_ECP_CLOUDFILES_ATTRIBUTION: windows_core::GUID = windows_core::GUID::from_u128(0x2932ff52_8378_4fc1_8edb_6bdc8f602709);
pub const GUID_ECP_CREATE_REDIRECTION: windows_core::GUID = windows_core::GUID::from_u128(0x188d6bd6_a126_4fa8_bdf2_1ccdf896f3e0);
pub const GUID_ECP_CSV_DOWN_LEVEL_OPEN: windows_core::GUID = windows_core::GUID::from_u128(0x4248be44_647f_488f_8be5_a08aaf70f028);
pub const GUID_ECP_CSV_QUERY_FILE_REVISION: windows_core::GUID = windows_core::GUID::from_u128(0x44aec90b_de65_4d46_8fbf_763f9d970b1d);
pub const GUID_ECP_CSV_QUERY_FILE_REVISION_FILE_ID_128: windows_core::GUID = windows_core::GUID::from_u128(0x7a3a4aa1_aa74_4bc6_b070_ab56a38c1fed);
pub const GUID_ECP_CSV_SET_HANDLE_PROPERTIES: windows_core::GUID = windows_core::GUID::from_u128(0x7a9fdd94_7b58_42bb_9740_3cb86983a615);
pub const GUID_ECP_DUAL_OPLOCK_KEY: windows_core::GUID = windows_core::GUID::from_u128(0x41621a14_b08b_4df1_b676_a05ffdf01bea);
pub const GUID_ECP_IO_DEVICE_HINT: windows_core::GUID = windows_core::GUID::from_u128(0xf315b732_ac6b_4d4d_be0c_b3126490e1a3);
pub const GUID_ECP_NETWORK_APP_INSTANCE: windows_core::GUID = windows_core::GUID::from_u128(0x6aa6bc45_a7ef_4af7_9008_fa462e144d74);
pub const GUID_ECP_NETWORK_APP_INSTANCE_VERSION: windows_core::GUID = windows_core::GUID::from_u128(0xb7d082b9_563b_4f07_a07b_524a8116a010);
pub const GUID_ECP_NETWORK_OPEN_CONTEXT: windows_core::GUID = windows_core::GUID::from_u128(0xc584edbf_00df_4d28_b884_35baca8911e8);
pub const GUID_ECP_NFS_OPEN: windows_core::GUID = windows_core::GUID::from_u128(0xf326d30c_e5f8_4fe7_ab74_f5a3196d92db);
pub const GUID_ECP_OPEN_PARAMETERS: windows_core::GUID = windows_core::GUID::from_u128(0xcd0a93c3_3bb7_463d_accb_969d3435a5a5);
pub const GUID_ECP_OPLOCK_KEY: windows_core::GUID = windows_core::GUID::from_u128(0x48850596_3050_4be7_9863_fec350ce8d7f);
pub const GUID_ECP_PREFETCH_OPEN: windows_core::GUID = windows_core::GUID::from_u128(0xe1777b21_847e_4837_aa45_64161d280655);
pub const GUID_ECP_QUERY_ON_CREATE: windows_core::GUID = windows_core::GUID::from_u128(0x1aca62e9_abb4_4ff2_bb5c_1c79025e417f);
pub const GUID_ECP_RKF_BYPASS: windows_core::GUID = windows_core::GUID::from_u128(0x02378cc6_f73c_489c_8282_564d1a99131b);
pub const GUID_ECP_SRV_OPEN: windows_core::GUID = windows_core::GUID::from_u128(0xbebfaebc_aabf_489d_9d2c_e9e361102853);
pub const GUID_ECP_TYPE_VETO_BINDING: windows_core::GUID = windows_core::GUID::from_u128(0x41778682_63fc_4956_86c5_2a4b79d251af);
pub const GreaterThan: FSRTL_COMPARISON_RESULT = 1;
pub const HEAP_CLASS_0: i32 = 0;
pub const HEAP_CLASS_1: i32 = 4096;
pub const HEAP_CLASS_2: i32 = 8192;
pub const HEAP_CLASS_3: i32 = 12288;
pub const HEAP_CLASS_4: i32 = 16384;
pub const HEAP_CLASS_5: i32 = 20480;
pub const HEAP_CLASS_6: i32 = 24576;
pub const HEAP_CLASS_7: i32 = 28672;
pub const HEAP_CLASS_8: i32 = 32768;
pub const HEAP_CLASS_MASK: i32 = 61440;
pub const HEAP_CREATE_VALID_MASK: i32 = 521215;
pub const HEAP_GLOBAL_TAG: i32 = 2048;
pub type HEAP_MEMORY_INFO_CLASS = i32;
pub const HEAP_SETTABLE_USER_FLAG1: i32 = 512;
pub const HEAP_SETTABLE_USER_FLAG2: i32 = 1024;
pub const HEAP_SETTABLE_USER_FLAG3: i32 = 2048;
pub const HEAP_SETTABLE_USER_FLAGS: i32 = 3584;
pub const HEAP_SETTABLE_USER_VALUE: i32 = 256;
pub const HEAP_TAG_MASK: i32 = 1073479680;
pub const HeapMemoryBasicInformation: HEAP_MEMORY_INFO_CLASS = 0;
pub const IOCTL_LMR_ARE_FILE_OBJECTS_ON_SAME_SERVER: i32 = 1310960;
pub const IOCTL_REDIR_QUERY_PATH: i32 = 1311119;
pub const IOCTL_REDIR_QUERY_PATH_EX: i32 = 1311123;
pub const IOCTL_VOLSNAP_FLUSH_AND_HOLD_WRITES: i32 = 5488640;
pub const IO_CREATE_STREAM_FILE_LITE: i32 = 2;
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "lsalookup", feature = "ntdef", feature = "ntsecapi", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IO_CREATE_STREAM_FILE_OPTIONS {
    pub Size: u16,
    pub Flags: u16,
    pub TargetDeviceObject: super::PDEVICE_OBJECT,
}
pub const IO_CREATE_STREAM_FILE_RAISE_ON_ERROR: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "lsalookup", feature = "ntdef", feature = "ntsecapi", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IO_DEVICE_HINT_ECP_CONTEXT {
    pub TargetDevice: super::PDEVICE_OBJECT,
    pub RemainingName: super::UNICODE_STRING,
}
pub const IO_FILE_OBJECT_NON_PAGED_POOL_CHARGE: i32 = 64;
pub const IO_FILE_OBJECT_PAGED_POOL_CHARGE: i32 = 1024;
pub const IO_IGNORE_READONLY_ATTRIBUTE: i32 = 64;
pub const IO_MM_PAGING_FILE: i32 = 16;
pub const IO_OPEN_PAGING_FILE: i32 = 2;
pub const IO_OPEN_TARGET_DIRECTORY: i32 = 4;
#[repr(C)]
#[cfg(feature = "wdm")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IO_PRIORITY_INFO {
    pub Size: u32,
    pub ThreadPriority: u32,
    pub PagePriority: u32,
    pub IoPriority: super::IO_PRIORITY_HINT,
}
pub const IO_REPARSE_TAG_ACRONIS_HSM_0: i32 = 96;
pub const IO_REPARSE_TAG_ACRONIS_HSM_1: i32 = 97;
pub const IO_REPARSE_TAG_ACRONIS_HSM_2: i32 = 98;
pub const IO_REPARSE_TAG_ACRONIS_HSM_3: i32 = 99;
pub const IO_REPARSE_TAG_ACRONIS_HSM_4: i32 = 100;
pub const IO_REPARSE_TAG_ACRONIS_HSM_5: i32 = 101;
pub const IO_REPARSE_TAG_ACRONIS_HSM_6: i32 = 102;
pub const IO_REPARSE_TAG_ACRONIS_HSM_7: i32 = 103;
pub const IO_REPARSE_TAG_ACRONIS_HSM_8: i32 = 104;
pub const IO_REPARSE_TAG_ACRONIS_HSM_9: i32 = 105;
pub const IO_REPARSE_TAG_ACRONIS_HSM_A: i32 = 106;
pub const IO_REPARSE_TAG_ACRONIS_HSM_B: i32 = 107;
pub const IO_REPARSE_TAG_ACRONIS_HSM_C: i32 = 108;
pub const IO_REPARSE_TAG_ACRONIS_HSM_D: i32 = 109;
pub const IO_REPARSE_TAG_ACRONIS_HSM_E: i32 = 110;
pub const IO_REPARSE_TAG_ACRONIS_HSM_F: i32 = 111;
pub const IO_REPARSE_TAG_ACTIVISION_HSM: i32 = 71;
pub const IO_REPARSE_TAG_ADA_HSM: i32 = 38;
pub const IO_REPARSE_TAG_ADOBE_HSM: i32 = 69;
pub const IO_REPARSE_TAG_ALERTBOOT: i32 = 536870988;
pub const IO_REPARSE_TAG_ALTIRIS_HSM: i32 = 25;
pub const IO_REPARSE_TAG_AMZN_APPSTREAM: i32 = 89;
pub const IO_REPARSE_TAG_APPXSTRM: u32 = 3221225492;
pub const IO_REPARSE_TAG_ARCO_BACKUP: i32 = 59;
pub const IO_REPARSE_TAG_ARKIVIO: i32 = 12;
pub const IO_REPARSE_TAG_AURISTOR_FS: i32 = 73;
pub const IO_REPARSE_TAG_AUTN_HSM: i32 = 39;
pub const IO_REPARSE_TAG_BRIDGEHEAD_HSM: i32 = 22;
pub const IO_REPARSE_TAG_C2CSYSTEMS_HSM: i32 = 49;
pub const IO_REPARSE_TAG_CARINGO_HSM: i32 = 52;
pub const IO_REPARSE_TAG_CARROLL_HSM: i32 = 60;
pub const IO_REPARSE_TAG_CITRIX_PM: i32 = 54;
pub const IO_REPARSE_TAG_COMMVAULT: i32 = 14;
pub const IO_REPARSE_TAG_COMMVAULT_HSM: i32 = 29;
pub const IO_REPARSE_TAG_COMTRADE_HSM: i32 = 61;
pub const IO_REPARSE_TAG_CTERA_HSM: i32 = 78;
pub const IO_REPARSE_TAG_DATAFIRST_HSM: i32 = 48;
pub const IO_REPARSE_TAG_DATAGLOBAL_HSM: i32 = 46;
pub const IO_REPARSE_TAG_DATASTOR_SIS: i32 = 30;
pub const IO_REPARSE_TAG_DFM: u32 = 2147483670;
pub const IO_REPARSE_TAG_DOR_HSM: i32 = 82;
pub const IO_REPARSE_TAG_DOUBLE_TAKE_HSM: i32 = 34;
pub const IO_REPARSE_TAG_DOUBLE_TAKE_SIS: i32 = 41;
pub const IO_REPARSE_TAG_DRIVE_EXTENDER: u32 = 2147483653;
pub const IO_REPARSE_TAG_DROPBOX_HSM: i32 = 68;
pub const IO_REPARSE_TAG_EASEFILTER_HSM: i32 = 87;
pub const IO_REPARSE_TAG_EASEVAULT_HSM: i32 = 62;
pub const IO_REPARSE_TAG_EDSI_HSM: i32 = 31;
pub const IO_REPARSE_TAG_ELTAN_HSM: i32 = 43;
pub const IO_REPARSE_TAG_EMC_HSM: i32 = 57;
pub const IO_REPARSE_TAG_ENIGMA_HSM: i32 = 17;
pub const IO_REPARSE_TAG_FILTER_MANAGER: u32 = 2147483659;
pub const IO_REPARSE_TAG_GLOBAL360_HSM: i32 = 24;
pub const IO_REPARSE_TAG_GOOGLE_HSM: i32 = 65;
pub const IO_REPARSE_TAG_GRAU_DATASTORAGE_HSM: i32 = 28;
pub const IO_REPARSE_TAG_HDS_HCP_HSM: i32 = 72;
pub const IO_REPARSE_TAG_HDS_HSM: i32 = 63;
pub const IO_REPARSE_TAG_HERMES_HSM: i32 = 26;
pub const IO_REPARSE_TAG_HP_BACKUP: i32 = 67;
pub const IO_REPARSE_TAG_HP_DATA_PROTECT: i32 = 70;
pub const IO_REPARSE_TAG_HP_HSM: i32 = 32;
pub const IO_REPARSE_TAG_HSAG_HSM: i32 = 37;
pub const IO_REPARSE_TAG_HUBSTOR_HSM: i32 = 85;
pub const IO_REPARSE_TAG_IFSTEST_CONGRUENT: i32 = 9;
pub const IO_REPARSE_TAG_IIS_CACHE: u32 = 2684354576;
pub const IO_REPARSE_TAG_IMANAGE_HSM: i32 = 536870998;
pub const IO_REPARSE_TAG_INTERCOPE_HSM: i32 = 19;
pub const IO_REPARSE_TAG_ITSTATION: i32 = 74;
pub const IO_REPARSE_TAG_KOM_NETWORKS_HSM: i32 = 20;
pub const IO_REPARSE_TAG_LX_BLK: u32 = 2147483686;
pub const IO_REPARSE_TAG_LX_CHR: u32 = 2147483685;
pub const IO_REPARSE_TAG_LX_FIFO: u32 = 2147483684;
pub const IO_REPARSE_TAG_LX_SYMLINK: u32 = 2684354589;
pub const IO_REPARSE_TAG_MAGINATICS_RDR: i32 = 64;
pub const IO_REPARSE_TAG_MAXISCALE_HSM: i32 = 536870965;
pub const IO_REPARSE_TAG_MEMORY_TECH_HSM: i32 = 21;
pub const IO_REPARSE_TAG_MIMOSA_HSM: i32 = 36;
pub const IO_REPARSE_TAG_MOONWALK_HSM: i32 = 10;
pub const IO_REPARSE_TAG_MTALOS: i32 = 77;
pub const IO_REPARSE_TAG_NEUSHIELD: i32 = 81;
pub const IO_REPARSE_TAG_NEXSAN_HSM: i32 = 40;
pub const IO_REPARSE_TAG_NIPPON_HSM: i32 = 79;
pub const IO_REPARSE_TAG_NVIDIA_UNIONFS: i32 = 536870996;
pub const IO_REPARSE_TAG_OPENAFS_DFS: i32 = 55;
pub const IO_REPARSE_TAG_OSR_SAMPLE: i32 = 536870935;
pub const IO_REPARSE_TAG_OVERTONE: i32 = 15;
pub const IO_REPARSE_TAG_PEER_GFS: i32 = 88;
pub const IO_REPARSE_TAG_POINTSOFT_HSM: i32 = 27;
pub const IO_REPARSE_TAG_QI_TECH_HSM: i32 = 536870959;
pub const IO_REPARSE_TAG_QUADDRA_HSM: i32 = 66;
pub const IO_REPARSE_TAG_QUEST_HSM: i32 = 45;
pub const IO_REPARSE_TAG_REDSTOR_HSM: i32 = 80;
pub const IO_REPARSE_TAG_RIVERBED_HSM: i32 = 51;
pub const IO_REPARSE_TAG_SER_HSM: i32 = 33;
pub const IO_REPARSE_TAG_SHX_BACKUP: i32 = 83;
pub const IO_REPARSE_TAG_SOLUTIONSOFT: i32 = 536870925;
pub const IO_REPARSE_TAG_SONY_HSM: i32 = 42;
pub const IO_REPARSE_TAG_SPHARSOFT: i32 = 75;
pub const IO_REPARSE_TAG_SYMANTEC_HSM: i32 = 18;
pub const IO_REPARSE_TAG_SYMANTEC_HSM2: i32 = 16;
pub const IO_REPARSE_TAG_TSINGHUA_UNIVERSITY_RESEARCH: i32 = 11;
pub const IO_REPARSE_TAG_UTIXO_HSM: i32 = 44;
pub const IO_REPARSE_TAG_VALID_VALUES: u32 = 4026597375;
pub const IO_REPARSE_TAG_VMWARE_PM: i32 = 58;
pub const IO_REPARSE_TAG_WATERFORD: i32 = 50;
pub const IO_REPARSE_TAG_WISDATA_HSM: i32 = 35;
pub const IO_REPARSE_TAG_ZLTI_HSM: i32 = 56;
pub const IO_STOP_ON_SYMLINK: i32 = 8;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IO_STOP_ON_SYMLINK_FILTER_ECP_v0 {
    pub Out: IO_STOP_ON_SYMLINK_FILTER_ECP_v0_0,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IO_STOP_ON_SYMLINK_FILTER_ECP_v0_0 {
    pub ReparseCount: u32,
    pub RemainingPathLength: u32,
}
#[repr(C)]
#[cfg(all(feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct KAPC_STATE {
    pub ApcListHead: [super::LIST_ENTRY; 2],
    pub Process: *mut super::KPROCESS,
    pub Anonymous: KAPC_STATE_0,
    pub KernelApcPending: super::BOOLEAN,
    pub Anonymous2: KAPC_STATE_1,
}
#[cfg(all(feature = "wdm", feature = "winnt"))]
impl Default for KAPC_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union KAPC_STATE_0 {
    pub InProgressFlags: u8,
    pub Anonymous: KAPC_STATE_0_0,
}
#[cfg(all(feature = "wdm", feature = "winnt"))]
impl Default for KAPC_STATE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KAPC_STATE_0_0 {
    pub _bitfield: super::BOOLEAN,
}
#[repr(C)]
#[cfg(all(feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union KAPC_STATE_1 {
    pub UserApcPendingAll: super::BOOLEAN,
    pub Anonymous: KAPC_STATE_1_0,
}
#[cfg(all(feature = "wdm", feature = "winnt"))]
impl Default for KAPC_STATE_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KAPC_STATE_1_0 {
    pub _bitfield: super::BOOLEAN,
}
pub const KAPC_STATE_ANY_USER_APC_PENDING_MASK: u32 = 3;
pub const KAPC_STATE_NORMAL_USER_APC_PENDING_MASK: u32 = 1;
pub const KAPC_STATE_SPECIAL_USER_APC_PENDING_MASK: u32 = 2;
#[repr(C)]
#[cfg(all(feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct KQUEUE {
    pub Header: super::DISPATCHER_HEADER,
    pub EntryListHead: super::LIST_ENTRY,
    pub CurrentCount: u32,
    pub MaximumCount: u32,
    pub ThreadListHead: super::LIST_ENTRY,
}
#[cfg(all(feature = "wdm", feature = "winnt"))]
impl Default for KQUEUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KnownFolderDesktop: FILE_KNOWN_FOLDER_TYPE = 1;
pub const KnownFolderDocuments: FILE_KNOWN_FOLDER_TYPE = 2;
pub const KnownFolderDownloads: FILE_KNOWN_FOLDER_TYPE = 3;
pub const KnownFolderMax: FILE_KNOWN_FOLDER_TYPE = 7;
pub const KnownFolderMusic: FILE_KNOWN_FOLDER_TYPE = 4;
pub const KnownFolderNone: FILE_KNOWN_FOLDER_TYPE = 0;
pub const KnownFolderOther: FILE_KNOWN_FOLDER_TYPE = 7;
pub const KnownFolderPictures: FILE_KNOWN_FOLDER_TYPE = 5;
pub const KnownFolderVideos: FILE_KNOWN_FOLDER_TYPE = 6;
#[repr(C)]
#[cfg(all(feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LARGE_MCB {
    pub GuardedMutex: super::PKGUARDED_MUTEX,
    pub BaseMcb: BASE_MCB,
}
pub type LBN = u32;
pub const LCN_CHECKSUM_VALID: _LCN_WEAK_REFERENCE_STATE = 4;
pub const LCN_IS_READ_ONLY: _LCN_WEAK_REFERENCE_STATE = 32;
pub const LCN_IS_STREAM_RESERVED: _LCN_WEAK_REFERENCE_STATE = 16;
pub const LCN_IS_VALID: _LCN_WEAK_REFERENCE_STATE = 8;
pub const LCN_WEAK_REFERENCE_BROKEN: _LCN_WEAK_REFERENCE_STATE = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LCN_WEAK_REFERENCE_BUFFER {
    pub Lcn: i64,
    pub LengthInClusters: i64,
    pub ReferenceCount: u32,
    pub State: LCN_WEAK_REFERENCE_STATE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LCN_WEAK_REFERENCE_CLEAR_INPUT_BUFFER {
    pub RangeCount: u32,
    pub Ranges: [LCN_WEAK_REFERENCE_RANGE; 1],
}
impl Default for LCN_WEAK_REFERENCE_CLEAR_INPUT_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type LCN_WEAK_REFERENCE_CREATE_FLAGS = u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LCN_WEAK_REFERENCE_CREATE_INPUT_BUFFER {
    pub Offset: i64,
    pub Length: i64,
    pub Flags: u32,
    pub Reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LCN_WEAK_REFERENCE_CREATE_OUTPUT_BUFFER {
    pub MappingCount: u32,
    pub VcnLcnMappings: [LCN_WEAK_REFERENCE_VCN_MAPPING; 1],
}
impl Default for LCN_WEAK_REFERENCE_CREATE_OUTPUT_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LCN_WEAK_REFERENCE_RANGE {
    pub StartOfRange: i64,
    pub CountOfRange: i64,
}
pub type LCN_WEAK_REFERENCE_STATE = u32;
pub const LCN_WEAK_REFERENCE_VALID: _LCN_WEAK_REFERENCE_STATE = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LCN_WEAK_REFERENCE_VCN_MAPPING {
    pub Vcn: i64,
    pub Lcn: i64,
    pub CountOfRange: i64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LINK_TRACKING_INFORMATION {
    pub Type: LINK_TRACKING_INFORMATION_TYPE,
    pub VolumeId: [u8; 16],
}
impl Default for LINK_TRACKING_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type LINK_TRACKING_INFORMATION_TYPE = i32;
pub const LX_FILE_METADATA_DEVICE_ID_EA_NAME: windows_core::PCSTR = windows_core::s!("$LXDEV");
pub const LX_FILE_METADATA_GID_EA_NAME: windows_core::PCSTR = windows_core::s!("$LXGID");
pub const LX_FILE_METADATA_MODE_EA_NAME: windows_core::PCSTR = windows_core::s!("$LXMOD");
pub const LX_FILE_METADATA_UID_EA_NAME: windows_core::PCSTR = windows_core::s!("$LXUID");
pub const LessThan: FSRTL_COMPARISON_RESULT = -1;
pub const MAP_DISABLE_PAGEFAULT_CLUSTERING: i32 = 256;
pub const MAP_HIGH_PRIORITY: i32 = 64;
pub const MAP_NO_READ: i32 = 16;
pub const MAP_WAIT: i32 = 1;
pub const MAX_UNICODE_STACK_BUFFER_LENGTH: i32 = 256;
#[repr(C)]
#[cfg(all(feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MCB {
    pub DummyFieldThatSizesThisStructureCorrectly: LARGE_MCB,
}
pub const MCB_FLAG_RAISE_ON_ALLOCATION_FAILURE: i32 = 1;
pub type MEMORY_INFORMATION_CLASS = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MEMORY_RANGE_ENTRY {
    pub VirtualAddress: *mut core::ffi::c_void,
    pub NumberOfBytes: usize,
}
pub type MMFLUSH_TYPE = i32;
pub const MM_FORCE_CLOSED_DATA: i32 = 1;
pub const MM_FORCE_CLOSED_IMAGE: i32 = 2;
pub const MM_FORCE_CLOSED_LATER_OK: i32 = 4;
pub const MM_IS_FILE_SECTION_ACTIVE_DATA: i32 = 2;
pub const MM_IS_FILE_SECTION_ACTIVE_IMAGE: i32 = 1;
pub const MM_IS_FILE_SECTION_ACTIVE_USER: i32 = 4;
#[repr(C)]
#[derive(Clone, Copy)]
pub union MM_PREFETCH_FLAGS {
    pub Flags: MM_PREFETCH_FLAGS_0,
    pub AllFlags: u32,
}
impl Default for MM_PREFETCH_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MM_PREFETCH_FLAGS_0 {
    pub _bitfield: u32,
}
impl MM_PREFETCH_FLAGS_0 {
    pub fn Priority(&self) -> u32 {
        (self._bitfield << 29) >> 29
    }
    pub fn set_Priority(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !7) | (value & 7);
    }
    pub fn RepurposePriority(&self) -> u32 {
        (self._bitfield << 26) >> 29
    }
    pub fn set_RepurposePriority(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(7 << 3)) | ((value & 7) << 3);
    }
    pub fn PriorityProtection(&self) -> bool {
        (self._bitfield >> 6) & 1 != 0
    }
    pub fn set_PriorityProtection(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 6)) | ((value as u32) << 6);
    }
    pub fn MustBeZero(&self) -> u32 {
        (self._bitfield << 23) >> 30
    }
    pub fn set_MustBeZero(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(3 << 7)) | ((value & 3) << 7);
    }
    pub fn CannotBeUsedAsFlags(&self) -> u32 {
        self._bitfield >> 9
    }
    pub fn set_CannotBeUsedAsFlags(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(8388607 << 9)) | ((value & 8388607) << 9);
    }
}
pub const MM_PREFETCH_FLAGS_MASK: i32 = 127;
#[repr(C)]
#[cfg(feature = "ntsecapi")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MSV1_0_ENUMUSERS_REQUEST {
    pub MessageType: super::MSV1_0_PROTOCOL_MESSAGE_TYPE,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "ntsecapi", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MSV1_0_ENUMUSERS_RESPONSE {
    pub MessageType: super::MSV1_0_PROTOCOL_MESSAGE_TYPE,
    pub NumberOfLoggedOnUsers: u32,
    pub LogonIds: super::PLUID,
    pub EnumHandles: super::PULONG,
}
#[repr(C)]
#[cfg(all(feature = "ntsecapi", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MSV1_0_GETCHALLENRESP_REQUEST {
    pub MessageType: super::MSV1_0_PROTOCOL_MESSAGE_TYPE,
    pub ParameterControl: u32,
    pub LogonId: super::LUID,
    pub Password: super::UNICODE_STRING,
    pub ChallengeToClient: [u8; 8],
    pub UserName: super::UNICODE_STRING,
    pub LogonDomainName: super::UNICODE_STRING,
    pub ServerName: super::UNICODE_STRING,
}
#[cfg(all(feature = "ntsecapi", feature = "winnt", feature = "winternl"))]
impl Default for MSV1_0_GETCHALLENRESP_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ntsecapi", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MSV1_0_GETCHALLENRESP_REQUEST_V1 {
    pub MessageType: super::MSV1_0_PROTOCOL_MESSAGE_TYPE,
    pub ParameterControl: u32,
    pub LogonId: super::LUID,
    pub Password: super::UNICODE_STRING,
    pub ChallengeToClient: [u8; 8],
}
#[cfg(all(feature = "ntsecapi", feature = "winnt", feature = "winternl"))]
impl Default for MSV1_0_GETCHALLENRESP_REQUEST_V1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ntsecapi", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MSV1_0_GETCHALLENRESP_RESPONSE {
    pub MessageType: super::MSV1_0_PROTOCOL_MESSAGE_TYPE,
    pub CaseSensitiveChallengeResponse: super::STRING,
    pub CaseInsensitiveChallengeResponse: super::STRING,
    pub UserName: super::UNICODE_STRING,
    pub LogonDomainName: super::UNICODE_STRING,
    pub UserSessionKey: [u8; 16],
    pub LanmanSessionKey: [u8; 8],
}
#[cfg(all(feature = "ntsecapi", feature = "winnt", feature = "winternl"))]
impl Default for MSV1_0_GETCHALLENRESP_RESPONSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ntsecapi", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MSV1_0_GETUSERINFO_REQUEST {
    pub MessageType: super::MSV1_0_PROTOCOL_MESSAGE_TYPE,
    pub LogonId: super::LUID,
}
#[repr(C)]
#[cfg(all(feature = "ntsecapi", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MSV1_0_GETUSERINFO_RESPONSE {
    pub MessageType: super::MSV1_0_PROTOCOL_MESSAGE_TYPE,
    pub UserSid: super::PSID,
    pub UserName: super::UNICODE_STRING,
    pub LogonDomainName: super::UNICODE_STRING,
    pub LogonServer: super::UNICODE_STRING,
    pub LogonType: super::SECURITY_LOGON_TYPE,
}
#[repr(C)]
#[cfg(feature = "ntsecapi")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MSV1_0_LM20_CHALLENGE_REQUEST {
    pub MessageType: super::MSV1_0_PROTOCOL_MESSAGE_TYPE,
}
#[repr(C)]
#[cfg(feature = "ntsecapi")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MSV1_0_LM20_CHALLENGE_RESPONSE {
    pub MessageType: super::MSV1_0_PROTOCOL_MESSAGE_TYPE,
    pub ChallengeToClient: [u8; 8],
}
#[cfg(feature = "ntsecapi")]
impl Default for MSV1_0_LM20_CHALLENGE_RESPONSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MUP_PROVIDER_INFORMATION {
    pub Level: u32,
    pub Buffer: *mut core::ffi::c_void,
    pub BufferSize: super::PULONG,
}
pub const MemoryBasicInformation: MEMORY_INFORMATION_CLASS = 0;
pub const MemoryType64KPage: RTL_MEMORY_TYPE = 2;
pub const MemoryTypeCustom: RTL_MEMORY_TYPE = 5;
pub const MemoryTypeHugePage: RTL_MEMORY_TYPE = 4;
pub const MemoryTypeLargePage: RTL_MEMORY_TYPE = 3;
pub const MemoryTypeMax: RTL_MEMORY_TYPE = 6;
pub const MemoryTypeNonPaged: RTL_MEMORY_TYPE = 1;
pub const MemoryTypePaged: RTL_MEMORY_TYPE = 0;
pub const MmFlushForDelete: MMFLUSH_TYPE = 0;
pub const MmFlushForWrite: MMFLUSH_TYPE = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NETWORK_APP_INSTANCE_ECP_CONTEXT {
    pub Size: u16,
    pub Reserved: u16,
    pub AppInstanceID: windows_core::GUID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NETWORK_APP_INSTANCE_VERSION_ECP_CONTEXT {
    pub Size: u16,
    pub Reserved: u16,
    pub VersionHigh: u64,
    pub VersionLow: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NETWORK_OPEN_ECP_CONTEXT {
    pub Size: u16,
    pub Reserved: u16,
    pub Anonymous: NETWORK_OPEN_ECP_CONTEXT_0,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NETWORK_OPEN_ECP_CONTEXT_0 {
    pub r#in: NETWORK_OPEN_ECP_CONTEXT_0_0,
    pub out: NETWORK_OPEN_ECP_CONTEXT_0_1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NETWORK_OPEN_ECP_CONTEXT_0_0 {
    pub Location: NETWORK_OPEN_LOCATION_QUALIFIER,
    pub Integrity: NETWORK_OPEN_INTEGRITY_QUALIFIER,
    pub Flags: NETWORK_OPEN_IN_FLAGS,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NETWORK_OPEN_ECP_CONTEXT_0_1 {
    pub Location: NETWORK_OPEN_LOCATION_QUALIFIER,
    pub Integrity: NETWORK_OPEN_INTEGRITY_QUALIFIER,
    pub Flags: NETWORK_OPEN_OUT_FLAGS,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NETWORK_OPEN_ECP_CONTEXT_V0 {
    pub Size: u16,
    pub Reserved: u16,
    pub Anonymous: NETWORK_OPEN_ECP_CONTEXT_V0_0,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NETWORK_OPEN_ECP_CONTEXT_V0_0 {
    pub r#in: NETWORK_OPEN_ECP_CONTEXT_V0_0_0,
    pub out: NETWORK_OPEN_ECP_CONTEXT_V0_0_1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NETWORK_OPEN_ECP_CONTEXT_V0_0_0 {
    pub Location: NETWORK_OPEN_LOCATION_QUALIFIER,
    pub Integrity: NETWORK_OPEN_INTEGRITY_QUALIFIER,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NETWORK_OPEN_ECP_CONTEXT_V0_0_1 {
    pub Location: NETWORK_OPEN_LOCATION_QUALIFIER,
    pub Integrity: NETWORK_OPEN_INTEGRITY_QUALIFIER,
}
pub const NETWORK_OPEN_ECP_IN_FLAG_DISABLE_HANDLE_COLLAPSING: i32 = 1;
pub const NETWORK_OPEN_ECP_IN_FLAG_DISABLE_HANDLE_DURABILITY: i32 = 2;
pub const NETWORK_OPEN_ECP_IN_FLAG_DISABLE_OPLOCKS: i32 = 4;
pub const NETWORK_OPEN_ECP_IN_FLAG_FORCE_BUFFERED_SYNCHRONOUS_IO_HACK: u32 = 2147483648;
pub const NETWORK_OPEN_ECP_IN_FLAG_FORCE_MAX_EOF_HACK: i32 = 1073741824;
pub const NETWORK_OPEN_ECP_IN_FLAG_REQ_MUTUAL_AUTH: i32 = 8;
pub const NETWORK_OPEN_ECP_OUT_FLAG_RET_MUTUAL_AUTH: i32 = 8;
pub type NETWORK_OPEN_INTEGRITY_QUALIFIER = i32;
pub type NETWORK_OPEN_IN_FLAGS = u32;
pub type NETWORK_OPEN_LOCATION_QUALIFIER = i32;
pub type NETWORK_OPEN_OUT_FLAGS = u32;
#[repr(C)]
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NFS_OPEN_ECP_CONTEXT {
    pub ExportAlias: super::PUNICODE_STRING,
    pub ClientSocketAddress: PSOCKADDR_STORAGE_NFS,
}
pub const NO_8DOT3_NAME_PRESENT: i32 = 1;
pub const NetworkOpenIntegrityAny: NETWORK_OPEN_INTEGRITY_QUALIFIER = 0;
pub const NetworkOpenIntegrityEncrypted: NETWORK_OPEN_INTEGRITY_QUALIFIER = 3;
pub const NetworkOpenIntegrityMaximum: NETWORK_OPEN_INTEGRITY_QUALIFIER = 4;
pub const NetworkOpenIntegrityNone: NETWORK_OPEN_INTEGRITY_QUALIFIER = 1;
pub const NetworkOpenIntegritySigned: NETWORK_OPEN_INTEGRITY_QUALIFIER = 2;
pub const NetworkOpenLocationAny: NETWORK_OPEN_LOCATION_QUALIFIER = 0;
pub const NetworkOpenLocationLoopback: NETWORK_OPEN_LOCATION_QUALIFIER = 2;
pub const NetworkOpenLocationRemote: NETWORK_OPEN_LOCATION_QUALIFIER = 1;
pub const NotifyTypeCreate: FS_FILTER_STREAM_FO_NOTIFICATION_TYPE = 0;
pub const NotifyTypeRetired: FS_FILTER_STREAM_FO_NOTIFICATION_TYPE = 1;
pub const NtfsLinkTrackingInformation: LINK_TRACKING_INFORMATION_TYPE = 0;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OPEN_REPARSE_LIST {
    pub OpenReparseList: super::LIST_ENTRY,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OPEN_REPARSE_LIST_ENTRY {
    pub OpenReparseListEntry: super::LIST_ENTRY,
    pub ReparseTag: u32,
    pub Flags: u32,
    pub ReparseGuid: windows_core::GUID,
    pub Size: u16,
    pub RemainingLength: u16,
}
pub const OPEN_REPARSE_POINT_OVERRIDE_CREATE_OPTION: i32 = 64;
pub const OPEN_REPARSE_POINT_REPARSE_ALWAYS: i32 = 126;
pub const OPEN_REPARSE_POINT_REPARSE_IF_CHILD_EXISTS: i32 = 2;
pub const OPEN_REPARSE_POINT_REPARSE_IF_CHILD_NOT_EXISTS: i32 = 4;
pub const OPEN_REPARSE_POINT_REPARSE_IF_DIRECTORY_FINAL_COMPONENT: i32 = 8;
pub const OPEN_REPARSE_POINT_REPARSE_IF_DIRECTORY_FINAL_COMPONENT_ALWAYS: i32 = 72;
pub const OPEN_REPARSE_POINT_REPARSE_IF_FINAL_COMPONENT: i32 = 40;
pub const OPEN_REPARSE_POINT_REPARSE_IF_FINAL_COMPONENT_ALWAYS: i32 = 104;
pub const OPEN_REPARSE_POINT_REPARSE_IF_NON_DIRECTORY_FINAL_COMPONENT: i32 = 32;
pub const OPEN_REPARSE_POINT_REPARSE_IF_NON_DIRECTORY_FINAL_COMPONENT_ALWAYS: i32 = 96;
pub const OPEN_REPARSE_POINT_REPARSE_IF_NON_DIRECTORY_NON_FINAL_COMPONENT: i32 = 16;
pub const OPEN_REPARSE_POINT_REPARSE_IF_NON_DIRECTORY_NON_FINAL_COMPONENT_ALWAYS: i32 = 80;
pub const OPEN_REPARSE_POINT_REPARSE_IF_NON_FINAL_COMPONENT: i32 = 22;
pub const OPEN_REPARSE_POINT_RETURN_REPARSE_DATA_BUFFER: i32 = 128;
pub const OPEN_REPARSE_POINT_TAG_ENCOUNTERED: i32 = 1;
pub const OPEN_REPARSE_POINT_VERSION_EX: u32 = 2147483648;
pub type OPLOCK = *mut core::ffi::c_void;
pub const OPLOCK_FLAG_BACK_OUT_ATOMIC_OPLOCK: i32 = 4;
pub const OPLOCK_FLAG_BREAKING_FOR_SHARING_VIOLATION: i32 = 128;
pub const OPLOCK_FLAG_CLOSING_DELETE_ON_CLOSE: i32 = 32;
pub const OPLOCK_FLAG_COMPLETE_IF_OPLOCKED: i32 = 1;
pub const OPLOCK_FLAG_IGNORE_OPLOCK_KEYS: i32 = 8;
pub const OPLOCK_FLAG_OPLOCK_KEY_CHECK_ONLY: i32 = 2;
pub const OPLOCK_FLAG_PARENT_OBJECT: i32 = 16;
pub const OPLOCK_FLAG_REMOVING_FILE_OR_LINK: i32 = 64;
pub const OPLOCK_FSCTRL_FLAG_ALL_KEYS_MATCH: i32 = 1;
pub const OPLOCK_FS_FILTER_FLAGS_MASK: i32 = 8;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OPLOCK_KEY_ECP_CONTEXT {
    pub OplockKey: windows_core::GUID,
    pub Reserved: u32,
}
pub const OPLOCK_NOTIFY_BREAK_WAIT_INTERIM_TIMEOUT: OPLOCK_NOTIFY_REASON = 0;
pub const OPLOCK_NOTIFY_BREAK_WAIT_TERMINATED: OPLOCK_NOTIFY_REASON = 1;
#[repr(C)]
#[cfg(feature = "usb")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OPLOCK_NOTIFY_PARAMS {
    pub NotifyReason: OPLOCK_NOTIFY_REASON,
    pub NotifyContext: *mut core::ffi::c_void,
    pub Irp: super::PIRP,
    pub Status: windows_core::NTSTATUS,
}
pub type OPLOCK_NOTIFY_REASON = i32;
pub const OPLOCK_UPPER_FLAG_CHECK_NO_BREAK: i32 = 65536;
pub const OPLOCK_UPPER_FLAG_NOTIFY_REFRESH_READ: i32 = 131072;
#[cfg(feature = "winnt")]
pub type PACQUIRE_FOR_LAZY_WRITE = Option<unsafe extern "C" fn(context: *const core::ffi::c_void, wait: super::BOOLEAN) -> super::BOOLEAN>;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type PACQUIRE_FOR_LAZY_WRITE_EX = Option<unsafe extern "C" fn(context: *const core::ffi::c_void, inflags: u32, outflags: super::PULONG) -> super::BOOLEAN>;
#[cfg(feature = "winnt")]
pub type PACQUIRE_FOR_READ_AHEAD = Option<unsafe extern "C" fn(context: *const core::ffi::c_void, wait: super::BOOLEAN) -> super::BOOLEAN>;
pub type PALLOCATE_VIRTUAL_MEMORY_EX_CALLBACK = *mut u8;
#[cfg(feature = "winnt")]
pub type PASYNC_READ_COMPLETION_CALLBACK = Option<unsafe extern "C" fn(context: *const core::ffi::c_void) -> super::BOOLEAN>;
#[cfg(feature = "winnt")]
pub type PATOMIC_CREATE_ECP_CONTEXT = *mut ATOMIC_CREATE_ECP_CONTEXT;
pub type PBASE_MCB = *mut BASE_MCB;
#[cfg(feature = "winnt")]
pub type PCACHE_MANAGER_CALLBACKS = *mut CACHE_MANAGER_CALLBACKS;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type PCACHE_MANAGER_CALLBACKS_EX = *mut CACHE_MANAGER_CALLBACKS_EX;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type PCACHE_MANAGER_CALLBACK_FUNCTIONS = *mut CACHE_MANAGER_CALLBACK_FUNCTIONS;
#[cfg(all(feature = "wdm", feature = "winnt"))]
pub type PCACHE_UNINITIALIZE_EVENT = *mut CACHE_UNINITIALIZE_EVENT;
#[cfg(all(feature = "usb", feature = "wdm", feature = "winnt"))]
pub type PCC_ASYNC_READ_CONTEXT = *mut CC_ASYNC_READ_CONTEXT;
pub type PCC_DIRTY_PAGES_INFO = *mut CC_DIRTY_PAGES_INFO;
#[cfg(feature = "ntdef")]
pub type PCC_ERROR_CALLBACK_CONTEXT = *mut CC_ERROR_CALLBACK_CONTEXT;
#[cfg(feature = "winnt")]
pub type PCC_FILE_SIZES = *mut CC_FILE_SIZES;
pub type PCC_POST_DEFERRED_WRITE = Option<unsafe extern "C" fn(context1: *const core::ffi::c_void, context2: *const core::ffi::c_void)>;
#[cfg(all(feature = "wdm", feature = "winnt"))]
pub type PCHECK_FOR_TRAVERSE_ACCESS = Option<unsafe extern "C" fn(notifycontext: *const core::ffi::c_void, targetcontext: *const core::ffi::c_void, subjectcontext: super::PSECURITY_SUBJECT_CONTEXT) -> super::BOOLEAN>;
#[cfg(feature = "usb")]
pub type PCOMPLETE_LOCK_IRP_ROUTINE = Option<unsafe extern "C" fn(context: *const core::ffi::c_void, irp: super::PIRP) -> windows_core::NTSTATUS>;
pub type PCOMPRESSED_DATA_INFO = *mut COMPRESSED_DATA_INFO;
#[cfg(all(feature = "basetsd", feature = "lsalookup", feature = "ntdef", feature = "ntsecapi", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
pub type PCOPY_INFORMATION = *mut COPY_INFORMATION;
#[cfg(feature = "winnt")]
pub type PCREATE_REDIRECTION_ECP_CONTEXT = *mut CREATE_REDIRECTION_ECP_CONTEXT;
#[cfg(feature = "winnt")]
pub type PCSV_DOWN_LEVEL_OPEN_ECP_CONTEXT = *mut CSV_DOWN_LEVEL_OPEN_ECP_CONTEXT;
pub type PCSV_QUERY_FILE_REVISION_ECP_CONTEXT = *mut CSV_QUERY_FILE_REVISION_ECP_CONTEXT;
#[cfg(feature = "winnt")]
pub type PCSV_QUERY_FILE_REVISION_ECP_CONTEXT_FILE_ID_128 = *mut CSV_QUERY_FILE_REVISION_ECP_CONTEXT_FILE_ID_128;
pub type PCSV_SET_HANDLE_PROPERTIES_ECP_CONTEXT = *mut CSV_SET_HANDLE_PROPERTIES_ECP_CONTEXT;
#[cfg(all(feature = "basetsd", feature = "lsalookup", feature = "ntdef", feature = "ntsecapi", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
pub type PDIRTY_PAGE_ROUTINE = Option<unsafe extern "C" fn(fileobject: super::PFILE_OBJECT, fileoffset: super::PLARGE_INTEGER, length: u32, oldestlsn: super::PLARGE_INTEGER, newestlsn: super::PLARGE_INTEGER, context1: *const core::ffi::c_void, context2: *const core::ffi::c_void)>;
pub type PDRIVER_FS_NOTIFICATION = *mut u8;
#[cfg(feature = "winnt")]
pub type PDUAL_OPLOCK_KEY_ECP_CONTEXT = *mut DUAL_OPLOCK_KEY_ECP_CONTEXT;
#[cfg(feature = "winnt")]
pub type PDUPLICATE_CLUSTER_DATA = *mut DUPLICATE_CLUSTER_DATA;
pub type PECP_HEADER = *mut ECP_HEADER;
#[cfg(feature = "ntddk")]
pub type PECP_LIST = *mut super::ECP_LIST;
pub type PECP_OPEN_PARAMETERS = *mut ECP_OPEN_PARAMETERS;
#[cfg(all(feature = "wdm", feature = "winnt"))]
pub type PEOF_WAIT_BLOCK = *mut EOF_WAIT_BLOCK;
#[cfg(feature = "winnt")]
pub type PEXTENT_READ_CACHE_INFO_BUFFER = *mut EXTENT_READ_CACHE_INFO_BUFFER;
pub type PEXTERNAL_CACHE_CALLBACK_EX = Option<unsafe extern "C" fn(externalcachecontext: *const core::ffi::c_void, dirtypagesinfo: PCC_DIRTY_PAGES_INFO)>;
#[cfg(feature = "winnt")]
pub type PFILE_ACCESS_INFORMATION = *mut FILE_ACCESS_INFORMATION;
#[cfg(feature = "winnt")]
pub type PFILE_ALLOCATION_INFORMATION = *mut FILE_ALLOCATION_INFORMATION;
#[cfg(all(feature = "ntddk", feature = "wdm", feature = "winnt"))]
pub type PFILE_ALL_INFORMATION = *mut FILE_ALL_INFORMATION;
#[cfg(feature = "winnt")]
pub type PFILE_BOTH_DIR_INFORMATION = *mut FILE_BOTH_DIR_INFORMATION;
#[cfg(feature = "winnt")]
pub type PFILE_COMPLETION_INFORMATION = *mut FILE_COMPLETION_INFORMATION;
#[cfg(feature = "winnt")]
pub type PFILE_COMPRESSION_INFORMATION = *mut FILE_COMPRESSION_INFORMATION;
#[cfg(feature = "winnt")]
pub type PFILE_DIRECTORY_INFORMATION = *mut FILE_DIRECTORY_INFORMATION;
pub type PFILE_EA_INFORMATION = *mut FILE_EA_INFORMATION;
#[cfg(feature = "winnt")]
pub type PFILE_END_OF_FILE_INFORMATION_EX = *mut FILE_END_OF_FILE_INFORMATION_EX;
pub type PFILE_FS_ATTRIBUTE_INFORMATION = *mut FILE_FS_ATTRIBUTE_INFORMATION;
#[cfg(feature = "winnt")]
pub type PFILE_FS_CONTROL_INFORMATION = *mut FILE_FS_CONTROL_INFORMATION;
pub type PFILE_FS_DATA_COPY_INFORMATION = *mut FILE_FS_DATA_COPY_INFORMATION;
#[cfg(feature = "winnt")]
pub type PFILE_FS_DRIVER_PATH_INFORMATION = *mut FILE_FS_DRIVER_PATH_INFORMATION;
pub type PFILE_FS_GUID_INFORMATION = *mut FILE_FS_GUID_INFORMATION;
pub type PFILE_FS_VOLUME_FLAGS_INFORMATION = *mut FILE_FS_VOLUME_FLAGS_INFORMATION;
#[cfg(feature = "winnt")]
pub type PFILE_FULL_DIR_INFORMATION = *mut FILE_FULL_DIR_INFORMATION;
pub type PFILE_GET_EA_INFORMATION = *mut FILE_GET_EA_INFORMATION;
#[cfg(feature = "winnt")]
pub type PFILE_ID_64_EXTD_BOTH_DIR_INFORMATION = *mut FILE_ID_64_EXTD_BOTH_DIR_INFORMATION;
#[cfg(feature = "winnt")]
pub type PFILE_ID_64_EXTD_DIR_INFORMATION = *mut FILE_ID_64_EXTD_DIR_INFORMATION;
#[cfg(feature = "winnt")]
pub type PFILE_ID_ALL_EXTD_BOTH_DIR_INFORMATION = *mut FILE_ID_ALL_EXTD_BOTH_DIR_INFORMATION;
#[cfg(feature = "winnt")]
pub type PFILE_ID_ALL_EXTD_DIR_INFORMATION = *mut FILE_ID_ALL_EXTD_DIR_INFORMATION;
#[cfg(feature = "winnt")]
pub type PFILE_ID_BOTH_DIR_INFORMATION = *mut FILE_ID_BOTH_DIR_INFORMATION;
#[cfg(feature = "winnt")]
pub type PFILE_ID_EXTD_BOTH_DIR_INFORMATION = *mut FILE_ID_EXTD_BOTH_DIR_INFORMATION;
#[cfg(feature = "winnt")]
pub type PFILE_ID_EXTD_DIR_INFORMATION = *mut FILE_ID_EXTD_DIR_INFORMATION;
#[cfg(feature = "winnt")]
pub type PFILE_ID_FULL_DIR_INFORMATION = *mut FILE_ID_FULL_DIR_INFORMATION;
#[cfg(feature = "winnt")]
pub type PFILE_ID_GLOBAL_TX_DIR_INFORMATION = *mut FILE_ID_GLOBAL_TX_DIR_INFORMATION;
#[cfg(feature = "winnt")]
pub type PFILE_ID_INFORMATION = *mut FILE_ID_INFORMATION;
#[cfg(feature = "wdm")]
pub type PFILE_INFORMATION_DEFINITION = *mut FILE_INFORMATION_DEFINITION;
#[cfg(feature = "winnt")]
pub type PFILE_INTERNAL_INFORMATION = *mut FILE_INTERNAL_INFORMATION;
pub type PFILE_KNOWN_FOLDER_INFORMATION = *mut FILE_KNOWN_FOLDER_INFORMATION;
#[cfg(feature = "winnt")]
pub type PFILE_LINKS_FULL_ID_INFORMATION = *mut FILE_LINKS_FULL_ID_INFORMATION;
pub type PFILE_LINKS_INFORMATION = *mut FILE_LINKS_INFORMATION;
#[cfg(feature = "winnt")]
pub type PFILE_LINK_ENTRY_FULL_ID_INFORMATION = *mut FILE_LINK_ENTRY_FULL_ID_INFORMATION;
pub type PFILE_LINK_ENTRY_INFORMATION = *mut FILE_LINK_ENTRY_INFORMATION;
#[cfg(feature = "winnt")]
pub type PFILE_LINK_INFORMATION = *mut FILE_LINK_INFORMATION;
#[cfg(all(feature = "basetsd", feature = "lsalookup", feature = "ntdef", feature = "ntsecapi", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
pub type PFILE_LOCK = *mut FILE_LOCK;
#[cfg(all(feature = "basetsd", feature = "lsalookup", feature = "ntdef", feature = "ntsecapi", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
pub type PFILE_LOCK_INFO = *mut FILE_LOCK_INFO;
#[cfg(feature = "winnt")]
pub type PFILE_MAILSLOT_QUERY_INFORMATION = *mut FILE_MAILSLOT_QUERY_INFORMATION;
#[cfg(feature = "winnt")]
pub type PFILE_MAILSLOT_SET_INFORMATION = *mut FILE_MAILSLOT_SET_INFORMATION;
pub type PFILE_MODE_INFORMATION = *mut FILE_MODE_INFORMATION;
#[cfg(feature = "winnt")]
pub type PFILE_MOVE_CLUSTER_INFORMATION = *mut FILE_MOVE_CLUSTER_INFORMATION;
pub type PFILE_NAMES_INFORMATION = *mut FILE_NAMES_INFORMATION;
pub type PFILE_NETWORK_PHYSICAL_NAME_INFORMATION = *mut FILE_NETWORK_PHYSICAL_NAME_INFORMATION;
pub type PFILE_OBJECTID_INFORMATION = *mut FILE_OBJECTID_INFORMATION;
#[cfg(feature = "winnt")]
pub type PFILE_PIPE_ASSIGN_EVENT_BUFFER = *mut FILE_PIPE_ASSIGN_EVENT_BUFFER;
pub type PFILE_PIPE_CLIENT_PROCESS_BUFFER = *mut FILE_PIPE_CLIENT_PROCESS_BUFFER;
pub type PFILE_PIPE_CLIENT_PROCESS_BUFFER_EX = *mut FILE_PIPE_CLIENT_PROCESS_BUFFER_EX;
pub type PFILE_PIPE_CLIENT_PROCESS_BUFFER_V2 = *mut FILE_PIPE_CLIENT_PROCESS_BUFFER_V2;
pub type PFILE_PIPE_CREATE_SYMLINK_INPUT = *mut FILE_PIPE_CREATE_SYMLINK_INPUT;
pub type PFILE_PIPE_DELETE_SYMLINK_INPUT = *mut FILE_PIPE_DELETE_SYMLINK_INPUT;
pub type PFILE_PIPE_EVENT_BUFFER = *mut FILE_PIPE_EVENT_BUFFER;
pub type PFILE_PIPE_INFORMATION = *mut FILE_PIPE_INFORMATION;
pub type PFILE_PIPE_LOCAL_INFORMATION = *mut FILE_PIPE_LOCAL_INFORMATION;
pub type PFILE_PIPE_PEEK_BUFFER = *mut FILE_PIPE_PEEK_BUFFER;
#[cfg(feature = "winnt")]
pub type PFILE_PIPE_REMOTE_INFORMATION = *mut FILE_PIPE_REMOTE_INFORMATION;
#[cfg(feature = "winnt")]
pub type PFILE_PIPE_SILO_ARRIVAL_INPUT = *mut FILE_PIPE_SILO_ARRIVAL_INPUT;
#[cfg(feature = "winnt")]
pub type PFILE_PIPE_WAIT_FOR_BUFFER = *mut FILE_PIPE_WAIT_FOR_BUFFER;
#[cfg(feature = "winnt")]
pub type PFILE_QUOTA_INFORMATION = *mut FILE_QUOTA_INFORMATION;
pub type PFILE_REMOTE_PROTOCOL_INFORMATION = *mut FILE_REMOTE_PROTOCOL_INFORMATION;
#[cfg(feature = "winnt")]
pub type PFILE_RENAME_INFORMATION = *mut FILE_RENAME_INFORMATION;
pub type PFILE_REPARSE_POINT_INFORMATION = *mut FILE_REPARSE_POINT_INFORMATION;
#[cfg(feature = "winnt")]
pub type PFILE_STANDARD_LINK_INFORMATION = *mut FILE_STANDARD_LINK_INFORMATION;
#[cfg(feature = "winioctl")]
pub type PFILE_STORAGE_RESERVE_ID_INFORMATION = *mut FILE_STORAGE_RESERVE_ID_INFORMATION;
#[cfg(feature = "winnt")]
pub type PFILE_STREAM_INFORMATION = *mut FILE_STREAM_INFORMATION;
pub type PFILE_STREAM_RESERVATION_INFORMATION = *mut FILE_STREAM_RESERVATION_INFORMATION;
#[cfg(feature = "winnt")]
pub type PFILE_TIMESTAMPS = *mut FILE_TIMESTAMPS;
#[cfg(feature = "winnt")]
pub type PFILE_TRACKING_INFORMATION = *mut FILE_TRACKING_INFORMATION;
pub type PFILE_VOLUME_NAME_INFORMATION = *mut FILE_VOLUME_NAME_INFORMATION;
#[cfg(feature = "winnt")]
pub type PFILTER_REPORT_CHANGE = Option<unsafe extern "C" fn(notifycontext: *const core::ffi::c_void, filtercontext: *const core::ffi::c_void) -> super::BOOLEAN>;
#[cfg(feature = "winnt")]
pub type PFLUSH_TO_LSN = Option<unsafe extern "C" fn(loghandle: *const core::ffi::c_void, lsn: super::LARGE_INTEGER)>;
#[cfg(all(feature = "ntdef", feature = "wdm", feature = "winnt"))]
pub type PFN_FSRTLTEARDOWNPERSTREAMCONTEXTS = Option<unsafe extern "C" fn(advancedheader: PFSRTL_ADVANCED_FCB_HEADER)>;
pub type PFREE_VIRTUAL_MEMORY_EX_CALLBACK = *mut u8;
#[cfg(feature = "winnt")]
pub type PFSCTL_GHOST_FILE_EXTENTS_INPUT_BUFFER = *mut FSCTL_GHOST_FILE_EXTENTS_INPUT_BUFFER;
#[cfg(feature = "winnt")]
pub type PFSCTL_QUERY_GHOSTED_FILE_EXTENTS_INPUT_RANGE = *mut FSCTL_QUERY_GHOSTED_FILE_EXTENTS_INPUT_RANGE;
pub type PFSCTL_QUERY_GHOSTED_FILE_EXTENTS_OUTPUT = *mut FSCTL_QUERY_GHOSTED_FILE_EXTENTS_OUTPUT;
pub type PFSCTL_QUERY_VOLUME_NUMA_INFO_OUTPUT = *mut FSCTL_QUERY_VOLUME_NUMA_INFO_OUTPUT;
#[cfg(feature = "winnt")]
pub type PFSCTL_UNMAP_SPACE_INPUT_BUFFER = *mut FSCTL_UNMAP_SPACE_INPUT_BUFFER;
#[cfg(feature = "winnt")]
pub type PFSCTL_UNMAP_SPACE_OUTPUT = *mut FSCTL_UNMAP_SPACE_OUTPUT;
#[cfg(all(feature = "ntdef", feature = "wdm", feature = "winnt"))]
pub type PFSRTL_ADVANCED_FCB_HEADER = *mut FSRTL_ADVANCED_FCB_HEADER;
#[cfg(feature = "usb")]
pub type PFSRTL_AUXILIARY_BUFFER = *mut FSRTL_AUXILIARY_BUFFER;
pub type PFSRTL_CHANGE_BACKING_TYPE = *mut FSRTL_CHANGE_BACKING_TYPE;
#[cfg(all(feature = "ntdef", feature = "wdm", feature = "winnt"))]
pub type PFSRTL_COMMON_FCB_HEADER = *mut FSRTL_COMMON_FCB_HEADER;
#[cfg(feature = "guiddef")]
pub type PFSRTL_EXTRA_CREATE_PARAMETER_CLEANUP_CALLBACK = Option<unsafe extern "C" fn(ecpcontext: *mut core::ffi::c_void, ecptype: super::LPCGUID)>;
pub type PFSRTL_MUP_PROVIDER_INFO_LEVEL_1 = *mut FSRTL_MUP_PROVIDER_INFO_LEVEL_1;
#[cfg(feature = "winternl")]
pub type PFSRTL_MUP_PROVIDER_INFO_LEVEL_2 = *mut FSRTL_MUP_PROVIDER_INFO_LEVEL_2;
#[cfg(feature = "winnt")]
pub type PFSRTL_PER_FILEOBJECT_CONTEXT = *mut FSRTL_PER_FILEOBJECT_CONTEXT;
#[cfg(all(feature = "wdm", feature = "winnt"))]
pub type PFSRTL_PER_FILE_CONTEXT = *mut FSRTL_PER_FILE_CONTEXT;
#[cfg(all(feature = "wdm", feature = "winnt"))]
pub type PFSRTL_PER_STREAM_CONTEXT = *mut FSRTL_PER_STREAM_CONTEXT;
#[cfg(all(feature = "wdm", feature = "winnt"))]
pub type PFSRTL_STACK_OVERFLOW_ROUTINE = Option<unsafe extern "C" fn(context: *const core::ffi::c_void, event: super::PKEVENT)>;
pub type PFSRTL_UNC_PROVIDER_REGISTRATION = *mut FSRTL_UNC_PROVIDER_REGISTRATION;
#[cfg(all(feature = "basetsd", feature = "lsalookup", feature = "minwindef", feature = "ntdef", feature = "ntsecapi", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
pub type PFS_FILTER_CALLBACK = Option<unsafe extern "C" fn(data: PFS_FILTER_CALLBACK_DATA, completioncontext: *mut *mut core::ffi::c_void) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "basetsd", feature = "lsalookup", feature = "minwindef", feature = "ntdef", feature = "ntsecapi", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
pub type PFS_FILTER_CALLBACKS = *mut FS_FILTER_CALLBACKS;
#[cfg(all(feature = "basetsd", feature = "lsalookup", feature = "minwindef", feature = "ntdef", feature = "ntsecapi", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
pub type PFS_FILTER_CALLBACK_DATA = *mut FS_FILTER_CALLBACK_DATA;
#[cfg(all(feature = "basetsd", feature = "lsalookup", feature = "minwindef", feature = "ntdef", feature = "ntsecapi", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
pub type PFS_FILTER_COMPLETION_CALLBACK = Option<unsafe extern "C" fn(data: PFS_FILTER_CALLBACK_DATA, operationstatus: windows_core::NTSTATUS, completioncontext: *const core::ffi::c_void)>;
#[cfg(all(feature = "minwindef", feature = "usb", feature = "wdm", feature = "winnt"))]
pub type PFS_FILTER_PARAMETERS = *mut FS_FILTER_PARAMETERS;
pub type PFS_FILTER_SECTION_SYNC_OUTPUT = *mut FS_FILTER_SECTION_SYNC_OUTPUT;
pub type PFS_FILTER_SECTION_SYNC_TYPE = *mut FS_FILTER_SECTION_SYNC_TYPE;
pub type PFS_FILTER_STREAM_FO_NOTIFICATION_TYPE = *mut FS_FILTER_STREAM_FO_NOTIFICATION_TYPE;
#[cfg(feature = "winnt")]
pub type PGENERATE_NAME_CONTEXT = *mut GENERATE_NAME_CONTEXT;
#[cfg(feature = "winnt")]
pub type PGHOSTED_FILE_EXTENT = *mut GHOSTED_FILE_EXTENT;
pub const PHCM_APPLICATION_DEFAULT: i8 = 0;
pub const PHCM_DISGUISE_FULL_PLACEHOLDERS: i8 = 3;
pub const PHCM_DISGUISE_PLACEHOLDERS: i8 = 1;
pub const PHCM_ERROR_INVALID_PARAMETER: i8 = -1;
pub const PHCM_ERROR_NO_PEB: i8 = -3;
pub const PHCM_ERROR_NO_TEB: i8 = -2;
pub const PHCM_EXPOSE_PLACEHOLDERS: i8 = 2;
pub const PHCM_MAX: i8 = 3;
#[repr(C)]
#[cfg(feature = "wdm")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PHYSICAL_EXTENTS_DESCRIPTOR {
    pub NumberOfRuns: u32,
    pub NumberOfValidRuns: u32,
    pub Run: [PHYSICAL_MEMORY_RUN; 1],
}
#[cfg(feature = "wdm")]
impl Default for PHYSICAL_EXTENTS_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "wdm")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PHYSICAL_MEMORY_DESCRIPTOR {
    pub NumberOfRuns: u32,
    pub NumberOfPages: super::PFN_NUMBER,
    pub Run: [PHYSICAL_MEMORY_RUN; 1],
}
#[cfg(feature = "wdm")]
impl Default for PHYSICAL_MEMORY_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "wdm")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PHYSICAL_MEMORY_RUN {
    pub BasePage: super::PFN_NUMBER,
    pub PageCount: super::PFN_NUMBER,
}
pub const PIN_CALLER_TRACKS_DIRTY_DATA: i32 = 32;
pub const PIN_EXCLUSIVE: i32 = 2;
pub const PIN_HIGH_PRIORITY: i32 = 64;
pub const PIN_IF_BCB: i32 = 8;
pub const PIN_NO_READ: i32 = 4;
pub const PIN_VERIFY_REQUIRED: i32 = 128;
pub const PIN_WAIT: i32 = 1;
#[cfg(all(feature = "basetsd", feature = "lsalookup", feature = "ntdef", feature = "ntsecapi", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
pub type PIO_CREATE_STREAM_FILE_OPTIONS = *mut IO_CREATE_STREAM_FILE_OPTIONS;
#[cfg(all(feature = "basetsd", feature = "lsalookup", feature = "ntdef", feature = "ntsecapi", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
pub type PIO_DEVICE_HINT_ECP_CONTEXT = *mut IO_DEVICE_HINT_ECP_CONTEXT;
#[cfg(feature = "wdm")]
pub type PIO_PRIORITY_INFO = *mut IO_PRIORITY_INFO;
pub type PIO_STOP_ON_SYMLINK_FILTER_ECP_v0 = *mut IO_STOP_ON_SYMLINK_FILTER_ECP_v0;
#[cfg(all(feature = "wdm", feature = "winnt"))]
pub type PKAPC_STATE = *mut KAPC_STATE;
#[cfg(all(feature = "wdm", feature = "winnt"))]
pub type PKQUEUE = *mut KQUEUE;
#[cfg(all(feature = "wdm", feature = "winnt"))]
pub type PLARGE_MCB = *mut LARGE_MCB;
pub type PLBN = *mut LBN;
pub type PLCN_WEAK_REFERENCE_BUFFER = *mut LCN_WEAK_REFERENCE_BUFFER;
pub type PLCN_WEAK_REFERENCE_CLEAR_INPUT_BUFFER = *mut LCN_WEAK_REFERENCE_CLEAR_INPUT_BUFFER;
pub type PLCN_WEAK_REFERENCE_CREATE_FLAGS = *mut LCN_WEAK_REFERENCE_CREATE_FLAGS;
pub type PLCN_WEAK_REFERENCE_CREATE_INPUT_BUFFER = *mut LCN_WEAK_REFERENCE_CREATE_INPUT_BUFFER;
pub type PLCN_WEAK_REFERENCE_CREATE_OUTPUT_BUFFER = *mut LCN_WEAK_REFERENCE_CREATE_OUTPUT_BUFFER;
pub type PLCN_WEAK_REFERENCE_RANGE = *mut LCN_WEAK_REFERENCE_RANGE;
pub type PLCN_WEAK_REFERENCE_STATE = *mut LCN_WEAK_REFERENCE_STATE;
pub type PLCN_WEAK_REFERENCE_VCN_MAPPING = *mut LCN_WEAK_REFERENCE_VCN_MAPPING;
pub type PLINK_TRACKING_INFORMATION = *mut LINK_TRACKING_INFORMATION;
pub type PLINK_TRACKING_INFORMATION_TYPE = *mut LINK_TRACKING_INFORMATION_TYPE;
#[cfg(all(feature = "wdm", feature = "winnt"))]
pub type PMCB = *mut MCB;
pub type PMEMORY_RANGE_ENTRY = *mut MEMORY_RANGE_ENTRY;
pub type PMM_PREFETCH_FLAGS = *mut MM_PREFETCH_FLAGS;
#[cfg(feature = "ntsecapi")]
pub type PMSV1_0_ENUMUSERS_REQUEST = *mut MSV1_0_ENUMUSERS_REQUEST;
#[cfg(all(feature = "minwindef", feature = "ntsecapi", feature = "winnt"))]
pub type PMSV1_0_ENUMUSERS_RESPONSE = *mut MSV1_0_ENUMUSERS_RESPONSE;
#[cfg(all(feature = "ntsecapi", feature = "winnt", feature = "winternl"))]
pub type PMSV1_0_GETCHALLENRESP_REQUEST = *mut MSV1_0_GETCHALLENRESP_REQUEST;
#[cfg(all(feature = "ntsecapi", feature = "winnt", feature = "winternl"))]
pub type PMSV1_0_GETCHALLENRESP_REQUEST_V1 = *mut MSV1_0_GETCHALLENRESP_REQUEST_V1;
#[cfg(all(feature = "ntsecapi", feature = "winnt", feature = "winternl"))]
pub type PMSV1_0_GETCHALLENRESP_RESPONSE = *mut MSV1_0_GETCHALLENRESP_RESPONSE;
#[cfg(all(feature = "ntsecapi", feature = "winnt"))]
pub type PMSV1_0_GETUSERINFO_REQUEST = *mut MSV1_0_GETUSERINFO_REQUEST;
#[cfg(all(feature = "ntsecapi", feature = "winnt", feature = "winternl"))]
pub type PMSV1_0_GETUSERINFO_RESPONSE = *mut MSV1_0_GETUSERINFO_RESPONSE;
#[cfg(feature = "ntsecapi")]
pub type PMSV1_0_LM20_CHALLENGE_REQUEST = *mut MSV1_0_LM20_CHALLENGE_REQUEST;
#[cfg(feature = "ntsecapi")]
pub type PMSV1_0_LM20_CHALLENGE_RESPONSE = *mut MSV1_0_LM20_CHALLENGE_RESPONSE;
#[cfg(feature = "minwindef")]
pub type PMUP_PROVIDER_INFORMATION = *mut MUP_PROVIDER_INFORMATION;
pub type PNETWORK_APP_INSTANCE_ECP_CONTEXT = *mut NETWORK_APP_INSTANCE_ECP_CONTEXT;
pub type PNETWORK_APP_INSTANCE_VERSION_ECP_CONTEXT = *mut NETWORK_APP_INSTANCE_VERSION_ECP_CONTEXT;
pub type PNETWORK_OPEN_ECP_CONTEXT = *mut NETWORK_OPEN_ECP_CONTEXT;
pub type PNETWORK_OPEN_ECP_CONTEXT_V0 = *mut NETWORK_OPEN_ECP_CONTEXT_V0;
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
pub type PNFS_OPEN_ECP_CONTEXT = *mut NFS_OPEN_ECP_CONTEXT;
pub type PNOTIFY_SYNC = *mut _REAL_NOTIFY_SYNC;
#[cfg(feature = "winnt")]
pub type POPEN_REPARSE_LIST = *mut OPEN_REPARSE_LIST;
#[cfg(feature = "winnt")]
pub type POPEN_REPARSE_LIST_ENTRY = *mut OPEN_REPARSE_LIST_ENTRY;
pub type POPLOCK = *mut *mut core::ffi::c_void;
#[cfg(feature = "usb")]
pub type POPLOCK_FS_PREPOST_IRP = Option<unsafe extern "C" fn(context: *const core::ffi::c_void, irp: super::PIRP)>;
#[cfg(feature = "usb")]
pub type POPLOCK_NOTIFY_PARAMS = *mut OPLOCK_NOTIFY_PARAMS;
#[cfg(feature = "usb")]
pub type POPLOCK_NOTIFY_ROUTINE = Option<unsafe extern "C" fn(notifyparams: POPLOCK_NOTIFY_PARAMS) -> windows_core::NTSTATUS>;
#[cfg(feature = "usb")]
pub type POPLOCK_WAIT_COMPLETE_ROUTINE = Option<unsafe extern "C" fn(context: *const core::ffi::c_void, irp: super::PIRP)>;
#[cfg(feature = "wdm")]
pub type PPHYSICAL_EXTENTS_DESCRIPTOR = *mut PHYSICAL_EXTENTS_DESCRIPTOR;
#[cfg(feature = "wdm")]
pub type PPHYSICAL_MEMORY_DESCRIPTOR = *mut PHYSICAL_MEMORY_DESCRIPTOR;
#[cfg(feature = "wdm")]
pub type PPHYSICAL_MEMORY_RUN = *mut PHYSICAL_MEMORY_RUN;
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
pub type PPNFS_OPEN_ECP_CONTEXT = *mut *mut NFS_OPEN_ECP_CONTEXT;
pub type PPREFETCH_OPEN_ECP_CONTEXT = *mut PREFETCH_OPEN_ECP_CONTEXT;
#[cfg(all(feature = "lsalookup", feature = "ntddk", feature = "ntdef", feature = "ntsecapi", feature = "winnt"))]
pub type PPREFIX_TABLE = *mut PREFIX_TABLE;
#[cfg(all(feature = "lsalookup", feature = "ntddk", feature = "ntdef", feature = "ntsecapi", feature = "winnt"))]
pub type PPREFIX_TABLE_ENTRY = *mut PREFIX_TABLE_ENTRY;
#[cfg(all(feature = "ntdef", feature = "winnt"))]
pub type PPUBLIC_BCB = *mut PUBLIC_BCB;
pub type PQUERY_DIRECT_ACCESS_EXTENTS = *mut QUERY_DIRECT_ACCESS_EXTENTS;
#[cfg(feature = "minwindef")]
pub type PQUERY_LOG_USAGE = Option<unsafe extern "C" fn(loghandle: *const core::ffi::c_void, percentagefull: super::PUSHORT)>;
#[cfg(feature = "wdm")]
pub type PQUERY_ON_CREATE_EA_INFORMATION = *mut QUERY_ON_CREATE_EA_INFORMATION;
#[cfg(all(feature = "wdm", feature = "winnt"))]
pub type PQUERY_ON_CREATE_ECP_CONTEXT = *mut QUERY_ON_CREATE_ECP_CONTEXT;
#[cfg(feature = "winnt")]
pub type PQUERY_ON_CREATE_FILE_LX_INFORMATION = *mut QUERY_ON_CREATE_FILE_LX_INFORMATION;
#[cfg(feature = "winnt")]
pub type PQUERY_ON_CREATE_FILE_STAT_INFORMATION = *mut QUERY_ON_CREATE_FILE_STAT_INFORMATION;
#[cfg(feature = "winnt")]
pub type PQUERY_ON_CREATE_SECURITY_INFORMATION = *mut QUERY_ON_CREATE_SECURITY_INFORMATION;
#[cfg(feature = "winnt")]
pub type PQUERY_ON_CREATE_USN_INFORMATION = *mut QUERY_ON_CREATE_USN_INFORMATION;
#[cfg(all(feature = "wdm", feature = "winnt", feature = "winternl"))]
pub type PQUERY_PATH_REQUEST = *mut QUERY_PATH_REQUEST;
#[cfg(all(feature = "ntddk", feature = "wdm", feature = "winnt", feature = "winternl"))]
pub type PQUERY_PATH_REQUEST_EX = *mut QUERY_PATH_REQUEST_EX;
pub type PQUERY_PATH_RESPONSE = *mut QUERY_PATH_RESPONSE;
pub type PQUERY_VIRTUAL_MEMORY_CALLBACK = *mut u8;
#[cfg(feature = "ntdef")]
pub type PREAD_AHEAD_PARAMETERS = *mut READ_AHEAD_PARAMETERS;
#[cfg(all(feature = "basetsd", feature = "lsalookup", feature = "ntdef", feature = "ntsecapi", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
pub type PREAD_LIST = *mut READ_LIST;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PREFETCH_OPEN_ECP_CONTEXT {
    pub Context: *mut core::ffi::c_void,
}
#[repr(C)]
#[cfg(all(feature = "lsalookup", feature = "ntddk", feature = "ntdef", feature = "ntsecapi", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PREFIX_TABLE {
    pub NodeTypeCode: super::CSHORT,
    pub NameLength: super::CSHORT,
    pub NextPrefixTree: PPREFIX_TABLE_ENTRY,
}
#[repr(C)]
#[cfg(all(feature = "lsalookup", feature = "ntddk", feature = "ntdef", feature = "ntsecapi", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PREFIX_TABLE_ENTRY {
    pub NodeTypeCode: super::CSHORT,
    pub NameLength: super::CSHORT,
    pub NextPrefixTree: *mut Self,
    pub Links: super::RTL_SPLAY_LINKS,
    pub Prefix: super::PSTRING,
}
pub type PREFS_COMPRESSION_FORMATS = *mut REFS_COMPRESSION_FORMATS;
pub type PREFS_DEALLOCATE_RANGES_ALLOCATOR = *mut REFS_DEALLOCATE_RANGES_ALLOCATOR;
pub type PREFS_DEALLOCATE_RANGES_INPUT_BUFFER = *mut REFS_DEALLOCATE_RANGES_INPUT_BUFFER;
pub type PREFS_DEALLOCATE_RANGES_INPUT_BUFFER_EX = *mut REFS_DEALLOCATE_RANGES_INPUT_BUFFER_EX;
pub type PREFS_DEALLOCATE_RANGES_RANGE = *mut REFS_DEALLOCATE_RANGES_RANGE;
#[cfg(feature = "winnt")]
pub type PREFS_QUERY_ROLLBACK_PROTECTION_INFO_OUTPUT_BUFFER = *mut REFS_QUERY_ROLLBACK_PROTECTION_INFO_OUTPUT_BUFFER;
pub type PREFS_QUERY_VOLUME_COMPRESSION_INFO_FLAGS = *mut REFS_QUERY_VOLUME_COMPRESSION_INFO_FLAGS;
pub type PREFS_QUERY_VOLUME_COMPRESSION_INFO_OUTPUT_BUFFER = *mut REFS_QUERY_VOLUME_COMPRESSION_INFO_OUTPUT_BUFFER;
pub type PREFS_QUERY_VOLUME_IO_METRICS_INFO_INPUT_BUFFER = *mut REFS_QUERY_VOLUME_IO_METRICS_INFO_INPUT_BUFFER;
pub type PREFS_QUERY_VOLUME_IO_METRICS_INFO_OUTPUT_BUFFER = *mut REFS_QUERY_VOLUME_IO_METRICS_INFO_OUTPUT_BUFFER;
pub type PREFS_QUERY_VOLUME_IO_METRICS_INFO_QUERY_TYPE = *mut REFS_QUERY_VOLUME_IO_METRICS_INFO_QUERY_TYPE;
pub type PREFS_QUERY_VOLUME_IO_METRICS_METRICS_DATA = *mut REFS_QUERY_VOLUME_IO_METRICS_METRICS_DATA;
pub type PREFS_QUERY_VOLUME_TOTAL_SHARED_LCNS_OUTPUT_BUFFER = *mut REFS_QUERY_VOLUME_TOTAL_SHARED_LCNS_OUTPUT_BUFFER;
pub type PREFS_REMOVE_HARDLINK_BACKPOINTER = *mut REFS_REMOVE_HARDLINK_BACKPOINTER;
#[cfg(feature = "winnt")]
pub type PREFS_SET_ROLLBACK_PROTECTION_INFO_INPUT_BUFFER = *mut REFS_SET_ROLLBACK_PROTECTION_INFO_INPUT_BUFFER;
pub type PREFS_SET_VOLUME_COMPRESSION_INFO_FLAGS = *mut REFS_SET_VOLUME_COMPRESSION_INFO_FLAGS;
pub type PREFS_SET_VOLUME_COMPRESSION_INFO_INPUT_BUFFER = *mut REFS_SET_VOLUME_COMPRESSION_INFO_INPUT_BUFFER;
pub type PREFS_SET_VOLUME_IO_METRICS_INFO_INPUT_BUFFER = *mut REFS_SET_VOLUME_IO_METRICS_INFO_INPUT_BUFFER;
pub type PREFS_STREAM_EXTENT = *mut REFS_STREAM_EXTENT;
pub type PREFS_STREAM_EXTENT_PROPERTIES = *mut REFS_STREAM_EXTENT_PROPERTIES;
pub type PREFS_STREAM_SNAPSHOT_LIST_OUTPUT_BUFFER = *mut REFS_STREAM_SNAPSHOT_LIST_OUTPUT_BUFFER;
pub type PREFS_STREAM_SNAPSHOT_LIST_OUTPUT_BUFFER_ENTRY = *mut REFS_STREAM_SNAPSHOT_LIST_OUTPUT_BUFFER_ENTRY;
pub type PREFS_STREAM_SNAPSHOT_MANAGEMENT_INPUT_BUFFER = *mut REFS_STREAM_SNAPSHOT_MANAGEMENT_INPUT_BUFFER;
pub type PREFS_STREAM_SNAPSHOT_OPERATION = *mut REFS_STREAM_SNAPSHOT_OPERATION;
pub type PREFS_STREAM_SNAPSHOT_QUERY_DELTAS_INPUT_BUFFER = *mut REFS_STREAM_SNAPSHOT_QUERY_DELTAS_INPUT_BUFFER;
pub type PREFS_STREAM_SNAPSHOT_QUERY_DELTAS_OUTPUT_BUFFER = *mut REFS_STREAM_SNAPSHOT_QUERY_DELTAS_OUTPUT_BUFFER;
#[cfg(feature = "winnt")]
pub type PREFS_VOLUME_COUNTER_INFO_INPUT_BUFFER = *mut REFS_VOLUME_COUNTER_INFO_INPUT_BUFFER;
#[cfg(feature = "winnt")]
pub type PREFS_VOLUME_DEDUP_INFO_INPUT_BUFFER = *mut REFS_VOLUME_DEDUP_INFO_INPUT_BUFFER;
#[cfg(feature = "winnt")]
pub type PREFS_VOLUME_DEDUP_INFO_OUTPUT_BUFFER = *mut REFS_VOLUME_DEDUP_INFO_OUTPUT_BUFFER;
pub type PRELEASE_FROM_LAZY_WRITE = Option<unsafe extern "C" fn(context: *const core::ffi::c_void)>;
pub type PRELEASE_FROM_READ_AHEAD = Option<unsafe extern "C" fn(context: *const core::ffi::c_void)>;
pub type PREMOTE_LINK_TRACKING_INFORMATION = *mut REMOTE_LINK_TRACKING_INFORMATION;
pub type PREPARSE_DATA_BUFFER = *mut REPARSE_DATA_BUFFER;
#[cfg(feature = "winnt")]
pub type PREPARSE_DATA_BUFFER_EX = *mut REPARSE_DATA_BUFFER_EX;
#[cfg(feature = "winnt")]
pub type PREPARSE_INDEX_KEY = *mut REPARSE_INDEX_KEY;
#[cfg(all(feature = "wdm", feature = "winnt"))]
pub type PRKAPC_STATE = *mut KAPC_STATE;
pub type PRKF_BYPASS_ECP_CONTEXT = *mut RKF_BYPASS_ECP_CONTEXT;
#[cfg(all(feature = "wdm", feature = "winnt"))]
pub type PRKQUEUE = *mut KQUEUE;
pub type PRTL_ALLOCATE_STRING_ROUTINE = Option<unsafe extern "system" fn(numberofbytes: usize) -> *mut core::ffi::c_void>;
pub type PRTL_FREE_STRING_ROUTINE = Option<unsafe extern "system" fn(buffer: *const core::ffi::c_void)>;
#[cfg(feature = "basetsd")]
pub type PRTL_HEAP_COMMIT_ROUTINE = Option<unsafe extern "system" fn(base: *const core::ffi::c_void, commitaddress: *mut *mut core::ffi::c_void, commitsize: super::PSIZE_T) -> windows_core::NTSTATUS>;
pub type PRTL_HEAP_MEMORY_LIMIT_DATA = *mut RTL_HEAP_MEMORY_LIMIT_DATA;
pub type PRTL_HEAP_MEMORY_LIMIT_INFO = *mut RTL_HEAP_MEMORY_LIMIT_INFO;
#[cfg(feature = "basetsd")]
pub type PRTL_HEAP_PARAMETERS = *mut RTL_HEAP_PARAMETERS;
pub type PRTL_MEMORY_TYPE = *mut RTL_MEMORY_TYPE;
pub type PRTL_REALLOCATE_STRING_ROUTINE = Option<unsafe extern "system" fn(numberofbytes: usize, buffer: *const core::ffi::c_void) -> *mut core::ffi::c_void>;
#[cfg(feature = "winnt")]
pub type PRTL_SEGMENT_HEAP_MEMORY_SOURCE = *mut RTL_SEGMENT_HEAP_MEMORY_SOURCE;
#[cfg(feature = "winnt")]
pub type PRTL_SEGMENT_HEAP_PARAMETERS = *mut RTL_SEGMENT_HEAP_PARAMETERS;
#[cfg(feature = "winnt")]
pub type PRTL_SEGMENT_HEAP_VA_CALLBACKS = *mut RTL_SEGMENT_HEAP_VA_CALLBACKS;
#[cfg(feature = "winnt")]
pub type PSECURITY_CLIENT_CONTEXT = *mut SECURITY_CLIENT_CONTEXT;
#[cfg(feature = "winnt")]
pub type PSET_CACHED_RUNS_STATE_INPUT_BUFFER = *mut SET_CACHED_RUNS_STATE_INPUT_BUFFER;
#[cfg(all(feature = "winnt", feature = "winternl"))]
pub type PSE_AUDIT_INFO = *mut SE_AUDIT_INFO;
pub type PSE_AUDIT_OPERATION = *mut SE_AUDIT_OPERATION;
#[cfg(feature = "winnt")]
pub type PSE_EXPORTS = *mut SE_EXPORTS;
pub type PSE_LOGON_SESSION_TERMINATED_ROUTINE = *mut u8;
pub type PSE_LOGON_SESSION_TERMINATED_ROUTINE_EX = *mut u8;
pub const PSMP_MAXIMUM_SYSAPP_CLAIM_VALUES: i32 = 4;
pub const PSMP_MINIMUM_SYSAPP_CLAIM_VALUES: i32 = 2;
pub type PSOCKADDR_STORAGE_NFS = *mut sockaddr_storage;
#[cfg(feature = "winnt")]
pub type PSOV_RANGE_CHECK_DATA = *mut SOV_RANGE_CHECK_DATA;
pub type PSRV_INSTANCE_TYPE = *mut SRV_INSTANCE_TYPE;
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
pub type PSRV_OPEN_ECP_CONTEXT = *mut SRV_OPEN_ECP_CONTEXT;
#[cfg(all(feature = "ntddk", feature = "wdm", feature = "winnt"))]
pub type PTUNNEL = *mut TUNNEL;
#[repr(C)]
#[cfg(all(feature = "ntdef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct PUBLIC_BCB {
    pub NodeTypeCode: super::CSHORT,
    pub NodeByteSize: super::CSHORT,
    pub MappedLength: u32,
    pub MappedFileOffset: super::LARGE_INTEGER,
}
#[cfg(all(feature = "ntdef", feature = "winnt"))]
impl Default for PUBLIC_BCB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "lsalookup", feature = "ntddk", feature = "ntdef", feature = "ntsecapi"))]
pub type PUNICODE_PREFIX_TABLE = *mut UNICODE_PREFIX_TABLE;
#[cfg(all(feature = "lsalookup", feature = "ntddk", feature = "ntdef", feature = "ntsecapi"))]
pub type PUNICODE_PREFIX_TABLE_ENTRY = *mut UNICODE_PREFIX_TABLE_ENTRY;
#[cfg(all(feature = "basetsd", feature = "lsalookup", feature = "ntdef", feature = "ntsecapi", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
pub type PUNLOCK_ROUTINE = Option<unsafe extern "C" fn(context: *const core::ffi::c_void, filelockinfo: PFILE_LOCK_INFO)>;
pub const PURGE_WITH_ACTIVE_VIEWS: i32 = 8;
pub type PVBN = *mut VBN;
#[cfg(feature = "winnt")]
pub type PVCN_RANGE_INPUT_BUFFER = *mut VCN_RANGE_INPUT_BUFFER;
#[cfg(feature = "winnt")]
pub type PVETO_BINDING_ECP_CONTEXT = *mut VETO_BINDING_ECP_CONTEXT;
#[cfg(feature = "winnt")]
pub type PVOLUME_REFS_INFO_BUFFER = *mut VOLUME_REFS_INFO_BUFFER;
pub const QUERY_DIRECT_ACCESS_DATA_EXTENTS: i32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct QUERY_DIRECT_ACCESS_EXTENTS {
    pub FileOffset: i64,
    pub Length: i64,
    pub Flags: u32,
    pub Reserved: u32,
}
pub const QUERY_DIRECT_ACCESS_IMAGE_EXTENTS: i32 = 1;
#[repr(C)]
#[cfg(feature = "wdm")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct QUERY_ON_CREATE_EA_INFORMATION {
    pub EaBufferSize: u32,
    pub EaBuffer: super::PFILE_FULL_EA_INFORMATION,
}
#[repr(C)]
#[cfg(all(feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct QUERY_ON_CREATE_ECP_CONTEXT {
    pub RequestedClasses: u32,
    pub ClassesProcessed: u32,
    pub ClassesWithErrors: u32,
    pub ClassesWithNoData: u32,
    pub StatInformation: QUERY_ON_CREATE_FILE_STAT_INFORMATION,
    pub LxInformation: QUERY_ON_CREATE_FILE_LX_INFORMATION,
    pub EaInformation: QUERY_ON_CREATE_EA_INFORMATION,
    pub Reserved: u32,
    pub CommonBufferSize: u32,
    pub CommonBuffer: *mut core::ffi::c_void,
    pub UsnInformation: QUERY_ON_CREATE_USN_INFORMATION,
    pub SecurityInformationRequested: super::SECURITY_INFORMATION,
    pub SecurityInformation: QUERY_ON_CREATE_SECURITY_INFORMATION,
}
#[cfg(all(feature = "wdm", feature = "winnt"))]
impl Default for QUERY_ON_CREATE_ECP_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
pub const QUERY_ON_CREATE_ECP_CONTEXT_COMMON_BUFFER_END: u32 = 136;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const QUERY_ON_CREATE_ECP_CONTEXT_COMMON_BUFFER_END: u32 = 152;
#[cfg(target_arch = "x86")]
pub const QUERY_ON_CREATE_ECP_CONTEXT_EA_INFO_END: u32 = 124;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const QUERY_ON_CREATE_ECP_CONTEXT_EA_INFO_END: u32 = 136;
pub const QUERY_ON_CREATE_ECP_CONTEXT_LX_INFO_END: u32 = 116;
#[cfg(target_arch = "x86")]
pub const QUERY_ON_CREATE_ECP_CONTEXT_SECURITY_INFO_END: u32 = 176;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const QUERY_ON_CREATE_ECP_CONTEXT_SECURITY_INFO_END: u32 = 200;
pub const QUERY_ON_CREATE_ECP_CONTEXT_STAT_INFO_END: u32 = 88;
#[cfg(target_arch = "x86")]
pub const QUERY_ON_CREATE_ECP_CONTEXT_USN_INFO_END: u32 = 160;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const QUERY_ON_CREATE_ECP_CONTEXT_USN_INFO_END: u32 = 176;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct QUERY_ON_CREATE_FILE_LX_INFORMATION {
    pub EffectiveAccess: super::ACCESS_MASK,
    pub LxFlags: u32,
    pub LxUid: u32,
    pub LxGid: u32,
    pub LxMode: u32,
    pub LxDeviceIdMajor: u32,
    pub LxDeviceIdMinor: u32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct QUERY_ON_CREATE_FILE_STAT_INFORMATION {
    pub FileId: super::LARGE_INTEGER,
    pub CreationTime: super::LARGE_INTEGER,
    pub LastAccessTime: super::LARGE_INTEGER,
    pub LastWriteTime: super::LARGE_INTEGER,
    pub ChangeTime: super::LARGE_INTEGER,
    pub AllocationSize: super::LARGE_INTEGER,
    pub EndOfFile: super::LARGE_INTEGER,
    pub FileAttributes: u32,
    pub ReparseTag: u32,
    pub NumberOfLinks: u32,
}
#[cfg(feature = "winnt")]
impl Default for QUERY_ON_CREATE_FILE_STAT_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct QUERY_ON_CREATE_SECURITY_INFORMATION {
    pub Reserved: u32,
    pub SecurityDescriptorSize: u32,
    pub SecurityDescriptor: super::PSECURITY_DESCRIPTOR,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct QUERY_ON_CREATE_USN_INFORMATION {
    pub Usn: super::USN,
    pub FileReferenceNumber: super::FILE_ID_128,
}
#[repr(C)]
#[cfg(all(feature = "wdm", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct QUERY_PATH_REQUEST {
    pub PathNameLength: u32,
    pub SecurityContext: super::PIO_SECURITY_CONTEXT,
    pub FilePathName: [u16; 1],
}
#[cfg(all(feature = "wdm", feature = "winnt", feature = "winternl"))]
impl Default for QUERY_PATH_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ntddk", feature = "wdm", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct QUERY_PATH_REQUEST_EX {
    pub pSecurityContext: super::PIO_SECURITY_CONTEXT,
    pub EaLength: u32,
    pub pEaBuffer: *mut core::ffi::c_void,
    pub PathName: super::UNICODE_STRING,
    pub DomainServiceName: super::UNICODE_STRING,
    pub EcpList: PECP_LIST,
    pub Silo: super::PESILO,
    pub Reserved: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct QUERY_PATH_RESPONSE {
    pub LengthAccepted: u32,
}
#[cfg(all(feature = "basetsd", feature = "winnt"))]
pub type QUERY_VIRTUAL_MEMORY_CALLBACK = Option<unsafe extern "C" fn(callbackcontext: super::HANDLE, processhandle: super::HANDLE, baseaddress: *const core::ffi::c_void, memoryinformationclass: HEAP_MEMORY_INFO_CLASS, memoryinformation: *mut core::ffi::c_void, memoryinformationlength: usize, returnlength: super::PSIZE_T) -> windows_core::NTSTATUS>;
pub const QoCFileEaInformation: i32 = 4;
pub const QoCFileLxInformation: i32 = 2;
pub const QoCFileSecurityInformation: i32 = 16;
pub const QoCFileStatInformation: i32 = 1;
pub const QoCFileUsnInformation: i32 = 8;
#[repr(C)]
#[cfg(feature = "ntdef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct READ_AHEAD_PARAMETERS {
    pub NodeByteSize: super::CSHORT,
    pub Granularity: u32,
    pub PipelinedRequestSize: u32,
    pub ReadAheadGrowthPercentage: u32,
}
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "lsalookup", feature = "ntdef", feature = "ntsecapi", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy)]
pub struct READ_LIST {
    pub FileObject: super::PFILE_OBJECT,
    pub NumberOfEntries: u32,
    pub IsImage: super::LOGICAL,
    pub List: [super::FILE_SEGMENT_ELEMENT; 1],
}
#[cfg(all(feature = "basetsd", feature = "lsalookup", feature = "ntdef", feature = "ntsecapi", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
impl Default for READ_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type REFS_COMPRESSION_FORMATS = i32;
pub const REFS_COMPRESSION_FORMAT_LZ4: REFS_COMPRESSION_FORMATS = 3;
pub const REFS_COMPRESSION_FORMAT_UNCHANGED: REFS_COMPRESSION_FORMATS = 0;
pub const REFS_COMPRESSION_FORMAT_UNCOMPRESSED: REFS_COMPRESSION_FORMATS = 2;
pub const REFS_COMPRESSION_FORMAT_UNKNOWN: REFS_COMPRESSION_FORMATS = 1;
pub const REFS_COMPRESSION_FORMAT_ZSTD: REFS_COMPRESSION_FORMATS = 4;
pub type REFS_DEALLOCATE_RANGES_ALLOCATOR = i32;
pub const REFS_DEALLOCATE_RANGES_ALLOCATOR_CAA: REFS_DEALLOCATE_RANGES_ALLOCATOR = 2;
pub const REFS_DEALLOCATE_RANGES_ALLOCATOR_MAA: REFS_DEALLOCATE_RANGES_ALLOCATOR = 3;
pub const REFS_DEALLOCATE_RANGES_ALLOCATOR_NONE: REFS_DEALLOCATE_RANGES_ALLOCATOR = 0;
pub const REFS_DEALLOCATE_RANGES_ALLOCATOR_SAA: REFS_DEALLOCATE_RANGES_ALLOCATOR = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct REFS_DEALLOCATE_RANGES_INPUT_BUFFER {
    pub RangeCount: u32,
    pub Ranges: [REFS_DEALLOCATE_RANGES_RANGE; 1],
}
impl Default for REFS_DEALLOCATE_RANGES_INPUT_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct REFS_DEALLOCATE_RANGES_INPUT_BUFFER_EX {
    pub RangeCount: u32,
    pub Allocator: REFS_DEALLOCATE_RANGES_ALLOCATOR,
    pub StreamReserveUpdateCount: i64,
    pub OffsetToRanges: u32,
    pub OffsetToLeakCounts: u32,
    pub Reserved: [u64; 2],
}
impl Default for REFS_DEALLOCATE_RANGES_INPUT_BUFFER_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct REFS_DEALLOCATE_RANGES_RANGE {
    pub StartOfRange: u64,
    pub CountOfRange: u64,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct REFS_QUERY_ROLLBACK_PROTECTION_INFO_OUTPUT_BUFFER {
    pub Version: u32,
    pub VolumeGuid: windows_core::GUID,
    pub RollbackProtectionGuid: windows_core::GUID,
    pub FailMountOnMismatch: super::BOOLEAN,
    pub FrozenVirtualClock: u64,
    pub CurrentVirtualClock: u64,
    pub ChecksumType: u16,
    pub ChecksumLength: u32,
    pub ChecksumOffset: u32,
    pub CustomPayloadLength: u32,
    pub CustomPayloadOffset: u32,
}
pub const REFS_QUERY_ROLLBACK_PROTECTION_INFO_OUTPUT_BUFFER_VERSION: i32 = 1;
pub type REFS_QUERY_VOLUME_COMPRESSION_INFO_FLAGS = i32;
pub const REFS_QUERY_VOLUME_COMPRESSION_INFO_FLAGS_RUNNING: REFS_QUERY_VOLUME_COMPRESSION_INFO_FLAGS = 1;
pub const REFS_QUERY_VOLUME_COMPRESSION_INFO_FLAGS_STOPPED: REFS_QUERY_VOLUME_COMPRESSION_INFO_FLAGS = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct REFS_QUERY_VOLUME_COMPRESSION_INFO_OUTPUT_BUFFER {
    pub Version: u32,
    pub DefaultCompressionFormat: REFS_COMPRESSION_FORMATS,
    pub DefaultCompressionLevel: i16,
    pub DefaultCompressionChunkSizeBytes: u32,
    pub VolumeClusterSizeBytes: u32,
    pub TotalVolumeClusters: u64,
    pub TotalAllocatedClusters: u64,
    pub TotalCompressibleClustersAllocated: u64,
    pub TotalCompressibleClustersInUse: u64,
    pub TotalCompressedClusters: u64,
    pub Flags: u32,
    pub CompressionTuning: u32,
    pub RecompressionTuning: u32,
    pub DecompressionTuning: u32,
    pub LastCompressionStatus: windows_core::NTSTATUS,
    pub Reserved: [u32; 8],
}
impl Default for REFS_QUERY_VOLUME_COMPRESSION_INFO_OUTPUT_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const REFS_QUERY_VOLUME_COMPRESSION_INFO_OUTPUT_BUFFER_VERSION: i32 = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct REFS_QUERY_VOLUME_IO_METRICS_INFO_INPUT_BUFFER {
    pub Version: u32,
    pub QueryType: REFS_QUERY_VOLUME_IO_METRICS_INFO_QUERY_TYPE,
    pub Reserved: [u32; 6],
    pub Anonymous: REFS_QUERY_VOLUME_IO_METRICS_INFO_INPUT_BUFFER_0,
}
impl Default for REFS_QUERY_VOLUME_IO_METRICS_INFO_INPUT_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union REFS_QUERY_VOLUME_IO_METRICS_INFO_INPUT_BUFFER_0 {
    pub UnusedAlign: u64,
    pub Parameters: REFS_QUERY_VOLUME_IO_METRICS_INFO_INPUT_BUFFER_0_0,
    pub MetricsData: REFS_QUERY_VOLUME_IO_METRICS_INFO_INPUT_BUFFER_0_1,
}
impl Default for REFS_QUERY_VOLUME_IO_METRICS_INFO_INPUT_BUFFER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct REFS_QUERY_VOLUME_IO_METRICS_INFO_INPUT_BUFFER_0_0 {
    pub Reserved: [u32; 6],
}
impl Default for REFS_QUERY_VOLUME_IO_METRICS_INFO_INPUT_BUFFER_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct REFS_QUERY_VOLUME_IO_METRICS_INFO_INPUT_BUFFER_0_1 {
    pub ResumeKeyBlob: [u64; 2],
    pub Reserved: [u32; 6],
}
impl Default for REFS_QUERY_VOLUME_IO_METRICS_INFO_INPUT_BUFFER_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const REFS_QUERY_VOLUME_IO_METRICS_INFO_INPUT_BUFFER_VERSION: i32 = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct REFS_QUERY_VOLUME_IO_METRICS_INFO_OUTPUT_BUFFER {
    pub Version: u32,
    pub QueryType: REFS_QUERY_VOLUME_IO_METRICS_INFO_QUERY_TYPE,
    pub Reserved: [u32; 6],
    pub Anonymous: REFS_QUERY_VOLUME_IO_METRICS_INFO_OUTPUT_BUFFER_0,
}
impl Default for REFS_QUERY_VOLUME_IO_METRICS_INFO_OUTPUT_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union REFS_QUERY_VOLUME_IO_METRICS_INFO_OUTPUT_BUFFER_0 {
    pub UnusedAlign: u64,
    pub Parameters: REFS_QUERY_VOLUME_IO_METRICS_INFO_OUTPUT_BUFFER_0_0,
    pub MetricsData: REFS_QUERY_VOLUME_IO_METRICS_INFO_OUTPUT_BUFFER_0_1,
}
impl Default for REFS_QUERY_VOLUME_IO_METRICS_INFO_OUTPUT_BUFFER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct REFS_QUERY_VOLUME_IO_METRICS_INFO_OUTPUT_BUFFER_0_0 {
    pub GlobalSecondsToTrack: u32,
    pub MetricsPeriodicitySeconds: u32,
    pub MetricsGenerationsPerContainer: u32,
    pub Reserved: [u32; 6],
}
impl Default for REFS_QUERY_VOLUME_IO_METRICS_INFO_OUTPUT_BUFFER_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct REFS_QUERY_VOLUME_IO_METRICS_INFO_OUTPUT_BUFFER_0_1 {
    pub EntryCount: u32,
    pub ResumeKeyBlob: [u64; 2],
    pub Reserved: [u32; 6],
    pub Metrics: [REFS_QUERY_VOLUME_IO_METRICS_METRICS_DATA; 1],
}
impl Default for REFS_QUERY_VOLUME_IO_METRICS_INFO_OUTPUT_BUFFER_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const REFS_QUERY_VOLUME_IO_METRICS_INFO_OUTPUT_BUFFER_VERSION: i32 = 1;
pub type REFS_QUERY_VOLUME_IO_METRICS_INFO_QUERY_TYPE = i32;
pub const REFS_QUERY_VOLUME_IO_METRICS_INFO_QUERY_TYPE_METRICS_DATA: REFS_QUERY_VOLUME_IO_METRICS_INFO_QUERY_TYPE = 2;
pub const REFS_QUERY_VOLUME_IO_METRICS_INFO_QUERY_TYPE_PARAMETERS: REFS_QUERY_VOLUME_IO_METRICS_INFO_QUERY_TYPE = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct REFS_QUERY_VOLUME_IO_METRICS_METRICS_DATA {
    pub PlaceHolder: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct REFS_QUERY_VOLUME_TOTAL_SHARED_LCNS_OUTPUT_BUFFER {
    pub Version: u32,
    pub TotalSharedLcns: u64,
}
pub const REFS_QUERY_VOLUME_TOTAL_SHARED_LCNS_OUTPUT_BUFFER_VERSION: i32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct REFS_REMOVE_HARDLINK_BACKPOINTER {
    pub ParentDirectory: u64,
    pub Reserved: u64,
    pub FileName: [u16; 1],
}
impl Default for REFS_REMOVE_HARDLINK_BACKPOINTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct REFS_SET_ROLLBACK_PROTECTION_INFO_INPUT_BUFFER {
    pub Version: u32,
    pub FailMountOnMismatch: super::BOOLEAN,
    pub CustomPayloadLength: u32,
    pub CustomPayloadOffset: u32,
    pub EnableRollbackProtection: super::BOOLEAN,
}
pub const REFS_SET_ROLLBACK_PROTECTION_INFO_INPUT_BUFFER_VERSION: i32 = 2;
pub const REFS_SET_ROLLBACK_PROTECTION_INFO_INPUT_BUFFER_VERSION_V1: i32 = 1;
pub type REFS_SET_VOLUME_COMPRESSION_INFO_FLAGS = i32;
pub const REFS_SET_VOLUME_COMPRESSION_INFO_FLAG_DISABLE_COMPRESSION: REFS_SET_VOLUME_COMPRESSION_INFO_FLAGS = 16;
pub const REFS_SET_VOLUME_COMPRESSION_INFO_FLAG_ENABLE_COMPRESSION: REFS_SET_VOLUME_COMPRESSION_INFO_FLAGS = 8;
pub const REFS_SET_VOLUME_COMPRESSION_INFO_FLAG_GC_ONLY: REFS_SET_VOLUME_COMPRESSION_INFO_FLAGS = 4;
pub const REFS_SET_VOLUME_COMPRESSION_INFO_FLAG_MAX: REFS_SET_VOLUME_COMPRESSION_INFO_FLAGS = 16;
pub const REFS_SET_VOLUME_COMPRESSION_INFO_FLAG_START_COMPRESSION: REFS_SET_VOLUME_COMPRESSION_INFO_FLAGS = 1;
pub const REFS_SET_VOLUME_COMPRESSION_INFO_FLAG_STOP_COMPRESSION: REFS_SET_VOLUME_COMPRESSION_INFO_FLAGS = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct REFS_SET_VOLUME_COMPRESSION_INFO_INPUT_BUFFER {
    pub Version: u32,
    pub CompressionFormat: REFS_COMPRESSION_FORMATS,
    pub CompressionLevel: i16,
    pub CompressionChunkSizeBytes: u32,
    pub Flags: u32,
    pub CompressionTuning: u32,
    pub RecompressionTuning: u32,
    pub DecompressionTuning: u32,
    pub Reserved: [u32; 6],
}
impl Default for REFS_SET_VOLUME_COMPRESSION_INFO_INPUT_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const REFS_SET_VOLUME_COMPRESSION_INFO_INPUT_BUFFER_VERSION: i32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct REFS_SET_VOLUME_IO_METRICS_INFO_INPUT_BUFFER {
    pub Version: u32,
    pub GlobalSecondsToTrack: u32,
    pub MetricsPeriodicitySeconds: u32,
    pub MetricsGenerationsPerContainer: u32,
    pub Reserved: [u32; 8],
}
impl Default for REFS_SET_VOLUME_IO_METRICS_INFO_INPUT_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const REFS_SET_VOLUME_IO_METRICS_INFO_INPUT_BUFFER_VERSION: i32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct REFS_STREAM_EXTENT {
    pub Vcn: i64,
    pub Lcn: i64,
    pub Length: i64,
    pub Properties: REFS_STREAM_EXTENT_PROPERTIES,
}
pub type REFS_STREAM_EXTENT_PROPERTIES = u16;
pub const REFS_STREAM_EXTENT_PROPERTY_CRC32: _REFS_STREAM_EXTENT_PROPERTIES = 128;
pub const REFS_STREAM_EXTENT_PROPERTY_CRC64: _REFS_STREAM_EXTENT_PROPERTIES = 256;
pub const REFS_STREAM_EXTENT_PROPERTY_GHOSTED: _REFS_STREAM_EXTENT_PROPERTIES = 512;
pub const REFS_STREAM_EXTENT_PROPERTY_READONLY: _REFS_STREAM_EXTENT_PROPERTIES = 1024;
pub const REFS_STREAM_EXTENT_PROPERTY_SPARSE: _REFS_STREAM_EXTENT_PROPERTIES = 8;
pub const REFS_STREAM_EXTENT_PROPERTY_STREAM_RESERVED: _REFS_STREAM_EXTENT_PROPERTIES = 32;
pub const REFS_STREAM_EXTENT_PROPERTY_VALID: _REFS_STREAM_EXTENT_PROPERTIES = 16;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct REFS_STREAM_SNAPSHOT_LIST_OUTPUT_BUFFER {
    pub EntryCount: u32,
    pub BufferSizeRequiredForQuery: u32,
    pub Reserved: [u32; 2],
    pub Entries: [REFS_STREAM_SNAPSHOT_LIST_OUTPUT_BUFFER_ENTRY; 1],
}
impl Default for REFS_STREAM_SNAPSHOT_LIST_OUTPUT_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct REFS_STREAM_SNAPSHOT_LIST_OUTPUT_BUFFER_ENTRY {
    pub NextEntryOffset: u32,
    pub SnapshotNameLength: u16,
    pub SnapshotCreationTime: u64,
    pub StreamSize: u64,
    pub StreamAllocationSize: u64,
    pub Reserved: [u64; 2],
    pub SnapshotName: [u16; 1],
}
impl Default for REFS_STREAM_SNAPSHOT_LIST_OUTPUT_BUFFER_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct REFS_STREAM_SNAPSHOT_MANAGEMENT_INPUT_BUFFER {
    pub Operation: REFS_STREAM_SNAPSHOT_OPERATION,
    pub SnapshotNameLength: u16,
    pub OperationInputBufferLength: u16,
    pub Reserved: [u64; 2],
    pub NameAndInputBuffer: [u16; 1],
}
impl Default for REFS_STREAM_SNAPSHOT_MANAGEMENT_INPUT_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type REFS_STREAM_SNAPSHOT_OPERATION = i32;
pub const REFS_STREAM_SNAPSHOT_OPERATION_CLEAR_SHADOW_BTREE: REFS_STREAM_SNAPSHOT_OPERATION = 6;
pub const REFS_STREAM_SNAPSHOT_OPERATION_CREATE: REFS_STREAM_SNAPSHOT_OPERATION = 1;
pub const REFS_STREAM_SNAPSHOT_OPERATION_INVALID: REFS_STREAM_SNAPSHOT_OPERATION = 0;
pub const REFS_STREAM_SNAPSHOT_OPERATION_LIST: REFS_STREAM_SNAPSHOT_OPERATION = 2;
pub const REFS_STREAM_SNAPSHOT_OPERATION_MAX: REFS_STREAM_SNAPSHOT_OPERATION = 6;
pub const REFS_STREAM_SNAPSHOT_OPERATION_QUERY_DELTAS: REFS_STREAM_SNAPSHOT_OPERATION = 3;
pub const REFS_STREAM_SNAPSHOT_OPERATION_REVERT: REFS_STREAM_SNAPSHOT_OPERATION = 4;
pub const REFS_STREAM_SNAPSHOT_OPERATION_SET_SHADOW_BTREE: REFS_STREAM_SNAPSHOT_OPERATION = 5;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct REFS_STREAM_SNAPSHOT_QUERY_DELTAS_INPUT_BUFFER {
    pub StartingVcn: i64,
    pub Flags: u32,
    pub Reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct REFS_STREAM_SNAPSHOT_QUERY_DELTAS_OUTPUT_BUFFER {
    pub ExtentCount: u32,
    pub Reserved: [u32; 2],
    pub Extents: [REFS_STREAM_EXTENT; 1],
}
impl Default for REFS_STREAM_SNAPSHOT_QUERY_DELTAS_OUTPUT_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct REFS_VOLUME_COUNTER_INFO_INPUT_BUFFER {
    pub ResetCounters: super::BOOLEAN,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct REFS_VOLUME_DEDUP_INFO_INPUT_BUFFER {
    pub Version: u32,
    pub SetDedupState: super::BOOLEAN,
    pub Enable: super::BOOLEAN,
    pub SetWeakRefState: super::BOOLEAN,
    pub EnableWeakRef: super::BOOLEAN,
    pub SetDirtyRangeTrackingState: super::BOOLEAN,
    pub EnableDirtyRangeTracking: super::BOOLEAN,
    pub SetWeakRefInconsistentState: super::BOOLEAN,
    pub SetWeakRefInconsistent: super::BOOLEAN,
}
pub const REFS_VOLUME_DEDUP_INFO_INPUT_BUFFER_VERSION: i32 = 2;
pub const REFS_VOLUME_DEDUP_INFO_INPUT_BUFFER_VERSION_V1: i32 = 1;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct REFS_VOLUME_DEDUP_INFO_OUTPUT_BUFFER {
    pub Version: u32,
    pub Enabled: super::BOOLEAN,
    pub EnabledWeakRef: super::BOOLEAN,
    pub EnabledDirtyRangeTracking: super::BOOLEAN,
    pub WeakRefInconsistent: super::BOOLEAN,
    pub IsClustered: super::BOOLEAN,
    pub VolumeIdHash: u32,
    pub VolumeGuid: windows_core::GUID,
    pub VolumeUniqueGuid: windows_core::GUID,
}
pub const REFS_VOLUME_DEDUP_INFO_OUTPUT_BUFFER_VERSION: i32 = 2;
pub const REFS_VOLUME_DEDUP_INFO_OUTPUT_BUFFER_VERSION_V1: i32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct REMOTE_LINK_TRACKING_INFORMATION {
    pub TargetFileObject: *mut core::ffi::c_void,
    pub TargetLinkTrackingInformationLength: u32,
    pub TargetLinkTrackingInformationBuffer: [u8; 1],
}
impl Default for REMOTE_LINK_TRACKING_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const REMOTE_PROTOCOL_FLAG_INTEGRITY: i32 = 16;
pub const REMOTE_PROTOCOL_FLAG_LOOPBACK: i32 = 1;
pub const REMOTE_PROTOCOL_FLAG_MUTUAL_AUTH: i32 = 32;
pub const REMOTE_PROTOCOL_FLAG_OFFLINE: i32 = 2;
pub const REMOTE_PROTOCOL_FLAG_PERSISTENT_HANDLE: i32 = 4;
pub const REMOTE_PROTOCOL_FLAG_PRIVACY: i32 = 8;
pub const REMOVED_8DOT3_NAME: i32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct REPARSE_DATA_BUFFER {
    pub ReparseTag: u32,
    pub ReparseDataLength: u16,
    pub Reserved: u16,
    pub Anonymous: REPARSE_DATA_BUFFER_0,
}
impl Default for REPARSE_DATA_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union REPARSE_DATA_BUFFER_0 {
    pub SymbolicLinkReparseBuffer: REPARSE_DATA_BUFFER_0_0,
    pub MountPointReparseBuffer: REPARSE_DATA_BUFFER_0_1,
    pub GenericReparseBuffer: REPARSE_DATA_BUFFER_0_2,
}
impl Default for REPARSE_DATA_BUFFER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct REPARSE_DATA_BUFFER_0_0 {
    pub SubstituteNameOffset: u16,
    pub SubstituteNameLength: u16,
    pub PrintNameOffset: u16,
    pub PrintNameLength: u16,
    pub Flags: u32,
    pub PathBuffer: [u16; 1],
}
impl Default for REPARSE_DATA_BUFFER_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct REPARSE_DATA_BUFFER_0_1 {
    pub SubstituteNameOffset: u16,
    pub SubstituteNameLength: u16,
    pub PrintNameOffset: u16,
    pub PrintNameLength: u16,
    pub PathBuffer: [u16; 1],
}
impl Default for REPARSE_DATA_BUFFER_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct REPARSE_DATA_BUFFER_0_2 {
    pub DataBuffer: [u8; 1],
}
impl Default for REPARSE_DATA_BUFFER_0_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct REPARSE_DATA_BUFFER_EX {
    pub Flags: u32,
    pub ExistingReparseTag: u32,
    pub ExistingReparseGuid: windows_core::GUID,
    pub Reserved: u64,
    pub Anonymous: REPARSE_DATA_BUFFER_EX_0,
}
#[cfg(feature = "winnt")]
impl Default for REPARSE_DATA_BUFFER_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union REPARSE_DATA_BUFFER_EX_0 {
    pub ReparseDataBuffer: REPARSE_DATA_BUFFER,
    pub ReparseGuidDataBuffer: super::REPARSE_GUID_DATA_BUFFER,
}
#[cfg(feature = "winnt")]
impl Default for REPARSE_DATA_BUFFER_EX_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const REPARSE_DATA_BUFFER_EX_HEADER_SIZE: u32 = 40;
pub const REPARSE_DATA_BUFFER_HEADER_SIZE: u32 = 8;
pub const REPARSE_DATA_EX_FLAG_GIVEN_TAG_OR_NONE: i32 = 1;
pub const REPARSE_GUID_DATA_BUFFER_EX_HEADER_SIZE: u32 = 56;
#[repr(C, packed(4))]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct REPARSE_INDEX_KEY {
    pub FileReparseTag: u32,
    pub FileId: super::LARGE_INTEGER,
}
#[cfg(feature = "winnt")]
impl Default for REPARSE_INDEX_KEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RETURN_NON_NT_USER_SESSION_KEY: i32 = 8;
pub const RETURN_PRIMARY_LOGON_DOMAINNAME: i32 = 4;
pub const RETURN_PRIMARY_USERNAME: i32 = 2;
pub const RETURN_RESERVED_PARAMETER: i32 = 128;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RKF_BYPASS_ECP_CONTEXT {
    pub Reserved: i32,
    pub Version: i32,
}
pub const RPI_SMB2_SERVERCAP_DFS: i32 = 1;
pub const RPI_SMB2_SERVERCAP_DIRECTORY_LEASING: i32 = 32;
pub const RPI_SMB2_SERVERCAP_ENCRYPTION_AWARE: i32 = 64;
pub const RPI_SMB2_SERVERCAP_LARGEMTU: i32 = 4;
pub const RPI_SMB2_SERVERCAP_LEASING: i32 = 2;
pub const RPI_SMB2_SERVERCAP_MULTICHANNEL: i32 = 8;
pub const RPI_SMB2_SERVERCAP_NOTIFICATIONS_AWARE: i32 = 128;
pub const RPI_SMB2_SERVERCAP_PERSISTENT_HANDLES: i32 = 16;
pub const RPI_SMB2_SHARECAP_ACCESS_BASED_DIRECTORY_ENUM: i32 = 256;
pub const RPI_SMB2_SHARECAP_ASYMMETRIC_SCALEOUT: i32 = 1024;
pub const RPI_SMB2_SHARECAP_CLUSTER: i32 = 64;
pub const RPI_SMB2_SHARECAP_CONTINUOUS_AVAILABILITY: i32 = 16;
pub const RPI_SMB2_SHARECAP_DFS: i32 = 8;
pub const RPI_SMB2_SHARECAP_ENCRYPTED: i32 = 128;
pub const RPI_SMB2_SHARECAP_IDENTITY_REMOTING: i32 = 512;
pub const RPI_SMB2_SHARECAP_SCALEOUT: i32 = 32;
pub const RPI_SMB2_SHARECAP_TIMEWARP: i32 = 2;
pub const RPI_SMB2_SHARETYPE_DISK: i32 = 0;
pub const RPI_SMB2_SHARETYPE_PIPE: i32 = 1;
pub const RPI_SMB2_SHARETYPE_PRINT: i32 = 2;
pub type RTL_ALLOCATE_STRING_ROUTINE = Option<unsafe extern "system" fn(numberofbytes: usize) -> *mut core::ffi::c_void>;
pub const RTL_DUPLICATE_UNICODE_STRING_ALLOCATE_NULL_STRING: i32 = 2;
pub const RTL_DUPLICATE_UNICODE_STRING_NULL_TERMINATE: i32 = 1;
pub type RTL_FREE_STRING_ROUTINE = Option<unsafe extern "system" fn(buffer: *const core::ffi::c_void)>;
#[cfg(feature = "basetsd")]
pub type RTL_HEAP_COMMIT_ROUTINE = Option<unsafe extern "system" fn(base: *const core::ffi::c_void, commitaddress: *mut *mut core::ffi::c_void, commitsize: super::PSIZE_T) -> windows_core::NTSTATUS>;
pub const RTL_HEAP_MEMORY_LIMIT_CURRENT_VERSION: i32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RTL_HEAP_MEMORY_LIMIT_DATA {
    pub CommitLimitBytes: usize,
    pub CommitLimitFailureCode: usize,
    pub MaxAllocationSizeBytes: usize,
    pub AllocationLimitFailureCode: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RTL_HEAP_MEMORY_LIMIT_INFO {
    pub Version: u32,
    pub Data: RTL_HEAP_MEMORY_LIMIT_DATA,
}
#[repr(C)]
#[cfg(feature = "basetsd")]
#[derive(Clone, Copy, Debug)]
pub struct RTL_HEAP_PARAMETERS {
    pub Length: u32,
    pub SegmentReserve: usize,
    pub SegmentCommit: usize,
    pub DeCommitFreeBlockThreshold: usize,
    pub DeCommitTotalFreeThreshold: usize,
    pub MaximumAllocationSize: usize,
    pub VirtualMemoryThreshold: usize,
    pub InitialCommit: usize,
    pub InitialReserve: usize,
    pub CommitRoutine: PRTL_HEAP_COMMIT_ROUTINE,
    pub Reserved: [usize; 2],
}
#[cfg(feature = "basetsd")]
impl Default for RTL_HEAP_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type RTL_MEMORY_TYPE = i32;
pub type RTL_REALLOCATE_STRING_ROUTINE = Option<unsafe extern "system" fn(numberofbytes: usize, buffer: *const core::ffi::c_void) -> *mut core::ffi::c_void>;
pub const RTL_SEGHEAP_MEM_SOURCE_ANY_NODE: u32 = 4294967295;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct RTL_SEGMENT_HEAP_MEMORY_SOURCE {
    pub Flags: u32,
    pub MemoryTypeMask: u32,
    pub NumaNode: u32,
    pub Anonymous: RTL_SEGMENT_HEAP_MEMORY_SOURCE_0,
    pub Reserved: [usize; 2],
}
#[cfg(feature = "winnt")]
impl Default for RTL_SEGMENT_HEAP_MEMORY_SOURCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union RTL_SEGMENT_HEAP_MEMORY_SOURCE_0 {
    pub PartitionHandle: super::HANDLE,
    pub Callbacks: *mut RTL_SEGMENT_HEAP_VA_CALLBACKS,
}
#[cfg(feature = "winnt")]
impl Default for RTL_SEGMENT_HEAP_MEMORY_SOURCE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct RTL_SEGMENT_HEAP_PARAMETERS {
    pub Version: u16,
    pub Size: u16,
    pub Flags: u32,
    pub MemorySource: RTL_SEGMENT_HEAP_MEMORY_SOURCE,
    pub Reserved: [usize; 4],
}
#[cfg(feature = "winnt")]
impl Default for RTL_SEGMENT_HEAP_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RTL_SEGMENT_HEAP_VA_CALLBACKS {
    pub CallbackContext: super::HANDLE,
    pub AllocateVirtualMemory: PALLOCATE_VIRTUAL_MEMORY_EX_CALLBACK,
    pub FreeVirtualMemory: PFREE_VIRTUAL_MEMORY_EX_CALLBACK,
    pub QueryVirtualMemory: PQUERY_VIRTUAL_MEMORY_CALLBACK,
}
pub const RTL_SYSTEM_VOLUME_INFORMATION_FOLDER: windows_core::PCWSTR = windows_core::w!("System Volume Information");
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SECURITY_CLIENT_CONTEXT {
    pub SecurityQos: super::SECURITY_QUALITY_OF_SERVICE,
    pub ClientToken: super::PACCESS_TOKEN,
    pub DirectlyAccessClientToken: super::BOOLEAN,
    pub DirectAccessEffectiveOnly: super::BOOLEAN,
    pub ServerIsRemote: super::BOOLEAN,
    pub ClientTokenControl: super::TOKEN_CONTROL,
}
pub const SECURITY_DESCRIPTOR_DO_NOT_FREE: i32 = 67108864;
pub const SEGMENT_HEAP_FLG_NO_LFH: i32 = 2;
pub const SEGMENT_HEAP_FLG_USE_PAGE_HEAP: i32 = 1;
pub const SEGMENT_HEAP_PARAMETERS_VERSION: i32 = 3;
pub const SEGMENT_HEAP_PARAMS_VALID_FLAGS: i32 = 3;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SET_CACHED_RUNS_STATE_INPUT_BUFFER {
    pub Enable: super::BOOLEAN,
}
#[repr(C)]
#[cfg(all(feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SE_AUDIT_INFO {
    pub Size: u32,
    pub AuditType: super::AUDIT_EVENT_TYPE,
    pub AuditOperation: SE_AUDIT_OPERATION,
    pub AuditFlags: u32,
    pub SubsystemName: super::UNICODE_STRING,
    pub ObjectTypeName: super::UNICODE_STRING,
    pub ObjectName: super::UNICODE_STRING,
    pub HandleId: *mut core::ffi::c_void,
    pub TransactionId: *mut windows_core::GUID,
    pub OperationId: *mut super::LUID,
    pub ObjectCreation: super::BOOLEAN,
    pub GenerateOnClose: super::BOOLEAN,
}
pub type SE_AUDIT_OPERATION = i32;
pub const SE_BACKUP_PRIVILEGES_CHECKED: i32 = 256;
pub const SE_DACL_UNTRUSTED: i32 = 64;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SE_EXPORTS {
    pub SeCreateTokenPrivilege: super::LUID,
    pub SeAssignPrimaryTokenPrivilege: super::LUID,
    pub SeLockMemoryPrivilege: super::LUID,
    pub SeIncreaseQuotaPrivilege: super::LUID,
    pub SeUnsolicitedInputPrivilege: super::LUID,
    pub SeTcbPrivilege: super::LUID,
    pub SeSecurityPrivilege: super::LUID,
    pub SeTakeOwnershipPrivilege: super::LUID,
    pub SeLoadDriverPrivilege: super::LUID,
    pub SeCreatePagefilePrivilege: super::LUID,
    pub SeIncreaseBasePriorityPrivilege: super::LUID,
    pub SeSystemProfilePrivilege: super::LUID,
    pub SeSystemtimePrivilege: super::LUID,
    pub SeProfileSingleProcessPrivilege: super::LUID,
    pub SeCreatePermanentPrivilege: super::LUID,
    pub SeBackupPrivilege: super::LUID,
    pub SeRestorePrivilege: super::LUID,
    pub SeShutdownPrivilege: super::LUID,
    pub SeDebugPrivilege: super::LUID,
    pub SeAuditPrivilege: super::LUID,
    pub SeSystemEnvironmentPrivilege: super::LUID,
    pub SeChangeNotifyPrivilege: super::LUID,
    pub SeRemoteShutdownPrivilege: super::LUID,
    pub SeNullSid: super::PSID,
    pub SeWorldSid: super::PSID,
    pub SeLocalSid: super::PSID,
    pub SeCreatorOwnerSid: super::PSID,
    pub SeCreatorGroupSid: super::PSID,
    pub SeNtAuthoritySid: super::PSID,
    pub SeDialupSid: super::PSID,
    pub SeNetworkSid: super::PSID,
    pub SeBatchSid: super::PSID,
    pub SeInteractiveSid: super::PSID,
    pub SeLocalSystemSid: super::PSID,
    pub SeAliasAdminsSid: super::PSID,
    pub SeAliasUsersSid: super::PSID,
    pub SeAliasGuestsSid: super::PSID,
    pub SeAliasPowerUsersSid: super::PSID,
    pub SeAliasAccountOpsSid: super::PSID,
    pub SeAliasSystemOpsSid: super::PSID,
    pub SeAliasPrintOpsSid: super::PSID,
    pub SeAliasBackupOpsSid: super::PSID,
    pub SeAuthenticatedUsersSid: super::PSID,
    pub SeRestrictedSid: super::PSID,
    pub SeAnonymousLogonSid: super::PSID,
    pub SeUndockPrivilege: super::LUID,
    pub SeSyncAgentPrivilege: super::LUID,
    pub SeEnableDelegationPrivilege: super::LUID,
    pub SeLocalServiceSid: super::PSID,
    pub SeNetworkServiceSid: super::PSID,
    pub SeManageVolumePrivilege: super::LUID,
    pub SeImpersonatePrivilege: super::LUID,
    pub SeCreateGlobalPrivilege: super::LUID,
    pub SeTrustedCredManAccessPrivilege: super::LUID,
    pub SeRelabelPrivilege: super::LUID,
    pub SeIncreaseWorkingSetPrivilege: super::LUID,
    pub SeTimeZonePrivilege: super::LUID,
    pub SeCreateSymbolicLinkPrivilege: super::LUID,
    pub SeIUserSid: super::PSID,
    pub SeUntrustedMandatorySid: super::PSID,
    pub SeLowMandatorySid: super::PSID,
    pub SeMediumMandatorySid: super::PSID,
    pub SeHighMandatorySid: super::PSID,
    pub SeSystemMandatorySid: super::PSID,
    pub SeOwnerRightsSid: super::PSID,
    pub SeAllAppPackagesSid: super::PSID,
    pub SeUserModeDriversSid: super::PSID,
    pub SeProcTrustWinTcbSid: super::PSID,
    pub SeTrustedInstallerSid: super::PSID,
    pub SeDelegateSessionUserImpersonatePrivilege: super::LUID,
    pub SeAppSiloSid: super::PSID,
    pub SeAppSiloVolumeRootMinimalCapabilitySid: super::PSID,
    pub SeAppSiloProfilesRootMinimalCapabilitySid: super::PSID,
    pub SeAppSiloPromptForAccessCapabilitySid: super::PSID,
    pub SeAppSiloAccessToPublisherDirectoryCapabilitySid: super::PSID,
}
#[cfg(feature = "winnt")]
pub type SE_LOGON_SESSION_TERMINATED_ROUTINE = Option<unsafe extern "C" fn(logonid: super::PLUID) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "ntddk", feature = "winnt"))]
pub type SE_LOGON_SESSION_TERMINATED_ROUTINE_EX = Option<unsafe extern "C" fn(logonid: super::PLUID, pserversilo: super::PESILO, context: *const core::ffi::c_void) -> windows_core::NTSTATUS>;
pub const SE_SERVER_SECURITY: i32 = 128;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SOV_RANGE_CHECK_DATA {
    pub RemoveZone: super::BOOLEAN,
    pub InRange: [u64; 2],
    pub ZidForRemoval: windows_core::GUID,
    pub Reserved1: u64,
    pub Reserved2: u64,
}
#[cfg(feature = "winnt")]
impl Default for SOV_RANGE_CHECK_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SPECIAL_ENCRYPTED_OPEN: i32 = 262144;
pub type SRV_INSTANCE_TYPE = i32;
#[repr(C)]
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SRV_OPEN_ECP_CONTEXT {
    pub ShareName: super::PUNICODE_STRING,
    pub SocketAddress: PSOCKADDR_STORAGE_NFS,
    pub OplockBlockState: super::BOOLEAN,
    pub OplockAppState: super::BOOLEAN,
    pub OplockFinalState: super::BOOLEAN,
    pub Version: u16,
    pub InstanceType: SRV_INSTANCE_TYPE,
}
pub const SRV_OPEN_ECP_CONTEXT_VERSION_2: i32 = 2;
pub const SUPPORTED_FS_FEATURES_BYPASS_IO: i32 = 8;
pub const SUPPORTED_FS_FEATURES_OFFLOAD_READ: i32 = 1;
pub const SUPPORTED_FS_FEATURES_OFFLOAD_WRITE: i32 = 2;
pub const SUPPORTED_FS_FEATURES_QUERY_OPEN: i32 = 4;
pub const SUPPORTED_FS_FEATURES_VALID_MASK: i32 = 15;
pub const SYMLINK_DIRECTORY: u32 = 2147483648;
pub const SYMLINK_FILE: i32 = 1073741824;
pub const SYMLINK_FLAG_RELATIVE: i32 = 1;
pub const SYMLINK_RESERVED_MASK: u32 = 4026531840;
pub const SYSTEM_PAGE_PRIORITY_BITS: i32 = 3;
pub const SYSTEM_PAGE_PRIORITY_LEVELS: i32 = 8;
pub const SrvInstanceTypeCsv: SRV_INSTANCE_TYPE = 2;
pub const SrvInstanceTypePrimary: SRV_INSTANCE_TYPE = 1;
pub const SrvInstanceTypeSBL: SRV_INSTANCE_TYPE = 3;
pub const SrvInstanceTypeSOV: SRV_INSTANCE_TYPE = 6;
pub const SrvInstanceTypeSR: SRV_INSTANCE_TYPE = 4;
pub const SrvInstanceTypeUndefined: SRV_INSTANCE_TYPE = 0;
pub const SrvInstanceTypeVMLM: SRV_INSTANCE_TYPE = 7;
pub const SrvInstanceTypeVSMB: SRV_INSTANCE_TYPE = 5;
pub type SspiAsyncContext = _SspiAsyncContext;
pub type SspiAsyncNotifyCallback = Option<unsafe extern "C" fn(handle: *const SspiAsyncContext, callbackdata: *const core::ffi::c_void)>;
pub const SyncTypeCreateSection: FS_FILTER_SECTION_SYNC_TYPE = 1;
pub const SyncTypeOther: FS_FILTER_SECTION_SYNC_TYPE = 0;
pub const TOKEN_AUDIT_NO_CHILD_PROCESS: i32 = 2097152;
pub const TOKEN_AUDIT_REDIRECTION_TRUST: i32 = 8388608;
pub const TOKEN_DO_NOT_USE_GLOBAL_ATTRIBS_FOR_QUERY: i32 = 131072;
pub const TOKEN_ENFORCE_REDIRECTION_TRUST: i32 = 4194304;
pub const TOKEN_HAS_BACKUP_PRIVILEGE: i32 = 2;
pub const TOKEN_HAS_IMPERSONATE_PRIVILEGE: i32 = 128;
pub const TOKEN_HAS_OWN_CLAIM_ATTRIBUTES: i32 = 32768;
pub const TOKEN_HAS_RESTORE_PRIVILEGE: i32 = 4;
pub const TOKEN_HAS_TRAVERSE_PRIVILEGE: i32 = 1;
pub const TOKEN_INHERIT_SECURITY_FLAGS: i32 = 3670016;
pub const TOKEN_IS_FILTERED: i32 = 2048;
pub const TOKEN_IS_RESTRICTED: i32 = 16;
pub const TOKEN_LEARNING_MODE_LOGGING: i32 = 16777216;
pub const TOKEN_LOWBOX: i32 = 16384;
pub const TOKEN_NOT_LOW: i32 = 8192;
pub const TOKEN_NO_CHILD_PROCESS: i32 = 524288;
pub const TOKEN_NO_CHILD_PROCESS_UNLESS_SECURE: i32 = 1048576;
pub const TOKEN_PERMISSIVE_LEARNING_MODE: i32 = 50331648;
pub const TOKEN_PRIVATE_NAMESPACE: i32 = 65536;
pub const TOKEN_REF_SYSTEM_MANAGED_ADMIN_FULL_TOKEN: i32 = 268435456;
pub const TOKEN_SANDBOX_INERT: i32 = 64;
pub const TOKEN_SESSION_NOT_REFERENCED: i32 = 32;
pub const TOKEN_SYSTEM_MANAGED_ADMIN_FULL_TOKEN: i32 = 134217728;
pub const TOKEN_UIACCESS: i32 = 4096;
pub const TOKEN_VIRTUALIZE_ALLOWED: i32 = 512;
pub const TOKEN_VIRTUALIZE_ENABLED: i32 = 1024;
pub const TOKEN_WRITE_RESTRICTED: i32 = 8;
#[repr(C)]
#[cfg(all(feature = "ntddk", feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct TUNNEL {
    pub Mutex: super::FAST_MUTEX,
    pub Cache: super::PRTL_SPLAY_LINKS,
    pub TimerQueue: super::LIST_ENTRY,
    pub NumEntries: u16,
}
#[cfg(all(feature = "ntddk", feature = "wdm", feature = "winnt"))]
impl Default for TUNNEL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "lsalookup", feature = "ntddk", feature = "ntdef", feature = "ntsecapi"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UNICODE_PREFIX_TABLE {
    pub NodeTypeCode: super::CSHORT,
    pub NameLength: super::CSHORT,
    pub NextPrefixTree: PUNICODE_PREFIX_TABLE_ENTRY,
    pub LastNextEntry: PUNICODE_PREFIX_TABLE_ENTRY,
}
#[repr(C)]
#[cfg(all(feature = "lsalookup", feature = "ntddk", feature = "ntdef", feature = "ntsecapi"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UNICODE_PREFIX_TABLE_ENTRY {
    pub NodeTypeCode: super::CSHORT,
    pub NameLength: super::CSHORT,
    pub NextPrefixTree: *mut Self,
    pub CaseMatch: *mut Self,
    pub Links: super::RTL_SPLAY_LINKS,
    pub Prefix: super::PUNICODE_STRING,
}
pub const UNINITIALIZE_CACHE_MAPS: i32 = 1;
pub const USE_PRIMARY_PASSWORD: i32 = 1;
pub const VACB_MAPPING_GRANULARITY: i32 = 262144;
pub const VACB_OFFSET_SHIFT: i32 = 18;
pub const VALID_COPY_FILE_CHUNK_FLAGS: i32 = 1;
pub type VBN = u32;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct VCN_RANGE_INPUT_BUFFER {
    pub StartingVcn: super::LARGE_INTEGER,
    pub ClusterCount: super::LARGE_INTEGER,
}
#[cfg(feature = "winnt")]
impl Default for VCN_RANGE_INPUT_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VETO_BINDING_ECP_CONTEXT {
    pub ShouldVetoBinding: super::BOOLEAN,
}
pub type VIRTUAL_MEMORY_INFORMATION_CLASS = i32;
pub const VOLSNAPCONTROLTYPE: i32 = 83;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct VOLUME_REFS_INFO_BUFFER {
    pub CacheSizeInBytes: super::LARGE_INTEGER,
    pub AllocatedCacheInBytes: super::LARGE_INTEGER,
    pub PopulatedCacheInBytes: super::LARGE_INTEGER,
    pub InErrorCacheInBytes: super::LARGE_INTEGER,
    pub MemoryUsedForCacheMetadata: super::LARGE_INTEGER,
    pub CacheLineSize: u32,
    pub CacheTransactionsOutstanding: i32,
    pub CacheLinesFree: i32,
    pub CacheLinesInError: i32,
    pub CacheHitsInBytes: super::LARGE_INTEGER,
    pub CacheMissesInBytes: super::LARGE_INTEGER,
    pub CachePopulationUpdatesInBytes: super::LARGE_INTEGER,
    pub CacheWriteThroughUpdatesInBytes: super::LARGE_INTEGER,
    pub CacheInvalidationsInBytes: super::LARGE_INTEGER,
    pub CacheOverReadsInBytes: super::LARGE_INTEGER,
    pub MetadataWrittenBytes: super::LARGE_INTEGER,
    pub CacheHitCounter: i32,
    pub CacheMissCounter: i32,
    pub CacheLineAllocationCounter: i32,
    pub CacheInvalidationsCounter: i32,
    pub CachePopulationUpdatesCounter: i32,
    pub CacheWriteThroughUpdatesCounter: i32,
    pub MaxCacheTransactionsOutstanding: i32,
    pub DataWritesReallocationCount: i64,
    pub DataInPlaceWriteCount: i64,
    pub MetadataAllocationsFastTierCount: i64,
    pub MetadataAllocationsSlowTierCount: i64,
    pub DataAllocationsFastTierCount: i64,
    pub DataAllocationsSlowTierCount: i64,
    pub DestagesSlowTierToFastTier: i64,
    pub DestagesFastTierToSlowTier: i64,
    pub SlowTierDataFillRatio: i32,
    pub FastTierDataFillRatio: i32,
    pub SlowTierMetadataFillRatio: i32,
    pub FastTierMetadataFillRatio: i32,
    pub SlowToFastDestageReadLatency: i64,
    pub SlowToFastDestageReadLatencyBase: i32,
    pub SlowToFastDestageWriteLatency: i64,
    pub SlowToFastDestageWriteLatencyBase: i32,
    pub FastToSlowDestageReadLatency: i64,
    pub FastToSlowDestageReadLatencyBase: i32,
    pub FastToSlowDestageWriteLatency: i64,
    pub FastToSlowDestageWriteLatencyBase: i32,
    pub SlowTierContainerFillRatio: i64,
    pub SlowTierContainerFillRatioBase: i32,
    pub FastTierContainerFillRatio: i64,
    pub FastTierContainerFillRatioBase: i32,
    pub Unused1: i32,
    pub Unused2: i32,
    pub Unused3: i32,
    pub Unused4: i32,
    pub TreeUpdateCount: i64,
    pub CheckpointCount: i64,
    pub LogWriteCount: i64,
    pub LogFillRatio: i32,
    pub ReadCacheInvalidationsForOverwrite: i32,
    pub ReadCacheInvalidationsForReuse: i32,
    pub ReadCacheInvalidationsGeneral: i32,
    pub ReadCacheChecksOnMount: i32,
    pub ReadCacheIssuesOnMount: i32,
    pub TrimLatency: i64,
    pub TrimLatencyBase: i32,
    pub DataCompactionCount: i64,
    pub CompactionReadLatency: i64,
    pub CompactionReadLatencyBase: i32,
    pub CompactionWriteLatency: i64,
    pub CompactionWriteLatencyBase: i32,
    pub DataInPlaceWriteClusterCount: super::LARGE_INTEGER,
    pub CompactionFailedDueToIneligibleContainer: i32,
    pub CompactionFailedDueToMaxFragmentation: i32,
    pub CompactedContainerFillRatio: i64,
    pub CompactedContainerFillRatioBase: i32,
    pub ContainerMoveRetryCount: i32,
    pub ContainerMoveFailedDueToIneligibleContainer: i32,
    pub CompactionFailureCount: i32,
    pub ContainerMoveFailureCount: i32,
    pub NumberOfDirtyMetadataPages: super::LARGE_INTEGER,
    pub NumberOfDirtyTableListEntries: i32,
    pub NumberOfDeleteQueueEntries: i32,
    pub MAAFilteredViewSize: i32,
    pub MAAFilteredViewInsertions: i32,
    pub MAAFilteredViewDeletions: i32,
    pub MAAFilteredViewCollisions: i32,
    pub MAAFilteredViewPurges: i32,
    pub MAARegionsVisitedPerAllocationSum: i64,
    pub MAARegionsVisitedPerAllocationBase: i32,
    pub MAAMaxRegionsVisitedPerAllocation: i32,
    pub TreeUpdateLatencyExclusive: i64,
    pub TreeUpdateLatencyTotal: i64,
    pub TreeUpdateLatencyBase: i32,
    pub CheckpointLatencyTreeUpdateExclusive: i64,
    pub CheckpointLatencyTreeUpdateTotal: i64,
    pub CheckpointLatencyTreeUpdateBase: i32,
    pub CheckpointLatencyTotal: i64,
    pub CheckpointLatencyTotalBase: i32,
}
#[cfg(feature = "winnt")]
impl Default for VOLUME_REFS_INFO_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const VmPrefetchInformation: VIRTUAL_MEMORY_INFORMATION_CLASS = 0;
pub const WCIFS_REDIRECTION_FLAGS_CREATE_SERVICED_FROM_LAYER: i32 = 1;
pub const WCIFS_REDIRECTION_FLAGS_CREATE_SERVICED_FROM_REGISTERED_LAYER: i32 = 4;
pub const WCIFS_REDIRECTION_FLAGS_CREATE_SERVICED_FROM_REMOTE_LAYER: i32 = 8;
pub const WCIFS_REDIRECTION_FLAGS_CREATE_SERVICED_FROM_SCRATCH: i32 = 2;
pub type _LCN_WEAK_REFERENCE_STATE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _REAL_NOTIFY_SYNC(pub u8);
pub type _REFS_STREAM_EXTENT_PROPERTIES = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _SspiAsyncContext(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct sockaddr_storage(pub u8);
