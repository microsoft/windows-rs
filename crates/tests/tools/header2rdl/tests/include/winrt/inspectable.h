

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

#ifndef __inspectable_h__
#define __inspectable_h__

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

#ifndef __IInspectable_FWD_DEFINED__
#define __IInspectable_FWD_DEFINED__
typedef interface IInspectable IInspectable;

#endif 	/* __IInspectable_FWD_DEFINED__ */


/* header files for imported files */
#include "wtypes.h"
#include "unknwn.h"
#include "hstring.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_inspectable_0000_0000 */
/* [local] */ 

//+-------------------------------------------------------------------------
//
//  Microsoft Windows
//  Copyright (c) Microsoft Corporation. All rights reserved.
//
//--------------------------------------------------------------------------
#if ( _MSC_VER >= 1020 )
#pragma once
#endif
typedef /* [unique] */  __RPC_unique_pointer IInspectable *LPINSPECTABLE;

typedef /* [v1_enum] */ 
enum TrustLevel
    {
        BaseTrust	= 0,
        PartialTrust	= ( BaseTrust + 1 ) ,
        FullTrust	= ( PartialTrust + 1 ) 
    } 	TrustLevel;



extern RPC_IF_HANDLE __MIDL_itf_inspectable_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_inspectable_0000_0000_v0_0_s_ifspec;

#ifndef __IInspectable_INTERFACE_DEFINED__
#define __IInspectable_INTERFACE_DEFINED__

/* interface IInspectable */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IInspectable;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("AF86E2E0-B12D-4c6a-9C5A-D7AA65101E90")
    IInspectable : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetIids( 
            /* [out] */ __RPC__out ULONG *iidCount,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRuntimeClassName( 
            /* [out] */ __RPC__deref_out_opt HSTRING *className) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTrustLevel( 
            /* [out] */ __RPC__out TrustLevel *trustLevel) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInspectableVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IInspectable * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IInspectable * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IInspectable * This);
        
        DECLSPEC_XFGVIRT(IInspectable, GetIids)
        HRESULT ( STDMETHODCALLTYPE *GetIids )( 
            __RPC__in IInspectable * This,
            /* [out] */ __RPC__out ULONG *iidCount,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids);
        
        DECLSPEC_XFGVIRT(IInspectable, GetRuntimeClassName)
        HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )( 
            __RPC__in IInspectable * This,
            /* [out] */ __RPC__deref_out_opt HSTRING *className);
        
        DECLSPEC_XFGVIRT(IInspectable, GetTrustLevel)
        HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )( 
            __RPC__in IInspectable * This,
            /* [out] */ __RPC__out TrustLevel *trustLevel);
        
        END_INTERFACE
    } IInspectableVtbl;

    interface IInspectable
    {
        CONST_VTBL struct IInspectableVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInspectable_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInspectable_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInspectable_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInspectable_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 

#define IInspectable_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 

#define IInspectable_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInspectable_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_inspectable_0000_0001 */
/* [local] */ 

#if defined(__cplusplus) && !defined(CINTERFACE)
//  IInspectable equivalent of IID_PPV_ARGS
//  IID_INS_ARGS(ppType)
//      ppType is the variable of type IType that will be filled
//
//      RESULTS in:  IID_IType, ppvType
//      will create a compiler error if wrong level of indirection is used.
//
extern "C++"
{
    template<typename T> _Post_equal_to_(pp) _Post_satisfies_(return == pp) void** IID_INS_ARGS_Helper(T** pp) 
    {
        (void)static_cast<IInspectable*>(*pp);    // make sure everyone derives from IInspectable
        return reinterpret_cast<void**>(pp);
    }
}
#define IID_INS_ARGS(ppType) __uuidof(**(ppType)), IID_INS_ARGS_Helper(ppType)
#endif


extern RPC_IF_HANDLE __MIDL_itf_inspectable_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_inspectable_0000_0001_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  HSTRING_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in HSTRING * ); 
unsigned char * __RPC_USER  HSTRING_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HSTRING * ); 
unsigned char * __RPC_USER  HSTRING_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HSTRING * ); 
void                      __RPC_USER  HSTRING_UserFree(     __RPC__in unsigned long *, __RPC__in HSTRING * ); 

unsigned long             __RPC_USER  HSTRING_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in HSTRING * ); 
unsigned char * __RPC_USER  HSTRING_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HSTRING * ); 
unsigned char * __RPC_USER  HSTRING_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HSTRING * ); 
void                      __RPC_USER  HSTRING_UserFree64(     __RPC__in unsigned long *, __RPC__in HSTRING * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


