//  Copyright (C) 1995-1999 Microsoft Corporation.  All rights reserved.
/* ----------------------------------------------------------------------------
 Microsoft   D.T.C (Distributed Transaction Coordinator)

 (c) 1995    Microsoft Corporation.  All Rights Reserved


Filename :  xolehlp.h
            contains DTC helper APIs used by RM's and application clients
            to obtain the transaction manager
----------------------------------------------------------------------------- */

#ifndef __XOLEHLP__H__
#define __XOLEHLP__H__
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)



/*----------------------------------------
//  Defines
//--------------------------------------*/
#ifdef _M_CEE_PURE
#define EXPORTAPI HRESULT
#else
#define EXPORTAPI STDAPI
#endif

/*----------------------------------------
// Constants
//--------------------------------------*/
const DWORD     OLE_TM_CONFIG_VERSION_1     = 1;
const DWORD     OLE_TM_CONFIG_VERSION_2     = 2;

typedef enum _APPLICATIONTYPE
{
    LOCAL_APPLICATIONTYPE,
    CLUSTERRESOURCE_APPLICATIONTYPE
} APPLICATIONTYPE;

const DWORD     OLE_TM_FLAG_NONE            = 0x00000000;
const DWORD     OLE_TM_FLAG_NODEMANDSTART   = 0x00000001;

// If this flag is set, the application specifies that it does not wish 
// to take advantage of any features that need agile recovery 
// support. 
// As a consequence,  the application will be restricted to 
// using the default transaction manager on a cluster.
const DWORD     OLE_TM_FLAG_NOAGILERECOVERY = 0x00000002;

// The following are flags used specifically for MSDTC.
const DWORD     OLE_TM_FLAG_QUERY_SERVICE_LOCKSTATUS = 0x80000000;
const DWORD     OLE_TM_FLAG_INTERNAL_TO_TM  =          0x40000000;

/*----------------------------------------
//  Structure definitions
//--------------------------------------*/
typedef struct _OLE_TM_CONFIG_PARAMS_V1
{
    DWORD       dwVersion;
    DWORD       dwcConcurrencyHint;
} OLE_TM_CONFIG_PARAMS_V1;

typedef struct _OLE_TM_CONFIG_PARAMS_V2
{
    DWORD           dwVersion;
    DWORD           dwcConcurrencyHint;
    APPLICATIONTYPE applicationType;
    GUID            clusterResourceId;
} OLE_TM_CONFIG_PARAMS_V2;


/*----------------------------------------
//  Function Prototypes
//--------------------------------------*/

/*----------------------------------------
//This API should be used to obtain an IUnknown or a ITransactionDispenser
//interface from the Microsoft Distributed Transaction Coordinator's proxy.
//Typically, a NULL is passed for the host name and the TM Name. In which 
//case the MS DTC on the same host is contacted and the interface provided
//for it.
//--------------------------------------*/
EXTERN_C HRESULT __cdecl DtcGetTransactionManager(
                                    /* in */ _In_opt_ LPSTR i_pszHost,
                                    /* in */ _In_opt_ LPSTR i_pszTmName,
                                    /* in */ REFIID i_riid,
                                    /* in */ DWORD i_dwReserved1,
                                    /* in */ WORD i_wcbReserved2,
                                    /* in */ _In_reads_bytes_opt_(i_wcbReserved2) void * i_pvReserved2,
                                    /* out */ void** o_ppvObject
                                    );
EXTERN_C HRESULT __cdecl DtcGetTransactionManagerC(
                                    /* in */ _In_opt_ LPSTR i_pszHost,
                                    /* in */ _In_opt_ LPSTR i_pszTmName,
                                    /* in */ REFIID i_riid,
                                    /* in */ DWORD i_dwReserved1,
                                    /* in */ WORD i_wcbReserved2,
                                    /* in */ _In_reads_bytes_opt_(i_wcbReserved2) void * i_pvReserved2,
                                    /* out */ void ** o_ppvObject
                                    );

EXTERN_C HRESULT __cdecl DtcGetTransactionManagerExA(
                                    /* in */ _In_opt_ LPSTR i_pszHost,
                                    /* in */ _In_opt_ LPSTR i_pszTmName,
                                    /* in */ REFIID i_riid,
                                    /* in */ DWORD i_grfOptions,
                                    /* in */ void * i_pvConfigParams,
                                    /* out */ void ** o_ppvObject
                                    );


EXTERN_C HRESULT __cdecl DtcGetTransactionManagerExW(
                                    /* in */ _In_opt_ LPWSTR i_pwszHost,
                                    /* in */ _In_opt_ LPWSTR i_pwszTmName,
                                    /* in */ REFIID i_riid,
                                    /* in */ DWORD i_grfOptions,
                                    /* in */ void * i_pvConfigParams,
                                    /* out */ void ** o_ppvObject
                                    );
#ifdef UNICODE
#define DtcGetTransactionManagerEx      DtcGetTransactionManagerExW
#else
#define DtcGetTransactionManagerEx      DtcGetTransactionManagerExA
#endif


#ifndef EXTERN_GUID
#define EXTERN_GUID(g,l1,s1,s2,c1,c2,c3,c4,c5,c6,c7,c8) DEFINE_GUID(g,l1,s1,s2,c1,c2,c3,c4,c5,c6,c7,c8)
#endif

/*----------------------------------------
// Define a CLSID that can be used to obtain a transaction manager instance via CoCreateInstance;
// this is an alternate to using DtcGetTransactionManager. 
//
// CLSID_MSDtcTransactionManager = {5B18AB61-091D-11d1-97DF-00C04FB9618A}
//--------------------------------------*/
EXTERN_GUID(CLSID_MSDtcTransactionManager, 0x5b18ab61, 0x91d, 0x11d1, 0x97, 0xdf, 0x0, 0xc0, 0x4f, 0xb9, 0x61, 0x8a);

/*----------------------------------------
// Define a CLSID that can be used with CoCreateInstance to instantiate a vanilla transaction
// object with the local transaction manager. It's equivalent to doing 
//
//  pTransactionDispenser->BeginTransaction(NULL, ISOLATIONLEVEL_UNSPECIFIED, ISOFLAG_RETAIN_DONTCARE, NULL, &ptx);
//
// CLSID_MSDtcTransaction = {39F8D76B-0928-11d1-97DF-00C04FB9618A}
//--------------------------------------*/
EXTERN_GUID(CLSID_MSDtcTransaction, 0x39f8d76b, 0x928, 0x11d1, 0x97, 0xdf, 0x0, 0xc0, 0x4f, 0xb9, 0x61, 0x8a);


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif
