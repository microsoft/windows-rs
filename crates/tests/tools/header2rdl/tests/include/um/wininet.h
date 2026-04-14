/*++

Copyright (c) 1995-1999  Microsoft Corporation

Module Name:

    wininet.h

Abstract:

    Contains manifests, macros, types and prototypes for Microsoft Windows
    Internet Extensions


--*/

#include <winapifamily.h>

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)


#if !defined(_WININET_)
#include <SpecStrings.h>
#define _WININET_


/*
 * Set up Structure Packing to be 4 bytes
 * for all wininet structures
 */
#if defined(_WIN64)
#include <pshpack8.h>
#else
#include <pshpack4.h>
#endif



#if defined(__cplusplus)
extern "C" {
#endif


#if !defined(_WINX32_)
#define INTERNETAPI        EXTERN_C DECLSPEC_IMPORT HRESULT STDAPICALLTYPE
#define INTERNETAPI_(type) EXTERN_C DECLSPEC_IMPORT type STDAPICALLTYPE
#define INTERNETAPIX(type, salannotation) EXTERN_C DECLSPEC_IMPORT salannotation type STDAPICALLTYPE
#define URLCACHEAPI        EXTERN_C DECLSPEC_IMPORT DWORD STDAPICALLTYPE
#define URLCACHEAPI_(type) EXTERN_C DECLSPEC_IMPORT type STDAPICALLTYPE
#else
#define INTERNETAPI        EXTERN_C HRESULT STDAPICALLTYPE
#define INTERNETAPI_(type) EXTERN_C type STDAPICALLTYPE
#define INTERNETAPIX(type, salannotation) EXTERN_C salannotation type STDAPICALLTYPE
#define URLCACHEAPI        EXTERN_C DWORD STDAPICALLTYPE
#define URLCACHEAPI_(type) EXTERN_C type STDAPICALLTYPE
#endif

#ifndef BOOLAPI
#define BOOLAPI INTERNETAPIX(BOOL, _Success_(return != FALSE))
#endif

//
// internet types
//

typedef LPVOID HINTERNET;
typedef HINTERNET * LPHINTERNET;

typedef WORD INTERNET_PORT;
typedef INTERNET_PORT * LPINTERNET_PORT;

//
// Internet APIs
//

//
// manifests
//

#define INTERNET_INVALID_PORT_NUMBER    0           // use the protocol-specific default

#define INTERNET_DEFAULT_FTP_PORT       21          // default for FTP servers
#define INTERNET_DEFAULT_GOPHER_PORT    70          //    "     "  gopher "
#define INTERNET_DEFAULT_HTTP_PORT      80          //    "     "  HTTP   "
#define INTERNET_DEFAULT_HTTPS_PORT     443         //    "     "  HTTPS  "
#define INTERNET_DEFAULT_SOCKS_PORT     1080        // default for SOCKS firewall servers.


//
// maximum field lengths (arbitrary)
//

#define INTERNET_MAX_HOST_NAME_LENGTH   256
#define INTERNET_MAX_USER_NAME_LENGTH   128
#define INTERNET_MAX_PASSWORD_LENGTH    128
#define INTERNET_MAX_PORT_NUMBER_LENGTH 5           // INTERNET_PORT is unsigned short
#define INTERNET_MAX_PORT_NUMBER_VALUE  65535       // maximum unsigned short value
#define INTERNET_MAX_PATH_LENGTH        2048
#define INTERNET_MAX_SCHEME_LENGTH      32          // longest protocol name length
#define INTERNET_MAX_URL_LENGTH         (INTERNET_MAX_SCHEME_LENGTH \
                                        + sizeof("://") \
                                        + INTERNET_MAX_PATH_LENGTH)

//
// values returned by InternetQueryOption() with INTERNET_OPTION_KEEP_CONNECTION:
//

#define INTERNET_KEEP_ALIVE_UNKNOWN     ((DWORD)-1)
#define INTERNET_KEEP_ALIVE_ENABLED     1
#define INTERNET_KEEP_ALIVE_DISABLED    0

//
// flags returned by InternetQueryOption() with INTERNET_OPTION_REQUEST_FLAGS
//

#define INTERNET_REQFLAG_FROM_CACHE     0x00000001  // response came from cache
#define INTERNET_REQFLAG_ASYNC          0x00000002  // request was made asynchronously
#define INTERNET_REQFLAG_VIA_PROXY      0x00000004  // request was made via a proxy
#define INTERNET_REQFLAG_NO_HEADERS     0x00000008  // orginal response contained no headers
#define INTERNET_REQFLAG_PASSIVE        0x00000010  // FTP: passive-mode connection
#define INTERNET_REQFLAG_CACHE_WRITE_DISABLED 0x00000040  // HTTPS: this request not cacheable
#define INTERNET_REQFLAG_NET_TIMEOUT    0x00000080  // w/ _FROM_CACHE: net request timed out


//
// flags for IDN enable/disable via INTERNET_OPTION_IDN
//

#define INTERNET_FLAG_IDN_DIRECT        0x00000001  // IDN enabled for direct connections
#define INTERNET_FLAG_IDN_PROXY         0x00000002  // IDN enabled for proxy

//
// flags common to open functions (not InternetOpen()):
//

#define INTERNET_FLAG_RELOAD            0x80000000  // retrieve the original item

//
// flags for InternetOpenUrl():
//

#define INTERNET_FLAG_RAW_DATA          0x40000000  // FTP/gopher find: receive the item as raw (structured) data
#define INTERNET_FLAG_EXISTING_CONNECT  0x20000000  // FTP: use existing InternetConnect handle for server if possible

//
// flags for InternetOpen():
//

#define INTERNET_FLAG_ASYNC             0x10000000  // this request is asynchronous (where supported)

//
// protocol-specific flags:
//

#define INTERNET_FLAG_PASSIVE           0x08000000  // used for FTP connections

//
// additional cache flags
//

#define INTERNET_FLAG_NO_CACHE_WRITE    0x04000000  // don't write this item to the cache
#define INTERNET_FLAG_DONT_CACHE        INTERNET_FLAG_NO_CACHE_WRITE
#define INTERNET_FLAG_MAKE_PERSISTENT   0x02000000  // make this item persistent in cache
#define INTERNET_FLAG_FROM_CACHE        0x01000000  // use offline semantics
#define INTERNET_FLAG_OFFLINE           INTERNET_FLAG_FROM_CACHE

//
// additional flags
//

#define INTERNET_FLAG_SECURE            0x00800000  // use PCT/SSL if applicable (HTTP)
#define INTERNET_FLAG_KEEP_CONNECTION   0x00400000  // use keep-alive semantics
#define INTERNET_FLAG_NO_AUTO_REDIRECT  0x00200000  // don't handle redirections automatically
#define INTERNET_FLAG_READ_PREFETCH     0x00100000  // do background read prefetch
#define INTERNET_FLAG_NO_COOKIES        0x00080000  // no automatic cookie handling
#define INTERNET_FLAG_NO_AUTH           0x00040000  // no automatic authentication handling
#define INTERNET_FLAG_RESTRICTED_ZONE   0x00020000  // apply restricted zone policies for cookies, auth
#define INTERNET_FLAG_CACHE_IF_NET_FAIL 0x00010000  // return cache file if net request fails

//
// Security Ignore Flags, Allow HttpOpenRequest to overide
//  Secure Channel (SSL/PCT) failures of the following types.
//

#define INTERNET_FLAG_IGNORE_REDIRECT_TO_HTTP   0x00008000 // ex: https:// to http://
#define INTERNET_FLAG_IGNORE_REDIRECT_TO_HTTPS  0x00004000 // ex: http:// to https://
#define INTERNET_FLAG_IGNORE_CERT_DATE_INVALID  0x00002000 // expired X509 Cert.
#define INTERNET_FLAG_IGNORE_CERT_CN_INVALID    0x00001000 // bad common name in X509 Cert.

//
// more caching flags
//

#define INTERNET_FLAG_RESYNCHRONIZE     0x00000800  // asking wininet to update an item if it is newer
#define INTERNET_FLAG_HYPERLINK         0x00000400  // asking wininet to do hyperlinking semantic which works right for scripts
#define INTERNET_FLAG_NO_UI             0x00000200  // no cookie popup
#define INTERNET_FLAG_PRAGMA_NOCACHE    0x00000100  // asking wininet to add "pragma: no-cache"
#define INTERNET_FLAG_CACHE_ASYNC       0x00000080  // ok to perform lazy cache-write
#define INTERNET_FLAG_FORMS_SUBMIT      0x00000040  // this is a forms submit
#define INTERNET_FLAG_FWD_BACK          0x00000020  // fwd-back button op
#define INTERNET_FLAG_NEED_FILE         0x00000010  // need a file for this request
#define INTERNET_FLAG_MUST_CACHE_REQUEST INTERNET_FLAG_NEED_FILE

//
// flags for FTP
//

#define INTERNET_FLAG_TRANSFER_ASCII    FTP_TRANSFER_TYPE_ASCII     // 0x00000001
#define INTERNET_FLAG_TRANSFER_BINARY   FTP_TRANSFER_TYPE_BINARY    // 0x00000002

//
// flags field masks
//

#define SECURITY_INTERNET_MASK  (INTERNET_FLAG_IGNORE_CERT_CN_INVALID    |  \
                                 INTERNET_FLAG_IGNORE_CERT_DATE_INVALID  |  \
                                 INTERNET_FLAG_IGNORE_REDIRECT_TO_HTTPS  |  \
                                 INTERNET_FLAG_IGNORE_REDIRECT_TO_HTTP   )

#define SECURITY_IGNORE_ERROR_MASK  (INTERNET_FLAG_IGNORE_CERT_CN_INVALID   |  \
                                     INTERNET_FLAG_IGNORE_CERT_DATE_INVALID |  \
                                     SECURITY_FLAG_IGNORE_UNKNOWN_CA        |  \
                                     SECURITY_FLAG_IGNORE_REVOCATION        |  \
                                     SECURITY_FLAG_IGNORE_WEAK_SIGNATURE)

#define INTERNET_FLAGS_MASK     (INTERNET_FLAG_RELOAD               \
                                | INTERNET_FLAG_RAW_DATA            \
                                | INTERNET_FLAG_EXISTING_CONNECT    \
                                | INTERNET_FLAG_ASYNC               \
                                | INTERNET_FLAG_PASSIVE             \
                                | INTERNET_FLAG_NO_CACHE_WRITE      \
                                | INTERNET_FLAG_MAKE_PERSISTENT     \
                                | INTERNET_FLAG_FROM_CACHE          \
                                | INTERNET_FLAG_SECURE              \
                                | INTERNET_FLAG_KEEP_CONNECTION     \
                                | INTERNET_FLAG_NO_AUTO_REDIRECT    \
                                | INTERNET_FLAG_READ_PREFETCH       \
                                | INTERNET_FLAG_NO_COOKIES          \
                                | INTERNET_FLAG_NO_AUTH             \
                                | INTERNET_FLAG_CACHE_IF_NET_FAIL   \
                                | SECURITY_INTERNET_MASK            \
                                | INTERNET_FLAG_RESYNCHRONIZE       \
                                | INTERNET_FLAG_HYPERLINK           \
                                | INTERNET_FLAG_NO_UI               \
                                | INTERNET_FLAG_PRAGMA_NOCACHE      \
                                | INTERNET_FLAG_CACHE_ASYNC         \
                                | INTERNET_FLAG_FORMS_SUBMIT        \
                                | INTERNET_FLAG_NEED_FILE           \
                                | INTERNET_FLAG_RESTRICTED_ZONE     \
                                | INTERNET_FLAG_TRANSFER_BINARY     \
                                | INTERNET_FLAG_TRANSFER_ASCII      \
                                | INTERNET_FLAG_FWD_BACK            \
                                | INTERNET_FLAG_BGUPDATE            \
                                )



#define INTERNET_ERROR_MASK_INSERT_CDROM                    0x1
#define INTERNET_ERROR_MASK_COMBINED_SEC_CERT               0x2
#define INTERNET_ERROR_MASK_NEED_MSN_SSPI_PKG               0X4
#define INTERNET_ERROR_MASK_LOGIN_FAILURE_DISPLAY_ENTITY_BODY 0x8

#define INTERNET_OPTIONS_MASK   (~INTERNET_FLAGS_MASK)

//
// common per-API flags (new APIs)
//

#define WININET_API_FLAG_ASYNC          0x00000001  // force async operation
#define WININET_API_FLAG_SYNC           0x00000004  // force sync operation
#define WININET_API_FLAG_USE_CONTEXT    0x00000008  // use value supplied in dwContext (even if 0)

//
// INTERNET_NO_CALLBACK - if this value is presented as the dwContext parameter
// then no call-backs will be made for that API
//

#define INTERNET_NO_CALLBACK            0

//
// structures/types
//

//
// INTERNET_SCHEME - enumerated URL scheme type
//

#ifndef _INTERNET_SCHEME_
#define _INTERNET_SCHEME_

typedef enum {
    INTERNET_SCHEME_PARTIAL = -2,
    INTERNET_SCHEME_UNKNOWN = -1,
    INTERNET_SCHEME_DEFAULT = 0,
    INTERNET_SCHEME_FTP,
    INTERNET_SCHEME_GOPHER,
    INTERNET_SCHEME_HTTP,
    INTERNET_SCHEME_HTTPS,
    INTERNET_SCHEME_FILE,
    INTERNET_SCHEME_NEWS,
    INTERNET_SCHEME_MAILTO,
    INTERNET_SCHEME_SOCKS,
    INTERNET_SCHEME_JAVASCRIPT,
    INTERNET_SCHEME_VBSCRIPT,
    INTERNET_SCHEME_RES,
    INTERNET_SCHEME_FIRST = INTERNET_SCHEME_FTP,
    INTERNET_SCHEME_LAST = INTERNET_SCHEME_RES
} INTERNET_SCHEME, * LPINTERNET_SCHEME;

#endif

//
// INTERNET_ASYNC_RESULT - this structure is returned to the application via
// the callback with INTERNET_STATUS_REQUEST_COMPLETE. It is not sufficient to
// just return the result of the async operation. If the API failed then the
// app cannot call GetLastError() because the thread context will be incorrect.
// Both the value returned by the async API and any resultant error code are
// made available. The app need not check dwError if dwResult indicates that
// the API succeeded (in this case dwError will be ERROR_SUCCESS)
//

typedef struct {

    //
    // dwResult - the HINTERNET, DWORD or BOOL return code from an async API
    //

    DWORD_PTR dwResult;

    //
    // dwError - the error code if the API failed
    //

    DWORD dwError;
} INTERNET_ASYNC_RESULT, * LPINTERNET_ASYNC_RESULT;


//
// INTERNET_DIAGNOSTIC_SOCKET_INFO - info about the socket in use
//

typedef struct {
    DWORD_PTR Socket;
    DWORD     SourcePort;
    DWORD     DestPort;
    DWORD     Flags;
} INTERNET_DIAGNOSTIC_SOCKET_INFO, * LPINTERNET_DIAGNOSTIC_SOCKET_INFO;

//
// INTERNET_DIAGNOSTIC_SOCKET_INFO.Flags definitions
//

#define IDSI_FLAG_KEEP_ALIVE    0x00000001  // set if from keep-alive pool
#define IDSI_FLAG_SECURE        0x00000002  // set if secure connection
#define IDSI_FLAG_PROXY         0x00000004  // set if using proxy
#define IDSI_FLAG_TUNNEL        0x00000008  // set if tunnelling through proxy

//
// INTERNET_PROXY_INFO - structure supplied with INTERNET_OPTION_PROXY to get/
// set proxy information on a InternetOpen() handle
//

typedef struct {

    //
    // dwAccessType - INTERNET_OPEN_TYPE_DIRECT, INTERNET_OPEN_TYPE_PROXY, or
    // INTERNET_OPEN_TYPE_PRECONFIG (set only)
    //

    DWORD dwAccessType;

    //
    // lpszProxy - proxy server list
    //

    LPCTSTR lpszProxy;

    //
    // lpszProxyBypass - proxy bypass list
    //

    LPCTSTR lpszProxyBypass;
} INTERNET_PROXY_INFO, * LPINTERNET_PROXY_INFO;

//
// INTERNET_PER_CONN_OPTION_LIST - set per-connection options such as proxy
// and autoconfig info
//
// Set and queried using Internet[Set|Query]Option with
// INTERNET_OPTION_PER_CONNECTION_OPTION
//

typedef struct {
    DWORD   dwOption;            // option to be queried or set
    union {
        DWORD    dwValue;        // dword value for the option
        LPSTR    pszValue;       // pointer to string value for the option
        FILETIME ftValue;        // file-time value for the option
    } Value;
} INTERNET_PER_CONN_OPTIONA, * LPINTERNET_PER_CONN_OPTIONA;
typedef struct {
    DWORD   dwOption;            // option to be queried or set
    union {
        DWORD    dwValue;        // dword value for the option
        LPWSTR   pszValue;       // pointer to string value for the option
        FILETIME ftValue;        // file-time value for the option
    } Value;
} INTERNET_PER_CONN_OPTIONW, * LPINTERNET_PER_CONN_OPTIONW;
#ifdef UNICODE
typedef INTERNET_PER_CONN_OPTIONW INTERNET_PER_CONN_OPTION;
typedef LPINTERNET_PER_CONN_OPTIONW LPINTERNET_PER_CONN_OPTION;
#else
typedef INTERNET_PER_CONN_OPTIONA INTERNET_PER_CONN_OPTION;
typedef LPINTERNET_PER_CONN_OPTIONA LPINTERNET_PER_CONN_OPTION;
#endif // UNICODE

typedef struct {
    DWORD   dwSize;             // size of the INTERNET_PER_CONN_OPTION_LIST struct
    LPSTR   pszConnection;      // connection name to set/query options
    DWORD   dwOptionCount;      // number of options to set/query
    DWORD   dwOptionError;      // on error, which option failed
    LPINTERNET_PER_CONN_OPTIONA  pOptions;
                                // array of options to set/query
} INTERNET_PER_CONN_OPTION_LISTA, * LPINTERNET_PER_CONN_OPTION_LISTA;
typedef struct {
    DWORD   dwSize;             // size of the INTERNET_PER_CONN_OPTION_LIST struct
    LPWSTR  pszConnection;      // connection name to set/query options
    DWORD   dwOptionCount;      // number of options to set/query
    DWORD   dwOptionError;      // on error, which option failed
    LPINTERNET_PER_CONN_OPTIONW  pOptions;
                                // array of options to set/query
} INTERNET_PER_CONN_OPTION_LISTW, * LPINTERNET_PER_CONN_OPTION_LISTW;
#ifdef UNICODE
typedef INTERNET_PER_CONN_OPTION_LISTW INTERNET_PER_CONN_OPTION_LIST;
typedef LPINTERNET_PER_CONN_OPTION_LISTW LPINTERNET_PER_CONN_OPTION_LIST;
#else
typedef INTERNET_PER_CONN_OPTION_LISTA INTERNET_PER_CONN_OPTION_LIST;
typedef LPINTERNET_PER_CONN_OPTION_LISTA LPINTERNET_PER_CONN_OPTION_LIST;
#endif // UNICODE

//
// Options used in INTERNET_PER_CONN_OPTION struct
//
#define INTERNET_PER_CONN_FLAGS                         1
#define INTERNET_PER_CONN_PROXY_SERVER                  2
#define INTERNET_PER_CONN_PROXY_BYPASS                  3
#define INTERNET_PER_CONN_AUTOCONFIG_URL                4
#define INTERNET_PER_CONN_AUTODISCOVERY_FLAGS           5
#define INTERNET_PER_CONN_AUTOCONFIG_SECONDARY_URL      6
#define INTERNET_PER_CONN_AUTOCONFIG_RELOAD_DELAY_MINS  7
#define INTERNET_PER_CONN_AUTOCONFIG_LAST_DETECT_TIME   8
#define INTERNET_PER_CONN_AUTOCONFIG_LAST_DETECT_URL    9
#define INTERNET_PER_CONN_FLAGS_UI                      10

//
// PER_CONN_FLAGS
//
#define PROXY_TYPE_DIRECT                               0x00000001   // direct to net
#define PROXY_TYPE_PROXY                                0x00000002   // via named proxy
#define PROXY_TYPE_AUTO_PROXY_URL                       0x00000004   // autoproxy URL
#define PROXY_TYPE_AUTO_DETECT                          0x00000008   // use autoproxy detection

//
// PER_CONN_AUTODISCOVERY_FLAGS
//
#define AUTO_PROXY_FLAG_USER_SET                        0x00000001   // user changed this setting
#define AUTO_PROXY_FLAG_ALWAYS_DETECT                   0x00000002   // force detection even when its not needed
#define AUTO_PROXY_FLAG_DETECTION_RUN                   0x00000004   // detection has been run
#define AUTO_PROXY_FLAG_MIGRATED                        0x00000008   // migration has just been done
#define AUTO_PROXY_FLAG_DONT_CACHE_PROXY_RESULT         0x00000010   // don't cache result of host=proxy name
#define AUTO_PROXY_FLAG_CACHE_INIT_RUN                  0x00000020   // don't initalize and run unless URL expired
#define AUTO_PROXY_FLAG_DETECTION_SUSPECT               0x00000040   // results of detection are suspect

//
// INTERNET_VERSION_INFO - version information returned via
// InternetQueryOption(..., INTERNET_OPTION_VERSION, ...)
//

typedef struct {
    DWORD dwMajorVersion;
    DWORD dwMinorVersion;
} INTERNET_VERSION_INFO, * LPINTERNET_VERSION_INFO;

//
// HTTP_VERSION_INFO - query or set global HTTP version (1.0 or 1.1)
//

#ifndef _HTTP_VERSION_INFO_
#define _HTTP_VERSION_INFO_

typedef struct {
    DWORD dwMajorVersion;
    DWORD dwMinorVersion;
} HTTP_VERSION_INFO, * LPHTTP_VERSION_INFO, * PHTTP_VERSION_INFO;

#endif

//
// INTERNET_CONNECTED_INFO - information used to set the global connected state
//

typedef struct {

    //
    // dwConnectedState - new connected/disconnected state.
    // See INTERNET_STATE_CONNECTED, etc.
    //

    DWORD dwConnectedState;

    //
    // dwFlags - flags controlling connected->disconnected (or disconnected->
    // connected) transition. See below
    //

    DWORD dwFlags;
} INTERNET_CONNECTED_INFO, * LPINTERNET_CONNECTED_INFO;


//
// flags for INTERNET_CONNECTED_INFO dwFlags
//

//
// ISO_FORCE_DISCONNECTED - if set when putting Wininet into disconnected mode,
// all outstanding requests will be aborted with a cancelled error
//

#define ISO_FORCE_DISCONNECTED  0x00000001


//
// URL_COMPONENTS - the constituent parts of an URL. Used in InternetCrackUrl()
// and InternetCreateUrl()
//
// For InternetCrackUrl(), if a pointer field and its corresponding length field
// are both 0 then that component is not returned. If the pointer field is NULL
// but the length field is not zero, then both the pointer and length fields are
// returned if both pointer and corresponding length fields are non-zero then
// the pointer field points to a buffer where the component is copied. The
// component may be un-escaped, depending on dwFlags
//
// For InternetCreateUrl(), the pointer fields should be NULL if the component
// is not required. If the corresponding length field is zero then the pointer
// field is the address of a zero-terminated string. If the length field is not
// zero then it is the string length of the corresponding pointer field
//

#if _MSC_VER >= 1200
#pragma warning(push)
#endif
#pragma warning( disable : 4121 )   // disable alignment warning

#ifndef _URL_COMPONENTS_
#define _URL_COMPONENTS_

typedef struct {
    DWORD   dwStructSize;       // size of this structure. Used in version check
    LPSTR   lpszScheme;         // pointer to scheme name
    DWORD   dwSchemeLength;     // length of scheme name
    INTERNET_SCHEME nScheme;    // enumerated scheme type (if known)
    LPSTR   lpszHostName;       // pointer to host name
    DWORD   dwHostNameLength;   // length of host name
    INTERNET_PORT nPort;        // converted port number
    LPSTR   lpszUserName;       // pointer to user name
    DWORD   dwUserNameLength;   // length of user name
    LPSTR   lpszPassword;       // pointer to password
    DWORD   dwPasswordLength;   // length of password
    LPSTR   lpszUrlPath;        // pointer to URL-path
    DWORD   dwUrlPathLength;    // length of URL-path
    LPSTR   lpszExtraInfo;      // pointer to extra information (e.g. ?foo or #foo)
    DWORD   dwExtraInfoLength;  // length of extra information
} URL_COMPONENTSA, * LPURL_COMPONENTSA;
typedef struct {
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
} URL_COMPONENTSW, * LPURL_COMPONENTSW;
#ifdef UNICODE
typedef URL_COMPONENTSW URL_COMPONENTS;
typedef LPURL_COMPONENTSW LPURL_COMPONENTS;
#else
typedef URL_COMPONENTSA URL_COMPONENTS;
typedef LPURL_COMPONENTSA LPURL_COMPONENTS;
#endif // UNICODE

#else

typedef struct {
    DWORD   dwStructSize;       // size of this structure. Used in version check
    LPSTR   lpszScheme;         // pointer to scheme name
    DWORD   dwSchemeLength;     // length of scheme name
    INTERNET_SCHEME nScheme;    // enumerated scheme type (if known)
    LPSTR   lpszHostName;       // pointer to host name
    DWORD   dwHostNameLength;   // length of host name
    INTERNET_PORT nPort;        // converted port number
    LPSTR   lpszUserName;       // pointer to user name
    DWORD   dwUserNameLength;   // length of user name
    LPSTR   lpszPassword;       // pointer to password
    DWORD   dwPasswordLength;   // length of password
    LPSTR   lpszUrlPath;        // pointer to URL-path
    DWORD   dwUrlPathLength;    // length of URL-path
    LPSTR   lpszExtraInfo;      // pointer to extra information (e.g. ?foo or #foo)
    DWORD   dwExtraInfoLength;  // length of extra information
} URL_COMPONENTSA, * LPURL_COMPONENTSA;

#endif

#if _MSC_VER >= 1200
#pragma warning(pop)
#else
#pragma warning( default : 4121 )   // restore alignment warning
#endif

//
// INTERNET_CERTIFICATE_INFO lpBuffer - contains the certificate returned from
// the server
//

typedef struct {

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

    LPTSTR lpszSubjectInfo;

    //
    // lpszIssuerInfo - the name of orgainzation, site, and server
    //   the cert was issues by.
    //

    LPTSTR lpszIssuerInfo;

    //
    // lpszProtocolName - the name of the protocol used to provide the secure
    //   connection.
    //

    LPTSTR lpszProtocolName;

    //
    // lpszSignatureAlgName - the name of the algorithm used for signing
    //  the certificate.
    //

    LPTSTR lpszSignatureAlgName;

    //
    // lpszEncryptionAlgName - the name of the algorithm used for
    //  doing encryption over the secure channel (SSL/PCT) connection.
    //

    LPTSTR lpszEncryptionAlgName;

    //
    // dwKeySize - size of the key.
    //

    DWORD dwKeySize;

} INTERNET_CERTIFICATE_INFO, * LPINTERNET_CERTIFICATE_INFO;


//
// INTERNET_BUFFERS - combines headers and data. May be chained for e.g. file
// upload or scatter/gather operations. For chunked read/write, lpcszHeader
// contains the chunked-ext
//

typedef struct _INTERNET_BUFFERSA {
    DWORD dwStructSize;                 // used for API versioning. Set to sizeof(INTERNET_BUFFERS)
    struct _INTERNET_BUFFERSA * Next;   // chain of buffers
    LPCSTR   lpcszHeader;               // pointer to headers (may be NULL)
    DWORD dwHeadersLength;              // length of headers if not NULL
    DWORD dwHeadersTotal;               // size of headers if not enough buffer
    LPVOID lpvBuffer;                   // pointer to data buffer (may be NULL)
    DWORD dwBufferLength;               // length of data buffer if not NULL
    DWORD dwBufferTotal;                // total size of chunk, or content-length if not chunked
    DWORD dwOffsetLow;                  // used for read-ranges (only used in HttpSendRequest2)
    DWORD dwOffsetHigh;
} INTERNET_BUFFERSA, * LPINTERNET_BUFFERSA;
typedef struct _INTERNET_BUFFERSW {
    DWORD dwStructSize;                 // used for API versioning. Set to sizeof(INTERNET_BUFFERS)
    struct _INTERNET_BUFFERSW * Next;   // chain of buffers
    LPCWSTR  lpcszHeader;               // pointer to headers (may be NULL)
    DWORD dwHeadersLength;              // length of headers if not NULL
    DWORD dwHeadersTotal;               // size of headers if not enough buffer
    LPVOID lpvBuffer;                   // pointer to data buffer (may be NULL)
    DWORD dwBufferLength;               // length of data buffer if not NULL
    DWORD dwBufferTotal;                // total size of chunk, or content-length if not chunked
    DWORD dwOffsetLow;                  // used for read-ranges (only used in HttpSendRequest2)
    DWORD dwOffsetHigh;
} INTERNET_BUFFERSW, * LPINTERNET_BUFFERSW;
#ifdef UNICODE
typedef INTERNET_BUFFERSW INTERNET_BUFFERS;
typedef LPINTERNET_BUFFERSW LPINTERNET_BUFFERS;
#else
typedef INTERNET_BUFFERSA INTERNET_BUFFERS;
typedef LPINTERNET_BUFFERSA LPINTERNET_BUFFERS;
#endif // UNICODE

//
// prototypes
//

BOOLAPI InternetTimeFromSystemTimeA(
    _In_ CONST SYSTEMTIME *pst,           // input GMT time
    _In_ DWORD dwRFC,                     // RFC format
    _Out_writes_bytes_(cbTime) LPSTR lpszTime,  // output string buffer
    _In_ DWORD cbTime                     // output buffer size
    );

BOOLAPI InternetTimeFromSystemTimeW(
    _In_ CONST SYSTEMTIME *pst,            // input GMT time
    _In_ DWORD dwRFC,                      // RFC format
    _Out_writes_bytes_(cbTime) LPWSTR lpszTime,  // output string buffer
    _In_ DWORD cbTime                      // output buffer size
    );

#ifdef UNICODE
#define InternetTimeFromSystemTime  InternetTimeFromSystemTimeW
#else
#ifdef _WINX32_
#define InternetTimeFromSystemTime  InternetTimeFromSystemTimeA
#else
BOOLAPI InternetTimeFromSystemTime(
    _In_ CONST SYSTEMTIME *pst,           // input GMT time
    _In_ DWORD dwRFC,                     // RFC format
    _Out_writes_bytes_(cbTime) LPSTR lpszTime,  // output string buffer
    _In_ DWORD cbTime                     // output buffer size
    );
#endif // _WINX32_
#endif // !UNICODE

//
// constants for InternetTimeFromSystemTime
//

#define INTERNET_RFC1123_FORMAT     0
#define INTERNET_RFC1123_BUFSIZE   30

BOOLAPI InternetTimeToSystemTimeA(
    _In_ LPCSTR lpszTime,          // NULL terminated string
    _Out_ SYSTEMTIME *pst,         // output in GMT time
    _Reserved_ DWORD dwReserved
    );

BOOLAPI InternetTimeToSystemTimeW(
    _In_ LPCWSTR lpszTime,         // NULL terminated string
    _Out_ SYSTEMTIME *pst,         // output in GMT time
    _Reserved_ DWORD dwReserved
    );

#ifdef UNICODE
#define InternetTimeToSystemTime  InternetTimeToSystemTimeW
#else
#ifdef _WINX32_
#define InternetTimeToSystemTime  InternetTimeToSystemTimeA
#else
BOOLAPI InternetTimeToSystemTime(
    _In_ LPCSTR lpszTime,          // NULL terminated string
    _Out_ SYSTEMTIME *pst,         // output in GMT time
    _Reserved_ DWORD dwReserved
    );
#endif // _WINX32_
#endif // !UNICODE


BOOLAPI
InternetCrackUrlA(
    _In_reads_(dwUrlLength) LPCSTR lpszUrl,
    _In_ DWORD dwUrlLength,
    _In_ DWORD dwFlags,
    _Inout_ LPURL_COMPONENTSA lpUrlComponents
    );
BOOLAPI
InternetCrackUrlW(
    _In_reads_(dwUrlLength) LPCWSTR lpszUrl,
    _In_ DWORD dwUrlLength,
    _In_ DWORD dwFlags,
    _Inout_ LPURL_COMPONENTSW lpUrlComponents
    );
#ifdef UNICODE
#define InternetCrackUrl  InternetCrackUrlW
#else
#define InternetCrackUrl  InternetCrackUrlA
#endif // !UNICODE

BOOLAPI
InternetCreateUrlA(
    _In_ LPURL_COMPONENTSA lpUrlComponents,
    _In_ DWORD dwFlags,
    _Out_writes_opt_(*lpdwUrlLength) LPSTR lpszUrl,
    _Inout_ LPDWORD lpdwUrlLength
    );
BOOLAPI
InternetCreateUrlW(
    _In_ LPURL_COMPONENTSW lpUrlComponents,
    _In_ DWORD dwFlags,
    _Out_writes_opt_(*lpdwUrlLength) LPWSTR lpszUrl,
    _Inout_ LPDWORD lpdwUrlLength
    );
#ifdef UNICODE
#define InternetCreateUrl  InternetCreateUrlW
#else
#define InternetCreateUrl  InternetCreateUrlA
#endif // !UNICODE

BOOLAPI
InternetCanonicalizeUrlA(
    _In_ LPCSTR lpszUrl,
    _Out_writes_(*lpdwBufferLength) LPSTR lpszBuffer,
    _Inout_ LPDWORD lpdwBufferLength,
    _In_ DWORD dwFlags
    );
BOOLAPI
InternetCanonicalizeUrlW(
    _In_ LPCWSTR lpszUrl,
    _Out_writes_(*lpdwBufferLength) LPWSTR lpszBuffer,
    _Inout_ LPDWORD lpdwBufferLength,
    _In_ DWORD dwFlags
    );
#ifdef UNICODE
#define InternetCanonicalizeUrl  InternetCanonicalizeUrlW
#else
#define InternetCanonicalizeUrl  InternetCanonicalizeUrlA
#endif // !UNICODE

BOOLAPI
InternetCombineUrlA(
    _In_ LPCSTR lpszBaseUrl,
    _In_ LPCSTR lpszRelativeUrl,
    _Out_writes_(*lpdwBufferLength) LPSTR lpszBuffer,
    _Inout_ LPDWORD lpdwBufferLength,
    _In_ DWORD dwFlags
    );
BOOLAPI
InternetCombineUrlW(
    _In_ LPCWSTR lpszBaseUrl,
    _In_ LPCWSTR lpszRelativeUrl,
    _Out_writes_(*lpdwBufferLength) LPWSTR lpszBuffer,
    _Inout_ LPDWORD lpdwBufferLength,
    _In_ DWORD dwFlags
    );
#ifdef UNICODE
#define InternetCombineUrl  InternetCombineUrlW
#else
#define InternetCombineUrl  InternetCombineUrlA
#endif // !UNICODE

//
// flags for InternetCrackUrl() and InternetCreateUrl()
//

#define ICU_ESCAPE      0x80000000  // (un)escape URL characters
#define ICU_USERNAME    0x40000000  // use internal username & password


//
// flags for InternetCanonicalizeUrl() and InternetCombineUrl()
//

#define ICU_NO_ENCODE   0x20000000  // Don't convert unsafe characters to escape sequence
#define ICU_DECODE      0x10000000  // Convert %XX escape sequences to characters
#define ICU_NO_META     0x08000000  // Don't convert .. etc. meta path sequences
#define ICU_ENCODE_SPACES_ONLY 0x04000000  // Encode spaces only
#define ICU_BROWSER_MODE 0x02000000 // Special encode/decode rules for browser
#define ICU_ENCODE_PERCENT      0x00001000      // Encode any percent (ASCII25)
        // signs encountered, default is to not encode percent.

INTERNETAPI_(HINTERNET) InternetOpenA(
    _In_opt_ LPCSTR lpszAgent,
    _In_ DWORD dwAccessType,
    _In_opt_ LPCSTR lpszProxy,
    _In_opt_ LPCSTR lpszProxyBypass,
    _In_ DWORD dwFlags
    );
INTERNETAPI_(HINTERNET) InternetOpenW(
    _In_opt_ LPCWSTR lpszAgent,
    _In_ DWORD dwAccessType,
    _In_opt_ LPCWSTR lpszProxy,
    _In_opt_ LPCWSTR lpszProxyBypass,
    _In_ DWORD dwFlags
    );
#ifdef UNICODE
#define InternetOpen  InternetOpenW
#else
#define InternetOpen  InternetOpenA
#endif // !UNICODE

//
// access types for InternetOpen()
//

#define INTERNET_OPEN_TYPE_PRECONFIG                    0   // use registry configuration
#define INTERNET_OPEN_TYPE_DIRECT                       1   // direct to net
#define INTERNET_OPEN_TYPE_PROXY                        3   // via named proxy
#define INTERNET_OPEN_TYPE_PRECONFIG_WITH_NO_AUTOPROXY  4   // prevent using java/script/INS

//
// old names for access types
//

#define PRE_CONFIG_INTERNET_ACCESS  INTERNET_OPEN_TYPE_PRECONFIG
#define LOCAL_INTERNET_ACCESS       INTERNET_OPEN_TYPE_DIRECT
#define CERN_PROXY_INTERNET_ACCESS  INTERNET_OPEN_TYPE_PROXY

BOOLAPI InternetCloseHandle(
    _In_ HINTERNET hInternet
    );

INTERNETAPI_(HINTERNET)
InternetConnectA(
    _In_ HINTERNET hInternet,
    _In_ LPCSTR lpszServerName,
    _In_ INTERNET_PORT nServerPort,
    _In_opt_ LPCSTR lpszUserName,
    _In_opt_ LPCSTR lpszPassword,
    _In_ DWORD dwService,
    _In_ DWORD dwFlags,
    _In_opt_ DWORD_PTR dwContext
    );
INTERNETAPI_(HINTERNET)
InternetConnectW(
    _In_ HINTERNET hInternet,
    _In_ LPCWSTR lpszServerName,
    _In_ INTERNET_PORT nServerPort,
    _In_opt_ LPCWSTR lpszUserName,
    _In_opt_ LPCWSTR lpszPassword,
    _In_ DWORD dwService,
    _In_ DWORD dwFlags,
    _In_opt_ DWORD_PTR dwContext
    );
#ifdef UNICODE
#define InternetConnect  InternetConnectW
#else
#define InternetConnect  InternetConnectA
#endif // !UNICODE

//
// service types for InternetConnect()
//

#define INTERNET_SERVICE_FTP    1
#define INTERNET_SERVICE_GOPHER 2
#define INTERNET_SERVICE_HTTP   3


INTERNETAPI_(HINTERNET) InternetOpenUrlA(
    _In_ HINTERNET hInternet,
    _In_ LPCSTR lpszUrl,
    _In_reads_opt_(dwHeadersLength) LPCSTR lpszHeaders,
    _In_ DWORD dwHeadersLength,
    _In_ DWORD dwFlags,
    _In_opt_ DWORD_PTR dwContext
    );
INTERNETAPI_(HINTERNET) InternetOpenUrlW(
    _In_ HINTERNET hInternet,
    _In_ LPCWSTR lpszUrl,
    _In_reads_opt_(dwHeadersLength) LPCWSTR lpszHeaders,
    _In_ DWORD dwHeadersLength,
    _In_ DWORD dwFlags,
    _In_opt_ DWORD_PTR dwContext
    );
#ifdef UNICODE
#define InternetOpenUrl  InternetOpenUrlW
#else
#define InternetOpenUrl  InternetOpenUrlA
#endif // !UNICODE

BOOLAPI InternetReadFile(
    _In_ HINTERNET hFile,
    _Out_writes_bytes_(dwNumberOfBytesToRead) __out_data_source(NETWORK) LPVOID lpBuffer,
    _In_ DWORD dwNumberOfBytesToRead,
    _Out_ LPDWORD lpdwNumberOfBytesRead
    );

BOOLAPI InternetReadFileExA(
    _In_ HINTERNET hFile,
    _Out_ __out_data_source(NETWORK) LPINTERNET_BUFFERSA lpBuffersOut,
    _In_ DWORD dwFlags,
    _In_opt_ DWORD_PTR dwContext
    );
BOOLAPI InternetReadFileExW(
    _In_ HINTERNET hFile,
    _Out_ __out_data_source(NETWORK) LPINTERNET_BUFFERSW lpBuffersOut,
    _In_ DWORD dwFlags,
    _In_opt_ DWORD_PTR dwContext
    );
#ifdef UNICODE
#define InternetReadFileEx  InternetReadFileExW
#else
#define InternetReadFileEx  InternetReadFileExA
#endif // !UNICODE

//
// flags for InternetReadFileEx()
//

#define IRF_ASYNC       WININET_API_FLAG_ASYNC
#define IRF_SYNC        WININET_API_FLAG_SYNC
#define IRF_USE_CONTEXT WININET_API_FLAG_USE_CONTEXT
#define IRF_NO_WAIT     0x00000008

INTERNETAPI_(DWORD) InternetSetFilePointer(
    _In_ HINTERNET hFile,
    _In_ LONG  lDistanceToMove,
    _Inout_opt_ PLONG lpDistanceToMoveHigh,
    _In_ DWORD dwMoveMethod,
    _Reserved_ DWORD_PTR dwContext
    );

BOOLAPI InternetWriteFile(
    _In_ HINTERNET hFile,
    _In_reads_bytes_(dwNumberOfBytesToWrite) LPCVOID lpBuffer,
    _In_ DWORD dwNumberOfBytesToWrite,
    _Out_ LPDWORD lpdwNumberOfBytesWritten
    );


BOOLAPI InternetQueryDataAvailable(
    _In_ HINTERNET hFile,
    _Out_opt_ __out_data_source(NETWORK) LPDWORD lpdwNumberOfBytesAvailable,
    _In_ DWORD dwFlags,
    _In_opt_ DWORD_PTR dwContext
    );

BOOLAPI InternetFindNextFileA(
    _In_ HINTERNET hFind,
    _Out_ LPVOID lpvFindData
    );
BOOLAPI InternetFindNextFileW(
    _In_ HINTERNET hFind,
    _Out_ LPVOID lpvFindData
    );
#ifdef UNICODE
#define InternetFindNextFile  InternetFindNextFileW
#else
#define InternetFindNextFile  InternetFindNextFileA
#endif // !UNICODE

BOOLAPI InternetQueryOptionA(
    _In_opt_ HINTERNET hInternet,
    _In_ DWORD dwOption,
    _Out_writes_bytes_to_opt_(*lpdwBufferLength, *lpdwBufferLength) __out_data_source(NETWORK) LPVOID lpBuffer,
    _Inout_ LPDWORD lpdwBufferLength
    );
BOOLAPI InternetQueryOptionW(
    _In_opt_ HINTERNET hInternet,
    _In_ DWORD dwOption,
    _Out_writes_bytes_to_opt_(*lpdwBufferLength, *lpdwBufferLength) __out_data_source(NETWORK) LPVOID lpBuffer,
    _Inout_ LPDWORD lpdwBufferLength
    );
#ifdef UNICODE
#define InternetQueryOption  InternetQueryOptionW
#else
#define InternetQueryOption  InternetQueryOptionA
#endif // !UNICODE

BOOLAPI InternetSetOptionA(
    _In_opt_ HINTERNET hInternet,
    _In_       DWORD dwOption,
    _In_opt_ LPVOID lpBuffer,
    _In_       DWORD dwBufferLength
    );
BOOLAPI InternetSetOptionW(
    _In_opt_ HINTERNET hInternet,
    _In_       DWORD dwOption,
    _In_opt_ LPVOID lpBuffer,
    _In_       DWORD dwBufferLength
    );
#ifdef UNICODE
#define InternetSetOption  InternetSetOptionW
#else
#define InternetSetOption  InternetSetOptionA
#endif // !UNICODE

BOOLAPI InternetSetOptionExA(
    _In_opt_ HINTERNET hInternet,
    _In_       DWORD dwOption,
    _In_opt_ LPVOID lpBuffer,
    _In_       DWORD dwBufferLength,
    _In_       DWORD dwFlags
    );
BOOLAPI InternetSetOptionExW(
    _In_opt_ HINTERNET hInternet,
    _In_       DWORD dwOption,
    _In_opt_ LPVOID lpBuffer,
    _In_       DWORD dwBufferLength,
    _In_       DWORD dwFlags
    );
#ifdef UNICODE
#define InternetSetOptionEx  InternetSetOptionExW
#else
#define InternetSetOptionEx  InternetSetOptionExA
#endif // !UNICODE

BOOLAPI InternetLockRequestFile(
    _In_  HINTERNET hInternet,
    _Out_ HANDLE * lphLockRequestInfo
    );

BOOLAPI InternetUnlockRequestFile(
    _Inout_ HANDLE hLockRequestInfo
    );

//
// flags for InternetSetOptionEx()
//

#define ISO_GLOBAL      0x00000001  // modify option globally
#define ISO_REGISTRY    0x00000002  // write option to registry (where applicable)

#define ISO_VALID_FLAGS (ISO_GLOBAL | ISO_REGISTRY)

//
// options manifests for Internet{Query|Set}Option
//

#define INTERNET_OPTION_CALLBACK                1
#define INTERNET_OPTION_CONNECT_TIMEOUT         2
#define INTERNET_OPTION_CONNECT_RETRIES         3
#define INTERNET_OPTION_CONNECT_BACKOFF         4
#define INTERNET_OPTION_SEND_TIMEOUT            5
#define INTERNET_OPTION_CONTROL_SEND_TIMEOUT    INTERNET_OPTION_SEND_TIMEOUT
#define INTERNET_OPTION_RECEIVE_TIMEOUT         6
#define INTERNET_OPTION_CONTROL_RECEIVE_TIMEOUT INTERNET_OPTION_RECEIVE_TIMEOUT
#define INTERNET_OPTION_DATA_SEND_TIMEOUT       7
#define INTERNET_OPTION_DATA_RECEIVE_TIMEOUT    8
#define INTERNET_OPTION_HANDLE_TYPE             9
#define INTERNET_OPTION_LISTEN_TIMEOUT          11
#define INTERNET_OPTION_READ_BUFFER_SIZE        12
#define INTERNET_OPTION_WRITE_BUFFER_SIZE       13

#define INTERNET_OPTION_ASYNC_ID                15
#define INTERNET_OPTION_ASYNC_PRIORITY          16

#define INTERNET_OPTION_PARENT_HANDLE           21
#define INTERNET_OPTION_KEEP_CONNECTION         22
#define INTERNET_OPTION_REQUEST_FLAGS           23
#define INTERNET_OPTION_EXTENDED_ERROR          24

#define INTERNET_OPTION_OFFLINE_MODE            26
#define INTERNET_OPTION_CACHE_STREAM_HANDLE     27
#define INTERNET_OPTION_USERNAME                28
#define INTERNET_OPTION_PASSWORD                29
#define INTERNET_OPTION_ASYNC                   30
#define INTERNET_OPTION_SECURITY_FLAGS          31
#define INTERNET_OPTION_SECURITY_CERTIFICATE_STRUCT 32
#define INTERNET_OPTION_DATAFILE_NAME           33
#define INTERNET_OPTION_URL                     34
#define INTERNET_OPTION_SECURITY_CERTIFICATE    35
#define INTERNET_OPTION_SECURITY_KEY_BITNESS    36
#define INTERNET_OPTION_REFRESH                 37
#define INTERNET_OPTION_PROXY                   38
#define INTERNET_OPTION_SETTINGS_CHANGED        39
#define INTERNET_OPTION_VERSION                 40
#define INTERNET_OPTION_USER_AGENT              41
#define INTERNET_OPTION_END_BROWSER_SESSION     42
#define INTERNET_OPTION_PROXY_USERNAME          43
#define INTERNET_OPTION_PROXY_PASSWORD          44
#define INTERNET_OPTION_CONTEXT_VALUE           45
#define INTERNET_OPTION_CONNECT_LIMIT           46
#define INTERNET_OPTION_SECURITY_SELECT_CLIENT_CERT 47
#define INTERNET_OPTION_POLICY                  48
#define INTERNET_OPTION_DISCONNECTED_TIMEOUT    49
#define INTERNET_OPTION_CONNECTED_STATE         50
#define INTERNET_OPTION_IDLE_STATE              51
#define INTERNET_OPTION_OFFLINE_SEMANTICS       52
#define INTERNET_OPTION_SECONDARY_CACHE_KEY     53
#define INTERNET_OPTION_CALLBACK_FILTER         54
#define INTERNET_OPTION_CONNECT_TIME            55
#define INTERNET_OPTION_SEND_THROUGHPUT         56
#define INTERNET_OPTION_RECEIVE_THROUGHPUT      57
#define INTERNET_OPTION_REQUEST_PRIORITY        58
#define INTERNET_OPTION_HTTP_VERSION            59
#define INTERNET_OPTION_RESET_URLCACHE_SESSION  60
#define INTERNET_OPTION_ERROR_MASK              62
#define INTERNET_OPTION_FROM_CACHE_TIMEOUT      63
#define INTERNET_OPTION_BYPASS_EDITED_ENTRY     64

#define INTERNET_OPTION_HTTP_DECODING           65

#define INTERNET_OPTION_DIAGNOSTIC_SOCKET_INFO  67
#define INTERNET_OPTION_CODEPAGE                68
#define INTERNET_OPTION_CACHE_TIMESTAMPS        69
#define INTERNET_OPTION_DISABLE_AUTODIAL        70
#define INTERNET_OPTION_MAX_CONNS_PER_SERVER     73
#define INTERNET_OPTION_MAX_CONNS_PER_1_0_SERVER 74
#define INTERNET_OPTION_PER_CONNECTION_OPTION   75
#define INTERNET_OPTION_DIGEST_AUTH_UNLOAD             76
#define INTERNET_OPTION_IGNORE_OFFLINE           77
#define INTERNET_OPTION_IDENTITY                 78
#define INTERNET_OPTION_REMOVE_IDENTITY          79
#define INTERNET_OPTION_ALTER_IDENTITY           80
#define INTERNET_OPTION_SUPPRESS_BEHAVIOR        81
#define INTERNET_OPTION_AUTODIAL_MODE            82
#define INTERNET_OPTION_AUTODIAL_CONNECTION      83
#define INTERNET_OPTION_CLIENT_CERT_CONTEXT      84
#define INTERNET_OPTION_AUTH_FLAGS               85
#define INTERNET_OPTION_COOKIES_3RD_PARTY        86
#define INTERNET_OPTION_DISABLE_PASSPORT_AUTH    87
#define INTERNET_OPTION_SEND_UTF8_SERVERNAME_TO_PROXY         88
#define INTERNET_OPTION_EXEMPT_CONNECTION_LIMIT  89
#define INTERNET_OPTION_ENABLE_PASSPORT_AUTH     90

#define INTERNET_OPTION_HIBERNATE_INACTIVE_WORKER_THREADS       91
#define INTERNET_OPTION_ACTIVATE_WORKER_THREADS                 92
#define INTERNET_OPTION_RESTORE_WORKER_THREAD_DEFAULTS          93
#define INTERNET_OPTION_SOCKET_SEND_BUFFER_LENGTH               94
#define INTERNET_OPTION_PROXY_SETTINGS_CHANGED                  95

#define INTERNET_OPTION_DATAFILE_EXT                            96

#define INTERNET_OPTION_CODEPAGE_PATH                100
#define INTERNET_OPTION_CODEPAGE_EXTRA               101
#define INTERNET_OPTION_IDN                          102
#define INTERNET_OPTION_MAX_CONNS_PER_PROXY          103
#define INTERNET_OPTION_SUPPRESS_SERVER_AUTH         104
#define INTERNET_OPTION_SERVER_CERT_CHAIN_CONTEXT    105


#define INTERNET_OPTION_ENABLE_REDIRECT_CACHE_READ   122


#define INTERNET_OPTION_COMPRESSED_CONTENT_LENGTH    147
#define INTERNET_OPTION_ENABLE_HTTP_PROTOCOL         148
#define INTERNET_OPTION_HTTP_PROTOCOL_USED           149


#define INTERNET_OPTION_ENCODE_EXTRA                 155


#define INTERNET_OPTION_HSTS                         157


#define INTERNET_OPTION_ENTERPRISE_CONTEXT           159


#define INTERNET_OPTION_CONNECTION_FILTER            162
#define INTERNET_OPTION_REFERER_TOKEN_BINDING_HOSTNAME          163


#define INTERNET_OPTION_TOKEN_BINDING_PUBLIC_KEY     181


#define INTERNET_OPTION_COOKIES_SAME_SITE_LEVEL      187


#define INTERNET_OPTION_REQUEST_ANNOTATION           193

#define INTERNET_FIRST_OPTION                   INTERNET_OPTION_CALLBACK
#define INTERNET_LAST_OPTION                    INTERNET_OPTION_REQUEST_ANNOTATION


//
// values for INTERNET_OPTION_PRIORITY
//

#define INTERNET_PRIORITY_FOREGROUND            1000

//
// Values for INTERNET_OPTION_COOKIES_SAME_SITE_LEVEL
//

#define HTTP_COOKIES_SAME_SITE_LEVEL_UNKNOWN        0x0
#define HTTP_COOKIES_SAME_SITE_LEVEL_SAME_SITE      0x1
#define HTTP_COOKIES_SAME_SITE_LEVEL_CROSS_SITE_LAX 0x2
#define HTTP_COOKIES_SAME_SITE_LEVEL_CROSS_SITE     0x3

#define HTTP_COOKIES_SAME_SITE_LEVEL_MAX HTTP_COOKIES_SAME_SITE_LEVEL_CROSS_SITE

//
// Values for INTERNET_OPTION_ENABLE_HTTP_PROTOCOL / INTERNET_OPTION_HTTP_PROTOCOL_USED
//

#define HTTP_PROTOCOL_FLAG_HTTP2 0x2
#define HTTP_PROTOCOL_MASK (HTTP_PROTOCOL_FLAG_HTTP2)

#define INTERNET_OPTION_REQUEST_ANNOTATION_MAX_LENGTH 0xFA00


//
// handle types
//

#define INTERNET_HANDLE_TYPE_INTERNET           1
#define INTERNET_HANDLE_TYPE_CONNECT_FTP        2
#define INTERNET_HANDLE_TYPE_CONNECT_GOPHER     3
#define INTERNET_HANDLE_TYPE_CONNECT_HTTP       4
#define INTERNET_HANDLE_TYPE_FTP_FIND           5
#define INTERNET_HANDLE_TYPE_FTP_FIND_HTML      6
#define INTERNET_HANDLE_TYPE_FTP_FILE           7
#define INTERNET_HANDLE_TYPE_FTP_FILE_HTML      8
#define INTERNET_HANDLE_TYPE_GOPHER_FIND        9
#define INTERNET_HANDLE_TYPE_GOPHER_FIND_HTML   10
#define INTERNET_HANDLE_TYPE_GOPHER_FILE        11
#define INTERNET_HANDLE_TYPE_GOPHER_FILE_HTML   12
#define INTERNET_HANDLE_TYPE_HTTP_REQUEST       13
#define INTERNET_HANDLE_TYPE_FILE_REQUEST       14


//
// values for INTERNET_OPTION_AUTH_FLAGS
//
#define AUTH_FLAG_DISABLE_NEGOTIATE             0x00000001
#define AUTH_FLAG_ENABLE_NEGOTIATE              0x00000002
#define AUTH_FLAG_DISABLE_BASIC_CLEARCHANNEL    0x00000004
#define AUTH_FLAG_DISABLE_SERVER_AUTH           0x00000008


//
// values for INTERNET_OPTION_SECURITY_FLAGS
//

// query only
#define SECURITY_FLAG_SECURE                    0x00000001 // can query only
#define SECURITY_FLAG_STRENGTH_WEAK             0x10000000
#define SECURITY_FLAG_STRENGTH_MEDIUM           0x40000000
#define SECURITY_FLAG_STRENGTH_STRONG           0x20000000
#define SECURITY_FLAG_UNKNOWNBIT                0x80000000
#define SECURITY_FLAG_FORTEZZA                  0x08000000
#define SECURITY_FLAG_NORMALBITNESS             SECURITY_FLAG_STRENGTH_WEAK



// The following are unused
#define SECURITY_FLAG_SSL                       0x00000002
#define SECURITY_FLAG_SSL3                      0x00000004
#define SECURITY_FLAG_PCT                       0x00000008
#define SECURITY_FLAG_PCT4                      0x00000010
#define SECURITY_FLAG_IETFSSL4                  0x00000020

// The following are for backwards compatability only.
#define SECURITY_FLAG_40BIT                     SECURITY_FLAG_STRENGTH_WEAK
#define SECURITY_FLAG_128BIT                    SECURITY_FLAG_STRENGTH_STRONG
#define SECURITY_FLAG_56BIT                     SECURITY_FLAG_STRENGTH_MEDIUM

// settable flags
#define SECURITY_FLAG_IGNORE_REVOCATION         0x00000080
#define SECURITY_FLAG_IGNORE_UNKNOWN_CA         0x00000100
#define SECURITY_FLAG_IGNORE_WRONG_USAGE        0x00000200
#define SECURITY_FLAG_IGNORE_CERT_CN_INVALID    0x00001000 //INTERNET_FLAG_IGNORE_CERT_CN_INVALID
#define SECURITY_FLAG_IGNORE_CERT_DATE_INVALID  0x00002000 //INTERNET_FLAG_IGNORE_CERT_DATE_INVALID
#define SECURITY_FLAG_IGNORE_REDIRECT_TO_HTTPS  0x00004000 //INTERNET_FLAG_IGNORE_REDIRECT_TO_HTTPS
#define SECURITY_FLAG_IGNORE_REDIRECT_TO_HTTP   0x00008000 //INTERNET_FLAG_IGNORE_REDIRECT_TO_HTTP
#define SECURITY_FLAG_IGNORE_WEAK_SIGNATURE     0x00010000

#define SECURITY_SET_MASK       (SECURITY_FLAG_IGNORE_REVOCATION | \
                                 SECURITY_FLAG_IGNORE_UNKNOWN_CA | \
                                 SECURITY_FLAG_IGNORE_WRONG_USAGE | \
                                 SECURITY_FLAG_IGNORE_CERT_CN_INVALID | \
                                 SECURITY_FLAG_IGNORE_CERT_DATE_INVALID | \
                                 SECURITY_FLAG_IGNORE_WEAK_SIGNATURE)

#define SECURITY_FLAG_OPT_IN_WEAK_SIGNATURE     0x00020000

// valid autodial modes
#define AUTODIAL_MODE_NEVER                     1
#define AUTODIAL_MODE_ALWAYS                    2
#define AUTODIAL_MODE_NO_NETWORK_PRESENT        4


BOOLAPI
InternetGetLastResponseInfoA(
    _Out_ LPDWORD lpdwError,
    _Out_writes_opt_(*lpdwBufferLength) LPSTR lpszBuffer,
    _Inout_ LPDWORD lpdwBufferLength
    );
BOOLAPI
InternetGetLastResponseInfoW(
    _Out_ LPDWORD lpdwError,
    _Out_writes_opt_(*lpdwBufferLength) LPWSTR lpszBuffer,
    _Inout_ LPDWORD lpdwBufferLength
    );
#ifdef UNICODE
#define InternetGetLastResponseInfo  InternetGetLastResponseInfoW
#else
#define InternetGetLastResponseInfo  InternetGetLastResponseInfoA
#endif // !UNICODE

//
// callback function for InternetSetStatusCallback
//

typedef
VOID
(CALLBACK * INTERNET_STATUS_CALLBACK)(
    _In_ HINTERNET hInternet,
    _In_opt_ DWORD_PTR dwContext,
    _In_ DWORD dwInternetStatus,
    _In_opt_ LPVOID lpvStatusInformation,
    _In_ DWORD dwStatusInformationLength
    );

typedef INTERNET_STATUS_CALLBACK * LPINTERNET_STATUS_CALLBACK;

INTERNETAPI_(INTERNET_STATUS_CALLBACK) InternetSetStatusCallbackA(
    _In_ HINTERNET hInternet,
    _In_opt_ INTERNET_STATUS_CALLBACK lpfnInternetCallback
    );

INTERNETAPI_(INTERNET_STATUS_CALLBACK) InternetSetStatusCallbackW(
    _In_ HINTERNET hInternet,
    _In_opt_ INTERNET_STATUS_CALLBACK lpfnInternetCallback
    );

#ifdef UNICODE
#define InternetSetStatusCallback  InternetSetStatusCallbackW
#else
#ifdef _WINX32_
#define InternetSetStatusCallback  InternetSetStatusCallbackA
#else
INTERNETAPI_(INTERNET_STATUS_CALLBACK) InternetSetStatusCallback(
    _In_ HINTERNET hInternet,
    _In_opt_ INTERNET_STATUS_CALLBACK lpfnInternetCallback
    );
#endif // _WINX32_
#endif // !UNICODE

//
// status manifests for Internet status callback
//

#define INTERNET_STATUS_RESOLVING_NAME          10
#define INTERNET_STATUS_NAME_RESOLVED           11
#define INTERNET_STATUS_CONNECTING_TO_SERVER    20
#define INTERNET_STATUS_CONNECTED_TO_SERVER     21
#define INTERNET_STATUS_SENDING_REQUEST         30
#define INTERNET_STATUS_REQUEST_SENT            31
#define INTERNET_STATUS_RECEIVING_RESPONSE      40
#define INTERNET_STATUS_RESPONSE_RECEIVED       41
#define INTERNET_STATUS_CTL_RESPONSE_RECEIVED   42
#define INTERNET_STATUS_PREFETCH                43
#define INTERNET_STATUS_CLOSING_CONNECTION      50
#define INTERNET_STATUS_CONNECTION_CLOSED       51
#define INTERNET_STATUS_HANDLE_CREATED          60
#define INTERNET_STATUS_HANDLE_CLOSING          70
#define INTERNET_STATUS_DETECTING_PROXY         80
#define INTERNET_STATUS_REQUEST_COMPLETE        100
#define INTERNET_STATUS_REDIRECT                110
#define INTERNET_STATUS_INTERMEDIATE_RESPONSE   120
#define INTERNET_STATUS_USER_INPUT_REQUIRED     140
#define INTERNET_STATUS_STATE_CHANGE            200
#define INTERNET_STATUS_COOKIE_SENT             320
#define INTERNET_STATUS_COOKIE_RECEIVED         321
#define INTERNET_STATUS_PRIVACY_IMPACTED        324
#define INTERNET_STATUS_P3P_HEADER              325
#define INTERNET_STATUS_P3P_POLICYREF           326
#define INTERNET_STATUS_COOKIE_HISTORY          327


//
// the following can be indicated in a state change notification:
//

#define INTERNET_STATE_CONNECTED                0x00000001  // connected state (mutually exclusive with disconnected)
#define INTERNET_STATE_DISCONNECTED             0x00000002  // disconnected from network
#define INTERNET_STATE_DISCONNECTED_BY_USER     0x00000010  // disconnected by user request
#define INTERNET_STATE_IDLE                     0x00000100  // no network requests being made (by Wininet)
#define INTERNET_STATE_BUSY                     0x00000200  // network requests being made (by Wininet)

//
// the following values are used for cookie state:
//

typedef enum {

    COOKIE_STATE_UNKNOWN        = 0x0,

    COOKIE_STATE_ACCEPT         = 0x1,
    COOKIE_STATE_PROMPT         = 0x2,
    COOKIE_STATE_LEASH          = 0x3,
    COOKIE_STATE_DOWNGRADE      = 0x4,
    COOKIE_STATE_REJECT         = 0x5,

    COOKIE_STATE_MAX            = COOKIE_STATE_REJECT,
}
InternetCookieState;


typedef struct {

    int         cSession;           // Session cookies received
    int         cPersistent;        // Persistent cookies received

    int         cAccepted;          // Number of cookies accepted
    int         cLeashed;           //               ... leashed
    int         cDowngraded;        //               ... converted to session-cookies
    int         cBlocked;           //               ... rejected

    const char *pszLocation;        // Optional: URL associated with reported cookie events
                                    // This can be used to override request URL
}
IncomingCookieState;

typedef struct {

    int     cSent;
    int     cSuppressed;

    const char *pszLocation;        // Optional: URL associated with reported cookie events
                                    // This can be used to override request URL
}
OutgoingCookieState;

typedef struct {

    BOOL    fAccepted;
    BOOL    fLeashed;
    BOOL    fDowngraded;
    BOOL    fRejected;
}
InternetCookieHistory;


typedef struct {

    DWORD   dwCookieState;
    BOOL    fAllowSession;
}
CookieDecision;


//
// if the following value is returned by InternetSetStatusCallback, then
// probably an invalid (non-code) address was supplied for the callback
//

#define INTERNET_INVALID_STATUS_CALLBACK        ((INTERNET_STATUS_CALLBACK)(-1L))

//
// FTP
//

//
// manifests
//

#define FTP_TRANSFER_TYPE_UNKNOWN   0x00000000
#define FTP_TRANSFER_TYPE_ASCII     0x00000001
#define FTP_TRANSFER_TYPE_BINARY    0x00000002

#define FTP_TRANSFER_TYPE_MASK      (FTP_TRANSFER_TYPE_ASCII | FTP_TRANSFER_TYPE_BINARY)

//
// prototypes
//

INTERNETAPI_(HINTERNET) FtpFindFirstFileA(
    _In_ HINTERNET hConnect,
    _In_opt_ LPCSTR lpszSearchFile,
    _Out_opt_ LPWIN32_FIND_DATAA lpFindFileData,
    _In_ DWORD dwFlags,
    _In_opt_ DWORD_PTR dwContext
    );
INTERNETAPI_(HINTERNET) FtpFindFirstFileW(
    _In_ HINTERNET hConnect,
    _In_opt_ LPCWSTR lpszSearchFile,
    _Out_opt_ LPWIN32_FIND_DATAW lpFindFileData,
    _In_ DWORD dwFlags,
    _In_opt_ DWORD_PTR dwContext
    );
#ifdef UNICODE
#define FtpFindFirstFile  FtpFindFirstFileW
#else
#define FtpFindFirstFile  FtpFindFirstFileA
#endif // !UNICODE

BOOLAPI FtpGetFileA(
    _In_ HINTERNET hConnect,
    _In_ LPCSTR lpszRemoteFile,
    _In_ LPCSTR lpszNewFile,
    _In_ BOOL fFailIfExists,
    _In_ DWORD dwFlagsAndAttributes,
    _In_ DWORD dwFlags,
    _In_opt_ DWORD_PTR dwContext
    );
BOOLAPI FtpGetFileW(
    _In_ HINTERNET hConnect,
    _In_ LPCWSTR lpszRemoteFile,
    _In_ LPCWSTR lpszNewFile,
    _In_ BOOL fFailIfExists,
    _In_ DWORD dwFlagsAndAttributes,
    _In_ DWORD dwFlags,
    _In_opt_ DWORD_PTR dwContext
    );
#ifdef UNICODE
#define FtpGetFile  FtpGetFileW
#else
#define FtpGetFile  FtpGetFileA
#endif // !UNICODE

BOOLAPI FtpPutFileA(
    _In_ HINTERNET hConnect,
    _In_ LPCSTR lpszLocalFile,
    _In_ LPCSTR lpszNewRemoteFile,
    _In_ DWORD dwFlags,
    _In_opt_ DWORD_PTR dwContext
    );
BOOLAPI FtpPutFileW(
    _In_ HINTERNET hConnect,
    _In_ LPCWSTR lpszLocalFile,
    _In_ LPCWSTR lpszNewRemoteFile,
    _In_ DWORD dwFlags,
    _In_opt_ DWORD_PTR dwContext
    );
#ifdef UNICODE
#define FtpPutFile  FtpPutFileW
#else
#define FtpPutFile  FtpPutFileA
#endif // !UNICODE

BOOLAPI FtpGetFileEx(
    _In_ HINTERNET hFtpSession,
    _In_ LPCSTR lpszRemoteFile,
    _In_ LPCWSTR lpszNewFile,
    _In_ BOOL fFailIfExists,
    _In_ DWORD dwFlagsAndAttributes,
    _In_ DWORD dwFlags,
    _In_opt_ DWORD_PTR dwContext
    );

BOOLAPI FtpPutFileEx(
    _In_ HINTERNET hFtpSession,
    _In_ LPCWSTR lpszLocalFile,
    _In_ LPCSTR lpszNewRemoteFile,
    _In_ DWORD dwFlags,
    _In_opt_ DWORD_PTR dwContext
    );

BOOLAPI FtpDeleteFileA(
    _In_ HINTERNET hConnect,
    _In_ LPCSTR lpszFileName
    );
BOOLAPI FtpDeleteFileW(
    _In_ HINTERNET hConnect,
    _In_ LPCWSTR lpszFileName
    );
#ifdef UNICODE
#define FtpDeleteFile  FtpDeleteFileW
#else
#define FtpDeleteFile  FtpDeleteFileA
#endif // !UNICODE

BOOLAPI FtpRenameFileA(
    _In_ HINTERNET hConnect,
    _In_ LPCSTR lpszExisting,
    _In_ LPCSTR lpszNew
    );
BOOLAPI FtpRenameFileW(
    _In_ HINTERNET hConnect,
    _In_ LPCWSTR lpszExisting,
    _In_ LPCWSTR lpszNew
    );
#ifdef UNICODE
#define FtpRenameFile  FtpRenameFileW
#else
#define FtpRenameFile  FtpRenameFileA
#endif // !UNICODE

INTERNETAPI_(HINTERNET) FtpOpenFileA(
    _In_ HINTERNET hConnect,
    _In_ LPCSTR lpszFileName,
    _In_ DWORD dwAccess,
    _In_ DWORD dwFlags,
    _In_opt_ DWORD_PTR dwContext
    );
INTERNETAPI_(HINTERNET) FtpOpenFileW(
    _In_ HINTERNET hConnect,
    _In_ LPCWSTR lpszFileName,
    _In_ DWORD dwAccess,
    _In_ DWORD dwFlags,
    _In_opt_ DWORD_PTR dwContext
    );
#ifdef UNICODE
#define FtpOpenFile  FtpOpenFileW
#else
#define FtpOpenFile  FtpOpenFileA
#endif // !UNICODE

BOOLAPI FtpCreateDirectoryA(
    _In_ HINTERNET hConnect,
    _In_ LPCSTR lpszDirectory
    );
BOOLAPI FtpCreateDirectoryW(
    _In_ HINTERNET hConnect,
    _In_ LPCWSTR lpszDirectory
    );
#ifdef UNICODE
#define FtpCreateDirectory  FtpCreateDirectoryW
#else
#define FtpCreateDirectory  FtpCreateDirectoryA
#endif // !UNICODE

BOOLAPI FtpRemoveDirectoryA(
    _In_ HINTERNET hConnect,
    _In_ LPCSTR lpszDirectory
    );
BOOLAPI FtpRemoveDirectoryW(
    _In_ HINTERNET hConnect,
    _In_ LPCWSTR lpszDirectory
    );
#ifdef UNICODE
#define FtpRemoveDirectory  FtpRemoveDirectoryW
#else
#define FtpRemoveDirectory  FtpRemoveDirectoryA
#endif // !UNICODE

BOOLAPI FtpSetCurrentDirectoryA(
    _In_ HINTERNET hConnect,
    _In_ LPCSTR lpszDirectory
    );
BOOLAPI FtpSetCurrentDirectoryW(
    _In_ HINTERNET hConnect,
    _In_ LPCWSTR lpszDirectory
    );
#ifdef UNICODE
#define FtpSetCurrentDirectory  FtpSetCurrentDirectoryW
#else
#define FtpSetCurrentDirectory  FtpSetCurrentDirectoryA
#endif // !UNICODE

BOOLAPI FtpGetCurrentDirectoryA(
    _In_ HINTERNET hConnect,
    _Out_writes_(*lpdwCurrentDirectory) LPSTR lpszCurrentDirectory,
    _Inout_ LPDWORD lpdwCurrentDirectory
    );
BOOLAPI FtpGetCurrentDirectoryW(
    _In_ HINTERNET hConnect,
    _Out_writes_(*lpdwCurrentDirectory) LPWSTR lpszCurrentDirectory,
    _Inout_ LPDWORD lpdwCurrentDirectory
    );
#ifdef UNICODE
#define FtpGetCurrentDirectory  FtpGetCurrentDirectoryW
#else
#define FtpGetCurrentDirectory  FtpGetCurrentDirectoryA
#endif // !UNICODE

BOOLAPI FtpCommandA(
    _In_ HINTERNET hConnect,
    _In_ BOOL fExpectResponse,
    _In_ DWORD dwFlags,
    _In_ LPCSTR lpszCommand,
    _In_opt_ DWORD_PTR dwContext,
    _Out_opt_ HINTERNET *phFtpCommand
    );
BOOLAPI FtpCommandW(
    _In_ HINTERNET hConnect,
    _In_ BOOL fExpectResponse,
    _In_ DWORD dwFlags,
    _In_ LPCWSTR lpszCommand,
    _In_opt_ DWORD_PTR dwContext,
    _Out_opt_ HINTERNET *phFtpCommand
    );
#ifdef UNICODE
#define FtpCommand  FtpCommandW
#else
#define FtpCommand  FtpCommandA
#endif // !UNICODE

INTERNETAPI_(DWORD) FtpGetFileSize(
    _In_ HINTERNET hFile,
    _Out_opt_ LPDWORD lpdwFileSizeHigh
    );



//
// Gopher
//

//
// manifests
//

//
// string field lengths (in characters, not bytes)
//

#define MAX_GOPHER_DISPLAY_TEXT     128
#define MAX_GOPHER_SELECTOR_TEXT    256
#define MAX_GOPHER_HOST_NAME        INTERNET_MAX_HOST_NAME_LENGTH
#define MAX_GOPHER_LOCATOR_LENGTH   (1                                  \
                                    + MAX_GOPHER_DISPLAY_TEXT           \
                                    + 1                                 \
                                    + MAX_GOPHER_SELECTOR_TEXT          \
                                    + 1                                 \
                                    + MAX_GOPHER_HOST_NAME              \
                                    + 1                                 \
                                    + INTERNET_MAX_PORT_NUMBER_LENGTH   \
                                    + 1                                 \
                                    + 1                                 \
                                    + 2                                 \
                                    )

//
// structures/types
//

//
// GOPHER_FIND_DATA - returns the results of a GopherFindFirstFile()/
// InternetFindNextFile() request
//

typedef struct {
    CHAR   DisplayString[MAX_GOPHER_DISPLAY_TEXT + 1];
    DWORD GopherType;   // GOPHER_TYPE_, if known
    DWORD SizeLow;
    DWORD SizeHigh;
    FILETIME LastModificationTime;
    CHAR   Locator[MAX_GOPHER_LOCATOR_LENGTH + 1];
} GOPHER_FIND_DATAA, * LPGOPHER_FIND_DATAA;
typedef struct {
    WCHAR  DisplayString[MAX_GOPHER_DISPLAY_TEXT + 1];
    DWORD GopherType;   // GOPHER_TYPE_, if known
    DWORD SizeLow;
    DWORD SizeHigh;
    FILETIME LastModificationTime;
    WCHAR  Locator[MAX_GOPHER_LOCATOR_LENGTH + 1];
} GOPHER_FIND_DATAW, * LPGOPHER_FIND_DATAW;
#ifdef UNICODE
typedef GOPHER_FIND_DATAW GOPHER_FIND_DATA;
typedef LPGOPHER_FIND_DATAW LPGOPHER_FIND_DATA;
#else
typedef GOPHER_FIND_DATAA GOPHER_FIND_DATA;
typedef LPGOPHER_FIND_DATAA LPGOPHER_FIND_DATA;
#endif // UNICODE

//
// manifests for GopherType
//

#define GOPHER_TYPE_TEXT_FILE       0x00000001
#define GOPHER_TYPE_DIRECTORY       0x00000002
#define GOPHER_TYPE_CSO             0x00000004
#define GOPHER_TYPE_ERROR           0x00000008
#define GOPHER_TYPE_MAC_BINHEX      0x00000010
#define GOPHER_TYPE_DOS_ARCHIVE     0x00000020
#define GOPHER_TYPE_UNIX_UUENCODED  0x00000040
#define GOPHER_TYPE_INDEX_SERVER    0x00000080
#define GOPHER_TYPE_TELNET          0x00000100
#define GOPHER_TYPE_BINARY          0x00000200
#define GOPHER_TYPE_REDUNDANT       0x00000400
#define GOPHER_TYPE_TN3270          0x00000800
#define GOPHER_TYPE_GIF             0x00001000
#define GOPHER_TYPE_IMAGE           0x00002000
#define GOPHER_TYPE_BITMAP          0x00004000
#define GOPHER_TYPE_MOVIE           0x00008000
#define GOPHER_TYPE_SOUND           0x00010000
#define GOPHER_TYPE_HTML            0x00020000
#define GOPHER_TYPE_PDF             0x00040000
#define GOPHER_TYPE_CALENDAR        0x00080000
#define GOPHER_TYPE_INLINE          0x00100000
#define GOPHER_TYPE_UNKNOWN         0x20000000
#define GOPHER_TYPE_ASK             0x40000000
#define GOPHER_TYPE_GOPHER_PLUS     0x80000000

//
// gopher type macros
//

#define IS_GOPHER_FILE(type)            (BOOL)(((type) & GOPHER_TYPE_FILE_MASK) ? TRUE : FALSE)
#define IS_GOPHER_DIRECTORY(type)       (BOOL)(((type) & GOPHER_TYPE_DIRECTORY) ? TRUE : FALSE)
#define IS_GOPHER_PHONE_SERVER(type)    (BOOL)(((type) & GOPHER_TYPE_CSO) ? TRUE : FALSE)
#define IS_GOPHER_ERROR(type)           (BOOL)(((type) & GOPHER_TYPE_ERROR) ? TRUE : FALSE)
#define IS_GOPHER_INDEX_SERVER(type)    (BOOL)(((type) & GOPHER_TYPE_INDEX_SERVER) ? TRUE : FALSE)
#define IS_GOPHER_TELNET_SESSION(type)  (BOOL)(((type) & GOPHER_TYPE_TELNET) ? TRUE : FALSE)
#define IS_GOPHER_BACKUP_SERVER(type)   (BOOL)(((type) & GOPHER_TYPE_REDUNDANT) ? TRUE : FALSE)
#define IS_GOPHER_TN3270_SESSION(type)  (BOOL)(((type) & GOPHER_TYPE_TN3270) ? TRUE : FALSE)
#define IS_GOPHER_ASK(type)             (BOOL)(((type) & GOPHER_TYPE_ASK) ? TRUE : FALSE)
#define IS_GOPHER_PLUS(type)            (BOOL)(((type) & GOPHER_TYPE_GOPHER_PLUS) ? TRUE : FALSE)

#define IS_GOPHER_TYPE_KNOWN(type)      (BOOL)(((type) & GOPHER_TYPE_UNKNOWN) ? FALSE : TRUE)

//
// GOPHER_TYPE_FILE_MASK - use this to determine if a locator identifies a
// (known) file type
//

#define GOPHER_TYPE_FILE_MASK       (GOPHER_TYPE_TEXT_FILE          \
                                    | GOPHER_TYPE_MAC_BINHEX        \
                                    | GOPHER_TYPE_DOS_ARCHIVE       \
                                    | GOPHER_TYPE_UNIX_UUENCODED    \
                                    | GOPHER_TYPE_BINARY            \
                                    | GOPHER_TYPE_GIF               \
                                    | GOPHER_TYPE_IMAGE             \
                                    | GOPHER_TYPE_BITMAP            \
                                    | GOPHER_TYPE_MOVIE             \
                                    | GOPHER_TYPE_SOUND             \
                                    | GOPHER_TYPE_HTML              \
                                    | GOPHER_TYPE_PDF               \
                                    | GOPHER_TYPE_CALENDAR          \
                                    | GOPHER_TYPE_INLINE            \
                                    )

//
// structured gopher attributes (as defined in gopher+ protocol document)
//

typedef struct {
    LPCTSTR Comment;
    LPCTSTR EmailAddress;
} GOPHER_ADMIN_ATTRIBUTE_TYPE, * LPGOPHER_ADMIN_ATTRIBUTE_TYPE;

typedef struct {
    FILETIME DateAndTime;
} GOPHER_MOD_DATE_ATTRIBUTE_TYPE, * LPGOPHER_MOD_DATE_ATTRIBUTE_TYPE;

typedef struct {
    DWORD Ttl;
} GOPHER_TTL_ATTRIBUTE_TYPE, * LPGOPHER_TTL_ATTRIBUTE_TYPE;

typedef struct {
    INT Score;
} GOPHER_SCORE_ATTRIBUTE_TYPE, * LPGOPHER_SCORE_ATTRIBUTE_TYPE;

typedef struct {
    INT LowerBound;
    INT UpperBound;
} GOPHER_SCORE_RANGE_ATTRIBUTE_TYPE, * LPGOPHER_SCORE_RANGE_ATTRIBUTE_TYPE;

typedef struct {
    LPCTSTR Site;
} GOPHER_SITE_ATTRIBUTE_TYPE, * LPGOPHER_SITE_ATTRIBUTE_TYPE;

typedef struct {
    LPCTSTR Organization;
} GOPHER_ORGANIZATION_ATTRIBUTE_TYPE, * LPGOPHER_ORGANIZATION_ATTRIBUTE_TYPE;

typedef struct {
    LPCTSTR Location;
} GOPHER_LOCATION_ATTRIBUTE_TYPE, * LPGOPHER_LOCATION_ATTRIBUTE_TYPE;

typedef struct {
    INT DegreesNorth;
    INT MinutesNorth;
    INT SecondsNorth;
    INT DegreesEast;
    INT MinutesEast;
    INT SecondsEast;
} GOPHER_GEOGRAPHICAL_LOCATION_ATTRIBUTE_TYPE, * LPGOPHER_GEOGRAPHICAL_LOCATION_ATTRIBUTE_TYPE;

typedef struct {
    INT Zone;
} GOPHER_TIMEZONE_ATTRIBUTE_TYPE, * LPGOPHER_TIMEZONE_ATTRIBUTE_TYPE;

typedef struct {
    LPCTSTR Provider;
} GOPHER_PROVIDER_ATTRIBUTE_TYPE, * LPGOPHER_PROVIDER_ATTRIBUTE_TYPE;

typedef struct {
    LPCTSTR Version;
} GOPHER_VERSION_ATTRIBUTE_TYPE, * LPGOPHER_VERSION_ATTRIBUTE_TYPE;

typedef struct {
    LPCTSTR ShortAbstract;
    LPCTSTR AbstractFile;
} GOPHER_ABSTRACT_ATTRIBUTE_TYPE, * LPGOPHER_ABSTRACT_ATTRIBUTE_TYPE;

typedef struct {
    LPCTSTR ContentType;
    LPCTSTR Language;
    DWORD Size;
} GOPHER_VIEW_ATTRIBUTE_TYPE, * LPGOPHER_VIEW_ATTRIBUTE_TYPE;

typedef struct {
    BOOL TreeWalk;
} GOPHER_VERONICA_ATTRIBUTE_TYPE, * LPGOPHER_VERONICA_ATTRIBUTE_TYPE;

typedef struct {
    LPCTSTR QuestionType;
    LPCTSTR QuestionText;
} GOPHER_ASK_ATTRIBUTE_TYPE, * LPGOPHER_ASK_ATTRIBUTE_TYPE;

//
// GOPHER_UNKNOWN_ATTRIBUTE_TYPE - this is returned if we retrieve an attribute
// that is not specified in the current gopher/gopher+ documentation. It is up
// to the application to parse the information
//

typedef struct {
    LPCTSTR Text;
} GOPHER_UNKNOWN_ATTRIBUTE_TYPE, * LPGOPHER_UNKNOWN_ATTRIBUTE_TYPE;

//
// GOPHER_ATTRIBUTE_TYPE - returned in the user's buffer when an enumerated
// GopherGetAttribute call is made
//

typedef struct {
    DWORD CategoryId;   // e.g. GOPHER_CATEGORY_ID_ADMIN
    DWORD AttributeId;  // e.g. GOPHER_ATTRIBUTE_ID_ADMIN
    union {
        GOPHER_ADMIN_ATTRIBUTE_TYPE Admin;
        GOPHER_MOD_DATE_ATTRIBUTE_TYPE ModDate;
        GOPHER_TTL_ATTRIBUTE_TYPE Ttl;
        GOPHER_SCORE_ATTRIBUTE_TYPE Score;
        GOPHER_SCORE_RANGE_ATTRIBUTE_TYPE ScoreRange;
        GOPHER_SITE_ATTRIBUTE_TYPE Site;
        GOPHER_ORGANIZATION_ATTRIBUTE_TYPE Organization;
        GOPHER_LOCATION_ATTRIBUTE_TYPE Location;
        GOPHER_GEOGRAPHICAL_LOCATION_ATTRIBUTE_TYPE GeographicalLocation;
        GOPHER_TIMEZONE_ATTRIBUTE_TYPE TimeZone;
        GOPHER_PROVIDER_ATTRIBUTE_TYPE Provider;
        GOPHER_VERSION_ATTRIBUTE_TYPE Version;
        GOPHER_ABSTRACT_ATTRIBUTE_TYPE Abstract;
        GOPHER_VIEW_ATTRIBUTE_TYPE View;
        GOPHER_VERONICA_ATTRIBUTE_TYPE Veronica;
        GOPHER_ASK_ATTRIBUTE_TYPE Ask;
        GOPHER_UNKNOWN_ATTRIBUTE_TYPE Unknown;
    } AttributeType;
} GOPHER_ATTRIBUTE_TYPE, * LPGOPHER_ATTRIBUTE_TYPE;

#define MAX_GOPHER_CATEGORY_NAME    128     // arbitrary
#define MAX_GOPHER_ATTRIBUTE_NAME   128     //     "
#define MIN_GOPHER_ATTRIBUTE_LENGTH 256     //     "

//
// known gopher attribute categories. See below for ordinals
//

#define GOPHER_INFO_CATEGORY        TEXT("+INFO")
#define GOPHER_ADMIN_CATEGORY       TEXT("+ADMIN")
#define GOPHER_VIEWS_CATEGORY       TEXT("+VIEWS")
#define GOPHER_ABSTRACT_CATEGORY    TEXT("+ABSTRACT")
#define GOPHER_VERONICA_CATEGORY    TEXT("+VERONICA")

//
// known gopher attributes. These are the attribute names as defined in the
// gopher+ protocol document
//

#define GOPHER_ADMIN_ATTRIBUTE      TEXT("Admin")
#define GOPHER_MOD_DATE_ATTRIBUTE   TEXT("Mod-Date")
#define GOPHER_TTL_ATTRIBUTE        TEXT("TTL")
#define GOPHER_SCORE_ATTRIBUTE      TEXT("Score")
#define GOPHER_RANGE_ATTRIBUTE      TEXT("Score-range")
#define GOPHER_SITE_ATTRIBUTE       TEXT("Site")
#define GOPHER_ORG_ATTRIBUTE        TEXT("Org")
#define GOPHER_LOCATION_ATTRIBUTE   TEXT("Loc")
#define GOPHER_GEOG_ATTRIBUTE       TEXT("Geog")
#define GOPHER_TIMEZONE_ATTRIBUTE   TEXT("TZ")
#define GOPHER_PROVIDER_ATTRIBUTE   TEXT("Provider")
#define GOPHER_VERSION_ATTRIBUTE    TEXT("Version")
#define GOPHER_ABSTRACT_ATTRIBUTE   TEXT("Abstract")
#define GOPHER_VIEW_ATTRIBUTE       TEXT("View")
#define GOPHER_TREEWALK_ATTRIBUTE   TEXT("treewalk")

//
// identifiers for attribute strings
//

#define GOPHER_ATTRIBUTE_ID_BASE        0xabcccc00

#define GOPHER_CATEGORY_ID_ALL          (GOPHER_ATTRIBUTE_ID_BASE + 1)

#define GOPHER_CATEGORY_ID_INFO         (GOPHER_ATTRIBUTE_ID_BASE + 2)
#define GOPHER_CATEGORY_ID_ADMIN        (GOPHER_ATTRIBUTE_ID_BASE + 3)
#define GOPHER_CATEGORY_ID_VIEWS        (GOPHER_ATTRIBUTE_ID_BASE + 4)
#define GOPHER_CATEGORY_ID_ABSTRACT     (GOPHER_ATTRIBUTE_ID_BASE + 5)
#define GOPHER_CATEGORY_ID_VERONICA     (GOPHER_ATTRIBUTE_ID_BASE + 6)
#define GOPHER_CATEGORY_ID_ASK          (GOPHER_ATTRIBUTE_ID_BASE + 7)

#define GOPHER_CATEGORY_ID_UNKNOWN      (GOPHER_ATTRIBUTE_ID_BASE + 8)

#define GOPHER_ATTRIBUTE_ID_ALL         (GOPHER_ATTRIBUTE_ID_BASE + 9)

#define GOPHER_ATTRIBUTE_ID_ADMIN       (GOPHER_ATTRIBUTE_ID_BASE + 10)
#define GOPHER_ATTRIBUTE_ID_MOD_DATE    (GOPHER_ATTRIBUTE_ID_BASE + 11)
#define GOPHER_ATTRIBUTE_ID_TTL         (GOPHER_ATTRIBUTE_ID_BASE + 12)
#define GOPHER_ATTRIBUTE_ID_SCORE       (GOPHER_ATTRIBUTE_ID_BASE + 13)
#define GOPHER_ATTRIBUTE_ID_RANGE       (GOPHER_ATTRIBUTE_ID_BASE + 14)
#define GOPHER_ATTRIBUTE_ID_SITE        (GOPHER_ATTRIBUTE_ID_BASE + 15)
#define GOPHER_ATTRIBUTE_ID_ORG         (GOPHER_ATTRIBUTE_ID_BASE + 16)
#define GOPHER_ATTRIBUTE_ID_LOCATION    (GOPHER_ATTRIBUTE_ID_BASE + 17)
#define GOPHER_ATTRIBUTE_ID_GEOG        (GOPHER_ATTRIBUTE_ID_BASE + 18)
#define GOPHER_ATTRIBUTE_ID_TIMEZONE    (GOPHER_ATTRIBUTE_ID_BASE + 19)
#define GOPHER_ATTRIBUTE_ID_PROVIDER    (GOPHER_ATTRIBUTE_ID_BASE + 20)
#define GOPHER_ATTRIBUTE_ID_VERSION     (GOPHER_ATTRIBUTE_ID_BASE + 21)
#define GOPHER_ATTRIBUTE_ID_ABSTRACT    (GOPHER_ATTRIBUTE_ID_BASE + 22)
#define GOPHER_ATTRIBUTE_ID_VIEW        (GOPHER_ATTRIBUTE_ID_BASE + 23)
#define GOPHER_ATTRIBUTE_ID_TREEWALK    (GOPHER_ATTRIBUTE_ID_BASE + 24)

#define GOPHER_ATTRIBUTE_ID_UNKNOWN     (GOPHER_ATTRIBUTE_ID_BASE + 25)

//
// prototypes
//

BOOLAPI GopherCreateLocatorA(
    _In_ LPCSTR lpszHost,
    _In_ INTERNET_PORT nServerPort,
    _In_opt_ LPCSTR lpszDisplayString,
    _In_opt_ LPCSTR lpszSelectorString,
    _In_ DWORD dwGopherType,
    _Out_writes_opt_(*lpdwBufferLength) LPSTR lpszLocator,
    _Inout_ LPDWORD lpdwBufferLength
    );
BOOLAPI GopherCreateLocatorW(
    _In_ LPCWSTR lpszHost,
    _In_ INTERNET_PORT nServerPort,
    _In_opt_ LPCWSTR lpszDisplayString,
    _In_opt_ LPCWSTR lpszSelectorString,
    _In_ DWORD dwGopherType,
    _Out_writes_opt_(*lpdwBufferLength) LPWSTR lpszLocator,
    _Inout_ LPDWORD lpdwBufferLength
    );
#ifdef UNICODE
#define GopherCreateLocator  GopherCreateLocatorW
#else
#define GopherCreateLocator  GopherCreateLocatorA
#endif // !UNICODE

BOOLAPI GopherGetLocatorTypeA(
    _In_ LPCSTR lpszLocator,
    _Out_ LPDWORD lpdwGopherType
    );
BOOLAPI GopherGetLocatorTypeW(
    _In_ LPCWSTR lpszLocator,
    _Out_ LPDWORD lpdwGopherType
    );
#ifdef UNICODE
#define GopherGetLocatorType  GopherGetLocatorTypeW
#else
#define GopherGetLocatorType  GopherGetLocatorTypeA
#endif // !UNICODE

INTERNETAPI_(HINTERNET) GopherFindFirstFileA(
    _In_ HINTERNET hConnect,
    _In_opt_ LPCSTR lpszLocator,
    _In_opt_ LPCSTR lpszSearchString,
    _Out_opt_ LPGOPHER_FIND_DATAA lpFindData,
    _In_ DWORD dwFlags,
    _In_opt_ DWORD_PTR dwContext
    );
INTERNETAPI_(HINTERNET) GopherFindFirstFileW(
    _In_ HINTERNET hConnect,
    _In_opt_ LPCWSTR lpszLocator,
    _In_opt_ LPCWSTR lpszSearchString,
    _Out_opt_ LPGOPHER_FIND_DATAW lpFindData,
    _In_ DWORD dwFlags,
    _In_opt_ DWORD_PTR dwContext
    );
#ifdef UNICODE
#define GopherFindFirstFile  GopherFindFirstFileW
#else
#define GopherFindFirstFile  GopherFindFirstFileA
#endif // !UNICODE

INTERNETAPI_(HINTERNET) GopherOpenFileA(
    _In_ HINTERNET hConnect,
    _In_ LPCSTR lpszLocator,
    _In_opt_ LPCSTR lpszView,
    _In_ DWORD dwFlags,
    _In_opt_ DWORD_PTR dwContext
    );
INTERNETAPI_(HINTERNET) GopherOpenFileW(
    _In_ HINTERNET hConnect,
    _In_ LPCWSTR lpszLocator,
    _In_opt_ LPCWSTR lpszView,
    _In_ DWORD dwFlags,
    _In_opt_ DWORD_PTR dwContext
    );
#ifdef UNICODE
#define GopherOpenFile  GopherOpenFileW
#else
#define GopherOpenFile  GopherOpenFileA
#endif // !UNICODE

typedef BOOL (CALLBACK * GOPHER_ATTRIBUTE_ENUMERATOR)(
    _In_ LPGOPHER_ATTRIBUTE_TYPE lpAttributeInfo,
    _In_ DWORD dwError
    );

BOOLAPI GopherGetAttributeA(
    _In_ HINTERNET hConnect,
    _In_ LPCSTR lpszLocator,
    _In_opt_ LPCSTR lpszAttributeName,
    _At_((LPSTR)lpBuffer, _Out_writes_(dwBufferLength)) LPBYTE lpBuffer,
    _In_ DWORD dwBufferLength,
    _Out_ LPDWORD lpdwCharactersReturned,
    _In_opt_ GOPHER_ATTRIBUTE_ENUMERATOR lpfnEnumerator,
    _In_opt_ DWORD_PTR dwContext
    );
BOOLAPI GopherGetAttributeW(
    _In_ HINTERNET hConnect,
    _In_ LPCWSTR lpszLocator,
    _In_opt_ LPCWSTR lpszAttributeName,
    _At_((LPWSTR)lpBuffer, _Out_writes_(dwBufferLength)) LPBYTE lpBuffer,
    _In_ DWORD dwBufferLength,
    _Out_ LPDWORD lpdwCharactersReturned,
    _In_opt_ GOPHER_ATTRIBUTE_ENUMERATOR lpfnEnumerator,
    _In_opt_ DWORD_PTR dwContext
    );
#ifdef UNICODE
#define GopherGetAttribute  GopherGetAttributeW
#else
#define GopherGetAttribute  GopherGetAttributeA
#endif // !UNICODE

//
// HTTP
//

//
// manifests
//

//
// the default major/minor HTTP version numbers
//

#define HTTP_MAJOR_VERSION      1
#define HTTP_MINOR_VERSION      0

#define HTTP_VERSIONA           "HTTP/1.0"
#define HTTP_VERSIONW           L"HTTP/1.0"

#ifdef UNICODE
#define HTTP_VERSION            HTTP_VERSIONW
#else
#define HTTP_VERSION            HTTP_VERSIONA
#endif

//
// HttpQueryInfo info levels. Generally, there is one info level
// for each potential RFC822/HTTP/MIME header that an HTTP server
// may send as part of a request response.
//
// The HTTP_QUERY_RAW_HEADERS info level is provided for clients
// that choose to perform their own header parsing.
//


#define HTTP_QUERY_MIME_VERSION                 0
#define HTTP_QUERY_CONTENT_TYPE                 1
#define HTTP_QUERY_CONTENT_TRANSFER_ENCODING    2
#define HTTP_QUERY_CONTENT_ID                   3
#define HTTP_QUERY_CONTENT_DESCRIPTION          4
#define HTTP_QUERY_CONTENT_LENGTH               5
#define HTTP_QUERY_CONTENT_LANGUAGE             6
#define HTTP_QUERY_ALLOW                        7
#define HTTP_QUERY_PUBLIC                       8
#define HTTP_QUERY_DATE                         9
#define HTTP_QUERY_EXPIRES                      10
#define HTTP_QUERY_LAST_MODIFIED                11
#define HTTP_QUERY_MESSAGE_ID                   12
#define HTTP_QUERY_URI                          13
#define HTTP_QUERY_DERIVED_FROM                 14
#define HTTP_QUERY_COST                         15
#define HTTP_QUERY_LINK                         16
#define HTTP_QUERY_PRAGMA                       17
#define HTTP_QUERY_VERSION                      18  // special: part of status line
#define HTTP_QUERY_STATUS_CODE                  19  // special: part of status line
#define HTTP_QUERY_STATUS_TEXT                  20  // special: part of status line
#define HTTP_QUERY_RAW_HEADERS                  21  // special: all headers as ASCIIZ
#define HTTP_QUERY_RAW_HEADERS_CRLF             22  // special: all headers
#define HTTP_QUERY_CONNECTION                   23
#define HTTP_QUERY_ACCEPT                       24
#define HTTP_QUERY_ACCEPT_CHARSET               25
#define HTTP_QUERY_ACCEPT_ENCODING              26
#define HTTP_QUERY_ACCEPT_LANGUAGE              27
#define HTTP_QUERY_AUTHORIZATION                28
#define HTTP_QUERY_CONTENT_ENCODING             29
#define HTTP_QUERY_FORWARDED                    30
#define HTTP_QUERY_FROM                         31
#define HTTP_QUERY_IF_MODIFIED_SINCE            32
#define HTTP_QUERY_LOCATION                     33
#define HTTP_QUERY_ORIG_URI                     34
#define HTTP_QUERY_REFERER                      35
#define HTTP_QUERY_RETRY_AFTER                  36
#define HTTP_QUERY_SERVER                       37
#define HTTP_QUERY_TITLE                        38
#define HTTP_QUERY_USER_AGENT                   39
#define HTTP_QUERY_WWW_AUTHENTICATE             40
#define HTTP_QUERY_PROXY_AUTHENTICATE           41
#define HTTP_QUERY_ACCEPT_RANGES                42
#define HTTP_QUERY_SET_COOKIE                   43
#define HTTP_QUERY_COOKIE                       44
#define HTTP_QUERY_REQUEST_METHOD               45  // special: GET/POST etc.
#define HTTP_QUERY_REFRESH                      46
#define HTTP_QUERY_CONTENT_DISPOSITION          47

//
// HTTP 1.1 defined headers
//

#define HTTP_QUERY_AGE                          48
#define HTTP_QUERY_CACHE_CONTROL                49
#define HTTP_QUERY_CONTENT_BASE                 50
#define HTTP_QUERY_CONTENT_LOCATION             51
#define HTTP_QUERY_CONTENT_MD5                  52
#define HTTP_QUERY_CONTENT_RANGE                53
#define HTTP_QUERY_ETAG                         54
#define HTTP_QUERY_HOST                         55
#define HTTP_QUERY_IF_MATCH                     56
#define HTTP_QUERY_IF_NONE_MATCH                57
#define HTTP_QUERY_IF_RANGE                     58
#define HTTP_QUERY_IF_UNMODIFIED_SINCE          59
#define HTTP_QUERY_MAX_FORWARDS                 60
#define HTTP_QUERY_PROXY_AUTHORIZATION          61
#define HTTP_QUERY_RANGE                        62
#define HTTP_QUERY_TRANSFER_ENCODING            63
#define HTTP_QUERY_UPGRADE                      64
#define HTTP_QUERY_VARY                         65
#define HTTP_QUERY_VIA                          66
#define HTTP_QUERY_WARNING                      67
#define HTTP_QUERY_EXPECT                       68
#define HTTP_QUERY_PROXY_CONNECTION             69
#define HTTP_QUERY_UNLESS_MODIFIED_SINCE        70



#define HTTP_QUERY_ECHO_REQUEST                 71
#define HTTP_QUERY_ECHO_REPLY                   72

// These are the set of headers that should be added back to a request when
// re-doing a request after a RETRY_WITH response.
#define HTTP_QUERY_ECHO_HEADERS                 73
#define HTTP_QUERY_ECHO_HEADERS_CRLF            74

#define HTTP_QUERY_PROXY_SUPPORT                75
#define HTTP_QUERY_AUTHENTICATION_INFO          76
#define HTTP_QUERY_PASSPORT_URLS                77
#define HTTP_QUERY_PASSPORT_CONFIG              78

#define HTTP_QUERY_X_CONTENT_TYPE_OPTIONS       79
#define HTTP_QUERY_P3P                          80
#define HTTP_QUERY_X_P2P_PEERDIST               81
#define HTTP_QUERY_TRANSLATE                    82
#define HTTP_QUERY_X_UA_COMPATIBLE              83
#define HTTP_QUERY_DEFAULT_STYLE                84
#define HTTP_QUERY_X_FRAME_OPTIONS              85
#define HTTP_QUERY_X_XSS_PROTECTION             86

#define HTTP_QUERY_SET_COOKIE2                  87

#define HTTP_QUERY_DO_NOT_TRACK                 88

#define HTTP_QUERY_KEEP_ALIVE                   89

#define HTTP_QUERY_HTTP2_SETTINGS               90

#define HTTP_QUERY_STRICT_TRANSPORT_SECURITY    91

#define HTTP_QUERY_TOKEN_BINDING                92

#define HTTP_QUERY_INCLUDE_REFERRED_TOKEN_BINDING_ID  93
#define HTTP_QUERY_INCLUDE_REFERER_TOKEN_BINDING_ID   HTTP_QUERY_INCLUDE_REFERRED_TOKEN_BINDING_ID

#define HTTP_QUERY_PUBLIC_KEY_PINS              94
#define HTTP_QUERY_PUBLIC_KEY_PINS_REPORT_ONLY  95

#define HTTP_QUERY_MAX                          95

//
// HTTP_QUERY_CUSTOM - if this special value is supplied as the dwInfoLevel
// parameter of HttpQueryInfo() then the lpBuffer parameter contains the name
// of the header we are to query
//

#define HTTP_QUERY_CUSTOM                       65535

//
// HTTP_QUERY_FLAG_REQUEST_HEADERS - if this bit is set in the dwInfoLevel
// parameter of HttpQueryInfo() then the request headers will be queried for the
// request information
//

#define HTTP_QUERY_FLAG_REQUEST_HEADERS         0x80000000

//
// HTTP_QUERY_FLAG_SYSTEMTIME - if this bit is set in the dwInfoLevel parameter
// of HttpQueryInfo() AND the header being queried contains date information,
// e.g. the "Expires:" header then lpBuffer will contain a SYSTEMTIME structure
// containing the date and time information converted from the header string
//

#define HTTP_QUERY_FLAG_SYSTEMTIME              0x40000000

//
// HTTP_QUERY_FLAG_NUMBER - if this bit is set in the dwInfoLevel parameter of
// HttpQueryInfo(), then the value of the header will be converted to a 32bit
// number before being returned to the caller, if applicable
//

#define HTTP_QUERY_FLAG_NUMBER                  0x20000000

//
// HTTP_QUERY_FLAG_COALESCE - combine the values from several headers of the
// same name into the output buffer
//

#define HTTP_QUERY_FLAG_COALESCE                0x10000000

//
// HTTP_QUERY_FLAG_NUMBER64 - if this bit is set in the dwInfoLevel parameter of
// HttpQueryInfo(), then the value of the header will be converted to a 64bit
// number before being returned to the caller, if applicable
//

#define HTTP_QUERY_FLAG_NUMBER64                0x08000000

//
// HTTP_QUERY_FLAG_COALESCE_WITH_COMMA - if this bit is set in the dwInfoLevel
// parameter of HttpQueryInfo(), then the values from several headers of the
// same name will be combined using comma as the delimiter
//

#define HTTP_QUERY_FLAG_COALESCE_WITH_COMMA     0x04000000

#define HTTP_QUERY_MODIFIER_FLAGS_MASK          (HTTP_QUERY_FLAG_REQUEST_HEADERS      \
                                                | HTTP_QUERY_FLAG_SYSTEMTIME          \
                                                | HTTP_QUERY_FLAG_NUMBER              \
                                                | HTTP_QUERY_FLAG_COALESCE            \
                                                | HTTP_QUERY_FLAG_NUMBER64            \
                                                | HTTP_QUERY_FLAG_COALESCE_WITH_COMMA \
                                                )

#define HTTP_QUERY_HEADER_MASK                  (~HTTP_QUERY_MODIFIER_FLAGS_MASK)

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
#define HTTP_STATUS_PARTIAL_CONTENT     206 // partial GET furfilled

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
#define HTTP_STATUS_MISDIRECTED_REQUEST 421 // misdirected request
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
// prototypes
//

INTERNETAPI_(HINTERNET) HttpOpenRequestA(
    _In_ HINTERNET hConnect,
    _In_opt_ LPCSTR lpszVerb,
    _In_opt_ LPCSTR lpszObjectName,
    _In_opt_ LPCSTR lpszVersion,
    _In_opt_ LPCSTR lpszReferrer,
    _In_opt_z_ LPCSTR FAR * lplpszAcceptTypes,
    _In_ DWORD dwFlags,
    _In_opt_ DWORD_PTR dwContext
    );
INTERNETAPI_(HINTERNET) HttpOpenRequestW(
    _In_ HINTERNET hConnect,
    _In_opt_ LPCWSTR lpszVerb,
    _In_opt_ LPCWSTR lpszObjectName,
    _In_opt_ LPCWSTR lpszVersion,
    _In_opt_ LPCWSTR lpszReferrer,
    _In_opt_z_ LPCWSTR FAR * lplpszAcceptTypes,
    _In_ DWORD dwFlags,
    _In_opt_ DWORD_PTR dwContext
    );
#ifdef UNICODE
#define HttpOpenRequest  HttpOpenRequestW
#else
#define HttpOpenRequest  HttpOpenRequestA
#endif // !UNICODE

BOOLAPI HttpAddRequestHeadersA(
    _In_ HINTERNET hRequest,
    _When_(dwHeadersLength == (DWORD)-1, _In_z_)
    _When_(dwHeadersLength != (DWORD)-1, _In_reads_(dwHeadersLength))
    LPCSTR lpszHeaders,
    _In_ DWORD dwHeadersLength,
    _In_ DWORD dwModifiers
    );
BOOLAPI HttpAddRequestHeadersW(
    _In_ HINTERNET hRequest,
    _When_(dwHeadersLength == (DWORD)-1, _In_z_)
    _When_(dwHeadersLength != (DWORD)-1, _In_reads_(dwHeadersLength))
    LPCWSTR lpszHeaders,
    _In_ DWORD dwHeadersLength,
    _In_ DWORD dwModifiers
    );
#ifdef UNICODE
#define HttpAddRequestHeaders  HttpAddRequestHeadersW
#else
#define HttpAddRequestHeaders  HttpAddRequestHeadersA
#endif // !UNICODE

//
// values for dwModifiers parameter of HttpAddRequestHeaders()
//

#define HTTP_ADDREQ_INDEX_MASK      0x0000FFFF
#define HTTP_ADDREQ_FLAGS_MASK      0xFFFF0000

//
// HTTP_ADDREQ_FLAG_ADD_IF_NEW - the header will only be added if it doesn't
// already exist
//

#define HTTP_ADDREQ_FLAG_ADD_IF_NEW 0x10000000

//
// HTTP_ADDREQ_FLAG_ADD - if HTTP_ADDREQ_FLAG_REPLACE is set but the header is
// not found then if this flag is set, the header is added anyway, so long as
// there is a valid header-value
//

#define HTTP_ADDREQ_FLAG_ADD        0x20000000

//
// HTTP_ADDREQ_FLAG_COALESCE - coalesce headers with same name. e.g.
// "Accept: text/*" and "Accept: audio/*" with this flag results in a single
// header: "Accept: text/*, audio/*"
//

#define HTTP_ADDREQ_FLAG_COALESCE_WITH_COMMA       0x40000000
#define HTTP_ADDREQ_FLAG_COALESCE_WITH_SEMICOLON   0x01000000
#define HTTP_ADDREQ_FLAG_COALESCE                  HTTP_ADDREQ_FLAG_COALESCE_WITH_COMMA

//
// HTTP_ADDREQ_FLAG_REPLACE - replaces the specified header. Only one header can
// be supplied in the buffer. If the header to be replaced is not the first
// in a list of headers with the same name, then the relative index should be
// supplied in the low 8 bits of the dwModifiers parameter. If the header-value
// part is missing, then the header is removed
//

#define HTTP_ADDREQ_FLAG_REPLACE    0x80000000


BOOLAPI HttpSendRequestA(
    _In_ HINTERNET hRequest,
    _In_reads_opt_(dwHeadersLength) LPCSTR lpszHeaders,
    _In_ DWORD dwHeadersLength,
    _In_reads_bytes_opt_(dwOptionalLength) LPVOID lpOptional,
    _In_ DWORD dwOptionalLength
    );
BOOLAPI HttpSendRequestW(
    _In_ HINTERNET hRequest,
    _In_reads_opt_(dwHeadersLength) LPCWSTR lpszHeaders,
    _In_ DWORD dwHeadersLength,
    _In_reads_bytes_opt_(dwOptionalLength) LPVOID lpOptional,
    _In_ DWORD dwOptionalLength
    );
#ifdef UNICODE
#define HttpSendRequest  HttpSendRequestW
#else
#define HttpSendRequest  HttpSendRequestA
#endif // !UNICODE

BOOLAPI HttpSendRequestExA(
    _In_ HINTERNET hRequest,
    _In_opt_ LPINTERNET_BUFFERSA lpBuffersIn,
    _Out_opt_ LPINTERNET_BUFFERSA lpBuffersOut,
    _In_ DWORD dwFlags,
    _In_opt_ DWORD_PTR dwContext
    );
BOOLAPI HttpSendRequestExW(
    _In_ HINTERNET hRequest,
    _In_opt_ LPINTERNET_BUFFERSW lpBuffersIn,
    _Out_opt_ LPINTERNET_BUFFERSW lpBuffersOut,
    _In_ DWORD dwFlags,
    _In_opt_ DWORD_PTR dwContext
    );
