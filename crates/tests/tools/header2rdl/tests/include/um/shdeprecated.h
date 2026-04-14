

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

#ifndef __shdeprecated_h__
#define __shdeprecated_h__

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

#ifndef __ITravelEntry_FWD_DEFINED__
#define __ITravelEntry_FWD_DEFINED__
typedef interface ITravelEntry ITravelEntry;

#endif 	/* __ITravelEntry_FWD_DEFINED__ */


#ifndef __ITravelLog_FWD_DEFINED__
#define __ITravelLog_FWD_DEFINED__
typedef interface ITravelLog ITravelLog;

#endif 	/* __ITravelLog_FWD_DEFINED__ */


#ifndef __IExpDispSupportXP_FWD_DEFINED__
#define __IExpDispSupportXP_FWD_DEFINED__
typedef interface IExpDispSupportXP IExpDispSupportXP;

#endif 	/* __IExpDispSupportXP_FWD_DEFINED__ */


#ifndef __IExpDispSupport_FWD_DEFINED__
#define __IExpDispSupport_FWD_DEFINED__
typedef interface IExpDispSupport IExpDispSupport;

#endif 	/* __IExpDispSupport_FWD_DEFINED__ */


#ifndef __IBrowserService_FWD_DEFINED__
#define __IBrowserService_FWD_DEFINED__
typedef interface IBrowserService IBrowserService;

#endif 	/* __IBrowserService_FWD_DEFINED__ */


#ifndef __IShellService_FWD_DEFINED__
#define __IShellService_FWD_DEFINED__
typedef interface IShellService IShellService;

#endif 	/* __IShellService_FWD_DEFINED__ */


#ifndef __IBrowserService2_FWD_DEFINED__
#define __IBrowserService2_FWD_DEFINED__
typedef interface IBrowserService2 IBrowserService2;

#endif 	/* __IBrowserService2_FWD_DEFINED__ */


#ifndef __IBrowserService3_FWD_DEFINED__
#define __IBrowserService3_FWD_DEFINED__
typedef interface IBrowserService3 IBrowserService3;

#endif 	/* __IBrowserService3_FWD_DEFINED__ */


#ifndef __IBrowserService4_FWD_DEFINED__
#define __IBrowserService4_FWD_DEFINED__
typedef interface IBrowserService4 IBrowserService4;

#endif 	/* __IBrowserService4_FWD_DEFINED__ */


#ifndef __ITrackShellMenu_FWD_DEFINED__
#define __ITrackShellMenu_FWD_DEFINED__
typedef interface ITrackShellMenu ITrackShellMenu;

#endif 	/* __ITrackShellMenu_FWD_DEFINED__ */


#ifndef __TrackShellMenu_FWD_DEFINED__
#define __TrackShellMenu_FWD_DEFINED__

#ifdef __cplusplus
typedef class TrackShellMenu TrackShellMenu;
#else
typedef struct TrackShellMenu TrackShellMenu;
#endif /* __cplusplus */

#endif 	/* __TrackShellMenu_FWD_DEFINED__ */


/* header files for imported files */
#include "objidl.h"
#include "ocidl.h"
#include "shtypes.h"
#include "tlogstg.h"
#include "shobjidl_core.h"
#include "hlink.h"
#include "exdisp.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_shdeprecated_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#define TLOG_BACK       -1
#define TLOG_CURRENT     0
#define TLOG_FORE        1

#define TLMENUF_INCLUDECURRENT      0x00000001
#define TLMENUF_CHECKCURRENT        (TLMENUF_INCLUDECURRENT | 0x00000002)
#define TLMENUF_BACK                0x00000010  // Default
#define TLMENUF_FORE                0x00000020
#define TLMENUF_BACKANDFORTH        (TLMENUF_BACK | TLMENUF_FORE | TLMENUF_INCLUDECURRENT)



extern RPC_IF_HANDLE __MIDL_itf_shdeprecated_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_shdeprecated_0000_0000_v0_0_s_ifspec;

#ifndef __ITravelEntry_INTERFACE_DEFINED__
#define __ITravelEntry_INTERFACE_DEFINED__

/* interface ITravelEntry */
/* [helpcontext][helpstring][hidden][object][uuid] */ 


