

/* this ALWAYS GENERATED file contains the definitions for the interfaces */


 /* File created by MIDL compiler version 8.01.0628 */
/* @@MIDL_FILE_HEADING(  ) */

#pragma warning( disable: 4049 )  /* more than 64k source lines */


/* verify that the <rpcndr.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCNDR_H_VERSION__
#define __REQUIRED_RPCNDR_H_VERSION__ 475
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

#ifndef __iwamreg_h__
#define __iwamreg_h__

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

#ifndef __IWamAdmin_FWD_DEFINED__
#define __IWamAdmin_FWD_DEFINED__
typedef interface IWamAdmin IWamAdmin;

#endif 	/* __IWamAdmin_FWD_DEFINED__ */


#ifndef __IWamAdmin2_FWD_DEFINED__
#define __IWamAdmin2_FWD_DEFINED__
typedef interface IWamAdmin2 IWamAdmin2;

#endif 	/* __IWamAdmin2_FWD_DEFINED__ */


#ifndef __IIISApplicationAdmin_FWD_DEFINED__
#define __IIISApplicationAdmin_FWD_DEFINED__
typedef interface IIISApplicationAdmin IIISApplicationAdmin;

#endif 	/* __IIISApplicationAdmin_FWD_DEFINED__ */


#ifndef __WamAdmin_FWD_DEFINED__
#define __WamAdmin_FWD_DEFINED__

#ifdef __cplusplus
typedef class WamAdmin WamAdmin;
#else
typedef struct WamAdmin WamAdmin;
#endif /* __cplusplus */

#endif 	/* __WamAdmin_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_wamreg_0000_0000 */
/* [local] */ 

/*++
                                                                                
Copyright (c) 1997-2001 Microsoft Corporation
                                                                                
Module Name: iwamreg.h
                                                                                
    WAM (Web Application Manager) Interfaces
                                                                                
--*/
#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#ifndef __WAMREG_IADM__IID
#define __WAMREG_IADM__IID
DEFINE_GUID(IID_IWamAdmin, 0x29822AB7, 0xF302, 0x11D0, 0x99, 0x53, 0x00, 0xC0, 0x4F, 0xD9, 0x19, 0xC1);
DEFINE_GUID(IID_IWamAdmin2, 0x29822AB8, 0xF302, 0x11D0, 0x99, 0x53, 0x00, 0xC0, 0x4F, 0xD9, 0x19, 0xC1);
DEFINE_GUID(IID_IIISApplicationAdmin, 0x7C4E1804, 0xE342, 0x483D, 0xA4, 0x3E, 0xA8, 0x50, 0xCF, 0xCC, 0x8D, 0x18);
DEFINE_GUID(IID_IIISApplicationAdmin2, 0xd643717a, 0xfc87, 0x4260, 0x88, 0xac, 0x6c, 0xe3, 0x5f, 0x0e, 0xc1, 0x4e);
DEFINE_GUID(LIBID_WAMREGLib, 0x29822AA8, 0xF302, 0x11D0, 0x99, 0x53, 0x00, 0xC0, 0x4F, 0xD9, 0x19, 0xC1);
DEFINE_GUID(CLSID_WamAdmin, 0x61738644, 0xF196, 0x11D0, 0x99, 0x53, 0x00, 0xC0, 0x4F, 0xD9, 0x19, 0xC1);
#endif //__WAMREG_IADM__IID
#define APPSTATUS_STOPPED	0
#define APPSTATUS_RUNNING	1
#define APPSTATUS_NOTDEFINED	2
typedef /* [public] */ 
enum __MIDL___MIDL_itf_wamreg_0000_0000_0001
    {
        eAppRunInProc	= 0,
        eAppRunOutProcIsolated	= ( eAppRunInProc + 1 ) ,
        eAppRunOutProcInDefaultPool	= ( eAppRunOutProcIsolated + 1 ) 
    } 	EAppMode;



extern RPC_IF_HANDLE __MIDL_itf_wamreg_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wamreg_0000_0000_v0_0_s_ifspec;

#ifndef __IWamAdmin_INTERFACE_DEFINED__
#define __IWamAdmin_INTERFACE_DEFINED__

/* interface IWamAdmin */
/* [object][unique][helpstring][uuid] */ 


EXTERN_C const IID IID_IWamAdmin;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("29822AB7-F302-11D0-9953-00C04FD919C1")
    IWamAdmin : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE AppCreate( 
            /* [string][unique][in] */ LPCWSTR szMDPath,
            /* [in] */ BOOL fInProc) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE AppDelete( 
            /* [string][unique][in] */ LPCWSTR szMDPath,
            /* [in] */ BOOL fRecursive) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE AppUnLoad( 
            /* [string][unique][in] */ LPCWSTR szMDPath,
            /* [in] */ BOOL fRecursive) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE AppGetStatus( 
            /* [string][unique][in] */ LPCWSTR szMDPath,
            /* [out] */ DWORD *pdwAppStatus) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE AppDeleteRecoverable( 
            /* [string][unique][in] */ LPCWSTR szMDPath,
            /* [in] */ BOOL fRecursive) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE AppRecover( 
            /* [string][unique][in] */ LPCWSTR szMDPath,
            /* [in] */ BOOL fRecursive) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWamAdminVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWamAdmin * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWamAdmin * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWamAdmin * This);
        
        DECLSPEC_XFGVIRT(IWamAdmin, AppCreate)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *AppCreate )( 
            IWamAdmin * This,
            /* [string][unique][in] */ LPCWSTR szMDPath,
            /* [in] */ BOOL fInProc);
        
        DECLSPEC_XFGVIRT(IWamAdmin, AppDelete)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *AppDelete )( 
            IWamAdmin * This,
            /* [string][unique][in] */ LPCWSTR szMDPath,
            /* [in] */ BOOL fRecursive);
        
        DECLSPEC_XFGVIRT(IWamAdmin, AppUnLoad)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *AppUnLoad )( 
            IWamAdmin * This,
            /* [string][unique][in] */ LPCWSTR szMDPath,
            /* [in] */ BOOL fRecursive);
        
        DECLSPEC_XFGVIRT(IWamAdmin, AppGetStatus)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *AppGetStatus )( 
            IWamAdmin * This,
            /* [string][unique][in] */ LPCWSTR szMDPath,
            /* [out] */ DWORD *pdwAppStatus);
        
        DECLSPEC_XFGVIRT(IWamAdmin, AppDeleteRecoverable)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *AppDeleteRecoverable )( 
            IWamAdmin * This,
            /* [string][unique][in] */ LPCWSTR szMDPath,
            /* [in] */ BOOL fRecursive);
        
        DECLSPEC_XFGVIRT(IWamAdmin, AppRecover)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *AppRecover )( 
            IWamAdmin * This,
            /* [string][unique][in] */ LPCWSTR szMDPath,
            /* [in] */ BOOL fRecursive);
        
        END_INTERFACE
    } IWamAdminVtbl;

    interface IWamAdmin
    {
        CONST_VTBL struct IWamAdminVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWamAdmin_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWamAdmin_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWamAdmin_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWamAdmin_AppCreate(This,szMDPath,fInProc)	\
    ( (This)->lpVtbl -> AppCreate(This,szMDPath,fInProc) ) 

#define IWamAdmin_AppDelete(This,szMDPath,fRecursive)	\
    ( (This)->lpVtbl -> AppDelete(This,szMDPath,fRecursive) ) 

#define IWamAdmin_AppUnLoad(This,szMDPath,fRecursive)	\
    ( (This)->lpVtbl -> AppUnLoad(This,szMDPath,fRecursive) ) 

#define IWamAdmin_AppGetStatus(This,szMDPath,pdwAppStatus)	\
    ( (This)->lpVtbl -> AppGetStatus(This,szMDPath,pdwAppStatus) ) 

#define IWamAdmin_AppDeleteRecoverable(This,szMDPath,fRecursive)	\
    ( (This)->lpVtbl -> AppDeleteRecoverable(This,szMDPath,fRecursive) ) 

#define IWamAdmin_AppRecover(This,szMDPath,fRecursive)	\
    ( (This)->lpVtbl -> AppRecover(This,szMDPath,fRecursive) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWamAdmin_INTERFACE_DEFINED__ */


#ifndef __IWamAdmin2_INTERFACE_DEFINED__
#define __IWamAdmin2_INTERFACE_DEFINED__

/* interface IWamAdmin2 */
/* [object][unique][helpstring][uuid] */ 


EXTERN_C const IID IID_IWamAdmin2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("29822AB8-F302-11D0-9953-00C04FD919C1")
    IWamAdmin2 : public IWamAdmin
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE AppCreate2( 
            /* [string][unique][in] */ LPCWSTR szMDPath,
            /* [in] */ DWORD dwAppMode) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWamAdmin2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWamAdmin2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWamAdmin2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWamAdmin2 * This);
        
        DECLSPEC_XFGVIRT(IWamAdmin, AppCreate)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *AppCreate )( 
            IWamAdmin2 * This,
            /* [string][unique][in] */ LPCWSTR szMDPath,
            /* [in] */ BOOL fInProc);
        
        DECLSPEC_XFGVIRT(IWamAdmin, AppDelete)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *AppDelete )( 
            IWamAdmin2 * This,
            /* [string][unique][in] */ LPCWSTR szMDPath,
            /* [in] */ BOOL fRecursive);
        
        DECLSPEC_XFGVIRT(IWamAdmin, AppUnLoad)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *AppUnLoad )( 
            IWamAdmin2 * This,
            /* [string][unique][in] */ LPCWSTR szMDPath,
            /* [in] */ BOOL fRecursive);
        
        DECLSPEC_XFGVIRT(IWamAdmin, AppGetStatus)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *AppGetStatus )( 
            IWamAdmin2 * This,
            /* [string][unique][in] */ LPCWSTR szMDPath,
            /* [out] */ DWORD *pdwAppStatus);
        
        DECLSPEC_XFGVIRT(IWamAdmin, AppDeleteRecoverable)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *AppDeleteRecoverable )( 
            IWamAdmin2 * This,
            /* [string][unique][in] */ LPCWSTR szMDPath,
            /* [in] */ BOOL fRecursive);
        
        DECLSPEC_XFGVIRT(IWamAdmin, AppRecover)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *AppRecover )( 
            IWamAdmin2 * This,
            /* [string][unique][in] */ LPCWSTR szMDPath,
            /* [in] */ BOOL fRecursive);
        
        DECLSPEC_XFGVIRT(IWamAdmin2, AppCreate2)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *AppCreate2 )( 
            IWamAdmin2 * This,
            /* [string][unique][in] */ LPCWSTR szMDPath,
            /* [in] */ DWORD dwAppMode);
        
        END_INTERFACE
    } IWamAdmin2Vtbl;

    interface IWamAdmin2
    {
        CONST_VTBL struct IWamAdmin2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWamAdmin2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWamAdmin2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWamAdmin2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWamAdmin2_AppCreate(This,szMDPath,fInProc)	\
    ( (This)->lpVtbl -> AppCreate(This,szMDPath,fInProc) ) 

#define IWamAdmin2_AppDelete(This,szMDPath,fRecursive)	\
    ( (This)->lpVtbl -> AppDelete(This,szMDPath,fRecursive) ) 

#define IWamAdmin2_AppUnLoad(This,szMDPath,fRecursive)	\
    ( (This)->lpVtbl -> AppUnLoad(This,szMDPath,fRecursive) ) 

#define IWamAdmin2_AppGetStatus(This,szMDPath,pdwAppStatus)	\
    ( (This)->lpVtbl -> AppGetStatus(This,szMDPath,pdwAppStatus) ) 

#define IWamAdmin2_AppDeleteRecoverable(This,szMDPath,fRecursive)	\
    ( (This)->lpVtbl -> AppDeleteRecoverable(This,szMDPath,fRecursive) ) 

#define IWamAdmin2_AppRecover(This,szMDPath,fRecursive)	\
    ( (This)->lpVtbl -> AppRecover(This,szMDPath,fRecursive) ) 


#define IWamAdmin2_AppCreate2(This,szMDPath,dwAppMode)	\
    ( (This)->lpVtbl -> AppCreate2(This,szMDPath,dwAppMode) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWamAdmin2_INTERFACE_DEFINED__ */


#ifndef __IIISApplicationAdmin_INTERFACE_DEFINED__
#define __IIISApplicationAdmin_INTERFACE_DEFINED__

/* interface IIISApplicationAdmin */
/* [object][unique][helpstring][uuid] */ 


EXTERN_C const IID IID_IIISApplicationAdmin;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7C4E1804-E342-483D-A43E-A850CFCC8D18")
    IIISApplicationAdmin : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE CreateApplication( 
            /* [string][unique][in] */ LPCWSTR szMDPath,
            /* [in] */ DWORD dwAppMode,
            /* [string][unique][in] */ LPCWSTR szAppPoolId,
            /* [in] */ BOOL fCreatePool) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE DeleteApplication( 
            /* [string][unique][in] */ LPCWSTR szMDPath,
            /* [in] */ BOOL fRecursive) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE CreateApplicationPool( 
            /* [string][unique][in] */ LPCWSTR szPool) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE DeleteApplicationPool( 
            /* [string][unique][in] */ LPCWSTR szPool) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE EnumerateApplicationsInPool( 
            /* [string][unique][in] */ LPCWSTR szPool,
            /* [out] */ BSTR *bstrBuffer) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE RecycleApplicationPool( 
            /* [string][unique][in] */ LPCWSTR szPool) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetProcessMode( 
            /* [out] */ DWORD *pdwMode) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IIISApplicationAdminVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IIISApplicationAdmin * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IIISApplicationAdmin * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IIISApplicationAdmin * This);
        
        DECLSPEC_XFGVIRT(IIISApplicationAdmin, CreateApplication)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *CreateApplication )( 
            IIISApplicationAdmin * This,
            /* [string][unique][in] */ LPCWSTR szMDPath,
            /* [in] */ DWORD dwAppMode,
            /* [string][unique][in] */ LPCWSTR szAppPoolId,
            /* [in] */ BOOL fCreatePool);
        
        DECLSPEC_XFGVIRT(IIISApplicationAdmin, DeleteApplication)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *DeleteApplication )( 
            IIISApplicationAdmin * This,
            /* [string][unique][in] */ LPCWSTR szMDPath,
            /* [in] */ BOOL fRecursive);
        
        DECLSPEC_XFGVIRT(IIISApplicationAdmin, CreateApplicationPool)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *CreateApplicationPool )( 
            IIISApplicationAdmin * This,
            /* [string][unique][in] */ LPCWSTR szPool);
        
        DECLSPEC_XFGVIRT(IIISApplicationAdmin, DeleteApplicationPool)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *DeleteApplicationPool )( 
            IIISApplicationAdmin * This,
            /* [string][unique][in] */ LPCWSTR szPool);
        
        DECLSPEC_XFGVIRT(IIISApplicationAdmin, EnumerateApplicationsInPool)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *EnumerateApplicationsInPool )( 
            IIISApplicationAdmin * This,
            /* [string][unique][in] */ LPCWSTR szPool,
            /* [out] */ BSTR *bstrBuffer);
        
        DECLSPEC_XFGVIRT(IIISApplicationAdmin, RecycleApplicationPool)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *RecycleApplicationPool )( 
            IIISApplicationAdmin * This,
            /* [string][unique][in] */ LPCWSTR szPool);
        
        DECLSPEC_XFGVIRT(IIISApplicationAdmin, GetProcessMode)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetProcessMode )( 
            IIISApplicationAdmin * This,
            /* [out] */ DWORD *pdwMode);
        
        END_INTERFACE
    } IIISApplicationAdminVtbl;

    interface IIISApplicationAdmin
    {
        CONST_VTBL struct IIISApplicationAdminVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IIISApplicationAdmin_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IIISApplicationAdmin_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IIISApplicationAdmin_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IIISApplicationAdmin_CreateApplication(This,szMDPath,dwAppMode,szAppPoolId,fCreatePool)	\
    ( (This)->lpVtbl -> CreateApplication(This,szMDPath,dwAppMode,szAppPoolId,fCreatePool) ) 

#define IIISApplicationAdmin_DeleteApplication(This,szMDPath,fRecursive)	\
    ( (This)->lpVtbl -> DeleteApplication(This,szMDPath,fRecursive) ) 

#define IIISApplicationAdmin_CreateApplicationPool(This,szPool)	\
    ( (This)->lpVtbl -> CreateApplicationPool(This,szPool) ) 

#define IIISApplicationAdmin_DeleteApplicationPool(This,szPool)	\
    ( (This)->lpVtbl -> DeleteApplicationPool(This,szPool) ) 

#define IIISApplicationAdmin_EnumerateApplicationsInPool(This,szPool,bstrBuffer)	\
    ( (This)->lpVtbl -> EnumerateApplicationsInPool(This,szPool,bstrBuffer) ) 

#define IIISApplicationAdmin_RecycleApplicationPool(This,szPool)	\
    ( (This)->lpVtbl -> RecycleApplicationPool(This,szPool) ) 

#define IIISApplicationAdmin_GetProcessMode(This,pdwMode)	\
    ( (This)->lpVtbl -> GetProcessMode(This,pdwMode) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IIISApplicationAdmin_INTERFACE_DEFINED__ */



#ifndef __WAMREGLib_LIBRARY_DEFINED__
#define __WAMREGLib_LIBRARY_DEFINED__

/* library WAMREGLib */
/* [helpstring][version][uuid] */ 


EXTERN_C const IID LIBID_WAMREGLib;

EXTERN_C const CLSID CLSID_WamAdmin;

#ifdef __cplusplus

class DECLSPEC_UUID("61738644-F196-11D0-9953-00C04FD919C1")
WamAdmin;
#endif
#endif /* __WAMREGLib_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_wamreg_0000_0004 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_wamreg_0000_0004_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wamreg_0000_0004_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  BSTR_UserSize(     unsigned long *, unsigned long            , BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal(  unsigned long *, unsigned char *, BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal(unsigned long *, unsigned char *, BSTR * ); 
void                      __RPC_USER  BSTR_UserFree(     unsigned long *, BSTR * ); 

unsigned long             __RPC_USER  BSTR_UserSize64(     unsigned long *, unsigned long            , BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal64(  unsigned long *, unsigned char *, BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal64(unsigned long *, unsigned char *, BSTR * ); 
void                      __RPC_USER  BSTR_UserFree64(     unsigned long *, BSTR * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