#ifdef UNICODE
#define HttpSendRequestEx  HttpSendRequestExW
#else
#define HttpSendRequestEx  HttpSendRequestExA
#endif // !UNICODE

//
// flags for HttpSendRequestEx(), HttpEndRequest()
//

#define HSR_ASYNC       WININET_API_FLAG_ASYNC          // force async
#define HSR_SYNC        WININET_API_FLAG_SYNC           // force sync
#define HSR_USE_CONTEXT WININET_API_FLAG_USE_CONTEXT    // use dwContext value
#define HSR_INITIATE    0x00000008                      // iterative operation (completed by HttpEndRequest)
#define HSR_DOWNLOAD    0x00000010                      // download to file
#define HSR_CHUNKED     0x00000020                      // operation is send of chunked data

BOOLAPI HttpEndRequestA(
    _In_ HINTERNET hRequest,
    _Out_opt_ LPINTERNET_BUFFERSA lpBuffersOut,
    _In_ DWORD dwFlags,
    _In_opt_ DWORD_PTR dwContext
    );
BOOLAPI HttpEndRequestW(
    _In_ HINTERNET hRequest,
    _Out_opt_ LPINTERNET_BUFFERSW lpBuffersOut,
    _In_ DWORD dwFlags,
    _In_opt_ DWORD_PTR dwContext
    );
#ifdef UNICODE
#define HttpEndRequest  HttpEndRequestW
#else
#define HttpEndRequest  HttpEndRequestA
#endif // !UNICODE

BOOLAPI
HttpQueryInfoA(
    _In_ HINTERNET hRequest,
    _In_ DWORD dwInfoLevel,
    _Inout_updates_bytes_to_opt_(*lpdwBufferLength, *lpdwBufferLength) __out_data_source(NETWORK) LPVOID lpBuffer,
    _Inout_ LPDWORD lpdwBufferLength,
    _Inout_opt_ LPDWORD lpdwIndex
    );
BOOLAPI
HttpQueryInfoW(
    _In_ HINTERNET hRequest,
    _In_ DWORD dwInfoLevel,
    _Inout_updates_bytes_to_opt_(*lpdwBufferLength, *lpdwBufferLength) __out_data_source(NETWORK) LPVOID lpBuffer,
    _Inout_ LPDWORD lpdwBufferLength,
    _Inout_opt_ LPDWORD lpdwIndex
    );
#ifdef UNICODE
#define HttpQueryInfo  HttpQueryInfoW
#else
#define HttpQueryInfo  HttpQueryInfoA
#endif // !UNICODE


//
// Cookie APIs
//


typedef struct
{
    PWSTR pwszName;
    PWSTR pwszValue;
    PWSTR pwszDomain;
    PWSTR pwszPath;
    DWORD dwFlags;
    FILETIME ftExpires;
    BOOL fExpiresSet;
} INTERNET_COOKIE2;

#define INTERNET_COOKIE_IS_SECURE                  0x00000001
#define INTERNET_COOKIE_IS_SESSION                 0x00000002

#define INTERNET_COOKIE_THIRD_PARTY                0x00000010
#define INTERNET_COOKIE_PROMPT_REQUIRED            0x00000020
#define INTERNET_COOKIE_EVALUATE_P3P               0x00000040
#define INTERNET_COOKIE_APPLY_P3P                  0x00000080

#define INTERNET_COOKIE_P3P_ENABLED                0x00000100
#define INTERNET_COOKIE_IS_RESTRICTED              0x00000200
#define INTERNET_COOKIE_IE6                        0x00000400
#define INTERNET_COOKIE_IS_LEGACY                  0x00000800

#define INTERNET_COOKIE_NON_SCRIPT                 0x00001000
#define INTERNET_COOKIE_HTTPONLY                   0x00002000
#define INTERNET_COOKIE_HOST_ONLY                  0x00004000
#define INTERNET_COOKIE_APPLY_HOST_ONLY            0x00008000


#define INTERNET_COOKIE_HOST_ONLY_APPLIED          0x00080000
#define INTERNET_COOKIE_SAME_SITE_STRICT           0x00100000
#define INTERNET_COOKIE_SAME_SITE_LAX              0x00200000
#define INTERNET_COOKIE_SAME_SITE_LEVEL_CROSS_SITE 0x00400000


BOOLAPI
InternetSetCookieA(
    _In_ LPCSTR lpszUrl,
    _In_opt_ LPCSTR lpszCookieName,
    _In_ LPCSTR lpszCookieData
    );
BOOLAPI
InternetSetCookieW(
    _In_ LPCWSTR lpszUrl,
    _In_opt_ LPCWSTR lpszCookieName,
    _In_ LPCWSTR lpszCookieData
    );
#ifdef UNICODE
#define InternetSetCookie  InternetSetCookieW
#else
#define InternetSetCookie  InternetSetCookieA
#endif // !UNICODE

BOOLAPI
InternetGetCookieA(
    _In_ LPCSTR lpszUrl,
    _In_opt_ LPCSTR lpszCookieName,
    _Out_writes_opt_(*lpdwSize) LPSTR lpszCookieData,
    _Inout_ LPDWORD lpdwSize
    );
BOOLAPI
InternetGetCookieW(
    _In_ LPCWSTR lpszUrl,
    _In_opt_ LPCWSTR lpszCookieName,
    _Out_writes_opt_(*lpdwSize) LPWSTR lpszCookieData,
    _Inout_ LPDWORD lpdwSize
    );
#ifdef UNICODE
#define InternetGetCookie  InternetGetCookieW
#else
#define InternetGetCookie  InternetGetCookieA
#endif // !UNICODE

INTERNETAPI_(DWORD)
InternetSetCookieExA(
    _In_ LPCSTR lpszUrl,
    _In_opt_ LPCSTR lpszCookieName,
    _In_ LPCSTR lpszCookieData,
    _In_ DWORD dwFlags,
    _In_opt_ DWORD_PTR dwReserved
    );
INTERNETAPI_(DWORD)
InternetSetCookieExW(
    _In_ LPCWSTR lpszUrl,
    _In_opt_ LPCWSTR lpszCookieName,
    _In_ LPCWSTR lpszCookieData,
    _In_ DWORD dwFlags,
    _In_opt_ DWORD_PTR dwReserved
    );
#ifdef UNICODE
#define InternetSetCookieEx  InternetSetCookieExW
#else
#define InternetSetCookieEx  InternetSetCookieExA
#endif // !UNICODE

BOOLAPI
InternetGetCookieExA(
    _In_ LPCSTR lpszUrl,
    _In_opt_ LPCSTR lpszCookieName,
    _In_reads_opt_(*lpdwSize) LPSTR lpszCookieData,
    _Inout_ LPDWORD lpdwSize,
    _In_ DWORD dwFlags,
    _Reserved_ LPVOID lpReserved
    );
BOOLAPI
InternetGetCookieExW(
    _In_ LPCWSTR lpszUrl,
    _In_opt_ LPCWSTR lpszCookieName,
    _In_reads_opt_(*lpdwSize) LPWSTR lpszCookieData,
    _Inout_ LPDWORD lpdwSize,
    _In_ DWORD dwFlags,
    _Reserved_ LPVOID lpReserved
    );
#ifdef UNICODE
#define InternetGetCookieEx  InternetGetCookieExW
#else
#define InternetGetCookieEx  InternetGetCookieExA
#endif // !UNICODE

INTERNETAPI_(VOID)
InternetFreeCookies(
    _Inout_opt_ INTERNET_COOKIE2 *pCookies,
    _In_ DWORD dwCookieCount
);

INTERNETAPI_(DWORD)
InternetGetCookieEx2(
    _In_ PCWSTR pcwszUrl,
    _In_opt_ PCWSTR pcwszCookieName,
    _In_ DWORD dwFlags,
    _Outptr_ INTERNET_COOKIE2 **ppCookies,
    _Out_ PDWORD pdwCookieCount
);

INTERNETAPI_(DWORD)
InternetSetCookieEx2(
    _In_ PCWSTR pcwszUrl,
    _In_ const INTERNET_COOKIE2 *pCookie,
    _In_opt_ PCWSTR pcwszP3PPolicy,
    _In_ DWORD dwFlags,
    _Out_ PDWORD pdwCookieState
);

//
// offline browsing
//

INTERNETAPI_(DWORD)
InternetAttemptConnect(
    _In_ DWORD dwReserved
    );

BOOLAPI
InternetCheckConnectionA(
    _In_ LPCSTR lpszUrl,
    _In_ DWORD dwFlags,
    _In_ DWORD dwReserved
    );
BOOLAPI
InternetCheckConnectionW(
    _In_ LPCWSTR lpszUrl,
    _In_ DWORD dwFlags,
    _In_ DWORD dwReserved
    );
#ifdef UNICODE
#define InternetCheckConnection  InternetCheckConnectionW
#else
#define InternetCheckConnection  InternetCheckConnectionA
#endif // !UNICODE


#define FLAG_ICC_FORCE_CONNECTION       0x00000001

//
// Internet UI
//

//
// InternetErrorDlg - Provides UI for certain Errors.
//

#define FLAGS_ERROR_UI_FILTER_FOR_ERRORS        0x01
#define FLAGS_ERROR_UI_FLAGS_CHANGE_OPTIONS     0x02
#define FLAGS_ERROR_UI_FLAGS_GENERATE_DATA      0x04
#define FLAGS_ERROR_UI_FLAGS_NO_UI              0x08
#define FLAGS_ERROR_UI_SERIALIZE_DIALOGS        0x10

//
// If SERIALIZE_DIALOGS flag set, client should implement thread-safe non-blocking callback...
//

DWORD InternetAuthNotifyCallback
(
    DWORD_PTR       dwContext,    // as passed to InternetErrorDlg
    DWORD           dwReturn,     // error code: success, resend, or cancel
    LPVOID          lpReserved    // reserved: will be set to null
);
typedef DWORD (CALLBACK * PFN_AUTH_NOTIFY) (DWORD_PTR, DWORD, LPVOID);

//
// ... and last parameter of InternetErrorDlg should point to...
//

typedef struct
{
    DWORD            cbStruct;    // size of this structure
    DWORD            dwOptions;   // reserved: must set to 0
    PFN_AUTH_NOTIFY  pfnNotify;   // notification callback to retry InternetErrorDlg
    DWORD_PTR        dwContext;   // context to pass to to notification function
}
    INTERNET_AUTH_NOTIFY_DATA;


BOOLAPI ResumeSuspendedDownload(
    _In_ HINTERNET hRequest,
    _In_ DWORD dwResultCode
    );

INTERNETAPI_(DWORD) InternetErrorDlg(
    _In_ HWND hWnd,
    _Inout_opt_ HINTERNET hRequest,
    _In_ DWORD dwError,
    _In_ DWORD dwFlags,
    _Inout_opt_ LPVOID * lppvData
    );

INTERNETAPI_(DWORD) InternetConfirmZoneCrossingA(
    _In_ HWND hWnd,
    _In_ LPSTR szUrlPrev,
    _In_ LPSTR szUrlNew,
    _In_ BOOL bPost
    );

INTERNETAPI_(DWORD) InternetConfirmZoneCrossingW(
    _In_ HWND hWnd,
    _In_ LPWSTR szUrlPrev,
    _In_ LPWSTR szUrlNew,
    _In_ BOOL bPost
    );

#ifdef UNICODE
#define InternetConfirmZoneCrossing  InternetConfirmZoneCrossingW
#else
#ifdef _WINX32_
#define InternetConfirmZoneCrossing  InternetConfirmZoneCrossingA
#else
INTERNETAPI_(DWORD) InternetConfirmZoneCrossing(
    _In_ HWND hWnd,
    _In_ LPSTR szUrlPrev,
    _In_ LPSTR szUrlNew,
    _In_ BOOL bPost
    );
#endif // _WINX32_
#endif // !UNICODE

//#if !defined(_WINERROR_)

//
// Internet API error returns
//

#define INTERNET_ERROR_BASE                     12000

#define ERROR_INTERNET_OUT_OF_HANDLES           (INTERNET_ERROR_BASE + 1)
#define ERROR_INTERNET_TIMEOUT                  (INTERNET_ERROR_BASE + 2)
#define ERROR_INTERNET_EXTENDED_ERROR           (INTERNET_ERROR_BASE + 3)
#define ERROR_INTERNET_INTERNAL_ERROR           (INTERNET_ERROR_BASE + 4)
#define ERROR_INTERNET_INVALID_URL              (INTERNET_ERROR_BASE + 5)
#define ERROR_INTERNET_UNRECOGNIZED_SCHEME      (INTERNET_ERROR_BASE + 6)
#define ERROR_INTERNET_NAME_NOT_RESOLVED        (INTERNET_ERROR_BASE + 7)
#define ERROR_INTERNET_PROTOCOL_NOT_FOUND       (INTERNET_ERROR_BASE + 8)
#define ERROR_INTERNET_INVALID_OPTION           (INTERNET_ERROR_BASE + 9)
#define ERROR_INTERNET_BAD_OPTION_LENGTH        (INTERNET_ERROR_BASE + 10)
#define ERROR_INTERNET_OPTION_NOT_SETTABLE      (INTERNET_ERROR_BASE + 11)
#define ERROR_INTERNET_SHUTDOWN                 (INTERNET_ERROR_BASE + 12)
#define ERROR_INTERNET_INCORRECT_USER_NAME      (INTERNET_ERROR_BASE + 13)
#define ERROR_INTERNET_INCORRECT_PASSWORD       (INTERNET_ERROR_BASE + 14)
#define ERROR_INTERNET_LOGIN_FAILURE            (INTERNET_ERROR_BASE + 15)
#define ERROR_INTERNET_INVALID_OPERATION        (INTERNET_ERROR_BASE + 16)
#define ERROR_INTERNET_OPERATION_CANCELLED      (INTERNET_ERROR_BASE + 17)
#define ERROR_INTERNET_INCORRECT_HANDLE_TYPE    (INTERNET_ERROR_BASE + 18)
#define ERROR_INTERNET_INCORRECT_HANDLE_STATE   (INTERNET_ERROR_BASE + 19)
#define ERROR_INTERNET_NOT_PROXY_REQUEST        (INTERNET_ERROR_BASE + 20)
#define ERROR_INTERNET_REGISTRY_VALUE_NOT_FOUND (INTERNET_ERROR_BASE + 21)
#define ERROR_INTERNET_BAD_REGISTRY_PARAMETER   (INTERNET_ERROR_BASE + 22)
#define ERROR_INTERNET_NO_DIRECT_ACCESS         (INTERNET_ERROR_BASE + 23)
#define ERROR_INTERNET_NO_CONTEXT               (INTERNET_ERROR_BASE + 24)
#define ERROR_INTERNET_NO_CALLBACK              (INTERNET_ERROR_BASE + 25)
#define ERROR_INTERNET_REQUEST_PENDING          (INTERNET_ERROR_BASE + 26)
#define ERROR_INTERNET_INCORRECT_FORMAT         (INTERNET_ERROR_BASE + 27)
#define ERROR_INTERNET_ITEM_NOT_FOUND           (INTERNET_ERROR_BASE + 28)
#define ERROR_INTERNET_CANNOT_CONNECT           (INTERNET_ERROR_BASE + 29)
#define ERROR_INTERNET_CONNECTION_ABORTED       (INTERNET_ERROR_BASE + 30)
#define ERROR_INTERNET_CONNECTION_RESET         (INTERNET_ERROR_BASE + 31)
#define ERROR_INTERNET_FORCE_RETRY              (INTERNET_ERROR_BASE + 32)
#define ERROR_INTERNET_INVALID_PROXY_REQUEST    (INTERNET_ERROR_BASE + 33)
#define ERROR_INTERNET_NEED_UI                  (INTERNET_ERROR_BASE + 34)

#define ERROR_INTERNET_HANDLE_EXISTS            (INTERNET_ERROR_BASE + 36)
#define ERROR_INTERNET_SEC_CERT_DATE_INVALID    (INTERNET_ERROR_BASE + 37)
#define ERROR_INTERNET_SEC_CERT_CN_INVALID      (INTERNET_ERROR_BASE + 38)
#define ERROR_INTERNET_HTTP_TO_HTTPS_ON_REDIR   (INTERNET_ERROR_BASE + 39)
#define ERROR_INTERNET_HTTPS_TO_HTTP_ON_REDIR   (INTERNET_ERROR_BASE + 40)
#define ERROR_INTERNET_MIXED_SECURITY           (INTERNET_ERROR_BASE + 41)
#define ERROR_INTERNET_CHG_POST_IS_NON_SECURE   (INTERNET_ERROR_BASE + 42)
#define ERROR_INTERNET_POST_IS_NON_SECURE       (INTERNET_ERROR_BASE + 43)
#define ERROR_INTERNET_CLIENT_AUTH_CERT_NEEDED  (INTERNET_ERROR_BASE + 44)
#define ERROR_INTERNET_INVALID_CA               (INTERNET_ERROR_BASE + 45)
#define ERROR_INTERNET_CLIENT_AUTH_NOT_SETUP    (INTERNET_ERROR_BASE + 46)
#define ERROR_INTERNET_ASYNC_THREAD_FAILED      (INTERNET_ERROR_BASE + 47)
#define ERROR_INTERNET_REDIRECT_SCHEME_CHANGE   (INTERNET_ERROR_BASE + 48)
#define ERROR_INTERNET_DIALOG_PENDING           (INTERNET_ERROR_BASE + 49)
#define ERROR_INTERNET_RETRY_DIALOG             (INTERNET_ERROR_BASE + 50)
#define ERROR_INTERNET_HTTPS_HTTP_SUBMIT_REDIR  (INTERNET_ERROR_BASE + 52)
#define ERROR_INTERNET_INSERT_CDROM             (INTERNET_ERROR_BASE + 53)
#define ERROR_INTERNET_FORTEZZA_LOGIN_NEEDED    (INTERNET_ERROR_BASE + 54)
#define ERROR_INTERNET_SEC_CERT_ERRORS          (INTERNET_ERROR_BASE + 55)
#define ERROR_INTERNET_SEC_CERT_NO_REV          (INTERNET_ERROR_BASE + 56)
#define ERROR_INTERNET_SEC_CERT_REV_FAILED      (INTERNET_ERROR_BASE + 57)


#define ERROR_HTTP_HSTS_REDIRECT_REQUIRED       (INTERNET_ERROR_BASE + 60)


#define ERROR_INTERNET_SEC_CERT_WEAK_SIGNATURE  (INTERNET_ERROR_BASE + 62)


//
// FTP API errors
//

#define ERROR_FTP_TRANSFER_IN_PROGRESS          (INTERNET_ERROR_BASE + 110)
#define ERROR_FTP_DROPPED                       (INTERNET_ERROR_BASE + 111)
#define ERROR_FTP_NO_PASSIVE_MODE               (INTERNET_ERROR_BASE + 112)

//
// gopher API errors
//

#define ERROR_GOPHER_PROTOCOL_ERROR             (INTERNET_ERROR_BASE + 130)
#define ERROR_GOPHER_NOT_FILE                   (INTERNET_ERROR_BASE + 131)
#define ERROR_GOPHER_DATA_ERROR                 (INTERNET_ERROR_BASE + 132)
#define ERROR_GOPHER_END_OF_DATA                (INTERNET_ERROR_BASE + 133)
#define ERROR_GOPHER_INVALID_LOCATOR            (INTERNET_ERROR_BASE + 134)
#define ERROR_GOPHER_INCORRECT_LOCATOR_TYPE     (INTERNET_ERROR_BASE + 135)
#define ERROR_GOPHER_NOT_GOPHER_PLUS            (INTERNET_ERROR_BASE + 136)
#define ERROR_GOPHER_ATTRIBUTE_NOT_FOUND        (INTERNET_ERROR_BASE + 137)
#define ERROR_GOPHER_UNKNOWN_LOCATOR            (INTERNET_ERROR_BASE + 138)

//
// HTTP API errors
//

#define ERROR_HTTP_HEADER_NOT_FOUND             (INTERNET_ERROR_BASE + 150)
#define ERROR_HTTP_DOWNLEVEL_SERVER             (INTERNET_ERROR_BASE + 151)
#define ERROR_HTTP_INVALID_SERVER_RESPONSE      (INTERNET_ERROR_BASE + 152)
#define ERROR_HTTP_INVALID_HEADER               (INTERNET_ERROR_BASE + 153)
#define ERROR_HTTP_INVALID_QUERY_REQUEST        (INTERNET_ERROR_BASE + 154)
#define ERROR_HTTP_HEADER_ALREADY_EXISTS        (INTERNET_ERROR_BASE + 155)
#define ERROR_HTTP_REDIRECT_FAILED              (INTERNET_ERROR_BASE + 156)
#define ERROR_HTTP_NOT_REDIRECTED               (INTERNET_ERROR_BASE + 160)
#define ERROR_HTTP_COOKIE_NEEDS_CONFIRMATION    (INTERNET_ERROR_BASE + 161)
#define ERROR_HTTP_COOKIE_DECLINED              (INTERNET_ERROR_BASE + 162)
#define ERROR_HTTP_REDIRECT_NEEDS_CONFIRMATION  (INTERNET_ERROR_BASE + 168)

//
// additional Internet API error codes
//

#define ERROR_INTERNET_SECURITY_CHANNEL_ERROR   (INTERNET_ERROR_BASE + 157)
#define ERROR_INTERNET_UNABLE_TO_CACHE_FILE     (INTERNET_ERROR_BASE + 158)
#define ERROR_INTERNET_TCPIP_NOT_INSTALLED      (INTERNET_ERROR_BASE + 159)
#define ERROR_INTERNET_DISCONNECTED             (INTERNET_ERROR_BASE + 163)
#define ERROR_INTERNET_SERVER_UNREACHABLE       (INTERNET_ERROR_BASE + 164)
#define ERROR_INTERNET_PROXY_SERVER_UNREACHABLE (INTERNET_ERROR_BASE + 165)

#define ERROR_INTERNET_BAD_AUTO_PROXY_SCRIPT    (INTERNET_ERROR_BASE + 166)
#define ERROR_INTERNET_UNABLE_TO_DOWNLOAD_SCRIPT (INTERNET_ERROR_BASE + 167)
#define ERROR_INTERNET_SEC_INVALID_CERT         (INTERNET_ERROR_BASE + 169)
#define ERROR_INTERNET_SEC_CERT_REVOKED         (INTERNET_ERROR_BASE + 170)

// InternetAutodial specific errors

#define ERROR_INTERNET_FAILED_DUETOSECURITYCHECK  (INTERNET_ERROR_BASE + 171)
#define ERROR_INTERNET_NOT_INITIALIZED          (INTERNET_ERROR_BASE + 172)
#define ERROR_INTERNET_NEED_MSN_SSPI_PKG          (INTERNET_ERROR_BASE + 173)
#define ERROR_INTERNET_LOGIN_FAILURE_DISPLAY_ENTITY_BODY   (INTERNET_ERROR_BASE + 174)

// Decoding/Decompression specific errors

#define ERROR_INTERNET_DECODING_FAILED          (INTERNET_ERROR_BASE + 175)

#define ERROR_INTERNET_CLIENT_AUTH_CERT_NEEDED_PROXY (INTERNET_ERROR_BASE + 187)
#define ERROR_INTERNET_SECURE_FAILURE_PROXY          (INTERNET_ERROR_BASE + 188)
#define ERROR_INTERNET_HTTP_PROTOCOL_MISMATCH   (INTERNET_ERROR_BASE + 190)
#define ERROR_INTERNET_GLOBAL_CALLBACK_FAILED   (INTERNET_ERROR_BASE + 191)
#define ERROR_INTERNET_FEATURE_DISABLED         (INTERNET_ERROR_BASE + 192)

#define INTERNET_ERROR_LAST                     (INTERNET_ERROR_BASE + 192)


//#endif // !defined(_WINERROR_)

//
// URLCACHE APIs
//

//
// datatype definitions.
//

//
// cache entry type flags.
//

#define NORMAL_CACHE_ENTRY              0x00000001
#define STICKY_CACHE_ENTRY              0x00000004
#define EDITED_CACHE_ENTRY              0x00000008
#define TRACK_OFFLINE_CACHE_ENTRY       0x00000010
#define TRACK_ONLINE_CACHE_ENTRY        0x00000020
#define SPARSE_CACHE_ENTRY              0x00010000
#define COOKIE_CACHE_ENTRY              0x00100000
#define URLHISTORY_CACHE_ENTRY          0x00200000


#define URLCACHE_FIND_DEFAULT_FILTER    NORMAL_CACHE_ENTRY             \
                                    |   COOKIE_CACHE_ENTRY             \
                                    |   URLHISTORY_CACHE_ENTRY         \
                                    |   TRACK_OFFLINE_CACHE_ENTRY      \
                                    |   TRACK_ONLINE_CACHE_ENTRY       \
                                    |   STICKY_CACHE_ENTRY



//
// INTERNET_CACHE_ENTRY_INFO -
//

#if _MSC_VER >= 1200
#pragma warning(push)
#endif
#pragma warning(disable:4201)

typedef struct _INTERNET_CACHE_ENTRY_INFOA {
    DWORD dwStructSize;         // version of cache system.
    LPSTR   lpszSourceUrlName;    // embedded pointer to the URL name string.
    LPSTR   lpszLocalFileName;  // embedded pointer to the local file name.
    DWORD CacheEntryType;       // cache type bit mask.
    DWORD dwUseCount;           // current users count of the cache entry.
    DWORD dwHitRate;            // num of times the cache entry was retrieved.
    DWORD dwSizeLow;            // low DWORD of the file size.
    DWORD dwSizeHigh;           // high DWORD of the file size.
    FILETIME LastModifiedTime;  // last modified time of the file in GMT format.
    FILETIME ExpireTime;        // expire time of the file in GMT format
    FILETIME LastAccessTime;    // last accessed time in GMT format
    FILETIME LastSyncTime;      // last time the URL was synchronized
                                // with the source
    LPSTR   lpHeaderInfo;        // embedded pointer to the header info.
    DWORD dwHeaderInfoSize;     // size of the above header.
    LPSTR   lpszFileExtension;  // File extension used to retrive the urldata as a file.
        union {                     // Exemption delta from last access time.
                DWORD dwReserved;
                DWORD dwExemptDelta;
    };                          // Exemption delta from last access
} INTERNET_CACHE_ENTRY_INFOA, * LPINTERNET_CACHE_ENTRY_INFOA;
typedef struct _INTERNET_CACHE_ENTRY_INFOW {
    DWORD dwStructSize;         // version of cache system.
    LPWSTR  lpszSourceUrlName;    // embedded pointer to the URL name string.
    LPWSTR  lpszLocalFileName;  // embedded pointer to the local file name.
    DWORD CacheEntryType;       // cache type bit mask.
    DWORD dwUseCount;           // current users count of the cache entry.
    DWORD dwHitRate;            // num of times the cache entry was retrieved.
    DWORD dwSizeLow;            // low DWORD of the file size.
    DWORD dwSizeHigh;           // high DWORD of the file size.
    FILETIME LastModifiedTime;  // last modified time of the file in GMT format.
    FILETIME ExpireTime;        // expire time of the file in GMT format
    FILETIME LastAccessTime;    // last accessed time in GMT format
    FILETIME LastSyncTime;      // last time the URL was synchronized
                                // with the source
    LPWSTR  lpHeaderInfo;        // embedded pointer to the header info.
    DWORD dwHeaderInfoSize;     // size of the above header.
    LPWSTR  lpszFileExtension;  // File extension used to retrive the urldata as a file.
        union {                     // Exemption delta from last access time.
                DWORD dwReserved;
                DWORD dwExemptDelta;
    };                          // Exemption delta from last access
} INTERNET_CACHE_ENTRY_INFOW, * LPINTERNET_CACHE_ENTRY_INFOW;
#ifdef UNICODE
typedef INTERNET_CACHE_ENTRY_INFOW INTERNET_CACHE_ENTRY_INFO;
typedef LPINTERNET_CACHE_ENTRY_INFOW LPINTERNET_CACHE_ENTRY_INFO;
#else
typedef INTERNET_CACHE_ENTRY_INFOA INTERNET_CACHE_ENTRY_INFO;
typedef LPINTERNET_CACHE_ENTRY_INFOA LPINTERNET_CACHE_ENTRY_INFO;
#endif // UNICODE

#if _MSC_VER >= 1200
#pragma warning(pop)
#else
#pragma warning(default:4201)
#endif

typedef struct _INTERNET_CACHE_TIMESTAMPS {
    FILETIME    ftExpires;
    FILETIME    ftLastModified;
} INTERNET_CACHE_TIMESTAMPS, *LPINTERNET_CACHE_TIMESTAMPS;



//
// Cache Group
//
typedef LONGLONG GROUPID;


//
// Cache Group Flags
//
#define CACHEGROUP_ATTRIBUTE_GET_ALL        0xffffffff
#define CACHEGROUP_ATTRIBUTE_BASIC          0x00000001
#define CACHEGROUP_ATTRIBUTE_FLAG           0x00000002
#define CACHEGROUP_ATTRIBUTE_TYPE           0x00000004
#define CACHEGROUP_ATTRIBUTE_QUOTA          0x00000008
#define CACHEGROUP_ATTRIBUTE_GROUPNAME      0x00000010
#define CACHEGROUP_ATTRIBUTE_STORAGE        0x00000020

#define CACHEGROUP_FLAG_NONPURGEABLE        0x00000001
#define CACHEGROUP_FLAG_GIDONLY             0x00000004

#define CACHEGROUP_FLAG_FLUSHURL_ONDELETE   0x00000002


#define CACHEGROUP_SEARCH_ALL               0x00000000
#define CACHEGROUP_SEARCH_BYURL             0x00000001

#define CACHEGROUP_TYPE_INVALID             0x00000001


//
// updatable cache group fields
//
#define CACHEGROUP_READWRITE_MASK                   \
            CACHEGROUP_ATTRIBUTE_TYPE               \
        |   CACHEGROUP_ATTRIBUTE_QUOTA              \
        |   CACHEGROUP_ATTRIBUTE_GROUPNAME          \
        |   CACHEGROUP_ATTRIBUTE_STORAGE

//
// INTERNET_CACHE_GROUP_INFO
//

#define  GROUPNAME_MAX_LENGTH       120
#define  GROUP_OWNER_STORAGE_SIZE   4
typedef struct _INTERNET_CACHE_GROUP_INFOA {
    DWORD           dwGroupSize;
    DWORD           dwGroupFlags;
    DWORD           dwGroupType;
    DWORD           dwDiskUsage;    // in KB
    DWORD           dwDiskQuota;    // in KB
    DWORD           dwOwnerStorage[GROUP_OWNER_STORAGE_SIZE];
    CHAR            szGroupName[GROUPNAME_MAX_LENGTH];
} INTERNET_CACHE_GROUP_INFOA, * LPINTERNET_CACHE_GROUP_INFOA;
typedef struct _INTERNET_CACHE_GROUP_INFOW {
    DWORD           dwGroupSize;
    DWORD           dwGroupFlags;
    DWORD           dwGroupType;
    DWORD           dwDiskUsage;    // in KB
    DWORD           dwDiskQuota;    // in KB
    DWORD           dwOwnerStorage[GROUP_OWNER_STORAGE_SIZE];
    WCHAR           szGroupName[GROUPNAME_MAX_LENGTH];
} INTERNET_CACHE_GROUP_INFOW, * LPINTERNET_CACHE_GROUP_INFOW;
#ifdef UNICODE
typedef INTERNET_CACHE_GROUP_INFOW INTERNET_CACHE_GROUP_INFO;
typedef LPINTERNET_CACHE_GROUP_INFOW LPINTERNET_CACHE_GROUP_INFO;
#else
typedef INTERNET_CACHE_GROUP_INFOA INTERNET_CACHE_GROUP_INFO;
typedef LPINTERNET_CACHE_GROUP_INFOA LPINTERNET_CACHE_GROUP_INFO;
#endif // UNICODE



//
// Cache APIs
//

BOOLAPI CreateUrlCacheEntryA(
    _In_       LPCSTR lpszUrlName,
    _In_       DWORD dwExpectedFileSize,
    _In_opt_ LPCSTR lpszFileExtension,
    _Inout_updates_(MAX_PATH) LPSTR lpszFileName,
    _In_       DWORD dwReserved
    );
BOOLAPI CreateUrlCacheEntryW(
    _In_       LPCWSTR lpszUrlName,
    _In_       DWORD dwExpectedFileSize,
    _In_opt_ LPCWSTR lpszFileExtension,
    _Inout_updates_(MAX_PATH) LPWSTR lpszFileName,
    _In_       DWORD dwReserved
    );
#ifdef UNICODE
#define CreateUrlCacheEntry  CreateUrlCacheEntryW
#else
#define CreateUrlCacheEntry  CreateUrlCacheEntryA
#endif // !UNICODE

BOOLAPI
CommitUrlCacheEntryA(
    _In_ LPCSTR lpszUrlName,
    _In_opt_ LPCSTR lpszLocalFileName,
    _In_ FILETIME ExpireTime,
    _In_ FILETIME LastModifiedTime,
    _In_ DWORD CacheEntryType,
    _In_reads_opt_(cchHeaderInfo) LPBYTE lpHeaderInfo,
    _In_ DWORD cchHeaderInfo,
    _Reserved_ LPCSTR lpszFileExtension,
    _In_opt_ LPCSTR lpszOriginalUrl
    );

BOOLAPI
CommitUrlCacheEntryW(
    _In_ LPCWSTR lpszUrlName,
    _In_opt_ LPCWSTR lpszLocalFileName,
    _In_ FILETIME ExpireTime,
    _In_ FILETIME LastModifiedTime,
    _In_ DWORD CacheEntryType,
    _In_reads_opt_(cchHeaderInfo) LPWSTR lpszHeaderInfo,
    _In_ DWORD cchHeaderInfo,
    _Reserved_ LPCWSTR lpszFileExtension,
    _In_opt_ LPCWSTR lpszOriginalUrl
    );

#ifdef UNICODE
#define CommitUrlCacheEntry CommitUrlCacheEntryW
#else
#define CommitUrlCacheEntry CommitUrlCacheEntryA
#endif

BOOLAPI
RetrieveUrlCacheEntryFileA(
    _In_ LPCSTR lpszUrlName,
    _Inout_updates_bytes_opt_(*lpcbCacheEntryInfo) LPINTERNET_CACHE_ENTRY_INFOA lpCacheEntryInfo,
    _Inout_ LPDWORD lpcbCacheEntryInfo,
    _Reserved_ DWORD dwReserved
    );
BOOLAPI
RetrieveUrlCacheEntryFileW(
    _In_ LPCWSTR lpszUrlName,
    _Inout_updates_bytes_opt_(*lpcbCacheEntryInfo) LPINTERNET_CACHE_ENTRY_INFOW lpCacheEntryInfo,
    _Inout_ LPDWORD lpcbCacheEntryInfo,
    _Reserved_ DWORD dwReserved
    );
#ifdef UNICODE
#define RetrieveUrlCacheEntryFile  RetrieveUrlCacheEntryFileW
#else
#define RetrieveUrlCacheEntryFile  RetrieveUrlCacheEntryFileA
#endif // !UNICODE

BOOLAPI UnlockUrlCacheEntryFileA(
    _In_ LPCSTR lpszUrlName,
    _Reserved_ DWORD dwReserved
    );

BOOLAPI UnlockUrlCacheEntryFileW(
    _In_ LPCWSTR lpszUrlName,
    _Reserved_ DWORD dwReserved
    );



#ifdef UNICODE
#define UnlockUrlCacheEntryFile  UnlockUrlCacheEntryFileW
#else
#ifdef _WINX32_
#define UnlockUrlCacheEntryFile  UnlockUrlCacheEntryFileA
#else
BOOLAPI UnlockUrlCacheEntryFile(
    _In_ LPCSTR lpszUrlName,
    _Reserved_ DWORD dwReserved
    );
#endif // _WINX32_
#endif // !UNICODE

INTERNETAPI_(HANDLE)
RetrieveUrlCacheEntryStreamA(
    _In_ LPCSTR  lpszUrlName,
    _Inout_updates_bytes_opt_(*lpcbCacheEntryInfo) LPINTERNET_CACHE_ENTRY_INFOA lpCacheEntryInfo,
    _Inout_ LPDWORD lpcbCacheEntryInfo,
    _In_ BOOL fRandomRead,
    _Reserved_ DWORD dwReserved
    );
INTERNETAPI_(HANDLE)
RetrieveUrlCacheEntryStreamW(
    _In_ LPCWSTR  lpszUrlName,
    _Inout_updates_bytes_opt_(*lpcbCacheEntryInfo) LPINTERNET_CACHE_ENTRY_INFOW lpCacheEntryInfo,
    _Inout_ LPDWORD lpcbCacheEntryInfo,
    _In_ BOOL fRandomRead,
    _Reserved_ DWORD dwReserved
    );
#ifdef UNICODE
#define RetrieveUrlCacheEntryStream  RetrieveUrlCacheEntryStreamW
#else
#define RetrieveUrlCacheEntryStream  RetrieveUrlCacheEntryStreamA
#endif // !UNICODE

BOOLAPI
ReadUrlCacheEntryStream(
    _In_ HANDLE hUrlCacheStream,
    _In_ DWORD dwLocation,
    _Out_writes_bytes_(*lpdwLen) __out_data_source(NETWORK) LPVOID lpBuffer,
    _Inout_ LPDWORD lpdwLen,
    _Reserved_ DWORD Reserved
    );

BOOLAPI
ReadUrlCacheEntryStreamEx(
    _In_ HANDLE hUrlCacheStream,
    _In_ DWORDLONG qwLocation,
    _Out_writes_bytes_(*lpdwLen) __out_data_source(NETWORK) LPVOID lpBuffer,
    _Inout_ LPDWORD lpdwLen
    );

BOOLAPI
UnlockUrlCacheEntryStream(
    _In_ HANDLE hUrlCacheStream,
    _Reserved_ DWORD Reserved
    );


BOOLAPI
GetUrlCacheEntryInfoA(
    _In_ LPCSTR lpszUrlName,
    _Inout_updates_bytes_opt_(*lpcbCacheEntryInfo) LPINTERNET_CACHE_ENTRY_INFOA lpCacheEntryInfo,
    _Inout_opt_ LPDWORD lpcbCacheEntryInfo
    );
BOOLAPI
GetUrlCacheEntryInfoW(
    _In_ LPCWSTR lpszUrlName,
    _Inout_updates_bytes_opt_(*lpcbCacheEntryInfo) LPINTERNET_CACHE_ENTRY_INFOW lpCacheEntryInfo,
    _Inout_opt_ LPDWORD lpcbCacheEntryInfo
    );
#ifdef UNICODE
#define GetUrlCacheEntryInfo  GetUrlCacheEntryInfoW
#else
#define GetUrlCacheEntryInfo  GetUrlCacheEntryInfoA
#endif // !UNICODE


URLCACHEAPI_(HANDLE) FindFirstUrlCacheGroup(
    _In_        DWORD       dwFlags,
    _In_        DWORD       dwFilter,
    _Reserved_  LPVOID      lpSearchCondition,
    _Reserved_  DWORD       dwSearchCondition,
    _Out_       GROUPID*    lpGroupId,
    _Reserved_  LPVOID      lpReserved
    );

BOOLAPI FindNextUrlCacheGroup(
    _In_        HANDLE      hFind,
    _Out_       GROUPID*    lpGroupId,
    _Reserved_  LPVOID      lpReserved
    );


BOOLAPI GetUrlCacheGroupAttributeA(
    _In_                            GROUPID                         gid,
    _Reserved_                      DWORD                           dwFlags,
    _In_                            DWORD                           dwAttributes,
    _Out_writes_bytes_(*lpcbGroupInfo)    LPINTERNET_CACHE_GROUP_INFOA    lpGroupInfo,
    _Inout_                         LPDWORD                         lpcbGroupInfo,
    _Reserved_                      LPVOID                          lpReserved
    );
BOOLAPI GetUrlCacheGroupAttributeW(
    _In_                            GROUPID                         gid,
    _Reserved_                      DWORD                           dwFlags,
    _In_                            DWORD                           dwAttributes,
    _Out_writes_bytes_(*lpcbGroupInfo)    LPINTERNET_CACHE_GROUP_INFOW    lpGroupInfo,
    _Inout_                         LPDWORD                         lpcbGroupInfo,
    _Reserved_                      LPVOID                          lpReserved
    );
#ifdef UNICODE
#define GetUrlCacheGroupAttribute  GetUrlCacheGroupAttributeW
#else
#define GetUrlCacheGroupAttribute  GetUrlCacheGroupAttributeA
#endif // !UNICODE

BOOLAPI SetUrlCacheGroupAttributeA(
    _In_        GROUPID                         gid,
    _Reserved_  DWORD                           dwFlags,
    _In_        DWORD                           dwAttributes,
    _In_        LPINTERNET_CACHE_GROUP_INFOA    lpGroupInfo,
    _Reserved_  LPVOID                          lpReserved
    );
BOOLAPI SetUrlCacheGroupAttributeW(
    _In_        GROUPID                         gid,
    _Reserved_  DWORD                           dwFlags,
    _In_        DWORD                           dwAttributes,
    _In_        LPINTERNET_CACHE_GROUP_INFOW    lpGroupInfo,
    _Reserved_  LPVOID                          lpReserved
    );
#ifdef UNICODE
#define SetUrlCacheGroupAttribute  SetUrlCacheGroupAttributeW
#else
#define SetUrlCacheGroupAttribute  SetUrlCacheGroupAttributeA
#endif // !UNICODE


BOOLAPI
GetUrlCacheEntryInfoExA(
    _In_ LPCSTR lpszUrl,
    _Inout_updates_bytes_opt_(*lpcbCacheEntryInfo) LPINTERNET_CACHE_ENTRY_INFOA lpCacheEntryInfo,
    _Inout_opt_ LPDWORD lpcbCacheEntryInfo,
    _Reserved_ LPSTR lpszRedirectUrl,
    _Reserved_ LPDWORD lpcbRedirectUrl,
    _Reserved_ LPVOID lpReserved,
    _In_ DWORD dwFlags
    );
BOOLAPI
GetUrlCacheEntryInfoExW(
    _In_ LPCWSTR lpszUrl,
    _Inout_updates_bytes_opt_(*lpcbCacheEntryInfo) LPINTERNET_CACHE_ENTRY_INFOW lpCacheEntryInfo,
    _Inout_opt_ LPDWORD lpcbCacheEntryInfo,
    _Reserved_ LPWSTR lpszRedirectUrl,
    _Reserved_ LPDWORD lpcbRedirectUrl,
    _Reserved_ LPVOID lpReserved,
    _In_ DWORD dwFlags
    );
#ifdef UNICODE
#define GetUrlCacheEntryInfoEx  GetUrlCacheEntryInfoExW
#else
#define GetUrlCacheEntryInfoEx  GetUrlCacheEntryInfoExA
#endif // !UNICODE

#define CACHE_ENTRY_ATTRIBUTE_FC    0x00000004
#define CACHE_ENTRY_HITRATE_FC      0x00000010
#define CACHE_ENTRY_MODTIME_FC      0x00000040
#define CACHE_ENTRY_EXPTIME_FC      0x00000080
#define CACHE_ENTRY_ACCTIME_FC      0x00000100
#define CACHE_ENTRY_SYNCTIME_FC     0x00000200
#define CACHE_ENTRY_HEADERINFO_FC   0x00000400
#define CACHE_ENTRY_EXEMPT_DELTA_FC 0x00000800

BOOLAPI SetUrlCacheEntryInfoA(
    _In_ LPCSTR lpszUrlName,
    _In_ LPINTERNET_CACHE_ENTRY_INFOA lpCacheEntryInfo,
    _In_ DWORD dwFieldControl
    );
BOOLAPI SetUrlCacheEntryInfoW(
    _In_ LPCWSTR lpszUrlName,
    _In_ LPINTERNET_CACHE_ENTRY_INFOW lpCacheEntryInfo,
    _In_ DWORD dwFieldControl
    );
#ifdef UNICODE
#define SetUrlCacheEntryInfo  SetUrlCacheEntryInfoW
#else
#define SetUrlCacheEntryInfo  SetUrlCacheEntryInfoA
#endif // !UNICODE


//
// Cache Group Functions
//


INTERNETAPI_(GROUPID) CreateUrlCacheGroup(
    _In_   DWORD  dwFlags,
    _Reserved_ LPVOID lpReserved
    );

BOOLAPI DeleteUrlCacheGroup(
    _In_   GROUPID GroupId,
    _In_   DWORD   dwFlags,       // must pass 0
    _Reserved_ LPVOID  lpReserved
    );

// Flags for SetUrlCacheEntryGroup
#define INTERNET_CACHE_GROUP_ADD      0
#define INTERNET_CACHE_GROUP_REMOVE   1

BOOLAPI SetUrlCacheEntryGroupA(
    _In_   LPCSTR   lpszUrlName,
    _In_   DWORD    dwFlags,
    _In_   GROUPID  GroupId,
    _Reserved_ LPBYTE   pbGroupAttributes,
    _Reserved_ DWORD    cbGroupAttributes,
    _Reserved_ LPVOID   lpReserved
    );

BOOLAPI SetUrlCacheEntryGroupW(
    _In_   LPCWSTR  lpszUrlName,
    _In_   DWORD    dwFlags,
    _In_   GROUPID  GroupId,
    _Reserved_ LPBYTE   pbGroupAttributes,
    _Reserved_ DWORD    cbGroupAttributes,
    _Reserved_ LPVOID   lpReserved
    );

#ifdef UNICODE
#define SetUrlCacheEntryGroup  SetUrlCacheEntryGroupW
#else
#ifdef _WINX32_
#define SetUrlCacheEntryGroup  SetUrlCacheEntryGroupA
#else
BOOLAPI SetUrlCacheEntryGroup(
    _In_   LPCSTR   lpszUrlName,
    _In_   DWORD    dwFlags,
    _In_   GROUPID  GroupId,
    _Reserved_ LPBYTE   pbGroupAttributes,
    _Reserved_ DWORD    cbGroupAttributes,
    _Reserved_ LPVOID   lpReserved
    );
#endif // _WINX32_
#endif // !UNICODE

INTERNETAPI_(HANDLE)
FindFirstUrlCacheEntryExA(
    _In_opt_ LPCSTR  lpszUrlSearchPattern,
    _In_ DWORD dwFlags,
    _In_ DWORD  dwFilter,
    _In_ GROUPID  GroupId,
    _Inout_updates_bytes_opt_(*lpcbCacheEntryInfo) LPINTERNET_CACHE_ENTRY_INFOA lpFirstCacheEntryInfo,
    _Inout_ LPDWORD lpcbCacheEntryInfo,
    _Reserved_ LPVOID lpGroupAttributes,
    _Reserved_ LPDWORD lpcbGroupAttributes,
    _Reserved_ LPVOID lpReserved
    );
INTERNETAPI_(HANDLE)
FindFirstUrlCacheEntryExW(
    _In_opt_ LPCWSTR  lpszUrlSearchPattern,
    _In_ DWORD dwFlags,
    _In_ DWORD  dwFilter,
    _In_ GROUPID  GroupId,
    _Inout_updates_bytes_opt_(*lpcbCacheEntryInfo) LPINTERNET_CACHE_ENTRY_INFOW lpFirstCacheEntryInfo,
    _Inout_ LPDWORD lpcbCacheEntryInfo,
    _Reserved_ LPVOID lpGroupAttributes,
    _Reserved_ LPDWORD lpcbGroupAttributes,
    _Reserved_ LPVOID lpReserved
    );
#ifdef UNICODE
#define FindFirstUrlCacheEntryEx  FindFirstUrlCacheEntryExW
#else
#define FindFirstUrlCacheEntryEx  FindFirstUrlCacheEntryExA
#endif // !UNICODE

BOOLAPI
FindNextUrlCacheEntryExA(
    _In_ HANDLE hEnumHandle,
    _Inout_updates_bytes_opt_(*lpcbCacheEntryInfo) LPINTERNET_CACHE_ENTRY_INFOA lpNextCacheEntryInfo,
    _Inout_ LPDWORD lpcbCacheEntryInfo,
    _Reserved_ LPVOID lpGroupAttributes,
    _Reserved_ LPDWORD lpcbGroupAttributes,
    _Reserved_ LPVOID lpReserved
    );
BOOLAPI
FindNextUrlCacheEntryExW(
    _In_ HANDLE hEnumHandle,
    _Inout_updates_bytes_opt_(*lpcbCacheEntryInfo) LPINTERNET_CACHE_ENTRY_INFOW lpNextCacheEntryInfo,
    _Inout_ LPDWORD lpcbCacheEntryInfo,
    _Reserved_ LPVOID lpGroupAttributes,
    _Reserved_ LPDWORD lpcbGroupAttributes,
    _Reserved_ LPVOID lpReserved
    );
#ifdef UNICODE
#define FindNextUrlCacheEntryEx  FindNextUrlCacheEntryExW
#else
#define FindNextUrlCacheEntryEx  FindNextUrlCacheEntryExA
#endif // !UNICODE

INTERNETAPI_(HANDLE)
FindFirstUrlCacheEntryA(
    _In_opt_ LPCSTR lpszUrlSearchPattern,
    _Inout_updates_bytes_opt_(*lpcbCacheEntryInfo) LPINTERNET_CACHE_ENTRY_INFOA lpFirstCacheEntryInfo,
    _Inout_ LPDWORD lpcbCacheEntryInfo
    );
INTERNETAPI_(HANDLE)
FindFirstUrlCacheEntryW(
    _In_opt_ LPCWSTR lpszUrlSearchPattern,
    _Inout_updates_bytes_opt_(*lpcbCacheEntryInfo) LPINTERNET_CACHE_ENTRY_INFOW lpFirstCacheEntryInfo,
    _Inout_ LPDWORD lpcbCacheEntryInfo
    );
#ifdef UNICODE
#define FindFirstUrlCacheEntry  FindFirstUrlCacheEntryW
#else
#define FindFirstUrlCacheEntry  FindFirstUrlCacheEntryA
#endif // !UNICODE

BOOLAPI
FindNextUrlCacheEntryA(
    _In_ HANDLE hEnumHandle,
    _Inout_updates_bytes_opt_(*lpcbCacheEntryInfo) LPINTERNET_CACHE_ENTRY_INFOA lpNextCacheEntryInfo,
    _Inout_ LPDWORD lpcbCacheEntryInfo
    );
BOOLAPI
FindNextUrlCacheEntryW(
    _In_ HANDLE hEnumHandle,
    _Inout_updates_bytes_opt_(*lpcbCacheEntryInfo) LPINTERNET_CACHE_ENTRY_INFOW lpNextCacheEntryInfo,
    _Inout_ LPDWORD lpcbCacheEntryInfo
    );
#ifdef UNICODE
#define FindNextUrlCacheEntry  FindNextUrlCacheEntryW
#else
#define FindNextUrlCacheEntry  FindNextUrlCacheEntryA
#endif // !UNICODE


BOOLAPI FindCloseUrlCache(
    _In_ HANDLE hEnumHandle
    );

BOOLAPI DeleteUrlCacheEntryA(
    _In_ LPCSTR lpszUrlName
    );

BOOLAPI DeleteUrlCacheEntryW(
    _In_ LPCWSTR lpszUrlName
    );

#ifdef UNICODE
#define DeleteUrlCacheEntry  DeleteUrlCacheEntryW
#else
#ifdef _WINX32_
#define DeleteUrlCacheEntry  DeleteUrlCacheEntryA
#else
BOOLAPI DeleteUrlCacheEntry(
    _In_ LPCSTR lpszUrlName
    );
#endif // _WINX32_
#endif // !UNICODE



//
// Autodial APIs
//

INTERNETAPI_(DWORD) InternetDialA(
    _In_ HWND     hwndParent,
    _In_opt_ LPSTR   lpszConnectoid,
    _In_ DWORD    dwFlags,
    _Out_ DWORD_PTR *lpdwConnection,
    _Reserved_ DWORD    dwReserved
    );

INTERNETAPI_(DWORD) InternetDialW(
    _In_ HWND     hwndParent,
    _In_opt_ LPWSTR   lpszConnectoid,
    _In_ DWORD    dwFlags,
    _Out_ DWORD_PTR *lpdwConnection,
    _Reserved_ DWORD    dwReserved
    );

#ifdef UNICODE
#define InternetDial  InternetDialW
#else
#ifdef _WINX32_
#define InternetDial  InternetDialA
#else
INTERNETAPI_(DWORD) InternetDial(
    _In_ HWND     hwndParent,
    _In_opt_ LPSTR   lpszConnectoid,
    _In_ DWORD    dwFlags,
    _Out_ LPDWORD lpdwConnection,
    _In_ DWORD    dwReserved
    );
#endif // _WINX32_
#endif // !UNICODE

// Flags for InternetDial - must not conflict with InternetAutodial flags
//                          as they are valid here also.
#define INTERNET_DIAL_FORCE_PROMPT     0x2000
#define INTERNET_DIAL_SHOW_OFFLINE     0x4000
#define INTERNET_DIAL_UNATTENDED       0x8000

INTERNETAPI_(DWORD) InternetHangUp(
    _In_   DWORD_PTR    dwConnection,
    _Reserved_ DWORD        dwReserved);

#define INTERENT_GOONLINE_REFRESH  0x00000001
#define INTERENT_GOONLINE_NOPROMPT 0x00000002
#define INTERENT_GOONLINE_MASK     0x00000003

BOOLAPI InternetGoOnlineA(
    _In_opt_ LPCSTR lpszURL,
    _In_ HWND     hwndParent,
    _In_ DWORD    dwFlags
    );

BOOLAPI InternetGoOnlineW(
    _In_opt_ LPCWSTR lpszURL,
    _In_ HWND     hwndParent,
    _In_ DWORD    dwFlags
    );

#ifdef UNICODE
#define InternetGoOnline  InternetGoOnlineW
#else
#ifdef _WINX32_
#define InternetGoOnline  InternetGoOnlineA
#else
BOOLAPI InternetGoOnline(
    _In_opt_ LPSTR   lpszURL,
    _In_ HWND     hwndParent,
    _In_ DWORD    dwFlags
    );
#endif // _WINX32_
#endif // !UNICODE

BOOLAPI InternetAutodial(
    _In_       DWORD  dwFlags,
    _In_opt_ HWND   hwndParent);

// Flags for InternetAutodial
#define INTERNET_AUTODIAL_FORCE_ONLINE          1
#define INTERNET_AUTODIAL_FORCE_UNATTENDED      2
#define INTERNET_AUTODIAL_FAILIFSECURITYCHECK   4
#define INTERNET_AUTODIAL_OVERRIDE_NET_PRESENT  8


#define INTERNET_AUTODIAL_FLAGS_MASK (INTERNET_AUTODIAL_FORCE_ONLINE | INTERNET_AUTODIAL_FORCE_UNATTENDED | INTERNET_AUTODIAL_FAILIFSECURITYCHECK | INTERNET_AUTODIAL_OVERRIDE_NET_PRESENT)
BOOLAPI InternetAutodialHangup(
    _Reserved_ DWORD    dwReserved);

BOOLAPI InternetGetConnectedState(
    _Out_  LPDWORD  lpdwFlags,
    _Reserved_ DWORD    dwReserved);

BOOLAPI
InternetGetConnectedStateExA(
    _Out_opt_ LPDWORD lpdwFlags,
    _Out_writes_opt_(cchNameLen) LPSTR lpszConnectionName,
    _In_ DWORD cchNameLen,
    _Reserved_ DWORD dwReserved
);

BOOLAPI
InternetGetConnectedStateExW(
    _Out_opt_ LPDWORD lpdwFlags,
    _Out_writes_opt_(cchNameLen) LPWSTR lpszConnectionName,
    _In_ DWORD cchNameLen,
    _Reserved_ DWORD dwReserved
);



#define  PROXY_AUTO_DETECT_TYPE_DHCP    1
#define  PROXY_AUTO_DETECT_TYPE_DNS_A   2

struct AutoProxyHelperFunctions;

typedef struct AutoProxyHelperVtbl
{
    BOOL ( __stdcall *IsResolvable )(
            LPSTR   lpszHost);

    DWORD ( __stdcall *GetIPAddress )(
           LPSTR   lpszIPAddress,
           LPDWORD lpdwIPAddressSize);

    DWORD ( __stdcall *ResolveHostName )(
           LPSTR   lpszHostName,
           LPSTR   lpszIPAddress,
           LPDWORD lpdwIPAddressSize);

    BOOL ( __stdcall *IsInNet )(
            LPSTR   lpszIPAddress,
            LPSTR   lpszDest,
            LPSTR   lpszMask);

    BOOL ( __stdcall *IsResolvableEx )(
            LPSTR lpszHost
        );

    DWORD ( __stdcall *GetIPAddressEx )(
            LPSTR     lpszIPAddress,
            LPDWORD lpdwIPAddressSize
        );

    DWORD ( __stdcall *ResolveHostNameEx )(
            LPSTR lpszHostName,
            LPSTR lpszIPAddress,
            LPDWORD lpdwIPAddressSize
        );

    BOOL ( __stdcall *IsInNetEx )(
            LPSTR lpszIPAddress,
            LPSTR lpszIPPrefix
        );

    DWORD ( __stdcall *SortIpList )(
            LPSTR lpszIPAddressList,
            LPSTR lpszIPSortedList,
            LPDWORD lpdwIPSortedListSize
        );


}AutoProxyHelperVtbl;

typedef struct
{
    DWORD dwStructSize;              // Size of struct
    LPSTR lpszScriptBuffer;          // Buffer to Pass
    DWORD dwScriptBufferSize;        // Size of buffer above
} AUTO_PROXY_SCRIPT_BUFFER, *LPAUTO_PROXY_SCRIPT_BUFFER;

typedef struct AutoProxyHelperFunctions
{
    const struct AutoProxyHelperVtbl * lpVtbl;
} AutoProxyHelperFunctions;


typedef BOOL ( CALLBACK *pfnInternetInitializeAutoProxyDll)(DWORD dwVersion,
                           LPSTR lpszDownloadedTempFile,
                           LPSTR lpszMime,
                           AutoProxyHelperFunctions* lpAutoProxyCallbacks,
                           LPAUTO_PROXY_SCRIPT_BUFFER lpAutoProxyScriptBuffer );

typedef BOOL (CALLBACK *pfnInternetDeInitializeAutoProxyDll)(LPSTR lpszMime,
                           DWORD dwReserved);

typedef BOOL (CALLBACK *pfnInternetGetProxyInfo)(LPCSTR lpszUrl,
                           DWORD dwUrlLength,
                           LPSTR lpszUrlHostName,
                           DWORD dwUrlHostNameLength,
                           LPSTR* lplpszProxyHostName,
                           LPDWORD lpdwProxyHostNameLength);

typedef enum{
    WPAD_CACHE_DELETE_CURRENT = 0x0,
    WPAD_CACHE_DELETE_ALL = 0x1
} WPAD_CACHE_DELETE;

BOOLAPI DeleteWpadCacheForNetworks(
    _In_ WPAD_CACHE_DELETE
);



BOOLAPI InternetInitializeAutoProxyDll(
    _In_ DWORD dwReserved
    );

BOOLAPI DetectAutoProxyUrl(
    _Out_writes_(cchAutoProxyUrl) PSTR pszAutoProxyUrl,
    _In_ DWORD cchAutoProxyUrl,
    _In_ DWORD dwDetectFlags
    );

BOOLAPI CreateMD5SSOHash (
    _In_ PWSTR    pszChallengeInfo,
    _In_ PWSTR    pwszRealm,
    _In_ PWSTR    pwszTarget,
    _Out_ PBYTE   pbHexHash
    );

#ifdef UNICODE
#define InternetGetConnectedStateEx  InternetGetConnectedStateExW
#else
#ifdef _WINX32_
#define InternetGetConnectedStateEx  InternetGetConnectedStateExA
#else
BOOLAPI InternetGetConnectedStateEx(
    _Out_ LPDWORD lpdwFlags,
    _Out_writes_opt_(dwNameLen) LPSTR lpszConnectionName,
    _In_ DWORD dwNameLen,
    _In_ DWORD dwReserved
    );
#endif // _WINX32_
#endif // !UNICODE

// Flags for InternetGetConnectedState and Ex
#define INTERNET_CONNECTION_MODEM           0x01
#define INTERNET_CONNECTION_LAN             0x02
#define INTERNET_CONNECTION_PROXY           0x04
#define INTERNET_CONNECTION_MODEM_BUSY      0x08  /* no longer used */
#define INTERNET_RAS_INSTALLED              0x10
#define INTERNET_CONNECTION_OFFLINE         0x20
#define INTERNET_CONNECTION_CONFIGURED      0x40

//
// Custom dial handler functions
//

// Custom dial handler prototype
typedef DWORD (CALLBACK * PFN_DIAL_HANDLER) (HWND, LPCSTR, DWORD, LPDWORD);

// Flags for custom dial handler
#define INTERNET_CUSTOMDIAL_CONNECT         0
#define INTERNET_CUSTOMDIAL_UNATTENDED      1
#define INTERNET_CUSTOMDIAL_DISCONNECT      2
#define INTERNET_CUSTOMDIAL_SHOWOFFLINE     4

// Custom dial handler supported functionality flags
#define INTERNET_CUSTOMDIAL_SAFE_FOR_UNATTENDED 1
#define INTERNET_CUSTOMDIAL_WILL_SUPPLY_STATE   2
#define INTERNET_CUSTOMDIAL_CAN_HANGUP          4

BOOLAPI InternetSetDialStateA(
    _In_opt_ LPCSTR lpszConnectoid,
    _In_ DWORD    dwState,
    _Reserved_ DWORD    dwReserved
    );

BOOLAPI InternetSetDialStateW(
    _In_opt_ LPCWSTR lpszConnectoid,
    _In_ DWORD    dwState,
    _Reserved_ DWORD    dwReserved
    );

#ifdef UNICODE
#define InternetSetDialState  InternetSetDialStateW
#else
#ifdef _WINX32_
#define InternetSetDialState  InternetSetDialStateA
#else
BOOLAPI InternetSetDialState(
    _In_opt_ LPCSTR lpszConnectoid,
    _In_ DWORD    dwState,
    _In_ DWORD    dwReserved
    );
#endif // _WINX32_
#endif // !UNICODE

// States for InternetSetDialState
#define INTERNET_DIALSTATE_DISCONNECTED     1



BOOLAPI InternetSetPerSiteCookieDecisionA(
    _In_ LPCSTR pchHostName,
    _In_ DWORD dwDecision
);
BOOLAPI InternetSetPerSiteCookieDecisionW(
    _In_ LPCWSTR pchHostName,
    _In_ DWORD dwDecision
);
#ifdef UNICODE
#define InternetSetPerSiteCookieDecision  InternetSetPerSiteCookieDecisionW
#else
#define InternetSetPerSiteCookieDecision  InternetSetPerSiteCookieDecisionA
#endif // !UNICODE

BOOLAPI InternetGetPerSiteCookieDecisionA(
    _In_ LPCSTR pchHostName,
    _Out_ unsigned long* pResult
);
BOOLAPI InternetGetPerSiteCookieDecisionW(
    _In_ LPCWSTR pchHostName,
    _Out_ unsigned long* pResult
);
#ifdef UNICODE
#define InternetGetPerSiteCookieDecision  InternetGetPerSiteCookieDecisionW
#else
#define InternetGetPerSiteCookieDecision  InternetGetPerSiteCookieDecisionA
#endif // !UNICODE

BOOLAPI InternetClearAllPerSiteCookieDecisions();


BOOLAPI InternetEnumPerSiteCookieDecisionA(
    _Out_writes_to_(*pcSiteNameSize, *pcSiteNameSize) LPSTR pszSiteName,
    _Inout_ unsigned long *pcSiteNameSize,
    _Out_ unsigned long *pdwDecision,
    _In_ unsigned long dwIndex
);
BOOLAPI InternetEnumPerSiteCookieDecisionW(
    _Out_writes_to_(*pcSiteNameSize, *pcSiteNameSize) LPWSTR pszSiteName,
    _Inout_ unsigned long *pcSiteNameSize,
    _Out_ unsigned long *pdwDecision,
    _In_ unsigned long dwIndex
);
#ifdef UNICODE
#define InternetEnumPerSiteCookieDecision  InternetEnumPerSiteCookieDecisionW
#else
#define InternetEnumPerSiteCookieDecision  InternetEnumPerSiteCookieDecisionA
#endif // !UNICODE


#define INTERNET_IDENTITY_FLAG_PRIVATE_CACHE        0x01
#define INTERNET_IDENTITY_FLAG_SHARED_CACHE         0x02
#define INTERNET_IDENTITY_FLAG_CLEAR_DATA           0x04
#define INTERNET_IDENTITY_FLAG_CLEAR_COOKIES        0x08
#define INTERNET_IDENTITY_FLAG_CLEAR_HISTORY        0x10
#define INTERNET_IDENTITY_FLAG_CLEAR_CONTENT        0x20

#define INTERNET_SUPPRESS_RESET_ALL                 0x00
#define INTERNET_SUPPRESS_COOKIE_POLICY             0x01
#define INTERNET_SUPPRESS_COOKIE_POLICY_RESET       0x02

//
// Privacy settings values and APIs
//

#define PRIVACY_TEMPLATE_NO_COOKIES     0
#define PRIVACY_TEMPLATE_HIGH           1
#define PRIVACY_TEMPLATE_MEDIUM_HIGH    2
#define PRIVACY_TEMPLATE_MEDIUM         3
#define PRIVACY_TEMPLATE_MEDIUM_LOW     4
#define PRIVACY_TEMPLATE_LOW            5
#define PRIVACY_TEMPLATE_CUSTOM         100
#define PRIVACY_TEMPLATE_ADVANCED       101

#define PRIVACY_TEMPLATE_MAX            PRIVACY_TEMPLATE_LOW

#define PRIVACY_TYPE_FIRST_PARTY        0
#define PRIVACY_TYPE_THIRD_PARTY        1

INTERNETAPI_(DWORD)
PrivacySetZonePreferenceW(
    _In_ DWORD       dwZone,
    _In_ DWORD       dwType,
    _In_ DWORD       dwTemplate,
    _In_opt_ LPCWSTR     pszPreference
    );

INTERNETAPI_(DWORD)
PrivacyGetZonePreferenceW(
    _In_ DWORD dwZone,
    _In_ DWORD dwType,
    _Out_opt_ LPDWORD pdwTemplate,
    _Out_writes_opt_(*pdwBufferLength) LPWSTR pszBuffer,
    _Inout_opt_ LPDWORD pdwBufferLength
    );


INTERNETAPI_(DWORD)
HttpIsHostHstsEnabled(
    _In_z_ PCWSTR pcwszUrl,
    _Out_ PBOOL pfIsHsts
);



#if defined(__cplusplus)
}
#endif


/*
 * Return packing to whatever it was before we
 * entered this file
 */
#include <poppack.h>


#endif // !defined(_WININET_)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion

