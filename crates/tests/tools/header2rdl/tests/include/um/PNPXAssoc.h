

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

#ifndef __pnpxassoc_h__
#define __pnpxassoc_h__

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

#ifndef __IPNPXAssociation_FWD_DEFINED__
#define __IPNPXAssociation_FWD_DEFINED__
typedef interface IPNPXAssociation IPNPXAssociation;

#endif 	/* __IPNPXAssociation_FWD_DEFINED__ */


#ifndef __IPNPXDeviceAssociation_FWD_DEFINED__
#define __IPNPXDeviceAssociation_FWD_DEFINED__
typedef interface IPNPXDeviceAssociation IPNPXDeviceAssociation;

#endif 	/* __IPNPXDeviceAssociation_FWD_DEFINED__ */


#ifndef __PNPXAssociation_FWD_DEFINED__
#define __PNPXAssociation_FWD_DEFINED__

#ifdef __cplusplus
typedef class PNPXAssociation PNPXAssociation;
#else
typedef struct PNPXAssociation PNPXAssociation;
#endif /* __cplusplus */

#endif 	/* __PNPXAssociation_FWD_DEFINED__ */


#ifndef __PNPXPairingHandler_FWD_DEFINED__
#define __PNPXPairingHandler_FWD_DEFINED__

#ifdef __cplusplus
typedef class PNPXPairingHandler PNPXPairingHandler;
#else
typedef struct PNPXPairingHandler PNPXPairingHandler;
#endif /* __cplusplus */

#endif 	/* __PNPXPairingHandler_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"
#include "FunctionDiscoveryProvider.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_pnpxassoc_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_pnpxassoc_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_pnpxassoc_0000_0000_v0_0_s_ifspec;

#ifndef __IPNPXAssociation_INTERFACE_DEFINED__
#define __IPNPXAssociation_INTERFACE_DEFINED__

