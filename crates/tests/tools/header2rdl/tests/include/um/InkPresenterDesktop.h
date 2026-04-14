

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

#ifndef __inkpresenterdesktop_h__
#define __inkpresenterdesktop_h__

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

#ifndef __IInkCommitRequestHandler_FWD_DEFINED__
#define __IInkCommitRequestHandler_FWD_DEFINED__
typedef interface IInkCommitRequestHandler IInkCommitRequestHandler;

#endif 	/* __IInkCommitRequestHandler_FWD_DEFINED__ */


#ifndef __IInkPresenterDesktop_FWD_DEFINED__
#define __IInkPresenterDesktop_FWD_DEFINED__
typedef interface IInkPresenterDesktop IInkPresenterDesktop;

#endif 	/* __IInkPresenterDesktop_FWD_DEFINED__ */


#ifndef __IInkHostWorkItem_FWD_DEFINED__
#define __IInkHostWorkItem_FWD_DEFINED__
typedef interface IInkHostWorkItem IInkHostWorkItem;

#endif 	/* __IInkHostWorkItem_FWD_DEFINED__ */


#ifndef __IInkDesktopHost_FWD_DEFINED__
#define __IInkDesktopHost_FWD_DEFINED__
typedef interface IInkDesktopHost IInkDesktopHost;

#endif 	/* __IInkDesktopHost_FWD_DEFINED__ */


#ifndef __InkDesktopHost_FWD_DEFINED__
#define __InkDesktopHost_FWD_DEFINED__

#ifdef __cplusplus
typedef class InkDesktopHost InkDesktopHost;
#else
typedef struct InkDesktopHost InkDesktopHost;
#endif /* __cplusplus */

#endif 	/* __InkDesktopHost_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"
#include "inspectable.h"
#include "weakreference.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_inkpresenterdesktop_0000_0000 */
/* [local] */ 

#pragma once
#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)


extern RPC_IF_HANDLE __MIDL_itf_inkpresenterdesktop_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_inkpresenterdesktop_0000_0000_v0_0_s_ifspec;

#ifndef __IInkCommitRequestHandler_INTERFACE_DEFINED__
#define __IInkCommitRequestHandler_INTERFACE_DEFINED__

/* interface IInkCommitRequestHandler */
/* [uuid][object] */ 


EXTERN_C const IID IID_IInkCommitRequestHandler;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("fabea3fc-b108-45b6-a9fc-8d08fa9f85cf")
    IInkCommitRequestHandler : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnCommitRequested( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInkCommitRequestHandlerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IInkCommitRequestHandler * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IInkCommitRequestHandler * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IInkCommitRequestHandler * This);
        
        DECLSPEC_XFGVIRT(IInkCommitRequestHandler, OnCommitRequested)
        HRESULT ( STDMETHODCALLTYPE *OnCommitRequested )( 
            __RPC__in IInkCommitRequestHandler * This);
        
        END_INTERFACE
    } IInkCommitRequestHandlerVtbl;

    interface IInkCommitRequestHandler
    {
        CONST_VTBL struct IInkCommitRequestHandlerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInkCommitRequestHandler_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInkCommitRequestHandler_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInkCommitRequestHandler_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInkCommitRequestHandler_OnCommitRequested(This)	\
    ( (This)->lpVtbl -> OnCommitRequested(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInkCommitRequestHandler_INTERFACE_DEFINED__ */


#ifndef __IInkPresenterDesktop_INTERFACE_DEFINED__
#define __IInkPresenterDesktop_INTERFACE_DEFINED__

/* interface IInkPresenterDesktop */
/* [uuid][object] */ 


EXTERN_C const IID IID_IInkPresenterDesktop;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("73f3c0d9-2e8b-48f3-895e-20cbd27b723b")
    IInkPresenterDesktop : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetRootVisual( 
            /* [in] */ __RPC__in_opt IUnknown *rootVisual,
            /* [unique][in] */ __RPC__in_opt IUnknown *device) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetCommitRequestHandler( 
            /* [in] */ __RPC__in_opt IInkCommitRequestHandler *handler) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSize( 
            /* [out] */ __RPC__out float *width,
            /* [out] */ __RPC__out float *height) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetSize( 
            /* [in] */ float width,
            /* [in] */ float height) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnHighContrastChanged( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInkPresenterDesktopVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IInkPresenterDesktop * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IInkPresenterDesktop * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IInkPresenterDesktop * This);
        
        DECLSPEC_XFGVIRT(IInkPresenterDesktop, SetRootVisual)
        HRESULT ( STDMETHODCALLTYPE *SetRootVisual )( 
            __RPC__in IInkPresenterDesktop * This,
            /* [in] */ __RPC__in_opt IUnknown *rootVisual,
            /* [unique][in] */ __RPC__in_opt IUnknown *device);
        
        DECLSPEC_XFGVIRT(IInkPresenterDesktop, SetCommitRequestHandler)
        HRESULT ( STDMETHODCALLTYPE *SetCommitRequestHandler )( 
            __RPC__in IInkPresenterDesktop * This,
            /* [in] */ __RPC__in_opt IInkCommitRequestHandler *handler);
        
        DECLSPEC_XFGVIRT(IInkPresenterDesktop, GetSize)
        HRESULT ( STDMETHODCALLTYPE *GetSize )( 
            __RPC__in IInkPresenterDesktop * This,
            /* [out] */ __RPC__out float *width,
            /* [out] */ __RPC__out float *height);
        
        DECLSPEC_XFGVIRT(IInkPresenterDesktop, SetSize)
        HRESULT ( STDMETHODCALLTYPE *SetSize )( 
            __RPC__in IInkPresenterDesktop * This,
            /* [in] */ float width,
            /* [in] */ float height);
        
        DECLSPEC_XFGVIRT(IInkPresenterDesktop, OnHighContrastChanged)
        HRESULT ( STDMETHODCALLTYPE *OnHighContrastChanged )( 
            __RPC__in IInkPresenterDesktop * This);
        
        END_INTERFACE
    } IInkPresenterDesktopVtbl;

    interface IInkPresenterDesktop
    {
        CONST_VTBL struct IInkPresenterDesktopVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInkPresenterDesktop_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInkPresenterDesktop_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInkPresenterDesktop_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInkPresenterDesktop_SetRootVisual(This,rootVisual,device)	\
    ( (This)->lpVtbl -> SetRootVisual(This,rootVisual,device) ) 

#define IInkPresenterDesktop_SetCommitRequestHandler(This,handler)	\
    ( (This)->lpVtbl -> SetCommitRequestHandler(This,handler) ) 

#define IInkPresenterDesktop_GetSize(This,width,height)	\
    ( (This)->lpVtbl -> GetSize(This,width,height) ) 

#define IInkPresenterDesktop_SetSize(This,width,height)	\
    ( (This)->lpVtbl -> SetSize(This,width,height) ) 

#define IInkPresenterDesktop_OnHighContrastChanged(This)	\
    ( (This)->lpVtbl -> OnHighContrastChanged(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInkPresenterDesktop_INTERFACE_DEFINED__ */


#ifndef __IInkHostWorkItem_INTERFACE_DEFINED__
#define __IInkHostWorkItem_INTERFACE_DEFINED__

/* interface IInkHostWorkItem */
/* [uuid][object] */ 


EXTERN_C const IID IID_IInkHostWorkItem;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("ccda0a9a-1b78-4632-bb96-97800662e26c")
    IInkHostWorkItem : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Invoke( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInkHostWorkItemVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IInkHostWorkItem * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IInkHostWorkItem * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IInkHostWorkItem * This);
        
        DECLSPEC_XFGVIRT(IInkHostWorkItem, Invoke)
        HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            __RPC__in IInkHostWorkItem * This);
        
        END_INTERFACE
    } IInkHostWorkItemVtbl;

    interface IInkHostWorkItem
    {
        CONST_VTBL struct IInkHostWorkItemVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInkHostWorkItem_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInkHostWorkItem_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInkHostWorkItem_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInkHostWorkItem_Invoke(This)	\
    ( (This)->lpVtbl -> Invoke(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInkHostWorkItem_INTERFACE_DEFINED__ */


#ifndef __IInkDesktopHost_INTERFACE_DEFINED__
#define __IInkDesktopHost_INTERFACE_DEFINED__

/* interface IInkDesktopHost */
/* [uuid][object] */ 


EXTERN_C const IID IID_IInkDesktopHost;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4ce7d875-a981-4140-a1ff-ad93258e8d59")
    IInkDesktopHost : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE QueueWorkItem( 
            /* [in] */ __RPC__in_opt IInkHostWorkItem *workItem) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateInkPresenter( 
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateAndInitializeInkPresenter( 
            /* [in] */ __RPC__in_opt IUnknown *rootVisual,
            /* [in] */ float width,
            /* [in] */ float height,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInkDesktopHostVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IInkDesktopHost * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IInkDesktopHost * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IInkDesktopHost * This);
        
        DECLSPEC_XFGVIRT(IInkDesktopHost, QueueWorkItem)
        HRESULT ( STDMETHODCALLTYPE *QueueWorkItem )( 
            __RPC__in IInkDesktopHost * This,
            /* [in] */ __RPC__in_opt IInkHostWorkItem *workItem);
        
        DECLSPEC_XFGVIRT(IInkDesktopHost, CreateInkPresenter)
        HRESULT ( STDMETHODCALLTYPE *CreateInkPresenter )( 
            __RPC__in IInkDesktopHost * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv);
        
        DECLSPEC_XFGVIRT(IInkDesktopHost, CreateAndInitializeInkPresenter)
        HRESULT ( STDMETHODCALLTYPE *CreateAndInitializeInkPresenter )( 
            __RPC__in IInkDesktopHost * This,
            /* [in] */ __RPC__in_opt IUnknown *rootVisual,
            /* [in] */ float width,
            /* [in] */ float height,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv);
        
        END_INTERFACE
    } IInkDesktopHostVtbl;

    interface IInkDesktopHost
    {
        CONST_VTBL struct IInkDesktopHostVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInkDesktopHost_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInkDesktopHost_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInkDesktopHost_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInkDesktopHost_QueueWorkItem(This,workItem)	\
    ( (This)->lpVtbl -> QueueWorkItem(This,workItem) ) 

#define IInkDesktopHost_CreateInkPresenter(This,riid,ppv)	\
    ( (This)->lpVtbl -> CreateInkPresenter(This,riid,ppv) ) 

#define IInkDesktopHost_CreateAndInitializeInkPresenter(This,rootVisual,width,height,riid,ppv)	\
    ( (This)->lpVtbl -> CreateAndInitializeInkPresenter(This,rootVisual,width,height,riid,ppv) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInkDesktopHost_INTERFACE_DEFINED__ */



#ifndef __InkDesktopHostLib_LIBRARY_DEFINED__
#define __InkDesktopHostLib_LIBRARY_DEFINED__

/* library InkDesktopHostLib */
/* [uuid] */ 


EXTERN_C const IID LIBID_InkDesktopHostLib;

EXTERN_C const CLSID CLSID_InkDesktopHost;

#ifdef __cplusplus

class DECLSPEC_UUID("062584a6-f830-4bdc-a4d2-0a10ab062b1d")
InkDesktopHost;
#endif
#endif /* __InkDesktopHostLib_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_inkpresenterdesktop_0000_0005 */
/* [local] */ 

#endif // NTDDI_VERSION >= NTDDI_WINTHRESHOLD
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_inkpresenterdesktop_0000_0005_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_inkpresenterdesktop_0000_0005_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


