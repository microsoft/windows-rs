

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

#ifndef __deletebrowsinghistory_h__
#define __deletebrowsinghistory_h__

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

#ifndef __IDeleteBrowsingHistory_FWD_DEFINED__
#define __IDeleteBrowsingHistory_FWD_DEFINED__
typedef interface IDeleteBrowsingHistory IDeleteBrowsingHistory;

#endif 	/* __IDeleteBrowsingHistory_FWD_DEFINED__ */


/* header files for imported files */
#include "ocidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_deletebrowsinghistory_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
// {31caf6e4-d6aa-4090-a050-a5ac8972e9ef} 
DEFINE_GUID( CATID_DeleteBrowsingHistory, 0x31caf6e4,0xd6aa,0x4090,0xa0,0x50,0xa5,0xac,0x89,0x72,0xe9,0xef);
EXTERN_C const GUID CATID_DeleteBrowsingHistory;
#define DELETE_BROWSING_HISTORY_HISTORY              0x0001   // Indicates that the History checkbox was selected.
#define DELETE_BROWSING_HISTORY_COOKIES              0x0002   // Indicates that the Cookies checkbox was selected.
#define DELETE_BROWSING_HISTORY_TIF                  0x0004   // Indicates that the Temporary Internet Files checkbox was selected.
#define DELETE_BROWSING_HISTORY_FORMDATA             0x0008   // Indicates that the Form data checkbox was selected.
#define DELETE_BROWSING_HISTORY_PASSWORDS            0x0010   // Indicates that the Passwords checkbox was selected.
#define DELETE_BROWSING_HISTORY_PRESERVEFAVORITES    0x0020   // Indicates that the Preseve Favorite website data checkbox is selected.
#define DELETE_BROWSING_HISTORY_DOWNLOADHISTORY      0x0040   // Indicates that the Download History checkbox was selected.


extern RPC_IF_HANDLE __MIDL_itf_deletebrowsinghistory_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_deletebrowsinghistory_0000_0000_v0_0_s_ifspec;

#ifndef __IDeleteBrowsingHistory_INTERFACE_DEFINED__
#define __IDeleteBrowsingHistory_INTERFACE_DEFINED__

/* interface IDeleteBrowsingHistory */
/* [object][helpstring][uuid] */ 


EXTERN_C const IID IID_IDeleteBrowsingHistory;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("cf38ed4b-2be7-4461-8b5e-9a466dc82ae3")
    IDeleteBrowsingHistory : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE DeleteBrowsingHistory( 
            DWORD dwFlags) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDeleteBrowsingHistoryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDeleteBrowsingHistory * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDeleteBrowsingHistory * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDeleteBrowsingHistory * This);
        
        DECLSPEC_XFGVIRT(IDeleteBrowsingHistory, DeleteBrowsingHistory)
        HRESULT ( STDMETHODCALLTYPE *DeleteBrowsingHistory )( 
            __RPC__in IDeleteBrowsingHistory * This,
            DWORD dwFlags);
        
        END_INTERFACE
    } IDeleteBrowsingHistoryVtbl;

    interface IDeleteBrowsingHistory
    {
        CONST_VTBL struct IDeleteBrowsingHistoryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDeleteBrowsingHistory_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDeleteBrowsingHistory_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDeleteBrowsingHistory_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDeleteBrowsingHistory_DeleteBrowsingHistory(This,dwFlags)	\
    ( (This)->lpVtbl -> DeleteBrowsingHistory(This,dwFlags) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDeleteBrowsingHistory_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_deletebrowsinghistory_0000_0001 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_deletebrowsinghistory_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_deletebrowsinghistory_0000_0001_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


