

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

#ifndef __efswrtinterop_h__
#define __efswrtinterop_h__

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

#ifndef __IProtectionPolicyManagerInterop_FWD_DEFINED__
#define __IProtectionPolicyManagerInterop_FWD_DEFINED__
typedef interface IProtectionPolicyManagerInterop IProtectionPolicyManagerInterop;

#endif 	/* __IProtectionPolicyManagerInterop_FWD_DEFINED__ */


#ifndef __IProtectionPolicyManagerInterop2_FWD_DEFINED__
#define __IProtectionPolicyManagerInterop2_FWD_DEFINED__
typedef interface IProtectionPolicyManagerInterop2 IProtectionPolicyManagerInterop2;

#endif 	/* __IProtectionPolicyManagerInterop2_FWD_DEFINED__ */


#ifndef __IProtectionPolicyManagerInterop3_FWD_DEFINED__
#define __IProtectionPolicyManagerInterop3_FWD_DEFINED__
typedef interface IProtectionPolicyManagerInterop3 IProtectionPolicyManagerInterop3;

#endif 	/* __IProtectionPolicyManagerInterop3_FWD_DEFINED__ */


/* header files for imported files */
#include "inspectable.h"
#include "asyncinfo.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_efswrtinterop_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_efswrtinterop_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_efswrtinterop_0000_0000_v0_0_s_ifspec;

#ifndef __IProtectionPolicyManagerInterop_INTERFACE_DEFINED__
#define __IProtectionPolicyManagerInterop_INTERFACE_DEFINED__

/* interface IProtectionPolicyManagerInterop */
/* [object][uuid] */ 


