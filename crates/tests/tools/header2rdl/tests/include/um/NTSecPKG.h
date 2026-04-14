/*++ BUILD Version: 0000     Increment this if a change has global effects

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    ntsecpkg.h

Abstract:

    This module defines the structures and APIs for use by a
    authentication or security package.

Revision History:

--*/

#ifndef _NTSECPKG_
#define _NTSECPKG_

#include <winapifamily.h>

#ifdef __cplusplus
extern "C" {
#endif

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)


/////////////////////////////////////////////////////////////////////////
//                                                                     //
// Data types used by authentication packages                          //
//                                                                     //
/////////////////////////////////////////////////////////////////////////

//
// opaque data type which represents a client request
//

typedef PVOID *PLSA_CLIENT_REQUEST;


//
// When a logon of a user is requested, the authentication package
// is expected to return one of the following structures indicating
// the contents of a user's token.
//

typedef enum _LSA_TOKEN_INFORMATION_TYPE {
    LsaTokenInformationNull,  // Implies LSA_TOKEN_INFORMATION_NULL data type
    LsaTokenInformationV1,    // Implies LSA_TOKEN_INFORMATION_V1 data type
    LsaTokenInformationV2,    // Implies LSA_TOKEN_INFORMATION_V2 data type
    LsaTokenInformationV3     // Implies LSA_TOKEN_INFORMATION_V3 data type
} LSA_TOKEN_INFORMATION_TYPE, *PLSA_TOKEN_INFORMATION_TYPE;


//
// The NULL information is used in cases where a non-authenticated
// system access is needed.  For example, a non-authentication network
// circuit (such as LAN Manager's null session) can be given NULL
// information.  This will result in an anonymous token being generated
// for the logon that gives the user no ability to access protected system
// resources, but does allow access to non-protected system resources.
//

typedef struct _LSA_TOKEN_INFORMATION_NULL {

    //
    // Time at which the security context becomes invalid.
    // Use a value in the distant future if the context
    // never expires.
    //

    LARGE_INTEGER ExpirationTime;

    //
    // The SID(s) of groups the user is to be made a member of.  This should
    // not include WORLD or other system defined and assigned
    // SIDs.  These will be added automatically by LSA.
    //
    // Each SID is expected to be in a separately allocated block
    // of memory.  The TOKEN_GROUPS structure is also expected to
    // be in a separately allocated block of memory.
    //

    PTOKEN_GROUPS Groups;

} LSA_TOKEN_INFORMATION_NULL, *PLSA_TOKEN_INFORMATION_NULL;


//
// The V1 token information structure is superceeded by the V2 token
// information structure.  The V1 strucure should only be used for
// backwards compatability.
// This structure contains information that an authentication package
// can place in a Version 1 NT token object.
//
// Do not make any changes to this structure without also updating
// the V3 structure below.
//

typedef struct _LSA_TOKEN_INFORMATION_V1 {

    //
    // Time at which the security context becomes invalid.
    // Use a value in the distant future if the context
    // never expires.
    //

    LARGE_INTEGER ExpirationTime;

    //
    // The SID of the user logging on.  The SID value is in a
    // separately allocated block of memory.
    //

    TOKEN_USER User;

    //
    // The SID(s) of groups the user is a member of.  This should
    // not include WORLD or other system defined and assigned
    // SIDs.  These will be added automatically by LSA.
    //
    // Each SID is expected to be in a separately allocated block
    // of memory.  The TOKEN_GROUPS structure is also expected to
    // be in a separately allocated block of memory.
    //

    PTOKEN_GROUPS Groups;

    //
    // This field is used to establish the primary group of the user.
    // This value does not have to correspond to one of the SIDs
    // assigned to the user.
    //
    // The SID pointed to by this structure is expected to be in
    // a separately allocated block of memory.
    //
    // This field is mandatory and must be filled in.
    //

    TOKEN_PRIMARY_GROUP PrimaryGroup;

    //
    // The privileges the user is assigned.  This list of privileges
    // will be augmented or over-ridden by any local security policy
    // assigned privileges.
    //
    // Each privilege is expected to be in a separately allocated
    // block of memory.  The TOKEN_PRIVILEGES structure is also
    // expected to be in a separately allocated block of memory.
    //
    // If there are no privileges to assign to the user, this field
    // may be set to NULL.
    //

    PTOKEN_PRIVILEGES Privileges;

    //
    // This field may be used to establish an explicit default
    // owner.  Normally, the user ID is used as the default owner.
    // If another value is desired, it must be specified here.
    //
    // The Owner.Sid field may be set to NULL to indicate there is no
    // alternate default owner value.
    //

    TOKEN_OWNER Owner;

    //
    // This field may be used to establish a default
    // protection for the user.  If no value is provided, then
    // a default protection that grants everyone all access will
    // be established.
    //
    // The DefaultDacl.DefaultDacl field may be set to NULL to indicate
    // there is no default protection.
    //

    TOKEN_DEFAULT_DACL DefaultDacl;

} LSA_TOKEN_INFORMATION_V1, *PLSA_TOKEN_INFORMATION_V1;

//
// The V2 information is used in most cases of logon.  The structure is identical
// to the V1 token information structure, with the exception that the memory allocation
// is handled differently.  The LSA_TOKEN_INFORMATION_V2 structure is intended to be
// allocated monolithiclly, with the privileges, DACL, sids, and group array either part of
// same allocation, or allocated and freed externally.
//

typedef LSA_TOKEN_INFORMATION_V1 LSA_TOKEN_INFORMATION_V2, *PLSA_TOKEN_INFORMATION_V2;

//
// The V3 token information structure adds claims support to the LSA token.  LSA assumes
// that the first fields in this structure are identical to those in LSA_TOKEN_INFORMATION_V1,
// so no changes should be made that are not reflected there as well.
//

typedef struct _LSA_TOKEN_INFORMATION_V3 {

    //
    // Time at which the security context becomes invalid.
    // Use a value in the distant future if the context
    // never expires.
    //

    LARGE_INTEGER ExpirationTime;

    //
    // The SID of the user logging on.  The SID value is in a
    // separately allocated block of memory.
    //

    TOKEN_USER User;

    //
    // The SID(s) of groups the user is a member of.  This should
    // not include WORLD or other system defined and assigned
    // SIDs.  These will be added automatically by LSA.
    //
    // Each SID is expected to be in a separately allocated block
    // of memory.  The TOKEN_GROUPS structure is also expected to
    // be in a separately allocated block of memory.
    //

    PTOKEN_GROUPS Groups;

    //
    // This field is used to establish the primary group of the user.
    // This value does not have to correspond to one of the SIDs
    // assigned to the user.
    //
    // The SID pointed to by this structure is expected to be in
    // a separately allocated block of memory.
    //
    // This field is mandatory and must be filled in.
    //

    TOKEN_PRIMARY_GROUP PrimaryGroup;

    //
    // The privileges the user is assigned.  This list of privileges
    // will be augmented or over-ridden by any local security policy
    // assigned privileges.
    //
    // Each privilege is expected to be in a separately allocated
    // block of memory.  The TOKEN_PRIVILEGES structure is also
    // expected to be in a separately allocated block of memory.
    //
    // If there are no privileges to assign to the user, this field
    // may be set to NULL.
    //

    PTOKEN_PRIVILEGES Privileges;

    //
    // This field may be used to establish an explicit default
    // owner.  Normally, the user ID is used as the default owner.
    // If another value is desired, it must be specified here.
    //
    // The Owner.Sid field may be set to NULL to indicate there is no
    // alternate default owner value.
    //

    TOKEN_OWNER Owner;

    //
    // This field may be used to establish a default
    // protection for the user.  If no value is provided, then
    // a default protection that grants everyone all access will
    // be established.
    //
    // The DefaultDacl.DefaultDacl field may be set to NULL to indicate
    // there is no default protection.
    //

    TOKEN_DEFAULT_DACL DefaultDacl;

    //
    // Note: do not change any fields above this comment without updating
    // the V1 structure as well!
    //

    //
    // This field stores the opaque user claims blob for the token. NULL
    // claims is valid, and indicates no additional user claims are present
    // in the token.  Claims are allow-only entities, and as such omitting
    // claims may restrict access.
    //

    TOKEN_USER_CLAIMS UserClaims;

    //
    // This field stores the opaque device claims blob for the token. Semantics
    // here are identical to user claims above.
    //

    TOKEN_DEVICE_CLAIMS DeviceClaims;

    //
    // The SID(s) of groups the authenticating device is a member of.  As with
    // user groups, this should not include WORLD or other system defined and
    // assigned SIDs.  NULL DeviceGroups is valid, and indicates that no compounding
    // should occur.  If DeviceGroups are present, LSA will add WORLD and other assigned
    // SIDs.
    //
    // Unlike user groups, there is no notion of a primary device group.
    //
    // Each SID is expected to be in a separately allocated block
    // of memory.  The TOKEN_GROUPS structure is also expected to
    // be in a separately allocated block of memory.
    //

    PTOKEN_GROUPS DeviceGroups;

} LSA_TOKEN_INFORMATION_V3, *PLSA_TOKEN_INFORMATION_V3;

/////////////////////////////////////////////////////////////////////////
//                                                                     //
// Interface definitions available for use by authentication packages  //
//                                                                     //
/////////////////////////////////////////////////////////////////////////



typedef NTSTATUS
(NTAPI LSA_CREATE_LOGON_SESSION) (
    _Inout_ PLUID LogonId
    );

typedef NTSTATUS
(NTAPI LSA_DELETE_LOGON_SESSION) (
    _In_ PLUID LogonId
    );

typedef NTSTATUS
(NTAPI LSA_ADD_CREDENTIAL) (
    _In_ PLUID LogonId,
    _In_ ULONG AuthenticationPackage,
    _In_ PLSA_STRING PrimaryKeyValue,
    _In_ PLSA_STRING Credentials
    );

typedef NTSTATUS
(NTAPI LSA_GET_CREDENTIALS) (
    _In_ PLUID LogonId,
    _In_ ULONG AuthenticationPackage,
    _Inout_ PULONG QueryContext,
    _In_ BOOLEAN RetrieveAllCredentials,
    _In_ PLSA_STRING PrimaryKeyValue,
    _Out_ PULONG PrimaryKeyLength,
    _In_ PLSA_STRING Credentials
    );

typedef NTSTATUS
(NTAPI LSA_DELETE_CREDENTIAL) (
    _In_ PLUID LogonId,
    _In_ ULONG AuthenticationPackage,
    _In_ PLSA_STRING PrimaryKeyValue
    );

typedef PVOID
(NTAPI LSA_ALLOCATE_LSA_HEAP) (
    _In_ ULONG Length
    );

typedef VOID
(NTAPI LSA_FREE_LSA_HEAP) (
    _In_ PVOID Base
    );

typedef PVOID
(NTAPI LSA_ALLOCATE_PRIVATE_HEAP) (
    _In_ SIZE_T Length
    );

typedef VOID
(NTAPI LSA_FREE_PRIVATE_HEAP) (
    _In_ PVOID Base
    );

typedef NTSTATUS
(NTAPI LSA_ALLOCATE_CLIENT_BUFFER) (
    _In_ PLSA_CLIENT_REQUEST ClientRequest,
    _In_ ULONG LengthRequired,
    _Outptr_result_bytebuffer_(LengthRequired) PVOID *ClientBaseAddress
    );

typedef NTSTATUS
(NTAPI LSA_FREE_CLIENT_BUFFER) (
    _In_ PLSA_CLIENT_REQUEST ClientRequest,
    _In_ PVOID ClientBaseAddress
    );

typedef NTSTATUS
(NTAPI LSA_COPY_TO_CLIENT_BUFFER) (
    _In_ PLSA_CLIENT_REQUEST ClientRequest,
    _In_ ULONG Length,
    _Out_writes_bytes_(Length) PVOID ClientBaseAddress,
    _In_reads_bytes_(Length) PVOID BufferToCopy
    );

typedef NTSTATUS
(NTAPI LSA_COPY_FROM_CLIENT_BUFFER) (
    _In_ PLSA_CLIENT_REQUEST ClientRequest,
    _In_ ULONG Length,
    _Out_writes_bytes_(Length) PVOID BufferToCopy,
    _In_reads_bytes_(Length) PVOID ClientBaseAddress
    );

typedef LSA_CREATE_LOGON_SESSION * PLSA_CREATE_LOGON_SESSION;
typedef LSA_DELETE_LOGON_SESSION * PLSA_DELETE_LOGON_SESSION;
typedef LSA_ADD_CREDENTIAL * PLSA_ADD_CREDENTIAL;
typedef LSA_GET_CREDENTIALS * PLSA_GET_CREDENTIALS;
typedef LSA_DELETE_CREDENTIAL * PLSA_DELETE_CREDENTIAL;
typedef LSA_ALLOCATE_LSA_HEAP * PLSA_ALLOCATE_LSA_HEAP;
typedef LSA_FREE_LSA_HEAP * PLSA_FREE_LSA_HEAP;
typedef LSA_ALLOCATE_PRIVATE_HEAP * PLSA_ALLOCATE_PRIVATE_HEAP;
typedef LSA_FREE_PRIVATE_HEAP * PLSA_FREE_PRIVATE_HEAP;
typedef LSA_ALLOCATE_CLIENT_BUFFER * PLSA_ALLOCATE_CLIENT_BUFFER;
typedef LSA_FREE_CLIENT_BUFFER * PLSA_FREE_CLIENT_BUFFER;
typedef LSA_COPY_TO_CLIENT_BUFFER * PLSA_COPY_TO_CLIENT_BUFFER;
typedef LSA_COPY_FROM_CLIENT_BUFFER * PLSA_COPY_FROM_CLIENT_BUFFER;

//
// The dispatch table of LSA services which are available to
// authentication packages.
//
typedef struct _LSA_DISPATCH_TABLE {
    PLSA_CREATE_LOGON_SESSION CreateLogonSession;
    PLSA_DELETE_LOGON_SESSION DeleteLogonSession;
    PLSA_ADD_CREDENTIAL AddCredential;
    PLSA_GET_CREDENTIALS GetCredentials;
    PLSA_DELETE_CREDENTIAL DeleteCredential;
    PLSA_ALLOCATE_LSA_HEAP AllocateLsaHeap;
    PLSA_FREE_LSA_HEAP FreeLsaHeap;
    PLSA_ALLOCATE_CLIENT_BUFFER AllocateClientBuffer;
    PLSA_FREE_CLIENT_BUFFER FreeClientBuffer;
    PLSA_COPY_TO_CLIENT_BUFFER CopyToClientBuffer;
    PLSA_COPY_FROM_CLIENT_BUFFER CopyFromClientBuffer;
} LSA_DISPATCH_TABLE, *PLSA_DISPATCH_TABLE;



////////////////////////////////////////////////////////////////////////////
//                                                                        //
// Interface definitions of services provided by authentication packages  //
//                                                                        //
////////////////////////////////////////////////////////////////////////////



//
// Routine names
//
// The routines provided by the DLL must be assigned the following names
// so that their addresses can be retrieved when the DLL is loaded.
//

#define LSA_AP_NAME_INITIALIZE_PACKAGE      "LsaApInitializePackage\0"
#define LSA_AP_NAME_LOGON_USER              "LsaApLogonUser\0"
#define LSA_AP_NAME_LOGON_USER_EX           "LsaApLogonUserEx\0"
#define LSA_AP_NAME_CALL_PACKAGE            "LsaApCallPackage\0"
#define LSA_AP_NAME_LOGON_TERMINATED        "LsaApLogonTerminated\0"
#define LSA_AP_NAME_CALL_PACKAGE_UNTRUSTED  "LsaApCallPackageUntrusted\0"
#define LSA_AP_NAME_CALL_PACKAGE_PASSTHROUGH "LsaApCallPackagePassthrough\0"


//
// Routine templates
//


typedef NTSTATUS
(NTAPI LSA_AP_INITIALIZE_PACKAGE) (
    _In_ ULONG AuthenticationPackageId,
    _In_ PLSA_DISPATCH_TABLE LsaDispatchTable,
    _In_opt_ PLSA_STRING Database,
    _In_opt_ PLSA_STRING Confidentiality,
    _Out_ PLSA_STRING *AuthenticationPackageName
    );

typedef NTSTATUS
(NTAPI LSA_AP_LOGON_USER) (
    _In_ PLSA_CLIENT_REQUEST ClientRequest,
    _In_ SECURITY_LOGON_TYPE LogonType,
    _In_reads_bytes_(AuthenticationInformationLength) PVOID AuthenticationInformation,
    _In_ PVOID ClientAuthenticationBase,
    _In_ ULONG AuthenticationInformationLength,
    _Outptr_result_bytebuffer_(*ProfileBufferLength) PVOID *ProfileBuffer,
    _Out_ PULONG ProfileBufferLength,
    _Out_ PLUID LogonId,
    _Out_ PNTSTATUS SubStatus,
    _Out_ PLSA_TOKEN_INFORMATION_TYPE TokenInformationType,
    _Outptr_ PVOID *TokenInformation,
    _Out_ PLSA_UNICODE_STRING *AccountName,
    _Out_ PLSA_UNICODE_STRING *AuthenticatingAuthority
    );

typedef NTSTATUS
(NTAPI LSA_AP_LOGON_USER_EX) (
    _In_ PLSA_CLIENT_REQUEST ClientRequest,
    _In_ SECURITY_LOGON_TYPE LogonType,
    _In_reads_bytes_(AuthenticationInformationLength) PVOID AuthenticationInformation,
    _In_ PVOID ClientAuthenticationBase,
    _In_ ULONG AuthenticationInformationLength,
    _Outptr_result_bytebuffer_(*ProfileBufferLength) PVOID *ProfileBuffer,
    _Out_ PULONG ProfileBufferLength,
    _Out_ PLUID LogonId,
    _Out_ PNTSTATUS SubStatus,
    _Out_ PLSA_TOKEN_INFORMATION_TYPE TokenInformationType,
    _Outptr_ PVOID *TokenInformation,
    _Out_ PUNICODE_STRING *AccountName,
    _Out_ PUNICODE_STRING *AuthenticatingAuthority,
    _Out_ PUNICODE_STRING *MachineName
    );

typedef NTSTATUS
(NTAPI LSA_AP_CALL_PACKAGE) (
    _In_ PLSA_CLIENT_REQUEST ClientRequest,
    _In_reads_bytes_(SubmitBufferLength) PVOID ProtocolSubmitBuffer,
    _In_ PVOID ClientBufferBase,
    _In_ ULONG SubmitBufferLength,
    _Outptr_result_bytebuffer_(*ReturnBufferLength) PVOID *ProtocolReturnBuffer,
    _Out_ PULONG ReturnBufferLength,
    _Out_ PNTSTATUS ProtocolStatus
    );

typedef NTSTATUS
(NTAPI LSA_AP_CALL_PACKAGE_PASSTHROUGH) (
    _In_ PLSA_CLIENT_REQUEST ClientRequest,
    _In_reads_bytes_(SubmitBufferLength) PVOID ProtocolSubmitBuffer,
    _In_ PVOID ClientBufferBase,
    _In_ ULONG SubmitBufferLength,
    _Outptr_result_bytebuffer_(*ReturnBufferLength) PVOID *ProtocolReturnBuffer,
    _Out_ PULONG ReturnBufferLength,
    _Out_ PNTSTATUS ProtocolStatus
    );

typedef VOID
(NTAPI LSA_AP_LOGON_TERMINATED) (
    _In_ PLUID LogonId
    );

typedef LSA_AP_CALL_PACKAGE LSA_AP_CALL_PACKAGE_UNTRUSTED;

typedef LSA_AP_INITIALIZE_PACKAGE * PLSA_AP_INITIALIZE_PACKAGE;
typedef LSA_AP_LOGON_USER * PLSA_AP_LOGON_USER;
typedef LSA_AP_LOGON_USER_EX * PLSA_AP_LOGON_USER_EX;
typedef LSA_AP_CALL_PACKAGE * PLSA_AP_CALL_PACKAGE;
typedef LSA_AP_CALL_PACKAGE_PASSTHROUGH * PLSA_AP_CALL_PACKAGE_PASSTHROUGH;
typedef LSA_AP_LOGON_TERMINATED * PLSA_AP_LOGON_TERMINATED;
typedef LSA_AP_CALL_PACKAGE_UNTRUSTED * PLSA_AP_CALL_PACKAGE_UNTRUSTED;


#ifndef _SAM_CREDENTIAL_UPDATE_DEFINED
#define _SAM_CREDENTIAL_UPDATE_DEFINED

typedef NTSTATUS (*PSAM_CREDENTIAL_UPDATE_NOTIFY_ROUTINE) (
    _In_                                   PUNICODE_STRING ClearPassword,
    _In_reads_bytes_(OldCredentialSize)         PVOID OldCredentials,
    _In_                                   ULONG OldCredentialSize,
    _In_                                   ULONG UserAccountControl,
    _In_opt_                               PUNICODE_STRING UPN,
    _In_                                   PUNICODE_STRING UserName,
    _In_                                   PUNICODE_STRING NetbiosDomainName,
    _In_                                   PUNICODE_STRING DnsDomainName,
    _Outptr_result_bytebuffer_(*NewCredentialSize) PVOID * NewCredentials,
    _Out_                                  ULONG * NewCredentialSize
    );

#define SAM_CREDENTIAL_UPDATE_NOTIFY_ROUTINE "CredentialUpdateNotify"

typedef BOOLEAN (*PSAM_CREDENTIAL_UPDATE_REGISTER_ROUTINE) (
    _Out_ PUNICODE_STRING CredentialName
    );

#define SAM_CREDENTIAL_UPDATE_REGISTER_ROUTINE "CredentialUpdateRegister"

typedef VOID (*PSAM_CREDENTIAL_UPDATE_FREE_ROUTINE) (
    _In_ PVOID p
    );

#define SAM_CREDENTIAL_UPDATE_FREE_ROUTINE "CredentialUpdateFree"

typedef struct {
    PSTR   Original;
    PSTR   Mapped;
    BOOLEAN Continuable;  // only honored for some operations
} SAM_REGISTER_MAPPING_ELEMENT, *PSAM_REGISTER_MAPPING_ELEMENT;

typedef struct {
                    ULONG                           Count;
    _Field_size_(Count) PSAM_REGISTER_MAPPING_ELEMENT   Elements;
} SAM_REGISTER_MAPPING_LIST, *PSAM_REGISTER_MAPPING_LIST;

typedef struct {
                    ULONG                          Count;
    _Field_size_(Count) PSAM_REGISTER_MAPPING_LIST     Lists;
} SAM_REGISTER_MAPPING_TABLE, *PSAM_REGISTER_MAPPING_TABLE;

typedef NTSTATUS (*PSAM_CREDENTIAL_UPDATE_REGISTER_MAPPED_ENTRYPOINTS_ROUTINE) (
    _Out_ SAM_REGISTER_MAPPING_TABLE *Table
    );

#define SAM_CREDENTIAL_UPDATE_REGISTER_MAPPED_ENTRYPOINTS_ROUTINE "RegisterMappedEntrypoints"

#endif // _SAM_CREDENTIAL_UPDATE_DEFINED


#ifdef SECURITY_KERNEL
//
// Can't use the windows.h def'ns in kernel mode.
//
typedef PVOID                   SEC_THREAD_START;
typedef PVOID                   SEC_ATTRS;
#else
typedef LPTHREAD_START_ROUTINE  SEC_THREAD_START;
typedef LPSECURITY_ATTRIBUTES   SEC_ATTRS;
#endif


#define SecEqualLuid(L1, L2)    \
            ( ( ((PLUID)L1)->LowPart == ((PLUID)L2)->LowPart ) && \
              ( ((PLUID)L1)->HighPart == ((PLUID)L2)->HighPart ) ) \

#define SecIsZeroLuid( L1 ) \
            ( ( (L1)->LowPart | (L1)->HighPart ) == 0 )

//
// The following structures are used by the helper functions
//

typedef struct _SECPKG_CLIENT_INFO {
    LUID            LogonId;            // Effective Logon Id
    ULONG           ProcessID;          // Process Id of caller
    ULONG           ThreadID;           // Thread Id of caller
    BOOLEAN         HasTcbPrivilege;    // Client has TCB
    BOOLEAN         Impersonating;      // Client is impersonating
    BOOLEAN         Restricted;         // Client is restricted

    //
    // NT 5.1
    //

    UCHAR                           ClientFlags;            // Extra flags about the client
    SECURITY_IMPERSONATION_LEVEL    ImpersonationLevel;     // Impersonation level of client

    //
    // NT 6
    //

    HANDLE                          ClientToken;

} SECPKG_CLIENT_INFO, * PSECPKG_CLIENT_INFO;

typedef struct _SECPKG_CLIENT_INFO_EX {
    LUID            LogonId;            // Effective Logon Id
    ULONG           ProcessID;          // Process Id of caller
    ULONG           ThreadID;           // Thread Id of caller
    BOOLEAN         HasTcbPrivilege;    // Client has TCB
    BOOLEAN         Impersonating;      // Client is impersonating
    BOOLEAN         Restricted;         // Client is restricted

    UCHAR                           ClientFlags;            // Extra flags about the client
    SECURITY_IMPERSONATION_LEVEL    ImpersonationLevel;     // Impersonation level of client

    HANDLE                          ClientToken;

    LUID                            IdentificationLogonId;
    HANDLE                          IdentificationToken;

} SECPKG_CLIENT_INFO_EX, * PSECPKG_CLIENT_INFO_EX;

#define SECPKG_CLIENT_PROCESS_TERMINATED    0x01    // The client process has terminated
#define SECPKG_CLIENT_THREAD_TERMINATED     0x02    // The client thread has terminated

typedef struct _SECPKG_CALL_INFO {
    ULONG           ProcessId;
    ULONG           ThreadId;
    ULONG           Attributes;
    ULONG           CallCount;
    PVOID           MechOid; // mechanism objection identifer
} SECPKG_CALL_INFO, * PSECPKG_CALL_INFO;


#define SECPKG_CALL_KERNEL_MODE     0x00000001  // Call originated in kernel mode
#define SECPKG_CALL_ANSI            0x00000002  // Call came from ANSI stub
#define SECPKG_CALL_URGENT          0x00000004  // Call designated urgent
#define SECPKG_CALL_RECURSIVE       0x00000008  // Call is recursing
#define SECPKG_CALL_IN_PROC         0x00000010  // Call originated in process
#define SECPKG_CALL_CLEANUP         0x00000020  // Call is cleanup from a client
#define SECPKG_CALL_WOWCLIENT       0x00000040  // Call is from a WOW client process
#define SECPKG_CALL_THREAD_TERM     0x00000080  // Call is from a thread that has term'd
#define SECPKG_CALL_PROCESS_TERM    0x00000100  // Call is from a process that has term'd
#define SECPKG_CALL_IS_TCB          0x00000200  // Call is from TCB
#define SECPKG_CALL_NETWORK_ONLY    0x00000400  // Call asks for network logon only, no cached logons
#define SECPKG_CALL_WINLOGON        0x00000800  // the caller of LsaLogonuser() is Winlogon
#define SECPKG_CALL_ASYNC_UPDATE    0x00001000  // asynchronous update for unlock
#define SECPKG_CALL_SYSTEM_PROC     0x00002000  // Call originated from the System process
#define SECPKG_CALL_NEGO            0x00004000  // Called by SPNEGO
#define SECPKG_CALL_NEGO_EXTENDER   0x00008000  // Called by NEGO extender
#define SECPKG_CALL_BUFFER_MARSHAL  0x00010000  // Buffer passed is marshaled (by RPC)
#define SECPKG_CALL_UNLOCK          0x00020000  // Unlock
#define SECPKG_CALL_CLOUDAP_CONNECT 0x00040000  // the caller of LsaLogonuser() is CloudAP during connection flow

//
// WOWXX: Additional defines to determine which type of WoW guest we are dealing with.
//
#define SECPKG_CALL_WOWX86          0x00000040
#define SECPKG_CALL_WOWA32          0x00040000

// Whenever Negotiate goes through different packages, it chooses which packages to try and if the error returned from the packages warrants a breakout. 
// The packages themselves, however, do not know the reason why the previous packages could not handle the logon. This is good for most cases, 
// but it's possible that package A wants package B to know why it failed so that package B can either change its behavior or log it. 
// The ledger here is optional and controlled via LSA. The contract is simple: 
// 1. To set the information, a package calls the LSA function LsaSetSecpkgFailureReason(). 
// 2. To get the information, a package calls the LSA function LsaGetSecpkgFailureReason().

// These enums are for special cases that might not be apparent based on the returned NTSTATUS;
typedef enum _SECPKG_FAILURE_SPECIAL_REASON {
    SecpkgFailureReason_Unknown = 0,    // There was an unknown failure reported.
    SecpkgFailureReason_NoFailure,      // There was no special failure reported.
    SecpkgFailureReason_LocalAccount,   // The client account was a local account.
    SecpkgFailureReason_DomainAccount,  // The client account was a domain account.
    SecpkgFailureReason_CloudAccount,   // The client account was a cloud account.
    SecpkgFailureReason_NullTarget,     // The targetname used during InitializeSecurityContext was null.
    SecpkgFailureReason_UnknownTarget,  // The targetname used during InitializeSecurityContext could not be resolved.
    SecpkgFailureReason_IpAddress,      // The targetname used during InitializeSecurityContext contained an IP Address.
    SecpkgFailureReason_DupTarget,      // The targetname used during InitializeSecurityContext has duplicates. E.g. duplicate SPS in AD. 
    SecpkgFailureReason_NoLineOfSight,  // The secpkg needed a line-of-sight to a Domain Controller, but none could be found. 
    SecpkgFailureReason_Loopback,       // The secpkg does not support loopback authentication.
    SecpkgFailureReason_NullSession,    // The secpkg does not handle null sessions.
} SECPKG_FAILURE_SPECIAL_REASON, * PSECPKG_FAILURE_SPECIAL_REASON;

typedef struct _SECPKG_FAILURE_REASON {
    NTSTATUS Status;
    SECPKG_FAILURE_SPECIAL_REASON Reason;
} SECPKG_FAILURE_REASON, * PSECPKG_FAILURE_REASON;

typedef struct _SECPKG_SUPPLEMENTAL_CRED {
    UNICODE_STRING PackageName;
    ULONG CredentialSize;
#ifdef MIDL_PASS
    [size_is(CredentialSize)]
#endif // MIDL_PASS
    PUCHAR Credentials;
} SECPKG_SUPPLEMENTAL_CRED, *PSECPKG_SUPPLEMENTAL_CRED;

typedef struct _SECPKG_BYTE_VECTOR
{
    ULONG ByteArrayOffset; // each element is a byte
    USHORT ByteArrayLength;
} SECPKG_BYTE_VECTOR, *PSECPKG_BYTE_VECTOR;

typedef struct _SECPKG_SHORT_VECTOR
{
    ULONG ShortArrayOffset; // each element is a short
    USHORT ShortArrayCount; // number of characters
} SECPKG_SHORT_VECTOR, *PSECPKG_SHORT_VECTOR;

//
// the supplied credential structure
//

typedef struct _SECPKG_SUPPLIED_CREDENTIAL {
    USHORT cbHeaderLength; // the length of the header
    USHORT cbStructureLength; //  pay load length including the header
    SECPKG_SHORT_VECTOR UserName; // unicode only
    SECPKG_SHORT_VECTOR DomainName; // unicode only
    SECPKG_BYTE_VECTOR PackedCredentials; // SEC_WINNT_AUTH_PACKED_CREDENTIALS
    ULONG CredFlags; // authidentity flags
} SECPKG_SUPPLIED_CREDENTIAL, *PSECPKG_SUPPLIED_CREDENTIAL;

//
// the credential structure used by Nego2-SPMI
//

#define SECPKG_CREDENTIAL_VERSION  201

//
//  credentials flags
//

#define SECPKG_CREDENTIAL_FLAGS_CALLER_HAS_TCB 0x1
#define SECPKG_CREDENTIAL_FLAGS_CREDMAN_CRED   0x2

