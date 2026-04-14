/*++ BUILD Version: 0001    // Increment this if a change has global effects

Copyright (c) 2000 Microsoft Corporation

Module Name:

    wincred.h

Abstract:

    This module contains the public data structures and API definitions
    needed for the Credential Manager.


Author:


Revision History:

--*/

#ifndef _WINCRED_H_
#define _WINCRED_H_
#include <winapifamily.h>

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)


#if !defined(_ADVAPI32_)
#define WINADVAPI    DECLSPEC_IMPORT
#else
#define WINADVAPI
#endif

#if !defined(CREDUIAPI)
#if !defined(_CREDUI_)
#define CREDUIAPI    EXTERN_C DECLSPEC_IMPORT
#else
#define CREDUIAPI    EXTERN_C
#endif
#endif

#ifdef __cplusplus
extern "C" {
#endif

//
// Ensure PCtxtHandle is defined
//

#ifndef __SECHANDLE_DEFINED__
typedef struct _SecHandle
{
    ULONG_PTR dwLower ;
    ULONG_PTR dwUpper ;
} SecHandle, * PSecHandle ;

#define __SECHANDLE_DEFINED__
#endif // __SECHANDLE_DEFINED__

typedef PSecHandle PCtxtHandle;



//
// Ensure FILETIME is defined
//

#ifndef _WINBASE_
#ifndef _FILETIME_
#define _FILETIME_
typedef struct _FILETIME
    {
    DWORD dwLowDateTime;
    DWORD dwHighDateTime;
    }   FILETIME;

typedef struct _FILETIME *PFILETIME;

typedef struct _FILETIME *LPFILETIME;

#endif // !_FILETIME
#endif // _WINBASE_

//
// Ensure NTSTATUS is defined
//
#ifndef _NTDEF_
typedef _Return_type_success_(return >= 0) LONG NTSTATUS, *PNTSTATUS;
#endif


//-----------------------------------------------------------------------------
// Macros
//-----------------------------------------------------------------------------

//
// Macro to determine whether CredUIPromptForCredentials should be called upon a failed
//      authentication attempt.
//
// Implemented as a macro so that the caller can delay load credui.dll only if this
//      macro returns TRUE.
//
// Include only status codes that imply the username/password are wrong or that the
//      password is expired.  In the former case, asking for a another username or password
//      is appropriate.  In the later case, we put up a different dialog asking the
//      user to change the password on the server.
//
// Don't include status codes such as ERROR_ACCOUNT_DISABLED, ERROR_ACCOUNT_RESTRICTION,
//      ERROR_ACCOUNT_LOCKED_OUT, ERROR_ACCOUNT_EXPIRED, ERROR_LOGON_TYPE_NOT_GRANTED.
//      For those, the user isn't going to have another account so prompting him
//      won't help.
//
// STATUS_DOWNGRADE_DETECTED is included to handle the case where a corporate laptop
//      is brought to another LAN.  A downgrade attack will indeed be detected,
//      but we want to popup UI to allow the user to connect to resources in the
//      other LAN.
//
// STATUS_NO_SUCH_LOGON_SESSION and related error codes come from error mappings of
//      SEC_E_NO_CREDENTIALS and generally indicates that there are not credentials
//      available for SSO and some need to be provided.
//
// STATUS_NO_SUCH_USER comes when a username is mistyped or a certificate credential is
//      used where a username/password credential is expected.
//
// Don't use the CREDUIP_* macros directly.  Their definition is private to credui.dll.
//

// Don't require ntstatus.h
#define STATUS_LOGON_FAILURE             ((NTSTATUS)0xC000006DL)     // ntsubauth
#define STATUS_WRONG_PASSWORD            ((NTSTATUS)0xC000006AL)     // ntsubauth
#define STATUS_PASSWORD_EXPIRED          ((NTSTATUS)0xC0000071L)     // ntsubauth
#define STATUS_PASSWORD_MUST_CHANGE      ((NTSTATUS)0xC0000224L)    // ntsubauth
#define STATUS_ACCESS_DENIED             ((NTSTATUS)0xC0000022L)
#define STATUS_DOWNGRADE_DETECTED        ((NTSTATUS)0xC0000388L)
#define STATUS_AUTHENTICATION_FIREWALL_FAILED ((NTSTATUS)0xC0000413L)
#define STATUS_ACCOUNT_DISABLED          ((NTSTATUS)0xC0000072L)     // ntsubauth
#define STATUS_ACCOUNT_RESTRICTION       ((NTSTATUS)0xC000006EL)     // ntsubauth
#define STATUS_ACCOUNT_LOCKED_OUT        ((NTSTATUS)0xC0000234L)    // ntsubauth
#define STATUS_ACCOUNT_EXPIRED           ((NTSTATUS)0xC0000193L)    // ntsubauth
#define STATUS_LOGON_TYPE_NOT_GRANTED    ((NTSTATUS)0xC000015BL)
#define STATUS_NO_SUCH_LOGON_SESSION     ((NTSTATUS)0xC000005FL)
#define STATUS_NO_SUCH_USER              ((NTSTATUS)0xC0000064L)     // ntsubauth

// Don't require lmerr.h
#define NERR_BASE       2100
#define NERR_PasswordExpired    (NERR_BASE+142) /* The password of this user has expired. */

#define CREDUIP_IS_USER_PASSWORD_ERROR( _Status ) ( \
        (_Status) == STATUS_SMB_GUEST_LOGON_BLOCKED || \
        (_Status) == HRESULT_FROM_NT( STATUS_SMB_GUEST_LOGON_BLOCKED ) || \
        (_Status) == ERROR_SMB_GUEST_LOGON_BLOCKED || \
        (_Status) == __HRESULT_FROM_WIN32( ERROR_SMB_GUEST_LOGON_BLOCKED ) || \
        (_Status) == STATUS_SMB_GUEST_LOGON_BLOCKED_SIGNING_REQUIRED || \
        (_Status) == HRESULT_FROM_NT( STATUS_SMB_GUEST_LOGON_BLOCKED_SIGNING_REQUIRED ) || \
        (_Status) == ERROR_SMB_GUEST_LOGON_BLOCKED_SIGNING_REQUIRED || \
        (_Status) == __HRESULT_FROM_WIN32( ERROR_SMB_GUEST_LOGON_BLOCKED_SIGNING_REQUIRED ) || \
        (_Status) == ERROR_SMB_GUEST_ENCRYPTION_NOT_SUPPORTED || \
        (_Status) == __HRESULT_FROM_WIN32( ERROR_SMB_GUEST_ENCRYPTION_NOT_SUPPORTED ) || \
        (_Status) == ERROR_NTLM_BLOCKED || \
        (_Status) == __HRESULT_FROM_WIN32( ERROR_NTLM_BLOCKED ) || \
        (_Status) == ERROR_LOGON_FAILURE || \
        (_Status) == __HRESULT_FROM_WIN32( ERROR_LOGON_FAILURE ) || \
        (_Status) == STATUS_LOGON_FAILURE || \
        (_Status) == HRESULT_FROM_NT( STATUS_LOGON_FAILURE ) || \
        (_Status) == ERROR_ACCESS_DENIED || \
        (_Status) == __HRESULT_FROM_WIN32( ERROR_ACCESS_DENIED ) || \
        (_Status) == STATUS_ACCESS_DENIED || \
        (_Status) == HRESULT_FROM_NT( STATUS_ACCESS_DENIED ) || \
        (_Status) == ERROR_INVALID_PASSWORD || \
        (_Status) == __HRESULT_FROM_WIN32( ERROR_INVALID_PASSWORD ) || \
        (_Status) == STATUS_WRONG_PASSWORD || \
        (_Status) == HRESULT_FROM_NT( STATUS_WRONG_PASSWORD ) || \
        (_Status) == STATUS_NO_SUCH_USER || \
        (_Status) == HRESULT_FROM_NT( STATUS_NO_SUCH_USER ) || \
        (_Status) == ERROR_NO_SUCH_USER || \
        (_Status) == __HRESULT_FROM_WIN32( ERROR_NO_SUCH_USER ) || \
        (_Status) == ERROR_NO_SUCH_LOGON_SESSION || \
        (_Status) == __HRESULT_FROM_WIN32( ERROR_NO_SUCH_LOGON_SESSION ) || \
        (_Status) == STATUS_NO_SUCH_LOGON_SESSION || \
        (_Status) == HRESULT_FROM_NT( STATUS_NO_SUCH_LOGON_SESSION ) || \
        (_Status) == SEC_E_NO_CREDENTIALS || \
        (_Status) == SEC_E_LOGON_DENIED || \
        (_Status) == SEC_E_NO_CONTEXT || \
        (_Status) == STATUS_NO_SECURITY_CONTEXT )

#define CREDUIP_IS_DOWNGRADE_ERROR( _Status ) ( \
        (_Status) == ERROR_DOWNGRADE_DETECTED || \
        (_Status) == __HRESULT_FROM_WIN32( ERROR_DOWNGRADE_DETECTED ) || \
        (_Status) == STATUS_DOWNGRADE_DETECTED || \
        (_Status) == HRESULT_FROM_NT( STATUS_DOWNGRADE_DETECTED ) || \
        (_Status) == SEC_E_DOWNGRADE_DETECTED \
)

#define CREDUIP_IS_EXPIRED_ERROR( _Status ) ( \
        (_Status) == ERROR_PASSWORD_EXPIRED || \
        (_Status) == __HRESULT_FROM_WIN32( ERROR_PASSWORD_EXPIRED ) || \
        (_Status) == STATUS_PASSWORD_EXPIRED || \
        (_Status) == HRESULT_FROM_NT( STATUS_PASSWORD_EXPIRED ) || \
        (_Status) == ERROR_PASSWORD_MUST_CHANGE || \
        (_Status) == __HRESULT_FROM_WIN32( ERROR_PASSWORD_MUST_CHANGE ) || \
        (_Status) == STATUS_PASSWORD_MUST_CHANGE || \
        (_Status) == HRESULT_FROM_NT( STATUS_PASSWORD_MUST_CHANGE ) || \
        (_Status) == NERR_PasswordExpired || \
        (_Status) == __HRESULT_FROM_WIN32( NERR_PasswordExpired ) \
)

#define CREDUI_IS_AUTHENTICATION_ERROR( _Status ) ( \
        CREDUIP_IS_USER_PASSWORD_ERROR( _Status ) || \
        CREDUIP_IS_DOWNGRADE_ERROR( _Status ) || \
        CREDUIP_IS_EXPIRED_ERROR( _Status ) \
)

#define CREDUI_NO_PROMPT_AUTHENTICATION_ERROR( _Status ) ( \
        (_Status) == ERROR_AUTHENTICATION_FIREWALL_FAILED || \
        (_Status) == __HRESULT_FROM_WIN32( ERROR_AUTHENTICATION_FIREWALL_FAILED ) || \
        (_Status) == STATUS_AUTHENTICATION_FIREWALL_FAILED || \
        (_Status) == HRESULT_FROM_NT( STATUS_AUTHENTICATION_FIREWALL_FAILED ) || \
        (_Status) == ERROR_ACCOUNT_DISABLED || \
        (_Status) == __HRESULT_FROM_WIN32( ERROR_ACCOUNT_DISABLED ) || \
        (_Status) == STATUS_ACCOUNT_DISABLED || \
        (_Status) == HRESULT_FROM_NT( STATUS_ACCOUNT_DISABLED ) || \
        (_Status) == ERROR_ACCOUNT_RESTRICTION || \
        (_Status) == __HRESULT_FROM_WIN32( ERROR_ACCOUNT_RESTRICTION ) || \
        (_Status) == STATUS_ACCOUNT_RESTRICTION || \
        (_Status) == HRESULT_FROM_NT( STATUS_ACCOUNT_RESTRICTION ) || \
        (_Status) == ERROR_ACCOUNT_LOCKED_OUT || \
        (_Status) == __HRESULT_FROM_WIN32( ERROR_ACCOUNT_LOCKED_OUT ) || \
        (_Status) == STATUS_ACCOUNT_LOCKED_OUT || \
        (_Status) == HRESULT_FROM_NT( STATUS_ACCOUNT_LOCKED_OUT ) || \
        (_Status) == ERROR_ACCOUNT_EXPIRED || \
        (_Status) == __HRESULT_FROM_WIN32( ERROR_ACCOUNT_EXPIRED ) || \
        (_Status) == STATUS_ACCOUNT_EXPIRED || \
        (_Status) == HRESULT_FROM_NT( STATUS_ACCOUNT_EXPIRED ) || \
        (_Status) == ERROR_LOGON_TYPE_NOT_GRANTED || \
        (_Status) == __HRESULT_FROM_WIN32( ERROR_LOGON_TYPE_NOT_GRANTED ) || \
        (_Status) == STATUS_LOGON_TYPE_NOT_GRANTED || \
        (_Status) == HRESULT_FROM_NT( STATUS_LOGON_TYPE_NOT_GRANTED ) \
)

//-----------------------------------------------------------------------------
// Structures
//-----------------------------------------------------------------------------

//
// Credential Attribute
//

// Maximum length of the various credential string fields (in characters)
#define CRED_MAX_STRING_LENGTH 256

// Maximum length of the UserName field.  The worst case is <User>@<DnsDomain>
#define CRED_MAX_USERNAME_LENGTH (256+1+256)

// Maximum length of the TargetName field for CRED_TYPE_GENERIC (in characters)
#define CRED_MAX_GENERIC_TARGET_NAME_LENGTH 32767

// Maximum length of the TargetName field for CRED_TYPE_DOMAIN_* (in characters)
//      Largest one is <DfsRoot>\<DfsShare>
#define CRED_MAX_DOMAIN_TARGET_NAME_LENGTH (256+1+80)

// Maximum length of a target namespace
#define CRED_MAX_TARGETNAME_NAMESPACE_LENGTH (256)

// Maximum length of a target attribute
#define CRED_MAX_TARGETNAME_ATTRIBUTE_LENGTH (256)

// Maximum size of the Credential Attribute Value field (in bytes)
#define CRED_MAX_VALUE_SIZE (256)

// Maximum number of attributes per credential
#define CRED_MAX_ATTRIBUTES 64

typedef struct _CREDENTIAL_ATTRIBUTEA {
    LPSTR Keyword;
    DWORD Flags;
    DWORD ValueSize;
    LPBYTE Value;
} CREDENTIAL_ATTRIBUTEA, *PCREDENTIAL_ATTRIBUTEA;

typedef struct _CREDENTIAL_ATTRIBUTEW {
#ifdef MIDL_PASS
    [string] wchar_t * Keyword;
#else // MIDL_PASS
    LPWSTR  Keyword;
#endif // MIDL_PASS
    DWORD Flags;
#ifdef MIDL_PASS
    [range(0,CRED_MAX_VALUE_SIZE)]
#endif // MIDL_PASS
    DWORD ValueSize;
#ifdef MIDL_PASS
    [size_is(ValueSize)]
#endif // MIDL_PASS
    LPBYTE Value;
} CREDENTIAL_ATTRIBUTEW, *PCREDENTIAL_ATTRIBUTEW;

#ifdef UNICODE
typedef CREDENTIAL_ATTRIBUTEW CREDENTIAL_ATTRIBUTE;
typedef PCREDENTIAL_ATTRIBUTEW PCREDENTIAL_ATTRIBUTE;
#else
typedef CREDENTIAL_ATTRIBUTEA CREDENTIAL_ATTRIBUTE;
typedef PCREDENTIAL_ATTRIBUTEA PCREDENTIAL_ATTRIBUTE;
#endif // UNICODE

