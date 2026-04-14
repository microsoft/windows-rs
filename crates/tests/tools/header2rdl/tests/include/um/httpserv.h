//
//    Copyright (C) Microsoft.  All rights reserved.
//
#ifndef _HTTPSERV_H_
#define _HTTPSERV_H_
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#if (!defined(_WIN64) && !defined(WIN32))
#error httpserv.h is only supported on WIN32 or WIN64 platforms
#endif


#include <ahadmin.h>
#include <http.h>

//
// Request deterministic notifications
//

// request is beginning
#define RQ_BEGIN_REQUEST                    0x00000001
// request is being authenticated
#define RQ_AUTHENTICATE_REQUEST             0x00000002
// request is being authorized
#define RQ_AUTHORIZE_REQUEST                0x00000004
// satisfy request from cache
#define RQ_RESOLVE_REQUEST_CACHE            0x00000008
// map handler for request
#define RQ_MAP_REQUEST_HANDLER              0x00000010
// acquire request state
#define RQ_ACQUIRE_REQUEST_STATE            0x00000020
// pre-execute handler
#define RQ_PRE_EXECUTE_REQUEST_HANDLER      0x00000040
// execute handler
#define RQ_EXECUTE_REQUEST_HANDLER          0x00000080
// release request state
#define RQ_RELEASE_REQUEST_STATE            0x00000100
// update cache
#define RQ_UPDATE_REQUEST_CACHE             0x00000200
// log request
#define RQ_LOG_REQUEST                      0x00000400
// end request
#define RQ_END_REQUEST                      0x00000800

//
// Request non-deterministic notifications
//

// send response status and headers
#define RQ_SEND_RESPONSE_STATUS_AND_HEADERS 0x08000000
// custom notification
#define RQ_CUSTOM_NOTIFICATION              0x10000000
// send response
#define RQ_SEND_RESPONSE                    0x20000000
// read entity
#define RQ_READ_ENTITY                      0x40000000
// map a url to a physical path
#define RQ_MAP_PATH                         0x80000000

// 
// Global notifications
//

// stop accepting new requests
#define GL_STOP_LISTENING               0x00000002
// cache cleanup before termination
#define GL_CACHE_CLEANUP                0x00000004
// cache operation
#define GL_CACHE_OPERATION              0x00000010
// health check
#define GL_HEALTH_CHECK                 0x00000020
// configuration changed
#define GL_CONFIGURATION_CHANGE         0x00000040
// file changed
#define GL_FILE_CHANGE                  0x00000080
// before request pipeline has started
#define GL_PRE_BEGIN_REQUEST            0x00000100
// application start
#define GL_APPLICATION_START            0x00000200
// resolve modules for an application
#define GL_APPLICATION_RESOLVE_MODULES  0x00000400
// application end
#define GL_APPLICATION_STOP             0x00000800
// RSCA query
#define GL_RSCA_QUERY                   0x00001000
// trace event was raised
#define GL_TRACE_EVENT                  0x00002000
// custom notification
#define GL_CUSTOM_NOTIFICATION          0x00004000
// thread cleanup notification
#define GL_THREAD_CLEANUP               0x00008000
// application preload notification
#define GL_APPLICATION_PRELOAD          0x00010000
// suspend process due to inactiviy
#define GL_SUSPEND_PROCESS              0x00020000

//
// Request notification return status
//

enum REQUEST_NOTIFICATION_STATUS
{
    RQ_NOTIFICATION_CONTINUE,                   // continue processing
                                                // for notification
    RQ_NOTIFICATION_PENDING,                    // suspend processing
                                                // for notification
    RQ_NOTIFICATION_FINISH_REQUEST              // finish request
                                                // processing
};

//
// Out of band return codes
//

enum GLOBAL_NOTIFICATION_STATUS
{
    GL_NOTIFICATION_CONTINUE,                  // continue processing
                                               // for notification
    GL_NOTIFICATION_HANDLED                    // finish processing for
                                               // notification
};

// 
// Priority class aliases
//

#define PRIORITY_ALIAS_FIRST              L"FIRST"
#define PRIORITY_ALIAS_HIGH               L"HIGH"
#define PRIORITY_ALIAS_MEDIUM             L"MEDIUM"
#define PRIORITY_ALIAS_LOW                L"LOW"
#define PRIORITY_ALIAS_LAST               L"LAST"

//
// Cache operations
//

enum CACHE_OPERATION
{
    CACHE_OPERATION_RETRIEVE,
    CACHE_OPERATION_ADD,
    CACHE_OPERATION_DELETE,
    CACHE_OPERATION_FLUSH_PREFIX,
    CACHE_OPERATION_ENUM
};

//
// Module identifier
//

typedef VOID*            HTTP_MODULE_ID;

//
// Flags for IHttpContext->CloneContext()
//

#define CLONE_FLAG_BASICS              0x01
#define CLONE_FLAG_HEADERS             0x02
#define CLONE_FLAG_ENTITY              0x04
#define CLONE_FLAG_NO_PRECONDITION     0x08
#define CLONE_FLAG_NO_DAV              0x10
#define CLONE_FLAG_USER_CACHE_VARY_BY  0x20 
#define CLONE_FLAG_SERVER_VARIABLE     0x40

//
// Flags for IHttpContext->ExecuteRequest()
//

#define EXECUTE_FLAG_NO_HEADERS                     0x01
#define EXECUTE_FLAG_IGNORE_CURRENT_INTERCEPTOR     0x02
#define EXECUTE_FLAG_IGNORE_APPPOOL                 0x04
#define EXECUTE_FLAG_DISABLE_CUSTOM_ERROR           0x08
#define EXECUTE_FLAG_SAME_URL                       0x10
// Do not flush the child response but copy it back to the parent
#define EXECUTE_FLAG_BUFFER_RESPONSE                0x20
// Child response is still eligible for http.sys caching
#define EXECUTE_FLAG_HTTP_CACHE_ELIGIBLE            0x40


//
// Forward declarations
//
struct HTTP_TRACE_CONFIGURATION;
struct HTTP_TRACE_EVENT;

class  IWorkerProcessFramework;
class  IWpfSettings;
class  IHttpTraceContext;
class  IHttpContext3;
class  IHttpCompletionInfo2;

//
// Module-specific context descriptor
//
class __declspec(uuid("f1927f76-790e-4ccb-a72e-396bdfdae05d"))
IHttpStoredContext
{
public:
    virtual
    VOID
    CleanupStoredContext(
        VOID
    ) = 0;
};

//
// Context container
//
class __declspec(uuid("d7fad7c9-aa27-4ab9-bd60-e55ccba3f5dc"))
IHttpModuleContextContainer
{
public:
    virtual
    IHttpStoredContext *
    GetModuleContext(
        _In_ HTTP_MODULE_ID       moduleId
    ) = 0;

    virtual
    HRESULT
    SetModuleContext(
        _In_ IHttpStoredContext * ppStoredContext,
        _In_ HTTP_MODULE_ID       moduleId
    ) = 0;  
};

//
// Container for named contexts
//

class __declspec(uuid("16db6f7c-1a94-47ca-a8e8-fea6005ff3ba"))
INamedContextContainer
{
public:
    virtual
    IHttpStoredContext *
    GetNamedContext(
        _In_ LPCWSTR szName
    ) = 0;

    virtual
    HRESULT
    SetNamedContext(
        _In_ IHttpStoredContext * ppStoredContext,
        _In_ LPCWSTR              szName
        ) = 0;
};

//
// Dispensed context container
//
class __declspec(uuid("2ae49359-95dd-4e48-ae20-c0cb9d0bc03a"))
IDispensedHttpModuleContextContainer : public IHttpModuleContextContainer
{
public:
    virtual
    VOID
    ReleaseContainer(
        VOID
    ) = 0;
};

//
// Performance counter descriptor
//
class __declspec(uuid("bdfc4c4a-12a4-4744-87d8-765eb320c59f"))
IHttpPerfCounterInfo
{
public:
    virtual
    VOID
    IncrementCounter(
        DWORD               dwCounterIndex,
        DWORD               dwValue = 1
    ) = 0;

    virtual
    VOID
    DecrementCounter(
        DWORD               dwCounterIndex,
        DWORD               dwValue = 1
    ) = 0;
};

//
// Application descriptor
//
class __declspec(uuid("3f75d9e6-1075-422c-ad89-93a85f2d7bdc"))
IHttpApplication
{
public:
    virtual
    PCWSTR
    GetApplicationPhysicalPath(
        VOID
    ) const = 0;

    virtual
    PCWSTR
    GetApplicationId(
        VOID
    ) const = 0;

    virtual
    PCWSTR
    GetAppConfigPath(
        VOID
    ) const = 0;

    virtual
    IHttpModuleContextContainer *
    GetModuleContextContainer(
        VOID
    ) = 0;
};

//
// IHttpApplication2 is available on Windows 8 and newer
//

class __declspec(uuid("311a3b7a-b673-4a1c-8349-2e98a23cadcb"))
IHttpApplication2 : public IHttpApplication
{
public:
    virtual
    VOID
    BeginApplicationWarmup(
        VOID
    ) = 0;

    virtual
    VOID
    EndApplicationWarmup(
        VOID
    ) = 0;

    virtual
    BOOL
    QueryIsWarmingUp(
        VOID
    ) = 0;
};

// 
// URI cache entry descriptor
// 
class __declspec(uuid("7e0e6167-0094-49a1-8287-ecf6dc6e73a6"))
IHttpUrlInfo
{
public:
    virtual
    IHttpModuleContextContainer *
    GetModuleContextContainer(
        VOID
    ) = 0;

    virtual
    BOOL
    IsFrequentlyHit(
        VOID
    ) const = 0;
};

// 
// Script map descriptor
// 
class __declspec(uuid("d7fe3d77-68bc-4d4a-851f-eec9fb68017c"))
IScriptMapInfo
{
public:
    virtual
    PCWSTR
    GetPath(
        VOID
    ) const = 0;

    virtual
    PCSTR
    GetAllowedVerbs(
        VOID
    ) const = 0;

    virtual
    _Ret_writes_(*pcchModules)
    PCWSTR
    GetModules(
        _Out_ DWORD *       pcchModules = NULL
    ) const = 0;

    virtual
    _Ret_writes_(*pcchScriptProcessor)
    PCWSTR
    GetScriptProcessor(
        _Out_ DWORD *       pcchScriptProcessor = NULL
    ) const = 0;

    virtual
    _Ret_writes_(*pcchManagedType)
    PCWSTR
    GetManagedType(
        _Out_ DWORD *       pcchManagedType = NULL
    ) const = 0;

    virtual
    BOOL
    GetAllowPathInfoForScriptMappings(
        VOID
    ) const = 0;

    virtual
    DWORD
    GetRequiredAccess(
        VOID
    ) const = 0;

    virtual
    DWORD
    GetResourceType(
        VOID
    ) const = 0;

    virtual
    BOOL
    GetIsStarScriptMap(
        VOID
    ) const = 0;

    virtual
    DWORD
    GetResponseBufferLimit(
        VOID
    ) const = 0;

    virtual
    PCWSTR
    GetName(
        VOID
    ) const = 0;
};

class __declspec(uuid("fd86e6de-fb0e-47dd-820a-e0da12be46e9"))
IHttpTokenEntry;

// 
// Metadata descriptor
// 
class __declspec(uuid("48b10633-825d-495e-93b0-225380053e8e"))
IMetadataInfo
{
public:
    virtual
    PCWSTR
    GetMetaPath(
        VOID
    ) const = 0;

    virtual
    PCWSTR
    GetVrPath(
        VOID
    ) const = 0;

    virtual
    IHttpTokenEntry *
    GetVrToken(
        VOID
    ) = 0;

    virtual
    IHttpModuleContextContainer *
    GetModuleContextContainer(
        VOID
    ) = 0;
};

class __declspec(uuid("3fc5c336-9ad7-45ea-af2f-31b59302c9fe"))
IReferencedMetadataInfo : public IMetadataInfo
{
public:
    virtual
    VOID
    ReferenceMetadata(
        VOID
    ) = 0;

    virtual
    VOID
    DereferenceMetadata(
        VOID
    ) = 0;
};

