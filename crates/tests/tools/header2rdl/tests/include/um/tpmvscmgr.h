

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

#ifndef __tpmvscmgr_h__
#define __tpmvscmgr_h__

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

#ifndef __ITpmVirtualSmartCardManagerStatusCallback_FWD_DEFINED__
#define __ITpmVirtualSmartCardManagerStatusCallback_FWD_DEFINED__
typedef interface ITpmVirtualSmartCardManagerStatusCallback ITpmVirtualSmartCardManagerStatusCallback;

#endif 	/* __ITpmVirtualSmartCardManagerStatusCallback_FWD_DEFINED__ */


#ifndef __ITpmVirtualSmartCardManager_FWD_DEFINED__
#define __ITpmVirtualSmartCardManager_FWD_DEFINED__
typedef interface ITpmVirtualSmartCardManager ITpmVirtualSmartCardManager;

#endif 	/* __ITpmVirtualSmartCardManager_FWD_DEFINED__ */


#ifndef __ITpmVirtualSmartCardManager2_FWD_DEFINED__
#define __ITpmVirtualSmartCardManager2_FWD_DEFINED__
typedef interface ITpmVirtualSmartCardManager2 ITpmVirtualSmartCardManager2;

#endif 	/* __ITpmVirtualSmartCardManager2_FWD_DEFINED__ */


#ifndef __ITpmVirtualSmartCardManager3_FWD_DEFINED__
#define __ITpmVirtualSmartCardManager3_FWD_DEFINED__
typedef interface ITpmVirtualSmartCardManager3 ITpmVirtualSmartCardManager3;

#endif 	/* __ITpmVirtualSmartCardManager3_FWD_DEFINED__ */


#ifndef __TpmVirtualSmartCardManager_FWD_DEFINED__
#define __TpmVirtualSmartCardManager_FWD_DEFINED__

#ifdef __cplusplus
typedef class TpmVirtualSmartCardManager TpmVirtualSmartCardManager;
#else
typedef struct TpmVirtualSmartCardManager TpmVirtualSmartCardManager;
#endif /* __cplusplus */

#endif 	/* __TpmVirtualSmartCardManager_FWD_DEFINED__ */


#ifndef __RemoteTpmVirtualSmartCardManager_FWD_DEFINED__
#define __RemoteTpmVirtualSmartCardManager_FWD_DEFINED__

#ifdef __cplusplus
typedef class RemoteTpmVirtualSmartCardManager RemoteTpmVirtualSmartCardManager;
#else
typedef struct RemoteTpmVirtualSmartCardManager RemoteTpmVirtualSmartCardManager;
#endif /* __cplusplus */

#endif 	/* __RemoteTpmVirtualSmartCardManager_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "TpmVscAttestation.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_tpmvscmgr_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#if (NTDDI_VERSION >= NTDDI_WIN8)
typedef /* [public][public][v1_enum] */ 
enum __MIDL___MIDL_itf_tpmvscmgr_0000_0000_0001
    {
        TPMVSCMGR_STATUS_VTPMSMARTCARD_INITIALIZING	= 0,
        TPMVSCMGR_STATUS_VTPMSMARTCARD_CREATING	= ( TPMVSCMGR_STATUS_VTPMSMARTCARD_INITIALIZING + 1 ) ,
        TPMVSCMGR_STATUS_VTPMSMARTCARD_DESTROYING	= ( TPMVSCMGR_STATUS_VTPMSMARTCARD_CREATING + 1 ) ,
        TPMVSCMGR_STATUS_VGIDSSIMULATOR_INITIALIZING	= ( TPMVSCMGR_STATUS_VTPMSMARTCARD_DESTROYING + 1 ) ,
        TPMVSCMGR_STATUS_VGIDSSIMULATOR_CREATING	= ( TPMVSCMGR_STATUS_VGIDSSIMULATOR_INITIALIZING + 1 ) ,
        TPMVSCMGR_STATUS_VGIDSSIMULATOR_DESTROYING	= ( TPMVSCMGR_STATUS_VGIDSSIMULATOR_CREATING + 1 ) ,
        TPMVSCMGR_STATUS_VREADER_INITIALIZING	= ( TPMVSCMGR_STATUS_VGIDSSIMULATOR_DESTROYING + 1 ) ,
        TPMVSCMGR_STATUS_VREADER_CREATING	= ( TPMVSCMGR_STATUS_VREADER_INITIALIZING + 1 ) ,
        TPMVSCMGR_STATUS_VREADER_DESTROYING	= ( TPMVSCMGR_STATUS_VREADER_CREATING + 1 ) ,
        TPMVSCMGR_STATUS_GENERATE_WAITING	= ( TPMVSCMGR_STATUS_VREADER_DESTROYING + 1 ) ,
        TPMVSCMGR_STATUS_GENERATE_AUTHENTICATING	= ( TPMVSCMGR_STATUS_GENERATE_WAITING + 1 ) ,
        TPMVSCMGR_STATUS_GENERATE_RUNNING	= ( TPMVSCMGR_STATUS_GENERATE_AUTHENTICATING + 1 ) ,
        TPMVSCMGR_STATUS_CARD_CREATED	= ( TPMVSCMGR_STATUS_GENERATE_RUNNING + 1 ) ,
        TPMVSCMGR_STATUS_CARD_DESTROYED	= ( TPMVSCMGR_STATUS_CARD_CREATED + 1 ) 
    } 	TPMVSCMGR_STATUS;

typedef /* [public][public][v1_enum] */ 
enum __MIDL___MIDL_itf_tpmvscmgr_0000_0000_0002
    {
        TPMVSCMGR_ERROR_IMPERSONATION	= 0,
        TPMVSCMGR_ERROR_PIN_COMPLEXITY	= ( TPMVSCMGR_ERROR_IMPERSONATION + 1 ) ,
        TPMVSCMGR_ERROR_READER_COUNT_LIMIT	= ( TPMVSCMGR_ERROR_PIN_COMPLEXITY + 1 ) ,
        TPMVSCMGR_ERROR_TERMINAL_SERVICES_SESSION	= ( TPMVSCMGR_ERROR_READER_COUNT_LIMIT + 1 ) ,
        TPMVSCMGR_ERROR_VTPMSMARTCARD_INITIALIZE	= ( TPMVSCMGR_ERROR_TERMINAL_SERVICES_SESSION + 1 ) ,
        TPMVSCMGR_ERROR_VTPMSMARTCARD_CREATE	= ( TPMVSCMGR_ERROR_VTPMSMARTCARD_INITIALIZE + 1 ) ,
        TPMVSCMGR_ERROR_VTPMSMARTCARD_DESTROY	= ( TPMVSCMGR_ERROR_VTPMSMARTCARD_CREATE + 1 ) ,
        TPMVSCMGR_ERROR_VGIDSSIMULATOR_INITIALIZE	= ( TPMVSCMGR_ERROR_VTPMSMARTCARD_DESTROY + 1 ) ,
        TPMVSCMGR_ERROR_VGIDSSIMULATOR_CREATE	= ( TPMVSCMGR_ERROR_VGIDSSIMULATOR_INITIALIZE + 1 ) ,
        TPMVSCMGR_ERROR_VGIDSSIMULATOR_DESTROY	= ( TPMVSCMGR_ERROR_VGIDSSIMULATOR_CREATE + 1 ) ,
        TPMVSCMGR_ERROR_VGIDSSIMULATOR_WRITE_PROPERTY	= ( TPMVSCMGR_ERROR_VGIDSSIMULATOR_DESTROY + 1 ) ,
        TPMVSCMGR_ERROR_VGIDSSIMULATOR_READ_PROPERTY	= ( TPMVSCMGR_ERROR_VGIDSSIMULATOR_WRITE_PROPERTY + 1 ) ,
        TPMVSCMGR_ERROR_VREADER_INITIALIZE	= ( TPMVSCMGR_ERROR_VGIDSSIMULATOR_READ_PROPERTY + 1 ) ,
        TPMVSCMGR_ERROR_VREADER_CREATE	= ( TPMVSCMGR_ERROR_VREADER_INITIALIZE + 1 ) ,
        TPMVSCMGR_ERROR_VREADER_DESTROY	= ( TPMVSCMGR_ERROR_VREADER_CREATE + 1 ) ,
        TPMVSCMGR_ERROR_GENERATE_LOCATE_READER	= ( TPMVSCMGR_ERROR_VREADER_DESTROY + 1 ) ,
        TPMVSCMGR_ERROR_GENERATE_FILESYSTEM	= ( TPMVSCMGR_ERROR_GENERATE_LOCATE_READER + 1 ) ,
        TPMVSCMGR_ERROR_CARD_CREATE	= ( TPMVSCMGR_ERROR_GENERATE_FILESYSTEM + 1 ) ,
        TPMVSCMGR_ERROR_CARD_DESTROY	= ( TPMVSCMGR_ERROR_CARD_CREATE + 1 ) 
    } 	TPMVSCMGR_ERROR;



extern RPC_IF_HANDLE __MIDL_itf_tpmvscmgr_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_tpmvscmgr_0000_0000_v0_0_s_ifspec;

#ifndef __ITpmVirtualSmartCardManagerStatusCallback_INTERFACE_DEFINED__
#define __ITpmVirtualSmartCardManagerStatusCallback_INTERFACE_DEFINED__

/* interface ITpmVirtualSmartCardManagerStatusCallback */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ITpmVirtualSmartCardManagerStatusCallback;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1A1BB35F-ABB8-451C-A1AE-33D98F1BEF4A")
    ITpmVirtualSmartCardManagerStatusCallback : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ReportProgress( 
            /* [in] */ TPMVSCMGR_STATUS Status) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ReportError( 
            /* [in] */ TPMVSCMGR_ERROR Error) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITpmVirtualSmartCardManagerStatusCallbackVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITpmVirtualSmartCardManagerStatusCallback * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITpmVirtualSmartCardManagerStatusCallback * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITpmVirtualSmartCardManagerStatusCallback * This);
        
        DECLSPEC_XFGVIRT(ITpmVirtualSmartCardManagerStatusCallback, ReportProgress)
        HRESULT ( STDMETHODCALLTYPE *ReportProgress )( 
            __RPC__in ITpmVirtualSmartCardManagerStatusCallback * This,
            /* [in] */ TPMVSCMGR_STATUS Status);
        
        DECLSPEC_XFGVIRT(ITpmVirtualSmartCardManagerStatusCallback, ReportError)
        HRESULT ( STDMETHODCALLTYPE *ReportError )( 
            __RPC__in ITpmVirtualSmartCardManagerStatusCallback * This,
            /* [in] */ TPMVSCMGR_ERROR Error);
        
        END_INTERFACE
    } ITpmVirtualSmartCardManagerStatusCallbackVtbl;

    interface ITpmVirtualSmartCardManagerStatusCallback
    {
        CONST_VTBL struct ITpmVirtualSmartCardManagerStatusCallbackVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITpmVirtualSmartCardManagerStatusCallback_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITpmVirtualSmartCardManagerStatusCallback_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITpmVirtualSmartCardManagerStatusCallback_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITpmVirtualSmartCardManagerStatusCallback_ReportProgress(This,Status)	\
    ( (This)->lpVtbl -> ReportProgress(This,Status) ) 

#define ITpmVirtualSmartCardManagerStatusCallback_ReportError(This,Error)	\
    ( (This)->lpVtbl -> ReportError(This,Error) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITpmVirtualSmartCardManagerStatusCallback_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_tpmvscmgr_0000_0001 */
/* [local] */ 

//
// TPM Virtual Smart Card Default Admin Key Algorithm ID
// 0x82 = 0x02 (3-key triple DES) |
//        0x80 (ISO/IEC 9797 padding method 2) |
//        0x00 (CBC mode)
//
#define	TPMVSC_DEFAULT_ADMIN_ALGORITHM_ID	( 0x82 )



extern RPC_IF_HANDLE __MIDL_itf_tpmvscmgr_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_tpmvscmgr_0000_0001_v0_0_s_ifspec;

#ifndef __ITpmVirtualSmartCardManager_INTERFACE_DEFINED__
#define __ITpmVirtualSmartCardManager_INTERFACE_DEFINED__

/* interface ITpmVirtualSmartCardManager */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ITpmVirtualSmartCardManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("112B1DFF-D9DC-41F7-869F-D67FEE7CB591")
    ITpmVirtualSmartCardManager : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateVirtualSmartCard( 
            /* [string][in] */ __RPC__in_string LPCWSTR pszFriendlyName,
            /* [in] */ BYTE bAdminAlgId,
            /* [size_is][in] */ __RPC__in_ecount_full(cbAdminKey) const BYTE *pbAdminKey,
            /* [in] */ DWORD cbAdminKey,
            /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(cbAdminKcv) const BYTE *pbAdminKcv,
            /* [in] */ DWORD cbAdminKcv,
            /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(cbPuk) const BYTE *pbPuk,
            /* [in] */ DWORD cbPuk,
            /* [size_is][in] */ __RPC__in_ecount_full(cbPin) const BYTE *pbPin,
            /* [in] */ DWORD cbPin,
            /* [in] */ BOOL fGenerate,
            /* [unique][in] */ __RPC__in_opt ITpmVirtualSmartCardManagerStatusCallback *pStatusCallback,
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *ppszInstanceId,
            /* [out] */ __RPC__out BOOL *pfNeedReboot) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DestroyVirtualSmartCard( 
            /* [string][in] */ __RPC__in_string LPCWSTR pszInstanceId,
            /* [unique][in] */ __RPC__in_opt ITpmVirtualSmartCardManagerStatusCallback *pStatusCallback,
            /* [out] */ __RPC__out BOOL *pfNeedReboot) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITpmVirtualSmartCardManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITpmVirtualSmartCardManager * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITpmVirtualSmartCardManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITpmVirtualSmartCardManager * This);
        
        DECLSPEC_XFGVIRT(ITpmVirtualSmartCardManager, CreateVirtualSmartCard)
        HRESULT ( STDMETHODCALLTYPE *CreateVirtualSmartCard )( 
            __RPC__in ITpmVirtualSmartCardManager * This,
            /* [string][in] */ __RPC__in_string LPCWSTR pszFriendlyName,
            /* [in] */ BYTE bAdminAlgId,
            /* [size_is][in] */ __RPC__in_ecount_full(cbAdminKey) const BYTE *pbAdminKey,
            /* [in] */ DWORD cbAdminKey,
            /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(cbAdminKcv) const BYTE *pbAdminKcv,
            /* [in] */ DWORD cbAdminKcv,
            /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(cbPuk) const BYTE *pbPuk,
            /* [in] */ DWORD cbPuk,
            /* [size_is][in] */ __RPC__in_ecount_full(cbPin) const BYTE *pbPin,
            /* [in] */ DWORD cbPin,
            /* [in] */ BOOL fGenerate,
            /* [unique][in] */ __RPC__in_opt ITpmVirtualSmartCardManagerStatusCallback *pStatusCallback,
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *ppszInstanceId,
            /* [out] */ __RPC__out BOOL *pfNeedReboot);
        
        DECLSPEC_XFGVIRT(ITpmVirtualSmartCardManager, DestroyVirtualSmartCard)
        HRESULT ( STDMETHODCALLTYPE *DestroyVirtualSmartCard )( 
            __RPC__in ITpmVirtualSmartCardManager * This,
            /* [string][in] */ __RPC__in_string LPCWSTR pszInstanceId,
            /* [unique][in] */ __RPC__in_opt ITpmVirtualSmartCardManagerStatusCallback *pStatusCallback,
            /* [out] */ __RPC__out BOOL *pfNeedReboot);
        
        END_INTERFACE
    } ITpmVirtualSmartCardManagerVtbl;

    interface ITpmVirtualSmartCardManager
    {
        CONST_VTBL struct ITpmVirtualSmartCardManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITpmVirtualSmartCardManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITpmVirtualSmartCardManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITpmVirtualSmartCardManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITpmVirtualSmartCardManager_CreateVirtualSmartCard(This,pszFriendlyName,bAdminAlgId,pbAdminKey,cbAdminKey,pbAdminKcv,cbAdminKcv,pbPuk,cbPuk,pbPin,cbPin,fGenerate,pStatusCallback,ppszInstanceId,pfNeedReboot)	\
    ( (This)->lpVtbl -> CreateVirtualSmartCard(This,pszFriendlyName,bAdminAlgId,pbAdminKey,cbAdminKey,pbAdminKcv,cbAdminKcv,pbPuk,cbPuk,pbPin,cbPin,fGenerate,pStatusCallback,ppszInstanceId,pfNeedReboot) ) 

#define ITpmVirtualSmartCardManager_DestroyVirtualSmartCard(This,pszInstanceId,pStatusCallback,pfNeedReboot)	\
    ( (This)->lpVtbl -> DestroyVirtualSmartCard(This,pszInstanceId,pStatusCallback,pfNeedReboot) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITpmVirtualSmartCardManager_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_tpmvscmgr_0000_0002 */
/* [local] */ 

#endif // (NTDDI_VERSION >= NTDDI_WIN8)
#if (NTDDI_VERSION >= NTDDI_WINBLUE)


extern RPC_IF_HANDLE __MIDL_itf_tpmvscmgr_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_tpmvscmgr_0000_0002_v0_0_s_ifspec;

#ifndef __ITpmVirtualSmartCardManager2_INTERFACE_DEFINED__
#define __ITpmVirtualSmartCardManager2_INTERFACE_DEFINED__

/* interface ITpmVirtualSmartCardManager2 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ITpmVirtualSmartCardManager2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("FDF8A2B9-02DE-47F4-BC26-AA85AB5E5267")
    ITpmVirtualSmartCardManager2 : public ITpmVirtualSmartCardManager
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateVirtualSmartCardWithPinPolicy( 
            /* [string][in] */ __RPC__in_string LPCWSTR pszFriendlyName,
            /* [in] */ BYTE bAdminAlgId,
            /* [size_is][in] */ __RPC__in_ecount_full(cbAdminKey) const BYTE *pbAdminKey,
            /* [in] */ DWORD cbAdminKey,
            /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(cbAdminKcv) const BYTE *pbAdminKcv,
            /* [in] */ DWORD cbAdminKcv,
            /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(cbPuk) const BYTE *pbPuk,
            /* [in] */ DWORD cbPuk,
            /* [size_is][in] */ __RPC__in_ecount_full(cbPin) const BYTE *pbPin,
            /* [in] */ DWORD cbPin,
            /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(cbPinPolicy) const BYTE *pbPinPolicy,
            /* [in] */ DWORD cbPinPolicy,
            /* [in] */ BOOL fGenerate,
            /* [unique][in] */ __RPC__in_opt ITpmVirtualSmartCardManagerStatusCallback *pStatusCallback,
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *ppszInstanceId,
            /* [out] */ __RPC__out BOOL *pfNeedReboot) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITpmVirtualSmartCardManager2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITpmVirtualSmartCardManager2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITpmVirtualSmartCardManager2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITpmVirtualSmartCardManager2 * This);
        
        DECLSPEC_XFGVIRT(ITpmVirtualSmartCardManager, CreateVirtualSmartCard)
        HRESULT ( STDMETHODCALLTYPE *CreateVirtualSmartCard )( 
            __RPC__in ITpmVirtualSmartCardManager2 * This,
            /* [string][in] */ __RPC__in_string LPCWSTR pszFriendlyName,
            /* [in] */ BYTE bAdminAlgId,
            /* [size_is][in] */ __RPC__in_ecount_full(cbAdminKey) const BYTE *pbAdminKey,
            /* [in] */ DWORD cbAdminKey,
            /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(cbAdminKcv) const BYTE *pbAdminKcv,
            /* [in] */ DWORD cbAdminKcv,
            /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(cbPuk) const BYTE *pbPuk,
            /* [in] */ DWORD cbPuk,
            /* [size_is][in] */ __RPC__in_ecount_full(cbPin) const BYTE *pbPin,
            /* [in] */ DWORD cbPin,
            /* [in] */ BOOL fGenerate,
            /* [unique][in] */ __RPC__in_opt ITpmVirtualSmartCardManagerStatusCallback *pStatusCallback,
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *ppszInstanceId,
            /* [out] */ __RPC__out BOOL *pfNeedReboot);
        
        DECLSPEC_XFGVIRT(ITpmVirtualSmartCardManager, DestroyVirtualSmartCard)
        HRESULT ( STDMETHODCALLTYPE *DestroyVirtualSmartCard )( 
            __RPC__in ITpmVirtualSmartCardManager2 * This,
            /* [string][in] */ __RPC__in_string LPCWSTR pszInstanceId,
            /* [unique][in] */ __RPC__in_opt ITpmVirtualSmartCardManagerStatusCallback *pStatusCallback,
            /* [out] */ __RPC__out BOOL *pfNeedReboot);
        
        DECLSPEC_XFGVIRT(ITpmVirtualSmartCardManager2, CreateVirtualSmartCardWithPinPolicy)
        HRESULT ( STDMETHODCALLTYPE *CreateVirtualSmartCardWithPinPolicy )( 
            __RPC__in ITpmVirtualSmartCardManager2 * This,
            /* [string][in] */ __RPC__in_string LPCWSTR pszFriendlyName,
            /* [in] */ BYTE bAdminAlgId,
            /* [size_is][in] */ __RPC__in_ecount_full(cbAdminKey) const BYTE *pbAdminKey,
            /* [in] */ DWORD cbAdminKey,
            /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(cbAdminKcv) const BYTE *pbAdminKcv,
            /* [in] */ DWORD cbAdminKcv,
            /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(cbPuk) const BYTE *pbPuk,
            /* [in] */ DWORD cbPuk,
            /* [size_is][in] */ __RPC__in_ecount_full(cbPin) const BYTE *pbPin,
            /* [in] */ DWORD cbPin,
            /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(cbPinPolicy) const BYTE *pbPinPolicy,
            /* [in] */ DWORD cbPinPolicy,
            /* [in] */ BOOL fGenerate,
            /* [unique][in] */ __RPC__in_opt ITpmVirtualSmartCardManagerStatusCallback *pStatusCallback,
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *ppszInstanceId,
            /* [out] */ __RPC__out BOOL *pfNeedReboot);
        
        END_INTERFACE
    } ITpmVirtualSmartCardManager2Vtbl;

    interface ITpmVirtualSmartCardManager2
    {
        CONST_VTBL struct ITpmVirtualSmartCardManager2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITpmVirtualSmartCardManager2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITpmVirtualSmartCardManager2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITpmVirtualSmartCardManager2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITpmVirtualSmartCardManager2_CreateVirtualSmartCard(This,pszFriendlyName,bAdminAlgId,pbAdminKey,cbAdminKey,pbAdminKcv,cbAdminKcv,pbPuk,cbPuk,pbPin,cbPin,fGenerate,pStatusCallback,ppszInstanceId,pfNeedReboot)	\
    ( (This)->lpVtbl -> CreateVirtualSmartCard(This,pszFriendlyName,bAdminAlgId,pbAdminKey,cbAdminKey,pbAdminKcv,cbAdminKcv,pbPuk,cbPuk,pbPin,cbPin,fGenerate,pStatusCallback,ppszInstanceId,pfNeedReboot) ) 

#define ITpmVirtualSmartCardManager2_DestroyVirtualSmartCard(This,pszInstanceId,pStatusCallback,pfNeedReboot)	\
    ( (This)->lpVtbl -> DestroyVirtualSmartCard(This,pszInstanceId,pStatusCallback,pfNeedReboot) ) 


#define ITpmVirtualSmartCardManager2_CreateVirtualSmartCardWithPinPolicy(This,pszFriendlyName,bAdminAlgId,pbAdminKey,cbAdminKey,pbAdminKcv,cbAdminKcv,pbPuk,cbPuk,pbPin,cbPin,pbPinPolicy,cbPinPolicy,fGenerate,pStatusCallback,ppszInstanceId,pfNeedReboot)	\
    ( (This)->lpVtbl -> CreateVirtualSmartCardWithPinPolicy(This,pszFriendlyName,bAdminAlgId,pbAdminKey,cbAdminKey,pbAdminKcv,cbAdminKcv,pbPuk,cbPuk,pbPin,cbPin,pbPinPolicy,cbPinPolicy,fGenerate,pStatusCallback,ppszInstanceId,pfNeedReboot) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITpmVirtualSmartCardManager2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_tpmvscmgr_0000_0003 */
/* [local] */ 

#endif // (NTDDI_VERSION >= NTDDI_WINBLUE)
#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)


extern RPC_IF_HANDLE __MIDL_itf_tpmvscmgr_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_tpmvscmgr_0000_0003_v0_0_s_ifspec;

#ifndef __ITpmVirtualSmartCardManager3_INTERFACE_DEFINED__
#define __ITpmVirtualSmartCardManager3_INTERFACE_DEFINED__

/* interface ITpmVirtualSmartCardManager3 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ITpmVirtualSmartCardManager3;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3C745A97-F375-4150-BE17-5950F694C699")
    ITpmVirtualSmartCardManager3 : public ITpmVirtualSmartCardManager2
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateVirtualSmartCardWithAttestation( 
            /* [string][in] */ __RPC__in_string LPCWSTR pszFriendlyName,
            /* [in] */ BYTE bAdminAlgId,
            /* [size_is][in] */ __RPC__in_ecount_full(cbAdminKey) const BYTE *pbAdminKey,
            /* [in] */ DWORD cbAdminKey,
            /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(cbAdminKcv) const BYTE *pbAdminKcv,
            /* [in] */ DWORD cbAdminKcv,
            /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(cbPuk) const BYTE *pbPuk,
            /* [in] */ DWORD cbPuk,
            /* [size_is][in] */ __RPC__in_ecount_full(cbPin) const BYTE *pbPin,
            /* [in] */ DWORD cbPin,
            /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(cbPinPolicy) const BYTE *pbPinPolicy,
            /* [in] */ DWORD cbPinPolicy,
            /* [in] */ TPMVSC_ATTESTATION_TYPE attestationType,
            /* [in] */ BOOL fGenerate,
            /* [unique][in] */ __RPC__in_opt ITpmVirtualSmartCardManagerStatusCallback *pStatusCallback,
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *ppszInstanceId) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITpmVirtualSmartCardManager3Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITpmVirtualSmartCardManager3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITpmVirtualSmartCardManager3 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITpmVirtualSmartCardManager3 * This);
        
        DECLSPEC_XFGVIRT(ITpmVirtualSmartCardManager, CreateVirtualSmartCard)
        HRESULT ( STDMETHODCALLTYPE *CreateVirtualSmartCard )( 
            __RPC__in ITpmVirtualSmartCardManager3 * This,
            /* [string][in] */ __RPC__in_string LPCWSTR pszFriendlyName,
            /* [in] */ BYTE bAdminAlgId,
            /* [size_is][in] */ __RPC__in_ecount_full(cbAdminKey) const BYTE *pbAdminKey,
            /* [in] */ DWORD cbAdminKey,
            /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(cbAdminKcv) const BYTE *pbAdminKcv,
            /* [in] */ DWORD cbAdminKcv,
            /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(cbPuk) const BYTE *pbPuk,
            /* [in] */ DWORD cbPuk,
            /* [size_is][in] */ __RPC__in_ecount_full(cbPin) const BYTE *pbPin,
            /* [in] */ DWORD cbPin,
            /* [in] */ BOOL fGenerate,
            /* [unique][in] */ __RPC__in_opt ITpmVirtualSmartCardManagerStatusCallback *pStatusCallback,
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *ppszInstanceId,
            /* [out] */ __RPC__out BOOL *pfNeedReboot);
        
        DECLSPEC_XFGVIRT(ITpmVirtualSmartCardManager, DestroyVirtualSmartCard)
        HRESULT ( STDMETHODCALLTYPE *DestroyVirtualSmartCard )( 
            __RPC__in ITpmVirtualSmartCardManager3 * This,
            /* [string][in] */ __RPC__in_string LPCWSTR pszInstanceId,
            /* [unique][in] */ __RPC__in_opt ITpmVirtualSmartCardManagerStatusCallback *pStatusCallback,
            /* [out] */ __RPC__out BOOL *pfNeedReboot);
        
        DECLSPEC_XFGVIRT(ITpmVirtualSmartCardManager2, CreateVirtualSmartCardWithPinPolicy)
        HRESULT ( STDMETHODCALLTYPE *CreateVirtualSmartCardWithPinPolicy )( 
            __RPC__in ITpmVirtualSmartCardManager3 * This,
            /* [string][in] */ __RPC__in_string LPCWSTR pszFriendlyName,
            /* [in] */ BYTE bAdminAlgId,
            /* [size_is][in] */ __RPC__in_ecount_full(cbAdminKey) const BYTE *pbAdminKey,
            /* [in] */ DWORD cbAdminKey,
            /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(cbAdminKcv) const BYTE *pbAdminKcv,
            /* [in] */ DWORD cbAdminKcv,
            /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(cbPuk) const BYTE *pbPuk,
            /* [in] */ DWORD cbPuk,
            /* [size_is][in] */ __RPC__in_ecount_full(cbPin) const BYTE *pbPin,
            /* [in] */ DWORD cbPin,
            /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(cbPinPolicy) const BYTE *pbPinPolicy,
            /* [in] */ DWORD cbPinPolicy,
            /* [in] */ BOOL fGenerate,
            /* [unique][in] */ __RPC__in_opt ITpmVirtualSmartCardManagerStatusCallback *pStatusCallback,
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *ppszInstanceId,
            /* [out] */ __RPC__out BOOL *pfNeedReboot);
        
        DECLSPEC_XFGVIRT(ITpmVirtualSmartCardManager3, CreateVirtualSmartCardWithAttestation)
        HRESULT ( STDMETHODCALLTYPE *CreateVirtualSmartCardWithAttestation )( 
            __RPC__in ITpmVirtualSmartCardManager3 * This,
            /* [string][in] */ __RPC__in_string LPCWSTR pszFriendlyName,
            /* [in] */ BYTE bAdminAlgId,
            /* [size_is][in] */ __RPC__in_ecount_full(cbAdminKey) const BYTE *pbAdminKey,
            /* [in] */ DWORD cbAdminKey,
            /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(cbAdminKcv) const BYTE *pbAdminKcv,
            /* [in] */ DWORD cbAdminKcv,
            /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(cbPuk) const BYTE *pbPuk,
            /* [in] */ DWORD cbPuk,
            /* [size_is][in] */ __RPC__in_ecount_full(cbPin) const BYTE *pbPin,
            /* [in] */ DWORD cbPin,
            /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(cbPinPolicy) const BYTE *pbPinPolicy,
            /* [in] */ DWORD cbPinPolicy,
            /* [in] */ TPMVSC_ATTESTATION_TYPE attestationType,
            /* [in] */ BOOL fGenerate,
            /* [unique][in] */ __RPC__in_opt ITpmVirtualSmartCardManagerStatusCallback *pStatusCallback,
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *ppszInstanceId);
        
        END_INTERFACE
    } ITpmVirtualSmartCardManager3Vtbl;

    interface ITpmVirtualSmartCardManager3
    {
        CONST_VTBL struct ITpmVirtualSmartCardManager3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITpmVirtualSmartCardManager3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITpmVirtualSmartCardManager3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITpmVirtualSmartCardManager3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITpmVirtualSmartCardManager3_CreateVirtualSmartCard(This,pszFriendlyName,bAdminAlgId,pbAdminKey,cbAdminKey,pbAdminKcv,cbAdminKcv,pbPuk,cbPuk,pbPin,cbPin,fGenerate,pStatusCallback,ppszInstanceId,pfNeedReboot)	\
    ( (This)->lpVtbl -> CreateVirtualSmartCard(This,pszFriendlyName,bAdminAlgId,pbAdminKey,cbAdminKey,pbAdminKcv,cbAdminKcv,pbPuk,cbPuk,pbPin,cbPin,fGenerate,pStatusCallback,ppszInstanceId,pfNeedReboot) ) 

#define ITpmVirtualSmartCardManager3_DestroyVirtualSmartCard(This,pszInstanceId,pStatusCallback,pfNeedReboot)	\
    ( (This)->lpVtbl -> DestroyVirtualSmartCard(This,pszInstanceId,pStatusCallback,pfNeedReboot) ) 


#define ITpmVirtualSmartCardManager3_CreateVirtualSmartCardWithPinPolicy(This,pszFriendlyName,bAdminAlgId,pbAdminKey,cbAdminKey,pbAdminKcv,cbAdminKcv,pbPuk,cbPuk,pbPin,cbPin,pbPinPolicy,cbPinPolicy,fGenerate,pStatusCallback,ppszInstanceId,pfNeedReboot)	\
    ( (This)->lpVtbl -> CreateVirtualSmartCardWithPinPolicy(This,pszFriendlyName,bAdminAlgId,pbAdminKey,cbAdminKey,pbAdminKcv,cbAdminKcv,pbPuk,cbPuk,pbPin,cbPin,pbPinPolicy,cbPinPolicy,fGenerate,pStatusCallback,ppszInstanceId,pfNeedReboot) ) 


#define ITpmVirtualSmartCardManager3_CreateVirtualSmartCardWithAttestation(This,pszFriendlyName,bAdminAlgId,pbAdminKey,cbAdminKey,pbAdminKcv,cbAdminKcv,pbPuk,cbPuk,pbPin,cbPin,pbPinPolicy,cbPinPolicy,attestationType,fGenerate,pStatusCallback,ppszInstanceId)	\
    ( (This)->lpVtbl -> CreateVirtualSmartCardWithAttestation(This,pszFriendlyName,bAdminAlgId,pbAdminKey,cbAdminKey,pbAdminKcv,cbAdminKcv,pbPuk,cbPuk,pbPin,cbPin,pbPinPolicy,cbPinPolicy,attestationType,fGenerate,pStatusCallback,ppszInstanceId) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITpmVirtualSmartCardManager3_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_tpmvscmgr_0000_0004 */
/* [local] */ 

#endif // (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)
#if (NTDDI_VERSION >= NTDDI_WIN8)


extern RPC_IF_HANDLE __MIDL_itf_tpmvscmgr_0000_0004_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_tpmvscmgr_0000_0004_v0_0_s_ifspec;


#ifndef __TpmVirtualSmartCardManagers_LIBRARY_DEFINED__
#define __TpmVirtualSmartCardManagers_LIBRARY_DEFINED__

/* library TpmVirtualSmartCardManagers */
/* [uuid] */ 


EXTERN_C const IID LIBID_TpmVirtualSmartCardManagers;

EXTERN_C const CLSID CLSID_TpmVirtualSmartCardManager;

#ifdef __cplusplus

class DECLSPEC_UUID("16A18E86-7F6E-4C20-AD89-4FFC0DB7A96A")
TpmVirtualSmartCardManager;
#endif

EXTERN_C const CLSID CLSID_RemoteTpmVirtualSmartCardManager;

#ifdef __cplusplus

class DECLSPEC_UUID("152EA2A8-70DC-4C59-8B2A-32AA3CA0DCAC")
RemoteTpmVirtualSmartCardManager;
#endif
#endif /* __TpmVirtualSmartCardManagers_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_tpmvscmgr_0000_0005 */
/* [local] */ 

#endif // (NTDDI_VERSION >= NTDDI_WIN8)
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_tpmvscmgr_0000_0005_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_tpmvscmgr_0000_0005_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


