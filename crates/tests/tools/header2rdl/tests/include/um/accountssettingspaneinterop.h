

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

#ifndef __accountssettingspaneinterop_h__
#define __accountssettingspaneinterop_h__

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

#ifndef __IAccountsSettingsPaneInterop_FWD_DEFINED__
#define __IAccountsSettingsPaneInterop_FWD_DEFINED__
typedef interface IAccountsSettingsPaneInterop IAccountsSettingsPaneInterop;

#endif 	/* __IAccountsSettingsPaneInterop_FWD_DEFINED__ */


/* header files for imported files */
#include "inspectable.h"
#include "asyncinfo.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_accountssettingspaneinterop_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#if (NTDDI_VERSION >= NTDDI_WIN10_RS1)
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_accountssettingspaneinterop_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_accountssettingspaneinterop_0000_0000_v0_0_s_ifspec;

#ifndef __IAccountsSettingsPaneInterop_INTERFACE_DEFINED__
#define __IAccountsSettingsPaneInterop_INTERFACE_DEFINED__

/* interface IAccountsSettingsPaneInterop */
/* [object][uuid] */ 


EXTERN_C const IID IID_IAccountsSettingsPaneInterop;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D3EE12AD-3865-4362-9746-B75A682DF0E6")
    IAccountsSettingsPaneInterop : public IInspectable
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetForWindow( 
            /* [in] */ __RPC__in HWND appWindow,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][retval][out] */ __RPC__deref_out_opt void **accountsSettingsPane) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ShowManageAccountsForWindowAsync( 
            /* [in] */ __RPC__in HWND appWindow,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][retval][out] */ __RPC__deref_out_opt void **asyncAction) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ShowAddAccountForWindowAsync( 
            /* [in] */ __RPC__in HWND appWindow,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][retval][out] */ __RPC__deref_out_opt void **asyncAction) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAccountsSettingsPaneInteropVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAccountsSettingsPaneInterop * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAccountsSettingsPaneInterop * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAccountsSettingsPaneInterop * This);
        
        DECLSPEC_XFGVIRT(IInspectable, GetIids)
        HRESULT ( STDMETHODCALLTYPE *GetIids )( 
            __RPC__in IAccountsSettingsPaneInterop * This,
            /* [out] */ __RPC__out ULONG *iidCount,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids);
        
        DECLSPEC_XFGVIRT(IInspectable, GetRuntimeClassName)
        HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )( 
            __RPC__in IAccountsSettingsPaneInterop * This,
            /* [out] */ __RPC__deref_out_opt HSTRING *className);
        
        DECLSPEC_XFGVIRT(IInspectable, GetTrustLevel)
        HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )( 
            __RPC__in IAccountsSettingsPaneInterop * This,
            /* [out] */ __RPC__out TrustLevel *trustLevel);
        
        DECLSPEC_XFGVIRT(IAccountsSettingsPaneInterop, GetForWindow)
        HRESULT ( STDMETHODCALLTYPE *GetForWindow )( 
            __RPC__in IAccountsSettingsPaneInterop * This,
            /* [in] */ __RPC__in HWND appWindow,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][retval][out] */ __RPC__deref_out_opt void **accountsSettingsPane);
        
        DECLSPEC_XFGVIRT(IAccountsSettingsPaneInterop, ShowManageAccountsForWindowAsync)
        HRESULT ( STDMETHODCALLTYPE *ShowManageAccountsForWindowAsync )( 
            __RPC__in IAccountsSettingsPaneInterop * This,
            /* [in] */ __RPC__in HWND appWindow,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][retval][out] */ __RPC__deref_out_opt void **asyncAction);
        
        DECLSPEC_XFGVIRT(IAccountsSettingsPaneInterop, ShowAddAccountForWindowAsync)
        HRESULT ( STDMETHODCALLTYPE *ShowAddAccountForWindowAsync )( 
            __RPC__in IAccountsSettingsPaneInterop * This,
            /* [in] */ __RPC__in HWND appWindow,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][retval][out] */ __RPC__deref_out_opt void **asyncAction);
        
        END_INTERFACE
    } IAccountsSettingsPaneInteropVtbl;

    interface IAccountsSettingsPaneInterop
    {
        CONST_VTBL struct IAccountsSettingsPaneInteropVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAccountsSettingsPaneInterop_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAccountsSettingsPaneInterop_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAccountsSettingsPaneInterop_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAccountsSettingsPaneInterop_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 

#define IAccountsSettingsPaneInterop_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 

#define IAccountsSettingsPaneInterop_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 


#define IAccountsSettingsPaneInterop_GetForWindow(This,appWindow,riid,accountsSettingsPane)	\
    ( (This)->lpVtbl -> GetForWindow(This,appWindow,riid,accountsSettingsPane) ) 

#define IAccountsSettingsPaneInterop_ShowManageAccountsForWindowAsync(This,appWindow,riid,asyncAction)	\
    ( (This)->lpVtbl -> ShowManageAccountsForWindowAsync(This,appWindow,riid,asyncAction) ) 

#define IAccountsSettingsPaneInterop_ShowAddAccountForWindowAsync(This,appWindow,riid,asyncAction)	\
    ( (This)->lpVtbl -> ShowAddAccountForWindowAsync(This,appWindow,riid,asyncAction) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAccountsSettingsPaneInterop_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_accountssettingspaneinterop_0000_0001 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#endif //(NTDDI_VERSION >= NTDDI_WIN10)


extern RPC_IF_HANDLE __MIDL_itf_accountssettingspaneinterop_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_accountssettingspaneinterop_0000_0001_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  HWND_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HWND * ); 
void                      __RPC_USER  HWND_UserFree(     __RPC__in unsigned long *, __RPC__in HWND * ); 

unsigned long             __RPC_USER  HWND_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HWND * ); 
void                      __RPC_USER  HWND_UserFree64(     __RPC__in unsigned long *, __RPC__in HWND * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