EXTERN_C const IID IID_ITravelEntry;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F46EDB3B-BC2F-11d0-9412-00AA00A3EBD3")
    ITravelEntry : public IUnknown
    {
    public:
        virtual /* [helpcontext][helpstring] */ HRESULT STDMETHODCALLTYPE Invoke( 
            /* [in] */ __RPC__in_opt IUnknown *punk) = 0;
        
        virtual /* [helpcontext][helpstring] */ HRESULT STDMETHODCALLTYPE Update( 
            /* [in] */ __RPC__in_opt IUnknown *punk,
            /* [in] */ BOOL fIsLocalAnchor) = 0;
        
        virtual /* [helpcontext][helpstring] */ HRESULT STDMETHODCALLTYPE GetPidl( 
            /* [out] */ __RPC__deref_out_opt PIDLIST_ABSOLUTE *ppidl) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITravelEntryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITravelEntry * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITravelEntry * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITravelEntry * This);
        
        DECLSPEC_XFGVIRT(ITravelEntry, Invoke)
        /* [helpcontext][helpstring] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            __RPC__in ITravelEntry * This,
            /* [in] */ __RPC__in_opt IUnknown *punk);
        
        DECLSPEC_XFGVIRT(ITravelEntry, Update)
        /* [helpcontext][helpstring] */ HRESULT ( STDMETHODCALLTYPE *Update )( 
            __RPC__in ITravelEntry * This,
            /* [in] */ __RPC__in_opt IUnknown *punk,
            /* [in] */ BOOL fIsLocalAnchor);
        
        DECLSPEC_XFGVIRT(ITravelEntry, GetPidl)
        /* [helpcontext][helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetPidl )( 
            __RPC__in ITravelEntry * This,
            /* [out] */ __RPC__deref_out_opt PIDLIST_ABSOLUTE *ppidl);
        
        END_INTERFACE
    } ITravelEntryVtbl;

    interface ITravelEntry
    {
        CONST_VTBL struct ITravelEntryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITravelEntry_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITravelEntry_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITravelEntry_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITravelEntry_Invoke(This,punk)	\
    ( (This)->lpVtbl -> Invoke(This,punk) ) 

#define ITravelEntry_Update(This,punk,fIsLocalAnchor)	\
    ( (This)->lpVtbl -> Update(This,punk,fIsLocalAnchor) ) 

#define ITravelEntry_GetPidl(This,ppidl)	\
    ( (This)->lpVtbl -> GetPidl(This,ppidl) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITravelEntry_INTERFACE_DEFINED__ */


#ifndef __ITravelLog_INTERFACE_DEFINED__
#define __ITravelLog_INTERFACE_DEFINED__

/* interface ITravelLog */
/* [helpcontext][helpstring][hidden][object][uuid] */ 


EXTERN_C const IID IID_ITravelLog;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("66A9CB08-4802-11d2-A561-00A0C92DBFE8")
    ITravelLog : public IUnknown
    {
    public:
        virtual /* [helpcontext][helpstring] */ HRESULT STDMETHODCALLTYPE AddEntry( 
            /* [in] */ __RPC__in_opt IUnknown *punk,
            /* [in] */ BOOL fIsLocalAnchor) = 0;
        
        virtual /* [helpcontext][helpstring] */ HRESULT STDMETHODCALLTYPE UpdateEntry( 
            /* [in] */ __RPC__in_opt IUnknown *punk,
            /* [in] */ BOOL fIsLocalAnchor) = 0;
        
        virtual /* [helpcontext][helpstring] */ HRESULT STDMETHODCALLTYPE UpdateExternal( 
            /* [in] */ __RPC__in_opt IUnknown *punk,
            /* [in] */ __RPC__in_opt IUnknown *punkHLBrowseContext) = 0;
        
        virtual /* [helpcontext][helpstring] */ HRESULT STDMETHODCALLTYPE Travel( 
            /* [in] */ __RPC__in_opt IUnknown *punk,
            /* [in] */ int iOffset) = 0;
        
        virtual /* [local][helpcontext][helpstring] */ HRESULT STDMETHODCALLTYPE GetTravelEntry( 
            /* [annotation][in] */ 
            _In_  IUnknown *punk,
            /* [annotation][in] */ 
            _In_  int iOffset,
            /* [annotation][out] */ 
            _Outptr_opt_  ITravelEntry **ppte) = 0;
        
        virtual /* [helpcontext][helpstring] */ HRESULT STDMETHODCALLTYPE FindTravelEntry( 
            /* [in] */ __RPC__in_opt IUnknown *punk,
            /* [in] */ __RPC__in PCIDLIST_ABSOLUTE pidl,
            /* [out] */ __RPC__deref_out_opt ITravelEntry **ppte) = 0;
        
        virtual /* [helpcontext][helpstring] */ HRESULT STDMETHODCALLTYPE GetToolTipText( 
            /* [in] */ __RPC__in_opt IUnknown *punk,
            /* [in] */ int iOffset,
            /* [in] */ int idsTemplate,
            /* [size_is][out] */ __RPC__out_ecount_full(cchText) LPWSTR pwzText,
            /* [in] */ DWORD cchText) = 0;
        
        virtual /* [helpcontext][helpstring] */ HRESULT STDMETHODCALLTYPE InsertMenuEntries( 
            /* [in] */ __RPC__in_opt IUnknown *punk,
            /* [in] */ __RPC__in HMENU hmenu,
            /* [in] */ int nPos,
            /* [in] */ int idFirst,
            /* [in] */ int idLast,
            /* [in] */ DWORD dwFlags) = 0;
        
        virtual /* [helpcontext][helpstring] */ HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ __RPC__deref_out_opt ITravelLog **pptl) = 0;
        
        virtual /* [local][helpcontext][helpstring] */ DWORD STDMETHODCALLTYPE CountEntries( 
            /* [in] */ IUnknown *punk) = 0;
        
        virtual /* [helpcontext][helpstring] */ HRESULT STDMETHODCALLTYPE Revert( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITravelLogVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITravelLog * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITravelLog * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITravelLog * This);
        
        DECLSPEC_XFGVIRT(ITravelLog, AddEntry)
        /* [helpcontext][helpstring] */ HRESULT ( STDMETHODCALLTYPE *AddEntry )( 
            __RPC__in ITravelLog * This,
            /* [in] */ __RPC__in_opt IUnknown *punk,
            /* [in] */ BOOL fIsLocalAnchor);
        
        DECLSPEC_XFGVIRT(ITravelLog, UpdateEntry)
        /* [helpcontext][helpstring] */ HRESULT ( STDMETHODCALLTYPE *UpdateEntry )( 
            __RPC__in ITravelLog * This,
            /* [in] */ __RPC__in_opt IUnknown *punk,
            /* [in] */ BOOL fIsLocalAnchor);
        
        DECLSPEC_XFGVIRT(ITravelLog, UpdateExternal)
        /* [helpcontext][helpstring] */ HRESULT ( STDMETHODCALLTYPE *UpdateExternal )( 
            __RPC__in ITravelLog * This,
            /* [in] */ __RPC__in_opt IUnknown *punk,
            /* [in] */ __RPC__in_opt IUnknown *punkHLBrowseContext);
        
        DECLSPEC_XFGVIRT(ITravelLog, Travel)
        /* [helpcontext][helpstring] */ HRESULT ( STDMETHODCALLTYPE *Travel )( 
            __RPC__in ITravelLog * This,
            /* [in] */ __RPC__in_opt IUnknown *punk,
            /* [in] */ int iOffset);
        
        DECLSPEC_XFGVIRT(ITravelLog, GetTravelEntry)
        /* [local][helpcontext][helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetTravelEntry )( 
            ITravelLog * This,
            /* [annotation][in] */ 
            _In_  IUnknown *punk,
            /* [annotation][in] */ 
            _In_  int iOffset,
            /* [annotation][out] */ 
            _Outptr_opt_  ITravelEntry **ppte);
        
        DECLSPEC_XFGVIRT(ITravelLog, FindTravelEntry)
        /* [helpcontext][helpstring] */ HRESULT ( STDMETHODCALLTYPE *FindTravelEntry )( 
            __RPC__in ITravelLog * This,
            /* [in] */ __RPC__in_opt IUnknown *punk,
            /* [in] */ __RPC__in PCIDLIST_ABSOLUTE pidl,
            /* [out] */ __RPC__deref_out_opt ITravelEntry **ppte);
        
        DECLSPEC_XFGVIRT(ITravelLog, GetToolTipText)
        /* [helpcontext][helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetToolTipText )( 
            __RPC__in ITravelLog * This,
            /* [in] */ __RPC__in_opt IUnknown *punk,
            /* [in] */ int iOffset,
            /* [in] */ int idsTemplate,
            /* [size_is][out] */ __RPC__out_ecount_full(cchText) LPWSTR pwzText,
            /* [in] */ DWORD cchText);
        
        DECLSPEC_XFGVIRT(ITravelLog, InsertMenuEntries)
        /* [helpcontext][helpstring] */ HRESULT ( STDMETHODCALLTYPE *InsertMenuEntries )( 
            __RPC__in ITravelLog * This,
            /* [in] */ __RPC__in_opt IUnknown *punk,
            /* [in] */ __RPC__in HMENU hmenu,
            /* [in] */ int nPos,
            /* [in] */ int idFirst,
            /* [in] */ int idLast,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(ITravelLog, Clone)
        /* [helpcontext][helpstring] */ HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in ITravelLog * This,
            /* [out] */ __RPC__deref_out_opt ITravelLog **pptl);
        
        DECLSPEC_XFGVIRT(ITravelLog, CountEntries)
        /* [local][helpcontext][helpstring] */ DWORD ( STDMETHODCALLTYPE *CountEntries )( 
            ITravelLog * This,
            /* [in] */ IUnknown *punk);
        
        DECLSPEC_XFGVIRT(ITravelLog, Revert)
        /* [helpcontext][helpstring] */ HRESULT ( STDMETHODCALLTYPE *Revert )( 
            __RPC__in ITravelLog * This);
        
        END_INTERFACE
    } ITravelLogVtbl;

    interface ITravelLog
    {
        CONST_VTBL struct ITravelLogVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITravelLog_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITravelLog_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITravelLog_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITravelLog_AddEntry(This,punk,fIsLocalAnchor)	\
    ( (This)->lpVtbl -> AddEntry(This,punk,fIsLocalAnchor) ) 

#define ITravelLog_UpdateEntry(This,punk,fIsLocalAnchor)	\
    ( (This)->lpVtbl -> UpdateEntry(This,punk,fIsLocalAnchor) ) 

#define ITravelLog_UpdateExternal(This,punk,punkHLBrowseContext)	\
    ( (This)->lpVtbl -> UpdateExternal(This,punk,punkHLBrowseContext) ) 

#define ITravelLog_Travel(This,punk,iOffset)	\
    ( (This)->lpVtbl -> Travel(This,punk,iOffset) ) 

#define ITravelLog_GetTravelEntry(This,punk,iOffset,ppte)	\
    ( (This)->lpVtbl -> GetTravelEntry(This,punk,iOffset,ppte) ) 

#define ITravelLog_FindTravelEntry(This,punk,pidl,ppte)	\
    ( (This)->lpVtbl -> FindTravelEntry(This,punk,pidl,ppte) ) 

#define ITravelLog_GetToolTipText(This,punk,iOffset,idsTemplate,pwzText,cchText)	\
    ( (This)->lpVtbl -> GetToolTipText(This,punk,iOffset,idsTemplate,pwzText,cchText) ) 

#define ITravelLog_InsertMenuEntries(This,punk,hmenu,nPos,idFirst,idLast,dwFlags)	\
    ( (This)->lpVtbl -> InsertMenuEntries(This,punk,hmenu,nPos,idFirst,idLast,dwFlags) ) 

#define ITravelLog_Clone(This,pptl)	\
    ( (This)->lpVtbl -> Clone(This,pptl) ) 

#define ITravelLog_CountEntries(This,punk)	\
    ( (This)->lpVtbl -> CountEntries(This,punk) ) 

#define ITravelLog_Revert(This)	\
    ( (This)->lpVtbl -> Revert(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [call_as] */ HRESULT STDMETHODCALLTYPE ITravelLog_RemoteGetTravelEntry_Proxy( 
    __RPC__in ITravelLog * This,
    /* [in] */ __RPC__in_opt IUnknown *punk,
    /* [in] */ int iOffset,
    /* [out] */ __RPC__deref_out_opt ITravelEntry **ppte);


void __RPC_STUB ITravelLog_RemoteGetTravelEntry_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __ITravelLog_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_shdeprecated_0000_0002 */
/* [local] */ 

#ifdef __cplusplus
class CIE4ConnectionPoint : public IConnectionPoint
{
    // This is how you actually fire the events
    // Those called by shell32 are virtual
    virtual HRESULT DoInvokeIE4(BOOL *pf, void **ppv, DISPID dispid, DISPPARAMS *pdispparams) PURE;
    // This helper function does work that callers of DoInvoke often need done
    virtual HRESULT DoInvokePIDLIE4(DISPID dispid, PCIDLIST_ABSOLUTE pidl, BOOL fCanCancel) PURE;
};
#else // !__cplusplus
typedef void *CIE4ConnectionPoint;

#endif// !__cplusplus


extern RPC_IF_HANDLE __MIDL_itf_shdeprecated_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_shdeprecated_0000_0002_v0_0_s_ifspec;

#ifndef __IExpDispSupportXP_INTERFACE_DEFINED__
#define __IExpDispSupportXP_INTERFACE_DEFINED__

/* interface IExpDispSupportXP */
/* [local][object][uuid] */ 


EXTERN_C const IID IID_IExpDispSupportXP;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2f0dd58c-f789-4f14-99fb-9293b3c9c212")
    IExpDispSupportXP : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE FindCIE4ConnectionPoint( 
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][out] */ 
            _Outptr_  CIE4ConnectionPoint **ppccp) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnTranslateAccelerator( 
            /* [annotation][in] */ 
            _In_  MSG *pMsg,
            /* [annotation][in] */ 
            _In_  DWORD grfModifiers) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnInvoke( 
            /* [annotation][in] */ 
            _In_  DISPID dispidMember,
            /* [annotation][in] */ 
            _In_  REFIID iid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][in] */ 
            _In_  DISPPARAMS *pdispparams,
            /* [annotation][out] */ 
            _Out_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_  EXCEPINFO *pexcepinfo,
            /* [annotation][out] */ 
            _Out_  UINT *puArgErr) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IExpDispSupportXPVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IExpDispSupportXP * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IExpDispSupportXP * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IExpDispSupportXP * This);
        
        DECLSPEC_XFGVIRT(IExpDispSupportXP, FindCIE4ConnectionPoint)
        HRESULT ( STDMETHODCALLTYPE *FindCIE4ConnectionPoint )( 
            IExpDispSupportXP * This,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][out] */ 
            _Outptr_  CIE4ConnectionPoint **ppccp);
        
        DECLSPEC_XFGVIRT(IExpDispSupportXP, OnTranslateAccelerator)
        HRESULT ( STDMETHODCALLTYPE *OnTranslateAccelerator )( 
            IExpDispSupportXP * This,
            /* [annotation][in] */ 
            _In_  MSG *pMsg,
            /* [annotation][in] */ 
            _In_  DWORD grfModifiers);
        
        DECLSPEC_XFGVIRT(IExpDispSupportXP, OnInvoke)
        HRESULT ( STDMETHODCALLTYPE *OnInvoke )( 
            IExpDispSupportXP * This,
            /* [annotation][in] */ 
            _In_  DISPID dispidMember,
            /* [annotation][in] */ 
            _In_  REFIID iid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][in] */ 
            _In_  DISPPARAMS *pdispparams,
            /* [annotation][out] */ 
            _Out_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_  EXCEPINFO *pexcepinfo,
            /* [annotation][out] */ 
            _Out_  UINT *puArgErr);
        
        END_INTERFACE
    } IExpDispSupportXPVtbl;

    interface IExpDispSupportXP
    {
        CONST_VTBL struct IExpDispSupportXPVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IExpDispSupportXP_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IExpDispSupportXP_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IExpDispSupportXP_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IExpDispSupportXP_FindCIE4ConnectionPoint(This,riid,ppccp)	\
    ( (This)->lpVtbl -> FindCIE4ConnectionPoint(This,riid,ppccp) ) 

#define IExpDispSupportXP_OnTranslateAccelerator(This,pMsg,grfModifiers)	\
    ( (This)->lpVtbl -> OnTranslateAccelerator(This,pMsg,grfModifiers) ) 

#define IExpDispSupportXP_OnInvoke(This,dispidMember,iid,lcid,wFlags,pdispparams,pVarResult,pexcepinfo,puArgErr)	\
    ( (This)->lpVtbl -> OnInvoke(This,dispidMember,iid,lcid,wFlags,pdispparams,pVarResult,pexcepinfo,puArgErr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IExpDispSupportXP_INTERFACE_DEFINED__ */


#ifndef __IExpDispSupport_INTERFACE_DEFINED__
#define __IExpDispSupport_INTERFACE_DEFINED__

/* interface IExpDispSupport */
/* [local][object][uuid] */ 


EXTERN_C const IID IID_IExpDispSupport;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0D7D1D00-6FC0-11D0-A974-00C04FD705A2")
    IExpDispSupport : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE FindConnectionPoint( 
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][out] */ 
            _Outptr_  IConnectionPoint **ppccp) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnTranslateAccelerator( 
            /* [annotation][in] */ 
            _In_  MSG *pMsg,
            /* [annotation][in] */ 
            _In_  DWORD grfModifiers) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnInvoke( 
            /* [annotation][in] */ 
            _In_  DISPID dispidMember,
            /* [annotation][in] */ 
            _In_  REFIID iid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][in] */ 
            _In_  DISPPARAMS *pdispparams,
            /* [annotation][out] */ 
            _Out_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_  EXCEPINFO *pexcepinfo,
            /* [annotation][out] */ 
            _Out_  UINT *puArgErr) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IExpDispSupportVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IExpDispSupport * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IExpDispSupport * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IExpDispSupport * This);
        
        DECLSPEC_XFGVIRT(IExpDispSupport, FindConnectionPoint)
        HRESULT ( STDMETHODCALLTYPE *FindConnectionPoint )( 
            IExpDispSupport * This,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][out] */ 
            _Outptr_  IConnectionPoint **ppccp);
        
        DECLSPEC_XFGVIRT(IExpDispSupport, OnTranslateAccelerator)
        HRESULT ( STDMETHODCALLTYPE *OnTranslateAccelerator )( 
            IExpDispSupport * This,
            /* [annotation][in] */ 
            _In_  MSG *pMsg,
            /* [annotation][in] */ 
            _In_  DWORD grfModifiers);
        
        DECLSPEC_XFGVIRT(IExpDispSupport, OnInvoke)
        HRESULT ( STDMETHODCALLTYPE *OnInvoke )( 
            IExpDispSupport * This,
            /* [annotation][in] */ 
            _In_  DISPID dispidMember,
            /* [annotation][in] */ 
            _In_  REFIID iid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][in] */ 
            _In_  DISPPARAMS *pdispparams,
            /* [annotation][out] */ 
            _Out_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_  EXCEPINFO *pexcepinfo,
            /* [annotation][out] */ 
            _Out_  UINT *puArgErr);
        
        END_INTERFACE
    } IExpDispSupportVtbl;

    interface IExpDispSupport
    {
        CONST_VTBL struct IExpDispSupportVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IExpDispSupport_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IExpDispSupport_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IExpDispSupport_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IExpDispSupport_FindConnectionPoint(This,riid,ppccp)	\
    ( (This)->lpVtbl -> FindConnectionPoint(This,riid,ppccp) ) 

#define IExpDispSupport_OnTranslateAccelerator(This,pMsg,grfModifiers)	\
    ( (This)->lpVtbl -> OnTranslateAccelerator(This,pMsg,grfModifiers) ) 

#define IExpDispSupport_OnInvoke(This,dispidMember,iid,lcid,wFlags,pdispparams,pVarResult,pexcepinfo,puArgErr)	\
    ( (This)->lpVtbl -> OnInvoke(This,dispidMember,iid,lcid,wFlags,pdispparams,pVarResult,pexcepinfo,puArgErr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IExpDispSupport_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_shdeprecated_0000_0004 */
/* [local] */ 

typedef /* [v1_enum] */ 
enum tagBNSTATE
    {
        BNS_NORMAL	= 0,
        BNS_BEGIN_NAVIGATE	= 1,
        BNS_NAVIGATE	= 2
    } 	BNSTATE;


enum SHELLBROWSERSHOWCONTROL
    {
        SBSC_HIDE	= 0,
        SBSC_SHOW	= 1,
        SBSC_TOGGLE	= 2,
        SBSC_QUERY	= 3
    } ;
//  values for IBrowserService::Get/SetFlags()
#define BSF_REGISTERASDROPTARGET   0x00000001
#define BSF_THEATERMODE            0x00000002
#define BSF_NOLOCALFILEWARNING     0x00000010
#define BSF_UISETBYAUTOMATION      0x00000100
#define BSF_RESIZABLE              0x00000200
#define BSF_CANMAXIMIZE            0x00000400
#define BSF_TOPBROWSER             0x00000800
#define BSF_NAVNOHISTORY           0x00001000
#define BSF_HTMLNAVCANCELED        0x00002000
#define BSF_DONTSHOWNAVCANCELPAGE  0x00004000
#define BSF_SETNAVIGATABLECODEPAGE 0x00008000
#define BSF_DELEGATEDNAVIGATION    0x00010000
#define BSF_TRUSTEDFORACTIVEX      0x00020000
#define BSF_MERGEDMENUS            0x00040000
#define BSF_FEEDNAVIGATION         0x00080000
#define BSF_FEEDSUBSCRIBED         0x00100000
#define HLNF_CALLERUNTRUSTED            0x00200000  // The navigate was possibly initiated by a web page by scripting code already on the system
#define HLNF_TRUSTEDFORACTIVEX          0x00400000  // The navigate should allow ActiveX prompts.
#define HLNF_DISABLEWINDOWRESTRICTIONS  0x00800000  // New window is created by url in zone that has the window restrictions security mitigation disabled
#define HLNF_TRUSTFIRSTDOWNLOAD         0x01000000  // New window is result of user initiated action.  Trust it if it immediatly trys to download.
#define HLNF_UNTRUSTEDFORDOWNLOAD       0x02000000  // Trident is navigating to an untrusted non-html file.  If we try to download it, do not.
#define SHHLNF_NOAUTOSELECT             0x04000000  // This navigate should not automatically select History ShellFolder
#define SHHLNF_WRITENOHISTORY           0x08000000  // This navigate should not go in the History ShellFolder
#define HLNF_EXTERNALNAVIGATE           0x10000000
#define HLNF_ALLOW_AUTONAVIGATE         0x20000000
#define HLNF_NEWWINDOWSMANAGED          0x80000000  // If this navigate results in a new window, it should be subject to popup manager.

#ifndef INTERNET_MAX_URL_LENGTH
#define INTERNET_MAX_PATH_LENGTH        2048
#define INTERNET_MAX_SCHEME_LENGTH      32          // longest protocol name length
#define INTERNET_MAX_URL_LENGTH         (INTERNET_MAX_SCHEME_LENGTH \
                                        + sizeof("://") \
                                        + INTERNET_MAX_PATH_LENGTH)
#endif


extern RPC_IF_HANDLE __MIDL_itf_shdeprecated_0000_0004_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_shdeprecated_0000_0004_v0_0_s_ifspec;

#ifndef __IBrowserService_INTERFACE_DEFINED__
#define __IBrowserService_INTERFACE_DEFINED__

/* interface IBrowserService */
/* [object][uuid] */ 


EXTERN_C const IID IID_IBrowserService;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("02ba3b52-0547-11d1-b833-00c04fc9b31f")
    IBrowserService : public IUnknown
    {
    public:
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE GetParentSite( 
            /* [annotation][out] */ 
            _Outptr_  IOleInPlaceSite **ppipsite) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE SetTitle( 
            /* [annotation][in] */ 
            _In_  IShellView *psv,
            /* [annotation][in] */ 
            _In_  LPCWSTR pszName) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE GetTitle( 
            /* [annotation][in] */ 
            _In_  IShellView *psv,
            /* [annotation][size_is][out] */ 
            _Out_writes_(cchName)  LPWSTR pszName,
            DWORD cchName) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE GetOleObject( 
            /* [annotation][out] */ 
            _Outptr_  IOleObject **ppobjv) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTravelLog( 
            /* [out] */ __RPC__deref_out_opt ITravelLog **pptl) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE ShowControlWindow( 
            UINT id,
            BOOL fShow) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE IsControlWindowShown( 
            UINT id,
            /* [annotation][out] */ 
            _Out_  BOOL *pfShown) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE IEGetDisplayName( 
            /* [annotation][in] */ 
            _In_  PCIDLIST_ABSOLUTE pidl,
            /* [annotation][out] */ 
            _Out_writes_(INTERNET_MAX_URL_LENGTH)  LPWSTR pwszName,
            UINT uFlags) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE IEParseDisplayName( 
            UINT uiCP,
            /* [annotation][in] */ 
            _In_  LPCWSTR pwszPath,
            /* [annotation][out] */ 
            _Outptr_  PIDLIST_ABSOLUTE *ppidlOut) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE DisplayParseError( 
            HRESULT hres,
            /* [annotation][in] */ 
            _In_  LPCWSTR pwszPath) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE NavigateToPidl( 
            /* [annotation][in] */ 
            _In_  PCIDLIST_ABSOLUTE pidl,
            DWORD grfHLNF) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE SetNavigateState( 
            BNSTATE bnstate) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE GetNavigateState( 
            /* [annotation][out] */ 
            _Out_  BNSTATE *pbnstate) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE NotifyRedirect( 
            /* [annotation][in] */ 
            _In_  IShellView *psv,
            /* [annotation][in] */ 
            _In_  PCIDLIST_ABSOLUTE pidl,
            /* [annotation][out] */ 
            _Out_  BOOL *pfDidBrowse) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE UpdateWindowList( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UpdateBackForwardState( void) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE SetFlags( 
            DWORD dwFlags,
            DWORD dwFlagMask) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE GetFlags( 
            /* [annotation][out] */ 
            _Out_  DWORD *pdwFlags) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE CanNavigateNow( void) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE GetPidl( 
            /* [annotation][out] */ 
            _Out_  PIDLIST_ABSOLUTE *ppidl) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE SetReferrer( 
            /* [annotation][in] */ 
            _In_  PCIDLIST_ABSOLUTE pidl) = 0;
        
        virtual /* [local] */ DWORD STDMETHODCALLTYPE GetBrowserIndex( void) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE GetBrowserByIndex( 
            DWORD dwID,
            /* [annotation][out] */ 
            _Outptr_  IUnknown **ppunk) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE GetHistoryObject( 
            /* [annotation][out] */ 
            _Outptr_  IOleObject **ppole,
            /* [annotation][out] */ 
            _Outptr_  IStream **pstm,
            /* [annotation][out] */ 
            _Outptr_  IBindCtx **ppbc) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE SetHistoryObject( 
            /* [annotation][in] */ 
            _In_  IOleObject *pole,
            BOOL fIsLocalAnchor) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE CacheOLEServer( 
            /* [annotation][in] */ 
            _In_  IOleObject *pole) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE GetSetCodePage( 
            /* [annotation][in] */ 
            _In_  VARIANT *pvarIn,
            /* [annotation][out] */ 
            _Out_  VARIANT *pvarOut) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE OnHttpEquiv( 
            /* [annotation][in] */ 
            _In_  IShellView *psv,
            BOOL fDone,
            /* [annotation][in] */ 
            _In_  VARIANT *pvarargIn,
            /* [annotation][out] */ 
            _Out_  VARIANT *pvarargOut) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE GetPalette( 
            /* [annotation][out] */ 
            _Out_  HPALETTE *hpal) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE RegisterWindow( 
            BOOL fForceRegister,
            int swc) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBrowserServiceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBrowserService * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBrowserService * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBrowserService * This);
        
        DECLSPEC_XFGVIRT(IBrowserService, GetParentSite)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetParentSite )( 
            IBrowserService * This,
            /* [annotation][out] */ 
            _Outptr_  IOleInPlaceSite **ppipsite);
        
        DECLSPEC_XFGVIRT(IBrowserService, SetTitle)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *SetTitle )( 
            IBrowserService * This,
            /* [annotation][in] */ 
            _In_  IShellView *psv,
            /* [annotation][in] */ 
            _In_  LPCWSTR pszName);
        
        DECLSPEC_XFGVIRT(IBrowserService, GetTitle)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetTitle )( 
            IBrowserService * This,
            /* [annotation][in] */ 
            _In_  IShellView *psv,
            /* [annotation][size_is][out] */ 
            _Out_writes_(cchName)  LPWSTR pszName,
            DWORD cchName);
        
        DECLSPEC_XFGVIRT(IBrowserService, GetOleObject)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetOleObject )( 
            IBrowserService * This,
            /* [annotation][out] */ 
            _Outptr_  IOleObject **ppobjv);
        
        DECLSPEC_XFGVIRT(IBrowserService, GetTravelLog)
        HRESULT ( STDMETHODCALLTYPE *GetTravelLog )( 
            __RPC__in IBrowserService * This,
            /* [out] */ __RPC__deref_out_opt ITravelLog **pptl);
        
        DECLSPEC_XFGVIRT(IBrowserService, ShowControlWindow)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *ShowControlWindow )( 
            IBrowserService * This,
            UINT id,
            BOOL fShow);
        
        DECLSPEC_XFGVIRT(IBrowserService, IsControlWindowShown)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *IsControlWindowShown )( 
            IBrowserService * This,
            UINT id,
            /* [annotation][out] */ 
            _Out_  BOOL *pfShown);
        
        DECLSPEC_XFGVIRT(IBrowserService, IEGetDisplayName)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *IEGetDisplayName )( 
            IBrowserService * This,
            /* [annotation][in] */ 
            _In_  PCIDLIST_ABSOLUTE pidl,
            /* [annotation][out] */ 
            _Out_writes_(INTERNET_MAX_URL_LENGTH)  LPWSTR pwszName,
            UINT uFlags);
        
        DECLSPEC_XFGVIRT(IBrowserService, IEParseDisplayName)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *IEParseDisplayName )( 
            IBrowserService * This,
            UINT uiCP,
            /* [annotation][in] */ 
            _In_  LPCWSTR pwszPath,
            /* [annotation][out] */ 
            _Outptr_  PIDLIST_ABSOLUTE *ppidlOut);
        
        DECLSPEC_XFGVIRT(IBrowserService, DisplayParseError)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *DisplayParseError )( 
            IBrowserService * This,
            HRESULT hres,
            /* [annotation][in] */ 
            _In_  LPCWSTR pwszPath);
        
        DECLSPEC_XFGVIRT(IBrowserService, NavigateToPidl)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *NavigateToPidl )( 
            IBrowserService * This,
            /* [annotation][in] */ 
            _In_  PCIDLIST_ABSOLUTE pidl,
            DWORD grfHLNF);
        
        DECLSPEC_XFGVIRT(IBrowserService, SetNavigateState)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *SetNavigateState )( 
            IBrowserService * This,
            BNSTATE bnstate);
        
        DECLSPEC_XFGVIRT(IBrowserService, GetNavigateState)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetNavigateState )( 
            IBrowserService * This,
            /* [annotation][out] */ 
            _Out_  BNSTATE *pbnstate);
        
        DECLSPEC_XFGVIRT(IBrowserService, NotifyRedirect)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *NotifyRedirect )( 
            IBrowserService * This,
            /* [annotation][in] */ 
            _In_  IShellView *psv,
            /* [annotation][in] */ 
            _In_  PCIDLIST_ABSOLUTE pidl,
            /* [annotation][out] */ 
            _Out_  BOOL *pfDidBrowse);
        
        DECLSPEC_XFGVIRT(IBrowserService, UpdateWindowList)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *UpdateWindowList )( 
            IBrowserService * This);
        
        DECLSPEC_XFGVIRT(IBrowserService, UpdateBackForwardState)
        HRESULT ( STDMETHODCALLTYPE *UpdateBackForwardState )( 
            __RPC__in IBrowserService * This);
        
        DECLSPEC_XFGVIRT(IBrowserService, SetFlags)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *SetFlags )( 
            IBrowserService * This,
            DWORD dwFlags,
            DWORD dwFlagMask);
        
        DECLSPEC_XFGVIRT(IBrowserService, GetFlags)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetFlags )( 
            IBrowserService * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwFlags);
        
        DECLSPEC_XFGVIRT(IBrowserService, CanNavigateNow)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *CanNavigateNow )( 
            IBrowserService * This);
        
        DECLSPEC_XFGVIRT(IBrowserService, GetPidl)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetPidl )( 
            IBrowserService * This,
            /* [annotation][out] */ 
            _Out_  PIDLIST_ABSOLUTE *ppidl);
        
        DECLSPEC_XFGVIRT(IBrowserService, SetReferrer)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *SetReferrer )( 
            IBrowserService * This,
            /* [annotation][in] */ 
            _In_  PCIDLIST_ABSOLUTE pidl);
        
        DECLSPEC_XFGVIRT(IBrowserService, GetBrowserIndex)
        /* [local] */ DWORD ( STDMETHODCALLTYPE *GetBrowserIndex )( 
            IBrowserService * This);
        
        DECLSPEC_XFGVIRT(IBrowserService, GetBrowserByIndex)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetBrowserByIndex )( 
            IBrowserService * This,
            DWORD dwID,
            /* [annotation][out] */ 
            _Outptr_  IUnknown **ppunk);
        
        DECLSPEC_XFGVIRT(IBrowserService, GetHistoryObject)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetHistoryObject )( 
            IBrowserService * This,
            /* [annotation][out] */ 
            _Outptr_  IOleObject **ppole,
            /* [annotation][out] */ 
            _Outptr_  IStream **pstm,
            /* [annotation][out] */ 
            _Outptr_  IBindCtx **ppbc);
        
        DECLSPEC_XFGVIRT(IBrowserService, SetHistoryObject)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *SetHistoryObject )( 
            IBrowserService * This,
            /* [annotation][in] */ 
            _In_  IOleObject *pole,
            BOOL fIsLocalAnchor);
        
        DECLSPEC_XFGVIRT(IBrowserService, CacheOLEServer)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *CacheOLEServer )( 
            IBrowserService * This,
            /* [annotation][in] */ 
            _In_  IOleObject *pole);
        
        DECLSPEC_XFGVIRT(IBrowserService, GetSetCodePage)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetSetCodePage )( 
            IBrowserService * This,
            /* [annotation][in] */ 
            _In_  VARIANT *pvarIn,
            /* [annotation][out] */ 
            _Out_  VARIANT *pvarOut);
        
        DECLSPEC_XFGVIRT(IBrowserService, OnHttpEquiv)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *OnHttpEquiv )( 
            IBrowserService * This,
            /* [annotation][in] */ 
            _In_  IShellView *psv,
            BOOL fDone,
            /* [annotation][in] */ 
            _In_  VARIANT *pvarargIn,
            /* [annotation][out] */ 
            _Out_  VARIANT *pvarargOut);
        
        DECLSPEC_XFGVIRT(IBrowserService, GetPalette)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetPalette )( 
            IBrowserService * This,
            /* [annotation][out] */ 
            _Out_  HPALETTE *hpal);
        
        DECLSPEC_XFGVIRT(IBrowserService, RegisterWindow)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *RegisterWindow )( 
            IBrowserService * This,
            BOOL fForceRegister,
            int swc);
        
        END_INTERFACE
    } IBrowserServiceVtbl;

    interface IBrowserService
    {
        CONST_VTBL struct IBrowserServiceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBrowserService_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBrowserService_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBrowserService_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBrowserService_GetParentSite(This,ppipsite)	\
    ( (This)->lpVtbl -> GetParentSite(This,ppipsite) ) 

#define IBrowserService_SetTitle(This,psv,pszName)	\
    ( (This)->lpVtbl -> SetTitle(This,psv,pszName) ) 

#define IBrowserService_GetTitle(This,psv,pszName,cchName)	\
    ( (This)->lpVtbl -> GetTitle(This,psv,pszName,cchName) ) 

#define IBrowserService_GetOleObject(This,ppobjv)	\
    ( (This)->lpVtbl -> GetOleObject(This,ppobjv) ) 

#define IBrowserService_GetTravelLog(This,pptl)	\
    ( (This)->lpVtbl -> GetTravelLog(This,pptl) ) 

#define IBrowserService_ShowControlWindow(This,id,fShow)	\
    ( (This)->lpVtbl -> ShowControlWindow(This,id,fShow) ) 

#define IBrowserService_IsControlWindowShown(This,id,pfShown)	\
    ( (This)->lpVtbl -> IsControlWindowShown(This,id,pfShown) ) 

#define IBrowserService_IEGetDisplayName(This,pidl,pwszName,uFlags)	\
    ( (This)->lpVtbl -> IEGetDisplayName(This,pidl,pwszName,uFlags) ) 

#define IBrowserService_IEParseDisplayName(This,uiCP,pwszPath,ppidlOut)	\
    ( (This)->lpVtbl -> IEParseDisplayName(This,uiCP,pwszPath,ppidlOut) ) 

#define IBrowserService_DisplayParseError(This,hres,pwszPath)	\
    ( (This)->lpVtbl -> DisplayParseError(This,hres,pwszPath) ) 

#define IBrowserService_NavigateToPidl(This,pidl,grfHLNF)	\
    ( (This)->lpVtbl -> NavigateToPidl(This,pidl,grfHLNF) ) 

#define IBrowserService_SetNavigateState(This,bnstate)	\
    ( (This)->lpVtbl -> SetNavigateState(This,bnstate) ) 

#define IBrowserService_GetNavigateState(This,pbnstate)	\
    ( (This)->lpVtbl -> GetNavigateState(This,pbnstate) ) 

#define IBrowserService_NotifyRedirect(This,psv,pidl,pfDidBrowse)	\
    ( (This)->lpVtbl -> NotifyRedirect(This,psv,pidl,pfDidBrowse) ) 

#define IBrowserService_UpdateWindowList(This)	\
    ( (This)->lpVtbl -> UpdateWindowList(This) ) 

#define IBrowserService_UpdateBackForwardState(This)	\
    ( (This)->lpVtbl -> UpdateBackForwardState(This) ) 

#define IBrowserService_SetFlags(This,dwFlags,dwFlagMask)	\
    ( (This)->lpVtbl -> SetFlags(This,dwFlags,dwFlagMask) ) 

#define IBrowserService_GetFlags(This,pdwFlags)	\
    ( (This)->lpVtbl -> GetFlags(This,pdwFlags) ) 

#define IBrowserService_CanNavigateNow(This)	\
    ( (This)->lpVtbl -> CanNavigateNow(This) ) 

#define IBrowserService_GetPidl(This,ppidl)	\
    ( (This)->lpVtbl -> GetPidl(This,ppidl) ) 

#define IBrowserService_SetReferrer(This,pidl)	\
    ( (This)->lpVtbl -> SetReferrer(This,pidl) ) 

#define IBrowserService_GetBrowserIndex(This)	\
    ( (This)->lpVtbl -> GetBrowserIndex(This) ) 

#define IBrowserService_GetBrowserByIndex(This,dwID,ppunk)	\
    ( (This)->lpVtbl -> GetBrowserByIndex(This,dwID,ppunk) ) 

#define IBrowserService_GetHistoryObject(This,ppole,pstm,ppbc)	\
    ( (This)->lpVtbl -> GetHistoryObject(This,ppole,pstm,ppbc) ) 

#define IBrowserService_SetHistoryObject(This,pole,fIsLocalAnchor)	\
    ( (This)->lpVtbl -> SetHistoryObject(This,pole,fIsLocalAnchor) ) 

#define IBrowserService_CacheOLEServer(This,pole)	\
    ( (This)->lpVtbl -> CacheOLEServer(This,pole) ) 

#define IBrowserService_GetSetCodePage(This,pvarIn,pvarOut)	\
    ( (This)->lpVtbl -> GetSetCodePage(This,pvarIn,pvarOut) ) 

#define IBrowserService_OnHttpEquiv(This,psv,fDone,pvarargIn,pvarargOut)	\
    ( (This)->lpVtbl -> OnHttpEquiv(This,psv,fDone,pvarargIn,pvarargOut) ) 

#define IBrowserService_GetPalette(This,hpal)	\
    ( (This)->lpVtbl -> GetPalette(This,hpal) ) 

#define IBrowserService_RegisterWindow(This,fForceRegister,swc)	\
    ( (This)->lpVtbl -> RegisterWindow(This,fForceRegister,swc) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBrowserService_INTERFACE_DEFINED__ */


#ifndef __IShellService_INTERFACE_DEFINED__
#define __IShellService_INTERFACE_DEFINED__

/* interface IShellService */
/* [local][object][uuid] */ 


EXTERN_C const IID IID_IShellService;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5836FB00-8187-11CF-A12B-00AA004AE837")
    IShellService : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetOwner( 
            /* [annotation][unique][in] */ 
            _In_opt_  IUnknown *punkOwner) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IShellServiceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IShellService * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IShellService * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IShellService * This);
        
        DECLSPEC_XFGVIRT(IShellService, SetOwner)
        HRESULT ( STDMETHODCALLTYPE *SetOwner )( 
            IShellService * This,
            /* [annotation][unique][in] */ 
            _In_opt_  IUnknown *punkOwner);
        
        END_INTERFACE
    } IShellServiceVtbl;

    interface IShellService
    {
        CONST_VTBL struct IShellServiceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IShellService_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IShellService_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IShellService_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IShellService_SetOwner(This,punkOwner)	\
    ( (This)->lpVtbl -> SetOwner(This,punkOwner) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IShellService_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_shdeprecated_0000_0006 */
/* [local] */ 


enum SECURELOCKCODE
    {
        SECURELOCK_NOCHANGE	= -1,
        SECURELOCK_SET_UNSECURE	= 0,
        SECURELOCK_SET_MIXED	= ( SECURELOCK_SET_UNSECURE + 1 ) ,
        SECURELOCK_SET_SECUREUNKNOWNBIT	= ( SECURELOCK_SET_MIXED + 1 ) ,
        SECURELOCK_SET_SECURE40BIT	= ( SECURELOCK_SET_SECUREUNKNOWNBIT + 1 ) ,
        SECURELOCK_SET_SECURE56BIT	= ( SECURELOCK_SET_SECURE40BIT + 1 ) ,
        SECURELOCK_SET_FORTEZZA	= ( SECURELOCK_SET_SECURE56BIT + 1 ) ,
        SECURELOCK_SET_SECURE128BIT	= ( SECURELOCK_SET_FORTEZZA + 1 ) ,
        SECURELOCK_FIRSTSUGGEST	= ( SECURELOCK_SET_SECURE128BIT + 1 ) ,
        SECURELOCK_SUGGEST_UNSECURE	= SECURELOCK_FIRSTSUGGEST,
        SECURELOCK_SUGGEST_MIXED	= ( SECURELOCK_SUGGEST_UNSECURE + 1 ) ,
        SECURELOCK_SUGGEST_SECUREUNKNOWNBIT	= ( SECURELOCK_SUGGEST_MIXED + 1 ) ,
        SECURELOCK_SUGGEST_SECURE40BIT	= ( SECURELOCK_SUGGEST_SECUREUNKNOWNBIT + 1 ) ,
        SECURELOCK_SUGGEST_SECURE56BIT	= ( SECURELOCK_SUGGEST_SECURE40BIT + 1 ) ,
        SECURELOCK_SUGGEST_FORTEZZA	= ( SECURELOCK_SUGGEST_SECURE56BIT + 1 ) ,
        SECURELOCK_SUGGEST_SECURE128BIT	= ( SECURELOCK_SUGGEST_FORTEZZA + 1 ) 
    } ;
#include <pshpack8.h>
typedef struct BASEBROWSERDATAXP
    {
    HWND _hwnd;
    ITravelLog *_ptl;
    IHlinkFrame *_phlf;
    IWebBrowser2 *_pautoWB2;
    IExpDispSupportXP *_pautoEDS;
    IShellService *_pautoSS;
    int _eSecureLockIcon;
    UINT _fCreatingViewWindow	: 1;
    UINT _uActivateState;
    PCIDLIST_ABSOLUTE _pidlViewState;
    IOleCommandTarget *_pctView;
    PIDLIST_ABSOLUTE _pidlCur;
    IShellView *_psv;
    IShellFolder *_psf;
    HWND _hwndView;
    LPWSTR _pszTitleCur;
    PIDLIST_ABSOLUTE _pidlPending;
    IShellView *_psvPending;
    IShellFolder *_psfPending;
    HWND _hwndViewPending;
    LPWSTR _pszTitlePending;
    BOOL _fIsViewMSHTML;
    BOOL _fPrivacyImpacted;
    CLSID _clsidView;
    CLSID _clsidViewPending;
    HWND _hwndFrame;
    } 	BASEBROWSERDATAXP;

typedef struct BASEBROWSERDATAXP *LPBASEBROWSERDATAXP;

typedef struct BASEBROWSERDATALH
    {
    HWND _hwnd;
    ITravelLog *_ptl;
    IHlinkFrame *_phlf;
    IWebBrowser2 *_pautoWB2;
    IExpDispSupport *_pautoEDS;
    IShellService *_pautoSS;
    int _eSecureLockIcon;
    UINT _fCreatingViewWindow	: 1;
    UINT _uActivateState;
    PCIDLIST_ABSOLUTE _pidlViewState;
    IOleCommandTarget *_pctView;
    PIDLIST_ABSOLUTE _pidlCur;
    IShellView *_psv;
    IShellFolder *_psf;
    HWND _hwndView;
    LPWSTR _pszTitleCur;
    PIDLIST_ABSOLUTE _pidlPending;
    IShellView *_psvPending;
    IShellFolder *_psfPending;
    HWND _hwndViewPending;
    LPWSTR _pszTitlePending;
    BOOL _fIsViewMSHTML;
    BOOL _fPrivacyImpacted;
    CLSID _clsidView;
    CLSID _clsidViewPending;
    HWND _hwndFrame;
    LONG _lPhishingFilterStatus;
    } 	BASEBROWSERDATALH;

typedef struct BASEBROWSERDATALH *LPBASEBROWSERDATALH;

#if 0
typedef BASEBROWSERDATALH BASEBROWSERDATA;

#endif
#if (NTDDI_VERSION >= NTDDI_VISTA) || (_WIN32_IE >= _WIN32_IE_IE70)
typedef BASEBROWSERDATALH BASEBROWSERDATA;
#else
typedef BASEBROWSERDATAXP BASEBROWSERDATA;
#endif // (NTDDI_VERSION >= NTDDI_VISTA) || (_WIN32_IE >= _WIN32_IE_IE70)
typedef const BASEBROWSERDATA *LPCBASEBROWSERDATA;

typedef BASEBROWSERDATA *LPBASEBROWSERDATA;

#define VIEW_PRIORITY_RESTRICTED    0x00000070  // a shell restriction is in place that forces this view to be the one we use
#define VIEW_PRIORITY_CACHEHIT      0x00000050  // we have registry info for the view, so the user has been there before
#define VIEW_PRIORITY_STALECACHEHIT 0x00000045  // we have stale registry info for the view, so we fall back to the 'all folders like this' default
#define VIEW_PRIORITY_USEASDEFAULT  0x00000043  // 'Use As Default' is more important than shellext, but less than a stale cache
#define VIEW_PRIORITY_SHELLEXT      0x00000040  // next we let the shell extension have its say
#define VIEW_PRIORITY_CACHEMISS     0x00000030  // if we have a cache miss, then we fall back to the 'all folders like this' default
#define VIEW_PRIORITY_INHERIT       0x00000020  // then try to inherit the view from the previous window
#define VIEW_PRIORITY_SHELLEXT_ASBACKUP 0x0015  // If classic view state is enabled we want the shell to default to an inheritted view
#define VIEW_PRIORITY_DESPERATE     0x00000010  // just pick the last view that the window supports
#define VIEW_PRIORITY_NONE          0x00000000  // dont have a view yet
typedef struct tagFolderSetData
    {
    FOLDERSETTINGS _fs;
    SHELLVIEWID _vidRestore;
    DWORD _dwViewPriority;
    } 	FOLDERSETDATA;

typedef struct tagFolderSetData *LPFOLDERSETDATA;

#if !defined(HMONITOR_DECLARED) && !defined(HMONITOR) && (WINVER < 0x0500)
#define HMONITOR_DECLARED
DECLARE_HANDLE(HMONITOR);
#endif
#if 0
typedef void *LPCREATESTRUCT;

typedef void NMHDR;

#endif
typedef struct SToolbarItem
    {
    IDockingWindow *ptbar;
    BORDERWIDTHS rcBorderTool;
    LPWSTR pwszItem;
    BOOL fShow;
    HMONITOR hMon;
    } 	TOOLBARITEM;

typedef struct SToolbarItem *LPTOOLBARITEM;

#define ITB_VIEW        ((UINT)-1)      // view
#include <poppack.h>


extern RPC_IF_HANDLE __MIDL_itf_shdeprecated_0000_0006_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_shdeprecated_0000_0006_v0_0_s_ifspec;

#ifndef __IBrowserService2_INTERFACE_DEFINED__
#define __IBrowserService2_INTERFACE_DEFINED__

/* interface IBrowserService2 */
/* [local][object][uuid] */ 


EXTERN_C const IID IID_IBrowserService2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("68BD21CC-438B-11d2-A560-00A0C92DBFE8")
    IBrowserService2 : public IBrowserService
    {
    public:
        virtual LRESULT STDMETHODCALLTYPE WndProcBS( 
            /* [annotation][in] */ 
            _In_  HWND hwnd,
            /* [annotation][in] */ 
            _In_  UINT uMsg,
            /* [annotation][in] */ 
            _In_  WPARAM wParam,
            /* [annotation][in] */ 
            _In_  LPARAM lParam) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetAsDefFolderSettings( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetViewRect( 
            /* [annotation][out] */ 
            _Out_  RECT *prc) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnSize( 
            /* [annotation][in] */ 
            _In_  WPARAM wParam) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnCreate( 
            /* [annotation][in] */ 
            _In_  struct tagCREATESTRUCTW *pcs) = 0;
        
        virtual LRESULT STDMETHODCALLTYPE OnCommand( 
            /* [annotation][in] */ 
            _In_  WPARAM wParam,
            /* [annotation][in] */ 
            _In_  LPARAM lParam) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnDestroy( void) = 0;
        
        virtual LRESULT STDMETHODCALLTYPE OnNotify( 
            /* [annotation][in] */ 
            _In_  struct tagNMHDR *pnm) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnSetFocus( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnFrameWindowActivateBS( 
            /* [annotation][in] */ 
            _In_  BOOL fActive) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ReleaseShellView( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ActivatePendingView( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateViewWindow( 
            /* [annotation][in] */ 
            _In_  IShellView *psvNew,
            /* [annotation][in] */ 
            _In_  IShellView *psvOld,
            /* [annotation][in] */ 
            _In_  LPRECT prcView,
            /* [annotation][out] */ 
            _Out_  HWND *phwnd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateBrowserPropSheetExt( 
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][iid_is][out] */ 
            _Outptr_  void **ppv) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetViewWindow( 
            /* [annotation][out] */ 
            _Out_  HWND *phwndView) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetBaseBrowserData( 
            /* [annotation][out] */ 
            _Outptr_  LPCBASEBROWSERDATA *pbbd) = 0;
        
        virtual LPBASEBROWSERDATA STDMETHODCALLTYPE PutBaseBrowserData( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InitializeTravelLog( 
            /* [annotation][in] */ 
            _In_  ITravelLog *ptl,
            /* [annotation][in] */ 
            _In_  DWORD dw) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetTopBrowser( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Offline( 
            /* [annotation][in] */ 
            _In_  int iCmd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AllowViewResize( 
            /* [annotation][in] */ 
            _In_  BOOL f) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetActivateState( 
            /* [annotation][in] */ 
            _In_  UINT u) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UpdateSecureLockIcon( 
            /* [annotation][in] */ 
            _In_  int eSecureLock) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InitializeDownloadManager( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InitializeTransitionSite( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE _Initialize( 
            /* [annotation][in] */ 
            _In_  HWND hwnd,
            /* [annotation][in] */ 
            _In_  IUnknown *pauto) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE _CancelPendingNavigationAsync( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE _CancelPendingView( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE _MaySaveChanges( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE _PauseOrResumeView( 
            BOOL fPaused) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE _DisableModeless( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE _NavigateToPidl( 
            /* [annotation][in] */ 
            _In_  PCIDLIST_ABSOLUTE pidl,
            /* [annotation][in] */ 
            _In_  DWORD grfHLNF,
            /* [annotation][in] */ 
            _In_  DWORD dwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE _TryShell2Rename( 
            /* [annotation][in] */ 
            _In_  IShellView *psv,
            /* [annotation][in] */ 
            _In_  PCIDLIST_ABSOLUTE pidlNew) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE _SwitchActivationNow( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE _ExecChildren( 
            /* [annotation][in] */ 
            _In_  IUnknown *punkBar,
            /* [annotation][in] */ 
            _In_  BOOL fBroadcast,
            /* [annotation][unique][in] */ 
            _In_opt_  const GUID *pguidCmdGroup,
            /* [annotation][in] */ 
            _In_  DWORD nCmdID,
            /* [annotation][in] */ 
            _In_  DWORD nCmdexecopt,
            /* [annotation][unique][in] */ 
            _In_opt_  VARIANTARG *pvarargIn,
            /* [annotation][unique][out][in] */ 
            _Inout_opt_  VARIANTARG *pvarargOut) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE _SendChildren( 
            /* [annotation][in] */ 
            _In_  HWND hwndBar,
            /* [annotation][in] */ 
            _In_  BOOL fBroadcast,
            /* [annotation][in] */ 
            _In_  UINT uMsg,
            /* [annotation][in] */ 
            _In_  WPARAM wParam,
            /* [annotation][in] */ 
            _In_  LPARAM lParam) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFolderSetData( 
            /* [out] */ struct tagFolderSetData *pfsd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE _OnFocusChange( 
            /* [annotation][in] */ 
            _In_  UINT itb) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE v_ShowHideChildWindows( 
            /* [annotation][in] */ 
            _In_  BOOL fChildOnly) = 0;
        
        virtual UINT STDMETHODCALLTYPE _get_itbLastFocus( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE _put_itbLastFocus( 
            /* [annotation][in] */ 
            _In_  UINT itbLastFocus) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE _UIActivateView( 
            /* [annotation][in] */ 
            _In_  UINT uState) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE _GetViewBorderRect( 
            /* [out] */ RECT *prc) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE _UpdateViewRectSize( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE _ResizeNextBorder( 
            /* [annotation][in] */ 
            _In_  UINT itb) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE _ResizeView( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE _GetEffectiveClientArea( 
            /* [annotation][out] */ 
            _Out_  LPRECT lprectBorder,
            /* [annotation][in] */ 
            _In_  HMONITOR hmon) = 0;
        
        virtual IStream *STDMETHODCALLTYPE v_GetViewStream( 
            /* [in] */ PCIDLIST_ABSOLUTE pidl,
            /* [annotation][in] */ 
            _In_  DWORD grfMode,
            /* [annotation][in] */ 
            _In_  LPCWSTR pwszName) = 0;
        
        virtual LRESULT STDMETHODCALLTYPE ForwardViewMsg( 
            /* [annotation][in] */ 
            _In_  UINT uMsg,
            /* [annotation][in] */ 
            _In_  WPARAM wParam,
            /* [annotation][in] */ 
            _In_  LPARAM lParam) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetAcceleratorMenu( 
            /* [annotation][in] */ 
            _In_  HACCEL hacc) = 0;
        
        virtual int STDMETHODCALLTYPE _GetToolbarCount( void) = 0;
        
        virtual LPTOOLBARITEM STDMETHODCALLTYPE _GetToolbarItem( 
            /* [annotation][in] */ 
            _In_  int itb) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE _SaveToolbars( 
            /* [in] */ IStream *pstm) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE _LoadToolbars( 
            /* [annotation][in] */ 
            _In_  IStream *pstm) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE _CloseAndReleaseToolbars( 
            BOOL fClose) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE v_MayGetNextToolbarFocus( 
            /* [annotation][in] */ 
            _In_  LPMSG lpMsg,
            /* [annotation][in] */ 
            _In_  UINT itbNext,
            /* [annotation][in] */ 
            _In_  int citb,
            /* [annotation][out] */ 
            _Out_  LPTOOLBARITEM *pptbi,
            /* [annotation][out] */ 
            _Out_  HWND *phwnd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE _ResizeNextBorderHelper( 
            UINT itb,
            BOOL bUseHmonitor) = 0;
        
        virtual UINT STDMETHODCALLTYPE _FindTBar( 
            /* [annotation][in] */ 
            _In_  IUnknown *punkSrc) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE _SetFocus( 
            /* [annotation][in] */ 
            _In_  LPTOOLBARITEM ptbi,
            /* [annotation][in] */ 
            _In_  HWND hwnd,
            /* [annotation][in] */ 
            _In_  LPMSG lpMsg) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE v_MayTranslateAccelerator( 
            /* [in] */ MSG *pmsg) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE _GetBorderDWHelper( 
            /* [annotation][in] */ 
            _In_  IUnknown *punkSrc,
            /* [annotation][out] */ 
            _Out_  LPRECT lprectBorder,
            /* [annotation][in] */ 
            _In_  BOOL bUseHmonitor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE v_CheckZoneCrossing( 
            /* [annotation][in] */ 
            _In_  PCIDLIST_ABSOLUTE pidl) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBrowserService2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IBrowserService2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IBrowserService2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IBrowserService2 * This);
        
        DECLSPEC_XFGVIRT(IBrowserService, GetParentSite)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetParentSite )( 
            IBrowserService2 * This,
            /* [annotation][out] */ 
            _Outptr_  IOleInPlaceSite **ppipsite);
        
        DECLSPEC_XFGVIRT(IBrowserService, SetTitle)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *SetTitle )( 
            IBrowserService2 * This,
            /* [annotation][in] */ 
            _In_  IShellView *psv,
            /* [annotation][in] */ 
            _In_  LPCWSTR pszName);
        
        DECLSPEC_XFGVIRT(IBrowserService, GetTitle)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetTitle )( 
            IBrowserService2 * This,
            /* [annotation][in] */ 
            _In_  IShellView *psv,
            /* [annotation][size_is][out] */ 
            _Out_writes_(cchName)  LPWSTR pszName,
            DWORD cchName);
        
        DECLSPEC_XFGVIRT(IBrowserService, GetOleObject)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetOleObject )( 
            IBrowserService2 * This,
            /* [annotation][out] */ 
            _Outptr_  IOleObject **ppobjv);
        
        DECLSPEC_XFGVIRT(IBrowserService, GetTravelLog)
        HRESULT ( STDMETHODCALLTYPE *GetTravelLog )( 
            IBrowserService2 * This,
            /* [out] */ ITravelLog **pptl);
        
        DECLSPEC_XFGVIRT(IBrowserService, ShowControlWindow)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *ShowControlWindow )( 
            IBrowserService2 * This,
            UINT id,
            BOOL fShow);
        
        DECLSPEC_XFGVIRT(IBrowserService, IsControlWindowShown)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *IsControlWindowShown )( 
            IBrowserService2 * This,
            UINT id,
            /* [annotation][out] */ 
            _Out_  BOOL *pfShown);
        
        DECLSPEC_XFGVIRT(IBrowserService, IEGetDisplayName)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *IEGetDisplayName )( 
            IBrowserService2 * This,
            /* [annotation][in] */ 
            _In_  PCIDLIST_ABSOLUTE pidl,
            /* [annotation][out] */ 
            _Out_writes_(INTERNET_MAX_URL_LENGTH)  LPWSTR pwszName,
            UINT uFlags);
        
        DECLSPEC_XFGVIRT(IBrowserService, IEParseDisplayName)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *IEParseDisplayName )( 
            IBrowserService2 * This,
            UINT uiCP,
            /* [annotation][in] */ 
            _In_  LPCWSTR pwszPath,
            /* [annotation][out] */ 
            _Outptr_  PIDLIST_ABSOLUTE *ppidlOut);
        
        DECLSPEC_XFGVIRT(IBrowserService, DisplayParseError)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *DisplayParseError )( 
            IBrowserService2 * This,
            HRESULT hres,
            /* [annotation][in] */ 
            _In_  LPCWSTR pwszPath);
        
        DECLSPEC_XFGVIRT(IBrowserService, NavigateToPidl)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *NavigateToPidl )( 
            IBrowserService2 * This,
            /* [annotation][in] */ 
            _In_  PCIDLIST_ABSOLUTE pidl,
            DWORD grfHLNF);
        
        DECLSPEC_XFGVIRT(IBrowserService, SetNavigateState)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *SetNavigateState )( 
            IBrowserService2 * This,
            BNSTATE bnstate);
        
        DECLSPEC_XFGVIRT(IBrowserService, GetNavigateState)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetNavigateState )( 
            IBrowserService2 * This,
            /* [annotation][out] */ 
            _Out_  BNSTATE *pbnstate);
        
        DECLSPEC_XFGVIRT(IBrowserService, NotifyRedirect)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *NotifyRedirect )( 
            IBrowserService2 * This,
            /* [annotation][in] */ 
            _In_  IShellView *psv,
            /* [annotation][in] */ 
            _In_  PCIDLIST_ABSOLUTE pidl,
            /* [annotation][out] */ 
            _Out_  BOOL *pfDidBrowse);
        
        DECLSPEC_XFGVIRT(IBrowserService, UpdateWindowList)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *UpdateWindowList )( 
            IBrowserService2 * This);
        
        DECLSPEC_XFGVIRT(IBrowserService, UpdateBackForwardState)
        HRESULT ( STDMETHODCALLTYPE *UpdateBackForwardState )( 
            IBrowserService2 * This);
        
        DECLSPEC_XFGVIRT(IBrowserService, SetFlags)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *SetFlags )( 
            IBrowserService2 * This,
            DWORD dwFlags,
            DWORD dwFlagMask);
        
        DECLSPEC_XFGVIRT(IBrowserService, GetFlags)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetFlags )( 
            IBrowserService2 * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwFlags);
        
        DECLSPEC_XFGVIRT(IBrowserService, CanNavigateNow)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *CanNavigateNow )( 
            IBrowserService2 * This);
        
        DECLSPEC_XFGVIRT(IBrowserService, GetPidl)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetPidl )( 
            IBrowserService2 * This,
            /* [annotation][out] */ 
            _Out_  PIDLIST_ABSOLUTE *ppidl);
        
        DECLSPEC_XFGVIRT(IBrowserService, SetReferrer)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *SetReferrer )( 
            IBrowserService2 * This,
            /* [annotation][in] */ 
            _In_  PCIDLIST_ABSOLUTE pidl);
        
        DECLSPEC_XFGVIRT(IBrowserService, GetBrowserIndex)
        /* [local] */ DWORD ( STDMETHODCALLTYPE *GetBrowserIndex )( 
            IBrowserService2 * This);
        
        DECLSPEC_XFGVIRT(IBrowserService, GetBrowserByIndex)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetBrowserByIndex )( 
            IBrowserService2 * This,
            DWORD dwID,
            /* [annotation][out] */ 
            _Outptr_  IUnknown **ppunk);
        
        DECLSPEC_XFGVIRT(IBrowserService, GetHistoryObject)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetHistoryObject )( 
            IBrowserService2 * This,
            /* [annotation][out] */ 
            _Outptr_  IOleObject **ppole,
            /* [annotation][out] */ 
            _Outptr_  IStream **pstm,
            /* [annotation][out] */ 
            _Outptr_  IBindCtx **ppbc);
        
        DECLSPEC_XFGVIRT(IBrowserService, SetHistoryObject)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *SetHistoryObject )( 
            IBrowserService2 * This,
            /* [annotation][in] */ 
            _In_  IOleObject *pole,
            BOOL fIsLocalAnchor);
        
        DECLSPEC_XFGVIRT(IBrowserService, CacheOLEServer)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *CacheOLEServer )( 
            IBrowserService2 * This,
            /* [annotation][in] */ 
            _In_  IOleObject *pole);
        
        DECLSPEC_XFGVIRT(IBrowserService, GetSetCodePage)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetSetCodePage )( 
            IBrowserService2 * This,
            /* [annotation][in] */ 
            _In_  VARIANT *pvarIn,
            /* [annotation][out] */ 
            _Out_  VARIANT *pvarOut);
        
        DECLSPEC_XFGVIRT(IBrowserService, OnHttpEquiv)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *OnHttpEquiv )( 
            IBrowserService2 * This,
            /* [annotation][in] */ 
            _In_  IShellView *psv,
            BOOL fDone,
            /* [annotation][in] */ 
            _In_  VARIANT *pvarargIn,
            /* [annotation][out] */ 
            _Out_  VARIANT *pvarargOut);
        
        DECLSPEC_XFGVIRT(IBrowserService, GetPalette)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetPalette )( 
            IBrowserService2 * This,
            /* [annotation][out] */ 
            _Out_  HPALETTE *hpal);
        
        DECLSPEC_XFGVIRT(IBrowserService, RegisterWindow)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *RegisterWindow )( 
            IBrowserService2 * This,
            BOOL fForceRegister,
            int swc);
        
        DECLSPEC_XFGVIRT(IBrowserService2, WndProcBS)
        LRESULT ( STDMETHODCALLTYPE *WndProcBS )( 
            IBrowserService2 * This,
            /* [annotation][in] */ 
            _In_  HWND hwnd,
            /* [annotation][in] */ 
            _In_  UINT uMsg,
            /* [annotation][in] */ 
            _In_  WPARAM wParam,
            /* [annotation][in] */ 
            _In_  LPARAM lParam);
        
        DECLSPEC_XFGVIRT(IBrowserService2, SetAsDefFolderSettings)
        HRESULT ( STDMETHODCALLTYPE *SetAsDefFolderSettings )( 
            IBrowserService2 * This);
        
        DECLSPEC_XFGVIRT(IBrowserService2, GetViewRect)
        HRESULT ( STDMETHODCALLTYPE *GetViewRect )( 
            IBrowserService2 * This,
            /* [annotation][out] */ 
            _Out_  RECT *prc);
        
        DECLSPEC_XFGVIRT(IBrowserService2, OnSize)
        HRESULT ( STDMETHODCALLTYPE *OnSize )( 
            IBrowserService2 * This,
            /* [annotation][in] */ 
            _In_  WPARAM wParam);
        
        DECLSPEC_XFGVIRT(IBrowserService2, OnCreate)
        HRESULT ( STDMETHODCALLTYPE *OnCreate )( 
            IBrowserService2 * This,
            /* [annotation][in] */ 
            _In_  struct tagCREATESTRUCTW *pcs);
        
        DECLSPEC_XFGVIRT(IBrowserService2, OnCommand)
        LRESULT ( STDMETHODCALLTYPE *OnCommand )( 
            IBrowserService2 * This,
            /* [annotation][in] */ 
            _In_  WPARAM wParam,
            /* [annotation][in] */ 
            _In_  LPARAM lParam);
        
        DECLSPEC_XFGVIRT(IBrowserService2, OnDestroy)
        HRESULT ( STDMETHODCALLTYPE *OnDestroy )( 
            IBrowserService2 * This);
        
        DECLSPEC_XFGVIRT(IBrowserService2, OnNotify)
        LRESULT ( STDMETHODCALLTYPE *OnNotify )( 
            IBrowserService2 * This,
            /* [annotation][in] */ 
            _In_  struct tagNMHDR *pnm);
        
        DECLSPEC_XFGVIRT(IBrowserService2, OnSetFocus)
        HRESULT ( STDMETHODCALLTYPE *OnSetFocus )( 
            IBrowserService2 * This);
        
        DECLSPEC_XFGVIRT(IBrowserService2, OnFrameWindowActivateBS)
        HRESULT ( STDMETHODCALLTYPE *OnFrameWindowActivateBS )( 
            IBrowserService2 * This,
            /* [annotation][in] */ 
            _In_  BOOL fActive);
        
        DECLSPEC_XFGVIRT(IBrowserService2, ReleaseShellView)
        HRESULT ( STDMETHODCALLTYPE *ReleaseShellView )( 
            IBrowserService2 * This);
        
        DECLSPEC_XFGVIRT(IBrowserService2, ActivatePendingView)
        HRESULT ( STDMETHODCALLTYPE *ActivatePendingView )( 
            IBrowserService2 * This);
        
        DECLSPEC_XFGVIRT(IBrowserService2, CreateViewWindow)
        HRESULT ( STDMETHODCALLTYPE *CreateViewWindow )( 
            IBrowserService2 * This,
            /* [annotation][in] */ 
            _In_  IShellView *psvNew,
            /* [annotation][in] */ 
            _In_  IShellView *psvOld,
            /* [annotation][in] */ 
            _In_  LPRECT prcView,
            /* [annotation][out] */ 
            _Out_  HWND *phwnd);
        
        DECLSPEC_XFGVIRT(IBrowserService2, CreateBrowserPropSheetExt)
        HRESULT ( STDMETHODCALLTYPE *CreateBrowserPropSheetExt )( 
            IBrowserService2 * This,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][iid_is][out] */ 
            _Outptr_  void **ppv);
        
        DECLSPEC_XFGVIRT(IBrowserService2, GetViewWindow)
        HRESULT ( STDMETHODCALLTYPE *GetViewWindow )( 
            IBrowserService2 * This,
            /* [annotation][out] */ 
            _Out_  HWND *phwndView);
        
        DECLSPEC_XFGVIRT(IBrowserService2, GetBaseBrowserData)
        HRESULT ( STDMETHODCALLTYPE *GetBaseBrowserData )( 
            IBrowserService2 * This,
            /* [annotation][out] */ 
            _Outptr_  LPCBASEBROWSERDATA *pbbd);
        
        DECLSPEC_XFGVIRT(IBrowserService2, PutBaseBrowserData)
        LPBASEBROWSERDATA ( STDMETHODCALLTYPE *PutBaseBrowserData )( 
            IBrowserService2 * This);
        
        DECLSPEC_XFGVIRT(IBrowserService2, InitializeTravelLog)
        HRESULT ( STDMETHODCALLTYPE *InitializeTravelLog )( 
            IBrowserService2 * This,
            /* [annotation][in] */ 
            _In_  ITravelLog *ptl,
            /* [annotation][in] */ 
            _In_  DWORD dw);
        
        DECLSPEC_XFGVIRT(IBrowserService2, SetTopBrowser)
        HRESULT ( STDMETHODCALLTYPE *SetTopBrowser )( 
            IBrowserService2 * This);
        
        DECLSPEC_XFGVIRT(IBrowserService2, Offline)
        HRESULT ( STDMETHODCALLTYPE *Offline )( 
            IBrowserService2 * This,
            /* [annotation][in] */ 
            _In_  int iCmd);
        
        DECLSPEC_XFGVIRT(IBrowserService2, AllowViewResize)
        HRESULT ( STDMETHODCALLTYPE *AllowViewResize )( 
            IBrowserService2 * This,
            /* [annotation][in] */ 
            _In_  BOOL f);
        
        DECLSPEC_XFGVIRT(IBrowserService2, SetActivateState)
        HRESULT ( STDMETHODCALLTYPE *SetActivateState )( 
            IBrowserService2 * This,
            /* [annotation][in] */ 
            _In_  UINT u);
        
        DECLSPEC_XFGVIRT(IBrowserService2, UpdateSecureLockIcon)
        HRESULT ( STDMETHODCALLTYPE *UpdateSecureLockIcon )( 
            IBrowserService2 * This,
            /* [annotation][in] */ 
            _In_  int eSecureLock);
        
        DECLSPEC_XFGVIRT(IBrowserService2, InitializeDownloadManager)
        HRESULT ( STDMETHODCALLTYPE *InitializeDownloadManager )( 
            IBrowserService2 * This);
        
        DECLSPEC_XFGVIRT(IBrowserService2, InitializeTransitionSite)
        HRESULT ( STDMETHODCALLTYPE *InitializeTransitionSite )( 
            IBrowserService2 * This);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _Initialize)
        HRESULT ( STDMETHODCALLTYPE *_Initialize )( 
            IBrowserService2 * This,
            /* [annotation][in] */ 
            _In_  HWND hwnd,
            /* [annotation][in] */ 
            _In_  IUnknown *pauto);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _CancelPendingNavigationAsync)
        HRESULT ( STDMETHODCALLTYPE *_CancelPendingNavigationAsync )( 
            IBrowserService2 * This);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _CancelPendingView)
        HRESULT ( STDMETHODCALLTYPE *_CancelPendingView )( 
            IBrowserService2 * This);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _MaySaveChanges)
        HRESULT ( STDMETHODCALLTYPE *_MaySaveChanges )( 
            IBrowserService2 * This);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _PauseOrResumeView)
        HRESULT ( STDMETHODCALLTYPE *_PauseOrResumeView )( 
            IBrowserService2 * This,
            BOOL fPaused);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _DisableModeless)
        HRESULT ( STDMETHODCALLTYPE *_DisableModeless )( 
            IBrowserService2 * This);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _NavigateToPidl)
        HRESULT ( STDMETHODCALLTYPE *_NavigateToPidl )( 
            IBrowserService2 * This,
            /* [annotation][in] */ 
            _In_  PCIDLIST_ABSOLUTE pidl,
            /* [annotation][in] */ 
            _In_  DWORD grfHLNF,
            /* [annotation][in] */ 
            _In_  DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _TryShell2Rename)
        HRESULT ( STDMETHODCALLTYPE *_TryShell2Rename )( 
            IBrowserService2 * This,
            /* [annotation][in] */ 
            _In_  IShellView *psv,
            /* [annotation][in] */ 
            _In_  PCIDLIST_ABSOLUTE pidlNew);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _SwitchActivationNow)
        HRESULT ( STDMETHODCALLTYPE *_SwitchActivationNow )( 
            IBrowserService2 * This);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _ExecChildren)
        HRESULT ( STDMETHODCALLTYPE *_ExecChildren )( 
            IBrowserService2 * This,
            /* [annotation][in] */ 
            _In_  IUnknown *punkBar,
            /* [annotation][in] */ 
            _In_  BOOL fBroadcast,
            /* [annotation][unique][in] */ 
            _In_opt_  const GUID *pguidCmdGroup,
            /* [annotation][in] */ 
            _In_  DWORD nCmdID,
            /* [annotation][in] */ 
            _In_  DWORD nCmdexecopt,
            /* [annotation][unique][in] */ 
            _In_opt_  VARIANTARG *pvarargIn,
            /* [annotation][unique][out][in] */ 
            _Inout_opt_  VARIANTARG *pvarargOut);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _SendChildren)
        HRESULT ( STDMETHODCALLTYPE *_SendChildren )( 
            IBrowserService2 * This,
            /* [annotation][in] */ 
            _In_  HWND hwndBar,
            /* [annotation][in] */ 
            _In_  BOOL fBroadcast,
            /* [annotation][in] */ 
            _In_  UINT uMsg,
            /* [annotation][in] */ 
            _In_  WPARAM wParam,
            /* [annotation][in] */ 
            _In_  LPARAM lParam);
        
        DECLSPEC_XFGVIRT(IBrowserService2, GetFolderSetData)
        HRESULT ( STDMETHODCALLTYPE *GetFolderSetData )( 
            IBrowserService2 * This,
            /* [out] */ struct tagFolderSetData *pfsd);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _OnFocusChange)
        HRESULT ( STDMETHODCALLTYPE *_OnFocusChange )( 
            IBrowserService2 * This,
            /* [annotation][in] */ 
            _In_  UINT itb);
        
        DECLSPEC_XFGVIRT(IBrowserService2, v_ShowHideChildWindows)
        HRESULT ( STDMETHODCALLTYPE *v_ShowHideChildWindows )( 
            IBrowserService2 * This,
            /* [annotation][in] */ 
            _In_  BOOL fChildOnly);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _get_itbLastFocus)
        UINT ( STDMETHODCALLTYPE *_get_itbLastFocus )( 
            IBrowserService2 * This);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _put_itbLastFocus)
        HRESULT ( STDMETHODCALLTYPE *_put_itbLastFocus )( 
            IBrowserService2 * This,
            /* [annotation][in] */ 
            _In_  UINT itbLastFocus);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _UIActivateView)
        HRESULT ( STDMETHODCALLTYPE *_UIActivateView )( 
            IBrowserService2 * This,
            /* [annotation][in] */ 
            _In_  UINT uState);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _GetViewBorderRect)
        HRESULT ( STDMETHODCALLTYPE *_GetViewBorderRect )( 
            IBrowserService2 * This,
            /* [out] */ RECT *prc);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _UpdateViewRectSize)
        HRESULT ( STDMETHODCALLTYPE *_UpdateViewRectSize )( 
            IBrowserService2 * This);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _ResizeNextBorder)
        HRESULT ( STDMETHODCALLTYPE *_ResizeNextBorder )( 
            IBrowserService2 * This,
            /* [annotation][in] */ 
            _In_  UINT itb);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _ResizeView)
        HRESULT ( STDMETHODCALLTYPE *_ResizeView )( 
            IBrowserService2 * This);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _GetEffectiveClientArea)
        HRESULT ( STDMETHODCALLTYPE *_GetEffectiveClientArea )( 
            IBrowserService2 * This,
            /* [annotation][out] */ 
            _Out_  LPRECT lprectBorder,
            /* [annotation][in] */ 
            _In_  HMONITOR hmon);
        
        DECLSPEC_XFGVIRT(IBrowserService2, v_GetViewStream)
        IStream *( STDMETHODCALLTYPE *v_GetViewStream )( 
            IBrowserService2 * This,
            /* [in] */ PCIDLIST_ABSOLUTE pidl,
            /* [annotation][in] */ 
            _In_  DWORD grfMode,
            /* [annotation][in] */ 
            _In_  LPCWSTR pwszName);
        
        DECLSPEC_XFGVIRT(IBrowserService2, ForwardViewMsg)
        LRESULT ( STDMETHODCALLTYPE *ForwardViewMsg )( 
            IBrowserService2 * This,
            /* [annotation][in] */ 
            _In_  UINT uMsg,
            /* [annotation][in] */ 
            _In_  WPARAM wParam,
            /* [annotation][in] */ 
            _In_  LPARAM lParam);
        
        DECLSPEC_XFGVIRT(IBrowserService2, SetAcceleratorMenu)
        HRESULT ( STDMETHODCALLTYPE *SetAcceleratorMenu )( 
            IBrowserService2 * This,
            /* [annotation][in] */ 
            _In_  HACCEL hacc);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _GetToolbarCount)
        int ( STDMETHODCALLTYPE *_GetToolbarCount )( 
            IBrowserService2 * This);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _GetToolbarItem)
        LPTOOLBARITEM ( STDMETHODCALLTYPE *_GetToolbarItem )( 
            IBrowserService2 * This,
            /* [annotation][in] */ 
            _In_  int itb);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _SaveToolbars)
        HRESULT ( STDMETHODCALLTYPE *_SaveToolbars )( 
            IBrowserService2 * This,
            /* [in] */ IStream *pstm);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _LoadToolbars)
        HRESULT ( STDMETHODCALLTYPE *_LoadToolbars )( 
            IBrowserService2 * This,
            /* [annotation][in] */ 
            _In_  IStream *pstm);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _CloseAndReleaseToolbars)
        HRESULT ( STDMETHODCALLTYPE *_CloseAndReleaseToolbars )( 
            IBrowserService2 * This,
            BOOL fClose);
        
        DECLSPEC_XFGVIRT(IBrowserService2, v_MayGetNextToolbarFocus)
        HRESULT ( STDMETHODCALLTYPE *v_MayGetNextToolbarFocus )( 
            IBrowserService2 * This,
            /* [annotation][in] */ 
            _In_  LPMSG lpMsg,
            /* [annotation][in] */ 
            _In_  UINT itbNext,
            /* [annotation][in] */ 
            _In_  int citb,
            /* [annotation][out] */ 
            _Out_  LPTOOLBARITEM *pptbi,
            /* [annotation][out] */ 
            _Out_  HWND *phwnd);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _ResizeNextBorderHelper)
        HRESULT ( STDMETHODCALLTYPE *_ResizeNextBorderHelper )( 
            IBrowserService2 * This,
            UINT itb,
            BOOL bUseHmonitor);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _FindTBar)
        UINT ( STDMETHODCALLTYPE *_FindTBar )( 
            IBrowserService2 * This,
            /* [annotation][in] */ 
            _In_  IUnknown *punkSrc);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _SetFocus)
        HRESULT ( STDMETHODCALLTYPE *_SetFocus )( 
            IBrowserService2 * This,
            /* [annotation][in] */ 
            _In_  LPTOOLBARITEM ptbi,
            /* [annotation][in] */ 
            _In_  HWND hwnd,
            /* [annotation][in] */ 
            _In_  LPMSG lpMsg);
        
        DECLSPEC_XFGVIRT(IBrowserService2, v_MayTranslateAccelerator)
        HRESULT ( STDMETHODCALLTYPE *v_MayTranslateAccelerator )( 
            IBrowserService2 * This,
            /* [in] */ MSG *pmsg);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _GetBorderDWHelper)
        HRESULT ( STDMETHODCALLTYPE *_GetBorderDWHelper )( 
            IBrowserService2 * This,
            /* [annotation][in] */ 
            _In_  IUnknown *punkSrc,
            /* [annotation][out] */ 
            _Out_  LPRECT lprectBorder,
            /* [annotation][in] */ 
            _In_  BOOL bUseHmonitor);
        
        DECLSPEC_XFGVIRT(IBrowserService2, v_CheckZoneCrossing)
        HRESULT ( STDMETHODCALLTYPE *v_CheckZoneCrossing )( 
            IBrowserService2 * This,
            /* [annotation][in] */ 
            _In_  PCIDLIST_ABSOLUTE pidl);
        
        END_INTERFACE
    } IBrowserService2Vtbl;

    interface IBrowserService2
    {
        CONST_VTBL struct IBrowserService2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBrowserService2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBrowserService2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBrowserService2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBrowserService2_GetParentSite(This,ppipsite)	\
    ( (This)->lpVtbl -> GetParentSite(This,ppipsite) ) 

#define IBrowserService2_SetTitle(This,psv,pszName)	\
    ( (This)->lpVtbl -> SetTitle(This,psv,pszName) ) 

#define IBrowserService2_GetTitle(This,psv,pszName,cchName)	\
    ( (This)->lpVtbl -> GetTitle(This,psv,pszName,cchName) ) 

#define IBrowserService2_GetOleObject(This,ppobjv)	\
    ( (This)->lpVtbl -> GetOleObject(This,ppobjv) ) 

#define IBrowserService2_GetTravelLog(This,pptl)	\
    ( (This)->lpVtbl -> GetTravelLog(This,pptl) ) 

#define IBrowserService2_ShowControlWindow(This,id,fShow)	\
    ( (This)->lpVtbl -> ShowControlWindow(This,id,fShow) ) 

#define IBrowserService2_IsControlWindowShown(This,id,pfShown)	\
    ( (This)->lpVtbl -> IsControlWindowShown(This,id,pfShown) ) 

#define IBrowserService2_IEGetDisplayName(This,pidl,pwszName,uFlags)	\
    ( (This)->lpVtbl -> IEGetDisplayName(This,pidl,pwszName,uFlags) ) 

#define IBrowserService2_IEParseDisplayName(This,uiCP,pwszPath,ppidlOut)	\
    ( (This)->lpVtbl -> IEParseDisplayName(This,uiCP,pwszPath,ppidlOut) ) 

#define IBrowserService2_DisplayParseError(This,hres,pwszPath)	\
    ( (This)->lpVtbl -> DisplayParseError(This,hres,pwszPath) ) 

#define IBrowserService2_NavigateToPidl(This,pidl,grfHLNF)	\
    ( (This)->lpVtbl -> NavigateToPidl(This,pidl,grfHLNF) ) 

#define IBrowserService2_SetNavigateState(This,bnstate)	\
    ( (This)->lpVtbl -> SetNavigateState(This,bnstate) ) 

#define IBrowserService2_GetNavigateState(This,pbnstate)	\
    ( (This)->lpVtbl -> GetNavigateState(This,pbnstate) ) 

#define IBrowserService2_NotifyRedirect(This,psv,pidl,pfDidBrowse)	\
    ( (This)->lpVtbl -> NotifyRedirect(This,psv,pidl,pfDidBrowse) ) 

#define IBrowserService2_UpdateWindowList(This)	\
    ( (This)->lpVtbl -> UpdateWindowList(This) ) 

#define IBrowserService2_UpdateBackForwardState(This)	\
    ( (This)->lpVtbl -> UpdateBackForwardState(This) ) 

#define IBrowserService2_SetFlags(This,dwFlags,dwFlagMask)	\
    ( (This)->lpVtbl -> SetFlags(This,dwFlags,dwFlagMask) ) 

#define IBrowserService2_GetFlags(This,pdwFlags)	\
    ( (This)->lpVtbl -> GetFlags(This,pdwFlags) ) 

#define IBrowserService2_CanNavigateNow(This)	\
    ( (This)->lpVtbl -> CanNavigateNow(This) ) 

#define IBrowserService2_GetPidl(This,ppidl)	\
    ( (This)->lpVtbl -> GetPidl(This,ppidl) ) 

#define IBrowserService2_SetReferrer(This,pidl)	\
    ( (This)->lpVtbl -> SetReferrer(This,pidl) ) 

#define IBrowserService2_GetBrowserIndex(This)	\
    ( (This)->lpVtbl -> GetBrowserIndex(This) ) 

#define IBrowserService2_GetBrowserByIndex(This,dwID,ppunk)	\
    ( (This)->lpVtbl -> GetBrowserByIndex(This,dwID,ppunk) ) 

#define IBrowserService2_GetHistoryObject(This,ppole,pstm,ppbc)	\
    ( (This)->lpVtbl -> GetHistoryObject(This,ppole,pstm,ppbc) ) 

#define IBrowserService2_SetHistoryObject(This,pole,fIsLocalAnchor)	\
    ( (This)->lpVtbl -> SetHistoryObject(This,pole,fIsLocalAnchor) ) 

#define IBrowserService2_CacheOLEServer(This,pole)	\
    ( (This)->lpVtbl -> CacheOLEServer(This,pole) ) 

#define IBrowserService2_GetSetCodePage(This,pvarIn,pvarOut)	\
    ( (This)->lpVtbl -> GetSetCodePage(This,pvarIn,pvarOut) ) 

#define IBrowserService2_OnHttpEquiv(This,psv,fDone,pvarargIn,pvarargOut)	\
    ( (This)->lpVtbl -> OnHttpEquiv(This,psv,fDone,pvarargIn,pvarargOut) ) 

#define IBrowserService2_GetPalette(This,hpal)	\
    ( (This)->lpVtbl -> GetPalette(This,hpal) ) 

#define IBrowserService2_RegisterWindow(This,fForceRegister,swc)	\
    ( (This)->lpVtbl -> RegisterWindow(This,fForceRegister,swc) ) 


#define IBrowserService2_WndProcBS(This,hwnd,uMsg,wParam,lParam)	\
    ( (This)->lpVtbl -> WndProcBS(This,hwnd,uMsg,wParam,lParam) ) 

#define IBrowserService2_SetAsDefFolderSettings(This)	\
    ( (This)->lpVtbl -> SetAsDefFolderSettings(This) ) 

#define IBrowserService2_GetViewRect(This,prc)	\
    ( (This)->lpVtbl -> GetViewRect(This,prc) ) 

#define IBrowserService2_OnSize(This,wParam)	\
    ( (This)->lpVtbl -> OnSize(This,wParam) ) 

#define IBrowserService2_OnCreate(This,pcs)	\
    ( (This)->lpVtbl -> OnCreate(This,pcs) ) 

#define IBrowserService2_OnCommand(This,wParam,lParam)	\
    ( (This)->lpVtbl -> OnCommand(This,wParam,lParam) ) 

#define IBrowserService2_OnDestroy(This)	\
    ( (This)->lpVtbl -> OnDestroy(This) ) 

#define IBrowserService2_OnNotify(This,pnm)	\
    ( (This)->lpVtbl -> OnNotify(This,pnm) ) 

#define IBrowserService2_OnSetFocus(This)	\
    ( (This)->lpVtbl -> OnSetFocus(This) ) 

#define IBrowserService2_OnFrameWindowActivateBS(This,fActive)	\
    ( (This)->lpVtbl -> OnFrameWindowActivateBS(This,fActive) ) 

#define IBrowserService2_ReleaseShellView(This)	\
    ( (This)->lpVtbl -> ReleaseShellView(This) ) 

#define IBrowserService2_ActivatePendingView(This)	\
    ( (This)->lpVtbl -> ActivatePendingView(This) ) 

#define IBrowserService2_CreateViewWindow(This,psvNew,psvOld,prcView,phwnd)	\
    ( (This)->lpVtbl -> CreateViewWindow(This,psvNew,psvOld,prcView,phwnd) ) 

#define IBrowserService2_CreateBrowserPropSheetExt(This,riid,ppv)	\
    ( (This)->lpVtbl -> CreateBrowserPropSheetExt(This,riid,ppv) ) 

#define IBrowserService2_GetViewWindow(This,phwndView)	\
    ( (This)->lpVtbl -> GetViewWindow(This,phwndView) ) 

#define IBrowserService2_GetBaseBrowserData(This,pbbd)	\
    ( (This)->lpVtbl -> GetBaseBrowserData(This,pbbd) ) 

#define IBrowserService2_PutBaseBrowserData(This)	\
    ( (This)->lpVtbl -> PutBaseBrowserData(This) ) 

#define IBrowserService2_InitializeTravelLog(This,ptl,dw)	\
    ( (This)->lpVtbl -> InitializeTravelLog(This,ptl,dw) ) 

#define IBrowserService2_SetTopBrowser(This)	\
    ( (This)->lpVtbl -> SetTopBrowser(This) ) 

#define IBrowserService2_Offline(This,iCmd)	\
    ( (This)->lpVtbl -> Offline(This,iCmd) ) 

#define IBrowserService2_AllowViewResize(This,f)	\
    ( (This)->lpVtbl -> AllowViewResize(This,f) ) 

#define IBrowserService2_SetActivateState(This,u)	\
    ( (This)->lpVtbl -> SetActivateState(This,u) ) 

#define IBrowserService2_UpdateSecureLockIcon(This,eSecureLock)	\
    ( (This)->lpVtbl -> UpdateSecureLockIcon(This,eSecureLock) ) 

#define IBrowserService2_InitializeDownloadManager(This)	\
    ( (This)->lpVtbl -> InitializeDownloadManager(This) ) 

#define IBrowserService2_InitializeTransitionSite(This)	\
    ( (This)->lpVtbl -> InitializeTransitionSite(This) ) 

#define IBrowserService2__Initialize(This,hwnd,pauto)	\
    ( (This)->lpVtbl -> _Initialize(This,hwnd,pauto) ) 

#define IBrowserService2__CancelPendingNavigationAsync(This)	\
    ( (This)->lpVtbl -> _CancelPendingNavigationAsync(This) ) 

#define IBrowserService2__CancelPendingView(This)	\
    ( (This)->lpVtbl -> _CancelPendingView(This) ) 

#define IBrowserService2__MaySaveChanges(This)	\
    ( (This)->lpVtbl -> _MaySaveChanges(This) ) 

#define IBrowserService2__PauseOrResumeView(This,fPaused)	\
    ( (This)->lpVtbl -> _PauseOrResumeView(This,fPaused) ) 

#define IBrowserService2__DisableModeless(This)	\
    ( (This)->lpVtbl -> _DisableModeless(This) ) 

#define IBrowserService2__NavigateToPidl(This,pidl,grfHLNF,dwFlags)	\
    ( (This)->lpVtbl -> _NavigateToPidl(This,pidl,grfHLNF,dwFlags) ) 

#define IBrowserService2__TryShell2Rename(This,psv,pidlNew)	\
    ( (This)->lpVtbl -> _TryShell2Rename(This,psv,pidlNew) ) 

#define IBrowserService2__SwitchActivationNow(This)	\
    ( (This)->lpVtbl -> _SwitchActivationNow(This) ) 

#define IBrowserService2__ExecChildren(This,punkBar,fBroadcast,pguidCmdGroup,nCmdID,nCmdexecopt,pvarargIn,pvarargOut)	\
    ( (This)->lpVtbl -> _ExecChildren(This,punkBar,fBroadcast,pguidCmdGroup,nCmdID,nCmdexecopt,pvarargIn,pvarargOut) ) 

#define IBrowserService2__SendChildren(This,hwndBar,fBroadcast,uMsg,wParam,lParam)	\
    ( (This)->lpVtbl -> _SendChildren(This,hwndBar,fBroadcast,uMsg,wParam,lParam) ) 

#define IBrowserService2_GetFolderSetData(This,pfsd)	\
    ( (This)->lpVtbl -> GetFolderSetData(This,pfsd) ) 

#define IBrowserService2__OnFocusChange(This,itb)	\
    ( (This)->lpVtbl -> _OnFocusChange(This,itb) ) 

#define IBrowserService2_v_ShowHideChildWindows(This,fChildOnly)	\
    ( (This)->lpVtbl -> v_ShowHideChildWindows(This,fChildOnly) ) 

#define IBrowserService2__get_itbLastFocus(This)	\
    ( (This)->lpVtbl -> _get_itbLastFocus(This) ) 

#define IBrowserService2__put_itbLastFocus(This,itbLastFocus)	\
    ( (This)->lpVtbl -> _put_itbLastFocus(This,itbLastFocus) ) 

#define IBrowserService2__UIActivateView(This,uState)	\
    ( (This)->lpVtbl -> _UIActivateView(This,uState) ) 

#define IBrowserService2__GetViewBorderRect(This,prc)	\
    ( (This)->lpVtbl -> _GetViewBorderRect(This,prc) ) 

#define IBrowserService2__UpdateViewRectSize(This)	\
    ( (This)->lpVtbl -> _UpdateViewRectSize(This) ) 

#define IBrowserService2__ResizeNextBorder(This,itb)	\
    ( (This)->lpVtbl -> _ResizeNextBorder(This,itb) ) 

#define IBrowserService2__ResizeView(This)	\
    ( (This)->lpVtbl -> _ResizeView(This) ) 

#define IBrowserService2__GetEffectiveClientArea(This,lprectBorder,hmon)	\
    ( (This)->lpVtbl -> _GetEffectiveClientArea(This,lprectBorder,hmon) ) 

#define IBrowserService2_v_GetViewStream(This,pidl,grfMode,pwszName)	\
    ( (This)->lpVtbl -> v_GetViewStream(This,pidl,grfMode,pwszName) ) 

#define IBrowserService2_ForwardViewMsg(This,uMsg,wParam,lParam)	\
    ( (This)->lpVtbl -> ForwardViewMsg(This,uMsg,wParam,lParam) ) 

#define IBrowserService2_SetAcceleratorMenu(This,hacc)	\
    ( (This)->lpVtbl -> SetAcceleratorMenu(This,hacc) ) 

#define IBrowserService2__GetToolbarCount(This)	\
    ( (This)->lpVtbl -> _GetToolbarCount(This) ) 

#define IBrowserService2__GetToolbarItem(This,itb)	\
    ( (This)->lpVtbl -> _GetToolbarItem(This,itb) ) 

#define IBrowserService2__SaveToolbars(This,pstm)	\
    ( (This)->lpVtbl -> _SaveToolbars(This,pstm) ) 

#define IBrowserService2__LoadToolbars(This,pstm)	\
    ( (This)->lpVtbl -> _LoadToolbars(This,pstm) ) 

#define IBrowserService2__CloseAndReleaseToolbars(This,fClose)	\
    ( (This)->lpVtbl -> _CloseAndReleaseToolbars(This,fClose) ) 

#define IBrowserService2_v_MayGetNextToolbarFocus(This,lpMsg,itbNext,citb,pptbi,phwnd)	\
    ( (This)->lpVtbl -> v_MayGetNextToolbarFocus(This,lpMsg,itbNext,citb,pptbi,phwnd) ) 

#define IBrowserService2__ResizeNextBorderHelper(This,itb,bUseHmonitor)	\
    ( (This)->lpVtbl -> _ResizeNextBorderHelper(This,itb,bUseHmonitor) ) 

#define IBrowserService2__FindTBar(This,punkSrc)	\
    ( (This)->lpVtbl -> _FindTBar(This,punkSrc) ) 

#define IBrowserService2__SetFocus(This,ptbi,hwnd,lpMsg)	\
    ( (This)->lpVtbl -> _SetFocus(This,ptbi,hwnd,lpMsg) ) 

#define IBrowserService2_v_MayTranslateAccelerator(This,pmsg)	\
    ( (This)->lpVtbl -> v_MayTranslateAccelerator(This,pmsg) ) 

#define IBrowserService2__GetBorderDWHelper(This,punkSrc,lprectBorder,bUseHmonitor)	\
    ( (This)->lpVtbl -> _GetBorderDWHelper(This,punkSrc,lprectBorder,bUseHmonitor) ) 

#define IBrowserService2_v_CheckZoneCrossing(This,pidl)	\
    ( (This)->lpVtbl -> v_CheckZoneCrossing(This,pidl) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBrowserService2_INTERFACE_DEFINED__ */


#ifndef __IBrowserService3_INTERFACE_DEFINED__
#define __IBrowserService3_INTERFACE_DEFINED__

/* interface IBrowserService3 */
/* [local][object][uuid] */ 

typedef 
enum IEPDNFLAGS
    {
        IEPDN_BINDINGUI	= 0x1
    } 	IEPDNFLAGS;


EXTERN_C const IID IID_IBrowserService3;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("27D7CE21-762D-48F3-86F3-40E2FD3749C4")
    IBrowserService3 : public IBrowserService2
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE _PositionViewWindow( 
            /* [annotation][in] */ 
            _In_  HWND hwnd,
            /* [annotation][in] */ 
            _In_  LPRECT prc) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IEParseDisplayNameEx( 
            /* [annotation][in] */ 
            _In_  UINT uiCP,
            /* [annotation][in] */ 
            _In_  LPCWSTR pwszPath,
            /* [annotation][in] */ 
            _In_  DWORD dwFlags,
            /* [annotation][out] */ 
            _Outptr_  PIDLIST_ABSOLUTE *ppidlOut) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBrowserService3Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IBrowserService3 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IBrowserService3 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IBrowserService3 * This);
        
        DECLSPEC_XFGVIRT(IBrowserService, GetParentSite)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetParentSite )( 
            IBrowserService3 * This,
            /* [annotation][out] */ 
            _Outptr_  IOleInPlaceSite **ppipsite);
        
        DECLSPEC_XFGVIRT(IBrowserService, SetTitle)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *SetTitle )( 
            IBrowserService3 * This,
            /* [annotation][in] */ 
            _In_  IShellView *psv,
            /* [annotation][in] */ 
            _In_  LPCWSTR pszName);
        
        DECLSPEC_XFGVIRT(IBrowserService, GetTitle)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetTitle )( 
            IBrowserService3 * This,
            /* [annotation][in] */ 
            _In_  IShellView *psv,
            /* [annotation][size_is][out] */ 
            _Out_writes_(cchName)  LPWSTR pszName,
            DWORD cchName);
        
        DECLSPEC_XFGVIRT(IBrowserService, GetOleObject)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetOleObject )( 
            IBrowserService3 * This,
            /* [annotation][out] */ 
            _Outptr_  IOleObject **ppobjv);
        
        DECLSPEC_XFGVIRT(IBrowserService, GetTravelLog)
        HRESULT ( STDMETHODCALLTYPE *GetTravelLog )( 
            IBrowserService3 * This,
            /* [out] */ ITravelLog **pptl);
        
        DECLSPEC_XFGVIRT(IBrowserService, ShowControlWindow)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *ShowControlWindow )( 
            IBrowserService3 * This,
            UINT id,
            BOOL fShow);
        
        DECLSPEC_XFGVIRT(IBrowserService, IsControlWindowShown)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *IsControlWindowShown )( 
            IBrowserService3 * This,
            UINT id,
            /* [annotation][out] */ 
            _Out_  BOOL *pfShown);
        
        DECLSPEC_XFGVIRT(IBrowserService, IEGetDisplayName)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *IEGetDisplayName )( 
            IBrowserService3 * This,
            /* [annotation][in] */ 
            _In_  PCIDLIST_ABSOLUTE pidl,
            /* [annotation][out] */ 
            _Out_writes_(INTERNET_MAX_URL_LENGTH)  LPWSTR pwszName,
            UINT uFlags);
        
        DECLSPEC_XFGVIRT(IBrowserService, IEParseDisplayName)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *IEParseDisplayName )( 
            IBrowserService3 * This,
            UINT uiCP,
            /* [annotation][in] */ 
            _In_  LPCWSTR pwszPath,
            /* [annotation][out] */ 
            _Outptr_  PIDLIST_ABSOLUTE *ppidlOut);
        
        DECLSPEC_XFGVIRT(IBrowserService, DisplayParseError)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *DisplayParseError )( 
            IBrowserService3 * This,
            HRESULT hres,
            /* [annotation][in] */ 
            _In_  LPCWSTR pwszPath);
        
        DECLSPEC_XFGVIRT(IBrowserService, NavigateToPidl)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *NavigateToPidl )( 
            IBrowserService3 * This,
            /* [annotation][in] */ 
            _In_  PCIDLIST_ABSOLUTE pidl,
            DWORD grfHLNF);
        
        DECLSPEC_XFGVIRT(IBrowserService, SetNavigateState)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *SetNavigateState )( 
            IBrowserService3 * This,
            BNSTATE bnstate);
        
        DECLSPEC_XFGVIRT(IBrowserService, GetNavigateState)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetNavigateState )( 
            IBrowserService3 * This,
            /* [annotation][out] */ 
            _Out_  BNSTATE *pbnstate);
        
        DECLSPEC_XFGVIRT(IBrowserService, NotifyRedirect)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *NotifyRedirect )( 
            IBrowserService3 * This,
            /* [annotation][in] */ 
            _In_  IShellView *psv,
            /* [annotation][in] */ 
            _In_  PCIDLIST_ABSOLUTE pidl,
            /* [annotation][out] */ 
            _Out_  BOOL *pfDidBrowse);
        
        DECLSPEC_XFGVIRT(IBrowserService, UpdateWindowList)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *UpdateWindowList )( 
            IBrowserService3 * This);
        
        DECLSPEC_XFGVIRT(IBrowserService, UpdateBackForwardState)
        HRESULT ( STDMETHODCALLTYPE *UpdateBackForwardState )( 
            IBrowserService3 * This);
        
        DECLSPEC_XFGVIRT(IBrowserService, SetFlags)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *SetFlags )( 
            IBrowserService3 * This,
            DWORD dwFlags,
            DWORD dwFlagMask);
        
        DECLSPEC_XFGVIRT(IBrowserService, GetFlags)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetFlags )( 
            IBrowserService3 * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwFlags);
        
        DECLSPEC_XFGVIRT(IBrowserService, CanNavigateNow)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *CanNavigateNow )( 
            IBrowserService3 * This);
        
        DECLSPEC_XFGVIRT(IBrowserService, GetPidl)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetPidl )( 
            IBrowserService3 * This,
            /* [annotation][out] */ 
            _Out_  PIDLIST_ABSOLUTE *ppidl);
        
        DECLSPEC_XFGVIRT(IBrowserService, SetReferrer)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *SetReferrer )( 
            IBrowserService3 * This,
            /* [annotation][in] */ 
            _In_  PCIDLIST_ABSOLUTE pidl);
        
        DECLSPEC_XFGVIRT(IBrowserService, GetBrowserIndex)
        /* [local] */ DWORD ( STDMETHODCALLTYPE *GetBrowserIndex )( 
            IBrowserService3 * This);
        
        DECLSPEC_XFGVIRT(IBrowserService, GetBrowserByIndex)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetBrowserByIndex )( 
            IBrowserService3 * This,
            DWORD dwID,
            /* [annotation][out] */ 
            _Outptr_  IUnknown **ppunk);
        
        DECLSPEC_XFGVIRT(IBrowserService, GetHistoryObject)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetHistoryObject )( 
            IBrowserService3 * This,
            /* [annotation][out] */ 
            _Outptr_  IOleObject **ppole,
            /* [annotation][out] */ 
            _Outptr_  IStream **pstm,
            /* [annotation][out] */ 
            _Outptr_  IBindCtx **ppbc);
        
        DECLSPEC_XFGVIRT(IBrowserService, SetHistoryObject)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *SetHistoryObject )( 
            IBrowserService3 * This,
            /* [annotation][in] */ 
            _In_  IOleObject *pole,
            BOOL fIsLocalAnchor);
        
        DECLSPEC_XFGVIRT(IBrowserService, CacheOLEServer)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *CacheOLEServer )( 
            IBrowserService3 * This,
            /* [annotation][in] */ 
            _In_  IOleObject *pole);
        
        DECLSPEC_XFGVIRT(IBrowserService, GetSetCodePage)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetSetCodePage )( 
            IBrowserService3 * This,
            /* [annotation][in] */ 
            _In_  VARIANT *pvarIn,
            /* [annotation][out] */ 
            _Out_  VARIANT *pvarOut);
        
        DECLSPEC_XFGVIRT(IBrowserService, OnHttpEquiv)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *OnHttpEquiv )( 
            IBrowserService3 * This,
            /* [annotation][in] */ 
            _In_  IShellView *psv,
            BOOL fDone,
            /* [annotation][in] */ 
            _In_  VARIANT *pvarargIn,
            /* [annotation][out] */ 
            _Out_  VARIANT *pvarargOut);
        
        DECLSPEC_XFGVIRT(IBrowserService, GetPalette)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetPalette )( 
            IBrowserService3 * This,
            /* [annotation][out] */ 
            _Out_  HPALETTE *hpal);
        
        DECLSPEC_XFGVIRT(IBrowserService, RegisterWindow)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *RegisterWindow )( 
            IBrowserService3 * This,
            BOOL fForceRegister,
            int swc);
        
        DECLSPEC_XFGVIRT(IBrowserService2, WndProcBS)
        LRESULT ( STDMETHODCALLTYPE *WndProcBS )( 
            IBrowserService3 * This,
            /* [annotation][in] */ 
            _In_  HWND hwnd,
            /* [annotation][in] */ 
            _In_  UINT uMsg,
            /* [annotation][in] */ 
            _In_  WPARAM wParam,
            /* [annotation][in] */ 
            _In_  LPARAM lParam);
        
        DECLSPEC_XFGVIRT(IBrowserService2, SetAsDefFolderSettings)
        HRESULT ( STDMETHODCALLTYPE *SetAsDefFolderSettings )( 
            IBrowserService3 * This);
        
        DECLSPEC_XFGVIRT(IBrowserService2, GetViewRect)
        HRESULT ( STDMETHODCALLTYPE *GetViewRect )( 
            IBrowserService3 * This,
            /* [annotation][out] */ 
            _Out_  RECT *prc);
        
        DECLSPEC_XFGVIRT(IBrowserService2, OnSize)
        HRESULT ( STDMETHODCALLTYPE *OnSize )( 
            IBrowserService3 * This,
            /* [annotation][in] */ 
            _In_  WPARAM wParam);
        
        DECLSPEC_XFGVIRT(IBrowserService2, OnCreate)
        HRESULT ( STDMETHODCALLTYPE *OnCreate )( 
            IBrowserService3 * This,
            /* [annotation][in] */ 
            _In_  struct tagCREATESTRUCTW *pcs);
        
        DECLSPEC_XFGVIRT(IBrowserService2, OnCommand)
        LRESULT ( STDMETHODCALLTYPE *OnCommand )( 
            IBrowserService3 * This,
            /* [annotation][in] */ 
            _In_  WPARAM wParam,
            /* [annotation][in] */ 
            _In_  LPARAM lParam);
        
        DECLSPEC_XFGVIRT(IBrowserService2, OnDestroy)
        HRESULT ( STDMETHODCALLTYPE *OnDestroy )( 
            IBrowserService3 * This);
        
        DECLSPEC_XFGVIRT(IBrowserService2, OnNotify)
        LRESULT ( STDMETHODCALLTYPE *OnNotify )( 
            IBrowserService3 * This,
            /* [annotation][in] */ 
            _In_  struct tagNMHDR *pnm);
        
        DECLSPEC_XFGVIRT(IBrowserService2, OnSetFocus)
        HRESULT ( STDMETHODCALLTYPE *OnSetFocus )( 
            IBrowserService3 * This);
        
        DECLSPEC_XFGVIRT(IBrowserService2, OnFrameWindowActivateBS)
        HRESULT ( STDMETHODCALLTYPE *OnFrameWindowActivateBS )( 
            IBrowserService3 * This,
            /* [annotation][in] */ 
            _In_  BOOL fActive);
        
        DECLSPEC_XFGVIRT(IBrowserService2, ReleaseShellView)
        HRESULT ( STDMETHODCALLTYPE *ReleaseShellView )( 
            IBrowserService3 * This);
        
        DECLSPEC_XFGVIRT(IBrowserService2, ActivatePendingView)
        HRESULT ( STDMETHODCALLTYPE *ActivatePendingView )( 
            IBrowserService3 * This);
        
        DECLSPEC_XFGVIRT(IBrowserService2, CreateViewWindow)
        HRESULT ( STDMETHODCALLTYPE *CreateViewWindow )( 
            IBrowserService3 * This,
            /* [annotation][in] */ 
            _In_  IShellView *psvNew,
            /* [annotation][in] */ 
            _In_  IShellView *psvOld,
            /* [annotation][in] */ 
            _In_  LPRECT prcView,
            /* [annotation][out] */ 
            _Out_  HWND *phwnd);
        
        DECLSPEC_XFGVIRT(IBrowserService2, CreateBrowserPropSheetExt)
        HRESULT ( STDMETHODCALLTYPE *CreateBrowserPropSheetExt )( 
            IBrowserService3 * This,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][iid_is][out] */ 
            _Outptr_  void **ppv);
        
        DECLSPEC_XFGVIRT(IBrowserService2, GetViewWindow)
        HRESULT ( STDMETHODCALLTYPE *GetViewWindow )( 
            IBrowserService3 * This,
            /* [annotation][out] */ 
            _Out_  HWND *phwndView);
        
        DECLSPEC_XFGVIRT(IBrowserService2, GetBaseBrowserData)
        HRESULT ( STDMETHODCALLTYPE *GetBaseBrowserData )( 
            IBrowserService3 * This,
            /* [annotation][out] */ 
            _Outptr_  LPCBASEBROWSERDATA *pbbd);
        
        DECLSPEC_XFGVIRT(IBrowserService2, PutBaseBrowserData)
        LPBASEBROWSERDATA ( STDMETHODCALLTYPE *PutBaseBrowserData )( 
            IBrowserService3 * This);
        
        DECLSPEC_XFGVIRT(IBrowserService2, InitializeTravelLog)
        HRESULT ( STDMETHODCALLTYPE *InitializeTravelLog )( 
            IBrowserService3 * This,
            /* [annotation][in] */ 
            _In_  ITravelLog *ptl,
            /* [annotation][in] */ 
            _In_  DWORD dw);
        
        DECLSPEC_XFGVIRT(IBrowserService2, SetTopBrowser)
        HRESULT ( STDMETHODCALLTYPE *SetTopBrowser )( 
            IBrowserService3 * This);
        
        DECLSPEC_XFGVIRT(IBrowserService2, Offline)
        HRESULT ( STDMETHODCALLTYPE *Offline )( 
            IBrowserService3 * This,
            /* [annotation][in] */ 
            _In_  int iCmd);
        
        DECLSPEC_XFGVIRT(IBrowserService2, AllowViewResize)
        HRESULT ( STDMETHODCALLTYPE *AllowViewResize )( 
            IBrowserService3 * This,
            /* [annotation][in] */ 
            _In_  BOOL f);
        
        DECLSPEC_XFGVIRT(IBrowserService2, SetActivateState)
        HRESULT ( STDMETHODCALLTYPE *SetActivateState )( 
            IBrowserService3 * This,
            /* [annotation][in] */ 
            _In_  UINT u);
        
        DECLSPEC_XFGVIRT(IBrowserService2, UpdateSecureLockIcon)
        HRESULT ( STDMETHODCALLTYPE *UpdateSecureLockIcon )( 
            IBrowserService3 * This,
            /* [annotation][in] */ 
            _In_  int eSecureLock);
        
        DECLSPEC_XFGVIRT(IBrowserService2, InitializeDownloadManager)
        HRESULT ( STDMETHODCALLTYPE *InitializeDownloadManager )( 
            IBrowserService3 * This);
        
        DECLSPEC_XFGVIRT(IBrowserService2, InitializeTransitionSite)
        HRESULT ( STDMETHODCALLTYPE *InitializeTransitionSite )( 
            IBrowserService3 * This);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _Initialize)
        HRESULT ( STDMETHODCALLTYPE *_Initialize )( 
            IBrowserService3 * This,
            /* [annotation][in] */ 
            _In_  HWND hwnd,
            /* [annotation][in] */ 
            _In_  IUnknown *pauto);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _CancelPendingNavigationAsync)
        HRESULT ( STDMETHODCALLTYPE *_CancelPendingNavigationAsync )( 
            IBrowserService3 * This);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _CancelPendingView)
        HRESULT ( STDMETHODCALLTYPE *_CancelPendingView )( 
            IBrowserService3 * This);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _MaySaveChanges)
        HRESULT ( STDMETHODCALLTYPE *_MaySaveChanges )( 
            IBrowserService3 * This);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _PauseOrResumeView)
        HRESULT ( STDMETHODCALLTYPE *_PauseOrResumeView )( 
            IBrowserService3 * This,
            BOOL fPaused);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _DisableModeless)
        HRESULT ( STDMETHODCALLTYPE *_DisableModeless )( 
            IBrowserService3 * This);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _NavigateToPidl)
        HRESULT ( STDMETHODCALLTYPE *_NavigateToPidl )( 
            IBrowserService3 * This,
            /* [annotation][in] */ 
            _In_  PCIDLIST_ABSOLUTE pidl,
            /* [annotation][in] */ 
            _In_  DWORD grfHLNF,
            /* [annotation][in] */ 
            _In_  DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _TryShell2Rename)
        HRESULT ( STDMETHODCALLTYPE *_TryShell2Rename )( 
            IBrowserService3 * This,
            /* [annotation][in] */ 
            _In_  IShellView *psv,
            /* [annotation][in] */ 
            _In_  PCIDLIST_ABSOLUTE pidlNew);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _SwitchActivationNow)
        HRESULT ( STDMETHODCALLTYPE *_SwitchActivationNow )( 
            IBrowserService3 * This);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _ExecChildren)
        HRESULT ( STDMETHODCALLTYPE *_ExecChildren )( 
            IBrowserService3 * This,
            /* [annotation][in] */ 
            _In_  IUnknown *punkBar,
            /* [annotation][in] */ 
            _In_  BOOL fBroadcast,
            /* [annotation][unique][in] */ 
            _In_opt_  const GUID *pguidCmdGroup,
            /* [annotation][in] */ 
            _In_  DWORD nCmdID,
            /* [annotation][in] */ 
            _In_  DWORD nCmdexecopt,
            /* [annotation][unique][in] */ 
            _In_opt_  VARIANTARG *pvarargIn,
            /* [annotation][unique][out][in] */ 
            _Inout_opt_  VARIANTARG *pvarargOut);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _SendChildren)
        HRESULT ( STDMETHODCALLTYPE *_SendChildren )( 
            IBrowserService3 * This,
            /* [annotation][in] */ 
            _In_  HWND hwndBar,
            /* [annotation][in] */ 
            _In_  BOOL fBroadcast,
            /* [annotation][in] */ 
            _In_  UINT uMsg,
            /* [annotation][in] */ 
            _In_  WPARAM wParam,
            /* [annotation][in] */ 
            _In_  LPARAM lParam);
        
        DECLSPEC_XFGVIRT(IBrowserService2, GetFolderSetData)
        HRESULT ( STDMETHODCALLTYPE *GetFolderSetData )( 
            IBrowserService3 * This,
            /* [out] */ struct tagFolderSetData *pfsd);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _OnFocusChange)
        HRESULT ( STDMETHODCALLTYPE *_OnFocusChange )( 
            IBrowserService3 * This,
            /* [annotation][in] */ 
            _In_  UINT itb);
        
        DECLSPEC_XFGVIRT(IBrowserService2, v_ShowHideChildWindows)
        HRESULT ( STDMETHODCALLTYPE *v_ShowHideChildWindows )( 
            IBrowserService3 * This,
            /* [annotation][in] */ 
            _In_  BOOL fChildOnly);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _get_itbLastFocus)
        UINT ( STDMETHODCALLTYPE *_get_itbLastFocus )( 
            IBrowserService3 * This);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _put_itbLastFocus)
        HRESULT ( STDMETHODCALLTYPE *_put_itbLastFocus )( 
            IBrowserService3 * This,
            /* [annotation][in] */ 
            _In_  UINT itbLastFocus);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _UIActivateView)
        HRESULT ( STDMETHODCALLTYPE *_UIActivateView )( 
            IBrowserService3 * This,
            /* [annotation][in] */ 
            _In_  UINT uState);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _GetViewBorderRect)
        HRESULT ( STDMETHODCALLTYPE *_GetViewBorderRect )( 
            IBrowserService3 * This,
            /* [out] */ RECT *prc);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _UpdateViewRectSize)
        HRESULT ( STDMETHODCALLTYPE *_UpdateViewRectSize )( 
            IBrowserService3 * This);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _ResizeNextBorder)
        HRESULT ( STDMETHODCALLTYPE *_ResizeNextBorder )( 
            IBrowserService3 * This,
            /* [annotation][in] */ 
            _In_  UINT itb);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _ResizeView)
        HRESULT ( STDMETHODCALLTYPE *_ResizeView )( 
            IBrowserService3 * This);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _GetEffectiveClientArea)
        HRESULT ( STDMETHODCALLTYPE *_GetEffectiveClientArea )( 
            IBrowserService3 * This,
            /* [annotation][out] */ 
            _Out_  LPRECT lprectBorder,
            /* [annotation][in] */ 
            _In_  HMONITOR hmon);
        
        DECLSPEC_XFGVIRT(IBrowserService2, v_GetViewStream)
        IStream *( STDMETHODCALLTYPE *v_GetViewStream )( 
            IBrowserService3 * This,
            /* [in] */ PCIDLIST_ABSOLUTE pidl,
            /* [annotation][in] */ 
            _In_  DWORD grfMode,
            /* [annotation][in] */ 
            _In_  LPCWSTR pwszName);
        
        DECLSPEC_XFGVIRT(IBrowserService2, ForwardViewMsg)
        LRESULT ( STDMETHODCALLTYPE *ForwardViewMsg )( 
            IBrowserService3 * This,
            /* [annotation][in] */ 
            _In_  UINT uMsg,
            /* [annotation][in] */ 
            _In_  WPARAM wParam,
            /* [annotation][in] */ 
            _In_  LPARAM lParam);
        
        DECLSPEC_XFGVIRT(IBrowserService2, SetAcceleratorMenu)
        HRESULT ( STDMETHODCALLTYPE *SetAcceleratorMenu )( 
            IBrowserService3 * This,
            /* [annotation][in] */ 
            _In_  HACCEL hacc);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _GetToolbarCount)
        int ( STDMETHODCALLTYPE *_GetToolbarCount )( 
            IBrowserService3 * This);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _GetToolbarItem)
        LPTOOLBARITEM ( STDMETHODCALLTYPE *_GetToolbarItem )( 
            IBrowserService3 * This,
            /* [annotation][in] */ 
            _In_  int itb);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _SaveToolbars)
        HRESULT ( STDMETHODCALLTYPE *_SaveToolbars )( 
            IBrowserService3 * This,
            /* [in] */ IStream *pstm);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _LoadToolbars)
        HRESULT ( STDMETHODCALLTYPE *_LoadToolbars )( 
            IBrowserService3 * This,
            /* [annotation][in] */ 
            _In_  IStream *pstm);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _CloseAndReleaseToolbars)
        HRESULT ( STDMETHODCALLTYPE *_CloseAndReleaseToolbars )( 
            IBrowserService3 * This,
            BOOL fClose);
        
        DECLSPEC_XFGVIRT(IBrowserService2, v_MayGetNextToolbarFocus)
        HRESULT ( STDMETHODCALLTYPE *v_MayGetNextToolbarFocus )( 
            IBrowserService3 * This,
            /* [annotation][in] */ 
            _In_  LPMSG lpMsg,
            /* [annotation][in] */ 
            _In_  UINT itbNext,
            /* [annotation][in] */ 
            _In_  int citb,
            /* [annotation][out] */ 
            _Out_  LPTOOLBARITEM *pptbi,
            /* [annotation][out] */ 
            _Out_  HWND *phwnd);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _ResizeNextBorderHelper)
        HRESULT ( STDMETHODCALLTYPE *_ResizeNextBorderHelper )( 
            IBrowserService3 * This,
            UINT itb,
            BOOL bUseHmonitor);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _FindTBar)
        UINT ( STDMETHODCALLTYPE *_FindTBar )( 
            IBrowserService3 * This,
            /* [annotation][in] */ 
            _In_  IUnknown *punkSrc);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _SetFocus)
        HRESULT ( STDMETHODCALLTYPE *_SetFocus )( 
            IBrowserService3 * This,
            /* [annotation][in] */ 
            _In_  LPTOOLBARITEM ptbi,
            /* [annotation][in] */ 
            _In_  HWND hwnd,
            /* [annotation][in] */ 
            _In_  LPMSG lpMsg);
        
        DECLSPEC_XFGVIRT(IBrowserService2, v_MayTranslateAccelerator)
        HRESULT ( STDMETHODCALLTYPE *v_MayTranslateAccelerator )( 
            IBrowserService3 * This,
            /* [in] */ MSG *pmsg);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _GetBorderDWHelper)
        HRESULT ( STDMETHODCALLTYPE *_GetBorderDWHelper )( 
            IBrowserService3 * This,
            /* [annotation][in] */ 
            _In_  IUnknown *punkSrc,
            /* [annotation][out] */ 
            _Out_  LPRECT lprectBorder,
            /* [annotation][in] */ 
            _In_  BOOL bUseHmonitor);
        
        DECLSPEC_XFGVIRT(IBrowserService2, v_CheckZoneCrossing)
        HRESULT ( STDMETHODCALLTYPE *v_CheckZoneCrossing )( 
            IBrowserService3 * This,
            /* [annotation][in] */ 
            _In_  PCIDLIST_ABSOLUTE pidl);
        
        DECLSPEC_XFGVIRT(IBrowserService3, _PositionViewWindow)
        HRESULT ( STDMETHODCALLTYPE *_PositionViewWindow )( 
            IBrowserService3 * This,
            /* [annotation][in] */ 
            _In_  HWND hwnd,
            /* [annotation][in] */ 
            _In_  LPRECT prc);
        
        DECLSPEC_XFGVIRT(IBrowserService3, IEParseDisplayNameEx)
        HRESULT ( STDMETHODCALLTYPE *IEParseDisplayNameEx )( 
            IBrowserService3 * This,
            /* [annotation][in] */ 
            _In_  UINT uiCP,
            /* [annotation][in] */ 
            _In_  LPCWSTR pwszPath,
            /* [annotation][in] */ 
            _In_  DWORD dwFlags,
            /* [annotation][out] */ 
            _Outptr_  PIDLIST_ABSOLUTE *ppidlOut);
        
        END_INTERFACE
    } IBrowserService3Vtbl;

    interface IBrowserService3
    {
        CONST_VTBL struct IBrowserService3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBrowserService3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBrowserService3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBrowserService3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBrowserService3_GetParentSite(This,ppipsite)	\
    ( (This)->lpVtbl -> GetParentSite(This,ppipsite) ) 

#define IBrowserService3_SetTitle(This,psv,pszName)	\
    ( (This)->lpVtbl -> SetTitle(This,psv,pszName) ) 

#define IBrowserService3_GetTitle(This,psv,pszName,cchName)	\
    ( (This)->lpVtbl -> GetTitle(This,psv,pszName,cchName) ) 

#define IBrowserService3_GetOleObject(This,ppobjv)	\
    ( (This)->lpVtbl -> GetOleObject(This,ppobjv) ) 

#define IBrowserService3_GetTravelLog(This,pptl)	\
    ( (This)->lpVtbl -> GetTravelLog(This,pptl) ) 

#define IBrowserService3_ShowControlWindow(This,id,fShow)	\
    ( (This)->lpVtbl -> ShowControlWindow(This,id,fShow) ) 

#define IBrowserService3_IsControlWindowShown(This,id,pfShown)	\
    ( (This)->lpVtbl -> IsControlWindowShown(This,id,pfShown) ) 

#define IBrowserService3_IEGetDisplayName(This,pidl,pwszName,uFlags)	\
    ( (This)->lpVtbl -> IEGetDisplayName(This,pidl,pwszName,uFlags) ) 

#define IBrowserService3_IEParseDisplayName(This,uiCP,pwszPath,ppidlOut)	\
    ( (This)->lpVtbl -> IEParseDisplayName(This,uiCP,pwszPath,ppidlOut) ) 

#define IBrowserService3_DisplayParseError(This,hres,pwszPath)	\
    ( (This)->lpVtbl -> DisplayParseError(This,hres,pwszPath) ) 

#define IBrowserService3_NavigateToPidl(This,pidl,grfHLNF)	\
    ( (This)->lpVtbl -> NavigateToPidl(This,pidl,grfHLNF) ) 

#define IBrowserService3_SetNavigateState(This,bnstate)	\
    ( (This)->lpVtbl -> SetNavigateState(This,bnstate) ) 

#define IBrowserService3_GetNavigateState(This,pbnstate)	\
    ( (This)->lpVtbl -> GetNavigateState(This,pbnstate) ) 

#define IBrowserService3_NotifyRedirect(This,psv,pidl,pfDidBrowse)	\
    ( (This)->lpVtbl -> NotifyRedirect(This,psv,pidl,pfDidBrowse) ) 

#define IBrowserService3_UpdateWindowList(This)	\
    ( (This)->lpVtbl -> UpdateWindowList(This) ) 

#define IBrowserService3_UpdateBackForwardState(This)	\
    ( (This)->lpVtbl -> UpdateBackForwardState(This) ) 

#define IBrowserService3_SetFlags(This,dwFlags,dwFlagMask)	\
    ( (This)->lpVtbl -> SetFlags(This,dwFlags,dwFlagMask) ) 

#define IBrowserService3_GetFlags(This,pdwFlags)	\
    ( (This)->lpVtbl -> GetFlags(This,pdwFlags) ) 

#define IBrowserService3_CanNavigateNow(This)	\
    ( (This)->lpVtbl -> CanNavigateNow(This) ) 

#define IBrowserService3_GetPidl(This,ppidl)	\
    ( (This)->lpVtbl -> GetPidl(This,ppidl) ) 

#define IBrowserService3_SetReferrer(This,pidl)	\
    ( (This)->lpVtbl -> SetReferrer(This,pidl) ) 

#define IBrowserService3_GetBrowserIndex(This)	\
    ( (This)->lpVtbl -> GetBrowserIndex(This) ) 

#define IBrowserService3_GetBrowserByIndex(This,dwID,ppunk)	\
    ( (This)->lpVtbl -> GetBrowserByIndex(This,dwID,ppunk) ) 

#define IBrowserService3_GetHistoryObject(This,ppole,pstm,ppbc)	\
    ( (This)->lpVtbl -> GetHistoryObject(This,ppole,pstm,ppbc) ) 

#define IBrowserService3_SetHistoryObject(This,pole,fIsLocalAnchor)	\
    ( (This)->lpVtbl -> SetHistoryObject(This,pole,fIsLocalAnchor) ) 

#define IBrowserService3_CacheOLEServer(This,pole)	\
    ( (This)->lpVtbl -> CacheOLEServer(This,pole) ) 

#define IBrowserService3_GetSetCodePage(This,pvarIn,pvarOut)	\
    ( (This)->lpVtbl -> GetSetCodePage(This,pvarIn,pvarOut) ) 

#define IBrowserService3_OnHttpEquiv(This,psv,fDone,pvarargIn,pvarargOut)	\
    ( (This)->lpVtbl -> OnHttpEquiv(This,psv,fDone,pvarargIn,pvarargOut) ) 

#define IBrowserService3_GetPalette(This,hpal)	\
    ( (This)->lpVtbl -> GetPalette(This,hpal) ) 

#define IBrowserService3_RegisterWindow(This,fForceRegister,swc)	\
    ( (This)->lpVtbl -> RegisterWindow(This,fForceRegister,swc) ) 


#define IBrowserService3_WndProcBS(This,hwnd,uMsg,wParam,lParam)	\
    ( (This)->lpVtbl -> WndProcBS(This,hwnd,uMsg,wParam,lParam) ) 

#define IBrowserService3_SetAsDefFolderSettings(This)	\
    ( (This)->lpVtbl -> SetAsDefFolderSettings(This) ) 

#define IBrowserService3_GetViewRect(This,prc)	\
    ( (This)->lpVtbl -> GetViewRect(This,prc) ) 

#define IBrowserService3_OnSize(This,wParam)	\
    ( (This)->lpVtbl -> OnSize(This,wParam) ) 

#define IBrowserService3_OnCreate(This,pcs)	\
    ( (This)->lpVtbl -> OnCreate(This,pcs) ) 

#define IBrowserService3_OnCommand(This,wParam,lParam)	\
    ( (This)->lpVtbl -> OnCommand(This,wParam,lParam) ) 

#define IBrowserService3_OnDestroy(This)	\
    ( (This)->lpVtbl -> OnDestroy(This) ) 

#define IBrowserService3_OnNotify(This,pnm)	\
    ( (This)->lpVtbl -> OnNotify(This,pnm) ) 

#define IBrowserService3_OnSetFocus(This)	\
    ( (This)->lpVtbl -> OnSetFocus(This) ) 

#define IBrowserService3_OnFrameWindowActivateBS(This,fActive)	\
    ( (This)->lpVtbl -> OnFrameWindowActivateBS(This,fActive) ) 

#define IBrowserService3_ReleaseShellView(This)	\
    ( (This)->lpVtbl -> ReleaseShellView(This) ) 

#define IBrowserService3_ActivatePendingView(This)	\
    ( (This)->lpVtbl -> ActivatePendingView(This) ) 

#define IBrowserService3_CreateViewWindow(This,psvNew,psvOld,prcView,phwnd)	\
    ( (This)->lpVtbl -> CreateViewWindow(This,psvNew,psvOld,prcView,phwnd) ) 

#define IBrowserService3_CreateBrowserPropSheetExt(This,riid,ppv)	\
    ( (This)->lpVtbl -> CreateBrowserPropSheetExt(This,riid,ppv) ) 

#define IBrowserService3_GetViewWindow(This,phwndView)	\
    ( (This)->lpVtbl -> GetViewWindow(This,phwndView) ) 

#define IBrowserService3_GetBaseBrowserData(This,pbbd)	\
    ( (This)->lpVtbl -> GetBaseBrowserData(This,pbbd) ) 

#define IBrowserService3_PutBaseBrowserData(This)	\
    ( (This)->lpVtbl -> PutBaseBrowserData(This) ) 

#define IBrowserService3_InitializeTravelLog(This,ptl,dw)	\
    ( (This)->lpVtbl -> InitializeTravelLog(This,ptl,dw) ) 

#define IBrowserService3_SetTopBrowser(This)	\
    ( (This)->lpVtbl -> SetTopBrowser(This) ) 

#define IBrowserService3_Offline(This,iCmd)	\
    ( (This)->lpVtbl -> Offline(This,iCmd) ) 

#define IBrowserService3_AllowViewResize(This,f)	\
    ( (This)->lpVtbl -> AllowViewResize(This,f) ) 

#define IBrowserService3_SetActivateState(This,u)	\
    ( (This)->lpVtbl -> SetActivateState(This,u) ) 

#define IBrowserService3_UpdateSecureLockIcon(This,eSecureLock)	\
    ( (This)->lpVtbl -> UpdateSecureLockIcon(This,eSecureLock) ) 

#define IBrowserService3_InitializeDownloadManager(This)	\
    ( (This)->lpVtbl -> InitializeDownloadManager(This) ) 

#define IBrowserService3_InitializeTransitionSite(This)	\
    ( (This)->lpVtbl -> InitializeTransitionSite(This) ) 

#define IBrowserService3__Initialize(This,hwnd,pauto)	\
    ( (This)->lpVtbl -> _Initialize(This,hwnd,pauto) ) 

#define IBrowserService3__CancelPendingNavigationAsync(This)	\
    ( (This)->lpVtbl -> _CancelPendingNavigationAsync(This) ) 

#define IBrowserService3__CancelPendingView(This)	\
    ( (This)->lpVtbl -> _CancelPendingView(This) ) 

#define IBrowserService3__MaySaveChanges(This)	\
    ( (This)->lpVtbl -> _MaySaveChanges(This) ) 

#define IBrowserService3__PauseOrResumeView(This,fPaused)	\
    ( (This)->lpVtbl -> _PauseOrResumeView(This,fPaused) ) 

#define IBrowserService3__DisableModeless(This)	\
    ( (This)->lpVtbl -> _DisableModeless(This) ) 

#define IBrowserService3__NavigateToPidl(This,pidl,grfHLNF,dwFlags)	\
    ( (This)->lpVtbl -> _NavigateToPidl(This,pidl,grfHLNF,dwFlags) ) 

#define IBrowserService3__TryShell2Rename(This,psv,pidlNew)	\
    ( (This)->lpVtbl -> _TryShell2Rename(This,psv,pidlNew) ) 

#define IBrowserService3__SwitchActivationNow(This)	\
    ( (This)->lpVtbl -> _SwitchActivationNow(This) ) 

#define IBrowserService3__ExecChildren(This,punkBar,fBroadcast,pguidCmdGroup,nCmdID,nCmdexecopt,pvarargIn,pvarargOut)	\
    ( (This)->lpVtbl -> _ExecChildren(This,punkBar,fBroadcast,pguidCmdGroup,nCmdID,nCmdexecopt,pvarargIn,pvarargOut) ) 

#define IBrowserService3__SendChildren(This,hwndBar,fBroadcast,uMsg,wParam,lParam)	\
    ( (This)->lpVtbl -> _SendChildren(This,hwndBar,fBroadcast,uMsg,wParam,lParam) ) 

#define IBrowserService3_GetFolderSetData(This,pfsd)	\
    ( (This)->lpVtbl -> GetFolderSetData(This,pfsd) ) 

#define IBrowserService3__OnFocusChange(This,itb)	\
    ( (This)->lpVtbl -> _OnFocusChange(This,itb) ) 

#define IBrowserService3_v_ShowHideChildWindows(This,fChildOnly)	\
    ( (This)->lpVtbl -> v_ShowHideChildWindows(This,fChildOnly) ) 

#define IBrowserService3__get_itbLastFocus(This)	\
    ( (This)->lpVtbl -> _get_itbLastFocus(This) ) 

#define IBrowserService3__put_itbLastFocus(This,itbLastFocus)	\
    ( (This)->lpVtbl -> _put_itbLastFocus(This,itbLastFocus) ) 

#define IBrowserService3__UIActivateView(This,uState)	\
    ( (This)->lpVtbl -> _UIActivateView(This,uState) ) 

#define IBrowserService3__GetViewBorderRect(This,prc)	\
    ( (This)->lpVtbl -> _GetViewBorderRect(This,prc) ) 

#define IBrowserService3__UpdateViewRectSize(This)	\
    ( (This)->lpVtbl -> _UpdateViewRectSize(This) ) 

#define IBrowserService3__ResizeNextBorder(This,itb)	\
    ( (This)->lpVtbl -> _ResizeNextBorder(This,itb) ) 

#define IBrowserService3__ResizeView(This)	\
    ( (This)->lpVtbl -> _ResizeView(This) ) 

#define IBrowserService3__GetEffectiveClientArea(This,lprectBorder,hmon)	\
    ( (This)->lpVtbl -> _GetEffectiveClientArea(This,lprectBorder,hmon) ) 

#define IBrowserService3_v_GetViewStream(This,pidl,grfMode,pwszName)	\
    ( (This)->lpVtbl -> v_GetViewStream(This,pidl,grfMode,pwszName) ) 

#define IBrowserService3_ForwardViewMsg(This,uMsg,wParam,lParam)	\
    ( (This)->lpVtbl -> ForwardViewMsg(This,uMsg,wParam,lParam) ) 

#define IBrowserService3_SetAcceleratorMenu(This,hacc)	\
    ( (This)->lpVtbl -> SetAcceleratorMenu(This,hacc) ) 

#define IBrowserService3__GetToolbarCount(This)	\
    ( (This)->lpVtbl -> _GetToolbarCount(This) ) 

#define IBrowserService3__GetToolbarItem(This,itb)	\
    ( (This)->lpVtbl -> _GetToolbarItem(This,itb) ) 

#define IBrowserService3__SaveToolbars(This,pstm)	\
    ( (This)->lpVtbl -> _SaveToolbars(This,pstm) ) 

#define IBrowserService3__LoadToolbars(This,pstm)	\
    ( (This)->lpVtbl -> _LoadToolbars(This,pstm) ) 

#define IBrowserService3__CloseAndReleaseToolbars(This,fClose)	\
    ( (This)->lpVtbl -> _CloseAndReleaseToolbars(This,fClose) ) 

#define IBrowserService3_v_MayGetNextToolbarFocus(This,lpMsg,itbNext,citb,pptbi,phwnd)	\
    ( (This)->lpVtbl -> v_MayGetNextToolbarFocus(This,lpMsg,itbNext,citb,pptbi,phwnd) ) 

#define IBrowserService3__ResizeNextBorderHelper(This,itb,bUseHmonitor)	\
    ( (This)->lpVtbl -> _ResizeNextBorderHelper(This,itb,bUseHmonitor) ) 

#define IBrowserService3__FindTBar(This,punkSrc)	\
    ( (This)->lpVtbl -> _FindTBar(This,punkSrc) ) 

#define IBrowserService3__SetFocus(This,ptbi,hwnd,lpMsg)	\
    ( (This)->lpVtbl -> _SetFocus(This,ptbi,hwnd,lpMsg) ) 

#define IBrowserService3_v_MayTranslateAccelerator(This,pmsg)	\
    ( (This)->lpVtbl -> v_MayTranslateAccelerator(This,pmsg) ) 

#define IBrowserService3__GetBorderDWHelper(This,punkSrc,lprectBorder,bUseHmonitor)	\
    ( (This)->lpVtbl -> _GetBorderDWHelper(This,punkSrc,lprectBorder,bUseHmonitor) ) 

#define IBrowserService3_v_CheckZoneCrossing(This,pidl)	\
    ( (This)->lpVtbl -> v_CheckZoneCrossing(This,pidl) ) 


#define IBrowserService3__PositionViewWindow(This,hwnd,prc)	\
    ( (This)->lpVtbl -> _PositionViewWindow(This,hwnd,prc) ) 

#define IBrowserService3_IEParseDisplayNameEx(This,uiCP,pwszPath,dwFlags,ppidlOut)	\
    ( (This)->lpVtbl -> IEParseDisplayNameEx(This,uiCP,pwszPath,dwFlags,ppidlOut) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBrowserService3_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_shdeprecated_0000_0008 */
/* [local] */ 

#if (NTDDI_VERSION >= NTDDI_VISTA) || (_WIN32_IE >= _WIN32_IE_IE70)


extern RPC_IF_HANDLE __MIDL_itf_shdeprecated_0000_0008_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_shdeprecated_0000_0008_v0_0_s_ifspec;

#ifndef __IBrowserService4_INTERFACE_DEFINED__
#define __IBrowserService4_INTERFACE_DEFINED__

/* interface IBrowserService4 */
/* [local][object][uuid] */ 


EXTERN_C const IID IID_IBrowserService4;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("639f1bff-e135-4096-abd8-e0f504d649a4")
    IBrowserService4 : public IBrowserService3
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ActivateView( 
            /* [in] */ BOOL fPendingView) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SaveViewState( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE _ResizeAllBorders( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBrowserService4Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IBrowserService4 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IBrowserService4 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IBrowserService4 * This);
        
        DECLSPEC_XFGVIRT(IBrowserService, GetParentSite)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetParentSite )( 
            IBrowserService4 * This,
            /* [annotation][out] */ 
            _Outptr_  IOleInPlaceSite **ppipsite);
        
        DECLSPEC_XFGVIRT(IBrowserService, SetTitle)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *SetTitle )( 
            IBrowserService4 * This,
            /* [annotation][in] */ 
            _In_  IShellView *psv,
            /* [annotation][in] */ 
            _In_  LPCWSTR pszName);
        
        DECLSPEC_XFGVIRT(IBrowserService, GetTitle)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetTitle )( 
            IBrowserService4 * This,
            /* [annotation][in] */ 
            _In_  IShellView *psv,
            /* [annotation][size_is][out] */ 
            _Out_writes_(cchName)  LPWSTR pszName,
            DWORD cchName);
        
        DECLSPEC_XFGVIRT(IBrowserService, GetOleObject)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetOleObject )( 
            IBrowserService4 * This,
            /* [annotation][out] */ 
            _Outptr_  IOleObject **ppobjv);
        
        DECLSPEC_XFGVIRT(IBrowserService, GetTravelLog)
        HRESULT ( STDMETHODCALLTYPE *GetTravelLog )( 
            IBrowserService4 * This,
            /* [out] */ ITravelLog **pptl);
        
        DECLSPEC_XFGVIRT(IBrowserService, ShowControlWindow)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *ShowControlWindow )( 
            IBrowserService4 * This,
            UINT id,
            BOOL fShow);
        
        DECLSPEC_XFGVIRT(IBrowserService, IsControlWindowShown)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *IsControlWindowShown )( 
            IBrowserService4 * This,
            UINT id,
            /* [annotation][out] */ 
            _Out_  BOOL *pfShown);
        
        DECLSPEC_XFGVIRT(IBrowserService, IEGetDisplayName)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *IEGetDisplayName )( 
            IBrowserService4 * This,
            /* [annotation][in] */ 
            _In_  PCIDLIST_ABSOLUTE pidl,
            /* [annotation][out] */ 
            _Out_writes_(INTERNET_MAX_URL_LENGTH)  LPWSTR pwszName,
            UINT uFlags);
        
        DECLSPEC_XFGVIRT(IBrowserService, IEParseDisplayName)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *IEParseDisplayName )( 
            IBrowserService4 * This,
            UINT uiCP,
            /* [annotation][in] */ 
            _In_  LPCWSTR pwszPath,
            /* [annotation][out] */ 
            _Outptr_  PIDLIST_ABSOLUTE *ppidlOut);
        
        DECLSPEC_XFGVIRT(IBrowserService, DisplayParseError)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *DisplayParseError )( 
            IBrowserService4 * This,
            HRESULT hres,
            /* [annotation][in] */ 
            _In_  LPCWSTR pwszPath);
        
        DECLSPEC_XFGVIRT(IBrowserService, NavigateToPidl)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *NavigateToPidl )( 
            IBrowserService4 * This,
            /* [annotation][in] */ 
            _In_  PCIDLIST_ABSOLUTE pidl,
            DWORD grfHLNF);
        
        DECLSPEC_XFGVIRT(IBrowserService, SetNavigateState)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *SetNavigateState )( 
            IBrowserService4 * This,
            BNSTATE bnstate);
        
        DECLSPEC_XFGVIRT(IBrowserService, GetNavigateState)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetNavigateState )( 
            IBrowserService4 * This,
            /* [annotation][out] */ 
            _Out_  BNSTATE *pbnstate);
        
        DECLSPEC_XFGVIRT(IBrowserService, NotifyRedirect)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *NotifyRedirect )( 
            IBrowserService4 * This,
            /* [annotation][in] */ 
            _In_  IShellView *psv,
            /* [annotation][in] */ 
            _In_  PCIDLIST_ABSOLUTE pidl,
            /* [annotation][out] */ 
            _Out_  BOOL *pfDidBrowse);
        
        DECLSPEC_XFGVIRT(IBrowserService, UpdateWindowList)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *UpdateWindowList )( 
            IBrowserService4 * This);
        
        DECLSPEC_XFGVIRT(IBrowserService, UpdateBackForwardState)
        HRESULT ( STDMETHODCALLTYPE *UpdateBackForwardState )( 
            IBrowserService4 * This);
        
        DECLSPEC_XFGVIRT(IBrowserService, SetFlags)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *SetFlags )( 
            IBrowserService4 * This,
            DWORD dwFlags,
            DWORD dwFlagMask);
        
        DECLSPEC_XFGVIRT(IBrowserService, GetFlags)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetFlags )( 
            IBrowserService4 * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwFlags);
        
        DECLSPEC_XFGVIRT(IBrowserService, CanNavigateNow)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *CanNavigateNow )( 
            IBrowserService4 * This);
        
        DECLSPEC_XFGVIRT(IBrowserService, GetPidl)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetPidl )( 
            IBrowserService4 * This,
            /* [annotation][out] */ 
            _Out_  PIDLIST_ABSOLUTE *ppidl);
        
        DECLSPEC_XFGVIRT(IBrowserService, SetReferrer)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *SetReferrer )( 
            IBrowserService4 * This,
            /* [annotation][in] */ 
            _In_  PCIDLIST_ABSOLUTE pidl);
        
        DECLSPEC_XFGVIRT(IBrowserService, GetBrowserIndex)
        /* [local] */ DWORD ( STDMETHODCALLTYPE *GetBrowserIndex )( 
            IBrowserService4 * This);
        
        DECLSPEC_XFGVIRT(IBrowserService, GetBrowserByIndex)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetBrowserByIndex )( 
            IBrowserService4 * This,
            DWORD dwID,
            /* [annotation][out] */ 
            _Outptr_  IUnknown **ppunk);
        
        DECLSPEC_XFGVIRT(IBrowserService, GetHistoryObject)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetHistoryObject )( 
            IBrowserService4 * This,
            /* [annotation][out] */ 
            _Outptr_  IOleObject **ppole,
            /* [annotation][out] */ 
            _Outptr_  IStream **pstm,
            /* [annotation][out] */ 
            _Outptr_  IBindCtx **ppbc);
        
        DECLSPEC_XFGVIRT(IBrowserService, SetHistoryObject)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *SetHistoryObject )( 
            IBrowserService4 * This,
            /* [annotation][in] */ 
            _In_  IOleObject *pole,
            BOOL fIsLocalAnchor);
        
        DECLSPEC_XFGVIRT(IBrowserService, CacheOLEServer)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *CacheOLEServer )( 
            IBrowserService4 * This,
            /* [annotation][in] */ 
            _In_  IOleObject *pole);
        
        DECLSPEC_XFGVIRT(IBrowserService, GetSetCodePage)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetSetCodePage )( 
            IBrowserService4 * This,
            /* [annotation][in] */ 
            _In_  VARIANT *pvarIn,
            /* [annotation][out] */ 
            _Out_  VARIANT *pvarOut);
        
        DECLSPEC_XFGVIRT(IBrowserService, OnHttpEquiv)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *OnHttpEquiv )( 
            IBrowserService4 * This,
            /* [annotation][in] */ 
            _In_  IShellView *psv,
            BOOL fDone,
            /* [annotation][in] */ 
            _In_  VARIANT *pvarargIn,
            /* [annotation][out] */ 
            _Out_  VARIANT *pvarargOut);
        
        DECLSPEC_XFGVIRT(IBrowserService, GetPalette)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetPalette )( 
            IBrowserService4 * This,
            /* [annotation][out] */ 
            _Out_  HPALETTE *hpal);
        
        DECLSPEC_XFGVIRT(IBrowserService, RegisterWindow)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *RegisterWindow )( 
            IBrowserService4 * This,
            BOOL fForceRegister,
            int swc);
        
        DECLSPEC_XFGVIRT(IBrowserService2, WndProcBS)
        LRESULT ( STDMETHODCALLTYPE *WndProcBS )( 
            IBrowserService4 * This,
            /* [annotation][in] */ 
            _In_  HWND hwnd,
            /* [annotation][in] */ 
            _In_  UINT uMsg,
            /* [annotation][in] */ 
            _In_  WPARAM wParam,
            /* [annotation][in] */ 
            _In_  LPARAM lParam);
        
        DECLSPEC_XFGVIRT(IBrowserService2, SetAsDefFolderSettings)
        HRESULT ( STDMETHODCALLTYPE *SetAsDefFolderSettings )( 
            IBrowserService4 * This);
        
        DECLSPEC_XFGVIRT(IBrowserService2, GetViewRect)
        HRESULT ( STDMETHODCALLTYPE *GetViewRect )( 
            IBrowserService4 * This,
            /* [annotation][out] */ 
            _Out_  RECT *prc);
        
        DECLSPEC_XFGVIRT(IBrowserService2, OnSize)
        HRESULT ( STDMETHODCALLTYPE *OnSize )( 
            IBrowserService4 * This,
            /* [annotation][in] */ 
            _In_  WPARAM wParam);
        
        DECLSPEC_XFGVIRT(IBrowserService2, OnCreate)
        HRESULT ( STDMETHODCALLTYPE *OnCreate )( 
            IBrowserService4 * This,
            /* [annotation][in] */ 
            _In_  struct tagCREATESTRUCTW *pcs);
        
        DECLSPEC_XFGVIRT(IBrowserService2, OnCommand)
        LRESULT ( STDMETHODCALLTYPE *OnCommand )( 
            IBrowserService4 * This,
            /* [annotation][in] */ 
            _In_  WPARAM wParam,
            /* [annotation][in] */ 
            _In_  LPARAM lParam);
        
        DECLSPEC_XFGVIRT(IBrowserService2, OnDestroy)
        HRESULT ( STDMETHODCALLTYPE *OnDestroy )( 
            IBrowserService4 * This);
        
        DECLSPEC_XFGVIRT(IBrowserService2, OnNotify)
        LRESULT ( STDMETHODCALLTYPE *OnNotify )( 
            IBrowserService4 * This,
            /* [annotation][in] */ 
            _In_  struct tagNMHDR *pnm);
        
        DECLSPEC_XFGVIRT(IBrowserService2, OnSetFocus)
        HRESULT ( STDMETHODCALLTYPE *OnSetFocus )( 
            IBrowserService4 * This);
        
        DECLSPEC_XFGVIRT(IBrowserService2, OnFrameWindowActivateBS)
        HRESULT ( STDMETHODCALLTYPE *OnFrameWindowActivateBS )( 
            IBrowserService4 * This,
            /* [annotation][in] */ 
            _In_  BOOL fActive);
        
        DECLSPEC_XFGVIRT(IBrowserService2, ReleaseShellView)
        HRESULT ( STDMETHODCALLTYPE *ReleaseShellView )( 
            IBrowserService4 * This);
        
        DECLSPEC_XFGVIRT(IBrowserService2, ActivatePendingView)
        HRESULT ( STDMETHODCALLTYPE *ActivatePendingView )( 
            IBrowserService4 * This);
        
        DECLSPEC_XFGVIRT(IBrowserService2, CreateViewWindow)
        HRESULT ( STDMETHODCALLTYPE *CreateViewWindow )( 
            IBrowserService4 * This,
            /* [annotation][in] */ 
            _In_  IShellView *psvNew,
            /* [annotation][in] */ 
            _In_  IShellView *psvOld,
            /* [annotation][in] */ 
            _In_  LPRECT prcView,
            /* [annotation][out] */ 
            _Out_  HWND *phwnd);
        
        DECLSPEC_XFGVIRT(IBrowserService2, CreateBrowserPropSheetExt)
        HRESULT ( STDMETHODCALLTYPE *CreateBrowserPropSheetExt )( 
            IBrowserService4 * This,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][iid_is][out] */ 
            _Outptr_  void **ppv);
        
        DECLSPEC_XFGVIRT(IBrowserService2, GetViewWindow)
        HRESULT ( STDMETHODCALLTYPE *GetViewWindow )( 
            IBrowserService4 * This,
            /* [annotation][out] */ 
            _Out_  HWND *phwndView);
        
        DECLSPEC_XFGVIRT(IBrowserService2, GetBaseBrowserData)
        HRESULT ( STDMETHODCALLTYPE *GetBaseBrowserData )( 
            IBrowserService4 * This,
            /* [annotation][out] */ 
            _Outptr_  LPCBASEBROWSERDATA *pbbd);
        
        DECLSPEC_XFGVIRT(IBrowserService2, PutBaseBrowserData)
        LPBASEBROWSERDATA ( STDMETHODCALLTYPE *PutBaseBrowserData )( 
            IBrowserService4 * This);
        
        DECLSPEC_XFGVIRT(IBrowserService2, InitializeTravelLog)
        HRESULT ( STDMETHODCALLTYPE *InitializeTravelLog )( 
            IBrowserService4 * This,
            /* [annotation][in] */ 
            _In_  ITravelLog *ptl,
            /* [annotation][in] */ 
            _In_  DWORD dw);
        
        DECLSPEC_XFGVIRT(IBrowserService2, SetTopBrowser)
        HRESULT ( STDMETHODCALLTYPE *SetTopBrowser )( 
            IBrowserService4 * This);
        
        DECLSPEC_XFGVIRT(IBrowserService2, Offline)
        HRESULT ( STDMETHODCALLTYPE *Offline )( 
            IBrowserService4 * This,
            /* [annotation][in] */ 
            _In_  int iCmd);
        
        DECLSPEC_XFGVIRT(IBrowserService2, AllowViewResize)
        HRESULT ( STDMETHODCALLTYPE *AllowViewResize )( 
            IBrowserService4 * This,
            /* [annotation][in] */ 
            _In_  BOOL f);
        
        DECLSPEC_XFGVIRT(IBrowserService2, SetActivateState)
        HRESULT ( STDMETHODCALLTYPE *SetActivateState )( 
            IBrowserService4 * This,
            /* [annotation][in] */ 
            _In_  UINT u);
        
        DECLSPEC_XFGVIRT(IBrowserService2, UpdateSecureLockIcon)
        HRESULT ( STDMETHODCALLTYPE *UpdateSecureLockIcon )( 
            IBrowserService4 * This,
            /* [annotation][in] */ 
            _In_  int eSecureLock);
        
        DECLSPEC_XFGVIRT(IBrowserService2, InitializeDownloadManager)
        HRESULT ( STDMETHODCALLTYPE *InitializeDownloadManager )( 
            IBrowserService4 * This);
        
        DECLSPEC_XFGVIRT(IBrowserService2, InitializeTransitionSite)
        HRESULT ( STDMETHODCALLTYPE *InitializeTransitionSite )( 
            IBrowserService4 * This);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _Initialize)
        HRESULT ( STDMETHODCALLTYPE *_Initialize )( 
            IBrowserService4 * This,
            /* [annotation][in] */ 
            _In_  HWND hwnd,
            /* [annotation][in] */ 
            _In_  IUnknown *pauto);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _CancelPendingNavigationAsync)
        HRESULT ( STDMETHODCALLTYPE *_CancelPendingNavigationAsync )( 
            IBrowserService4 * This);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _CancelPendingView)
        HRESULT ( STDMETHODCALLTYPE *_CancelPendingView )( 
            IBrowserService4 * This);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _MaySaveChanges)
        HRESULT ( STDMETHODCALLTYPE *_MaySaveChanges )( 
            IBrowserService4 * This);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _PauseOrResumeView)
        HRESULT ( STDMETHODCALLTYPE *_PauseOrResumeView )( 
            IBrowserService4 * This,
            BOOL fPaused);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _DisableModeless)
        HRESULT ( STDMETHODCALLTYPE *_DisableModeless )( 
            IBrowserService4 * This);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _NavigateToPidl)
        HRESULT ( STDMETHODCALLTYPE *_NavigateToPidl )( 
            IBrowserService4 * This,
            /* [annotation][in] */ 
            _In_  PCIDLIST_ABSOLUTE pidl,
            /* [annotation][in] */ 
            _In_  DWORD grfHLNF,
            /* [annotation][in] */ 
            _In_  DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _TryShell2Rename)
        HRESULT ( STDMETHODCALLTYPE *_TryShell2Rename )( 
            IBrowserService4 * This,
            /* [annotation][in] */ 
            _In_  IShellView *psv,
            /* [annotation][in] */ 
            _In_  PCIDLIST_ABSOLUTE pidlNew);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _SwitchActivationNow)
        HRESULT ( STDMETHODCALLTYPE *_SwitchActivationNow )( 
            IBrowserService4 * This);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _ExecChildren)
        HRESULT ( STDMETHODCALLTYPE *_ExecChildren )( 
            IBrowserService4 * This,
            /* [annotation][in] */ 
            _In_  IUnknown *punkBar,
            /* [annotation][in] */ 
            _In_  BOOL fBroadcast,
            /* [annotation][unique][in] */ 
            _In_opt_  const GUID *pguidCmdGroup,
            /* [annotation][in] */ 
            _In_  DWORD nCmdID,
            /* [annotation][in] */ 
            _In_  DWORD nCmdexecopt,
            /* [annotation][unique][in] */ 
            _In_opt_  VARIANTARG *pvarargIn,
            /* [annotation][unique][out][in] */ 
            _Inout_opt_  VARIANTARG *pvarargOut);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _SendChildren)
        HRESULT ( STDMETHODCALLTYPE *_SendChildren )( 
            IBrowserService4 * This,
            /* [annotation][in] */ 
            _In_  HWND hwndBar,
            /* [annotation][in] */ 
            _In_  BOOL fBroadcast,
            /* [annotation][in] */ 
            _In_  UINT uMsg,
            /* [annotation][in] */ 
            _In_  WPARAM wParam,
            /* [annotation][in] */ 
            _In_  LPARAM lParam);
        
        DECLSPEC_XFGVIRT(IBrowserService2, GetFolderSetData)
        HRESULT ( STDMETHODCALLTYPE *GetFolderSetData )( 
            IBrowserService4 * This,
            /* [out] */ struct tagFolderSetData *pfsd);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _OnFocusChange)
        HRESULT ( STDMETHODCALLTYPE *_OnFocusChange )( 
            IBrowserService4 * This,
            /* [annotation][in] */ 
            _In_  UINT itb);
        
        DECLSPEC_XFGVIRT(IBrowserService2, v_ShowHideChildWindows)
        HRESULT ( STDMETHODCALLTYPE *v_ShowHideChildWindows )( 
            IBrowserService4 * This,
            /* [annotation][in] */ 
            _In_  BOOL fChildOnly);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _get_itbLastFocus)
        UINT ( STDMETHODCALLTYPE *_get_itbLastFocus )( 
            IBrowserService4 * This);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _put_itbLastFocus)
        HRESULT ( STDMETHODCALLTYPE *_put_itbLastFocus )( 
            IBrowserService4 * This,
            /* [annotation][in] */ 
            _In_  UINT itbLastFocus);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _UIActivateView)
        HRESULT ( STDMETHODCALLTYPE *_UIActivateView )( 
            IBrowserService4 * This,
            /* [annotation][in] */ 
            _In_  UINT uState);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _GetViewBorderRect)
        HRESULT ( STDMETHODCALLTYPE *_GetViewBorderRect )( 
            IBrowserService4 * This,
            /* [out] */ RECT *prc);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _UpdateViewRectSize)
        HRESULT ( STDMETHODCALLTYPE *_UpdateViewRectSize )( 
            IBrowserService4 * This);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _ResizeNextBorder)
        HRESULT ( STDMETHODCALLTYPE *_ResizeNextBorder )( 
            IBrowserService4 * This,
            /* [annotation][in] */ 
            _In_  UINT itb);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _ResizeView)
        HRESULT ( STDMETHODCALLTYPE *_ResizeView )( 
            IBrowserService4 * This);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _GetEffectiveClientArea)
        HRESULT ( STDMETHODCALLTYPE *_GetEffectiveClientArea )( 
            IBrowserService4 * This,
            /* [annotation][out] */ 
            _Out_  LPRECT lprectBorder,
            /* [annotation][in] */ 
            _In_  HMONITOR hmon);
        
        DECLSPEC_XFGVIRT(IBrowserService2, v_GetViewStream)
        IStream *( STDMETHODCALLTYPE *v_GetViewStream )( 
            IBrowserService4 * This,
            /* [in] */ PCIDLIST_ABSOLUTE pidl,
            /* [annotation][in] */ 
            _In_  DWORD grfMode,
            /* [annotation][in] */ 
            _In_  LPCWSTR pwszName);
        
        DECLSPEC_XFGVIRT(IBrowserService2, ForwardViewMsg)
        LRESULT ( STDMETHODCALLTYPE *ForwardViewMsg )( 
            IBrowserService4 * This,
            /* [annotation][in] */ 
            _In_  UINT uMsg,
            /* [annotation][in] */ 
            _In_  WPARAM wParam,
            /* [annotation][in] */ 
            _In_  LPARAM lParam);
        
        DECLSPEC_XFGVIRT(IBrowserService2, SetAcceleratorMenu)
        HRESULT ( STDMETHODCALLTYPE *SetAcceleratorMenu )( 
            IBrowserService4 * This,
            /* [annotation][in] */ 
            _In_  HACCEL hacc);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _GetToolbarCount)
        int ( STDMETHODCALLTYPE *_GetToolbarCount )( 
            IBrowserService4 * This);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _GetToolbarItem)
        LPTOOLBARITEM ( STDMETHODCALLTYPE *_GetToolbarItem )( 
            IBrowserService4 * This,
            /* [annotation][in] */ 
            _In_  int itb);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _SaveToolbars)
        HRESULT ( STDMETHODCALLTYPE *_SaveToolbars )( 
            IBrowserService4 * This,
            /* [in] */ IStream *pstm);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _LoadToolbars)
        HRESULT ( STDMETHODCALLTYPE *_LoadToolbars )( 
            IBrowserService4 * This,
            /* [annotation][in] */ 
            _In_  IStream *pstm);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _CloseAndReleaseToolbars)
        HRESULT ( STDMETHODCALLTYPE *_CloseAndReleaseToolbars )( 
            IBrowserService4 * This,
            BOOL fClose);
        
        DECLSPEC_XFGVIRT(IBrowserService2, v_MayGetNextToolbarFocus)
        HRESULT ( STDMETHODCALLTYPE *v_MayGetNextToolbarFocus )( 
            IBrowserService4 * This,
            /* [annotation][in] */ 
            _In_  LPMSG lpMsg,
            /* [annotation][in] */ 
            _In_  UINT itbNext,
            /* [annotation][in] */ 
            _In_  int citb,
            /* [annotation][out] */ 
            _Out_  LPTOOLBARITEM *pptbi,
            /* [annotation][out] */ 
            _Out_  HWND *phwnd);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _ResizeNextBorderHelper)
        HRESULT ( STDMETHODCALLTYPE *_ResizeNextBorderHelper )( 
            IBrowserService4 * This,
            UINT itb,
            BOOL bUseHmonitor);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _FindTBar)
        UINT ( STDMETHODCALLTYPE *_FindTBar )( 
            IBrowserService4 * This,
            /* [annotation][in] */ 
            _In_  IUnknown *punkSrc);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _SetFocus)
        HRESULT ( STDMETHODCALLTYPE *_SetFocus )( 
            IBrowserService4 * This,
            /* [annotation][in] */ 
            _In_  LPTOOLBARITEM ptbi,
            /* [annotation][in] */ 
            _In_  HWND hwnd,
            /* [annotation][in] */ 
            _In_  LPMSG lpMsg);
        
        DECLSPEC_XFGVIRT(IBrowserService2, v_MayTranslateAccelerator)
        HRESULT ( STDMETHODCALLTYPE *v_MayTranslateAccelerator )( 
            IBrowserService4 * This,
            /* [in] */ MSG *pmsg);
        
        DECLSPEC_XFGVIRT(IBrowserService2, _GetBorderDWHelper)
        HRESULT ( STDMETHODCALLTYPE *_GetBorderDWHelper )( 
            IBrowserService4 * This,
            /* [annotation][in] */ 
            _In_  IUnknown *punkSrc,
            /* [annotation][out] */ 
            _Out_  LPRECT lprectBorder,
            /* [annotation][in] */ 
            _In_  BOOL bUseHmonitor);
        
        DECLSPEC_XFGVIRT(IBrowserService2, v_CheckZoneCrossing)
        HRESULT ( STDMETHODCALLTYPE *v_CheckZoneCrossing )( 
            IBrowserService4 * This,
            /* [annotation][in] */ 
            _In_  PCIDLIST_ABSOLUTE pidl);
        
        DECLSPEC_XFGVIRT(IBrowserService3, _PositionViewWindow)
        HRESULT ( STDMETHODCALLTYPE *_PositionViewWindow )( 
            IBrowserService4 * This,
            /* [annotation][in] */ 
            _In_  HWND hwnd,
            /* [annotation][in] */ 
            _In_  LPRECT prc);
        
        DECLSPEC_XFGVIRT(IBrowserService3, IEParseDisplayNameEx)
        HRESULT ( STDMETHODCALLTYPE *IEParseDisplayNameEx )( 
            IBrowserService4 * This,
            /* [annotation][in] */ 
            _In_  UINT uiCP,
            /* [annotation][in] */ 
            _In_  LPCWSTR pwszPath,
            /* [annotation][in] */ 
            _In_  DWORD dwFlags,
            /* [annotation][out] */ 
            _Outptr_  PIDLIST_ABSOLUTE *ppidlOut);
        
        DECLSPEC_XFGVIRT(IBrowserService4, ActivateView)
        HRESULT ( STDMETHODCALLTYPE *ActivateView )( 
            IBrowserService4 * This,
            /* [in] */ BOOL fPendingView);
        
        DECLSPEC_XFGVIRT(IBrowserService4, SaveViewState)
        HRESULT ( STDMETHODCALLTYPE *SaveViewState )( 
            IBrowserService4 * This);
        
        DECLSPEC_XFGVIRT(IBrowserService4, _ResizeAllBorders)
        HRESULT ( STDMETHODCALLTYPE *_ResizeAllBorders )( 
            IBrowserService4 * This);
        
        END_INTERFACE
    } IBrowserService4Vtbl;

    interface IBrowserService4
    {
        CONST_VTBL struct IBrowserService4Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBrowserService4_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBrowserService4_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBrowserService4_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBrowserService4_GetParentSite(This,ppipsite)	\
    ( (This)->lpVtbl -> GetParentSite(This,ppipsite) ) 

#define IBrowserService4_SetTitle(This,psv,pszName)	\
    ( (This)->lpVtbl -> SetTitle(This,psv,pszName) ) 

#define IBrowserService4_GetTitle(This,psv,pszName,cchName)	\
    ( (This)->lpVtbl -> GetTitle(This,psv,pszName,cchName) ) 

#define IBrowserService4_GetOleObject(This,ppobjv)	\
    ( (This)->lpVtbl -> GetOleObject(This,ppobjv) ) 

#define IBrowserService4_GetTravelLog(This,pptl)	\
    ( (This)->lpVtbl -> GetTravelLog(This,pptl) ) 

#define IBrowserService4_ShowControlWindow(This,id,fShow)	\
    ( (This)->lpVtbl -> ShowControlWindow(This,id,fShow) ) 

#define IBrowserService4_IsControlWindowShown(This,id,pfShown)	\
    ( (This)->lpVtbl -> IsControlWindowShown(This,id,pfShown) ) 

#define IBrowserService4_IEGetDisplayName(This,pidl,pwszName,uFlags)	\
    ( (This)->lpVtbl -> IEGetDisplayName(This,pidl,pwszName,uFlags) ) 

#define IBrowserService4_IEParseDisplayName(This,uiCP,pwszPath,ppidlOut)	\
    ( (This)->lpVtbl -> IEParseDisplayName(This,uiCP,pwszPath,ppidlOut) ) 

#define IBrowserService4_DisplayParseError(This,hres,pwszPath)	\
    ( (This)->lpVtbl -> DisplayParseError(This,hres,pwszPath) ) 

#define IBrowserService4_NavigateToPidl(This,pidl,grfHLNF)	\
    ( (This)->lpVtbl -> NavigateToPidl(This,pidl,grfHLNF) ) 

#define IBrowserService4_SetNavigateState(This,bnstate)	\
    ( (This)->lpVtbl -> SetNavigateState(This,bnstate) ) 

#define IBrowserService4_GetNavigateState(This,pbnstate)	\
    ( (This)->lpVtbl -> GetNavigateState(This,pbnstate) ) 

#define IBrowserService4_NotifyRedirect(This,psv,pidl,pfDidBrowse)	\
    ( (This)->lpVtbl -> NotifyRedirect(This,psv,pidl,pfDidBrowse) ) 

#define IBrowserService4_UpdateWindowList(This)	\
    ( (This)->lpVtbl -> UpdateWindowList(This) ) 

#define IBrowserService4_UpdateBackForwardState(This)	\
    ( (This)->lpVtbl -> UpdateBackForwardState(This) ) 

#define IBrowserService4_SetFlags(This,dwFlags,dwFlagMask)	\
    ( (This)->lpVtbl -> SetFlags(This,dwFlags,dwFlagMask) ) 

#define IBrowserService4_GetFlags(This,pdwFlags)	\
    ( (This)->lpVtbl -> GetFlags(This,pdwFlags) ) 

#define IBrowserService4_CanNavigateNow(This)	\
    ( (This)->lpVtbl -> CanNavigateNow(This) ) 

#define IBrowserService4_GetPidl(This,ppidl)	\
    ( (This)->lpVtbl -> GetPidl(This,ppidl) ) 

#define IBrowserService4_SetReferrer(This,pidl)	\
    ( (This)->lpVtbl -> SetReferrer(This,pidl) ) 

#define IBrowserService4_GetBrowserIndex(This)	\
    ( (This)->lpVtbl -> GetBrowserIndex(This) ) 

#define IBrowserService4_GetBrowserByIndex(This,dwID,ppunk)	\
    ( (This)->lpVtbl -> GetBrowserByIndex(This,dwID,ppunk) ) 

#define IBrowserService4_GetHistoryObject(This,ppole,pstm,ppbc)	\
    ( (This)->lpVtbl -> GetHistoryObject(This,ppole,pstm,ppbc) ) 

#define IBrowserService4_SetHistoryObject(This,pole,fIsLocalAnchor)	\
    ( (This)->lpVtbl -> SetHistoryObject(This,pole,fIsLocalAnchor) ) 

#define IBrowserService4_CacheOLEServer(This,pole)	\
    ( (This)->lpVtbl -> CacheOLEServer(This,pole) ) 

#define IBrowserService4_GetSetCodePage(This,pvarIn,pvarOut)	\
    ( (This)->lpVtbl -> GetSetCodePage(This,pvarIn,pvarOut) ) 

#define IBrowserService4_OnHttpEquiv(This,psv,fDone,pvarargIn,pvarargOut)	\
    ( (This)->lpVtbl -> OnHttpEquiv(This,psv,fDone,pvarargIn,pvarargOut) ) 

#define IBrowserService4_GetPalette(This,hpal)	\
    ( (This)->lpVtbl -> GetPalette(This,hpal) ) 

#define IBrowserService4_RegisterWindow(This,fForceRegister,swc)	\
    ( (This)->lpVtbl -> RegisterWindow(This,fForceRegister,swc) ) 


#define IBrowserService4_WndProcBS(This,hwnd,uMsg,wParam,lParam)	\
    ( (This)->lpVtbl -> WndProcBS(This,hwnd,uMsg,wParam,lParam) ) 

#define IBrowserService4_SetAsDefFolderSettings(This)	\
    ( (This)->lpVtbl -> SetAsDefFolderSettings(This) ) 

#define IBrowserService4_GetViewRect(This,prc)	\
    ( (This)->lpVtbl -> GetViewRect(This,prc) ) 

#define IBrowserService4_OnSize(This,wParam)	\
    ( (This)->lpVtbl -> OnSize(This,wParam) ) 

#define IBrowserService4_OnCreate(This,pcs)	\
    ( (This)->lpVtbl -> OnCreate(This,pcs) ) 

#define IBrowserService4_OnCommand(This,wParam,lParam)	\
    ( (This)->lpVtbl -> OnCommand(This,wParam,lParam) ) 

#define IBrowserService4_OnDestroy(This)	\
    ( (This)->lpVtbl -> OnDestroy(This) ) 

#define IBrowserService4_OnNotify(This,pnm)	\
    ( (This)->lpVtbl -> OnNotify(This,pnm) ) 

#define IBrowserService4_OnSetFocus(This)	\
    ( (This)->lpVtbl -> OnSetFocus(This) ) 

#define IBrowserService4_OnFrameWindowActivateBS(This,fActive)	\
    ( (This)->lpVtbl -> OnFrameWindowActivateBS(This,fActive) ) 

#define IBrowserService4_ReleaseShellView(This)	\
    ( (This)->lpVtbl -> ReleaseShellView(This) ) 

#define IBrowserService4_ActivatePendingView(This)	\
    ( (This)->lpVtbl -> ActivatePendingView(This) ) 

#define IBrowserService4_CreateViewWindow(This,psvNew,psvOld,prcView,phwnd)	\
    ( (This)->lpVtbl -> CreateViewWindow(This,psvNew,psvOld,prcView,phwnd) ) 

#define IBrowserService4_CreateBrowserPropSheetExt(This,riid,ppv)	\
    ( (This)->lpVtbl -> CreateBrowserPropSheetExt(This,riid,ppv) ) 

#define IBrowserService4_GetViewWindow(This,phwndView)	\
    ( (This)->lpVtbl -> GetViewWindow(This,phwndView) ) 

#define IBrowserService4_GetBaseBrowserData(This,pbbd)	\
    ( (This)->lpVtbl -> GetBaseBrowserData(This,pbbd) ) 

#define IBrowserService4_PutBaseBrowserData(This)	\
    ( (This)->lpVtbl -> PutBaseBrowserData(This) ) 

#define IBrowserService4_InitializeTravelLog(This,ptl,dw)	\
    ( (This)->lpVtbl -> InitializeTravelLog(This,ptl,dw) ) 

#define IBrowserService4_SetTopBrowser(This)	\
    ( (This)->lpVtbl -> SetTopBrowser(This) ) 

#define IBrowserService4_Offline(This,iCmd)	\
    ( (This)->lpVtbl -> Offline(This,iCmd) ) 

#define IBrowserService4_AllowViewResize(This,f)	\
    ( (This)->lpVtbl -> AllowViewResize(This,f) ) 

#define IBrowserService4_SetActivateState(This,u)	\
    ( (This)->lpVtbl -> SetActivateState(This,u) ) 

#define IBrowserService4_UpdateSecureLockIcon(This,eSecureLock)	\
    ( (This)->lpVtbl -> UpdateSecureLockIcon(This,eSecureLock) ) 

#define IBrowserService4_InitializeDownloadManager(This)	\
    ( (This)->lpVtbl -> InitializeDownloadManager(This) ) 

#define IBrowserService4_InitializeTransitionSite(This)	\
    ( (This)->lpVtbl -> InitializeTransitionSite(This) ) 

#define IBrowserService4__Initialize(This,hwnd,pauto)	\
    ( (This)->lpVtbl -> _Initialize(This,hwnd,pauto) ) 

#define IBrowserService4__CancelPendingNavigationAsync(This)	\
    ( (This)->lpVtbl -> _CancelPendingNavigationAsync(This) ) 

#define IBrowserService4__CancelPendingView(This)	\
    ( (This)->lpVtbl -> _CancelPendingView(This) ) 

#define IBrowserService4__MaySaveChanges(This)	\
    ( (This)->lpVtbl -> _MaySaveChanges(This) ) 

#define IBrowserService4__PauseOrResumeView(This,fPaused)	\
    ( (This)->lpVtbl -> _PauseOrResumeView(This,fPaused) ) 

#define IBrowserService4__DisableModeless(This)	\
    ( (This)->lpVtbl -> _DisableModeless(This) ) 

#define IBrowserService4__NavigateToPidl(This,pidl,grfHLNF,dwFlags)	\
    ( (This)->lpVtbl -> _NavigateToPidl(This,pidl,grfHLNF,dwFlags) ) 

#define IBrowserService4__TryShell2Rename(This,psv,pidlNew)	\
    ( (This)->lpVtbl -> _TryShell2Rename(This,psv,pidlNew) ) 

#define IBrowserService4__SwitchActivationNow(This)	\
    ( (This)->lpVtbl -> _SwitchActivationNow(This) ) 

#define IBrowserService4__ExecChildren(This,punkBar,fBroadcast,pguidCmdGroup,nCmdID,nCmdexecopt,pvarargIn,pvarargOut)	\
    ( (This)->lpVtbl -> _ExecChildren(This,punkBar,fBroadcast,pguidCmdGroup,nCmdID,nCmdexecopt,pvarargIn,pvarargOut) ) 

#define IBrowserService4__SendChildren(This,hwndBar,fBroadcast,uMsg,wParam,lParam)	\
    ( (This)->lpVtbl -> _SendChildren(This,hwndBar,fBroadcast,uMsg,wParam,lParam) ) 

#define IBrowserService4_GetFolderSetData(This,pfsd)	\
    ( (This)->lpVtbl -> GetFolderSetData(This,pfsd) ) 

#define IBrowserService4__OnFocusChange(This,itb)	\
    ( (This)->lpVtbl -> _OnFocusChange(This,itb) ) 

#define IBrowserService4_v_ShowHideChildWindows(This,fChildOnly)	\
    ( (This)->lpVtbl -> v_ShowHideChildWindows(This,fChildOnly) ) 

#define IBrowserService4__get_itbLastFocus(This)	\
    ( (This)->lpVtbl -> _get_itbLastFocus(This) ) 

#define IBrowserService4__put_itbLastFocus(This,itbLastFocus)	\
    ( (This)->lpVtbl -> _put_itbLastFocus(This,itbLastFocus) ) 

#define IBrowserService4__UIActivateView(This,uState)	\
    ( (This)->lpVtbl -> _UIActivateView(This,uState) ) 

#define IBrowserService4__GetViewBorderRect(This,prc)	\
    ( (This)->lpVtbl -> _GetViewBorderRect(This,prc) ) 

#define IBrowserService4__UpdateViewRectSize(This)	\
    ( (This)->lpVtbl -> _UpdateViewRectSize(This) ) 

#define IBrowserService4__ResizeNextBorder(This,itb)	\
    ( (This)->lpVtbl -> _ResizeNextBorder(This,itb) ) 

#define IBrowserService4__ResizeView(This)	\
    ( (This)->lpVtbl -> _ResizeView(This) ) 

#define IBrowserService4__GetEffectiveClientArea(This,lprectBorder,hmon)	\
    ( (This)->lpVtbl -> _GetEffectiveClientArea(This,lprectBorder,hmon) ) 

#define IBrowserService4_v_GetViewStream(This,pidl,grfMode,pwszName)	\
    ( (This)->lpVtbl -> v_GetViewStream(This,pidl,grfMode,pwszName) ) 

#define IBrowserService4_ForwardViewMsg(This,uMsg,wParam,lParam)	\
    ( (This)->lpVtbl -> ForwardViewMsg(This,uMsg,wParam,lParam) ) 

#define IBrowserService4_SetAcceleratorMenu(This,hacc)	\
    ( (This)->lpVtbl -> SetAcceleratorMenu(This,hacc) ) 

#define IBrowserService4__GetToolbarCount(This)	\
    ( (This)->lpVtbl -> _GetToolbarCount(This) ) 

#define IBrowserService4__GetToolbarItem(This,itb)	\
    ( (This)->lpVtbl -> _GetToolbarItem(This,itb) ) 

#define IBrowserService4__SaveToolbars(This,pstm)	\
    ( (This)->lpVtbl -> _SaveToolbars(This,pstm) ) 

#define IBrowserService4__LoadToolbars(This,pstm)	\
    ( (This)->lpVtbl -> _LoadToolbars(This,pstm) ) 

#define IBrowserService4__CloseAndReleaseToolbars(This,fClose)	\
    ( (This)->lpVtbl -> _CloseAndReleaseToolbars(This,fClose) ) 

#define IBrowserService4_v_MayGetNextToolbarFocus(This,lpMsg,itbNext,citb,pptbi,phwnd)	\
    ( (This)->lpVtbl -> v_MayGetNextToolbarFocus(This,lpMsg,itbNext,citb,pptbi,phwnd) ) 

#define IBrowserService4__ResizeNextBorderHelper(This,itb,bUseHmonitor)	\
    ( (This)->lpVtbl -> _ResizeNextBorderHelper(This,itb,bUseHmonitor) ) 

#define IBrowserService4__FindTBar(This,punkSrc)	\
    ( (This)->lpVtbl -> _FindTBar(This,punkSrc) ) 

#define IBrowserService4__SetFocus(This,ptbi,hwnd,lpMsg)	\
    ( (This)->lpVtbl -> _SetFocus(This,ptbi,hwnd,lpMsg) ) 

#define IBrowserService4_v_MayTranslateAccelerator(This,pmsg)	\
    ( (This)->lpVtbl -> v_MayTranslateAccelerator(This,pmsg) ) 

#define IBrowserService4__GetBorderDWHelper(This,punkSrc,lprectBorder,bUseHmonitor)	\
    ( (This)->lpVtbl -> _GetBorderDWHelper(This,punkSrc,lprectBorder,bUseHmonitor) ) 

#define IBrowserService4_v_CheckZoneCrossing(This,pidl)	\
    ( (This)->lpVtbl -> v_CheckZoneCrossing(This,pidl) ) 


#define IBrowserService4__PositionViewWindow(This,hwnd,prc)	\
    ( (This)->lpVtbl -> _PositionViewWindow(This,hwnd,prc) ) 

#define IBrowserService4_IEParseDisplayNameEx(This,uiCP,pwszPath,dwFlags,ppidlOut)	\
    ( (This)->lpVtbl -> IEParseDisplayNameEx(This,uiCP,pwszPath,dwFlags,ppidlOut) ) 


#define IBrowserService4_ActivateView(This,fPendingView)	\
    ( (This)->lpVtbl -> ActivateView(This,fPendingView) ) 

#define IBrowserService4_SaveViewState(This)	\
    ( (This)->lpVtbl -> SaveViewState(This) ) 

#define IBrowserService4__ResizeAllBorders(This)	\
    ( (This)->lpVtbl -> _ResizeAllBorders(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBrowserService4_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_shdeprecated_0000_0009 */
/* [local] */ 

#endif // (NTDDI_VERSION >= NTDDI_VISTA) || (_WIN32_IE >= _WIN32_IE_IE70)
#if (_WIN32_IE >= _WIN32_IE_IE60)


extern RPC_IF_HANDLE __MIDL_itf_shdeprecated_0000_0009_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_shdeprecated_0000_0009_v0_0_s_ifspec;

#ifndef __ITrackShellMenu_INTERFACE_DEFINED__
#define __ITrackShellMenu_INTERFACE_DEFINED__

/* interface ITrackShellMenu */
/* [local][unique][object][uuid] */ 


EXTERN_C const IID IID_ITrackShellMenu;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8278F932-2A3E-11d2-838F-00C04FD918D0")
    ITrackShellMenu : public IShellMenu
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetObscured( 
            /* [in] */ HWND hwndTB,
            /* [in] */ IUnknown *punkBand,
            DWORD dwSMSetFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Popup( 
            /* [in] */ HWND hwnd,
            /* [in] */ POINTL *ppt,
            /* [in] */ RECTL *prcExclude,
            MP_POPUPFLAGS dwFlags) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITrackShellMenuVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ITrackShellMenu * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ITrackShellMenu * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ITrackShellMenu * This);
        
        DECLSPEC_XFGVIRT(IShellMenu, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            ITrackShellMenu * This,
            /* [annotation][unique][in] */ 
            _In_opt_  IShellMenuCallback *psmc,
            /* [annotation][in] */ 
            _In_  UINT uId,
            /* [annotation][in] */ 
            _In_  UINT uIdAncestor,
            /* [annotation][in] */ 
            _In_  DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IShellMenu, GetMenuInfo)
        HRESULT ( STDMETHODCALLTYPE *GetMenuInfo )( 
            ITrackShellMenu * This,
            /* [annotation][out] */ 
            _Outptr_opt_  IShellMenuCallback **ppsmc,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puId,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puIdAncestor,
            /* [annotation][out] */ 
            _Out_opt_  DWORD *pdwFlags);
        
        DECLSPEC_XFGVIRT(IShellMenu, SetShellFolder)
        HRESULT ( STDMETHODCALLTYPE *SetShellFolder )( 
            ITrackShellMenu * This,
            /* [annotation][unique][in] */ 
            _In_opt_  IShellFolder *psf,
            /* [annotation][unique][in] */ 
            _In_opt_  PCIDLIST_ABSOLUTE pidlFolder,
            /* [annotation][unique][in] */ 
            _In_opt_  HKEY hKey,
            /* [annotation][in] */ 
            _In_  DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IShellMenu, GetShellFolder)
        HRESULT ( STDMETHODCALLTYPE *GetShellFolder )( 
            ITrackShellMenu * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwFlags,
            /* [annotation][out] */ 
            _Outptr_  PIDLIST_ABSOLUTE *ppidl,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _Outptr_  void **ppv);
        
        DECLSPEC_XFGVIRT(IShellMenu, SetMenu)
        HRESULT ( STDMETHODCALLTYPE *SetMenu )( 
            ITrackShellMenu * This,
            /* [annotation][unique][in] */ 
            _In_opt_  HMENU hmenu,
            /* [annotation][unique][in] */ 
            _In_opt_  HWND hwnd,
            /* [annotation][in] */ 
            _In_  DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IShellMenu, GetMenu)
        HRESULT ( STDMETHODCALLTYPE *GetMenu )( 
            ITrackShellMenu * This,
            /* [annotation][out] */ 
            _Out_opt_  HMENU *phmenu,
            /* [annotation][out] */ 
            _Out_opt_  HWND *phwnd,
            /* [annotation][out] */ 
            _Out_opt_  DWORD *pdwFlags);
        
        DECLSPEC_XFGVIRT(IShellMenu, InvalidateItem)
        HRESULT ( STDMETHODCALLTYPE *InvalidateItem )( 
            ITrackShellMenu * This,
            /* [annotation][in] */ 
            _In_opt_  LPSMDATA psmd,
            /* [annotation][in] */ 
            _In_  DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IShellMenu, GetState)
        HRESULT ( STDMETHODCALLTYPE *GetState )( 
            ITrackShellMenu * This,
            /* [annotation][out] */ 
            _Out_  LPSMDATA psmd);
        
        DECLSPEC_XFGVIRT(IShellMenu, SetMenuToolbar)
        HRESULT ( STDMETHODCALLTYPE *SetMenuToolbar )( 
            ITrackShellMenu * This,
            /* [annotation][in] */ 
            _In_  IUnknown *punk,
            /* [annotation][in] */ 
            _In_  DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(ITrackShellMenu, SetObscured)
        HRESULT ( STDMETHODCALLTYPE *SetObscured )( 
            ITrackShellMenu * This,
            /* [in] */ HWND hwndTB,
            /* [in] */ IUnknown *punkBand,
            DWORD dwSMSetFlags);
        
        DECLSPEC_XFGVIRT(ITrackShellMenu, Popup)
        HRESULT ( STDMETHODCALLTYPE *Popup )( 
            ITrackShellMenu * This,
            /* [in] */ HWND hwnd,
            /* [in] */ POINTL *ppt,
            /* [in] */ RECTL *prcExclude,
            MP_POPUPFLAGS dwFlags);
        
        END_INTERFACE
    } ITrackShellMenuVtbl;

    interface ITrackShellMenu
    {
        CONST_VTBL struct ITrackShellMenuVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITrackShellMenu_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITrackShellMenu_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITrackShellMenu_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITrackShellMenu_Initialize(This,psmc,uId,uIdAncestor,dwFlags)	\
    ( (This)->lpVtbl -> Initialize(This,psmc,uId,uIdAncestor,dwFlags) ) 

#define ITrackShellMenu_GetMenuInfo(This,ppsmc,puId,puIdAncestor,pdwFlags)	\
    ( (This)->lpVtbl -> GetMenuInfo(This,ppsmc,puId,puIdAncestor,pdwFlags) ) 

#define ITrackShellMenu_SetShellFolder(This,psf,pidlFolder,hKey,dwFlags)	\
    ( (This)->lpVtbl -> SetShellFolder(This,psf,pidlFolder,hKey,dwFlags) ) 

#define ITrackShellMenu_GetShellFolder(This,pdwFlags,ppidl,riid,ppv)	\
    ( (This)->lpVtbl -> GetShellFolder(This,pdwFlags,ppidl,riid,ppv) ) 

#define ITrackShellMenu_SetMenu(This,hmenu,hwnd,dwFlags)	\
    ( (This)->lpVtbl -> SetMenu(This,hmenu,hwnd,dwFlags) ) 

#define ITrackShellMenu_GetMenu(This,phmenu,phwnd,pdwFlags)	\
    ( (This)->lpVtbl -> GetMenu(This,phmenu,phwnd,pdwFlags) ) 

#define ITrackShellMenu_InvalidateItem(This,psmd,dwFlags)	\
    ( (This)->lpVtbl -> InvalidateItem(This,psmd,dwFlags) ) 

#define ITrackShellMenu_GetState(This,psmd)	\
    ( (This)->lpVtbl -> GetState(This,psmd) ) 

#define ITrackShellMenu_SetMenuToolbar(This,punk,dwFlags)	\
    ( (This)->lpVtbl -> SetMenuToolbar(This,punk,dwFlags) ) 


#define ITrackShellMenu_SetObscured(This,hwndTB,punkBand,dwSMSetFlags)	\
    ( (This)->lpVtbl -> SetObscured(This,hwndTB,punkBand,dwSMSetFlags) ) 

#define ITrackShellMenu_Popup(This,hwnd,ppt,prcExclude,dwFlags)	\
    ( (This)->lpVtbl -> Popup(This,hwnd,ppt,prcExclude,dwFlags) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITrackShellMenu_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_shdeprecated_0000_0010 */
/* [local] */ 

#endif  // _WIN32_IE_IE60


extern RPC_IF_HANDLE __MIDL_itf_shdeprecated_0000_0010_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_shdeprecated_0000_0010_v0_0_s_ifspec;


#ifndef __ShellDeprecatedObjects_LIBRARY_DEFINED__
#define __ShellDeprecatedObjects_LIBRARY_DEFINED__

/* library ShellDeprecatedObjects */
/* [uuid] */ 


EXTERN_C const IID LIBID_ShellDeprecatedObjects;

EXTERN_C const CLSID CLSID_TrackShellMenu;

#ifdef __cplusplus

class DECLSPEC_UUID("8278F931-2A3E-11d2-838F-00C04FD918D0")
TrackShellMenu;
#endif
#endif /* __ShellDeprecatedObjects_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_shdeprecated_0000_0011 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_shdeprecated_0000_0011_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_shdeprecated_0000_0011_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  HMENU_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in HMENU * ); 
unsigned char * __RPC_USER  HMENU_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HMENU * ); 
unsigned char * __RPC_USER  HMENU_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HMENU * ); 
void                      __RPC_USER  HMENU_UserFree(     __RPC__in unsigned long *, __RPC__in HMENU * ); 

unsigned long             __RPC_USER  PCIDLIST_ABSOLUTE_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in PCIDLIST_ABSOLUTE * ); 
unsigned char * __RPC_USER  PCIDLIST_ABSOLUTE_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in PCIDLIST_ABSOLUTE * ); 
unsigned char * __RPC_USER  PCIDLIST_ABSOLUTE_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out PCIDLIST_ABSOLUTE * ); 
void                      __RPC_USER  PCIDLIST_ABSOLUTE_UserFree(     __RPC__in unsigned long *, __RPC__in PCIDLIST_ABSOLUTE * ); 