typedef struct _SECPKG_CREDENTIAL {
    ULONG64 Version; // contains SECPKG_CREDENTIAL_VERSION
    USHORT cbHeaderLength;   // the length of the header
    ULONG cbStructureLength; // pay load length including the header,
    // all the content of this structure is within a contiguous buffer
    ULONG ClientProcess; // the caller's identity
    ULONG ClientThread;  // the caller's identity
    LUID LogonId;        // the caller's identity
    HANDLE ClientToken;  // the caller's identity
    ULONG SessionId;     // the caller's identity
    LUID ModifiedId;     // the caller's identity
    ULONG fCredentials;  // inbound or outbound?
    ULONG Flags;  // contains SECPKG_CREDENTIAL_FLAGS
    SECPKG_BYTE_VECTOR PrincipalName; // not used
    SECPKG_BYTE_VECTOR PackageList;   // list of packages, relevant only to SPNEGO
    SECPKG_BYTE_VECTOR MarshaledSuppliedCreds; // contains a SECPKG_SUPPLIED_CREDENTIAL structure
} SECPKG_CREDENTIAL, *PSECPKG_CREDENTIAL;

typedef ULONG_PTR LSA_SEC_HANDLE;
typedef LSA_SEC_HANDLE * PLSA_SEC_HANDLE;
typedef struct _SECPKG_SUPPLEMENTAL_CRED_ARRAY {
    ULONG CredentialCount;
#ifdef MIDL_PASS
    [size_is(CredentialCount)] SECPKG_SUPPLEMENTAL_CRED Credentials[*];
#else // MIDL_PASS
    SECPKG_SUPPLEMENTAL_CRED Credentials[1];
#endif // MIDL_PASS
} SECPKG_SUPPLEMENTAL_CRED_ARRAY, *PSECPKG_SUPPLEMENTAL_CRED_ARRAY;

typedef struct _SECPKG_SURROGATE_LOGON_ENTRY {
    GUID Type;
    PVOID Data;
} SECPKG_SURROGATE_LOGON_ENTRY, *PSECPKG_SURROGATE_LOGON_ENTRY;

typedef struct _SECPKG_SURROGATE_LOGON {
    ULONG Version;
    LUID SurrogateLogonID;
    ULONG EntryCount;
    PSECPKG_SURROGATE_LOGON_ENTRY Entries;
} SECPKG_SURROGATE_LOGON, *PSECPKG_SURROGATE_LOGON;

#define SECPKG_SURROGATE_LOGON_VERSION_1 1

//
// This flag is used for to indicate which buffers in the LSA are located
// in the client's address space
//

#define SECBUFFER_UNMAPPED      0x40000000

//
// This flag is used to indicate that the buffer was mapped into the LSA
// from kernel mode.
//

#define SECBUFFER_KERNEL_MAP    0x20000000

typedef NTSTATUS
(NTAPI LSA_CALLBACK_FUNCTION)(
    ULONG_PTR    Argument1,
    ULONG_PTR    Argument2,
    PSecBuffer  InputBuffer,
    PSecBuffer  OutputBuffer
    );

typedef LSA_CALLBACK_FUNCTION * PLSA_CALLBACK_FUNCTION;



#define PRIMARY_CRED_CLEAR_PASSWORD                 0x00000001
#define PRIMARY_CRED_OWF_PASSWORD                   0x00000002
#define PRIMARY_CRED_UPDATE                         0x00000004  // this is a change of existing creds
#define PRIMARY_CRED_CACHED_LOGON                   0x00000008
#define PRIMARY_CRED_LOGON_NO_TCB                   0x00000010
#define PRIMARY_CRED_LOGON_LUA                      0x00000020
#define PRIMARY_CRED_INTERACTIVE_SMARTCARD_LOGON    0x00000040
#define PRIMARY_CRED_REFRESH_NEEDED                 0x00000080  // unlock refresh needed
#define PRIMARY_CRED_INTERNET_USER                  0x00000100  // online identity credential, consumer accounts like MSA
#define PRIMARY_CRED_AUTH_ID                        0x00000200  // credential is unencrypted SEC_WINNT_AUTH_IDENTITY_EX2
#define PRIMARY_CRED_DO_NOT_SPLIT                   0x00000400
#define PRIMARY_CRED_PROTECTED_USER                 0x00000800
#define PRIMARY_CRED_EX                             0x00001000  // SECPKG_PRIMARY_CRED_EX
#define PRIMARY_CRED_TRANSFER                       0x00002000  // transfer credential
#define PRIMARY_CRED_RESTRICTED_TS                  0x00004000  // restricted TS
#define PRIMARY_CRED_PACKED_CREDS                   0x00008000  // PSEC_WINNT_AUTH_PACKED_CREDENTIALS
#define PRIMARY_CRED_ENTERPRISE_INTERNET_USER       0x00010000  // online identity credential, enterprise accounts like AAD
#define PRIMARY_CRED_ENCRYPTED_CREDGUARD_PASSWORD   0x00020000  // password is encrypted by CredGuard
#define PRIMARY_CRED_CACHED_INTERACTIVE_LOGON       0x00040000  // the actual logon type seen by the SSP was CachedInteractive
                                                                // Nego does optimized logon and async online logon for Interactive Logon type
                                                                // sent via Winlogon. LSA distributes the original LogonType to SSPs for their
                                                                // SpAcceptCredentials() function, so the SSPs don't know if the logon was optimized
                                                                // or online. However if the logon package adds this flag then SSPs know for sure
                                                                // that the logon was cached interactive(no network) and the absence of this flag
                                                                // denotes network logon was attempted.
#define PRIMARY_CRED_INTERACTIVE_NGC_LOGON          0x00080000
#define PRIMARY_CRED_INTERACTIVE_FIDO_LOGON         0x00100000
#define PRIMARY_CRED_ARSO_LOGON                     0x00200000
#define PRIMARY_CRED_SUPPLEMENTAL                   0x00400000  // The update is only to move supplemental credentials around
                                                                // all primary credentials fields except the LogonId should be ignored
#define PRIMARY_CRED_FOR_PASSWORD_CHANGE            0x00800000  // The credential will be used for a password change
#define PRIMARY_CRED_LOCAL_USER                     0x01000000  // The credential is for a local user

#define PRIMARY_CRED_LOGON_PACKAGE_SHIFT            24
#define PRIMARY_CRED_PACKAGE_MASK                   0xff000000

//
// For cached logons, the RPC id of the package doing the logon is identified
// by shifting the flags to the right by the PRIMARY_CRED_LOGON_PACKAGE_SHIFT.
//

typedef struct _SECPKG_PRIMARY_CRED {
    LUID LogonId;
    UNICODE_STRING DownlevelName;   // Sam Account Name
    UNICODE_STRING DomainName;      // Netbios domain name where account is located
    UNICODE_STRING Password;
    UNICODE_STRING OldPassword;
    PSID UserSid;
    ULONG Flags;
    UNICODE_STRING DnsDomainName;   // DNS domain name where account is located (if known)
    UNICODE_STRING Upn;             // UPN of account (if known)

    UNICODE_STRING LogonServer;
    UNICODE_STRING Spare1;
    UNICODE_STRING Spare2;
    UNICODE_STRING Spare3;
    UNICODE_STRING Spare4;
} SECPKG_PRIMARY_CRED, *PSECPKG_PRIMARY_CRED;

//
// Creating an extension for SECPKG_PRIMARY_CRED->Flags field
//

#define SECPKG_PRIMARY_CRED_EX_FLAGS_EX_DELEGATION_TOKEN     0x1

//
// SECPKG_PRIMARY_CRED_EX has the same layout of SECPKG_PRIMARY_CRED for existing fields.
// this is designed to work with existing code transparently.
//

typedef struct _SECPKG_PRIMARY_CRED_EX {
    LUID LogonId;
    UNICODE_STRING DownlevelName;   // Sam Account Name
    UNICODE_STRING DomainName;      // Netbios domain name where account is located
    UNICODE_STRING Password;
    UNICODE_STRING OldPassword;
    PSID UserSid;
    ULONG Flags;
    UNICODE_STRING DnsDomainName;   // DNS domain name where account is located (if known)
    UNICODE_STRING Upn;             // UPN of account (if known)

    UNICODE_STRING LogonServer;
    UNICODE_STRING Spare1;
    UNICODE_STRING Spare2;
    UNICODE_STRING Spare3;
    UNICODE_STRING Spare4;

    //
    // extensions
    //

    ULONG_PTR      PackageId;       // originating package
    LUID           PrevLogonId;     // if not zero, the logon having up-to-date credential
                                    // system wide.
    ULONG          FlagsEx;         // See SECPKG_PRIMARY_CRED_EX_FLAGS_EX_* for potential values
} SECPKG_PRIMARY_CRED_EX, *PSECPKG_PRIMARY_CRED_EX;

//
// Maximum size of stored credentials.
//

#define MAX_CRED_SIZE 1024

// Values for MachineState

#define SECPKG_STATE_ENCRYPTION_PERMITTED               0x01
#define SECPKG_STATE_STRONG_ENCRYPTION_PERMITTED        0x02
#define SECPKG_STATE_DOMAIN_CONTROLLER                  0x04
#define SECPKG_STATE_WORKSTATION                        0x08
#define SECPKG_STATE_STANDALONE                         0x10
#define SECPKG_STATE_CRED_ISOLATION_ENABLED             0x20
#define SECPKG_STATE_RESERVED_1                   0x80000000

typedef struct _SECPKG_PARAMETERS {
    ULONG           Version;
    ULONG           MachineState;
    ULONG           SetupMode;
    PSID            DomainSid;
    UNICODE_STRING  DomainName;
    UNICODE_STRING  DnsDomainName;
    GUID            DomainGuid;
} SECPKG_PARAMETERS, *PSECPKG_PARAMETERS;


//
// Extended Package information structures
//

typedef enum _SECPKG_EXTENDED_INFORMATION_CLASS {
    SecpkgGssInfo = 1,
    SecpkgContextThunks,
    SecpkgMutualAuthLevel,
    SecpkgWowClientDll,
    SecpkgExtraOids,
    SecpkgMaxInfo,
    SecpkgNego2Info,
} SECPKG_EXTENDED_INFORMATION_CLASS;

typedef struct _SECPKG_GSS_INFO {
    ULONG   EncodedIdLength;
    UCHAR   EncodedId[4];
} SECPKG_GSS_INFO, * PSECPKG_GSS_INFO;

typedef struct _SECPKG_CONTEXT_THUNKS {
    ULONG   InfoLevelCount;
    ULONG   Levels[1];
} SECPKG_CONTEXT_THUNKS, *PSECPKG_CONTEXT_THUNKS;

typedef struct _SECPKG_MUTUAL_AUTH_LEVEL {
    ULONG   MutualAuthLevel;
} SECPKG_MUTUAL_AUTH_LEVEL, * PSECPKG_MUTUAL_AUTH_LEVEL;

typedef struct _SECPKG_WOW_CLIENT_DLL {
    SECURITY_STRING WowClientDllPath;
} SECPKG_WOW_CLIENT_DLL, * PSECPKG_WOW_CLIENT_DLL;

#define SECPKG_MAX_OID_LENGTH   32

typedef struct _SECPKG_SERIALIZED_OID {
    ULONG OidLength;
    ULONG OidAttributes;
    UCHAR OidValue[ SECPKG_MAX_OID_LENGTH ];
} SECPKG_SERIALIZED_OID, * PSECPKG_SERIALIZED_OID;

typedef struct _SECPKG_EXTRA_OIDS {
    ULONG   OidCount;
    SECPKG_SERIALIZED_OID Oids[ 1 ];
} SECPKG_EXTRA_OIDS, * PSECPKG_EXTRA_OIDS;

// used by Nego2
typedef struct _SECPKG_NEGO2_INFO {
    UCHAR AuthScheme[16]; // auth id
    ULONG PackageFlags;
} SECPKG_NEGO2_INFO, * PSECPKG_NEGO2_INFO;

typedef struct _SECPKG_EXTENDED_INFORMATION {
    SECPKG_EXTENDED_INFORMATION_CLASS   Class;
    union {
        SECPKG_GSS_INFO            GssInfo;
        SECPKG_CONTEXT_THUNKS      ContextThunks;
        SECPKG_MUTUAL_AUTH_LEVEL   MutualAuthLevel;
        SECPKG_WOW_CLIENT_DLL      WowClientDll;
        SECPKG_EXTRA_OIDS          ExtraOids;
        SECPKG_NEGO2_INFO          Nego2Info;
    } Info;
} SECPKG_EXTENDED_INFORMATION, * PSECPKG_EXTENDED_INFORMATION;

typedef struct  _SECPKG_TARGETINFO
{
    PSID    DomainSid;
    PCWSTR  ComputerName;
} SECPKG_TARGETINFO, *PSECPKG_TARGETINFO;

// Flag values for SECPKG_NTLM_TARGETINFO.Flags field below.
#define SECPKG_MSVAV_FLAGS_VALID              0x01
#define SECPKG_MSVAV_TIMESTAMP_VALID          0x02

typedef struct  _SECPKG_NTLM_TARGETINFO
{
    // Flags contains zero or SECPKG_MSVAV_* values from above
    ULONG    Flags;

    LPWSTR   MsvAvNbComputerName;
    LPWSTR   MsvAvNbDomainName;
    LPWSTR   MsvAvDnsComputerName;
    LPWSTR   MsvAvDnsDomainName;
    LPWSTR   MsvAvDnsTreeName;
    ULONG    MsvAvFlags;
    FILETIME MsvAvTimestamp;
    LPWSTR   MsvAvTargetName;
} SECPKG_NTLM_TARGETINFO, *PSECPKG_NTLM_TARGETINFO;

#define SECPKG_ATTR_SASL_CONTEXT    0x00010000

typedef struct _SecPkgContext_SaslContext {
    PVOID   SaslContext;
} SecPkgContext_SaslContext, * PSecPkgContext_SaslContext;

//
// Setting this value as the first context thunk value will cause all
// calls to go to the LSA:
//

#define SECPKG_ATTR_THUNK_ALL   0x00010000


#ifndef SECURITY_USER_DATA_DEFINED
#define SECURITY_USER_DATA_DEFINED

typedef struct _SECURITY_USER_DATA {
    SECURITY_STRING UserName;           // User name
    SECURITY_STRING LogonDomainName;    // Domain the user logged on to
    SECURITY_STRING LogonServer;        // Server that logged the user on
    PSID            pSid;               // SID of user
} SECURITY_USER_DATA, *PSECURITY_USER_DATA;

typedef SECURITY_USER_DATA SecurityUserData, * PSecurityUserData;

#define UNDERSTANDS_LONG_NAMES  1
#define NO_LONG_NAMES           2

#endif // SECURITY_USER_DATA_DEFINED

//
// common call package interface
//

#define SECPKG_ALL_PACKAGES ((ULONG) -2)

//
// the message types defined here are applied to all the packages. the minimum
// message type value is set to 1024 to avoid conflict with any existing per
// package message type.
//

typedef enum _SECPKG_CALL_PACKAGE_MESSAGE_TYPE
{
    SecPkgCallPackageMinMessage = 1024,
    SecPkgCallPackagePinDcMessage = SecPkgCallPackageMinMessage,
    SecPkgCallPackageUnpinAllDcsMessage,
    SecPkgCallPackageTransferCredMessage,
    SecPkgCallPackageMaxMessage = SecPkgCallPackageTransferCredMessage,      // update when adding new messages
} SECPKG_CALL_PACKAGE_MESSAGE_TYPE, *PSECPKG_CALL_PACKAGE_MESSAGE_TYPE;

typedef struct _SECPKG_CALL_PACKAGE_PIN_DC_REQUEST
{
    ULONG          MessageType;
    ULONG          Flags;       // reserved, must be 0
    UNICODE_STRING DomainName;
    UNICODE_STRING DcName;
    ULONG          DcFlags;
} SECPKG_CALL_PACKAGE_PIN_DC_REQUEST, *PSECPKG_CALL_PACKAGE_PIN_DC_REQUEST;

typedef struct _SECPKG_CALL_PACKAGE_UNPIN_ALL_DCS_REQUEST
{
    ULONG          MessageType;
    ULONG          Flags;  // reserved, must be 0
} SECPKG_CALL_PACKAGE_UNPIN_ALL_DCS_REQUEST, *PSECPKG_CALL_PACKAGE_UNPIN_ALL_DCS_REQUEST;

#define SECPKG_CALL_PACKAGE_TRANSFER_CRED_REQUEST_FLAG_OPTIMISTIC_LOGON    0x1
#define SECPKG_CALL_PACKAGE_TRANSFER_CRED_REQUEST_FLAG_CLEANUP_CREDENTIALS 0x2
#define SECPKG_CALL_PACKAGE_TRANSFER_CRED_REQUEST_FLAG_TO_SSO_SESSION      0x4

typedef struct _SECPKG_CALL_PACKAGE_TRANSFER_CRED_REQUEST {
    ULONG       MessageType;
    LUID        OriginLogonId;
    LUID        DestinationLogonId;
    ULONG       Flags;
} SECPKG_CALL_PACKAGE_TRANSFER_CRED_REQUEST, *PSECPKG_CALL_PACKAGE_TRANSFER_CRED_REQUEST;


////////////////////////////////////////////////////////////////////////////
//                                                                        //
// Interface definitions for redirected use of credentials over a remote  //
// connection. Any packages supported by SPNego may work with remote      //
// credentials.                                                           //
//                                                                        //
////////////////////////////////////////////////////////////////////////////
#ifdef __cplusplus
extern "C" {
#endif

// Called to initialize the channel back to the system which handles
// credential operations.
typedef NTSTATUS
(NTAPI LSA_REDIRECTED_LOGON_INIT)(
        HANDLE RedirectedLogonHandle,
        const UNICODE_STRING* PackageName,
        ULONG SessionId,
        const LUID* LogonId
        );

// Used to send credential use requests to the server. The call information
// is packaged up inside of Buffer, and ReturnBuffer contains the response.
// The format of Buffer and ReturnBuffer are AP/SSP-specific, and are simply
// routed over the remote channel, as initialized by LSA_REDIRECTED_LOGON_INIT.
typedef NTSTATUS
(NTAPI LSA_REDIRECTED_LOGON_CALLBACK)(
        HANDLE RedirectedLogonHandle,
        PVOID Buffer,
        ULONG BufferLength,
        PVOID* ReturnBuffer,
        ULONG* ReturnBufferLength
        );

// Cleans up after a redirected logon is no longer needed. After this point,
// the remote credentials are no longer usable.
typedef VOID
(NTAPI LSA_REDIRECTED_LOGON_CLEANUP_CALLBACK)(
        HANDLE RedirectedLogonHandle
        );

// The authentication package handling the redirected logon should use
// this to retrieve its credential buffer.
typedef NTSTATUS
(NTAPI LSA_REDIRECTED_LOGON_GET_LOGON_CREDS)(
        HANDLE RedirectedLogonHandle,
        PBYTE* LogonBuffer,
        PULONG LogonBufferLength
        );

// The authentication package should use this to retrieve the supplemental
// credentials passed over a remote credential guard connection. These
// credentials are used to light up remoting for other security packages.
typedef NTSTATUS
(NTAPI LSA_REDIRECTED_LOGON_GET_SUPP_CREDS)(
        HANDLE RedirectedLogonHandle,
        PSECPKG_SUPPLEMENTAL_CRED_ARRAY* SupplementalCredentials
        );

// The authentication package should use this to retrieve the SID associated
// associated with the TSPkg logon session. This is intended to bind the NLA session to the interactive logon session
typedef NTSTATUS
(NTAPI LSA_REDIRECTED_LOGON_GET_SID)(
        HANDLE RedirectedLogonHandle,
        PSID* Sid
        );

#ifdef __cplusplus
} // extern "C"
#endif

typedef LSA_REDIRECTED_LOGON_INIT *PLSA_REDIRECTED_LOGON_INIT;
typedef LSA_REDIRECTED_LOGON_CALLBACK *PLSA_REDIRECTED_LOGON_CALLBACK;
typedef LSA_REDIRECTED_LOGON_GET_LOGON_CREDS *PLSA_REDIRECTED_LOGON_GET_LOGON_CREDS;
typedef LSA_REDIRECTED_LOGON_GET_SUPP_CREDS *PLSA_REDIRECTED_LOGON_GET_SUPP_CREDS;
typedef LSA_REDIRECTED_LOGON_CLEANUP_CALLBACK *PLSA_REDIRECTED_LOGON_CLEANUP_CALLBACK;

typedef LSA_REDIRECTED_LOGON_GET_SID *PLSA_REDIRECTED_LOGON_GET_SID;

#define SECPKG_REDIRECTED_LOGON_GUID_INITIALIZER { 0xc2be5457, 0x82eb, 0x483e, { 0xae, 0x4e, 0x74, 0x68, 0xef, 0x14, 0xd5, 0x9 } }
typedef struct _SECPKG_REDIRECTED_LOGON_BUFFER {
    GUID RedirectedLogonGuid;
    HANDLE RedirectedLogonHandle;
    PLSA_REDIRECTED_LOGON_INIT Init;
    PLSA_REDIRECTED_LOGON_CALLBACK Callback;
    PLSA_REDIRECTED_LOGON_CLEANUP_CALLBACK CleanupCallback;
    PLSA_REDIRECTED_LOGON_GET_LOGON_CREDS GetLogonCreds;
    PLSA_REDIRECTED_LOGON_GET_SUPP_CREDS GetSupplementalCreds;
    PLSA_REDIRECTED_LOGON_GET_SID GetRedirectedLogonSid;
} SECPKG_REDIRECTED_LOGON_BUFFER, *PSECPKG_REDIRECTED_LOGON_BUFFER;

typedef struct _SECPKG_POST_LOGON_USER_INFO
{
    ULONG  Flags;        // reserved
    LUID   LogonId;
    LUID   LinkedLogonId;
} SECPKG_POST_LOGON_USER_INFO, *PSECPKG_POST_LOGON_USER_INFO;

//////////////////////////////////////////////////////////////////////////
//
// The following prototypes are to functions that are provided by the SPMgr
// to security packages.
//
//////////////////////////////////////////////////////////////////////////

typedef NTSTATUS
(NTAPI LSA_IMPERSONATE_CLIENT) (
    VOID
    );


typedef NTSTATUS
(NTAPI LSA_UNLOAD_PACKAGE)(
    VOID
    );

typedef NTSTATUS
(NTAPI LSA_DUPLICATE_HANDLE)(
    _In_ HANDLE     SourceHandle,
    _Out_ PHANDLE   DestionationHandle);


typedef NTSTATUS
(NTAPI LSA_SAVE_SUPPLEMENTAL_CREDENTIALS)(
    _In_ PLUID LogonId,
    _In_ ULONG SupplementalCredSize,
    _In_reads_bytes_(SupplementalCredSize) PVOID SupplementalCreds,
    _In_ BOOLEAN Synchronous
    );


typedef HANDLE
(NTAPI LSA_CREATE_THREAD)(
    _In_ SEC_ATTRS SecurityAttributes,
    _In_ ULONG StackSize,
    _In_ SEC_THREAD_START StartFunction,
    _In_ PVOID ThreadParameter,
    _In_ ULONG CreationFlags,
    _Out_ PULONG ThreadId
    );


typedef NTSTATUS
(NTAPI LSA_GET_CLIENT_INFO)(
    _Out_ PSECPKG_CLIENT_INFO ClientInfo
    );

typedef NTSTATUS
(NTAPI LSA_GET_CLIENT_INFO_EX)(
    _Out_ PSECPKG_CLIENT_INFO_EX ClientInfo,
    _In_ ULONG StructSize
    );

typedef HANDLE
(NTAPI LSA_REGISTER_NOTIFICATION)(
    _In_ SEC_THREAD_START StartFunction,
    _In_opt_ PVOID Parameter,
    _In_ ULONG NotificationType,
    _In_ ULONG NotificationClass,
    _In_ ULONG NotificationFlags,
    _In_ ULONG IntervalMinutes,
    _In_opt_ HANDLE WaitEvent
    );


typedef NTSTATUS
(NTAPI LSA_CANCEL_NOTIFICATION)(
    _In_ HANDLE NotifyHandle
    );

typedef NTSTATUS
(NTAPI LSA_MAP_BUFFER)(
    _In_ PSecBuffer InputBuffer,
    _Out_ PSecBuffer OutputBuffer
    );

typedef NTSTATUS
(NTAPI LSA_CREATE_TOKEN) (
    _In_ PLUID LogonId,
    _In_ PTOKEN_SOURCE TokenSource,
    _In_ SECURITY_LOGON_TYPE LogonType,
    _In_ SECURITY_IMPERSONATION_LEVEL ImpersonationLevel,
    _In_ LSA_TOKEN_INFORMATION_TYPE TokenInformationType,
    _In_ PVOID TokenInformation,
    _In_opt_ PTOKEN_GROUPS TokenGroups,
    _In_ PUNICODE_STRING AccountName,
    _In_ PUNICODE_STRING AuthorityName,
    _In_opt_ PUNICODE_STRING Workstation,
    _In_opt_ PUNICODE_STRING ProfilePath,
    _Out_ PHANDLE Token,
    _Out_ PNTSTATUS SubStatus
    );

typedef enum _SECPKG_SESSIONINFO_TYPE {
    SecSessionPrimaryCred       // SessionInformation is SECPKG_PRIMARY_CRED
} SECPKG_SESSIONINFO_TYPE;

typedef NTSTATUS
(NTAPI LSA_CREATE_TOKEN_EX) (
    _In_ PLUID LogonId,
    _In_ PTOKEN_SOURCE TokenSource,
    _In_ SECURITY_LOGON_TYPE LogonType,
    _In_ SECURITY_IMPERSONATION_LEVEL ImpersonationLevel,
    _In_ LSA_TOKEN_INFORMATION_TYPE TokenInformationType,
    _In_ PVOID TokenInformation,
    _In_opt_ PTOKEN_GROUPS TokenGroups,
    _In_opt_ PUNICODE_STRING Workstation,
    _In_opt_ PUNICODE_STRING ProfilePath,
    _In_ PVOID SessionInformation,
    _In_ SECPKG_SESSIONINFO_TYPE SessionInformationType,
    _Out_ PHANDLE Token,
    _Out_ PNTSTATUS SubStatus
    );

typedef VOID
(NTAPI LSA_AUDIT_LOGON) (
    _In_ NTSTATUS Status,
    _In_ NTSTATUS SubStatus,
    _In_opt_ PUNICODE_STRING AccountName,
    _In_opt_ PUNICODE_STRING AuthenticatingAuthority,
    _In_opt_ PUNICODE_STRING WorkstationName,
    _In_opt_ PSID UserSid,
    _In_ SECURITY_LOGON_TYPE LogonType,
    _In_ PTOKEN_SOURCE TokenSource,
    _In_ PLUID LogonId
    );

typedef NTSTATUS
(NTAPI LSA_CALL_PACKAGE) (
    _In_ PUNICODE_STRING AuthenticationPackage,
    _In_reads_bytes_(SubmitBufferLength) PVOID ProtocolSubmitBuffer,
    _In_ ULONG SubmitBufferLength,
    _Outptr_result_bytebuffer_(*ReturnBufferLength) PVOID *ProtocolReturnBuffer,
    _Out_ PULONG ReturnBufferLength,
    _Out_ PNTSTATUS ProtocolStatus
    );

typedef NTSTATUS
(NTAPI LSA_CALL_PACKAGEEX) (
    _In_ PUNICODE_STRING AuthenticationPackage,
    _In_ PVOID ClientBufferBase,
    _In_reads_bytes_(SubmitBufferLength) PVOID ProtocolSubmitBuffer,
    _In_ ULONG SubmitBufferLength,
    _Outptr_result_bytebuffer_(*ReturnBufferLength) PVOID *ProtocolReturnBuffer,
    _Out_ PULONG ReturnBufferLength,
    _Out_ PNTSTATUS ProtocolStatus
    );

typedef NTSTATUS
(NTAPI LSA_CALL_PACKAGE_PASSTHROUGH) (
    _In_ PUNICODE_STRING AuthenticationPackage,
    _In_ PVOID ClientBufferBase,
    _In_reads_bytes_(SubmitBufferLength) PVOID ProtocolSubmitBuffer,
    _In_ ULONG SubmitBufferLength,
    _Outptr_result_bytebuffer_(*ReturnBufferLength) PVOID *ProtocolReturnBuffer,
    _Out_ PULONG ReturnBufferLength,
    _Out_ PNTSTATUS ProtocolStatus
    );

typedef BOOLEAN
(NTAPI LSA_GET_CALL_INFO) (
    _Out_ PSECPKG_CALL_INFO   Info
    );

typedef PVOID
(NTAPI LSA_CREATE_SHARED_MEMORY)(
    _In_ ULONG MaxSize,
    _In_ ULONG InitialSize
    );

typedef PVOID
(NTAPI LSA_ALLOCATE_SHARED_MEMORY)(
    _In_ PVOID SharedMem,
    _In_ ULONG Size
    );

typedef VOID
(NTAPI LSA_FREE_SHARED_MEMORY)(
    _In_    PVOID SharedMem,
    _Inout_opt_ PVOID Memory
    );

typedef BOOLEAN
(NTAPI LSA_DELETE_SHARED_MEMORY)(
    _In_ PVOID SharedMem
    );

typedef NTSTATUS
(NTAPI LSA_GET_APP_MODE_INFO)(
    _Out_opt_  PULONG      UserFunction,
    _Out_opt_  PULONG_PTR  Argument1,
    _Out_opt_  PULONG_PTR  Argument2,
    _Out_opt_  PSecBuffer  UserData,
    _Out_opt_  PBOOLEAN    ReturnToLsa
    );

typedef NTSTATUS
(NTAPI LSA_SET_APP_MODE_INFO)(
    _In_      ULONG      UserFunction,
    _In_opt_  ULONG_PTR  Argument1,
    _In_opt_  ULONG_PTR  Argument2,
    _In_opt_  PSecBuffer UserData,
    _In_opt_  BOOLEAN    ReturnToLsa
    );

typedef NTSTATUS
(NTAPI LSA_GET_SECPKG_FAILURE_REASON)(
    _In_ const ULONG_PTR PackageID,
    _Out_ SECPKG_FAILURE_REASON* Reason
    );

typedef NTSTATUS
(NTAPI LSA_SET_SECPKG_FAILURE_REASON)(
    _In_ const SECPKG_FAILURE_REASON Reason
    );

//
// Account Access
//

typedef enum _SECPKG_NAME_TYPE {
    SecNameSamCompatible,
    SecNameAlternateId,
    SecNameFlat,
    SecNameDN,
    SecNameSPN
} SECPKG_NAME_TYPE;

typedef NTSTATUS
(NTAPI LSA_OPEN_SAM_USER)(
    _In_ PSECURITY_STRING Name,
    _In_ SECPKG_NAME_TYPE NameType,
    _In_ PSECURITY_STRING Prefix,
    _In_ BOOLEAN AllowGuest,
    _In_ ULONG Reserved,
    _Out_ PVOID * UserHandle
    );

typedef NTSTATUS
(NTAPI LSA_GET_USER_CREDENTIALS)(
    _In_ PVOID UserHandle,
    _Outptr_result_bytebuffer_(*PrimaryCredsSize) PVOID * PrimaryCreds,
    _Out_ PULONG PrimaryCredsSize,
    _Outptr_result_bytebuffer_(*SupplementalCredsSize) PVOID * SupplementalCreds,
    _Out_ PULONG SupplementalCredsSize
    );

typedef NTSTATUS
(NTAPI LSA_GET_USER_AUTH_DATA)(
    _In_ PVOID UserHandle,
    _Outptr_result_bytebuffer_(*UserAuthDataSize) PUCHAR * UserAuthData,
    _Out_ PULONG UserAuthDataSize
    );

typedef NTSTATUS
(NTAPI LSA_CLOSE_SAM_USER)(
    _In_ PVOID UserHandle
    );

typedef NTSTATUS
(NTAPI LSA_GET_AUTH_DATA_FOR_USER)(
    _In_ PSECURITY_STRING Name,
    _In_ SECPKG_NAME_TYPE NameType,
    _In_opt_ PSECURITY_STRING Prefix,
    _Outptr_result_bytebuffer_(*UserAuthDataSize) PUCHAR * UserAuthData,
    _Out_ PULONG UserAuthDataSize,
    _Out_opt_ PUNICODE_STRING UserFlatName
    );

typedef NTSTATUS
(NTAPI LSA_CONVERT_AUTH_DATA_TO_TOKEN)(
    _In_ PVOID UserAuthData,
    _In_ ULONG UserAuthDataSize,
    _In_ SECURITY_IMPERSONATION_LEVEL ImpersonationLevel,
    _In_ PTOKEN_SOURCE TokenSource,
    _In_ SECURITY_LOGON_TYPE LogonType,
    _In_ PUNICODE_STRING AuthorityName,
    _Out_ PHANDLE Token,
    _Out_ PLUID LogonId,
    _Out_ PUNICODE_STRING AccountName,
    _Out_ PNTSTATUS SubStatus
    );

typedef NTSTATUS
(NTAPI LSA_CRACK_SINGLE_NAME)(
    _In_ ULONG FormatOffered,
    _In_ BOOLEAN PerformAtGC,
    _In_ PUNICODE_STRING NameInput,
    _In_opt_ PUNICODE_STRING Prefix,
    _In_ ULONG RequestedFormat,
    _Out_ PUNICODE_STRING CrackedName,
    _Out_ PUNICODE_STRING DnsDomainName,
    _Out_ PULONG SubStatus
    );

typedef NTSTATUS
(NTAPI LSA_AUDIT_ACCOUNT_LOGON)(
    _In_ ULONG AuditId,
    _In_ BOOLEAN Success,
    _In_opt_ PUNICODE_STRING Source,
    _In_opt_ PUNICODE_STRING ClientName,
    _In_opt_ PUNICODE_STRING MappedName,
    _In_ NTSTATUS Status
    );


typedef NTSTATUS
(NTAPI LSA_CLIENT_CALLBACK)(
    _In_  PCHAR      Callback,
    _In_  ULONG_PTR  Argument1,
    _In_  ULONG_PTR  Argument2,
    _In_  PSecBuffer Input,
    _Out_ PSecBuffer Output
    );

typedef
NTSTATUS
(NTAPI LSA_REGISTER_CALLBACK)(
    _In_ ULONG   CallbackId,
    _In_ PLSA_CALLBACK_FUNCTION Callback
    );

typedef NTSTATUS
(NTAPI LSA_GET_EXTENDED_CALL_FLAGS)(
    _Out_ PULONG Flags
    );



#define NOTIFIER_FLAG_NEW_THREAD    0x00000001
#define NOTIFIER_FLAG_ONE_SHOT      0x00000002
#define NOTIFIER_FLAG_SECONDS       0x80000000

#define NOTIFIER_TYPE_INTERVAL      1
#define NOTIFIER_TYPE_HANDLE_WAIT   2
#define NOTIFIER_TYPE_STATE_CHANGE  3
#define NOTIFIER_TYPE_NOTIFY_EVENT  4
#define NOTIFIER_TYPE_IMMEDIATE 16

#define NOTIFY_CLASS_PACKAGE_CHANGE     1
#define NOTIFY_CLASS_ROLE_CHANGE        2
#define NOTIFY_CLASS_DOMAIN_CHANGE      3
#define NOTIFY_CLASS_REGISTRY_CHANGE    4

typedef struct _SECPKG_EVENT_PACKAGE_CHANGE {
    ULONG   ChangeType;
    LSA_SEC_HANDLE  PackageId;
    SECURITY_STRING PackageName;
} SECPKG_EVENT_PACKAGE_CHANGE, * PSECPKG_EVENT_PACKAGE_CHANGE;

#define SECPKG_PACKAGE_CHANGE_LOAD      0
#define SECPKG_PACKAGE_CHANGE_UNLOAD    1
#define SECPKG_PACKAGE_CHANGE_SELECT    2

typedef struct _SECPKG_EVENT_ROLE_CHANGE {
    ULONG   PreviousRole;
    ULONG   NewRole;
} SECPKG_EVENT_ROLE_CHANGE, * PSECPKG_EVENT_ROLE_CHANGE;

typedef struct _SECPKG_PARAMETERS SECPKG_EVENT_DOMAIN_CHANGE;
typedef struct _SECPKG_PARAMETERS * PSECPKG_EVENT_DOMAIN_CHANGE;


typedef struct _SECPKG_EVENT_NOTIFY {
    ULONG EventClass;
    ULONG Reserved;
    ULONG EventDataSize;
    PVOID EventData;
    PVOID PackageParameter;
} SECPKG_EVENT_NOTIFY, *PSECPKG_EVENT_NOTIFY;


typedef
NTSTATUS
(NTAPI LSA_UPDATE_PRIMARY_CREDENTIALS)(
    _In_ PSECPKG_PRIMARY_CRED PrimaryCredentials,
    _In_opt_ PSECPKG_SUPPLEMENTAL_CRED_ARRAY Credentials
    );

typedef
VOID
(NTAPI LSA_PROTECT_MEMORY)(
    _Inout_updates_bytes_(BufferSize) PVOID Buffer,
    _In_ ULONG BufferSize
    );

typedef
NTSTATUS
(NTAPI LSA_OPEN_TOKEN_BY_LOGON_ID)(
    _In_ PLUID LogonId,
    _Out_ HANDLE *RetTokenHandle
    );

typedef
NTSTATUS
(NTAPI LSA_EXPAND_AUTH_DATA_FOR_DOMAIN)(
    _In_reads_bytes_(UserAuthDataSize) PUCHAR UserAuthData,
    _In_ ULONG UserAuthDataSize,
    _In_ PVOID Reserved,
    _Outptr_result_bytebuffer_(*ExpandedAuthDataSize) PUCHAR * ExpandedAuthData,
    _Out_ PULONG ExpandedAuthDataSize
    );

typedef enum _CRED_FETCH {
    CredFetchDefault = 0,
    CredFetchDPAPI,
    CredFetchForced
} CRED_FETCH, *PCRED_FETCH;

typedef NTSTATUS
(NTAPI LSA_GET_SERVICE_ACCOUNT_PASSWORD)(
    _In_ PUNICODE_STRING AccountName,
    _In_opt_ PUNICODE_STRING DomainName,
    _In_ CRED_FETCH CredFetch,
    _Inout_ FILETIME *FileTimeExpiry,
    _Out_ PUNICODE_STRING CurrentPassword,
    _Out_ PUNICODE_STRING PreviousPassword,
    _Out_opt_ FILETIME *FileTimeCurrPwdValidForOutbound);

typedef VOID
(NTAPI LSA_AUDIT_LOGON_EX) (
    _In_ NTSTATUS Status,
    _In_ NTSTATUS SubStatus,
    _In_opt_ PUNICODE_STRING AccountName,
    _In_opt_ PUNICODE_STRING AuthenticatingAuthority,
    _In_opt_ PUNICODE_STRING WorkstationName,
    _In_opt_ PSID UserSid,
    _In_ SECURITY_LOGON_TYPE LogonType,
    _In_ SECURITY_IMPERSONATION_LEVEL ImpersonationLevel,
    _In_ PTOKEN_SOURCE TokenSource,
    _In_ PLUID LogonId
    );

typedef NTSTATUS
(NTAPI LSA_CHECK_PROTECTED_USER_BY_TOKEN)(
    _In_  HANDLE   UserToken,
    _Out_ PBOOLEAN ProtectedUser
    );

typedef NTSTATUS
(NTAPI LSA_QUERY_CLIENT_REQUEST) (
    _In_ PLSA_CLIENT_REQUEST ClientRequest,
    _In_ ULONG QueryType,
    _Out_ PVOID* ReplyBuffer
    );

#define LSA_QUERY_CLIENT_PRELOGON_SESSION_ID 1

typedef LSA_IMPERSONATE_CLIENT * PLSA_IMPERSONATE_CLIENT;
typedef LSA_UNLOAD_PACKAGE * PLSA_UNLOAD_PACKAGE;
typedef LSA_DUPLICATE_HANDLE * PLSA_DUPLICATE_HANDLE;
typedef LSA_SAVE_SUPPLEMENTAL_CREDENTIALS * PLSA_SAVE_SUPPLEMENTAL_CREDENTIALS;
typedef LSA_CREATE_THREAD * PLSA_CREATE_THREAD;
typedef LSA_GET_CLIENT_INFO * PLSA_GET_CLIENT_INFO;
typedef LSA_GET_CLIENT_INFO_EX * PLSA_GET_CLIENT_INFO_EX;
typedef LSA_REGISTER_NOTIFICATION * PLSA_REGISTER_NOTIFICATION;
typedef LSA_CANCEL_NOTIFICATION * PLSA_CANCEL_NOTIFICATION;
typedef LSA_MAP_BUFFER * PLSA_MAP_BUFFER;
typedef LSA_CREATE_TOKEN * PLSA_CREATE_TOKEN;
typedef LSA_AUDIT_LOGON * PLSA_AUDIT_LOGON;
typedef LSA_CALL_PACKAGE * PLSA_CALL_PACKAGE;
typedef LSA_CALL_PACKAGEEX * PLSA_CALL_PACKAGEEX;
typedef LSA_GET_CALL_INFO * PLSA_GET_CALL_INFO;
typedef LSA_CREATE_SHARED_MEMORY * PLSA_CREATE_SHARED_MEMORY;
typedef LSA_ALLOCATE_SHARED_MEMORY * PLSA_ALLOCATE_SHARED_MEMORY;
typedef LSA_FREE_SHARED_MEMORY * PLSA_FREE_SHARED_MEMORY;
typedef LSA_DELETE_SHARED_MEMORY * PLSA_DELETE_SHARED_MEMORY;
typedef LSA_OPEN_SAM_USER * PLSA_OPEN_SAM_USER;
typedef LSA_GET_USER_CREDENTIALS * PLSA_GET_USER_CREDENTIALS;
typedef LSA_GET_USER_AUTH_DATA * PLSA_GET_USER_AUTH_DATA;
typedef LSA_CLOSE_SAM_USER * PLSA_CLOSE_SAM_USER;
typedef LSA_CONVERT_AUTH_DATA_TO_TOKEN * PLSA_CONVERT_AUTH_DATA_TO_TOKEN;
typedef LSA_CLIENT_CALLBACK * PLSA_CLIENT_CALLBACK;
typedef LSA_REGISTER_CALLBACK * PLSA_REGISTER_CALLBACK;
typedef LSA_UPDATE_PRIMARY_CREDENTIALS * PLSA_UPDATE_PRIMARY_CREDENTIALS;
typedef LSA_GET_AUTH_DATA_FOR_USER * PLSA_GET_AUTH_DATA_FOR_USER;
typedef LSA_CRACK_SINGLE_NAME * PLSA_CRACK_SINGLE_NAME;
typedef LSA_AUDIT_ACCOUNT_LOGON * PLSA_AUDIT_ACCOUNT_LOGON;
typedef LSA_CALL_PACKAGE_PASSTHROUGH * PLSA_CALL_PACKAGE_PASSTHROUGH;
typedef LSA_PROTECT_MEMORY * PLSA_PROTECT_MEMORY;
typedef LSA_OPEN_TOKEN_BY_LOGON_ID * PLSA_OPEN_TOKEN_BY_LOGON_ID;
typedef LSA_EXPAND_AUTH_DATA_FOR_DOMAIN * PLSA_EXPAND_AUTH_DATA_FOR_DOMAIN;
typedef LSA_CREATE_TOKEN_EX * PLSA_CREATE_TOKEN_EX;
typedef LSA_GET_EXTENDED_CALL_FLAGS  * PLSA_GET_EXTENDED_CALL_FLAGS;
typedef LSA_GET_SERVICE_ACCOUNT_PASSWORD * PLSA_GET_SERVICE_ACCOUNT_PASSWORD;
typedef LSA_AUDIT_LOGON_EX * PLSA_AUDIT_LOGON_EX;
typedef LSA_CHECK_PROTECTED_USER_BY_TOKEN *PLSA_CHECK_PROTECTED_USER_BY_TOKEN;
typedef LSA_QUERY_CLIENT_REQUEST *PLSA_QUERY_CLIENT_REQUEST;
typedef LSA_GET_APP_MODE_INFO *PLSA_GET_APP_MODE_INFO;
typedef LSA_SET_APP_MODE_INFO *PLSA_SET_APP_MODE_INFO;
typedef LSA_GET_SECPKG_FAILURE_REASON* PLSA_GET_SECPKG_FAILURE_REASON;
typedef LSA_SET_SECPKG_FAILURE_REASON* PLSA_SET_SECPKG_FAILURE_REASON;

#ifdef _WINCRED_H_

//
// When passing a credential around, the CredentialBlob field is encrypted.
// This structure describes this encrypted form.
//
//
#ifndef _ENCRYPTED_CREDENTIAL_DEFINED
#define _ENCRYPTED_CREDENTIAL_DEFINED

typedef struct _ENCRYPTED_CREDENTIALW {

    //
    // The credential
    //
    // The CredentialBlob field points to the encrypted credential
    // The CredentialBlobSize field is the length (in bytes) of the encrypted credential
    //

    CREDENTIALW Cred;

    //
    // The size in bytes of the clear text credential blob
    //

    ULONG ClearCredentialBlobSize;
} ENCRYPTED_CREDENTIALW, *PENCRYPTED_CREDENTIALW;
#endif // _ENCRYPTED_CREDENTIAL_DEFINED

//
// Values for CredFlags parameter
//

#define CREDP_FLAGS_IN_PROCESS              0x01    // Caller is in-process. Password data may be returned
#define CREDP_FLAGS_USE_MIDL_HEAP           0x02    // Allocated buffer should use MIDL_user_allocte
#define CREDP_FLAGS_DONT_CACHE_TI           0x04    // TargetInformation shouldn't be cached for CredGetTargetInfo
#define CREDP_FLAGS_CLEAR_PASSWORD          0x08    // Credential blob is passed in in-the-clear
#define CREDP_FLAGS_USER_ENCRYPTED_PASSWORD 0x10    // Credential blob is passed protected by RtlEncryptMemory
#define CREDP_FLAGS_TRUSTED_CALLER          0x20     // Caller is a trusted process (eg. logon process).
#define CREDP_FLAGS_VALIDATE_PROXY_TARGET   0x40     // Validate that TargetName is proxy


typedef NTSTATUS
(NTAPI CredReadFn) (
    _In_ PLUID LogonId,
    _In_ ULONG CredFlags,
    _In_ LPWSTR TargetName,
    _In_ ULONG Type,
    _In_ ULONG Flags,
    _Outptr_ PENCRYPTED_CREDENTIALW *Credential
    );

typedef NTSTATUS
(NTAPI CredReadDomainCredentialsFn) (
    _In_ PLUID LogonId,
    _In_ ULONG CredFlags,
    _In_ PCREDENTIAL_TARGET_INFORMATIONW TargetInfo,
    _In_ ULONG Flags,
    _Out_ PULONG Count,
    _Outptr_result_buffer_(*Count) PENCRYPTED_CREDENTIALW **Credential
    );

typedef VOID
(NTAPI CredFreeCredentialsFn) (
    _In_ ULONG Count,
    _Inout_updates_opt_(Count) PENCRYPTED_CREDENTIALW *Credentials
    );

typedef NTSTATUS
(NTAPI CredWriteFn) (
    _In_ PLUID LogonId,
    _In_ ULONG CredFlags,
    _In_ PENCRYPTED_CREDENTIALW Credential,
    _In_ ULONG Flags
    );

typedef NTSTATUS
(NTAPI CrediUnmarshalandDecodeStringFn)(
    _In_  LPWSTR  MarshaledString,
    _Outptr_result_bytebuffer_(*BlobSize) LPBYTE  *Blob,
    _Out_ ULONG *BlobSize,
    _Out_ BOOLEAN *IsFailureFatal
    );

#ifdef __cplusplus
extern "C"
{
#endif

NTSTATUS
CredMarshalTargetInfo (
    _In_        PCREDENTIAL_TARGET_INFORMATIONW InTargetInfo,
    _Outptr_result_bytebuffer_(*BufferSize) PUSHORT     *Buffer,
    _Out_       PULONG                          BufferSize
    );

NTSTATUS
CredUnmarshalTargetInfo (
    _In_reads_bytes_(BufferSize) PUSHORT                 Buffer,
    _In_      ULONG                                 BufferSize,
    _Outptr_opt_ PCREDENTIAL_TARGET_INFORMATIONW *RetTargetInfo,
    _Out_opt_ PULONG                                RetActualSize
    );

#ifdef __cplusplus
} // extern "C"
#endif

