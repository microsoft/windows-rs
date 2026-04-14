

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

#ifndef __documentsource_h__
#define __documentsource_h__

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

#ifndef __IPrintPreviewPageCollection_FWD_DEFINED__
#define __IPrintPreviewPageCollection_FWD_DEFINED__
typedef interface IPrintPreviewPageCollection IPrintPreviewPageCollection;

#endif 	/* __IPrintPreviewPageCollection_FWD_DEFINED__ */


#ifndef __IPrintDocumentPageSource_FWD_DEFINED__
#define __IPrintDocumentPageSource_FWD_DEFINED__
typedef interface IPrintDocumentPageSource IPrintDocumentPageSource;

#endif 	/* __IPrintDocumentPageSource_FWD_DEFINED__ */


#ifndef __IPrintPreviewPageCollection_FWD_DEFINED__
#define __IPrintPreviewPageCollection_FWD_DEFINED__
typedef interface IPrintPreviewPageCollection IPrintPreviewPageCollection;

#endif 	/* __IPrintPreviewPageCollection_FWD_DEFINED__ */


#ifndef __IPrintDocumentPageSource_FWD_DEFINED__
#define __IPrintDocumentPageSource_FWD_DEFINED__
typedef interface IPrintDocumentPageSource IPrintDocumentPageSource;

#endif 	/* __IPrintDocumentPageSource_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"
#include "Inspectable.h"
#include "DocumentTarget.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_documentsource_0000_0000 */
/* [local] */ 

//+-------------------------------------------------------------------------
//
//  Microsoft Windows
//  Copyright (c) Microsoft Corporation. All rights reserved.
//
//--------------------------------------------------------------------------
#if (NTDDI_VERSION >= NTDDI_WIN8)
#define	JOB_PAGE_APPLICATION_DEFINED	( ( UINT32  )-1 )



extern RPC_IF_HANDLE __MIDL_itf_documentsource_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_documentsource_0000_0000_v0_0_s_ifspec;

#ifndef __IPrintPreviewPageCollection_INTERFACE_DEFINED__
#define __IPrintPreviewPageCollection_INTERFACE_DEFINED__

/* interface IPrintPreviewPageCollection */
/* [object][helpstring][uuid] */ 


EXTERN_C const IID IID_IPrintPreviewPageCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0b31cc62-d7ec-4747-9d6e-f2537d870f2b")
    IPrintPreviewPageCollection : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Paginate( 
            /* [in] */ UINT32 currentJobPage,
            /* [in] */ __RPC__in_opt IInspectable *printTaskOptions) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE MakePage( 
            /* [in] */ UINT32 desiredJobPage,
            /* [in] */ FLOAT width,
            /* [in] */ FLOAT height) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPrintPreviewPageCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPrintPreviewPageCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPrintPreviewPageCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPrintPreviewPageCollection * This);
        
        DECLSPEC_XFGVIRT(IPrintPreviewPageCollection, Paginate)
        HRESULT ( STDMETHODCALLTYPE *Paginate )( 
            __RPC__in IPrintPreviewPageCollection * This,
            /* [in] */ UINT32 currentJobPage,
            /* [in] */ __RPC__in_opt IInspectable *printTaskOptions);
        
        DECLSPEC_XFGVIRT(IPrintPreviewPageCollection, MakePage)
        HRESULT ( STDMETHODCALLTYPE *MakePage )( 
            __RPC__in IPrintPreviewPageCollection * This,
            /* [in] */ UINT32 desiredJobPage,
            /* [in] */ FLOAT width,
            /* [in] */ FLOAT height);
        
        END_INTERFACE
    } IPrintPreviewPageCollectionVtbl;

    interface IPrintPreviewPageCollection
    {
        CONST_VTBL struct IPrintPreviewPageCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPrintPreviewPageCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPrintPreviewPageCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPrintPreviewPageCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPrintPreviewPageCollection_Paginate(This,currentJobPage,printTaskOptions)	\
    ( (This)->lpVtbl -> Paginate(This,currentJobPage,printTaskOptions) ) 

#define IPrintPreviewPageCollection_MakePage(This,desiredJobPage,width,height)	\
    ( (This)->lpVtbl -> MakePage(This,desiredJobPage,width,height) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPrintPreviewPageCollection_INTERFACE_DEFINED__ */


#ifndef __IPrintDocumentPageSource_INTERFACE_DEFINED__
#define __IPrintDocumentPageSource_INTERFACE_DEFINED__

/* interface IPrintDocumentPageSource */
/* [object][helpstring][uuid] */ 


EXTERN_C const IID IID_IPrintDocumentPageSource;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("a96bb1db-172e-4667-82b5-ad97a252318f")
    IPrintDocumentPageSource : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetPreviewPageCollection( 
            /* [in] */ __RPC__in_opt IPrintDocumentPackageTarget *docPackageTarget,
            /* [out] */ __RPC__deref_out_opt IPrintPreviewPageCollection **docPageCollection) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE MakeDocument( 
            /* [in] */ __RPC__in_opt IInspectable *printTaskOptions,
            /* [in] */ __RPC__in_opt IPrintDocumentPackageTarget *docPackageTarget) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPrintDocumentPageSourceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPrintDocumentPageSource * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPrintDocumentPageSource * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPrintDocumentPageSource * This);
        
        DECLSPEC_XFGVIRT(IPrintDocumentPageSource, GetPreviewPageCollection)
        HRESULT ( STDMETHODCALLTYPE *GetPreviewPageCollection )( 
            __RPC__in IPrintDocumentPageSource * This,
            /* [in] */ __RPC__in_opt IPrintDocumentPackageTarget *docPackageTarget,
            /* [out] */ __RPC__deref_out_opt IPrintPreviewPageCollection **docPageCollection);
        
        DECLSPEC_XFGVIRT(IPrintDocumentPageSource, MakeDocument)
        HRESULT ( STDMETHODCALLTYPE *MakeDocument )( 
            __RPC__in IPrintDocumentPageSource * This,
            /* [in] */ __RPC__in_opt IInspectable *printTaskOptions,
            /* [in] */ __RPC__in_opt IPrintDocumentPackageTarget *docPackageTarget);
        
        END_INTERFACE
    } IPrintDocumentPageSourceVtbl;

    interface IPrintDocumentPageSource
    {
        CONST_VTBL struct IPrintDocumentPageSourceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPrintDocumentPageSource_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPrintDocumentPageSource_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPrintDocumentPageSource_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPrintDocumentPageSource_GetPreviewPageCollection(This,docPackageTarget,docPageCollection)	\
    ( (This)->lpVtbl -> GetPreviewPageCollection(This,docPackageTarget,docPageCollection) ) 

#define IPrintDocumentPageSource_MakeDocument(This,printTaskOptions,docPackageTarget)	\
    ( (This)->lpVtbl -> MakeDocument(This,printTaskOptions,docPackageTarget) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPrintDocumentPageSource_INTERFACE_DEFINED__ */



#ifndef __PrintDocumentSource_LIBRARY_DEFINED__
#define __PrintDocumentSource_LIBRARY_DEFINED__

/* library PrintDocumentSource */
/* [helpstring][version][uuid] */ 




EXTERN_C const IID LIBID_PrintDocumentSource;
#endif /* __PrintDocumentSource_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_documentsource_0000_0003 */
/* [local] */ 

#endif //(NTDDI_VERSION >= NTDDI_WIN8)


extern RPC_IF_HANDLE __MIDL_itf_documentsource_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_documentsource_0000_0003_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


