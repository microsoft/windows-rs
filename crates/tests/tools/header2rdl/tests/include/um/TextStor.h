

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

#ifndef __textstor_h__
#define __textstor_h__

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

#ifndef __ITextStoreACP_FWD_DEFINED__
#define __ITextStoreACP_FWD_DEFINED__
typedef interface ITextStoreACP ITextStoreACP;

#endif 	/* __ITextStoreACP_FWD_DEFINED__ */


#ifndef __ITextStoreACP2_FWD_DEFINED__
#define __ITextStoreACP2_FWD_DEFINED__
typedef interface ITextStoreACP2 ITextStoreACP2;

#endif 	/* __ITextStoreACP2_FWD_DEFINED__ */


#ifndef __ITextStoreACPSink_FWD_DEFINED__
#define __ITextStoreACPSink_FWD_DEFINED__
typedef interface ITextStoreACPSink ITextStoreACPSink;

#endif 	/* __ITextStoreACPSink_FWD_DEFINED__ */


#ifndef __IAnchor_FWD_DEFINED__
#define __IAnchor_FWD_DEFINED__
typedef interface IAnchor IAnchor;

#endif 	/* __IAnchor_FWD_DEFINED__ */


#ifndef __ITextStoreAnchor_FWD_DEFINED__
#define __ITextStoreAnchor_FWD_DEFINED__
typedef interface ITextStoreAnchor ITextStoreAnchor;

#endif 	/* __ITextStoreAnchor_FWD_DEFINED__ */


#ifndef __ITextStoreAnchorSink_FWD_DEFINED__
#define __ITextStoreAnchorSink_FWD_DEFINED__
typedef interface ITextStoreAnchorSink ITextStoreAnchorSink;

#endif 	/* __ITextStoreAnchorSink_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_textstor_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)

DEFINE_GUID (GUID_TS_SERVICE_DATAOBJECT, 0x6086fbb5, 0xe225, 0x46ce, 0xa7, 0x70, 0xc1, 0xbb, 0xd3, 0xe0, 0x5d, 0x7b);
DEFINE_GUID (GUID_TS_SERVICE_ACCESSIBLE, 0xf9786200, 0xa5bf, 0x4a0f, 0x8c, 0x24, 0xfb, 0x16, 0xf5, 0xd1, 0xaa, 0xbb);
DEFINE_GUID (GUID_TS_SERVICE_ACTIVEX,    0xea937a50, 0xc9a6, 0x4b7d, 0x89, 0x4a, 0x49, 0xd9, 0x9b, 0x78, 0x48, 0x34);
#define TS_E_INVALIDPOS      MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0x0200)
#define TS_E_NOLOCK          MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0x0201)
#define TS_E_NOOBJECT        MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0x0202)
#define TS_E_NOSERVICE       MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0x0203)
#define TS_E_NOINTERFACE     MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0x0204)
#define TS_E_NOSELECTION     MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0x0205)
#define TS_E_NOLAYOUT        MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0x0206)
#define TS_E_INVALIDPOINT    MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0x0207)
#define TS_E_SYNCHRONOUS     MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0x0208)
#define TS_E_READONLY        MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0x0209)
#define TS_E_FORMAT          MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0x020a)
#define TS_S_ASYNC           MAKE_HRESULT(SEVERITY_SUCCESS, FACILITY_ITF, 0x0300)
#define	TS_AS_TEXT_CHANGE	( 0x1 )

#define	TS_AS_SEL_CHANGE	( 0x2 )

#define	TS_AS_LAYOUT_CHANGE	( 0x4 )

#define	TS_AS_ATTR_CHANGE	( 0x8 )

#define	TS_AS_STATUS_CHANGE	( 0x10 )

#define	TS_AS_ALL_SINKS	( ( ( ( ( TS_AS_TEXT_CHANGE | TS_AS_SEL_CHANGE )  | TS_AS_LAYOUT_CHANGE )  | TS_AS_ATTR_CHANGE )  | TS_AS_STATUS_CHANGE )  )

#define	TS_LF_SYNC	( 0x1 )

#define	TS_LF_READ	( 0x2 )

#define	TS_LF_READWRITE	( 0x6 )

#define	TS_SD_READONLY	( 0x1 )

#define	TS_SD_LOADING	( 0x2 )

#define	TS_SD_RESERVED	( 0x4 )

#define	TS_SD_TKBAUTOCORRECTENABLE	( 0x8 )

#define	TS_SD_TKBPREDICTIONENABLE	( 0x10 )

#define	TS_SD_UIINTEGRATIONENABLE	( 0x20 )

#define	TS_SD_INPUTPANEMANUALDISPLAYENABLE	( 0x40 )

#define	TS_SD_EMBEDDEDHANDWRITINGVIEW_ENABLED	( 0x80 )

#define	TS_SD_EMBEDDEDHANDWRITINGVIEW_VISIBLE	( 0x100 )

#define	TS_SS_DISJOINTSEL	( 0x1 )

#define	TS_SS_REGIONS	( 0x2 )

#define	TS_SS_TRANSITORY	( 0x4 )

#define	TS_SS_NOHIDDENTEXT	( 0x8 )

#define	TS_SS_TKBAUTOCORRECTENABLE	( 0x10 )

#define	TS_SS_TKBPREDICTIONENABLE	( 0x20 )

#define	TS_SS_UWPCONTROL	( 0x40 )

#define	TS_SD_MASKALL	( ( TS_SD_READONLY | TS_SD_LOADING )  )

#define	TS_ST_CORRECTION	( 0x1 )

#define	TS_IE_CORRECTION	( 0x1 )

#define	TS_IE_COMPOSITION	( 0x2 )

#define	TS_TC_CORRECTION	( 0x1 )

#define	TS_IAS_NOQUERY	( 0x1 )

#define	TS_IAS_QUERYONLY	( 0x2 )

typedef /* [uuid] */  DECLSPEC_UUID("fec4f516-c503-45b1-a5fd-7a3d8ab07049") struct TS_STATUS
    {
    DWORD dwDynamicFlags;
    DWORD dwStaticFlags;
    } 	TS_STATUS;

typedef /* [uuid] */  DECLSPEC_UUID("f3181bd6-bcf0-41d3-a81c-474b17ec38fb") struct TS_TEXTCHANGE
    {
    LONG acpStart;
    LONG acpOldEnd;
    LONG acpNewEnd;
    } 	TS_TEXTCHANGE;

typedef /* [public][public][public][public][public][public][public][public][public][uuid] */  DECLSPEC_UUID("05fcf85b-5e9c-4c3e-ab71-29471d4f38e7") 
enum __MIDL___MIDL_itf_textstor_0000_0000_0001
    {
        TS_AE_NONE	= 0,
        TS_AE_START	= 1,
        TS_AE_END	= 2
    } 	TsActiveSelEnd;

typedef /* [uuid] */  DECLSPEC_UUID("7ecc3ffa-8f73-4d91-98ed-76f8ac5b1600") struct TS_SELECTIONSTYLE
    {
    TsActiveSelEnd ase;
    BOOL fInterimChar;
    } 	TS_SELECTIONSTYLE;

typedef /* [uuid] */  DECLSPEC_UUID("c4b9c33b-8a0d-4426-bebe-d444a4701fe9") struct TS_SELECTION_ACP
    {
    LONG acpStart;
    LONG acpEnd;
    TS_SELECTIONSTYLE style;
    } 	TS_SELECTION_ACP;

typedef /* [uuid] */  DECLSPEC_UUID("b03413d2-0723-4c4e-9e08-2e9c1ff3772b") struct TS_SELECTION_ANCHOR
    {
    IAnchor *paStart;
    IAnchor *paEnd;
    TS_SELECTIONSTYLE style;
    } 	TS_SELECTION_ANCHOR;

#define	TS_DEFAULT_SELECTION	( ( ULONG  )-1 )

#define	GXFPF_ROUND_NEAREST	( 0x1 )

#define	GXFPF_NEAREST	( 0x2 )

#define	TS_CHAR_EMBEDDED	( 0xfffc )

#define	TS_CHAR_REGION	( 0 )

#define	TS_CHAR_REPLACEMENT	( 0xfffd )

typedef /* [uuid] */  DECLSPEC_UUID("ef3457d9-8446-49a7-a9e6-b50d9d5f3fd9") GUID TS_ATTRID;

typedef /* [uuid] */  DECLSPEC_UUID("2cc2b33f-1174-4507-b8d9-5bc0eb37c197") struct TS_ATTRVAL
    {
    TS_ATTRID idAttr;
    DWORD dwOverlapId;
    VARIANT varValue;
    } 	TS_ATTRVAL;

#define	TS_ATTR_FIND_BACKWARDS	( 0x1 )

#define	TS_ATTR_FIND_WANT_OFFSET	( 0x2 )

#define	TS_ATTR_FIND_UPDATESTART	( 0x4 )

#define	TS_ATTR_FIND_WANT_VALUE	( 0x8 )

#define	TS_ATTR_FIND_WANT_END	( 0x10 )

#define	TS_ATTR_FIND_HIDDEN	( 0x20 )

typedef /* [uuid] */  DECLSPEC_UUID("1faf509e-44c1-458e-950a-38a96705a62b") DWORD TsViewCookie;

#define	TS_VCOOKIE_NUL	( 0xffffffff )

typedef /* [public][public][public][uuid] */  DECLSPEC_UUID("7899d7c4-5f07-493c-a89a-fac8e777f476") 
enum __MIDL___MIDL_itf_textstor_0000_0000_0002
    {
        TS_LC_CREATE	= 0,
        TS_LC_CHANGE	= 1,
        TS_LC_DESTROY	= 2
    } 	TsLayoutCode;

typedef /* [public][public][public][uuid] */  DECLSPEC_UUID("033b0df0-f193-4170-b47b-141afc247878") 
enum __MIDL___MIDL_itf_textstor_0000_0000_0003
    {
        TS_RT_PLAIN	= 0,
        TS_RT_HIDDEN	= ( TS_RT_PLAIN + 1 ) ,
        TS_RT_OPAQUE	= ( TS_RT_HIDDEN + 1 ) 
    } 	TsRunType;

typedef /* [uuid] */  DECLSPEC_UUID("a6231949-37c5-4b74-a24e-2a26c327201d") struct TS_RUNINFO
    {
    ULONG uCount;
    TsRunType type;
    } 	TS_RUNINFO;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_textstor_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_textstor_0000_0000_v0_0_s_ifspec;

#ifndef __ITextStoreACP_INTERFACE_DEFINED__
#define __ITextStoreACP_INTERFACE_DEFINED__

/* interface ITextStoreACP */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ITextStoreACP;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("28888fe3-c2a0-483a-a3ea-8cb1ce51ff3d")
    ITextStoreACP : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AdviseSink( 
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][in] */ __RPC__in_opt IUnknown *punk,
            /* [in] */ DWORD dwMask) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnadviseSink( 
            /* [in] */ __RPC__in_opt IUnknown *punk) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RequestLock( 
            /* [in] */ DWORD dwLockFlags,
            /* [out] */ __RPC__out HRESULT *phrSession) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStatus( 
            /* [out] */ __RPC__out TS_STATUS *pdcs) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE QueryInsert( 
            /* [in] */ LONG acpTestStart,
            /* [in] */ LONG acpTestEnd,
            /* [in] */ ULONG cch,
            /* [out] */ __RPC__out LONG *pacpResultStart,
            /* [out] */ __RPC__out LONG *pacpResultEnd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSelection( 
            /* [in] */ ULONG ulIndex,
            /* [in] */ ULONG ulCount,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(ulCount, *pcFetched) TS_SELECTION_ACP *pSelection,
            /* [out] */ __RPC__out ULONG *pcFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetSelection( 
            /* [in] */ ULONG ulCount,
            /* [size_is][in] */ __RPC__in_ecount_full(ulCount) const TS_SELECTION_ACP *pSelection) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetText( 
            /* [in] */ LONG acpStart,
            /* [in] */ LONG acpEnd,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(cchPlainReq, *pcchPlainRet) WCHAR *pchPlain,
            /* [in] */ ULONG cchPlainReq,
            /* [out] */ __RPC__out ULONG *pcchPlainRet,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(cRunInfoReq, *pcRunInfoRet) TS_RUNINFO *prgRunInfo,
            /* [in] */ ULONG cRunInfoReq,
            /* [out] */ __RPC__out ULONG *pcRunInfoRet,
            /* [out] */ __RPC__out LONG *pacpNext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetText( 
            /* [in] */ DWORD dwFlags,
            /* [in] */ LONG acpStart,
            /* [in] */ LONG acpEnd,
            /* [size_is][in] */ __RPC__in_ecount_full(cch) const WCHAR *pchText,
            /* [in] */ ULONG cch,
            /* [out] */ __RPC__out TS_TEXTCHANGE *pChange) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFormattedText( 
            /* [in] */ LONG acpStart,
            /* [in] */ LONG acpEnd,
            /* [out] */ __RPC__deref_out_opt IDataObject **ppDataObject) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetEmbedded( 
            /* [in] */ LONG acpPos,
            /* [in] */ __RPC__in REFGUID rguidService,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt IUnknown **ppunk) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE QueryInsertEmbedded( 
            /* [in] */ __RPC__in const GUID *pguidService,
            /* [in] */ __RPC__in const FORMATETC *pFormatEtc,
            /* [out] */ __RPC__out BOOL *pfInsertable) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InsertEmbedded( 
            /* [in] */ DWORD dwFlags,
            /* [in] */ LONG acpStart,
            /* [in] */ LONG acpEnd,
            /* [in] */ __RPC__in_opt IDataObject *pDataObject,
            /* [out] */ __RPC__out TS_TEXTCHANGE *pChange) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InsertTextAtSelection( 
            /* [in] */ DWORD dwFlags,
            /* [size_is][in] */ __RPC__in_ecount_full(cch) const WCHAR *pchText,
            /* [in] */ ULONG cch,
            /* [out] */ __RPC__out LONG *pacpStart,
            /* [out] */ __RPC__out LONG *pacpEnd,
            /* [out] */ __RPC__out TS_TEXTCHANGE *pChange) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InsertEmbeddedAtSelection( 
            /* [in] */ DWORD dwFlags,
            /* [in] */ __RPC__in_opt IDataObject *pDataObject,
            /* [out] */ __RPC__out LONG *pacpStart,
            /* [out] */ __RPC__out LONG *pacpEnd,
            /* [out] */ __RPC__out TS_TEXTCHANGE *pChange) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RequestSupportedAttrs( 
            /* [in] */ DWORD dwFlags,
            /* [in] */ ULONG cFilterAttrs,
            /* [unique][size_is][in] */ __RPC__in_ecount_full_opt(cFilterAttrs) const TS_ATTRID *paFilterAttrs) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RequestAttrsAtPosition( 
            /* [in] */ LONG acpPos,
            /* [in] */ ULONG cFilterAttrs,
            /* [unique][size_is][in] */ __RPC__in_ecount_full_opt(cFilterAttrs) const TS_ATTRID *paFilterAttrs,
            /* [in] */ DWORD dwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RequestAttrsTransitioningAtPosition( 
            /* [in] */ LONG acpPos,
            /* [in] */ ULONG cFilterAttrs,
            /* [unique][size_is][in] */ __RPC__in_ecount_full_opt(cFilterAttrs) const TS_ATTRID *paFilterAttrs,
            /* [in] */ DWORD dwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FindNextAttrTransition( 
            /* [in] */ LONG acpStart,
            /* [in] */ LONG acpHalt,
            /* [in] */ ULONG cFilterAttrs,
            /* [unique][size_is][in] */ __RPC__in_ecount_full_opt(cFilterAttrs) const TS_ATTRID *paFilterAttrs,
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__out LONG *pacpNext,
            /* [out] */ __RPC__out BOOL *pfFound,
            /* [out] */ __RPC__out LONG *plFoundOffset) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RetrieveRequestedAttrs( 
            /* [in] */ ULONG ulCount,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(ulCount, *pcFetched) TS_ATTRVAL *paAttrVals,
            /* [out] */ __RPC__out ULONG *pcFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetEndACP( 
            /* [out] */ __RPC__out LONG *pacp) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetActiveView( 
            /* [out] */ __RPC__out TsViewCookie *pvcView) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetACPFromPoint( 
            /* [in] */ TsViewCookie vcView,
            /* [in] */ __RPC__in const POINT *ptScreen,
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__out LONG *pacp) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTextExt( 
            /* [in] */ TsViewCookie vcView,
            /* [in] */ LONG acpStart,
            /* [in] */ LONG acpEnd,
            /* [out] */ __RPC__out RECT *prc,
            /* [out] */ __RPC__out BOOL *pfClipped) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetScreenExt( 
            /* [in] */ TsViewCookie vcView,
            /* [out] */ __RPC__out RECT *prc) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetWnd( 
            /* [in] */ TsViewCookie vcView,
            /* [out] */ __RPC__deref_out_opt HWND *phwnd) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITextStoreACPVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITextStoreACP * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITextStoreACP * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITextStoreACP * This);
        
        DECLSPEC_XFGVIRT(ITextStoreACP, AdviseSink)
        HRESULT ( STDMETHODCALLTYPE *AdviseSink )( 
            __RPC__in ITextStoreACP * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][in] */ __RPC__in_opt IUnknown *punk,
            /* [in] */ DWORD dwMask);
        
        DECLSPEC_XFGVIRT(ITextStoreACP, UnadviseSink)
        HRESULT ( STDMETHODCALLTYPE *UnadviseSink )( 
            __RPC__in ITextStoreACP * This,
            /* [in] */ __RPC__in_opt IUnknown *punk);
        
        DECLSPEC_XFGVIRT(ITextStoreACP, RequestLock)
        HRESULT ( STDMETHODCALLTYPE *RequestLock )( 
            __RPC__in ITextStoreACP * This,
            /* [in] */ DWORD dwLockFlags,
            /* [out] */ __RPC__out HRESULT *phrSession);
        
        DECLSPEC_XFGVIRT(ITextStoreACP, GetStatus)
        HRESULT ( STDMETHODCALLTYPE *GetStatus )( 
            __RPC__in ITextStoreACP * This,
            /* [out] */ __RPC__out TS_STATUS *pdcs);
        
        DECLSPEC_XFGVIRT(ITextStoreACP, QueryInsert)
        HRESULT ( STDMETHODCALLTYPE *QueryInsert )( 
            __RPC__in ITextStoreACP * This,
            /* [in] */ LONG acpTestStart,
            /* [in] */ LONG acpTestEnd,
            /* [in] */ ULONG cch,
            /* [out] */ __RPC__out LONG *pacpResultStart,
            /* [out] */ __RPC__out LONG *pacpResultEnd);
        
        DECLSPEC_XFGVIRT(ITextStoreACP, GetSelection)
        HRESULT ( STDMETHODCALLTYPE *GetSelection )( 
            __RPC__in ITextStoreACP * This,
            /* [in] */ ULONG ulIndex,
            /* [in] */ ULONG ulCount,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(ulCount, *pcFetched) TS_SELECTION_ACP *pSelection,
            /* [out] */ __RPC__out ULONG *pcFetched);
        
        DECLSPEC_XFGVIRT(ITextStoreACP, SetSelection)
        HRESULT ( STDMETHODCALLTYPE *SetSelection )( 
            __RPC__in ITextStoreACP * This,
            /* [in] */ ULONG ulCount,
            /* [size_is][in] */ __RPC__in_ecount_full(ulCount) const TS_SELECTION_ACP *pSelection);
        
        DECLSPEC_XFGVIRT(ITextStoreACP, GetText)
        HRESULT ( STDMETHODCALLTYPE *GetText )( 
            __RPC__in ITextStoreACP * This,
            /* [in] */ LONG acpStart,
            /* [in] */ LONG acpEnd,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(cchPlainReq, *pcchPlainRet) WCHAR *pchPlain,
            /* [in] */ ULONG cchPlainReq,
            /* [out] */ __RPC__out ULONG *pcchPlainRet,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(cRunInfoReq, *pcRunInfoRet) TS_RUNINFO *prgRunInfo,
            /* [in] */ ULONG cRunInfoReq,
            /* [out] */ __RPC__out ULONG *pcRunInfoRet,
            /* [out] */ __RPC__out LONG *pacpNext);
        
        DECLSPEC_XFGVIRT(ITextStoreACP, SetText)
        HRESULT ( STDMETHODCALLTYPE *SetText )( 
            __RPC__in ITextStoreACP * This,
            /* [in] */ DWORD dwFlags,
            /* [in] */ LONG acpStart,
            /* [in] */ LONG acpEnd,
            /* [size_is][in] */ __RPC__in_ecount_full(cch) const WCHAR *pchText,
            /* [in] */ ULONG cch,
            /* [out] */ __RPC__out TS_TEXTCHANGE *pChange);
        
        DECLSPEC_XFGVIRT(ITextStoreACP, GetFormattedText)
        HRESULT ( STDMETHODCALLTYPE *GetFormattedText )( 
            __RPC__in ITextStoreACP * This,
            /* [in] */ LONG acpStart,
            /* [in] */ LONG acpEnd,
            /* [out] */ __RPC__deref_out_opt IDataObject **ppDataObject);
        
        DECLSPEC_XFGVIRT(ITextStoreACP, GetEmbedded)
        HRESULT ( STDMETHODCALLTYPE *GetEmbedded )( 
            __RPC__in ITextStoreACP * This,
            /* [in] */ LONG acpPos,
            /* [in] */ __RPC__in REFGUID rguidService,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt IUnknown **ppunk);
        
        DECLSPEC_XFGVIRT(ITextStoreACP, QueryInsertEmbedded)
        HRESULT ( STDMETHODCALLTYPE *QueryInsertEmbedded )( 
            __RPC__in ITextStoreACP * This,
            /* [in] */ __RPC__in const GUID *pguidService,
            /* [in] */ __RPC__in const FORMATETC *pFormatEtc,
            /* [out] */ __RPC__out BOOL *pfInsertable);
        
        DECLSPEC_XFGVIRT(ITextStoreACP, InsertEmbedded)
        HRESULT ( STDMETHODCALLTYPE *InsertEmbedded )( 
            __RPC__in ITextStoreACP * This,
            /* [in] */ DWORD dwFlags,
            /* [in] */ LONG acpStart,
            /* [in] */ LONG acpEnd,
            /* [in] */ __RPC__in_opt IDataObject *pDataObject,
            /* [out] */ __RPC__out TS_TEXTCHANGE *pChange);
        
        DECLSPEC_XFGVIRT(ITextStoreACP, InsertTextAtSelection)
        HRESULT ( STDMETHODCALLTYPE *InsertTextAtSelection )( 
            __RPC__in ITextStoreACP * This,
            /* [in] */ DWORD dwFlags,
            /* [size_is][in] */ __RPC__in_ecount_full(cch) const WCHAR *pchText,
            /* [in] */ ULONG cch,
            /* [out] */ __RPC__out LONG *pacpStart,
            /* [out] */ __RPC__out LONG *pacpEnd,
            /* [out] */ __RPC__out TS_TEXTCHANGE *pChange);
        
        DECLSPEC_XFGVIRT(ITextStoreACP, InsertEmbeddedAtSelection)
        HRESULT ( STDMETHODCALLTYPE *InsertEmbeddedAtSelection )( 
            __RPC__in ITextStoreACP * This,
            /* [in] */ DWORD dwFlags,
            /* [in] */ __RPC__in_opt IDataObject *pDataObject,
            /* [out] */ __RPC__out LONG *pacpStart,
            /* [out] */ __RPC__out LONG *pacpEnd,
            /* [out] */ __RPC__out TS_TEXTCHANGE *pChange);
        
        DECLSPEC_XFGVIRT(ITextStoreACP, RequestSupportedAttrs)
        HRESULT ( STDMETHODCALLTYPE *RequestSupportedAttrs )( 
            __RPC__in ITextStoreACP * This,
            /* [in] */ DWORD dwFlags,
            /* [in] */ ULONG cFilterAttrs,
            /* [unique][size_is][in] */ __RPC__in_ecount_full_opt(cFilterAttrs) const TS_ATTRID *paFilterAttrs);
        
        DECLSPEC_XFGVIRT(ITextStoreACP, RequestAttrsAtPosition)
        HRESULT ( STDMETHODCALLTYPE *RequestAttrsAtPosition )( 
            __RPC__in ITextStoreACP * This,
            /* [in] */ LONG acpPos,
            /* [in] */ ULONG cFilterAttrs,
            /* [unique][size_is][in] */ __RPC__in_ecount_full_opt(cFilterAttrs) const TS_ATTRID *paFilterAttrs,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(ITextStoreACP, RequestAttrsTransitioningAtPosition)
        HRESULT ( STDMETHODCALLTYPE *RequestAttrsTransitioningAtPosition )( 
            __RPC__in ITextStoreACP * This,
            /* [in] */ LONG acpPos,
            /* [in] */ ULONG cFilterAttrs,
            /* [unique][size_is][in] */ __RPC__in_ecount_full_opt(cFilterAttrs) const TS_ATTRID *paFilterAttrs,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(ITextStoreACP, FindNextAttrTransition)
        HRESULT ( STDMETHODCALLTYPE *FindNextAttrTransition )( 
            __RPC__in ITextStoreACP * This,
            /* [in] */ LONG acpStart,
            /* [in] */ LONG acpHalt,
            /* [in] */ ULONG cFilterAttrs,
            /* [unique][size_is][in] */ __RPC__in_ecount_full_opt(cFilterAttrs) const TS_ATTRID *paFilterAttrs,
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__out LONG *pacpNext,
            /* [out] */ __RPC__out BOOL *pfFound,
            /* [out] */ __RPC__out LONG *plFoundOffset);
        
        DECLSPEC_XFGVIRT(ITextStoreACP, RetrieveRequestedAttrs)
        HRESULT ( STDMETHODCALLTYPE *RetrieveRequestedAttrs )( 
            __RPC__in ITextStoreACP * This,
            /* [in] */ ULONG ulCount,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(ulCount, *pcFetched) TS_ATTRVAL *paAttrVals,
            /* [out] */ __RPC__out ULONG *pcFetched);
        
        DECLSPEC_XFGVIRT(ITextStoreACP, GetEndACP)
        HRESULT ( STDMETHODCALLTYPE *GetEndACP )( 
            __RPC__in ITextStoreACP * This,
            /* [out] */ __RPC__out LONG *pacp);
        
        DECLSPEC_XFGVIRT(ITextStoreACP, GetActiveView)
        HRESULT ( STDMETHODCALLTYPE *GetActiveView )( 
            __RPC__in ITextStoreACP * This,
            /* [out] */ __RPC__out TsViewCookie *pvcView);
        
        DECLSPEC_XFGVIRT(ITextStoreACP, GetACPFromPoint)
        HRESULT ( STDMETHODCALLTYPE *GetACPFromPoint )( 
            __RPC__in ITextStoreACP * This,
            /* [in] */ TsViewCookie vcView,
            /* [in] */ __RPC__in const POINT *ptScreen,
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__out LONG *pacp);
        
        DECLSPEC_XFGVIRT(ITextStoreACP, GetTextExt)
        HRESULT ( STDMETHODCALLTYPE *GetTextExt )( 
            __RPC__in ITextStoreACP * This,
            /* [in] */ TsViewCookie vcView,
            /* [in] */ LONG acpStart,
            /* [in] */ LONG acpEnd,
            /* [out] */ __RPC__out RECT *prc,
            /* [out] */ __RPC__out BOOL *pfClipped);
        
        DECLSPEC_XFGVIRT(ITextStoreACP, GetScreenExt)
        HRESULT ( STDMETHODCALLTYPE *GetScreenExt )( 
            __RPC__in ITextStoreACP * This,
            /* [in] */ TsViewCookie vcView,
            /* [out] */ __RPC__out RECT *prc);
        
        DECLSPEC_XFGVIRT(ITextStoreACP, GetWnd)
        HRESULT ( STDMETHODCALLTYPE *GetWnd )( 
            __RPC__in ITextStoreACP * This,
            /* [in] */ TsViewCookie vcView,
            /* [out] */ __RPC__deref_out_opt HWND *phwnd);
        
        END_INTERFACE
    } ITextStoreACPVtbl;

    interface ITextStoreACP
    {
        CONST_VTBL struct ITextStoreACPVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITextStoreACP_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITextStoreACP_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITextStoreACP_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITextStoreACP_AdviseSink(This,riid,punk,dwMask)	\
    ( (This)->lpVtbl -> AdviseSink(This,riid,punk,dwMask) ) 

#define ITextStoreACP_UnadviseSink(This,punk)	\
    ( (This)->lpVtbl -> UnadviseSink(This,punk) ) 

#define ITextStoreACP_RequestLock(This,dwLockFlags,phrSession)	\
    ( (This)->lpVtbl -> RequestLock(This,dwLockFlags,phrSession) ) 

#define ITextStoreACP_GetStatus(This,pdcs)	\
    ( (This)->lpVtbl -> GetStatus(This,pdcs) ) 

#define ITextStoreACP_QueryInsert(This,acpTestStart,acpTestEnd,cch,pacpResultStart,pacpResultEnd)	\
    ( (This)->lpVtbl -> QueryInsert(This,acpTestStart,acpTestEnd,cch,pacpResultStart,pacpResultEnd) ) 

#define ITextStoreACP_GetSelection(This,ulIndex,ulCount,pSelection,pcFetched)	\
    ( (This)->lpVtbl -> GetSelection(This,ulIndex,ulCount,pSelection,pcFetched) ) 

#define ITextStoreACP_SetSelection(This,ulCount,pSelection)	\
    ( (This)->lpVtbl -> SetSelection(This,ulCount,pSelection) ) 

#define ITextStoreACP_GetText(This,acpStart,acpEnd,pchPlain,cchPlainReq,pcchPlainRet,prgRunInfo,cRunInfoReq,pcRunInfoRet,pacpNext)	\
    ( (This)->lpVtbl -> GetText(This,acpStart,acpEnd,pchPlain,cchPlainReq,pcchPlainRet,prgRunInfo,cRunInfoReq,pcRunInfoRet,pacpNext) ) 

#define ITextStoreACP_SetText(This,dwFlags,acpStart,acpEnd,pchText,cch,pChange)	\
    ( (This)->lpVtbl -> SetText(This,dwFlags,acpStart,acpEnd,pchText,cch,pChange) ) 

#define ITextStoreACP_GetFormattedText(This,acpStart,acpEnd,ppDataObject)	\
    ( (This)->lpVtbl -> GetFormattedText(This,acpStart,acpEnd,ppDataObject) ) 

#define ITextStoreACP_GetEmbedded(This,acpPos,rguidService,riid,ppunk)	\
    ( (This)->lpVtbl -> GetEmbedded(This,acpPos,rguidService,riid,ppunk) ) 

#define ITextStoreACP_QueryInsertEmbedded(This,pguidService,pFormatEtc,pfInsertable)	\
    ( (This)->lpVtbl -> QueryInsertEmbedded(This,pguidService,pFormatEtc,pfInsertable) ) 

#define ITextStoreACP_InsertEmbedded(This,dwFlags,acpStart,acpEnd,pDataObject,pChange)	\
    ( (This)->lpVtbl -> InsertEmbedded(This,dwFlags,acpStart,acpEnd,pDataObject,pChange) ) 

#define ITextStoreACP_InsertTextAtSelection(This,dwFlags,pchText,cch,pacpStart,pacpEnd,pChange)	\
    ( (This)->lpVtbl -> InsertTextAtSelection(This,dwFlags,pchText,cch,pacpStart,pacpEnd,pChange) ) 

#define ITextStoreACP_InsertEmbeddedAtSelection(This,dwFlags,pDataObject,pacpStart,pacpEnd,pChange)	\
    ( (This)->lpVtbl -> InsertEmbeddedAtSelection(This,dwFlags,pDataObject,pacpStart,pacpEnd,pChange) ) 

#define ITextStoreACP_RequestSupportedAttrs(This,dwFlags,cFilterAttrs,paFilterAttrs)	\
    ( (This)->lpVtbl -> RequestSupportedAttrs(This,dwFlags,cFilterAttrs,paFilterAttrs) ) 

#define ITextStoreACP_RequestAttrsAtPosition(This,acpPos,cFilterAttrs,paFilterAttrs,dwFlags)	\
    ( (This)->lpVtbl -> RequestAttrsAtPosition(This,acpPos,cFilterAttrs,paFilterAttrs,dwFlags) ) 

#define ITextStoreACP_RequestAttrsTransitioningAtPosition(This,acpPos,cFilterAttrs,paFilterAttrs,dwFlags)	\
    ( (This)->lpVtbl -> RequestAttrsTransitioningAtPosition(This,acpPos,cFilterAttrs,paFilterAttrs,dwFlags) ) 

#define ITextStoreACP_FindNextAttrTransition(This,acpStart,acpHalt,cFilterAttrs,paFilterAttrs,dwFlags,pacpNext,pfFound,plFoundOffset)	\
    ( (This)->lpVtbl -> FindNextAttrTransition(This,acpStart,acpHalt,cFilterAttrs,paFilterAttrs,dwFlags,pacpNext,pfFound,plFoundOffset) ) 

#define ITextStoreACP_RetrieveRequestedAttrs(This,ulCount,paAttrVals,pcFetched)	\
    ( (This)->lpVtbl -> RetrieveRequestedAttrs(This,ulCount,paAttrVals,pcFetched) ) 

#define ITextStoreACP_GetEndACP(This,pacp)	\
    ( (This)->lpVtbl -> GetEndACP(This,pacp) ) 

#define ITextStoreACP_GetActiveView(This,pvcView)	\
    ( (This)->lpVtbl -> GetActiveView(This,pvcView) ) 

#define ITextStoreACP_GetACPFromPoint(This,vcView,ptScreen,dwFlags,pacp)	\
    ( (This)->lpVtbl -> GetACPFromPoint(This,vcView,ptScreen,dwFlags,pacp) ) 

#define ITextStoreACP_GetTextExt(This,vcView,acpStart,acpEnd,prc,pfClipped)	\
    ( (This)->lpVtbl -> GetTextExt(This,vcView,acpStart,acpEnd,prc,pfClipped) ) 

#define ITextStoreACP_GetScreenExt(This,vcView,prc)	\
    ( (This)->lpVtbl -> GetScreenExt(This,vcView,prc) ) 

#define ITextStoreACP_GetWnd(This,vcView,phwnd)	\
    ( (This)->lpVtbl -> GetWnd(This,vcView,phwnd) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITextStoreACP_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_textstor_0000_0001 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)


extern RPC_IF_HANDLE __MIDL_itf_textstor_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_textstor_0000_0001_v0_0_s_ifspec;

#ifndef __ITextStoreACP2_INTERFACE_DEFINED__
#define __ITextStoreACP2_INTERFACE_DEFINED__

/* interface ITextStoreACP2 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ITextStoreACP2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f86ad89f-5fe4-4b8d-bb9f-ef3797a84f1f")
    ITextStoreACP2 : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AdviseSink( 
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][in] */ __RPC__in_opt IUnknown *punk,
            /* [in] */ DWORD dwMask) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnadviseSink( 
            /* [in] */ __RPC__in_opt IUnknown *punk) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RequestLock( 
            /* [in] */ DWORD dwLockFlags,
            /* [out] */ __RPC__out HRESULT *phrSession) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStatus( 
            /* [out] */ __RPC__out TS_STATUS *pdcs) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE QueryInsert( 
            /* [in] */ LONG acpTestStart,
            /* [in] */ LONG acpTestEnd,
            /* [in] */ ULONG cch,
            /* [out] */ __RPC__out LONG *pacpResultStart,
            /* [out] */ __RPC__out LONG *pacpResultEnd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSelection( 
            /* [in] */ ULONG ulIndex,
            /* [in] */ ULONG ulCount,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(ulCount, *pcFetched) TS_SELECTION_ACP *pSelection,
            /* [out] */ __RPC__out ULONG *pcFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetSelection( 
            /* [in] */ ULONG ulCount,
            /* [size_is][in] */ __RPC__in_ecount_full(ulCount) const TS_SELECTION_ACP *pSelection) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetText( 
            /* [in] */ LONG acpStart,
            /* [in] */ LONG acpEnd,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(cchPlainReq, *pcchPlainRet) WCHAR *pchPlain,
            /* [in] */ ULONG cchPlainReq,
            /* [out] */ __RPC__out ULONG *pcchPlainRet,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(cRunInfoReq, *pcRunInfoRet) TS_RUNINFO *prgRunInfo,
            /* [in] */ ULONG cRunInfoReq,
            /* [out] */ __RPC__out ULONG *pcRunInfoRet,
            /* [out] */ __RPC__out LONG *pacpNext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetText( 
            /* [in] */ DWORD dwFlags,
            /* [in] */ LONG acpStart,
            /* [in] */ LONG acpEnd,
            /* [size_is][in] */ __RPC__in_ecount_full(cch) const WCHAR *pchText,
            /* [in] */ ULONG cch,
            /* [out] */ __RPC__out TS_TEXTCHANGE *pChange) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFormattedText( 
            /* [in] */ LONG acpStart,
            /* [in] */ LONG acpEnd,
            /* [out] */ __RPC__deref_out_opt IDataObject **ppDataObject) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetEmbedded( 
            /* [in] */ LONG acpPos,
            /* [in] */ __RPC__in REFGUID rguidService,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt IUnknown **ppunk) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE QueryInsertEmbedded( 
            /* [in] */ __RPC__in const GUID *pguidService,
            /* [in] */ __RPC__in const FORMATETC *pFormatEtc,
            /* [out] */ __RPC__out BOOL *pfInsertable) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InsertEmbedded( 
            /* [in] */ DWORD dwFlags,
            /* [in] */ LONG acpStart,
            /* [in] */ LONG acpEnd,
            /* [in] */ __RPC__in_opt IDataObject *pDataObject,
            /* [out] */ __RPC__out TS_TEXTCHANGE *pChange) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InsertTextAtSelection( 
            /* [in] */ DWORD dwFlags,
            /* [size_is][in] */ __RPC__in_ecount_full(cch) const WCHAR *pchText,
            /* [in] */ ULONG cch,
            /* [out] */ __RPC__out LONG *pacpStart,
            /* [out] */ __RPC__out LONG *pacpEnd,
            /* [out] */ __RPC__out TS_TEXTCHANGE *pChange) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InsertEmbeddedAtSelection( 
            /* [in] */ DWORD dwFlags,
            /* [in] */ __RPC__in_opt IDataObject *pDataObject,
            /* [out] */ __RPC__out LONG *pacpStart,
            /* [out] */ __RPC__out LONG *pacpEnd,
            /* [out] */ __RPC__out TS_TEXTCHANGE *pChange) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RequestSupportedAttrs( 
            /* [in] */ DWORD dwFlags,
            /* [in] */ ULONG cFilterAttrs,
            /* [unique][size_is][in] */ __RPC__in_ecount_full_opt(cFilterAttrs) const TS_ATTRID *paFilterAttrs) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RequestAttrsAtPosition( 
            /* [in] */ LONG acpPos,
            /* [in] */ ULONG cFilterAttrs,
            /* [unique][size_is][in] */ __RPC__in_ecount_full_opt(cFilterAttrs) const TS_ATTRID *paFilterAttrs,
            /* [in] */ DWORD dwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RequestAttrsTransitioningAtPosition( 
            /* [in] */ LONG acpPos,
            /* [in] */ ULONG cFilterAttrs,
            /* [unique][size_is][in] */ __RPC__in_ecount_full_opt(cFilterAttrs) const TS_ATTRID *paFilterAttrs,
            /* [in] */ DWORD dwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FindNextAttrTransition( 
            /* [in] */ LONG acpStart,
            /* [in] */ LONG acpHalt,
            /* [in] */ ULONG cFilterAttrs,
            /* [unique][size_is][in] */ __RPC__in_ecount_full_opt(cFilterAttrs) const TS_ATTRID *paFilterAttrs,
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__out LONG *pacpNext,
            /* [out] */ __RPC__out BOOL *pfFound,
            /* [out] */ __RPC__out LONG *plFoundOffset) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RetrieveRequestedAttrs( 
            /* [in] */ ULONG ulCount,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(ulCount, *pcFetched) TS_ATTRVAL *paAttrVals,
            /* [out] */ __RPC__out ULONG *pcFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetEndACP( 
            /* [out] */ __RPC__out LONG *pacp) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetActiveView( 
            /* [out] */ __RPC__out TsViewCookie *pvcView) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetACPFromPoint( 
            /* [in] */ TsViewCookie vcView,
            /* [in] */ __RPC__in const POINT *ptScreen,
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__out LONG *pacp) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTextExt( 
            /* [in] */ TsViewCookie vcView,
            /* [in] */ LONG acpStart,
            /* [in] */ LONG acpEnd,
            /* [out] */ __RPC__out RECT *prc,
            /* [out] */ __RPC__out BOOL *pfClipped) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetScreenExt( 
            /* [in] */ TsViewCookie vcView,
            /* [out] */ __RPC__out RECT *prc) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITextStoreACP2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITextStoreACP2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITextStoreACP2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITextStoreACP2 * This);
        
        DECLSPEC_XFGVIRT(ITextStoreACP2, AdviseSink)
        HRESULT ( STDMETHODCALLTYPE *AdviseSink )( 
            __RPC__in ITextStoreACP2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][in] */ __RPC__in_opt IUnknown *punk,
            /* [in] */ DWORD dwMask);
        
        DECLSPEC_XFGVIRT(ITextStoreACP2, UnadviseSink)
        HRESULT ( STDMETHODCALLTYPE *UnadviseSink )( 
            __RPC__in ITextStoreACP2 * This,
            /* [in] */ __RPC__in_opt IUnknown *punk);
        
        DECLSPEC_XFGVIRT(ITextStoreACP2, RequestLock)
        HRESULT ( STDMETHODCALLTYPE *RequestLock )( 
            __RPC__in ITextStoreACP2 * This,
            /* [in] */ DWORD dwLockFlags,
            /* [out] */ __RPC__out HRESULT *phrSession);
        
        DECLSPEC_XFGVIRT(ITextStoreACP2, GetStatus)
        HRESULT ( STDMETHODCALLTYPE *GetStatus )( 
            __RPC__in ITextStoreACP2 * This,
            /* [out] */ __RPC__out TS_STATUS *pdcs);
        
        DECLSPEC_XFGVIRT(ITextStoreACP2, QueryInsert)
        HRESULT ( STDMETHODCALLTYPE *QueryInsert )( 
            __RPC__in ITextStoreACP2 * This,
            /* [in] */ LONG acpTestStart,
            /* [in] */ LONG acpTestEnd,
            /* [in] */ ULONG cch,
            /* [out] */ __RPC__out LONG *pacpResultStart,
            /* [out] */ __RPC__out LONG *pacpResultEnd);
        
        DECLSPEC_XFGVIRT(ITextStoreACP2, GetSelection)
        HRESULT ( STDMETHODCALLTYPE *GetSelection )( 
            __RPC__in ITextStoreACP2 * This,
            /* [in] */ ULONG ulIndex,
            /* [in] */ ULONG ulCount,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(ulCount, *pcFetched) TS_SELECTION_ACP *pSelection,
            /* [out] */ __RPC__out ULONG *pcFetched);
        
        DECLSPEC_XFGVIRT(ITextStoreACP2, SetSelection)
        HRESULT ( STDMETHODCALLTYPE *SetSelection )( 
            __RPC__in ITextStoreACP2 * This,
            /* [in] */ ULONG ulCount,
            /* [size_is][in] */ __RPC__in_ecount_full(ulCount) const TS_SELECTION_ACP *pSelection);
        
        DECLSPEC_XFGVIRT(ITextStoreACP2, GetText)
        HRESULT ( STDMETHODCALLTYPE *GetText )( 
            __RPC__in ITextStoreACP2 * This,
            /* [in] */ LONG acpStart,
            /* [in] */ LONG acpEnd,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(cchPlainReq, *pcchPlainRet) WCHAR *pchPlain,
            /* [in] */ ULONG cchPlainReq,
            /* [out] */ __RPC__out ULONG *pcchPlainRet,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(cRunInfoReq, *pcRunInfoRet) TS_RUNINFO *prgRunInfo,
            /* [in] */ ULONG cRunInfoReq,
            /* [out] */ __RPC__out ULONG *pcRunInfoRet,
            /* [out] */ __RPC__out LONG *pacpNext);
        
        DECLSPEC_XFGVIRT(ITextStoreACP2, SetText)
        HRESULT ( STDMETHODCALLTYPE *SetText )( 
            __RPC__in ITextStoreACP2 * This,
            /* [in] */ DWORD dwFlags,
            /* [in] */ LONG acpStart,
            /* [in] */ LONG acpEnd,
            /* [size_is][in] */ __RPC__in_ecount_full(cch) const WCHAR *pchText,
            /* [in] */ ULONG cch,
            /* [out] */ __RPC__out TS_TEXTCHANGE *pChange);
        
        DECLSPEC_XFGVIRT(ITextStoreACP2, GetFormattedText)
        HRESULT ( STDMETHODCALLTYPE *GetFormattedText )( 
            __RPC__in ITextStoreACP2 * This,
            /* [in] */ LONG acpStart,
            /* [in] */ LONG acpEnd,
            /* [out] */ __RPC__deref_out_opt IDataObject **ppDataObject);
        
        DECLSPEC_XFGVIRT(ITextStoreACP2, GetEmbedded)
        HRESULT ( STDMETHODCALLTYPE *GetEmbedded )( 
            __RPC__in ITextStoreACP2 * This,
            /* [in] */ LONG acpPos,
            /* [in] */ __RPC__in REFGUID rguidService,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt IUnknown **ppunk);
        
        DECLSPEC_XFGVIRT(ITextStoreACP2, QueryInsertEmbedded)
        HRESULT ( STDMETHODCALLTYPE *QueryInsertEmbedded )( 
            __RPC__in ITextStoreACP2 * This,
            /* [in] */ __RPC__in const GUID *pguidService,
            /* [in] */ __RPC__in const FORMATETC *pFormatEtc,
            /* [out] */ __RPC__out BOOL *pfInsertable);
        
        DECLSPEC_XFGVIRT(ITextStoreACP2, InsertEmbedded)
        HRESULT ( STDMETHODCALLTYPE *InsertEmbedded )( 
            __RPC__in ITextStoreACP2 * This,
            /* [in] */ DWORD dwFlags,
            /* [in] */ LONG acpStart,
            /* [in] */ LONG acpEnd,
            /* [in] */ __RPC__in_opt IDataObject *pDataObject,
            /* [out] */ __RPC__out TS_TEXTCHANGE *pChange);
        
        DECLSPEC_XFGVIRT(ITextStoreACP2, InsertTextAtSelection)
        HRESULT ( STDMETHODCALLTYPE *InsertTextAtSelection )( 
            __RPC__in ITextStoreACP2 * This,
            /* [in] */ DWORD dwFlags,
            /* [size_is][in] */ __RPC__in_ecount_full(cch) const WCHAR *pchText,
            /* [in] */ ULONG cch,
            /* [out] */ __RPC__out LONG *pacpStart,
            /* [out] */ __RPC__out LONG *pacpEnd,
            /* [out] */ __RPC__out TS_TEXTCHANGE *pChange);
        
        DECLSPEC_XFGVIRT(ITextStoreACP2, InsertEmbeddedAtSelection)
        HRESULT ( STDMETHODCALLTYPE *InsertEmbeddedAtSelection )( 
            __RPC__in ITextStoreACP2 * This,
            /* [in] */ DWORD dwFlags,
            /* [in] */ __RPC__in_opt IDataObject *pDataObject,
            /* [out] */ __RPC__out LONG *pacpStart,
            /* [out] */ __RPC__out LONG *pacpEnd,
            /* [out] */ __RPC__out TS_TEXTCHANGE *pChange);
        
        DECLSPEC_XFGVIRT(ITextStoreACP2, RequestSupportedAttrs)
        HRESULT ( STDMETHODCALLTYPE *RequestSupportedAttrs )( 
            __RPC__in ITextStoreACP2 * This,
            /* [in] */ DWORD dwFlags,
            /* [in] */ ULONG cFilterAttrs,
            /* [unique][size_is][in] */ __RPC__in_ecount_full_opt(cFilterAttrs) const TS_ATTRID *paFilterAttrs);
        
        DECLSPEC_XFGVIRT(ITextStoreACP2, RequestAttrsAtPosition)
        HRESULT ( STDMETHODCALLTYPE *RequestAttrsAtPosition )( 
            __RPC__in ITextStoreACP2 * This,
            /* [in] */ LONG acpPos,
            /* [in] */ ULONG cFilterAttrs,
            /* [unique][size_is][in] */ __RPC__in_ecount_full_opt(cFilterAttrs) const TS_ATTRID *paFilterAttrs,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(ITextStoreACP2, RequestAttrsTransitioningAtPosition)
        HRESULT ( STDMETHODCALLTYPE *RequestAttrsTransitioningAtPosition )( 
            __RPC__in ITextStoreACP2 * This,
            /* [in] */ LONG acpPos,
            /* [in] */ ULONG cFilterAttrs,
            /* [unique][size_is][in] */ __RPC__in_ecount_full_opt(cFilterAttrs) const TS_ATTRID *paFilterAttrs,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(ITextStoreACP2, FindNextAttrTransition)
        HRESULT ( STDMETHODCALLTYPE *FindNextAttrTransition )( 
            __RPC__in ITextStoreACP2 * This,
            /* [in] */ LONG acpStart,
            /* [in] */ LONG acpHalt,
            /* [in] */ ULONG cFilterAttrs,
            /* [unique][size_is][in] */ __RPC__in_ecount_full_opt(cFilterAttrs) const TS_ATTRID *paFilterAttrs,
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__out LONG *pacpNext,
            /* [out] */ __RPC__out BOOL *pfFound,
            /* [out] */ __RPC__out LONG *plFoundOffset);
        
        DECLSPEC_XFGVIRT(ITextStoreACP2, RetrieveRequestedAttrs)
        HRESULT ( STDMETHODCALLTYPE *RetrieveRequestedAttrs )( 
            __RPC__in ITextStoreACP2 * This,
            /* [in] */ ULONG ulCount,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(ulCount, *pcFetched) TS_ATTRVAL *paAttrVals,
            /* [out] */ __RPC__out ULONG *pcFetched);
        
        DECLSPEC_XFGVIRT(ITextStoreACP2, GetEndACP)
        HRESULT ( STDMETHODCALLTYPE *GetEndACP )( 
            __RPC__in ITextStoreACP2 * This,
            /* [out] */ __RPC__out LONG *pacp);
        
        DECLSPEC_XFGVIRT(ITextStoreACP2, GetActiveView)
        HRESULT ( STDMETHODCALLTYPE *GetActiveView )( 
            __RPC__in ITextStoreACP2 * This,
            /* [out] */ __RPC__out TsViewCookie *pvcView);
        
        DECLSPEC_XFGVIRT(ITextStoreACP2, GetACPFromPoint)
        HRESULT ( STDMETHODCALLTYPE *GetACPFromPoint )( 
            __RPC__in ITextStoreACP2 * This,
            /* [in] */ TsViewCookie vcView,
            /* [in] */ __RPC__in const POINT *ptScreen,
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__out LONG *pacp);
        
        DECLSPEC_XFGVIRT(ITextStoreACP2, GetTextExt)
        HRESULT ( STDMETHODCALLTYPE *GetTextExt )( 
            __RPC__in ITextStoreACP2 * This,
            /* [in] */ TsViewCookie vcView,
            /* [in] */ LONG acpStart,
            /* [in] */ LONG acpEnd,
            /* [out] */ __RPC__out RECT *prc,
            /* [out] */ __RPC__out BOOL *pfClipped);
        
        DECLSPEC_XFGVIRT(ITextStoreACP2, GetScreenExt)
        HRESULT ( STDMETHODCALLTYPE *GetScreenExt )( 
            __RPC__in ITextStoreACP2 * This,
            /* [in] */ TsViewCookie vcView,
            /* [out] */ __RPC__out RECT *prc);
        
        END_INTERFACE
    } ITextStoreACP2Vtbl;

    interface ITextStoreACP2
    {
        CONST_VTBL struct ITextStoreACP2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITextStoreACP2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITextStoreACP2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITextStoreACP2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITextStoreACP2_AdviseSink(This,riid,punk,dwMask)	\
    ( (This)->lpVtbl -> AdviseSink(This,riid,punk,dwMask) ) 

#define ITextStoreACP2_UnadviseSink(This,punk)	\
    ( (This)->lpVtbl -> UnadviseSink(This,punk) ) 

#define ITextStoreACP2_RequestLock(This,dwLockFlags,phrSession)	\
    ( (This)->lpVtbl -> RequestLock(This,dwLockFlags,phrSession) ) 

#define ITextStoreACP2_GetStatus(This,pdcs)	\
    ( (This)->lpVtbl -> GetStatus(This,pdcs) ) 

#define ITextStoreACP2_QueryInsert(This,acpTestStart,acpTestEnd,cch,pacpResultStart,pacpResultEnd)	\
    ( (This)->lpVtbl -> QueryInsert(This,acpTestStart,acpTestEnd,cch,pacpResultStart,pacpResultEnd) ) 

#define ITextStoreACP2_GetSelection(This,ulIndex,ulCount,pSelection,pcFetched)	\
    ( (This)->lpVtbl -> GetSelection(This,ulIndex,ulCount,pSelection,pcFetched) ) 

#define ITextStoreACP2_SetSelection(This,ulCount,pSelection)	\
    ( (This)->lpVtbl -> SetSelection(This,ulCount,pSelection) ) 

#define ITextStoreACP2_GetText(This,acpStart,acpEnd,pchPlain,cchPlainReq,pcchPlainRet,prgRunInfo,cRunInfoReq,pcRunInfoRet,pacpNext)	\
    ( (This)->lpVtbl -> GetText(This,acpStart,acpEnd,pchPlain,cchPlainReq,pcchPlainRet,prgRunInfo,cRunInfoReq,pcRunInfoRet,pacpNext) ) 

#define ITextStoreACP2_SetText(This,dwFlags,acpStart,acpEnd,pchText,cch,pChange)	\
    ( (This)->lpVtbl -> SetText(This,dwFlags,acpStart,acpEnd,pchText,cch,pChange) ) 

#define ITextStoreACP2_GetFormattedText(This,acpStart,acpEnd,ppDataObject)	\
    ( (This)->lpVtbl -> GetFormattedText(This,acpStart,acpEnd,ppDataObject) ) 

#define ITextStoreACP2_GetEmbedded(This,acpPos,rguidService,riid,ppunk)	\
    ( (This)->lpVtbl -> GetEmbedded(This,acpPos,rguidService,riid,ppunk) ) 

#define ITextStoreACP2_QueryInsertEmbedded(This,pguidService,pFormatEtc,pfInsertable)	\
    ( (This)->lpVtbl -> QueryInsertEmbedded(This,pguidService,pFormatEtc,pfInsertable) ) 

#define ITextStoreACP2_InsertEmbedded(This,dwFlags,acpStart,acpEnd,pDataObject,pChange)	\
    ( (This)->lpVtbl -> InsertEmbedded(This,dwFlags,acpStart,acpEnd,pDataObject,pChange) ) 

#define ITextStoreACP2_InsertTextAtSelection(This,dwFlags,pchText,cch,pacpStart,pacpEnd,pChange)	\
    ( (This)->lpVtbl -> InsertTextAtSelection(This,dwFlags,pchText,cch,pacpStart,pacpEnd,pChange) ) 

#define ITextStoreACP2_InsertEmbeddedAtSelection(This,dwFlags,pDataObject,pacpStart,pacpEnd,pChange)	\
    ( (This)->lpVtbl -> InsertEmbeddedAtSelection(This,dwFlags,pDataObject,pacpStart,pacpEnd,pChange) ) 

#define ITextStoreACP2_RequestSupportedAttrs(This,dwFlags,cFilterAttrs,paFilterAttrs)	\
    ( (This)->lpVtbl -> RequestSupportedAttrs(This,dwFlags,cFilterAttrs,paFilterAttrs) ) 

#define ITextStoreACP2_RequestAttrsAtPosition(This,acpPos,cFilterAttrs,paFilterAttrs,dwFlags)	\
    ( (This)->lpVtbl -> RequestAttrsAtPosition(This,acpPos,cFilterAttrs,paFilterAttrs,dwFlags) ) 

#define ITextStoreACP2_RequestAttrsTransitioningAtPosition(This,acpPos,cFilterAttrs,paFilterAttrs,dwFlags)	\
    ( (This)->lpVtbl -> RequestAttrsTransitioningAtPosition(This,acpPos,cFilterAttrs,paFilterAttrs,dwFlags) ) 

#define ITextStoreACP2_FindNextAttrTransition(This,acpStart,acpHalt,cFilterAttrs,paFilterAttrs,dwFlags,pacpNext,pfFound,plFoundOffset)	\
    ( (This)->lpVtbl -> FindNextAttrTransition(This,acpStart,acpHalt,cFilterAttrs,paFilterAttrs,dwFlags,pacpNext,pfFound,plFoundOffset) ) 

#define ITextStoreACP2_RetrieveRequestedAttrs(This,ulCount,paAttrVals,pcFetched)	\
    ( (This)->lpVtbl -> RetrieveRequestedAttrs(This,ulCount,paAttrVals,pcFetched) ) 

#define ITextStoreACP2_GetEndACP(This,pacp)	\
    ( (This)->lpVtbl -> GetEndACP(This,pacp) ) 

#define ITextStoreACP2_GetActiveView(This,pvcView)	\
    ( (This)->lpVtbl -> GetActiveView(This,pvcView) ) 

#define ITextStoreACP2_GetACPFromPoint(This,vcView,ptScreen,dwFlags,pacp)	\
    ( (This)->lpVtbl -> GetACPFromPoint(This,vcView,ptScreen,dwFlags,pacp) ) 

#define ITextStoreACP2_GetTextExt(This,vcView,acpStart,acpEnd,prc,pfClipped)	\
    ( (This)->lpVtbl -> GetTextExt(This,vcView,acpStart,acpEnd,prc,pfClipped) ) 

#define ITextStoreACP2_GetScreenExt(This,vcView,prc)	\
    ( (This)->lpVtbl -> GetScreenExt(This,vcView,prc) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITextStoreACP2_INTERFACE_DEFINED__ */


#ifndef __ITextStoreACPSink_INTERFACE_DEFINED__
#define __ITextStoreACPSink_INTERFACE_DEFINED__

/* interface ITextStoreACPSink */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ITextStoreACPSink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("22d44c94-a419-4542-a272-ae26093ececf")
    ITextStoreACPSink : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnTextChange( 
            /* [in] */ DWORD dwFlags,
            /* [in] */ __RPC__in const TS_TEXTCHANGE *pChange) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnSelectionChange( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnLayoutChange( 
            /* [in] */ TsLayoutCode lcode,
            /* [in] */ TsViewCookie vcView) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnStatusChange( 
            /* [in] */ DWORD dwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnAttrsChange( 
            /* [in] */ LONG acpStart,
            /* [in] */ LONG acpEnd,
            /* [in] */ ULONG cAttrs,
            /* [size_is][in] */ __RPC__in_ecount_full(cAttrs) const TS_ATTRID *paAttrs) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnLockGranted( 
            /* [in] */ DWORD dwLockFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnStartEditTransaction( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnEndEditTransaction( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITextStoreACPSinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITextStoreACPSink * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITextStoreACPSink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITextStoreACPSink * This);
        
        DECLSPEC_XFGVIRT(ITextStoreACPSink, OnTextChange)
        HRESULT ( STDMETHODCALLTYPE *OnTextChange )( 
            __RPC__in ITextStoreACPSink * This,
            /* [in] */ DWORD dwFlags,
            /* [in] */ __RPC__in const TS_TEXTCHANGE *pChange);
        
        DECLSPEC_XFGVIRT(ITextStoreACPSink, OnSelectionChange)
        HRESULT ( STDMETHODCALLTYPE *OnSelectionChange )( 
            __RPC__in ITextStoreACPSink * This);
        
        DECLSPEC_XFGVIRT(ITextStoreACPSink, OnLayoutChange)
        HRESULT ( STDMETHODCALLTYPE *OnLayoutChange )( 
            __RPC__in ITextStoreACPSink * This,
            /* [in] */ TsLayoutCode lcode,
            /* [in] */ TsViewCookie vcView);
        
        DECLSPEC_XFGVIRT(ITextStoreACPSink, OnStatusChange)
        HRESULT ( STDMETHODCALLTYPE *OnStatusChange )( 
            __RPC__in ITextStoreACPSink * This,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(ITextStoreACPSink, OnAttrsChange)
        HRESULT ( STDMETHODCALLTYPE *OnAttrsChange )( 
            __RPC__in ITextStoreACPSink * This,
            /* [in] */ LONG acpStart,
            /* [in] */ LONG acpEnd,
            /* [in] */ ULONG cAttrs,
            /* [size_is][in] */ __RPC__in_ecount_full(cAttrs) const TS_ATTRID *paAttrs);
        
        DECLSPEC_XFGVIRT(ITextStoreACPSink, OnLockGranted)
        HRESULT ( STDMETHODCALLTYPE *OnLockGranted )( 
            __RPC__in ITextStoreACPSink * This,
            /* [in] */ DWORD dwLockFlags);
        
        DECLSPEC_XFGVIRT(ITextStoreACPSink, OnStartEditTransaction)
        HRESULT ( STDMETHODCALLTYPE *OnStartEditTransaction )( 
            __RPC__in ITextStoreACPSink * This);
        
        DECLSPEC_XFGVIRT(ITextStoreACPSink, OnEndEditTransaction)
        HRESULT ( STDMETHODCALLTYPE *OnEndEditTransaction )( 
            __RPC__in ITextStoreACPSink * This);
        
        END_INTERFACE
    } ITextStoreACPSinkVtbl;

    interface ITextStoreACPSink
    {
        CONST_VTBL struct ITextStoreACPSinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITextStoreACPSink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITextStoreACPSink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITextStoreACPSink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITextStoreACPSink_OnTextChange(This,dwFlags,pChange)	\
    ( (This)->lpVtbl -> OnTextChange(This,dwFlags,pChange) ) 

#define ITextStoreACPSink_OnSelectionChange(This)	\
    ( (This)->lpVtbl -> OnSelectionChange(This) ) 

#define ITextStoreACPSink_OnLayoutChange(This,lcode,vcView)	\
    ( (This)->lpVtbl -> OnLayoutChange(This,lcode,vcView) ) 

#define ITextStoreACPSink_OnStatusChange(This,dwFlags)	\
    ( (This)->lpVtbl -> OnStatusChange(This,dwFlags) ) 

#define ITextStoreACPSink_OnAttrsChange(This,acpStart,acpEnd,cAttrs,paAttrs)	\
    ( (This)->lpVtbl -> OnAttrsChange(This,acpStart,acpEnd,cAttrs,paAttrs) ) 

#define ITextStoreACPSink_OnLockGranted(This,dwLockFlags)	\
    ( (This)->lpVtbl -> OnLockGranted(This,dwLockFlags) ) 

#define ITextStoreACPSink_OnStartEditTransaction(This)	\
    ( (This)->lpVtbl -> OnStartEditTransaction(This) ) 

#define ITextStoreACPSink_OnEndEditTransaction(This)	\
    ( (This)->lpVtbl -> OnEndEditTransaction(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITextStoreACPSink_INTERFACE_DEFINED__ */


#ifndef __IAnchor_INTERFACE_DEFINED__
#define __IAnchor_INTERFACE_DEFINED__

/* interface IAnchor */
/* [unique][uuid][object] */ 

typedef /* [public][public][public][uuid] */  DECLSPEC_UUID("daa8601e-7695-426f-9bb7-498a6aa64b68") 
enum __MIDL_IAnchor_0001
    {
        TS_GR_BACKWARD	= 0,
        TS_GR_FORWARD	= 1
    } 	TsGravity;

typedef /* [public][public][uuid] */  DECLSPEC_UUID("898e19df-4fb4-4af3-8daf-9b3c1145c79d") 
enum __MIDL_IAnchor_0002
    {
        TS_SD_BACKWARD	= 0,
        TS_SD_FORWARD	= 1
    } 	TsShiftDir;

#define	TS_CH_PRECEDING_DEL	( 1 )

#define	TS_CH_FOLLOWING_DEL	( 2 )

#define	TS_SHIFT_COUNT_HIDDEN	( 0x1 )

#define	TS_SHIFT_HALT_HIDDEN	( 0x2 )

#define	TS_SHIFT_HALT_VISIBLE	( 0x4 )

#define	TS_SHIFT_COUNT_ONLY	( 0x8 )


EXTERN_C const IID IID_IAnchor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0feb7e34-5a60-4356-8ef7-abdec2ff7cf8")
    IAnchor : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetGravity( 
            /* [in] */ TsGravity gravity) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetGravity( 
            /* [out] */ __RPC__out TsGravity *pgravity) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsEqual( 
            /* [in] */ __RPC__in_opt IAnchor *paWith,
            /* [out] */ __RPC__out BOOL *pfEqual) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Compare( 
            /* [in] */ __RPC__in_opt IAnchor *paWith,
            /* [out] */ __RPC__out LONG *plResult) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Shift( 
            /* [in] */ DWORD dwFlags,
            /* [in] */ LONG cchReq,
            /* [out] */ __RPC__out LONG *pcch,
            /* [in] */ __RPC__in_opt IAnchor *paHaltAnchor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ShiftTo( 
            /* [in] */ __RPC__in_opt IAnchor *paSite) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ShiftRegion( 
            /* [in] */ DWORD dwFlags,
            /* [in] */ TsShiftDir dir,
            /* [out] */ __RPC__out BOOL *pfNoRegion) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetChangeHistoryMask( 
            /* [in] */ DWORD dwMask) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetChangeHistory( 
            /* [out] */ __RPC__out DWORD *pdwHistory) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ClearChangeHistory( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ __RPC__deref_out_opt IAnchor **ppaClone) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAnchorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAnchor * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAnchor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAnchor * This);
        
        DECLSPEC_XFGVIRT(IAnchor, SetGravity)
        HRESULT ( STDMETHODCALLTYPE *SetGravity )( 
            __RPC__in IAnchor * This,
            /* [in] */ TsGravity gravity);
        
        DECLSPEC_XFGVIRT(IAnchor, GetGravity)
        HRESULT ( STDMETHODCALLTYPE *GetGravity )( 
            __RPC__in IAnchor * This,
            /* [out] */ __RPC__out TsGravity *pgravity);
        
        DECLSPEC_XFGVIRT(IAnchor, IsEqual)
        HRESULT ( STDMETHODCALLTYPE *IsEqual )( 
            __RPC__in IAnchor * This,
            /* [in] */ __RPC__in_opt IAnchor *paWith,
            /* [out] */ __RPC__out BOOL *pfEqual);
        
        DECLSPEC_XFGVIRT(IAnchor, Compare)
        HRESULT ( STDMETHODCALLTYPE *Compare )( 
            __RPC__in IAnchor * This,
            /* [in] */ __RPC__in_opt IAnchor *paWith,
            /* [out] */ __RPC__out LONG *plResult);
        
        DECLSPEC_XFGVIRT(IAnchor, Shift)
        HRESULT ( STDMETHODCALLTYPE *Shift )( 
            __RPC__in IAnchor * This,
            /* [in] */ DWORD dwFlags,
            /* [in] */ LONG cchReq,
            /* [out] */ __RPC__out LONG *pcch,
            /* [in] */ __RPC__in_opt IAnchor *paHaltAnchor);
        
        DECLSPEC_XFGVIRT(IAnchor, ShiftTo)
        HRESULT ( STDMETHODCALLTYPE *ShiftTo )( 
            __RPC__in IAnchor * This,
            /* [in] */ __RPC__in_opt IAnchor *paSite);
        
        DECLSPEC_XFGVIRT(IAnchor, ShiftRegion)
        HRESULT ( STDMETHODCALLTYPE *ShiftRegion )( 
            __RPC__in IAnchor * This,
            /* [in] */ DWORD dwFlags,
            /* [in] */ TsShiftDir dir,
            /* [out] */ __RPC__out BOOL *pfNoRegion);
        
        DECLSPEC_XFGVIRT(IAnchor, SetChangeHistoryMask)
        HRESULT ( STDMETHODCALLTYPE *SetChangeHistoryMask )( 
            __RPC__in IAnchor * This,
            /* [in] */ DWORD dwMask);
        
        DECLSPEC_XFGVIRT(IAnchor, GetChangeHistory)
        HRESULT ( STDMETHODCALLTYPE *GetChangeHistory )( 
            __RPC__in IAnchor * This,
            /* [out] */ __RPC__out DWORD *pdwHistory);
        
        DECLSPEC_XFGVIRT(IAnchor, ClearChangeHistory)
        HRESULT ( STDMETHODCALLTYPE *ClearChangeHistory )( 
            __RPC__in IAnchor * This);
        
        DECLSPEC_XFGVIRT(IAnchor, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IAnchor * This,
            /* [out] */ __RPC__deref_out_opt IAnchor **ppaClone);
        
        END_INTERFACE
    } IAnchorVtbl;

    interface IAnchor
    {
        CONST_VTBL struct IAnchorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAnchor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAnchor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAnchor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAnchor_SetGravity(This,gravity)	\
    ( (This)->lpVtbl -> SetGravity(This,gravity) ) 

#define IAnchor_GetGravity(This,pgravity)	\
    ( (This)->lpVtbl -> GetGravity(This,pgravity) ) 

#define IAnchor_IsEqual(This,paWith,pfEqual)	\
    ( (This)->lpVtbl -> IsEqual(This,paWith,pfEqual) ) 

#define IAnchor_Compare(This,paWith,plResult)	\
    ( (This)->lpVtbl -> Compare(This,paWith,plResult) ) 

#define IAnchor_Shift(This,dwFlags,cchReq,pcch,paHaltAnchor)	\
    ( (This)->lpVtbl -> Shift(This,dwFlags,cchReq,pcch,paHaltAnchor) ) 

#define IAnchor_ShiftTo(This,paSite)	\
    ( (This)->lpVtbl -> ShiftTo(This,paSite) ) 

#define IAnchor_ShiftRegion(This,dwFlags,dir,pfNoRegion)	\
    ( (This)->lpVtbl -> ShiftRegion(This,dwFlags,dir,pfNoRegion) ) 

#define IAnchor_SetChangeHistoryMask(This,dwMask)	\
    ( (This)->lpVtbl -> SetChangeHistoryMask(This,dwMask) ) 

#define IAnchor_GetChangeHistory(This,pdwHistory)	\
    ( (This)->lpVtbl -> GetChangeHistory(This,pdwHistory) ) 

#define IAnchor_ClearChangeHistory(This)	\
    ( (This)->lpVtbl -> ClearChangeHistory(This) ) 

#define IAnchor_Clone(This,ppaClone)	\
    ( (This)->lpVtbl -> Clone(This,ppaClone) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAnchor_INTERFACE_DEFINED__ */


#ifndef __ITextStoreAnchor_INTERFACE_DEFINED__
#define __ITextStoreAnchor_INTERFACE_DEFINED__

/* interface ITextStoreAnchor */
/* [unique][uuid][object] */ 

#define	TS_GTA_HIDDEN	( 0x1 )

#define	TS_GEA_HIDDEN	( 0x1 )


EXTERN_C const IID IID_ITextStoreAnchor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9b2077b0-5f18-4dec-bee9-3cc722f5dfe0")
    ITextStoreAnchor : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AdviseSink( 
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][in] */ __RPC__in_opt IUnknown *punk,
            /* [in] */ DWORD dwMask) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnadviseSink( 
            /* [in] */ __RPC__in_opt IUnknown *punk) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RequestLock( 
            /* [in] */ DWORD dwLockFlags,
            /* [out] */ __RPC__out HRESULT *phrSession) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStatus( 
            /* [out] */ __RPC__out TS_STATUS *pdcs) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE QueryInsert( 
            /* [in] */ __RPC__in_opt IAnchor *paTestStart,
            /* [in] */ __RPC__in_opt IAnchor *paTestEnd,
            /* [in] */ ULONG cch,
            /* [out] */ __RPC__deref_out_opt IAnchor **ppaResultStart,
            /* [out] */ __RPC__deref_out_opt IAnchor **ppaResultEnd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSelection( 
            /* [in] */ ULONG ulIndex,
            /* [in] */ ULONG ulCount,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(ulCount, *pcFetched) TS_SELECTION_ANCHOR *pSelection,
            /* [out] */ __RPC__out ULONG *pcFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetSelection( 
            /* [in] */ ULONG ulCount,
            /* [size_is][in] */ __RPC__in_ecount_full(ulCount) const TS_SELECTION_ANCHOR *pSelection) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetText( 
            /* [in] */ DWORD dwFlags,
            /* [in] */ __RPC__in_opt IAnchor *paStart,
            /* [in] */ __RPC__in_opt IAnchor *paEnd,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(cchReq, *pcch) WCHAR *pchText,
            /* [in] */ ULONG cchReq,
            /* [out] */ __RPC__out ULONG *pcch,
            /* [in] */ BOOL fUpdateAnchor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetText( 
            /* [in] */ DWORD dwFlags,
            /* [in] */ __RPC__in_opt IAnchor *paStart,
            /* [in] */ __RPC__in_opt IAnchor *paEnd,
            /* [size_is][in] */ __RPC__in_ecount_full(cch) const WCHAR *pchText,
            /* [in] */ ULONG cch) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFormattedText( 
            /* [in] */ __RPC__in_opt IAnchor *paStart,
            /* [in] */ __RPC__in_opt IAnchor *paEnd,
            /* [out] */ __RPC__deref_out_opt IDataObject **ppDataObject) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetEmbedded( 
            /* [in] */ DWORD dwFlags,
            /* [in] */ __RPC__in_opt IAnchor *paPos,
            /* [in] */ __RPC__in REFGUID rguidService,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt IUnknown **ppunk) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InsertEmbedded( 
            /* [in] */ DWORD dwFlags,
            /* [in] */ __RPC__in_opt IAnchor *paStart,
            /* [in] */ __RPC__in_opt IAnchor *paEnd,
            /* [in] */ __RPC__in_opt IDataObject *pDataObject) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RequestSupportedAttrs( 
            /* [in] */ DWORD dwFlags,
            /* [in] */ ULONG cFilterAttrs,
            /* [unique][size_is][in] */ __RPC__in_ecount_full_opt(cFilterAttrs) const TS_ATTRID *paFilterAttrs) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RequestAttrsAtPosition( 
            /* [in] */ __RPC__in_opt IAnchor *paPos,
            /* [in] */ ULONG cFilterAttrs,
            /* [unique][size_is][in] */ __RPC__in_ecount_full_opt(cFilterAttrs) const TS_ATTRID *paFilterAttrs,
            /* [in] */ DWORD dwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RequestAttrsTransitioningAtPosition( 
            /* [in] */ __RPC__in_opt IAnchor *paPos,
            /* [in] */ ULONG cFilterAttrs,
            /* [unique][size_is][in] */ __RPC__in_ecount_full_opt(cFilterAttrs) const TS_ATTRID *paFilterAttrs,
            /* [in] */ DWORD dwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FindNextAttrTransition( 
            /* [in] */ __RPC__in_opt IAnchor *paStart,
            /* [in] */ __RPC__in_opt IAnchor *paHalt,
            /* [in] */ ULONG cFilterAttrs,
            /* [unique][size_is][in] */ __RPC__in_ecount_full_opt(cFilterAttrs) const TS_ATTRID *paFilterAttrs,
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__out BOOL *pfFound,
            /* [out] */ __RPC__out LONG *plFoundOffset) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RetrieveRequestedAttrs( 
            /* [in] */ ULONG ulCount,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(ulCount, *pcFetched) TS_ATTRVAL *paAttrVals,
            /* [out] */ __RPC__out ULONG *pcFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStart( 
            /* [out] */ __RPC__deref_out_opt IAnchor **ppaStart) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetEnd( 
            /* [out] */ __RPC__deref_out_opt IAnchor **ppaEnd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetActiveView( 
            /* [out] */ __RPC__out TsViewCookie *pvcView) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAnchorFromPoint( 
            /* [in] */ TsViewCookie vcView,
            /* [in] */ __RPC__in const POINT *ptScreen,
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__deref_out_opt IAnchor **ppaSite) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTextExt( 
            /* [in] */ TsViewCookie vcView,
            /* [in] */ __RPC__in_opt IAnchor *paStart,
            /* [in] */ __RPC__in_opt IAnchor *paEnd,
            /* [out] */ __RPC__out RECT *prc,
            /* [out] */ __RPC__out BOOL *pfClipped) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetScreenExt( 
            /* [in] */ TsViewCookie vcView,
            /* [out] */ __RPC__out RECT *prc) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetWnd( 
            /* [in] */ TsViewCookie vcView,
            /* [out] */ __RPC__deref_out_opt HWND *phwnd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE QueryInsertEmbedded( 
            /* [in] */ __RPC__in const GUID *pguidService,
            /* [in] */ __RPC__in const FORMATETC *pFormatEtc,
            /* [out] */ __RPC__out BOOL *pfInsertable) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InsertTextAtSelection( 
            /* [in] */ DWORD dwFlags,
            /* [size_is][in] */ __RPC__in_ecount_full(cch) const WCHAR *pchText,
            /* [in] */ ULONG cch,
            /* [out] */ __RPC__deref_out_opt IAnchor **ppaStart,
            /* [out] */ __RPC__deref_out_opt IAnchor **ppaEnd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InsertEmbeddedAtSelection( 
            /* [in] */ DWORD dwFlags,
            /* [in] */ __RPC__in_opt IDataObject *pDataObject,
            /* [out] */ __RPC__deref_out_opt IAnchor **ppaStart,
            /* [out] */ __RPC__deref_out_opt IAnchor **ppaEnd) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITextStoreAnchorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITextStoreAnchor * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITextStoreAnchor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITextStoreAnchor * This);
        
        DECLSPEC_XFGVIRT(ITextStoreAnchor, AdviseSink)
        HRESULT ( STDMETHODCALLTYPE *AdviseSink )( 
            __RPC__in ITextStoreAnchor * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][in] */ __RPC__in_opt IUnknown *punk,
            /* [in] */ DWORD dwMask);
        
        DECLSPEC_XFGVIRT(ITextStoreAnchor, UnadviseSink)
        HRESULT ( STDMETHODCALLTYPE *UnadviseSink )( 
            __RPC__in ITextStoreAnchor * This,
            /* [in] */ __RPC__in_opt IUnknown *punk);
        
        DECLSPEC_XFGVIRT(ITextStoreAnchor, RequestLock)
        HRESULT ( STDMETHODCALLTYPE *RequestLock )( 
            __RPC__in ITextStoreAnchor * This,
            /* [in] */ DWORD dwLockFlags,
            /* [out] */ __RPC__out HRESULT *phrSession);
        
        DECLSPEC_XFGVIRT(ITextStoreAnchor, GetStatus)
        HRESULT ( STDMETHODCALLTYPE *GetStatus )( 
            __RPC__in ITextStoreAnchor * This,
            /* [out] */ __RPC__out TS_STATUS *pdcs);
        
        DECLSPEC_XFGVIRT(ITextStoreAnchor, QueryInsert)
        HRESULT ( STDMETHODCALLTYPE *QueryInsert )( 
            __RPC__in ITextStoreAnchor * This,
            /* [in] */ __RPC__in_opt IAnchor *paTestStart,
            /* [in] */ __RPC__in_opt IAnchor *paTestEnd,
            /* [in] */ ULONG cch,
            /* [out] */ __RPC__deref_out_opt IAnchor **ppaResultStart,
            /* [out] */ __RPC__deref_out_opt IAnchor **ppaResultEnd);
        
        DECLSPEC_XFGVIRT(ITextStoreAnchor, GetSelection)
        HRESULT ( STDMETHODCALLTYPE *GetSelection )( 
            __RPC__in ITextStoreAnchor * This,
            /* [in] */ ULONG ulIndex,
            /* [in] */ ULONG ulCount,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(ulCount, *pcFetched) TS_SELECTION_ANCHOR *pSelection,
            /* [out] */ __RPC__out ULONG *pcFetched);
        
        DECLSPEC_XFGVIRT(ITextStoreAnchor, SetSelection)
        HRESULT ( STDMETHODCALLTYPE *SetSelection )( 
            __RPC__in ITextStoreAnchor * This,
            /* [in] */ ULONG ulCount,
            /* [size_is][in] */ __RPC__in_ecount_full(ulCount) const TS_SELECTION_ANCHOR *pSelection);
        
        DECLSPEC_XFGVIRT(ITextStoreAnchor, GetText)
        HRESULT ( STDMETHODCALLTYPE *GetText )( 
            __RPC__in ITextStoreAnchor * This,
            /* [in] */ DWORD dwFlags,
            /* [in] */ __RPC__in_opt IAnchor *paStart,
            /* [in] */ __RPC__in_opt IAnchor *paEnd,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(cchReq, *pcch) WCHAR *pchText,
            /* [in] */ ULONG cchReq,
            /* [out] */ __RPC__out ULONG *pcch,
            /* [in] */ BOOL fUpdateAnchor);
        
        DECLSPEC_XFGVIRT(ITextStoreAnchor, SetText)
        HRESULT ( STDMETHODCALLTYPE *SetText )( 
            __RPC__in ITextStoreAnchor * This,
            /* [in] */ DWORD dwFlags,
            /* [in] */ __RPC__in_opt IAnchor *paStart,
            /* [in] */ __RPC__in_opt IAnchor *paEnd,
            /* [size_is][in] */ __RPC__in_ecount_full(cch) const WCHAR *pchText,
            /* [in] */ ULONG cch);
        
        DECLSPEC_XFGVIRT(ITextStoreAnchor, GetFormattedText)
        HRESULT ( STDMETHODCALLTYPE *GetFormattedText )( 
            __RPC__in ITextStoreAnchor * This,
            /* [in] */ __RPC__in_opt IAnchor *paStart,
            /* [in] */ __RPC__in_opt IAnchor *paEnd,
            /* [out] */ __RPC__deref_out_opt IDataObject **ppDataObject);
        
        DECLSPEC_XFGVIRT(ITextStoreAnchor, GetEmbedded)
        HRESULT ( STDMETHODCALLTYPE *GetEmbedded )( 
            __RPC__in ITextStoreAnchor * This,
            /* [in] */ DWORD dwFlags,
            /* [in] */ __RPC__in_opt IAnchor *paPos,
            /* [in] */ __RPC__in REFGUID rguidService,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt IUnknown **ppunk);
        
        DECLSPEC_XFGVIRT(ITextStoreAnchor, InsertEmbedded)
        HRESULT ( STDMETHODCALLTYPE *InsertEmbedded )( 
            __RPC__in ITextStoreAnchor * This,
            /* [in] */ DWORD dwFlags,
            /* [in] */ __RPC__in_opt IAnchor *paStart,
            /* [in] */ __RPC__in_opt IAnchor *paEnd,
            /* [in] */ __RPC__in_opt IDataObject *pDataObject);
        
        DECLSPEC_XFGVIRT(ITextStoreAnchor, RequestSupportedAttrs)
        HRESULT ( STDMETHODCALLTYPE *RequestSupportedAttrs )( 
            __RPC__in ITextStoreAnchor * This,
            /* [in] */ DWORD dwFlags,
            /* [in] */ ULONG cFilterAttrs,
            /* [unique][size_is][in] */ __RPC__in_ecount_full_opt(cFilterAttrs) const TS_ATTRID *paFilterAttrs);
        
        DECLSPEC_XFGVIRT(ITextStoreAnchor, RequestAttrsAtPosition)
        HRESULT ( STDMETHODCALLTYPE *RequestAttrsAtPosition )( 
            __RPC__in ITextStoreAnchor * This,
            /* [in] */ __RPC__in_opt IAnchor *paPos,
            /* [in] */ ULONG cFilterAttrs,
            /* [unique][size_is][in] */ __RPC__in_ecount_full_opt(cFilterAttrs) const TS_ATTRID *paFilterAttrs,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(ITextStoreAnchor, RequestAttrsTransitioningAtPosition)
        HRESULT ( STDMETHODCALLTYPE *RequestAttrsTransitioningAtPosition )( 
            __RPC__in ITextStoreAnchor * This,
            /* [in] */ __RPC__in_opt IAnchor *paPos,
            /* [in] */ ULONG cFilterAttrs,
            /* [unique][size_is][in] */ __RPC__in_ecount_full_opt(cFilterAttrs) const TS_ATTRID *paFilterAttrs,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(ITextStoreAnchor, FindNextAttrTransition)
        HRESULT ( STDMETHODCALLTYPE *FindNextAttrTransition )( 
            __RPC__in ITextStoreAnchor * This,
            /* [in] */ __RPC__in_opt IAnchor *paStart,
            /* [in] */ __RPC__in_opt IAnchor *paHalt,
            /* [in] */ ULONG cFilterAttrs,
            /* [unique][size_is][in] */ __RPC__in_ecount_full_opt(cFilterAttrs) const TS_ATTRID *paFilterAttrs,
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__out BOOL *pfFound,
            /* [out] */ __RPC__out LONG *plFoundOffset);
        
        DECLSPEC_XFGVIRT(ITextStoreAnchor, RetrieveRequestedAttrs)
        HRESULT ( STDMETHODCALLTYPE *RetrieveRequestedAttrs )( 
            __RPC__in ITextStoreAnchor * This,
            /* [in] */ ULONG ulCount,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(ulCount, *pcFetched) TS_ATTRVAL *paAttrVals,
            /* [out] */ __RPC__out ULONG *pcFetched);
        
        DECLSPEC_XFGVIRT(ITextStoreAnchor, GetStart)
        HRESULT ( STDMETHODCALLTYPE *GetStart )( 
            __RPC__in ITextStoreAnchor * This,
            /* [out] */ __RPC__deref_out_opt IAnchor **ppaStart);
        
        DECLSPEC_XFGVIRT(ITextStoreAnchor, GetEnd)
        HRESULT ( STDMETHODCALLTYPE *GetEnd )( 
            __RPC__in ITextStoreAnchor * This,
            /* [out] */ __RPC__deref_out_opt IAnchor **ppaEnd);
        
        DECLSPEC_XFGVIRT(ITextStoreAnchor, GetActiveView)
        HRESULT ( STDMETHODCALLTYPE *GetActiveView )( 
            __RPC__in ITextStoreAnchor * This,
            /* [out] */ __RPC__out TsViewCookie *pvcView);
        
        DECLSPEC_XFGVIRT(ITextStoreAnchor, GetAnchorFromPoint)
        HRESULT ( STDMETHODCALLTYPE *GetAnchorFromPoint )( 
            __RPC__in ITextStoreAnchor * This,
            /* [in] */ TsViewCookie vcView,
            /* [in] */ __RPC__in const POINT *ptScreen,
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__deref_out_opt IAnchor **ppaSite);
        
        DECLSPEC_XFGVIRT(ITextStoreAnchor, GetTextExt)
        HRESULT ( STDMETHODCALLTYPE *GetTextExt )( 
            __RPC__in ITextStoreAnchor * This,
            /* [in] */ TsViewCookie vcView,
            /* [in] */ __RPC__in_opt IAnchor *paStart,
            /* [in] */ __RPC__in_opt IAnchor *paEnd,
            /* [out] */ __RPC__out RECT *prc,
            /* [out] */ __RPC__out BOOL *pfClipped);
        
        DECLSPEC_XFGVIRT(ITextStoreAnchor, GetScreenExt)
        HRESULT ( STDMETHODCALLTYPE *GetScreenExt )( 
            __RPC__in ITextStoreAnchor * This,
            /* [in] */ TsViewCookie vcView,
            /* [out] */ __RPC__out RECT *prc);
        
        DECLSPEC_XFGVIRT(ITextStoreAnchor, GetWnd)
        HRESULT ( STDMETHODCALLTYPE *GetWnd )( 
            __RPC__in ITextStoreAnchor * This,
            /* [in] */ TsViewCookie vcView,
            /* [out] */ __RPC__deref_out_opt HWND *phwnd);
        
        DECLSPEC_XFGVIRT(ITextStoreAnchor, QueryInsertEmbedded)
        HRESULT ( STDMETHODCALLTYPE *QueryInsertEmbedded )( 
            __RPC__in ITextStoreAnchor * This,
            /* [in] */ __RPC__in const GUID *pguidService,
            /* [in] */ __RPC__in const FORMATETC *pFormatEtc,
            /* [out] */ __RPC__out BOOL *pfInsertable);
        
        DECLSPEC_XFGVIRT(ITextStoreAnchor, InsertTextAtSelection)
        HRESULT ( STDMETHODCALLTYPE *InsertTextAtSelection )( 
            __RPC__in ITextStoreAnchor * This,
            /* [in] */ DWORD dwFlags,
            /* [size_is][in] */ __RPC__in_ecount_full(cch) const WCHAR *pchText,
            /* [in] */ ULONG cch,
            /* [out] */ __RPC__deref_out_opt IAnchor **ppaStart,
            /* [out] */ __RPC__deref_out_opt IAnchor **ppaEnd);
        
        DECLSPEC_XFGVIRT(ITextStoreAnchor, InsertEmbeddedAtSelection)
        HRESULT ( STDMETHODCALLTYPE *InsertEmbeddedAtSelection )( 
            __RPC__in ITextStoreAnchor * This,
            /* [in] */ DWORD dwFlags,
            /* [in] */ __RPC__in_opt IDataObject *pDataObject,
            /* [out] */ __RPC__deref_out_opt IAnchor **ppaStart,
            /* [out] */ __RPC__deref_out_opt IAnchor **ppaEnd);
        
        END_INTERFACE
    } ITextStoreAnchorVtbl;

    interface ITextStoreAnchor
    {
        CONST_VTBL struct ITextStoreAnchorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITextStoreAnchor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITextStoreAnchor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITextStoreAnchor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITextStoreAnchor_AdviseSink(This,riid,punk,dwMask)	\
    ( (This)->lpVtbl -> AdviseSink(This,riid,punk,dwMask) ) 

#define ITextStoreAnchor_UnadviseSink(This,punk)	\
    ( (This)->lpVtbl -> UnadviseSink(This,punk) ) 

#define ITextStoreAnchor_RequestLock(This,dwLockFlags,phrSession)	\
    ( (This)->lpVtbl -> RequestLock(This,dwLockFlags,phrSession) ) 

#define ITextStoreAnchor_GetStatus(This,pdcs)	\
    ( (This)->lpVtbl -> GetStatus(This,pdcs) ) 

#define ITextStoreAnchor_QueryInsert(This,paTestStart,paTestEnd,cch,ppaResultStart,ppaResultEnd)	\
    ( (This)->lpVtbl -> QueryInsert(This,paTestStart,paTestEnd,cch,ppaResultStart,ppaResultEnd) ) 

#define ITextStoreAnchor_GetSelection(This,ulIndex,ulCount,pSelection,pcFetched)	\
    ( (This)->lpVtbl -> GetSelection(This,ulIndex,ulCount,pSelection,pcFetched) ) 

#define ITextStoreAnchor_SetSelection(This,ulCount,pSelection)	\
    ( (This)->lpVtbl -> SetSelection(This,ulCount,pSelection) ) 

#define ITextStoreAnchor_GetText(This,dwFlags,paStart,paEnd,pchText,cchReq,pcch,fUpdateAnchor)	\
    ( (This)->lpVtbl -> GetText(This,dwFlags,paStart,paEnd,pchText,cchReq,pcch,fUpdateAnchor) ) 

#define ITextStoreAnchor_SetText(This,dwFlags,paStart,paEnd,pchText,cch)	\
    ( (This)->lpVtbl -> SetText(This,dwFlags,paStart,paEnd,pchText,cch) ) 

#define ITextStoreAnchor_GetFormattedText(This,paStart,paEnd,ppDataObject)	\
    ( (This)->lpVtbl -> GetFormattedText(This,paStart,paEnd,ppDataObject) ) 

#define ITextStoreAnchor_GetEmbedded(This,dwFlags,paPos,rguidService,riid,ppunk)	\
    ( (This)->lpVtbl -> GetEmbedded(This,dwFlags,paPos,rguidService,riid,ppunk) ) 

#define ITextStoreAnchor_InsertEmbedded(This,dwFlags,paStart,paEnd,pDataObject)	\
    ( (This)->lpVtbl -> InsertEmbedded(This,dwFlags,paStart,paEnd,pDataObject) ) 

#define ITextStoreAnchor_RequestSupportedAttrs(This,dwFlags,cFilterAttrs,paFilterAttrs)	\
    ( (This)->lpVtbl -> RequestSupportedAttrs(This,dwFlags,cFilterAttrs,paFilterAttrs) ) 

#define ITextStoreAnchor_RequestAttrsAtPosition(This,paPos,cFilterAttrs,paFilterAttrs,dwFlags)	\
    ( (This)->lpVtbl -> RequestAttrsAtPosition(This,paPos,cFilterAttrs,paFilterAttrs,dwFlags) ) 

#define ITextStoreAnchor_RequestAttrsTransitioningAtPosition(This,paPos,cFilterAttrs,paFilterAttrs,dwFlags)	\
    ( (This)->lpVtbl -> RequestAttrsTransitioningAtPosition(This,paPos,cFilterAttrs,paFilterAttrs,dwFlags) ) 

#define ITextStoreAnchor_FindNextAttrTransition(This,paStart,paHalt,cFilterAttrs,paFilterAttrs,dwFlags,pfFound,plFoundOffset)	\
    ( (This)->lpVtbl -> FindNextAttrTransition(This,paStart,paHalt,cFilterAttrs,paFilterAttrs,dwFlags,pfFound,plFoundOffset) ) 

#define ITextStoreAnchor_RetrieveRequestedAttrs(This,ulCount,paAttrVals,pcFetched)	\
    ( (This)->lpVtbl -> RetrieveRequestedAttrs(This,ulCount,paAttrVals,pcFetched) ) 

#define ITextStoreAnchor_GetStart(This,ppaStart)	\
    ( (This)->lpVtbl -> GetStart(This,ppaStart) ) 

#define ITextStoreAnchor_GetEnd(This,ppaEnd)	\
    ( (This)->lpVtbl -> GetEnd(This,ppaEnd) ) 

#define ITextStoreAnchor_GetActiveView(This,pvcView)	\
    ( (This)->lpVtbl -> GetActiveView(This,pvcView) ) 

#define ITextStoreAnchor_GetAnchorFromPoint(This,vcView,ptScreen,dwFlags,ppaSite)	\
    ( (This)->lpVtbl -> GetAnchorFromPoint(This,vcView,ptScreen,dwFlags,ppaSite) ) 

#define ITextStoreAnchor_GetTextExt(This,vcView,paStart,paEnd,prc,pfClipped)	\
    ( (This)->lpVtbl -> GetTextExt(This,vcView,paStart,paEnd,prc,pfClipped) ) 

#define ITextStoreAnchor_GetScreenExt(This,vcView,prc)	\
    ( (This)->lpVtbl -> GetScreenExt(This,vcView,prc) ) 

#define ITextStoreAnchor_GetWnd(This,vcView,phwnd)	\
    ( (This)->lpVtbl -> GetWnd(This,vcView,phwnd) ) 

#define ITextStoreAnchor_QueryInsertEmbedded(This,pguidService,pFormatEtc,pfInsertable)	\
    ( (This)->lpVtbl -> QueryInsertEmbedded(This,pguidService,pFormatEtc,pfInsertable) ) 

#define ITextStoreAnchor_InsertTextAtSelection(This,dwFlags,pchText,cch,ppaStart,ppaEnd)	\
    ( (This)->lpVtbl -> InsertTextAtSelection(This,dwFlags,pchText,cch,ppaStart,ppaEnd) ) 

#define ITextStoreAnchor_InsertEmbeddedAtSelection(This,dwFlags,pDataObject,ppaStart,ppaEnd)	\
    ( (This)->lpVtbl -> InsertEmbeddedAtSelection(This,dwFlags,pDataObject,ppaStart,ppaEnd) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITextStoreAnchor_INTERFACE_DEFINED__ */


#ifndef __ITextStoreAnchorSink_INTERFACE_DEFINED__
#define __ITextStoreAnchorSink_INTERFACE_DEFINED__

/* interface ITextStoreAnchorSink */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ITextStoreAnchorSink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("aa80e905-2021-11d2-93e0-0060b067b86e")
    ITextStoreAnchorSink : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnTextChange( 
            /* [in] */ DWORD dwFlags,
            /* [in] */ __RPC__in_opt IAnchor *paStart,
            /* [in] */ __RPC__in_opt IAnchor *paEnd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnSelectionChange( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnLayoutChange( 
            /* [in] */ TsLayoutCode lcode,
            /* [in] */ TsViewCookie vcView) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnStatusChange( 
            /* [in] */ DWORD dwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnAttrsChange( 
            /* [in] */ __RPC__in_opt IAnchor *paStart,
            /* [in] */ __RPC__in_opt IAnchor *paEnd,
            /* [in] */ ULONG cAttrs,
            /* [size_is][in] */ __RPC__in_ecount_full(cAttrs) const TS_ATTRID *paAttrs) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnLockGranted( 
            /* [in] */ DWORD dwLockFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnStartEditTransaction( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnEndEditTransaction( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITextStoreAnchorSinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITextStoreAnchorSink * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITextStoreAnchorSink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITextStoreAnchorSink * This);
        
        DECLSPEC_XFGVIRT(ITextStoreAnchorSink, OnTextChange)
        HRESULT ( STDMETHODCALLTYPE *OnTextChange )( 
            __RPC__in ITextStoreAnchorSink * This,
            /* [in] */ DWORD dwFlags,
            /* [in] */ __RPC__in_opt IAnchor *paStart,
            /* [in] */ __RPC__in_opt IAnchor *paEnd);
        
        DECLSPEC_XFGVIRT(ITextStoreAnchorSink, OnSelectionChange)
        HRESULT ( STDMETHODCALLTYPE *OnSelectionChange )( 
            __RPC__in ITextStoreAnchorSink * This);
        
        DECLSPEC_XFGVIRT(ITextStoreAnchorSink, OnLayoutChange)
        HRESULT ( STDMETHODCALLTYPE *OnLayoutChange )( 
            __RPC__in ITextStoreAnchorSink * This,
            /* [in] */ TsLayoutCode lcode,
            /* [in] */ TsViewCookie vcView);
        
        DECLSPEC_XFGVIRT(ITextStoreAnchorSink, OnStatusChange)
        HRESULT ( STDMETHODCALLTYPE *OnStatusChange )( 
            __RPC__in ITextStoreAnchorSink * This,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(ITextStoreAnchorSink, OnAttrsChange)
        HRESULT ( STDMETHODCALLTYPE *OnAttrsChange )( 
            __RPC__in ITextStoreAnchorSink * This,
            /* [in] */ __RPC__in_opt IAnchor *paStart,
            /* [in] */ __RPC__in_opt IAnchor *paEnd,
            /* [in] */ ULONG cAttrs,
            /* [size_is][in] */ __RPC__in_ecount_full(cAttrs) const TS_ATTRID *paAttrs);
        
        DECLSPEC_XFGVIRT(ITextStoreAnchorSink, OnLockGranted)
        HRESULT ( STDMETHODCALLTYPE *OnLockGranted )( 
            __RPC__in ITextStoreAnchorSink * This,
            /* [in] */ DWORD dwLockFlags);
        
        DECLSPEC_XFGVIRT(ITextStoreAnchorSink, OnStartEditTransaction)
        HRESULT ( STDMETHODCALLTYPE *OnStartEditTransaction )( 
            __RPC__in ITextStoreAnchorSink * This);
        
        DECLSPEC_XFGVIRT(ITextStoreAnchorSink, OnEndEditTransaction)
        HRESULT ( STDMETHODCALLTYPE *OnEndEditTransaction )( 
            __RPC__in ITextStoreAnchorSink * This);
        
        END_INTERFACE
    } ITextStoreAnchorSinkVtbl;

    interface ITextStoreAnchorSink
    {
        CONST_VTBL struct ITextStoreAnchorSinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITextStoreAnchorSink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITextStoreAnchorSink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITextStoreAnchorSink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITextStoreAnchorSink_OnTextChange(This,dwFlags,paStart,paEnd)	\
    ( (This)->lpVtbl -> OnTextChange(This,dwFlags,paStart,paEnd) ) 

#define ITextStoreAnchorSink_OnSelectionChange(This)	\
    ( (This)->lpVtbl -> OnSelectionChange(This) ) 

#define ITextStoreAnchorSink_OnLayoutChange(This,lcode,vcView)	\
    ( (This)->lpVtbl -> OnLayoutChange(This,lcode,vcView) ) 

#define ITextStoreAnchorSink_OnStatusChange(This,dwFlags)	\
    ( (This)->lpVtbl -> OnStatusChange(This,dwFlags) ) 

#define ITextStoreAnchorSink_OnAttrsChange(This,paStart,paEnd,cAttrs,paAttrs)	\
    ( (This)->lpVtbl -> OnAttrsChange(This,paStart,paEnd,cAttrs,paAttrs) ) 

#define ITextStoreAnchorSink_OnLockGranted(This,dwLockFlags)	\
    ( (This)->lpVtbl -> OnLockGranted(This,dwLockFlags) ) 

#define ITextStoreAnchorSink_OnStartEditTransaction(This)	\
    ( (This)->lpVtbl -> OnStartEditTransaction(This) ) 

#define ITextStoreAnchorSink_OnEndEditTransaction(This)	\
    ( (This)->lpVtbl -> OnEndEditTransaction(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITextStoreAnchorSink_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_textstor_0000_0006 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_textstor_0000_0006_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_textstor_0000_0006_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  CLIPFORMAT_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in CLIPFORMAT * ); 
unsigned char * __RPC_USER  CLIPFORMAT_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in CLIPFORMAT * ); 
unsigned char * __RPC_USER  CLIPFORMAT_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out CLIPFORMAT * ); 
void                      __RPC_USER  CLIPFORMAT_UserFree(     __RPC__in unsigned long *, __RPC__in CLIPFORMAT * ); 

unsigned long             __RPC_USER  HWND_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HWND * ); 
void                      __RPC_USER  HWND_UserFree(     __RPC__in unsigned long *, __RPC__in HWND * ); 

unsigned long             __RPC_USER  VARIANT_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

unsigned long             __RPC_USER  CLIPFORMAT_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in CLIPFORMAT * ); 
unsigned char * __RPC_USER  CLIPFORMAT_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in CLIPFORMAT * ); 
unsigned char * __RPC_USER  CLIPFORMAT_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out CLIPFORMAT * ); 
void                      __RPC_USER  CLIPFORMAT_UserFree64(     __RPC__in unsigned long *, __RPC__in CLIPFORMAT * ); 

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


