

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

#ifndef __dmodshow_h__
#define __dmodshow_h__

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

#ifndef __IDMOWrapperFilter_FWD_DEFINED__
#define __IDMOWrapperFilter_FWD_DEFINED__
typedef interface IDMOWrapperFilter IDMOWrapperFilter;

#endif 	/* __IDMOWrapperFilter_FWD_DEFINED__ */


/* header files for imported files */
#include "unknwn.h"
#include "objidl.h"
#include "mediaobj.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_dmodshow_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
DEFINE_GUID(CLSID_DMOWrapperFilter, 0x94297043,0xbd82,0x4dfd,0xb0,0xde,0x81,0x77,0x73,0x9c,0x6d,0x20);
DEFINE_GUID(CLSID_DMOFilterCategory,0xbcd5796c,0xbd52,0x4d30,0xab,0x76,0x70,0xf9,0x75,0xb8,0x91,0x99);


extern RPC_IF_HANDLE __MIDL_itf_dmodshow_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_dmodshow_0000_0000_v0_0_s_ifspec;

#ifndef __IDMOWrapperFilter_INTERFACE_DEFINED__
#define __IDMOWrapperFilter_INTERFACE_DEFINED__

/* interface IDMOWrapperFilter */
/* [uuid][object] */ 


EXTERN_C const IID IID_IDMOWrapperFilter;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("52d6f586-9f0f-4824-8fc8-e32ca04930c2")
    IDMOWrapperFilter : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Init( 
            __RPC__in REFCLSID clsidDMO,
            __RPC__in REFCLSID catDMO) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDMOWrapperFilterVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDMOWrapperFilter * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDMOWrapperFilter * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDMOWrapperFilter * This);
        
        DECLSPEC_XFGVIRT(IDMOWrapperFilter, Init)
        HRESULT ( STDMETHODCALLTYPE *Init )( 
            __RPC__in IDMOWrapperFilter * This,
            __RPC__in REFCLSID clsidDMO,
            __RPC__in REFCLSID catDMO);
        
        END_INTERFACE
    } IDMOWrapperFilterVtbl;

    interface IDMOWrapperFilter
    {
        CONST_VTBL struct IDMOWrapperFilterVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDMOWrapperFilter_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDMOWrapperFilter_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDMOWrapperFilter_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDMOWrapperFilter_Init(This,clsidDMO,catDMO)	\
    ( (This)->lpVtbl -> Init(This,clsidDMO,catDMO) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDMOWrapperFilter_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_dmodshow_0000_0001 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_dmodshow_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_dmodshow_0000_0001_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


