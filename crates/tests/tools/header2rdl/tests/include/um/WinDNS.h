/*++

Copyright (c) 1996-2005  Microsoft Corporation

Module Name:

    windns.h

Abstract:

    Domain Name System (DNS)

    DNS definitions and DNS API.

Author:

    Jim Gilroy (jamesg)     December 7, 1996

Revision History:

--*/


#ifndef _WINDNS_INCLUDED_
#define _WINDNS_INCLUDED_
#include <winapifamily.h>
#include <windnsdef.h>

#pragma region Desktop Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)



#ifdef __cplusplus
extern "C"
{
#endif  // __cplusplus

//
//  IP Address Array type
//

typedef struct  _IP4_ARRAY
{
    DWORD           AddrCount;
#ifdef MIDL_PASS
    [size_is( AddrCount )]  IP4_ADDRESS  AddrArray[];
#else
    IP4_ADDRESS     AddrArray[1];
#endif
}
IP4_ARRAY, *PIP4_ARRAY;

//
//  IP6 string max is 45 bytes
//      - 6 WORDs in colon+hex (5 chars)
//      - last DWORD as IP4 (15 chars)
//  but include
//      - 11 bytes for scope ID
//      - 6 bytes for port (inc. colon)
//      - two bytes to bracket address with port
//      - terminating NULL
//
//  Note:  this is a change to previous def, but a single
//      definition continaing space for ALL possible IPv6
//      address strings, we elminate many possible errors

#undef  IP6_ADDRESS_STRING_LENGTH
#define IP6_ADDRESS_STRING_LENGTH           (65)
#define IP6_ADDRESS_STRING_BUFFER_LENGTH    (65)

//
//  IP4/IP6 combined maximum
//

#define DNS_ADDRESS_STRING_LENGTH           (IP6_ADDRESS_STRING_LENGTH)


//
//  Inline byte flipping -- can be done in registers
//

#define INLINE_WORD_FLIP(out, in)   \
        {                           \
            WORD _in = (in);        \
            (out) = (_in << 8) | (_in >> 8);  \
        }
#define INLINE_HTONS(out, in)   INLINE_WORD_FLIP(out, in)
#define INLINE_NTOHS(out, in)   INLINE_WORD_FLIP(out, in)

#define INLINE_DWORD_FLIP(out, in)  \
        {                           \
            DWORD _in = (in);       \
            (out) = ((_in << 8) & 0x00ff0000) | \
                    (_in << 24)               | \
                    ((_in >> 8) & 0x0000ff00) | \
                    (_in >> 24);                \
        }
#define INLINE_NTOHL(out, in) INLINE_DWORD_FLIP(out, in)
#define INLINE_HTONL(out, in) INLINE_DWORD_FLIP(out, in)


//
//  Inline byte flip and write to packet (unaligned)
//

#define INLINE_WRITE_FLIPPED_WORD( pout, in ) \
            INLINE_WORD_FLIP( *((UNALIGNED WORD *)(pout)), in )

#define INLINE_WRITE_FLIPPED_DWORD( pout, in ) \
            INLINE_DWORD_FLIP( *((UNALIGNED DWORD *)(pout)), in )




//
//  Basic DNS definitions
//

//
//  DNS port for both UDP and TCP is 53. For DoT, the port is 853.
//

#define DNS_PORT_HOST_ORDER     (0x0035)    // port 53
#define DNS_PORT_NET_ORDER      (0x3500)

#define INTERNET_DEFAULT_DNS_PORT DNS_PORT_HOST_ORDER
#define INTERNET_DEFAULT_DOT_PORT (853)

//
//  Reverse lookup domain names
//

#define DNS_IP4_REVERSE_DOMAIN_STRING_A     ("in-addr.arpa.")
#define DNS_IP4_REVERSE_DOMAIN_STRING_W     (L"in-addr.arpa.")

#define DNS_MAX_IP4_REVERSE_NAME_LENGTH     (IP4_ADDRESS_STRING_LENGTH+15)
            //(IP4_ADDRESS_STRING_LENGTH+1+sizeof(DNS_IP4_REVERSE_DOMAIN_STRING_A))

#define DNS_IP6_REVERSE_DOMAIN_STRING_A     ("ip6.arpa.")
#define DNS_IP6_REVERSE_DOMAIN_STRING_W     (L"ip6.arpa.")

#define DNS_MAX_IP6_REVERSE_NAME_LENGTH     (75)
            //(64+sizeof(DNS_IP6_REVERSE_DOMAIN_STRING_A))

//  Combined

#define DNS_MAX_REVERSE_NAME_LENGTH     DNS_MAX_IP6_REVERSE_NAME_LENGTH

#ifdef UNICODE
#define DNS_IP4_REVERSE_DOMAIN_STRING   DNS_IP4_REVERSE_DOMAIN_STRING_W
#define DNS_IP6_REVERSE_DOMAIN_STRING   DNS_IP6_REVERSE_DOMAIN_STRING_W
#else
#define DNS_IP4_REVERSE_DOMAIN_STRING   DNS_IP4_REVERSE_DOMAIN_STRING_A
#define DNS_IP6_REVERSE_DOMAIN_STRING   DNS_IP6_REVERSE_DOMAIN_STRING_A
#endif

//
//  Backcompat only -- name def contains terminating NULL
//

#define DNS_MAX_IP4_REVERSE_NAME_BUFFER_LENGTH  DNS_MAX_IP4_REVERSE_NAME_LENGTH
#define DNS_MAX_IP6_REVERSE_NAME_BUFFER_LENGTH  DNS_MAX_IP6_REVERSE_NAME_LENGTH
#define DNS_MAX_REVERSE_NAME_BUFFER_LENGTH      DNS_MAX_REVERSE_NAME_LENGTH


//
//  DNS Text string limited by size representable
//      in a single byte length field

#define DNS_MAX_TEXT_STRING_LENGTH  (255)




//
//  DNS On-The-Wire Structures
//

#pragma pack(push, 1)

//
//  Byte flip DNS header to\from host order.
//
//  Note that this does NOT flip flags, as definition above defines
//  flags as individual bytes for direct access to net byte order.
//

#define DNS_BYTE_FLIP_HEADER_COUNTS(pHeader)       \
        {                                   \
            PDNS_HEADER _head = (pHeader);  \
            INLINE_HTONS(_head->Xid,            _head->Xid             ); \
            INLINE_HTONS(_head->QuestionCount,  _head->QuestionCount   ); \
            INLINE_HTONS(_head->AnswerCount,    _head->AnswerCount     ); \
            INLINE_HTONS(_head->NameServerCount,_head->NameServerCount ); \
            INLINE_HTONS(_head->AdditionalCount,_head->AdditionalCount ); \
        }

//
//  Question name follows header
//

#define DNS_OFFSET_TO_QUESTION_NAME     sizeof(DNS_HEADER)

//
//  Question immediately follows header so compressed question name
//      0xC000 | sizeof(DNS_HEADER)

#define DNS_COMPRESSED_QUESTION_NAME  (0xC00C)


//
//  Packet extraction macros
//

#define DNS_QUESTION_NAME_FROM_HEADER( _pHeader_ ) \
            ( (PCHAR)( (PDNS_HEADER)(_pHeader_) + 1 ) )

#define DNS_ANSWER_FROM_QUESTION( _pQuestion_ ) \
            ( (PCHAR)( (PDNS_QUESTION)(_pQuestion_) + 1 ) )


//
//  DNS Question
//

typedef struct _DNS_WIRE_QUESTION
{
    //  Preceded by question name

    WORD    QuestionType;
    WORD    QuestionClass;
}
DNS_WIRE_QUESTION, *PDNS_WIRE_QUESTION;


//
//  DNS Resource Record
//

typedef struct _DNS_WIRE_RECORD
{
    //  Preceded by record owner name

    WORD    RecordType;
    WORD    RecordClass;
    DWORD   TimeToLive;
    WORD    DataLength;

    //  Followed by record data
}
DNS_WIRE_RECORD, *PDNS_WIRE_RECORD;

#pragma pack(pop)


//
//  DNS Query Types
//

#define DNS_OPCODE_QUERY            0   // Query
#define DNS_OPCODE_IQUERY           1   // Obsolete: IP to name
#define DNS_OPCODE_SERVER_STATUS    2   // Obsolete: DNS ping
#define DNS_OPCODE_UNKNOWN          3   // Unknown
#define DNS_OPCODE_NOTIFY           4   // Notify
#define DNS_OPCODE_UPDATE           5   // Dynamic Update

//
//  DNS response codes.
//
//  Sent in the "ResponseCode" field of a DNS_HEADER.
//

#define DNS_RCODE_NOERROR       0
#define DNS_RCODE_FORMERR       1       // Format error
#define DNS_RCODE_SERVFAIL      2       // Server failure
#define DNS_RCODE_NXDOMAIN      3       // Name error
#define DNS_RCODE_NOTIMPL       4       // Not implemented
#define DNS_RCODE_REFUSED       5       // Refused
#define DNS_RCODE_YXDOMAIN      6       // Domain name should not exist
#define DNS_RCODE_YXRRSET       7       // RR set should not exist
#define DNS_RCODE_NXRRSET       8       // RR set does not exist
#define DNS_RCODE_NOTAUTH       9       // Not authoritative for zone
#define DNS_RCODE_NOTZONE       10      // Name is not zone
#define DNS_RCODE_MAX           15

//
//  Extended RCODEs
//

#define DNS_RCODE_BADVERS       16      // Bad EDNS version
#define DNS_RCODE_BADSIG        16      // Bad signature
#define DNS_RCODE_BADKEY        17      // Bad key
#define DNS_RCODE_BADTIME       18      // Bad timestamp

//
//  Mappings to friendly names
//

#define DNS_RCODE_NO_ERROR          DNS_RCODE_NOERROR
#define DNS_RCODE_FORMAT_ERROR      DNS_RCODE_FORMERR
#define DNS_RCODE_SERVER_FAILURE    DNS_RCODE_SERVFAIL
#define DNS_RCODE_NAME_ERROR        DNS_RCODE_NXDOMAIN
#define DNS_RCODE_NOT_IMPLEMENTED   DNS_RCODE_NOTIMPL


//
//  DNS Classes
//
//  Classes are on the wire as WORDs.
//
//  _CLASS_ defines in host order.
//  _RCLASS_ defines in net byte order.
//
//  Generally we'll avoid byte flip and test class in net byte order.
//

#define DNS_CLASS_INTERNET  0x0001      //  1
#define DNS_CLASS_CSNET     0x0002      //  2
#define DNS_CLASS_CHAOS     0x0003      //  3
#define DNS_CLASS_HESIOD    0x0004      //  4
#define DNS_CLASS_NONE      0x00fe      //  254
#define DNS_CLASS_ALL       0x00ff      //  255
#define DNS_CLASS_ANY       0x00ff      //  255
#define DNS_CLASS_UNICAST_RESPONSE   0x8000  // Set the top-bit of the field to one

#define DNS_RCLASS_INTERNET 0x0100      //  1
#define DNS_RCLASS_CSNET    0x0200      //  2
#define DNS_RCLASS_CHAOS    0x0300      //  3
#define DNS_RCLASS_HESIOD   0x0400      //  4
#define DNS_RCLASS_NONE     0xfe00      //  254
#define DNS_RCLASS_ALL      0xff00      //  255
#define DNS_RCLASS_ANY      0xff00      //  255
#define DNS_RCLASS_UNICAST_RESPONSE   0x0080  // Set the top-bit of the field to one, in net order!
#define DNS_RCLASS_MDNS_CACHE_FLUSH   0x0080  // mDNS cache flush bit set on record announcement in net order!

//
//  WINS + NBSTAT flag field
//

#define DNS_WINS_FLAG_SCOPE     (0x80000000)
#define DNS_WINS_FLAG_LOCAL     (0x00010000)


//
//  Helpful checks
//

#define IS_WORD_ALIGNED(p)      ( !((UINT_PTR)(p) & (UINT_PTR)1) )
#define IS_DWORD_ALIGNED(p)     ( !((UINT_PTR)(p) & (UINT_PTR)3) )
#define IS_QWORD_ALIGNED(p)     ( !((UINT_PTR)(p) & (UINT_PTR)7) )


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)


//
//  DNS config API
//

//
//  Types of DNS configuration info
//

typedef enum
{
    //  In Win2K
    DnsConfigPrimaryDomainName_W,
    DnsConfigPrimaryDomainName_A,
    DnsConfigPrimaryDomainName_UTF8,

    //  Not available yet
    DnsConfigAdapterDomainName_W,
    DnsConfigAdapterDomainName_A,
    DnsConfigAdapterDomainName_UTF8,

    //  In Win2K
    DnsConfigDnsServerList,

    //  Not available yet
    DnsConfigSearchList,
    DnsConfigAdapterInfo,

    //  In Win2K
    DnsConfigPrimaryHostNameRegistrationEnabled,
    DnsConfigAdapterHostNameRegistrationEnabled,
    DnsConfigAddressRegistrationMaxCount,

    //  In WindowsXP
    DnsConfigHostName_W,
    DnsConfigHostName_A,
    DnsConfigHostName_UTF8,
    DnsConfigFullHostName_W,
    DnsConfigFullHostName_A,
    DnsConfigFullHostName_UTF8,

    DnsConfigNameServer
}
DNS_CONFIG_TYPE;

//
//  Config API flags
//

//
//  DNS_CONFIG_FLAG_ALLOC -- Causes config info to be allocated.
//      Free with LocalFree().
//
#define DNS_CONFIG_FLAG_ALLOC   (0x00000001)

DNS_STATUS
WINAPI
DnsQueryConfig(
    _In_                                      DNS_CONFIG_TYPE     Config,
    _In_                                      DWORD               Flag,
    _In_opt_                                  PCWSTR              pwsAdapterName,
    _In_opt_                                  PVOID               pReserved,
    _Out_writes_bytes_to_opt_(*pBufLen, *pBufLen) PVOID               pBuffer,
    _Inout_                                   PDWORD              pBufLen
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)


//
//  Resource record set building
//
//  pFirst points to first record in list.
//  pLast points to last record in list.
//

typedef struct _DnsRRSet
{
    PDNS_RECORD     pFirstRR;
    PDNS_RECORD     pLastRR;
}
DNS_RRSET, *PDNS_RRSET;


//
//  To init pFirst is NULL.
//  But pLast points at the location of the pFirst pointer -- essentially
//  treating the pFirst ptr as a DNS_RECORD.  (It is a DNS_RECORD with
//  only a pNext field, but that's the only part we use.)
//
//  Then when the first record is added to the list, the pNext field of
//  this dummy record (which corresponds to pFirst's value) is set to
//  point at the first record.  So pFirst then properly points at the
//  first record.
//
//  (This works only because pNext is the first field in a
//  DNS_RECORD structure and hence casting a PDNS_RECORD ptr to
//  PDNS_RECORD* and dereferencing yields its pNext field)
//
//  Use TERMINATE when have built RR set by grabbing records out of
//  existing set.   This makes sure that at the end, the last RR is
//  properly NULL terminated.
//

#define DNS_RRSET_INIT( rrset )                 \
        {                                       \
            PDNS_RRSET  _prrset = &(rrset);     \
            _prrset->pFirstRR = NULL;           \
            _prrset->pLastRR = (PDNS_RECORD) &_prrset->pFirstRR; \
        }

#define DNS_RRSET_ADD( rrset, pnewRR )          \
        {                                       \
            PDNS_RRSET  _prrset = &(rrset);     \
            PDNS_RECORD _prrnew = (pnewRR);     \
            _prrset->pLastRR->pNext = _prrnew;  \
            _prrset->pLastRR = _prrnew;         \
        }

#define DNS_RRSET_TERMINATE( rrset )            \
        {                                       \
            PDNS_RRSET  _prrset = &(rrset);     \
            _prrset->pLastRR->pNext = NULL;     \
        }


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)


typedef
VOID
(WINAPI *DNS_PROXY_COMPLETION_ROUTINE) (
    _In_opt_ void *completionContext,
    _In_ DNS_STATUS status);


typedef enum DNS_PROXY_INFORMATION_TYPE {
                DNS_PROXY_INFORMATION_DIRECT,
                DNS_PROXY_INFORMATION_DEFAULT_SETTINGS,
                DNS_PROXY_INFORMATION_PROXY_NAME,
                DNS_PROXY_INFORMATION_DOES_NOT_EXIST
}   DNS_PROXY_INFORMATION_TYPE;

typedef struct DNS_PROXY_INFORMATION {
                _In_ ULONG version;  // Current version is 1
                _Out_ DNS_PROXY_INFORMATION_TYPE proxyInformationType;
                _Out_opt_ PWSTR proxyName;
} DNS_PROXY_INFORMATION;


//
//  Record set manipulation
//

//
//  Record Copy
//  Record copy functions also do conversion between character sets.
//
//  Note, it might be advisable to directly expose non-Ex copy
//  functions _W, _A for record and set, to avoid exposing the
//  conversion enum.
//

PDNS_RECORD
WINAPI
DnsRecordCopyEx(
    _In_    PDNS_RECORD     pRecord,
    _In_    DNS_CHARSET     CharSetIn,
    _In_    DNS_CHARSET     CharSetOut
    );

PDNS_RECORD
WINAPI
DnsRecordSetCopyEx(
    _In_    PDNS_RECORD     pRecordSet,
    _In_    DNS_CHARSET     CharSetIn,
    _In_    DNS_CHARSET     CharSetOut
    );

#ifdef UNICODE
#define DnsRecordCopy(pRR)  \
        DnsRecordCopyEx( (pRR), DnsCharSetUnicode, DnsCharSetUnicode )
#define DnsRecordSetCopy(pRR)  \
        DnsRecordSetCopyEx( (pRR), DnsCharSetUnicode, DnsCharSetUnicode )
#else
#define DnsRecordCopy(pRR)  \
        DnsRecordCopyEx( (pRR), DnsCharSetAnsi, DnsCharSetAnsi )
#define DnsRecordSetCopy(pRR)  \
        DnsRecordSetCopyEx( (pRR), DnsCharSetAnsi, DnsCharSetAnsi )
#endif


//
//  Record Compare
//
//  Note:  these routines only compare records of the SAME character set.
//  (ANSI, unicode or UTF8).  Furthermore the routines assume the character
//  set is indicated within the record.  If compare of user created, rather
//  than DNS API created record lists is desired, then caller should use
//  DnsRecordCopy API and compare copies.
//

BOOL
WINAPI
DnsRecordCompare(
    _In_            PDNS_RECORD     pRecord1,
    _In_            PDNS_RECORD     pRecord2
    );

BOOL
WINAPI
DnsRecordSetCompare(
    _Inout_                 PDNS_RECORD     pRR1,
    _Inout_                 PDNS_RECORD     pRR2,
    _Outptr_opt_result_maybenull_     PDNS_RECORD *   ppDiff1,
    _Outptr_opt_result_maybenull_     PDNS_RECORD *   ppDiff2
    );

//
//  Detach next record set from record list
//

PDNS_RECORD
DnsRecordSetDetach(
    _Inout_         PDNS_RECORD     pRecordList
    );


//
//  Backward compatibility with Win2K, do not use for XP+ applications
//
//  To free record lists, code
//      DnsFree( pRecordList, DnsFreeRecordList );
//

#define DnsFreeRecordListDeep   DnsFreeRecordList


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

//
//  Free structures returned from dnsapi.dll
//
//  Currently supported free structures:
//      Flat -- flat structure, including those allocated by DnsQueryConfig()
//      RecordList -- deep record list free, including sub-fields of DNS_RECORD;
//          includes those returned by DnsQuery() or DnsRecordSetCopy()
//

typedef enum
{
    DnsFreeFlat = 0,
    DnsFreeRecordList,
    DnsFreeParsedMessageFields
}
DNS_FREE_TYPE;

VOID
WINAPI
DnsFree(
    _Pre_opt_valid_ _Frees_ptr_opt_    PVOID    pData,
    _In_        DNS_FREE_TYPE   FreeType
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

#if(_WIN32_WINNT >= 0x0501)
#define DnsRecordListFree(p,t)  DnsFree(p,DnsFreeRecordList)
#else
VOID
WINAPI
DnsRecordListFree(
    _Inout_opt_ PDNS_RECORD     pRecordList,
    _In_        DNS_FREE_TYPE   FreeType
    );
#endif /* _WIN32_WINNT >= 0x0501 */

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

//
//  Sets pfFlat to FALSE if a record has been parsed into its corresponding struct format and
//  TRUE if it has been flat read (i.e. just a data buffer), ullFlags is currently unused and
//  exists for forwards compatibility
//

DNS_STATUS
WINAPI
DnsIsFlatRecord(
    _In_    PDNS_RECORD    pRecord,
    _In_    ULONG64        ullFlags,
    _Out_   BOOL           *pfFlat
    );

//
//  DNS Query API
//

//
//  Options for DnsQuery
//

#define DNS_QUERY_STANDARD                  0x00000000
#define DNS_QUERY_ACCEPT_TRUNCATED_RESPONSE 0x00000001
#define DNS_QUERY_USE_TCP_ONLY              0x00000002
#define DNS_QUERY_NO_RECURSION              0x00000004
#define DNS_QUERY_BYPASS_CACHE              0x00000008
#define DNS_QUERY_NO_WIRE_QUERY             0x00000010
#define DNS_QUERY_NO_LOCAL_NAME             0x00000020
#define DNS_QUERY_NO_HOSTS_FILE             0x00000040
#define DNS_QUERY_NO_NETBT                  0x00000080
#define DNS_QUERY_WIRE_ONLY                 0x00000100
#define DNS_QUERY_RETURN_MESSAGE            0x00000200
#define DNS_QUERY_MULTICAST_ONLY            0x00000400
#define DNS_QUERY_NO_MULTICAST              0x00000800
#define DNS_QUERY_TREAT_AS_FQDN             0x00001000
#define DNS_QUERY_ADDRCONFIG                0x00002000
#define DNS_QUERY_DUAL_ADDR                 0x00004000
#define DNS_QUERY_DONT_RESET_TTL_VALUES     0x00100000
#define DNS_QUERY_DISABLE_IDN_ENCODING      0x00200000
#define DNS_QUERY_APPEND_MULTILABEL         0x00800000
#define DNS_QUERY_DNSSEC_OK                 0x01000000  // Sets DNSSEC OK (DO) bit in query
#define DNS_QUERY_DNSSEC_CHECKING_DISABLED  0x02000000  // Sets DNSSEC checking disabled (CD) bit in query
#define DNS_QUERY_DNSSEC_REQUIRED           0x04000000  // Sets DNSSEC OK (DO) bit in query AND requires that response contains authenticated data (AD) bit set
#define DNS_QUERY_RESERVED                  0xf0000000

//  Backward compatibility with Win2K
//  Do not use

#define DNS_QUERY_CACHE_ONLY                DNS_QUERY_NO_WIRE_QUERY

//
//  When DNS_QUERY_PARSE_ALL_RECORDS is NOT set: the following records will be parsed:
//  DNS_TYPE_A
//  DNS_TYPE_NS
//  DNS_TYPE_MD
//  DNS_TYPE_MF
//  DNS_TYPE_CNAME
//  DNS_TYPE_SOA
//  DNS_TYPE_MB
//  DNS_TYPE_MG
//  DNS_TYPE_MR
//  DNS_TYPE_WKS
//  DNS_TYPE_PTR
//  DNS_TYPE_HINFO
//  DNS_TYPE_MINFO
//  DNS_TYPE_MX
//  DNS_TYPE_TEXT
//  DNS_TYPE_RP
//  DNS_TYPE_AFSDB
//  DNS_TYPE_X25
//  DNS_TYPE_ISDN
//  DNS_TYPE_RT
//  DNS_TYPE_SIG
//  DNS_TYPE_KEY
//  DNS_TYPE_AAAA
//  DNS_TYPE_SRV
//  DNS_TYPE_ATMA
//  DNS_TYPE_NAPTR
//  DNS_TYPE_DNAME
//  DNS_TYPE_OPT
//  DNS_TYPE_DS
//  DNS_TYPE_RRSIG
//  DNS_TYPE_NSEC
//  DNS_TYPE_DNSKEY
//  DNS_TYPE_DHCID
//  DNS_TYPE_NSEC3
//  DNS_TYPE_NSEC3PARAM
//  DNS_TYPE_TLSA
//  DNS_TYPE_TKEY
//  DNS_TYPE_TSIG
//  DNS_TYPE_WINS
//  DNS_TYPE_WINSR
//  All other record types will be returned in flat format (so long as they are flat read compatible)
//  and it is the caller's responsibility to parse them if needed
//  NOTE: to get any other record types back in a parsed format (where available) one must set DNS_QUERY_PARSE_ALL_RECORDS
//

//
//  When DNS_QUERY_PARSE_ALL_RECORDS is set: ONLY parsed records will be returned (unknown records which cannot be parsed will not be returned)
//  NOTE: for backwards compatiblity one MUST set this flag to parse DNS_TYPE_SVCB/DNS_TYPE_HTTPS, or any new record types defined in the future
//

#define DNS_QUERY_PARSE_ALL_RECORDS         0x0400000000000000



DNS_STATUS
WINAPI
DnsQuery_A(
    _In_                PCSTR           pszName,
    _In_                WORD            wType,
    _In_                DWORD           Options,
    _Inout_opt_         PVOID           pExtra,
    _Outptr_result_maybenull_     PDNS_RECORD *   ppQueryResults,
    _Outptr_opt_result_maybenull_ PVOID *         pReserved
    );

DNS_STATUS
WINAPI
DnsQuery_UTF8(
    _In_                PCSTR           pszName,
    _In_                WORD            wType,
    _In_                DWORD           Options,
    _Inout_opt_         PVOID           pExtra,
    _Outptr_result_maybenull_     PDNS_RECORD *   ppQueryResults,
    _Outptr_opt_result_maybenull_ PVOID *         pReserved
    );

DNS_STATUS
WINAPI
DnsQuery_W(
    _In_                PCWSTR          pszName,
    _In_                WORD            wType,
    _In_                DWORD           Options,
    _Inout_opt_         PVOID           pExtra,
    _Outptr_result_maybenull_     PDNS_RECORD *   ppQueryResults,
    _Outptr_opt_result_maybenull_ PVOID *         pReserved
    );

#ifdef UNICODE
#define DnsQuery DnsQuery_W
#else
#define DnsQuery DnsQuery_A
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)


//
//  DnsQueryEx
//

#if !defined ( USE_PRIVATE_DNS_ADDR ) || defined (MIDL_PASS)
#define DNS_QUERY_REQUEST_VERSION1  0x1
#define DNS_QUERY_REQUEST_VERSION2  0x2
#endif

#define DNS_QUERY_RESULTS_VERSION1  0x1

typedef struct _DNS_QUERY_RESULT
{
    _In_        ULONG           Version;
    _Out_       DNS_STATUS      QueryStatus;
    _Out_       ULONG64         QueryOptions;
    _Out_       PDNS_RECORD     pQueryRecords;
    _Inout_opt_ PVOID           Reserved;
}
DNS_QUERY_RESULT, *PDNS_QUERY_RESULT;

typedef
VOID
WINAPI
DNS_QUERY_COMPLETION_ROUTINE(
    _In_        PVOID               pQueryContext,
    _Inout_     PDNS_QUERY_RESULT   pQueryResults
);

typedef DNS_QUERY_COMPLETION_ROUTINE *PDNS_QUERY_COMPLETION_ROUTINE;


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#define DNS_APP_SETTINGS_VERSION1 0x1

#define DNS_APP_SETTINGS_EXCLUSIVE_SERVERS 0x1

typedef struct _DNS_APPLICATION_SETTINGS
{
    ULONG Version;
    ULONG64 Flags;
} DNS_APPLICATION_SETTINGS;

VOID
DnsFreeCustomServers(
    _Inout_  DWORD               *pcServers,
    _Inout_  DNS_CUSTOM_SERVER   **ppServers
    );

DWORD
DnsGetApplicationSettings(
    _Out_                              DWORD                    *pcServers,
    _Outptr_result_buffer_(*pcServers) DNS_CUSTOM_SERVER        **ppDefaultServers,
    _Out_opt_                          DNS_APPLICATION_SETTINGS *pSettings
    );

DWORD
DnsSetApplicationSettings(
    _In_                 DWORD                           cServers,
    _In_reads_(cServers) const DNS_CUSTOM_SERVER         *pServers,
    _In_opt_             const DNS_APPLICATION_SETTINGS  *pSettings
    );

#if !defined ( USE_PRIVATE_DNS_ADDR ) || defined (MIDL_PASS)

typedef struct _DNS_QUERY_REQUEST
{
    _In_        ULONG           Version;
    _In_opt_    PCWSTR          QueryName;
    _In_        WORD            QueryType;
    _In_        ULONG64         QueryOptions;
    _In_opt_    PDNS_ADDR_ARRAY pDnsServerList;
    _In_opt_    ULONG           InterfaceIndex;
    _In_opt_    PDNS_QUERY_COMPLETION_ROUTINE   pQueryCompletionCallback;
    _In_        PVOID           pQueryContext;
}
DNS_QUERY_REQUEST, *PDNS_QUERY_REQUEST;

typedef struct DECLSPEC_ALIGN(8) _DNS_QUERY_CANCEL
{
                CHAR            Reserved[32];
}
DNS_QUERY_CANCEL, *PDNS_QUERY_CANCEL;

DNS_STATUS
WINAPI
DnsQueryEx(
    _In_        PDNS_QUERY_REQUEST  pQueryRequest,
    _Inout_     PDNS_QUERY_RESULT   pQueryResults,
    _Inout_opt_ PDNS_QUERY_CANCEL   pCancelHandle
    );

DNS_STATUS
WINAPI
DnsCancelQuery(
    _In_        PDNS_QUERY_CANCEL    pCancelHandle
    );

#define DNS_QUERY_REQUEST_VERSION3  0x3

typedef struct _DNS_QUERY_REQUEST3
{
    ULONG           Version;
    PCWSTR          QueryName;
    WORD            QueryType;
    ULONG64         QueryOptions;
    PDNS_ADDR_ARRAY pDnsServerList;
    ULONG           InterfaceIndex;
    PDNS_QUERY_COMPLETION_ROUTINE   pQueryCompletionCallback;
    PVOID           pQueryContext;
    BOOL            IsNetworkQueryRequired;
    DWORD           RequiredNetworkIndex;
    DWORD           cCustomServers;

#ifdef MIDL_PASS
    [size_is(cCustomServers)]
#endif
    _Field_size_(cCustomServers)
    DNS_CUSTOM_SERVER *pCustomServers;
}
DNS_QUERY_REQUEST3, *PDNS_QUERY_REQUEST3;

#endif // !defined ( USE_PRIVATE_DNS_ADDR ) || defined (MIDL_PASS)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

//
//  DnsQueryRaw
//

#define DNS_PROTOCOL_UNSPECIFIED    0
#define DNS_PROTOCOL_UDP            1
#define DNS_PROTOCOL_TCP            2
#define DNS_PROTOCOL_DOH            3
#define DNS_PROTOCOL_DOT            4
#define DNS_PROTOCOL_NO_WIRE        5

#define DNS_QUERY_RAW_RESULTS_VERSION1  0x1

#pragma warning(push)
#pragma warning(disable: 4201) // nameless struct/unions

typedef struct _DNS_QUERY_RAW_RESULT
{
    ULONG                                           version;
    DNS_STATUS                                      queryStatus;
    ULONG64                                         queryOptions;
    ULONG64                                         queryRawOptions;
    ULONG64                                         responseFlags;
    ULONG                                           queryRawResponseSize;
#ifdef MIDL_PASS
    [size_is(queryRawResponseSize)]
#endif
    _Field_size_bytes_(queryRawResponseSize) BYTE   *queryRawResponse;
    PDNS_RECORD                                     queryRecords;
    ULONG                                           protocol;

    union
    {
#if !defined (MIDL_PASS) && defined (_WS2TCPIP_H_)
        SOCKADDR_INET                               sourceAddr;
#endif
        CHAR                                        maxSa[DNS_ADDR_MAX_SOCKADDR_LENGTH];
    };
}
DNS_QUERY_RAW_RESULT;

#pragma warning(pop)

VOID
WINAPI
DnsQueryRawResultFree(
    _Frees_ptr_opt_ DNS_QUERY_RAW_RESULT *queryResults
);

typedef
VOID
(CALLBACK *DNS_QUERY_RAW_COMPLETION_ROUTINE)(
    _In_        VOID                   *queryContext,
    _In_        DNS_QUERY_RAW_RESULT   *queryResults
);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#define DNS_QUERY_RAW_REQUEST_VERSION1  0x1

#pragma warning(push)
#pragma warning(disable: 4201) // nameless struct/unions

typedef struct _DNS_QUERY_RAW_REQUEST
{
    ULONG                                               version;
    ULONG                                               resultsVersion;
    ULONG                                               dnsQueryRawSize;
#ifdef MIDL_PASS
    [size_is(dnsQueryRawSize)]
#endif
    _Field_size_bytes_(dnsQueryRawSize) BYTE            *dnsQueryRaw;
    PWSTR                                               dnsQueryName;
    USHORT                                              dnsQueryType;
    ULONG64                                             queryOptions;
    ULONG                                               interfaceIndex;
    DNS_QUERY_RAW_COMPLETION_ROUTINE                    queryCompletionCallback;
    VOID                                                *queryContext;
    ULONG64                                             queryRawOptions;
    ULONG                                               customServersSize;
#ifdef MIDL_PASS
    [size_is(customServersSize)]
#endif
    _Field_size_(customServersSize) DNS_CUSTOM_SERVER   *customServers;
    ULONG                                               protocol;

    union
    {
#if !defined(MIDL_PASS) && defined(_WS2TCPIP_H_)
        SOCKADDR_INET                                   sourceAddr;
#endif
        CHAR                                            maxSa[DNS_ADDR_MAX_SOCKADDR_LENGTH];
    };
}
DNS_QUERY_RAW_REQUEST;

#pragma warning(pop)

#define DNS_QUERY_RAW_OPTION_BEST_EFFORT_PARSE          0x0000000000000001

typedef struct DECLSPEC_ALIGN(8) _DNS_QUERY_RAW_CANCEL
{
    CHAR            reserved[32];
}
DNS_QUERY_RAW_CANCEL;

DNS_STATUS
WINAPI
DnsQueryRaw(
    _In_        DNS_QUERY_RAW_REQUEST   *queryRequest,
    _Inout_     DNS_QUERY_RAW_CANCEL    *cancelHandle
);

DNS_STATUS
WINAPI
DnsCancelQueryRaw(
    _In_        DNS_QUERY_RAW_CANCEL    *cancelHandle
);

//
//  DNS Update API
//
//      DnsAcquireContextHandle
//      DnsReleaseContextHandle
//      DnsModifyRecordsInSet
//      DnsReplaceRecordSet
//

//
//  Update flags
//

#define DNS_UPDATE_SECURITY_USE_DEFAULT     0x00000000
#define DNS_UPDATE_SECURITY_OFF             0x00000010
#define DNS_UPDATE_SECURITY_ON              0x00000020
#define DNS_UPDATE_SECURITY_ONLY            0x00000100
#define DNS_UPDATE_CACHE_SECURITY_CONTEXT   0x00000200
#define DNS_UPDATE_TEST_USE_LOCAL_SYS_ACCT  0x00000400
#define DNS_UPDATE_FORCE_SECURITY_NEGO      0x00000800
#define DNS_UPDATE_TRY_ALL_MASTER_SERVERS   0x00001000
#define DNS_UPDATE_SKIP_NO_UPDATE_ADAPTERS  0x00002000
#define DNS_UPDATE_REMOTE_SERVER            0x00004000
#define DNS_UPDATE_RESERVED                 0xffff0000


//
//  Note:  pCredentials paramater is currently respectively
//  PSEC_WINNT_AUTH_IDENTITY_W or PSEC_WINNT_AUTH_IDENTITY_A.
//  Using PVOID to obviate the need for including rpcdce.h
//  in order to include this file and to leave open the
//  possibility of alternative credential specifications in
//  the future.
//

DNS_STATUS
WINAPI
DnsAcquireContextHandle_W(
    _In_            DWORD           CredentialFlags,
    _In_opt_        PVOID           Credentials,
    _Outptr_     PHANDLE         pContext
    );

DNS_STATUS
WINAPI
DnsAcquireContextHandle_A(
    _In_            DWORD           CredentialFlags,
    _In_opt_        PVOID           Credentials,
    _Outptr_     PHANDLE         pContext
    );

#ifdef UNICODE
#define DnsAcquireContextHandle DnsAcquireContextHandle_W
#else
#define DnsAcquireContextHandle DnsAcquireContextHandle_A
#endif

VOID
WINAPI
DnsReleaseContextHandle(
    _In_        HANDLE          hContext
    );

//
//  Dynamic Update API
//

DNS_STATUS
WINAPI
DnsModifyRecordsInSet_W(
    _In_opt_        PDNS_RECORD     pAddRecords,
    _In_opt_        PDNS_RECORD     pDeleteRecords,
    _In_            DWORD           Options,
    _In_opt_        HANDLE          hCredentials,
    _Inout_opt_     PVOID           pExtraList,
    _Inout_opt_     PVOID           pReserved
    );

DNS_STATUS
WINAPI
DnsModifyRecordsInSet_A(
    _In_opt_        PDNS_RECORD     pAddRecords,
    _In_opt_        PDNS_RECORD     pDeleteRecords,
    _In_            DWORD           Options,
    _In_opt_        HANDLE          hCredentials,
    _Inout_opt_     PVOID           pExtraList,
    _Inout_opt_     PVOID           pReserved
    );

DNS_STATUS
WINAPI
DnsModifyRecordsInSet_UTF8(
    _In_opt_        PDNS_RECORD     pAddRecords,
    _In_opt_        PDNS_RECORD     pDeleteRecords,
    _In_            DWORD           Options,
    _In_opt_        HANDLE          hCredentials,
    _Inout_opt_     PVOID           pExtraList,
    _Inout_opt_     PVOID           pReserved
    );

#ifdef UNICODE
#define DnsModifyRecordsInSet  DnsModifyRecordsInSet_W
#else
#define DnsModifyRecordsInSet  DnsModifyRecordsInSet_A
#endif


DNS_STATUS
WINAPI
DnsReplaceRecordSetW(
    _In_            PDNS_RECORD     pReplaceSet,
    _In_            DWORD           Options,
    _In_opt_        HANDLE          hContext,
    _Inout_opt_     PVOID           pExtraInfo,
    _Inout_opt_     PVOID           pReserved
    );

DNS_STATUS
WINAPI
DnsReplaceRecordSetA(
    _In_            PDNS_RECORD     pReplaceSet,
    _In_            DWORD           Options,
    _In_opt_        HANDLE          hContext,
    _Inout_opt_     PVOID           pExtraInfo,
    _Inout_opt_     PVOID           pReserved
    );

DNS_STATUS
WINAPI
DnsReplaceRecordSetUTF8(
    _In_            PDNS_RECORD     pReplaceSet,
    _In_            DWORD           Options,
    _In_opt_        HANDLE          hContext,
    _Inout_opt_     PVOID           pExtraInfo,
    _Inout_opt_     PVOID           pReserved
    );

#ifdef UNICODE
#define DnsReplaceRecordSet  DnsReplaceRecordSetW
#else
#define DnsReplaceRecordSet  DnsReplaceRecordSetA
#endif



//
//  DNS name validation
//

typedef enum _DNS_NAME_FORMAT
{
    DnsNameDomain,
    DnsNameDomainLabel,
    DnsNameHostnameFull,
    DnsNameHostnameLabel,
    DnsNameWildcard,
    DnsNameSrvRecord,
    DnsNameValidateTld
}
DNS_NAME_FORMAT;


DNS_STATUS
DnsValidateName_W(
    _In_    PCWSTR          pszName,
    _In_    DNS_NAME_FORMAT Format
    );

DNS_STATUS
DnsValidateName_A(
    _In_    PCSTR           pszName,
    _In_    DNS_NAME_FORMAT Format
    );

DNS_STATUS
DnsValidateName_UTF8(
    _In_    PCSTR           pszName,
    _In_    DNS_NAME_FORMAT Format
    );

#ifdef UNICODE
#define DnsValidateName(p,f)    DnsValidateName_W( (p), (f) )
#else
#define DnsValidateName(p,f)    DnsValidateName_A( (p), (f) )
#endif

//
// DNS server validation
//

// DNS server validation error codes
#define DNS_VALSVR_ERROR_INVALID_ADDR               0x01
#define DNS_VALSVR_ERROR_INVALID_NAME               0x02
#define DNS_VALSVR_ERROR_UNREACHABLE                0x03
#define DNS_VALSVR_ERROR_NO_RESPONSE                0x04
#define DNS_VALSVR_ERROR_NO_AUTH                    0x05
#define DNS_VALSVR_ERROR_REFUSED                    0x06

#define DNS_VALSVR_ERROR_NO_TCP                     0x10
#define DNS_VALSVR_ERROR_UNKNOWN                    0xFF

// Winsock2.h must be included before windns.h to use the validate server function
#ifdef _WS2DEF_

DNS_STATUS
DnsValidateServerStatus(
    _In_        PSOCKADDR   server,
    _In_opt_    PCWSTR      queryName,
    _Out_       PDWORD      serverStatus
    );

#endif

//
//  DNS name comparison
//

BOOL
WINAPI
DnsNameCompare_A(
    _In_    PCSTR           pName1,
    _In_    PCSTR           pName2
    );

BOOL
WINAPI
DnsNameCompare_W(
    _In_    PCWSTR          pName1,
    _In_    PCWSTR          pName2
    );

#ifdef UNICODE
#define DnsNameCompare(n1,n2)   DnsNameCompare_W( (n1),(n2) )
#else
#define DnsNameCompare(n1,n2)   DnsNameCompare_A( (n1),(n2) )
#endif



//
//  DNS message "roll-your-own" routines
//

BOOL
WINAPI
DnsWriteQuestionToBuffer_W(
    _Inout_     PDNS_MESSAGE_BUFFER pDnsBuffer,
    _Inout_     PDWORD              pdwBufferSize,
    _In_        PCWSTR              pszName,
    _In_        WORD                wType,
    _In_        WORD                Xid,
    _In_        BOOL                fRecursionDesired
    );

BOOL
WINAPI
DnsWriteQuestionToBuffer_UTF8(
    _Inout_     PDNS_MESSAGE_BUFFER pDnsBuffer,
    _Inout_     PDWORD              pdwBufferSize,
    _In_        PCSTR               pszName,
    _In_        WORD                wType,
    _In_        WORD                Xid,
    _In_        BOOL                fRecursionDesired
    );

DNS_STATUS
WINAPI
DnsExtractRecordsFromMessage_W(
    _In_            PDNS_MESSAGE_BUFFER pDnsBuffer,
    _In_            WORD                wMessageLength,
    _Outptr_     PDNS_RECORD *       ppRecord
    );

DNS_STATUS
WINAPI
DnsExtractRecordsFromMessage_UTF8(
    _In_            PDNS_MESSAGE_BUFFER pDnsBuffer,
    _In_            WORD                wMessageLength,
    _Outptr_     PDNS_RECORD *       ppRecord
    );



//
//  DNS name resolution policy table
//

DWORD
WINAPI
DnsGetProxyInformation(
    _In_        PCWSTR                          hostName,
    _Inout_     DNS_PROXY_INFORMATION *         proxyInformation,
    _Inout_opt_ DNS_PROXY_INFORMATION *         defaultProxyInformation,
    _In_opt_    DNS_PROXY_COMPLETION_ROUTINE    completionRoutine,
    _In_opt_    void *                          completionContext
    );

VOID
WINAPI
DnsFreeProxyName(
    _Frees_ptr_opt_ PWSTR   proxyName
    );

//
//  Connections Proxy configuration APIs
//

//
// Connection Proxy APIs.
//

#define DNS_CONNECTION_NAME_MAX_LENGTH                      64
#define DNS_CONNECTION_PROXY_INFO_CURRENT_VERSION           1
#define DNS_CONNECTION_PROXY_INFO_SERVER_MAX_LENGTH         256
#define DNS_CONNECTION_PROXY_INFO_FRIENDLY_NAME_MAX_LENGTH  64
#define DNS_CONNECTION_PROXY_INFO_USERNAME_MAX_LENGTH       128
#define DNS_CONNECTION_PROXY_INFO_PASSWORD_MAX_LENGTH       128
#define DNS_CONNECTION_PROXY_INFO_EXCEPTION_MAX_LENGTH      1024
#define DNS_CONNECTION_PROXY_INFO_EXTRA_INFO_MAX_LENGTH     1024

//
// Proxy type definition.
//

typedef enum _DNS_CONNECTION_PROXY_TYPE
{
    DNS_CONNECTION_PROXY_TYPE_NULL      = 0,
    DNS_CONNECTION_PROXY_TYPE_HTTP      = 1,
    DNS_CONNECTION_PROXY_TYPE_WAP       = 2,
    DNS_CONNECTION_PROXY_TYPE_SOCKS4    = 4,
    DNS_CONNECTION_PROXY_TYPE_SOCKS5    = 5
} DNS_CONNECTION_PROXY_TYPE;

//
// This enum indicate whether the DNS_CONNECTION_PROXY_INFO data structure
// contains the 'CONFIG' information or the 'SCRIPT' information.
//

typedef enum _DNS_CONNECTION_PROXY_INFO_SWITCH
{
    DNS_CONNECTION_PROXY_INFO_SWITCH_CONFIG = 0,
    DNS_CONNECTION_PROXY_INFO_SWITCH_SCRIPT,
    DNS_CONNECTION_PROXY_INFO_SWITCH_WPAD
} DNS_CONNECTION_PROXY_INFO_SWITCH;

//
// These flags can be ORed to form the 'Flags' field of DNS_CONNECTION_PROXY_INFO.
//

#define DNS_CONNECTION_PROXY_INFO_FLAG_DISABLED     0x1
#define DNS_CONNECTION_PROXY_INFO_FLAG_BYPASSLOCAL  0x2

#pragma warning(push)
#pragma warning(disable: 4201)

typedef struct _DNS_CONNECTION_PROXY_INFO
{
    DWORD Version;
    WCHAR *pwszFriendlyName;
    DWORD Flags;
    DNS_CONNECTION_PROXY_INFO_SWITCH Switch;
    union
    {
        struct _DNS_CONNECTION_PROXY_INFO_CONFIG
        {
            WCHAR *pwszServer;
            WCHAR *pwszUsername;
            WCHAR *pwszPassword;
            WCHAR *pwszException;
            WCHAR *pwszExtraInfo;
            WORD Port;
        } Config;

        struct _DNS_CONNECTION_PROXY_INFO_SCRIPT
        {
            WCHAR *pwszScript;
            WCHAR *pwszUsername;
            WCHAR *pwszPassword;
        } Script;
    };
} DNS_CONNECTION_PROXY_INFO, *PDNS_CONNECTION_PROXY_INFO;

#pragma warning(pop)

typedef struct _DNS_CONNECTION_PROXY_INFO_EX
{
    DNS_CONNECTION_PROXY_INFO ProxyInfo;
    DWORD dwInterfaceIndex;
    WCHAR *pwszConnectionName;
    BOOL fDirectConfiguration;
    HANDLE hConnection;
} DNS_CONNECTION_PROXY_INFO_EX, *PDNS_CONNECTION_PROXY_INFO_EX;

typedef struct _DNS_CONNECTION_PROXY_ELEMENT
{
    DNS_CONNECTION_PROXY_TYPE Type;
    DNS_CONNECTION_PROXY_INFO Info;
} DNS_CONNECTION_PROXY_ELEMENT;

typedef struct _DNS_CONNECTION_PROXY_LIST
{
    DWORD cProxies;
    DNS_CONNECTION_PROXY_ELEMENT *pProxies;
} DNS_CONNECTION_PROXY_LIST;

typedef struct _DNS_CONNECTION_NAME
{
    WCHAR wszName[DNS_CONNECTION_NAME_MAX_LENGTH + 1];
} DNS_CONNECTION_NAME;

typedef struct _DNS_CONNECTION_NAME_LIST
{
    DWORD cNames;
    DNS_CONNECTION_NAME *pNames;
} DNS_CONNECTION_NAME_LIST;

DWORD
DnsConnectionGetProxyInfoForHostUrl(
    _In_z_ PCWSTR pwszHostUrl,
    _In_reads_opt_(dwSelectionContextLength) BYTE *pSelectionContext,
    _In_ DWORD dwSelectionContextLength,
    _In_ DWORD dwExplicitInterfaceIndex,
    _Out_ DNS_CONNECTION_PROXY_INFO_EX *pProxyInfoEx
);

DWORD
DnsConnectionGetProxyInfoForHostUrlEx(
    _In_z_ PCWSTR pwszHostUrl,
    _In_reads_opt_(dwSelectionContextLength) BYTE *pSelectionContext,
    _In_ DWORD dwSelectionContextLength,
    _In_ DWORD dwExplicitInterfaceIndex,
    _In_opt_z_ PCWSTR pwszConnectionName,
    _Out_ DNS_CONNECTION_PROXY_INFO_EX *pProxyInfoEx
);

VOID
DnsConnectionFreeProxyInfoEx(
    _Inout_ DNS_CONNECTION_PROXY_INFO_EX *pProxyInfoEx
);

DWORD
DnsConnectionGetProxyInfo(
    _In_z_ PCWSTR pwszConnectionName,
    _In_ DNS_CONNECTION_PROXY_TYPE Type,
    _Out_ DNS_CONNECTION_PROXY_INFO *pProxyInfo
);

VOID
DnsConnectionFreeProxyInfo(
    _Inout_ DNS_CONNECTION_PROXY_INFO *pProxyInfo
);

DWORD
DnsConnectionSetProxyInfo(
    _In_z_ PCWSTR pwszConnectionName,
    _In_ DNS_CONNECTION_PROXY_TYPE Type,
    _In_ const DNS_CONNECTION_PROXY_INFO *pProxyInfo
);

DWORD
DnsConnectionDeleteProxyInfo(
    _In_z_ PCWSTR pwszConnectionName,
    _In_  DNS_CONNECTION_PROXY_TYPE Type
);

DWORD
DnsConnectionGetProxyList(
    _In_z_ PCWSTR pwszConnectionName,
    _Out_ DNS_CONNECTION_PROXY_LIST *pProxyList
);

VOID
DnsConnectionFreeProxyList(
    _Inout_ DNS_CONNECTION_PROXY_LIST *pProxyList
);

DWORD
DnsConnectionGetNameList(
    _Out_ DNS_CONNECTION_NAME_LIST *pNameList
);

VOID
DnsConnectionFreeNameList(
    _Inout_ DNS_CONNECTION_NAME_LIST *pNameList
);

//
// Connection <-> Active Interface Index mapping APIs
//

typedef struct _DNS_CONNECTION_IFINDEX_ENTRY
{
    PCWSTR pwszConnectionName;
    DWORD dwIfIndex;
} DNS_CONNECTION_IFINDEX_ENTRY;

typedef struct _DNS_CONNECTION_IFINDEX_LIST
{
    DNS_CONNECTION_IFINDEX_ENTRY *pConnectionIfIndexEntries;
    DWORD nEntries;
}  DNS_CONNECTION_IFINDEX_LIST;

DWORD
DnsConnectionUpdateIfIndexTable(
    _In_ DNS_CONNECTION_IFINDEX_LIST *pConnectionIfIndexEntries
);

//
// Connection Policy Configuration APIs
//

#define DNS_CONNECTION_POLICY_ENTRY_ONDEMAND 0x00000001

typedef struct _DNS_CONNECTION_POLICY_ENTRY
{
    PCWSTR pwszHost;
    PCWSTR pwszAppId;

    DWORD cbAppSid;
#ifdef MIDL_PASS
    [size_is(cbAppSid)]
#endif
    PBYTE pbAppSid;

    DWORD nConnections;
#ifdef MIDL_PASS
    [size_is(nConnections)]
#endif
    PCWSTR *ppwszConnections;

    DWORD dwPolicyEntryFlags;
} DNS_CONNECTION_POLICY_ENTRY, *PDNS_CONNECTION_POLICY_ENTRY;

typedef struct _DNS_CONNECTION_POLICY_ENTRY_LIST
{
#ifdef MIDL_PASS
    [size_is(nEntries)] DNS_CONNECTION_POLICY_ENTRY *pPolicyEntries;
#else
    DNS_CONNECTION_POLICY_ENTRY *pPolicyEntries;
#endif
    DWORD nEntries;
} DNS_CONNECTION_POLICY_ENTRY_LIST;

typedef enum
{
    TAG_DNS_CONNECTION_POLICY_TAG_DEFAULT = 0,
    TAG_DNS_CONNECTION_POLICY_TAG_CONNECTION_MANAGER,
    TAG_DNS_CONNECTION_POLICY_TAG_WWWPT
} DNS_CONNECTION_POLICY_TAG;

DWORD
DnsConnectionSetPolicyEntries(
    _In_ DNS_CONNECTION_POLICY_TAG PolicyEntryTag,
    _In_ DNS_CONNECTION_POLICY_ENTRY_LIST *pPolicyEntryList
);

DWORD
DnsConnectionDeletePolicyEntries(
    _In_ DNS_CONNECTION_POLICY_TAG PolicyEntryTag
);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)


#ifdef __midl
typedef[string] wchar_t *DNSSD_RPC_STRING;
#endif

typedef struct _DNS_SERVICE_INSTANCE
{
#ifdef __midl
    DNSSD_RPC_STRING    pszInstanceName;
#else
    LPWSTR              pszInstanceName;
#endif
#ifdef __midl
    DNSSD_RPC_STRING    pszHostName;
#else
    LPWSTR              pszHostName;
#endif

    IP4_ADDRESS *ip4Address;
    IP6_ADDRESS *ip6Address;

    WORD wPort;
    WORD wPriority;
    WORD wWeight;

    // Property list
    DWORD dwPropertyCount;
#ifdef __midl
    [size_is(dwPropertyCount)] DNSSD_RPC_STRING *keys;
    [size_is(dwPropertyCount)] DNSSD_RPC_STRING *values;
#else
    PWSTR *keys;
    PWSTR *values;
#endif

    DWORD dwInterfaceIndex;
} DNS_SERVICE_INSTANCE, *PDNS_SERVICE_INSTANCE;

#ifndef __midl

PDNS_SERVICE_INSTANCE
WINAPI
DnsServiceConstructInstance(
    _In_ PCWSTR pServiceName,
    _In_ PCWSTR pHostName,
    _In_opt_ PIP4_ADDRESS pIp4,
    _In_opt_ PIP6_ADDRESS pIp6,
    _In_ WORD wPort,
    _In_ WORD wPriority,
    _In_ WORD wWeight,
    _In_ DWORD dwPropertiesCount,
    _In_reads_(dwPropertiesCount) PCWSTR *keys,
    _In_reads_(dwPropertiesCount) PCWSTR *values
    );

PDNS_SERVICE_INSTANCE
WINAPI
DnsServiceCopyInstance(
    _In_ PDNS_SERVICE_INSTANCE pOrig
    );

VOID
WINAPI
DnsServiceFreeInstance(
    _In_ PDNS_SERVICE_INSTANCE pInstance
    );

//
// Cancellation mechanism
//

typedef struct _DNS_SERVICE_CANCEL{
    PVOID reserved;
} DNS_SERVICE_CANCEL, *PDNS_SERVICE_CANCEL;

//
// Browse
//

typedef
VOID
WINAPI
DNS_SERVICE_BROWSE_CALLBACK(
    _In_    DWORD Status,
    _In_    PVOID pQueryContext,
    _In_    PDNS_RECORD pDnsRecord
    );

typedef DNS_SERVICE_BROWSE_CALLBACK *PDNS_SERVICE_BROWSE_CALLBACK;


#pragma warning(push)
#pragma warning(disable: 4201)

//
// For DNS_QUERY_REQUEST_VERSION2 Version,
// pBrowseCallback needs to be a DNS_QUERY_COMPLETION_ROUTINE callback
//
typedef struct _DNS_SERVICE_BROWSE_REQUEST
{
    ULONG   Version;
    ULONG   InterfaceIndex;
    PCWSTR  QueryName;
    union
    {
        PDNS_SERVICE_BROWSE_CALLBACK pBrowseCallback;
        DNS_QUERY_COMPLETION_ROUTINE *pBrowseCallbackV2;
    };
    PVOID   pQueryContext;
} DNS_SERVICE_BROWSE_REQUEST, *PDNS_SERVICE_BROWSE_REQUEST;

#pragma warning(pop)

DNS_STATUS
WINAPI
DnsServiceBrowse(
    _In_    PDNS_SERVICE_BROWSE_REQUEST    pRequest,
    _Inout_ PDNS_SERVICE_CANCEL            pCancel
    );

DNS_STATUS
WINAPI
DnsServiceBrowseCancel(
    _In_ PDNS_SERVICE_CANCEL pCancelHandle
    );

//
// Resolve
//

typedef
VOID
WINAPI
DNS_SERVICE_RESOLVE_COMPLETE(
    _In_ DWORD Status,
    _In_ PVOID pQueryContext,
    _In_ PDNS_SERVICE_INSTANCE pInstance
    );

typedef DNS_SERVICE_RESOLVE_COMPLETE *PDNS_SERVICE_RESOLVE_COMPLETE;

typedef struct _DNS_SERVICE_RESOLVE_REQUEST{
    ULONG   Version;
    ULONG   InterfaceIndex;
    PWSTR   QueryName;
    PDNS_SERVICE_RESOLVE_COMPLETE pResolveCompletionCallback;
    PVOID pQueryContext;
} DNS_SERVICE_RESOLVE_REQUEST, *PDNS_SERVICE_RESOLVE_REQUEST;

DNS_STATUS
WINAPI
DnsServiceResolve(
    _In_    PDNS_SERVICE_RESOLVE_REQUEST    pRequest,
    _Inout_ PDNS_SERVICE_CANCEL             pCancel
    );

DNS_STATUS
WINAPI
DnsServiceResolveCancel(
    _In_ PDNS_SERVICE_CANCEL pCancelHandle
    );

//
// Register
//

typedef
VOID
WINAPI
DNS_SERVICE_REGISTER_COMPLETE(
    _In_    DWORD Status,
    _In_    PVOID pQueryContext,
    _In_   PDNS_SERVICE_INSTANCE pInstance
    );

typedef DNS_SERVICE_REGISTER_COMPLETE *PDNS_SERVICE_REGISTER_COMPLETE;

typedef struct _DNS_SERVICE_REGISTER_REQUEST{
    ULONG Version;
    ULONG   InterfaceIndex;
    PDNS_SERVICE_INSTANCE pServiceInstance;
    PDNS_SERVICE_REGISTER_COMPLETE pRegisterCompletionCallback;
    PVOID pQueryContext;
    HANDLE hCredentials;
    BOOL unicastEnabled;
} DNS_SERVICE_REGISTER_REQUEST, *PDNS_SERVICE_REGISTER_REQUEST;

DWORD
WINAPI
DnsServiceRegister(
    _In_    PDNS_SERVICE_REGISTER_REQUEST   pRequest,
    _Inout_opt_ PDNS_SERVICE_CANCEL             pCancel
    );

DWORD
WINAPI
DnsServiceDeRegister(
    _In_    PDNS_SERVICE_REGISTER_REQUEST   pRequest,
    _Inout_opt_ PDNS_SERVICE_CANCEL             pCancel
    );

DWORD
WINAPI
DnsServiceRegisterCancel(
    _In_ PDNS_SERVICE_CANCEL pCancelHandle
    );

#endif  // __midl

typedef struct _MDNS_QUERY_HANDLE
{
    WCHAR nameBuf[DNS_MAX_NAME_BUFFER_LENGTH];
    WORD  wType;

    // Internal notification bookkeeping, do not edit
    PVOID pSubscription;
    PVOID pWnfCallbackParams;
    ULONG stateNameData[2];
} MDNS_QUERY_HANDLE, *PMDNS_QUERY_HANDLE;

typedef VOID WINAPI MDNS_QUERY_CALLBACK(
    _In_    PVOID pQueryContext,
    _Inout_ PMDNS_QUERY_HANDLE pQueryHandle,
    _Inout_ PDNS_QUERY_RESULT pQueryResults
    );

typedef MDNS_QUERY_CALLBACK *PMDNS_QUERY_CALLBACK;

typedef struct _MDNS_QUERY_REQUEST
{
    ULONG                   Version;
    ULONG                   ulRefCount;      // Refcount for async query
    PCWSTR                  Query;
    WORD                    QueryType;
    ULONG64                 QueryOptions;
    ULONG                   InterfaceIndex;
    PMDNS_QUERY_CALLBACK    pQueryCallback;
    PVOID                   pQueryContext;
    BOOL                    fAnswerReceived; // Reserved
    ULONG                   ulResendCount;   // Reserved
} MDNS_QUERY_REQUEST, *PMDNS_QUERY_REQUEST;

DNS_STATUS WINAPI DnsStartMulticastQuery(
    _In_        PMDNS_QUERY_REQUEST pQueryRequest,
    _Inout_     PMDNS_QUERY_HANDLE pHandle
    );

DNS_STATUS WINAPI DnsStopMulticastQuery(
    _Inout_     PMDNS_QUERY_HANDLE pHandle
    );

//
// End of mDNS definitions
//


//
// Note: Below API is not yet library linkable and hence need to be loaded using GetProcAddress.
//

#if (NTDDI_VERSION >= NTDDI_WIN11_GE)
BOOL
WINAPI
DnsIsZtEnabled(VOID);
#endif  //NTDDI version check

#ifdef __cplusplus
}
#endif  // __cplusplus

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#endif // _WINDNS_INCLUDED_
