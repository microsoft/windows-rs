

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

#ifndef __winsatcominterfacei_h__
#define __winsatcominterfacei_h__

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

#ifndef __IProvideWinSATAssessmentInfo_FWD_DEFINED__
#define __IProvideWinSATAssessmentInfo_FWD_DEFINED__
typedef interface IProvideWinSATAssessmentInfo IProvideWinSATAssessmentInfo;

#endif 	/* __IProvideWinSATAssessmentInfo_FWD_DEFINED__ */


#ifndef __IProvideWinSATResultsInfo_FWD_DEFINED__
#define __IProvideWinSATResultsInfo_FWD_DEFINED__
typedef interface IProvideWinSATResultsInfo IProvideWinSATResultsInfo;

#endif 	/* __IProvideWinSATResultsInfo_FWD_DEFINED__ */


#ifndef __IQueryRecentWinSATAssessment_FWD_DEFINED__
#define __IQueryRecentWinSATAssessment_FWD_DEFINED__
typedef interface IQueryRecentWinSATAssessment IQueryRecentWinSATAssessment;

#endif 	/* __IQueryRecentWinSATAssessment_FWD_DEFINED__ */


#ifndef __IProvideWinSATVisuals_FWD_DEFINED__
#define __IProvideWinSATVisuals_FWD_DEFINED__
typedef interface IProvideWinSATVisuals IProvideWinSATVisuals;

#endif 	/* __IProvideWinSATVisuals_FWD_DEFINED__ */


#ifndef __IQueryAllWinSATAssessments_FWD_DEFINED__
#define __IQueryAllWinSATAssessments_FWD_DEFINED__
typedef interface IQueryAllWinSATAssessments IQueryAllWinSATAssessments;

#endif 	/* __IQueryAllWinSATAssessments_FWD_DEFINED__ */


#ifndef __IWinSATInitiateEvents_FWD_DEFINED__
#define __IWinSATInitiateEvents_FWD_DEFINED__
typedef interface IWinSATInitiateEvents IWinSATInitiateEvents;

#endif 	/* __IWinSATInitiateEvents_FWD_DEFINED__ */


#ifndef __IInitiateWinSATAssessment_FWD_DEFINED__
#define __IInitiateWinSATAssessment_FWD_DEFINED__
typedef interface IInitiateWinSATAssessment IInitiateWinSATAssessment;

#endif 	/* __IInitiateWinSATAssessment_FWD_DEFINED__ */


#ifndef __IAccessibleWinSAT_FWD_DEFINED__
#define __IAccessibleWinSAT_FWD_DEFINED__
typedef interface IAccessibleWinSAT IAccessibleWinSAT;

#endif 	/* __IAccessibleWinSAT_FWD_DEFINED__ */


#ifndef __IQueryOEMWinSATCustomization_FWD_DEFINED__
#define __IQueryOEMWinSATCustomization_FWD_DEFINED__
typedef interface IQueryOEMWinSATCustomization IQueryOEMWinSATCustomization;

#endif 	/* __IQueryOEMWinSATCustomization_FWD_DEFINED__ */


#ifndef __CInitiateWinSAT_FWD_DEFINED__
#define __CInitiateWinSAT_FWD_DEFINED__

#ifdef __cplusplus
typedef class CInitiateWinSAT CInitiateWinSAT;
#else
typedef struct CInitiateWinSAT CInitiateWinSAT;
#endif /* __cplusplus */

#endif 	/* __CInitiateWinSAT_FWD_DEFINED__ */


#ifndef __CQueryWinSAT_FWD_DEFINED__
#define __CQueryWinSAT_FWD_DEFINED__

#ifdef __cplusplus
typedef class CQueryWinSAT CQueryWinSAT;
#else
typedef struct CQueryWinSAT CQueryWinSAT;
#endif /* __cplusplus */

#endif 	/* __CQueryWinSAT_FWD_DEFINED__ */


#ifndef __CQueryAllWinSAT_FWD_DEFINED__
#define __CQueryAllWinSAT_FWD_DEFINED__

#ifdef __cplusplus
typedef class CQueryAllWinSAT CQueryAllWinSAT;
#else
typedef struct CQueryAllWinSAT CQueryAllWinSAT;
#endif /* __cplusplus */

#endif 	/* __CQueryAllWinSAT_FWD_DEFINED__ */


#ifndef __CProvideWinSATVisuals_FWD_DEFINED__
#define __CProvideWinSATVisuals_FWD_DEFINED__

#ifdef __cplusplus
typedef class CProvideWinSATVisuals CProvideWinSATVisuals;
#else
typedef struct CProvideWinSATVisuals CProvideWinSATVisuals;
#endif /* __cplusplus */

#endif 	/* __CProvideWinSATVisuals_FWD_DEFINED__ */


#ifndef __CAccessiblityWinSAT_FWD_DEFINED__
#define __CAccessiblityWinSAT_FWD_DEFINED__

#ifdef __cplusplus
typedef class CAccessiblityWinSAT CAccessiblityWinSAT;
#else
typedef struct CAccessiblityWinSAT CAccessiblityWinSAT;
#endif /* __cplusplus */

#endif 	/* __CAccessiblityWinSAT_FWD_DEFINED__ */


#ifndef __CQueryOEMWinSATCustomization_FWD_DEFINED__
#define __CQueryOEMWinSATCustomization_FWD_DEFINED__

#ifdef __cplusplus
typedef class CQueryOEMWinSATCustomization CQueryOEMWinSATCustomization;
#else
typedef struct CQueryOEMWinSATCustomization CQueryOEMWinSATCustomization;
#endif /* __cplusplus */

