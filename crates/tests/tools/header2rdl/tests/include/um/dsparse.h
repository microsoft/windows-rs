/*++ BUILD Version: 0001    // Increment this if a change has global effects

Copyright (c) 1996-2000 Microsoft Corporation

Module Name:

    DSPARSE.h

Abstract:

    This file contains structures, function prototypes, and definitions
    for public NTDS APIs other than directory interfaces like LDAP.

Environment:

    User Mode - Win32

Notes:

--*/


#ifndef _DSPARSE_H_
#define _DSPARSE_H_

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Familyy or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)


#include <schedule.h>

#define DSPARSE

#ifdef __cplusplus
extern "C" {
#endif

typedef enum _DS_MANGLE_FOR {
        DS_MANGLE_UNKNOWN = 0,
        DS_MANGLE_OBJECT_RDN_FOR_DELETION,
        DS_MANGLE_OBJECT_RDN_FOR_NAME_CONFLICT,
        } DS_MANGLE_FOR;

//////////////////////////////////////////////////////////////////////////
//                                                                      //
// Prototypes                                                           //
//                                                                      //
//////////////////////////////////////////////////////////////////////////

// ==========================================================
// DSMakeSpn -- client call to create SPN for a service to which it wants to
// authenticate.
// This name is then passed to "pszTargetName" of InitializeSecurityContext().
//
// Notes:
// If the service name is a DNS host name, or canonical DNS service name
// e.g. "www.ms.com", i.e., caller resolved with gethostbyname, then instance
// name should be NULL.
// Realm is host name minus first component, unless it is in the exception list
//
// If the service name is NetBIOS machine name, then instance name should be
// NULL
// Form must be <domain>\<machine>
// Realm will be <domain>
//
// If the service name is that of a replicated service, where each replica has
// its own account (e.g., with SRV records) then the caller must supply the
// instance name then realm name is same as ServiceName
//
// If the service name is a DN, then must also supply instance name
// (DN could be name of service object (incl RPC or Winsock), name of machine
// account, name of domain object)
// then realm name is domain part of the DN
//
// If the service name is NetBIOS domain name, then must also supply instance
// name; realm name is domain name
//
// If the service is named by an IP address -- then use referring service name
// as service name
//
//  ServiceClass - e.g. "http", "ftp", "ldap", GUID
//  ServiceName - DNS or DN; assumes we can compute domain from service name
//  InstanceName OPTIONAL- DNS name of host for instance of service
//  InstancePort - port number for instance (0 if default)
//  Referrer OPTIONAL- DNS name of host that gave this referral
//  pcSpnLength - in -- max length IN CHARACTERS of principal name;
//                out -- actual
//                Length includes terminator
//  pszSPN - server principal name
//
// If buffer is not large enough, ERROR_BUFFER_OVERFLOW is returned and the
// needed length is returned in pcSpnLength.
//
//

_Check_return_
_Success_(return == ERROR_SUCCESS)
DSPARSE
DWORD
WINAPI
DsMakeSpnW(
    _In_ LPCWSTR ServiceClass,
    _In_ LPCWSTR ServiceName,
    _In_opt_ LPCWSTR InstanceName,
    _In_ USHORT InstancePort,
    _In_opt_ LPCWSTR Referrer,
    _Inout_ DWORD *pcSpnLength,
    _Out_writes_to_opt_(*pcSpnLength, *pcSpnLength) LPWSTR pszSpn
);

_Check_return_
_Success_(return == ERROR_SUCCESS)
DSPARSE
DWORD
WINAPI
DsMakeSpnA(
    _In_ LPCSTR ServiceClass,
    _In_ LPCSTR ServiceName,
    _In_opt_ LPCSTR InstanceName,
    _In_ USHORT InstancePort,
    _In_opt_ LPCSTR Referrer,
    _Inout_ DWORD *pcSpnLength,
    _Out_writes_to_opt_ (*pcSpnLength, *pcSpnLength) LPSTR pszSpn
);

#ifdef UNICODE
#define DsMakeSpn DsMakeSpnW
#else
#define DsMakeSpn DsMakeSpnA
#endif

// ==========================================================
// DsCrackSpn() -- parse an SPN into the ServiceClass,
// ServiceName, and InstanceName (and InstancePort) pieces.
// An SPN is passed in, along with a pointer to the maximum length
// for each piece and a pointer to a buffer where each piece should go.
// On exit, the maximum lengths are updated to the actual length for each piece
// and the buffer contain the appropriate piece. The InstancePort is 0 if not
// present.
//
// DWORD DsCrackSpn(
//      IN LPTSTR pszSPN,               // the SPN to parse
//      IN OUT PUSHORT pcServiceClass,  // input -- max length of ServiceClass;
//                                         output -- actual length
//      OUT LPCTSTR ServiceClass,       // the ServiceClass part of the SPN
//      IN OUT PUSHORT pcServiceName,   // input -- max length of ServiceName;
//                                         output -- actual length
//      OUT LPCTSTR ServiceName,        // the ServiceName part of the SPN
//      IN OUT PUSHORT pcInstance,      // input -- max length of ServiceClass;
//                                         output -- actual length
//      OUT LPCTSTR InstanceName,  // the InstanceName part of the SPN
//      OUT PUSHORT InstancePort          // instance port
//
// Note: lengths are in characters; all string lengths include terminators
// All arguments except pszSpn are optional.
//

_Check_return_
DSPARSE
DWORD
WINAPI
DsCrackSpnA(
    _In_ LPCSTR pszSpn,
    _Inout_opt_ LPDWORD pcServiceClass,
    _Out_writes_to_opt_ (*pcServiceClass, *pcServiceClass) LPSTR ServiceClass,
    _Inout_opt_ LPDWORD pcServiceName,
    _Out_writes_to_opt_ (*pcServiceName, *pcServiceName) LPSTR ServiceName,
    _Inout_opt_ LPDWORD pcInstanceName,
    _Out_writes_to_opt_ (*pcInstanceName, *pcInstanceName) LPSTR InstanceName,
    _Out_opt_ USHORT *pInstancePort
    );

_Check_return_
DSPARSE
DWORD
WINAPI
DsCrackSpnW(
    _In_ LPCWSTR pszSpn,
    _Inout_opt_ DWORD *pcServiceClass,
    _Out_writes_to_opt_ (*pcServiceClass, *pcServiceClass) LPWSTR ServiceClass,
    _Inout_opt_ DWORD *pcServiceName,
    _Out_writes_to_opt_ (*pcServiceName, *pcServiceName) LPWSTR ServiceName,
    _Inout_opt_ DWORD *pcInstanceName,
    _Out_writes_to_opt_ (*pcInstanceName, *pcInstanceName) LPWSTR InstanceName,
    _Out_opt_ USHORT *pInstancePort
    );

#ifdef UNICODE
#define DsCrackSpn DsCrackSpnW
#else
#define DsCrackSpn DsCrackSpnA
#endif

#ifndef MIDL_PASS
/*++
==========================================================
_Check_return_
DSPARSE
DWORD
WINAPI
DsQuoteRdnValue(
    DWORD    cUnquotedRdnValueLength,
    _In_reads_(cUnquotedRdnValueLength)   LPCTCH   psUnquotedRdnValue,
    _Inout_ DWORD    *pcQuotedRdnValueLength,
    _Out_writes_to_(*pcQuotedRdnValueLength, *pcQuotedRdnValueLength)    LPTCH    psQuotedRdnValue
    )
/*++

Description

    This client call converts an RDN value into a quoted RDN value if
    the RDN value contains characters that require quotes. The resultant
    RDN can be submitted as part of a DN to the DS using various APIs
    such as LDAP.

    No quotes are added if none are needed. In this case, the
    output RDN value will be the same as the input RDN value.

    The RDN is quoted in accordance with the specification "Lightweight
    Directory Access Protocol (v3): UTF-8 String Representation of
    Distinguished Names", RFC 2253.

    The input and output RDN values are *NOT* NULL terminated.

    The changes made by this call can be undone by calling
    DsUnquoteRdnValue().

Arguments:

    cUnquotedRdnValueLength - The length of psUnquotedRdnValue in chars.

    psUnquotedRdnValue - Unquoted RDN value.

    pcQuotedRdnValueeLength - IN, maximum length of psQuotedRdnValue, in chars
                        OUT ERROR_SUCCESS, chars utilized in psQuotedRdnValue
                        OUT ERROR_BUFFER_OVERFLOW, chars needed in psQuotedRdnValue

    psQuotedRdnValue - The resultant and perhaps quoted RDN value

Return Value:
    ERROR_SUCCESS
        If quotes or escapes were needed, then psQuotedRdnValue contains
        the quoted, escaped version of psUnquotedRdnValue. Otherwise,
        psQuotedRdnValue contains a copy of psUnquotedRdnValue. In either
        case, pcQuotedRdnValueLength contains the space utilized, in chars.

    ERROR_BUFFER_OVERFLOW
        psQuotedRdnValueLength contains the space needed, in chars,
        to hold psQuotedRdnValue.

    ERROR_INVALID_PARAMETER
        Invalid parameter.

    ERROR_NOT_ENOUGH_MEMORY
        Allocation error.

--*/

_Check_return_
_Success_(return == 0)
DSPARSE
DWORD
WINAPI
DsQuoteRdnValueW(
    DWORD    cUnquotedRdnValueLength,
    _In_reads_(cUnquotedRdnValueLength) IN     LPCWCH   psUnquotedRdnValue,
    _Inout_ DWORD    *pcQuotedRdnValueLength,
    _Out_writes_to_(*pcQuotedRdnValueLength, *pcQuotedRdnValueLength) LPWCH    psQuotedRdnValue
);

_Check_return_
_Success_(return == 0)
DSPARSE
DWORD
WINAPI
DsQuoteRdnValueA(
    DWORD    cUnquotedRdnValueLength,
    _In_reads_ (cUnquotedRdnValueLength) IN     LPCCH    psUnquotedRdnValue,
    _Inout_ DWORD    *pcQuotedRdnValueLength,
    _Out_writes_to_ (*pcQuotedRdnValueLength, *pcQuotedRdnValueLength) LPCH     psQuotedRdnValue
);

#ifdef UNICODE
#define DsQuoteRdnValue DsQuoteRdnValueW
#else
#define DsQuoteRdnValue DsQuoteRdnValueA
#endif

/*++
==========================================================
_Check_return_
DSPARSE
DWORD
WINAPI
DsUnquoteRdnValue(
    DWORD    cQuotedRdnValueLength,
    _In_reads_(cQuotedRdnValueLength)     LPCTCH   psQuotedRdnValue,
    _Inout_ DWORD    *pcUnquotedRdnValueLength,
    _Out_writes_to_(*pcUnquotedRdnValueLength,*pcUnquotedRdnValueLength)    LPTCH    psUnquotedRdnValue
    )

Description

    This client call converts a quoted RDN Value into an unquoted RDN
    Value. The resultant RDN value should *NOT* be submitted as part
    of a DN to the DS using various APIs such as LDAP.

    When psQuotedRdnValue is quoted:
        The leading and trailing quote are removed.

        Whitespace before the first quote is discarded.

        Whitespace trailing the last quote is discarded.

        Escapes are removed and the char following the escape is kept.

    The following actions are taken when psQuotedRdnValue is unquoted:

        Leading whitespace is discarded.

        Trailing whitespace is kept.

        Escaped non-special chars return an error.

        Unescaped special chars return an error.

        RDN values beginning with # (ignoring leading whitespace) are
        treated as a stringized BER value and converted accordingly.

        Escaped hex digits (\89) are converted into a binary byte (0x89).

        Escapes are removed from escaped special chars.

    The following actions are always taken:
        Escaped special chars are unescaped.

    The input and output RDN values are not NULL terminated.

Arguments:

    cQuotedRdnValueLength - The length of psQuotedRdnValue in chars.

    psQuotedRdnValue - RDN value that may be quoted and may be escaped.

    pcUnquotedRdnValueLength - IN, maximum length of psUnquotedRdnValue, in chars
                          OUT ERROR_SUCCESS, chars used in psUnquotedRdnValue
                          OUT ERROR_BUFFER_OVERFLOW, chars needed for psUnquotedRdnValue

    psUnquotedRdnValue - The resultant unquoted RDN value.

Return Value:
    ERROR_SUCCESS
        psUnquotedRdnValue contains the unquoted and unescaped version
        of psQuotedRdnValue. pcUnquotedRdnValueLength contains the space
        used, in chars.

    ERROR_BUFFER_OVERFLOW
        psUnquotedRdnValueLength contains the space needed, in chars,
        to hold psUnquotedRdnValue.

    ERROR_INVALID_PARAMETER
        Invalid parameter.

    ERROR_NOT_ENOUGH_MEMORY
        Allocation error.

--*/

_Check_return_
_Success_(return == 0)
DSPARSE
DWORD
WINAPI
DsUnquoteRdnValueW(
    DWORD    cQuotedRdnValueLength,
    _In_reads_ (cQuotedRdnValueLength) LPCWCH   psQuotedRdnValue,
    _Inout_ DWORD    *pcUnquotedRdnValueLength,
    _Out_writes_to_ (*pcUnquotedRdnValueLength, *pcUnquotedRdnValueLength) LPWCH    psUnquotedRdnValue
);

_Check_return_
_Success_(return == 0)
DSPARSE
DWORD
WINAPI
DsUnquoteRdnValueA(
    DWORD    cQuotedRdnValueLength,
    _In_reads_ (cQuotedRdnValueLength) LPCCH    psQuotedRdnValue,
    _Inout_ DWORD    *pcUnquotedRdnValueLength,
    _Out_writes_to_ (*pcUnquotedRdnValueLength, *pcUnquotedRdnValueLength) LPCH     psUnquotedRdnValue
);

#ifdef UNICODE
#define DsUnquoteRdnValue DsUnquoteRdnValueW
#else
#define DsUnquoteRdnValue DsUnquoteRdnValueA
#endif

/*++
==========================================================
_Check_return_
DSPARSE
DWORD
WINAPI
DsGetRdnW(
    IN OUT LPCWCH   *ppDN,
    IN OUT DWORD    *pcDN,
    OUT    LPCWCH   *ppKey,
    OUT    DWORD    *pcKey,
    OUT    LPCWCH   *ppVal,
    OUT    DWORD    *pcVal
    )

Description

    This client call accepts a DN with quoted RDNs and returns the address
    and length, in chars, of the key and value for the first RDN in the DN.
    The RDN value returned is still quoted. Use DsUnquoteRdnValue to unquote
    the value for display.

    This client call also returns the address and length of the rest of the
    DN. A subsequent call using the returned DN address and length will
    return information about the next RDN.

    The following loop processes each RDN in pDN:
        ccDN = wcslen(pDN)
        while (ccDN) {
            error = DsGetRdn(&pDN,
                             &ccDN,
                             &pKey,
                             &ccKey,
                             &pVal,
                             &ccVal);
            if (error != ERROR_SUCCESS) {
                process error;
                return;
            }
            if (ccKey) {
                process pKey;
            }
            if (ccVal) {
                process pVal;
            }
        }

    For example, given the DN "cn=bob,dc=com", the first call to DsGetRdnW
    returns the addresses for ",dc=com", "cn", and "bob" with respective
    lengths of 7, 2, and 3. A subsequent call with ",dc=com" returns "",
    "dc", and "com" with respective lengths 0, 2, and 3.

Arguments:
    ppDN
        IN : *ppDN points to a DN
        OUT: *ppDN points to the rest of the DN following the first RDN
    pcDN
        IN : *pcDN is the count of chars in the input *ppDN, not including
             any terminating NULL
        OUT: *pcDN is the count of chars in the output *ppDN, not including
             any terminating NULL
    ppKey
        OUT: Undefined if *pcKey is 0. Otherwise, *ppKey points to the first
             key in the DN
    pcKey
        OUT: *pcKey is the count of chars in *ppKey.

    ppVal
        OUT: Undefined if *pcVal is 0. Otherwise, *ppVal points to the first
             value in the DN
    pcVal
        OUT: *pcVal is the count of chars in *ppVal

Return Value:
    ERROR_SUCCESS
        If *pccDN is not 0, then *ppDN points to the rest of the DN following
        the first RDN. If *pccDN is 0, then *ppDN is undefined.

        If *pccKey is not 0, then *ppKey points to the first key in DN. If
        *pccKey is 0, then *ppKey is undefined.

        If *pccVal is not 0, then *ppVal points to the first value in DN. If
        *pccVal is 0, then *ppVal is undefined.

    ERROR_DS_NAME_UNPARSEABLE
        The first RDN in *ppDN could not be parsed. All output parameters
        are undefined.

    Any other error
        All output parameters are undefined.

--*/
_Check_return_
_Success_(return == 0)
DSPARSE
DWORD
WINAPI
DsGetRdnW(
    _Inout_ _At_(*ppDN, _Pre_readable_size_(*pcDN) _Post_readable_size_(*pcDN)) LPCWCH   *ppDN,
    _Inout_ DWORD    *pcDN,
    _Outptr_result_buffer_(*pcKey)    LPCWCH   *ppKey,
    _Out_    DWORD    *pcKey,
    _Outptr_result_buffer_(*pcVal)    LPCWCH   *ppVal,
    _Out_    DWORD    *pcVal
    );


/*++
==========================================================

_Check_return_
DSPARSE
BOOL
WINAPI
DsCrackUnquotedMangledRdnW(
     IN LPCWSTR pszRDN,
     IN DWORD cchRDN,
     OUT OPTIONAL GUID *pGuid,
     OUT OPTIONAL DS_MANGLE_FOR *peDsMangleFor
     );

Description

Determine whether the given RDN is in mangled form. If so, the mangled RDN
is decoded, and the guid and mangle type are returned.

The RDN should already be in unquoted form. See DsUnquoteRdnValue.

Arguments:

    pszRDN (IN) - Character string containing RDN. Termination is optional.

    cchRDN (IN) - Length of RDN excluding termination, if any

    pGuid (OUT, OPTIONAL) - Pointer to storage to receive decoded guid.
                            Only returned if RDN is mangled.

    peDsMangleFor (OUT, OPTIONAL) - Pointer to storage to receive mangle type.
                            Only returned if RDN is mangled

Return Value:

    BOOL - Whether the RDN is mangled or not

--*/

DSPARSE
BOOL
WINAPI
DsCrackUnquotedMangledRdnW(
     _In_reads_(cchRDN) LPCWSTR pszRDN,
     DWORD cchRDN,
     _Out_opt_ GUID *pGuid,
     _Out_opt_ DS_MANGLE_FOR *peDsMangleFor
     );

DSPARSE
BOOL
WINAPI
DsCrackUnquotedMangledRdnA(
     _In_reads_(cchRDN) LPCSTR pszRDN,
     DWORD cchRDN,
     _Out_opt_ GUID *pGuid,
     _Out_opt_ DS_MANGLE_FOR *peDsMangleFor
     );

#ifdef UNICODE
#define DsCrackUnquotedMangledRdn DsCrackUnquotedMangledRdnW
#else
#define DsCrackUnquotedMangledRdn DsCrackUnquotedMangledRdnA
#endif

/*++
==========================================================

_Check_return_
DSPARSE
BOOL
WINAPI
DsIsMangledRdnValueW(
    LPCWSTR pszRdn,
    DWORD cRdn,
    DS_MANGLE_FOR eDsMangleForDesired
    );

Description

    Determine if the given RDN Value is mangled, and of the given type. Note that
    the key portion of an RDN should not be supplied.

    The name may be quoted or unquoted.  This routine tries to unquote the value.  If
    the unquote operation fails, the routine proceeds to attempt the unmangle.

    A change was made in the default quoting behavior of DNs returned from the DS
    between Windows 2000 and Windows XP. This routine transparently handles RDNs with
    special characters in either form.

    The routine expects the value part of the RDN.

    If you have full DN, use DsIsMangledDn() below.

    To check for deleted name:
        DsIsMangledRdnValueW( rdn, rdnlen, DS_MANGLE_OBJECT_FOR_DELETION )
    To check for a conflicted name:
        DsIsMangledRdnValueW( rdn, rdnlen, DS_MANGLE_OBJECT_FOR_NAME_CONFLICT )

Arguments:

    pszRdn (IN) - RDN value character string. Termination is not required and
        is ignored.

    cRdn (IN) - Length of RDN value in characters excluding termination

    eDsMangleForDesired (IN) - Type of mangling to check for

Return Value:

    BOOL - True if the Rdn is mangled and is of the required type

--*/

DSPARSE
BOOL
WINAPI
DsIsMangledRdnValueW(
    _In_reads_(cRdn) LPCWSTR pszRdn,
    DWORD cRdn,
    DS_MANGLE_FOR eDsMangleForDesired
    );

DSPARSE
BOOL
WINAPI
DsIsMangledRdnValueA(
    _In_reads_(cRdn) LPCSTR pszRdn,
    DWORD cRdn,
    DS_MANGLE_FOR eDsMangleForDesired
    );

#ifdef UNICODE
#define DsIsMangledRdnValue DsIsMangledRdnValueW
#else
#define DsIsMangledRdnValue DsIsMangledRdnValueA
#endif

/*++
==========================================================

DSPARSE
BOOL
WINAPI
DsIsMangledDnW(
    LPCWSTR pszDn,
    DS_MANGLE_FOR eDsMangleFor
    );

Description

    Determine if the first RDN in a quoted DN is a mangled name of given type.

    The DN must be suitable for input to DsGetRdn().

    To check for deleted name:
        DsIsMangledDnW( dn, DS_MANGLE_OBJECT_FOR_DELETION )
    To check for a conflicted name:
        DsIsMangledDnW( Dn, DS_MANGLE_OBJECT_FOR_NAME_CONFLICT )

Arguments:

    pszDn (IN) - Quoted Distinguished Name as returned by DS functions

    eDsMangleFor (IN) - Type of mangling to check for

Return Value:

    BOOL - True if first RDN is mangled and is of the given mangle type

--*/

DSPARSE
BOOL
WINAPI
DsIsMangledDnA(
    _In_ LPCSTR pszDn,
    DS_MANGLE_FOR eDsMangleFor
    );

DSPARSE
BOOL
WINAPI
DsIsMangledDnW(
    _In_ LPCWSTR pszDn,
    DS_MANGLE_FOR eDsMangleFor
    );

#ifdef UNICODE
#define DsIsMangledDn DsIsMangledDnW
#else
#define DsIsMangledDn DsIsMangledDnA
#endif

// does there need to be a dsparsep.w?

// ==========================================================
// DsCrackSpn2() -- parse a counted-length SPN into the ServiceClass,
// ServiceName, and InstanceName (and InstancePort) pieces.
// An SPN is passed in, along with a pointer to the maximum length
// for each piece and a pointer to a buffer where each piece should go.
// On exit, the maximum lengths are updated to the actual length for each piece
// and the buffer contain the appropriate piece. The InstancePort is 0 if not
// present.
//
// DWORD DsCrackSpn2(
//      IN LPTSTR pszSPN,           // the SPN to parse (does not have to be NULL-terminated)
//      IN DWORD cSpn,            // length of pszSPN
//      IN OUT PUSHORT pcServiceClass OPTIONAL,
//       input -- max length of ServiceClass;
//       output -- actual length
//      OUT LPCTSTR ServiceClass OPTIONAL, // the ServiceClass part of the SPN
//      IN OUT PUSHORT pcServiceName OPTIONAL,
//       input -- max length of ServiceName;
//       output -- actual length
//      OUT LPCTSTR ServiceName OPTIONAL,  // the ServiceName part of the SPN
//      IN OUT PUSHORT pcInstance OPTIONAL,
//       input -- max length of ServiceClass;
//       output -- actual length
//      OUT LPCTSTR InstanceName OPTIONAL,  // the InstanceName part of the SPN
//      OUT PUSHORT InstancePort OPTIONAL    // instance port
//
// Note: lengths are in characters; all string lengths include terminators
// All arguments except pszSpn are optional.
//

DSPARSE
DWORD
WINAPI
DsCrackSpn2A(
    _In_reads_ (cSpn) LPCSTR pszSpn,
    _In_ DWORD cSpn,
    _Inout_opt_ LPDWORD pcServiceClass,
    _Out_writes_to_opt_ (*pcServiceClass, *pcServiceClass) LPSTR ServiceClass,
    _Inout_opt_ LPDWORD pcServiceName,
    _Out_writes_to_opt_ (*pcServiceName, *pcServiceName) LPSTR ServiceName,
    _Inout_opt_ LPDWORD pcInstanceName,
    _Out_writes_to_opt_ (*pcInstanceName, *pcInstanceName) LPSTR InstanceName,
    _Out_opt_ USHORT *pInstancePort
    );

DSPARSE
DWORD
WINAPI
DsCrackSpn2W(
    _In_reads_ (cSpn) LPCWSTR pszSpn,
    _In_ DWORD cSpn,
    _Inout_opt_ DWORD *pcServiceClass,
    _Out_writes_to_opt_ (*pcServiceClass, *pcServiceClass) LPWSTR ServiceClass,
    _Inout_opt_ DWORD *pcServiceName,
    _Out_writes_to_opt_ (*pcServiceName, *pcServiceName) LPWSTR ServiceName,
    _Inout_opt_ DWORD *pcInstanceName,
    _Out_writes_to_opt_ (*pcInstanceName, *pcInstanceName) LPWSTR InstanceName,
    _Out_opt_ USHORT *pInstancePort
    );

DSPARSE
DWORD
WINAPI
DsCrackSpn3W(
    _In_ LPCWSTR pszSpn,
    IN DWORD cSpn,
    IN OUT DWORD *pcHostName,
    _Out_writes_to_ (*pcHostName, *pcHostName) LPWSTR HostName,
    IN OUT DWORD *pcInstanceName,
    _Out_writes_to_ (*pcInstanceName, *pcInstanceName) LPWSTR InstanceName,
    OUT USHORT *pPortNumber,
    IN OUT DWORD *pcDomainName,
    _Out_writes_to_ (*pcDomainName, *pcDomainName) LPWSTR DomainName,
    IN OUT DWORD *pcRealmName,
    _Out_writes_to_ (*pcRealmName, *pcRealmName) LPWSTR RealmName
    );

DSPARSE
DWORD
WINAPI
DsCrackSpn4W(
    _In_ LPCWSTR pszSpn,
    IN DWORD cSpn,
    IN OUT DWORD *pcHostName,
    _Out_writes_to_ (*pcHostName, *pcHostName) LPWSTR HostName,
    IN OUT DWORD *pcInstanceName,
    _Out_writes_to_ (*pcInstanceName, *pcInstanceName) LPWSTR InstanceName,
    IN OUT DWORD *pcPortName,
    _Out_writes_to_ (*pcPortName, *pcPortName) LPWSTR PortName,
    IN OUT DWORD *pcDomainName,
    _Out_writes_to_ (*pcDomainName, *pcDomainName) LPWSTR DomainName,
    IN OUT DWORD *pcRealmName,
    _Out_writes_to_ (*pcRealmName, *pcRealmName) LPWSTR RealmName
    );

#ifdef UNICODE
#define DsCrackSpn2 DsCrackSpn2W
#else
#define DsCrackSpn2 DsCrackSpn2A
#endif

#ifdef __cplusplus
}
#endif
#endif // !MIDL_PASS


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#endif // _DSPARSE_H_