// Number of bytes consumed by the trailing size ULONG
#define CRED_MARSHALED_TI_SIZE_SIZE 12

#endif // _WINCRED_H_


//
// Pure 32-bit versions of credential structures for packages
// running wow64:
//

typedef struct _SEC_WINNT_AUTH_IDENTITY32 {
    ULONG User;
    ULONG UserLength;
    ULONG Domain;
    ULONG DomainLength;
    ULONG Password;
    ULONG PasswordLength;
    ULONG Flags;
} SEC_WINNT_AUTH_IDENTITY32, * PSEC_WINNT_AUTH_IDENTITY32;

typedef struct _SEC_WINNT_AUTH_IDENTITY_EX32 {
    ULONG Version;
    ULONG Length;
    ULONG User;
    ULONG UserLength;
    ULONG Domain;
    ULONG DomainLength;
    ULONG Password;
    ULONG PasswordLength;
    ULONG Flags;
    ULONG PackageList;
    ULONG PackageListLength;
} SEC_WINNT_AUTH_IDENTITY_EX32, * PSEC_WINNT_AUTH_IDENTITY_EX32;

// Functions provided by the SPM to the packages:
typedef struct _LSA_SECPKG_FUNCTION_TABLE {
    PLSA_CREATE_LOGON_SESSION CreateLogonSession;
    PLSA_DELETE_LOGON_SESSION DeleteLogonSession;
    PLSA_ADD_CREDENTIAL AddCredential;
    PLSA_GET_CREDENTIALS GetCredentials;
    PLSA_DELETE_CREDENTIAL DeleteCredential;
    PLSA_ALLOCATE_LSA_HEAP AllocateLsaHeap;
    PLSA_FREE_LSA_HEAP FreeLsaHeap;
    PLSA_ALLOCATE_CLIENT_BUFFER AllocateClientBuffer;
    PLSA_FREE_CLIENT_BUFFER FreeClientBuffer;
    PLSA_COPY_TO_CLIENT_BUFFER CopyToClientBuffer;
    PLSA_COPY_FROM_CLIENT_BUFFER CopyFromClientBuffer;
    PLSA_IMPERSONATE_CLIENT ImpersonateClient;
    PLSA_UNLOAD_PACKAGE UnloadPackage;
    PLSA_DUPLICATE_HANDLE DuplicateHandle;
    PLSA_SAVE_SUPPLEMENTAL_CREDENTIALS SaveSupplementalCredentials;
    PLSA_CREATE_THREAD CreateThread;
    PLSA_GET_CLIENT_INFO GetClientInfo;
    PLSA_REGISTER_NOTIFICATION RegisterNotification;
    PLSA_CANCEL_NOTIFICATION CancelNotification;
    PLSA_MAP_BUFFER MapBuffer;
    PLSA_CREATE_TOKEN CreateToken;
    PLSA_AUDIT_LOGON AuditLogon;
    PLSA_CALL_PACKAGE CallPackage;
    PLSA_FREE_LSA_HEAP FreeReturnBuffer;
    PLSA_GET_CALL_INFO GetCallInfo;
    PLSA_CALL_PACKAGEEX CallPackageEx;
    PLSA_CREATE_SHARED_MEMORY CreateSharedMemory;
    PLSA_ALLOCATE_SHARED_MEMORY AllocateSharedMemory;
    PLSA_FREE_SHARED_MEMORY FreeSharedMemory;
    PLSA_DELETE_SHARED_MEMORY DeleteSharedMemory;
    PLSA_OPEN_SAM_USER OpenSamUser;
    PLSA_GET_USER_CREDENTIALS GetUserCredentials;
    PLSA_GET_USER_AUTH_DATA GetUserAuthData;
    PLSA_CLOSE_SAM_USER CloseSamUser;
    PLSA_CONVERT_AUTH_DATA_TO_TOKEN ConvertAuthDataToToken;
    PLSA_CLIENT_CALLBACK ClientCallback;
    PLSA_UPDATE_PRIMARY_CREDENTIALS UpdateCredentials;
    PLSA_GET_AUTH_DATA_FOR_USER GetAuthDataForUser;
    PLSA_CRACK_SINGLE_NAME CrackSingleName;
    PLSA_AUDIT_ACCOUNT_LOGON AuditAccountLogon;
    PLSA_CALL_PACKAGE_PASSTHROUGH CallPackagePassthrough;
#ifdef _WINCRED_H_
    CredReadFn *CrediRead;
    CredReadDomainCredentialsFn *CrediReadDomainCredentials;
    CredFreeCredentialsFn *CrediFreeCredentials;
#else // _WINCRED_H_
    PLSA_PROTECT_MEMORY DummyFunction1;
    PLSA_PROTECT_MEMORY DummyFunction2;
    PLSA_PROTECT_MEMORY DummyFunction3;
#endif // _WINCRED_H_
    PLSA_PROTECT_MEMORY LsaProtectMemory;
    PLSA_PROTECT_MEMORY LsaUnprotectMemory;
    PLSA_OPEN_TOKEN_BY_LOGON_ID OpenTokenByLogonId;
    PLSA_EXPAND_AUTH_DATA_FOR_DOMAIN ExpandAuthDataForDomain;
    PLSA_ALLOCATE_PRIVATE_HEAP AllocatePrivateHeap;
    PLSA_FREE_PRIVATE_HEAP FreePrivateHeap;
    PLSA_CREATE_TOKEN_EX CreateTokenEx;
#ifdef _WINCRED_H_
    CredWriteFn *CrediWrite;
    CrediUnmarshalandDecodeStringFn *CrediUnmarshalandDecodeString;
#else // _WINCRED_H_
    PLSA_PROTECT_MEMORY DummyFunction4;
    PLSA_PROTECT_MEMORY DummyFunction5;
#endif // _WINCRED_H_
    PLSA_PROTECT_MEMORY DummyFunction6;
    PLSA_GET_EXTENDED_CALL_FLAGS      GetExtendedCallFlags;
    PLSA_DUPLICATE_HANDLE             DuplicateTokenHandle;
    PLSA_GET_SERVICE_ACCOUNT_PASSWORD GetServiceAccountPassword;
    PLSA_PROTECT_MEMORY DummyFunction7;
    PLSA_AUDIT_LOGON_EX AuditLogonEx;
    PLSA_CHECK_PROTECTED_USER_BY_TOKEN CheckProtectedUserByToken;
    PLSA_QUERY_CLIENT_REQUEST QueryClientRequest;
    PLSA_GET_APP_MODE_INFO GetAppModeInfo;
    PLSA_SET_APP_MODE_INFO SetAppModeInfo;
    PLSA_GET_CLIENT_INFO_EX GetClientInfoEx;
    PLSA_GET_SECPKG_FAILURE_REASON GetSecpkgFailureReason;
    PLSA_SET_SECPKG_FAILURE_REASON SetSecpkgFailureReason;
} LSA_SECPKG_FUNCTION_TABLE, *PLSA_SECPKG_FUNCTION_TABLE;


typedef
PVOID
(NTAPI LSA_LOCATE_PKG_BY_ID)(
    _In_ ULONG PackgeId
    );

typedef LSA_LOCATE_PKG_BY_ID * PLSA_LOCATE_PKG_BY_ID;

typedef struct _SECPKG_DLL_FUNCTIONS {
    PLSA_ALLOCATE_LSA_HEAP AllocateHeap;
    PLSA_FREE_LSA_HEAP FreeHeap;
    PLSA_REGISTER_CALLBACK RegisterCallback;
    PLSA_LOCATE_PKG_BY_ID LocatePackageById;
} SECPKG_DLL_FUNCTIONS, * PSECPKG_DLL_FUNCTIONS;


//
// The following prototypes are to functions that will be called only while
// in the Security Package Manager context.
//

typedef NTSTATUS
(NTAPI SpInitializeFn)(
    _In_ ULONG_PTR PackageId,
    _In_ PSECPKG_PARAMETERS Parameters,
    _In_ PLSA_SECPKG_FUNCTION_TABLE FunctionTable
    );

typedef NTSTATUS
(NTAPI SpShutdownFn)(
    VOID
    );

typedef NTSTATUS
(NTAPI SpGetInfoFn)(
    _Out_ PSecPkgInfo PackageInfo
    );

typedef NTSTATUS
(NTAPI SpGetExtendedInformationFn)(
    _In_  SECPKG_EXTENDED_INFORMATION_CLASS Class,
    _Outptr_ PSECPKG_EXTENDED_INFORMATION * ppInformation
    );

typedef NTSTATUS
(NTAPI SpSetExtendedInformationFn)(
    _In_ SECPKG_EXTENDED_INFORMATION_CLASS Class,
    _In_ PSECPKG_EXTENDED_INFORMATION Info
    );

typedef NTSTATUS
(LSA_AP_LOGON_USER_EX2) (
    _In_ PLSA_CLIENT_REQUEST ClientRequest,
    _In_ SECURITY_LOGON_TYPE LogonType,
    _In_reads_bytes_(SubmitBufferSize) PVOID ProtocolSubmitBuffer,
    _In_ PVOID ClientBufferBase,
    _In_ ULONG SubmitBufferSize,
    _Outptr_result_bytebuffer_(*ProfileBufferSize) PVOID *ProfileBuffer,
    _Out_ PULONG ProfileBufferSize,
    _Out_ PLUID LogonId,
    _Out_ PNTSTATUS SubStatus,
    _Out_ PLSA_TOKEN_INFORMATION_TYPE TokenInformationType,
    _Outptr_ PVOID *TokenInformation,
    _Out_ PUNICODE_STRING *AccountName,
    _Out_ PUNICODE_STRING *AuthenticatingAuthority,
    _Out_ PUNICODE_STRING *MachineName,
    _Out_ PSECPKG_PRIMARY_CRED PrimaryCredentials,
    _Outptr_ PSECPKG_SUPPLEMENTAL_CRED_ARRAY * SupplementalCredentials
    );

typedef LSA_AP_LOGON_USER_EX2 *PLSA_AP_LOGON_USER_EX2;
#define LSA_AP_NAME_LOGON_USER_EX2 "LsaApLogonUserEx2\0"

typedef NTSTATUS
(LSA_AP_LOGON_USER_EX3) (
    _In_ PLSA_CLIENT_REQUEST ClientRequest,
    _In_ SECURITY_LOGON_TYPE LogonType,
    _In_reads_bytes_(SubmitBufferSize) PVOID ProtocolSubmitBuffer,
    _In_ PVOID ClientBufferBase,
    _In_ ULONG SubmitBufferSize,
    _Inout_ PSECPKG_SURROGATE_LOGON SurrogateLogon,
    _Outptr_result_bytebuffer_(*ProfileBufferSize) PVOID *ProfileBuffer,
    _Out_ PULONG ProfileBufferSize,
    _Out_ PLUID LogonId,
    _Out_ PNTSTATUS SubStatus,
    _Out_ PLSA_TOKEN_INFORMATION_TYPE TokenInformationType,
    _Outptr_ PVOID *TokenInformation,
    _Out_ PUNICODE_STRING *AccountName,
    _Out_ PUNICODE_STRING *AuthenticatingAuthority,
    _Out_ PUNICODE_STRING *MachineName,
    _Out_ PSECPKG_PRIMARY_CRED PrimaryCredentials,
    _Outptr_ PSECPKG_SUPPLEMENTAL_CRED_ARRAY * SupplementalCredentials
    );

typedef LSA_AP_LOGON_USER_EX3 *PLSA_AP_LOGON_USER_EX3;

typedef NTSTATUS
(LSA_AP_PRE_LOGON_USER_SURROGATE) (
    _In_ PLSA_CLIENT_REQUEST ClientRequest,
    _In_ SECURITY_LOGON_TYPE LogonType,
    _In_reads_bytes_(SubmitBufferSize) PVOID ProtocolSubmitBuffer,
    _In_ PVOID ClientBufferBase,
    _In_ ULONG SubmitBufferSize,
    _Inout_ PSECPKG_SURROGATE_LOGON SurrogateLogon,
    _Out_ PNTSTATUS SubStatus
    );

typedef LSA_AP_PRE_LOGON_USER_SURROGATE *PLSA_AP_PRE_LOGON_USER_SURROGATE;

typedef NTSTATUS
(LSA_AP_POST_LOGON_USER_SURROGATE) (
    _In_ PLSA_CLIENT_REQUEST ClientRequest,
    _In_ SECURITY_LOGON_TYPE LogonType,
    _In_reads_bytes_(SubmitBufferSize) PVOID ProtocolSubmitBuffer,
    _In_ PVOID ClientBufferBase,
    _In_ ULONG SubmitBufferSize,
    _In_ PSECPKG_SURROGATE_LOGON SurrogateLogon,
    _In_reads_bytes_(ProfileBufferSize) PVOID ProfileBuffer,
    _In_ ULONG ProfileBufferSize,
    _In_ PLUID LogonId,
    _In_ NTSTATUS Status,
    _In_ NTSTATUS SubStatus,
    _In_ LSA_TOKEN_INFORMATION_TYPE TokenInformationType,
    _In_ PVOID TokenInformation,
    _In_ PUNICODE_STRING AccountName,
    _In_ PUNICODE_STRING AuthenticatingAuthority,
    _In_ PUNICODE_STRING MachineName,
    _In_ PSECPKG_PRIMARY_CRED PrimaryCredentials,
    _In_ PSECPKG_SUPPLEMENTAL_CRED_ARRAY SupplementalCredentials
    );

typedef LSA_AP_POST_LOGON_USER_SURROGATE *PLSA_AP_POST_LOGON_USER_SURROGATE;

typedef NTSTATUS
(NTAPI SpAcceptCredentialsFn)(
    _In_ SECURITY_LOGON_TYPE LogonType,
    _In_ PUNICODE_STRING AccountName,
    _In_opt_ PSECPKG_PRIMARY_CRED PrimaryCredentials,
    _In_opt_ PSECPKG_SUPPLEMENTAL_CRED SupplementalCredentials
    );
#define SP_ACCEPT_CREDENTIALS_NAME "SpAcceptCredentials\0"

typedef NTSTATUS
(NTAPI SpAcquireCredentialsHandleFn)(
    _In_opt_ PUNICODE_STRING PrincipalName,
    _In_ ULONG CredentialUseFlags,
    _In_opt_ PLUID LogonId,
    _In_opt_ PVOID AuthorizationData,
    _In_opt_ PVOID GetKeyFunciton,
    _In_opt_ PVOID GetKeyArgument,
    _Out_ PLSA_SEC_HANDLE CredentialHandle,
    _Out_ PTimeStamp ExpirationTime
    );

typedef NTSTATUS
(NTAPI SpFreeCredentialsHandleFn)(
    _In_ LSA_SEC_HANDLE CredentialHandle
    );

typedef NTSTATUS
(NTAPI SpQueryCredentialsAttributesFn)(
    _In_ LSA_SEC_HANDLE CredentialHandle,
    _In_ ULONG CredentialAttribute,
    _Inout_ PVOID Buffer
    );

typedef NTSTATUS
(NTAPI SpSetCredentialsAttributesFn)(
    _In_ LSA_SEC_HANDLE CredentialHandle,
    _In_ ULONG CredentialAttribute,
    _In_reads_bytes_(BufferSize) PVOID Buffer,
    _In_ ULONG BufferSize );

typedef NTSTATUS
(NTAPI SpAddCredentialsFn)(
    _In_ LSA_SEC_HANDLE CredentialHandle,
    _In_opt_ PUNICODE_STRING PrincipalName,
    _In_ PUNICODE_STRING Package,
    _In_ ULONG CredentialUseFlags,
    _In_ PVOID AuthorizationData,
    _In_ PVOID GetKeyFunciton,
    _In_ PVOID GetKeyArgument,
    _Out_ PTimeStamp ExpirationTime
    );

typedef NTSTATUS
(NTAPI SpSaveCredentialsFn)(
    _In_ LSA_SEC_HANDLE CredentialHandle,
    _In_ PSecBuffer Credentials);

typedef NTSTATUS
(NTAPI SpGetCredentialsFn)(
    _In_ LSA_SEC_HANDLE CredentialHandle,
    _Inout_ PSecBuffer Credentials
    );

typedef NTSTATUS
(NTAPI SpDeleteCredentialsFn)(
    _In_ LSA_SEC_HANDLE CredentialHandle,
    _In_ PSecBuffer Key
    );

typedef NTSTATUS
(NTAPI SpInitLsaModeContextFn)(
    _In_opt_ LSA_SEC_HANDLE CredentialHandle,
    _In_opt_ LSA_SEC_HANDLE ContextHandle,
    _In_opt_ PUNICODE_STRING TargetName,
    _In_ ULONG ContextRequirements,
    _In_ ULONG TargetDataRep,
    _In_ PSecBufferDesc InputBuffers,
    _Out_ PLSA_SEC_HANDLE NewContextHandle,
    _Inout_ PSecBufferDesc OutputBuffers,
    _Out_ PULONG ContextAttributes,
    _Out_ PTimeStamp ExpirationTime,
    _Out_ PBOOLEAN MappedContext,
    _Out_ PSecBuffer ContextData
    );




typedef NTSTATUS
(NTAPI SpDeleteContextFn)(
    _In_ LSA_SEC_HANDLE ContextHandle
    );

typedef NTSTATUS
(NTAPI SpApplyControlTokenFn)(
    _In_ LSA_SEC_HANDLE ContextHandle,
    _In_ PSecBufferDesc ControlToken);


typedef NTSTATUS
(NTAPI SpAcceptLsaModeContextFn)(
    _In_opt_ LSA_SEC_HANDLE CredentialHandle,
    _In_opt_ LSA_SEC_HANDLE ContextHandle,
    _In_ PSecBufferDesc InputBuffer,
    _In_ ULONG ContextRequirements,
    _In_ ULONG TargetDataRep,
    _Out_ PLSA_SEC_HANDLE NewContextHandle,
    _Inout_ PSecBufferDesc OutputBuffer,
    _Out_ PULONG ContextAttributes,
    _Out_ PTimeStamp ExpirationTime,
    _Out_ PBOOLEAN MappedContext,
    _Out_ PSecBuffer ContextData
    );




typedef NTSTATUS
(NTAPI SpGetUserInfoFn)(
    _In_ PLUID LogonId,
    _In_ ULONG Flags,
    _Outptr_ PSecurityUserData * UserData
    );

typedef NTSTATUS
(NTAPI SpQueryContextAttributesFn)(
    _In_ LSA_SEC_HANDLE ContextHandle,
    _In_ ULONG ContextAttribute,
    _Inout_ PVOID Buffer);

typedef NTSTATUS
(NTAPI SpSetContextAttributesFn)(
    _In_ LSA_SEC_HANDLE ContextHandle,
    _In_ ULONG ContextAttribute,
    _In_reads_bytes_(BufferSize) PVOID Buffer,
    _In_ ULONG BufferSize );

typedef NTSTATUS
(NTAPI SpChangeAccountPasswordFn)(
    _In_ PUNICODE_STRING      pDomainName,
    _In_ PUNICODE_STRING      pAccountName,
    _In_ PUNICODE_STRING      pOldPassword,
    _In_ PUNICODE_STRING      pNewPassword,
    _In_ BOOLEAN              Impersonating,
    _Inout_ PSecBufferDesc   pOutput
    );

typedef NTSTATUS
(NTAPI SpQueryMetaDataFn)(
    _In_opt_ LSA_SEC_HANDLE CredentialHandle,
    _In_opt_ PUNICODE_STRING TargetName,
    _In_ ULONG ContextRequirements,
    _Out_ PULONG MetaDataLength,
    _Outptr_result_bytebuffer_(*MetaDataLength) PUCHAR* MetaData,
    _Inout_ PLSA_SEC_HANDLE ContextHandle
    );

typedef NTSTATUS
(NTAPI SpExchangeMetaDataFn)(
    _In_opt_ LSA_SEC_HANDLE CredentialHandle,
    _In_opt_ PUNICODE_STRING TargetName,
    _In_ ULONG ContextRequirements,
    _In_ ULONG MetaDataLength,
    _In_reads_bytes_(MetaDataLength) PUCHAR MetaData,
    _Inout_ PLSA_SEC_HANDLE ContextHandle
    );

typedef NTSTATUS
(NTAPI SpGetCredUIContextFn)(
   _In_ LSA_SEC_HANDLE ContextHandle,
   _In_ GUID* CredType,
   _Out_ PULONG FlatCredUIContextLength,
   _Outptr_result_bytebuffer_(*FlatCredUIContextLength)  PUCHAR* FlatCredUIContext
   );

typedef NTSTATUS
(NTAPI SpUpdateCredentialsFn)(
  _In_ LSA_SEC_HANDLE ContextHandle,
  _In_ GUID* CredType,
  _In_ ULONG FlatCredUIContextLength,
  _In_reads_bytes_(FlatCredUIContextLength) PUCHAR FlatCredUIContext
  );

typedef NTSTATUS
(NTAPI SpValidateTargetInfoFn) (
    _In_opt_ PLSA_CLIENT_REQUEST ClientRequest,
    _In_reads_bytes_(SubmitBufferLength) PVOID ProtocolSubmitBuffer,
    _In_ PVOID ClientBufferBase,
    _In_ ULONG SubmitBufferLength,
    _In_ PSECPKG_TARGETINFO TargetInfo
    );

typedef NTSTATUS
(NTAPI SpExtractTargetInfoFn) (
    _In_opt_ PLSA_CLIENT_REQUEST ClientRequest,
    _In_reads_bytes_(SubmitBufferLength) PVOID ProtocolSubmitBuffer,
    _In_opt_ PVOID ClientBufferBase,
    _In_ ULONG SubmitBufferLength,
    _Result_nullonfailure_ _Outptr_result_bytebuffer_(*pcbTargetInfo) PVOID* ppvTargetInfo,
    _Out_ ULONG* pcbTargetInfo
    );

typedef NTSTATUS
(NTAPI LSA_AP_POST_LOGON_USER) (
    _In_ PSECPKG_POST_LOGON_USER_INFO PostLogonUserInfo
    );

typedef NTSTATUS
(NTAPI SpGetRemoteCredGuardLogonBufferFn) (
    _In_ LSA_SEC_HANDLE CredHandle,
    _In_ LSA_SEC_HANDLE ContextHandle,
    _In_ const UNICODE_STRING* TargetName,
    _Out_ PHANDLE RedirectedLogonHandle,
    _Out_ PLSA_REDIRECTED_LOGON_CALLBACK* Callback,
    _Out_ PLSA_REDIRECTED_LOGON_CLEANUP_CALLBACK* CleanupCallback,
    _Out_ PULONG LogonBufferSize,
    _Outptr_result_bytebuffer_(*LogonBufferSize) PVOID* LogonBuffer
    );

typedef NTSTATUS
(NTAPI SpGetRemoteCredGuardSupplementalCredsFn) (
    _In_ LSA_SEC_HANDLE CredHandle,
    _In_ const UNICODE_STRING* TargetName,
    _Out_ PHANDLE RedirectedLogonHandle,
    _Out_ PLSA_REDIRECTED_LOGON_CALLBACK* Callback,
    _Out_ PLSA_REDIRECTED_LOGON_CLEANUP_CALLBACK* CleanupCallback,
    _Out_ PULONG SupplementalCredsSize,
    _Outptr_result_bytebuffer_(*SupplementalCredsSize) PVOID * SupplementalCreds
    );

typedef NTSTATUS
(NTAPI SpGetTbalSupplementalCredsFn) (
    _In_ LUID LogonId,
    _Out_ PULONG SupplementalCredsSize,
    _Outptr_result_bytebuffer_(*SupplementalCredsSize) PVOID * SupplementalCreds
    );

typedef struct _SECPKG_FUNCTION_TABLE {
    PLSA_AP_INITIALIZE_PACKAGE InitializePackage;                           // SECPKG_INTERFACE_VERSION
    PLSA_AP_LOGON_USER LogonUser;                                           // SECPKG_INTERFACE_VERSION
    PLSA_AP_CALL_PACKAGE CallPackage;                                       // SECPKG_INTERFACE_VERSION
    PLSA_AP_LOGON_TERMINATED LogonTerminated;                               // SECPKG_INTERFACE_VERSION
    PLSA_AP_CALL_PACKAGE_UNTRUSTED CallPackageUntrusted;                    // SECPKG_INTERFACE_VERSION
    PLSA_AP_CALL_PACKAGE_PASSTHROUGH CallPackagePassthrough;                // SECPKG_INTERFACE_VERSION
    PLSA_AP_LOGON_USER_EX LogonUserEx;                                      // SECPKG_INTERFACE_VERSION
    PLSA_AP_LOGON_USER_EX2 LogonUserEx2;                                    // SECPKG_INTERFACE_VERSION
    SpInitializeFn * Initialize;                                            // SECPKG_INTERFACE_VERSION
    SpShutdownFn * Shutdown;                                                // SECPKG_INTERFACE_VERSION
    SpGetInfoFn * GetInfo;                                                  // SECPKG_INTERFACE_VERSION
    SpAcceptCredentialsFn * AcceptCredentials;                              // SECPKG_INTERFACE_VERSION
    SpAcquireCredentialsHandleFn * AcquireCredentialsHandle;                // SECPKG_INTERFACE_VERSION
    SpQueryCredentialsAttributesFn * QueryCredentialsAttributes;            // SECPKG_INTERFACE_VERSION
    SpFreeCredentialsHandleFn * FreeCredentialsHandle;                      // SECPKG_INTERFACE_VERSION
    SpSaveCredentialsFn * SaveCredentials;                                  // SECPKG_INTERFACE_VERSION
    SpGetCredentialsFn * GetCredentials;                                    // SECPKG_INTERFACE_VERSION
    SpDeleteCredentialsFn * DeleteCredentials;                              // SECPKG_INTERFACE_VERSION
    SpInitLsaModeContextFn * InitLsaModeContext;                            // SECPKG_INTERFACE_VERSION
    SpAcceptLsaModeContextFn * AcceptLsaModeContext;                        // SECPKG_INTERFACE_VERSION
    SpDeleteContextFn * DeleteContext;                                      // SECPKG_INTERFACE_VERSION
    SpApplyControlTokenFn * ApplyControlToken;                              // SECPKG_INTERFACE_VERSION
    SpGetUserInfoFn * GetUserInfo;                                          // SECPKG_INTERFACE_VERSION
    SpGetExtendedInformationFn * GetExtendedInformation;                    // SECPKG_INTERFACE_VERSION
    SpQueryContextAttributesFn * QueryContextAttributes;                    // SECPKG_INTERFACE_VERSION
    SpAddCredentialsFn * AddCredentials;                                    // SECPKG_INTERFACE_VERSION
    SpSetExtendedInformationFn * SetExtendedInformation;                    // SECPKG_INTERFACE_VERSION

    SpSetContextAttributesFn * SetContextAttributes;                        // SECPKG_INTERFACE_VERSION_2

    SpSetCredentialsAttributesFn * SetCredentialsAttributes;                // SECPKG_INTERFACE_VERSION_3

    SpChangeAccountPasswordFn * ChangeAccountPassword;                      // SECPKG_INTERFACE_VERSION_4

    SpQueryMetaDataFn* QueryMetaData;                                       // SECPKG_INTERFACE_VERSION_5
    SpExchangeMetaDataFn* ExchangeMetaData;                                 // SECPKG_INTERFACE_VERSION_5
    SpGetCredUIContextFn* GetCredUIContext;                                 // SECPKG_INTERFACE_VERSION_5
    SpUpdateCredentialsFn* UpdateCredentials;                               // SECPKG_INTERFACE_VERSION_5

    SpValidateTargetInfoFn* ValidateTargetInfo;                             // SECPKG_INTERFACE_VERSION_6

    LSA_AP_POST_LOGON_USER* PostLogonUser;                                  // SECPKG_INTERFACE_VERSION_7

    SpGetRemoteCredGuardLogonBufferFn* GetRemoteCredGuardLogonBuffer;       // SECPKG_INTERFACE_VERSION_8
    SpGetRemoteCredGuardSupplementalCredsFn* GetRemoteCredGuardSupplementalCreds; // SECPKG_INTERFACE_VERSION_8

    SpGetTbalSupplementalCredsFn* GetTbalSupplementalCreds;                 // SECPKG_INTERFACE_VERSION_9

    PLSA_AP_LOGON_USER_EX3 LogonUserEx3;                                    // SECPKG_INTERFACE_VERSION_10
    PLSA_AP_PRE_LOGON_USER_SURROGATE PreLogonUserSurrogate;                 // SECPKG_INTERFACE_VERSION_10
    PLSA_AP_POST_LOGON_USER_SURROGATE PostLogonUserSurrogate;               // SECPKG_INTERFACE_VERSION_10

    SpExtractTargetInfoFn* ExtractTargetInfo;                               // SECPKG_INTERFACE_VERSION_11
} SECPKG_FUNCTION_TABLE, *PSECPKG_FUNCTION_TABLE;

//
// The following prototypes are to functions that will be called while in the
// context of a user process that is using the functions through the security
// DLL.
//
typedef NTSTATUS
(NTAPI SpInstanceInitFn)(
    _In_ ULONG Version,
    _In_ PSECPKG_DLL_FUNCTIONS FunctionTable,
    _Outptr_ PVOID * UserFunctions
    );

typedef NTSTATUS
(NTAPI SpInitUserModeContextFn)(
    _In_ LSA_SEC_HANDLE ContextHandle,
    _In_ PSecBuffer PackedContext
    );

typedef NTSTATUS
(NTAPI SpMakeSignatureFn)(
    _In_ LSA_SEC_HANDLE ContextHandle,
    _In_ ULONG QualityOfProtection,
    _In_ PSecBufferDesc MessageBuffers,
    _In_ ULONG MessageSequenceNumber
    );

typedef NTSTATUS
(NTAPI SpVerifySignatureFn)(
    _In_ LSA_SEC_HANDLE ContextHandle,
    _In_ PSecBufferDesc MessageBuffers,
    _In_ ULONG MessageSequenceNumber,
    _Out_opt_ PULONG QualityOfProtection
    );

typedef NTSTATUS
(NTAPI SpSealMessageFn)(
    _In_ LSA_SEC_HANDLE ContextHandle,
    _In_ ULONG QualityOfProtection,
    _In_ PSecBufferDesc MessageBuffers,
    _In_ ULONG MessageSequenceNumber
    );

typedef NTSTATUS
(NTAPI SpUnsealMessageFn)(
    _In_ LSA_SEC_HANDLE ContextHandle,
    _In_ PSecBufferDesc MessageBuffers,
    _In_ ULONG MessageSequenceNumber,
    _Out_opt_ PULONG QualityOfProtection
    );


typedef NTSTATUS
(NTAPI SpGetContextTokenFn)(
    _In_ LSA_SEC_HANDLE ContextHandle,
    _Out_ PHANDLE ImpersonationToken
    );


typedef NTSTATUS
(NTAPI SpExportSecurityContextFn)(
    _In_        LSA_SEC_HANDLE phContext,             // (in) context to export
    _In_        ULONG          fFlags,                // (in) option flags
    _Out_       PSecBuffer     pPackedContext,        // (out) marshalled context
    _Out_opt_   PHANDLE        pToken                 // (out, optional) token handle for impersonation
    );

typedef NTSTATUS
(NTAPI SpImportSecurityContextFn)(
    _In_ PSecBuffer           pPackedContext,        // (in) marshalled context
    _In_opt_ HANDLE           Token,                 // (in, optional) handle to token for context
    _Out_ PLSA_SEC_HANDLE     phContext              // (out) new context handle
    );


typedef NTSTATUS
(NTAPI SpCompleteAuthTokenFn)(
    _In_ LSA_SEC_HANDLE ContextHandle,
    _In_ PSecBufferDesc InputBuffer
    );


typedef NTSTATUS
(NTAPI SpFormatCredentialsFn)(
    _In_ PSecBuffer Credentials,
    _Out_ PSecBuffer FormattedCredentials
    );

typedef NTSTATUS
(NTAPI SpMarshallSupplementalCredsFn)(
    _In_ ULONG CredentialSize,
    _In_reads_bytes_(CredentialSize) PUCHAR Credentials,
    _Out_ PULONG MarshalledCredSize,
    _Outptr_result_bytebuffer_(*MarshalledCredSize) PVOID * MarshalledCreds
    );

//
// AttributeInfo flag
//
// The highest order bit of AttributeInfo indicates whether the attribute data is ANSI (0) or UNICODE (1).
// The remaining bits store an enumeration, indicating which call the attribute came from:
//    SECPKG_CREDENTIAL_ATTRIBUTE - Attribute from SetCredentialsAttributes
//
#define SECPKG_UNICODE_ATTRIBUTE 0x80000000
#define SECPKG_ANSI_ATTRIBUTE 0
#define SECPKG_CREDENTIAL_ATTRIBUTE 0

//
// On success, MarshaledAttributeData contains MarshaledAttributeDataSize bytes of packed data.
// If no work needs to be done, returns STATUS_NOT_SUPPORTED and leaves MarshaledAttributeData NULL.
//
typedef NTSTATUS
(NTAPI SpMarshalAttributeDataFn)(
    _In_ DWORD AttributeInfo,
    _In_ ULONG Attribute,
    _In_ ULONG AttributeDataSize,
    _In_reads_bytes_(AttributeDataSize) PBYTE AttributeData,
    _Out_ PULONG MarshaledAttributeDataSize,
    _Outptr_result_bytebuffer_(*MarshaledAttributeDataSize) PBYTE * MarshaledAttributeData);

typedef struct _SECPKG_USER_FUNCTION_TABLE {
    SpInstanceInitFn *                      InstanceInit;
    SpInitUserModeContextFn *               InitUserModeContext;
    SpMakeSignatureFn *                     MakeSignature;
    SpVerifySignatureFn *                   VerifySignature;
    SpSealMessageFn *                       SealMessage;
    SpUnsealMessageFn *                     UnsealMessage;
    SpGetContextTokenFn *                   GetContextToken;
    SpQueryContextAttributesFn *            QueryContextAttributes;
    SpCompleteAuthTokenFn *                 CompleteAuthToken;
    SpDeleteContextFn *                     DeleteUserModeContext;
    SpFormatCredentialsFn *                 FormatCredentials;
    SpMarshallSupplementalCredsFn *         MarshallSupplementalCreds;
    SpExportSecurityContextFn *             ExportContext;
    SpImportSecurityContextFn *             ImportContext;
    SpMarshalAttributeDataFn *              MarshalAttributeData; // SECPKG_INTERFACE_VERSION_2
} SECPKG_USER_FUNCTION_TABLE, *PSECPKG_USER_FUNCTION_TABLE;


typedef NTSTATUS
(SEC_ENTRY * SpLsaModeInitializeFn)(
    _In_ ULONG LsaVersion,
    _Out_ PULONG PackageVersion,
    _Outptr_result_buffer_(*pcTables) PSECPKG_FUNCTION_TABLE * ppTables,
    _Out_ PULONG pcTables
    );

typedef NTSTATUS
(SEC_ENTRY * SpUserModeInitializeFn)(
    _In_ ULONG LsaVersion,
    _Out_ PULONG PackageVersion,
    _Outptr_result_buffer_(*pcTables) PSECPKG_USER_FUNCTION_TABLE *ppTables,
    _Out_ PULONG pcTables
    );

#define SECPKG_LSAMODEINIT_NAME     "SpLsaModeInitialize"
#define SECPKG_USERMODEINIT_NAME    "SpUserModeInitialize"

//
// Version of the security package interface.
//
// These defines are used for all of the following:
//
// * Passed by the LSA to SpLsaModeInitializeFn to indicate the version of the LSA.
//      All packages currently expect the LSA to pass SECPKG_INTERFACE_VERSION.
//
// * Passed by secur32.dll to SpUserModeInitialzeFn to indicate the version of the secur32 DLL.
//      All packages currently expect secur32 to pass SECPKG_INTERFACE_VERSION.
//
// * Returned from SpLsaModeInitializeFn to indicate the version of SECPKG_FUNCTION_TABLE.
//      SECPKG_INTERFACE_VERSION indicates all fields through SetExtendedInformation are defined (potentially to NULL)
//      SECPKG_INTERFACE_VERSION_2 indicates all fields through SetContextAttributes are defined (potentially to NULL)
//      SECPKG_INTERFACE_VERSION_3 indicates all fields through SetCredentialsAttributes are defined (potentially to NULL)
//      SECPKG_INTERFACE_VERSION_4 indicates all fields through ChangeAccountPassword are defined (potentially to NULL)
//      SECPKG_INTERFACE_VERSION_5 indicates all fields through UpdateCredentials are defined (potentially to NULL)
//      SECPKG_INTERFACE_VERSION_6 indicates all fields through ValidateTargetInfo are defined (potentially to NULL)
//      SECPKG_INTERFACE_VERSION_7 indicates all fields through PostLogonUserInfo are defined (potentially to NULL)
//      SECPKG_INTERFACE_VERSION_8 indicates all fields through GetRemoteSupplementalCreds are defined (potentially to NULL)
//      SECPKG_INTERFACE_VERSION_9 indicates all fields through GetTbalSupplementalCreds are defined (potentially to NULL)
//      SECPKG_INTERFACE_VERSION_10 indicates all fields through PostLogonUserSurrogate are defined (potentially to NULL)
//      SECPKG_INTERFACE_VERSION_11 indicates all fields through ExtractTargetInfo are defined (potentially to NULL)
//
// * Returned from SpUserModeInitializeFn to indicate the version of the auth package.
//      SECPKG_INTERFACE_VERSION indicates all fields through ImportContext are defined (potentially to NULL)
//      SECPKG_INTERFACE_VERSION_2 indicates all fields through MarshalAttributeData are defined (potentially to NULL)
//

#define SECPKG_INTERFACE_VERSION    0x00010000
#define SECPKG_INTERFACE_VERSION_2  0x00020000
#define SECPKG_INTERFACE_VERSION_3  0x00040000
#define SECPKG_INTERFACE_VERSION_4  0x00080000
#define SECPKG_INTERFACE_VERSION_5  0x00100000
#define SECPKG_INTERFACE_VERSION_6  0x00200000
#define SECPKG_INTERFACE_VERSION_7  0x00400000
#define SECPKG_INTERFACE_VERSION_8  0x00800000
#define SECPKG_INTERFACE_VERSION_9  0x01000000
#define SECPKG_INTERFACE_VERSION_10 0x02000000
#define SECPKG_INTERFACE_VERSION_11 0x04000000

typedef enum _KSEC_CONTEXT_TYPE {
    KSecPaged,
    KSecNonPaged
} KSEC_CONTEXT_TYPE;

typedef struct _KSEC_LIST_ENTRY {
    LIST_ENTRY List;
    LONG RefCount;
    ULONG Signature;
    PVOID OwningList;
    PVOID Reserved;
} KSEC_LIST_ENTRY, * PKSEC_LIST_ENTRY;

#define KsecInitializeListEntry( Entry, SigValue ) \
    ((PKSEC_LIST_ENTRY) Entry)->List.Flink = ((PKSEC_LIST_ENTRY) Entry)->List.Blink = NULL; \
    ((PKSEC_LIST_ENTRY) Entry)->RefCount = 1; \
    ((PKSEC_LIST_ENTRY) Entry)->Signature = SigValue; \
    ((PKSEC_LIST_ENTRY) Entry)->OwningList = NULL; \
    ((PKSEC_LIST_ENTRY) Entry)->Reserved = NULL;



typedef PVOID
(SEC_ENTRY KSEC_CREATE_CONTEXT_LIST)(
    _In_ KSEC_CONTEXT_TYPE Type
    );

typedef VOID
(SEC_ENTRY KSEC_INSERT_LIST_ENTRY)(
    _In_ PVOID List,
    _In_ PKSEC_LIST_ENTRY Entry
    );

typedef NTSTATUS
(SEC_ENTRY KSEC_REFERENCE_LIST_ENTRY)(
    _In_ PKSEC_LIST_ENTRY Entry,
    _In_ ULONG Signature,
    _In_ BOOLEAN RemoveNoRef
    );

typedef VOID
(SEC_ENTRY KSEC_DEREFERENCE_LIST_ENTRY)(
    _In_ PKSEC_LIST_ENTRY Entry,
    _Out_opt_ BOOLEAN * Delete
    );

typedef NTSTATUS
(SEC_ENTRY KSEC_SERIALIZE_WINNT_AUTH_DATA)(
    _In_ PVOID pvAuthData,
    _Out_ PULONG Size,
    _Outptr_result_bytebuffer_(*Size) PVOID * SerializedData );

typedef NTSTATUS
(SEC_ENTRY KSEC_SERIALIZE_SCHANNEL_AUTH_DATA)(
    _In_ PVOID pvAuthData,
    _Out_ PULONG Size,
    _Outptr_result_bytebuffer_(*Size) PVOID * SerializedData );

#ifndef MIDL_PASS

KSEC_CREATE_CONTEXT_LIST KSecCreateContextList;
KSEC_INSERT_LIST_ENTRY KSecInsertListEntry;
KSEC_REFERENCE_LIST_ENTRY KSecReferenceListEntry;
KSEC_DEREFERENCE_LIST_ENTRY KSecDereferenceListEntry;
KSEC_SERIALIZE_WINNT_AUTH_DATA KSecSerializeWinntAuthData;
KSEC_SERIALIZE_SCHANNEL_AUTH_DATA KSecSerializeSchannelAuthData;

#endif // not valid for MIDL_PASS

typedef KSEC_CREATE_CONTEXT_LIST * PKSEC_CREATE_CONTEXT_LIST;
typedef KSEC_INSERT_LIST_ENTRY * PKSEC_INSERT_LIST_ENTRY;
typedef KSEC_REFERENCE_LIST_ENTRY * PKSEC_REFERENCE_LIST_ENTRY;
typedef KSEC_DEREFERENCE_LIST_ENTRY * PKSEC_DEREFERENCE_LIST_ENTRY;
typedef KSEC_SERIALIZE_WINNT_AUTH_DATA * PKSEC_SERIALIZE_WINNT_AUTH_DATA;
typedef KSEC_SERIALIZE_SCHANNEL_AUTH_DATA * PKSEC_SERIALIZE_SCHANNEL_AUTH_DATA;

typedef PVOID
(SEC_ENTRY KSEC_LOCATE_PKG_BY_ID)(
    _In_ ULONG PackageId
    );

typedef KSEC_LOCATE_PKG_BY_ID * PKSEC_LOCATE_PKG_BY_ID;

#ifndef MIDL_PASS

KSEC_LOCATE_PKG_BY_ID KSecLocatePackageById;

#endif // not valid for MIDL_PASS

typedef struct _SECPKG_KERNEL_FUNCTIONS {
    PLSA_ALLOCATE_LSA_HEAP AllocateHeap;
    PLSA_FREE_LSA_HEAP FreeHeap;
    PKSEC_CREATE_CONTEXT_LIST CreateContextList;
    PKSEC_INSERT_LIST_ENTRY InsertListEntry;
    PKSEC_REFERENCE_LIST_ENTRY ReferenceListEntry;
    PKSEC_DEREFERENCE_LIST_ENTRY DereferenceListEntry;
    PKSEC_SERIALIZE_WINNT_AUTH_DATA SerializeWinntAuthData;
    PKSEC_SERIALIZE_SCHANNEL_AUTH_DATA SerializeSchannelAuthData;
    PKSEC_LOCATE_PKG_BY_ID LocatePackageById;
} SECPKG_KERNEL_FUNCTIONS, *PSECPKG_KERNEL_FUNCTIONS;

typedef NTSTATUS
(NTAPI KspInitPackageFn)(
    _In_ PSECPKG_KERNEL_FUNCTIONS    FunctionTable
    );

typedef NTSTATUS
(NTAPI KspDeleteContextFn)(
    _In_ LSA_SEC_HANDLE ContextId,
    _Out_ PLSA_SEC_HANDLE LsaContextId
    );

typedef NTSTATUS
(NTAPI KspInitContextFn)(
    _In_ LSA_SEC_HANDLE ContextId,
    _In_ PSecBuffer ContextData,
    _Out_ PLSA_SEC_HANDLE NewContextId
    );

typedef NTSTATUS
(NTAPI KspMakeSignatureFn)(
    _In_ LSA_SEC_HANDLE ContextId,
    _In_ ULONG fQOP,
    _In_ PSecBufferDesc Message,
    _In_ ULONG MessageSeqNo
    );

typedef NTSTATUS
(NTAPI KspVerifySignatureFn)(
    _In_ LSA_SEC_HANDLE ContextId,
    _In_ PSecBufferDesc Message,
    _In_ ULONG MessageSeqNo,
    _Out_opt_ PULONG pfQOP
    );


typedef NTSTATUS
(NTAPI KspSealMessageFn)(
    _In_ LSA_SEC_HANDLE ContextId,
    _In_ ULONG fQOP,
    _In_ PSecBufferDesc Message,
    _In_ ULONG MessageSeqNo
    );

typedef NTSTATUS
(NTAPI KspUnsealMessageFn)(
    _In_ LSA_SEC_HANDLE ContextId,
    _In_ PSecBufferDesc Message,
    _In_ ULONG MessageSeqNo,
    _Out_opt_ PULONG pfQOP
    );

typedef NTSTATUS
(NTAPI KspGetTokenFn)(
    _In_ LSA_SEC_HANDLE ContextId,
    _Out_opt_ PHANDLE ImpersonationToken,
    _Outptr_opt_ PACCESS_TOKEN * RawToken
    );

typedef NTSTATUS
(NTAPI KspQueryAttributesFn)(
    _In_ LSA_SEC_HANDLE ContextId,
    _In_ ULONG Attribute,
    _Inout_ PVOID Buffer
    );

typedef NTSTATUS
(NTAPI KspCompleteTokenFn)(
    _In_ LSA_SEC_HANDLE ContextId,
    _In_ PSecBufferDesc Token
    );


typedef NTSTATUS
(NTAPI KspMapHandleFn)(
    _In_ LSA_SEC_HANDLE ContextId,
    _Out_ PLSA_SEC_HANDLE LsaContextId
    );

typedef NTSTATUS
(NTAPI KspSetPagingModeFn)(
    _In_ BOOLEAN PagingMode
    );

typedef NTSTATUS
(NTAPI KspSerializeAuthDataFn)(
    _In_ PVOID pvAuthData,
    _Out_ PULONG Size,
    _Outptr_result_bytebuffer_(*Size) PVOID * SerializedData
    );

typedef struct _SECPKG_KERNEL_FUNCTION_TABLE {
    KspInitPackageFn *      Initialize;
    KspDeleteContextFn *    DeleteContext;
    KspInitContextFn *      InitContext;
    KspMapHandleFn *        MapHandle;
    KspMakeSignatureFn *    Sign;
    KspVerifySignatureFn *  Verify;
    KspSealMessageFn *      Seal;
    KspUnsealMessageFn *    Unseal;
    KspGetTokenFn *         GetToken;
    KspQueryAttributesFn *  QueryAttributes;
    KspCompleteTokenFn *    CompleteToken;
    SpExportSecurityContextFn * ExportContext;
    SpImportSecurityContextFn * ImportContext;
    KspSetPagingModeFn *    SetPackagePagingMode;
    KspSerializeAuthDataFn * SerializeAuthData;
} SECPKG_KERNEL_FUNCTION_TABLE, *PSECPKG_KERNEL_FUNCTION_TABLE;

SECURITY_STATUS
SEC_ENTRY
KSecRegisterSecurityProvider(
    _In_ PSECURITY_STRING    ProviderName,
    _In_ PSECPKG_KERNEL_FUNCTION_TABLE Table
    );

SECURITY_STATUS
SEC_ENTRY
KSecLocatePackage(
    _In_ PUNICODE_STRING PackageName,
    _Outptr_ PSECPKG_KERNEL_FUNCTION_TABLE * Package,
    _Out_ PULONG_PTR PackageId
    );

extern SECPKG_KERNEL_FUNCTIONS KspKernelFunctions;



#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#ifdef __cplusplus
}
#endif

#endif /* _NTSECPKG_ */

