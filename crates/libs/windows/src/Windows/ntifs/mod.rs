#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn NtAccessCheckAndAuditAlarm(subsystemname: *const super::ntsecapi::UNICODE_STRING, handleid: Option<*const core::ffi::c_void>, objecttypename: *const super::ntsecapi::UNICODE_STRING, objectname: *const super::ntsecapi::UNICODE_STRING, securitydescriptor: super::winnt::PSECURITY_DESCRIPTOR, desiredaccess: super::winnt::ACCESS_MASK, genericmapping: *const super::winnt::GENERIC_MAPPING, objectcreation: bool, grantedaccess: *mut super::winnt::ACCESS_MASK, accessstatus: *mut super::bcrypt::NTSTATUS, generateonclose: *mut bool) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtAccessCheckAndAuditAlarm(subsystemname : *const super::ntsecapi::UNICODE_STRING, handleid : *const core::ffi::c_void, objecttypename : *const super::ntsecapi::UNICODE_STRING, objectname : *const super::ntsecapi::UNICODE_STRING, securitydescriptor : super::winnt::PSECURITY_DESCRIPTOR, desiredaccess : super::winnt::ACCESS_MASK, genericmapping : *const super::winnt::GENERIC_MAPPING, objectcreation : bool, grantedaccess : *mut super::winnt::ACCESS_MASK, accessstatus : *mut super::bcrypt::NTSTATUS, generateonclose : *mut bool) -> super::bcrypt::NTSTATUS);
    unsafe { NtAccessCheckAndAuditAlarm(subsystemname, handleid.unwrap_or(core::mem::zeroed()) as _, objecttypename, objectname, securitydescriptor, desiredaccess, genericmapping, objectcreation, grantedaccess as _, accessstatus as _, generateonclose as _) }
}
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn NtAccessCheckByTypeAndAuditAlarm(subsystemname: *const super::ntsecapi::UNICODE_STRING, handleid: Option<*const core::ffi::c_void>, objecttypename: *const super::ntsecapi::UNICODE_STRING, objectname: *const super::ntsecapi::UNICODE_STRING, securitydescriptor: super::winnt::PSECURITY_DESCRIPTOR, principalselfsid: Option<super::winnt::PSID>, desiredaccess: super::winnt::ACCESS_MASK, audittype: super::winnt::AUDIT_EVENT_TYPE, flags: u32, objecttypelist: Option<&[super::winnt::OBJECT_TYPE_LIST]>, genericmapping: *const super::winnt::GENERIC_MAPPING, objectcreation: bool, grantedaccess: *mut super::winnt::ACCESS_MASK, accessstatus: *mut super::bcrypt::NTSTATUS, generateonclose: *mut bool) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtAccessCheckByTypeAndAuditAlarm(subsystemname : *const super::ntsecapi::UNICODE_STRING, handleid : *const core::ffi::c_void, objecttypename : *const super::ntsecapi::UNICODE_STRING, objectname : *const super::ntsecapi::UNICODE_STRING, securitydescriptor : super::winnt::PSECURITY_DESCRIPTOR, principalselfsid : super::winnt::PSID, desiredaccess : super::winnt::ACCESS_MASK, audittype : super::winnt::AUDIT_EVENT_TYPE, flags : u32, objecttypelist : *const super::winnt::OBJECT_TYPE_LIST, objecttypelistlength : u32, genericmapping : *const super::winnt::GENERIC_MAPPING, objectcreation : bool, grantedaccess : *mut super::winnt::ACCESS_MASK, accessstatus : *mut super::bcrypt::NTSTATUS, generateonclose : *mut bool) -> super::bcrypt::NTSTATUS);
    unsafe { NtAccessCheckByTypeAndAuditAlarm(subsystemname, handleid.unwrap_or(core::mem::zeroed()) as _, objecttypename, objectname, securitydescriptor, principalselfsid.unwrap_or(core::mem::zeroed()) as _, desiredaccess, audittype, flags, core::mem::transmute(objecttypelist.map_or(core::ptr::null(), |slice| slice.as_ptr())), objecttypelist.map_or(0, |slice| slice.len().try_into().unwrap()), genericmapping, objectcreation, grantedaccess as _, accessstatus as _, generateonclose as _) }
}
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn NtAccessCheckByTypeResultListAndAuditAlarm(subsystemname: *const super::ntsecapi::UNICODE_STRING, handleid: Option<*const core::ffi::c_void>, objecttypename: *const super::ntsecapi::UNICODE_STRING, objectname: *const super::ntsecapi::UNICODE_STRING, securitydescriptor: super::winnt::PSECURITY_DESCRIPTOR, principalselfsid: Option<super::winnt::PSID>, desiredaccess: super::winnt::ACCESS_MASK, audittype: super::winnt::AUDIT_EVENT_TYPE, flags: u32, objecttypelist: Option<*const super::winnt::OBJECT_TYPE_LIST>, objecttypelistlength: u32, genericmapping: *const super::winnt::GENERIC_MAPPING, objectcreation: bool, grantedaccess: *mut super::winnt::ACCESS_MASK, accessstatus: *mut super::bcrypt::NTSTATUS, generateonclose: *mut bool) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtAccessCheckByTypeResultListAndAuditAlarm(subsystemname : *const super::ntsecapi::UNICODE_STRING, handleid : *const core::ffi::c_void, objecttypename : *const super::ntsecapi::UNICODE_STRING, objectname : *const super::ntsecapi::UNICODE_STRING, securitydescriptor : super::winnt::PSECURITY_DESCRIPTOR, principalselfsid : super::winnt::PSID, desiredaccess : super::winnt::ACCESS_MASK, audittype : super::winnt::AUDIT_EVENT_TYPE, flags : u32, objecttypelist : *const super::winnt::OBJECT_TYPE_LIST, objecttypelistlength : u32, genericmapping : *const super::winnt::GENERIC_MAPPING, objectcreation : bool, grantedaccess : *mut super::winnt::ACCESS_MASK, accessstatus : *mut super::bcrypt::NTSTATUS, generateonclose : *mut bool) -> super::bcrypt::NTSTATUS);
    unsafe { NtAccessCheckByTypeResultListAndAuditAlarm(subsystemname, handleid.unwrap_or(core::mem::zeroed()) as _, objecttypename, objectname, securitydescriptor, principalselfsid.unwrap_or(core::mem::zeroed()) as _, desiredaccess, audittype, flags, objecttypelist.unwrap_or(core::mem::zeroed()) as _, objecttypelistlength, genericmapping, objectcreation, grantedaccess as _, accessstatus as _, generateonclose as _) }
}
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn NtAccessCheckByTypeResultListAndAuditAlarmByHandle(
    subsystemname: *const super::ntsecapi::UNICODE_STRING,
    handleid: Option<*const core::ffi::c_void>,
    clienttoken: super::winnt::HANDLE,
    objecttypename: *const super::ntsecapi::UNICODE_STRING,
    objectname: *const super::ntsecapi::UNICODE_STRING,
    securitydescriptor: super::winnt::PSECURITY_DESCRIPTOR,
    principalselfsid: Option<super::winnt::PSID>,
    desiredaccess: super::winnt::ACCESS_MASK,
    audittype: super::winnt::AUDIT_EVENT_TYPE,
    flags: u32,
    objecttypelist: Option<*const super::winnt::OBJECT_TYPE_LIST>,
    objecttypelistlength: u32,
    genericmapping: *const super::winnt::GENERIC_MAPPING,
    objectcreation: bool,
    grantedaccess: *mut super::winnt::ACCESS_MASK,
    accessstatus: *mut super::bcrypt::NTSTATUS,
    generateonclose: *mut bool,
) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtAccessCheckByTypeResultListAndAuditAlarmByHandle(subsystemname : *const super::ntsecapi::UNICODE_STRING, handleid : *const core::ffi::c_void, clienttoken : super::winnt::HANDLE, objecttypename : *const super::ntsecapi::UNICODE_STRING, objectname : *const super::ntsecapi::UNICODE_STRING, securitydescriptor : super::winnt::PSECURITY_DESCRIPTOR, principalselfsid : super::winnt::PSID, desiredaccess : super::winnt::ACCESS_MASK, audittype : super::winnt::AUDIT_EVENT_TYPE, flags : u32, objecttypelist : *const super::winnt::OBJECT_TYPE_LIST, objecttypelistlength : u32, genericmapping : *const super::winnt::GENERIC_MAPPING, objectcreation : bool, grantedaccess : *mut super::winnt::ACCESS_MASK, accessstatus : *mut super::bcrypt::NTSTATUS, generateonclose : *mut bool) -> super::bcrypt::NTSTATUS);
    unsafe { NtAccessCheckByTypeResultListAndAuditAlarmByHandle(subsystemname, handleid.unwrap_or(core::mem::zeroed()) as _, clienttoken, objecttypename, objectname, securitydescriptor, principalselfsid.unwrap_or(core::mem::zeroed()) as _, desiredaccess, audittype, flags, objecttypelist.unwrap_or(core::mem::zeroed()) as _, objecttypelistlength, genericmapping, objectcreation, grantedaccess as _, accessstatus as _, generateonclose as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn NtAdjustGroupsToken(tokenhandle: super::winnt::HANDLE, resettodefault: bool, newstate: Option<*const super::winnt::TOKEN_GROUPS>, bufferlength: u32, previousstate: Option<*mut super::winnt::TOKEN_GROUPS>, returnlength: *mut u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtAdjustGroupsToken(tokenhandle : super::winnt::HANDLE, resettodefault : bool, newstate : *const super::winnt::TOKEN_GROUPS, bufferlength : u32, previousstate : *mut super::winnt::TOKEN_GROUPS, returnlength : *mut u32) -> super::bcrypt::NTSTATUS);
    unsafe { NtAdjustGroupsToken(tokenhandle, resettodefault, newstate.unwrap_or(core::mem::zeroed()) as _, bufferlength, previousstate.unwrap_or(core::mem::zeroed()) as _, returnlength as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn NtAdjustPrivilegesToken(tokenhandle: super::winnt::HANDLE, disableallprivileges: bool, newstate: Option<*const super::winnt::TOKEN_PRIVILEGES>, bufferlength: u32, previousstate: Option<*mut super::winnt::TOKEN_PRIVILEGES>, returnlength: *mut u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtAdjustPrivilegesToken(tokenhandle : super::winnt::HANDLE, disableallprivileges : bool, newstate : *const super::winnt::TOKEN_PRIVILEGES, bufferlength : u32, previousstate : *mut super::winnt::TOKEN_PRIVILEGES, returnlength : *mut u32) -> super::bcrypt::NTSTATUS);
    unsafe { NtAdjustPrivilegesToken(tokenhandle, disableallprivileges, newstate.unwrap_or(core::mem::zeroed()) as _, bufferlength, previousstate.unwrap_or(core::mem::zeroed()) as _, returnlength as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn NtAllocateVirtualMemory(processhandle: super::winnt::HANDLE, baseaddress: *mut *mut core::ffi::c_void, zerobits: usize, regionsize: *mut usize, allocationtype: u32, protect: u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtAllocateVirtualMemory(processhandle : super::winnt::HANDLE, baseaddress : *mut *mut core::ffi::c_void, zerobits : usize, regionsize : *mut usize, allocationtype : u32, protect : u32) -> super::bcrypt::NTSTATUS);
    unsafe { NtAllocateVirtualMemory(processhandle, baseaddress as _, zerobits, regionsize as _, allocationtype, protect) }
}
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi"))]
#[inline]
pub unsafe fn NtCloseObjectAuditAlarm(subsystemname: *const super::ntsecapi::UNICODE_STRING, handleid: Option<*const core::ffi::c_void>, generateonclose: bool) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtCloseObjectAuditAlarm(subsystemname : *const super::ntsecapi::UNICODE_STRING, handleid : *const core::ffi::c_void, generateonclose : bool) -> super::bcrypt::NTSTATUS);
    unsafe { NtCloseObjectAuditAlarm(subsystemname, handleid.unwrap_or(core::mem::zeroed()) as _, generateonclose) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn NtCopyFileChunk(sourcehandle: super::winnt::HANDLE, desthandle: super::winnt::HANDLE, event: Option<super::winnt::HANDLE>, iostatusblock: *mut super::winternl::IO_STATUS_BLOCK, length: u32, sourceoffset: *const i64, destoffset: *const i64, sourcekey: Option<*const u32>, destkey: Option<*const u32>, flags: u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtCopyFileChunk(sourcehandle : super::winnt::HANDLE, desthandle : super::winnt::HANDLE, event : super::winnt::HANDLE, iostatusblock : *mut super::winternl::IO_STATUS_BLOCK, length : u32, sourceoffset : *const i64, destoffset : *const i64, sourcekey : *const u32, destkey : *const u32, flags : u32) -> super::bcrypt::NTSTATUS);
    unsafe { NtCopyFileChunk(sourcehandle, desthandle, event.unwrap_or(core::mem::zeroed()) as _, iostatusblock as _, length, sourceoffset, destoffset, sourcekey.unwrap_or(core::mem::zeroed()) as _, destkey.unwrap_or(core::mem::zeroed()) as _, flags) }
}
#[cfg(all(feature = "bcrypt", feature = "d3dkmthk", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn NtCreateSection(sectionhandle: *mut super::winnt::HANDLE, desiredaccess: super::winnt::ACCESS_MASK, objectattributes: Option<*const super::d3dkmthk::OBJECT_ATTRIBUTES>, maximumsize: Option<*const i64>, sectionpageprotection: u32, allocationattributes: u32, filehandle: Option<super::winnt::HANDLE>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtCreateSection(sectionhandle : *mut super::winnt::HANDLE, desiredaccess : super::winnt::ACCESS_MASK, objectattributes : *const super::d3dkmthk::OBJECT_ATTRIBUTES, maximumsize : *const i64, sectionpageprotection : u32, allocationattributes : u32, filehandle : super::winnt::HANDLE) -> super::bcrypt::NTSTATUS);
    unsafe { NtCreateSection(sectionhandle as _, desiredaccess, objectattributes.unwrap_or(core::mem::zeroed()) as _, maximumsize.unwrap_or(core::mem::zeroed()) as _, sectionpageprotection, allocationattributes, filehandle.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "bcrypt", feature = "d3dkmthk", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn NtCreateSectionEx(sectionhandle: *mut super::winnt::HANDLE, desiredaccess: super::winnt::ACCESS_MASK, objectattributes: Option<*const super::d3dkmthk::OBJECT_ATTRIBUTES>, maximumsize: Option<*const i64>, sectionpageprotection: u32, allocationattributes: u32, filehandle: Option<super::winnt::HANDLE>, extendedparameters: Option<&mut [super::winnt::MEM_EXTENDED_PARAMETER]>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtCreateSectionEx(sectionhandle : *mut super::winnt::HANDLE, desiredaccess : super::winnt::ACCESS_MASK, objectattributes : *const super::d3dkmthk::OBJECT_ATTRIBUTES, maximumsize : *const i64, sectionpageprotection : u32, allocationattributes : u32, filehandle : super::winnt::HANDLE, extendedparameters : *mut super::winnt::MEM_EXTENDED_PARAMETER, extendedparametercount : u32) -> super::bcrypt::NTSTATUS);
    unsafe { NtCreateSectionEx(sectionhandle as _, desiredaccess, objectattributes.unwrap_or(core::mem::zeroed()) as _, maximumsize.unwrap_or(core::mem::zeroed()) as _, sectionpageprotection, allocationattributes, filehandle.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(extendedparameters.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), extendedparameters.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi"))]
#[inline]
pub unsafe fn NtDeleteObjectAuditAlarm(subsystemname: *const super::ntsecapi::UNICODE_STRING, handleid: Option<*const core::ffi::c_void>, generateonclose: bool) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtDeleteObjectAuditAlarm(subsystemname : *const super::ntsecapi::UNICODE_STRING, handleid : *const core::ffi::c_void, generateonclose : bool) -> super::bcrypt::NTSTATUS);
    unsafe { NtDeleteObjectAuditAlarm(subsystemname, handleid.unwrap_or(core::mem::zeroed()) as _, generateonclose) }
}
#[cfg(all(feature = "bcrypt", feature = "d3dkmthk", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn NtDuplicateToken(existingtokenhandle: super::winnt::HANDLE, desiredaccess: super::winnt::ACCESS_MASK, objectattributes: Option<*const super::d3dkmthk::OBJECT_ATTRIBUTES>, effectiveonly: bool, tokentype: super::winnt::TOKEN_TYPE, newtokenhandle: *mut super::winnt::HANDLE) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtDuplicateToken(existingtokenhandle : super::winnt::HANDLE, desiredaccess : super::winnt::ACCESS_MASK, objectattributes : *const super::d3dkmthk::OBJECT_ATTRIBUTES, effectiveonly : bool, tokentype : super::winnt::TOKEN_TYPE, newtokenhandle : *mut super::winnt::HANDLE) -> super::bcrypt::NTSTATUS);
    unsafe { NtDuplicateToken(existingtokenhandle, desiredaccess, objectattributes.unwrap_or(core::mem::zeroed()) as _, effectiveonly, tokentype, newtokenhandle as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn NtFilterToken(existingtokenhandle: super::winnt::HANDLE, flags: u32, sidstodisable: Option<*const super::winnt::TOKEN_GROUPS>, privilegestodelete: Option<*const super::winnt::TOKEN_PRIVILEGES>, restrictedsids: Option<*const super::winnt::TOKEN_GROUPS>, newtokenhandle: *mut super::winnt::HANDLE) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtFilterToken(existingtokenhandle : super::winnt::HANDLE, flags : u32, sidstodisable : *const super::winnt::TOKEN_GROUPS, privilegestodelete : *const super::winnt::TOKEN_PRIVILEGES, restrictedsids : *const super::winnt::TOKEN_GROUPS, newtokenhandle : *mut super::winnt::HANDLE) -> super::bcrypt::NTSTATUS);
    unsafe { NtFilterToken(existingtokenhandle, flags, sidstodisable.unwrap_or(core::mem::zeroed()) as _, privilegestodelete.unwrap_or(core::mem::zeroed()) as _, restrictedsids.unwrap_or(core::mem::zeroed()) as _, newtokenhandle as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn NtFlushBuffersFileEx(filehandle: super::winnt::HANDLE, flags: u32, parameters: *const core::ffi::c_void, parameterssize: u32, iostatusblock: *mut super::winternl::IO_STATUS_BLOCK) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtFlushBuffersFileEx(filehandle : super::winnt::HANDLE, flags : u32, parameters : *const core::ffi::c_void, parameterssize : u32, iostatusblock : *mut super::winternl::IO_STATUS_BLOCK) -> super::bcrypt::NTSTATUS);
    unsafe { NtFlushBuffersFileEx(filehandle, flags, parameters, parameterssize, iostatusblock as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn NtFreeVirtualMemory(processhandle: super::winnt::HANDLE, baseaddress: *mut *mut core::ffi::c_void, regionsize: *mut usize, freetype: u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtFreeVirtualMemory(processhandle : super::winnt::HANDLE, baseaddress : *mut *mut core::ffi::c_void, regionsize : *mut usize, freetype : u32) -> super::bcrypt::NTSTATUS);
    unsafe { NtFreeVirtualMemory(processhandle, baseaddress as _, regionsize as _, freetype) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn NtFsControlFile(filehandle: super::winnt::HANDLE, event: Option<super::winnt::HANDLE>, apcroutine: super::winternl::PIO_APC_ROUTINE, apccontext: Option<*const core::ffi::c_void>, iostatusblock: *mut super::winternl::IO_STATUS_BLOCK, fscontrolcode: u32, inputbuffer: Option<*const core::ffi::c_void>, inputbufferlength: u32, outputbuffer: Option<*mut core::ffi::c_void>, outputbufferlength: u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtFsControlFile(filehandle : super::winnt::HANDLE, event : super::winnt::HANDLE, apcroutine : super::winternl::PIO_APC_ROUTINE, apccontext : *const core::ffi::c_void, iostatusblock : *mut super::winternl::IO_STATUS_BLOCK, fscontrolcode : u32, inputbuffer : *const core::ffi::c_void, inputbufferlength : u32, outputbuffer : *mut core::ffi::c_void, outputbufferlength : u32) -> super::bcrypt::NTSTATUS);
    unsafe { NtFsControlFile(filehandle, event.unwrap_or(core::mem::zeroed()) as _, apcroutine, apccontext.unwrap_or(core::mem::zeroed()) as _, iostatusblock as _, fscontrolcode, inputbuffer.unwrap_or(core::mem::zeroed()) as _, inputbufferlength, outputbuffer.unwrap_or(core::mem::zeroed()) as _, outputbufferlength) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn NtImpersonateAnonymousToken(threadhandle: super::winnt::HANDLE) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtImpersonateAnonymousToken(threadhandle : super::winnt::HANDLE) -> super::bcrypt::NTSTATUS);
    unsafe { NtImpersonateAnonymousToken(threadhandle) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn NtLockFile(filehandle: super::winnt::HANDLE, event: Option<super::winnt::HANDLE>, apcroutine: super::winternl::PIO_APC_ROUTINE, apccontext: Option<*const core::ffi::c_void>, iostatusblock: *mut super::winternl::IO_STATUS_BLOCK, byteoffset: *const i64, length: *const i64, key: u32, failimmediately: bool, exclusivelock: bool) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtLockFile(filehandle : super::winnt::HANDLE, event : super::winnt::HANDLE, apcroutine : super::winternl::PIO_APC_ROUTINE, apccontext : *const core::ffi::c_void, iostatusblock : *mut super::winternl::IO_STATUS_BLOCK, byteoffset : *const i64, length : *const i64, key : u32, failimmediately : bool, exclusivelock : bool) -> super::bcrypt::NTSTATUS);
    unsafe { NtLockFile(filehandle, event.unwrap_or(core::mem::zeroed()) as _, apcroutine, apccontext.unwrap_or(core::mem::zeroed()) as _, iostatusblock as _, byteoffset, length, key, failimmediately, exclusivelock) }
}
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn NtOpenObjectAuditAlarm(subsystemname: *const super::ntsecapi::UNICODE_STRING, handleid: Option<*const core::ffi::c_void>, objecttypename: *const super::ntsecapi::UNICODE_STRING, objectname: *const super::ntsecapi::UNICODE_STRING, securitydescriptor: Option<super::winnt::PSECURITY_DESCRIPTOR>, clienttoken: super::winnt::HANDLE, desiredaccess: super::winnt::ACCESS_MASK, grantedaccess: super::winnt::ACCESS_MASK, privileges: Option<*const super::winnt::PRIVILEGE_SET>, objectcreation: bool, accessgranted: bool, generateonclose: *mut bool) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtOpenObjectAuditAlarm(subsystemname : *const super::ntsecapi::UNICODE_STRING, handleid : *const core::ffi::c_void, objecttypename : *const super::ntsecapi::UNICODE_STRING, objectname : *const super::ntsecapi::UNICODE_STRING, securitydescriptor : super::winnt::PSECURITY_DESCRIPTOR, clienttoken : super::winnt::HANDLE, desiredaccess : super::winnt::ACCESS_MASK, grantedaccess : super::winnt::ACCESS_MASK, privileges : *const super::winnt::PRIVILEGE_SET, objectcreation : bool, accessgranted : bool, generateonclose : *mut bool) -> super::bcrypt::NTSTATUS);
    unsafe { NtOpenObjectAuditAlarm(subsystemname, handleid.unwrap_or(core::mem::zeroed()) as _, objecttypename, objectname, securitydescriptor.unwrap_or(core::mem::zeroed()) as _, clienttoken, desiredaccess, grantedaccess, privileges.unwrap_or(core::mem::zeroed()) as _, objectcreation, accessgranted, generateonclose as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn NtOpenProcessToken(processhandle: super::winnt::HANDLE, desiredaccess: super::winnt::ACCESS_MASK, tokenhandle: *mut super::winnt::HANDLE) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtOpenProcessToken(processhandle : super::winnt::HANDLE, desiredaccess : super::winnt::ACCESS_MASK, tokenhandle : *mut super::winnt::HANDLE) -> super::bcrypt::NTSTATUS);
    unsafe { NtOpenProcessToken(processhandle, desiredaccess, tokenhandle as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn NtOpenProcessTokenEx(processhandle: super::winnt::HANDLE, desiredaccess: super::winnt::ACCESS_MASK, handleattributes: u32, tokenhandle: *mut super::winnt::HANDLE) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtOpenProcessTokenEx(processhandle : super::winnt::HANDLE, desiredaccess : super::winnt::ACCESS_MASK, handleattributes : u32, tokenhandle : *mut super::winnt::HANDLE) -> super::bcrypt::NTSTATUS);
    unsafe { NtOpenProcessTokenEx(processhandle, desiredaccess, handleattributes, tokenhandle as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn NtOpenThreadToken(threadhandle: super::winnt::HANDLE, desiredaccess: super::winnt::ACCESS_MASK, openasself: bool, tokenhandle: *mut super::winnt::HANDLE) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtOpenThreadToken(threadhandle : super::winnt::HANDLE, desiredaccess : super::winnt::ACCESS_MASK, openasself : bool, tokenhandle : *mut super::winnt::HANDLE) -> super::bcrypt::NTSTATUS);
    unsafe { NtOpenThreadToken(threadhandle, desiredaccess, openasself, tokenhandle as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn NtOpenThreadTokenEx(threadhandle: super::winnt::HANDLE, desiredaccess: super::winnt::ACCESS_MASK, openasself: bool, handleattributes: u32, tokenhandle: *mut super::winnt::HANDLE) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtOpenThreadTokenEx(threadhandle : super::winnt::HANDLE, desiredaccess : super::winnt::ACCESS_MASK, openasself : bool, handleattributes : u32, tokenhandle : *mut super::winnt::HANDLE) -> super::bcrypt::NTSTATUS);
    unsafe { NtOpenThreadTokenEx(threadhandle, desiredaccess, openasself, handleattributes, tokenhandle as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn NtPrivilegeCheck(clienttoken: super::winnt::HANDLE, requiredprivileges: *mut super::winnt::PRIVILEGE_SET, result: *mut bool) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtPrivilegeCheck(clienttoken : super::winnt::HANDLE, requiredprivileges : *mut super::winnt::PRIVILEGE_SET, result : *mut bool) -> super::bcrypt::NTSTATUS);
    unsafe { NtPrivilegeCheck(clienttoken, requiredprivileges as _, result as _) }
}
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn NtPrivilegeObjectAuditAlarm(subsystemname: *const super::ntsecapi::UNICODE_STRING, handleid: Option<*const core::ffi::c_void>, clienttoken: super::winnt::HANDLE, desiredaccess: super::winnt::ACCESS_MASK, privileges: *const super::winnt::PRIVILEGE_SET, accessgranted: bool) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtPrivilegeObjectAuditAlarm(subsystemname : *const super::ntsecapi::UNICODE_STRING, handleid : *const core::ffi::c_void, clienttoken : super::winnt::HANDLE, desiredaccess : super::winnt::ACCESS_MASK, privileges : *const super::winnt::PRIVILEGE_SET, accessgranted : bool) -> super::bcrypt::NTSTATUS);
    unsafe { NtPrivilegeObjectAuditAlarm(subsystemname, handleid.unwrap_or(core::mem::zeroed()) as _, clienttoken, desiredaccess, privileges, accessgranted) }
}
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn NtPrivilegedServiceAuditAlarm(subsystemname: *const super::ntsecapi::UNICODE_STRING, servicename: *const super::ntsecapi::UNICODE_STRING, clienttoken: super::winnt::HANDLE, privileges: *const super::winnt::PRIVILEGE_SET, accessgranted: bool) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtPrivilegedServiceAuditAlarm(subsystemname : *const super::ntsecapi::UNICODE_STRING, servicename : *const super::ntsecapi::UNICODE_STRING, clienttoken : super::winnt::HANDLE, privileges : *const super::winnt::PRIVILEGE_SET, accessgranted : bool) -> super::bcrypt::NTSTATUS);
    unsafe { NtPrivilegedServiceAuditAlarm(subsystemname, servicename, clienttoken, privileges, accessgranted) }
}
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn NtQueryDirectoryFile(filehandle: super::winnt::HANDLE, event: Option<super::winnt::HANDLE>, apcroutine: super::winternl::PIO_APC_ROUTINE, apccontext: Option<*const core::ffi::c_void>, iostatusblock: *mut super::winternl::IO_STATUS_BLOCK, fileinformation: *mut core::ffi::c_void, length: u32, fileinformationclass: super::winternl::FILE_INFORMATION_CLASS, returnsingleentry: bool, filename: Option<*const super::ntsecapi::UNICODE_STRING>, restartscan: bool) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtQueryDirectoryFile(filehandle : super::winnt::HANDLE, event : super::winnt::HANDLE, apcroutine : super::winternl::PIO_APC_ROUTINE, apccontext : *const core::ffi::c_void, iostatusblock : *mut super::winternl::IO_STATUS_BLOCK, fileinformation : *mut core::ffi::c_void, length : u32, fileinformationclass : super::winternl::FILE_INFORMATION_CLASS, returnsingleentry : bool, filename : *const super::ntsecapi::UNICODE_STRING, restartscan : bool) -> super::bcrypt::NTSTATUS);
    unsafe { NtQueryDirectoryFile(filehandle, event.unwrap_or(core::mem::zeroed()) as _, apcroutine, apccontext.unwrap_or(core::mem::zeroed()) as _, iostatusblock as _, fileinformation as _, length, fileinformationclass, returnsingleentry, filename.unwrap_or(core::mem::zeroed()) as _, restartscan) }
}
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn NtQueryDirectoryFileEx(filehandle: super::winnt::HANDLE, event: Option<super::winnt::HANDLE>, apcroutine: super::winternl::PIO_APC_ROUTINE, apccontext: Option<*const core::ffi::c_void>, iostatusblock: *mut super::winternl::IO_STATUS_BLOCK, fileinformation: *mut core::ffi::c_void, length: u32, fileinformationclass: super::winternl::FILE_INFORMATION_CLASS, queryflags: u32, filename: Option<*const super::ntsecapi::UNICODE_STRING>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtQueryDirectoryFileEx(filehandle : super::winnt::HANDLE, event : super::winnt::HANDLE, apcroutine : super::winternl::PIO_APC_ROUTINE, apccontext : *const core::ffi::c_void, iostatusblock : *mut super::winternl::IO_STATUS_BLOCK, fileinformation : *mut core::ffi::c_void, length : u32, fileinformationclass : super::winternl::FILE_INFORMATION_CLASS, queryflags : u32, filename : *const super::ntsecapi::UNICODE_STRING) -> super::bcrypt::NTSTATUS);
    unsafe { NtQueryDirectoryFileEx(filehandle, event.unwrap_or(core::mem::zeroed()) as _, apcroutine, apccontext.unwrap_or(core::mem::zeroed()) as _, iostatusblock as _, fileinformation as _, length, fileinformationclass, queryflags, filename.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "bcrypt", feature = "d3dkmthk", feature = "lsalookup", feature = "ntsecapi", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn NtQueryInformationByName(objectattributes: *const super::d3dkmthk::OBJECT_ATTRIBUTES, iostatusblock: *mut super::winternl::IO_STATUS_BLOCK, fileinformation: *mut core::ffi::c_void, length: u32, fileinformationclass: super::winternl::FILE_INFORMATION_CLASS) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtQueryInformationByName(objectattributes : *const super::d3dkmthk::OBJECT_ATTRIBUTES, iostatusblock : *mut super::winternl::IO_STATUS_BLOCK, fileinformation : *mut core::ffi::c_void, length : u32, fileinformationclass : super::winternl::FILE_INFORMATION_CLASS) -> super::bcrypt::NTSTATUS);
    unsafe { NtQueryInformationByName(objectattributes, iostatusblock as _, fileinformation as _, length, fileinformationclass) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn NtQueryInformationFile(filehandle: super::winnt::HANDLE, iostatusblock: *mut super::winternl::IO_STATUS_BLOCK, fileinformation: *mut core::ffi::c_void, length: u32, fileinformationclass: super::winternl::FILE_INFORMATION_CLASS) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtQueryInformationFile(filehandle : super::winnt::HANDLE, iostatusblock : *mut super::winternl::IO_STATUS_BLOCK, fileinformation : *mut core::ffi::c_void, length : u32, fileinformationclass : super::winternl::FILE_INFORMATION_CLASS) -> super::bcrypt::NTSTATUS);
    unsafe { NtQueryInformationFile(filehandle, iostatusblock as _, fileinformation as _, length, fileinformationclass) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn NtQueryInformationToken(tokenhandle: super::winnt::HANDLE, tokeninformationclass: super::winnt::TOKEN_INFORMATION_CLASS, tokeninformation: Option<*mut core::ffi::c_void>, tokeninformationlength: u32, returnlength: *mut u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtQueryInformationToken(tokenhandle : super::winnt::HANDLE, tokeninformationclass : super::winnt::TOKEN_INFORMATION_CLASS, tokeninformation : *mut core::ffi::c_void, tokeninformationlength : u32, returnlength : *mut u32) -> super::bcrypt::NTSTATUS);
    unsafe { NtQueryInformationToken(tokenhandle, tokeninformationclass, tokeninformation.unwrap_or(core::mem::zeroed()) as _, tokeninformationlength, returnlength as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn NtQueryQuotaInformationFile(filehandle: super::winnt::HANDLE, iostatusblock: *mut super::winternl::IO_STATUS_BLOCK, buffer: *mut core::ffi::c_void, length: u32, returnsingleentry: bool, sidlist: Option<*const core::ffi::c_void>, sidlistlength: u32, startsid: Option<super::winnt::PSID>, restartscan: bool) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtQueryQuotaInformationFile(filehandle : super::winnt::HANDLE, iostatusblock : *mut super::winternl::IO_STATUS_BLOCK, buffer : *mut core::ffi::c_void, length : u32, returnsingleentry : bool, sidlist : *const core::ffi::c_void, sidlistlength : u32, startsid : super::winnt::PSID, restartscan : bool) -> super::bcrypt::NTSTATUS);
    unsafe { NtQueryQuotaInformationFile(filehandle, iostatusblock as _, buffer as _, length, returnsingleentry, sidlist.unwrap_or(core::mem::zeroed()) as _, sidlistlength, startsid.unwrap_or(core::mem::zeroed()) as _, restartscan) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn NtQuerySecurityObject(handle: super::winnt::HANDLE, securityinformation: super::winnt::SECURITY_INFORMATION, securitydescriptor: Option<super::winnt::PSECURITY_DESCRIPTOR>, length: u32, lengthneeded: *mut u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtQuerySecurityObject(handle : super::winnt::HANDLE, securityinformation : super::winnt::SECURITY_INFORMATION, securitydescriptor : super::winnt::PSECURITY_DESCRIPTOR, length : u32, lengthneeded : *mut u32) -> super::bcrypt::NTSTATUS);
    unsafe { NtQuerySecurityObject(handle, securityinformation, securitydescriptor.unwrap_or(core::mem::zeroed()) as _, length, lengthneeded as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn NtQueryVirtualMemory(processhandle: super::winnt::HANDLE, baseaddress: Option<*const core::ffi::c_void>, memoryinformationclass: MEMORY_INFORMATION_CLASS, memoryinformation: *mut core::ffi::c_void, memoryinformationlength: usize, returnlength: Option<*mut usize>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtQueryVirtualMemory(processhandle : super::winnt::HANDLE, baseaddress : *const core::ffi::c_void, memoryinformationclass : MEMORY_INFORMATION_CLASS, memoryinformation : *mut core::ffi::c_void, memoryinformationlength : usize, returnlength : *mut usize) -> super::bcrypt::NTSTATUS);
    unsafe { NtQueryVirtualMemory(processhandle, baseaddress.unwrap_or(core::mem::zeroed()) as _, memoryinformationclass, memoryinformation as _, memoryinformationlength, returnlength.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "bcrypt", feature = "wdm", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn NtQueryVolumeInformationFile(filehandle: super::winnt::HANDLE, iostatusblock: *mut super::winternl::IO_STATUS_BLOCK, fsinformation: *mut core::ffi::c_void, length: u32, fsinformationclass: super::wdm::FS_INFORMATION_CLASS) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtQueryVolumeInformationFile(filehandle : super::winnt::HANDLE, iostatusblock : *mut super::winternl::IO_STATUS_BLOCK, fsinformation : *mut core::ffi::c_void, length : u32, fsinformationclass : super::wdm::FS_INFORMATION_CLASS) -> super::bcrypt::NTSTATUS);
    unsafe { NtQueryVolumeInformationFile(filehandle, iostatusblock as _, fsinformation as _, length, fsinformationclass) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn NtReadFile(filehandle: super::winnt::HANDLE, event: Option<super::winnt::HANDLE>, apcroutine: super::winternl::PIO_APC_ROUTINE, apccontext: Option<*const core::ffi::c_void>, iostatusblock: *mut super::winternl::IO_STATUS_BLOCK, buffer: *mut core::ffi::c_void, length: u32, byteoffset: Option<*const i64>, key: Option<*const u32>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtReadFile(filehandle : super::winnt::HANDLE, event : super::winnt::HANDLE, apcroutine : super::winternl::PIO_APC_ROUTINE, apccontext : *const core::ffi::c_void, iostatusblock : *mut super::winternl::IO_STATUS_BLOCK, buffer : *mut core::ffi::c_void, length : u32, byteoffset : *const i64, key : *const u32) -> super::bcrypt::NTSTATUS);
    unsafe { NtReadFile(filehandle, event.unwrap_or(core::mem::zeroed()) as _, apcroutine, apccontext.unwrap_or(core::mem::zeroed()) as _, iostatusblock as _, buffer as _, length, byteoffset.unwrap_or(core::mem::zeroed()) as _, key.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn NtSetInformationFile(filehandle: super::winnt::HANDLE, iostatusblock: *mut super::winternl::IO_STATUS_BLOCK, fileinformation: *const core::ffi::c_void, length: u32, fileinformationclass: super::winternl::FILE_INFORMATION_CLASS) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtSetInformationFile(filehandle : super::winnt::HANDLE, iostatusblock : *mut super::winternl::IO_STATUS_BLOCK, fileinformation : *const core::ffi::c_void, length : u32, fileinformationclass : super::winternl::FILE_INFORMATION_CLASS) -> super::bcrypt::NTSTATUS);
    unsafe { NtSetInformationFile(filehandle, iostatusblock as _, fileinformation, length, fileinformationclass) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn NtSetInformationToken(tokenhandle: super::winnt::HANDLE, tokeninformationclass: super::winnt::TOKEN_INFORMATION_CLASS, tokeninformation: *const core::ffi::c_void, tokeninformationlength: u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtSetInformationToken(tokenhandle : super::winnt::HANDLE, tokeninformationclass : super::winnt::TOKEN_INFORMATION_CLASS, tokeninformation : *const core::ffi::c_void, tokeninformationlength : u32) -> super::bcrypt::NTSTATUS);
    unsafe { NtSetInformationToken(tokenhandle, tokeninformationclass, tokeninformation, tokeninformationlength) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn NtSetInformationVirtualMemory(processhandle: super::winnt::HANDLE, vminformationclass: VIRTUAL_MEMORY_INFORMATION_CLASS, virtualaddresses: &[MEMORY_RANGE_ENTRY], vminformation: *const core::ffi::c_void, vminformationlength: u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtSetInformationVirtualMemory(processhandle : super::winnt::HANDLE, vminformationclass : VIRTUAL_MEMORY_INFORMATION_CLASS, numberofentries : usize, virtualaddresses : *const MEMORY_RANGE_ENTRY, vminformation : *const core::ffi::c_void, vminformationlength : u32) -> super::bcrypt::NTSTATUS);
    unsafe { NtSetInformationVirtualMemory(processhandle, vminformationclass, virtualaddresses.len().try_into().unwrap(), core::mem::transmute(virtualaddresses.as_ptr()), vminformation, vminformationlength) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn NtSetQuotaInformationFile(filehandle: super::winnt::HANDLE, iostatusblock: *mut super::winternl::IO_STATUS_BLOCK, buffer: *const core::ffi::c_void, length: u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtSetQuotaInformationFile(filehandle : super::winnt::HANDLE, iostatusblock : *mut super::winternl::IO_STATUS_BLOCK, buffer : *const core::ffi::c_void, length : u32) -> super::bcrypt::NTSTATUS);
    unsafe { NtSetQuotaInformationFile(filehandle, iostatusblock as _, buffer, length) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn NtSetSecurityObject(handle: super::winnt::HANDLE, securityinformation: super::winnt::SECURITY_INFORMATION, securitydescriptor: super::winnt::PSECURITY_DESCRIPTOR) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtSetSecurityObject(handle : super::winnt::HANDLE, securityinformation : super::winnt::SECURITY_INFORMATION, securitydescriptor : super::winnt::PSECURITY_DESCRIPTOR) -> super::bcrypt::NTSTATUS);
    unsafe { NtSetSecurityObject(handle, securityinformation, securitydescriptor) }
}
#[cfg(all(feature = "bcrypt", feature = "wdm", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn NtSetVolumeInformationFile(filehandle: super::winnt::HANDLE, iostatusblock: *mut super::winternl::IO_STATUS_BLOCK, fsinformation: *const core::ffi::c_void, length: u32, fsinformationclass: super::wdm::FS_INFORMATION_CLASS) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtSetVolumeInformationFile(filehandle : super::winnt::HANDLE, iostatusblock : *mut super::winternl::IO_STATUS_BLOCK, fsinformation : *const core::ffi::c_void, length : u32, fsinformationclass : super::wdm::FS_INFORMATION_CLASS) -> super::bcrypt::NTSTATUS);
    unsafe { NtSetVolumeInformationFile(filehandle, iostatusblock as _, fsinformation, length, fsinformationclass) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn NtUnlockFile(filehandle: super::winnt::HANDLE, iostatusblock: *mut super::winternl::IO_STATUS_BLOCK, byteoffset: *const i64, length: *const i64, key: u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtUnlockFile(filehandle : super::winnt::HANDLE, iostatusblock : *mut super::winternl::IO_STATUS_BLOCK, byteoffset : *const i64, length : *const i64, key : u32) -> super::bcrypt::NTSTATUS);
    unsafe { NtUnlockFile(filehandle, iostatusblock as _, byteoffset, length, key) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn NtWriteFile(filehandle: super::winnt::HANDLE, event: Option<super::winnt::HANDLE>, apcroutine: super::winternl::PIO_APC_ROUTINE, apccontext: Option<*const core::ffi::c_void>, iostatusblock: *mut super::winternl::IO_STATUS_BLOCK, buffer: *const core::ffi::c_void, length: u32, byteoffset: Option<*const i64>, key: Option<*const u32>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtWriteFile(filehandle : super::winnt::HANDLE, event : super::winnt::HANDLE, apcroutine : super::winternl::PIO_APC_ROUTINE, apccontext : *const core::ffi::c_void, iostatusblock : *mut super::winternl::IO_STATUS_BLOCK, buffer : *const core::ffi::c_void, length : u32, byteoffset : *const i64, key : *const u32) -> super::bcrypt::NTSTATUS);
    unsafe { NtWriteFile(filehandle, event.unwrap_or(core::mem::zeroed()) as _, apcroutine, apccontext.unwrap_or(core::mem::zeroed()) as _, iostatusblock as _, buffer, length, byteoffset.unwrap_or(core::mem::zeroed()) as _, key.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "lsalookup", feature = "ntddk", feature = "ntdef", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn PfxFindPrefix(prefixtable: *const PREFIX_TABLE, fullname: *const super::ntsecapi::STRING) -> PPREFIX_TABLE_ENTRY {
    windows_core::link!("ntdll.dll" "system" fn PfxFindPrefix(prefixtable : *const PREFIX_TABLE, fullname : *const super::ntsecapi::STRING) -> PPREFIX_TABLE_ENTRY);
    unsafe { PfxFindPrefix(prefixtable, fullname) }
}
#[cfg(all(feature = "lsalookup", feature = "ntddk", feature = "ntdef", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn PfxInitialize() -> PREFIX_TABLE {
    windows_core::link!("ntdll.dll" "system" fn PfxInitialize(prefixtable : *mut PREFIX_TABLE));
    unsafe {
        let mut result__ = core::mem::zeroed();
        PfxInitialize(&mut result__);
        result__
    }
}
#[cfg(all(feature = "lsalookup", feature = "ntddk", feature = "ntdef", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn PfxInsertPrefix(prefixtable: *const PREFIX_TABLE, prefix: *const super::ntsecapi::STRING, prefixtableentry: *mut PREFIX_TABLE_ENTRY) -> bool {
    windows_core::link!("ntdll.dll" "system" fn PfxInsertPrefix(prefixtable : *const PREFIX_TABLE, prefix : *const super::ntsecapi::STRING, prefixtableentry : *mut PREFIX_TABLE_ENTRY) -> bool);
    unsafe { PfxInsertPrefix(prefixtable, prefix, prefixtableentry as _) }
}
#[cfg(all(feature = "lsalookup", feature = "ntddk", feature = "ntdef", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn PfxRemovePrefix(prefixtable: *const PREFIX_TABLE, prefixtableentry: *const PREFIX_TABLE_ENTRY) {
    windows_core::link!("ntdll.dll" "system" fn PfxRemovePrefix(prefixtable : *const PREFIX_TABLE, prefixtableentry : *const PREFIX_TABLE_ENTRY));
    unsafe { PfxRemovePrefix(prefixtable, prefixtableentry) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn RtlAbsoluteToSelfRelativeSD(absolutesecuritydescriptor: super::winnt::PSECURITY_DESCRIPTOR, selfrelativesecuritydescriptor: Option<super::winnt::PSECURITY_DESCRIPTOR>, bufferlength: *mut u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlAbsoluteToSelfRelativeSD(absolutesecuritydescriptor : super::winnt::PSECURITY_DESCRIPTOR, selfrelativesecuritydescriptor : super::winnt::PSECURITY_DESCRIPTOR, bufferlength : *mut u32) -> super::bcrypt::NTSTATUS);
    unsafe { RtlAbsoluteToSelfRelativeSD(absolutesecuritydescriptor, selfrelativesecuritydescriptor.unwrap_or(core::mem::zeroed()) as _, bufferlength as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn RtlAddAccessAllowedAce(acl: *mut super::winnt::ACL, acerevision: u32, accessmask: super::winnt::ACCESS_MASK, sid: super::winnt::PSID) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlAddAccessAllowedAce(acl : *mut super::winnt::ACL, acerevision : u32, accessmask : super::winnt::ACCESS_MASK, sid : super::winnt::PSID) -> super::bcrypt::NTSTATUS);
    unsafe { RtlAddAccessAllowedAce(acl as _, acerevision, accessmask, sid) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn RtlAddAccessAllowedAceEx(acl: *mut super::winnt::ACL, acerevision: u32, aceflags: u32, accessmask: super::winnt::ACCESS_MASK, sid: super::winnt::PSID) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlAddAccessAllowedAceEx(acl : *mut super::winnt::ACL, acerevision : u32, aceflags : u32, accessmask : super::winnt::ACCESS_MASK, sid : super::winnt::PSID) -> super::bcrypt::NTSTATUS);
    unsafe { RtlAddAccessAllowedAceEx(acl as _, acerevision, aceflags, accessmask, sid) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn RtlAddAce(acl: *mut super::winnt::ACL, acerevision: u32, startingaceindex: u32, acelist: *const core::ffi::c_void, acelistlength: u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlAddAce(acl : *mut super::winnt::ACL, acerevision : u32, startingaceindex : u32, acelist : *const core::ffi::c_void, acelistlength : u32) -> super::bcrypt::NTSTATUS);
    unsafe { RtlAddAce(acl as _, acerevision, startingaceindex, acelist, acelistlength) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn RtlAllocateAndInitializeSid(identifierauthority: *const super::winnt::SID_IDENTIFIER_AUTHORITY, subauthoritycount: u8, subauthority0: u32, subauthority1: u32, subauthority2: u32, subauthority3: u32, subauthority4: u32, subauthority5: u32, subauthority6: u32, subauthority7: u32, sid: *mut super::winnt::PSID) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlAllocateAndInitializeSid(identifierauthority : *const super::winnt::SID_IDENTIFIER_AUTHORITY, subauthoritycount : u8, subauthority0 : u32, subauthority1 : u32, subauthority2 : u32, subauthority3 : u32, subauthority4 : u32, subauthority5 : u32, subauthority6 : u32, subauthority7 : u32, sid : *mut super::winnt::PSID) -> super::bcrypt::NTSTATUS);
    unsafe { RtlAllocateAndInitializeSid(identifierauthority, subauthoritycount, subauthority0, subauthority1, subauthority2, subauthority3, subauthority4, subauthority5, subauthority6, subauthority7, sid as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn RtlAllocateAndInitializeSidEx(identifierauthority: *const super::winnt::SID_IDENTIFIER_AUTHORITY, subauthorities: &[u32], sid: *mut super::winnt::PSID) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlAllocateAndInitializeSidEx(identifierauthority : *const super::winnt::SID_IDENTIFIER_AUTHORITY, subauthoritycount : u8, subauthorities : *const u32, sid : *mut super::winnt::PSID) -> super::bcrypt::NTSTATUS);
    unsafe { RtlAllocateAndInitializeSidEx(identifierauthority, subauthorities.len().try_into().unwrap(), core::mem::transmute(subauthorities.as_ptr()), sid as _) }
}
#[inline]
pub unsafe fn RtlAllocateHeap(heaphandle: *const core::ffi::c_void, flags: Option<u32>, size: usize) -> *mut core::ffi::c_void {
    windows_core::link!("ntdll.dll" "system" fn RtlAllocateHeap(heaphandle : *const core::ffi::c_void, flags : u32, size : usize) -> *mut core::ffi::c_void);
    unsafe { RtlAllocateHeap(heaphandle, flags.unwrap_or(core::mem::zeroed()) as _, size) }
}
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn RtlAppendStringToString(destination: *mut super::ntsecapi::STRING, source: *const super::ntsecapi::STRING) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlAppendStringToString(destination : *mut super::ntsecapi::STRING, source : *const super::ntsecapi::STRING) -> super::bcrypt::NTSTATUS);
    unsafe { RtlAppendStringToString(destination as _, source) }
}
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
#[inline]
pub unsafe fn RtlCompareAltitudes(altitude1: *const super::ntsecapi::UNICODE_STRING, altitude2: *const super::ntsecapi::UNICODE_STRING) -> i32 {
    windows_core::link!("ntdll.dll" "system" fn RtlCompareAltitudes(altitude1 : *const super::ntsecapi::UNICODE_STRING, altitude2 : *const super::ntsecapi::UNICODE_STRING) -> i32);
    unsafe { RtlCompareAltitudes(altitude1, altitude2) }
}
#[inline]
pub unsafe fn RtlCompareMemoryUlong(source: *const core::ffi::c_void, length: usize, pattern: u32) -> usize {
    windows_core::link!("ntdll.dll" "system" fn RtlCompareMemoryUlong(source : *const core::ffi::c_void, length : usize, pattern : u32) -> usize);
    unsafe { RtlCompareMemoryUlong(source, length, pattern) }
}
#[cfg(feature = "bcrypt")]
#[inline]
pub unsafe fn RtlCompressBuffer(compressionformatandengine: u16, uncompressedbuffer: &[u8], compressedbuffer: &mut [u8], uncompressedchunksize: u32, finalcompressedsize: *mut u32, workspace: *const core::ffi::c_void) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlCompressBuffer(compressionformatandengine : u16, uncompressedbuffer : *const u8, uncompressedbuffersize : u32, compressedbuffer : *mut u8, compressedbuffersize : u32, uncompressedchunksize : u32, finalcompressedsize : *mut u32, workspace : *const core::ffi::c_void) -> super::bcrypt::NTSTATUS);
    unsafe { RtlCompressBuffer(compressionformatandengine, core::mem::transmute(uncompressedbuffer.as_ptr()), uncompressedbuffer.len().try_into().unwrap(), core::mem::transmute(compressedbuffer.as_ptr()), compressedbuffer.len().try_into().unwrap(), uncompressedchunksize, finalcompressedsize as _, workspace) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RtlCopyLuid(destinationluid: *mut super::winnt::LUID, sourceluid: *const super::winnt::LUID) {
    windows_core::link!("ntdll.dll" "system" fn RtlCopyLuid(destinationluid : *mut super::winnt::LUID, sourceluid : *const super::winnt::LUID));
    unsafe { RtlCopyLuid(destinationluid as _, sourceluid) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn RtlCopySid(destinationsidlength: u32, destinationsid: super::winnt::PSID, sourcesid: super::winnt::PSID) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlCopySid(destinationsidlength : u32, destinationsid : super::winnt::PSID, sourcesid : super::winnt::PSID) -> super::bcrypt::NTSTATUS);
    unsafe { RtlCopySid(destinationsidlength, destinationsid as _, sourcesid) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn RtlCreateAcl(acl: *mut super::winnt::ACL, acllength: u32, aclrevision: u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlCreateAcl(acl : *mut super::winnt::ACL, acllength : u32, aclrevision : u32) -> super::bcrypt::NTSTATUS);
    unsafe { RtlCreateAcl(acl as _, acllength, aclrevision) }
}
#[cfg(feature = "bcrypt")]
#[inline]
pub unsafe fn RtlCreateHeap(flags: u32, heapbase: Option<*const core::ffi::c_void>, reservesize: Option<usize>, commitsize: Option<usize>, lock: Option<*const core::ffi::c_void>, parameters: *mut RTL_HEAP_PARAMETERS) -> *mut core::ffi::c_void {
    windows_core::link!("ntdll.dll" "system" fn RtlCreateHeap(flags : u32, heapbase : *const core::ffi::c_void, reservesize : usize, commitsize : usize, lock : *const core::ffi::c_void, parameters : *mut RTL_HEAP_PARAMETERS) -> *mut core::ffi::c_void);
    unsafe { RtlCreateHeap(flags, heapbase.unwrap_or(core::mem::zeroed()) as _, reservesize.unwrap_or(core::mem::zeroed()) as _, commitsize.unwrap_or(core::mem::zeroed()) as _, lock.unwrap_or(core::mem::zeroed()) as _, parameters as _) }
}
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn RtlCreateServiceSid(servicename: *const super::ntsecapi::UNICODE_STRING, servicesid: Option<super::winnt::PSID>, servicesidlength: *mut u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlCreateServiceSid(servicename : *const super::ntsecapi::UNICODE_STRING, servicesid : super::winnt::PSID, servicesidlength : *mut u32) -> super::bcrypt::NTSTATUS);
    unsafe { RtlCreateServiceSid(servicename, servicesid.unwrap_or(core::mem::zeroed()) as _, servicesidlength as _) }
}
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi"))]
#[inline]
pub unsafe fn RtlCreateSystemVolumeInformationFolder(volumerootpath: *const super::ntsecapi::UNICODE_STRING) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlCreateSystemVolumeInformationFolder(volumerootpath : *const super::ntsecapi::UNICODE_STRING) -> super::bcrypt::NTSTATUS);
    unsafe { RtlCreateSystemVolumeInformationFolder(volumerootpath) }
}
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
#[inline]
pub unsafe fn RtlCreateUnicodeString<P1>(destinationstring: *mut super::ntsecapi::UNICODE_STRING, sourcestring: P1) -> bool
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ntdll.dll" "system" fn RtlCreateUnicodeString(destinationstring : *mut super::ntsecapi::UNICODE_STRING, sourcestring : windows_core::PCWSTR) -> bool);
    unsafe { RtlCreateUnicodeString(destinationstring as _, sourcestring.param().abi()) }
}
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn RtlCreateVirtualAccountSid(name: *const super::ntsecapi::UNICODE_STRING, basesubauthority: u32, sid: super::winnt::PSID, sidlength: *mut u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlCreateVirtualAccountSid(name : *const super::ntsecapi::UNICODE_STRING, basesubauthority : u32, sid : super::winnt::PSID, sidlength : *mut u32) -> super::bcrypt::NTSTATUS);
    unsafe { RtlCreateVirtualAccountSid(name, basesubauthority, sid as _, sidlength as _) }
}
#[cfg(all(feature = "bcrypt", feature = "minwindef", feature = "ntnls"))]
#[inline]
pub unsafe fn RtlCustomCPToUnicodeN(customcp: *const super::ntnls::CPTABLEINFO, unicodestring: *mut u16, maxbytesinunicodestring: u32, bytesinunicodestring: Option<*mut u32>, customcpstring: &[u8]) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlCustomCPToUnicodeN(customcp : *const super::ntnls::CPTABLEINFO, unicodestring : *mut u16, maxbytesinunicodestring : u32, bytesinunicodestring : *mut u32, customcpstring : *const i8, bytesincustomcpstring : u32) -> super::bcrypt::NTSTATUS);
    unsafe { RtlCustomCPToUnicodeN(customcp, unicodestring as _, maxbytesinunicodestring, bytesinunicodestring.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(customcpstring.as_ptr()), customcpstring.len().try_into().unwrap()) }
}
#[cfg(feature = "bcrypt")]
#[inline]
pub unsafe fn RtlDecompressBuffer(compressionformat: u16, uncompressedbuffer: &mut [u8], compressedbuffer: &[u8], finaluncompressedsize: *mut u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlDecompressBuffer(compressionformat : u16, uncompressedbuffer : *mut u8, uncompressedbuffersize : u32, compressedbuffer : *const u8, compressedbuffersize : u32, finaluncompressedsize : *mut u32) -> super::bcrypt::NTSTATUS);
    unsafe { RtlDecompressBuffer(compressionformat, core::mem::transmute(uncompressedbuffer.as_ptr()), uncompressedbuffer.len().try_into().unwrap(), core::mem::transmute(compressedbuffer.as_ptr()), compressedbuffer.len().try_into().unwrap(), finaluncompressedsize as _) }
}
#[cfg(feature = "bcrypt")]
#[inline]
pub unsafe fn RtlDecompressBufferEx(compressionformat: u16, uncompressedbuffer: &mut [u8], compressedbuffer: &[u8], finaluncompressedsize: *mut u32, workspace: Option<*const core::ffi::c_void>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlDecompressBufferEx(compressionformat : u16, uncompressedbuffer : *mut u8, uncompressedbuffersize : u32, compressedbuffer : *const u8, compressedbuffersize : u32, finaluncompressedsize : *mut u32, workspace : *const core::ffi::c_void) -> super::bcrypt::NTSTATUS);
    unsafe { RtlDecompressBufferEx(compressionformat, core::mem::transmute(uncompressedbuffer.as_ptr()), uncompressedbuffer.len().try_into().unwrap(), core::mem::transmute(compressedbuffer.as_ptr()), compressedbuffer.len().try_into().unwrap(), finaluncompressedsize as _, workspace.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "bcrypt")]
#[inline]
pub unsafe fn RtlDecompressFragment(compressionformat: u16, uncompressedfragment: &mut [u8], compressedbuffer: &[u8], fragmentoffset: u32, finaluncompressedsize: *mut u32, workspace: *const core::ffi::c_void) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlDecompressFragment(compressionformat : u16, uncompressedfragment : *mut u8, uncompressedfragmentsize : u32, compressedbuffer : *const u8, compressedbuffersize : u32, fragmentoffset : u32, finaluncompressedsize : *mut u32, workspace : *const core::ffi::c_void) -> super::bcrypt::NTSTATUS);
    unsafe { RtlDecompressFragment(compressionformat, core::mem::transmute(uncompressedfragment.as_ptr()), uncompressedfragment.len().try_into().unwrap(), core::mem::transmute(compressedbuffer.as_ptr()), compressedbuffer.len().try_into().unwrap(), fragmentoffset, finaluncompressedsize as _, workspace) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn RtlDeleteAce(acl: *mut super::winnt::ACL, aceindex: u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlDeleteAce(acl : *mut super::winnt::ACL, aceindex : u32) -> super::bcrypt::NTSTATUS);
    unsafe { RtlDeleteAce(acl as _, aceindex) }
}
#[inline]
pub unsafe fn RtlDestroyHeap(heaphandle: *const core::ffi::c_void) -> *mut core::ffi::c_void {
    windows_core::link!("ntdll.dll" "system" fn RtlDestroyHeap(heaphandle : *const core::ffi::c_void) -> *mut core::ffi::c_void);
    unsafe { RtlDestroyHeap(heaphandle) }
}
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi"))]
#[inline]
pub unsafe fn RtlDowncaseUnicodeString(destinationstring: *mut super::ntsecapi::UNICODE_STRING, sourcestring: *const super::ntsecapi::UNICODE_STRING, allocatedestinationstring: bool) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlDowncaseUnicodeString(destinationstring : *mut super::ntsecapi::UNICODE_STRING, sourcestring : *const super::ntsecapi::UNICODE_STRING, allocatedestinationstring : bool) -> super::bcrypt::NTSTATUS);
    unsafe { RtlDowncaseUnicodeString(destinationstring as _, sourcestring, allocatedestinationstring) }
}
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi"))]
#[inline]
pub unsafe fn RtlDuplicateUnicodeString(flags: u32, stringin: *const super::ntsecapi::UNICODE_STRING, stringout: *mut super::ntsecapi::UNICODE_STRING) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlDuplicateUnicodeString(flags : u32, stringin : *const super::ntsecapi::UNICODE_STRING, stringout : *mut super::ntsecapi::UNICODE_STRING) -> super::bcrypt::NTSTATUS);
    unsafe { RtlDuplicateUnicodeString(flags, stringin, stringout as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RtlEqualPrefixSid(sid1: super::winnt::PSID, sid2: super::winnt::PSID) -> bool {
    windows_core::link!("ntdll.dll" "system" fn RtlEqualPrefixSid(sid1 : super::winnt::PSID, sid2 : super::winnt::PSID) -> bool);
    unsafe { RtlEqualPrefixSid(sid1, sid2) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RtlEqualSid(sid1: super::winnt::PSID, sid2: super::winnt::PSID) -> bool {
    windows_core::link!("ntdll.dll" "system" fn RtlEqualSid(sid1 : super::winnt::PSID, sid2 : super::winnt::PSID) -> bool);
    unsafe { RtlEqualSid(sid1, sid2) }
}
#[cfg(feature = "ntdef")]
#[inline]
pub unsafe fn RtlFreeHeap(heaphandle: *const core::ffi::c_void, flags: Option<u32>, baseaddress: *mut core::ffi::c_void) -> super::ntdef::LOGICAL {
    windows_core::link!("ntdll.dll" "system" fn RtlFreeHeap(heaphandle : *const core::ffi::c_void, flags : u32, baseaddress : *mut core::ffi::c_void) -> super::ntdef::LOGICAL);
    unsafe { RtlFreeHeap(heaphandle, flags.unwrap_or(core::mem::zeroed()) as _, baseaddress as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RtlFreeSid(sid: super::winnt::PSID) -> *mut core::ffi::c_void {
    windows_core::link!("ntdll.dll" "system" fn RtlFreeSid(sid : super::winnt::PSID) -> *mut core::ffi::c_void);
    unsafe { RtlFreeSid(sid) }
}
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi"))]
#[inline]
pub unsafe fn RtlGenerate8dot3Name(name: *const super::ntsecapi::UNICODE_STRING, allowextendedcharacters: bool, context: *mut GENERATE_NAME_CONTEXT, name8dot3: *mut super::ntsecapi::UNICODE_STRING) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlGenerate8dot3Name(name : *const super::ntsecapi::UNICODE_STRING, allowextendedcharacters : bool, context : *mut GENERATE_NAME_CONTEXT, name8dot3 : *mut super::ntsecapi::UNICODE_STRING) -> super::bcrypt::NTSTATUS);
    unsafe { RtlGenerate8dot3Name(name, allowextendedcharacters, context as _, name8dot3 as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn RtlGetAce(acl: *const super::winnt::ACL, aceindex: u32, ace: *mut *mut core::ffi::c_void) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlGetAce(acl : *const super::winnt::ACL, aceindex : u32, ace : *mut *mut core::ffi::c_void) -> super::bcrypt::NTSTATUS);
    unsafe { RtlGetAce(acl, aceindex, ace as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn RtlGetAcesBufferSize(acl: *const super::winnt::ACL, acesbuffersize: *mut u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlGetAcesBufferSize(acl : *const super::winnt::ACL, acesbuffersize : *mut u32) -> super::bcrypt::NTSTATUS);
    unsafe { RtlGetAcesBufferSize(acl, acesbuffersize as _) }
}
#[cfg(feature = "bcrypt")]
#[inline]
pub unsafe fn RtlGetCompressionWorkSpaceSize(compressionformatandengine: u16, compressbufferworkspacesize: *mut u32, compressfragmentworkspacesize: *mut u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlGetCompressionWorkSpaceSize(compressionformatandengine : u16, compressbufferworkspacesize : *mut u32, compressfragmentworkspacesize : *mut u32) -> super::bcrypt::NTSTATUS);
    unsafe { RtlGetCompressionWorkSpaceSize(compressionformatandengine, compressbufferworkspacesize as _, compressfragmentworkspacesize as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn RtlGetDaclSecurityDescriptor(securitydescriptor: super::winnt::PSECURITY_DESCRIPTOR, daclpresent: *mut bool, dacl: *mut super::winnt::PACL, dacldefaulted: *mut bool) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlGetDaclSecurityDescriptor(securitydescriptor : super::winnt::PSECURITY_DESCRIPTOR, daclpresent : *mut bool, dacl : *mut super::winnt::PACL, dacldefaulted : *mut bool) -> super::bcrypt::NTSTATUS);
    unsafe { RtlGetDaclSecurityDescriptor(securitydescriptor, daclpresent as _, dacl as _, dacldefaulted as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn RtlGetGroupSecurityDescriptor(securitydescriptor: super::winnt::PSECURITY_DESCRIPTOR, group: *mut super::winnt::PSID, groupdefaulted: *mut bool) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlGetGroupSecurityDescriptor(securitydescriptor : super::winnt::PSECURITY_DESCRIPTOR, group : *mut super::winnt::PSID, groupdefaulted : *mut bool) -> super::bcrypt::NTSTATUS);
    unsafe { RtlGetGroupSecurityDescriptor(securitydescriptor, group as _, groupdefaulted as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn RtlGetOwnerSecurityDescriptor(securitydescriptor: super::winnt::PSECURITY_DESCRIPTOR, owner: *mut super::winnt::PSID, ownerdefaulted: *mut bool) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlGetOwnerSecurityDescriptor(securitydescriptor : super::winnt::PSECURITY_DESCRIPTOR, owner : *mut super::winnt::PSID, ownerdefaulted : *mut bool) -> super::bcrypt::NTSTATUS);
    unsafe { RtlGetOwnerSecurityDescriptor(securitydescriptor, owner as _, ownerdefaulted as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn RtlGetSaclSecurityDescriptor(securitydescriptor: super::winnt::PSECURITY_DESCRIPTOR, saclpresent: *mut bool, sacl: *mut super::winnt::PACL, sacldefaulted: *mut bool) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlGetSaclSecurityDescriptor(securitydescriptor : super::winnt::PSECURITY_DESCRIPTOR, saclpresent : *mut bool, sacl : *mut super::winnt::PACL, sacldefaulted : *mut bool) -> super::bcrypt::NTSTATUS);
    unsafe { RtlGetSaclSecurityDescriptor(securitydescriptor, saclpresent as _, sacl as _, sacldefaulted as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RtlIdentifierAuthoritySid(sid: super::winnt::PSID) -> super::winnt::PSID_IDENTIFIER_AUTHORITY {
    windows_core::link!("ntdll.dll" "system" fn RtlIdentifierAuthoritySid(sid : super::winnt::PSID) -> super::winnt::PSID_IDENTIFIER_AUTHORITY);
    unsafe { RtlIdentifierAuthoritySid(sid) }
}
#[cfg(feature = "bcrypt")]
#[inline]
pub unsafe fn RtlIdnToAscii<P1>(flags: u32, sourcestring: P1, sourcestringlength: i32, destinationstring: windows_core::PWSTR, destinationstringlength: *mut i32) -> super::bcrypt::NTSTATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ntdll.dll" "system" fn RtlIdnToAscii(flags : u32, sourcestring : windows_core::PCWSTR, sourcestringlength : i32, destinationstring : windows_core::PWSTR, destinationstringlength : *mut i32) -> super::bcrypt::NTSTATUS);
    unsafe { RtlIdnToAscii(flags, sourcestring.param().abi(), sourcestringlength, core::mem::transmute(destinationstring), destinationstringlength as _) }
}
#[cfg(feature = "bcrypt")]
#[inline]
pub unsafe fn RtlIdnToNameprepUnicode<P1>(flags: u32, sourcestring: P1, sourcestringlength: i32, destinationstring: windows_core::PWSTR, destinationstringlength: *mut i32) -> super::bcrypt::NTSTATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ntdll.dll" "system" fn RtlIdnToNameprepUnicode(flags : u32, sourcestring : windows_core::PCWSTR, sourcestringlength : i32, destinationstring : windows_core::PWSTR, destinationstringlength : *mut i32) -> super::bcrypt::NTSTATUS);
    unsafe { RtlIdnToNameprepUnicode(flags, sourcestring.param().abi(), sourcestringlength, core::mem::transmute(destinationstring), destinationstringlength as _) }
}
#[cfg(feature = "bcrypt")]
#[inline]
pub unsafe fn RtlIdnToUnicode<P1>(flags: u32, sourcestring: P1, sourcestringlength: i32, destinationstring: windows_core::PWSTR, destinationstringlength: *mut i32) -> super::bcrypt::NTSTATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ntdll.dll" "system" fn RtlIdnToUnicode(flags : u32, sourcestring : windows_core::PCWSTR, sourcestringlength : i32, destinationstring : windows_core::PWSTR, destinationstringlength : *mut i32) -> super::bcrypt::NTSTATUS);
    unsafe { RtlIdnToUnicode(flags, sourcestring.param().abi(), sourcestringlength, core::mem::transmute(destinationstring), destinationstringlength as _) }
}
#[cfg(all(feature = "minwindef", feature = "ntnls"))]
#[inline]
pub unsafe fn RtlInitCodePageTable(tablebase: Option<&[u16; 2]>, codepagetable: *mut super::ntnls::CPTABLEINFO) {
    windows_core::link!("ntdll.dll" "system" fn RtlInitCodePageTable(tablebase : *const u16, codepagetable : *mut super::ntnls::CPTABLEINFO));
    unsafe { RtlInitCodePageTable(core::mem::transmute(tablebase.map_or(core::ptr::null(), |slice| slice.as_ptr())), codepagetable as _) }
}
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi"))]
#[inline]
pub unsafe fn RtlInitUnicodeStringEx<P1>(destinationstring: *mut super::ntsecapi::UNICODE_STRING, sourcestring: P1) -> super::bcrypt::NTSTATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ntdll.dll" "system" fn RtlInitUnicodeStringEx(destinationstring : *mut super::ntsecapi::UNICODE_STRING, sourcestring : windows_core::PCWSTR) -> super::bcrypt::NTSTATUS);
    unsafe { RtlInitUnicodeStringEx(destinationstring as _, sourcestring.param().abi()) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn RtlInitializeSid(sid: super::winnt::PSID, identifierauthority: *const super::winnt::SID_IDENTIFIER_AUTHORITY, subauthoritycount: u8) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlInitializeSid(sid : super::winnt::PSID, identifierauthority : *const super::winnt::SID_IDENTIFIER_AUTHORITY, subauthoritycount : u8) -> super::bcrypt::NTSTATUS);
    unsafe { RtlInitializeSid(sid as _, identifierauthority, subauthoritycount) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn RtlInitializeSidEx(sid: super::winnt::PSID, identifierauthority: *const super::winnt::SID_IDENTIFIER_AUTHORITY, subauthoritycount: u8) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "C" fn RtlInitializeSidEx(sid : super::winnt::PSID, identifierauthority : *const super::winnt::SID_IDENTIFIER_AUTHORITY, subauthoritycount : u8) -> super::bcrypt::NTSTATUS);
    unsafe { RtlInitializeSidEx(sid as _, identifierauthority, subauthoritycount) }
}
#[inline]
pub unsafe fn RtlIsCloudFilesPlaceholder(fileattributes: u32, reparsetag: u32) -> bool {
    windows_core::link!("ntdll.dll" "system" fn RtlIsCloudFilesPlaceholder(fileattributes : u32, reparsetag : u32) -> bool);
    unsafe { RtlIsCloudFilesPlaceholder(fileattributes, reparsetag) }
}
#[inline]
pub unsafe fn RtlIsNonEmptyDirectoryReparsePointAllowed(reparsetag: u32) -> bool {
    windows_core::link!("ntdll.dll" "system" fn RtlIsNonEmptyDirectoryReparsePointAllowed(reparsetag : u32) -> bool);
    unsafe { RtlIsNonEmptyDirectoryReparsePointAllowed(reparsetag) }
}
#[cfg(feature = "bcrypt")]
#[inline]
pub unsafe fn RtlIsNormalizedString<P1>(normform: u32, sourcestring: P1, sourcestringlength: i32, normalized: *mut bool) -> super::bcrypt::NTSTATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ntdll.dll" "system" fn RtlIsNormalizedString(normform : u32, sourcestring : windows_core::PCWSTR, sourcestringlength : i32, normalized : *mut bool) -> super::bcrypt::NTSTATUS);
    unsafe { RtlIsNormalizedString(normform, sourcestring.param().abi(), sourcestringlength, normalized as _) }
}
#[inline]
pub unsafe fn RtlIsPartialPlaceholder(fileattributes: u32, reparsetag: u32) -> bool {
    windows_core::link!("ntdll.dll" "system" fn RtlIsPartialPlaceholder(fileattributes : u32, reparsetag : u32) -> bool);
    unsafe { RtlIsPartialPlaceholder(fileattributes, reparsetag) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn RtlIsPartialPlaceholderFileHandle(filehandle: super::winnt::HANDLE, ispartialplaceholder: *mut bool) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlIsPartialPlaceholderFileHandle(filehandle : super::winnt::HANDLE, ispartialplaceholder : *mut bool) -> super::bcrypt::NTSTATUS);
    unsafe { RtlIsPartialPlaceholderFileHandle(filehandle, ispartialplaceholder as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winternl"))]
#[inline]
pub unsafe fn RtlIsPartialPlaceholderFileInfo(infobuffer: *const core::ffi::c_void, infoclass: super::winternl::FILE_INFORMATION_CLASS, ispartialplaceholder: *mut bool) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlIsPartialPlaceholderFileInfo(infobuffer : *const core::ffi::c_void, infoclass : super::winternl::FILE_INFORMATION_CLASS, ispartialplaceholder : *mut bool) -> super::bcrypt::NTSTATUS);
    unsafe { RtlIsPartialPlaceholderFileInfo(infobuffer, infoclass, ispartialplaceholder as _) }
}
#[inline]
pub unsafe fn RtlLengthRequiredSid(subauthoritycount: u32) -> u32 {
    windows_core::link!("ntdll.dll" "system" fn RtlLengthRequiredSid(subauthoritycount : u32) -> u32);
    unsafe { RtlLengthRequiredSid(subauthoritycount) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RtlLengthSid(sid: super::winnt::PSID) -> u32 {
    windows_core::link!("ntdll.dll" "system" fn RtlLengthSid(sid : super::winnt::PSID) -> u32);
    unsafe { RtlLengthSid(sid) }
}
#[cfg(feature = "bcrypt")]
#[inline]
pub unsafe fn RtlMultiByteToUnicodeN(unicodestring: *mut u16, maxbytesinunicodestring: u32, bytesinunicodestring: Option<*mut u32>, multibytestring: &[u8]) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlMultiByteToUnicodeN(unicodestring : *mut u16, maxbytesinunicodestring : u32, bytesinunicodestring : *mut u32, multibytestring : *const i8, bytesinmultibytestring : u32) -> super::bcrypt::NTSTATUS);
    unsafe { RtlMultiByteToUnicodeN(unicodestring as _, maxbytesinunicodestring, bytesinunicodestring.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(multibytestring.as_ptr()), multibytestring.len().try_into().unwrap()) }
}
#[cfg(feature = "bcrypt")]
#[inline]
pub unsafe fn RtlMultiByteToUnicodeSize(bytesinunicodestring: *mut u32, multibytestring: &[u8]) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlMultiByteToUnicodeSize(bytesinunicodestring : *mut u32, multibytestring : *const i8, bytesinmultibytestring : u32) -> super::bcrypt::NTSTATUS);
    unsafe { RtlMultiByteToUnicodeSize(bytesinunicodestring as _, core::mem::transmute(multibytestring.as_ptr()), multibytestring.len().try_into().unwrap()) }
}
#[cfg(feature = "bcrypt")]
#[inline]
pub unsafe fn RtlNormalizeString<P1>(normform: u32, sourcestring: P1, sourcestringlength: i32, destinationstring: windows_core::PWSTR, destinationstringlength: *mut i32) -> super::bcrypt::NTSTATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ntdll.dll" "system" fn RtlNormalizeString(normform : u32, sourcestring : windows_core::PCWSTR, sourcestringlength : i32, destinationstring : windows_core::PWSTR, destinationstringlength : *mut i32) -> super::bcrypt::NTSTATUS);
    unsafe { RtlNormalizeString(normform, sourcestring.param().abi(), sourcestringlength, core::mem::transmute(destinationstring), destinationstringlength as _) }
}
#[cfg(feature = "bcrypt")]
#[inline]
pub unsafe fn RtlNtStatusToDosErrorNoTeb(status: super::bcrypt::NTSTATUS) -> u32 {
    windows_core::link!("ntdll.dll" "system" fn RtlNtStatusToDosErrorNoTeb(status : super::bcrypt::NTSTATUS) -> u32);
    unsafe { RtlNtStatusToDosErrorNoTeb(status) }
}
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn RtlOemStringToUnicodeString(destinationstring: *mut super::ntsecapi::UNICODE_STRING, sourcestring: *const super::ntsecapi::STRING, allocatedestinationstring: bool) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlOemStringToUnicodeString(destinationstring : *mut super::ntsecapi::UNICODE_STRING, sourcestring : *const super::ntsecapi::STRING, allocatedestinationstring : bool) -> super::bcrypt::NTSTATUS);
    unsafe { RtlOemStringToUnicodeString(destinationstring as _, sourcestring, allocatedestinationstring) }
}
#[cfg(feature = "bcrypt")]
#[inline]
pub unsafe fn RtlOemToUnicodeN(unicodestring: *mut u16, maxbytesinunicodestring: u32, bytesinunicodestring: Option<*mut u32>, oemstring: &[u8]) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlOemToUnicodeN(unicodestring : *mut u16, maxbytesinunicodestring : u32, bytesinunicodestring : *mut u32, oemstring : *const i8, bytesinoemstring : u32) -> super::bcrypt::NTSTATUS);
    unsafe { RtlOemToUnicodeN(unicodestring as _, maxbytesinunicodestring, bytesinunicodestring.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(oemstring.as_ptr()), oemstring.len().try_into().unwrap()) }
}
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn RtlPrefixString(string1: *const super::ntsecapi::STRING, string2: *const super::ntsecapi::STRING, caseinsensitive: bool) -> bool {
    windows_core::link!("ntdll.dll" "system" fn RtlPrefixString(string1 : *const super::ntsecapi::STRING, string2 : *const super::ntsecapi::STRING, caseinsensitive : bool) -> bool);
    unsafe { RtlPrefixString(string1, string2, caseinsensitive) }
}
#[cfg(feature = "bcrypt")]
#[inline]
pub unsafe fn RtlQueryPackageIdentity(tokenobject: *const core::ffi::c_void, packagefullname: windows_core::PWSTR, packagesize: *mut usize, appid: Option<windows_core::PWSTR>, appidsize: Option<*mut usize>, packaged: Option<*mut bool>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlQueryPackageIdentity(tokenobject : *const core::ffi::c_void, packagefullname : windows_core::PWSTR, packagesize : *mut usize, appid : windows_core::PWSTR, appidsize : *mut usize, packaged : *mut bool) -> super::bcrypt::NTSTATUS);
    unsafe { RtlQueryPackageIdentity(tokenobject, core::mem::transmute(packagefullname), packagesize as _, appid.unwrap_or(core::mem::zeroed()) as _, appidsize.unwrap_or(core::mem::zeroed()) as _, packaged.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "bcrypt")]
#[inline]
pub unsafe fn RtlQueryPackageIdentityEx(tokenobject: *const core::ffi::c_void, packagefullname: windows_core::PWSTR, packagesize: *mut usize, appid: Option<windows_core::PWSTR>, appidsize: Option<*mut usize>, dynamicid: Option<*mut windows_core::GUID>, flags: Option<*mut u64>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlQueryPackageIdentityEx(tokenobject : *const core::ffi::c_void, packagefullname : windows_core::PWSTR, packagesize : *mut usize, appid : windows_core::PWSTR, appidsize : *mut usize, dynamicid : *mut windows_core::GUID, flags : *mut u64) -> super::bcrypt::NTSTATUS);
    unsafe { RtlQueryPackageIdentityEx(tokenobject, core::mem::transmute(packagefullname), packagesize as _, appid.unwrap_or(core::mem::zeroed()) as _, appidsize.unwrap_or(core::mem::zeroed()) as _, dynamicid.unwrap_or(core::mem::zeroed()) as _, flags.unwrap_or(core::mem::zeroed()) as _) }
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
#[inline]
pub unsafe fn RtlRandom(seed: *mut u32) -> u32 {
    windows_core::link!("ntdll.dll" "system" fn RtlRandom(seed : *mut u32) -> u32);
    unsafe { RtlRandom(seed as _) }
}
#[inline]
pub unsafe fn RtlRandomEx(seed: *mut u32) -> u32 {
    windows_core::link!("ntdll.dll" "system" fn RtlRandomEx(seed : *mut u32) -> u32);
    unsafe { RtlRandomEx(seed as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn RtlReplaceSidInSd(securitydescriptor: super::winnt::PSECURITY_DESCRIPTOR, oldsid: super::winnt::PSID, newsid: super::winnt::PSID, numchanges: *mut u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlReplaceSidInSd(securitydescriptor : super::winnt::PSECURITY_DESCRIPTOR, oldsid : super::winnt::PSID, newsid : super::winnt::PSID, numchanges : *mut u32) -> super::bcrypt::NTSTATUS);
    unsafe { RtlReplaceSidInSd(securitydescriptor as _, oldsid, newsid, numchanges as _) }
}
#[inline]
pub unsafe fn RtlSecondsSince1970ToTime(elapsedseconds: u32) -> i64 {
    windows_core::link!("ntdll.dll" "system" fn RtlSecondsSince1970ToTime(elapsedseconds : u32, time : *mut i64));
    unsafe {
        let mut result__ = core::mem::zeroed();
        RtlSecondsSince1970ToTime(elapsedseconds, &mut result__);
        result__
    }
}
#[inline]
pub unsafe fn RtlSecondsSince1980ToTime(elapsedseconds: u32) -> i64 {
    windows_core::link!("ntdll.dll" "system" fn RtlSecondsSince1980ToTime(elapsedseconds : u32, time : *mut i64));
    unsafe {
        let mut result__ = core::mem::zeroed();
        RtlSecondsSince1980ToTime(elapsedseconds, &mut result__);
        result__
    }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn RtlSelfRelativeToAbsoluteSD(selfrelativesecuritydescriptor: super::winnt::PSECURITY_DESCRIPTOR, absolutesecuritydescriptor: Option<super::winnt::PSECURITY_DESCRIPTOR>, absolutesecuritydescriptorsize: *mut u32, dacl: Option<*mut super::winnt::ACL>, daclsize: *mut u32, sacl: Option<*mut super::winnt::ACL>, saclsize: *mut u32, owner: Option<super::winnt::PSID>, ownersize: *mut u32, primarygroup: Option<super::winnt::PSID>, primarygroupsize: *mut u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlSelfRelativeToAbsoluteSD(selfrelativesecuritydescriptor : super::winnt::PSECURITY_DESCRIPTOR, absolutesecuritydescriptor : super::winnt::PSECURITY_DESCRIPTOR, absolutesecuritydescriptorsize : *mut u32, dacl : *mut super::winnt::ACL, daclsize : *mut u32, sacl : *mut super::winnt::ACL, saclsize : *mut u32, owner : super::winnt::PSID, ownersize : *mut u32, primarygroup : super::winnt::PSID, primarygroupsize : *mut u32) -> super::bcrypt::NTSTATUS);
    unsafe { RtlSelfRelativeToAbsoluteSD(selfrelativesecuritydescriptor, absolutesecuritydescriptor.unwrap_or(core::mem::zeroed()) as _, absolutesecuritydescriptorsize as _, dacl.unwrap_or(core::mem::zeroed()) as _, daclsize as _, sacl.unwrap_or(core::mem::zeroed()) as _, saclsize as _, owner.unwrap_or(core::mem::zeroed()) as _, ownersize as _, primarygroup.unwrap_or(core::mem::zeroed()) as _, primarygroupsize as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn RtlSetGroupSecurityDescriptor(securitydescriptor: super::winnt::PSECURITY_DESCRIPTOR, group: Option<super::winnt::PSID>, groupdefaulted: bool) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlSetGroupSecurityDescriptor(securitydescriptor : super::winnt::PSECURITY_DESCRIPTOR, group : super::winnt::PSID, groupdefaulted : bool) -> super::bcrypt::NTSTATUS);
    unsafe { RtlSetGroupSecurityDescriptor(securitydescriptor as _, group.unwrap_or(core::mem::zeroed()) as _, groupdefaulted) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn RtlSetOwnerSecurityDescriptor(securitydescriptor: super::winnt::PSECURITY_DESCRIPTOR, owner: Option<super::winnt::PSID>, ownerdefaulted: bool) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlSetOwnerSecurityDescriptor(securitydescriptor : super::winnt::PSECURITY_DESCRIPTOR, owner : super::winnt::PSID, ownerdefaulted : bool) -> super::bcrypt::NTSTATUS);
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
pub unsafe fn RtlSubAuthorityCountSid(sid: super::winnt::PSID) -> super::minwindef::PUCHAR {
    windows_core::link!("ntdll.dll" "system" fn RtlSubAuthorityCountSid(sid : super::winnt::PSID) -> super::minwindef::PUCHAR);
    unsafe { RtlSubAuthorityCountSid(sid) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn RtlSubAuthoritySid(sid: super::winnt::PSID, subauthority: u32) -> super::minwindef::PULONG {
    windows_core::link!("ntdll.dll" "system" fn RtlSubAuthoritySid(sid : super::winnt::PSID, subauthority : u32) -> super::minwindef::PULONG);
    unsafe { RtlSubAuthoritySid(sid, subauthority) }
}
#[inline]
pub unsafe fn RtlTimeToSecondsSince1980(time: *const i64, elapsedseconds: *mut u32) -> bool {
    windows_core::link!("ntdll.dll" "system" fn RtlTimeToSecondsSince1980(time : *const i64, elapsedseconds : *mut u32) -> bool);
    unsafe { RtlTimeToSecondsSince1980(time, elapsedseconds as _) }
}
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn RtlUnicodeStringToCountedOemString(destinationstring: super::winternl::POEM_STRING, sourcestring: *const super::ntsecapi::UNICODE_STRING, allocatedestinationstring: bool) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlUnicodeStringToCountedOemString(destinationstring : super::winternl::POEM_STRING, sourcestring : *const super::ntsecapi::UNICODE_STRING, allocatedestinationstring : bool) -> super::bcrypt::NTSTATUS);
    unsafe { RtlUnicodeStringToCountedOemString(destinationstring, sourcestring, allocatedestinationstring) }
}
#[cfg(all(feature = "bcrypt", feature = "minwindef", feature = "ntnls"))]
#[inline]
pub unsafe fn RtlUnicodeToCustomCPN(customcp: *const super::ntnls::CPTABLEINFO, customcpstring: &mut [u8], bytesincustomcpstring: Option<*mut u32>, unicodestring: *const u16, bytesinunicodestring: u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlUnicodeToCustomCPN(customcp : *const super::ntnls::CPTABLEINFO, customcpstring : *mut i8, maxbytesincustomcpstring : u32, bytesincustomcpstring : *mut u32, unicodestring : *const u16, bytesinunicodestring : u32) -> super::bcrypt::NTSTATUS);
    unsafe { RtlUnicodeToCustomCPN(customcp, core::mem::transmute(customcpstring.as_ptr()), customcpstring.len().try_into().unwrap(), bytesincustomcpstring.unwrap_or(core::mem::zeroed()) as _, unicodestring, bytesinunicodestring) }
}
#[cfg(feature = "bcrypt")]
#[inline]
pub unsafe fn RtlUnicodeToMultiByteN(multibytestring: &mut [u8], bytesinmultibytestring: Option<*mut u32>, unicodestring: *const u16, bytesinunicodestring: u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlUnicodeToMultiByteN(multibytestring : *mut i8, maxbytesinmultibytestring : u32, bytesinmultibytestring : *mut u32, unicodestring : *const u16, bytesinunicodestring : u32) -> super::bcrypt::NTSTATUS);
    unsafe { RtlUnicodeToMultiByteN(core::mem::transmute(multibytestring.as_ptr()), multibytestring.len().try_into().unwrap(), bytesinmultibytestring.unwrap_or(core::mem::zeroed()) as _, unicodestring, bytesinunicodestring) }
}
#[cfg(feature = "bcrypt")]
#[inline]
pub unsafe fn RtlUnicodeToOemN(oemstring: &mut [u8], bytesinoemstring: Option<*mut u32>, unicodestring: *const u16, bytesinunicodestring: u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlUnicodeToOemN(oemstring : *mut i8, maxbytesinoemstring : u32, bytesinoemstring : *mut u32, unicodestring : *const u16, bytesinunicodestring : u32) -> super::bcrypt::NTSTATUS);
    unsafe { RtlUnicodeToOemN(core::mem::transmute(oemstring.as_ptr()), oemstring.len().try_into().unwrap(), bytesinoemstring.unwrap_or(core::mem::zeroed()) as _, unicodestring, bytesinunicodestring) }
}
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn RtlUpcaseUnicodeStringToCountedOemString(destinationstring: super::winternl::POEM_STRING, sourcestring: *const super::ntsecapi::UNICODE_STRING, allocatedestinationstring: bool) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlUpcaseUnicodeStringToCountedOemString(destinationstring : super::winternl::POEM_STRING, sourcestring : *const super::ntsecapi::UNICODE_STRING, allocatedestinationstring : bool) -> super::bcrypt::NTSTATUS);
    unsafe { RtlUpcaseUnicodeStringToCountedOemString(destinationstring, sourcestring, allocatedestinationstring) }
}
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn RtlUpcaseUnicodeStringToOemString(destinationstring: super::winternl::POEM_STRING, sourcestring: *const super::ntsecapi::UNICODE_STRING, allocatedestinationstring: bool) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlUpcaseUnicodeStringToOemString(destinationstring : super::winternl::POEM_STRING, sourcestring : *const super::ntsecapi::UNICODE_STRING, allocatedestinationstring : bool) -> super::bcrypt::NTSTATUS);
    unsafe { RtlUpcaseUnicodeStringToOemString(destinationstring, sourcestring, allocatedestinationstring) }
}
#[cfg(all(feature = "bcrypt", feature = "minwindef", feature = "ntnls"))]
#[inline]
pub unsafe fn RtlUpcaseUnicodeToCustomCPN(customcp: *const super::ntnls::CPTABLEINFO, customcpstring: &mut [u8], bytesincustomcpstring: Option<*mut u32>, unicodestring: *const u16, bytesinunicodestring: u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlUpcaseUnicodeToCustomCPN(customcp : *const super::ntnls::CPTABLEINFO, customcpstring : *mut i8, maxbytesincustomcpstring : u32, bytesincustomcpstring : *mut u32, unicodestring : *const u16, bytesinunicodestring : u32) -> super::bcrypt::NTSTATUS);
    unsafe { RtlUpcaseUnicodeToCustomCPN(customcp, core::mem::transmute(customcpstring.as_ptr()), customcpstring.len().try_into().unwrap(), bytesincustomcpstring.unwrap_or(core::mem::zeroed()) as _, unicodestring, bytesinunicodestring) }
}
#[cfg(feature = "bcrypt")]
#[inline]
pub unsafe fn RtlUpcaseUnicodeToMultiByteN(multibytestring: &mut [u8], bytesinmultibytestring: Option<*mut u32>, unicodestring: *const u16, bytesinunicodestring: u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlUpcaseUnicodeToMultiByteN(multibytestring : *mut i8, maxbytesinmultibytestring : u32, bytesinmultibytestring : *mut u32, unicodestring : *const u16, bytesinunicodestring : u32) -> super::bcrypt::NTSTATUS);
    unsafe { RtlUpcaseUnicodeToMultiByteN(core::mem::transmute(multibytestring.as_ptr()), multibytestring.len().try_into().unwrap(), bytesinmultibytestring.unwrap_or(core::mem::zeroed()) as _, unicodestring, bytesinunicodestring) }
}
#[cfg(feature = "bcrypt")]
#[inline]
pub unsafe fn RtlUpcaseUnicodeToOemN(oemstring: &mut [u8], bytesinoemstring: Option<*mut u32>, unicodestring: *const u16, bytesinunicodestring: u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlUpcaseUnicodeToOemN(oemstring : *mut i8, maxbytesinoemstring : u32, bytesinoemstring : *mut u32, unicodestring : *const u16, bytesinunicodestring : u32) -> super::bcrypt::NTSTATUS);
    unsafe { RtlUpcaseUnicodeToOemN(core::mem::transmute(oemstring.as_ptr()), oemstring.len().try_into().unwrap(), bytesinoemstring.unwrap_or(core::mem::zeroed()) as _, unicodestring, bytesinunicodestring) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RtlValidSid(sid: super::winnt::PSID) -> bool {
    windows_core::link!("ntdll.dll" "system" fn RtlValidSid(sid : super::winnt::PSID) -> bool);
    unsafe { RtlValidSid(sid) }
}
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi"))]
#[inline]
pub unsafe fn RtlValidateUnicodeString(flags: Option<u32>, string: *const super::ntsecapi::UNICODE_STRING) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlValidateUnicodeString(flags : u32, string : *const super::ntsecapi::UNICODE_STRING) -> super::bcrypt::NTSTATUS);
    unsafe { RtlValidateUnicodeString(flags.unwrap_or(core::mem::zeroed()) as _, string) }
}
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn RtlxOemStringToUnicodeSize(oemstring: *const super::ntsecapi::STRING) -> u32 {
    windows_core::link!("ntdll.dll" "system" fn RtlxOemStringToUnicodeSize(oemstring : *const super::ntsecapi::STRING) -> u32);
    unsafe { RtlxOemStringToUnicodeSize(oemstring) }
}
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
#[inline]
pub unsafe fn RtlxUnicodeStringToOemSize(unicodestring: *const super::ntsecapi::UNICODE_STRING) -> u32 {
    windows_core::link!("ntdll.dll" "system" fn RtlxUnicodeStringToOemSize(unicodestring : *const super::ntsecapi::UNICODE_STRING) -> u32);
    unsafe { RtlxUnicodeStringToOemSize(unicodestring) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn ZwAllocateVirtualMemory(processhandle: super::winnt::HANDLE, baseaddress: *mut *mut core::ffi::c_void, zerobits: usize, regionsize: *mut usize, allocationtype: u32, protect: u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwAllocateVirtualMemory(processhandle : super::winnt::HANDLE, baseaddress : *mut *mut core::ffi::c_void, zerobits : usize, regionsize : *mut usize, allocationtype : u32, protect : u32) -> super::bcrypt::NTSTATUS);
    unsafe { ZwAllocateVirtualMemory(processhandle, baseaddress as _, zerobits, regionsize as _, allocationtype, protect) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn ZwAllocateVirtualMemoryEx(processhandle: super::winnt::HANDLE, baseaddress: *mut *mut core::ffi::c_void, regionsize: *mut usize, allocationtype: u32, pageprotection: u32, extendedparameters: Option<&mut [super::winnt::MEM_EXTENDED_PARAMETER]>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwAllocateVirtualMemoryEx(processhandle : super::winnt::HANDLE, baseaddress : *mut *mut core::ffi::c_void, regionsize : *mut usize, allocationtype : u32, pageprotection : u32, extendedparameters : *mut super::winnt::MEM_EXTENDED_PARAMETER, extendedparametercount : u32) -> super::bcrypt::NTSTATUS);
    unsafe { ZwAllocateVirtualMemoryEx(processhandle, baseaddress as _, regionsize as _, allocationtype, pageprotection, core::mem::transmute(extendedparameters.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), extendedparameters.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[cfg(all(feature = "bcrypt", feature = "d3dkmthk", feature = "lsalookup", feature = "ntdef", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn ZwCreateEvent(eventhandle: *mut super::winnt::HANDLE, desiredaccess: super::winnt::ACCESS_MASK, objectattributes: Option<*const super::d3dkmthk::OBJECT_ATTRIBUTES>, eventtype: super::ntdef::EVENT_TYPE, initialstate: bool) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwCreateEvent(eventhandle : *mut super::winnt::HANDLE, desiredaccess : super::winnt::ACCESS_MASK, objectattributes : *const super::d3dkmthk::OBJECT_ATTRIBUTES, eventtype : super::ntdef::EVENT_TYPE, initialstate : bool) -> super::bcrypt::NTSTATUS);
    unsafe { ZwCreateEvent(eventhandle as _, desiredaccess, objectattributes.unwrap_or(core::mem::zeroed()) as _, eventtype, initialstate) }
}
#[cfg(all(feature = "bcrypt", feature = "d3dkmthk", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn ZwDeleteFile(objectattributes: *const super::d3dkmthk::OBJECT_ATTRIBUTES) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwDeleteFile(objectattributes : *const super::d3dkmthk::OBJECT_ATTRIBUTES) -> super::bcrypt::NTSTATUS);
    unsafe { ZwDeleteFile(objectattributes) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn ZwDuplicateObject(sourceprocesshandle: super::winnt::HANDLE, sourcehandle: super::winnt::HANDLE, targetprocesshandle: Option<super::winnt::HANDLE>, targethandle: Option<*mut super::winnt::HANDLE>, desiredaccess: super::winnt::ACCESS_MASK, handleattributes: u32, options: u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwDuplicateObject(sourceprocesshandle : super::winnt::HANDLE, sourcehandle : super::winnt::HANDLE, targetprocesshandle : super::winnt::HANDLE, targethandle : *mut super::winnt::HANDLE, desiredaccess : super::winnt::ACCESS_MASK, handleattributes : u32, options : u32) -> super::bcrypt::NTSTATUS);
    unsafe { ZwDuplicateObject(sourceprocesshandle, sourcehandle, targetprocesshandle.unwrap_or(core::mem::zeroed()) as _, targethandle.unwrap_or(core::mem::zeroed()) as _, desiredaccess, handleattributes, options) }
}
#[cfg(all(feature = "bcrypt", feature = "d3dkmthk", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn ZwDuplicateToken(existingtokenhandle: super::winnt::HANDLE, desiredaccess: super::winnt::ACCESS_MASK, objectattributes: Option<*const super::d3dkmthk::OBJECT_ATTRIBUTES>, effectiveonly: bool, tokentype: super::winnt::TOKEN_TYPE, newtokenhandle: *mut super::winnt::HANDLE) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwDuplicateToken(existingtokenhandle : super::winnt::HANDLE, desiredaccess : super::winnt::ACCESS_MASK, objectattributes : *const super::d3dkmthk::OBJECT_ATTRIBUTES, effectiveonly : bool, tokentype : super::winnt::TOKEN_TYPE, newtokenhandle : *mut super::winnt::HANDLE) -> super::bcrypt::NTSTATUS);
    unsafe { ZwDuplicateToken(existingtokenhandle, desiredaccess, objectattributes.unwrap_or(core::mem::zeroed()) as _, effectiveonly, tokentype, newtokenhandle as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn ZwFlushBuffersFile(filehandle: super::winnt::HANDLE, iostatusblock: *mut super::winternl::IO_STATUS_BLOCK) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "C" fn ZwFlushBuffersFile(filehandle : super::winnt::HANDLE, iostatusblock : *mut super::winternl::IO_STATUS_BLOCK) -> super::bcrypt::NTSTATUS);
    unsafe { ZwFlushBuffersFile(filehandle, iostatusblock as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn ZwFlushBuffersFileEx(filehandle: super::winnt::HANDLE, flags: u32, parameters: *const core::ffi::c_void, parameterssize: u32, iostatusblock: *mut super::winternl::IO_STATUS_BLOCK) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "C" fn ZwFlushBuffersFileEx(filehandle : super::winnt::HANDLE, flags : u32, parameters : *const core::ffi::c_void, parameterssize : u32, iostatusblock : *mut super::winternl::IO_STATUS_BLOCK) -> super::bcrypt::NTSTATUS);
    unsafe { ZwFlushBuffersFileEx(filehandle, flags, parameters, parameterssize, iostatusblock as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn ZwFlushVirtualMemory(processhandle: super::winnt::HANDLE, baseaddress: *mut *mut core::ffi::c_void, regionsize: *mut usize, iostatus: *mut super::winternl::IO_STATUS_BLOCK) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwFlushVirtualMemory(processhandle : super::winnt::HANDLE, baseaddress : *mut *mut core::ffi::c_void, regionsize : *mut usize, iostatus : *mut super::winternl::IO_STATUS_BLOCK) -> super::bcrypt::NTSTATUS);
    unsafe { ZwFlushVirtualMemory(processhandle, baseaddress as _, regionsize as _, iostatus as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn ZwFreeVirtualMemory(processhandle: super::winnt::HANDLE, baseaddress: *mut *mut core::ffi::c_void, regionsize: *mut usize, freetype: u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwFreeVirtualMemory(processhandle : super::winnt::HANDLE, baseaddress : *mut *mut core::ffi::c_void, regionsize : *mut usize, freetype : u32) -> super::bcrypt::NTSTATUS);
    unsafe { ZwFreeVirtualMemory(processhandle, baseaddress as _, regionsize as _, freetype) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn ZwFsControlFile(filehandle: super::winnt::HANDLE, event: Option<super::winnt::HANDLE>, apcroutine: super::winternl::PIO_APC_ROUTINE, apccontext: Option<*const core::ffi::c_void>, iostatusblock: *mut super::winternl::IO_STATUS_BLOCK, fscontrolcode: u32, inputbuffer: Option<*const core::ffi::c_void>, inputbufferlength: u32, outputbuffer: Option<*mut core::ffi::c_void>, outputbufferlength: u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwFsControlFile(filehandle : super::winnt::HANDLE, event : super::winnt::HANDLE, apcroutine : super::winternl::PIO_APC_ROUTINE, apccontext : *const core::ffi::c_void, iostatusblock : *mut super::winternl::IO_STATUS_BLOCK, fscontrolcode : u32, inputbuffer : *const core::ffi::c_void, inputbufferlength : u32, outputbuffer : *mut core::ffi::c_void, outputbufferlength : u32) -> super::bcrypt::NTSTATUS);
    unsafe { ZwFsControlFile(filehandle, event.unwrap_or(core::mem::zeroed()) as _, apcroutine, apccontext.unwrap_or(core::mem::zeroed()) as _, iostatusblock as _, fscontrolcode, inputbuffer.unwrap_or(core::mem::zeroed()) as _, inputbufferlength, outputbuffer.unwrap_or(core::mem::zeroed()) as _, outputbufferlength) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn ZwLockFile(filehandle: super::winnt::HANDLE, event: Option<super::winnt::HANDLE>, apcroutine: super::winternl::PIO_APC_ROUTINE, apccontext: Option<*const core::ffi::c_void>, iostatusblock: *mut super::winternl::IO_STATUS_BLOCK, byteoffset: *const i64, length: *const i64, key: u32, failimmediately: bool, exclusivelock: bool) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwLockFile(filehandle : super::winnt::HANDLE, event : super::winnt::HANDLE, apcroutine : super::winternl::PIO_APC_ROUTINE, apccontext : *const core::ffi::c_void, iostatusblock : *mut super::winternl::IO_STATUS_BLOCK, byteoffset : *const i64, length : *const i64, key : u32, failimmediately : bool, exclusivelock : bool) -> super::bcrypt::NTSTATUS);
    unsafe { ZwLockFile(filehandle, event.unwrap_or(core::mem::zeroed()) as _, apcroutine, apccontext.unwrap_or(core::mem::zeroed()) as _, iostatusblock as _, byteoffset, length, key, failimmediately, exclusivelock) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn ZwNotifyChangeKey(keyhandle: super::winnt::HANDLE, event: Option<super::winnt::HANDLE>, apcroutine: super::winternl::PIO_APC_ROUTINE, apccontext: Option<*const core::ffi::c_void>, iostatusblock: *mut super::winternl::IO_STATUS_BLOCK, completionfilter: u32, watchtree: bool, buffer: Option<*mut core::ffi::c_void>, buffersize: u32, asynchronous: bool) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwNotifyChangeKey(keyhandle : super::winnt::HANDLE, event : super::winnt::HANDLE, apcroutine : super::winternl::PIO_APC_ROUTINE, apccontext : *const core::ffi::c_void, iostatusblock : *mut super::winternl::IO_STATUS_BLOCK, completionfilter : u32, watchtree : bool, buffer : *mut core::ffi::c_void, buffersize : u32, asynchronous : bool) -> super::bcrypt::NTSTATUS);
    unsafe { ZwNotifyChangeKey(keyhandle, event.unwrap_or(core::mem::zeroed()) as _, apcroutine, apccontext.unwrap_or(core::mem::zeroed()) as _, iostatusblock as _, completionfilter, watchtree, buffer.unwrap_or(core::mem::zeroed()) as _, buffersize, asynchronous) }
}
#[cfg(all(feature = "bcrypt", feature = "d3dkmthk", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn ZwOpenDirectoryObject(directoryhandle: *mut super::winnt::HANDLE, desiredaccess: super::winnt::ACCESS_MASK, objectattributes: *const super::d3dkmthk::OBJECT_ATTRIBUTES) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwOpenDirectoryObject(directoryhandle : *mut super::winnt::HANDLE, desiredaccess : super::winnt::ACCESS_MASK, objectattributes : *const super::d3dkmthk::OBJECT_ATTRIBUTES) -> super::bcrypt::NTSTATUS);
    unsafe { ZwOpenDirectoryObject(directoryhandle as _, desiredaccess, objectattributes) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn ZwOpenProcessTokenEx(processhandle: super::winnt::HANDLE, desiredaccess: super::winnt::ACCESS_MASK, handleattributes: u32, tokenhandle: *mut super::winnt::HANDLE) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwOpenProcessTokenEx(processhandle : super::winnt::HANDLE, desiredaccess : super::winnt::ACCESS_MASK, handleattributes : u32, tokenhandle : *mut super::winnt::HANDLE) -> super::bcrypt::NTSTATUS);
    unsafe { ZwOpenProcessTokenEx(processhandle, desiredaccess, handleattributes, tokenhandle as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn ZwOpenThreadTokenEx(threadhandle: super::winnt::HANDLE, desiredaccess: super::winnt::ACCESS_MASK, openasself: bool, handleattributes: u32, tokenhandle: *mut super::winnt::HANDLE) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwOpenThreadTokenEx(threadhandle : super::winnt::HANDLE, desiredaccess : super::winnt::ACCESS_MASK, openasself : bool, handleattributes : u32, tokenhandle : *mut super::winnt::HANDLE) -> super::bcrypt::NTSTATUS);
    unsafe { ZwOpenThreadTokenEx(threadhandle, desiredaccess, openasself, handleattributes, tokenhandle as _) }
}
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn ZwQueryDirectoryFile(filehandle: super::winnt::HANDLE, event: Option<super::winnt::HANDLE>, apcroutine: super::winternl::PIO_APC_ROUTINE, apccontext: Option<*const core::ffi::c_void>, iostatusblock: *mut super::winternl::IO_STATUS_BLOCK, fileinformation: *mut core::ffi::c_void, length: u32, fileinformationclass: super::winternl::FILE_INFORMATION_CLASS, returnsingleentry: bool, filename: Option<*const super::ntsecapi::UNICODE_STRING>, restartscan: bool) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwQueryDirectoryFile(filehandle : super::winnt::HANDLE, event : super::winnt::HANDLE, apcroutine : super::winternl::PIO_APC_ROUTINE, apccontext : *const core::ffi::c_void, iostatusblock : *mut super::winternl::IO_STATUS_BLOCK, fileinformation : *mut core::ffi::c_void, length : u32, fileinformationclass : super::winternl::FILE_INFORMATION_CLASS, returnsingleentry : bool, filename : *const super::ntsecapi::UNICODE_STRING, restartscan : bool) -> super::bcrypt::NTSTATUS);
    unsafe { ZwQueryDirectoryFile(filehandle, event.unwrap_or(core::mem::zeroed()) as _, apcroutine, apccontext.unwrap_or(core::mem::zeroed()) as _, iostatusblock as _, fileinformation as _, length, fileinformationclass, returnsingleentry, filename.unwrap_or(core::mem::zeroed()) as _, restartscan) }
}
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn ZwQueryDirectoryFileEx(filehandle: super::winnt::HANDLE, event: Option<super::winnt::HANDLE>, apcroutine: super::winternl::PIO_APC_ROUTINE, apccontext: Option<*const core::ffi::c_void>, iostatusblock: *mut super::winternl::IO_STATUS_BLOCK, fileinformation: *mut core::ffi::c_void, length: u32, fileinformationclass: super::winternl::FILE_INFORMATION_CLASS, queryflags: u32, filename: Option<*const super::ntsecapi::UNICODE_STRING>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwQueryDirectoryFileEx(filehandle : super::winnt::HANDLE, event : super::winnt::HANDLE, apcroutine : super::winternl::PIO_APC_ROUTINE, apccontext : *const core::ffi::c_void, iostatusblock : *mut super::winternl::IO_STATUS_BLOCK, fileinformation : *mut core::ffi::c_void, length : u32, fileinformationclass : super::winternl::FILE_INFORMATION_CLASS, queryflags : u32, filename : *const super::ntsecapi::UNICODE_STRING) -> super::bcrypt::NTSTATUS);
    unsafe { ZwQueryDirectoryFileEx(filehandle, event.unwrap_or(core::mem::zeroed()) as _, apcroutine, apccontext.unwrap_or(core::mem::zeroed()) as _, iostatusblock as _, fileinformation as _, length, fileinformationclass, queryflags, filename.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn ZwQueryEaFile(filehandle: super::winnt::HANDLE, iostatusblock: *mut super::winternl::IO_STATUS_BLOCK, buffer: *mut core::ffi::c_void, length: u32, returnsingleentry: bool, ealist: Option<*const core::ffi::c_void>, ealistlength: u32, eaindex: Option<*const u32>, restartscan: bool) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "C" fn ZwQueryEaFile(filehandle : super::winnt::HANDLE, iostatusblock : *mut super::winternl::IO_STATUS_BLOCK, buffer : *mut core::ffi::c_void, length : u32, returnsingleentry : bool, ealist : *const core::ffi::c_void, ealistlength : u32, eaindex : *const u32, restartscan : bool) -> super::bcrypt::NTSTATUS);
    unsafe { ZwQueryEaFile(filehandle, iostatusblock as _, buffer as _, length, returnsingleentry, ealist.unwrap_or(core::mem::zeroed()) as _, ealistlength, eaindex.unwrap_or(core::mem::zeroed()) as _, restartscan) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn ZwQueryInformationToken(tokenhandle: super::winnt::HANDLE, tokeninformationclass: super::winnt::TOKEN_INFORMATION_CLASS, tokeninformation: Option<*mut core::ffi::c_void>, tokeninformationlength: u32, returnlength: *mut u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwQueryInformationToken(tokenhandle : super::winnt::HANDLE, tokeninformationclass : super::winnt::TOKEN_INFORMATION_CLASS, tokeninformation : *mut core::ffi::c_void, tokeninformationlength : u32, returnlength : *mut u32) -> super::bcrypt::NTSTATUS);
    unsafe { ZwQueryInformationToken(tokenhandle, tokeninformationclass, tokeninformation.unwrap_or(core::mem::zeroed()) as _, tokeninformationlength, returnlength as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn ZwQueryObject(handle: Option<super::winnt::HANDLE>, objectinformationclass: super::winternl::OBJECT_INFORMATION_CLASS, objectinformation: Option<*mut core::ffi::c_void>, objectinformationlength: u32, returnlength: Option<*mut u32>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwQueryObject(handle : super::winnt::HANDLE, objectinformationclass : super::winternl::OBJECT_INFORMATION_CLASS, objectinformation : *mut core::ffi::c_void, objectinformationlength : u32, returnlength : *mut u32) -> super::bcrypt::NTSTATUS);
    unsafe { ZwQueryObject(handle.unwrap_or(core::mem::zeroed()) as _, objectinformationclass, objectinformation.unwrap_or(core::mem::zeroed()) as _, objectinformationlength, returnlength.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn ZwQueryQuotaInformationFile(filehandle: super::winnt::HANDLE, iostatusblock: *mut super::winternl::IO_STATUS_BLOCK, buffer: *mut core::ffi::c_void, length: u32, returnsingleentry: bool, sidlist: Option<*const core::ffi::c_void>, sidlistlength: u32, startsid: Option<super::winnt::PSID>, restartscan: bool) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwQueryQuotaInformationFile(filehandle : super::winnt::HANDLE, iostatusblock : *mut super::winternl::IO_STATUS_BLOCK, buffer : *mut core::ffi::c_void, length : u32, returnsingleentry : bool, sidlist : *const core::ffi::c_void, sidlistlength : u32, startsid : super::winnt::PSID, restartscan : bool) -> super::bcrypt::NTSTATUS);
    unsafe { ZwQueryQuotaInformationFile(filehandle, iostatusblock as _, buffer as _, length, returnsingleentry, sidlist.unwrap_or(core::mem::zeroed()) as _, sidlistlength, startsid.unwrap_or(core::mem::zeroed()) as _, restartscan) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn ZwQuerySecurityObject(handle: super::winnt::HANDLE, securityinformation: super::winnt::SECURITY_INFORMATION, securitydescriptor: super::winnt::PSECURITY_DESCRIPTOR, length: u32, lengthneeded: *mut u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwQuerySecurityObject(handle : super::winnt::HANDLE, securityinformation : super::winnt::SECURITY_INFORMATION, securitydescriptor : super::winnt::PSECURITY_DESCRIPTOR, length : u32, lengthneeded : *mut u32) -> super::bcrypt::NTSTATUS);
    unsafe { ZwQuerySecurityObject(handle, securityinformation, securitydescriptor as _, length, lengthneeded as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn ZwQueryVirtualMemory(processhandle: super::winnt::HANDLE, baseaddress: Option<*const core::ffi::c_void>, memoryinformationclass: MEMORY_INFORMATION_CLASS, memoryinformation: *mut core::ffi::c_void, memoryinformationlength: usize, returnlength: Option<*mut usize>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwQueryVirtualMemory(processhandle : super::winnt::HANDLE, baseaddress : *const core::ffi::c_void, memoryinformationclass : MEMORY_INFORMATION_CLASS, memoryinformation : *mut core::ffi::c_void, memoryinformationlength : usize, returnlength : *mut usize) -> super::bcrypt::NTSTATUS);
    unsafe { ZwQueryVirtualMemory(processhandle, baseaddress.unwrap_or(core::mem::zeroed()) as _, memoryinformationclass, memoryinformation as _, memoryinformationlength, returnlength.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn ZwSetEaFile(filehandle: super::winnt::HANDLE, iostatusblock: *mut super::winternl::IO_STATUS_BLOCK, buffer: *const core::ffi::c_void, length: u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "C" fn ZwSetEaFile(filehandle : super::winnt::HANDLE, iostatusblock : *mut super::winternl::IO_STATUS_BLOCK, buffer : *const core::ffi::c_void, length : u32) -> super::bcrypt::NTSTATUS);
    unsafe { ZwSetEaFile(filehandle, iostatusblock as _, buffer, length) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn ZwSetEvent(eventhandle: super::winnt::HANDLE, previousstate: Option<*mut i32>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwSetEvent(eventhandle : super::winnt::HANDLE, previousstate : *mut i32) -> super::bcrypt::NTSTATUS);
    unsafe { ZwSetEvent(eventhandle, previousstate.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn ZwSetInformationToken(tokenhandle: super::winnt::HANDLE, tokeninformationclass: super::winnt::TOKEN_INFORMATION_CLASS, tokeninformation: *const core::ffi::c_void, tokeninformationlength: u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwSetInformationToken(tokenhandle : super::winnt::HANDLE, tokeninformationclass : super::winnt::TOKEN_INFORMATION_CLASS, tokeninformation : *const core::ffi::c_void, tokeninformationlength : u32) -> super::bcrypt::NTSTATUS);
    unsafe { ZwSetInformationToken(tokenhandle, tokeninformationclass, tokeninformation, tokeninformationlength) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn ZwSetInformationVirtualMemory(processhandle: super::winnt::HANDLE, vminformationclass: VIRTUAL_MEMORY_INFORMATION_CLASS, virtualaddresses: &[MEMORY_RANGE_ENTRY], vminformation: *const core::ffi::c_void, vminformationlength: u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwSetInformationVirtualMemory(processhandle : super::winnt::HANDLE, vminformationclass : VIRTUAL_MEMORY_INFORMATION_CLASS, numberofentries : usize, virtualaddresses : *const MEMORY_RANGE_ENTRY, vminformation : *const core::ffi::c_void, vminformationlength : u32) -> super::bcrypt::NTSTATUS);
    unsafe { ZwSetInformationVirtualMemory(processhandle, vminformationclass, virtualaddresses.len().try_into().unwrap(), core::mem::transmute(virtualaddresses.as_ptr()), vminformation, vminformationlength) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn ZwSetQuotaInformationFile(filehandle: super::winnt::HANDLE, iostatusblock: *mut super::winternl::IO_STATUS_BLOCK, buffer: *const core::ffi::c_void, length: u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwSetQuotaInformationFile(filehandle : super::winnt::HANDLE, iostatusblock : *mut super::winternl::IO_STATUS_BLOCK, buffer : *const core::ffi::c_void, length : u32) -> super::bcrypt::NTSTATUS);
    unsafe { ZwSetQuotaInformationFile(filehandle, iostatusblock as _, buffer, length) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn ZwSetSecurityObject(handle: super::winnt::HANDLE, securityinformation: super::winnt::SECURITY_INFORMATION, securitydescriptor: super::winnt::PSECURITY_DESCRIPTOR) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwSetSecurityObject(handle : super::winnt::HANDLE, securityinformation : super::winnt::SECURITY_INFORMATION, securitydescriptor : super::winnt::PSECURITY_DESCRIPTOR) -> super::bcrypt::NTSTATUS);
    unsafe { ZwSetSecurityObject(handle, securityinformation, securitydescriptor) }
}
#[cfg(all(feature = "bcrypt", feature = "wdm", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn ZwSetVolumeInformationFile(filehandle: super::winnt::HANDLE, iostatusblock: *mut super::winternl::IO_STATUS_BLOCK, fsinformation: *const core::ffi::c_void, length: u32, fsinformationclass: super::wdm::FS_INFORMATION_CLASS) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwSetVolumeInformationFile(filehandle : super::winnt::HANDLE, iostatusblock : *mut super::winternl::IO_STATUS_BLOCK, fsinformation : *const core::ffi::c_void, length : u32, fsinformationclass : super::wdm::FS_INFORMATION_CLASS) -> super::bcrypt::NTSTATUS);
    unsafe { ZwSetVolumeInformationFile(filehandle, iostatusblock as _, fsinformation, length, fsinformationclass) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn ZwUnlockFile(filehandle: super::winnt::HANDLE, iostatusblock: *mut super::winternl::IO_STATUS_BLOCK, byteoffset: *const i64, length: *const i64, key: u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwUnlockFile(filehandle : super::winnt::HANDLE, iostatusblock : *mut super::winternl::IO_STATUS_BLOCK, byteoffset : *const i64, length : *const i64, key : u32) -> super::bcrypt::NTSTATUS);
    unsafe { ZwUnlockFile(filehandle, iostatusblock as _, byteoffset, length, key) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn ZwWaitForSingleObject(handle: super::winnt::HANDLE, alertable: bool, timeout: Option<*const i64>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwWaitForSingleObject(handle : super::winnt::HANDLE, alertable : bool, timeout : *const i64) -> super::bcrypt::NTSTATUS);
    unsafe { ZwWaitForSingleObject(handle, alertable, timeout.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
pub type ALLOCATE_VIRTUAL_MEMORY_EX_CALLBACK = Option<unsafe extern "system" fn(callbackcontext: super::winnt::HANDLE, processhandle: super::winnt::HANDLE, baseaddress: *mut *mut core::ffi::c_void, regionsize: *mut usize, allocationtype: u32, pageprotection: u32, extendedparameters: *mut super::winnt::MEM_EXTENDED_PARAMETER, extendedparametercount: u32) -> super::bcrypt::NTSTATUS>;
pub const ANSI_DOS_DOT: u32 = 34;
pub const ANSI_DOS_QM: u32 = 62;
pub const ANSI_DOS_STAR: u32 = 60;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
    pub Usn: super::winnt::USN,
    pub SuppressFileAttributeInheritanceMask: u32,
    pub InOpFlags: u32,
    pub OutOpFlags: u32,
    pub InGenFlags: u32,
    pub OutGenFlags: u32,
    pub CaseSensitiveFlagsMask: u32,
    pub InCaseSensitiveFlags: u32,
    pub OutCaseSensitiveFlags: u32,
}
pub const ATOMIC_CREATE_ECP_IN_FLAG_BEST_EFFORT: u32 = 256;
pub const ATOMIC_CREATE_ECP_IN_FLAG_EOF_SPECIFIED: u32 = 4;
pub const ATOMIC_CREATE_ECP_IN_FLAG_FILE_ATTRIBUTES_SPECIFIED: u32 = 32;
pub const ATOMIC_CREATE_ECP_IN_FLAG_GEN_FLAGS_SPECIFIED: u32 = 32768;
pub const ATOMIC_CREATE_ECP_IN_FLAG_MARK_USN_SOURCE_INFO: u32 = 2048;
pub const ATOMIC_CREATE_ECP_IN_FLAG_OPERATION_MASK: u32 = 255;
pub const ATOMIC_CREATE_ECP_IN_FLAG_OP_FLAGS_SPECIFIED: u32 = 128;
pub const ATOMIC_CREATE_ECP_IN_FLAG_REPARSE_POINT_SPECIFIED: u32 = 2;
pub const ATOMIC_CREATE_ECP_IN_FLAG_SPARSE_SPECIFIED: u32 = 1;
pub const ATOMIC_CREATE_ECP_IN_FLAG_SUPPRESS_DIR_CHANGE_NOTIFY: u32 = 1024;
pub const ATOMIC_CREATE_ECP_IN_FLAG_SUPPRESS_FILE_ATTRIBUTE_INHERITANCE: u32 = 64;
pub const ATOMIC_CREATE_ECP_IN_FLAG_SUPPRESS_PARENT_TIMESTAMPS_UPDATE: u32 = 512;
pub const ATOMIC_CREATE_ECP_IN_FLAG_TIMESTAMPS_SPECIFIED: u32 = 16;
pub const ATOMIC_CREATE_ECP_IN_FLAG_VDL_SPECIFIED: u32 = 8;
pub const ATOMIC_CREATE_ECP_IN_FLAG_WRITE_USN_CLOSE_RECORD: u32 = 4096;
pub const ATOMIC_CREATE_ECP_IN_OP_FLAG_CASE_SENSITIVE_FLAGS_SPECIFIED: u32 = 1;
pub const ATOMIC_CREATE_ECP_OUT_FLAG_EOF_SET: u32 = 4;
pub const ATOMIC_CREATE_ECP_OUT_FLAG_FILE_ATTRIBUTES_RETURNED: u32 = 512;
pub const ATOMIC_CREATE_ECP_OUT_FLAG_FILE_ATTRIBUTES_SET: u32 = 32;
pub const ATOMIC_CREATE_ECP_OUT_FLAG_FILE_ATTRIBUTE_INHERITANCE_SUPPRESSED: u32 = 64;
pub const ATOMIC_CREATE_ECP_OUT_FLAG_OPERATION_MASK: u32 = 255;
pub const ATOMIC_CREATE_ECP_OUT_FLAG_OP_FLAGS_HONORED: u32 = 128;
pub const ATOMIC_CREATE_ECP_OUT_FLAG_REPARSE_POINT_SET: u32 = 2;
pub const ATOMIC_CREATE_ECP_OUT_FLAG_SPARSE_SET: u32 = 1;
pub const ATOMIC_CREATE_ECP_OUT_FLAG_TIMESTAMPS_RETURNED: u32 = 256;
pub const ATOMIC_CREATE_ECP_OUT_FLAG_TIMESTAMPS_SET: u32 = 16;
pub const ATOMIC_CREATE_ECP_OUT_FLAG_USN_CLOSE_RECORD_WRITTEN: u32 = 2048;
pub const ATOMIC_CREATE_ECP_OUT_FLAG_USN_RETURNED: u32 = 4096;
pub const ATOMIC_CREATE_ECP_OUT_FLAG_USN_SOURCE_INFO_MARKED: u32 = 1024;
pub const ATOMIC_CREATE_ECP_OUT_FLAG_VDL_SET: u32 = 8;
pub const ATOMIC_CREATE_ECP_OUT_OP_FLAG_CASE_SENSITIVE_FLAGS_SET: u32 = 1;
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BASE_MCB {
    pub MaximumPairCount: u32,
    pub PairCount: u32,
    pub PoolType: u16,
    pub Flags: u16,
    pub Mapping: *mut core::ffi::c_void,
}
impl Default for BASE_MCB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct CACHE_MANAGER_CALLBACKS {
    pub AcquireForLazyWrite: PACQUIRE_FOR_LAZY_WRITE,
    pub ReleaseFromLazyWrite: PRELEASE_FROM_LAZY_WRITE,
    pub AcquireForReadAhead: PACQUIRE_FOR_READ_AHEAD,
    pub ReleaseFromReadAhead: PRELEASE_FROM_READ_AHEAD,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct CACHE_MANAGER_CALLBACKS_EX {
    pub Version: u16,
    pub Size: u16,
    pub Functions: CACHE_MANAGER_CALLBACK_FUNCTIONS,
}
pub const CACHE_MANAGER_CALLBACKS_EX_V1: u32 = 1;
#[repr(C)]
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
    pub Event: super::wdm::KEVENT,
}
#[cfg(all(feature = "wdm", feature = "winnt"))]
impl Default for CACHE_UNINITIALIZE_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CACHE_USE_DIRECT_ACCESS_MAPPING: u32 = 1;
pub const CACHE_VALID_FLAGS: u32 = 1;
pub const CC_ACQUIRE_DONT_WAIT: u32 = 1;
pub const CC_ACQUIRE_SUPPORTS_ASYNC_LAZYWRITE: u32 = 1;
pub const CC_AGGRESSIVE_UNMAP_BEHIND: u32 = 1;
#[repr(C)]
#[cfg(all(feature = "usb", feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy, Debug)]
pub struct CC_ASYNC_READ_CONTEXT {
    pub CompletionRoutine: PASYNC_READ_COMPLETION_CALLBACK,
    pub Context: *mut core::ffi::c_void,
    pub Mdl: super::usb::PMDL,
    pub RequestorMode: super::wdm::KPROCESSOR_MODE,
    pub NestingLevel: u32,
}
#[cfg(all(feature = "usb", feature = "wdm", feature = "winnt"))]
impl Default for CC_ASYNC_READ_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
pub const CC_DIRTY_PAGES_INFO_V1: u32 = 1;
pub const CC_DISABLE_DIRTY_PAGE_TRACKING: u32 = 8;
pub const CC_DISABLE_READ_AHEAD: u32 = 2;
pub const CC_DISABLE_UNMAP_BEHIND: u32 = 32;
pub const CC_DISABLE_WRITE_BEHIND: u32 = 4;
pub const CC_ENABLE_CPU_CACHE: u32 = 268435456;
pub const CC_ENABLE_DISK_IO_ACCOUNTING: u32 = 16;
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "ntdef"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CC_ERROR_CALLBACK_CONTEXT {
    pub NodeByteSize: super::ntdef::CSHORT,
    pub ErrorCode: super::bcrypt::NTSTATUS,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CC_FILE_SIZES {
    pub AllocationSize: i64,
    pub FileSize: i64,
    pub ValidDataLength: i64,
}
pub const CC_FLUSH_AND_PURGE_GATHER_DIRTY_BITS: u32 = 2;
pub const CC_FLUSH_AND_PURGE_NO_PURGE: u32 = 1;
pub const CC_FLUSH_AND_PURGE_WRITEABLE_VIEWS_NOTSEEN: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub const COMPRESSION_ENGINE_MASK: u32 = 65280;
pub const COMPRESSION_ENGINE_MAX: u32 = 512;
pub const COMPRESSION_FORMAT_ENGINE_MASK: u32 = 65535;
pub const COMPRESSION_FORMAT_MASK: u32 = 255;
pub const COMPRESSION_FORMAT_MAX: u32 = 8;
pub const COPY_FILE_CHUNK_DUPLICATE_EXTENTS: u32 = 1;
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntsecapi", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct COPY_INFORMATION {
    pub SourceFileObject: super::wdm::PFILE_OBJECT,
    pub SourceFileOffset: i64,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CREATE_REDIRECTION_ECP_CONTEXT {
    pub Size: u16,
    pub Flags: u16,
    pub FileId: super::winnt::FILE_ID_128,
    pub VolumeGuid: windows_core::GUID,
}
pub const CREATE_REDIRECTION_FLAGS_SERVICED_FROM_LAYER: u32 = 1;
pub const CREATE_REDIRECTION_FLAGS_SERVICED_FROM_REGISTERED_LAYER: u32 = 4;
pub const CREATE_REDIRECTION_FLAGS_SERVICED_FROM_REMOTE_LAYER: u32 = 8;
pub const CREATE_REDIRECTION_FLAGS_SERVICED_FROM_SCRATCH: u32 = 2;
pub const CREATE_REDIRECTION_FLAGS_SERVICED_FROM_USER_MODE: u32 = 16;
pub type CSV_DOWN_LEVEL_FILE_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CSV_DOWN_LEVEL_OPEN_ECP_CONTEXT {
    pub Version: u32,
    pub IsResume: bool,
    pub FileType: CSV_DOWN_LEVEL_FILE_TYPE,
    pub SourceNodeId: u32,
    pub DestinationNodeId: u32,
}
pub const CSV_DOWN_LEVEL_OPEN_ECP_CONTEXT_V1: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CSV_QUERY_FILE_REVISION_ECP_CONTEXT_FILE_ID_128 {
    pub FileId: super::winnt::FILE_ID_128,
    pub FileRevision: [i64; 3],
}
#[cfg(feature = "winnt")]
impl Default for CSV_QUERY_FILE_REVISION_ECP_CONTEXT_FILE_ID_128 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CSV_SET_HANDLE_PROPERTIES_ECP_CONTEXT {
    pub Size: usize,
    pub PauseTimeoutInSeconds: u32,
    pub Flags: u32,
}
pub const CSV_SET_HANDLE_PROPERTIES_ECP_CONTEXT_FLAGS_VALID_ONLY_IF_CSV_COORDINATOR: u32 = 1;
pub const ChangeDataControlArea: FSRTL_CHANGE_BACKING_TYPE = 0;
pub const ChangeImageControlArea: FSRTL_CHANGE_BACKING_TYPE = 1;
pub const ChangeSharedCacheMap: FSRTL_CHANGE_BACKING_TYPE = 2;
pub const CsvCsvFsInternalFileObject: CSV_DOWN_LEVEL_FILE_TYPE = 1;
pub const CsvDownLevelFileObject: CSV_DOWN_LEVEL_FILE_TYPE = 0;
pub const DD_MUP_DEVICE_NAME: windows_core::PCWSTR = windows_core::w!("\\Device\\Mup");
pub const DEVICE_RESET_KEEP_STACK: u32 = 4;
pub const DEVICE_RESET_RESERVED_0: u32 = 1;
pub const DEVICE_RESET_RESERVED_1: u32 = 2;
pub const DOS_DOT: u32 = 34;
pub const DOS_QM: u32 = 62;
pub const DOS_STAR: u32 = 60;
pub const DO_NOT_PURGE_DIRTY_PAGES: u32 = 4;
pub const DO_NOT_RETRY_PURGE: u32 = 2;
pub const DO_SUPPORTS_PERSISTENT_ACLS: u32 = 131072;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntsecapi", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
pub type DRIVER_FS_NOTIFICATION = Option<unsafe extern "system" fn(deviceobject: *const super::wdm::DEVICE_OBJECT, fsactive: bool)>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DUAL_OPLOCK_KEY_ECP_CONTEXT {
    pub ParentOplockKey: windows_core::GUID,
    pub TargetOplockKey: windows_core::GUID,
    pub ParentOplockKeySet: bool,
    pub TargetOplockKeySet: bool,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DUPLICATE_CLUSTER_DATA {
    pub SourceLcn: i64,
    pub TargetLcn: i64,
    pub TargetFileOffset: i64,
    pub DuplicationLimit: u32,
    pub Reserved: u32,
}
pub const DfsLinkTrackingInformation: LINK_TRACKING_INFORMATION_TYPE = 1;
pub const EA_NAME_NETWORK_OPEN_ECP_INTEGRITY: windows_core::PCSTR = windows_core::s!("ECP{c584edbf-00df-4d28-00b8-8435baca8911e8}-INTEGRITY");
pub const EA_NAME_NETWORK_OPEN_ECP_INTEGRITY_U: windows_core::PCWSTR = windows_core::w!("ECP{c584edbf-00df-4d28-00b8-8435baca8911e8}-INTEGRITY");
pub const EA_NAME_NETWORK_OPEN_ECP_PRIVACY: windows_core::PCSTR = windows_core::s!("ECP{c584edbf-00df-4d28-00b8-8435baca8911e8}-PRIVACY");
pub const EA_NAME_NETWORK_OPEN_ECP_PRIVACY_U: windows_core::PCWSTR = windows_core::w!("ECP{c584edbf-00df-4d28-00b8-8435baca8911e8}-PRIVACY");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ECP_HEADER(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ECP_OPEN_PARAMETERS {
    pub Size: u16,
    pub Reserved: u16,
    pub Flags: u32,
}
pub const ECP_OPEN_PARAMETERS_FLAG_FAIL_ON_CASE_SENSITIVE_DIR: u32 = 16;
pub const ECP_OPEN_PARAMETERS_FLAG_IGNORE_DIR_CASE_SENSITIVITY: u32 = 8;
pub const ECP_OPEN_PARAMETERS_FLAG_OPEN_FOR_DELETE: u32 = 4;
pub const ECP_OPEN_PARAMETERS_FLAG_OPEN_FOR_READ: u32 = 1;
pub const ECP_OPEN_PARAMETERS_FLAG_OPEN_FOR_WRITE: u32 = 2;
pub const ECP_TYPE_CLFS_CREATE_CONTAINER: windows_core::GUID = windows_core::GUID::from_u128(0x8650c9fe_0cec_8bf6_bd1e_835956541090);
pub const ECP_TYPE_IO_STOP_ON_SYMLINK_FILTER_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x940e5d56_1646_4d3c_87b6_577ec36a1466);
pub const ECP_TYPE_OPEN_REPARSE_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x323eb6a8_affd_4d95_8230_863bce09d37a);
#[repr(C)]
#[cfg(all(feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct EOF_WAIT_BLOCK {
    pub EofWaitLinks: super::winnt::LIST_ENTRY,
    pub Event: super::wdm::KEVENT,
}
#[cfg(all(feature = "wdm", feature = "winnt"))]
impl Default for EOF_WAIT_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EXTENT_READ_CACHE_INFO_BUFFER {
    pub AllocatedCache: i64,
    pub PopulatedCache: i64,
    pub InErrorCache: i64,
}
pub const EqualTo: FSRTL_COMPARISON_RESULT = 0;
pub type FAST_IO_POSSIBLE = i32;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_ACCESS_INFORMATION {
    pub AccessFlags: super::winnt::ACCESS_MASK,
}
pub const FILE_ACTION_ADDED_STREAM: u32 = 6;
pub const FILE_ACTION_ID_NOT_TUNNELLED: u32 = 10;
pub const FILE_ACTION_MODIFIED_STREAM: u32 = 8;
pub const FILE_ACTION_REMOVED_BY_DELETE: u32 = 9;
pub const FILE_ACTION_REMOVED_STREAM: u32 = 7;
pub const FILE_ACTION_TUNNELLED_ID_COLLISION: u32 = 11;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_ALLOCATION_INFORMATION {
    pub AllocationSize: i64,
}
#[repr(C)]
#[cfg(all(feature = "ntddk", feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_ALL_INFORMATION {
    pub BasicInformation: super::wdm::FILE_BASIC_INFORMATION,
    pub StandardInformation: super::wdm::FILE_STANDARD_INFORMATION,
    pub InternalInformation: FILE_INTERNAL_INFORMATION,
    pub EaInformation: FILE_EA_INFORMATION,
    pub AccessInformation: FILE_ACCESS_INFORMATION,
    pub PositionInformation: super::wdm::FILE_POSITION_INFORMATION,
    pub ModeInformation: FILE_MODE_INFORMATION,
    pub AlignmentInformation: super::ntddk::FILE_ALIGNMENT_INFORMATION,
    pub NameInformation: super::ntddk::FILE_NAME_INFORMATION,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FILE_BOTH_DIR_INFORMATION {
    pub NextEntryOffset: u32,
    pub FileIndex: u32,
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub EndOfFile: i64,
    pub AllocationSize: i64,
    pub FileAttributes: u32,
    pub FileNameLength: u32,
    pub EaSize: u32,
    pub ShortNameLength: super::winnt::CCHAR,
    pub ShortName: [u16; 12],
    pub FileName: [u16; 1],
}
#[cfg(feature = "winnt")]
impl Default for FILE_BOTH_DIR_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FILE_CLEANUP_FILE_DELETED: u32 = 4;
pub const FILE_CLEANUP_FILE_REMAINS: u32 = 2;
pub const FILE_CLEANUP_LINK_DELETED: u32 = 8;
pub const FILE_CLEANUP_POSIX_STYLE_DELETE: u32 = 32;
pub const FILE_CLEANUP_STREAM_DELETED: u32 = 16;
pub const FILE_CLEANUP_UNKNOWN: u32 = 0;
pub const FILE_CLEANUP_WRONG_DEVICE: u32 = 1;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FILE_COMPLETION_INFORMATION {
    pub Port: super::winnt::HANDLE,
    pub Key: *mut core::ffi::c_void,
}
#[cfg(feature = "winnt")]
impl Default for FILE_COMPLETION_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FILE_COMPRESSION_INFORMATION {
    pub CompressedFileSize: i64,
    pub CompressionFormat: u16,
    pub CompressionUnitShift: u8,
    pub ChunkShift: u8,
    pub ClusterShift: u8,
    pub Reserved: [u8; 3],
}
impl Default for FILE_COMPRESSION_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FILE_DIRECTORY_INFORMATION {
    pub NextEntryOffset: u32,
    pub FileIndex: u32,
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub EndOfFile: i64,
    pub AllocationSize: i64,
    pub FileAttributes: u32,
    pub FileNameLength: u32,
    pub FileName: [u16; 1],
}
impl Default for FILE_DIRECTORY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_EA_INFORMATION {
    pub EaSize: u32,
}
pub const FILE_EA_TYPE_ASCII: u32 = 65533;
pub const FILE_EA_TYPE_ASN1: u32 = 65501;
pub const FILE_EA_TYPE_BINARY: u32 = 65534;
pub const FILE_EA_TYPE_BITMAP: u32 = 65531;
pub const FILE_EA_TYPE_EA: u32 = 65518;
pub const FILE_EA_TYPE_FAMILY_IDS: u32 = 65281;
pub const FILE_EA_TYPE_ICON: u32 = 65529;
pub const FILE_EA_TYPE_METAFILE: u32 = 65530;
pub const FILE_EA_TYPE_MVMT: u32 = 65503;
pub const FILE_EA_TYPE_MVST: u32 = 65502;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_END_OF_FILE_INFORMATION_EX {
    pub EndOfFile: i64,
    pub PagingFileSizeInMM: i64,
    pub PagingFileMaxSize: i64,
    pub Flags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_FS_CONTROL_INFORMATION {
    pub FreeSpaceStartFiltering: i64,
    pub FreeSpaceThreshold: i64,
    pub FreeSpaceStopFiltering: i64,
    pub DefaultQuotaThreshold: i64,
    pub DefaultQuotaLimit: i64,
    pub FileSystemControlFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_FS_DATA_COPY_INFORMATION {
    pub NumberOfCopies: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FILE_FS_DRIVER_PATH_INFORMATION {
    pub DriverInPath: bool,
    pub DriverNameLength: u32,
    pub DriverName: [u16; 1],
}
impl Default for FILE_FS_DRIVER_PATH_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_FS_GUID_INFORMATION {
    pub FsGuid: windows_core::GUID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_FS_VOLUME_FLAGS_INFORMATION {
    pub Flags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FILE_FULL_DIR_INFORMATION {
    pub NextEntryOffset: u32,
    pub FileIndex: u32,
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub EndOfFile: i64,
    pub AllocationSize: i64,
    pub FileAttributes: u32,
    pub FileNameLength: u32,
    pub EaSize: u32,
    pub FileName: [u16; 1],
}
impl Default for FILE_FULL_DIR_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_GET_QUOTA_INFORMATION {
    pub NextEntryOffset: u32,
    pub SidLength: u32,
    pub Sid: super::winnt::SID,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FILE_ID_64_EXTD_BOTH_DIR_INFORMATION {
    pub NextEntryOffset: u32,
    pub FileIndex: u32,
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub EndOfFile: i64,
    pub AllocationSize: i64,
    pub FileAttributes: u32,
    pub FileNameLength: u32,
    pub EaSize: u32,
    pub ReparsePointTag: u32,
    pub FileId: i64,
    pub ShortNameLength: super::winnt::CCHAR,
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FILE_ID_64_EXTD_DIR_INFORMATION {
    pub NextEntryOffset: u32,
    pub FileIndex: u32,
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub EndOfFile: i64,
    pub AllocationSize: i64,
    pub FileAttributes: u32,
    pub FileNameLength: u32,
    pub EaSize: u32,
    pub ReparsePointTag: u32,
    pub FileId: i64,
    pub FileName: [u16; 1],
}
impl Default for FILE_ID_64_EXTD_DIR_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FILE_ID_ALL_EXTD_BOTH_DIR_INFORMATION {
    pub NextEntryOffset: u32,
    pub FileIndex: u32,
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub EndOfFile: i64,
    pub AllocationSize: i64,
    pub FileAttributes: u32,
    pub FileNameLength: u32,
    pub EaSize: u32,
    pub ReparsePointTag: u32,
    pub FileId: i64,
    pub FileId128: super::winnt::FILE_ID_128,
    pub ShortNameLength: super::winnt::CCHAR,
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FILE_ID_ALL_EXTD_DIR_INFORMATION {
    pub NextEntryOffset: u32,
    pub FileIndex: u32,
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub EndOfFile: i64,
    pub AllocationSize: i64,
    pub FileAttributes: u32,
    pub FileNameLength: u32,
    pub EaSize: u32,
    pub ReparsePointTag: u32,
    pub FileId: i64,
    pub FileId128: super::winnt::FILE_ID_128,
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FILE_ID_BOTH_DIR_INFORMATION {
    pub NextEntryOffset: u32,
    pub FileIndex: u32,
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub EndOfFile: i64,
    pub AllocationSize: i64,
    pub FileAttributes: u32,
    pub FileNameLength: u32,
    pub EaSize: u32,
    pub ShortNameLength: super::winnt::CCHAR,
    pub ShortName: [u16; 12],
    pub FileId: i64,
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FILE_ID_EXTD_BOTH_DIR_INFORMATION {
    pub NextEntryOffset: u32,
    pub FileIndex: u32,
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub EndOfFile: i64,
    pub AllocationSize: i64,
    pub FileAttributes: u32,
    pub FileNameLength: u32,
    pub EaSize: u32,
    pub ReparsePointTag: u32,
    pub FileId: super::winnt::FILE_ID_128,
    pub ShortNameLength: super::winnt::CCHAR,
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FILE_ID_EXTD_DIR_INFORMATION {
    pub NextEntryOffset: u32,
    pub FileIndex: u32,
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub EndOfFile: i64,
    pub AllocationSize: i64,
    pub FileAttributes: u32,
    pub FileNameLength: u32,
    pub EaSize: u32,
    pub ReparsePointTag: u32,
    pub FileId: super::winnt::FILE_ID_128,
    pub FileName: [u16; 1],
}
#[cfg(feature = "winnt")]
impl Default for FILE_ID_EXTD_DIR_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FILE_ID_FULL_DIR_INFORMATION {
    pub NextEntryOffset: u32,
    pub FileIndex: u32,
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub EndOfFile: i64,
    pub AllocationSize: i64,
    pub FileAttributes: u32,
    pub FileNameLength: u32,
    pub EaSize: u32,
    pub FileId: i64,
    pub FileName: [u16; 1],
}
impl Default for FILE_ID_FULL_DIR_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FILE_ID_GLOBAL_TX_DIR_INFORMATION {
    pub NextEntryOffset: u32,
    pub FileIndex: u32,
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub EndOfFile: i64,
    pub AllocationSize: i64,
    pub FileAttributes: u32,
    pub FileNameLength: u32,
    pub FileId: i64,
    pub LockingTransactionId: windows_core::GUID,
    pub TxInfoFlags: u32,
    pub FileName: [u16; 1],
}
impl Default for FILE_ID_GLOBAL_TX_DIR_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FILE_ID_GLOBAL_TX_DIR_INFO_FLAG_VISIBLE_OUTSIDE_TX: u32 = 4;
pub const FILE_ID_GLOBAL_TX_DIR_INFO_FLAG_VISIBLE_TO_TX: u32 = 2;
pub const FILE_ID_GLOBAL_TX_DIR_INFO_FLAG_WRITELOCKED: u32 = 1;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_ID_INFORMATION {
    pub VolumeSerialNumber: u64,
    pub FileId: super::winnt::FILE_ID_128,
}
#[repr(C)]
#[cfg(feature = "winternl")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_INFORMATION_DEFINITION {
    pub Class: super::winternl::FILE_INFORMATION_CLASS,
    pub NextEntryOffset: u32,
    pub FileNameOffset: u32,
    pub FileNameLengthOffset: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_INTERNAL_INFORMATION {
    pub IndexNumber: i64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_KNOWN_FOLDER_INFORMATION {
    pub Type: FILE_KNOWN_FOLDER_TYPE,
}
pub type FILE_KNOWN_FOLDER_TYPE = i32;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_LINKS_FULL_ID_INFORMATION {
    pub BytesNeeded: u32,
    pub EntriesReturned: u32,
    pub Entry: FILE_LINK_ENTRY_FULL_ID_INFORMATION,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_LINKS_INFORMATION {
    pub BytesNeeded: u32,
    pub EntriesReturned: u32,
    pub Entry: FILE_LINK_ENTRY_INFORMATION,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FILE_LINK_ENTRY_FULL_ID_INFORMATION {
    pub NextEntryOffset: u32,
    pub ParentFileId: super::winnt::FILE_ID_128,
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub const FILE_LINK_FORCE_RESIZE_SOURCE_SR: u32 = 256;
pub const FILE_LINK_FORCE_RESIZE_SR: u32 = 384;
pub const FILE_LINK_FORCE_RESIZE_TARGET_SR: u32 = 128;
pub const FILE_LINK_IGNORE_READONLY_ATTRIBUTE: u32 = 64;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct FILE_LINK_INFORMATION {
    pub Anonymous: FILE_LINK_INFORMATION_0,
    pub RootDirectory: super::winnt::HANDLE,
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
    pub ReplaceIfExists: bool,
    pub Flags: u32,
}
#[cfg(feature = "winnt")]
impl Default for FILE_LINK_INFORMATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FILE_LINK_NO_DECREASE_AVAILABLE_SPACE: u32 = 32;
pub const FILE_LINK_NO_INCREASE_AVAILABLE_SPACE: u32 = 16;
pub const FILE_LINK_POSIX_SEMANTICS: u32 = 2;
pub const FILE_LINK_PRESERVE_AVAILABLE_SPACE: u32 = 48;
pub const FILE_LINK_REPLACE_IF_EXISTS: u32 = 1;
pub const FILE_LINK_SUPPRESS_STORAGE_RESERVE_INHERITANCE: u32 = 8;
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntsecapi", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug)]
pub struct FILE_LOCK {
    pub CompleteLockIrpRoutine: PCOMPLETE_LOCK_IRP_ROUTINE,
    pub UnlockRoutine: PUNLOCK_ROUTINE,
    pub FastIoIsQuestionable: bool,
    pub SpareC: [bool; 3],
    pub LockInformation: *mut core::ffi::c_void,
    pub LastReturnedLockInfo: FILE_LOCK_INFO,
    pub LastReturnedLock: *mut core::ffi::c_void,
    pub LockRequestsInProgress: i32,
}
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntsecapi", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
impl Default for FILE_LOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntsecapi", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FILE_LOCK_INFO {
    pub StartingByte: i64,
    pub Length: i64,
    pub ExclusiveLock: bool,
    pub Key: u32,
    pub FileObject: super::wdm::PFILE_OBJECT,
    pub ProcessId: *mut core::ffi::c_void,
    pub EndingByte: i64,
}
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntsecapi", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
impl Default for FILE_LOCK_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_MAILSLOT_QUERY_INFORMATION {
    pub MaximumMessageSize: u32,
    pub MailslotQuota: u32,
    pub NextMessageSize: u32,
    pub MessagesAvailable: u32,
    pub ReadTimeout: i64,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_MAILSLOT_SET_INFORMATION {
    pub ReadTimeout: super::winnt::PLARGE_INTEGER,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_MODE_INFORMATION {
    pub Mode: u32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FILE_MOVE_CLUSTER_INFORMATION {
    pub ClusterCount: u32,
    pub RootDirectory: super::winnt::HANDLE,
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub const FILE_NEED_EA: u32 = 128;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FILE_NETWORK_PHYSICAL_NAME_INFORMATION {
    pub FileNameLength: u32,
    pub FileName: [u16; 1],
}
impl Default for FILE_NETWORK_PHYSICAL_NAME_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FILE_NOTIFY_CHANGE_EA: u32 = 128;
pub const FILE_NOTIFY_CHANGE_NAME: u32 = 3;
pub const FILE_NOTIFY_CHANGE_STREAM_NAME: u32 = 512;
pub const FILE_NOTIFY_CHANGE_STREAM_SIZE: u32 = 1024;
pub const FILE_NOTIFY_CHANGE_STREAM_WRITE: u32 = 2048;
pub const FILE_NOTIFY_VALID_MASK: u32 = 4095;
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub const FILE_OPBATCH_BREAK_UNDERWAY: u32 = 9;
pub const FILE_OPLOCK_BROKEN_TO_LEVEL_2: u32 = 7;
pub const FILE_OPLOCK_BROKEN_TO_NONE: u32 = 8;
pub const FILE_PIPE_ACCEPT_REMOTE_CLIENTS: u32 = 0;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_PIPE_ASSIGN_EVENT_BUFFER {
    pub EventHandle: super::winnt::HANDLE,
    pub KeyValue: u32,
}
pub const FILE_PIPE_BYTE_STREAM_MODE: u32 = 0;
pub const FILE_PIPE_BYTE_STREAM_TYPE: u32 = 0;
pub const FILE_PIPE_CLIENT_END: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FILE_PIPE_CLIENT_PROCESS_BUFFER {
    pub ClientSession: *mut core::ffi::c_void,
    pub ClientProcess: *mut core::ffi::c_void,
}
impl Default for FILE_PIPE_CLIENT_PROCESS_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FILE_PIPE_CLIENT_PROCESS_BUFFER_V2 {
    pub ClientSession: u64,
    pub ClientProcess: *mut core::ffi::c_void,
}
impl Default for FILE_PIPE_CLIENT_PROCESS_BUFFER_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FILE_PIPE_CLOSING_STATE: u32 = 4;
pub const FILE_PIPE_COMPLETE_OPERATION: u32 = 1;
pub const FILE_PIPE_COMPUTER_NAME_LENGTH: u32 = 15;
pub const FILE_PIPE_CONNECTED_STATE: u32 = 3;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_PIPE_CREATE_SYMLINK_INPUT {
    pub NameOffset: u16,
    pub NameLength: u16,
    pub SubstituteNameOffset: u16,
    pub SubstituteNameLength: u16,
    pub Flags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_PIPE_DELETE_SYMLINK_INPUT {
    pub NameOffset: u16,
    pub NameLength: u16,
}
pub const FILE_PIPE_DISCONNECTED_STATE: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_PIPE_EVENT_BUFFER {
    pub NamedPipeState: u32,
    pub EntryType: u32,
    pub ByteCount: u32,
    pub KeyValue: u32,
    pub NumberRequests: u32,
}
pub const FILE_PIPE_FULL_DUPLEX: u32 = 2;
pub const FILE_PIPE_INBOUND: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_PIPE_INFORMATION {
    pub ReadMode: u32,
    pub CompletionMode: u32,
}
pub const FILE_PIPE_LISTENING_STATE: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
pub const FILE_PIPE_MESSAGE_MODE: u32 = 1;
pub const FILE_PIPE_MESSAGE_TYPE: u32 = 1;
pub const FILE_PIPE_OUTBOUND: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub const FILE_PIPE_QUEUE_OPERATION: u32 = 0;
pub const FILE_PIPE_READ_DATA: u32 = 0;
pub const FILE_PIPE_REJECT_REMOTE_CLIENTS: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_PIPE_REMOTE_INFORMATION {
    pub CollectDataTime: i64,
    pub MaximumCollectionCount: u32,
}
pub const FILE_PIPE_SERVER_END: u32 = 1;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_PIPE_SILO_ARRIVAL_INPUT {
    pub JobHandle: super::winnt::HANDLE,
}
pub const FILE_PIPE_SYMLINK_FLAG_GLOBAL: u32 = 1;
pub const FILE_PIPE_SYMLINK_FLAG_RELATIVE: u32 = 2;
pub const FILE_PIPE_SYMLINK_VALID_FLAGS: u32 = 3;
pub const FILE_PIPE_TYPE_VALID_MASK: u32 = 3;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FILE_PIPE_WAIT_FOR_BUFFER {
    pub Timeout: i64,
    pub NameLength: u32,
    pub TimeoutSpecified: bool,
    pub Name: [u16; 1],
}
impl Default for FILE_PIPE_WAIT_FOR_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FILE_PIPE_WRITE_SPACE: u32 = 1;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_QUOTA_INFORMATION {
    pub NextEntryOffset: u32,
    pub SidLength: u32,
    pub ChangeTime: i64,
    pub QuotaUsed: i64,
    pub QuotaThreshold: i64,
    pub QuotaLimit: i64,
    pub Sid: super::winnt::SID,
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_REMOTE_PROTOCOL_INFORMATION_1_0 {
    pub Server: FILE_REMOTE_PROTOCOL_INFORMATION_1_0_0,
    pub Share: FILE_REMOTE_PROTOCOL_INFORMATION_1_0_1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_REMOTE_PROTOCOL_INFORMATION_1_0_0 {
    pub Capabilities: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub const FILE_RENAME_FORCE_RESIZE_SOURCE_SR: u32 = 256;
pub const FILE_RENAME_FORCE_RESIZE_SR: u32 = 384;
pub const FILE_RENAME_FORCE_RESIZE_TARGET_SR: u32 = 128;
pub const FILE_RENAME_IGNORE_READONLY_ATTRIBUTE: u32 = 64;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct FILE_RENAME_INFORMATION {
    pub Anonymous: FILE_RENAME_INFORMATION_0,
    pub RootDirectory: super::winnt::HANDLE,
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
    pub ReplaceIfExists: bool,
    pub Flags: u32,
}
#[cfg(feature = "winnt")]
impl Default for FILE_RENAME_INFORMATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FILE_RENAME_NO_DECREASE_AVAILABLE_SPACE: u32 = 32;
pub const FILE_RENAME_NO_INCREASE_AVAILABLE_SPACE: u32 = 16;
pub const FILE_RENAME_POSIX_SEMANTICS: u32 = 2;
pub const FILE_RENAME_PRESERVE_AVAILABLE_SPACE: u32 = 48;
pub const FILE_RENAME_REPLACE_IF_EXISTS: u32 = 1;
pub const FILE_RENAME_SUPPRESS_PIN_STATE_INHERITANCE: u32 = 4;
pub const FILE_RENAME_SUPPRESS_STORAGE_RESERVE_INHERITANCE: u32 = 8;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_REPARSE_POINT_INFORMATION {
    pub FileReference: i64,
    pub Tag: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_STANDARD_LINK_INFORMATION {
    pub NumberOfAccessibleLinks: u32,
    pub TotalNumberOfLinks: u32,
    pub DeletePending: bool,
    pub Directory: bool,
}
#[repr(C)]
#[cfg(feature = "winioctl")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_STORAGE_RESERVE_ID_INFORMATION {
    pub StorageReserveId: super::winioctl::STORAGE_RESERVE_ID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FILE_STREAM_INFORMATION {
    pub NextEntryOffset: u32,
    pub StreamNameLength: u32,
    pub StreamSize: i64,
    pub StreamAllocationSize: i64,
    pub StreamName: [u16; 1],
}
impl Default for FILE_STREAM_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_STREAM_RESERVATION_INFORMATION {
    pub TrackedReservation: i64,
    pub EnforcedReservation: i64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_TIMESTAMPS {
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FILE_TRACKING_INFORMATION {
    pub DestinationFile: super::winnt::HANDLE,
    pub ObjectInformationLength: u32,
    pub ObjectInformation: [i8; 1],
}
#[cfg(feature = "winnt")]
impl Default for FILE_TRACKING_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FILE_VC_CONTENT_INDEX_DISABLED: u32 = 8;
pub const FILE_VC_LOG_QUOTA_LIMIT: u32 = 32;
pub const FILE_VC_LOG_QUOTA_THRESHOLD: u32 = 16;
pub const FILE_VC_LOG_VOLUME_LIMIT: u32 = 128;
pub const FILE_VC_LOG_VOLUME_THRESHOLD: u32 = 64;
pub const FILE_VC_QUOTAS_INCOMPLETE: u32 = 256;
pub const FILE_VC_QUOTAS_REBUILDING: u32 = 512;
pub const FILE_VC_QUOTA_ENFORCE: u32 = 2;
pub const FILE_VC_QUOTA_MASK: u32 = 3;
pub const FILE_VC_QUOTA_NONE: u32 = 0;
pub const FILE_VC_QUOTA_TRACK: u32 = 1;
pub const FILE_VC_VALID_MASK: u32 = 1023;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FILE_VOLUME_NAME_INFORMATION {
    pub DeviceNameLength: u32,
    pub DeviceName: [u16; 1],
}
impl Default for FILE_VOLUME_NAME_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FLAGS_DELAY_REASONS_BITMAP_SCANNED: u32 = 2;
pub const FLAGS_DELAY_REASONS_LOG_FILE_FULL: u32 = 1;
pub const FLAGS_END_OF_FILE_INFO_EX_EXTEND_PAGING: u32 = 1;
pub const FLAGS_END_OF_FILE_INFO_EX_NO_EXTRA_PAGING_EXTEND: u32 = 2;
pub const FLAGS_END_OF_FILE_INFO_EX_TIME_CONSTRAINED: u32 = 4;
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
pub type FREE_VIRTUAL_MEMORY_EX_CALLBACK = Option<unsafe extern "system" fn(callbackcontext: super::winnt::HANDLE, processhandle: super::winnt::HANDLE, baseaddress: *mut *mut core::ffi::c_void, regionsize: *mut usize, freetype: u32) -> super::bcrypt::NTSTATUS>;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FSCTL_GHOST_FILE_EXTENTS_INPUT_BUFFER {
    pub FileOffset: i64,
    pub ByteCount: i64,
    pub RecallOwnerGuid: windows_core::GUID,
    pub RecallMetadataBufferSize: u32,
    pub RecallMetadataBuffer: [u8; 1],
}
impl Default for FSCTL_GHOST_FILE_EXTENTS_INPUT_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FSCTL_LMR_GET_LINK_TRACKING_INFORMATION: u32 = 1310952;
pub const FSCTL_LMR_SET_LINK_TRACKING_INFORMATION: u32 = 1310956;
pub const FSCTL_MAILSLOT_PEEK: u32 = 802819;
pub const FSCTL_PIPE_ASSIGN_EVENT: u32 = 1114112;
pub const FSCTL_PIPE_CREATE_SYMLINK: u32 = 1114188;
pub const FSCTL_PIPE_DELETE_SYMLINK: u32 = 1114192;
pub const FSCTL_PIPE_DISABLE_IMPERSONATE: u32 = 1114180;
pub const FSCTL_PIPE_DISCONNECT: u32 = 1114116;
pub const FSCTL_PIPE_FLUSH: u32 = 1146944;
pub const FSCTL_PIPE_GET_CONNECTION_ATTRIBUTE: u32 = 1114160;
pub const FSCTL_PIPE_GET_HANDLE_ATTRIBUTE: u32 = 1114168;
pub const FSCTL_PIPE_GET_PIPE_ATTRIBUTE: u32 = 1114152;
pub const FSCTL_PIPE_IMPERSONATE: u32 = 1114140;
pub const FSCTL_PIPE_INTERNAL_READ: u32 = 1138676;
pub const FSCTL_PIPE_INTERNAL_READ_OVFLOW: u32 = 1138688;
pub const FSCTL_PIPE_INTERNAL_TRANSCEIVE: u32 = 1171455;
pub const FSCTL_PIPE_INTERNAL_WRITE: u32 = 1155064;
pub const FSCTL_PIPE_LISTEN: u32 = 1114120;
pub const FSCTL_PIPE_PEEK: u32 = 1130508;
pub const FSCTL_PIPE_QUERY_CLIENT_PROCESS: u32 = 1114148;
pub const FSCTL_PIPE_QUERY_CLIENT_PROCESS_V2: u32 = 1114196;
pub const FSCTL_PIPE_QUERY_EVENT: u32 = 1114128;
pub const FSCTL_PIPE_SET_CLIENT_PROCESS: u32 = 1114144;
pub const FSCTL_PIPE_SET_CONNECTION_ATTRIBUTE: u32 = 1114164;
pub const FSCTL_PIPE_SET_HANDLE_ATTRIBUTE: u32 = 1114172;
pub const FSCTL_PIPE_SET_PIPE_ATTRIBUTE: u32 = 1114156;
pub const FSCTL_PIPE_SILO_ARRIVAL: u32 = 1146952;
pub const FSCTL_PIPE_TRANSCEIVE: u32 = 1163287;
pub const FSCTL_PIPE_WAIT: u32 = 1114136;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FSCTL_QUERY_GHOSTED_FILE_EXTENTS_INPUT_RANGE {
    pub FileOffset: i64,
    pub ByteCount: i64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FSCTL_QUERY_VOLUME_NUMA_INFO_OUTPUT {
    pub NumaNode: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FSCTL_UNMAP_SPACE_INPUT_BUFFER {
    pub BytesToUnmap: i64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FSCTL_UNMAP_SPACE_OUTPUT {
    pub BytesUnmapped: i64,
}
pub const FSRTL_ADD_TC_CASE_SENSITIVE: u32 = 1;
pub const FSRTL_ADD_TC_KEY_BY_SHORT_NAME: u32 = 2;
#[repr(C)]
#[cfg(all(feature = "ntdef", feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct FSRTL_ADVANCED_FCB_HEADER {
    pub Base: FSRTL_COMMON_FCB_HEADER,
    pub FastMutex: super::wdm::PFAST_MUTEX,
    pub FilterContexts: super::winnt::LIST_ENTRY,
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct FSRTL_ALLOCATE_ECPLIST_FLAGS(pub u32);
pub const FSRTL_ALLOCATE_ECPLIST_FLAG_CHARGE_QUOTA: u32 = 1;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct FSRTL_ALLOCATE_ECP_FLAGS(pub u32);
pub const FSRTL_ALLOCATE_ECP_FLAG_CHARGE_QUOTA: u32 = 1;
pub const FSRTL_ALLOCATE_ECP_FLAG_NONPAGED_POOL: u32 = 2;
pub const FSRTL_ASYNC_CACHED_READ_TOP_LEVEL_IRP: isize = 7;
#[repr(C)]
#[cfg(feature = "usb")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FSRTL_AUXILIARY_BUFFER {
    pub Buffer: *mut core::ffi::c_void,
    pub Length: u32,
    pub Flags: u32,
    pub Mdl: super::usb::PMDL,
}
#[cfg(feature = "usb")]
impl Default for FSRTL_AUXILIARY_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FSRTL_AUXILIARY_FLAG_DEALLOCATE: u32 = 1;
pub const FSRTL_CACHE_TOP_LEVEL_IRP: isize = 2;
pub const FSRTL_CC_FLUSH_ERROR_FLAG_NO_HARD_ERROR: u32 = 1;
pub const FSRTL_CC_FLUSH_ERROR_FLAG_NO_LOG_ENTRY: u32 = 2;
pub type FSRTL_CHANGE_BACKING_TYPE = i32;
#[repr(C)]
#[cfg(all(feature = "ntdef", feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FSRTL_COMMON_FCB_HEADER {
    pub NodeTypeCode: super::ntdef::CSHORT,
    pub NodeByteSize: super::ntdef::CSHORT,
    pub Flags: u8,
    pub IsFastIoPossible: u8,
    pub Flags2: u8,
    pub _bitfield: u8,
    pub Resource: super::wdm::PERESOURCE,
    pub PagingIoResource: super::wdm::PERESOURCE,
    pub AllocationSize: i64,
    pub FileSize: i64,
    pub ValidDataLength: i64,
}
pub type FSRTL_COMPARISON_RESULT = i32;
pub const FSRTL_DRIVER_BACKING_FLAG_USE_PAGE_FILE: u32 = 1;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct FSRTL_ECP_LOOKASIDE_FLAGS(pub u32);
pub const FSRTL_ECP_LOOKASIDE_FLAG_NONPAGED_POOL: u32 = 2;
pub const FSRTL_FAST_IO_TOP_LEVEL_IRP: isize = 4;
pub const FSRTL_FAT_LEGAL: u32 = 1;
pub const FSRTL_FCB_HEADER_V0: u32 = 0;
pub const FSRTL_FCB_HEADER_V1: u32 = 1;
pub const FSRTL_FCB_HEADER_V2: u32 = 2;
pub const FSRTL_FCB_HEADER_V3: u32 = 3;
pub const FSRTL_FCB_HEADER_V4: u32 = 4;
pub const FSRTL_FCB_HEADER_V5: u32 = 5;
pub const FSRTL_FIND_TC_CASE_SENSITIVE: u32 = 1;
pub const FSRTL_FLAG2_BYPASSIO_STREAM_PAUSED: u32 = 32;
pub const FSRTL_FLAG2_DO_MODIFIED_WRITE: u32 = 1;
pub const FSRTL_FLAG2_IS_PAGING_FILE: u32 = 8;
pub const FSRTL_FLAG2_PURGE_WHEN_MAPPED: u32 = 4;
pub const FSRTL_FLAG2_SUPPORTS_FILTER_CONTEXTS: u32 = 2;
pub const FSRTL_FLAG2_WRITABLE_USER_MAPPED_FILE: u32 = 16;
pub const FSRTL_FLAG_ACQUIRE_MAIN_RSRC_EX: u32 = 8;
pub const FSRTL_FLAG_ACQUIRE_MAIN_RSRC_SH: u32 = 16;
pub const FSRTL_FLAG_ADVANCED_HEADER: u32 = 64;
pub const FSRTL_FLAG_EOF_ADVANCE_ACTIVE: u32 = 128;
pub const FSRTL_FLAG_FILE_LENGTH_CHANGED: u32 = 2;
pub const FSRTL_FLAG_FILE_MODIFIED: u32 = 1;
pub const FSRTL_FLAG_LIMIT_MODIFIED_PAGES: u32 = 4;
pub const FSRTL_FLAG_USER_MAPPED_FILE: u32 = 32;
pub const FSRTL_FSP_TOP_LEVEL_IRP: isize = 1;
pub const FSRTL_HPFS_LEGAL: u32 = 2;
pub const FSRTL_MAX_TOP_LEVEL_IRP_FLAG: isize = 65535;
pub const FSRTL_MOD_WRITE_TOP_LEVEL_IRP: isize = 3;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FSRTL_MUP_PROVIDER_INFO_LEVEL_1 {
    pub ProviderId: u32,
}
#[repr(C)]
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FSRTL_MUP_PROVIDER_INFO_LEVEL_2 {
    pub ProviderId: u32,
    pub ProviderName: super::ntsecapi::UNICODE_STRING,
}
pub const FSRTL_NETWORK1_TOP_LEVEL_IRP: isize = 5;
pub const FSRTL_NETWORK2_TOP_LEVEL_IRP: isize = 6;
pub const FSRTL_NTFS_LEGAL: u32 = 4;
pub const FSRTL_NTFS_STREAM_LEGAL: u32 = 20;
pub const FSRTL_OLE_LEGAL: u32 = 16;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FSRTL_PER_FILEOBJECT_CONTEXT {
    pub Links: super::winnt::LIST_ENTRY,
    pub OwnerId: *mut core::ffi::c_void,
    pub InstanceId: *mut core::ffi::c_void,
}
#[cfg(feature = "winnt")]
impl Default for FSRTL_PER_FILEOBJECT_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FSRTL_PER_FILE_CONTEXT {
    pub Links: super::winnt::LIST_ENTRY,
    pub OwnerId: *mut core::ffi::c_void,
    pub InstanceId: *mut core::ffi::c_void,
    pub FreeCallback: super::wdm::PFREE_FUNCTION,
}
#[cfg(all(feature = "wdm", feature = "winnt"))]
impl Default for FSRTL_PER_FILE_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FSRTL_PER_STREAM_CONTEXT {
    pub Links: super::winnt::LIST_ENTRY,
    pub OwnerId: *mut core::ffi::c_void,
    pub InstanceId: *mut core::ffi::c_void,
    pub FreeCallback: super::wdm::PFREE_FUNCTION,
}
#[cfg(all(feature = "wdm", feature = "winnt"))]
impl Default for FSRTL_PER_STREAM_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct FSRTL_UNC_HARDENING_CAPABILITIES(pub u32);
pub const FSRTL_UNC_HARDENING_CAPABILITIES_INTEGRITY: u32 = 2;
pub const FSRTL_UNC_HARDENING_CAPABILITIES_MUTUAL_AUTH: u32 = 1;
pub const FSRTL_UNC_HARDENING_CAPABILITIES_PRIVACY: u32 = 4;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct FSRTL_UNC_PROVIDER_FLAGS(pub u32);
pub const FSRTL_UNC_PROVIDER_FLAGS_CONTAINER_AWARE: u32 = 8;
pub const FSRTL_UNC_PROVIDER_FLAGS_CSC_ENABLED: u32 = 2;
pub const FSRTL_UNC_PROVIDER_FLAGS_DOMAIN_SVC_AWARE: u32 = 4;
pub const FSRTL_UNC_PROVIDER_FLAGS_MAILSLOTS_SUPPORTED: u32 = 1;
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FSRTL_UNC_PROVIDER_REGISTRATION_0_0 {
    pub _bitfield: u32,
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FSRTL_UNC_PROVIDER_REGISTRATION_1_0 {
    pub _bitfield: u32,
}
pub const FSRTL_UNC_REGISTRATION_CURRENT_VERSION: u32 = 513;
pub const FSRTL_UNC_REGISTRATION_VERSION_0200: u32 = 512;
pub const FSRTL_UNC_REGISTRATION_VERSION_0201: u32 = 513;
pub const FSRTL_VIRTDISK_FULLY_ALLOCATED: u32 = 1;
pub const FSRTL_VIRTDISK_NO_DRIVE_LETTER: u32 = 2;
pub const FSRTL_VOLSNAP_TOP_LEVEL_IRP: isize = 8;
pub const FSRTL_VOLUME_BACKGROUND_FORMAT: u32 = 14;
pub const FSRTL_VOLUME_CHANGE_SIZE: u32 = 13;
pub const FSRTL_VOLUME_DISMOUNT: u32 = 1;
pub const FSRTL_VOLUME_DISMOUNT_FAILED: u32 = 2;
pub const FSRTL_VOLUME_FORCED_CLOSED: u32 = 10;
pub const FSRTL_VOLUME_INFO_MAKE_COMPAT: u32 = 11;
pub const FSRTL_VOLUME_LOCK: u32 = 3;
pub const FSRTL_VOLUME_LOCK_FAILED: u32 = 4;
pub const FSRTL_VOLUME_MOUNT: u32 = 6;
pub const FSRTL_VOLUME_NEEDS_CHKDSK: u32 = 7;
pub const FSRTL_VOLUME_PREPARING_EJECT: u32 = 12;
pub const FSRTL_VOLUME_UNLOCK: u32 = 5;
pub const FSRTL_VOLUME_WEARING_OUT: u32 = 9;
pub const FSRTL_VOLUME_WORM_NEAR_FULL: u32 = 8;
pub const FSRTL_WILD_CHARACTER: u32 = 8;
pub const FS_FILTER_ACQUIRE_FOR_CC_FLUSH: u8 = 251;
pub const FS_FILTER_ACQUIRE_FOR_MOD_WRITE: u8 = 253;
pub const FS_FILTER_ACQUIRE_FOR_SECTION_SYNCHRONIZATION: u8 = 255;
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "minwindef", feature = "ntdef", feature = "ntsecapi", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
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
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "minwindef", feature = "ntdef", feature = "ntsecapi", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy)]
pub struct FS_FILTER_CALLBACK_DATA {
    pub SizeOfFsFilterCallbackData: u32,
    pub Operation: u8,
    pub Reserved: u8,
    pub DeviceObject: *mut super::wdm::DEVICE_OBJECT,
    pub FileObject: *mut super::wdm::FILE_OBJECT,
    pub Parameters: FS_FILTER_PARAMETERS,
}
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "minwindef", feature = "ntdef", feature = "ntsecapi", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
impl Default for FS_FILTER_CALLBACK_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "minwindef", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy)]
pub union FS_FILTER_PARAMETERS {
    pub AcquireForModifiedPageWriter: FS_FILTER_PARAMETERS_0,
    pub ReleaseForModifiedPageWriter: FS_FILTER_PARAMETERS_1,
    pub AcquireForSectionSynchronization: FS_FILTER_PARAMETERS_2,
    pub QueryOpen: FS_FILTER_PARAMETERS_3,
    pub Others: FS_FILTER_PARAMETERS_4,
}
#[cfg(all(feature = "bcrypt", feature = "minwindef", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
impl Default for FS_FILTER_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "minwindef", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FS_FILTER_PARAMETERS_0 {
    pub EndingOffset: super::winnt::PLARGE_INTEGER,
    pub ResourceToRelease: *mut super::wdm::PERESOURCE,
}
#[cfg(all(feature = "bcrypt", feature = "minwindef", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
impl Default for FS_FILTER_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "minwindef", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FS_FILTER_PARAMETERS_1 {
    pub ResourceToRelease: super::wdm::PERESOURCE,
}
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "minwindef", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FS_FILTER_PARAMETERS_2 {
    pub SyncType: FS_FILTER_SECTION_SYNC_TYPE,
    pub PageProtection: u32,
    pub OutputInformation: PFS_FILTER_SECTION_SYNC_OUTPUT,
    pub Flags: u32,
    pub AllocationAttributes: u32,
}
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "minwindef", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FS_FILTER_PARAMETERS_3 {
    pub Irp: super::usb::PIRP,
    pub FileInformation: *mut core::ffi::c_void,
    pub Length: super::minwindef::PULONG,
    pub FileInformationClass: super::winternl::FILE_INFORMATION_CLASS,
    pub CompletionStatus: super::bcrypt::NTSTATUS,
}
#[cfg(all(feature = "bcrypt", feature = "minwindef", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
impl Default for FS_FILTER_PARAMETERS_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "minwindef", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FS_FILTER_PARAMETERS_4 {
    pub Argument1: *mut core::ffi::c_void,
    pub Argument2: *mut core::ffi::c_void,
    pub Argument3: *mut core::ffi::c_void,
    pub Argument4: *mut core::ffi::c_void,
    pub Argument5: *mut core::ffi::c_void,
}
#[cfg(all(feature = "bcrypt", feature = "minwindef", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
impl Default for FS_FILTER_PARAMETERS_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FS_FILTER_QUERY_OPEN: u8 = 249;
pub const FS_FILTER_RELEASE_FOR_CC_FLUSH: u8 = 250;
pub const FS_FILTER_RELEASE_FOR_MOD_WRITE: u8 = 252;
pub const FS_FILTER_RELEASE_FOR_SECTION_SYNCHRONIZATION: u8 = 254;
pub const FS_FILTER_SECTION_SYNC_IMAGE_EXTENTS_ARE_NOT_RVA: u32 = 8;
pub const FS_FILTER_SECTION_SYNC_IN_FLAG_DONT_UPDATE_LAST_ACCESS: u32 = 1;
pub const FS_FILTER_SECTION_SYNC_IN_FLAG_DONT_UPDATE_LAST_WRITE: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FS_FILTER_SECTION_SYNC_OUTPUT {
    pub StructureSize: u32,
    pub SizeReturned: u32,
    pub Flags: u32,
    pub DesiredReadAlignment: u32,
}
pub const FS_FILTER_SECTION_SYNC_SUPPORTS_ASYNC_PARALLEL_IO: u32 = 1;
pub const FS_FILTER_SECTION_SYNC_SUPPORTS_DIRECT_MAP_DATA: u32 = 2;
pub const FS_FILTER_SECTION_SYNC_SUPPORTS_DIRECT_MAP_IMAGE: u32 = 4;
pub type FS_FILTER_SECTION_SYNC_TYPE = i32;
pub type FS_FILTER_STREAM_FO_NOTIFICATION_TYPE = i32;
pub const FastIoIsNotPossible: FAST_IO_POSSIBLE = 0;
pub const FastIoIsPossible: FAST_IO_POSSIBLE = 1;
pub const FastIoIsQuestionable: FAST_IO_POSSIBLE = 2;
pub const GCR_ALLOW_LM: u32 = 4096;
pub const GCR_ALLOW_NO_TARGET: u32 = 8192;
pub const GCR_ALLOW_NTLM: u32 = 256;
pub const GCR_MACHINE_CREDENTIAL: u32 = 1024;
pub const GCR_NTLM3_PARMS: u32 = 32;
pub const GCR_TARGET_INFO: u32 = 64;
pub const GCR_USE_OEM_SET: u32 = 512;
pub const GCR_USE_OWF_PASSWORD: u32 = 2048;
pub const GCR_VSM_PROTECTED_PASSWORD: u32 = 16384;
pub const GENERATE_CLIENT_CHALLENGE: u32 = 16;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GENERATE_NAME_CONTEXT {
    pub Checksum: u16,
    pub ChecksumInserted: bool,
    pub NameLength: u8,
    pub NameBuffer: [u16; 8],
    pub ExtensionLength: u32,
    pub ExtensionBuffer: [u16; 4],
    pub LastIndexValue: u32,
}
impl Default for GENERATE_NAME_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GHOSTED_FILE_EXTENT {
    pub FileOffset: i64,
    pub ByteCount: i64,
    pub RecallOwnerGuid: windows_core::GUID,
    pub NextEntryOffset: u32,
    pub RecallMetadataBufferSize: u32,
    pub RecallMetadataBuffer: [u8; 1],
}
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
pub const HEAP_CLASS_0: u32 = 0;
pub const HEAP_CLASS_1: u32 = 4096;
pub const HEAP_CLASS_2: u32 = 8192;
pub const HEAP_CLASS_3: u32 = 12288;
pub const HEAP_CLASS_4: u32 = 16384;
pub const HEAP_CLASS_5: u32 = 20480;
pub const HEAP_CLASS_6: u32 = 24576;
pub const HEAP_CLASS_7: u32 = 28672;
pub const HEAP_CLASS_8: u32 = 32768;
pub const HEAP_CLASS_MASK: u32 = 61440;
pub const HEAP_CREATE_VALID_MASK: u32 = 521215;
pub const HEAP_GLOBAL_TAG: u32 = 2048;
pub type HEAP_MEMORY_INFO_CLASS = i32;
pub const HEAP_SETTABLE_USER_FLAG1: u32 = 512;
pub const HEAP_SETTABLE_USER_FLAG2: u32 = 1024;
pub const HEAP_SETTABLE_USER_FLAG3: u32 = 2048;
pub const HEAP_SETTABLE_USER_FLAGS: u32 = 3584;
pub const HEAP_SETTABLE_USER_VALUE: u32 = 256;
pub const HEAP_TAG_MASK: u32 = 1073479680;
pub const HeapMemoryBasicInformation: HEAP_MEMORY_INFO_CLASS = 0;
pub const IOCTL_LMR_ARE_FILE_OBJECTS_ON_SAME_SERVER: u32 = 1310960;
pub const IOCTL_REDIR_QUERY_PATH: u32 = 1311119;
pub const IOCTL_REDIR_QUERY_PATH_EX: u32 = 1311123;
pub const IOCTL_VOLSNAP_FLUSH_AND_HOLD_WRITES: u32 = 5488640;
pub const IO_CREATE_STREAM_FILE_LITE: u32 = 2;
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntsecapi", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_CREATE_STREAM_FILE_OPTIONS {
    pub Size: u16,
    pub Flags: u16,
    pub TargetDeviceObject: super::wdm::PDEVICE_OBJECT,
}
pub const IO_CREATE_STREAM_FILE_RAISE_ON_ERROR: u32 = 1;
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntsecapi", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_DEVICE_HINT_ECP_CONTEXT {
    pub TargetDevice: super::wdm::PDEVICE_OBJECT,
    pub RemainingName: super::ntsecapi::UNICODE_STRING,
}
pub const IO_FILE_OBJECT_NON_PAGED_POOL_CHARGE: u32 = 64;
pub const IO_FILE_OBJECT_PAGED_POOL_CHARGE: u32 = 1024;
pub const IO_IGNORE_READONLY_ATTRIBUTE: u32 = 64;
pub const IO_MM_PAGING_FILE: u32 = 16;
pub const IO_OPEN_PAGING_FILE: u32 = 2;
pub const IO_OPEN_TARGET_DIRECTORY: u32 = 4;
#[repr(C)]
#[cfg(feature = "wdm")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_PRIORITY_INFO {
    pub Size: u32,
    pub ThreadPriority: u32,
    pub PagePriority: u32,
    pub IoPriority: super::wdm::IO_PRIORITY_HINT,
}
pub const IO_REPARSE_TAG_ACRONIS_HSM_0: u32 = 96;
pub const IO_REPARSE_TAG_ACRONIS_HSM_1: u32 = 97;
pub const IO_REPARSE_TAG_ACRONIS_HSM_2: u32 = 98;
pub const IO_REPARSE_TAG_ACRONIS_HSM_3: u32 = 99;
pub const IO_REPARSE_TAG_ACRONIS_HSM_4: u32 = 100;
pub const IO_REPARSE_TAG_ACRONIS_HSM_5: u32 = 101;
pub const IO_REPARSE_TAG_ACRONIS_HSM_6: u32 = 102;
pub const IO_REPARSE_TAG_ACRONIS_HSM_7: u32 = 103;
pub const IO_REPARSE_TAG_ACRONIS_HSM_8: u32 = 104;
pub const IO_REPARSE_TAG_ACRONIS_HSM_9: u32 = 105;
pub const IO_REPARSE_TAG_ACRONIS_HSM_A: u32 = 106;
pub const IO_REPARSE_TAG_ACRONIS_HSM_B: u32 = 107;
pub const IO_REPARSE_TAG_ACRONIS_HSM_C: u32 = 108;
pub const IO_REPARSE_TAG_ACRONIS_HSM_D: u32 = 109;
pub const IO_REPARSE_TAG_ACRONIS_HSM_E: u32 = 110;
pub const IO_REPARSE_TAG_ACRONIS_HSM_F: u32 = 111;
pub const IO_REPARSE_TAG_ACTIVISION_HSM: u32 = 71;
pub const IO_REPARSE_TAG_ADA_HSM: u32 = 38;
pub const IO_REPARSE_TAG_ADOBE_HSM: u32 = 69;
pub const IO_REPARSE_TAG_ALERTBOOT: u32 = 536870988;
pub const IO_REPARSE_TAG_ALTIRIS_HSM: u32 = 25;
pub const IO_REPARSE_TAG_AMZN_APPSTREAM: u32 = 89;
pub const IO_REPARSE_TAG_APPXSTRM: u32 = 3221225492;
pub const IO_REPARSE_TAG_ARCO_BACKUP: u32 = 59;
pub const IO_REPARSE_TAG_ARKIVIO: u32 = 12;
pub const IO_REPARSE_TAG_AURISTOR_FS: u32 = 73;
pub const IO_REPARSE_TAG_AUTN_HSM: u32 = 39;
pub const IO_REPARSE_TAG_BRIDGEHEAD_HSM: u32 = 22;
pub const IO_REPARSE_TAG_C2CSYSTEMS_HSM: u32 = 49;
pub const IO_REPARSE_TAG_CARINGO_HSM: u32 = 52;
pub const IO_REPARSE_TAG_CARROLL_HSM: u32 = 60;
pub const IO_REPARSE_TAG_CITRIX_PM: u32 = 54;
pub const IO_REPARSE_TAG_COMMVAULT: u32 = 14;
pub const IO_REPARSE_TAG_COMMVAULT_HSM: u32 = 29;
pub const IO_REPARSE_TAG_COMTRADE_HSM: u32 = 61;
pub const IO_REPARSE_TAG_CTERA_HSM: u32 = 78;
pub const IO_REPARSE_TAG_DATAFIRST_HSM: u32 = 48;
pub const IO_REPARSE_TAG_DATAGLOBAL_HSM: u32 = 46;
pub const IO_REPARSE_TAG_DATASTOR_SIS: u32 = 30;
pub const IO_REPARSE_TAG_DFM: u32 = 2147483670;
pub const IO_REPARSE_TAG_DOR_HSM: u32 = 82;
pub const IO_REPARSE_TAG_DOUBLE_TAKE_HSM: u32 = 34;
pub const IO_REPARSE_TAG_DOUBLE_TAKE_SIS: u32 = 41;
pub const IO_REPARSE_TAG_DRIVE_EXTENDER: u32 = 2147483653;
pub const IO_REPARSE_TAG_DROPBOX_HSM: u32 = 68;
pub const IO_REPARSE_TAG_EASEFILTER_HSM: u32 = 87;
pub const IO_REPARSE_TAG_EASEVAULT_HSM: u32 = 62;
pub const IO_REPARSE_TAG_EDSI_HSM: u32 = 31;
pub const IO_REPARSE_TAG_ELTAN_HSM: u32 = 43;
pub const IO_REPARSE_TAG_EMC_HSM: u32 = 57;
pub const IO_REPARSE_TAG_ENIGMA_HSM: u32 = 17;
pub const IO_REPARSE_TAG_FILTER_MANAGER: u32 = 2147483659;
pub const IO_REPARSE_TAG_GLOBAL360_HSM: u32 = 24;
pub const IO_REPARSE_TAG_GOOGLE_HSM: u32 = 65;
pub const IO_REPARSE_TAG_GRAU_DATASTORAGE_HSM: u32 = 28;
pub const IO_REPARSE_TAG_HDS_HCP_HSM: u32 = 72;
pub const IO_REPARSE_TAG_HDS_HSM: u32 = 63;
pub const IO_REPARSE_TAG_HERMES_HSM: u32 = 26;
pub const IO_REPARSE_TAG_HP_BACKUP: u32 = 67;
pub const IO_REPARSE_TAG_HP_DATA_PROTECT: u32 = 70;
pub const IO_REPARSE_TAG_HP_HSM: u32 = 32;
pub const IO_REPARSE_TAG_HSAG_HSM: u32 = 37;
pub const IO_REPARSE_TAG_HUBSTOR_HSM: u32 = 85;
pub const IO_REPARSE_TAG_IFSTEST_CONGRUENT: u32 = 9;
pub const IO_REPARSE_TAG_IIS_CACHE: u32 = 2684354576;
pub const IO_REPARSE_TAG_IMANAGE_HSM: u32 = 536870998;
pub const IO_REPARSE_TAG_INTERCOPE_HSM: u32 = 19;
pub const IO_REPARSE_TAG_ITSTATION: u32 = 74;
pub const IO_REPARSE_TAG_KOM_NETWORKS_HSM: u32 = 20;
pub const IO_REPARSE_TAG_LX_BLK: u32 = 2147483686;
pub const IO_REPARSE_TAG_LX_CHR: u32 = 2147483685;
pub const IO_REPARSE_TAG_LX_FIFO: u32 = 2147483684;
pub const IO_REPARSE_TAG_LX_SYMLINK: u32 = 2684354589;
pub const IO_REPARSE_TAG_MAGINATICS_RDR: u32 = 64;
pub const IO_REPARSE_TAG_MAXISCALE_HSM: u32 = 536870965;
pub const IO_REPARSE_TAG_MEMORY_TECH_HSM: u32 = 21;
pub const IO_REPARSE_TAG_MIMOSA_HSM: u32 = 36;
pub const IO_REPARSE_TAG_MOONWALK_HSM: u32 = 10;
pub const IO_REPARSE_TAG_MTALOS: u32 = 77;
pub const IO_REPARSE_TAG_NEUSHIELD: u32 = 81;
pub const IO_REPARSE_TAG_NEXSAN_HSM: u32 = 40;
pub const IO_REPARSE_TAG_NIPPON_HSM: u32 = 79;
pub const IO_REPARSE_TAG_NVIDIA_UNIONFS: u32 = 536870996;
pub const IO_REPARSE_TAG_OPENAFS_DFS: u32 = 55;
pub const IO_REPARSE_TAG_OSR_SAMPLE: u32 = 536870935;
pub const IO_REPARSE_TAG_OVERTONE: u32 = 15;
pub const IO_REPARSE_TAG_PEER_GFS: u32 = 88;
pub const IO_REPARSE_TAG_POINTSOFT_HSM: u32 = 27;
pub const IO_REPARSE_TAG_QI_TECH_HSM: u32 = 536870959;
pub const IO_REPARSE_TAG_QUADDRA_HSM: u32 = 66;
pub const IO_REPARSE_TAG_QUEST_HSM: u32 = 45;
pub const IO_REPARSE_TAG_REDSTOR_HSM: u32 = 80;
pub const IO_REPARSE_TAG_RIVERBED_HSM: u32 = 51;
pub const IO_REPARSE_TAG_SER_HSM: u32 = 33;
pub const IO_REPARSE_TAG_SHX_BACKUP: u32 = 83;
pub const IO_REPARSE_TAG_SOLUTIONSOFT: u32 = 536870925;
pub const IO_REPARSE_TAG_SONY_HSM: u32 = 42;
pub const IO_REPARSE_TAG_SPHARSOFT: u32 = 75;
pub const IO_REPARSE_TAG_SYMANTEC_HSM: u32 = 18;
pub const IO_REPARSE_TAG_SYMANTEC_HSM2: u32 = 16;
pub const IO_REPARSE_TAG_TSINGHUA_UNIVERSITY_RESEARCH: u32 = 11;
pub const IO_REPARSE_TAG_UTIXO_HSM: u32 = 44;
pub const IO_REPARSE_TAG_VALID_VALUES: u32 = 4026597375;
pub const IO_REPARSE_TAG_VMWARE_PM: u32 = 58;
pub const IO_REPARSE_TAG_WATERFORD: u32 = 50;
pub const IO_REPARSE_TAG_WISDATA_HSM: u32 = 35;
pub const IO_REPARSE_TAG_ZLTI_HSM: u32 = 56;
pub const IO_STOP_ON_SYMLINK: u32 = 8;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_STOP_ON_SYMLINK_FILTER_ECP_v0 {
    pub Out: IO_STOP_ON_SYMLINK_FILTER_ECP_v0_0,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_STOP_ON_SYMLINK_FILTER_ECP_v0_0 {
    pub ReparseCount: u32,
    pub RemainingPathLength: u32,
}
#[repr(C)]
#[cfg(all(feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct KAPC_STATE {
    pub ApcListHead: [super::winnt::LIST_ENTRY; 2],
    pub Process: *mut super::wdm::_KPROCESS,
    pub Anonymous: KAPC_STATE_0,
    pub KernelApcPending: bool,
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KAPC_STATE_0_0 {
    pub _bitfield: bool,
}
#[repr(C)]
#[cfg(all(feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union KAPC_STATE_1 {
    pub UserApcPendingAll: bool,
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KAPC_STATE_1_0 {
    pub _bitfield: bool,
}
pub const KAPC_STATE_ANY_USER_APC_PENDING_MASK: u32 = 3;
pub const KAPC_STATE_NORMAL_USER_APC_PENDING_MASK: u32 = 1;
pub const KAPC_STATE_SPECIAL_USER_APC_PENDING_MASK: u32 = 2;
#[repr(C)]
#[cfg(all(feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct KQUEUE {
    pub Header: super::wdm::DISPATCHER_HEADER,
    pub EntryListHead: super::winnt::LIST_ENTRY,
    pub CurrentCount: u32,
    pub MaximumCount: u32,
    pub ThreadListHead: super::winnt::LIST_ENTRY,
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LARGE_MCB {
    pub GuardedMutex: super::wdm::PKGUARDED_MUTEX,
    pub BaseMcb: BASE_MCB,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LBN(pub u32);
pub const LCN_CHECKSUM_VALID: LCN_WEAK_REFERENCE_STATE = 4;
pub const LCN_IS_READ_ONLY: LCN_WEAK_REFERENCE_STATE = 32;
pub const LCN_IS_STREAM_RESERVED: LCN_WEAK_REFERENCE_STATE = 16;
pub const LCN_IS_VALID: LCN_WEAK_REFERENCE_STATE = 8;
pub const LCN_WEAK_REFERENCE_BROKEN: LCN_WEAK_REFERENCE_STATE = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LCN_WEAK_REFERENCE_BUFFER {
    pub Lcn: i64,
    pub LengthInClusters: i64,
    pub ReferenceCount: u32,
    pub State: LCN_WEAK_REFERENCE_STATE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LCN_WEAK_REFERENCE_CLEAR_INPUT_BUFFER {
    pub RangeCount: u32,
    pub Ranges: [LCN_WEAK_REFERENCE_RANGE; 1],
}
impl Default for LCN_WEAK_REFERENCE_CLEAR_INPUT_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LCN_WEAK_REFERENCE_CREATE_FLAGS(pub u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LCN_WEAK_REFERENCE_CREATE_INPUT_BUFFER {
    pub Offset: i64,
    pub Length: i64,
    pub Flags: u32,
    pub Reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LCN_WEAK_REFERENCE_RANGE {
    pub StartOfRange: i64,
    pub CountOfRange: i64,
}
pub type LCN_WEAK_REFERENCE_STATE = u32;
pub const LCN_WEAK_REFERENCE_VALID: LCN_WEAK_REFERENCE_STATE = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LCN_WEAK_REFERENCE_VCN_MAPPING {
    pub Vcn: i64,
    pub Lcn: i64,
    pub CountOfRange: i64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub const MAP_DISABLE_PAGEFAULT_CLUSTERING: u32 = 256;
pub const MAP_HIGH_PRIORITY: u32 = 64;
pub const MAP_NO_READ: u32 = 16;
pub const MAP_WAIT: u32 = 1;
pub const MAX_UNICODE_STACK_BUFFER_LENGTH: u32 = 256;
#[repr(C)]
#[cfg(all(feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MCB {
    pub DummyFieldThatSizesThisStructureCorrectly: LARGE_MCB,
}
pub const MCB_FLAG_RAISE_ON_ALLOCATION_FAILURE: u32 = 1;
pub type MEMORY_INFORMATION_CLASS = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MEMORY_RANGE_ENTRY {
    pub VirtualAddress: *mut core::ffi::c_void,
    pub NumberOfBytes: usize,
}
impl Default for MEMORY_RANGE_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type MMFLUSH_TYPE = i32;
pub const MM_FORCE_CLOSED_DATA: u32 = 1;
pub const MM_FORCE_CLOSED_IMAGE: u32 = 2;
pub const MM_FORCE_CLOSED_LATER_OK: u32 = 4;
pub const MM_IS_FILE_SECTION_ACTIVE_DATA: u32 = 2;
pub const MM_IS_FILE_SECTION_ACTIVE_IMAGE: u32 = 1;
pub const MM_IS_FILE_SECTION_ACTIVE_USER: u32 = 4;
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MM_PREFETCH_FLAGS_0 {
    pub _bitfield: u32,
}
pub const MM_PREFETCH_FLAGS_MASK: u32 = 127;
#[repr(C)]
#[cfg(feature = "ntsecapi")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MSV1_0_ENUMUSERS_REQUEST {
    pub MessageType: super::ntsecapi::MSV1_0_PROTOCOL_MESSAGE_TYPE,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "ntsecapi", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MSV1_0_ENUMUSERS_RESPONSE {
    pub MessageType: super::ntsecapi::MSV1_0_PROTOCOL_MESSAGE_TYPE,
    pub NumberOfLoggedOnUsers: u32,
    pub LogonIds: super::winnt::PLUID,
    pub EnumHandles: super::minwindef::PULONG,
}
#[repr(C)]
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MSV1_0_GETCHALLENRESP_REQUEST {
    pub MessageType: super::ntsecapi::MSV1_0_PROTOCOL_MESSAGE_TYPE,
    pub ParameterControl: u32,
    pub LogonId: super::winnt::LUID,
    pub Password: super::ntsecapi::UNICODE_STRING,
    pub ChallengeToClient: [u8; 8],
    pub UserName: super::ntsecapi::UNICODE_STRING,
    pub LogonDomainName: super::ntsecapi::UNICODE_STRING,
    pub ServerName: super::ntsecapi::UNICODE_STRING,
}
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
impl Default for MSV1_0_GETCHALLENRESP_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MSV1_0_GETCHALLENRESP_REQUEST_V1 {
    pub MessageType: super::ntsecapi::MSV1_0_PROTOCOL_MESSAGE_TYPE,
    pub ParameterControl: u32,
    pub LogonId: super::winnt::LUID,
    pub Password: super::ntsecapi::UNICODE_STRING,
    pub ChallengeToClient: [u8; 8],
}
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
impl Default for MSV1_0_GETCHALLENRESP_REQUEST_V1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MSV1_0_GETCHALLENRESP_RESPONSE {
    pub MessageType: super::ntsecapi::MSV1_0_PROTOCOL_MESSAGE_TYPE,
    pub CaseSensitiveChallengeResponse: super::ntsecapi::STRING,
    pub CaseInsensitiveChallengeResponse: super::ntsecapi::STRING,
    pub UserName: super::ntsecapi::UNICODE_STRING,
    pub LogonDomainName: super::ntsecapi::UNICODE_STRING,
    pub UserSessionKey: [u8; 16],
    pub LanmanSessionKey: [u8; 8],
}
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
impl Default for MSV1_0_GETCHALLENRESP_RESPONSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ntsecapi", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MSV1_0_GETUSERINFO_REQUEST {
    pub MessageType: super::ntsecapi::MSV1_0_PROTOCOL_MESSAGE_TYPE,
    pub LogonId: super::winnt::LUID,
}
#[repr(C)]
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MSV1_0_GETUSERINFO_RESPONSE {
    pub MessageType: super::ntsecapi::MSV1_0_PROTOCOL_MESSAGE_TYPE,
    pub UserSid: super::winnt::PSID,
    pub UserName: super::ntsecapi::UNICODE_STRING,
    pub LogonDomainName: super::ntsecapi::UNICODE_STRING,
    pub LogonServer: super::ntsecapi::UNICODE_STRING,
    pub LogonType: super::ntsecapi::SECURITY_LOGON_TYPE,
}
#[repr(C)]
#[cfg(feature = "ntsecapi")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MSV1_0_LM20_CHALLENGE_REQUEST {
    pub MessageType: super::ntsecapi::MSV1_0_PROTOCOL_MESSAGE_TYPE,
}
#[repr(C)]
#[cfg(feature = "ntsecapi")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MSV1_0_LM20_CHALLENGE_RESPONSE {
    pub MessageType: super::ntsecapi::MSV1_0_PROTOCOL_MESSAGE_TYPE,
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MUP_PROVIDER_INFORMATION {
    pub Level: u32,
    pub Buffer: *mut core::ffi::c_void,
    pub BufferSize: super::minwindef::PULONG,
}
#[cfg(feature = "minwindef")]
impl Default for MUP_PROVIDER_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NETWORK_APP_INSTANCE_ECP_CONTEXT {
    pub Size: u16,
    pub Reserved: u16,
    pub AppInstanceID: windows_core::GUID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NETWORK_APP_INSTANCE_VERSION_ECP_CONTEXT {
    pub Size: u16,
    pub Reserved: u16,
    pub VersionHigh: u64,
    pub VersionLow: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NETWORK_OPEN_ECP_CONTEXT {
    pub Size: u16,
    pub Reserved: u16,
    pub Anonymous: NETWORK_OPEN_ECP_CONTEXT_0,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NETWORK_OPEN_ECP_CONTEXT_0 {
    pub r#in: NETWORK_OPEN_ECP_CONTEXT_0_0,
    pub out: NETWORK_OPEN_ECP_CONTEXT_0_1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NETWORK_OPEN_ECP_CONTEXT_0_0 {
    pub Location: NETWORK_OPEN_LOCATION_QUALIFIER,
    pub Integrity: NETWORK_OPEN_INTEGRITY_QUALIFIER,
    pub Flags: NETWORK_OPEN_IN_FLAGS,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NETWORK_OPEN_ECP_CONTEXT_0_1 {
    pub Location: NETWORK_OPEN_LOCATION_QUALIFIER,
    pub Integrity: NETWORK_OPEN_INTEGRITY_QUALIFIER,
    pub Flags: NETWORK_OPEN_OUT_FLAGS,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NETWORK_OPEN_ECP_CONTEXT_V0 {
    pub Size: u16,
    pub Reserved: u16,
    pub Anonymous: NETWORK_OPEN_ECP_CONTEXT_V0_0,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NETWORK_OPEN_ECP_CONTEXT_V0_0 {
    pub r#in: NETWORK_OPEN_ECP_CONTEXT_V0_0_0,
    pub out: NETWORK_OPEN_ECP_CONTEXT_V0_0_1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NETWORK_OPEN_ECP_CONTEXT_V0_0_0 {
    pub Location: NETWORK_OPEN_LOCATION_QUALIFIER,
    pub Integrity: NETWORK_OPEN_INTEGRITY_QUALIFIER,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NETWORK_OPEN_ECP_CONTEXT_V0_0_1 {
    pub Location: NETWORK_OPEN_LOCATION_QUALIFIER,
    pub Integrity: NETWORK_OPEN_INTEGRITY_QUALIFIER,
}
pub const NETWORK_OPEN_ECP_IN_FLAG_DISABLE_HANDLE_COLLAPSING: u32 = 1;
pub const NETWORK_OPEN_ECP_IN_FLAG_DISABLE_HANDLE_DURABILITY: u32 = 2;
pub const NETWORK_OPEN_ECP_IN_FLAG_DISABLE_OPLOCKS: u32 = 4;
pub const NETWORK_OPEN_ECP_IN_FLAG_FORCE_BUFFERED_SYNCHRONOUS_IO_HACK: u32 = 2147483648;
pub const NETWORK_OPEN_ECP_IN_FLAG_FORCE_MAX_EOF_HACK: u32 = 1073741824;
pub const NETWORK_OPEN_ECP_IN_FLAG_REQ_MUTUAL_AUTH: u32 = 8;
pub const NETWORK_OPEN_ECP_OUT_FLAG_RET_MUTUAL_AUTH: u32 = 8;
pub type NETWORK_OPEN_INTEGRITY_QUALIFIER = i32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct NETWORK_OPEN_IN_FLAGS(pub u32);
pub type NETWORK_OPEN_LOCATION_QUALIFIER = i32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct NETWORK_OPEN_OUT_FLAGS(pub u32);
#[repr(C)]
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NFS_OPEN_ECP_CONTEXT {
    pub ExportAlias: super::ntsecapi::PUNICODE_STRING,
    pub ClientSocketAddress: PSOCKADDR_STORAGE_NFS,
}
pub const NO_8DOT3_NAME_PRESENT: u32 = 1;
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct OPEN_REPARSE_LIST {
    pub OpenReparseList: super::winnt::LIST_ENTRY,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct OPEN_REPARSE_LIST_ENTRY {
    pub OpenReparseListEntry: super::winnt::LIST_ENTRY,
    pub ReparseTag: u32,
    pub Flags: u32,
    pub ReparseGuid: windows_core::GUID,
    pub Size: u16,
    pub RemainingLength: u16,
}
pub const OPEN_REPARSE_POINT_OVERRIDE_CREATE_OPTION: u32 = 64;
pub const OPEN_REPARSE_POINT_REPARSE_ALWAYS: u32 = 126;
pub const OPEN_REPARSE_POINT_REPARSE_IF_CHILD_EXISTS: u32 = 2;
pub const OPEN_REPARSE_POINT_REPARSE_IF_CHILD_NOT_EXISTS: u32 = 4;
pub const OPEN_REPARSE_POINT_REPARSE_IF_DIRECTORY_FINAL_COMPONENT: u32 = 8;
pub const OPEN_REPARSE_POINT_REPARSE_IF_DIRECTORY_FINAL_COMPONENT_ALWAYS: u32 = 72;
pub const OPEN_REPARSE_POINT_REPARSE_IF_FINAL_COMPONENT: u32 = 40;
pub const OPEN_REPARSE_POINT_REPARSE_IF_FINAL_COMPONENT_ALWAYS: u32 = 104;
pub const OPEN_REPARSE_POINT_REPARSE_IF_NON_DIRECTORY_FINAL_COMPONENT: u32 = 32;
pub const OPEN_REPARSE_POINT_REPARSE_IF_NON_DIRECTORY_FINAL_COMPONENT_ALWAYS: u32 = 96;
pub const OPEN_REPARSE_POINT_REPARSE_IF_NON_DIRECTORY_NON_FINAL_COMPONENT: u32 = 16;
pub const OPEN_REPARSE_POINT_REPARSE_IF_NON_DIRECTORY_NON_FINAL_COMPONENT_ALWAYS: u32 = 80;
pub const OPEN_REPARSE_POINT_REPARSE_IF_NON_FINAL_COMPONENT: u32 = 22;
pub const OPEN_REPARSE_POINT_RETURN_REPARSE_DATA_BUFFER: u32 = 128;
pub const OPEN_REPARSE_POINT_TAG_ENCOUNTERED: u32 = 1;
pub const OPEN_REPARSE_POINT_VERSION_EX: u32 = 2147483648;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OPLOCK(pub *mut core::ffi::c_void);
impl OPLOCK {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for OPLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const OPLOCK_FLAG_BACK_OUT_ATOMIC_OPLOCK: u32 = 4;
pub const OPLOCK_FLAG_BREAKING_FOR_SHARING_VIOLATION: u32 = 128;
pub const OPLOCK_FLAG_CLOSING_DELETE_ON_CLOSE: u32 = 32;
pub const OPLOCK_FLAG_COMPLETE_IF_OPLOCKED: u32 = 1;
pub const OPLOCK_FLAG_IGNORE_OPLOCK_KEYS: u32 = 8;
pub const OPLOCK_FLAG_OPLOCK_KEY_CHECK_ONLY: u32 = 2;
pub const OPLOCK_FLAG_PARENT_OBJECT: u32 = 16;
pub const OPLOCK_FLAG_REMOVING_FILE_OR_LINK: u32 = 64;
pub const OPLOCK_FSCTRL_FLAG_ALL_KEYS_MATCH: u32 = 1;
pub const OPLOCK_FS_FILTER_FLAGS_MASK: u32 = 8;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct OPLOCK_KEY_ECP_CONTEXT {
    pub OplockKey: windows_core::GUID,
    pub Reserved: u32,
}
pub const OPLOCK_NOTIFY_BREAK_WAIT_INTERIM_TIMEOUT: OPLOCK_NOTIFY_REASON = 0;
pub const OPLOCK_NOTIFY_BREAK_WAIT_TERMINATED: OPLOCK_NOTIFY_REASON = 1;
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "usb"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OPLOCK_NOTIFY_PARAMS {
    pub NotifyReason: OPLOCK_NOTIFY_REASON,
    pub NotifyContext: *mut core::ffi::c_void,
    pub Irp: super::usb::PIRP,
    pub Status: super::bcrypt::NTSTATUS,
}
#[cfg(all(feature = "bcrypt", feature = "usb"))]
impl Default for OPLOCK_NOTIFY_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type OPLOCK_NOTIFY_REASON = i32;
pub const OPLOCK_UPPER_FLAG_CHECK_NO_BREAK: u32 = 65536;
pub const OPLOCK_UPPER_FLAG_NOTIFY_REFRESH_READ: u32 = 131072;
pub type PACQUIRE_FOR_LAZY_WRITE = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, wait: bool) -> bool>;
pub type PACQUIRE_FOR_LAZY_WRITE_EX = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, inflags: u32, outflags: *mut u32) -> bool>;
pub type PACQUIRE_FOR_READ_AHEAD = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, wait: bool) -> bool>;
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
pub type PALLOCATE_VIRTUAL_MEMORY_EX_CALLBACK = *mut ALLOCATE_VIRTUAL_MEMORY_EX_CALLBACK;
pub type PASYNC_READ_COMPLETION_CALLBACK = Option<unsafe extern "system" fn(context: *const core::ffi::c_void) -> bool>;
#[cfg(feature = "winnt")]
pub type PATOMIC_CREATE_ECP_CONTEXT = *mut ATOMIC_CREATE_ECP_CONTEXT;
pub type PBASE_MCB = *mut BASE_MCB;
pub type PCACHE_MANAGER_CALLBACKS = *mut CACHE_MANAGER_CALLBACKS;
pub type PCACHE_MANAGER_CALLBACKS_EX = *mut CACHE_MANAGER_CALLBACKS_EX;
pub type PCACHE_MANAGER_CALLBACK_FUNCTIONS = *mut CACHE_MANAGER_CALLBACK_FUNCTIONS;
#[cfg(all(feature = "wdm", feature = "winnt"))]
pub type PCACHE_UNINITIALIZE_EVENT = *mut CACHE_UNINITIALIZE_EVENT;
#[cfg(all(feature = "usb", feature = "wdm", feature = "winnt"))]
pub type PCC_ASYNC_READ_CONTEXT = *mut CC_ASYNC_READ_CONTEXT;
pub type PCC_DIRTY_PAGES_INFO = *mut CC_DIRTY_PAGES_INFO;
#[cfg(all(feature = "bcrypt", feature = "ntdef"))]
pub type PCC_ERROR_CALLBACK_CONTEXT = *mut CC_ERROR_CALLBACK_CONTEXT;
pub type PCC_FILE_SIZES = *mut CC_FILE_SIZES;
pub type PCC_POST_DEFERRED_WRITE = Option<unsafe extern "system" fn(context1: *const core::ffi::c_void, context2: *const core::ffi::c_void)>;
#[cfg(all(feature = "wdm", feature = "winnt"))]
pub type PCHECK_FOR_TRAVERSE_ACCESS = Option<unsafe extern "system" fn(notifycontext: *const core::ffi::c_void, targetcontext: *const core::ffi::c_void, subjectcontext: *const super::wdm::SECURITY_SUBJECT_CONTEXT) -> bool>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntsecapi", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
pub type PCOMPLETE_LOCK_IRP_ROUTINE = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, irp: *const super::wdm::IRP) -> super::bcrypt::NTSTATUS>;
pub type PCOMPRESSED_DATA_INFO = *mut COMPRESSED_DATA_INFO;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntsecapi", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
pub type PCOPY_INFORMATION = *mut COPY_INFORMATION;
#[cfg(feature = "winnt")]
pub type PCREATE_REDIRECTION_ECP_CONTEXT = *mut CREATE_REDIRECTION_ECP_CONTEXT;
pub type PCSV_DOWN_LEVEL_OPEN_ECP_CONTEXT = *mut CSV_DOWN_LEVEL_OPEN_ECP_CONTEXT;
pub type PCSV_QUERY_FILE_REVISION_ECP_CONTEXT = *mut CSV_QUERY_FILE_REVISION_ECP_CONTEXT;
#[cfg(feature = "winnt")]
pub type PCSV_QUERY_FILE_REVISION_ECP_CONTEXT_FILE_ID_128 = *mut CSV_QUERY_FILE_REVISION_ECP_CONTEXT_FILE_ID_128;
pub type PCSV_SET_HANDLE_PROPERTIES_ECP_CONTEXT = *mut CSV_SET_HANDLE_PROPERTIES_ECP_CONTEXT;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntsecapi", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
pub type PDIRTY_PAGE_ROUTINE = Option<unsafe extern "system" fn(fileobject: *const super::wdm::FILE_OBJECT, fileoffset: *const i64, length: u32, oldestlsn: *const i64, newestlsn: *const i64, context1: *const core::ffi::c_void, context2: *const core::ffi::c_void)>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntsecapi", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
pub type PDRIVER_FS_NOTIFICATION = *mut DRIVER_FS_NOTIFICATION;
pub type PDUAL_OPLOCK_KEY_ECP_CONTEXT = *mut DUAL_OPLOCK_KEY_ECP_CONTEXT;
pub type PDUPLICATE_CLUSTER_DATA = *mut DUPLICATE_CLUSTER_DATA;
pub type PECP_HEADER = *mut ECP_HEADER;
#[cfg(feature = "ntddk")]
pub type PECP_LIST = *mut super::ntddk::ECP_LIST;
pub type PECP_OPEN_PARAMETERS = *mut ECP_OPEN_PARAMETERS;
#[cfg(all(feature = "wdm", feature = "winnt"))]
pub type PEOF_WAIT_BLOCK = *mut EOF_WAIT_BLOCK;
pub type PEXTENT_READ_CACHE_INFO_BUFFER = *mut EXTENT_READ_CACHE_INFO_BUFFER;
pub type PEXTERNAL_CACHE_CALLBACK_EX = Option<unsafe extern "system" fn(externalcachecontext: *const core::ffi::c_void, dirtypagesinfo: *const CC_DIRTY_PAGES_INFO)>;
#[cfg(feature = "winnt")]
pub type PFILE_ACCESS_INFORMATION = *mut FILE_ACCESS_INFORMATION;
pub type PFILE_ALLOCATION_INFORMATION = *mut FILE_ALLOCATION_INFORMATION;
#[cfg(all(feature = "ntddk", feature = "wdm", feature = "winnt"))]
pub type PFILE_ALL_INFORMATION = *mut FILE_ALL_INFORMATION;
#[cfg(feature = "winnt")]
pub type PFILE_BOTH_DIR_INFORMATION = *mut FILE_BOTH_DIR_INFORMATION;
#[cfg(feature = "winnt")]
pub type PFILE_COMPLETION_INFORMATION = *mut FILE_COMPLETION_INFORMATION;
pub type PFILE_COMPRESSION_INFORMATION = *mut FILE_COMPRESSION_INFORMATION;
pub type PFILE_DIRECTORY_INFORMATION = *mut FILE_DIRECTORY_INFORMATION;
pub type PFILE_EA_INFORMATION = *mut FILE_EA_INFORMATION;
pub type PFILE_END_OF_FILE_INFORMATION_EX = *mut FILE_END_OF_FILE_INFORMATION_EX;
pub type PFILE_FS_ATTRIBUTE_INFORMATION = *mut FILE_FS_ATTRIBUTE_INFORMATION;
pub type PFILE_FS_CONTROL_INFORMATION = *mut FILE_FS_CONTROL_INFORMATION;
pub type PFILE_FS_DATA_COPY_INFORMATION = *mut FILE_FS_DATA_COPY_INFORMATION;
pub type PFILE_FS_DRIVER_PATH_INFORMATION = *mut FILE_FS_DRIVER_PATH_INFORMATION;
pub type PFILE_FS_GUID_INFORMATION = *mut FILE_FS_GUID_INFORMATION;
pub type PFILE_FS_VOLUME_FLAGS_INFORMATION = *mut FILE_FS_VOLUME_FLAGS_INFORMATION;
pub type PFILE_FULL_DIR_INFORMATION = *mut FILE_FULL_DIR_INFORMATION;
pub type PFILE_GET_EA_INFORMATION = *mut FILE_GET_EA_INFORMATION;
#[cfg(feature = "winnt")]
pub type PFILE_ID_64_EXTD_BOTH_DIR_INFORMATION = *mut FILE_ID_64_EXTD_BOTH_DIR_INFORMATION;
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
pub type PFILE_ID_FULL_DIR_INFORMATION = *mut FILE_ID_FULL_DIR_INFORMATION;
pub type PFILE_ID_GLOBAL_TX_DIR_INFORMATION = *mut FILE_ID_GLOBAL_TX_DIR_INFORMATION;
#[cfg(feature = "winnt")]
pub type PFILE_ID_INFORMATION = *mut FILE_ID_INFORMATION;
#[cfg(feature = "winternl")]
pub type PFILE_INFORMATION_DEFINITION = *mut FILE_INFORMATION_DEFINITION;
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
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntsecapi", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
pub type PFILE_LOCK = *mut FILE_LOCK;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntsecapi", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
pub type PFILE_LOCK_INFO = *mut FILE_LOCK_INFO;
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
pub type PFILE_PIPE_REMOTE_INFORMATION = *mut FILE_PIPE_REMOTE_INFORMATION;
#[cfg(feature = "winnt")]
pub type PFILE_PIPE_SILO_ARRIVAL_INPUT = *mut FILE_PIPE_SILO_ARRIVAL_INPUT;
pub type PFILE_PIPE_WAIT_FOR_BUFFER = *mut FILE_PIPE_WAIT_FOR_BUFFER;
#[cfg(feature = "winnt")]
pub type PFILE_QUOTA_INFORMATION = *mut FILE_QUOTA_INFORMATION;
pub type PFILE_REMOTE_PROTOCOL_INFORMATION = *mut FILE_REMOTE_PROTOCOL_INFORMATION;
#[cfg(feature = "winnt")]
pub type PFILE_RENAME_INFORMATION = *mut FILE_RENAME_INFORMATION;
pub type PFILE_REPARSE_POINT_INFORMATION = *mut FILE_REPARSE_POINT_INFORMATION;
pub type PFILE_STANDARD_LINK_INFORMATION = *mut FILE_STANDARD_LINK_INFORMATION;
#[cfg(feature = "winioctl")]
pub type PFILE_STORAGE_RESERVE_ID_INFORMATION = *mut FILE_STORAGE_RESERVE_ID_INFORMATION;
pub type PFILE_STREAM_INFORMATION = *mut FILE_STREAM_INFORMATION;
pub type PFILE_STREAM_RESERVATION_INFORMATION = *mut FILE_STREAM_RESERVATION_INFORMATION;
pub type PFILE_TIMESTAMPS = *mut FILE_TIMESTAMPS;
#[cfg(feature = "winnt")]
pub type PFILE_TRACKING_INFORMATION = *mut FILE_TRACKING_INFORMATION;
pub type PFILE_VOLUME_NAME_INFORMATION = *mut FILE_VOLUME_NAME_INFORMATION;
pub type PFILTER_REPORT_CHANGE = Option<unsafe extern "system" fn(notifycontext: *const core::ffi::c_void, filtercontext: *const core::ffi::c_void) -> bool>;
pub type PFLUSH_TO_LSN = Option<unsafe extern "system" fn(loghandle: *const core::ffi::c_void, lsn: i64)>;
#[cfg(all(feature = "ntdef", feature = "wdm", feature = "winnt"))]
pub type PFN_FSRTLTEARDOWNPERSTREAMCONTEXTS = Option<unsafe extern "system" fn(advancedheader: *const FSRTL_ADVANCED_FCB_HEADER)>;
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
pub type PFREE_VIRTUAL_MEMORY_EX_CALLBACK = *mut FREE_VIRTUAL_MEMORY_EX_CALLBACK;
pub type PFSCTL_GHOST_FILE_EXTENTS_INPUT_BUFFER = *mut FSCTL_GHOST_FILE_EXTENTS_INPUT_BUFFER;
pub type PFSCTL_QUERY_GHOSTED_FILE_EXTENTS_INPUT_RANGE = *mut FSCTL_QUERY_GHOSTED_FILE_EXTENTS_INPUT_RANGE;
pub type PFSCTL_QUERY_GHOSTED_FILE_EXTENTS_OUTPUT = *mut FSCTL_QUERY_GHOSTED_FILE_EXTENTS_OUTPUT;
pub type PFSCTL_QUERY_VOLUME_NUMA_INFO_OUTPUT = *mut FSCTL_QUERY_VOLUME_NUMA_INFO_OUTPUT;
pub type PFSCTL_UNMAP_SPACE_INPUT_BUFFER = *mut FSCTL_UNMAP_SPACE_INPUT_BUFFER;
pub type PFSCTL_UNMAP_SPACE_OUTPUT = *mut FSCTL_UNMAP_SPACE_OUTPUT;
#[cfg(all(feature = "ntdef", feature = "wdm", feature = "winnt"))]
pub type PFSRTL_ADVANCED_FCB_HEADER = *mut FSRTL_ADVANCED_FCB_HEADER;
#[cfg(feature = "usb")]
pub type PFSRTL_AUXILIARY_BUFFER = *mut FSRTL_AUXILIARY_BUFFER;
pub type PFSRTL_CHANGE_BACKING_TYPE = *mut FSRTL_CHANGE_BACKING_TYPE;
#[cfg(all(feature = "ntdef", feature = "wdm", feature = "winnt"))]
pub type PFSRTL_COMMON_FCB_HEADER = *mut FSRTL_COMMON_FCB_HEADER;
pub type PFSRTL_EXTRA_CREATE_PARAMETER_CLEANUP_CALLBACK = Option<unsafe extern "system" fn(ecpcontext: *mut core::ffi::c_void, ecptype: *const windows_core::GUID)>;
pub type PFSRTL_MUP_PROVIDER_INFO_LEVEL_1 = *mut FSRTL_MUP_PROVIDER_INFO_LEVEL_1;
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
pub type PFSRTL_MUP_PROVIDER_INFO_LEVEL_2 = *mut FSRTL_MUP_PROVIDER_INFO_LEVEL_2;
#[cfg(feature = "winnt")]
pub type PFSRTL_PER_FILEOBJECT_CONTEXT = *mut FSRTL_PER_FILEOBJECT_CONTEXT;
#[cfg(all(feature = "wdm", feature = "winnt"))]
pub type PFSRTL_PER_FILE_CONTEXT = *mut FSRTL_PER_FILE_CONTEXT;
#[cfg(all(feature = "wdm", feature = "winnt"))]
pub type PFSRTL_PER_STREAM_CONTEXT = *mut FSRTL_PER_STREAM_CONTEXT;
#[cfg(all(feature = "wdm", feature = "winnt"))]
pub type PFSRTL_STACK_OVERFLOW_ROUTINE = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, event: *const super::wdm::KEVENT)>;
pub type PFSRTL_UNC_PROVIDER_REGISTRATION = *mut FSRTL_UNC_PROVIDER_REGISTRATION;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "minwindef", feature = "ntdef", feature = "ntsecapi", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
pub type PFS_FILTER_CALLBACK = Option<unsafe extern "system" fn(data: *const FS_FILTER_CALLBACK_DATA, completioncontext: *mut *mut core::ffi::c_void) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "minwindef", feature = "ntdef", feature = "ntsecapi", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
pub type PFS_FILTER_CALLBACKS = *mut FS_FILTER_CALLBACKS;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "minwindef", feature = "ntdef", feature = "ntsecapi", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
pub type PFS_FILTER_CALLBACK_DATA = *mut FS_FILTER_CALLBACK_DATA;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "minwindef", feature = "ntdef", feature = "ntsecapi", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
pub type PFS_FILTER_COMPLETION_CALLBACK = Option<unsafe extern "system" fn(data: *const FS_FILTER_CALLBACK_DATA, operationstatus: super::bcrypt::NTSTATUS, completioncontext: *const core::ffi::c_void)>;
#[cfg(all(feature = "bcrypt", feature = "minwindef", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
pub type PFS_FILTER_PARAMETERS = *mut FS_FILTER_PARAMETERS;
pub type PFS_FILTER_SECTION_SYNC_OUTPUT = *mut FS_FILTER_SECTION_SYNC_OUTPUT;
pub type PFS_FILTER_SECTION_SYNC_TYPE = *mut FS_FILTER_SECTION_SYNC_TYPE;
pub type PFS_FILTER_STREAM_FO_NOTIFICATION_TYPE = *mut FS_FILTER_STREAM_FO_NOTIFICATION_TYPE;
pub type PGENERATE_NAME_CONTEXT = *mut GENERATE_NAME_CONTEXT;
pub type PGHOSTED_FILE_EXTENT = *mut GHOSTED_FILE_EXTENT;
pub const PHCM_APPLICATION_DEFAULT: i8 = 0;
pub const PHCM_DISGUISE_FULL_PLACEHOLDERS: i8 = 3;
pub const PHCM_DISGUISE_PLACEHOLDERS: i8 = 1;
pub const PHCM_ERROR_INVALID_PARAMETER: i8 = -1;
pub const PHCM_ERROR_NO_PEB: i8 = -3;
pub const PHCM_ERROR_NO_TEB: i8 = -2;
pub const PHCM_EXPOSE_PLACEHOLDERS: i8 = 2;
pub const PHCM_MAX: i8 = 2;
#[repr(C)]
#[cfg(feature = "wdm")]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PHYSICAL_MEMORY_DESCRIPTOR {
    pub NumberOfRuns: u32,
    pub NumberOfPages: super::wdm::PFN_NUMBER,
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PHYSICAL_MEMORY_RUN {
    pub BasePage: super::wdm::PFN_NUMBER,
    pub PageCount: super::wdm::PFN_NUMBER,
}
pub const PIN_CALLER_TRACKS_DIRTY_DATA: u32 = 32;
pub const PIN_EXCLUSIVE: u32 = 2;
pub const PIN_HIGH_PRIORITY: u32 = 64;
pub const PIN_IF_BCB: u32 = 8;
pub const PIN_NO_READ: u32 = 4;
pub const PIN_VERIFY_REQUIRED: u32 = 128;
pub const PIN_WAIT: u32 = 1;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntsecapi", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
pub type PIO_CREATE_STREAM_FILE_OPTIONS = *mut IO_CREATE_STREAM_FILE_OPTIONS;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntsecapi", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
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
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
pub type PMSV1_0_GETCHALLENRESP_REQUEST = *mut MSV1_0_GETCHALLENRESP_REQUEST;
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
pub type PMSV1_0_GETCHALLENRESP_REQUEST_V1 = *mut MSV1_0_GETCHALLENRESP_REQUEST_V1;
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
pub type PMSV1_0_GETCHALLENRESP_RESPONSE = *mut MSV1_0_GETCHALLENRESP_RESPONSE;
#[cfg(all(feature = "ntsecapi", feature = "winnt"))]
pub type PMSV1_0_GETUSERINFO_REQUEST = *mut MSV1_0_GETUSERINFO_REQUEST;
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct POPLOCK(pub *mut *mut core::ffi::c_void);
impl POPLOCK {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for POPLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntsecapi", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
pub type POPLOCK_FS_PREPOST_IRP = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, irp: *const super::wdm::IRP)>;
#[cfg(all(feature = "bcrypt", feature = "usb"))]
pub type POPLOCK_NOTIFY_PARAMS = *mut OPLOCK_NOTIFY_PARAMS;
#[cfg(all(feature = "bcrypt", feature = "usb"))]
pub type POPLOCK_NOTIFY_ROUTINE = Option<unsafe extern "system" fn(notifyparams: *const OPLOCK_NOTIFY_PARAMS) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntsecapi", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
pub type POPLOCK_WAIT_COMPLETE_ROUTINE = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, irp: *const super::wdm::IRP)>;
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
#[cfg(feature = "ntdef")]
pub type PPUBLIC_BCB = *mut PUBLIC_BCB;
pub type PQUERY_DIRECT_ACCESS_EXTENTS = *mut QUERY_DIRECT_ACCESS_EXTENTS;
pub type PQUERY_LOG_USAGE = Option<unsafe extern "system" fn(loghandle: *const core::ffi::c_void, percentagefull: *mut u16)>;
#[cfg(feature = "wdm")]
pub type PQUERY_ON_CREATE_EA_INFORMATION = *mut QUERY_ON_CREATE_EA_INFORMATION;
#[cfg(all(feature = "wdm", feature = "winnt"))]
pub type PQUERY_ON_CREATE_ECP_CONTEXT = *mut QUERY_ON_CREATE_ECP_CONTEXT;
#[cfg(feature = "winnt")]
pub type PQUERY_ON_CREATE_FILE_LX_INFORMATION = *mut QUERY_ON_CREATE_FILE_LX_INFORMATION;
pub type PQUERY_ON_CREATE_FILE_STAT_INFORMATION = *mut QUERY_ON_CREATE_FILE_STAT_INFORMATION;
#[cfg(feature = "winnt")]
pub type PQUERY_ON_CREATE_SECURITY_INFORMATION = *mut QUERY_ON_CREATE_SECURITY_INFORMATION;
#[cfg(feature = "winnt")]
pub type PQUERY_ON_CREATE_USN_INFORMATION = *mut QUERY_ON_CREATE_USN_INFORMATION;
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "wdm", feature = "winnt"))]
pub type PQUERY_PATH_REQUEST = *mut QUERY_PATH_REQUEST;
#[cfg(all(feature = "lsalookup", feature = "ntddk", feature = "ntsecapi", feature = "wdm", feature = "winnt"))]
pub type PQUERY_PATH_REQUEST_EX = *mut QUERY_PATH_REQUEST_EX;
pub type PQUERY_PATH_RESPONSE = *mut QUERY_PATH_RESPONSE;
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
pub type PQUERY_VIRTUAL_MEMORY_CALLBACK = *mut QUERY_VIRTUAL_MEMORY_CALLBACK;
#[cfg(feature = "ntdef")]
pub type PREAD_AHEAD_PARAMETERS = *mut READ_AHEAD_PARAMETERS;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntsecapi", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
pub type PREAD_LIST = *mut READ_LIST;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PREFETCH_OPEN_ECP_CONTEXT {
    pub Context: *mut core::ffi::c_void,
}
impl Default for PREFETCH_OPEN_ECP_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "lsalookup", feature = "ntddk", feature = "ntdef", feature = "ntsecapi", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PREFIX_TABLE {
    pub NodeTypeCode: super::ntdef::CSHORT,
    pub NameLength: super::ntdef::CSHORT,
    pub NextPrefixTree: PPREFIX_TABLE_ENTRY,
}
#[repr(C)]
#[cfg(all(feature = "lsalookup", feature = "ntddk", feature = "ntdef", feature = "ntsecapi", feature = "winnt"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PREFIX_TABLE_ENTRY {
    pub NodeTypeCode: super::ntdef::CSHORT,
    pub NameLength: super::ntdef::CSHORT,
    pub NextPrefixTree: *mut Self,
    pub Links: super::ntddk::RTL_SPLAY_LINKS,
    pub Prefix: super::ntsecapi::PSTRING,
}
#[cfg(all(feature = "lsalookup", feature = "ntddk", feature = "ntdef", feature = "ntsecapi", feature = "winnt"))]
impl Default for PREFIX_TABLE_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PREFS_COMPRESSION_FORMATS = *mut REFS_COMPRESSION_FORMATS;
pub type PREFS_DEALLOCATE_RANGES_ALLOCATOR = *mut REFS_DEALLOCATE_RANGES_ALLOCATOR;
pub type PREFS_DEALLOCATE_RANGES_INPUT_BUFFER = *mut REFS_DEALLOCATE_RANGES_INPUT_BUFFER;
pub type PREFS_DEALLOCATE_RANGES_INPUT_BUFFER_EX = *mut REFS_DEALLOCATE_RANGES_INPUT_BUFFER_EX;
pub type PREFS_DEALLOCATE_RANGES_RANGE = *mut REFS_DEALLOCATE_RANGES_RANGE;
pub type PREFS_QUERY_ROLLBACK_PROTECTION_INFO_OUTPUT_BUFFER = *mut REFS_QUERY_ROLLBACK_PROTECTION_INFO_OUTPUT_BUFFER;
pub type PREFS_QUERY_VOLUME_COMPRESSION_INFO_FLAGS = *mut REFS_QUERY_VOLUME_COMPRESSION_INFO_FLAGS;
#[cfg(feature = "bcrypt")]
pub type PREFS_QUERY_VOLUME_COMPRESSION_INFO_OUTPUT_BUFFER = *mut REFS_QUERY_VOLUME_COMPRESSION_INFO_OUTPUT_BUFFER;
pub type PREFS_QUERY_VOLUME_IO_METRICS_INFO_INPUT_BUFFER = *mut REFS_QUERY_VOLUME_IO_METRICS_INFO_INPUT_BUFFER;
pub type PREFS_QUERY_VOLUME_IO_METRICS_INFO_OUTPUT_BUFFER = *mut REFS_QUERY_VOLUME_IO_METRICS_INFO_OUTPUT_BUFFER;
pub type PREFS_QUERY_VOLUME_IO_METRICS_INFO_QUERY_TYPE = *mut REFS_QUERY_VOLUME_IO_METRICS_INFO_QUERY_TYPE;
pub type PREFS_QUERY_VOLUME_IO_METRICS_METRICS_DATA = *mut REFS_QUERY_VOLUME_IO_METRICS_METRICS_DATA;
pub type PREFS_QUERY_VOLUME_TOTAL_SHARED_LCNS_OUTPUT_BUFFER = *mut REFS_QUERY_VOLUME_TOTAL_SHARED_LCNS_OUTPUT_BUFFER;
pub type PREFS_REMOVE_HARDLINK_BACKPOINTER = *mut REFS_REMOVE_HARDLINK_BACKPOINTER;
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
pub type PREFS_VOLUME_COUNTER_INFO_INPUT_BUFFER = *mut REFS_VOLUME_COUNTER_INFO_INPUT_BUFFER;
pub type PREFS_VOLUME_DEDUP_INFO_INPUT_BUFFER = *mut REFS_VOLUME_DEDUP_INFO_INPUT_BUFFER;
pub type PREFS_VOLUME_DEDUP_INFO_OUTPUT_BUFFER = *mut REFS_VOLUME_DEDUP_INFO_OUTPUT_BUFFER;
pub type PRELEASE_FROM_LAZY_WRITE = Option<unsafe extern "system" fn(context: *const core::ffi::c_void)>;
pub type PRELEASE_FROM_READ_AHEAD = Option<unsafe extern "system" fn(context: *const core::ffi::c_void)>;
pub type PREMOTE_LINK_TRACKING_INFORMATION = *mut REMOTE_LINK_TRACKING_INFORMATION;
pub type PREPARSE_DATA_BUFFER = *mut REPARSE_DATA_BUFFER;
#[cfg(feature = "winnt")]
pub type PREPARSE_DATA_BUFFER_EX = *mut REPARSE_DATA_BUFFER_EX;
pub type PREPARSE_INDEX_KEY = *mut REPARSE_INDEX_KEY;
#[cfg(all(feature = "wdm", feature = "winnt"))]
pub type PRKAPC_STATE = *mut KAPC_STATE;
pub type PRKF_BYPASS_ECP_CONTEXT = *mut RKF_BYPASS_ECP_CONTEXT;
#[cfg(all(feature = "wdm", feature = "winnt"))]
pub type PRKQUEUE = *mut KQUEUE;
pub type PRTL_ALLOCATE_STRING_ROUTINE = Option<unsafe extern "system" fn(numberofbytes: usize) -> *mut core::ffi::c_void>;
pub type PRTL_FREE_STRING_ROUTINE = Option<unsafe extern "system" fn(buffer: *const core::ffi::c_void)>;
#[cfg(feature = "bcrypt")]
pub type PRTL_HEAP_COMMIT_ROUTINE = Option<unsafe extern "system" fn(base: *const core::ffi::c_void, commitaddress: *mut *mut core::ffi::c_void, commitsize: *mut usize) -> super::bcrypt::NTSTATUS>;
pub type PRTL_HEAP_MEMORY_LIMIT_DATA = *mut RTL_HEAP_MEMORY_LIMIT_DATA;
pub type PRTL_HEAP_MEMORY_LIMIT_INFO = *mut RTL_HEAP_MEMORY_LIMIT_INFO;
#[cfg(feature = "bcrypt")]
pub type PRTL_HEAP_PARAMETERS = *mut RTL_HEAP_PARAMETERS;
pub type PRTL_MEMORY_TYPE = *mut RTL_MEMORY_TYPE;
pub type PRTL_REALLOCATE_STRING_ROUTINE = Option<unsafe extern "system" fn(numberofbytes: usize, buffer: *const core::ffi::c_void) -> *mut core::ffi::c_void>;
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
pub type PRTL_SEGMENT_HEAP_MEMORY_SOURCE = *mut RTL_SEGMENT_HEAP_MEMORY_SOURCE;
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
pub type PRTL_SEGMENT_HEAP_PARAMETERS = *mut RTL_SEGMENT_HEAP_PARAMETERS;
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
pub type PRTL_SEGMENT_HEAP_VA_CALLBACKS = *mut RTL_SEGMENT_HEAP_VA_CALLBACKS;
#[cfg(feature = "winnt")]
pub type PSECURITY_CLIENT_CONTEXT = *mut SECURITY_CLIENT_CONTEXT;
pub type PSET_CACHED_RUNS_STATE_INPUT_BUFFER = *mut SET_CACHED_RUNS_STATE_INPUT_BUFFER;
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
pub type PSE_AUDIT_INFO = *mut SE_AUDIT_INFO;
pub type PSE_AUDIT_OPERATION = *mut SE_AUDIT_OPERATION;
#[cfg(feature = "winnt")]
pub type PSE_EXPORTS = *mut SE_EXPORTS;
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
pub type PSE_LOGON_SESSION_TERMINATED_ROUTINE = *mut SE_LOGON_SESSION_TERMINATED_ROUTINE;
#[cfg(all(feature = "bcrypt", feature = "ntddk", feature = "winnt"))]
pub type PSE_LOGON_SESSION_TERMINATED_ROUTINE_EX = *mut SE_LOGON_SESSION_TERMINATED_ROUTINE_EX;
pub const PSMP_MAXIMUM_SYSAPP_CLAIM_VALUES: u32 = 4;
pub const PSMP_MINIMUM_SYSAPP_CLAIM_VALUES: u32 = 2;
pub type PSOCKADDR_STORAGE_NFS = *mut sockaddr_storage;
pub type PSOV_RANGE_CHECK_DATA = *mut SOV_RANGE_CHECK_DATA;
pub type PSRV_INSTANCE_TYPE = *mut SRV_INSTANCE_TYPE;
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
pub type PSRV_OPEN_ECP_CONTEXT = *mut SRV_OPEN_ECP_CONTEXT;
#[cfg(all(feature = "ntddk", feature = "wdm", feature = "winnt"))]
pub type PTUNNEL = *mut TUNNEL;
#[repr(C)]
#[cfg(feature = "ntdef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PUBLIC_BCB {
    pub NodeTypeCode: super::ntdef::CSHORT,
    pub NodeByteSize: super::ntdef::CSHORT,
    pub MappedLength: u32,
    pub MappedFileOffset: i64,
}
#[cfg(all(feature = "lsalookup", feature = "ntddk", feature = "ntdef", feature = "ntsecapi"))]
pub type PUNICODE_PREFIX_TABLE = *mut UNICODE_PREFIX_TABLE;
#[cfg(all(feature = "lsalookup", feature = "ntddk", feature = "ntdef", feature = "ntsecapi"))]
pub type PUNICODE_PREFIX_TABLE_ENTRY = *mut UNICODE_PREFIX_TABLE_ENTRY;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntsecapi", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
pub type PUNLOCK_ROUTINE = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, filelockinfo: *const FILE_LOCK_INFO)>;
pub const PURGE_WITH_ACTIVE_VIEWS: u32 = 8;
pub type PVBN = *mut VBN;
pub type PVCN_RANGE_INPUT_BUFFER = *mut VCN_RANGE_INPUT_BUFFER;
pub type PVETO_BINDING_ECP_CONTEXT = *mut VETO_BINDING_ECP_CONTEXT;
pub type PVOLUME_REFS_INFO_BUFFER = *mut VOLUME_REFS_INFO_BUFFER;
pub const QUERY_DIRECT_ACCESS_DATA_EXTENTS: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct QUERY_DIRECT_ACCESS_EXTENTS {
    pub FileOffset: i64,
    pub Length: i64,
    pub Flags: u32,
    pub Reserved: u32,
}
pub const QUERY_DIRECT_ACCESS_IMAGE_EXTENTS: u32 = 1;
#[repr(C)]
#[cfg(feature = "wdm")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct QUERY_ON_CREATE_EA_INFORMATION {
    pub EaBufferSize: u32,
    pub EaBuffer: super::wdm::PFILE_FULL_EA_INFORMATION,
}
#[repr(C)]
#[cfg(all(feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy, Debug, PartialEq)]
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
    pub SecurityInformationRequested: super::winnt::SECURITY_INFORMATION,
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct QUERY_ON_CREATE_FILE_LX_INFORMATION {
    pub EffectiveAccess: super::winnt::ACCESS_MASK,
    pub LxFlags: u32,
    pub LxUid: u32,
    pub LxGid: u32,
    pub LxMode: u32,
    pub LxDeviceIdMajor: u32,
    pub LxDeviceIdMinor: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct QUERY_ON_CREATE_FILE_STAT_INFORMATION {
    pub FileId: i64,
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub AllocationSize: i64,
    pub EndOfFile: i64,
    pub FileAttributes: u32,
    pub ReparseTag: u32,
    pub NumberOfLinks: u32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct QUERY_ON_CREATE_SECURITY_INFORMATION {
    pub Reserved: u32,
    pub SecurityDescriptorSize: u32,
    pub SecurityDescriptor: super::winnt::PSECURITY_DESCRIPTOR,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct QUERY_ON_CREATE_USN_INFORMATION {
    pub Usn: super::winnt::USN,
    pub FileReferenceNumber: super::winnt::FILE_ID_128,
}
#[repr(C)]
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct QUERY_PATH_REQUEST {
    pub PathNameLength: u32,
    pub SecurityContext: super::wdm::PIO_SECURITY_CONTEXT,
    pub FilePathName: [u16; 1],
}
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "wdm", feature = "winnt"))]
impl Default for QUERY_PATH_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "lsalookup", feature = "ntddk", feature = "ntsecapi", feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct QUERY_PATH_REQUEST_EX {
    pub pSecurityContext: super::wdm::PIO_SECURITY_CONTEXT,
    pub EaLength: u32,
    pub pEaBuffer: *mut core::ffi::c_void,
    pub PathName: super::ntsecapi::UNICODE_STRING,
    pub DomainServiceName: super::ntsecapi::UNICODE_STRING,
    pub EcpList: PECP_LIST,
    pub Silo: super::ntddk::PESILO,
    pub Reserved: usize,
}
#[cfg(all(feature = "lsalookup", feature = "ntddk", feature = "ntsecapi", feature = "wdm", feature = "winnt"))]
impl Default for QUERY_PATH_REQUEST_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct QUERY_PATH_RESPONSE {
    pub LengthAccepted: u32,
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
pub type QUERY_VIRTUAL_MEMORY_CALLBACK = Option<unsafe extern "system" fn(callbackcontext: super::winnt::HANDLE, processhandle: super::winnt::HANDLE, baseaddress: *const core::ffi::c_void, memoryinformationclass: HEAP_MEMORY_INFO_CLASS, memoryinformation: *mut core::ffi::c_void, memoryinformationlength: usize, returnlength: *mut usize) -> super::bcrypt::NTSTATUS>;
pub const QoCFileEaInformation: u32 = 4;
pub const QoCFileLxInformation: u32 = 2;
pub const QoCFileSecurityInformation: u32 = 16;
pub const QoCFileStatInformation: u32 = 1;
pub const QoCFileUsnInformation: u32 = 8;
#[repr(C)]
#[cfg(feature = "ntdef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct READ_AHEAD_PARAMETERS {
    pub NodeByteSize: super::ntdef::CSHORT,
    pub Granularity: u32,
    pub PipelinedRequestSize: u32,
    pub ReadAheadGrowthPercentage: u32,
}
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntsecapi", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy)]
pub struct READ_LIST {
    pub FileObject: super::wdm::PFILE_OBJECT,
    pub NumberOfEntries: u32,
    pub IsImage: super::ntdef::LOGICAL,
    pub List: [super::winnt::FILE_SEGMENT_ELEMENT; 1],
}
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntsecapi", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct REFS_DEALLOCATE_RANGES_RANGE {
    pub StartOfRange: u64,
    pub CountOfRange: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct REFS_QUERY_ROLLBACK_PROTECTION_INFO_OUTPUT_BUFFER {
    pub Version: u32,
    pub VolumeGuid: windows_core::GUID,
    pub RollbackProtectionGuid: windows_core::GUID,
    pub FailMountOnMismatch: bool,
    pub FrozenVirtualClock: u64,
    pub CurrentVirtualClock: u64,
    pub ChecksumType: u16,
    pub ChecksumLength: u32,
    pub ChecksumOffset: u32,
    pub CustomPayloadLength: u32,
    pub CustomPayloadOffset: u32,
}
pub const REFS_QUERY_ROLLBACK_PROTECTION_INFO_OUTPUT_BUFFER_VERSION: u32 = 1;
pub type REFS_QUERY_VOLUME_COMPRESSION_INFO_FLAGS = i32;
pub const REFS_QUERY_VOLUME_COMPRESSION_INFO_FLAGS_RUNNING: REFS_QUERY_VOLUME_COMPRESSION_INFO_FLAGS = 1;
pub const REFS_QUERY_VOLUME_COMPRESSION_INFO_FLAGS_STOPPED: REFS_QUERY_VOLUME_COMPRESSION_INFO_FLAGS = 2;
#[repr(C)]
#[cfg(feature = "bcrypt")]
#[derive(Clone, Copy, Debug, PartialEq)]
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
    pub LastCompressionStatus: super::bcrypt::NTSTATUS,
    pub Reserved: [u32; 8],
}
#[cfg(feature = "bcrypt")]
impl Default for REFS_QUERY_VOLUME_COMPRESSION_INFO_OUTPUT_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const REFS_QUERY_VOLUME_COMPRESSION_INFO_OUTPUT_BUFFER_VERSION: u32 = 1;
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct REFS_QUERY_VOLUME_IO_METRICS_INFO_INPUT_BUFFER_0_0 {
    pub Reserved: [u32; 6],
}
impl Default for REFS_QUERY_VOLUME_IO_METRICS_INFO_INPUT_BUFFER_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct REFS_QUERY_VOLUME_IO_METRICS_INFO_INPUT_BUFFER_0_1 {
    pub ResumeKeyBlob: [u64; 2],
    pub Reserved: [u32; 6],
}
impl Default for REFS_QUERY_VOLUME_IO_METRICS_INFO_INPUT_BUFFER_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const REFS_QUERY_VOLUME_IO_METRICS_INFO_INPUT_BUFFER_VERSION: u32 = 1;
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub const REFS_QUERY_VOLUME_IO_METRICS_INFO_OUTPUT_BUFFER_VERSION: u32 = 1;
pub type REFS_QUERY_VOLUME_IO_METRICS_INFO_QUERY_TYPE = i32;
pub const REFS_QUERY_VOLUME_IO_METRICS_INFO_QUERY_TYPE_METRICS_DATA: REFS_QUERY_VOLUME_IO_METRICS_INFO_QUERY_TYPE = 2;
pub const REFS_QUERY_VOLUME_IO_METRICS_INFO_QUERY_TYPE_PARAMETERS: REFS_QUERY_VOLUME_IO_METRICS_INFO_QUERY_TYPE = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct REFS_QUERY_VOLUME_IO_METRICS_METRICS_DATA {
    pub PlaceHolder: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct REFS_QUERY_VOLUME_TOTAL_SHARED_LCNS_OUTPUT_BUFFER {
    pub Version: u32,
    pub TotalSharedLcns: u64,
}
pub const REFS_QUERY_VOLUME_TOTAL_SHARED_LCNS_OUTPUT_BUFFER_VERSION: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct REFS_SET_ROLLBACK_PROTECTION_INFO_INPUT_BUFFER {
    pub Version: u32,
    pub FailMountOnMismatch: bool,
    pub CustomPayloadLength: u32,
    pub CustomPayloadOffset: u32,
    pub EnableRollbackProtection: bool,
}
pub const REFS_SET_ROLLBACK_PROTECTION_INFO_INPUT_BUFFER_VERSION: u32 = 2;
pub const REFS_SET_ROLLBACK_PROTECTION_INFO_INPUT_BUFFER_VERSION_V1: u32 = 1;
pub type REFS_SET_VOLUME_COMPRESSION_INFO_FLAGS = i32;
pub const REFS_SET_VOLUME_COMPRESSION_INFO_FLAG_DISABLE_COMPRESSION: REFS_SET_VOLUME_COMPRESSION_INFO_FLAGS = 16;
pub const REFS_SET_VOLUME_COMPRESSION_INFO_FLAG_ENABLE_COMPRESSION: REFS_SET_VOLUME_COMPRESSION_INFO_FLAGS = 8;
pub const REFS_SET_VOLUME_COMPRESSION_INFO_FLAG_GC_ONLY: REFS_SET_VOLUME_COMPRESSION_INFO_FLAGS = 4;
pub const REFS_SET_VOLUME_COMPRESSION_INFO_FLAG_MAX: REFS_SET_VOLUME_COMPRESSION_INFO_FLAGS = 16;
pub const REFS_SET_VOLUME_COMPRESSION_INFO_FLAG_START_COMPRESSION: REFS_SET_VOLUME_COMPRESSION_INFO_FLAGS = 1;
pub const REFS_SET_VOLUME_COMPRESSION_INFO_FLAG_STOP_COMPRESSION: REFS_SET_VOLUME_COMPRESSION_INFO_FLAGS = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub const REFS_SET_VOLUME_COMPRESSION_INFO_INPUT_BUFFER_VERSION: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub const REFS_SET_VOLUME_IO_METRICS_INFO_INPUT_BUFFER_VERSION: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct REFS_STREAM_EXTENT {
    pub Vcn: i64,
    pub Lcn: i64,
    pub Length: i64,
    pub Properties: REFS_STREAM_EXTENT_PROPERTIES,
}
pub type REFS_STREAM_EXTENT_PROPERTIES = u16;
pub const REFS_STREAM_EXTENT_PROPERTY_CRC32: REFS_STREAM_EXTENT_PROPERTIES = 128;
pub const REFS_STREAM_EXTENT_PROPERTY_CRC64: REFS_STREAM_EXTENT_PROPERTIES = 256;
pub const REFS_STREAM_EXTENT_PROPERTY_GHOSTED: REFS_STREAM_EXTENT_PROPERTIES = 512;
pub const REFS_STREAM_EXTENT_PROPERTY_READONLY: REFS_STREAM_EXTENT_PROPERTIES = 1024;
pub const REFS_STREAM_EXTENT_PROPERTY_SPARSE: REFS_STREAM_EXTENT_PROPERTIES = 8;
pub const REFS_STREAM_EXTENT_PROPERTY_STREAM_RESERVED: REFS_STREAM_EXTENT_PROPERTIES = 32;
pub const REFS_STREAM_EXTENT_PROPERTY_VALID: REFS_STREAM_EXTENT_PROPERTIES = 16;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct REFS_STREAM_SNAPSHOT_QUERY_DELTAS_INPUT_BUFFER {
    pub StartingVcn: i64,
    pub Flags: u32,
    pub Reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct REFS_VOLUME_COUNTER_INFO_INPUT_BUFFER {
    pub ResetCounters: bool,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct REFS_VOLUME_DEDUP_INFO_INPUT_BUFFER {
    pub Version: u32,
    pub SetDedupState: bool,
    pub Enable: bool,
    pub SetWeakRefState: bool,
    pub EnableWeakRef: bool,
    pub SetDirtyRangeTrackingState: bool,
    pub EnableDirtyRangeTracking: bool,
    pub SetWeakRefInconsistentState: bool,
    pub SetWeakRefInconsistent: bool,
}
pub const REFS_VOLUME_DEDUP_INFO_INPUT_BUFFER_VERSION: u32 = 2;
pub const REFS_VOLUME_DEDUP_INFO_INPUT_BUFFER_VERSION_V1: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct REFS_VOLUME_DEDUP_INFO_OUTPUT_BUFFER {
    pub Version: u32,
    pub Enabled: bool,
    pub EnabledWeakRef: bool,
    pub EnabledDirtyRangeTracking: bool,
    pub WeakRefInconsistent: bool,
    pub IsClustered: bool,
    pub VolumeIdHash: u32,
    pub VolumeGuid: windows_core::GUID,
    pub VolumeUniqueGuid: windows_core::GUID,
}
pub const REFS_VOLUME_DEDUP_INFO_OUTPUT_BUFFER_VERSION: u32 = 2;
pub const REFS_VOLUME_DEDUP_INFO_OUTPUT_BUFFER_VERSION_V1: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub const REMOTE_PROTOCOL_FLAG_INTEGRITY: u32 = 16;
pub const REMOTE_PROTOCOL_FLAG_LOOPBACK: u32 = 1;
pub const REMOTE_PROTOCOL_FLAG_MUTUAL_AUTH: u32 = 32;
pub const REMOTE_PROTOCOL_FLAG_OFFLINE: u32 = 2;
pub const REMOTE_PROTOCOL_FLAG_PERSISTENT_HANDLE: u32 = 4;
pub const REMOTE_PROTOCOL_FLAG_PRIVACY: u32 = 8;
pub const REMOVED_8DOT3_NAME: u32 = 2;
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
    pub ReparseGuidDataBuffer: super::winnt::REPARSE_GUID_DATA_BUFFER,
}
#[cfg(feature = "winnt")]
impl Default for REPARSE_DATA_BUFFER_EX_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const REPARSE_DATA_BUFFER_EX_HEADER_SIZE: u32 = 40;
pub const REPARSE_DATA_BUFFER_HEADER_SIZE: u32 = 8;
pub const REPARSE_DATA_EX_FLAG_GIVEN_TAG_OR_NONE: u32 = 1;
pub const REPARSE_GUID_DATA_BUFFER_EX_HEADER_SIZE: u32 = 56;
#[repr(C, packed(4))]
#[derive(Clone, Copy, Default)]
pub struct REPARSE_INDEX_KEY {
    pub FileReparseTag: u32,
    pub FileId: i64,
}
pub const RETURN_NON_NT_USER_SESSION_KEY: u32 = 8;
pub const RETURN_PRIMARY_LOGON_DOMAINNAME: u32 = 4;
pub const RETURN_PRIMARY_USERNAME: u32 = 2;
pub const RETURN_RESERVED_PARAMETER: u32 = 128;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RKF_BYPASS_ECP_CONTEXT {
    pub Reserved: i32,
    pub Version: i32,
}
pub const RPI_SMB2_SERVERCAP_DFS: u32 = 1;
pub const RPI_SMB2_SERVERCAP_DIRECTORY_LEASING: u32 = 32;
pub const RPI_SMB2_SERVERCAP_ENCRYPTION_AWARE: u32 = 64;
pub const RPI_SMB2_SERVERCAP_LARGEMTU: u32 = 4;
pub const RPI_SMB2_SERVERCAP_LEASING: u32 = 2;
pub const RPI_SMB2_SERVERCAP_MULTICHANNEL: u32 = 8;
pub const RPI_SMB2_SERVERCAP_NOTIFICATIONS_AWARE: u32 = 128;
pub const RPI_SMB2_SERVERCAP_PERSISTENT_HANDLES: u32 = 16;
pub const RPI_SMB2_SHARECAP_ACCESS_BASED_DIRECTORY_ENUM: u32 = 256;
pub const RPI_SMB2_SHARECAP_ASYMMETRIC_SCALEOUT: u32 = 1024;
pub const RPI_SMB2_SHARECAP_CLUSTER: u32 = 64;
pub const RPI_SMB2_SHARECAP_CONTINUOUS_AVAILABILITY: u32 = 16;
pub const RPI_SMB2_SHARECAP_DFS: u32 = 8;
pub const RPI_SMB2_SHARECAP_ENCRYPTED: u32 = 128;
pub const RPI_SMB2_SHARECAP_IDENTITY_REMOTING: u32 = 512;
pub const RPI_SMB2_SHARECAP_SCALEOUT: u32 = 32;
pub const RPI_SMB2_SHARECAP_TIMEWARP: u32 = 2;
pub const RPI_SMB2_SHARETYPE_DISK: u32 = 0;
pub const RPI_SMB2_SHARETYPE_PIPE: u32 = 1;
pub const RPI_SMB2_SHARETYPE_PRINT: u32 = 2;
pub type RTL_ALLOCATE_STRING_ROUTINE = Option<unsafe extern "system" fn(numberofbytes: usize) -> *mut core::ffi::c_void>;
pub const RTL_DUPLICATE_UNICODE_STRING_ALLOCATE_NULL_STRING: u32 = 2;
pub const RTL_DUPLICATE_UNICODE_STRING_NULL_TERMINATE: u32 = 1;
pub type RTL_FREE_STRING_ROUTINE = Option<unsafe extern "system" fn(buffer: *const core::ffi::c_void)>;
#[cfg(feature = "bcrypt")]
pub type RTL_HEAP_COMMIT_ROUTINE = Option<unsafe extern "system" fn(base: *const core::ffi::c_void, commitaddress: *mut *mut core::ffi::c_void, commitsize: *mut usize) -> super::bcrypt::NTSTATUS>;
pub const RTL_HEAP_MEMORY_LIMIT_CURRENT_VERSION: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RTL_HEAP_MEMORY_LIMIT_DATA {
    pub CommitLimitBytes: usize,
    pub CommitLimitFailureCode: usize,
    pub MaxAllocationSizeBytes: usize,
    pub AllocationLimitFailureCode: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RTL_HEAP_MEMORY_LIMIT_INFO {
    pub Version: u32,
    pub Data: RTL_HEAP_MEMORY_LIMIT_DATA,
}
#[repr(C)]
#[cfg(feature = "bcrypt")]
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
#[cfg(feature = "bcrypt")]
impl Default for RTL_HEAP_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type RTL_MEMORY_TYPE = i32;
pub type RTL_REALLOCATE_STRING_ROUTINE = Option<unsafe extern "system" fn(numberofbytes: usize, buffer: *const core::ffi::c_void) -> *mut core::ffi::c_void>;
pub const RTL_SEGHEAP_MEM_SOURCE_ANY_NODE: u32 = 4294967295;
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct RTL_SEGMENT_HEAP_MEMORY_SOURCE {
    pub Flags: u32,
    pub MemoryTypeMask: u32,
    pub NumaNode: u32,
    pub Anonymous: RTL_SEGMENT_HEAP_MEMORY_SOURCE_0,
    pub Reserved: [usize; 2],
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
impl Default for RTL_SEGMENT_HEAP_MEMORY_SOURCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union RTL_SEGMENT_HEAP_MEMORY_SOURCE_0 {
    pub PartitionHandle: super::winnt::HANDLE,
    pub Callbacks: *mut RTL_SEGMENT_HEAP_VA_CALLBACKS,
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
impl Default for RTL_SEGMENT_HEAP_MEMORY_SOURCE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct RTL_SEGMENT_HEAP_PARAMETERS {
    pub Version: u16,
    pub Size: u16,
    pub Flags: u32,
    pub MemorySource: RTL_SEGMENT_HEAP_MEMORY_SOURCE,
    pub Reserved: [usize; 4],
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
impl Default for RTL_SEGMENT_HEAP_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RTL_SEGMENT_HEAP_VA_CALLBACKS {
    pub CallbackContext: super::winnt::HANDLE,
    pub AllocateVirtualMemory: PALLOCATE_VIRTUAL_MEMORY_EX_CALLBACK,
    pub FreeVirtualMemory: PFREE_VIRTUAL_MEMORY_EX_CALLBACK,
    pub QueryVirtualMemory: PQUERY_VIRTUAL_MEMORY_CALLBACK,
}
pub const RTL_SYSTEM_VOLUME_INFORMATION_FOLDER: windows_core::PCWSTR = windows_core::w!("System Volume Information");
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SECURITY_CLIENT_CONTEXT {
    pub SecurityQos: super::winnt::SECURITY_QUALITY_OF_SERVICE,
    pub ClientToken: super::winnt::PACCESS_TOKEN,
    pub DirectlyAccessClientToken: bool,
    pub DirectAccessEffectiveOnly: bool,
    pub ServerIsRemote: bool,
    pub ClientTokenControl: super::winnt::TOKEN_CONTROL,
}
pub const SECURITY_DESCRIPTOR_DO_NOT_FREE: u32 = 67108864;
pub const SEGMENT_HEAP_FLG_NO_LFH: u32 = 2;
pub const SEGMENT_HEAP_FLG_USE_PAGE_HEAP: u32 = 1;
pub const SEGMENT_HEAP_PARAMETERS_VERSION: u32 = 3;
pub const SEGMENT_HEAP_PARAMS_VALID_FLAGS: u32 = 3;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SET_CACHED_RUNS_STATE_INPUT_BUFFER {
    pub Enable: bool,
}
#[repr(C)]
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SE_AUDIT_INFO {
    pub Size: u32,
    pub AuditType: super::winnt::AUDIT_EVENT_TYPE,
    pub AuditOperation: SE_AUDIT_OPERATION,
    pub AuditFlags: u32,
    pub SubsystemName: super::ntsecapi::UNICODE_STRING,
    pub ObjectTypeName: super::ntsecapi::UNICODE_STRING,
    pub ObjectName: super::ntsecapi::UNICODE_STRING,
    pub HandleId: *mut core::ffi::c_void,
    pub TransactionId: *mut windows_core::GUID,
    pub OperationId: *mut super::winnt::LUID,
    pub ObjectCreation: bool,
    pub GenerateOnClose: bool,
}
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
impl Default for SE_AUDIT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type SE_AUDIT_OPERATION = i32;
pub const SE_BACKUP_PRIVILEGES_CHECKED: u32 = 256;
pub const SE_DACL_UNTRUSTED: u32 = 64;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SE_EXPORTS {
    pub SeCreateTokenPrivilege: super::winnt::LUID,
    pub SeAssignPrimaryTokenPrivilege: super::winnt::LUID,
    pub SeLockMemoryPrivilege: super::winnt::LUID,
    pub SeIncreaseQuotaPrivilege: super::winnt::LUID,
    pub SeUnsolicitedInputPrivilege: super::winnt::LUID,
    pub SeTcbPrivilege: super::winnt::LUID,
    pub SeSecurityPrivilege: super::winnt::LUID,
    pub SeTakeOwnershipPrivilege: super::winnt::LUID,
    pub SeLoadDriverPrivilege: super::winnt::LUID,
    pub SeCreatePagefilePrivilege: super::winnt::LUID,
    pub SeIncreaseBasePriorityPrivilege: super::winnt::LUID,
    pub SeSystemProfilePrivilege: super::winnt::LUID,
    pub SeSystemtimePrivilege: super::winnt::LUID,
    pub SeProfileSingleProcessPrivilege: super::winnt::LUID,
    pub SeCreatePermanentPrivilege: super::winnt::LUID,
    pub SeBackupPrivilege: super::winnt::LUID,
    pub SeRestorePrivilege: super::winnt::LUID,
    pub SeShutdownPrivilege: super::winnt::LUID,
    pub SeDebugPrivilege: super::winnt::LUID,
    pub SeAuditPrivilege: super::winnt::LUID,
    pub SeSystemEnvironmentPrivilege: super::winnt::LUID,
    pub SeChangeNotifyPrivilege: super::winnt::LUID,
    pub SeRemoteShutdownPrivilege: super::winnt::LUID,
    pub SeNullSid: super::winnt::PSID,
    pub SeWorldSid: super::winnt::PSID,
    pub SeLocalSid: super::winnt::PSID,
    pub SeCreatorOwnerSid: super::winnt::PSID,
    pub SeCreatorGroupSid: super::winnt::PSID,
    pub SeNtAuthoritySid: super::winnt::PSID,
    pub SeDialupSid: super::winnt::PSID,
    pub SeNetworkSid: super::winnt::PSID,
    pub SeBatchSid: super::winnt::PSID,
    pub SeInteractiveSid: super::winnt::PSID,
    pub SeLocalSystemSid: super::winnt::PSID,
    pub SeAliasAdminsSid: super::winnt::PSID,
    pub SeAliasUsersSid: super::winnt::PSID,
    pub SeAliasGuestsSid: super::winnt::PSID,
    pub SeAliasPowerUsersSid: super::winnt::PSID,
    pub SeAliasAccountOpsSid: super::winnt::PSID,
    pub SeAliasSystemOpsSid: super::winnt::PSID,
    pub SeAliasPrintOpsSid: super::winnt::PSID,
    pub SeAliasBackupOpsSid: super::winnt::PSID,
    pub SeAuthenticatedUsersSid: super::winnt::PSID,
    pub SeRestrictedSid: super::winnt::PSID,
    pub SeAnonymousLogonSid: super::winnt::PSID,
    pub SeUndockPrivilege: super::winnt::LUID,
    pub SeSyncAgentPrivilege: super::winnt::LUID,
    pub SeEnableDelegationPrivilege: super::winnt::LUID,
    pub SeLocalServiceSid: super::winnt::PSID,
    pub SeNetworkServiceSid: super::winnt::PSID,
    pub SeManageVolumePrivilege: super::winnt::LUID,
    pub SeImpersonatePrivilege: super::winnt::LUID,
    pub SeCreateGlobalPrivilege: super::winnt::LUID,
    pub SeTrustedCredManAccessPrivilege: super::winnt::LUID,
    pub SeRelabelPrivilege: super::winnt::LUID,
    pub SeIncreaseWorkingSetPrivilege: super::winnt::LUID,
    pub SeTimeZonePrivilege: super::winnt::LUID,
    pub SeCreateSymbolicLinkPrivilege: super::winnt::LUID,
    pub SeIUserSid: super::winnt::PSID,
    pub SeUntrustedMandatorySid: super::winnt::PSID,
    pub SeLowMandatorySid: super::winnt::PSID,
    pub SeMediumMandatorySid: super::winnt::PSID,
    pub SeHighMandatorySid: super::winnt::PSID,
    pub SeSystemMandatorySid: super::winnt::PSID,
    pub SeOwnerRightsSid: super::winnt::PSID,
    pub SeAllAppPackagesSid: super::winnt::PSID,
    pub SeUserModeDriversSid: super::winnt::PSID,
    pub SeProcTrustWinTcbSid: super::winnt::PSID,
    pub SeTrustedInstallerSid: super::winnt::PSID,
    pub SeDelegateSessionUserImpersonatePrivilege: super::winnt::LUID,
    pub SeAppSiloSid: super::winnt::PSID,
    pub SeAppSiloVolumeRootMinimalCapabilitySid: super::winnt::PSID,
    pub SeAppSiloProfilesRootMinimalCapabilitySid: super::winnt::PSID,
    pub SeAppSiloPromptForAccessCapabilitySid: super::winnt::PSID,
    pub SeAppSiloAccessToPublisherDirectoryCapabilitySid: super::winnt::PSID,
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
pub type SE_LOGON_SESSION_TERMINATED_ROUTINE = Option<unsafe extern "system" fn(logonid: *const super::winnt::LUID) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "ntddk", feature = "winnt"))]
pub type SE_LOGON_SESSION_TERMINATED_ROUTINE_EX = Option<unsafe extern "system" fn(logonid: *const super::winnt::LUID, pserversilo: *const super::ntddk::_EJOB, context: *const core::ffi::c_void) -> super::bcrypt::NTSTATUS>;
pub const SE_SERVER_SECURITY: u32 = 128;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SOV_RANGE_CHECK_DATA {
    pub RemoveZone: bool,
    pub InRange: [u64; 2],
    pub ZidForRemoval: windows_core::GUID,
    pub Reserved1: u64,
    pub Reserved2: u64,
}
impl Default for SOV_RANGE_CHECK_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SPECIAL_ENCRYPTED_OPEN: u32 = 262144;
pub type SRV_INSTANCE_TYPE = i32;
#[repr(C)]
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SRV_OPEN_ECP_CONTEXT {
    pub ShareName: super::ntsecapi::PUNICODE_STRING,
    pub SocketAddress: PSOCKADDR_STORAGE_NFS,
    pub OplockBlockState: bool,
    pub OplockAppState: bool,
    pub OplockFinalState: bool,
    pub Version: u16,
    pub InstanceType: SRV_INSTANCE_TYPE,
}
pub const SRV_OPEN_ECP_CONTEXT_VERSION_2: u32 = 2;
pub const SUPPORTED_FS_FEATURES_BYPASS_IO: u32 = 8;
pub const SUPPORTED_FS_FEATURES_OFFLOAD_READ: u32 = 1;
pub const SUPPORTED_FS_FEATURES_OFFLOAD_WRITE: u32 = 2;
pub const SUPPORTED_FS_FEATURES_QUERY_OPEN: u32 = 4;
pub const SUPPORTED_FS_FEATURES_VALID_MASK: u32 = 15;
pub const SYMLINK_DIRECTORY: u32 = 2147483648;
pub const SYMLINK_FILE: u32 = 1073741824;
pub const SYMLINK_FLAG_RELATIVE: u32 = 1;
pub const SYMLINK_RESERVED_MASK: u32 = 4026531840;
pub const SYSTEM_PAGE_PRIORITY_BITS: u32 = 3;
pub const SYSTEM_PAGE_PRIORITY_LEVELS: u32 = 8;
pub const SrvInstanceTypeCsv: SRV_INSTANCE_TYPE = 2;
pub const SrvInstanceTypePrimary: SRV_INSTANCE_TYPE = 1;
pub const SrvInstanceTypeSBL: SRV_INSTANCE_TYPE = 3;
pub const SrvInstanceTypeSOV: SRV_INSTANCE_TYPE = 6;
pub const SrvInstanceTypeSR: SRV_INSTANCE_TYPE = 4;
pub const SrvInstanceTypeUndefined: SRV_INSTANCE_TYPE = 0;
pub const SrvInstanceTypeVMLM: SRV_INSTANCE_TYPE = 7;
pub const SrvInstanceTypeVSMB: SRV_INSTANCE_TYPE = 5;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SspiAsyncContext(pub u8);
pub type SspiAsyncNotifyCallback = Option<unsafe extern "system" fn(handle: *const SspiAsyncContext, callbackdata: *const core::ffi::c_void)>;
pub const SyncTypeCreateSection: FS_FILTER_SECTION_SYNC_TYPE = 1;
pub const SyncTypeOther: FS_FILTER_SECTION_SYNC_TYPE = 0;
pub const TOKEN_AUDIT_NO_CHILD_PROCESS: u32 = 2097152;
pub const TOKEN_AUDIT_REDIRECTION_TRUST: u32 = 8388608;
pub const TOKEN_DO_NOT_USE_GLOBAL_ATTRIBS_FOR_QUERY: u32 = 131072;
pub const TOKEN_ENFORCE_REDIRECTION_TRUST: u32 = 4194304;
pub const TOKEN_HAS_BACKUP_PRIVILEGE: u32 = 2;
pub const TOKEN_HAS_IMPERSONATE_PRIVILEGE: u32 = 128;
pub const TOKEN_HAS_OWN_CLAIM_ATTRIBUTES: u32 = 32768;
pub const TOKEN_HAS_RESTORE_PRIVILEGE: u32 = 4;
pub const TOKEN_HAS_TRAVERSE_PRIVILEGE: u32 = 1;
pub const TOKEN_INHERIT_SECURITY_FLAGS: u32 = 3670016;
pub const TOKEN_IS_FILTERED: u32 = 2048;
pub const TOKEN_IS_RESTRICTED: u32 = 16;
pub const TOKEN_LEARNING_MODE_LOGGING: u32 = 16777216;
pub const TOKEN_LOWBOX: u32 = 16384;
pub const TOKEN_NOT_LOW: u32 = 8192;
pub const TOKEN_NO_CHILD_PROCESS: u32 = 524288;
pub const TOKEN_NO_CHILD_PROCESS_UNLESS_SECURE: u32 = 1048576;
pub const TOKEN_PERMISSIVE_LEARNING_MODE: u32 = 50331648;
pub const TOKEN_PRIVATE_NAMESPACE: u32 = 65536;
pub const TOKEN_REF_SYSTEM_MANAGED_ADMIN_FULL_TOKEN: u32 = 268435456;
pub const TOKEN_SANDBOX_INERT: u32 = 64;
pub const TOKEN_SESSION_NOT_REFERENCED: u32 = 32;
pub const TOKEN_SYSTEM_MANAGED_ADMIN_FULL_TOKEN: u32 = 134217728;
pub const TOKEN_UIACCESS: u32 = 4096;
pub const TOKEN_VIRTUALIZE_ALLOWED: u32 = 512;
pub const TOKEN_VIRTUALIZE_ENABLED: u32 = 1024;
pub const TOKEN_WRITE_RESTRICTED: u32 = 8;
#[repr(C)]
#[cfg(all(feature = "ntddk", feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct TUNNEL {
    pub Mutex: super::wdm::FAST_MUTEX,
    pub Cache: super::ntddk::PRTL_SPLAY_LINKS,
    pub TimerQueue: super::winnt::LIST_ENTRY,
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UNICODE_PREFIX_TABLE {
    pub NodeTypeCode: super::ntdef::CSHORT,
    pub NameLength: super::ntdef::CSHORT,
    pub NextPrefixTree: PUNICODE_PREFIX_TABLE_ENTRY,
    pub LastNextEntry: PUNICODE_PREFIX_TABLE_ENTRY,
}
#[repr(C)]
#[cfg(all(feature = "lsalookup", feature = "ntddk", feature = "ntdef", feature = "ntsecapi"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UNICODE_PREFIX_TABLE_ENTRY {
    pub NodeTypeCode: super::ntdef::CSHORT,
    pub NameLength: super::ntdef::CSHORT,
    pub NextPrefixTree: *mut Self,
    pub CaseMatch: *mut Self,
    pub Links: super::ntddk::RTL_SPLAY_LINKS,
    pub Prefix: super::ntsecapi::PUNICODE_STRING,
}
#[cfg(all(feature = "lsalookup", feature = "ntddk", feature = "ntdef", feature = "ntsecapi"))]
impl Default for UNICODE_PREFIX_TABLE_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const UNINITIALIZE_CACHE_MAPS: u32 = 1;
pub const USE_PRIMARY_PASSWORD: u32 = 1;
pub const VACB_MAPPING_GRANULARITY: u32 = 262144;
pub const VACB_OFFSET_SHIFT: u32 = 18;
pub const VALID_COPY_FILE_CHUNK_FLAGS: u32 = 1;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct VBN(pub u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct VCN_RANGE_INPUT_BUFFER {
    pub StartingVcn: i64,
    pub ClusterCount: i64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct VETO_BINDING_ECP_CONTEXT {
    pub ShouldVetoBinding: bool,
}
pub type VIRTUAL_MEMORY_INFORMATION_CLASS = i32;
pub const VOLSNAPCONTROLTYPE: u32 = 83;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct VOLUME_REFS_INFO_BUFFER {
    pub CacheSizeInBytes: i64,
    pub AllocatedCacheInBytes: i64,
    pub PopulatedCacheInBytes: i64,
    pub InErrorCacheInBytes: i64,
    pub MemoryUsedForCacheMetadata: i64,
    pub CacheLineSize: u32,
    pub CacheTransactionsOutstanding: i32,
    pub CacheLinesFree: i32,
    pub CacheLinesInError: i32,
    pub CacheHitsInBytes: i64,
    pub CacheMissesInBytes: i64,
    pub CachePopulationUpdatesInBytes: i64,
    pub CacheWriteThroughUpdatesInBytes: i64,
    pub CacheInvalidationsInBytes: i64,
    pub CacheOverReadsInBytes: i64,
    pub MetadataWrittenBytes: i64,
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
    pub DataInPlaceWriteClusterCount: i64,
    pub CompactionFailedDueToIneligibleContainer: i32,
    pub CompactionFailedDueToMaxFragmentation: i32,
    pub CompactedContainerFillRatio: i64,
    pub CompactedContainerFillRatioBase: i32,
    pub ContainerMoveRetryCount: i32,
    pub ContainerMoveFailedDueToIneligibleContainer: i32,
    pub CompactionFailureCount: i32,
    pub ContainerMoveFailureCount: i32,
    pub NumberOfDirtyMetadataPages: i64,
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
pub const VmPrefetchInformation: VIRTUAL_MEMORY_INFORMATION_CLASS = 0;
pub const WCIFS_REDIRECTION_FLAGS_CREATE_SERVICED_FROM_LAYER: u32 = 1;
pub const WCIFS_REDIRECTION_FLAGS_CREATE_SERVICED_FROM_REGISTERED_LAYER: u32 = 4;
pub const WCIFS_REDIRECTION_FLAGS_CREATE_SERVICED_FROM_REMOTE_LAYER: u32 = 8;
pub const WCIFS_REDIRECTION_FLAGS_CREATE_SERVICED_FROM_SCRATCH: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _REAL_NOTIFY_SYNC(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct sockaddr_storage(pub u8);
