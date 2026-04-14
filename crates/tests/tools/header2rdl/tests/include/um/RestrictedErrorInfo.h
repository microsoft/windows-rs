

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

#ifndef __restrictederrorinfo_h__
#define __restrictederrorinfo_h__

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

#ifndef __IRestrictedErrorInfo_FWD_DEFINED__
#define __IRestrictedErrorInfo_FWD_DEFINED__
typedef interface IRestrictedErrorInfo IRestrictedErrorInfo;

#endif 	/* __IRestrictedErrorInfo_FWD_DEFINED__ */


#ifndef __ILanguageExceptionErrorInfo_FWD_DEFINED__
#define __ILanguageExceptionErrorInfo_FWD_DEFINED__
typedef interface ILanguageExceptionErrorInfo ILanguageExceptionErrorInfo;

#endif 	/* __ILanguageExceptionErrorInfo_FWD_DEFINED__ */


#ifndef __ILanguageExceptionTransform_FWD_DEFINED__
#define __ILanguageExceptionTransform_FWD_DEFINED__
typedef interface ILanguageExceptionTransform ILanguageExceptionTransform;

#endif 	/* __ILanguageExceptionTransform_FWD_DEFINED__ */


#ifndef __ILanguageExceptionStackBackTrace_FWD_DEFINED__
#define __ILanguageExceptionStackBackTrace_FWD_DEFINED__
typedef interface ILanguageExceptionStackBackTrace ILanguageExceptionStackBackTrace;

#endif 	/* __ILanguageExceptionStackBackTrace_FWD_DEFINED__ */


#ifndef __ILanguageExceptionErrorInfo2_FWD_DEFINED__
#define __ILanguageExceptionErrorInfo2_FWD_DEFINED__
typedef interface ILanguageExceptionErrorInfo2 ILanguageExceptionErrorInfo2;

#endif 	/* __ILanguageExceptionErrorInfo2_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_restrictederrorinfo_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)


extern RPC_IF_HANDLE __MIDL_itf_restrictederrorinfo_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_restrictederrorinfo_0000_0000_v0_0_s_ifspec;

#ifndef __IRestrictedErrorInfo_INTERFACE_DEFINED__
#define __IRestrictedErrorInfo_INTERFACE_DEFINED__

/* interface IRestrictedErrorInfo */
/* [object][unique][uuid] */ 


EXTERN_C const IID IID_IRestrictedErrorInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("82BA7092-4C88-427D-A7BC-16DD93FEB67E")
    IRestrictedErrorInfo : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetErrorDetails( 
            /* [out] */ __RPC__deref_out_opt BSTR *description,
            /* [out] */ __RPC__out HRESULT *error,
            /* [out] */ __RPC__deref_out_opt BSTR *restrictedDescription,
            /* [out] */ __RPC__deref_out_opt BSTR *capabilitySid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetReference( 
            /* [out] */ __RPC__deref_out_opt BSTR *reference) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRestrictedErrorInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRestrictedErrorInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRestrictedErrorInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRestrictedErrorInfo * This);
        
        DECLSPEC_XFGVIRT(IRestrictedErrorInfo, GetErrorDetails)
        HRESULT ( STDMETHODCALLTYPE *GetErrorDetails )( 
            __RPC__in IRestrictedErrorInfo * This,
            /* [out] */ __RPC__deref_out_opt BSTR *description,
            /* [out] */ __RPC__out HRESULT *error,
            /* [out] */ __RPC__deref_out_opt BSTR *restrictedDescription,
            /* [out] */ __RPC__deref_out_opt BSTR *capabilitySid);
        
        DECLSPEC_XFGVIRT(IRestrictedErrorInfo, GetReference)
        HRESULT ( STDMETHODCALLTYPE *GetReference )( 
            __RPC__in IRestrictedErrorInfo * This,
            /* [out] */ __RPC__deref_out_opt BSTR *reference);
        
        END_INTERFACE
    } IRestrictedErrorInfoVtbl;

    interface IRestrictedErrorInfo
    {
        CONST_VTBL struct IRestrictedErrorInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRestrictedErrorInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRestrictedErrorInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRestrictedErrorInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRestrictedErrorInfo_GetErrorDetails(This,description,error,restrictedDescription,capabilitySid)	\
    ( (This)->lpVtbl -> GetErrorDetails(This,description,error,restrictedDescription,capabilitySid) ) 

#define IRestrictedErrorInfo_GetReference(This,reference)	\
    ( (This)->lpVtbl -> GetReference(This,reference) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRestrictedErrorInfo_INTERFACE_DEFINED__ */


#ifndef __ILanguageExceptionErrorInfo_INTERFACE_DEFINED__
#define __ILanguageExceptionErrorInfo_INTERFACE_DEFINED__

/* interface ILanguageExceptionErrorInfo */
/* [object][unique][uuid] */ 


EXTERN_C const IID IID_ILanguageExceptionErrorInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("04a2dbf3-df83-116c-0946-0812abf6e07d")
    ILanguageExceptionErrorInfo : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetLanguageException( 
            /* [out] */ __RPC__deref_out_opt IUnknown **languageException) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ILanguageExceptionErrorInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ILanguageExceptionErrorInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ILanguageExceptionErrorInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ILanguageExceptionErrorInfo * This);
        
        DECLSPEC_XFGVIRT(ILanguageExceptionErrorInfo, GetLanguageException)
        HRESULT ( STDMETHODCALLTYPE *GetLanguageException )( 
            __RPC__in ILanguageExceptionErrorInfo * This,
            /* [out] */ __RPC__deref_out_opt IUnknown **languageException);
        
        END_INTERFACE
    } ILanguageExceptionErrorInfoVtbl;

    interface ILanguageExceptionErrorInfo
    {
        CONST_VTBL struct ILanguageExceptionErrorInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ILanguageExceptionErrorInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ILanguageExceptionErrorInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ILanguageExceptionErrorInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ILanguageExceptionErrorInfo_GetLanguageException(This,languageException)	\
    ( (This)->lpVtbl -> GetLanguageException(This,languageException) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ILanguageExceptionErrorInfo_INTERFACE_DEFINED__ */


#ifndef __ILanguageExceptionTransform_INTERFACE_DEFINED__
#define __ILanguageExceptionTransform_INTERFACE_DEFINED__

/* interface ILanguageExceptionTransform */
/* [object][unique][uuid] */ 


EXTERN_C const IID IID_ILanguageExceptionTransform;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("FEB5A271-A6CD-45CE-880A-696706BADC65")
    ILanguageExceptionTransform : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetTransformedRestrictedErrorInfo( 
            /* [out] */ __RPC__deref_out_opt IRestrictedErrorInfo **restrictedErrorInfo) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ILanguageExceptionTransformVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ILanguageExceptionTransform * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ILanguageExceptionTransform * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ILanguageExceptionTransform * This);
        
        DECLSPEC_XFGVIRT(ILanguageExceptionTransform, GetTransformedRestrictedErrorInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTransformedRestrictedErrorInfo )( 
            __RPC__in ILanguageExceptionTransform * This,
            /* [out] */ __RPC__deref_out_opt IRestrictedErrorInfo **restrictedErrorInfo);
        
        END_INTERFACE
    } ILanguageExceptionTransformVtbl;

    interface ILanguageExceptionTransform
    {
        CONST_VTBL struct ILanguageExceptionTransformVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ILanguageExceptionTransform_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ILanguageExceptionTransform_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ILanguageExceptionTransform_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ILanguageExceptionTransform_GetTransformedRestrictedErrorInfo(This,restrictedErrorInfo)	\
    ( (This)->lpVtbl -> GetTransformedRestrictedErrorInfo(This,restrictedErrorInfo) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ILanguageExceptionTransform_INTERFACE_DEFINED__ */


#ifndef __ILanguageExceptionStackBackTrace_INTERFACE_DEFINED__
#define __ILanguageExceptionStackBackTrace_INTERFACE_DEFINED__

/* interface ILanguageExceptionStackBackTrace */
/* [object][unique][uuid] */ 


EXTERN_C const IID IID_ILanguageExceptionStackBackTrace;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("CBE53FB5-F967-4258-8D34-42F5E25833DE")
    ILanguageExceptionStackBackTrace : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetStackBackTrace( 
            /* [in] */ ULONG maxFramesToCapture,
            /* [max_is][out][in] */ __RPC__inout_ecount_full(( ( maxFramesToCapture - 1 )  + 1 ) ) UINT_PTR stackBackTrace[  ],
            /* [out] */ __RPC__out ULONG *framesCaptured) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ILanguageExceptionStackBackTraceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ILanguageExceptionStackBackTrace * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ILanguageExceptionStackBackTrace * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ILanguageExceptionStackBackTrace * This);
        
        DECLSPEC_XFGVIRT(ILanguageExceptionStackBackTrace, GetStackBackTrace)
        HRESULT ( STDMETHODCALLTYPE *GetStackBackTrace )( 
            __RPC__in ILanguageExceptionStackBackTrace * This,
            /* [in] */ ULONG maxFramesToCapture,
            /* [max_is][out][in] */ __RPC__inout_ecount_full(( ( maxFramesToCapture - 1 )  + 1 ) ) UINT_PTR stackBackTrace[  ],
            /* [out] */ __RPC__out ULONG *framesCaptured);
        
        END_INTERFACE
    } ILanguageExceptionStackBackTraceVtbl;

    interface ILanguageExceptionStackBackTrace
    {
        CONST_VTBL struct ILanguageExceptionStackBackTraceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ILanguageExceptionStackBackTrace_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ILanguageExceptionStackBackTrace_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ILanguageExceptionStackBackTrace_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ILanguageExceptionStackBackTrace_GetStackBackTrace(This,maxFramesToCapture,stackBackTrace,framesCaptured)	\
    ( (This)->lpVtbl -> GetStackBackTrace(This,maxFramesToCapture,stackBackTrace,framesCaptured) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ILanguageExceptionStackBackTrace_INTERFACE_DEFINED__ */


#ifndef __ILanguageExceptionErrorInfo2_INTERFACE_DEFINED__
#define __ILanguageExceptionErrorInfo2_INTERFACE_DEFINED__

/* interface ILanguageExceptionErrorInfo2 */
/* [object][unique][uuid] */ 


EXTERN_C const IID IID_ILanguageExceptionErrorInfo2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5746E5C4-5B97-424C-B620-2822915734DD")
    ILanguageExceptionErrorInfo2 : public ILanguageExceptionErrorInfo
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetPreviousLanguageExceptionErrorInfo( 
            /* [out] */ __RPC__deref_out_opt ILanguageExceptionErrorInfo2 **previousLanguageExceptionErrorInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CapturePropagationContext( 
            /* [in] */ __RPC__in_opt IUnknown *languageException) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPropagationContextHead( 
            /* [out] */ __RPC__deref_out_opt ILanguageExceptionErrorInfo2 **propagatedLanguageExceptionErrorInfoHead) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ILanguageExceptionErrorInfo2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ILanguageExceptionErrorInfo2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ILanguageExceptionErrorInfo2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ILanguageExceptionErrorInfo2 * This);
        
        DECLSPEC_XFGVIRT(ILanguageExceptionErrorInfo, GetLanguageException)
        HRESULT ( STDMETHODCALLTYPE *GetLanguageException )( 
            __RPC__in ILanguageExceptionErrorInfo2 * This,
            /* [out] */ __RPC__deref_out_opt IUnknown **languageException);
        
        DECLSPEC_XFGVIRT(ILanguageExceptionErrorInfo2, GetPreviousLanguageExceptionErrorInfo)
        HRESULT ( STDMETHODCALLTYPE *GetPreviousLanguageExceptionErrorInfo )( 
            __RPC__in ILanguageExceptionErrorInfo2 * This,
            /* [out] */ __RPC__deref_out_opt ILanguageExceptionErrorInfo2 **previousLanguageExceptionErrorInfo);
        
        DECLSPEC_XFGVIRT(ILanguageExceptionErrorInfo2, CapturePropagationContext)
        HRESULT ( STDMETHODCALLTYPE *CapturePropagationContext )( 
            __RPC__in ILanguageExceptionErrorInfo2 * This,
            /* [in] */ __RPC__in_opt IUnknown *languageException);
        
        DECLSPEC_XFGVIRT(ILanguageExceptionErrorInfo2, GetPropagationContextHead)
        HRESULT ( STDMETHODCALLTYPE *GetPropagationContextHead )( 
            __RPC__in ILanguageExceptionErrorInfo2 * This,
            /* [out] */ __RPC__deref_out_opt ILanguageExceptionErrorInfo2 **propagatedLanguageExceptionErrorInfoHead);
        
        END_INTERFACE
    } ILanguageExceptionErrorInfo2Vtbl;

    interface ILanguageExceptionErrorInfo2
    {
        CONST_VTBL struct ILanguageExceptionErrorInfo2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ILanguageExceptionErrorInfo2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ILanguageExceptionErrorInfo2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ILanguageExceptionErrorInfo2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ILanguageExceptionErrorInfo2_GetLanguageException(This,languageException)	\
    ( (This)->lpVtbl -> GetLanguageException(This,languageException) ) 


#define ILanguageExceptionErrorInfo2_GetPreviousLanguageExceptionErrorInfo(This,previousLanguageExceptionErrorInfo)	\
    ( (This)->lpVtbl -> GetPreviousLanguageExceptionErrorInfo(This,previousLanguageExceptionErrorInfo) ) 

#define ILanguageExceptionErrorInfo2_CapturePropagationContext(This,languageException)	\
    ( (This)->lpVtbl -> CapturePropagationContext(This,languageException) ) 

#define ILanguageExceptionErrorInfo2_GetPropagationContextHead(This,propagatedLanguageExceptionErrorInfoHead)	\
    ( (This)->lpVtbl -> GetPropagationContextHead(This,propagatedLanguageExceptionErrorInfoHead) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ILanguageExceptionErrorInfo2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_restrictederrorinfo_0000_0005 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_restrictederrorinfo_0000_0005_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_restrictederrorinfo_0000_0005_v0_0_s_ifspec;

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


