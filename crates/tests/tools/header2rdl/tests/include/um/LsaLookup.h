/*++

Copyright (c) Microsoft Corporation, 1992 -

Module Name:

    lsalookup.h

Abstract:

    LSA Policy Lookup API

--*/

#if _MSC_VER >= 1200
#pragma warning(push)
#pragma warning(disable:4820) /* padding added after data member */
#endif

#ifndef _LSALOOKUP_
#define _LSALOOKUP_

#include <winapifamily.h>

#pragma region Desktop Family or OneCore Family 
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#ifdef __cplusplus
extern "C" {
#endif

#ifdef _NTDEF_

typedef UNICODE_STRING LSA_UNICODE_STRING, *PLSA_UNICODE_STRING;
typedef STRING LSA_STRING, *PLSA_STRING;
typedef OBJECT_ATTRIBUTES LSA_OBJECT_ATTRIBUTES, *PLSA_OBJECT_ATTRIBUTES;

#else // _NTDEF_

typedef struct _LSA_UNICODE_STRING {
    USHORT Length;
    USHORT MaximumLength;
#ifdef MIDL_PASS
    [size_is(MaximumLength/2), length_is(Length/2)]
#endif // MIDL_PASS
    PWSTR  Buffer;
} LSA_UNICODE_STRING, *PLSA_UNICODE_STRING;

typedef struct _LSA_STRING {
    USHORT Length;
    USHORT MaximumLength;
    PCHAR Buffer;
} LSA_STRING, *PLSA_STRING;

typedef struct _LSA_OBJECT_ATTRIBUTES {
    ULONG Length;
    HANDLE RootDirectory;
    PLSA_UNICODE_STRING ObjectName;
    ULONG Attributes;
    PVOID SecurityDescriptor;        // Points to type SECURITY_DESCRIPTOR
    PVOID SecurityQualityOfService;  // Points to type SECURITY_QUALITY_OF_SERVICE
} LSA_OBJECT_ATTRIBUTES, *PLSA_OBJECT_ATTRIBUTES;

#endif // _NTDEF_

//
// The following data type is used to identify a domain
//

typedef struct _LSA_TRUST_INFORMATION {

    LSA_UNICODE_STRING Name;
    PSID Sid;

} LSA_TRUST_INFORMATION, *PLSA_TRUST_INFORMATION;

// where members have the following usage:
//
//     Name - The name of the domain.
//
//     Sid - A pointer to the Sid of the Domain
//

//
// The following data type is used in name and SID lookup services to
// describe the domains referenced in the lookup operation.
//

typedef struct _LSA_REFERENCED_DOMAIN_LIST {

    ULONG Entries;
    PLSA_TRUST_INFORMATION Domains;

} LSA_REFERENCED_DOMAIN_LIST, *PLSA_REFERENCED_DOMAIN_LIST;

// where members have the following usage:
//
//     Entries - Is a count of the number of domains described in the
//         Domains array.
//
//     Domains - Is a pointer to an array of Entries LSA_TRUST_INFORMATION data
//         structures.
//

//
// The following data type is used in name to SID lookup services to describe
// the domains referenced in the lookup operation.
//

#if (_WIN32_WINNT >= 0x0501)
typedef struct _LSA_TRANSLATED_SID2 {

    SID_NAME_USE Use;
    PSID         Sid;
    LONG         DomainIndex;
    ULONG        Flags;

} LSA_TRANSLATED_SID2, *PLSA_TRANSLATED_SID2;

// where members have the following usage:
//
//     Use - identifies the use of the SID.  If this value is SidUnknown or
//         SidInvalid, then the remainder of the record is not set and
//         should be ignored.
//
//     Sid - Contains the complete Sid of the tranlated SID
//
//     DomainIndex - Is the index of an entry in a related
//         LSA_REFERENCED_DOMAIN_LIST data structure describing the
//         domain in which the account was found.
//
//         If there is no corresponding reference domain for an entry, then
//         this field will contain a negative value.
//
#endif

//
// The following data type is used in SID to name lookup services to
// describe the domains referenced in the lookup operation.
//

typedef struct _LSA_TRANSLATED_NAME {

    SID_NAME_USE Use;
    LSA_UNICODE_STRING Name;
    LONG DomainIndex;

} LSA_TRANSLATED_NAME, *PLSA_TRANSLATED_NAME;

// where the members have the following usage:
//
//     Use - Identifies the use of the name.  If this value is SidUnknown
//         or SidInvalid, then the remainder of the record is not set and
//         should be ignored.  If this value is SidWellKnownGroup then the
//         Name field is invalid, but the DomainIndex field is not.
//
//     Name - Contains the isolated name of the translated SID.
//
//     DomainIndex - Is the index of an entry in a related
//         LSA_REFERENCED_DOMAIN_LIST data structure describing the domain
//         in which the account was found.
//
//         If there is no corresponding reference domain for an entry, then
//         this field will contain a negative value.
//

//
// The following structure specifies the account domain info
// (corresponds to the PolicyAccountDomainInformation information class).
//

typedef struct _POLICY_ACCOUNT_DOMAIN_INFO {

    LSA_UNICODE_STRING DomainName;
    PSID DomainSid;

} POLICY_ACCOUNT_DOMAIN_INFO, *PPOLICY_ACCOUNT_DOMAIN_INFO;

// where the members have the following usage:
//
//     DomainName - Is the name of the domain
//
//     DomainSid - Is the Sid of the domain
//

//
// The following structure corresponds to the PolicyDnsDomainInformation
// information class
//

typedef struct _POLICY_DNS_DOMAIN_INFO
{
    LSA_UNICODE_STRING Name;
    LSA_UNICODE_STRING DnsDomainName;
    LSA_UNICODE_STRING DnsForestName;
    GUID DomainGuid;
    PSID Sid;

} POLICY_DNS_DOMAIN_INFO, *PPOLICY_DNS_DOMAIN_INFO;

// where the members have the following usage:
//
//      Name - Is the name of the Domain
//
//      DnsDomainName - Is the DNS name of the domain
//
//      DnsForestName - Is the DNS forest name of the domain
//
//      DomainGuid - Is the GUID of the domain
//
//      Sid - Is the Sid of the domain


//
// Access types for the Lookup Policy object
//
// Choose values to correspond to the POLICY_* access types
//

#define LOOKUP_VIEW_LOCAL_INFORMATION       0x00000001
#define LOOKUP_TRANSLATE_NAMES              0x00000800

//
// The following data type defines the classes of Lookup Policy
// Domain Information that may be queried. The values are chosen
// to match corresponding POLICY_INFORMATION_CLASS values.
//

typedef enum _LSA_LOOKUP_DOMAIN_INFO_CLASS {

    AccountDomainInformation = 5,
    DnsDomainInformation     = 12

} LSA_LOOKUP_DOMAIN_INFO_CLASS, *PLSA_LOOKUP_DOMAIN_INFO_CLASS;

//
// Lookup handle
//

typedef PVOID LSA_LOOKUP_HANDLE, *PLSA_LOOKUP_HANDLE;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

NTSTATUS
LsaLookupOpenLocalPolicy(
    _In_     PLSA_OBJECT_ATTRIBUTES ObjectAttributes,
    _In_     ACCESS_MASK AccessMask,
    _Inout_  PLSA_LOOKUP_HANDLE PolicyHandle
    );

NTSTATUS
LsaLookupClose(
    _In_ LSA_LOOKUP_HANDLE ObjectHandle
    );

NTSTATUS
LsaLookupTranslateSids(
    _In_  LSA_LOOKUP_HANDLE PolicyHandle,
    _In_  ULONG Count,
    _In_  PSID *Sids,
    _Out_ PLSA_REFERENCED_DOMAIN_LIST *ReferencedDomains,
    _Out_ PLSA_TRANSLATED_NAME *Names
    );

#if (_WIN32_WINNT >= 0x0501)
NTSTATUS
LsaLookupTranslateNames(
    _In_  LSA_LOOKUP_HANDLE PolicyHandle,
    _In_  ULONG Flags,
    _In_  ULONG Count,
    _In_  PLSA_UNICODE_STRING Names,
    _Out_ PLSA_REFERENCED_DOMAIN_LIST *ReferencedDomains,
    _Out_ PLSA_TRANSLATED_SID2 *Sids
    );
#endif

NTSTATUS
LsaLookupGetDomainInfo(
    _In_  LSA_LOOKUP_HANDLE PolicyHandle,
    _In_  LSA_LOOKUP_DOMAIN_INFO_CLASS DomainInfoClass,
    _Out_ PVOID *DomainInfo
    );

NTSTATUS
LsaLookupFreeMemory(
    _In_ PVOID Buffer
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#pragma region Desktop or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#ifdef __cplusplus
}
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#endif // _LSALOOKUP_

#if _MSC_VER >= 1200
#pragma warning(pop)
#endif