//
// Special values of the TargetName field
//
#define CRED_SESSION_WILDCARD_NAME_W L"*Session"
#define CRED_SESSION_WILDCARD_NAME_A "*Session"
#define CRED_UNIVERSAL_WILDCARD_W L'*'
#define CRED_UNIVERSAL_WILDCARD_A '*'
#define CRED_SESSION_WILDCARD_NAME_LENGTH (sizeof(CRED_SESSION_WILDCARD_NAME_A)-1)
#define CRED_TARGETNAME_DOMAIN_NAMESPACE_W L"Domain"
#define CRED_TARGETNAME_DOMAIN_NAMESPACE_A "Domain"
#define CRED_TARGETNAME_DOMAIN_NAMESPACE_LENGTH (sizeof(CRED_TARGETNAME_DOMAIN_NAMESPACE_A)-1)
#define CRED_UNIVERSAL_WILDCARD_W L'*'
#define CRED_UNIVERSAL_WILDCARD_A '*'
#define CRED_TARGETNAME_LEGACYGENERIC_NAMESPACE_W L"LegacyGeneric"
#define CRED_TARGETNAME_LEGACYGENERIC_NAMESPACE_A "LegacyGeneric"
#define CRED_TARGETNAME_LEGACYGENERIC_NAMESPACE_LENGTH (sizeof(CRED_TARGETNAME_LEGACYGENERIC_NAMESPACE_A)-1)
#define CRED_TARGETNAME_NAMESPACE_SEPERATOR_W L':'
#define CRED_TARGETNAME_NAMESPACE_SEPERATOR_A ':'
#define CRED_TARGETNAME_ATTRIBUTE_SEPERATOR_W L'='
#define CRED_TARGETNAME_ATTRIBUTE_SEPERATOR_A '='
#define CRED_TARGETNAME_DOMAIN_EXTENDED_USERNAME_SEPARATOR_W L'|'
#define CRED_TARGETNAME_DOMAIN_EXTENDED_USERNAME_SEPARATOR_A '|'
#define CRED_TARGETNAME_ATTRIBUTE_TARGET_W L"target"
#define CRED_TARGETNAME_ATTRIBUTE_TARGET_A "target"
#define CRED_TARGETNAME_ATTRIBUTE_TARGET_LENGTH (sizeof(CRED_TARGETNAME_ATTRIBUTE_TARGET_A)-1)
#define CRED_TARGETNAME_ATTRIBUTE_NAME_W L"name"
#define CRED_TARGETNAME_ATTRIBUTE_NAME_A "name"
#define CRED_TARGETNAME_ATTRIBUTE_NAME_LENGTH (sizeof(CRED_TARGETNAME_ATTRIBUTE_NAME_A)-1)
#define CRED_TARGETNAME_ATTRIBUTE_BATCH_W L"batch"
#define CRED_TARGETNAME_ATTRIBUTE_BATCH_A "batch"
#define CRED_TARGETNAME_ATTRIBUTE_BATCH_LENGTH (sizeof(CRED_TARGETNAME_ATTRIBUTE_BATCH_A)-1)
#define CRED_TARGETNAME_ATTRIBUTE_INTERACTIVE_W L"interactive"
#define CRED_TARGETNAME_ATTRIBUTE_INTERACTIVE_A "interactive"
#define CRED_TARGETNAME_ATTRIBUTE_INTERACTIVE_LENGTH (sizeof(CRED_TARGETNAME_ATTRIBUTE_INTERACTIVE_A)-1)
#define CRED_TARGETNAME_ATTRIBUTE_SERVICE_W L"service"
#define CRED_TARGETNAME_ATTRIBUTE_SERVICE_A "service"
#define CRED_TARGETNAME_ATTRIBUTE_SERVICE_LENGTH (sizeof(CRED_TARGETNAME_ATTRIBUTE_SERVICE_A)-1)
#define CRED_TARGETNAME_ATTRIBUTE_NETWORK_W L"network"
#define CRED_TARGETNAME_ATTRIBUTE_NETWORK_A "network"
#define CRED_TARGETNAME_ATTRIBUTE_NETWORK_LENGTH (sizeof(CRED_TARGETNAME_ATTRIBUTE_NETWORK_A)-1)
#define CRED_TARGETNAME_ATTRIBUTE_NETWORKCLEARTEXT_W L"networkcleartext"
#define CRED_TARGETNAME_ATTRIBUTE_NETWORKCLEARTEXT_A "networkcleartext"
#define CRED_TARGETNAME_ATTRIBUTE_NETWORKCLEARTEXT_LENGTH (sizeof(CRED_TARGETNAME_ATTRIBUTE_NETWORKCLEARTEXT_A)-1)
#define CRED_TARGETNAME_ATTRIBUTE_REMOTEINTERACTIVE_W L"remoteinteractive"
#define CRED_TARGETNAME_ATTRIBUTE_REMOTEINTERACTIVE_A "remoteinteractive"
#define CRED_TARGETNAME_ATTRIBUTE_REMOTEINTERACTIVE_LENGTH (sizeof(CRED_TARGETNAME_ATTRIBUTE_REMOTEINTERACTIVE_A)-1)
#define CRED_TARGETNAME_ATTRIBUTE_CACHEDINTERACTIVE_W L"cachedinteractive"
#define CRED_TARGETNAME_ATTRIBUTE_CACHEDINTERACTIVE_A "cachedinteractive"
#define CRED_TARGETNAME_ATTRIBUTE_CACHEDINTERACTIVE_LENGTH (sizeof(CRED_TARGETNAME_ATTRIBUTE_CACHEDINTERACTIVE_A)-1)

#ifdef UNICODE
#define CRED_SESSION_WILDCARD_NAME CRED_SESSION_WILDCARD_NAME_W
#define CRED_TARGETNAME_DOMAIN_NAMESPACE CRED_TARGETNAME_DOMAIN_NAMESPACE_W
#define CRED_UNIVERSAL_WILDCARD = CRED_UNIVERSAL_WILDCARD_W
#define CRED_TARGETNAME_NAMESPACE_SEPERATOR = CRED_TARGETNAME_NAMESPACE_SEPERATOR_W
#define CRED_TARGETNAME_ATTRIBUTE_SEPERATOR = CRED_TARGETNAME_ATTRIBUTE_SEPERATOR_W
#define CRED_TARGETNAME_ATTRIBUTE_NAME CRED_TARGETNAME_ATTRIBUTE_NAME_W
#define CRED_TARGETNAME_ATTRIBUTE_TARGET CRED_TARGETNAME_ATTRIBUTE_TARGET_W
#define CRED_TARGETNAME_ATTRIBUTE_BATCH CRED_TARGETNAME_ATTRIBUTE_BATCH_W
#define CRED_TARGETNAME_ATTRIBUTE_INTERACTIVE CRED_TARGETNAME_ATTRIBUTE_INTERACTIVE_W
#define CRED_TARGETNAME_ATTRIBUTE_SERVICE CRED_TARGETNAME_ATTRIBUTE_SERVICE_W
#define CRED_TARGETNAME_ATTRIBUTE_NETWORK CRED_TARGETNAME_ATTRIBUTE_NETWORK_W
#define CRED_TARGETNAME_ATTRIBUTE_NETWORKCLEARTEXT CRED_TARGETNAME_ATTRIBUTE_NETWORKCLEARTEXT_W
#define CRED_TARGETNAME_ATTRIBUTE_REMOTEINTERACTIVE CRED_TARGETNAME_ATTRIBUTE_REMOTEINTERACTIVE_W
#define CRED_TARGETNAME_ATTRIBUTE_CACHEDINTERACTIVE CRED_TARGETNAME_ATTRIBUTE_CACHEDINTERACTIVE_W

#else
#define CRED_SESSION_WILDCARD_NAME CRED_SESSION_WILDCARD_NAME_A
#define CRED_TARGETNAME_DOMAIN_NAMESPACE CRED_TARGETNAME_DOMAIN_NAMESPACE_A
#define CRED_UNIVERSAL_WILDCARD = CRED_UNIVERSAL_WILDCARD_A
#define CRED_TARGETNAME_NAMESPACE_SEPERATOR = CRED_TARGETNAME_NAMESPACE_SEPERATOR_A
#define CRED_TARGETNAME_ATTRIBUTE_SEPERATOR = CRED_TARGETNAME_ATTRIBUTE_SEPERATOR_A
#define CRED_TARGETNAME_ATTRIBUTE_NAME CRED_TARGETNAME_ATTRIBUTE_NAME_A
#define CRED_TARGETNAME_ATTRIBUTE_TARGET CRED_TARGETNAME_ATTRIBUTE_TARGET_A
#define CRED_TARGETNAME_ATTRIBUTE_BATCH CRED_TARGETNAME_ATTRIBUTE_BATCH_A
#define CRED_TARGETNAME_ATTRIBUTE_INTERACTIVE CRED_TARGETNAME_ATTRIBUTE_INTERACTIVE_A
#define CRED_TARGETNAME_ATTRIBUTE_SERVICE CRED_TARGETNAME_ATTRIBUTE_SERVICE_A
#define CRED_TARGETNAME_ATTRIBUTE_NETWORK CRED_TARGETNAME_ATTRIBUTE_NETWORK_A
#define CRED_TARGETNAME_ATTRIBUTE_NETWORKCLEARTEXT CRED_TARGETNAME_ATTRIBUTE_NETWORKCLEARTEXT_A
#define CRED_TARGETNAME_ATTRIBUTE_REMOTEINTERACTIVE CRED_TARGETNAME_ATTRIBUTE_REMOTEINTERACTIVE_A
#define CRED_TARGETNAME_ATTRIBUTE_CACHEDINTERACTIVE CRED_TARGETNAME_ATTRIBUTE_CACHEDINTERACTIVE_A
#endif // UNICODE


//
// Add\Extract Logon type from flags
//
#define CRED_LOGON_TYPES_MASK             0xF000  // Mask to get logon types

#define CredAppendLogonTypeToFlags(Flags, LogonType)      (Flags) |= ((LogonType) << 12)
#define CredGetLogonTypeFromFlags(Flags)                  ((SECURITY_LOGON_TYPE)(((Flags) & CRED_LOGON_TYPES_MASK) >> 12))
#define CredRemoveLogonTypeFromFlags(Flags)               (Flags) &= ~CRED_LOGON_TYPES_MASK

