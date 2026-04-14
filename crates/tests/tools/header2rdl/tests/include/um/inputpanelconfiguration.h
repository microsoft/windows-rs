

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

#ifndef __inputpanelconfiguration_h__
#define __inputpanelconfiguration_h__

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

#ifndef __IInputPanelConfiguration_FWD_DEFINED__
#define __IInputPanelConfiguration_FWD_DEFINED__
typedef interface IInputPanelConfiguration IInputPanelConfiguration;

#endif 	/* __IInputPanelConfiguration_FWD_DEFINED__ */


#ifndef __IInputPanelInvocationConfiguration_FWD_DEFINED__
#define __IInputPanelInvocationConfiguration_FWD_DEFINED__
typedef interface IInputPanelInvocationConfiguration IInputPanelInvocationConfiguration;

#endif 	/* __IInputPanelInvocationConfiguration_FWD_DEFINED__ */


#ifndef __InputPanelConfiguration_FWD_DEFINED__
#define __InputPanelConfiguration_FWD_DEFINED__

#ifdef __cplusplus
typedef class InputPanelConfiguration InputPanelConfiguration;
#else
typedef struct InputPanelConfiguration InputPanelConfiguration;
#endif /* __cplusplus */

#endif 	/* __InputPanelConfiguration_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_inputpanelconfiguration_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#if (NTDDI_VERSION >= NTDDI_WIN8)


extern RPC_IF_HANDLE __MIDL_itf_inputpanelconfiguration_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_inputpanelconfiguration_0000_0000_v0_0_s_ifspec;

#ifndef __IInputPanelConfiguration_INTERFACE_DEFINED__
#define __IInputPanelConfiguration_INTERFACE_DEFINED__

/* interface IInputPanelConfiguration */
/* [object][helpstring][unique][uuid] */ 


EXTERN_C const IID IID_IInputPanelConfiguration;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("41C81592-514C-48BD-A22E-E6AF638521A6")
    IInputPanelConfiguration : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE EnableFocusTracking( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInputPanelConfigurationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IInputPanelConfiguration * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IInputPanelConfiguration * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IInputPanelConfiguration * This);
        
        DECLSPEC_XFGVIRT(IInputPanelConfiguration, EnableFocusTracking)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *EnableFocusTracking )( 
            __RPC__in IInputPanelConfiguration * This);
        
        END_INTERFACE
    } IInputPanelConfigurationVtbl;

    interface IInputPanelConfiguration
    {
        CONST_VTBL struct IInputPanelConfigurationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInputPanelConfiguration_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInputPanelConfiguration_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInputPanelConfiguration_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInputPanelConfiguration_EnableFocusTracking(This)	\
    ( (This)->lpVtbl -> EnableFocusTracking(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInputPanelConfiguration_INTERFACE_DEFINED__ */


#ifndef __IInputPanelInvocationConfiguration_INTERFACE_DEFINED__
#define __IInputPanelInvocationConfiguration_INTERFACE_DEFINED__

/* interface IInputPanelInvocationConfiguration */
/* [object][helpstring][unique][uuid] */ 


EXTERN_C const IID IID_IInputPanelInvocationConfiguration;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A213F136-3B45-4362-A332-EFB6547CD432")
    IInputPanelInvocationConfiguration : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE RequireTouchInEditControl( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInputPanelInvocationConfigurationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IInputPanelInvocationConfiguration * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IInputPanelInvocationConfiguration * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IInputPanelInvocationConfiguration * This);
        
        DECLSPEC_XFGVIRT(IInputPanelInvocationConfiguration, RequireTouchInEditControl)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *RequireTouchInEditControl )( 
            __RPC__in IInputPanelInvocationConfiguration * This);
        
        END_INTERFACE
    } IInputPanelInvocationConfigurationVtbl;

    interface IInputPanelInvocationConfiguration
    {
        CONST_VTBL struct IInputPanelInvocationConfigurationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInputPanelInvocationConfiguration_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInputPanelInvocationConfiguration_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInputPanelInvocationConfiguration_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInputPanelInvocationConfiguration_RequireTouchInEditControl(This)	\
    ( (This)->lpVtbl -> RequireTouchInEditControl(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInputPanelInvocationConfiguration_INTERFACE_DEFINED__ */



#ifndef __InputPanelConfigurationLib_LIBRARY_DEFINED__
#define __InputPanelConfigurationLib_LIBRARY_DEFINED__

/* library InputPanelConfigurationLib */
/* [helpstring][version][uuid] */ 


EXTERN_C const IID LIBID_InputPanelConfigurationLib;

EXTERN_C const CLSID CLSID_InputPanelConfiguration;

#ifdef __cplusplus

class DECLSPEC_UUID("2853ADD3-F096-4C63-A78F-7FA3EA837FB7")
InputPanelConfiguration;
#endif
#endif /* __InputPanelConfigurationLib_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_inputpanelconfiguration_0000_0003 */
/* [local] */ 

#endif // NTDDI_VERSION >= NTDDI_WIN8
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_inputpanelconfiguration_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_inputpanelconfiguration_0000_0003_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


