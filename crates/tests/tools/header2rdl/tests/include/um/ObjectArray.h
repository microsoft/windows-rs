

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

#ifndef __objectarray_h__
#define __objectarray_h__

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

#ifndef __IObjectArray_FWD_DEFINED__
#define __IObjectArray_FWD_DEFINED__
typedef interface IObjectArray IObjectArray;

#endif 	/* __IObjectArray_FWD_DEFINED__ */


#ifndef __IObjectCollection_FWD_DEFINED__
#define __IObjectCollection_FWD_DEFINED__
typedef interface IObjectCollection IObjectCollection;

#endif 	/* __IObjectCollection_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_objectarray_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_objectarray_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_objectarray_0000_0000_v0_0_s_ifspec;

#ifndef __IObjectArray_INTERFACE_DEFINED__
#define __IObjectArray_INTERFACE_DEFINED__

/* interface IObjectArray */
/* [unique][object][uuid][helpstring] */ 


EXTERN_C const IID IID_IObjectArray;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("92CA9DCD-5622-4bba-A805-5E9F541BD8C9")
    IObjectArray : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetCount( 
            /* [out] */ __RPC__out UINT *pcObjects) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAt( 
            /* [in] */ UINT uiIndex,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IObjectArrayVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IObjectArray * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IObjectArray * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IObjectArray * This);
        
        DECLSPEC_XFGVIRT(IObjectArray, GetCount)
        HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            __RPC__in IObjectArray * This,
            /* [out] */ __RPC__out UINT *pcObjects);
        
        DECLSPEC_XFGVIRT(IObjectArray, GetAt)
        HRESULT ( STDMETHODCALLTYPE *GetAt )( 
            __RPC__in IObjectArray * This,
            /* [in] */ UINT uiIndex,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv);
        
        END_INTERFACE
    } IObjectArrayVtbl;

    interface IObjectArray
    {
        CONST_VTBL struct IObjectArrayVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IObjectArray_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IObjectArray_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IObjectArray_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IObjectArray_GetCount(This,pcObjects)	\
    ( (This)->lpVtbl -> GetCount(This,pcObjects) ) 

#define IObjectArray_GetAt(This,uiIndex,riid,ppv)	\
    ( (This)->lpVtbl -> GetAt(This,uiIndex,riid,ppv) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IObjectArray_INTERFACE_DEFINED__ */


#ifndef __IObjectCollection_INTERFACE_DEFINED__
#define __IObjectCollection_INTERFACE_DEFINED__

/* interface IObjectCollection */
/* [unique][object][uuid] */ 


EXTERN_C const IID IID_IObjectCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5632b1a4-e38a-400a-928a-d4cd63230295")
    IObjectCollection : public IObjectArray
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AddObject( 
            /* [in] */ __RPC__in_opt IUnknown *punk) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddFromArray( 
            /* [in] */ __RPC__in_opt IObjectArray *poaSource) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveObjectAt( 
            /* [in] */ UINT uiIndex) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clear( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IObjectCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IObjectCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IObjectCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IObjectCollection * This);
        
        DECLSPEC_XFGVIRT(IObjectArray, GetCount)
        HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            __RPC__in IObjectCollection * This,
            /* [out] */ __RPC__out UINT *pcObjects);
        
        DECLSPEC_XFGVIRT(IObjectArray, GetAt)
        HRESULT ( STDMETHODCALLTYPE *GetAt )( 
            __RPC__in IObjectCollection * This,
            /* [in] */ UINT uiIndex,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv);
        
        DECLSPEC_XFGVIRT(IObjectCollection, AddObject)
        HRESULT ( STDMETHODCALLTYPE *AddObject )( 
            __RPC__in IObjectCollection * This,
            /* [in] */ __RPC__in_opt IUnknown *punk);
        
        DECLSPEC_XFGVIRT(IObjectCollection, AddFromArray)
        HRESULT ( STDMETHODCALLTYPE *AddFromArray )( 
            __RPC__in IObjectCollection * This,
            /* [in] */ __RPC__in_opt IObjectArray *poaSource);
        
        DECLSPEC_XFGVIRT(IObjectCollection, RemoveObjectAt)
        HRESULT ( STDMETHODCALLTYPE *RemoveObjectAt )( 
            __RPC__in IObjectCollection * This,
            /* [in] */ UINT uiIndex);
        
        DECLSPEC_XFGVIRT(IObjectCollection, Clear)
        HRESULT ( STDMETHODCALLTYPE *Clear )( 
            __RPC__in IObjectCollection * This);
        
        END_INTERFACE
    } IObjectCollectionVtbl;

    interface IObjectCollection
    {
        CONST_VTBL struct IObjectCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IObjectCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IObjectCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IObjectCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IObjectCollection_GetCount(This,pcObjects)	\
    ( (This)->lpVtbl -> GetCount(This,pcObjects) ) 

#define IObjectCollection_GetAt(This,uiIndex,riid,ppv)	\
    ( (This)->lpVtbl -> GetAt(This,uiIndex,riid,ppv) ) 


#define IObjectCollection_AddObject(This,punk)	\
    ( (This)->lpVtbl -> AddObject(This,punk) ) 

#define IObjectCollection_AddFromArray(This,poaSource)	\
    ( (This)->lpVtbl -> AddFromArray(This,poaSource) ) 

#define IObjectCollection_RemoveObjectAt(This,uiIndex)	\
    ( (This)->lpVtbl -> RemoveObjectAt(This,uiIndex) ) 

#define IObjectCollection_Clear(This)	\
    ( (This)->lpVtbl -> Clear(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IObjectCollection_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_objectarray_0000_0002 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_objectarray_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_objectarray_0000_0002_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