#endif 	/* __CQueryOEMWinSATCustomization_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "msxml.h"
#include "oleacc.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_winsatcominterfacei_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
typedef /* [public][public] */ 
enum __MIDL___MIDL_itf_winsatcominterfacei_0000_0000_0001
    {
        WINSAT_OEM_DATA_VALID	= 0,
        WINSAT_OEM_DATA_NON_SYS_CONFIG_MATCH	= 1,
        WINSAT_OEM_DATA_INVALID	= 2,
        WINSAT_OEM_NO_DATA_SUPPLIED	= 3
    } 	WINSAT_OEM_CUSTOMIZATION_STATE;

typedef /* [public][public][public] */ 
enum __MIDL___MIDL_itf_winsatcominterfacei_0000_0000_0002
    {
        WINSAT_ASSESSMENT_STATE_MIN	= 0,
        WINSAT_ASSESSMENT_STATE_UNKNOWN	= 0,
        WINSAT_ASSESSMENT_STATE_VALID	= 1,
        WINSAT_ASSESSMENT_STATE_INCOHERENT_WITH_HARDWARE	= 2,
        WINSAT_ASSESSMENT_STATE_NOT_AVAILABLE	= 3,
        WINSAT_ASSESSMENT_STATE_INVALID	= 4,
        WINSAT_ASSESSMENT_STATE_MAX	= 4
    } 	WINSAT_ASSESSMENT_STATE;

typedef /* [public][public] */ 
enum __MIDL___MIDL_itf_winsatcominterfacei_0000_0000_0003
    {
        WINSAT_ASSESSMENT_MEMORY	= 0,
        WINSAT_ASSESSMENT_CPU	= 1,
        WINSAT_ASSESSMENT_DISK	= 2,
        WINSAT_ASSESSMENT_D3D	= 3,
        WINSAT_ASSESSMENT_GRAPHICS	= 4
    } 	WINSAT_ASSESSMENT_TYPE;

typedef /* [public][public] */ 
enum __MIDL___MIDL_itf_winsatcominterfacei_0000_0000_0004
    {
        WINSAT_BITMAP_SIZE_SMALL	= 0,
        WINSAT_BITMAP_SIZE_NORMAL	= 1
    } 	WINSAT_BITMAP_SIZE;



extern RPC_IF_HANDLE __MIDL_itf_winsatcominterfacei_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_winsatcominterfacei_0000_0000_v0_0_s_ifspec;

#ifndef __IProvideWinSATAssessmentInfo_INTERFACE_DEFINED__
#define __IProvideWinSATAssessmentInfo_INTERFACE_DEFINED__

