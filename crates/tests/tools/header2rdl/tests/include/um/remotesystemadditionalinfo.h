

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

#ifndef __remotesystemadditionalinfo_h__
#define __remotesystemadditionalinfo_h__

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

#ifndef __IRemoteSystemAdditionalInfoProvider_FWD_DEFINED__
#define __IRemoteSystemAdditionalInfoProvider_FWD_DEFINED__
typedef interface IRemoteSystemAdditionalInfoProvider IRemoteSystemAdditionalInfoProvider;

#endif 	/* __IRemoteSystemAdditionalInfoProvider_FWD_DEFINED__ */


/* header files for imported files */
#include "unknwn.h"
#include "hstring.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_remotesystemadditionalinfo_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#if (NTDDI_VERSION >= NTDDI_WIN10_RS4)
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)


extern RPC_IF_HANDLE __MIDL_itf_remotesystemadditionalinfo_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_remotesystemadditionalinfo_0000_0000_v0_0_s_ifspec;

#ifndef __IRemoteSystemAdditionalInfoProvider_INTERFACE_DEFINED__
#define __IRemoteSystemAdditionalInfoProvider_INTERFACE_DEFINED__

/* interface IRemoteSystemAdditionalInfoProvider */
/* [uuid][unique][object] */ 


EXTERN_C const IID IID_IRemoteSystemAdditionalInfoProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("EEAA3D5F-EC63-4D27-AF38-E86B1D7292CB")
    IRemoteSystemAdditionalInfoProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetAdditionalInfo( 
            /* [out] */ __RPC__deref_out_opt HSTRING *deduplicationId,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **mapView) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRemoteSystemAdditionalInfoProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRemoteSystemAdditionalInfoProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRemoteSystemAdditionalInfoProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRemoteSystemAdditionalInfoProvider * This);
        
        DECLSPEC_XFGVIRT(IRemoteSystemAdditionalInfoProvider, GetAdditionalInfo)
        HRESULT ( STDMETHODCALLTYPE *GetAdditionalInfo )( 
            __RPC__in IRemoteSystemAdditionalInfoProvider * This,
            /* [out] */ __RPC__deref_out_opt HSTRING *deduplicationId,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **mapView);
        
        END_INTERFACE
    } IRemoteSystemAdditionalInfoProviderVtbl;

    interface IRemoteSystemAdditionalInfoProvider
    {
        CONST_VTBL struct IRemoteSystemAdditionalInfoProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRemoteSystemAdditionalInfoProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRemoteSystemAdditionalInfoProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRemoteSystemAdditionalInfoProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRemoteSystemAdditionalInfoProvider_GetAdditionalInfo(This,deduplicationId,riid,mapView)	\
    ( (This)->lpVtbl -> GetAdditionalInfo(This,deduplicationId,riid,mapView) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRemoteSystemAdditionalInfoProvider_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_remotesystemadditionalinfo_0000_0001 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#endif /* (NTDDI_VERSION >= NTDDI_WIN10_RS4) */


extern RPC_IF_HANDLE __MIDL_itf_remotesystemadditionalinfo_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_remotesystemadditionalinfo_0000_0001_v0_0_s_ifspec;

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