EXTERN_C const IID IID_IProtectionPolicyManagerInterop;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4652651d-c1fe-4ba1-9F0a-c0f56596f721")
    IProtectionPolicyManagerInterop : public IInspectable
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE RequestAccessForWindowAsync( 
            /* [in] */ __RPC__in HWND appWindow,
            /* [in] */ __RPC__in HSTRING sourceIdentity,
            /* [in] */ __RPC__in HSTRING targetIdentity,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][retval][out] */ __RPC__deref_out_opt void **asyncOperation) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetForWindow( 
            /* [in] */ __RPC__in HWND appWindow,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][retval][out] */ __RPC__deref_out_opt void **result) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IProtectionPolicyManagerInteropVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IProtectionPolicyManagerInterop * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IProtectionPolicyManagerInterop * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IProtectionPolicyManagerInterop * This);
        
        DECLSPEC_XFGVIRT(IInspectable, GetIids)
        HRESULT ( STDMETHODCALLTYPE *GetIids )( 
            __RPC__in IProtectionPolicyManagerInterop * This,
            /* [out] */ __RPC__out ULONG *iidCount,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids);
        
        DECLSPEC_XFGVIRT(IInspectable, GetRuntimeClassName)
        HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )( 
            __RPC__in IProtectionPolicyManagerInterop * This,
            /* [out] */ __RPC__deref_out_opt HSTRING *className);
        
        DECLSPEC_XFGVIRT(IInspectable, GetTrustLevel)
        HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )( 
            __RPC__in IProtectionPolicyManagerInterop * This,
            /* [out] */ __RPC__out TrustLevel *trustLevel);
        
        DECLSPEC_XFGVIRT(IProtectionPolicyManagerInterop, RequestAccessForWindowAsync)
        HRESULT ( STDMETHODCALLTYPE *RequestAccessForWindowAsync )( 
            __RPC__in IProtectionPolicyManagerInterop * This,
            /* [in] */ __RPC__in HWND appWindow,
            /* [in] */ __RPC__in HSTRING sourceIdentity,
            /* [in] */ __RPC__in HSTRING targetIdentity,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][retval][out] */ __RPC__deref_out_opt void **asyncOperation);
        
        DECLSPEC_XFGVIRT(IProtectionPolicyManagerInterop, GetForWindow)
        HRESULT ( STDMETHODCALLTYPE *GetForWindow )( 
            __RPC__in IProtectionPolicyManagerInterop * This,
            /* [in] */ __RPC__in HWND appWindow,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][retval][out] */ __RPC__deref_out_opt void **result);
        
        END_INTERFACE
    } IProtectionPolicyManagerInteropVtbl;

    interface IProtectionPolicyManagerInterop
    {
        CONST_VTBL struct IProtectionPolicyManagerInteropVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IProtectionPolicyManagerInterop_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IProtectionPolicyManagerInterop_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IProtectionPolicyManagerInterop_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IProtectionPolicyManagerInterop_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 

#define IProtectionPolicyManagerInterop_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 

#define IProtectionPolicyManagerInterop_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 


#define IProtectionPolicyManagerInterop_RequestAccessForWindowAsync(This,appWindow,sourceIdentity,targetIdentity,riid,asyncOperation)	\
    ( (This)->lpVtbl -> RequestAccessForWindowAsync(This,appWindow,sourceIdentity,targetIdentity,riid,asyncOperation) ) 

#define IProtectionPolicyManagerInterop_GetForWindow(This,appWindow,riid,result)	\
    ( (This)->lpVtbl -> GetForWindow(This,appWindow,riid,result) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IProtectionPolicyManagerInterop_INTERFACE_DEFINED__ */


#ifndef __IProtectionPolicyManagerInterop2_INTERFACE_DEFINED__
#define __IProtectionPolicyManagerInterop2_INTERFACE_DEFINED__

/* interface IProtectionPolicyManagerInterop2 */
/* [object][uuid] */ 


EXTERN_C const IID IID_IProtectionPolicyManagerInterop2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("157cfbe4-a78d-4156-b384-61fdac41e686")
    IProtectionPolicyManagerInterop2 : public IInspectable
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE RequestAccessForAppWithWindowAsync( 
            /* [in] */ __RPC__in HWND appWindow,
            /* [in] */ __RPC__in HSTRING sourceIdentity,
            /* [in] */ __RPC__in HSTRING appPackageFamilyName,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][retval][out] */ __RPC__deref_out_opt void **asyncOperation) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RequestAccessWithAuditingInfoForWindowAsync( 
            /* [in] */ __RPC__in HWND appWindow,
            /* [in] */ __RPC__in HSTRING sourceIdentity,
            /* [in] */ __RPC__in HSTRING targetIdentity,
            /* [in] */ __RPC__in_opt IUnknown *auditInfoUnk,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][retval][out] */ __RPC__deref_out_opt void **asyncOperation) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RequestAccessWithMessageForWindowAsync( 
            /* [in] */ __RPC__in HWND appWindow,
            /* [in] */ __RPC__in HSTRING sourceIdentity,
            /* [in] */ __RPC__in HSTRING targetIdentity,
            /* [in] */ __RPC__in_opt IUnknown *auditInfoUnk,
            /* [in] */ __RPC__in HSTRING messageFromApp,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][retval][out] */ __RPC__deref_out_opt void **asyncOperation) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RequestAccessForAppWithAuditingInfoForWindowAsync( 
            /* [in] */ __RPC__in HWND appWindow,
            /* [in] */ __RPC__in HSTRING sourceIdentity,
            /* [in] */ __RPC__in HSTRING appPackageFamilyName,
            /* [in] */ __RPC__in_opt IUnknown *auditInfoUnk,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][retval][out] */ __RPC__deref_out_opt void **asyncOperation) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RequestAccessForAppWithMessageForWindowAsync( 
            /* [in] */ __RPC__in HWND appWindow,
            /* [in] */ __RPC__in HSTRING sourceIdentity,
            /* [in] */ __RPC__in HSTRING appPackageFamilyName,
            /* [in] */ __RPC__in_opt IUnknown *auditInfoUnk,
            /* [in] */ __RPC__in HSTRING messageFromApp,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][retval][out] */ __RPC__deref_out_opt void **asyncOperation) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IProtectionPolicyManagerInterop2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IProtectionPolicyManagerInterop2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IProtectionPolicyManagerInterop2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IProtectionPolicyManagerInterop2 * This);
        
        DECLSPEC_XFGVIRT(IInspectable, GetIids)
        HRESULT ( STDMETHODCALLTYPE *GetIids )( 
            __RPC__in IProtectionPolicyManagerInterop2 * This,
            /* [out] */ __RPC__out ULONG *iidCount,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids);
        
        DECLSPEC_XFGVIRT(IInspectable, GetRuntimeClassName)
        HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )( 
            __RPC__in IProtectionPolicyManagerInterop2 * This,
            /* [out] */ __RPC__deref_out_opt HSTRING *className);
        
        DECLSPEC_XFGVIRT(IInspectable, GetTrustLevel)
        HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )( 
            __RPC__in IProtectionPolicyManagerInterop2 * This,
            /* [out] */ __RPC__out TrustLevel *trustLevel);
        
        DECLSPEC_XFGVIRT(IProtectionPolicyManagerInterop2, RequestAccessForAppWithWindowAsync)
        HRESULT ( STDMETHODCALLTYPE *RequestAccessForAppWithWindowAsync )( 
            __RPC__in IProtectionPolicyManagerInterop2 * This,
            /* [in] */ __RPC__in HWND appWindow,
            /* [in] */ __RPC__in HSTRING sourceIdentity,
            /* [in] */ __RPC__in HSTRING appPackageFamilyName,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][retval][out] */ __RPC__deref_out_opt void **asyncOperation);
        
        DECLSPEC_XFGVIRT(IProtectionPolicyManagerInterop2, RequestAccessWithAuditingInfoForWindowAsync)
        HRESULT ( STDMETHODCALLTYPE *RequestAccessWithAuditingInfoForWindowAsync )( 
            __RPC__in IProtectionPolicyManagerInterop2 * This,
            /* [in] */ __RPC__in HWND appWindow,
            /* [in] */ __RPC__in HSTRING sourceIdentity,
            /* [in] */ __RPC__in HSTRING targetIdentity,
            /* [in] */ __RPC__in_opt IUnknown *auditInfoUnk,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][retval][out] */ __RPC__deref_out_opt void **asyncOperation);
        
        DECLSPEC_XFGVIRT(IProtectionPolicyManagerInterop2, RequestAccessWithMessageForWindowAsync)
        HRESULT ( STDMETHODCALLTYPE *RequestAccessWithMessageForWindowAsync )( 
            __RPC__in IProtectionPolicyManagerInterop2 * This,
            /* [in] */ __RPC__in HWND appWindow,
            /* [in] */ __RPC__in HSTRING sourceIdentity,
            /* [in] */ __RPC__in HSTRING targetIdentity,
            /* [in] */ __RPC__in_opt IUnknown *auditInfoUnk,
            /* [in] */ __RPC__in HSTRING messageFromApp,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][retval][out] */ __RPC__deref_out_opt void **asyncOperation);
        
        DECLSPEC_XFGVIRT(IProtectionPolicyManagerInterop2, RequestAccessForAppWithAuditingInfoForWindowAsync)
        HRESULT ( STDMETHODCALLTYPE *RequestAccessForAppWithAuditingInfoForWindowAsync )( 
            __RPC__in IProtectionPolicyManagerInterop2 * This,
            /* [in] */ __RPC__in HWND appWindow,
            /* [in] */ __RPC__in HSTRING sourceIdentity,
            /* [in] */ __RPC__in HSTRING appPackageFamilyName,
            /* [in] */ __RPC__in_opt IUnknown *auditInfoUnk,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][retval][out] */ __RPC__deref_out_opt void **asyncOperation);
        
        DECLSPEC_XFGVIRT(IProtectionPolicyManagerInterop2, RequestAccessForAppWithMessageForWindowAsync)
        HRESULT ( STDMETHODCALLTYPE *RequestAccessForAppWithMessageForWindowAsync )( 
            __RPC__in IProtectionPolicyManagerInterop2 * This,
            /* [in] */ __RPC__in HWND appWindow,
            /* [in] */ __RPC__in HSTRING sourceIdentity,
            /* [in] */ __RPC__in HSTRING appPackageFamilyName,
            /* [in] */ __RPC__in_opt IUnknown *auditInfoUnk,
            /* [in] */ __RPC__in HSTRING messageFromApp,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][retval][out] */ __RPC__deref_out_opt void **asyncOperation);
        
        END_INTERFACE
    } IProtectionPolicyManagerInterop2Vtbl;

    interface IProtectionPolicyManagerInterop2
    {
        CONST_VTBL struct IProtectionPolicyManagerInterop2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IProtectionPolicyManagerInterop2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IProtectionPolicyManagerInterop2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IProtectionPolicyManagerInterop2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IProtectionPolicyManagerInterop2_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 

#define IProtectionPolicyManagerInterop2_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 

#define IProtectionPolicyManagerInterop2_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 


#define IProtectionPolicyManagerInterop2_RequestAccessForAppWithWindowAsync(This,appWindow,sourceIdentity,appPackageFamilyName,riid,asyncOperation)	\
    ( (This)->lpVtbl -> RequestAccessForAppWithWindowAsync(This,appWindow,sourceIdentity,appPackageFamilyName,riid,asyncOperation) ) 

#define IProtectionPolicyManagerInterop2_RequestAccessWithAuditingInfoForWindowAsync(This,appWindow,sourceIdentity,targetIdentity,auditInfoUnk,riid,asyncOperation)	\
    ( (This)->lpVtbl -> RequestAccessWithAuditingInfoForWindowAsync(This,appWindow,sourceIdentity,targetIdentity,auditInfoUnk,riid,asyncOperation) ) 

#define IProtectionPolicyManagerInterop2_RequestAccessWithMessageForWindowAsync(This,appWindow,sourceIdentity,targetIdentity,auditInfoUnk,messageFromApp,riid,asyncOperation)	\
    ( (This)->lpVtbl -> RequestAccessWithMessageForWindowAsync(This,appWindow,sourceIdentity,targetIdentity,auditInfoUnk,messageFromApp,riid,asyncOperation) ) 

#define IProtectionPolicyManagerInterop2_RequestAccessForAppWithAuditingInfoForWindowAsync(This,appWindow,sourceIdentity,appPackageFamilyName,auditInfoUnk,riid,asyncOperation)	\
    ( (This)->lpVtbl -> RequestAccessForAppWithAuditingInfoForWindowAsync(This,appWindow,sourceIdentity,appPackageFamilyName,auditInfoUnk,riid,asyncOperation) ) 

#define IProtectionPolicyManagerInterop2_RequestAccessForAppWithMessageForWindowAsync(This,appWindow,sourceIdentity,appPackageFamilyName,auditInfoUnk,messageFromApp,riid,asyncOperation)	\
    ( (This)->lpVtbl -> RequestAccessForAppWithMessageForWindowAsync(This,appWindow,sourceIdentity,appPackageFamilyName,auditInfoUnk,messageFromApp,riid,asyncOperation) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IProtectionPolicyManagerInterop2_INTERFACE_DEFINED__ */


#ifndef __IProtectionPolicyManagerInterop3_INTERFACE_DEFINED__
#define __IProtectionPolicyManagerInterop3_INTERFACE_DEFINED__

/* interface IProtectionPolicyManagerInterop3 */
/* [object][uuid] */ 


EXTERN_C const IID IID_IProtectionPolicyManagerInterop3;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("c1c03933-b398-4d93-b0fd-2972adf802c2")
    IProtectionPolicyManagerInterop3 : public IInspectable
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE RequestAccessWithBehaviorForWindowAsync( 
            /* [in] */ __RPC__in HWND appWindow,
            /* [in] */ __RPC__in HSTRING sourceIdentity,
            /* [in] */ __RPC__in HSTRING targetIdentity,
            /* [in] */ __RPC__in_opt IUnknown *auditInfoUnk,
            /* [in] */ __RPC__in HSTRING messageFromApp,
            /* [in] */ UINT32 behavior,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][retval][out] */ __RPC__deref_out_opt void **asyncOperation) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RequestAccessForAppWithBehaviorForWindowAsync( 
            /* [in] */ __RPC__in HWND appWindow,
            /* [in] */ __RPC__in HSTRING sourceIdentity,
            /* [in] */ __RPC__in HSTRING appPackageFamilyName,
            /* [in] */ __RPC__in_opt IUnknown *auditInfoUnk,
            /* [in] */ __RPC__in HSTRING messageFromApp,
            /* [in] */ UINT32 behavior,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][retval][out] */ __RPC__deref_out_opt void **asyncOperation) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RequestAccessToFilesForAppForWindowAsync( 
            /* [in] */ __RPC__in HWND appWindow,
            /* [in] */ __RPC__in_opt IUnknown *sourceItemListUnk,
            /* [in] */ __RPC__in HSTRING appPackageFamilyName,
            /* [in] */ __RPC__in_opt IUnknown *auditInfoUnk,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][retval][out] */ __RPC__deref_out_opt void **asyncOperation) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RequestAccessToFilesForAppWithMessageAndBehaviorForWindowAsync( 
            /* [in] */ __RPC__in HWND appWindow,
            /* [in] */ __RPC__in_opt IUnknown *sourceItemListUnk,
            /* [in] */ __RPC__in HSTRING appPackageFamilyName,
            /* [in] */ __RPC__in_opt IUnknown *auditInfoUnk,
            /* [in] */ __RPC__in HSTRING messageFromApp,
            /* [in] */ UINT32 behavior,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][retval][out] */ __RPC__deref_out_opt void **asyncOperation) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RequestAccessToFilesForProcessForWindowAsync( 
            /* [in] */ __RPC__in HWND appWindow,
            /* [in] */ __RPC__in_opt IUnknown *sourceItemListUnk,
            /* [in] */ UINT32 processId,
            /* [in] */ __RPC__in_opt IUnknown *auditInfoUnk,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][retval][out] */ __RPC__deref_out_opt void **asyncOperation) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RequestAccessToFilesForProcessWithMessageAndBehaviorForWindowAsync( 
            /* [in] */ __RPC__in HWND appWindow,
            /* [in] */ __RPC__in_opt IUnknown *sourceItemListUnk,
            /* [in] */ UINT32 processId,
            /* [in] */ __RPC__in_opt IUnknown *auditInfoUnk,
            /* [in] */ __RPC__in HSTRING messageFromApp,
            /* [in] */ UINT32 behavior,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][retval][out] */ __RPC__deref_out_opt void **asyncOperation) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IProtectionPolicyManagerInterop3Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IProtectionPolicyManagerInterop3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IProtectionPolicyManagerInterop3 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IProtectionPolicyManagerInterop3 * This);
        
        DECLSPEC_XFGVIRT(IInspectable, GetIids)
        HRESULT ( STDMETHODCALLTYPE *GetIids )( 
            __RPC__in IProtectionPolicyManagerInterop3 * This,
            /* [out] */ __RPC__out ULONG *iidCount,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids);
        
        DECLSPEC_XFGVIRT(IInspectable, GetRuntimeClassName)
        HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )( 
            __RPC__in IProtectionPolicyManagerInterop3 * This,
            /* [out] */ __RPC__deref_out_opt HSTRING *className);
        
        DECLSPEC_XFGVIRT(IInspectable, GetTrustLevel)
        HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )( 
            __RPC__in IProtectionPolicyManagerInterop3 * This,
            /* [out] */ __RPC__out TrustLevel *trustLevel);
        
        DECLSPEC_XFGVIRT(IProtectionPolicyManagerInterop3, RequestAccessWithBehaviorForWindowAsync)
        HRESULT ( STDMETHODCALLTYPE *RequestAccessWithBehaviorForWindowAsync )( 
            __RPC__in IProtectionPolicyManagerInterop3 * This,
            /* [in] */ __RPC__in HWND appWindow,
            /* [in] */ __RPC__in HSTRING sourceIdentity,
            /* [in] */ __RPC__in HSTRING targetIdentity,
            /* [in] */ __RPC__in_opt IUnknown *auditInfoUnk,
            /* [in] */ __RPC__in HSTRING messageFromApp,
            /* [in] */ UINT32 behavior,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][retval][out] */ __RPC__deref_out_opt void **asyncOperation);
        
        DECLSPEC_XFGVIRT(IProtectionPolicyManagerInterop3, RequestAccessForAppWithBehaviorForWindowAsync)
        HRESULT ( STDMETHODCALLTYPE *RequestAccessForAppWithBehaviorForWindowAsync )( 
            __RPC__in IProtectionPolicyManagerInterop3 * This,
            /* [in] */ __RPC__in HWND appWindow,
            /* [in] */ __RPC__in HSTRING sourceIdentity,
            /* [in] */ __RPC__in HSTRING appPackageFamilyName,
            /* [in] */ __RPC__in_opt IUnknown *auditInfoUnk,
            /* [in] */ __RPC__in HSTRING messageFromApp,
            /* [in] */ UINT32 behavior,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][retval][out] */ __RPC__deref_out_opt void **asyncOperation);
        
        DECLSPEC_XFGVIRT(IProtectionPolicyManagerInterop3, RequestAccessToFilesForAppForWindowAsync)
        HRESULT ( STDMETHODCALLTYPE *RequestAccessToFilesForAppForWindowAsync )( 
            __RPC__in IProtectionPolicyManagerInterop3 * This,
            /* [in] */ __RPC__in HWND appWindow,
            /* [in] */ __RPC__in_opt IUnknown *sourceItemListUnk,
            /* [in] */ __RPC__in HSTRING appPackageFamilyName,
            /* [in] */ __RPC__in_opt IUnknown *auditInfoUnk,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][retval][out] */ __RPC__deref_out_opt void **asyncOperation);
        
        DECLSPEC_XFGVIRT(IProtectionPolicyManagerInterop3, RequestAccessToFilesForAppWithMessageAndBehaviorForWindowAsync)
        HRESULT ( STDMETHODCALLTYPE *RequestAccessToFilesForAppWithMessageAndBehaviorForWindowAsync )( 
            __RPC__in IProtectionPolicyManagerInterop3 * This,
            /* [in] */ __RPC__in HWND appWindow,
            /* [in] */ __RPC__in_opt IUnknown *sourceItemListUnk,
            /* [in] */ __RPC__in HSTRING appPackageFamilyName,
            /* [in] */ __RPC__in_opt IUnknown *auditInfoUnk,
            /* [in] */ __RPC__in HSTRING messageFromApp,
            /* [in] */ UINT32 behavior,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][retval][out] */ __RPC__deref_out_opt void **asyncOperation);
        
        DECLSPEC_XFGVIRT(IProtectionPolicyManagerInterop3, RequestAccessToFilesForProcessForWindowAsync)
        HRESULT ( STDMETHODCALLTYPE *RequestAccessToFilesForProcessForWindowAsync )( 
            __RPC__in IProtectionPolicyManagerInterop3 * This,
            /* [in] */ __RPC__in HWND appWindow,
            /* [in] */ __RPC__in_opt IUnknown *sourceItemListUnk,
            /* [in] */ UINT32 processId,
            /* [in] */ __RPC__in_opt IUnknown *auditInfoUnk,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][retval][out] */ __RPC__deref_out_opt void **asyncOperation);
        
        DECLSPEC_XFGVIRT(IProtectionPolicyManagerInterop3, RequestAccessToFilesForProcessWithMessageAndBehaviorForWindowAsync)
        HRESULT ( STDMETHODCALLTYPE *RequestAccessToFilesForProcessWithMessageAndBehaviorForWindowAsync )( 
            __RPC__in IProtectionPolicyManagerInterop3 * This,
            /* [in] */ __RPC__in HWND appWindow,
            /* [in] */ __RPC__in_opt IUnknown *sourceItemListUnk,
            /* [in] */ UINT32 processId,
            /* [in] */ __RPC__in_opt IUnknown *auditInfoUnk,
            /* [in] */ __RPC__in HSTRING messageFromApp,
            /* [in] */ UINT32 behavior,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][retval][out] */ __RPC__deref_out_opt void **asyncOperation);
        
        END_INTERFACE
    } IProtectionPolicyManagerInterop3Vtbl;

    interface IProtectionPolicyManagerInterop3
    {
        CONST_VTBL struct IProtectionPolicyManagerInterop3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IProtectionPolicyManagerInterop3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IProtectionPolicyManagerInterop3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IProtectionPolicyManagerInterop3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IProtectionPolicyManagerInterop3_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 

#define IProtectionPolicyManagerInterop3_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 

#define IProtectionPolicyManagerInterop3_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 


#define IProtectionPolicyManagerInterop3_RequestAccessWithBehaviorForWindowAsync(This,appWindow,sourceIdentity,targetIdentity,auditInfoUnk,messageFromApp,behavior,riid,asyncOperation)	\
    ( (This)->lpVtbl -> RequestAccessWithBehaviorForWindowAsync(This,appWindow,sourceIdentity,targetIdentity,auditInfoUnk,messageFromApp,behavior,riid,asyncOperation) ) 

#define IProtectionPolicyManagerInterop3_RequestAccessForAppWithBehaviorForWindowAsync(This,appWindow,sourceIdentity,appPackageFamilyName,auditInfoUnk,messageFromApp,behavior,riid,asyncOperation)	\
    ( (This)->lpVtbl -> RequestAccessForAppWithBehaviorForWindowAsync(This,appWindow,sourceIdentity,appPackageFamilyName,auditInfoUnk,messageFromApp,behavior,riid,asyncOperation) ) 

#define IProtectionPolicyManagerInterop3_RequestAccessToFilesForAppForWindowAsync(This,appWindow,sourceItemListUnk,appPackageFamilyName,auditInfoUnk,riid,asyncOperation)	\
    ( (This)->lpVtbl -> RequestAccessToFilesForAppForWindowAsync(This,appWindow,sourceItemListUnk,appPackageFamilyName,auditInfoUnk,riid,asyncOperation) ) 

#define IProtectionPolicyManagerInterop3_RequestAccessToFilesForAppWithMessageAndBehaviorForWindowAsync(This,appWindow,sourceItemListUnk,appPackageFamilyName,auditInfoUnk,messageFromApp,behavior,riid,asyncOperation)	\
    ( (This)->lpVtbl -> RequestAccessToFilesForAppWithMessageAndBehaviorForWindowAsync(This,appWindow,sourceItemListUnk,appPackageFamilyName,auditInfoUnk,messageFromApp,behavior,riid,asyncOperation) ) 

#define IProtectionPolicyManagerInterop3_RequestAccessToFilesForProcessForWindowAsync(This,appWindow,sourceItemListUnk,processId,auditInfoUnk,riid,asyncOperation)	\
    ( (This)->lpVtbl -> RequestAccessToFilesForProcessForWindowAsync(This,appWindow,sourceItemListUnk,processId,auditInfoUnk,riid,asyncOperation) ) 

#define IProtectionPolicyManagerInterop3_RequestAccessToFilesForProcessWithMessageAndBehaviorForWindowAsync(This,appWindow,sourceItemListUnk,processId,auditInfoUnk,messageFromApp,behavior,riid,asyncOperation)	\
    ( (This)->lpVtbl -> RequestAccessToFilesForProcessWithMessageAndBehaviorForWindowAsync(This,appWindow,sourceItemListUnk,processId,auditInfoUnk,messageFromApp,behavior,riid,asyncOperation) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IProtectionPolicyManagerInterop3_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_efswrtinterop_0000_0003 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#endif //(NTDDI_VERSION >= NTDDI_WINTHRESHOLD)


extern RPC_IF_HANDLE __MIDL_itf_efswrtinterop_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_efswrtinterop_0000_0003_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  HSTRING_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in HSTRING * ); 
unsigned char * __RPC_USER  HSTRING_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HSTRING * ); 
unsigned char * __RPC_USER  HSTRING_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HSTRING * ); 
void                      __RPC_USER  HSTRING_UserFree(     __RPC__in unsigned long *, __RPC__in HSTRING * ); 

unsigned long             __RPC_USER  HWND_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HWND * ); 
void                      __RPC_USER  HWND_UserFree(     __RPC__in unsigned long *, __RPC__in HWND * ); 

unsigned long             __RPC_USER  HSTRING_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in HSTRING * ); 
unsigned char * __RPC_USER  HSTRING_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HSTRING * ); 
unsigned char * __RPC_USER  HSTRING_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HSTRING * ); 
void                      __RPC_USER  HSTRING_UserFree64(     __RPC__in unsigned long *, __RPC__in HSTRING * ); 

unsigned long             __RPC_USER  HWND_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HWND * ); 
void                      __RPC_USER  HWND_UserFree64(     __RPC__in unsigned long *, __RPC__in HWND * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


