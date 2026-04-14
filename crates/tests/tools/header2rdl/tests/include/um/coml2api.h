//+---------------------------------------------------------------------------
//
//  Microsoft Windows
//  Copyright (c) Microsoft Corporation. All rights reserved.
//
//  File:       coml2api.h
//
//  Contents:   Structured storage, property sets, and related APIs.
//
//----------------------------------------------------------------------------

#if !defined(_COML2API_H_)
#define _COML2API_H_

#ifdef _MSC_VER
#pragma once
#endif  // _MSC_VER

#include <apiset.h>
#include <apisetcconv.h>

#include <combaseapi.h>
#include <objidl.h>
#include <propidlbase.h>

#pragma region Application and Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)

//
// Common typedefs for paramaters used in Storage API's, gleamed from storage.h
// Also contains Storage error codes, which should be moved into the storage
// idl files.
//

#define CWCSTORAGENAME 32

/* Storage instantiation modes */
#define STGM_DIRECT             0x00000000L
#define STGM_TRANSACTED         0x00010000L
#define STGM_SIMPLE             0x08000000L

#define STGM_READ               0x00000000L
#define STGM_WRITE              0x00000001L
#define STGM_READWRITE          0x00000002L

#define STGM_SHARE_DENY_NONE    0x00000040L
#define STGM_SHARE_DENY_READ    0x00000030L
#define STGM_SHARE_DENY_WRITE   0x00000020L
#define STGM_SHARE_EXCLUSIVE    0x00000010L

#define STGM_PRIORITY           0x00040000L
#define STGM_DELETEONRELEASE    0x04000000L
#if (WINVER >= 400)
#define STGM_NOSCRATCH          0x00100000L
#endif /* WINVER */

#define STGM_CREATE             0x00001000L
#define STGM_CONVERT            0x00020000L
#define STGM_FAILIFTHERE        0x00000000L

#define STGM_NOSNAPSHOT         0x00200000L
#if (_WIN32_WINNT >= 0x0500)
#define STGM_DIRECT_SWMR        0x00400000L
#endif

typedef DWORD STGFMT;

#define STGFMT_STORAGE          0
#define STGFMT_NATIVE           1
#define STGFMT_FILE             3
#define STGFMT_ANY              4
#define STGFMT_DOCFILE          5

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)

// This is a legacy define to allow old component to builds
#define STGFMT_DOCUMENT         0

// Structured storage APIs
_Check_return_
WINOLEAPI
StgCreateDocfile(
    _In_opt_ _Null_terminated_ const WCHAR* pwcsName,
    _In_ DWORD grfMode,
    _Reserved_ DWORD reserved,
    _Outptr_ IStorage** ppstgOpen
    );

_Check_return_
WINOLEAPI
StgCreateDocfileOnILockBytes(
    _In_ ILockBytes* plkbyt,
    _In_ DWORD grfMode,
    _In_ DWORD reserved,
    _Outptr_ IStorage** ppstgOpen
    );

_Check_return_
WINOLEAPI
StgOpenStorage(
    _In_opt_ _Null_terminated_ const WCHAR* pwcsName,
    _In_opt_ IStorage* pstgPriority,
    _In_ DWORD grfMode,
    _In_opt_z_ SNB snbExclude,
    _In_ DWORD reserved,
    _Outptr_ IStorage** ppstgOpen
    );

_Check_return_
WINOLEAPI
StgOpenStorageOnILockBytes(
    _In_ ILockBytes* plkbyt,
    _In_opt_ IStorage* pstgPriority,
    _In_ DWORD grfMode,
    _In_opt_z_ SNB snbExclude,
    _Reserved_ DWORD reserved,
    _Outptr_ IStorage** ppstgOpen
    );

_Check_return_
WINOLEAPI
StgIsStorageFile(
    _In_ _Null_terminated_ const WCHAR* pwcsName
    );

_Check_return_
WINOLEAPI
StgIsStorageILockBytes(
    _In_ ILockBytes* plkbyt
    );

_Check_return_
WINOLEAPI
StgSetTimes(
    _In_ _Null_terminated_ const WCHAR* lpszName,
    _In_opt_ const FILETIME* pctime,
    _In_opt_ const FILETIME* patime,
    _In_opt_ const FILETIME* pmtime
    );

// STG initialization options for StgCreateStorageEx and StgOpenStorageEx
#if _WIN32_WINNT == 0x500
#define STGOPTIONS_VERSION 1
#elif _WIN32_WINNT > 0x500
#define STGOPTIONS_VERSION 2
#else
#define STGOPTIONS_VERSION 0
#endif

typedef struct tagSTGOPTIONS
{
    USHORT usVersion;            // Versions 1 and 2 supported
    USHORT reserved;             // must be 0 for padding
    ULONG ulSectorSize;          // docfile header sector size (512)
#if STGOPTIONS_VERSION >= 2
    const WCHAR *pwcsTemplateFile;  // version 2 or above
#endif
} STGOPTIONS;

_Check_return_
WINOLEAPI
StgCreateStorageEx(
    _In_opt_ _Null_terminated_ const WCHAR* pwcsName,
    _In_ DWORD grfMode,
    _In_ DWORD stgfmt,
    _In_ DWORD grfAttrs,
    _Inout_opt_ STGOPTIONS* pStgOptions,
    _In_opt_ PSECURITY_DESCRIPTOR pSecurityDescriptor,
    _In_ REFIID riid,
    _Outptr_ void** ppObjectOpen
    );

_Check_return_
WINOLEAPI
StgOpenStorageEx(
    _In_ _Null_terminated_ const WCHAR* pwcsName,
    _In_ DWORD grfMode,
    _In_ DWORD stgfmt,
    _In_ DWORD grfAttrs,
    _Inout_opt_ STGOPTIONS* pStgOptions,
    _In_opt_ PSECURITY_DESCRIPTOR pSecurityDescriptor,
    _In_ REFIID riid,
    _Outptr_ void** ppObjectOpen
    );

#ifndef _STGCREATEPROPSTG_DEFINED_

_Check_return_
WINOLEAPI
StgCreatePropStg(
    _In_ IUnknown* pUnk,
    _In_ REFFMTID fmtid,
    _In_ const CLSID* pclsid,
    _In_ DWORD grfFlags,
    _Reserved_ DWORD dwReserved,
    _Outptr_ IPropertyStorage** ppPropStg
    );

_Check_return_
WINOLEAPI
StgOpenPropStg(
    _In_ IUnknown* pUnk,
    _In_ REFFMTID fmtid,
    _In_ DWORD grfFlags,
    _Reserved_ DWORD dwReserved,
    _Outptr_ IPropertyStorage** ppPropStg
    );

_Check_return_
WINOLEAPI
StgCreatePropSetStg(
    _In_ IStorage* pStorage,
    _Reserved_ DWORD dwReserved,
    _Outptr_ IPropertySetStorage** ppPropSetStg
    );

#define CCH_MAX_PROPSTG_NAME    31

_Check_return_
WINOLEAPI
FmtIdToPropStgName(
    _In_ const FMTID* pfmtid,
    _Out_writes_(CCH_MAX_PROPSTG_NAME+1) LPOLESTR oszName
    );

_Check_return_
WINOLEAPI
PropStgNameToFmtId(
    _In_ const LPOLESTR oszName,
    _Out_ FMTID* pfmtid
    );

#endif // _STGCREATEPROPSTG_DEFINED_

// Helper functions
WINOLEAPI
ReadClassStg(
    _In_ LPSTORAGE pStg,
    _Out_ CLSID  FAR * pclsid
    );

WINOLEAPI
WriteClassStg(
    _In_ LPSTORAGE pStg,
    _In_ REFCLSID rclsid
    );

WINOLEAPI
ReadClassStm(
    _In_ LPSTREAM pStm,
    _Out_ CLSID  FAR * pclsid
    );

WINOLEAPI
WriteClassStm(
    _In_ LPSTREAM pStm,
    _In_ REFCLSID rclsid
    );

// Storage utility APIs
_Check_return_
WINOLEAPI
GetHGlobalFromILockBytes(
    _In_ LPLOCKBYTES plkbyt,
    _Out_ HGLOBAL  FAR * phglobal
    );

_Check_return_
WINOLEAPI
CreateILockBytesOnHGlobal(
    _In_opt_ HGLOBAL hGlobal,
    _In_ BOOL fDeleteOnRelease,
    _Outptr_ LPLOCKBYTES  FAR * pplkbyt
    );

// ConvertTo APIs
WINOLEAPI
GetConvertStg(
    _In_ LPSTORAGE pStg
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion

#endif     // __COML2API_H__
