//+---------------------------------------------------------------------------
//
//  Microsoft Windows NT5.0
//  Copyright (C) Microsoft Corporation, 1997-1999.
//
//  File:       O E M U P G E X . H
//
//  Contents:   Function prototypes needed for OEM network upgrade
//
//  Notes:  These functions are obsolete for Windows XP (SP1 and later),
//          Windows .NET Server, and later operating systems.
//
//----------------------------------------------------------------------------

#ifndef __OEMUPGEX_H__
#define __OEMUPGEX_H__

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#if (NTDDI_VERSION <= NTDDI_WINXP)

#ifdef __cplusplus
extern "C" {
#endif

#ifndef WINNT32P_H
typedef enum {
    UNKNOWN,
    NT_WORKSTATION,
    NT_SERVER
} PRODUCTTYPE;
#endif

typedef struct
{
    PRODUCTTYPE ProductType;
    DWORD       dwBuildNumber;
} ProductInfo;

typedef struct
{
    ProductInfo From;
    ProductInfo To;
} NetUpgradeInfo;

typedef struct
{
    WCHAR   szCompanyName[256];
    WCHAR   szSupportNumber[256];
    WCHAR   szSupportUrl[256];
    WCHAR   szInstructionsToUser[1024];
} VENDORINFO;

const DWORD c_dwMaxProductIdLen = 255;

const DWORD NUA_LOAD_POST_UPGRADE        = 0x0001;
const DWORD NUA_REQUEST_ABORT_UPGRADE    = 0x0002;
const DWORD NUA_ABORT_UPGRADE            = 0x0004;
const DWORD NUA_SKIP_INSTALL_IN_GUI_MODE = 0x80000L;

typedef struct
{
    LPTSTR mszServicesNotToBeDeleted;
} NetUpgradeData;

// Names of functions exported from OEM DLL
//

#define c_szPreUpgradeInitialize        "PreUpgradeInitialize"
#define c_szDoPreUpgradeProcessing      "DoPreUpgradeProcessing"
#define c_szPostUpgradeInitialize       "PostUpgradeInitialize"
#define c_szDoPostUpgradeProcessing     "DoPostUpgradeProcessing"

// Names of functions exported from netupgrd.dll
//
#define c_szNetUpgradeAddSection        "NetUpgradeAddSection"
#define c_szNetUpgradeAddLineToSection  "NetUpgradeAddLineToSection"

// Keys in OemSection
//
#define  c_szInfToRunBeforeInstall      TEXT("InfToRunBeforeInstall")
#define  c_szInfToRunAfterInstall       TEXT("InfToRunAfterInstall")

//
// Prototypes for functions exported from OEM DLL
//

//+---------------------------------------------------------------------------
//
//  Function:   PreUpgradeInitialize
//
//  Purpose:    Intialize OEM DLL
//
//  Arguments:
//      szWorkingDir     [in]   name of temporary directory to be used
//      pNetUpgradeInfo  [in]   pointer to NetUpgradeInfo structure
//      pviVendorInfo    [out]  information about OEM
//      pdwFlags         [out]  pointer to flags
//      pNetUpgradeData  [out]  pointer to NetUpgradeData structure
//
//  Returns:    ERROR_SUCCESS in case of success, win32 error otherwise
//
//  Notes:
//          This function is obsolete for Windows XP (SP1 and later),
//          Windows .NET Server, and later operating systems.
//
//      This function is called before any other function in this dll.
//      The main purpose of calling this function is to obtain
//      identification information and to allow the DLL to initialize
//      its internal data
//
__drv_preferredFunction("(see documentation)", "Obsolete") 
LONG __stdcall
PreUpgradeInitialize(_In_  LPCTSTR         szWorkingDir,
                     _In_  NetUpgradeInfo* pNetUpgradeInfo,
                     _Out_ VENDORINFO*     pviVendorInfo,
                     _Out_ DWORD*          pdwFlags,
                     _Out_ NetUpgradeData* pNetUpgradeData);

typedef LONG
(__stdcall *PreUpgradeInitializePrototype)
     (_In_  LPCTSTR         szWorkingDir,
      _In_ NetUpgradeInfo* pNetUpgradeInfo,
      _Out_ VENDORINFO*     pviVendorInfo,
      _Out_ DWORD*          pdwFlags,
      _Out_ NetUpgradeData* pNetUpgradeData);

//+---------------------------------------------------------------------------
//
//  Function:   DoPreUpgradeProcessing
//
//  Purpose:    Intialize OEM DLL
//
//  Arguments:
//      hParentWindow    [in]  window handle for showing UI
//      hkeyParams       [in]  handle to parameters key in registry
//      szPreNT5InfId    [in]  pre-NT5 InfID
//      szPreNT5Instance [in]  pre-NT5 instance name
//      szNT5InfId       [in]  NT5 InfId
//      szSectionName    [in]  section name to be used for writing info
//      pviVendorInfo    [out] information about OEM
//      pdwFlags         [out] pointer to flags
//      pvReserved       [in]  reserved
//
//  Returns:    ERROR_SUCCESS in case of success, win32 error otherwise
//
//  Notes:
//          This function is obsolete for Windows XP (SP1 and later),
//          Windows .NET Server, and later operating systems.
//
//      This function is called once per component to be upgraded.
//
LONG __stdcall
DoPreUpgradeProcessing(_In_   HWND        hParentWindow,
                       _In_   HKEY        hkeyParams,
                       _In_   LPCTSTR     szPreNT5InfId,
                       _In_   LPCTSTR     szPreNT5Instance,
                       _In_   LPCTSTR     szNT5InfId,
                       _In_   LPCTSTR     szSectionName,
                       _Out_  VENDORINFO* pviVendorInfo,
                       _Out_  DWORD*      pdwFlags,
                       _In_   LPVOID      pvReserved);

typedef LONG
(__stdcall *DoPreUpgradeProcessingPrototype)
     (IN   HWND    hParentWindow,
      IN   HKEY    hkeyParams,
      IN   LPCTSTR szPreNT5InfId,
      IN   LPCTSTR szPreNT5Instance,
      IN   LPCTSTR szNT5InfId,
      IN   LPCTSTR szSectionName,
      OUT  VENDORINFO* pviVendorInfo,
      OUT  DWORD*  pdwFlags,
      IN   LPVOID  pvReserved);


//+---------------------------------------------------------------------------
//
//  Function:   PostUpgradeInitialize
//
//  Purpose:    Intialize OEM DLL during GUI mode setup
//
//  Arguments:
//      szWorkingDir     [in]   name of temporary directory to be used
//      pNetUpgradeInfo  [in]   pointer to NetUpgradeInfo structure
//      pviVendorInfo    [out]  information about OEM
//      pvReserved       [out]  reserved
//
//  Returns:    ERROR_SUCCESS in case of success, win32 error otherwise
//
//  Notes:
//          This function is obsolete for Windows XP (SP1 and later),
//          Windows .NET Server, and later operating systems.
//
//      This function is called in GUI mode setup before
//      any other function in this dll .
//      The main purpose of calling this function is to obtain
//      identification information and to allow the DLL to initialize
//      its internal data
//
__drv_preferredFunction("(see documentation)", "Obsolete") 
LONG __stdcall
PostUpgradeInitialize(IN LPCTSTR          szWorkingDir,
                      _In_  NetUpgradeInfo* pNetUpgradeInfo,
                      _Out_ VENDORINFO*     pviVendorInfo,
                      _Out_ LPVOID          pvReserved);

typedef LONG
(__stdcall *PostUpgradeInitializePrototype)
     (IN LPCTSTR          szWorkingDir,
      IN  NetUpgradeInfo* pNetUpgradeInfo,
      OUT VENDORINFO*     pviVendorInfo,
      OUT LPVOID          pvReserved);



//+---------------------------------------------------------------------------
//
//  Function:   DoPostUpgradeProcessing
//
//  Purpose:    Intialize OEM DLL
//
//  Arguments:
//      hParentWindow    [in]  window handle for showing UI
//      hkeyParams       [in]  handle to parameters key in registry
//      szPreNT5Instance [in]  pre-NT5 instance name
//      szNT5InfId       [in]  NT5 InfId
//      hinfAnswerFile   [in]  handle to answer-file
//      szSectionName    [in]  name of section having component parameters
//      pviVendorInfo    [out] information about OEM
//      pvReserved       [in]  reserved
//
//  Returns:    ERROR_SUCCESS in case of success, win32 error otherwise
//
//  Notes:
//          This function is obsolete for Windows XP (SP1 and later),
//          Windows .NET Server, and later operating systems.
//
//      This function is called once per component upgraded.
//
LONG __stdcall
DoPostUpgradeProcessing(_In_  HWND    hParentWindow,
                        _In_  HKEY    hkeyParams,
                        _In_  LPCTSTR szPreNT5Instance,
                        _In_  LPCTSTR szNT5InfId,
                        _In_  HINF    hinfAnswerFile,
                        _In_  LPCTSTR szSectionName,
                        _Out_ VENDORINFO* pviVendorInfo,
                        _In_  LPVOID  pvReserved);

typedef LONG
(__stdcall *DoPostUpgradeProcessingPrototype)
     (IN  HWND    hParentWindow,
      IN  HKEY    hkeyParams,
      IN  LPCTSTR szPreNT5Instance,
      IN  LPCTSTR szNT5InfId,
      IN  HINF    hinfAnswerFile,
      IN  LPCTSTR szSectionName,
      OUT VENDORINFO* pviVendorInfo,
      IN  LPVOID  pvReserved);


//
// Prototypes for functions exported by netupgrd.dll
//
//
//          These functions are obsolete for Windows XP (SP1 and later),
//          Windows .NET Server, and later operating systems.


__drv_preferredFunction("(see documentation)", "Obsolete") 
LONG __stdcall
NetUpgradeAddSection(_In_ LPCTSTR szSectionName);

typedef LONG
(__stdcall *NetUpgradeAddSectionPrototype) (IN LPCTSTR szSectionName);


__drv_preferredFunction("(see documentation)", "Obsolete") 
LONG __stdcall
NetUpgradeAddLineToSection(_In_ LPCTSTR szSectionName,
                           _In_ LPCTSTR szLine);

typedef LONG
(__stdcall *NetUpgradeAddLineToSectionPrototype) (IN LPCTSTR szSectionName,
                                                  IN LPCTSTR szLine);


#ifdef __cplusplus
}
#endif

#endif      // NTDDI_VERSION

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif      // __OEMUPGEX_H__ 
