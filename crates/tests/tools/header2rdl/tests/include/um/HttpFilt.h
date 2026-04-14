/*++

Copyright (c) 1997-1999  Microsoft Corporation

Module Name:

    httpfilt.h

Abstract:

    This module contains the Microsoft HTTP filter extension info

Revision History:

--*/

#ifndef _HTTPFILT_H_
#define _HTTPFILT_H_
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#ifdef __cplusplus
extern "C" {
#endif

//
// Define ULONG_PTR if necessary
//

#ifndef _W64
#if !defined(__midl) && (defined(_X86_) || defined(_ARM_)) && _MSC_VER >= 1300
#define _W64 __w64
#else
#define _W64
#endif
#endif

//
// The INT_PTR is guaranteed to be the same size as a pointer.  Its
// size with change with pointer size (32/64).  It should be used
// anywhere that a pointer is cast to an integer type. UINT_PTR is
// the unsigned variation.
//
// __int3264 is intrinsic to 64b MIDL but not to old MIDL or to C compiler.
//
#if ( 501 < __midl )

    typedef unsigned __int3264 ULONG_PTR, *PULONG_PTR;

#else  // midl64
// old midl and C++ compiler

#if defined(_WIN64)
    typedef unsigned __int64 ULONG_PTR, *PULONG_PTR;
#else
    typedef _W64 unsigned long ULONG_PTR, *PULONG_PTR;
#endif
#endif // midl64


//
//  Current version of the filter spec is 10.0
//

#define HTTP_FILTER_REVISION    MAKELONG( 0, 10 )

#define SF_MAX_USERNAME         (256+1)
#define SF_MAX_PASSWORD         (256+1)
#define SF_MAX_AUTH_TYPE        (32+1)

#define SF_MAX_FILTER_DESC_LEN  (256+1)


//
//  These values can be used with the pfnSFCallback function supplied in
//  the filter context structure
//

enum SF_REQ_TYPE
{
    //
    //  Sends a complete HTTP server response header including
    //  the status, server version, message time and MIME version.
    //
    //  Server extensions should append other information at the end,
    //  such as Content-type, Content-length etc followed by an extra
    //  '\r\n'.
    //
    //  pData - Zero terminated string pointing to optional
    //      status string (i.e., "401 Access Denied") or NULL for
    //      the default response of "200 OK".
    //
    //  ul1 - Zero terminated string pointing to optional data to be
    //      appended and set with the header.  If NULL, the header will
    //      be terminated with an empty line.
    //

    SF_REQ_SEND_RESPONSE_HEADER,

    //
    //  If the server denies the HTTP request, add the specified headers
    //  to the server error response.
    //
    //  This allows an authentication filter to advertise its services
    //  w/o filtering every request.  Generally the headers will be
    //  WWW-Authenticate headers with custom authentication schemes but
    //  no restriction is placed on what headers may be specified.
    //
    //  pData - Zero terminated string pointing to one or more header lines
    //      with terminating '\r\n'.
    //

    SF_REQ_ADD_HEADERS_ON_DENIAL,

    //
    //  Only used by raw data filters that return SF_STATUS_READ_NEXT
    //
    //  ul1 - size in bytes for the next read
    //

    SF_REQ_SET_NEXT_READ_SIZE,

    //
    //  Used to indicate this request is a proxy request
    //
    //  ul1 - The proxy flags to set
    //      0x00000001 - This is a HTTP proxy request
    //
    //

    SF_REQ_SET_PROXY_INFO,

    //
    //  Returns the connection ID contained in the ConnID field of an
    //  ISAPI Application's Extension Control Block.  This value can be used
    //  as a key to cooridinate shared data between Filters and Applications.
    //
    //  pData - Pointer to DWORD that receives the connection ID.
    //

    SF_REQ_GET_CONNID,

    //
    // Used to set a SSPI security context + impersonation token
    // derived from a client certificate.
    //
    // pData - certificate info ( PHTTP_FILTER_CERTIFICATE_INFO )
    // ul1 - CtxtHandle*
    // ul2 - impersonation handle
    //

    SF_REQ_SET_CERTIFICATE_INFO,

    //
    // Used to get an IIS property
    // as defined in SF_PROPERTY_IIS
    //
    // ul1 - Property ID
    //

    SF_REQ_GET_PROPERTY,

    //
    // Used to normalize an URL
    //
    // pData - URL to normalize
    //

    SF_REQ_NORMALIZE_URL,

    //
    // Disable Notifications
    //
    // ul1 - notifications to disable
    //

    SF_REQ_DISABLE_NOTIFICATIONS,

} ;


enum SF_PROPERTY_IIS
{
    SF_PROPERTY_SSL_CTXT,
    SF_PROPERTY_INSTANCE_NUM_ID
} ;


//
//  These values are returned by the filter entry point when a new request is
//  received indicating their interest in this particular request
//

enum SF_STATUS_TYPE
{
    //
    //  The filter has handled the HTTP request.  The server should disconnect
    //  the session.
    //

    SF_STATUS_REQ_FINISHED = 0x8000000,

    //
    //  Same as SF_STATUS_FINISHED except the server should keep the TCP
    //  session open if the option was negotiated
    //

    SF_STATUS_REQ_FINISHED_KEEP_CONN,

    //
    //  The next filter in the notification chain should be called
    //

    SF_STATUS_REQ_NEXT_NOTIFICATION,

    //
    //  This filter handled the notification.  No other handles should be
    //  called for this particular notification type
    //

    SF_STATUS_REQ_HANDLED_NOTIFICATION,

    //
    //  An error occurred.  The server should use GetLastError() and indicate
    //  the error to the client
    //

    SF_STATUS_REQ_ERROR,

    //
    //  The filter is an opaque stream filter and we're negotiating the
    //  session parameters.  Only valid for raw read notification.
    //

    SF_STATUS_REQ_READ_NEXT
};

//
//  pvNotification points to this structure for all request notification types
//

typedef struct _HTTP_FILTER_CONTEXT
{
    DWORD          cbSize;

    //
    //  This is the structure revision level.
    //

    DWORD          Revision;

    //
    //  Private context information for the server.
    //

    PVOID          ServerContext;
    DWORD          ulReserved;

    //
    //  TRUE if this request is coming over a secure port
    //

    BOOL           fIsSecurePort;

    //
    //  A context that can be used by the filter
    //

    PVOID          pFilterContext;

    //
    //  Server callbacks
    //

    BOOL (WINAPI * GetServerVariable) (
        struct _HTTP_FILTER_CONTEXT * pfc,
        LPSTR                         lpszVariableName,
        LPVOID                        lpvBuffer,
        LPDWORD                       lpdwSize
        );

    BOOL (WINAPI * AddResponseHeaders) (
        struct _HTTP_FILTER_CONTEXT * pfc,
        LPSTR                         lpszHeaders,
        DWORD                         dwReserved
        );

    BOOL (WINAPI * WriteClient)  (
        struct _HTTP_FILTER_CONTEXT * pfc,
        LPVOID                        Buffer,
        LPDWORD                       lpdwBytes,
        DWORD                         dwReserved
        );

    VOID * (WINAPI * AllocMem) (
        struct _HTTP_FILTER_CONTEXT * pfc,
        DWORD                         cbSize,
        DWORD                         dwReserved
        );

    BOOL (WINAPI * ServerSupportFunction) (
        struct _HTTP_FILTER_CONTEXT * pfc,
        enum SF_REQ_TYPE              sfReq,
        PVOID                         pData,
        ULONG_PTR                     ul1,
        ULONG_PTR                     ul2
        );
} HTTP_FILTER_CONTEXT, *PHTTP_FILTER_CONTEXT;

//
//  This structure is the notification info for the read and send raw data
//  notification types
//

typedef struct _HTTP_FILTER_RAW_DATA
{
    //
    //  This is a pointer to the data for the filter to process.
    //

    PVOID         pvInData;
    DWORD         cbInData;       // Number of valid data bytes
    DWORD         cbInBuffer;     // Total size of buffer

    DWORD         dwReserved;

} HTTP_FILTER_RAW_DATA, *PHTTP_FILTER_RAW_DATA;

//
//  This structure is the notification info for when the server is about to
//  process the client headers
//

typedef struct _HTTP_FILTER_PREPROC_HEADERS
{
    //
    //  For SF_NOTIFY_PREPROC_HEADERS, retrieves the specified header value.
    //  Header names should include the trailing ':'.  The special values
    //  'method', 'url' and 'version' can be used to retrieve the individual
    //  portions of the request line
    //

    BOOL (WINAPI * GetHeader) (
        struct _HTTP_FILTER_CONTEXT * pfc,
        LPSTR                         lpszName,
        LPVOID                        lpvBuffer,
        LPDWORD                       lpdwSize
        );

    //
    //  Replaces this header value to the specified value.  To delete a header,
    //  specified a value of '\0'.
    //

    BOOL (WINAPI * SetHeader) (
        struct _HTTP_FILTER_CONTEXT * pfc,
        LPSTR                         lpszName,
        LPSTR                         lpszValue
        );

    //
    //  Adds the specified header and value
    //

    BOOL (WINAPI * AddHeader) (
        struct _HTTP_FILTER_CONTEXT * pfc,
        LPSTR                         lpszName,
        LPSTR                         lpszValue
        );

    DWORD HttpStatus;               // New in 4.0, status for SEND_RESPONSE
    DWORD dwReserved;               // New in 4.0

} HTTP_FILTER_PREPROC_HEADERS, *PHTTP_FILTER_PREPROC_HEADERS;

typedef HTTP_FILTER_PREPROC_HEADERS HTTP_FILTER_SEND_RESPONSE;
typedef HTTP_FILTER_PREPROC_HEADERS *PHTTP_FILTER_SEND_RESPONSE;

//
//  Authentication information for this request.
//

typedef struct _HTTP_FILTER_AUTHENT
{
    //
    //  Pointer to username and password, empty strings for the anonymous user
    //
    //  Client's can overwrite these buffers which are guaranteed to be at
    //  least SF_MAX_USERNAME and SF_MAX_PASSWORD bytes large.
    //

    CHAR * pszUser;
    DWORD  cbUserBuff;

    CHAR * pszPassword;
    DWORD  cbPasswordBuff;

} HTTP_FILTER_AUTHENT, *PHTTP_FILTER_AUTHENT;



//
//  Indicates the server is going to use the specific physical mapping for
//  the specified URL.  Filters can modify the physical path in place.
//

typedef struct _HTTP_FILTER_URL_MAP
{
    const CHAR * pszURL;

    CHAR *       pszPhysicalPath;
    DWORD        cbPathBuff;

} HTTP_FILTER_URL_MAP, *PHTTP_FILTER_URL_MAP;

//
//  Indicates the server is going to use the specific physical mapping for
//  the specified URL.  Filters can modify the physical path in place.
//
//  Additional members beyond those from HTTP_FILTER_URL_MAP are
//  informational.
//

typedef struct _HTTP_FILTER_URL_MAP_EX
{
    const CHAR * pszURL;

    CHAR *       pszPhysicalPath;
    DWORD        cbPathBuff;

    //
    // The AccessPerm metabase property that applies to this URL
    //
    DWORD        dwFlags;

    //
    // Number of matching characters in physical path corresponding
    // to the metabase node that applies.
    //
    DWORD        cchMatchingPath;

    //
    // Number of matching characters in the URL corresponding
    // to the metabase node that applies.
    //
    DWORD        cchMatchingURL;

    //
    // The physical path of the dll or exe that to which this
    // URL is script mapped.  This member will be NULL if no
    // script map applies.
    //
    const CHAR * pszScriptMapEntry;

} HTTP_FILTER_URL_MAP_EX, *PHTTP_FILTER_URL_MAP_EX;


//
//  Bitfield indicating the requested resource has been denied by the server due
//  to a logon failure, an ACL on a resource, an ISAPI Filter or an
//  ISAPI Application/CGI Application.
//
//  SF_DENIED_BY_CONFIG can appear with SF_DENIED_LOGON if the server
//  configuration did not allow the user to logon.
//

#define SF_DENIED_LOGON             0x00000001
#define SF_DENIED_RESOURCE          0x00000002
#define SF_DENIED_FILTER            0x00000004
#define SF_DENIED_APPLICATION       0x00000008

#define SF_DENIED_BY_CONFIG         0x00010000

typedef struct _HTTP_FILTER_ACCESS_DENIED
{
    const CHAR * pszURL;            // Requesting URL
    const CHAR * pszPhysicalPath;   // Physical path of resource
    DWORD        dwReason;          // Bitfield of SF_DENIED flags

} HTTP_FILTER_ACCESS_DENIED, *PHTTP_FILTER_ACCESS_DENIED;


//
//  The log information about to be written to the server log file.  The
//  string pointers can be replaced but the memory must remain valid until
//  the next notification
//

typedef struct _HTTP_FILTER_LOG
{
    const CHAR * pszClientHostName;
    const CHAR * pszClientUserName;
    const CHAR * pszServerName;
    const CHAR * pszOperation;
    const CHAR * pszTarget;
    const CHAR * pszParameters;

    DWORD  dwHttpStatus;
    DWORD  dwWin32Status;

    DWORD  dwBytesSent;             // IIS 4.0 and later
    DWORD  dwBytesRecvd;            // IIS 4.0 and later
    DWORD  msTimeForProcessing;     // IIS 4.0 and later

} HTTP_FILTER_LOG, *PHTTP_FILTER_LOG;

//
// Called once the client request has been authenticated.
//

typedef struct _HTTP_FILTER_AUTH_COMPLETE_INFO
{
    //
    //  For SF_NOTIFY_AUTH_COMPLETE, retrieves the specified header value.
    //  Header names should include the trailing ':'.  The special values
    //  'method', 'url' and 'version' can be used to retrieve the individual
    //  portions of the request line
    //

    BOOL (WINAPI * GetHeader) (
        struct _HTTP_FILTER_CONTEXT * pfc,
        LPSTR                         lpszName,
        LPVOID                        lpvBuffer,
        LPDWORD                       lpdwSize
        );

    //
    //  Replaces this header value to the specified value.  To delete a header,
    //  specified a value of '\0'.
    //

    BOOL (WINAPI * SetHeader) (
        struct _HTTP_FILTER_CONTEXT * pfc,
        LPSTR                         lpszName,
        LPSTR                         lpszValue
        );

    //
    //  Adds the specified header and value
    //

    BOOL (WINAPI * AddHeader) (
        struct _HTTP_FILTER_CONTEXT * pfc,
        LPSTR                         lpszName,
        LPSTR                         lpszValue
        );
        
    //
    //  Get the authenticated user impersonation token
    //
    
    BOOL (WINAPI * GetUserToken) (
        struct _HTTP_FILTER_CONTEXT * pfc,
        HANDLE *                      phToken
        );
    
    //
    //  Status code to use when sending response
    //
    
    DWORD HttpStatus;               
    
    //
    //  Determines whether to reset auth if URL changed
    //
    
    BOOL  fResetAuth;             
    
    //
    //  Reserved
    //
    
    DWORD dwReserved;            
    
} HTTP_FILTER_AUTH_COMPLETE_INFO, *PHTTP_FILTER_AUTH_COMPLETE_INFO;

//
//  Notification Flags
//
//  SF_NOTIFY_SECURE_PORT
//  SF_NOTIFY_NONSECURE_PORT
//
//      Indicates whether the application wants to be notified for transactions
//      that are happenning on the server port(s) that support data encryption
//      (such as PCT and SSL), on only the non-secure port(s) or both.
//
//  SF_NOTIFY_READ_RAW_DATA
//
//      Applications are notified after the server reads a block of memory
//      from the client but before the server does any processing on the
//      block.  The data block may contain HTTP headers and entity data.
//
//
//

#define SF_NOTIFY_SECURE_PORT               0x00000001
#define SF_NOTIFY_NONSECURE_PORT            0x00000002

#define SF_NOTIFY_READ_RAW_DATA             0x00008000
#define SF_NOTIFY_PREPROC_HEADERS           0x00004000
#define SF_NOTIFY_AUTHENTICATION            0x00002000
#define SF_NOTIFY_URL_MAP                   0x00001000
#define SF_NOTIFY_ACCESS_DENIED             0x00000800
#define SF_NOTIFY_SEND_RESPONSE             0x00000040
#define SF_NOTIFY_SEND_RAW_DATA             0x00000400
#define SF_NOTIFY_LOG                       0x00000200
#define SF_NOTIFY_END_OF_REQUEST            0x00000080
#define SF_NOTIFY_END_OF_NET_SESSION        0x00000100
#define SF_NOTIFY_AUTH_COMPLETE             0x04000000

//
//  Filter ordering flags
//
//  Filters will tend to be notified by their specified
//  ordering.  For ties, notification order is determined by load order.
//
//  SF_NOTIFY_ORDER_HIGH - Authentication or data transformation filters
//  SF_NOTIFY_ORDER_MEDIUM
//  SF_NOTIFY_ORDER_LOW  - Logging filters that want the results of any other
//                      filters might specify this order.
//

#define SF_NOTIFY_ORDER_HIGH               0x00080000
#define SF_NOTIFY_ORDER_MEDIUM             0x00040000
#define SF_NOTIFY_ORDER_LOW                0x00020000
#define SF_NOTIFY_ORDER_DEFAULT            SF_NOTIFY_ORDER_LOW

#define SF_NOTIFY_ORDER_MASK               (SF_NOTIFY_ORDER_HIGH   |    \
                                            SF_NOTIFY_ORDER_MEDIUM |    \
                                            SF_NOTIFY_ORDER_LOW)

//
//  Filter version information, passed to GetFilterVersion
//

typedef struct _HTTP_FILTER_VERSION
{
    //
    //  Version of the spec the server is using
    //

    DWORD  dwServerFilterVersion;

    //
    //  Fields specified by the client
    //

    DWORD  dwFilterVersion;
    CHAR   lpszFilterDesc[SF_MAX_FILTER_DESC_LEN];
    DWORD  dwFlags;


} HTTP_FILTER_VERSION, *PHTTP_FILTER_VERSION;



//
//  A filter DLL's entry point looks like this.  The return code should be
//  an SF_STATUS_TYPE
//
//  NotificationType - Type of notification
//  pvNotification - Pointer to notification specific data
//

DWORD
WINAPI
HttpFilterProc(
    HTTP_FILTER_CONTEXT *      pfc,
    DWORD                      NotificationType,
    VOID *                     pvNotification
    );

BOOL
WINAPI
GetFilterVersion(
    HTTP_FILTER_VERSION * pVer
    );

BOOL
WINAPI
TerminateFilter(
    DWORD dwFlags
    );


#ifdef __cplusplus
}
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif //_HTTPFILT_H_