unsigned long             __RPC_USER  PIDLIST_ABSOLUTE_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in PIDLIST_ABSOLUTE * ); 
unsigned char * __RPC_USER  PIDLIST_ABSOLUTE_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in PIDLIST_ABSOLUTE * ); 
unsigned char * __RPC_USER  PIDLIST_ABSOLUTE_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out PIDLIST_ABSOLUTE * ); 
void                      __RPC_USER  PIDLIST_ABSOLUTE_UserFree(     __RPC__in unsigned long *, __RPC__in PIDLIST_ABSOLUTE * ); 

unsigned long             __RPC_USER  HMENU_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in HMENU * ); 
unsigned char * __RPC_USER  HMENU_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HMENU * ); 
unsigned char * __RPC_USER  HMENU_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HMENU * ); 
void                      __RPC_USER  HMENU_UserFree64(     __RPC__in unsigned long *, __RPC__in HMENU * ); 

unsigned long             __RPC_USER  PCIDLIST_ABSOLUTE_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in PCIDLIST_ABSOLUTE * ); 
unsigned char * __RPC_USER  PCIDLIST_ABSOLUTE_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in PCIDLIST_ABSOLUTE * ); 
unsigned char * __RPC_USER  PCIDLIST_ABSOLUTE_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out PCIDLIST_ABSOLUTE * ); 
void                      __RPC_USER  PCIDLIST_ABSOLUTE_UserFree64(     __RPC__in unsigned long *, __RPC__in PCIDLIST_ABSOLUTE * ); 

unsigned long             __RPC_USER  PIDLIST_ABSOLUTE_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in PIDLIST_ABSOLUTE * ); 
unsigned char * __RPC_USER  PIDLIST_ABSOLUTE_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in PIDLIST_ABSOLUTE * ); 
unsigned char * __RPC_USER  PIDLIST_ABSOLUTE_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out PIDLIST_ABSOLUTE * ); 
void                      __RPC_USER  PIDLIST_ABSOLUTE_UserFree64(     __RPC__in unsigned long *, __RPC__in PIDLIST_ABSOLUTE * ); 

/* [local][helpcontext][helpstring] */ HRESULT STDMETHODCALLTYPE ITravelLog_GetTravelEntry_Proxy( 
    ITravelLog * This,
    /* [annotation][in] */ 
    _In_  IUnknown *punk,
    /* [annotation][in] */ 
    _In_  int iOffset,
    /* [annotation][out] */ 
    _Outptr_opt_  ITravelEntry **ppte);


/* [call_as] */ HRESULT STDMETHODCALLTYPE ITravelLog_GetTravelEntry_Stub( 
    __RPC__in ITravelLog * This,
    /* [in] */ __RPC__in_opt IUnknown *punk,
    /* [in] */ int iOffset,
    /* [out] */ __RPC__deref_out_opt ITravelEntry **ppte);



/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


