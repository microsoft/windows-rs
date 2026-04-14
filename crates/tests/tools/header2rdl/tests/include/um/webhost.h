
//
// Web Host APIs
//
// Copyright (c) 2002 Microsoft Corporation
//

#pragma once
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#pragma pack(push, 8)

#include <comdef.h>

//
// ListenerChannel Callback provides notifications related to listenerChannels 
// from Protocol Handlers to worker process framework.
// It also allows protocol handlers to access additional parameters such
// as Id of the listenerChannel of the Blob they may contain additional parameters
// passed from listener adapter
//

struct __declspec(uuid("dc3b0a85-9da7-47e4-ba1b-e27da9db8a1e")) IListenerChannelCallback;
struct IListenerChannelCallback : IUnknown {
    virtual HRESULT __stdcall ReportStarted() = 0;

    virtual HRESULT __stdcall ReportStopped(HRESULT hr) = 0;

    virtual HRESULT __stdcall ReportMessageReceived() = 0;

    virtual HRESULT __stdcall GetId(DWORD* pdwListenerChannelId) = 0;

    virtual HRESULT __stdcall GetBlobLength(DWORD* pcbLength) = 0;

    virtual HRESULT __stdcall GetBlob(PBYTE pBlob, DWORD* pcbBlob) = 0;
};

//
// AppDomainInfo contains details about an appdomain
//

struct __declspec(uuid("5BC9C234-6CD7-49bf-A07A-6FDB7F22DFFF")) IAppDomainInfo;
struct IAppDomainInfo : IUnknown {
    virtual HRESULT __stdcall GetId(
        BSTR *pbstrAppDomainId ) = 0;

    virtual HRESULT __stdcall GetVirtualPath(
        BSTR *pbstrVirtualPath ) = 0;

    virtual HRESULT __stdcall GetPhysicalPath(
        BSTR *pbstrPhysicalPath ) = 0;

    virtual HRESULT __stdcall GetSiteId(
        DWORD *pdwSiteId ) = 0;

    virtual HRESULT __stdcall GetIsIdle(
        DWORD *pfIsIdle ) = 0;
};

//
// Enumerator to get access to the information about
// all app domains managed by ProcessHost
//

struct __declspec(uuid("F79648FB-558B-4a09-88F1-1E3BCB30E34F")) IAppDomainInfoEnum;
struct IAppDomainInfoEnum : IUnknown {
    virtual HRESULT __stdcall GetData(
        IAppDomainInfo ** ppAppDomainInfo ) = 0;

    virtual HRESULT __stdcall Count(
        DWORD * pdwCount ) = 0;

    virtual HRESULT __stdcall MoveNext(
        DWORD * pfMoreItems ) = 0;

    virtual HRESULT __stdcall Reset(
         ) = 0;
};

//
// Helper functions for Process Host that are exposed (indirectly) by worker process
// framework through the protocol manager that interfaces worker process framework
// with the managed Process Host
//
struct __declspec(uuid("35f9c4c1-3800-4d17-99bc-018a62243687")) IProcessHostSupportFunctions;
struct IProcessHostSupportFunctions : IUnknown {
    virtual HRESULT __stdcall GetApplicationProperties(
        LPCWSTR appId, 
        BSTR   *pbstrVirtualPath,
        BSTR   *pbstrPhysicalPath,
        BSTR   *pbstrSiteName,
        BSTR   *pbstrSiteId) = 0;

    virtual HRESULT __stdcall MapPath(
        LPCWSTR appId, 
        LPCWSTR virtualPath, 
        BSTR   *pbstrPhysicalPath) = 0;

    virtual HRESULT __stdcall GetConfigToken(
        LPCWSTR appId, 
        HANDLE *pToken) = 0;

    virtual HRESULT __stdcall GetAppHostConfigFilename(
        BSTR   *pbstrPhysicalPath) = 0;
    
    virtual HRESULT __stdcall GetRootWebConfigFilename(
        BSTR   *pbstrPhysicalPath) = 0;

    virtual HRESULT __stdcall GetNativeConfigurationSystem(
        PPVOID ppConfigSystem) = 0;
};

//
// Ping callback interface
//
struct __declspec(uuid("f11dc4c9-ddd1-4566-ad53-cf6f3a28fefe")) 
                                                        IProcessPingCallback;
struct IProcessPingCallback : IUnknown {
    virtual HRESULT __stdcall Respond() = 0;
};

//
// Process Host
// 
struct __declspec(uuid("0ccd465e-3114-4ca3-ad50-cea561307e93")) IProcessHost;
struct IProcessHost : IUnknown {
    virtual HRESULT __stdcall StartApplication(
        PCWSTR appId,
        PCWSTR appPath,
        IUnknown ** ppRuntimeObject ) = 0;
    
    virtual HRESULT __stdcall ShutdownApplication(
        LPCWSTR appId) = 0;

    virtual HRESULT __stdcall Shutdown() = 0;

    virtual HRESULT __stdcall EnumerateAppDomains(
        IAppDomainInfoEnum ** pAppDomainsInfo ) = 0;
};

//
// Process Protocol Handlers Manager
//
struct __declspec(uuid("1cc9099d-0a8d-41cb-87d6-845e4f8c4e91")) IPphManager;
struct IPphManager : IUnknown {
    virtual HRESULT __stdcall StartProcessProtocolListenerChannel(
        LPCWSTR protocolId,
        IListenerChannelCallback *pListenerChannelCallback) = 0;

    virtual HRESULT __stdcall StopProcessProtocolListenerChannel(
        LPCWSTR protocolId,
        DWORD listenerChannelId,
        BOOL immediate) = 0;

    virtual HRESULT __stdcall StopProcessProtocol(
        LPCWSTR protocolId,
        BOOL immediate) = 0;
};

//
// Process Host Idle and Health Check
//
struct __declspec(uuid("9d98b251-453e-44f6-9cec-8b5aed970129")) 
                                            IProcessHostIdleAndHealthMonitor;
struct IProcessHostIdleAndHealthMonitor : IUnknown {
    //
    // Check if process host is idle
    //
    virtual HRESULT __stdcall IsIdle(
        BOOL *pfIdle) = 0;

    //
    // Perform health check
    //

    virtual HRESULT __stdcall Ping(
        IProcessPingCallback *pCallback) = 0;
};


//
// Helper to create and configure Process Host
//

struct __declspec(uuid("02fd465d-5c5d-4b7e-95b6-82faa031b74a")) 
                                                    IProcessHostFactoryHelper;
struct IProcessHostFactoryHelper : IUnknown {
    virtual HRESULT __stdcall GetProcessHost(
        IProcessHostSupportFunctions *pFunctions,
        IUnknown **ppProcessHost) = 0;
};

//
// Application preload util
//
struct __declspec(uuid("940D8ADD-9E40-4475-9A67-2CDCDF57995C")) 
                                                    IApplicationPreloadUtil;
struct IApplicationPreloadUtil : IUnknown {
    virtual
    HRESULT
    __stdcall
    GetApplicationPreloadInfo(
        PCWSTR pszContext,
        BOOL * pfEnabled,
        BSTR * pbstrType,
        SAFEARRAY ** psaPreloadValues
    ) = 0;
    
    virtual
    HRESULT
    __stdcall
    ReportApplicationPreloadFailure(
        PCWSTR pszContext,
        HRESULT hrFailureCode,
        PCWSTR pszErrorString
    ) = 0;
};

//
// Application preload manager
//
struct __declspec(uuid("AE54F424-71BC-4da5-AA2F-8C0CD53496FC")) 
                                                    IApplicationPreloadManager;
struct IApplicationPreloadManager : IUnknown {
    virtual HRESULT __stdcall SetApplicationPreloadUtil(
        IApplicationPreloadUtil * pPreload
    ) = 0;
    
    virtual HRESULT __stdcall SetApplicationPreloadState(
        PCWSTR pszContext,
        PCWSTR pszAppId,
        BOOL fEnabled
    ) = 0;
};

//
// Function pointer declarations
//
HRESULT __stdcall GetIsapiProcessHost(
    IProcessHostSupportFunctions *pFunctions,
    IProcessHost **ppProcessHost);

// Corresponding function typedef
typedef HRESULT (__stdcall *PFNGetIsapiProcessHost)(
    IProcessHostSupportFunctions *pFunctions,
    IProcessHost **ppProcessHost);

#pragma pack(pop)



#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

