/*++

   Copyright    (c)    2004    Microsoft Corporation

   Module  Name :
     hostable_web_core.h

   Abstract:

     Hostable web core enables processes other than the W3WP.exe to load the IISCore 
     and effectively host pages without having to reverse-engineer IIS W3 core 
     functionality (ISAPI filter/extension support, authentication, authorization, 
     configuring http.sys, compression, etc.).  

     This header file defines the interface between the hostable web core and the hosting process

     Note: Only single instance of the hostable web core will be allowed within one process
     Attempt to initialize more than one will result in failure

   Environment:
       Win32 - User Mode

   Project:
      IIS7

--*/

#ifndef _HOSTABLE_WEB_CORE_H_
#define _HOSTABLE_WEB_CORE_H_

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

// 
// The actual names of the hostable webcore DLL and the start/stop entry points
//

#define WEB_CORE_DLL_NAME                L"hwebcore.dll"
#define WEB_CORE_ACTIVATE_DLL_ENTRY      "WebCoreActivate"
#define WEB_CORE_SHUTDOWN_DLL_ENTRY      "WebCoreShutdown"
#define WEB_CORE_SET_METADATA_DLL_ENTRY  "WebCoreSetMetadata" 


//
// Prototype for the WEB_CORE_SET_METADATA_DLL_ENTRY call used to
// set metadata options for a web core activation. 
//
// Return values:
//
// HRESULT_FROM_WIN32( ERROR_SERVICE_ALREADY_RUNNING ) - returned if
//   instance of the hostable web core is already running in the process
//
// HRESULT_FROM_WIN32( ERROR_NOT_SUPPORTED ) - returned if the metadata
//   being set is not supported.
//
// HRESULT_FROM_WIN32( ERROR_INVALID_DATA ) - most likely this error
//   means configuration error in the AppHostConfigFile
//
// any other error that occurs during activation will be reported
//

typedef HRESULT
(*PFN_WEB_CORE_SET_METADATA_DLL_ENTRY)
(
    IN PCWSTR                           pszMetadataType,
    IN PCWSTR                           pszValue
);


//
// Prototype for the WEB_CORE_ACTIVATE_DLL_ENTRY call used to
// start the hostable webcore. 
// Hosting process must provide path to the Application Host config file
// that contains the description of sites/applications/appools
// second parameter is meant mainly for identifying webcore instance
// when reporting NT event log events
//
// Return values:
//
// HRESULT_FROM_WIN32( ERROR_SERVICE_ALREADY_RUNNING ) - returned if
//   instance of the hostable web core is already running in the process
//
// HRESULT_FROM_WIN32( ERROR_INVALID_DATA ) - most likely this error
//   means configuration error in the AppHostConfigFile
//
// any other error that occurs during activation will be reported
//

typedef HRESULT
(*PFN_WEB_CORE_ACTIVATE)
(
    IN PCWSTR                           pszAppHostConfigFile,
    IN PCWSTR                           pszRootWebConfigFile,    
    IN PCWSTR                           pszInstanceName
);


//
// Prototype for the WEB_CORE_SHUTDOWN_DLL_ENTRY call used to
// stop the hostable webcore. 
// Hosting process may choose between immediate shutdown and graceful shutdown
// If graceful shutdown is chosen then hostable webcore will stop receiving
// new requests and wait for the currently executing requests to complete for certain time
// (as specified in the config)
//
// Return values:
//
// HRESULT_FROM_WIN32( ERROR_SERVICE_NOT_ACTIVE ) - returned if
//   hostable web core is not running and there is nothing to stop
//
// HRESULT_FROM_WIN32( ERROR_INVALID_SERVICE_CONTROL ) - returned if shutdown
//   is already in progress. Note that this error wouldn't be returned
//   if current shutdown in progress been triggered by the callback thread 
//   (the one that notifies about config change or bad health). In that case 
//   host asking for shutdown will be blocked until callback thread is completed
//
// HRESULT_FROM_WIN32( ERROR_SERVICE_REQUEST_TIMEOUT ) - returned if graceful
//   shutdown was requested but was not able to complete within a given period
//   This is rather a WARNING because webcore gets shutdown forcefully 
//   if graceful shutdown attempt times out. But caller may want to be informed
//   that some requests may have been forcefully cancelled
//
// any other error that occurs during termination will be reported
//

typedef HRESULT
(*PFN_WEB_CORE_SHUTDOWN)
(
    IN DWORD                     fImmediate
);


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif

