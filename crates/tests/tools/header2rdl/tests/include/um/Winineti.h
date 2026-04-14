
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#include <iedial.h>
#include <schannel.h>

#if !defined(SECURITY_WIN32)
#define SECURITY_WIN32
#endif

#include <sspi.h>

#if !defined(_WININETEX_)
#define _WININETEX_

#if defined(__cplusplus)
extern "C" {
#endif

#define MAX_CACHE_ENTRY_INFO_SIZE       4096
#define INTERNET_REQFLAG_FROM_APP_CACHE 0x00000100  // Response came from AppCache

//
// These two flags should really be included in a new mask called INTERNET_FLAGS_INTERNAL_MASK.
// Since BGUPDATE is included in INTERNET_FLAGS_MASK already it will cause compile
// errors when used without winineti.h.  FTP_FOLDER_VIEW is not being included
// so as to not compound the problem.
//

#define INTERNET_FLAG_BGUPDATE          0x00000008
#define INTERNET_FLAG_FTP_FOLDER_VIEW   0x00000004
#define INTERNET_FLAGS_MASK_INTERNAL (INTERNET_FLAGS_MASK | \
                                      INTERNET_FLAG_FTP_FOLDER_VIEW)

//
// INTERNET_PREFETCH_STATUS -
//

typedef struct {

    //
    // dwStatus - status of download. See INTERNET_PREFETCH_ flags
    //

    DWORD dwStatus;

    //
    // dwSize - size of file downloaded so far
    //

    DWORD dwSize;
} INTERNET_PREFETCH_STATUS, * LPINTERNET_PREFETCH_STATUS;

//
// INTERNET_PREFETCH_STATUS - dwStatus values
//

#define INTERNET_PREFETCH_PROGRESS  0
#define INTERNET_PREFETCH_COMPLETE  1
#define INTERNET_PREFETCH_ABORTED   2

#ifdef _WS2DEF_

typedef struct {
    SOCKADDR_STORAGE LocalAddress;
    SOCKADDR_STORAGE RemoteAddress;
} INTERNET_CONNECTION_INFO, *PINTERNET_CONNECTION_INFO;

#endif

#define INTERNET_ONLINE_OFFLINE_INFO    INTERNET_CONNECTED_INFO
#define LPINTERNET_ONLINE_OFFLINE_INFO  LPINTERNET_CONNECTED_INFO
#define dwOfflineState                  dwConnectedState


#define ISO_FORCE_OFFLINE       ISO_FORCE_DISCONNECTED


// These flags keep track of security errors on a cached certificate entry
// from WinVerify that was ignored in order to complete the security handshake.
// These flags should not use the same bits as any other SECURITY_FLAGS.

#define DLG_FLAGS_INVALID_CA                    0x01000000
#define DLG_FLAGS_SEC_CERT_CN_INVALID           0x02000000
#define DLG_FLAGS_SEC_CERT_DATE_INVALID         0x04000000
#define DLG_FLAGS_WEAK_SIGNATURE                0x00200000
#define DLG_FLAGS_INSECURE_FALLBACK             0x00400000
#define DLG_FLAGS_SEC_CERT_REV_FAILED           0x00800000

#ifdef __WINCRYPT_H__
#ifdef ALGIDDEF

//
// INTERNET_SECURITY_INFO - contains information about certificate
// and encryption settings for a connection.
//

#define INTERNET_SECURITY_INFO_DEFINED

typedef struct {

    //
    // dwSize - Size of INTERNET_SECURITY_INFO structure.
    //
    DWORD dwSize;

    //
    // pCertificate - Cert context pointing to leaf of certificate chain.
    //
    PCCERT_CONTEXT pCertificate;


    //
    // pcCertChain - Certificate chain for the certificate
    //

    PCCERT_CHAIN_CONTEXT pcCertChain;


    // SECPKG_ATTR_CONNECTION_INFO
    SecPkgContext_ConnectionInfo connectionInfo;

    // SECPKG_ATTR_CIPHER_INFO (Supported on >= LH)
    SecPkgContext_CipherInfo cipherInfo;

    //
    // pcUnverifiedCertChain - Cert chain from initial WinVerifyTrust state
    //
    PCCERT_CHAIN_CONTEXT pcUnverifiedCertChain;

    //
    // Channel Binding Token
    //

    SecPkgContext_Bindings channelBindingToken;

} INTERNET_SECURITY_INFO, * LPINTERNET_SECURITY_INFO;


typedef struct {
    //
    // dwSize - size of INTERNET_SECURITY_CONNECTION_INFO
    //
    DWORD dwSize;

    // fSecure - Is this a secure connection.
    BOOL fSecure;

    // SECPKG_ATTR_CONNECTION_INFO
    SecPkgContext_ConnectionInfo connectionInfo;

    // SECPKG_ATTR_CIPHER_INFO (Supported on >= LH)
    SecPkgContext_CipherInfo cipherInfo;
} INTERNET_SECURITY_CONNECTION_INFO , * LPINTERNET_SECURITY_CONNECTION_INFO;


BOOLAPI InternetAlgIdToStringA(
    _In_                            ALG_ID  ai,
    _Out_writes_(*lpdwstrLength)    LPSTR lpstr,
    _Inout_                         LPDWORD lpdwstrLength,
    _Reserved_                      DWORD   dwReserved
    );
BOOLAPI InternetAlgIdToStringW(
    _In_                            ALG_ID  ai,
    _Out_writes_(*lpdwstrLength)    LPWSTR lpstr,
    _Inout_                         LPDWORD lpdwstrLength,
    _Reserved_                      DWORD   dwReserved
    );
#ifdef UNICODE
#define InternetAlgIdToString  InternetAlgIdToStringW
#else
#define InternetAlgIdToString  InternetAlgIdToStringA
#endif // !UNICODE

BOOLAPI
InternetSecurityProtocolToStringA(
    _In_                                    DWORD   dwProtocol,
    _Out_writes_to_opt_(*lpdwstrLength, *lpdwstrLength)    LPSTR lpstr,
    _Inout_                                 LPDWORD lpdwstrLength,
    _Reserved_                              DWORD   dwReserved
    );
BOOLAPI
InternetSecurityProtocolToStringW(
    _In_                                    DWORD   dwProtocol,
    _Out_writes_to_opt_(*lpdwstrLength, *lpdwstrLength)    LPWSTR lpstr,
    _Inout_                                 LPDWORD lpdwstrLength,
    _Reserved_                              DWORD   dwReserved
    );
#ifdef UNICODE
#define InternetSecurityProtocolToString  InternetSecurityProtocolToStringW
#else
#define InternetSecurityProtocolToString  InternetSecurityProtocolToStringA
#endif // !UNICODE

#if (_WIN32_IE >= _WIN32_IE_IE70)

//
// This is an API for BrowseUI.  It retrieves
// security info based on a URL
//

BOOLAPI InternetGetSecurityInfoByURLA(
    _In_       LPSTR    lpszURL,
    _Out_      PCCERT_CHAIN_CONTEXT * ppCertChain,
    _Out_      DWORD  *pdwSecureFlags
    );

BOOLAPI InternetGetSecurityInfoByURLW(
    _In_       LPCWSTR  lpszURL,
    _Out_      PCCERT_CHAIN_CONTEXT * ppCertChain,
    _Out_      DWORD  *pdwSecureFlags
    );

#ifdef UNICODE
#define InternetGetSecurityInfoByURL  InternetGetSecurityInfoByURLW
#else
#ifdef _WINX32_
#define InternetGetSecurityInfoByURL  InternetGetSecurityInfoByURLA
#else
BOOLAPI InternetGetSecurityInfoByURL(
    _In_       LPSTR    lpszURL,
    _Out_      PCCERT_CHAIN_CONTEXT * ppCertChain,
    _Out_      DWORD  *pdwSecureFlags
    );
#endif // _WINX32_
#endif // !UNICODE

#endif // _WIN32_IE >= _WIN32_IE_IE70

#endif // ALGIDDEF
#endif // __WINCRYPT_H__

#ifdef INTERNET_SECURITY_INFO_DEFINED

INTERNETAPI_(DWORD) ShowSecurityInfo(
    _In_ HWND                          hWndParent,
    _In_ LPINTERNET_SECURITY_INFO      pSecurityInfo
    );
#endif // INTERNET_SECURITY_INFO_DEFINED



INTERNETAPI_(DWORD) ShowX509EncodedCertificate(
    _In_                HWND    hWndParent,
    _In_reads_bytes_(cbCert) LPBYTE  lpCert,
    _In_                DWORD   cbCert
    );

INTERNETAPI_(DWORD) ShowClientAuthCerts(
    _In_ HWND hWndParent
    );

INTERNETAPI_(DWORD) ParseX509EncodedCertificateForListBoxEntry(
    _In_reads_bytes_(cbCert)                     LPBYTE  lpCert,
    _In_                                    DWORD   cbCert,
    _Out_writes_opt_(*lpdwListBoxEntry)     LPSTR   lpszListBoxEntry,
    _Inout_                                 LPDWORD lpdwListBoxEntry
    );

//
// This is a private API for Trident.  It displays
// security info based on a URL
//

BOOLAPI InternetShowSecurityInfoByURLA(
    _In_       LPSTR    lpszURL,
    _In_       HWND     hwndParent
    );

BOOLAPI InternetShowSecurityInfoByURLW(
    _In_       LPCWSTR  lpszURL,
    _In_       HWND     hwndParent
    );

// The InternetGetCertByURL function was never declared in the wininet header
// files.  It is obsolete and deprecated. An attempt was made to add it for
// completeness since it is still exported from wininet.dll. However other
// code has declared it incorrectly.  This causes compiler errors because
// the function ends up with two different declarations in the same compilation
// unit.  It is again not included in the header as anything but this comment.
//
// BOOLAPI InternetGetCertByURL(
//    _In_        LPSTR   lpszURL,
//    _Inout_updates_bytes_(dwcbCertText) LPSTR lpszCertText,
//    _Inout_     DWORD    dwcbCertText
//    );

#ifdef UNICODE
#define InternetShowSecurityInfoByURL  InternetShowSecurityInfoByURLW
#else
#ifdef _WINX32_
#define InternetShowSecurityInfoByURL  InternetShowSecurityInfoByURLA
#else
BOOLAPI InternetShowSecurityInfoByURL(
    _In_       LPSTR    lpszURL,
    _In_       HWND     hwndParent
    );
#endif // _WINX32_
#endif // !UNICODE

//Fortezza related exports. not public

// The commands that InternetFortezzaCommand supports.

typedef enum {
    FORTCMD_LOGON                   = 1,
    FORTCMD_LOGOFF                  = 2,
    FORTCMD_CHG_PERSONALITY         = 3,
} FORTCMD;


BOOLAPI InternetFortezzaCommand(
    _In_ DWORD dwCommand,
    _In_ HWND hwnd,
    _Reserved_ DWORD_PTR dwReserved);


typedef enum {
    FORTSTAT_INSTALLED          = 0x00000001,
    FORTSTAT_LOGGEDON           = 0x00000002,
}   FORTSTAT ;

BOOLAPI InternetQueryFortezzaStatus(
    _Out_ DWORD *pdwStatus,
    _Reserved_ DWORD_PTR dwReserved
);



BOOLAPI InternetDebugGetLocalTime(
    _Out_ SYSTEMTIME * pstLocalTime,
    _Out_opt_ DWORD      * pdwReserved
    );


// causes InternetCreateUrlA to escape chars in authority components (user, pwd, host)
#define ICU_ESCAPE_AUTHORITY 0x00002000

#define INTERNET_SERVICE_URL    0
//
// InternetConnectUrl() - a macro which allows you to specify an URL instead of
// the component parts to InternetConnect(). If any API which uses the returned
// connect handle specifies a NULL path then the URL-path part of the URL
// specified in InternetConnectUrl() will be used
//

#define InternetConnectUrl(hInternet, lpszUrl, dwFlags, dwContext) \
    InternetConnect(hInternet,                      \
                    lpszUrl,                        \
                    INTERNET_INVALID_PORT_NUMBER,   \
                    NULL,                           \
                    NULL,                           \
                    INTERNET_SERVICE_URL,           \
                    dwFlags,                        \
                    dwContext                       \
                    )

BOOLAPI InternetWriteFileExA(
    _In_ HINTERNET hFile,
    _In_ LPINTERNET_BUFFERSA lpBuffersIn,
    _In_ DWORD dwFlags,
    _In_opt_ DWORD_PTR dwContext
    );
BOOLAPI InternetWriteFileExW(
    _In_ HINTERNET hFile,
    _In_ LPINTERNET_BUFFERSW lpBuffersIn,
    _In_ DWORD dwFlags,
    _In_opt_ DWORD_PTR dwContext
    );
#ifdef UNICODE
#define InternetWriteFileEx  InternetWriteFileExW
#else
#define InternetWriteFileEx  InternetWriteFileExA
#endif // !UNICODE

#define INTERNET_OPTION_CONTEXT_VALUE_OLD       10
#define INTERNET_OPTION_NET_SPEED               61
// Pass in pointer to INTERNET_SECURITY_CONNECTION_INFO to be filled in.
#define INTERNET_OPTION_SECURITY_CONNECTION_INFO  66
#define INTERNET_OPTION_DETECT_POST_SEND        71
#define INTERNET_OPTION_DISABLE_NTLM_PREAUTH    72
#define INTERNET_OPTION_ORIGINAL_CONNECT_FLAGS                  97


#define INTERNET_OPTION_CERT_ERROR_FLAGS             98
#define INTERNET_OPTION_IGNORE_CERT_ERROR_FLAGS      99

#define INTERNET_OPTION_SESSION_START_TIME           106
#define INTERNET_OPTION_PROXY_CREDENTIALS            107
#define INTERNET_OPTION_EXTENDED_CALLBACKS           108
#define INTERNET_OPTION_PROXY_FROM_REQUEST           109
#define INTERNET_OPTION_ALLOW_FAILED_CONNECT_CONTENT 110
#define INTERNET_OPTION_CACHE_PARTITION              111
#define INTERNET_OPTION_AUTODIAL_HWND                112
#define INTERNET_OPTION_SERVER_CREDENTIALS           113
#define INTERNET_OPTION_WPAD_SLEEP                   114
#define INTERNET_OPTION_FAIL_ON_CACHE_WRITE_ERROR    115
#define INTERNET_OPTION_DOWNLOAD_MODE                116
#define INTERNET_OPTION_RESPONSE_RESUMABLE           117

// option for WP
#define INTERNET_OPTION_CM_HANDLE_COPY_REF           118

#define INTERNET_OPTION_CONNECTION_INFO              120
#define INTERNET_OPTION_BACKGROUND_CONNECTIONS       121
#define INTERNET_OPTION_DO_NOT_TRACK                 123
#define INTERNET_OPTION_USE_MODIFIED_HEADER_FILTER   124
#define INTERNET_OPTION_WWA_MODE                     125
#define INTERNET_OPTION_UPGRADE_TO_WEB_SOCKET        126
#define INTERNET_OPTION_WEB_SOCKET_KEEPALIVE_INTERVAL           127
#define INTERNET_OPTION_UNLOAD_NOTIFY_EVENT          128
#define INTERNET_OPTION_SOCKET_NODELAY               129
#define INTERNET_OPTION_APP_CACHE                    130
#define INTERNET_OPTION_DEPENDENCY_HANDLE            131
#define INTERNET_OPTION_USE_FIRST_AVAILABLE_CONNECTION          132
#define INTERNET_OPTION_TIMED_CONNECTION_LIMIT_BYPASS           133
#define INTERNET_OPTION_WEB_SOCKET_CLOSE_TIMEOUT     134
#define INTERNET_OPTION_FLUSH_STATE                  135
#define INTERNET_OPTION_DISALLOW_PREMATURE_EOF       137
#define INTERNET_OPTION_SOCKET_NOTIFICATION_IOCTL    138
#define INTERNET_OPTION_CACHE_ENTRY_EXTRA_DATA       139
#define INTERNET_OPTION_MAX_QUERY_BUFFER_SIZE        140
#define INTERNET_OPTION_FALSE_START                  141
#define INTERNET_OPTION_USER_PASS_SERVER_ONLY        142
#define INTERNET_OPTION_SERVER_AUTH_SCHEME           143
#define INTERNET_OPTION_PROXY_AUTH_SCHEME            144
#define INTERNET_OPTION_TUNNEL_ONLY                  145
#define INTERNET_OPTION_SOURCE_PORT                  146
#define INTERNET_OPTION_ENABLE_DUO                   INTERNET_OPTION_ENABLE_HTTP_PROTOCOL
#define INTERNET_OPTION_DUO_USED                     INTERNET_OPTION_HTTP_PROTOCOL_USED
#define INTERNET_OPTION_CHUNK_ENCODE_REQUEST         150
#define INTERNET_OPTION_SECURE_FAILURE               151
#define INTERNET_OPTION_NOTIFY_SENDING_COOKIE        152
#define INTERNET_OPTION_CLIENT_CERT_ISSUER_LIST      153
#define INTERNET_OPTION_RESET                        154
#define INTERNET_OPTION_SERVER_ADDRESS_INFO          156
#define INTERNET_OPTION_ENABLE_WBOEXT                158
#define INTERNET_OPTION_DISABLE_INSECURE_FALLBACK    160
#define INTERNET_OPTION_ALLOW_INSECURE_FALLBACK      161
#define INTERNET_OPTION_SET_IN_PRIVATE               164
#define INTERNET_OPTION_DOWNLOAD_MODE_HANDLE         165
#define INTERNET_OPTION_EDGE_COOKIES                 166
#define INTERNET_OPTION_NO_HTTP_SERVER_AUTH          167
#define INTERNET_OPTION_ENABLE_HEADER_CALLBACKS      168
#define INTERNET_OPTION_PRESERVE_REQUEST_SERVER_CREDENTIALS_ON_REDIRECT 169
#define INTERNET_OPTION_PRESERVE_REFERER_ON_HTTPS_TO_HTTP_REDIRECT 170
#define INTERNET_OPTION_TCP_FAST_OPEN                171
#define INTERNET_OPTION_SYNC_MODE_AUTOMATIC_SESSION_DISABLED 172
#define INTERNET_OPTION_ENABLE_ZLIB_DEFLATE          173
#define INTERNET_OPTION_ENCODE_FALLBACK_FOR_REDIRECT_URI 174
#define INTERNET_OPTION_EDGE_COOKIES_TEMP            175
#define INTERNET_OPTION_OPT_IN_WEAK_SIGNATURE        176
#define INTERNET_OPTION_PARSE_LINE_FOLDING           177
#define INTERNET_OPTION_FORCE_DECODE                 178
#define INTERNET_OPTION_COOKIES_APPLY_HOST_ONLY      179
#define INTERNET_OPTION_EDGE_MODE                    180
#define INTERNET_OPTION_CANCEL_CACHE_WRITE           182
#define INTERNET_OPTION_AUTH_SCHEME_SELECTED         183
#define INTERNET_OPTION_NOCACHE_WRITE_IN_PRIVATE     184
#define INTERNET_OPTION_ACTIVITY_ID                  185
#define INTERNET_OPTION_REQUEST_TIMES                186
#define INTERNET_OPTION_GLOBAL_CALLBACK              188
#define INTERNET_OPTION_ENABLE_TEST_SIGNING          189
#define INTERNET_OPTION_DISABLE_PROXY_LINK_LOCAL_NAME_RESOLUTION 190
#define INTERNET_OPTION_HTTP_09                      191
#define INTERNET_OPTION_CALLER_MODULE                192
#define INTERNET_LAST_OPTION_INTERNAL           INTERNET_OPTION_REQUEST_ANNOTATION
#define INTERNET_OPTION_OFFLINE_TIMEOUT INTERNET_OPTION_DISCONNECTED_TIMEOUT
#define INTERNET_OPTION_LINE_STATE      INTERNET_OPTION_CONNECTED_STATE

//
// Deprecated - Values for INTERNET_OPTION_ENABLE_DUO / INTERNET_OPTION_DUO_USED
//

#define DUO_PROTOCOL_FLAG_SPDY3 0x1
#define DUO_PROTOCOL_MASK (DUO_PROTOCOL_FLAG_SPDY3)

typedef struct _INTERNET_DOWNLOAD_MODE_HANDLE
{
    PCWSTR pcwszFileName;
    HANDLE *phFile;
} INTERNET_DOWNLOAD_MODE_HANDLE, *PINTERNET_DOWNLOAD_MODE_HANDLE;

typedef enum _REQUEST_TIMES
{
    NameResolutionStart = 0,
    NameResolutionEnd,
    ConnectionEstablishmentStart,
    ConnectionEstablishmentEnd,
    TLSHandshakeStart,
    TLSHandshakeEnd,
    HttpRequestTimeMax = 32
} REQUEST_TIMES;

typedef struct _HTTP_REQUEST_TIMES
{
    ULONG cTimes;
    ULONGLONG rgTimes[HttpRequestTimeMax];
} HTTP_REQUEST_TIMES;

#define AUTH_FLAG_RESET                         0x00000000 /* let registry decide */

//
// Internet Auth Schemes
//

#define INTERNET_AUTH_SCHEME_BASIC          0
#define INTERNET_AUTH_SCHEME_DIGEST         1
#define INTERNET_AUTH_SCHEME_NTLM           2
#define INTERNET_AUTH_SCHEME_KERBEROS       3
#define INTERNET_AUTH_SCHEME_NEGOTIATE      4
#define INTERNET_AUTH_SCHEME_PASSPORT       5
#define INTERNET_AUTH_SCHEME_UNKNOWN        6


#define INTERNET_STATUS_SENDING_COOKIE          328
#define INTERNET_STATUS_REQUEST_HEADERS_SET     329
#define INTERNET_STATUS_RESPONSE_HEADERS_SET    330

//
// Extended callbacks.
//
// They can happen outside the context of the registered session's API calls.
//

#define INTERNET_STATUS_PROXY_CREDENTIALS       400
#define INTERNET_STATUS_SERVER_CREDENTIALS      401
#define INTERNET_STATUS_SERVER_CONNECTION_STATE 410
#define INTERNET_STATUS_END_BROWSER_SESSION     420
#define INTERNET_STATUS_COOKIE                  430

//
// Extended callback data structures.
//

typedef struct _INTERNET_SERVER_CONNECTION_STATE {
    LPCWSTR lpcwszHostName;
    BOOL fProxy;                    // Is this a proxy?
    DWORD dwCounter;                // Server connection state callback counter.
    DWORD dwConnectionLimit;        // Current Limit.
    DWORD dwAvailableCreates;       // Connections allowed to be created.
    DWORD dwAvailableKeepAlives;    // Keep alive connections available to be used.
    DWORD dwActiveConnections;      // Current number of active connections.
    DWORD dwWaiters;                // Number of request waiting on connections.
} INTERNET_SERVER_CONNECTION_STATE, *PINTERNET_SERVER_CONNECTION_STATE;

typedef struct _INTERNET_END_BROWSER_SESSION_DATA {
    LPVOID lpBuffer;
    DWORD dwBufferLength;
} INTERNET_END_BROWSER_SESSION_DATA, *PINTERNET_END_BROWSER_SESSION_DATA;

typedef struct _INTERNET_CALLBACK_COOKIE {
    PCWSTR pcwszName;
    PCWSTR pcwszValue;
    PCWSTR pcwszDomain;
    PCWSTR pcwszPath;
    FILETIME ftExpires;
    DWORD dwFlags;
} INTERNET_CALLBACK_COOKIE, *PINTERNET_CALLBACK_COOKIE;

#if _MSC_VER >= 1200
#pragma warning(push)
#endif
#pragma warning(disable:4201)

typedef struct _INTERNET_CREDENTIALS {
    LPCWSTR lpcwszHostName;
    DWORD dwPort;
    DWORD dwScheme;
    LPCWSTR lpcwszUrl;
    LPCWSTR lpcwszRealm;
    BOOL fAuthIdentity;  // TRUE if pAuthIdentityOpaque is used in below struct.
    union {
        struct {
            LPCWSTR lpcwszUserName;
            LPCWSTR lpcwszPassword;
        };
        PVOID pAuthIdentityOpaque;
    };
} INTERNET_CREDENTIALS, *PINTERNET_CREDENTIALS;

#if _MSC_VER >= 1200
#pragma warning(pop)
#else
#pragma warning(default:4201)
#endif

//
// !!! NOTE !!!
//
// these defines are needed beecause PREFAST donesn't understand enums in
// range specifications. Make sue that these are always in sync with any
// changes to the InternetCookieState enum, above.
//
#define COOKIE_STATE_LB     0   // COOKIE_STATE_UNKNOWN
#define COOKIE_STATE_UB     5   // COOKIE_STATE_MAX

/* maximum length of privacy-settings in Unicode characters */
#define     MaxPrivacySettings      0x4000

// Utility for mapping P3P compact-policy tokens to unique indexes
INTERNETAPI_(int)   FindP3PPolicySymbol(_In_ const char *pszSymbol);



#define INTERNET_STATE_ONLINE       INTERNET_STATE_CONNECTED
#define INTERNET_STATE_OFFLINE      INTERNET_STATE_DISCONNECTED
#define INTERNET_STATE_OFFLINE_USER INTERNET_STATE_DISCONNECTED_BY_USER
#define INTERNET_LINE_STATE_MASK    (INTERNET_STATE_ONLINE | INTERNET_STATE_OFFLINE)
#define INTERNET_BUSY_STATE_MASK    (INTERNET_STATE_IDLE | INTERNET_STATE_BUSY)



//
// the following are used with InternetSetOption(..., INTERNET_OPTION_CALLBACK_FILTER, ...)
// to filter out unrequired callbacks. INTERNET_STATUS_REQUEST_COMPLETE cannot
// be filtered out
//

#define INTERNET_STATUS_FILTER_RESOLVING        0x00000001
#define INTERNET_STATUS_FILTER_RESOLVED         0x00000002
#define INTERNET_STATUS_FILTER_CONNECTING       0x00000004
#define INTERNET_STATUS_FILTER_CONNECTED        0x00000008
#define INTERNET_STATUS_FILTER_SENDING          0x00000010
#define INTERNET_STATUS_FILTER_SENT             0x00000020
#define INTERNET_STATUS_FILTER_RECEIVING        0x00000040
#define INTERNET_STATUS_FILTER_RECEIVED         0x00000080
#define INTERNET_STATUS_FILTER_CLOSING          0x00000100
#define INTERNET_STATUS_FILTER_CLOSED           0x00000200
#define INTERNET_STATUS_FILTER_HANDLE_CREATED   0x00000400
#define INTERNET_STATUS_FILTER_HANDLE_CLOSING   0x00000800
#define INTERNET_STATUS_FILTER_PREFETCH         0x00001000
#define INTERNET_STATUS_FILTER_REDIRECT         0x00002000
#define INTERNET_STATUS_FILTER_STATE_CHANGE     0x00004000


//
// Note that adding any HTTP_QUERY_* codes here must be followed
//   by an equivlent line in wininet\http\hashgen\hashgen.cpp
//   please see that file for further information regarding
//   the addition of new HTTP headers
//


// These are not part of HTTP 1.1 yet. We will propose these to the
// HTTP extensions working group. These are required for the client-caps support
// we are doing in conjuntion with IIS.


//
// Modify the response headers rather than request headers.  The set of response headers allowed to be
// modified is restricted.
//

#define HTTP_ADDREQ_FLAG_RESPONSE_HEADERS 0x02000000

#define HTTP_ADDREQ_FLAG_ALLOW_EMPTY_VALUES 0x04000000

INTERNETAPI_(DWORD)
HttpGetServerCredentials(
    _In_ PWSTR pwszUrl,
    _Outptr_result_z_ PWSTR *ppwszUserName,
    _Outptr_result_z_ PWSTR *ppwszPassword
);

//
// HTTP push notifications
//

DECLARE_HANDLE(HTTP_PUSH_WAIT_HANDLE);

typedef struct _HTTP_PUSH_TRANSPORT_SETTING
{
    GUID TransportSettingId;
    GUID BrokerEventId;
} HTTP_PUSH_TRANSPORT_SETTING, *PHTTP_PUSH_TRANSPORT_SETTING;

typedef struct _HTTP_PUSH_NOTIFICATION_STATUS
{
    BOOL ChannelStatusValid;
    DWORD ChannelStatus;
} HTTP_PUSH_NOTIFICATION_STATUS, *PHTTP_NOTIFICATION_STATUS;

typedef enum  _HTTP_PUSH_WAIT_TYPE
{
    HttpPushWaitEnableComplete,
    HttpPushWaitReceiveComplete,
    HttpPushWaitSendComplete
} HTTP_PUSH_WAIT_TYPE;

INTERNETAPI_(DWORD)
HttpPushEnable(
    _In_ HINTERNET hRequest,
    _In_ HTTP_PUSH_TRANSPORT_SETTING *pTransportSetting,
    _Out_ HTTP_PUSH_WAIT_HANDLE *phWait
);

INTERNETAPI_(DWORD)
HttpPushWait(
    _In_ HTTP_PUSH_WAIT_HANDLE hWait,
    _In_ HTTP_PUSH_WAIT_TYPE eType,
    _Out_opt_ HTTP_PUSH_NOTIFICATION_STATUS *pNotificationStatus
);

INTERNETAPI_(VOID)
HttpPushClose(
    _In_ HTTP_PUSH_WAIT_HANDLE hWait
);

typedef struct _INTERNET_COOKIE {
    DWORD cbSize;
    LPSTR pszName;
    LPSTR pszData;
    LPSTR pszDomain;
    LPSTR pszPath;
    FILETIME *pftExpires;
    DWORD dwFlags;
    LPSTR pszUrl;
    LPSTR pszP3PPolicy;
} INTERNET_COOKIE, *PINTERNET_COOKIE;

//
// Support for handling cookie policy
//

typedef struct _COOKIE_DLG_INFO {
    LPWSTR  pszServer;
    PINTERNET_COOKIE pic;
    DWORD   dwStopWarning;
    INT     cx;
    INT     cy;
    LPWSTR  pszHeader;
    DWORD   dwOperation;
} COOKIE_DLG_INFO, *PCOOKIE_DLG_INFO;

// values returned from cookie UI, for dwStopWarning member
#define COOKIE_DONT_ALLOW       1
#define COOKIE_ALLOW            2
#define COOKIE_ALLOW_ALL        4
#define COOKIE_DONT_ALLOW_ALL   8

// values for dwOperation member
#define COOKIE_OP_SET           0x01
#define COOKIE_OP_MODIFY        0x02
#define COOKIE_OP_GET           0x04
#define COOKIE_OP_SESSION       0x08
#define COOKIE_OP_PERSISTENT    0x10
#define COOKIE_OP_3RD_PARTY     0x20



#define INTERNET_COOKIE_PERSISTENT_HOST_ONLY       0x00010000
// INTERNET_COOKIE_RESTRICTED_ZONE is the same as INTERNET_FLAG_RESTRICTED_ZONE
#define INTERNET_COOKIE_RESTRICTED_ZONE            0x00020000
#define INTERNET_COOKIE_EDGE_COOKIES               0x00040000

#define INTERNET_COOKIE_ALL_COOKIES                0x20000000
#define INTERNET_COOKIE_NO_CALLBACK                0x40000000
#define INTERNET_COOKIE_ECTX_3RDPARTY              0x80000000

#define COOKIE_ALLOWED_SET_FLAGS        ( INTERNET_COOKIE_THIRD_PARTY     | \
                                          INTERNET_COOKIE_PROMPT_REQUIRED | \
                                          INTERNET_COOKIE_EVALUATE_P3P    | \
                                          INTERNET_COOKIE_NON_SCRIPT      | \
                                          INTERNET_COOKIE_RESTRICTED_ZONE | \
                                          INTERNET_COOKIE_NO_CALLBACK     | \
                                          INTERNET_COOKIE_APPLY_HOST_ONLY | \
                                          INTERNET_COOKIE_EDGE_COOKIES      \
                                        )

#define COOKIE_ALLOWED_COOKIE_FLAGS     ( INTERNET_COOKIE_IS_SECURE         | \
                                          INTERNET_COOKIE_IS_SESSION        | \
                                          INTERNET_COOKIE_IS_RESTRICTED     | \
                                          INTERNET_COOKIE_HTTPONLY          | \
                                          INTERNET_COOKIE_HOST_ONLY         | \
                                          INTERNET_COOKIE_HOST_ONLY_APPLIED | \
                                          INTERNET_COOKIE_ECTX_3RDPARTY     | \
                                          INTERNET_COOKIE_SAME_SITE_STRICT  | \
                                          INTERNET_COOKIE_SAME_SITE_LAX       \
                                        )

#define COOKIE_ALLOWED_GET_FLAGS        ( INTERNET_COOKIE_NON_SCRIPT                 | \
                                          INTERNET_COOKIE_THIRD_PARTY                | \
                                          INTERNET_COOKIE_SAME_SITE_LEVEL_CROSS_SITE | \
                                          INTERNET_FLAG_RESTRICTED_ZONE              | \
                                          INTERNET_COOKIE_ALL_COOKIES                | \
                                          INTERNET_COOKIE_EDGE_COOKIES                 \
                                        )
//
// DAV Detection
//
BOOLAPI HttpCheckDavComplianceA(
    _In_ LPCSTR lpszUrl,
    _In_ LPCSTR lpszComplianceToken,
    _Inout_ LPBOOL lpfFound,
    _In_ HWND hWnd,
    _In_ LPVOID lpvReserved
    );
//
// DAV Detection
//
BOOLAPI HttpCheckDavComplianceW(
    _In_ LPCWSTR lpszUrl,
    _In_ LPCWSTR lpszComplianceToken,
    _Inout_ LPBOOL lpfFound,
    _In_ HWND hWnd,
    _In_ LPVOID lpvReserved
    );
#ifdef UNICODE
#define HttpCheckDavCompliance  HttpCheckDavComplianceW
#else
#define HttpCheckDavCompliance  HttpCheckDavComplianceA
#endif // !UNICODE

#define FLAGS_ERROR_UI_SHOW_IDN_HOSTNAME        0x20
#define ERROR_INTERNET_NO_NEW_CONTAINERS        (INTERNET_ERROR_BASE + 51)
#define ERROR_INTERNET_SOURCE_PORT_IN_USE       (INTERNET_ERROR_BASE + 58)
#define ERROR_INTERNET_INSECURE_FALLBACK_REQUIRED (INTERNET_ERROR_BASE + 59)
#define ERROR_INTERNET_PROXY_ALERT              (INTERNET_ERROR_BASE + 61)
//
// Error code for WP.
//

#define ERROR_INTERNET_NO_CM_CONNECTION         (INTERNET_ERROR_BASE + 80)
#define ERROR_HTTP_PUSH_STATUS_CODE_NOT_SUPPORTED (INTERNET_ERROR_BASE + 147)
#define ERROR_HTTP_PUSH_RETRY_NOT_SUPPORTED     (INTERNET_ERROR_BASE + 148)
#define ERROR_HTTP_PUSH_ENABLE_FAILED           (INTERNET_ERROR_BASE + 149)
#define ERROR_INTERNET_DISALLOW_INPRIVATE       (INTERNET_ERROR_BASE + 189)

#define ERROR_INTERNET_OFFLINE  ERROR_INTERNET_DISCONNECTED

//
// internal error codes that are used to communicate specific information inside
// of Wininet but which are meaningless at the interface
//

#define INTERNET_INTERNAL_ERROR_BASE            (INTERNET_ERROR_BASE + 900)

#define ERROR_INTERNET_INTERNAL_SOCKET_ERROR    (INTERNET_INTERNAL_ERROR_BASE + 1)
#define ERROR_INTERNET_CONNECTION_AVAILABLE     (INTERNET_INTERNAL_ERROR_BASE + 2)
#define ERROR_INTERNET_NO_KNOWN_SERVERS         (INTERNET_INTERNAL_ERROR_BASE + 3)
#define ERROR_INTERNET_PING_FAILED              (INTERNET_INTERNAL_ERROR_BASE + 4)
#define ERROR_INTERNET_NO_PING_SUPPORT          (INTERNET_INTERNAL_ERROR_BASE + 5)
#define ERROR_INTERNET_CACHE_SUCCESS            (INTERNET_INTERNAL_ERROR_BASE + 6)
#define ERROR_HTTP_COOKIE_NEEDS_CONFIRMATION_EX (INTERNET_INTERNAL_ERROR_BASE + 7)

#define HTTP_1_1_CACHE_ENTRY            0x00000040
#define STATIC_CACHE_ENTRY              0x00000080
#define MUST_REVALIDATE_CACHE_ENTRY     0x00000100
#define SHORTPATH_CACHE_ENTRY           0x00000200
#define DOWNLOAD_CACHE_ENTRY            0x00000400
#define REDIRECT_CACHE_ENTRY            0x00000800
#define COOKIE_ACCEPTED_CACHE_ENTRY     0x00001000
#define COOKIE_LEASHED_CACHE_ENTRY      0x00002000
#define COOKIE_DOWNGRADED_CACHE_ENTRY   0x00004000
#define COOKIE_REJECTED_CACHE_ENTRY     0x00008000
#define PRIVACY_MODE_CACHE_ENTRY        0x00020000
#define XDR_CACHE_ENTRY                 0x00040000
#define IMMUTABLE_CACHE_ENTRY           0x00080000
#define PENDING_DELETE_CACHE_ENTRY      0x00400000
#define OTHER_USER_CACHE_ENTRY          0x00800000
#define PRIVACY_IMPACTED_CACHE_ENTRY    0x02000000
#define POST_RESPONSE_CACHE_ENTRY       0x04000000
#define INSTALLED_CACHE_ENTRY           0x10000000
#define POST_CHECK_CACHE_ENTRY          0x20000000
#define IDENTITY_CACHE_ENTRY            0x80000000
#define ANY_CACHE_ENTRY                 0xFFFFFFFF

// We include some entry types even if app doesn't specifically ask for them.
#define INCLUDE_BY_DEFAULT_CACHE_ENTRY \
  ( HTTP_1_1_CACHE_ENTRY \
  | STATIC_CACHE_ENTRY \
  | MUST_REVALIDATE_CACHE_ENTRY \
  | PRIVACY_IMPACTED_CACHE_ENTRY \
  | POST_CHECK_CACHE_ENTRY \
  | COOKIE_ACCEPTED_CACHE_ENTRY \
  | COOKIE_LEASHED_CACHE_ENTRY \
  | COOKIE_DOWNGRADED_CACHE_ENTRY \
  | COOKIE_REJECTED_CACHE_ENTRY \
  | SHORTPATH_CACHE_ENTRY \
  | IMMUTABLE_CACHE_ENTRY \
  )

#define CACHEGROUP_FLAG_VALID               0x00000007

//
// Well known sticky group ID
//
#define CACHEGROUP_ID_BUILTIN_STICKY       0x1000000000000007

//
// INTERNET_CACHE_CONFIG_PATH_ENTRY
//

typedef struct _INTERNET_CACHE_CONFIG_PATH_ENTRYA {
    CHAR   CachePath[MAX_PATH];
    DWORD dwCacheSize;  // in KBytes
} INTERNET_CACHE_CONFIG_PATH_ENTRYA, * LPINTERNET_CACHE_CONFIG_PATH_ENTRYA;
typedef struct _INTERNET_CACHE_CONFIG_PATH_ENTRYW {
    WCHAR  CachePath[MAX_PATH];
    DWORD dwCacheSize;  // in KBytes
} INTERNET_CACHE_CONFIG_PATH_ENTRYW, * LPINTERNET_CACHE_CONFIG_PATH_ENTRYW;
#ifdef UNICODE
typedef INTERNET_CACHE_CONFIG_PATH_ENTRYW INTERNET_CACHE_CONFIG_PATH_ENTRY;
typedef LPINTERNET_CACHE_CONFIG_PATH_ENTRYW LPINTERNET_CACHE_CONFIG_PATH_ENTRY;
#else
typedef INTERNET_CACHE_CONFIG_PATH_ENTRYA INTERNET_CACHE_CONFIG_PATH_ENTRY;
typedef LPINTERNET_CACHE_CONFIG_PATH_ENTRYA LPINTERNET_CACHE_CONFIG_PATH_ENTRY;
#endif // UNICODE

//
// INTERNET_CACHE_CONFIG_INFO
//

#if _MSC_VER >= 1200
#pragma warning(push)
#endif
#pragma warning(disable:4201)

typedef struct _INTERNET_CACHE_CONFIG_INFOA {
    DWORD dwStructSize;
    DWORD dwContainer;
    DWORD dwQuota;
    DWORD dwReserved4;
    BOOL  fPerUser;
    DWORD dwSyncMode;
    DWORD dwNumCachePaths;
    union
    {
        struct
        {
            CHAR   CachePath[MAX_PATH];
            DWORD dwCacheSize;
        };
        INTERNET_CACHE_CONFIG_PATH_ENTRYA CachePaths[ANYSIZE_ARRAY];
    };
    DWORD dwNormalUsage;
    DWORD dwExemptUsage;
} INTERNET_CACHE_CONFIG_INFOA, * LPINTERNET_CACHE_CONFIG_INFOA;
typedef struct _INTERNET_CACHE_CONFIG_INFOW {
    DWORD dwStructSize;
    DWORD dwContainer;
    DWORD dwQuota;
    DWORD dwReserved4;
    BOOL  fPerUser;
    DWORD dwSyncMode;
    DWORD dwNumCachePaths;
    union
    {
        struct
        {
            WCHAR  CachePath[MAX_PATH];
            DWORD dwCacheSize;
        };
        INTERNET_CACHE_CONFIG_PATH_ENTRYW CachePaths[ANYSIZE_ARRAY];
    };
    DWORD dwNormalUsage;
    DWORD dwExemptUsage;
} INTERNET_CACHE_CONFIG_INFOW, * LPINTERNET_CACHE_CONFIG_INFOW;
#ifdef UNICODE
typedef INTERNET_CACHE_CONFIG_INFOW INTERNET_CACHE_CONFIG_INFO;
typedef LPINTERNET_CACHE_CONFIG_INFOW LPINTERNET_CACHE_CONFIG_INFO;
#else
typedef INTERNET_CACHE_CONFIG_INFOA INTERNET_CACHE_CONFIG_INFO;
typedef LPINTERNET_CACHE_CONFIG_INFOA LPINTERNET_CACHE_CONFIG_INFO;
#endif // UNICODE

#if _MSC_VER >= 1200
#pragma warning(pop)
#else
#pragma warning(default:4201)
#endif


BOOLAPI IsUrlCacheEntryExpiredA(
    _In_      LPCSTR        lpszUrlName,
    _In_      DWORD           dwFlags,
    _Inout_   FILETIME*       pftLastModified
    );
BOOLAPI IsUrlCacheEntryExpiredW(
    _In_      LPCWSTR        lpszUrlName,
    _In_      DWORD           dwFlags,
    _Inout_   FILETIME*       pftLastModified
    );
#ifdef UNICODE
#define IsUrlCacheEntryExpired  IsUrlCacheEntryExpiredW
#else
#define IsUrlCacheEntryExpired  IsUrlCacheEntryExpiredA
#endif // !UNICODE

BOOLAPI CreateUrlCacheEntryExW(
    _In_       LPCWSTR lpszUrlName,
    _In_       DWORD dwExpectedFileSize,
    _In_opt_   LPCWSTR lpszFileExtension,
    _Inout_updates_(MAX_PATH) LPWSTR lpszFileName,
    _In_       DWORD dwReserved,
    _In_       BOOL fPreserveIncomingFileName
    );

#define INTERNET_CACHE_FLAG_ALLOW_COLLISIONS     0x00000100
#define INTERNET_CACHE_FLAG_INSTALLED_ENTRY      0x00000200
#define INTERNET_CACHE_FLAG_ENTRY_OR_MAPPING     0x00000400
#define INTERNET_CACHE_FLAG_ADD_FILENAME_ONLY    0x00000800
#define INTERNET_CACHE_FLAG_GET_STRUCT_ONLY      0x00001000
#define CACHE_ENTRY_TYPE_FC         0x00001000
#define CACHE_ENTRY_MODIFY_DATA_FC  0x80000000 // this appears unused

//
// Specialized APIs for storing/retrieving binary blobs
//

URLCACHEAPI_(DWORD) GetUrlCacheEntryBinaryBlob(
    _In_ PCWSTR                                 pwszUrlName,
    _Out_ DWORD                                 *dwType,
    _Out_ FILETIME                              *pftExpireTime,
    _Out_ FILETIME                              *pftAccessTime,
    _Out_ FILETIME                              *pftModifiedTime,
    _Outptr_result_buffer_all_maybenull_(*pcbBlob) BYTE  **ppbBlob,
    _Out_ DWORD                                 *pcbBlob
);

URLCACHEAPI_(DWORD) CommitUrlCacheEntryBinaryBlob(
    _In_ PCWSTR                          pwszUrlName,
    _In_ DWORD                           dwType,
    _In_ FILETIME                        ftExpireTime,
    _In_ FILETIME                        ftModifiedTime,
    _In_reads_opt_(cbBlob) const BYTE   *pbBlob,
    _In_ DWORD                           cbBlob
);


// Flags for CreateContainer

#define INTERNET_CACHE_CONTAINER_NOSUBDIRS (0x1)
#define INTERNET_CACHE_CONTAINER_AUTODELETE (0x2)
#define INTERNET_CACHE_CONTAINER_RESERVED1 (0x4)
#define INTERNET_CACHE_CONTAINER_NODESKTOPINIT (0x8)
#define INTERNET_CACHE_CONTAINER_MAP_ENABLED (0x10)
#define INTERNET_CACHE_CONTAINER_BLOOM_FILTER (0x20)
#define INTERNET_CACHE_CONTAINER_SHARE_READ (0x100)
#define INTERNET_CACHE_CONTAINER_SHARE_READ_WRITE (0x300)

BOOLAPI CreateUrlCacheContainerA(
    _In_ LPCSTR Name,
    _In_ LPCSTR lpCachePrefix,
    _In_opt_ LPCSTR lpszCachePath,
    _In_ DWORD KBCacheLimit,
    _In_ DWORD dwContainerType,
    _In_ DWORD dwOptions,
    _Reserved_ LPVOID pvBuffer,
    _Reserved_ LPDWORD cbBuffer
    );
BOOLAPI CreateUrlCacheContainerW(
    _In_ LPCWSTR Name,
    _In_ LPCWSTR lpCachePrefix,
    _In_opt_ LPCWSTR lpszCachePath,
    _In_ DWORD KBCacheLimit,
    _In_ DWORD dwContainerType,
    _In_ DWORD dwOptions,
    _Reserved_ LPVOID pvBuffer,
    _Reserved_ LPDWORD cbBuffer
    );
#ifdef UNICODE
#define CreateUrlCacheContainer  CreateUrlCacheContainerW
#else
#define CreateUrlCacheContainer  CreateUrlCacheContainerA
#endif // !UNICODE

BOOLAPI DeleteUrlCacheContainerA(
    _In_ LPCSTR Name,
    _In_ DWORD dwOptions
    );
BOOLAPI DeleteUrlCacheContainerW(
    _In_ LPCWSTR Name,
    _In_ DWORD dwOptions
    );
#ifdef UNICODE
#define DeleteUrlCacheContainer  DeleteUrlCacheContainerW
#else
#define DeleteUrlCacheContainer  DeleteUrlCacheContainerA
#endif // !UNICODE

//
// INTERNET_CACHE_ENTRY_INFO -
//


typedef struct _INTERNET_CACHE_CONTAINER_INFOA {
    DWORD dwCacheVersion;       // version of software
    LPSTR   lpszName;             // embedded pointer to the container name string.
    LPSTR   lpszCachePrefix;      // embedded pointer to the container URL prefix
    LPSTR   lpszVolumeLabel;      // embedded pointer to the container volume label if any.
    LPSTR   lpszVolumeTitle;      // embedded pointer to the container volume title if any.
} INTERNET_CACHE_CONTAINER_INFOA, * LPINTERNET_CACHE_CONTAINER_INFOA;
typedef struct _INTERNET_CACHE_CONTAINER_INFOW {
    DWORD dwCacheVersion;       // version of software
    LPWSTR  lpszName;             // embedded pointer to the container name string.
    LPWSTR  lpszCachePrefix;      // embedded pointer to the container URL prefix
    LPWSTR  lpszVolumeLabel;      // embedded pointer to the container volume label if any.
    LPWSTR  lpszVolumeTitle;      // embedded pointer to the container volume title if any.
} INTERNET_CACHE_CONTAINER_INFOW, * LPINTERNET_CACHE_CONTAINER_INFOW;
#ifdef UNICODE
typedef INTERNET_CACHE_CONTAINER_INFOW INTERNET_CACHE_CONTAINER_INFO;
typedef LPINTERNET_CACHE_CONTAINER_INFOW LPINTERNET_CACHE_CONTAINER_INFO;
#else
typedef INTERNET_CACHE_CONTAINER_INFOA INTERNET_CACHE_CONTAINER_INFO;
typedef LPINTERNET_CACHE_CONTAINER_INFOA LPINTERNET_CACHE_CONTAINER_INFO;
#endif // UNICODE

//  FindFirstContainer options
#define CACHE_FIND_CONTAINER_RETURN_NOCHANGE (0x1)

INTERNETAPI_(HANDLE)
FindFirstUrlCacheContainerA(
    _Inout_ LPDWORD pdwModified,
    _Out_writes_bytes_(*lpcbContainerInfo) LPINTERNET_CACHE_CONTAINER_INFOA lpContainerInfo,
    _Inout_ LPDWORD lpcbContainerInfo,
    _In_ DWORD dwOptions
    );
INTERNETAPI_(HANDLE)
FindFirstUrlCacheContainerW(
    _Inout_ LPDWORD pdwModified,
    _Out_writes_bytes_(*lpcbContainerInfo) LPINTERNET_CACHE_CONTAINER_INFOW lpContainerInfo,
    _Inout_ LPDWORD lpcbContainerInfo,
    _In_ DWORD dwOptions
    );
#ifdef UNICODE
#define FindFirstUrlCacheContainer  FindFirstUrlCacheContainerW
#else
#define FindFirstUrlCacheContainer  FindFirstUrlCacheContainerA
#endif // !UNICODE

BOOLAPI
FindNextUrlCacheContainerA(
    _In_ HANDLE hEnumHandle,
    _Out_writes_bytes_(*lpcbContainerInfo) LPINTERNET_CACHE_CONTAINER_INFOA lpContainerInfo,
    _Inout_ LPDWORD lpcbContainerInfo
    );
BOOLAPI
FindNextUrlCacheContainerW(
    _In_ HANDLE hEnumHandle,
    _Out_writes_bytes_(*lpcbContainerInfo) LPINTERNET_CACHE_CONTAINER_INFOW lpContainerInfo,
    _Inout_ LPDWORD lpcbContainerInfo
    );
#ifdef UNICODE
#define FindNextUrlCacheContainer  FindNextUrlCacheContainerW
#else
#define FindNextUrlCacheContainer  FindNextUrlCacheContainerA
#endif // !UNICODE


typedef enum {
    WININET_SYNC_MODE_NEVER=0,
    WININET_SYNC_MODE_ON_EXPIRY, // bogus
    WININET_SYNC_MODE_ONCE_PER_SESSION,
    WININET_SYNC_MODE_ALWAYS,
    WININET_SYNC_MODE_AUTOMATIC,
    WININET_SYNC_MODE_DEFAULT = WININET_SYNC_MODE_AUTOMATIC
} WININET_SYNC_MODE;


BOOLAPI FreeUrlCacheSpaceA(
    _In_opt_ LPCSTR lpszCachePath,
    _In_ DWORD dwSize,
    _In_ DWORD dwFilter
    );
BOOLAPI FreeUrlCacheSpaceW(
    _In_opt_ LPCWSTR lpszCachePath,
    _In_ DWORD dwSize,
    _In_ DWORD dwFilter
    );
#ifdef UNICODE
#define FreeUrlCacheSpace  FreeUrlCacheSpaceW
#else
#define FreeUrlCacheSpace  FreeUrlCacheSpaceA
#endif // !UNICODE

DWORD UrlCacheFreeGlobalSpace(
    _In_ ULONGLONG ullTargetSize,
    _In_ DWORD dwFilter
    );

DWORD UrlCacheGetGlobalCacheSize(
    _In_ DWORD dwFilter,
    _Out_ PULONGLONG pullSize,
    _Out_ PULONGLONG pullLimit
    );


//
// default max cache limits for system Disk Space Policies
//

#define CACHE_CONFIG_DISK_SPACE_VERYLOW_IE                        26214400ULL  //  25 MB
#define CACHE_CONFIG_DISK_SPACE_LOW_IE                            52428800ULL  //  50 MB
#define CACHE_CONFIG_DISK_SPACE_BELOWNORMAL_IE                   104857600ULL  // 100 MB
#define CACHE_CONFIG_DISK_SPACE_NORMAL_IE                        346030080ULL  // 330 MB

#define CACHE_CONFIG_DISK_SPACE_VERYLOW_IE_TOTAL                  36700160ULL  //  35 MB
#define CACHE_CONFIG_DISK_SPACE_LOW_IE_TOTAL                      78643200ULL  //  75 MB
#define CACHE_CONFIG_DISK_SPACE_BELOWNORMAL_IE_TOTAL             157286400ULL  // 150 MB
#define CACHE_CONFIG_DISK_SPACE_NORMAL_IE_TOTAL                  519045120ULL  // 495 MB

#define CACHE_CONFIG_DISK_SPACE_VERYLOW_APPCONTAINER              15728640ULL  //  15 MB
#define CACHE_CONFIG_DISK_SPACE_LOW_APPCONTAINER                  26214400ULL  //  25 MB
#define CACHE_CONFIG_DISK_SPACE_BELOWNORMAL_APPCONTAINER          36700160ULL  //  35 MB
#define CACHE_CONFIG_DISK_SPACE_NORMAL_APPCONTAINER               52428800ULL  //  50 MB

#define CACHE_CONFIG_DISK_SPACE_VERYLOW_APPCONTAINER_TOTAL        36700160ULL  //   35 MB
#define CACHE_CONFIG_DISK_SPACE_LOW_APPCONTAINER_TOTAL           104857600ULL  //  100 MB
#define CACHE_CONFIG_DISK_SPACE_BELOWNORMAL_APPCONTAINER_TOTAL   262144000ULL  //  250 MB
#define CACHE_CONFIG_DISK_SPACE_NORMAL_APPCONTAINER_TOTAL       1048576000ULL  // 1000 MB


//
// config APIs.
//

#define CACHE_CONFIG_FORCE_CLEANUP_FC                                0x00000020
#define CACHE_CONFIG_DISK_CACHE_PATHS_FC                             0x00000040
#define CACHE_CONFIG_SYNC_MODE_FC                                    0x00000080
#define CACHE_CONFIG_CONTENT_PATHS_FC                                0x00000100
#define CACHE_CONFIG_COOKIES_PATHS_FC                                0x00000200
#define CACHE_CONFIG_HISTORY_PATHS_FC                                0x00000400
#define CACHE_CONFIG_QUOTA_FC                                        0x00000800
#define CACHE_CONFIG_USER_MODE_FC                                    0x00001000
#define CACHE_CONFIG_CONTENT_USAGE_FC                                0x00002000
#define CACHE_CONFIG_STICKY_CONTENT_USAGE_FC                         0x00004000

BOOLAPI
GetUrlCacheConfigInfoA(
    _Inout_ LPINTERNET_CACHE_CONFIG_INFOA lpCacheConfigInfo,
    _Reserved_ LPDWORD lpcbCacheConfigInfo,
    _In_ DWORD dwFieldControl
    );
BOOLAPI
GetUrlCacheConfigInfoW(
    _Inout_ LPINTERNET_CACHE_CONFIG_INFOW lpCacheConfigInfo,
    _Reserved_ LPDWORD lpcbCacheConfigInfo,
    _In_ DWORD dwFieldControl
    );
#ifdef UNICODE
#define GetUrlCacheConfigInfo  GetUrlCacheConfigInfoW
#else
#define GetUrlCacheConfigInfo  GetUrlCacheConfigInfoA
#endif // !UNICODE

BOOLAPI SetUrlCacheConfigInfoA(
    _In_ LPINTERNET_CACHE_CONFIG_INFOA lpCacheConfigInfo,
    _In_ DWORD dwFieldControl
    );
BOOLAPI SetUrlCacheConfigInfoW(
    _In_ LPINTERNET_CACHE_CONFIG_INFOW lpCacheConfigInfo,
    _In_ DWORD dwFieldControl
    );
#ifdef UNICODE
#define SetUrlCacheConfigInfo  SetUrlCacheConfigInfoW
#else
#define SetUrlCacheConfigInfo  SetUrlCacheConfigInfoA
#endif // !UNICODE

INTERNETAPI_(DWORD) RunOnceUrlCache(
        _In_ HWND      hwnd,
        _In_ HINSTANCE hinst,
        _In_ LPSTR     lpszCmd,
        _In_ int       nCmdShow);

INTERNETAPI_(DWORD) DeleteIE3Cache(
        _In_ HWND      hwnd,
        _In_ HINSTANCE hinst,
        _In_ LPSTR     lpszCmd,
        _In_ int       nCmdShow);

BOOLAPI UpdateUrlCacheContentPath(_In_ LPCSTR szNewPath);

// Cache header data defines.

#define CACHE_HEADER_DATA_CURRENT_SETTINGS_VERSION  0
#define CACHE_HEADER_DATA_CONLIST_CHANGE_COUNT      1
#define CACHE_HEADER_DATA_COOKIE_CHANGE_COUNT       2

#define CACHE_HEADER_DATA_NOTIFICATION_HWND         3
#define CACHE_HEADER_DATA_NOTIFICATION_MESG         4
#define CACHE_HEADER_DATA_ROOTGROUP_OFFSET          5
#define CACHE_HEADER_DATA_GID_LOW                   6
#define CACHE_HEADER_DATA_GID_HIGH                  7

#define CACHE_HEADER_DATA_LAST_SCAVENGE_TIMESTAMP   8
#define CACHE_HEADER_DATA_CACHE_READ_COUNT_SINCE_LAST_SCAVENGE 9
#define CACHE_HEADER_DATA_CACHE_WRITE_COUNT_SINCE_LAST_SCAVENGE 10
#define CACHE_HEADER_DATA_HSTS_CHANGE_COUNT         11
#define CACHE_HEADER_DATA_CACHE_RESERVED_12         12
#define CACHE_HEADER_DATA_CACHE_RESERVED_13         13

#define CACHE_HEADER_DATA_SSL_STATE_COUNT           14
// legacy alias for ssl state count
#define CACHE_HEADER_DATA_DOWNLOAD_PARTIAL CACHE_HEADER_DATA_SSL_STATE_COUNT

#define CACHE_HEADER_DATA_CACHE_RESERVED_15         15
#define CACHE_HEADER_DATA_CACHE_RESERVED_16         16
#define CACHE_HEADER_DATA_CACHE_RESERVED_17         17
#define CACHE_HEADER_DATA_CACHE_RESERVED_18         18
#define CACHE_HEADER_DATA_CACHE_RESERVED_19         19
#define CACHE_HEADER_DATA_CACHE_RESERVED_20         20

#define CACHE_HEADER_DATA_NOTIFICATION_FILTER       21
#define CACHE_HEADER_DATA_ROOT_LEAK_OFFSET          22

#define CACHE_HEADER_DATA_CACHE_RESERVED_23         23
#define CACHE_HEADER_DATA_CACHE_RESERVED_24         24
#define CACHE_HEADER_DATA_CACHE_RESERVED_25         25
#define CACHE_HEADER_DATA_CACHE_RESERVED_26         26

#define CACHE_HEADER_DATA_ROOT_GROUPLIST_OFFSET     27 // offset to group list

#define CACHE_HEADER_DATA_CACHE_RESERVED_28         28
#define CACHE_HEADER_DATA_CACHE_RESERVED_29         29
#define CACHE_HEADER_DATA_CACHE_RESERVED_30         30
#define CACHE_HEADER_DATA_CACHE_RESERVED_31         31

#define CACHE_HEADER_DATA_LAST                      31

// options for cache notification filter
#define CACHE_NOTIFY_ADD_URL                        0x00000001
#define CACHE_NOTIFY_DELETE_URL                     0x00000002
#define CACHE_NOTIFY_UPDATE_URL                     0x00000004
#define CACHE_NOTIFY_DELETE_ALL                     0x00000008
#define CACHE_NOTIFY_URL_SET_STICKY                 0x00000010
#define CACHE_NOTIFY_URL_UNSET_STICKY               0x00000020
#define CACHE_NOTIFY_SET_ONLINE                     0x00000100
#define CACHE_NOTIFY_SET_OFFLINE                    0x00000200

#define CACHE_NOTIFY_FILTER_CHANGED                 0x10000000

BOOLAPI
RegisterUrlCacheNotification(
    _In_opt_ HWND    hWnd,
    _In_       UINT    uMsg,
    _In_       GROUPID gid,
    _In_       DWORD   dwOpsFilter,
    _In_       DWORD   dwReserved
    );

BOOLAPI
GetUrlCacheHeaderData(_In_ DWORD nIdx, _Out_ LPDWORD lpdwData);

BOOLAPI
SetUrlCacheHeaderData(_In_ DWORD nIdx, _In_ DWORD dwData);

BOOLAPI
IncrementUrlCacheHeaderData(_In_ DWORD nIdx, _Out_ LPDWORD lpdwData);

BOOLAPI
LoadUrlCacheContent();

BOOLAPI
GetUrlCacheContainerInfoA(
    _In_ LPSTR lpszUrlName,
    _Out_writes_bytes_(*lpcbContainerInfo) LPINTERNET_CACHE_CONTAINER_INFOA lpContainerInfo,
    _Inout_ LPDWORD lpcbContainerInfo,
    _In_ DWORD dwOptions
    );
BOOLAPI
GetUrlCacheContainerInfoW(
    _In_ LPWSTR lpszUrlName,
    _Out_writes_bytes_(*lpcbContainerInfo) LPINTERNET_CACHE_CONTAINER_INFOW lpContainerInfo,
    _Inout_ LPDWORD lpcbContainerInfo,
    _In_ DWORD dwOptions
    );
#ifdef UNICODE
#define GetUrlCacheContainerInfo  GetUrlCacheContainerInfoW
#else
#define GetUrlCacheContainerInfo  GetUrlCacheContainerInfoA
#endif // !UNICODE

#ifdef _WINSOCK2API_
INTERNETAPI_(DWORD)
HttpGetTunnelSocket(
    _In_ HINTERNET hRequest,
    _Out_ SOCKET *pSocket,
    _Outptr_result_buffer_all_maybenull_(*pdwDataLength) PBYTE *ppbData,
    _Out_ PDWORD pdwDataLength
    );
#endif

//
// AppCache
//

typedef PVOID APP_CACHE_HANDLE;

#define APP_CACHE_LOOKUP_NO_MASTER_ONLY 0x1

INTERNETAPI_(DWORD)
AppCacheLookup(
    _In_ PCWSTR pwszUrl,
    _In_ DWORD dwFlags,
    _Out_ APP_CACHE_HANDLE *phAppCache
    );

typedef enum _APP_CACHE_STATE
{
    AppCacheStateNoUpdateNeeded,
    AppCacheStateUpdateNeeded,
    AppCacheStateUpdateNeededNew,
    AppCacheStateUpdateNeededMasterOnly
} APP_CACHE_STATE;

INTERNETAPI_(DWORD)
AppCacheCheckManifest(
    _In_opt_ PCWSTR pwszMasterUrl,
    _In_ PCWSTR pwszManifestUrl,
    _In_reads_bytes_(dwManifestDataSize) const BYTE *pbManifestData,
    _In_ DWORD dwManifestDataSize,
    _In_reads_bytes_(dwManifestResponseHeadersSize) const BYTE *pbManifestResponseHeaders,
    _In_ DWORD dwManifestResponseHeadersSize,
    _Out_ APP_CACHE_STATE *peState,
    _Out_ APP_CACHE_HANDLE *phNewAppCache
    );

#define APP_CACHE_ENTRY_TYPE_MASTER   0x01
#define APP_CACHE_ENTRY_TYPE_EXPLICIT 0x02
#define APP_CACHE_ENTRY_TYPE_FALLBACK 0x04
#define APP_CACHE_ENTRY_TYPE_FOREIGN  0x08
#define APP_CACHE_ENTRY_TYPE_MANIFEST 0x10

typedef struct _APP_CACHE_DOWNLOAD_ENTRY
{
    PWSTR pwszUrl;
    DWORD dwEntryType;
} APP_CACHE_DOWNLOAD_ENTRY;

typedef struct _APP_CACHE_DOWNLOAD_LIST
{
    DWORD dwEntryCount;
    _Field_size_(dwEntryCount) APP_CACHE_DOWNLOAD_ENTRY *pEntries;
} APP_CACHE_DOWNLOAD_LIST;

INTERNETAPI_(DWORD)
AppCacheGetDownloadList(
    _In_ APP_CACHE_HANDLE hAppCache,
    _Out_ APP_CACHE_DOWNLOAD_LIST *pDownloadList
    );

INTERNETAPI_(VOID)
AppCacheFreeDownloadList(
    _Inout_ APP_CACHE_DOWNLOAD_LIST *pDownloadList
    );

typedef enum _APP_CACHE_FINALIZE_STATE
{
    AppCacheFinalizeStateIncomplete,
    AppCacheFinalizeStateManifestChange,
    AppCacheFinalizeStateComplete
} APP_CACHE_FINALIZE_STATE;

INTERNETAPI_(DWORD)
AppCacheFinalize(
    _In_ APP_CACHE_HANDLE hAppCache,
    _In_reads_bytes_(dwManifestDataSize) const BYTE *pbManifestData,
    _In_ DWORD dwManifestDataSize,
    _Out_ APP_CACHE_FINALIZE_STATE *peState
    );

INTERNETAPI_(DWORD)
AppCacheGetFallbackUrl(
    _In_ APP_CACHE_HANDLE hAppCache,
    _In_ PCWSTR pwszUrl,
    _Outptr_result_z_ PWSTR *ppwszFallbackUrl
    );

INTERNETAPI_(DWORD)
AppCacheGetManifestUrl(
    _In_ APP_CACHE_HANDLE hAppCache,
    _Outptr_result_z_ PWSTR *ppwszManifestUrl
    );

INTERNETAPI_(DWORD)
AppCacheDuplicateHandle(
    _In_ APP_CACHE_HANDLE hAppCache,
    _Outptr_ APP_CACHE_HANDLE *phDuplicatedAppCache
    );

INTERNETAPI_(VOID)
AppCacheCloseHandle(
    _In_ APP_CACHE_HANDLE hAppCache
    );

typedef struct _APP_CACHE_GROUP_INFO
{
    PWSTR pwszManifestUrl;
    FILETIME ftLastAccessTime;
    ULONGLONG ullSize;
} APP_CACHE_GROUP_INFO;

typedef struct _APP_CACHE_GROUP_LIST
{
    DWORD dwAppCacheGroupCount;
    APP_CACHE_GROUP_INFO *pAppCacheGroups;
} APP_CACHE_GROUP_LIST;

INTERNETAPI_(VOID)
AppCacheFreeGroupList(
    _Inout_ APP_CACHE_GROUP_LIST *pAppCacheGroupList
    );

INTERNETAPI_(DWORD)
AppCacheGetGroupList(
    _Out_ APP_CACHE_GROUP_LIST *pAppCacheGroupList
    );

INTERNETAPI_(DWORD)
AppCacheGetInfo(
    _In_ APP_CACHE_HANDLE hAppCache,
    _Out_ APP_CACHE_GROUP_INFO *pAppCacheInfo
    );

INTERNETAPI_(DWORD)
AppCacheDeleteGroup(
    _In_ PCWSTR pwszManifestUrl
   );

INTERNETAPI_(DWORD)
AppCacheFreeSpace(
    _In_ FILETIME ftCutOff
    );

INTERNETAPI_(DWORD)
AppCacheGetIEGroupList(
    _Out_ APP_CACHE_GROUP_LIST *pAppCacheGroupList
    );

INTERNETAPI_(DWORD)
AppCacheDeleteIEGroup(
    _In_ PCWSTR pwszManifestUrl
    );

INTERNETAPI_(DWORD)
AppCacheFreeIESpace(
    _In_ FILETIME ftCutOff
    );

INTERNETAPI_(DWORD)
AppCacheCreateAndCommitFile(
    _In_ APP_CACHE_HANDLE hAppCache,
    _In_ PCWSTR pwszSourceFilePath,
    _In_ PCWSTR pwszUrl,
    _In_reads_bytes_(dwResponseHeadersSize) const BYTE *pbResponseHeaders,
    _In_ DWORD dwResponseHeadersSize
    );

//
//  Dependency Table
//

typedef PVOID HTTP_DEPENDENCY_HANDLE;

INTERNETAPI_(DWORD)
HttpOpenDependencyHandle(
    _In_ HINTERNET hRequestHandle,
    _In_ BOOL fBackground,
    _Outptr_ HTTP_DEPENDENCY_HANDLE *phDependencyHandle
);

INTERNETAPI_(VOID)
HttpCloseDependencyHandle(
    _In_ HTTP_DEPENDENCY_HANDLE hDependencyHandle
);

INTERNETAPI_(DWORD)
HttpDuplicateDependencyHandle(
    _In_ HTTP_DEPENDENCY_HANDLE hDependencyHandle,
    _Outptr_ HTTP_DEPENDENCY_HANDLE *phDuplicatedDependencyHandle
);

INTERNETAPI_(DWORD)
HttpIndicatePageLoadComplete(
    _In_ HTTP_DEPENDENCY_HANDLE hDependencyHandle
);

//
// UrlCache
//

typedef PVOID URLCACHE_HANDLE;

typedef struct _URLCACHE_ENTRY_INFO {
  PWSTR pwszSourceUrlName;
  PWSTR pwszLocalFileName;
  DWORD dwCacheEntryType;
  DWORD dwUseCount;
  DWORD dwHitRate;
  DWORD dwSizeLow;
  DWORD dwSizeHigh;
  FILETIME ftLastModifiedTime;
  FILETIME ftExpireTime;
  FILETIME ftLastAccessTime;
  FILETIME ftLastSyncTime;
  PBYTE pbHeaderInfo;
  DWORD cbHeaderInfoSize;
  PBYTE pbExtraData;
  DWORD cbExtraDataSize;
} URLCACHE_ENTRY_INFO, *PURLCACHE_ENTRY_INFO;

URLCACHEAPI_(VOID)
UrlCacheFreeEntryInfo(
    _Inout_ PURLCACHE_ENTRY_INFO pCacheEntryInfo
    );

URLCACHEAPI
UrlCacheGetEntryInfo(
    _In_opt_ APP_CACHE_HANDLE hAppCache,
    _In_ PCWSTR pcwszUrl,
    _Out_opt_ PURLCACHE_ENTRY_INFO pCacheEntryInfo
    );

URLCACHEAPI_(VOID)
UrlCacheCloseEntryHandle(
    _In_ URLCACHE_HANDLE hEntryFile
    );

URLCACHEAPI
UrlCacheRetrieveEntryFile(
    _In_opt_ APP_CACHE_HANDLE hAppCache,
    _In_ PCWSTR pcwszUrl,
    _Out_ PURLCACHE_ENTRY_INFO pCacheEntryInfo,
    _Out_ URLCACHE_HANDLE *phEntryFile
    );

URLCACHEAPI
UrlCacheReadEntryStream(
    _In_ URLCACHE_HANDLE hUrlCacheStream,
    _In_ ULONGLONG ullLocation,
    _Inout_ PVOID pBuffer,
    _In_ DWORD dwBufferLen,
    _Out_ PDWORD pdwBufferLen
    );

URLCACHEAPI
UrlCacheRetrieveEntryStream(
    _In_opt_ APP_CACHE_HANDLE hAppCache,
    _In_ PCWSTR pcwszUrl,
    _In_ BOOL fRandomRead,
    _Out_ PURLCACHE_ENTRY_INFO pCacheEntryInfo,
    _Out_ URLCACHE_HANDLE *phEntryStream
    );

URLCACHEAPI
UrlCacheUpdateEntryExtraData(
    _In_opt_ APP_CACHE_HANDLE hAppCache,
    _In_ PCWSTR pcwszUrl,
    _In_reads_bytes_(cbExtraData) const BYTE *pbExtraData,
    _In_ DWORD cbExtraData
    );

URLCACHEAPI
UrlCacheCreateContainer(
    _In_z_ const WCHAR *pwszName,
    _In_z_ const WCHAR *pwszPrefix,
    _In_z_ const WCHAR *pwszDirectory,
    _In_ ULONGLONG ullLimit,
    _In_ DWORD dwOptions
    );

URLCACHEAPI
UrlCacheCheckEntriesExist(
    _In_reads_(cEntries) PCWSTR *rgpwszUrls,
    _In_ DWORD cEntries,
    _Out_writes_(cEntries) BOOL *rgfExist
    );

URLCACHEAPI
UrlCacheGetContentPaths(
    _Outptr_result_buffer_(*pcDirectories) PWSTR **pppwszDirectories,
    _Out_ DWORD *pcDirectories
    );

typedef enum _URL_CACHE_LIMIT_TYPE
{
    UrlCacheLimitTypeIE = 0,
    UrlCacheLimitTypeIETotal,
    UrlCacheLimitTypeAppContainer,
    UrlCacheLimitTypeAppContainerTotal,
    UrlCacheLimitTypeNum
} URL_CACHE_LIMIT_TYPE;

URLCACHEAPI
UrlCacheGetGlobalLimit(
    _In_ URL_CACHE_LIMIT_TYPE            limitType,
    _Out_ ULONGLONG                      *pullLimit
);

URLCACHEAPI
UrlCacheSetGlobalLimit(
    _In_ URL_CACHE_LIMIT_TYPE            limitType,
    _In_ ULONGLONG                       ullLimit
);

URLCACHEAPI
UrlCacheReloadSettings(
);

URLCACHEAPI
UrlCacheContainerSetEntryMaximumAge(
    _In_z_ const WCHAR                   *pwszPrefix,
    _In_   DWORD                         dwEntryMaxAge
);

URLCACHEAPI
UrlCacheFindFirstEntry(
    _In_opt_z_ const WCHAR *pwszPrefix,
    _In_ DWORD dwFlags,
    _In_ DWORD dwFilter,
    _In_ GROUPID GroupId,
    _Out_ PURLCACHE_ENTRY_INFO pCacheEntryInfo,
    _Out_ HANDLE *phFind
);

URLCACHEAPI
UrlCacheFindNextEntry(
    _In_ HANDLE hFind,
    _Out_ PURLCACHE_ENTRY_INFO pCacheEntryInfo
);

URLCACHEAPI
UrlCacheServer(
);

#define CACHE_CONFIG_CONTENT_QUOTA_FC                                0x00008000
#define CACHE_CONFIG_TOTAL_CONTENT_QUOTA_FC                          0x00010000
#define CACHE_CONFIG_APPCONTAINER_CONTENT_QUOTA_FC                   0x00020000
#define CACHE_CONFIG_APPCONTAINER_TOTAL_CONTENT_QUOTA_FC             0x00040000

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


INTERNETAPI
InternetGetDialEngineW(
    _In_ LPWSTR               pwzConnectoid,
    _In_ IDialEventSink *     pdes,
    _Out_ IDialEngine **      ppde
    );

INTERNETAPI
InternetGetDialBrandingW(
    _In_ LPWSTR               pwzConnectoid,
    _Out_ IDialBranding **    ppdb
    );


BOOLAPI
ReadGuidsForConnectedNetworks(
    _Out_opt_ DWORD *pcNetworks,
    _Out_opt_ PWSTR **pppwszNetworkGuids,
    _Out_opt_ BSTR  **pppbstrNetworkNames,
    _Out_opt_ PWSTR **pppwszGWMacs,
    _Out_opt_ DWORD *pcGatewayMacs,
    _Out_opt_ DWORD *pdwFlags
    );

//
// INTERNET_AUTOPROXY_INIT_FLAGS - Flags for InternetInitializeAutoProxyDll
//
#define INTERNET_AUTOPROXY_INIT_DEFAULT 0x1
#define INTERNET_AUTOPROXY_INIT_DOWNLOADSYNC 0x2
#define INTERNET_AUTOPROXY_INIT_QUERYSTATE 0x4
#define INTERNET_AUTOPROXY_INIT_ONLYQUERY 0x8

#define INTERNET_AUTOPROXY_INIT_MASK (INTERNET_AUTOPROXY_INIT_DEFAULT|INTERNET_AUTOPROXY_INIT_DOWNLOADSYNC|INTERNET_AUTOPROXY_INIT_QUERYSTATE|INTERNET_AUTOPROXY_INIT_ONLYQUERY)


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

// Registry entries used by the dialing code
// All of these entries are in:
// HKCU\software\microsoft\windows\current version\internet settings

#define REGSTR_DIAL_AUTOCONNECT     "AutoConnect"

// Registry entries for legacy cookies
#define REGSTR_LEASH_LEGACY_COOKIES "LeashLegacyCookies"



// Used by security manager.

BOOLAPI IsHostInProxyBypassList(
    _In_ INTERNET_SCHEME tScheme,
    _In_reads_(cchHost) LPCSTR lpszHost,
    _In_ DWORD cchHost);

typedef struct _WININET_PROXY_INFO
{
    BOOL            fProxy;                // Is this a proxy or DIRECT?
    BOOL            fBypass;               // If DIRECT, is it bypassing a proxy (intranet) or is all traffic DIRECT (internet)
    INTERNET_SCHEME ProxyScheme;           // The scheme of the proxy, SOCKS, HTTP (CERN Proxy), HTTPS (SSL through Proxy)
    PWSTR           pwszProxy;             // Hostname of the proxy.
    INTERNET_PORT   ProxyPort;             // Port of the proxy.
} WININET_PROXY_INFO;

typedef struct _WININET_PROXY_INFO_LIST
{
    DWORD dwProxyInfoCount;
    WININET_PROXY_INFO *pProxyInfo;
} WININET_PROXY_INFO_LIST;

INTERNETAPI_(VOID)
InternetFreeProxyInfoList(
    _Inout_ WININET_PROXY_INFO_LIST *pProxyInfoList
);

INTERNETAPI_(DWORD)
InternetGetProxyForUrl(
    _In_ HINTERNET hInternet,
    _In_ PCWSTR pcwszUrl,
    _Out_ WININET_PROXY_INFO_LIST *pProxyInfoList
);

// Used by Shell to determine if anyone has loaded wininet yet
// Shell code calls OpenMutex with this name and if no mutex is
// obtained, we know that no copy of wininet has been loaded yet
#if _WIN32_WINNT >= _WIN32_WINNT_WINXP
#define LOCAL_NAMESPACE_PREFIX            "Local\\"
#define LOCAL_NAMESPACE_PREFIX_W          L"Local\\"
#else
#define LOCAL_NAMESPACE_PREFIX
#define LOCAL_NAMESPACE_PREFIX_W
#endif
#define WININET_STARTUP_MUTEX   LOCAL_NAMESPACE_PREFIX   "WininetStartupMutex"
#define WININET_STARTUP_MUTEX_W LOCAL_NAMESPACE_PREFIX_W L"WininetStartupMutex"


BOOL DoConnectoidsExist(void); // Returns TRUE if any RAS connectoids exist and FALSE otherwise

