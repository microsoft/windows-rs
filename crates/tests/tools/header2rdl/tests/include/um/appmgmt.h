//*************************************************************
//
// appmgmt.h
//
// APIs for operations on MSI applications which are deployed
// and managed in the NT Directory.
//
// Copyright (c) Microsoft Corporation 1998-1999
// All rights reserved
//
//*************************************************************

#ifndef _APPMGMT_H_
#define _APPMGMT_H_

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#ifdef __cplusplus
extern "C" {
#endif

typedef enum _INSTALLSPECTYPE
{
    APPNAME = 1,
    FILEEXT,
    PROGID,
    COMCLASS
} INSTALLSPECTYPE;

typedef union _INSTALLSPEC
{
    struct
    {
        WCHAR * Name;
        GUID    GPOId;
    } AppName;
    WCHAR * FileExt;
    WCHAR * ProgId;
    struct
    {
        GUID    Clsid;
        DWORD   ClsCtx;
    } COMClass;
} INSTALLSPEC;

typedef struct _INSTALLDATA
{
    INSTALLSPECTYPE Type;
    INSTALLSPEC     Spec;
} INSTALLDATA, *PINSTALLDATA;

typedef enum
{
    ABSENT,
    ASSIGNED,
    PUBLISHED
} APPSTATE;

#define LOCALSTATE_ASSIGNED                 0x1     // app is assigned
#define LOCALSTATE_PUBLISHED                0x2     // app is published
#define LOCALSTATE_UNINSTALL_UNMANAGED      0x4     // uninstall any unmanaged version before assigning
#define LOCALSTATE_POLICYREMOVE_ORPHAN      0x8     // app is orphaned when policy removed
#define LOCALSTATE_POLICYREMOVE_UNINSTALL   0x10    // app is uninstalled when policy removed
#define LOCALSTATE_ORPHANED                 0x20    // app is orphaned after being applied
#define LOCALSTATE_UNINSTALLED              0x40    // app is uninstalled after being applied

typedef struct _LOCALMANAGEDAPPLICATION
{
    LPWSTR      pszDeploymentName;
    LPWSTR      pszPolicyName;
    LPWSTR      pszProductId;
    DWORD       dwState;
} LOCALMANAGEDAPPLICATION, *PLOCALMANAGEDAPPLICATION;

#define MANAGED_APPS_USERAPPLICATIONS  0x1
#define MANAGED_APPS_FROMCATEGORY      0x2
#define MANAGED_APPS_INFOLEVEL_DEFAULT 0x10000

#define MANAGED_APPTYPE_WINDOWSINSTALLER 0x1
#define MANAGED_APPTYPE_SETUPEXE         0x2
#define MANAGED_APPTYPE_UNSUPPORTED      0x3

typedef struct _MANAGEDAPPLICATION
{
    LPWSTR      pszPackageName;
    LPWSTR      pszPublisher;
    DWORD       dwVersionHi;
    DWORD       dwVersionLo;
    DWORD       dwRevision;
    GUID        GpoId;
    LPWSTR      pszPolicyName;
    GUID        ProductId;
    LANGID      Language;
    LPWSTR      pszOwner;
    LPWSTR      pszCompany;
    LPWSTR      pszComments;
    LPWSTR      pszContact;
    LPWSTR      pszSupportUrl;
    DWORD       dwPathType;
    BOOL        bInstalled;
} MANAGEDAPPLICATION, *PMANAGEDAPPLICATION;

typedef struct _APPCATEGORYINFO
{
    LCID        Locale;
    LPWSTR      pszDescription;
    GUID        AppCategoryId;
} APPCATEGORYINFO;

typedef struct _APPCATEGORYINFOLIST
{
    DWORD               cCategory;
#ifdef MIDL_PASS
    [size_is(cCategory)]
#endif
    APPCATEGORYINFO *   pCategoryInfo;
} APPCATEGORYINFOLIST;

#ifndef WINAPI
#define WINAPI
#endif

DWORD WINAPI
InstallApplication(
#if !defined(__midl)
    _In_ 
#endif
	PINSTALLDATA pInstallInfo
    );

DWORD WINAPI
UninstallApplication(
#if !defined(__midl)
    _In_ 
#endif
    LPWSTR      ProductCode,
#if !defined(__midl)
    _In_ 
#endif
	DWORD       dwStatus
    );

DWORD WINAPI
CommandLineFromMsiDescriptor(
#if !defined(__midl)
    _In_
#endif
    LPWSTR      Descriptor,
#if !defined(__midl)
    _Out_writes_( *CommandLineLength )
#endif
    LPWSTR      CommandLine,
    DWORD *     CommandLineLength
    );

DWORD WINAPI
GetManagedApplications(
#if !defined(__midl)
    _In_ 
#endif
    GUID *               pCategory,
#if !defined(__midl)
    _In_ 
#endif
    DWORD                dwQueryFlags,
#if !defined(__midl)
    _In_ 
#endif
	DWORD                dwInfoLevel,
#if !defined(__midl)
    _Out_ 
#endif
	LPDWORD              pdwApps,
#if !defined(__midl)
    _Out_ 
#endif
	PMANAGEDAPPLICATION* prgManagedApps
    );

DWORD WINAPI
GetLocalManagedApplications(
#if !defined(__midl)
    _In_ 
#endif
	BOOL                        bUserApps,
#if !defined(__midl)
    _Out_ 
#endif
	LPDWORD                     pdwApps,
#if !defined(__midl)
    _Out_ 
#endif
	PLOCALMANAGEDAPPLICATION*   prgLocalApps
    );

void WINAPI
GetLocalManagedApplicationData(
#if !defined(__midl)
    _In_ 
#endif
	LPWSTR              ProductCode,
#if !defined(__midl)
    _Outptr_ 
#endif
	LPWSTR *            DisplayName,
#if !defined(__midl)
    _Outptr_ 
#endif
	LPWSTR *            SupportUrl
    );

DWORD WINAPI
GetManagedApplicationCategories(
#if !defined(__midl)
    _Reserved_ 
#endif
	DWORD                dwReserved,
#if !defined(__midl)
    _Out_ 
#endif
	APPCATEGORYINFOLIST* pAppCategory
    );

#ifdef __cplusplus
}
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif
