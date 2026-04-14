//+-------------------------------------------------------------------------
//
//  iepmapi.h -- This module defines the IE Protected Mode APIs
// 
//  Copyright (c) Microsoft Corp. All rights reserved.
//
//--------------------------------------------------------------------------
#ifndef _IEPMAPI_
#define _IEPMAPI_


#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


// Remap old API names to new ones.
#define IEGetWriteableHKCU      IEGetWriteableLowHKCU


// IE Launch Option Flags

typedef enum {
    IELAUNCHOPTION_SCRIPTDEBUG  = 0x00000001,
    IELAUNCHOPTION_FORCE_COMPAT = 0x00000002,
    IELAUNCHOPTION_FORCE_EDGE   = 0x00000004,
    IELAUNCHOPTION_LOCK_ENGINE  = 0x00000008,
} IELAUNCHOPTION_FLAGS;

//+-------------------------------------------------------------------------
//
// Structure:   IELAUNCHURLINFO
//
// Members:
//              cbSize - Size of the structure, in bytes.
//              dwCreationFlags - Process Creation flags used by CreateProcess 
//                                and CreateProcessAsUser functions.
//              dwLaunchOptions - Combination of IELAUNCHOPTION_FLAGS
//--------------------------------------------------------------------------
typedef struct _IELAUNCHURLINFO {
    DWORD cbSize;
    DWORD dwCreationFlags;
    DWORD dwLaunchOptionFlags;
} IELAUNCHURLINFO, *LPIELAUNCHURLINFO;

//+-------------------------------------------------------------------------
//
//  Method:     IESaveFile
// 
//  Synopsis:   Saves the file to the location selected by the user in a 
//              previous call to IEShowSaveFileDialog
//
//--------------------------------------------------------------------------
STDAPI IESaveFile(                                   
    _In_ HANDLE    hState,                           
    _In_ LPCWSTR   lpwstrSourceFile                  
    );                          


//+-------------------------------------------------------------------------
//
//  Method:     IECancelSaveFile
// 
//  Synopsis:   Cancels the save operation and releases the resources 
//              allocated for the previous call to IEShowSaveFileDialog
//
//--------------------------------------------------------------------------
STDAPI IECancelSaveFile(
    _In_ HANDLE    hState
    );


//+-------------------------------------------------------------------------
//
//  Method:     IEShowSaveFileDialog
// 
//  Synopsis:   Shows the standard SaveFile dialog from a higher integrity 
//              user context
//
//  Remarks:    When no longer needed, call CoTaskMemFree to release
//              lppwstrDestinationFilePath 
//
//--------------------------------------------------------------------------
STDAPI IEShowSaveFileDialog(                         
    _In_        HWND     hwnd,                       
    _In_        LPCWSTR  lpwstrInitialFileName,      
    _In_opt_    LPCWSTR  lpwstrInitialDir,           
    _In_opt_    LPCWSTR  lpwstrFilter,               
    _In_opt_    LPCWSTR  lpwstrDefExt,               
    _In_        DWORD    dwFilterIndex,              
    _In_        DWORD    dwFlags,                    
    _Outptr_ LPWSTR   *lppwstrDestinationFilePath,
    _Out_       HANDLE   *phState                    
    );                                               



//+-------------------------------------------------------------------------
//
//  Method:     IEShowOpenFileDialog
// 
//  Synopsis:   Shows the standard OpenFile dialog from a higher integrity 
//              user context
//
//--------------------------------------------------------------------------
STDAPI IEShowOpenFileDialog(
    __in                            HWND     hwnd,
    __inout_ecount(cchMaxFileName)  LPWSTR   lpwstrFileName,
    __in                            DWORD    cchMaxFileName,
    __in_opt                        LPCWSTR  lpwstrInitialDir,
    __in_opt                        LPCWSTR  lpwstrFilter,
    __in_opt                        LPCWSTR  lpwstrDefExt,
    __in                            DWORD    dwFilterIndex,
    __in                            DWORD    dwFlags,
    __out                           HANDLE   *phFile
    );


//+-------------------------------------------------------------------------
//
//  Method:     IEGetWriteableLowHKCU
// 
//  Synopsis:   Returns a handle to a write accessible location under 
//              HKEY_CURRENT_USER for MIC Low process
//
//  Remarks:    When no longer needed, call RegCloseKey function to close 
//              the HKEY
//
//--------------------------------------------------------------------------
STDAPI IEGetWriteableLowHKCU(                           
    _Out_ HKEY    *pHKey                             
    );                                               


//+-------------------------------------------------------------------------
//
//  Method:     IEGetWriteableFolderPath
// 
//  Synopsis:   Returns the current location of the specified folder. 
//              In protected mode, the path points to a location where 
//              the user has write permissions
//
//  Remarks:    When no longer needed, call CoTaskMemFree to release the
//              lppwstrPath
//
//--------------------------------------------------------------------------
STDAPI IEGetWriteableFolderPath(                     
    _In_        REFGUID clsidFolderID,               
    _Outptr_ LPWSTR   *lppwstrPath                
    );                                               


//+-------------------------------------------------------------------------
//
//  Method:     IEIsProtectedModeProcess
// 
//  Synopsis:   Determines if Internet Explorer is running in protected mode
//
//--------------------------------------------------------------------------
STDAPI IEIsProtectedModeProcess(                     
    _Out_ BOOL     *pbResult                         
    );                                               


//+-------------------------------------------------------------------------
//
//  Method:     IEIsProtectedModeURL
// 
//  Synopsis:   Determines if the URL runs in Protected Mode or not
//
//--------------------------------------------------------------------------
STDAPI IEIsProtectedModeURL(                     
    _In_ LPCWSTR lpwstrUrl
    );

//+-------------------------------------------------------------------------
//
//  Method:     IELaunchURL
// 
//  Synopsis:   Launches the appropriate IE to handle the navigation to 
//              the URL
//
//--------------------------------------------------------------------------
STDAPI IELaunchURL(                     
    _In_        LPCWSTR lpwstrUrl,
    _Inout_     PROCESS_INFORMATION *lpProcInfo,
    _In_opt_    VOID *lpInfo
    );

//+-------------------------------------------------------------------------
//
//  Method:     IERefreshElevationPolicy
// 
//  Synopsis:   Causes the next elevation policy look up to refresh from
//              the registry
//
//--------------------------------------------------------------------------
STDAPI IERefreshElevationPolicy();

//+-------------------------------------------------------------------------
//
//  Method:     IEGetProtectedModeCookie
// 
//  Synopsis:   Retrieves the cookie data from the Protected Mode Cookie store
//
//--------------------------------------------------------------------------
STDAPI IEGetProtectedModeCookie(_In_ LPCWSTR lpszURL,
                                _In_ LPCWSTR lpszCookieName,
                                _Inout_updates_(*pcchCookieData) LPWSTR lpszCookieData,
                                _Inout_ DWORD *pcchCookieData,
                                _In_ DWORD dwFlags);

//+-------------------------------------------------------------------------
//
//  Method:     IESetProtectedModeCookie
// 
//  Synopsis:   Set the cookie data in the Protected Mode Cookie store
//
//--------------------------------------------------------------------------
STDAPI IESetProtectedModeCookie(_In_ LPCWSTR lpszURL,
                                _In_ LPCWSTR lpszCookieName,
                                _In_ LPCWSTR lpszCookieData,
                                _In_ DWORD dwFlags);

//+-------------------------------------------------------------------------
//
//  Method:     IERegisterWritableRegistryKey
// 
//  Synopsis:   Register a subkey path during installation so that 
//              low process can write into this location during run time.
//
//--------------------------------------------------------------------------
STDAPI IERegisterWritableRegistryKey(
    GUID guid,
    _In_ LPCWSTR lpSubkey,
    BOOL fSubkeyAllowed
);

//+-------------------------------------------------------------------------
//
//  Method:     IERegisterWritableRegistryValue
// 
//  Synopsis:   Register a value in a subkey path during installation so that 
//              low process can write into this location during run time.
//
//--------------------------------------------------------------------------
STDAPI IERegisterWritableRegistryValue(
    GUID guid,
    _In_ LPCWSTR lpPath, 
    _In_ LPCWSTR lpValueName, 
    DWORD dwType, 
    _In_reads_bytes_opt_(cbMaxData) const BYTE* lpData, 
    DWORD cbMaxData
);

//+-------------------------------------------------------------------------
//
//  Method:     IEUnregisterWritableRegistryKey
// 
//  Synopsis:   Unregister a registry path during uninstallation so that 
//              low process can not write into this location
//
//--------------------------------------------------------------------------
STDAPI IEUnregisterWritableRegistry(
    GUID guid
);

//+-------------------------------------------------------------------------
//
//  Method:     IERegCreateKeyEx
// 
//  Synopsis:   Call Broker to do RegCreateKeyEx on pre-registered locations
//
//--------------------------------------------------------------------------
STDAPI IERegCreateKeyEx(
  _In_      LPCWSTR lpSubKey,
  _In_      DWORD Reserved,
  _In_opt_  LPWSTR lpClass,
  _In_      DWORD dwOptions,
  _In_      REGSAM samDesired,
  _In_opt_  LPSECURITY_ATTRIBUTES lpSecurityAttributes,
  _Out_     PHKEY phkResult,
  _Out_     LPDWORD lpdwDisposition);

//+-------------------------------------------------------------------------
//
//  Method:     IERegSetValueEx
// 
//  Synopsis:   Call Broker to do RegSetValueEx on pre-registered location
//
//--------------------------------------------------------------------------
STDAPI IERegSetValueEx(
  _In_ LPCWSTR lpSubKey,
  _In_ LPCWSTR lpValueName,
  _In_ DWORD Reserved,
  _In_ DWORD dwType,
  _In_reads_bytes_(cbData) const BYTE* lpData,
  _In_ DWORD cbData);

//+-------------------------------------------------------------------------
//
//  Method:     IECreateFile
// 
//  Synopsis:   wrapper to CreateFile
//
//--------------------------------------------------------------------------
HANDLE IECreateFile(
  _In_ LPCWSTR lpFileName,
  DWORD dwDesiredAccess,
  DWORD dwShareMode,
  _In_ LPSECURITY_ATTRIBUTES lpSecurityAttributes,
  DWORD dwCreationDisposition,
  DWORD dwFlagsAndAttributes,
  _In_opt_ HANDLE hTemplateFile
);

//+-------------------------------------------------------------------------
//
//  Method:     IEDeleteFile   
// 
//  Synopsis:   wrapper to DeleteFile
//
//--------------------------------------------------------------------------
BOOL IEDeleteFile(
  _In_ LPCWSTR lpFileName
);

//+-------------------------------------------------------------------------
//
//  Method:     IERemoveDirectory
// 
//  Synopsis:   wrapper to RemoveDirectory
//
//--------------------------------------------------------------------------
BOOL IERemoveDirectory(
  _In_ LPCWSTR lpPathName
);

//+-------------------------------------------------------------------------
//
//  Method:     IEMoveFileEx
// 
//  Synopsis:   wrapper to MoveFileEx
//
//--------------------------------------------------------------------------
BOOL IEMoveFileEx(
  _In_ LPCWSTR lpExistingFileName,
  _In_ LPCWSTR lpNewFileName,
  DWORD dwFlags
);

//+-------------------------------------------------------------------------
//
//  Method:     IECreateDirectory
// 
//  Synopsis:   wrapper to CreateDirectory
//
//--------------------------------------------------------------------------
BOOL IECreateDirectory(
  _In_ LPCWSTR lpPathName,
  _In_ LPSECURITY_ATTRIBUTES lpSecurityAttributes
);

//+-------------------------------------------------------------------------
//
//  Method:     IEGetFileAttributesEx
// 
//  Synopsis:   wrapper to GetFileAttributesEx
//
//--------------------------------------------------------------------------
BOOL IEGetFileAttributesEx(
  _In_ LPCWSTR lpFileName,
  GET_FILEEX_INFO_LEVELS fInfoLevelId,
  _In_ LPVOID lpFileInformation
);

//+-------------------------------------------------------------------------
//
//  Method:     IEFindFirstFile
// 
//  Synopsis:   wrapper to FindFirstFile
//
//--------------------------------------------------------------------------
HANDLE IEFindFirstFile(
  _In_ LPCWSTR lpFileName,
  _In_ LPWIN32_FIND_DATA lpFindFileData
);

// registration ID for enhanced protected mode compatibility
EXTERN_C GUID CATID_AppContainerCompatible;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif //_IEPMAPI_