BOOLAPI GetDiskInfoA(
    _In_   PCSTR pszPath,
    _Out_opt_ PDWORD pdwClusterSize,
    _Out_opt_ PDWORDLONG pdlAvail,
    _Out_opt_ PDWORDLONG pdlTotal);

typedef BOOL (*CACHE_OPERATOR)(INTERNET_CACHE_ENTRY_INFO* pcei, PDWORD pcbcei, PVOID pOpData);

BOOL PerformOperationOverUrlCacheA(
    _In_opt_    PCSTR     pszUrlSearchPattern,
    _In_          DWORD     dwFlags,
    _In_          DWORD     dwFilter,
    _In_          GROUPID   GroupId,
    _Reserved_    PVOID     pReserved1,
    _Reserved_    PDWORD    pdwReserved2,
    _Reserved_    PVOID     pReserved3,
    _In_          CACHE_OPERATOR op,
    _Inout_       PVOID     pOperatorData
    );

BOOL IsProfilesEnabled();

INTERNETAPI_(DWORD) _GetFileExtensionFromUrl(
    _In_ LPSTR lpszUrl,
    _In_ DWORD dwFlags,
    _Inout_updates_bytes_(*pcchExt) LPSTR lpszExt,
    _Inout_ DWORD *pcchExt
);

INTERNETAPI_(DWORD) InternalInternetGetCookie(
    _In_ LPCSTR lpszUrl,
    _Out_writes_(*lpdwDataSize) LPSTR lpszCookieData,
    _Inout_ DWORD *lpdwDataSize
);


//  in cookimp.cxx and cookexp.cxx
BOOLAPI ImportCookieFileA(
    _In_ LPCSTR szFilename
);
//  in cookimp.cxx and cookexp.cxx
BOOLAPI ImportCookieFileW(
    _In_ LPCWSTR szFilename
);
#ifdef UNICODE
#define ImportCookieFile  ImportCookieFileW
#else
#define ImportCookieFile  ImportCookieFileA
#endif // !UNICODE
BOOLAPI ExportCookieFileA(
    _In_ LPCSTR szFilename,
    _In_ BOOL fAppend
);
BOOLAPI ExportCookieFileW(
    _In_ LPCWSTR szFilename,
    _In_ BOOL fAppend
);
#ifdef UNICODE
#define ExportCookieFile  ExportCookieFileW
#else
#define ExportCookieFile  ExportCookieFileA
#endif // !UNICODE

BOOLAPI IsDomainLegalCookieDomainA(    // in "wininet\http\cookie.cxx"
    _In_ LPCSTR pchDomain,
    _In_ LPCSTR pchFullDomain
);
BOOLAPI IsDomainLegalCookieDomainW(    // in "wininet\http\cookie.cxx"
    _In_ LPCWSTR pchDomain,
    _In_ LPCWSTR pchFullDomain
);
#ifdef UNICODE
#define IsDomainLegalCookieDomain  IsDomainLegalCookieDomainW
#else
#define IsDomainLegalCookieDomain  IsDomainLegalCookieDomainA
#endif // !UNICODE


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

#define INTERNET_SUPPRESS_COOKIE_PERSIST            0x03
#define INTERNET_SUPPRESS_COOKIE_PERSIST_RESET      0x04
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

typedef enum _HTTP_WEB_SOCKET_OPERATION
{
    HTTP_WEB_SOCKET_SEND_OPERATION                      = 0,
    HTTP_WEB_SOCKET_RECEIVE_OPERATION                   = 1,
    HTTP_WEB_SOCKET_CLOSE_OPERATION                     = 2,
    HTTP_WEB_SOCKET_SHUTDOWN_OPERATION                  = 3
} HTTP_WEB_SOCKET_OPERATION;

typedef enum _HTTP_WEB_SOCKET_BUFFER_TYPE
{
    HTTP_WEB_SOCKET_BINARY_MESSAGE_TYPE                 = 0,
    HTTP_WEB_SOCKET_BINARY_FRAGMENT_TYPE                = 1,
    HTTP_WEB_SOCKET_UTF8_MESSAGE_TYPE                   = 2,
    HTTP_WEB_SOCKET_UTF8_FRAGMENT_TYPE                  = 3,
    HTTP_WEB_SOCKET_CLOSE_TYPE                          = 4,
    HTTP_WEB_SOCKET_PING_TYPE                           = 5
} HTTP_WEB_SOCKET_BUFFER_TYPE;

typedef enum _HTTP_WEB_SOCKET_CLOSE_STATUS
{
    HTTP_WEB_SOCKET_SUCCESS_CLOSE_STATUS                = 1000,
    HTTP_WEB_SOCKET_ENDPOINT_TERMINATED_CLOSE_STATUS    = 1001,
    HTTP_WEB_SOCKET_PROTOCOL_ERROR_CLOSE_STATUS         = 1002,
    HTTP_WEB_SOCKET_INVALID_DATA_TYPE_CLOSE_STATUS      = 1003,
    HTTP_WEB_SOCKET_EMPTY_CLOSE_STATUS                  = 1005,
    HTTP_WEB_SOCKET_ABORTED_CLOSE_STATUS                = 1006,
    HTTP_WEB_SOCKET_INVALID_PAYLOAD_CLOSE_STATUS        = 1007,
    HTTP_WEB_SOCKET_POLICY_VIOLATION_CLOSE_STATUS       = 1008,
    HTTP_WEB_SOCKET_MESSAGE_TOO_BIG_CLOSE_STATUS        = 1009,
    HTTP_WEB_SOCKET_UNSUPPORTED_EXTENSIONS_CLOSE_STATUS = 1010,
    HTTP_WEB_SOCKET_SERVER_ERROR_CLOSE_STATUS           = 1011,
    HTTP_WEB_SOCKET_SECURE_HANDSHAKE_ERROR_CLOSE_STATUS = 1015
} HTTP_WEB_SOCKET_CLOSE_STATUS;

typedef struct _HTTP_WEB_SOCKET_ASYNC_RESULT
{
    INTERNET_ASYNC_RESULT AsyncResult;
    HTTP_WEB_SOCKET_OPERATION Operation;
    HTTP_WEB_SOCKET_BUFFER_TYPE BufferType;
    DWORD dwBytesTransferred;
} HTTP_WEB_SOCKET_ASYNC_RESULT;

#define HTTP_WEB_SOCKET_MAX_CLOSE_REASON_LENGTH 123
#define HTTP_WEB_SOCKET_MIN_KEEPALIVE_VALUE 10000

INTERNETAPI_(HINTERNET) HttpWebSocketCompleteUpgrade(
    _In_ HINTERNET hRequest,
    _In_ DWORD_PTR dwContext);

BOOLAPI HttpWebSocketSend(
    _In_ HINTERNET hWebSocket,
    _In_ HTTP_WEB_SOCKET_BUFFER_TYPE BufferType,
    _In_reads_bytes_opt_(dwBufferLength) PVOID pvBuffer,
    _In_ DWORD dwBufferLength);

BOOLAPI HttpWebSocketReceive(
    _In_ HINTERNET hWebSocket,
    _Out_writes_bytes_to_(dwBufferLength, *pdwBytesRead) PVOID pvBuffer,
    _In_ DWORD dwBufferLength,
    _Out_range_(0, dwBufferLength) DWORD *pdwBytesRead,
    _Out_ HTTP_WEB_SOCKET_BUFFER_TYPE *pBufferType);

BOOLAPI HttpWebSocketClose(
    _In_ HINTERNET hWebSocket,
    _In_ USHORT usStatus,
    _In_reads_bytes_opt_(dwReasonLength) PVOID pvReason,
    _In_range_(0, HTTP_WEB_SOCKET_MAX_CLOSE_REASON_LENGTH) DWORD dwReasonLength);

BOOLAPI HttpWebSocketShutdown(
    _In_ HINTERNET hWebSocket,
    _In_ USHORT usStatus,
    _In_reads_bytes_opt_(dwReasonLength) PVOID pvReason,
    _In_range_(0, HTTP_WEB_SOCKET_MAX_CLOSE_REASON_LENGTH) DWORD dwReasonLength);

BOOLAPI HttpWebSocketQueryCloseStatus(
    _In_ HINTERNET hWebSocket,
    _Out_ USHORT *pusStatus,
    _Out_writes_bytes_to_opt_(dwReasonLength, *pdwReasonLengthConsumed) PVOID pvReason,
    _In_range_(0, HTTP_WEB_SOCKET_MAX_CLOSE_REASON_LENGTH) DWORD dwReasonLength,
    _Out_range_(0, HTTP_WEB_SOCKET_MAX_CLOSE_REASON_LENGTH) DWORD *pdwReasonLengthConsumed);


#ifdef _WS2DEF_

//
// callback function for INTERNET_OPTION_GLOBAL_CALLBACK option
//

#define INTERNET_GLOBAL_CALLBACK_SENDING_HTTP_HEADERS          0x00000001
#define INTERNET_GLOBAL_CALLBACK_DETECTING_PROXY               0x00000002

typedef
DWORD
(CALLBACK * INTERNET_GLOBAL_NOTIFICATION_CALLBACK)(
    _In_ HINTERNET hInternet,
    _In_opt_ PVOID pvContext,
    _In_ PCSTR pcszUrl,
    _In_ PCSTR pcszHost,
    _In_ PSOCKADDR_STORAGE pRemoteAddress,
    _In_ DWORD dwNotification,
    _In_opt_ PVOID pvEventData,
    _In_ DWORD dwEventDataLength
);

typedef struct _INTERNET_GLOBAL_CALLBACK
{
    INTERNET_GLOBAL_NOTIFICATION_CALLBACK pfnGlobalNotificationCallback;
    DWORD dwNotifications;
    PVOID pvContext;
    ULONGLONG ullPriority;
    GUID guidRegistrationId;
} INTERNET_GLOBAL_CALLBACK;

#endif

STDAPI_(DWORD)
InternetConvertUrlFromWireToWideChar(
    _In_reads_(cchUrl) PCSTR pcszUrl,
    _In_ DWORD cchUrl,
    _In_ PCWSTR pcwszBaseUrl,
    _In_ DWORD dwCodePageHost,
    _In_ DWORD dwCodePagePath,
    _In_ BOOL fEncodePathExtra,
    _In_ DWORD dwCodePageExtra,
    _Outptr_result_z_ PWSTR *ppwszConvertedUrl
);

STDAPI_(DWORD)
HttpPreConnect(
    _In_ HANDLE hRequest,
    _In_ PCWSTR pwszUrl,
    _In_ DWORD cConnections
);

#ifndef _HTTP_POLICY_EXTENSION_
#define _HTTP_POLICY_EXTENSION_

typedef enum
{
    POLICY_EXTENSION_TYPE_NONE = 0,
    POLICY_EXTENSION_TYPE_WINHTTP = 1,
    POLICY_EXTENSION_TYPE_WININET = 2
} HTTP_POLICY_EXTENSION_TYPE;

typedef enum
{
    POLICY_EXTENSION_VERSION1 = 1
}  HTTP_POLICY_EXTENSION_VERSION;

typedef DWORD
(WINAPI * HTTP_POLICY_EXTENSION_INIT)(_In_ HTTP_POLICY_EXTENSION_VERSION Version,
                                      _In_ HTTP_POLICY_EXTENSION_TYPE Type,
                                      _In_ VOID* pvData,
                                      _In_ ULONG cbData);

typedef DWORD
(WINAPI * HTTP_POLICY_EXTENSION_SHUTDOWN)(_In_ HTTP_POLICY_EXTENSION_TYPE Type);

#endif


#if defined(__cplusplus)
}
#endif

#endif // !define(_WININETEX_)
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
