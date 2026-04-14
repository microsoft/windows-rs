

/* this ALWAYS GENERATED file contains the definitions for the interfaces */


 /* File created by MIDL compiler version 8.01.0628 */
/* @@MIDL_FILE_HEADING(  ) */



/* verify that the <rpcndr.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCNDR_H_VERSION__
#define __REQUIRED_RPCNDR_H_VERSION__ 500
#endif

/* verify that the <rpcsal.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCSAL_H_VERSION__
#define __REQUIRED_RPCSAL_H_VERSION__ 100
#endif

#include "rpc.h"
#include "rpcndr.h"

#ifndef __RPCNDR_H_VERSION__
#error this stub requires an updated version of <rpcndr.h>
#endif /* __RPCNDR_H_VERSION__ */

#ifndef COM_NO_WINDOWS_H
#include "windows.h"
#include "ole2.h"
#endif /*COM_NO_WINDOWS_H*/

#ifndef __mimeinfo_h__
#define __mimeinfo_h__

#if defined(_MSC_VER) && (_MSC_VER >= 1020)
#pragma once
#endif

#ifndef DECLSPEC_XFGVIRT
#if defined(_CONTROL_FLOW_GUARD_XFG)
#define DECLSPEC_XFGVIRT(base, func) __declspec(xfg_virtual(base, func))
#else
#define DECLSPEC_XFGVIRT(base, func)
#endif
#endif

/* Forward Declarations */ 

#ifndef __IMimeInfo_FWD_DEFINED__
#define __IMimeInfo_FWD_DEFINED__
typedef interface IMimeInfo IMimeInfo;

#endif 	/* __IMimeInfo_FWD_DEFINED__ */


/* header files for imported files */
#include "objidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_mimeinfo_0000_0000 */
/* [local] */ 

//=--------------------------------------------------------------------------=
// MimeInfo.h
//=--------------------------------------------------------------------------=
// (C) Copyright 1995-1998 Microsoft Corporation.  All Rights Reserved.
//
// THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF
// ANY KIND, EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// THE IMPLIED WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A
// PARTICULAR PURPOSE.
//=--------------------------------------------------------------------------=

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#pragma comment(lib,"uuid.lib")

//--------------------------------------------------------------------------
// IMimeInfo Interfaces.



extern RPC_IF_HANDLE __MIDL_itf_mimeinfo_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mimeinfo_0000_0000_v0_0_s_ifspec;

#ifndef __IMimeInfo_INTERFACE_DEFINED__
#define __IMimeInfo_INTERFACE_DEFINED__

/* interface IMimeInfo */
/* [unique][uuid][object][local] */ 

typedef /* [unique] */ IMimeInfo *LPMIMEINFO;


EXTERN_C const IID IID_IMimeInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F77459A0-BF9A-11cf-BA4E-00C04FD70816")
    IMimeInfo : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetMimeCLSIDMapping( 
            /* [out] */ UINT *pcTypes,
            /* [out] */ LPCSTR **ppszTypes,
            /* [out] */ CLSID **ppclsID) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMimeInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMimeInfo * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMimeInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMimeInfo * This);
        
        DECLSPEC_XFGVIRT(IMimeInfo, GetMimeCLSIDMapping)
        HRESULT ( STDMETHODCALLTYPE *GetMimeCLSIDMapping )( 
            IMimeInfo * This,
            /* [out] */ UINT *pcTypes,
            /* [out] */ LPCSTR **ppszTypes,
            /* [out] */ CLSID **ppclsID);
        
        END_INTERFACE
    } IMimeInfoVtbl;

    interface IMimeInfo
    {
        CONST_VTBL struct IMimeInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMimeInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMimeInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMimeInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMimeInfo_GetMimeCLSIDMapping(This,pcTypes,ppszTypes,ppclsID)	\
    ( (This)->lpVtbl -> GetMimeCLSIDMapping(This,pcTypes,ppszTypes,ppclsID) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMimeInfo_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mimeinfo_0000_0001 */
/* [local] */ 

#define SID_IMimeInfo IID_IMimeInfo
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_mimeinfo_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mimeinfo_0000_0001_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