//
// Values of the Credential Flags field.
//
#define CRED_FLAGS_PASSWORD_FOR_CERT    0x0001
#define CRED_FLAGS_PROMPT_NOW           0x0002
#define CRED_FLAGS_USERNAME_TARGET      0x0004
#define CRED_FLAGS_OWF_CRED_BLOB        0x0008
#define CRED_FLAGS_REQUIRE_CONFIRMATION 0x0010

//
//  Valid only for return and only with CredReadDomainCredentials().
//  Indicates credential was returned due to wildcard match
//  of targetname with credential.
//

#define CRED_FLAGS_WILDCARD_MATCH       0x0020

//
// Valid only for return
// Indicates that the credential is VSM protected
//

#define CRED_FLAGS_VSM_PROTECTED        0x0040

#define CRED_FLAGS_NGC_CERT             0x0080

//
// Mask of all valid flags
//

#define CRED_FLAGS_VALID_FLAGS          0xF0FF

//
//  Bit mask for only those flags which can be passed to the credman
//  APIs.
//

#define CRED_FLAGS_VALID_INPUT_FLAGS    0xF09F

//
// Values of the Credential Type field.
//
#define CRED_TYPE_GENERIC               1
#define CRED_TYPE_DOMAIN_PASSWORD       2
#define CRED_TYPE_DOMAIN_CERTIFICATE    3
#define CRED_TYPE_DOMAIN_VISIBLE_PASSWORD 4
#define CRED_TYPE_GENERIC_CERTIFICATE   5
#define CRED_TYPE_DOMAIN_EXTENDED       6
#define CRED_TYPE_MAXIMUM               7       // Maximum supported cred type
#define CRED_TYPE_MAXIMUM_EX  (CRED_TYPE_MAXIMUM+1000)  // Allow new applications to run on old OSes

//
// Maximum size of the CredBlob field (in bytes)
//

#define CRED_MAX_CREDENTIAL_BLOB_SIZE   (5*512)

//
// Values of the Credential Persist field
//
#define CRED_PERSIST_NONE               0
#define CRED_PERSIST_SESSION            1
#define CRED_PERSIST_LOCAL_MACHINE      2
#define CRED_PERSIST_ENTERPRISE         3



//
// A credential
//
typedef struct _CREDENTIALA {
    DWORD Flags;
    DWORD Type;
    LPSTR TargetName;
    LPSTR Comment;
    FILETIME LastWritten;
    DWORD CredentialBlobSize;
    _Field_size_bytes_(CredentialBlobSize) LPBYTE CredentialBlob;
    DWORD Persist;
    DWORD AttributeCount;
    PCREDENTIAL_ATTRIBUTEA Attributes;
    LPSTR TargetAlias;
    LPSTR UserName;
} CREDENTIALA, *PCREDENTIALA;

typedef struct _CREDENTIALW {
    DWORD Flags;
    DWORD Type;
#ifdef MIDL_PASS
    [string,max_is(CRED_MAX_GENERIC_TARGET_NAME_LENGTH-1)] wchar_t *TargetName;
#else // MIDL_PASS
    LPWSTR TargetName;
#endif // MIDL_PASS
#ifdef MIDL_PASS
    [string,max_is(CRED_MAX_STRING_LENGTH-1)] wchar_t *Comment;
#else // MIDL_PASS
    LPWSTR Comment;
#endif // MIDL_PASS
    FILETIME LastWritten;
#ifdef MIDL_PASS
    [range(0,CRED_MAX_CREDENTIAL_BLOB_SIZE)]
#endif // MIDL_PASS
    DWORD CredentialBlobSize;
#ifdef MIDL_PASS
    [size_is(CredentialBlobSize)]
#endif // MIDL_PASS
    LPBYTE CredentialBlob;
    DWORD Persist;
#ifdef MIDL_PASS
    [range(0,CRED_MAX_ATTRIBUTES)]
#endif // MIDL_PASS
    DWORD AttributeCount;
#ifdef MIDL_PASS
    [size_is(AttributeCount)]
#endif // MIDL_PASS
    PCREDENTIAL_ATTRIBUTEW Attributes;
#ifdef MIDL_PASS
    [string,max_is(CRED_MAX_STRING_LENGTH-1)] wchar_t *TargetAlias;
#else // MIDL_PASS
    LPWSTR TargetAlias;
#endif // MIDL_PASS
#ifdef MIDL_PASS
    [string,max_is(CRED_MAX_USERNAME_LENGTH-1)] wchar_t *UserName;
#else // MIDL_PASS
    LPWSTR UserName;
#endif // MIDL_PASS
} CREDENTIALW, *PCREDENTIALW;

#ifdef UNICODE
typedef CREDENTIALW CREDENTIAL;
typedef PCREDENTIALW PCREDENTIAL;
#else
typedef CREDENTIALA CREDENTIAL;
typedef PCREDENTIALA PCREDENTIAL;
#endif // UNICODE

//
// Value of the Flags field in CREDENTIAL_TARGET_INFORMATION
//

#define CRED_TI_SERVER_FORMAT_UNKNOWN   0x0001  // Don't know if server name is DNS or netbios format
#define CRED_TI_DOMAIN_FORMAT_UNKNOWN   0x0002  // Don't know if domain name is DNS or netbios format
#define CRED_TI_ONLY_PASSWORD_REQUIRED  0x0004  // Server only requires a password and not a username
#define CRED_TI_USERNAME_TARGET         0x0008  // TargetName is username
#define CRED_TI_CREATE_EXPLICIT_CRED    0x0010  // When creating a cred, create one named TargetInfo->TargetName
#define CRED_TI_WORKGROUP_MEMBER        0x0020  // Indicates the machine is a member of a workgroup
#define CRED_TI_DNSTREE_IS_DFS_SERVER   0x0040  // used to tell credman that the DNSTreeName could be DFS server
#define CRED_TI_VALID_FLAGS             0xF07F


//
// A credential target
//

typedef struct _CREDENTIAL_TARGET_INFORMATIONA {
    LPSTR TargetName;
    LPSTR NetbiosServerName;
    LPSTR DnsServerName;
    LPSTR NetbiosDomainName;
    LPSTR DnsDomainName;
    LPSTR DnsTreeName;
    LPSTR PackageName;
    ULONG Flags;
    DWORD CredTypeCount;
    LPDWORD CredTypes;
} CREDENTIAL_TARGET_INFORMATIONA, *PCREDENTIAL_TARGET_INFORMATIONA;

typedef struct _CREDENTIAL_TARGET_INFORMATIONW {
#ifdef MIDL_PASS
    [string,max_is(CRED_MAX_DOMAIN_TARGET_NAME_LENGTH-1)] wchar_t *TargetName;
    [string,max_is(CRED_MAX_DOMAIN_TARGET_NAME_LENGTH-1)] wchar_t *NetbiosServerName;
    [string,max_is(CRED_MAX_DOMAIN_TARGET_NAME_LENGTH-1)] wchar_t *DnsServerName;
    [string,max_is(CRED_MAX_DOMAIN_TARGET_NAME_LENGTH-1)] wchar_t *NetbiosDomainName;
    [string,max_is(CRED_MAX_DOMAIN_TARGET_NAME_LENGTH-1)] wchar_t *DnsDomainName;
    [string,max_is(CRED_MAX_DOMAIN_TARGET_NAME_LENGTH-1)] wchar_t *DnsTreeName;
    [string,max_is(CRED_MAX_STRING_LENGTH-1)] wchar_t *PackageName;
#else // MIDL_PASS
    LPWSTR TargetName;
    LPWSTR NetbiosServerName;
    LPWSTR DnsServerName;
    LPWSTR NetbiosDomainName;
    LPWSTR DnsDomainName;
    LPWSTR DnsTreeName;
    LPWSTR PackageName;
#endif // MIDL_PASS
    ULONG Flags;
#ifdef MIDL_PASS
    [range(0,CRED_TYPE_MAXIMUM_EX)]
#endif // MIDL_PASS
    DWORD CredTypeCount;
#ifdef MIDL_PASS
    [size_is(CredTypeCount)]
#endif // MIDL_PASS
    LPDWORD CredTypes;
} CREDENTIAL_TARGET_INFORMATIONW, *PCREDENTIAL_TARGET_INFORMATIONW;

#ifdef UNICODE
typedef CREDENTIAL_TARGET_INFORMATIONW CREDENTIAL_TARGET_INFORMATION;
typedef PCREDENTIAL_TARGET_INFORMATIONW PCREDENTIAL_TARGET_INFORMATION;
#else
typedef CREDENTIAL_TARGET_INFORMATIONA CREDENTIAL_TARGET_INFORMATION;
typedef PCREDENTIAL_TARGET_INFORMATIONA PCREDENTIAL_TARGET_INFORMATION;
#endif // UNICODE

//
// Certificate credential information
//
// The cbSize should be the size of the structure, sizeof(CERT_CREDENTIAL_INFO),
// rgbHashofCert is the hash of the cert which is to be used as the credential.
//

#define CERT_HASH_LENGTH        20  // SHA1 hashes are used for cert hashes

typedef struct _CERT_CREDENTIAL_INFO {
    ULONG cbSize;
    UCHAR rgbHashOfCert[CERT_HASH_LENGTH];
} CERT_CREDENTIAL_INFO, *PCERT_CREDENTIAL_INFO;

//
// Username Target credential information
//
// This credential can be pass to LsaLogonUser to ask it to find a credential with a
// TargetName of UserName.
//

typedef struct _USERNAME_TARGET_CREDENTIAL_INFO {
    LPWSTR UserName;
} USERNAME_TARGET_CREDENTIAL_INFO, *PUSERNAME_TARGET_CREDENTIAL_INFO;

//
// Marshaled credential blob information.
//

typedef struct _BINARY_BLOB_CREDENTIAL_INFO {
    ULONG cbBlob;
    LPBYTE pbBlob;
} BINARY_BLOB_CREDENTIAL_INFO, *PBINARY_BLOB_CREDENTIAL_INFO;

//
// Credential type for credential marshaling routines
//

typedef enum _CRED_MARSHAL_TYPE {
    CertCredential = 1,
    UsernameTargetCredential,
    BinaryBlobCredential,
    UsernameForPackedCredentials,  // internal only, reserved
    BinaryBlobForSystem,  // internal only, via CredProtectEx
} CRED_MARSHAL_TYPE, *PCRED_MARSHAL_TYPE;

//
// Protection type for credential providers secret protection routines
//

typedef enum _CRED_PROTECTION_TYPE {
    CredUnprotected,
    CredUserProtection,
    CredTrustedProtection,
    CredForSystemProtection,
} CRED_PROTECTION_TYPE, *PCRED_PROTECTION_TYPE;

//
// Values for authentication buffers packing
//
#define CRED_PACK_PROTECTED_CREDENTIALS      0x1
#define CRED_PACK_WOW_BUFFER                 0x2
#define CRED_PACK_GENERIC_CREDENTIALS        0x4
#define CRED_PACK_ID_PROVIDER_CREDENTIALS    0x8

//
// Credential UI info
//

#define _CREDUI_INFO_DEFINED

typedef struct _CREDUI_INFOA
{
    DWORD cbSize;
    HWND hwndParent;
    PCSTR pszMessageText;
    PCSTR pszCaptionText;
    HBITMAP hbmBanner;
} CREDUI_INFOA, *PCREDUI_INFOA;

typedef struct _CREDUI_INFOW
{
    DWORD cbSize;
    HWND hwndParent;
    PCWSTR pszMessageText;
    PCWSTR pszCaptionText;
    HBITMAP hbmBanner;
} CREDUI_INFOW, *PCREDUI_INFOW;

#ifdef UNICODE
typedef CREDUI_INFOW CREDUI_INFO;
typedef PCREDUI_INFOW PCREDUI_INFO;
#else
typedef CREDUI_INFOA CREDUI_INFO;
typedef PCREDUI_INFOA PCREDUI_INFO;
#endif

//-----------------------------------------------------------------------------
// Values
//-----------------------------------------------------------------------------

// String length limits:

#define CREDUI_MAX_MESSAGE_LENGTH           1024
#define CREDUI_MAX_CAPTION_LENGTH           128
#define CREDUI_MAX_GENERIC_TARGET_LENGTH    CRED_MAX_GENERIC_TARGET_NAME_LENGTH
#define CREDUI_MAX_DOMAIN_TARGET_LENGTH     CRED_MAX_DOMAIN_TARGET_NAME_LENGTH

//
//  Username can be in <domain>\<user> or <user>@<domain>
//  Length in characters, not including NULL termination.
//

#define CREDUI_MAX_USERNAME_LENGTH          CRED_MAX_USERNAME_LENGTH
#define CREDUI_MAX_PASSWORD_LENGTH          (512 / 2)


//  Packed credential returned by SspiEncodeAuthIdentityAsStrings().
//  Length in characters, not including NULL termination.
//

#define CREDUI_MAX_PACKED_CREDENTIALS_LENGTH    ((MAXUSHORT / 2) - 2)

// maximum length in bytes for binary credential blobs

#define CREDUI_MAX_CREDENTIALS_BLOB_SIZE        (MAXUSHORT)

//
// Flags for CredUIPromptForCredentials and/or CredUICmdLinePromptForCredentials
//

#define CREDUI_FLAGS_INCORRECT_PASSWORD     0x00001     // indicates the username is valid, but password is not
#define CREDUI_FLAGS_DO_NOT_PERSIST         0x00002     // Do not show "Save" checkbox, and do not persist credentials
#define CREDUI_FLAGS_REQUEST_ADMINISTRATOR  0x00004     // Populate list box with admin accounts
#define CREDUI_FLAGS_EXCLUDE_CERTIFICATES   0x00008     // do not include certificates in the drop list
#define CREDUI_FLAGS_REQUIRE_CERTIFICATE    0x00010
#define CREDUI_FLAGS_SHOW_SAVE_CHECK_BOX    0x00040
#define CREDUI_FLAGS_ALWAYS_SHOW_UI         0x00080
#define CREDUI_FLAGS_REQUIRE_SMARTCARD      0x00100
#define CREDUI_FLAGS_PASSWORD_ONLY_OK       0x00200
#define CREDUI_FLAGS_VALIDATE_USERNAME      0x00400
#define CREDUI_FLAGS_COMPLETE_USERNAME      0x00800     //
#define CREDUI_FLAGS_PERSIST                0x01000     // Do not show "Save" checkbox, but persist credentials anyway
#define CREDUI_FLAGS_SERVER_CREDENTIAL      0x04000
#define CREDUI_FLAGS_EXPECT_CONFIRMATION    0x20000     // do not persist unless caller later confirms credential via CredUIConfirmCredential() api
#define CREDUI_FLAGS_GENERIC_CREDENTIALS    0x40000     // Credential is a generic credential
#define CREDUI_FLAGS_USERNAME_TARGET_CREDENTIALS 0x80000 // Credential has a username as the target
#define CREDUI_FLAGS_KEEP_USERNAME         0x100000             // don't allow the user to change the supplied username


//
// Mask of flags valid for CredUIPromptForCredentials
//
#define CREDUI_FLAGS_PROMPT_VALID ( \
        CREDUI_FLAGS_INCORRECT_PASSWORD | \
        CREDUI_FLAGS_DO_NOT_PERSIST | \
        CREDUI_FLAGS_REQUEST_ADMINISTRATOR | \
        CREDUI_FLAGS_EXCLUDE_CERTIFICATES | \
        CREDUI_FLAGS_REQUIRE_CERTIFICATE | \
        CREDUI_FLAGS_SHOW_SAVE_CHECK_BOX | \
        CREDUI_FLAGS_ALWAYS_SHOW_UI | \
        CREDUI_FLAGS_REQUIRE_SMARTCARD | \
        CREDUI_FLAGS_PASSWORD_ONLY_OK | \
        CREDUI_FLAGS_VALIDATE_USERNAME | \
        CREDUI_FLAGS_COMPLETE_USERNAME | \
        CREDUI_FLAGS_PERSIST | \
        CREDUI_FLAGS_SERVER_CREDENTIAL | \
        CREDUI_FLAGS_EXPECT_CONFIRMATION | \
        CREDUI_FLAGS_GENERIC_CREDENTIALS | \
        CREDUI_FLAGS_USERNAME_TARGET_CREDENTIALS | \
        CREDUI_FLAGS_KEEP_USERNAME )


//
// Flags for CredUIPromptForWindowsCredentials and CPUS_CREDUI Usage Scenarios
//

#define CREDUIWIN_GENERIC                   0x00000001  // Plain text username/password is being requested
#define CREDUIWIN_CHECKBOX                  0x00000002  // Show the Save Credential checkbox
#define CREDUIWIN_AUTHPACKAGE_ONLY          0x00000010  // Only Cred Providers that support the input auth package should enumerate
#define CREDUIWIN_IN_CRED_ONLY              0x00000020  // Only the incoming cred for the specific auth package should be enumerated
#define CREDUIWIN_ENUMERATE_ADMINS          0x00000100  // Cred Providers should enumerate administrators only
#define CREDUIWIN_ENUMERATE_CURRENT_USER    0x00000200  // Only the incoming cred for the specific auth package should be enumerated
#define CREDUIWIN_SECURE_PROMPT             0x00001000  // The Credui prompt should be displayed on the secure desktop
#define CREDUIWIN_PREPROMPTING              0X00002000  // CredUI is invoked by SspiPromptForCredentials and the client is prompting before a prior handshake
#define CREDUIWIN_PACK_32_WOW               0x10000000  // Tell the credential provider it should be packing its Auth Blob 32 bit even though it is running 64 native
#define CREDUIWIN_USE_V2                    0x00000040  // For the rejuvenated CREDUI experience
#if (NTDDI_VERSION >= NTDDI_WIN10_RS1)
#define CREDUIWIN_IGNORE_CLOUDAUTHORITY_NAME            0x00040000  // Tell the credential provider it should not pack AAD authority name
#endif
#if (NTDDI_VERSION >= NTDDI_WIN10_RS5)
#define CREDUIWIN_DOWNLEVEL_HELLO_AS_SMART_CARD         0x80000000  // Force collected Hello credentials to be packed in a smart card auth buffer.
#endif


#if (NTDDI_VERSION >= NTDDI_WIN10_RS5)

#define CREDUIWIN_VALID_FLAGS            ( \
        CREDUIWIN_GENERIC                | \
        CREDUIWIN_CHECKBOX               | \
        CREDUIWIN_AUTHPACKAGE_ONLY       | \
        CREDUIWIN_IN_CRED_ONLY           | \
        CREDUIWIN_ENUMERATE_ADMINS       | \
        CREDUIWIN_ENUMERATE_CURRENT_USER | \
        CREDUIWIN_SECURE_PROMPT          | \
        CREDUIWIN_PREPROMPTING           | \
        CREDUIWIN_PACK_32_WOW            | \
        CREDUIWIN_IGNORE_CLOUDAUTHORITY_NAME    | \
        CREDUIWIN_DOWNLEVEL_HELLO_AS_SMART_CARD )

#elif (NTDDI_VERSION >= NTDDI_WIN10_RS1)

#define CREDUIWIN_VALID_FLAGS            ( \
        CREDUIWIN_GENERIC                | \
        CREDUIWIN_CHECKBOX               | \
        CREDUIWIN_AUTHPACKAGE_ONLY       | \
        CREDUIWIN_IN_CRED_ONLY           | \
        CREDUIWIN_ENUMERATE_ADMINS       | \
        CREDUIWIN_ENUMERATE_CURRENT_USER | \
        CREDUIWIN_SECURE_PROMPT          | \
        CREDUIWIN_PREPROMPTING           | \
        CREDUIWIN_PACK_32_WOW            | \
        CREDUIWIN_IGNORE_CLOUDAUTHORITY_NAME    )

#else

#define CREDUIWIN_VALID_FLAGS            ( \
        CREDUIWIN_GENERIC                | \
        CREDUIWIN_CHECKBOX               | \
        CREDUIWIN_AUTHPACKAGE_ONLY       | \
        CREDUIWIN_IN_CRED_ONLY           | \
        CREDUIWIN_ENUMERATE_ADMINS       | \
        CREDUIWIN_ENUMERATE_CURRENT_USER | \
        CREDUIWIN_SECURE_PROMPT          | \
        CREDUIWIN_PREPROMPTING           | \
        CREDUIWIN_PACK_32_WOW            )

#endif

// AuthPackageId in case of Back Button on Picker Screeen
// CAD----- : Credui Authpackage iD's
#define BACK_BUTTON_IDENTIFY_AUTH_PACKAGE  0xCAD00001
#define CREDUI_FOOTER_LINK_AUTHPACKAGE_ID   0XCAD0002
#define CREDUI_PICKERSCREEN_AUTHPACKAGE_ID  0XCAD0003


//-----------------------------------------------------------------------------
// Functions
//-----------------------------------------------------------------------------


//
// Values of flags to CredWrite and CredWriteDomainCredentials
//

#define CRED_PRESERVE_CREDENTIAL_BLOB 0x1

WINADVAPI
BOOL
WINAPI
CredWriteW (
    _In_ PCREDENTIALW Credential,
    _In_ DWORD Flags
    );

WINADVAPI
BOOL
WINAPI
CredWriteA (
    _In_ PCREDENTIALA Credential,
    _In_ DWORD Flags
    );

#ifdef UNICODE
#define CredWrite CredWriteW
#else
#define CredWrite CredWriteA
#endif // UNICODE


WINADVAPI
BOOL
WINAPI
CredReadW (
    _In_ LPCWSTR TargetName,
    _In_ DWORD Type,
    _Reserved_ DWORD Flags,
    _Out_ PCREDENTIALW *Credential
    );

WINADVAPI
BOOL
WINAPI
CredReadA (
    _In_ LPCSTR TargetName,
    _In_ DWORD Type,
    _Reserved_ DWORD Flags,
    _Out_ PCREDENTIALA *Credential
    );

#ifdef UNICODE
#define CredRead CredReadW
#else
#define CredRead CredReadA
#endif // UNICODE


//
// Values of flags to CredEnumerate
//

#define CRED_ENUMERATE_ALL_CREDENTIALS 0x1

WINADVAPI
BOOL
WINAPI
CredEnumerateW (
    _In_opt_ LPCWSTR Filter,
    _Reserved_ DWORD Flags,
    _Out_ DWORD *Count,
    _Outptr_result_buffer_(*Count) PCREDENTIALW **Credential
    );

WINADVAPI
BOOL
WINAPI
CredEnumerateA (
    _In_opt_ LPCSTR Filter,
    _Reserved_ DWORD Flags,
    _Out_ DWORD *Count,
    _Outptr_result_buffer_(*Count) PCREDENTIALA **Credential
    );

#ifdef UNICODE
#define CredEnumerate CredEnumerateW
#else
#define CredEnumerate CredEnumerateA
#endif // UNICODE


WINADVAPI
BOOL
WINAPI
CredWriteDomainCredentialsW (
    _In_ PCREDENTIAL_TARGET_INFORMATIONW TargetInfo,
    _In_ PCREDENTIALW Credential,
    _In_ DWORD Flags
    );

WINADVAPI
BOOL
WINAPI
CredWriteDomainCredentialsA (
    _In_ PCREDENTIAL_TARGET_INFORMATIONA TargetInfo,
    _In_ PCREDENTIALA Credential,
    _In_ DWORD Flags
    );

#ifdef UNICODE
#define CredWriteDomainCredentials CredWriteDomainCredentialsW
#else
#define CredWriteDomainCredentials CredWriteDomainCredentialsA
#endif // UNICODE



//
// Values of flags to CredReadDomainCredentials
//

#define CRED_CACHE_TARGET_INFORMATION 0x1


WINADVAPI
BOOL
WINAPI
CredReadDomainCredentialsW (
    _In_ PCREDENTIAL_TARGET_INFORMATIONW TargetInfo,
    _In_ DWORD Flags,
    _Out_ DWORD *Count,
    _Outptr_result_buffer_(*Count) PCREDENTIALW **Credential
    );

WINADVAPI
BOOL
WINAPI
CredReadDomainCredentialsA (
    _In_ PCREDENTIAL_TARGET_INFORMATIONA TargetInfo,
    _In_ DWORD Flags,
    _Out_ DWORD *Count,
    _Outptr_result_buffer_(*Count) PCREDENTIALA **Credential
    );

#ifdef UNICODE
#define CredReadDomainCredentials CredReadDomainCredentialsW
#else
#define CredReadDomainCredentials CredReadDomainCredentialsA
#endif // UNICODE


WINADVAPI
BOOL
WINAPI
CredDeleteW (
    _In_ LPCWSTR TargetName,
    _In_ DWORD Type,
    _Reserved_ DWORD Flags
    );

WINADVAPI
BOOL
WINAPI
CredDeleteA (
    _In_ LPCSTR TargetName,
    _In_ DWORD Type,
    _Reserved_ DWORD Flags
    );

#ifdef UNICODE
#define CredDelete CredDeleteW
#else
#define CredDelete CredDeleteA
#endif // UNICODE

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

WINADVAPI
BOOL
WINAPI
CredRenameW (
    _In_ LPCWSTR OldTargetName,
    _In_ LPCWSTR NewTargetName,
    _In_ DWORD Type,
    _Reserved_ DWORD Flags
    );

WINADVAPI
BOOL
WINAPI
CredRenameA (
    _In_ LPCSTR OldTargetName,
    _In_ LPCSTR NewTargetName,
    _In_ DWORD Type,
    _Reserved_ DWORD Flags
    );

#ifdef UNICODE
#define CredRename CredRenameW
#else
#define CredRename CredRenameA
#endif // UNICODE

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

//
// Values of flags to CredGetTargetInfo
//

#define CRED_ALLOW_NAME_RESOLUTION 0x1


WINADVAPI
BOOL
WINAPI
CredGetTargetInfoW (
    _In_ LPCWSTR TargetName,
    _In_ DWORD Flags,
    _Out_ PCREDENTIAL_TARGET_INFORMATIONW *TargetInfo
    );

WINADVAPI
BOOL
WINAPI
CredGetTargetInfoA (
    _In_ LPCSTR TargetName,
    _In_ DWORD Flags,
    _Out_ PCREDENTIAL_TARGET_INFORMATIONA *TargetInfo
    );

#ifdef UNICODE
#define CredGetTargetInfo CredGetTargetInfoW
#else
#define CredGetTargetInfo CredGetTargetInfoA
#endif // UNICODE

WINADVAPI
BOOL
WINAPI
CredMarshalCredentialW(
    _In_ CRED_MARSHAL_TYPE CredType,
    _In_ PVOID Credential,
    _Out_ LPWSTR *MarshaledCredential
    );

WINADVAPI
BOOL
WINAPI
CredMarshalCredentialA(
    _In_ CRED_MARSHAL_TYPE CredType,
    _In_ PVOID Credential,
    _Out_ LPSTR *MarshaledCredential
    );

#ifdef UNICODE
#define CredMarshalCredential CredMarshalCredentialW
#else
#define CredMarshalCredential CredMarshalCredentialA
#endif // UNICODE

WINADVAPI
BOOL
WINAPI
CredUnmarshalCredentialW(
    _In_ LPCWSTR MarshaledCredential,
    _Out_ PCRED_MARSHAL_TYPE CredType,
    _Out_ PVOID *Credential
    );

WINADVAPI
BOOL
WINAPI
CredUnmarshalCredentialA(
    _In_ LPCSTR MarshaledCredential,
    _Out_ PCRED_MARSHAL_TYPE CredType,
    _Out_ PVOID *Credential
    );

#ifdef UNICODE
#define CredUnmarshalCredential CredUnmarshalCredentialW
#else
#define CredUnmarshalCredential CredUnmarshalCredentialA
#endif // UNICODE

WINADVAPI
BOOL
WINAPI
CredIsMarshaledCredentialW(
    _In_ LPCWSTR MarshaledCredential
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

WINADVAPI
BOOL
WINAPI
CredIsMarshaledCredentialA(
    _In_ LPCSTR MarshaledCredential
    );

#ifdef UNICODE
#define CredIsMarshaledCredential CredIsMarshaledCredentialW
#else
#define CredIsMarshaledCredential CredIsMarshaledCredentialA
#endif // UNICODE

CREDUIAPI
BOOL
WINAPI
CredUnPackAuthenticationBufferW(
    _In_ DWORD                                      dwFlags,
    _In_reads_bytes_(cbAuthBuffer) PVOID                 pAuthBuffer,
    _In_ DWORD                                      cbAuthBuffer,
    _Out_writes_opt_(*pcchMaxUserName) LPWSTR       pszUserName,
    _Inout_ DWORD*                                  pcchMaxUserName,
    _Out_writes_opt_(*pcchMaxDomainName) LPWSTR     pszDomainName,
    _Inout_opt_ DWORD*                              pcchMaxDomainName,
    _Out_writes_opt_(*pcchMaxPassword) LPWSTR       pszPassword,
    _Inout_ DWORD*                                  pcchMaxPassword
    );

CREDUIAPI
BOOL
WINAPI
CredUnPackAuthenticationBufferA(
    _In_ DWORD                                      dwFlags,
    _In_reads_bytes_(cbAuthBuffer) PVOID                 pAuthBuffer,
    _In_ DWORD                                      cbAuthBuffer,
    _Out_writes_opt_(*pcchlMaxUserName) LPSTR       pszUserName,
    _Inout_ DWORD*                                  pcchlMaxUserName,
    _Out_writes_opt_(*pcchMaxDomainName) LPSTR      pszDomainName,
    _Inout_opt_ DWORD*                              pcchMaxDomainName,
    _Out_writes_opt_(*pcchMaxPassword) LPSTR        pszPassword,
    _Inout_ DWORD*                                  pcchMaxPassword
    );

#ifdef UNICODE
#define CredUnPackAuthenticationBuffer CredUnPackAuthenticationBufferW
#else
#define CredUnPackAuthenticationBuffer CredUnPackAuthenticationBufferA
#endif //UNICODE

CREDUIAPI
BOOL
WINAPI
CredPackAuthenticationBufferW(
    _In_ DWORD                                      dwFlags,
    _In_ LPWSTR                                     pszUserName,
    _In_ LPWSTR                                     pszPassword,
    _Out_writes_bytes_opt_(*pcbPackedCredentials) PBYTE   pPackedCredentials,
    _Inout_ DWORD*                                  pcbPackedCredentials
    );

CREDUIAPI
BOOL
WINAPI
CredPackAuthenticationBufferA(
    _In_ DWORD                                      dwFlags,
    _In_ LPSTR                                      pszUserName,
    _In_ LPSTR                                      pszPassword,
    _Out_writes_bytes_opt_(*pcbPackedCredentials) PBYTE   pPackedCredentials,
    _Inout_ DWORD*                                  pcbPackedCredentials
    );

#ifdef UNICODE
#define CredPackAuthenticationBuffer CredPackAuthenticationBufferW
#else
#define CredPackAuthenticationBuffer CredPackAuthenticationBufferA
#endif //UNICODE

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

_Success_(return)
WINADVAPI
BOOL
WINAPI
CredProtectW(
    _In_ BOOL                               fAsSelf,
    _In_reads_(cchCredentials) LPWSTR       pszCredentials,
    _In_ DWORD                              cchCredentials,
    _Out_writes_to_(*pcchMaxChars, *pcchMaxChars) LPWSTR pszProtectedCredentials,
    _Inout_ DWORD*                          pcchMaxChars,
    _Out_opt_ CRED_PROTECTION_TYPE*         ProtectionType
    );

#define CRED_PROTECT_AS_SELF        0x1
#define CRED_PROTECT_TO_SYSTEM      0x2
#define CRED_PROTECT_VALID_FLAGS    (CRED_PROTECT_AS_SELF | CRED_PROTECT_TO_SYSTEM)

_Success_(return)
WINADVAPI
BOOL
WINAPI
CredProtectEx(
    _In_ ULONG                              Flags,
    _In_reads_(cchCredentials) LPWSTR       pszCredentials,
    _In_ DWORD                              cchCredentials,
    _Out_writes_to_(*pcchMaxChars, *pcchMaxChars) LPWSTR pszProtectedCredentials,
    _Inout_ DWORD*                          pcchMaxChars,
    _Out_opt_ CRED_PROTECTION_TYPE*         ProtectionType
    );

WINADVAPI
BOOL
WINAPI
CredProtectA(
    _In_ BOOL                            fAsSelf,
    _In_reads_(cchCredentials) LPSTR     pszCredentials,
    _In_ DWORD                           cchCredentials,
    _Out_writes_(*pcchMaxChars) LPSTR    pszProtectedCredentials,
    _Inout_ DWORD*                       pcchMaxChars,
    _Out_opt_ CRED_PROTECTION_TYPE*      ProtectionType
    );

#ifdef UNICODE
#define CredProtect CredProtectW
#else
#define CredProtect CredProtectA
#endif //UNICODE

_Success_(return)
WINADVAPI
BOOL
WINAPI
CredUnprotectW(
    _In_ BOOL                                   fAsSelf,
    _In_reads_(cchProtectedCredentials) LPWSTR  pszProtectedCredentials,
    _In_ DWORD                                  cchProtectedCredentials,
    _Out_writes_to_opt_(*pcchMaxChars, *pcchMaxChars) LPWSTR pszCredentials,
    _Inout_ DWORD*                              pcchMaxChars
    );

#define CRED_UNPROTECT_AS_SELF          0x1
#define CRED_UNPROTECT_ALLOW_TO_SYSTEM  0x2
#define CRED_UNPROTECT_VALID_FLAGS      (CRED_UNPROTECT_AS_SELF | CRED_UNPROTECT_ALLOW_TO_SYSTEM)

_Success_(return)
WINADVAPI
BOOL
WINAPI
CredUnprotectEx(
    _In_ ULONG                                  Flags,
    _In_reads_(cchProtectedCredentials) LPWSTR  pszProtectedCredentials,
    _In_ DWORD                                  cchProtectedCredentials,
    _Out_writes_to_opt_(*pcchMaxChars, *pcchMaxChars) LPWSTR pszCredentials,
    _Inout_ DWORD*                              pcchMaxChars
    );

WINADVAPI
BOOL
WINAPI
CredUnprotectA(
    _In_ BOOL                                   fAsSelf,
    _In_reads_(cchProtectedCredentials) LPSTR   pszProtectedCredentials,
    _In_ DWORD                                  cchProtectedCredentials,
    _Out_writes_opt_(*pcchMaxChars) LPSTR       pszCredentials,
    _Inout_ DWORD*                              pcchMaxChars
    );

#ifdef UNICODE
#define CredUnprotect CredUnprotectW
#else
#define CredUnprotect CredUnprotectA
#endif //UNICODE

WINADVAPI
BOOL
WINAPI
CredIsProtectedW(
    _In_ LPWSTR                 pszProtectedCredentials,
    _Out_ CRED_PROTECTION_TYPE* pProtectionType
    );

WINADVAPI
BOOL
WINAPI
CredIsProtectedA(
    _In_ LPSTR                  pszProtectedCredentials,
    _Out_ CRED_PROTECTION_TYPE* pProtectionType
    );

#ifdef UNICODE
#define CredIsProtected CredIsProtectedW
#else
#define CredIsProtected CredIsProtectedA
#endif //UNICODE



WINADVAPI
BOOL
WINAPI
CredFindBestCredentialW (
    _In_  LPCWSTR       TargetName,
    _In_  DWORD         Type,
    _In_  DWORD         Flags,
    _Out_ PCREDENTIALW *Credential
    );

WINADVAPI
BOOL
WINAPI
CredFindBestCredentialA (
    _In_  LPCSTR        TargetName,
    _In_  DWORD         Type,
    _In_  DWORD         Flags,
    _Out_ PCREDENTIALA *Credential
    );

#ifdef UNICODE
#define CredFindBestCredential CredFindBestCredentialW
#else
#define CredFindBestCredential CredFindBestCredentialA
#endif // UNICODE




WINADVAPI
BOOL
WINAPI
CredGetSessionTypes (
    _In_ DWORD MaximumPersistCount,
    _Out_writes_(MaximumPersistCount) LPDWORD MaximumPersist
    );


WINADVAPI
VOID
WINAPI
CredFree (
    _In_ PVOID Buffer
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

CREDUIAPI
DWORD
WINAPI
CredUIPromptForCredentialsW(
    _In_opt_ PCREDUI_INFOW pUiInfo,
    _In_opt_ PCWSTR pszTargetName,
    _Reserved_ PCtxtHandle pContext,
    _In_ DWORD dwAuthError,
    _Inout_updates_(ulUserNameBufferSize) PWSTR pszUserName,
    _In_ ULONG ulUserNameBufferSize,
    _Inout_updates_(ulPasswordBufferSize) PWSTR pszPassword,
    _In_ ULONG ulPasswordBufferSize,
    _Inout_opt_ BOOL *save,
    _In_ DWORD dwFlags
    );

CREDUIAPI
DWORD
WINAPI
CredUIPromptForCredentialsA(
    _In_opt_ PCREDUI_INFOA pUiInfo,
    _In_opt_ PCSTR pszTargetName,
    _Reserved_ PCtxtHandle pContext,
    _In_ DWORD dwAuthError,
    _Inout_updates_(ulUserNameBufferSize) PSTR  pszUserName,
    _In_ ULONG ulUserNameBufferSize,
    _Inout_updates_(ulPasswordBufferSize) PSTR pszPassword,
    _In_ ULONG ulPasswordBufferSize,
    _Inout_opt_ BOOL *save,
    _In_ DWORD dwFlags
    );

#ifdef UNICODE
#define CredUIPromptForCredentials CredUIPromptForCredentialsW
#else
#define CredUIPromptForCredentials CredUIPromptForCredentialsA
#endif

CREDUIAPI
DWORD
WINAPI
CredUIPromptForWindowsCredentialsW(
    _In_opt_ PCREDUI_INFOW pUiInfo,
    _In_ DWORD dwAuthError,
    _Inout_ ULONG *pulAuthPackage,
    _In_reads_bytes_opt_(ulInAuthBufferSize) LPCVOID pvInAuthBuffer,
    _In_ ULONG ulInAuthBufferSize,
    _Outptr_result_bytebuffer_to_(*pulOutAuthBufferSize, *pulOutAuthBufferSize) LPVOID * ppvOutAuthBuffer,
    _Out_ ULONG * pulOutAuthBufferSize,
    _Inout_opt_ BOOL *pfSave,
    _In_ DWORD dwFlags
    );

CREDUIAPI
DWORD
WINAPI
CredUIPromptForWindowsCredentialsA(
    _In_opt_ PCREDUI_INFOA pUiInfo,
    _In_ DWORD dwAuthError,
    _Inout_ ULONG *pulAuthPackage,
    _In_reads_bytes_opt_(ulInAuthBufferSize) LPCVOID pvInAuthBuffer,
    _In_ ULONG ulInAuthBufferSize,
    _Outptr_result_bytebuffer_to_(*pulOutAuthBufferSize, *pulOutAuthBufferSize) LPVOID * ppvOutAuthBuffer,
    _Out_ ULONG * pulOutAuthBufferSize,
    _Inout_opt_ BOOL *pfSave,
    _In_ DWORD dwFlags
    );

#ifdef UNICODE
#define CredUIPromptForWindowsCredentials CredUIPromptForWindowsCredentialsW
#else
#define CredUIPromptForWindowsCredentials CredUIPromptForWindowsCredentialsA
#endif


CREDUIAPI
DWORD
WINAPI
CredUIParseUserNameW(
    _In_ PCWSTR UserName,
    _Out_writes_(userBufferSize) WCHAR *user,
    _In_ ULONG userBufferSize,
    _Out_writes_(domainBufferSize) WCHAR *domain,
    _In_ ULONG domainBufferSize
    );

CREDUIAPI
DWORD
WINAPI
CredUIParseUserNameA(
    _In_ PCSTR userName,
    _Out_writes_(userBufferSize) CHAR *user,
    _In_ ULONG userBufferSize,
    _Out_writes_(domainBufferSize) CHAR *domain,
    _In_ ULONG domainBufferSize
    );

#ifdef UNICODE
#define CredUIParseUserName CredUIParseUserNameW
#else
#define CredUIParseUserName CredUIParseUserNameA
#endif



CREDUIAPI
DWORD
WINAPI
CredUICmdLinePromptForCredentialsW(
    _In_opt_ PCWSTR pszTargetName,
    _Reserved_ PCtxtHandle pContext,
    _In_ DWORD dwAuthError,
    _Inout_updates_(ulUserBufferSize) PWSTR UserName,
    _In_ ULONG ulUserBufferSize,
    _Inout_updates_(ulPasswordBufferSize) PWSTR pszPassword,
    _In_ ULONG ulPasswordBufferSize,
    _Inout_opt_ PBOOL pfSave,
    _In_ DWORD dwFlags
    );

CREDUIAPI
DWORD
WINAPI
CredUICmdLinePromptForCredentialsA(
    _In_opt_ PCSTR pszTargetName,
    _Reserved_ PCtxtHandle pContext,
    _In_ DWORD dwAuthError,
    _Inout_updates_(ulUserBufferSize) PSTR UserName,
    _In_ ULONG ulUserBufferSize,
    _Inout_updates_(ulPasswordBufferSize) PSTR pszPassword,
    _In_ ULONG ulPasswordBufferSize,
    _Inout_opt_ PBOOL pfSave,
    _In_ DWORD dwFlags
    );

#ifdef UNICODE
#define CredUICmdLinePromptForCredentials CredUICmdLinePromptForCredentialsW
#else
#define CredUICmdLinePromptForCredentials CredUICmdLinePromptForCredentialsA
#endif

//
// Call this API with bConfirm set to TRUE to confirm that the credential (previously created
// via CredUIGetCredentials or CredUIPromptForCredentials worked, or with bConfirm set to FALSE
// to indicate it didn't

CREDUIAPI
DWORD
WINAPI
CredUIConfirmCredentialsW(
    _In_ PCWSTR pszTargetName,
    _In_ BOOL  bConfirm
    );

CREDUIAPI
DWORD
WINAPI
CredUIConfirmCredentialsA(
    _In_ PCSTR pszTargetName,
    _In_ BOOL  bConfirm
    );

#ifdef UNICODE
#define CredUIConfirmCredentials CredUIConfirmCredentialsW
#else
#define CredUIConfirmCredentials CredUIConfirmCredentialsA
#endif


CREDUIAPI
DWORD
WINAPI
CredUIStoreSSOCredW (
    _In_opt_ PCWSTR pszRealm,
    _In_ PCWSTR pszUsername,
    _In_ PCWSTR pszPassword,
    _In_ BOOL   bPersist
    );

CREDUIAPI
DWORD
WINAPI
CredUIReadSSOCredW (
    _In_opt_ PCWSTR pszRealm,
    _Outptr_ PWSTR* ppszUsername
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#ifdef __cplusplus
}
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#endif // _WINCRED_H_


