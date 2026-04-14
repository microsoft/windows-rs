#include <winapifamily.h>


//+---------------------------------------------------------------------------
//
//  Microsoft Windows
//  Copyright (c) Microsoft Corporation. All rights reserved.
//
//  File:       objbase.h
//
//  Contents:   Component object model definitions.
//
//----------------------------------------------------------------------------

#include <rpc.h>
#include <rpcndr.h>


#if !defined( _OBJBASE_H_ )
#define _OBJBASE_H_

#if _MSC_VER > 1000
#pragma once
#endif

#include <pshpack8.h>
#include <combaseapi.h>
#include <coml2api.h>

// COM initialization flags; passed to CoInitialize.
typedef enum tagCOINIT
{
  COINIT_APARTMENTTHREADED  = 0x2,      // Apartment model

#if  (_WIN32_WINNT >= 0x0400 ) || defined(_WIN32_DCOM) // DCOM
  // These constants are only valid on Windows NT 4.0
  COINIT_MULTITHREADED      = COINITBASE_MULTITHREADED,
  COINIT_DISABLE_OLE1DDE    = 0x4,      // Don't use DDE for Ole1 support.
  COINIT_SPEED_OVER_MEMORY  = 0x8,      // Trade memory for speed.
#endif // DCOM
} COINIT;



// interface marshaling definitions
#define MARSHALINTERFACE_MIN 500 // minimum number of bytes for interface marshal


/*  flags for internet asynchronous and layout docfile */
#define ASYNC_MODE_COMPATIBILITY    0x00000001L
#define ASYNC_MODE_DEFAULT          0x00000000L

#define STGTY_REPEAT                0x00000100L
#define STG_TOEND                   0xFFFFFFFFL

#define STG_LAYOUT_SEQUENTIAL       0x00000000L
#define STG_LAYOUT_INTERLEAVED      0x00000001L

#include <objidl.h>

#ifdef _OLE32_
#ifdef _OLE32PRIV_
BOOL _fastcall wIsEqualGUID(REFGUID rguid1, REFGUID rguid2);
#define IsEqualGUID(rguid1, rguid2) wIsEqualGUID(rguid1, rguid2)
#else
#define __INLINE_ISEQUAL_GUID
#endif  // _OLE32PRIV_
#endif  // _OLE32_


#pragma region Desktop Partition
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

/****** STD Object API Prototypes *****************************************/

WINOLEAPI_(DWORD) CoBuildVersion( VOID );

/* init/uninit */

_Check_return_ WINOLEAPI  CoInitialize(_In_opt_ LPVOID pvReserved);

#define COM_SUPPORT_MALLOC_SPIES
_Check_return_ WINOLEAPI  CoRegisterMallocSpy(_In_ LPMALLOCSPY pMallocSpy);
_Check_return_ WINOLEAPI  CoRevokeMallocSpy(void);
WINOLEAPI  CoCreateStandardMalloc(_In_ DWORD memctx, _Outptr_ IMalloc FAR* FAR* ppMalloc);

#if (_WIN32_WINNT >= 0x0501)


_Check_return_ WINOLEAPI  CoRegisterInitializeSpy(_In_ IInitializeSpy *pSpy, _Out_ ULARGE_INTEGER *puliCookie);
_Check_return_ WINOLEAPI  CoRevokeInitializeSpy(_In_ ULARGE_INTEGER uliCookie);

// COM System Security Descriptors (used when the corresponding registry
// entries are absent)
typedef enum tagCOMSD
{
    SD_LAUNCHPERMISSIONS = 0,       // Machine wide launch permissions
    SD_ACCESSPERMISSIONS = 1,       // Machine wide acesss permissions
    SD_LAUNCHRESTRICTIONS = 2,      // Machine wide launch limits
    SD_ACCESSRESTRICTIONS = 3       // Machine wide access limits

} COMSD;
_Check_return_ WINOLEAPI  CoGetSystemSecurityPermissions(COMSD comSDType, PSECURITY_DESCRIPTOR *ppSD);

#endif

/* dll loading helpers; keeps track of ref counts and unloads all on exit */

WINOLEAPI_(HINSTANCE) CoLoadLibrary(_In_ LPOLESTR lpszLibName, _In_ BOOL bAutoFree);
WINOLEAPI_(void) CoFreeLibrary(_In_ HINSTANCE hInst);
WINOLEAPI_(void) CoFreeAllLibraries(void);


#if (_WIN32_WINNT >= 0x0400 ) || defined(_WIN32_DCOM) // DCOM


_Check_return_ WINOLEAPI CoGetInstanceFromFile(
    _In_opt_ COSERVERINFO *            pServerInfo,
    _In_opt_ CLSID        *            pClsid,
    _In_opt_ IUnknown     *            punkOuter, // only relevant locally
    _In_ DWORD                         dwClsCtx,
    _In_ DWORD                         grfMode,
    _In_ _Null_terminated_ OLECHAR *    pwszName,
    _In_ DWORD                         dwCount,
    _Inout_updates_(dwCount) MULTI_QI * pResults );


_Check_return_ WINOLEAPI CoGetInstanceFromIStorage(
    _In_opt_ COSERVERINFO *            pServerInfo,
    _In_opt_ CLSID        *            pClsid,
    _In_opt_ IUnknown     *            punkOuter, // only relevant locally
    _In_ DWORD                         dwClsCtx,
    _In_ struct IStorage  *            pstg,
    _In_ DWORD                         dwCount,
    _Inout_updates_(dwCount) MULTI_QI * pResults );

#endif // DCOM

/* Call related APIs */
#if (_WIN32_WINNT >= 0x0500 ) || defined(_WIN32_DCOM) // DCOM


WINOLEAPI CoAllowSetForegroundWindow(_In_ IUnknown *pUnk, _In_opt_ LPVOID lpvReserved);


WINOLEAPI DcomChannelSetHResult(_In_opt_ LPVOID pvReserved, _In_opt_ ULONG* pulReserved, _In_ HRESULT appsHR);

#endif

/* other helpers */
WINOLEAPI_(BOOL) CoIsOle1Class(_In_ REFCLSID rclsid);
_Check_return_ WINOLEAPI CLSIDFromProgIDEx (_In_ LPCOLESTR lpszProgID, _Out_ LPCLSID lpclsid);

WINOLEAPI_(BOOL) CoFileTimeToDosDateTime(
                 _In_ FILETIME FAR* lpFileTime, _Out_ LPWORD lpDosDate, _Out_ LPWORD lpDosTime);
WINOLEAPI_(BOOL) CoDosDateTimeToFileTime(
                       _In_ WORD nDosDate, _In_ WORD nDosTime, _Out_ FILETIME FAR* lpFileTime);
WINOLEAPI  CoFileTimeNow( _Out_ FILETIME FAR* lpFileTime );

_Check_return_ WINOLEAPI CoRegisterMessageFilter( _In_opt_ LPMESSAGEFILTER lpMessageFilter,
                                _Outptr_opt_result_maybenull_ LPMESSAGEFILTER FAR* lplpMessageFilter );

#if (_WIN32_WINNT >= 0x0400 ) || defined(_WIN32_DCOM) // DCOM

WINOLEAPI CoRegisterChannelHook( _In_ REFGUID ExtensionUuid, _In_ IChannelHook *pChannelHook );
#endif // DCOM


/* TreatAs APIS */

_Check_return_ WINOLEAPI CoTreatAsClass(_In_ REFCLSID clsidOld, _In_ REFCLSID clsidNew);


/****** DV APIs ***********************************************************/

WINOLEAPI CreateDataAdviseHolder(_Outptr_ LPDATAADVISEHOLDER FAR* ppDAHolder);

WINOLEAPI CreateDataCache(_In_opt_ LPUNKNOWN pUnkOuter, _In_ REFCLSID rclsid,
                          _In_ REFIID iid, _Out_ LPVOID FAR* ppv);


/****** Storage API Prototypes ********************************************/


_Check_return_ WINOLEAPI StgOpenAsyncDocfileOnIFillLockBytes( _In_ IFillLockBytes *pflb,
             _In_ DWORD grfMode,
             _In_ DWORD asyncFlags,
             _Outptr_ IStorage** ppstgOpen);

_Check_return_ WINOLEAPI StgGetIFillLockBytesOnILockBytes( _In_ ILockBytes *pilb,
             _Outptr_ IFillLockBytes** ppflb);

_Check_return_ WINOLEAPI StgGetIFillLockBytesOnFile(_In_ _Null_terminated_ OLECHAR const *pwcsName,
             _Outptr_ IFillLockBytes** ppflb);

_Check_return_ WINOLEAPI StgOpenLayoutDocfile(_In_ _Null_terminated_ OLECHAR const *pwcsDfName,
             _In_ DWORD grfMode,
             _In_ DWORD reserved,
             _Outptr_ IStorage** ppstgOpen);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#pragma region Desktop Partition
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

WINOLEAPI  CoInstall(
    _In_ IBindCtx     * pbc,
    _In_ DWORD          dwFlags,
    _In_ uCLSSPEC     * pClassSpec,
    _In_ QUERYCONTEXT * pQuery,
    _In_ LPWSTR         pszCodeBase);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#pragma region PC Application Partition
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_PC_APP)

//
//  Moniker APIs
//

_Check_return_ WINOLEAPI  BindMoniker(_In_ LPMONIKER pmk, _In_ DWORD grfOpt, _In_ REFIID iidResult, _Outptr_ LPVOID FAR* ppvResult);

_Check_return_ WINOLEAPI  CoGetObject(_In_ LPCWSTR pszName, _In_opt_ BIND_OPTS *pBindOptions, _In_ REFIID riid, _Outptr_ void **ppv);
_Check_return_ WINOLEAPI  MkParseDisplayName(_In_ LPBC pbc, _In_ LPCOLESTR szUserName,
                _Out_ ULONG FAR * pchEaten, _Outptr_ LPMONIKER FAR * ppmk);
_Check_return_ WINOLEAPI  MonikerRelativePathTo(_In_ LPMONIKER pmkSrc, _In_ LPMONIKER pmkDest, _Outptr_ LPMONIKER
                FAR* ppmkRelPath, _In_ BOOL dwReserved);
_Check_return_ WINOLEAPI  MonikerCommonPrefixWith(_In_ LPMONIKER pmkThis, _In_ LPMONIKER pmkOther,
                _Outptr_ LPMONIKER FAR* ppmkCommon);
_Check_return_ WINOLEAPI  CreateBindCtx(_In_ DWORD reserved, _Outptr_ LPBC FAR* ppbc);
_Check_return_ WINOLEAPI  CreateGenericComposite(_In_opt_ LPMONIKER pmkFirst, _In_opt_ LPMONIKER pmkRest,
                _Outptr_ LPMONIKER FAR* ppmkComposite);
_Check_return_ WINOLEAPI  GetClassFile (_In_ LPCOLESTR szFilename, _Out_ CLSID FAR* pclsid);

_Check_return_ WINOLEAPI  CreateClassMoniker(_In_ REFCLSID rclsid, _Outptr_ LPMONIKER FAR* ppmk);

_Check_return_ WINOLEAPI  CreateFileMoniker(_In_ LPCOLESTR lpszPathName, _Outptr_ LPMONIKER FAR* ppmk);

_Check_return_ WINOLEAPI  CreateItemMoniker(_In_ LPCOLESTR lpszDelim, _In_ LPCOLESTR lpszItem,
                          _Outptr_ LPMONIKER FAR* ppmk);
_Check_return_ WINOLEAPI  CreateAntiMoniker(_Outptr_ LPMONIKER FAR* ppmk);
_Check_return_ WINOLEAPI  CreatePointerMoniker(_In_opt_ LPUNKNOWN punk, _Outptr_ LPMONIKER FAR* ppmk);
_Check_return_ WINOLEAPI  CreateObjrefMoniker(_In_opt_ LPUNKNOWN punk, _Outptr_ LPMONIKER FAR * ppmk);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_PC_APP) */
#pragma endregion

#pragma region Desktop Partition
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

_Check_return_ WINOLEAPI  GetRunningObjectTable( _In_ DWORD reserved, _Outptr_ LPRUNNINGOBJECTTABLE FAR* pprot);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#include <urlmon.h>
#include <propidl.h>

#pragma region Desktop Partition
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

//
// Standard Progress Indicator implementation
//
WINOLEAPI CreateStdProgressIndicator(_In_ HWND hwndParent,
                                   _In_ LPCOLESTR pszTitle,
                                   _In_ IBindStatusCallback * pIbscCaller,
                                   _Outptr_ IBindStatusCallback ** ppIbsc);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


#ifndef RC_INVOKED
#include <poppack.h>
#endif // RC_INVOKED

#endif     // __OBJBASE_H__
