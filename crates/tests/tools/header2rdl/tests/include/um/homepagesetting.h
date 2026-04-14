

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

#ifndef __homepagesetting_h__
#define __homepagesetting_h__

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

#ifndef __IHomePageSetting_FWD_DEFINED__
#define __IHomePageSetting_FWD_DEFINED__
typedef interface IHomePageSetting IHomePageSetting;

#endif 	/* __IHomePageSetting_FWD_DEFINED__ */


#ifndef __HomePageSetting_FWD_DEFINED__
#define __HomePageSetting_FWD_DEFINED__

#ifdef __cplusplus
typedef class HomePageSetting HomePageSetting;
#else
typedef struct HomePageSetting HomePageSetting;
#endif /* __cplusplus */

#endif 	/* __HomePageSetting_FWD_DEFINED__ */


/* header files for imported files */
#include "objidl.h"
#include "oleidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_homepagesetting_0000_0000 */
/* [local] */ 

//=--------------------------------------------------------------------------=
// homepagesetting.h
//=--------------------------------------------------------------------------=
// (C) Copyright Microsoft Corporation.  All Rights Reserved.
//
// THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF
// ANY KIND, EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// THE IMPLIED WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A
// PARTICULAR PURPOSE.
//=--------------------------------------------------------------------------=

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_homepagesetting_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_homepagesetting_0000_0000_v0_0_s_ifspec;

#ifndef __IHomePageSetting_INTERFACE_DEFINED__
#define __IHomePageSetting_INTERFACE_DEFINED__

/* interface IHomePageSetting */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IHomePageSetting;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("FDFC244F-18FA-4FF2-B08E-1D618F3FFBE4")
    IHomePageSetting : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetHomePage( 
            /* [in] */ HWND hwnd,
            /* [in] */ LPCWSTR homePageUri,
            /* [in] */ LPCWSTR brandingMessage) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsHomePage( 
            /* [in] */ LPCWSTR uri,
            /* [out] */ BOOL *isDefault) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetHomePageToBrowserDefault( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IHomePageSettingVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IHomePageSetting * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IHomePageSetting * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IHomePageSetting * This);
        
        DECLSPEC_XFGVIRT(IHomePageSetting, SetHomePage)
        HRESULT ( STDMETHODCALLTYPE *SetHomePage )( 
            IHomePageSetting * This,
            /* [in] */ HWND hwnd,
            /* [in] */ LPCWSTR homePageUri,
            /* [in] */ LPCWSTR brandingMessage);
        
        DECLSPEC_XFGVIRT(IHomePageSetting, IsHomePage)
        HRESULT ( STDMETHODCALLTYPE *IsHomePage )( 
            IHomePageSetting * This,
            /* [in] */ LPCWSTR uri,
            /* [out] */ BOOL *isDefault);
        
        DECLSPEC_XFGVIRT(IHomePageSetting, SetHomePageToBrowserDefault)
        HRESULT ( STDMETHODCALLTYPE *SetHomePageToBrowserDefault )( 
            IHomePageSetting * This);
        
        END_INTERFACE
    } IHomePageSettingVtbl;

    interface IHomePageSetting
    {
        CONST_VTBL struct IHomePageSettingVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IHomePageSetting_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IHomePageSetting_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IHomePageSetting_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IHomePageSetting_SetHomePage(This,hwnd,homePageUri,brandingMessage)	\
    ( (This)->lpVtbl -> SetHomePage(This,hwnd,homePageUri,brandingMessage) ) 

#define IHomePageSetting_IsHomePage(This,uri,isDefault)	\
    ( (This)->lpVtbl -> IsHomePage(This,uri,isDefault) ) 

#define IHomePageSetting_SetHomePageToBrowserDefault(This)	\
    ( (This)->lpVtbl -> SetHomePageToBrowserDefault(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IHomePageSetting_INTERFACE_DEFINED__ */



#ifndef __IEHomePageSettingObjects_LIBRARY_DEFINED__
#define __IEHomePageSettingObjects_LIBRARY_DEFINED__

/* library IEHomePageSettingObjects */
/* [uuid] */ 


EXTERN_C const IID LIBID_IEHomePageSettingObjects;

EXTERN_C const CLSID CLSID_HomePageSetting;

#ifdef __cplusplus

class DECLSPEC_UUID("374CEDE0-873A-4C4F-BC86-BCC8CF5116A3")
HomePageSetting;
#endif
#endif /* __IEHomePageSettingObjects_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_homepagesetting_0000_0002 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_homepagesetting_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_homepagesetting_0000_0002_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


