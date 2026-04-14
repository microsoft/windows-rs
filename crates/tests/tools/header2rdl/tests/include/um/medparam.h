

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

#ifndef __medparam_h__
#define __medparam_h__

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

#ifndef __IMediaParamInfo_FWD_DEFINED__
#define __IMediaParamInfo_FWD_DEFINED__
typedef interface IMediaParamInfo IMediaParamInfo;

#endif 	/* __IMediaParamInfo_FWD_DEFINED__ */


#ifndef __IMediaParams_FWD_DEFINED__
#define __IMediaParams_FWD_DEFINED__
typedef interface IMediaParams IMediaParams;

#endif 	/* __IMediaParams_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"
#include "strmif.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_medparam_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
typedef float MP_DATA;

typedef 
enum _MP_Type
    {
        MPT_INT	= 0,
        MPT_FLOAT	= ( MPT_INT + 1 ) ,
        MPT_BOOL	= ( MPT_FLOAT + 1 ) ,
        MPT_ENUM	= ( MPT_BOOL + 1 ) ,
        MPT_MAX	= ( MPT_ENUM + 1 ) 
    } 	MP_TYPE;

#define	MPBOOL_TRUE	( 1 )

#define	MPBOOL_FALSE	( 0 )

typedef 
enum _MP_CURVE_TYPE
    {
        MP_CURVE_JUMP	= 0x1,
        MP_CURVE_LINEAR	= 0x2,
        MP_CURVE_SQUARE	= 0x4,
        MP_CURVE_INVSQUARE	= 0x8,
        MP_CURVE_SINE	= 0x10
    } 	MP_CURVE_TYPE;

typedef DWORD MP_CAPS;

#define	MP_CAPS_CURVE_JUMP	( MP_CURVE_JUMP )

#define	MP_CAPS_CURVE_LINEAR	( MP_CURVE_LINEAR )

#define	MP_CAPS_CURVE_SQUARE	( MP_CURVE_SQUARE )

#define	MP_CAPS_CURVE_INVSQUARE	( MP_CURVE_INVSQUARE )

#define	MP_CAPS_CURVE_SINE	( MP_CURVE_SINE )

typedef struct _MP_PARAMINFO
    {
    MP_TYPE mpType;
    MP_CAPS mopCaps;
    MP_DATA mpdMinValue;
    MP_DATA mpdMaxValue;
    MP_DATA mpdNeutralValue;
    WCHAR szUnitText[ 32 ];
    WCHAR szLabel[ 32 ];
    } 	MP_PARAMINFO;

typedef DWORD DWORD;

#define	DWORD_ALLPARAMS	( -1 )

typedef DWORD MP_TIMEDATA;

DEFINE_GUID(GUID_TIME_REFERENCE,
0x93ad712b, 0xdaa0, 0x4ffe, 0xbc, 0x81, 0xb0, 0xce, 0x50, 0xf, 0xcd, 0xd9);
DEFINE_GUID(GUID_TIME_MUSIC,
0x574c49d, 0x5b04, 0x4b15, 0xa5, 0x42, 0xae, 0x28, 0x20, 0x30, 0x11, 0x7b);
DEFINE_GUID(GUID_TIME_SAMPLES,
0xa8593d05, 0xc43, 0x4984, 0x9a, 0x63, 0x97, 0xaf, 0x9e, 0x2, 0xc4, 0xc0);
typedef DWORD MP_FLAGS;

#define	MPF_ENVLP_STANDARD	( 0 )

#define	MPF_ENVLP_BEGIN_CURRENTVAL	( 0x1 )

#define	MPF_ENVLP_BEGIN_NEUTRALVAL	( 0x2 )

typedef struct _MP_ENVELOPE_SEGMENT
    {
    REFERENCE_TIME rtStart;
    REFERENCE_TIME rtEnd;
    MP_DATA valStart;
    MP_DATA valEnd;
    MP_CURVE_TYPE iCurve;
    MP_FLAGS flags;
    } 	MP_ENVELOPE_SEGMENT;

#define	MPF_PUNCHIN_REFTIME	( 0 )

#define	MPF_PUNCHIN_NOW	( 0x1 )

#define	MPF_PUNCHIN_STOPPED	( 0x2 )



extern RPC_IF_HANDLE __MIDL_itf_medparam_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_medparam_0000_0000_v0_0_s_ifspec;

#ifndef __IMediaParamInfo_INTERFACE_DEFINED__
#define __IMediaParamInfo_INTERFACE_DEFINED__

/* interface IMediaParamInfo */
/* [version][uuid][object] */ 


EXTERN_C const IID IID_IMediaParamInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6d6cbb60-a223-44aa-842f-a2f06750be6d")
    IMediaParamInfo : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetParamCount( 
            /* [out] */ __RPC__out DWORD *pdwParams) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetParamInfo( 
            /* [in] */ DWORD dwParamIndex,
            /* [out] */ __RPC__out MP_PARAMINFO *pInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetParamText( 
            /* [in] */ DWORD dwParamIndex,
            /* [out] */ __RPC__deref_out_opt WCHAR **ppwchText) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNumTimeFormats( 
            /* [out] */ __RPC__out DWORD *pdwNumTimeFormats) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSupportedTimeFormat( 
            /* [in] */ DWORD dwFormatIndex,
            /* [out] */ __RPC__out GUID *pguidTimeFormat) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCurrentTimeFormat( 
            /* [out] */ __RPC__out GUID *pguidTimeFormat,
            /* [out] */ __RPC__out MP_TIMEDATA *pTimeData) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMediaParamInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMediaParamInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMediaParamInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMediaParamInfo * This);
        
        DECLSPEC_XFGVIRT(IMediaParamInfo, GetParamCount)
        HRESULT ( STDMETHODCALLTYPE *GetParamCount )( 
            __RPC__in IMediaParamInfo * This,
            /* [out] */ __RPC__out DWORD *pdwParams);
        
        DECLSPEC_XFGVIRT(IMediaParamInfo, GetParamInfo)
        HRESULT ( STDMETHODCALLTYPE *GetParamInfo )( 
            __RPC__in IMediaParamInfo * This,
            /* [in] */ DWORD dwParamIndex,
            /* [out] */ __RPC__out MP_PARAMINFO *pInfo);
        
        DECLSPEC_XFGVIRT(IMediaParamInfo, GetParamText)
        HRESULT ( STDMETHODCALLTYPE *GetParamText )( 
            __RPC__in IMediaParamInfo * This,
            /* [in] */ DWORD dwParamIndex,
            /* [out] */ __RPC__deref_out_opt WCHAR **ppwchText);
        
        DECLSPEC_XFGVIRT(IMediaParamInfo, GetNumTimeFormats)
        HRESULT ( STDMETHODCALLTYPE *GetNumTimeFormats )( 
            __RPC__in IMediaParamInfo * This,
            /* [out] */ __RPC__out DWORD *pdwNumTimeFormats);
        
        DECLSPEC_XFGVIRT(IMediaParamInfo, GetSupportedTimeFormat)
        HRESULT ( STDMETHODCALLTYPE *GetSupportedTimeFormat )( 
            __RPC__in IMediaParamInfo * This,
            /* [in] */ DWORD dwFormatIndex,
            /* [out] */ __RPC__out GUID *pguidTimeFormat);
        
        DECLSPEC_XFGVIRT(IMediaParamInfo, GetCurrentTimeFormat)
        HRESULT ( STDMETHODCALLTYPE *GetCurrentTimeFormat )( 
            __RPC__in IMediaParamInfo * This,
            /* [out] */ __RPC__out GUID *pguidTimeFormat,
            /* [out] */ __RPC__out MP_TIMEDATA *pTimeData);
        
        END_INTERFACE
    } IMediaParamInfoVtbl;

    interface IMediaParamInfo
    {
        CONST_VTBL struct IMediaParamInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMediaParamInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMediaParamInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMediaParamInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMediaParamInfo_GetParamCount(This,pdwParams)	\
    ( (This)->lpVtbl -> GetParamCount(This,pdwParams) ) 

#define IMediaParamInfo_GetParamInfo(This,dwParamIndex,pInfo)	\
    ( (This)->lpVtbl -> GetParamInfo(This,dwParamIndex,pInfo) ) 

#define IMediaParamInfo_GetParamText(This,dwParamIndex,ppwchText)	\
    ( (This)->lpVtbl -> GetParamText(This,dwParamIndex,ppwchText) ) 

#define IMediaParamInfo_GetNumTimeFormats(This,pdwNumTimeFormats)	\
    ( (This)->lpVtbl -> GetNumTimeFormats(This,pdwNumTimeFormats) ) 

#define IMediaParamInfo_GetSupportedTimeFormat(This,dwFormatIndex,pguidTimeFormat)	\
    ( (This)->lpVtbl -> GetSupportedTimeFormat(This,dwFormatIndex,pguidTimeFormat) ) 

#define IMediaParamInfo_GetCurrentTimeFormat(This,pguidTimeFormat,pTimeData)	\
    ( (This)->lpVtbl -> GetCurrentTimeFormat(This,pguidTimeFormat,pTimeData) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMediaParamInfo_INTERFACE_DEFINED__ */


#ifndef __IMediaParams_INTERFACE_DEFINED__
#define __IMediaParams_INTERFACE_DEFINED__

/* interface IMediaParams */
/* [version][uuid][object] */ 


EXTERN_C const IID IID_IMediaParams;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6d6cbb61-a223-44aa-842f-a2f06750be6e")
    IMediaParams : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetParam( 
            /* [in] */ DWORD dwParamIndex,
            /* [out] */ __RPC__out MP_DATA *pValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetParam( 
            /* [in] */ DWORD dwParamIndex,
            /* [in] */ MP_DATA value) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddEnvelope( 
            /* [in] */ DWORD dwParamIndex,
            /* [in] */ DWORD cSegments,
            /* [in] */ __RPC__in MP_ENVELOPE_SEGMENT *pEnvelopeSegments) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FlushEnvelope( 
            /* [in] */ DWORD dwParamIndex,
            /* [in] */ REFERENCE_TIME refTimeStart,
            /* [in] */ REFERENCE_TIME refTimeEnd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetTimeFormat( 
            /* [in] */ GUID guidTimeFormat,
            /* [in] */ MP_TIMEDATA mpTimeData) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMediaParamsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMediaParams * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMediaParams * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMediaParams * This);
        
        DECLSPEC_XFGVIRT(IMediaParams, GetParam)
        HRESULT ( STDMETHODCALLTYPE *GetParam )( 
            __RPC__in IMediaParams * This,
            /* [in] */ DWORD dwParamIndex,
            /* [out] */ __RPC__out MP_DATA *pValue);
        
        DECLSPEC_XFGVIRT(IMediaParams, SetParam)
        HRESULT ( STDMETHODCALLTYPE *SetParam )( 
            __RPC__in IMediaParams * This,
            /* [in] */ DWORD dwParamIndex,
            /* [in] */ MP_DATA value);
        
        DECLSPEC_XFGVIRT(IMediaParams, AddEnvelope)
        HRESULT ( STDMETHODCALLTYPE *AddEnvelope )( 
            __RPC__in IMediaParams * This,
            /* [in] */ DWORD dwParamIndex,
            /* [in] */ DWORD cSegments,
            /* [in] */ __RPC__in MP_ENVELOPE_SEGMENT *pEnvelopeSegments);
        
        DECLSPEC_XFGVIRT(IMediaParams, FlushEnvelope)
        HRESULT ( STDMETHODCALLTYPE *FlushEnvelope )( 
            __RPC__in IMediaParams * This,
            /* [in] */ DWORD dwParamIndex,
            /* [in] */ REFERENCE_TIME refTimeStart,
            /* [in] */ REFERENCE_TIME refTimeEnd);
        
        DECLSPEC_XFGVIRT(IMediaParams, SetTimeFormat)
        HRESULT ( STDMETHODCALLTYPE *SetTimeFormat )( 
            __RPC__in IMediaParams * This,
            /* [in] */ GUID guidTimeFormat,
            /* [in] */ MP_TIMEDATA mpTimeData);
        
        END_INTERFACE
    } IMediaParamsVtbl;

    interface IMediaParams
    {
        CONST_VTBL struct IMediaParamsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMediaParams_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMediaParams_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMediaParams_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMediaParams_GetParam(This,dwParamIndex,pValue)	\
    ( (This)->lpVtbl -> GetParam(This,dwParamIndex,pValue) ) 

#define IMediaParams_SetParam(This,dwParamIndex,value)	\
    ( (This)->lpVtbl -> SetParam(This,dwParamIndex,value) ) 

#define IMediaParams_AddEnvelope(This,dwParamIndex,cSegments,pEnvelopeSegments)	\
    ( (This)->lpVtbl -> AddEnvelope(This,dwParamIndex,cSegments,pEnvelopeSegments) ) 

#define IMediaParams_FlushEnvelope(This,dwParamIndex,refTimeStart,refTimeEnd)	\
    ( (This)->lpVtbl -> FlushEnvelope(This,dwParamIndex,refTimeStart,refTimeEnd) ) 

#define IMediaParams_SetTimeFormat(This,guidTimeFormat,mpTimeData)	\
    ( (This)->lpVtbl -> SetTimeFormat(This,guidTimeFormat,mpTimeData) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMediaParams_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_medparam_0000_0002 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_medparam_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_medparam_0000_0002_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


