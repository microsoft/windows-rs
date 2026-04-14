/*++

Copyright (c) 1998-2005 Microsoft Corporation

Module Name:

    listeneradapter.h

Abstract:

    The exported routines for the wbhstipm dll.

--*/

#ifndef _LISTENER_ADAPTER_H_
#define _LISTENER_ADAPTER_H_

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#ifdef __cplusplus
extern "C" {
#endif

//
//
//
// Enumerations
//

//
// callback function definitions
//

//
// *** Config Manager Callback functions ***
//

//
// Notifies the listener adapter that the configuration
// manager has connected.  After this call the listener
// adapter can expect configuration notifications to fire so 
// it can setup the listener to wait on incoming requests.   
// This notification will always proceed any configuration 
// notifications, and is a signal that they listener adapter
// should be in a clean state and ready to receive new
// configuration.
//
// WAS does not wait on the completion of this notification.
// 
typedef VOID
(*PFN_WEBHOST_LISTENER_CONFIG_MANAGER_CONNECTED)(
    IN VOID * pContext
    );

//
// Notifies the listener adapter that the WAS service
// has disconnected.  This may happen if WAS were to 
// crash.  In this case, the listener adapter is expected
// to cleanup everything that WAS configured.  Once WAS
// restarts it will provide any running listener adapters
// with configuration info again, so the listener adapter
// has to prepare to be re-configured.
//
// WAS does not wait on the completion of this notification.
// 
// 
typedef VOID
(*PFN_WEBHOST_LISTENER_CONFIG_MANAGER_DISCONNECT)(
    IN VOID * pContext,
    IN HRESULT hr
    );


//
// Notifies a listener adapter that WAS has completed passing
// all configuration for the service.  No more configuration
// will be passed until change notifications cause it.  At this
// point a listener adapter should activate all configured
// applications, not before.  Before this notification the listener
// adapter may have partial configuration data and may mis-route
// due to not realizing that another application that changes
// the routing exists.  It is also at this point that a service
// waiting on all applications and app pools to be configured
// can declare itself started.
//
// WAS does not wait on the completion of this notification.
// 
//
typedef VOID
(*PFN_WEBHOST_LISTENER_CONFIG_MANAGER_INITIALIZATION_COMPLETED)(
    IN VOID * pContext
    );


//
// *** App Pool Callback functions ***
//

//
// The application in the eyes of this listener adapter
// has been created.  In other words, an application that
// wants to participate in this app pool has been configured
// to use this protocol.  
//
// WAS does not wait on the completion of this notification.
//

typedef VOID
(*PFN_WEBHOST_LISTENER_APPLICATION_POOL_CREATED)(
    IN VOID * pContext,
    IN LPCWSTR AppPoolId,
    IN PSID    PSID
    );

//
// A notification the the app pool in the eyes of this
// listener is being deleted.  Once this call completes,
// the listener adapter should be capable of creating
// this app pool again should it be told to.  
//
// WAS does not wait on the completion of this notification.
//
typedef VOID
(*PFN_WEBHOST_LISTENER_APPLICATION_POOL_DELETED)(
    IN VOID * pContext,
    IN LPCWSTR AppPoolId
    );

//
// A notification that worker processes running 
// under this app pool will be running with a new
// identity.  
//
// WAS does wait on the completion of this notification.
//
// Once all all of the listener adapters have completed 
// this notification any recycles neccessary will be 
// performed, and worker processes will again be able to launch.  
// A listener adapter blocking on this routine can block 
// all others in the app pool from starting worker processes.
//
typedef VOID
(*PFN_WEBHOST_LISTENER_APPLICATION_POOL_IDENTITY_CHANGED)(
    IN VOID * pContext,
    IN LPCWSTR AppPoolId,
    IN PSID    PSID
    );

//
// Notify's a listener adapter that the state of an
// app pool has changed.  Either the app pool is now capable
// of launching and running worker processes, or the app 
// pool is now disabled and will not run worker processes.
// 
// WAS does not wait on the completion of this notification.
//
typedef VOID
(*PFN_WEBHOST_LISTENER_APPLICATION_POOL_STATE_CHANGED)(
    IN VOID * pContext,
    IN LPCWSTR AppPoolId,
    IN BOOL    fIsEnabled
    );

//
// Notify's a listener adapter that it is possible to 
// launch other instances of this listener channel.  The listener
// adapter should call start listener channel when it receives the 
// next request to be processed.  The listener adapter
// should not call start listener channel after that until it is
// once again told to through this function.
//
// WAS does not wait on the completion of this notification.
//
typedef VOID
(*PFN_WEBHOST_LISTENER_APPLICATION_POOL_CAN_OPEN_NEW_LISTENER_CHANNEL_INSTANCE)(
    IN VOID * pContext,
    IN LPCWSTR AppPoolId,
    IN DWORD ListenerChannelId
    );

//
// Notify's a listener adapter that all instances of a particular
// listener channel have been stopped.  During shutdown a listener adapter
// should wait on this before stopping, otherwise restarting
// may have problems, with listener channels all ready being in use.
//
// WAS does not wait on the completion of this notification.
//
typedef VOID
(*PFN_WEBHOST_LISTENER_APPLICATION_POOL_ALL_LISTENER_CHANNEL_INSTANCES_STOPPED)(
    IN VOID * pContext,
    IN LPCWSTR AppPoolId,
    IN DWORD ListenerChannelId
    );

//
// *** App Pool Callback functions ***
//

//
// Notifies the listener adapter that an application exists in
// the eyes of this listener adapter.  Whether the listener 
// adapter chooses to configure this application with the listener
// is up to the listener adapter.  The RequestsBlocked value tells the
// listener adapter that while you should listen on this application
// you should not process requests.  It is to block requests from going
// to parent applications because the child does not support the
// protocol.
//
// WAS does not wait on the completion of this notification.
//
typedef VOID
(*PFN_WEBHOST_LISTENER_APPLICATION_CREATED)(
    IN VOID * pContext,
    IN LPCWSTR AppKey,
    IN LPCWSTR Path,
    IN DWORD   SiteId,
    IN LPCWSTR AppPoolId,
    IN PBYTE   Bindings,
    IN DWORD   NumberOfBindings,
    IN BOOL    RequestsBlocked
    );

//
// Notifies the listener adapter that the application no longer
// should exist in the listener adapters eyes.  This may mean
// that the application was deleted, or that all applications for
// this site no longer use this protocol.
//
// WAS does not wait on the completion of this notification.
//
typedef VOID
(*PFN_WEBHOST_LISTENER_APPLICATION_DELETED)(
    IN VOID * pContext,
    IN LPCWSTR AppKey
    );

//
// Notifies the listener adapter that the bindings for a particular
// application have changed.
//
// WAS does not wait on the completion of this notification.
//
typedef VOID
(*PFN_WEBHOST_LISTENER_APPLICATION_BINDINGS_CHANGED)(
    IN VOID * pContext,
    IN LPCWSTR AppKey,
    IN PBYTE   Bindings,
    IN DWORD   NumberOfBindings
    );

//
// Notifies the listener adapter that the app pool for a particular
// application have changed.
//
// WAS does not wait on the completion of this notification.
//
typedef VOID
(*PFN_WEBHOST_LISTENER_APPLICATION_APP_POOL_CHANGED)(
    IN VOID * pContext,
    IN LPCWSTR AppKey,
    IN LPCWSTR AppPoolId
    );

//
// Notifies the listener adapter that it should either no longer
// reject requests for the application, or that it should start
// rejecting requests for the application.
//
// WAS does not wait on the completion of this notification.
//
typedef VOID
(*PFN_WEBHOST_LISTENER_APPLICATION_REQUESTS_BLOCKED_CHANGED)(
    IN VOID * pContext,
    IN LPCWSTR AppKey,
    IN BOOL    RequestsBlocked
    );

//
// Callback structure
//
struct WEBHOST_LISTENER_CALLBACKS
{
    DWORD                                                                         dwBytesInCallbackStructure;
    PFN_WEBHOST_LISTENER_CONFIG_MANAGER_CONNECTED                                 pfnWebhostListenerConfigManagerConnected;
    PFN_WEBHOST_LISTENER_CONFIG_MANAGER_DISCONNECT                                pfnWebhostListenerConfigManagerDisconnected;
    PFN_WEBHOST_LISTENER_CONFIG_MANAGER_INITIALIZATION_COMPLETED                  pfnWebhostListenerConfigManagerInitializationCompleted;

    PFN_WEBHOST_LISTENER_APPLICATION_POOL_CREATED                                 pfnWebhostListenerApplicationPoolCreated;
    PFN_WEBHOST_LISTENER_APPLICATION_POOL_DELETED                                 pfnWebhostListenerApplicationPoolDeleted;
    PFN_WEBHOST_LISTENER_APPLICATION_POOL_IDENTITY_CHANGED                        pfnWebhostListenerApplicationPoolIdentityChanged;
    PFN_WEBHOST_LISTENER_APPLICATION_POOL_STATE_CHANGED                           pfnWebhostListenerApplicationPoolStateChanged;
    PFN_WEBHOST_LISTENER_APPLICATION_POOL_CAN_OPEN_NEW_LISTENER_CHANNEL_INSTANCE  pfnWebhostListenerApplicationPoolCanOpenNewListenerChannelInstance;
    PFN_WEBHOST_LISTENER_APPLICATION_POOL_ALL_LISTENER_CHANNEL_INSTANCES_STOPPED  pfnWebhostListenerApplicationPoolAllListenerChannelInstancesStopped;

    PFN_WEBHOST_LISTENER_APPLICATION_CREATED                                      pfnWebhostListenerApplicationCreated;
    PFN_WEBHOST_LISTENER_APPLICATION_DELETED                                      pfnWebhostListenerApplicationDeleted;
    PFN_WEBHOST_LISTENER_APPLICATION_BINDINGS_CHANGED                             pfnWebhostListenerApplicationBindingsChanged;
    PFN_WEBHOST_LISTENER_APPLICATION_APP_POOL_CHANGED                             pfnWebhostListenerApplicationAppPoolChanged;
    PFN_WEBHOST_LISTENER_APPLICATION_REQUESTS_BLOCKED_CHANGED                     pfnWebhostListenerApplicationRequestsBlockedChanged;
};

//
// exported functions
//

//
// Webhost client can call to determine
// the version of webhost it is running 
// against.  This information will allow 
// it to understand what exported functions
// it should expect.
//
HRESULT
WebhostGetVersion(
    OUT DWORD* pMajorVersion,
    OUT DWORD* pMinorVersion
    );

//
// Registers a protocol for WAS to honor
// when it is running.  The ProtocolHandle returned
// can be used to activate other calls against
// the registered protocol.
//
// pListenerCallbacks will be 
//
HRESULT
WebhostRegisterProtocol(
    IN LPCWSTR ProtocolId,
    IN VOID*   pListenerCallbacks,
    IN VOID *  pContext,
    OUT DWORD* pProtocolHandle
    );

//
// Before a protocol goes down, it should first 
// stop any listener channels that were running, and wait for
// those listener channels to stop (detemined through private means).
// Then it should unregister the protocol with the W3SVC
// client library.
//
HRESULT
WebhostUnregisterProtocol(
    IN DWORD   ProtocolHandle
    );

//
// This will request a listener channel be started in a 
// worker process for this app pool.  
//
// It is a failure to call if the client
// library is not in a connected state.
//
HRESULT
WebhostOpenListenerChannelInstance(
    IN DWORD   ProtocolHandle,
    IN LPCWSTR AppPoolId,
    IN DWORD   ListenerChannelId,
    IN PBYTE   ListenerChannelBlob,
    IN DWORD   ListenerChannelBlobByteCount
    );

//
// Function will stop all instances of
// this listener channel.  It is not a failure to call
// with no listener channels running.  It is a failure
// to call if not in a connected state.
//
HRESULT
WebhostCloseAllListenerChannelInstances(
    IN DWORD   ProtocolHandle,
    IN LPCWSTR AppPoolId,
    IN DWORD   ListenerChannelId
    );

//
// =========================================================================================
//


#ifdef __cplusplus
}
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif  // _LISTENER_ADAPTER_H_

