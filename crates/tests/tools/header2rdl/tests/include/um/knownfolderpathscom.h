

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

#ifndef __knownfolderpathscom_h__
#define __knownfolderpathscom_h__

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

#ifndef __IUserDataPathsInterop_FWD_DEFINED__
#define __IUserDataPathsInterop_FWD_DEFINED__
typedef interface IUserDataPathsInterop IUserDataPathsInterop;

#endif 	/* __IUserDataPathsInterop_FWD_DEFINED__ */


#ifndef __ISystemDataPathsInterop_FWD_DEFINED__
#define __ISystemDataPathsInterop_FWD_DEFINED__
typedef interface ISystemDataPathsInterop ISystemDataPathsInterop;

#endif 	/* __ISystemDataPathsInterop_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_knownfolderpathscom_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_knownfolderpathscom_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_knownfolderpathscom_0000_0000_v0_0_s_ifspec;

#ifndef __IUserDataPathsInterop_INTERFACE_DEFINED__
#define __IUserDataPathsInterop_INTERFACE_DEFINED__

/* interface IUserDataPathsInterop */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IUserDataPathsInterop;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F12EF0B5-BEFB-46C9-9EE5-8A0681F13F8C")
    IUserDataPathsInterop : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_AdminTools( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *value) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_CDBurning( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *value) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_NetworkShortcuts( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *value) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Programs( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *value) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_SendTo( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *value) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_StartMenu( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *value) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Startup( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *value) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUserDataPathsInteropVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IUserDataPathsInterop * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IUserDataPathsInterop * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IUserDataPathsInterop * This);
        
        DECLSPEC_XFGVIRT(IUserDataPathsInterop, get_AdminTools)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_AdminTools )( 
            __RPC__in IUserDataPathsInterop * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *value);
        
        DECLSPEC_XFGVIRT(IUserDataPathsInterop, get_CDBurning)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_CDBurning )( 
            __RPC__in IUserDataPathsInterop * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *value);
        
        DECLSPEC_XFGVIRT(IUserDataPathsInterop, get_NetworkShortcuts)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_NetworkShortcuts )( 
            __RPC__in IUserDataPathsInterop * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *value);
        
        DECLSPEC_XFGVIRT(IUserDataPathsInterop, get_Programs)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Programs )( 
            __RPC__in IUserDataPathsInterop * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *value);
        
        DECLSPEC_XFGVIRT(IUserDataPathsInterop, get_SendTo)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_SendTo )( 
            __RPC__in IUserDataPathsInterop * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *value);
        
        DECLSPEC_XFGVIRT(IUserDataPathsInterop, get_StartMenu)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_StartMenu )( 
            __RPC__in IUserDataPathsInterop * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *value);
        
        DECLSPEC_XFGVIRT(IUserDataPathsInterop, get_Startup)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Startup )( 
            __RPC__in IUserDataPathsInterop * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *value);
        
        END_INTERFACE
    } IUserDataPathsInteropVtbl;

    interface IUserDataPathsInterop
    {
        CONST_VTBL struct IUserDataPathsInteropVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUserDataPathsInterop_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUserDataPathsInterop_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUserDataPathsInterop_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUserDataPathsInterop_get_AdminTools(This,value)	\
    ( (This)->lpVtbl -> get_AdminTools(This,value) ) 

#define IUserDataPathsInterop_get_CDBurning(This,value)	\
    ( (This)->lpVtbl -> get_CDBurning(This,value) ) 

#define IUserDataPathsInterop_get_NetworkShortcuts(This,value)	\
    ( (This)->lpVtbl -> get_NetworkShortcuts(This,value) ) 

#define IUserDataPathsInterop_get_Programs(This,value)	\
    ( (This)->lpVtbl -> get_Programs(This,value) ) 

#define IUserDataPathsInterop_get_SendTo(This,value)	\
    ( (This)->lpVtbl -> get_SendTo(This,value) ) 

#define IUserDataPathsInterop_get_StartMenu(This,value)	\
    ( (This)->lpVtbl -> get_StartMenu(This,value) ) 

#define IUserDataPathsInterop_get_Startup(This,value)	\
    ( (This)->lpVtbl -> get_Startup(This,value) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUserDataPathsInterop_INTERFACE_DEFINED__ */


#ifndef __ISystemDataPathsInterop_INTERFACE_DEFINED__
#define __ISystemDataPathsInterop_INTERFACE_DEFINED__

/* interface ISystemDataPathsInterop */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ISystemDataPathsInterop;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("864D0B26-3A37-4251-9A02-B231198DE060")
    ISystemDataPathsInterop : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_CommonAdminTools( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *value) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_CommonOemLinks( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *value) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_CommonPrograms( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *value) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_CommonStartMenu( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *value) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_CommonStartup( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *value) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_CommonTemplates( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *value) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_LocalizedResources( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *value) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ProgramFiles( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *value) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ProgramFilesCommon( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *value) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ProgramFilesCommonHost( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *value) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ProgramFilesCommonX64( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *value) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ProgramFilesCommonX86( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *value) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ProgramFilesCommonArm( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *value) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ProgramFilesHost( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *value) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ProgramFilesX64( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *value) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ProgramFilesX86( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *value) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ProgramFilesArm( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *value) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Resource( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *value) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISystemDataPathsInteropVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISystemDataPathsInterop * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISystemDataPathsInterop * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISystemDataPathsInterop * This);
        
        DECLSPEC_XFGVIRT(ISystemDataPathsInterop, get_CommonAdminTools)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_CommonAdminTools )( 
            __RPC__in ISystemDataPathsInterop * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *value);
        
        DECLSPEC_XFGVIRT(ISystemDataPathsInterop, get_CommonOemLinks)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_CommonOemLinks )( 
            __RPC__in ISystemDataPathsInterop * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *value);
        
        DECLSPEC_XFGVIRT(ISystemDataPathsInterop, get_CommonPrograms)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_CommonPrograms )( 
            __RPC__in ISystemDataPathsInterop * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *value);
        
        DECLSPEC_XFGVIRT(ISystemDataPathsInterop, get_CommonStartMenu)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_CommonStartMenu )( 
            __RPC__in ISystemDataPathsInterop * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *value);
        
        DECLSPEC_XFGVIRT(ISystemDataPathsInterop, get_CommonStartup)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_CommonStartup )( 
            __RPC__in ISystemDataPathsInterop * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *value);
        
        DECLSPEC_XFGVIRT(ISystemDataPathsInterop, get_CommonTemplates)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_CommonTemplates )( 
            __RPC__in ISystemDataPathsInterop * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *value);
        
        DECLSPEC_XFGVIRT(ISystemDataPathsInterop, get_LocalizedResources)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_LocalizedResources )( 
            __RPC__in ISystemDataPathsInterop * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *value);
        
        DECLSPEC_XFGVIRT(ISystemDataPathsInterop, get_ProgramFiles)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProgramFiles )( 
            __RPC__in ISystemDataPathsInterop * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *value);
        
        DECLSPEC_XFGVIRT(ISystemDataPathsInterop, get_ProgramFilesCommon)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProgramFilesCommon )( 
            __RPC__in ISystemDataPathsInterop * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *value);
        
        DECLSPEC_XFGVIRT(ISystemDataPathsInterop, get_ProgramFilesCommonHost)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProgramFilesCommonHost )( 
            __RPC__in ISystemDataPathsInterop * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *value);
        
        DECLSPEC_XFGVIRT(ISystemDataPathsInterop, get_ProgramFilesCommonX64)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProgramFilesCommonX64 )( 
            __RPC__in ISystemDataPathsInterop * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *value);
        
        DECLSPEC_XFGVIRT(ISystemDataPathsInterop, get_ProgramFilesCommonX86)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProgramFilesCommonX86 )( 
            __RPC__in ISystemDataPathsInterop * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *value);
        
        DECLSPEC_XFGVIRT(ISystemDataPathsInterop, get_ProgramFilesCommonArm)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProgramFilesCommonArm )( 
            __RPC__in ISystemDataPathsInterop * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *value);
        
        DECLSPEC_XFGVIRT(ISystemDataPathsInterop, get_ProgramFilesHost)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProgramFilesHost )( 
            __RPC__in ISystemDataPathsInterop * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *value);
        
        DECLSPEC_XFGVIRT(ISystemDataPathsInterop, get_ProgramFilesX64)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProgramFilesX64 )( 
            __RPC__in ISystemDataPathsInterop * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *value);
        
        DECLSPEC_XFGVIRT(ISystemDataPathsInterop, get_ProgramFilesX86)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProgramFilesX86 )( 
            __RPC__in ISystemDataPathsInterop * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *value);
        
        DECLSPEC_XFGVIRT(ISystemDataPathsInterop, get_ProgramFilesArm)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProgramFilesArm )( 
            __RPC__in ISystemDataPathsInterop * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *value);
        
        DECLSPEC_XFGVIRT(ISystemDataPathsInterop, get_Resource)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Resource )( 
            __RPC__in ISystemDataPathsInterop * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *value);
        
        END_INTERFACE
    } ISystemDataPathsInteropVtbl;

    interface ISystemDataPathsInterop
    {
        CONST_VTBL struct ISystemDataPathsInteropVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISystemDataPathsInterop_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISystemDataPathsInterop_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISystemDataPathsInterop_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISystemDataPathsInterop_get_CommonAdminTools(This,value)	\
    ( (This)->lpVtbl -> get_CommonAdminTools(This,value) ) 

#define ISystemDataPathsInterop_get_CommonOemLinks(This,value)	\
    ( (This)->lpVtbl -> get_CommonOemLinks(This,value) ) 

#define ISystemDataPathsInterop_get_CommonPrograms(This,value)	\
    ( (This)->lpVtbl -> get_CommonPrograms(This,value) ) 

#define ISystemDataPathsInterop_get_CommonStartMenu(This,value)	\
    ( (This)->lpVtbl -> get_CommonStartMenu(This,value) ) 

#define ISystemDataPathsInterop_get_CommonStartup(This,value)	\
    ( (This)->lpVtbl -> get_CommonStartup(This,value) ) 

#define ISystemDataPathsInterop_get_CommonTemplates(This,value)	\
    ( (This)->lpVtbl -> get_CommonTemplates(This,value) ) 

#define ISystemDataPathsInterop_get_LocalizedResources(This,value)	\
    ( (This)->lpVtbl -> get_LocalizedResources(This,value) ) 

#define ISystemDataPathsInterop_get_ProgramFiles(This,value)	\
    ( (This)->lpVtbl -> get_ProgramFiles(This,value) ) 

#define ISystemDataPathsInterop_get_ProgramFilesCommon(This,value)	\
    ( (This)->lpVtbl -> get_ProgramFilesCommon(This,value) ) 

#define ISystemDataPathsInterop_get_ProgramFilesCommonHost(This,value)	\
    ( (This)->lpVtbl -> get_ProgramFilesCommonHost(This,value) ) 

#define ISystemDataPathsInterop_get_ProgramFilesCommonX64(This,value)	\
    ( (This)->lpVtbl -> get_ProgramFilesCommonX64(This,value) ) 

#define ISystemDataPathsInterop_get_ProgramFilesCommonX86(This,value)	\
    ( (This)->lpVtbl -> get_ProgramFilesCommonX86(This,value) ) 

#define ISystemDataPathsInterop_get_ProgramFilesCommonArm(This,value)	\
    ( (This)->lpVtbl -> get_ProgramFilesCommonArm(This,value) ) 

#define ISystemDataPathsInterop_get_ProgramFilesHost(This,value)	\
    ( (This)->lpVtbl -> get_ProgramFilesHost(This,value) ) 

#define ISystemDataPathsInterop_get_ProgramFilesX64(This,value)	\
    ( (This)->lpVtbl -> get_ProgramFilesX64(This,value) ) 

#define ISystemDataPathsInterop_get_ProgramFilesX86(This,value)	\
    ( (This)->lpVtbl -> get_ProgramFilesX86(This,value) ) 

#define ISystemDataPathsInterop_get_ProgramFilesArm(This,value)	\
    ( (This)->lpVtbl -> get_ProgramFilesArm(This,value) ) 

#define ISystemDataPathsInterop_get_Resource(This,value)	\
    ( (This)->lpVtbl -> get_Resource(This,value) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISystemDataPathsInterop_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_knownfolderpathscom_0000_0002 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_knownfolderpathscom_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_knownfolderpathscom_0000_0002_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