// 
// Provides an interface to an HTTP request object.  The methods on this
// class can be used to inspect and modify request data.
// 
class __declspec(uuid("e8698f7e-576e-4cac-a309-67435355faef"))
IHttpRequest
{
public:
    virtual
    HTTP_REQUEST *
    GetRawHttpRequest(
        VOID
    ) = 0;

    virtual
    const HTTP_REQUEST *
    GetRawHttpRequest(
        VOID
    ) const = 0;

    virtual
    _Ret_writes_bytes_(*pcchHeaderValue)
    PCSTR
    GetHeader(
        _In_ PCSTR              pszHeaderName,
        _Out_ USHORT *          pcchHeaderValue = NULL
    ) const = 0;

    virtual
    _Ret_writes_bytes_(*pcchHeaderValue)
    PCSTR
    GetHeader(
        _In_  HTTP_HEADER_ID    ulHeaderIndex,
        _Out_ USHORT *          pcchHeaderValue = NULL
    ) const = 0;

    virtual
    HRESULT
    SetHeader(
        _In_ PCSTR              pszHeaderName,
        _In_reads_bytes_(cchHeaderValue)
        PCSTR                   pszHeaderValue,
        _In_ USHORT             cchHeaderValue,
        _In_ BOOL               fReplace
    ) = 0;

    virtual
    HRESULT
    SetHeader(
        _In_ HTTP_HEADER_ID     ulHeaderIndex,
        _In_ PCSTR              pszHeaderValue,
        _In_ USHORT             cchHeaderValue,
        _In_ BOOL               fReplace
    ) = 0;

    virtual
    HRESULT
    DeleteHeader(
        _In_ PCSTR              pszHeaderName
    ) = 0;

    virtual
    HRESULT
    DeleteHeader(
        _In_ HTTP_HEADER_ID     ulHeaderIndex
    ) = 0;

    virtual
    PCSTR
    GetHttpMethod(
        VOID
    ) const = 0;

    virtual
    HRESULT
    SetHttpMethod(
        _In_ PCSTR              pszHttpMethod
    ) = 0;

    virtual
    HRESULT
    SetUrl(
        _In_reads_(cchUrl)
        PCWSTR                  pszUrl,
        _In_ DWORD              cchUrl,
        _In_ BOOL               fResetQueryString
    ) = 0;

    virtual
    HRESULT
    SetUrl(
        _In_reads_bytes_(cchUrl)
        PCSTR                   pszUrl,
        _In_ DWORD              cchUrl,
        _In_ BOOL               fResetQueryString
    ) = 0;

    virtual
    BOOL
    GetUrlChanged(
        VOID
    ) const = 0;

    virtual
    PCWSTR
    GetForwardedUrl(
        VOID
    ) const = 0;

    virtual
    PSOCKADDR
    GetLocalAddress(
        VOID
    ) const = 0;

    virtual
    PSOCKADDR
    GetRemoteAddress(
        VOID
    ) const = 0;

    virtual
    HRESULT
    ReadEntityBody(
        _Out_writes_bytes_all_(cbBuffer)
        VOID *                  pvBuffer,
        _In_  DWORD             cbBuffer,
        _In_  BOOL              fAsync,
        _Out_ DWORD *           pcbBytesReceived,
        _Out_ BOOL *            pfCompletionPending = NULL
    ) = 0;

    virtual
    HRESULT
    InsertEntityBody(
        _In_reads_bytes_(cbBuffer)
        VOID *                  pvBuffer,
        _In_ DWORD              cbBuffer
    ) = 0;

    virtual
    DWORD
    GetRemainingEntityBytes(
        VOID
    ) = 0;

    virtual
    VOID
    GetHttpVersion(
        _Out_ USHORT *          pMajorVersion,
        _Out_ USHORT *          pMinorVersion
    ) const = 0;

    virtual
    HRESULT
    GetClientCertificate(
        _Outptr_
        HTTP_SSL_CLIENT_CERT_INFO **    ppClientCertInfo,
        _Out_ BOOL *                    pfClientCertNegotiated
    ) = 0;

    virtual
    HRESULT
    NegotiateClientCertificate(
        _In_ BOOL               fAsync,
        _Out_ BOOL *            pfCompletionPending = NULL
    ) = 0;

    virtual
    DWORD
    GetSiteId(
        VOID
    ) const = 0;

    virtual
    HRESULT
    GetHeaderChanges(
        _In_ DWORD          dwOldChangeNumber,
        _Out_ DWORD *       pdwNewChangeNumber,
        _Inout_ PCSTR       knownHeaderSnapshot[HttpHeaderRequestMaximum],
        _Inout_ DWORD *     pdwUnknownHeaderSnapshot,
        _Inout_ PCSTR **    ppUnknownHeaderNameSnapshot,
        _Inout_ PCSTR **    ppUnknownHeaderValueSnapshot,
        _Out_writes_all_(HttpHeaderRequestMaximum+1)
        DWORD               diffedKnownHeaderIndices[HttpHeaderRequestMaximum+1],
        _Out_ DWORD *       pdwDiffedUnknownHeaders,
        _Out_ DWORD **      ppDiffedUnknownHeaderIndices
    ) = 0;
};

class __declspec(uuid("d9244ae1-51f8-4aa1-a66d-19277c33e610"))
IHttpRequest2 : public IHttpRequest
{
public:
    virtual
    HRESULT
    GetChannelBindingToken(
        _Outptr_result_bytebuffer_to_(*pTokenSize, *pTokenSize)
        PBYTE *             ppToken,
        _Out_ DWORD *       pTokenSize
    ) = 0;
};

//
// Custom Completion routine definition
//

typedef 
REQUEST_NOTIFICATION_STATUS
(WINAPI * PFN_ASYNC_COMPLETION )(
    IHttpContext3 *            pHttpContext,
    IHttpCompletionInfo2 *     pCompletionInfo,
    VOID  *                    pvCompletionContext
);

class __declspec(uuid("b32e4e0f-4057-4feb-aeab-7b69c03c6314"))
IHttpRequest3 : public IHttpRequest2
{
public:    
    virtual
    HRESULT
    ReadEntityBody(
        _Out_writes_bytes_all_(cbBuffer)
        VOID *                          pvBuffer,
        _In_     DWORD                  cbBuffer,
        _In_     BOOL                   fAsync,
        _In_     PFN_ASYNC_COMPLETION   pfnCompletionCallback,
        _In_     VOID *                 pvCompletionContext,
        _Out_    DWORD *                pcbBytesReceived,
        _Out_    BOOL *                 pfCompletionPending = NULL
    ) = 0;
};

class __declspec(uuid("3816f517-f631-49f0-8b51-16d6f0c6ffb2"))
IHttpRequest4 : public IHttpRequest3
{
public:
    virtual
    HRESULT
    ReadEntityBody(
        _Out_writes_bytes_all_(cbBuffer)
        VOID*                       pvBuffer,
        _In_      DWORD             cbBuffer,
        _In_      BOOL              fAsync,
        _In_      ULONG             ulFlags,
        _Out_     DWORD *           pcbBytesReceived,
        _Out_     BOOL *            pfCompletionPending = NULL
    ) = 0;
};

class __declspec(uuid("cb1c40ca-70f2-41a0-add2-881f5ef57388"))
IHttpCachePolicy
{
public:
    virtual
    HTTP_CACHE_POLICY *
    GetKernelCachePolicy(
        VOID
    ) = 0;

    virtual
    VOID
    SetKernelCacheInvalidatorSet(
        VOID
    ) = 0;

    virtual
    HTTP_CACHE_POLICY *
    GetUserCachePolicy(
        VOID
    ) = 0;

    virtual
    HRESULT
    AppendVaryByHeader(
        _In_ PCSTR   pszHeader
    ) = 0;

    virtual
    PCSTR
    GetVaryByHeaders(
        VOID
    ) const = 0;

    virtual
    HRESULT
    AppendVaryByQueryString(
        _In_ PCSTR   pszParam
    ) = 0;

    virtual
    PCSTR
    GetVaryByQueryStrings(
        VOID
    ) const = 0;

    virtual
    HRESULT
    SetVaryByValue(
        _In_ PCSTR   pszValue
    ) = 0;

    virtual
    PCSTR
    GetVaryByValue(
        VOID
    ) const = 0;

    virtual
    BOOL
    IsUserCacheEnabled(
        VOID
    ) const = 0;

    virtual
    VOID
    DisableUserCache(
        VOID
    ) = 0;

    virtual
    BOOL
    IsCached(
        VOID
    ) const = 0;

    virtual
    VOID
    SetIsCached(
        VOID
    ) = 0;

    virtual
    BOOL
    GetKernelCacheInvalidatorSet(
        VOID
    ) const = 0;
};

class __declspec(uuid("9f4ba807-050e-4495-ae55-8870f7e9194a"))
IHttpCachePolicy2 : public IHttpCachePolicy
{
public:
    virtual
    BOOL
    IsForceUpdateSet(
        VOID
    ) const = 0;

    virtual
    VOID
    SetForceUpdate(
        VOID
    ) = 0;
};

// 
// Response descriptor
// 
class __declspec(uuid("7e1c6b38-628f-4e6c-95dc-41237eb7f95e"))
IHttpResponse
{
public:
    virtual
    HTTP_RESPONSE *
    GetRawHttpResponse(
        VOID
    ) = 0;

    virtual
    const HTTP_RESPONSE *
    GetRawHttpResponse(
        VOID
    ) const = 0;

    virtual
    IHttpCachePolicy *
    GetCachePolicy(
        VOID
    ) = 0;

    virtual
    HRESULT
    SetStatus(
        _In_ USHORT                         statusCode,
        _In_ PCSTR                          pszReason,
        _In_ USHORT                         uSubStatus = 0,
        _In_ HRESULT                        hrErrorToReport = S_OK,
        _In_opt_ IAppHostConfigException *  pException = NULL,
        _In_ BOOL                           fTrySkipCustomErrors = FALSE
    ) = 0;

    virtual
    HRESULT
    SetHeader(
        _In_ PCSTR              pszHeaderName,
        _In_ PCSTR              pszHeaderValue,
        _In_ USHORT             cchHeaderValue,
        _In_ BOOL               fReplace
    ) = 0;

    virtual
    HRESULT
    SetHeader(
        _In_ HTTP_HEADER_ID     ulHeaderIndex,
        _In_ PCSTR              pszHeaderValue,
        _In_ USHORT             cchHeaderValue,
        _In_ BOOL               fReplace
    ) = 0;

    virtual
    HRESULT
    DeleteHeader(
        _In_ PCSTR              pszHeaderName
    ) = 0;

    virtual
    HRESULT
    DeleteHeader(
        _In_ HTTP_HEADER_ID     ulHeaderIndex
    ) = 0;

    virtual
    _Ret_writes_bytes_(*pcchHeaderValue)
    PCSTR
    GetHeader(
        _In_ PCSTR              pszHeaderName,
        _Out_ USHORT *          pcchHeaderValue = NULL
    ) const = 0;

    virtual
    _Ret_writes_bytes_(*pcchHeaderValue)
    PCSTR
    GetHeader(
        _In_  HTTP_HEADER_ID    ulHeaderIndex,
        _Out_ USHORT *          pcchHeaderValue = NULL
    ) const = 0;

    virtual
    VOID
    Clear(
        VOID
    ) = 0;

    virtual
    VOID
    ClearHeaders(
        VOID
    ) = 0;

    virtual
    VOID
    SetNeedDisconnect(
        VOID
    ) = 0;

    virtual
    VOID
    ResetConnection(
        VOID
    ) = 0;

    virtual
    VOID
    DisableKernelCache(
        ULONG reason = 9
    ) = 0;

    virtual
    BOOL
    GetKernelCacheEnabled(
        VOID
    ) const = 0;

    virtual
    VOID
    SuppressHeaders(
        VOID
    ) = 0;

    virtual
    BOOL
    GetHeadersSuppressed(
        VOID
    ) const = 0;

    virtual
    HRESULT
    Flush(
        _In_ BOOL               fAsync,
        _In_ BOOL               fMoreData,
        _Out_ DWORD *           pcbSent,
        _Out_ BOOL *            pfCompletionExpected = NULL
    ) = 0;

    virtual
    HRESULT
    Redirect(
        _In_ PCSTR              pszUrl,
        _In_ BOOL               fResetStatusCode = TRUE,
        _In_ BOOL               fIncludeParameters = FALSE
    ) = 0;

    virtual
    HRESULT
    WriteEntityChunkByReference(
        _In_ HTTP_DATA_CHUNK *  pDataChunk,
        _In_ LONG               lInsertPosition = -1
    ) = 0;

    virtual
    HRESULT
    WriteEntityChunks(
        _In_reads_(nChunks)
        HTTP_DATA_CHUNK *       pDataChunks,
        _In_  DWORD             nChunks,
        _In_  BOOL              fAsync,
        _In_  BOOL              fMoreData,
        _Out_ DWORD *           pcbSent,
        _Out_ BOOL *            pfCompletionExpected = NULL
    ) = 0;

    virtual
    VOID
    DisableBuffering(
        VOID
    ) = 0;

    virtual
    VOID
    GetStatus(
        _Out_ USHORT *              pStatusCode,
        _Out_ USHORT *              pSubStatus = NULL,
        _Outptr_opt_result_bytebuffer_(*pcchReason)
        PCSTR *                     ppszReason = NULL,
        _Out_ USHORT *              pcchReason = NULL,
        _Out_ HRESULT *             phrErrorToReport = NULL,
        _Outptr_opt_ 
        PCWSTR *                    ppszModule = NULL,
        _Out_ DWORD *               pdwNotification = NULL,
        _Outptr_opt_ 
        IAppHostConfigException **  ppException = NULL,
        _Out_ BOOL *                pfTrySkipCustomErrors = NULL
    ) = 0;

    virtual
    HRESULT
    SetErrorDescription(
        _In_reads_(cchDescription)
        PCWSTR                      pszDescription,
        _In_ DWORD                  cchDescription,
        _In_ BOOL                   fHtmlEncode = TRUE
    ) = 0;

    virtual
    _Ret_writes_(*pcchDescription)
    PCWSTR
    GetErrorDescription(
        _Out_ DWORD *               pcchDescription = NULL
    ) = 0;

    virtual
    HRESULT
    GetHeaderChanges(
        _In_ DWORD          dwOldChangeNumber,
        _Out_ DWORD *       pdwNewChangeNumber,
        _Inout_ PCSTR       knownHeaderSnapshot[HttpHeaderResponseMaximum],
        _Inout_ DWORD *     pdwUnknownHeaderSnapshot,
        _Inout_ PCSTR **    ppUnknownHeaderNameSnapshot,
        _Inout_ PCSTR **    ppUnknownHeaderValueSnapshot,
        _Out_writes_(HttpHeaderResponseMaximum+1)
        DWORD               diffedKnownHeaderIndices[HttpHeaderResponseMaximum+1],
        _Out_ DWORD *       pdwDiffedUnknownHeaders,
        _Out_ DWORD **      ppDiffedUnknownHeaderIndices
    ) = 0;

    virtual
    VOID
    CloseConnection(
        VOID
    ) = 0;
};

