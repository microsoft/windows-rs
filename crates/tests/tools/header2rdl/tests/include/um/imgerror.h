

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

#ifndef __imgerror_h__
#define __imgerror_h__

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

#ifndef __IImgErrorInfo_FWD_DEFINED__
#define __IImgErrorInfo_FWD_DEFINED__
typedef interface IImgErrorInfo IImgErrorInfo;

#endif 	/* __IImgErrorInfo_FWD_DEFINED__ */


#ifndef __IImgCreateErrorInfo_FWD_DEFINED__
#define __IImgCreateErrorInfo_FWD_DEFINED__
typedef interface IImgCreateErrorInfo IImgCreateErrorInfo;

#endif 	/* __IImgCreateErrorInfo_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_imgerror_0000_0000 */
/* [local] */ 

//+-------------------------------------------------------------------------
//
//  Microsoft Windows
//  Copyright (c) Microsoft Corporation. All rights reserved.
//
//--------------------------------------------------------------------------
#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


typedef /* [public][public][public] */ struct __MIDL___MIDL_itf_imgerror_0000_0000_0001
    {
    BSTR description;
    GUID guid;
    DWORD helpContext;
    BSTR helpFile;
    BSTR source;
    BSTR devDescription;
    GUID errorID;
    ULONG cUserParameters;
    /* [size_is] */ BSTR *aUserParameters;
    BSTR userFallback;
    DWORD exceptionID;
    } 	ImgErrorInfo;



extern RPC_IF_HANDLE __MIDL_itf_imgerror_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_imgerror_0000_0000_v0_0_s_ifspec;

#ifndef __IImgErrorInfo_INTERFACE_DEFINED__
#define __IImgErrorInfo_INTERFACE_DEFINED__

/* interface IImgErrorInfo */
/* [ref][helpstring][local][uuid][object] */ 


EXTERN_C const IID IID_IImgErrorInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2bce4ece-d30e-445a-9423-6829be945ad8")
    IImgErrorInfo : public IErrorInfo
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetDeveloperDescription( 
            /* [annotation][out] */ 
            _Out_  BSTR *pbstrDevDescription) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetUserErrorId( 
            /* [annotation][out] */ 
            _Out_  GUID *pErrorId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetUserParameterCount( 
            /* [annotation][out] */ 
            _Out_  ULONG *pcUserParams) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetUserParameter( 
            /* [annotation][in] */ 
            _In_  ULONG cParam,
            /* [annotation][out] */ 
            _Out_  BSTR *pbstrParam) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetUserFallback( 
            /* [annotation][out] */ 
            _Out_  BSTR *pbstrFallback) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetExceptionId( 
            /* [annotation][out] */ 
            _Out_  DWORD *pExceptionId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DetachErrorInfo( 
            /* [annotation][out] */ 
            _Out_  ImgErrorInfo *pErrorInfo) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IImgErrorInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IImgErrorInfo * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IImgErrorInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IImgErrorInfo * This);
        
        DECLSPEC_XFGVIRT(IErrorInfo, GetGUID)
        HRESULT ( STDMETHODCALLTYPE *GetGUID )( 
            IImgErrorInfo * This,
            /* [out] */ GUID *pGUID);
        
        DECLSPEC_XFGVIRT(IErrorInfo, GetSource)
        HRESULT ( STDMETHODCALLTYPE *GetSource )( 
            IImgErrorInfo * This,
            /* [out] */ BSTR *pBstrSource);
        
        DECLSPEC_XFGVIRT(IErrorInfo, GetDescription)
        HRESULT ( STDMETHODCALLTYPE *GetDescription )( 
            IImgErrorInfo * This,
            /* [out] */ BSTR *pBstrDescription);
        
        DECLSPEC_XFGVIRT(IErrorInfo, GetHelpFile)
        HRESULT ( STDMETHODCALLTYPE *GetHelpFile )( 
            IImgErrorInfo * This,
            /* [out] */ BSTR *pBstrHelpFile);
        
        DECLSPEC_XFGVIRT(IErrorInfo, GetHelpContext)
        HRESULT ( STDMETHODCALLTYPE *GetHelpContext )( 
            IImgErrorInfo * This,
            /* [out] */ DWORD *pdwHelpContext);
        
        DECLSPEC_XFGVIRT(IImgErrorInfo, GetDeveloperDescription)
        HRESULT ( STDMETHODCALLTYPE *GetDeveloperDescription )( 
            IImgErrorInfo * This,
            /* [annotation][out] */ 
            _Out_  BSTR *pbstrDevDescription);
        
        DECLSPEC_XFGVIRT(IImgErrorInfo, GetUserErrorId)
        HRESULT ( STDMETHODCALLTYPE *GetUserErrorId )( 
            IImgErrorInfo * This,
            /* [annotation][out] */ 
            _Out_  GUID *pErrorId);
        
        DECLSPEC_XFGVIRT(IImgErrorInfo, GetUserParameterCount)
        HRESULT ( STDMETHODCALLTYPE *GetUserParameterCount )( 
            IImgErrorInfo * This,
            /* [annotation][out] */ 
            _Out_  ULONG *pcUserParams);
        
        DECLSPEC_XFGVIRT(IImgErrorInfo, GetUserParameter)
        HRESULT ( STDMETHODCALLTYPE *GetUserParameter )( 
            IImgErrorInfo * This,
            /* [annotation][in] */ 
            _In_  ULONG cParam,
            /* [annotation][out] */ 
            _Out_  BSTR *pbstrParam);
        
        DECLSPEC_XFGVIRT(IImgErrorInfo, GetUserFallback)
        HRESULT ( STDMETHODCALLTYPE *GetUserFallback )( 
            IImgErrorInfo * This,
            /* [annotation][out] */ 
            _Out_  BSTR *pbstrFallback);
        
        DECLSPEC_XFGVIRT(IImgErrorInfo, GetExceptionId)
        HRESULT ( STDMETHODCALLTYPE *GetExceptionId )( 
            IImgErrorInfo * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pExceptionId);
        
        DECLSPEC_XFGVIRT(IImgErrorInfo, DetachErrorInfo)
        HRESULT ( STDMETHODCALLTYPE *DetachErrorInfo )( 
            IImgErrorInfo * This,
            /* [annotation][out] */ 
            _Out_  ImgErrorInfo *pErrorInfo);
        
        END_INTERFACE
    } IImgErrorInfoVtbl;

    interface IImgErrorInfo
    {
        CONST_VTBL struct IImgErrorInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IImgErrorInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IImgErrorInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IImgErrorInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IImgErrorInfo_GetGUID(This,pGUID)	\
    ( (This)->lpVtbl -> GetGUID(This,pGUID) ) 

#define IImgErrorInfo_GetSource(This,pBstrSource)	\
    ( (This)->lpVtbl -> GetSource(This,pBstrSource) ) 

#define IImgErrorInfo_GetDescription(This,pBstrDescription)	\
    ( (This)->lpVtbl -> GetDescription(This,pBstrDescription) ) 

#define IImgErrorInfo_GetHelpFile(This,pBstrHelpFile)	\
    ( (This)->lpVtbl -> GetHelpFile(This,pBstrHelpFile) ) 

#define IImgErrorInfo_GetHelpContext(This,pdwHelpContext)	\
    ( (This)->lpVtbl -> GetHelpContext(This,pdwHelpContext) ) 


#define IImgErrorInfo_GetDeveloperDescription(This,pbstrDevDescription)	\
    ( (This)->lpVtbl -> GetDeveloperDescription(This,pbstrDevDescription) ) 

#define IImgErrorInfo_GetUserErrorId(This,pErrorId)	\
    ( (This)->lpVtbl -> GetUserErrorId(This,pErrorId) ) 

#define IImgErrorInfo_GetUserParameterCount(This,pcUserParams)	\
    ( (This)->lpVtbl -> GetUserParameterCount(This,pcUserParams) ) 

#define IImgErrorInfo_GetUserParameter(This,cParam,pbstrParam)	\
    ( (This)->lpVtbl -> GetUserParameter(This,cParam,pbstrParam) ) 

#define IImgErrorInfo_GetUserFallback(This,pbstrFallback)	\
    ( (This)->lpVtbl -> GetUserFallback(This,pbstrFallback) ) 

#define IImgErrorInfo_GetExceptionId(This,pExceptionId)	\
    ( (This)->lpVtbl -> GetExceptionId(This,pExceptionId) ) 

#define IImgErrorInfo_DetachErrorInfo(This,pErrorInfo)	\
    ( (This)->lpVtbl -> DetachErrorInfo(This,pErrorInfo) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IImgErrorInfo_INTERFACE_DEFINED__ */


#ifndef __IImgCreateErrorInfo_INTERFACE_DEFINED__
#define __IImgCreateErrorInfo_INTERFACE_DEFINED__

/* interface IImgCreateErrorInfo */
/* [ref][helpstring][local][uuid][object] */ 


EXTERN_C const IID IID_IImgCreateErrorInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1c55a64c-07cd-4fb5-90f7-b753d91f0c9e")
    IImgCreateErrorInfo : public ICreateErrorInfo
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AttachToErrorInfo( 
            /* [annotation][out][in] */ 
            _Inout_  ImgErrorInfo *pErrorInfo) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IImgCreateErrorInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IImgCreateErrorInfo * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IImgCreateErrorInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IImgCreateErrorInfo * This);
        
        DECLSPEC_XFGVIRT(ICreateErrorInfo, SetGUID)
        HRESULT ( STDMETHODCALLTYPE *SetGUID )( 
            IImgCreateErrorInfo * This,
            /* [in] */ REFGUID rguid);
        
        DECLSPEC_XFGVIRT(ICreateErrorInfo, SetSource)
        HRESULT ( STDMETHODCALLTYPE *SetSource )( 
            IImgCreateErrorInfo * This,
            /* [in] */ LPOLESTR szSource);
        
        DECLSPEC_XFGVIRT(ICreateErrorInfo, SetDescription)
        HRESULT ( STDMETHODCALLTYPE *SetDescription )( 
            IImgCreateErrorInfo * This,
            /* [in] */ LPOLESTR szDescription);
        
        DECLSPEC_XFGVIRT(ICreateErrorInfo, SetHelpFile)
        HRESULT ( STDMETHODCALLTYPE *SetHelpFile )( 
            IImgCreateErrorInfo * This,
            /* [in] */ LPOLESTR szHelpFile);
        
        DECLSPEC_XFGVIRT(ICreateErrorInfo, SetHelpContext)
        HRESULT ( STDMETHODCALLTYPE *SetHelpContext )( 
            IImgCreateErrorInfo * This,
            /* [in] */ DWORD dwHelpContext);
        
        DECLSPEC_XFGVIRT(IImgCreateErrorInfo, AttachToErrorInfo)
        HRESULT ( STDMETHODCALLTYPE *AttachToErrorInfo )( 
            IImgCreateErrorInfo * This,
            /* [annotation][out][in] */ 
            _Inout_  ImgErrorInfo *pErrorInfo);
        
        END_INTERFACE
    } IImgCreateErrorInfoVtbl;

    interface IImgCreateErrorInfo
    {
        CONST_VTBL struct IImgCreateErrorInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IImgCreateErrorInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IImgCreateErrorInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IImgCreateErrorInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IImgCreateErrorInfo_SetGUID(This,rguid)	\
    ( (This)->lpVtbl -> SetGUID(This,rguid) ) 

#define IImgCreateErrorInfo_SetSource(This,szSource)	\
    ( (This)->lpVtbl -> SetSource(This,szSource) ) 

#define IImgCreateErrorInfo_SetDescription(This,szDescription)	\
    ( (This)->lpVtbl -> SetDescription(This,szDescription) ) 

#define IImgCreateErrorInfo_SetHelpFile(This,szHelpFile)	\
    ( (This)->lpVtbl -> SetHelpFile(This,szHelpFile) ) 

#define IImgCreateErrorInfo_SetHelpContext(This,dwHelpContext)	\
    ( (This)->lpVtbl -> SetHelpContext(This,dwHelpContext) ) 


#define IImgCreateErrorInfo_AttachToErrorInfo(This,pErrorInfo)	\
    ( (This)->lpVtbl -> AttachToErrorInfo(This,pErrorInfo) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IImgCreateErrorInfo_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_imgerror_0000_0002 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_imgerror_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_imgerror_0000_0002_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


