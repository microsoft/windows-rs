/*++

   Copyright    (c)    2004    Microsoft Corporation

   Module  Name :
     wp_framework.h

   Abstract:

    Definition of 
    - IWorkerProcessFramework and related data and interfaces
    - IProtocolManager and related interfaces


    Protocol manager is a module that gets loaded by worker process in order 
    to handle traffic for a protocol. The actual startup/stopping of protocol 
    activity happens by starting/stopping protocol ListenerChannels.
    
    Protocol manager is expected to handle
    - protocol ListenerChannel commands 
    - health/lifetime monitoring for the internal activity 
    - custom actions (aka RSCA queries/functions)
    - provide custom interfaces what other protocol managers may leverage 
      (it is meant to allow for protocol managers or their internal components
      to share objects with other protocol managers)


   Worker process framework is set of interfaces available in the worker 
   process that are presented to protocol managers.  

   Worker process framework allows protocol managers to
   - retrieve the Config interface to be used within the worker process. 
   - retrieve certain cooked values such as AppPool name that protocol managers
     may need
   - report unhealthy events
   - load another protocol manager to retrieve custom interface
   - some additional utility functions (such as one that returns site ID and 
     virtual path for given AppID)
   
--*/

#ifndef _WP_FRAMEWORK_H_
#define _WP_FRAMEWORK_H_

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#include <unknwn.h>

//
// Forward declarations
//

class  IProtocolManager;

//
// Simple refcounted base class 
//

class IWpfReferencedObject
{
public:
    virtual
    ULONG 
    AddRef(
        VOID
    ) = 0;

    virtual
    ULONG 
    Release
    (
        VOID
    ) = 0;
    
};

//
// IWorkerProcessFramework related interfaces
//

// 
// WorkerProcessFramework implements multiple interfaces
// that are used by IProtocolManager instances to retrieve
// configuration/settings and perform actions related to the worker process
// monitoring
//


enum WPF_INTERFACE_ID_ENUM
{
    // retrieve native config interface
    WPF_CONFIG_API_ID = 1,  
    
    // helps to translate application ID to site and virtual path
    WPF_APPLICATION_INFO_UTIL_ID,     
    
    // exposes some settings that have process wide impact such
    // as recycling counters, CLR version ...
    WPF_SETTINGS_ID,     

    // way to request a recycle, shutdown worker pocess
    WPF_ACTIONS_ID,      
    
    // way to ask worker process framework to
    // load a protocol manager and request custom interface
    // of that. This interface allows multiple protocol managers
    // to share objects between them
    //
    WPF_EXPOSE_PROTOCOL_MANAGER_CUSTOM_INTERFACE_ID,  

    // way to get at preload information for an application
    WPF_APPLICATION_PRELOAD_INFO_UTIL_ID,
};


class IWorkerProcessFramework: public IWpfReferencedObject
{
public:

    virtual
    HRESULT 
    GetWpfInterface(
        IN  WPF_INTERFACE_ID_ENUM  WpfInterfaceId,
        OUT PVOID*                 ppInterface 
    ) = 0;

    virtual
    HRESULT 
    GetCustomInterface(
        IN  DWORD                           InterfaceId,
        OUT PVOID*                          ppInterface 
    ) = 0;

    //
    // return the IWorkerProcessFramework version
    // (it is meant to allow consumer to determine
    // what interfaces are available for retrieval)
    //
    virtual
    HRESULT 
    GetInterfaceVersion(
        OUT PDWORD      pdwMajorVersion,
        OUT PDWORD      pdwMinorVersion
    ) = 0;
};

//
// Interface ID: WPF_APPLICATION_INFO_ID
//
// Exposes helper function to retrieve SiteID and Virtual path for the 
// given AppId
//

class IWpfApplicationInfoUtil: public IWpfReferencedObject
{
public:
    virtual 
    HRESULT 
    GetApplicationPropertiesFromAppId(
        IN                                 PCWSTR      pszAppId, 
        _Out_writes_opt_(*pcchVirtualPath) PWSTR       pszVirtualPath,
        OUT                                DWORD *     pcchVirtualPath,
        OUT                                DWORD *     pdwSiteId ) = 0;
};


//
// enums used by IWpfSettings (WFP_SETTINGS_ID) interface
// 
    
enum WPF_SETTINGS_STRING_ENUM
{
    CLR_VERSION,
    APP_POOL_NAME,
    APP_HOST_FILE_NAME,
    ROOT_WEB_CONFIG_FILE_NAME,
    CLR_CONFIG_FILE_NAME,
    CLR_VERSION_PATH
};

enum WPF_SETTINGS_DWORD_ENUM
{
    PERIODIC_RESTART_REQUESTS = 1,
    PERIODIC_RESTART_TIME,
    IDLE_TIMEOUT,
    MANAGED_PIPELINE_MODE,
    THREAD_AFFINITY_POLICY,
    IDLE_TIMEOUT_ACTION
};

enum WPF_SETTINGS_ULONGLONG_ENUM
{
    PERIODIC_RESTART_VIRTUAL_MEMORY,
    PERIODIC_RESTART_PRIVATE_MEMORY,
};

enum WPF_SETTINGS_BOOL_ENUM
{
    ENABLED_CENTRAL_BINARY_LOGGING,
    ENABLED_HOSTABLE_WEB_CORE
};

enum WPF_SETTINGS_HANDLE_ENUM
{
    ANONYMOUS_USER_TOKEN
};

//
// Interface ID: WPF_SETTINGS_ID
//
// Exposes Application Pool / worker process related settings
// (based on applicationhost.config settings)
//

class IWpfSettings : public IWpfReferencedObject
{
public:
    virtual
    HRESULT
    GetDwordProperty(
        IN  WPF_SETTINGS_DWORD_ENUM  SettingId,
        OUT DWORD*                   pdwSetting
    ) = 0;

    virtual
    HRESULT
    GetUlonglongProperty(
        IN  WPF_SETTINGS_ULONGLONG_ENUM  SettingId,
        OUT ULONGLONG*                   pSetting
    ) = 0;

    virtual
    HRESULT
    GetStringProperty(
        IN                             WPF_SETTINGS_STRING_ENUM  SettingId,
        _Out_writes_opt_(*pcchSetting) PWSTR                     pszSetting,
        OUT                            DWORD*                    pcchSetting
    ) = 0;
    
    virtual
    HRESULT
    GetBoolProperty(
        IN  WPF_SETTINGS_BOOL_ENUM  SettingId,
        OUT BOOL *                  pfSetting
    ) = 0;

    virtual
    HRESULT
    GetHandleProperty(
        IN  WPF_SETTINGS_HANDLE_ENUM  SettingId,
        OUT HANDLE *                  phSetting
    ) = 0;
};


//
// Interface ID: WPF_ACTIONS_ID
//
// Allows protocol manager to request recycling or
// allows to report health issues (FailWorkerProcess)
//


class IWpfActions : public IWpfReferencedObject
{
public:

    virtual
    VOID
    RecycleWorkerProcess(
        IN PCWSTR   pszReason
    ) = 0;
    
    virtual
    VOID
    FailWorkerProcess(
        IN PCWSTR   pszReason,
        IN HRESULT  hrFailureToReport,
        IN BOOL     fFailOnNextPing = FALSE        
    ) = 0;
};

//
// Interface ID: WPF_EXPOSE_PROTOCOL_MANAGER_CUSTOM_INTERFACE_ID
//
// Some protocol managers may have dependencies on other protocol managers
// (for reasons such as controling the lifetime of a shared singleton
// object). Use of this interface is expected to be very limited.
//

class IWpfExposeProtocolManagerCustomInterface: public IWpfReferencedObject
{
public:

    //
    // Load the given protocol manager and retrieve the requested interface
    // The meaning of the call is to enable one protocol manager to leverage 
    // some functionality implemented in other protocol manager.
    // Note that we don't allow full access to ProtocolManager interfaces 
    // because most of them are only meant for the interaction with 
    // the Worker Process Framework.
    //
    // Also note that when done using custom interface it is necessary 
    // to release the reference to custom interface not later then in 
    // Shutdown() call.
    //

    virtual
    HRESULT
    LoadProtocolManagerAndGetCustomInterface(
        IN PCWSTR                   pProtocolManagerDll,
        IN PCWSTR                   pProtocolManagerDllInitFunction,
        IN DWORD                    dwCustomInterfaceId,
        OUT PVOID*                  ppCustomInterface
    ) = 0;

};

//
// IProtocolManager related interfaces
// - exposed on the Protocol Manager for the Worker Process Framework to use    
//

class IWpfListenerChannelCallback;


//
// IProtocolManager interface and related enum
// listing supported interfaces
//

enum PM_INTERFACE_ID_ENUM
{
    // Protocol ListenerChannel management interface
    PM_LISTENER_CHANNEL_MANAGER_ID = 1,  
    
    // Health/ idle monitoring interface
    PM_HEALTH_AND_IDLE_MONITOR_ID,     
    
    // Custom actions interface (for RSCA query)
    PM_CUSTOM_ACTIONS_ID,  

    // Application preload support
    PM_APPLICATION_PRELOAD_ID,

    // Action on idle timeout
    PM_IDLE_TIMEOUT_ID
};

//
// Finally the actual declaration of the IProtocolManager
//

class IProtocolManager   : public IWpfReferencedObject
{
public:
    virtual
    HRESULT 
    GetPmInterface(
        IN  PM_INTERFACE_ID_ENUM  PmInterfaceId,
        OUT PVOID*                ppInterface 
    ) = 0;

    
    virtual
    HRESULT 
    GetCustomInterface(
        IN  DWORD                           InterfaceId,
        OUT PVOID*                          ppInterface 
    ) = 0;
    
    virtual 
    HRESULT 
    Shutdown(
        IN BOOL             fImmediate
    ) = 0;

};


//
// Callback class used by IPmMonitor (PM_MONITOR_ID) interface
//

class IHealthStatusCallback: public IWpfReferencedObject
{
public:
   virtual 
    HRESULT 
    ReportHealth( 
        BOOL fHealthy
    ) = 0;   
};

//
// Interface ID: PM_HEALTH_AND_IDLE_MONITOR_ID
// IProtocolManager needs to expose health/ recycling monitoring means
//

class IPmHealthAndIdleMonitor : public IWpfReferencedObject
{
public:

    virtual 
    HRESULT 
    CheckIdle(
        OUT BOOL *pfIdle
    ) = 0;

    virtual 
    HRESULT 
    CheckHealth(   // health ping response
        IHealthStatusCallback * pIHealthStatusCallback
    ) = 0;    
};

//
// Callback class used by IPmCustomActions (PM_CUSTOM_ACTIONS_PM) interface
// 

class ICustomActionResultCallback
        : public IWpfReferencedObject
{
public:
    virtual
    HRESULT
    ReportResult( 
        IN HRESULT              hrStatus,
        IN PBYTE                pbResponse,
        IN DWORD                cbResponse
    ) = 0;
};

//
// Callback class used by IPmCustomActions2 (PM_CUSTOM_ACTIONS_2_PM) interface
// 

class ICustomAction2ResultCallback
        : public IWpfReferencedObject
{
public:
    virtual
    HRESULT
    ReportResult( 
        IN HRESULT              hrStatus,
        IN BOOL                 fFinalResponse,
        IN PBYTE                pbResponse,
        IN DWORD                cbResponse
    ) = 0;
};


//
// Interface ID: WPF_APPLICATION_PRELOAD_INFO_UTIL_ID
//
// Exposes helper function to retrieve SiteID and Virtual path for the 
// given AppId
//
class IWpfApplicationPreloadUtil : public IWpfReferencedObject
{
public:
    virtual 
    HRESULT
    GetApplicationPreloadInfo(
        IN PCWSTR pszConfigPath,
        OUT BOOL * pfEnabled,
        OUT BSTR * pbstrType,
        OUT SAFEARRAY ** psaPreloadValues
    ) = 0;

    virtual
    HRESULT
    ReportApplicationPreloadFailure(
        IN PCWSTR pszConfigPath,
        IN HRESULT hrFailureCode,
        IN PCWSTR pszErrorString
    ) = 0;
};


class IPmApplicationPreload : public IWpfReferencedObject
{
public:

    virtual
    HRESULT
    PreloadApplication(
        IN DWORD dwSiteId,
        IN PCWSTR pszAppConfigPath,
        IN BOOL fRecycledWorkerProcess = FALSE
    ) = 0;
};

//
// Interface ID: PM_CUSTOM_ACTIONS_ID
// IProtocolManager needs to expose a way for Custom actions
// (so that custom queries such as RSCA query can be implemented)
//

class IPmCustomActions : public  IWpfReferencedObject
{
public:

    //
    // RunCustomAction can report results back
    // by using a callback interface
    //

    virtual
    HRESULT
    RunCustomAction(
        IN PCWSTR   pszFunctionName,
        IN PCWSTR   pszFunctionArgs,
        IN ICustomActionResultCallback * pCompletionCallbackClass
    ) = 0 ;

};

//
// Interface ID: PM_CUSTOM_ACTIONS_2_ID
// IProtocolManager needs to expose a way for asynchronous custom actions
// (so that custom queries such as RSCA query can be implemented)
//

class IPmCustomActions2 : public  IWpfReferencedObject
{
public:

    //
    // RegisterCustomActionCallback is used for actions that will result
    // in multiple responses.
    //
    virtual
    HRESULT
    RegisterCustomActionCallback(
        IN PCWSTR   pszFunctionName,
        IN PCWSTR   pszFunctionArgs,
        IN ICustomAction2ResultCallback * pCompletionCallbackClass
    ) = 0 ;

    virtual
    HRESULT
    UnregisterCustomActionCallback(
        IN PCWSTR pszFunctionName
    ) = 0;
};


//
// Used by IPmListenerChannelManager (PM_LISTENER_CHANNEL_MANAGER_ID) 
// Interface ListenerChannelCallback provides notifications about 
// ListenerChannel status changes from Protocol managers.
// It also allows protocol managers to access extra parameters/blob
// that is communicated to worker process when ListenerChannels
// are asked to be started
//

class IWpfListenerChannelCallback : public IWpfReferencedObject
{
public:
    virtual 
    HRESULT 
    ReportStarted(
        VOID
    ) = 0;

    virtual 
    HRESULT 
    ReportStopped(
        HRESULT hr
    ) = 0;

    
    virtual 
    HRESULT 
    ReportMessageReceived(
        VOID
    ) = 0;

    virtual 
    HRESULT 
    GetId(
        OUT DWORD* pdwListenerChannelId
    ) = 0;

    virtual 
    HRESULT 
    GetBlob(
        IN OUT PBYTE pBlob, 
        IN OUT DWORD* pcbBlob 
    ) = 0;
};

//
// Interface ID: PM_LISTENER_CHANNEL_MANAGER_ID
// IProtocolManager needs to expose interface that allows
// controlling ListenerChannels (starting and stopping them)
//

class IPmListenerChannelManager: public IWpfReferencedObject
{
public:

    // Function will ask that a ListenerChannel be launched for
    // a protocol.
    virtual
    HRESULT
    StartListenerChannel(
        IN PCWSTR                          protocolId,
        IN IWpfListenerChannelCallback *   pListenerChannelCallback 
    ) = 0;

    virtual
    HRESULT 
    StopListenerChannel(
        IN PCWSTR                        protocolId,
        IN IWpfListenerChannelCallback * pListenerChannelCallback,
        IN BOOL                          fImmediate
    ) = 0;

    
};

//
// Interface ID: PM_IDLE_TIMEOUT_ID
// IProtocolManager needs to expose interface that get notification
// when idle timeout reached
//

class IPmIdleTimeOutAction : public IWpfReferencedObject
{
public:

    // Function will tell that worker process memory 
    // has been paged out.
    virtual
    HRESULT
    SuspendProcess(
        VOID 
    ) = 0;
};

//
// Default entrypoint of the protocol manager dll that allows worker 
// process to instantiate the ProtocolManager instance
//

#define DEFAULT_PROTOCOL_MANAGER_INIT_FUNCTION      L"GetProtocolManager"

typedef HRESULT (*PFN_GET_PROTOCOL_MANAGER)
                    ( IN  IWorkerProcessFramework * pWpFramework,
                      OUT IProtocolManager **       ppProtocolManager );


//
// Entry point for the Managed Runtime Loader method, which can be used
// to customize the loading of the Managed Runtime.
//

#define MANAGED_RUNTIME_LOADER_FUNCTION             "LoadManagedRuntime"
#define MANAGED_RUNTIME_LOADER_EX_FUNCTION          "LoadManagedRuntimeEx"

HRESULT __stdcall LoadManagedRuntime(
    PCWSTR pwszRuntimeVersion,
    IUnknown ** ppManagedRuntimeHost );

// Corresponding function typedef
typedef HRESULT (__stdcall *PFNLoadManagedRuntime)(
    PCWSTR pwszRuntimeVersion,
    IUnknown ** ppManagedRuntimeHost );

typedef HRESULT (__stdcall *PFNLoadManagedRuntimeEx)(
    PCWSTR pwszRuntimeVersion,
    PCWSTR pszClrConfigFile,
    IUnknown ** ppManagedRuntimeHost );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif
