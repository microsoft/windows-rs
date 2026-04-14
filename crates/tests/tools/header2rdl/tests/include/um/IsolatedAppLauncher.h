

/* this ALWAYS GENERATED file contains the definitions for the interfaces */


 /* File created by MIDL compiler version 8.01.0628 */
/* @@MIDL_FILE_HEADING(  ) */

#pragma warning( disable: 4049 )  /* more than 64k source lines */


/* verify that the <rpcndr.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCNDR_H_VERSION__
#define __REQUIRED_RPCNDR_H_VERSION__ 475
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

#ifndef __isolatedapplauncher_h__
#define __isolatedapplauncher_h__

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

#ifndef __IIsolatedAppLauncher_FWD_DEFINED__
#define __IIsolatedAppLauncher_FWD_DEFINED__
typedef interface IIsolatedAppLauncher IIsolatedAppLauncher;

#endif 	/* __IIsolatedAppLauncher_FWD_DEFINED__ */


#ifndef __IsolatedAppLauncher_FWD_DEFINED__
#define __IsolatedAppLauncher_FWD_DEFINED__

#ifdef __cplusplus
typedef class IsolatedAppLauncher IsolatedAppLauncher;
#else
typedef struct IsolatedAppLauncher IsolatedAppLauncher;
#endif /* __cplusplus */

#endif 	/* __IsolatedAppLauncher_FWD_DEFINED__ */


#ifndef __IIsolatedProcessLauncher_FWD_DEFINED__
#define __IIsolatedProcessLauncher_FWD_DEFINED__
typedef interface IIsolatedProcessLauncher IIsolatedProcessLauncher;

#endif 	/* __IIsolatedProcessLauncher_FWD_DEFINED__ */


#ifndef __IIsolatedProcessLauncher2_FWD_DEFINED__
#define __IIsolatedProcessLauncher2_FWD_DEFINED__
typedef interface IIsolatedProcessLauncher2 IIsolatedProcessLauncher2;

#endif 	/* __IIsolatedProcessLauncher2_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_isolatedapplauncher_0000_0000 */
/* [local] */ 

#pragma warning(disable: 4995)
#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
typedef struct _IsolatedAppLauncherTelemetryParameters
    {
    BOOL EnableForLaunch;
    GUID CorrelationGUID;
    } 	IsolatedAppLauncherTelemetryParameters;

#pragma deprecated(IsolatedAppLauncherTelemetryParameters)


extern RPC_IF_HANDLE __MIDL_itf_isolatedapplauncher_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_isolatedapplauncher_0000_0000_v0_0_s_ifspec;

#ifndef __IIsolatedAppLauncher_INTERFACE_DEFINED__
#define __IIsolatedAppLauncher_INTERFACE_DEFINED__

/* interface IIsolatedAppLauncher */
/* [uuid][object] */ 


EXTERN_C const IID IID_IIsolatedAppLauncher;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F686878F-7B42-4CC4-96FB-F4F3B6E3D24D")
    IIsolatedAppLauncher : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Launch( 
            /* [string][in] */ __RPC__in_string LPCWSTR appUserModelId,
            /* [string][in] */ __RPC__in_string LPCWSTR arguments,
            /* [in] */ __RPC__in const IsolatedAppLauncherTelemetryParameters *telemetryParameters) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IIsolatedAppLauncherVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IIsolatedAppLauncher * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IIsolatedAppLauncher * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IIsolatedAppLauncher * This);
        
        DECLSPEC_XFGVIRT(IIsolatedAppLauncher, Launch)
        HRESULT ( STDMETHODCALLTYPE *Launch )( 
            __RPC__in IIsolatedAppLauncher * This,
            /* [string][in] */ __RPC__in_string LPCWSTR appUserModelId,
            /* [string][in] */ __RPC__in_string LPCWSTR arguments,
            /* [in] */ __RPC__in const IsolatedAppLauncherTelemetryParameters *telemetryParameters);
        
        END_INTERFACE
    } IIsolatedAppLauncherVtbl;

    interface IIsolatedAppLauncher
    {
        CONST_VTBL struct IIsolatedAppLauncherVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IIsolatedAppLauncher_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IIsolatedAppLauncher_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IIsolatedAppLauncher_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IIsolatedAppLauncher_Launch(This,appUserModelId,arguments,telemetryParameters)	\
    ( (This)->lpVtbl -> Launch(This,appUserModelId,arguments,telemetryParameters) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IIsolatedAppLauncher_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_isolatedapplauncher_0000_0001 */
/* [local] */ 

#pragma deprecated(IIsolatedAppLauncher)


extern RPC_IF_HANDLE __MIDL_itf_isolatedapplauncher_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_isolatedapplauncher_0000_0001_v0_0_s_ifspec;


#ifndef __IsolatedAppLauncherLibrary_LIBRARY_DEFINED__
#define __IsolatedAppLauncherLibrary_LIBRARY_DEFINED__

/* library IsolatedAppLauncherLibrary */
/* [version][uuid] */ 


EXTERN_C const IID LIBID_IsolatedAppLauncherLibrary;

EXTERN_C const CLSID CLSID_IsolatedAppLauncher;

#ifdef __cplusplus

class DECLSPEC_UUID("BC812430-E75E-4FD1-9641-1F9F1E2D9A1F")
IsolatedAppLauncher;
#endif
#endif /* __IsolatedAppLauncherLibrary_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_isolatedapplauncher_0000_0002 */
/* [local] */ 

#pragma deprecated(IsolatedAppLauncherLibrary)
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region App Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
#if (NTDDI_VERSION >= NTDDI_WIN10_19H1)
__declspec(deprecated("IsProcessInWDAGContainer is deprecated and might not work on all platforms. For more info, see MSDN."))
STDAPI IsProcessInWDAGContainer(_In_ PVOID Reserved, _Out_ BOOL * isProcessInWDAGContainer);
#endif // (NTDDI_VERSION >= NTDDI_WIN10_19H1)
__declspec(deprecated("IsProcessInIsolatedContainer is deprecated and might not work on all platforms. For more info, see MSDN."))
STDAPI IsProcessInIsolatedContainer(_Out_ BOOL *isProcessInIsolatedContainer);
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#if (NTDDI_VERSION >= NTDDI_WIN10_NI)


extern RPC_IF_HANDLE __MIDL_itf_isolatedapplauncher_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_isolatedapplauncher_0000_0002_v0_0_s_ifspec;

#ifndef __IIsolatedProcessLauncher_INTERFACE_DEFINED__
#define __IIsolatedProcessLauncher_INTERFACE_DEFINED__

/* interface IIsolatedProcessLauncher */
/* [uuid][object] */ 


EXTERN_C const IID IID_IIsolatedProcessLauncher;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1aa24232-9a91-4201-88cb-122f9d6522e0")
    IIsolatedProcessLauncher : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE LaunchProcess( 
            /* [string][in] */ __RPC__in_string LPCWSTR process,
            /* [string][in] */ __RPC__in_string LPCWSTR arguments,
            /* [string][in] */ __RPC__in_string LPCWSTR workingDirectory) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ShareDirectory( 
            /* [string][in] */ __RPC__in_string LPCWSTR hostPath,
            /* [string][in] */ __RPC__in_string LPCWSTR containerPath,
            /* [in] */ BOOL readOnly) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetContainerGuid( 
            /* [out] */ __RPC__out GUID *guid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AllowSetForegroundAccess( 
            /* [in] */ UINT pid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsContainerRunning( 
            /* [out] */ __RPC__out BOOL *running) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IIsolatedProcessLauncherVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IIsolatedProcessLauncher * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IIsolatedProcessLauncher * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IIsolatedProcessLauncher * This);
        
        DECLSPEC_XFGVIRT(IIsolatedProcessLauncher, LaunchProcess)
        HRESULT ( STDMETHODCALLTYPE *LaunchProcess )( 
            __RPC__in IIsolatedProcessLauncher * This,
            /* [string][in] */ __RPC__in_string LPCWSTR process,
            /* [string][in] */ __RPC__in_string LPCWSTR arguments,
            /* [string][in] */ __RPC__in_string LPCWSTR workingDirectory);
        
        DECLSPEC_XFGVIRT(IIsolatedProcessLauncher, ShareDirectory)
        HRESULT ( STDMETHODCALLTYPE *ShareDirectory )( 
            __RPC__in IIsolatedProcessLauncher * This,
            /* [string][in] */ __RPC__in_string LPCWSTR hostPath,
            /* [string][in] */ __RPC__in_string LPCWSTR containerPath,
            /* [in] */ BOOL readOnly);
        
        DECLSPEC_XFGVIRT(IIsolatedProcessLauncher, GetContainerGuid)
        HRESULT ( STDMETHODCALLTYPE *GetContainerGuid )( 
            __RPC__in IIsolatedProcessLauncher * This,
            /* [out] */ __RPC__out GUID *guid);
        
        DECLSPEC_XFGVIRT(IIsolatedProcessLauncher, AllowSetForegroundAccess)
        HRESULT ( STDMETHODCALLTYPE *AllowSetForegroundAccess )( 
            __RPC__in IIsolatedProcessLauncher * This,
            /* [in] */ UINT pid);
        
        DECLSPEC_XFGVIRT(IIsolatedProcessLauncher, IsContainerRunning)
        HRESULT ( STDMETHODCALLTYPE *IsContainerRunning )( 
            __RPC__in IIsolatedProcessLauncher * This,
            /* [out] */ __RPC__out BOOL *running);
        
        END_INTERFACE
    } IIsolatedProcessLauncherVtbl;

    interface IIsolatedProcessLauncher
    {
        CONST_VTBL struct IIsolatedProcessLauncherVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IIsolatedProcessLauncher_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IIsolatedProcessLauncher_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IIsolatedProcessLauncher_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IIsolatedProcessLauncher_LaunchProcess(This,process,arguments,workingDirectory)	\
    ( (This)->lpVtbl -> LaunchProcess(This,process,arguments,workingDirectory) ) 

#define IIsolatedProcessLauncher_ShareDirectory(This,hostPath,containerPath,readOnly)	\
    ( (This)->lpVtbl -> ShareDirectory(This,hostPath,containerPath,readOnly) ) 

#define IIsolatedProcessLauncher_GetContainerGuid(This,guid)	\
    ( (This)->lpVtbl -> GetContainerGuid(This,guid) ) 

#define IIsolatedProcessLauncher_AllowSetForegroundAccess(This,pid)	\
    ( (This)->lpVtbl -> AllowSetForegroundAccess(This,pid) ) 

#define IIsolatedProcessLauncher_IsContainerRunning(This,running)	\
    ( (This)->lpVtbl -> IsContainerRunning(This,running) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IIsolatedProcessLauncher_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_isolatedapplauncher_0000_0003 */
/* [local] */ 

#pragma deprecated(IIsolatedProcessLauncher)


extern RPC_IF_HANDLE __MIDL_itf_isolatedapplauncher_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_isolatedapplauncher_0000_0003_v0_0_s_ifspec;

#ifndef __IIsolatedProcessLauncher2_INTERFACE_DEFINED__
#define __IIsolatedProcessLauncher2_INTERFACE_DEFINED__

/* interface IIsolatedProcessLauncher2 */
/* [uuid][object] */ 


EXTERN_C const IID IID_IIsolatedProcessLauncher2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("780e4416-5e72-4123-808e-66dc6479feef")
    IIsolatedProcessLauncher2 : public IIsolatedProcessLauncher
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE LaunchProcess2( 
            /* [string][in] */ __RPC__in_string LPCWSTR process,
            /* [string][in] */ __RPC__in_string LPCWSTR arguments,
            /* [string][in] */ __RPC__in_string LPCWSTR workingDirectory,
            /* [in] */ __RPC__in REFGUID correlationGuid) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IIsolatedProcessLauncher2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IIsolatedProcessLauncher2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IIsolatedProcessLauncher2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IIsolatedProcessLauncher2 * This);
        
        DECLSPEC_XFGVIRT(IIsolatedProcessLauncher, LaunchProcess)
        HRESULT ( STDMETHODCALLTYPE *LaunchProcess )( 
            __RPC__in IIsolatedProcessLauncher2 * This,
            /* [string][in] */ __RPC__in_string LPCWSTR process,
            /* [string][in] */ __RPC__in_string LPCWSTR arguments,
            /* [string][in] */ __RPC__in_string LPCWSTR workingDirectory);
        
        DECLSPEC_XFGVIRT(IIsolatedProcessLauncher, ShareDirectory)
        HRESULT ( STDMETHODCALLTYPE *ShareDirectory )( 
            __RPC__in IIsolatedProcessLauncher2 * This,
            /* [string][in] */ __RPC__in_string LPCWSTR hostPath,
            /* [string][in] */ __RPC__in_string LPCWSTR containerPath,
            /* [in] */ BOOL readOnly);
        
        DECLSPEC_XFGVIRT(IIsolatedProcessLauncher, GetContainerGuid)
        HRESULT ( STDMETHODCALLTYPE *GetContainerGuid )( 
            __RPC__in IIsolatedProcessLauncher2 * This,
            /* [out] */ __RPC__out GUID *guid);
        
        DECLSPEC_XFGVIRT(IIsolatedProcessLauncher, AllowSetForegroundAccess)
        HRESULT ( STDMETHODCALLTYPE *AllowSetForegroundAccess )( 
            __RPC__in IIsolatedProcessLauncher2 * This,
            /* [in] */ UINT pid);
        
        DECLSPEC_XFGVIRT(IIsolatedProcessLauncher, IsContainerRunning)
        HRESULT ( STDMETHODCALLTYPE *IsContainerRunning )( 
            __RPC__in IIsolatedProcessLauncher2 * This,
            /* [out] */ __RPC__out BOOL *running);
        
        DECLSPEC_XFGVIRT(IIsolatedProcessLauncher2, LaunchProcess2)
        HRESULT ( STDMETHODCALLTYPE *LaunchProcess2 )( 
            __RPC__in IIsolatedProcessLauncher2 * This,
            /* [string][in] */ __RPC__in_string LPCWSTR process,
            /* [string][in] */ __RPC__in_string LPCWSTR arguments,
            /* [string][in] */ __RPC__in_string LPCWSTR workingDirectory,
            /* [in] */ __RPC__in REFGUID correlationGuid);
        
        END_INTERFACE
    } IIsolatedProcessLauncher2Vtbl;

    interface IIsolatedProcessLauncher2
    {
        CONST_VTBL struct IIsolatedProcessLauncher2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IIsolatedProcessLauncher2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IIsolatedProcessLauncher2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IIsolatedProcessLauncher2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IIsolatedProcessLauncher2_LaunchProcess(This,process,arguments,workingDirectory)	\
    ( (This)->lpVtbl -> LaunchProcess(This,process,arguments,workingDirectory) ) 

#define IIsolatedProcessLauncher2_ShareDirectory(This,hostPath,containerPath,readOnly)	\
    ( (This)->lpVtbl -> ShareDirectory(This,hostPath,containerPath,readOnly) ) 

#define IIsolatedProcessLauncher2_GetContainerGuid(This,guid)	\
    ( (This)->lpVtbl -> GetContainerGuid(This,guid) ) 

#define IIsolatedProcessLauncher2_AllowSetForegroundAccess(This,pid)	\
    ( (This)->lpVtbl -> AllowSetForegroundAccess(This,pid) ) 

#define IIsolatedProcessLauncher2_IsContainerRunning(This,running)	\
    ( (This)->lpVtbl -> IsContainerRunning(This,running) ) 


#define IIsolatedProcessLauncher2_LaunchProcess2(This,process,arguments,workingDirectory,correlationGuid)	\
    ( (This)->lpVtbl -> LaunchProcess2(This,process,arguments,workingDirectory,correlationGuid) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IIsolatedProcessLauncher2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_isolatedapplauncher_0000_0004 */
/* [local] */ 

#pragma deprecated(IIsolatedProcessLauncher2)
#endif // NTDDI_WIN10_NI


extern RPC_IF_HANDLE __MIDL_itf_isolatedapplauncher_0000_0004_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_isolatedapplauncher_0000_0004_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


