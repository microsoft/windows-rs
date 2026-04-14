/********************************************************************************
*                                                                               *
* securitybaseapi.h -- ApiSet Contract for api-ms-win-security-base-l1          *
*                                                                               *
* Copyright (c) Microsoft Corporation. All rights reserved.                     *
*                                                                               *
********************************************************************************/

#ifdef _MSC_VER
#pragma once
#endif // _MSC_VER

#ifndef _APISECUREBASE_
#define _APISECUREBASE_

#include <apiset.h>
#include <apisetcconv.h>
#include <minwindef.h>
#include <minwinbase.h>

#ifdef __cplusplus
extern "C" {
#endif

//
//
// Security APIs
//

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

WINADVAPI
BOOL
WINAPI
AccessCheck(
    _In_ PSECURITY_DESCRIPTOR pSecurityDescriptor,
    _In_ HANDLE ClientToken,
    _In_ DWORD DesiredAccess,
    _In_ PGENERIC_MAPPING GenericMapping,
    _Out_writes_bytes_to_opt_(*PrivilegeSetLength,*PrivilegeSetLength) PPRIVILEGE_SET PrivilegeSet,
    _Inout_ LPDWORD PrivilegeSetLength,
    _Out_ LPDWORD GrantedAccess,
    _Out_ LPBOOL AccessStatus
    );

WINADVAPI
BOOL
WINAPI
AccessCheckAndAuditAlarmW(
    _In_ LPCWSTR SubsystemName,
    _In_opt_ LPVOID HandleId,
    _In_ LPWSTR ObjectTypeName,
    _In_opt_ LPWSTR ObjectName,
    _In_ PSECURITY_DESCRIPTOR SecurityDescriptor,
    _In_ DWORD DesiredAccess,
    _In_ PGENERIC_MAPPING GenericMapping,
    _In_ BOOL ObjectCreation,
    _Out_ LPDWORD GrantedAccess,
    _Out_ LPBOOL AccessStatus,
    _Out_ LPBOOL pfGenerateOnClose
    );

#ifdef UNICODE
#define AccessCheckAndAuditAlarm  AccessCheckAndAuditAlarmW
#endif

WINADVAPI
BOOL
WINAPI
AccessCheckByType(
    _In_ PSECURITY_DESCRIPTOR pSecurityDescriptor,
    _In_opt_ PSID PrincipalSelfSid,
    _In_ HANDLE ClientToken,
    _In_ DWORD DesiredAccess,
    _Inout_updates_opt_(ObjectTypeListLength) POBJECT_TYPE_LIST ObjectTypeList,
    _In_ DWORD ObjectTypeListLength,
    _In_ PGENERIC_MAPPING GenericMapping,
    _Out_writes_bytes_to_opt_(*PrivilegeSetLength,*PrivilegeSetLength) PPRIVILEGE_SET PrivilegeSet,
    _Inout_ LPDWORD PrivilegeSetLength,
    _Out_ LPDWORD GrantedAccess,
    _Out_ LPBOOL AccessStatus
    );

WINADVAPI
BOOL
WINAPI
AccessCheckByTypeResultList(
    _In_ PSECURITY_DESCRIPTOR pSecurityDescriptor,
    _In_opt_ PSID PrincipalSelfSid,
    _In_ HANDLE ClientToken,
    _In_ DWORD DesiredAccess,
    _Inout_updates_opt_(ObjectTypeListLength) POBJECT_TYPE_LIST ObjectTypeList,
    _In_ DWORD ObjectTypeListLength,
    _In_ PGENERIC_MAPPING GenericMapping,
    _Out_writes_bytes_to_opt_(*PrivilegeSetLength,*PrivilegeSetLength) PPRIVILEGE_SET PrivilegeSet,
    _Inout_ LPDWORD PrivilegeSetLength,
    _Out_writes_(ObjectTypeListLength) LPDWORD GrantedAccessList,
    _Out_writes_(ObjectTypeListLength) LPDWORD AccessStatusList
    );

WINADVAPI
BOOL
WINAPI
AccessCheckByTypeAndAuditAlarmW(
    _In_ LPCWSTR SubsystemName,
    _In_ LPVOID HandleId,
    _In_ LPCWSTR ObjectTypeName,
    _In_opt_ LPCWSTR ObjectName,
    _In_ PSECURITY_DESCRIPTOR SecurityDescriptor,
    _In_opt_ PSID PrincipalSelfSid,
    _In_ DWORD DesiredAccess,
    _In_ AUDIT_EVENT_TYPE AuditType,
    _In_ DWORD Flags,
    _Inout_updates_opt_(ObjectTypeListLength) POBJECT_TYPE_LIST ObjectTypeList,
    _In_ DWORD ObjectTypeListLength,
    _In_ PGENERIC_MAPPING GenericMapping,
    _In_ BOOL ObjectCreation,
    _Out_ LPDWORD GrantedAccess,
    _Out_ LPBOOL AccessStatus,
    _Out_ LPBOOL pfGenerateOnClose
    );

#ifdef UNICODE
#define AccessCheckByTypeAndAuditAlarm  AccessCheckByTypeAndAuditAlarmW
#endif

WINADVAPI
BOOL
WINAPI
AccessCheckByTypeResultListAndAuditAlarmW(
    _In_ LPCWSTR SubsystemName,
    _In_ LPVOID HandleId,
    _In_ LPCWSTR ObjectTypeName,
    _In_opt_ LPCWSTR ObjectName,
    _In_ PSECURITY_DESCRIPTOR SecurityDescriptor,
    _In_opt_ PSID PrincipalSelfSid,
    _In_ DWORD DesiredAccess,
    _In_ AUDIT_EVENT_TYPE AuditType,
    _In_ DWORD Flags,
    _Inout_updates_opt_(ObjectTypeListLength) POBJECT_TYPE_LIST ObjectTypeList,
    _In_ DWORD ObjectTypeListLength,
    _In_ PGENERIC_MAPPING GenericMapping,
    _In_ BOOL ObjectCreation,
    _Out_writes_(ObjectTypeListLength) LPDWORD GrantedAccessList,
    _Out_writes_(ObjectTypeListLength) LPDWORD AccessStatusList,
    _Out_ LPBOOL pfGenerateOnClose
    );

#ifdef UNICODE
#define AccessCheckByTypeResultListAndAuditAlarm  AccessCheckByTypeResultListAndAuditAlarmW
#endif

WINADVAPI
BOOL
WINAPI
AccessCheckByTypeResultListAndAuditAlarmByHandleW(
    _In_ LPCWSTR SubsystemName,
    _In_ LPVOID HandleId,
    _In_ HANDLE ClientToken,
    _In_ LPCWSTR ObjectTypeName,
    _In_opt_ LPCWSTR ObjectName,
    _In_ PSECURITY_DESCRIPTOR SecurityDescriptor,
    _In_opt_ PSID PrincipalSelfSid,
    _In_ DWORD DesiredAccess,
    _In_ AUDIT_EVENT_TYPE AuditType,
    _In_ DWORD Flags,
    _Inout_updates_opt_(ObjectTypeListLength) POBJECT_TYPE_LIST ObjectTypeList,
    _In_ DWORD ObjectTypeListLength,
    _In_ PGENERIC_MAPPING GenericMapping,
    _In_ BOOL ObjectCreation,
    _Out_writes_(ObjectTypeListLength) LPDWORD GrantedAccessList,
    _Out_writes_(ObjectTypeListLength) LPDWORD AccessStatusList,
    _Out_ LPBOOL pfGenerateOnClose
    );

#ifdef UNICODE
#define AccessCheckByTypeResultListAndAuditAlarmByHandle  AccessCheckByTypeResultListAndAuditAlarmByHandleW
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

WINADVAPI
BOOL
WINAPI
AddAccessAllowedAce(
    _Inout_ PACL pAcl,
    _In_ DWORD dwAceRevision,
    _In_ DWORD AccessMask,
    _In_ PSID pSid
    );

WINADVAPI
BOOL
WINAPI
AddAccessAllowedAceEx(
    _Inout_ PACL pAcl,
    _In_ DWORD dwAceRevision,
    _In_ DWORD AceFlags,
    _In_ DWORD AccessMask,
    _In_ PSID pSid
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

WINADVAPI
BOOL
WINAPI
AddAccessAllowedObjectAce(
    _Inout_ PACL pAcl,
    _In_ DWORD dwAceRevision,
    _In_ DWORD AceFlags,
    _In_ DWORD AccessMask,
    _In_opt_ GUID* ObjectTypeGuid,
    _In_opt_ GUID* InheritedObjectTypeGuid,
    _In_ PSID pSid
    );

WINADVAPI
BOOL
WINAPI
AddAccessDeniedAce(
    _Inout_ PACL pAcl,
    _In_ DWORD dwAceRevision,
    _In_ DWORD AccessMask,
    _In_ PSID pSid
    );

WINADVAPI
BOOL
WINAPI
AddAccessDeniedAceEx(
    _Inout_ PACL pAcl,
    _In_ DWORD dwAceRevision,
    _In_ DWORD AceFlags,
    _In_ DWORD AccessMask,
    _In_ PSID pSid
    );

WINADVAPI
BOOL
WINAPI
AddAccessDeniedObjectAce(
    _Inout_ PACL pAcl,
    _In_ DWORD dwAceRevision,
    _In_ DWORD AceFlags,
    _In_ DWORD AccessMask,
    _In_opt_ GUID* ObjectTypeGuid,
    _In_opt_ GUID* InheritedObjectTypeGuid,
    _In_ PSID pSid
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

WINADVAPI
BOOL
WINAPI
AddAce(
    _Inout_ PACL pAcl,
    _In_ DWORD dwAceRevision,
    _In_ DWORD dwStartingAceIndex,
    _In_reads_bytes_(nAceListLength) LPVOID pAceList,
    _In_ DWORD nAceListLength
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

WINADVAPI
BOOL
WINAPI
AddAuditAccessAce(
    _Inout_ PACL pAcl,
    _In_ DWORD dwAceRevision,
    _In_ DWORD dwAccessMask,
    _In_ PSID pSid,
    _In_ BOOL bAuditSuccess,
    _In_ BOOL bAuditFailure
    );

WINADVAPI
BOOL
WINAPI
AddAuditAccessAceEx(
    _Inout_ PACL pAcl,
    _In_ DWORD dwAceRevision,
    _In_ DWORD AceFlags,
    _In_ DWORD dwAccessMask,
    _In_ PSID pSid,
    _In_ BOOL bAuditSuccess,
    _In_ BOOL bAuditFailure
    );

WINADVAPI
BOOL
WINAPI
AddAuditAccessObjectAce(
    _Inout_ PACL pAcl,
    _In_ DWORD dwAceRevision,
    _In_ DWORD AceFlags,
    _In_ DWORD AccessMask,
    _In_opt_ GUID* ObjectTypeGuid,
    _In_opt_ GUID* InheritedObjectTypeGuid,
    _In_ PSID pSid,
    _In_ BOOL bAuditSuccess,
    _In_ BOOL bAuditFailure
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

#if (_WIN32_WINNT >= 0x0600)

WINADVAPI
BOOL
WINAPI
AddMandatoryAce(
    _Inout_ PACL pAcl,
    _In_ DWORD dwAceRevision,
    _In_ DWORD AceFlags,
    _In_ DWORD MandatoryPolicy,
    _In_ PSID pLabelSid
    );

#endif /* _WIN32_WINNT >=  0x0600 */

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN8)

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

WINADVAPI
BOOL
WINAPI
AddResourceAttributeAce(
    _Inout_ PACL pAcl,
    _In_ DWORD dwAceRevision,
    _In_ DWORD AceFlags,
    _In_ DWORD AccessMask,
    _In_ PSID pSid,
    _In_ PCLAIM_SECURITY_ATTRIBUTES_INFORMATION pAttributeInfo,
    _Out_ PDWORD pReturnLength
    );

WINADVAPI
BOOL
WINAPI
AddScopedPolicyIDAce(
    _Inout_ PACL pAcl,
    _In_ DWORD dwAceRevision,
    _In_ DWORD AceFlags,
    _In_ DWORD AccessMask,
    _In_ PSID pSid
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#endif // (_WIN32_WINNT >= _WIN32_WINNT_WIN8)

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

WINADVAPI
BOOL
WINAPI
AdjustTokenGroups(
    _In_ HANDLE TokenHandle,
    _In_ BOOL ResetToDefault,
    _In_opt_ PTOKEN_GROUPS NewState,
    _In_ DWORD BufferLength,
    _Out_writes_bytes_to_opt_(BufferLength,*ReturnLength) PTOKEN_GROUPS PreviousState,
    _Out_opt_ PDWORD ReturnLength
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

WINADVAPI
BOOL
WINAPI
AdjustTokenPrivileges(
    _In_ HANDLE TokenHandle,
    _In_ BOOL DisableAllPrivileges,
    _In_opt_ PTOKEN_PRIVILEGES NewState,
    _In_ DWORD BufferLength,
    _Out_writes_bytes_to_opt_(BufferLength,*ReturnLength) PTOKEN_PRIVILEGES PreviousState,
    _Out_opt_ PDWORD ReturnLength
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

WINADVAPI
BOOL
WINAPI
AllocateAndInitializeSid(
    _In_ PSID_IDENTIFIER_AUTHORITY pIdentifierAuthority,
    _In_ BYTE nSubAuthorityCount,
    _In_ DWORD nSubAuthority0,
    _In_ DWORD nSubAuthority1,
    _In_ DWORD nSubAuthority2,
    _In_ DWORD nSubAuthority3,
    _In_ DWORD nSubAuthority4,
    _In_ DWORD nSubAuthority5,
    _In_ DWORD nSubAuthority6,
    _In_ DWORD nSubAuthority7,
    _Outptr_ PSID* pSid
    );

WINADVAPI
BOOL
WINAPI
AllocateLocallyUniqueId(
    _Out_ PLUID Luid
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

WINADVAPI
BOOL
WINAPI
AreAllAccessesGranted(
    _In_ DWORD GrantedAccess,
    _In_ DWORD DesiredAccess
    );

WINADVAPI
BOOL
WINAPI
AreAnyAccessesGranted(
    _In_ DWORD GrantedAccess,
    _In_ DWORD DesiredAccess
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

WINADVAPI
BOOL
APIENTRY
CheckTokenMembership(
    _In_opt_ HANDLE TokenHandle,
    _In_ PSID SidToCheck,
    _Out_ PBOOL IsMember
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN8)

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

WINADVAPI
BOOL
APIENTRY
CheckTokenCapability(
    _In_opt_ HANDLE TokenHandle,
    _In_ PSID CapabilitySidToCheck,
    _Out_ PBOOL HasCapability
    );

WINADVAPI
BOOL
APIENTRY
GetAppContainerAce(
    _In_ PACL Acl,
    _In_ DWORD StartingAceIndex,
    _Outptr_ PVOID* AppContainerAce,
    _Out_opt_ DWORD* AppContainerAceIndex
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

WINADVAPI
BOOL
APIENTRY
CheckTokenMembershipEx(
    _In_opt_ HANDLE TokenHandle,
    _In_ PSID SidToCheck,
    _In_ DWORD Flags,
    _Out_ PBOOL IsMember
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#endif // (_WIN32_WINNT >= _WIN32_WINNT_WIN8)

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

WINADVAPI
BOOL
WINAPI
ConvertToAutoInheritPrivateObjectSecurity(
    _In_opt_ PSECURITY_DESCRIPTOR ParentDescriptor,
    _In_ PSECURITY_DESCRIPTOR CurrentSecurityDescriptor,
    _Outptr_ PSECURITY_DESCRIPTOR* NewSecurityDescriptor,
    _In_opt_ GUID* ObjectType,
    _In_ BOOLEAN IsDirectoryObject,
    _In_ PGENERIC_MAPPING GenericMapping
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

WINADVAPI
BOOL
WINAPI
CopySid(
    _In_ DWORD nDestinationSidLength,
    _Out_writes_bytes_(nDestinationSidLength) PSID pDestinationSid,
    _In_ PSID pSourceSid
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

WINADVAPI
BOOL
WINAPI
CreatePrivateObjectSecurity(
    _In_opt_ PSECURITY_DESCRIPTOR ParentDescriptor,
    _In_opt_ PSECURITY_DESCRIPTOR CreatorDescriptor,
    _Outptr_ PSECURITY_DESCRIPTOR* NewDescriptor,
    _In_ BOOL IsDirectoryObject,
    _In_opt_ HANDLE Token,
    _In_ PGENERIC_MAPPING GenericMapping
    );

WINADVAPI
BOOL
WINAPI
CreatePrivateObjectSecurityEx(
    _In_opt_ PSECURITY_DESCRIPTOR ParentDescriptor,
    _In_opt_ PSECURITY_DESCRIPTOR CreatorDescriptor,
    _Outptr_ PSECURITY_DESCRIPTOR* NewDescriptor,
    _In_opt_ GUID* ObjectType,
    _In_ BOOL IsContainerObject,
    _In_ ULONG AutoInheritFlags,
    _In_opt_ HANDLE Token,
    _In_ PGENERIC_MAPPING GenericMapping
    );

WINADVAPI
BOOL
WINAPI
CreatePrivateObjectSecurityWithMultipleInheritance(
    _In_opt_ PSECURITY_DESCRIPTOR ParentDescriptor,
    _In_opt_ PSECURITY_DESCRIPTOR CreatorDescriptor,
    _Outptr_ PSECURITY_DESCRIPTOR* NewDescriptor,
    _In_reads_opt_(GuidCount) GUID** ObjectTypes,
    _In_ ULONG GuidCount,
    _In_ BOOL IsContainerObject,
    _In_ ULONG AutoInheritFlags,
    _In_opt_ HANDLE Token,
    _In_ PGENERIC_MAPPING GenericMapping
    );

WINADVAPI
BOOL
APIENTRY
CreateRestrictedToken(
    _In_ HANDLE ExistingTokenHandle,
    _In_ DWORD Flags,
    _In_ DWORD DisableSidCount,
    _In_reads_opt_(DisableSidCount) PSID_AND_ATTRIBUTES SidsToDisable,
    _In_ DWORD DeletePrivilegeCount,
    _In_reads_opt_(DeletePrivilegeCount) PLUID_AND_ATTRIBUTES PrivilegesToDelete,
    _In_ DWORD RestrictedSidCount,
    _In_reads_opt_(RestrictedSidCount) PSID_AND_ATTRIBUTES SidsToRestrict,
    _Outptr_ PHANDLE NewTokenHandle
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

#if (_WIN32_WINNT >= 0x0501)

WINADVAPI
_Success_(return != FALSE)
BOOL
WINAPI
CreateWellKnownSid(
    _In_ WELL_KNOWN_SID_TYPE WellKnownSidType,
    _In_opt_ PSID DomainSid,
    _Out_writes_bytes_to_opt_(*cbSid,*cbSid) PSID pSid,
    _Inout_ DWORD* cbSid
    );

WINADVAPI
_Success_(return != FALSE)
BOOL
WINAPI
EqualDomainSid(
    _In_ PSID pSid1,
    _In_ PSID pSid2,
    _Out_ BOOL* pfEqual
    );

#endif //(_WIN32_WINNT >= 0x0501)

WINADVAPI
BOOL
WINAPI
DeleteAce(
    _Inout_ PACL pAcl,
    _In_ DWORD dwAceIndex
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

WINADVAPI
BOOL
WINAPI
DestroyPrivateObjectSecurity(
    _Pre_valid_ _Post_invalid_ PSECURITY_DESCRIPTOR* ObjectDescriptor
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

WINADVAPI
BOOL
WINAPI
DuplicateToken(
    _In_ HANDLE ExistingTokenHandle,
    _In_ SECURITY_IMPERSONATION_LEVEL ImpersonationLevel,
    _Outptr_ PHANDLE DuplicateTokenHandle
    );

WINADVAPI
BOOL
WINAPI
DuplicateTokenEx(
    _In_ HANDLE hExistingToken,
    _In_ DWORD dwDesiredAccess,
    _In_opt_ LPSECURITY_ATTRIBUTES lpTokenAttributes,
    _In_ SECURITY_IMPERSONATION_LEVEL ImpersonationLevel,
    _In_ TOKEN_TYPE TokenType,
    _Outptr_ PHANDLE phNewToken
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

WINADVAPI
BOOL
WINAPI
EqualPrefixSid(
    _In_ PSID pSid1,
    _In_ PSID pSid2
    );

WINADVAPI
BOOL
WINAPI
EqualSid(
    _In_ PSID pSid1,
    _In_ PSID pSid2
    );

WINADVAPI
BOOL
WINAPI
FindFirstFreeAce(
    _In_ PACL pAcl,
    _Outptr_ LPVOID* pAce
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

WINADVAPI
PVOID
WINAPI
FreeSid(
    _In_ PSID pSid
    );

WINADVAPI
BOOL
WINAPI
GetAce(
    _In_ PACL pAcl,
    _In_ DWORD dwAceIndex,
    _Outptr_ LPVOID* pAce
    );

WINADVAPI
BOOL
WINAPI
GetAclInformation(
    _In_ PACL pAcl,
    _Out_writes_bytes_(nAclInformationLength) LPVOID pAclInformation,
    _In_ DWORD nAclInformationLength,
    _In_ ACL_INFORMATION_CLASS dwAclInformationClass
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

WINADVAPI
BOOL
WINAPI
GetFileSecurityW(
    _In_ LPCWSTR lpFileName,
    _In_ SECURITY_INFORMATION RequestedInformation,
    _Out_writes_bytes_to_opt_(nLength,*lpnLengthNeeded) PSECURITY_DESCRIPTOR pSecurityDescriptor,
    _In_ DWORD nLength,
    _Out_ LPDWORD lpnLengthNeeded
    );

#ifdef UNICODE
#define GetFileSecurity  GetFileSecurityW
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

WINADVAPI
BOOL
WINAPI
GetKernelObjectSecurity(
    _In_ HANDLE Handle,
    _In_ SECURITY_INFORMATION RequestedInformation,
    _Out_writes_bytes_opt_(nLength) PSECURITY_DESCRIPTOR pSecurityDescriptor,
    _In_ DWORD nLength,
    _Out_ LPDWORD lpnLengthNeeded
    );

WINADVAPI
_Post_satisfies_(return >= 8 && return <= SECURITY_MAX_SID_SIZE)
_Success_(1)
DWORD
WINAPI
GetLengthSid(
    _In_ _Post_readable_byte_size_(return) PSID pSid
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

WINADVAPI
_Success_(return != FALSE)
BOOL
WINAPI
GetPrivateObjectSecurity(
    _In_ PSECURITY_DESCRIPTOR ObjectDescriptor,
    _In_ SECURITY_INFORMATION SecurityInformation,
    _Out_writes_bytes_to_opt_(DescriptorLength,*ReturnLength) PSECURITY_DESCRIPTOR ResultantDescriptor,
    _In_ DWORD DescriptorLength,
    _Out_ PDWORD ReturnLength
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

WINADVAPI
BOOL
WINAPI
GetSecurityDescriptorControl(
    _In_ PSECURITY_DESCRIPTOR pSecurityDescriptor,
    _Out_ PSECURITY_DESCRIPTOR_CONTROL pControl,
    _Out_ LPDWORD lpdwRevision
    );

WINADVAPI
BOOL
WINAPI
GetSecurityDescriptorDacl(
    _In_ PSECURITY_DESCRIPTOR pSecurityDescriptor,
    _Out_ LPBOOL lpbDaclPresent,
    _Outptr_ PACL* pDacl,
    _Out_ LPBOOL lpbDaclDefaulted
    );

WINADVAPI
BOOL
WINAPI
GetSecurityDescriptorGroup(
    _In_ PSECURITY_DESCRIPTOR pSecurityDescriptor,
    _Outptr_ PSID* pGroup,
    _Out_ LPBOOL lpbGroupDefaulted
    );

WINADVAPI
DWORD
WINAPI
GetSecurityDescriptorLength(
    _In_ PSECURITY_DESCRIPTOR pSecurityDescriptor
    );

WINADVAPI
BOOL
WINAPI
GetSecurityDescriptorOwner(
    _In_ PSECURITY_DESCRIPTOR pSecurityDescriptor,
    _Outptr_ PSID* pOwner,
    _Out_ LPBOOL lpbOwnerDefaulted
    );

WINADVAPI
DWORD
WINAPI
GetSecurityDescriptorRMControl(
    _In_ PSECURITY_DESCRIPTOR SecurityDescriptor,
    _Out_ PUCHAR RMControl
    );

WINADVAPI
BOOL
WINAPI
GetSecurityDescriptorSacl(
    _In_ PSECURITY_DESCRIPTOR pSecurityDescriptor,
    _Out_ LPBOOL lpbSaclPresent,
    _Outptr_ PACL* pSacl,
    _Out_ LPBOOL lpbSaclDefaulted
    );

WINADVAPI
PSID_IDENTIFIER_AUTHORITY
WINAPI
GetSidIdentifierAuthority(
    _In_ PSID pSid
    );

WINADVAPI
DWORD
WINAPI
GetSidLengthRequired(
    _In_ UCHAR nSubAuthorityCount
    );

WINADVAPI
PDWORD
WINAPI
GetSidSubAuthority(
    _In_ PSID pSid,
    _In_ DWORD nSubAuthority
    );

WINADVAPI
PUCHAR
WINAPI
GetSidSubAuthorityCount(
    _In_ PSID pSid
    );

WINADVAPI
BOOL
WINAPI
GetTokenInformation(
    _In_ HANDLE TokenHandle,
    _In_ TOKEN_INFORMATION_CLASS TokenInformationClass,
    _Out_writes_bytes_to_opt_(TokenInformationLength,*ReturnLength) LPVOID TokenInformation,
    _In_ DWORD TokenInformationLength,
    _Out_ PDWORD ReturnLength
    );

#if (_WIN32_WINNT >= 0x0501)

WINADVAPI
_Success_(return != FALSE)
BOOL
WINAPI
GetWindowsAccountDomainSid(
    _In_ PSID pSid,
    _Out_writes_bytes_to_opt_(*cbDomainSid,*cbDomainSid) PSID pDomainSid,
    _Inout_ DWORD* cbDomainSid
    );

#endif //(_WIN32_WINNT >= 0x0501)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

WINADVAPI
BOOL
APIENTRY
ImpersonateAnonymousToken(
    _In_ HANDLE ThreadHandle
    );

_Must_inspect_result_
WINADVAPI
BOOL
WINAPI
ImpersonateLoggedOnUser(
    _In_ HANDLE hToken
    );

_Must_inspect_result_
WINADVAPI
BOOL
WINAPI
ImpersonateSelf(
    _In_ SECURITY_IMPERSONATION_LEVEL ImpersonationLevel
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

WINADVAPI
BOOL
WINAPI
InitializeAcl(
    _Out_writes_bytes_(nAclLength) PACL pAcl,
    _In_ DWORD nAclLength,
    _In_ DWORD dwAclRevision
    );

WINADVAPI
BOOL
WINAPI
InitializeSecurityDescriptor(
    _Out_ PSECURITY_DESCRIPTOR pSecurityDescriptor,
    _In_ DWORD dwRevision
    );

WINADVAPI
BOOL
WINAPI
InitializeSid(
    _Out_writes_bytes_(_Inexpressible_(GetSidLengthRequired(nSubAuthorityCount))) PSID Sid,
    _In_ PSID_IDENTIFIER_AUTHORITY pIdentifierAuthority,
    _In_ BYTE nSubAuthorityCount
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

WINADVAPI
BOOL
WINAPI
IsTokenRestricted(
    _In_ HANDLE TokenHandle
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

WINADVAPI
BOOL
WINAPI
IsValidAcl(
    _In_ PACL pAcl
    );

WINADVAPI
BOOL
WINAPI
IsValidSecurityDescriptor(
    _In_ PSECURITY_DESCRIPTOR pSecurityDescriptor
    );

WINADVAPI
BOOL
WINAPI
IsValidSid(
    _In_ PSID pSid
    );

#if (_WIN32_WINNT >= 0x0501)

WINADVAPI
BOOL
WINAPI
IsWellKnownSid(
    _In_ PSID pSid,
    _In_ WELL_KNOWN_SID_TYPE WellKnownSidType
    );

#endif // (_WIN32_WINNT >= 0x0501)

WINADVAPI
_Success_(return != FALSE)
BOOL
WINAPI
MakeAbsoluteSD(
    _In_ PSECURITY_DESCRIPTOR pSelfRelativeSecurityDescriptor,
    _Out_writes_bytes_to_opt_(*lpdwAbsoluteSecurityDescriptorSize,*lpdwAbsoluteSecurityDescriptorSize) PSECURITY_DESCRIPTOR pAbsoluteSecurityDescriptor,
    _Inout_ LPDWORD lpdwAbsoluteSecurityDescriptorSize,
    _Out_writes_bytes_to_opt_(*lpdwDaclSize,*lpdwDaclSize) PACL pDacl,
    _Inout_ LPDWORD lpdwDaclSize,
    _Out_writes_bytes_to_opt_(*lpdwSaclSize,*lpdwSaclSize) PACL pSacl,
    _Inout_ LPDWORD lpdwSaclSize,
    _Out_writes_bytes_to_opt_(*lpdwOwnerSize,*lpdwOwnerSize) PSID pOwner,
    _Inout_ LPDWORD lpdwOwnerSize,
    _Out_writes_bytes_to_opt_(*lpdwPrimaryGroupSize,*lpdwPrimaryGroupSize) PSID pPrimaryGroup,
    _Inout_ LPDWORD lpdwPrimaryGroupSize
    );

WINADVAPI
_Success_(return != FALSE)
BOOL
WINAPI
MakeSelfRelativeSD(
    _In_ PSECURITY_DESCRIPTOR pAbsoluteSecurityDescriptor,
    _Out_writes_bytes_to_opt_(*lpdwBufferLength,*lpdwBufferLength) PSECURITY_DESCRIPTOR pSelfRelativeSecurityDescriptor,
    _Inout_ LPDWORD lpdwBufferLength
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

WINADVAPI
VOID
WINAPI
MapGenericMask(
    _Inout_ PDWORD AccessMask,
    _In_ PGENERIC_MAPPING GenericMapping
    );

WINADVAPI
BOOL
WINAPI
ObjectCloseAuditAlarmW(
    _In_ LPCWSTR SubsystemName,
    _In_ LPVOID HandleId,
    _In_ BOOL GenerateOnClose
    );

#ifdef UNICODE
#define ObjectCloseAuditAlarm  ObjectCloseAuditAlarmW
#endif

WINADVAPI
BOOL
WINAPI
ObjectDeleteAuditAlarmW(
    _In_ LPCWSTR SubsystemName,
    _In_ LPVOID HandleId,
    _In_ BOOL GenerateOnClose
    );

#ifdef UNICODE
#define ObjectDeleteAuditAlarm  ObjectDeleteAuditAlarmW
#endif

WINADVAPI
BOOL
WINAPI
ObjectOpenAuditAlarmW(
    _In_ LPCWSTR SubsystemName,
    _In_ LPVOID HandleId,
    _In_ LPWSTR ObjectTypeName,
    _In_opt_ LPWSTR ObjectName,
    _In_ PSECURITY_DESCRIPTOR pSecurityDescriptor,
    _In_ HANDLE ClientToken,
    _In_ DWORD DesiredAccess,
    _In_ DWORD GrantedAccess,
    _In_opt_ PPRIVILEGE_SET Privileges,
    _In_ BOOL ObjectCreation,
    _In_ BOOL AccessGranted,
    _Out_ LPBOOL GenerateOnClose
    );

#ifdef UNICODE
#define ObjectOpenAuditAlarm  ObjectOpenAuditAlarmW
#endif

WINADVAPI
BOOL
WINAPI
ObjectPrivilegeAuditAlarmW(
    _In_ LPCWSTR SubsystemName,
    _In_ LPVOID HandleId,
    _In_ HANDLE ClientToken,
    _In_ DWORD DesiredAccess,
    _In_ PPRIVILEGE_SET Privileges,
    _In_ BOOL AccessGranted
    );

#ifdef UNICODE
#define ObjectPrivilegeAuditAlarm  ObjectPrivilegeAuditAlarmW
#endif

WINADVAPI
BOOL
WINAPI
PrivilegeCheck(
    _In_ HANDLE ClientToken,
    _Inout_ PPRIVILEGE_SET RequiredPrivileges,
    _Out_ LPBOOL pfResult
    );

WINADVAPI
BOOL
WINAPI
PrivilegedServiceAuditAlarmW(
    _In_ LPCWSTR SubsystemName,
    _In_ LPCWSTR ServiceName,
    _In_ HANDLE ClientToken,
    _In_ PPRIVILEGE_SET Privileges,
    _In_ BOOL AccessGranted
    );

#ifdef UNICODE
#define PrivilegedServiceAuditAlarm  PrivilegedServiceAuditAlarmW
#endif

#if (_WIN32_WINNT >= 0x0600)

WINADVAPI
VOID
WINAPI
QuerySecurityAccessMask(
    _In_ SECURITY_INFORMATION SecurityInformation,
    _Out_ LPDWORD DesiredAccess
    );

#endif // (_WIN32_WINNT >= 0x0600)

WINADVAPI
BOOL
WINAPI
RevertToSelf(
    VOID
    );

WINADVAPI
BOOL
WINAPI
SetAclInformation(
    _Inout_ PACL pAcl,
    _In_reads_bytes_(nAclInformationLength) LPVOID pAclInformation,
    _In_ DWORD nAclInformationLength,
    _In_ ACL_INFORMATION_CLASS dwAclInformationClass
    );

WINADVAPI
BOOL
WINAPI
SetFileSecurityW(
    _In_ LPCWSTR lpFileName,
    _In_ SECURITY_INFORMATION SecurityInformation,
    _In_ PSECURITY_DESCRIPTOR pSecurityDescriptor
    );

#ifdef UNICODE
#define SetFileSecurity  SetFileSecurityW
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

WINADVAPI
BOOL
WINAPI
SetKernelObjectSecurity(
    _In_ HANDLE Handle,
    _In_ SECURITY_INFORMATION SecurityInformation,
    _In_ PSECURITY_DESCRIPTOR SecurityDescriptor
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

WINADVAPI
BOOL
WINAPI
SetPrivateObjectSecurity(
    _In_ SECURITY_INFORMATION SecurityInformation,
    _In_ PSECURITY_DESCRIPTOR ModificationDescriptor,
    _Inout_ PSECURITY_DESCRIPTOR* ObjectsSecurityDescriptor,
    _In_ PGENERIC_MAPPING GenericMapping,
    _In_opt_ HANDLE Token
    );

WINADVAPI
BOOL
WINAPI
SetPrivateObjectSecurityEx(
    _In_ SECURITY_INFORMATION SecurityInformation,
    _In_ PSECURITY_DESCRIPTOR ModificationDescriptor,
    _Inout_ PSECURITY_DESCRIPTOR* ObjectsSecurityDescriptor,
    _In_ ULONG AutoInheritFlags,
    _In_ PGENERIC_MAPPING GenericMapping,
    _In_opt_ HANDLE Token
    );

#if (_WIN32_WINNT >= 0x0600)

WINADVAPI
VOID
WINAPI
SetSecurityAccessMask(
    _In_ SECURITY_INFORMATION SecurityInformation,
    _Out_ LPDWORD DesiredAccess
    );

#endif // (_WIN32_WINNT >= 0x0600)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

WINADVAPI
BOOL
WINAPI
SetSecurityDescriptorControl(
    _In_ PSECURITY_DESCRIPTOR pSecurityDescriptor,
    _In_ SECURITY_DESCRIPTOR_CONTROL ControlBitsOfInterest,
    _In_ SECURITY_DESCRIPTOR_CONTROL ControlBitsToSet
    );

WINADVAPI
BOOL
WINAPI
SetSecurityDescriptorDacl(
    _Inout_ PSECURITY_DESCRIPTOR pSecurityDescriptor,
    _In_ BOOL bDaclPresent,
    _In_opt_ PACL pDacl,
    _In_ BOOL bDaclDefaulted
    );

WINADVAPI
BOOL
WINAPI
SetSecurityDescriptorGroup(
    _Inout_ PSECURITY_DESCRIPTOR pSecurityDescriptor,
    _In_opt_ PSID pGroup,
    _In_ BOOL bGroupDefaulted
    );

WINADVAPI
BOOL
WINAPI
SetSecurityDescriptorOwner(
    _Inout_ PSECURITY_DESCRIPTOR pSecurityDescriptor,
    _In_opt_ PSID pOwner,
    _In_ BOOL bOwnerDefaulted
    );

WINADVAPI
DWORD
WINAPI
SetSecurityDescriptorRMControl(
    _Inout_ PSECURITY_DESCRIPTOR SecurityDescriptor,
    _In_opt_ PUCHAR RMControl
    );

WINADVAPI
BOOL
WINAPI
SetSecurityDescriptorSacl(
    _Inout_ PSECURITY_DESCRIPTOR pSecurityDescriptor,
    _In_ BOOL bSaclPresent,
    _In_opt_ PACL pSacl,
    _In_ BOOL bSaclDefaulted
    );

WINADVAPI
BOOL
WINAPI
SetTokenInformation(
    _In_ HANDLE TokenHandle,
    _In_ TOKEN_INFORMATION_CLASS TokenInformationClass,
    _In_reads_bytes_(TokenInformationLength) LPVOID TokenInformation,
    _In_ DWORD TokenInformationLength
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN8)

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#define SIGNING_LEVEL_FILE_CACHE_FLAG_NOT_VALIDATED  0x01
#define SIGNING_LEVEL_FILE_CACHE_FLAG_VALIDATE_ONLY  0x04

#define SIGNING_LEVEL_MICROSOFT 8

WINADVAPI
BOOL
WINAPI
SetCachedSigningLevel(
    _In_reads_(SourceFileCount) PHANDLE SourceFiles,
    _In_ ULONG SourceFileCount,
    _In_ ULONG Flags,
    _In_opt_ HANDLE TargetFile
    );

WINADVAPI
BOOL
WINAPI
GetCachedSigningLevel(
    _In_ HANDLE File,
    _Out_ PULONG Flags,
    _Out_ PULONG SigningLevel,
    _Out_writes_bytes_to_opt_(*ThumbprintSize,*ThumbprintSize) PUCHAR Thumbprint,
    _Inout_opt_ PULONG ThumbprintSize,
    _Out_opt_ PULONG ThumbprintAlgorithm
    );

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#endif // (_WIN32_WINNT >= _WIN32_WINNT_WIN8)
 #if (_WIN32_WINNT >= _WIN32_WINNT_WIN10)

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

WINADVAPI
LONG
WINAPI
CveEventWrite(
    _In_ PCWSTR CveId,
    _In_opt_ PCWSTR AdditionalDetails
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#endif // (_WIN32_WINNT >= _WIN32_WINNT_WIN10)
 #if (_WIN32_WINNT >= _WIN32_WINNT_WIN10)

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

WINADVAPI
BOOL
WINAPI
DeriveCapabilitySidsFromName(
    _In_ LPCWSTR CapName,
    _Outptr_result_buffer_maybenull_(*CapabilityGroupSidCount) PSID** CapabilityGroupSids,
    _Out_ DWORD* CapabilityGroupSidCount,
    _Outptr_result_buffer_maybenull_(*CapabilitySidCount) PSID** CapabilitySids,
    _Out_ DWORD* CapabilitySidCount
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#endif // (_WIN32_WINNT >= _WIN32_WINNT_WIN10)

#ifdef __cplusplus
}
#endif

#endif // _APISECUREBASE_
