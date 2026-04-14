

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

#ifndef __certadm_h__
#define __certadm_h__

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

#ifndef __ICertAdmin_FWD_DEFINED__
#define __ICertAdmin_FWD_DEFINED__
typedef interface ICertAdmin ICertAdmin;

#endif 	/* __ICertAdmin_FWD_DEFINED__ */


#ifndef __ICertAdmin2_FWD_DEFINED__
#define __ICertAdmin2_FWD_DEFINED__
typedef interface ICertAdmin2 ICertAdmin2;

#endif 	/* __ICertAdmin2_FWD_DEFINED__ */


#ifndef __IOCSPProperty_FWD_DEFINED__
#define __IOCSPProperty_FWD_DEFINED__
typedef interface IOCSPProperty IOCSPProperty;

#endif 	/* __IOCSPProperty_FWD_DEFINED__ */


#ifndef __IOCSPPropertyCollection_FWD_DEFINED__
#define __IOCSPPropertyCollection_FWD_DEFINED__
typedef interface IOCSPPropertyCollection IOCSPPropertyCollection;

#endif 	/* __IOCSPPropertyCollection_FWD_DEFINED__ */


#ifndef __IOCSPCAConfiguration_FWD_DEFINED__
#define __IOCSPCAConfiguration_FWD_DEFINED__
typedef interface IOCSPCAConfiguration IOCSPCAConfiguration;

#endif 	/* __IOCSPCAConfiguration_FWD_DEFINED__ */


#ifndef __IOCSPCAConfigurationCollection_FWD_DEFINED__
#define __IOCSPCAConfigurationCollection_FWD_DEFINED__
typedef interface IOCSPCAConfigurationCollection IOCSPCAConfigurationCollection;

#endif 	/* __IOCSPCAConfigurationCollection_FWD_DEFINED__ */


#ifndef __IOCSPAdmin_FWD_DEFINED__
#define __IOCSPAdmin_FWD_DEFINED__
typedef interface IOCSPAdmin IOCSPAdmin;

#endif 	/* __IOCSPAdmin_FWD_DEFINED__ */


#ifndef __CCertAdmin_FWD_DEFINED__
#define __CCertAdmin_FWD_DEFINED__

#ifdef __cplusplus
typedef class CCertAdmin CCertAdmin;
#else
typedef struct CCertAdmin CCertAdmin;
#endif /* __cplusplus */

#endif 	/* __CCertAdmin_FWD_DEFINED__ */


#ifndef __CCertView_FWD_DEFINED__
#define __CCertView_FWD_DEFINED__

#ifdef __cplusplus
typedef class CCertView CCertView;
#else
typedef struct CCertView CCertView;
#endif /* __cplusplus */

#endif 	/* __CCertView_FWD_DEFINED__ */


#ifndef __OCSPPropertyCollection_FWD_DEFINED__
#define __OCSPPropertyCollection_FWD_DEFINED__

#ifdef __cplusplus
typedef class OCSPPropertyCollection OCSPPropertyCollection;
#else
typedef struct OCSPPropertyCollection OCSPPropertyCollection;
#endif /* __cplusplus */

#endif 	/* __OCSPPropertyCollection_FWD_DEFINED__ */


#ifndef __OCSPAdmin_FWD_DEFINED__
#define __OCSPAdmin_FWD_DEFINED__

#ifdef __cplusplus
typedef class OCSPAdmin OCSPAdmin;
#else
typedef struct OCSPAdmin OCSPAdmin;
#endif /* __cplusplus */

#endif 	/* __OCSPAdmin_FWD_DEFINED__ */


/* header files for imported files */
#include "wtypes.h"
#include "certview.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_certadm_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#define	CA_DISP_INCOMPLETE	( 0 )

#define	CA_DISP_ERROR	( 0x1 )

#define	CA_DISP_REVOKED	( 0x2 )

#define	CA_DISP_VALID	( 0x3 )

#define	CA_DISP_INVALID	( 0x4 )

#define	CA_DISP_UNDER_SUBMISSION	( 0x5 )

#define	KRA_DISP_EXPIRED	( 0 )

#define	KRA_DISP_NOTFOUND	( 0x1 )

#define	KRA_DISP_REVOKED	( 0x2 )

#define	KRA_DISP_VALID	( 0x3 )

#define	KRA_DISP_INVALID	( 0x4 )

#define	KRA_DISP_UNTRUSTED	( 0x5 )

#define	KRA_DISP_NOTLOADED	( 0x6 )

#define	CA_ACCESS_ADMIN	( 0x1 )

#define	CA_ACCESS_OFFICER	( 0x2 )

#define	CA_ACCESS_AUDITOR	( 0x4 )

#define	CA_ACCESS_OPERATOR	( 0x8 )

#define	CA_ACCESS_MASKROLES	( 0xff )

#define	CA_ACCESS_READ	( 0x100 )

#define	CA_ACCESS_ENROLL	( 0x200 )



extern RPC_IF_HANDLE __MIDL_itf_certadm_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_certadm_0000_0000_v0_0_s_ifspec;

#ifndef __ICertAdmin_INTERFACE_DEFINED__
#define __ICertAdmin_INTERFACE_DEFINED__

/* interface ICertAdmin */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_ICertAdmin;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("34df6950-7fb6-11d0-8817-00a0c903b83c")
    ICertAdmin : public IDispatch
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE IsValidCertificate( 
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [in] */ __RPC__in const BSTR strSerialNumber,
            /* [retval][out] */ __RPC__out LONG *pDisposition) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRevocationReason( 
            /* [retval][out] */ __RPC__out LONG *pReason) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RevokeCertificate( 
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [in] */ __RPC__in const BSTR strSerialNumber,
            /* [in] */ LONG Reason,
            /* [in] */ DATE Date) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetRequestAttributes( 
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [in] */ LONG RequestId,
            /* [in] */ __RPC__in const BSTR strAttributes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetCertificateExtension( 
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [in] */ LONG RequestId,
            /* [in] */ __RPC__in const BSTR strExtensionName,
            /* [in] */ LONG Type,
            /* [in] */ LONG Flags,
            /* [in] */ __RPC__in const VARIANT *pvarValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DenyRequest( 
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [in] */ LONG RequestId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ResubmitRequest( 
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [in] */ LONG RequestId,
            /* [retval][out] */ __RPC__out LONG *pDisposition) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE PublishCRL( 
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [in] */ DATE Date) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCRL( 
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [in] */ LONG Flags,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrCRL) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ImportCertificate( 
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [in] */ __RPC__in const BSTR strCertificate,
            /* [in] */ LONG Flags,
            /* [retval][out] */ __RPC__out LONG *pRequestId) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICertAdminVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICertAdmin * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICertAdmin * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICertAdmin * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ICertAdmin * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ICertAdmin * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ICertAdmin * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ICertAdmin * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(ICertAdmin, IsValidCertificate)
        HRESULT ( STDMETHODCALLTYPE *IsValidCertificate )( 
            __RPC__in ICertAdmin * This,
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [in] */ __RPC__in const BSTR strSerialNumber,
            /* [retval][out] */ __RPC__out LONG *pDisposition);
        
        DECLSPEC_XFGVIRT(ICertAdmin, GetRevocationReason)
        HRESULT ( STDMETHODCALLTYPE *GetRevocationReason )( 
            __RPC__in ICertAdmin * This,
            /* [retval][out] */ __RPC__out LONG *pReason);
        
        DECLSPEC_XFGVIRT(ICertAdmin, RevokeCertificate)
        HRESULT ( STDMETHODCALLTYPE *RevokeCertificate )( 
            __RPC__in ICertAdmin * This,
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [in] */ __RPC__in const BSTR strSerialNumber,
            /* [in] */ LONG Reason,
            /* [in] */ DATE Date);
        
        DECLSPEC_XFGVIRT(ICertAdmin, SetRequestAttributes)
        HRESULT ( STDMETHODCALLTYPE *SetRequestAttributes )( 
            __RPC__in ICertAdmin * This,
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [in] */ LONG RequestId,
            /* [in] */ __RPC__in const BSTR strAttributes);
        
        DECLSPEC_XFGVIRT(ICertAdmin, SetCertificateExtension)
        HRESULT ( STDMETHODCALLTYPE *SetCertificateExtension )( 
            __RPC__in ICertAdmin * This,
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [in] */ LONG RequestId,
            /* [in] */ __RPC__in const BSTR strExtensionName,
            /* [in] */ LONG Type,
            /* [in] */ LONG Flags,
            /* [in] */ __RPC__in const VARIANT *pvarValue);
        
        DECLSPEC_XFGVIRT(ICertAdmin, DenyRequest)
        HRESULT ( STDMETHODCALLTYPE *DenyRequest )( 
            __RPC__in ICertAdmin * This,
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [in] */ LONG RequestId);
        
        DECLSPEC_XFGVIRT(ICertAdmin, ResubmitRequest)
        HRESULT ( STDMETHODCALLTYPE *ResubmitRequest )( 
            __RPC__in ICertAdmin * This,
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [in] */ LONG RequestId,
            /* [retval][out] */ __RPC__out LONG *pDisposition);
        
        DECLSPEC_XFGVIRT(ICertAdmin, PublishCRL)
        HRESULT ( STDMETHODCALLTYPE *PublishCRL )( 
            __RPC__in ICertAdmin * This,
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [in] */ DATE Date);
        
        DECLSPEC_XFGVIRT(ICertAdmin, GetCRL)
        HRESULT ( STDMETHODCALLTYPE *GetCRL )( 
            __RPC__in ICertAdmin * This,
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [in] */ LONG Flags,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrCRL);
        
        DECLSPEC_XFGVIRT(ICertAdmin, ImportCertificate)
        HRESULT ( STDMETHODCALLTYPE *ImportCertificate )( 
            __RPC__in ICertAdmin * This,
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [in] */ __RPC__in const BSTR strCertificate,
            /* [in] */ LONG Flags,
            /* [retval][out] */ __RPC__out LONG *pRequestId);
        
        END_INTERFACE
    } ICertAdminVtbl;

    interface ICertAdmin
    {
        CONST_VTBL struct ICertAdminVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICertAdmin_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICertAdmin_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICertAdmin_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICertAdmin_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ICertAdmin_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ICertAdmin_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ICertAdmin_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ICertAdmin_IsValidCertificate(This,strConfig,strSerialNumber,pDisposition)	\
    ( (This)->lpVtbl -> IsValidCertificate(This,strConfig,strSerialNumber,pDisposition) ) 

#define ICertAdmin_GetRevocationReason(This,pReason)	\
    ( (This)->lpVtbl -> GetRevocationReason(This,pReason) ) 

#define ICertAdmin_RevokeCertificate(This,strConfig,strSerialNumber,Reason,Date)	\
    ( (This)->lpVtbl -> RevokeCertificate(This,strConfig,strSerialNumber,Reason,Date) ) 

#define ICertAdmin_SetRequestAttributes(This,strConfig,RequestId,strAttributes)	\
    ( (This)->lpVtbl -> SetRequestAttributes(This,strConfig,RequestId,strAttributes) ) 

#define ICertAdmin_SetCertificateExtension(This,strConfig,RequestId,strExtensionName,Type,Flags,pvarValue)	\
    ( (This)->lpVtbl -> SetCertificateExtension(This,strConfig,RequestId,strExtensionName,Type,Flags,pvarValue) ) 

#define ICertAdmin_DenyRequest(This,strConfig,RequestId)	\
    ( (This)->lpVtbl -> DenyRequest(This,strConfig,RequestId) ) 

#define ICertAdmin_ResubmitRequest(This,strConfig,RequestId,pDisposition)	\
    ( (This)->lpVtbl -> ResubmitRequest(This,strConfig,RequestId,pDisposition) ) 

#define ICertAdmin_PublishCRL(This,strConfig,Date)	\
    ( (This)->lpVtbl -> PublishCRL(This,strConfig,Date) ) 

#define ICertAdmin_GetCRL(This,strConfig,Flags,pstrCRL)	\
    ( (This)->lpVtbl -> GetCRL(This,strConfig,Flags,pstrCRL) ) 

#define ICertAdmin_ImportCertificate(This,strConfig,strCertificate,Flags,pRequestId)	\
    ( (This)->lpVtbl -> ImportCertificate(This,strConfig,strCertificate,Flags,pRequestId) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICertAdmin_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_certadm_0000_0001 */
/* [local] */ 

#define	CA_CRL_BASE	( 0x1 )

#define	CA_CRL_DELTA	( 0x2 )

#define	CA_CRL_REPUBLISH	( 0x10 )

#define	ICF_ALLOWFOREIGN	( 0x10000 )

#define	ICF_EXISTINGROW	( 0x20000 )

#define	IKF_OVERWRITE	( 0x10000 )

#define	CDR_EXPIRED	( 1 )

#define	CDR_REQUEST_LAST_CHANGED	( 2 )



extern RPC_IF_HANDLE __MIDL_itf_certadm_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_certadm_0000_0001_v0_0_s_ifspec;

#ifndef __ICertAdmin2_INTERFACE_DEFINED__
#define __ICertAdmin2_INTERFACE_DEFINED__

/* interface ICertAdmin2 */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_ICertAdmin2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f7c3ac41-b8ce-4fb4-aa58-3d1dc0e36b39")
    ICertAdmin2 : public ICertAdmin
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE PublishCRLs( 
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [in] */ DATE Date,
            /* [in] */ LONG CRLFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCAProperty( 
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [in] */ LONG PropId,
            /* [in] */ LONG PropIndex,
            /* [in] */ LONG PropType,
            /* [in] */ LONG Flags,
            /* [retval][out] */ __RPC__out VARIANT *pvarPropertyValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetCAProperty( 
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [in] */ LONG PropId,
            /* [in] */ LONG PropIndex,
            /* [in] */ LONG PropType,
            /* [in] */ __RPC__in VARIANT *pvarPropertyValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCAPropertyFlags( 
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [in] */ LONG PropId,
            /* [retval][out] */ __RPC__out LONG *pPropFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCAPropertyDisplayName( 
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [in] */ LONG PropId,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrDisplayName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetArchivedKey( 
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [in] */ LONG RequestId,
            /* [in] */ LONG Flags,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrArchivedKey) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetConfigEntry( 
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [in] */ __RPC__in const BSTR strNodePath,
            /* [in] */ __RPC__in const BSTR strEntryName,
            /* [retval][out] */ __RPC__out VARIANT *pvarEntry) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetConfigEntry( 
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [in] */ __RPC__in const BSTR strNodePath,
            /* [in] */ __RPC__in const BSTR strEntryName,
            /* [in] */ __RPC__in VARIANT *pvarEntry) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ImportKey( 
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [in] */ LONG RequestId,
            /* [in] */ __RPC__in const BSTR strCertHash,
            /* [in] */ LONG Flags,
            /* [in] */ __RPC__in const BSTR strKey) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMyRoles( 
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [retval][out] */ __RPC__out LONG *pRoles) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteRow( 
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [in] */ LONG Flags,
            /* [in] */ DATE Date,
            /* [in] */ LONG Table,
            /* [in] */ LONG RowId,
            /* [retval][out] */ __RPC__out LONG *pcDeleted) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICertAdmin2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICertAdmin2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICertAdmin2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICertAdmin2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ICertAdmin2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ICertAdmin2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ICertAdmin2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ICertAdmin2 * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(ICertAdmin, IsValidCertificate)
        HRESULT ( STDMETHODCALLTYPE *IsValidCertificate )( 
            __RPC__in ICertAdmin2 * This,
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [in] */ __RPC__in const BSTR strSerialNumber,
            /* [retval][out] */ __RPC__out LONG *pDisposition);
        
        DECLSPEC_XFGVIRT(ICertAdmin, GetRevocationReason)
        HRESULT ( STDMETHODCALLTYPE *GetRevocationReason )( 
            __RPC__in ICertAdmin2 * This,
            /* [retval][out] */ __RPC__out LONG *pReason);
        
        DECLSPEC_XFGVIRT(ICertAdmin, RevokeCertificate)
        HRESULT ( STDMETHODCALLTYPE *RevokeCertificate )( 
            __RPC__in ICertAdmin2 * This,
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [in] */ __RPC__in const BSTR strSerialNumber,
            /* [in] */ LONG Reason,
            /* [in] */ DATE Date);
        
        DECLSPEC_XFGVIRT(ICertAdmin, SetRequestAttributes)
        HRESULT ( STDMETHODCALLTYPE *SetRequestAttributes )( 
            __RPC__in ICertAdmin2 * This,
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [in] */ LONG RequestId,
            /* [in] */ __RPC__in const BSTR strAttributes);
        
        DECLSPEC_XFGVIRT(ICertAdmin, SetCertificateExtension)
        HRESULT ( STDMETHODCALLTYPE *SetCertificateExtension )( 
            __RPC__in ICertAdmin2 * This,
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [in] */ LONG RequestId,
            /* [in] */ __RPC__in const BSTR strExtensionName,
            /* [in] */ LONG Type,
            /* [in] */ LONG Flags,
            /* [in] */ __RPC__in const VARIANT *pvarValue);
        
        DECLSPEC_XFGVIRT(ICertAdmin, DenyRequest)
        HRESULT ( STDMETHODCALLTYPE *DenyRequest )( 
            __RPC__in ICertAdmin2 * This,
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [in] */ LONG RequestId);
        
        DECLSPEC_XFGVIRT(ICertAdmin, ResubmitRequest)
        HRESULT ( STDMETHODCALLTYPE *ResubmitRequest )( 
            __RPC__in ICertAdmin2 * This,
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [in] */ LONG RequestId,
            /* [retval][out] */ __RPC__out LONG *pDisposition);
        
        DECLSPEC_XFGVIRT(ICertAdmin, PublishCRL)
        HRESULT ( STDMETHODCALLTYPE *PublishCRL )( 
            __RPC__in ICertAdmin2 * This,
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [in] */ DATE Date);
        
        DECLSPEC_XFGVIRT(ICertAdmin, GetCRL)
        HRESULT ( STDMETHODCALLTYPE *GetCRL )( 
            __RPC__in ICertAdmin2 * This,
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [in] */ LONG Flags,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrCRL);
        
        DECLSPEC_XFGVIRT(ICertAdmin, ImportCertificate)
        HRESULT ( STDMETHODCALLTYPE *ImportCertificate )( 
            __RPC__in ICertAdmin2 * This,
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [in] */ __RPC__in const BSTR strCertificate,
            /* [in] */ LONG Flags,
            /* [retval][out] */ __RPC__out LONG *pRequestId);
        
        DECLSPEC_XFGVIRT(ICertAdmin2, PublishCRLs)
        HRESULT ( STDMETHODCALLTYPE *PublishCRLs )( 
            __RPC__in ICertAdmin2 * This,
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [in] */ DATE Date,
            /* [in] */ LONG CRLFlags);
        
        DECLSPEC_XFGVIRT(ICertAdmin2, GetCAProperty)
        HRESULT ( STDMETHODCALLTYPE *GetCAProperty )( 
            __RPC__in ICertAdmin2 * This,
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [in] */ LONG PropId,
            /* [in] */ LONG PropIndex,
            /* [in] */ LONG PropType,
            /* [in] */ LONG Flags,
            /* [retval][out] */ __RPC__out VARIANT *pvarPropertyValue);
        
        DECLSPEC_XFGVIRT(ICertAdmin2, SetCAProperty)
        HRESULT ( STDMETHODCALLTYPE *SetCAProperty )( 
            __RPC__in ICertAdmin2 * This,
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [in] */ LONG PropId,
            /* [in] */ LONG PropIndex,
            /* [in] */ LONG PropType,
            /* [in] */ __RPC__in VARIANT *pvarPropertyValue);
        
        DECLSPEC_XFGVIRT(ICertAdmin2, GetCAPropertyFlags)
        HRESULT ( STDMETHODCALLTYPE *GetCAPropertyFlags )( 
            __RPC__in ICertAdmin2 * This,
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [in] */ LONG PropId,
            /* [retval][out] */ __RPC__out LONG *pPropFlags);
        
        DECLSPEC_XFGVIRT(ICertAdmin2, GetCAPropertyDisplayName)
        HRESULT ( STDMETHODCALLTYPE *GetCAPropertyDisplayName )( 
            __RPC__in ICertAdmin2 * This,
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [in] */ LONG PropId,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrDisplayName);
        
        DECLSPEC_XFGVIRT(ICertAdmin2, GetArchivedKey)
        HRESULT ( STDMETHODCALLTYPE *GetArchivedKey )( 
            __RPC__in ICertAdmin2 * This,
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [in] */ LONG RequestId,
            /* [in] */ LONG Flags,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrArchivedKey);
        
        DECLSPEC_XFGVIRT(ICertAdmin2, GetConfigEntry)
        HRESULT ( STDMETHODCALLTYPE *GetConfigEntry )( 
            __RPC__in ICertAdmin2 * This,
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [in] */ __RPC__in const BSTR strNodePath,
            /* [in] */ __RPC__in const BSTR strEntryName,
            /* [retval][out] */ __RPC__out VARIANT *pvarEntry);
        
        DECLSPEC_XFGVIRT(ICertAdmin2, SetConfigEntry)
        HRESULT ( STDMETHODCALLTYPE *SetConfigEntry )( 
            __RPC__in ICertAdmin2 * This,
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [in] */ __RPC__in const BSTR strNodePath,
            /* [in] */ __RPC__in const BSTR strEntryName,
            /* [in] */ __RPC__in VARIANT *pvarEntry);
        
        DECLSPEC_XFGVIRT(ICertAdmin2, ImportKey)
        HRESULT ( STDMETHODCALLTYPE *ImportKey )( 
            __RPC__in ICertAdmin2 * This,
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [in] */ LONG RequestId,
            /* [in] */ __RPC__in const BSTR strCertHash,
            /* [in] */ LONG Flags,
            /* [in] */ __RPC__in const BSTR strKey);
        
        DECLSPEC_XFGVIRT(ICertAdmin2, GetMyRoles)
        HRESULT ( STDMETHODCALLTYPE *GetMyRoles )( 
            __RPC__in ICertAdmin2 * This,
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [retval][out] */ __RPC__out LONG *pRoles);
        
        DECLSPEC_XFGVIRT(ICertAdmin2, DeleteRow)
        HRESULT ( STDMETHODCALLTYPE *DeleteRow )( 
            __RPC__in ICertAdmin2 * This,
            /* [in] */ __RPC__in const BSTR strConfig,
            /* [in] */ LONG Flags,
            /* [in] */ DATE Date,
            /* [in] */ LONG Table,
            /* [in] */ LONG RowId,
            /* [retval][out] */ __RPC__out LONG *pcDeleted);
        
        END_INTERFACE
    } ICertAdmin2Vtbl;

    interface ICertAdmin2
    {
        CONST_VTBL struct ICertAdmin2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICertAdmin2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICertAdmin2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICertAdmin2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICertAdmin2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ICertAdmin2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ICertAdmin2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ICertAdmin2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ICertAdmin2_IsValidCertificate(This,strConfig,strSerialNumber,pDisposition)	\
    ( (This)->lpVtbl -> IsValidCertificate(This,strConfig,strSerialNumber,pDisposition) ) 

#define ICertAdmin2_GetRevocationReason(This,pReason)	\
    ( (This)->lpVtbl -> GetRevocationReason(This,pReason) ) 

#define ICertAdmin2_RevokeCertificate(This,strConfig,strSerialNumber,Reason,Date)	\
    ( (This)->lpVtbl -> RevokeCertificate(This,strConfig,strSerialNumber,Reason,Date) ) 

#define ICertAdmin2_SetRequestAttributes(This,strConfig,RequestId,strAttributes)	\
    ( (This)->lpVtbl -> SetRequestAttributes(This,strConfig,RequestId,strAttributes) ) 

#define ICertAdmin2_SetCertificateExtension(This,strConfig,RequestId,strExtensionName,Type,Flags,pvarValue)	\
    ( (This)->lpVtbl -> SetCertificateExtension(This,strConfig,RequestId,strExtensionName,Type,Flags,pvarValue) ) 

#define ICertAdmin2_DenyRequest(This,strConfig,RequestId)	\
    ( (This)->lpVtbl -> DenyRequest(This,strConfig,RequestId) ) 

#define ICertAdmin2_ResubmitRequest(This,strConfig,RequestId,pDisposition)	\
    ( (This)->lpVtbl -> ResubmitRequest(This,strConfig,RequestId,pDisposition) ) 

#define ICertAdmin2_PublishCRL(This,strConfig,Date)	\
    ( (This)->lpVtbl -> PublishCRL(This,strConfig,Date) ) 

#define ICertAdmin2_GetCRL(This,strConfig,Flags,pstrCRL)	\
    ( (This)->lpVtbl -> GetCRL(This,strConfig,Flags,pstrCRL) ) 

#define ICertAdmin2_ImportCertificate(This,strConfig,strCertificate,Flags,pRequestId)	\
    ( (This)->lpVtbl -> ImportCertificate(This,strConfig,strCertificate,Flags,pRequestId) ) 


#define ICertAdmin2_PublishCRLs(This,strConfig,Date,CRLFlags)	\
    ( (This)->lpVtbl -> PublishCRLs(This,strConfig,Date,CRLFlags) ) 

#define ICertAdmin2_GetCAProperty(This,strConfig,PropId,PropIndex,PropType,Flags,pvarPropertyValue)	\
    ( (This)->lpVtbl -> GetCAProperty(This,strConfig,PropId,PropIndex,PropType,Flags,pvarPropertyValue) ) 

#define ICertAdmin2_SetCAProperty(This,strConfig,PropId,PropIndex,PropType,pvarPropertyValue)	\
    ( (This)->lpVtbl -> SetCAProperty(This,strConfig,PropId,PropIndex,PropType,pvarPropertyValue) ) 

#define ICertAdmin2_GetCAPropertyFlags(This,strConfig,PropId,pPropFlags)	\
    ( (This)->lpVtbl -> GetCAPropertyFlags(This,strConfig,PropId,pPropFlags) ) 

#define ICertAdmin2_GetCAPropertyDisplayName(This,strConfig,PropId,pstrDisplayName)	\
    ( (This)->lpVtbl -> GetCAPropertyDisplayName(This,strConfig,PropId,pstrDisplayName) ) 

#define ICertAdmin2_GetArchivedKey(This,strConfig,RequestId,Flags,pstrArchivedKey)	\
    ( (This)->lpVtbl -> GetArchivedKey(This,strConfig,RequestId,Flags,pstrArchivedKey) ) 

#define ICertAdmin2_GetConfigEntry(This,strConfig,strNodePath,strEntryName,pvarEntry)	\
    ( (This)->lpVtbl -> GetConfigEntry(This,strConfig,strNodePath,strEntryName,pvarEntry) ) 

#define ICertAdmin2_SetConfigEntry(This,strConfig,strNodePath,strEntryName,pvarEntry)	\
    ( (This)->lpVtbl -> SetConfigEntry(This,strConfig,strNodePath,strEntryName,pvarEntry) ) 

#define ICertAdmin2_ImportKey(This,strConfig,RequestId,strCertHash,Flags,strKey)	\
    ( (This)->lpVtbl -> ImportKey(This,strConfig,RequestId,strCertHash,Flags,strKey) ) 

#define ICertAdmin2_GetMyRoles(This,strConfig,pRoles)	\
    ( (This)->lpVtbl -> GetMyRoles(This,strConfig,pRoles) ) 

#define ICertAdmin2_DeleteRow(This,strConfig,Flags,Date,Table,RowId,pcDeleted)	\
    ( (This)->lpVtbl -> DeleteRow(This,strConfig,Flags,Date,Table,RowId,pcDeleted) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICertAdmin2_INTERFACE_DEFINED__ */


#ifndef __IOCSPProperty_INTERFACE_DEFINED__
#define __IOCSPProperty_INTERFACE_DEFINED__

/* interface IOCSPProperty */
/* [unique][helpstring][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_IOCSPProperty;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("66FB7839-5F04-4C25-AD18-9FF1A8376EE0")
    IOCSPProperty : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Value( 
            /* [retval][out] */ __RPC__out VARIANT *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Value( 
            /* [in] */ VARIANT newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Modified( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOCSPPropertyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IOCSPProperty * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IOCSPProperty * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IOCSPProperty * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IOCSPProperty * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IOCSPProperty * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IOCSPProperty * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IOCSPProperty * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IOCSPProperty, get_Name)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IOCSPProperty * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IOCSPProperty, get_Value)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Value )( 
            __RPC__in IOCSPProperty * This,
            /* [retval][out] */ __RPC__out VARIANT *pVal);
        
        DECLSPEC_XFGVIRT(IOCSPProperty, put_Value)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Value )( 
            __RPC__in IOCSPProperty * This,
            /* [in] */ VARIANT newVal);
        
        DECLSPEC_XFGVIRT(IOCSPProperty, get_Modified)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Modified )( 
            __RPC__in IOCSPProperty * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pVal);
        
        END_INTERFACE
    } IOCSPPropertyVtbl;

    interface IOCSPProperty
    {
        CONST_VTBL struct IOCSPPropertyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOCSPProperty_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOCSPProperty_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOCSPProperty_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOCSPProperty_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IOCSPProperty_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IOCSPProperty_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IOCSPProperty_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IOCSPProperty_get_Name(This,pVal)	\
    ( (This)->lpVtbl -> get_Name(This,pVal) ) 

#define IOCSPProperty_get_Value(This,pVal)	\
    ( (This)->lpVtbl -> get_Value(This,pVal) ) 

#define IOCSPProperty_put_Value(This,newVal)	\
    ( (This)->lpVtbl -> put_Value(This,newVal) ) 

#define IOCSPProperty_get_Modified(This,pVal)	\
    ( (This)->lpVtbl -> get_Modified(This,pVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOCSPProperty_INTERFACE_DEFINED__ */


#ifndef __IOCSPPropertyCollection_INTERFACE_DEFINED__
#define __IOCSPPropertyCollection_INTERFACE_DEFINED__

/* interface IOCSPPropertyCollection */
/* [unique][helpstring][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_IOCSPPropertyCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2597C18D-54E6-4B74-9FA9-A6BFDA99CBBE")
    IOCSPPropertyCollection : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ LONG Index,
            /* [retval][out] */ __RPC__out VARIANT *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out LONG *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ItemByName( 
            /* [in] */ __RPC__in const BSTR bstrPropName,
            /* [retval][out] */ __RPC__out VARIANT *pVal) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateProperty( 
            /* [in] */ __RPC__in const BSTR bstrPropName,
            /* [in] */ __RPC__in const VARIANT *pVarPropValue,
            /* [retval][out] */ __RPC__deref_out_opt IOCSPProperty **ppVal) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE DeleteProperty( 
            /* [in] */ __RPC__in const BSTR bstrPropName) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE InitializeFromProperties( 
            /* [in] */ __RPC__in const VARIANT *pVarProperties) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetAllProperties( 
            /* [retval][out] */ __RPC__out VARIANT *pVarProperties) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOCSPPropertyCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IOCSPPropertyCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IOCSPPropertyCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IOCSPPropertyCollection * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IOCSPPropertyCollection * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IOCSPPropertyCollection * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IOCSPPropertyCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IOCSPPropertyCollection * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IOCSPPropertyCollection, get__NewEnum)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IOCSPPropertyCollection * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppVal);
        
        DECLSPEC_XFGVIRT(IOCSPPropertyCollection, get_Item)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IOCSPPropertyCollection * This,
            /* [in] */ LONG Index,
            /* [retval][out] */ __RPC__out VARIANT *pVal);
        
        DECLSPEC_XFGVIRT(IOCSPPropertyCollection, get_Count)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IOCSPPropertyCollection * This,
            /* [retval][out] */ __RPC__out LONG *pVal);
        
        DECLSPEC_XFGVIRT(IOCSPPropertyCollection, get_ItemByName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ItemByName )( 
            __RPC__in IOCSPPropertyCollection * This,
            /* [in] */ __RPC__in const BSTR bstrPropName,
            /* [retval][out] */ __RPC__out VARIANT *pVal);
        
        DECLSPEC_XFGVIRT(IOCSPPropertyCollection, CreateProperty)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateProperty )( 
            __RPC__in IOCSPPropertyCollection * This,
            /* [in] */ __RPC__in const BSTR bstrPropName,
            /* [in] */ __RPC__in const VARIANT *pVarPropValue,
            /* [retval][out] */ __RPC__deref_out_opt IOCSPProperty **ppVal);
        
        DECLSPEC_XFGVIRT(IOCSPPropertyCollection, DeleteProperty)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *DeleteProperty )( 
            __RPC__in IOCSPPropertyCollection * This,
            /* [in] */ __RPC__in const BSTR bstrPropName);
        
        DECLSPEC_XFGVIRT(IOCSPPropertyCollection, InitializeFromProperties)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *InitializeFromProperties )( 
            __RPC__in IOCSPPropertyCollection * This,
            /* [in] */ __RPC__in const VARIANT *pVarProperties);
        
        DECLSPEC_XFGVIRT(IOCSPPropertyCollection, GetAllProperties)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetAllProperties )( 
            __RPC__in IOCSPPropertyCollection * This,
            /* [retval][out] */ __RPC__out VARIANT *pVarProperties);
        
        END_INTERFACE
    } IOCSPPropertyCollectionVtbl;

    interface IOCSPPropertyCollection
    {
        CONST_VTBL struct IOCSPPropertyCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOCSPPropertyCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOCSPPropertyCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOCSPPropertyCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOCSPPropertyCollection_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IOCSPPropertyCollection_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IOCSPPropertyCollection_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IOCSPPropertyCollection_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IOCSPPropertyCollection_get__NewEnum(This,ppVal)	\
    ( (This)->lpVtbl -> get__NewEnum(This,ppVal) ) 

#define IOCSPPropertyCollection_get_Item(This,Index,pVal)	\
    ( (This)->lpVtbl -> get_Item(This,Index,pVal) ) 

#define IOCSPPropertyCollection_get_Count(This,pVal)	\
    ( (This)->lpVtbl -> get_Count(This,pVal) ) 

#define IOCSPPropertyCollection_get_ItemByName(This,bstrPropName,pVal)	\
    ( (This)->lpVtbl -> get_ItemByName(This,bstrPropName,pVal) ) 

#define IOCSPPropertyCollection_CreateProperty(This,bstrPropName,pVarPropValue,ppVal)	\
    ( (This)->lpVtbl -> CreateProperty(This,bstrPropName,pVarPropValue,ppVal) ) 

#define IOCSPPropertyCollection_DeleteProperty(This,bstrPropName)	\
    ( (This)->lpVtbl -> DeleteProperty(This,bstrPropName) ) 

#define IOCSPPropertyCollection_InitializeFromProperties(This,pVarProperties)	\
    ( (This)->lpVtbl -> InitializeFromProperties(This,pVarProperties) ) 

#define IOCSPPropertyCollection_GetAllProperties(This,pVarProperties)	\
    ( (This)->lpVtbl -> GetAllProperties(This,pVarProperties) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOCSPPropertyCollection_INTERFACE_DEFINED__ */


#ifndef __IOCSPCAConfiguration_INTERFACE_DEFINED__
#define __IOCSPCAConfiguration_INTERFACE_DEFINED__

/* interface IOCSPCAConfiguration */
/* [unique][helpstring][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_IOCSPCAConfiguration;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("AEC92B40-3D46-433F-87D1-B84D5C1E790D")
    IOCSPCAConfiguration : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Identifier( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CACertificate( 
            /* [retval][out] */ __RPC__out VARIANT *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_HashAlgorithm( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_HashAlgorithm( 
            /* [in] */ __RPC__in const BSTR newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SigningFlags( 
            /* [retval][out] */ __RPC__out ULONG *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_SigningFlags( 
            /* [in] */ ULONG newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SigningCertificate( 
            /* [retval][out] */ __RPC__out VARIANT *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_SigningCertificate( 
            /* [in] */ VARIANT newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ReminderDuration( 
            /* [retval][out] */ __RPC__out ULONG *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_ReminderDuration( 
            /* [in] */ ULONG newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ErrorCode( 
            /* [retval][out] */ __RPC__out ULONG *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CSPName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_KeySpec( 
            /* [retval][out] */ __RPC__out ULONG *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ProviderCLSID( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_ProviderCLSID( 
            /* [in] */ __RPC__in const BSTR newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ProviderProperties( 
            /* [retval][out] */ __RPC__out VARIANT *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_ProviderProperties( 
            /* [in] */ VARIANT newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Modified( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_LocalRevocationInformation( 
            /* [retval][out] */ __RPC__out VARIANT *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_LocalRevocationInformation( 
            /* [in] */ VARIANT newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SigningCertificateTemplate( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_SigningCertificateTemplate( 
            /* [in] */ __RPC__in const BSTR newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CAConfig( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_CAConfig( 
            /* [in] */ __RPC__in const BSTR newVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOCSPCAConfigurationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IOCSPCAConfiguration * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IOCSPCAConfiguration * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IOCSPCAConfiguration * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IOCSPCAConfiguration * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IOCSPCAConfiguration * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IOCSPCAConfiguration * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IOCSPCAConfiguration * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IOCSPCAConfiguration, get_Identifier)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Identifier )( 
            __RPC__in IOCSPCAConfiguration * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IOCSPCAConfiguration, get_CACertificate)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CACertificate )( 
            __RPC__in IOCSPCAConfiguration * This,
            /* [retval][out] */ __RPC__out VARIANT *pVal);
        
        DECLSPEC_XFGVIRT(IOCSPCAConfiguration, get_HashAlgorithm)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_HashAlgorithm )( 
            __RPC__in IOCSPCAConfiguration * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IOCSPCAConfiguration, put_HashAlgorithm)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_HashAlgorithm )( 
            __RPC__in IOCSPCAConfiguration * This,
            /* [in] */ __RPC__in const BSTR newVal);
        
        DECLSPEC_XFGVIRT(IOCSPCAConfiguration, get_SigningFlags)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SigningFlags )( 
            __RPC__in IOCSPCAConfiguration * This,
            /* [retval][out] */ __RPC__out ULONG *pVal);
        
        DECLSPEC_XFGVIRT(IOCSPCAConfiguration, put_SigningFlags)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SigningFlags )( 
            __RPC__in IOCSPCAConfiguration * This,
            /* [in] */ ULONG newVal);
        
        DECLSPEC_XFGVIRT(IOCSPCAConfiguration, get_SigningCertificate)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SigningCertificate )( 
            __RPC__in IOCSPCAConfiguration * This,
            /* [retval][out] */ __RPC__out VARIANT *pVal);
        
        DECLSPEC_XFGVIRT(IOCSPCAConfiguration, put_SigningCertificate)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SigningCertificate )( 
            __RPC__in IOCSPCAConfiguration * This,
            /* [in] */ VARIANT newVal);
        
        DECLSPEC_XFGVIRT(IOCSPCAConfiguration, get_ReminderDuration)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ReminderDuration )( 
            __RPC__in IOCSPCAConfiguration * This,
            /* [retval][out] */ __RPC__out ULONG *pVal);
        
        DECLSPEC_XFGVIRT(IOCSPCAConfiguration, put_ReminderDuration)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ReminderDuration )( 
            __RPC__in IOCSPCAConfiguration * This,
            /* [in] */ ULONG newVal);
        
        DECLSPEC_XFGVIRT(IOCSPCAConfiguration, get_ErrorCode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ErrorCode )( 
            __RPC__in IOCSPCAConfiguration * This,
            /* [retval][out] */ __RPC__out ULONG *pVal);
        
        DECLSPEC_XFGVIRT(IOCSPCAConfiguration, get_CSPName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CSPName )( 
            __RPC__in IOCSPCAConfiguration * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IOCSPCAConfiguration, get_KeySpec)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_KeySpec )( 
            __RPC__in IOCSPCAConfiguration * This,
            /* [retval][out] */ __RPC__out ULONG *pVal);
        
        DECLSPEC_XFGVIRT(IOCSPCAConfiguration, get_ProviderCLSID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProviderCLSID )( 
            __RPC__in IOCSPCAConfiguration * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IOCSPCAConfiguration, put_ProviderCLSID)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ProviderCLSID )( 
            __RPC__in IOCSPCAConfiguration * This,
            /* [in] */ __RPC__in const BSTR newVal);
        
        DECLSPEC_XFGVIRT(IOCSPCAConfiguration, get_ProviderProperties)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProviderProperties )( 
            __RPC__in IOCSPCAConfiguration * This,
            /* [retval][out] */ __RPC__out VARIANT *pVal);
        
        DECLSPEC_XFGVIRT(IOCSPCAConfiguration, put_ProviderProperties)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ProviderProperties )( 
            __RPC__in IOCSPCAConfiguration * This,
            /* [in] */ VARIANT newVal);
        
        DECLSPEC_XFGVIRT(IOCSPCAConfiguration, get_Modified)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Modified )( 
            __RPC__in IOCSPCAConfiguration * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pVal);
        
        DECLSPEC_XFGVIRT(IOCSPCAConfiguration, get_LocalRevocationInformation)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LocalRevocationInformation )( 
            __RPC__in IOCSPCAConfiguration * This,
            /* [retval][out] */ __RPC__out VARIANT *pVal);
        
        DECLSPEC_XFGVIRT(IOCSPCAConfiguration, put_LocalRevocationInformation)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_LocalRevocationInformation )( 
            __RPC__in IOCSPCAConfiguration * This,
            /* [in] */ VARIANT newVal);
        
        DECLSPEC_XFGVIRT(IOCSPCAConfiguration, get_SigningCertificateTemplate)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SigningCertificateTemplate )( 
            __RPC__in IOCSPCAConfiguration * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IOCSPCAConfiguration, put_SigningCertificateTemplate)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SigningCertificateTemplate )( 
            __RPC__in IOCSPCAConfiguration * This,
            /* [in] */ __RPC__in const BSTR newVal);
        
        DECLSPEC_XFGVIRT(IOCSPCAConfiguration, get_CAConfig)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CAConfig )( 
            __RPC__in IOCSPCAConfiguration * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IOCSPCAConfiguration, put_CAConfig)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_CAConfig )( 
            __RPC__in IOCSPCAConfiguration * This,
            /* [in] */ __RPC__in const BSTR newVal);
        
        END_INTERFACE
    } IOCSPCAConfigurationVtbl;

    interface IOCSPCAConfiguration
    {
        CONST_VTBL struct IOCSPCAConfigurationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOCSPCAConfiguration_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOCSPCAConfiguration_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOCSPCAConfiguration_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOCSPCAConfiguration_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IOCSPCAConfiguration_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IOCSPCAConfiguration_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IOCSPCAConfiguration_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IOCSPCAConfiguration_get_Identifier(This,pVal)	\
    ( (This)->lpVtbl -> get_Identifier(This,pVal) ) 

#define IOCSPCAConfiguration_get_CACertificate(This,pVal)	\
    ( (This)->lpVtbl -> get_CACertificate(This,pVal) ) 

#define IOCSPCAConfiguration_get_HashAlgorithm(This,pVal)	\
    ( (This)->lpVtbl -> get_HashAlgorithm(This,pVal) ) 

#define IOCSPCAConfiguration_put_HashAlgorithm(This,newVal)	\
    ( (This)->lpVtbl -> put_HashAlgorithm(This,newVal) ) 

#define IOCSPCAConfiguration_get_SigningFlags(This,pVal)	\
    ( (This)->lpVtbl -> get_SigningFlags(This,pVal) ) 

#define IOCSPCAConfiguration_put_SigningFlags(This,newVal)	\
    ( (This)->lpVtbl -> put_SigningFlags(This,newVal) ) 

#define IOCSPCAConfiguration_get_SigningCertificate(This,pVal)	\
    ( (This)->lpVtbl -> get_SigningCertificate(This,pVal) ) 

#define IOCSPCAConfiguration_put_SigningCertificate(This,newVal)	\
    ( (This)->lpVtbl -> put_SigningCertificate(This,newVal) ) 

#define IOCSPCAConfiguration_get_ReminderDuration(This,pVal)	\
    ( (This)->lpVtbl -> get_ReminderDuration(This,pVal) ) 

#define IOCSPCAConfiguration_put_ReminderDuration(This,newVal)	\
    ( (This)->lpVtbl -> put_ReminderDuration(This,newVal) ) 

#define IOCSPCAConfiguration_get_ErrorCode(This,pVal)	\
    ( (This)->lpVtbl -> get_ErrorCode(This,pVal) ) 

#define IOCSPCAConfiguration_get_CSPName(This,pVal)	\
    ( (This)->lpVtbl -> get_CSPName(This,pVal) ) 

#define IOCSPCAConfiguration_get_KeySpec(This,pVal)	\
    ( (This)->lpVtbl -> get_KeySpec(This,pVal) ) 

#define IOCSPCAConfiguration_get_ProviderCLSID(This,pVal)	\
    ( (This)->lpVtbl -> get_ProviderCLSID(This,pVal) ) 

#define IOCSPCAConfiguration_put_ProviderCLSID(This,newVal)	\
    ( (This)->lpVtbl -> put_ProviderCLSID(This,newVal) ) 

#define IOCSPCAConfiguration_get_ProviderProperties(This,pVal)	\
    ( (This)->lpVtbl -> get_ProviderProperties(This,pVal) ) 

#define IOCSPCAConfiguration_put_ProviderProperties(This,newVal)	\
    ( (This)->lpVtbl -> put_ProviderProperties(This,newVal) ) 

#define IOCSPCAConfiguration_get_Modified(This,pVal)	\
    ( (This)->lpVtbl -> get_Modified(This,pVal) ) 

#define IOCSPCAConfiguration_get_LocalRevocationInformation(This,pVal)	\
    ( (This)->lpVtbl -> get_LocalRevocationInformation(This,pVal) ) 

#define IOCSPCAConfiguration_put_LocalRevocationInformation(This,newVal)	\
    ( (This)->lpVtbl -> put_LocalRevocationInformation(This,newVal) ) 

#define IOCSPCAConfiguration_get_SigningCertificateTemplate(This,pVal)	\
    ( (This)->lpVtbl -> get_SigningCertificateTemplate(This,pVal) ) 

#define IOCSPCAConfiguration_put_SigningCertificateTemplate(This,newVal)	\
    ( (This)->lpVtbl -> put_SigningCertificateTemplate(This,newVal) ) 

#define IOCSPCAConfiguration_get_CAConfig(This,pVal)	\
    ( (This)->lpVtbl -> get_CAConfig(This,pVal) ) 

#define IOCSPCAConfiguration_put_CAConfig(This,newVal)	\
    ( (This)->lpVtbl -> put_CAConfig(This,newVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOCSPCAConfiguration_INTERFACE_DEFINED__ */


#ifndef __IOCSPCAConfigurationCollection_INTERFACE_DEFINED__
#define __IOCSPCAConfigurationCollection_INTERFACE_DEFINED__

/* interface IOCSPCAConfigurationCollection */
/* [unique][helpstring][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_IOCSPCAConfigurationCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2BEBEA0B-5ECE-4F28-A91C-86B4BB20F0D3")
    IOCSPCAConfigurationCollection : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ LONG Index,
            /* [retval][out] */ __RPC__out VARIANT *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out LONG *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ItemByName( 
            /* [in] */ __RPC__in const BSTR bstrIdentifier,
            /* [retval][out] */ __RPC__out VARIANT *pVal) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateCAConfiguration( 
            /* [in] */ __RPC__in const BSTR bstrIdentifier,
            /* [in] */ VARIANT varCACert,
            /* [retval][out] */ __RPC__deref_out_opt IOCSPCAConfiguration **ppVal) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE DeleteCAConfiguration( 
            /* [in] */ __RPC__in const BSTR bstrIdentifier) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOCSPCAConfigurationCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IOCSPCAConfigurationCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IOCSPCAConfigurationCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IOCSPCAConfigurationCollection * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IOCSPCAConfigurationCollection * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IOCSPCAConfigurationCollection * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IOCSPCAConfigurationCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IOCSPCAConfigurationCollection * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IOCSPCAConfigurationCollection, get__NewEnum)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IOCSPCAConfigurationCollection * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **pVal);
        
        DECLSPEC_XFGVIRT(IOCSPCAConfigurationCollection, get_Item)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IOCSPCAConfigurationCollection * This,
            /* [in] */ LONG Index,
            /* [retval][out] */ __RPC__out VARIANT *pVal);
        
        DECLSPEC_XFGVIRT(IOCSPCAConfigurationCollection, get_Count)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IOCSPCAConfigurationCollection * This,
            /* [retval][out] */ __RPC__out LONG *pVal);
        
        DECLSPEC_XFGVIRT(IOCSPCAConfigurationCollection, get_ItemByName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ItemByName )( 
            __RPC__in IOCSPCAConfigurationCollection * This,
            /* [in] */ __RPC__in const BSTR bstrIdentifier,
            /* [retval][out] */ __RPC__out VARIANT *pVal);
        
        DECLSPEC_XFGVIRT(IOCSPCAConfigurationCollection, CreateCAConfiguration)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateCAConfiguration )( 
            __RPC__in IOCSPCAConfigurationCollection * This,
            /* [in] */ __RPC__in const BSTR bstrIdentifier,
            /* [in] */ VARIANT varCACert,
            /* [retval][out] */ __RPC__deref_out_opt IOCSPCAConfiguration **ppVal);
        
        DECLSPEC_XFGVIRT(IOCSPCAConfigurationCollection, DeleteCAConfiguration)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *DeleteCAConfiguration )( 
            __RPC__in IOCSPCAConfigurationCollection * This,
            /* [in] */ __RPC__in const BSTR bstrIdentifier);
        
        END_INTERFACE
    } IOCSPCAConfigurationCollectionVtbl;

    interface IOCSPCAConfigurationCollection
    {
        CONST_VTBL struct IOCSPCAConfigurationCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOCSPCAConfigurationCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOCSPCAConfigurationCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOCSPCAConfigurationCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOCSPCAConfigurationCollection_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IOCSPCAConfigurationCollection_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IOCSPCAConfigurationCollection_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IOCSPCAConfigurationCollection_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IOCSPCAConfigurationCollection_get__NewEnum(This,pVal)	\
    ( (This)->lpVtbl -> get__NewEnum(This,pVal) ) 

#define IOCSPCAConfigurationCollection_get_Item(This,Index,pVal)	\
    ( (This)->lpVtbl -> get_Item(This,Index,pVal) ) 

#define IOCSPCAConfigurationCollection_get_Count(This,pVal)	\
    ( (This)->lpVtbl -> get_Count(This,pVal) ) 

#define IOCSPCAConfigurationCollection_get_ItemByName(This,bstrIdentifier,pVal)	\
    ( (This)->lpVtbl -> get_ItemByName(This,bstrIdentifier,pVal) ) 

#define IOCSPCAConfigurationCollection_CreateCAConfiguration(This,bstrIdentifier,varCACert,ppVal)	\
    ( (This)->lpVtbl -> CreateCAConfiguration(This,bstrIdentifier,varCACert,ppVal) ) 

#define IOCSPCAConfigurationCollection_DeleteCAConfiguration(This,bstrIdentifier)	\
    ( (This)->lpVtbl -> DeleteCAConfiguration(This,bstrIdentifier) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOCSPCAConfigurationCollection_INTERFACE_DEFINED__ */


#ifndef __IOCSPAdmin_INTERFACE_DEFINED__
#define __IOCSPAdmin_INTERFACE_DEFINED__

/* interface IOCSPAdmin */
/* [unique][helpstring][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_IOCSPAdmin;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("322E830D-67DB-4FE9-9577-4596D9F09294")
    IOCSPAdmin : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_OCSPServiceProperties( 
            /* [retval][out] */ __RPC__deref_out_opt IOCSPPropertyCollection **ppVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_OCSPCAConfigurationCollection( 
            /* [retval][out] */ __RPC__deref_out_opt IOCSPCAConfigurationCollection **pVal) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetConfiguration( 
            /* [in] */ __RPC__in const BSTR bstrServerName,
            /* [in] */ VARIANT_BOOL bForce) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetConfiguration( 
            /* [in] */ __RPC__in const BSTR bstrServerName,
            /* [in] */ VARIANT_BOOL bForce) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetMyRoles( 
            /* [in] */ __RPC__in const BSTR bstrServerName,
            /* [retval][out] */ __RPC__out LONG *pRoles) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Ping( 
            /* [in] */ __RPC__in const BSTR bstrServerName) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetSecurity( 
            /* [in] */ __RPC__in const BSTR bstrServerName,
            /* [in] */ __RPC__in const BSTR bstrVal) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetSecurity( 
            /* [in] */ __RPC__in const BSTR bstrServerName,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetSigningCertificates( 
            /* [in] */ __RPC__in const BSTR bstrServerName,
            /* [in] */ __RPC__in const VARIANT *pCACertVar,
            /* [retval][out] */ __RPC__out VARIANT *pVal) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetHashAlgorithms( 
            /* [in] */ __RPC__in const BSTR bstrServerName,
            /* [in] */ __RPC__in const BSTR bstrCAId,
            /* [retval][out] */ __RPC__out VARIANT *pVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOCSPAdminVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IOCSPAdmin * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IOCSPAdmin * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IOCSPAdmin * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IOCSPAdmin * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IOCSPAdmin * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IOCSPAdmin * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IOCSPAdmin * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IOCSPAdmin, get_OCSPServiceProperties)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OCSPServiceProperties )( 
            __RPC__in IOCSPAdmin * This,
            /* [retval][out] */ __RPC__deref_out_opt IOCSPPropertyCollection **ppVal);
        
        DECLSPEC_XFGVIRT(IOCSPAdmin, get_OCSPCAConfigurationCollection)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OCSPCAConfigurationCollection )( 
            __RPC__in IOCSPAdmin * This,
            /* [retval][out] */ __RPC__deref_out_opt IOCSPCAConfigurationCollection **pVal);
        
        DECLSPEC_XFGVIRT(IOCSPAdmin, GetConfiguration)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetConfiguration )( 
            __RPC__in IOCSPAdmin * This,
            /* [in] */ __RPC__in const BSTR bstrServerName,
            /* [in] */ VARIANT_BOOL bForce);
        
        DECLSPEC_XFGVIRT(IOCSPAdmin, SetConfiguration)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetConfiguration )( 
            __RPC__in IOCSPAdmin * This,
            /* [in] */ __RPC__in const BSTR bstrServerName,
            /* [in] */ VARIANT_BOOL bForce);
        
        DECLSPEC_XFGVIRT(IOCSPAdmin, GetMyRoles)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetMyRoles )( 
            __RPC__in IOCSPAdmin * This,
            /* [in] */ __RPC__in const BSTR bstrServerName,
            /* [retval][out] */ __RPC__out LONG *pRoles);
        
        DECLSPEC_XFGVIRT(IOCSPAdmin, Ping)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Ping )( 
            __RPC__in IOCSPAdmin * This,
            /* [in] */ __RPC__in const BSTR bstrServerName);
        
        DECLSPEC_XFGVIRT(IOCSPAdmin, SetSecurity)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetSecurity )( 
            __RPC__in IOCSPAdmin * This,
            /* [in] */ __RPC__in const BSTR bstrServerName,
            /* [in] */ __RPC__in const BSTR bstrVal);
        
        DECLSPEC_XFGVIRT(IOCSPAdmin, GetSecurity)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetSecurity )( 
            __RPC__in IOCSPAdmin * This,
            /* [in] */ __RPC__in const BSTR bstrServerName,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IOCSPAdmin, GetSigningCertificates)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetSigningCertificates )( 
            __RPC__in IOCSPAdmin * This,
            /* [in] */ __RPC__in const BSTR bstrServerName,
            /* [in] */ __RPC__in const VARIANT *pCACertVar,
            /* [retval][out] */ __RPC__out VARIANT *pVal);
        
        DECLSPEC_XFGVIRT(IOCSPAdmin, GetHashAlgorithms)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetHashAlgorithms )( 
            __RPC__in IOCSPAdmin * This,
            /* [in] */ __RPC__in const BSTR bstrServerName,
            /* [in] */ __RPC__in const BSTR bstrCAId,
            /* [retval][out] */ __RPC__out VARIANT *pVal);
        
        END_INTERFACE
    } IOCSPAdminVtbl;

    interface IOCSPAdmin
    {
        CONST_VTBL struct IOCSPAdminVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOCSPAdmin_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOCSPAdmin_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOCSPAdmin_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOCSPAdmin_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IOCSPAdmin_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IOCSPAdmin_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IOCSPAdmin_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IOCSPAdmin_get_OCSPServiceProperties(This,ppVal)	\
    ( (This)->lpVtbl -> get_OCSPServiceProperties(This,ppVal) ) 

#define IOCSPAdmin_get_OCSPCAConfigurationCollection(This,pVal)	\
    ( (This)->lpVtbl -> get_OCSPCAConfigurationCollection(This,pVal) ) 

#define IOCSPAdmin_GetConfiguration(This,bstrServerName,bForce)	\
    ( (This)->lpVtbl -> GetConfiguration(This,bstrServerName,bForce) ) 

#define IOCSPAdmin_SetConfiguration(This,bstrServerName,bForce)	\
    ( (This)->lpVtbl -> SetConfiguration(This,bstrServerName,bForce) ) 

#define IOCSPAdmin_GetMyRoles(This,bstrServerName,pRoles)	\
    ( (This)->lpVtbl -> GetMyRoles(This,bstrServerName,pRoles) ) 

#define IOCSPAdmin_Ping(This,bstrServerName)	\
    ( (This)->lpVtbl -> Ping(This,bstrServerName) ) 

#define IOCSPAdmin_SetSecurity(This,bstrServerName,bstrVal)	\
    ( (This)->lpVtbl -> SetSecurity(This,bstrServerName,bstrVal) ) 

#define IOCSPAdmin_GetSecurity(This,bstrServerName,pVal)	\
    ( (This)->lpVtbl -> GetSecurity(This,bstrServerName,pVal) ) 

#define IOCSPAdmin_GetSigningCertificates(This,bstrServerName,pCACertVar,pVal)	\
    ( (This)->lpVtbl -> GetSigningCertificates(This,bstrServerName,pCACertVar,pVal) ) 

#define IOCSPAdmin_GetHashAlgorithms(This,bstrServerName,bstrCAId,pVal)	\
    ( (This)->lpVtbl -> GetHashAlgorithms(This,bstrServerName,bstrCAId,pVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOCSPAdmin_INTERFACE_DEFINED__ */



#ifndef __CERTADMINLib_LIBRARY_DEFINED__
#define __CERTADMINLib_LIBRARY_DEFINED__

/* library CERTADMINLib */
/* [helpstring][version][uuid] */ 


enum OCSPSigningFlag
    {
        OCSP_SF_SILENT	= 0x1,
        OCSP_SF_USE_CACERT	= 0x2,
        OCSP_SF_ALLOW_SIGNINGCERT_AUTORENEWAL	= 0x4,
        OCSP_SF_FORCE_SIGNINGCERT_ISSUER_ISCA	= 0x8,
        OCSP_SF_AUTODISCOVER_SIGNINGCERT	= 0x10,
        OCSP_SF_MANUAL_ASSIGN_SIGNINGCERT	= 0x20,
        OCSP_SF_RESPONDER_ID_KEYHASH	= 0x40,
        OCSP_SF_RESPONDER_ID_NAME	= 0x80,
        OCSP_SF_ALLOW_NONCE_EXTENSION	= 0x100,
        OCSP_SF_ALLOW_SIGNINGCERT_AUTOENROLLMENT	= 0x200
    } ;

enum OCSPRequestFlag
    {
        OCSP_RF_REJECT_SIGNED_REQUESTS	= 0x1
    } ;
#define wszOCSPCAPROP_CACERTIFICATE		L"CACertificate"
#define wszOCSPCAPROP_HASHALGORITHMID		L"HashAlgorithmId"
#define wszOCSPCAPROP_SIGNINGFLAGS		L"SigningFlags"
#define wszOCSPCAPROP_REMINDERDURATION		L"ReminderDuration"
#define wszOCSPCAPROP_SIGNINGCERTIFICATE		L"SigningCertificate"
#define wszOCSPCAPROP_CSPNAME			L"CSPName"
#define wszOCSPCAPROP_KEYSPEC			L"KeySpec"
#define wszOCSPCAPROP_ERRORCODE			L"ErrorCode"
#define wszOCSPCAPROP_PROVIDERCLSID		L"ProviderCLSID"
#define wszOCSPCAPROP_PROVIDERPROPERTIES		L"Provider"
#define wszOCSPCAPROP_LOCALREVOCATIONINFORMATION	L"LocalRevocationInformation"
#define wszOCSPCAPROP_SIGNINGCERTIFICATETEMPLATE	L"SigningCertificateTemplate"
#define wszOCSPCAPROP_CACONFIG	L"CAConfig"
#define wszOCSPPROP_LOGLEVEL  			L"LogLevel"
#define wszOCSPPROP_DEBUG                  	L"Debug"
#define wszOCSPPROP_AUDITFILTER			L"AuditFilter"
#define wszOCSPPROP_ARRAYCONTROLLER		L"ArrayController"
#define wszOCSPPROP_ARRAYMEMBERS		    L"ArrayMembers"
#define wszOCSPPROP_ENROLLPOLLINTERVAL   L"EnrollPollInterval"
#define wszOCSPISAPIPROP_VIRTUALROOTNAME		L"VirtualRootName"
#define wszOCSPISAPIPROP_NUMOFTHREADS		L"NumOfThreads"
#define wszOCSPISAPIPROP_NUMOFBACKENDCONNECTIONS	L"NumOfBackendConnections"
#define wszOCSPISAPIPROP_REFRESHRATE		L"RefreshRate"
#define wszOCSPISAPIPROP_MAXNUMOFCACHEENTRIES	L"MaxNumOfCacheEntries"
#define wszOCSPISAPIPROP_MAXAGE			L"MaxAge"
#define wszOCSPISAPIPROP_DEBUG			L"ISAPIDebug"
#define wszOCSPCOMMONPROP_REQFLAGS			        L"RequestFlags"
#define wszOCSPCOMMONPROP_MAXINCOMINGMESSAGESIZE	    L"MaxIncomingMessageSize"
#define wszOCSPCOMMONPROP_MAXNUMOFREQUESTENTRIES	    L"MaxNumOfRequestEntries"
#define wszOCSPREVPROP_CRLURLTIMEOUT		L"CrlUrlTimeOut"
#define wszOCSPREVPROP_BASECRLURLS		L"BaseCrlUrls"
#define wszOCSPREVPROP_SERIALNUMBERSDIRS	L"IssuedSerialNumbersDirectories"
#define wszOCSPREVPROP_BASECRL			L"BaseCrl"
#define wszOCSPREVPROP_DELTACRLURLS		L"DeltaCrlUrls"
#define wszOCSPREVPROP_DELTACRL			L"DeltaCrl"
#define wszOCSPREVPROP_REFRESHTIMEOUT	L"RefreshTimeOut"
#define wszOCSPREVPROP_ERRORCODE			L"RevocationErrorCode"
#define wszOCSPREVPROP_ALLOWUSERONLYCRLS	L"AllowUserOnlyCrls"
#define wszOCSPREVPROP_ALLOWCAONLYCRLS	L"AllowCAOnlyCrls"

EXTERN_C const IID LIBID_CERTADMINLib;

EXTERN_C const CLSID CLSID_CCertAdmin;

#ifdef __cplusplus

class DECLSPEC_UUID("37eabaf0-7fb6-11d0-8817-00a0c903b83c")
CCertAdmin;
#endif

EXTERN_C const CLSID CLSID_CCertView;

#ifdef __cplusplus

class DECLSPEC_UUID("a12d0f7a-1e84-11d1-9bd6-00c04fb683fa")
CCertView;
#endif

EXTERN_C const CLSID CLSID_OCSPPropertyCollection;

#ifdef __cplusplus

class DECLSPEC_UUID("F935A528-BA8A-4DD9-BA79-F283275CB2DE")
OCSPPropertyCollection;
#endif

EXTERN_C const CLSID CLSID_OCSPAdmin;

#ifdef __cplusplus

class DECLSPEC_UUID("D3F73511-92C9-47CB-8FF2-8D891A7C4DE4")
OCSPAdmin;
#endif
#endif /* __CERTADMINLib_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_certadm_0000_0008 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_certadm_0000_0008_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_certadm_0000_0008_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  BSTR_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  VARIANT_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

unsigned long             __RPC_USER  BSTR_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree64(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  VARIANT_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree64(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


