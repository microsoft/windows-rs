

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

#ifndef __hxhelppaneproxy_h__
#define __hxhelppaneproxy_h__

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

#ifndef __IHxHelpPane_FWD_DEFINED__
#define __IHxHelpPane_FWD_DEFINED__
typedef interface IHxHelpPane IHxHelpPane;

#endif 	/* __IHxHelpPane_FWD_DEFINED__ */


#ifndef __IHxInteractiveUser_FWD_DEFINED__
#define __IHxInteractiveUser_FWD_DEFINED__
typedef interface IHxInteractiveUser IHxInteractiveUser;

#endif 	/* __IHxInteractiveUser_FWD_DEFINED__ */


#ifndef __HxHelpPane_FWD_DEFINED__
#define __HxHelpPane_FWD_DEFINED__

#ifdef __cplusplus
typedef class HxHelpPane HxHelpPane;
#else
typedef struct HxHelpPane HxHelpPane;
#endif /* __cplusplus */

#endif 	/* __HxHelpPane_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_hxhelppaneproxy_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)




extern RPC_IF_HANDLE __MIDL_itf_hxhelppaneproxy_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_hxhelppaneproxy_0000_0000_v0_0_s_ifspec;

#ifndef __IHxHelpPane_INTERFACE_DEFINED__
#define __IHxHelpPane_INTERFACE_DEFINED__

/* interface IHxHelpPane */
/* [hidden][oleautomation][helpstring][uuid][unique][object] */ 


EXTERN_C const IID IID_IHxHelpPane;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8cec5884-07a1-11d9-b15e-000d56bfe6ee")
    IHxHelpPane : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE DisplayTask( 
            /* [in] */ __RPC__in BSTR bstrUrl) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DisplayContents( 
            /* [annotation][in] */ 
            _In_opt_  BSTR bstrUrl) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DisplaySearchResults( 
            /* [in] */ __RPC__in BSTR bstrSearchQuery) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IHxHelpPaneVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IHxHelpPane * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IHxHelpPane * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IHxHelpPane * This);
        
        DECLSPEC_XFGVIRT(IHxHelpPane, DisplayTask)
        HRESULT ( STDMETHODCALLTYPE *DisplayTask )( 
            __RPC__in IHxHelpPane * This,
            /* [in] */ __RPC__in BSTR bstrUrl);
        
        DECLSPEC_XFGVIRT(IHxHelpPane, DisplayContents)
        HRESULT ( STDMETHODCALLTYPE *DisplayContents )( 
            __RPC__in IHxHelpPane * This,
            /* [annotation][in] */ 
            _In_opt_  BSTR bstrUrl);
        
        DECLSPEC_XFGVIRT(IHxHelpPane, DisplaySearchResults)
        HRESULT ( STDMETHODCALLTYPE *DisplaySearchResults )( 
            __RPC__in IHxHelpPane * This,
            /* [in] */ __RPC__in BSTR bstrSearchQuery);
        
        END_INTERFACE
    } IHxHelpPaneVtbl;

    interface IHxHelpPane
    {
        CONST_VTBL struct IHxHelpPaneVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IHxHelpPane_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IHxHelpPane_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IHxHelpPane_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IHxHelpPane_DisplayTask(This,bstrUrl)	\
    ( (This)->lpVtbl -> DisplayTask(This,bstrUrl) ) 

#define IHxHelpPane_DisplayContents(This,bstrUrl)	\
    ( (This)->lpVtbl -> DisplayContents(This,bstrUrl) ) 

#define IHxHelpPane_DisplaySearchResults(This,bstrSearchQuery)	\
    ( (This)->lpVtbl -> DisplaySearchResults(This,bstrSearchQuery) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IHxHelpPane_INTERFACE_DEFINED__ */


#ifndef __IHxInteractiveUser_INTERFACE_DEFINED__
#define __IHxInteractiveUser_INTERFACE_DEFINED__

/* interface IHxInteractiveUser */
/* [hidden][oleautomation][helpstring][uuid][unique][object] */ 


EXTERN_C const IID IID_IHxInteractiveUser;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8cec595b-07a1-11d9-b15e-000d56bfe6ee")
    IHxInteractiveUser : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Execute( 
            /* [in] */ __RPC__in LPCWSTR pcUrl) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IHxInteractiveUserVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IHxInteractiveUser * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IHxInteractiveUser * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IHxInteractiveUser * This);
        
        DECLSPEC_XFGVIRT(IHxInteractiveUser, Execute)
        HRESULT ( STDMETHODCALLTYPE *Execute )( 
            __RPC__in IHxInteractiveUser * This,
            /* [in] */ __RPC__in LPCWSTR pcUrl);
        
        END_INTERFACE
    } IHxInteractiveUserVtbl;

    interface IHxInteractiveUser
    {
        CONST_VTBL struct IHxInteractiveUserVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IHxInteractiveUser_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IHxInteractiveUser_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IHxInteractiveUser_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IHxInteractiveUser_Execute(This,pcUrl)	\
    ( (This)->lpVtbl -> Execute(This,pcUrl) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IHxInteractiveUser_INTERFACE_DEFINED__ */



#ifndef __HelpPane_LIBRARY_DEFINED__
#define __HelpPane_LIBRARY_DEFINED__

/* library HelpPane */
/* [helpstring][version][uuid] */ 


EXTERN_C const IID LIBID_HelpPane;

EXTERN_C const CLSID CLSID_HxHelpPane;

#ifdef __cplusplus

class DECLSPEC_UUID("8cec58e7-07a1-11d9-b15e-000d56bfe6ee")
HxHelpPane;
#endif
#endif /* __HelpPane_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_hxhelppaneproxy_0000_0003 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_hxhelppaneproxy_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_hxhelppaneproxy_0000_0003_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  BSTR_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  BSTR_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree64(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


