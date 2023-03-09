#[cfg(feature = "Win32_Security_Authorization_UI")]
pub mod UI;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AuthzAccessCheck<P0, P1, P2>(flags: AUTHZ_ACCESS_CHECK_FLAGS, hauthzclientcontext: P0, prequest: *const AUTHZ_ACCESS_REQUEST, hauditevent: P1, psecuritydescriptor: P2, optionalsecuritydescriptorarray: ::core::option::Option<&[super::PSECURITY_DESCRIPTOR]>, preply: *mut AUTHZ_ACCESS_REPLY, phaccesscheckresults: ::core::option::Option<*mut isize>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<AUTHZ_CLIENT_CONTEXT_HANDLE>,
    P1: ::windows::core::IntoParam<AUTHZ_AUDIT_EVENT_HANDLE>,
    P2: ::windows::core::IntoParam<super::PSECURITY_DESCRIPTOR>,
{
    ::windows::imp::link ! ( "authz.dll""system" fn AuthzAccessCheck ( flags : AUTHZ_ACCESS_CHECK_FLAGS , hauthzclientcontext : AUTHZ_CLIENT_CONTEXT_HANDLE , prequest : *const AUTHZ_ACCESS_REQUEST , hauditevent : AUTHZ_AUDIT_EVENT_HANDLE , psecuritydescriptor : super:: PSECURITY_DESCRIPTOR , optionalsecuritydescriptorarray : *const super:: PSECURITY_DESCRIPTOR , optionalsecuritydescriptorcount : u32 , preply : *mut AUTHZ_ACCESS_REPLY , phaccesscheckresults : *mut isize ) -> super::super::Foundation:: BOOL );
    AuthzAccessCheck(flags, hauthzclientcontext.into_param().abi(), prequest, hauditevent.into_param().abi(), psecuritydescriptor.into_param().abi(), ::core::mem::transmute(optionalsecuritydescriptorarray.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), optionalsecuritydescriptorarray.as_deref().map_or(0, |slice| slice.len() as _), preply, ::core::mem::transmute(phaccesscheckresults.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AuthzAddSidsToContext<P0>(hauthzclientcontext: P0, sids: ::core::option::Option<*const super::SID_AND_ATTRIBUTES>, sidcount: u32, restrictedsids: ::core::option::Option<*const super::SID_AND_ATTRIBUTES>, restrictedsidcount: u32, phnewauthzclientcontext: *mut AUTHZ_CLIENT_CONTEXT_HANDLE) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<AUTHZ_CLIENT_CONTEXT_HANDLE>,
{
    ::windows::imp::link ! ( "authz.dll""system" fn AuthzAddSidsToContext ( hauthzclientcontext : AUTHZ_CLIENT_CONTEXT_HANDLE , sids : *const super:: SID_AND_ATTRIBUTES , sidcount : u32 , restrictedsids : *const super:: SID_AND_ATTRIBUTES , restrictedsidcount : u32 , phnewauthzclientcontext : *mut AUTHZ_CLIENT_CONTEXT_HANDLE ) -> super::super::Foundation:: BOOL );
    AuthzAddSidsToContext(hauthzclientcontext.into_param().abi(), ::core::mem::transmute(sids.unwrap_or(::std::ptr::null())), sidcount, ::core::mem::transmute(restrictedsids.unwrap_or(::std::ptr::null())), restrictedsidcount, phnewauthzclientcontext)
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AuthzCachedAccessCheck<P0, P1>(flags: u32, haccesscheckresults: P0, prequest: *const AUTHZ_ACCESS_REQUEST, hauditevent: P1, preply: *mut AUTHZ_ACCESS_REPLY) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<AUTHZ_ACCESS_CHECK_RESULTS_HANDLE>,
    P1: ::windows::core::IntoParam<AUTHZ_AUDIT_EVENT_HANDLE>,
{
    ::windows::imp::link ! ( "authz.dll""system" fn AuthzCachedAccessCheck ( flags : u32 , haccesscheckresults : AUTHZ_ACCESS_CHECK_RESULTS_HANDLE , prequest : *const AUTHZ_ACCESS_REQUEST , hauditevent : AUTHZ_AUDIT_EVENT_HANDLE , preply : *mut AUTHZ_ACCESS_REPLY ) -> super::super::Foundation:: BOOL );
    AuthzCachedAccessCheck(flags, haccesscheckresults.into_param().abi(), prequest, hauditevent.into_param().abi(), preply)
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AuthzEnumerateSecurityEventSources(dwflags: u32, buffer: *mut AUTHZ_SOURCE_SCHEMA_REGISTRATION, pdwcount: *mut u32, pdwlength: *mut u32) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "authz.dll""system" fn AuthzEnumerateSecurityEventSources ( dwflags : u32 , buffer : *mut AUTHZ_SOURCE_SCHEMA_REGISTRATION , pdwcount : *mut u32 , pdwlength : *mut u32 ) -> super::super::Foundation:: BOOL );
    AuthzEnumerateSecurityEventSources(dwflags, buffer, pdwcount, pdwlength)
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AuthzEvaluateSacl<P0, P1>(authzclientcontext: P0, prequest: *const AUTHZ_ACCESS_REQUEST, sacl: *const super::ACL, grantedaccess: u32, accessgranted: P1, pbgenerateaudit: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<AUTHZ_CLIENT_CONTEXT_HANDLE>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "authz.dll""system" fn AuthzEvaluateSacl ( authzclientcontext : AUTHZ_CLIENT_CONTEXT_HANDLE , prequest : *const AUTHZ_ACCESS_REQUEST , sacl : *const super:: ACL , grantedaccess : u32 , accessgranted : super::super::Foundation:: BOOL , pbgenerateaudit : *mut super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
    AuthzEvaluateSacl(authzclientcontext.into_param().abi(), prequest, sacl, grantedaccess, accessgranted.into_param().abi(), pbgenerateaudit)
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AuthzFreeAuditEvent<P0>(hauditevent: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<AUTHZ_AUDIT_EVENT_HANDLE>,
{
    ::windows::imp::link ! ( "authz.dll""system" fn AuthzFreeAuditEvent ( hauditevent : AUTHZ_AUDIT_EVENT_HANDLE ) -> super::super::Foundation:: BOOL );
    AuthzFreeAuditEvent(hauditevent.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AuthzFreeCentralAccessPolicyCache() -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "authz.dll""system" fn AuthzFreeCentralAccessPolicyCache ( ) -> super::super::Foundation:: BOOL );
    AuthzFreeCentralAccessPolicyCache()
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AuthzFreeContext<P0>(hauthzclientcontext: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<AUTHZ_CLIENT_CONTEXT_HANDLE>,
{
    ::windows::imp::link ! ( "authz.dll""system" fn AuthzFreeContext ( hauthzclientcontext : AUTHZ_CLIENT_CONTEXT_HANDLE ) -> super::super::Foundation:: BOOL );
    AuthzFreeContext(hauthzclientcontext.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AuthzFreeHandle<P0>(haccesscheckresults: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<AUTHZ_ACCESS_CHECK_RESULTS_HANDLE>,
{
    ::windows::imp::link ! ( "authz.dll""system" fn AuthzFreeHandle ( haccesscheckresults : AUTHZ_ACCESS_CHECK_RESULTS_HANDLE ) -> super::super::Foundation:: BOOL );
    AuthzFreeHandle(haccesscheckresults.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AuthzFreeResourceManager<P0>(hauthzresourcemanager: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<AUTHZ_RESOURCE_MANAGER_HANDLE>,
{
    ::windows::imp::link ! ( "authz.dll""system" fn AuthzFreeResourceManager ( hauthzresourcemanager : AUTHZ_RESOURCE_MANAGER_HANDLE ) -> super::super::Foundation:: BOOL );
    AuthzFreeResourceManager(hauthzresourcemanager.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AuthzGetInformationFromContext<P0>(hauthzclientcontext: P0, infoclass: AUTHZ_CONTEXT_INFORMATION_CLASS, buffersize: u32, psizerequired: *mut u32, buffer: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<AUTHZ_CLIENT_CONTEXT_HANDLE>,
{
    ::windows::imp::link ! ( "authz.dll""system" fn AuthzGetInformationFromContext ( hauthzclientcontext : AUTHZ_CLIENT_CONTEXT_HANDLE , infoclass : AUTHZ_CONTEXT_INFORMATION_CLASS , buffersize : u32 , psizerequired : *mut u32 , buffer : *mut ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    AuthzGetInformationFromContext(hauthzclientcontext.into_param().abi(), infoclass, buffersize, psizerequired, buffer)
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AuthzInitializeCompoundContext<P0, P1>(usercontext: P0, devicecontext: P1, phcompoundcontext: *mut AUTHZ_CLIENT_CONTEXT_HANDLE) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<AUTHZ_CLIENT_CONTEXT_HANDLE>,
    P1: ::windows::core::IntoParam<AUTHZ_CLIENT_CONTEXT_HANDLE>,
{
    ::windows::imp::link ! ( "authz.dll""system" fn AuthzInitializeCompoundContext ( usercontext : AUTHZ_CLIENT_CONTEXT_HANDLE , devicecontext : AUTHZ_CLIENT_CONTEXT_HANDLE , phcompoundcontext : *mut AUTHZ_CLIENT_CONTEXT_HANDLE ) -> super::super::Foundation:: BOOL );
    AuthzInitializeCompoundContext(usercontext.into_param().abi(), devicecontext.into_param().abi(), phcompoundcontext)
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AuthzInitializeContextFromAuthzContext<P0>(flags: u32, hauthzclientcontext: P0, pexpirationtime: ::core::option::Option<*const i64>, identifier: super::super::Foundation::LUID, dynamicgroupargs: *const ::core::ffi::c_void, phnewauthzclientcontext: *mut AUTHZ_CLIENT_CONTEXT_HANDLE) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<AUTHZ_CLIENT_CONTEXT_HANDLE>,
{
    ::windows::imp::link ! ( "authz.dll""system" fn AuthzInitializeContextFromAuthzContext ( flags : u32 , hauthzclientcontext : AUTHZ_CLIENT_CONTEXT_HANDLE , pexpirationtime : *const i64 , identifier : super::super::Foundation:: LUID , dynamicgroupargs : *const ::core::ffi::c_void , phnewauthzclientcontext : *mut AUTHZ_CLIENT_CONTEXT_HANDLE ) -> super::super::Foundation:: BOOL );
    AuthzInitializeContextFromAuthzContext(flags, hauthzclientcontext.into_param().abi(), ::core::mem::transmute(pexpirationtime.unwrap_or(::std::ptr::null())), ::core::mem::transmute(identifier), dynamicgroupargs, phnewauthzclientcontext)
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AuthzInitializeContextFromSid<P0, P1>(flags: u32, usersid: P0, hauthzresourcemanager: P1, pexpirationtime: ::core::option::Option<*const i64>, identifier: super::super::Foundation::LUID, dynamicgroupargs: ::core::option::Option<*const ::core::ffi::c_void>, phauthzclientcontext: *mut AUTHZ_CLIENT_CONTEXT_HANDLE) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::PSID>,
    P1: ::windows::core::IntoParam<AUTHZ_RESOURCE_MANAGER_HANDLE>,
{
    ::windows::imp::link ! ( "authz.dll""system" fn AuthzInitializeContextFromSid ( flags : u32 , usersid : super::super::Foundation:: PSID , hauthzresourcemanager : AUTHZ_RESOURCE_MANAGER_HANDLE , pexpirationtime : *const i64 , identifier : super::super::Foundation:: LUID , dynamicgroupargs : *const ::core::ffi::c_void , phauthzclientcontext : *mut AUTHZ_CLIENT_CONTEXT_HANDLE ) -> super::super::Foundation:: BOOL );
    AuthzInitializeContextFromSid(flags, usersid.into_param().abi(), hauthzresourcemanager.into_param().abi(), ::core::mem::transmute(pexpirationtime.unwrap_or(::std::ptr::null())), ::core::mem::transmute(identifier), ::core::mem::transmute(dynamicgroupargs.unwrap_or(::std::ptr::null())), phauthzclientcontext)
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AuthzInitializeContextFromToken<P0, P1>(flags: u32, tokenhandle: P0, hauthzresourcemanager: P1, pexpirationtime: ::core::option::Option<*const i64>, identifier: super::super::Foundation::LUID, dynamicgroupargs: ::core::option::Option<*const ::core::ffi::c_void>, phauthzclientcontext: *mut AUTHZ_CLIENT_CONTEXT_HANDLE) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<AUTHZ_RESOURCE_MANAGER_HANDLE>,
{
    ::windows::imp::link ! ( "authz.dll""system" fn AuthzInitializeContextFromToken ( flags : u32 , tokenhandle : super::super::Foundation:: HANDLE , hauthzresourcemanager : AUTHZ_RESOURCE_MANAGER_HANDLE , pexpirationtime : *const i64 , identifier : super::super::Foundation:: LUID , dynamicgroupargs : *const ::core::ffi::c_void , phauthzclientcontext : *mut AUTHZ_CLIENT_CONTEXT_HANDLE ) -> super::super::Foundation:: BOOL );
    AuthzInitializeContextFromToken(flags, tokenhandle.into_param().abi(), hauthzresourcemanager.into_param().abi(), ::core::mem::transmute(pexpirationtime.unwrap_or(::std::ptr::null())), ::core::mem::transmute(identifier), ::core::mem::transmute(dynamicgroupargs.unwrap_or(::std::ptr::null())), phauthzclientcontext)
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AuthzInitializeObjectAccessAuditEvent<P0, P1, P2, P3, P4>(flags: AUTHZ_INITIALIZE_OBJECT_ACCESS_AUDIT_EVENT_FLAGS, hauditeventtype: P0, szoperationtype: P1, szobjecttype: P2, szobjectname: P3, szadditionalinfo: P4, phauditevent: *mut isize, dwadditionalparametercount: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<AUTHZ_AUDIT_EVENT_TYPE_HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P4: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "authz.dll""cdecl" fn AuthzInitializeObjectAccessAuditEvent ( flags : AUTHZ_INITIALIZE_OBJECT_ACCESS_AUDIT_EVENT_FLAGS , hauditeventtype : AUTHZ_AUDIT_EVENT_TYPE_HANDLE , szoperationtype : :: windows::core::PCWSTR , szobjecttype : :: windows::core::PCWSTR , szobjectname : :: windows::core::PCWSTR , szadditionalinfo : :: windows::core::PCWSTR , phauditevent : *mut isize , dwadditionalparametercount : u32 ) -> super::super::Foundation:: BOOL );
    AuthzInitializeObjectAccessAuditEvent(flags, hauditeventtype.into_param().abi(), szoperationtype.into_param().abi(), szobjecttype.into_param().abi(), szobjectname.into_param().abi(), szadditionalinfo.into_param().abi(), phauditevent, dwadditionalparametercount)
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AuthzInitializeObjectAccessAuditEvent2<P0, P1, P2, P3, P4, P5>(flags: u32, hauditeventtype: P0, szoperationtype: P1, szobjecttype: P2, szobjectname: P3, szadditionalinfo: P4, szadditionalinfo2: P5, phauditevent: *mut isize, dwadditionalparametercount: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<AUTHZ_AUDIT_EVENT_TYPE_HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P4: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P5: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "authz.dll""cdecl" fn AuthzInitializeObjectAccessAuditEvent2 ( flags : u32 , hauditeventtype : AUTHZ_AUDIT_EVENT_TYPE_HANDLE , szoperationtype : :: windows::core::PCWSTR , szobjecttype : :: windows::core::PCWSTR , szobjectname : :: windows::core::PCWSTR , szadditionalinfo : :: windows::core::PCWSTR , szadditionalinfo2 : :: windows::core::PCWSTR , phauditevent : *mut isize , dwadditionalparametercount : u32 ) -> super::super::Foundation:: BOOL );
    AuthzInitializeObjectAccessAuditEvent2(flags, hauditeventtype.into_param().abi(), szoperationtype.into_param().abi(), szobjecttype.into_param().abi(), szobjectname.into_param().abi(), szadditionalinfo.into_param().abi(), szadditionalinfo2.into_param().abi(), phauditevent, dwadditionalparametercount)
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AuthzInitializeRemoteResourceManager(prpcinitinfo: *const AUTHZ_RPC_INIT_INFO_CLIENT, phauthzresourcemanager: *mut AUTHZ_RESOURCE_MANAGER_HANDLE) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "authz.dll""system" fn AuthzInitializeRemoteResourceManager ( prpcinitinfo : *const AUTHZ_RPC_INIT_INFO_CLIENT , phauthzresourcemanager : *mut AUTHZ_RESOURCE_MANAGER_HANDLE ) -> super::super::Foundation:: BOOL );
    AuthzInitializeRemoteResourceManager(prpcinitinfo, phauthzresourcemanager)
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AuthzInitializeResourceManager<P0>(flags: u32, pfndynamicaccesscheck: PFN_AUTHZ_DYNAMIC_ACCESS_CHECK, pfncomputedynamicgroups: PFN_AUTHZ_COMPUTE_DYNAMIC_GROUPS, pfnfreedynamicgroups: PFN_AUTHZ_FREE_DYNAMIC_GROUPS, szresourcemanagername: P0, phauthzresourcemanager: *mut AUTHZ_RESOURCE_MANAGER_HANDLE) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "authz.dll""system" fn AuthzInitializeResourceManager ( flags : u32 , pfndynamicaccesscheck : PFN_AUTHZ_DYNAMIC_ACCESS_CHECK , pfncomputedynamicgroups : PFN_AUTHZ_COMPUTE_DYNAMIC_GROUPS , pfnfreedynamicgroups : PFN_AUTHZ_FREE_DYNAMIC_GROUPS , szresourcemanagername : :: windows::core::PCWSTR , phauthzresourcemanager : *mut AUTHZ_RESOURCE_MANAGER_HANDLE ) -> super::super::Foundation:: BOOL );
    AuthzInitializeResourceManager(flags, pfndynamicaccesscheck, pfncomputedynamicgroups, pfnfreedynamicgroups, szresourcemanagername.into_param().abi(), phauthzresourcemanager)
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AuthzInitializeResourceManagerEx(flags: AUTHZ_RESOURCE_MANAGER_FLAGS, pauthzinitinfo: ::core::option::Option<*const AUTHZ_INIT_INFO>, phauthzresourcemanager: *mut AUTHZ_RESOURCE_MANAGER_HANDLE) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "authz.dll""system" fn AuthzInitializeResourceManagerEx ( flags : AUTHZ_RESOURCE_MANAGER_FLAGS , pauthzinitinfo : *const AUTHZ_INIT_INFO , phauthzresourcemanager : *mut AUTHZ_RESOURCE_MANAGER_HANDLE ) -> super::super::Foundation:: BOOL );
    AuthzInitializeResourceManagerEx(flags, ::core::mem::transmute(pauthzinitinfo.unwrap_or(::std::ptr::null())), phauthzresourcemanager)
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AuthzInstallSecurityEventSource(dwflags: u32, pregistration: *const AUTHZ_SOURCE_SCHEMA_REGISTRATION) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "authz.dll""system" fn AuthzInstallSecurityEventSource ( dwflags : u32 , pregistration : *const AUTHZ_SOURCE_SCHEMA_REGISTRATION ) -> super::super::Foundation:: BOOL );
    AuthzInstallSecurityEventSource(dwflags, pregistration)
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AuthzModifyClaims<P0>(hauthzclientcontext: P0, claimclass: AUTHZ_CONTEXT_INFORMATION_CLASS, pclaimoperations: *const AUTHZ_SECURITY_ATTRIBUTE_OPERATION, pclaims: ::core::option::Option<*const AUTHZ_SECURITY_ATTRIBUTES_INFORMATION>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<AUTHZ_CLIENT_CONTEXT_HANDLE>,
{
    ::windows::imp::link ! ( "authz.dll""system" fn AuthzModifyClaims ( hauthzclientcontext : AUTHZ_CLIENT_CONTEXT_HANDLE , claimclass : AUTHZ_CONTEXT_INFORMATION_CLASS , pclaimoperations : *const AUTHZ_SECURITY_ATTRIBUTE_OPERATION , pclaims : *const AUTHZ_SECURITY_ATTRIBUTES_INFORMATION ) -> super::super::Foundation:: BOOL );
    AuthzModifyClaims(hauthzclientcontext.into_param().abi(), claimclass, pclaimoperations, ::core::mem::transmute(pclaims.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AuthzModifySecurityAttributes<P0>(hauthzclientcontext: P0, poperations: *const AUTHZ_SECURITY_ATTRIBUTE_OPERATION, pattributes: ::core::option::Option<*const AUTHZ_SECURITY_ATTRIBUTES_INFORMATION>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<AUTHZ_CLIENT_CONTEXT_HANDLE>,
{
    ::windows::imp::link ! ( "authz.dll""system" fn AuthzModifySecurityAttributes ( hauthzclientcontext : AUTHZ_CLIENT_CONTEXT_HANDLE , poperations : *const AUTHZ_SECURITY_ATTRIBUTE_OPERATION , pattributes : *const AUTHZ_SECURITY_ATTRIBUTES_INFORMATION ) -> super::super::Foundation:: BOOL );
    AuthzModifySecurityAttributes(hauthzclientcontext.into_param().abi(), poperations, ::core::mem::transmute(pattributes.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AuthzModifySids<P0>(hauthzclientcontext: P0, sidclass: AUTHZ_CONTEXT_INFORMATION_CLASS, psidoperations: *const AUTHZ_SID_OPERATION, psids: ::core::option::Option<*const super::TOKEN_GROUPS>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<AUTHZ_CLIENT_CONTEXT_HANDLE>,
{
    ::windows::imp::link ! ( "authz.dll""system" fn AuthzModifySids ( hauthzclientcontext : AUTHZ_CLIENT_CONTEXT_HANDLE , sidclass : AUTHZ_CONTEXT_INFORMATION_CLASS , psidoperations : *const AUTHZ_SID_OPERATION , psids : *const super:: TOKEN_GROUPS ) -> super::super::Foundation:: BOOL );
    AuthzModifySids(hauthzclientcontext.into_param().abi(), sidclass, psidoperations, ::core::mem::transmute(psids.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AuthzOpenObjectAudit<P0, P1, P2>(flags: u32, hauthzclientcontext: P0, prequest: *const AUTHZ_ACCESS_REQUEST, hauditevent: P1, psecuritydescriptor: P2, optionalsecuritydescriptorarray: ::core::option::Option<&[super::PSECURITY_DESCRIPTOR]>, preply: *const AUTHZ_ACCESS_REPLY) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<AUTHZ_CLIENT_CONTEXT_HANDLE>,
    P1: ::windows::core::IntoParam<AUTHZ_AUDIT_EVENT_HANDLE>,
    P2: ::windows::core::IntoParam<super::PSECURITY_DESCRIPTOR>,
{
    ::windows::imp::link ! ( "authz.dll""system" fn AuthzOpenObjectAudit ( flags : u32 , hauthzclientcontext : AUTHZ_CLIENT_CONTEXT_HANDLE , prequest : *const AUTHZ_ACCESS_REQUEST , hauditevent : AUTHZ_AUDIT_EVENT_HANDLE , psecuritydescriptor : super:: PSECURITY_DESCRIPTOR , optionalsecuritydescriptorarray : *const super:: PSECURITY_DESCRIPTOR , optionalsecuritydescriptorcount : u32 , preply : *const AUTHZ_ACCESS_REPLY ) -> super::super::Foundation:: BOOL );
    AuthzOpenObjectAudit(flags, hauthzclientcontext.into_param().abi(), prequest, hauditevent.into_param().abi(), psecuritydescriptor.into_param().abi(), ::core::mem::transmute(optionalsecuritydescriptorarray.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), optionalsecuritydescriptorarray.as_deref().map_or(0, |slice| slice.len() as _), preply)
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`, `\"Win32_System_Threading\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
#[inline]
pub unsafe fn AuthzRegisterCapChangeNotification(phcapchangesubscription: *mut *mut AUTHZ_CAP_CHANGE_SUBSCRIPTION_HANDLE__, pfncapchangecallback: super::super::System::Threading::LPTHREAD_START_ROUTINE, pcallbackcontext: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "authz.dll""system" fn AuthzRegisterCapChangeNotification ( phcapchangesubscription : *mut *mut AUTHZ_CAP_CHANGE_SUBSCRIPTION_HANDLE__ , pfncapchangecallback : super::super::System::Threading:: LPTHREAD_START_ROUTINE , pcallbackcontext : *const ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    AuthzRegisterCapChangeNotification(phcapchangesubscription, pfncapchangecallback, ::core::mem::transmute(pcallbackcontext.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AuthzRegisterSecurityEventSource<P0>(dwflags: u32, szeventsourcename: P0, pheventprovider: *mut isize) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "authz.dll""system" fn AuthzRegisterSecurityEventSource ( dwflags : u32 , szeventsourcename : :: windows::core::PCWSTR , pheventprovider : *mut isize ) -> super::super::Foundation:: BOOL );
    AuthzRegisterSecurityEventSource(dwflags, szeventsourcename.into_param().abi(), pheventprovider)
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AuthzReportSecurityEvent<P0, P1>(dwflags: u32, heventprovider: P0, dwauditid: u32, pusersid: P1, dwcount: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE>,
    P1: ::windows::core::IntoParam<super::super::Foundation::PSID>,
{
    ::windows::imp::link ! ( "authz.dll""cdecl" fn AuthzReportSecurityEvent ( dwflags : u32 , heventprovider : AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE , dwauditid : u32 , pusersid : super::super::Foundation:: PSID , dwcount : u32 ) -> super::super::Foundation:: BOOL );
    AuthzReportSecurityEvent(dwflags, heventprovider.into_param().abi(), dwauditid, pusersid.into_param().abi(), dwcount)
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AuthzReportSecurityEventFromParams<P0, P1>(dwflags: u32, heventprovider: P0, dwauditid: u32, pusersid: P1, pparams: *const AUDIT_PARAMS) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE>,
    P1: ::windows::core::IntoParam<super::super::Foundation::PSID>,
{
    ::windows::imp::link ! ( "authz.dll""system" fn AuthzReportSecurityEventFromParams ( dwflags : u32 , heventprovider : AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE , dwauditid : u32 , pusersid : super::super::Foundation:: PSID , pparams : *const AUDIT_PARAMS ) -> super::super::Foundation:: BOOL );
    AuthzReportSecurityEventFromParams(dwflags, heventprovider.into_param().abi(), dwauditid, pusersid.into_param().abi(), pparams)
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AuthzSetAppContainerInformation<P0, P1>(hauthzclientcontext: P0, pappcontainersid: P1, pcapabilitysids: ::core::option::Option<&[super::SID_AND_ATTRIBUTES]>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<AUTHZ_CLIENT_CONTEXT_HANDLE>,
    P1: ::windows::core::IntoParam<super::super::Foundation::PSID>,
{
    ::windows::imp::link ! ( "authz.dll""system" fn AuthzSetAppContainerInformation ( hauthzclientcontext : AUTHZ_CLIENT_CONTEXT_HANDLE , pappcontainersid : super::super::Foundation:: PSID , capabilitycount : u32 , pcapabilitysids : *const super:: SID_AND_ATTRIBUTES ) -> super::super::Foundation:: BOOL );
    AuthzSetAppContainerInformation(hauthzclientcontext.into_param().abi(), pappcontainersid.into_param().abi(), pcapabilitysids.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pcapabilitysids.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AuthzUninstallSecurityEventSource<P0>(dwflags: u32, szeventsourcename: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "authz.dll""system" fn AuthzUninstallSecurityEventSource ( dwflags : u32 , szeventsourcename : :: windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    AuthzUninstallSecurityEventSource(dwflags, szeventsourcename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AuthzUnregisterCapChangeNotification(hcapchangesubscription: *const AUTHZ_CAP_CHANGE_SUBSCRIPTION_HANDLE__) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "authz.dll""system" fn AuthzUnregisterCapChangeNotification ( hcapchangesubscription : *const AUTHZ_CAP_CHANGE_SUBSCRIPTION_HANDLE__ ) -> super::super::Foundation:: BOOL );
    AuthzUnregisterCapChangeNotification(hcapchangesubscription)
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AuthzUnregisterSecurityEventSource(dwflags: u32, pheventprovider: *mut isize) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "authz.dll""system" fn AuthzUnregisterSecurityEventSource ( dwflags : u32 , pheventprovider : *mut isize ) -> super::super::Foundation:: BOOL );
    AuthzUnregisterSecurityEventSource(dwflags, pheventprovider)
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
#[inline]
pub unsafe fn BuildExplicitAccessWithNameA<P0>(pexplicitaccess: *mut EXPLICIT_ACCESS_A, ptrusteename: P0, accesspermissions: u32, accessmode: ACCESS_MODE, inheritance: super::ACE_FLAGS)
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn BuildExplicitAccessWithNameA ( pexplicitaccess : *mut EXPLICIT_ACCESS_A , ptrusteename : :: windows::core::PCSTR , accesspermissions : u32 , accessmode : ACCESS_MODE , inheritance : super:: ACE_FLAGS ) -> ( ) );
    BuildExplicitAccessWithNameA(pexplicitaccess, ptrusteename.into_param().abi(), accesspermissions, accessmode, inheritance)
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
#[inline]
pub unsafe fn BuildExplicitAccessWithNameW<P0>(pexplicitaccess: *mut EXPLICIT_ACCESS_W, ptrusteename: P0, accesspermissions: u32, accessmode: ACCESS_MODE, inheritance: super::ACE_FLAGS)
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn BuildExplicitAccessWithNameW ( pexplicitaccess : *mut EXPLICIT_ACCESS_W , ptrusteename : :: windows::core::PCWSTR , accesspermissions : u32 , accessmode : ACCESS_MODE , inheritance : super:: ACE_FLAGS ) -> ( ) );
    BuildExplicitAccessWithNameW(pexplicitaccess, ptrusteename.into_param().abi(), accesspermissions, accessmode, inheritance)
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
#[inline]
pub unsafe fn BuildImpersonateExplicitAccessWithNameA<P0>(pexplicitaccess: *mut EXPLICIT_ACCESS_A, ptrusteename: P0, ptrustee: ::core::option::Option<*const TRUSTEE_A>, accesspermissions: u32, accessmode: ACCESS_MODE, inheritance: u32)
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn BuildImpersonateExplicitAccessWithNameA ( pexplicitaccess : *mut EXPLICIT_ACCESS_A , ptrusteename : :: windows::core::PCSTR , ptrustee : *const TRUSTEE_A , accesspermissions : u32 , accessmode : ACCESS_MODE , inheritance : u32 ) -> ( ) );
    BuildImpersonateExplicitAccessWithNameA(pexplicitaccess, ptrusteename.into_param().abi(), ::core::mem::transmute(ptrustee.unwrap_or(::std::ptr::null())), accesspermissions, accessmode, inheritance)
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
#[inline]
pub unsafe fn BuildImpersonateExplicitAccessWithNameW<P0>(pexplicitaccess: *mut EXPLICIT_ACCESS_W, ptrusteename: P0, ptrustee: ::core::option::Option<*const TRUSTEE_W>, accesspermissions: u32, accessmode: ACCESS_MODE, inheritance: u32)
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn BuildImpersonateExplicitAccessWithNameW ( pexplicitaccess : *mut EXPLICIT_ACCESS_W , ptrusteename : :: windows::core::PCWSTR , ptrustee : *const TRUSTEE_W , accesspermissions : u32 , accessmode : ACCESS_MODE , inheritance : u32 ) -> ( ) );
    BuildImpersonateExplicitAccessWithNameW(pexplicitaccess, ptrusteename.into_param().abi(), ::core::mem::transmute(ptrustee.unwrap_or(::std::ptr::null())), accesspermissions, accessmode, inheritance)
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
#[inline]
pub unsafe fn BuildImpersonateTrusteeA(ptrustee: *mut TRUSTEE_A, pimpersonatetrustee: ::core::option::Option<*const TRUSTEE_A>) {
    ::windows::imp::link ! ( "advapi32.dll""system" fn BuildImpersonateTrusteeA ( ptrustee : *mut TRUSTEE_A , pimpersonatetrustee : *const TRUSTEE_A ) -> ( ) );
    BuildImpersonateTrusteeA(ptrustee, ::core::mem::transmute(pimpersonatetrustee.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
#[inline]
pub unsafe fn BuildImpersonateTrusteeW(ptrustee: *mut TRUSTEE_W, pimpersonatetrustee: ::core::option::Option<*const TRUSTEE_W>) {
    ::windows::imp::link ! ( "advapi32.dll""system" fn BuildImpersonateTrusteeW ( ptrustee : *mut TRUSTEE_W , pimpersonatetrustee : *const TRUSTEE_W ) -> ( ) );
    BuildImpersonateTrusteeW(ptrustee, ::core::mem::transmute(pimpersonatetrustee.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BuildSecurityDescriptorA<P0>(powner: ::core::option::Option<*const TRUSTEE_A>, pgroup: ::core::option::Option<*const TRUSTEE_A>, plistofaccessentries: ::core::option::Option<&[EXPLICIT_ACCESS_A]>, plistofauditentries: ::core::option::Option<&[EXPLICIT_ACCESS_A]>, poldsd: P0, psizenewsd: *mut u32, pnewsd: *mut super::PSECURITY_DESCRIPTOR) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<super::PSECURITY_DESCRIPTOR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn BuildSecurityDescriptorA ( powner : *const TRUSTEE_A , pgroup : *const TRUSTEE_A , ccountofaccessentries : u32 , plistofaccessentries : *const EXPLICIT_ACCESS_A , ccountofauditentries : u32 , plistofauditentries : *const EXPLICIT_ACCESS_A , poldsd : super:: PSECURITY_DESCRIPTOR , psizenewsd : *mut u32 , pnewsd : *mut super:: PSECURITY_DESCRIPTOR ) -> super::super::Foundation:: WIN32_ERROR );
    BuildSecurityDescriptorA(
        ::core::mem::transmute(powner.unwrap_or(::std::ptr::null())),
        ::core::mem::transmute(pgroup.unwrap_or(::std::ptr::null())),
        plistofaccessentries.as_deref().map_or(0, |slice| slice.len() as _),
        ::core::mem::transmute(plistofaccessentries.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())),
        plistofauditentries.as_deref().map_or(0, |slice| slice.len() as _),
        ::core::mem::transmute(plistofauditentries.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())),
        poldsd.into_param().abi(),
        psizenewsd,
        pnewsd,
    )
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BuildSecurityDescriptorW<P0>(powner: ::core::option::Option<*const TRUSTEE_W>, pgroup: ::core::option::Option<*const TRUSTEE_W>, plistofaccessentries: ::core::option::Option<&[EXPLICIT_ACCESS_W]>, plistofauditentries: ::core::option::Option<&[EXPLICIT_ACCESS_W]>, poldsd: P0, psizenewsd: *mut u32, pnewsd: *mut super::PSECURITY_DESCRIPTOR) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<super::PSECURITY_DESCRIPTOR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn BuildSecurityDescriptorW ( powner : *const TRUSTEE_W , pgroup : *const TRUSTEE_W , ccountofaccessentries : u32 , plistofaccessentries : *const EXPLICIT_ACCESS_W , ccountofauditentries : u32 , plistofauditentries : *const EXPLICIT_ACCESS_W , poldsd : super:: PSECURITY_DESCRIPTOR , psizenewsd : *mut u32 , pnewsd : *mut super:: PSECURITY_DESCRIPTOR ) -> super::super::Foundation:: WIN32_ERROR );
    BuildSecurityDescriptorW(
        ::core::mem::transmute(powner.unwrap_or(::std::ptr::null())),
        ::core::mem::transmute(pgroup.unwrap_or(::std::ptr::null())),
        plistofaccessentries.as_deref().map_or(0, |slice| slice.len() as _),
        ::core::mem::transmute(plistofaccessentries.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())),
        plistofauditentries.as_deref().map_or(0, |slice| slice.len() as _),
        ::core::mem::transmute(plistofauditentries.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())),
        poldsd.into_param().abi(),
        psizenewsd,
        pnewsd,
    )
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
#[inline]
pub unsafe fn BuildTrusteeWithNameA<P0>(ptrustee: *mut TRUSTEE_A, pname: P0)
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn BuildTrusteeWithNameA ( ptrustee : *mut TRUSTEE_A , pname : :: windows::core::PCSTR ) -> ( ) );
    BuildTrusteeWithNameA(ptrustee, pname.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
#[inline]
pub unsafe fn BuildTrusteeWithNameW<P0>(ptrustee: *mut TRUSTEE_W, pname: P0)
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn BuildTrusteeWithNameW ( ptrustee : *mut TRUSTEE_W , pname : :: windows::core::PCWSTR ) -> ( ) );
    BuildTrusteeWithNameW(ptrustee, pname.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
#[inline]
pub unsafe fn BuildTrusteeWithObjectsAndNameA<P0, P1, P2>(ptrustee: *mut TRUSTEE_A, pobjname: ::core::option::Option<*const OBJECTS_AND_NAME_A>, objecttype: SE_OBJECT_TYPE, objecttypename: P0, inheritedobjecttypename: P1, name: P2)
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn BuildTrusteeWithObjectsAndNameA ( ptrustee : *mut TRUSTEE_A , pobjname : *const OBJECTS_AND_NAME_A , objecttype : SE_OBJECT_TYPE , objecttypename : :: windows::core::PCSTR , inheritedobjecttypename : :: windows::core::PCSTR , name : :: windows::core::PCSTR ) -> ( ) );
    BuildTrusteeWithObjectsAndNameA(ptrustee, ::core::mem::transmute(pobjname.unwrap_or(::std::ptr::null())), objecttype, objecttypename.into_param().abi(), inheritedobjecttypename.into_param().abi(), name.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
#[inline]
pub unsafe fn BuildTrusteeWithObjectsAndNameW<P0, P1, P2>(ptrustee: *mut TRUSTEE_W, pobjname: ::core::option::Option<*const OBJECTS_AND_NAME_W>, objecttype: SE_OBJECT_TYPE, objecttypename: P0, inheritedobjecttypename: P1, name: P2)
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn BuildTrusteeWithObjectsAndNameW ( ptrustee : *mut TRUSTEE_W , pobjname : *const OBJECTS_AND_NAME_W , objecttype : SE_OBJECT_TYPE , objecttypename : :: windows::core::PCWSTR , inheritedobjecttypename : :: windows::core::PCWSTR , name : :: windows::core::PCWSTR ) -> ( ) );
    BuildTrusteeWithObjectsAndNameW(ptrustee, ::core::mem::transmute(pobjname.unwrap_or(::std::ptr::null())), objecttype, objecttypename.into_param().abi(), inheritedobjecttypename.into_param().abi(), name.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BuildTrusteeWithObjectsAndSidA<P0>(ptrustee: *mut TRUSTEE_A, pobjsid: ::core::option::Option<*const OBJECTS_AND_SID>, pobjectguid: ::core::option::Option<*const ::windows::core::GUID>, pinheritedobjectguid: ::core::option::Option<*const ::windows::core::GUID>, psid: P0)
where
    P0: ::windows::core::IntoParam<super::super::Foundation::PSID>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn BuildTrusteeWithObjectsAndSidA ( ptrustee : *mut TRUSTEE_A , pobjsid : *const OBJECTS_AND_SID , pobjectguid : *const :: windows::core::GUID , pinheritedobjectguid : *const :: windows::core::GUID , psid : super::super::Foundation:: PSID ) -> ( ) );
    BuildTrusteeWithObjectsAndSidA(ptrustee, ::core::mem::transmute(pobjsid.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pobjectguid.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pinheritedobjectguid.unwrap_or(::std::ptr::null())), psid.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BuildTrusteeWithObjectsAndSidW<P0>(ptrustee: *mut TRUSTEE_W, pobjsid: ::core::option::Option<*const OBJECTS_AND_SID>, pobjectguid: ::core::option::Option<*const ::windows::core::GUID>, pinheritedobjectguid: ::core::option::Option<*const ::windows::core::GUID>, psid: P0)
where
    P0: ::windows::core::IntoParam<super::super::Foundation::PSID>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn BuildTrusteeWithObjectsAndSidW ( ptrustee : *mut TRUSTEE_W , pobjsid : *const OBJECTS_AND_SID , pobjectguid : *const :: windows::core::GUID , pinheritedobjectguid : *const :: windows::core::GUID , psid : super::super::Foundation:: PSID ) -> ( ) );
    BuildTrusteeWithObjectsAndSidW(ptrustee, ::core::mem::transmute(pobjsid.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pobjectguid.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pinheritedobjectguid.unwrap_or(::std::ptr::null())), psid.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BuildTrusteeWithSidA<P0>(ptrustee: *mut TRUSTEE_A, psid: P0)
where
    P0: ::windows::core::IntoParam<super::super::Foundation::PSID>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn BuildTrusteeWithSidA ( ptrustee : *mut TRUSTEE_A , psid : super::super::Foundation:: PSID ) -> ( ) );
    BuildTrusteeWithSidA(ptrustee, psid.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BuildTrusteeWithSidW<P0>(ptrustee: *mut TRUSTEE_W, psid: P0)
where
    P0: ::windows::core::IntoParam<super::super::Foundation::PSID>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn BuildTrusteeWithSidW ( ptrustee : *mut TRUSTEE_W , psid : super::super::Foundation:: PSID ) -> ( ) );
    BuildTrusteeWithSidW(ptrustee, psid.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ConvertSecurityDescriptorToStringSecurityDescriptorA<P0>(securitydescriptor: P0, requestedstringsdrevision: u32, securityinformation: u32, stringsecuritydescriptor: *mut ::windows::core::PSTR, stringsecuritydescriptorlen: ::core::option::Option<*mut u32>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::PSECURITY_DESCRIPTOR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn ConvertSecurityDescriptorToStringSecurityDescriptorA ( securitydescriptor : super:: PSECURITY_DESCRIPTOR , requestedstringsdrevision : u32 , securityinformation : u32 , stringsecuritydescriptor : *mut :: windows::core::PSTR , stringsecuritydescriptorlen : *mut u32 ) -> super::super::Foundation:: BOOL );
    ConvertSecurityDescriptorToStringSecurityDescriptorA(securitydescriptor.into_param().abi(), requestedstringsdrevision, securityinformation, stringsecuritydescriptor, ::core::mem::transmute(stringsecuritydescriptorlen.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ConvertSecurityDescriptorToStringSecurityDescriptorW<P0>(securitydescriptor: P0, requestedstringsdrevision: u32, securityinformation: u32, stringsecuritydescriptor: *mut ::windows::core::PWSTR, stringsecuritydescriptorlen: ::core::option::Option<*mut u32>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::PSECURITY_DESCRIPTOR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn ConvertSecurityDescriptorToStringSecurityDescriptorW ( securitydescriptor : super:: PSECURITY_DESCRIPTOR , requestedstringsdrevision : u32 , securityinformation : u32 , stringsecuritydescriptor : *mut :: windows::core::PWSTR , stringsecuritydescriptorlen : *mut u32 ) -> super::super::Foundation:: BOOL );
    ConvertSecurityDescriptorToStringSecurityDescriptorW(securitydescriptor.into_param().abi(), requestedstringsdrevision, securityinformation, stringsecuritydescriptor, ::core::mem::transmute(stringsecuritydescriptorlen.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ConvertSidToStringSidA<P0>(sid: P0, stringsid: *mut ::windows::core::PSTR) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::PSID>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn ConvertSidToStringSidA ( sid : super::super::Foundation:: PSID , stringsid : *mut :: windows::core::PSTR ) -> super::super::Foundation:: BOOL );
    ConvertSidToStringSidA(sid.into_param().abi(), stringsid)
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ConvertSidToStringSidW<P0>(sid: P0, stringsid: *mut ::windows::core::PWSTR) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::PSID>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn ConvertSidToStringSidW ( sid : super::super::Foundation:: PSID , stringsid : *mut :: windows::core::PWSTR ) -> super::super::Foundation:: BOOL );
    ConvertSidToStringSidW(sid.into_param().abi(), stringsid)
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ConvertStringSecurityDescriptorToSecurityDescriptorA<P0>(stringsecuritydescriptor: P0, stringsdrevision: u32, securitydescriptor: *mut super::PSECURITY_DESCRIPTOR, securitydescriptorsize: ::core::option::Option<*mut u32>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn ConvertStringSecurityDescriptorToSecurityDescriptorA ( stringsecuritydescriptor : :: windows::core::PCSTR , stringsdrevision : u32 , securitydescriptor : *mut super:: PSECURITY_DESCRIPTOR , securitydescriptorsize : *mut u32 ) -> super::super::Foundation:: BOOL );
    ConvertStringSecurityDescriptorToSecurityDescriptorA(stringsecuritydescriptor.into_param().abi(), stringsdrevision, securitydescriptor, ::core::mem::transmute(securitydescriptorsize.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ConvertStringSecurityDescriptorToSecurityDescriptorW<P0>(stringsecuritydescriptor: P0, stringsdrevision: u32, securitydescriptor: *mut super::PSECURITY_DESCRIPTOR, securitydescriptorsize: ::core::option::Option<*mut u32>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn ConvertStringSecurityDescriptorToSecurityDescriptorW ( stringsecuritydescriptor : :: windows::core::PCWSTR , stringsdrevision : u32 , securitydescriptor : *mut super:: PSECURITY_DESCRIPTOR , securitydescriptorsize : *mut u32 ) -> super::super::Foundation:: BOOL );
    ConvertStringSecurityDescriptorToSecurityDescriptorW(stringsecuritydescriptor.into_param().abi(), stringsdrevision, securitydescriptor, ::core::mem::transmute(securitydescriptorsize.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ConvertStringSidToSidA<P0>(stringsid: P0, sid: *mut super::super::Foundation::PSID) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn ConvertStringSidToSidA ( stringsid : :: windows::core::PCSTR , sid : *mut super::super::Foundation:: PSID ) -> super::super::Foundation:: BOOL );
    ConvertStringSidToSidA(stringsid.into_param().abi(), sid)
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ConvertStringSidToSidW<P0>(stringsid: P0, sid: *mut super::super::Foundation::PSID) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn ConvertStringSidToSidW ( stringsid : :: windows::core::PCWSTR , sid : *mut super::super::Foundation:: PSID ) -> super::super::Foundation:: BOOL );
    ConvertStringSidToSidW(stringsid.into_param().abi(), sid)
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FreeInheritedFromArray(pinheritarray: &[INHERITED_FROMW], pfnarray: ::core::option::Option<*const FN_OBJECT_MGR_FUNCTS>) -> super::super::Foundation::WIN32_ERROR {
    ::windows::imp::link ! ( "advapi32.dll""system" fn FreeInheritedFromArray ( pinheritarray : *const INHERITED_FROMW , acecnt : u16 , pfnarray : *const FN_OBJECT_MGR_FUNCTS ) -> super::super::Foundation:: WIN32_ERROR );
    FreeInheritedFromArray(::core::mem::transmute(pinheritarray.as_ptr()), pinheritarray.len() as _, ::core::mem::transmute(pfnarray.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetAuditedPermissionsFromAclA(pacl: *const super::ACL, ptrustee: *const TRUSTEE_A, psuccessfulauditedrights: *mut u32, pfailedauditrights: *mut u32) -> super::super::Foundation::WIN32_ERROR {
    ::windows::imp::link ! ( "advapi32.dll""system" fn GetAuditedPermissionsFromAclA ( pacl : *const super:: ACL , ptrustee : *const TRUSTEE_A , psuccessfulauditedrights : *mut u32 , pfailedauditrights : *mut u32 ) -> super::super::Foundation:: WIN32_ERROR );
    GetAuditedPermissionsFromAclA(pacl, ptrustee, psuccessfulauditedrights, pfailedauditrights)
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetAuditedPermissionsFromAclW(pacl: *const super::ACL, ptrustee: *const TRUSTEE_W, psuccessfulauditedrights: *mut u32, pfailedauditrights: *mut u32) -> super::super::Foundation::WIN32_ERROR {
    ::windows::imp::link ! ( "advapi32.dll""system" fn GetAuditedPermissionsFromAclW ( pacl : *const super:: ACL , ptrustee : *const TRUSTEE_W , psuccessfulauditedrights : *mut u32 , pfailedauditrights : *mut u32 ) -> super::super::Foundation:: WIN32_ERROR );
    GetAuditedPermissionsFromAclW(pacl, ptrustee, psuccessfulauditedrights, pfailedauditrights)
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetEffectiveRightsFromAclA(pacl: *const super::ACL, ptrustee: *const TRUSTEE_A, paccessrights: *mut u32) -> super::super::Foundation::WIN32_ERROR {
    ::windows::imp::link ! ( "advapi32.dll""system" fn GetEffectiveRightsFromAclA ( pacl : *const super:: ACL , ptrustee : *const TRUSTEE_A , paccessrights : *mut u32 ) -> super::super::Foundation:: WIN32_ERROR );
    GetEffectiveRightsFromAclA(pacl, ptrustee, paccessrights)
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetEffectiveRightsFromAclW(pacl: *const super::ACL, ptrustee: *const TRUSTEE_W, paccessrights: *mut u32) -> super::super::Foundation::WIN32_ERROR {
    ::windows::imp::link ! ( "advapi32.dll""system" fn GetEffectiveRightsFromAclW ( pacl : *const super:: ACL , ptrustee : *const TRUSTEE_W , paccessrights : *mut u32 ) -> super::super::Foundation:: WIN32_ERROR );
    GetEffectiveRightsFromAclW(pacl, ptrustee, paccessrights)
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetExplicitEntriesFromAclA(pacl: *const super::ACL, pccountofexplicitentries: *mut u32, plistofexplicitentries: *mut *mut EXPLICIT_ACCESS_A) -> super::super::Foundation::WIN32_ERROR {
    ::windows::imp::link ! ( "advapi32.dll""system" fn GetExplicitEntriesFromAclA ( pacl : *const super:: ACL , pccountofexplicitentries : *mut u32 , plistofexplicitentries : *mut *mut EXPLICIT_ACCESS_A ) -> super::super::Foundation:: WIN32_ERROR );
    GetExplicitEntriesFromAclA(pacl, pccountofexplicitentries, plistofexplicitentries)
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetExplicitEntriesFromAclW(pacl: *const super::ACL, pccountofexplicitentries: *mut u32, plistofexplicitentries: *mut *mut EXPLICIT_ACCESS_W) -> super::super::Foundation::WIN32_ERROR {
    ::windows::imp::link ! ( "advapi32.dll""system" fn GetExplicitEntriesFromAclW ( pacl : *const super:: ACL , pccountofexplicitentries : *mut u32 , plistofexplicitentries : *mut *mut EXPLICIT_ACCESS_W ) -> super::super::Foundation:: WIN32_ERROR );
    GetExplicitEntriesFromAclW(pacl, pccountofexplicitentries, plistofexplicitentries)
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetInheritanceSourceA<P0, P1>(pobjectname: P0, objecttype: SE_OBJECT_TYPE, securityinfo: u32, container: P1, pobjectclassguids: ::core::option::Option<&[*const ::windows::core::GUID]>, pacl: *const super::ACL, pfnarray: ::core::option::Option<*const FN_OBJECT_MGR_FUNCTS>, pgenericmapping: *const super::GENERIC_MAPPING, pinheritarray: *mut INHERITED_FROMA) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn GetInheritanceSourceA ( pobjectname : :: windows::core::PCSTR , objecttype : SE_OBJECT_TYPE , securityinfo : u32 , container : super::super::Foundation:: BOOL , pobjectclassguids : *const *const :: windows::core::GUID , guidcount : u32 , pacl : *const super:: ACL , pfnarray : *const FN_OBJECT_MGR_FUNCTS , pgenericmapping : *const super:: GENERIC_MAPPING , pinheritarray : *mut INHERITED_FROMA ) -> super::super::Foundation:: WIN32_ERROR );
    GetInheritanceSourceA(pobjectname.into_param().abi(), objecttype, securityinfo, container.into_param().abi(), ::core::mem::transmute(pobjectclassguids.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pobjectclassguids.as_deref().map_or(0, |slice| slice.len() as _), pacl, ::core::mem::transmute(pfnarray.unwrap_or(::std::ptr::null())), pgenericmapping, pinheritarray)
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetInheritanceSourceW<P0, P1>(pobjectname: P0, objecttype: SE_OBJECT_TYPE, securityinfo: u32, container: P1, pobjectclassguids: ::core::option::Option<&[*const ::windows::core::GUID]>, pacl: *const super::ACL, pfnarray: ::core::option::Option<*const FN_OBJECT_MGR_FUNCTS>, pgenericmapping: *const super::GENERIC_MAPPING, pinheritarray: *mut INHERITED_FROMW) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn GetInheritanceSourceW ( pobjectname : :: windows::core::PCWSTR , objecttype : SE_OBJECT_TYPE , securityinfo : u32 , container : super::super::Foundation:: BOOL , pobjectclassguids : *const *const :: windows::core::GUID , guidcount : u32 , pacl : *const super:: ACL , pfnarray : *const FN_OBJECT_MGR_FUNCTS , pgenericmapping : *const super:: GENERIC_MAPPING , pinheritarray : *mut INHERITED_FROMW ) -> super::super::Foundation:: WIN32_ERROR );
    GetInheritanceSourceW(pobjectname.into_param().abi(), objecttype, securityinfo, container.into_param().abi(), ::core::mem::transmute(pobjectclassguids.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pobjectclassguids.as_deref().map_or(0, |slice| slice.len() as _), pacl, ::core::mem::transmute(pfnarray.unwrap_or(::std::ptr::null())), pgenericmapping, pinheritarray)
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
#[inline]
pub unsafe fn GetMultipleTrusteeA(ptrustee: ::core::option::Option<*const TRUSTEE_A>) -> *mut TRUSTEE_A {
    ::windows::imp::link ! ( "advapi32.dll""system" fn GetMultipleTrusteeA ( ptrustee : *const TRUSTEE_A ) -> *mut TRUSTEE_A );
    GetMultipleTrusteeA(::core::mem::transmute(ptrustee.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
#[inline]
pub unsafe fn GetMultipleTrusteeOperationA(ptrustee: ::core::option::Option<*const TRUSTEE_A>) -> MULTIPLE_TRUSTEE_OPERATION {
    ::windows::imp::link ! ( "advapi32.dll""system" fn GetMultipleTrusteeOperationA ( ptrustee : *const TRUSTEE_A ) -> MULTIPLE_TRUSTEE_OPERATION );
    GetMultipleTrusteeOperationA(::core::mem::transmute(ptrustee.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
#[inline]
pub unsafe fn GetMultipleTrusteeOperationW(ptrustee: ::core::option::Option<*const TRUSTEE_W>) -> MULTIPLE_TRUSTEE_OPERATION {
    ::windows::imp::link ! ( "advapi32.dll""system" fn GetMultipleTrusteeOperationW ( ptrustee : *const TRUSTEE_W ) -> MULTIPLE_TRUSTEE_OPERATION );
    GetMultipleTrusteeOperationW(::core::mem::transmute(ptrustee.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
#[inline]
pub unsafe fn GetMultipleTrusteeW(ptrustee: ::core::option::Option<*const TRUSTEE_W>) -> *mut TRUSTEE_W {
    ::windows::imp::link ! ( "advapi32.dll""system" fn GetMultipleTrusteeW ( ptrustee : *const TRUSTEE_W ) -> *mut TRUSTEE_W );
    GetMultipleTrusteeW(::core::mem::transmute(ptrustee.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetNamedSecurityInfoA<P0>(pobjectname: P0, objecttype: SE_OBJECT_TYPE, securityinfo: super::OBJECT_SECURITY_INFORMATION, ppsidowner: ::core::option::Option<*mut super::super::Foundation::PSID>, ppsidgroup: ::core::option::Option<*mut super::super::Foundation::PSID>, ppdacl: ::core::option::Option<*mut *mut super::ACL>, ppsacl: ::core::option::Option<*mut *mut super::ACL>, ppsecuritydescriptor: *mut super::PSECURITY_DESCRIPTOR) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn GetNamedSecurityInfoA ( pobjectname : :: windows::core::PCSTR , objecttype : SE_OBJECT_TYPE , securityinfo : super:: OBJECT_SECURITY_INFORMATION , ppsidowner : *mut super::super::Foundation:: PSID , ppsidgroup : *mut super::super::Foundation:: PSID , ppdacl : *mut *mut super:: ACL , ppsacl : *mut *mut super:: ACL , ppsecuritydescriptor : *mut super:: PSECURITY_DESCRIPTOR ) -> super::super::Foundation:: WIN32_ERROR );
    GetNamedSecurityInfoA(pobjectname.into_param().abi(), objecttype, securityinfo, ::core::mem::transmute(ppsidowner.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppsidgroup.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppdacl.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppsacl.unwrap_or(::std::ptr::null_mut())), ppsecuritydescriptor)
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetNamedSecurityInfoW<P0>(pobjectname: P0, objecttype: SE_OBJECT_TYPE, securityinfo: super::OBJECT_SECURITY_INFORMATION, ppsidowner: ::core::option::Option<*mut super::super::Foundation::PSID>, ppsidgroup: ::core::option::Option<*mut super::super::Foundation::PSID>, ppdacl: ::core::option::Option<*mut *mut super::ACL>, ppsacl: ::core::option::Option<*mut *mut super::ACL>, ppsecuritydescriptor: *mut super::PSECURITY_DESCRIPTOR) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn GetNamedSecurityInfoW ( pobjectname : :: windows::core::PCWSTR , objecttype : SE_OBJECT_TYPE , securityinfo : super:: OBJECT_SECURITY_INFORMATION , ppsidowner : *mut super::super::Foundation:: PSID , ppsidgroup : *mut super::super::Foundation:: PSID , ppdacl : *mut *mut super:: ACL , ppsacl : *mut *mut super:: ACL , ppsecuritydescriptor : *mut super:: PSECURITY_DESCRIPTOR ) -> super::super::Foundation:: WIN32_ERROR );
    GetNamedSecurityInfoW(pobjectname.into_param().abi(), objecttype, securityinfo, ::core::mem::transmute(ppsidowner.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppsidgroup.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppdacl.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppsacl.unwrap_or(::std::ptr::null_mut())), ppsecuritydescriptor)
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSecurityInfo<P0>(handle: P0, objecttype: SE_OBJECT_TYPE, securityinfo: u32, ppsidowner: ::core::option::Option<*mut super::super::Foundation::PSID>, ppsidgroup: ::core::option::Option<*mut super::super::Foundation::PSID>, ppdacl: ::core::option::Option<*mut *mut super::ACL>, ppsacl: ::core::option::Option<*mut *mut super::ACL>, ppsecuritydescriptor: ::core::option::Option<*mut super::PSECURITY_DESCRIPTOR>) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn GetSecurityInfo ( handle : super::super::Foundation:: HANDLE , objecttype : SE_OBJECT_TYPE , securityinfo : u32 , ppsidowner : *mut super::super::Foundation:: PSID , ppsidgroup : *mut super::super::Foundation:: PSID , ppdacl : *mut *mut super:: ACL , ppsacl : *mut *mut super:: ACL , ppsecuritydescriptor : *mut super:: PSECURITY_DESCRIPTOR ) -> super::super::Foundation:: WIN32_ERROR );
    GetSecurityInfo(handle.into_param().abi(), objecttype, securityinfo, ::core::mem::transmute(ppsidowner.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppsidgroup.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppdacl.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppsacl.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppsecuritydescriptor.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
#[inline]
pub unsafe fn GetTrusteeFormA(ptrustee: *const TRUSTEE_A) -> TRUSTEE_FORM {
    ::windows::imp::link ! ( "advapi32.dll""system" fn GetTrusteeFormA ( ptrustee : *const TRUSTEE_A ) -> TRUSTEE_FORM );
    GetTrusteeFormA(ptrustee)
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
#[inline]
pub unsafe fn GetTrusteeFormW(ptrustee: *const TRUSTEE_W) -> TRUSTEE_FORM {
    ::windows::imp::link ! ( "advapi32.dll""system" fn GetTrusteeFormW ( ptrustee : *const TRUSTEE_W ) -> TRUSTEE_FORM );
    GetTrusteeFormW(ptrustee)
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
#[inline]
pub unsafe fn GetTrusteeNameA(ptrustee: *const TRUSTEE_A) -> ::windows::core::PSTR {
    ::windows::imp::link ! ( "advapi32.dll""system" fn GetTrusteeNameA ( ptrustee : *const TRUSTEE_A ) -> :: windows::core::PSTR );
    GetTrusteeNameA(ptrustee)
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
#[inline]
pub unsafe fn GetTrusteeNameW(ptrustee: *const TRUSTEE_W) -> ::windows::core::PWSTR {
    ::windows::imp::link ! ( "advapi32.dll""system" fn GetTrusteeNameW ( ptrustee : *const TRUSTEE_W ) -> :: windows::core::PWSTR );
    GetTrusteeNameW(ptrustee)
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
#[inline]
pub unsafe fn GetTrusteeTypeA(ptrustee: ::core::option::Option<*const TRUSTEE_A>) -> TRUSTEE_TYPE {
    ::windows::imp::link ! ( "advapi32.dll""system" fn GetTrusteeTypeA ( ptrustee : *const TRUSTEE_A ) -> TRUSTEE_TYPE );
    GetTrusteeTypeA(::core::mem::transmute(ptrustee.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
#[inline]
pub unsafe fn GetTrusteeTypeW(ptrustee: ::core::option::Option<*const TRUSTEE_W>) -> TRUSTEE_TYPE {
    ::windows::imp::link ! ( "advapi32.dll""system" fn GetTrusteeTypeW ( ptrustee : *const TRUSTEE_W ) -> TRUSTEE_TYPE );
    GetTrusteeTypeW(::core::mem::transmute(ptrustee.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LookupSecurityDescriptorPartsA<P0>(ppowner: ::core::option::Option<*mut *mut TRUSTEE_A>, ppgroup: ::core::option::Option<*mut *mut TRUSTEE_A>, pccountofaccessentries: ::core::option::Option<*mut u32>, pplistofaccessentries: *mut *mut EXPLICIT_ACCESS_A, pccountofauditentries: ::core::option::Option<*mut u32>, pplistofauditentries: *mut *mut EXPLICIT_ACCESS_A, psd: P0) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<super::PSECURITY_DESCRIPTOR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn LookupSecurityDescriptorPartsA ( ppowner : *mut *mut TRUSTEE_A , ppgroup : *mut *mut TRUSTEE_A , pccountofaccessentries : *mut u32 , pplistofaccessentries : *mut *mut EXPLICIT_ACCESS_A , pccountofauditentries : *mut u32 , pplistofauditentries : *mut *mut EXPLICIT_ACCESS_A , psd : super:: PSECURITY_DESCRIPTOR ) -> super::super::Foundation:: WIN32_ERROR );
    LookupSecurityDescriptorPartsA(::core::mem::transmute(ppowner.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppgroup.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pccountofaccessentries.unwrap_or(::std::ptr::null_mut())), pplistofaccessentries, ::core::mem::transmute(pccountofauditentries.unwrap_or(::std::ptr::null_mut())), pplistofauditentries, psd.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LookupSecurityDescriptorPartsW<P0>(ppowner: ::core::option::Option<*mut *mut TRUSTEE_W>, ppgroup: ::core::option::Option<*mut *mut TRUSTEE_W>, pccountofaccessentries: ::core::option::Option<*mut u32>, pplistofaccessentries: *mut *mut EXPLICIT_ACCESS_W, pccountofauditentries: ::core::option::Option<*mut u32>, pplistofauditentries: *mut *mut EXPLICIT_ACCESS_W, psd: P0) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<super::PSECURITY_DESCRIPTOR>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn LookupSecurityDescriptorPartsW ( ppowner : *mut *mut TRUSTEE_W , ppgroup : *mut *mut TRUSTEE_W , pccountofaccessentries : *mut u32 , pplistofaccessentries : *mut *mut EXPLICIT_ACCESS_W , pccountofauditentries : *mut u32 , pplistofauditentries : *mut *mut EXPLICIT_ACCESS_W , psd : super:: PSECURITY_DESCRIPTOR ) -> super::super::Foundation:: WIN32_ERROR );
    LookupSecurityDescriptorPartsW(::core::mem::transmute(ppowner.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppgroup.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pccountofaccessentries.unwrap_or(::std::ptr::null_mut())), pplistofaccessentries, ::core::mem::transmute(pccountofauditentries.unwrap_or(::std::ptr::null_mut())), pplistofauditentries, psd.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetEntriesInAclA(plistofexplicitentries: ::core::option::Option<&[EXPLICIT_ACCESS_A]>, oldacl: ::core::option::Option<*const super::ACL>, newacl: *mut *mut super::ACL) -> super::super::Foundation::WIN32_ERROR {
    ::windows::imp::link ! ( "advapi32.dll""system" fn SetEntriesInAclA ( ccountofexplicitentries : u32 , plistofexplicitentries : *const EXPLICIT_ACCESS_A , oldacl : *const super:: ACL , newacl : *mut *mut super:: ACL ) -> super::super::Foundation:: WIN32_ERROR );
    SetEntriesInAclA(plistofexplicitentries.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(plistofexplicitentries.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), ::core::mem::transmute(oldacl.unwrap_or(::std::ptr::null())), newacl)
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetEntriesInAclW(plistofexplicitentries: ::core::option::Option<&[EXPLICIT_ACCESS_W]>, oldacl: ::core::option::Option<*const super::ACL>, newacl: *mut *mut super::ACL) -> super::super::Foundation::WIN32_ERROR {
    ::windows::imp::link ! ( "advapi32.dll""system" fn SetEntriesInAclW ( ccountofexplicitentries : u32 , plistofexplicitentries : *const EXPLICIT_ACCESS_W , oldacl : *const super:: ACL , newacl : *mut *mut super:: ACL ) -> super::super::Foundation:: WIN32_ERROR );
    SetEntriesInAclW(plistofexplicitentries.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(plistofexplicitentries.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), ::core::mem::transmute(oldacl.unwrap_or(::std::ptr::null())), newacl)
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetNamedSecurityInfoA<P0, P1, P2>(pobjectname: P0, objecttype: SE_OBJECT_TYPE, securityinfo: super::OBJECT_SECURITY_INFORMATION, psidowner: P1, psidgroup: P2, pdacl: ::core::option::Option<*const super::ACL>, psacl: ::core::option::Option<*const super::ACL>) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::PSID>,
    P2: ::windows::core::IntoParam<super::super::Foundation::PSID>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn SetNamedSecurityInfoA ( pobjectname : :: windows::core::PCSTR , objecttype : SE_OBJECT_TYPE , securityinfo : super:: OBJECT_SECURITY_INFORMATION , psidowner : super::super::Foundation:: PSID , psidgroup : super::super::Foundation:: PSID , pdacl : *const super:: ACL , psacl : *const super:: ACL ) -> super::super::Foundation:: WIN32_ERROR );
    SetNamedSecurityInfoA(pobjectname.into_param().abi(), objecttype, securityinfo, psidowner.into_param().abi(), psidgroup.into_param().abi(), ::core::mem::transmute(pdacl.unwrap_or(::std::ptr::null())), ::core::mem::transmute(psacl.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetNamedSecurityInfoW<P0, P1, P2>(pobjectname: P0, objecttype: SE_OBJECT_TYPE, securityinfo: super::OBJECT_SECURITY_INFORMATION, psidowner: P1, psidgroup: P2, pdacl: ::core::option::Option<*const super::ACL>, psacl: ::core::option::Option<*const super::ACL>) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::PSID>,
    P2: ::windows::core::IntoParam<super::super::Foundation::PSID>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn SetNamedSecurityInfoW ( pobjectname : :: windows::core::PCWSTR , objecttype : SE_OBJECT_TYPE , securityinfo : super:: OBJECT_SECURITY_INFORMATION , psidowner : super::super::Foundation:: PSID , psidgroup : super::super::Foundation:: PSID , pdacl : *const super:: ACL , psacl : *const super:: ACL ) -> super::super::Foundation:: WIN32_ERROR );
    SetNamedSecurityInfoW(pobjectname.into_param().abi(), objecttype, securityinfo, psidowner.into_param().abi(), psidgroup.into_param().abi(), ::core::mem::transmute(pdacl.unwrap_or(::std::ptr::null())), ::core::mem::transmute(psacl.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetSecurityInfo<P0, P1, P2>(handle: P0, objecttype: SE_OBJECT_TYPE, securityinfo: u32, psidowner: P1, psidgroup: P2, pdacl: ::core::option::Option<*const super::ACL>, psacl: ::core::option::Option<*const super::ACL>) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<super::super::Foundation::PSID>,
    P2: ::windows::core::IntoParam<super::super::Foundation::PSID>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn SetSecurityInfo ( handle : super::super::Foundation:: HANDLE , objecttype : SE_OBJECT_TYPE , securityinfo : u32 , psidowner : super::super::Foundation:: PSID , psidgroup : super::super::Foundation:: PSID , pdacl : *const super:: ACL , psacl : *const super:: ACL ) -> super::super::Foundation:: WIN32_ERROR );
    SetSecurityInfo(handle.into_param().abi(), objecttype, securityinfo, psidowner.into_param().abi(), psidgroup.into_param().abi(), ::core::mem::transmute(pdacl.unwrap_or(::std::ptr::null())), ::core::mem::transmute(psacl.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TreeResetNamedSecurityInfoA<P0, P1, P2, P3>(pobjectname: P0, objecttype: SE_OBJECT_TYPE, securityinfo: u32, powner: P1, pgroup: P2, pdacl: ::core::option::Option<*const super::ACL>, psacl: ::core::option::Option<*const super::ACL>, keepexplicit: P3, fnprogress: FN_PROGRESS, progressinvokesetting: PROG_INVOKE_SETTING, args: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::PSID>,
    P2: ::windows::core::IntoParam<super::super::Foundation::PSID>,
    P3: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn TreeResetNamedSecurityInfoA ( pobjectname : :: windows::core::PCSTR , objecttype : SE_OBJECT_TYPE , securityinfo : u32 , powner : super::super::Foundation:: PSID , pgroup : super::super::Foundation:: PSID , pdacl : *const super:: ACL , psacl : *const super:: ACL , keepexplicit : super::super::Foundation:: BOOL , fnprogress : FN_PROGRESS , progressinvokesetting : PROG_INVOKE_SETTING , args : *const ::core::ffi::c_void ) -> super::super::Foundation:: WIN32_ERROR );
    TreeResetNamedSecurityInfoA(pobjectname.into_param().abi(), objecttype, securityinfo, powner.into_param().abi(), pgroup.into_param().abi(), ::core::mem::transmute(pdacl.unwrap_or(::std::ptr::null())), ::core::mem::transmute(psacl.unwrap_or(::std::ptr::null())), keepexplicit.into_param().abi(), fnprogress, progressinvokesetting, ::core::mem::transmute(args.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TreeResetNamedSecurityInfoW<P0, P1, P2, P3>(pobjectname: P0, objecttype: SE_OBJECT_TYPE, securityinfo: u32, powner: P1, pgroup: P2, pdacl: ::core::option::Option<*const super::ACL>, psacl: ::core::option::Option<*const super::ACL>, keepexplicit: P3, fnprogress: FN_PROGRESS, progressinvokesetting: PROG_INVOKE_SETTING, args: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::PSID>,
    P2: ::windows::core::IntoParam<super::super::Foundation::PSID>,
    P3: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn TreeResetNamedSecurityInfoW ( pobjectname : :: windows::core::PCWSTR , objecttype : SE_OBJECT_TYPE , securityinfo : u32 , powner : super::super::Foundation:: PSID , pgroup : super::super::Foundation:: PSID , pdacl : *const super:: ACL , psacl : *const super:: ACL , keepexplicit : super::super::Foundation:: BOOL , fnprogress : FN_PROGRESS , progressinvokesetting : PROG_INVOKE_SETTING , args : *const ::core::ffi::c_void ) -> super::super::Foundation:: WIN32_ERROR );
    TreeResetNamedSecurityInfoW(pobjectname.into_param().abi(), objecttype, securityinfo, powner.into_param().abi(), pgroup.into_param().abi(), ::core::mem::transmute(pdacl.unwrap_or(::std::ptr::null())), ::core::mem::transmute(psacl.unwrap_or(::std::ptr::null())), keepexplicit.into_param().abi(), fnprogress, progressinvokesetting, ::core::mem::transmute(args.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TreeSetNamedSecurityInfoA<P0, P1, P2>(pobjectname: P0, objecttype: SE_OBJECT_TYPE, securityinfo: u32, powner: P1, pgroup: P2, pdacl: ::core::option::Option<*const super::ACL>, psacl: ::core::option::Option<*const super::ACL>, dwaction: TREE_SEC_INFO, fnprogress: FN_PROGRESS, progressinvokesetting: PROG_INVOKE_SETTING, args: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::PSID>,
    P2: ::windows::core::IntoParam<super::super::Foundation::PSID>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn TreeSetNamedSecurityInfoA ( pobjectname : :: windows::core::PCSTR , objecttype : SE_OBJECT_TYPE , securityinfo : u32 , powner : super::super::Foundation:: PSID , pgroup : super::super::Foundation:: PSID , pdacl : *const super:: ACL , psacl : *const super:: ACL , dwaction : TREE_SEC_INFO , fnprogress : FN_PROGRESS , progressinvokesetting : PROG_INVOKE_SETTING , args : *const ::core::ffi::c_void ) -> super::super::Foundation:: WIN32_ERROR );
    TreeSetNamedSecurityInfoA(pobjectname.into_param().abi(), objecttype, securityinfo, powner.into_param().abi(), pgroup.into_param().abi(), ::core::mem::transmute(pdacl.unwrap_or(::std::ptr::null())), ::core::mem::transmute(psacl.unwrap_or(::std::ptr::null())), dwaction, fnprogress, progressinvokesetting, ::core::mem::transmute(args.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TreeSetNamedSecurityInfoW<P0, P1, P2>(pobjectname: P0, objecttype: SE_OBJECT_TYPE, securityinfo: u32, powner: P1, pgroup: P2, pdacl: ::core::option::Option<*const super::ACL>, psacl: ::core::option::Option<*const super::ACL>, dwaction: TREE_SEC_INFO, fnprogress: FN_PROGRESS, progressinvokesetting: PROG_INVOKE_SETTING, args: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::PSID>,
    P2: ::windows::core::IntoParam<super::super::Foundation::PSID>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn TreeSetNamedSecurityInfoW ( pobjectname : :: windows::core::PCWSTR , objecttype : SE_OBJECT_TYPE , securityinfo : u32 , powner : super::super::Foundation:: PSID , pgroup : super::super::Foundation:: PSID , pdacl : *const super:: ACL , psacl : *const super:: ACL , dwaction : TREE_SEC_INFO , fnprogress : FN_PROGRESS , progressinvokesetting : PROG_INVOKE_SETTING , args : *const ::core::ffi::c_void ) -> super::super::Foundation:: WIN32_ERROR );
    TreeSetNamedSecurityInfoW(pobjectname.into_param().abi(), objecttype, securityinfo, powner.into_param().abi(), pgroup.into_param().abi(), ::core::mem::transmute(pdacl.unwrap_or(::std::ptr::null())), ::core::mem::transmute(psacl.unwrap_or(::std::ptr::null())), dwaction, fnprogress, progressinvokesetting, ::core::mem::transmute(args.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAzApplication(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAzApplication {
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).Name)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetName<P0>(&self, bstrname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetName)(::windows::core::Interface::as_raw(self), bstrname.into_param().abi()).ok()
    }
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).Description)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDescription<P0>(&self, bstrdescription: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetDescription)(::windows::core::Interface::as_raw(self), bstrdescription.into_param().abi()).ok()
    }
    pub unsafe fn ApplicationData(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).ApplicationData)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetApplicationData<P0>(&self, bstrapplicationdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetApplicationData)(::windows::core::Interface::as_raw(self), bstrapplicationdata.into_param().abi()).ok()
    }
    pub unsafe fn AuthzInterfaceClsid(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).AuthzInterfaceClsid)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetAuthzInterfaceClsid<P0>(&self, bstrprop: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetAuthzInterfaceClsid)(::windows::core::Interface::as_raw(self), bstrprop.into_param().abi()).ok()
    }
    pub unsafe fn Version(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).Version)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetVersion<P0>(&self, bstrprop: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetVersion)(::windows::core::Interface::as_raw(self), bstrprop.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GenerateAudits(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).GenerateAudits)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetGenerateAudits<P0>(&self, bprop: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetGenerateAudits)(::windows::core::Interface::as_raw(self), bprop.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ApplyStoreSacl(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).ApplyStoreSacl)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetApplyStoreSacl<P0>(&self, bprop: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetApplyStoreSacl)(::windows::core::Interface::as_raw(self), bprop.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Writable(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).Writable)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetProperty(&self, lpropid: i32, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).GetProperty)(::windows::core::Interface::as_raw(self), lpropid, ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetProperty(&self, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetProperty)(::windows::core::Interface::as_raw(self), lpropid, ::core::mem::transmute(varprop), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PolicyAdministrators(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).PolicyAdministrators)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PolicyReaders(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).PolicyReaders)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPolicyAdministrator<P0>(&self, bstradmin: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).AddPolicyAdministrator)(::windows::core::Interface::as_raw(self), bstradmin.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePolicyAdministrator<P0>(&self, bstradmin: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).DeletePolicyAdministrator)(::windows::core::Interface::as_raw(self), bstradmin.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPolicyReader<P0>(&self, bstrreader: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).AddPolicyReader)(::windows::core::Interface::as_raw(self), bstrreader.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePolicyReader<P0>(&self, bstrreader: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).DeletePolicyReader)(::windows::core::Interface::as_raw(self), bstrreader.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Scopes(&self) -> ::windows::core::Result<IAzScopes> {
        let mut result__ = ::windows::core::zeroed::<IAzScopes>();
        (::windows::core::Interface::vtable(self).Scopes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn OpenScope<P0>(&self, bstrscopename: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzScope>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzScope>();
        (::windows::core::Interface::vtable(self).OpenScope)(::windows::core::Interface::as_raw(self), bstrscopename.into_param().abi(), ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateScope<P0>(&self, bstrscopename: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzScope>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzScope>();
        (::windows::core::Interface::vtable(self).CreateScope)(::windows::core::Interface::as_raw(self), bstrscopename.into_param().abi(), ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteScope<P0>(&self, bstrscopename: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).DeleteScope)(::windows::core::Interface::as_raw(self), bstrscopename.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Operations(&self) -> ::windows::core::Result<IAzOperations> {
        let mut result__ = ::windows::core::zeroed::<IAzOperations>();
        (::windows::core::Interface::vtable(self).Operations)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn OpenOperation<P0>(&self, bstroperationname: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzOperation>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzOperation>();
        (::windows::core::Interface::vtable(self).OpenOperation)(::windows::core::Interface::as_raw(self), bstroperationname.into_param().abi(), ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateOperation<P0>(&self, bstroperationname: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzOperation>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzOperation>();
        (::windows::core::Interface::vtable(self).CreateOperation)(::windows::core::Interface::as_raw(self), bstroperationname.into_param().abi(), ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteOperation<P0>(&self, bstroperationname: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).DeleteOperation)(::windows::core::Interface::as_raw(self), bstroperationname.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Tasks(&self) -> ::windows::core::Result<IAzTasks> {
        let mut result__ = ::windows::core::zeroed::<IAzTasks>();
        (::windows::core::Interface::vtable(self).Tasks)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn OpenTask<P0>(&self, bstrtaskname: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzTask>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzTask>();
        (::windows::core::Interface::vtable(self).OpenTask)(::windows::core::Interface::as_raw(self), bstrtaskname.into_param().abi(), ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateTask<P0>(&self, bstrtaskname: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzTask>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzTask>();
        (::windows::core::Interface::vtable(self).CreateTask)(::windows::core::Interface::as_raw(self), bstrtaskname.into_param().abi(), ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteTask<P0>(&self, bstrtaskname: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).DeleteTask)(::windows::core::Interface::as_raw(self), bstrtaskname.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ApplicationGroups(&self) -> ::windows::core::Result<IAzApplicationGroups> {
        let mut result__ = ::windows::core::zeroed::<IAzApplicationGroups>();
        (::windows::core::Interface::vtable(self).ApplicationGroups)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn OpenApplicationGroup<P0>(&self, bstrgroupname: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzApplicationGroup>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzApplicationGroup>();
        (::windows::core::Interface::vtable(self).OpenApplicationGroup)(::windows::core::Interface::as_raw(self), bstrgroupname.into_param().abi(), ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateApplicationGroup<P0>(&self, bstrgroupname: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzApplicationGroup>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzApplicationGroup>();
        (::windows::core::Interface::vtable(self).CreateApplicationGroup)(::windows::core::Interface::as_raw(self), bstrgroupname.into_param().abi(), ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteApplicationGroup<P0>(&self, bstrgroupname: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).DeleteApplicationGroup)(::windows::core::Interface::as_raw(self), bstrgroupname.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Roles(&self) -> ::windows::core::Result<IAzRoles> {
        let mut result__ = ::windows::core::zeroed::<IAzRoles>();
        (::windows::core::Interface::vtable(self).Roles)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn OpenRole<P0>(&self, bstrrolename: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzRole>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzRole>();
        (::windows::core::Interface::vtable(self).OpenRole)(::windows::core::Interface::as_raw(self), bstrrolename.into_param().abi(), ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateRole<P0>(&self, bstrrolename: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzRole>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzRole>();
        (::windows::core::Interface::vtable(self).CreateRole)(::windows::core::Interface::as_raw(self), bstrrolename.into_param().abi(), ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteRole<P0>(&self, bstrrolename: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).DeleteRole)(::windows::core::Interface::as_raw(self), bstrrolename.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn InitializeClientContextFromToken(&self, ulltokenhandle: u64, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzClientContext> {
        let mut result__ = ::windows::core::zeroed::<IAzClientContext>();
        (::windows::core::Interface::vtable(self).InitializeClientContextFromToken)(::windows::core::Interface::as_raw(self), ulltokenhandle, ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPropertyItem(&self, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddPropertyItem)(::windows::core::Interface::as_raw(self), lpropid, ::core::mem::transmute(varprop), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePropertyItem(&self, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeletePropertyItem)(::windows::core::Interface::as_raw(self), lpropid, ::core::mem::transmute(varprop), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Submit(&self, lflags: i32, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Submit)(::windows::core::Interface::as_raw(self), lflags, ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn InitializeClientContextFromName<P0, P1>(&self, clientname: P0, domainname: P1, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzClientContext>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzClientContext>();
        (::windows::core::Interface::vtable(self).InitializeClientContextFromName)(::windows::core::Interface::as_raw(self), clientname.into_param().abi(), domainname.into_param().abi(), ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DelegatedPolicyUsers(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).DelegatedPolicyUsers)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddDelegatedPolicyUser<P0>(&self, bstrdelegatedpolicyuser: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).AddDelegatedPolicyUser)(::windows::core::Interface::as_raw(self), bstrdelegatedpolicyuser.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteDelegatedPolicyUser<P0>(&self, bstrdelegatedpolicyuser: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).DeleteDelegatedPolicyUser)(::windows::core::Interface::as_raw(self), bstrdelegatedpolicyuser.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn InitializeClientContextFromStringSid<P0>(&self, sidstring: P0, loptions: i32, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzClientContext>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzClientContext>();
        (::windows::core::Interface::vtable(self).InitializeClientContextFromStringSid)(::windows::core::Interface::as_raw(self), sidstring.into_param().abi(), loptions, ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PolicyAdministratorsName(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).PolicyAdministratorsName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PolicyReadersName(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).PolicyReadersName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPolicyAdministratorName<P0>(&self, bstradmin: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).AddPolicyAdministratorName)(::windows::core::Interface::as_raw(self), bstradmin.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePolicyAdministratorName<P0>(&self, bstradmin: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).DeletePolicyAdministratorName)(::windows::core::Interface::as_raw(self), bstradmin.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPolicyReaderName<P0>(&self, bstrreader: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).AddPolicyReaderName)(::windows::core::Interface::as_raw(self), bstrreader.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePolicyReaderName<P0>(&self, bstrreader: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).DeletePolicyReaderName)(::windows::core::Interface::as_raw(self), bstrreader.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DelegatedPolicyUsersName(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).DelegatedPolicyUsersName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddDelegatedPolicyUserName<P0>(&self, bstrdelegatedpolicyuser: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).AddDelegatedPolicyUserName)(::windows::core::Interface::as_raw(self), bstrdelegatedpolicyuser.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteDelegatedPolicyUserName<P0>(&self, bstrdelegatedpolicyuser: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).DeleteDelegatedPolicyUserName)(::windows::core::Interface::as_raw(self), bstrdelegatedpolicyuser.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IAzApplication, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAzApplication {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAzApplication {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAzApplication {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAzApplication").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IAzApplication {
    type Vtable = IAzApplication_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAzApplication {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IAzApplication {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x987bc7c7_b813_4d27_bede_6ba5ae867e95);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzApplication_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdescription: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdescription: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub ApplicationData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrapplicationdata: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetApplicationData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrapplicationdata: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub AuthzInterfaceClsid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrprop: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetAuthzInterfaceClsid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Version: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrprop: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GenerateAudits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbprop: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GenerateAudits: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetGenerateAudits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bprop: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetGenerateAudits: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ApplyStoreSacl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbprop: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ApplyStoreSacl: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetApplyStoreSacl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bprop: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetApplyStoreSacl: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Writable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfprop: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Writable: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpropid: i32, varreserved: super::super::System::Com::VARIANT, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PolicyAdministrators: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvaradmins: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PolicyAdministrators: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PolicyReaders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarreaders: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PolicyReaders: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddPolicyAdministrator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstradmin: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddPolicyAdministrator: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeletePolicyAdministrator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstradmin: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeletePolicyAdministrator: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddPolicyReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrreader: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddPolicyReader: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeletePolicyReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrreader: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeletePolicyReader: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Scopes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppscopecollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Scopes: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub OpenScope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrscopename: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT, ppscope: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    OpenScope: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateScope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrscopename: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT, ppscope: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateScope: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteScope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrscopename: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteScope: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Operations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppoperationcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Operations: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub OpenOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstroperationname: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT, ppoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    OpenOperation: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstroperationname: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT, ppoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateOperation: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstroperationname: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteOperation: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Tasks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptaskcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Tasks: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub OpenTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtaskname: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT, pptask: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    OpenTask: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtaskname: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT, pptask: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateTask: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtaskname: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteTask: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ApplicationGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppgroupcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ApplicationGroups: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub OpenApplicationGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrgroupname: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    OpenApplicationGroup: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateApplicationGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrgroupname: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateApplicationGroup: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteApplicationGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrgroupname: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteApplicationGroup: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Roles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprolecollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Roles: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub OpenRole: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrrolename: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT, pprole: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    OpenRole: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateRole: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrrolename: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT, pprole: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateRole: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteRole: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrrolename: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteRole: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub InitializeClientContextFromToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulltokenhandle: u64, varreserved: super::super::System::Com::VARIANT, ppclientcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    InitializeClientContextFromToken: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddPropertyItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddPropertyItem: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeletePropertyItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeletePropertyItem: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Submit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Submit: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub InitializeClientContextFromName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clientname: ::std::mem::MaybeUninit<::windows::core::BSTR>, domainname: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT, ppclientcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    InitializeClientContextFromName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DelegatedPolicyUsers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvardelegatedpolicyusers: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DelegatedPolicyUsers: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddDelegatedPolicyUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdelegatedpolicyuser: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddDelegatedPolicyUser: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteDelegatedPolicyUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdelegatedpolicyuser: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteDelegatedPolicyUser: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub InitializeClientContextFromStringSid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sidstring: ::std::mem::MaybeUninit<::windows::core::BSTR>, loptions: i32, varreserved: super::super::System::Com::VARIANT, ppclientcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    InitializeClientContextFromStringSid: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PolicyAdministratorsName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvaradmins: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PolicyAdministratorsName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PolicyReadersName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarreaders: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PolicyReadersName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddPolicyAdministratorName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstradmin: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddPolicyAdministratorName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeletePolicyAdministratorName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstradmin: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeletePolicyAdministratorName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddPolicyReaderName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrreader: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddPolicyReaderName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeletePolicyReaderName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrreader: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeletePolicyReaderName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DelegatedPolicyUsersName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvardelegatedpolicyusers: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DelegatedPolicyUsersName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddDelegatedPolicyUserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdelegatedpolicyuser: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddDelegatedPolicyUserName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteDelegatedPolicyUserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdelegatedpolicyuser: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteDelegatedPolicyUserName: usize,
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAzApplication2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAzApplication2 {
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.Name)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetName<P0>(&self, bstrname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetName)(::windows::core::Interface::as_raw(self), bstrname.into_param().abi()).ok()
    }
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.Description)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDescription<P0>(&self, bstrdescription: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetDescription)(::windows::core::Interface::as_raw(self), bstrdescription.into_param().abi()).ok()
    }
    pub unsafe fn ApplicationData(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.ApplicationData)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetApplicationData<P0>(&self, bstrapplicationdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetApplicationData)(::windows::core::Interface::as_raw(self), bstrapplicationdata.into_param().abi()).ok()
    }
    pub unsafe fn AuthzInterfaceClsid(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.AuthzInterfaceClsid)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetAuthzInterfaceClsid<P0>(&self, bstrprop: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetAuthzInterfaceClsid)(::windows::core::Interface::as_raw(self), bstrprop.into_param().abi()).ok()
    }
    pub unsafe fn Version(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.Version)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetVersion<P0>(&self, bstrprop: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetVersion)(::windows::core::Interface::as_raw(self), bstrprop.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GenerateAudits(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.GenerateAudits)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetGenerateAudits<P0>(&self, bprop: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.SetGenerateAudits)(::windows::core::Interface::as_raw(self), bprop.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ApplyStoreSacl(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.ApplyStoreSacl)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetApplyStoreSacl<P0>(&self, bprop: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.SetApplyStoreSacl)(::windows::core::Interface::as_raw(self), bprop.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Writable(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.Writable)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetProperty(&self, lpropid: i32, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.GetProperty)(::windows::core::Interface::as_raw(self), lpropid, ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetProperty(&self, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetProperty)(::windows::core::Interface::as_raw(self), lpropid, ::core::mem::transmute(varprop), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PolicyAdministrators(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.PolicyAdministrators)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PolicyReaders(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.PolicyReaders)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPolicyAdministrator<P0>(&self, bstradmin: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.AddPolicyAdministrator)(::windows::core::Interface::as_raw(self), bstradmin.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePolicyAdministrator<P0>(&self, bstradmin: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.DeletePolicyAdministrator)(::windows::core::Interface::as_raw(self), bstradmin.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPolicyReader<P0>(&self, bstrreader: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.AddPolicyReader)(::windows::core::Interface::as_raw(self), bstrreader.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePolicyReader<P0>(&self, bstrreader: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.DeletePolicyReader)(::windows::core::Interface::as_raw(self), bstrreader.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Scopes(&self) -> ::windows::core::Result<IAzScopes> {
        let mut result__ = ::windows::core::zeroed::<IAzScopes>();
        (::windows::core::Interface::vtable(self).base__.Scopes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn OpenScope<P0>(&self, bstrscopename: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzScope>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzScope>();
        (::windows::core::Interface::vtable(self).base__.OpenScope)(::windows::core::Interface::as_raw(self), bstrscopename.into_param().abi(), ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateScope<P0>(&self, bstrscopename: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzScope>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzScope>();
        (::windows::core::Interface::vtable(self).base__.CreateScope)(::windows::core::Interface::as_raw(self), bstrscopename.into_param().abi(), ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteScope<P0>(&self, bstrscopename: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.DeleteScope)(::windows::core::Interface::as_raw(self), bstrscopename.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Operations(&self) -> ::windows::core::Result<IAzOperations> {
        let mut result__ = ::windows::core::zeroed::<IAzOperations>();
        (::windows::core::Interface::vtable(self).base__.Operations)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn OpenOperation<P0>(&self, bstroperationname: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzOperation>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzOperation>();
        (::windows::core::Interface::vtable(self).base__.OpenOperation)(::windows::core::Interface::as_raw(self), bstroperationname.into_param().abi(), ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateOperation<P0>(&self, bstroperationname: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzOperation>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzOperation>();
        (::windows::core::Interface::vtable(self).base__.CreateOperation)(::windows::core::Interface::as_raw(self), bstroperationname.into_param().abi(), ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteOperation<P0>(&self, bstroperationname: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.DeleteOperation)(::windows::core::Interface::as_raw(self), bstroperationname.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Tasks(&self) -> ::windows::core::Result<IAzTasks> {
        let mut result__ = ::windows::core::zeroed::<IAzTasks>();
        (::windows::core::Interface::vtable(self).base__.Tasks)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn OpenTask<P0>(&self, bstrtaskname: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzTask>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzTask>();
        (::windows::core::Interface::vtable(self).base__.OpenTask)(::windows::core::Interface::as_raw(self), bstrtaskname.into_param().abi(), ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateTask<P0>(&self, bstrtaskname: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzTask>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzTask>();
        (::windows::core::Interface::vtable(self).base__.CreateTask)(::windows::core::Interface::as_raw(self), bstrtaskname.into_param().abi(), ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteTask<P0>(&self, bstrtaskname: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.DeleteTask)(::windows::core::Interface::as_raw(self), bstrtaskname.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ApplicationGroups(&self) -> ::windows::core::Result<IAzApplicationGroups> {
        let mut result__ = ::windows::core::zeroed::<IAzApplicationGroups>();
        (::windows::core::Interface::vtable(self).base__.ApplicationGroups)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn OpenApplicationGroup<P0>(&self, bstrgroupname: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzApplicationGroup>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzApplicationGroup>();
        (::windows::core::Interface::vtable(self).base__.OpenApplicationGroup)(::windows::core::Interface::as_raw(self), bstrgroupname.into_param().abi(), ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateApplicationGroup<P0>(&self, bstrgroupname: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzApplicationGroup>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzApplicationGroup>();
        (::windows::core::Interface::vtable(self).base__.CreateApplicationGroup)(::windows::core::Interface::as_raw(self), bstrgroupname.into_param().abi(), ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteApplicationGroup<P0>(&self, bstrgroupname: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.DeleteApplicationGroup)(::windows::core::Interface::as_raw(self), bstrgroupname.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Roles(&self) -> ::windows::core::Result<IAzRoles> {
        let mut result__ = ::windows::core::zeroed::<IAzRoles>();
        (::windows::core::Interface::vtable(self).base__.Roles)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn OpenRole<P0>(&self, bstrrolename: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzRole>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzRole>();
        (::windows::core::Interface::vtable(self).base__.OpenRole)(::windows::core::Interface::as_raw(self), bstrrolename.into_param().abi(), ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateRole<P0>(&self, bstrrolename: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzRole>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzRole>();
        (::windows::core::Interface::vtable(self).base__.CreateRole)(::windows::core::Interface::as_raw(self), bstrrolename.into_param().abi(), ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteRole<P0>(&self, bstrrolename: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.DeleteRole)(::windows::core::Interface::as_raw(self), bstrrolename.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn InitializeClientContextFromToken(&self, ulltokenhandle: u64, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzClientContext> {
        let mut result__ = ::windows::core::zeroed::<IAzClientContext>();
        (::windows::core::Interface::vtable(self).base__.InitializeClientContextFromToken)(::windows::core::Interface::as_raw(self), ulltokenhandle, ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPropertyItem(&self, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.AddPropertyItem)(::windows::core::Interface::as_raw(self), lpropid, ::core::mem::transmute(varprop), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePropertyItem(&self, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.DeletePropertyItem)(::windows::core::Interface::as_raw(self), lpropid, ::core::mem::transmute(varprop), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Submit(&self, lflags: i32, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Submit)(::windows::core::Interface::as_raw(self), lflags, ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn InitializeClientContextFromName<P0, P1>(&self, clientname: P0, domainname: P1, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzClientContext>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzClientContext>();
        (::windows::core::Interface::vtable(self).base__.InitializeClientContextFromName)(::windows::core::Interface::as_raw(self), clientname.into_param().abi(), domainname.into_param().abi(), ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DelegatedPolicyUsers(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.DelegatedPolicyUsers)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddDelegatedPolicyUser<P0>(&self, bstrdelegatedpolicyuser: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.AddDelegatedPolicyUser)(::windows::core::Interface::as_raw(self), bstrdelegatedpolicyuser.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteDelegatedPolicyUser<P0>(&self, bstrdelegatedpolicyuser: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.DeleteDelegatedPolicyUser)(::windows::core::Interface::as_raw(self), bstrdelegatedpolicyuser.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn InitializeClientContextFromStringSid<P0>(&self, sidstring: P0, loptions: i32, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzClientContext>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzClientContext>();
        (::windows::core::Interface::vtable(self).base__.InitializeClientContextFromStringSid)(::windows::core::Interface::as_raw(self), sidstring.into_param().abi(), loptions, ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PolicyAdministratorsName(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.PolicyAdministratorsName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PolicyReadersName(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.PolicyReadersName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPolicyAdministratorName<P0>(&self, bstradmin: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.AddPolicyAdministratorName)(::windows::core::Interface::as_raw(self), bstradmin.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePolicyAdministratorName<P0>(&self, bstradmin: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.DeletePolicyAdministratorName)(::windows::core::Interface::as_raw(self), bstradmin.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPolicyReaderName<P0>(&self, bstrreader: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.AddPolicyReaderName)(::windows::core::Interface::as_raw(self), bstrreader.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePolicyReaderName<P0>(&self, bstrreader: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.DeletePolicyReaderName)(::windows::core::Interface::as_raw(self), bstrreader.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DelegatedPolicyUsersName(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.DelegatedPolicyUsersName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddDelegatedPolicyUserName<P0>(&self, bstrdelegatedpolicyuser: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.AddDelegatedPolicyUserName)(::windows::core::Interface::as_raw(self), bstrdelegatedpolicyuser.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteDelegatedPolicyUserName<P0>(&self, bstrdelegatedpolicyuser: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.DeleteDelegatedPolicyUserName)(::windows::core::Interface::as_raw(self), bstrdelegatedpolicyuser.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn InitializeClientContextFromToken2(&self, ultokenhandlelowpart: u32, ultokenhandlehighpart: u32, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzClientContext2> {
        let mut result__ = ::windows::core::zeroed::<IAzClientContext2>();
        (::windows::core::Interface::vtable(self).InitializeClientContextFromToken2)(::windows::core::Interface::as_raw(self), ultokenhandlelowpart, ultokenhandlehighpart, ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn InitializeClientContext2<P0>(&self, identifyingstring: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzClientContext2>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzClientContext2>();
        (::windows::core::Interface::vtable(self).InitializeClientContext2)(::windows::core::Interface::as_raw(self), identifyingstring.into_param().abi(), ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IAzApplication2, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IAzApplication);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAzApplication2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAzApplication2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAzApplication2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAzApplication2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IAzApplication2 {
    type Vtable = IAzApplication2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAzApplication2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IAzApplication2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x086a68af_a249_437c_b18d_d4d86d6a9660);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzApplication2_Vtbl {
    pub base__: IAzApplication_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub InitializeClientContextFromToken2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ultokenhandlelowpart: u32, ultokenhandlehighpart: u32, varreserved: super::super::System::Com::VARIANT, ppclientcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    InitializeClientContextFromToken2: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub InitializeClientContext2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, identifyingstring: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT, ppclientcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    InitializeClientContext2: usize,
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAzApplication3(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAzApplication3 {
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.base__.Name)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetName<P0>(&self, bstrname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetName)(::windows::core::Interface::as_raw(self), bstrname.into_param().abi()).ok()
    }
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.base__.Description)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDescription<P0>(&self, bstrdescription: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetDescription)(::windows::core::Interface::as_raw(self), bstrdescription.into_param().abi()).ok()
    }
    pub unsafe fn ApplicationData(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.base__.ApplicationData)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetApplicationData<P0>(&self, bstrapplicationdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetApplicationData)(::windows::core::Interface::as_raw(self), bstrapplicationdata.into_param().abi()).ok()
    }
    pub unsafe fn AuthzInterfaceClsid(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.base__.AuthzInterfaceClsid)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetAuthzInterfaceClsid<P0>(&self, bstrprop: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetAuthzInterfaceClsid)(::windows::core::Interface::as_raw(self), bstrprop.into_param().abi()).ok()
    }
    pub unsafe fn Version(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.base__.Version)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetVersion<P0>(&self, bstrprop: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetVersion)(::windows::core::Interface::as_raw(self), bstrprop.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GenerateAudits(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.base__.GenerateAudits)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetGenerateAudits<P0>(&self, bprop: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetGenerateAudits)(::windows::core::Interface::as_raw(self), bprop.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ApplyStoreSacl(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.base__.ApplyStoreSacl)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetApplyStoreSacl<P0>(&self, bprop: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetApplyStoreSacl)(::windows::core::Interface::as_raw(self), bprop.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Writable(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.base__.Writable)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetProperty(&self, lpropid: i32, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.base__.GetProperty)(::windows::core::Interface::as_raw(self), lpropid, ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetProperty(&self, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetProperty)(::windows::core::Interface::as_raw(self), lpropid, ::core::mem::transmute(varprop), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PolicyAdministrators(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.base__.PolicyAdministrators)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PolicyReaders(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.base__.PolicyReaders)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPolicyAdministrator<P0>(&self, bstradmin: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.AddPolicyAdministrator)(::windows::core::Interface::as_raw(self), bstradmin.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePolicyAdministrator<P0>(&self, bstradmin: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.DeletePolicyAdministrator)(::windows::core::Interface::as_raw(self), bstradmin.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPolicyReader<P0>(&self, bstrreader: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.AddPolicyReader)(::windows::core::Interface::as_raw(self), bstrreader.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePolicyReader<P0>(&self, bstrreader: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.DeletePolicyReader)(::windows::core::Interface::as_raw(self), bstrreader.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Scopes(&self) -> ::windows::core::Result<IAzScopes> {
        let mut result__ = ::windows::core::zeroed::<IAzScopes>();
        (::windows::core::Interface::vtable(self).base__.base__.Scopes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn OpenScope<P0>(&self, bstrscopename: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzScope>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzScope>();
        (::windows::core::Interface::vtable(self).base__.base__.OpenScope)(::windows::core::Interface::as_raw(self), bstrscopename.into_param().abi(), ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateScope<P0>(&self, bstrscopename: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzScope>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzScope>();
        (::windows::core::Interface::vtable(self).base__.base__.CreateScope)(::windows::core::Interface::as_raw(self), bstrscopename.into_param().abi(), ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteScope<P0>(&self, bstrscopename: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.DeleteScope)(::windows::core::Interface::as_raw(self), bstrscopename.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Operations(&self) -> ::windows::core::Result<IAzOperations> {
        let mut result__ = ::windows::core::zeroed::<IAzOperations>();
        (::windows::core::Interface::vtable(self).base__.base__.Operations)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn OpenOperation<P0>(&self, bstroperationname: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzOperation>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzOperation>();
        (::windows::core::Interface::vtable(self).base__.base__.OpenOperation)(::windows::core::Interface::as_raw(self), bstroperationname.into_param().abi(), ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateOperation<P0>(&self, bstroperationname: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzOperation>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzOperation>();
        (::windows::core::Interface::vtable(self).base__.base__.CreateOperation)(::windows::core::Interface::as_raw(self), bstroperationname.into_param().abi(), ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteOperation<P0>(&self, bstroperationname: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.DeleteOperation)(::windows::core::Interface::as_raw(self), bstroperationname.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Tasks(&self) -> ::windows::core::Result<IAzTasks> {
        let mut result__ = ::windows::core::zeroed::<IAzTasks>();
        (::windows::core::Interface::vtable(self).base__.base__.Tasks)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn OpenTask<P0>(&self, bstrtaskname: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzTask>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzTask>();
        (::windows::core::Interface::vtable(self).base__.base__.OpenTask)(::windows::core::Interface::as_raw(self), bstrtaskname.into_param().abi(), ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateTask<P0>(&self, bstrtaskname: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzTask>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzTask>();
        (::windows::core::Interface::vtable(self).base__.base__.CreateTask)(::windows::core::Interface::as_raw(self), bstrtaskname.into_param().abi(), ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteTask<P0>(&self, bstrtaskname: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.DeleteTask)(::windows::core::Interface::as_raw(self), bstrtaskname.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ApplicationGroups(&self) -> ::windows::core::Result<IAzApplicationGroups> {
        let mut result__ = ::windows::core::zeroed::<IAzApplicationGroups>();
        (::windows::core::Interface::vtable(self).base__.base__.ApplicationGroups)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn OpenApplicationGroup<P0>(&self, bstrgroupname: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzApplicationGroup>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzApplicationGroup>();
        (::windows::core::Interface::vtable(self).base__.base__.OpenApplicationGroup)(::windows::core::Interface::as_raw(self), bstrgroupname.into_param().abi(), ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateApplicationGroup<P0>(&self, bstrgroupname: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzApplicationGroup>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzApplicationGroup>();
        (::windows::core::Interface::vtable(self).base__.base__.CreateApplicationGroup)(::windows::core::Interface::as_raw(self), bstrgroupname.into_param().abi(), ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteApplicationGroup<P0>(&self, bstrgroupname: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.DeleteApplicationGroup)(::windows::core::Interface::as_raw(self), bstrgroupname.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Roles(&self) -> ::windows::core::Result<IAzRoles> {
        let mut result__ = ::windows::core::zeroed::<IAzRoles>();
        (::windows::core::Interface::vtable(self).base__.base__.Roles)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn OpenRole<P0>(&self, bstrrolename: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzRole>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzRole>();
        (::windows::core::Interface::vtable(self).base__.base__.OpenRole)(::windows::core::Interface::as_raw(self), bstrrolename.into_param().abi(), ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateRole<P0>(&self, bstrrolename: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzRole>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzRole>();
        (::windows::core::Interface::vtable(self).base__.base__.CreateRole)(::windows::core::Interface::as_raw(self), bstrrolename.into_param().abi(), ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteRole<P0>(&self, bstrrolename: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.DeleteRole)(::windows::core::Interface::as_raw(self), bstrrolename.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn InitializeClientContextFromToken(&self, ulltokenhandle: u64, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzClientContext> {
        let mut result__ = ::windows::core::zeroed::<IAzClientContext>();
        (::windows::core::Interface::vtable(self).base__.base__.InitializeClientContextFromToken)(::windows::core::Interface::as_raw(self), ulltokenhandle, ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPropertyItem(&self, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.AddPropertyItem)(::windows::core::Interface::as_raw(self), lpropid, ::core::mem::transmute(varprop), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePropertyItem(&self, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.DeletePropertyItem)(::windows::core::Interface::as_raw(self), lpropid, ::core::mem::transmute(varprop), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Submit(&self, lflags: i32, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.Submit)(::windows::core::Interface::as_raw(self), lflags, ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn InitializeClientContextFromName<P0, P1>(&self, clientname: P0, domainname: P1, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzClientContext>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzClientContext>();
        (::windows::core::Interface::vtable(self).base__.base__.InitializeClientContextFromName)(::windows::core::Interface::as_raw(self), clientname.into_param().abi(), domainname.into_param().abi(), ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DelegatedPolicyUsers(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.base__.DelegatedPolicyUsers)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddDelegatedPolicyUser<P0>(&self, bstrdelegatedpolicyuser: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.AddDelegatedPolicyUser)(::windows::core::Interface::as_raw(self), bstrdelegatedpolicyuser.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteDelegatedPolicyUser<P0>(&self, bstrdelegatedpolicyuser: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.DeleteDelegatedPolicyUser)(::windows::core::Interface::as_raw(self), bstrdelegatedpolicyuser.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn InitializeClientContextFromStringSid<P0>(&self, sidstring: P0, loptions: i32, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzClientContext>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzClientContext>();
        (::windows::core::Interface::vtable(self).base__.base__.InitializeClientContextFromStringSid)(::windows::core::Interface::as_raw(self), sidstring.into_param().abi(), loptions, ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PolicyAdministratorsName(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.base__.PolicyAdministratorsName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PolicyReadersName(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.base__.PolicyReadersName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPolicyAdministratorName<P0>(&self, bstradmin: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.AddPolicyAdministratorName)(::windows::core::Interface::as_raw(self), bstradmin.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePolicyAdministratorName<P0>(&self, bstradmin: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.DeletePolicyAdministratorName)(::windows::core::Interface::as_raw(self), bstradmin.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPolicyReaderName<P0>(&self, bstrreader: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.AddPolicyReaderName)(::windows::core::Interface::as_raw(self), bstrreader.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePolicyReaderName<P0>(&self, bstrreader: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.DeletePolicyReaderName)(::windows::core::Interface::as_raw(self), bstrreader.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DelegatedPolicyUsersName(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.base__.DelegatedPolicyUsersName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddDelegatedPolicyUserName<P0>(&self, bstrdelegatedpolicyuser: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.AddDelegatedPolicyUserName)(::windows::core::Interface::as_raw(self), bstrdelegatedpolicyuser.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteDelegatedPolicyUserName<P0>(&self, bstrdelegatedpolicyuser: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.DeleteDelegatedPolicyUserName)(::windows::core::Interface::as_raw(self), bstrdelegatedpolicyuser.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn InitializeClientContextFromToken2(&self, ultokenhandlelowpart: u32, ultokenhandlehighpart: u32, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzClientContext2> {
        let mut result__ = ::windows::core::zeroed::<IAzClientContext2>();
        (::windows::core::Interface::vtable(self).base__.InitializeClientContextFromToken2)(::windows::core::Interface::as_raw(self), ultokenhandlelowpart, ultokenhandlehighpart, ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn InitializeClientContext2<P0>(&self, identifyingstring: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzClientContext2>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzClientContext2>();
        (::windows::core::Interface::vtable(self).base__.InitializeClientContext2)(::windows::core::Interface::as_raw(self), identifyingstring.into_param().abi(), ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ScopeExists<P0>(&self, bstrscopename: P0) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).ScopeExists)(::windows::core::Interface::as_raw(self), bstrscopename.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenScope2<P0>(&self, bstrscopename: P0) -> ::windows::core::Result<IAzScope2>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzScope2>();
        (::windows::core::Interface::vtable(self).OpenScope2)(::windows::core::Interface::as_raw(self), bstrscopename.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateScope2<P0>(&self, bstrscopename: P0) -> ::windows::core::Result<IAzScope2>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzScope2>();
        (::windows::core::Interface::vtable(self).CreateScope2)(::windows::core::Interface::as_raw(self), bstrscopename.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn DeleteScope2<P0>(&self, bstrscopename: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).DeleteScope2)(::windows::core::Interface::as_raw(self), bstrscopename.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RoleDefinitions(&self) -> ::windows::core::Result<IAzRoleDefinitions> {
        let mut result__ = ::windows::core::zeroed::<IAzRoleDefinitions>();
        (::windows::core::Interface::vtable(self).RoleDefinitions)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateRoleDefinition<P0>(&self, bstrroledefinitionname: P0) -> ::windows::core::Result<IAzRoleDefinition>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzRoleDefinition>();
        (::windows::core::Interface::vtable(self).CreateRoleDefinition)(::windows::core::Interface::as_raw(self), bstrroledefinitionname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenRoleDefinition<P0>(&self, bstrroledefinitionname: P0) -> ::windows::core::Result<IAzRoleDefinition>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzRoleDefinition>();
        (::windows::core::Interface::vtable(self).OpenRoleDefinition)(::windows::core::Interface::as_raw(self), bstrroledefinitionname.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn DeleteRoleDefinition<P0>(&self, bstrroledefinitionname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).DeleteRoleDefinition)(::windows::core::Interface::as_raw(self), bstrroledefinitionname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RoleAssignments(&self) -> ::windows::core::Result<IAzRoleAssignments> {
        let mut result__ = ::windows::core::zeroed::<IAzRoleAssignments>();
        (::windows::core::Interface::vtable(self).RoleAssignments)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateRoleAssignment<P0>(&self, bstrroleassignmentname: P0) -> ::windows::core::Result<IAzRoleAssignment>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzRoleAssignment>();
        (::windows::core::Interface::vtable(self).CreateRoleAssignment)(::windows::core::Interface::as_raw(self), bstrroleassignmentname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenRoleAssignment<P0>(&self, bstrroleassignmentname: P0) -> ::windows::core::Result<IAzRoleAssignment>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzRoleAssignment>();
        (::windows::core::Interface::vtable(self).OpenRoleAssignment)(::windows::core::Interface::as_raw(self), bstrroleassignmentname.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn DeleteRoleAssignment<P0>(&self, bstrroleassignmentname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).DeleteRoleAssignment)(::windows::core::Interface::as_raw(self), bstrroleassignmentname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BizRulesEnabled(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).BizRulesEnabled)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBizRulesEnabled<P0>(&self, benabled: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetBizRulesEnabled)(::windows::core::Interface::as_raw(self), benabled.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IAzApplication3, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IAzApplication, IAzApplication2);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAzApplication3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAzApplication3 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAzApplication3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAzApplication3").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IAzApplication3 {
    type Vtable = IAzApplication3_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAzApplication3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IAzApplication3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x181c845e_7196_4a7d_ac2e_020c0bb7a303);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzApplication3_Vtbl {
    pub base__: IAzApplication2_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub ScopeExists: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrscopename: ::std::mem::MaybeUninit<::windows::core::BSTR>, pbexist: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ScopeExists: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OpenScope2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrscopename: ::std::mem::MaybeUninit<::windows::core::BSTR>, ppscope2: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OpenScope2: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateScope2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrscopename: ::std::mem::MaybeUninit<::windows::core::BSTR>, ppscope2: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateScope2: usize,
    pub DeleteScope2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrscopename: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub RoleDefinitions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pproledefinitions: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RoleDefinitions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateRoleDefinition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrroledefinitionname: ::std::mem::MaybeUninit<::windows::core::BSTR>, pproledefinitions: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateRoleDefinition: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OpenRoleDefinition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrroledefinitionname: ::std::mem::MaybeUninit<::windows::core::BSTR>, pproledefinitions: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OpenRoleDefinition: usize,
    pub DeleteRoleDefinition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrroledefinitionname: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub RoleAssignments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pproleassignments: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RoleAssignments: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateRoleAssignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrroleassignmentname: ::std::mem::MaybeUninit<::windows::core::BSTR>, pproleassignment: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateRoleAssignment: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OpenRoleAssignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrroleassignmentname: ::std::mem::MaybeUninit<::windows::core::BSTR>, pproleassignment: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OpenRoleAssignment: usize,
    pub DeleteRoleAssignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrroleassignmentname: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub BizRulesEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BizRulesEnabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetBizRulesEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, benabled: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetBizRulesEnabled: usize,
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAzApplicationGroup(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAzApplicationGroup {
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).Name)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetName<P0>(&self, bstrname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetName)(::windows::core::Interface::as_raw(self), bstrname.into_param().abi()).ok()
    }
    pub unsafe fn Type(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).Type)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetType(&self, lprop: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetType)(::windows::core::Interface::as_raw(self), lprop).ok()
    }
    pub unsafe fn LdapQuery(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).LdapQuery)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLdapQuery<P0>(&self, bstrprop: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetLdapQuery)(::windows::core::Interface::as_raw(self), bstrprop.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AppMembers(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).AppMembers)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AppNonMembers(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).AppNonMembers)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Members(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).Members)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn NonMembers(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).NonMembers)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).Description)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDescription<P0>(&self, bstrdescription: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetDescription)(::windows::core::Interface::as_raw(self), bstrdescription.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddAppMember<P0>(&self, bstrprop: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).AddAppMember)(::windows::core::Interface::as_raw(self), bstrprop.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteAppMember<P0>(&self, bstrprop: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).DeleteAppMember)(::windows::core::Interface::as_raw(self), bstrprop.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddAppNonMember<P0>(&self, bstrprop: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).AddAppNonMember)(::windows::core::Interface::as_raw(self), bstrprop.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteAppNonMember<P0>(&self, bstrprop: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).DeleteAppNonMember)(::windows::core::Interface::as_raw(self), bstrprop.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddMember<P0>(&self, bstrprop: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).AddMember)(::windows::core::Interface::as_raw(self), bstrprop.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteMember<P0>(&self, bstrprop: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).DeleteMember)(::windows::core::Interface::as_raw(self), bstrprop.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddNonMember<P0>(&self, bstrprop: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).AddNonMember)(::windows::core::Interface::as_raw(self), bstrprop.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteNonMember<P0>(&self, bstrprop: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).DeleteNonMember)(::windows::core::Interface::as_raw(self), bstrprop.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Writable(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).Writable)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetProperty(&self, lpropid: i32, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).GetProperty)(::windows::core::Interface::as_raw(self), lpropid, ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetProperty(&self, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetProperty)(::windows::core::Interface::as_raw(self), lpropid, ::core::mem::transmute(varprop), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPropertyItem(&self, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddPropertyItem)(::windows::core::Interface::as_raw(self), lpropid, ::core::mem::transmute(varprop), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePropertyItem(&self, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeletePropertyItem)(::windows::core::Interface::as_raw(self), lpropid, ::core::mem::transmute(varprop), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Submit(&self, lflags: i32, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Submit)(::windows::core::Interface::as_raw(self), lflags, ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddMemberName<P0>(&self, bstrprop: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).AddMemberName)(::windows::core::Interface::as_raw(self), bstrprop.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteMemberName<P0>(&self, bstrprop: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).DeleteMemberName)(::windows::core::Interface::as_raw(self), bstrprop.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddNonMemberName<P0>(&self, bstrprop: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).AddNonMemberName)(::windows::core::Interface::as_raw(self), bstrprop.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteNonMemberName<P0>(&self, bstrprop: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).DeleteNonMemberName)(::windows::core::Interface::as_raw(self), bstrprop.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn MembersName(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).MembersName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn NonMembersName(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).NonMembersName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IAzApplicationGroup, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAzApplicationGroup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAzApplicationGroup {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAzApplicationGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAzApplicationGroup").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IAzApplicationGroup {
    type Vtable = IAzApplicationGroup_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAzApplicationGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IAzApplicationGroup {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf1b744cd_58a6_4e06_9fbf_36f6d779e21e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzApplicationGroup_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plprop: *mut i32) -> ::windows::core::HRESULT,
    pub SetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lprop: i32) -> ::windows::core::HRESULT,
    pub LdapQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrprop: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetLdapQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AppMembers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AppMembers: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AppNonMembers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AppNonMembers: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Members: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Members: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub NonMembers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    NonMembers: usize,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdescription: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdescription: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddAppMember: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddAppMember: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteAppMember: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteAppMember: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddAppNonMember: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddAppNonMember: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteAppNonMember: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteAppNonMember: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddMember: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddMember: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteMember: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteMember: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddNonMember: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddNonMember: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteNonMember: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteNonMember: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Writable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfprop: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Writable: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpropid: i32, varreserved: super::super::System::Com::VARIANT, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddPropertyItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddPropertyItem: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeletePropertyItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeletePropertyItem: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Submit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Submit: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddMemberName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddMemberName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteMemberName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteMemberName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddNonMemberName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddNonMemberName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteNonMemberName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteNonMemberName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub MembersName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    MembersName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub NonMembersName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    NonMembersName: usize,
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAzApplicationGroup2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAzApplicationGroup2 {
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.Name)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetName<P0>(&self, bstrname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetName)(::windows::core::Interface::as_raw(self), bstrname.into_param().abi()).ok()
    }
    pub unsafe fn Type(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).base__.Type)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetType(&self, lprop: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetType)(::windows::core::Interface::as_raw(self), lprop).ok()
    }
    pub unsafe fn LdapQuery(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.LdapQuery)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLdapQuery<P0>(&self, bstrprop: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetLdapQuery)(::windows::core::Interface::as_raw(self), bstrprop.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AppMembers(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.AppMembers)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AppNonMembers(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.AppNonMembers)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Members(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.Members)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn NonMembers(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.NonMembers)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.Description)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDescription<P0>(&self, bstrdescription: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetDescription)(::windows::core::Interface::as_raw(self), bstrdescription.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddAppMember<P0>(&self, bstrprop: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.AddAppMember)(::windows::core::Interface::as_raw(self), bstrprop.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteAppMember<P0>(&self, bstrprop: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.DeleteAppMember)(::windows::core::Interface::as_raw(self), bstrprop.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddAppNonMember<P0>(&self, bstrprop: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.AddAppNonMember)(::windows::core::Interface::as_raw(self), bstrprop.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteAppNonMember<P0>(&self, bstrprop: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.DeleteAppNonMember)(::windows::core::Interface::as_raw(self), bstrprop.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddMember<P0>(&self, bstrprop: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.AddMember)(::windows::core::Interface::as_raw(self), bstrprop.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteMember<P0>(&self, bstrprop: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.DeleteMember)(::windows::core::Interface::as_raw(self), bstrprop.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddNonMember<P0>(&self, bstrprop: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.AddNonMember)(::windows::core::Interface::as_raw(self), bstrprop.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteNonMember<P0>(&self, bstrprop: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.DeleteNonMember)(::windows::core::Interface::as_raw(self), bstrprop.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Writable(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.Writable)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetProperty(&self, lpropid: i32, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.GetProperty)(::windows::core::Interface::as_raw(self), lpropid, ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetProperty(&self, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetProperty)(::windows::core::Interface::as_raw(self), lpropid, ::core::mem::transmute(varprop), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPropertyItem(&self, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.AddPropertyItem)(::windows::core::Interface::as_raw(self), lpropid, ::core::mem::transmute(varprop), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePropertyItem(&self, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.DeletePropertyItem)(::windows::core::Interface::as_raw(self), lpropid, ::core::mem::transmute(varprop), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Submit(&self, lflags: i32, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Submit)(::windows::core::Interface::as_raw(self), lflags, ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddMemberName<P0>(&self, bstrprop: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.AddMemberName)(::windows::core::Interface::as_raw(self), bstrprop.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteMemberName<P0>(&self, bstrprop: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.DeleteMemberName)(::windows::core::Interface::as_raw(self), bstrprop.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddNonMemberName<P0>(&self, bstrprop: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.AddNonMemberName)(::windows::core::Interface::as_raw(self), bstrprop.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteNonMemberName<P0>(&self, bstrprop: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.DeleteNonMemberName)(::windows::core::Interface::as_raw(self), bstrprop.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn MembersName(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.MembersName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn NonMembersName(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.NonMembersName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn BizRule(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).BizRule)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetBizRule<P0>(&self, bstrprop: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetBizRule)(::windows::core::Interface::as_raw(self), bstrprop.into_param().abi()).ok()
    }
    pub unsafe fn BizRuleLanguage(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).BizRuleLanguage)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetBizRuleLanguage<P0>(&self, bstrprop: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetBizRuleLanguage)(::windows::core::Interface::as_raw(self), bstrprop.into_param().abi()).ok()
    }
    pub unsafe fn BizRuleImportedPath(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).BizRuleImportedPath)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetBizRuleImportedPath<P0>(&self, bstrprop: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetBizRuleImportedPath)(::windows::core::Interface::as_raw(self), bstrprop.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn RoleAssignments<P0, P1>(&self, bstrscopename: P0, brecursive: P1) -> ::windows::core::Result<IAzRoleAssignments>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzRoleAssignments>();
        (::windows::core::Interface::vtable(self).RoleAssignments)(::windows::core::Interface::as_raw(self), bstrscopename.into_param().abi(), brecursive.into_param().abi(), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IAzApplicationGroup2, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IAzApplicationGroup);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAzApplicationGroup2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAzApplicationGroup2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAzApplicationGroup2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAzApplicationGroup2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IAzApplicationGroup2 {
    type Vtable = IAzApplicationGroup2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAzApplicationGroup2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IAzApplicationGroup2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3f0613fc_b71a_464e_a11d_5b881a56cefa);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzApplicationGroup2_Vtbl {
    pub base__: IAzApplicationGroup_Vtbl,
    pub BizRule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrprop: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetBizRule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub BizRuleLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrprop: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetBizRuleLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub BizRuleImportedPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrprop: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetBizRuleImportedPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub RoleAssignments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrscopename: ::std::mem::MaybeUninit<::windows::core::BSTR>, brecursive: super::super::Foundation::VARIANT_BOOL, pproleassignments: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    RoleAssignments: usize,
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAzApplicationGroups(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAzApplicationGroups {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_Item(&self, index: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).get_Item)(::windows::core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).Count)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self)._NewEnum)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IAzApplicationGroups, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAzApplicationGroups {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAzApplicationGroups {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAzApplicationGroups {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAzApplicationGroups").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IAzApplicationGroups {
    type Vtable = IAzApplicationGroups_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAzApplicationGroups {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IAzApplicationGroups {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4ce66ad5_9f3c_469d_a911_b99887a7e685);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzApplicationGroups_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, pvarobtptr: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumptr: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAzApplications(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAzApplications {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_Item(&self, index: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).get_Item)(::windows::core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).Count)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self)._NewEnum)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IAzApplications, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAzApplications {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAzApplications {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAzApplications {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAzApplications").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IAzApplications {
    type Vtable = IAzApplications_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAzApplications {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IAzApplications {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x929b11a9_95c5_4a84_a29a_20ad42c2f16c);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzApplications_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, pvarobtptr: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumptr: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAzAuthorizationStore(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAzAuthorizationStore {
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).Description)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDescription<P0>(&self, bstrdescription: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetDescription)(::windows::core::Interface::as_raw(self), bstrdescription.into_param().abi()).ok()
    }
    pub unsafe fn ApplicationData(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).ApplicationData)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetApplicationData<P0>(&self, bstrapplicationdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetApplicationData)(::windows::core::Interface::as_raw(self), bstrapplicationdata.into_param().abi()).ok()
    }
    pub unsafe fn DomainTimeout(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).DomainTimeout)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDomainTimeout(&self, lprop: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDomainTimeout)(::windows::core::Interface::as_raw(self), lprop).ok()
    }
    pub unsafe fn ScriptEngineTimeout(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).ScriptEngineTimeout)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetScriptEngineTimeout(&self, lprop: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetScriptEngineTimeout)(::windows::core::Interface::as_raw(self), lprop).ok()
    }
    pub unsafe fn MaxScriptEngines(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).MaxScriptEngines)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetMaxScriptEngines(&self, lprop: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMaxScriptEngines)(::windows::core::Interface::as_raw(self), lprop).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GenerateAudits(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).GenerateAudits)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetGenerateAudits<P0>(&self, bprop: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetGenerateAudits)(::windows::core::Interface::as_raw(self), bprop.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Writable(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).Writable)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetProperty(&self, lpropid: i32, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).GetProperty)(::windows::core::Interface::as_raw(self), lpropid, ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetProperty(&self, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetProperty)(::windows::core::Interface::as_raw(self), lpropid, ::core::mem::transmute(varprop), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPropertyItem(&self, lpropid: AZ_PROP_CONSTANTS, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddPropertyItem)(::windows::core::Interface::as_raw(self), lpropid, ::core::mem::transmute(varprop), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePropertyItem(&self, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeletePropertyItem)(::windows::core::Interface::as_raw(self), lpropid, ::core::mem::transmute(varprop), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PolicyAdministrators(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).PolicyAdministrators)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PolicyReaders(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).PolicyReaders)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPolicyAdministrator<P0>(&self, bstradmin: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).AddPolicyAdministrator)(::windows::core::Interface::as_raw(self), bstradmin.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePolicyAdministrator<P0>(&self, bstradmin: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).DeletePolicyAdministrator)(::windows::core::Interface::as_raw(self), bstradmin.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPolicyReader<P0>(&self, bstrreader: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).AddPolicyReader)(::windows::core::Interface::as_raw(self), bstrreader.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePolicyReader<P0>(&self, bstrreader: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).DeletePolicyReader)(::windows::core::Interface::as_raw(self), bstrreader.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Initialize<P0>(&self, lflags: AZ_PROP_CONSTANTS, bstrpolicyurl: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), lflags, bstrpolicyurl.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn UpdateCache(&self, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UpdateCache)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Delete(&self, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Delete)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Applications(&self) -> ::windows::core::Result<IAzApplications> {
        let mut result__ = ::windows::core::zeroed::<IAzApplications>();
        (::windows::core::Interface::vtable(self).Applications)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn OpenApplication<P0>(&self, bstrapplicationname: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzApplication>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzApplication>();
        (::windows::core::Interface::vtable(self).OpenApplication)(::windows::core::Interface::as_raw(self), bstrapplicationname.into_param().abi(), ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateApplication<P0>(&self, bstrapplicationname: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzApplication>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzApplication>();
        (::windows::core::Interface::vtable(self).CreateApplication)(::windows::core::Interface::as_raw(self), bstrapplicationname.into_param().abi(), ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteApplication<P0>(&self, bstrapplicationname: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).DeleteApplication)(::windows::core::Interface::as_raw(self), bstrapplicationname.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ApplicationGroups(&self) -> ::windows::core::Result<IAzApplicationGroups> {
        let mut result__ = ::windows::core::zeroed::<IAzApplicationGroups>();
        (::windows::core::Interface::vtable(self).ApplicationGroups)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateApplicationGroup<P0>(&self, bstrgroupname: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzApplicationGroup>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzApplicationGroup>();
        (::windows::core::Interface::vtable(self).CreateApplicationGroup)(::windows::core::Interface::as_raw(self), bstrgroupname.into_param().abi(), ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn OpenApplicationGroup<P0>(&self, bstrgroupname: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzApplicationGroup>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzApplicationGroup>();
        (::windows::core::Interface::vtable(self).OpenApplicationGroup)(::windows::core::Interface::as_raw(self), bstrgroupname.into_param().abi(), ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteApplicationGroup<P0>(&self, bstrgroupname: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).DeleteApplicationGroup)(::windows::core::Interface::as_raw(self), bstrgroupname.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Submit(&self, lflags: i32, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Submit)(::windows::core::Interface::as_raw(self), lflags, ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DelegatedPolicyUsers(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).DelegatedPolicyUsers)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddDelegatedPolicyUser<P0>(&self, bstrdelegatedpolicyuser: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).AddDelegatedPolicyUser)(::windows::core::Interface::as_raw(self), bstrdelegatedpolicyuser.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteDelegatedPolicyUser<P0>(&self, bstrdelegatedpolicyuser: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).DeleteDelegatedPolicyUser)(::windows::core::Interface::as_raw(self), bstrdelegatedpolicyuser.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    pub unsafe fn TargetMachine(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).TargetMachine)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ApplyStoreSacl(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).ApplyStoreSacl)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetApplyStoreSacl<P0>(&self, bapplystoresacl: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetApplyStoreSacl)(::windows::core::Interface::as_raw(self), bapplystoresacl.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PolicyAdministratorsName(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).PolicyAdministratorsName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PolicyReadersName(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).PolicyReadersName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPolicyAdministratorName<P0>(&self, bstradmin: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).AddPolicyAdministratorName)(::windows::core::Interface::as_raw(self), bstradmin.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePolicyAdministratorName<P0>(&self, bstradmin: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).DeletePolicyAdministratorName)(::windows::core::Interface::as_raw(self), bstradmin.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPolicyReaderName<P0>(&self, bstrreader: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).AddPolicyReaderName)(::windows::core::Interface::as_raw(self), bstrreader.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePolicyReaderName<P0>(&self, bstrreader: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).DeletePolicyReaderName)(::windows::core::Interface::as_raw(self), bstrreader.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DelegatedPolicyUsersName(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).DelegatedPolicyUsersName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddDelegatedPolicyUserName<P0>(&self, bstrdelegatedpolicyuser: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).AddDelegatedPolicyUserName)(::windows::core::Interface::as_raw(self), bstrdelegatedpolicyuser.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteDelegatedPolicyUserName<P0>(&self, bstrdelegatedpolicyuser: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).DeleteDelegatedPolicyUserName)(::windows::core::Interface::as_raw(self), bstrdelegatedpolicyuser.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    pub unsafe fn CloseApplication<P0>(&self, bstrapplicationname: P0, lflag: i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).CloseApplication)(::windows::core::Interface::as_raw(self), bstrapplicationname.into_param().abi(), lflag).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IAzAuthorizationStore, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAzAuthorizationStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAzAuthorizationStore {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAzAuthorizationStore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAzAuthorizationStore").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IAzAuthorizationStore {
    type Vtable = IAzAuthorizationStore_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAzAuthorizationStore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IAzAuthorizationStore {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xedbd9ca9_9b82_4f6a_9e8b_98301e450f14);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzAuthorizationStore_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdescription: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdescription: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub ApplicationData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrapplicationdata: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetApplicationData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrapplicationdata: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub DomainTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plprop: *mut i32) -> ::windows::core::HRESULT,
    pub SetDomainTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lprop: i32) -> ::windows::core::HRESULT,
    pub ScriptEngineTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plprop: *mut i32) -> ::windows::core::HRESULT,
    pub SetScriptEngineTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lprop: i32) -> ::windows::core::HRESULT,
    pub MaxScriptEngines: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plprop: *mut i32) -> ::windows::core::HRESULT,
    pub SetMaxScriptEngines: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lprop: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GenerateAudits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbprop: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GenerateAudits: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetGenerateAudits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bprop: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetGenerateAudits: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Writable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfprop: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Writable: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpropid: i32, varreserved: super::super::System::Com::VARIANT, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddPropertyItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpropid: AZ_PROP_CONSTANTS, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddPropertyItem: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeletePropertyItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeletePropertyItem: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PolicyAdministrators: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvaradmins: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PolicyAdministrators: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PolicyReaders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarreaders: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PolicyReaders: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddPolicyAdministrator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstradmin: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddPolicyAdministrator: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeletePolicyAdministrator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstradmin: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeletePolicyAdministrator: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddPolicyReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrreader: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddPolicyReader: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeletePolicyReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrreader: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeletePolicyReader: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: AZ_PROP_CONSTANTS, bstrpolicyurl: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Initialize: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub UpdateCache: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    UpdateCache: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Delete: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Applications: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppappcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Applications: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub OpenApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrapplicationname: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT, ppapplication: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    OpenApplication: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrapplicationname: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT, ppapplication: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateApplication: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrapplicationname: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteApplication: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ApplicationGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppgroupcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ApplicationGroups: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateApplicationGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrgroupname: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateApplicationGroup: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub OpenApplicationGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrgroupname: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    OpenApplicationGroup: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteApplicationGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrgroupname: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteApplicationGroup: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Submit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Submit: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DelegatedPolicyUsers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvardelegatedpolicyusers: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DelegatedPolicyUsers: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddDelegatedPolicyUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdelegatedpolicyuser: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddDelegatedPolicyUser: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteDelegatedPolicyUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdelegatedpolicyuser: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteDelegatedPolicyUser: usize,
    pub TargetMachine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtargetmachine: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ApplyStoreSacl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbapplystoresacl: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ApplyStoreSacl: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetApplyStoreSacl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bapplystoresacl: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetApplyStoreSacl: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PolicyAdministratorsName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvaradmins: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PolicyAdministratorsName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PolicyReadersName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarreaders: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PolicyReadersName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddPolicyAdministratorName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstradmin: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddPolicyAdministratorName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeletePolicyAdministratorName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstradmin: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeletePolicyAdministratorName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddPolicyReaderName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrreader: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddPolicyReaderName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeletePolicyReaderName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrreader: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeletePolicyReaderName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DelegatedPolicyUsersName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvardelegatedpolicyusers: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DelegatedPolicyUsersName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddDelegatedPolicyUserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdelegatedpolicyuser: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddDelegatedPolicyUserName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteDelegatedPolicyUserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdelegatedpolicyuser: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteDelegatedPolicyUserName: usize,
    pub CloseApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrapplicationname: ::std::mem::MaybeUninit<::windows::core::BSTR>, lflag: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAzAuthorizationStore2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAzAuthorizationStore2 {
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.Description)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDescription<P0>(&self, bstrdescription: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetDescription)(::windows::core::Interface::as_raw(self), bstrdescription.into_param().abi()).ok()
    }
    pub unsafe fn ApplicationData(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.ApplicationData)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetApplicationData<P0>(&self, bstrapplicationdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetApplicationData)(::windows::core::Interface::as_raw(self), bstrapplicationdata.into_param().abi()).ok()
    }
    pub unsafe fn DomainTimeout(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).base__.DomainTimeout)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDomainTimeout(&self, lprop: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetDomainTimeout)(::windows::core::Interface::as_raw(self), lprop).ok()
    }
    pub unsafe fn ScriptEngineTimeout(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).base__.ScriptEngineTimeout)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetScriptEngineTimeout(&self, lprop: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetScriptEngineTimeout)(::windows::core::Interface::as_raw(self), lprop).ok()
    }
    pub unsafe fn MaxScriptEngines(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).base__.MaxScriptEngines)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetMaxScriptEngines(&self, lprop: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetMaxScriptEngines)(::windows::core::Interface::as_raw(self), lprop).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GenerateAudits(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.GenerateAudits)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetGenerateAudits<P0>(&self, bprop: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.SetGenerateAudits)(::windows::core::Interface::as_raw(self), bprop.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Writable(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.Writable)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetProperty(&self, lpropid: i32, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.GetProperty)(::windows::core::Interface::as_raw(self), lpropid, ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetProperty(&self, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetProperty)(::windows::core::Interface::as_raw(self), lpropid, ::core::mem::transmute(varprop), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPropertyItem(&self, lpropid: AZ_PROP_CONSTANTS, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.AddPropertyItem)(::windows::core::Interface::as_raw(self), lpropid, ::core::mem::transmute(varprop), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePropertyItem(&self, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.DeletePropertyItem)(::windows::core::Interface::as_raw(self), lpropid, ::core::mem::transmute(varprop), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PolicyAdministrators(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.PolicyAdministrators)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PolicyReaders(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.PolicyReaders)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPolicyAdministrator<P0>(&self, bstradmin: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.AddPolicyAdministrator)(::windows::core::Interface::as_raw(self), bstradmin.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePolicyAdministrator<P0>(&self, bstradmin: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.DeletePolicyAdministrator)(::windows::core::Interface::as_raw(self), bstradmin.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPolicyReader<P0>(&self, bstrreader: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.AddPolicyReader)(::windows::core::Interface::as_raw(self), bstrreader.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePolicyReader<P0>(&self, bstrreader: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.DeletePolicyReader)(::windows::core::Interface::as_raw(self), bstrreader.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Initialize<P0>(&self, lflags: AZ_PROP_CONSTANTS, bstrpolicyurl: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.Initialize)(::windows::core::Interface::as_raw(self), lflags, bstrpolicyurl.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn UpdateCache(&self, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.UpdateCache)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Delete(&self, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Delete)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Applications(&self) -> ::windows::core::Result<IAzApplications> {
        let mut result__ = ::windows::core::zeroed::<IAzApplications>();
        (::windows::core::Interface::vtable(self).base__.Applications)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn OpenApplication<P0>(&self, bstrapplicationname: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzApplication>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzApplication>();
        (::windows::core::Interface::vtable(self).base__.OpenApplication)(::windows::core::Interface::as_raw(self), bstrapplicationname.into_param().abi(), ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateApplication<P0>(&self, bstrapplicationname: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzApplication>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzApplication>();
        (::windows::core::Interface::vtable(self).base__.CreateApplication)(::windows::core::Interface::as_raw(self), bstrapplicationname.into_param().abi(), ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteApplication<P0>(&self, bstrapplicationname: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.DeleteApplication)(::windows::core::Interface::as_raw(self), bstrapplicationname.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ApplicationGroups(&self) -> ::windows::core::Result<IAzApplicationGroups> {
        let mut result__ = ::windows::core::zeroed::<IAzApplicationGroups>();
        (::windows::core::Interface::vtable(self).base__.ApplicationGroups)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateApplicationGroup<P0>(&self, bstrgroupname: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzApplicationGroup>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzApplicationGroup>();
        (::windows::core::Interface::vtable(self).base__.CreateApplicationGroup)(::windows::core::Interface::as_raw(self), bstrgroupname.into_param().abi(), ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn OpenApplicationGroup<P0>(&self, bstrgroupname: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzApplicationGroup>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzApplicationGroup>();
        (::windows::core::Interface::vtable(self).base__.OpenApplicationGroup)(::windows::core::Interface::as_raw(self), bstrgroupname.into_param().abi(), ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteApplicationGroup<P0>(&self, bstrgroupname: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.DeleteApplicationGroup)(::windows::core::Interface::as_raw(self), bstrgroupname.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Submit(&self, lflags: i32, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Submit)(::windows::core::Interface::as_raw(self), lflags, ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DelegatedPolicyUsers(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.DelegatedPolicyUsers)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddDelegatedPolicyUser<P0>(&self, bstrdelegatedpolicyuser: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.AddDelegatedPolicyUser)(::windows::core::Interface::as_raw(self), bstrdelegatedpolicyuser.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteDelegatedPolicyUser<P0>(&self, bstrdelegatedpolicyuser: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.DeleteDelegatedPolicyUser)(::windows::core::Interface::as_raw(self), bstrdelegatedpolicyuser.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    pub unsafe fn TargetMachine(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.TargetMachine)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ApplyStoreSacl(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.ApplyStoreSacl)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetApplyStoreSacl<P0>(&self, bapplystoresacl: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.SetApplyStoreSacl)(::windows::core::Interface::as_raw(self), bapplystoresacl.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PolicyAdministratorsName(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.PolicyAdministratorsName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PolicyReadersName(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.PolicyReadersName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPolicyAdministratorName<P0>(&self, bstradmin: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.AddPolicyAdministratorName)(::windows::core::Interface::as_raw(self), bstradmin.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePolicyAdministratorName<P0>(&self, bstradmin: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.DeletePolicyAdministratorName)(::windows::core::Interface::as_raw(self), bstradmin.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPolicyReaderName<P0>(&self, bstrreader: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.AddPolicyReaderName)(::windows::core::Interface::as_raw(self), bstrreader.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePolicyReaderName<P0>(&self, bstrreader: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.DeletePolicyReaderName)(::windows::core::Interface::as_raw(self), bstrreader.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DelegatedPolicyUsersName(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.DelegatedPolicyUsersName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddDelegatedPolicyUserName<P0>(&self, bstrdelegatedpolicyuser: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.AddDelegatedPolicyUserName)(::windows::core::Interface::as_raw(self), bstrdelegatedpolicyuser.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteDelegatedPolicyUserName<P0>(&self, bstrdelegatedpolicyuser: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.DeleteDelegatedPolicyUserName)(::windows::core::Interface::as_raw(self), bstrdelegatedpolicyuser.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    pub unsafe fn CloseApplication<P0>(&self, bstrapplicationname: P0, lflag: i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.CloseApplication)(::windows::core::Interface::as_raw(self), bstrapplicationname.into_param().abi(), lflag).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn OpenApplication2<P0>(&self, bstrapplicationname: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzApplication2>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzApplication2>();
        (::windows::core::Interface::vtable(self).OpenApplication2)(::windows::core::Interface::as_raw(self), bstrapplicationname.into_param().abi(), ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateApplication2<P0>(&self, bstrapplicationname: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzApplication2>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzApplication2>();
        (::windows::core::Interface::vtable(self).CreateApplication2)(::windows::core::Interface::as_raw(self), bstrapplicationname.into_param().abi(), ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IAzAuthorizationStore2, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IAzAuthorizationStore);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAzAuthorizationStore2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAzAuthorizationStore2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAzAuthorizationStore2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAzAuthorizationStore2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IAzAuthorizationStore2 {
    type Vtable = IAzAuthorizationStore2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAzAuthorizationStore2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IAzAuthorizationStore2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb11e5584_d577_4273_b6c5_0973e0f8e80d);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzAuthorizationStore2_Vtbl {
    pub base__: IAzAuthorizationStore_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub OpenApplication2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrapplicationname: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT, ppapplication: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    OpenApplication2: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateApplication2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrapplicationname: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT, ppapplication: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateApplication2: usize,
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAzAuthorizationStore3(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAzAuthorizationStore3 {
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.base__.Description)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDescription<P0>(&self, bstrdescription: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetDescription)(::windows::core::Interface::as_raw(self), bstrdescription.into_param().abi()).ok()
    }
    pub unsafe fn ApplicationData(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.base__.ApplicationData)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetApplicationData<P0>(&self, bstrapplicationdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetApplicationData)(::windows::core::Interface::as_raw(self), bstrapplicationdata.into_param().abi()).ok()
    }
    pub unsafe fn DomainTimeout(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).base__.base__.DomainTimeout)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDomainTimeout(&self, lprop: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetDomainTimeout)(::windows::core::Interface::as_raw(self), lprop).ok()
    }
    pub unsafe fn ScriptEngineTimeout(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).base__.base__.ScriptEngineTimeout)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetScriptEngineTimeout(&self, lprop: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetScriptEngineTimeout)(::windows::core::Interface::as_raw(self), lprop).ok()
    }
    pub unsafe fn MaxScriptEngines(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).base__.base__.MaxScriptEngines)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetMaxScriptEngines(&self, lprop: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetMaxScriptEngines)(::windows::core::Interface::as_raw(self), lprop).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GenerateAudits(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.base__.GenerateAudits)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetGenerateAudits<P0>(&self, bprop: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetGenerateAudits)(::windows::core::Interface::as_raw(self), bprop.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Writable(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.base__.Writable)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetProperty(&self, lpropid: i32, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.base__.GetProperty)(::windows::core::Interface::as_raw(self), lpropid, ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetProperty(&self, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetProperty)(::windows::core::Interface::as_raw(self), lpropid, ::core::mem::transmute(varprop), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPropertyItem(&self, lpropid: AZ_PROP_CONSTANTS, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.AddPropertyItem)(::windows::core::Interface::as_raw(self), lpropid, ::core::mem::transmute(varprop), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePropertyItem(&self, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.DeletePropertyItem)(::windows::core::Interface::as_raw(self), lpropid, ::core::mem::transmute(varprop), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PolicyAdministrators(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.base__.PolicyAdministrators)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PolicyReaders(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.base__.PolicyReaders)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPolicyAdministrator<P0>(&self, bstradmin: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.AddPolicyAdministrator)(::windows::core::Interface::as_raw(self), bstradmin.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePolicyAdministrator<P0>(&self, bstradmin: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.DeletePolicyAdministrator)(::windows::core::Interface::as_raw(self), bstradmin.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPolicyReader<P0>(&self, bstrreader: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.AddPolicyReader)(::windows::core::Interface::as_raw(self), bstrreader.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePolicyReader<P0>(&self, bstrreader: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.DeletePolicyReader)(::windows::core::Interface::as_raw(self), bstrreader.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Initialize<P0>(&self, lflags: AZ_PROP_CONSTANTS, bstrpolicyurl: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.Initialize)(::windows::core::Interface::as_raw(self), lflags, bstrpolicyurl.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn UpdateCache(&self, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.UpdateCache)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Delete(&self, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.Delete)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Applications(&self) -> ::windows::core::Result<IAzApplications> {
        let mut result__ = ::windows::core::zeroed::<IAzApplications>();
        (::windows::core::Interface::vtable(self).base__.base__.Applications)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn OpenApplication<P0>(&self, bstrapplicationname: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzApplication>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzApplication>();
        (::windows::core::Interface::vtable(self).base__.base__.OpenApplication)(::windows::core::Interface::as_raw(self), bstrapplicationname.into_param().abi(), ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateApplication<P0>(&self, bstrapplicationname: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzApplication>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzApplication>();
        (::windows::core::Interface::vtable(self).base__.base__.CreateApplication)(::windows::core::Interface::as_raw(self), bstrapplicationname.into_param().abi(), ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteApplication<P0>(&self, bstrapplicationname: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.DeleteApplication)(::windows::core::Interface::as_raw(self), bstrapplicationname.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ApplicationGroups(&self) -> ::windows::core::Result<IAzApplicationGroups> {
        let mut result__ = ::windows::core::zeroed::<IAzApplicationGroups>();
        (::windows::core::Interface::vtable(self).base__.base__.ApplicationGroups)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateApplicationGroup<P0>(&self, bstrgroupname: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzApplicationGroup>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzApplicationGroup>();
        (::windows::core::Interface::vtable(self).base__.base__.CreateApplicationGroup)(::windows::core::Interface::as_raw(self), bstrgroupname.into_param().abi(), ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn OpenApplicationGroup<P0>(&self, bstrgroupname: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzApplicationGroup>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzApplicationGroup>();
        (::windows::core::Interface::vtable(self).base__.base__.OpenApplicationGroup)(::windows::core::Interface::as_raw(self), bstrgroupname.into_param().abi(), ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteApplicationGroup<P0>(&self, bstrgroupname: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.DeleteApplicationGroup)(::windows::core::Interface::as_raw(self), bstrgroupname.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Submit(&self, lflags: i32, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.Submit)(::windows::core::Interface::as_raw(self), lflags, ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DelegatedPolicyUsers(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.base__.DelegatedPolicyUsers)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddDelegatedPolicyUser<P0>(&self, bstrdelegatedpolicyuser: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.AddDelegatedPolicyUser)(::windows::core::Interface::as_raw(self), bstrdelegatedpolicyuser.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteDelegatedPolicyUser<P0>(&self, bstrdelegatedpolicyuser: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.DeleteDelegatedPolicyUser)(::windows::core::Interface::as_raw(self), bstrdelegatedpolicyuser.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    pub unsafe fn TargetMachine(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.base__.TargetMachine)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ApplyStoreSacl(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.base__.ApplyStoreSacl)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetApplyStoreSacl<P0>(&self, bapplystoresacl: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetApplyStoreSacl)(::windows::core::Interface::as_raw(self), bapplystoresacl.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PolicyAdministratorsName(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.base__.PolicyAdministratorsName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PolicyReadersName(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.base__.PolicyReadersName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPolicyAdministratorName<P0>(&self, bstradmin: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.AddPolicyAdministratorName)(::windows::core::Interface::as_raw(self), bstradmin.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePolicyAdministratorName<P0>(&self, bstradmin: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.DeletePolicyAdministratorName)(::windows::core::Interface::as_raw(self), bstradmin.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPolicyReaderName<P0>(&self, bstrreader: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.AddPolicyReaderName)(::windows::core::Interface::as_raw(self), bstrreader.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePolicyReaderName<P0>(&self, bstrreader: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.DeletePolicyReaderName)(::windows::core::Interface::as_raw(self), bstrreader.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DelegatedPolicyUsersName(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.base__.DelegatedPolicyUsersName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddDelegatedPolicyUserName<P0>(&self, bstrdelegatedpolicyuser: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.AddDelegatedPolicyUserName)(::windows::core::Interface::as_raw(self), bstrdelegatedpolicyuser.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteDelegatedPolicyUserName<P0>(&self, bstrdelegatedpolicyuser: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.DeleteDelegatedPolicyUserName)(::windows::core::Interface::as_raw(self), bstrdelegatedpolicyuser.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    pub unsafe fn CloseApplication<P0>(&self, bstrapplicationname: P0, lflag: i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.CloseApplication)(::windows::core::Interface::as_raw(self), bstrapplicationname.into_param().abi(), lflag).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn OpenApplication2<P0>(&self, bstrapplicationname: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzApplication2>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzApplication2>();
        (::windows::core::Interface::vtable(self).base__.OpenApplication2)(::windows::core::Interface::as_raw(self), bstrapplicationname.into_param().abi(), ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateApplication2<P0>(&self, bstrapplicationname: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzApplication2>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzApplication2>();
        (::windows::core::Interface::vtable(self).base__.CreateApplication2)(::windows::core::Interface::as_raw(self), bstrapplicationname.into_param().abi(), ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsUpdateNeeded(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).IsUpdateNeeded)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BizruleGroupSupported(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).BizruleGroupSupported)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn UpgradeStoresFunctionalLevel(&self, lfunctionallevel: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UpgradeStoresFunctionalLevel)(::windows::core::Interface::as_raw(self), lfunctionallevel).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsFunctionalLevelUpgradeSupported(&self, lfunctionallevel: i32) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).IsFunctionalLevelUpgradeSupported)(::windows::core::Interface::as_raw(self), lfunctionallevel, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSchemaVersion(&self, plmajorversion: *mut i32, plminorversion: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSchemaVersion)(::windows::core::Interface::as_raw(self), plmajorversion, plminorversion).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IAzAuthorizationStore3, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IAzAuthorizationStore, IAzAuthorizationStore2);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAzAuthorizationStore3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAzAuthorizationStore3 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAzAuthorizationStore3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAzAuthorizationStore3").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IAzAuthorizationStore3 {
    type Vtable = IAzAuthorizationStore3_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAzAuthorizationStore3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IAzAuthorizationStore3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xabc08425_0c86_4fa0_9be3_7189956c926e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzAuthorizationStore3_Vtbl {
    pub base__: IAzAuthorizationStore2_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub IsUpdateNeeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbisupdateneeded: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsUpdateNeeded: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub BizruleGroupSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbsupported: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BizruleGroupSupported: usize,
    pub UpgradeStoresFunctionalLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lfunctionallevel: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsFunctionalLevelUpgradeSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lfunctionallevel: i32, pbsupported: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsFunctionalLevelUpgradeSupported: usize,
    pub GetSchemaVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmajorversion: *mut i32, plminorversion: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAzBizRuleContext(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAzBizRuleContext {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBusinessRuleResult<P0>(&self, bresult: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetBusinessRuleResult)(::windows::core::Interface::as_raw(self), bresult.into_param().abi()).ok()
    }
    pub unsafe fn SetBusinessRuleString<P0>(&self, bstrbusinessrulestring: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetBusinessRuleString)(::windows::core::Interface::as_raw(self), bstrbusinessrulestring.into_param().abi()).ok()
    }
    pub unsafe fn BusinessRuleString(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).BusinessRuleString)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetParameter<P0>(&self, bstrparametername: P0) -> ::windows::core::Result<super::super::System::Com::VARIANT>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).GetParameter)(::windows::core::Interface::as_raw(self), bstrparametername.into_param().abi(), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IAzBizRuleContext, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAzBizRuleContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAzBizRuleContext {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAzBizRuleContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAzBizRuleContext").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IAzBizRuleContext {
    type Vtable = IAzBizRuleContext_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAzBizRuleContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IAzBizRuleContext {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe192f17d_d59f_455e_a152_940316cd77b2);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzBizRuleContext_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SetBusinessRuleResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bresult: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetBusinessRuleResult: usize,
    pub SetBusinessRuleString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrbusinessrulestring: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub BusinessRuleString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrbusinessrulestring: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetParameter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrparametername: ::std::mem::MaybeUninit<::windows::core::BSTR>, pvarparametervalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetParameter: usize,
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAzBizRuleInterfaces(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAzBizRuleInterfaces {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddInterface<P0>(&self, bstrinterfacename: P0, linterfaceflag: i32, varinterface: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).AddInterface)(::windows::core::Interface::as_raw(self), bstrinterfacename.into_param().abi(), linterfaceflag, ::core::mem::transmute(varinterface)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddInterfaces(&self, varinterfacenames: super::super::System::Com::VARIANT, varinterfaceflags: super::super::System::Com::VARIANT, varinterfaces: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddInterfaces)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(varinterfacenames), ::core::mem::transmute(varinterfaceflags), ::core::mem::transmute(varinterfaces)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetInterfaceValue<P0>(&self, bstrinterfacename: P0, linterfaceflag: *mut i32, varinterface: *mut super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).GetInterfaceValue)(::windows::core::Interface::as_raw(self), bstrinterfacename.into_param().abi(), linterfaceflag, varinterface).ok()
    }
    pub unsafe fn Remove<P0>(&self, bstrinterfacename: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).Remove)(::windows::core::Interface::as_raw(self), bstrinterfacename.into_param().abi()).ok()
    }
    pub unsafe fn RemoveAll(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveAll)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Count(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).Count)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IAzBizRuleInterfaces, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAzBizRuleInterfaces {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAzBizRuleInterfaces {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAzBizRuleInterfaces {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAzBizRuleInterfaces").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IAzBizRuleInterfaces {
    type Vtable = IAzBizRuleInterfaces_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAzBizRuleInterfaces {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IAzBizRuleInterfaces {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe94128c7_e9da_44cc_b0bd_53036f3aab3d);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzBizRuleInterfaces_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrinterfacename: ::std::mem::MaybeUninit<::windows::core::BSTR>, linterfaceflag: i32, varinterface: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddInterface: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddInterfaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varinterfacenames: super::super::System::Com::VARIANT, varinterfaceflags: super::super::System::Com::VARIANT, varinterfaces: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddInterfaces: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetInterfaceValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrinterfacename: ::std::mem::MaybeUninit<::windows::core::BSTR>, linterfaceflag: *mut i32, varinterface: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetInterfaceValue: usize,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrinterfacename: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub RemoveAll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAzBizRuleParameters(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAzBizRuleParameters {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddParameter<P0>(&self, bstrparametername: P0, varparametervalue: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).AddParameter)(::windows::core::Interface::as_raw(self), bstrparametername.into_param().abi(), ::core::mem::transmute(varparametervalue)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddParameters(&self, varparameternames: super::super::System::Com::VARIANT, varparametervalues: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddParameters)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(varparameternames), ::core::mem::transmute(varparametervalues)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetParameterValue<P0>(&self, bstrparametername: P0) -> ::windows::core::Result<super::super::System::Com::VARIANT>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).GetParameterValue)(::windows::core::Interface::as_raw(self), bstrparametername.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Remove<P0>(&self, varparametername: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).Remove)(::windows::core::Interface::as_raw(self), varparametername.into_param().abi()).ok()
    }
    pub unsafe fn RemoveAll(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveAll)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Count(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).Count)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IAzBizRuleParameters, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAzBizRuleParameters {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAzBizRuleParameters {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAzBizRuleParameters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAzBizRuleParameters").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IAzBizRuleParameters {
    type Vtable = IAzBizRuleParameters_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAzBizRuleParameters {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IAzBizRuleParameters {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfc17685f_e25d_4dcd_bae1_276ec9533cb5);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzBizRuleParameters_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddParameter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrparametername: ::std::mem::MaybeUninit<::windows::core::BSTR>, varparametervalue: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddParameter: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varparameternames: super::super::System::Com::VARIANT, varparametervalues: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddParameters: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetParameterValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrparametername: ::std::mem::MaybeUninit<::windows::core::BSTR>, pvarparametervalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetParameterValue: usize,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varparametername: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub RemoveAll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAzClientContext(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAzClientContext {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AccessCheck<P0>(&self, bstrobjectname: P0, varscopenames: super::super::System::Com::VARIANT, varoperations: super::super::System::Com::VARIANT, varparameternames: super::super::System::Com::VARIANT, varparametervalues: super::super::System::Com::VARIANT, varinterfacenames: super::super::System::Com::VARIANT, varinterfaceflags: super::super::System::Com::VARIANT, varinterfaces: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).AccessCheck)(::windows::core::Interface::as_raw(self), bstrobjectname.into_param().abi(), ::core::mem::transmute(varscopenames), ::core::mem::transmute(varoperations), ::core::mem::transmute(varparameternames), ::core::mem::transmute(varparametervalues), ::core::mem::transmute(varinterfacenames), ::core::mem::transmute(varinterfaceflags), ::core::mem::transmute(varinterfaces), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetBusinessRuleString(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetBusinessRuleString)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn UserDn(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).UserDn)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn UserSamCompat(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).UserSamCompat)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn UserDisplay(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).UserDisplay)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn UserGuid(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).UserGuid)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn UserCanonical(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).UserCanonical)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn UserUpn(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).UserUpn)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn UserDnsSamCompat(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).UserDnsSamCompat)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetProperty(&self, lpropid: i32, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).GetProperty)(::windows::core::Interface::as_raw(self), lpropid, ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetRoles<P0>(&self, bstrscopename: P0) -> ::windows::core::Result<super::super::System::Com::VARIANT>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).GetRoles)(::windows::core::Interface::as_raw(self), bstrscopename.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn RoleForAccessCheck(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).RoleForAccessCheck)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetRoleForAccessCheck<P0>(&self, bstrprop: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetRoleForAccessCheck)(::windows::core::Interface::as_raw(self), bstrprop.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IAzClientContext, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAzClientContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAzClientContext {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAzClientContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAzClientContext").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IAzClientContext {
    type Vtable = IAzClientContext_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAzClientContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IAzClientContext {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeff1f00b_488a_466d_afd9_a401c5f9eef5);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzClientContext_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AccessCheck: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrobjectname: ::std::mem::MaybeUninit<::windows::core::BSTR>, varscopenames: super::super::System::Com::VARIANT, varoperations: super::super::System::Com::VARIANT, varparameternames: super::super::System::Com::VARIANT, varparametervalues: super::super::System::Com::VARIANT, varinterfacenames: super::super::System::Com::VARIANT, varinterfaceflags: super::super::System::Com::VARIANT, varinterfaces: super::super::System::Com::VARIANT, pvarresults: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AccessCheck: usize,
    pub GetBusinessRuleString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrbusinessrulestring: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub UserDn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrprop: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub UserSamCompat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrprop: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub UserDisplay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrprop: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub UserGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrprop: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub UserCanonical: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrprop: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub UserUpn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrprop: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub UserDnsSamCompat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrprop: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpropid: i32, varreserved: super::super::System::Com::VARIANT, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetRoles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrscopename: ::std::mem::MaybeUninit<::windows::core::BSTR>, pvarrolenames: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetRoles: usize,
    pub RoleForAccessCheck: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrprop: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetRoleForAccessCheck: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAzClientContext2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAzClientContext2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AccessCheck<P0>(&self, bstrobjectname: P0, varscopenames: super::super::System::Com::VARIANT, varoperations: super::super::System::Com::VARIANT, varparameternames: super::super::System::Com::VARIANT, varparametervalues: super::super::System::Com::VARIANT, varinterfacenames: super::super::System::Com::VARIANT, varinterfaceflags: super::super::System::Com::VARIANT, varinterfaces: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.AccessCheck)(::windows::core::Interface::as_raw(self), bstrobjectname.into_param().abi(), ::core::mem::transmute(varscopenames), ::core::mem::transmute(varoperations), ::core::mem::transmute(varparameternames), ::core::mem::transmute(varparametervalues), ::core::mem::transmute(varinterfacenames), ::core::mem::transmute(varinterfaceflags), ::core::mem::transmute(varinterfaces), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetBusinessRuleString(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.GetBusinessRuleString)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn UserDn(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.UserDn)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn UserSamCompat(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.UserSamCompat)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn UserDisplay(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.UserDisplay)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn UserGuid(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.UserGuid)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn UserCanonical(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.UserCanonical)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn UserUpn(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.UserUpn)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn UserDnsSamCompat(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.UserDnsSamCompat)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetProperty(&self, lpropid: i32, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.GetProperty)(::windows::core::Interface::as_raw(self), lpropid, ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetRoles<P0>(&self, bstrscopename: P0) -> ::windows::core::Result<super::super::System::Com::VARIANT>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.GetRoles)(::windows::core::Interface::as_raw(self), bstrscopename.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn RoleForAccessCheck(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.RoleForAccessCheck)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetRoleForAccessCheck<P0>(&self, bstrprop: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetRoleForAccessCheck)(::windows::core::Interface::as_raw(self), bstrprop.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetAssignedScopesPage(&self, loptions: i32, pagesize: i32, pvarcursor: *mut super::super::System::Com::VARIANT, pvarscopenames: *mut super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAssignedScopesPage)(::windows::core::Interface::as_raw(self), loptions, pagesize, pvarcursor, pvarscopenames).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddRoles<P0>(&self, varroles: super::super::System::Com::VARIANT, bstrscopename: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).AddRoles)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(varroles), bstrscopename.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddApplicationGroups(&self, varapplicationgroups: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddApplicationGroups)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(varapplicationgroups)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddStringSids(&self, varstringsids: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddStringSids)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(varstringsids)).ok()
    }
    pub unsafe fn SetLDAPQueryDN<P0>(&self, bstrldapquerydn: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetLDAPQueryDN)(::windows::core::Interface::as_raw(self), bstrldapquerydn.into_param().abi()).ok()
    }
    pub unsafe fn LDAPQueryDN(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).LDAPQueryDN)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IAzClientContext2, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IAzClientContext);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAzClientContext2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAzClientContext2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAzClientContext2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAzClientContext2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IAzClientContext2 {
    type Vtable = IAzClientContext2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAzClientContext2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IAzClientContext2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2b0c92b8_208a_488a_8f81_e4edb22111cd);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzClientContext2_Vtbl {
    pub base__: IAzClientContext_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetAssignedScopesPage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, loptions: i32, pagesize: i32, pvarcursor: *mut super::super::System::Com::VARIANT, pvarscopenames: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetAssignedScopesPage: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddRoles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varroles: super::super::System::Com::VARIANT, bstrscopename: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddRoles: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddApplicationGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varapplicationgroups: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddApplicationGroups: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddStringSids: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varstringsids: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddStringSids: usize,
    pub SetLDAPQueryDN: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrldapquerydn: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub LDAPQueryDN: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrldapquerydn: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAzClientContext3(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAzClientContext3 {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AccessCheck<P0>(&self, bstrobjectname: P0, varscopenames: super::super::System::Com::VARIANT, varoperations: super::super::System::Com::VARIANT, varparameternames: super::super::System::Com::VARIANT, varparametervalues: super::super::System::Com::VARIANT, varinterfacenames: super::super::System::Com::VARIANT, varinterfaceflags: super::super::System::Com::VARIANT, varinterfaces: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.base__.AccessCheck)(::windows::core::Interface::as_raw(self), bstrobjectname.into_param().abi(), ::core::mem::transmute(varscopenames), ::core::mem::transmute(varoperations), ::core::mem::transmute(varparameternames), ::core::mem::transmute(varparametervalues), ::core::mem::transmute(varinterfacenames), ::core::mem::transmute(varinterfaceflags), ::core::mem::transmute(varinterfaces), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetBusinessRuleString(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.base__.GetBusinessRuleString)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn UserDn(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.base__.UserDn)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn UserSamCompat(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.base__.UserSamCompat)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn UserDisplay(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.base__.UserDisplay)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn UserGuid(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.base__.UserGuid)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn UserCanonical(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.base__.UserCanonical)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn UserUpn(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.base__.UserUpn)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn UserDnsSamCompat(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.base__.UserDnsSamCompat)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetProperty(&self, lpropid: i32, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.base__.GetProperty)(::windows::core::Interface::as_raw(self), lpropid, ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetRoles<P0>(&self, bstrscopename: P0) -> ::windows::core::Result<super::super::System::Com::VARIANT>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.base__.GetRoles)(::windows::core::Interface::as_raw(self), bstrscopename.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn RoleForAccessCheck(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.base__.RoleForAccessCheck)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetRoleForAccessCheck<P0>(&self, bstrprop: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetRoleForAccessCheck)(::windows::core::Interface::as_raw(self), bstrprop.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetAssignedScopesPage(&self, loptions: i32, pagesize: i32, pvarcursor: *mut super::super::System::Com::VARIANT, pvarscopenames: *mut super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetAssignedScopesPage)(::windows::core::Interface::as_raw(self), loptions, pagesize, pvarcursor, pvarscopenames).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddRoles<P0>(&self, varroles: super::super::System::Com::VARIANT, bstrscopename: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.AddRoles)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(varroles), bstrscopename.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddApplicationGroups(&self, varapplicationgroups: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.AddApplicationGroups)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(varapplicationgroups)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddStringSids(&self, varstringsids: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.AddStringSids)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(varstringsids)).ok()
    }
    pub unsafe fn SetLDAPQueryDN<P0>(&self, bstrldapquerydn: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetLDAPQueryDN)(::windows::core::Interface::as_raw(self), bstrldapquerydn.into_param().abi()).ok()
    }
    pub unsafe fn LDAPQueryDN(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.LDAPQueryDN)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn AccessCheck2<P0, P1>(&self, bstrobjectname: P0, bstrscopename: P1, loperation: i32) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).AccessCheck2)(::windows::core::Interface::as_raw(self), bstrobjectname.into_param().abi(), bstrscopename.into_param().abi(), loperation, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsInRoleAssignment<P0, P1>(&self, bstrscopename: P0, bstrrolename: P1) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).IsInRoleAssignment)(::windows::core::Interface::as_raw(self), bstrscopename.into_param().abi(), bstrrolename.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetOperations<P0>(&self, bstrscopename: P0) -> ::windows::core::Result<IAzOperations>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzOperations>();
        (::windows::core::Interface::vtable(self).GetOperations)(::windows::core::Interface::as_raw(self), bstrscopename.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTasks<P0>(&self, bstrscopename: P0) -> ::windows::core::Result<IAzTasks>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzTasks>();
        (::windows::core::Interface::vtable(self).GetTasks)(::windows::core::Interface::as_raw(self), bstrscopename.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BizRuleParameters(&self) -> ::windows::core::Result<IAzBizRuleParameters> {
        let mut result__ = ::windows::core::zeroed::<IAzBizRuleParameters>();
        (::windows::core::Interface::vtable(self).BizRuleParameters)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BizRuleInterfaces(&self) -> ::windows::core::Result<IAzBizRuleInterfaces> {
        let mut result__ = ::windows::core::zeroed::<IAzBizRuleInterfaces>();
        (::windows::core::Interface::vtable(self).BizRuleInterfaces)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetGroups<P0>(&self, bstrscopename: P0, uloptions: AZ_PROP_CONSTANTS) -> ::windows::core::Result<super::super::System::Com::VARIANT>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).GetGroups)(::windows::core::Interface::as_raw(self), bstrscopename.into_param().abi(), uloptions, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Sids(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).Sids)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IAzClientContext3, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IAzClientContext, IAzClientContext2);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAzClientContext3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAzClientContext3 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAzClientContext3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAzClientContext3").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IAzClientContext3 {
    type Vtable = IAzClientContext3_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAzClientContext3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IAzClientContext3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x11894fde_1deb_4b4b_8907_6d1cda1f5d4f);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzClientContext3_Vtbl {
    pub base__: IAzClientContext2_Vtbl,
    pub AccessCheck2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrobjectname: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrscopename: ::std::mem::MaybeUninit<::windows::core::BSTR>, loperation: i32, plresult: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsInRoleAssignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrscopename: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrrolename: ::std::mem::MaybeUninit<::windows::core::BSTR>, pbisinrole: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsInRoleAssignment: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrscopename: ::std::mem::MaybeUninit<::windows::core::BSTR>, ppoperationcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetOperations: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetTasks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrscopename: ::std::mem::MaybeUninit<::windows::core::BSTR>, pptaskcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetTasks: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub BizRuleParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppbizruleparam: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    BizRuleParameters: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub BizRuleInterfaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppbizruleinterfaces: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    BizRuleInterfaces: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrscopename: ::std::mem::MaybeUninit<::windows::core::BSTR>, uloptions: AZ_PROP_CONSTANTS, pgrouparray: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetGroups: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Sids: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstringsidarray: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Sids: usize,
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAzNameResolver(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAzNameResolver {
    pub unsafe fn NameFromSid<P0>(&self, bstrsid: P0, psidtype: *mut i32, pbstrname: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).NameFromSid)(::windows::core::Interface::as_raw(self), bstrsid.into_param().abi(), psidtype, ::core::mem::transmute(pbstrname)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn NamesFromSids(&self, vsids: super::super::System::Com::VARIANT, pvsidtypes: *mut super::super::System::Com::VARIANT, pvnames: *mut super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).NamesFromSids)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(vsids), pvsidtypes, pvnames).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IAzNameResolver, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAzNameResolver {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAzNameResolver {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAzNameResolver {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAzNameResolver").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IAzNameResolver {
    type Vtable = IAzNameResolver_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAzNameResolver {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IAzNameResolver {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x504d0f15_73e2_43df_a870_a64f40714f53);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzNameResolver_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub NameFromSid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsid: ::std::mem::MaybeUninit<::windows::core::BSTR>, psidtype: *mut i32, pbstrname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub NamesFromSids: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vsids: super::super::System::Com::VARIANT, pvsidtypes: *mut super::super::System::Com::VARIANT, pvnames: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    NamesFromSids: usize,
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAzObjectPicker(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAzObjectPicker {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetPrincipals<P0, P1>(&self, hparentwnd: P0, bstrtitle: P1, pvsidtypes: *mut super::super::System::Com::VARIANT, pvnames: *mut super::super::System::Com::VARIANT, pvsids: *mut super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).GetPrincipals)(::windows::core::Interface::as_raw(self), hparentwnd.into_param().abi(), bstrtitle.into_param().abi(), pvsidtypes, pvnames, pvsids).ok()
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).Name)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IAzObjectPicker, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAzObjectPicker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAzObjectPicker {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAzObjectPicker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAzObjectPicker").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IAzObjectPicker {
    type Vtable = IAzObjectPicker_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAzObjectPicker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IAzObjectPicker {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x63130a48_699a_42d8_bf01_c62ac3fb79f9);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzObjectPicker_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetPrincipals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hparentwnd: super::super::Foundation::HWND, bstrtitle: ::std::mem::MaybeUninit<::windows::core::BSTR>, pvsidtypes: *mut super::super::System::Com::VARIANT, pvnames: *mut super::super::System::Com::VARIANT, pvsids: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetPrincipals: usize,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAzOperation(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAzOperation {
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).Name)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetName<P0>(&self, bstrname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetName)(::windows::core::Interface::as_raw(self), bstrname.into_param().abi()).ok()
    }
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).Description)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDescription<P0>(&self, bstrdescription: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetDescription)(::windows::core::Interface::as_raw(self), bstrdescription.into_param().abi()).ok()
    }
    pub unsafe fn ApplicationData(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).ApplicationData)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetApplicationData<P0>(&self, bstrapplicationdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetApplicationData)(::windows::core::Interface::as_raw(self), bstrapplicationdata.into_param().abi()).ok()
    }
    pub unsafe fn OperationID(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).OperationID)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetOperationID(&self, lprop: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOperationID)(::windows::core::Interface::as_raw(self), lprop).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Writable(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).Writable)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetProperty(&self, lpropid: i32, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).GetProperty)(::windows::core::Interface::as_raw(self), lpropid, ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetProperty(&self, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetProperty)(::windows::core::Interface::as_raw(self), lpropid, ::core::mem::transmute(varprop), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Submit(&self, lflags: i32, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Submit)(::windows::core::Interface::as_raw(self), lflags, ::core::mem::transmute(varreserved)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IAzOperation, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAzOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAzOperation {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAzOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAzOperation").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IAzOperation {
    type Vtable = IAzOperation_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAzOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IAzOperation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5e56b24f_ea01_4d61_be44_c49b5e4eaf74);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzOperation_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdescription: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdescription: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub ApplicationData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrapplicationdata: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetApplicationData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrapplicationdata: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub OperationID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plprop: *mut i32) -> ::windows::core::HRESULT,
    pub SetOperationID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lprop: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Writable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfprop: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Writable: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpropid: i32, varreserved: super::super::System::Com::VARIANT, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Submit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Submit: usize,
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAzOperation2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAzOperation2 {
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.Name)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetName<P0>(&self, bstrname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetName)(::windows::core::Interface::as_raw(self), bstrname.into_param().abi()).ok()
    }
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.Description)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDescription<P0>(&self, bstrdescription: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetDescription)(::windows::core::Interface::as_raw(self), bstrdescription.into_param().abi()).ok()
    }
    pub unsafe fn ApplicationData(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.ApplicationData)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetApplicationData<P0>(&self, bstrapplicationdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetApplicationData)(::windows::core::Interface::as_raw(self), bstrapplicationdata.into_param().abi()).ok()
    }
    pub unsafe fn OperationID(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).base__.OperationID)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetOperationID(&self, lprop: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetOperationID)(::windows::core::Interface::as_raw(self), lprop).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Writable(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.Writable)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetProperty(&self, lpropid: i32, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.GetProperty)(::windows::core::Interface::as_raw(self), lpropid, ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetProperty(&self, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetProperty)(::windows::core::Interface::as_raw(self), lpropid, ::core::mem::transmute(varprop), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Submit(&self, lflags: i32, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Submit)(::windows::core::Interface::as_raw(self), lflags, ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn RoleAssignments<P0, P1>(&self, bstrscopename: P0, brecursive: P1) -> ::windows::core::Result<IAzRoleAssignments>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzRoleAssignments>();
        (::windows::core::Interface::vtable(self).RoleAssignments)(::windows::core::Interface::as_raw(self), bstrscopename.into_param().abi(), brecursive.into_param().abi(), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IAzOperation2, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IAzOperation);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAzOperation2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAzOperation2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAzOperation2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAzOperation2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IAzOperation2 {
    type Vtable = IAzOperation2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAzOperation2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IAzOperation2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1f5ea01f_44a2_4184_9c48_a75b4dcc8ccc);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzOperation2_Vtbl {
    pub base__: IAzOperation_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub RoleAssignments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrscopename: ::std::mem::MaybeUninit<::windows::core::BSTR>, brecursive: super::super::Foundation::VARIANT_BOOL, pproleassignments: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    RoleAssignments: usize,
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAzOperations(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAzOperations {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_Item(&self, index: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).get_Item)(::windows::core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).Count)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self)._NewEnum)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IAzOperations, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAzOperations {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAzOperations {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAzOperations {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAzOperations").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IAzOperations {
    type Vtable = IAzOperations_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAzOperations {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IAzOperations {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x90ef9c07_9706_49d9_af80_0438a5f3ec35);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzOperations_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, pvarobtptr: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumptr: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAzPrincipalLocator(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAzPrincipalLocator {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn NameResolver(&self) -> ::windows::core::Result<IAzNameResolver> {
        let mut result__ = ::windows::core::zeroed::<IAzNameResolver>();
        (::windows::core::Interface::vtable(self).NameResolver)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ObjectPicker(&self) -> ::windows::core::Result<IAzObjectPicker> {
        let mut result__ = ::windows::core::zeroed::<IAzObjectPicker>();
        (::windows::core::Interface::vtable(self).ObjectPicker)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IAzPrincipalLocator, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAzPrincipalLocator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAzPrincipalLocator {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAzPrincipalLocator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAzPrincipalLocator").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IAzPrincipalLocator {
    type Vtable = IAzPrincipalLocator_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAzPrincipalLocator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IAzPrincipalLocator {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe5c3507d_ad6a_4992_9c7f_74ab480b44cc);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzPrincipalLocator_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub NameResolver: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppnameresolver: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    NameResolver: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ObjectPicker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppobjectpicker: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ObjectPicker: usize,
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAzRole(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAzRole {
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).Name)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetName<P0>(&self, bstrname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetName)(::windows::core::Interface::as_raw(self), bstrname.into_param().abi()).ok()
    }
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).Description)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDescription<P0>(&self, bstrdescription: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetDescription)(::windows::core::Interface::as_raw(self), bstrdescription.into_param().abi()).ok()
    }
    pub unsafe fn ApplicationData(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).ApplicationData)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetApplicationData<P0>(&self, bstrapplicationdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetApplicationData)(::windows::core::Interface::as_raw(self), bstrapplicationdata.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddAppMember<P0>(&self, bstrprop: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).AddAppMember)(::windows::core::Interface::as_raw(self), bstrprop.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteAppMember<P0>(&self, bstrprop: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).DeleteAppMember)(::windows::core::Interface::as_raw(self), bstrprop.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddTask<P0>(&self, bstrprop: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).AddTask)(::windows::core::Interface::as_raw(self), bstrprop.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteTask<P0>(&self, bstrprop: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).DeleteTask)(::windows::core::Interface::as_raw(self), bstrprop.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddOperation<P0>(&self, bstrprop: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).AddOperation)(::windows::core::Interface::as_raw(self), bstrprop.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteOperation<P0>(&self, bstrprop: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).DeleteOperation)(::windows::core::Interface::as_raw(self), bstrprop.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddMember<P0>(&self, bstrprop: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).AddMember)(::windows::core::Interface::as_raw(self), bstrprop.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteMember<P0>(&self, bstrprop: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).DeleteMember)(::windows::core::Interface::as_raw(self), bstrprop.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Writable(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).Writable)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetProperty(&self, lpropid: i32, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).GetProperty)(::windows::core::Interface::as_raw(self), lpropid, ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetProperty(&self, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetProperty)(::windows::core::Interface::as_raw(self), lpropid, ::core::mem::transmute(varprop), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AppMembers(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).AppMembers)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Members(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).Members)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Operations(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).Operations)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Tasks(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).Tasks)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPropertyItem(&self, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddPropertyItem)(::windows::core::Interface::as_raw(self), lpropid, ::core::mem::transmute(varprop), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePropertyItem(&self, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeletePropertyItem)(::windows::core::Interface::as_raw(self), lpropid, ::core::mem::transmute(varprop), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Submit(&self, lflags: i32, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Submit)(::windows::core::Interface::as_raw(self), lflags, ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddMemberName<P0>(&self, bstrprop: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).AddMemberName)(::windows::core::Interface::as_raw(self), bstrprop.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteMemberName<P0>(&self, bstrprop: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).DeleteMemberName)(::windows::core::Interface::as_raw(self), bstrprop.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn MembersName(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).MembersName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IAzRole, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAzRole {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAzRole {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAzRole {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAzRole").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IAzRole {
    type Vtable = IAzRole_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAzRole {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IAzRole {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x859e0d8d_62d7_41d8_a034_c0cd5d43fdfa);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzRole_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdescription: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdescription: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub ApplicationData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrapplicationdata: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetApplicationData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrapplicationdata: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddAppMember: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddAppMember: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteAppMember: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteAppMember: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddTask: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteTask: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddOperation: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteOperation: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddMember: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddMember: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteMember: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteMember: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Writable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfprop: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Writable: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpropid: i32, varreserved: super::super::System::Com::VARIANT, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AppMembers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AppMembers: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Members: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Members: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Operations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Operations: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Tasks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Tasks: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddPropertyItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddPropertyItem: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeletePropertyItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeletePropertyItem: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Submit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Submit: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddMemberName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddMemberName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteMemberName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteMemberName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub MembersName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    MembersName: usize,
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAzRoleAssignment(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAzRoleAssignment {
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.Name)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetName<P0>(&self, bstrname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetName)(::windows::core::Interface::as_raw(self), bstrname.into_param().abi()).ok()
    }
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.Description)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDescription<P0>(&self, bstrdescription: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetDescription)(::windows::core::Interface::as_raw(self), bstrdescription.into_param().abi()).ok()
    }
    pub unsafe fn ApplicationData(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.ApplicationData)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetApplicationData<P0>(&self, bstrapplicationdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetApplicationData)(::windows::core::Interface::as_raw(self), bstrapplicationdata.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddAppMember<P0>(&self, bstrprop: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.AddAppMember)(::windows::core::Interface::as_raw(self), bstrprop.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteAppMember<P0>(&self, bstrprop: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.DeleteAppMember)(::windows::core::Interface::as_raw(self), bstrprop.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddTask<P0>(&self, bstrprop: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.AddTask)(::windows::core::Interface::as_raw(self), bstrprop.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteTask<P0>(&self, bstrprop: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.DeleteTask)(::windows::core::Interface::as_raw(self), bstrprop.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddOperation<P0>(&self, bstrprop: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.AddOperation)(::windows::core::Interface::as_raw(self), bstrprop.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteOperation<P0>(&self, bstrprop: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.DeleteOperation)(::windows::core::Interface::as_raw(self), bstrprop.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddMember<P0>(&self, bstrprop: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.AddMember)(::windows::core::Interface::as_raw(self), bstrprop.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteMember<P0>(&self, bstrprop: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.DeleteMember)(::windows::core::Interface::as_raw(self), bstrprop.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Writable(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.Writable)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetProperty(&self, lpropid: i32, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.GetProperty)(::windows::core::Interface::as_raw(self), lpropid, ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetProperty(&self, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetProperty)(::windows::core::Interface::as_raw(self), lpropid, ::core::mem::transmute(varprop), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AppMembers(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.AppMembers)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Members(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.Members)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Operations(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.Operations)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Tasks(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.Tasks)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPropertyItem(&self, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.AddPropertyItem)(::windows::core::Interface::as_raw(self), lpropid, ::core::mem::transmute(varprop), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePropertyItem(&self, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.DeletePropertyItem)(::windows::core::Interface::as_raw(self), lpropid, ::core::mem::transmute(varprop), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Submit(&self, lflags: i32, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Submit)(::windows::core::Interface::as_raw(self), lflags, ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddMemberName<P0>(&self, bstrprop: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.AddMemberName)(::windows::core::Interface::as_raw(self), bstrprop.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteMemberName<P0>(&self, bstrprop: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.DeleteMemberName)(::windows::core::Interface::as_raw(self), bstrprop.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn MembersName(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.MembersName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn AddRoleDefinition<P0>(&self, bstrroledefinition: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).AddRoleDefinition)(::windows::core::Interface::as_raw(self), bstrroledefinition.into_param().abi()).ok()
    }
    pub unsafe fn DeleteRoleDefinition<P0>(&self, bstrroledefinition: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).DeleteRoleDefinition)(::windows::core::Interface::as_raw(self), bstrroledefinition.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RoleDefinitions(&self) -> ::windows::core::Result<IAzRoleDefinitions> {
        let mut result__ = ::windows::core::zeroed::<IAzRoleDefinitions>();
        (::windows::core::Interface::vtable(self).RoleDefinitions)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Scope(&self) -> ::windows::core::Result<IAzScope> {
        let mut result__ = ::windows::core::zeroed::<IAzScope>();
        (::windows::core::Interface::vtable(self).Scope)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IAzRoleAssignment, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IAzRole);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAzRoleAssignment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAzRoleAssignment {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAzRoleAssignment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAzRoleAssignment").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IAzRoleAssignment {
    type Vtable = IAzRoleAssignment_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAzRoleAssignment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IAzRoleAssignment {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x55647d31_0d5a_4fa3_b4ac_2b5f9ad5ab76);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzRoleAssignment_Vtbl {
    pub base__: IAzRole_Vtbl,
    pub AddRoleDefinition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrroledefinition: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub DeleteRoleDefinition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrroledefinition: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub RoleDefinitions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pproledefinitions: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RoleDefinitions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Scope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppscope: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Scope: usize,
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAzRoleAssignments(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAzRoleAssignments {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_Item(&self, index: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).get_Item)(::windows::core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).Count)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self)._NewEnum)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IAzRoleAssignments, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAzRoleAssignments {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAzRoleAssignments {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAzRoleAssignments {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAzRoleAssignments").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IAzRoleAssignments {
    type Vtable = IAzRoleAssignments_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAzRoleAssignments {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IAzRoleAssignments {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9c80b900_fceb_4d73_a0f4_c83b0bbf2481);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzRoleAssignments_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, pvarobtptr: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumptr: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAzRoleDefinition(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAzRoleDefinition {
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.Name)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetName<P0>(&self, bstrname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetName)(::windows::core::Interface::as_raw(self), bstrname.into_param().abi()).ok()
    }
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.Description)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDescription<P0>(&self, bstrdescription: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetDescription)(::windows::core::Interface::as_raw(self), bstrdescription.into_param().abi()).ok()
    }
    pub unsafe fn ApplicationData(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.ApplicationData)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetApplicationData<P0>(&self, bstrapplicationdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetApplicationData)(::windows::core::Interface::as_raw(self), bstrapplicationdata.into_param().abi()).ok()
    }
    pub unsafe fn BizRule(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.BizRule)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetBizRule<P0>(&self, bstrprop: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetBizRule)(::windows::core::Interface::as_raw(self), bstrprop.into_param().abi()).ok()
    }
    pub unsafe fn BizRuleLanguage(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.BizRuleLanguage)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetBizRuleLanguage<P0>(&self, bstrprop: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetBizRuleLanguage)(::windows::core::Interface::as_raw(self), bstrprop.into_param().abi()).ok()
    }
    pub unsafe fn BizRuleImportedPath(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.BizRuleImportedPath)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetBizRuleImportedPath<P0>(&self, bstrprop: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetBizRuleImportedPath)(::windows::core::Interface::as_raw(self), bstrprop.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRoleDefinition(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.IsRoleDefinition)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIsRoleDefinition<P0>(&self, fprop: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.SetIsRoleDefinition)(::windows::core::Interface::as_raw(self), fprop.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Operations(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.Operations)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Tasks(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.Tasks)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddOperation<P0>(&self, bstrop: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.AddOperation)(::windows::core::Interface::as_raw(self), bstrop.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteOperation<P0>(&self, bstrop: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.DeleteOperation)(::windows::core::Interface::as_raw(self), bstrop.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddTask<P0>(&self, bstrtask: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.AddTask)(::windows::core::Interface::as_raw(self), bstrtask.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteTask<P0>(&self, bstrtask: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.DeleteTask)(::windows::core::Interface::as_raw(self), bstrtask.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Writable(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.Writable)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetProperty(&self, lpropid: i32, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.GetProperty)(::windows::core::Interface::as_raw(self), lpropid, ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetProperty(&self, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetProperty)(::windows::core::Interface::as_raw(self), lpropid, ::core::mem::transmute(varprop), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPropertyItem(&self, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.AddPropertyItem)(::windows::core::Interface::as_raw(self), lpropid, ::core::mem::transmute(varprop), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePropertyItem(&self, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.DeletePropertyItem)(::windows::core::Interface::as_raw(self), lpropid, ::core::mem::transmute(varprop), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Submit(&self, lflags: i32, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Submit)(::windows::core::Interface::as_raw(self), lflags, ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn RoleAssignments<P0, P1>(&self, bstrscopename: P0, brecursive: P1) -> ::windows::core::Result<IAzRoleAssignments>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzRoleAssignments>();
        (::windows::core::Interface::vtable(self).RoleAssignments)(::windows::core::Interface::as_raw(self), bstrscopename.into_param().abi(), brecursive.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn AddRoleDefinition<P0>(&self, bstrroledefinition: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).AddRoleDefinition)(::windows::core::Interface::as_raw(self), bstrroledefinition.into_param().abi()).ok()
    }
    pub unsafe fn DeleteRoleDefinition<P0>(&self, bstrroledefinition: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).DeleteRoleDefinition)(::windows::core::Interface::as_raw(self), bstrroledefinition.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RoleDefinitions(&self) -> ::windows::core::Result<IAzRoleDefinitions> {
        let mut result__ = ::windows::core::zeroed::<IAzRoleDefinitions>();
        (::windows::core::Interface::vtable(self).RoleDefinitions)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IAzRoleDefinition, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IAzTask);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAzRoleDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAzRoleDefinition {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAzRoleDefinition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAzRoleDefinition").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IAzRoleDefinition {
    type Vtable = IAzRoleDefinition_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAzRoleDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IAzRoleDefinition {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd97fcea1_2599_44f1_9fc3_58e9fbe09466);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzRoleDefinition_Vtbl {
    pub base__: IAzTask_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub RoleAssignments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrscopename: ::std::mem::MaybeUninit<::windows::core::BSTR>, brecursive: super::super::Foundation::VARIANT_BOOL, pproleassignments: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    RoleAssignments: usize,
    pub AddRoleDefinition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrroledefinition: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub DeleteRoleDefinition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrroledefinition: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub RoleDefinitions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pproledefinitions: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RoleDefinitions: usize,
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAzRoleDefinitions(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAzRoleDefinitions {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_Item(&self, index: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).get_Item)(::windows::core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).Count)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self)._NewEnum)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IAzRoleDefinitions, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAzRoleDefinitions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAzRoleDefinitions {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAzRoleDefinitions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAzRoleDefinitions").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IAzRoleDefinitions {
    type Vtable = IAzRoleDefinitions_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAzRoleDefinitions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IAzRoleDefinitions {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x881f25a5_d755_4550_957a_d503a3b34001);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzRoleDefinitions_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, pvarobtptr: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumptr: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAzRoles(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAzRoles {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_Item(&self, index: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).get_Item)(::windows::core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).Count)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self)._NewEnum)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IAzRoles, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAzRoles {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAzRoles {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAzRoles {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAzRoles").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IAzRoles {
    type Vtable = IAzRoles_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAzRoles {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IAzRoles {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x95e0f119_13b4_4dae_b65f_2f7d60d822e4);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzRoles_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, pvarobtptr: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumptr: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAzScope(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAzScope {
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).Name)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetName<P0>(&self, bstrname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetName)(::windows::core::Interface::as_raw(self), bstrname.into_param().abi()).ok()
    }
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).Description)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDescription<P0>(&self, bstrdescription: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetDescription)(::windows::core::Interface::as_raw(self), bstrdescription.into_param().abi()).ok()
    }
    pub unsafe fn ApplicationData(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).ApplicationData)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetApplicationData<P0>(&self, bstrapplicationdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetApplicationData)(::windows::core::Interface::as_raw(self), bstrapplicationdata.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Writable(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).Writable)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetProperty(&self, lpropid: i32, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).GetProperty)(::windows::core::Interface::as_raw(self), lpropid, ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetProperty(&self, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetProperty)(::windows::core::Interface::as_raw(self), lpropid, ::core::mem::transmute(varprop), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPropertyItem(&self, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddPropertyItem)(::windows::core::Interface::as_raw(self), lpropid, ::core::mem::transmute(varprop), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePropertyItem(&self, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeletePropertyItem)(::windows::core::Interface::as_raw(self), lpropid, ::core::mem::transmute(varprop), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PolicyAdministrators(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).PolicyAdministrators)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PolicyReaders(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).PolicyReaders)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPolicyAdministrator<P0>(&self, bstradmin: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).AddPolicyAdministrator)(::windows::core::Interface::as_raw(self), bstradmin.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePolicyAdministrator<P0>(&self, bstradmin: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).DeletePolicyAdministrator)(::windows::core::Interface::as_raw(self), bstradmin.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPolicyReader<P0>(&self, bstrreader: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).AddPolicyReader)(::windows::core::Interface::as_raw(self), bstrreader.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePolicyReader<P0>(&self, bstrreader: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).DeletePolicyReader)(::windows::core::Interface::as_raw(self), bstrreader.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ApplicationGroups(&self) -> ::windows::core::Result<IAzApplicationGroups> {
        let mut result__ = ::windows::core::zeroed::<IAzApplicationGroups>();
        (::windows::core::Interface::vtable(self).ApplicationGroups)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn OpenApplicationGroup<P0>(&self, bstrgroupname: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzApplicationGroup>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzApplicationGroup>();
        (::windows::core::Interface::vtable(self).OpenApplicationGroup)(::windows::core::Interface::as_raw(self), bstrgroupname.into_param().abi(), ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateApplicationGroup<P0>(&self, bstrgroupname: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzApplicationGroup>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzApplicationGroup>();
        (::windows::core::Interface::vtable(self).CreateApplicationGroup)(::windows::core::Interface::as_raw(self), bstrgroupname.into_param().abi(), ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteApplicationGroup<P0>(&self, bstrgroupname: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).DeleteApplicationGroup)(::windows::core::Interface::as_raw(self), bstrgroupname.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Roles(&self) -> ::windows::core::Result<IAzRoles> {
        let mut result__ = ::windows::core::zeroed::<IAzRoles>();
        (::windows::core::Interface::vtable(self).Roles)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn OpenRole<P0>(&self, bstrrolename: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzRole>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzRole>();
        (::windows::core::Interface::vtable(self).OpenRole)(::windows::core::Interface::as_raw(self), bstrrolename.into_param().abi(), ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateRole<P0>(&self, bstrrolename: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzRole>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzRole>();
        (::windows::core::Interface::vtable(self).CreateRole)(::windows::core::Interface::as_raw(self), bstrrolename.into_param().abi(), ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteRole<P0>(&self, bstrrolename: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).DeleteRole)(::windows::core::Interface::as_raw(self), bstrrolename.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Tasks(&self) -> ::windows::core::Result<IAzTasks> {
        let mut result__ = ::windows::core::zeroed::<IAzTasks>();
        (::windows::core::Interface::vtable(self).Tasks)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn OpenTask<P0>(&self, bstrtaskname: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzTask>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzTask>();
        (::windows::core::Interface::vtable(self).OpenTask)(::windows::core::Interface::as_raw(self), bstrtaskname.into_param().abi(), ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateTask<P0>(&self, bstrtaskname: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzTask>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzTask>();
        (::windows::core::Interface::vtable(self).CreateTask)(::windows::core::Interface::as_raw(self), bstrtaskname.into_param().abi(), ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteTask<P0>(&self, bstrtaskname: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).DeleteTask)(::windows::core::Interface::as_raw(self), bstrtaskname.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Submit(&self, lflags: i32, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Submit)(::windows::core::Interface::as_raw(self), lflags, ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CanBeDelegated(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).CanBeDelegated)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BizrulesWritable(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).BizrulesWritable)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PolicyAdministratorsName(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).PolicyAdministratorsName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PolicyReadersName(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).PolicyReadersName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPolicyAdministratorName<P0>(&self, bstradmin: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).AddPolicyAdministratorName)(::windows::core::Interface::as_raw(self), bstradmin.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePolicyAdministratorName<P0>(&self, bstradmin: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).DeletePolicyAdministratorName)(::windows::core::Interface::as_raw(self), bstradmin.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPolicyReaderName<P0>(&self, bstrreader: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).AddPolicyReaderName)(::windows::core::Interface::as_raw(self), bstrreader.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePolicyReaderName<P0>(&self, bstrreader: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).DeletePolicyReaderName)(::windows::core::Interface::as_raw(self), bstrreader.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IAzScope, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAzScope {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAzScope {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAzScope {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAzScope").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IAzScope {
    type Vtable = IAzScope_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAzScope {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IAzScope {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00e52487_e08d_4514_b62e_877d5645f5ab);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzScope_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdescription: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdescription: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub ApplicationData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrapplicationdata: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetApplicationData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrapplicationdata: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Writable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfprop: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Writable: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpropid: i32, varreserved: super::super::System::Com::VARIANT, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddPropertyItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddPropertyItem: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeletePropertyItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeletePropertyItem: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PolicyAdministrators: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvaradmins: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PolicyAdministrators: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PolicyReaders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarreaders: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PolicyReaders: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddPolicyAdministrator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstradmin: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddPolicyAdministrator: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeletePolicyAdministrator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstradmin: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeletePolicyAdministrator: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddPolicyReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrreader: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddPolicyReader: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeletePolicyReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrreader: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeletePolicyReader: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ApplicationGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppgroupcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ApplicationGroups: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub OpenApplicationGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrgroupname: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    OpenApplicationGroup: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateApplicationGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrgroupname: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateApplicationGroup: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteApplicationGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrgroupname: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteApplicationGroup: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Roles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprolecollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Roles: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub OpenRole: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrrolename: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT, pprole: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    OpenRole: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateRole: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrrolename: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT, pprole: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateRole: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteRole: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrrolename: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteRole: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Tasks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptaskcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Tasks: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub OpenTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtaskname: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT, pptask: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    OpenTask: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtaskname: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT, pptask: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateTask: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtaskname: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteTask: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Submit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Submit: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CanBeDelegated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfprop: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CanBeDelegated: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub BizrulesWritable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfprop: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BizrulesWritable: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PolicyAdministratorsName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvaradmins: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PolicyAdministratorsName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PolicyReadersName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarreaders: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PolicyReadersName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddPolicyAdministratorName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstradmin: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddPolicyAdministratorName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeletePolicyAdministratorName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstradmin: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeletePolicyAdministratorName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddPolicyReaderName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrreader: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddPolicyReaderName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeletePolicyReaderName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrreader: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeletePolicyReaderName: usize,
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAzScope2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAzScope2 {
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.Name)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetName<P0>(&self, bstrname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetName)(::windows::core::Interface::as_raw(self), bstrname.into_param().abi()).ok()
    }
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.Description)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDescription<P0>(&self, bstrdescription: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetDescription)(::windows::core::Interface::as_raw(self), bstrdescription.into_param().abi()).ok()
    }
    pub unsafe fn ApplicationData(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.ApplicationData)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetApplicationData<P0>(&self, bstrapplicationdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetApplicationData)(::windows::core::Interface::as_raw(self), bstrapplicationdata.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Writable(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.Writable)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetProperty(&self, lpropid: i32, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.GetProperty)(::windows::core::Interface::as_raw(self), lpropid, ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetProperty(&self, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetProperty)(::windows::core::Interface::as_raw(self), lpropid, ::core::mem::transmute(varprop), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPropertyItem(&self, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.AddPropertyItem)(::windows::core::Interface::as_raw(self), lpropid, ::core::mem::transmute(varprop), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePropertyItem(&self, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.DeletePropertyItem)(::windows::core::Interface::as_raw(self), lpropid, ::core::mem::transmute(varprop), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PolicyAdministrators(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.PolicyAdministrators)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PolicyReaders(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.PolicyReaders)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPolicyAdministrator<P0>(&self, bstradmin: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.AddPolicyAdministrator)(::windows::core::Interface::as_raw(self), bstradmin.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePolicyAdministrator<P0>(&self, bstradmin: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.DeletePolicyAdministrator)(::windows::core::Interface::as_raw(self), bstradmin.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPolicyReader<P0>(&self, bstrreader: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.AddPolicyReader)(::windows::core::Interface::as_raw(self), bstrreader.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePolicyReader<P0>(&self, bstrreader: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.DeletePolicyReader)(::windows::core::Interface::as_raw(self), bstrreader.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ApplicationGroups(&self) -> ::windows::core::Result<IAzApplicationGroups> {
        let mut result__ = ::windows::core::zeroed::<IAzApplicationGroups>();
        (::windows::core::Interface::vtable(self).base__.ApplicationGroups)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn OpenApplicationGroup<P0>(&self, bstrgroupname: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzApplicationGroup>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzApplicationGroup>();
        (::windows::core::Interface::vtable(self).base__.OpenApplicationGroup)(::windows::core::Interface::as_raw(self), bstrgroupname.into_param().abi(), ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateApplicationGroup<P0>(&self, bstrgroupname: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzApplicationGroup>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzApplicationGroup>();
        (::windows::core::Interface::vtable(self).base__.CreateApplicationGroup)(::windows::core::Interface::as_raw(self), bstrgroupname.into_param().abi(), ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteApplicationGroup<P0>(&self, bstrgroupname: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.DeleteApplicationGroup)(::windows::core::Interface::as_raw(self), bstrgroupname.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Roles(&self) -> ::windows::core::Result<IAzRoles> {
        let mut result__ = ::windows::core::zeroed::<IAzRoles>();
        (::windows::core::Interface::vtable(self).base__.Roles)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn OpenRole<P0>(&self, bstrrolename: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzRole>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzRole>();
        (::windows::core::Interface::vtable(self).base__.OpenRole)(::windows::core::Interface::as_raw(self), bstrrolename.into_param().abi(), ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateRole<P0>(&self, bstrrolename: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzRole>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzRole>();
        (::windows::core::Interface::vtable(self).base__.CreateRole)(::windows::core::Interface::as_raw(self), bstrrolename.into_param().abi(), ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteRole<P0>(&self, bstrrolename: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.DeleteRole)(::windows::core::Interface::as_raw(self), bstrrolename.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Tasks(&self) -> ::windows::core::Result<IAzTasks> {
        let mut result__ = ::windows::core::zeroed::<IAzTasks>();
        (::windows::core::Interface::vtable(self).base__.Tasks)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn OpenTask<P0>(&self, bstrtaskname: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzTask>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzTask>();
        (::windows::core::Interface::vtable(self).base__.OpenTask)(::windows::core::Interface::as_raw(self), bstrtaskname.into_param().abi(), ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateTask<P0>(&self, bstrtaskname: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzTask>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzTask>();
        (::windows::core::Interface::vtable(self).base__.CreateTask)(::windows::core::Interface::as_raw(self), bstrtaskname.into_param().abi(), ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteTask<P0>(&self, bstrtaskname: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.DeleteTask)(::windows::core::Interface::as_raw(self), bstrtaskname.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Submit(&self, lflags: i32, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Submit)(::windows::core::Interface::as_raw(self), lflags, ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CanBeDelegated(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.CanBeDelegated)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BizrulesWritable(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.BizrulesWritable)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PolicyAdministratorsName(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.PolicyAdministratorsName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PolicyReadersName(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.PolicyReadersName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPolicyAdministratorName<P0>(&self, bstradmin: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.AddPolicyAdministratorName)(::windows::core::Interface::as_raw(self), bstradmin.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePolicyAdministratorName<P0>(&self, bstradmin: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.DeletePolicyAdministratorName)(::windows::core::Interface::as_raw(self), bstradmin.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPolicyReaderName<P0>(&self, bstrreader: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.AddPolicyReaderName)(::windows::core::Interface::as_raw(self), bstrreader.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePolicyReaderName<P0>(&self, bstrreader: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.DeletePolicyReaderName)(::windows::core::Interface::as_raw(self), bstrreader.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RoleDefinitions(&self) -> ::windows::core::Result<IAzRoleDefinitions> {
        let mut result__ = ::windows::core::zeroed::<IAzRoleDefinitions>();
        (::windows::core::Interface::vtable(self).RoleDefinitions)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateRoleDefinition<P0>(&self, bstrroledefinitionname: P0) -> ::windows::core::Result<IAzRoleDefinition>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzRoleDefinition>();
        (::windows::core::Interface::vtable(self).CreateRoleDefinition)(::windows::core::Interface::as_raw(self), bstrroledefinitionname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenRoleDefinition<P0>(&self, bstrroledefinitionname: P0) -> ::windows::core::Result<IAzRoleDefinition>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzRoleDefinition>();
        (::windows::core::Interface::vtable(self).OpenRoleDefinition)(::windows::core::Interface::as_raw(self), bstrroledefinitionname.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn DeleteRoleDefinition<P0>(&self, bstrroledefinitionname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).DeleteRoleDefinition)(::windows::core::Interface::as_raw(self), bstrroledefinitionname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RoleAssignments(&self) -> ::windows::core::Result<IAzRoleAssignments> {
        let mut result__ = ::windows::core::zeroed::<IAzRoleAssignments>();
        (::windows::core::Interface::vtable(self).RoleAssignments)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateRoleAssignment<P0>(&self, bstrroleassignmentname: P0) -> ::windows::core::Result<IAzRoleAssignment>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzRoleAssignment>();
        (::windows::core::Interface::vtable(self).CreateRoleAssignment)(::windows::core::Interface::as_raw(self), bstrroleassignmentname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenRoleAssignment<P0>(&self, bstrroleassignmentname: P0) -> ::windows::core::Result<IAzRoleAssignment>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzRoleAssignment>();
        (::windows::core::Interface::vtable(self).OpenRoleAssignment)(::windows::core::Interface::as_raw(self), bstrroleassignmentname.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn DeleteRoleAssignment<P0>(&self, bstrroleassignmentname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).DeleteRoleAssignment)(::windows::core::Interface::as_raw(self), bstrroleassignmentname.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IAzScope2, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IAzScope);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAzScope2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAzScope2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAzScope2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAzScope2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IAzScope2 {
    type Vtable = IAzScope2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAzScope2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IAzScope2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xee9fe8c9_c9f3_40e2_aa12_d1d8599727fd);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzScope2_Vtbl {
    pub base__: IAzScope_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub RoleDefinitions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pproledefinitions: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RoleDefinitions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateRoleDefinition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrroledefinitionname: ::std::mem::MaybeUninit<::windows::core::BSTR>, pproledefinitions: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateRoleDefinition: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OpenRoleDefinition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrroledefinitionname: ::std::mem::MaybeUninit<::windows::core::BSTR>, pproledefinitions: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OpenRoleDefinition: usize,
    pub DeleteRoleDefinition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrroledefinitionname: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub RoleAssignments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pproleassignments: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RoleAssignments: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateRoleAssignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrroleassignmentname: ::std::mem::MaybeUninit<::windows::core::BSTR>, pproleassignment: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateRoleAssignment: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OpenRoleAssignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrroleassignmentname: ::std::mem::MaybeUninit<::windows::core::BSTR>, pproleassignment: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OpenRoleAssignment: usize,
    pub DeleteRoleAssignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrroleassignmentname: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAzScopes(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAzScopes {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_Item(&self, index: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).get_Item)(::windows::core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).Count)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self)._NewEnum)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IAzScopes, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAzScopes {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAzScopes {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAzScopes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAzScopes").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IAzScopes {
    type Vtable = IAzScopes_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAzScopes {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IAzScopes {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x78e14853_9f5e_406d_9b91_6bdba6973510);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzScopes_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, pvarobtptr: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumptr: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAzTask(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAzTask {
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).Name)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetName<P0>(&self, bstrname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetName)(::windows::core::Interface::as_raw(self), bstrname.into_param().abi()).ok()
    }
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).Description)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDescription<P0>(&self, bstrdescription: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetDescription)(::windows::core::Interface::as_raw(self), bstrdescription.into_param().abi()).ok()
    }
    pub unsafe fn ApplicationData(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).ApplicationData)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetApplicationData<P0>(&self, bstrapplicationdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetApplicationData)(::windows::core::Interface::as_raw(self), bstrapplicationdata.into_param().abi()).ok()
    }
    pub unsafe fn BizRule(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).BizRule)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetBizRule<P0>(&self, bstrprop: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetBizRule)(::windows::core::Interface::as_raw(self), bstrprop.into_param().abi()).ok()
    }
    pub unsafe fn BizRuleLanguage(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).BizRuleLanguage)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetBizRuleLanguage<P0>(&self, bstrprop: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetBizRuleLanguage)(::windows::core::Interface::as_raw(self), bstrprop.into_param().abi()).ok()
    }
    pub unsafe fn BizRuleImportedPath(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).BizRuleImportedPath)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetBizRuleImportedPath<P0>(&self, bstrprop: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetBizRuleImportedPath)(::windows::core::Interface::as_raw(self), bstrprop.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRoleDefinition(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).IsRoleDefinition)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIsRoleDefinition<P0>(&self, fprop: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetIsRoleDefinition)(::windows::core::Interface::as_raw(self), fprop.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Operations(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).Operations)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Tasks(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).Tasks)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddOperation<P0>(&self, bstrop: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).AddOperation)(::windows::core::Interface::as_raw(self), bstrop.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteOperation<P0>(&self, bstrop: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).DeleteOperation)(::windows::core::Interface::as_raw(self), bstrop.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddTask<P0>(&self, bstrtask: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).AddTask)(::windows::core::Interface::as_raw(self), bstrtask.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteTask<P0>(&self, bstrtask: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).DeleteTask)(::windows::core::Interface::as_raw(self), bstrtask.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Writable(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).Writable)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetProperty(&self, lpropid: i32, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).GetProperty)(::windows::core::Interface::as_raw(self), lpropid, ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetProperty(&self, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetProperty)(::windows::core::Interface::as_raw(self), lpropid, ::core::mem::transmute(varprop), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPropertyItem(&self, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddPropertyItem)(::windows::core::Interface::as_raw(self), lpropid, ::core::mem::transmute(varprop), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePropertyItem(&self, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeletePropertyItem)(::windows::core::Interface::as_raw(self), lpropid, ::core::mem::transmute(varprop), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Submit(&self, lflags: i32, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Submit)(::windows::core::Interface::as_raw(self), lflags, ::core::mem::transmute(varreserved)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IAzTask, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAzTask {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAzTask {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAzTask {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAzTask").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IAzTask {
    type Vtable = IAzTask_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAzTask {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IAzTask {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcb94e592_2e0e_4a6c_a336_b89a6dc1e388);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzTask_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdescription: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdescription: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub ApplicationData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrapplicationdata: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetApplicationData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrapplicationdata: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub BizRule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrprop: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetBizRule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub BizRuleLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrprop: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetBizRuleLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub BizRuleImportedPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrprop: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetBizRuleImportedPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsRoleDefinition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfprop: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsRoleDefinition: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetIsRoleDefinition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fprop: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetIsRoleDefinition: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Operations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Operations: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Tasks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Tasks: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrop: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddOperation: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrop: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteOperation: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtask: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddTask: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtask: ::std::mem::MaybeUninit<::windows::core::BSTR>, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteTask: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Writable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfprop: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Writable: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpropid: i32, varreserved: super::super::System::Com::VARIANT, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddPropertyItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddPropertyItem: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeletePropertyItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeletePropertyItem: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Submit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Submit: usize,
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAzTask2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAzTask2 {
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.Name)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetName<P0>(&self, bstrname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetName)(::windows::core::Interface::as_raw(self), bstrname.into_param().abi()).ok()
    }
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.Description)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDescription<P0>(&self, bstrdescription: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetDescription)(::windows::core::Interface::as_raw(self), bstrdescription.into_param().abi()).ok()
    }
    pub unsafe fn ApplicationData(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.ApplicationData)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetApplicationData<P0>(&self, bstrapplicationdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetApplicationData)(::windows::core::Interface::as_raw(self), bstrapplicationdata.into_param().abi()).ok()
    }
    pub unsafe fn BizRule(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.BizRule)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetBizRule<P0>(&self, bstrprop: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetBizRule)(::windows::core::Interface::as_raw(self), bstrprop.into_param().abi()).ok()
    }
    pub unsafe fn BizRuleLanguage(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.BizRuleLanguage)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetBizRuleLanguage<P0>(&self, bstrprop: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetBizRuleLanguage)(::windows::core::Interface::as_raw(self), bstrprop.into_param().abi()).ok()
    }
    pub unsafe fn BizRuleImportedPath(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.BizRuleImportedPath)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetBizRuleImportedPath<P0>(&self, bstrprop: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetBizRuleImportedPath)(::windows::core::Interface::as_raw(self), bstrprop.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRoleDefinition(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.IsRoleDefinition)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIsRoleDefinition<P0>(&self, fprop: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.SetIsRoleDefinition)(::windows::core::Interface::as_raw(self), fprop.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Operations(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.Operations)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Tasks(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.Tasks)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddOperation<P0>(&self, bstrop: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.AddOperation)(::windows::core::Interface::as_raw(self), bstrop.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteOperation<P0>(&self, bstrop: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.DeleteOperation)(::windows::core::Interface::as_raw(self), bstrop.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddTask<P0>(&self, bstrtask: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.AddTask)(::windows::core::Interface::as_raw(self), bstrtask.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteTask<P0>(&self, bstrtask: P0, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.DeleteTask)(::windows::core::Interface::as_raw(self), bstrtask.into_param().abi(), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Writable(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.Writable)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetProperty(&self, lpropid: i32, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.GetProperty)(::windows::core::Interface::as_raw(self), lpropid, ::core::mem::transmute(varreserved), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetProperty(&self, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetProperty)(::windows::core::Interface::as_raw(self), lpropid, ::core::mem::transmute(varprop), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPropertyItem(&self, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.AddPropertyItem)(::windows::core::Interface::as_raw(self), lpropid, ::core::mem::transmute(varprop), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePropertyItem(&self, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.DeletePropertyItem)(::windows::core::Interface::as_raw(self), lpropid, ::core::mem::transmute(varprop), ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Submit(&self, lflags: i32, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Submit)(::windows::core::Interface::as_raw(self), lflags, ::core::mem::transmute(varreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn RoleAssignments<P0, P1>(&self, bstrscopename: P0, brecursive: P1) -> ::windows::core::Result<IAzRoleAssignments>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = ::windows::core::zeroed::<IAzRoleAssignments>();
        (::windows::core::Interface::vtable(self).RoleAssignments)(::windows::core::Interface::as_raw(self), bstrscopename.into_param().abi(), brecursive.into_param().abi(), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IAzTask2, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IAzTask);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAzTask2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAzTask2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAzTask2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAzTask2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IAzTask2 {
    type Vtable = IAzTask2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAzTask2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IAzTask2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03a9a5ee_48c8_4832_9025_aad503c46526);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzTask2_Vtbl {
    pub base__: IAzTask_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub RoleAssignments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrscopename: ::std::mem::MaybeUninit<::windows::core::BSTR>, brecursive: super::super::Foundation::VARIANT_BOOL, pproleassignments: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    RoleAssignments: usize,
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAzTasks(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAzTasks {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_Item(&self, index: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).get_Item)(::windows::core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).Count)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self)._NewEnum)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IAzTasks, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAzTasks {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAzTasks {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAzTasks {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAzTasks").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IAzTasks {
    type Vtable = IAzTasks_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAzTasks {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IAzTasks {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb338ccab_4c85_4388_8c0a_c58592bad398);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzTasks_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, pvarobtptr: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumptr: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACCCTRL_DEFAULT_PROVIDER: ::windows::core::PCWSTR = ::windows::w!("Windows NT Access Provider");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACCCTRL_DEFAULT_PROVIDERA: ::windows::core::PCSTR = ::windows::s!("Windows NT Access Provider");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACCCTRL_DEFAULT_PROVIDERW: ::windows::core::PCWSTR = ::windows::w!("Windows NT Access Provider");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_ACCESS_NO_OPTIONS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_ACCESS_PROTECTED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_ACCESS_SUPPORTS_OBJECT_ENTRIES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_CHANGE_ACCESS: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_CHANGE_OWNER: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_DELETE: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_DIR_CREATE_CHILD: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_DIR_CREATE_OBJECT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_DIR_DELETE_CHILD: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_DIR_LIST: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_DIR_TRAVERSE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_FILE_APPEND: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_FILE_CREATE_PIPE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_FILE_EXECUTE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_FILE_READ: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_FILE_READ_ATTRIB: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_FILE_READ_PROP: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_FILE_WRITE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_FILE_WRITE_ATTRIB: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_FILE_WRITE_PROP: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_KERNEL_ALERT: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_KERNEL_CONTROL: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_KERNEL_DIMPERSONATE: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_KERNEL_DUP_HANDLE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_KERNEL_GET_CONTEXT: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_KERNEL_GET_INFO: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_KERNEL_IMPERSONATE: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_KERNEL_PROCESS: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_KERNEL_SET_CONTEXT: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_KERNEL_SET_INFO: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_KERNEL_TERMINATE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_KERNEL_THREAD: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_KERNEL_TOKEN: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_KERNEL_VM: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_KERNEL_VM_READ: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_KERNEL_VM_WRITE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_PERM_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_PERM_10: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_PERM_11: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_PERM_12: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_PERM_13: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_PERM_14: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_PERM_15: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_PERM_16: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_PERM_17: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_PERM_18: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_PERM_19: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_PERM_2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_PERM_20: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_PERM_3: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_PERM_4: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_PERM_5: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_PERM_6: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_PERM_7: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_PERM_8: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_PERM_9: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_PRINT_JADMIN: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_PRINT_PADMIN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_PRINT_PUSE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_PRINT_SADMIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_PRINT_SLIST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_READ_CONTROL: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_REG_CREATE_CHILD: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_REG_LINK: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_REG_LIST: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_REG_NOTIFY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_REG_QUERY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_REG_SET: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_RESERVED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_STD_RIGHTS_ALL: u32 = 4160749568u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_SVC_GET_INFO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_SVC_INTERROGATE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_SVC_LIST: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_SVC_PAUSE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_SVC_SET_INFO: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_SVC_START: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_SVC_STATUS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_SVC_STOP: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_SVC_UCONTROL: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_SYNCHRONIZE: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_SYSTEM_ACCESS: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_WIN_CLIPBRD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_WIN_CREATE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_WIN_EXIT: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_WIN_GLOBAL_ATOMS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_WIN_LIST: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_WIN_LIST_DESK: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_WIN_READ_ATTRIBS: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_WIN_SCREEN: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_WIN_WRITE_ATTRIBS: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const APF_AuditFailure: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const APF_AuditSuccess: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const APF_ValidFlags: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AP_ParamTypeBits: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AP_ParamTypeMask: i32 = 255i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUDIT_TYPE_LEGACY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUDIT_TYPE_WMI: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZP_WPD_EVENT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_ALLOW_MULTIPLE_SOURCE_INSTANCES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_AUDIT_INSTANCE_INFORMATION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_COMPUTE_PRIVILEGES: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_FLAG_ALLOW_MULTIPLE_SOURCE_INSTANCES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_INIT_INFO_VERSION_V1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_MIGRATED_LEGACY_PUBLISHER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_REQUIRE_S4U_LOGON: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_RPC_INIT_INFO_CLIENT_VERSION_V1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_SECURITY_ATTRIBUTES_INFORMATION_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_SECURITY_ATTRIBUTES_INFORMATION_VERSION_V1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_SECURITY_ATTRIBUTE_TYPE_BOOLEAN: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_SECURITY_ATTRIBUTE_TYPE_FQBN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_SECURITY_ATTRIBUTE_TYPE_INT64: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_SECURITY_ATTRIBUTE_TYPE_INVALID: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_SECURITY_ATTRIBUTE_TYPE_OCTET_STRING: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_SECURITY_ATTRIBUTE_TYPE_SID: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_SECURITY_ATTRIBUTE_TYPE_STRING: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_SECURITY_ATTRIBUTE_TYPE_UINT64: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_SKIP_TOKEN_GROUPS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_WPD_CATEGORY_FLAG: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AzAuthorizationStore: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb2bcff59_a757_4b0b_a1bc_ea69981da69e);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AzBizRuleContext: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c2dc96f_8d51_434b_b33c_379bccae77c3);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AzPrincipalLocator: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x483afb5d_70df_4e16_abdc_a1de4d015a3e);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const INHERITED_ACCESS_ENTRY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const INHERITED_GRANDPARENT: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const INHERITED_PARENT: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const OLESCRIPT_E_SYNTAX: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147352319i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_ACCESS_ALLOWED: ::windows::core::PCWSTR = ::windows::w!("A");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_ACCESS_CONTROL_ASSISTANCE_OPS: ::windows::core::PCWSTR = ::windows::w!("AA");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_ACCESS_DENIED: ::windows::core::PCWSTR = ::windows::w!("D");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_ACCESS_FILTER: ::windows::core::PCWSTR = ::windows::w!("FL");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_ACCOUNT_OPERATORS: ::windows::core::PCWSTR = ::windows::w!("AO");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_ACE_BEGIN: ::windows::core::PCWSTR = ::windows::w!("(");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_ACE_COND_ATTRIBUTE_PREFIX: ::windows::core::PCWSTR = ::windows::w!("@");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_ACE_COND_BEGIN: ::windows::core::PCWSTR = ::windows::w!("(");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_ACE_COND_BLOB_PREFIX: ::windows::core::PCWSTR = ::windows::w!("#");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_ACE_COND_DEVICE_ATTRIBUTE_PREFIX: ::windows::core::PCWSTR = ::windows::w!("@DEVICE.");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_ACE_COND_END: ::windows::core::PCWSTR = ::windows::w!(")");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_ACE_COND_RESOURCE_ATTRIBUTE_PREFIX: ::windows::core::PCWSTR = ::windows::w!("@RESOURCE.");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_ACE_COND_SID_PREFIX: ::windows::core::PCWSTR = ::windows::w!("SID");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_ACE_COND_TOKEN_ATTRIBUTE_PREFIX: ::windows::core::PCWSTR = ::windows::w!("@TOKEN.");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_ACE_COND_USER_ATTRIBUTE_PREFIX: ::windows::core::PCWSTR = ::windows::w!("@USER.");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_ACE_END: ::windows::core::PCWSTR = ::windows::w!(")");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_ALARM: ::windows::core::PCWSTR = ::windows::w!("AL");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_ALIAS_PREW2KCOMPACC: ::windows::core::PCWSTR = ::windows::w!("RU");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_ALIAS_SIZE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_ALL_APP_PACKAGES: ::windows::core::PCWSTR = ::windows::w!("AC");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_ANONYMOUS: ::windows::core::PCWSTR = ::windows::w!("AN");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_AUDIT: ::windows::core::PCWSTR = ::windows::w!("AU");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_AUDIT_FAILURE: ::windows::core::PCWSTR = ::windows::w!("FA");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_AUDIT_SUCCESS: ::windows::core::PCWSTR = ::windows::w!("SA");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_AUTHENTICATED_USERS: ::windows::core::PCWSTR = ::windows::w!("AU");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_AUTHORITY_ASSERTED: ::windows::core::PCWSTR = ::windows::w!("AS");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_AUTO_INHERITED: ::windows::core::PCWSTR = ::windows::w!("AI");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_AUTO_INHERIT_REQ: ::windows::core::PCWSTR = ::windows::w!("AR");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_BACKUP_OPERATORS: ::windows::core::PCWSTR = ::windows::w!("BO");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_BLOB: ::windows::core::PCWSTR = ::windows::w!("TX");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_BOOLEAN: ::windows::core::PCWSTR = ::windows::w!("TB");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_BUILTIN_ADMINISTRATORS: ::windows::core::PCWSTR = ::windows::w!("BA");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_BUILTIN_GUESTS: ::windows::core::PCWSTR = ::windows::w!("BG");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_BUILTIN_USERS: ::windows::core::PCWSTR = ::windows::w!("BU");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_CALLBACK_ACCESS_ALLOWED: ::windows::core::PCWSTR = ::windows::w!("XA");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_CALLBACK_ACCESS_DENIED: ::windows::core::PCWSTR = ::windows::w!("XD");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_CALLBACK_AUDIT: ::windows::core::PCWSTR = ::windows::w!("XU");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_CALLBACK_OBJECT_ACCESS_ALLOWED: ::windows::core::PCWSTR = ::windows::w!("ZA");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_CERTSVC_DCOM_ACCESS: ::windows::core::PCWSTR = ::windows::w!("CD");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_CERT_SERV_ADMINISTRATORS: ::windows::core::PCWSTR = ::windows::w!("CA");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_CLONEABLE_CONTROLLERS: ::windows::core::PCWSTR = ::windows::w!("CN");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_CONTAINER_INHERIT: ::windows::core::PCWSTR = ::windows::w!("CI");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_CONTROL_ACCESS: ::windows::core::PCWSTR = ::windows::w!("CR");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_CREATE_CHILD: ::windows::core::PCWSTR = ::windows::w!("CC");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_CREATOR_GROUP: ::windows::core::PCWSTR = ::windows::w!("CG");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_CREATOR_OWNER: ::windows::core::PCWSTR = ::windows::w!("CO");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_CRITICAL: ::windows::core::PCWSTR = ::windows::w!("CR");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_CRYPTO_OPERATORS: ::windows::core::PCWSTR = ::windows::w!("CY");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_DACL: ::windows::core::PCWSTR = ::windows::w!("D");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_DELETE_CHILD: ::windows::core::PCWSTR = ::windows::w!("DC");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_DELETE_TREE: ::windows::core::PCWSTR = ::windows::w!("DT");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_DELIMINATOR: ::windows::core::PCWSTR = ::windows::w!(":");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_DOMAIN_ADMINISTRATORS: ::windows::core::PCWSTR = ::windows::w!("DA");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_DOMAIN_COMPUTERS: ::windows::core::PCWSTR = ::windows::w!("DC");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_DOMAIN_DOMAIN_CONTROLLERS: ::windows::core::PCWSTR = ::windows::w!("DD");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_DOMAIN_GUESTS: ::windows::core::PCWSTR = ::windows::w!("DG");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_DOMAIN_USERS: ::windows::core::PCWSTR = ::windows::w!("DU");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_ENTERPRISE_ADMINS: ::windows::core::PCWSTR = ::windows::w!("EA");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_ENTERPRISE_DOMAIN_CONTROLLERS: ::windows::core::PCWSTR = ::windows::w!("ED");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_ENTERPRISE_KEY_ADMINS: ::windows::core::PCWSTR = ::windows::w!("EK");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_ENTERPRISE_RO_DCs: ::windows::core::PCWSTR = ::windows::w!("RO");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_EVENT_LOG_READERS: ::windows::core::PCWSTR = ::windows::w!("ER");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_EVERYONE: ::windows::core::PCWSTR = ::windows::w!("WD");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_FILE_ALL: ::windows::core::PCWSTR = ::windows::w!("FA");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_FILE_EXECUTE: ::windows::core::PCWSTR = ::windows::w!("FX");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_FILE_READ: ::windows::core::PCWSTR = ::windows::w!("FR");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_FILE_WRITE: ::windows::core::PCWSTR = ::windows::w!("FW");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_GENERIC_ALL: ::windows::core::PCWSTR = ::windows::w!("GA");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_GENERIC_EXECUTE: ::windows::core::PCWSTR = ::windows::w!("GX");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_GENERIC_READ: ::windows::core::PCWSTR = ::windows::w!("GR");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_GENERIC_WRITE: ::windows::core::PCWSTR = ::windows::w!("GW");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_GROUP: ::windows::core::PCWSTR = ::windows::w!("G");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_GROUP_POLICY_ADMINS: ::windows::core::PCWSTR = ::windows::w!("PA");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_HYPER_V_ADMINS: ::windows::core::PCWSTR = ::windows::w!("HA");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_IIS_USERS: ::windows::core::PCWSTR = ::windows::w!("IS");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_INHERITED: ::windows::core::PCWSTR = ::windows::w!("ID");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_INHERIT_ONLY: ::windows::core::PCWSTR = ::windows::w!("IO");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_INT: ::windows::core::PCWSTR = ::windows::w!("TI");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_INTERACTIVE: ::windows::core::PCWSTR = ::windows::w!("IU");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_KEY_ADMINS: ::windows::core::PCWSTR = ::windows::w!("KA");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_KEY_ALL: ::windows::core::PCWSTR = ::windows::w!("KA");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_KEY_EXECUTE: ::windows::core::PCWSTR = ::windows::w!("KX");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_KEY_READ: ::windows::core::PCWSTR = ::windows::w!("KR");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_KEY_WRITE: ::windows::core::PCWSTR = ::windows::w!("KW");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_LIST_CHILDREN: ::windows::core::PCWSTR = ::windows::w!("LC");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_LIST_OBJECT: ::windows::core::PCWSTR = ::windows::w!("LO");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_LOCAL_ADMIN: ::windows::core::PCWSTR = ::windows::w!("LA");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_LOCAL_GUEST: ::windows::core::PCWSTR = ::windows::w!("LG");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_LOCAL_SERVICE: ::windows::core::PCWSTR = ::windows::w!("LS");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_LOCAL_SYSTEM: ::windows::core::PCWSTR = ::windows::w!("SY");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_MANDATORY_LABEL: ::windows::core::PCWSTR = ::windows::w!("ML");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_ML_HIGH: ::windows::core::PCWSTR = ::windows::w!("HI");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_ML_LOW: ::windows::core::PCWSTR = ::windows::w!("LW");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_ML_MEDIUM: ::windows::core::PCWSTR = ::windows::w!("ME");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_ML_MEDIUM_PLUS: ::windows::core::PCWSTR = ::windows::w!("MP");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_ML_SYSTEM: ::windows::core::PCWSTR = ::windows::w!("SI");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_NETWORK: ::windows::core::PCWSTR = ::windows::w!("NU");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_NETWORK_CONFIGURATION_OPS: ::windows::core::PCWSTR = ::windows::w!("NO");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_NETWORK_SERVICE: ::windows::core::PCWSTR = ::windows::w!("NS");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_NO_EXECUTE_UP: ::windows::core::PCWSTR = ::windows::w!("NX");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_NO_PROPAGATE: ::windows::core::PCWSTR = ::windows::w!("NP");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_NO_READ_UP: ::windows::core::PCWSTR = ::windows::w!("NR");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_NO_WRITE_UP: ::windows::core::PCWSTR = ::windows::w!("NW");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_NULL_ACL: ::windows::core::PCWSTR = ::windows::w!("NO_ACCESS_CONTROL");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_OBJECT_ACCESS_ALLOWED: ::windows::core::PCWSTR = ::windows::w!("OA");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_OBJECT_ACCESS_DENIED: ::windows::core::PCWSTR = ::windows::w!("OD");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_OBJECT_ALARM: ::windows::core::PCWSTR = ::windows::w!("OL");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_OBJECT_AUDIT: ::windows::core::PCWSTR = ::windows::w!("OU");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_OBJECT_INHERIT: ::windows::core::PCWSTR = ::windows::w!("OI");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_OWNER: ::windows::core::PCWSTR = ::windows::w!("O");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_OWNER_RIGHTS: ::windows::core::PCWSTR = ::windows::w!("OW");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_PERFLOG_USERS: ::windows::core::PCWSTR = ::windows::w!("LU");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_PERFMON_USERS: ::windows::core::PCWSTR = ::windows::w!("MU");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_PERSONAL_SELF: ::windows::core::PCWSTR = ::windows::w!("PS");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_POWER_USERS: ::windows::core::PCWSTR = ::windows::w!("PU");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_PRINTER_OPERATORS: ::windows::core::PCWSTR = ::windows::w!("PO");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_PROCESS_TRUST_LABEL: ::windows::core::PCWSTR = ::windows::w!("TL");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_PROTECTED: ::windows::core::PCWSTR = ::windows::w!("P");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_PROTECTED_USERS: ::windows::core::PCWSTR = ::windows::w!("AP");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_RAS_SERVERS: ::windows::core::PCWSTR = ::windows::w!("RS");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_RDS_ENDPOINT_SERVERS: ::windows::core::PCWSTR = ::windows::w!("ES");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_RDS_MANAGEMENT_SERVERS: ::windows::core::PCWSTR = ::windows::w!("MS");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_RDS_REMOTE_ACCESS_SERVERS: ::windows::core::PCWSTR = ::windows::w!("RA");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_READ_CONTROL: ::windows::core::PCWSTR = ::windows::w!("RC");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_READ_PROPERTY: ::windows::core::PCWSTR = ::windows::w!("RP");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_REMOTE_DESKTOP: ::windows::core::PCWSTR = ::windows::w!("RD");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_REMOTE_MANAGEMENT_USERS: ::windows::core::PCWSTR = ::windows::w!("RM");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_REPLICATOR: ::windows::core::PCWSTR = ::windows::w!("RE");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_RESOURCE_ATTRIBUTE: ::windows::core::PCWSTR = ::windows::w!("RA");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_RESTRICTED_CODE: ::windows::core::PCWSTR = ::windows::w!("RC");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_REVISION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_SACL: ::windows::core::PCWSTR = ::windows::w!("S");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_SCHEMA_ADMINISTRATORS: ::windows::core::PCWSTR = ::windows::w!("SA");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_SCOPED_POLICY_ID: ::windows::core::PCWSTR = ::windows::w!("SP");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_SELF_WRITE: ::windows::core::PCWSTR = ::windows::w!("SW");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_SEPERATOR: ::windows::core::PCWSTR = ::windows::w!(";");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_SERVER_OPERATORS: ::windows::core::PCWSTR = ::windows::w!("SO");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_SERVICE: ::windows::core::PCWSTR = ::windows::w!("SU");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_SERVICE_ASSERTED: ::windows::core::PCWSTR = ::windows::w!("SS");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_SID: ::windows::core::PCWSTR = ::windows::w!("TD");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_SPACE: ::windows::core::PCWSTR = ::windows::w!(" ");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_STANDARD_DELETE: ::windows::core::PCWSTR = ::windows::w!("SD");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_TRUST_PROTECTED_FILTER: ::windows::core::PCWSTR = ::windows::w!("TP");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_UINT: ::windows::core::PCWSTR = ::windows::w!("TU");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_USER_MODE_DRIVERS: ::windows::core::PCWSTR = ::windows::w!("UD");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_WRITE_DAC: ::windows::core::PCWSTR = ::windows::w!("WD");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_WRITE_OWNER: ::windows::core::PCWSTR = ::windows::w!("WO");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_WRITE_PROPERTY: ::windows::core::PCWSTR = ::windows::w!("WP");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_WRITE_RESTRICTED_CODE: ::windows::core::PCWSTR = ::windows::w!("WR");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SDDL_WSTRING: ::windows::core::PCWSTR = ::windows::w!("TS");
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const TRUSTEE_ACCESS_ALL: i32 = -1i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const TRUSTEE_ACCESS_ALLOWED: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const TRUSTEE_ACCESS_EXPLICIT: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const TRUSTEE_ACCESS_READ: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const TRUSTEE_ACCESS_WRITE: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const _AUTHZ_SS_MAXSIZE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ACCESS_MODE(pub i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const NOT_USED_ACCESS: ACCESS_MODE = ACCESS_MODE(0i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const GRANT_ACCESS: ACCESS_MODE = ACCESS_MODE(1i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SET_ACCESS: ACCESS_MODE = ACCESS_MODE(2i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const DENY_ACCESS: ACCESS_MODE = ACCESS_MODE(3i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const REVOKE_ACCESS: ACCESS_MODE = ACCESS_MODE(4i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SET_AUDIT_SUCCESS: ACCESS_MODE = ACCESS_MODE(5i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SET_AUDIT_FAILURE: ACCESS_MODE = ACCESS_MODE(6i32);
impl ::core::marker::Copy for ACCESS_MODE {}
impl ::core::clone::Clone for ACCESS_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ACCESS_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for ACCESS_MODE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for ACCESS_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ACCESS_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ACTRL_ACCESS_ENTRY_ACCESS_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_ACCESS_ALLOWED: ACTRL_ACCESS_ENTRY_ACCESS_FLAGS = ACTRL_ACCESS_ENTRY_ACCESS_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_ACCESS_DENIED: ACTRL_ACCESS_ENTRY_ACCESS_FLAGS = ACTRL_ACCESS_ENTRY_ACCESS_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_AUDIT_SUCCESS: ACTRL_ACCESS_ENTRY_ACCESS_FLAGS = ACTRL_ACCESS_ENTRY_ACCESS_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ACTRL_AUDIT_FAILURE: ACTRL_ACCESS_ENTRY_ACCESS_FLAGS = ACTRL_ACCESS_ENTRY_ACCESS_FLAGS(8u32);
impl ::core::marker::Copy for ACTRL_ACCESS_ENTRY_ACCESS_FLAGS {}
impl ::core::clone::Clone for ACTRL_ACCESS_ENTRY_ACCESS_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ACTRL_ACCESS_ENTRY_ACCESS_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for ACTRL_ACCESS_ENTRY_ACCESS_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for ACTRL_ACCESS_ENTRY_ACCESS_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ACTRL_ACCESS_ENTRY_ACCESS_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AUDIT_PARAM_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const APT_None: AUDIT_PARAM_TYPE = AUDIT_PARAM_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const APT_String: AUDIT_PARAM_TYPE = AUDIT_PARAM_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const APT_Ulong: AUDIT_PARAM_TYPE = AUDIT_PARAM_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const APT_Pointer: AUDIT_PARAM_TYPE = AUDIT_PARAM_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const APT_Sid: AUDIT_PARAM_TYPE = AUDIT_PARAM_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const APT_LogonId: AUDIT_PARAM_TYPE = AUDIT_PARAM_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const APT_ObjectTypeList: AUDIT_PARAM_TYPE = AUDIT_PARAM_TYPE(7i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const APT_Luid: AUDIT_PARAM_TYPE = AUDIT_PARAM_TYPE(8i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const APT_Guid: AUDIT_PARAM_TYPE = AUDIT_PARAM_TYPE(9i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const APT_Time: AUDIT_PARAM_TYPE = AUDIT_PARAM_TYPE(10i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const APT_Int64: AUDIT_PARAM_TYPE = AUDIT_PARAM_TYPE(11i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const APT_IpAddress: AUDIT_PARAM_TYPE = AUDIT_PARAM_TYPE(12i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const APT_LogonIdWithSid: AUDIT_PARAM_TYPE = AUDIT_PARAM_TYPE(13i32);
impl ::core::marker::Copy for AUDIT_PARAM_TYPE {}
impl ::core::clone::Clone for AUDIT_PARAM_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AUDIT_PARAM_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for AUDIT_PARAM_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for AUDIT_PARAM_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUDIT_PARAM_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AUTHZ_ACCESS_CHECK_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_ACCESS_CHECK_NO_DEEP_COPY_SD: AUTHZ_ACCESS_CHECK_FLAGS = AUTHZ_ACCESS_CHECK_FLAGS(1u32);
impl ::core::marker::Copy for AUTHZ_ACCESS_CHECK_FLAGS {}
impl ::core::clone::Clone for AUTHZ_ACCESS_CHECK_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AUTHZ_ACCESS_CHECK_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for AUTHZ_ACCESS_CHECK_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for AUTHZ_ACCESS_CHECK_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUTHZ_ACCESS_CHECK_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AUTHZ_AUDIT_EVENT_INFORMATION_CLASS(pub i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AuthzAuditEventInfoFlags: AUTHZ_AUDIT_EVENT_INFORMATION_CLASS = AUTHZ_AUDIT_EVENT_INFORMATION_CLASS(1i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AuthzAuditEventInfoOperationType: AUTHZ_AUDIT_EVENT_INFORMATION_CLASS = AUTHZ_AUDIT_EVENT_INFORMATION_CLASS(2i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AuthzAuditEventInfoObjectType: AUTHZ_AUDIT_EVENT_INFORMATION_CLASS = AUTHZ_AUDIT_EVENT_INFORMATION_CLASS(3i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AuthzAuditEventInfoObjectName: AUTHZ_AUDIT_EVENT_INFORMATION_CLASS = AUTHZ_AUDIT_EVENT_INFORMATION_CLASS(4i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AuthzAuditEventInfoAdditionalInfo: AUTHZ_AUDIT_EVENT_INFORMATION_CLASS = AUTHZ_AUDIT_EVENT_INFORMATION_CLASS(5i32);
impl ::core::marker::Copy for AUTHZ_AUDIT_EVENT_INFORMATION_CLASS {}
impl ::core::clone::Clone for AUTHZ_AUDIT_EVENT_INFORMATION_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AUTHZ_AUDIT_EVENT_INFORMATION_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for AUTHZ_AUDIT_EVENT_INFORMATION_CLASS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for AUTHZ_AUDIT_EVENT_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUTHZ_AUDIT_EVENT_INFORMATION_CLASS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AUTHZ_CONTEXT_INFORMATION_CLASS(pub i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AuthzContextInfoUserSid: AUTHZ_CONTEXT_INFORMATION_CLASS = AUTHZ_CONTEXT_INFORMATION_CLASS(1i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AuthzContextInfoGroupsSids: AUTHZ_CONTEXT_INFORMATION_CLASS = AUTHZ_CONTEXT_INFORMATION_CLASS(2i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AuthzContextInfoRestrictedSids: AUTHZ_CONTEXT_INFORMATION_CLASS = AUTHZ_CONTEXT_INFORMATION_CLASS(3i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AuthzContextInfoPrivileges: AUTHZ_CONTEXT_INFORMATION_CLASS = AUTHZ_CONTEXT_INFORMATION_CLASS(4i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AuthzContextInfoExpirationTime: AUTHZ_CONTEXT_INFORMATION_CLASS = AUTHZ_CONTEXT_INFORMATION_CLASS(5i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AuthzContextInfoServerContext: AUTHZ_CONTEXT_INFORMATION_CLASS = AUTHZ_CONTEXT_INFORMATION_CLASS(6i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AuthzContextInfoIdentifier: AUTHZ_CONTEXT_INFORMATION_CLASS = AUTHZ_CONTEXT_INFORMATION_CLASS(7i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AuthzContextInfoSource: AUTHZ_CONTEXT_INFORMATION_CLASS = AUTHZ_CONTEXT_INFORMATION_CLASS(8i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AuthzContextInfoAll: AUTHZ_CONTEXT_INFORMATION_CLASS = AUTHZ_CONTEXT_INFORMATION_CLASS(9i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AuthzContextInfoAuthenticationId: AUTHZ_CONTEXT_INFORMATION_CLASS = AUTHZ_CONTEXT_INFORMATION_CLASS(10i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AuthzContextInfoSecurityAttributes: AUTHZ_CONTEXT_INFORMATION_CLASS = AUTHZ_CONTEXT_INFORMATION_CLASS(11i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AuthzContextInfoDeviceSids: AUTHZ_CONTEXT_INFORMATION_CLASS = AUTHZ_CONTEXT_INFORMATION_CLASS(12i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AuthzContextInfoUserClaims: AUTHZ_CONTEXT_INFORMATION_CLASS = AUTHZ_CONTEXT_INFORMATION_CLASS(13i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AuthzContextInfoDeviceClaims: AUTHZ_CONTEXT_INFORMATION_CLASS = AUTHZ_CONTEXT_INFORMATION_CLASS(14i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AuthzContextInfoAppContainerSid: AUTHZ_CONTEXT_INFORMATION_CLASS = AUTHZ_CONTEXT_INFORMATION_CLASS(15i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AuthzContextInfoCapabilitySids: AUTHZ_CONTEXT_INFORMATION_CLASS = AUTHZ_CONTEXT_INFORMATION_CLASS(16i32);
impl ::core::marker::Copy for AUTHZ_CONTEXT_INFORMATION_CLASS {}
impl ::core::clone::Clone for AUTHZ_CONTEXT_INFORMATION_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AUTHZ_CONTEXT_INFORMATION_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for AUTHZ_CONTEXT_INFORMATION_CLASS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for AUTHZ_CONTEXT_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUTHZ_CONTEXT_INFORMATION_CLASS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AUTHZ_GENERATE_RESULTS(pub u32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_GENERATE_SUCCESS_AUDIT: AUTHZ_GENERATE_RESULTS = AUTHZ_GENERATE_RESULTS(1u32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_GENERATE_FAILURE_AUDIT: AUTHZ_GENERATE_RESULTS = AUTHZ_GENERATE_RESULTS(2u32);
impl ::core::marker::Copy for AUTHZ_GENERATE_RESULTS {}
impl ::core::clone::Clone for AUTHZ_GENERATE_RESULTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AUTHZ_GENERATE_RESULTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for AUTHZ_GENERATE_RESULTS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for AUTHZ_GENERATE_RESULTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUTHZ_GENERATE_RESULTS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AUTHZ_INITIALIZE_OBJECT_ACCESS_AUDIT_EVENT_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_NO_SUCCESS_AUDIT: AUTHZ_INITIALIZE_OBJECT_ACCESS_AUDIT_EVENT_FLAGS = AUTHZ_INITIALIZE_OBJECT_ACCESS_AUDIT_EVENT_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_NO_FAILURE_AUDIT: AUTHZ_INITIALIZE_OBJECT_ACCESS_AUDIT_EVENT_FLAGS = AUTHZ_INITIALIZE_OBJECT_ACCESS_AUDIT_EVENT_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_NO_ALLOC_STRINGS: AUTHZ_INITIALIZE_OBJECT_ACCESS_AUDIT_EVENT_FLAGS = AUTHZ_INITIALIZE_OBJECT_ACCESS_AUDIT_EVENT_FLAGS(4u32);
impl ::core::marker::Copy for AUTHZ_INITIALIZE_OBJECT_ACCESS_AUDIT_EVENT_FLAGS {}
impl ::core::clone::Clone for AUTHZ_INITIALIZE_OBJECT_ACCESS_AUDIT_EVENT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AUTHZ_INITIALIZE_OBJECT_ACCESS_AUDIT_EVENT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for AUTHZ_INITIALIZE_OBJECT_ACCESS_AUDIT_EVENT_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for AUTHZ_INITIALIZE_OBJECT_ACCESS_AUDIT_EVENT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUTHZ_INITIALIZE_OBJECT_ACCESS_AUDIT_EVENT_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AUTHZ_RESOURCE_MANAGER_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_RM_FLAG_NO_AUDIT: AUTHZ_RESOURCE_MANAGER_FLAGS = AUTHZ_RESOURCE_MANAGER_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_RM_FLAG_INITIALIZE_UNDER_IMPERSONATION: AUTHZ_RESOURCE_MANAGER_FLAGS = AUTHZ_RESOURCE_MANAGER_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_RM_FLAG_NO_CENTRAL_ACCESS_POLICIES: AUTHZ_RESOURCE_MANAGER_FLAGS = AUTHZ_RESOURCE_MANAGER_FLAGS(4u32);
impl ::core::marker::Copy for AUTHZ_RESOURCE_MANAGER_FLAGS {}
impl ::core::clone::Clone for AUTHZ_RESOURCE_MANAGER_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AUTHZ_RESOURCE_MANAGER_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for AUTHZ_RESOURCE_MANAGER_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for AUTHZ_RESOURCE_MANAGER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUTHZ_RESOURCE_MANAGER_FLAGS").field(&self.0).finish()
    }
}
impl AUTHZ_RESOURCE_MANAGER_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for AUTHZ_RESOURCE_MANAGER_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for AUTHZ_RESOURCE_MANAGER_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for AUTHZ_RESOURCE_MANAGER_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for AUTHZ_RESOURCE_MANAGER_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for AUTHZ_RESOURCE_MANAGER_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AUTHZ_SECURITY_ATTRIBUTE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_SECURITY_ATTRIBUTE_NON_INHERITABLE: AUTHZ_SECURITY_ATTRIBUTE_FLAGS = AUTHZ_SECURITY_ATTRIBUTE_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_SECURITY_ATTRIBUTE_VALUE_CASE_SENSITIVE: AUTHZ_SECURITY_ATTRIBUTE_FLAGS = AUTHZ_SECURITY_ATTRIBUTE_FLAGS(2u32);
impl ::core::marker::Copy for AUTHZ_SECURITY_ATTRIBUTE_FLAGS {}
impl ::core::clone::Clone for AUTHZ_SECURITY_ATTRIBUTE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AUTHZ_SECURITY_ATTRIBUTE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for AUTHZ_SECURITY_ATTRIBUTE_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for AUTHZ_SECURITY_ATTRIBUTE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUTHZ_SECURITY_ATTRIBUTE_FLAGS").field(&self.0).finish()
    }
}
impl AUTHZ_SECURITY_ATTRIBUTE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for AUTHZ_SECURITY_ATTRIBUTE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for AUTHZ_SECURITY_ATTRIBUTE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for AUTHZ_SECURITY_ATTRIBUTE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for AUTHZ_SECURITY_ATTRIBUTE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for AUTHZ_SECURITY_ATTRIBUTE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AUTHZ_SECURITY_ATTRIBUTE_OPERATION(pub i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_SECURITY_ATTRIBUTE_OPERATION_NONE: AUTHZ_SECURITY_ATTRIBUTE_OPERATION = AUTHZ_SECURITY_ATTRIBUTE_OPERATION(0i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_SECURITY_ATTRIBUTE_OPERATION_REPLACE_ALL: AUTHZ_SECURITY_ATTRIBUTE_OPERATION = AUTHZ_SECURITY_ATTRIBUTE_OPERATION(1i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_SECURITY_ATTRIBUTE_OPERATION_ADD: AUTHZ_SECURITY_ATTRIBUTE_OPERATION = AUTHZ_SECURITY_ATTRIBUTE_OPERATION(2i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_SECURITY_ATTRIBUTE_OPERATION_DELETE: AUTHZ_SECURITY_ATTRIBUTE_OPERATION = AUTHZ_SECURITY_ATTRIBUTE_OPERATION(3i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_SECURITY_ATTRIBUTE_OPERATION_REPLACE: AUTHZ_SECURITY_ATTRIBUTE_OPERATION = AUTHZ_SECURITY_ATTRIBUTE_OPERATION(4i32);
impl ::core::marker::Copy for AUTHZ_SECURITY_ATTRIBUTE_OPERATION {}
impl ::core::clone::Clone for AUTHZ_SECURITY_ATTRIBUTE_OPERATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AUTHZ_SECURITY_ATTRIBUTE_OPERATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for AUTHZ_SECURITY_ATTRIBUTE_OPERATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for AUTHZ_SECURITY_ATTRIBUTE_OPERATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUTHZ_SECURITY_ATTRIBUTE_OPERATION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AUTHZ_SID_OPERATION(pub i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_SID_OPERATION_NONE: AUTHZ_SID_OPERATION = AUTHZ_SID_OPERATION(0i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_SID_OPERATION_REPLACE_ALL: AUTHZ_SID_OPERATION = AUTHZ_SID_OPERATION(1i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_SID_OPERATION_ADD: AUTHZ_SID_OPERATION = AUTHZ_SID_OPERATION(2i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_SID_OPERATION_DELETE: AUTHZ_SID_OPERATION = AUTHZ_SID_OPERATION(3i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AUTHZ_SID_OPERATION_REPLACE: AUTHZ_SID_OPERATION = AUTHZ_SID_OPERATION(4i32);
impl ::core::marker::Copy for AUTHZ_SID_OPERATION {}
impl ::core::clone::Clone for AUTHZ_SID_OPERATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AUTHZ_SID_OPERATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for AUTHZ_SID_OPERATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for AUTHZ_SID_OPERATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUTHZ_SID_OPERATION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AZ_PROP_CONSTANTS(pub i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_NAME: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(1i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_DESCRIPTION: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(2i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_WRITABLE: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(3i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_APPLICATION_DATA: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(4i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_CHILD_CREATE: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(5i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_MAX_APPLICATION_NAME_LENGTH: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(512i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_MAX_OPERATION_NAME_LENGTH: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(64i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_MAX_TASK_NAME_LENGTH: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(64i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_MAX_SCOPE_NAME_LENGTH: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(65536i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_MAX_GROUP_NAME_LENGTH: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(64i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_MAX_ROLE_NAME_LENGTH: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(64i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_MAX_NAME_LENGTH: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(65536i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_MAX_DESCRIPTION_LENGTH: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(1024i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_MAX_APPLICATION_DATA_LENGTH: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(4096i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_SUBMIT_FLAG_ABORT: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(1i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_SUBMIT_FLAG_FLUSH: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(2i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_MAX_POLICY_URL_LENGTH: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(65536i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_AZSTORE_FLAG_CREATE: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(1i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_AZSTORE_FLAG_MANAGE_STORE_ONLY: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(2i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_AZSTORE_FLAG_BATCH_UPDATE: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(4i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_AZSTORE_FLAG_AUDIT_IS_CRITICAL: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(8i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_AZSTORE_FORCE_APPLICATION_CLOSE: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(16i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_AZSTORE_NT6_FUNCTION_LEVEL: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(32i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_AZSTORE_FLAG_MANAGE_ONLY_PASSIVE_SUBMIT: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(32768i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_AZSTORE_DOMAIN_TIMEOUT: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(100i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_AZSTORE_DEFAULT_DOMAIN_TIMEOUT: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(15000i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_AZSTORE_SCRIPT_ENGINE_TIMEOUT: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(101i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_AZSTORE_MIN_DOMAIN_TIMEOUT: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(500i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_AZSTORE_MIN_SCRIPT_ENGINE_TIMEOUT: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(5000i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_AZSTORE_DEFAULT_SCRIPT_ENGINE_TIMEOUT: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(45000i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_AZSTORE_MAX_SCRIPT_ENGINES: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(102i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_AZSTORE_DEFAULT_MAX_SCRIPT_ENGINES: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(120i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_AZSTORE_MAJOR_VERSION: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(103i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_AZSTORE_MINOR_VERSION: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(104i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_AZSTORE_TARGET_MACHINE: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(105i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_AZTORE_IS_ADAM_INSTANCE: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(106i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_OPERATION_ID: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(200i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_TASK_OPERATIONS: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(300i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_TASK_BIZRULE: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(301i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_TASK_BIZRULE_LANGUAGE: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(302i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_TASK_TASKS: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(303i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_TASK_BIZRULE_IMPORTED_PATH: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(304i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_TASK_IS_ROLE_DEFINITION: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(305i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_MAX_TASK_BIZRULE_LENGTH: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(65536i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_MAX_TASK_BIZRULE_LANGUAGE_LENGTH: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(64i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_MAX_TASK_BIZRULE_IMPORTED_PATH_LENGTH: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(512i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_MAX_BIZRULE_STRING: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(65536i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_GROUP_TYPE: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(400i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_GROUPTYPE_LDAP_QUERY: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(1i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_GROUPTYPE_BASIC: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(2i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_GROUPTYPE_BIZRULE: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(3i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_GROUP_APP_MEMBERS: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(401i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_GROUP_APP_NON_MEMBERS: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(402i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_GROUP_LDAP_QUERY: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(403i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_MAX_GROUP_LDAP_QUERY_LENGTH: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(4096i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_GROUP_MEMBERS: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(404i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_GROUP_NON_MEMBERS: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(405i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_GROUP_MEMBERS_NAME: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(406i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_GROUP_NON_MEMBERS_NAME: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(407i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_GROUP_BIZRULE: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(408i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_GROUP_BIZRULE_LANGUAGE: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(409i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_GROUP_BIZRULE_IMPORTED_PATH: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(410i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_MAX_GROUP_BIZRULE_LENGTH: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(65536i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_MAX_GROUP_BIZRULE_LANGUAGE_LENGTH: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(64i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_MAX_GROUP_BIZRULE_IMPORTED_PATH_LENGTH: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(512i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_ROLE_APP_MEMBERS: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(500i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_ROLE_MEMBERS: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(501i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_ROLE_OPERATIONS: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(502i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_ROLE_TASKS: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(504i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_ROLE_MEMBERS_NAME: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(505i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_SCOPE_BIZRULES_WRITABLE: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(600i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_SCOPE_CAN_BE_DELEGATED: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(601i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_CLIENT_CONTEXT_USER_DN: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(700i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_CLIENT_CONTEXT_USER_SAM_COMPAT: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(701i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_CLIENT_CONTEXT_USER_DISPLAY: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(702i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_CLIENT_CONTEXT_USER_GUID: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(703i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_CLIENT_CONTEXT_USER_CANONICAL: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(704i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_CLIENT_CONTEXT_USER_UPN: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(705i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_CLIENT_CONTEXT_USER_DNS_SAM_COMPAT: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(707i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_CLIENT_CONTEXT_ROLE_FOR_ACCESS_CHECK: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(708i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_CLIENT_CONTEXT_LDAP_QUERY_DN: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(709i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_APPLICATION_AUTHZ_INTERFACE_CLSID: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(800i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_APPLICATION_VERSION: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(801i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_MAX_APPLICATION_VERSION_LENGTH: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(512i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_APPLICATION_NAME: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(802i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_APPLICATION_BIZRULE_ENABLED: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(803i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_APPLY_STORE_SACL: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(900i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_GENERATE_AUDITS: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(901i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_POLICY_ADMINS: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(902i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_POLICY_READERS: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(903i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_DELEGATED_POLICY_USERS: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(904i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_POLICY_ADMINS_NAME: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(905i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_POLICY_READERS_NAME: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(906i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_PROP_DELEGATED_POLICY_USERS_NAME: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(907i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_CLIENT_CONTEXT_SKIP_GROUP: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(1i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_CLIENT_CONTEXT_SKIP_LDAP_QUERY: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(1i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_CLIENT_CONTEXT_GET_GROUP_RECURSIVE: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(2i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const AZ_CLIENT_CONTEXT_GET_GROUPS_STORE_LEVEL_ONLY: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(2i32);
impl ::core::marker::Copy for AZ_PROP_CONSTANTS {}
impl ::core::clone::Clone for AZ_PROP_CONSTANTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AZ_PROP_CONSTANTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for AZ_PROP_CONSTANTS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for AZ_PROP_CONSTANTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AZ_PROP_CONSTANTS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MULTIPLE_TRUSTEE_OPERATION(pub i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const NO_MULTIPLE_TRUSTEE: MULTIPLE_TRUSTEE_OPERATION = MULTIPLE_TRUSTEE_OPERATION(0i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const TRUSTEE_IS_IMPERSONATE: MULTIPLE_TRUSTEE_OPERATION = MULTIPLE_TRUSTEE_OPERATION(1i32);
impl ::core::marker::Copy for MULTIPLE_TRUSTEE_OPERATION {}
impl ::core::clone::Clone for MULTIPLE_TRUSTEE_OPERATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MULTIPLE_TRUSTEE_OPERATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MULTIPLE_TRUSTEE_OPERATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MULTIPLE_TRUSTEE_OPERATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MULTIPLE_TRUSTEE_OPERATION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROG_INVOKE_SETTING(pub i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ProgressInvokeNever: PROG_INVOKE_SETTING = PROG_INVOKE_SETTING(1i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ProgressInvokeEveryObject: PROG_INVOKE_SETTING = PROG_INVOKE_SETTING(2i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ProgressInvokeOnError: PROG_INVOKE_SETTING = PROG_INVOKE_SETTING(3i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ProgressCancelOperation: PROG_INVOKE_SETTING = PROG_INVOKE_SETTING(4i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ProgressRetryOperation: PROG_INVOKE_SETTING = PROG_INVOKE_SETTING(5i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const ProgressInvokePrePostError: PROG_INVOKE_SETTING = PROG_INVOKE_SETTING(6i32);
impl ::core::marker::Copy for PROG_INVOKE_SETTING {}
impl ::core::clone::Clone for PROG_INVOKE_SETTING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROG_INVOKE_SETTING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PROG_INVOKE_SETTING {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PROG_INVOKE_SETTING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROG_INVOKE_SETTING").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SE_OBJECT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SE_UNKNOWN_OBJECT_TYPE: SE_OBJECT_TYPE = SE_OBJECT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SE_FILE_OBJECT: SE_OBJECT_TYPE = SE_OBJECT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SE_SERVICE: SE_OBJECT_TYPE = SE_OBJECT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SE_PRINTER: SE_OBJECT_TYPE = SE_OBJECT_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SE_REGISTRY_KEY: SE_OBJECT_TYPE = SE_OBJECT_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SE_LMSHARE: SE_OBJECT_TYPE = SE_OBJECT_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SE_KERNEL_OBJECT: SE_OBJECT_TYPE = SE_OBJECT_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SE_WINDOW_OBJECT: SE_OBJECT_TYPE = SE_OBJECT_TYPE(7i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SE_DS_OBJECT: SE_OBJECT_TYPE = SE_OBJECT_TYPE(8i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SE_DS_OBJECT_ALL: SE_OBJECT_TYPE = SE_OBJECT_TYPE(9i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SE_PROVIDER_DEFINED_OBJECT: SE_OBJECT_TYPE = SE_OBJECT_TYPE(10i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SE_WMIGUID_OBJECT: SE_OBJECT_TYPE = SE_OBJECT_TYPE(11i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SE_REGISTRY_WOW64_32KEY: SE_OBJECT_TYPE = SE_OBJECT_TYPE(12i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const SE_REGISTRY_WOW64_64KEY: SE_OBJECT_TYPE = SE_OBJECT_TYPE(13i32);
impl ::core::marker::Copy for SE_OBJECT_TYPE {}
impl ::core::clone::Clone for SE_OBJECT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SE_OBJECT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SE_OBJECT_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SE_OBJECT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SE_OBJECT_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TREE_SEC_INFO(pub u32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const TREE_SEC_INFO_SET: TREE_SEC_INFO = TREE_SEC_INFO(1u32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const TREE_SEC_INFO_RESET: TREE_SEC_INFO = TREE_SEC_INFO(2u32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const TREE_SEC_INFO_RESET_KEEP_EXPLICIT: TREE_SEC_INFO = TREE_SEC_INFO(3u32);
impl ::core::marker::Copy for TREE_SEC_INFO {}
impl ::core::clone::Clone for TREE_SEC_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TREE_SEC_INFO {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for TREE_SEC_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for TREE_SEC_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TREE_SEC_INFO").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TRUSTEE_FORM(pub i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const TRUSTEE_IS_SID: TRUSTEE_FORM = TRUSTEE_FORM(0i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const TRUSTEE_IS_NAME: TRUSTEE_FORM = TRUSTEE_FORM(1i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const TRUSTEE_BAD_FORM: TRUSTEE_FORM = TRUSTEE_FORM(2i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const TRUSTEE_IS_OBJECTS_AND_SID: TRUSTEE_FORM = TRUSTEE_FORM(3i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const TRUSTEE_IS_OBJECTS_AND_NAME: TRUSTEE_FORM = TRUSTEE_FORM(4i32);
impl ::core::marker::Copy for TRUSTEE_FORM {}
impl ::core::clone::Clone for TRUSTEE_FORM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TRUSTEE_FORM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for TRUSTEE_FORM {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for TRUSTEE_FORM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TRUSTEE_FORM").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TRUSTEE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const TRUSTEE_IS_UNKNOWN: TRUSTEE_TYPE = TRUSTEE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const TRUSTEE_IS_USER: TRUSTEE_TYPE = TRUSTEE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const TRUSTEE_IS_GROUP: TRUSTEE_TYPE = TRUSTEE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const TRUSTEE_IS_DOMAIN: TRUSTEE_TYPE = TRUSTEE_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const TRUSTEE_IS_ALIAS: TRUSTEE_TYPE = TRUSTEE_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const TRUSTEE_IS_WELL_KNOWN_GROUP: TRUSTEE_TYPE = TRUSTEE_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const TRUSTEE_IS_DELETED: TRUSTEE_TYPE = TRUSTEE_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const TRUSTEE_IS_INVALID: TRUSTEE_TYPE = TRUSTEE_TYPE(7i32);
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub const TRUSTEE_IS_COMPUTER: TRUSTEE_TYPE = TRUSTEE_TYPE(8i32);
impl ::core::marker::Copy for TRUSTEE_TYPE {}
impl ::core::clone::Clone for TRUSTEE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TRUSTEE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for TRUSTEE_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for TRUSTEE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TRUSTEE_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct ACTRL_ACCESSA {
    pub cEntries: u32,
    pub pPropertyAccessList: *mut ACTRL_PROPERTY_ENTRYA,
}
impl ::core::marker::Copy for ACTRL_ACCESSA {}
impl ::core::clone::Clone for ACTRL_ACCESSA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACTRL_ACCESSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACTRL_ACCESSA").field("cEntries", &self.cEntries).field("pPropertyAccessList", &self.pPropertyAccessList).finish()
    }
}
impl ::windows::core::TypeKind for ACTRL_ACCESSA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for ACTRL_ACCESSA {
    fn eq(&self, other: &Self) -> bool {
        self.cEntries == other.cEntries && self.pPropertyAccessList == other.pPropertyAccessList
    }
}
impl ::core::cmp::Eq for ACTRL_ACCESSA {}
impl ::core::default::Default for ACTRL_ACCESSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct ACTRL_ACCESSW {
    pub cEntries: u32,
    pub pPropertyAccessList: *mut ACTRL_PROPERTY_ENTRYW,
}
impl ::core::marker::Copy for ACTRL_ACCESSW {}
impl ::core::clone::Clone for ACTRL_ACCESSW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACTRL_ACCESSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACTRL_ACCESSW").field("cEntries", &self.cEntries).field("pPropertyAccessList", &self.pPropertyAccessList).finish()
    }
}
impl ::windows::core::TypeKind for ACTRL_ACCESSW {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for ACTRL_ACCESSW {
    fn eq(&self, other: &Self) -> bool {
        self.cEntries == other.cEntries && self.pPropertyAccessList == other.pPropertyAccessList
    }
}
impl ::core::cmp::Eq for ACTRL_ACCESSW {}
impl ::core::default::Default for ACTRL_ACCESSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct ACTRL_ACCESS_ENTRYA {
    pub Trustee: TRUSTEE_A,
    pub fAccessFlags: ACTRL_ACCESS_ENTRY_ACCESS_FLAGS,
    pub Access: u32,
    pub ProvSpecificAccess: u32,
    pub Inheritance: super::ACE_FLAGS,
    pub lpInheritProperty: ::windows::core::PSTR,
}
impl ::core::marker::Copy for ACTRL_ACCESS_ENTRYA {}
impl ::core::clone::Clone for ACTRL_ACCESS_ENTRYA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACTRL_ACCESS_ENTRYA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACTRL_ACCESS_ENTRYA").field("Trustee", &self.Trustee).field("fAccessFlags", &self.fAccessFlags).field("Access", &self.Access).field("ProvSpecificAccess", &self.ProvSpecificAccess).field("Inheritance", &self.Inheritance).field("lpInheritProperty", &self.lpInheritProperty).finish()
    }
}
impl ::windows::core::TypeKind for ACTRL_ACCESS_ENTRYA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for ACTRL_ACCESS_ENTRYA {
    fn eq(&self, other: &Self) -> bool {
        self.Trustee == other.Trustee && self.fAccessFlags == other.fAccessFlags && self.Access == other.Access && self.ProvSpecificAccess == other.ProvSpecificAccess && self.Inheritance == other.Inheritance && self.lpInheritProperty == other.lpInheritProperty
    }
}
impl ::core::cmp::Eq for ACTRL_ACCESS_ENTRYA {}
impl ::core::default::Default for ACTRL_ACCESS_ENTRYA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct ACTRL_ACCESS_ENTRYW {
    pub Trustee: TRUSTEE_W,
    pub fAccessFlags: ACTRL_ACCESS_ENTRY_ACCESS_FLAGS,
    pub Access: u32,
    pub ProvSpecificAccess: u32,
    pub Inheritance: super::ACE_FLAGS,
    pub lpInheritProperty: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for ACTRL_ACCESS_ENTRYW {}
impl ::core::clone::Clone for ACTRL_ACCESS_ENTRYW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACTRL_ACCESS_ENTRYW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACTRL_ACCESS_ENTRYW").field("Trustee", &self.Trustee).field("fAccessFlags", &self.fAccessFlags).field("Access", &self.Access).field("ProvSpecificAccess", &self.ProvSpecificAccess).field("Inheritance", &self.Inheritance).field("lpInheritProperty", &self.lpInheritProperty).finish()
    }
}
impl ::windows::core::TypeKind for ACTRL_ACCESS_ENTRYW {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for ACTRL_ACCESS_ENTRYW {
    fn eq(&self, other: &Self) -> bool {
        self.Trustee == other.Trustee && self.fAccessFlags == other.fAccessFlags && self.Access == other.Access && self.ProvSpecificAccess == other.ProvSpecificAccess && self.Inheritance == other.Inheritance && self.lpInheritProperty == other.lpInheritProperty
    }
}
impl ::core::cmp::Eq for ACTRL_ACCESS_ENTRYW {}
impl ::core::default::Default for ACTRL_ACCESS_ENTRYW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct ACTRL_ACCESS_ENTRY_LISTA {
    pub cEntries: u32,
    pub pAccessList: *mut ACTRL_ACCESS_ENTRYA,
}
impl ::core::marker::Copy for ACTRL_ACCESS_ENTRY_LISTA {}
impl ::core::clone::Clone for ACTRL_ACCESS_ENTRY_LISTA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACTRL_ACCESS_ENTRY_LISTA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACTRL_ACCESS_ENTRY_LISTA").field("cEntries", &self.cEntries).field("pAccessList", &self.pAccessList).finish()
    }
}
impl ::windows::core::TypeKind for ACTRL_ACCESS_ENTRY_LISTA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for ACTRL_ACCESS_ENTRY_LISTA {
    fn eq(&self, other: &Self) -> bool {
        self.cEntries == other.cEntries && self.pAccessList == other.pAccessList
    }
}
impl ::core::cmp::Eq for ACTRL_ACCESS_ENTRY_LISTA {}
impl ::core::default::Default for ACTRL_ACCESS_ENTRY_LISTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct ACTRL_ACCESS_ENTRY_LISTW {
    pub cEntries: u32,
    pub pAccessList: *mut ACTRL_ACCESS_ENTRYW,
}
impl ::core::marker::Copy for ACTRL_ACCESS_ENTRY_LISTW {}
impl ::core::clone::Clone for ACTRL_ACCESS_ENTRY_LISTW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACTRL_ACCESS_ENTRY_LISTW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACTRL_ACCESS_ENTRY_LISTW").field("cEntries", &self.cEntries).field("pAccessList", &self.pAccessList).finish()
    }
}
impl ::windows::core::TypeKind for ACTRL_ACCESS_ENTRY_LISTW {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for ACTRL_ACCESS_ENTRY_LISTW {
    fn eq(&self, other: &Self) -> bool {
        self.cEntries == other.cEntries && self.pAccessList == other.pAccessList
    }
}
impl ::core::cmp::Eq for ACTRL_ACCESS_ENTRY_LISTW {}
impl ::core::default::Default for ACTRL_ACCESS_ENTRY_LISTW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct ACTRL_ACCESS_INFOA {
    pub fAccessPermission: u32,
    pub lpAccessPermissionName: ::windows::core::PSTR,
}
impl ::core::marker::Copy for ACTRL_ACCESS_INFOA {}
impl ::core::clone::Clone for ACTRL_ACCESS_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACTRL_ACCESS_INFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACTRL_ACCESS_INFOA").field("fAccessPermission", &self.fAccessPermission).field("lpAccessPermissionName", &self.lpAccessPermissionName).finish()
    }
}
impl ::windows::core::TypeKind for ACTRL_ACCESS_INFOA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for ACTRL_ACCESS_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.fAccessPermission == other.fAccessPermission && self.lpAccessPermissionName == other.lpAccessPermissionName
    }
}
impl ::core::cmp::Eq for ACTRL_ACCESS_INFOA {}
impl ::core::default::Default for ACTRL_ACCESS_INFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct ACTRL_ACCESS_INFOW {
    pub fAccessPermission: u32,
    pub lpAccessPermissionName: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for ACTRL_ACCESS_INFOW {}
impl ::core::clone::Clone for ACTRL_ACCESS_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACTRL_ACCESS_INFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACTRL_ACCESS_INFOW").field("fAccessPermission", &self.fAccessPermission).field("lpAccessPermissionName", &self.lpAccessPermissionName).finish()
    }
}
impl ::windows::core::TypeKind for ACTRL_ACCESS_INFOW {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for ACTRL_ACCESS_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.fAccessPermission == other.fAccessPermission && self.lpAccessPermissionName == other.lpAccessPermissionName
    }
}
impl ::core::cmp::Eq for ACTRL_ACCESS_INFOW {}
impl ::core::default::Default for ACTRL_ACCESS_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct ACTRL_CONTROL_INFOA {
    pub lpControlId: ::windows::core::PSTR,
    pub lpControlName: ::windows::core::PSTR,
}
impl ::core::marker::Copy for ACTRL_CONTROL_INFOA {}
impl ::core::clone::Clone for ACTRL_CONTROL_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACTRL_CONTROL_INFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACTRL_CONTROL_INFOA").field("lpControlId", &self.lpControlId).field("lpControlName", &self.lpControlName).finish()
    }
}
impl ::windows::core::TypeKind for ACTRL_CONTROL_INFOA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for ACTRL_CONTROL_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.lpControlId == other.lpControlId && self.lpControlName == other.lpControlName
    }
}
impl ::core::cmp::Eq for ACTRL_CONTROL_INFOA {}
impl ::core::default::Default for ACTRL_CONTROL_INFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct ACTRL_CONTROL_INFOW {
    pub lpControlId: ::windows::core::PWSTR,
    pub lpControlName: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for ACTRL_CONTROL_INFOW {}
impl ::core::clone::Clone for ACTRL_CONTROL_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACTRL_CONTROL_INFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACTRL_CONTROL_INFOW").field("lpControlId", &self.lpControlId).field("lpControlName", &self.lpControlName).finish()
    }
}
impl ::windows::core::TypeKind for ACTRL_CONTROL_INFOW {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for ACTRL_CONTROL_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.lpControlId == other.lpControlId && self.lpControlName == other.lpControlName
    }
}
impl ::core::cmp::Eq for ACTRL_CONTROL_INFOW {}
impl ::core::default::Default for ACTRL_CONTROL_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ACTRL_OVERLAPPED {
    pub Anonymous: ACTRL_OVERLAPPED_0,
    pub Reserved2: u32,
    pub hEvent: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ACTRL_OVERLAPPED {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ACTRL_OVERLAPPED {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for ACTRL_OVERLAPPED {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ACTRL_OVERLAPPED {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union ACTRL_OVERLAPPED_0 {
    pub Provider: *mut ::core::ffi::c_void,
    pub Reserved1: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ACTRL_OVERLAPPED_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ACTRL_OVERLAPPED_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for ACTRL_OVERLAPPED_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ACTRL_OVERLAPPED_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct ACTRL_PROPERTY_ENTRYA {
    pub lpProperty: ::windows::core::PSTR,
    pub pAccessEntryList: *mut ACTRL_ACCESS_ENTRY_LISTA,
    pub fListFlags: u32,
}
impl ::core::marker::Copy for ACTRL_PROPERTY_ENTRYA {}
impl ::core::clone::Clone for ACTRL_PROPERTY_ENTRYA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACTRL_PROPERTY_ENTRYA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACTRL_PROPERTY_ENTRYA").field("lpProperty", &self.lpProperty).field("pAccessEntryList", &self.pAccessEntryList).field("fListFlags", &self.fListFlags).finish()
    }
}
impl ::windows::core::TypeKind for ACTRL_PROPERTY_ENTRYA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for ACTRL_PROPERTY_ENTRYA {
    fn eq(&self, other: &Self) -> bool {
        self.lpProperty == other.lpProperty && self.pAccessEntryList == other.pAccessEntryList && self.fListFlags == other.fListFlags
    }
}
impl ::core::cmp::Eq for ACTRL_PROPERTY_ENTRYA {}
impl ::core::default::Default for ACTRL_PROPERTY_ENTRYA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct ACTRL_PROPERTY_ENTRYW {
    pub lpProperty: ::windows::core::PWSTR,
    pub pAccessEntryList: *mut ACTRL_ACCESS_ENTRY_LISTW,
    pub fListFlags: u32,
}
impl ::core::marker::Copy for ACTRL_PROPERTY_ENTRYW {}
impl ::core::clone::Clone for ACTRL_PROPERTY_ENTRYW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACTRL_PROPERTY_ENTRYW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACTRL_PROPERTY_ENTRYW").field("lpProperty", &self.lpProperty).field("pAccessEntryList", &self.pAccessEntryList).field("fListFlags", &self.fListFlags).finish()
    }
}
impl ::windows::core::TypeKind for ACTRL_PROPERTY_ENTRYW {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for ACTRL_PROPERTY_ENTRYW {
    fn eq(&self, other: &Self) -> bool {
        self.lpProperty == other.lpProperty && self.pAccessEntryList == other.pAccessEntryList && self.fListFlags == other.fListFlags
    }
}
impl ::core::cmp::Eq for ACTRL_PROPERTY_ENTRYW {}
impl ::core::default::Default for ACTRL_PROPERTY_ENTRYW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct AUDIT_IP_ADDRESS {
    pub pIpAddress: [u8; 128],
}
impl ::core::marker::Copy for AUDIT_IP_ADDRESS {}
impl ::core::clone::Clone for AUDIT_IP_ADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AUDIT_IP_ADDRESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUDIT_IP_ADDRESS").field("pIpAddress", &self.pIpAddress).finish()
    }
}
impl ::windows::core::TypeKind for AUDIT_IP_ADDRESS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for AUDIT_IP_ADDRESS {
    fn eq(&self, other: &Self) -> bool {
        self.pIpAddress == other.pIpAddress
    }
}
impl ::core::cmp::Eq for AUDIT_IP_ADDRESS {}
impl ::core::default::Default for AUDIT_IP_ADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct AUDIT_OBJECT_TYPE {
    pub ObjectType: ::windows::core::GUID,
    pub Flags: u16,
    pub Level: u16,
    pub AccessMask: u32,
}
impl ::core::marker::Copy for AUDIT_OBJECT_TYPE {}
impl ::core::clone::Clone for AUDIT_OBJECT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AUDIT_OBJECT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUDIT_OBJECT_TYPE").field("ObjectType", &self.ObjectType).field("Flags", &self.Flags).field("Level", &self.Level).field("AccessMask", &self.AccessMask).finish()
    }
}
impl ::windows::core::TypeKind for AUDIT_OBJECT_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for AUDIT_OBJECT_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.ObjectType == other.ObjectType && self.Flags == other.Flags && self.Level == other.Level && self.AccessMask == other.AccessMask
    }
}
impl ::core::cmp::Eq for AUDIT_OBJECT_TYPE {}
impl ::core::default::Default for AUDIT_OBJECT_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct AUDIT_OBJECT_TYPES {
    pub Count: u16,
    pub Flags: u16,
    pub pObjectTypes: *mut AUDIT_OBJECT_TYPE,
}
impl ::core::marker::Copy for AUDIT_OBJECT_TYPES {}
impl ::core::clone::Clone for AUDIT_OBJECT_TYPES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AUDIT_OBJECT_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUDIT_OBJECT_TYPES").field("Count", &self.Count).field("Flags", &self.Flags).field("pObjectTypes", &self.pObjectTypes).finish()
    }
}
impl ::windows::core::TypeKind for AUDIT_OBJECT_TYPES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for AUDIT_OBJECT_TYPES {
    fn eq(&self, other: &Self) -> bool {
        self.Count == other.Count && self.Flags == other.Flags && self.pObjectTypes == other.pObjectTypes
    }
}
impl ::core::cmp::Eq for AUDIT_OBJECT_TYPES {}
impl ::core::default::Default for AUDIT_OBJECT_TYPES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct AUDIT_PARAM {
    pub Type: AUDIT_PARAM_TYPE,
    pub Length: u32,
    pub Flags: u32,
    pub Anonymous1: AUDIT_PARAM_0,
    pub Anonymous2: AUDIT_PARAM_1,
}
impl ::core::marker::Copy for AUDIT_PARAM {}
impl ::core::clone::Clone for AUDIT_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for AUDIT_PARAM {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for AUDIT_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub union AUDIT_PARAM_0 {
    pub Data0: usize,
    pub String: ::windows::core::PWSTR,
    pub u: usize,
    pub psid: *mut super::SID,
    pub pguid: *mut ::windows::core::GUID,
    pub LogonId_LowPart: u32,
    pub pObjectTypes: *mut AUDIT_OBJECT_TYPES,
    pub pIpAddress: *mut AUDIT_IP_ADDRESS,
}
impl ::core::marker::Copy for AUDIT_PARAM_0 {}
impl ::core::clone::Clone for AUDIT_PARAM_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for AUDIT_PARAM_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for AUDIT_PARAM_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub union AUDIT_PARAM_1 {
    pub Data1: usize,
    pub LogonId_HighPart: i32,
}
impl ::core::marker::Copy for AUDIT_PARAM_1 {}
impl ::core::clone::Clone for AUDIT_PARAM_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for AUDIT_PARAM_1 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for AUDIT_PARAM_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct AUDIT_PARAMS {
    pub Length: u32,
    pub Flags: u32,
    pub Count: u16,
    pub Parameters: *mut AUDIT_PARAM,
}
impl ::core::marker::Copy for AUDIT_PARAMS {}
impl ::core::clone::Clone for AUDIT_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AUDIT_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUDIT_PARAMS").field("Length", &self.Length).field("Flags", &self.Flags).field("Count", &self.Count).field("Parameters", &self.Parameters).finish()
    }
}
impl ::windows::core::TypeKind for AUDIT_PARAMS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for AUDIT_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.Flags == other.Flags && self.Count == other.Count && self.Parameters == other.Parameters
    }
}
impl ::core::cmp::Eq for AUDIT_PARAMS {}
impl ::core::default::Default for AUDIT_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AUTHZ_ACCESS_CHECK_RESULTS_HANDLE(pub isize);
impl AUTHZ_ACCESS_CHECK_RESULTS_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for AUTHZ_ACCESS_CHECK_RESULTS_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for AUTHZ_ACCESS_CHECK_RESULTS_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for AUTHZ_ACCESS_CHECK_RESULTS_HANDLE {}
impl ::core::fmt::Debug for AUTHZ_ACCESS_CHECK_RESULTS_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUTHZ_ACCESS_CHECK_RESULTS_HANDLE").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for AUTHZ_ACCESS_CHECK_RESULTS_HANDLE {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct AUTHZ_ACCESS_REPLY {
    pub ResultListLength: u32,
    pub GrantedAccessMask: *mut u32,
    pub SaclEvaluationResults: *mut AUTHZ_GENERATE_RESULTS,
    pub Error: *mut u32,
}
impl ::core::marker::Copy for AUTHZ_ACCESS_REPLY {}
impl ::core::clone::Clone for AUTHZ_ACCESS_REPLY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AUTHZ_ACCESS_REPLY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUTHZ_ACCESS_REPLY").field("ResultListLength", &self.ResultListLength).field("GrantedAccessMask", &self.GrantedAccessMask).field("SaclEvaluationResults", &self.SaclEvaluationResults).field("Error", &self.Error).finish()
    }
}
impl ::windows::core::TypeKind for AUTHZ_ACCESS_REPLY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for AUTHZ_ACCESS_REPLY {
    fn eq(&self, other: &Self) -> bool {
        self.ResultListLength == other.ResultListLength && self.GrantedAccessMask == other.GrantedAccessMask && self.SaclEvaluationResults == other.SaclEvaluationResults && self.Error == other.Error
    }
}
impl ::core::cmp::Eq for AUTHZ_ACCESS_REPLY {}
impl ::core::default::Default for AUTHZ_ACCESS_REPLY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct AUTHZ_ACCESS_REQUEST {
    pub DesiredAccess: u32,
    pub PrincipalSelfSid: super::super::Foundation::PSID,
    pub ObjectTypeList: *mut super::OBJECT_TYPE_LIST,
    pub ObjectTypeListLength: u32,
    pub OptionalArguments: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for AUTHZ_ACCESS_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for AUTHZ_ACCESS_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for AUTHZ_ACCESS_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUTHZ_ACCESS_REQUEST").field("DesiredAccess", &self.DesiredAccess).field("PrincipalSelfSid", &self.PrincipalSelfSid).field("ObjectTypeList", &self.ObjectTypeList).field("ObjectTypeListLength", &self.ObjectTypeListLength).field("OptionalArguments", &self.OptionalArguments).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for AUTHZ_ACCESS_REQUEST {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for AUTHZ_ACCESS_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.DesiredAccess == other.DesiredAccess && self.PrincipalSelfSid == other.PrincipalSelfSid && self.ObjectTypeList == other.ObjectTypeList && self.ObjectTypeListLength == other.ObjectTypeListLength && self.OptionalArguments == other.OptionalArguments
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for AUTHZ_ACCESS_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for AUTHZ_ACCESS_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AUTHZ_AUDIT_EVENT_HANDLE(pub isize);
impl AUTHZ_AUDIT_EVENT_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for AUTHZ_AUDIT_EVENT_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for AUTHZ_AUDIT_EVENT_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for AUTHZ_AUDIT_EVENT_HANDLE {}
impl ::core::fmt::Debug for AUTHZ_AUDIT_EVENT_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUTHZ_AUDIT_EVENT_HANDLE").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for AUTHZ_AUDIT_EVENT_HANDLE {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AUTHZ_AUDIT_EVENT_TYPE_HANDLE(pub isize);
impl AUTHZ_AUDIT_EVENT_TYPE_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for AUTHZ_AUDIT_EVENT_TYPE_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for AUTHZ_AUDIT_EVENT_TYPE_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for AUTHZ_AUDIT_EVENT_TYPE_HANDLE {}
impl ::core::fmt::Debug for AUTHZ_AUDIT_EVENT_TYPE_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUTHZ_AUDIT_EVENT_TYPE_HANDLE").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for AUTHZ_AUDIT_EVENT_TYPE_HANDLE {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct AUTHZ_AUDIT_EVENT_TYPE_LEGACY {
    pub CategoryId: u16,
    pub AuditId: u16,
    pub ParameterCount: u16,
}
impl ::core::marker::Copy for AUTHZ_AUDIT_EVENT_TYPE_LEGACY {}
impl ::core::clone::Clone for AUTHZ_AUDIT_EVENT_TYPE_LEGACY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AUTHZ_AUDIT_EVENT_TYPE_LEGACY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUTHZ_AUDIT_EVENT_TYPE_LEGACY").field("CategoryId", &self.CategoryId).field("AuditId", &self.AuditId).field("ParameterCount", &self.ParameterCount).finish()
    }
}
impl ::windows::core::TypeKind for AUTHZ_AUDIT_EVENT_TYPE_LEGACY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for AUTHZ_AUDIT_EVENT_TYPE_LEGACY {
    fn eq(&self, other: &Self) -> bool {
        self.CategoryId == other.CategoryId && self.AuditId == other.AuditId && self.ParameterCount == other.ParameterCount
    }
}
impl ::core::cmp::Eq for AUTHZ_AUDIT_EVENT_TYPE_LEGACY {}
impl ::core::default::Default for AUTHZ_AUDIT_EVENT_TYPE_LEGACY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct AUTHZ_AUDIT_EVENT_TYPE_OLD {
    pub Version: u32,
    pub dwFlags: u32,
    pub RefCount: i32,
    pub hAudit: usize,
    pub LinkId: super::super::Foundation::LUID,
    pub u: AUTHZ_AUDIT_EVENT_TYPE_UNION,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for AUTHZ_AUDIT_EVENT_TYPE_OLD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for AUTHZ_AUDIT_EVENT_TYPE_OLD {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for AUTHZ_AUDIT_EVENT_TYPE_OLD {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for AUTHZ_AUDIT_EVENT_TYPE_OLD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub union AUTHZ_AUDIT_EVENT_TYPE_UNION {
    pub Legacy: AUTHZ_AUDIT_EVENT_TYPE_LEGACY,
}
impl ::core::marker::Copy for AUTHZ_AUDIT_EVENT_TYPE_UNION {}
impl ::core::clone::Clone for AUTHZ_AUDIT_EVENT_TYPE_UNION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for AUTHZ_AUDIT_EVENT_TYPE_UNION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for AUTHZ_AUDIT_EVENT_TYPE_UNION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct AUTHZ_CAP_CHANGE_SUBSCRIPTION_HANDLE__ {
    pub unused: i32,
}
impl ::core::marker::Copy for AUTHZ_CAP_CHANGE_SUBSCRIPTION_HANDLE__ {}
impl ::core::clone::Clone for AUTHZ_CAP_CHANGE_SUBSCRIPTION_HANDLE__ {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AUTHZ_CAP_CHANGE_SUBSCRIPTION_HANDLE__ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUTHZ_CAP_CHANGE_SUBSCRIPTION_HANDLE__").field("unused", &self.unused).finish()
    }
}
impl ::windows::core::TypeKind for AUTHZ_CAP_CHANGE_SUBSCRIPTION_HANDLE__ {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for AUTHZ_CAP_CHANGE_SUBSCRIPTION_HANDLE__ {
    fn eq(&self, other: &Self) -> bool {
        self.unused == other.unused
    }
}
impl ::core::cmp::Eq for AUTHZ_CAP_CHANGE_SUBSCRIPTION_HANDLE__ {}
impl ::core::default::Default for AUTHZ_CAP_CHANGE_SUBSCRIPTION_HANDLE__ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AUTHZ_CLIENT_CONTEXT_HANDLE(pub isize);
impl AUTHZ_CLIENT_CONTEXT_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for AUTHZ_CLIENT_CONTEXT_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for AUTHZ_CLIENT_CONTEXT_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for AUTHZ_CLIENT_CONTEXT_HANDLE {}
impl ::core::fmt::Debug for AUTHZ_CLIENT_CONTEXT_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUTHZ_CLIENT_CONTEXT_HANDLE").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for AUTHZ_CLIENT_CONTEXT_HANDLE {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct AUTHZ_INIT_INFO {
    pub version: u16,
    pub szResourceManagerName: ::windows::core::PCWSTR,
    pub pfnDynamicAccessCheck: PFN_AUTHZ_DYNAMIC_ACCESS_CHECK,
    pub pfnComputeDynamicGroups: PFN_AUTHZ_COMPUTE_DYNAMIC_GROUPS,
    pub pfnFreeDynamicGroups: PFN_AUTHZ_FREE_DYNAMIC_GROUPS,
    pub pfnGetCentralAccessPolicy: PFN_AUTHZ_GET_CENTRAL_ACCESS_POLICY,
    pub pfnFreeCentralAccessPolicy: PFN_AUTHZ_FREE_CENTRAL_ACCESS_POLICY,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for AUTHZ_INIT_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for AUTHZ_INIT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for AUTHZ_INIT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUTHZ_INIT_INFO").field("version", &self.version).field("szResourceManagerName", &self.szResourceManagerName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for AUTHZ_INIT_INFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for AUTHZ_INIT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct AUTHZ_REGISTRATION_OBJECT_TYPE_NAME_OFFSET {
    pub szObjectTypeName: ::windows::core::PWSTR,
    pub dwOffset: u32,
}
impl ::core::marker::Copy for AUTHZ_REGISTRATION_OBJECT_TYPE_NAME_OFFSET {}
impl ::core::clone::Clone for AUTHZ_REGISTRATION_OBJECT_TYPE_NAME_OFFSET {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AUTHZ_REGISTRATION_OBJECT_TYPE_NAME_OFFSET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUTHZ_REGISTRATION_OBJECT_TYPE_NAME_OFFSET").field("szObjectTypeName", &self.szObjectTypeName).field("dwOffset", &self.dwOffset).finish()
    }
}
impl ::windows::core::TypeKind for AUTHZ_REGISTRATION_OBJECT_TYPE_NAME_OFFSET {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for AUTHZ_REGISTRATION_OBJECT_TYPE_NAME_OFFSET {
    fn eq(&self, other: &Self) -> bool {
        self.szObjectTypeName == other.szObjectTypeName && self.dwOffset == other.dwOffset
    }
}
impl ::core::cmp::Eq for AUTHZ_REGISTRATION_OBJECT_TYPE_NAME_OFFSET {}
impl ::core::default::Default for AUTHZ_REGISTRATION_OBJECT_TYPE_NAME_OFFSET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AUTHZ_RESOURCE_MANAGER_HANDLE(pub isize);
impl AUTHZ_RESOURCE_MANAGER_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for AUTHZ_RESOURCE_MANAGER_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for AUTHZ_RESOURCE_MANAGER_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for AUTHZ_RESOURCE_MANAGER_HANDLE {}
impl ::core::fmt::Debug for AUTHZ_RESOURCE_MANAGER_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUTHZ_RESOURCE_MANAGER_HANDLE").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for AUTHZ_RESOURCE_MANAGER_HANDLE {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct AUTHZ_RPC_INIT_INFO_CLIENT {
    pub version: u16,
    pub ObjectUuid: ::windows::core::PWSTR,
    pub ProtSeq: ::windows::core::PWSTR,
    pub NetworkAddr: ::windows::core::PWSTR,
    pub Endpoint: ::windows::core::PWSTR,
    pub Options: ::windows::core::PWSTR,
    pub ServerSpn: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for AUTHZ_RPC_INIT_INFO_CLIENT {}
impl ::core::clone::Clone for AUTHZ_RPC_INIT_INFO_CLIENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AUTHZ_RPC_INIT_INFO_CLIENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUTHZ_RPC_INIT_INFO_CLIENT").field("version", &self.version).field("ObjectUuid", &self.ObjectUuid).field("ProtSeq", &self.ProtSeq).field("NetworkAddr", &self.NetworkAddr).field("Endpoint", &self.Endpoint).field("Options", &self.Options).field("ServerSpn", &self.ServerSpn).finish()
    }
}
impl ::windows::core::TypeKind for AUTHZ_RPC_INIT_INFO_CLIENT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for AUTHZ_RPC_INIT_INFO_CLIENT {
    fn eq(&self, other: &Self) -> bool {
        self.version == other.version && self.ObjectUuid == other.ObjectUuid && self.ProtSeq == other.ProtSeq && self.NetworkAddr == other.NetworkAddr && self.Endpoint == other.Endpoint && self.Options == other.Options && self.ServerSpn == other.ServerSpn
    }
}
impl ::core::cmp::Eq for AUTHZ_RPC_INIT_INFO_CLIENT {}
impl ::core::default::Default for AUTHZ_RPC_INIT_INFO_CLIENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct AUTHZ_SECURITY_ATTRIBUTES_INFORMATION {
    pub Version: u16,
    pub Reserved: u16,
    pub AttributeCount: u32,
    pub Attribute: AUTHZ_SECURITY_ATTRIBUTES_INFORMATION_0,
}
impl ::core::marker::Copy for AUTHZ_SECURITY_ATTRIBUTES_INFORMATION {}
impl ::core::clone::Clone for AUTHZ_SECURITY_ATTRIBUTES_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for AUTHZ_SECURITY_ATTRIBUTES_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for AUTHZ_SECURITY_ATTRIBUTES_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub union AUTHZ_SECURITY_ATTRIBUTES_INFORMATION_0 {
    pub pAttributeV1: *mut AUTHZ_SECURITY_ATTRIBUTE_V1,
}
impl ::core::marker::Copy for AUTHZ_SECURITY_ATTRIBUTES_INFORMATION_0 {}
impl ::core::clone::Clone for AUTHZ_SECURITY_ATTRIBUTES_INFORMATION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for AUTHZ_SECURITY_ATTRIBUTES_INFORMATION_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for AUTHZ_SECURITY_ATTRIBUTES_INFORMATION_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct AUTHZ_SECURITY_ATTRIBUTE_FQBN_VALUE {
    pub Version: u64,
    pub pName: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for AUTHZ_SECURITY_ATTRIBUTE_FQBN_VALUE {}
impl ::core::clone::Clone for AUTHZ_SECURITY_ATTRIBUTE_FQBN_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AUTHZ_SECURITY_ATTRIBUTE_FQBN_VALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUTHZ_SECURITY_ATTRIBUTE_FQBN_VALUE").field("Version", &self.Version).field("pName", &self.pName).finish()
    }
}
impl ::windows::core::TypeKind for AUTHZ_SECURITY_ATTRIBUTE_FQBN_VALUE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for AUTHZ_SECURITY_ATTRIBUTE_FQBN_VALUE {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.pName == other.pName
    }
}
impl ::core::cmp::Eq for AUTHZ_SECURITY_ATTRIBUTE_FQBN_VALUE {}
impl ::core::default::Default for AUTHZ_SECURITY_ATTRIBUTE_FQBN_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct AUTHZ_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {
    pub pValue: *mut ::core::ffi::c_void,
    pub ValueLength: u32,
}
impl ::core::marker::Copy for AUTHZ_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {}
impl ::core::clone::Clone for AUTHZ_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AUTHZ_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUTHZ_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE").field("pValue", &self.pValue).field("ValueLength", &self.ValueLength).finish()
    }
}
impl ::windows::core::TypeKind for AUTHZ_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for AUTHZ_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {
    fn eq(&self, other: &Self) -> bool {
        self.pValue == other.pValue && self.ValueLength == other.ValueLength
    }
}
impl ::core::cmp::Eq for AUTHZ_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {}
impl ::core::default::Default for AUTHZ_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct AUTHZ_SECURITY_ATTRIBUTE_V1 {
    pub pName: ::windows::core::PWSTR,
    pub ValueType: u16,
    pub Reserved: u16,
    pub Flags: AUTHZ_SECURITY_ATTRIBUTE_FLAGS,
    pub ValueCount: u32,
    pub Values: AUTHZ_SECURITY_ATTRIBUTE_V1_0,
}
impl ::core::marker::Copy for AUTHZ_SECURITY_ATTRIBUTE_V1 {}
impl ::core::clone::Clone for AUTHZ_SECURITY_ATTRIBUTE_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for AUTHZ_SECURITY_ATTRIBUTE_V1 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for AUTHZ_SECURITY_ATTRIBUTE_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub union AUTHZ_SECURITY_ATTRIBUTE_V1_0 {
    pub pInt64: *mut i64,
    pub pUint64: *mut u64,
    pub ppString: *mut ::windows::core::PWSTR,
    pub pFqbn: *mut AUTHZ_SECURITY_ATTRIBUTE_FQBN_VALUE,
    pub pOctetString: *mut AUTHZ_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE,
}
impl ::core::marker::Copy for AUTHZ_SECURITY_ATTRIBUTE_V1_0 {}
impl ::core::clone::Clone for AUTHZ_SECURITY_ATTRIBUTE_V1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for AUTHZ_SECURITY_ATTRIBUTE_V1_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for AUTHZ_SECURITY_ATTRIBUTE_V1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE(pub isize);
impl AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE {}
impl ::core::fmt::Debug for AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct AUTHZ_SOURCE_SCHEMA_REGISTRATION {
    pub dwFlags: u32,
    pub szEventSourceName: ::windows::core::PWSTR,
    pub szEventMessageFile: ::windows::core::PWSTR,
    pub szEventSourceXmlSchemaFile: ::windows::core::PWSTR,
    pub szEventAccessStringsFile: ::windows::core::PWSTR,
    pub szExecutableImagePath: ::windows::core::PWSTR,
    pub Anonymous: AUTHZ_SOURCE_SCHEMA_REGISTRATION_0,
    pub dwObjectTypeNameCount: u32,
    pub ObjectTypeNames: [AUTHZ_REGISTRATION_OBJECT_TYPE_NAME_OFFSET; 1],
}
impl ::core::marker::Copy for AUTHZ_SOURCE_SCHEMA_REGISTRATION {}
impl ::core::clone::Clone for AUTHZ_SOURCE_SCHEMA_REGISTRATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for AUTHZ_SOURCE_SCHEMA_REGISTRATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for AUTHZ_SOURCE_SCHEMA_REGISTRATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub union AUTHZ_SOURCE_SCHEMA_REGISTRATION_0 {
    pub pReserved: *mut ::core::ffi::c_void,
    pub pProviderGuid: *mut ::windows::core::GUID,
}
impl ::core::marker::Copy for AUTHZ_SOURCE_SCHEMA_REGISTRATION_0 {}
impl ::core::clone::Clone for AUTHZ_SOURCE_SCHEMA_REGISTRATION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for AUTHZ_SOURCE_SCHEMA_REGISTRATION_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for AUTHZ_SOURCE_SCHEMA_REGISTRATION_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct EXPLICIT_ACCESS_A {
    pub grfAccessPermissions: u32,
    pub grfAccessMode: ACCESS_MODE,
    pub grfInheritance: super::ACE_FLAGS,
    pub Trustee: TRUSTEE_A,
}
impl ::core::marker::Copy for EXPLICIT_ACCESS_A {}
impl ::core::clone::Clone for EXPLICIT_ACCESS_A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EXPLICIT_ACCESS_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EXPLICIT_ACCESS_A").field("grfAccessPermissions", &self.grfAccessPermissions).field("grfAccessMode", &self.grfAccessMode).field("grfInheritance", &self.grfInheritance).field("Trustee", &self.Trustee).finish()
    }
}
impl ::windows::core::TypeKind for EXPLICIT_ACCESS_A {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for EXPLICIT_ACCESS_A {
    fn eq(&self, other: &Self) -> bool {
        self.grfAccessPermissions == other.grfAccessPermissions && self.grfAccessMode == other.grfAccessMode && self.grfInheritance == other.grfInheritance && self.Trustee == other.Trustee
    }
}
impl ::core::cmp::Eq for EXPLICIT_ACCESS_A {}
impl ::core::default::Default for EXPLICIT_ACCESS_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct EXPLICIT_ACCESS_W {
    pub grfAccessPermissions: u32,
    pub grfAccessMode: ACCESS_MODE,
    pub grfInheritance: super::ACE_FLAGS,
    pub Trustee: TRUSTEE_W,
}
impl ::core::marker::Copy for EXPLICIT_ACCESS_W {}
impl ::core::clone::Clone for EXPLICIT_ACCESS_W {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EXPLICIT_ACCESS_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EXPLICIT_ACCESS_W").field("grfAccessPermissions", &self.grfAccessPermissions).field("grfAccessMode", &self.grfAccessMode).field("grfInheritance", &self.grfInheritance).field("Trustee", &self.Trustee).finish()
    }
}
impl ::windows::core::TypeKind for EXPLICIT_ACCESS_W {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for EXPLICIT_ACCESS_W {
    fn eq(&self, other: &Self) -> bool {
        self.grfAccessPermissions == other.grfAccessPermissions && self.grfAccessMode == other.grfAccessMode && self.grfInheritance == other.grfInheritance && self.Trustee == other.Trustee
    }
}
impl ::core::cmp::Eq for EXPLICIT_ACCESS_W {}
impl ::core::default::Default for EXPLICIT_ACCESS_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct FN_OBJECT_MGR_FUNCTS {
    pub Placeholder: u32,
}
impl ::core::marker::Copy for FN_OBJECT_MGR_FUNCTS {}
impl ::core::clone::Clone for FN_OBJECT_MGR_FUNCTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FN_OBJECT_MGR_FUNCTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FN_OBJECT_MGR_FUNCTS").field("Placeholder", &self.Placeholder).finish()
    }
}
impl ::windows::core::TypeKind for FN_OBJECT_MGR_FUNCTS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for FN_OBJECT_MGR_FUNCTS {
    fn eq(&self, other: &Self) -> bool {
        self.Placeholder == other.Placeholder
    }
}
impl ::core::cmp::Eq for FN_OBJECT_MGR_FUNCTS {}
impl ::core::default::Default for FN_OBJECT_MGR_FUNCTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct INHERITED_FROMA {
    pub GenerationGap: i32,
    pub AncestorName: ::windows::core::PSTR,
}
impl ::core::marker::Copy for INHERITED_FROMA {}
impl ::core::clone::Clone for INHERITED_FROMA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for INHERITED_FROMA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INHERITED_FROMA").field("GenerationGap", &self.GenerationGap).field("AncestorName", &self.AncestorName).finish()
    }
}
impl ::windows::core::TypeKind for INHERITED_FROMA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for INHERITED_FROMA {
    fn eq(&self, other: &Self) -> bool {
        self.GenerationGap == other.GenerationGap && self.AncestorName == other.AncestorName
    }
}
impl ::core::cmp::Eq for INHERITED_FROMA {}
impl ::core::default::Default for INHERITED_FROMA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct INHERITED_FROMW {
    pub GenerationGap: i32,
    pub AncestorName: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for INHERITED_FROMW {}
impl ::core::clone::Clone for INHERITED_FROMW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for INHERITED_FROMW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INHERITED_FROMW").field("GenerationGap", &self.GenerationGap).field("AncestorName", &self.AncestorName).finish()
    }
}
impl ::windows::core::TypeKind for INHERITED_FROMW {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for INHERITED_FROMW {
    fn eq(&self, other: &Self) -> bool {
        self.GenerationGap == other.GenerationGap && self.AncestorName == other.AncestorName
    }
}
impl ::core::cmp::Eq for INHERITED_FROMW {}
impl ::core::default::Default for INHERITED_FROMW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct OBJECTS_AND_NAME_A {
    pub ObjectsPresent: super::SYSTEM_AUDIT_OBJECT_ACE_FLAGS,
    pub ObjectType: SE_OBJECT_TYPE,
    pub ObjectTypeName: ::windows::core::PSTR,
    pub InheritedObjectTypeName: ::windows::core::PSTR,
    pub ptstrName: ::windows::core::PSTR,
}
impl ::core::marker::Copy for OBJECTS_AND_NAME_A {}
impl ::core::clone::Clone for OBJECTS_AND_NAME_A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for OBJECTS_AND_NAME_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OBJECTS_AND_NAME_A").field("ObjectsPresent", &self.ObjectsPresent).field("ObjectType", &self.ObjectType).field("ObjectTypeName", &self.ObjectTypeName).field("InheritedObjectTypeName", &self.InheritedObjectTypeName).field("ptstrName", &self.ptstrName).finish()
    }
}
impl ::windows::core::TypeKind for OBJECTS_AND_NAME_A {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for OBJECTS_AND_NAME_A {
    fn eq(&self, other: &Self) -> bool {
        self.ObjectsPresent == other.ObjectsPresent && self.ObjectType == other.ObjectType && self.ObjectTypeName == other.ObjectTypeName && self.InheritedObjectTypeName == other.InheritedObjectTypeName && self.ptstrName == other.ptstrName
    }
}
impl ::core::cmp::Eq for OBJECTS_AND_NAME_A {}
impl ::core::default::Default for OBJECTS_AND_NAME_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct OBJECTS_AND_NAME_W {
    pub ObjectsPresent: super::SYSTEM_AUDIT_OBJECT_ACE_FLAGS,
    pub ObjectType: SE_OBJECT_TYPE,
    pub ObjectTypeName: ::windows::core::PWSTR,
    pub InheritedObjectTypeName: ::windows::core::PWSTR,
    pub ptstrName: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for OBJECTS_AND_NAME_W {}
impl ::core::clone::Clone for OBJECTS_AND_NAME_W {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for OBJECTS_AND_NAME_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OBJECTS_AND_NAME_W").field("ObjectsPresent", &self.ObjectsPresent).field("ObjectType", &self.ObjectType).field("ObjectTypeName", &self.ObjectTypeName).field("InheritedObjectTypeName", &self.InheritedObjectTypeName).field("ptstrName", &self.ptstrName).finish()
    }
}
impl ::windows::core::TypeKind for OBJECTS_AND_NAME_W {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for OBJECTS_AND_NAME_W {
    fn eq(&self, other: &Self) -> bool {
        self.ObjectsPresent == other.ObjectsPresent && self.ObjectType == other.ObjectType && self.ObjectTypeName == other.ObjectTypeName && self.InheritedObjectTypeName == other.InheritedObjectTypeName && self.ptstrName == other.ptstrName
    }
}
impl ::core::cmp::Eq for OBJECTS_AND_NAME_W {}
impl ::core::default::Default for OBJECTS_AND_NAME_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct OBJECTS_AND_SID {
    pub ObjectsPresent: super::SYSTEM_AUDIT_OBJECT_ACE_FLAGS,
    pub ObjectTypeGuid: ::windows::core::GUID,
    pub InheritedObjectTypeGuid: ::windows::core::GUID,
    pub pSid: *mut super::SID,
}
impl ::core::marker::Copy for OBJECTS_AND_SID {}
impl ::core::clone::Clone for OBJECTS_AND_SID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for OBJECTS_AND_SID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OBJECTS_AND_SID").field("ObjectsPresent", &self.ObjectsPresent).field("ObjectTypeGuid", &self.ObjectTypeGuid).field("InheritedObjectTypeGuid", &self.InheritedObjectTypeGuid).field("pSid", &self.pSid).finish()
    }
}
impl ::windows::core::TypeKind for OBJECTS_AND_SID {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for OBJECTS_AND_SID {
    fn eq(&self, other: &Self) -> bool {
        self.ObjectsPresent == other.ObjectsPresent && self.ObjectTypeGuid == other.ObjectTypeGuid && self.InheritedObjectTypeGuid == other.InheritedObjectTypeGuid && self.pSid == other.pSid
    }
}
impl ::core::cmp::Eq for OBJECTS_AND_SID {}
impl ::core::default::Default for OBJECTS_AND_SID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct TRUSTEE_A {
    pub pMultipleTrustee: *mut TRUSTEE_A,
    pub MultipleTrusteeOperation: MULTIPLE_TRUSTEE_OPERATION,
    pub TrusteeForm: TRUSTEE_FORM,
    pub TrusteeType: TRUSTEE_TYPE,
    pub ptstrName: ::windows::core::PSTR,
}
impl ::core::marker::Copy for TRUSTEE_A {}
impl ::core::clone::Clone for TRUSTEE_A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TRUSTEE_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRUSTEE_A").field("pMultipleTrustee", &self.pMultipleTrustee).field("MultipleTrusteeOperation", &self.MultipleTrusteeOperation).field("TrusteeForm", &self.TrusteeForm).field("TrusteeType", &self.TrusteeType).field("ptstrName", &self.ptstrName).finish()
    }
}
impl ::windows::core::TypeKind for TRUSTEE_A {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for TRUSTEE_A {
    fn eq(&self, other: &Self) -> bool {
        self.pMultipleTrustee == other.pMultipleTrustee && self.MultipleTrusteeOperation == other.MultipleTrusteeOperation && self.TrusteeForm == other.TrusteeForm && self.TrusteeType == other.TrusteeType && self.ptstrName == other.ptstrName
    }
}
impl ::core::cmp::Eq for TRUSTEE_A {}
impl ::core::default::Default for TRUSTEE_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct TRUSTEE_ACCESSA {
    pub lpProperty: ::windows::core::PSTR,
    pub Access: u32,
    pub fAccessFlags: u32,
    pub fReturnedAccess: u32,
}
impl ::core::marker::Copy for TRUSTEE_ACCESSA {}
impl ::core::clone::Clone for TRUSTEE_ACCESSA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TRUSTEE_ACCESSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRUSTEE_ACCESSA").field("lpProperty", &self.lpProperty).field("Access", &self.Access).field("fAccessFlags", &self.fAccessFlags).field("fReturnedAccess", &self.fReturnedAccess).finish()
    }
}
impl ::windows::core::TypeKind for TRUSTEE_ACCESSA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for TRUSTEE_ACCESSA {
    fn eq(&self, other: &Self) -> bool {
        self.lpProperty == other.lpProperty && self.Access == other.Access && self.fAccessFlags == other.fAccessFlags && self.fReturnedAccess == other.fReturnedAccess
    }
}
impl ::core::cmp::Eq for TRUSTEE_ACCESSA {}
impl ::core::default::Default for TRUSTEE_ACCESSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct TRUSTEE_ACCESSW {
    pub lpProperty: ::windows::core::PWSTR,
    pub Access: u32,
    pub fAccessFlags: u32,
    pub fReturnedAccess: u32,
}
impl ::core::marker::Copy for TRUSTEE_ACCESSW {}
impl ::core::clone::Clone for TRUSTEE_ACCESSW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TRUSTEE_ACCESSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRUSTEE_ACCESSW").field("lpProperty", &self.lpProperty).field("Access", &self.Access).field("fAccessFlags", &self.fAccessFlags).field("fReturnedAccess", &self.fReturnedAccess).finish()
    }
}
impl ::windows::core::TypeKind for TRUSTEE_ACCESSW {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for TRUSTEE_ACCESSW {
    fn eq(&self, other: &Self) -> bool {
        self.lpProperty == other.lpProperty && self.Access == other.Access && self.fAccessFlags == other.fAccessFlags && self.fReturnedAccess == other.fReturnedAccess
    }
}
impl ::core::cmp::Eq for TRUSTEE_ACCESSW {}
impl ::core::default::Default for TRUSTEE_ACCESSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub struct TRUSTEE_W {
    pub pMultipleTrustee: *mut TRUSTEE_W,
    pub MultipleTrusteeOperation: MULTIPLE_TRUSTEE_OPERATION,
    pub TrusteeForm: TRUSTEE_FORM,
    pub TrusteeType: TRUSTEE_TYPE,
    pub ptstrName: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for TRUSTEE_W {}
impl ::core::clone::Clone for TRUSTEE_W {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TRUSTEE_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRUSTEE_W").field("pMultipleTrustee", &self.pMultipleTrustee).field("MultipleTrusteeOperation", &self.MultipleTrusteeOperation).field("TrusteeForm", &self.TrusteeForm).field("TrusteeType", &self.TrusteeType).field("ptstrName", &self.ptstrName).finish()
    }
}
impl ::windows::core::TypeKind for TRUSTEE_W {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for TRUSTEE_W {
    fn eq(&self, other: &Self) -> bool {
        self.pMultipleTrustee == other.pMultipleTrustee && self.MultipleTrusteeOperation == other.MultipleTrusteeOperation && self.TrusteeForm == other.TrusteeForm && self.TrusteeType == other.TrusteeType && self.ptstrName == other.ptstrName
    }
}
impl ::core::cmp::Eq for TRUSTEE_W {}
impl ::core::default::Default for TRUSTEE_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type FN_PROGRESS = ::core::option::Option<unsafe extern "system" fn(pobjectname: ::windows::core::PCWSTR, status: u32, pinvokesetting: *mut PROG_INVOKE_SETTING, args: *const ::core::ffi::c_void, securityset: super::super::Foundation::BOOL) -> ()>;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_AUTHZ_COMPUTE_DYNAMIC_GROUPS = ::core::option::Option<unsafe extern "system" fn(hauthzclientcontext: AUTHZ_CLIENT_CONTEXT_HANDLE, args: *const ::core::ffi::c_void, psidattrarray: *mut *mut super::SID_AND_ATTRIBUTES, psidcount: *mut u32, prestrictedsidattrarray: *mut *mut super::SID_AND_ATTRIBUTES, prestrictedsidcount: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_AUTHZ_DYNAMIC_ACCESS_CHECK = ::core::option::Option<unsafe extern "system" fn(hauthzclientcontext: AUTHZ_CLIENT_CONTEXT_HANDLE, pace: *const super::ACE_HEADER, pargs: *const ::core::ffi::c_void, pbaceapplicable: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`*"]
pub type PFN_AUTHZ_FREE_CENTRAL_ACCESS_POLICY = ::core::option::Option<unsafe extern "system" fn(pcentralaccesspolicy: *const ::core::ffi::c_void) -> ()>;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_AUTHZ_FREE_DYNAMIC_GROUPS = ::core::option::Option<unsafe extern "system" fn(psidattrarray: *const super::SID_AND_ATTRIBUTES) -> ()>;
#[doc = "*Required features: `\"Win32_Security_Authorization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_AUTHZ_GET_CENTRAL_ACCESS_POLICY = ::core::option::Option<unsafe extern "system" fn(hauthzclientcontext: AUTHZ_CLIENT_CONTEXT_HANDLE, capid: super::super::Foundation::PSID, pargs: *const ::core::ffi::c_void, pcentralaccesspolicyapplicable: *mut super::super::Foundation::BOOL, ppcentralaccesspolicy: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
