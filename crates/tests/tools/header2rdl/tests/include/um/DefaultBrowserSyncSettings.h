

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

#ifndef __defaultbrowsersyncsettings_h__
#define __defaultbrowsersyncsettings_h__

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

#ifndef __IDefaultBrowserSyncSettings_FWD_DEFINED__
#define __IDefaultBrowserSyncSettings_FWD_DEFINED__
typedef interface IDefaultBrowserSyncSettings IDefaultBrowserSyncSettings;

#endif 	/* __IDefaultBrowserSyncSettings_FWD_DEFINED__ */


#ifndef __DefaultBrowserSyncSettings_FWD_DEFINED__
#define __DefaultBrowserSyncSettings_FWD_DEFINED__

#ifdef __cplusplus
typedef class DefaultBrowserSyncSettings DefaultBrowserSyncSettings;
#else
typedef struct DefaultBrowserSyncSettings DefaultBrowserSyncSettings;
#endif /* __cplusplus */

#endif 	/* __DefaultBrowserSyncSettings_FWD_DEFINED__ */


/* header files for imported files */
#include "oleidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_defaultbrowsersyncsettings_0000_0000 */
/* [local] */ 

#if (NTDDI_VERSION >= NTDDI_WINBLUE)
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_defaultbrowsersyncsettings_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_defaultbrowsersyncsettings_0000_0000_v0_0_s_ifspec;

#ifndef __IDefaultBrowserSyncSettings_INTERFACE_DEFINED__
#define __IDefaultBrowserSyncSettings_INTERFACE_DEFINED__

/* interface IDefaultBrowserSyncSettings */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IDefaultBrowserSyncSettings;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7A27FAAD-5AE6-4255-9030-C530936292E3")
    IDefaultBrowserSyncSettings : public IUnknown
    {
    public:
        virtual BOOL STDMETHODCALLTYPE IsEnabled( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDefaultBrowserSyncSettingsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDefaultBrowserSyncSettings * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDefaultBrowserSyncSettings * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDefaultBrowserSyncSettings * This);
        
        DECLSPEC_XFGVIRT(IDefaultBrowserSyncSettings, IsEnabled)
        BOOL ( STDMETHODCALLTYPE *IsEnabled )( 
            IDefaultBrowserSyncSettings * This);
        
        END_INTERFACE
    } IDefaultBrowserSyncSettingsVtbl;

    interface IDefaultBrowserSyncSettings
    {
        CONST_VTBL struct IDefaultBrowserSyncSettingsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDefaultBrowserSyncSettings_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDefaultBrowserSyncSettings_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDefaultBrowserSyncSettings_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDefaultBrowserSyncSettings_IsEnabled(This)	\
    ( (This)->lpVtbl -> IsEnabled(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDefaultBrowserSyncSettings_INTERFACE_DEFINED__ */



#ifndef __BrowserSyncSettings_LIBRARY_DEFINED__
#define __BrowserSyncSettings_LIBRARY_DEFINED__

/* library BrowserSyncSettings */
/* [uuid] */ 


EXTERN_C const IID LIBID_BrowserSyncSettings;

EXTERN_C const CLSID CLSID_DefaultBrowserSyncSettings;

#ifdef __cplusplus

class DECLSPEC_UUID("3AC83423-3112-4AA6-9B5B-1FEB23D0C5F9")
DefaultBrowserSyncSettings;
#endif
#endif /* __BrowserSyncSettings_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_defaultbrowsersyncsettings_0000_0002 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#endif // (NTDDI_VERSION >= NTDDI_WINBLUE)


extern RPC_IF_HANDLE __MIDL_itf_defaultbrowsersyncsettings_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_defaultbrowsersyncsettings_0000_0002_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