/* interface IPNPXAssociation */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IPNPXAssociation;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0bd7e521-4da6-42d5-81ba-1981b6b94075")
    IPNPXAssociation : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Associate( 
            /* [unique][string][in] */ __RPC__in_opt_string const WCHAR *pszSubcategory) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Unassociate( 
            /* [unique][string][in] */ __RPC__in_opt_string const WCHAR *pszSubcategory) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Delete( 
            /* [unique][string][in] */ __RPC__in_opt_string const WCHAR *pszSubcategory) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPNPXAssociationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPNPXAssociation * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPNPXAssociation * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPNPXAssociation * This);
        
        DECLSPEC_XFGVIRT(IPNPXAssociation, Associate)
        HRESULT ( STDMETHODCALLTYPE *Associate )( 
            __RPC__in IPNPXAssociation * This,
            /* [unique][string][in] */ __RPC__in_opt_string const WCHAR *pszSubcategory);
        
        DECLSPEC_XFGVIRT(IPNPXAssociation, Unassociate)
        HRESULT ( STDMETHODCALLTYPE *Unassociate )( 
            __RPC__in IPNPXAssociation * This,
            /* [unique][string][in] */ __RPC__in_opt_string const WCHAR *pszSubcategory);
        
        DECLSPEC_XFGVIRT(IPNPXAssociation, Delete)
        HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IPNPXAssociation * This,
            /* [unique][string][in] */ __RPC__in_opt_string const WCHAR *pszSubcategory);
        
        END_INTERFACE
    } IPNPXAssociationVtbl;

    interface IPNPXAssociation
    {
        CONST_VTBL struct IPNPXAssociationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPNPXAssociation_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPNPXAssociation_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPNPXAssociation_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPNPXAssociation_Associate(This,pszSubcategory)	\
    ( (This)->lpVtbl -> Associate(This,pszSubcategory) ) 

#define IPNPXAssociation_Unassociate(This,pszSubcategory)	\
    ( (This)->lpVtbl -> Unassociate(This,pszSubcategory) ) 

#define IPNPXAssociation_Delete(This,pszSubcategory)	\
    ( (This)->lpVtbl -> Delete(This,pszSubcategory) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPNPXAssociation_INTERFACE_DEFINED__ */


#ifndef __IPNPXDeviceAssociation_INTERFACE_DEFINED__
#define __IPNPXDeviceAssociation_INTERFACE_DEFINED__

/* interface IPNPXDeviceAssociation */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IPNPXDeviceAssociation;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("EED366D0-35B8-4fc5-8D20-7E5BD31F6DED")
    IPNPXDeviceAssociation : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Associate( 
            /* [unique][string][in] */ __RPC__in_opt_string const WCHAR *pszSubCategory,
            /* [in] */ __RPC__in_opt IFunctionDiscoveryNotification *pIFunctionDiscoveryNotification) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Unassociate( 
            /* [unique][string][in] */ __RPC__in_opt_string const WCHAR *pszSubCategory,
            /* [in] */ __RPC__in_opt IFunctionDiscoveryNotification *pIFunctionDiscoveryNotification) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Delete( 
            /* [unique][string][in] */ __RPC__in_opt_string const WCHAR *pszSubcategory,
            /* [in] */ __RPC__in_opt IFunctionDiscoveryNotification *pIFunctionDiscoveryNotification) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPNPXDeviceAssociationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPNPXDeviceAssociation * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPNPXDeviceAssociation * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPNPXDeviceAssociation * This);
        
        DECLSPEC_XFGVIRT(IPNPXDeviceAssociation, Associate)
        HRESULT ( STDMETHODCALLTYPE *Associate )( 
            __RPC__in IPNPXDeviceAssociation * This,
            /* [unique][string][in] */ __RPC__in_opt_string const WCHAR *pszSubCategory,
            /* [in] */ __RPC__in_opt IFunctionDiscoveryNotification *pIFunctionDiscoveryNotification);
        
        DECLSPEC_XFGVIRT(IPNPXDeviceAssociation, Unassociate)
        HRESULT ( STDMETHODCALLTYPE *Unassociate )( 
            __RPC__in IPNPXDeviceAssociation * This,
            /* [unique][string][in] */ __RPC__in_opt_string const WCHAR *pszSubCategory,
            /* [in] */ __RPC__in_opt IFunctionDiscoveryNotification *pIFunctionDiscoveryNotification);
        
        DECLSPEC_XFGVIRT(IPNPXDeviceAssociation, Delete)
        HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IPNPXDeviceAssociation * This,
            /* [unique][string][in] */ __RPC__in_opt_string const WCHAR *pszSubcategory,
            /* [in] */ __RPC__in_opt IFunctionDiscoveryNotification *pIFunctionDiscoveryNotification);
        
        END_INTERFACE
    } IPNPXDeviceAssociationVtbl;

    interface IPNPXDeviceAssociation
    {
        CONST_VTBL struct IPNPXDeviceAssociationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPNPXDeviceAssociation_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPNPXDeviceAssociation_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPNPXDeviceAssociation_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPNPXDeviceAssociation_Associate(This,pszSubCategory,pIFunctionDiscoveryNotification)	\
    ( (This)->lpVtbl -> Associate(This,pszSubCategory,pIFunctionDiscoveryNotification) ) 

#define IPNPXDeviceAssociation_Unassociate(This,pszSubCategory,pIFunctionDiscoveryNotification)	\
    ( (This)->lpVtbl -> Unassociate(This,pszSubCategory,pIFunctionDiscoveryNotification) ) 

#define IPNPXDeviceAssociation_Delete(This,pszSubcategory,pIFunctionDiscoveryNotification)	\
    ( (This)->lpVtbl -> Delete(This,pszSubcategory,pIFunctionDiscoveryNotification) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPNPXDeviceAssociation_INTERFACE_DEFINED__ */



#ifndef __PNPXAssociation_LIBRARY_DEFINED__
#define __PNPXAssociation_LIBRARY_DEFINED__

/* library PNPXAssociation */
/* [helpstring][version][uuid] */ 


EXTERN_C const IID LIBID_PNPXAssociation;

EXTERN_C const CLSID CLSID_PNPXAssociation;

#ifdef __cplusplus

class DECLSPEC_UUID("cee8ccc9-4f6b-4469-a235-5a22869eef03")
PNPXAssociation;
#endif

EXTERN_C const CLSID CLSID_PNPXPairingHandler;

#ifdef __cplusplus

class DECLSPEC_UUID("B8A27942-ADE7-4085-AA6E-4FADC7ADA1EF")
PNPXPairingHandler;
#endif
#endif /* __PNPXAssociation_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_pnpxassoc_0000_0003 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_pnpxassoc_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_pnpxassoc_0000_0003_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


