

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

#ifndef __faxcom_h__
#define __faxcom_h__

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

#ifndef __IFaxTiff_FWD_DEFINED__
#define __IFaxTiff_FWD_DEFINED__
typedef interface IFaxTiff IFaxTiff;

#endif 	/* __IFaxTiff_FWD_DEFINED__ */


#ifndef __IFaxServer_FWD_DEFINED__
#define __IFaxServer_FWD_DEFINED__
typedef interface IFaxServer IFaxServer;

#endif 	/* __IFaxServer_FWD_DEFINED__ */


#ifndef __IFaxPort_FWD_DEFINED__
#define __IFaxPort_FWD_DEFINED__
typedef interface IFaxPort IFaxPort;

#endif 	/* __IFaxPort_FWD_DEFINED__ */


#ifndef __IFaxPorts_FWD_DEFINED__
#define __IFaxPorts_FWD_DEFINED__
typedef interface IFaxPorts IFaxPorts;

#endif 	/* __IFaxPorts_FWD_DEFINED__ */


#ifndef __IFaxRoutingMethod_FWD_DEFINED__
#define __IFaxRoutingMethod_FWD_DEFINED__
typedef interface IFaxRoutingMethod IFaxRoutingMethod;

#endif 	/* __IFaxRoutingMethod_FWD_DEFINED__ */


#ifndef __IFaxRoutingMethods_FWD_DEFINED__
#define __IFaxRoutingMethods_FWD_DEFINED__
typedef interface IFaxRoutingMethods IFaxRoutingMethods;

#endif 	/* __IFaxRoutingMethods_FWD_DEFINED__ */


#ifndef __IFaxStatus_FWD_DEFINED__
#define __IFaxStatus_FWD_DEFINED__
typedef interface IFaxStatus IFaxStatus;

#endif 	/* __IFaxStatus_FWD_DEFINED__ */


#ifndef __IFaxDoc_FWD_DEFINED__
#define __IFaxDoc_FWD_DEFINED__
typedef interface IFaxDoc IFaxDoc;

#endif 	/* __IFaxDoc_FWD_DEFINED__ */


#ifndef __IFaxJobs_FWD_DEFINED__
#define __IFaxJobs_FWD_DEFINED__
typedef interface IFaxJobs IFaxJobs;

#endif 	/* __IFaxJobs_FWD_DEFINED__ */


#ifndef __IFaxJob_FWD_DEFINED__
#define __IFaxJob_FWD_DEFINED__
typedef interface IFaxJob IFaxJob;

#endif 	/* __IFaxJob_FWD_DEFINED__ */


#ifndef __FaxTiff_FWD_DEFINED__
#define __FaxTiff_FWD_DEFINED__

#ifdef __cplusplus
typedef class FaxTiff FaxTiff;
#else
typedef struct FaxTiff FaxTiff;
#endif /* __cplusplus */

#endif 	/* __FaxTiff_FWD_DEFINED__ */


#ifndef __FaxServer_FWD_DEFINED__
#define __FaxServer_FWD_DEFINED__

#ifdef __cplusplus
typedef class FaxServer FaxServer;
#else
typedef struct FaxServer FaxServer;
#endif /* __cplusplus */

#endif 	/* __FaxServer_FWD_DEFINED__ */


#ifndef __FaxPort_FWD_DEFINED__
#define __FaxPort_FWD_DEFINED__

#ifdef __cplusplus
typedef class FaxPort FaxPort;
#else
typedef struct FaxPort FaxPort;
#endif /* __cplusplus */

#endif 	/* __FaxPort_FWD_DEFINED__ */


#ifndef __FaxPorts_FWD_DEFINED__
#define __FaxPorts_FWD_DEFINED__

#ifdef __cplusplus
typedef class FaxPorts FaxPorts;
#else
typedef struct FaxPorts FaxPorts;
#endif /* __cplusplus */

#endif 	/* __FaxPorts_FWD_DEFINED__ */


#ifndef __FaxRoutingMethod_FWD_DEFINED__
#define __FaxRoutingMethod_FWD_DEFINED__

#ifdef __cplusplus
typedef class FaxRoutingMethod FaxRoutingMethod;
#else
typedef struct FaxRoutingMethod FaxRoutingMethod;
#endif /* __cplusplus */

#endif 	/* __FaxRoutingMethod_FWD_DEFINED__ */


#ifndef __FaxRoutingMethods_FWD_DEFINED__
#define __FaxRoutingMethods_FWD_DEFINED__

#ifdef __cplusplus
typedef class FaxRoutingMethods FaxRoutingMethods;
#else
typedef struct FaxRoutingMethods FaxRoutingMethods;
#endif /* __cplusplus */

#endif 	/* __FaxRoutingMethods_FWD_DEFINED__ */


#ifndef __FaxStatus_FWD_DEFINED__
#define __FaxStatus_FWD_DEFINED__

#ifdef __cplusplus
typedef class FaxStatus FaxStatus;
#else
typedef struct FaxStatus FaxStatus;
#endif /* __cplusplus */

#endif 	/* __FaxStatus_FWD_DEFINED__ */


#ifndef __FaxDoc_FWD_DEFINED__
#define __FaxDoc_FWD_DEFINED__

#ifdef __cplusplus
typedef class FaxDoc FaxDoc;
#else
typedef struct FaxDoc FaxDoc;
#endif /* __cplusplus */

#endif 	/* __FaxDoc_FWD_DEFINED__ */


#ifndef __FaxJobs_FWD_DEFINED__
#define __FaxJobs_FWD_DEFINED__

#ifdef __cplusplus
typedef class FaxJobs FaxJobs;
#else
typedef struct FaxJobs FaxJobs;
#endif /* __cplusplus */

#endif 	/* __FaxJobs_FWD_DEFINED__ */


#ifndef __FaxJob_FWD_DEFINED__
#define __FaxJob_FWD_DEFINED__

#ifdef __cplusplus
typedef class FaxJob FaxJob;
#else
typedef struct FaxJob FaxJob;
#endif /* __cplusplus */

#endif 	/* __FaxJob_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_faxcom_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_faxcom_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_faxcom_0000_0000_v0_0_s_ifspec;

#ifndef __IFaxTiff_INTERFACE_DEFINED__
#define __IFaxTiff_INTERFACE_DEFINED__

/* interface IFaxTiff */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IFaxTiff;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("b19bb45f-b91c-11d1-83e1-00c04fb6e984")
    IFaxTiff : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ReceiveTime( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Image( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Image( 
            /* [in] */ __RPC__in BSTR newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RecipientName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SenderName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Routing( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CallerId( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Csid( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Tsid( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RecipientNumber( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RawReceiveTime( 
            /* [retval][out] */ __RPC__out VARIANT *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TiffTagString( 
            /* [in] */ int tagID,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFaxTiffVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFaxTiff * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFaxTiff * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFaxTiff * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFaxTiff * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFaxTiff * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFaxTiff * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFaxTiff * This,
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
        
        DECLSPEC_XFGVIRT(IFaxTiff, get_ReceiveTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ReceiveTime )( 
            __RPC__in IFaxTiff * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFaxTiff, get_Image)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Image )( 
            __RPC__in IFaxTiff * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFaxTiff, put_Image)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Image )( 
            __RPC__in IFaxTiff * This,
            /* [in] */ __RPC__in BSTR newVal);
        
        DECLSPEC_XFGVIRT(IFaxTiff, get_RecipientName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RecipientName )( 
            __RPC__in IFaxTiff * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFaxTiff, get_SenderName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SenderName )( 
            __RPC__in IFaxTiff * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFaxTiff, get_Routing)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Routing )( 
            __RPC__in IFaxTiff * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFaxTiff, get_CallerId)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CallerId )( 
            __RPC__in IFaxTiff * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFaxTiff, get_Csid)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Csid )( 
            __RPC__in IFaxTiff * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFaxTiff, get_Tsid)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Tsid )( 
            __RPC__in IFaxTiff * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFaxTiff, get_RecipientNumber)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RecipientNumber )( 
            __RPC__in IFaxTiff * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFaxTiff, get_RawReceiveTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RawReceiveTime )( 
            __RPC__in IFaxTiff * This,
            /* [retval][out] */ __RPC__out VARIANT *pVal);
        
        DECLSPEC_XFGVIRT(IFaxTiff, get_TiffTagString)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TiffTagString )( 
            __RPC__in IFaxTiff * This,
            /* [in] */ int tagID,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        END_INTERFACE
    } IFaxTiffVtbl;

    interface IFaxTiff
    {
        CONST_VTBL struct IFaxTiffVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFaxTiff_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFaxTiff_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFaxTiff_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFaxTiff_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFaxTiff_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFaxTiff_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFaxTiff_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFaxTiff_get_ReceiveTime(This,pVal)	\
    ( (This)->lpVtbl -> get_ReceiveTime(This,pVal) ) 

#define IFaxTiff_get_Image(This,pVal)	\
    ( (This)->lpVtbl -> get_Image(This,pVal) ) 

#define IFaxTiff_put_Image(This,newVal)	\
    ( (This)->lpVtbl -> put_Image(This,newVal) ) 

#define IFaxTiff_get_RecipientName(This,pVal)	\
    ( (This)->lpVtbl -> get_RecipientName(This,pVal) ) 

#define IFaxTiff_get_SenderName(This,pVal)	\
    ( (This)->lpVtbl -> get_SenderName(This,pVal) ) 

#define IFaxTiff_get_Routing(This,pVal)	\
    ( (This)->lpVtbl -> get_Routing(This,pVal) ) 

#define IFaxTiff_get_CallerId(This,pVal)	\
    ( (This)->lpVtbl -> get_CallerId(This,pVal) ) 

#define IFaxTiff_get_Csid(This,pVal)	\
    ( (This)->lpVtbl -> get_Csid(This,pVal) ) 

#define IFaxTiff_get_Tsid(This,pVal)	\
    ( (This)->lpVtbl -> get_Tsid(This,pVal) ) 

#define IFaxTiff_get_RecipientNumber(This,pVal)	\
    ( (This)->lpVtbl -> get_RecipientNumber(This,pVal) ) 

#define IFaxTiff_get_RawReceiveTime(This,pVal)	\
    ( (This)->lpVtbl -> get_RawReceiveTime(This,pVal) ) 

#define IFaxTiff_get_TiffTagString(This,tagID,pVal)	\
    ( (This)->lpVtbl -> get_TiffTagString(This,tagID,pVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFaxTiff_INTERFACE_DEFINED__ */


#ifndef __IFaxServer_INTERFACE_DEFINED__
#define __IFaxServer_INTERFACE_DEFINED__

/* interface IFaxServer */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IFaxServer;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D73733C7-CC80-11D0-B225-00C04FB6C2F5")
    IFaxServer : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Connect( 
            __RPC__in BSTR ServerName) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Disconnect( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetPorts( 
            /* [retval][out] */ __RPC__out VARIANT *retval) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateDocument( 
            __RPC__in BSTR FileName,
            /* [retval][out] */ __RPC__out VARIANT *retval) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetJobs( 
            /* [retval][out] */ __RPC__out VARIANT *retval) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Retries( 
            /* [retval][out] */ __RPC__out long *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Retries( 
            /* [in] */ long newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RetryDelay( 
            /* [retval][out] */ __RPC__out long *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_RetryDelay( 
            /* [in] */ long newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DirtyDays( 
            /* [retval][out] */ __RPC__out long *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_DirtyDays( 
            /* [in] */ long newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Branding( 
            /* [retval][out] */ __RPC__out BOOL *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Branding( 
            /* [in] */ BOOL newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_UseDeviceTsid( 
            /* [retval][out] */ __RPC__out BOOL *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_UseDeviceTsid( 
            /* [in] */ BOOL newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ServerCoverpage( 
            /* [retval][out] */ __RPC__out BOOL *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_ServerCoverpage( 
            /* [in] */ BOOL newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PauseServerQueue( 
            /* [retval][out] */ __RPC__out BOOL *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_PauseServerQueue( 
            /* [in] */ BOOL newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ArchiveOutboundFaxes( 
            /* [retval][out] */ __RPC__out BOOL *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_ArchiveOutboundFaxes( 
            /* [in] */ BOOL newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ArchiveDirectory( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_ArchiveDirectory( 
            /* [in] */ __RPC__in BSTR newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ServerMapiProfile( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_ServerMapiProfile( 
            /* [in] */ __RPC__in BSTR newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DiscountRateStartHour( 
            /* [retval][out] */ __RPC__out short *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_DiscountRateStartHour( 
            /* [in] */ short newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DiscountRateStartMinute( 
            /* [retval][out] */ __RPC__out short *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_DiscountRateStartMinute( 
            /* [in] */ short newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DiscountRateEndHour( 
            /* [retval][out] */ __RPC__out short *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_DiscountRateEndHour( 
            /* [in] */ short newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DiscountRateEndMinute( 
            /* [retval][out] */ __RPC__out short *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_DiscountRateEndMinute( 
            /* [in] */ short newVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFaxServerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFaxServer * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFaxServer * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFaxServer * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFaxServer * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFaxServer * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFaxServer * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFaxServer * This,
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
        
        DECLSPEC_XFGVIRT(IFaxServer, Connect)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Connect )( 
            __RPC__in IFaxServer * This,
            __RPC__in BSTR ServerName);
        
        DECLSPEC_XFGVIRT(IFaxServer, Disconnect)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Disconnect )( 
            __RPC__in IFaxServer * This);
        
        DECLSPEC_XFGVIRT(IFaxServer, GetPorts)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetPorts )( 
            __RPC__in IFaxServer * This,
            /* [retval][out] */ __RPC__out VARIANT *retval);
        
        DECLSPEC_XFGVIRT(IFaxServer, CreateDocument)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateDocument )( 
            __RPC__in IFaxServer * This,
            __RPC__in BSTR FileName,
            /* [retval][out] */ __RPC__out VARIANT *retval);
        
        DECLSPEC_XFGVIRT(IFaxServer, GetJobs)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetJobs )( 
            __RPC__in IFaxServer * This,
            /* [retval][out] */ __RPC__out VARIANT *retval);
        
        DECLSPEC_XFGVIRT(IFaxServer, get_Retries)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Retries )( 
            __RPC__in IFaxServer * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IFaxServer, put_Retries)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Retries )( 
            __RPC__in IFaxServer * This,
            /* [in] */ long newVal);
        
        DECLSPEC_XFGVIRT(IFaxServer, get_RetryDelay)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RetryDelay )( 
            __RPC__in IFaxServer * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IFaxServer, put_RetryDelay)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_RetryDelay )( 
            __RPC__in IFaxServer * This,
            /* [in] */ long newVal);
        
        DECLSPEC_XFGVIRT(IFaxServer, get_DirtyDays)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DirtyDays )( 
            __RPC__in IFaxServer * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IFaxServer, put_DirtyDays)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DirtyDays )( 
            __RPC__in IFaxServer * This,
            /* [in] */ long newVal);
        
        DECLSPEC_XFGVIRT(IFaxServer, get_Branding)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Branding )( 
            __RPC__in IFaxServer * This,
            /* [retval][out] */ __RPC__out BOOL *pVal);
        
        DECLSPEC_XFGVIRT(IFaxServer, put_Branding)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Branding )( 
            __RPC__in IFaxServer * This,
            /* [in] */ BOOL newVal);
        
        DECLSPEC_XFGVIRT(IFaxServer, get_UseDeviceTsid)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UseDeviceTsid )( 
            __RPC__in IFaxServer * This,
            /* [retval][out] */ __RPC__out BOOL *pVal);
        
        DECLSPEC_XFGVIRT(IFaxServer, put_UseDeviceTsid)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_UseDeviceTsid )( 
            __RPC__in IFaxServer * This,
            /* [in] */ BOOL newVal);
        
        DECLSPEC_XFGVIRT(IFaxServer, get_ServerCoverpage)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ServerCoverpage )( 
            __RPC__in IFaxServer * This,
            /* [retval][out] */ __RPC__out BOOL *pVal);
        
        DECLSPEC_XFGVIRT(IFaxServer, put_ServerCoverpage)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ServerCoverpage )( 
            __RPC__in IFaxServer * This,
            /* [in] */ BOOL newVal);
        
        DECLSPEC_XFGVIRT(IFaxServer, get_PauseServerQueue)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PauseServerQueue )( 
            __RPC__in IFaxServer * This,
            /* [retval][out] */ __RPC__out BOOL *pVal);
        
        DECLSPEC_XFGVIRT(IFaxServer, put_PauseServerQueue)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PauseServerQueue )( 
            __RPC__in IFaxServer * This,
            /* [in] */ BOOL newVal);
        
        DECLSPEC_XFGVIRT(IFaxServer, get_ArchiveOutboundFaxes)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ArchiveOutboundFaxes )( 
            __RPC__in IFaxServer * This,
            /* [retval][out] */ __RPC__out BOOL *pVal);
        
        DECLSPEC_XFGVIRT(IFaxServer, put_ArchiveOutboundFaxes)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ArchiveOutboundFaxes )( 
            __RPC__in IFaxServer * This,
            /* [in] */ BOOL newVal);
        
        DECLSPEC_XFGVIRT(IFaxServer, get_ArchiveDirectory)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ArchiveDirectory )( 
            __RPC__in IFaxServer * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFaxServer, put_ArchiveDirectory)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ArchiveDirectory )( 
            __RPC__in IFaxServer * This,
            /* [in] */ __RPC__in BSTR newVal);
        
        DECLSPEC_XFGVIRT(IFaxServer, get_ServerMapiProfile)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ServerMapiProfile )( 
            __RPC__in IFaxServer * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFaxServer, put_ServerMapiProfile)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ServerMapiProfile )( 
            __RPC__in IFaxServer * This,
            /* [in] */ __RPC__in BSTR newVal);
        
        DECLSPEC_XFGVIRT(IFaxServer, get_DiscountRateStartHour)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DiscountRateStartHour )( 
            __RPC__in IFaxServer * This,
            /* [retval][out] */ __RPC__out short *pVal);
        
        DECLSPEC_XFGVIRT(IFaxServer, put_DiscountRateStartHour)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DiscountRateStartHour )( 
            __RPC__in IFaxServer * This,
            /* [in] */ short newVal);
        
        DECLSPEC_XFGVIRT(IFaxServer, get_DiscountRateStartMinute)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DiscountRateStartMinute )( 
            __RPC__in IFaxServer * This,
            /* [retval][out] */ __RPC__out short *pVal);
        
        DECLSPEC_XFGVIRT(IFaxServer, put_DiscountRateStartMinute)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DiscountRateStartMinute )( 
            __RPC__in IFaxServer * This,
            /* [in] */ short newVal);
        
        DECLSPEC_XFGVIRT(IFaxServer, get_DiscountRateEndHour)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DiscountRateEndHour )( 
            __RPC__in IFaxServer * This,
            /* [retval][out] */ __RPC__out short *pVal);
        
        DECLSPEC_XFGVIRT(IFaxServer, put_DiscountRateEndHour)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DiscountRateEndHour )( 
            __RPC__in IFaxServer * This,
            /* [in] */ short newVal);
        
        DECLSPEC_XFGVIRT(IFaxServer, get_DiscountRateEndMinute)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DiscountRateEndMinute )( 
            __RPC__in IFaxServer * This,
            /* [retval][out] */ __RPC__out short *pVal);
        
        DECLSPEC_XFGVIRT(IFaxServer, put_DiscountRateEndMinute)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DiscountRateEndMinute )( 
            __RPC__in IFaxServer * This,
            /* [in] */ short newVal);
        
        END_INTERFACE
    } IFaxServerVtbl;

    interface IFaxServer
    {
        CONST_VTBL struct IFaxServerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFaxServer_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFaxServer_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFaxServer_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFaxServer_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFaxServer_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFaxServer_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFaxServer_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFaxServer_Connect(This,ServerName)	\
    ( (This)->lpVtbl -> Connect(This,ServerName) ) 

#define IFaxServer_Disconnect(This)	\
    ( (This)->lpVtbl -> Disconnect(This) ) 

#define IFaxServer_GetPorts(This,retval)	\
    ( (This)->lpVtbl -> GetPorts(This,retval) ) 

#define IFaxServer_CreateDocument(This,FileName,retval)	\
    ( (This)->lpVtbl -> CreateDocument(This,FileName,retval) ) 

#define IFaxServer_GetJobs(This,retval)	\
    ( (This)->lpVtbl -> GetJobs(This,retval) ) 

#define IFaxServer_get_Retries(This,pVal)	\
    ( (This)->lpVtbl -> get_Retries(This,pVal) ) 

#define IFaxServer_put_Retries(This,newVal)	\
    ( (This)->lpVtbl -> put_Retries(This,newVal) ) 

#define IFaxServer_get_RetryDelay(This,pVal)	\
    ( (This)->lpVtbl -> get_RetryDelay(This,pVal) ) 

#define IFaxServer_put_RetryDelay(This,newVal)	\
    ( (This)->lpVtbl -> put_RetryDelay(This,newVal) ) 

#define IFaxServer_get_DirtyDays(This,pVal)	\
    ( (This)->lpVtbl -> get_DirtyDays(This,pVal) ) 

#define IFaxServer_put_DirtyDays(This,newVal)	\
    ( (This)->lpVtbl -> put_DirtyDays(This,newVal) ) 

#define IFaxServer_get_Branding(This,pVal)	\
    ( (This)->lpVtbl -> get_Branding(This,pVal) ) 

#define IFaxServer_put_Branding(This,newVal)	\
    ( (This)->lpVtbl -> put_Branding(This,newVal) ) 

#define IFaxServer_get_UseDeviceTsid(This,pVal)	\
    ( (This)->lpVtbl -> get_UseDeviceTsid(This,pVal) ) 

#define IFaxServer_put_UseDeviceTsid(This,newVal)	\
    ( (This)->lpVtbl -> put_UseDeviceTsid(This,newVal) ) 

#define IFaxServer_get_ServerCoverpage(This,pVal)	\
    ( (This)->lpVtbl -> get_ServerCoverpage(This,pVal) ) 

#define IFaxServer_put_ServerCoverpage(This,newVal)	\
    ( (This)->lpVtbl -> put_ServerCoverpage(This,newVal) ) 

#define IFaxServer_get_PauseServerQueue(This,pVal)	\
    ( (This)->lpVtbl -> get_PauseServerQueue(This,pVal) ) 

#define IFaxServer_put_PauseServerQueue(This,newVal)	\
    ( (This)->lpVtbl -> put_PauseServerQueue(This,newVal) ) 

#define IFaxServer_get_ArchiveOutboundFaxes(This,pVal)	\
    ( (This)->lpVtbl -> get_ArchiveOutboundFaxes(This,pVal) ) 

#define IFaxServer_put_ArchiveOutboundFaxes(This,newVal)	\
    ( (This)->lpVtbl -> put_ArchiveOutboundFaxes(This,newVal) ) 

#define IFaxServer_get_ArchiveDirectory(This,pVal)	\
    ( (This)->lpVtbl -> get_ArchiveDirectory(This,pVal) ) 

#define IFaxServer_put_ArchiveDirectory(This,newVal)	\
    ( (This)->lpVtbl -> put_ArchiveDirectory(This,newVal) ) 

#define IFaxServer_get_ServerMapiProfile(This,pVal)	\
    ( (This)->lpVtbl -> get_ServerMapiProfile(This,pVal) ) 

#define IFaxServer_put_ServerMapiProfile(This,newVal)	\
    ( (This)->lpVtbl -> put_ServerMapiProfile(This,newVal) ) 

#define IFaxServer_get_DiscountRateStartHour(This,pVal)	\
    ( (This)->lpVtbl -> get_DiscountRateStartHour(This,pVal) ) 

#define IFaxServer_put_DiscountRateStartHour(This,newVal)	\
    ( (This)->lpVtbl -> put_DiscountRateStartHour(This,newVal) ) 

#define IFaxServer_get_DiscountRateStartMinute(This,pVal)	\
    ( (This)->lpVtbl -> get_DiscountRateStartMinute(This,pVal) ) 

#define IFaxServer_put_DiscountRateStartMinute(This,newVal)	\
    ( (This)->lpVtbl -> put_DiscountRateStartMinute(This,newVal) ) 

#define IFaxServer_get_DiscountRateEndHour(This,pVal)	\
    ( (This)->lpVtbl -> get_DiscountRateEndHour(This,pVal) ) 

#define IFaxServer_put_DiscountRateEndHour(This,newVal)	\
    ( (This)->lpVtbl -> put_DiscountRateEndHour(This,newVal) ) 

#define IFaxServer_get_DiscountRateEndMinute(This,pVal)	\
    ( (This)->lpVtbl -> get_DiscountRateEndMinute(This,pVal) ) 

#define IFaxServer_put_DiscountRateEndMinute(This,newVal)	\
    ( (This)->lpVtbl -> put_DiscountRateEndMinute(This,newVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFaxServer_INTERFACE_DEFINED__ */


#ifndef __IFaxPort_INTERFACE_DEFINED__
#define __IFaxPort_INTERFACE_DEFINED__

/* interface IFaxPort */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IFaxPort;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D73733CC-CC80-11D0-B225-00C04FB6C2F5")
    IFaxPort : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DeviceId( 
            /* [retval][out] */ __RPC__out long *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Rings( 
            /* [retval][out] */ __RPC__out long *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Rings( 
            /* [in] */ long newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Csid( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Csid( 
            /* [in] */ __RPC__in BSTR newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Tsid( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Tsid( 
            /* [in] */ __RPC__in BSTR newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Send( 
            /* [retval][out] */ __RPC__out BOOL *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Send( 
            /* [in] */ BOOL newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Receive( 
            /* [retval][out] */ __RPC__out BOOL *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Receive( 
            /* [in] */ BOOL newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Priority( 
            /* [retval][out] */ __RPC__out long *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Priority( 
            /* [in] */ long newVal) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetRoutingMethods( 
            /* [retval][out] */ __RPC__out VARIANT *retval) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CanModify( 
            /* [retval][out] */ __RPC__out BOOL *pVal) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetStatus( 
            /* [retval][out] */ __RPC__out VARIANT *retval) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFaxPortVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFaxPort * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFaxPort * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFaxPort * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFaxPort * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFaxPort * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFaxPort * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFaxPort * This,
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
        
        DECLSPEC_XFGVIRT(IFaxPort, get_Name)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IFaxPort * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFaxPort, get_DeviceId)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DeviceId )( 
            __RPC__in IFaxPort * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IFaxPort, get_Rings)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Rings )( 
            __RPC__in IFaxPort * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IFaxPort, put_Rings)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Rings )( 
            __RPC__in IFaxPort * This,
            /* [in] */ long newVal);
        
        DECLSPEC_XFGVIRT(IFaxPort, get_Csid)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Csid )( 
            __RPC__in IFaxPort * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFaxPort, put_Csid)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Csid )( 
            __RPC__in IFaxPort * This,
            /* [in] */ __RPC__in BSTR newVal);
        
        DECLSPEC_XFGVIRT(IFaxPort, get_Tsid)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Tsid )( 
            __RPC__in IFaxPort * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFaxPort, put_Tsid)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Tsid )( 
            __RPC__in IFaxPort * This,
            /* [in] */ __RPC__in BSTR newVal);
        
        DECLSPEC_XFGVIRT(IFaxPort, get_Send)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Send )( 
            __RPC__in IFaxPort * This,
            /* [retval][out] */ __RPC__out BOOL *pVal);
        
        DECLSPEC_XFGVIRT(IFaxPort, put_Send)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Send )( 
            __RPC__in IFaxPort * This,
            /* [in] */ BOOL newVal);
        
        DECLSPEC_XFGVIRT(IFaxPort, get_Receive)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Receive )( 
            __RPC__in IFaxPort * This,
            /* [retval][out] */ __RPC__out BOOL *pVal);
        
        DECLSPEC_XFGVIRT(IFaxPort, put_Receive)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Receive )( 
            __RPC__in IFaxPort * This,
            /* [in] */ BOOL newVal);
        
        DECLSPEC_XFGVIRT(IFaxPort, get_Priority)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Priority )( 
            __RPC__in IFaxPort * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IFaxPort, put_Priority)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Priority )( 
            __RPC__in IFaxPort * This,
            /* [in] */ long newVal);
        
        DECLSPEC_XFGVIRT(IFaxPort, GetRoutingMethods)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetRoutingMethods )( 
            __RPC__in IFaxPort * This,
            /* [retval][out] */ __RPC__out VARIANT *retval);
        
        DECLSPEC_XFGVIRT(IFaxPort, get_CanModify)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CanModify )( 
            __RPC__in IFaxPort * This,
            /* [retval][out] */ __RPC__out BOOL *pVal);
        
        DECLSPEC_XFGVIRT(IFaxPort, GetStatus)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetStatus )( 
            __RPC__in IFaxPort * This,
            /* [retval][out] */ __RPC__out VARIANT *retval);
        
        END_INTERFACE
    } IFaxPortVtbl;

    interface IFaxPort
    {
        CONST_VTBL struct IFaxPortVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFaxPort_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFaxPort_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFaxPort_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFaxPort_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFaxPort_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFaxPort_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFaxPort_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFaxPort_get_Name(This,pVal)	\
    ( (This)->lpVtbl -> get_Name(This,pVal) ) 

#define IFaxPort_get_DeviceId(This,pVal)	\
    ( (This)->lpVtbl -> get_DeviceId(This,pVal) ) 

#define IFaxPort_get_Rings(This,pVal)	\
    ( (This)->lpVtbl -> get_Rings(This,pVal) ) 

#define IFaxPort_put_Rings(This,newVal)	\
    ( (This)->lpVtbl -> put_Rings(This,newVal) ) 

#define IFaxPort_get_Csid(This,pVal)	\
    ( (This)->lpVtbl -> get_Csid(This,pVal) ) 

#define IFaxPort_put_Csid(This,newVal)	\
    ( (This)->lpVtbl -> put_Csid(This,newVal) ) 

#define IFaxPort_get_Tsid(This,pVal)	\
    ( (This)->lpVtbl -> get_Tsid(This,pVal) ) 

#define IFaxPort_put_Tsid(This,newVal)	\
    ( (This)->lpVtbl -> put_Tsid(This,newVal) ) 

#define IFaxPort_get_Send(This,pVal)	\
    ( (This)->lpVtbl -> get_Send(This,pVal) ) 

#define IFaxPort_put_Send(This,newVal)	\
    ( (This)->lpVtbl -> put_Send(This,newVal) ) 

#define IFaxPort_get_Receive(This,pVal)	\
    ( (This)->lpVtbl -> get_Receive(This,pVal) ) 

#define IFaxPort_put_Receive(This,newVal)	\
    ( (This)->lpVtbl -> put_Receive(This,newVal) ) 

#define IFaxPort_get_Priority(This,pVal)	\
    ( (This)->lpVtbl -> get_Priority(This,pVal) ) 

#define IFaxPort_put_Priority(This,newVal)	\
    ( (This)->lpVtbl -> put_Priority(This,newVal) ) 

#define IFaxPort_GetRoutingMethods(This,retval)	\
    ( (This)->lpVtbl -> GetRoutingMethods(This,retval) ) 

#define IFaxPort_get_CanModify(This,pVal)	\
    ( (This)->lpVtbl -> get_CanModify(This,pVal) ) 

#define IFaxPort_GetStatus(This,retval)	\
    ( (This)->lpVtbl -> GetStatus(This,retval) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFaxPort_INTERFACE_DEFINED__ */


#ifndef __IFaxPorts_INTERFACE_DEFINED__
#define __IFaxPorts_INTERFACE_DEFINED__

/* interface IFaxPorts */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IFaxPorts;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D73733D5-CC80-11D0-B225-00C04FB6C2F5")
    IFaxPorts : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            long Index,
            /* [retval][out] */ __RPC__out VARIANT *pVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFaxPortsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFaxPorts * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFaxPorts * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFaxPorts * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFaxPorts * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFaxPorts * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFaxPorts * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFaxPorts * This,
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
        
        DECLSPEC_XFGVIRT(IFaxPorts, get_Count)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IFaxPorts * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IFaxPorts, get_Item)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IFaxPorts * This,
            long Index,
            /* [retval][out] */ __RPC__out VARIANT *pVal);
        
        END_INTERFACE
    } IFaxPortsVtbl;

    interface IFaxPorts
    {
        CONST_VTBL struct IFaxPortsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFaxPorts_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFaxPorts_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFaxPorts_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFaxPorts_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFaxPorts_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFaxPorts_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFaxPorts_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFaxPorts_get_Count(This,pVal)	\
    ( (This)->lpVtbl -> get_Count(This,pVal) ) 

#define IFaxPorts_get_Item(This,Index,pVal)	\
    ( (This)->lpVtbl -> get_Item(This,Index,pVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFaxPorts_INTERFACE_DEFINED__ */


#ifndef __IFaxRoutingMethod_INTERFACE_DEFINED__
#define __IFaxRoutingMethod_INTERFACE_DEFINED__

/* interface IFaxRoutingMethod */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IFaxRoutingMethod;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2199F5F3-CEFC-11D0-A341-0000F800E68D")
    IFaxRoutingMethod : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DeviceId( 
            /* [retval][out] */ __RPC__out long *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Enable( 
            /* [retval][out] */ __RPC__out BOOL *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Enable( 
            /* [in] */ BOOL newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DeviceName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Guid( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_FunctionName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ImageName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_FriendlyName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ExtensionName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RoutingData( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFaxRoutingMethodVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFaxRoutingMethod * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFaxRoutingMethod * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFaxRoutingMethod * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFaxRoutingMethod * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFaxRoutingMethod * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFaxRoutingMethod * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFaxRoutingMethod * This,
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
        
        DECLSPEC_XFGVIRT(IFaxRoutingMethod, get_DeviceId)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DeviceId )( 
            __RPC__in IFaxRoutingMethod * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IFaxRoutingMethod, get_Enable)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Enable )( 
            __RPC__in IFaxRoutingMethod * This,
            /* [retval][out] */ __RPC__out BOOL *pVal);
        
        DECLSPEC_XFGVIRT(IFaxRoutingMethod, put_Enable)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Enable )( 
            __RPC__in IFaxRoutingMethod * This,
            /* [in] */ BOOL newVal);
        
        DECLSPEC_XFGVIRT(IFaxRoutingMethod, get_DeviceName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DeviceName )( 
            __RPC__in IFaxRoutingMethod * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFaxRoutingMethod, get_Guid)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Guid )( 
            __RPC__in IFaxRoutingMethod * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFaxRoutingMethod, get_FunctionName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FunctionName )( 
            __RPC__in IFaxRoutingMethod * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFaxRoutingMethod, get_ImageName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ImageName )( 
            __RPC__in IFaxRoutingMethod * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFaxRoutingMethod, get_FriendlyName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FriendlyName )( 
            __RPC__in IFaxRoutingMethod * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFaxRoutingMethod, get_ExtensionName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ExtensionName )( 
            __RPC__in IFaxRoutingMethod * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFaxRoutingMethod, get_RoutingData)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RoutingData )( 
            __RPC__in IFaxRoutingMethod * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        END_INTERFACE
    } IFaxRoutingMethodVtbl;

    interface IFaxRoutingMethod
    {
        CONST_VTBL struct IFaxRoutingMethodVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFaxRoutingMethod_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFaxRoutingMethod_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFaxRoutingMethod_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFaxRoutingMethod_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFaxRoutingMethod_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFaxRoutingMethod_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFaxRoutingMethod_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFaxRoutingMethod_get_DeviceId(This,pVal)	\
    ( (This)->lpVtbl -> get_DeviceId(This,pVal) ) 

#define IFaxRoutingMethod_get_Enable(This,pVal)	\
    ( (This)->lpVtbl -> get_Enable(This,pVal) ) 

#define IFaxRoutingMethod_put_Enable(This,newVal)	\
    ( (This)->lpVtbl -> put_Enable(This,newVal) ) 

#define IFaxRoutingMethod_get_DeviceName(This,pVal)	\
    ( (This)->lpVtbl -> get_DeviceName(This,pVal) ) 

#define IFaxRoutingMethod_get_Guid(This,pVal)	\
    ( (This)->lpVtbl -> get_Guid(This,pVal) ) 

#define IFaxRoutingMethod_get_FunctionName(This,pVal)	\
    ( (This)->lpVtbl -> get_FunctionName(This,pVal) ) 

#define IFaxRoutingMethod_get_ImageName(This,pVal)	\
    ( (This)->lpVtbl -> get_ImageName(This,pVal) ) 

#define IFaxRoutingMethod_get_FriendlyName(This,pVal)	\
    ( (This)->lpVtbl -> get_FriendlyName(This,pVal) ) 

#define IFaxRoutingMethod_get_ExtensionName(This,pVal)	\
    ( (This)->lpVtbl -> get_ExtensionName(This,pVal) ) 

#define IFaxRoutingMethod_get_RoutingData(This,pVal)	\
    ( (This)->lpVtbl -> get_RoutingData(This,pVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFaxRoutingMethod_INTERFACE_DEFINED__ */


#ifndef __IFaxRoutingMethods_INTERFACE_DEFINED__
#define __IFaxRoutingMethods_INTERFACE_DEFINED__

/* interface IFaxRoutingMethods */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IFaxRoutingMethods;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2199F5F5-CEFC-11D0-A341-0000F800E68D")
    IFaxRoutingMethods : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ long Index,
            /* [retval][out] */ __RPC__out VARIANT *pVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFaxRoutingMethodsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFaxRoutingMethods * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFaxRoutingMethods * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFaxRoutingMethods * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFaxRoutingMethods * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFaxRoutingMethods * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFaxRoutingMethods * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFaxRoutingMethods * This,
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
        
        DECLSPEC_XFGVIRT(IFaxRoutingMethods, get_Count)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IFaxRoutingMethods * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IFaxRoutingMethods, get_Item)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IFaxRoutingMethods * This,
            /* [in] */ long Index,
            /* [retval][out] */ __RPC__out VARIANT *pVal);
        
        END_INTERFACE
    } IFaxRoutingMethodsVtbl;

    interface IFaxRoutingMethods
    {
        CONST_VTBL struct IFaxRoutingMethodsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFaxRoutingMethods_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFaxRoutingMethods_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFaxRoutingMethods_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFaxRoutingMethods_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFaxRoutingMethods_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFaxRoutingMethods_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFaxRoutingMethods_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFaxRoutingMethods_get_Count(This,pVal)	\
    ( (This)->lpVtbl -> get_Count(This,pVal) ) 

#define IFaxRoutingMethods_get_Item(This,Index,pVal)	\
    ( (This)->lpVtbl -> get_Item(This,Index,pVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFaxRoutingMethods_INTERFACE_DEFINED__ */


#ifndef __IFaxStatus_INTERFACE_DEFINED__
#define __IFaxStatus_INTERFACE_DEFINED__

/* interface IFaxStatus */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IFaxStatus;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8B97E605-D054-11D0-B226-00C04FB6C2F5")
    IFaxStatus : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CallerId( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Csid( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CurrentPage( 
            /* [retval][out] */ __RPC__out long *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DeviceId( 
            /* [retval][out] */ __RPC__out long *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DeviceName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DocumentName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Send( 
            /* [retval][out] */ __RPC__out BOOL *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Receive( 
            /* [retval][out] */ __RPC__out BOOL *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Address( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RoutingString( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SenderName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RecipientName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DocumentSize( 
            /* [retval][out] */ __RPC__out long *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Description( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PageCount( 
            /* [retval][out] */ __RPC__out long *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Tsid( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_StartTime( 
            /* [retval][out] */ __RPC__out DATE *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SubmittedTime( 
            /* [retval][out] */ __RPC__out DATE *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ElapsedTime( 
            /* [retval][out] */ __RPC__out DATE *pVal) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Refresh( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFaxStatusVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFaxStatus * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFaxStatus * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFaxStatus * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFaxStatus * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFaxStatus * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFaxStatus * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFaxStatus * This,
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
        
        DECLSPEC_XFGVIRT(IFaxStatus, get_CallerId)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CallerId )( 
            __RPC__in IFaxStatus * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFaxStatus, get_Csid)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Csid )( 
            __RPC__in IFaxStatus * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFaxStatus, get_CurrentPage)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CurrentPage )( 
            __RPC__in IFaxStatus * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IFaxStatus, get_DeviceId)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DeviceId )( 
            __RPC__in IFaxStatus * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IFaxStatus, get_DeviceName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DeviceName )( 
            __RPC__in IFaxStatus * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFaxStatus, get_DocumentName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DocumentName )( 
            __RPC__in IFaxStatus * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFaxStatus, get_Send)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Send )( 
            __RPC__in IFaxStatus * This,
            /* [retval][out] */ __RPC__out BOOL *pVal);
        
        DECLSPEC_XFGVIRT(IFaxStatus, get_Receive)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Receive )( 
            __RPC__in IFaxStatus * This,
            /* [retval][out] */ __RPC__out BOOL *pVal);
        
        DECLSPEC_XFGVIRT(IFaxStatus, get_Address)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Address )( 
            __RPC__in IFaxStatus * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFaxStatus, get_RoutingString)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RoutingString )( 
            __RPC__in IFaxStatus * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFaxStatus, get_SenderName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SenderName )( 
            __RPC__in IFaxStatus * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFaxStatus, get_RecipientName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RecipientName )( 
            __RPC__in IFaxStatus * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFaxStatus, get_DocumentSize)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DocumentSize )( 
            __RPC__in IFaxStatus * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IFaxStatus, get_Description)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in IFaxStatus * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFaxStatus, get_PageCount)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PageCount )( 
            __RPC__in IFaxStatus * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IFaxStatus, get_Tsid)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Tsid )( 
            __RPC__in IFaxStatus * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFaxStatus, get_StartTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StartTime )( 
            __RPC__in IFaxStatus * This,
            /* [retval][out] */ __RPC__out DATE *pVal);
        
        DECLSPEC_XFGVIRT(IFaxStatus, get_SubmittedTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SubmittedTime )( 
            __RPC__in IFaxStatus * This,
            /* [retval][out] */ __RPC__out DATE *pVal);
        
        DECLSPEC_XFGVIRT(IFaxStatus, get_ElapsedTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ElapsedTime )( 
            __RPC__in IFaxStatus * This,
            /* [retval][out] */ __RPC__out DATE *pVal);
        
        DECLSPEC_XFGVIRT(IFaxStatus, Refresh)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in IFaxStatus * This);
        
        END_INTERFACE
    } IFaxStatusVtbl;

    interface IFaxStatus
    {
        CONST_VTBL struct IFaxStatusVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFaxStatus_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFaxStatus_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFaxStatus_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFaxStatus_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFaxStatus_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFaxStatus_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFaxStatus_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFaxStatus_get_CallerId(This,pVal)	\
    ( (This)->lpVtbl -> get_CallerId(This,pVal) ) 

#define IFaxStatus_get_Csid(This,pVal)	\
    ( (This)->lpVtbl -> get_Csid(This,pVal) ) 

#define IFaxStatus_get_CurrentPage(This,pVal)	\
    ( (This)->lpVtbl -> get_CurrentPage(This,pVal) ) 

#define IFaxStatus_get_DeviceId(This,pVal)	\
    ( (This)->lpVtbl -> get_DeviceId(This,pVal) ) 

#define IFaxStatus_get_DeviceName(This,pVal)	\
    ( (This)->lpVtbl -> get_DeviceName(This,pVal) ) 

#define IFaxStatus_get_DocumentName(This,pVal)	\
    ( (This)->lpVtbl -> get_DocumentName(This,pVal) ) 

#define IFaxStatus_get_Send(This,pVal)	\
    ( (This)->lpVtbl -> get_Send(This,pVal) ) 

#define IFaxStatus_get_Receive(This,pVal)	\
    ( (This)->lpVtbl -> get_Receive(This,pVal) ) 

#define IFaxStatus_get_Address(This,pVal)	\
    ( (This)->lpVtbl -> get_Address(This,pVal) ) 

#define IFaxStatus_get_RoutingString(This,pVal)	\
    ( (This)->lpVtbl -> get_RoutingString(This,pVal) ) 

#define IFaxStatus_get_SenderName(This,pVal)	\
    ( (This)->lpVtbl -> get_SenderName(This,pVal) ) 

#define IFaxStatus_get_RecipientName(This,pVal)	\
    ( (This)->lpVtbl -> get_RecipientName(This,pVal) ) 

#define IFaxStatus_get_DocumentSize(This,pVal)	\
    ( (This)->lpVtbl -> get_DocumentSize(This,pVal) ) 

#define IFaxStatus_get_Description(This,pVal)	\
    ( (This)->lpVtbl -> get_Description(This,pVal) ) 

#define IFaxStatus_get_PageCount(This,pVal)	\
    ( (This)->lpVtbl -> get_PageCount(This,pVal) ) 

#define IFaxStatus_get_Tsid(This,pVal)	\
    ( (This)->lpVtbl -> get_Tsid(This,pVal) ) 

#define IFaxStatus_get_StartTime(This,pVal)	\
    ( (This)->lpVtbl -> get_StartTime(This,pVal) ) 

#define IFaxStatus_get_SubmittedTime(This,pVal)	\
    ( (This)->lpVtbl -> get_SubmittedTime(This,pVal) ) 

#define IFaxStatus_get_ElapsedTime(This,pVal)	\
    ( (This)->lpVtbl -> get_ElapsedTime(This,pVal) ) 

#define IFaxStatus_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFaxStatus_INTERFACE_DEFINED__ */


#ifndef __IFaxDoc_INTERFACE_DEFINED__
#define __IFaxDoc_INTERFACE_DEFINED__

/* interface IFaxDoc */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IFaxDoc;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("FA21F4C5-5C4C-11D1-83CF-00C04FB6E984")
    IFaxDoc : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_FileName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_FileName( 
            /* [in] */ __RPC__in BSTR newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CoverpageName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_CoverpageName( 
            /* [in] */ __RPC__in BSTR newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SendCoverpage( 
            /* [retval][out] */ __RPC__out BOOL *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_SendCoverpage( 
            /* [in] */ BOOL newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ServerCoverpage( 
            /* [retval][out] */ __RPC__out BOOL *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_ServerCoverpage( 
            /* [in] */ BOOL newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DiscountSend( 
            /* [retval][out] */ __RPC__out BOOL *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_DiscountSend( 
            /* [in] */ BOOL newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RecipientName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_RecipientName( 
            /* [in] */ __RPC__in BSTR newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RecipientCompany( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_RecipientCompany( 
            /* [in] */ __RPC__in BSTR newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RecipientAddress( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_RecipientAddress( 
            /* [in] */ __RPC__in BSTR newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RecipientCity( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_RecipientCity( 
            /* [in] */ __RPC__in BSTR newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RecipientState( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_RecipientState( 
            /* [in] */ __RPC__in BSTR newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RecipientZip( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_RecipientZip( 
            /* [in] */ __RPC__in BSTR newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RecipientCountry( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_RecipientCountry( 
            /* [in] */ __RPC__in BSTR newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RecipientTitle( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_RecipientTitle( 
            /* [in] */ __RPC__in BSTR newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RecipientDepartment( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_RecipientDepartment( 
            /* [in] */ __RPC__in BSTR newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RecipientOffice( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_RecipientOffice( 
            /* [in] */ __RPC__in BSTR newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RecipientHomePhone( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_RecipientHomePhone( 
            /* [in] */ __RPC__in BSTR newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RecipientOfficePhone( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_RecipientOfficePhone( 
            /* [in] */ __RPC__in BSTR newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SenderName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_SenderName( 
            /* [in] */ __RPC__in BSTR newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SenderCompany( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_SenderCompany( 
            /* [in] */ __RPC__in BSTR newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SenderAddress( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_SenderAddress( 
            /* [in] */ __RPC__in BSTR newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SenderTitle( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_SenderTitle( 
            /* [in] */ __RPC__in BSTR newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SenderDepartment( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_SenderDepartment( 
            /* [in] */ __RPC__in BSTR newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SenderOffice( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_SenderOffice( 
            /* [in] */ __RPC__in BSTR newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SenderHomePhone( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_SenderHomePhone( 
            /* [in] */ __RPC__in BSTR newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SenderOfficePhone( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_SenderOfficePhone( 
            /* [in] */ __RPC__in BSTR newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CoverpageNote( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_CoverpageNote( 
            /* [in] */ __RPC__in BSTR newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CoverpageSubject( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_CoverpageSubject( 
            /* [in] */ __RPC__in BSTR newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Tsid( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Tsid( 
            /* [in] */ __RPC__in BSTR newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_BillingCode( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_BillingCode( 
            /* [in] */ __RPC__in BSTR newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_EmailAddress( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_EmailAddress( 
            /* [in] */ __RPC__in BSTR newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DisplayName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_DisplayName( 
            /* [in] */ __RPC__in BSTR newVal) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Send( 
            /* [retval][out] */ __RPC__out long *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_FaxNumber( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_FaxNumber( 
            /* [in] */ __RPC__in BSTR newVal) = 0;
        
        virtual /* [helpstring][id][propputref] */ HRESULT STDMETHODCALLTYPE putref_ConnectionObject( 
            /* [in] */ __RPC__in_opt IDispatch *newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CallHandle( 
            /* [retval][out] */ __RPC__out long *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_CallHandle( 
            /* [in] */ long newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SenderFax( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_SenderFax( 
            /* [in] */ __RPC__in BSTR newVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFaxDocVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFaxDoc * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFaxDoc * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFaxDoc * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFaxDoc * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFaxDoc * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFaxDoc * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFaxDoc * This,
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
        
        DECLSPEC_XFGVIRT(IFaxDoc, get_FileName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FileName )( 
            __RPC__in IFaxDoc * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFaxDoc, put_FileName)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_FileName )( 
            __RPC__in IFaxDoc * This,
            /* [in] */ __RPC__in BSTR newVal);
        
        DECLSPEC_XFGVIRT(IFaxDoc, get_CoverpageName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CoverpageName )( 
            __RPC__in IFaxDoc * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFaxDoc, put_CoverpageName)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_CoverpageName )( 
            __RPC__in IFaxDoc * This,
            /* [in] */ __RPC__in BSTR newVal);
        
        DECLSPEC_XFGVIRT(IFaxDoc, get_SendCoverpage)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SendCoverpage )( 
            __RPC__in IFaxDoc * This,
            /* [retval][out] */ __RPC__out BOOL *pVal);
        
        DECLSPEC_XFGVIRT(IFaxDoc, put_SendCoverpage)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SendCoverpage )( 
            __RPC__in IFaxDoc * This,
            /* [in] */ BOOL newVal);
        
        DECLSPEC_XFGVIRT(IFaxDoc, get_ServerCoverpage)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ServerCoverpage )( 
            __RPC__in IFaxDoc * This,
            /* [retval][out] */ __RPC__out BOOL *pVal);
        
        DECLSPEC_XFGVIRT(IFaxDoc, put_ServerCoverpage)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ServerCoverpage )( 
            __RPC__in IFaxDoc * This,
            /* [in] */ BOOL newVal);
        
        DECLSPEC_XFGVIRT(IFaxDoc, get_DiscountSend)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DiscountSend )( 
            __RPC__in IFaxDoc * This,
            /* [retval][out] */ __RPC__out BOOL *pVal);
        
        DECLSPEC_XFGVIRT(IFaxDoc, put_DiscountSend)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DiscountSend )( 
            __RPC__in IFaxDoc * This,
            /* [in] */ BOOL newVal);
        
        DECLSPEC_XFGVIRT(IFaxDoc, get_RecipientName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RecipientName )( 
            __RPC__in IFaxDoc * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFaxDoc, put_RecipientName)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_RecipientName )( 
            __RPC__in IFaxDoc * This,
            /* [in] */ __RPC__in BSTR newVal);
        
        DECLSPEC_XFGVIRT(IFaxDoc, get_RecipientCompany)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RecipientCompany )( 
            __RPC__in IFaxDoc * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFaxDoc, put_RecipientCompany)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_RecipientCompany )( 
            __RPC__in IFaxDoc * This,
            /* [in] */ __RPC__in BSTR newVal);
        
        DECLSPEC_XFGVIRT(IFaxDoc, get_RecipientAddress)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RecipientAddress )( 
            __RPC__in IFaxDoc * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFaxDoc, put_RecipientAddress)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_RecipientAddress )( 
            __RPC__in IFaxDoc * This,
            /* [in] */ __RPC__in BSTR newVal);
        
        DECLSPEC_XFGVIRT(IFaxDoc, get_RecipientCity)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RecipientCity )( 
            __RPC__in IFaxDoc * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFaxDoc, put_RecipientCity)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_RecipientCity )( 
            __RPC__in IFaxDoc * This,
            /* [in] */ __RPC__in BSTR newVal);
        
        DECLSPEC_XFGVIRT(IFaxDoc, get_RecipientState)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RecipientState )( 
            __RPC__in IFaxDoc * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFaxDoc, put_RecipientState)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_RecipientState )( 
            __RPC__in IFaxDoc * This,
            /* [in] */ __RPC__in BSTR newVal);
        
        DECLSPEC_XFGVIRT(IFaxDoc, get_RecipientZip)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RecipientZip )( 
            __RPC__in IFaxDoc * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFaxDoc, put_RecipientZip)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_RecipientZip )( 
            __RPC__in IFaxDoc * This,
            /* [in] */ __RPC__in BSTR newVal);
        
        DECLSPEC_XFGVIRT(IFaxDoc, get_RecipientCountry)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RecipientCountry )( 
            __RPC__in IFaxDoc * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFaxDoc, put_RecipientCountry)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_RecipientCountry )( 
            __RPC__in IFaxDoc * This,
            /* [in] */ __RPC__in BSTR newVal);
        
        DECLSPEC_XFGVIRT(IFaxDoc, get_RecipientTitle)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RecipientTitle )( 
            __RPC__in IFaxDoc * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFaxDoc, put_RecipientTitle)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_RecipientTitle )( 
            __RPC__in IFaxDoc * This,
            /* [in] */ __RPC__in BSTR newVal);
        
        DECLSPEC_XFGVIRT(IFaxDoc, get_RecipientDepartment)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RecipientDepartment )( 
            __RPC__in IFaxDoc * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFaxDoc, put_RecipientDepartment)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_RecipientDepartment )( 
            __RPC__in IFaxDoc * This,
            /* [in] */ __RPC__in BSTR newVal);
        
        DECLSPEC_XFGVIRT(IFaxDoc, get_RecipientOffice)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RecipientOffice )( 
            __RPC__in IFaxDoc * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFaxDoc, put_RecipientOffice)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_RecipientOffice )( 
            __RPC__in IFaxDoc * This,
            /* [in] */ __RPC__in BSTR newVal);
        
        DECLSPEC_XFGVIRT(IFaxDoc, get_RecipientHomePhone)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RecipientHomePhone )( 
            __RPC__in IFaxDoc * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFaxDoc, put_RecipientHomePhone)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_RecipientHomePhone )( 
            __RPC__in IFaxDoc * This,
            /* [in] */ __RPC__in BSTR newVal);
        
        DECLSPEC_XFGVIRT(IFaxDoc, get_RecipientOfficePhone)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RecipientOfficePhone )( 
            __RPC__in IFaxDoc * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFaxDoc, put_RecipientOfficePhone)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_RecipientOfficePhone )( 
            __RPC__in IFaxDoc * This,
            /* [in] */ __RPC__in BSTR newVal);
        
        DECLSPEC_XFGVIRT(IFaxDoc, get_SenderName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SenderName )( 
            __RPC__in IFaxDoc * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFaxDoc, put_SenderName)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SenderName )( 
            __RPC__in IFaxDoc * This,
            /* [in] */ __RPC__in BSTR newVal);
        
        DECLSPEC_XFGVIRT(IFaxDoc, get_SenderCompany)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SenderCompany )( 
            __RPC__in IFaxDoc * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFaxDoc, put_SenderCompany)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SenderCompany )( 
            __RPC__in IFaxDoc * This,
            /* [in] */ __RPC__in BSTR newVal);
        
        DECLSPEC_XFGVIRT(IFaxDoc, get_SenderAddress)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SenderAddress )( 
            __RPC__in IFaxDoc * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFaxDoc, put_SenderAddress)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SenderAddress )( 
            __RPC__in IFaxDoc * This,
            /* [in] */ __RPC__in BSTR newVal);
        
        DECLSPEC_XFGVIRT(IFaxDoc, get_SenderTitle)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SenderTitle )( 
            __RPC__in IFaxDoc * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFaxDoc, put_SenderTitle)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SenderTitle )( 
            __RPC__in IFaxDoc * This,
            /* [in] */ __RPC__in BSTR newVal);
        
        DECLSPEC_XFGVIRT(IFaxDoc, get_SenderDepartment)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SenderDepartment )( 
            __RPC__in IFaxDoc * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFaxDoc, put_SenderDepartment)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SenderDepartment )( 
            __RPC__in IFaxDoc * This,
            /* [in] */ __RPC__in BSTR newVal);
        
        DECLSPEC_XFGVIRT(IFaxDoc, get_SenderOffice)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SenderOffice )( 
            __RPC__in IFaxDoc * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFaxDoc, put_SenderOffice)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SenderOffice )( 
            __RPC__in IFaxDoc * This,
            /* [in] */ __RPC__in BSTR newVal);
        
        DECLSPEC_XFGVIRT(IFaxDoc, get_SenderHomePhone)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SenderHomePhone )( 
            __RPC__in IFaxDoc * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFaxDoc, put_SenderHomePhone)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SenderHomePhone )( 
            __RPC__in IFaxDoc * This,
            /* [in] */ __RPC__in BSTR newVal);
        
        DECLSPEC_XFGVIRT(IFaxDoc, get_SenderOfficePhone)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SenderOfficePhone )( 
            __RPC__in IFaxDoc * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFaxDoc, put_SenderOfficePhone)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SenderOfficePhone )( 
            __RPC__in IFaxDoc * This,
            /* [in] */ __RPC__in BSTR newVal);
        
        DECLSPEC_XFGVIRT(IFaxDoc, get_CoverpageNote)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CoverpageNote )( 
            __RPC__in IFaxDoc * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFaxDoc, put_CoverpageNote)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_CoverpageNote )( 
            __RPC__in IFaxDoc * This,
            /* [in] */ __RPC__in BSTR newVal);
        
        DECLSPEC_XFGVIRT(IFaxDoc, get_CoverpageSubject)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CoverpageSubject )( 
            __RPC__in IFaxDoc * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFaxDoc, put_CoverpageSubject)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_CoverpageSubject )( 
            __RPC__in IFaxDoc * This,
            /* [in] */ __RPC__in BSTR newVal);
        
        DECLSPEC_XFGVIRT(IFaxDoc, get_Tsid)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Tsid )( 
            __RPC__in IFaxDoc * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFaxDoc, put_Tsid)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Tsid )( 
            __RPC__in IFaxDoc * This,
            /* [in] */ __RPC__in BSTR newVal);
        
        DECLSPEC_XFGVIRT(IFaxDoc, get_BillingCode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_BillingCode )( 
            __RPC__in IFaxDoc * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFaxDoc, put_BillingCode)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_BillingCode )( 
            __RPC__in IFaxDoc * This,
            /* [in] */ __RPC__in BSTR newVal);
        
        DECLSPEC_XFGVIRT(IFaxDoc, get_EmailAddress)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EmailAddress )( 
            __RPC__in IFaxDoc * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFaxDoc, put_EmailAddress)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_EmailAddress )( 
            __RPC__in IFaxDoc * This,
            /* [in] */ __RPC__in BSTR newVal);
        
        DECLSPEC_XFGVIRT(IFaxDoc, get_DisplayName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DisplayName )( 
            __RPC__in IFaxDoc * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFaxDoc, put_DisplayName)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DisplayName )( 
            __RPC__in IFaxDoc * This,
            /* [in] */ __RPC__in BSTR newVal);
        
        DECLSPEC_XFGVIRT(IFaxDoc, Send)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Send )( 
            __RPC__in IFaxDoc * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IFaxDoc, get_FaxNumber)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FaxNumber )( 
            __RPC__in IFaxDoc * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFaxDoc, put_FaxNumber)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_FaxNumber )( 
            __RPC__in IFaxDoc * This,
            /* [in] */ __RPC__in BSTR newVal);
        
        DECLSPEC_XFGVIRT(IFaxDoc, putref_ConnectionObject)
        /* [helpstring][id][propputref] */ HRESULT ( STDMETHODCALLTYPE *putref_ConnectionObject )( 
            __RPC__in IFaxDoc * This,
            /* [in] */ __RPC__in_opt IDispatch *newVal);
        
        DECLSPEC_XFGVIRT(IFaxDoc, get_CallHandle)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CallHandle )( 
            __RPC__in IFaxDoc * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IFaxDoc, put_CallHandle)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_CallHandle )( 
            __RPC__in IFaxDoc * This,
            /* [in] */ long newVal);
        
        DECLSPEC_XFGVIRT(IFaxDoc, get_SenderFax)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SenderFax )( 
            __RPC__in IFaxDoc * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFaxDoc, put_SenderFax)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SenderFax )( 
            __RPC__in IFaxDoc * This,
            /* [in] */ __RPC__in BSTR newVal);
        
        END_INTERFACE
    } IFaxDocVtbl;

    interface IFaxDoc
    {
        CONST_VTBL struct IFaxDocVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFaxDoc_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFaxDoc_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFaxDoc_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFaxDoc_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFaxDoc_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFaxDoc_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFaxDoc_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFaxDoc_get_FileName(This,pVal)	\
    ( (This)->lpVtbl -> get_FileName(This,pVal) ) 

#define IFaxDoc_put_FileName(This,newVal)	\
    ( (This)->lpVtbl -> put_FileName(This,newVal) ) 

#define IFaxDoc_get_CoverpageName(This,pVal)	\
    ( (This)->lpVtbl -> get_CoverpageName(This,pVal) ) 

#define IFaxDoc_put_CoverpageName(This,newVal)	\
    ( (This)->lpVtbl -> put_CoverpageName(This,newVal) ) 

#define IFaxDoc_get_SendCoverpage(This,pVal)	\
    ( (This)->lpVtbl -> get_SendCoverpage(This,pVal) ) 

#define IFaxDoc_put_SendCoverpage(This,newVal)	\
    ( (This)->lpVtbl -> put_SendCoverpage(This,newVal) ) 

#define IFaxDoc_get_ServerCoverpage(This,pVal)	\
    ( (This)->lpVtbl -> get_ServerCoverpage(This,pVal) ) 

#define IFaxDoc_put_ServerCoverpage(This,newVal)	\
    ( (This)->lpVtbl -> put_ServerCoverpage(This,newVal) ) 

#define IFaxDoc_get_DiscountSend(This,pVal)	\
    ( (This)->lpVtbl -> get_DiscountSend(This,pVal) ) 

#define IFaxDoc_put_DiscountSend(This,newVal)	\
    ( (This)->lpVtbl -> put_DiscountSend(This,newVal) ) 

#define IFaxDoc_get_RecipientName(This,pVal)	\
    ( (This)->lpVtbl -> get_RecipientName(This,pVal) ) 

#define IFaxDoc_put_RecipientName(This,newVal)	\
    ( (This)->lpVtbl -> put_RecipientName(This,newVal) ) 

#define IFaxDoc_get_RecipientCompany(This,pVal)	\
    ( (This)->lpVtbl -> get_RecipientCompany(This,pVal) ) 

#define IFaxDoc_put_RecipientCompany(This,newVal)	\
    ( (This)->lpVtbl -> put_RecipientCompany(This,newVal) ) 

#define IFaxDoc_get_RecipientAddress(This,pVal)	\
    ( (This)->lpVtbl -> get_RecipientAddress(This,pVal) ) 

#define IFaxDoc_put_RecipientAddress(This,newVal)	\
    ( (This)->lpVtbl -> put_RecipientAddress(This,newVal) ) 

#define IFaxDoc_get_RecipientCity(This,pVal)	\
    ( (This)->lpVtbl -> get_RecipientCity(This,pVal) ) 

#define IFaxDoc_put_RecipientCity(This,newVal)	\
    ( (This)->lpVtbl -> put_RecipientCity(This,newVal) ) 

#define IFaxDoc_get_RecipientState(This,pVal)	\
    ( (This)->lpVtbl -> get_RecipientState(This,pVal) ) 

#define IFaxDoc_put_RecipientState(This,newVal)	\
    ( (This)->lpVtbl -> put_RecipientState(This,newVal) ) 

#define IFaxDoc_get_RecipientZip(This,pVal)	\
    ( (This)->lpVtbl -> get_RecipientZip(This,pVal) ) 

#define IFaxDoc_put_RecipientZip(This,newVal)	\
    ( (This)->lpVtbl -> put_RecipientZip(This,newVal) ) 

#define IFaxDoc_get_RecipientCountry(This,pVal)	\
    ( (This)->lpVtbl -> get_RecipientCountry(This,pVal) ) 

#define IFaxDoc_put_RecipientCountry(This,newVal)	\
    ( (This)->lpVtbl -> put_RecipientCountry(This,newVal) ) 

#define IFaxDoc_get_RecipientTitle(This,pVal)	\
    ( (This)->lpVtbl -> get_RecipientTitle(This,pVal) ) 

#define IFaxDoc_put_RecipientTitle(This,newVal)	\
    ( (This)->lpVtbl -> put_RecipientTitle(This,newVal) ) 

#define IFaxDoc_get_RecipientDepartment(This,pVal)	\
    ( (This)->lpVtbl -> get_RecipientDepartment(This,pVal) ) 

#define IFaxDoc_put_RecipientDepartment(This,newVal)	\
    ( (This)->lpVtbl -> put_RecipientDepartment(This,newVal) ) 

#define IFaxDoc_get_RecipientOffice(This,pVal)	\
    ( (This)->lpVtbl -> get_RecipientOffice(This,pVal) ) 

#define IFaxDoc_put_RecipientOffice(This,newVal)	\
    ( (This)->lpVtbl -> put_RecipientOffice(This,newVal) ) 

#define IFaxDoc_get_RecipientHomePhone(This,pVal)	\
    ( (This)->lpVtbl -> get_RecipientHomePhone(This,pVal) ) 

#define IFaxDoc_put_RecipientHomePhone(This,newVal)	\
    ( (This)->lpVtbl -> put_RecipientHomePhone(This,newVal) ) 

#define IFaxDoc_get_RecipientOfficePhone(This,pVal)	\
    ( (This)->lpVtbl -> get_RecipientOfficePhone(This,pVal) ) 

#define IFaxDoc_put_RecipientOfficePhone(This,newVal)	\
    ( (This)->lpVtbl -> put_RecipientOfficePhone(This,newVal) ) 

#define IFaxDoc_get_SenderName(This,pVal)	\
    ( (This)->lpVtbl -> get_SenderName(This,pVal) ) 

#define IFaxDoc_put_SenderName(This,newVal)	\
    ( (This)->lpVtbl -> put_SenderName(This,newVal) ) 

#define IFaxDoc_get_SenderCompany(This,pVal)	\
    ( (This)->lpVtbl -> get_SenderCompany(This,pVal) ) 

#define IFaxDoc_put_SenderCompany(This,newVal)	\
    ( (This)->lpVtbl -> put_SenderCompany(This,newVal) ) 

#define IFaxDoc_get_SenderAddress(This,pVal)	\
    ( (This)->lpVtbl -> get_SenderAddress(This,pVal) ) 

#define IFaxDoc_put_SenderAddress(This,newVal)	\
    ( (This)->lpVtbl -> put_SenderAddress(This,newVal) ) 

#define IFaxDoc_get_SenderTitle(This,pVal)	\
    ( (This)->lpVtbl -> get_SenderTitle(This,pVal) ) 

#define IFaxDoc_put_SenderTitle(This,newVal)	\
    ( (This)->lpVtbl -> put_SenderTitle(This,newVal) ) 

#define IFaxDoc_get_SenderDepartment(This,pVal)	\
    ( (This)->lpVtbl -> get_SenderDepartment(This,pVal) ) 

#define IFaxDoc_put_SenderDepartment(This,newVal)	\
    ( (This)->lpVtbl -> put_SenderDepartment(This,newVal) ) 

#define IFaxDoc_get_SenderOffice(This,pVal)	\
    ( (This)->lpVtbl -> get_SenderOffice(This,pVal) ) 

#define IFaxDoc_put_SenderOffice(This,newVal)	\
    ( (This)->lpVtbl -> put_SenderOffice(This,newVal) ) 

#define IFaxDoc_get_SenderHomePhone(This,pVal)	\
    ( (This)->lpVtbl -> get_SenderHomePhone(This,pVal) ) 

#define IFaxDoc_put_SenderHomePhone(This,newVal)	\
    ( (This)->lpVtbl -> put_SenderHomePhone(This,newVal) ) 

#define IFaxDoc_get_SenderOfficePhone(This,pVal)	\
    ( (This)->lpVtbl -> get_SenderOfficePhone(This,pVal) ) 

#define IFaxDoc_put_SenderOfficePhone(This,newVal)	\
    ( (This)->lpVtbl -> put_SenderOfficePhone(This,newVal) ) 

#define IFaxDoc_get_CoverpageNote(This,pVal)	\
    ( (This)->lpVtbl -> get_CoverpageNote(This,pVal) ) 

#define IFaxDoc_put_CoverpageNote(This,newVal)	\
    ( (This)->lpVtbl -> put_CoverpageNote(This,newVal) ) 

#define IFaxDoc_get_CoverpageSubject(This,pVal)	\
    ( (This)->lpVtbl -> get_CoverpageSubject(This,pVal) ) 

#define IFaxDoc_put_CoverpageSubject(This,newVal)	\
    ( (This)->lpVtbl -> put_CoverpageSubject(This,newVal) ) 

#define IFaxDoc_get_Tsid(This,pVal)	\
    ( (This)->lpVtbl -> get_Tsid(This,pVal) ) 

#define IFaxDoc_put_Tsid(This,newVal)	\
    ( (This)->lpVtbl -> put_Tsid(This,newVal) ) 

#define IFaxDoc_get_BillingCode(This,pVal)	\
    ( (This)->lpVtbl -> get_BillingCode(This,pVal) ) 

#define IFaxDoc_put_BillingCode(This,newVal)	\
    ( (This)->lpVtbl -> put_BillingCode(This,newVal) ) 

#define IFaxDoc_get_EmailAddress(This,pVal)	\
    ( (This)->lpVtbl -> get_EmailAddress(This,pVal) ) 

#define IFaxDoc_put_EmailAddress(This,newVal)	\
    ( (This)->lpVtbl -> put_EmailAddress(This,newVal) ) 

#define IFaxDoc_get_DisplayName(This,pVal)	\
    ( (This)->lpVtbl -> get_DisplayName(This,pVal) ) 

#define IFaxDoc_put_DisplayName(This,newVal)	\
    ( (This)->lpVtbl -> put_DisplayName(This,newVal) ) 

#define IFaxDoc_Send(This,pVal)	\
    ( (This)->lpVtbl -> Send(This,pVal) ) 

#define IFaxDoc_get_FaxNumber(This,pVal)	\
    ( (This)->lpVtbl -> get_FaxNumber(This,pVal) ) 

#define IFaxDoc_put_FaxNumber(This,newVal)	\
    ( (This)->lpVtbl -> put_FaxNumber(This,newVal) ) 

#define IFaxDoc_putref_ConnectionObject(This,newVal)	\
    ( (This)->lpVtbl -> putref_ConnectionObject(This,newVal) ) 

#define IFaxDoc_get_CallHandle(This,pVal)	\
    ( (This)->lpVtbl -> get_CallHandle(This,pVal) ) 

#define IFaxDoc_put_CallHandle(This,newVal)	\
    ( (This)->lpVtbl -> put_CallHandle(This,newVal) ) 

#define IFaxDoc_get_SenderFax(This,pVal)	\
    ( (This)->lpVtbl -> get_SenderFax(This,pVal) ) 

#define IFaxDoc_put_SenderFax(This,newVal)	\
    ( (This)->lpVtbl -> put_SenderFax(This,newVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFaxDoc_INTERFACE_DEFINED__ */


#ifndef __IFaxJobs_INTERFACE_DEFINED__
#define __IFaxJobs_INTERFACE_DEFINED__

/* interface IFaxJobs */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IFaxJobs;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("55DABFD3-5C9E-11D1-B791-000000000000")
    IFaxJobs : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            long Index,
            /* [retval][out] */ __RPC__out VARIANT *pVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFaxJobsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFaxJobs * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFaxJobs * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFaxJobs * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFaxJobs * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFaxJobs * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFaxJobs * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFaxJobs * This,
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
        
        DECLSPEC_XFGVIRT(IFaxJobs, get_Count)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IFaxJobs * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IFaxJobs, get_Item)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IFaxJobs * This,
            long Index,
            /* [retval][out] */ __RPC__out VARIANT *pVal);
        
        END_INTERFACE
    } IFaxJobsVtbl;

    interface IFaxJobs
    {
        CONST_VTBL struct IFaxJobsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFaxJobs_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFaxJobs_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFaxJobs_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFaxJobs_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFaxJobs_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFaxJobs_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFaxJobs_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFaxJobs_get_Count(This,pVal)	\
    ( (This)->lpVtbl -> get_Count(This,pVal) ) 

#define IFaxJobs_get_Item(This,Index,pVal)	\
    ( (This)->lpVtbl -> get_Item(This,Index,pVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFaxJobs_INTERFACE_DEFINED__ */


#ifndef __IFaxJob_INTERFACE_DEFINED__
#define __IFaxJob_INTERFACE_DEFINED__

/* interface IFaxJob */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IFaxJob;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("55DABFD5-5C9E-11D1-B791-000000000000")
    IFaxJob : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_JobId( 
            /* [retval][out] */ __RPC__out long *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Type( 
            /* [retval][out] */ __RPC__out long *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_UserName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_QueueStatus( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DeviceStatus( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PageCount( 
            /* [retval][out] */ __RPC__out long *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_FaxNumber( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RecipientName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Tsid( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SenderName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SenderCompany( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SenderDept( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_BillingCode( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DisplayName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DiscountSend( 
            /* [retval][out] */ __RPC__out BOOL *pVal) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetStatus( 
            long Command) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Refresh( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFaxJobVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFaxJob * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFaxJob * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFaxJob * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFaxJob * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFaxJob * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFaxJob * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFaxJob * This,
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
        
        DECLSPEC_XFGVIRT(IFaxJob, get_JobId)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_JobId )( 
            __RPC__in IFaxJob * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IFaxJob, get_Type)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Type )( 
            __RPC__in IFaxJob * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IFaxJob, get_UserName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UserName )( 
            __RPC__in IFaxJob * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFaxJob, get_QueueStatus)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_QueueStatus )( 
            __RPC__in IFaxJob * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFaxJob, get_DeviceStatus)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DeviceStatus )( 
            __RPC__in IFaxJob * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFaxJob, get_PageCount)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PageCount )( 
            __RPC__in IFaxJob * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IFaxJob, get_FaxNumber)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FaxNumber )( 
            __RPC__in IFaxJob * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFaxJob, get_RecipientName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RecipientName )( 
            __RPC__in IFaxJob * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFaxJob, get_Tsid)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Tsid )( 
            __RPC__in IFaxJob * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFaxJob, get_SenderName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SenderName )( 
            __RPC__in IFaxJob * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFaxJob, get_SenderCompany)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SenderCompany )( 
            __RPC__in IFaxJob * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFaxJob, get_SenderDept)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SenderDept )( 
            __RPC__in IFaxJob * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFaxJob, get_BillingCode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_BillingCode )( 
            __RPC__in IFaxJob * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFaxJob, get_DisplayName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DisplayName )( 
            __RPC__in IFaxJob * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFaxJob, get_DiscountSend)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DiscountSend )( 
            __RPC__in IFaxJob * This,
            /* [retval][out] */ __RPC__out BOOL *pVal);
        
        DECLSPEC_XFGVIRT(IFaxJob, SetStatus)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetStatus )( 
            __RPC__in IFaxJob * This,
            long Command);
        
        DECLSPEC_XFGVIRT(IFaxJob, Refresh)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in IFaxJob * This);
        
        END_INTERFACE
    } IFaxJobVtbl;

    interface IFaxJob
    {
        CONST_VTBL struct IFaxJobVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFaxJob_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFaxJob_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFaxJob_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFaxJob_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFaxJob_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFaxJob_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFaxJob_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFaxJob_get_JobId(This,pVal)	\
    ( (This)->lpVtbl -> get_JobId(This,pVal) ) 

#define IFaxJob_get_Type(This,pVal)	\
    ( (This)->lpVtbl -> get_Type(This,pVal) ) 

#define IFaxJob_get_UserName(This,pVal)	\
    ( (This)->lpVtbl -> get_UserName(This,pVal) ) 

#define IFaxJob_get_QueueStatus(This,pVal)	\
    ( (This)->lpVtbl -> get_QueueStatus(This,pVal) ) 

#define IFaxJob_get_DeviceStatus(This,pVal)	\
    ( (This)->lpVtbl -> get_DeviceStatus(This,pVal) ) 

#define IFaxJob_get_PageCount(This,pVal)	\
    ( (This)->lpVtbl -> get_PageCount(This,pVal) ) 

#define IFaxJob_get_FaxNumber(This,pVal)	\
    ( (This)->lpVtbl -> get_FaxNumber(This,pVal) ) 

#define IFaxJob_get_RecipientName(This,pVal)	\
    ( (This)->lpVtbl -> get_RecipientName(This,pVal) ) 

#define IFaxJob_get_Tsid(This,pVal)	\
    ( (This)->lpVtbl -> get_Tsid(This,pVal) ) 

#define IFaxJob_get_SenderName(This,pVal)	\
    ( (This)->lpVtbl -> get_SenderName(This,pVal) ) 

#define IFaxJob_get_SenderCompany(This,pVal)	\
    ( (This)->lpVtbl -> get_SenderCompany(This,pVal) ) 

#define IFaxJob_get_SenderDept(This,pVal)	\
    ( (This)->lpVtbl -> get_SenderDept(This,pVal) ) 

#define IFaxJob_get_BillingCode(This,pVal)	\
    ( (This)->lpVtbl -> get_BillingCode(This,pVal) ) 

#define IFaxJob_get_DisplayName(This,pVal)	\
    ( (This)->lpVtbl -> get_DisplayName(This,pVal) ) 

#define IFaxJob_get_DiscountSend(This,pVal)	\
    ( (This)->lpVtbl -> get_DiscountSend(This,pVal) ) 

#define IFaxJob_SetStatus(This,Command)	\
    ( (This)->lpVtbl -> SetStatus(This,Command) ) 

#define IFaxJob_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFaxJob_INTERFACE_DEFINED__ */



#ifndef __FAXCOMLib_LIBRARY_DEFINED__
#define __FAXCOMLib_LIBRARY_DEFINED__

/* library FAXCOMLib */
/* [helpstring][version][uuid] */ 


EXTERN_C const IID LIBID_FAXCOMLib;

EXTERN_C const CLSID CLSID_FaxTiff;

#ifdef __cplusplus

class DECLSPEC_UUID("87099231-C7AF-11D0-B225-00C04FB6C2F5")
FaxTiff;
#endif

EXTERN_C const CLSID CLSID_FaxServer;

#ifdef __cplusplus

class DECLSPEC_UUID("D73733C8-CC80-11D0-B225-00C04FB6C2F5")
FaxServer;
#endif

EXTERN_C const CLSID CLSID_FaxPort;

#ifdef __cplusplus

class DECLSPEC_UUID("D73733CD-CC80-11D0-B225-00C04FB6C2F5")
FaxPort;
#endif

EXTERN_C const CLSID CLSID_FaxPorts;

#ifdef __cplusplus

class DECLSPEC_UUID("D73733D6-CC80-11D0-B225-00C04FB6C2F5")
FaxPorts;
#endif

EXTERN_C const CLSID CLSID_FaxRoutingMethod;

#ifdef __cplusplus

class DECLSPEC_UUID("2199F5F4-CEFC-11D0-A341-0000F800E68D")
FaxRoutingMethod;
#endif

EXTERN_C const CLSID CLSID_FaxRoutingMethods;

#ifdef __cplusplus

class DECLSPEC_UUID("2199F5F6-CEFC-11D0-A341-0000F800E68D")
FaxRoutingMethods;
#endif

EXTERN_C const CLSID CLSID_FaxStatus;

#ifdef __cplusplus

class DECLSPEC_UUID("8B97E606-D054-11D0-B226-00C04FB6C2F5")
FaxStatus;
#endif

EXTERN_C const CLSID CLSID_FaxDoc;

#ifdef __cplusplus

class DECLSPEC_UUID("FA21F4C6-5C4C-11D1-83CF-00C04FB6E984")
FaxDoc;
#endif

EXTERN_C const CLSID CLSID_FaxJobs;

#ifdef __cplusplus

class DECLSPEC_UUID("55DABFD4-5C9E-11D1-B791-000000000000")
FaxJobs;
#endif

EXTERN_C const CLSID CLSID_FaxJob;

#ifdef __cplusplus

class DECLSPEC_UUID("55DABFD6-5C9E-11D1-B791-000000000000")
FaxJob;
#endif
#endif /* __FAXCOMLib_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_faxcom_0000_0011 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_faxcom_0000_0011_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_faxcom_0000_0011_v0_0_s_ifspec;

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