class __declspec(uuid("0822c871-f14e-4974-a1e6-5b3c1f09b76a"))
IHttpResponse2 : public IHttpResponse
{
public:
    virtual
    HRESULT
    WriteEntityChunks(
        _In_reads_(nChunks)
        HTTP_DATA_CHUNK *               pDataChunks,
        _In_  DWORD                     nChunks,
        _In_  BOOL                      fAsync,
        _In_  BOOL                      fMoreData,
        _In_  PFN_ASYNC_COMPLETION      pfnCompletionCallback,
        _In_  VOID *                    pvCompletionContext,
        _Out_ DWORD *                   pcbSent,
        _Out_ BOOL *                    pfCompletionExpected = NULL
    ) = 0;

    virtual
    HRESULT
    Flush(
        _In_ BOOL                       fAsync,
        _In_ BOOL                       fMoreData,
        _In_  PFN_ASYNC_COMPLETION      pfnCompletionCallback,
        _In_  VOID *                    pvCompletionContext,
        _Out_ DWORD *                   pcbSent,
        _Out_ BOOL *                    pfCompletionExpected = NULL
    ) = 0;
};

//
// Add the SPDY/3 & HTTP/2.0 Push-Promise support
//

class __declspec(uuid("19578e49-d080-45da-b673-96cb475d3c5d"))
IHttpResponse3 : public IHttpResponse2
{
public:
    virtual
    HRESULT
    PushPromise(
        _In_ PCSTR                       pszVerb,
        _In_ PCWSTR                      pwszPath,
        _In_opt_ PCWSTR                  pwszQuery,
        _In_ DWORD                       cHeaders,
        _In_opt_count_(cHeaders) PCSTR * rgHeaderNames,
        _In_opt_count_(cHeaders) PCSTR * rgHeaderValues
    ) = 0;
};

//
// Add support for certain HTTP/2.0 features like trailing headers
// and GOAWAY or RST_STREAM frames.
//

class __declspec(uuid("1a2acc57-cae2-4f28-b4ab-00c8f96b12ec"))
IHttpResponse4 : public IHttpResponse3
{
public:
    virtual
    HRESULT
    DeleteTrailer(
        _In_ PCSTR  pszHeaderName
    ) = 0;

    virtual
    PCSTR
    GetTrailer(
        _In_  PCSTR    pszHeaderName,
        _Out_ USHORT * pcchHeaderValue = NULL
    ) const = 0;

    virtual
    VOID
    ResetStream(
        _In_ ULONG errorCode
    ) = 0;

    virtual
    VOID
    SetNeedGoAway(
        VOID
    ) = 0;

    virtual
    HRESULT
    SetTrailer(
        _In_ PCSTR  pszHeaderName,
        _In_ PCSTR  pszHeaderValue,
        _In_ USHORT cchHeaderValue,
        _In_ BOOL fReplace
    ) = 0;
};

// 
// User descriptor
// 
class __declspec(uuid("8059e6f8-10ce-4d61-b47e-5a1d8d9a8b67"))
IHttpUser
{
public:
    virtual
    PCWSTR
    GetRemoteUserName(
        VOID
    ) = 0;

    virtual
    PCWSTR
    GetUserName(
        VOID
    ) = 0;

    virtual
    PCWSTR
    GetAuthenticationType(
        VOID
    ) = 0;

    virtual
    PCWSTR
    GetPassword(
        VOID
    ) = 0;

    virtual
    HANDLE
    GetImpersonationToken(
        VOID
    ) = 0;

    virtual
    HANDLE
    GetPrimaryToken(
        VOID
    ) = 0;

    virtual
    VOID
    ReferenceUser(
        VOID
    ) = 0;

    virtual
    VOID
    DereferenceUser(
        VOID
    ) = 0;

    virtual
    BOOL
    SupportsIsInRole(
        VOID
    ) = 0;

    virtual
    HRESULT
    IsInRole(
        _In_  PCWSTR  pszRoleName,
        _Out_ BOOL *  pfInRole
    ) = 0;

    virtual
    PVOID
    GetUserVariable(
        _In_ PCSTR    pszVariableName
    ) = 0;
};

#define HTTP_USER_VARIABLE_SID              "SID"
#define HTTP_USER_VARIABLE_CTXT_HANDLE      "CtxtHandle"
#define HTTP_USER_VARIABLE_CRED_HANDLE      "CredHandle"

class __declspec(uuid("841d9a71-75f4-4626-8b97-66046ca7e45b"))
IHttpConnectionStoredContext : public IHttpStoredContext
{
public:
    virtual
    VOID
    NotifyDisconnect(
        VOID
    ) = 0;
};

class __declspec(uuid("f3dd2fb3-4d11-4295-b8ab-4cb667add1fe"))
IHttpConnectionModuleContextContainer : public IHttpModuleContextContainer
{
public:
    virtual
    IHttpConnectionStoredContext *
    GetConnectionModuleContext(
        _In_ HTTP_MODULE_ID       moduleId
    ) = 0;

    virtual
    HRESULT
    SetConnectionModuleContext(
        _In_ IHttpConnectionStoredContext *   ppStoredContext,
        _In_ HTTP_MODULE_ID                   moduleId
    ) = 0;
};

// 
// Connection descriptor
// 
class __declspec(uuid("d9a5de00-3346-4599-9826-fe88565e1226"))
IHttpConnection
{
public:
    virtual
    BOOL
    IsConnected(
        VOID
    ) const = 0;

    virtual
    
    _Ret_opt_ _Post_writable_byte_size_(cbAllocation)
    VOID *
    AllocateMemory(
        DWORD               cbAllocation
    ) = 0;

    virtual
    IHttpConnectionModuleContextContainer *
    GetModuleContextContainer(
        VOID
    ) = 0;
};

// 
// Forward declarations
// 
class __declspec(uuid("71e95595-8c74-44d9-88a9-f5112d5f5900"))
IHttpFileInfo;

class __declspec(uuid("eb16a6ec-ba5d-436f-bf24-3ede13906450"))
IHttpSite;

class __declspec(uuid("671e6d34-9380-4df4-b453-91129df02b24"))
ICustomNotificationProvider;

class __declspec(uuid("6f3f657d-2fb8-43c6-a096-5064b41f0580"))
IHttpEventProvider;

class CHttpModule;

//
// IHttpContext extended interface versions (deprecated)
//
enum HTTP_CONTEXT_INTERFACE_VERSION
{
};

// 
// Context object representing the processing of an HTTP request
// 
class __declspec(uuid("424c1b8c-a1ba-44d7-ac98-9f8f457701a5"))
IHttpContext
{
 public:
    virtual
    IHttpSite *
    GetSite(
        VOID
    ) = 0;

    virtual
    IHttpApplication *
    GetApplication(
        VOID
    ) = 0;

    virtual
    IHttpConnection *
    GetConnection(
        VOID
    ) = 0;

    virtual
    IHttpRequest *
    GetRequest(
        VOID
    ) = 0;

    virtual
    IHttpResponse *
    GetResponse(
        VOID
    ) = 0;

    virtual
    BOOL
    GetResponseHeadersSent(
        VOID
    ) const = 0;

    virtual
    IHttpUser *
    GetUser(
        VOID
    ) const = 0;

    virtual
    IHttpModuleContextContainer *
    GetModuleContextContainer(
        VOID
    ) = 0;

    virtual
    VOID
    IndicateCompletion(
        _In_ REQUEST_NOTIFICATION_STATUS     notificationStatus
    ) = 0;

    virtual
    HRESULT
    PostCompletion(
        _In_ DWORD                cbBytes
    ) = 0;

    virtual
    VOID
    DisableNotifications(
        _In_ DWORD                dwNotifications,
        _In_ DWORD                dwPostNotifications
    ) = 0;

    virtual
    BOOL
    GetNextNotification(
        _In_ REQUEST_NOTIFICATION_STATUS    status,
        _Out_ DWORD *                       pdwNotification,
        _Out_ BOOL *                        pfIsPostNotification,
        _Outptr_ CHttpModule **          ppModuleInfo,
        _Outptr_ IHttpEventProvider **   ppRequestOutput
    ) = 0;

    virtual
    BOOL
    GetIsLastNotification(
        _In_ REQUEST_NOTIFICATION_STATUS   status
    ) = 0;

    virtual
    HRESULT
    ExecuteRequest(
        _In_ BOOL                   fAsync,
        _In_ IHttpContext *         pHttpContext,
        _In_ DWORD                  dwExecuteFlags,
        _In_ IHttpUser *            pHttpUser,
        _Out_ BOOL *                pfCompletionExpected = NULL
    ) = 0;

    virtual
    DWORD
    GetExecuteFlags(
        VOID
    ) const = 0;

    virtual
    HRESULT
    GetServerVariable(
        _In_ PCSTR          pszVariableName,
        _Outptr_result_buffer_(*pcchValueLength)
        PCWSTR *            ppszValue,
        _Out_ DWORD *       pcchValueLength
    ) = 0;

    virtual
    HRESULT
    GetServerVariable(
        _In_ PCSTR          pszVariableName,
        _Outptr_result_bytebuffer_(*pcchValueLength)
        PCSTR *             ppszValue,
        _Out_ DWORD *       pcchValueLength
    ) = 0;

    virtual
    HRESULT
    SetServerVariable(
        PCSTR               pszVariableName,
        PCWSTR              pszVariableValue
    ) = 0;

    virtual
    
    _Ret_opt_ _Post_writable_byte_size_(cbAllocation)
    VOID *
    AllocateRequestMemory(
        _In_ DWORD          cbAllocation
    ) = 0;

    virtual
    IHttpUrlInfo *
    GetUrlInfo(
        VOID
    ) = 0;

    virtual
    IMetadataInfo *
    GetMetadata(
        VOID
    ) = 0;

    virtual
    _Ret_writes_(*pcchPhysicalPath)
    PCWSTR
    GetPhysicalPath(
        _Out_ DWORD *       pcchPhysicalPath = NULL
    ) = 0;

    virtual
    _Ret_writes_(*pcchScriptName)
    PCWSTR
    GetScriptName(
        _Out_ DWORD *       pcchScriptName = NULL
    ) const = 0;

    virtual
    _Ret_writes_(*pcchScriptTranslated)
    PCWSTR
    GetScriptTranslated(
        _Out_ DWORD *       pcchScriptTranslated = NULL
    ) = 0;

    virtual
    IScriptMapInfo *
    GetScriptMap(
        VOID
    ) const = 0;

    virtual
    VOID
    SetRequestHandled(
        VOID
    ) = 0;

    virtual
    IHttpFileInfo *
    GetFileInfo(
        VOID
    ) const = 0;

    virtual
    HRESULT
    MapPath(
        _In_ PCWSTR         pszUrl,
        _Inout_updates_(*pcbPhysicalPath)
        PWSTR               pszPhysicalPath,
        _Inout_ DWORD *     pcbPhysicalPath
    ) = 0;

    virtual
    HRESULT
    NotifyCustomNotification(
        _In_ ICustomNotificationProvider *  pCustomOutput,
        _Out_ BOOL *                        pfCompletionExpected
    ) = 0;

    virtual
    IHttpContext *
    GetParentContext(
        VOID
    ) const = 0;

    virtual
    IHttpContext *
    GetRootContext(
        VOID
    ) const = 0;

    virtual
    HRESULT
    CloneContext(
        _In_ DWORD          dwCloneFlags,
        _Outptr_
        IHttpContext **     ppHttpContext
    ) = 0;

    virtual
    HRESULT
    ReleaseClonedContext(
        VOID
    ) = 0;

    virtual
    HRESULT
    GetCurrentExecutionStats(
        _Out_ DWORD *   pdwNotification,
        _Out_ DWORD *   pdwNotificationStartTickCount = NULL,
        _Out_ PCWSTR *  ppszModule = NULL,
        _Out_ DWORD *   pdwModuleStartTickCount = NULL,
        _Out_ DWORD *   pdwAsyncNotification = NULL,
        _Out_ DWORD *   pdwAsyncNotificationStartTickCount = NULL
    ) const = 0;

    virtual
    IHttpTraceContext *
    GetTraceContext(
        VOID
    ) const = 0;

    virtual
    HRESULT
    GetServerVarChanges(
        _In_    DWORD       dwOldChangeNumber,
        _Out_   DWORD *     pdwNewChangeNumber,
        _Inout_ DWORD *     pdwVariableSnapshot,
        _Inout_ _At_(*ppVariableNameSnapshot, _Pre_readable_size_(*pdwVariableSnapshot) _Post_readable_size_(*pdwVariableSnapshot))
                PCSTR **    ppVariableNameSnapshot,
        _Inout_ _At_(*ppVariableValueSnapshot, _Pre_readable_size_(*pdwVariableSnapshot) _Post_readable_size_(*pdwVariableSnapshot))
                PCWSTR **   ppVariableValueSnapshot,
        _Out_   DWORD *     pdwDiffedVariables,
        _Out_   DWORD **    ppDiffedVariableIndices
    ) = 0;

    virtual
    HRESULT
    CancelIo(
        VOID
    ) = 0;

    virtual
    HRESULT
    MapHandler(
        _In_ DWORD          dwSiteId,
        _In_ PCWSTR         pszSiteName,
        _In_ PCWSTR         pszUrl,
        _In_ PCSTR          pszVerb,
        _Outptr_
        IScriptMapInfo **   ppScriptMap,
        _In_ BOOL           fIgnoreWildcardMappings = FALSE
    ) = 0;

    __declspec(deprecated("This method is deprecated. Use the HttpGetExtendedInterface helper function instead."))
    virtual
    HRESULT
    GetExtendedInterface(
        _In_ HTTP_CONTEXT_INTERFACE_VERSION version,
        _Outptr_ PVOID * ppInterface
    ) = 0;
};


// 
// Context object representing the processing of an HTTP request
// IHttpContext2 is available on Windows 7 and newer
//
class __declspec(uuid("c986182c-cf4a-4482-8205-0dbbc1fd6cee"))
IHttpContext2 : public IHttpContext
{
public:
    virtual
    IHttpUser *
    GetOriginalUser(
        VOID
    ) const = 0;
};

//
// Context object representing the processing of an HTTP request
// IHttpContext3 is available on Windows 8 and newer
//

class __declspec(uuid("bf53c022-ca4b-4349-a38e-a4b78ad897fb"))
IHttpContext3 : public IHttpContext2
{
public:
    virtual
    INamedContextContainer *
    GetNamedContextContainer(
        VOID
    ) = 0;

    virtual
    VOID
    EnableFullDuplex(
        VOID
    ) = 0;

    virtual
    HRESULT
    InheritServerVariable(
        _In_ PCSTR szName,
        _In_ BOOL  fFailIfNotExist = FALSE
    ) = 0;

    virtual
    HRESULT
    PostCompletion(
        DWORD                             cbBytes,
        LPOVERLAPPED_COMPLETION_ROUTINE   pfnCompletionRoutine,
        VOID *                            pvCompletionContext
    ) = 0;
};

//
// Context object representing the processing of an HTTP request
// IHttpContext4 is available in Windows 9 and newer.
//

class __declspec(uuid("2c9463d8-4ec4-4622-8917-d486321a9443"))
IHttpContext4 : public IHttpContext3
{
public:
    //
    // Gets the request start time as returned by GetTickCount64()
    //
    virtual
    ULONGLONG
    GetRequestStartTime(
        VOID
    ) = 0;

    //
    // Gets the elapsed time in milliseconds
    //
    virtual
    ULONGLONG
    GetRequestElapsedTime(
        VOID
    ) = 0;
};

class __declspec(uuid("9f9098d5-915c-4294-a52e-66532a232bc9"))
IHttpTraceContext
{
public:
    virtual
    HRESULT
    GetTraceConfiguration(
        _Inout_ HTTP_TRACE_CONFIGURATION *  pHttpTraceConfiguration
    ) = 0;
    
    virtual    
    HRESULT
    SetTraceConfiguration(
        _In_ HTTP_MODULE_ID              moduleId,
        _In_ HTTP_TRACE_CONFIGURATION *  pHttpTraceConfiguration,
        _In_ DWORD                       cHttpTraceConfiguration = 1
    ) = 0;

    virtual
    HRESULT
    RaiseTraceEvent(
        _In_ HTTP_TRACE_EVENT * pTraceEvent
    ) = 0;

    virtual
    LPCGUID
    GetTraceActivityId(
    ) = 0;

    virtual
    HRESULT
    QuickTrace(
        _In_ PCWSTR         pszData1,
        _In_opt_ PCWSTR     pszData2 = NULL,
        _In_ HRESULT        hrLastError = S_OK,
        //
        // 4 == TRACE_LEVEL_INFORMATION
        //
        _In_ UCHAR          Level = 4
    ) = 0;
};

class __declspec(uuid("37776aff-852e-4eec-93a5-b85a285a95b8"))
IHttpCacheSpecificData;

//
// Cache helpers
//
class __declspec(uuid("cdef2aad-20b3-4512-b1b1-094b3844aeb2"))
IHttpCacheKey
{
public:
    virtual
    DWORD
    GetHash(
        VOID
    ) const = 0;

    virtual
    PCWSTR
    GetCacheName(
        VOID
    ) const = 0;

    virtual
    bool
    GetIsEqual(
        _In_ IHttpCacheKey *            pCacheCompareKey
    ) const = 0;

    virtual
    bool
    GetIsPrefix(
        _In_ IHttpCacheKey *            pCacheCompareKey
    ) const = 0;

    virtual
    VOID
    Enum(
        _In_ IHttpCacheSpecificData *   pCacheData
    ) = 0;
};

class __declspec(uuid("37776aff-852e-4eec-93a5-b85a285a95b8"))
IHttpCacheSpecificData
{
public:
    virtual
    IHttpCacheKey *
    GetCacheKey(
        VOID
    ) const = 0;

    virtual
    VOID
    ReferenceCacheData(
        VOID
    ) = 0;

    virtual
    VOID
    DereferenceCacheData(
        VOID
    ) = 0;

    virtual
    VOID
    ResetTTL(
        VOID
    ) = 0;

    virtual
    VOID
    DecrementTTL(
        _Out_ BOOL *    pfTTLExpired
    ) = 0;

    virtual
    VOID
    SetFlushed(
        VOID
    ) = 0;

    virtual
    BOOL
    GetFlushed(
        VOID
    ) const = 0;
};

// 
// Site descriptor
// 
class __declspec(uuid("eb16a6ec-ba5d-436f-bf24-3ede13906450"))
IHttpSite
{
public:
    virtual
    DWORD
    GetSiteId(
        VOID
    ) const = 0;

    virtual
    PCWSTR
    GetSiteName(
        VOID
    ) const = 0;

    virtual
    IHttpModuleContextContainer *
    GetModuleContextContainer(
        VOID
    ) = 0;

    virtual
    IHttpPerfCounterInfo *
    GetPerfCounterInfo(
        VOID
    ) = 0;
};

//
// File change monitor
//
class __declspec(uuid("985422da-b0cf-473b-ba9e-8148ceb3e240"))
IHttpFileMonitor
{
public:
    virtual
    IHttpModuleContextContainer *
    GetModuleContextContainer(
        VOID
    ) = 0;

    virtual
    VOID
    DereferenceFileMonitor(
        VOID
    ) = 0;
};

//
// File descriptor
// 
class __declspec(uuid("71e95595-8c74-44d9-88a9-f5112d5f5900"))
IHttpFileInfo : public IHttpCacheSpecificData
{
public:
    virtual
    DWORD
    GetAttributes(
        VOID
    ) const = 0;

    virtual
    VOID
    GetSize(
        _Out_ ULARGE_INTEGER *  pliSize
    ) const = 0;

    virtual
    const BYTE *
    GetFileBuffer(
        VOID
    ) const = 0;

    virtual
    HANDLE
    GetFileHandle(
        VOID
    ) const = 0;

    virtual
    PCWSTR
    GetFilePath(
        VOID
    ) const = 0;

    virtual
    PCSTR
    GetETag(
        _Out_ USHORT *          pcchETag = NULL
    ) const = 0;

    virtual
    VOID
    GetLastModifiedTime(
        _Out_ FILETIME *        pFileTime
    ) const = 0;

    virtual
    PCSTR
    GetLastModifiedString(
        VOID
    ) const = 0;

    virtual
    BOOL
    GetHttpCacheAllowed(
        _Out_ DWORD *           pSecondsToLive
    ) const = 0;

    virtual
    HRESULT
    AccessCheck(
        _In_ HANDLE             hUserToken,
        _In_ PSID               pUserSid
    ) = 0;

    virtual
    HANDLE
    GetVrToken(
        VOID
    ) const = 0;

    virtual
    PCWSTR
    GetVrPath(
        VOID
    ) const = 0;

    virtual
    IHttpModuleContextContainer *
    GetModuleContextContainer(
        VOID
    ) = 0;

    virtual
    BOOL
    CheckIfFileHasChanged(
        _In_ HANDLE             hUserToken
    ) = 0;
};


// 
// Token-cache entry
// 
class __declspec(uuid("fd86e6de-fb0e-47dd-820a-e0da12be46e9"))
IHttpTokenEntry : public IHttpCacheSpecificData
{
public:
    virtual
    HANDLE
    GetImpersonationToken(
        VOID
    ) = 0;

    virtual
    HANDLE
    GetPrimaryToken(
        VOID
    ) = 0;

    virtual
    PSID
    GetSid(
        VOID
    ) = 0;
};


//
// IHttpServer extended interface versions
//
enum HTTP_SERVER_INTERFACE_VERSION
{
    HTTP_SERVER_INTERFACE_V2,
    HTTP_SERVER_INTERFACE_V3
};


//
// Global utility descriptor
//
class __declspec(uuid("eda2a40f-fb92-4d6d-b52b-c8c207380b4e"))
IHttpServer
{
public:
    virtual
    BOOL
    IsCommandLineLaunch(
        VOID
    ) const = 0;

    virtual
    PCWSTR
    GetAppPoolName(
        VOID
    ) const = 0;

    virtual
    HRESULT
    AssociateWithThreadPool(
        _In_ HANDLE                             hHandle,
        _In_ LPOVERLAPPED_COMPLETION_ROUTINE    completionRoutine
    ) = 0;

    virtual
    VOID
    IncrementThreadCount(
        VOID
    ) = 0;

    virtual
    VOID
    DecrementThreadCount(
        VOID
    ) = 0;

    virtual
    VOID
    ReportUnhealthy(
        _In_ PCWSTR                 pszReasonString,
        _In_ HRESULT                hrReason
    ) = 0;

    virtual
    VOID
    RecycleProcess(
        _In_ PCWSTR                 pszReason
    ) = 0;

    virtual
    IAppHostAdminManager *
    GetAdminManager(
        VOID
    ) const = 0;

    virtual
    HRESULT
    GetFileInfo(
        _In_ PCWSTR                 pszPhysicalPath,
        _In_ HANDLE                 hUserToken,
        _In_ PSID                   pSid,
        _In_ PCWSTR                 pszChangeNotificationPath,
        _In_ HANDLE                 hChangeNotificationToken,
        _In_ BOOL                   fCache,
        _Outptr_
        IHttpFileInfo **            ppFileInfo,
        _In_opt_
        IHttpTraceContext *         pHttpTraceContext = NULL
    ) = 0;

    virtual
    HRESULT
    FlushKernelCache(
        _In_ PCWSTR                 pszUrl
    ) = 0;

    virtual
    HRESULT
    DoCacheOperation(
        _In_ CACHE_OPERATION        cacheOperation,
        _In_ IHttpCacheKey *        pCacheKey,
        _Outptr_
        IHttpCacheSpecificData **   ppCacheSpecificData,
        _In_opt_
        IHttpTraceContext *         pHttpTraceContext = NULL
    ) = 0;

    virtual
    GLOBAL_NOTIFICATION_STATUS
    NotifyCustomNotification(
        _In_ ICustomNotificationProvider * pCustomOutput
    ) = 0;

    virtual
    IHttpPerfCounterInfo *
    GetPerfCounterInfo(
        VOID
    ) = 0;

    virtual
    VOID
    RecycleApplication(
        _In_ PCWSTR             pszAppConfigPath
    ) = 0;

    virtual
    VOID
    NotifyConfigurationChange(
        _In_ PCWSTR             pszPath
    ) = 0;

    virtual
    VOID
    NotifyFileChange(
        _In_ PCWSTR             pszFileName
    ) = 0;

    virtual
    IDispensedHttpModuleContextContainer *
    DispenseContainer(
        VOID
    ) = 0;

    virtual
    HRESULT
    AddFragmentToCache(
        _In_ HTTP_DATA_CHUNK *  pDataChunk,
        _In_ PCWSTR             pszFragmentName
    ) = 0;

    virtual
    HRESULT
    ReadFragmentFromCache(
        _In_ PCWSTR             pszFragmentName,
        _Out_writes_bytes_all_(cbSize)
        BYTE *                  pvBuffer,
        _In_ DWORD              cbSize,
        _Out_ DWORD *           pcbCopied
    ) = 0;

    virtual
    HRESULT
    RemoveFragmentFromCache(
        _In_ PCWSTR             pszFragmentName
    ) = 0;

    virtual
    HRESULT
    GetWorkerProcessSettings(
        _Outptr_
        IWpfSettings **         ppWorkerProcessSettings
    ) = 0;

    virtual
    HRESULT
    GetProtocolManagerCustomInterface(
        _In_ PCWSTR             pProtocolManagerDll,
        _In_ PCWSTR             pProtocolManagerDllInitFunction,
        _In_ DWORD              dwCustomInterfaceId,
        _Outptr_ PVOID*      ppCustomInterface
    ) = 0;

    virtual
    BOOL
    SatisfiesPrecondition(
        _In_ PCWSTR             pszPrecondition,
        _Out_ BOOL *            pfUnknownPrecondition = NULL
    ) const = 0;

    virtual
    IHttpTraceContext *
    GetTraceContext(
        VOID
    ) const = 0;

    virtual
    HRESULT
    RegisterFileChangeMonitor(
        _In_ PCWSTR             pszPath,
        _In_ HANDLE             hToken,
        _Outptr_
        IHttpFileMonitor **     ppFileMonitor
    ) = 0;

    virtual
    HRESULT
    GetExtendedInterface(
        _In_ HTTP_SERVER_INTERFACE_VERSION version,
        _Outptr_ PVOID * ppInterface
    ) = 0;
};

class __declspec(uuid("34af637e-afe8-4556-bcc1-767f8e0b4a4e"))
IHttpServer2 : public IHttpServer
{
public:

    virtual
    HRESULT
    GetToken(
        _In_ PCWSTR                     pszUserName,
        _In_ PCWSTR                     pszPassword,
        _In_ DWORD                      dwLogonMethod,
        _Outptr_
        IHttpTokenEntry **              ppTokenEntry,
        _In_opt_ PCWSTR                 pszDefaultDomain = NULL,
        _In_opt_ PSOCKADDR              pSockAddr = NULL,
        _In_opt_ IHttpTraceContext *    pHttpTraceContext = NULL
    ) = 0;

    virtual
    _Ret_writes_(*pcchConfigFilePath)
    PCWSTR
    GetAppPoolConfigFile(
        _Out_ DWORD * pcchConfigFilePath = NULL
    ) const = 0;

    virtual
    HRESULT
    GetExtendedInterface(
        _In_ const GUID &       Version1,
        _In_ PVOID              pInput,
        _In_ const GUID &       Version2,
        _Outptr_ PVOID *     ppOutput
    ) = 0;

    virtual
    HRESULT
    GetMetadata(
        _In_ PCWSTR                 pszSiteName,
        _In_ PCWSTR                 pszUrl,
        _Outptr_
        IReferencedMetadataInfo  ** ppMetadataInfo
    ) = 0;

    virtual
    HRESULT
    GetWorkerProcessFramework(
        _Outptr_
        IWorkerProcessFramework **  ppWorkerProcessFramework
    ) = 0;
};

class __declspec(uuid("f7a66d59-c0a2-48b5-a88f-be5975c960fb"))
IHttpServer3 : public IHttpServer2
{
public:

    virtual
    HRESULT
    QueryRequestProperty(
        _In_ HTTP_OPAQUE_ID id,
        _In_ HTTP_REQUEST_PROPERTY propertyId,
        _In_reads_bytes_opt_(qualifierSize) PVOID pQualifier,
        _In_ ULONG qualifierSize,
        _Out_writes_bytes_to_opt_(outputBufferSize, *pcbBytesReturned) PVOID pOutput,
        _In_ ULONG outputBufferSize,
        _Out_opt_ PULONG pcbBytesReturned,
        _In_ LPOVERLAPPED pOverlapped
    ) = 0;
};

//
// Helper function to get extended HTTP interfaces.
//
// Template parameters (HttpType1 and HttpType2)
// can be deduced from the arguments to the function.
//
// Example:
//
//   IHttpRequest * pHttpRequest = pHttpContext->GetRequest();
//   IHttpRequest2 * pHttpRequest2;
//   HRESULT hr = HttpGetExtendedInterface(g_pHttpServer, pHttpRequest, &pHttpRequest2);
//   if( SUCCEEDED(hr) )
//   {
//      // Use pHttpRequest2.
//   }
//
// Where pHttpContext is an IHttpContext pointer and
// g_pHttpServer is an IHttpServer pointer.
//

template <class HttpType1, class HttpType2>
HRESULT
HttpGetExtendedInterface(
    _In_ IHttpServer *          pHttpServer,
    _In_ HttpType1 *            pInput,
    _Outptr_ HttpType2 **    ppOutput
)
{
    HRESULT         hr;
    IHttpServer2 *  pHttpServer2;
    hr = pHttpServer->GetExtendedInterface(HTTP_SERVER_INTERFACE_V2,
                                           reinterpret_cast<void**>(&pHttpServer2) );
    if (SUCCEEDED(hr))
    {
        hr = pHttpServer2->GetExtendedInterface(__uuidof(HttpType1),
                                                pInput,
                                                __uuidof(HttpType2),
                                                reinterpret_cast<void**>(ppOutput) );
    }
    return hr;
}

//
// Notification specific output for notifications
//

class __declspec(uuid("6f3f657d-2fb8-43c6-a096-5064b41f0580"))
IHttpEventProvider
{
public:
    virtual
    VOID
    SetErrorStatus(
        _In_ HRESULT            hrError
    ) = 0;
};

//
// Completion information for notifications
//
class __declspec(uuid("49dd20e3-d9c0-463c-8821-f3413b55cc00"))
IHttpCompletionInfo
{
public:
    virtual
    DWORD
    GetCompletionBytes(
        VOID
    ) const = 0;

    virtual
    HRESULT
    GetCompletionStatus(
        VOID
    ) const = 0;
};

//
// Extended completion information available in Windows 8 and newer
//
class __declspec(uuid("53129f70-0196-47ec-b16c-127906e2afeb"))
IHttpCompletionInfo2 : public IHttpCompletionInfo
{
public:
    virtual
    DWORD
    GetCompletedOperation(
        VOID
    ) const = 0;
};

//
// RQ_ and GL_ CUSTOM_NOTIFICATION outputs
//
class __declspec(uuid("671e6d34-9380-4df4-b453-91129df02b24"))
ICustomNotificationProvider : public IHttpEventProvider
{
public:
    virtual
    PCWSTR
    QueryNotificationType(
        VOID
    ) = 0;
};

//
// RQ_REQUEST_AUTHENTICATE descriptor
//
class __declspec(uuid("304d51d0-0307-45ed-83fd-dd3fc032fdfc"))
IAuthenticationProvider : public IHttpEventProvider
{
public:
    virtual
    VOID
    SetUser(
        _In_ IHttpUser *        pUser
    ) = 0;
};

//
// RQ_MAP_REQUEST_HANDLER
//
class __declspec(uuid("fea3ce6b-e346-47e7-b2a6-ad265baeff2c"))
IMapHandlerProvider : public IHttpEventProvider
{
public:
    virtual
    HRESULT
    SetScriptName(
        _In_reads_(cchScriptName)
        PCWSTR                  pszScriptName,
        _In_ DWORD              cchScriptName
    ) = 0;

    virtual
    VOID
    SetScriptMap(
        _In_ IScriptMapInfo *   pScriptMap
    ) = 0;

    virtual
    VOID
    SetFileInfo(
        _In_ IHttpFileInfo *    pFileInfo
    ) = 0;
};

//
// RQ_MAP_PATH
//
class __declspec(uuid("8efdf557-a8f1-4bc9-b462-6df3b038a59a"))
IMapPathProvider : public IHttpEventProvider
{
public:
    virtual
    PCWSTR
    GetUrl(
        VOID
    ) const = 0;

    virtual
    PCWSTR
    GetPhysicalPath(
        VOID
    ) const = 0;

    virtual
    HRESULT
    SetPhysicalPath(
        _In_reads_(cchPhysicalPath)
        PCWSTR                  pszPhysicalPath,
        _In_ DWORD              cchPhysicalPath
    ) = 0;
};

//
// RQ_SEND_RESPONSE
//
class __declspec(uuid("57f2e7bc-0bcf-4a9f-94a4-10e55c6e5b51"))
ISendResponseProvider : public IHttpEventProvider
{
public:
    virtual
    BOOL
    GetHeadersBeingSent(
        VOID
    ) const = 0;

    virtual
    DWORD
    GetFlags(
        VOID
    ) const = 0;

    virtual
    VOID
    SetFlags(
        _In_ DWORD dwFlags
    ) = 0;

    virtual
    HTTP_LOG_DATA *
    GetLogData(
        VOID
    ) const = 0;

    virtual
    HRESULT
    SetLogData(
        _In_ HTTP_LOG_DATA *    pLogData
    ) = 0;

    virtual
    BOOL
    GetReadyToLogData(
        VOID
    ) const = 0;
};

//
// RQ_READ_ENTITY
//
class __declspec(uuid("fe6d905a-99b8-49fd-b389-cfc809562b81"))
IReadEntityProvider : public IHttpEventProvider
{
public:
    virtual
    VOID
    GetEntity(
        _Outptr_result_bytebuffer_to_(*pcbBuffer, *pcbData)
        PVOID *                 ppBuffer,
        _Out_ DWORD *           pcbData,
        _Out_ DWORD *           pcbBuffer
    ) = 0;

    virtual
    VOID
    SetEntity(
        _In_reads_(cbBuffer)
        PVOID                   pBuffer,
        _In_ DWORD              cbData,
        _In_ DWORD              cbBuffer
    ) = 0;
};

//
// GL_PRE_BEGIN_REQUEST provider
//
class __declspec(uuid("fb715d26-aff9-476a-8fc0-6b1acb3d1098"))
IPreBeginRequestProvider : public IHttpEventProvider
{
public:
    virtual
    IHttpContext *
    GetHttpContext(
        VOID
    ) = 0;
};

//
// GL_APPLICATION_START provider
//
class __declspec(uuid("1de2c71c-c126-4512-aed3-f4f885e14997"))
IHttpApplicationProvider : public IHttpEventProvider
{
public:
    virtual
    IHttpApplication *
    GetApplication(
        VOID
    ) = 0;
};

typedef IHttpApplicationProvider    IHttpApplicationStartProvider;

class __declspec(uuid("ba32d330-9ea8-4b9e-89f1-8c76a323277f"))
IHttpModuleFactory;

//
// GL_APPLICATION_RESOLVE_MODULES provider
//
class __declspec(uuid("0617d9b9-e20f-4a9f-94f9-35403b3be01e"))
IHttpApplicationResolveModulesProvider : public IHttpApplicationProvider
{
public:
    virtual 
    HRESULT
    RegisterModule(
        _In_ HTTP_MODULE_ID       parentModuleId,
        _In_ IHttpModuleFactory * pModuleFactory,
        _In_ PCWSTR               pszModuleName,
        _In_ PCWSTR               pszModuleType,
        _In_ PCWSTR               pszModulePreCondition,
        _In_ DWORD                dwRequestNotifications,
        _In_ DWORD                dwPostRequestNotifications
    ) = 0;

    virtual
    HRESULT
    SetPriorityForRequestNotification(
        _In_ PCWSTR               pszModuleName,
        _In_ DWORD                dwRequestNotification,
        _In_ PCWSTR               pszPriorityAlias
    ) = 0;
};

//
// GL_APPLICATION_STOP provider
//
typedef IHttpApplicationProvider   IHttpApplicationStopProvider;

//
// GL_RSCA_QUERY provider
//
class __declspec(uuid("63fdc43f-934a-4ee5-bcd8-7e7b50b75605"))
IGlobalRSCAQueryProvider : public IHttpEventProvider
{
public:
    virtual
    PCWSTR
    GetFunctionName(
        VOID
    ) const = 0;

    virtual
    PCWSTR
    GetFunctionParameters(
        VOID
    ) const = 0;

    virtual
    
    HRESULT
    GetOutputBuffer(
        _In_ DWORD          cbBuffer,
        _Outptr_result_bytebuffer_to_(cbBuffer, 0)
        BYTE **             ppbBuffer
    ) = 0;

    virtual
    HRESULT
    ResizeOutputBuffer(
        _In_ DWORD          cbNewBuffer,
        _In_ _In_range_(<=, cbNewBuffer)
        DWORD               cbBytesToCopy,
        _Inout_ _At_(*ppbBuffer, _Pre_readable_size_(cbBytesToCopy) _Post_readable_size_(cbBytesToCopy) _Post_writable_size_(cbNewBuffer))
        BYTE **             ppbBuffer
    ) = 0;

    virtual
    VOID
    SetResult(
        _In_ DWORD          cbData,
        _In_ HRESULT        hr
    ) = 0;
};

//
// GL_STOP_LISTENING
//
class __declspec(uuid("41f9a601-e25d-4ac8-8a1f-635698a30ab9"))
IGlobalStopListeningProvider : public IHttpEventProvider
{
public:
    virtual
    BOOL
    DrainRequestsGracefully(
        VOID
    ) const = 0;
};

//
// GL_CACHE_OPERATION
//
class __declspec(uuid("58925fb9-7c5e-4684-833b-4a04e1286690"))
ICacheProvider : public IHttpEventProvider
{
public:
    virtual
    CACHE_OPERATION
    GetCacheOperation(
        VOID
    ) const = 0;

    virtual
    IHttpCacheKey *
    GetCacheKey(
        VOID
    ) const = 0;

    virtual
    IHttpCacheSpecificData *
    GetCacheRecord(
        VOID
    ) const = 0;

    virtual
    VOID
    SetCacheRecord(
        _In_ IHttpCacheSpecificData *   pCacheRecord
    ) = 0;

    virtual
    IHttpTraceContext *
    GetTraceContext(
        VOID
    ) const = 0;
};

//
// GL_CONFIGURATION_CHANGE
//
class __declspec(uuid("3405f3b4-b3d6-4b73-b5f5-4d8a3cc642ce"))
IGlobalConfigurationChangeProvider : public IHttpEventProvider
{
public:
    virtual
    PCWSTR
    GetChangePath(
        VOID
    ) const = 0;
};

//
// GL_FILE_CHANGE
//
class __declspec(uuid("ece31ee5-0486-4fb0-a875-6739a2d7daf5"))
IGlobalFileChangeProvider : public IHttpEventProvider
{
public:
    virtual
    PCWSTR
    GetFileName(
        VOID
    ) const = 0;

    virtual
    IHttpFileMonitor *
    GetFileMonitor(
        VOID
    ) = 0;
};

//
// GL_TRACE_EVENT
//
class __declspec(uuid("7c6bb150-0310-4718-a01f-6faceb62dc1d"))
IGlobalTraceEventProvider : public IHttpEventProvider
{
public:
    virtual
    HRESULT
    GetTraceEvent(
        _Outptr_
        HTTP_TRACE_EVENT **     ppTraceEvent
    ) = 0;

    virtual
    BOOL
    CheckSubscription(
        _In_ HTTP_MODULE_ID     ModuleId
    ) = 0;     

    virtual
    HRESULT 
    GetCurrentHttpRequestContext(
        _Outptr_
        IHttpContext **         ppHttpContext
    ) = 0;
};

//
// GL_THREAD_CLEANUP
//
class __declspec(uuid("6b36a149-8620-45a0-8197-00814a706e2e"))
IGlobalThreadCleanupProvider : public IHttpEventProvider
{
public:
    virtual
    IHttpApplication *
    GetApplication(
        VOID
    ) = 0;
};

//
// GL_APPLICATION_PRELOAD
//
class __declspec(uuid("2111f8d6-0c41-4ff7-bd45-5c04c7e91a73"))
IGlobalApplicationPreloadProvider : public IHttpEventProvider
{
public:
    virtual
    HRESULT
    CreateContext(
        _Outptr_ IHttpContext ** ppHttpContext
    ) = 0;

    virtual
    HRESULT
    ExecuteRequest(
        _In_ IHttpContext *         pHttpContext,
        _In_ IHttpUser *            pHttpUser
    ) = 0;
};

//
// GL_APPLICATION_PRELOAD
// IGlobalApplicationPreloadProvider2 is available on Windows 8 and newer
//

class __declspec(uuid("251db2dd-81e7-44ce-ae02-46529f2d71ea"))
IGlobalApplicationPreloadProvider2 : public IGlobalApplicationPreloadProvider
{
public:
    virtual
    BOOL
    IsProcessRecycled(
        VOID
    ) const = 0;
};

//
// GL_SUSPEND_PROCESS
//
class __declspec(uuid("7fd8f303-1031-4e50-918a-b7b916fd6c54"))
IGlobalSuspendProcessCallback : public IHttpEventProvider
{
public:
    virtual
    HRESULT
    Resume(
        VOID
    ) = 0;
};

class CHttpModule
{
public:
    // RQ_BEGIN_REQUEST

    virtual 
    REQUEST_NOTIFICATION_STATUS
    OnBeginRequest(
        _In_ IHttpContext *         pHttpContext,
        _In_ IHttpEventProvider *   pProvider
    )
    {
        UNREFERENCED_PARAMETER( pHttpContext );
        UNREFERENCED_PARAMETER( pProvider );
        OutputDebugStringA(
            "This module subscribed to event ");
        OutputDebugStringA(__FUNCTION__);
        OutputDebugStringA(
            " but did not override the method in its CHttpModule implementation."
            "  Please check the method signature to make sure it matches the corresponding method.\n");
        DebugBreak();
        
        return RQ_NOTIFICATION_CONTINUE;
    }

    virtual 
    REQUEST_NOTIFICATION_STATUS
    OnPostBeginRequest(
        _In_ IHttpContext *         pHttpContext,
        _In_ IHttpEventProvider *   pProvider
    )
    {
        UNREFERENCED_PARAMETER( pHttpContext );
        UNREFERENCED_PARAMETER( pProvider );
        OutputDebugStringA(
            "This module subscribed to event ");
        OutputDebugStringA(__FUNCTION__);
        OutputDebugStringA(
            " but did not override the method in its CHttpModule implementation."
            "  Please check the method signature to make sure it matches the corresponding method.\n");
        DebugBreak();
        
        return RQ_NOTIFICATION_CONTINUE;
    }


    // RQ_AUTHENTICATE_REQUEST

    virtual 
    REQUEST_NOTIFICATION_STATUS
    OnAuthenticateRequest(
        _In_ IHttpContext *             pHttpContext,
        _In_ IAuthenticationProvider *  pProvider
    )
    {
        UNREFERENCED_PARAMETER( pHttpContext );
        UNREFERENCED_PARAMETER( pProvider );
        OutputDebugStringA(
            "This module subscribed to event ");
        OutputDebugStringA(__FUNCTION__);
        OutputDebugStringA(
            " but did not override the method in its CHttpModule implementation."
            "  Please check the method signature to make sure it matches the corresponding method.\n");
        DebugBreak();
        
        return RQ_NOTIFICATION_CONTINUE;
    }


    virtual 
    REQUEST_NOTIFICATION_STATUS
    OnPostAuthenticateRequest(
        _In_ IHttpContext *             pHttpContext,
        _In_ IHttpEventProvider *       pProvider
    )
    {
        UNREFERENCED_PARAMETER( pHttpContext );
        UNREFERENCED_PARAMETER( pProvider );
        OutputDebugStringA(
            "This module subscribed to event ");
        OutputDebugStringA(__FUNCTION__);
        OutputDebugStringA(
            " but did not override the method in its CHttpModule implementation."
            "  Please check the method signature to make sure it matches the corresponding method.\n");
        DebugBreak();
        
        return RQ_NOTIFICATION_CONTINUE;
    }


    // RQ_AUTHORIZE_REQUEST

    virtual 
    REQUEST_NOTIFICATION_STATUS
    OnAuthorizeRequest(
        _In_ IHttpContext *             pHttpContext,
        _In_ IHttpEventProvider *       pProvider
    )
    {
        UNREFERENCED_PARAMETER( pHttpContext );
        UNREFERENCED_PARAMETER( pProvider );
        OutputDebugStringA(
            "This module subscribed to event ");
        OutputDebugStringA(__FUNCTION__);
        OutputDebugStringA(
            " but did not override the method in its CHttpModule implementation."
            "  Please check the method signature to make sure it matches the corresponding method.\n");
        DebugBreak();
        
        return RQ_NOTIFICATION_CONTINUE;
    }


    virtual 
    REQUEST_NOTIFICATION_STATUS
    OnPostAuthorizeRequest(
        _In_ IHttpContext *             pHttpContext,
        _In_ IHttpEventProvider *       pProvider
    )
    {
        UNREFERENCED_PARAMETER( pHttpContext );
        UNREFERENCED_PARAMETER( pProvider );
        OutputDebugStringA(
            "This module subscribed to event ");
        OutputDebugStringA(__FUNCTION__);
        OutputDebugStringA(
            " but did not override the method in its CHttpModule implementation."
            "  Please check the method signature to make sure it matches the corresponding method.\n");
        DebugBreak();
        
        return RQ_NOTIFICATION_CONTINUE;
    }


    // RQ_RESOLVE_REQUEST_CACHE

    virtual 
    REQUEST_NOTIFICATION_STATUS
    OnResolveRequestCache(
        _In_ IHttpContext *             pHttpContext,
        _In_ IHttpEventProvider *       pProvider
    )
    {
        UNREFERENCED_PARAMETER( pHttpContext );
        UNREFERENCED_PARAMETER( pProvider );
        OutputDebugStringA(
            "This module subscribed to event ");
        OutputDebugStringA(__FUNCTION__);
        OutputDebugStringA(
            " but did not override the method in its CHttpModule implementation."
            "  Please check the method signature to make sure it matches the corresponding method.\n");
        DebugBreak();
        
        return RQ_NOTIFICATION_CONTINUE;
    }


    virtual 
    REQUEST_NOTIFICATION_STATUS
    OnPostResolveRequestCache(
        _In_ IHttpContext *             pHttpContext,
        _In_ IHttpEventProvider *       pProvider
    )
    {
        UNREFERENCED_PARAMETER( pHttpContext );
        UNREFERENCED_PARAMETER( pProvider );
        OutputDebugStringA(
            "This module subscribed to event ");
        OutputDebugStringA(__FUNCTION__);
        OutputDebugStringA(
            " but did not override the method in its CHttpModule implementation."
            "  Please check the method signature to make sure it matches the corresponding method.\n");
        DebugBreak();
        
        return RQ_NOTIFICATION_CONTINUE;
    }


    // RQ_MAP_REQUEST_HANDLER

    virtual 
    REQUEST_NOTIFICATION_STATUS
    OnMapRequestHandler(
        _In_ IHttpContext *             pHttpContext,
        _In_ IMapHandlerProvider *      pProvider
    )
    {
        UNREFERENCED_PARAMETER( pHttpContext );
        UNREFERENCED_PARAMETER( pProvider );
        OutputDebugStringA(
            "This module subscribed to event ");
        OutputDebugStringA(__FUNCTION__);
        OutputDebugStringA(
            " but did not override the method in its CHttpModule implementation."
            "  Please check the method signature to make sure it matches the corresponding method.\n");
        DebugBreak();
        
        return RQ_NOTIFICATION_CONTINUE;
    }


    virtual 
    REQUEST_NOTIFICATION_STATUS
    OnPostMapRequestHandler(
        _In_ IHttpContext *             pHttpContext,
        _In_ IHttpEventProvider *       pProvider
    )
    {
        UNREFERENCED_PARAMETER( pHttpContext );
        UNREFERENCED_PARAMETER( pProvider );
        OutputDebugStringA(
            "This module subscribed to event ");
        OutputDebugStringA(__FUNCTION__);
        OutputDebugStringA(
            " but did not override the method in its CHttpModule implementation."
            "  Please check the method signature to make sure it matches the corresponding method.\n");
        DebugBreak();
        
        return RQ_NOTIFICATION_CONTINUE;
    }


    // RQ_ACQUIRE_REQUEST_STATE

    virtual 
    REQUEST_NOTIFICATION_STATUS
    OnAcquireRequestState(
        _In_ IHttpContext *             pHttpContext,
        _In_ IHttpEventProvider *       pProvider
    )
    {
        UNREFERENCED_PARAMETER( pHttpContext );
        UNREFERENCED_PARAMETER( pProvider );
        OutputDebugStringA(
            "This module subscribed to event ");
        OutputDebugStringA(__FUNCTION__);
        OutputDebugStringA(
            " but did not override the method in its CHttpModule implementation."
            "  Please check the method signature to make sure it matches the corresponding method.\n");
        DebugBreak();
        
        return RQ_NOTIFICATION_CONTINUE;
    }


    virtual 
    REQUEST_NOTIFICATION_STATUS
    OnPostAcquireRequestState(
        _In_ IHttpContext *             pHttpContext,
        _In_ IHttpEventProvider *       pProvider
    )
    {
        UNREFERENCED_PARAMETER( pHttpContext );
        UNREFERENCED_PARAMETER( pProvider );
        OutputDebugStringA(
            "This module subscribed to event ");
        OutputDebugStringA(__FUNCTION__);
        OutputDebugStringA(
            " but did not override the method in its CHttpModule implementation."
            "  Please check the method signature to make sure it matches the corresponding method.\n");
        DebugBreak();
        
        return RQ_NOTIFICATION_CONTINUE;
    }


    // RQ_PRE_EXECUTE_REQUEST_HANDLER

    virtual 
    REQUEST_NOTIFICATION_STATUS
    OnPreExecuteRequestHandler(
        _In_ IHttpContext *             pHttpContext,
        _In_ IHttpEventProvider *       pProvider
    )
    {
        UNREFERENCED_PARAMETER( pHttpContext );
        UNREFERENCED_PARAMETER( pProvider );
        OutputDebugStringA(
            "This module subscribed to event ");
        OutputDebugStringA(__FUNCTION__);
        OutputDebugStringA(
            " but did not override the method in its CHttpModule implementation."
            "  Please check the method signature to make sure it matches the corresponding method.\n");
        DebugBreak();
        
        return RQ_NOTIFICATION_CONTINUE;
    }


    virtual 
    REQUEST_NOTIFICATION_STATUS
    OnPostPreExecuteRequestHandler(
        _In_ IHttpContext *             pHttpContext,
        _In_ IHttpEventProvider *       pProvider
    )
    {
        UNREFERENCED_PARAMETER( pHttpContext );
        UNREFERENCED_PARAMETER( pProvider );
        OutputDebugStringA(
            "This module subscribed to event ");
        OutputDebugStringA(__FUNCTION__);
        OutputDebugStringA(
            " but did not override the method in its CHttpModule implementation."
            "  Please check the method signature to make sure it matches the corresponding method.\n");
        DebugBreak();
        
        return RQ_NOTIFICATION_CONTINUE;
    }


    // RQ_EXECUTE_REQUEST_HANDLER

    virtual 
    REQUEST_NOTIFICATION_STATUS
    OnExecuteRequestHandler(
        _In_ IHttpContext *             pHttpContext,
        _In_ IHttpEventProvider *       pProvider
    )
    {
        UNREFERENCED_PARAMETER( pHttpContext );
        UNREFERENCED_PARAMETER( pProvider );
        OutputDebugStringA(
            "This module subscribed to event ");
        OutputDebugStringA(__FUNCTION__);
        OutputDebugStringA(
            " but did not override the method in its CHttpModule implementation."
            "  Please check the method signature to make sure it matches the corresponding method.\n");
        DebugBreak();
        
        return RQ_NOTIFICATION_CONTINUE;
    }


    virtual 
    REQUEST_NOTIFICATION_STATUS
    OnPostExecuteRequestHandler(
        _In_ IHttpContext *             pHttpContext,
        _In_ IHttpEventProvider *       pProvider
    )    
    {
        UNREFERENCED_PARAMETER( pHttpContext );
        UNREFERENCED_PARAMETER( pProvider );
        OutputDebugStringA(
            "This module subscribed to event ");
        OutputDebugStringA(__FUNCTION__);
        OutputDebugStringA(
            " but did not override the method in its CHttpModule implementation."
            "  Please check the method signature to make sure it matches the corresponding method.\n");
        DebugBreak();
        
        return RQ_NOTIFICATION_CONTINUE;
    }

    // RQ_RELEASE_REQUEST_STATE

    virtual 
    REQUEST_NOTIFICATION_STATUS
    OnReleaseRequestState(
        _In_ IHttpContext *             pHttpContext,
        _In_ IHttpEventProvider *       pProvider
    )
    {
        UNREFERENCED_PARAMETER( pHttpContext );
        UNREFERENCED_PARAMETER( pProvider );
        OutputDebugStringA(
            "This module subscribed to event ");
        OutputDebugStringA(__FUNCTION__);
        OutputDebugStringA(
            " but did not override the method in its CHttpModule implementation."
            "  Please check the method signature to make sure it matches the corresponding method.\n");
        DebugBreak();
        
        return RQ_NOTIFICATION_CONTINUE;
    }


    virtual 
    REQUEST_NOTIFICATION_STATUS
    OnPostReleaseRequestState(
        _In_ IHttpContext *             pHttpContext,
        _In_ IHttpEventProvider *       pProvider
    )
    {
        UNREFERENCED_PARAMETER( pHttpContext );
        UNREFERENCED_PARAMETER( pProvider );
        OutputDebugStringA(
            "This module subscribed to event ");
        OutputDebugStringA(__FUNCTION__);
        OutputDebugStringA(
            " but did not override the method in its CHttpModule implementation."
            "  Please check the method signature to make sure it matches the corresponding method.\n");
        DebugBreak();
        
        return RQ_NOTIFICATION_CONTINUE;
    }


    // RQ_UPDATE_REQUEST_CACHE

    virtual 
    REQUEST_NOTIFICATION_STATUS
    OnUpdateRequestCache(
        _In_ IHttpContext *             pHttpContext,
        _In_ IHttpEventProvider *       pProvider
    )
    {
        UNREFERENCED_PARAMETER( pHttpContext );
        UNREFERENCED_PARAMETER( pProvider );
        OutputDebugStringA(
            "This module subscribed to event ");
        OutputDebugStringA(__FUNCTION__);
        OutputDebugStringA(
            " but did not override the method in its CHttpModule implementation."
            "  Please check the method signature to make sure it matches the corresponding method.\n");
        DebugBreak();
        
        return RQ_NOTIFICATION_CONTINUE;
    }


    virtual 
    REQUEST_NOTIFICATION_STATUS
    OnPostUpdateRequestCache(
        _In_ IHttpContext *             pHttpContext,
        _In_ IHttpEventProvider *       pProvider
    )
    {
        UNREFERENCED_PARAMETER( pHttpContext );
        UNREFERENCED_PARAMETER( pProvider );
        OutputDebugStringA(
            "This module subscribed to event ");
        OutputDebugStringA(__FUNCTION__);
        OutputDebugStringA(
            " but did not override the method in its CHttpModule implementation."
            "  Please check the method signature to make sure it matches the corresponding method.\n");
        DebugBreak();
        
        return RQ_NOTIFICATION_CONTINUE;
    }

    // RQ_LOG_REQUEST

    virtual 
    REQUEST_NOTIFICATION_STATUS
    OnLogRequest(
        _In_ IHttpContext *             pHttpContext,
        _In_ IHttpEventProvider *       pProvider
    )
    {
        UNREFERENCED_PARAMETER( pHttpContext );
        UNREFERENCED_PARAMETER( pProvider );
        OutputDebugStringA(
            "This module subscribed to event ");
        OutputDebugStringA(__FUNCTION__);
        OutputDebugStringA(
            " but did not override the method in its CHttpModule implementation."
            "  Please check the method signature to make sure it matches the corresponding method.\n");
        DebugBreak();
        
        return RQ_NOTIFICATION_CONTINUE;
    }

    virtual 
    REQUEST_NOTIFICATION_STATUS
    OnPostLogRequest(
        _In_ IHttpContext *             pHttpContext,
        _In_ IHttpEventProvider *       pProvider
    )
    {
        UNREFERENCED_PARAMETER( pHttpContext );
        UNREFERENCED_PARAMETER( pProvider );
        OutputDebugStringA(
            "This module subscribed to event ");
        OutputDebugStringA(__FUNCTION__);
        OutputDebugStringA(
            " but did not override the method in its CHttpModule implementation."
            "  Please check the method signature to make sure it matches the corresponding method.\n");
        DebugBreak();

        return RQ_NOTIFICATION_CONTINUE;
    }

    // RQ_END_REQUEST

    virtual 
    REQUEST_NOTIFICATION_STATUS
    OnEndRequest(
        _In_ IHttpContext *             pHttpContext,
        _In_ IHttpEventProvider *       pProvider
    )
    {
        UNREFERENCED_PARAMETER( pHttpContext );
        UNREFERENCED_PARAMETER( pProvider );
        OutputDebugStringA(
            "This module subscribed to event ");
        OutputDebugStringA(__FUNCTION__);
        OutputDebugStringA(
            " but did not override the method in its CHttpModule implementation."
            "  Please check the method signature to make sure it matches the corresponding method.\n");
        DebugBreak();

        return RQ_NOTIFICATION_CONTINUE;
    }

    virtual 
    REQUEST_NOTIFICATION_STATUS
    OnPostEndRequest(
        _In_ IHttpContext *             pHttpContext,
        _In_ IHttpEventProvider *       pProvider
    )
    {
        UNREFERENCED_PARAMETER( pHttpContext );
        UNREFERENCED_PARAMETER( pProvider );
        OutputDebugStringA(
            "This module subscribed to event ");
        OutputDebugStringA(__FUNCTION__);
        OutputDebugStringA(
            " but did not override the method in its CHttpModule implementation."
            "  Please check the method signature to make sure it matches the corresponding method.\n");
        DebugBreak();
        
        return RQ_NOTIFICATION_CONTINUE;
    }

    // RQ_SEND_RESPONSE

    virtual 
    REQUEST_NOTIFICATION_STATUS
    OnSendResponse(
        _In_ IHttpContext *             pHttpContext,
        _In_ ISendResponseProvider *    pProvider
    )
    {
        UNREFERENCED_PARAMETER( pHttpContext );
        UNREFERENCED_PARAMETER( pProvider );
        OutputDebugStringA(
            "This module subscribed to event ");
        OutputDebugStringA(__FUNCTION__);
        OutputDebugStringA(
            " but did not override the method in its CHttpModule implementation."
            "  Please check the method signature to make sure it matches the corresponding method.\n");
        DebugBreak();
        
        return RQ_NOTIFICATION_CONTINUE;
    }

    // RQ_MAP_PATH

    virtual 
    REQUEST_NOTIFICATION_STATUS
    OnMapPath(
        _In_ IHttpContext *             pHttpContext,
        _In_ IMapPathProvider *         pProvider
    )
    {
        UNREFERENCED_PARAMETER( pHttpContext );
        UNREFERENCED_PARAMETER( pProvider );
        OutputDebugStringA(
            "This module subscribed to event ");
        OutputDebugStringA(__FUNCTION__);
        OutputDebugStringA(
            " but did not override the method in its CHttpModule implementation."
            "  Please check the method signature to make sure it matches the corresponding method.\n");
        DebugBreak();
        
        return RQ_NOTIFICATION_CONTINUE;
    }

    // RQ_READ_ENTITY

    virtual 
    REQUEST_NOTIFICATION_STATUS
    OnReadEntity(
        _In_ IHttpContext *                 pHttpContext,
        _In_ IReadEntityProvider *          pProvider
    )
    {
        UNREFERENCED_PARAMETER( pHttpContext );
        UNREFERENCED_PARAMETER( pProvider );
        OutputDebugStringA(
            "This module subscribed to event ");
        OutputDebugStringA(__FUNCTION__);
        OutputDebugStringA(
            " but did not override the method in its CHttpModule implementation."
            "  Please check the method signature to make sure it matches the corresponding method.\n");
        DebugBreak();
        
        return RQ_NOTIFICATION_CONTINUE;
    }

    // RQ_CUSTOM_NOTIFICATION

    virtual 
    REQUEST_NOTIFICATION_STATUS
    OnCustomRequestNotification(
        _In_ IHttpContext *                 pHttpContext,
        _In_ ICustomNotificationProvider *  pProvider
    )
    {
        UNREFERENCED_PARAMETER( pHttpContext );
        UNREFERENCED_PARAMETER( pProvider );
        OutputDebugStringA(
            "This module subscribed to event ");
        OutputDebugStringA(__FUNCTION__);
        OutputDebugStringA(
            " but did not override the method in its CHttpModule implementation."
            "  Please check the method signature to make sure it matches the corresponding method.\n");
        DebugBreak();
        
        return RQ_NOTIFICATION_CONTINUE;
    }

    // Completion

    virtual 
    REQUEST_NOTIFICATION_STATUS
    OnAsyncCompletion(
        _In_ IHttpContext *             pHttpContext,
        _In_ DWORD                      dwNotification,
        _In_ BOOL                       fPostNotification,
        _In_ IHttpEventProvider *       pProvider,
        _In_ IHttpCompletionInfo *      pCompletionInfo
    )    
    {
        UNREFERENCED_PARAMETER( pHttpContext );
        UNREFERENCED_PARAMETER( dwNotification );
        UNREFERENCED_PARAMETER( fPostNotification );
        UNREFERENCED_PARAMETER( pProvider );
        UNREFERENCED_PARAMETER( pCompletionInfo );
        OutputDebugStringA(
            "This module subscribed to event ");
        OutputDebugStringA(__FUNCTION__);
        OutputDebugStringA(
            " but did not override the method in its CHttpModule implementation."
            "  Please check the method signature to make sure it matches the corresponding method.\n");
        DebugBreak();
        
        return RQ_NOTIFICATION_CONTINUE;
    }

    virtual
    VOID
    Dispose(
        VOID
    )
    {
        delete this;
    }

 protected:

    CHttpModule()
    {}

    virtual
    ~CHttpModule()
    {}
};

class CGlobalModule
{
public:

    // GL_STOP_LISTENING

    virtual 
    GLOBAL_NOTIFICATION_STATUS
    OnGlobalStopListening(
        _In_ IGlobalStopListeningProvider * pProvider
    )
    {
        UNREFERENCED_PARAMETER( pProvider );
        OutputDebugStringA(
            "This module subscribed to event ");
        OutputDebugStringA(__FUNCTION__);
        OutputDebugStringA(
            " but did not override the method in its CGlobalModule implementation."
            "  Please check the method signature to make sure it matches the corresponding method.\n");
        DebugBreak();

        return GL_NOTIFICATION_CONTINUE;
    }

    // GL_CACHE_CLEANUP
    
    virtual 
    GLOBAL_NOTIFICATION_STATUS
    OnGlobalCacheCleanup(
        VOID
    )
    {
        OutputDebugStringA(
            "This module subscribed to event ");
        OutputDebugStringA(__FUNCTION__);
        OutputDebugStringA(
            " but did not override the method in its CGlobalModule implementation."
            "  Please check the method signature to make sure it matches the corresponding method.\n");
        DebugBreak();

        return GL_NOTIFICATION_CONTINUE;
    }

    // GL_CACHE_OPERATION
    
    virtual 
    GLOBAL_NOTIFICATION_STATUS
    OnGlobalCacheOperation(
        _In_ ICacheProvider * pProvider
    )
    {
        UNREFERENCED_PARAMETER( pProvider );
        OutputDebugStringA(
            "This module subscribed to event ");
        OutputDebugStringA(__FUNCTION__);
        OutputDebugStringA(
            " but did not override the method in its CGlobalModule implementation."
            "  Please check the method signature to make sure it matches the corresponding method.\n");
        DebugBreak();

        return GL_NOTIFICATION_CONTINUE;
    }

    // GL_HEALTH_CHECK
    
    virtual 
    GLOBAL_NOTIFICATION_STATUS
    OnGlobalHealthCheck(
        VOID
    )
    {
        OutputDebugStringA(
            "This module subscribed to event ");
        OutputDebugStringA(__FUNCTION__);
        OutputDebugStringA(
            " but did not override the method in its CGlobalModule implementation."
            "  Please check the method signature to make sure it matches the corresponding method.\n");
        DebugBreak();

        return GL_NOTIFICATION_CONTINUE;
    }

    // GL_CONFIGURATION_CHANGE
    
    virtual 
    GLOBAL_NOTIFICATION_STATUS
    OnGlobalConfigurationChange(
        _In_ IGlobalConfigurationChangeProvider * pProvider
    )
    {
        UNREFERENCED_PARAMETER( pProvider );
        OutputDebugStringA(
            "This module subscribed to event ");
        OutputDebugStringA(__FUNCTION__);
        OutputDebugStringA(
            " but did not override the method in its CGlobalModule implementation."
            "  Please check the method signature to make sure it matches the corresponding method.\n");
        DebugBreak();

        return GL_NOTIFICATION_CONTINUE;
    }

    // GL_FILE_CHANGE 
    
    virtual 
    GLOBAL_NOTIFICATION_STATUS
    OnGlobalFileChange(
        _In_ IGlobalFileChangeProvider * pProvider
    )
    {
        UNREFERENCED_PARAMETER( pProvider );
        OutputDebugStringA(
            "This module subscribed to event ");
        OutputDebugStringA(__FUNCTION__);
        OutputDebugStringA(
            " but did not override the method in its CGlobalModule implementation."
            "  Please check the method signature to make sure it matches the corresponding method.\n");
        DebugBreak();

        return GL_NOTIFICATION_CONTINUE;
    }

    // GL_PRE_BEGIN_REQUEST 
    
    virtual 
    GLOBAL_NOTIFICATION_STATUS
    OnGlobalPreBeginRequest(
        _In_ IPreBeginRequestProvider * pProvider
    )
    {
        UNREFERENCED_PARAMETER( pProvider );
        OutputDebugStringA(
            "This module subscribed to event ");
        OutputDebugStringA(__FUNCTION__);
        OutputDebugStringA(
            " but did not override the method in its CGlobalModule implementation."
            "  Please check the method signature to make sure it matches the corresponding method.\n");
        DebugBreak();

        return GL_NOTIFICATION_CONTINUE;
    }

    // GL_APPLICATION_START 
    
    virtual 
    GLOBAL_NOTIFICATION_STATUS
    OnGlobalApplicationStart(
        _In_ IHttpApplicationStartProvider * pProvider
    )
    {
        UNREFERENCED_PARAMETER( pProvider );
        OutputDebugStringA(
            "This module subscribed to event ");
        OutputDebugStringA(__FUNCTION__);
        OutputDebugStringA(
            " but did not override the method in its CGlobalModule implementation."
            "  Please check the method signature to make sure it matches the corresponding method.\n");
        DebugBreak();

        return GL_NOTIFICATION_CONTINUE;
    }

    // GL_APPLICATION_RESOLVE_MODULES
    
    virtual 
    GLOBAL_NOTIFICATION_STATUS
    OnGlobalApplicationResolveModules(
        _In_ IHttpApplicationResolveModulesProvider * pProvider
    )
    {
        UNREFERENCED_PARAMETER( pProvider );
        OutputDebugStringA(
            "This module subscribed to event ");
        OutputDebugStringA(__FUNCTION__);
        OutputDebugStringA(
            " but did not override the method in its CGlobalModule implementation."
            "  Please check the method signature to make sure it matches the corresponding method.\n");
        DebugBreak();

        return GL_NOTIFICATION_CONTINUE;
    }

    // GL_APPLICATION_STOP

    virtual 
    GLOBAL_NOTIFICATION_STATUS
    OnGlobalApplicationStop(
        _In_ IHttpApplicationStopProvider * pProvider
    )
    {
        UNREFERENCED_PARAMETER( pProvider );
        OutputDebugStringA(
            "This module subscribed to event ");
        OutputDebugStringA(__FUNCTION__);
        OutputDebugStringA(
            " but did not override the method in its CGlobalModule implementation."
            "  Please check the method signature to make sure it matches the corresponding method.\n");
        DebugBreak();

        return GL_NOTIFICATION_CONTINUE;
    }

    // GL_RSCA_QUERY
    
    virtual 
    GLOBAL_NOTIFICATION_STATUS
    OnGlobalRSCAQuery(
        _In_ IGlobalRSCAQueryProvider * pProvider
    )
    {
        UNREFERENCED_PARAMETER( pProvider );
        OutputDebugStringA(
            "This module subscribed to event ");
        OutputDebugStringA(__FUNCTION__);
        OutputDebugStringA(
            " but did not override the method in its CGlobalModule implementation."
            "  Please check the method signature to make sure it matches the corresponding method.\n");
        DebugBreak();

        return GL_NOTIFICATION_CONTINUE;
    }

    // GL_TRACE_EVENT
    
    virtual 
    GLOBAL_NOTIFICATION_STATUS
    OnGlobalTraceEvent(
        _In_ IGlobalTraceEventProvider * pProvider
    )
    {
        UNREFERENCED_PARAMETER( pProvider );
        OutputDebugStringA(
            "This module subscribed to event ");
        OutputDebugStringA(__FUNCTION__);
        OutputDebugStringA(
            " but did not override the method in its CGlobalModule implementation."
            "  Please check the method signature to make sure it matches the corresponding method.\n");
        DebugBreak();

        return GL_NOTIFICATION_CONTINUE;
    }

    // GL_CUSTOM_NOTIFICATION
    
    virtual 
    GLOBAL_NOTIFICATION_STATUS
    OnGlobalCustomNotification(
        _In_ ICustomNotificationProvider * pProvider
    )
    {
        UNREFERENCED_PARAMETER( pProvider );
        OutputDebugStringA(
            "This module subscribed to event ");
        OutputDebugStringA(__FUNCTION__);
        OutputDebugStringA(
            " but did not override the method in its CGlobalModule implementation."
            "  Please check the method signature to make sure it matches the corresponding method.\n");
        DebugBreak();

        return GL_NOTIFICATION_CONTINUE;
    }

    virtual
    VOID
    Terminate(
        VOID
    ) = 0;

    // GL_THREAD_CLEANUP
    
    virtual 
    GLOBAL_NOTIFICATION_STATUS
    OnGlobalThreadCleanup(
        _In_ IGlobalThreadCleanupProvider * pProvider
    )
    {
        UNREFERENCED_PARAMETER( pProvider );
        OutputDebugStringA(
            "This module subscribed to event ");
        OutputDebugStringA(__FUNCTION__);
        OutputDebugStringA(
            " but did not override the method in its CGlobalModule implementation."
            "  Please check the method signature to make sure it matches the corresponding method.\n");
        DebugBreak();

        return GL_NOTIFICATION_CONTINUE;
    }

    // GL_APPLICATION_PRELOAD
    
    virtual 
    GLOBAL_NOTIFICATION_STATUS
    OnGlobalApplicationPreload(
        _In_ IGlobalApplicationPreloadProvider * pProvider
    )
    {
        UNREFERENCED_PARAMETER( pProvider );
        OutputDebugStringA(
            "This module subscribed to event ");
        OutputDebugStringA(__FUNCTION__);
        OutputDebugStringA(
            " but did not override the method in its CGlobalModule implementation."
            "  Please check the method signature to make sure it matches the corresponding method.\n");
        DebugBreak();

        return GL_NOTIFICATION_CONTINUE;
    }

    // GL_SUSPEND_PROCESS 
    virtual 
    GLOBAL_NOTIFICATION_STATUS
    OnSuspendProcess(
        _Outptr_ IGlobalSuspendProcessCallback **       pCallback
    )
    {
        UNREFERENCED_PARAMETER( pCallback );
        OutputDebugStringA(
            "This module subscribed to event ");
        OutputDebugStringA(__FUNCTION__);
        OutputDebugStringA(
            " but did not override the method in its CGlobalModule implementation."
            "  Please check the method signature to make sure it matches the corresponding method.\n");
        DebugBreak();

        return GL_NOTIFICATION_CONTINUE;
    }
};

class __declspec(uuid("85c1679c-0b21-491c-afb5-c7b5c86464c4"))
IModuleAllocator
{
public:
    virtual
    
    _Ret_opt_ _Post_writable_byte_size_(cbAllocation)
    VOID *
    AllocateMemory(
        _In_ DWORD                  cbAllocation
    ) = 0;
};

class __declspec(uuid("ba32d330-9ea8-4b9e-89f1-8c76a323277f"))
IHttpModuleFactory
{
public:
    virtual
    HRESULT
    GetHttpModule(
        _Outptr_ CHttpModule **  ppModule,
        _In_ IModuleAllocator *     pAllocator
    ) = 0;

    virtual
    VOID
    Terminate(
        VOID
    ) = 0;
};

//
// Register-module descriptor
//
class __declspec(uuid("07e5beb3-b798-459d-a98a-e6c485b2b3bc"))
IHttpModuleRegistrationInfo
{
public:
    virtual 
    PCWSTR
    GetName(
        VOID
    ) const = 0;

    virtual 
    HTTP_MODULE_ID
    GetId(
        VOID
    ) const = 0;

    virtual 
    HRESULT
    SetRequestNotifications(
        _In_ IHttpModuleFactory * pModuleFactory,
        _In_ DWORD                dwRequestNotifications,
        _In_ DWORD                dwPostRequestNotifications
    ) = 0;

    virtual 
    HRESULT
    SetGlobalNotifications(
        _In_ CGlobalModule *      pGlobalModule,
        _In_ DWORD                dwGlobalNotifications
    ) = 0;

    virtual
    HRESULT
    SetPriorityForRequestNotification(
        _In_ DWORD                dwRequestNotification,
        _In_ PCWSTR               pszPriority
    ) = 0;

    virtual
    HRESULT
    SetPriorityForGlobalNotification(
        _In_ DWORD                dwGlobalNotification,
        _In_ PCWSTR               pszPriority
    ) = 0;
};

class __declspec(uuid("4fd11cbf-8cc5-418e-8000-c9e765f88533"))
IHttpModuleRegistrationInfo2 : public IHttpModuleRegistrationInfo
{
public:
    virtual
    HRESULT
    SetPriorityForPostRequestNotification(
        _In_ DWORD                dwPostRequestNotification,
        _In_ PCWSTR               pszPriority
    ) = 0;
};
//
// Register Module entry point
// 

typedef
HRESULT
(WINAPI * PFN_REGISTERMODULE)(
    DWORD                           dwServerVersion,
    IHttpModuleRegistrationInfo *   pModuleInfo,
    IHttpServer *                   pGlobalInfo
);

#define MODULE_REGISTERMODULE   "RegisterModule"


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif
