/*++

Copyright (2024) Microsoft. All rights reserved.

Module Name:

    windnsdef.h

Abstract:

    Domain Name System (DNS) definitions.

Revision History:

--*/


#ifndef _WINDNSDEF_INCLUDED_
#define _WINDNSDEF_INCLUDED_

#if _MSC_VER > 1000
#pragma once
#endif

#include <winapifamily.h>

#pragma region Desktop Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

#ifdef __cplusplus
extern "C" {
#endif


#ifndef QWORD
typedef unsigned __int64 QWORD, *PQWORD;
#endif

//
//  IP Address
//

typedef DWORD   IP4_ADDRESS, *PIP4_ADDRESS;

#define SIZEOF_IP4_ADDRESS                  (4)
#define IP4_ADDRESS_STRING_LENGTH           (16)

//  Backcompat only -- length def contains terminating NULL

#define IP4_ADDRESS_STRING_BUFFER_LENGTH    (16)

//
//  IPv6 Address
//

#ifdef MIDL_PASS

#ifdef _WIN64
#pragma pack(push, 8)
#else
#pragma pack(push, 4)
#endif

typedef struct
{
    QWORD       IP6Qword[2];
}
IP6_ADDRESS, *PIP6_ADDRESS;

#pragma pack(pop)

#else // MIDL_PASS

typedef union
{
#ifdef _WIN64
    QWORD       IP6Qword[2];
#endif
    DWORD       IP6Dword[4];
    WORD        IP6Word[8];
    BYTE        IP6Byte[16];
#ifdef  IN6_ADDR
    IN6_ADDR    In6;
#endif
}
IP6_ADDRESS, *PIP6_ADDRESS;

#endif // MIDL_PASS

#define DNS_ADDR_MAX_SOCKADDR_LENGTH         (32)

//
//  DNS Message Header
//

typedef struct _DNS_HEADER_EXT
{
    WORD            Reserved : 15;
    WORD            DnssecOk : 1;
    BYTE            chRcode;
    BYTE            chVersion;
}
DNS_HEADER_EXT, *PDNS_HEADER_EXT;

//
//  DNS Record Types
//
//  _TYPE_ defines are in host byte order.
//  _RTYPE_ defines are in net byte order.
//
//  Generally always deal with types in host byte order as we index
//  resource record functions by type.
//

#define DNS_TYPE_ZERO       0x0000

//  RFC 1034/1035
#define DNS_TYPE_A          0x0001      //  1
#define DNS_TYPE_NS         0x0002      //  2
#define DNS_TYPE_MD         0x0003      //  3
#define DNS_TYPE_MF         0x0004      //  4
#define DNS_TYPE_CNAME      0x0005      //  5
#define DNS_TYPE_SOA        0x0006      //  6
#define DNS_TYPE_MB         0x0007      //  7
#define DNS_TYPE_MG         0x0008      //  8
#define DNS_TYPE_MR         0x0009      //  9
#define DNS_TYPE_NULL       0x000a      //  10
#define DNS_TYPE_WKS        0x000b      //  11
#define DNS_TYPE_PTR        0x000c      //  12
#define DNS_TYPE_HINFO      0x000d      //  13
#define DNS_TYPE_MINFO      0x000e      //  14
#define DNS_TYPE_MX         0x000f      //  15
#define DNS_TYPE_TEXT       0x0010      //  16

//  RFC 1183
#define DNS_TYPE_RP         0x0011      //  17
#define DNS_TYPE_AFSDB      0x0012      //  18
#define DNS_TYPE_X25        0x0013      //  19
#define DNS_TYPE_ISDN       0x0014      //  20
#define DNS_TYPE_RT         0x0015      //  21

//  RFC 1348
#define DNS_TYPE_NSAP       0x0016      //  22
#define DNS_TYPE_NSAPPTR    0x0017      //  23

//  RFC 2065    (DNS security)
#define DNS_TYPE_SIG        0x0018      //  24
#define DNS_TYPE_KEY        0x0019      //  25

//  RFC 1664    (X.400 mail)
#define DNS_TYPE_PX         0x001a      //  26

//  RFC 1712    (Geographic position)
#define DNS_TYPE_GPOS       0x001b      //  27

//  RFC 1886    (IPv6 Address)
#define DNS_TYPE_AAAA       0x001c      //  28

//  RFC 1876    (Geographic location)
#define DNS_TYPE_LOC        0x001d      //  29

//  RFC 2065    (Secure negative response)
#define DNS_TYPE_NXT        0x001e      //  30

//  Patton      (Endpoint Identifier)
#define DNS_TYPE_EID        0x001f      //  31

//  Patton      (Nimrod Locator)
#define DNS_TYPE_NIMLOC     0x0020      //  32

//  RFC 2052    (Service location)
#define DNS_TYPE_SRV        0x0021      //  33

//  ATM Standard something-or-another (ATM Address)
#define DNS_TYPE_ATMA       0x0022      //  34

//  RFC 2168    (Naming Authority Pointer)
#define DNS_TYPE_NAPTR      0x0023      //  35

//  RFC 2230    (Key Exchanger)
#define DNS_TYPE_KX         0x0024      //  36

//  RFC 2538    (CERT)
#define DNS_TYPE_CERT       0x0025      //  37

//  A6 Draft    (A6)
#define DNS_TYPE_A6         0x0026      //  38

//  DNAME Draft (DNAME)
#define DNS_TYPE_DNAME      0x0027      //  39

//  Eastlake    (Kitchen Sink)
#define DNS_TYPE_SINK       0x0028      //  40

//  RFC 2671    (EDNS OPT)
#define DNS_TYPE_OPT        0x0029      //  41

//  RFC 4034    (DNSSEC DS)
#define DNS_TYPE_DS         0x002b      //  43

//  RFC 4034    (DNSSEC RRSIG)
#define DNS_TYPE_RRSIG      0x002e      //  46

//  RFC 4034    (DNSSEC NSEC)
#define DNS_TYPE_NSEC       0x002f      //  47

//  RFC 4034    (DNSSEC DNSKEY)
#define DNS_TYPE_DNSKEY     0x0030      //  48

//  RFC 4701    (DHCID)
#define DNS_TYPE_DHCID      0x0031      //  49

//  RFC 5155    (DNSSEC NSEC3)
#define DNS_TYPE_NSEC3      0x0032      //  50

//  RFC 5155    (DNSSEC NSEC3PARAM)
#define DNS_TYPE_NSEC3PARAM 0x0033      //  51

//  RFC 6698    (TLSA)
#define DNS_TYPE_TLSA       0x0034      //  52

//  draft-ietf-dnsop-svcb-https
#define DNS_TYPE_SVCB       0x0040      //  64

//  draft-ietf-dnsop-svcb-https
#define DNS_TYPE_HTTPS      0x0041      //  65

//
//  IANA Reserved
//

#define DNS_TYPE_UINFO      0x0064      //  100
#define DNS_TYPE_UID        0x0065      //  101
#define DNS_TYPE_GID        0x0066      //  102
#define DNS_TYPE_UNSPEC     0x0067      //  103

//
//  Query only types (1035, 1995)
//      - Crawford      (ADDRS)
//      - TKEY draft    (TKEY)
//      - TSIG draft    (TSIG)
//      - RFC 1995      (IXFR)
//      - RFC 1035      (AXFR up)
//

#define DNS_TYPE_ADDRS      0x00f8      //  248
#define DNS_TYPE_TKEY       0x00f9      //  249
#define DNS_TYPE_TSIG       0x00fa      //  250
#define DNS_TYPE_IXFR       0x00fb      //  251
#define DNS_TYPE_AXFR       0x00fc      //  252
#define DNS_TYPE_MAILB      0x00fd      //  253
#define DNS_TYPE_MAILA      0x00fe      //  254
#define DNS_TYPE_ALL        0x00ff      //  255
#define DNS_TYPE_ANY        0x00ff      //  255

//
//  Private use Microsoft types --  See www.iana.org/assignments/dns-parameters
//

#define DNS_TYPE_WINS       0xff01      //  64K - 255
#define DNS_TYPE_WINSR      0xff02      //  64K - 254
#define DNS_TYPE_NBSTAT     (DNS_TYPE_WINSR)

//
//  DNS Record Types -- Net Byte Order
//

#define DNS_RTYPE_A              0x0100  //  1
#define DNS_RTYPE_NS             0x0200  //  2
#define DNS_RTYPE_MD             0x0300  //  3
#define DNS_RTYPE_MF             0x0400  //  4
#define DNS_RTYPE_CNAME          0x0500  //  5
#define DNS_RTYPE_SOA            0x0600  //  6
#define DNS_RTYPE_MB             0x0700  //  7
#define DNS_RTYPE_MG             0x0800  //  8
#define DNS_RTYPE_MR             0x0900  //  9
#define DNS_RTYPE_NULL           0x0a00  //  10
#define DNS_RTYPE_WKS            0x0b00  //  11
#define DNS_RTYPE_PTR            0x0c00  //  12
#define DNS_RTYPE_HINFO          0x0d00  //  13
#define DNS_RTYPE_MINFO          0x0e00  //  14
#define DNS_RTYPE_MX             0x0f00  //  15
#define DNS_RTYPE_TEXT           0x1000  //  16
#define DNS_RTYPE_RP             0x1100  //  17
#define DNS_RTYPE_AFSDB          0x1200  //  18
#define DNS_RTYPE_X25            0x1300  //  19
#define DNS_RTYPE_ISDN           0x1400  //  20
#define DNS_RTYPE_RT             0x1500  //  21
#define DNS_RTYPE_NSAP           0x1600  //  22
#define DNS_RTYPE_NSAPPTR        0x1700  //  23
#define DNS_RTYPE_SIG            0x1800  //  24
#define DNS_RTYPE_KEY            0x1900  //  25
#define DNS_RTYPE_PX             0x1a00  //  26
#define DNS_RTYPE_GPOS           0x1b00  //  27
#define DNS_RTYPE_AAAA           0x1c00  //  28
#define DNS_RTYPE_LOC            0x1d00  //  29
#define DNS_RTYPE_NXT            0x1e00  //  30
#define DNS_RTYPE_EID            0x1f00  //  31
#define DNS_RTYPE_NIMLOC         0x2000  //  32
#define DNS_RTYPE_SRV            0x2100  //  33
#define DNS_RTYPE_ATMA           0x2200  //  34
#define DNS_RTYPE_NAPTR          0x2300  //  35
#define DNS_RTYPE_KX             0x2400  //  36
#define DNS_RTYPE_CERT           0x2500  //  37
#define DNS_RTYPE_A6             0x2600  //  38
#define DNS_RTYPE_DNAME          0x2700  //  39
#define DNS_RTYPE_SINK           0x2800  //  40
#define DNS_RTYPE_OPT            0x2900  //  41

#define DNS_RTYPE_DS             0x2b00  //  43
#define DNS_RTYPE_RRSIG          0x2e00  //  46
#define DNS_RTYPE_NSEC           0x2f00  //  47
#define DNS_RTYPE_DNSKEY         0x3000  //  48
#define DNS_RTYPE_DHCID          0x3100  //  49
#define DNS_RTYPE_NSEC3          0x3200  //  50
#define DNS_RTYPE_NSEC3PARAM     0x3300  //  51
#define DNS_RTYPE_TLSA           0x3400  //  52

//
//  IANA Reserved
//

#define DNS_RTYPE_UINFO          0x6400  //  100
#define DNS_RTYPE_UID            0x6500  //  101
#define DNS_RTYPE_GID            0x6600  //  102
#define DNS_RTYPE_UNSPEC         0x6700  //  103

//
//  Query only types
//

#define DNS_RTYPE_TKEY           0xf900  //  249
#define DNS_RTYPE_TSIG           0xfa00  //  250
#define DNS_RTYPE_IXFR           0xfb00  //  251
#define DNS_RTYPE_AXFR           0xfc00  //  252
#define DNS_RTYPE_MAILB          0xfd00  //  253
#define DNS_RTYPE_MAILA          0xfe00  //  254
#define DNS_RTYPE_ALL            0xff00  //  255
#define DNS_RTYPE_ANY            0xff00  //  255

//
//  Private use Microsoft types --  See www.iana.org/assignments/dns-parameters
//

#define DNS_RTYPE_WINS           0x01ff  //  64K - 255
#define DNS_RTYPE_WINSR          0x02ff  //  64K - 254

//
//  Record type specific definitions
//

//
//  ATMA (ATM address type) formats
//
//  Define these directly for any environment (ex NT4)
//  without winsock2 ATM support (ws2atm.h)
//

#ifndef  ATMA_E164
#define DNS_ATMA_FORMAT_E164            1
#define DNS_ATMA_FORMAT_AESA            2
#define DNS_ATMA_MAX_ADDR_LENGTH        (20)
#else
#define DNS_ATMA_FORMAT_E164            ATM_E164
#define DNS_ATMA_FORMAT_AESA            ATM_AESA
#define DNS_ATMA_MAX_ADDR_LENGTH        ATM_ADDR_SIZE
#endif

#define DNS_ATMA_AESA_ADDR_LENGTH       (20)
#define DNS_ATMA_MAX_RECORD_LENGTH      (DNS_ATMA_MAX_ADDR_LENGTH+1)

//
//  DNSSEC defs
//

//  DNSSEC algorithms

#define DNSSEC_ALGORITHM_RSAMD5                 1
#define DNSSEC_ALGORITHM_RSASHA1                5
#define DNSSEC_ALGORITHM_RSASHA1_NSEC3          7
#define DNSSEC_ALGORITHM_RSASHA256              8
#define DNSSEC_ALGORITHM_RSASHA512              10
#define DNSSEC_ALGORITHM_ECDSAP256_SHA256       13
#define DNSSEC_ALGORITHM_ECDSAP384_SHA384       14
#define DNSSEC_ALGORITHM_NULL                   253
#define DNSSEC_ALGORITHM_PRIVATE                254

//  DNSSEC DS record digest algorithms

#define DNSSEC_DIGEST_ALGORITHM_SHA1            1
#define DNSSEC_DIGEST_ALGORITHM_SHA256          2
#define DNSSEC_DIGEST_ALGORITHM_SHA384          4

//  DNSSEC KEY protocol table

#define DNSSEC_PROTOCOL_NONE        0
#define DNSSEC_PROTOCOL_TLS         1
#define DNSSEC_PROTOCOL_EMAIL       2
#define DNSSEC_PROTOCOL_DNSSEC      3
#define DNSSEC_PROTOCOL_IPSEC       4

//  DNSSEC KEY flag field

#define DNSSEC_KEY_FLAG_NOAUTH          0x0001
#define DNSSEC_KEY_FLAG_NOCONF          0x0002
#define DNSSEC_KEY_FLAG_FLAG2           0x0004
#define DNSSEC_KEY_FLAG_EXTEND          0x0008
#define DNSSEC_KEY_FLAG_
#define DNSSEC_KEY_FLAG_FLAG4           0x0010
#define DNSSEC_KEY_FLAG_FLAG5           0x0020

// bits 6,7 are name type

#define DNSSEC_KEY_FLAG_USER            0x0000
#define DNSSEC_KEY_FLAG_ZONE            0x0040
#define DNSSEC_KEY_FLAG_HOST            0x0080
#define DNSSEC_KEY_FLAG_NTPE3           0x00c0

// bits 8-11 are reserved for future use

#define DNSSEC_KEY_FLAG_FLAG8           0x0100
#define DNSSEC_KEY_FLAG_FLAG9           0x0200
#define DNSSEC_KEY_FLAG_FLAG10          0x0400
#define DNSSEC_KEY_FLAG_FLAG11          0x0800

// bits 12-15 are sig field

#define DNSSEC_KEY_FLAG_SIG0            0x0000
#define DNSSEC_KEY_FLAG_SIG1            0x1000
#define DNSSEC_KEY_FLAG_SIG2            0x2000
#define DNSSEC_KEY_FLAG_SIG3            0x3000
#define DNSSEC_KEY_FLAG_SIG4            0x4000
#define DNSSEC_KEY_FLAG_SIG5            0x5000
#define DNSSEC_KEY_FLAG_SIG6            0x6000
#define DNSSEC_KEY_FLAG_SIG7            0x7000
#define DNSSEC_KEY_FLAG_SIG8            0x8000
#define DNSSEC_KEY_FLAG_SIG9            0x9000
#define DNSSEC_KEY_FLAG_SIG10           0xa000
#define DNSSEC_KEY_FLAG_SIG11           0xb000
#define DNSSEC_KEY_FLAG_SIG12           0xc000
#define DNSSEC_KEY_FLAG_SIG13           0xd000
#define DNSSEC_KEY_FLAG_SIG14           0xe000
#define DNSSEC_KEY_FLAG_SIG15           0xf000


//
//  TKEY modes
//

#define DNS_TKEY_MODE_SERVER_ASSIGN         1
#define DNS_TKEY_MODE_DIFFIE_HELLMAN        2
#define DNS_TKEY_MODE_GSS                   3
#define DNS_TKEY_MODE_RESOLVER_ASSIGN       4


//
//  DNS resource record structure
//

//
//  Record data for specific types
//

typedef struct
{
    IP4_ADDRESS     IpAddress;
}
DNS_A_DATA, *PDNS_A_DATA;

typedef struct
{
    PWSTR           pNameHost;
}
DNS_PTR_DATAW, *PDNS_PTR_DATAW;

typedef struct
{
    PSTR            pNameHost;
}
DNS_PTR_DATAA, *PDNS_PTR_DATAA;

typedef struct
{
    PWSTR           pNamePrimaryServer;
    PWSTR           pNameAdministrator;
    DWORD           dwSerialNo;
    DWORD           dwRefresh;
    DWORD           dwRetry;
    DWORD           dwExpire;
    DWORD           dwDefaultTtl;
}
DNS_SOA_DATAW, *PDNS_SOA_DATAW;

typedef struct
{
    PSTR            pNamePrimaryServer;
    PSTR            pNameAdministrator;
    DWORD           dwSerialNo;
    DWORD           dwRefresh;
    DWORD           dwRetry;
    DWORD           dwExpire;
    DWORD           dwDefaultTtl;
}
DNS_SOA_DATAA, *PDNS_SOA_DATAA;

typedef struct
{
    PWSTR           pNameMailbox;
    PWSTR           pNameErrorsMailbox;
}
DNS_MINFO_DATAW, *PDNS_MINFO_DATAW;

typedef struct
{
    PSTR            pNameMailbox;
    PSTR            pNameErrorsMailbox;
}
DNS_MINFO_DATAA, *PDNS_MINFO_DATAA;

typedef struct
{
    PWSTR           pNameExchange;
    WORD            wPreference;
    WORD            Pad;        // keep ptrs DWORD aligned
}
DNS_MX_DATAW, *PDNS_MX_DATAW;

typedef struct
{
    PSTR            pNameExchange;
    WORD            wPreference;
    WORD            Pad;        // keep ptrs DWORD aligned
}
DNS_MX_DATAA, *PDNS_MX_DATAA;

typedef struct
{
    DWORD           dwStringCount;
#ifdef MIDL_PASS
    [size_is(dwStringCount)] PWSTR pStringArray[];
#else
    PWSTR           pStringArray[1];
#endif
}
DNS_TXT_DATAW, *PDNS_TXT_DATAW;

typedef struct
{
    DWORD           dwStringCount;
#ifdef MIDL_PASS
    [size_is(dwStringCount)] PSTR  pStringArray[];
#else
    PSTR            pStringArray[1];
#endif
}
DNS_TXT_DATAA, *PDNS_TXT_DATAA;

typedef struct
{
    DWORD           dwByteCount;
#ifdef MIDL_PASS
    [size_is(dwByteCount)] BYTE Data[];
#else
    BYTE            Data[1];
#endif
}
DNS_NULL_DATA, *PDNS_NULL_DATA;

typedef struct
{
    IP4_ADDRESS     IpAddress;
    UCHAR           chProtocol;
    BYTE            BitMask[1];
}
DNS_WKS_DATA, *PDNS_WKS_DATA;

typedef struct
{
    IP6_ADDRESS     Ip6Address;
}
DNS_AAAA_DATA, *PDNS_AAAA_DATA;

typedef struct
{
    WORD            wTypeCovered;
    BYTE            chAlgorithm;
    BYTE            chLabelCount;
    DWORD           dwOriginalTtl;
    DWORD           dwExpiration;
    DWORD           dwTimeSigned;
    WORD            wKeyTag;
    WORD            wSignatureLength;
    PWSTR           pNameSigner;
#ifdef MIDL_PASS
    [size_is(wSignatureLength)] BYTE  Signature[];
#else
    BYTE            Signature[1];
#endif
}
DNS_SIG_DATAW, *PDNS_SIG_DATAW, DNS_RRSIG_DATAW, *PDNS_RRSIG_DATAW;

typedef struct
{
    WORD            wTypeCovered;
    BYTE            chAlgorithm;
    BYTE            chLabelCount;
    DWORD           dwOriginalTtl;
    DWORD           dwExpiration;
    DWORD           dwTimeSigned;
    WORD            wKeyTag;
    WORD            wSignatureLength;
    PSTR            pNameSigner;
#ifdef MIDL_PASS
    [size_is(wSignatureLength)] BYTE  Signature[];
#else
    BYTE            Signature[1];
#endif
}
DNS_SIG_DATAA, *PDNS_SIG_DATAA, DNS_RRSIG_DATAA, *PDNS_RRSIG_DATAA;

typedef struct
{
    WORD            wFlags;
    BYTE            chProtocol;
    BYTE            chAlgorithm;
    WORD            wKeyLength;
    WORD            wPad;            // keep byte field aligned
#ifdef MIDL_PASS
    [size_is(wKeyLength)] BYTE Key[];
#else
    BYTE            Key[1];
#endif
}
DNS_KEY_DATA, *PDNS_KEY_DATA, DNS_DNSKEY_DATA, *PDNS_DNSKEY_DATA;

typedef struct
{
    DWORD           dwByteCount;
#ifdef MIDL_PASS
    [size_is(dwByteCount)] BYTE DHCID[];
#else
    BYTE            DHCID[1];
#endif
}
DNS_DHCID_DATA, *PDNS_DHCID_DATA;

typedef struct
{
    PWSTR           pNextDomainName;
    WORD            wTypeBitMapsLength;
    WORD            wPad;            // keep byte field aligned
#ifdef MIDL_PASS
    [size_is(wTypeBitMapsLength)] BYTE  TypeBitMaps[];
#else
    BYTE            TypeBitMaps[1];
#endif
}
DNS_NSEC_DATAW, *PDNS_NSEC_DATAW;

typedef struct
{
    PSTR            pNextDomainName;
    WORD            wTypeBitMapsLength;
    WORD            wPad;            // keep byte field aligned
#ifdef MIDL_PASS
    [size_is(wTypeBitMapsLength)] BYTE  TypeBitMaps[];
#else
    BYTE            TypeBitMaps[1];
#endif
}
DNS_NSEC_DATAA, *PDNS_NSEC_DATAA;

typedef struct
{
    BYTE            chAlgorithm;
    BYTE            bFlags;
    WORD            wIterations;
    BYTE            bSaltLength;
    BYTE            bHashLength;
    WORD            wTypeBitMapsLength;
#ifdef MIDL_PASS
    [size_is(bSaltLength+bHashLength+wTypeBitMapsLength)] BYTE  chData[];
#else
    BYTE            chData[1];
#endif
}
DNS_NSEC3_DATA, *PDNS_NSEC3_DATA;

typedef struct
{
    BYTE            chAlgorithm;
    BYTE            bFlags;
    WORD            wIterations;
    BYTE            bSaltLength;
    BYTE            bPad[3];        // keep salt field aligned
#ifdef MIDL_PASS
    [size_is(bSaltLength)] BYTE  pbSalt[];
#else
    BYTE            pbSalt[1];
#endif
}
DNS_NSEC3PARAM_DATA, *PDNS_NSEC3PARAM_DATA;

typedef struct
{
    BYTE            bCertUsage;
    BYTE            bSelector;
    BYTE            bMatchingType;
    WORD            bCertificateAssociationDataLength;
    BYTE            bPad[3];        // keep certificate association data field aligned
#ifdef MIDL_PASS
    [size_is(bCertificateAssociationDataLength)] BYTE  bCertificateAssociationData[];
#else
    BYTE            bCertificateAssociationData[1];
#endif
}
DNS_TLSA_DATA, *PDNS_TLSA_DATA;

typedef struct
{
    WORD            wKeyTag;
    BYTE            chAlgorithm;
    BYTE            chDigestType;
    WORD            wDigestLength;
    WORD            wPad;            // keep byte field aligned
#ifdef MIDL_PASS
    [size_is(wDigestLength)] BYTE  Digest[];
#else
    BYTE            Digest[1];
#endif
}
DNS_DS_DATA, *PDNS_DS_DATA;

typedef struct
{
    WORD            wDataLength;
    WORD            wPad;            // keep byte field aligned
#ifdef MIDL_PASS
    [size_is(wDataLength)] BYTE Data[];
#else
    BYTE            Data[1];
#endif
}
DNS_OPT_DATA, *PDNS_OPT_DATA;

typedef struct
{
    WORD            wVersion;
    WORD            wSize;
    WORD            wHorPrec;
    WORD            wVerPrec;
    DWORD           dwLatitude;
    DWORD           dwLongitude;
    DWORD           dwAltitude;
}
DNS_LOC_DATA, *PDNS_LOC_DATA;

typedef struct
{
    PWSTR           pNameNext;
    WORD            wNumTypes;
#ifdef MIDL_PASS
    [size_is(wNumTypes)] WORD wTypes[];
#else
    WORD            wTypes[1];
#endif
}
DNS_NXT_DATAW, *PDNS_NXT_DATAW;

typedef struct
{
    PSTR            pNameNext;
    WORD            wNumTypes;
#ifdef MIDL_PASS
    [size_is(wNumTypes)] WORD wTypes[];
#else
    WORD            wTypes[1];
#endif
}
DNS_NXT_DATAA, *PDNS_NXT_DATAA;

typedef struct
{
    PWSTR           pNameTarget;
    WORD            wPriority;
    WORD            wWeight;
    WORD            wPort;
    WORD            Pad;            // keep ptrs DWORD aligned
}
DNS_SRV_DATAW, *PDNS_SRV_DATAW;

typedef struct
{
    PSTR            pNameTarget;
    WORD            wPriority;
    WORD            wWeight;
    WORD            wPort;
    WORD            Pad;            // keep ptrs DWORD aligned
}
DNS_SRV_DATAA, *PDNS_SRV_DATAA;

typedef struct
{
    WORD            wOrder;
    WORD            wPreference;
    PWSTR           pFlags;
    PWSTR           pService;
    PWSTR           pRegularExpression;
    PWSTR           pReplacement;
}
DNS_NAPTR_DATAW, *PDNS_NAPTR_DATAW;

typedef struct
{
    WORD            wOrder;
    WORD            wPreference;
    PSTR            pFlags;
    PSTR            pService;
    PSTR            pRegularExpression;
    PSTR            pReplacement;
}
DNS_NAPTR_DATAA, *PDNS_NAPTR_DATAA;



typedef struct
{
    BYTE            AddressType;
    BYTE            Address[ DNS_ATMA_MAX_ADDR_LENGTH ];

    //  E164 -- Null terminated string of less than
    //      DNS_ATMA_MAX_ADDR_LENGTH
    //
    //  For NSAP (AESA) BCD encoding of exactly
    //      DNS_ATMA_AESA_ADDR_LENGTH
}
DNS_ATMA_DATA, *PDNS_ATMA_DATA;


typedef struct
{
    PWSTR           pNameAlgorithm;

#ifdef MIDL_PASS
    [size_is(cAlgNameLength)]
#endif
    PBYTE           pAlgorithmPacket;

#ifdef MIDL_PASS
    [size_is(wKeyLength)]
#endif
    PBYTE           pKey;

#ifdef MIDL_PASS
    [size_is(wOtherLength)]
#endif
    PBYTE           pOtherData;

    DWORD           dwCreateTime;
    DWORD           dwExpireTime;
    WORD            wMode;
    WORD            wError;
    WORD            wKeyLength;
    WORD            wOtherLength;
    UCHAR           cAlgNameLength;
    BOOL            bPacketPointers;
}
DNS_TKEY_DATAW, *PDNS_TKEY_DATAW;

typedef struct
{
    PSTR            pNameAlgorithm;

#ifdef MIDL_PASS
    [size_is(cAlgNameLength)]
#endif
    PBYTE           pAlgorithmPacket;

#ifdef MIDL_PASS
    [size_is(wKeyLength)]
#endif
    PBYTE           pKey;

#ifdef MIDL_PASS
    [size_is(wOtherLength)]
#endif
    PBYTE           pOtherData;

    DWORD           dwCreateTime;
    DWORD           dwExpireTime;
    WORD            wMode;
    WORD            wError;
    WORD            wKeyLength;
    WORD            wOtherLength;
    UCHAR           cAlgNameLength;
    BOOL            bPacketPointers;
}
DNS_TKEY_DATAA, *PDNS_TKEY_DATAA;

typedef struct
{
    PWSTR           pNameAlgorithm;

#ifdef MIDL_PASS
    [size_is(cAlgNameLength)]
#endif
    PBYTE           pAlgorithmPacket;

#ifdef MIDL_PASS
    [size_is(wSigLength)]
#endif
    PBYTE           pSignature;

#ifdef MIDL_PASS
    [size_is(wOtherLength)]
#endif
    PBYTE           pOtherData;

    LONGLONG        i64CreateTime;
    WORD            wFudgeTime;
    WORD            wOriginalXid;
    WORD            wError;
    WORD            wSigLength;
    WORD            wOtherLength;
    UCHAR           cAlgNameLength;
    BOOL            bPacketPointers;
}
DNS_TSIG_DATAW, *PDNS_TSIG_DATAW;

typedef struct
{
    PSTR            pNameAlgorithm;

#ifdef MIDL_PASS
    [size_is(cAlgNameLength)]
#endif
    PBYTE           pAlgorithmPacket;

#ifdef MIDL_PASS
    [size_is(wSigLength)]
#endif
    PBYTE           pSignature;

#ifdef MIDL_PASS
        [size_is(wOtherLength)]
#endif
    PBYTE           pOtherData;

    LONGLONG        i64CreateTime;
    WORD            wFudgeTime;
    WORD            wOriginalXid;
    WORD            wError;
    WORD            wSigLength;
    WORD            wOtherLength;
    UCHAR           cAlgNameLength;
    BOOL            bPacketPointers;
}
DNS_TSIG_DATAA, *PDNS_TSIG_DATAA;

typedef struct
{
    DWORD           dwByteCount;
#ifdef MIDL_PASS
    [size_is(dwByteCount)] BYTE bData[];
#else
    BYTE            bData[1];
#endif
}
DNS_UNKNOWN_DATA, *PDNS_UNKNOWN_DATA;

//
//  MS only types -- only hit the wire in MS-MS zone transfer
//

typedef struct
{
    DWORD           dwMappingFlag;
    DWORD           dwLookupTimeout;
    DWORD           dwCacheTimeout;
    DWORD           cWinsServerCount;
#ifdef MIDL_PASS
    [size_is(cWinsServerCount)]
    IP4_ADDRESS     WinsServers[];
#else
    IP4_ADDRESS     WinsServers[1];
#endif
}
DNS_WINS_DATA, *PDNS_WINS_DATA;

typedef struct
{
    DWORD           dwMappingFlag;
    DWORD           dwLookupTimeout;
    DWORD           dwCacheTimeout;
    PWSTR           pNameResultDomain;
}
DNS_WINSR_DATAW, *PDNS_WINSR_DATAW;

typedef struct
{
    DWORD           dwMappingFlag;
    DWORD           dwLookupTimeout;
    DWORD           dwCacheTimeout;
    PSTR            pNameResultDomain;
}
DNS_WINSR_DATAA, *PDNS_WINSR_DATAA;

#define DDR_MAX_IP_HINTS 4

typedef enum _DNS_SVCB_PARAM_TYPE
{
    DnsSvcbParamMandatory      = 0,
    DnsSvcbParamAlpn           = 1,
    DnsSvcbParamNoDefaultAlpn  = 2,
    DnsSvcbParamPort           = 3,
    DnsSvcbParamIpv4Hint       = 4,
    DnsSvcbParamEch            = 5,
    DnsSvcbParamIpv6Hint       = 6,
    DnsSvcbParamDohPath        = 7,
    DnsSvcbParamDohPathOpenDns = 65432,
} DNS_SVCB_PARAM_TYPE;

typedef struct _DNS_SVCB_PARAM_MANDATORY
{
    WORD cMandatoryKeys;
    WORD rgwMandatoryKeys[1];
} DNS_SVCB_PARAM_MANDATORY;

typedef struct _DNS_SVCB_PARAM_ALPN_ID
{
    BYTE cBytes;
    BYTE *pbId;
} DNS_SVCB_PARAM_ALPN_ID;

typedef struct _DNS_SVCB_PARAM_ALPN
{
    WORD                   cIds;
    DNS_SVCB_PARAM_ALPN_ID rgIds[1];
} DNS_SVCB_PARAM_ALPN;

typedef struct _DNS_SVCB_PARAM_IPV4
{
    WORD        cIps;
    IP4_ADDRESS rgIps[1];
} DNS_SVCB_PARAM_IPV4;

typedef struct _DNS_SVCB_PARAM_IPV6
{
    WORD        cIps;
    IP6_ADDRESS rgIps[1];
} DNS_SVCB_PARAM_IPV6;

typedef struct _DNS_SVCB_PARAM_UNKNOWN
{
    WORD cBytes;
    BYTE pbSvcParamValue[1];
} DNS_SVCB_PARAM_UNKNOWN;

#pragma warning(push)
#pragma warning(disable: 4201) // nameless struct/union (anonymous union for SVCB parameter variants)
typedef struct _DNS_SVCB_PARAM
{
    WORD wSvcParamKey;
    union
    {
        DNS_SVCB_PARAM_IPV4      *pIpv4Hints;
        DNS_SVCB_PARAM_IPV6      *pIpv6Hints;
        DNS_SVCB_PARAM_MANDATORY *pMandatory;
        DNS_SVCB_PARAM_ALPN      *pAlpn;
        WORD                     wPort;
        DNS_SVCB_PARAM_UNKNOWN   *pUnknown;
        PSTR                     pszDohPath;
        PVOID                    pReserved;
    };
} DNS_SVCB_PARAM;
#pragma warning(pop)

typedef struct _DNS_SVCB_DATA
{
    WORD           wSvcPriority;
    PSTR           pszTargetName;
    WORD           cSvcParams;
    DNS_SVCB_PARAM *pSvcParams;
} DNS_SVCB_DATA;

//
//  Unicode/ANSI record types
//

#ifdef UNICODE
typedef DNS_PTR_DATAW   DNS_PTR_DATA,   *PDNS_PTR_DATA;
typedef DNS_SOA_DATAW   DNS_SOA_DATA,   *PDNS_SOA_DATA;
typedef DNS_MINFO_DATAW DNS_MINFO_DATA, *PDNS_MINFO_DATA;
typedef DNS_MX_DATAW    DNS_MX_DATA,    *PDNS_MX_DATA;
typedef DNS_TXT_DATAW   DNS_TXT_DATA,   *PDNS_TXT_DATA;
typedef DNS_SIG_DATAW   DNS_SIG_DATA,   *PDNS_SIG_DATA;
typedef DNS_NXT_DATAW   DNS_NXT_DATA,   *PDNS_NXT_DATA;
typedef DNS_SRV_DATAW   DNS_SRV_DATA,   *PDNS_SRV_DATA;
typedef DNS_NAPTR_DATAW DNS_NAPTR_DATA, *PDNS_NAPTR_DATA;
typedef DNS_RRSIG_DATAW DNS_RRSIG_DATA, *PDNS_RRSIG_DATA;
typedef DNS_NSEC_DATAW  DNS_NSEC_DATA,  *PDNS_NSEC_DATA;
typedef DNS_TKEY_DATAW  DNS_TKEY_DATA,  *PDNS_TKEY_DATA;
typedef DNS_TSIG_DATAW  DNS_TSIG_DATA,  *PDNS_TSIG_DATA;
typedef DNS_WINSR_DATAW DNS_WINSR_DATA, *PDNS_WINSR_DATA;
#else
typedef DNS_PTR_DATAA   DNS_PTR_DATA,   *PDNS_PTR_DATA;
typedef DNS_SOA_DATAA   DNS_SOA_DATA,   *PDNS_SOA_DATA;
typedef DNS_MINFO_DATAA DNS_MINFO_DATA, *PDNS_MINFO_DATA;
typedef DNS_MX_DATAA    DNS_MX_DATA,    *PDNS_MX_DATA;
typedef DNS_TXT_DATAA   DNS_TXT_DATA,   *PDNS_TXT_DATA;
typedef DNS_SIG_DATAA   DNS_SIG_DATA,   *PDNS_SIG_DATA;
typedef DNS_NXT_DATAA   DNS_NXT_DATA,   *PDNS_NXT_DATA;
typedef DNS_SRV_DATAA   DNS_SRV_DATA,   *PDNS_SRV_DATA;
typedef DNS_NAPTR_DATAA DNS_NAPTR_DATA, *PDNS_NAPTR_DATA;
typedef DNS_RRSIG_DATAA DNS_RRSIG_DATA, *PDNS_RRSIG_DATA;
typedef DNS_NSEC_DATAA  DNS_NSEC_DATA,  *PDNS_NSEC_DATA;
typedef DNS_TKEY_DATAA  DNS_TKEY_DATA,  *PDNS_TKEY_DATA;
typedef DNS_TSIG_DATAA  DNS_TSIG_DATA,  *PDNS_TSIG_DATA;
typedef DNS_WINSR_DATAA DNS_WINSR_DATA, *PDNS_WINSR_DATA;
#endif

//
//  Length of non-fixed-length data types
//

#define DNS_TEXT_RECORD_LENGTH(StringCount) \
            (FIELD_OFFSET(DNS_TXT_DATA, pStringArray) + ((StringCount) * sizeof(PCHAR)))

#define DNS_NULL_RECORD_LENGTH(ByteCount) \
            (FIELD_OFFSET(DNS_NULL_DATA, Data) + (ByteCount))

#define DNS_WKS_RECORD_LENGTH(ByteCount) \
            (FIELD_OFFSET(DNS_WKS_DATA, BitMask) + (ByteCount))

#define DNS_WINS_RECORD_LENGTH(IpCount) \
            (FIELD_OFFSET(DNS_WINS_DATA, WinsServers) + ((IpCount) * sizeof(IP4_ADDRESS)))

#define DNS_KEY_RECORD_LENGTH(ByteCount) \
            (FIELD_OFFSET(DNS_KEY_DATA, Key) + (ByteCount))

#define DNS_SIG_RECORD_LENGTH(ByteCount) \
            (FIELD_OFFSET(DNS_SIG_DATA, Signature) + (ByteCount))

#define DNS_NSEC_RECORD_LENGTH(ByteCount) \
            (FIELD_OFFSET(DNS_NSEC_DATA, TypeBitMaps) + (ByteCount))

#define DNS_DS_RECORD_LENGTH(ByteCount) \
            (FIELD_OFFSET(DNS_DS_DATA, Digest) + (ByteCount))

#define DNS_OPT_RECORD_LENGTH(ByteCount) \
            (FIELD_OFFSET(DNS_OPT_DATA, Data) + (ByteCount))

#define DNS_DHCID_RECORD_LENGTH(ByteCount) \
            (FIELD_OFFSET(DNS_DHCID_DATA, DHCID) + (ByteCount))

#define DNS_NSEC3_RECORD_LENGTH(ByteCount) \
            (FIELD_OFFSET(DNS_NSEC3_DATA, chData) + (ByteCount))

#define DNS_NSEC3PARAM_RECORD_LENGTH(ByteCount) \
            (FIELD_OFFSET(DNS_NSEC3PARAM_DATA, pbSalt) + (ByteCount))

#define DNS_TLSA_RECORD_LENGTH(ByteCount) \
            (FIELD_OFFSET(DNS_TLSA_DATA, bCertificateAssociationData) + (ByteCount))

#define DNS_UNKNOWN_RECORD_LENGTH(ByteCount) \
            (FIELD_OFFSET(DNS_UNKNOWN_DATA, bData) + (ByteCount))

//
//  Record flags
//

typedef struct _DnsRecordFlags
{
    DWORD   Section     : 2;
    DWORD   Delete      : 1;
    DWORD   CharSet     : 2;
    DWORD   Unused      : 3;

    DWORD   Reserved    : 24;
}
DNS_RECORD_FLAGS;

//
//  Wire Record Sections
//
//  Useable both in record flags "Section" and as index into
//  wire message header section counts.
//

typedef enum _DnsSection
{
    DnsSectionQuestion,
    DnsSectionAnswer,
    DnsSectionAuthority,
    DnsSectionAddtional,
}
DNS_SECTION;

//  Update message section names

#define DnsSectionZone      DnsSectionQuestion
#define DnsSectionPrereq    DnsSectionAnswer
#define DnsSectionUpdate    DnsSectionAuthority


//
//  Record flags as bit flags
//  These may be or'd together to set the fields
//

//  RR Section in packet

#define     DNSREC_SECTION      (0x00000003)

#define     DNSREC_QUESTION     (0x00000000)
#define     DNSREC_ANSWER       (0x00000001)
#define     DNSREC_AUTHORITY    (0x00000002)
#define     DNSREC_ADDITIONAL   (0x00000003)

//  RR Section in packet (update)

#define     DNSREC_ZONE         (0x00000000)
#define     DNSREC_PREREQ       (0x00000001)
#define     DNSREC_UPDATE       (0x00000002)

//  Delete RR (update) or No-exist (prerequisite)

#define     DNSREC_DELETE       (0x00000004)
#define     DNSREC_NOEXIST      (0x00000004)


//
//  Record \ RR set structure
//
//  Note:  The dwReserved flag serves to insure that the substructures
//  start on 64-bit boundaries.  Do NOT pack this structure, as the
//  substructures may contain pointers or int64 values which are
//  properly aligned unpacked.
//

#ifdef MIDL_PASS

#define PDNS_RECORD     PVOID
#define PDNS_RECORDA    PVOID
#define PDNS_RECORDW    PVOID

#else

typedef _Struct_size_bytes_(FIELD_OFFSET(struct _DnsRecordW, Data) + wDataLength) struct _DnsRecordW
{
    struct _DnsRecordW *    pNext;
    PWSTR                   pName;
    WORD                    wType;
    WORD                    wDataLength;    // Not referenced for DNS record types
                                            // defined above.
    union
    {
        DWORD               DW;     // flags as DWORD
        DNS_RECORD_FLAGS    S;      // flags as structure

    } Flags;

    DWORD                   dwTtl;
    DWORD                   dwReserved;

    //  Record Data

    union
    {
        DNS_A_DATA          A;
        DNS_SOA_DATAW       SOA, Soa;
        DNS_PTR_DATAW       PTR, Ptr,
                            NS, Ns,
                            CNAME, Cname,
                            DNAME, Dname,
                            MB, Mb,
                            MD, Md,
                            MF, Mf,
                            MG, Mg,
                            MR, Mr;
        DNS_MINFO_DATAW     MINFO, Minfo,
                            RP, Rp;
        DNS_MX_DATAW        MX, Mx,
                            AFSDB, Afsdb,
                            RT, Rt;
        DNS_TXT_DATAW       HINFO, Hinfo,
                            ISDN, Isdn,
                            TXT, Txt,
                            X25;
        DNS_NULL_DATA       Null;
        DNS_WKS_DATA        WKS, Wks;
        DNS_AAAA_DATA       AAAA;
        DNS_KEY_DATA        KEY, Key;
        DNS_SIG_DATAW       SIG, Sig;
        DNS_ATMA_DATA       ATMA, Atma;
        DNS_NXT_DATAW       NXT, Nxt;
        DNS_SRV_DATAW       SRV, Srv;
        DNS_NAPTR_DATAW     NAPTR, Naptr;
        DNS_OPT_DATA        OPT, Opt;
        DNS_DS_DATA         DS, Ds;
        DNS_RRSIG_DATAW     RRSIG, Rrsig;
        DNS_NSEC_DATAW      NSEC, Nsec;
        DNS_DNSKEY_DATA     DNSKEY, Dnskey;
        DNS_TKEY_DATAW      TKEY, Tkey;
        DNS_TSIG_DATAW      TSIG, Tsig;
        DNS_WINS_DATA       WINS, Wins;
        DNS_WINSR_DATAW     WINSR, WinsR, NBSTAT, Nbstat;
        DNS_DHCID_DATA      DHCID;
        DNS_NSEC3_DATA      NSEC3, Nsec3;
        DNS_NSEC3PARAM_DATA NSEC3PARAM, Nsec3Param;
        DNS_TLSA_DATA       TLSA, Tlsa;
        DNS_SVCB_DATA       SVCB, Svcb;
        DNS_UNKNOWN_DATA    UNKNOWN, Unknown;
        PBYTE               pDataPtr;

    } Data;
}
DNS_RECORDW, *PDNS_RECORDW;

typedef struct _DnsRecordOptW
{
    struct _DnsRecordW *    pNext;
    PWSTR                   pName;
    WORD                    wType;
    WORD                    wDataLength;    // Not referenced for DNS record types
                                            // defined above.
    union
    {
        DWORD               DW;     // flags as DWORD
        DNS_RECORD_FLAGS    S;      // flags as structure

    } Flags;

    DNS_HEADER_EXT          ExtHeader;      // TTL
    WORD                    wPayloadSize;   // dwReserved;
    WORD                    wReserved;

    //  Record Data
    union
    {
        DNS_OPT_DATA        OPT, Opt;

    } Data;
}
DNS_RECORD_OPTW, *PDNS_RECORD_OPTW;


typedef _Struct_size_bytes_(FIELD_OFFSET(struct _DnsRecordA, Data) + wDataLength) struct _DnsRecordA
{
    struct _DnsRecordA *    pNext;
    PSTR                    pName;
    WORD                    wType;
    WORD                    wDataLength; // Not referenced for DNS record types
                                     // defined above.
    union
    {
        DWORD               DW;     // flags as DWORD
        DNS_RECORD_FLAGS    S;      // flags as structure

    } Flags;

    DWORD               dwTtl;
    DWORD               dwReserved;

    //  Record Data

    union
    {
        DNS_A_DATA          A;
        DNS_SOA_DATAA       SOA, Soa;
        DNS_PTR_DATAA       PTR, Ptr,
                            NS, Ns,
                            CNAME, Cname,
                            DNAME, Dname,
                            MB, Mb,
                            MD, Md,
                            MF, Mf,
                            MG, Mg,
                            MR, Mr;
        DNS_MINFO_DATAA     MINFO, Minfo,
                            RP, Rp;
        DNS_MX_DATAA        MX, Mx,
                            AFSDB, Afsdb,
                            RT, Rt;
        DNS_TXT_DATAA       HINFO, Hinfo,
                            ISDN, Isdn,
                            TXT, Txt,
                            X25;
        DNS_NULL_DATA       Null;
        DNS_WKS_DATA        WKS, Wks;
        DNS_AAAA_DATA       AAAA;
        DNS_KEY_DATA        KEY, Key;
        DNS_SIG_DATAA       SIG, Sig;
        DNS_ATMA_DATA       ATMA, Atma;
        DNS_NXT_DATAA       NXT, Nxt;
        DNS_SRV_DATAA       SRV, Srv;
        DNS_NAPTR_DATAA     NAPTR, Naptr;
        DNS_OPT_DATA        OPT, Opt;
        DNS_DS_DATA         DS, Ds;
        DNS_RRSIG_DATAA     RRSIG, Rrsig;
        DNS_NSEC_DATAA      NSEC, Nsec;
        DNS_DNSKEY_DATA     DNSKEY, Dnskey;
        DNS_TKEY_DATAA      TKEY, Tkey;
        DNS_TSIG_DATAA      TSIG, Tsig;
        DNS_WINS_DATA       WINS, Wins;
        DNS_WINSR_DATAA     WINSR, WinsR, NBSTAT, Nbstat;
        DNS_DHCID_DATA      DHCID;
        DNS_NSEC3_DATA      NSEC3, Nsec3;
        DNS_NSEC3PARAM_DATA NSEC3PARAM, Nsec3Param;
        DNS_TLSA_DATA       TLSA, Tlsa;
        DNS_SVCB_DATA       SVCB, Svcb;
        DNS_UNKNOWN_DATA    UNKNOWN, Unknown;
        PBYTE               pDataPtr;

    } Data;
}
DNS_RECORDA, *PDNS_RECORDA;


typedef struct _DnsRecordOptA
{
    struct _DnsRecordA *    pNext;
    PSTR                    pName;
    WORD                    wType;
    WORD                    wDataLength; // Not referenced for DNS record types
                                     // defined above.
    union
    {
        DWORD               DW;     // flags as DWORD
        DNS_RECORD_FLAGS    S;      // flags as structure

    } Flags;

    DNS_HEADER_EXT          ExtHeader;      // TTL
    WORD                    wPayloadSize;   // dwReserved;
    WORD                    wReserved;

    //  Record Data

    union
    {
        DNS_OPT_DATA        OPT, Opt;

    } Data;
}
DNS_RECORD_OPTA, *PDNS_RECORD_OPTA;


#ifdef UNICODE
typedef DNS_RECORDW         DNS_RECORD, *PDNS_RECORD;
typedef DNS_RECORD_OPTW     DNS_RECORD_OPT, *PDNS_RECORD_OPT;
#else
typedef DNS_RECORDA         DNS_RECORD, *PDNS_RECORD;
typedef DNS_RECORD_OPTA     DNS_RECORD_OPT, *PDNS_RECORD_OPT;
#endif

//
//  Header or fixed size of DNS_RECORD
//

#define DNS_RECORD_FIXED_SIZE       FIELD_OFFSET( DNS_RECORD, Data )
#define SIZEOF_DNS_RECORD_HEADER    DNS_RECORD_FIXED_SIZE

#endif  // MIDL_PASS

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

//
//  DNS public types
//

#if !defined(_Return_type_success_)
# define _Return_type_success_(expr)
#endif

typedef _Return_type_success_(return == 0) LONG    DNS_STATUS;
typedef DNS_STATUS                                *PDNS_STATUS;

//
//  DNS Address structures representing both IPv4 and IPv6 addresses.
//

#pragma pack(push, 1)

#if defined( MIDL_PASS )

typedef struct _DnsAddr
{
    CHAR        MaxSa[ DNS_ADDR_MAX_SOCKADDR_LENGTH ];
    DWORD       DnsAddrUserDword[ 8 ];
}
DNS_ADDR, *PDNS_ADDR;

typedef struct _DnsAddrArray
{
    DWORD           MaxCount;
    DWORD           AddrCount;
    DWORD           Tag;
    WORD            Family;
    WORD            WordReserved;
    DWORD           Flags;
    DWORD           MatchFlag;
    DWORD           Reserved1;
    DWORD           Reserved2;
    [size_is( AddrCount )]  DNS_ADDR    AddrArray[];
}
DNS_ADDR_ARRAY, *PDNS_ADDR_ARRAY;

#elif !defined( USE_PRIVATE_DNS_ADDR )

typedef struct _DnsAddr
{
    CHAR            MaxSa[ DNS_ADDR_MAX_SOCKADDR_LENGTH ];
    union
    {
        DWORD       DnsAddrUserDword[ 8 ];
    }
    Data;
}
DNS_ADDR, *PDNS_ADDR;

typedef struct _DnsAddrArray
{
    DWORD           MaxCount;
    DWORD           AddrCount;
    DWORD           Tag;
    WORD            Family;
    WORD            WordReserved;
    DWORD           Flags;
    DWORD           MatchFlag;
    DWORD           Reserved1;
    DWORD           Reserved2;
    DNS_ADDR        AddrArray[ 1 ];
}
DNS_ADDR_ARRAY, *PDNS_ADDR_ARRAY;

#endif  // MIDL_PASS
#pragma pack(pop)

//
//  DNS UDP packets no more than 512 bytes
//

#define DNS_RFC_MAX_UDP_PACKET_LENGTH   (512)


//
//  DNS Names limited to 255, 63 in any one label
//

#define DNS_MAX_NAME_LENGTH             (255)
#define DNS_MAX_LABEL_LENGTH            (63)
#define DNS_MAX_NAME_BUFFER_LENGTH      (256)
#define DNS_MAX_LABEL_BUFFER_LENGTH     (64)

#pragma pack(push, 1)

//
//  DNS Message Header
//

typedef struct _DNS_HEADER
{
    WORD    Xid;

#ifdef MIDL_PASS
    WORD    Flags;
#else
    BYTE    RecursionDesired : 1;
    BYTE    Truncation : 1;
    BYTE    Authoritative : 1;
    BYTE    Opcode : 4;
    BYTE    IsResponse : 1;

    BYTE    ResponseCode : 4;
    BYTE    CheckingDisabled : 1;
    BYTE    AuthenticatedData : 1;
    BYTE    Reserved : 1;
    BYTE    RecursionAvailable : 1;
#endif

    WORD    QuestionCount;
    WORD    AnswerCount;
    WORD    NameServerCount;
    WORD    AdditionalCount;
}
DNS_HEADER, *PDNS_HEADER;

//
//  Flags as WORD
//

#define DNS_HEADER_FLAGS(pHead)     ( *((PWORD)(pHead)+1) )

#pragma pack(pop)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion


#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

typedef enum _DNS_CHARSET
{
    DnsCharSetUnknown,
    DnsCharSetUnicode,
    DnsCharSetUtf8,
    DnsCharSetAnsi,
}
DNS_CHARSET;

typedef struct _DNS_MESSAGE_BUFFER
{
    DNS_HEADER  MessageHead;
    CHAR        MessageBody[1];
}
DNS_MESSAGE_BUFFER, *PDNS_MESSAGE_BUFFER;

#define DNS_CUSTOM_SERVER_TYPE_UDP 0x1
#define DNS_CUSTOM_SERVER_TYPE_DOH 0x2
#define DNS_CUSTOM_SERVER_TYPE_DOT 0x3

#define DNS_CUSTOM_SERVER_UDP_FALLBACK 0x1
#define DNS_CUSTOM_SERVER_UPGRADE_FROM_WELL_KNOWN_SERVERS 0x2

#pragma warning(push)
#pragma warning(disable: 4201) // nameless struct/union (anonymous unions for DNS_CUSTOM_SERVER representation variants)

#ifdef MIDL_PASS

typedef struct _DNS_CUSTOM_SERVER
{
    DWORD    dwServerType;
    ULONG64  ullFlags;

    [switch_type(DWORD)]
    [switch_is(dwServerType)]
    union
    {
        [case(DNS_CUSTOM_SERVER_TYPE_DOH)] PWSTR  pwszTemplate;
        [case(DNS_CUSTOM_SERVER_TYPE_DOT)] PWSTR  pwszHostname;
        [case(DNS_CUSTOM_SERVER_TYPE_UDP)] ;
    };

    CHAR MaxSa[DNS_ADDR_MAX_SOCKADDR_LENGTH];
} DNS_CUSTOM_SERVER;

#else

typedef struct _DNS_CUSTOM_SERVER
{
    DWORD    dwServerType;
    ULONG64  ullFlags;

    union
    {
        PWSTR pwszTemplate;
        PWSTR pwszHostname;
    };

    union
    {
#ifdef _WS2TCPIP_H_
        SOCKADDR_INET ServerAddr;
#endif
        CHAR          MaxSa[DNS_ADDR_MAX_SOCKADDR_LENGTH];
    };
} DNS_CUSTOM_SERVER;

#endif // MIDL_PASS

#pragma warning(pop)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion


#pragma region Desktop Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

#ifdef __cplusplus
}
#endif /* __cplusplus */

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#endif // _WINDNSDEF_INCLUDED_
