/*++

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    winhttp.h

Abstract:

    Contains manifests, macros, types and prototypes for Windows HTTP Services

--*/

#if !defined(_WINHTTPX_)
#define _WINHTTPX_

#include <winapifamily.h>

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)


/*
 * Set up Structure Packing to be 4 bytes for all winhttp structures
 */

#if defined(_WIN64)
#include <pshpack8.h>
#else
#include <pshpack4.h>
#endif


#if defined(__cplusplus)
extern "C" {
#endif


#if !defined(_WINHTTP_INTERNAL_)
#define WINHTTPAPI DECLSPEC_IMPORT
#else
#define WINHTTPAPI
#endif

#ifndef BOOLAPI
#define BOOLAPI WINHTTPAPI BOOL WINAPI
#endif

//
// types
//

typedef LPVOID HINTERNET;
typedef HINTERNET * LPHINTERNET;

typedef WORD INTERNET_PORT;
typedef INTERNET_PORT * LPINTERNET_PORT;

//
// manifests
//

#define INTERNET_DEFAULT_PORT           0           // use the protocol-specific default
#define INTERNET_DEFAULT_HTTP_PORT      80          //    "     "  HTTP   "
#define INTERNET_DEFAULT_HTTPS_PORT     443         //    "     "  HTTPS  "

// flags for WinHttpOpen():
#define WINHTTP_FLAG_ASYNC              0x10000000  // this session is asynchronous (where supported)
#define WINHTTP_FLAG_SECURE_DEFAULTS    0x30000000  // note that this flag also forces async

// flags for WinHttpOpenRequest():
#define WINHTTP_FLAG_SECURE                0x00800000  // use SSL if applicable (HTTPS)
#define WINHTTP_FLAG_ESCAPE_PERCENT        0x00000004  // if escaping enabled, escape percent as well
#define WINHTTP_FLAG_NULL_CODEPAGE         0x00000008  // assume all symbols are ASCII, use fast convertion
#define WINHTTP_FLAG_ESCAPE_DISABLE        0x00000040  // disable escaping
#define WINHTTP_FLAG_ESCAPE_DISABLE_QUERY  0x00000080  // if escaping enabled escape path part, but do not escape query
#define WINHTTP_FLAG_BYPASS_PROXY_CACHE    0x00000100  // add "pragma: no-cache" request header
#define WINHTTP_FLAG_REFRESH               WINHTTP_FLAG_BYPASS_PROXY_CACHE
#define WINHTTP_FLAG_AUTOMATIC_CHUNKING    0x00000200  // Send request without content-length header or chunked TE


#define SECURITY_FLAG_IGNORE_UNKNOWN_CA         0x00000100
#define SECURITY_FLAG_IGNORE_CERT_WRONG_USAGE   0x00000200
#define SECURITY_FLAG_IGNORE_CERT_CN_INVALID    0x00001000 // bad common name in X509 Cert.
#define SECURITY_FLAG_IGNORE_CERT_DATE_INVALID  0x00002000 // expired X509 Cert.
#define SECURITY_FLAG_IGNORE_ALL_CERT_ERRORS    (SECURITY_FLAG_IGNORE_UNKNOWN_CA        | \
                                                 SECURITY_FLAG_IGNORE_CERT_WRONG_USAGE  | \
                                                 SECURITY_FLAG_IGNORE_CERT_CN_INVALID   | \
                                                 SECURITY_FLAG_IGNORE_CERT_DATE_INVALID)


//
// WINHTTP_ASYNC_RESULT - this structure is returned to the application via
// the callback with WINHTTP_CALLBACK_STATUS_REQUEST_COMPLETE. It is not sufficient to
// just return the result of the async operation. If the API failed then the
// app cannot call GetLastError() because the thread context will be incorrect.
// Both the value returned by the async API and any resultant error code are
// made available. The app need not check dwError if dwResult indicates that
// the API succeeded (in this case dwError will be ERROR_SUCCESS)
//

typedef struct _WINHTTP_ASYNC_RESULT
{
    DWORD_PTR dwResult;  // indicates which async API has encountered an error
    DWORD dwError;       // the error code if the API failed
} WINHTTP_ASYNC_RESULT, *LPWINHTTP_ASYNC_RESULT, *PWINHTTP_ASYNC_RESULT;


//
// HTTP_VERSION_INFO - query or set global HTTP version (1.0 or 1.1)
//

#ifndef _HTTP_VERSION_INFO_
#define _HTTP_VERSION_INFO_

typedef struct _HTTP_VERSION_INFO
{
    DWORD dwMajorVersion;
    DWORD dwMinorVersion;
} HTTP_VERSION_INFO, *LPHTTP_VERSION_INFO, *PHTTP_VERSION_INFO;

#endif


//
// INTERNET_SCHEME - URL scheme type
//

#ifndef _INTERNET_SCHEME_
#define _INTERNET_SCHEME_

typedef int INTERNET_SCHEME, *LPINTERNET_SCHEME;

#define INTERNET_SCHEME_HTTP        (1)
#define INTERNET_SCHEME_HTTPS       (2)
#define INTERNET_SCHEME_FTP         (3)
#define INTERNET_SCHEME_SOCKS       (4)


#endif


//
// URL_COMPONENTS - the constituent parts of an URL. Used in WinHttpCrackUrl()
// and WinHttpCreateUrl()
//
// For WinHttpCrackUrl(), if a pointer field and its corresponding length field
// are both 0 then that component is not returned. If the pointer field is NULL
// but the length field is not zero, then both the pointer and length fields are
// returned if both pointer and corresponding length fields are non-zero then
// the pointer field points to a buffer where the component is copied. The
// component may be un-escaped, depending on dwFlags
//
// For WinHttpCreateUrl(), the pointer fields should be NULL if the component
// is not required. If the corresponding length field is zero then the pointer
// field is the address of a zero-terminated string. If the length field is not
// zero then it is the string length of the corresponding pointer field
//

#pragma warning( disable : 4121 )   // disable alignment warning

#ifndef _URL_COMPONENTS_
#define _URL_COMPONENTS_

typedef struct _WINHTTP_URL_COMPONENTS
{
    DWORD   dwStructSize;       // size of this structure. Used in version check
    LPWSTR  lpszScheme;         // pointer to scheme name
    DWORD   dwSchemeLength;     // length of scheme name
    INTERNET_SCHEME nScheme;    // enumerated scheme type (if known)
    LPWSTR  lpszHostName;       // pointer to host name
    DWORD   dwHostNameLength;   // length of host name
    INTERNET_PORT nPort;        // converted port number
    LPWSTR  lpszUserName;       // pointer to user name
    DWORD   dwUserNameLength;   // length of user name
    LPWSTR  lpszPassword;       // pointer to password
    DWORD   dwPasswordLength;   // length of password
    LPWSTR  lpszUrlPath;        // pointer to URL-path
    DWORD   dwUrlPathLength;    // length of URL-path
    LPWSTR  lpszExtraInfo;      // pointer to extra information (e.g. ?foo or #foo)
    DWORD   dwExtraInfoLength;  // length of extra information
} URL_COMPONENTS, *LPURL_COMPONENTS;

typedef URL_COMPONENTS URL_COMPONENTSW;
typedef LPURL_COMPONENTS LPURL_COMPONENTSW;

#endif

#pragma warning( default : 4121 )   // restore alignment warning

//
// WINHTTP_PROXY_INFO - structure supplied with WINHTTP_OPTION_PROXY to get/
// set proxy information on a WinHttpOpen() handle
//

typedef struct _WINHTTP_PROXY_INFO
{
    DWORD  dwAccessType;      // see WINHTTP_ACCESS_* types below
    LPWSTR lpszProxy;         // proxy server list
    LPWSTR lpszProxyBypass;   // proxy bypass list
} WINHTTP_PROXY_INFO, *LPWINHTTP_PROXY_INFO, *PWINHTTP_PROXY_INFO;

typedef WINHTTP_PROXY_INFO WINHTTP_PROXY_INFOW;
typedef LPWINHTTP_PROXY_INFO LPWINHTTP_PROXY_INFOW;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

typedef struct _WINHTTP_AUTOPROXY_OPTIONS
{
    DWORD   dwFlags;
    DWORD   dwAutoDetectFlags;
    LPCWSTR lpszAutoConfigUrl;
    LPVOID  lpvReserved;
    DWORD   dwReserved;
    BOOL    fAutoLogonIfChallenged;
} WINHTTP_AUTOPROXY_OPTIONS, *PWINHTTP_AUTOPROXY_OPTIONS;

#define WINHTTP_AUTOPROXY_AUTO_DETECT           0x00000001
#define WINHTTP_AUTOPROXY_CONFIG_URL            0x00000002
#define WINHTTP_AUTOPROXY_HOST_KEEPCASE         0x00000004
#define WINHTTP_AUTOPROXY_HOST_LOWERCASE        0x00000008
#define WINHTTP_AUTOPROXY_ALLOW_AUTOCONFIG      0x00000100
#define WINHTTP_AUTOPROXY_ALLOW_STATIC          0x00000200
#define WINHTTP_AUTOPROXY_ALLOW_CM              0x00000400
#define WINHTTP_AUTOPROXY_USE_INTERFACE_CONFIG  0x00000800
#define WINHTTP_AUTOPROXY_RUN_INPROCESS         0x00010000
#define WINHTTP_AUTOPROXY_RUN_OUTPROCESS_ONLY   0x00020000
#define WINHTTP_AUTOPROXY_NO_DIRECTACCESS       0x00040000
#define WINHTTP_AUTOPROXY_NO_CACHE_CLIENT       0x00080000
#define WINHTTP_AUTOPROXY_NO_CACHE_SVC          0x00100000


#define WINHTTP_AUTOPROXY_SORT_RESULTS          0x00400000


//
// Flags for dwAutoDetectFlags
//
#define WINHTTP_AUTO_DETECT_TYPE_DHCP           0x00000001
#define WINHTTP_AUTO_DETECT_TYPE_DNS_A          0x00000002

//
// WINHTTP_PROXY_RESULT - structure containing parsed proxy result,
// see WinHttpGetProxyForUrlEx and WinHttpGetProxyResult, use WinHttpFreeProxyResult to free its members.
//

typedef struct _WINHTTP_PROXY_RESULT_ENTRY
{
    BOOL            fProxy;                // Is this a proxy or DIRECT?
    BOOL            fBypass;               // If DIRECT, is it bypassing a proxy (intranet) or is all traffic DIRECT (internet)
    INTERNET_SCHEME ProxyScheme;           // The scheme of the proxy, SOCKS, HTTP (CERN Proxy), HTTPS (SSL through Proxy)
    PWSTR           pwszProxy;             // Hostname of the proxy.
    INTERNET_PORT   ProxyPort;             // Port of the proxy.
} WINHTTP_PROXY_RESULT_ENTRY;

typedef struct _WINHTTP_PROXY_RESULT
{
    DWORD cEntries;
    WINHTTP_PROXY_RESULT_ENTRY *pEntries;
} WINHTTP_PROXY_RESULT;

typedef struct _WINHTTP_PROXY_RESULT_EX
{
    DWORD cEntries;
    WINHTTP_PROXY_RESULT_ENTRY *pEntries;
    HANDLE hProxyDetectionHandle;
    DWORD dwProxyInterfaceAffinity;
} WINHTTP_PROXY_RESULT_EX;


#define NETWORKING_KEY_BUFSIZE 128

typedef struct _WinHttpProxyNetworkKey
{
    unsigned char pbBuffer[NETWORKING_KEY_BUFSIZE];
} WINHTTP_PROXY_NETWORKING_KEY, *PWINHTTP_PROXY_NETWORKING_KEY;

#define WINHTTP_PROXY_TYPE_DIRECT             0x00000001   // Direct to net
#define WINHTTP_PROXY_TYPE_PROXY              0x00000002   // Via named proxy
#define WINHTTP_PROXY_TYPE_AUTO_PROXY_URL     0x00000004   // Autoproxy URL
#define WINHTTP_PROXY_TYPE_AUTO_DETECT        0x00000008   // Use autoproxy detection

typedef struct _WINHTTP_PROXY_SETTINGS
{
    DWORD dwStructSize;
    DWORD dwFlags;
    DWORD dwCurrentSettingsVersion;
    PWSTR pwszConnectionName;
    PWSTR pwszProxy;
    PWSTR pwszProxyBypass;
    PWSTR pwszAutoconfigUrl;
    PWSTR pwszAutoconfigSecondaryUrl;
    DWORD dwAutoDiscoveryFlags;
    PWSTR pwszLastKnownGoodAutoConfigUrl;
    DWORD dwAutoconfigReloadDelayMins;
    FILETIME ftLastKnownDetectTime;
    DWORD dwDetectedInterfaceIpCount;
    PDWORD pdwDetectedInterfaceIp;
    DWORD cNetworkKeys;
    PWINHTTP_PROXY_NETWORKING_KEY pNetworkKeys;
} WINHTTP_PROXY_SETTINGS, *PWINHTTP_PROXY_SETTINGS;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */


#pragma endregion

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

//
// WINHTTP_CERTIFICATE_INFO lpBuffer - contains the certificate returned from
// the server
//

typedef struct _WINHTTP_CERTIFICATE_INFO
{
    //
    // ftExpiry - date the certificate expires.
    //

    FILETIME ftExpiry;

    //
    // ftStart - date the certificate becomes valid.
    //

    FILETIME ftStart;

    //
    // lpszSubjectInfo - the name of organization, site, and server
    //   the cert. was issued for.
    //

    LPWSTR lpszSubjectInfo;

    //
    // lpszIssuerInfo - the name of organization, site, and server
    //   the cert was issues by.
    //

    LPWSTR lpszIssuerInfo;

    //
    // lpszProtocolName - the name of the protocol used to provide the secure
    //   connection.
    //

    LPWSTR lpszProtocolName;

    //
    // lpszSignatureAlgName - the name of the algorithm used for signing
    //  the certificate.
    //

    LPWSTR lpszSignatureAlgName;

    //
    // lpszEncryptionAlgName - the name of the algorithm used for
    //  doing encryption over the secure channel (SSL) connection.
    //

    LPWSTR lpszEncryptionAlgName;

    //
    // dwKeySize - size of the key.
    //

    DWORD dwKeySize;

} WINHTTP_CERTIFICATE_INFO, *PWINHTTP_CERTIFICATE_INFO;

#ifdef _WS2DEF_

typedef struct _WINHTTP_CONNECTION_INFO
{
    DWORD cbSize;
    SOCKADDR_STORAGE LocalAddress;  // local ip, local port
    SOCKADDR_STORAGE RemoteAddress; // remote ip, remote port
} WINHTTP_CONNECTION_INFO, *PWINHTTP_CONNECTION_INFO;

#endif

#ifdef __SCHANNEL_H__

typedef struct _WINHTTP_SECURITY_INFO
{
    SecPkgContext_ConnectionInfo ConnectionInfo;
    SecPkgContext_CipherInfo CipherInfo;
} WINHTTP_SECURITY_INFO, *PWINHTTP_SECURITY_INFO;

#endif

typedef enum _WINHTTP_REQUEST_TIME_ENTRY
{
    WinHttpProxyDetectionStart = 0,
    WinHttpProxyDetectionEnd,

    WinHttpConnectionAcquireStart,
    WinHttpConnectionAcquireWaitEnd,
    WinHttpConnectionAcquireEnd,

    WinHttpNameResolutionStart,
    WinHttpNameResolutionEnd,

    WinHttpConnectionEstablishmentStart,
    WinHttpConnectionEstablishmentEnd,

    WinHttpTlsHandshakeClientLeg1Start,
    WinHttpTlsHandshakeClientLeg1End,
    WinHttpTlsHandshakeClientLeg2Start,
    WinHttpTlsHandshakeClientLeg2End,
    WinHttpTlsHandshakeClientLeg3Start,
    WinHttpTlsHandshakeClientLeg3End,

    WinHttpStreamWaitStart,
    WinHttpStreamWaitEnd,

    WinHttpSendRequestStart,
    WinHttpSendRequestHeadersCompressionStart,
    WinHttpSendRequestHeadersCompressionEnd,
    WinHttpSendRequestHeadersEnd,
    WinHttpSendRequestEnd,

    WinHttpReceiveResponseStart,
    WinHttpReceiveResponseHeadersDecompressionStart,
    WinHttpReceiveResponseHeadersDecompressionEnd,
    WinHttpReceiveResponseHeadersEnd,
    WinHttpReceiveResponseBodyDecompressionDelta,
    WinHttpReceiveResponseEnd,

    WinHttpProxyTunnelStart,
    WinHttpProxyTunnelEnd,

    WinHttpProxyTlsHandshakeClientLeg1Start,
    WinHttpProxyTlsHandshakeClientLeg1End,
    WinHttpProxyTlsHandshakeClientLeg2Start,
    WinHttpProxyTlsHandshakeClientLeg2End,
    WinHttpProxyTlsHandshakeClientLeg3Start,
    WinHttpProxyTlsHandshakeClientLeg3End,

    WinHttpRequestTimeLast,
    WinHttpRequestTimeMax = 64
} WINHTTP_REQUEST_TIME_ENTRY;

typedef struct _WINHTTP_REQUEST_TIMES
{
    ULONG cTimes;
    ULONGLONG rgullTimes[WinHttpRequestTimeMax];
} WINHTTP_REQUEST_TIMES, *PWINHTTP_REQUEST_TIMES;

typedef enum _WINHTTP_REQUEST_STAT_ENTRY
{
    WinHttpConnectFailureCount = 0,
    WinHttpProxyFailureCount,

    WinHttpTlsHandshakeClientLeg1Size,
    WinHttpTlsHandshakeServerLeg1Size,
    WinHttpTlsHandshakeClientLeg2Size,
    WinHttpTlsHandshakeServerLeg2Size,

    WinHttpRequestHeadersSize,
    WinHttpRequestHeadersCompressedSize,

    WinHttpResponseHeadersSize,
    WinHttpResponseHeadersCompressedSize,
    WinHttpResponseBodySize,
    WinHttpResponseBodyCompressedSize,

    WinHttpProxyTlsHandshakeClientLeg1Size,
    WinHttpProxyTlsHandshakeServerLeg1Size,
    WinHttpProxyTlsHandshakeClientLeg2Size,
    WinHttpProxyTlsHandshakeServerLeg2Size,

    WinHttpRequestStatLast,
    WinHttpRequestStatMax = 32
} WINHTTP_REQUEST_STAT_ENTRY;

#define WINHTTP_REQUEST_STAT_FLAG_TCP_FAST_OPEN                 0x00000001
#define WINHTTP_REQUEST_STAT_FLAG_TLS_SESSION_RESUMPTION        0x00000002
#define WINHTTP_REQUEST_STAT_FLAG_TLS_FALSE_START               0x00000004
#define WINHTTP_REQUEST_STAT_FLAG_PROXY_TLS_SESSION_RESUMPTION  0x00000008
#define WINHTTP_REQUEST_STAT_FLAG_PROXY_TLS_FALSE_START         0x00000010
#define WINHTTP_REQUEST_STAT_FLAG_FIRST_REQUEST                 0x00000020

typedef struct _WINHTTP_REQUEST_STATS
{
    ULONGLONG ullFlags;
    ULONG ulIndex;
    ULONG cStats;
    ULONGLONG rgullStats[WinHttpRequestStatMax];
} WINHTTP_REQUEST_STATS, *PWINHTTP_REQUEST_STATS;

#define WINHTTP_MATCH_CONNECTION_GUID_FLAG_REQUIRE_MARKED_CONNECTION 0x00000001

#define WINHTTP_MATCH_CONNECTION_GUID_FLAGS_MASK WINHTTP_MATCH_CONNECTION_GUID_FLAG_REQUIRE_MARKED_CONNECTION

typedef struct _WINHTTP_MATCH_CONNECTION_GUID
{
    GUID ConnectionGuid;
    ULONGLONG ullFlags;
} WINHTTP_MATCH_CONNECTION_GUID, *PWINHTTP_MATCH_CONNECTION_GUID;

#pragma warning(push)
#pragma warning(disable:4201) //nameless unions

typedef struct _WINHTTP_EXTENDED_HEADER
{
    union
    {
        PCWSTR pwszName;
        PCSTR  pszName;
    };
    union
    {
        PCWSTR pwszValue;
        PCSTR  pszValue;
    };
} WINHTTP_EXTENDED_HEADER, *PWINHTTP_EXTENDED_HEADER;

#pragma warning(pop)

typedef union _WINHTTP_HEADER_NAME
{
    PCWSTR pwszName;
    PCSTR  pszName;
} WINHTTP_HEADER_NAME, *PWINHTTP_HEADER_NAME;

#define WINHTTP_RESOLVER_CACHE_CONFIG_FLAG_SOFT_LIMIT      0x00000001
#define WINHTTP_RESOLVER_CACHE_CONFIG_FLAG_BYPASS_CACHE    0x00000002
#define WINHTTP_RESOLVER_CACHE_CONFIG_FLAG_USE_DNS_TTL     0x00000004
#define WINHTTP_RESOLVER_CACHE_CONFIG_FLAG_CONN_USE_TTL    0x00000008

typedef enum _WINHTTP_SECURE_DNS_SETTING
{
    WinHttpSecureDnsSettingDefault                   = 0,
    WinHttpSecureDnsSettingForcePlaintext            = 1,
    WinHttpSecureDnsSettingRequireEncryption         = 2,
    WinHttpSecureDnsSettingTryEncryptionWithFallback = 3,
    WinHttpSecureDnsSettingMax                       = 4
} WINHTTP_SECURE_DNS_SETTING;


typedef struct _WINHTTP_RESOLVER_CACHE_CONFIG
{
    ULONG ulMaxResolverCacheEntries;

    //
    // ulMaxCacheEntryAge is the maximum allowed age of a cache entry specified in minutes.
    //

    ULONG ulMaxCacheEntryAge;

    //
    // ulMinCacheEntryTtl the minimum TTL of a cache entry specified in seconds.
    //

    ULONG ulMinCacheEntryTtl;

    WINHTTP_SECURE_DNS_SETTING SecureDnsSetting;

    //
    // If WINHTTP_RESOLVER_CACHE_CONFIG_FLAG_CONN_USE_TTL is set, then ullCOnnResolutionWaitTime
    // can be used to control how frequently a re-resolution attempt is made for any connection.
    // It is specified in 100 nanosecond units, and the default is 600000000 (one minute).
    //

    ULONGLONG ullConnResolutionWaitTime;

    ULONGLONG ullFlags;
} WINHTTP_RESOLVER_CACHE_CONFIG, *PWINHTTP_RESOLVER_CACHE_CONFIG;

//
// Structures for WinHttpQueryConnectionGroup
//

typedef struct _WINHTTP_CONNECTION_GROUP
{
    ULONG cConnections;
    GUID guidGroup;
} WINHTTP_CONNECTION_GROUP, *PWINHTTP_CONNECTION_GROUP;

typedef struct _WINHTTP_HOST_CONNECTION_GROUP
{
    PCWSTR pwszHost;
    ULONG cConnectionGroups;
    PWINHTTP_CONNECTION_GROUP pConnectionGroups;
} WINHTTP_HOST_CONNECTION_GROUP, *PWINHTTP_HOST_CONNECTION_GROUP;

typedef struct _WINHTTP_QUERY_CONNECTION_GROUP_RESULT
{
    ULONG cHosts;
    PWINHTTP_HOST_CONNECTION_GROUP pHostConnectionGroups;
} WINHTTP_QUERY_CONNECTION_GROUP_RESULT, *PWINHTTP_QUERY_CONNECTION_GROUP_RESULT;

#define WINHTTP_QUERY_CONNECTION_GROUP_FLAG_INSECURE 0x0000000000000001ull


typedef struct _WINHTTP_HTTP2_RECEIVE_WINDOW
{
    ULONG ulStreamWindow;
    ULONG ulStreamWindowUpdateDelta;
} WINHTTP_HTTP2_RECEIVE_WINDOW, *PWINHTTP_HTTP2_RECEIVE_WINDOW;

//
// constants for WinHttpTimeFromSystemTime
//

#define WINHTTP_TIME_FORMAT_BUFSIZE   62

//
// options manifests for WinHttp{Query|Set}Option
//

#define WINHTTP_FIRST_OPTION                            WINHTTP_OPTION_CALLBACK

#define WINHTTP_OPTION_CALLBACK                         1
#define WINHTTP_OPTION_RESOLVE_TIMEOUT                  2
#define WINHTTP_OPTION_CONNECT_TIMEOUT                  3
#define WINHTTP_OPTION_CONNECT_RETRIES                  4
#define WINHTTP_OPTION_SEND_TIMEOUT                     5
#define WINHTTP_OPTION_RECEIVE_TIMEOUT                  6
#define WINHTTP_OPTION_RECEIVE_RESPONSE_TIMEOUT         7
#define WINHTTP_OPTION_HANDLE_TYPE                      9
#define WINHTTP_OPTION_READ_BUFFER_SIZE                 12
#define WINHTTP_OPTION_WRITE_BUFFER_SIZE                13
#define WINHTTP_OPTION_PARENT_HANDLE                    21
#define WINHTTP_OPTION_EXTENDED_ERROR                   24
#define WINHTTP_OPTION_SECURITY_FLAGS                   31
#define WINHTTP_OPTION_SECURITY_CERTIFICATE_STRUCT      32
#define WINHTTP_OPTION_URL                              34
#define WINHTTP_OPTION_SECURITY_KEY_BITNESS             36
#define WINHTTP_OPTION_PROXY                            38
#define WINHTTP_OPTION_PROXY_RESULT_ENTRY               39


#define WINHTTP_OPTION_USER_AGENT                       41
#define WINHTTP_OPTION_CONTEXT_VALUE                    45
#define WINHTTP_OPTION_CLIENT_CERT_CONTEXT              47
#define WINHTTP_OPTION_REQUEST_PRIORITY                 58
#define WINHTTP_OPTION_HTTP_VERSION                     59
#define WINHTTP_OPTION_DISABLE_FEATURE                  63

#define WINHTTP_OPTION_CODEPAGE                         68
#define WINHTTP_OPTION_MAX_CONNS_PER_SERVER             73
#define WINHTTP_OPTION_MAX_CONNS_PER_1_0_SERVER         74
#define WINHTTP_OPTION_AUTOLOGON_POLICY                 77
#define WINHTTP_OPTION_SERVER_CERT_CONTEXT              78
#define WINHTTP_OPTION_ENABLE_FEATURE                   79
#define WINHTTP_OPTION_WORKER_THREAD_COUNT              80
#define WINHTTP_OPTION_PASSPORT_COBRANDING_TEXT         81
#define WINHTTP_OPTION_PASSPORT_COBRANDING_URL          82
#define WINHTTP_OPTION_CONFIGURE_PASSPORT_AUTH          83
#define WINHTTP_OPTION_SECURE_PROTOCOLS                 84
#define WINHTTP_OPTION_ENABLETRACING                    85
#define WINHTTP_OPTION_PASSPORT_SIGN_OUT                86
#define WINHTTP_OPTION_PASSPORT_RETURN_URL              87
#define WINHTTP_OPTION_REDIRECT_POLICY                  88
#define WINHTTP_OPTION_MAX_HTTP_AUTOMATIC_REDIRECTS     89
#define WINHTTP_OPTION_MAX_HTTP_STATUS_CONTINUE         90
#define WINHTTP_OPTION_MAX_RESPONSE_HEADER_SIZE         91
#define WINHTTP_OPTION_MAX_RESPONSE_DRAIN_SIZE          92
#define WINHTTP_OPTION_CONNECTION_INFO                  93
#define WINHTTP_OPTION_CLIENT_CERT_ISSUER_LIST          94
#define WINHTTP_OPTION_SPN                              96

#define WINHTTP_OPTION_GLOBAL_PROXY_CREDS               97
#define WINHTTP_OPTION_GLOBAL_SERVER_CREDS              98

#define WINHTTP_OPTION_UNLOAD_NOTIFY_EVENT              99
#define WINHTTP_OPTION_REJECT_USERPWD_IN_URL            100
#define WINHTTP_OPTION_USE_GLOBAL_SERVER_CREDENTIALS    101


#define WINHTTP_OPTION_RECEIVE_PROXY_CONNECT_RESPONSE   103
#define WINHTTP_OPTION_IS_PROXY_CONNECT_RESPONSE        104

#define WINHTTP_OPTION_NETWORK_INTERFACE_AFFINITY       105

#define WINHTTP_OPTION_SERVER_SPN_USED                  106
#define WINHTTP_OPTION_PROXY_SPN_USED                   107

#define WINHTTP_OPTION_SERVER_CBT                       108


#define WINHTTP_OPTION_UNSAFE_HEADER_PARSING            110
#define WINHTTP_OPTION_ASSURED_NON_BLOCKING_CALLBACKS   111


#define WINHTTP_OPTION_UPGRADE_TO_WEB_SOCKET            114
#define WINHTTP_OPTION_WEB_SOCKET_CLOSE_TIMEOUT         115
#define WINHTTP_OPTION_WEB_SOCKET_KEEPALIVE_INTERVAL    116


#define WINHTTP_OPTION_DECOMPRESSION                    118


#define WINHTTP_OPTION_WEB_SOCKET_RECEIVE_BUFFER_SIZE   122
#define WINHTTP_OPTION_WEB_SOCKET_SEND_BUFFER_SIZE      123


#define WINHTTP_OPTION_TCP_PRIORITY_HINT                128


#define WINHTTP_OPTION_CONNECTION_FILTER                131


#define WINHTTP_OPTION_ENABLE_HTTP_PROTOCOL             133
#define WINHTTP_OPTION_HTTP_PROTOCOL_USED               134


#define WINHTTP_OPTION_KDC_PROXY_SETTINGS               136

#define WINHTTP_OPTION_PROXY_DISABLE_SERVICE_CALLS      137

#define WINHTTP_OPTION_ENCODE_EXTRA                     138
#define WINHTTP_OPTION_DISABLE_STREAM_QUEUE             139

#define WINHTTP_OPTION_IPV6_FAST_FALLBACK               140

#define WINHTTP_OPTION_CONNECTION_STATS_V0              141
#define WINHTTP_OPTION_REQUEST_TIMES                    142

#define WINHTTP_OPTION_EXPIRE_CONNECTION                143

#define WINHTTP_OPTION_DISABLE_SECURE_PROTOCOL_FALLBACK 144

#define WINHTTP_OPTION_HTTP_PROTOCOL_REQUIRED           145

#define WINHTTP_OPTION_REQUEST_STATS                    146

#define WINHTTP_OPTION_SERVER_CERT_CHAIN_CONTEXT        147

#define WINHTTP_OPTION_SERVER_CERT_CHAIN_BUILD_FLAGS    148


#define WINHTTP_OPTION_CONNECTION_STATS_V1              150

#define WINHTTP_OPTION_SECURITY_INFO                    151

#define WINHTTP_OPTION_TCP_KEEPALIVE                    152

#define WINHTTP_OPTION_TCP_FAST_OPEN                    153

#define WINHTTP_OPTION_TLS_FALSE_START                  154

#define WINHTTP_OPTION_IGNORE_CERT_REVOCATION_OFFLINE   155


#define WINHTTP_OPTION_TLS_PROTOCOL_INSECURE_FALLBACK   158

#define WINHTTP_OPTION_STREAM_ERROR_CODE                159

#define WINHTTP_OPTION_REQUIRE_STREAM_END               160

#define WINHTTP_OPTION_ENABLE_HTTP2_PLUS_CLIENT_CERT    161

#define WINHTTP_OPTION_FAILED_CONNECTION_RETRIES        162


#define WINHTTP_OPTION_HTTP2_KEEPALIVE                  164

#define WINHTTP_OPTION_RESOLUTION_HOSTNAME              165

#define WINHTTP_OPTION_SET_TOKEN_BINDING                166

#define WINHTTP_OPTION_TOKEN_BINDING_PUBLIC_KEY         167

#define WINHTTP_OPTION_REFERER_TOKEN_BINDING_HOSTNAME   168

#define WINHTTP_OPTION_HTTP2_PLUS_TRANSFER_ENCODING     169

#define WINHTTP_OPTION_RESOLVER_CACHE_CONFIG            170

#define WINHTTP_OPTION_DISABLE_CERT_CHAIN_BUILDING      171

#define WINHTTP_OPTION_BACKGROUND_CONNECTIONS           172

#define WINHTTP_OPTION_FIRST_AVAILABLE_CONNECTION       173


#define WINHTTP_OPTION_TCP_PRIORITY_STATUS              177

#define WINHTTP_OPTION_CONNECTION_GUID                  178

#define WINHTTP_OPTION_MATCH_CONNECTION_GUID            179


#define WINHTTP_OPTION_HTTP2_RECEIVE_WINDOW             183

#define WINHTTP_OPTION_FEATURE_SUPPORTED                184

#define WINHTTP_OPTION_QUIC_STATS                       185


#define WINHTTP_OPTION_HTTP3_KEEPALIVE                  188
#define WINHTTP_OPTION_HTTP3_HANDSHAKE_TIMEOUT          189
#define WINHTTP_OPTION_HTTP3_INITIAL_RTT                190
#define WINHTTP_OPTION_HTTP3_STREAM_ERROR_CODE          191
#define WINHTTP_OPTION_REQUEST_ANNOTATION               192
#define WINHTTP_OPTION_DISABLE_PROXY_AUTH_SCHEMES       193
#define WINHTTP_OPTION_REVERT_IMPERSONATION_SERVER_CERT 194

#define WINHTTP_OPTION_DISABLE_GLOBAL_POOLING           195

#define WINHTTP_OPTION_USE_SESSION_SCH_CRED             196

#define WINHTTP_OPTION_SERVER_CERT_CHAIN_BUILD_CACHE_ONLY 199

#define WINHTTP_OPTION_QUIC_STATS_V2                    200


#define WINHTTP_OPTION_QUIC_STREAM_STATS                202

#define WINHTTP_OPTION_USE_LOOKASIDE                    203

#define WINHTTP_OPTION_ERROR_LOG_GUID                   204

#define WINHTTP_OPTION_ENABLE_FAST_FORWARDING           205
#define WINHTTP_OPTION_FAST_FORWARDING_RESPONSE_DATA    206

#define WINHTTP_OPTION_UPGRADE_TO_PROTOCOL              207

#define WINHTTP_OPTION_CONNECTION_STATS_V2              208

#define WINHTTP_OPTION_FAST_FORWARDING_RESPONSE_STATUS  209

#define WINHTTP_OPTION_DSCP_TAG                         210

#define WINHTTP_OPTION_HTTP11_DOWNGRADE_TTL             211

#define WINHTTP_OPTION_SESSION_ERROR_LOG_GUID           212


#define WINHTTP_LAST_OPTION                             WINHTTP_OPTION_SESSION_ERROR_LOG_GUID

#define WINHTTP_OPTION_USERNAME                         0x1000
#define WINHTTP_OPTION_PASSWORD                         0x1001
#define WINHTTP_OPTION_PROXY_USERNAME                   0x1002
#define WINHTTP_OPTION_PROXY_PASSWORD                   0x1003


// manifest value for WINHTTP_OPTION_MAX_CONNS_PER_SERVER and WINHTTP_OPTION_MAX_CONNS_PER_1_0_SERVER
#define WINHTTP_CONNS_PER_SERVER_UNLIMITED              0xFFFFFFFF


#define WINHTTP_CONNECTION_RETRY_CONDITION_408              0x1
#define WINHTTP_CONNECTION_RETRY_CONDITION_SSL_HANDSHAKE    0x2
#define WINHTTP_CONNECTION_RETRY_CONDITION_STALE_CONNECTION 0x4

#define WINHTTP_CONNECTION_RETRY_CONDITION_MASK           \
    (WINHTTP_CONNECTION_RETRY_CONDITION_408 |             \
     WINHTTP_CONNECTION_RETRY_CONDITION_SSL_HANDSHAKE |   \
     WINHTTP_CONNECTION_RETRY_CONDITION_STALE_CONNECTION) \

typedef struct _WINHTTP_FAILED_CONNECTION_RETRIES
{
    DWORD dwMaxRetries;
    DWORD dwAllowedRetryConditions;
} WINHTTP_FAILED_CONNECTION_RETRIES, *PWINHTTP_FAILED_CONNECTION_RETRIES;

//
// Values for WINHTTP_OPTION_DECOMPRESSION
//

#define WINHTTP_DECOMPRESSION_FLAG_GZIP     0x00000001
#define WINHTTP_DECOMPRESSION_FLAG_DEFLATE  0x00000002

#define WINHTTP_DECOMPRESSION_FLAG_ALL ( \
    WINHTTP_DECOMPRESSION_FLAG_GZIP    | \
    WINHTTP_DECOMPRESSION_FLAG_DEFLATE)

//
// Values for WINHTTP_OPTION_ENABLE_HTTP_PROTOCOL / WINHTTP_OPTION_HTTP_PROTOCOL_USED
//

#define WINHTTP_PROTOCOL_FLAG_HTTP2 0x1
#define WINHTTP_PROTOCOL_FLAG_HTTP3 0x2
#define WINHTTP_PROTOCOL_MASK (WINHTTP_PROTOCOL_FLAG_HTTP2 | WINHTTP_PROTOCOL_FLAG_HTTP3)

#define WINHTTP_OPTION_REQUEST_ANNOTATION_MAX_LENGTH 0xFA00


// values for WINHTTP_OPTION_AUTOLOGON_POLICY
#define WINHTTP_AUTOLOGON_SECURITY_LEVEL_MEDIUM     0
#define WINHTTP_AUTOLOGON_SECURITY_LEVEL_LOW        1
#define WINHTTP_AUTOLOGON_SECURITY_LEVEL_HIGH       2
#define WINHTTP_AUTOLOGON_SECURITY_LEVEL_PROXY_ONLY 3

#define WINHTTP_AUTOLOGON_SECURITY_LEVEL_DEFAULT        WINHTTP_AUTOLOGON_SECURITY_LEVEL_MEDIUM
#define WINHTTP_AUTOLOGON_SECURITY_LEVEL_MAX            WINHTTP_AUTOLOGON_SECURITY_LEVEL_PROXY_ONLY

// values for WINHTTP_OPTION_REDIRECT_POLICY
#define WINHTTP_OPTION_REDIRECT_POLICY_NEVER                        0
#define WINHTTP_OPTION_REDIRECT_POLICY_DISALLOW_HTTPS_TO_HTTP       1
#define WINHTTP_OPTION_REDIRECT_POLICY_ALWAYS                       2

#define WINHTTP_OPTION_REDIRECT_POLICY_LAST            WINHTTP_OPTION_REDIRECT_POLICY_ALWAYS
#define WINHTTP_OPTION_REDIRECT_POLICY_DEFAULT         WINHTTP_OPTION_REDIRECT_POLICY_DISALLOW_HTTPS_TO_HTTP

#define WINHTTP_DISABLE_PASSPORT_AUTH    0x00000000
#define WINHTTP_ENABLE_PASSPORT_AUTH     0x10000000
#define WINHTTP_DISABLE_PASSPORT_KEYRING 0x20000000
#define WINHTTP_ENABLE_PASSPORT_KEYRING  0x40000000

//
// Bits passed with WINHTTP_OPTION_DISABLE_PROXY_AUTH_SCHEMES option
//

#define WINHTTP_PROXY_DISABLE_SCHEME_BASIC          0x00000001
#define WINHTTP_PROXY_DISABLE_SCHEME_DIGEST         0x00000002
#define WINHTTP_PROXY_DISABLE_SCHEME_NTLM           0x00000004
#define WINHTTP_PROXY_DISABLE_SCHEME_KERBEROS       0x00000008
#define WINHTTP_PROXY_DISABLE_SCHEME_NEGOTIATE      0x00000010
#define WINHTTP_PROXY_DISABLE_AUTH_LOCAL_SERVICE    0x00000100


#define WINHTTP_SERVER_CERT_CHAIN_CACHE_ONLY_URL_RETRIEVAL    0x00000004
#define WINHTTP_SERVER_CERT_CHAIN_DISABLE_AIA                 0x00002000
#define WINHTTP_SERVER_CERT_CHAIN_REVOCATION_CHECK_CACHE_ONLY 0x80000000

// values for WINHTTP_OPTION_DISABLE_FEATURE
#define WINHTTP_DISABLE_COOKIES                   0x00000001
#define WINHTTP_DISABLE_REDIRECTS                 0x00000002
#define WINHTTP_DISABLE_AUTHENTICATION            0x00000004
#define WINHTTP_DISABLE_KEEP_ALIVE                0x00000008

// values for WINHTTP_OPTION_ENABLE_FEATURE
#define WINHTTP_ENABLE_SSL_REVOCATION             0x00000001
#define WINHTTP_ENABLE_SSL_REVERT_IMPERSONATION   0x00000002

// values for WINHTTP_OPTION_SPN
#define WINHTTP_DISABLE_SPN_SERVER_PORT           0x00000000
#define WINHTTP_ENABLE_SPN_SERVER_PORT            0x00000001
#define WINHTTP_OPTION_SPN_MASK                   WINHTTP_ENABLE_SPN_SERVER_PORT

typedef struct tagWINHTTP_CREDS
{
    LPSTR lpszUserName;
    LPSTR lpszPassword;
    LPSTR lpszRealm;
    DWORD dwAuthScheme;
    LPSTR lpszHostName;
    DWORD dwPort;
} WINHTTP_CREDS, *PWINHTTP_CREDS;

// structure for WINHTTP_OPTION_GLOBAL_SERVER_CREDS and
// WINHTTP_OPTION_GLOBAL_PROXY_CREDS
typedef struct tagWINHTTP_CREDS_EX
{
    LPSTR lpszUserName;
    LPSTR lpszPassword;
    LPSTR lpszRealm;
    DWORD dwAuthScheme;
    LPSTR lpszHostName;
    DWORD dwPort;
    LPSTR lpszUrl;
} WINHTTP_CREDS_EX, *PWINHTTP_CREDS_EX;

//
// winhttp handle types
//
#define WINHTTP_HANDLE_TYPE_SESSION                  1
#define WINHTTP_HANDLE_TYPE_CONNECT                  2
#define WINHTTP_HANDLE_TYPE_REQUEST                  3
#define WINHTTP_HANDLE_TYPE_PROXY_RESOLVER           4
#define WINHTTP_HANDLE_TYPE_WEBSOCKET                5
#define WINHTTP_HANDLE_TYPE_PROTOCOL                 6

//
// values for auth schemes
//
#define WINHTTP_AUTH_SCHEME_BASIC      0x00000001
#define WINHTTP_AUTH_SCHEME_NTLM       0x00000002
#define WINHTTP_AUTH_SCHEME_PASSPORT   0x00000004
#define WINHTTP_AUTH_SCHEME_DIGEST     0x00000008
#define WINHTTP_AUTH_SCHEME_NEGOTIATE  0x00000010

// WinHttp supported Authentication Targets

#define WINHTTP_AUTH_TARGET_SERVER 0x00000000
#define WINHTTP_AUTH_TARGET_PROXY  0x00000001

//
// values for WINHTTP_OPTION_SECURITY_FLAGS
//

// query only
#define SECURITY_FLAG_SECURE                    0x00000001 // can query only
#define SECURITY_FLAG_STRENGTH_WEAK             0x10000000
#define SECURITY_FLAG_STRENGTH_MEDIUM           0x40000000
#define SECURITY_FLAG_STRENGTH_STRONG           0x20000000



// Secure connection error status flags
#define WINHTTP_CALLBACK_STATUS_FLAG_CERT_REV_FAILED         0x00000001
#define WINHTTP_CALLBACK_STATUS_FLAG_INVALID_CERT            0x00000002
#define WINHTTP_CALLBACK_STATUS_FLAG_CERT_REVOKED            0x00000004
#define WINHTTP_CALLBACK_STATUS_FLAG_INVALID_CA              0x00000008
#define WINHTTP_CALLBACK_STATUS_FLAG_CERT_CN_INVALID         0x00000010
#define WINHTTP_CALLBACK_STATUS_FLAG_CERT_DATE_INVALID       0x00000020
#define WINHTTP_CALLBACK_STATUS_FLAG_CERT_WRONG_USAGE        0x00000040
#define WINHTTP_CALLBACK_STATUS_FLAG_SECURITY_CHANNEL_ERROR  0x80000000


#define WINHTTP_FLAG_SECURE_PROTOCOL_SSL2   0x00000008
#define WINHTTP_FLAG_SECURE_PROTOCOL_SSL3   0x00000020
#define WINHTTP_FLAG_SECURE_PROTOCOL_TLS1   0x00000080
#define WINHTTP_FLAG_SECURE_PROTOCOL_TLS1_1 0x00000200
#define WINHTTP_FLAG_SECURE_PROTOCOL_TLS1_2 0x00000800
#define WINHTTP_FLAG_SECURE_PROTOCOL_TLS1_3 0x00002000
#define WINHTTP_FLAG_SECURE_PROTOCOL_ALL    (WINHTTP_FLAG_SECURE_PROTOCOL_SSL2 | \
                                             WINHTTP_FLAG_SECURE_PROTOCOL_SSL3 | \
                                             WINHTTP_FLAG_SECURE_PROTOCOL_TLS1)


//
// callback function for WinHttpSetStatusCallback
//

typedef
VOID
(CALLBACK * WINHTTP_STATUS_CALLBACK)(
    IN HINTERNET hInternet,
    IN DWORD_PTR dwContext,
    IN DWORD dwInternetStatus,
    IN LPVOID lpvStatusInformation OPTIONAL,
    IN DWORD dwStatusInformationLength
    );

typedef WINHTTP_STATUS_CALLBACK * LPWINHTTP_STATUS_CALLBACK;


//
// status manifests for WinHttp status callback
//

#define WINHTTP_CALLBACK_STATUS_RESOLVING_NAME              0x00000001
#define WINHTTP_CALLBACK_STATUS_NAME_RESOLVED               0x00000002
#define WINHTTP_CALLBACK_STATUS_CONNECTING_TO_SERVER        0x00000004
#define WINHTTP_CALLBACK_STATUS_CONNECTED_TO_SERVER         0x00000008
#define WINHTTP_CALLBACK_STATUS_SENDING_REQUEST             0x00000010
#define WINHTTP_CALLBACK_STATUS_REQUEST_SENT                0x00000020
#define WINHTTP_CALLBACK_STATUS_RECEIVING_RESPONSE          0x00000040
#define WINHTTP_CALLBACK_STATUS_RESPONSE_RECEIVED           0x00000080
#define WINHTTP_CALLBACK_STATUS_CLOSING_CONNECTION          0x00000100
#define WINHTTP_CALLBACK_STATUS_CONNECTION_CLOSED           0x00000200
#define WINHTTP_CALLBACK_STATUS_HANDLE_CREATED              0x00000400
#define WINHTTP_CALLBACK_STATUS_HANDLE_CLOSING              0x00000800
#define WINHTTP_CALLBACK_STATUS_DETECTING_PROXY             0x00001000
#define WINHTTP_CALLBACK_STATUS_REDIRECT                    0x00004000
#define WINHTTP_CALLBACK_STATUS_INTERMEDIATE_RESPONSE       0x00008000
#define WINHTTP_CALLBACK_STATUS_SECURE_FAILURE              0x00010000
#define WINHTTP_CALLBACK_STATUS_HEADERS_AVAILABLE           0x00020000
#define WINHTTP_CALLBACK_STATUS_DATA_AVAILABLE              0x00040000
#define WINHTTP_CALLBACK_STATUS_READ_COMPLETE               0x00080000
#define WINHTTP_CALLBACK_STATUS_WRITE_COMPLETE              0x00100000
#define WINHTTP_CALLBACK_STATUS_REQUEST_ERROR               0x00200000
#define WINHTTP_CALLBACK_STATUS_SENDREQUEST_COMPLETE        0x00400000


#define WINHTTP_CALLBACK_STATUS_GETPROXYFORURL_COMPLETE     0x01000000
#define WINHTTP_CALLBACK_STATUS_CLOSE_COMPLETE              0x02000000
#define WINHTTP_CALLBACK_STATUS_SHUTDOWN_COMPLETE           0x04000000
#define WINHTTP_CALLBACK_STATUS_GETPROXYSETTINGS_COMPLETE   0x08000000
#define WINHTTP_CALLBACK_STATUS_SETTINGS_WRITE_COMPLETE     0x10000000
#define WINHTTP_CALLBACK_STATUS_SETTINGS_READ_COMPLETE      0x20000000

// API Enums for WINHTTP_CALLBACK_STATUS_REQUEST_ERROR:
#define API_RECEIVE_RESPONSE        (1)
#define API_QUERY_DATA_AVAILABLE    (2)
#define API_READ_DATA               (3)
#define API_WRITE_DATA              (4)
#define API_SEND_REQUEST            (5)
#define API_GET_PROXY_FOR_URL       (6)
#define API_GET_PROXY_SETTINGS      (7)

#define WINHTTP_CALLBACK_FLAG_RESOLVE_NAME              (WINHTTP_CALLBACK_STATUS_RESOLVING_NAME | WINHTTP_CALLBACK_STATUS_NAME_RESOLVED)
#define WINHTTP_CALLBACK_FLAG_CONNECT_TO_SERVER         (WINHTTP_CALLBACK_STATUS_CONNECTING_TO_SERVER | WINHTTP_CALLBACK_STATUS_CONNECTED_TO_SERVER)
#define WINHTTP_CALLBACK_FLAG_SEND_REQUEST              (WINHTTP_CALLBACK_STATUS_SENDING_REQUEST | WINHTTP_CALLBACK_STATUS_REQUEST_SENT)
#define WINHTTP_CALLBACK_FLAG_RECEIVE_RESPONSE          (WINHTTP_CALLBACK_STATUS_RECEIVING_RESPONSE | WINHTTP_CALLBACK_STATUS_RESPONSE_RECEIVED)
#define WINHTTP_CALLBACK_FLAG_CLOSE_CONNECTION          (WINHTTP_CALLBACK_STATUS_CLOSING_CONNECTION | WINHTTP_CALLBACK_STATUS_CONNECTION_CLOSED)
#define WINHTTP_CALLBACK_FLAG_HANDLES                   (WINHTTP_CALLBACK_STATUS_HANDLE_CREATED | WINHTTP_CALLBACK_STATUS_HANDLE_CLOSING)
#define WINHTTP_CALLBACK_FLAG_DETECTING_PROXY           WINHTTP_CALLBACK_STATUS_DETECTING_PROXY
#define WINHTTP_CALLBACK_FLAG_REDIRECT                  WINHTTP_CALLBACK_STATUS_REDIRECT
#define WINHTTP_CALLBACK_FLAG_INTERMEDIATE_RESPONSE     WINHTTP_CALLBACK_STATUS_INTERMEDIATE_RESPONSE
#define WINHTTP_CALLBACK_FLAG_SECURE_FAILURE            WINHTTP_CALLBACK_STATUS_SECURE_FAILURE
#define WINHTTP_CALLBACK_FLAG_SENDREQUEST_COMPLETE      WINHTTP_CALLBACK_STATUS_SENDREQUEST_COMPLETE
#define WINHTTP_CALLBACK_FLAG_HEADERS_AVAILABLE         WINHTTP_CALLBACK_STATUS_HEADERS_AVAILABLE
#define WINHTTP_CALLBACK_FLAG_DATA_AVAILABLE            WINHTTP_CALLBACK_STATUS_DATA_AVAILABLE
#define WINHTTP_CALLBACK_FLAG_READ_COMPLETE             WINHTTP_CALLBACK_STATUS_READ_COMPLETE
#define WINHTTP_CALLBACK_FLAG_WRITE_COMPLETE            WINHTTP_CALLBACK_STATUS_WRITE_COMPLETE
#define WINHTTP_CALLBACK_FLAG_REQUEST_ERROR             WINHTTP_CALLBACK_STATUS_REQUEST_ERROR


#define WINHTTP_CALLBACK_FLAG_GETPROXYFORURL_COMPLETE   WINHTTP_CALLBACK_STATUS_GETPROXYFORURL_COMPLETE
#define WINHTTP_CALLBACK_FLAG_GETPROXYSETTINGS_COMPLETE WINHTTP_CALLBACK_STATUS_GETPROXYSETTINGS_COMPLETE

#define WINHTTP_CALLBACK_FLAG_ALL_COMPLETIONS           (WINHTTP_CALLBACK_STATUS_SENDREQUEST_COMPLETE       \
                                                        | WINHTTP_CALLBACK_STATUS_HEADERS_AVAILABLE         \
                                                        | WINHTTP_CALLBACK_STATUS_DATA_AVAILABLE            \
                                                        | WINHTTP_CALLBACK_STATUS_READ_COMPLETE             \
                                                        | WINHTTP_CALLBACK_STATUS_WRITE_COMPLETE            \
                                                        | WINHTTP_CALLBACK_STATUS_REQUEST_ERROR             \
                                                        | WINHTTP_CALLBACK_STATUS_GETPROXYFORURL_COMPLETE   \
                                                        | WINHTTP_CALLBACK_STATUS_GETPROXYSETTINGS_COMPLETE)

#define WINHTTP_CALLBACK_FLAG_ALL_NOTIFICATIONS         0xffffffff

//
// if the following value is returned by WinHttpSetStatusCallback, then
// probably an invalid (non-code) address was supplied for the callback
//

#define WINHTTP_INVALID_STATUS_CALLBACK        ((WINHTTP_STATUS_CALLBACK)(-1L))


//
// WinHttpQueryHeaders info levels. Generally, there is one info level
// for each potential RFC822/HTTP/MIME header that an HTTP server
// may send as part of a request response.
//
// The WINHTTP_QUERY_RAW_HEADERS info level is provided for clients
// that choose to perform their own header parsing.
//


#define WINHTTP_QUERY_MIME_VERSION                 0
#define WINHTTP_QUERY_CONTENT_TYPE                 1
#define WINHTTP_QUERY_CONTENT_TRANSFER_ENCODING    2
#define WINHTTP_QUERY_CONTENT_ID                   3
#define WINHTTP_QUERY_CONTENT_DESCRIPTION          4
#define WINHTTP_QUERY_CONTENT_LENGTH               5
#define WINHTTP_QUERY_CONTENT_LANGUAGE             6
#define WINHTTP_QUERY_ALLOW                        7
#define WINHTTP_QUERY_PUBLIC                       8
#define WINHTTP_QUERY_DATE                         9
#define WINHTTP_QUERY_EXPIRES                      10
#define WINHTTP_QUERY_LAST_MODIFIED                11
#define WINHTTP_QUERY_MESSAGE_ID                   12
#define WINHTTP_QUERY_URI                          13
#define WINHTTP_QUERY_DERIVED_FROM                 14
#define WINHTTP_QUERY_COST                         15
#define WINHTTP_QUERY_LINK                         16
#define WINHTTP_QUERY_PRAGMA                       17
#define WINHTTP_QUERY_VERSION                      18  // special: part of status line
#define WINHTTP_QUERY_STATUS_CODE                  19  // special: part of status line
#define WINHTTP_QUERY_STATUS_TEXT                  20  // special: part of status line
#define WINHTTP_QUERY_RAW_HEADERS                  21  // special: all headers as ASCIIZ
#define WINHTTP_QUERY_RAW_HEADERS_CRLF             22  // special: all headers
#define WINHTTP_QUERY_CONNECTION                   23
#define WINHTTP_QUERY_ACCEPT                       24
#define WINHTTP_QUERY_ACCEPT_CHARSET               25
#define WINHTTP_QUERY_ACCEPT_ENCODING              26
#define WINHTTP_QUERY_ACCEPT_LANGUAGE              27
#define WINHTTP_QUERY_AUTHORIZATION                28
#define WINHTTP_QUERY_CONTENT_ENCODING             29
#define WINHTTP_QUERY_FORWARDED                    30
#define WINHTTP_QUERY_FROM                         31
#define WINHTTP_QUERY_IF_MODIFIED_SINCE            32
#define WINHTTP_QUERY_LOCATION                     33
#define WINHTTP_QUERY_ORIG_URI                     34
#define WINHTTP_QUERY_REFERER                      35
#define WINHTTP_QUERY_RETRY_AFTER                  36
#define WINHTTP_QUERY_SERVER                       37
#define WINHTTP_QUERY_TITLE                        38
#define WINHTTP_QUERY_USER_AGENT                   39
#define WINHTTP_QUERY_WWW_AUTHENTICATE             40
#define WINHTTP_QUERY_PROXY_AUTHENTICATE           41
#define WINHTTP_QUERY_ACCEPT_RANGES                42
#define WINHTTP_QUERY_SET_COOKIE                   43
#define WINHTTP_QUERY_COOKIE                       44
#define WINHTTP_QUERY_REQUEST_METHOD               45  // special: GET/POST etc.
#define WINHTTP_QUERY_REFRESH                      46
#define WINHTTP_QUERY_CONTENT_DISPOSITION          47

//
// HTTP 1.1 defined headers
//

#define WINHTTP_QUERY_AGE                          48
#define WINHTTP_QUERY_CACHE_CONTROL                49
#define WINHTTP_QUERY_CONTENT_BASE                 50
#define WINHTTP_QUERY_CONTENT_LOCATION             51
#define WINHTTP_QUERY_CONTENT_MD5                  52
#define WINHTTP_QUERY_CONTENT_RANGE                53
#define WINHTTP_QUERY_ETAG                         54
#define WINHTTP_QUERY_HOST                         55
#define WINHTTP_QUERY_IF_MATCH                     56
#define WINHTTP_QUERY_IF_NONE_MATCH                57
#define WINHTTP_QUERY_IF_RANGE                     58
#define WINHTTP_QUERY_IF_UNMODIFIED_SINCE          59
#define WINHTTP_QUERY_MAX_FORWARDS                 60
#define WINHTTP_QUERY_PROXY_AUTHORIZATION          61
#define WINHTTP_QUERY_RANGE                        62
#define WINHTTP_QUERY_TRANSFER_ENCODING            63
#define WINHTTP_QUERY_UPGRADE                      64
#define WINHTTP_QUERY_VARY                         65
#define WINHTTP_QUERY_VIA                          66
#define WINHTTP_QUERY_WARNING                      67
#define WINHTTP_QUERY_EXPECT                       68
#define WINHTTP_QUERY_PROXY_CONNECTION             69
#define WINHTTP_QUERY_UNLESS_MODIFIED_SINCE        70



#define WINHTTP_QUERY_PROXY_SUPPORT                75
#define WINHTTP_QUERY_AUTHENTICATION_INFO          76
#define WINHTTP_QUERY_PASSPORT_URLS                77
#define WINHTTP_QUERY_PASSPORT_CONFIG              78

#define WINHTTP_QUERY_MAX                          78

#define WINHTTP_QUERY_EX_ALL_HEADERS               WINHTTP_QUERY_RAW_HEADERS

//
// WINHTTP_QUERY_CUSTOM - if this special value is supplied as the dwInfoLevel
// parameter of WinHttpQueryHeaders() then the lpBuffer parameter contains the name
// of the header we are to query
//

#define WINHTTP_QUERY_CUSTOM                       65535

//
// WINHTTP_QUERY_FLAG_REQUEST_HEADERS - if this bit is set in the dwInfoLevel
// parameter of WinHttpQueryHeaders() then the request headers will be queried for the
// request information
//

#define WINHTTP_QUERY_FLAG_REQUEST_HEADERS         0x80000000

//
// WINHTTP_QUERY_FLAG_SYSTEMTIME - if this bit is set in the dwInfoLevel parameter
// of WinHttpQueryHeaders() AND the header being queried contains date information,
// e.g. the "Expires:" header then lpBuffer will contain a SYSTEMTIME structure
// containing the date and time information converted from the header string
//

#define WINHTTP_QUERY_FLAG_SYSTEMTIME              0x40000000

//
// WINHTTP_QUERY_FLAG_NUMBER - if this bit is set in the dwInfoLevel parameter of
// HttpQueryHeader(), then the value of the header will be converted to a number
// before being returned to the caller, if applicable
//

#define WINHTTP_QUERY_FLAG_NUMBER                  0x20000000

//
// HTTP_QUERY_FLAG_NUMBER64 - if this bit is set in the dwInfoLevel parameter of
// HttpQueryInfo(), then the value of the header will be converted to a 64bit
// number before being returned to the caller, if applicable
//

#define WINHTTP_QUERY_FLAG_NUMBER64                0x08000000

//
// HTTP_QUERY_FLAG_TRAILERS - if this bit is set in the dwInfoLevel parameter of
// WinHttpQueryHeaders(), then the response trailers will be queried, if they exist
//

#define WINHTTP_QUERY_FLAG_TRAILERS                0x02000000

//
// WINHTTP_QUERY_FLAG_WIRE_ENCODING - if this bit is set in the dwInfoLevel parameter
// of WinHttpQueryHeaders(), then the value of the header will be returned
// with as it gets encoded when sent over the wire.
//

#define WINHTTP_QUERY_FLAG_WIRE_ENCODING           0x01000000


//
// HTTP Response Status Codes:
//

#define HTTP_STATUS_CONTINUE            100 // OK to continue with request
#define HTTP_STATUS_SWITCH_PROTOCOLS    101 // server has switched protocols in upgrade header

#define HTTP_STATUS_OK                  200 // request completed
#define HTTP_STATUS_CREATED             201 // object created, reason = new URI
#define HTTP_STATUS_ACCEPTED            202 // async completion (TBS)
#define HTTP_STATUS_PARTIAL             203 // partial completion
#define HTTP_STATUS_NO_CONTENT          204 // no info to return
#define HTTP_STATUS_RESET_CONTENT       205 // request completed, but clear form
#define HTTP_STATUS_PARTIAL_CONTENT     206 // partial GET fulfilled
#define HTTP_STATUS_WEBDAV_MULTI_STATUS 207 // WebDAV Multi-Status

#define HTTP_STATUS_AMBIGUOUS           300 // server couldn't decide what to return
#define HTTP_STATUS_MOVED               301 // object permanently moved
#define HTTP_STATUS_REDIRECT            302 // object temporarily moved
#define HTTP_STATUS_REDIRECT_METHOD     303 // redirection w/ new access method
#define HTTP_STATUS_NOT_MODIFIED        304 // if-modified-since was not modified
#define HTTP_STATUS_USE_PROXY           305 // redirection to proxy, location header specifies proxy to use
#define HTTP_STATUS_REDIRECT_KEEP_VERB  307 // HTTP/1.1: keep same verb
#define HTTP_STATUS_PERMANENT_REDIRECT  308 // Object permanently moved keep verb

#define HTTP_STATUS_BAD_REQUEST         400 // invalid syntax
#define HTTP_STATUS_DENIED              401 // access denied
#define HTTP_STATUS_PAYMENT_REQ         402 // payment required
#define HTTP_STATUS_FORBIDDEN           403 // request forbidden
#define HTTP_STATUS_NOT_FOUND           404 // object not found
#define HTTP_STATUS_BAD_METHOD          405 // method is not allowed
#define HTTP_STATUS_NONE_ACCEPTABLE     406 // no response acceptable to client found
#define HTTP_STATUS_PROXY_AUTH_REQ      407 // proxy authentication required
#define HTTP_STATUS_REQUEST_TIMEOUT     408 // server timed out waiting for request
#define HTTP_STATUS_CONFLICT            409 // user should resubmit with more info
#define HTTP_STATUS_GONE                410 // the resource is no longer available
#define HTTP_STATUS_LENGTH_REQUIRED     411 // the server refused to accept request w/o a length
#define HTTP_STATUS_PRECOND_FAILED      412 // precondition given in request failed
#define HTTP_STATUS_REQUEST_TOO_LARGE   413 // request entity was too large
#define HTTP_STATUS_URI_TOO_LONG        414 // request URI too long
#define HTTP_STATUS_UNSUPPORTED_MEDIA   415 // unsupported media type
#define HTTP_STATUS_RETRY_WITH          449 // retry after doing the appropriate action.

#define HTTP_STATUS_SERVER_ERROR        500 // internal server error
#define HTTP_STATUS_NOT_SUPPORTED       501 // required not supported
#define HTTP_STATUS_BAD_GATEWAY         502 // error response received from gateway
#define HTTP_STATUS_SERVICE_UNAVAIL     503 // temporarily overloaded
#define HTTP_STATUS_GATEWAY_TIMEOUT     504 // timed out waiting for gateway
#define HTTP_STATUS_VERSION_NOT_SUP     505 // HTTP version not supported

#define HTTP_STATUS_FIRST               HTTP_STATUS_CONTINUE
#define HTTP_STATUS_LAST                HTTP_STATUS_VERSION_NOT_SUP

//
// Flags for CrackUrl() and CombineUrl()
//

#define ICU_NO_ENCODE               0x20000000  // Don't convert unsafe characters to escape sequence
#define ICU_DECODE                  0x10000000  // Convert %XX escape sequences to characters
#define ICU_NO_META                 0x08000000  // Don't convert .. etc. meta path sequences
#define ICU_ENCODE_SPACES_ONLY      0x04000000  // Encode spaces only
#define ICU_BROWSER_MODE            0x02000000  // Special encode/decode rules for browser
#define ICU_ENCODE_PERCENT          0x00001000  // Encode any percent (ASCII25) signs encountered, default is to not encode percent.

//
// Flags for WinHttpCrackUrl() and WinHttpCreateUrl()
//

#define ICU_ESCAPE                  0x80000000  // (Un)escape URL characters
#define ICU_INCLUDE_DEFAULT_PORT    0x00008000  // Include default port numbers in URLs
#define ICU_REJECT_USERPWD          0x00004000  // Rejects URLs whick have username/pwd sections
#define ICU_ESCAPE_AUTHORITY        0x00002000  // Escape chars in authority components (user, pwd, host)

// WinHttpOpen dwAccessType values (also for WINHTTP_PROXY_INFO::dwAccessType)
#define WINHTTP_ACCESS_TYPE_DEFAULT_PROXY               0
#define WINHTTP_ACCESS_TYPE_NO_PROXY                    1
#define WINHTTP_ACCESS_TYPE_NAMED_PROXY                 3
#define WINHTTP_ACCESS_TYPE_AUTOMATIC_PROXY             4

// WinHttpOpen prettifiers for optional parameters
#define WINHTTP_NO_PROXY_NAME     NULL
#define WINHTTP_NO_PROXY_BYPASS   NULL

#define WINHTTP_NO_CLIENT_CERT_CONTEXT NULL

// WinHttpOpenRequest prettifers for optional parameters
#define WINHTTP_NO_REFERER             NULL
#define WINHTTP_DEFAULT_ACCEPT_TYPES   NULL

//
// Values for dwModifiers parameter of WinHttpAddRequestHeaders()
//

#define WINHTTP_ADDREQ_INDEX_MASK      0x0000FFFF
#define WINHTTP_ADDREQ_FLAGS_MASK      0xFFFF0000

//
// WINHTTP_ADDREQ_FLAG_ADD_IF_NEW - the header will only be added if it doesn't
// already exist
//

#define WINHTTP_ADDREQ_FLAG_ADD_IF_NEW 0x10000000

//
// WINHTTP_ADDREQ_FLAG_ADD - if WINHTTP_ADDREQ_FLAG_REPLACE is set but the header is
// not found then if this flag is set, the header is added anyway, so long as
// there is a valid header-value
//

#define WINHTTP_ADDREQ_FLAG_ADD        0x20000000

//
// WINHTTP_ADDREQ_FLAG_COALESCE - coalesce headers with same name. e.g.
// "Accept: text/*" and "Accept: audio/*" with this flag results in a single
// header: "Accept: text/*, audio/*"
//

#define WINHTTP_ADDREQ_FLAG_COALESCE_WITH_COMMA       0x40000000
#define WINHTTP_ADDREQ_FLAG_COALESCE_WITH_SEMICOLON   0x01000000
#define WINHTTP_ADDREQ_FLAG_COALESCE                  WINHTTP_ADDREQ_FLAG_COALESCE_WITH_COMMA

//
// WINHTTP_ADDREQ_FLAG_REPLACE - replaces the specified header. Only one header can
// be supplied in the buffer. If the header to be replaced is not the first
// in a list of headers with the same name, then the relative index should be
// supplied in the low 8 bits of the dwModifiers parameter. If the header-value
// part is missing, then the header is removed
//

#define WINHTTP_ADDREQ_FLAG_REPLACE    0x80000000

//
// values for ullFlags member of WINHTTP_EXTENDED_HEADER
//

//
// WINHTTP_EXTENDED_HEADER_FLAG_UNICODE - indicates the value of the request header
// is unicode.
//

#define WINHTTP_EXTENDED_HEADER_FLAG_UNICODE 0x00000001

//
// values for ullFlags for WinHttpReadDataEx
//

//
// WINHTTP_READ_DATA_EX_FLAG_FILL_BUFFER - if set, don't complete ReadDataEx
// until the data buffer has been filled or the response is complete.
//

#define WINHTTP_READ_DATA_EX_FLAG_FILL_BUFFER 0x0000000000000001ull


#define WINHTTP_IGNORE_REQUEST_TOTAL_LENGTH 0

// WinHttpSendRequest prettifiers for optional parameters.
#define WINHTTP_NO_ADDITIONAL_HEADERS   NULL
#define WINHTTP_NO_REQUEST_DATA         NULL

// WinHttpQueryHeaders prettifiers for optional parameters.
#define WINHTTP_HEADER_NAME_BY_INDEX           NULL
#define WINHTTP_NO_OUTPUT_BUFFER               NULL
#define WINHTTP_NO_HEADER_INDEX                NULL

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

typedef struct _WINHTTP_CURRENT_USER_IE_PROXY_CONFIG
{
    BOOL    fAutoDetect;
    LPWSTR  lpszAutoConfigUrl;
    LPWSTR  lpszProxy;
    LPWSTR  lpszProxyBypass;
} WINHTTP_CURRENT_USER_IE_PROXY_CONFIG, *PWINHTTP_CURRENT_USER_IE_PROXY_CONFIG;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

//#if !defined(_WINERROR_)

//
// WinHttp API error returns
//

#define WINHTTP_ERROR_BASE                     12000

#define ERROR_WINHTTP_OUT_OF_HANDLES           (WINHTTP_ERROR_BASE + 1)
#define ERROR_WINHTTP_TIMEOUT                  (WINHTTP_ERROR_BASE + 2)
#define ERROR_WINHTTP_INTERNAL_ERROR           (WINHTTP_ERROR_BASE + 4)
#define ERROR_WINHTTP_INVALID_URL              (WINHTTP_ERROR_BASE + 5)
#define ERROR_WINHTTP_UNRECOGNIZED_SCHEME      (WINHTTP_ERROR_BASE + 6)
#define ERROR_WINHTTP_NAME_NOT_RESOLVED        (WINHTTP_ERROR_BASE + 7)
#define ERROR_WINHTTP_INVALID_OPTION           (WINHTTP_ERROR_BASE + 9)
#define ERROR_WINHTTP_OPTION_NOT_SETTABLE      (WINHTTP_ERROR_BASE + 11)
#define ERROR_WINHTTP_SHUTDOWN                 (WINHTTP_ERROR_BASE + 12)


#define ERROR_WINHTTP_LOGIN_FAILURE            (WINHTTP_ERROR_BASE + 15)
#define ERROR_WINHTTP_OPERATION_CANCELLED      (WINHTTP_ERROR_BASE + 17)
#define ERROR_WINHTTP_INCORRECT_HANDLE_TYPE    (WINHTTP_ERROR_BASE + 18)
#define ERROR_WINHTTP_INCORRECT_HANDLE_STATE   (WINHTTP_ERROR_BASE + 19)
#define ERROR_WINHTTP_CANNOT_CONNECT           (WINHTTP_ERROR_BASE + 29)
#define ERROR_WINHTTP_CONNECTION_ERROR         (WINHTTP_ERROR_BASE + 30)
#define ERROR_WINHTTP_RESEND_REQUEST           (WINHTTP_ERROR_BASE + 32)

#define ERROR_WINHTTP_CLIENT_AUTH_CERT_NEEDED  (WINHTTP_ERROR_BASE + 44)


//
// WinHttpRequest Component errors
//
#define ERROR_WINHTTP_CANNOT_CALL_BEFORE_OPEN	(WINHTTP_ERROR_BASE + 100)
#define ERROR_WINHTTP_CANNOT_CALL_BEFORE_SEND	(WINHTTP_ERROR_BASE + 101)
#define ERROR_WINHTTP_CANNOT_CALL_AFTER_SEND	(WINHTTP_ERROR_BASE + 102)
#define ERROR_WINHTTP_CANNOT_CALL_AFTER_OPEN	(WINHTTP_ERROR_BASE + 103)


//
// HTTP API errors
//

#define ERROR_WINHTTP_HEADER_NOT_FOUND             (WINHTTP_ERROR_BASE + 150)
#define ERROR_WINHTTP_INVALID_SERVER_RESPONSE      (WINHTTP_ERROR_BASE + 152)
#define ERROR_WINHTTP_INVALID_HEADER               (WINHTTP_ERROR_BASE + 153)
#define ERROR_WINHTTP_INVALID_QUERY_REQUEST        (WINHTTP_ERROR_BASE + 154)
#define ERROR_WINHTTP_HEADER_ALREADY_EXISTS        (WINHTTP_ERROR_BASE + 155)
#define ERROR_WINHTTP_REDIRECT_FAILED              (WINHTTP_ERROR_BASE + 156)



//
// additional WinHttp API error codes
//

//
// additional WinHttp API error codes
//

#define ERROR_WINHTTP_AUTO_PROXY_SERVICE_ERROR  (WINHTTP_ERROR_BASE + 178)
#define ERROR_WINHTTP_BAD_AUTO_PROXY_SCRIPT     (WINHTTP_ERROR_BASE + 166)
#define ERROR_WINHTTP_UNABLE_TO_DOWNLOAD_SCRIPT (WINHTTP_ERROR_BASE + 167)
#define ERROR_WINHTTP_UNHANDLED_SCRIPT_TYPE     (WINHTTP_ERROR_BASE + 176)
#define ERROR_WINHTTP_SCRIPT_EXECUTION_ERROR    (WINHTTP_ERROR_BASE + 177)

#define ERROR_WINHTTP_NOT_INITIALIZED          (WINHTTP_ERROR_BASE + 172)
#define ERROR_WINHTTP_SECURE_FAILURE           (WINHTTP_ERROR_BASE + 175)


//
// Certificate security errors. These are raised only by the WinHttpRequest
// component. The WinHTTP Win32 API will return ERROR_WINHTTP_SECURE_FAILE and
// provide additional information via the WINHTTP_CALLBACK_STATUS_SECURE_FAILURE
// callback notification.
//
#define ERROR_WINHTTP_SECURE_CERT_DATE_INVALID    (WINHTTP_ERROR_BASE + 37)
#define ERROR_WINHTTP_SECURE_CERT_CN_INVALID      (WINHTTP_ERROR_BASE + 38)
#define ERROR_WINHTTP_SECURE_INVALID_CA           (WINHTTP_ERROR_BASE + 45)
#define ERROR_WINHTTP_SECURE_CERT_REV_FAILED      (WINHTTP_ERROR_BASE + 57)
#define ERROR_WINHTTP_SECURE_CHANNEL_ERROR        (WINHTTP_ERROR_BASE + 157)
#define ERROR_WINHTTP_SECURE_INVALID_CERT         (WINHTTP_ERROR_BASE + 169)
#define ERROR_WINHTTP_SECURE_CERT_REVOKED         (WINHTTP_ERROR_BASE + 170)
#define ERROR_WINHTTP_SECURE_CERT_WRONG_USAGE     (WINHTTP_ERROR_BASE + 179)


#define ERROR_WINHTTP_AUTODETECTION_FAILED                  (WINHTTP_ERROR_BASE + 180)
#define ERROR_WINHTTP_HEADER_COUNT_EXCEEDED                 (WINHTTP_ERROR_BASE + 181)
#define ERROR_WINHTTP_HEADER_SIZE_OVERFLOW                  (WINHTTP_ERROR_BASE + 182)
#define ERROR_WINHTTP_CHUNKED_ENCODING_HEADER_SIZE_OVERFLOW (WINHTTP_ERROR_BASE + 183)
#define ERROR_WINHTTP_RESPONSE_DRAIN_OVERFLOW               (WINHTTP_ERROR_BASE + 184)
#define ERROR_WINHTTP_CLIENT_CERT_NO_PRIVATE_KEY            (WINHTTP_ERROR_BASE + 185)
#define ERROR_WINHTTP_CLIENT_CERT_NO_ACCESS_PRIVATE_KEY     (WINHTTP_ERROR_BASE + 186)

#define ERROR_WINHTTP_CLIENT_AUTH_CERT_NEEDED_PROXY         (WINHTTP_ERROR_BASE + 187)
#define ERROR_WINHTTP_SECURE_FAILURE_PROXY                  (WINHTTP_ERROR_BASE + 188)
#define ERROR_WINHTTP_RESERVED_189                          (WINHTTP_ERROR_BASE + 189)
#define ERROR_WINHTTP_HTTP_PROTOCOL_MISMATCH                (WINHTTP_ERROR_BASE + 190)
#define ERROR_WINHTTP_GLOBAL_CALLBACK_FAILED                (WINHTTP_ERROR_BASE + 191)
#define ERROR_WINHTTP_FEATURE_DISABLED                      (WINHTTP_ERROR_BASE + 192)

#define ERROR_WINHTTP_FAST_FORWARDING_NOT_SUPPORTED         (WINHTTP_ERROR_BASE + 193)


#define WINHTTP_ERROR_LAST                                  ERROR_WINHTTP_FAST_FORWARDING_NOT_SUPPORTED

#define WINHTTP_RESET_STATE                     0x00000001
#define WINHTTP_RESET_SWPAD_CURRENT_NETWORK     0x00000002
#define WINHTTP_RESET_SWPAD_ALL                 0x00000004
#define WINHTTP_RESET_SCRIPT_CACHE              0x00000008
#define WINHTTP_RESET_ALL                       0x0000FFFF
#define WINHTTP_RESET_NOTIFY_NETWORK_CHANGED    0x00010000
#define WINHTTP_RESET_OUT_OF_PROC               0x00020000
#define WINHTTP_RESET_DISCARD_RESOLVERS         0x00040000


//#endif // !defined(_WINERROR_)


//
// prototypes
//

WINHTTPAPI
WINHTTP_STATUS_CALLBACK
WINAPI
WinHttpSetStatusCallback
(
    IN HINTERNET hInternet,
    IN WINHTTP_STATUS_CALLBACK lpfnInternetCallback,
    IN DWORD dwNotificationFlags,
    IN DWORD_PTR dwReserved
);

WINHTTPAPI
BOOL
WINAPI
WinHttpTimeFromSystemTime
(
    _In_ CONST SYSTEMTIME *pst,  // input GMT time
    _Out_writes_bytes_(WINHTTP_TIME_FORMAT_BUFSIZE) LPWSTR pwszTime // output string buffer
);

WINHTTPAPI
BOOL
WINAPI
WinHttpTimeToSystemTime
(
    _In_z_ LPCWSTR pwszTime,        // NULL terminated string
    _Out_ SYSTEMTIME *pst           // output in GMT time
);

WINHTTPAPI
BOOL
WINAPI
WinHttpCrackUrl
(
    _In_reads_(dwUrlLength) LPCWSTR pwszUrl,
    _In_ DWORD dwUrlLength,
    _In_ DWORD dwFlags,
    _Inout_ LPURL_COMPONENTS lpUrlComponents
);

_Success_(return != FALSE)
WINHTTPAPI
BOOL
WINAPI
WinHttpCreateUrl
(
    _In_ LPURL_COMPONENTS lpUrlComponents,
    _In_ DWORD dwFlags,
    _Out_writes_to_opt_(*pdwUrlLength, *pdwUrlLength) LPWSTR pwszUrl,
    _Inout_ LPDWORD pdwUrlLength
);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

WINHTTPAPI BOOL WINAPI WinHttpCheckPlatform(void);

WINHTTPAPI BOOL WINAPI WinHttpGetDefaultProxyConfiguration( IN OUT WINHTTP_PROXY_INFO * pProxyInfo);
WINHTTPAPI BOOL WINAPI WinHttpSetDefaultProxyConfiguration( IN WINHTTP_PROXY_INFO * pProxyInfo);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

WINHTTPAPI
HINTERNET
WINAPI
WinHttpOpen
(
    _In_opt_z_ LPCWSTR pszAgentW,
    _In_ DWORD dwAccessType,
    _In_opt_z_ LPCWSTR pszProxyW,
    _In_opt_z_ LPCWSTR pszProxyBypassW,
    _In_ DWORD dwFlags
);

WINHTTPAPI
BOOL
WINAPI
WinHttpCloseHandle
(
    IN HINTERNET hInternet
);

WINHTTPAPI
HINTERNET
WINAPI
WinHttpConnect
(
    IN HINTERNET hSession,
    IN LPCWSTR pswzServerName,
    IN INTERNET_PORT nServerPort,
    IN DWORD dwReserved
);

WINHTTPAPI
BOOL
WINAPI
WinHttpReadData
(
    IN HINTERNET hRequest,
    _Out_writes_bytes_to_(dwNumberOfBytesToRead, *lpdwNumberOfBytesRead) __out_data_source(NETWORK) LPVOID lpBuffer,
    IN DWORD dwNumberOfBytesToRead,
    OUT LPDWORD lpdwNumberOfBytesRead
);

WINHTTPAPI
DWORD
WINAPI
WinHttpReadDataEx
(
    IN HINTERNET hRequest,
    _Out_writes_bytes_to_(dwNumberOfBytesToRead, *lpdwNumberOfBytesRead) __out_data_source(NETWORK) LPVOID lpBuffer,
    IN DWORD dwNumberOfBytesToRead,
    OUT LPDWORD lpdwNumberOfBytesRead,
    IN ULONGLONG ullFlags,
    IN DWORD cbProperty,
    _In_reads_bytes_opt_(cbProperty) PVOID pvProperty
);

WINHTTPAPI
BOOL
WINAPI
WinHttpWriteData
(
    IN HINTERNET hRequest,
    _In_reads_bytes_opt_(dwNumberOfBytesToWrite) LPCVOID lpBuffer,
    IN DWORD dwNumberOfBytesToWrite,
    OUT LPDWORD lpdwNumberOfBytesWritten
);

WINHTTPAPI
BOOL
WINAPI
WinHttpQueryDataAvailable
(
    IN HINTERNET hRequest,
    __out_data_source(NETWORK) LPDWORD lpdwNumberOfBytesAvailable
);

_Success_(return != FALSE)
WINHTTPAPI
BOOL
WINAPI
WinHttpQueryOption
(
    IN HINTERNET hInternet,
    IN DWORD dwOption,
    _Out_writes_bytes_to_opt_(*lpdwBufferLength, *lpdwBufferLength) __out_data_source(NETWORK) LPVOID lpBuffer,
    IN OUT LPDWORD lpdwBufferLength
);

WINHTTPAPI
BOOL
WINAPI
WinHttpSetOption
(
    _In_opt_ HINTERNET hInternet,
    _In_ DWORD dwOption,
    _When_((dwOption == WINHTTP_OPTION_USERNAME ||
            dwOption == WINHTTP_OPTION_PASSWORD ||
            dwOption == WINHTTP_OPTION_PROXY_USERNAME ||
            dwOption == WINHTTP_OPTION_PROXY_PASSWORD ||
            dwOption == WINHTTP_OPTION_USER_AGENT),
           _At_((LPCWSTR)lpBuffer, _In_reads_(dwBufferLength)))
    _When_((dwOption == WINHTTP_OPTION_CLIENT_CERT_CONTEXT),
           _In_reads_bytes_opt_(dwBufferLength))
    _When_((dwOption != WINHTTP_OPTION_USERNAME &&
            dwOption != WINHTTP_OPTION_PASSWORD &&
            dwOption != WINHTTP_OPTION_PROXY_USERNAME &&
            dwOption != WINHTTP_OPTION_PROXY_PASSWORD &&
            dwOption != WINHTTP_OPTION_CLIENT_CERT_CONTEXT &&
            dwOption != WINHTTP_OPTION_USER_AGENT),
           _In_reads_bytes_(dwBufferLength))
    LPVOID lpBuffer,
    _In_ DWORD dwBufferLength
);

WINHTTPAPI
BOOL
WINAPI
WinHttpSetTimeouts
(
    IN HINTERNET    hInternet,           // Session/Request handle.
    IN int          nResolveTimeout,
    IN int          nConnectTimeout,
    IN int          nSendTimeout,
    IN int          nReceiveTimeout
);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

WINHTTPAPI
DWORD
WINAPI
WinHttpIsHostInProxyBypassList
(
    _In_ const WINHTTP_PROXY_INFO *pProxyInfo,
    _In_z_ PCWSTR pwszHost,
    _In_ INTERNET_SCHEME tScheme,
    _In_ INTERNET_PORT nPort,
    _Out_ BOOL *pfIsInBypassList
);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

//
// prototypes
//

WINHTTPAPI
HINTERNET
WINAPI
WinHttpOpenRequest
(
    IN HINTERNET hConnect,
    IN LPCWSTR pwszVerb,
    IN LPCWSTR pwszObjectName,
    IN LPCWSTR pwszVersion,
    IN LPCWSTR pwszReferrer OPTIONAL,
    IN LPCWSTR FAR * ppwszAcceptTypes OPTIONAL,
    IN DWORD dwFlags
);

WINHTTPAPI
BOOL
WINAPI
WinHttpAddRequestHeaders
(
    IN HINTERNET hRequest,
    _When_(dwHeadersLength == (DWORD)-1, _In_z_)
    _When_(dwHeadersLength != (DWORD)-1, _In_reads_(dwHeadersLength))
    LPCWSTR lpszHeaders,
    IN DWORD dwHeadersLength,
    IN DWORD dwModifiers
);

WINHTTPAPI
DWORD
WINAPI
WinHttpAddRequestHeadersEx
(
    IN HINTERNET hRequest,
    IN DWORD dwModifiers,
    IN ULONGLONG ullFlags,
    IN ULONGLONG ullExtra,
    IN DWORD cHeaders,
    _In_reads_(cHeaders) WINHTTP_EXTENDED_HEADER *pHeaders
);

WINHTTPAPI
BOOL
WINAPI
WinHttpSendRequest
(
    IN HINTERNET hRequest,
    _In_reads_opt_(dwHeadersLength) LPCWSTR lpszHeaders,
    IN DWORD dwHeadersLength,
    _In_reads_bytes_opt_(dwOptionalLength) LPVOID lpOptional,
    IN DWORD dwOptionalLength,
    IN DWORD dwTotalLength,
    IN DWORD_PTR dwContext
);

WINHTTPAPI
BOOL
WINAPI
WinHttpSetCredentials
(
    IN HINTERNET   hRequest,        // HINTERNET handle returned by WinHttpOpenRequest.
    IN DWORD       AuthTargets,     // Only WINHTTP_AUTH_TARGET_SERVER and
                                    // WINHTTP_AUTH_TARGET_PROXY are supported
                                    // in this version and they are mutually
                                    // exclusive
    IN DWORD       AuthScheme,      // must be one of the supported Auth Schemes
                                    // returned from WinHttpQueryAuthSchemes()
    IN LPCWSTR     pwszUserName,    // 1) NULL if default creds is to be used, in
                                    // which case pszPassword will be ignored
    IN LPCWSTR     pwszPassword,    // 1) "" == Blank Password; 2)Parameter ignored
                                    // if pszUserName is NULL; 3) Invalid to pass in
                                    // NULL if pszUserName is not NULL
    IN LPVOID      pAuthParams
);

WINHTTPAPI
BOOL
WINAPI
WinHttpQueryAuthSchemes
(
    IN  HINTERNET   hRequest,             // HINTERNET handle returned by WinHttpOpenRequest
    OUT LPDWORD     lpdwSupportedSchemes, // a bitmap of available Authentication Schemes
    OUT LPDWORD     lpdwFirstScheme,      // returns the first auth scheme returned by the server
    OUT LPDWORD     pdwAuthTarget
);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

WINHTTPAPI
BOOL
WINAPI
WinHttpQueryAuthParams
(
    IN  HINTERNET   hRequest,        // HINTERNET handle returned by WinHttpOpenRequest
    IN  DWORD       AuthScheme,
    OUT LPVOID*     pAuthParams      // Scheme-specific Advanced auth parameters
);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

WINHTTPAPI
BOOL
WINAPI
WinHttpReceiveResponse
(
    IN HINTERNET hRequest,
    IN LPVOID lpReserved
);

_Success_(return != FALSE)
WINHTTPAPI
BOOL
WINAPI
WinHttpQueryHeaders
(
    IN     HINTERNET hRequest,
    IN     DWORD     dwInfoLevel,
    IN     LPCWSTR   pwszName OPTIONAL,
    _Out_writes_bytes_to_opt_(*lpdwBufferLength, *lpdwBufferLength) __out_data_source(NETWORK) LPVOID lpBuffer,
    IN OUT LPDWORD   lpdwBufferLength,
    IN OUT LPDWORD   lpdwIndex OPTIONAL
);

WINHTTPAPI
DWORD
WINAPI
WinHttpQueryHeadersEx
(
    _In_ HINTERNET hRequest,
    _In_ DWORD dwInfoLevel,
    _In_ ULONGLONG ullFlags,
    _In_ UINT uiCodePage,
    _Inout_opt_ PDWORD pdwIndex,
    _In_opt_ PWINHTTP_HEADER_NAME pHeaderName,
    _Out_writes_bytes_to_opt_(*pdwBufferLength, *pdwBufferLength) PVOID pBuffer,
    _Inout_ PDWORD pdwBufferLength,
    _Out_writes_opt_(*pdwHeadersCount) PWINHTTP_EXTENDED_HEADER *ppHeaders,
    _Out_ PDWORD pdwHeadersCount
);

WINHTTPAPI
DWORD
WINAPI
WinHttpQueryConnectionGroup
(
    _In_ HINTERNET hInternet,
    _In_opt_ const GUID *pGuidConnection,
    _In_ ULONGLONG ullFlags,
    _Inout_ PWINHTTP_QUERY_CONNECTION_GROUP_RESULT *ppResult
);

WINHTTPAPI
VOID
WINAPI
WinHttpFreeQueryConnectionGroupResult
(
    _Inout_ WINHTTP_QUERY_CONNECTION_GROUP_RESULT *pResult
);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

WINHTTPAPI
BOOL
WINAPI
WinHttpDetectAutoProxyConfigUrl
(
    DWORD dwAutoDetectFlags,
    _Outptr_result_maybenull_ LPWSTR * ppwstrAutoConfigUrl
);

WINHTTPAPI
BOOL
WINAPI
WinHttpGetProxyForUrl
(
    IN  HINTERNET                   hSession,
    IN  LPCWSTR                     lpcwszUrl,
    IN  WINHTTP_AUTOPROXY_OPTIONS * pAutoProxyOptions,
    OUT WINHTTP_PROXY_INFO *        pProxyInfo
);

WINHTTPAPI
DWORD
WINAPI
WinHttpCreateProxyResolver
(
    _In_ HINTERNET hSession,
    _Out_ HINTERNET *phResolver
);

WINHTTPAPI
DWORD
WINAPI
WinHttpGetProxyForUrlEx
(
    _In_ HINTERNET hResolver,
    _In_ PCWSTR pcwszUrl,
    _In_ WINHTTP_AUTOPROXY_OPTIONS *pAutoProxyOptions,
    _In_opt_ DWORD_PTR pContext
);

WINHTTPAPI
DWORD
WINAPI
WinHttpGetProxyForUrlEx2
(
    _In_ HINTERNET hResolver,
    _In_ PCWSTR pcwszUrl,
    _In_ WINHTTP_AUTOPROXY_OPTIONS *pAutoProxyOptions,
    _In_ DWORD cbInterfaceSelectionContext,
    _In_reads_bytes_opt_(cbInterfaceSelectionContext) BYTE *pInterfaceSelectionContext,
    _In_opt_ DWORD_PTR pContext
);

WINHTTPAPI
DWORD
WINAPI
WinHttpGetProxyResult
(
    _In_ HINTERNET hResolver,
    _Out_ WINHTTP_PROXY_RESULT *pProxyResult
);

WINHTTPAPI
DWORD
WINAPI
WinHttpGetProxyResultEx
(
    _In_ HINTERNET hResolver,
    _Out_ WINHTTP_PROXY_RESULT_EX *pProxyResultEx
);

WINHTTPAPI
VOID
WINAPI
WinHttpFreeProxyResult
(
    _Inout_ WINHTTP_PROXY_RESULT *pProxyResult
);

WINHTTPAPI
VOID
WINAPI
WinHttpFreeProxyResultEx
(
    _Inout_ WINHTTP_PROXY_RESULT_EX *pProxyResultEx
);

WINHTTPAPI
DWORD
WINAPI
WinHttpResetAutoProxy
(
    _In_ HINTERNET hSession,
    _In_ DWORD dwFlags
);

WINHTTPAPI
BOOL
WINAPI
WinHttpGetIEProxyConfigForCurrentUser
(
    IN OUT WINHTTP_CURRENT_USER_IE_PROXY_CONFIG * pProxyConfig
);

WINHTTPAPI
DWORD
WINAPI
WinHttpWriteProxySettings(
    _In_ HINTERNET hSession,
    _In_ BOOL fForceUpdate,
    _In_ WINHTTP_PROXY_SETTINGS *pWinHttpProxySettings
);

WINHTTPAPI
DWORD
WINAPI
WinHttpReadProxySettings(
    _In_ HINTERNET hSession,
    _In_opt_ PCWSTR pcwszConnectionName,
    _In_ BOOL fFallBackToDefaultSettings,
    _In_ BOOL fSetAutoDiscoverForDefaultSettings,
    _Out_ DWORD *pdwSettingsVersion,
    _Out_ BOOL *pfDefaultSettingsAreReturned,
    _Out_ WINHTTP_PROXY_SETTINGS *pWinHttpProxySettings
);

WINHTTPAPI
VOID
WINAPI
WinHttpFreeProxySettings(
    _In_ WINHTTP_PROXY_SETTINGS *pWinHttpProxySettings
);

WINHTTPAPI
DWORD
WINAPI
WinHttpGetProxySettingsVersion(
    _In_ HINTERNET hSession,
    _Out_ DWORD *pdwProxySettingsVersion
);

WINHTTPAPI
DWORD
WINAPI
WinHttpSetProxySettingsPerUser(
    _In_ BOOL fProxySettingsPerUser
);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

typedef enum _WINHTTP_WEB_SOCKET_OPERATION
{
    WINHTTP_WEB_SOCKET_SEND_OPERATION                   = 0,
    WINHTTP_WEB_SOCKET_RECEIVE_OPERATION                = 1,
    WINHTTP_WEB_SOCKET_CLOSE_OPERATION                  = 2,
    WINHTTP_WEB_SOCKET_SHUTDOWN_OPERATION               = 3
} WINHTTP_WEB_SOCKET_OPERATION;

typedef enum _WINHTTP_WEB_SOCKET_BUFFER_TYPE
{
    WINHTTP_WEB_SOCKET_BINARY_MESSAGE_BUFFER_TYPE       = 0,
    WINHTTP_WEB_SOCKET_BINARY_FRAGMENT_BUFFER_TYPE      = 1,
    WINHTTP_WEB_SOCKET_UTF8_MESSAGE_BUFFER_TYPE         = 2,
    WINHTTP_WEB_SOCKET_UTF8_FRAGMENT_BUFFER_TYPE        = 3,
    WINHTTP_WEB_SOCKET_CLOSE_BUFFER_TYPE                = 4
} WINHTTP_WEB_SOCKET_BUFFER_TYPE;

typedef enum _WINHTTP_WEB_SOCKET_CLOSE_STATUS
{
    WINHTTP_WEB_SOCKET_SUCCESS_CLOSE_STATUS                = 1000,
    WINHTTP_WEB_SOCKET_ENDPOINT_TERMINATED_CLOSE_STATUS    = 1001,
    WINHTTP_WEB_SOCKET_PROTOCOL_ERROR_CLOSE_STATUS         = 1002,
    WINHTTP_WEB_SOCKET_INVALID_DATA_TYPE_CLOSE_STATUS      = 1003,
    WINHTTP_WEB_SOCKET_EMPTY_CLOSE_STATUS                  = 1005,
    WINHTTP_WEB_SOCKET_ABORTED_CLOSE_STATUS                = 1006,
    WINHTTP_WEB_SOCKET_INVALID_PAYLOAD_CLOSE_STATUS        = 1007,
    WINHTTP_WEB_SOCKET_POLICY_VIOLATION_CLOSE_STATUS       = 1008,
    WINHTTP_WEB_SOCKET_MESSAGE_TOO_BIG_CLOSE_STATUS        = 1009,
    WINHTTP_WEB_SOCKET_UNSUPPORTED_EXTENSIONS_CLOSE_STATUS = 1010,
    WINHTTP_WEB_SOCKET_SERVER_ERROR_CLOSE_STATUS           = 1011,
    WINHTTP_WEB_SOCKET_SECURE_HANDSHAKE_ERROR_CLOSE_STATUS = 1015
} WINHTTP_WEB_SOCKET_CLOSE_STATUS;

typedef struct _WINHTTP_WEB_SOCKET_ASYNC_RESULT
{
    WINHTTP_ASYNC_RESULT AsyncResult;
    WINHTTP_WEB_SOCKET_OPERATION Operation;
} WINHTTP_WEB_SOCKET_ASYNC_RESULT;

typedef struct _WINHTTP_WEB_SOCKET_STATUS
{
    DWORD dwBytesTransferred;
    WINHTTP_WEB_SOCKET_BUFFER_TYPE eBufferType;
} WINHTTP_WEB_SOCKET_STATUS;

#define WINHTTP_WEB_SOCKET_MAX_CLOSE_REASON_LENGTH 123
#define WINHTTP_WEB_SOCKET_MIN_KEEPALIVE_VALUE 15000

WINHTTPAPI
HINTERNET
WINAPI
WinHttpWebSocketCompleteUpgrade
(
    _In_ HINTERNET hRequest,
    _In_opt_ DWORD_PTR pContext
);

WINHTTPAPI
DWORD
WINAPI
WinHttpWebSocketSend
(
    _In_ HINTERNET hWebSocket,
    _In_ WINHTTP_WEB_SOCKET_BUFFER_TYPE eBufferType,
    _In_reads_opt_(dwBufferLength) PVOID pvBuffer,
    _In_ DWORD dwBufferLength
);

WINHTTPAPI
DWORD
WINAPI
WinHttpWebSocketReceive
(
    _In_ HINTERNET hWebSocket,
    _Out_writes_bytes_to_(dwBufferLength, *pdwBytesRead) PVOID pvBuffer,
    _In_ DWORD dwBufferLength,
    _Out_range_(0, dwBufferLength) DWORD *pdwBytesRead,
    _Out_ WINHTTP_WEB_SOCKET_BUFFER_TYPE *peBufferType
);

WINHTTPAPI
DWORD
WINAPI
WinHttpWebSocketShutdown
(
    _In_ HINTERNET hWebSocket,
    _In_ USHORT usStatus,
    _In_reads_bytes_opt_(dwReasonLength) PVOID pvReason,
    _In_range_(0, WINHTTP_WEB_SOCKET_MAX_CLOSE_REASON_LENGTH) DWORD dwReasonLength
);

WINHTTPAPI
DWORD
WINAPI
WinHttpWebSocketClose
(
    _In_ HINTERNET hWebSocket,
    _In_ USHORT usStatus,
    _In_reads_bytes_opt_(dwReasonLength) PVOID pvReason,
    _In_range_(0, WINHTTP_WEB_SOCKET_MAX_CLOSE_REASON_LENGTH) DWORD dwReasonLength
);

WINHTTPAPI
DWORD
WINAPI
WinHttpWebSocketQueryCloseStatus
(
    _In_ HINTERNET hWebSocket,
    _Out_ USHORT *pusStatus,
    _Out_writes_bytes_to_opt_(dwReasonLength, *pdwReasonLengthConsumed) PVOID pvReason,
    _In_range_(0, WINHTTP_WEB_SOCKET_MAX_CLOSE_REASON_LENGTH) DWORD dwReasonLength,
    _Out_range_(0, WINHTTP_WEB_SOCKET_MAX_CLOSE_REASON_LENGTH) DWORD *pdwReasonLengthConsumed
);

typedef enum _WINHTTP_PROTOCOL_OPERATION
{
    WINHTTP_PROTOCOL_SEND_OPERATION                   = 0,
    WINHTTP_PROTOCOL_RECEIVE_OPERATION                = 1,
} WINHTTP_PROTOCOL_OPERATION;

typedef struct _WINHTTP_PROTOCOL_ASYNC_RESULT
{
    WINHTTP_ASYNC_RESULT AsyncResult;
    WINHTTP_PROTOCOL_OPERATION Operation;
} WINHTTP_PROTOCOL_ASYNC_RESULT;

WINHTTPAPI
HINTERNET
WINAPI
WinHttpProtocolCompleteUpgrade
(
    _In_ HINTERNET hRequest,
    _In_opt_ DWORD_PTR dwContext
);

WINHTTPAPI
DWORD
WINAPI
WinHttpProtocolSend
(
    _In_ HINTERNET ProtocolHandle,
    _In_ ULONGLONG Flags,
    _In_reads_opt_(dwBufferLength) PVOID pvBuffer,
    _In_ DWORD dwBufferLength
);

WINHTTPAPI
DWORD
WINAPI
WinHttpProtocolReceive
(
    _In_ HINTERNET ProtocolHandle,
    _In_ ULONGLONG Flags,
    _Out_writes_bytes_to_(dwBufferLength, *pdwBytesRead) PVOID pvBuffer,
    _In_ DWORD dwBufferLength,
    _Out_range_(0, dwBufferLength) DWORD *pdwBytesRead
);


//
// Proxy change notification APIs
//

typedef PVOID WINHTTP_PROXY_CHANGE_REGISTRATION_HANDLE;

typedef
VOID
(CALLBACK *WINHTTP_PROXY_CHANGE_CALLBACK)(
    _In_ ULONGLONG ullFlags,
    _In_ PVOID pvContext
);

#define WINHTTP_PROXY_NOTIFY_CHANGE 0x0001

WINHTTPAPI
DWORD
WINAPI
WinHttpRegisterProxyChangeNotification(
    _In_ ULONGLONG ullFlags,
    _In_ WINHTTP_PROXY_CHANGE_CALLBACK pfnCallback,
    _In_ PVOID pvContext,
    _Out_ WINHTTP_PROXY_CHANGE_REGISTRATION_HANDLE *hRegistration
);

WINHTTPAPI
DWORD
WINAPI
WinHttpUnregisterProxyChangeNotification(
    _In_ WINHTTP_PROXY_CHANGE_REGISTRATION_HANDLE hRegistration
);

//
// Extended proxy settings APIs
//

typedef enum _WINHTTP_PROXY_SETTINGS_TYPE
{
    WinHttpProxySettingsTypeUnknown,
    WinHttpProxySettingsTypeWsl,
    WinHttpProxySettingsTypeWsa,
    WinHttpProxySettingsTypeXBox
} WINHTTP_PROXY_SETTINGS_TYPE, *PWINHTTP_PROXY_SETTINGS_TYPE;


typedef struct _WINHTTP_PROXY_SETTINGS_EX
{
    ULONGLONG ullGenerationId;
    ULONGLONG ullFlags;

    PCWSTR pcwszAutoconfigUrl;
    PCWSTR pcwszProxy;
    PCWSTR pcwszSecureProxy;

    DWORD cProxyBypasses;
    PCWSTR *rgpcwszProxyBypasses;

    DWORD dwInterfaceIndex;
    PCWSTR pcwszConnectionName;
} WINHTTP_PROXY_SETTINGS_EX, *PWINHTTP_PROXY_SETTINGS_EX;

typedef struct _WINHTTP_PROXY_SETTINGS_PARAM
{
    ULONGLONG ullFlags;

    PCWSTR pcwszConnectionName;
    PCWSTR pcwszProbeHost;
} WINHTTP_PROXY_SETTINGS_PARAM, *PWINHTTP_PROXY_SETTINGS_PARAM;

WINHTTPAPI
DWORD
WINAPI
WinHttpGetProxySettingsEx(
    _In_ HINTERNET hResolver,
    _In_ WINHTTP_PROXY_SETTINGS_TYPE ProxySettingsType,
    _In_opt_ PWINHTTP_PROXY_SETTINGS_PARAM pProxySettingsParam,
    _In_opt_ DWORD_PTR pContext
);

WINHTTPAPI
DWORD
WINAPI
WinHttpGetProxySettingsResultEx(
    _In_ HINTERNET hResolver,
    _Out_ PVOID pProxySettingsEx
);

WINHTTPAPI
DWORD
WINAPI
WinHttpFreeProxySettingsEx(
    _In_ WINHTTP_PROXY_SETTINGS_TYPE ProxySettingsType,
    _In_ PVOID pProxySettingsEx
);

typedef enum _WINHTTP_FAST_FORWARDING_STATE
{
    WinHttpFastForwardingStateInProgress          = 0,
    WinHttpFastForwardingStateSucceeded           = 1,
    WinHttpFastForwardingStateClientSideFailed    = 2,
    WinHttpFastForwardingStateServerSideFailed    = 3
} WINHTTP_FAST_FORWARDING_STATE, *PWINHTTP_FAST_FORWARDING_STATE;

typedef struct _WINHTTP_FAST_FORWARDING_STATUS
{
    WINHTTP_FAST_FORWARDING_STATE TransferState;
    LONG NtStatus; // This is NTSTATUS.
    DWORD dwError;
    ULONGLONG ullBytesTransferred;
} WINHTTP_FAST_FORWARDING_STATUS, *PWINHTTP_FAST_FORWARDING_STATUS;

//
// Feature IDs for WINHTTP_OPTION_FEATURE_SUPPORTED
//


#define WINHTTP_FEATURE_DISABLE_STREAM_QUEUE                                 1
#define WINHTTP_FEATURE_IPV6_FAST_FALLBACK                                   2
#define WINHTTP_FEATURE_CONNECTION_STATS_V0                                  3
#define WINHTTP_FEATURE_REQUEST_TIMES                                        4
#define WINHTTP_FEATURE_EXPIRE_CONNECTION                                    5
#define WINHTTP_FEATURE_DISABLE_SECURE_PROTOCOL_FALLBACK                     6
#define WINHTTP_FEATURE_HTTP_PROTOCOL_REQUIRED                               7
#define WINHTTP_FEATURE_REQUEST_STATS                                        8
#define WINHTTP_FEATURE_SERVER_CERT_CHAIN_CONTEXT                            9


#define WINHTTP_FEATURE_CONNECTION_STATS_V1                                  12
#define WINHTTP_FEATURE_SECURITY_INFO                                        13
#define WINHTTP_FEATURE_TCP_KEEPALIVE                                        14
#define WINHTTP_FEATURE_TCP_FAST_OPEN                                        15
#define WINHTTP_FEATURE_TLS_FALSE_START                                      16
#define WINHTTP_FEATURE_IGNORE_CERT_REVOCATION_OFFLINE                       17


#define WINHTTP_FEATURE_TLS_PROTOCOL_INSECURE_FALLBACK                       20
#define WINHTTP_FEATURE_STREAM_ERROR_CODE                                    21
#define WINHTTP_FEATURE_REQUIRE_STREAM_END                                   22
#define WINHTTP_FEATURE_ENABLE_HTTP2_PLUS_CLIENT_CERT                        23
#define WINHTTP_FEATURE_FAILED_CONNECTION_RETRIES                            24


#define WINHTTP_FEATURE_HTTP2_KEEPALIVE                                      26
#define WINHTTP_FEATURE_RESOLUTION_HOSTNAME                                  27
#define WINHTTP_FEATURE_SET_TOKEN_BINDING                                    28
#define WINHTTP_FEATURE_TOKEN_BINDING_PUBLIC_KEY                             29
#define WINHTTP_FEATURE_REFERER_TOKEN_BINDING_HOSTNAME                       30
#define WINHTTP_FEATURE_HTTP2_PLUS_TRANSFER_ENCODING                         31
#define WINHTTP_FEATURE_RESOLVER_CACHE_CONFIG                                32
#define WINHTTP_FEATURE_DISABLE_CERT_CHAIN_BUILDING                          33
#define WINHTTP_FEATURE_BACKGROUND_CONNECTIONS                               34
#define WINHTTP_FEATURE_FIRST_AVAILABLE_CONNECTION                           35


#define WINHTTP_FEATURE_TCP_PRIORITY_STATUS                                  37
#define WINHTTP_FEATURE_CONNECTION_GUID                                      38
#define WINHTTP_FEATURE_MATCH_CONNECTION_GUID                                39


#define WINHTTP_FEATURE_HTTP2_RECEIVE_WINDOW                                 43
#define WINHTTP_FEATURE_IS_FEATURE_SUPPORTED                                 44


#define WINHTTP_FEATURE_ADD_REQUEST_HEADERS_EX                               46
#define WINHTTP_FEATURE_SET_PROXY_SETINGS_PER_USER                           47
#define WINHTTP_FEATURE_READ_DATA_EX                                         48
#define WINHTTP_FEATURE_QUERY_HEADERS_EX                                     49
#define WINHTTP_FEATURE_QUERY_CONNECTION_GROUP                               50
#define WINHTTP_FEATURE_FREE_QUERY_CONNECTION_GROUP_RESULT                   51
#define WINHTTP_FEATURE_SECURITY_FLAG_IGNORE_ALL_CERT_ERRORS                 52
#define WINHTTP_FEATURE_FLAG_SECURE_DEFAULTS                                 53
#define WINHTTP_FEATURE_EXTENDED_HEADER_FLAG_UNICODE                         54
#define WINHTTP_FEATURE_QUERY_FLAG_TRAILERS                                  55
#define WINHTTP_FEATURE_QUERY_FLAG_WIRE_ENCODING                             56
#define WINHTTP_FEATURE_RESOLVER_CACHE_CONFIG_FLAG_SOFT_LIMIT                57
#define WINHTTP_FEATURE_RESOLVER_CACHE_CONFIG_FLAG_BYPASS_CACHE              58
#define WINHTTP_FEATURE_FLAG_AUTOMATIC_CHUNKING                              59
#define WINHTTP_FEATURE_QUERY_CONNECTION_GROUP_FLAG_INSECURE                 60
#define WINHTTP_FEATURE_MATCH_CONNECTION_GUID_FLAG_REQUIRE_MARKED_CONNECTION 61
#define WINHTTP_FEATURE_QUERY_EX_ALL_HEADERS                                 62
#define WINHTTP_FEATURE_READ_DATA_EX_FLAG_FILL_BUFFER                        63
#define WINHTTP_FEATURE_RESOLVER_CACHE_CONFIG_FLAG_USE_DNS_TTL               64
#define WINHTTP_FEATURE_RESOLVER_CACHE_CONFIG_FLAG_CONN_USE_TTL              65
#define WINHTTP_FEATURE_QUIC_STATS                                           66


#define WINHTTP_FEATURE_HTTP3_KEEPALIVE                                      69
#define WINHTTP_FEATURE_HTTP3_HANDSHAKE_TIMEOUT                              70
#define WINHTTP_FEATURE_HTTP3_INITIAL_RTT                                    71
#define WINHTTP_FEATURE_HTTP3_STREAM_ERROR_CODE                              72
#define WINHTTP_FEATURE_REQUEST_ANNOTATION                                   73
#define WINHTTP_FEATURE_DISABLE_PROXY_AUTH_SCHEMES                           74
#define WINHTTP_FEATURE_REVERT_IMPERSONATION_SERVER_CERT                     75
#define WINHTTP_FEATURE_DISABLE_GLOBAL_POOLING                               76
#define WINHTTP_FEATURE_GET_PROXY_SETTINGS_EX                                77
#define WINHTTP_FEATURE_SESSION_SCH_CRED                                     78
#define WINHTTP_FEATURE_QUIC_STATS_V2                                        79
#define WINHTTP_FEATURE_URL_INCLUDE_DEFAULT_PORT                             80
#define WINHTTP_FEATURE_QUIC_STREAM_STATS                                    81
#define WINHTTP_FEATURE_USE_LOOKASIDE                                        82
#define WINHTTP_FEATURE_ERROR_LOG_GUID                                       83


#define WINHTTP_FEATURE_UPGRADE_TO_PROTOCOL                                  88

#define WINHTTP_FEATURE_CONNECTION_STATS_V2                                  89
#define WINHTTP_FEATURE_FAST_FORWARD_RESPONSE                                90
#define WINHTTP_FEATURE_DISABLE_AIA_FLAG                                     91

#define WINHTTP_FEATURE_DSCP_TAG                                             92
#define WINHTTP_FEATURE_HTTP11_DOWNGRADE_TTL                                 93
#define WINHTTP_FEATURE_SESSION_ERROR_LOG_GUID                               94

#define WINHTTP_FEATURE_GET_PROXY_SETTINGS_EX_XBOX                           95



#if defined(__cplusplus)
}
#endif


/*
 * Return packing to whatever it was before we
 * entered this file
 */
#include <poppack.h>


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#endif // !defined(_WINHTTPX_)