/* interface IProvideWinSATAssessmentInfo */
/* [unique][oleautomation][dual][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IProvideWinSATAssessmentInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0CD1C380-52D3-4678-AC6F-E929E480BE9E")
    IProvideWinSATAssessmentInfo : public IDispatch
    {
    public:
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_Score( 
            /* [retval][out] */ __RPC__out float *score) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_Title( 
            /* [string][retval][out] */ __RPC__deref_out_opt_string BSTR *title) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_Description( 
            /* [string][retval][out] */ __RPC__deref_out_opt_string BSTR *description) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IProvideWinSATAssessmentInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IProvideWinSATAssessmentInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IProvideWinSATAssessmentInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IProvideWinSATAssessmentInfo * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IProvideWinSATAssessmentInfo * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IProvideWinSATAssessmentInfo * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IProvideWinSATAssessmentInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IProvideWinSATAssessmentInfo * This,
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
        
        DECLSPEC_XFGVIRT(IProvideWinSATAssessmentInfo, get_Score)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Score )( 
            __RPC__in IProvideWinSATAssessmentInfo * This,
            /* [retval][out] */ __RPC__out float *score);
        
        DECLSPEC_XFGVIRT(IProvideWinSATAssessmentInfo, get_Title)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Title )( 
            __RPC__in IProvideWinSATAssessmentInfo * This,
            /* [string][retval][out] */ __RPC__deref_out_opt_string BSTR *title);
        
        DECLSPEC_XFGVIRT(IProvideWinSATAssessmentInfo, get_Description)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in IProvideWinSATAssessmentInfo * This,
            /* [string][retval][out] */ __RPC__deref_out_opt_string BSTR *description);
        
        END_INTERFACE
    } IProvideWinSATAssessmentInfoVtbl;

    interface IProvideWinSATAssessmentInfo
    {
        CONST_VTBL struct IProvideWinSATAssessmentInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IProvideWinSATAssessmentInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IProvideWinSATAssessmentInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IProvideWinSATAssessmentInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IProvideWinSATAssessmentInfo_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IProvideWinSATAssessmentInfo_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IProvideWinSATAssessmentInfo_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IProvideWinSATAssessmentInfo_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IProvideWinSATAssessmentInfo_get_Score(This,score)	\
    ( (This)->lpVtbl -> get_Score(This,score) ) 

#define IProvideWinSATAssessmentInfo_get_Title(This,title)	\
    ( (This)->lpVtbl -> get_Title(This,title) ) 

#define IProvideWinSATAssessmentInfo_get_Description(This,description)	\
    ( (This)->lpVtbl -> get_Description(This,description) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IProvideWinSATAssessmentInfo_INTERFACE_DEFINED__ */


#ifndef __IProvideWinSATResultsInfo_INTERFACE_DEFINED__
#define __IProvideWinSATResultsInfo_INTERFACE_DEFINED__

/* interface IProvideWinSATResultsInfo */
/* [unique][oleautomation][dual][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IProvideWinSATResultsInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F8334D5D-568E-4075-875F-9DF341506640")
    IProvideWinSATResultsInfo : public IDispatch
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetAssessmentInfo( 
            /* [in] */ WINSAT_ASSESSMENT_TYPE assessment,
            /* [retval][out] */ __RPC__deref_out_opt IProvideWinSATAssessmentInfo **ppinfo) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_AssessmentState( 
            /* [retval][out] */ __RPC__out WINSAT_ASSESSMENT_STATE *state) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_AssessmentDateTime( 
            /* [retval][out] */ __RPC__out VARIANT *fileTime) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_SystemRating( 
            /* [retval][out] */ __RPC__out float *level) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_RatingStateDesc( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *description) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IProvideWinSATResultsInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IProvideWinSATResultsInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IProvideWinSATResultsInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IProvideWinSATResultsInfo * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IProvideWinSATResultsInfo * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IProvideWinSATResultsInfo * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IProvideWinSATResultsInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IProvideWinSATResultsInfo * This,
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
        
        DECLSPEC_XFGVIRT(IProvideWinSATResultsInfo, GetAssessmentInfo)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetAssessmentInfo )( 
            __RPC__in IProvideWinSATResultsInfo * This,
            /* [in] */ WINSAT_ASSESSMENT_TYPE assessment,
            /* [retval][out] */ __RPC__deref_out_opt IProvideWinSATAssessmentInfo **ppinfo);
        
        DECLSPEC_XFGVIRT(IProvideWinSATResultsInfo, get_AssessmentState)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_AssessmentState )( 
            __RPC__in IProvideWinSATResultsInfo * This,
            /* [retval][out] */ __RPC__out WINSAT_ASSESSMENT_STATE *state);
        
        DECLSPEC_XFGVIRT(IProvideWinSATResultsInfo, get_AssessmentDateTime)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_AssessmentDateTime )( 
            __RPC__in IProvideWinSATResultsInfo * This,
            /* [retval][out] */ __RPC__out VARIANT *fileTime);
        
        DECLSPEC_XFGVIRT(IProvideWinSATResultsInfo, get_SystemRating)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_SystemRating )( 
            __RPC__in IProvideWinSATResultsInfo * This,
            /* [retval][out] */ __RPC__out float *level);
        
        DECLSPEC_XFGVIRT(IProvideWinSATResultsInfo, get_RatingStateDesc)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_RatingStateDesc )( 
            __RPC__in IProvideWinSATResultsInfo * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *description);
        
        END_INTERFACE
    } IProvideWinSATResultsInfoVtbl;

    interface IProvideWinSATResultsInfo
    {
        CONST_VTBL struct IProvideWinSATResultsInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IProvideWinSATResultsInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IProvideWinSATResultsInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IProvideWinSATResultsInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IProvideWinSATResultsInfo_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IProvideWinSATResultsInfo_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IProvideWinSATResultsInfo_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IProvideWinSATResultsInfo_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IProvideWinSATResultsInfo_GetAssessmentInfo(This,assessment,ppinfo)	\
    ( (This)->lpVtbl -> GetAssessmentInfo(This,assessment,ppinfo) ) 

#define IProvideWinSATResultsInfo_get_AssessmentState(This,state)	\
    ( (This)->lpVtbl -> get_AssessmentState(This,state) ) 

#define IProvideWinSATResultsInfo_get_AssessmentDateTime(This,fileTime)	\
    ( (This)->lpVtbl -> get_AssessmentDateTime(This,fileTime) ) 

#define IProvideWinSATResultsInfo_get_SystemRating(This,level)	\
    ( (This)->lpVtbl -> get_SystemRating(This,level) ) 

#define IProvideWinSATResultsInfo_get_RatingStateDesc(This,description)	\
    ( (This)->lpVtbl -> get_RatingStateDesc(This,description) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IProvideWinSATResultsInfo_INTERFACE_DEFINED__ */


#ifndef __IQueryRecentWinSATAssessment_INTERFACE_DEFINED__
#define __IQueryRecentWinSATAssessment_INTERFACE_DEFINED__

/* interface IQueryRecentWinSATAssessment */
/* [unique][oleautomation][dual][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IQueryRecentWinSATAssessment;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F8AD5D1F-3B47-4bdc-9375-7C6B1DA4ECA7")
    IQueryRecentWinSATAssessment : public IDispatch
    {
    public:
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_XML( 
            /* [string][in] */ __RPC__in_string BSTR xPath,
            /* [defaultvalue][string][in] */ __RPC__in_string BSTR namespaces,
            /* [retval][out] */ __RPC__deref_out_opt IXMLDOMNodeList **ppDomNodeList) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_Info( 
            /* [retval][out] */ __RPC__deref_out_opt IProvideWinSATResultsInfo **ppWinSATAssessmentInfo) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IQueryRecentWinSATAssessmentVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IQueryRecentWinSATAssessment * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IQueryRecentWinSATAssessment * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IQueryRecentWinSATAssessment * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IQueryRecentWinSATAssessment * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IQueryRecentWinSATAssessment * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IQueryRecentWinSATAssessment * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IQueryRecentWinSATAssessment * This,
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
        
        DECLSPEC_XFGVIRT(IQueryRecentWinSATAssessment, get_XML)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_XML )( 
            __RPC__in IQueryRecentWinSATAssessment * This,
            /* [string][in] */ __RPC__in_string BSTR xPath,
            /* [defaultvalue][string][in] */ __RPC__in_string BSTR namespaces,
            /* [retval][out] */ __RPC__deref_out_opt IXMLDOMNodeList **ppDomNodeList);
        
        DECLSPEC_XFGVIRT(IQueryRecentWinSATAssessment, get_Info)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Info )( 
            __RPC__in IQueryRecentWinSATAssessment * This,
            /* [retval][out] */ __RPC__deref_out_opt IProvideWinSATResultsInfo **ppWinSATAssessmentInfo);
        
        END_INTERFACE
    } IQueryRecentWinSATAssessmentVtbl;

    interface IQueryRecentWinSATAssessment
    {
        CONST_VTBL struct IQueryRecentWinSATAssessmentVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IQueryRecentWinSATAssessment_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IQueryRecentWinSATAssessment_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IQueryRecentWinSATAssessment_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IQueryRecentWinSATAssessment_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IQueryRecentWinSATAssessment_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IQueryRecentWinSATAssessment_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IQueryRecentWinSATAssessment_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IQueryRecentWinSATAssessment_get_XML(This,xPath,namespaces,ppDomNodeList)	\
    ( (This)->lpVtbl -> get_XML(This,xPath,namespaces,ppDomNodeList) ) 

#define IQueryRecentWinSATAssessment_get_Info(This,ppWinSATAssessmentInfo)	\
    ( (This)->lpVtbl -> get_Info(This,ppWinSATAssessmentInfo) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IQueryRecentWinSATAssessment_INTERFACE_DEFINED__ */


#ifndef __IProvideWinSATVisuals_INTERFACE_DEFINED__
#define __IProvideWinSATVisuals_INTERFACE_DEFINED__

/* interface IProvideWinSATVisuals */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IProvideWinSATVisuals;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A9F4ADE0-871A-42a3-B813-3078D25162C9")
    IProvideWinSATVisuals : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE get_Bitmap( 
            /* [in] */ WINSAT_BITMAP_SIZE bitmapSize,
            WINSAT_ASSESSMENT_STATE state,
            float rating,
            /* [out] */ __RPC__deref_out_opt HBITMAP *pBitmap) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IProvideWinSATVisualsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IProvideWinSATVisuals * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IProvideWinSATVisuals * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IProvideWinSATVisuals * This);
        
        DECLSPEC_XFGVIRT(IProvideWinSATVisuals, get_Bitmap)
        HRESULT ( STDMETHODCALLTYPE *get_Bitmap )( 
            __RPC__in IProvideWinSATVisuals * This,
            /* [in] */ WINSAT_BITMAP_SIZE bitmapSize,
            WINSAT_ASSESSMENT_STATE state,
            float rating,
            /* [out] */ __RPC__deref_out_opt HBITMAP *pBitmap);
        
        END_INTERFACE
    } IProvideWinSATVisualsVtbl;

    interface IProvideWinSATVisuals
    {
        CONST_VTBL struct IProvideWinSATVisualsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IProvideWinSATVisuals_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IProvideWinSATVisuals_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IProvideWinSATVisuals_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IProvideWinSATVisuals_get_Bitmap(This,bitmapSize,state,rating,pBitmap)	\
    ( (This)->lpVtbl -> get_Bitmap(This,bitmapSize,state,rating,pBitmap) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IProvideWinSATVisuals_INTERFACE_DEFINED__ */


#ifndef __IQueryAllWinSATAssessments_INTERFACE_DEFINED__
#define __IQueryAllWinSATAssessments_INTERFACE_DEFINED__

/* interface IQueryAllWinSATAssessments */
/* [unique][oleautomation][dual][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IQueryAllWinSATAssessments;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0B89ED1D-6398-4fea-87FC-567D8D19176F")
    IQueryAllWinSATAssessments : public IDispatch
    {
    public:
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_AllXML( 
            /* [string][in] */ __RPC__in_string BSTR xPath,
            /* [defaultvalue][string][in] */ __RPC__in_string BSTR namespaces,
            /* [retval][out] */ __RPC__deref_out_opt IXMLDOMNodeList **ppDomNodeList) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IQueryAllWinSATAssessmentsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IQueryAllWinSATAssessments * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IQueryAllWinSATAssessments * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IQueryAllWinSATAssessments * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IQueryAllWinSATAssessments * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IQueryAllWinSATAssessments * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IQueryAllWinSATAssessments * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IQueryAllWinSATAssessments * This,
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
        
        DECLSPEC_XFGVIRT(IQueryAllWinSATAssessments, get_AllXML)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_AllXML )( 
            __RPC__in IQueryAllWinSATAssessments * This,
            /* [string][in] */ __RPC__in_string BSTR xPath,
            /* [defaultvalue][string][in] */ __RPC__in_string BSTR namespaces,
            /* [retval][out] */ __RPC__deref_out_opt IXMLDOMNodeList **ppDomNodeList);
        
        END_INTERFACE
    } IQueryAllWinSATAssessmentsVtbl;

    interface IQueryAllWinSATAssessments
    {
        CONST_VTBL struct IQueryAllWinSATAssessmentsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IQueryAllWinSATAssessments_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IQueryAllWinSATAssessments_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IQueryAllWinSATAssessments_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IQueryAllWinSATAssessments_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IQueryAllWinSATAssessments_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IQueryAllWinSATAssessments_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IQueryAllWinSATAssessments_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IQueryAllWinSATAssessments_get_AllXML(This,xPath,namespaces,ppDomNodeList)	\
    ( (This)->lpVtbl -> get_AllXML(This,xPath,namespaces,ppDomNodeList) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IQueryAllWinSATAssessments_INTERFACE_DEFINED__ */


#ifndef __IWinSATInitiateEvents_INTERFACE_DEFINED__
#define __IWinSATInitiateEvents_INTERFACE_DEFINED__

/* interface IWinSATInitiateEvents */
/* [uuid][object] */ 


EXTERN_C const IID IID_IWinSATInitiateEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("262A1918-BA0D-41d5-92C2-FAB4633EE74F")
    IWinSATInitiateEvents : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE WinSATComplete( 
            /* [in] */ HRESULT hresult,
            /* [string][in] */ __RPC__in_string LPCWSTR strDescription) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WinSATUpdate( 
            /* [in] */ UINT uCurrentTick,
            /* [in] */ UINT uTickTotal,
            /* [string][in] */ __RPC__in_string LPCWSTR strCurrentState) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWinSATInitiateEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWinSATInitiateEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWinSATInitiateEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWinSATInitiateEvents * This);
        
        DECLSPEC_XFGVIRT(IWinSATInitiateEvents, WinSATComplete)
        HRESULT ( STDMETHODCALLTYPE *WinSATComplete )( 
            __RPC__in IWinSATInitiateEvents * This,
            /* [in] */ HRESULT hresult,
            /* [string][in] */ __RPC__in_string LPCWSTR strDescription);
        
        DECLSPEC_XFGVIRT(IWinSATInitiateEvents, WinSATUpdate)
        HRESULT ( STDMETHODCALLTYPE *WinSATUpdate )( 
            __RPC__in IWinSATInitiateEvents * This,
            /* [in] */ UINT uCurrentTick,
            /* [in] */ UINT uTickTotal,
            /* [string][in] */ __RPC__in_string LPCWSTR strCurrentState);
        
        END_INTERFACE
    } IWinSATInitiateEventsVtbl;

    interface IWinSATInitiateEvents
    {
        CONST_VTBL struct IWinSATInitiateEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWinSATInitiateEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWinSATInitiateEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWinSATInitiateEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWinSATInitiateEvents_WinSATComplete(This,hresult,strDescription)	\
    ( (This)->lpVtbl -> WinSATComplete(This,hresult,strDescription) ) 

#define IWinSATInitiateEvents_WinSATUpdate(This,uCurrentTick,uTickTotal,strCurrentState)	\
    ( (This)->lpVtbl -> WinSATUpdate(This,uCurrentTick,uTickTotal,strCurrentState) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWinSATInitiateEvents_INTERFACE_DEFINED__ */


#ifndef __IInitiateWinSATAssessment_INTERFACE_DEFINED__
#define __IInitiateWinSATAssessment_INTERFACE_DEFINED__

/* interface IInitiateWinSATAssessment */
/* [uuid][object] */ 


EXTERN_C const IID IID_IInitiateWinSATAssessment;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D983FC50-F5BF-49d5-B5ED-CCCB18AA7FC1")
    IInitiateWinSATAssessment : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE InitiateAssessment( 
            /* [string][in] */ __RPC__in_string LPCWSTR cmdLine,
            /* [unique][optional][in] */ __RPC__in_opt IWinSATInitiateEvents *pCallbacks,
            /* [unique][optional][in] */ __RPC__in_opt HWND callerHwnd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InitiateFormalAssessment( 
            /* [unique][optional][in] */ __RPC__in_opt IWinSATInitiateEvents *pCallbacks,
            /* [unique][optional][in] */ __RPC__in_opt HWND callerHwnd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CancelAssessment( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInitiateWinSATAssessmentVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IInitiateWinSATAssessment * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IInitiateWinSATAssessment * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IInitiateWinSATAssessment * This);
        
        DECLSPEC_XFGVIRT(IInitiateWinSATAssessment, InitiateAssessment)
        HRESULT ( STDMETHODCALLTYPE *InitiateAssessment )( 
            __RPC__in IInitiateWinSATAssessment * This,
            /* [string][in] */ __RPC__in_string LPCWSTR cmdLine,
            /* [unique][optional][in] */ __RPC__in_opt IWinSATInitiateEvents *pCallbacks,
            /* [unique][optional][in] */ __RPC__in_opt HWND callerHwnd);
        
        DECLSPEC_XFGVIRT(IInitiateWinSATAssessment, InitiateFormalAssessment)
        HRESULT ( STDMETHODCALLTYPE *InitiateFormalAssessment )( 
            __RPC__in IInitiateWinSATAssessment * This,
            /* [unique][optional][in] */ __RPC__in_opt IWinSATInitiateEvents *pCallbacks,
            /* [unique][optional][in] */ __RPC__in_opt HWND callerHwnd);
        
        DECLSPEC_XFGVIRT(IInitiateWinSATAssessment, CancelAssessment)
        HRESULT ( STDMETHODCALLTYPE *CancelAssessment )( 
            __RPC__in IInitiateWinSATAssessment * This);
        
        END_INTERFACE
    } IInitiateWinSATAssessmentVtbl;

    interface IInitiateWinSATAssessment
    {
        CONST_VTBL struct IInitiateWinSATAssessmentVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInitiateWinSATAssessment_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInitiateWinSATAssessment_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInitiateWinSATAssessment_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInitiateWinSATAssessment_InitiateAssessment(This,cmdLine,pCallbacks,callerHwnd)	\
    ( (This)->lpVtbl -> InitiateAssessment(This,cmdLine,pCallbacks,callerHwnd) ) 

#define IInitiateWinSATAssessment_InitiateFormalAssessment(This,pCallbacks,callerHwnd)	\
    ( (This)->lpVtbl -> InitiateFormalAssessment(This,pCallbacks,callerHwnd) ) 

#define IInitiateWinSATAssessment_CancelAssessment(This)	\
    ( (This)->lpVtbl -> CancelAssessment(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInitiateWinSATAssessment_INTERFACE_DEFINED__ */


#ifndef __IAccessibleWinSAT_INTERFACE_DEFINED__
#define __IAccessibleWinSAT_INTERFACE_DEFINED__

/* interface IAccessibleWinSAT */
/* [uuid][object] */ 


EXTERN_C const IID IID_IAccessibleWinSAT;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("30e6018a-94a8-4ff8-a69a-71b67413f07b")
    IAccessibleWinSAT : public IAccessible
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetAccessiblityData( 
            /* [string][in] */ __RPC__in_string LPCWSTR wsName,
            /* [string][in] */ __RPC__in_string LPCWSTR wsValue,
            /* [string][in] */ __RPC__in_string LPCWSTR wsDesc) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAccessibleWinSATVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAccessibleWinSAT * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAccessibleWinSAT * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAccessibleWinSAT * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IAccessibleWinSAT * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IAccessibleWinSAT * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IAccessibleWinSAT * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IAccessibleWinSAT * This,
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
        
        DECLSPEC_XFGVIRT(IAccessible, get_accParent)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accParent )( 
            __RPC__in IAccessibleWinSAT * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppdispParent);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accChildCount)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accChildCount )( 
            __RPC__in IAccessibleWinSAT * This,
            /* [retval][out] */ __RPC__out long *pcountChildren);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accChild)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accChild )( 
            __RPC__in IAccessibleWinSAT * This,
            /* [in] */ VARIANT varChild,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppdispChild);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accName)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accName )( 
            __RPC__in IAccessibleWinSAT * This,
            /* [optional][in] */ VARIANT varChild,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pszName);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accValue)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accValue )( 
            __RPC__in IAccessibleWinSAT * This,
            /* [optional][in] */ VARIANT varChild,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pszValue);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accDescription)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accDescription )( 
            __RPC__in IAccessibleWinSAT * This,
            /* [optional][in] */ VARIANT varChild,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pszDescription);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accRole)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accRole )( 
            __RPC__in IAccessibleWinSAT * This,
            /* [optional][in] */ VARIANT varChild,
            /* [retval][out] */ __RPC__out VARIANT *pvarRole);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accState)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accState )( 
            __RPC__in IAccessibleWinSAT * This,
            /* [optional][in] */ VARIANT varChild,
            /* [retval][out] */ __RPC__out VARIANT *pvarState);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accHelp)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accHelp )( 
            __RPC__in IAccessibleWinSAT * This,
            /* [optional][in] */ VARIANT varChild,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pszHelp);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accHelpTopic)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accHelpTopic )( 
            __RPC__in IAccessibleWinSAT * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pszHelpFile,
            /* [optional][in] */ VARIANT varChild,
            /* [retval][out] */ __RPC__out long *pidTopic);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accKeyboardShortcut)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accKeyboardShortcut )( 
            __RPC__in IAccessibleWinSAT * This,
            /* [optional][in] */ VARIANT varChild,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pszKeyboardShortcut);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accFocus)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accFocus )( 
            __RPC__in IAccessibleWinSAT * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarChild);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accSelection)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accSelection )( 
            __RPC__in IAccessibleWinSAT * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarChildren);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accDefaultAction)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accDefaultAction )( 
            __RPC__in IAccessibleWinSAT * This,
            /* [optional][in] */ VARIANT varChild,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pszDefaultAction);
        
        DECLSPEC_XFGVIRT(IAccessible, accSelect)
        /* [id][hidden] */ HRESULT ( STDMETHODCALLTYPE *accSelect )( 
            __RPC__in IAccessibleWinSAT * This,
            /* [in] */ long flagsSelect,
            /* [optional][in] */ VARIANT varChild);
        
        DECLSPEC_XFGVIRT(IAccessible, accLocation)
        /* [id][hidden] */ HRESULT ( STDMETHODCALLTYPE *accLocation )( 
            __RPC__in IAccessibleWinSAT * This,
            /* [out] */ __RPC__out long *pxLeft,
            /* [out] */ __RPC__out long *pyTop,
            /* [out] */ __RPC__out long *pcxWidth,
            /* [out] */ __RPC__out long *pcyHeight,
            /* [optional][in] */ VARIANT varChild);
        
        DECLSPEC_XFGVIRT(IAccessible, accNavigate)
        /* [id][hidden] */ HRESULT ( STDMETHODCALLTYPE *accNavigate )( 
            __RPC__in IAccessibleWinSAT * This,
            /* [in] */ long navDir,
            /* [optional][in] */ VARIANT varStart,
            /* [retval][out] */ __RPC__out VARIANT *pvarEndUpAt);
        
        DECLSPEC_XFGVIRT(IAccessible, accHitTest)
        /* [id][hidden] */ HRESULT ( STDMETHODCALLTYPE *accHitTest )( 
            __RPC__in IAccessibleWinSAT * This,
            /* [in] */ long xLeft,
            /* [in] */ long yTop,
            /* [retval][out] */ __RPC__out VARIANT *pvarChild);
        
        DECLSPEC_XFGVIRT(IAccessible, accDoDefaultAction)
        /* [id][hidden] */ HRESULT ( STDMETHODCALLTYPE *accDoDefaultAction )( 
            __RPC__in IAccessibleWinSAT * This,
            /* [optional][in] */ VARIANT varChild);
        
        DECLSPEC_XFGVIRT(IAccessible, put_accName)
        /* [id][propput][hidden] */ HRESULT ( STDMETHODCALLTYPE *put_accName )( 
            __RPC__in IAccessibleWinSAT * This,
            /* [optional][in] */ VARIANT varChild,
            /* [in] */ __RPC__in BSTR szName);
        
        DECLSPEC_XFGVIRT(IAccessible, put_accValue)
        /* [id][propput][hidden] */ HRESULT ( STDMETHODCALLTYPE *put_accValue )( 
            __RPC__in IAccessibleWinSAT * This,
            /* [optional][in] */ VARIANT varChild,
            /* [in] */ __RPC__in BSTR szValue);
        
        DECLSPEC_XFGVIRT(IAccessibleWinSAT, SetAccessiblityData)
        HRESULT ( STDMETHODCALLTYPE *SetAccessiblityData )( 
            __RPC__in IAccessibleWinSAT * This,
            /* [string][in] */ __RPC__in_string LPCWSTR wsName,
            /* [string][in] */ __RPC__in_string LPCWSTR wsValue,
            /* [string][in] */ __RPC__in_string LPCWSTR wsDesc);
        
        END_INTERFACE
    } IAccessibleWinSATVtbl;

    interface IAccessibleWinSAT
    {
        CONST_VTBL struct IAccessibleWinSATVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAccessibleWinSAT_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAccessibleWinSAT_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAccessibleWinSAT_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAccessibleWinSAT_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IAccessibleWinSAT_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IAccessibleWinSAT_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IAccessibleWinSAT_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IAccessibleWinSAT_get_accParent(This,ppdispParent)	\
    ( (This)->lpVtbl -> get_accParent(This,ppdispParent) ) 

#define IAccessibleWinSAT_get_accChildCount(This,pcountChildren)	\
    ( (This)->lpVtbl -> get_accChildCount(This,pcountChildren) ) 

#define IAccessibleWinSAT_get_accChild(This,varChild,ppdispChild)	\
    ( (This)->lpVtbl -> get_accChild(This,varChild,ppdispChild) ) 

#define IAccessibleWinSAT_get_accName(This,varChild,pszName)	\
    ( (This)->lpVtbl -> get_accName(This,varChild,pszName) ) 

#define IAccessibleWinSAT_get_accValue(This,varChild,pszValue)	\
    ( (This)->lpVtbl -> get_accValue(This,varChild,pszValue) ) 

#define IAccessibleWinSAT_get_accDescription(This,varChild,pszDescription)	\
    ( (This)->lpVtbl -> get_accDescription(This,varChild,pszDescription) ) 

#define IAccessibleWinSAT_get_accRole(This,varChild,pvarRole)	\
    ( (This)->lpVtbl -> get_accRole(This,varChild,pvarRole) ) 

#define IAccessibleWinSAT_get_accState(This,varChild,pvarState)	\
    ( (This)->lpVtbl -> get_accState(This,varChild,pvarState) ) 

#define IAccessibleWinSAT_get_accHelp(This,varChild,pszHelp)	\
    ( (This)->lpVtbl -> get_accHelp(This,varChild,pszHelp) ) 

#define IAccessibleWinSAT_get_accHelpTopic(This,pszHelpFile,varChild,pidTopic)	\
    ( (This)->lpVtbl -> get_accHelpTopic(This,pszHelpFile,varChild,pidTopic) ) 

#define IAccessibleWinSAT_get_accKeyboardShortcut(This,varChild,pszKeyboardShortcut)	\
    ( (This)->lpVtbl -> get_accKeyboardShortcut(This,varChild,pszKeyboardShortcut) ) 

#define IAccessibleWinSAT_get_accFocus(This,pvarChild)	\
    ( (This)->lpVtbl -> get_accFocus(This,pvarChild) ) 

#define IAccessibleWinSAT_get_accSelection(This,pvarChildren)	\
    ( (This)->lpVtbl -> get_accSelection(This,pvarChildren) ) 

#define IAccessibleWinSAT_get_accDefaultAction(This,varChild,pszDefaultAction)	\
    ( (This)->lpVtbl -> get_accDefaultAction(This,varChild,pszDefaultAction) ) 

#define IAccessibleWinSAT_accSelect(This,flagsSelect,varChild)	\
    ( (This)->lpVtbl -> accSelect(This,flagsSelect,varChild) ) 

#define IAccessibleWinSAT_accLocation(This,pxLeft,pyTop,pcxWidth,pcyHeight,varChild)	\
    ( (This)->lpVtbl -> accLocation(This,pxLeft,pyTop,pcxWidth,pcyHeight,varChild) ) 

#define IAccessibleWinSAT_accNavigate(This,navDir,varStart,pvarEndUpAt)	\
    ( (This)->lpVtbl -> accNavigate(This,navDir,varStart,pvarEndUpAt) ) 

#define IAccessibleWinSAT_accHitTest(This,xLeft,yTop,pvarChild)	\
    ( (This)->lpVtbl -> accHitTest(This,xLeft,yTop,pvarChild) ) 

#define IAccessibleWinSAT_accDoDefaultAction(This,varChild)	\
    ( (This)->lpVtbl -> accDoDefaultAction(This,varChild) ) 

#define IAccessibleWinSAT_put_accName(This,varChild,szName)	\
    ( (This)->lpVtbl -> put_accName(This,varChild,szName) ) 

#define IAccessibleWinSAT_put_accValue(This,varChild,szValue)	\
    ( (This)->lpVtbl -> put_accValue(This,varChild,szValue) ) 


#define IAccessibleWinSAT_SetAccessiblityData(This,wsName,wsValue,wsDesc)	\
    ( (This)->lpVtbl -> SetAccessiblityData(This,wsName,wsValue,wsDesc) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAccessibleWinSAT_INTERFACE_DEFINED__ */


#ifndef __IQueryOEMWinSATCustomization_INTERFACE_DEFINED__
#define __IQueryOEMWinSATCustomization_INTERFACE_DEFINED__

/* interface IQueryOEMWinSATCustomization */
/* [uuid][object] */ 


EXTERN_C const IID IID_IQueryOEMWinSATCustomization;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("BC9A6A9F-AD4E-420e-9953-B34671E9DF22")
    IQueryOEMWinSATCustomization : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetOEMPrePopulationInfo( 
            /* [retval][out] */ __RPC__out WINSAT_OEM_CUSTOMIZATION_STATE *state) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IQueryOEMWinSATCustomizationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IQueryOEMWinSATCustomization * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IQueryOEMWinSATCustomization * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IQueryOEMWinSATCustomization * This);
        
        DECLSPEC_XFGVIRT(IQueryOEMWinSATCustomization, GetOEMPrePopulationInfo)
        HRESULT ( STDMETHODCALLTYPE *GetOEMPrePopulationInfo )( 
            __RPC__in IQueryOEMWinSATCustomization * This,
            /* [retval][out] */ __RPC__out WINSAT_OEM_CUSTOMIZATION_STATE *state);
        
        END_INTERFACE
    } IQueryOEMWinSATCustomizationVtbl;

    interface IQueryOEMWinSATCustomization
    {
        CONST_VTBL struct IQueryOEMWinSATCustomizationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IQueryOEMWinSATCustomization_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IQueryOEMWinSATCustomization_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IQueryOEMWinSATCustomization_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IQueryOEMWinSATCustomization_GetOEMPrePopulationInfo(This,state)	\
    ( (This)->lpVtbl -> GetOEMPrePopulationInfo(This,state) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IQueryOEMWinSATCustomization_INTERFACE_DEFINED__ */



#ifndef __WINSATLib_LIBRARY_DEFINED__
#define __WINSATLib_LIBRARY_DEFINED__

/* library WINSATLib */
/* [helpstring][version][uuid] */ 


EXTERN_C const IID LIBID_WINSATLib;

EXTERN_C const CLSID CLSID_CInitiateWinSAT;

#ifdef __cplusplus

class DECLSPEC_UUID("489331DC-F5E0-4528-9FDA-45331BF4A571")
CInitiateWinSAT;
#endif

EXTERN_C const CLSID CLSID_CQueryWinSAT;

#ifdef __cplusplus

class DECLSPEC_UUID("F3BDFAD3-F276-49e9-9B17-C474F48F0764")
CQueryWinSAT;
#endif

EXTERN_C const CLSID CLSID_CQueryAllWinSAT;

#ifdef __cplusplus

class DECLSPEC_UUID("05DF8D13-C355-47f4-A11E-851B338CEFB8")
CQueryAllWinSAT;
#endif

EXTERN_C const CLSID CLSID_CProvideWinSATVisuals;

#ifdef __cplusplus

class DECLSPEC_UUID("9F377D7E-E551-44f8-9F94-9DB392B03B7B")
CProvideWinSATVisuals;
#endif

EXTERN_C const CLSID CLSID_CAccessiblityWinSAT;

#ifdef __cplusplus

class DECLSPEC_UUID("6e18f9c6-a3eb-495a-89b7-956482e19f7a")
CAccessiblityWinSAT;
#endif

EXTERN_C const CLSID CLSID_CQueryOEMWinSATCustomization;

#ifdef __cplusplus

class DECLSPEC_UUID("C47A41B7-B729-424f-9AF9-5CB3934F2DFA")
CQueryOEMWinSATCustomization;
#endif
#endif /* __WINSATLib_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_winsatcominterfacei_0000_0010 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_winsatcominterfacei_0000_0010_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_winsatcominterfacei_0000_0010_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  BSTR_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  HBITMAP_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in HBITMAP * ); 
unsigned char * __RPC_USER  HBITMAP_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HBITMAP * ); 
unsigned char * __RPC_USER  HBITMAP_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HBITMAP * ); 
void                      __RPC_USER  HBITMAP_UserFree(     __RPC__in unsigned long *, __RPC__in HBITMAP * ); 

unsigned long             __RPC_USER  HWND_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HWND * ); 
void                      __RPC_USER  HWND_UserFree(     __RPC__in unsigned long *, __RPC__in HWND * ); 

unsigned long             __RPC_USER  VARIANT_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

unsigned long             __RPC_USER  BSTR_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree64(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  HBITMAP_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in HBITMAP * ); 
unsigned char * __RPC_USER  HBITMAP_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HBITMAP * ); 
unsigned char * __RPC_USER  HBITMAP_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HBITMAP * ); 
void                      __RPC_USER  HBITMAP_UserFree64(     __RPC__in unsigned long *, __RPC__in HBITMAP * ); 

unsigned long             __RPC_USER  HWND_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HWND * ); 
void                      __RPC_USER  HWND_UserFree64(     __RPC__in unsigned long *, __RPC__in HWND * ); 

unsigned long             __RPC_USER  VARIANT_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree64(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


