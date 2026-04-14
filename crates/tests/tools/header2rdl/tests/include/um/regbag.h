

/* this ALWAYS GENERATED file contains the definitions for the interfaces */


 /* File created by MIDL compiler version 8.01.0628 */
/* @@MIDL_FILE_HEADING(  ) */



/* verify that the <rpcndr.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCNDR_H_VERSION__
#define __REQUIRED_RPCNDR_H_VERSION__ 501
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

#ifndef __regbag_h__
#define __regbag_h__

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

#ifndef __ICreatePropBagOnRegKey_FWD_DEFINED__
#define __ICreatePropBagOnRegKey_FWD_DEFINED__
typedef interface ICreatePropBagOnRegKey ICreatePropBagOnRegKey;

#endif 	/* __ICreatePropBagOnRegKey_FWD_DEFINED__ */


/* header files for imported files */
#include "objidl.h"
#include "oaidl.h"
#include "ocidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_regbag_0000_0000 */
/* [local] */ 

//+-------------------------------------------------------------------------
//
//  Microsoft Windows
//  Copyright (C) Microsoft Corporation, 1999-2000.
//
//--------------------------------------------------------------------------
#pragma once
#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_regbag_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_regbag_0000_0000_v0_0_s_ifspec;

#ifndef __ICreatePropBagOnRegKey_INTERFACE_DEFINED__
#define __ICreatePropBagOnRegKey_INTERFACE_DEFINED__

/* interface ICreatePropBagOnRegKey */
/* [local][unique][helpstring][uuid][restricted][hidden][object] */ 


EXTERN_C const IID IID_ICreatePropBagOnRegKey;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8A674B48-1F63-11d3-B64C-00C04F79498E")
    ICreatePropBagOnRegKey : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Create( 
            /* [in] */ HKEY hkey,
            /* [in] */ LPCOLESTR subkey,
            /* [in] */ DWORD ulOptions,
            /* [in] */ DWORD samDesired,
            REFIID iid,
            /* [out] */ LPVOID *ppBag) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICreatePropBagOnRegKeyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ICreatePropBagOnRegKey * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ICreatePropBagOnRegKey * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ICreatePropBagOnRegKey * This);
        
        DECLSPEC_XFGVIRT(ICreatePropBagOnRegKey, Create)
        HRESULT ( STDMETHODCALLTYPE *Create )( 
            ICreatePropBagOnRegKey * This,
            /* [in] */ HKEY hkey,
            /* [in] */ LPCOLESTR subkey,
            /* [in] */ DWORD ulOptions,
            /* [in] */ DWORD samDesired,
            REFIID iid,
            /* [out] */ LPVOID *ppBag);
        
        END_INTERFACE
    } ICreatePropBagOnRegKeyVtbl;

    interface ICreatePropBagOnRegKey
    {
        CONST_VTBL struct ICreatePropBagOnRegKeyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICreatePropBagOnRegKey_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICreatePropBagOnRegKey_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICreatePropBagOnRegKey_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICreatePropBagOnRegKey_Create(This,hkey,subkey,ulOptions,samDesired,iid,ppBag)	\
    ( (This)->lpVtbl -> Create(This,hkey,subkey,ulOptions,samDesired,iid,ppBag) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICreatePropBagOnRegKey_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_regbag_0000_0001 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_regbag_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_regbag_0000_0001_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


