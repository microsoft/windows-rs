#[cfg(feature = "Win32_Security_AppLocker")]
pub mod AppLocker;
#[cfg(feature = "Win32_Security_Authentication")]
pub mod Authentication;
#[cfg(feature = "Win32_Security_Authorization")]
pub mod Authorization;
#[cfg(feature = "Win32_Security_ConfigurationSnapin")]
pub mod ConfigurationSnapin;
#[cfg(feature = "Win32_Security_Credentials")]
pub mod Credentials;
#[cfg(feature = "Win32_Security_Cryptography")]
pub mod Cryptography;
#[cfg(feature = "Win32_Security_DiagnosticDataQuery")]
pub mod DiagnosticDataQuery;
#[cfg(feature = "Win32_Security_DirectoryServices")]
pub mod DirectoryServices;
#[cfg(feature = "Win32_Security_EnterpriseData")]
pub mod EnterpriseData;
#[cfg(feature = "Win32_Security_ExtensibleAuthenticationProtocol")]
pub mod ExtensibleAuthenticationProtocol;
#[cfg(feature = "Win32_Security_Isolation")]
pub mod Isolation;
#[cfg(feature = "Win32_Security_LicenseProtection")]
pub mod LicenseProtection;
#[cfg(feature = "Win32_Security_NetworkAccessProtection")]
pub mod NetworkAccessProtection;
#[cfg(feature = "Win32_Security_Tpm")]
pub mod Tpm;
#[cfg(feature = "Win32_Security_WinTrust")]
pub mod WinTrust;
#[cfg(feature = "Win32_Security_WinWlx")]
pub mod WinWlx;
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AccessCheck<P0, P1>(psecuritydescriptor: P0, clienttoken: P1, desiredaccess: u32, genericmapping: *const GENERIC_MAPPING, privilegeset: ::core::option::Option<*mut PRIVILEGE_SET>, privilegesetlength: *mut u32, grantedaccess: *mut u32, accessstatus: *mut i32) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<PSECURITY_DESCRIPTOR>,
    P1: ::windows::core::IntoParam<super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn AccessCheck ( psecuritydescriptor : PSECURITY_DESCRIPTOR , clienttoken : super::Foundation:: HANDLE , desiredaccess : u32 , genericmapping : *const GENERIC_MAPPING , privilegeset : *mut PRIVILEGE_SET , privilegesetlength : *mut u32 , grantedaccess : *mut u32 , accessstatus : *mut i32 ) -> super::Foundation:: BOOL );
    AccessCheck(psecuritydescriptor.into_param().abi(), clienttoken.into_param().abi(), desiredaccess, genericmapping, ::core::mem::transmute(privilegeset.unwrap_or(::std::ptr::null_mut())), privilegesetlength, grantedaccess, accessstatus)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AccessCheckAndAuditAlarmA<P0, P1, P2, P3, P4>(subsystemname: P0, handleid: ::core::option::Option<*const ::core::ffi::c_void>, objecttypename: P1, objectname: P2, securitydescriptor: P3, desiredaccess: u32, genericmapping: *const GENERIC_MAPPING, objectcreation: P4, grantedaccess: *mut u32, accessstatus: *mut i32, pfgenerateonclose: *mut i32) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P3: ::windows::core::IntoParam<PSECURITY_DESCRIPTOR>,
    P4: ::windows::core::IntoParam<super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn AccessCheckAndAuditAlarmA ( subsystemname : :: windows::core::PCSTR , handleid : *const ::core::ffi::c_void , objecttypename : :: windows::core::PCSTR , objectname : :: windows::core::PCSTR , securitydescriptor : PSECURITY_DESCRIPTOR , desiredaccess : u32 , genericmapping : *const GENERIC_MAPPING , objectcreation : super::Foundation:: BOOL , grantedaccess : *mut u32 , accessstatus : *mut i32 , pfgenerateonclose : *mut i32 ) -> super::Foundation:: BOOL );
    AccessCheckAndAuditAlarmA(subsystemname.into_param().abi(), ::core::mem::transmute(handleid.unwrap_or(::std::ptr::null())), objecttypename.into_param().abi(), objectname.into_param().abi(), securitydescriptor.into_param().abi(), desiredaccess, genericmapping, objectcreation.into_param().abi(), grantedaccess, accessstatus, pfgenerateonclose)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AccessCheckAndAuditAlarmW<P0, P1, P2, P3, P4>(subsystemname: P0, handleid: ::core::option::Option<*const ::core::ffi::c_void>, objecttypename: P1, objectname: P2, securitydescriptor: P3, desiredaccess: u32, genericmapping: *const GENERIC_MAPPING, objectcreation: P4, grantedaccess: *mut u32, accessstatus: *mut i32, pfgenerateonclose: *mut i32) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P3: ::windows::core::IntoParam<PSECURITY_DESCRIPTOR>,
    P4: ::windows::core::IntoParam<super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn AccessCheckAndAuditAlarmW ( subsystemname : :: windows::core::PCWSTR , handleid : *const ::core::ffi::c_void , objecttypename : :: windows::core::PCWSTR , objectname : :: windows::core::PCWSTR , securitydescriptor : PSECURITY_DESCRIPTOR , desiredaccess : u32 , genericmapping : *const GENERIC_MAPPING , objectcreation : super::Foundation:: BOOL , grantedaccess : *mut u32 , accessstatus : *mut i32 , pfgenerateonclose : *mut i32 ) -> super::Foundation:: BOOL );
    AccessCheckAndAuditAlarmW(subsystemname.into_param().abi(), ::core::mem::transmute(handleid.unwrap_or(::std::ptr::null())), objecttypename.into_param().abi(), objectname.into_param().abi(), securitydescriptor.into_param().abi(), desiredaccess, genericmapping, objectcreation.into_param().abi(), grantedaccess, accessstatus, pfgenerateonclose)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AccessCheckByType<P0, P1, P2>(psecuritydescriptor: P0, principalselfsid: P1, clienttoken: P2, desiredaccess: u32, objecttypelist: ::core::option::Option<&mut [OBJECT_TYPE_LIST]>, genericmapping: *const GENERIC_MAPPING, privilegeset: ::core::option::Option<*mut PRIVILEGE_SET>, privilegesetlength: *mut u32, grantedaccess: *mut u32, accessstatus: *mut i32) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<PSECURITY_DESCRIPTOR>,
    P1: ::windows::core::IntoParam<super::Foundation::PSID>,
    P2: ::windows::core::IntoParam<super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn AccessCheckByType ( psecuritydescriptor : PSECURITY_DESCRIPTOR , principalselfsid : super::Foundation:: PSID , clienttoken : super::Foundation:: HANDLE , desiredaccess : u32 , objecttypelist : *mut OBJECT_TYPE_LIST , objecttypelistlength : u32 , genericmapping : *const GENERIC_MAPPING , privilegeset : *mut PRIVILEGE_SET , privilegesetlength : *mut u32 , grantedaccess : *mut u32 , accessstatus : *mut i32 ) -> super::Foundation:: BOOL );
    AccessCheckByType(psecuritydescriptor.into_param().abi(), principalselfsid.into_param().abi(), clienttoken.into_param().abi(), desiredaccess, ::core::mem::transmute(objecttypelist.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), objecttypelist.as_deref().map_or(0, |slice| slice.len() as _), genericmapping, ::core::mem::transmute(privilegeset.unwrap_or(::std::ptr::null_mut())), privilegesetlength, grantedaccess, accessstatus)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AccessCheckByTypeAndAuditAlarmA<P0, P1, P2, P3, P4, P5>(subsystemname: P0, handleid: *const ::core::ffi::c_void, objecttypename: P1, objectname: P2, securitydescriptor: P3, principalselfsid: P4, desiredaccess: u32, audittype: AUDIT_EVENT_TYPE, flags: u32, objecttypelist: ::core::option::Option<&mut [OBJECT_TYPE_LIST]>, genericmapping: *const GENERIC_MAPPING, objectcreation: P5, grantedaccess: *mut u32, accessstatus: *mut i32, pfgenerateonclose: *mut i32) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P3: ::windows::core::IntoParam<PSECURITY_DESCRIPTOR>,
    P4: ::windows::core::IntoParam<super::Foundation::PSID>,
    P5: ::windows::core::IntoParam<super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn AccessCheckByTypeAndAuditAlarmA ( subsystemname : :: windows::core::PCSTR , handleid : *const ::core::ffi::c_void , objecttypename : :: windows::core::PCSTR , objectname : :: windows::core::PCSTR , securitydescriptor : PSECURITY_DESCRIPTOR , principalselfsid : super::Foundation:: PSID , desiredaccess : u32 , audittype : AUDIT_EVENT_TYPE , flags : u32 , objecttypelist : *mut OBJECT_TYPE_LIST , objecttypelistlength : u32 , genericmapping : *const GENERIC_MAPPING , objectcreation : super::Foundation:: BOOL , grantedaccess : *mut u32 , accessstatus : *mut i32 , pfgenerateonclose : *mut i32 ) -> super::Foundation:: BOOL );
    AccessCheckByTypeAndAuditAlarmA(
        subsystemname.into_param().abi(),
        handleid,
        objecttypename.into_param().abi(),
        objectname.into_param().abi(),
        securitydescriptor.into_param().abi(),
        principalselfsid.into_param().abi(),
        desiredaccess,
        audittype,
        flags,
        ::core::mem::transmute(objecttypelist.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())),
        objecttypelist.as_deref().map_or(0, |slice| slice.len() as _),
        genericmapping,
        objectcreation.into_param().abi(),
        grantedaccess,
        accessstatus,
        pfgenerateonclose,
    )
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AccessCheckByTypeAndAuditAlarmW<P0, P1, P2, P3, P4, P5>(subsystemname: P0, handleid: *const ::core::ffi::c_void, objecttypename: P1, objectname: P2, securitydescriptor: P3, principalselfsid: P4, desiredaccess: u32, audittype: AUDIT_EVENT_TYPE, flags: u32, objecttypelist: ::core::option::Option<&mut [OBJECT_TYPE_LIST]>, genericmapping: *const GENERIC_MAPPING, objectcreation: P5, grantedaccess: *mut u32, accessstatus: *mut i32, pfgenerateonclose: *mut i32) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P3: ::windows::core::IntoParam<PSECURITY_DESCRIPTOR>,
    P4: ::windows::core::IntoParam<super::Foundation::PSID>,
    P5: ::windows::core::IntoParam<super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn AccessCheckByTypeAndAuditAlarmW ( subsystemname : :: windows::core::PCWSTR , handleid : *const ::core::ffi::c_void , objecttypename : :: windows::core::PCWSTR , objectname : :: windows::core::PCWSTR , securitydescriptor : PSECURITY_DESCRIPTOR , principalselfsid : super::Foundation:: PSID , desiredaccess : u32 , audittype : AUDIT_EVENT_TYPE , flags : u32 , objecttypelist : *mut OBJECT_TYPE_LIST , objecttypelistlength : u32 , genericmapping : *const GENERIC_MAPPING , objectcreation : super::Foundation:: BOOL , grantedaccess : *mut u32 , accessstatus : *mut i32 , pfgenerateonclose : *mut i32 ) -> super::Foundation:: BOOL );
    AccessCheckByTypeAndAuditAlarmW(
        subsystemname.into_param().abi(),
        handleid,
        objecttypename.into_param().abi(),
        objectname.into_param().abi(),
        securitydescriptor.into_param().abi(),
        principalselfsid.into_param().abi(),
        desiredaccess,
        audittype,
        flags,
        ::core::mem::transmute(objecttypelist.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())),
        objecttypelist.as_deref().map_or(0, |slice| slice.len() as _),
        genericmapping,
        objectcreation.into_param().abi(),
        grantedaccess,
        accessstatus,
        pfgenerateonclose,
    )
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AccessCheckByTypeResultList<P0, P1, P2>(psecuritydescriptor: P0, principalselfsid: P1, clienttoken: P2, desiredaccess: u32, objecttypelist: ::core::option::Option<*mut OBJECT_TYPE_LIST>, objecttypelistlength: u32, genericmapping: *const GENERIC_MAPPING, privilegeset: ::core::option::Option<*mut PRIVILEGE_SET>, privilegesetlength: *mut u32, grantedaccesslist: *mut u32, accessstatuslist: *mut u32) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<PSECURITY_DESCRIPTOR>,
    P1: ::windows::core::IntoParam<super::Foundation::PSID>,
    P2: ::windows::core::IntoParam<super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn AccessCheckByTypeResultList ( psecuritydescriptor : PSECURITY_DESCRIPTOR , principalselfsid : super::Foundation:: PSID , clienttoken : super::Foundation:: HANDLE , desiredaccess : u32 , objecttypelist : *mut OBJECT_TYPE_LIST , objecttypelistlength : u32 , genericmapping : *const GENERIC_MAPPING , privilegeset : *mut PRIVILEGE_SET , privilegesetlength : *mut u32 , grantedaccesslist : *mut u32 , accessstatuslist : *mut u32 ) -> super::Foundation:: BOOL );
    AccessCheckByTypeResultList(psecuritydescriptor.into_param().abi(), principalselfsid.into_param().abi(), clienttoken.into_param().abi(), desiredaccess, ::core::mem::transmute(objecttypelist.unwrap_or(::std::ptr::null_mut())), objecttypelistlength, genericmapping, ::core::mem::transmute(privilegeset.unwrap_or(::std::ptr::null_mut())), privilegesetlength, grantedaccesslist, accessstatuslist)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AccessCheckByTypeResultListAndAuditAlarmA<P0, P1, P2, P3, P4, P5>(subsystemname: P0, handleid: *const ::core::ffi::c_void, objecttypename: P1, objectname: P2, securitydescriptor: P3, principalselfsid: P4, desiredaccess: u32, audittype: AUDIT_EVENT_TYPE, flags: u32, objecttypelist: ::core::option::Option<*mut OBJECT_TYPE_LIST>, objecttypelistlength: u32, genericmapping: *const GENERIC_MAPPING, objectcreation: P5, grantedaccess: *mut u32, accessstatuslist: *mut u32, pfgenerateonclose: *mut i32) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P3: ::windows::core::IntoParam<PSECURITY_DESCRIPTOR>,
    P4: ::windows::core::IntoParam<super::Foundation::PSID>,
    P5: ::windows::core::IntoParam<super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn AccessCheckByTypeResultListAndAuditAlarmA ( subsystemname : :: windows::core::PCSTR , handleid : *const ::core::ffi::c_void , objecttypename : :: windows::core::PCSTR , objectname : :: windows::core::PCSTR , securitydescriptor : PSECURITY_DESCRIPTOR , principalselfsid : super::Foundation:: PSID , desiredaccess : u32 , audittype : AUDIT_EVENT_TYPE , flags : u32 , objecttypelist : *mut OBJECT_TYPE_LIST , objecttypelistlength : u32 , genericmapping : *const GENERIC_MAPPING , objectcreation : super::Foundation:: BOOL , grantedaccess : *mut u32 , accessstatuslist : *mut u32 , pfgenerateonclose : *mut i32 ) -> super::Foundation:: BOOL );
    AccessCheckByTypeResultListAndAuditAlarmA(subsystemname.into_param().abi(), handleid, objecttypename.into_param().abi(), objectname.into_param().abi(), securitydescriptor.into_param().abi(), principalselfsid.into_param().abi(), desiredaccess, audittype, flags, ::core::mem::transmute(objecttypelist.unwrap_or(::std::ptr::null_mut())), objecttypelistlength, genericmapping, objectcreation.into_param().abi(), grantedaccess, accessstatuslist, pfgenerateonclose)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AccessCheckByTypeResultListAndAuditAlarmByHandleA<P0, P1, P2, P3, P4, P5, P6>(subsystemname: P0, handleid: *const ::core::ffi::c_void, clienttoken: P1, objecttypename: P2, objectname: P3, securitydescriptor: P4, principalselfsid: P5, desiredaccess: u32, audittype: AUDIT_EVENT_TYPE, flags: u32, objecttypelist: ::core::option::Option<*mut OBJECT_TYPE_LIST>, objecttypelistlength: u32, genericmapping: *const GENERIC_MAPPING, objectcreation: P6, grantedaccess: *mut u32, accessstatuslist: *mut u32, pfgenerateonclose: *mut i32) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<super::Foundation::HANDLE>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P4: ::windows::core::IntoParam<PSECURITY_DESCRIPTOR>,
    P5: ::windows::core::IntoParam<super::Foundation::PSID>,
    P6: ::windows::core::IntoParam<super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn AccessCheckByTypeResultListAndAuditAlarmByHandleA ( subsystemname : :: windows::core::PCSTR , handleid : *const ::core::ffi::c_void , clienttoken : super::Foundation:: HANDLE , objecttypename : :: windows::core::PCSTR , objectname : :: windows::core::PCSTR , securitydescriptor : PSECURITY_DESCRIPTOR , principalselfsid : super::Foundation:: PSID , desiredaccess : u32 , audittype : AUDIT_EVENT_TYPE , flags : u32 , objecttypelist : *mut OBJECT_TYPE_LIST , objecttypelistlength : u32 , genericmapping : *const GENERIC_MAPPING , objectcreation : super::Foundation:: BOOL , grantedaccess : *mut u32 , accessstatuslist : *mut u32 , pfgenerateonclose : *mut i32 ) -> super::Foundation:: BOOL );
    AccessCheckByTypeResultListAndAuditAlarmByHandleA(subsystemname.into_param().abi(), handleid, clienttoken.into_param().abi(), objecttypename.into_param().abi(), objectname.into_param().abi(), securitydescriptor.into_param().abi(), principalselfsid.into_param().abi(), desiredaccess, audittype, flags, ::core::mem::transmute(objecttypelist.unwrap_or(::std::ptr::null_mut())), objecttypelistlength, genericmapping, objectcreation.into_param().abi(), grantedaccess, accessstatuslist, pfgenerateonclose)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AccessCheckByTypeResultListAndAuditAlarmByHandleW<P0, P1, P2, P3, P4, P5, P6>(subsystemname: P0, handleid: *const ::core::ffi::c_void, clienttoken: P1, objecttypename: P2, objectname: P3, securitydescriptor: P4, principalselfsid: P5, desiredaccess: u32, audittype: AUDIT_EVENT_TYPE, flags: u32, objecttypelist: ::core::option::Option<*mut OBJECT_TYPE_LIST>, objecttypelistlength: u32, genericmapping: *const GENERIC_MAPPING, objectcreation: P6, grantedaccesslist: *mut u32, accessstatuslist: *mut u32, pfgenerateonclose: *mut i32) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<super::Foundation::HANDLE>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P4: ::windows::core::IntoParam<PSECURITY_DESCRIPTOR>,
    P5: ::windows::core::IntoParam<super::Foundation::PSID>,
    P6: ::windows::core::IntoParam<super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn AccessCheckByTypeResultListAndAuditAlarmByHandleW ( subsystemname : :: windows::core::PCWSTR , handleid : *const ::core::ffi::c_void , clienttoken : super::Foundation:: HANDLE , objecttypename : :: windows::core::PCWSTR , objectname : :: windows::core::PCWSTR , securitydescriptor : PSECURITY_DESCRIPTOR , principalselfsid : super::Foundation:: PSID , desiredaccess : u32 , audittype : AUDIT_EVENT_TYPE , flags : u32 , objecttypelist : *mut OBJECT_TYPE_LIST , objecttypelistlength : u32 , genericmapping : *const GENERIC_MAPPING , objectcreation : super::Foundation:: BOOL , grantedaccesslist : *mut u32 , accessstatuslist : *mut u32 , pfgenerateonclose : *mut i32 ) -> super::Foundation:: BOOL );
    AccessCheckByTypeResultListAndAuditAlarmByHandleW(subsystemname.into_param().abi(), handleid, clienttoken.into_param().abi(), objecttypename.into_param().abi(), objectname.into_param().abi(), securitydescriptor.into_param().abi(), principalselfsid.into_param().abi(), desiredaccess, audittype, flags, ::core::mem::transmute(objecttypelist.unwrap_or(::std::ptr::null_mut())), objecttypelistlength, genericmapping, objectcreation.into_param().abi(), grantedaccesslist, accessstatuslist, pfgenerateonclose)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AccessCheckByTypeResultListAndAuditAlarmW<P0, P1, P2, P3, P4, P5>(subsystemname: P0, handleid: *const ::core::ffi::c_void, objecttypename: P1, objectname: P2, securitydescriptor: P3, principalselfsid: P4, desiredaccess: u32, audittype: AUDIT_EVENT_TYPE, flags: u32, objecttypelist: ::core::option::Option<*mut OBJECT_TYPE_LIST>, objecttypelistlength: u32, genericmapping: *const GENERIC_MAPPING, objectcreation: P5, grantedaccesslist: *mut u32, accessstatuslist: *mut u32, pfgenerateonclose: *mut i32) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P3: ::windows::core::IntoParam<PSECURITY_DESCRIPTOR>,
    P4: ::windows::core::IntoParam<super::Foundation::PSID>,
    P5: ::windows::core::IntoParam<super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn AccessCheckByTypeResultListAndAuditAlarmW ( subsystemname : :: windows::core::PCWSTR , handleid : *const ::core::ffi::c_void , objecttypename : :: windows::core::PCWSTR , objectname : :: windows::core::PCWSTR , securitydescriptor : PSECURITY_DESCRIPTOR , principalselfsid : super::Foundation:: PSID , desiredaccess : u32 , audittype : AUDIT_EVENT_TYPE , flags : u32 , objecttypelist : *mut OBJECT_TYPE_LIST , objecttypelistlength : u32 , genericmapping : *const GENERIC_MAPPING , objectcreation : super::Foundation:: BOOL , grantedaccesslist : *mut u32 , accessstatuslist : *mut u32 , pfgenerateonclose : *mut i32 ) -> super::Foundation:: BOOL );
    AccessCheckByTypeResultListAndAuditAlarmW(subsystemname.into_param().abi(), handleid, objecttypename.into_param().abi(), objectname.into_param().abi(), securitydescriptor.into_param().abi(), principalselfsid.into_param().abi(), desiredaccess, audittype, flags, ::core::mem::transmute(objecttypelist.unwrap_or(::std::ptr::null_mut())), objecttypelistlength, genericmapping, objectcreation.into_param().abi(), grantedaccesslist, accessstatuslist, pfgenerateonclose)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddAccessAllowedAce<P0>(pacl: *mut ACL, dwacerevision: ACE_REVISION, accessmask: u32, psid: P0) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::Foundation::PSID>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn AddAccessAllowedAce ( pacl : *mut ACL , dwacerevision : ACE_REVISION , accessmask : u32 , psid : super::Foundation:: PSID ) -> super::Foundation:: BOOL );
    AddAccessAllowedAce(pacl, dwacerevision, accessmask, psid.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddAccessAllowedAceEx<P0>(pacl: *mut ACL, dwacerevision: ACE_REVISION, aceflags: ACE_FLAGS, accessmask: u32, psid: P0) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::Foundation::PSID>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn AddAccessAllowedAceEx ( pacl : *mut ACL , dwacerevision : ACE_REVISION , aceflags : ACE_FLAGS , accessmask : u32 , psid : super::Foundation:: PSID ) -> super::Foundation:: BOOL );
    AddAccessAllowedAceEx(pacl, dwacerevision, aceflags, accessmask, psid.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddAccessAllowedObjectAce<P0>(pacl: *mut ACL, dwacerevision: ACE_REVISION, aceflags: ACE_FLAGS, accessmask: u32, objecttypeguid: ::core::option::Option<*const ::windows::core::GUID>, inheritedobjecttypeguid: ::core::option::Option<*const ::windows::core::GUID>, psid: P0) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::Foundation::PSID>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn AddAccessAllowedObjectAce ( pacl : *mut ACL , dwacerevision : ACE_REVISION , aceflags : ACE_FLAGS , accessmask : u32 , objecttypeguid : *const :: windows::core::GUID , inheritedobjecttypeguid : *const :: windows::core::GUID , psid : super::Foundation:: PSID ) -> super::Foundation:: BOOL );
    AddAccessAllowedObjectAce(pacl, dwacerevision, aceflags, accessmask, ::core::mem::transmute(objecttypeguid.unwrap_or(::std::ptr::null())), ::core::mem::transmute(inheritedobjecttypeguid.unwrap_or(::std::ptr::null())), psid.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddAccessDeniedAce<P0>(pacl: *mut ACL, dwacerevision: ACE_REVISION, accessmask: u32, psid: P0) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::Foundation::PSID>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn AddAccessDeniedAce ( pacl : *mut ACL , dwacerevision : ACE_REVISION , accessmask : u32 , psid : super::Foundation:: PSID ) -> super::Foundation:: BOOL );
    AddAccessDeniedAce(pacl, dwacerevision, accessmask, psid.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddAccessDeniedAceEx<P0>(pacl: *mut ACL, dwacerevision: ACE_REVISION, aceflags: ACE_FLAGS, accessmask: u32, psid: P0) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::Foundation::PSID>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn AddAccessDeniedAceEx ( pacl : *mut ACL , dwacerevision : ACE_REVISION , aceflags : ACE_FLAGS , accessmask : u32 , psid : super::Foundation:: PSID ) -> super::Foundation:: BOOL );
    AddAccessDeniedAceEx(pacl, dwacerevision, aceflags, accessmask, psid.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddAccessDeniedObjectAce<P0>(pacl: *mut ACL, dwacerevision: ACE_REVISION, aceflags: ACE_FLAGS, accessmask: u32, objecttypeguid: ::core::option::Option<*const ::windows::core::GUID>, inheritedobjecttypeguid: ::core::option::Option<*const ::windows::core::GUID>, psid: P0) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::Foundation::PSID>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn AddAccessDeniedObjectAce ( pacl : *mut ACL , dwacerevision : ACE_REVISION , aceflags : ACE_FLAGS , accessmask : u32 , objecttypeguid : *const :: windows::core::GUID , inheritedobjecttypeguid : *const :: windows::core::GUID , psid : super::Foundation:: PSID ) -> super::Foundation:: BOOL );
    AddAccessDeniedObjectAce(pacl, dwacerevision, aceflags, accessmask, ::core::mem::transmute(objecttypeguid.unwrap_or(::std::ptr::null())), ::core::mem::transmute(inheritedobjecttypeguid.unwrap_or(::std::ptr::null())), psid.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddAce(pacl: *mut ACL, dwacerevision: ACE_REVISION, dwstartingaceindex: u32, pacelist: *const ::core::ffi::c_void, nacelistlength: u32) -> super::Foundation::BOOL {
    ::windows::imp::link ! ( "advapi32.dll""system" fn AddAce ( pacl : *mut ACL , dwacerevision : ACE_REVISION , dwstartingaceindex : u32 , pacelist : *const ::core::ffi::c_void , nacelistlength : u32 ) -> super::Foundation:: BOOL );
    AddAce(pacl, dwacerevision, dwstartingaceindex, pacelist, nacelistlength)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddAuditAccessAce<P0, P1, P2>(pacl: *mut ACL, dwacerevision: ACE_REVISION, dwaccessmask: u32, psid: P0, bauditsuccess: P1, bauditfailure: P2) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::Foundation::PSID>,
    P1: ::windows::core::IntoParam<super::Foundation::BOOL>,
    P2: ::windows::core::IntoParam<super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn AddAuditAccessAce ( pacl : *mut ACL , dwacerevision : ACE_REVISION , dwaccessmask : u32 , psid : super::Foundation:: PSID , bauditsuccess : super::Foundation:: BOOL , bauditfailure : super::Foundation:: BOOL ) -> super::Foundation:: BOOL );
    AddAuditAccessAce(pacl, dwacerevision, dwaccessmask, psid.into_param().abi(), bauditsuccess.into_param().abi(), bauditfailure.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddAuditAccessAceEx<P0, P1, P2>(pacl: *mut ACL, dwacerevision: ACE_REVISION, aceflags: ACE_FLAGS, dwaccessmask: u32, psid: P0, bauditsuccess: P1, bauditfailure: P2) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::Foundation::PSID>,
    P1: ::windows::core::IntoParam<super::Foundation::BOOL>,
    P2: ::windows::core::IntoParam<super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn AddAuditAccessAceEx ( pacl : *mut ACL , dwacerevision : ACE_REVISION , aceflags : ACE_FLAGS , dwaccessmask : u32 , psid : super::Foundation:: PSID , bauditsuccess : super::Foundation:: BOOL , bauditfailure : super::Foundation:: BOOL ) -> super::Foundation:: BOOL );
    AddAuditAccessAceEx(pacl, dwacerevision, aceflags, dwaccessmask, psid.into_param().abi(), bauditsuccess.into_param().abi(), bauditfailure.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddAuditAccessObjectAce<P0, P1, P2>(pacl: *mut ACL, dwacerevision: ACE_REVISION, aceflags: ACE_FLAGS, accessmask: u32, objecttypeguid: ::core::option::Option<*const ::windows::core::GUID>, inheritedobjecttypeguid: ::core::option::Option<*const ::windows::core::GUID>, psid: P0, bauditsuccess: P1, bauditfailure: P2) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::Foundation::PSID>,
    P1: ::windows::core::IntoParam<super::Foundation::BOOL>,
    P2: ::windows::core::IntoParam<super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn AddAuditAccessObjectAce ( pacl : *mut ACL , dwacerevision : ACE_REVISION , aceflags : ACE_FLAGS , accessmask : u32 , objecttypeguid : *const :: windows::core::GUID , inheritedobjecttypeguid : *const :: windows::core::GUID , psid : super::Foundation:: PSID , bauditsuccess : super::Foundation:: BOOL , bauditfailure : super::Foundation:: BOOL ) -> super::Foundation:: BOOL );
    AddAuditAccessObjectAce(pacl, dwacerevision, aceflags, accessmask, ::core::mem::transmute(objecttypeguid.unwrap_or(::std::ptr::null())), ::core::mem::transmute(inheritedobjecttypeguid.unwrap_or(::std::ptr::null())), psid.into_param().abi(), bauditsuccess.into_param().abi(), bauditfailure.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddConditionalAce<P0, P1>(pacl: *mut ACL, dwacerevision: ACE_REVISION, aceflags: ACE_FLAGS, acetype: u8, accessmask: u32, psid: P0, conditionstr: P1, returnlength: *mut u32) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::Foundation::PSID>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn AddConditionalAce ( pacl : *mut ACL , dwacerevision : ACE_REVISION , aceflags : ACE_FLAGS , acetype : u8 , accessmask : u32 , psid : super::Foundation:: PSID , conditionstr : :: windows::core::PCWSTR , returnlength : *mut u32 ) -> super::Foundation:: BOOL );
    AddConditionalAce(pacl, dwacerevision, aceflags, acetype, accessmask, psid.into_param().abi(), conditionstr.into_param().abi(), returnlength)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddMandatoryAce<P0>(pacl: *mut ACL, dwacerevision: ACE_REVISION, aceflags: ACE_FLAGS, mandatorypolicy: u32, plabelsid: P0) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::Foundation::PSID>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn AddMandatoryAce ( pacl : *mut ACL , dwacerevision : ACE_REVISION , aceflags : ACE_FLAGS , mandatorypolicy : u32 , plabelsid : super::Foundation:: PSID ) -> super::Foundation:: BOOL );
    AddMandatoryAce(pacl, dwacerevision, aceflags, mandatorypolicy, plabelsid.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddResourceAttributeAce<P0>(pacl: *mut ACL, dwacerevision: ACE_REVISION, aceflags: ACE_FLAGS, accessmask: u32, psid: P0, pattributeinfo: *const CLAIM_SECURITY_ATTRIBUTES_INFORMATION, preturnlength: *mut u32) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::Foundation::PSID>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn AddResourceAttributeAce ( pacl : *mut ACL , dwacerevision : ACE_REVISION , aceflags : ACE_FLAGS , accessmask : u32 , psid : super::Foundation:: PSID , pattributeinfo : *const CLAIM_SECURITY_ATTRIBUTES_INFORMATION , preturnlength : *mut u32 ) -> super::Foundation:: BOOL );
    AddResourceAttributeAce(pacl, dwacerevision, aceflags, accessmask, psid.into_param().abi(), pattributeinfo, preturnlength)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddScopedPolicyIDAce<P0>(pacl: *mut ACL, dwacerevision: ACE_REVISION, aceflags: ACE_FLAGS, accessmask: u32, psid: P0) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::Foundation::PSID>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn AddScopedPolicyIDAce ( pacl : *mut ACL , dwacerevision : ACE_REVISION , aceflags : ACE_FLAGS , accessmask : u32 , psid : super::Foundation:: PSID ) -> super::Foundation:: BOOL );
    AddScopedPolicyIDAce(pacl, dwacerevision, aceflags, accessmask, psid.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AdjustTokenGroups<P0, P1>(tokenhandle: P0, resettodefault: P1, newstate: ::core::option::Option<*const TOKEN_GROUPS>, bufferlength: u32, previousstate: ::core::option::Option<*mut TOKEN_GROUPS>, returnlength: ::core::option::Option<*mut u32>) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn AdjustTokenGroups ( tokenhandle : super::Foundation:: HANDLE , resettodefault : super::Foundation:: BOOL , newstate : *const TOKEN_GROUPS , bufferlength : u32 , previousstate : *mut TOKEN_GROUPS , returnlength : *mut u32 ) -> super::Foundation:: BOOL );
    AdjustTokenGroups(tokenhandle.into_param().abi(), resettodefault.into_param().abi(), ::core::mem::transmute(newstate.unwrap_or(::std::ptr::null())), bufferlength, ::core::mem::transmute(previousstate.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(returnlength.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AdjustTokenPrivileges<P0, P1>(tokenhandle: P0, disableallprivileges: P1, newstate: ::core::option::Option<*const TOKEN_PRIVILEGES>, bufferlength: u32, previousstate: ::core::option::Option<*mut TOKEN_PRIVILEGES>, returnlength: ::core::option::Option<*mut u32>) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn AdjustTokenPrivileges ( tokenhandle : super::Foundation:: HANDLE , disableallprivileges : super::Foundation:: BOOL , newstate : *const TOKEN_PRIVILEGES , bufferlength : u32 , previousstate : *mut TOKEN_PRIVILEGES , returnlength : *mut u32 ) -> super::Foundation:: BOOL );
    AdjustTokenPrivileges(tokenhandle.into_param().abi(), disableallprivileges.into_param().abi(), ::core::mem::transmute(newstate.unwrap_or(::std::ptr::null())), bufferlength, ::core::mem::transmute(previousstate.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(returnlength.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AllocateAndInitializeSid(pidentifierauthority: *const SID_IDENTIFIER_AUTHORITY, nsubauthoritycount: u8, nsubauthority0: u32, nsubauthority1: u32, nsubauthority2: u32, nsubauthority3: u32, nsubauthority4: u32, nsubauthority5: u32, nsubauthority6: u32, nsubauthority7: u32, psid: *mut super::Foundation::PSID) -> super::Foundation::BOOL {
    ::windows::imp::link ! ( "advapi32.dll""system" fn AllocateAndInitializeSid ( pidentifierauthority : *const SID_IDENTIFIER_AUTHORITY , nsubauthoritycount : u8 , nsubauthority0 : u32 , nsubauthority1 : u32 , nsubauthority2 : u32 , nsubauthority3 : u32 , nsubauthority4 : u32 , nsubauthority5 : u32 , nsubauthority6 : u32 , nsubauthority7 : u32 , psid : *mut super::Foundation:: PSID ) -> super::Foundation:: BOOL );
    AllocateAndInitializeSid(pidentifierauthority, nsubauthoritycount, nsubauthority0, nsubauthority1, nsubauthority2, nsubauthority3, nsubauthority4, nsubauthority5, nsubauthority6, nsubauthority7, psid)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AllocateLocallyUniqueId(luid: *mut super::Foundation::LUID) -> super::Foundation::BOOL {
    ::windows::imp::link ! ( "advapi32.dll""system" fn AllocateLocallyUniqueId ( luid : *mut super::Foundation:: LUID ) -> super::Foundation:: BOOL );
    AllocateLocallyUniqueId(luid)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AreAllAccessesGranted(grantedaccess: u32, desiredaccess: u32) -> super::Foundation::BOOL {
    ::windows::imp::link ! ( "advapi32.dll""system" fn AreAllAccessesGranted ( grantedaccess : u32 , desiredaccess : u32 ) -> super::Foundation:: BOOL );
    AreAllAccessesGranted(grantedaccess, desiredaccess)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AreAnyAccessesGranted(grantedaccess: u32, desiredaccess: u32) -> super::Foundation::BOOL {
    ::windows::imp::link ! ( "advapi32.dll""system" fn AreAnyAccessesGranted ( grantedaccess : u32 , desiredaccess : u32 ) -> super::Foundation:: BOOL );
    AreAnyAccessesGranted(grantedaccess, desiredaccess)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CheckTokenCapability<P0, P1>(tokenhandle: P0, capabilitysidtocheck: P1, hascapability: *mut super::Foundation::BOOL) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<super::Foundation::PSID>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn CheckTokenCapability ( tokenhandle : super::Foundation:: HANDLE , capabilitysidtocheck : super::Foundation:: PSID , hascapability : *mut super::Foundation:: BOOL ) -> super::Foundation:: BOOL );
    CheckTokenCapability(tokenhandle.into_param().abi(), capabilitysidtocheck.into_param().abi(), hascapability)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CheckTokenMembership<P0, P1>(tokenhandle: P0, sidtocheck: P1, ismember: *mut super::Foundation::BOOL) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<super::Foundation::PSID>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn CheckTokenMembership ( tokenhandle : super::Foundation:: HANDLE , sidtocheck : super::Foundation:: PSID , ismember : *mut super::Foundation:: BOOL ) -> super::Foundation:: BOOL );
    CheckTokenMembership(tokenhandle.into_param().abi(), sidtocheck.into_param().abi(), ismember)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CheckTokenMembershipEx<P0, P1>(tokenhandle: P0, sidtocheck: P1, flags: u32, ismember: *mut super::Foundation::BOOL) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<super::Foundation::PSID>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn CheckTokenMembershipEx ( tokenhandle : super::Foundation:: HANDLE , sidtocheck : super::Foundation:: PSID , flags : u32 , ismember : *mut super::Foundation:: BOOL ) -> super::Foundation:: BOOL );
    CheckTokenMembershipEx(tokenhandle.into_param().abi(), sidtocheck.into_param().abi(), flags, ismember)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ConvertToAutoInheritPrivateObjectSecurity<P0, P1, P2>(parentdescriptor: P0, currentsecuritydescriptor: P1, newsecuritydescriptor: *mut PSECURITY_DESCRIPTOR, objecttype: ::core::option::Option<*const ::windows::core::GUID>, isdirectoryobject: P2, genericmapping: *const GENERIC_MAPPING) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<PSECURITY_DESCRIPTOR>,
    P1: ::windows::core::IntoParam<PSECURITY_DESCRIPTOR>,
    P2: ::windows::core::IntoParam<super::Foundation::BOOLEAN>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn ConvertToAutoInheritPrivateObjectSecurity ( parentdescriptor : PSECURITY_DESCRIPTOR , currentsecuritydescriptor : PSECURITY_DESCRIPTOR , newsecuritydescriptor : *mut PSECURITY_DESCRIPTOR , objecttype : *const :: windows::core::GUID , isdirectoryobject : super::Foundation:: BOOLEAN , genericmapping : *const GENERIC_MAPPING ) -> super::Foundation:: BOOL );
    ConvertToAutoInheritPrivateObjectSecurity(parentdescriptor.into_param().abi(), currentsecuritydescriptor.into_param().abi(), newsecuritydescriptor, ::core::mem::transmute(objecttype.unwrap_or(::std::ptr::null())), isdirectoryobject.into_param().abi(), genericmapping)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CopySid<P0>(ndestinationsidlength: u32, pdestinationsid: super::Foundation::PSID, psourcesid: P0) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::Foundation::PSID>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn CopySid ( ndestinationsidlength : u32 , pdestinationsid : super::Foundation:: PSID , psourcesid : super::Foundation:: PSID ) -> super::Foundation:: BOOL );
    CopySid(ndestinationsidlength, pdestinationsid, psourcesid.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreatePrivateObjectSecurity<P0, P1, P2, P3>(parentdescriptor: P0, creatordescriptor: P1, newdescriptor: *mut PSECURITY_DESCRIPTOR, isdirectoryobject: P2, token: P3, genericmapping: *const GENERIC_MAPPING) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<PSECURITY_DESCRIPTOR>,
    P1: ::windows::core::IntoParam<PSECURITY_DESCRIPTOR>,
    P2: ::windows::core::IntoParam<super::Foundation::BOOL>,
    P3: ::windows::core::IntoParam<super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn CreatePrivateObjectSecurity ( parentdescriptor : PSECURITY_DESCRIPTOR , creatordescriptor : PSECURITY_DESCRIPTOR , newdescriptor : *mut PSECURITY_DESCRIPTOR , isdirectoryobject : super::Foundation:: BOOL , token : super::Foundation:: HANDLE , genericmapping : *const GENERIC_MAPPING ) -> super::Foundation:: BOOL );
    CreatePrivateObjectSecurity(parentdescriptor.into_param().abi(), creatordescriptor.into_param().abi(), newdescriptor, isdirectoryobject.into_param().abi(), token.into_param().abi(), genericmapping)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreatePrivateObjectSecurityEx<P0, P1, P2, P3>(parentdescriptor: P0, creatordescriptor: P1, newdescriptor: *mut PSECURITY_DESCRIPTOR, objecttype: ::core::option::Option<*const ::windows::core::GUID>, iscontainerobject: P2, autoinheritflags: SECURITY_AUTO_INHERIT_FLAGS, token: P3, genericmapping: *const GENERIC_MAPPING) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<PSECURITY_DESCRIPTOR>,
    P1: ::windows::core::IntoParam<PSECURITY_DESCRIPTOR>,
    P2: ::windows::core::IntoParam<super::Foundation::BOOL>,
    P3: ::windows::core::IntoParam<super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn CreatePrivateObjectSecurityEx ( parentdescriptor : PSECURITY_DESCRIPTOR , creatordescriptor : PSECURITY_DESCRIPTOR , newdescriptor : *mut PSECURITY_DESCRIPTOR , objecttype : *const :: windows::core::GUID , iscontainerobject : super::Foundation:: BOOL , autoinheritflags : SECURITY_AUTO_INHERIT_FLAGS , token : super::Foundation:: HANDLE , genericmapping : *const GENERIC_MAPPING ) -> super::Foundation:: BOOL );
    CreatePrivateObjectSecurityEx(parentdescriptor.into_param().abi(), creatordescriptor.into_param().abi(), newdescriptor, ::core::mem::transmute(objecttype.unwrap_or(::std::ptr::null())), iscontainerobject.into_param().abi(), autoinheritflags, token.into_param().abi(), genericmapping)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreatePrivateObjectSecurityWithMultipleInheritance<P0, P1, P2, P3>(parentdescriptor: P0, creatordescriptor: P1, newdescriptor: *mut PSECURITY_DESCRIPTOR, objecttypes: ::core::option::Option<&[*const ::windows::core::GUID]>, iscontainerobject: P2, autoinheritflags: SECURITY_AUTO_INHERIT_FLAGS, token: P3, genericmapping: *const GENERIC_MAPPING) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<PSECURITY_DESCRIPTOR>,
    P1: ::windows::core::IntoParam<PSECURITY_DESCRIPTOR>,
    P2: ::windows::core::IntoParam<super::Foundation::BOOL>,
    P3: ::windows::core::IntoParam<super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn CreatePrivateObjectSecurityWithMultipleInheritance ( parentdescriptor : PSECURITY_DESCRIPTOR , creatordescriptor : PSECURITY_DESCRIPTOR , newdescriptor : *mut PSECURITY_DESCRIPTOR , objecttypes : *const *const :: windows::core::GUID , guidcount : u32 , iscontainerobject : super::Foundation:: BOOL , autoinheritflags : SECURITY_AUTO_INHERIT_FLAGS , token : super::Foundation:: HANDLE , genericmapping : *const GENERIC_MAPPING ) -> super::Foundation:: BOOL );
    CreatePrivateObjectSecurityWithMultipleInheritance(parentdescriptor.into_param().abi(), creatordescriptor.into_param().abi(), newdescriptor, ::core::mem::transmute(objecttypes.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), objecttypes.as_deref().map_or(0, |slice| slice.len() as _), iscontainerobject.into_param().abi(), autoinheritflags, token.into_param().abi(), genericmapping)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateRestrictedToken<P0>(existingtokenhandle: P0, flags: CREATE_RESTRICTED_TOKEN_FLAGS, sidstodisable: ::core::option::Option<&[SID_AND_ATTRIBUTES]>, privilegestodelete: ::core::option::Option<&[LUID_AND_ATTRIBUTES]>, sidstorestrict: ::core::option::Option<&[SID_AND_ATTRIBUTES]>, newtokenhandle: *mut super::Foundation::HANDLE) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn CreateRestrictedToken ( existingtokenhandle : super::Foundation:: HANDLE , flags : CREATE_RESTRICTED_TOKEN_FLAGS , disablesidcount : u32 , sidstodisable : *const SID_AND_ATTRIBUTES , deleteprivilegecount : u32 , privilegestodelete : *const LUID_AND_ATTRIBUTES , restrictedsidcount : u32 , sidstorestrict : *const SID_AND_ATTRIBUTES , newtokenhandle : *mut super::Foundation:: HANDLE ) -> super::Foundation:: BOOL );
    CreateRestrictedToken(
        existingtokenhandle.into_param().abi(),
        flags,
        sidstodisable.as_deref().map_or(0, |slice| slice.len() as _),
        ::core::mem::transmute(sidstodisable.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())),
        privilegestodelete.as_deref().map_or(0, |slice| slice.len() as _),
        ::core::mem::transmute(privilegestodelete.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())),
        sidstorestrict.as_deref().map_or(0, |slice| slice.len() as _),
        ::core::mem::transmute(sidstorestrict.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())),
        newtokenhandle,
    )
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateWellKnownSid<P0>(wellknownsidtype: WELL_KNOWN_SID_TYPE, domainsid: P0, psid: super::Foundation::PSID, cbsid: *mut u32) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::Foundation::PSID>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn CreateWellKnownSid ( wellknownsidtype : WELL_KNOWN_SID_TYPE , domainsid : super::Foundation:: PSID , psid : super::Foundation:: PSID , cbsid : *mut u32 ) -> super::Foundation:: BOOL );
    CreateWellKnownSid(wellknownsidtype, domainsid.into_param().abi(), psid, cbsid)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeleteAce(pacl: *mut ACL, dwaceindex: u32) -> super::Foundation::BOOL {
    ::windows::imp::link ! ( "advapi32.dll""system" fn DeleteAce ( pacl : *mut ACL , dwaceindex : u32 ) -> super::Foundation:: BOOL );
    DeleteAce(pacl, dwaceindex)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeriveCapabilitySidsFromName<P0>(capname: P0, capabilitygroupsids: *mut *mut super::Foundation::PSID, capabilitygroupsidcount: *mut u32, capabilitysids: *mut *mut super::Foundation::PSID, capabilitysidcount: *mut u32) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "api-ms-win-security-base-l1-2-2.dll""system" fn DeriveCapabilitySidsFromName ( capname : :: windows::core::PCWSTR , capabilitygroupsids : *mut *mut super::Foundation:: PSID , capabilitygroupsidcount : *mut u32 , capabilitysids : *mut *mut super::Foundation:: PSID , capabilitysidcount : *mut u32 ) -> super::Foundation:: BOOL );
    DeriveCapabilitySidsFromName(capname.into_param().abi(), capabilitygroupsids, capabilitygroupsidcount, capabilitysids, capabilitysidcount)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DestroyPrivateObjectSecurity(objectdescriptor: *const PSECURITY_DESCRIPTOR) -> super::Foundation::BOOL {
    ::windows::imp::link ! ( "advapi32.dll""system" fn DestroyPrivateObjectSecurity ( objectdescriptor : *const PSECURITY_DESCRIPTOR ) -> super::Foundation:: BOOL );
    DestroyPrivateObjectSecurity(objectdescriptor)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DuplicateToken<P0>(existingtokenhandle: P0, impersonationlevel: SECURITY_IMPERSONATION_LEVEL, duplicatetokenhandle: *mut super::Foundation::HANDLE) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn DuplicateToken ( existingtokenhandle : super::Foundation:: HANDLE , impersonationlevel : SECURITY_IMPERSONATION_LEVEL , duplicatetokenhandle : *mut super::Foundation:: HANDLE ) -> super::Foundation:: BOOL );
    DuplicateToken(existingtokenhandle.into_param().abi(), impersonationlevel, duplicatetokenhandle)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DuplicateTokenEx<P0>(hexistingtoken: P0, dwdesiredaccess: TOKEN_ACCESS_MASK, lptokenattributes: ::core::option::Option<*const SECURITY_ATTRIBUTES>, impersonationlevel: SECURITY_IMPERSONATION_LEVEL, tokentype: TOKEN_TYPE, phnewtoken: *mut super::Foundation::HANDLE) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn DuplicateTokenEx ( hexistingtoken : super::Foundation:: HANDLE , dwdesiredaccess : TOKEN_ACCESS_MASK , lptokenattributes : *const SECURITY_ATTRIBUTES , impersonationlevel : SECURITY_IMPERSONATION_LEVEL , tokentype : TOKEN_TYPE , phnewtoken : *mut super::Foundation:: HANDLE ) -> super::Foundation:: BOOL );
    DuplicateTokenEx(hexistingtoken.into_param().abi(), dwdesiredaccess, ::core::mem::transmute(lptokenattributes.unwrap_or(::std::ptr::null())), impersonationlevel, tokentype, phnewtoken)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EqualDomainSid<P0, P1>(psid1: P0, psid2: P1, pfequal: *mut super::Foundation::BOOL) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::Foundation::PSID>,
    P1: ::windows::core::IntoParam<super::Foundation::PSID>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn EqualDomainSid ( psid1 : super::Foundation:: PSID , psid2 : super::Foundation:: PSID , pfequal : *mut super::Foundation:: BOOL ) -> super::Foundation:: BOOL );
    EqualDomainSid(psid1.into_param().abi(), psid2.into_param().abi(), pfequal)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EqualPrefixSid<P0, P1>(psid1: P0, psid2: P1) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::Foundation::PSID>,
    P1: ::windows::core::IntoParam<super::Foundation::PSID>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn EqualPrefixSid ( psid1 : super::Foundation:: PSID , psid2 : super::Foundation:: PSID ) -> super::Foundation:: BOOL );
    EqualPrefixSid(psid1.into_param().abi(), psid2.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EqualSid<P0, P1>(psid1: P0, psid2: P1) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::Foundation::PSID>,
    P1: ::windows::core::IntoParam<super::Foundation::PSID>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn EqualSid ( psid1 : super::Foundation:: PSID , psid2 : super::Foundation:: PSID ) -> super::Foundation:: BOOL );
    EqualSid(psid1.into_param().abi(), psid2.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindFirstFreeAce(pacl: *const ACL, pace: *mut *mut ::core::ffi::c_void) -> super::Foundation::BOOL {
    ::windows::imp::link ! ( "advapi32.dll""system" fn FindFirstFreeAce ( pacl : *const ACL , pace : *mut *mut ::core::ffi::c_void ) -> super::Foundation:: BOOL );
    FindFirstFreeAce(pacl, pace)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FreeSid<P0>(psid: P0) -> *mut ::core::ffi::c_void
where
    P0: ::windows::core::IntoParam<super::Foundation::PSID>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn FreeSid ( psid : super::Foundation:: PSID ) -> *mut ::core::ffi::c_void );
    FreeSid(psid.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetAce(pacl: *const ACL, dwaceindex: u32, pace: *mut *mut ::core::ffi::c_void) -> super::Foundation::BOOL {
    ::windows::imp::link ! ( "advapi32.dll""system" fn GetAce ( pacl : *const ACL , dwaceindex : u32 , pace : *mut *mut ::core::ffi::c_void ) -> super::Foundation:: BOOL );
    GetAce(pacl, dwaceindex, pace)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetAclInformation(pacl: *const ACL, paclinformation: *mut ::core::ffi::c_void, naclinformationlength: u32, dwaclinformationclass: ACL_INFORMATION_CLASS) -> super::Foundation::BOOL {
    ::windows::imp::link ! ( "advapi32.dll""system" fn GetAclInformation ( pacl : *const ACL , paclinformation : *mut ::core::ffi::c_void , naclinformationlength : u32 , dwaclinformationclass : ACL_INFORMATION_CLASS ) -> super::Foundation:: BOOL );
    GetAclInformation(pacl, paclinformation, naclinformationlength, dwaclinformationclass)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetAppContainerAce(acl: *const ACL, startingaceindex: u32, appcontainerace: *mut *mut ::core::ffi::c_void, appcontaineraceindex: ::core::option::Option<*mut u32>) -> super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetAppContainerAce ( acl : *const ACL , startingaceindex : u32 , appcontainerace : *mut *mut ::core::ffi::c_void , appcontaineraceindex : *mut u32 ) -> super::Foundation:: BOOL );
    GetAppContainerAce(acl, startingaceindex, appcontainerace, ::core::mem::transmute(appcontaineraceindex.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCachedSigningLevel<P0>(file: P0, flags: *mut u32, signinglevel: *mut u32, thumbprint: ::core::option::Option<*mut u8>, thumbprintsize: ::core::option::Option<*mut u32>, thumbprintalgorithm: ::core::option::Option<*mut u32>) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetCachedSigningLevel ( file : super::Foundation:: HANDLE , flags : *mut u32 , signinglevel : *mut u32 , thumbprint : *mut u8 , thumbprintsize : *mut u32 , thumbprintalgorithm : *mut u32 ) -> super::Foundation:: BOOL );
    GetCachedSigningLevel(file.into_param().abi(), flags, signinglevel, ::core::mem::transmute(thumbprint.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(thumbprintsize.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(thumbprintalgorithm.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFileSecurityA<P0>(lpfilename: P0, requestedinformation: u32, psecuritydescriptor: PSECURITY_DESCRIPTOR, nlength: u32, lpnlengthneeded: *mut u32) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn GetFileSecurityA ( lpfilename : :: windows::core::PCSTR , requestedinformation : u32 , psecuritydescriptor : PSECURITY_DESCRIPTOR , nlength : u32 , lpnlengthneeded : *mut u32 ) -> super::Foundation:: BOOL );
    GetFileSecurityA(lpfilename.into_param().abi(), requestedinformation, psecuritydescriptor, nlength, lpnlengthneeded)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFileSecurityW<P0>(lpfilename: P0, requestedinformation: u32, psecuritydescriptor: PSECURITY_DESCRIPTOR, nlength: u32, lpnlengthneeded: *mut u32) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn GetFileSecurityW ( lpfilename : :: windows::core::PCWSTR , requestedinformation : u32 , psecuritydescriptor : PSECURITY_DESCRIPTOR , nlength : u32 , lpnlengthneeded : *mut u32 ) -> super::Foundation:: BOOL );
    GetFileSecurityW(lpfilename.into_param().abi(), requestedinformation, psecuritydescriptor, nlength, lpnlengthneeded)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetKernelObjectSecurity<P0>(handle: P0, requestedinformation: u32, psecuritydescriptor: PSECURITY_DESCRIPTOR, nlength: u32, lpnlengthneeded: *mut u32) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn GetKernelObjectSecurity ( handle : super::Foundation:: HANDLE , requestedinformation : u32 , psecuritydescriptor : PSECURITY_DESCRIPTOR , nlength : u32 , lpnlengthneeded : *mut u32 ) -> super::Foundation:: BOOL );
    GetKernelObjectSecurity(handle.into_param().abi(), requestedinformation, psecuritydescriptor, nlength, lpnlengthneeded)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetLengthSid(psid: super::Foundation::PSID) -> u32 {
    ::windows::imp::link ! ( "advapi32.dll""system" fn GetLengthSid ( psid : super::Foundation:: PSID ) -> u32 );
    GetLengthSid(psid)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPrivateObjectSecurity<P0>(objectdescriptor: P0, securityinformation: u32, resultantdescriptor: PSECURITY_DESCRIPTOR, descriptorlength: u32, returnlength: *mut u32) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<PSECURITY_DESCRIPTOR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn GetPrivateObjectSecurity ( objectdescriptor : PSECURITY_DESCRIPTOR , securityinformation : u32 , resultantdescriptor : PSECURITY_DESCRIPTOR , descriptorlength : u32 , returnlength : *mut u32 ) -> super::Foundation:: BOOL );
    GetPrivateObjectSecurity(objectdescriptor.into_param().abi(), securityinformation, resultantdescriptor, descriptorlength, returnlength)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSecurityDescriptorControl<P0>(psecuritydescriptor: P0, pcontrol: *mut u16, lpdwrevision: *mut u32) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<PSECURITY_DESCRIPTOR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn GetSecurityDescriptorControl ( psecuritydescriptor : PSECURITY_DESCRIPTOR , pcontrol : *mut u16 , lpdwrevision : *mut u32 ) -> super::Foundation:: BOOL );
    GetSecurityDescriptorControl(psecuritydescriptor.into_param().abi(), pcontrol, lpdwrevision)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSecurityDescriptorDacl<P0>(psecuritydescriptor: P0, lpbdaclpresent: *mut i32, pdacl: *mut *mut ACL, lpbdacldefaulted: *mut i32) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<PSECURITY_DESCRIPTOR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn GetSecurityDescriptorDacl ( psecuritydescriptor : PSECURITY_DESCRIPTOR , lpbdaclpresent : *mut i32 , pdacl : *mut *mut ACL , lpbdacldefaulted : *mut i32 ) -> super::Foundation:: BOOL );
    GetSecurityDescriptorDacl(psecuritydescriptor.into_param().abi(), lpbdaclpresent, pdacl, lpbdacldefaulted)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSecurityDescriptorGroup<P0>(psecuritydescriptor: P0, pgroup: *mut super::Foundation::PSID, lpbgroupdefaulted: *mut i32) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<PSECURITY_DESCRIPTOR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn GetSecurityDescriptorGroup ( psecuritydescriptor : PSECURITY_DESCRIPTOR , pgroup : *mut super::Foundation:: PSID , lpbgroupdefaulted : *mut i32 ) -> super::Foundation:: BOOL );
    GetSecurityDescriptorGroup(psecuritydescriptor.into_param().abi(), pgroup, lpbgroupdefaulted)
}
#[doc = "*Required features: `\"Win32_Security\"`*"]
#[inline]
pub unsafe fn GetSecurityDescriptorLength<P0>(psecuritydescriptor: P0) -> u32
where
    P0: ::windows::core::IntoParam<PSECURITY_DESCRIPTOR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn GetSecurityDescriptorLength ( psecuritydescriptor : PSECURITY_DESCRIPTOR ) -> u32 );
    GetSecurityDescriptorLength(psecuritydescriptor.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSecurityDescriptorOwner<P0>(psecuritydescriptor: P0, powner: *mut super::Foundation::PSID, lpbownerdefaulted: *mut i32) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<PSECURITY_DESCRIPTOR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn GetSecurityDescriptorOwner ( psecuritydescriptor : PSECURITY_DESCRIPTOR , powner : *mut super::Foundation:: PSID , lpbownerdefaulted : *mut i32 ) -> super::Foundation:: BOOL );
    GetSecurityDescriptorOwner(psecuritydescriptor.into_param().abi(), powner, lpbownerdefaulted)
}
#[doc = "*Required features: `\"Win32_Security\"`*"]
#[inline]
pub unsafe fn GetSecurityDescriptorRMControl<P0>(securitydescriptor: P0, rmcontrol: *mut u8) -> u32
where
    P0: ::windows::core::IntoParam<PSECURITY_DESCRIPTOR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn GetSecurityDescriptorRMControl ( securitydescriptor : PSECURITY_DESCRIPTOR , rmcontrol : *mut u8 ) -> u32 );
    GetSecurityDescriptorRMControl(securitydescriptor.into_param().abi(), rmcontrol)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSecurityDescriptorSacl<P0>(psecuritydescriptor: P0, lpbsaclpresent: *mut i32, psacl: *mut *mut ACL, lpbsacldefaulted: *mut i32) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<PSECURITY_DESCRIPTOR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn GetSecurityDescriptorSacl ( psecuritydescriptor : PSECURITY_DESCRIPTOR , lpbsaclpresent : *mut i32 , psacl : *mut *mut ACL , lpbsacldefaulted : *mut i32 ) -> super::Foundation:: BOOL );
    GetSecurityDescriptorSacl(psecuritydescriptor.into_param().abi(), lpbsaclpresent, psacl, lpbsacldefaulted)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSidIdentifierAuthority<P0>(psid: P0) -> *mut SID_IDENTIFIER_AUTHORITY
where
    P0: ::windows::core::IntoParam<super::Foundation::PSID>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn GetSidIdentifierAuthority ( psid : super::Foundation:: PSID ) -> *mut SID_IDENTIFIER_AUTHORITY );
    GetSidIdentifierAuthority(psid.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security\"`*"]
#[inline]
pub unsafe fn GetSidLengthRequired(nsubauthoritycount: u8) -> u32 {
    ::windows::imp::link ! ( "advapi32.dll""system" fn GetSidLengthRequired ( nsubauthoritycount : u8 ) -> u32 );
    GetSidLengthRequired(nsubauthoritycount)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSidSubAuthority<P0>(psid: P0, nsubauthority: u32) -> *mut u32
where
    P0: ::windows::core::IntoParam<super::Foundation::PSID>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn GetSidSubAuthority ( psid : super::Foundation:: PSID , nsubauthority : u32 ) -> *mut u32 );
    GetSidSubAuthority(psid.into_param().abi(), nsubauthority)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSidSubAuthorityCount<P0>(psid: P0) -> *mut u8
where
    P0: ::windows::core::IntoParam<super::Foundation::PSID>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn GetSidSubAuthorityCount ( psid : super::Foundation:: PSID ) -> *mut u8 );
    GetSidSubAuthorityCount(psid.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetTokenInformation<P0>(tokenhandle: P0, tokeninformationclass: TOKEN_INFORMATION_CLASS, tokeninformation: ::core::option::Option<*mut ::core::ffi::c_void>, tokeninformationlength: u32, returnlength: *mut u32) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn GetTokenInformation ( tokenhandle : super::Foundation:: HANDLE , tokeninformationclass : TOKEN_INFORMATION_CLASS , tokeninformation : *mut ::core::ffi::c_void , tokeninformationlength : u32 , returnlength : *mut u32 ) -> super::Foundation:: BOOL );
    GetTokenInformation(tokenhandle.into_param().abi(), tokeninformationclass, ::core::mem::transmute(tokeninformation.unwrap_or(::std::ptr::null_mut())), tokeninformationlength, returnlength)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetUserObjectSecurity<P0>(hobj: P0, psirequested: *const u32, psid: PSECURITY_DESCRIPTOR, nlength: u32, lpnlengthneeded: *mut u32) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "user32.dll""system" fn GetUserObjectSecurity ( hobj : super::Foundation:: HANDLE , psirequested : *const u32 , psid : PSECURITY_DESCRIPTOR , nlength : u32 , lpnlengthneeded : *mut u32 ) -> super::Foundation:: BOOL );
    GetUserObjectSecurity(hobj.into_param().abi(), psirequested, psid, nlength, lpnlengthneeded)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetWindowsAccountDomainSid<P0>(psid: P0, pdomainsid: super::Foundation::PSID, cbdomainsid: *mut u32) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::Foundation::PSID>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn GetWindowsAccountDomainSid ( psid : super::Foundation:: PSID , pdomainsid : super::Foundation:: PSID , cbdomainsid : *mut u32 ) -> super::Foundation:: BOOL );
    GetWindowsAccountDomainSid(psid.into_param().abi(), pdomainsid, cbdomainsid)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImpersonateAnonymousToken<P0>(threadhandle: P0) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn ImpersonateAnonymousToken ( threadhandle : super::Foundation:: HANDLE ) -> super::Foundation:: BOOL );
    ImpersonateAnonymousToken(threadhandle.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImpersonateLoggedOnUser<P0>(htoken: P0) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn ImpersonateLoggedOnUser ( htoken : super::Foundation:: HANDLE ) -> super::Foundation:: BOOL );
    ImpersonateLoggedOnUser(htoken.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImpersonateSelf(impersonationlevel: SECURITY_IMPERSONATION_LEVEL) -> super::Foundation::BOOL {
    ::windows::imp::link ! ( "advapi32.dll""system" fn ImpersonateSelf ( impersonationlevel : SECURITY_IMPERSONATION_LEVEL ) -> super::Foundation:: BOOL );
    ImpersonateSelf(impersonationlevel)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InitializeAcl(pacl: *mut ACL, nacllength: u32, dwaclrevision: ACE_REVISION) -> super::Foundation::BOOL {
    ::windows::imp::link ! ( "advapi32.dll""system" fn InitializeAcl ( pacl : *mut ACL , nacllength : u32 , dwaclrevision : ACE_REVISION ) -> super::Foundation:: BOOL );
    InitializeAcl(pacl, nacllength, dwaclrevision)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InitializeSecurityDescriptor(psecuritydescriptor: PSECURITY_DESCRIPTOR, dwrevision: u32) -> super::Foundation::BOOL {
    ::windows::imp::link ! ( "advapi32.dll""system" fn InitializeSecurityDescriptor ( psecuritydescriptor : PSECURITY_DESCRIPTOR , dwrevision : u32 ) -> super::Foundation:: BOOL );
    InitializeSecurityDescriptor(psecuritydescriptor, dwrevision)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InitializeSid(sid: super::Foundation::PSID, pidentifierauthority: *const SID_IDENTIFIER_AUTHORITY, nsubauthoritycount: u8) -> super::Foundation::BOOL {
    ::windows::imp::link ! ( "advapi32.dll""system" fn InitializeSid ( sid : super::Foundation:: PSID , pidentifierauthority : *const SID_IDENTIFIER_AUTHORITY , nsubauthoritycount : u8 ) -> super::Foundation:: BOOL );
    InitializeSid(sid, pidentifierauthority, nsubauthoritycount)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsTokenRestricted<P0>(tokenhandle: P0) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn IsTokenRestricted ( tokenhandle : super::Foundation:: HANDLE ) -> super::Foundation:: BOOL );
    IsTokenRestricted(tokenhandle.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsValidAcl(pacl: *const ACL) -> super::Foundation::BOOL {
    ::windows::imp::link ! ( "advapi32.dll""system" fn IsValidAcl ( pacl : *const ACL ) -> super::Foundation:: BOOL );
    IsValidAcl(pacl)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsValidSecurityDescriptor<P0>(psecuritydescriptor: P0) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<PSECURITY_DESCRIPTOR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn IsValidSecurityDescriptor ( psecuritydescriptor : PSECURITY_DESCRIPTOR ) -> super::Foundation:: BOOL );
    IsValidSecurityDescriptor(psecuritydescriptor.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsValidSid<P0>(psid: P0) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::Foundation::PSID>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn IsValidSid ( psid : super::Foundation:: PSID ) -> super::Foundation:: BOOL );
    IsValidSid(psid.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsWellKnownSid<P0>(psid: P0, wellknownsidtype: WELL_KNOWN_SID_TYPE) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::Foundation::PSID>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn IsWellKnownSid ( psid : super::Foundation:: PSID , wellknownsidtype : WELL_KNOWN_SID_TYPE ) -> super::Foundation:: BOOL );
    IsWellKnownSid(psid.into_param().abi(), wellknownsidtype)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LogonUserA<P0, P1, P2>(lpszusername: P0, lpszdomain: P1, lpszpassword: P2, dwlogontype: LOGON32_LOGON, dwlogonprovider: LOGON32_PROVIDER, phtoken: *mut super::Foundation::HANDLE) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn LogonUserA ( lpszusername : :: windows::core::PCSTR , lpszdomain : :: windows::core::PCSTR , lpszpassword : :: windows::core::PCSTR , dwlogontype : LOGON32_LOGON , dwlogonprovider : LOGON32_PROVIDER , phtoken : *mut super::Foundation:: HANDLE ) -> super::Foundation:: BOOL );
    LogonUserA(lpszusername.into_param().abi(), lpszdomain.into_param().abi(), lpszpassword.into_param().abi(), dwlogontype, dwlogonprovider, phtoken)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LogonUserExA<P0, P1, P2>(lpszusername: P0, lpszdomain: P1, lpszpassword: P2, dwlogontype: LOGON32_LOGON, dwlogonprovider: LOGON32_PROVIDER, phtoken: ::core::option::Option<*mut super::Foundation::HANDLE>, pplogonsid: ::core::option::Option<*mut super::Foundation::PSID>, ppprofilebuffer: ::core::option::Option<*mut *mut ::core::ffi::c_void>, pdwprofilelength: ::core::option::Option<*mut u32>, pquotalimits: ::core::option::Option<*mut QUOTA_LIMITS>) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn LogonUserExA ( lpszusername : :: windows::core::PCSTR , lpszdomain : :: windows::core::PCSTR , lpszpassword : :: windows::core::PCSTR , dwlogontype : LOGON32_LOGON , dwlogonprovider : LOGON32_PROVIDER , phtoken : *mut super::Foundation:: HANDLE , pplogonsid : *mut super::Foundation:: PSID , ppprofilebuffer : *mut *mut ::core::ffi::c_void , pdwprofilelength : *mut u32 , pquotalimits : *mut QUOTA_LIMITS ) -> super::Foundation:: BOOL );
    LogonUserExA(
        lpszusername.into_param().abi(),
        lpszdomain.into_param().abi(),
        lpszpassword.into_param().abi(),
        dwlogontype,
        dwlogonprovider,
        ::core::mem::transmute(phtoken.unwrap_or(::std::ptr::null_mut())),
        ::core::mem::transmute(pplogonsid.unwrap_or(::std::ptr::null_mut())),
        ::core::mem::transmute(ppprofilebuffer.unwrap_or(::std::ptr::null_mut())),
        ::core::mem::transmute(pdwprofilelength.unwrap_or(::std::ptr::null_mut())),
        ::core::mem::transmute(pquotalimits.unwrap_or(::std::ptr::null_mut())),
    )
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LogonUserExW<P0, P1, P2>(lpszusername: P0, lpszdomain: P1, lpszpassword: P2, dwlogontype: LOGON32_LOGON, dwlogonprovider: LOGON32_PROVIDER, phtoken: ::core::option::Option<*mut super::Foundation::HANDLE>, pplogonsid: ::core::option::Option<*mut super::Foundation::PSID>, ppprofilebuffer: ::core::option::Option<*mut *mut ::core::ffi::c_void>, pdwprofilelength: ::core::option::Option<*mut u32>, pquotalimits: ::core::option::Option<*mut QUOTA_LIMITS>) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn LogonUserExW ( lpszusername : :: windows::core::PCWSTR , lpszdomain : :: windows::core::PCWSTR , lpszpassword : :: windows::core::PCWSTR , dwlogontype : LOGON32_LOGON , dwlogonprovider : LOGON32_PROVIDER , phtoken : *mut super::Foundation:: HANDLE , pplogonsid : *mut super::Foundation:: PSID , ppprofilebuffer : *mut *mut ::core::ffi::c_void , pdwprofilelength : *mut u32 , pquotalimits : *mut QUOTA_LIMITS ) -> super::Foundation:: BOOL );
    LogonUserExW(
        lpszusername.into_param().abi(),
        lpszdomain.into_param().abi(),
        lpszpassword.into_param().abi(),
        dwlogontype,
        dwlogonprovider,
        ::core::mem::transmute(phtoken.unwrap_or(::std::ptr::null_mut())),
        ::core::mem::transmute(pplogonsid.unwrap_or(::std::ptr::null_mut())),
        ::core::mem::transmute(ppprofilebuffer.unwrap_or(::std::ptr::null_mut())),
        ::core::mem::transmute(pdwprofilelength.unwrap_or(::std::ptr::null_mut())),
        ::core::mem::transmute(pquotalimits.unwrap_or(::std::ptr::null_mut())),
    )
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LogonUserW<P0, P1, P2>(lpszusername: P0, lpszdomain: P1, lpszpassword: P2, dwlogontype: LOGON32_LOGON, dwlogonprovider: LOGON32_PROVIDER, phtoken: *mut super::Foundation::HANDLE) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn LogonUserW ( lpszusername : :: windows::core::PCWSTR , lpszdomain : :: windows::core::PCWSTR , lpszpassword : :: windows::core::PCWSTR , dwlogontype : LOGON32_LOGON , dwlogonprovider : LOGON32_PROVIDER , phtoken : *mut super::Foundation:: HANDLE ) -> super::Foundation:: BOOL );
    LogonUserW(lpszusername.into_param().abi(), lpszdomain.into_param().abi(), lpszpassword.into_param().abi(), dwlogontype, dwlogonprovider, phtoken)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LookupAccountNameA<P0, P1>(lpsystemname: P0, lpaccountname: P1, sid: super::Foundation::PSID, cbsid: *mut u32, referenceddomainname: ::windows::core::PSTR, cchreferenceddomainname: *mut u32, peuse: *mut SID_NAME_USE) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn LookupAccountNameA ( lpsystemname : :: windows::core::PCSTR , lpaccountname : :: windows::core::PCSTR , sid : super::Foundation:: PSID , cbsid : *mut u32 , referenceddomainname : :: windows::core::PSTR , cchreferenceddomainname : *mut u32 , peuse : *mut SID_NAME_USE ) -> super::Foundation:: BOOL );
    LookupAccountNameA(lpsystemname.into_param().abi(), lpaccountname.into_param().abi(), sid, cbsid, ::core::mem::transmute(referenceddomainname), cchreferenceddomainname, peuse)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LookupAccountNameW<P0, P1>(lpsystemname: P0, lpaccountname: P1, sid: super::Foundation::PSID, cbsid: *mut u32, referenceddomainname: ::windows::core::PWSTR, cchreferenceddomainname: *mut u32, peuse: *mut SID_NAME_USE) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn LookupAccountNameW ( lpsystemname : :: windows::core::PCWSTR , lpaccountname : :: windows::core::PCWSTR , sid : super::Foundation:: PSID , cbsid : *mut u32 , referenceddomainname : :: windows::core::PWSTR , cchreferenceddomainname : *mut u32 , peuse : *mut SID_NAME_USE ) -> super::Foundation:: BOOL );
    LookupAccountNameW(lpsystemname.into_param().abi(), lpaccountname.into_param().abi(), sid, cbsid, ::core::mem::transmute(referenceddomainname), cchreferenceddomainname, peuse)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LookupAccountSidA<P0, P1>(lpsystemname: P0, sid: P1, name: ::windows::core::PSTR, cchname: *mut u32, referenceddomainname: ::windows::core::PSTR, cchreferenceddomainname: *mut u32, peuse: *mut SID_NAME_USE) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<super::Foundation::PSID>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn LookupAccountSidA ( lpsystemname : :: windows::core::PCSTR , sid : super::Foundation:: PSID , name : :: windows::core::PSTR , cchname : *mut u32 , referenceddomainname : :: windows::core::PSTR , cchreferenceddomainname : *mut u32 , peuse : *mut SID_NAME_USE ) -> super::Foundation:: BOOL );
    LookupAccountSidA(lpsystemname.into_param().abi(), sid.into_param().abi(), ::core::mem::transmute(name), cchname, ::core::mem::transmute(referenceddomainname), cchreferenceddomainname, peuse)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LookupAccountSidW<P0, P1>(lpsystemname: P0, sid: P1, name: ::windows::core::PWSTR, cchname: *mut u32, referenceddomainname: ::windows::core::PWSTR, cchreferenceddomainname: *mut u32, peuse: *mut SID_NAME_USE) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<super::Foundation::PSID>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn LookupAccountSidW ( lpsystemname : :: windows::core::PCWSTR , sid : super::Foundation:: PSID , name : :: windows::core::PWSTR , cchname : *mut u32 , referenceddomainname : :: windows::core::PWSTR , cchreferenceddomainname : *mut u32 , peuse : *mut SID_NAME_USE ) -> super::Foundation:: BOOL );
    LookupAccountSidW(lpsystemname.into_param().abi(), sid.into_param().abi(), ::core::mem::transmute(name), cchname, ::core::mem::transmute(referenceddomainname), cchreferenceddomainname, peuse)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LookupPrivilegeDisplayNameA<P0, P1>(lpsystemname: P0, lpname: P1, lpdisplayname: ::windows::core::PSTR, cchdisplayname: *mut u32, lplanguageid: *mut u32) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn LookupPrivilegeDisplayNameA ( lpsystemname : :: windows::core::PCSTR , lpname : :: windows::core::PCSTR , lpdisplayname : :: windows::core::PSTR , cchdisplayname : *mut u32 , lplanguageid : *mut u32 ) -> super::Foundation:: BOOL );
    LookupPrivilegeDisplayNameA(lpsystemname.into_param().abi(), lpname.into_param().abi(), ::core::mem::transmute(lpdisplayname), cchdisplayname, lplanguageid)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LookupPrivilegeDisplayNameW<P0, P1>(lpsystemname: P0, lpname: P1, lpdisplayname: ::windows::core::PWSTR, cchdisplayname: *mut u32, lplanguageid: *mut u32) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn LookupPrivilegeDisplayNameW ( lpsystemname : :: windows::core::PCWSTR , lpname : :: windows::core::PCWSTR , lpdisplayname : :: windows::core::PWSTR , cchdisplayname : *mut u32 , lplanguageid : *mut u32 ) -> super::Foundation:: BOOL );
    LookupPrivilegeDisplayNameW(lpsystemname.into_param().abi(), lpname.into_param().abi(), ::core::mem::transmute(lpdisplayname), cchdisplayname, lplanguageid)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LookupPrivilegeNameA<P0>(lpsystemname: P0, lpluid: *const super::Foundation::LUID, lpname: ::windows::core::PSTR, cchname: *mut u32) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn LookupPrivilegeNameA ( lpsystemname : :: windows::core::PCSTR , lpluid : *const super::Foundation:: LUID , lpname : :: windows::core::PSTR , cchname : *mut u32 ) -> super::Foundation:: BOOL );
    LookupPrivilegeNameA(lpsystemname.into_param().abi(), lpluid, ::core::mem::transmute(lpname), cchname)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LookupPrivilegeNameW<P0>(lpsystemname: P0, lpluid: *const super::Foundation::LUID, lpname: ::windows::core::PWSTR, cchname: *mut u32) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn LookupPrivilegeNameW ( lpsystemname : :: windows::core::PCWSTR , lpluid : *const super::Foundation:: LUID , lpname : :: windows::core::PWSTR , cchname : *mut u32 ) -> super::Foundation:: BOOL );
    LookupPrivilegeNameW(lpsystemname.into_param().abi(), lpluid, ::core::mem::transmute(lpname), cchname)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LookupPrivilegeValueA<P0, P1>(lpsystemname: P0, lpname: P1, lpluid: *mut super::Foundation::LUID) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn LookupPrivilegeValueA ( lpsystemname : :: windows::core::PCSTR , lpname : :: windows::core::PCSTR , lpluid : *mut super::Foundation:: LUID ) -> super::Foundation:: BOOL );
    LookupPrivilegeValueA(lpsystemname.into_param().abi(), lpname.into_param().abi(), lpluid)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LookupPrivilegeValueW<P0, P1>(lpsystemname: P0, lpname: P1, lpluid: *mut super::Foundation::LUID) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn LookupPrivilegeValueW ( lpsystemname : :: windows::core::PCWSTR , lpname : :: windows::core::PCWSTR , lpluid : *mut super::Foundation:: LUID ) -> super::Foundation:: BOOL );
    LookupPrivilegeValueW(lpsystemname.into_param().abi(), lpname.into_param().abi(), lpluid)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MakeAbsoluteSD<P0>(pselfrelativesecuritydescriptor: P0, pabsolutesecuritydescriptor: PSECURITY_DESCRIPTOR, lpdwabsolutesecuritydescriptorsize: *mut u32, pdacl: ::core::option::Option<*mut ACL>, lpdwdaclsize: *mut u32, psacl: ::core::option::Option<*mut ACL>, lpdwsaclsize: *mut u32, powner: super::Foundation::PSID, lpdwownersize: *mut u32, pprimarygroup: super::Foundation::PSID, lpdwprimarygroupsize: *mut u32) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<PSECURITY_DESCRIPTOR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn MakeAbsoluteSD ( pselfrelativesecuritydescriptor : PSECURITY_DESCRIPTOR , pabsolutesecuritydescriptor : PSECURITY_DESCRIPTOR , lpdwabsolutesecuritydescriptorsize : *mut u32 , pdacl : *mut ACL , lpdwdaclsize : *mut u32 , psacl : *mut ACL , lpdwsaclsize : *mut u32 , powner : super::Foundation:: PSID , lpdwownersize : *mut u32 , pprimarygroup : super::Foundation:: PSID , lpdwprimarygroupsize : *mut u32 ) -> super::Foundation:: BOOL );
    MakeAbsoluteSD(pselfrelativesecuritydescriptor.into_param().abi(), pabsolutesecuritydescriptor, lpdwabsolutesecuritydescriptorsize, ::core::mem::transmute(pdacl.unwrap_or(::std::ptr::null_mut())), lpdwdaclsize, ::core::mem::transmute(psacl.unwrap_or(::std::ptr::null_mut())), lpdwsaclsize, powner, lpdwownersize, pprimarygroup, lpdwprimarygroupsize)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MakeSelfRelativeSD<P0>(pabsolutesecuritydescriptor: P0, pselfrelativesecuritydescriptor: PSECURITY_DESCRIPTOR, lpdwbufferlength: *mut u32) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<PSECURITY_DESCRIPTOR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn MakeSelfRelativeSD ( pabsolutesecuritydescriptor : PSECURITY_DESCRIPTOR , pselfrelativesecuritydescriptor : PSECURITY_DESCRIPTOR , lpdwbufferlength : *mut u32 ) -> super::Foundation:: BOOL );
    MakeSelfRelativeSD(pabsolutesecuritydescriptor.into_param().abi(), pselfrelativesecuritydescriptor, lpdwbufferlength)
}
#[doc = "*Required features: `\"Win32_Security\"`*"]
#[inline]
pub unsafe fn MapGenericMask(accessmask: *mut u32, genericmapping: *const GENERIC_MAPPING) {
    ::windows::imp::link ! ( "advapi32.dll""system" fn MapGenericMask ( accessmask : *mut u32 , genericmapping : *const GENERIC_MAPPING ) -> ( ) );
    MapGenericMask(accessmask, genericmapping)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ObjectCloseAuditAlarmA<P0, P1>(subsystemname: P0, handleid: *const ::core::ffi::c_void, generateonclose: P1) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn ObjectCloseAuditAlarmA ( subsystemname : :: windows::core::PCSTR , handleid : *const ::core::ffi::c_void , generateonclose : super::Foundation:: BOOL ) -> super::Foundation:: BOOL );
    ObjectCloseAuditAlarmA(subsystemname.into_param().abi(), handleid, generateonclose.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ObjectCloseAuditAlarmW<P0, P1>(subsystemname: P0, handleid: *const ::core::ffi::c_void, generateonclose: P1) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn ObjectCloseAuditAlarmW ( subsystemname : :: windows::core::PCWSTR , handleid : *const ::core::ffi::c_void , generateonclose : super::Foundation:: BOOL ) -> super::Foundation:: BOOL );
    ObjectCloseAuditAlarmW(subsystemname.into_param().abi(), handleid, generateonclose.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ObjectDeleteAuditAlarmA<P0, P1>(subsystemname: P0, handleid: *const ::core::ffi::c_void, generateonclose: P1) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn ObjectDeleteAuditAlarmA ( subsystemname : :: windows::core::PCSTR , handleid : *const ::core::ffi::c_void , generateonclose : super::Foundation:: BOOL ) -> super::Foundation:: BOOL );
    ObjectDeleteAuditAlarmA(subsystemname.into_param().abi(), handleid, generateonclose.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ObjectDeleteAuditAlarmW<P0, P1>(subsystemname: P0, handleid: *const ::core::ffi::c_void, generateonclose: P1) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn ObjectDeleteAuditAlarmW ( subsystemname : :: windows::core::PCWSTR , handleid : *const ::core::ffi::c_void , generateonclose : super::Foundation:: BOOL ) -> super::Foundation:: BOOL );
    ObjectDeleteAuditAlarmW(subsystemname.into_param().abi(), handleid, generateonclose.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ObjectOpenAuditAlarmA<P0, P1, P2, P3, P4, P5, P6>(subsystemname: P0, handleid: *const ::core::ffi::c_void, objecttypename: P1, objectname: P2, psecuritydescriptor: P3, clienttoken: P4, desiredaccess: u32, grantedaccess: u32, privileges: ::core::option::Option<*const PRIVILEGE_SET>, objectcreation: P5, accessgranted: P6, generateonclose: *mut i32) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P3: ::windows::core::IntoParam<PSECURITY_DESCRIPTOR>,
    P4: ::windows::core::IntoParam<super::Foundation::HANDLE>,
    P5: ::windows::core::IntoParam<super::Foundation::BOOL>,
    P6: ::windows::core::IntoParam<super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn ObjectOpenAuditAlarmA ( subsystemname : :: windows::core::PCSTR , handleid : *const ::core::ffi::c_void , objecttypename : :: windows::core::PCSTR , objectname : :: windows::core::PCSTR , psecuritydescriptor : PSECURITY_DESCRIPTOR , clienttoken : super::Foundation:: HANDLE , desiredaccess : u32 , grantedaccess : u32 , privileges : *const PRIVILEGE_SET , objectcreation : super::Foundation:: BOOL , accessgranted : super::Foundation:: BOOL , generateonclose : *mut i32 ) -> super::Foundation:: BOOL );
    ObjectOpenAuditAlarmA(subsystemname.into_param().abi(), handleid, objecttypename.into_param().abi(), objectname.into_param().abi(), psecuritydescriptor.into_param().abi(), clienttoken.into_param().abi(), desiredaccess, grantedaccess, ::core::mem::transmute(privileges.unwrap_or(::std::ptr::null())), objectcreation.into_param().abi(), accessgranted.into_param().abi(), generateonclose)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ObjectOpenAuditAlarmW<P0, P1, P2, P3, P4, P5, P6>(subsystemname: P0, handleid: *const ::core::ffi::c_void, objecttypename: P1, objectname: P2, psecuritydescriptor: P3, clienttoken: P4, desiredaccess: u32, grantedaccess: u32, privileges: ::core::option::Option<*const PRIVILEGE_SET>, objectcreation: P5, accessgranted: P6, generateonclose: *mut i32) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P3: ::windows::core::IntoParam<PSECURITY_DESCRIPTOR>,
    P4: ::windows::core::IntoParam<super::Foundation::HANDLE>,
    P5: ::windows::core::IntoParam<super::Foundation::BOOL>,
    P6: ::windows::core::IntoParam<super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn ObjectOpenAuditAlarmW ( subsystemname : :: windows::core::PCWSTR , handleid : *const ::core::ffi::c_void , objecttypename : :: windows::core::PCWSTR , objectname : :: windows::core::PCWSTR , psecuritydescriptor : PSECURITY_DESCRIPTOR , clienttoken : super::Foundation:: HANDLE , desiredaccess : u32 , grantedaccess : u32 , privileges : *const PRIVILEGE_SET , objectcreation : super::Foundation:: BOOL , accessgranted : super::Foundation:: BOOL , generateonclose : *mut i32 ) -> super::Foundation:: BOOL );
    ObjectOpenAuditAlarmW(subsystemname.into_param().abi(), handleid, objecttypename.into_param().abi(), objectname.into_param().abi(), psecuritydescriptor.into_param().abi(), clienttoken.into_param().abi(), desiredaccess, grantedaccess, ::core::mem::transmute(privileges.unwrap_or(::std::ptr::null())), objectcreation.into_param().abi(), accessgranted.into_param().abi(), generateonclose)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ObjectPrivilegeAuditAlarmA<P0, P1, P2>(subsystemname: P0, handleid: *const ::core::ffi::c_void, clienttoken: P1, desiredaccess: u32, privileges: *const PRIVILEGE_SET, accessgranted: P2) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<super::Foundation::HANDLE>,
    P2: ::windows::core::IntoParam<super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn ObjectPrivilegeAuditAlarmA ( subsystemname : :: windows::core::PCSTR , handleid : *const ::core::ffi::c_void , clienttoken : super::Foundation:: HANDLE , desiredaccess : u32 , privileges : *const PRIVILEGE_SET , accessgranted : super::Foundation:: BOOL ) -> super::Foundation:: BOOL );
    ObjectPrivilegeAuditAlarmA(subsystemname.into_param().abi(), handleid, clienttoken.into_param().abi(), desiredaccess, privileges, accessgranted.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ObjectPrivilegeAuditAlarmW<P0, P1, P2>(subsystemname: P0, handleid: *const ::core::ffi::c_void, clienttoken: P1, desiredaccess: u32, privileges: *const PRIVILEGE_SET, accessgranted: P2) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<super::Foundation::HANDLE>,
    P2: ::windows::core::IntoParam<super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn ObjectPrivilegeAuditAlarmW ( subsystemname : :: windows::core::PCWSTR , handleid : *const ::core::ffi::c_void , clienttoken : super::Foundation:: HANDLE , desiredaccess : u32 , privileges : *const PRIVILEGE_SET , accessgranted : super::Foundation:: BOOL ) -> super::Foundation:: BOOL );
    ObjectPrivilegeAuditAlarmW(subsystemname.into_param().abi(), handleid, clienttoken.into_param().abi(), desiredaccess, privileges, accessgranted.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PrivilegeCheck<P0>(clienttoken: P0, requiredprivileges: *mut PRIVILEGE_SET, pfresult: *mut i32) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn PrivilegeCheck ( clienttoken : super::Foundation:: HANDLE , requiredprivileges : *mut PRIVILEGE_SET , pfresult : *mut i32 ) -> super::Foundation:: BOOL );
    PrivilegeCheck(clienttoken.into_param().abi(), requiredprivileges, pfresult)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PrivilegedServiceAuditAlarmA<P0, P1, P2, P3>(subsystemname: P0, servicename: P1, clienttoken: P2, privileges: *const PRIVILEGE_SET, accessgranted: P3) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<super::Foundation::HANDLE>,
    P3: ::windows::core::IntoParam<super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn PrivilegedServiceAuditAlarmA ( subsystemname : :: windows::core::PCSTR , servicename : :: windows::core::PCSTR , clienttoken : super::Foundation:: HANDLE , privileges : *const PRIVILEGE_SET , accessgranted : super::Foundation:: BOOL ) -> super::Foundation:: BOOL );
    PrivilegedServiceAuditAlarmA(subsystemname.into_param().abi(), servicename.into_param().abi(), clienttoken.into_param().abi(), privileges, accessgranted.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PrivilegedServiceAuditAlarmW<P0, P1, P2, P3>(subsystemname: P0, servicename: P1, clienttoken: P2, privileges: *const PRIVILEGE_SET, accessgranted: P3) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<super::Foundation::HANDLE>,
    P3: ::windows::core::IntoParam<super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn PrivilegedServiceAuditAlarmW ( subsystemname : :: windows::core::PCWSTR , servicename : :: windows::core::PCWSTR , clienttoken : super::Foundation:: HANDLE , privileges : *const PRIVILEGE_SET , accessgranted : super::Foundation:: BOOL ) -> super::Foundation:: BOOL );
    PrivilegedServiceAuditAlarmW(subsystemname.into_param().abi(), servicename.into_param().abi(), clienttoken.into_param().abi(), privileges, accessgranted.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security\"`*"]
#[inline]
pub unsafe fn QuerySecurityAccessMask(securityinformation: u32) -> u32 {
    ::windows::imp::link ! ( "advapi32.dll""system" fn QuerySecurityAccessMask ( securityinformation : u32 , desiredaccess : *mut u32 ) -> ( ) );
    let mut result__ = ::windows::core::zeroed::<u32>();
    QuerySecurityAccessMask(securityinformation, &mut result__);
    ::std::mem::transmute(result__)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RevertToSelf() -> super::Foundation::BOOL {
    ::windows::imp::link ! ( "advapi32.dll""system" fn RevertToSelf ( ) -> super::Foundation:: BOOL );
    RevertToSelf()
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtlConvertSidToUnicodeString<P0, P1>(unicodestring: *mut super::Foundation::UNICODE_STRING, sid: P0, allocatedestinationstring: P1) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::Foundation::PSID>,
    P1: ::windows::core::IntoParam<super::Foundation::BOOLEAN>,
{
    ::windows::imp::link ! ( "ntdll.dll""system" fn RtlConvertSidToUnicodeString ( unicodestring : *mut super::Foundation:: UNICODE_STRING , sid : super::Foundation:: PSID , allocatedestinationstring : super::Foundation:: BOOLEAN ) -> super::Foundation:: NTSTATUS );
    RtlConvertSidToUnicodeString(unicodestring, sid.into_param().abi(), allocatedestinationstring.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtlNormalizeSecurityDescriptor<P0>(securitydescriptor: *mut PSECURITY_DESCRIPTOR, securitydescriptorlength: u32, newsecuritydescriptor: ::core::option::Option<*mut PSECURITY_DESCRIPTOR>, newsecuritydescriptorlength: ::core::option::Option<*mut u32>, checkonly: P0) -> super::Foundation::BOOLEAN
where
    P0: ::windows::core::IntoParam<super::Foundation::BOOLEAN>,
{
    ::windows::imp::link ! ( "ntdll.dll""system" fn RtlNormalizeSecurityDescriptor ( securitydescriptor : *mut PSECURITY_DESCRIPTOR , securitydescriptorlength : u32 , newsecuritydescriptor : *mut PSECURITY_DESCRIPTOR , newsecuritydescriptorlength : *mut u32 , checkonly : super::Foundation:: BOOLEAN ) -> super::Foundation:: BOOLEAN );
    RtlNormalizeSecurityDescriptor(securitydescriptor, securitydescriptorlength, ::core::mem::transmute(newsecuritydescriptor.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(newsecuritydescriptorlength.unwrap_or(::std::ptr::null_mut())), checkonly.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetAclInformation(pacl: *mut ACL, paclinformation: *const ::core::ffi::c_void, naclinformationlength: u32, dwaclinformationclass: ACL_INFORMATION_CLASS) -> super::Foundation::BOOL {
    ::windows::imp::link ! ( "advapi32.dll""system" fn SetAclInformation ( pacl : *mut ACL , paclinformation : *const ::core::ffi::c_void , naclinformationlength : u32 , dwaclinformationclass : ACL_INFORMATION_CLASS ) -> super::Foundation:: BOOL );
    SetAclInformation(pacl, paclinformation, naclinformationlength, dwaclinformationclass)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetCachedSigningLevel<P0>(sourcefiles: &[super::Foundation::HANDLE], flags: u32, targetfile: P0) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn SetCachedSigningLevel ( sourcefiles : *const super::Foundation:: HANDLE , sourcefilecount : u32 , flags : u32 , targetfile : super::Foundation:: HANDLE ) -> super::Foundation:: BOOL );
    SetCachedSigningLevel(::core::mem::transmute(sourcefiles.as_ptr()), sourcefiles.len() as _, flags, targetfile.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetFileSecurityA<P0, P1>(lpfilename: P0, securityinformation: u32, psecuritydescriptor: P1) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<PSECURITY_DESCRIPTOR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn SetFileSecurityA ( lpfilename : :: windows::core::PCSTR , securityinformation : u32 , psecuritydescriptor : PSECURITY_DESCRIPTOR ) -> super::Foundation:: BOOL );
    SetFileSecurityA(lpfilename.into_param().abi(), securityinformation, psecuritydescriptor.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetFileSecurityW<P0, P1>(lpfilename: P0, securityinformation: u32, psecuritydescriptor: P1) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<PSECURITY_DESCRIPTOR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn SetFileSecurityW ( lpfilename : :: windows::core::PCWSTR , securityinformation : u32 , psecuritydescriptor : PSECURITY_DESCRIPTOR ) -> super::Foundation:: BOOL );
    SetFileSecurityW(lpfilename.into_param().abi(), securityinformation, psecuritydescriptor.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetKernelObjectSecurity<P0, P1>(handle: P0, securityinformation: u32, securitydescriptor: P1) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<PSECURITY_DESCRIPTOR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn SetKernelObjectSecurity ( handle : super::Foundation:: HANDLE , securityinformation : u32 , securitydescriptor : PSECURITY_DESCRIPTOR ) -> super::Foundation:: BOOL );
    SetKernelObjectSecurity(handle.into_param().abi(), securityinformation, securitydescriptor.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetPrivateObjectSecurity<P0, P1>(securityinformation: u32, modificationdescriptor: P0, objectssecuritydescriptor: *mut PSECURITY_DESCRIPTOR, genericmapping: *const GENERIC_MAPPING, token: P1) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<PSECURITY_DESCRIPTOR>,
    P1: ::windows::core::IntoParam<super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn SetPrivateObjectSecurity ( securityinformation : u32 , modificationdescriptor : PSECURITY_DESCRIPTOR , objectssecuritydescriptor : *mut PSECURITY_DESCRIPTOR , genericmapping : *const GENERIC_MAPPING , token : super::Foundation:: HANDLE ) -> super::Foundation:: BOOL );
    SetPrivateObjectSecurity(securityinformation, modificationdescriptor.into_param().abi(), objectssecuritydescriptor, genericmapping, token.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetPrivateObjectSecurityEx<P0, P1>(securityinformation: u32, modificationdescriptor: P0, objectssecuritydescriptor: *mut PSECURITY_DESCRIPTOR, autoinheritflags: SECURITY_AUTO_INHERIT_FLAGS, genericmapping: *const GENERIC_MAPPING, token: P1) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<PSECURITY_DESCRIPTOR>,
    P1: ::windows::core::IntoParam<super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn SetPrivateObjectSecurityEx ( securityinformation : u32 , modificationdescriptor : PSECURITY_DESCRIPTOR , objectssecuritydescriptor : *mut PSECURITY_DESCRIPTOR , autoinheritflags : SECURITY_AUTO_INHERIT_FLAGS , genericmapping : *const GENERIC_MAPPING , token : super::Foundation:: HANDLE ) -> super::Foundation:: BOOL );
    SetPrivateObjectSecurityEx(securityinformation, modificationdescriptor.into_param().abi(), objectssecuritydescriptor, autoinheritflags, genericmapping, token.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security\"`*"]
#[inline]
pub unsafe fn SetSecurityAccessMask(securityinformation: u32) -> u32 {
    ::windows::imp::link ! ( "advapi32.dll""system" fn SetSecurityAccessMask ( securityinformation : u32 , desiredaccess : *mut u32 ) -> ( ) );
    let mut result__ = ::windows::core::zeroed::<u32>();
    SetSecurityAccessMask(securityinformation, &mut result__);
    ::std::mem::transmute(result__)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetSecurityDescriptorControl<P0>(psecuritydescriptor: P0, controlbitsofinterest: SECURITY_DESCRIPTOR_CONTROL, controlbitstoset: SECURITY_DESCRIPTOR_CONTROL) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<PSECURITY_DESCRIPTOR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn SetSecurityDescriptorControl ( psecuritydescriptor : PSECURITY_DESCRIPTOR , controlbitsofinterest : SECURITY_DESCRIPTOR_CONTROL , controlbitstoset : SECURITY_DESCRIPTOR_CONTROL ) -> super::Foundation:: BOOL );
    SetSecurityDescriptorControl(psecuritydescriptor.into_param().abi(), controlbitsofinterest, controlbitstoset)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetSecurityDescriptorDacl<P0, P1>(psecuritydescriptor: PSECURITY_DESCRIPTOR, bdaclpresent: P0, pdacl: ::core::option::Option<*const ACL>, bdacldefaulted: P1) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::Foundation::BOOL>,
    P1: ::windows::core::IntoParam<super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn SetSecurityDescriptorDacl ( psecuritydescriptor : PSECURITY_DESCRIPTOR , bdaclpresent : super::Foundation:: BOOL , pdacl : *const ACL , bdacldefaulted : super::Foundation:: BOOL ) -> super::Foundation:: BOOL );
    SetSecurityDescriptorDacl(psecuritydescriptor, bdaclpresent.into_param().abi(), ::core::mem::transmute(pdacl.unwrap_or(::std::ptr::null())), bdacldefaulted.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetSecurityDescriptorGroup<P0, P1>(psecuritydescriptor: PSECURITY_DESCRIPTOR, pgroup: P0, bgroupdefaulted: P1) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::Foundation::PSID>,
    P1: ::windows::core::IntoParam<super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn SetSecurityDescriptorGroup ( psecuritydescriptor : PSECURITY_DESCRIPTOR , pgroup : super::Foundation:: PSID , bgroupdefaulted : super::Foundation:: BOOL ) -> super::Foundation:: BOOL );
    SetSecurityDescriptorGroup(psecuritydescriptor, pgroup.into_param().abi(), bgroupdefaulted.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetSecurityDescriptorOwner<P0, P1>(psecuritydescriptor: PSECURITY_DESCRIPTOR, powner: P0, bownerdefaulted: P1) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::Foundation::PSID>,
    P1: ::windows::core::IntoParam<super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn SetSecurityDescriptorOwner ( psecuritydescriptor : PSECURITY_DESCRIPTOR , powner : super::Foundation:: PSID , bownerdefaulted : super::Foundation:: BOOL ) -> super::Foundation:: BOOL );
    SetSecurityDescriptorOwner(psecuritydescriptor, powner.into_param().abi(), bownerdefaulted.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security\"`*"]
#[inline]
pub unsafe fn SetSecurityDescriptorRMControl(securitydescriptor: PSECURITY_DESCRIPTOR, rmcontrol: ::core::option::Option<*const u8>) -> u32 {
    ::windows::imp::link ! ( "advapi32.dll""system" fn SetSecurityDescriptorRMControl ( securitydescriptor : PSECURITY_DESCRIPTOR , rmcontrol : *const u8 ) -> u32 );
    SetSecurityDescriptorRMControl(securitydescriptor, ::core::mem::transmute(rmcontrol.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetSecurityDescriptorSacl<P0, P1>(psecuritydescriptor: PSECURITY_DESCRIPTOR, bsaclpresent: P0, psacl: ::core::option::Option<*const ACL>, bsacldefaulted: P1) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::Foundation::BOOL>,
    P1: ::windows::core::IntoParam<super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn SetSecurityDescriptorSacl ( psecuritydescriptor : PSECURITY_DESCRIPTOR , bsaclpresent : super::Foundation:: BOOL , psacl : *const ACL , bsacldefaulted : super::Foundation:: BOOL ) -> super::Foundation:: BOOL );
    SetSecurityDescriptorSacl(psecuritydescriptor, bsaclpresent.into_param().abi(), ::core::mem::transmute(psacl.unwrap_or(::std::ptr::null())), bsacldefaulted.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetTokenInformation<P0>(tokenhandle: P0, tokeninformationclass: TOKEN_INFORMATION_CLASS, tokeninformation: *const ::core::ffi::c_void, tokeninformationlength: u32) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn SetTokenInformation ( tokenhandle : super::Foundation:: HANDLE , tokeninformationclass : TOKEN_INFORMATION_CLASS , tokeninformation : *const ::core::ffi::c_void , tokeninformationlength : u32 ) -> super::Foundation:: BOOL );
    SetTokenInformation(tokenhandle.into_param().abi(), tokeninformationclass, tokeninformation, tokeninformationlength)
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetUserObjectSecurity<P0, P1>(hobj: P0, psirequested: *const OBJECT_SECURITY_INFORMATION, psid: P1) -> super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<PSECURITY_DESCRIPTOR>,
{
    ::windows::imp::link ! ( "user32.dll""system" fn SetUserObjectSecurity ( hobj : super::Foundation:: HANDLE , psirequested : *const OBJECT_SECURITY_INFORMATION , psid : PSECURITY_DESCRIPTOR ) -> super::Foundation:: BOOL );
    SetUserObjectSecurity(hobj.into_param().abi(), psirequested, psid.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const CVT_SECONDS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SECURITY_APP_PACKAGE_AUTHORITY: SID_IDENTIFIER_AUTHORITY = SID_IDENTIFIER_AUTHORITY { Value: [0, 0, 0, 0, 0, 15] };
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SECURITY_AUTHENTICATION_AUTHORITY: SID_IDENTIFIER_AUTHORITY = SID_IDENTIFIER_AUTHORITY { Value: [0, 0, 0, 0, 0, 18] };
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SECURITY_CREATOR_SID_AUTHORITY: SID_IDENTIFIER_AUTHORITY = SID_IDENTIFIER_AUTHORITY { Value: [0, 0, 0, 0, 0, 3] };
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const SECURITY_DYNAMIC_TRACKING: super::Foundation::BOOLEAN = super::Foundation::BOOLEAN(1u8);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SECURITY_LOCAL_SID_AUTHORITY: SID_IDENTIFIER_AUTHORITY = SID_IDENTIFIER_AUTHORITY { Value: [0, 0, 0, 0, 0, 2] };
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SECURITY_MANDATORY_LABEL_AUTHORITY: SID_IDENTIFIER_AUTHORITY = SID_IDENTIFIER_AUTHORITY { Value: [0, 0, 0, 0, 0, 16] };
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SECURITY_NON_UNIQUE_AUTHORITY: SID_IDENTIFIER_AUTHORITY = SID_IDENTIFIER_AUTHORITY { Value: [0, 0, 0, 0, 0, 4] };
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SECURITY_NT_AUTHORITY: SID_IDENTIFIER_AUTHORITY = SID_IDENTIFIER_AUTHORITY { Value: [0, 0, 0, 0, 0, 5] };
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SECURITY_NULL_SID_AUTHORITY: SID_IDENTIFIER_AUTHORITY = SID_IDENTIFIER_AUTHORITY { Value: [0, 0, 0, 0, 0, 0] };
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SECURITY_PROCESS_TRUST_AUTHORITY: SID_IDENTIFIER_AUTHORITY = SID_IDENTIFIER_AUTHORITY { Value: [0, 0, 0, 0, 0, 19] };
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SECURITY_RESOURCE_MANAGER_AUTHORITY: SID_IDENTIFIER_AUTHORITY = SID_IDENTIFIER_AUTHORITY { Value: [0, 0, 0, 0, 0, 9] };
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SECURITY_SCOPED_POLICY_ID_AUTHORITY: SID_IDENTIFIER_AUTHORITY = SID_IDENTIFIER_AUTHORITY { Value: [0, 0, 0, 0, 0, 17] };
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const SECURITY_STATIC_TRACKING: super::Foundation::BOOLEAN = super::Foundation::BOOLEAN(0u8);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SECURITY_WORLD_SID_AUTHORITY: SID_IDENTIFIER_AUTHORITY = SID_IDENTIFIER_AUTHORITY { Value: [0, 0, 0, 0, 0, 1] };
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SE_ASSIGNPRIMARYTOKEN_NAME: ::windows::core::PCWSTR = ::windows::w!("SeAssignPrimaryTokenPrivilege");
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SE_AUDIT_NAME: ::windows::core::PCWSTR = ::windows::w!("SeAuditPrivilege");
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SE_BACKUP_NAME: ::windows::core::PCWSTR = ::windows::w!("SeBackupPrivilege");
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SE_CHANGE_NOTIFY_NAME: ::windows::core::PCWSTR = ::windows::w!("SeChangeNotifyPrivilege");
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SE_CREATE_GLOBAL_NAME: ::windows::core::PCWSTR = ::windows::w!("SeCreateGlobalPrivilege");
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SE_CREATE_PAGEFILE_NAME: ::windows::core::PCWSTR = ::windows::w!("SeCreatePagefilePrivilege");
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SE_CREATE_PERMANENT_NAME: ::windows::core::PCWSTR = ::windows::w!("SeCreatePermanentPrivilege");
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SE_CREATE_SYMBOLIC_LINK_NAME: ::windows::core::PCWSTR = ::windows::w!("SeCreateSymbolicLinkPrivilege");
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SE_CREATE_TOKEN_NAME: ::windows::core::PCWSTR = ::windows::w!("SeCreateTokenPrivilege");
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SE_DEBUG_NAME: ::windows::core::PCWSTR = ::windows::w!("SeDebugPrivilege");
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SE_DELEGATE_SESSION_USER_IMPERSONATE_NAME: ::windows::core::PCWSTR = ::windows::w!("SeDelegateSessionUserImpersonatePrivilege");
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SE_ENABLE_DELEGATION_NAME: ::windows::core::PCWSTR = ::windows::w!("SeEnableDelegationPrivilege");
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SE_IMPERSONATE_NAME: ::windows::core::PCWSTR = ::windows::w!("SeImpersonatePrivilege");
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SE_INCREASE_QUOTA_NAME: ::windows::core::PCWSTR = ::windows::w!("SeIncreaseQuotaPrivilege");
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SE_INC_BASE_PRIORITY_NAME: ::windows::core::PCWSTR = ::windows::w!("SeIncreaseBasePriorityPrivilege");
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SE_INC_WORKING_SET_NAME: ::windows::core::PCWSTR = ::windows::w!("SeIncreaseWorkingSetPrivilege");
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SE_LOAD_DRIVER_NAME: ::windows::core::PCWSTR = ::windows::w!("SeLoadDriverPrivilege");
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SE_LOCK_MEMORY_NAME: ::windows::core::PCWSTR = ::windows::w!("SeLockMemoryPrivilege");
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SE_MACHINE_ACCOUNT_NAME: ::windows::core::PCWSTR = ::windows::w!("SeMachineAccountPrivilege");
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SE_MANAGE_VOLUME_NAME: ::windows::core::PCWSTR = ::windows::w!("SeManageVolumePrivilege");
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SE_PROF_SINGLE_PROCESS_NAME: ::windows::core::PCWSTR = ::windows::w!("SeProfileSingleProcessPrivilege");
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SE_RELABEL_NAME: ::windows::core::PCWSTR = ::windows::w!("SeRelabelPrivilege");
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SE_REMOTE_SHUTDOWN_NAME: ::windows::core::PCWSTR = ::windows::w!("SeRemoteShutdownPrivilege");
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SE_RESTORE_NAME: ::windows::core::PCWSTR = ::windows::w!("SeRestorePrivilege");
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SE_SECURITY_NAME: ::windows::core::PCWSTR = ::windows::w!("SeSecurityPrivilege");
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SE_SHUTDOWN_NAME: ::windows::core::PCWSTR = ::windows::w!("SeShutdownPrivilege");
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SE_SYNC_AGENT_NAME: ::windows::core::PCWSTR = ::windows::w!("SeSyncAgentPrivilege");
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SE_SYSTEMTIME_NAME: ::windows::core::PCWSTR = ::windows::w!("SeSystemtimePrivilege");
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SE_SYSTEM_ENVIRONMENT_NAME: ::windows::core::PCWSTR = ::windows::w!("SeSystemEnvironmentPrivilege");
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SE_SYSTEM_PROFILE_NAME: ::windows::core::PCWSTR = ::windows::w!("SeSystemProfilePrivilege");
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SE_TAKE_OWNERSHIP_NAME: ::windows::core::PCWSTR = ::windows::w!("SeTakeOwnershipPrivilege");
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SE_TCB_NAME: ::windows::core::PCWSTR = ::windows::w!("SeTcbPrivilege");
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SE_TIME_ZONE_NAME: ::windows::core::PCWSTR = ::windows::w!("SeTimeZonePrivilege");
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SE_TRUSTED_CREDMAN_ACCESS_NAME: ::windows::core::PCWSTR = ::windows::w!("SeTrustedCredManAccessPrivilege");
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SE_UNDOCK_NAME: ::windows::core::PCWSTR = ::windows::w!("SeUndockPrivilege");
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SE_UNSOLICITED_INPUT_NAME: ::windows::core::PCWSTR = ::windows::w!("SeUnsolicitedInputPrivilege");
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const cwcFILENAMESUFFIXMAX: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const cwcHRESULTSTRING: u32 = 40u32;
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const szLBRACE: ::windows::core::PCSTR = ::windows::s!("{");
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const szLPAREN: ::windows::core::PCSTR = ::windows::s!("(");
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const szRBRACE: ::windows::core::PCSTR = ::windows::s!("}");
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const szRPAREN: ::windows::core::PCSTR = ::windows::s!(")");
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const wszCERTENROLLSHAREPATH: ::windows::core::PCWSTR = ::windows::w!("CertSrv\\CertEnroll");
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const wszFCSAPARM_CERTFILENAMESUFFIX: ::windows::core::PCWSTR = ::windows::w!("%4");
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const wszFCSAPARM_CONFIGDN: ::windows::core::PCWSTR = ::windows::w!("%6");
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const wszFCSAPARM_CRLDELTAFILENAMESUFFIX: ::windows::core::PCWSTR = ::windows::w!("%9");
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const wszFCSAPARM_CRLFILENAMESUFFIX: ::windows::core::PCWSTR = ::windows::w!("%8");
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const wszFCSAPARM_DOMAINDN: ::windows::core::PCWSTR = ::windows::w!("%5");
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const wszFCSAPARM_DSCACERTATTRIBUTE: ::windows::core::PCWSTR = ::windows::w!("%11");
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const wszFCSAPARM_DSCRLATTRIBUTE: ::windows::core::PCWSTR = ::windows::w!("%10");
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const wszFCSAPARM_DSCROSSCERTPAIRATTRIBUTE: ::windows::core::PCWSTR = ::windows::w!("%14");
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const wszFCSAPARM_DSKRACERTATTRIBUTE: ::windows::core::PCWSTR = ::windows::w!("%13");
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const wszFCSAPARM_DSUSERCERTATTRIBUTE: ::windows::core::PCWSTR = ::windows::w!("%12");
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const wszFCSAPARM_SANITIZEDCANAME: ::windows::core::PCWSTR = ::windows::w!("%3");
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const wszFCSAPARM_SANITIZEDCANAMEHASH: ::windows::core::PCWSTR = ::windows::w!("%7");
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const wszFCSAPARM_SERVERDNSNAME: ::windows::core::PCWSTR = ::windows::w!("%1");
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const wszFCSAPARM_SERVERSHORTNAME: ::windows::core::PCWSTR = ::windows::w!("%2");
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const wszLBRACE: ::windows::core::PCWSTR = ::windows::w!("{");
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const wszLPAREN: ::windows::core::PCWSTR = ::windows::w!("(");
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const wszRBRACE: ::windows::core::PCWSTR = ::windows::w!("}");
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const wszRPAREN: ::windows::core::PCWSTR = ::windows::w!(")");
#[doc = "*Required features: `\"Win32_Security\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ACE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const CONTAINER_INHERIT_ACE: ACE_FLAGS = ACE_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const FAILED_ACCESS_ACE_FLAG: ACE_FLAGS = ACE_FLAGS(128u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const INHERIT_ONLY_ACE: ACE_FLAGS = ACE_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const INHERITED_ACE: ACE_FLAGS = ACE_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const NO_PROPAGATE_INHERIT_ACE: ACE_FLAGS = ACE_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const OBJECT_INHERIT_ACE: ACE_FLAGS = ACE_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SUCCESSFUL_ACCESS_ACE_FLAG: ACE_FLAGS = ACE_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SUB_CONTAINERS_AND_OBJECTS_INHERIT: ACE_FLAGS = ACE_FLAGS(3u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SUB_CONTAINERS_ONLY_INHERIT: ACE_FLAGS = ACE_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SUB_OBJECTS_ONLY_INHERIT: ACE_FLAGS = ACE_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const INHERIT_NO_PROPAGATE: ACE_FLAGS = ACE_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const INHERIT_ONLY: ACE_FLAGS = ACE_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const NO_INHERITANCE: ACE_FLAGS = ACE_FLAGS(0u32);
impl ::core::marker::Copy for ACE_FLAGS {}
impl ::core::clone::Clone for ACE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ACE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for ACE_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for ACE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ACE_FLAGS").field(&self.0).finish()
    }
}
impl ACE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for ACE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for ACE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for ACE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for ACE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for ACE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Security\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ACE_REVISION(pub u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const ACL_REVISION: ACE_REVISION = ACE_REVISION(2u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const ACL_REVISION_DS: ACE_REVISION = ACE_REVISION(4u32);
impl ::core::marker::Copy for ACE_REVISION {}
impl ::core::clone::Clone for ACE_REVISION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ACE_REVISION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for ACE_REVISION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for ACE_REVISION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ACE_REVISION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ACL_INFORMATION_CLASS(pub i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const AclRevisionInformation: ACL_INFORMATION_CLASS = ACL_INFORMATION_CLASS(1i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const AclSizeInformation: ACL_INFORMATION_CLASS = ACL_INFORMATION_CLASS(2i32);
impl ::core::marker::Copy for ACL_INFORMATION_CLASS {}
impl ::core::clone::Clone for ACL_INFORMATION_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ACL_INFORMATION_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for ACL_INFORMATION_CLASS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for ACL_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ACL_INFORMATION_CLASS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AUDIT_EVENT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const AuditEventObjectAccess: AUDIT_EVENT_TYPE = AUDIT_EVENT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const AuditEventDirectoryServiceAccess: AUDIT_EVENT_TYPE = AUDIT_EVENT_TYPE(1i32);
impl ::core::marker::Copy for AUDIT_EVENT_TYPE {}
impl ::core::clone::Clone for AUDIT_EVENT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AUDIT_EVENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for AUDIT_EVENT_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for AUDIT_EVENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUDIT_EVENT_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLAIM_SECURITY_ATTRIBUTE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const CLAIM_SECURITY_ATTRIBUTE_NON_INHERITABLE: CLAIM_SECURITY_ATTRIBUTE_FLAGS = CLAIM_SECURITY_ATTRIBUTE_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const CLAIM_SECURITY_ATTRIBUTE_VALUE_CASE_SENSITIVE: CLAIM_SECURITY_ATTRIBUTE_FLAGS = CLAIM_SECURITY_ATTRIBUTE_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const CLAIM_SECURITY_ATTRIBUTE_USE_FOR_DENY_ONLY: CLAIM_SECURITY_ATTRIBUTE_FLAGS = CLAIM_SECURITY_ATTRIBUTE_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const CLAIM_SECURITY_ATTRIBUTE_DISABLED_BY_DEFAULT: CLAIM_SECURITY_ATTRIBUTE_FLAGS = CLAIM_SECURITY_ATTRIBUTE_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const CLAIM_SECURITY_ATTRIBUTE_DISABLED: CLAIM_SECURITY_ATTRIBUTE_FLAGS = CLAIM_SECURITY_ATTRIBUTE_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const CLAIM_SECURITY_ATTRIBUTE_MANDATORY: CLAIM_SECURITY_ATTRIBUTE_FLAGS = CLAIM_SECURITY_ATTRIBUTE_FLAGS(32u32);
impl ::core::marker::Copy for CLAIM_SECURITY_ATTRIBUTE_FLAGS {}
impl ::core::clone::Clone for CLAIM_SECURITY_ATTRIBUTE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CLAIM_SECURITY_ATTRIBUTE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CLAIM_SECURITY_ATTRIBUTE_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CLAIM_SECURITY_ATTRIBUTE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLAIM_SECURITY_ATTRIBUTE_FLAGS").field(&self.0).finish()
    }
}
impl CLAIM_SECURITY_ATTRIBUTE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for CLAIM_SECURITY_ATTRIBUTE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CLAIM_SECURITY_ATTRIBUTE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CLAIM_SECURITY_ATTRIBUTE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CLAIM_SECURITY_ATTRIBUTE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CLAIM_SECURITY_ATTRIBUTE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Security\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE(pub u16);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_INT64: CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE = CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE(1u16);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_UINT64: CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE = CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE(2u16);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_STRING: CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE = CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE(3u16);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_OCTET_STRING: CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE = CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE(16u16);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_FQBN: CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE = CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE(4u16);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_SID: CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE = CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE(5u16);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_BOOLEAN: CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE = CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE(6u16);
impl ::core::marker::Copy for CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE {}
impl ::core::clone::Clone for CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CREATE_RESTRICTED_TOKEN_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const DISABLE_MAX_PRIVILEGE: CREATE_RESTRICTED_TOKEN_FLAGS = CREATE_RESTRICTED_TOKEN_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SANDBOX_INERT: CREATE_RESTRICTED_TOKEN_FLAGS = CREATE_RESTRICTED_TOKEN_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const LUA_TOKEN: CREATE_RESTRICTED_TOKEN_FLAGS = CREATE_RESTRICTED_TOKEN_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WRITE_RESTRICTED: CREATE_RESTRICTED_TOKEN_FLAGS = CREATE_RESTRICTED_TOKEN_FLAGS(8u32);
impl ::core::marker::Copy for CREATE_RESTRICTED_TOKEN_FLAGS {}
impl ::core::clone::Clone for CREATE_RESTRICTED_TOKEN_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CREATE_RESTRICTED_TOKEN_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CREATE_RESTRICTED_TOKEN_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CREATE_RESTRICTED_TOKEN_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CREATE_RESTRICTED_TOKEN_FLAGS").field(&self.0).finish()
    }
}
impl CREATE_RESTRICTED_TOKEN_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for CREATE_RESTRICTED_TOKEN_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CREATE_RESTRICTED_TOKEN_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CREATE_RESTRICTED_TOKEN_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CREATE_RESTRICTED_TOKEN_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CREATE_RESTRICTED_TOKEN_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Security\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ENUM_PERIOD(pub i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const ENUM_PERIOD_INVALID: ENUM_PERIOD = ENUM_PERIOD(-1i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const ENUM_PERIOD_SECONDS: ENUM_PERIOD = ENUM_PERIOD(0i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const ENUM_PERIOD_MINUTES: ENUM_PERIOD = ENUM_PERIOD(1i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const ENUM_PERIOD_HOURS: ENUM_PERIOD = ENUM_PERIOD(2i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const ENUM_PERIOD_DAYS: ENUM_PERIOD = ENUM_PERIOD(3i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const ENUM_PERIOD_WEEKS: ENUM_PERIOD = ENUM_PERIOD(4i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const ENUM_PERIOD_MONTHS: ENUM_PERIOD = ENUM_PERIOD(5i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const ENUM_PERIOD_YEARS: ENUM_PERIOD = ENUM_PERIOD(6i32);
impl ::core::marker::Copy for ENUM_PERIOD {}
impl ::core::clone::Clone for ENUM_PERIOD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ENUM_PERIOD {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for ENUM_PERIOD {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for ENUM_PERIOD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ENUM_PERIOD").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LOGON32_LOGON(pub u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const LOGON32_LOGON_BATCH: LOGON32_LOGON = LOGON32_LOGON(4u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const LOGON32_LOGON_INTERACTIVE: LOGON32_LOGON = LOGON32_LOGON(2u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const LOGON32_LOGON_NETWORK: LOGON32_LOGON = LOGON32_LOGON(3u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const LOGON32_LOGON_NETWORK_CLEARTEXT: LOGON32_LOGON = LOGON32_LOGON(8u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const LOGON32_LOGON_NEW_CREDENTIALS: LOGON32_LOGON = LOGON32_LOGON(9u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const LOGON32_LOGON_SERVICE: LOGON32_LOGON = LOGON32_LOGON(5u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const LOGON32_LOGON_UNLOCK: LOGON32_LOGON = LOGON32_LOGON(7u32);
impl ::core::marker::Copy for LOGON32_LOGON {}
impl ::core::clone::Clone for LOGON32_LOGON {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LOGON32_LOGON {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for LOGON32_LOGON {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for LOGON32_LOGON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LOGON32_LOGON").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LOGON32_PROVIDER(pub u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const LOGON32_PROVIDER_DEFAULT: LOGON32_PROVIDER = LOGON32_PROVIDER(0u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const LOGON32_PROVIDER_WINNT50: LOGON32_PROVIDER = LOGON32_PROVIDER(3u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const LOGON32_PROVIDER_WINNT40: LOGON32_PROVIDER = LOGON32_PROVIDER(2u32);
impl ::core::marker::Copy for LOGON32_PROVIDER {}
impl ::core::clone::Clone for LOGON32_PROVIDER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LOGON32_PROVIDER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for LOGON32_PROVIDER {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for LOGON32_PROVIDER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LOGON32_PROVIDER").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MANDATORY_LEVEL(pub i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const MandatoryLevelUntrusted: MANDATORY_LEVEL = MANDATORY_LEVEL(0i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const MandatoryLevelLow: MANDATORY_LEVEL = MANDATORY_LEVEL(1i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const MandatoryLevelMedium: MANDATORY_LEVEL = MANDATORY_LEVEL(2i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const MandatoryLevelHigh: MANDATORY_LEVEL = MANDATORY_LEVEL(3i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const MandatoryLevelSystem: MANDATORY_LEVEL = MANDATORY_LEVEL(4i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const MandatoryLevelSecureProcess: MANDATORY_LEVEL = MANDATORY_LEVEL(5i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const MandatoryLevelCount: MANDATORY_LEVEL = MANDATORY_LEVEL(6i32);
impl ::core::marker::Copy for MANDATORY_LEVEL {}
impl ::core::clone::Clone for MANDATORY_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MANDATORY_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MANDATORY_LEVEL {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MANDATORY_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MANDATORY_LEVEL").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OBJECT_SECURITY_INFORMATION(pub u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const ATTRIBUTE_SECURITY_INFORMATION: OBJECT_SECURITY_INFORMATION = OBJECT_SECURITY_INFORMATION(32u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const BACKUP_SECURITY_INFORMATION: OBJECT_SECURITY_INFORMATION = OBJECT_SECURITY_INFORMATION(65536u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const DACL_SECURITY_INFORMATION: OBJECT_SECURITY_INFORMATION = OBJECT_SECURITY_INFORMATION(4u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const GROUP_SECURITY_INFORMATION: OBJECT_SECURITY_INFORMATION = OBJECT_SECURITY_INFORMATION(2u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const LABEL_SECURITY_INFORMATION: OBJECT_SECURITY_INFORMATION = OBJECT_SECURITY_INFORMATION(16u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const OWNER_SECURITY_INFORMATION: OBJECT_SECURITY_INFORMATION = OBJECT_SECURITY_INFORMATION(1u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const PROTECTED_DACL_SECURITY_INFORMATION: OBJECT_SECURITY_INFORMATION = OBJECT_SECURITY_INFORMATION(2147483648u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const PROTECTED_SACL_SECURITY_INFORMATION: OBJECT_SECURITY_INFORMATION = OBJECT_SECURITY_INFORMATION(1073741824u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SACL_SECURITY_INFORMATION: OBJECT_SECURITY_INFORMATION = OBJECT_SECURITY_INFORMATION(8u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SCOPE_SECURITY_INFORMATION: OBJECT_SECURITY_INFORMATION = OBJECT_SECURITY_INFORMATION(64u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const UNPROTECTED_DACL_SECURITY_INFORMATION: OBJECT_SECURITY_INFORMATION = OBJECT_SECURITY_INFORMATION(536870912u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const UNPROTECTED_SACL_SECURITY_INFORMATION: OBJECT_SECURITY_INFORMATION = OBJECT_SECURITY_INFORMATION(268435456u32);
impl ::core::marker::Copy for OBJECT_SECURITY_INFORMATION {}
impl ::core::clone::Clone for OBJECT_SECURITY_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OBJECT_SECURITY_INFORMATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for OBJECT_SECURITY_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for OBJECT_SECURITY_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OBJECT_SECURITY_INFORMATION").field(&self.0).finish()
    }
}
impl OBJECT_SECURITY_INFORMATION {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for OBJECT_SECURITY_INFORMATION {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for OBJECT_SECURITY_INFORMATION {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for OBJECT_SECURITY_INFORMATION {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for OBJECT_SECURITY_INFORMATION {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for OBJECT_SECURITY_INFORMATION {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Security\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SECURITY_AUTO_INHERIT_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SEF_AVOID_OWNER_CHECK: SECURITY_AUTO_INHERIT_FLAGS = SECURITY_AUTO_INHERIT_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SEF_AVOID_OWNER_RESTRICTION: SECURITY_AUTO_INHERIT_FLAGS = SECURITY_AUTO_INHERIT_FLAGS(4096u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SEF_AVOID_PRIVILEGE_CHECK: SECURITY_AUTO_INHERIT_FLAGS = SECURITY_AUTO_INHERIT_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SEF_DACL_AUTO_INHERIT: SECURITY_AUTO_INHERIT_FLAGS = SECURITY_AUTO_INHERIT_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SEF_DEFAULT_DESCRIPTOR_FOR_OBJECT: SECURITY_AUTO_INHERIT_FLAGS = SECURITY_AUTO_INHERIT_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SEF_DEFAULT_GROUP_FROM_PARENT: SECURITY_AUTO_INHERIT_FLAGS = SECURITY_AUTO_INHERIT_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SEF_DEFAULT_OWNER_FROM_PARENT: SECURITY_AUTO_INHERIT_FLAGS = SECURITY_AUTO_INHERIT_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SEF_MACL_NO_EXECUTE_UP: SECURITY_AUTO_INHERIT_FLAGS = SECURITY_AUTO_INHERIT_FLAGS(1024u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SEF_MACL_NO_READ_UP: SECURITY_AUTO_INHERIT_FLAGS = SECURITY_AUTO_INHERIT_FLAGS(512u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SEF_MACL_NO_WRITE_UP: SECURITY_AUTO_INHERIT_FLAGS = SECURITY_AUTO_INHERIT_FLAGS(256u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SEF_SACL_AUTO_INHERIT: SECURITY_AUTO_INHERIT_FLAGS = SECURITY_AUTO_INHERIT_FLAGS(2u32);
impl ::core::marker::Copy for SECURITY_AUTO_INHERIT_FLAGS {}
impl ::core::clone::Clone for SECURITY_AUTO_INHERIT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SECURITY_AUTO_INHERIT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SECURITY_AUTO_INHERIT_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SECURITY_AUTO_INHERIT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SECURITY_AUTO_INHERIT_FLAGS").field(&self.0).finish()
    }
}
impl SECURITY_AUTO_INHERIT_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for SECURITY_AUTO_INHERIT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SECURITY_AUTO_INHERIT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SECURITY_AUTO_INHERIT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SECURITY_AUTO_INHERIT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SECURITY_AUTO_INHERIT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Security\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SECURITY_DESCRIPTOR_CONTROL(pub u16);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SE_OWNER_DEFAULTED: SECURITY_DESCRIPTOR_CONTROL = SECURITY_DESCRIPTOR_CONTROL(1u16);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SE_GROUP_DEFAULTED: SECURITY_DESCRIPTOR_CONTROL = SECURITY_DESCRIPTOR_CONTROL(2u16);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SE_DACL_PRESENT: SECURITY_DESCRIPTOR_CONTROL = SECURITY_DESCRIPTOR_CONTROL(4u16);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SE_DACL_DEFAULTED: SECURITY_DESCRIPTOR_CONTROL = SECURITY_DESCRIPTOR_CONTROL(8u16);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SE_SACL_PRESENT: SECURITY_DESCRIPTOR_CONTROL = SECURITY_DESCRIPTOR_CONTROL(16u16);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SE_SACL_DEFAULTED: SECURITY_DESCRIPTOR_CONTROL = SECURITY_DESCRIPTOR_CONTROL(32u16);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SE_DACL_AUTO_INHERIT_REQ: SECURITY_DESCRIPTOR_CONTROL = SECURITY_DESCRIPTOR_CONTROL(256u16);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SE_SACL_AUTO_INHERIT_REQ: SECURITY_DESCRIPTOR_CONTROL = SECURITY_DESCRIPTOR_CONTROL(512u16);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SE_DACL_AUTO_INHERITED: SECURITY_DESCRIPTOR_CONTROL = SECURITY_DESCRIPTOR_CONTROL(1024u16);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SE_SACL_AUTO_INHERITED: SECURITY_DESCRIPTOR_CONTROL = SECURITY_DESCRIPTOR_CONTROL(2048u16);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SE_DACL_PROTECTED: SECURITY_DESCRIPTOR_CONTROL = SECURITY_DESCRIPTOR_CONTROL(4096u16);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SE_SACL_PROTECTED: SECURITY_DESCRIPTOR_CONTROL = SECURITY_DESCRIPTOR_CONTROL(8192u16);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SE_RM_CONTROL_VALID: SECURITY_DESCRIPTOR_CONTROL = SECURITY_DESCRIPTOR_CONTROL(16384u16);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SE_SELF_RELATIVE: SECURITY_DESCRIPTOR_CONTROL = SECURITY_DESCRIPTOR_CONTROL(32768u16);
impl ::core::marker::Copy for SECURITY_DESCRIPTOR_CONTROL {}
impl ::core::clone::Clone for SECURITY_DESCRIPTOR_CONTROL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SECURITY_DESCRIPTOR_CONTROL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SECURITY_DESCRIPTOR_CONTROL {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SECURITY_DESCRIPTOR_CONTROL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SECURITY_DESCRIPTOR_CONTROL").field(&self.0).finish()
    }
}
impl SECURITY_DESCRIPTOR_CONTROL {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for SECURITY_DESCRIPTOR_CONTROL {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SECURITY_DESCRIPTOR_CONTROL {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SECURITY_DESCRIPTOR_CONTROL {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SECURITY_DESCRIPTOR_CONTROL {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SECURITY_DESCRIPTOR_CONTROL {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Security\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SECURITY_IMPERSONATION_LEVEL(pub i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SecurityAnonymous: SECURITY_IMPERSONATION_LEVEL = SECURITY_IMPERSONATION_LEVEL(0i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SecurityIdentification: SECURITY_IMPERSONATION_LEVEL = SECURITY_IMPERSONATION_LEVEL(1i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SecurityImpersonation: SECURITY_IMPERSONATION_LEVEL = SECURITY_IMPERSONATION_LEVEL(2i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SecurityDelegation: SECURITY_IMPERSONATION_LEVEL = SECURITY_IMPERSONATION_LEVEL(3i32);
impl ::core::marker::Copy for SECURITY_IMPERSONATION_LEVEL {}
impl ::core::clone::Clone for SECURITY_IMPERSONATION_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SECURITY_IMPERSONATION_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SECURITY_IMPERSONATION_LEVEL {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SECURITY_IMPERSONATION_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SECURITY_IMPERSONATION_LEVEL").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SID_NAME_USE(pub i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SidTypeUser: SID_NAME_USE = SID_NAME_USE(1i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SidTypeGroup: SID_NAME_USE = SID_NAME_USE(2i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SidTypeDomain: SID_NAME_USE = SID_NAME_USE(3i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SidTypeAlias: SID_NAME_USE = SID_NAME_USE(4i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SidTypeWellKnownGroup: SID_NAME_USE = SID_NAME_USE(5i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SidTypeDeletedAccount: SID_NAME_USE = SID_NAME_USE(6i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SidTypeInvalid: SID_NAME_USE = SID_NAME_USE(7i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SidTypeUnknown: SID_NAME_USE = SID_NAME_USE(8i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SidTypeComputer: SID_NAME_USE = SID_NAME_USE(9i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SidTypeLabel: SID_NAME_USE = SID_NAME_USE(10i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SidTypeLogonSession: SID_NAME_USE = SID_NAME_USE(11i32);
impl ::core::marker::Copy for SID_NAME_USE {}
impl ::core::clone::Clone for SID_NAME_USE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SID_NAME_USE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SID_NAME_USE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SID_NAME_USE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SID_NAME_USE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SYSTEM_AUDIT_OBJECT_ACE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const ACE_OBJECT_TYPE_PRESENT: SYSTEM_AUDIT_OBJECT_ACE_FLAGS = SYSTEM_AUDIT_OBJECT_ACE_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const ACE_INHERITED_OBJECT_TYPE_PRESENT: SYSTEM_AUDIT_OBJECT_ACE_FLAGS = SYSTEM_AUDIT_OBJECT_ACE_FLAGS(2u32);
impl ::core::marker::Copy for SYSTEM_AUDIT_OBJECT_ACE_FLAGS {}
impl ::core::clone::Clone for SYSTEM_AUDIT_OBJECT_ACE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SYSTEM_AUDIT_OBJECT_ACE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SYSTEM_AUDIT_OBJECT_ACE_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SYSTEM_AUDIT_OBJECT_ACE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYSTEM_AUDIT_OBJECT_ACE_FLAGS").field(&self.0).finish()
    }
}
impl SYSTEM_AUDIT_OBJECT_ACE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for SYSTEM_AUDIT_OBJECT_ACE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SYSTEM_AUDIT_OBJECT_ACE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SYSTEM_AUDIT_OBJECT_ACE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SYSTEM_AUDIT_OBJECT_ACE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SYSTEM_AUDIT_OBJECT_ACE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Security\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TOKEN_ACCESS_MASK(pub u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TOKEN_DELETE: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(65536u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TOKEN_READ_CONTROL: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(131072u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TOKEN_WRITE_DAC: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(262144u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TOKEN_WRITE_OWNER: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(524288u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TOKEN_ACCESS_SYSTEM_SECURITY: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(16777216u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TOKEN_ASSIGN_PRIMARY: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(1u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TOKEN_DUPLICATE: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(2u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TOKEN_IMPERSONATE: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(4u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TOKEN_QUERY: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(8u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TOKEN_QUERY_SOURCE: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(16u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TOKEN_ADJUST_PRIVILEGES: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(32u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TOKEN_ADJUST_GROUPS: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(64u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TOKEN_ADJUST_DEFAULT: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(128u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TOKEN_ADJUST_SESSIONID: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(256u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TOKEN_READ: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(131080u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TOKEN_WRITE: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(131296u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TOKEN_EXECUTE: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(131072u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TOKEN_TRUST_CONSTRAINT_MASK: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(131096u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TOKEN_ACCESS_PSEUDO_HANDLE_WIN8: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(24u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TOKEN_ACCESS_PSEUDO_HANDLE: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(24u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TOKEN_ALL_ACCESS: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(983551u32);
impl ::core::marker::Copy for TOKEN_ACCESS_MASK {}
impl ::core::clone::Clone for TOKEN_ACCESS_MASK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TOKEN_ACCESS_MASK {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for TOKEN_ACCESS_MASK {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for TOKEN_ACCESS_MASK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TOKEN_ACCESS_MASK").field(&self.0).finish()
    }
}
impl TOKEN_ACCESS_MASK {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for TOKEN_ACCESS_MASK {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for TOKEN_ACCESS_MASK {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for TOKEN_ACCESS_MASK {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for TOKEN_ACCESS_MASK {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for TOKEN_ACCESS_MASK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Security\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TOKEN_ELEVATION_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TokenElevationTypeDefault: TOKEN_ELEVATION_TYPE = TOKEN_ELEVATION_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TokenElevationTypeFull: TOKEN_ELEVATION_TYPE = TOKEN_ELEVATION_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TokenElevationTypeLimited: TOKEN_ELEVATION_TYPE = TOKEN_ELEVATION_TYPE(3i32);
impl ::core::marker::Copy for TOKEN_ELEVATION_TYPE {}
impl ::core::clone::Clone for TOKEN_ELEVATION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TOKEN_ELEVATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for TOKEN_ELEVATION_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for TOKEN_ELEVATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TOKEN_ELEVATION_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TOKEN_INFORMATION_CLASS(pub i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TokenUser: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(1i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TokenGroups: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(2i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TokenPrivileges: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(3i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TokenOwner: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(4i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TokenPrimaryGroup: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(5i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TokenDefaultDacl: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(6i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TokenSource: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(7i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TokenType: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(8i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TokenImpersonationLevel: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(9i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TokenStatistics: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(10i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TokenRestrictedSids: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(11i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TokenSessionId: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(12i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TokenGroupsAndPrivileges: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(13i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TokenSessionReference: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(14i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TokenSandBoxInert: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(15i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TokenAuditPolicy: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(16i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TokenOrigin: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(17i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TokenElevationType: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(18i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TokenLinkedToken: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(19i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TokenElevation: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(20i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TokenHasRestrictions: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(21i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TokenAccessInformation: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(22i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TokenVirtualizationAllowed: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(23i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TokenVirtualizationEnabled: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(24i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TokenIntegrityLevel: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(25i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TokenUIAccess: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(26i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TokenMandatoryPolicy: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(27i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TokenLogonSid: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(28i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TokenIsAppContainer: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(29i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TokenCapabilities: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(30i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TokenAppContainerSid: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(31i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TokenAppContainerNumber: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(32i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TokenUserClaimAttributes: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(33i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TokenDeviceClaimAttributes: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(34i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TokenRestrictedUserClaimAttributes: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(35i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TokenRestrictedDeviceClaimAttributes: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(36i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TokenDeviceGroups: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(37i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TokenRestrictedDeviceGroups: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(38i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TokenSecurityAttributes: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(39i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TokenIsRestricted: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(40i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TokenProcessTrustLevel: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(41i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TokenPrivateNameSpace: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(42i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TokenSingletonAttributes: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(43i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TokenBnoIsolation: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(44i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TokenChildProcessFlags: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(45i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TokenIsLessPrivilegedAppContainer: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(46i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TokenIsSandboxed: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(47i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const MaxTokenInfoClass: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(48i32);
impl ::core::marker::Copy for TOKEN_INFORMATION_CLASS {}
impl ::core::clone::Clone for TOKEN_INFORMATION_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TOKEN_INFORMATION_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for TOKEN_INFORMATION_CLASS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for TOKEN_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TOKEN_INFORMATION_CLASS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TOKEN_MANDATORY_POLICY_ID(pub u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TOKEN_MANDATORY_POLICY_OFF: TOKEN_MANDATORY_POLICY_ID = TOKEN_MANDATORY_POLICY_ID(0u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TOKEN_MANDATORY_POLICY_NO_WRITE_UP: TOKEN_MANDATORY_POLICY_ID = TOKEN_MANDATORY_POLICY_ID(1u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TOKEN_MANDATORY_POLICY_NEW_PROCESS_MIN: TOKEN_MANDATORY_POLICY_ID = TOKEN_MANDATORY_POLICY_ID(2u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TOKEN_MANDATORY_POLICY_VALID_MASK: TOKEN_MANDATORY_POLICY_ID = TOKEN_MANDATORY_POLICY_ID(3u32);
impl ::core::marker::Copy for TOKEN_MANDATORY_POLICY_ID {}
impl ::core::clone::Clone for TOKEN_MANDATORY_POLICY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TOKEN_MANDATORY_POLICY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for TOKEN_MANDATORY_POLICY_ID {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for TOKEN_MANDATORY_POLICY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TOKEN_MANDATORY_POLICY_ID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TOKEN_PRIVILEGES_ATTRIBUTES(pub u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SE_PRIVILEGE_ENABLED: TOKEN_PRIVILEGES_ATTRIBUTES = TOKEN_PRIVILEGES_ATTRIBUTES(2u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SE_PRIVILEGE_ENABLED_BY_DEFAULT: TOKEN_PRIVILEGES_ATTRIBUTES = TOKEN_PRIVILEGES_ATTRIBUTES(1u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SE_PRIVILEGE_REMOVED: TOKEN_PRIVILEGES_ATTRIBUTES = TOKEN_PRIVILEGES_ATTRIBUTES(4u32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const SE_PRIVILEGE_USED_FOR_ACCESS: TOKEN_PRIVILEGES_ATTRIBUTES = TOKEN_PRIVILEGES_ATTRIBUTES(2147483648u32);
impl ::core::marker::Copy for TOKEN_PRIVILEGES_ATTRIBUTES {}
impl ::core::clone::Clone for TOKEN_PRIVILEGES_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TOKEN_PRIVILEGES_ATTRIBUTES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for TOKEN_PRIVILEGES_ATTRIBUTES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for TOKEN_PRIVILEGES_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TOKEN_PRIVILEGES_ATTRIBUTES").field(&self.0).finish()
    }
}
impl TOKEN_PRIVILEGES_ATTRIBUTES {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for TOKEN_PRIVILEGES_ATTRIBUTES {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for TOKEN_PRIVILEGES_ATTRIBUTES {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for TOKEN_PRIVILEGES_ATTRIBUTES {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for TOKEN_PRIVILEGES_ATTRIBUTES {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for TOKEN_PRIVILEGES_ATTRIBUTES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Security\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TOKEN_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TokenPrimary: TOKEN_TYPE = TOKEN_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const TokenImpersonation: TOKEN_TYPE = TOKEN_TYPE(2i32);
impl ::core::marker::Copy for TOKEN_TYPE {}
impl ::core::clone::Clone for TOKEN_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TOKEN_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for TOKEN_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for TOKEN_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TOKEN_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WELL_KNOWN_SID_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinNullSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinWorldSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinLocalSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinCreatorOwnerSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinCreatorGroupSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinCreatorOwnerServerSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinCreatorGroupServerSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinNtAuthoritySid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(7i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinDialupSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(8i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinNetworkSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(9i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinBatchSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(10i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinInteractiveSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(11i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinServiceSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(12i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinAnonymousSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(13i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinProxySid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(14i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinEnterpriseControllersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(15i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinSelfSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(16i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinAuthenticatedUserSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(17i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinRestrictedCodeSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(18i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinTerminalServerSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(19i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinRemoteLogonIdSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(20i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinLogonIdsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(21i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinLocalSystemSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(22i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinLocalServiceSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(23i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinNetworkServiceSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(24i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinBuiltinDomainSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(25i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinBuiltinAdministratorsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(26i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinBuiltinUsersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(27i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinBuiltinGuestsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(28i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinBuiltinPowerUsersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(29i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinBuiltinAccountOperatorsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(30i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinBuiltinSystemOperatorsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(31i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinBuiltinPrintOperatorsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(32i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinBuiltinBackupOperatorsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(33i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinBuiltinReplicatorSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(34i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinBuiltinPreWindows2000CompatibleAccessSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(35i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinBuiltinRemoteDesktopUsersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(36i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinBuiltinNetworkConfigurationOperatorsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(37i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinAccountAdministratorSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(38i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinAccountGuestSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(39i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinAccountKrbtgtSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(40i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinAccountDomainAdminsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(41i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinAccountDomainUsersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(42i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinAccountDomainGuestsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(43i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinAccountComputersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(44i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinAccountControllersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(45i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinAccountCertAdminsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(46i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinAccountSchemaAdminsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(47i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinAccountEnterpriseAdminsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(48i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinAccountPolicyAdminsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(49i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinAccountRasAndIasServersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(50i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinNTLMAuthenticationSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(51i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinDigestAuthenticationSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(52i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinSChannelAuthenticationSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(53i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinThisOrganizationSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(54i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinOtherOrganizationSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(55i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinBuiltinIncomingForestTrustBuildersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(56i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinBuiltinPerfMonitoringUsersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(57i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinBuiltinPerfLoggingUsersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(58i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinBuiltinAuthorizationAccessSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(59i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinBuiltinTerminalServerLicenseServersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(60i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinBuiltinDCOMUsersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(61i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinBuiltinIUsersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(62i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinIUserSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(63i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinBuiltinCryptoOperatorsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(64i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinUntrustedLabelSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(65i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinLowLabelSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(66i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinMediumLabelSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(67i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinHighLabelSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(68i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinSystemLabelSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(69i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinWriteRestrictedCodeSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(70i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinCreatorOwnerRightsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(71i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinCacheablePrincipalsGroupSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(72i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinNonCacheablePrincipalsGroupSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(73i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinEnterpriseReadonlyControllersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(74i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinAccountReadonlyControllersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(75i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinBuiltinEventLogReadersGroup: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(76i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinNewEnterpriseReadonlyControllersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(77i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinBuiltinCertSvcDComAccessGroup: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(78i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinMediumPlusLabelSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(79i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinLocalLogonSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(80i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinConsoleLogonSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(81i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinThisOrganizationCertificateSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(82i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinApplicationPackageAuthoritySid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(83i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinBuiltinAnyPackageSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(84i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinCapabilityInternetClientSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(85i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinCapabilityInternetClientServerSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(86i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinCapabilityPrivateNetworkClientServerSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(87i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinCapabilityPicturesLibrarySid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(88i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinCapabilityVideosLibrarySid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(89i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinCapabilityMusicLibrarySid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(90i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinCapabilityDocumentsLibrarySid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(91i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinCapabilitySharedUserCertificatesSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(92i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinCapabilityEnterpriseAuthenticationSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(93i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinCapabilityRemovableStorageSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(94i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinBuiltinRDSRemoteAccessServersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(95i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinBuiltinRDSEndpointServersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(96i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinBuiltinRDSManagementServersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(97i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinUserModeDriversSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(98i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinBuiltinHyperVAdminsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(99i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinAccountCloneableControllersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(100i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinBuiltinAccessControlAssistanceOperatorsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(101i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinBuiltinRemoteManagementUsersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(102i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinAuthenticationAuthorityAssertedSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(103i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinAuthenticationServiceAssertedSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(104i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinLocalAccountSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(105i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinLocalAccountAndAdministratorSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(106i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinAccountProtectedUsersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(107i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinCapabilityAppointmentsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(108i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinCapabilityContactsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(109i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinAccountDefaultSystemManagedSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(110i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinBuiltinDefaultSystemManagedGroupSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(111i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinBuiltinStorageReplicaAdminsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(112i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinAccountKeyAdminsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(113i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinAccountEnterpriseKeyAdminsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(114i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinAuthenticationKeyTrustSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(115i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinAuthenticationKeyPropertyMFASid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(116i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinAuthenticationKeyPropertyAttestationSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(117i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinAuthenticationFreshKeyAuthSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(118i32);
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub const WinBuiltinDeviceOwnersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(119i32);
impl ::core::marker::Copy for WELL_KNOWN_SID_TYPE {}
impl ::core::clone::Clone for WELL_KNOWN_SID_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WELL_KNOWN_SID_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WELL_KNOWN_SID_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WELL_KNOWN_SID_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WELL_KNOWN_SID_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub struct ACCESS_ALLOWED_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl ::core::marker::Copy for ACCESS_ALLOWED_ACE {}
impl ::core::clone::Clone for ACCESS_ALLOWED_ACE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACCESS_ALLOWED_ACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACCESS_ALLOWED_ACE").field("Header", &self.Header).field("Mask", &self.Mask).field("SidStart", &self.SidStart).finish()
    }
}
impl ::windows::core::TypeKind for ACCESS_ALLOWED_ACE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for ACCESS_ALLOWED_ACE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Mask == other.Mask && self.SidStart == other.SidStart
    }
}
impl ::core::cmp::Eq for ACCESS_ALLOWED_ACE {}
impl ::core::default::Default for ACCESS_ALLOWED_ACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub struct ACCESS_ALLOWED_CALLBACK_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl ::core::marker::Copy for ACCESS_ALLOWED_CALLBACK_ACE {}
impl ::core::clone::Clone for ACCESS_ALLOWED_CALLBACK_ACE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACCESS_ALLOWED_CALLBACK_ACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACCESS_ALLOWED_CALLBACK_ACE").field("Header", &self.Header).field("Mask", &self.Mask).field("SidStart", &self.SidStart).finish()
    }
}
impl ::windows::core::TypeKind for ACCESS_ALLOWED_CALLBACK_ACE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for ACCESS_ALLOWED_CALLBACK_ACE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Mask == other.Mask && self.SidStart == other.SidStart
    }
}
impl ::core::cmp::Eq for ACCESS_ALLOWED_CALLBACK_ACE {}
impl ::core::default::Default for ACCESS_ALLOWED_CALLBACK_ACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub struct ACCESS_ALLOWED_CALLBACK_OBJECT_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub Flags: SYSTEM_AUDIT_OBJECT_ACE_FLAGS,
    pub ObjectType: ::windows::core::GUID,
    pub InheritedObjectType: ::windows::core::GUID,
    pub SidStart: u32,
}
impl ::core::marker::Copy for ACCESS_ALLOWED_CALLBACK_OBJECT_ACE {}
impl ::core::clone::Clone for ACCESS_ALLOWED_CALLBACK_OBJECT_ACE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACCESS_ALLOWED_CALLBACK_OBJECT_ACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACCESS_ALLOWED_CALLBACK_OBJECT_ACE").field("Header", &self.Header).field("Mask", &self.Mask).field("Flags", &self.Flags).field("ObjectType", &self.ObjectType).field("InheritedObjectType", &self.InheritedObjectType).field("SidStart", &self.SidStart).finish()
    }
}
impl ::windows::core::TypeKind for ACCESS_ALLOWED_CALLBACK_OBJECT_ACE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for ACCESS_ALLOWED_CALLBACK_OBJECT_ACE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Mask == other.Mask && self.Flags == other.Flags && self.ObjectType == other.ObjectType && self.InheritedObjectType == other.InheritedObjectType && self.SidStart == other.SidStart
    }
}
impl ::core::cmp::Eq for ACCESS_ALLOWED_CALLBACK_OBJECT_ACE {}
impl ::core::default::Default for ACCESS_ALLOWED_CALLBACK_OBJECT_ACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub struct ACCESS_ALLOWED_OBJECT_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub Flags: SYSTEM_AUDIT_OBJECT_ACE_FLAGS,
    pub ObjectType: ::windows::core::GUID,
    pub InheritedObjectType: ::windows::core::GUID,
    pub SidStart: u32,
}
impl ::core::marker::Copy for ACCESS_ALLOWED_OBJECT_ACE {}
impl ::core::clone::Clone for ACCESS_ALLOWED_OBJECT_ACE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACCESS_ALLOWED_OBJECT_ACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACCESS_ALLOWED_OBJECT_ACE").field("Header", &self.Header).field("Mask", &self.Mask).field("Flags", &self.Flags).field("ObjectType", &self.ObjectType).field("InheritedObjectType", &self.InheritedObjectType).field("SidStart", &self.SidStart).finish()
    }
}
impl ::windows::core::TypeKind for ACCESS_ALLOWED_OBJECT_ACE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for ACCESS_ALLOWED_OBJECT_ACE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Mask == other.Mask && self.Flags == other.Flags && self.ObjectType == other.ObjectType && self.InheritedObjectType == other.InheritedObjectType && self.SidStart == other.SidStart
    }
}
impl ::core::cmp::Eq for ACCESS_ALLOWED_OBJECT_ACE {}
impl ::core::default::Default for ACCESS_ALLOWED_OBJECT_ACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub struct ACCESS_DENIED_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl ::core::marker::Copy for ACCESS_DENIED_ACE {}
impl ::core::clone::Clone for ACCESS_DENIED_ACE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACCESS_DENIED_ACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACCESS_DENIED_ACE").field("Header", &self.Header).field("Mask", &self.Mask).field("SidStart", &self.SidStart).finish()
    }
}
impl ::windows::core::TypeKind for ACCESS_DENIED_ACE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for ACCESS_DENIED_ACE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Mask == other.Mask && self.SidStart == other.SidStart
    }
}
impl ::core::cmp::Eq for ACCESS_DENIED_ACE {}
impl ::core::default::Default for ACCESS_DENIED_ACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub struct ACCESS_DENIED_CALLBACK_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl ::core::marker::Copy for ACCESS_DENIED_CALLBACK_ACE {}
impl ::core::clone::Clone for ACCESS_DENIED_CALLBACK_ACE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACCESS_DENIED_CALLBACK_ACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACCESS_DENIED_CALLBACK_ACE").field("Header", &self.Header).field("Mask", &self.Mask).field("SidStart", &self.SidStart).finish()
    }
}
impl ::windows::core::TypeKind for ACCESS_DENIED_CALLBACK_ACE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for ACCESS_DENIED_CALLBACK_ACE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Mask == other.Mask && self.SidStart == other.SidStart
    }
}
impl ::core::cmp::Eq for ACCESS_DENIED_CALLBACK_ACE {}
impl ::core::default::Default for ACCESS_DENIED_CALLBACK_ACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub struct ACCESS_DENIED_CALLBACK_OBJECT_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub Flags: SYSTEM_AUDIT_OBJECT_ACE_FLAGS,
    pub ObjectType: ::windows::core::GUID,
    pub InheritedObjectType: ::windows::core::GUID,
    pub SidStart: u32,
}
impl ::core::marker::Copy for ACCESS_DENIED_CALLBACK_OBJECT_ACE {}
impl ::core::clone::Clone for ACCESS_DENIED_CALLBACK_OBJECT_ACE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACCESS_DENIED_CALLBACK_OBJECT_ACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACCESS_DENIED_CALLBACK_OBJECT_ACE").field("Header", &self.Header).field("Mask", &self.Mask).field("Flags", &self.Flags).field("ObjectType", &self.ObjectType).field("InheritedObjectType", &self.InheritedObjectType).field("SidStart", &self.SidStart).finish()
    }
}
impl ::windows::core::TypeKind for ACCESS_DENIED_CALLBACK_OBJECT_ACE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for ACCESS_DENIED_CALLBACK_OBJECT_ACE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Mask == other.Mask && self.Flags == other.Flags && self.ObjectType == other.ObjectType && self.InheritedObjectType == other.InheritedObjectType && self.SidStart == other.SidStart
    }
}
impl ::core::cmp::Eq for ACCESS_DENIED_CALLBACK_OBJECT_ACE {}
impl ::core::default::Default for ACCESS_DENIED_CALLBACK_OBJECT_ACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub struct ACCESS_DENIED_OBJECT_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub Flags: SYSTEM_AUDIT_OBJECT_ACE_FLAGS,
    pub ObjectType: ::windows::core::GUID,
    pub InheritedObjectType: ::windows::core::GUID,
    pub SidStart: u32,
}
impl ::core::marker::Copy for ACCESS_DENIED_OBJECT_ACE {}
impl ::core::clone::Clone for ACCESS_DENIED_OBJECT_ACE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACCESS_DENIED_OBJECT_ACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACCESS_DENIED_OBJECT_ACE").field("Header", &self.Header).field("Mask", &self.Mask).field("Flags", &self.Flags).field("ObjectType", &self.ObjectType).field("InheritedObjectType", &self.InheritedObjectType).field("SidStart", &self.SidStart).finish()
    }
}
impl ::windows::core::TypeKind for ACCESS_DENIED_OBJECT_ACE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for ACCESS_DENIED_OBJECT_ACE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Mask == other.Mask && self.Flags == other.Flags && self.ObjectType == other.ObjectType && self.InheritedObjectType == other.InheritedObjectType && self.SidStart == other.SidStart
    }
}
impl ::core::cmp::Eq for ACCESS_DENIED_OBJECT_ACE {}
impl ::core::default::Default for ACCESS_DENIED_OBJECT_ACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub struct ACCESS_REASONS {
    pub Data: [u32; 32],
}
impl ::core::marker::Copy for ACCESS_REASONS {}
impl ::core::clone::Clone for ACCESS_REASONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACCESS_REASONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACCESS_REASONS").field("Data", &self.Data).finish()
    }
}
impl ::windows::core::TypeKind for ACCESS_REASONS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for ACCESS_REASONS {
    fn eq(&self, other: &Self) -> bool {
        self.Data == other.Data
    }
}
impl ::core::cmp::Eq for ACCESS_REASONS {}
impl ::core::default::Default for ACCESS_REASONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub struct ACE_HEADER {
    pub AceType: u8,
    pub AceFlags: u8,
    pub AceSize: u16,
}
impl ::core::marker::Copy for ACE_HEADER {}
impl ::core::clone::Clone for ACE_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACE_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACE_HEADER").field("AceType", &self.AceType).field("AceFlags", &self.AceFlags).field("AceSize", &self.AceSize).finish()
    }
}
impl ::windows::core::TypeKind for ACE_HEADER {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for ACE_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.AceType == other.AceType && self.AceFlags == other.AceFlags && self.AceSize == other.AceSize
    }
}
impl ::core::cmp::Eq for ACE_HEADER {}
impl ::core::default::Default for ACE_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub struct ACL {
    pub AclRevision: u8,
    pub Sbz1: u8,
    pub AclSize: u16,
    pub AceCount: u16,
    pub Sbz2: u16,
}
impl ::core::marker::Copy for ACL {}
impl ::core::clone::Clone for ACL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACL").field("AclRevision", &self.AclRevision).field("Sbz1", &self.Sbz1).field("AclSize", &self.AclSize).field("AceCount", &self.AceCount).field("Sbz2", &self.Sbz2).finish()
    }
}
impl ::windows::core::TypeKind for ACL {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for ACL {
    fn eq(&self, other: &Self) -> bool {
        self.AclRevision == other.AclRevision && self.Sbz1 == other.Sbz1 && self.AclSize == other.AclSize && self.AceCount == other.AceCount && self.Sbz2 == other.Sbz2
    }
}
impl ::core::cmp::Eq for ACL {}
impl ::core::default::Default for ACL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub struct ACL_REVISION_INFORMATION {
    pub AclRevision: u32,
}
impl ::core::marker::Copy for ACL_REVISION_INFORMATION {}
impl ::core::clone::Clone for ACL_REVISION_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACL_REVISION_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACL_REVISION_INFORMATION").field("AclRevision", &self.AclRevision).finish()
    }
}
impl ::windows::core::TypeKind for ACL_REVISION_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for ACL_REVISION_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.AclRevision == other.AclRevision
    }
}
impl ::core::cmp::Eq for ACL_REVISION_INFORMATION {}
impl ::core::default::Default for ACL_REVISION_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub struct ACL_SIZE_INFORMATION {
    pub AceCount: u32,
    pub AclBytesInUse: u32,
    pub AclBytesFree: u32,
}
impl ::core::marker::Copy for ACL_SIZE_INFORMATION {}
impl ::core::clone::Clone for ACL_SIZE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACL_SIZE_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACL_SIZE_INFORMATION").field("AceCount", &self.AceCount).field("AclBytesInUse", &self.AclBytesInUse).field("AclBytesFree", &self.AclBytesFree).finish()
    }
}
impl ::windows::core::TypeKind for ACL_SIZE_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for ACL_SIZE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.AceCount == other.AceCount && self.AclBytesInUse == other.AclBytesInUse && self.AclBytesFree == other.AclBytesFree
    }
}
impl ::core::cmp::Eq for ACL_SIZE_INFORMATION {}
impl ::core::default::Default for ACL_SIZE_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub struct CLAIM_SECURITY_ATTRIBUTES_INFORMATION {
    pub Version: u16,
    pub Reserved: u16,
    pub AttributeCount: u32,
    pub Attribute: CLAIM_SECURITY_ATTRIBUTES_INFORMATION_0,
}
impl ::core::marker::Copy for CLAIM_SECURITY_ATTRIBUTES_INFORMATION {}
impl ::core::clone::Clone for CLAIM_SECURITY_ATTRIBUTES_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for CLAIM_SECURITY_ATTRIBUTES_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for CLAIM_SECURITY_ATTRIBUTES_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub union CLAIM_SECURITY_ATTRIBUTES_INFORMATION_0 {
    pub pAttributeV1: *mut CLAIM_SECURITY_ATTRIBUTE_V1,
}
impl ::core::marker::Copy for CLAIM_SECURITY_ATTRIBUTES_INFORMATION_0 {}
impl ::core::clone::Clone for CLAIM_SECURITY_ATTRIBUTES_INFORMATION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for CLAIM_SECURITY_ATTRIBUTES_INFORMATION_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for CLAIM_SECURITY_ATTRIBUTES_INFORMATION_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub struct CLAIM_SECURITY_ATTRIBUTE_FQBN_VALUE {
    pub Version: u64,
    pub Name: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for CLAIM_SECURITY_ATTRIBUTE_FQBN_VALUE {}
impl ::core::clone::Clone for CLAIM_SECURITY_ATTRIBUTE_FQBN_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CLAIM_SECURITY_ATTRIBUTE_FQBN_VALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLAIM_SECURITY_ATTRIBUTE_FQBN_VALUE").field("Version", &self.Version).field("Name", &self.Name).finish()
    }
}
impl ::windows::core::TypeKind for CLAIM_SECURITY_ATTRIBUTE_FQBN_VALUE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CLAIM_SECURITY_ATTRIBUTE_FQBN_VALUE {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Name == other.Name
    }
}
impl ::core::cmp::Eq for CLAIM_SECURITY_ATTRIBUTE_FQBN_VALUE {}
impl ::core::default::Default for CLAIM_SECURITY_ATTRIBUTE_FQBN_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub struct CLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {
    pub pValue: *mut ::core::ffi::c_void,
    pub ValueLength: u32,
}
impl ::core::marker::Copy for CLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {}
impl ::core::clone::Clone for CLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE").field("pValue", &self.pValue).field("ValueLength", &self.ValueLength).finish()
    }
}
impl ::windows::core::TypeKind for CLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {
    fn eq(&self, other: &Self) -> bool {
        self.pValue == other.pValue && self.ValueLength == other.ValueLength
    }
}
impl ::core::cmp::Eq for CLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {}
impl ::core::default::Default for CLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub struct CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1 {
    pub Name: u32,
    pub ValueType: CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE,
    pub Reserved: u16,
    pub Flags: CLAIM_SECURITY_ATTRIBUTE_FLAGS,
    pub ValueCount: u32,
    pub Values: CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1_0,
}
impl ::core::marker::Copy for CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1 {}
impl ::core::clone::Clone for CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub union CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1_0 {
    pub pInt64: [u32; 1],
    pub pUint64: [u32; 1],
    pub ppString: [u32; 1],
    pub pFqbn: [u32; 1],
    pub pOctetString: [u32; 1],
}
impl ::core::marker::Copy for CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1_0 {}
impl ::core::clone::Clone for CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub struct CLAIM_SECURITY_ATTRIBUTE_V1 {
    pub Name: ::windows::core::PWSTR,
    pub ValueType: CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE,
    pub Reserved: u16,
    pub Flags: u32,
    pub ValueCount: u32,
    pub Values: CLAIM_SECURITY_ATTRIBUTE_V1_0,
}
impl ::core::marker::Copy for CLAIM_SECURITY_ATTRIBUTE_V1 {}
impl ::core::clone::Clone for CLAIM_SECURITY_ATTRIBUTE_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for CLAIM_SECURITY_ATTRIBUTE_V1 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for CLAIM_SECURITY_ATTRIBUTE_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub union CLAIM_SECURITY_ATTRIBUTE_V1_0 {
    pub pInt64: *mut i64,
    pub pUint64: *mut u64,
    pub ppString: *mut ::windows::core::PWSTR,
    pub pFqbn: *mut CLAIM_SECURITY_ATTRIBUTE_FQBN_VALUE,
    pub pOctetString: *mut CLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE,
}
impl ::core::marker::Copy for CLAIM_SECURITY_ATTRIBUTE_V1_0 {}
impl ::core::clone::Clone for CLAIM_SECURITY_ATTRIBUTE_V1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for CLAIM_SECURITY_ATTRIBUTE_V1_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for CLAIM_SECURITY_ATTRIBUTE_V1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub struct GENERIC_MAPPING {
    pub GenericRead: u32,
    pub GenericWrite: u32,
    pub GenericExecute: u32,
    pub GenericAll: u32,
}
impl ::core::marker::Copy for GENERIC_MAPPING {}
impl ::core::clone::Clone for GENERIC_MAPPING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GENERIC_MAPPING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GENERIC_MAPPING").field("GenericRead", &self.GenericRead).field("GenericWrite", &self.GenericWrite).field("GenericExecute", &self.GenericExecute).field("GenericAll", &self.GenericAll).finish()
    }
}
impl ::windows::core::TypeKind for GENERIC_MAPPING {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for GENERIC_MAPPING {
    fn eq(&self, other: &Self) -> bool {
        self.GenericRead == other.GenericRead && self.GenericWrite == other.GenericWrite && self.GenericExecute == other.GenericExecute && self.GenericAll == other.GenericAll
    }
}
impl ::core::cmp::Eq for GENERIC_MAPPING {}
impl ::core::default::Default for GENERIC_MAPPING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HDIAGNOSTIC_DATA_QUERY_SESSION(pub isize);
impl HDIAGNOSTIC_DATA_QUERY_SESSION {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HDIAGNOSTIC_DATA_QUERY_SESSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HDIAGNOSTIC_DATA_QUERY_SESSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HDIAGNOSTIC_DATA_QUERY_SESSION {}
impl ::core::fmt::Debug for HDIAGNOSTIC_DATA_QUERY_SESSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HDIAGNOSTIC_DATA_QUERY_SESSION").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for HDIAGNOSTIC_DATA_QUERY_SESSION {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION(pub isize);
impl HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION {}
impl ::core::fmt::Debug for HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION(pub isize);
impl HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION {}
impl ::core::fmt::Debug for HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HDIAGNOSTIC_EVENT_TAG_DESCRIPTION(pub isize);
impl HDIAGNOSTIC_EVENT_TAG_DESCRIPTION {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HDIAGNOSTIC_EVENT_TAG_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HDIAGNOSTIC_EVENT_TAG_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HDIAGNOSTIC_EVENT_TAG_DESCRIPTION {}
impl ::core::fmt::Debug for HDIAGNOSTIC_EVENT_TAG_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HDIAGNOSTIC_EVENT_TAG_DESCRIPTION").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for HDIAGNOSTIC_EVENT_TAG_DESCRIPTION {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HDIAGNOSTIC_RECORD(pub isize);
impl HDIAGNOSTIC_RECORD {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HDIAGNOSTIC_RECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HDIAGNOSTIC_RECORD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HDIAGNOSTIC_RECORD {}
impl ::core::fmt::Debug for HDIAGNOSTIC_RECORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HDIAGNOSTIC_RECORD").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for HDIAGNOSTIC_RECORD {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HDIAGNOSTIC_REPORT(pub isize);
impl HDIAGNOSTIC_REPORT {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HDIAGNOSTIC_REPORT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HDIAGNOSTIC_REPORT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HDIAGNOSTIC_REPORT {}
impl ::core::fmt::Debug for HDIAGNOSTIC_REPORT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HDIAGNOSTIC_REPORT").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for HDIAGNOSTIC_REPORT {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct LLFILETIME {
    pub Anonymous: LLFILETIME_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LLFILETIME {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LLFILETIME {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for LLFILETIME {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LLFILETIME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union LLFILETIME_0 {
    pub ll: i64,
    pub ft: super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LLFILETIME_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LLFILETIME_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for LLFILETIME_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LLFILETIME_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct LUID_AND_ATTRIBUTES {
    pub Luid: super::Foundation::LUID,
    pub Attributes: TOKEN_PRIVILEGES_ATTRIBUTES,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LUID_AND_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LUID_AND_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for LUID_AND_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LUID_AND_ATTRIBUTES").field("Luid", &self.Luid).field("Attributes", &self.Attributes).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for LUID_AND_ATTRIBUTES {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for LUID_AND_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.Luid == other.Luid && self.Attributes == other.Attributes
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for LUID_AND_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LUID_AND_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NCRYPT_DESCRIPTOR_HANDLE(pub isize);
impl NCRYPT_DESCRIPTOR_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for NCRYPT_DESCRIPTOR_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for NCRYPT_DESCRIPTOR_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for NCRYPT_DESCRIPTOR_HANDLE {}
impl ::core::fmt::Debug for NCRYPT_DESCRIPTOR_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NCRYPT_DESCRIPTOR_HANDLE").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for NCRYPT_DESCRIPTOR_HANDLE {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NCRYPT_STREAM_HANDLE(pub isize);
impl NCRYPT_STREAM_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for NCRYPT_STREAM_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for NCRYPT_STREAM_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for NCRYPT_STREAM_HANDLE {}
impl ::core::fmt::Debug for NCRYPT_STREAM_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NCRYPT_STREAM_HANDLE").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for NCRYPT_STREAM_HANDLE {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub struct OBJECT_TYPE_LIST {
    pub Level: u16,
    pub Sbz: u16,
    pub ObjectType: *mut ::windows::core::GUID,
}
impl ::core::marker::Copy for OBJECT_TYPE_LIST {}
impl ::core::clone::Clone for OBJECT_TYPE_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for OBJECT_TYPE_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OBJECT_TYPE_LIST").field("Level", &self.Level).field("Sbz", &self.Sbz).field("ObjectType", &self.ObjectType).finish()
    }
}
impl ::windows::core::TypeKind for OBJECT_TYPE_LIST {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for OBJECT_TYPE_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Level == other.Level && self.Sbz == other.Sbz && self.ObjectType == other.ObjectType
    }
}
impl ::core::cmp::Eq for OBJECT_TYPE_LIST {}
impl ::core::default::Default for OBJECT_TYPE_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PRIVILEGE_SET {
    pub PrivilegeCount: u32,
    pub Control: u32,
    pub Privilege: [LUID_AND_ATTRIBUTES; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PRIVILEGE_SET {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PRIVILEGE_SET {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PRIVILEGE_SET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PRIVILEGE_SET").field("PrivilegeCount", &self.PrivilegeCount).field("Control", &self.Control).field("Privilege", &self.Privilege).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for PRIVILEGE_SET {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PRIVILEGE_SET {
    fn eq(&self, other: &Self) -> bool {
        self.PrivilegeCount == other.PrivilegeCount && self.Control == other.Control && self.Privilege == other.Privilege
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PRIVILEGE_SET {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PRIVILEGE_SET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PSECURITY_DESCRIPTOR(pub *mut ::core::ffi::c_void);
impl PSECURITY_DESCRIPTOR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl ::core::default::Default for PSECURITY_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for PSECURITY_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for PSECURITY_DESCRIPTOR {}
impl ::core::fmt::Debug for PSECURITY_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PSECURITY_DESCRIPTOR").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for PSECURITY_DESCRIPTOR {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub struct QUOTA_LIMITS {
    pub PagedPoolLimit: usize,
    pub NonPagedPoolLimit: usize,
    pub MinimumWorkingSetSize: usize,
    pub MaximumWorkingSetSize: usize,
    pub PagefileLimit: usize,
    pub TimeLimit: i64,
}
impl ::core::marker::Copy for QUOTA_LIMITS {}
impl ::core::clone::Clone for QUOTA_LIMITS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for QUOTA_LIMITS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUOTA_LIMITS").field("PagedPoolLimit", &self.PagedPoolLimit).field("NonPagedPoolLimit", &self.NonPagedPoolLimit).field("MinimumWorkingSetSize", &self.MinimumWorkingSetSize).field("MaximumWorkingSetSize", &self.MaximumWorkingSetSize).field("PagefileLimit", &self.PagefileLimit).field("TimeLimit", &self.TimeLimit).finish()
    }
}
impl ::windows::core::TypeKind for QUOTA_LIMITS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for QUOTA_LIMITS {
    fn eq(&self, other: &Self) -> bool {
        self.PagedPoolLimit == other.PagedPoolLimit && self.NonPagedPoolLimit == other.NonPagedPoolLimit && self.MinimumWorkingSetSize == other.MinimumWorkingSetSize && self.MaximumWorkingSetSize == other.MaximumWorkingSetSize && self.PagefileLimit == other.PagefileLimit && self.TimeLimit == other.TimeLimit
    }
}
impl ::core::cmp::Eq for QUOTA_LIMITS {}
impl ::core::default::Default for QUOTA_LIMITS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SAFER_LEVEL_HANDLE(pub isize);
impl SAFER_LEVEL_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for SAFER_LEVEL_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for SAFER_LEVEL_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for SAFER_LEVEL_HANDLE {}
impl ::core::fmt::Debug for SAFER_LEVEL_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SAFER_LEVEL_HANDLE").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for SAFER_LEVEL_HANDLE {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SC_HANDLE(pub isize);
impl SC_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for SC_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for SC_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for SC_HANDLE {}
impl ::core::fmt::Debug for SC_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SC_HANDLE").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for SC_HANDLE {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SECURITY_ATTRIBUTES {
    pub nLength: u32,
    pub lpSecurityDescriptor: *mut ::core::ffi::c_void,
    pub bInheritHandle: super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SECURITY_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SECURITY_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SECURITY_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SECURITY_ATTRIBUTES").field("nLength", &self.nLength).field("lpSecurityDescriptor", &self.lpSecurityDescriptor).field("bInheritHandle", &self.bInheritHandle).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for SECURITY_ATTRIBUTES {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SECURITY_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.nLength == other.nLength && self.lpSecurityDescriptor == other.lpSecurityDescriptor && self.bInheritHandle == other.bInheritHandle
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SECURITY_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SECURITY_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SECURITY_CAPABILITIES {
    pub AppContainerSid: super::Foundation::PSID,
    pub Capabilities: *mut SID_AND_ATTRIBUTES,
    pub CapabilityCount: u32,
    pub Reserved: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SECURITY_CAPABILITIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SECURITY_CAPABILITIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SECURITY_CAPABILITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SECURITY_CAPABILITIES").field("AppContainerSid", &self.AppContainerSid).field("Capabilities", &self.Capabilities).field("CapabilityCount", &self.CapabilityCount).field("Reserved", &self.Reserved).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for SECURITY_CAPABILITIES {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SECURITY_CAPABILITIES {
    fn eq(&self, other: &Self) -> bool {
        self.AppContainerSid == other.AppContainerSid && self.Capabilities == other.Capabilities && self.CapabilityCount == other.CapabilityCount && self.Reserved == other.Reserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SECURITY_CAPABILITIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SECURITY_CAPABILITIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SECURITY_DESCRIPTOR {
    pub Revision: u8,
    pub Sbz1: u8,
    pub Control: SECURITY_DESCRIPTOR_CONTROL,
    pub Owner: super::Foundation::PSID,
    pub Group: super::Foundation::PSID,
    pub Sacl: *mut ACL,
    pub Dacl: *mut ACL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SECURITY_DESCRIPTOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SECURITY_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SECURITY_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SECURITY_DESCRIPTOR").field("Revision", &self.Revision).field("Sbz1", &self.Sbz1).field("Control", &self.Control).field("Owner", &self.Owner).field("Group", &self.Group).field("Sacl", &self.Sacl).field("Dacl", &self.Dacl).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for SECURITY_DESCRIPTOR {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SECURITY_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Revision == other.Revision && self.Sbz1 == other.Sbz1 && self.Control == other.Control && self.Owner == other.Owner && self.Group == other.Group && self.Sacl == other.Sacl && self.Dacl == other.Dacl
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SECURITY_DESCRIPTOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SECURITY_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub struct SECURITY_DESCRIPTOR_RELATIVE {
    pub Revision: u8,
    pub Sbz1: u8,
    pub Control: SECURITY_DESCRIPTOR_CONTROL,
    pub Owner: u32,
    pub Group: u32,
    pub Sacl: u32,
    pub Dacl: u32,
}
impl ::core::marker::Copy for SECURITY_DESCRIPTOR_RELATIVE {}
impl ::core::clone::Clone for SECURITY_DESCRIPTOR_RELATIVE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SECURITY_DESCRIPTOR_RELATIVE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SECURITY_DESCRIPTOR_RELATIVE").field("Revision", &self.Revision).field("Sbz1", &self.Sbz1).field("Control", &self.Control).field("Owner", &self.Owner).field("Group", &self.Group).field("Sacl", &self.Sacl).field("Dacl", &self.Dacl).finish()
    }
}
impl ::windows::core::TypeKind for SECURITY_DESCRIPTOR_RELATIVE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SECURITY_DESCRIPTOR_RELATIVE {
    fn eq(&self, other: &Self) -> bool {
        self.Revision == other.Revision && self.Sbz1 == other.Sbz1 && self.Control == other.Control && self.Owner == other.Owner && self.Group == other.Group && self.Sacl == other.Sacl && self.Dacl == other.Dacl
    }
}
impl ::core::cmp::Eq for SECURITY_DESCRIPTOR_RELATIVE {}
impl ::core::default::Default for SECURITY_DESCRIPTOR_RELATIVE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SECURITY_QUALITY_OF_SERVICE {
    pub Length: u32,
    pub ImpersonationLevel: SECURITY_IMPERSONATION_LEVEL,
    pub ContextTrackingMode: u8,
    pub EffectiveOnly: super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SECURITY_QUALITY_OF_SERVICE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SECURITY_QUALITY_OF_SERVICE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SECURITY_QUALITY_OF_SERVICE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SECURITY_QUALITY_OF_SERVICE").field("Length", &self.Length).field("ImpersonationLevel", &self.ImpersonationLevel).field("ContextTrackingMode", &self.ContextTrackingMode).field("EffectiveOnly", &self.EffectiveOnly).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for SECURITY_QUALITY_OF_SERVICE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SECURITY_QUALITY_OF_SERVICE {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.ImpersonationLevel == other.ImpersonationLevel && self.ContextTrackingMode == other.ContextTrackingMode && self.EffectiveOnly == other.EffectiveOnly
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SECURITY_QUALITY_OF_SERVICE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SECURITY_QUALITY_OF_SERVICE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SE_ACCESS_REPLY {
    pub Size: u32,
    pub ResultListCount: u32,
    pub GrantedAccess: *mut u32,
    pub AccessStatus: *mut u32,
    pub AccessReason: *mut ACCESS_REASONS,
    pub Privileges: *mut *mut PRIVILEGE_SET,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SE_ACCESS_REPLY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SE_ACCESS_REPLY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SE_ACCESS_REPLY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SE_ACCESS_REPLY").field("Size", &self.Size).field("ResultListCount", &self.ResultListCount).field("GrantedAccess", &self.GrantedAccess).field("AccessStatus", &self.AccessStatus).field("AccessReason", &self.AccessReason).field("Privileges", &self.Privileges).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for SE_ACCESS_REPLY {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SE_ACCESS_REPLY {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.ResultListCount == other.ResultListCount && self.GrantedAccess == other.GrantedAccess && self.AccessStatus == other.AccessStatus && self.AccessReason == other.AccessReason && self.Privileges == other.Privileges
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SE_ACCESS_REPLY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SE_ACCESS_REPLY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SE_ACCESS_REQUEST {
    pub Size: u32,
    pub SeSecurityDescriptor: *mut SE_SECURITY_DESCRIPTOR,
    pub DesiredAccess: u32,
    pub PreviouslyGrantedAccess: u32,
    pub PrincipalSelfSid: super::Foundation::PSID,
    pub GenericMapping: *mut GENERIC_MAPPING,
    pub ObjectTypeListCount: u32,
    pub ObjectTypeList: *mut OBJECT_TYPE_LIST,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SE_ACCESS_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SE_ACCESS_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SE_ACCESS_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SE_ACCESS_REQUEST").field("Size", &self.Size).field("SeSecurityDescriptor", &self.SeSecurityDescriptor).field("DesiredAccess", &self.DesiredAccess).field("PreviouslyGrantedAccess", &self.PreviouslyGrantedAccess).field("PrincipalSelfSid", &self.PrincipalSelfSid).field("GenericMapping", &self.GenericMapping).field("ObjectTypeListCount", &self.ObjectTypeListCount).field("ObjectTypeList", &self.ObjectTypeList).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for SE_ACCESS_REQUEST {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SE_ACCESS_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.SeSecurityDescriptor == other.SeSecurityDescriptor && self.DesiredAccess == other.DesiredAccess && self.PreviouslyGrantedAccess == other.PreviouslyGrantedAccess && self.PrincipalSelfSid == other.PrincipalSelfSid && self.GenericMapping == other.GenericMapping && self.ObjectTypeListCount == other.ObjectTypeListCount && self.ObjectTypeList == other.ObjectTypeList
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SE_ACCESS_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SE_ACCESS_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SE_IMPERSONATION_STATE {
    pub Token: *mut ::core::ffi::c_void,
    pub CopyOnOpen: super::Foundation::BOOLEAN,
    pub EffectiveOnly: super::Foundation::BOOLEAN,
    pub Level: SECURITY_IMPERSONATION_LEVEL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SE_IMPERSONATION_STATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SE_IMPERSONATION_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SE_IMPERSONATION_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SE_IMPERSONATION_STATE").field("Token", &self.Token).field("CopyOnOpen", &self.CopyOnOpen).field("EffectiveOnly", &self.EffectiveOnly).field("Level", &self.Level).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for SE_IMPERSONATION_STATE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SE_IMPERSONATION_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.Token == other.Token && self.CopyOnOpen == other.CopyOnOpen && self.EffectiveOnly == other.EffectiveOnly && self.Level == other.Level
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SE_IMPERSONATION_STATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SE_IMPERSONATION_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub struct SE_SECURITY_DESCRIPTOR {
    pub Size: u32,
    pub Flags: u32,
    pub SecurityDescriptor: PSECURITY_DESCRIPTOR,
}
impl ::core::marker::Copy for SE_SECURITY_DESCRIPTOR {}
impl ::core::clone::Clone for SE_SECURITY_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SE_SECURITY_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SE_SECURITY_DESCRIPTOR").field("Size", &self.Size).field("Flags", &self.Flags).field("SecurityDescriptor", &self.SecurityDescriptor).finish()
    }
}
impl ::windows::core::TypeKind for SE_SECURITY_DESCRIPTOR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SE_SECURITY_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Flags == other.Flags && self.SecurityDescriptor == other.SecurityDescriptor
    }
}
impl ::core::cmp::Eq for SE_SECURITY_DESCRIPTOR {}
impl ::core::default::Default for SE_SECURITY_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub union SE_SID {
    pub Sid: SID,
    pub Buffer: [u8; 68],
}
impl ::core::marker::Copy for SE_SID {}
impl ::core::clone::Clone for SE_SID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for SE_SID {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for SE_SID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub struct SID {
    pub Revision: u8,
    pub SubAuthorityCount: u8,
    pub IdentifierAuthority: SID_IDENTIFIER_AUTHORITY,
    pub SubAuthority: [u32; 1],
}
impl ::core::marker::Copy for SID {}
impl ::core::clone::Clone for SID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SID").field("Revision", &self.Revision).field("SubAuthorityCount", &self.SubAuthorityCount).field("IdentifierAuthority", &self.IdentifierAuthority).field("SubAuthority", &self.SubAuthority).finish()
    }
}
impl ::windows::core::TypeKind for SID {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SID {
    fn eq(&self, other: &Self) -> bool {
        self.Revision == other.Revision && self.SubAuthorityCount == other.SubAuthorityCount && self.IdentifierAuthority == other.IdentifierAuthority && self.SubAuthority == other.SubAuthority
    }
}
impl ::core::cmp::Eq for SID {}
impl ::core::default::Default for SID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SID_AND_ATTRIBUTES {
    pub Sid: super::Foundation::PSID,
    pub Attributes: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SID_AND_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SID_AND_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SID_AND_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SID_AND_ATTRIBUTES").field("Sid", &self.Sid).field("Attributes", &self.Attributes).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for SID_AND_ATTRIBUTES {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SID_AND_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.Sid == other.Sid && self.Attributes == other.Attributes
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SID_AND_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SID_AND_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SID_AND_ATTRIBUTES_HASH {
    pub SidCount: u32,
    pub SidAttr: *mut SID_AND_ATTRIBUTES,
    pub Hash: [usize; 32],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SID_AND_ATTRIBUTES_HASH {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SID_AND_ATTRIBUTES_HASH {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SID_AND_ATTRIBUTES_HASH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SID_AND_ATTRIBUTES_HASH").field("SidCount", &self.SidCount).field("SidAttr", &self.SidAttr).field("Hash", &self.Hash).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for SID_AND_ATTRIBUTES_HASH {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SID_AND_ATTRIBUTES_HASH {
    fn eq(&self, other: &Self) -> bool {
        self.SidCount == other.SidCount && self.SidAttr == other.SidAttr && self.Hash == other.Hash
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SID_AND_ATTRIBUTES_HASH {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SID_AND_ATTRIBUTES_HASH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub struct SID_IDENTIFIER_AUTHORITY {
    pub Value: [u8; 6],
}
impl ::core::marker::Copy for SID_IDENTIFIER_AUTHORITY {}
impl ::core::clone::Clone for SID_IDENTIFIER_AUTHORITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SID_IDENTIFIER_AUTHORITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SID_IDENTIFIER_AUTHORITY").field("Value", &self.Value).finish()
    }
}
impl ::windows::core::TypeKind for SID_IDENTIFIER_AUTHORITY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SID_IDENTIFIER_AUTHORITY {
    fn eq(&self, other: &Self) -> bool {
        self.Value == other.Value
    }
}
impl ::core::cmp::Eq for SID_IDENTIFIER_AUTHORITY {}
impl ::core::default::Default for SID_IDENTIFIER_AUTHORITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub struct SYSTEM_ACCESS_FILTER_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl ::core::marker::Copy for SYSTEM_ACCESS_FILTER_ACE {}
impl ::core::clone::Clone for SYSTEM_ACCESS_FILTER_ACE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_ACCESS_FILTER_ACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_ACCESS_FILTER_ACE").field("Header", &self.Header).field("Mask", &self.Mask).field("SidStart", &self.SidStart).finish()
    }
}
impl ::windows::core::TypeKind for SYSTEM_ACCESS_FILTER_ACE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SYSTEM_ACCESS_FILTER_ACE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Mask == other.Mask && self.SidStart == other.SidStart
    }
}
impl ::core::cmp::Eq for SYSTEM_ACCESS_FILTER_ACE {}
impl ::core::default::Default for SYSTEM_ACCESS_FILTER_ACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub struct SYSTEM_ALARM_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl ::core::marker::Copy for SYSTEM_ALARM_ACE {}
impl ::core::clone::Clone for SYSTEM_ALARM_ACE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_ALARM_ACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_ALARM_ACE").field("Header", &self.Header).field("Mask", &self.Mask).field("SidStart", &self.SidStart).finish()
    }
}
impl ::windows::core::TypeKind for SYSTEM_ALARM_ACE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SYSTEM_ALARM_ACE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Mask == other.Mask && self.SidStart == other.SidStart
    }
}
impl ::core::cmp::Eq for SYSTEM_ALARM_ACE {}
impl ::core::default::Default for SYSTEM_ALARM_ACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub struct SYSTEM_ALARM_CALLBACK_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl ::core::marker::Copy for SYSTEM_ALARM_CALLBACK_ACE {}
impl ::core::clone::Clone for SYSTEM_ALARM_CALLBACK_ACE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_ALARM_CALLBACK_ACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_ALARM_CALLBACK_ACE").field("Header", &self.Header).field("Mask", &self.Mask).field("SidStart", &self.SidStart).finish()
    }
}
impl ::windows::core::TypeKind for SYSTEM_ALARM_CALLBACK_ACE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SYSTEM_ALARM_CALLBACK_ACE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Mask == other.Mask && self.SidStart == other.SidStart
    }
}
impl ::core::cmp::Eq for SYSTEM_ALARM_CALLBACK_ACE {}
impl ::core::default::Default for SYSTEM_ALARM_CALLBACK_ACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub struct SYSTEM_ALARM_CALLBACK_OBJECT_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub Flags: SYSTEM_AUDIT_OBJECT_ACE_FLAGS,
    pub ObjectType: ::windows::core::GUID,
    pub InheritedObjectType: ::windows::core::GUID,
    pub SidStart: u32,
}
impl ::core::marker::Copy for SYSTEM_ALARM_CALLBACK_OBJECT_ACE {}
impl ::core::clone::Clone for SYSTEM_ALARM_CALLBACK_OBJECT_ACE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_ALARM_CALLBACK_OBJECT_ACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_ALARM_CALLBACK_OBJECT_ACE").field("Header", &self.Header).field("Mask", &self.Mask).field("Flags", &self.Flags).field("ObjectType", &self.ObjectType).field("InheritedObjectType", &self.InheritedObjectType).field("SidStart", &self.SidStart).finish()
    }
}
impl ::windows::core::TypeKind for SYSTEM_ALARM_CALLBACK_OBJECT_ACE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SYSTEM_ALARM_CALLBACK_OBJECT_ACE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Mask == other.Mask && self.Flags == other.Flags && self.ObjectType == other.ObjectType && self.InheritedObjectType == other.InheritedObjectType && self.SidStart == other.SidStart
    }
}
impl ::core::cmp::Eq for SYSTEM_ALARM_CALLBACK_OBJECT_ACE {}
impl ::core::default::Default for SYSTEM_ALARM_CALLBACK_OBJECT_ACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub struct SYSTEM_ALARM_OBJECT_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub Flags: u32,
    pub ObjectType: ::windows::core::GUID,
    pub InheritedObjectType: ::windows::core::GUID,
    pub SidStart: u32,
}
impl ::core::marker::Copy for SYSTEM_ALARM_OBJECT_ACE {}
impl ::core::clone::Clone for SYSTEM_ALARM_OBJECT_ACE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_ALARM_OBJECT_ACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_ALARM_OBJECT_ACE").field("Header", &self.Header).field("Mask", &self.Mask).field("Flags", &self.Flags).field("ObjectType", &self.ObjectType).field("InheritedObjectType", &self.InheritedObjectType).field("SidStart", &self.SidStart).finish()
    }
}
impl ::windows::core::TypeKind for SYSTEM_ALARM_OBJECT_ACE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SYSTEM_ALARM_OBJECT_ACE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Mask == other.Mask && self.Flags == other.Flags && self.ObjectType == other.ObjectType && self.InheritedObjectType == other.InheritedObjectType && self.SidStart == other.SidStart
    }
}
impl ::core::cmp::Eq for SYSTEM_ALARM_OBJECT_ACE {}
impl ::core::default::Default for SYSTEM_ALARM_OBJECT_ACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub struct SYSTEM_AUDIT_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl ::core::marker::Copy for SYSTEM_AUDIT_ACE {}
impl ::core::clone::Clone for SYSTEM_AUDIT_ACE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_AUDIT_ACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_AUDIT_ACE").field("Header", &self.Header).field("Mask", &self.Mask).field("SidStart", &self.SidStart).finish()
    }
}
impl ::windows::core::TypeKind for SYSTEM_AUDIT_ACE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SYSTEM_AUDIT_ACE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Mask == other.Mask && self.SidStart == other.SidStart
    }
}
impl ::core::cmp::Eq for SYSTEM_AUDIT_ACE {}
impl ::core::default::Default for SYSTEM_AUDIT_ACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub struct SYSTEM_AUDIT_CALLBACK_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl ::core::marker::Copy for SYSTEM_AUDIT_CALLBACK_ACE {}
impl ::core::clone::Clone for SYSTEM_AUDIT_CALLBACK_ACE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_AUDIT_CALLBACK_ACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_AUDIT_CALLBACK_ACE").field("Header", &self.Header).field("Mask", &self.Mask).field("SidStart", &self.SidStart).finish()
    }
}
impl ::windows::core::TypeKind for SYSTEM_AUDIT_CALLBACK_ACE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SYSTEM_AUDIT_CALLBACK_ACE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Mask == other.Mask && self.SidStart == other.SidStart
    }
}
impl ::core::cmp::Eq for SYSTEM_AUDIT_CALLBACK_ACE {}
impl ::core::default::Default for SYSTEM_AUDIT_CALLBACK_ACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub struct SYSTEM_AUDIT_CALLBACK_OBJECT_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub Flags: SYSTEM_AUDIT_OBJECT_ACE_FLAGS,
    pub ObjectType: ::windows::core::GUID,
    pub InheritedObjectType: ::windows::core::GUID,
    pub SidStart: u32,
}
impl ::core::marker::Copy for SYSTEM_AUDIT_CALLBACK_OBJECT_ACE {}
impl ::core::clone::Clone for SYSTEM_AUDIT_CALLBACK_OBJECT_ACE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_AUDIT_CALLBACK_OBJECT_ACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_AUDIT_CALLBACK_OBJECT_ACE").field("Header", &self.Header).field("Mask", &self.Mask).field("Flags", &self.Flags).field("ObjectType", &self.ObjectType).field("InheritedObjectType", &self.InheritedObjectType).field("SidStart", &self.SidStart).finish()
    }
}
impl ::windows::core::TypeKind for SYSTEM_AUDIT_CALLBACK_OBJECT_ACE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SYSTEM_AUDIT_CALLBACK_OBJECT_ACE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Mask == other.Mask && self.Flags == other.Flags && self.ObjectType == other.ObjectType && self.InheritedObjectType == other.InheritedObjectType && self.SidStart == other.SidStart
    }
}
impl ::core::cmp::Eq for SYSTEM_AUDIT_CALLBACK_OBJECT_ACE {}
impl ::core::default::Default for SYSTEM_AUDIT_CALLBACK_OBJECT_ACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub struct SYSTEM_AUDIT_OBJECT_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub Flags: SYSTEM_AUDIT_OBJECT_ACE_FLAGS,
    pub ObjectType: ::windows::core::GUID,
    pub InheritedObjectType: ::windows::core::GUID,
    pub SidStart: u32,
}
impl ::core::marker::Copy for SYSTEM_AUDIT_OBJECT_ACE {}
impl ::core::clone::Clone for SYSTEM_AUDIT_OBJECT_ACE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_AUDIT_OBJECT_ACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_AUDIT_OBJECT_ACE").field("Header", &self.Header).field("Mask", &self.Mask).field("Flags", &self.Flags).field("ObjectType", &self.ObjectType).field("InheritedObjectType", &self.InheritedObjectType).field("SidStart", &self.SidStart).finish()
    }
}
impl ::windows::core::TypeKind for SYSTEM_AUDIT_OBJECT_ACE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SYSTEM_AUDIT_OBJECT_ACE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Mask == other.Mask && self.Flags == other.Flags && self.ObjectType == other.ObjectType && self.InheritedObjectType == other.InheritedObjectType && self.SidStart == other.SidStart
    }
}
impl ::core::cmp::Eq for SYSTEM_AUDIT_OBJECT_ACE {}
impl ::core::default::Default for SYSTEM_AUDIT_OBJECT_ACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub struct SYSTEM_MANDATORY_LABEL_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl ::core::marker::Copy for SYSTEM_MANDATORY_LABEL_ACE {}
impl ::core::clone::Clone for SYSTEM_MANDATORY_LABEL_ACE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_MANDATORY_LABEL_ACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_MANDATORY_LABEL_ACE").field("Header", &self.Header).field("Mask", &self.Mask).field("SidStart", &self.SidStart).finish()
    }
}
impl ::windows::core::TypeKind for SYSTEM_MANDATORY_LABEL_ACE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SYSTEM_MANDATORY_LABEL_ACE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Mask == other.Mask && self.SidStart == other.SidStart
    }
}
impl ::core::cmp::Eq for SYSTEM_MANDATORY_LABEL_ACE {}
impl ::core::default::Default for SYSTEM_MANDATORY_LABEL_ACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub struct SYSTEM_PROCESS_TRUST_LABEL_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl ::core::marker::Copy for SYSTEM_PROCESS_TRUST_LABEL_ACE {}
impl ::core::clone::Clone for SYSTEM_PROCESS_TRUST_LABEL_ACE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_PROCESS_TRUST_LABEL_ACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_PROCESS_TRUST_LABEL_ACE").field("Header", &self.Header).field("Mask", &self.Mask).field("SidStart", &self.SidStart).finish()
    }
}
impl ::windows::core::TypeKind for SYSTEM_PROCESS_TRUST_LABEL_ACE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SYSTEM_PROCESS_TRUST_LABEL_ACE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Mask == other.Mask && self.SidStart == other.SidStart
    }
}
impl ::core::cmp::Eq for SYSTEM_PROCESS_TRUST_LABEL_ACE {}
impl ::core::default::Default for SYSTEM_PROCESS_TRUST_LABEL_ACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub struct SYSTEM_RESOURCE_ATTRIBUTE_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl ::core::marker::Copy for SYSTEM_RESOURCE_ATTRIBUTE_ACE {}
impl ::core::clone::Clone for SYSTEM_RESOURCE_ATTRIBUTE_ACE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_RESOURCE_ATTRIBUTE_ACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_RESOURCE_ATTRIBUTE_ACE").field("Header", &self.Header).field("Mask", &self.Mask).field("SidStart", &self.SidStart).finish()
    }
}
impl ::windows::core::TypeKind for SYSTEM_RESOURCE_ATTRIBUTE_ACE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SYSTEM_RESOURCE_ATTRIBUTE_ACE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Mask == other.Mask && self.SidStart == other.SidStart
    }
}
impl ::core::cmp::Eq for SYSTEM_RESOURCE_ATTRIBUTE_ACE {}
impl ::core::default::Default for SYSTEM_RESOURCE_ATTRIBUTE_ACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub struct SYSTEM_SCOPED_POLICY_ID_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl ::core::marker::Copy for SYSTEM_SCOPED_POLICY_ID_ACE {}
impl ::core::clone::Clone for SYSTEM_SCOPED_POLICY_ID_ACE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_SCOPED_POLICY_ID_ACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_SCOPED_POLICY_ID_ACE").field("Header", &self.Header).field("Mask", &self.Mask).field("SidStart", &self.SidStart).finish()
    }
}
impl ::windows::core::TypeKind for SYSTEM_SCOPED_POLICY_ID_ACE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SYSTEM_SCOPED_POLICY_ID_ACE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Mask == other.Mask && self.SidStart == other.SidStart
    }
}
impl ::core::cmp::Eq for SYSTEM_SCOPED_POLICY_ID_ACE {}
impl ::core::default::Default for SYSTEM_SCOPED_POLICY_ID_ACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TOKEN_ACCESS_INFORMATION {
    pub SidHash: *mut SID_AND_ATTRIBUTES_HASH,
    pub RestrictedSidHash: *mut SID_AND_ATTRIBUTES_HASH,
    pub Privileges: *mut TOKEN_PRIVILEGES,
    pub AuthenticationId: super::Foundation::LUID,
    pub TokenType: TOKEN_TYPE,
    pub ImpersonationLevel: SECURITY_IMPERSONATION_LEVEL,
    pub MandatoryPolicy: TOKEN_MANDATORY_POLICY,
    pub Flags: u32,
    pub AppContainerNumber: u32,
    pub PackageSid: super::Foundation::PSID,
    pub CapabilitiesHash: *mut SID_AND_ATTRIBUTES_HASH,
    pub TrustLevelSid: super::Foundation::PSID,
    pub SecurityAttributes: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TOKEN_ACCESS_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TOKEN_ACCESS_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TOKEN_ACCESS_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TOKEN_ACCESS_INFORMATION")
            .field("SidHash", &self.SidHash)
            .field("RestrictedSidHash", &self.RestrictedSidHash)
            .field("Privileges", &self.Privileges)
            .field("AuthenticationId", &self.AuthenticationId)
            .field("TokenType", &self.TokenType)
            .field("ImpersonationLevel", &self.ImpersonationLevel)
            .field("MandatoryPolicy", &self.MandatoryPolicy)
            .field("Flags", &self.Flags)
            .field("AppContainerNumber", &self.AppContainerNumber)
            .field("PackageSid", &self.PackageSid)
            .field("CapabilitiesHash", &self.CapabilitiesHash)
            .field("TrustLevelSid", &self.TrustLevelSid)
            .field("SecurityAttributes", &self.SecurityAttributes)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for TOKEN_ACCESS_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TOKEN_ACCESS_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.SidHash == other.SidHash && self.RestrictedSidHash == other.RestrictedSidHash && self.Privileges == other.Privileges && self.AuthenticationId == other.AuthenticationId && self.TokenType == other.TokenType && self.ImpersonationLevel == other.ImpersonationLevel && self.MandatoryPolicy == other.MandatoryPolicy && self.Flags == other.Flags && self.AppContainerNumber == other.AppContainerNumber && self.PackageSid == other.PackageSid && self.CapabilitiesHash == other.CapabilitiesHash && self.TrustLevelSid == other.TrustLevelSid && self.SecurityAttributes == other.SecurityAttributes
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TOKEN_ACCESS_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TOKEN_ACCESS_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TOKEN_APPCONTAINER_INFORMATION {
    pub TokenAppContainer: super::Foundation::PSID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TOKEN_APPCONTAINER_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TOKEN_APPCONTAINER_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TOKEN_APPCONTAINER_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TOKEN_APPCONTAINER_INFORMATION").field("TokenAppContainer", &self.TokenAppContainer).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for TOKEN_APPCONTAINER_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TOKEN_APPCONTAINER_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.TokenAppContainer == other.TokenAppContainer
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TOKEN_APPCONTAINER_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TOKEN_APPCONTAINER_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub struct TOKEN_AUDIT_POLICY {
    pub PerUserPolicy: [u8; 30],
}
impl ::core::marker::Copy for TOKEN_AUDIT_POLICY {}
impl ::core::clone::Clone for TOKEN_AUDIT_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TOKEN_AUDIT_POLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TOKEN_AUDIT_POLICY").field("PerUserPolicy", &self.PerUserPolicy).finish()
    }
}
impl ::windows::core::TypeKind for TOKEN_AUDIT_POLICY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for TOKEN_AUDIT_POLICY {
    fn eq(&self, other: &Self) -> bool {
        self.PerUserPolicy == other.PerUserPolicy
    }
}
impl ::core::cmp::Eq for TOKEN_AUDIT_POLICY {}
impl ::core::default::Default for TOKEN_AUDIT_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TOKEN_CONTROL {
    pub TokenId: super::Foundation::LUID,
    pub AuthenticationId: super::Foundation::LUID,
    pub ModifiedId: super::Foundation::LUID,
    pub TokenSource: TOKEN_SOURCE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TOKEN_CONTROL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TOKEN_CONTROL {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TOKEN_CONTROL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TOKEN_CONTROL").field("TokenId", &self.TokenId).field("AuthenticationId", &self.AuthenticationId).field("ModifiedId", &self.ModifiedId).field("TokenSource", &self.TokenSource).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for TOKEN_CONTROL {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TOKEN_CONTROL {
    fn eq(&self, other: &Self) -> bool {
        self.TokenId == other.TokenId && self.AuthenticationId == other.AuthenticationId && self.ModifiedId == other.ModifiedId && self.TokenSource == other.TokenSource
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TOKEN_CONTROL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TOKEN_CONTROL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub struct TOKEN_DEFAULT_DACL {
    pub DefaultDacl: *mut ACL,
}
impl ::core::marker::Copy for TOKEN_DEFAULT_DACL {}
impl ::core::clone::Clone for TOKEN_DEFAULT_DACL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TOKEN_DEFAULT_DACL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TOKEN_DEFAULT_DACL").field("DefaultDacl", &self.DefaultDacl).finish()
    }
}
impl ::windows::core::TypeKind for TOKEN_DEFAULT_DACL {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for TOKEN_DEFAULT_DACL {
    fn eq(&self, other: &Self) -> bool {
        self.DefaultDacl == other.DefaultDacl
    }
}
impl ::core::cmp::Eq for TOKEN_DEFAULT_DACL {}
impl ::core::default::Default for TOKEN_DEFAULT_DACL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub struct TOKEN_DEVICE_CLAIMS {
    pub DeviceClaims: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for TOKEN_DEVICE_CLAIMS {}
impl ::core::clone::Clone for TOKEN_DEVICE_CLAIMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TOKEN_DEVICE_CLAIMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TOKEN_DEVICE_CLAIMS").field("DeviceClaims", &self.DeviceClaims).finish()
    }
}
impl ::windows::core::TypeKind for TOKEN_DEVICE_CLAIMS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for TOKEN_DEVICE_CLAIMS {
    fn eq(&self, other: &Self) -> bool {
        self.DeviceClaims == other.DeviceClaims
    }
}
impl ::core::cmp::Eq for TOKEN_DEVICE_CLAIMS {}
impl ::core::default::Default for TOKEN_DEVICE_CLAIMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub struct TOKEN_ELEVATION {
    pub TokenIsElevated: u32,
}
impl ::core::marker::Copy for TOKEN_ELEVATION {}
impl ::core::clone::Clone for TOKEN_ELEVATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TOKEN_ELEVATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TOKEN_ELEVATION").field("TokenIsElevated", &self.TokenIsElevated).finish()
    }
}
impl ::windows::core::TypeKind for TOKEN_ELEVATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for TOKEN_ELEVATION {
    fn eq(&self, other: &Self) -> bool {
        self.TokenIsElevated == other.TokenIsElevated
    }
}
impl ::core::cmp::Eq for TOKEN_ELEVATION {}
impl ::core::default::Default for TOKEN_ELEVATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TOKEN_GROUPS {
    pub GroupCount: u32,
    pub Groups: [SID_AND_ATTRIBUTES; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TOKEN_GROUPS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TOKEN_GROUPS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TOKEN_GROUPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TOKEN_GROUPS").field("GroupCount", &self.GroupCount).field("Groups", &self.Groups).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for TOKEN_GROUPS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TOKEN_GROUPS {
    fn eq(&self, other: &Self) -> bool {
        self.GroupCount == other.GroupCount && self.Groups == other.Groups
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TOKEN_GROUPS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TOKEN_GROUPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TOKEN_GROUPS_AND_PRIVILEGES {
    pub SidCount: u32,
    pub SidLength: u32,
    pub Sids: *mut SID_AND_ATTRIBUTES,
    pub RestrictedSidCount: u32,
    pub RestrictedSidLength: u32,
    pub RestrictedSids: *mut SID_AND_ATTRIBUTES,
    pub PrivilegeCount: u32,
    pub PrivilegeLength: u32,
    pub Privileges: *mut LUID_AND_ATTRIBUTES,
    pub AuthenticationId: super::Foundation::LUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TOKEN_GROUPS_AND_PRIVILEGES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TOKEN_GROUPS_AND_PRIVILEGES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TOKEN_GROUPS_AND_PRIVILEGES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TOKEN_GROUPS_AND_PRIVILEGES")
            .field("SidCount", &self.SidCount)
            .field("SidLength", &self.SidLength)
            .field("Sids", &self.Sids)
            .field("RestrictedSidCount", &self.RestrictedSidCount)
            .field("RestrictedSidLength", &self.RestrictedSidLength)
            .field("RestrictedSids", &self.RestrictedSids)
            .field("PrivilegeCount", &self.PrivilegeCount)
            .field("PrivilegeLength", &self.PrivilegeLength)
            .field("Privileges", &self.Privileges)
            .field("AuthenticationId", &self.AuthenticationId)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for TOKEN_GROUPS_AND_PRIVILEGES {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TOKEN_GROUPS_AND_PRIVILEGES {
    fn eq(&self, other: &Self) -> bool {
        self.SidCount == other.SidCount && self.SidLength == other.SidLength && self.Sids == other.Sids && self.RestrictedSidCount == other.RestrictedSidCount && self.RestrictedSidLength == other.RestrictedSidLength && self.RestrictedSids == other.RestrictedSids && self.PrivilegeCount == other.PrivilegeCount && self.PrivilegeLength == other.PrivilegeLength && self.Privileges == other.Privileges && self.AuthenticationId == other.AuthenticationId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TOKEN_GROUPS_AND_PRIVILEGES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TOKEN_GROUPS_AND_PRIVILEGES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TOKEN_LINKED_TOKEN {
    pub LinkedToken: super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TOKEN_LINKED_TOKEN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TOKEN_LINKED_TOKEN {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TOKEN_LINKED_TOKEN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TOKEN_LINKED_TOKEN").field("LinkedToken", &self.LinkedToken).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for TOKEN_LINKED_TOKEN {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TOKEN_LINKED_TOKEN {
    fn eq(&self, other: &Self) -> bool {
        self.LinkedToken == other.LinkedToken
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TOKEN_LINKED_TOKEN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TOKEN_LINKED_TOKEN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TOKEN_MANDATORY_LABEL {
    pub Label: SID_AND_ATTRIBUTES,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TOKEN_MANDATORY_LABEL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TOKEN_MANDATORY_LABEL {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TOKEN_MANDATORY_LABEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TOKEN_MANDATORY_LABEL").field("Label", &self.Label).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for TOKEN_MANDATORY_LABEL {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TOKEN_MANDATORY_LABEL {
    fn eq(&self, other: &Self) -> bool {
        self.Label == other.Label
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TOKEN_MANDATORY_LABEL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TOKEN_MANDATORY_LABEL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub struct TOKEN_MANDATORY_POLICY {
    pub Policy: TOKEN_MANDATORY_POLICY_ID,
}
impl ::core::marker::Copy for TOKEN_MANDATORY_POLICY {}
impl ::core::clone::Clone for TOKEN_MANDATORY_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TOKEN_MANDATORY_POLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TOKEN_MANDATORY_POLICY").field("Policy", &self.Policy).finish()
    }
}
impl ::windows::core::TypeKind for TOKEN_MANDATORY_POLICY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for TOKEN_MANDATORY_POLICY {
    fn eq(&self, other: &Self) -> bool {
        self.Policy == other.Policy
    }
}
impl ::core::cmp::Eq for TOKEN_MANDATORY_POLICY {}
impl ::core::default::Default for TOKEN_MANDATORY_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TOKEN_ORIGIN {
    pub OriginatingLogonSession: super::Foundation::LUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TOKEN_ORIGIN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TOKEN_ORIGIN {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TOKEN_ORIGIN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TOKEN_ORIGIN").field("OriginatingLogonSession", &self.OriginatingLogonSession).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for TOKEN_ORIGIN {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TOKEN_ORIGIN {
    fn eq(&self, other: &Self) -> bool {
        self.OriginatingLogonSession == other.OriginatingLogonSession
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TOKEN_ORIGIN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TOKEN_ORIGIN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TOKEN_OWNER {
    pub Owner: super::Foundation::PSID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TOKEN_OWNER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TOKEN_OWNER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TOKEN_OWNER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TOKEN_OWNER").field("Owner", &self.Owner).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for TOKEN_OWNER {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TOKEN_OWNER {
    fn eq(&self, other: &Self) -> bool {
        self.Owner == other.Owner
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TOKEN_OWNER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TOKEN_OWNER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TOKEN_PRIMARY_GROUP {
    pub PrimaryGroup: super::Foundation::PSID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TOKEN_PRIMARY_GROUP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TOKEN_PRIMARY_GROUP {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TOKEN_PRIMARY_GROUP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TOKEN_PRIMARY_GROUP").field("PrimaryGroup", &self.PrimaryGroup).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for TOKEN_PRIMARY_GROUP {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TOKEN_PRIMARY_GROUP {
    fn eq(&self, other: &Self) -> bool {
        self.PrimaryGroup == other.PrimaryGroup
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TOKEN_PRIMARY_GROUP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TOKEN_PRIMARY_GROUP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TOKEN_PRIVILEGES {
    pub PrivilegeCount: u32,
    pub Privileges: [LUID_AND_ATTRIBUTES; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TOKEN_PRIVILEGES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TOKEN_PRIVILEGES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TOKEN_PRIVILEGES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TOKEN_PRIVILEGES").field("PrivilegeCount", &self.PrivilegeCount).field("Privileges", &self.Privileges).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for TOKEN_PRIVILEGES {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TOKEN_PRIVILEGES {
    fn eq(&self, other: &Self) -> bool {
        self.PrivilegeCount == other.PrivilegeCount && self.Privileges == other.Privileges
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TOKEN_PRIVILEGES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TOKEN_PRIVILEGES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TOKEN_SOURCE {
    pub SourceName: [u8; 8],
    pub SourceIdentifier: super::Foundation::LUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TOKEN_SOURCE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TOKEN_SOURCE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TOKEN_SOURCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TOKEN_SOURCE").field("SourceName", &self.SourceName).field("SourceIdentifier", &self.SourceIdentifier).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for TOKEN_SOURCE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TOKEN_SOURCE {
    fn eq(&self, other: &Self) -> bool {
        self.SourceName == other.SourceName && self.SourceIdentifier == other.SourceIdentifier
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TOKEN_SOURCE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TOKEN_SOURCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TOKEN_STATISTICS {
    pub TokenId: super::Foundation::LUID,
    pub AuthenticationId: super::Foundation::LUID,
    pub ExpirationTime: i64,
    pub TokenType: TOKEN_TYPE,
    pub ImpersonationLevel: SECURITY_IMPERSONATION_LEVEL,
    pub DynamicCharged: u32,
    pub DynamicAvailable: u32,
    pub GroupCount: u32,
    pub PrivilegeCount: u32,
    pub ModifiedId: super::Foundation::LUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TOKEN_STATISTICS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TOKEN_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TOKEN_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TOKEN_STATISTICS").field("TokenId", &self.TokenId).field("AuthenticationId", &self.AuthenticationId).field("ExpirationTime", &self.ExpirationTime).field("TokenType", &self.TokenType).field("ImpersonationLevel", &self.ImpersonationLevel).field("DynamicCharged", &self.DynamicCharged).field("DynamicAvailable", &self.DynamicAvailable).field("GroupCount", &self.GroupCount).field("PrivilegeCount", &self.PrivilegeCount).field("ModifiedId", &self.ModifiedId).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for TOKEN_STATISTICS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TOKEN_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        self.TokenId == other.TokenId && self.AuthenticationId == other.AuthenticationId && self.ExpirationTime == other.ExpirationTime && self.TokenType == other.TokenType && self.ImpersonationLevel == other.ImpersonationLevel && self.DynamicCharged == other.DynamicCharged && self.DynamicAvailable == other.DynamicAvailable && self.GroupCount == other.GroupCount && self.PrivilegeCount == other.PrivilegeCount && self.ModifiedId == other.ModifiedId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TOKEN_STATISTICS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TOKEN_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TOKEN_USER {
    pub User: SID_AND_ATTRIBUTES,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TOKEN_USER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TOKEN_USER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TOKEN_USER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TOKEN_USER").field("User", &self.User).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for TOKEN_USER {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TOKEN_USER {
    fn eq(&self, other: &Self) -> bool {
        self.User == other.User
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TOKEN_USER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TOKEN_USER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub struct TOKEN_USER_CLAIMS {
    pub UserClaims: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for TOKEN_USER_CLAIMS {}
impl ::core::clone::Clone for TOKEN_USER_CLAIMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TOKEN_USER_CLAIMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TOKEN_USER_CLAIMS").field("UserClaims", &self.UserClaims).finish()
    }
}
impl ::windows::core::TypeKind for TOKEN_USER_CLAIMS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for TOKEN_USER_CLAIMS {
    fn eq(&self, other: &Self) -> bool {
        self.UserClaims == other.UserClaims
    }
}
impl ::core::cmp::Eq for TOKEN_USER_CLAIMS {}
impl ::core::default::Default for TOKEN_USER_CLAIMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PLSA_AP_CALL_PACKAGE_UNTRUSTED = ::core::option::Option<unsafe extern "system" fn(clientrequest: *const *const ::core::ffi::c_void, protocolsubmitbuffer: *const ::core::ffi::c_void, clientbufferbase: *const ::core::ffi::c_void, submitbufferlength: u32, protocolreturnbuffer: *mut *mut ::core::ffi::c_void, returnbufferlength: *mut u32, protocolstatus: *mut i32) -> super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_Security\"`*"]
pub type SEC_THREAD_START = ::core::option::Option<unsafe extern "system" fn(lpthreadparameter: *mut ::core::ffi::c_void) -> u32>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
