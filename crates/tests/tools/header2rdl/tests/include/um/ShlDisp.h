

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

#ifndef __shldisp_h__
#define __shldisp_h__

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

#ifndef __IFolderViewOC_FWD_DEFINED__
#define __IFolderViewOC_FWD_DEFINED__
typedef interface IFolderViewOC IFolderViewOC;

#endif 	/* __IFolderViewOC_FWD_DEFINED__ */


#ifndef __DShellFolderViewEvents_FWD_DEFINED__
#define __DShellFolderViewEvents_FWD_DEFINED__
typedef interface DShellFolderViewEvents DShellFolderViewEvents;

#endif 	/* __DShellFolderViewEvents_FWD_DEFINED__ */


#ifndef __ShellFolderViewOC_FWD_DEFINED__
#define __ShellFolderViewOC_FWD_DEFINED__

#ifdef __cplusplus
typedef class ShellFolderViewOC ShellFolderViewOC;
#else
typedef struct ShellFolderViewOC ShellFolderViewOC;
#endif /* __cplusplus */

#endif 	/* __ShellFolderViewOC_FWD_DEFINED__ */


#ifndef __DFConstraint_FWD_DEFINED__
#define __DFConstraint_FWD_DEFINED__
typedef interface DFConstraint DFConstraint;

#endif 	/* __DFConstraint_FWD_DEFINED__ */


#ifndef __FolderItem_FWD_DEFINED__
#define __FolderItem_FWD_DEFINED__
typedef interface FolderItem FolderItem;

#endif 	/* __FolderItem_FWD_DEFINED__ */


#ifndef __FolderItems_FWD_DEFINED__
#define __FolderItems_FWD_DEFINED__
typedef interface FolderItems FolderItems;

#endif 	/* __FolderItems_FWD_DEFINED__ */


#ifndef __FolderItemVerb_FWD_DEFINED__
#define __FolderItemVerb_FWD_DEFINED__
typedef interface FolderItemVerb FolderItemVerb;

#endif 	/* __FolderItemVerb_FWD_DEFINED__ */


#ifndef __FolderItemVerbs_FWD_DEFINED__
#define __FolderItemVerbs_FWD_DEFINED__
typedef interface FolderItemVerbs FolderItemVerbs;

#endif 	/* __FolderItemVerbs_FWD_DEFINED__ */


#ifndef __Folder_FWD_DEFINED__
#define __Folder_FWD_DEFINED__
typedef interface Folder Folder;

#endif 	/* __Folder_FWD_DEFINED__ */


#ifndef __Folder2_FWD_DEFINED__
#define __Folder2_FWD_DEFINED__
typedef interface Folder2 Folder2;

#endif 	/* __Folder2_FWD_DEFINED__ */


#ifndef __Folder3_FWD_DEFINED__
#define __Folder3_FWD_DEFINED__
typedef interface Folder3 Folder3;

#endif 	/* __Folder3_FWD_DEFINED__ */


#ifndef __FolderItem2_FWD_DEFINED__
#define __FolderItem2_FWD_DEFINED__
typedef interface FolderItem2 FolderItem2;

#endif 	/* __FolderItem2_FWD_DEFINED__ */


#ifndef __ShellFolderItem_FWD_DEFINED__
#define __ShellFolderItem_FWD_DEFINED__

#ifdef __cplusplus
typedef class ShellFolderItem ShellFolderItem;
#else
typedef struct ShellFolderItem ShellFolderItem;
#endif /* __cplusplus */

#endif 	/* __ShellFolderItem_FWD_DEFINED__ */


#ifndef __FolderItems2_FWD_DEFINED__
#define __FolderItems2_FWD_DEFINED__
typedef interface FolderItems2 FolderItems2;

#endif 	/* __FolderItems2_FWD_DEFINED__ */


#ifndef __FolderItems3_FWD_DEFINED__
#define __FolderItems3_FWD_DEFINED__
typedef interface FolderItems3 FolderItems3;

#endif 	/* __FolderItems3_FWD_DEFINED__ */


#ifndef __IShellLinkDual_FWD_DEFINED__
#define __IShellLinkDual_FWD_DEFINED__
typedef interface IShellLinkDual IShellLinkDual;

#endif 	/* __IShellLinkDual_FWD_DEFINED__ */


#ifndef __IShellLinkDual2_FWD_DEFINED__
#define __IShellLinkDual2_FWD_DEFINED__
typedef interface IShellLinkDual2 IShellLinkDual2;

#endif 	/* __IShellLinkDual2_FWD_DEFINED__ */


#ifndef __ShellLinkObject_FWD_DEFINED__
#define __ShellLinkObject_FWD_DEFINED__

#ifdef __cplusplus
typedef class ShellLinkObject ShellLinkObject;
#else
typedef struct ShellLinkObject ShellLinkObject;
#endif /* __cplusplus */

#endif 	/* __ShellLinkObject_FWD_DEFINED__ */


#ifndef __IShellFolderViewDual_FWD_DEFINED__
#define __IShellFolderViewDual_FWD_DEFINED__
typedef interface IShellFolderViewDual IShellFolderViewDual;

#endif 	/* __IShellFolderViewDual_FWD_DEFINED__ */


#ifndef __IShellFolderViewDual2_FWD_DEFINED__
#define __IShellFolderViewDual2_FWD_DEFINED__
typedef interface IShellFolderViewDual2 IShellFolderViewDual2;

#endif 	/* __IShellFolderViewDual2_FWD_DEFINED__ */


#ifndef __IShellFolderViewDual3_FWD_DEFINED__
#define __IShellFolderViewDual3_FWD_DEFINED__
typedef interface IShellFolderViewDual3 IShellFolderViewDual3;

#endif 	/* __IShellFolderViewDual3_FWD_DEFINED__ */


#ifndef __ShellFolderView_FWD_DEFINED__
#define __ShellFolderView_FWD_DEFINED__

#ifdef __cplusplus
typedef class ShellFolderView ShellFolderView;
#else
typedef struct ShellFolderView ShellFolderView;
#endif /* __cplusplus */

#endif 	/* __ShellFolderView_FWD_DEFINED__ */


#ifndef __IShellDispatch_FWD_DEFINED__
#define __IShellDispatch_FWD_DEFINED__
typedef interface IShellDispatch IShellDispatch;

#endif 	/* __IShellDispatch_FWD_DEFINED__ */


#ifndef __IShellDispatch2_FWD_DEFINED__
#define __IShellDispatch2_FWD_DEFINED__
typedef interface IShellDispatch2 IShellDispatch2;

#endif 	/* __IShellDispatch2_FWD_DEFINED__ */


#ifndef __IShellDispatch3_FWD_DEFINED__
#define __IShellDispatch3_FWD_DEFINED__
typedef interface IShellDispatch3 IShellDispatch3;

#endif 	/* __IShellDispatch3_FWD_DEFINED__ */


#ifndef __IShellDispatch4_FWD_DEFINED__
#define __IShellDispatch4_FWD_DEFINED__
typedef interface IShellDispatch4 IShellDispatch4;

#endif 	/* __IShellDispatch4_FWD_DEFINED__ */


#ifndef __IShellDispatch5_FWD_DEFINED__
#define __IShellDispatch5_FWD_DEFINED__
typedef interface IShellDispatch5 IShellDispatch5;

#endif 	/* __IShellDispatch5_FWD_DEFINED__ */


#ifndef __IShellDispatch6_FWD_DEFINED__
#define __IShellDispatch6_FWD_DEFINED__
typedef interface IShellDispatch6 IShellDispatch6;

#endif 	/* __IShellDispatch6_FWD_DEFINED__ */


#ifndef __Shell_FWD_DEFINED__
#define __Shell_FWD_DEFINED__

#ifdef __cplusplus
typedef class Shell Shell;
#else
typedef struct Shell Shell;
#endif /* __cplusplus */

#endif 	/* __Shell_FWD_DEFINED__ */


#ifndef __ShellDispatchInproc_FWD_DEFINED__
#define __ShellDispatchInproc_FWD_DEFINED__

#ifdef __cplusplus
typedef class ShellDispatchInproc ShellDispatchInproc;
#else
typedef struct ShellDispatchInproc ShellDispatchInproc;
#endif /* __cplusplus */

#endif 	/* __ShellDispatchInproc_FWD_DEFINED__ */


#ifndef __IFileSearchBand_FWD_DEFINED__
#define __IFileSearchBand_FWD_DEFINED__
typedef interface IFileSearchBand IFileSearchBand;

#endif 	/* __IFileSearchBand_FWD_DEFINED__ */


#ifndef __FileSearchBand_FWD_DEFINED__
#define __FileSearchBand_FWD_DEFINED__

#ifdef __cplusplus
typedef class FileSearchBand FileSearchBand;
#else
typedef struct FileSearchBand FileSearchBand;
#endif /* __cplusplus */

#endif 	/* __FileSearchBand_FWD_DEFINED__ */


#ifndef __IWebWizardHost_FWD_DEFINED__
#define __IWebWizardHost_FWD_DEFINED__
typedef interface IWebWizardHost IWebWizardHost;

#endif 	/* __IWebWizardHost_FWD_DEFINED__ */


#ifndef __IWebWizardHost2_FWD_DEFINED__
#define __IWebWizardHost2_FWD_DEFINED__
typedef interface IWebWizardHost2 IWebWizardHost2;

#endif 	/* __IWebWizardHost2_FWD_DEFINED__ */


#ifndef __INewWDEvents_FWD_DEFINED__
#define __INewWDEvents_FWD_DEFINED__
typedef interface INewWDEvents INewWDEvents;

#endif 	/* __INewWDEvents_FWD_DEFINED__ */


#ifndef __IAutoComplete_FWD_DEFINED__
#define __IAutoComplete_FWD_DEFINED__
typedef interface IAutoComplete IAutoComplete;

#endif 	/* __IAutoComplete_FWD_DEFINED__ */


#ifndef __IAutoComplete2_FWD_DEFINED__
#define __IAutoComplete2_FWD_DEFINED__
typedef interface IAutoComplete2 IAutoComplete2;

#endif 	/* __IAutoComplete2_FWD_DEFINED__ */


#ifndef __IEnumACString_FWD_DEFINED__
#define __IEnumACString_FWD_DEFINED__
typedef interface IEnumACString IEnumACString;

#endif 	/* __IEnumACString_FWD_DEFINED__ */


#ifndef __IDataObjectAsyncCapability_FWD_DEFINED__
#define __IDataObjectAsyncCapability_FWD_DEFINED__
typedef interface IDataObjectAsyncCapability IDataObjectAsyncCapability;

#endif 	/* __IDataObjectAsyncCapability_FWD_DEFINED__ */


/* header files for imported files */
#include "ocidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_shldisp_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#pragma once
#pragma warning(push)
#pragma warning(disable:4001) 
#pragma once
#pragma warning(push)
#pragma warning(disable:4001) 
#pragma once
#pragma warning(pop)
#pragma warning(pop)
#pragma region Desktop Family
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_shldisp_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_shldisp_0000_0000_v0_0_s_ifspec;


#ifndef __Shell32_LIBRARY_DEFINED__
#define __Shell32_LIBRARY_DEFINED__

/* library Shell32 */
/* [version][lcid][helpstring][uuid] */ 





typedef /* [helpstring][uuid] */  DECLSPEC_UUID("35f1a0d0-3e9a-11d2-8499-005345000000") 
enum OfflineFolderStatus
    {
        OFS_INACTIVE	= -1,
        OFS_ONLINE	= ( OFS_INACTIVE + 1 ) ,
        OFS_OFFLINE	= ( OFS_ONLINE + 1 ) ,
        OFS_SERVERBACK	= ( OFS_OFFLINE + 1 ) ,
        OFS_DIRTYCACHE	= ( OFS_SERVERBACK + 1 ) 
    } 	OfflineFolderStatus;

typedef /* [helpstring][uuid] */  DECLSPEC_UUID("742A99A0-C77E-11D0-A32C-00A0C91EEDBA") 
enum ShellFolderViewOptions
    {
        SFVVO_SHOWALLOBJECTS	= 0x1,
        SFVVO_SHOWEXTENSIONS	= 0x2,
        SFVVO_SHOWCOMPCOLOR	= 0x8,
        SFVVO_SHOWSYSFILES	= 0x20,
        SFVVO_WIN95CLASSIC	= 0x40,
        SFVVO_DOUBLECLICKINWEBVIEW	= 0x80,
        SFVVO_DESKTOPHTML	= 0x200
    } 	ShellFolderViewOptions;

#if (NTDDI_VERSION >= NTDDI_WIN8)
#endif // NTDDI_WIN8
typedef /* [helpstring][uuid] */  DECLSPEC_UUID("CA31EA20-48D0-11CF-8350-444553540000") 
enum ShellSpecialFolderConstants
    {
        ssfDESKTOP	= 0,
        ssfPROGRAMS	= 0x2,
        ssfCONTROLS	= 0x3,
        ssfPRINTERS	= 0x4,
        ssfPERSONAL	= 0x5,
        ssfFAVORITES	= 0x6,
        ssfSTARTUP	= 0x7,
        ssfRECENT	= 0x8,
        ssfSENDTO	= 0x9,
        ssfBITBUCKET	= 0xa,
        ssfSTARTMENU	= 0xb,
        ssfDESKTOPDIRECTORY	= 0x10,
        ssfDRIVES	= 0x11,
        ssfNETWORK	= 0x12,
        ssfNETHOOD	= 0x13,
        ssfFONTS	= 0x14,
        ssfTEMPLATES	= 0x15,
        ssfCOMMONSTARTMENU	= 0x16,
        ssfCOMMONPROGRAMS	= 0x17,
        ssfCOMMONSTARTUP	= 0x18,
        ssfCOMMONDESKTOPDIR	= 0x19,
        ssfAPPDATA	= 0x1a,
        ssfPRINTHOOD	= 0x1b,
        ssfLOCALAPPDATA	= 0x1c,
        ssfALTSTARTUP	= 0x1d,
        ssfCOMMONALTSTARTUP	= 0x1e,
        ssfCOMMONFAVORITES	= 0x1f,
        ssfINTERNETCACHE	= 0x20,
        ssfCOOKIES	= 0x21,
        ssfHISTORY	= 0x22,
        ssfCOMMONAPPDATA	= 0x23,
        ssfWINDOWS	= 0x24,
        ssfSYSTEM	= 0x25,
        ssfPROGRAMFILES	= 0x26,
        ssfMYPICTURES	= 0x27,
        ssfPROFILE	= 0x28,
        ssfSYSTEMx86	= 0x29,
        ssfPROGRAMFILESx86	= 0x30
    } 	ShellSpecialFolderConstants;


EXTERN_C const IID LIBID_Shell32;

#ifndef __IFolderViewOC_INTERFACE_DEFINED__
#define __IFolderViewOC_INTERFACE_DEFINED__

/* interface IFolderViewOC */
/* [object][dual][oleautomation][hidden][helpcontext][helpstring][uuid] */ 


EXTERN_C const IID IID_IFolderViewOC;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9BA05970-F6A8-11CF-A442-00A0C90A8F39")
    IFolderViewOC : public IDispatch
    {
    public:
        virtual /* [helpcontext][helpstring] */ HRESULT STDMETHODCALLTYPE SetFolderView( 
            /* [in] */ __RPC__in_opt IDispatch *pdisp) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFolderViewOCVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFolderViewOC * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFolderViewOC * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFolderViewOC * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFolderViewOC * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFolderViewOC * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFolderViewOC * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFolderViewOC * This,
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
        
        DECLSPEC_XFGVIRT(IFolderViewOC, SetFolderView)
        /* [helpcontext][helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetFolderView )( 
            __RPC__in IFolderViewOC * This,
            /* [in] */ __RPC__in_opt IDispatch *pdisp);
        
        END_INTERFACE
    } IFolderViewOCVtbl;

    interface IFolderViewOC
    {
        CONST_VTBL struct IFolderViewOCVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFolderViewOC_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFolderViewOC_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFolderViewOC_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFolderViewOC_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFolderViewOC_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFolderViewOC_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFolderViewOC_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFolderViewOC_SetFolderView(This,pdisp)	\
    ( (This)->lpVtbl -> SetFolderView(This,pdisp) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFolderViewOC_INTERFACE_DEFINED__ */


#ifndef __DShellFolderViewEvents_DISPINTERFACE_DEFINED__
#define __DShellFolderViewEvents_DISPINTERFACE_DEFINED__

/* dispinterface DShellFolderViewEvents */
/* [helpstring][uuid] */ 


EXTERN_C const IID DIID_DShellFolderViewEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)

    MIDL_INTERFACE("62112AA2-EBE4-11cf-A5FB-0020AFE7292D")
    DShellFolderViewEvents : public IDispatch
    {
    };
    
#else 	/* C style interface */

    typedef struct DShellFolderViewEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in DShellFolderViewEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in DShellFolderViewEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in DShellFolderViewEvents * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in DShellFolderViewEvents * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in DShellFolderViewEvents * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in DShellFolderViewEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            DShellFolderViewEvents * This,
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
        
        END_INTERFACE
    } DShellFolderViewEventsVtbl;

    interface DShellFolderViewEvents
    {
        CONST_VTBL struct DShellFolderViewEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define DShellFolderViewEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define DShellFolderViewEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define DShellFolderViewEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define DShellFolderViewEvents_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define DShellFolderViewEvents_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define DShellFolderViewEvents_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define DShellFolderViewEvents_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */


#endif 	/* __DShellFolderViewEvents_DISPINTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_ShellFolderViewOC;

#ifdef __cplusplus

class DECLSPEC_UUID("9BA05971-F6A8-11CF-A442-00A0C90A8F39")
ShellFolderViewOC;
#endif

#ifndef __DFConstraint_INTERFACE_DEFINED__
#define __DFConstraint_INTERFACE_DEFINED__

/* interface DFConstraint */
/* [object][dual][oleautomation][helpstring][uuid] */ 


EXTERN_C const IID IID_DFConstraint;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4a3df050-23bd-11d2-939f-00a0c91eedba")
    DFConstraint : public IDispatch
    {
    public:
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbs) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Value( 
            /* [retval][out] */ __RPC__out VARIANT *pv) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct DFConstraintVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in DFConstraint * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in DFConstraint * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in DFConstraint * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in DFConstraint * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in DFConstraint * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in DFConstraint * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            DFConstraint * This,
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
        
        DECLSPEC_XFGVIRT(DFConstraint, get_Name)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in DFConstraint * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbs);
        
        DECLSPEC_XFGVIRT(DFConstraint, get_Value)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Value )( 
            __RPC__in DFConstraint * This,
            /* [retval][out] */ __RPC__out VARIANT *pv);
        
        END_INTERFACE
    } DFConstraintVtbl;

    interface DFConstraint
    {
        CONST_VTBL struct DFConstraintVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define DFConstraint_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define DFConstraint_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define DFConstraint_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define DFConstraint_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define DFConstraint_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define DFConstraint_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define DFConstraint_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define DFConstraint_get_Name(This,pbs)	\
    ( (This)->lpVtbl -> get_Name(This,pbs) ) 

#define DFConstraint_get_Value(This,pv)	\
    ( (This)->lpVtbl -> get_Value(This,pv) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __DFConstraint_INTERFACE_DEFINED__ */


#ifndef __FolderItem_INTERFACE_DEFINED__
#define __FolderItem_INTERFACE_DEFINED__

/* interface FolderItem */
/* [object][dual][oleautomation][helpstring][uuid] */ 

typedef /* [unique] */  __RPC_unique_pointer FolderItem *LPFOLDERITEM;


EXTERN_C const IID IID_FolderItem;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("FAC32C80-CBE4-11CE-8350-444553540000")
    FolderItem : public IDispatch
    {
    public:
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Application( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppid) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Parent( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppid) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbs) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Name( 
            /* [in] */ __RPC__in BSTR bs) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Path( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbs) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_GetLink( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppid) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_GetFolder( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppid) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_IsLink( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pb) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_IsFolder( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pb) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_IsFileSystem( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pb) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_IsBrowsable( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pb) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_ModifyDate( 
            /* [retval][out] */ __RPC__out DATE *pdt) = 0;
        
        virtual /* [helpstring][propput] */ HRESULT STDMETHODCALLTYPE put_ModifyDate( 
            /* [in] */ DATE dt) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Size( 
            /* [retval][out] */ __RPC__out LONG *pul) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Type( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbs) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Verbs( 
            /* [retval][out] */ __RPC__deref_out_opt FolderItemVerbs **ppfic) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE InvokeVerb( 
            /* [optional][in] */ VARIANT vVerb) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct FolderItemVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in FolderItem * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in FolderItem * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in FolderItem * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in FolderItem * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in FolderItem * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in FolderItem * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            FolderItem * This,
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
        
        DECLSPEC_XFGVIRT(FolderItem, get_Application)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Application )( 
            __RPC__in FolderItem * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppid);
        
        DECLSPEC_XFGVIRT(FolderItem, get_Parent)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Parent )( 
            __RPC__in FolderItem * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppid);
        
        DECLSPEC_XFGVIRT(FolderItem, get_Name)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in FolderItem * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbs);
        
        DECLSPEC_XFGVIRT(FolderItem, put_Name)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Name )( 
            __RPC__in FolderItem * This,
            /* [in] */ __RPC__in BSTR bs);
        
        DECLSPEC_XFGVIRT(FolderItem, get_Path)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Path )( 
            __RPC__in FolderItem * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbs);
        
        DECLSPEC_XFGVIRT(FolderItem, get_GetLink)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_GetLink )( 
            __RPC__in FolderItem * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppid);
        
        DECLSPEC_XFGVIRT(FolderItem, get_GetFolder)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_GetFolder )( 
            __RPC__in FolderItem * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppid);
        
        DECLSPEC_XFGVIRT(FolderItem, get_IsLink)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_IsLink )( 
            __RPC__in FolderItem * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pb);
        
        DECLSPEC_XFGVIRT(FolderItem, get_IsFolder)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_IsFolder )( 
            __RPC__in FolderItem * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pb);
        
        DECLSPEC_XFGVIRT(FolderItem, get_IsFileSystem)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_IsFileSystem )( 
            __RPC__in FolderItem * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pb);
        
        DECLSPEC_XFGVIRT(FolderItem, get_IsBrowsable)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_IsBrowsable )( 
            __RPC__in FolderItem * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pb);
        
        DECLSPEC_XFGVIRT(FolderItem, get_ModifyDate)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ModifyDate )( 
            __RPC__in FolderItem * This,
            /* [retval][out] */ __RPC__out DATE *pdt);
        
        DECLSPEC_XFGVIRT(FolderItem, put_ModifyDate)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ModifyDate )( 
            __RPC__in FolderItem * This,
            /* [in] */ DATE dt);
        
        DECLSPEC_XFGVIRT(FolderItem, get_Size)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Size )( 
            __RPC__in FolderItem * This,
            /* [retval][out] */ __RPC__out LONG *pul);
        
        DECLSPEC_XFGVIRT(FolderItem, get_Type)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Type )( 
            __RPC__in FolderItem * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbs);
        
        DECLSPEC_XFGVIRT(FolderItem, Verbs)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Verbs )( 
            __RPC__in FolderItem * This,
            /* [retval][out] */ __RPC__deref_out_opt FolderItemVerbs **ppfic);
        
        DECLSPEC_XFGVIRT(FolderItem, InvokeVerb)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *InvokeVerb )( 
            __RPC__in FolderItem * This,
            /* [optional][in] */ VARIANT vVerb);
        
        END_INTERFACE
    } FolderItemVtbl;

    interface FolderItem
    {
        CONST_VTBL struct FolderItemVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define FolderItem_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define FolderItem_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define FolderItem_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define FolderItem_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define FolderItem_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define FolderItem_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define FolderItem_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define FolderItem_get_Application(This,ppid)	\
    ( (This)->lpVtbl -> get_Application(This,ppid) ) 

#define FolderItem_get_Parent(This,ppid)	\
    ( (This)->lpVtbl -> get_Parent(This,ppid) ) 

#define FolderItem_get_Name(This,pbs)	\
    ( (This)->lpVtbl -> get_Name(This,pbs) ) 

#define FolderItem_put_Name(This,bs)	\
    ( (This)->lpVtbl -> put_Name(This,bs) ) 

#define FolderItem_get_Path(This,pbs)	\
    ( (This)->lpVtbl -> get_Path(This,pbs) ) 

#define FolderItem_get_GetLink(This,ppid)	\
    ( (This)->lpVtbl -> get_GetLink(This,ppid) ) 

#define FolderItem_get_GetFolder(This,ppid)	\
    ( (This)->lpVtbl -> get_GetFolder(This,ppid) ) 

#define FolderItem_get_IsLink(This,pb)	\
    ( (This)->lpVtbl -> get_IsLink(This,pb) ) 

#define FolderItem_get_IsFolder(This,pb)	\
    ( (This)->lpVtbl -> get_IsFolder(This,pb) ) 

#define FolderItem_get_IsFileSystem(This,pb)	\
    ( (This)->lpVtbl -> get_IsFileSystem(This,pb) ) 

#define FolderItem_get_IsBrowsable(This,pb)	\
    ( (This)->lpVtbl -> get_IsBrowsable(This,pb) ) 

#define FolderItem_get_ModifyDate(This,pdt)	\
    ( (This)->lpVtbl -> get_ModifyDate(This,pdt) ) 

#define FolderItem_put_ModifyDate(This,dt)	\
    ( (This)->lpVtbl -> put_ModifyDate(This,dt) ) 

#define FolderItem_get_Size(This,pul)	\
    ( (This)->lpVtbl -> get_Size(This,pul) ) 

#define FolderItem_get_Type(This,pbs)	\
    ( (This)->lpVtbl -> get_Type(This,pbs) ) 

#define FolderItem_Verbs(This,ppfic)	\
    ( (This)->lpVtbl -> Verbs(This,ppfic) ) 

#define FolderItem_InvokeVerb(This,vVerb)	\
    ( (This)->lpVtbl -> InvokeVerb(This,vVerb) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __FolderItem_INTERFACE_DEFINED__ */


#ifndef __FolderItems_INTERFACE_DEFINED__
#define __FolderItems_INTERFACE_DEFINED__

/* interface FolderItems */
/* [object][dual][oleautomation][helpstring][uuid] */ 


EXTERN_C const IID IID_FolderItems;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("744129E0-CBE5-11CE-8350-444553540000")
    FolderItems : public IDispatch
    {
    public:
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *plCount) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Application( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppid) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Parent( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppid) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Item( 
            /* [optional][in] */ VARIANT index,
            /* [retval][out] */ __RPC__deref_out_opt FolderItem **ppid) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE _NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppunk) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct FolderItemsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in FolderItems * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in FolderItems * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in FolderItems * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in FolderItems * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in FolderItems * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in FolderItems * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            FolderItems * This,
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
        
        DECLSPEC_XFGVIRT(FolderItems, get_Count)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in FolderItems * This,
            /* [retval][out] */ __RPC__out long *plCount);
        
        DECLSPEC_XFGVIRT(FolderItems, get_Application)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Application )( 
            __RPC__in FolderItems * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppid);
        
        DECLSPEC_XFGVIRT(FolderItems, get_Parent)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Parent )( 
            __RPC__in FolderItems * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppid);
        
        DECLSPEC_XFGVIRT(FolderItems, Item)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Item )( 
            __RPC__in FolderItems * This,
            /* [optional][in] */ VARIANT index,
            /* [retval][out] */ __RPC__deref_out_opt FolderItem **ppid);
        
        DECLSPEC_XFGVIRT(FolderItems, _NewEnum)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *_NewEnum )( 
            __RPC__in FolderItems * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppunk);
        
        END_INTERFACE
    } FolderItemsVtbl;

    interface FolderItems
    {
        CONST_VTBL struct FolderItemsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define FolderItems_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define FolderItems_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define FolderItems_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define FolderItems_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define FolderItems_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define FolderItems_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define FolderItems_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define FolderItems_get_Count(This,plCount)	\
    ( (This)->lpVtbl -> get_Count(This,plCount) ) 

#define FolderItems_get_Application(This,ppid)	\
    ( (This)->lpVtbl -> get_Application(This,ppid) ) 

#define FolderItems_get_Parent(This,ppid)	\
    ( (This)->lpVtbl -> get_Parent(This,ppid) ) 

#define FolderItems_Item(This,index,ppid)	\
    ( (This)->lpVtbl -> Item(This,index,ppid) ) 

#define FolderItems__NewEnum(This,ppunk)	\
    ( (This)->lpVtbl -> _NewEnum(This,ppunk) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __FolderItems_INTERFACE_DEFINED__ */


#ifndef __FolderItemVerb_INTERFACE_DEFINED__
#define __FolderItemVerb_INTERFACE_DEFINED__

/* interface FolderItemVerb */
/* [object][dual][oleautomation][helpstring][uuid] */ 


EXTERN_C const IID IID_FolderItemVerb;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("08EC3E00-50B0-11CF-960C-0080C7F4EE85")
    FolderItemVerb : public IDispatch
    {
    public:
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Application( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppid) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Parent( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppid) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbs) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE DoIt( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct FolderItemVerbVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in FolderItemVerb * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in FolderItemVerb * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in FolderItemVerb * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in FolderItemVerb * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in FolderItemVerb * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in FolderItemVerb * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            FolderItemVerb * This,
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
        
        DECLSPEC_XFGVIRT(FolderItemVerb, get_Application)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Application )( 
            __RPC__in FolderItemVerb * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppid);
        
        DECLSPEC_XFGVIRT(FolderItemVerb, get_Parent)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Parent )( 
            __RPC__in FolderItemVerb * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppid);
        
        DECLSPEC_XFGVIRT(FolderItemVerb, get_Name)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in FolderItemVerb * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbs);
        
        DECLSPEC_XFGVIRT(FolderItemVerb, DoIt)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *DoIt )( 
            __RPC__in FolderItemVerb * This);
        
        END_INTERFACE
    } FolderItemVerbVtbl;

    interface FolderItemVerb
    {
        CONST_VTBL struct FolderItemVerbVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define FolderItemVerb_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define FolderItemVerb_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define FolderItemVerb_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define FolderItemVerb_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define FolderItemVerb_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define FolderItemVerb_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define FolderItemVerb_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define FolderItemVerb_get_Application(This,ppid)	\
    ( (This)->lpVtbl -> get_Application(This,ppid) ) 

#define FolderItemVerb_get_Parent(This,ppid)	\
    ( (This)->lpVtbl -> get_Parent(This,ppid) ) 

#define FolderItemVerb_get_Name(This,pbs)	\
    ( (This)->lpVtbl -> get_Name(This,pbs) ) 

#define FolderItemVerb_DoIt(This)	\
    ( (This)->lpVtbl -> DoIt(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __FolderItemVerb_INTERFACE_DEFINED__ */


#ifndef __FolderItemVerbs_INTERFACE_DEFINED__
#define __FolderItemVerbs_INTERFACE_DEFINED__

/* interface FolderItemVerbs */
/* [object][dual][oleautomation][helpstring][uuid] */ 


EXTERN_C const IID IID_FolderItemVerbs;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1F8352C0-50B0-11CF-960C-0080C7F4EE85")
    FolderItemVerbs : public IDispatch
    {
    public:
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *plCount) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Application( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppid) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Parent( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppid) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Item( 
            /* [optional][in] */ VARIANT index,
            /* [retval][out] */ __RPC__deref_out_opt FolderItemVerb **ppid) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE _NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppunk) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct FolderItemVerbsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in FolderItemVerbs * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in FolderItemVerbs * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in FolderItemVerbs * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in FolderItemVerbs * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in FolderItemVerbs * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in FolderItemVerbs * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            FolderItemVerbs * This,
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
        
        DECLSPEC_XFGVIRT(FolderItemVerbs, get_Count)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in FolderItemVerbs * This,
            /* [retval][out] */ __RPC__out long *plCount);
        
        DECLSPEC_XFGVIRT(FolderItemVerbs, get_Application)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Application )( 
            __RPC__in FolderItemVerbs * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppid);
        
        DECLSPEC_XFGVIRT(FolderItemVerbs, get_Parent)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Parent )( 
            __RPC__in FolderItemVerbs * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppid);
        
        DECLSPEC_XFGVIRT(FolderItemVerbs, Item)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Item )( 
            __RPC__in FolderItemVerbs * This,
            /* [optional][in] */ VARIANT index,
            /* [retval][out] */ __RPC__deref_out_opt FolderItemVerb **ppid);
        
        DECLSPEC_XFGVIRT(FolderItemVerbs, _NewEnum)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *_NewEnum )( 
            __RPC__in FolderItemVerbs * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppunk);
        
        END_INTERFACE
    } FolderItemVerbsVtbl;

    interface FolderItemVerbs
    {
        CONST_VTBL struct FolderItemVerbsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define FolderItemVerbs_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define FolderItemVerbs_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define FolderItemVerbs_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define FolderItemVerbs_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define FolderItemVerbs_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define FolderItemVerbs_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define FolderItemVerbs_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define FolderItemVerbs_get_Count(This,plCount)	\
    ( (This)->lpVtbl -> get_Count(This,plCount) ) 

#define FolderItemVerbs_get_Application(This,ppid)	\
    ( (This)->lpVtbl -> get_Application(This,ppid) ) 

#define FolderItemVerbs_get_Parent(This,ppid)	\
    ( (This)->lpVtbl -> get_Parent(This,ppid) ) 

#define FolderItemVerbs_Item(This,index,ppid)	\
    ( (This)->lpVtbl -> Item(This,index,ppid) ) 

#define FolderItemVerbs__NewEnum(This,ppunk)	\
    ( (This)->lpVtbl -> _NewEnum(This,ppunk) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __FolderItemVerbs_INTERFACE_DEFINED__ */


#ifndef __Folder_INTERFACE_DEFINED__
#define __Folder_INTERFACE_DEFINED__

/* interface Folder */
/* [object][dual][oleautomation][helpstring][uuid] */ 


EXTERN_C const IID IID_Folder;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("BBCBDE60-C3FF-11CE-8350-444553540000")
    Folder : public IDispatch
    {
    public:
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Title( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbs) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Application( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppid) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Parent( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppid) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_ParentFolder( 
            /* [retval][out] */ __RPC__deref_out_opt Folder **ppsf) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Items( 
            /* [retval][out] */ __RPC__deref_out_opt FolderItems **ppid) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE ParseName( 
            /* [in] */ __RPC__in BSTR bName,
            /* [retval][out] */ __RPC__deref_out_opt FolderItem **ppid) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE NewFolder( 
            /* [in] */ __RPC__in BSTR bName,
            /* [optional][in] */ VARIANT vOptions) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE MoveHere( 
            /* [in] */ VARIANT vItem,
            /* [optional][in] */ VARIANT vOptions) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE CopyHere( 
            /* [in] */ VARIANT vItem,
            /* [optional][in] */ VARIANT vOptions) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetDetailsOf( 
            /* [in] */ VARIANT vItem,
            /* [in] */ int iColumn,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbs) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct FolderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in Folder * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in Folder * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in Folder * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in Folder * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in Folder * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in Folder * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            Folder * This,
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
        
        DECLSPEC_XFGVIRT(Folder, get_Title)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Title )( 
            __RPC__in Folder * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbs);
        
        DECLSPEC_XFGVIRT(Folder, get_Application)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Application )( 
            __RPC__in Folder * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppid);
        
        DECLSPEC_XFGVIRT(Folder, get_Parent)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Parent )( 
            __RPC__in Folder * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppid);
        
        DECLSPEC_XFGVIRT(Folder, get_ParentFolder)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ParentFolder )( 
            __RPC__in Folder * This,
            /* [retval][out] */ __RPC__deref_out_opt Folder **ppsf);
        
        DECLSPEC_XFGVIRT(Folder, Items)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Items )( 
            __RPC__in Folder * This,
            /* [retval][out] */ __RPC__deref_out_opt FolderItems **ppid);
        
        DECLSPEC_XFGVIRT(Folder, ParseName)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ParseName )( 
            __RPC__in Folder * This,
            /* [in] */ __RPC__in BSTR bName,
            /* [retval][out] */ __RPC__deref_out_opt FolderItem **ppid);
        
        DECLSPEC_XFGVIRT(Folder, NewFolder)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *NewFolder )( 
            __RPC__in Folder * This,
            /* [in] */ __RPC__in BSTR bName,
            /* [optional][in] */ VARIANT vOptions);
        
        DECLSPEC_XFGVIRT(Folder, MoveHere)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *MoveHere )( 
            __RPC__in Folder * This,
            /* [in] */ VARIANT vItem,
            /* [optional][in] */ VARIANT vOptions);
        
        DECLSPEC_XFGVIRT(Folder, CopyHere)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *CopyHere )( 
            __RPC__in Folder * This,
            /* [in] */ VARIANT vItem,
            /* [optional][in] */ VARIANT vOptions);
        
        DECLSPEC_XFGVIRT(Folder, GetDetailsOf)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetDetailsOf )( 
            __RPC__in Folder * This,
            /* [in] */ VARIANT vItem,
            /* [in] */ int iColumn,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbs);
        
        END_INTERFACE
    } FolderVtbl;

    interface Folder
    {
        CONST_VTBL struct FolderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define Folder_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define Folder_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define Folder_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define Folder_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define Folder_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define Folder_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define Folder_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define Folder_get_Title(This,pbs)	\
    ( (This)->lpVtbl -> get_Title(This,pbs) ) 

#define Folder_get_Application(This,ppid)	\
    ( (This)->lpVtbl -> get_Application(This,ppid) ) 

#define Folder_get_Parent(This,ppid)	\
    ( (This)->lpVtbl -> get_Parent(This,ppid) ) 

#define Folder_get_ParentFolder(This,ppsf)	\
    ( (This)->lpVtbl -> get_ParentFolder(This,ppsf) ) 

#define Folder_Items(This,ppid)	\
    ( (This)->lpVtbl -> Items(This,ppid) ) 

#define Folder_ParseName(This,bName,ppid)	\
    ( (This)->lpVtbl -> ParseName(This,bName,ppid) ) 

#define Folder_NewFolder(This,bName,vOptions)	\
    ( (This)->lpVtbl -> NewFolder(This,bName,vOptions) ) 

#define Folder_MoveHere(This,vItem,vOptions)	\
    ( (This)->lpVtbl -> MoveHere(This,vItem,vOptions) ) 

#define Folder_CopyHere(This,vItem,vOptions)	\
    ( (This)->lpVtbl -> CopyHere(This,vItem,vOptions) ) 

#define Folder_GetDetailsOf(This,vItem,iColumn,pbs)	\
    ( (This)->lpVtbl -> GetDetailsOf(This,vItem,iColumn,pbs) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __Folder_INTERFACE_DEFINED__ */


#ifndef __Folder2_INTERFACE_DEFINED__
#define __Folder2_INTERFACE_DEFINED__

/* interface Folder2 */
/* [object][dual][oleautomation][helpstring][uuid] */ 


EXTERN_C const IID IID_Folder2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f0d2d8ef-3890-11d2-bf8b-00c04fb93661")
    Folder2 : public Folder
    {
    public:
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Self( 
            /* [retval][out] */ __RPC__deref_out_opt FolderItem **ppfi) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_OfflineStatus( 
            /* [retval][out] */ __RPC__out LONG *pul) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Synchronize( void) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_HaveToShowWebViewBarricade( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbHaveToShowWebViewBarricade) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE DismissedWebViewBarricade( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct Folder2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in Folder2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in Folder2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in Folder2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in Folder2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in Folder2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in Folder2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            Folder2 * This,
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
        
        DECLSPEC_XFGVIRT(Folder, get_Title)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Title )( 
            __RPC__in Folder2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbs);
        
        DECLSPEC_XFGVIRT(Folder, get_Application)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Application )( 
            __RPC__in Folder2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppid);
        
        DECLSPEC_XFGVIRT(Folder, get_Parent)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Parent )( 
            __RPC__in Folder2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppid);
        
        DECLSPEC_XFGVIRT(Folder, get_ParentFolder)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ParentFolder )( 
            __RPC__in Folder2 * This,
            /* [retval][out] */ __RPC__deref_out_opt Folder **ppsf);
        
        DECLSPEC_XFGVIRT(Folder, Items)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Items )( 
            __RPC__in Folder2 * This,
            /* [retval][out] */ __RPC__deref_out_opt FolderItems **ppid);
        
        DECLSPEC_XFGVIRT(Folder, ParseName)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ParseName )( 
            __RPC__in Folder2 * This,
            /* [in] */ __RPC__in BSTR bName,
            /* [retval][out] */ __RPC__deref_out_opt FolderItem **ppid);
        
        DECLSPEC_XFGVIRT(Folder, NewFolder)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *NewFolder )( 
            __RPC__in Folder2 * This,
            /* [in] */ __RPC__in BSTR bName,
            /* [optional][in] */ VARIANT vOptions);
        
        DECLSPEC_XFGVIRT(Folder, MoveHere)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *MoveHere )( 
            __RPC__in Folder2 * This,
            /* [in] */ VARIANT vItem,
            /* [optional][in] */ VARIANT vOptions);
        
        DECLSPEC_XFGVIRT(Folder, CopyHere)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *CopyHere )( 
            __RPC__in Folder2 * This,
            /* [in] */ VARIANT vItem,
            /* [optional][in] */ VARIANT vOptions);
        
        DECLSPEC_XFGVIRT(Folder, GetDetailsOf)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetDetailsOf )( 
            __RPC__in Folder2 * This,
            /* [in] */ VARIANT vItem,
            /* [in] */ int iColumn,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbs);
        
        DECLSPEC_XFGVIRT(Folder2, get_Self)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Self )( 
            __RPC__in Folder2 * This,
            /* [retval][out] */ __RPC__deref_out_opt FolderItem **ppfi);
        
        DECLSPEC_XFGVIRT(Folder2, get_OfflineStatus)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OfflineStatus )( 
            __RPC__in Folder2 * This,
            /* [retval][out] */ __RPC__out LONG *pul);
        
        DECLSPEC_XFGVIRT(Folder2, Synchronize)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Synchronize )( 
            __RPC__in Folder2 * This);
        
        DECLSPEC_XFGVIRT(Folder2, get_HaveToShowWebViewBarricade)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_HaveToShowWebViewBarricade )( 
            __RPC__in Folder2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbHaveToShowWebViewBarricade);
        
        DECLSPEC_XFGVIRT(Folder2, DismissedWebViewBarricade)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *DismissedWebViewBarricade )( 
            __RPC__in Folder2 * This);
        
        END_INTERFACE
    } Folder2Vtbl;

    interface Folder2
    {
        CONST_VTBL struct Folder2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define Folder2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define Folder2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define Folder2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define Folder2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define Folder2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define Folder2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define Folder2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define Folder2_get_Title(This,pbs)	\
    ( (This)->lpVtbl -> get_Title(This,pbs) ) 

#define Folder2_get_Application(This,ppid)	\
    ( (This)->lpVtbl -> get_Application(This,ppid) ) 

#define Folder2_get_Parent(This,ppid)	\
    ( (This)->lpVtbl -> get_Parent(This,ppid) ) 

#define Folder2_get_ParentFolder(This,ppsf)	\
    ( (This)->lpVtbl -> get_ParentFolder(This,ppsf) ) 

#define Folder2_Items(This,ppid)	\
    ( (This)->lpVtbl -> Items(This,ppid) ) 

#define Folder2_ParseName(This,bName,ppid)	\
    ( (This)->lpVtbl -> ParseName(This,bName,ppid) ) 

#define Folder2_NewFolder(This,bName,vOptions)	\
    ( (This)->lpVtbl -> NewFolder(This,bName,vOptions) ) 

#define Folder2_MoveHere(This,vItem,vOptions)	\
    ( (This)->lpVtbl -> MoveHere(This,vItem,vOptions) ) 

#define Folder2_CopyHere(This,vItem,vOptions)	\
    ( (This)->lpVtbl -> CopyHere(This,vItem,vOptions) ) 

#define Folder2_GetDetailsOf(This,vItem,iColumn,pbs)	\
    ( (This)->lpVtbl -> GetDetailsOf(This,vItem,iColumn,pbs) ) 


#define Folder2_get_Self(This,ppfi)	\
    ( (This)->lpVtbl -> get_Self(This,ppfi) ) 

#define Folder2_get_OfflineStatus(This,pul)	\
    ( (This)->lpVtbl -> get_OfflineStatus(This,pul) ) 

#define Folder2_Synchronize(This)	\
    ( (This)->lpVtbl -> Synchronize(This) ) 

#define Folder2_get_HaveToShowWebViewBarricade(This,pbHaveToShowWebViewBarricade)	\
    ( (This)->lpVtbl -> get_HaveToShowWebViewBarricade(This,pbHaveToShowWebViewBarricade) ) 

#define Folder2_DismissedWebViewBarricade(This)	\
    ( (This)->lpVtbl -> DismissedWebViewBarricade(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __Folder2_INTERFACE_DEFINED__ */


#ifndef __Folder3_INTERFACE_DEFINED__
#define __Folder3_INTERFACE_DEFINED__

/* interface Folder3 */
/* [object][dual][oleautomation][helpstring][uuid] */ 


EXTERN_C const IID IID_Folder3;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A7AE5F64-C4D7-4d7f-9307-4D24EE54B841")
    Folder3 : public Folder2
    {
    public:
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_ShowWebViewBarricade( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbShowWebViewBarricade) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_ShowWebViewBarricade( 
            /* [in] */ VARIANT_BOOL bShowWebViewBarricade) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct Folder3Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in Folder3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in Folder3 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in Folder3 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in Folder3 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in Folder3 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in Folder3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            Folder3 * This,
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
        
        DECLSPEC_XFGVIRT(Folder, get_Title)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Title )( 
            __RPC__in Folder3 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbs);
        
        DECLSPEC_XFGVIRT(Folder, get_Application)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Application )( 
            __RPC__in Folder3 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppid);
        
        DECLSPEC_XFGVIRT(Folder, get_Parent)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Parent )( 
            __RPC__in Folder3 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppid);
        
        DECLSPEC_XFGVIRT(Folder, get_ParentFolder)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ParentFolder )( 
            __RPC__in Folder3 * This,
            /* [retval][out] */ __RPC__deref_out_opt Folder **ppsf);
        
        DECLSPEC_XFGVIRT(Folder, Items)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Items )( 
            __RPC__in Folder3 * This,
            /* [retval][out] */ __RPC__deref_out_opt FolderItems **ppid);
        
        DECLSPEC_XFGVIRT(Folder, ParseName)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ParseName )( 
            __RPC__in Folder3 * This,
            /* [in] */ __RPC__in BSTR bName,
            /* [retval][out] */ __RPC__deref_out_opt FolderItem **ppid);
        
        DECLSPEC_XFGVIRT(Folder, NewFolder)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *NewFolder )( 
            __RPC__in Folder3 * This,
            /* [in] */ __RPC__in BSTR bName,
            /* [optional][in] */ VARIANT vOptions);
        
        DECLSPEC_XFGVIRT(Folder, MoveHere)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *MoveHere )( 
            __RPC__in Folder3 * This,
            /* [in] */ VARIANT vItem,
            /* [optional][in] */ VARIANT vOptions);
        
        DECLSPEC_XFGVIRT(Folder, CopyHere)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *CopyHere )( 
            __RPC__in Folder3 * This,
            /* [in] */ VARIANT vItem,
            /* [optional][in] */ VARIANT vOptions);
        
        DECLSPEC_XFGVIRT(Folder, GetDetailsOf)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetDetailsOf )( 
            __RPC__in Folder3 * This,
            /* [in] */ VARIANT vItem,
            /* [in] */ int iColumn,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbs);
        
        DECLSPEC_XFGVIRT(Folder2, get_Self)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Self )( 
            __RPC__in Folder3 * This,
            /* [retval][out] */ __RPC__deref_out_opt FolderItem **ppfi);
        
        DECLSPEC_XFGVIRT(Folder2, get_OfflineStatus)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OfflineStatus )( 
            __RPC__in Folder3 * This,
            /* [retval][out] */ __RPC__out LONG *pul);
        
        DECLSPEC_XFGVIRT(Folder2, Synchronize)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Synchronize )( 
            __RPC__in Folder3 * This);
        
        DECLSPEC_XFGVIRT(Folder2, get_HaveToShowWebViewBarricade)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_HaveToShowWebViewBarricade )( 
            __RPC__in Folder3 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbHaveToShowWebViewBarricade);
        
        DECLSPEC_XFGVIRT(Folder2, DismissedWebViewBarricade)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *DismissedWebViewBarricade )( 
            __RPC__in Folder3 * This);
        
        DECLSPEC_XFGVIRT(Folder3, get_ShowWebViewBarricade)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ShowWebViewBarricade )( 
            __RPC__in Folder3 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbShowWebViewBarricade);
        
        DECLSPEC_XFGVIRT(Folder3, put_ShowWebViewBarricade)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ShowWebViewBarricade )( 
            __RPC__in Folder3 * This,
            /* [in] */ VARIANT_BOOL bShowWebViewBarricade);
        
        END_INTERFACE
    } Folder3Vtbl;

    interface Folder3
    {
        CONST_VTBL struct Folder3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define Folder3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define Folder3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define Folder3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define Folder3_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define Folder3_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define Folder3_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define Folder3_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define Folder3_get_Title(This,pbs)	\
    ( (This)->lpVtbl -> get_Title(This,pbs) ) 

#define Folder3_get_Application(This,ppid)	\
    ( (This)->lpVtbl -> get_Application(This,ppid) ) 

#define Folder3_get_Parent(This,ppid)	\
    ( (This)->lpVtbl -> get_Parent(This,ppid) ) 

#define Folder3_get_ParentFolder(This,ppsf)	\
    ( (This)->lpVtbl -> get_ParentFolder(This,ppsf) ) 

#define Folder3_Items(This,ppid)	\
    ( (This)->lpVtbl -> Items(This,ppid) ) 

#define Folder3_ParseName(This,bName,ppid)	\
    ( (This)->lpVtbl -> ParseName(This,bName,ppid) ) 

#define Folder3_NewFolder(This,bName,vOptions)	\
    ( (This)->lpVtbl -> NewFolder(This,bName,vOptions) ) 

#define Folder3_MoveHere(This,vItem,vOptions)	\
    ( (This)->lpVtbl -> MoveHere(This,vItem,vOptions) ) 

#define Folder3_CopyHere(This,vItem,vOptions)	\
    ( (This)->lpVtbl -> CopyHere(This,vItem,vOptions) ) 

#define Folder3_GetDetailsOf(This,vItem,iColumn,pbs)	\
    ( (This)->lpVtbl -> GetDetailsOf(This,vItem,iColumn,pbs) ) 


#define Folder3_get_Self(This,ppfi)	\
    ( (This)->lpVtbl -> get_Self(This,ppfi) ) 

#define Folder3_get_OfflineStatus(This,pul)	\
    ( (This)->lpVtbl -> get_OfflineStatus(This,pul) ) 

#define Folder3_Synchronize(This)	\
    ( (This)->lpVtbl -> Synchronize(This) ) 

#define Folder3_get_HaveToShowWebViewBarricade(This,pbHaveToShowWebViewBarricade)	\
    ( (This)->lpVtbl -> get_HaveToShowWebViewBarricade(This,pbHaveToShowWebViewBarricade) ) 

#define Folder3_DismissedWebViewBarricade(This)	\
    ( (This)->lpVtbl -> DismissedWebViewBarricade(This) ) 


#define Folder3_get_ShowWebViewBarricade(This,pbShowWebViewBarricade)	\
    ( (This)->lpVtbl -> get_ShowWebViewBarricade(This,pbShowWebViewBarricade) ) 

#define Folder3_put_ShowWebViewBarricade(This,bShowWebViewBarricade)	\
    ( (This)->lpVtbl -> put_ShowWebViewBarricade(This,bShowWebViewBarricade) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __Folder3_INTERFACE_DEFINED__ */


#ifndef __FolderItem2_INTERFACE_DEFINED__
#define __FolderItem2_INTERFACE_DEFINED__

/* interface FolderItem2 */
/* [object][dual][oleautomation][helpstring][uuid] */ 


EXTERN_C const IID IID_FolderItem2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("edc817aa-92b8-11d1-b075-00c04fc33aa5")
    FolderItem2 : public FolderItem
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE InvokeVerbEx( 
            /* [optional][in] */ VARIANT vVerb,
            /* [optional][in] */ VARIANT vArgs) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE ExtendedProperty( 
            /* [in] */ __RPC__in BSTR bstrPropName,
            /* [retval][out] */ __RPC__out VARIANT *pvRet) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct FolderItem2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in FolderItem2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in FolderItem2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in FolderItem2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in FolderItem2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in FolderItem2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in FolderItem2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            FolderItem2 * This,
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
        
        DECLSPEC_XFGVIRT(FolderItem, get_Application)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Application )( 
            __RPC__in FolderItem2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppid);
        
        DECLSPEC_XFGVIRT(FolderItem, get_Parent)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Parent )( 
            __RPC__in FolderItem2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppid);
        
        DECLSPEC_XFGVIRT(FolderItem, get_Name)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in FolderItem2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbs);
        
        DECLSPEC_XFGVIRT(FolderItem, put_Name)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Name )( 
            __RPC__in FolderItem2 * This,
            /* [in] */ __RPC__in BSTR bs);
        
        DECLSPEC_XFGVIRT(FolderItem, get_Path)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Path )( 
            __RPC__in FolderItem2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbs);
        
        DECLSPEC_XFGVIRT(FolderItem, get_GetLink)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_GetLink )( 
            __RPC__in FolderItem2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppid);
        
        DECLSPEC_XFGVIRT(FolderItem, get_GetFolder)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_GetFolder )( 
            __RPC__in FolderItem2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppid);
        
        DECLSPEC_XFGVIRT(FolderItem, get_IsLink)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_IsLink )( 
            __RPC__in FolderItem2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pb);
        
        DECLSPEC_XFGVIRT(FolderItem, get_IsFolder)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_IsFolder )( 
            __RPC__in FolderItem2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pb);
        
        DECLSPEC_XFGVIRT(FolderItem, get_IsFileSystem)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_IsFileSystem )( 
            __RPC__in FolderItem2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pb);
        
        DECLSPEC_XFGVIRT(FolderItem, get_IsBrowsable)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_IsBrowsable )( 
            __RPC__in FolderItem2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pb);
        
        DECLSPEC_XFGVIRT(FolderItem, get_ModifyDate)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ModifyDate )( 
            __RPC__in FolderItem2 * This,
            /* [retval][out] */ __RPC__out DATE *pdt);
        
        DECLSPEC_XFGVIRT(FolderItem, put_ModifyDate)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ModifyDate )( 
            __RPC__in FolderItem2 * This,
            /* [in] */ DATE dt);
        
        DECLSPEC_XFGVIRT(FolderItem, get_Size)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Size )( 
            __RPC__in FolderItem2 * This,
            /* [retval][out] */ __RPC__out LONG *pul);
        
        DECLSPEC_XFGVIRT(FolderItem, get_Type)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Type )( 
            __RPC__in FolderItem2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbs);
        
        DECLSPEC_XFGVIRT(FolderItem, Verbs)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Verbs )( 
            __RPC__in FolderItem2 * This,
            /* [retval][out] */ __RPC__deref_out_opt FolderItemVerbs **ppfic);
        
        DECLSPEC_XFGVIRT(FolderItem, InvokeVerb)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *InvokeVerb )( 
            __RPC__in FolderItem2 * This,
            /* [optional][in] */ VARIANT vVerb);
        
        DECLSPEC_XFGVIRT(FolderItem2, InvokeVerbEx)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *InvokeVerbEx )( 
            __RPC__in FolderItem2 * This,
            /* [optional][in] */ VARIANT vVerb,
            /* [optional][in] */ VARIANT vArgs);
        
        DECLSPEC_XFGVIRT(FolderItem2, ExtendedProperty)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ExtendedProperty )( 
            __RPC__in FolderItem2 * This,
            /* [in] */ __RPC__in BSTR bstrPropName,
            /* [retval][out] */ __RPC__out VARIANT *pvRet);
        
        END_INTERFACE
    } FolderItem2Vtbl;

    interface FolderItem2
    {
        CONST_VTBL struct FolderItem2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define FolderItem2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define FolderItem2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define FolderItem2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define FolderItem2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define FolderItem2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define FolderItem2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define FolderItem2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define FolderItem2_get_Application(This,ppid)	\
    ( (This)->lpVtbl -> get_Application(This,ppid) ) 

#define FolderItem2_get_Parent(This,ppid)	\
    ( (This)->lpVtbl -> get_Parent(This,ppid) ) 

#define FolderItem2_get_Name(This,pbs)	\
    ( (This)->lpVtbl -> get_Name(This,pbs) ) 

#define FolderItem2_put_Name(This,bs)	\
    ( (This)->lpVtbl -> put_Name(This,bs) ) 

#define FolderItem2_get_Path(This,pbs)	\
    ( (This)->lpVtbl -> get_Path(This,pbs) ) 

#define FolderItem2_get_GetLink(This,ppid)	\
    ( (This)->lpVtbl -> get_GetLink(This,ppid) ) 

#define FolderItem2_get_GetFolder(This,ppid)	\
    ( (This)->lpVtbl -> get_GetFolder(This,ppid) ) 

#define FolderItem2_get_IsLink(This,pb)	\
    ( (This)->lpVtbl -> get_IsLink(This,pb) ) 

#define FolderItem2_get_IsFolder(This,pb)	\
    ( (This)->lpVtbl -> get_IsFolder(This,pb) ) 

#define FolderItem2_get_IsFileSystem(This,pb)	\
    ( (This)->lpVtbl -> get_IsFileSystem(This,pb) ) 

#define FolderItem2_get_IsBrowsable(This,pb)	\
    ( (This)->lpVtbl -> get_IsBrowsable(This,pb) ) 

#define FolderItem2_get_ModifyDate(This,pdt)	\
    ( (This)->lpVtbl -> get_ModifyDate(This,pdt) ) 

#define FolderItem2_put_ModifyDate(This,dt)	\
    ( (This)->lpVtbl -> put_ModifyDate(This,dt) ) 

#define FolderItem2_get_Size(This,pul)	\
    ( (This)->lpVtbl -> get_Size(This,pul) ) 

#define FolderItem2_get_Type(This,pbs)	\
    ( (This)->lpVtbl -> get_Type(This,pbs) ) 

#define FolderItem2_Verbs(This,ppfic)	\
    ( (This)->lpVtbl -> Verbs(This,ppfic) ) 

#define FolderItem2_InvokeVerb(This,vVerb)	\
    ( (This)->lpVtbl -> InvokeVerb(This,vVerb) ) 


#define FolderItem2_InvokeVerbEx(This,vVerb,vArgs)	\
    ( (This)->lpVtbl -> InvokeVerbEx(This,vVerb,vArgs) ) 

#define FolderItem2_ExtendedProperty(This,bstrPropName,pvRet)	\
    ( (This)->lpVtbl -> ExtendedProperty(This,bstrPropName,pvRet) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __FolderItem2_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_ShellFolderItem;

#ifdef __cplusplus

class DECLSPEC_UUID("2fe352ea-fd1f-11d2-b1f4-00c04f8eeb3e")
ShellFolderItem;
#endif

#ifndef __FolderItems2_INTERFACE_DEFINED__
#define __FolderItems2_INTERFACE_DEFINED__

/* interface FolderItems2 */
/* [object][dual][oleautomation][helpstring][uuid] */ 


EXTERN_C const IID IID_FolderItems2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C94F0AD0-F363-11d2-A327-00C04F8EEC7F")
    FolderItems2 : public FolderItems
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE InvokeVerbEx( 
            /* [optional][in] */ VARIANT vVerb,
            /* [optional][in] */ VARIANT vArgs) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct FolderItems2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in FolderItems2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in FolderItems2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in FolderItems2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in FolderItems2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in FolderItems2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in FolderItems2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            FolderItems2 * This,
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
        
        DECLSPEC_XFGVIRT(FolderItems, get_Count)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in FolderItems2 * This,
            /* [retval][out] */ __RPC__out long *plCount);
        
        DECLSPEC_XFGVIRT(FolderItems, get_Application)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Application )( 
            __RPC__in FolderItems2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppid);
        
        DECLSPEC_XFGVIRT(FolderItems, get_Parent)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Parent )( 
            __RPC__in FolderItems2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppid);
        
        DECLSPEC_XFGVIRT(FolderItems, Item)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Item )( 
            __RPC__in FolderItems2 * This,
            /* [optional][in] */ VARIANT index,
            /* [retval][out] */ __RPC__deref_out_opt FolderItem **ppid);
        
        DECLSPEC_XFGVIRT(FolderItems, _NewEnum)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *_NewEnum )( 
            __RPC__in FolderItems2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppunk);
        
        DECLSPEC_XFGVIRT(FolderItems2, InvokeVerbEx)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *InvokeVerbEx )( 
            __RPC__in FolderItems2 * This,
            /* [optional][in] */ VARIANT vVerb,
            /* [optional][in] */ VARIANT vArgs);
        
        END_INTERFACE
    } FolderItems2Vtbl;

    interface FolderItems2
    {
        CONST_VTBL struct FolderItems2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define FolderItems2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define FolderItems2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define FolderItems2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define FolderItems2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define FolderItems2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define FolderItems2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define FolderItems2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define FolderItems2_get_Count(This,plCount)	\
    ( (This)->lpVtbl -> get_Count(This,plCount) ) 

#define FolderItems2_get_Application(This,ppid)	\
    ( (This)->lpVtbl -> get_Application(This,ppid) ) 

#define FolderItems2_get_Parent(This,ppid)	\
    ( (This)->lpVtbl -> get_Parent(This,ppid) ) 

#define FolderItems2_Item(This,index,ppid)	\
    ( (This)->lpVtbl -> Item(This,index,ppid) ) 

#define FolderItems2__NewEnum(This,ppunk)	\
    ( (This)->lpVtbl -> _NewEnum(This,ppunk) ) 


#define FolderItems2_InvokeVerbEx(This,vVerb,vArgs)	\
    ( (This)->lpVtbl -> InvokeVerbEx(This,vVerb,vArgs) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __FolderItems2_INTERFACE_DEFINED__ */


#ifndef __FolderItems3_INTERFACE_DEFINED__
#define __FolderItems3_INTERFACE_DEFINED__

/* interface FolderItems3 */
/* [object][dual][oleautomation][helpstring][uuid] */ 


EXTERN_C const IID IID_FolderItems3;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("eaa7c309-bbec-49d5-821d-64d966cb667f")
    FolderItems3 : public FolderItems2
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Filter( 
            /* [in] */ long grfFlags,
            /* [in] */ __RPC__in BSTR bstrFileSpec) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Verbs( 
            /* [retval][out] */ __RPC__deref_out_opt FolderItemVerbs **ppfic) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct FolderItems3Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in FolderItems3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in FolderItems3 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in FolderItems3 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in FolderItems3 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in FolderItems3 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in FolderItems3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            FolderItems3 * This,
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
        
        DECLSPEC_XFGVIRT(FolderItems, get_Count)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in FolderItems3 * This,
            /* [retval][out] */ __RPC__out long *plCount);
        
        DECLSPEC_XFGVIRT(FolderItems, get_Application)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Application )( 
            __RPC__in FolderItems3 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppid);
        
        DECLSPEC_XFGVIRT(FolderItems, get_Parent)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Parent )( 
            __RPC__in FolderItems3 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppid);
        
        DECLSPEC_XFGVIRT(FolderItems, Item)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Item )( 
            __RPC__in FolderItems3 * This,
            /* [optional][in] */ VARIANT index,
            /* [retval][out] */ __RPC__deref_out_opt FolderItem **ppid);
        
        DECLSPEC_XFGVIRT(FolderItems, _NewEnum)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *_NewEnum )( 
            __RPC__in FolderItems3 * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppunk);
        
        DECLSPEC_XFGVIRT(FolderItems2, InvokeVerbEx)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *InvokeVerbEx )( 
            __RPC__in FolderItems3 * This,
            /* [optional][in] */ VARIANT vVerb,
            /* [optional][in] */ VARIANT vArgs);
        
        DECLSPEC_XFGVIRT(FolderItems3, Filter)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Filter )( 
            __RPC__in FolderItems3 * This,
            /* [in] */ long grfFlags,
            /* [in] */ __RPC__in BSTR bstrFileSpec);
        
        DECLSPEC_XFGVIRT(FolderItems3, get_Verbs)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Verbs )( 
            __RPC__in FolderItems3 * This,
            /* [retval][out] */ __RPC__deref_out_opt FolderItemVerbs **ppfic);
        
        END_INTERFACE
    } FolderItems3Vtbl;

    interface FolderItems3
    {
        CONST_VTBL struct FolderItems3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define FolderItems3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define FolderItems3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define FolderItems3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define FolderItems3_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define FolderItems3_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define FolderItems3_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define FolderItems3_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define FolderItems3_get_Count(This,plCount)	\
    ( (This)->lpVtbl -> get_Count(This,plCount) ) 

#define FolderItems3_get_Application(This,ppid)	\
    ( (This)->lpVtbl -> get_Application(This,ppid) ) 

#define FolderItems3_get_Parent(This,ppid)	\
    ( (This)->lpVtbl -> get_Parent(This,ppid) ) 

#define FolderItems3_Item(This,index,ppid)	\
    ( (This)->lpVtbl -> Item(This,index,ppid) ) 

#define FolderItems3__NewEnum(This,ppunk)	\
    ( (This)->lpVtbl -> _NewEnum(This,ppunk) ) 


#define FolderItems3_InvokeVerbEx(This,vVerb,vArgs)	\
    ( (This)->lpVtbl -> InvokeVerbEx(This,vVerb,vArgs) ) 


#define FolderItems3_Filter(This,grfFlags,bstrFileSpec)	\
    ( (This)->lpVtbl -> Filter(This,grfFlags,bstrFileSpec) ) 

#define FolderItems3_get_Verbs(This,ppfic)	\
    ( (This)->lpVtbl -> get_Verbs(This,ppfic) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __FolderItems3_INTERFACE_DEFINED__ */


#ifndef __IShellLinkDual_INTERFACE_DEFINED__
#define __IShellLinkDual_INTERFACE_DEFINED__

/* interface IShellLinkDual */
/* [object][hidden][dual][oleautomation][helpstring][uuid] */ 


EXTERN_C const IID IID_IShellLinkDual;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("88A05C00-F000-11CE-8350-444553540000")
    IShellLinkDual : public IDispatch
    {
    public:
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Path( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbs) = 0;
        
        virtual /* [helpstring][propput] */ HRESULT STDMETHODCALLTYPE put_Path( 
            /* [in] */ __RPC__in BSTR bs) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Description( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbs) = 0;
        
        virtual /* [helpstring][propput] */ HRESULT STDMETHODCALLTYPE put_Description( 
            /* [in] */ __RPC__in BSTR bs) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_WorkingDirectory( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbs) = 0;
        
        virtual /* [helpstring][propput] */ HRESULT STDMETHODCALLTYPE put_WorkingDirectory( 
            /* [in] */ __RPC__in BSTR bs) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Arguments( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbs) = 0;
        
        virtual /* [helpstring][propput] */ HRESULT STDMETHODCALLTYPE put_Arguments( 
            /* [in] */ __RPC__in BSTR bs) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Hotkey( 
            /* [retval][out] */ __RPC__out int *piHK) = 0;
        
        virtual /* [helpstring][propput] */ HRESULT STDMETHODCALLTYPE put_Hotkey( 
            /* [in] */ int iHK) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_ShowCommand( 
            /* [retval][out] */ __RPC__out int *piShowCommand) = 0;
        
        virtual /* [helpstring][propput] */ HRESULT STDMETHODCALLTYPE put_ShowCommand( 
            /* [in] */ int iShowCommand) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Resolve( 
            /* [in] */ int fFlags) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetIconLocation( 
            /* [out] */ __RPC__deref_out_opt BSTR *pbs,
            /* [retval][out] */ __RPC__out int *piIcon) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetIconLocation( 
            /* [in] */ __RPC__in BSTR bs,
            /* [in] */ int iIcon) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Save( 
            /* [optional][in] */ VARIANT vWhere) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IShellLinkDualVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IShellLinkDual * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IShellLinkDual * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IShellLinkDual * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IShellLinkDual * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IShellLinkDual * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IShellLinkDual * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IShellLinkDual * This,
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
        
        DECLSPEC_XFGVIRT(IShellLinkDual, get_Path)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Path )( 
            __RPC__in IShellLinkDual * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbs);
        
        DECLSPEC_XFGVIRT(IShellLinkDual, put_Path)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Path )( 
            __RPC__in IShellLinkDual * This,
            /* [in] */ __RPC__in BSTR bs);
        
        DECLSPEC_XFGVIRT(IShellLinkDual, get_Description)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in IShellLinkDual * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbs);
        
        DECLSPEC_XFGVIRT(IShellLinkDual, put_Description)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Description )( 
            __RPC__in IShellLinkDual * This,
            /* [in] */ __RPC__in BSTR bs);
        
        DECLSPEC_XFGVIRT(IShellLinkDual, get_WorkingDirectory)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_WorkingDirectory )( 
            __RPC__in IShellLinkDual * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbs);
        
        DECLSPEC_XFGVIRT(IShellLinkDual, put_WorkingDirectory)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_WorkingDirectory )( 
            __RPC__in IShellLinkDual * This,
            /* [in] */ __RPC__in BSTR bs);
        
        DECLSPEC_XFGVIRT(IShellLinkDual, get_Arguments)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Arguments )( 
            __RPC__in IShellLinkDual * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbs);
        
        DECLSPEC_XFGVIRT(IShellLinkDual, put_Arguments)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Arguments )( 
            __RPC__in IShellLinkDual * This,
            /* [in] */ __RPC__in BSTR bs);
        
        DECLSPEC_XFGVIRT(IShellLinkDual, get_Hotkey)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Hotkey )( 
            __RPC__in IShellLinkDual * This,
            /* [retval][out] */ __RPC__out int *piHK);
        
        DECLSPEC_XFGVIRT(IShellLinkDual, put_Hotkey)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Hotkey )( 
            __RPC__in IShellLinkDual * This,
            /* [in] */ int iHK);
        
        DECLSPEC_XFGVIRT(IShellLinkDual, get_ShowCommand)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ShowCommand )( 
            __RPC__in IShellLinkDual * This,
            /* [retval][out] */ __RPC__out int *piShowCommand);
        
        DECLSPEC_XFGVIRT(IShellLinkDual, put_ShowCommand)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ShowCommand )( 
            __RPC__in IShellLinkDual * This,
            /* [in] */ int iShowCommand);
        
        DECLSPEC_XFGVIRT(IShellLinkDual, Resolve)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Resolve )( 
            __RPC__in IShellLinkDual * This,
            /* [in] */ int fFlags);
        
        DECLSPEC_XFGVIRT(IShellLinkDual, GetIconLocation)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetIconLocation )( 
            __RPC__in IShellLinkDual * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbs,
            /* [retval][out] */ __RPC__out int *piIcon);
        
        DECLSPEC_XFGVIRT(IShellLinkDual, SetIconLocation)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetIconLocation )( 
            __RPC__in IShellLinkDual * This,
            /* [in] */ __RPC__in BSTR bs,
            /* [in] */ int iIcon);
        
        DECLSPEC_XFGVIRT(IShellLinkDual, Save)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Save )( 
            __RPC__in IShellLinkDual * This,
            /* [optional][in] */ VARIANT vWhere);
        
        END_INTERFACE
    } IShellLinkDualVtbl;

    interface IShellLinkDual
    {
        CONST_VTBL struct IShellLinkDualVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IShellLinkDual_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IShellLinkDual_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IShellLinkDual_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IShellLinkDual_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IShellLinkDual_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IShellLinkDual_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IShellLinkDual_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IShellLinkDual_get_Path(This,pbs)	\
    ( (This)->lpVtbl -> get_Path(This,pbs) ) 

#define IShellLinkDual_put_Path(This,bs)	\
    ( (This)->lpVtbl -> put_Path(This,bs) ) 

#define IShellLinkDual_get_Description(This,pbs)	\
    ( (This)->lpVtbl -> get_Description(This,pbs) ) 

#define IShellLinkDual_put_Description(This,bs)	\
    ( (This)->lpVtbl -> put_Description(This,bs) ) 

#define IShellLinkDual_get_WorkingDirectory(This,pbs)	\
    ( (This)->lpVtbl -> get_WorkingDirectory(This,pbs) ) 

#define IShellLinkDual_put_WorkingDirectory(This,bs)	\
    ( (This)->lpVtbl -> put_WorkingDirectory(This,bs) ) 

#define IShellLinkDual_get_Arguments(This,pbs)	\
    ( (This)->lpVtbl -> get_Arguments(This,pbs) ) 

#define IShellLinkDual_put_Arguments(This,bs)	\
    ( (This)->lpVtbl -> put_Arguments(This,bs) ) 

#define IShellLinkDual_get_Hotkey(This,piHK)	\
    ( (This)->lpVtbl -> get_Hotkey(This,piHK) ) 

#define IShellLinkDual_put_Hotkey(This,iHK)	\
    ( (This)->lpVtbl -> put_Hotkey(This,iHK) ) 

#define IShellLinkDual_get_ShowCommand(This,piShowCommand)	\
    ( (This)->lpVtbl -> get_ShowCommand(This,piShowCommand) ) 

#define IShellLinkDual_put_ShowCommand(This,iShowCommand)	\
    ( (This)->lpVtbl -> put_ShowCommand(This,iShowCommand) ) 

#define IShellLinkDual_Resolve(This,fFlags)	\
    ( (This)->lpVtbl -> Resolve(This,fFlags) ) 

#define IShellLinkDual_GetIconLocation(This,pbs,piIcon)	\
    ( (This)->lpVtbl -> GetIconLocation(This,pbs,piIcon) ) 

#define IShellLinkDual_SetIconLocation(This,bs,iIcon)	\
    ( (This)->lpVtbl -> SetIconLocation(This,bs,iIcon) ) 

#define IShellLinkDual_Save(This,vWhere)	\
    ( (This)->lpVtbl -> Save(This,vWhere) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IShellLinkDual_INTERFACE_DEFINED__ */


#ifndef __IShellLinkDual2_INTERFACE_DEFINED__
#define __IShellLinkDual2_INTERFACE_DEFINED__

/* interface IShellLinkDual2 */
/* [object][hidden][dual][oleautomation][helpstring][uuid] */ 


EXTERN_C const IID IID_IShellLinkDual2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("317EE249-F12E-11d2-B1E4-00C04F8EEB3E")
    IShellLinkDual2 : public IShellLinkDual
    {
    public:
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Target( 
            /* [retval][out] */ __RPC__deref_out_opt FolderItem **ppfi) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IShellLinkDual2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IShellLinkDual2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IShellLinkDual2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IShellLinkDual2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IShellLinkDual2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IShellLinkDual2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IShellLinkDual2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IShellLinkDual2 * This,
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
        
        DECLSPEC_XFGVIRT(IShellLinkDual, get_Path)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Path )( 
            __RPC__in IShellLinkDual2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbs);
        
        DECLSPEC_XFGVIRT(IShellLinkDual, put_Path)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Path )( 
            __RPC__in IShellLinkDual2 * This,
            /* [in] */ __RPC__in BSTR bs);
        
        DECLSPEC_XFGVIRT(IShellLinkDual, get_Description)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in IShellLinkDual2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbs);
        
        DECLSPEC_XFGVIRT(IShellLinkDual, put_Description)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Description )( 
            __RPC__in IShellLinkDual2 * This,
            /* [in] */ __RPC__in BSTR bs);
        
        DECLSPEC_XFGVIRT(IShellLinkDual, get_WorkingDirectory)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_WorkingDirectory )( 
            __RPC__in IShellLinkDual2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbs);
        
        DECLSPEC_XFGVIRT(IShellLinkDual, put_WorkingDirectory)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_WorkingDirectory )( 
            __RPC__in IShellLinkDual2 * This,
            /* [in] */ __RPC__in BSTR bs);
        
        DECLSPEC_XFGVIRT(IShellLinkDual, get_Arguments)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Arguments )( 
            __RPC__in IShellLinkDual2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbs);
        
        DECLSPEC_XFGVIRT(IShellLinkDual, put_Arguments)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Arguments )( 
            __RPC__in IShellLinkDual2 * This,
            /* [in] */ __RPC__in BSTR bs);
        
        DECLSPEC_XFGVIRT(IShellLinkDual, get_Hotkey)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Hotkey )( 
            __RPC__in IShellLinkDual2 * This,
            /* [retval][out] */ __RPC__out int *piHK);
        
        DECLSPEC_XFGVIRT(IShellLinkDual, put_Hotkey)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Hotkey )( 
            __RPC__in IShellLinkDual2 * This,
            /* [in] */ int iHK);
        
        DECLSPEC_XFGVIRT(IShellLinkDual, get_ShowCommand)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ShowCommand )( 
            __RPC__in IShellLinkDual2 * This,
            /* [retval][out] */ __RPC__out int *piShowCommand);
        
        DECLSPEC_XFGVIRT(IShellLinkDual, put_ShowCommand)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ShowCommand )( 
            __RPC__in IShellLinkDual2 * This,
            /* [in] */ int iShowCommand);
        
        DECLSPEC_XFGVIRT(IShellLinkDual, Resolve)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Resolve )( 
            __RPC__in IShellLinkDual2 * This,
            /* [in] */ int fFlags);
        
        DECLSPEC_XFGVIRT(IShellLinkDual, GetIconLocation)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetIconLocation )( 
            __RPC__in IShellLinkDual2 * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbs,
            /* [retval][out] */ __RPC__out int *piIcon);
        
        DECLSPEC_XFGVIRT(IShellLinkDual, SetIconLocation)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetIconLocation )( 
            __RPC__in IShellLinkDual2 * This,
            /* [in] */ __RPC__in BSTR bs,
            /* [in] */ int iIcon);
        
        DECLSPEC_XFGVIRT(IShellLinkDual, Save)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Save )( 
            __RPC__in IShellLinkDual2 * This,
            /* [optional][in] */ VARIANT vWhere);
        
        DECLSPEC_XFGVIRT(IShellLinkDual2, get_Target)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Target )( 
            __RPC__in IShellLinkDual2 * This,
            /* [retval][out] */ __RPC__deref_out_opt FolderItem **ppfi);
        
        END_INTERFACE
    } IShellLinkDual2Vtbl;

    interface IShellLinkDual2
    {
        CONST_VTBL struct IShellLinkDual2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IShellLinkDual2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IShellLinkDual2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IShellLinkDual2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IShellLinkDual2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IShellLinkDual2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IShellLinkDual2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IShellLinkDual2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IShellLinkDual2_get_Path(This,pbs)	\
    ( (This)->lpVtbl -> get_Path(This,pbs) ) 

#define IShellLinkDual2_put_Path(This,bs)	\
    ( (This)->lpVtbl -> put_Path(This,bs) ) 

#define IShellLinkDual2_get_Description(This,pbs)	\
    ( (This)->lpVtbl -> get_Description(This,pbs) ) 

#define IShellLinkDual2_put_Description(This,bs)	\
    ( (This)->lpVtbl -> put_Description(This,bs) ) 

#define IShellLinkDual2_get_WorkingDirectory(This,pbs)	\
    ( (This)->lpVtbl -> get_WorkingDirectory(This,pbs) ) 

#define IShellLinkDual2_put_WorkingDirectory(This,bs)	\
    ( (This)->lpVtbl -> put_WorkingDirectory(This,bs) ) 

#define IShellLinkDual2_get_Arguments(This,pbs)	\
    ( (This)->lpVtbl -> get_Arguments(This,pbs) ) 

#define IShellLinkDual2_put_Arguments(This,bs)	\
    ( (This)->lpVtbl -> put_Arguments(This,bs) ) 

#define IShellLinkDual2_get_Hotkey(This,piHK)	\
    ( (This)->lpVtbl -> get_Hotkey(This,piHK) ) 

#define IShellLinkDual2_put_Hotkey(This,iHK)	\
    ( (This)->lpVtbl -> put_Hotkey(This,iHK) ) 

#define IShellLinkDual2_get_ShowCommand(This,piShowCommand)	\
    ( (This)->lpVtbl -> get_ShowCommand(This,piShowCommand) ) 

#define IShellLinkDual2_put_ShowCommand(This,iShowCommand)	\
    ( (This)->lpVtbl -> put_ShowCommand(This,iShowCommand) ) 

#define IShellLinkDual2_Resolve(This,fFlags)	\
    ( (This)->lpVtbl -> Resolve(This,fFlags) ) 

#define IShellLinkDual2_GetIconLocation(This,pbs,piIcon)	\
    ( (This)->lpVtbl -> GetIconLocation(This,pbs,piIcon) ) 

#define IShellLinkDual2_SetIconLocation(This,bs,iIcon)	\
    ( (This)->lpVtbl -> SetIconLocation(This,bs,iIcon) ) 

#define IShellLinkDual2_Save(This,vWhere)	\
    ( (This)->lpVtbl -> Save(This,vWhere) ) 


#define IShellLinkDual2_get_Target(This,ppfi)	\
    ( (This)->lpVtbl -> get_Target(This,ppfi) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IShellLinkDual2_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_ShellLinkObject;

#ifdef __cplusplus

class DECLSPEC_UUID("11219420-1768-11d1-95BE-00609797EA4F")
ShellLinkObject;
#endif

#ifndef __IShellFolderViewDual_INTERFACE_DEFINED__
#define __IShellFolderViewDual_INTERFACE_DEFINED__

/* interface IShellFolderViewDual */
/* [object][dual][hidden][oleautomation][helpstring][uuid] */ 


EXTERN_C const IID IID_IShellFolderViewDual;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("E7A1AF80-4D96-11CF-960C-0080C7F4EE85")
    IShellFolderViewDual : public IDispatch
    {
    public:
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Application( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppid) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Parent( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppid) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Folder( 
            /* [retval][out] */ __RPC__deref_out_opt Folder **ppid) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SelectedItems( 
            /* [retval][out] */ __RPC__deref_out_opt FolderItems **ppid) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_FocusedItem( 
            /* [retval][out] */ __RPC__deref_out_opt FolderItem **ppid) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SelectItem( 
            /* [in] */ __RPC__in VARIANT *pvfi,
            /* [in] */ int dwFlags) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE PopupItemMenu( 
            /* [in] */ __RPC__in_opt FolderItem *pfi,
            /* [optional][in] */ VARIANT vx,
            /* [optional][in] */ VARIANT vy,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbs) = 0;
        
        virtual /* [helpcontext][helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Script( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppDisp) = 0;
        
        virtual /* [helpcontext][helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_ViewOptions( 
            /* [retval][out] */ __RPC__out long *plViewOptions) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IShellFolderViewDualVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IShellFolderViewDual * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IShellFolderViewDual * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IShellFolderViewDual * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IShellFolderViewDual * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IShellFolderViewDual * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IShellFolderViewDual * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IShellFolderViewDual * This,
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
        
        DECLSPEC_XFGVIRT(IShellFolderViewDual, get_Application)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Application )( 
            __RPC__in IShellFolderViewDual * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppid);
        
        DECLSPEC_XFGVIRT(IShellFolderViewDual, get_Parent)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Parent )( 
            __RPC__in IShellFolderViewDual * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppid);
        
        DECLSPEC_XFGVIRT(IShellFolderViewDual, get_Folder)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Folder )( 
            __RPC__in IShellFolderViewDual * This,
            /* [retval][out] */ __RPC__deref_out_opt Folder **ppid);
        
        DECLSPEC_XFGVIRT(IShellFolderViewDual, SelectedItems)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SelectedItems )( 
            __RPC__in IShellFolderViewDual * This,
            /* [retval][out] */ __RPC__deref_out_opt FolderItems **ppid);
        
        DECLSPEC_XFGVIRT(IShellFolderViewDual, get_FocusedItem)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FocusedItem )( 
            __RPC__in IShellFolderViewDual * This,
            /* [retval][out] */ __RPC__deref_out_opt FolderItem **ppid);
        
        DECLSPEC_XFGVIRT(IShellFolderViewDual, SelectItem)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SelectItem )( 
            __RPC__in IShellFolderViewDual * This,
            /* [in] */ __RPC__in VARIANT *pvfi,
            /* [in] */ int dwFlags);
        
        DECLSPEC_XFGVIRT(IShellFolderViewDual, PopupItemMenu)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *PopupItemMenu )( 
            __RPC__in IShellFolderViewDual * This,
            /* [in] */ __RPC__in_opt FolderItem *pfi,
            /* [optional][in] */ VARIANT vx,
            /* [optional][in] */ VARIANT vy,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbs);
        
        DECLSPEC_XFGVIRT(IShellFolderViewDual, get_Script)
        /* [helpcontext][helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Script )( 
            __RPC__in IShellFolderViewDual * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppDisp);
        
        DECLSPEC_XFGVIRT(IShellFolderViewDual, get_ViewOptions)
        /* [helpcontext][helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ViewOptions )( 
            __RPC__in IShellFolderViewDual * This,
            /* [retval][out] */ __RPC__out long *plViewOptions);
        
        END_INTERFACE
    } IShellFolderViewDualVtbl;

    interface IShellFolderViewDual
    {
        CONST_VTBL struct IShellFolderViewDualVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IShellFolderViewDual_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IShellFolderViewDual_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IShellFolderViewDual_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IShellFolderViewDual_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IShellFolderViewDual_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IShellFolderViewDual_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IShellFolderViewDual_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IShellFolderViewDual_get_Application(This,ppid)	\
    ( (This)->lpVtbl -> get_Application(This,ppid) ) 

#define IShellFolderViewDual_get_Parent(This,ppid)	\
    ( (This)->lpVtbl -> get_Parent(This,ppid) ) 

#define IShellFolderViewDual_get_Folder(This,ppid)	\
    ( (This)->lpVtbl -> get_Folder(This,ppid) ) 

#define IShellFolderViewDual_SelectedItems(This,ppid)	\
    ( (This)->lpVtbl -> SelectedItems(This,ppid) ) 

#define IShellFolderViewDual_get_FocusedItem(This,ppid)	\
    ( (This)->lpVtbl -> get_FocusedItem(This,ppid) ) 

#define IShellFolderViewDual_SelectItem(This,pvfi,dwFlags)	\
    ( (This)->lpVtbl -> SelectItem(This,pvfi,dwFlags) ) 

#define IShellFolderViewDual_PopupItemMenu(This,pfi,vx,vy,pbs)	\
    ( (This)->lpVtbl -> PopupItemMenu(This,pfi,vx,vy,pbs) ) 

#define IShellFolderViewDual_get_Script(This,ppDisp)	\
    ( (This)->lpVtbl -> get_Script(This,ppDisp) ) 

#define IShellFolderViewDual_get_ViewOptions(This,plViewOptions)	\
    ( (This)->lpVtbl -> get_ViewOptions(This,plViewOptions) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IShellFolderViewDual_INTERFACE_DEFINED__ */


#ifndef __IShellFolderViewDual2_INTERFACE_DEFINED__
#define __IShellFolderViewDual2_INTERFACE_DEFINED__

/* interface IShellFolderViewDual2 */
/* [object][dual][hidden][oleautomation][helpstring][uuid] */ 


EXTERN_C const IID IID_IShellFolderViewDual2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("31C147b6-0ADE-4A3C-B514-DDF932EF6D17")
    IShellFolderViewDual2 : public IShellFolderViewDual
    {
    public:
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_CurrentViewMode( 
            /* [retval][out] */ __RPC__out UINT *pViewMode) = 0;
        
        virtual /* [helpstring][propput] */ HRESULT STDMETHODCALLTYPE put_CurrentViewMode( 
            /* [in] */ UINT ViewMode) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SelectItemRelative( 
            /* [in] */ int iRelative) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IShellFolderViewDual2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IShellFolderViewDual2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IShellFolderViewDual2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IShellFolderViewDual2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IShellFolderViewDual2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IShellFolderViewDual2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IShellFolderViewDual2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IShellFolderViewDual2 * This,
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
        
        DECLSPEC_XFGVIRT(IShellFolderViewDual, get_Application)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Application )( 
            __RPC__in IShellFolderViewDual2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppid);
        
        DECLSPEC_XFGVIRT(IShellFolderViewDual, get_Parent)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Parent )( 
            __RPC__in IShellFolderViewDual2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppid);
        
        DECLSPEC_XFGVIRT(IShellFolderViewDual, get_Folder)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Folder )( 
            __RPC__in IShellFolderViewDual2 * This,
            /* [retval][out] */ __RPC__deref_out_opt Folder **ppid);
        
        DECLSPEC_XFGVIRT(IShellFolderViewDual, SelectedItems)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SelectedItems )( 
            __RPC__in IShellFolderViewDual2 * This,
            /* [retval][out] */ __RPC__deref_out_opt FolderItems **ppid);
        
        DECLSPEC_XFGVIRT(IShellFolderViewDual, get_FocusedItem)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FocusedItem )( 
            __RPC__in IShellFolderViewDual2 * This,
            /* [retval][out] */ __RPC__deref_out_opt FolderItem **ppid);
        
        DECLSPEC_XFGVIRT(IShellFolderViewDual, SelectItem)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SelectItem )( 
            __RPC__in IShellFolderViewDual2 * This,
            /* [in] */ __RPC__in VARIANT *pvfi,
            /* [in] */ int dwFlags);
        
        DECLSPEC_XFGVIRT(IShellFolderViewDual, PopupItemMenu)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *PopupItemMenu )( 
            __RPC__in IShellFolderViewDual2 * This,
            /* [in] */ __RPC__in_opt FolderItem *pfi,
            /* [optional][in] */ VARIANT vx,
            /* [optional][in] */ VARIANT vy,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbs);
        
        DECLSPEC_XFGVIRT(IShellFolderViewDual, get_Script)
        /* [helpcontext][helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Script )( 
            __RPC__in IShellFolderViewDual2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppDisp);
        
        DECLSPEC_XFGVIRT(IShellFolderViewDual, get_ViewOptions)
        /* [helpcontext][helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ViewOptions )( 
            __RPC__in IShellFolderViewDual2 * This,
            /* [retval][out] */ __RPC__out long *plViewOptions);
        
        DECLSPEC_XFGVIRT(IShellFolderViewDual2, get_CurrentViewMode)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CurrentViewMode )( 
            __RPC__in IShellFolderViewDual2 * This,
            /* [retval][out] */ __RPC__out UINT *pViewMode);
        
        DECLSPEC_XFGVIRT(IShellFolderViewDual2, put_CurrentViewMode)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_CurrentViewMode )( 
            __RPC__in IShellFolderViewDual2 * This,
            /* [in] */ UINT ViewMode);
        
        DECLSPEC_XFGVIRT(IShellFolderViewDual2, SelectItemRelative)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SelectItemRelative )( 
            __RPC__in IShellFolderViewDual2 * This,
            /* [in] */ int iRelative);
        
        END_INTERFACE
    } IShellFolderViewDual2Vtbl;

    interface IShellFolderViewDual2
    {
        CONST_VTBL struct IShellFolderViewDual2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IShellFolderViewDual2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IShellFolderViewDual2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IShellFolderViewDual2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IShellFolderViewDual2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IShellFolderViewDual2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IShellFolderViewDual2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IShellFolderViewDual2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IShellFolderViewDual2_get_Application(This,ppid)	\
    ( (This)->lpVtbl -> get_Application(This,ppid) ) 

#define IShellFolderViewDual2_get_Parent(This,ppid)	\
    ( (This)->lpVtbl -> get_Parent(This,ppid) ) 

#define IShellFolderViewDual2_get_Folder(This,ppid)	\
    ( (This)->lpVtbl -> get_Folder(This,ppid) ) 

#define IShellFolderViewDual2_SelectedItems(This,ppid)	\
    ( (This)->lpVtbl -> SelectedItems(This,ppid) ) 

#define IShellFolderViewDual2_get_FocusedItem(This,ppid)	\
    ( (This)->lpVtbl -> get_FocusedItem(This,ppid) ) 

#define IShellFolderViewDual2_SelectItem(This,pvfi,dwFlags)	\
    ( (This)->lpVtbl -> SelectItem(This,pvfi,dwFlags) ) 

#define IShellFolderViewDual2_PopupItemMenu(This,pfi,vx,vy,pbs)	\
    ( (This)->lpVtbl -> PopupItemMenu(This,pfi,vx,vy,pbs) ) 

#define IShellFolderViewDual2_get_Script(This,ppDisp)	\
    ( (This)->lpVtbl -> get_Script(This,ppDisp) ) 

#define IShellFolderViewDual2_get_ViewOptions(This,plViewOptions)	\
    ( (This)->lpVtbl -> get_ViewOptions(This,plViewOptions) ) 


#define IShellFolderViewDual2_get_CurrentViewMode(This,pViewMode)	\
    ( (This)->lpVtbl -> get_CurrentViewMode(This,pViewMode) ) 

#define IShellFolderViewDual2_put_CurrentViewMode(This,ViewMode)	\
    ( (This)->lpVtbl -> put_CurrentViewMode(This,ViewMode) ) 

#define IShellFolderViewDual2_SelectItemRelative(This,iRelative)	\
    ( (This)->lpVtbl -> SelectItemRelative(This,iRelative) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IShellFolderViewDual2_INTERFACE_DEFINED__ */


#ifndef __IShellFolderViewDual3_INTERFACE_DEFINED__
#define __IShellFolderViewDual3_INTERFACE_DEFINED__

/* interface IShellFolderViewDual3 */
/* [object][dual][hidden][oleautomation][helpstring][uuid] */ 


EXTERN_C const IID IID_IShellFolderViewDual3;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("29EC8E6C-46D3-411f-BAAA-611A6C9CAC66")
    IShellFolderViewDual3 : public IShellFolderViewDual2
    {
    public:
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_GroupBy( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrGroupBy) = 0;
        
        virtual /* [helpstring][propput] */ HRESULT STDMETHODCALLTYPE put_GroupBy( 
            /* [in] */ __RPC__in BSTR bstrGroupBy) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_FolderFlags( 
            /* [retval][out] */ __RPC__out DWORD *pdwFlags) = 0;
        
        virtual /* [helpstring][propput] */ HRESULT STDMETHODCALLTYPE put_FolderFlags( 
            /* [in] */ DWORD dwFlags) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_SortColumns( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrSortColumns) = 0;
        
        virtual /* [helpstring][propput] */ HRESULT STDMETHODCALLTYPE put_SortColumns( 
            /* [in] */ __RPC__in BSTR bstrSortColumns) = 0;
        
        virtual /* [helpstring][propput] */ HRESULT STDMETHODCALLTYPE put_IconSize( 
            /* [in] */ int iIconSize) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_IconSize( 
            /* [retval][out] */ __RPC__out int *piIconSize) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE FilterView( 
            /* [unique][in] */ __RPC__in_opt BSTR bstrFilterText) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IShellFolderViewDual3Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IShellFolderViewDual3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IShellFolderViewDual3 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IShellFolderViewDual3 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IShellFolderViewDual3 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IShellFolderViewDual3 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IShellFolderViewDual3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IShellFolderViewDual3 * This,
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
        
        DECLSPEC_XFGVIRT(IShellFolderViewDual, get_Application)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Application )( 
            __RPC__in IShellFolderViewDual3 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppid);
        
        DECLSPEC_XFGVIRT(IShellFolderViewDual, get_Parent)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Parent )( 
            __RPC__in IShellFolderViewDual3 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppid);
        
        DECLSPEC_XFGVIRT(IShellFolderViewDual, get_Folder)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Folder )( 
            __RPC__in IShellFolderViewDual3 * This,
            /* [retval][out] */ __RPC__deref_out_opt Folder **ppid);
        
        DECLSPEC_XFGVIRT(IShellFolderViewDual, SelectedItems)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SelectedItems )( 
            __RPC__in IShellFolderViewDual3 * This,
            /* [retval][out] */ __RPC__deref_out_opt FolderItems **ppid);
        
        DECLSPEC_XFGVIRT(IShellFolderViewDual, get_FocusedItem)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FocusedItem )( 
            __RPC__in IShellFolderViewDual3 * This,
            /* [retval][out] */ __RPC__deref_out_opt FolderItem **ppid);
        
        DECLSPEC_XFGVIRT(IShellFolderViewDual, SelectItem)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SelectItem )( 
            __RPC__in IShellFolderViewDual3 * This,
            /* [in] */ __RPC__in VARIANT *pvfi,
            /* [in] */ int dwFlags);
        
        DECLSPEC_XFGVIRT(IShellFolderViewDual, PopupItemMenu)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *PopupItemMenu )( 
            __RPC__in IShellFolderViewDual3 * This,
            /* [in] */ __RPC__in_opt FolderItem *pfi,
            /* [optional][in] */ VARIANT vx,
            /* [optional][in] */ VARIANT vy,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbs);
        
        DECLSPEC_XFGVIRT(IShellFolderViewDual, get_Script)
        /* [helpcontext][helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Script )( 
            __RPC__in IShellFolderViewDual3 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppDisp);
        
        DECLSPEC_XFGVIRT(IShellFolderViewDual, get_ViewOptions)
        /* [helpcontext][helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ViewOptions )( 
            __RPC__in IShellFolderViewDual3 * This,
            /* [retval][out] */ __RPC__out long *plViewOptions);
        
        DECLSPEC_XFGVIRT(IShellFolderViewDual2, get_CurrentViewMode)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CurrentViewMode )( 
            __RPC__in IShellFolderViewDual3 * This,
            /* [retval][out] */ __RPC__out UINT *pViewMode);
        
        DECLSPEC_XFGVIRT(IShellFolderViewDual2, put_CurrentViewMode)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_CurrentViewMode )( 
            __RPC__in IShellFolderViewDual3 * This,
            /* [in] */ UINT ViewMode);
        
        DECLSPEC_XFGVIRT(IShellFolderViewDual2, SelectItemRelative)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SelectItemRelative )( 
            __RPC__in IShellFolderViewDual3 * This,
            /* [in] */ int iRelative);
        
        DECLSPEC_XFGVIRT(IShellFolderViewDual3, get_GroupBy)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_GroupBy )( 
            __RPC__in IShellFolderViewDual3 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrGroupBy);
        
        DECLSPEC_XFGVIRT(IShellFolderViewDual3, put_GroupBy)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_GroupBy )( 
            __RPC__in IShellFolderViewDual3 * This,
            /* [in] */ __RPC__in BSTR bstrGroupBy);
        
        DECLSPEC_XFGVIRT(IShellFolderViewDual3, get_FolderFlags)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FolderFlags )( 
            __RPC__in IShellFolderViewDual3 * This,
            /* [retval][out] */ __RPC__out DWORD *pdwFlags);
        
        DECLSPEC_XFGVIRT(IShellFolderViewDual3, put_FolderFlags)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_FolderFlags )( 
            __RPC__in IShellFolderViewDual3 * This,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IShellFolderViewDual3, get_SortColumns)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SortColumns )( 
            __RPC__in IShellFolderViewDual3 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrSortColumns);
        
        DECLSPEC_XFGVIRT(IShellFolderViewDual3, put_SortColumns)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SortColumns )( 
            __RPC__in IShellFolderViewDual3 * This,
            /* [in] */ __RPC__in BSTR bstrSortColumns);
        
        DECLSPEC_XFGVIRT(IShellFolderViewDual3, put_IconSize)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_IconSize )( 
            __RPC__in IShellFolderViewDual3 * This,
            /* [in] */ int iIconSize);
        
        DECLSPEC_XFGVIRT(IShellFolderViewDual3, get_IconSize)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_IconSize )( 
            __RPC__in IShellFolderViewDual3 * This,
            /* [retval][out] */ __RPC__out int *piIconSize);
        
        DECLSPEC_XFGVIRT(IShellFolderViewDual3, FilterView)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *FilterView )( 
            __RPC__in IShellFolderViewDual3 * This,
            /* [unique][in] */ __RPC__in_opt BSTR bstrFilterText);
        
        END_INTERFACE
    } IShellFolderViewDual3Vtbl;

    interface IShellFolderViewDual3
    {
        CONST_VTBL struct IShellFolderViewDual3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IShellFolderViewDual3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IShellFolderViewDual3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IShellFolderViewDual3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IShellFolderViewDual3_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IShellFolderViewDual3_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IShellFolderViewDual3_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IShellFolderViewDual3_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IShellFolderViewDual3_get_Application(This,ppid)	\
    ( (This)->lpVtbl -> get_Application(This,ppid) ) 

#define IShellFolderViewDual3_get_Parent(This,ppid)	\
    ( (This)->lpVtbl -> get_Parent(This,ppid) ) 

#define IShellFolderViewDual3_get_Folder(This,ppid)	\
    ( (This)->lpVtbl -> get_Folder(This,ppid) ) 

#define IShellFolderViewDual3_SelectedItems(This,ppid)	\
    ( (This)->lpVtbl -> SelectedItems(This,ppid) ) 

#define IShellFolderViewDual3_get_FocusedItem(This,ppid)	\
    ( (This)->lpVtbl -> get_FocusedItem(This,ppid) ) 

#define IShellFolderViewDual3_SelectItem(This,pvfi,dwFlags)	\
    ( (This)->lpVtbl -> SelectItem(This,pvfi,dwFlags) ) 

#define IShellFolderViewDual3_PopupItemMenu(This,pfi,vx,vy,pbs)	\
    ( (This)->lpVtbl -> PopupItemMenu(This,pfi,vx,vy,pbs) ) 

#define IShellFolderViewDual3_get_Script(This,ppDisp)	\
    ( (This)->lpVtbl -> get_Script(This,ppDisp) ) 

#define IShellFolderViewDual3_get_ViewOptions(This,plViewOptions)	\
    ( (This)->lpVtbl -> get_ViewOptions(This,plViewOptions) ) 


#define IShellFolderViewDual3_get_CurrentViewMode(This,pViewMode)	\
    ( (This)->lpVtbl -> get_CurrentViewMode(This,pViewMode) ) 

#define IShellFolderViewDual3_put_CurrentViewMode(This,ViewMode)	\
    ( (This)->lpVtbl -> put_CurrentViewMode(This,ViewMode) ) 

#define IShellFolderViewDual3_SelectItemRelative(This,iRelative)	\
    ( (This)->lpVtbl -> SelectItemRelative(This,iRelative) ) 


#define IShellFolderViewDual3_get_GroupBy(This,pbstrGroupBy)	\
    ( (This)->lpVtbl -> get_GroupBy(This,pbstrGroupBy) ) 

#define IShellFolderViewDual3_put_GroupBy(This,bstrGroupBy)	\
    ( (This)->lpVtbl -> put_GroupBy(This,bstrGroupBy) ) 

#define IShellFolderViewDual3_get_FolderFlags(This,pdwFlags)	\
    ( (This)->lpVtbl -> get_FolderFlags(This,pdwFlags) ) 

#define IShellFolderViewDual3_put_FolderFlags(This,dwFlags)	\
    ( (This)->lpVtbl -> put_FolderFlags(This,dwFlags) ) 

#define IShellFolderViewDual3_get_SortColumns(This,pbstrSortColumns)	\
    ( (This)->lpVtbl -> get_SortColumns(This,pbstrSortColumns) ) 

#define IShellFolderViewDual3_put_SortColumns(This,bstrSortColumns)	\
    ( (This)->lpVtbl -> put_SortColumns(This,bstrSortColumns) ) 

#define IShellFolderViewDual3_put_IconSize(This,iIconSize)	\
    ( (This)->lpVtbl -> put_IconSize(This,iIconSize) ) 

#define IShellFolderViewDual3_get_IconSize(This,piIconSize)	\
    ( (This)->lpVtbl -> get_IconSize(This,piIconSize) ) 

#define IShellFolderViewDual3_FilterView(This,bstrFilterText)	\
    ( (This)->lpVtbl -> FilterView(This,bstrFilterText) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IShellFolderViewDual3_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_ShellFolderView;

#ifdef __cplusplus

class DECLSPEC_UUID("62112AA1-EBE4-11cf-A5FB-0020AFE7292D")
ShellFolderView;
#endif

#ifndef __IShellDispatch_INTERFACE_DEFINED__
#define __IShellDispatch_INTERFACE_DEFINED__

/* interface IShellDispatch */
/* [object][dual][hidden][oleautomation][helpstring][uuid] */ 


EXTERN_C const IID IID_IShellDispatch;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D8F015C0-C278-11CE-A49E-444553540000")
    IShellDispatch : public IDispatch
    {
    public:
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Application( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppid) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Parent( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppid) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE NameSpace( 
            /* [in] */ VARIANT vDir,
            /* [retval][out] */ __RPC__deref_out_opt Folder **ppsdf) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE BrowseForFolder( 
            /* [in] */ long Hwnd,
            /* [in] */ __RPC__in BSTR Title,
            /* [in] */ long Options,
            /* [optional][in] */ VARIANT RootFolder,
            /* [retval][out] */ __RPC__deref_out_opt Folder **ppsdf) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Windows( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppid) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Open( 
            /* [in] */ VARIANT vDir) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Explore( 
            /* [in] */ VARIANT vDir) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE MinimizeAll( void) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE UndoMinimizeALL( void) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE FileRun( void) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE CascadeWindows( void) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE TileVertically( void) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE TileHorizontally( void) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE ShutdownWindows( void) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Suspend( void) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE EjectPC( void) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetTime( void) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE TrayProperties( void) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Help( void) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE FindFiles( void) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE FindComputer( void) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE RefreshMenu( void) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE ControlPanelItem( 
            /* [in] */ __RPC__in BSTR bstrDir) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IShellDispatchVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IShellDispatch * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IShellDispatch * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IShellDispatch * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IShellDispatch * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IShellDispatch * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IShellDispatch * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IShellDispatch * This,
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
        
        DECLSPEC_XFGVIRT(IShellDispatch, get_Application)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Application )( 
            __RPC__in IShellDispatch * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppid);
        
        DECLSPEC_XFGVIRT(IShellDispatch, get_Parent)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Parent )( 
            __RPC__in IShellDispatch * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppid);
        
        DECLSPEC_XFGVIRT(IShellDispatch, NameSpace)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *NameSpace )( 
            __RPC__in IShellDispatch * This,
            /* [in] */ VARIANT vDir,
            /* [retval][out] */ __RPC__deref_out_opt Folder **ppsdf);
        
        DECLSPEC_XFGVIRT(IShellDispatch, BrowseForFolder)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *BrowseForFolder )( 
            __RPC__in IShellDispatch * This,
            /* [in] */ long Hwnd,
            /* [in] */ __RPC__in BSTR Title,
            /* [in] */ long Options,
            /* [optional][in] */ VARIANT RootFolder,
            /* [retval][out] */ __RPC__deref_out_opt Folder **ppsdf);
        
        DECLSPEC_XFGVIRT(IShellDispatch, Windows)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Windows )( 
            __RPC__in IShellDispatch * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppid);
        
        DECLSPEC_XFGVIRT(IShellDispatch, Open)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Open )( 
            __RPC__in IShellDispatch * This,
            /* [in] */ VARIANT vDir);
        
        DECLSPEC_XFGVIRT(IShellDispatch, Explore)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Explore )( 
            __RPC__in IShellDispatch * This,
            /* [in] */ VARIANT vDir);
        
        DECLSPEC_XFGVIRT(IShellDispatch, MinimizeAll)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *MinimizeAll )( 
            __RPC__in IShellDispatch * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, UndoMinimizeALL)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *UndoMinimizeALL )( 
            __RPC__in IShellDispatch * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, FileRun)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *FileRun )( 
            __RPC__in IShellDispatch * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, CascadeWindows)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *CascadeWindows )( 
            __RPC__in IShellDispatch * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, TileVertically)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *TileVertically )( 
            __RPC__in IShellDispatch * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, TileHorizontally)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *TileHorizontally )( 
            __RPC__in IShellDispatch * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, ShutdownWindows)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ShutdownWindows )( 
            __RPC__in IShellDispatch * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, Suspend)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Suspend )( 
            __RPC__in IShellDispatch * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, EjectPC)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *EjectPC )( 
            __RPC__in IShellDispatch * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, SetTime)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetTime )( 
            __RPC__in IShellDispatch * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, TrayProperties)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *TrayProperties )( 
            __RPC__in IShellDispatch * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, Help)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Help )( 
            __RPC__in IShellDispatch * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, FindFiles)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *FindFiles )( 
            __RPC__in IShellDispatch * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, FindComputer)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *FindComputer )( 
            __RPC__in IShellDispatch * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, RefreshMenu)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *RefreshMenu )( 
            __RPC__in IShellDispatch * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, ControlPanelItem)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ControlPanelItem )( 
            __RPC__in IShellDispatch * This,
            /* [in] */ __RPC__in BSTR bstrDir);
        
        END_INTERFACE
    } IShellDispatchVtbl;

    interface IShellDispatch
    {
        CONST_VTBL struct IShellDispatchVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IShellDispatch_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IShellDispatch_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IShellDispatch_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IShellDispatch_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IShellDispatch_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IShellDispatch_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IShellDispatch_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IShellDispatch_get_Application(This,ppid)	\
    ( (This)->lpVtbl -> get_Application(This,ppid) ) 

#define IShellDispatch_get_Parent(This,ppid)	\
    ( (This)->lpVtbl -> get_Parent(This,ppid) ) 

#define IShellDispatch_NameSpace(This,vDir,ppsdf)	\
    ( (This)->lpVtbl -> NameSpace(This,vDir,ppsdf) ) 

#define IShellDispatch_BrowseForFolder(This,Hwnd,Title,Options,RootFolder,ppsdf)	\
    ( (This)->lpVtbl -> BrowseForFolder(This,Hwnd,Title,Options,RootFolder,ppsdf) ) 

#define IShellDispatch_Windows(This,ppid)	\
    ( (This)->lpVtbl -> Windows(This,ppid) ) 

#define IShellDispatch_Open(This,vDir)	\
    ( (This)->lpVtbl -> Open(This,vDir) ) 

#define IShellDispatch_Explore(This,vDir)	\
    ( (This)->lpVtbl -> Explore(This,vDir) ) 

#define IShellDispatch_MinimizeAll(This)	\
    ( (This)->lpVtbl -> MinimizeAll(This) ) 

#define IShellDispatch_UndoMinimizeALL(This)	\
    ( (This)->lpVtbl -> UndoMinimizeALL(This) ) 

#define IShellDispatch_FileRun(This)	\
    ( (This)->lpVtbl -> FileRun(This) ) 

#define IShellDispatch_CascadeWindows(This)	\
    ( (This)->lpVtbl -> CascadeWindows(This) ) 

#define IShellDispatch_TileVertically(This)	\
    ( (This)->lpVtbl -> TileVertically(This) ) 

#define IShellDispatch_TileHorizontally(This)	\
    ( (This)->lpVtbl -> TileHorizontally(This) ) 

#define IShellDispatch_ShutdownWindows(This)	\
    ( (This)->lpVtbl -> ShutdownWindows(This) ) 

#define IShellDispatch_Suspend(This)	\
    ( (This)->lpVtbl -> Suspend(This) ) 

#define IShellDispatch_EjectPC(This)	\
    ( (This)->lpVtbl -> EjectPC(This) ) 

#define IShellDispatch_SetTime(This)	\
    ( (This)->lpVtbl -> SetTime(This) ) 

#define IShellDispatch_TrayProperties(This)	\
    ( (This)->lpVtbl -> TrayProperties(This) ) 

#define IShellDispatch_Help(This)	\
    ( (This)->lpVtbl -> Help(This) ) 

#define IShellDispatch_FindFiles(This)	\
    ( (This)->lpVtbl -> FindFiles(This) ) 

#define IShellDispatch_FindComputer(This)	\
    ( (This)->lpVtbl -> FindComputer(This) ) 

#define IShellDispatch_RefreshMenu(This)	\
    ( (This)->lpVtbl -> RefreshMenu(This) ) 

#define IShellDispatch_ControlPanelItem(This,bstrDir)	\
    ( (This)->lpVtbl -> ControlPanelItem(This,bstrDir) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IShellDispatch_INTERFACE_DEFINED__ */


#ifndef __IShellDispatch2_INTERFACE_DEFINED__
#define __IShellDispatch2_INTERFACE_DEFINED__

/* interface IShellDispatch2 */
/* [object][dual][hidden][oleautomation][helpstring][uuid] */ 


EXTERN_C const IID IID_IShellDispatch2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A4C6892C-3BA9-11d2-9DEA-00C04FB16162")
    IShellDispatch2 : public IShellDispatch
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE IsRestricted( 
            /* [in] */ __RPC__in BSTR Group,
            /* [in] */ __RPC__in BSTR Restriction,
            /* [retval][out] */ __RPC__out long *plRestrictValue) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE ShellExecute( 
            /* [in] */ __RPC__in BSTR File,
            /* [optional][in] */ VARIANT vArgs,
            /* [optional][in] */ VARIANT vDir,
            /* [optional][in] */ VARIANT vOperation,
            /* [optional][in] */ VARIANT vShow) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE FindPrinter( 
            /* [optional][in] */ __RPC__in BSTR name,
            /* [optional][in] */ __RPC__in BSTR location,
            /* [optional][in] */ __RPC__in BSTR model) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetSystemInformation( 
            /* [in] */ __RPC__in BSTR name,
            /* [retval][out] */ __RPC__out VARIANT *pv) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE ServiceStart( 
            /* [in] */ __RPC__in BSTR ServiceName,
            /* [in] */ VARIANT Persistent,
            /* [retval][out] */ __RPC__out VARIANT *pSuccess) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE ServiceStop( 
            /* [in] */ __RPC__in BSTR ServiceName,
            /* [in] */ VARIANT Persistent,
            /* [retval][out] */ __RPC__out VARIANT *pSuccess) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE IsServiceRunning( 
            /* [in] */ __RPC__in BSTR ServiceName,
            /* [retval][out] */ __RPC__out VARIANT *pRunning) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE CanStartStopService( 
            /* [in] */ __RPC__in BSTR ServiceName,
            /* [retval][out] */ __RPC__out VARIANT *pCanStartStop) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE ShowBrowserBar( 
            /* [in] */ __RPC__in BSTR bstrClsid,
            /* [in] */ VARIANT bShow,
            /* [retval][out] */ __RPC__out VARIANT *pSuccess) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IShellDispatch2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IShellDispatch2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IShellDispatch2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IShellDispatch2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IShellDispatch2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IShellDispatch2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IShellDispatch2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IShellDispatch2 * This,
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
        
        DECLSPEC_XFGVIRT(IShellDispatch, get_Application)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Application )( 
            __RPC__in IShellDispatch2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppid);
        
        DECLSPEC_XFGVIRT(IShellDispatch, get_Parent)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Parent )( 
            __RPC__in IShellDispatch2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppid);
        
        DECLSPEC_XFGVIRT(IShellDispatch, NameSpace)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *NameSpace )( 
            __RPC__in IShellDispatch2 * This,
            /* [in] */ VARIANT vDir,
            /* [retval][out] */ __RPC__deref_out_opt Folder **ppsdf);
        
        DECLSPEC_XFGVIRT(IShellDispatch, BrowseForFolder)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *BrowseForFolder )( 
            __RPC__in IShellDispatch2 * This,
            /* [in] */ long Hwnd,
            /* [in] */ __RPC__in BSTR Title,
            /* [in] */ long Options,
            /* [optional][in] */ VARIANT RootFolder,
            /* [retval][out] */ __RPC__deref_out_opt Folder **ppsdf);
        
        DECLSPEC_XFGVIRT(IShellDispatch, Windows)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Windows )( 
            __RPC__in IShellDispatch2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppid);
        
        DECLSPEC_XFGVIRT(IShellDispatch, Open)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Open )( 
            __RPC__in IShellDispatch2 * This,
            /* [in] */ VARIANT vDir);
        
        DECLSPEC_XFGVIRT(IShellDispatch, Explore)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Explore )( 
            __RPC__in IShellDispatch2 * This,
            /* [in] */ VARIANT vDir);
        
        DECLSPEC_XFGVIRT(IShellDispatch, MinimizeAll)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *MinimizeAll )( 
            __RPC__in IShellDispatch2 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, UndoMinimizeALL)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *UndoMinimizeALL )( 
            __RPC__in IShellDispatch2 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, FileRun)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *FileRun )( 
            __RPC__in IShellDispatch2 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, CascadeWindows)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *CascadeWindows )( 
            __RPC__in IShellDispatch2 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, TileVertically)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *TileVertically )( 
            __RPC__in IShellDispatch2 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, TileHorizontally)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *TileHorizontally )( 
            __RPC__in IShellDispatch2 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, ShutdownWindows)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ShutdownWindows )( 
            __RPC__in IShellDispatch2 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, Suspend)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Suspend )( 
            __RPC__in IShellDispatch2 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, EjectPC)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *EjectPC )( 
            __RPC__in IShellDispatch2 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, SetTime)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetTime )( 
            __RPC__in IShellDispatch2 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, TrayProperties)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *TrayProperties )( 
            __RPC__in IShellDispatch2 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, Help)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Help )( 
            __RPC__in IShellDispatch2 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, FindFiles)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *FindFiles )( 
            __RPC__in IShellDispatch2 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, FindComputer)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *FindComputer )( 
            __RPC__in IShellDispatch2 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, RefreshMenu)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *RefreshMenu )( 
            __RPC__in IShellDispatch2 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, ControlPanelItem)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ControlPanelItem )( 
            __RPC__in IShellDispatch2 * This,
            /* [in] */ __RPC__in BSTR bstrDir);
        
        DECLSPEC_XFGVIRT(IShellDispatch2, IsRestricted)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *IsRestricted )( 
            __RPC__in IShellDispatch2 * This,
            /* [in] */ __RPC__in BSTR Group,
            /* [in] */ __RPC__in BSTR Restriction,
            /* [retval][out] */ __RPC__out long *plRestrictValue);
        
        DECLSPEC_XFGVIRT(IShellDispatch2, ShellExecute)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ShellExecute )( 
            __RPC__in IShellDispatch2 * This,
            /* [in] */ __RPC__in BSTR File,
            /* [optional][in] */ VARIANT vArgs,
            /* [optional][in] */ VARIANT vDir,
            /* [optional][in] */ VARIANT vOperation,
            /* [optional][in] */ VARIANT vShow);
        
        DECLSPEC_XFGVIRT(IShellDispatch2, FindPrinter)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *FindPrinter )( 
            __RPC__in IShellDispatch2 * This,
            /* [optional][in] */ __RPC__in BSTR name,
            /* [optional][in] */ __RPC__in BSTR location,
            /* [optional][in] */ __RPC__in BSTR model);
        
        DECLSPEC_XFGVIRT(IShellDispatch2, GetSystemInformation)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetSystemInformation )( 
            __RPC__in IShellDispatch2 * This,
            /* [in] */ __RPC__in BSTR name,
            /* [retval][out] */ __RPC__out VARIANT *pv);
        
        DECLSPEC_XFGVIRT(IShellDispatch2, ServiceStart)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ServiceStart )( 
            __RPC__in IShellDispatch2 * This,
            /* [in] */ __RPC__in BSTR ServiceName,
            /* [in] */ VARIANT Persistent,
            /* [retval][out] */ __RPC__out VARIANT *pSuccess);
        
        DECLSPEC_XFGVIRT(IShellDispatch2, ServiceStop)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ServiceStop )( 
            __RPC__in IShellDispatch2 * This,
            /* [in] */ __RPC__in BSTR ServiceName,
            /* [in] */ VARIANT Persistent,
            /* [retval][out] */ __RPC__out VARIANT *pSuccess);
        
        DECLSPEC_XFGVIRT(IShellDispatch2, IsServiceRunning)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *IsServiceRunning )( 
            __RPC__in IShellDispatch2 * This,
            /* [in] */ __RPC__in BSTR ServiceName,
            /* [retval][out] */ __RPC__out VARIANT *pRunning);
        
        DECLSPEC_XFGVIRT(IShellDispatch2, CanStartStopService)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *CanStartStopService )( 
            __RPC__in IShellDispatch2 * This,
            /* [in] */ __RPC__in BSTR ServiceName,
            /* [retval][out] */ __RPC__out VARIANT *pCanStartStop);
        
        DECLSPEC_XFGVIRT(IShellDispatch2, ShowBrowserBar)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ShowBrowserBar )( 
            __RPC__in IShellDispatch2 * This,
            /* [in] */ __RPC__in BSTR bstrClsid,
            /* [in] */ VARIANT bShow,
            /* [retval][out] */ __RPC__out VARIANT *pSuccess);
        
        END_INTERFACE
    } IShellDispatch2Vtbl;

    interface IShellDispatch2
    {
        CONST_VTBL struct IShellDispatch2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IShellDispatch2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IShellDispatch2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IShellDispatch2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IShellDispatch2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IShellDispatch2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IShellDispatch2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IShellDispatch2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IShellDispatch2_get_Application(This,ppid)	\
    ( (This)->lpVtbl -> get_Application(This,ppid) ) 

#define IShellDispatch2_get_Parent(This,ppid)	\
    ( (This)->lpVtbl -> get_Parent(This,ppid) ) 

#define IShellDispatch2_NameSpace(This,vDir,ppsdf)	\
    ( (This)->lpVtbl -> NameSpace(This,vDir,ppsdf) ) 

#define IShellDispatch2_BrowseForFolder(This,Hwnd,Title,Options,RootFolder,ppsdf)	\
    ( (This)->lpVtbl -> BrowseForFolder(This,Hwnd,Title,Options,RootFolder,ppsdf) ) 

#define IShellDispatch2_Windows(This,ppid)	\
    ( (This)->lpVtbl -> Windows(This,ppid) ) 

#define IShellDispatch2_Open(This,vDir)	\
    ( (This)->lpVtbl -> Open(This,vDir) ) 

#define IShellDispatch2_Explore(This,vDir)	\
    ( (This)->lpVtbl -> Explore(This,vDir) ) 

#define IShellDispatch2_MinimizeAll(This)	\
    ( (This)->lpVtbl -> MinimizeAll(This) ) 

#define IShellDispatch2_UndoMinimizeALL(This)	\
    ( (This)->lpVtbl -> UndoMinimizeALL(This) ) 

#define IShellDispatch2_FileRun(This)	\
    ( (This)->lpVtbl -> FileRun(This) ) 

#define IShellDispatch2_CascadeWindows(This)	\
    ( (This)->lpVtbl -> CascadeWindows(This) ) 

#define IShellDispatch2_TileVertically(This)	\
    ( (This)->lpVtbl -> TileVertically(This) ) 

#define IShellDispatch2_TileHorizontally(This)	\
    ( (This)->lpVtbl -> TileHorizontally(This) ) 

#define IShellDispatch2_ShutdownWindows(This)	\
    ( (This)->lpVtbl -> ShutdownWindows(This) ) 

#define IShellDispatch2_Suspend(This)	\
    ( (This)->lpVtbl -> Suspend(This) ) 

#define IShellDispatch2_EjectPC(This)	\
    ( (This)->lpVtbl -> EjectPC(This) ) 

#define IShellDispatch2_SetTime(This)	\
    ( (This)->lpVtbl -> SetTime(This) ) 

#define IShellDispatch2_TrayProperties(This)	\
    ( (This)->lpVtbl -> TrayProperties(This) ) 

#define IShellDispatch2_Help(This)	\
    ( (This)->lpVtbl -> Help(This) ) 

#define IShellDispatch2_FindFiles(This)	\
    ( (This)->lpVtbl -> FindFiles(This) ) 

#define IShellDispatch2_FindComputer(This)	\
    ( (This)->lpVtbl -> FindComputer(This) ) 

#define IShellDispatch2_RefreshMenu(This)	\
    ( (This)->lpVtbl -> RefreshMenu(This) ) 

#define IShellDispatch2_ControlPanelItem(This,bstrDir)	\
    ( (This)->lpVtbl -> ControlPanelItem(This,bstrDir) ) 


#define IShellDispatch2_IsRestricted(This,Group,Restriction,plRestrictValue)	\
    ( (This)->lpVtbl -> IsRestricted(This,Group,Restriction,plRestrictValue) ) 

#define IShellDispatch2_ShellExecute(This,File,vArgs,vDir,vOperation,vShow)	\
    ( (This)->lpVtbl -> ShellExecute(This,File,vArgs,vDir,vOperation,vShow) ) 

#define IShellDispatch2_FindPrinter(This,name,location,model)	\
    ( (This)->lpVtbl -> FindPrinter(This,name,location,model) ) 

#define IShellDispatch2_GetSystemInformation(This,name,pv)	\
    ( (This)->lpVtbl -> GetSystemInformation(This,name,pv) ) 

#define IShellDispatch2_ServiceStart(This,ServiceName,Persistent,pSuccess)	\
    ( (This)->lpVtbl -> ServiceStart(This,ServiceName,Persistent,pSuccess) ) 

#define IShellDispatch2_ServiceStop(This,ServiceName,Persistent,pSuccess)	\
    ( (This)->lpVtbl -> ServiceStop(This,ServiceName,Persistent,pSuccess) ) 

#define IShellDispatch2_IsServiceRunning(This,ServiceName,pRunning)	\
    ( (This)->lpVtbl -> IsServiceRunning(This,ServiceName,pRunning) ) 

#define IShellDispatch2_CanStartStopService(This,ServiceName,pCanStartStop)	\
    ( (This)->lpVtbl -> CanStartStopService(This,ServiceName,pCanStartStop) ) 

#define IShellDispatch2_ShowBrowserBar(This,bstrClsid,bShow,pSuccess)	\
    ( (This)->lpVtbl -> ShowBrowserBar(This,bstrClsid,bShow,pSuccess) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IShellDispatch2_INTERFACE_DEFINED__ */


#ifndef __IShellDispatch3_INTERFACE_DEFINED__
#define __IShellDispatch3_INTERFACE_DEFINED__

/* interface IShellDispatch3 */
/* [object][dual][hidden][oleautomation][helpstring][uuid] */ 


EXTERN_C const IID IID_IShellDispatch3;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("177160ca-bb5a-411c-841d-bd38facdeaa0")
    IShellDispatch3 : public IShellDispatch2
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE AddToRecent( 
            /* [in] */ VARIANT varFile,
            /* [optional][in] */ __RPC__in BSTR bstrCategory) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IShellDispatch3Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IShellDispatch3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IShellDispatch3 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IShellDispatch3 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IShellDispatch3 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IShellDispatch3 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IShellDispatch3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IShellDispatch3 * This,
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
        
        DECLSPEC_XFGVIRT(IShellDispatch, get_Application)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Application )( 
            __RPC__in IShellDispatch3 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppid);
        
        DECLSPEC_XFGVIRT(IShellDispatch, get_Parent)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Parent )( 
            __RPC__in IShellDispatch3 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppid);
        
        DECLSPEC_XFGVIRT(IShellDispatch, NameSpace)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *NameSpace )( 
            __RPC__in IShellDispatch3 * This,
            /* [in] */ VARIANT vDir,
            /* [retval][out] */ __RPC__deref_out_opt Folder **ppsdf);
        
        DECLSPEC_XFGVIRT(IShellDispatch, BrowseForFolder)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *BrowseForFolder )( 
            __RPC__in IShellDispatch3 * This,
            /* [in] */ long Hwnd,
            /* [in] */ __RPC__in BSTR Title,
            /* [in] */ long Options,
            /* [optional][in] */ VARIANT RootFolder,
            /* [retval][out] */ __RPC__deref_out_opt Folder **ppsdf);
        
        DECLSPEC_XFGVIRT(IShellDispatch, Windows)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Windows )( 
            __RPC__in IShellDispatch3 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppid);
        
        DECLSPEC_XFGVIRT(IShellDispatch, Open)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Open )( 
            __RPC__in IShellDispatch3 * This,
            /* [in] */ VARIANT vDir);
        
        DECLSPEC_XFGVIRT(IShellDispatch, Explore)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Explore )( 
            __RPC__in IShellDispatch3 * This,
            /* [in] */ VARIANT vDir);
        
        DECLSPEC_XFGVIRT(IShellDispatch, MinimizeAll)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *MinimizeAll )( 
            __RPC__in IShellDispatch3 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, UndoMinimizeALL)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *UndoMinimizeALL )( 
            __RPC__in IShellDispatch3 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, FileRun)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *FileRun )( 
            __RPC__in IShellDispatch3 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, CascadeWindows)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *CascadeWindows )( 
            __RPC__in IShellDispatch3 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, TileVertically)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *TileVertically )( 
            __RPC__in IShellDispatch3 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, TileHorizontally)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *TileHorizontally )( 
            __RPC__in IShellDispatch3 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, ShutdownWindows)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ShutdownWindows )( 
            __RPC__in IShellDispatch3 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, Suspend)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Suspend )( 
            __RPC__in IShellDispatch3 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, EjectPC)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *EjectPC )( 
            __RPC__in IShellDispatch3 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, SetTime)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetTime )( 
            __RPC__in IShellDispatch3 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, TrayProperties)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *TrayProperties )( 
            __RPC__in IShellDispatch3 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, Help)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Help )( 
            __RPC__in IShellDispatch3 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, FindFiles)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *FindFiles )( 
            __RPC__in IShellDispatch3 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, FindComputer)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *FindComputer )( 
            __RPC__in IShellDispatch3 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, RefreshMenu)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *RefreshMenu )( 
            __RPC__in IShellDispatch3 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, ControlPanelItem)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ControlPanelItem )( 
            __RPC__in IShellDispatch3 * This,
            /* [in] */ __RPC__in BSTR bstrDir);
        
        DECLSPEC_XFGVIRT(IShellDispatch2, IsRestricted)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *IsRestricted )( 
            __RPC__in IShellDispatch3 * This,
            /* [in] */ __RPC__in BSTR Group,
            /* [in] */ __RPC__in BSTR Restriction,
            /* [retval][out] */ __RPC__out long *plRestrictValue);
        
        DECLSPEC_XFGVIRT(IShellDispatch2, ShellExecute)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ShellExecute )( 
            __RPC__in IShellDispatch3 * This,
            /* [in] */ __RPC__in BSTR File,
            /* [optional][in] */ VARIANT vArgs,
            /* [optional][in] */ VARIANT vDir,
            /* [optional][in] */ VARIANT vOperation,
            /* [optional][in] */ VARIANT vShow);
        
        DECLSPEC_XFGVIRT(IShellDispatch2, FindPrinter)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *FindPrinter )( 
            __RPC__in IShellDispatch3 * This,
            /* [optional][in] */ __RPC__in BSTR name,
            /* [optional][in] */ __RPC__in BSTR location,
            /* [optional][in] */ __RPC__in BSTR model);
        
        DECLSPEC_XFGVIRT(IShellDispatch2, GetSystemInformation)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetSystemInformation )( 
            __RPC__in IShellDispatch3 * This,
            /* [in] */ __RPC__in BSTR name,
            /* [retval][out] */ __RPC__out VARIANT *pv);
        
        DECLSPEC_XFGVIRT(IShellDispatch2, ServiceStart)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ServiceStart )( 
            __RPC__in IShellDispatch3 * This,
            /* [in] */ __RPC__in BSTR ServiceName,
            /* [in] */ VARIANT Persistent,
            /* [retval][out] */ __RPC__out VARIANT *pSuccess);
        
        DECLSPEC_XFGVIRT(IShellDispatch2, ServiceStop)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ServiceStop )( 
            __RPC__in IShellDispatch3 * This,
            /* [in] */ __RPC__in BSTR ServiceName,
            /* [in] */ VARIANT Persistent,
            /* [retval][out] */ __RPC__out VARIANT *pSuccess);
        
        DECLSPEC_XFGVIRT(IShellDispatch2, IsServiceRunning)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *IsServiceRunning )( 
            __RPC__in IShellDispatch3 * This,
            /* [in] */ __RPC__in BSTR ServiceName,
            /* [retval][out] */ __RPC__out VARIANT *pRunning);
        
        DECLSPEC_XFGVIRT(IShellDispatch2, CanStartStopService)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *CanStartStopService )( 
            __RPC__in IShellDispatch3 * This,
            /* [in] */ __RPC__in BSTR ServiceName,
            /* [retval][out] */ __RPC__out VARIANT *pCanStartStop);
        
        DECLSPEC_XFGVIRT(IShellDispatch2, ShowBrowserBar)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ShowBrowserBar )( 
            __RPC__in IShellDispatch3 * This,
            /* [in] */ __RPC__in BSTR bstrClsid,
            /* [in] */ VARIANT bShow,
            /* [retval][out] */ __RPC__out VARIANT *pSuccess);
        
        DECLSPEC_XFGVIRT(IShellDispatch3, AddToRecent)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *AddToRecent )( 
            __RPC__in IShellDispatch3 * This,
            /* [in] */ VARIANT varFile,
            /* [optional][in] */ __RPC__in BSTR bstrCategory);
        
        END_INTERFACE
    } IShellDispatch3Vtbl;

    interface IShellDispatch3
    {
        CONST_VTBL struct IShellDispatch3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IShellDispatch3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IShellDispatch3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IShellDispatch3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IShellDispatch3_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IShellDispatch3_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IShellDispatch3_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IShellDispatch3_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IShellDispatch3_get_Application(This,ppid)	\
    ( (This)->lpVtbl -> get_Application(This,ppid) ) 

#define IShellDispatch3_get_Parent(This,ppid)	\
    ( (This)->lpVtbl -> get_Parent(This,ppid) ) 

#define IShellDispatch3_NameSpace(This,vDir,ppsdf)	\
    ( (This)->lpVtbl -> NameSpace(This,vDir,ppsdf) ) 

#define IShellDispatch3_BrowseForFolder(This,Hwnd,Title,Options,RootFolder,ppsdf)	\
    ( (This)->lpVtbl -> BrowseForFolder(This,Hwnd,Title,Options,RootFolder,ppsdf) ) 

#define IShellDispatch3_Windows(This,ppid)	\
    ( (This)->lpVtbl -> Windows(This,ppid) ) 

#define IShellDispatch3_Open(This,vDir)	\
    ( (This)->lpVtbl -> Open(This,vDir) ) 

#define IShellDispatch3_Explore(This,vDir)	\
    ( (This)->lpVtbl -> Explore(This,vDir) ) 

#define IShellDispatch3_MinimizeAll(This)	\
    ( (This)->lpVtbl -> MinimizeAll(This) ) 

#define IShellDispatch3_UndoMinimizeALL(This)	\
    ( (This)->lpVtbl -> UndoMinimizeALL(This) ) 

#define IShellDispatch3_FileRun(This)	\
    ( (This)->lpVtbl -> FileRun(This) ) 

#define IShellDispatch3_CascadeWindows(This)	\
    ( (This)->lpVtbl -> CascadeWindows(This) ) 

#define IShellDispatch3_TileVertically(This)	\
    ( (This)->lpVtbl -> TileVertically(This) ) 

#define IShellDispatch3_TileHorizontally(This)	\
    ( (This)->lpVtbl -> TileHorizontally(This) ) 

#define IShellDispatch3_ShutdownWindows(This)	\
    ( (This)->lpVtbl -> ShutdownWindows(This) ) 

#define IShellDispatch3_Suspend(This)	\
    ( (This)->lpVtbl -> Suspend(This) ) 

#define IShellDispatch3_EjectPC(This)	\
    ( (This)->lpVtbl -> EjectPC(This) ) 

#define IShellDispatch3_SetTime(This)	\
    ( (This)->lpVtbl -> SetTime(This) ) 

#define IShellDispatch3_TrayProperties(This)	\
    ( (This)->lpVtbl -> TrayProperties(This) ) 

#define IShellDispatch3_Help(This)	\
    ( (This)->lpVtbl -> Help(This) ) 

#define IShellDispatch3_FindFiles(This)	\
    ( (This)->lpVtbl -> FindFiles(This) ) 

#define IShellDispatch3_FindComputer(This)	\
    ( (This)->lpVtbl -> FindComputer(This) ) 

#define IShellDispatch3_RefreshMenu(This)	\
    ( (This)->lpVtbl -> RefreshMenu(This) ) 

#define IShellDispatch3_ControlPanelItem(This,bstrDir)	\
    ( (This)->lpVtbl -> ControlPanelItem(This,bstrDir) ) 


#define IShellDispatch3_IsRestricted(This,Group,Restriction,plRestrictValue)	\
    ( (This)->lpVtbl -> IsRestricted(This,Group,Restriction,plRestrictValue) ) 

#define IShellDispatch3_ShellExecute(This,File,vArgs,vDir,vOperation,vShow)	\
    ( (This)->lpVtbl -> ShellExecute(This,File,vArgs,vDir,vOperation,vShow) ) 

#define IShellDispatch3_FindPrinter(This,name,location,model)	\
    ( (This)->lpVtbl -> FindPrinter(This,name,location,model) ) 

#define IShellDispatch3_GetSystemInformation(This,name,pv)	\
    ( (This)->lpVtbl -> GetSystemInformation(This,name,pv) ) 

#define IShellDispatch3_ServiceStart(This,ServiceName,Persistent,pSuccess)	\
    ( (This)->lpVtbl -> ServiceStart(This,ServiceName,Persistent,pSuccess) ) 

#define IShellDispatch3_ServiceStop(This,ServiceName,Persistent,pSuccess)	\
    ( (This)->lpVtbl -> ServiceStop(This,ServiceName,Persistent,pSuccess) ) 

#define IShellDispatch3_IsServiceRunning(This,ServiceName,pRunning)	\
    ( (This)->lpVtbl -> IsServiceRunning(This,ServiceName,pRunning) ) 

#define IShellDispatch3_CanStartStopService(This,ServiceName,pCanStartStop)	\
    ( (This)->lpVtbl -> CanStartStopService(This,ServiceName,pCanStartStop) ) 

#define IShellDispatch3_ShowBrowserBar(This,bstrClsid,bShow,pSuccess)	\
    ( (This)->lpVtbl -> ShowBrowserBar(This,bstrClsid,bShow,pSuccess) ) 


#define IShellDispatch3_AddToRecent(This,varFile,bstrCategory)	\
    ( (This)->lpVtbl -> AddToRecent(This,varFile,bstrCategory) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IShellDispatch3_INTERFACE_DEFINED__ */


#ifndef __IShellDispatch4_INTERFACE_DEFINED__
#define __IShellDispatch4_INTERFACE_DEFINED__

/* interface IShellDispatch4 */
/* [object][dual][hidden][oleautomation][helpstring][uuid] */ 


EXTERN_C const IID IID_IShellDispatch4;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("efd84b2d-4bcf-4298-be25-eb542a59fbda")
    IShellDispatch4 : public IShellDispatch3
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE WindowsSecurity( void) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE ToggleDesktop( void) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE ExplorerPolicy( 
            /* [in] */ __RPC__in BSTR bstrPolicyName,
            /* [retval][out] */ __RPC__out VARIANT *pValue) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetSetting( 
            /* [in] */ long lSetting,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pResult) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IShellDispatch4Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IShellDispatch4 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IShellDispatch4 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IShellDispatch4 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IShellDispatch4 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IShellDispatch4 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IShellDispatch4 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IShellDispatch4 * This,
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
        
        DECLSPEC_XFGVIRT(IShellDispatch, get_Application)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Application )( 
            __RPC__in IShellDispatch4 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppid);
        
        DECLSPEC_XFGVIRT(IShellDispatch, get_Parent)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Parent )( 
            __RPC__in IShellDispatch4 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppid);
        
        DECLSPEC_XFGVIRT(IShellDispatch, NameSpace)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *NameSpace )( 
            __RPC__in IShellDispatch4 * This,
            /* [in] */ VARIANT vDir,
            /* [retval][out] */ __RPC__deref_out_opt Folder **ppsdf);
        
        DECLSPEC_XFGVIRT(IShellDispatch, BrowseForFolder)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *BrowseForFolder )( 
            __RPC__in IShellDispatch4 * This,
            /* [in] */ long Hwnd,
            /* [in] */ __RPC__in BSTR Title,
            /* [in] */ long Options,
            /* [optional][in] */ VARIANT RootFolder,
            /* [retval][out] */ __RPC__deref_out_opt Folder **ppsdf);
        
        DECLSPEC_XFGVIRT(IShellDispatch, Windows)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Windows )( 
            __RPC__in IShellDispatch4 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppid);
        
        DECLSPEC_XFGVIRT(IShellDispatch, Open)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Open )( 
            __RPC__in IShellDispatch4 * This,
            /* [in] */ VARIANT vDir);
        
        DECLSPEC_XFGVIRT(IShellDispatch, Explore)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Explore )( 
            __RPC__in IShellDispatch4 * This,
            /* [in] */ VARIANT vDir);
        
        DECLSPEC_XFGVIRT(IShellDispatch, MinimizeAll)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *MinimizeAll )( 
            __RPC__in IShellDispatch4 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, UndoMinimizeALL)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *UndoMinimizeALL )( 
            __RPC__in IShellDispatch4 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, FileRun)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *FileRun )( 
            __RPC__in IShellDispatch4 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, CascadeWindows)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *CascadeWindows )( 
            __RPC__in IShellDispatch4 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, TileVertically)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *TileVertically )( 
            __RPC__in IShellDispatch4 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, TileHorizontally)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *TileHorizontally )( 
            __RPC__in IShellDispatch4 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, ShutdownWindows)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ShutdownWindows )( 
            __RPC__in IShellDispatch4 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, Suspend)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Suspend )( 
            __RPC__in IShellDispatch4 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, EjectPC)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *EjectPC )( 
            __RPC__in IShellDispatch4 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, SetTime)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetTime )( 
            __RPC__in IShellDispatch4 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, TrayProperties)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *TrayProperties )( 
            __RPC__in IShellDispatch4 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, Help)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Help )( 
            __RPC__in IShellDispatch4 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, FindFiles)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *FindFiles )( 
            __RPC__in IShellDispatch4 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, FindComputer)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *FindComputer )( 
            __RPC__in IShellDispatch4 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, RefreshMenu)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *RefreshMenu )( 
            __RPC__in IShellDispatch4 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, ControlPanelItem)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ControlPanelItem )( 
            __RPC__in IShellDispatch4 * This,
            /* [in] */ __RPC__in BSTR bstrDir);
        
        DECLSPEC_XFGVIRT(IShellDispatch2, IsRestricted)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *IsRestricted )( 
            __RPC__in IShellDispatch4 * This,
            /* [in] */ __RPC__in BSTR Group,
            /* [in] */ __RPC__in BSTR Restriction,
            /* [retval][out] */ __RPC__out long *plRestrictValue);
        
        DECLSPEC_XFGVIRT(IShellDispatch2, ShellExecute)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ShellExecute )( 
            __RPC__in IShellDispatch4 * This,
            /* [in] */ __RPC__in BSTR File,
            /* [optional][in] */ VARIANT vArgs,
            /* [optional][in] */ VARIANT vDir,
            /* [optional][in] */ VARIANT vOperation,
            /* [optional][in] */ VARIANT vShow);
        
        DECLSPEC_XFGVIRT(IShellDispatch2, FindPrinter)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *FindPrinter )( 
            __RPC__in IShellDispatch4 * This,
            /* [optional][in] */ __RPC__in BSTR name,
            /* [optional][in] */ __RPC__in BSTR location,
            /* [optional][in] */ __RPC__in BSTR model);
        
        DECLSPEC_XFGVIRT(IShellDispatch2, GetSystemInformation)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetSystemInformation )( 
            __RPC__in IShellDispatch4 * This,
            /* [in] */ __RPC__in BSTR name,
            /* [retval][out] */ __RPC__out VARIANT *pv);
        
        DECLSPEC_XFGVIRT(IShellDispatch2, ServiceStart)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ServiceStart )( 
            __RPC__in IShellDispatch4 * This,
            /* [in] */ __RPC__in BSTR ServiceName,
            /* [in] */ VARIANT Persistent,
            /* [retval][out] */ __RPC__out VARIANT *pSuccess);
        
        DECLSPEC_XFGVIRT(IShellDispatch2, ServiceStop)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ServiceStop )( 
            __RPC__in IShellDispatch4 * This,
            /* [in] */ __RPC__in BSTR ServiceName,
            /* [in] */ VARIANT Persistent,
            /* [retval][out] */ __RPC__out VARIANT *pSuccess);
        
        DECLSPEC_XFGVIRT(IShellDispatch2, IsServiceRunning)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *IsServiceRunning )( 
            __RPC__in IShellDispatch4 * This,
            /* [in] */ __RPC__in BSTR ServiceName,
            /* [retval][out] */ __RPC__out VARIANT *pRunning);
        
        DECLSPEC_XFGVIRT(IShellDispatch2, CanStartStopService)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *CanStartStopService )( 
            __RPC__in IShellDispatch4 * This,
            /* [in] */ __RPC__in BSTR ServiceName,
            /* [retval][out] */ __RPC__out VARIANT *pCanStartStop);
        
        DECLSPEC_XFGVIRT(IShellDispatch2, ShowBrowserBar)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ShowBrowserBar )( 
            __RPC__in IShellDispatch4 * This,
            /* [in] */ __RPC__in BSTR bstrClsid,
            /* [in] */ VARIANT bShow,
            /* [retval][out] */ __RPC__out VARIANT *pSuccess);
        
        DECLSPEC_XFGVIRT(IShellDispatch3, AddToRecent)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *AddToRecent )( 
            __RPC__in IShellDispatch4 * This,
            /* [in] */ VARIANT varFile,
            /* [optional][in] */ __RPC__in BSTR bstrCategory);
        
        DECLSPEC_XFGVIRT(IShellDispatch4, WindowsSecurity)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *WindowsSecurity )( 
            __RPC__in IShellDispatch4 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch4, ToggleDesktop)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ToggleDesktop )( 
            __RPC__in IShellDispatch4 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch4, ExplorerPolicy)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ExplorerPolicy )( 
            __RPC__in IShellDispatch4 * This,
            /* [in] */ __RPC__in BSTR bstrPolicyName,
            /* [retval][out] */ __RPC__out VARIANT *pValue);
        
        DECLSPEC_XFGVIRT(IShellDispatch4, GetSetting)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetSetting )( 
            __RPC__in IShellDispatch4 * This,
            /* [in] */ long lSetting,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pResult);
        
        END_INTERFACE
    } IShellDispatch4Vtbl;

    interface IShellDispatch4
    {
        CONST_VTBL struct IShellDispatch4Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IShellDispatch4_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IShellDispatch4_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IShellDispatch4_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IShellDispatch4_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IShellDispatch4_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IShellDispatch4_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IShellDispatch4_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IShellDispatch4_get_Application(This,ppid)	\
    ( (This)->lpVtbl -> get_Application(This,ppid) ) 

#define IShellDispatch4_get_Parent(This,ppid)	\
    ( (This)->lpVtbl -> get_Parent(This,ppid) ) 

#define IShellDispatch4_NameSpace(This,vDir,ppsdf)	\
    ( (This)->lpVtbl -> NameSpace(This,vDir,ppsdf) ) 

#define IShellDispatch4_BrowseForFolder(This,Hwnd,Title,Options,RootFolder,ppsdf)	\
    ( (This)->lpVtbl -> BrowseForFolder(This,Hwnd,Title,Options,RootFolder,ppsdf) ) 

#define IShellDispatch4_Windows(This,ppid)	\
    ( (This)->lpVtbl -> Windows(This,ppid) ) 

#define IShellDispatch4_Open(This,vDir)	\
    ( (This)->lpVtbl -> Open(This,vDir) ) 

#define IShellDispatch4_Explore(This,vDir)	\
    ( (This)->lpVtbl -> Explore(This,vDir) ) 

#define IShellDispatch4_MinimizeAll(This)	\
    ( (This)->lpVtbl -> MinimizeAll(This) ) 

#define IShellDispatch4_UndoMinimizeALL(This)	\
    ( (This)->lpVtbl -> UndoMinimizeALL(This) ) 

#define IShellDispatch4_FileRun(This)	\
    ( (This)->lpVtbl -> FileRun(This) ) 

#define IShellDispatch4_CascadeWindows(This)	\
    ( (This)->lpVtbl -> CascadeWindows(This) ) 

#define IShellDispatch4_TileVertically(This)	\
    ( (This)->lpVtbl -> TileVertically(This) ) 

#define IShellDispatch4_TileHorizontally(This)	\
    ( (This)->lpVtbl -> TileHorizontally(This) ) 

#define IShellDispatch4_ShutdownWindows(This)	\
    ( (This)->lpVtbl -> ShutdownWindows(This) ) 

#define IShellDispatch4_Suspend(This)	\
    ( (This)->lpVtbl -> Suspend(This) ) 

#define IShellDispatch4_EjectPC(This)	\
    ( (This)->lpVtbl -> EjectPC(This) ) 

#define IShellDispatch4_SetTime(This)	\
    ( (This)->lpVtbl -> SetTime(This) ) 

#define IShellDispatch4_TrayProperties(This)	\
    ( (This)->lpVtbl -> TrayProperties(This) ) 

#define IShellDispatch4_Help(This)	\
    ( (This)->lpVtbl -> Help(This) ) 

#define IShellDispatch4_FindFiles(This)	\
    ( (This)->lpVtbl -> FindFiles(This) ) 

#define IShellDispatch4_FindComputer(This)	\
    ( (This)->lpVtbl -> FindComputer(This) ) 

#define IShellDispatch4_RefreshMenu(This)	\
    ( (This)->lpVtbl -> RefreshMenu(This) ) 

#define IShellDispatch4_ControlPanelItem(This,bstrDir)	\
    ( (This)->lpVtbl -> ControlPanelItem(This,bstrDir) ) 


#define IShellDispatch4_IsRestricted(This,Group,Restriction,plRestrictValue)	\
    ( (This)->lpVtbl -> IsRestricted(This,Group,Restriction,plRestrictValue) ) 

#define IShellDispatch4_ShellExecute(This,File,vArgs,vDir,vOperation,vShow)	\
    ( (This)->lpVtbl -> ShellExecute(This,File,vArgs,vDir,vOperation,vShow) ) 

#define IShellDispatch4_FindPrinter(This,name,location,model)	\
    ( (This)->lpVtbl -> FindPrinter(This,name,location,model) ) 

#define IShellDispatch4_GetSystemInformation(This,name,pv)	\
    ( (This)->lpVtbl -> GetSystemInformation(This,name,pv) ) 

#define IShellDispatch4_ServiceStart(This,ServiceName,Persistent,pSuccess)	\
    ( (This)->lpVtbl -> ServiceStart(This,ServiceName,Persistent,pSuccess) ) 

#define IShellDispatch4_ServiceStop(This,ServiceName,Persistent,pSuccess)	\
    ( (This)->lpVtbl -> ServiceStop(This,ServiceName,Persistent,pSuccess) ) 

#define IShellDispatch4_IsServiceRunning(This,ServiceName,pRunning)	\
    ( (This)->lpVtbl -> IsServiceRunning(This,ServiceName,pRunning) ) 

#define IShellDispatch4_CanStartStopService(This,ServiceName,pCanStartStop)	\
    ( (This)->lpVtbl -> CanStartStopService(This,ServiceName,pCanStartStop) ) 

#define IShellDispatch4_ShowBrowserBar(This,bstrClsid,bShow,pSuccess)	\
    ( (This)->lpVtbl -> ShowBrowserBar(This,bstrClsid,bShow,pSuccess) ) 


#define IShellDispatch4_AddToRecent(This,varFile,bstrCategory)	\
    ( (This)->lpVtbl -> AddToRecent(This,varFile,bstrCategory) ) 


#define IShellDispatch4_WindowsSecurity(This)	\
    ( (This)->lpVtbl -> WindowsSecurity(This) ) 

#define IShellDispatch4_ToggleDesktop(This)	\
    ( (This)->lpVtbl -> ToggleDesktop(This) ) 

#define IShellDispatch4_ExplorerPolicy(This,bstrPolicyName,pValue)	\
    ( (This)->lpVtbl -> ExplorerPolicy(This,bstrPolicyName,pValue) ) 

#define IShellDispatch4_GetSetting(This,lSetting,pResult)	\
    ( (This)->lpVtbl -> GetSetting(This,lSetting,pResult) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IShellDispatch4_INTERFACE_DEFINED__ */


#ifndef __IShellDispatch5_INTERFACE_DEFINED__
#define __IShellDispatch5_INTERFACE_DEFINED__

/* interface IShellDispatch5 */
/* [object][dual][hidden][oleautomation][helpstring][uuid] */ 


EXTERN_C const IID IID_IShellDispatch5;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("866738b9-6cf2-4de8-8767-f794ebe74f4e")
    IShellDispatch5 : public IShellDispatch4
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE WindowSwitcher( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IShellDispatch5Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IShellDispatch5 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IShellDispatch5 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IShellDispatch5 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IShellDispatch5 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IShellDispatch5 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IShellDispatch5 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IShellDispatch5 * This,
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
        
        DECLSPEC_XFGVIRT(IShellDispatch, get_Application)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Application )( 
            __RPC__in IShellDispatch5 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppid);
        
        DECLSPEC_XFGVIRT(IShellDispatch, get_Parent)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Parent )( 
            __RPC__in IShellDispatch5 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppid);
        
        DECLSPEC_XFGVIRT(IShellDispatch, NameSpace)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *NameSpace )( 
            __RPC__in IShellDispatch5 * This,
            /* [in] */ VARIANT vDir,
            /* [retval][out] */ __RPC__deref_out_opt Folder **ppsdf);
        
        DECLSPEC_XFGVIRT(IShellDispatch, BrowseForFolder)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *BrowseForFolder )( 
            __RPC__in IShellDispatch5 * This,
            /* [in] */ long Hwnd,
            /* [in] */ __RPC__in BSTR Title,
            /* [in] */ long Options,
            /* [optional][in] */ VARIANT RootFolder,
            /* [retval][out] */ __RPC__deref_out_opt Folder **ppsdf);
        
        DECLSPEC_XFGVIRT(IShellDispatch, Windows)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Windows )( 
            __RPC__in IShellDispatch5 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppid);
        
        DECLSPEC_XFGVIRT(IShellDispatch, Open)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Open )( 
            __RPC__in IShellDispatch5 * This,
            /* [in] */ VARIANT vDir);
        
        DECLSPEC_XFGVIRT(IShellDispatch, Explore)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Explore )( 
            __RPC__in IShellDispatch5 * This,
            /* [in] */ VARIANT vDir);
        
        DECLSPEC_XFGVIRT(IShellDispatch, MinimizeAll)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *MinimizeAll )( 
            __RPC__in IShellDispatch5 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, UndoMinimizeALL)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *UndoMinimizeALL )( 
            __RPC__in IShellDispatch5 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, FileRun)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *FileRun )( 
            __RPC__in IShellDispatch5 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, CascadeWindows)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *CascadeWindows )( 
            __RPC__in IShellDispatch5 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, TileVertically)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *TileVertically )( 
            __RPC__in IShellDispatch5 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, TileHorizontally)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *TileHorizontally )( 
            __RPC__in IShellDispatch5 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, ShutdownWindows)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ShutdownWindows )( 
            __RPC__in IShellDispatch5 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, Suspend)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Suspend )( 
            __RPC__in IShellDispatch5 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, EjectPC)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *EjectPC )( 
            __RPC__in IShellDispatch5 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, SetTime)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetTime )( 
            __RPC__in IShellDispatch5 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, TrayProperties)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *TrayProperties )( 
            __RPC__in IShellDispatch5 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, Help)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Help )( 
            __RPC__in IShellDispatch5 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, FindFiles)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *FindFiles )( 
            __RPC__in IShellDispatch5 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, FindComputer)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *FindComputer )( 
            __RPC__in IShellDispatch5 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, RefreshMenu)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *RefreshMenu )( 
            __RPC__in IShellDispatch5 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, ControlPanelItem)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ControlPanelItem )( 
            __RPC__in IShellDispatch5 * This,
            /* [in] */ __RPC__in BSTR bstrDir);
        
        DECLSPEC_XFGVIRT(IShellDispatch2, IsRestricted)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *IsRestricted )( 
            __RPC__in IShellDispatch5 * This,
            /* [in] */ __RPC__in BSTR Group,
            /* [in] */ __RPC__in BSTR Restriction,
            /* [retval][out] */ __RPC__out long *plRestrictValue);
        
        DECLSPEC_XFGVIRT(IShellDispatch2, ShellExecute)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ShellExecute )( 
            __RPC__in IShellDispatch5 * This,
            /* [in] */ __RPC__in BSTR File,
            /* [optional][in] */ VARIANT vArgs,
            /* [optional][in] */ VARIANT vDir,
            /* [optional][in] */ VARIANT vOperation,
            /* [optional][in] */ VARIANT vShow);
        
        DECLSPEC_XFGVIRT(IShellDispatch2, FindPrinter)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *FindPrinter )( 
            __RPC__in IShellDispatch5 * This,
            /* [optional][in] */ __RPC__in BSTR name,
            /* [optional][in] */ __RPC__in BSTR location,
            /* [optional][in] */ __RPC__in BSTR model);
        
        DECLSPEC_XFGVIRT(IShellDispatch2, GetSystemInformation)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetSystemInformation )( 
            __RPC__in IShellDispatch5 * This,
            /* [in] */ __RPC__in BSTR name,
            /* [retval][out] */ __RPC__out VARIANT *pv);
        
        DECLSPEC_XFGVIRT(IShellDispatch2, ServiceStart)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ServiceStart )( 
            __RPC__in IShellDispatch5 * This,
            /* [in] */ __RPC__in BSTR ServiceName,
            /* [in] */ VARIANT Persistent,
            /* [retval][out] */ __RPC__out VARIANT *pSuccess);
        
        DECLSPEC_XFGVIRT(IShellDispatch2, ServiceStop)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ServiceStop )( 
            __RPC__in IShellDispatch5 * This,
            /* [in] */ __RPC__in BSTR ServiceName,
            /* [in] */ VARIANT Persistent,
            /* [retval][out] */ __RPC__out VARIANT *pSuccess);
        
        DECLSPEC_XFGVIRT(IShellDispatch2, IsServiceRunning)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *IsServiceRunning )( 
            __RPC__in IShellDispatch5 * This,
            /* [in] */ __RPC__in BSTR ServiceName,
            /* [retval][out] */ __RPC__out VARIANT *pRunning);
        
        DECLSPEC_XFGVIRT(IShellDispatch2, CanStartStopService)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *CanStartStopService )( 
            __RPC__in IShellDispatch5 * This,
            /* [in] */ __RPC__in BSTR ServiceName,
            /* [retval][out] */ __RPC__out VARIANT *pCanStartStop);
        
        DECLSPEC_XFGVIRT(IShellDispatch2, ShowBrowserBar)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ShowBrowserBar )( 
            __RPC__in IShellDispatch5 * This,
            /* [in] */ __RPC__in BSTR bstrClsid,
            /* [in] */ VARIANT bShow,
            /* [retval][out] */ __RPC__out VARIANT *pSuccess);
        
        DECLSPEC_XFGVIRT(IShellDispatch3, AddToRecent)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *AddToRecent )( 
            __RPC__in IShellDispatch5 * This,
            /* [in] */ VARIANT varFile,
            /* [optional][in] */ __RPC__in BSTR bstrCategory);
        
        DECLSPEC_XFGVIRT(IShellDispatch4, WindowsSecurity)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *WindowsSecurity )( 
            __RPC__in IShellDispatch5 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch4, ToggleDesktop)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ToggleDesktop )( 
            __RPC__in IShellDispatch5 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch4, ExplorerPolicy)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ExplorerPolicy )( 
            __RPC__in IShellDispatch5 * This,
            /* [in] */ __RPC__in BSTR bstrPolicyName,
            /* [retval][out] */ __RPC__out VARIANT *pValue);
        
        DECLSPEC_XFGVIRT(IShellDispatch4, GetSetting)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetSetting )( 
            __RPC__in IShellDispatch5 * This,
            /* [in] */ long lSetting,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pResult);
        
        DECLSPEC_XFGVIRT(IShellDispatch5, WindowSwitcher)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *WindowSwitcher )( 
            __RPC__in IShellDispatch5 * This);
        
        END_INTERFACE
    } IShellDispatch5Vtbl;

    interface IShellDispatch5
    {
        CONST_VTBL struct IShellDispatch5Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IShellDispatch5_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IShellDispatch5_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IShellDispatch5_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IShellDispatch5_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IShellDispatch5_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IShellDispatch5_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IShellDispatch5_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IShellDispatch5_get_Application(This,ppid)	\
    ( (This)->lpVtbl -> get_Application(This,ppid) ) 

#define IShellDispatch5_get_Parent(This,ppid)	\
    ( (This)->lpVtbl -> get_Parent(This,ppid) ) 

#define IShellDispatch5_NameSpace(This,vDir,ppsdf)	\
    ( (This)->lpVtbl -> NameSpace(This,vDir,ppsdf) ) 

#define IShellDispatch5_BrowseForFolder(This,Hwnd,Title,Options,RootFolder,ppsdf)	\
    ( (This)->lpVtbl -> BrowseForFolder(This,Hwnd,Title,Options,RootFolder,ppsdf) ) 

#define IShellDispatch5_Windows(This,ppid)	\
    ( (This)->lpVtbl -> Windows(This,ppid) ) 

#define IShellDispatch5_Open(This,vDir)	\
    ( (This)->lpVtbl -> Open(This,vDir) ) 

#define IShellDispatch5_Explore(This,vDir)	\
    ( (This)->lpVtbl -> Explore(This,vDir) ) 

#define IShellDispatch5_MinimizeAll(This)	\
    ( (This)->lpVtbl -> MinimizeAll(This) ) 

#define IShellDispatch5_UndoMinimizeALL(This)	\
    ( (This)->lpVtbl -> UndoMinimizeALL(This) ) 

#define IShellDispatch5_FileRun(This)	\
    ( (This)->lpVtbl -> FileRun(This) ) 

#define IShellDispatch5_CascadeWindows(This)	\
    ( (This)->lpVtbl -> CascadeWindows(This) ) 

#define IShellDispatch5_TileVertically(This)	\
    ( (This)->lpVtbl -> TileVertically(This) ) 

#define IShellDispatch5_TileHorizontally(This)	\
    ( (This)->lpVtbl -> TileHorizontally(This) ) 

#define IShellDispatch5_ShutdownWindows(This)	\
    ( (This)->lpVtbl -> ShutdownWindows(This) ) 

#define IShellDispatch5_Suspend(This)	\
    ( (This)->lpVtbl -> Suspend(This) ) 

#define IShellDispatch5_EjectPC(This)	\
    ( (This)->lpVtbl -> EjectPC(This) ) 

#define IShellDispatch5_SetTime(This)	\
    ( (This)->lpVtbl -> SetTime(This) ) 

#define IShellDispatch5_TrayProperties(This)	\
    ( (This)->lpVtbl -> TrayProperties(This) ) 

#define IShellDispatch5_Help(This)	\
    ( (This)->lpVtbl -> Help(This) ) 

#define IShellDispatch5_FindFiles(This)	\
    ( (This)->lpVtbl -> FindFiles(This) ) 

#define IShellDispatch5_FindComputer(This)	\
    ( (This)->lpVtbl -> FindComputer(This) ) 

#define IShellDispatch5_RefreshMenu(This)	\
    ( (This)->lpVtbl -> RefreshMenu(This) ) 

#define IShellDispatch5_ControlPanelItem(This,bstrDir)	\
    ( (This)->lpVtbl -> ControlPanelItem(This,bstrDir) ) 


#define IShellDispatch5_IsRestricted(This,Group,Restriction,plRestrictValue)	\
    ( (This)->lpVtbl -> IsRestricted(This,Group,Restriction,plRestrictValue) ) 

#define IShellDispatch5_ShellExecute(This,File,vArgs,vDir,vOperation,vShow)	\
    ( (This)->lpVtbl -> ShellExecute(This,File,vArgs,vDir,vOperation,vShow) ) 

#define IShellDispatch5_FindPrinter(This,name,location,model)	\
    ( (This)->lpVtbl -> FindPrinter(This,name,location,model) ) 

#define IShellDispatch5_GetSystemInformation(This,name,pv)	\
    ( (This)->lpVtbl -> GetSystemInformation(This,name,pv) ) 

#define IShellDispatch5_ServiceStart(This,ServiceName,Persistent,pSuccess)	\
    ( (This)->lpVtbl -> ServiceStart(This,ServiceName,Persistent,pSuccess) ) 

#define IShellDispatch5_ServiceStop(This,ServiceName,Persistent,pSuccess)	\
    ( (This)->lpVtbl -> ServiceStop(This,ServiceName,Persistent,pSuccess) ) 

#define IShellDispatch5_IsServiceRunning(This,ServiceName,pRunning)	\
    ( (This)->lpVtbl -> IsServiceRunning(This,ServiceName,pRunning) ) 

#define IShellDispatch5_CanStartStopService(This,ServiceName,pCanStartStop)	\
    ( (This)->lpVtbl -> CanStartStopService(This,ServiceName,pCanStartStop) ) 

#define IShellDispatch5_ShowBrowserBar(This,bstrClsid,bShow,pSuccess)	\
    ( (This)->lpVtbl -> ShowBrowserBar(This,bstrClsid,bShow,pSuccess) ) 


#define IShellDispatch5_AddToRecent(This,varFile,bstrCategory)	\
    ( (This)->lpVtbl -> AddToRecent(This,varFile,bstrCategory) ) 


#define IShellDispatch5_WindowsSecurity(This)	\
    ( (This)->lpVtbl -> WindowsSecurity(This) ) 

#define IShellDispatch5_ToggleDesktop(This)	\
    ( (This)->lpVtbl -> ToggleDesktop(This) ) 

#define IShellDispatch5_ExplorerPolicy(This,bstrPolicyName,pValue)	\
    ( (This)->lpVtbl -> ExplorerPolicy(This,bstrPolicyName,pValue) ) 

#define IShellDispatch5_GetSetting(This,lSetting,pResult)	\
    ( (This)->lpVtbl -> GetSetting(This,lSetting,pResult) ) 


#define IShellDispatch5_WindowSwitcher(This)	\
    ( (This)->lpVtbl -> WindowSwitcher(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IShellDispatch5_INTERFACE_DEFINED__ */


#ifndef __IShellDispatch6_INTERFACE_DEFINED__
#define __IShellDispatch6_INTERFACE_DEFINED__

/* interface IShellDispatch6 */
/* [object][dual][hidden][oleautomation][helpstring][uuid] */ 


EXTERN_C const IID IID_IShellDispatch6;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("286e6f1b-7113-4355-9562-96b7e9d64c54")
    IShellDispatch6 : public IShellDispatch5
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SearchCommand( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IShellDispatch6Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IShellDispatch6 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IShellDispatch6 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IShellDispatch6 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IShellDispatch6 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IShellDispatch6 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IShellDispatch6 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IShellDispatch6 * This,
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
        
        DECLSPEC_XFGVIRT(IShellDispatch, get_Application)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Application )( 
            __RPC__in IShellDispatch6 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppid);
        
        DECLSPEC_XFGVIRT(IShellDispatch, get_Parent)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Parent )( 
            __RPC__in IShellDispatch6 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppid);
        
        DECLSPEC_XFGVIRT(IShellDispatch, NameSpace)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *NameSpace )( 
            __RPC__in IShellDispatch6 * This,
            /* [in] */ VARIANT vDir,
            /* [retval][out] */ __RPC__deref_out_opt Folder **ppsdf);
        
        DECLSPEC_XFGVIRT(IShellDispatch, BrowseForFolder)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *BrowseForFolder )( 
            __RPC__in IShellDispatch6 * This,
            /* [in] */ long Hwnd,
            /* [in] */ __RPC__in BSTR Title,
            /* [in] */ long Options,
            /* [optional][in] */ VARIANT RootFolder,
            /* [retval][out] */ __RPC__deref_out_opt Folder **ppsdf);
        
        DECLSPEC_XFGVIRT(IShellDispatch, Windows)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Windows )( 
            __RPC__in IShellDispatch6 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppid);
        
        DECLSPEC_XFGVIRT(IShellDispatch, Open)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Open )( 
            __RPC__in IShellDispatch6 * This,
            /* [in] */ VARIANT vDir);
        
        DECLSPEC_XFGVIRT(IShellDispatch, Explore)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Explore )( 
            __RPC__in IShellDispatch6 * This,
            /* [in] */ VARIANT vDir);
        
        DECLSPEC_XFGVIRT(IShellDispatch, MinimizeAll)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *MinimizeAll )( 
            __RPC__in IShellDispatch6 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, UndoMinimizeALL)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *UndoMinimizeALL )( 
            __RPC__in IShellDispatch6 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, FileRun)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *FileRun )( 
            __RPC__in IShellDispatch6 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, CascadeWindows)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *CascadeWindows )( 
            __RPC__in IShellDispatch6 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, TileVertically)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *TileVertically )( 
            __RPC__in IShellDispatch6 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, TileHorizontally)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *TileHorizontally )( 
            __RPC__in IShellDispatch6 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, ShutdownWindows)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ShutdownWindows )( 
            __RPC__in IShellDispatch6 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, Suspend)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Suspend )( 
            __RPC__in IShellDispatch6 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, EjectPC)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *EjectPC )( 
            __RPC__in IShellDispatch6 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, SetTime)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetTime )( 
            __RPC__in IShellDispatch6 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, TrayProperties)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *TrayProperties )( 
            __RPC__in IShellDispatch6 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, Help)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Help )( 
            __RPC__in IShellDispatch6 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, FindFiles)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *FindFiles )( 
            __RPC__in IShellDispatch6 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, FindComputer)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *FindComputer )( 
            __RPC__in IShellDispatch6 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, RefreshMenu)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *RefreshMenu )( 
            __RPC__in IShellDispatch6 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch, ControlPanelItem)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ControlPanelItem )( 
            __RPC__in IShellDispatch6 * This,
            /* [in] */ __RPC__in BSTR bstrDir);
        
        DECLSPEC_XFGVIRT(IShellDispatch2, IsRestricted)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *IsRestricted )( 
            __RPC__in IShellDispatch6 * This,
            /* [in] */ __RPC__in BSTR Group,
            /* [in] */ __RPC__in BSTR Restriction,
            /* [retval][out] */ __RPC__out long *plRestrictValue);
        
        DECLSPEC_XFGVIRT(IShellDispatch2, ShellExecute)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ShellExecute )( 
            __RPC__in IShellDispatch6 * This,
            /* [in] */ __RPC__in BSTR File,
            /* [optional][in] */ VARIANT vArgs,
            /* [optional][in] */ VARIANT vDir,
            /* [optional][in] */ VARIANT vOperation,
            /* [optional][in] */ VARIANT vShow);
        
        DECLSPEC_XFGVIRT(IShellDispatch2, FindPrinter)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *FindPrinter )( 
            __RPC__in IShellDispatch6 * This,
            /* [optional][in] */ __RPC__in BSTR name,
            /* [optional][in] */ __RPC__in BSTR location,
            /* [optional][in] */ __RPC__in BSTR model);
        
        DECLSPEC_XFGVIRT(IShellDispatch2, GetSystemInformation)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetSystemInformation )( 
            __RPC__in IShellDispatch6 * This,
            /* [in] */ __RPC__in BSTR name,
            /* [retval][out] */ __RPC__out VARIANT *pv);
        
        DECLSPEC_XFGVIRT(IShellDispatch2, ServiceStart)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ServiceStart )( 
            __RPC__in IShellDispatch6 * This,
            /* [in] */ __RPC__in BSTR ServiceName,
            /* [in] */ VARIANT Persistent,
            /* [retval][out] */ __RPC__out VARIANT *pSuccess);
        
        DECLSPEC_XFGVIRT(IShellDispatch2, ServiceStop)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ServiceStop )( 
            __RPC__in IShellDispatch6 * This,
            /* [in] */ __RPC__in BSTR ServiceName,
            /* [in] */ VARIANT Persistent,
            /* [retval][out] */ __RPC__out VARIANT *pSuccess);
        
        DECLSPEC_XFGVIRT(IShellDispatch2, IsServiceRunning)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *IsServiceRunning )( 
            __RPC__in IShellDispatch6 * This,
            /* [in] */ __RPC__in BSTR ServiceName,
            /* [retval][out] */ __RPC__out VARIANT *pRunning);
        
        DECLSPEC_XFGVIRT(IShellDispatch2, CanStartStopService)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *CanStartStopService )( 
            __RPC__in IShellDispatch6 * This,
            /* [in] */ __RPC__in BSTR ServiceName,
            /* [retval][out] */ __RPC__out VARIANT *pCanStartStop);
        
        DECLSPEC_XFGVIRT(IShellDispatch2, ShowBrowserBar)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ShowBrowserBar )( 
            __RPC__in IShellDispatch6 * This,
            /* [in] */ __RPC__in BSTR bstrClsid,
            /* [in] */ VARIANT bShow,
            /* [retval][out] */ __RPC__out VARIANT *pSuccess);
        
        DECLSPEC_XFGVIRT(IShellDispatch3, AddToRecent)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *AddToRecent )( 
            __RPC__in IShellDispatch6 * This,
            /* [in] */ VARIANT varFile,
            /* [optional][in] */ __RPC__in BSTR bstrCategory);
        
        DECLSPEC_XFGVIRT(IShellDispatch4, WindowsSecurity)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *WindowsSecurity )( 
            __RPC__in IShellDispatch6 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch4, ToggleDesktop)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ToggleDesktop )( 
            __RPC__in IShellDispatch6 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch4, ExplorerPolicy)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ExplorerPolicy )( 
            __RPC__in IShellDispatch6 * This,
            /* [in] */ __RPC__in BSTR bstrPolicyName,
            /* [retval][out] */ __RPC__out VARIANT *pValue);
        
        DECLSPEC_XFGVIRT(IShellDispatch4, GetSetting)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetSetting )( 
            __RPC__in IShellDispatch6 * This,
            /* [in] */ long lSetting,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pResult);
        
        DECLSPEC_XFGVIRT(IShellDispatch5, WindowSwitcher)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *WindowSwitcher )( 
            __RPC__in IShellDispatch6 * This);
        
        DECLSPEC_XFGVIRT(IShellDispatch6, SearchCommand)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SearchCommand )( 
            __RPC__in IShellDispatch6 * This);
        
        END_INTERFACE
    } IShellDispatch6Vtbl;

    interface IShellDispatch6
    {
        CONST_VTBL struct IShellDispatch6Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IShellDispatch6_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IShellDispatch6_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IShellDispatch6_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IShellDispatch6_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IShellDispatch6_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IShellDispatch6_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IShellDispatch6_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IShellDispatch6_get_Application(This,ppid)	\
    ( (This)->lpVtbl -> get_Application(This,ppid) ) 

#define IShellDispatch6_get_Parent(This,ppid)	\
    ( (This)->lpVtbl -> get_Parent(This,ppid) ) 

#define IShellDispatch6_NameSpace(This,vDir,ppsdf)	\
    ( (This)->lpVtbl -> NameSpace(This,vDir,ppsdf) ) 

#define IShellDispatch6_BrowseForFolder(This,Hwnd,Title,Options,RootFolder,ppsdf)	\
    ( (This)->lpVtbl -> BrowseForFolder(This,Hwnd,Title,Options,RootFolder,ppsdf) ) 

#define IShellDispatch6_Windows(This,ppid)	\
    ( (This)->lpVtbl -> Windows(This,ppid) ) 

#define IShellDispatch6_Open(This,vDir)	\
    ( (This)->lpVtbl -> Open(This,vDir) ) 

#define IShellDispatch6_Explore(This,vDir)	\
    ( (This)->lpVtbl -> Explore(This,vDir) ) 

#define IShellDispatch6_MinimizeAll(This)	\
    ( (This)->lpVtbl -> MinimizeAll(This) ) 

#define IShellDispatch6_UndoMinimizeALL(This)	\
    ( (This)->lpVtbl -> UndoMinimizeALL(This) ) 

#define IShellDispatch6_FileRun(This)	\
    ( (This)->lpVtbl -> FileRun(This) ) 

#define IShellDispatch6_CascadeWindows(This)	\
    ( (This)->lpVtbl -> CascadeWindows(This) ) 

#define IShellDispatch6_TileVertically(This)	\
    ( (This)->lpVtbl -> TileVertically(This) ) 

#define IShellDispatch6_TileHorizontally(This)	\
    ( (This)->lpVtbl -> TileHorizontally(This) ) 

#define IShellDispatch6_ShutdownWindows(This)	\
    ( (This)->lpVtbl -> ShutdownWindows(This) ) 

#define IShellDispatch6_Suspend(This)	\
    ( (This)->lpVtbl -> Suspend(This) ) 

#define IShellDispatch6_EjectPC(This)	\
    ( (This)->lpVtbl -> EjectPC(This) ) 

#define IShellDispatch6_SetTime(This)	\
    ( (This)->lpVtbl -> SetTime(This) ) 

#define IShellDispatch6_TrayProperties(This)	\
    ( (This)->lpVtbl -> TrayProperties(This) ) 

#define IShellDispatch6_Help(This)	\
    ( (This)->lpVtbl -> Help(This) ) 

#define IShellDispatch6_FindFiles(This)	\
    ( (This)->lpVtbl -> FindFiles(This) ) 

#define IShellDispatch6_FindComputer(This)	\
    ( (This)->lpVtbl -> FindComputer(This) ) 

#define IShellDispatch6_RefreshMenu(This)	\
    ( (This)->lpVtbl -> RefreshMenu(This) ) 

#define IShellDispatch6_ControlPanelItem(This,bstrDir)	\
    ( (This)->lpVtbl -> ControlPanelItem(This,bstrDir) ) 


#define IShellDispatch6_IsRestricted(This,Group,Restriction,plRestrictValue)	\
    ( (This)->lpVtbl -> IsRestricted(This,Group,Restriction,plRestrictValue) ) 

#define IShellDispatch6_ShellExecute(This,File,vArgs,vDir,vOperation,vShow)	\
    ( (This)->lpVtbl -> ShellExecute(This,File,vArgs,vDir,vOperation,vShow) ) 

#define IShellDispatch6_FindPrinter(This,name,location,model)	\
    ( (This)->lpVtbl -> FindPrinter(This,name,location,model) ) 

#define IShellDispatch6_GetSystemInformation(This,name,pv)	\
    ( (This)->lpVtbl -> GetSystemInformation(This,name,pv) ) 

#define IShellDispatch6_ServiceStart(This,ServiceName,Persistent,pSuccess)	\
    ( (This)->lpVtbl -> ServiceStart(This,ServiceName,Persistent,pSuccess) ) 

#define IShellDispatch6_ServiceStop(This,ServiceName,Persistent,pSuccess)	\
    ( (This)->lpVtbl -> ServiceStop(This,ServiceName,Persistent,pSuccess) ) 

#define IShellDispatch6_IsServiceRunning(This,ServiceName,pRunning)	\
    ( (This)->lpVtbl -> IsServiceRunning(This,ServiceName,pRunning) ) 

#define IShellDispatch6_CanStartStopService(This,ServiceName,pCanStartStop)	\
    ( (This)->lpVtbl -> CanStartStopService(This,ServiceName,pCanStartStop) ) 

#define IShellDispatch6_ShowBrowserBar(This,bstrClsid,bShow,pSuccess)	\
    ( (This)->lpVtbl -> ShowBrowserBar(This,bstrClsid,bShow,pSuccess) ) 


#define IShellDispatch6_AddToRecent(This,varFile,bstrCategory)	\
    ( (This)->lpVtbl -> AddToRecent(This,varFile,bstrCategory) ) 


#define IShellDispatch6_WindowsSecurity(This)	\
    ( (This)->lpVtbl -> WindowsSecurity(This) ) 

#define IShellDispatch6_ToggleDesktop(This)	\
    ( (This)->lpVtbl -> ToggleDesktop(This) ) 

#define IShellDispatch6_ExplorerPolicy(This,bstrPolicyName,pValue)	\
    ( (This)->lpVtbl -> ExplorerPolicy(This,bstrPolicyName,pValue) ) 

#define IShellDispatch6_GetSetting(This,lSetting,pResult)	\
    ( (This)->lpVtbl -> GetSetting(This,lSetting,pResult) ) 


#define IShellDispatch6_WindowSwitcher(This)	\
    ( (This)->lpVtbl -> WindowSwitcher(This) ) 


#define IShellDispatch6_SearchCommand(This)	\
    ( (This)->lpVtbl -> SearchCommand(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IShellDispatch6_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_Shell;

#ifdef __cplusplus

class DECLSPEC_UUID("13709620-C279-11CE-A49E-444553540000")
Shell;
#endif

EXTERN_C const CLSID CLSID_ShellDispatchInproc;

#ifdef __cplusplus

class DECLSPEC_UUID("0A89A860-D7B1-11CE-8350-444553540000")
ShellDispatchInproc;
#endif

#ifndef __IFileSearchBand_INTERFACE_DEFINED__
#define __IFileSearchBand_INTERFACE_DEFINED__

/* interface IFileSearchBand */
/* [object][unique][hidden][dual][oleautomation][helpstring][uuid] */ 


EXTERN_C const IID IID_IFileSearchBand;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2D91EEA1-9932-11d2-BE86-00A0C9A83DA1")
    IFileSearchBand : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetFocus( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetSearchParameters( 
            /* [in] */ __RPC__deref_in_opt BSTR *pbstrSearchID,
            /* [in] */ VARIANT_BOOL bNavToResults,
            /* [optional][in] */ __RPC__in VARIANT *pvarScope,
            /* [optional][in] */ __RPC__in VARIANT *pvarQueryFile) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_SearchID( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrSearchID) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Scope( 
            /* [retval][out] */ __RPC__out VARIANT *pvarScope) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_QueryFile( 
            /* [retval][out] */ __RPC__out VARIANT *pvarFile) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFileSearchBandVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFileSearchBand * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFileSearchBand * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFileSearchBand * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFileSearchBand * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFileSearchBand * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFileSearchBand * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFileSearchBand * This,
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
        
        DECLSPEC_XFGVIRT(IFileSearchBand, SetFocus)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetFocus )( 
            __RPC__in IFileSearchBand * This);
        
        DECLSPEC_XFGVIRT(IFileSearchBand, SetSearchParameters)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetSearchParameters )( 
            __RPC__in IFileSearchBand * This,
            /* [in] */ __RPC__deref_in_opt BSTR *pbstrSearchID,
            /* [in] */ VARIANT_BOOL bNavToResults,
            /* [optional][in] */ __RPC__in VARIANT *pvarScope,
            /* [optional][in] */ __RPC__in VARIANT *pvarQueryFile);
        
        DECLSPEC_XFGVIRT(IFileSearchBand, get_SearchID)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_SearchID )( 
            __RPC__in IFileSearchBand * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrSearchID);
        
        DECLSPEC_XFGVIRT(IFileSearchBand, get_Scope)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Scope )( 
            __RPC__in IFileSearchBand * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarScope);
        
        DECLSPEC_XFGVIRT(IFileSearchBand, get_QueryFile)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_QueryFile )( 
            __RPC__in IFileSearchBand * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarFile);
        
        END_INTERFACE
    } IFileSearchBandVtbl;

    interface IFileSearchBand
    {
        CONST_VTBL struct IFileSearchBandVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFileSearchBand_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFileSearchBand_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFileSearchBand_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFileSearchBand_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFileSearchBand_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFileSearchBand_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFileSearchBand_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFileSearchBand_SetFocus(This)	\
    ( (This)->lpVtbl -> SetFocus(This) ) 

#define IFileSearchBand_SetSearchParameters(This,pbstrSearchID,bNavToResults,pvarScope,pvarQueryFile)	\
    ( (This)->lpVtbl -> SetSearchParameters(This,pbstrSearchID,bNavToResults,pvarScope,pvarQueryFile) ) 

#define IFileSearchBand_get_SearchID(This,pbstrSearchID)	\
    ( (This)->lpVtbl -> get_SearchID(This,pbstrSearchID) ) 

#define IFileSearchBand_get_Scope(This,pvarScope)	\
    ( (This)->lpVtbl -> get_Scope(This,pvarScope) ) 

#define IFileSearchBand_get_QueryFile(This,pvarFile)	\
    ( (This)->lpVtbl -> get_QueryFile(This,pvarFile) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFileSearchBand_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_FileSearchBand;

#ifdef __cplusplus

class DECLSPEC_UUID("C4EE31F3-4768-11D2-BE5C-00A0C9A83DA1")
FileSearchBand;
#endif

#ifndef __IWebWizardHost_INTERFACE_DEFINED__
#define __IWebWizardHost_INTERFACE_DEFINED__

/* interface IWebWizardHost */
/* [helpstring][dual][object][uuid] */ 


EXTERN_C const IID IID_IWebWizardHost;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("18bcc359-4990-4bfb-b951-3c83702be5f9")
    IWebWizardHost : public IDispatch
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE FinalBack( void) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE FinalNext( void) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Cancel( void) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE put_Caption( 
            /* [in] */ __RPC__in BSTR bstrCaption) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_Caption( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrCaption) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE put_Property( 
            /* [in] */ __RPC__in BSTR bstrPropertyName,
            /* [in] */ __RPC__in VARIANT *pvProperty) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_Property( 
            /* [in] */ __RPC__in BSTR bstrPropertyName,
            /* [retval][out] */ __RPC__out VARIANT *pvProperty) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SetWizardButtons( 
            /* [in] */ VARIANT_BOOL vfEnableBack,
            /* [in] */ VARIANT_BOOL vfEnableNext,
            /* [in] */ VARIANT_BOOL vfLastPage) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SetHeaderText( 
            /* [in] */ __RPC__in BSTR bstrHeaderTitle,
            /* [in] */ __RPC__in BSTR bstrHeaderSubtitle) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWebWizardHostVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWebWizardHost * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWebWizardHost * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWebWizardHost * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IWebWizardHost * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IWebWizardHost * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IWebWizardHost * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IWebWizardHost * This,
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
        
        DECLSPEC_XFGVIRT(IWebWizardHost, FinalBack)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *FinalBack )( 
            __RPC__in IWebWizardHost * This);
        
        DECLSPEC_XFGVIRT(IWebWizardHost, FinalNext)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *FinalNext )( 
            __RPC__in IWebWizardHost * This);
        
        DECLSPEC_XFGVIRT(IWebWizardHost, Cancel)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Cancel )( 
            __RPC__in IWebWizardHost * This);
        
        DECLSPEC_XFGVIRT(IWebWizardHost, put_Caption)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Caption )( 
            __RPC__in IWebWizardHost * This,
            /* [in] */ __RPC__in BSTR bstrCaption);
        
        DECLSPEC_XFGVIRT(IWebWizardHost, get_Caption)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Caption )( 
            __RPC__in IWebWizardHost * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrCaption);
        
        DECLSPEC_XFGVIRT(IWebWizardHost, put_Property)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Property )( 
            __RPC__in IWebWizardHost * This,
            /* [in] */ __RPC__in BSTR bstrPropertyName,
            /* [in] */ __RPC__in VARIANT *pvProperty);
        
        DECLSPEC_XFGVIRT(IWebWizardHost, get_Property)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Property )( 
            __RPC__in IWebWizardHost * This,
            /* [in] */ __RPC__in BSTR bstrPropertyName,
            /* [retval][out] */ __RPC__out VARIANT *pvProperty);
        
        DECLSPEC_XFGVIRT(IWebWizardHost, SetWizardButtons)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetWizardButtons )( 
            __RPC__in IWebWizardHost * This,
            /* [in] */ VARIANT_BOOL vfEnableBack,
            /* [in] */ VARIANT_BOOL vfEnableNext,
            /* [in] */ VARIANT_BOOL vfLastPage);
        
        DECLSPEC_XFGVIRT(IWebWizardHost, SetHeaderText)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetHeaderText )( 
            __RPC__in IWebWizardHost * This,
            /* [in] */ __RPC__in BSTR bstrHeaderTitle,
            /* [in] */ __RPC__in BSTR bstrHeaderSubtitle);
        
        END_INTERFACE
    } IWebWizardHostVtbl;

    interface IWebWizardHost
    {
        CONST_VTBL struct IWebWizardHostVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWebWizardHost_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWebWizardHost_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWebWizardHost_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWebWizardHost_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IWebWizardHost_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IWebWizardHost_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IWebWizardHost_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IWebWizardHost_FinalBack(This)	\
    ( (This)->lpVtbl -> FinalBack(This) ) 

#define IWebWizardHost_FinalNext(This)	\
    ( (This)->lpVtbl -> FinalNext(This) ) 

#define IWebWizardHost_Cancel(This)	\
    ( (This)->lpVtbl -> Cancel(This) ) 

#define IWebWizardHost_put_Caption(This,bstrCaption)	\
    ( (This)->lpVtbl -> put_Caption(This,bstrCaption) ) 

#define IWebWizardHost_get_Caption(This,pbstrCaption)	\
    ( (This)->lpVtbl -> get_Caption(This,pbstrCaption) ) 

#define IWebWizardHost_put_Property(This,bstrPropertyName,pvProperty)	\
    ( (This)->lpVtbl -> put_Property(This,bstrPropertyName,pvProperty) ) 

#define IWebWizardHost_get_Property(This,bstrPropertyName,pvProperty)	\
    ( (This)->lpVtbl -> get_Property(This,bstrPropertyName,pvProperty) ) 

#define IWebWizardHost_SetWizardButtons(This,vfEnableBack,vfEnableNext,vfLastPage)	\
    ( (This)->lpVtbl -> SetWizardButtons(This,vfEnableBack,vfEnableNext,vfLastPage) ) 

#define IWebWizardHost_SetHeaderText(This,bstrHeaderTitle,bstrHeaderSubtitle)	\
    ( (This)->lpVtbl -> SetHeaderText(This,bstrHeaderTitle,bstrHeaderSubtitle) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWebWizardHost_INTERFACE_DEFINED__ */


#ifndef __IWebWizardHost2_INTERFACE_DEFINED__
#define __IWebWizardHost2_INTERFACE_DEFINED__

/* interface IWebWizardHost2 */
/* [dual][object][uuid] */ 


EXTERN_C const IID IID_IWebWizardHost2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F9C013DC-3C23-4041-8E39-CFB402F7EA59")
    IWebWizardHost2 : public IWebWizardHost
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SignString( 
            /* [in] */ __RPC__in BSTR value,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *signedValue) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWebWizardHost2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWebWizardHost2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWebWizardHost2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWebWizardHost2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IWebWizardHost2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IWebWizardHost2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IWebWizardHost2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IWebWizardHost2 * This,
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
        
        DECLSPEC_XFGVIRT(IWebWizardHost, FinalBack)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *FinalBack )( 
            __RPC__in IWebWizardHost2 * This);
        
        DECLSPEC_XFGVIRT(IWebWizardHost, FinalNext)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *FinalNext )( 
            __RPC__in IWebWizardHost2 * This);
        
        DECLSPEC_XFGVIRT(IWebWizardHost, Cancel)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Cancel )( 
            __RPC__in IWebWizardHost2 * This);
        
        DECLSPEC_XFGVIRT(IWebWizardHost, put_Caption)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Caption )( 
            __RPC__in IWebWizardHost2 * This,
            /* [in] */ __RPC__in BSTR bstrCaption);
        
        DECLSPEC_XFGVIRT(IWebWizardHost, get_Caption)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Caption )( 
            __RPC__in IWebWizardHost2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrCaption);
        
        DECLSPEC_XFGVIRT(IWebWizardHost, put_Property)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Property )( 
            __RPC__in IWebWizardHost2 * This,
            /* [in] */ __RPC__in BSTR bstrPropertyName,
            /* [in] */ __RPC__in VARIANT *pvProperty);
        
        DECLSPEC_XFGVIRT(IWebWizardHost, get_Property)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Property )( 
            __RPC__in IWebWizardHost2 * This,
            /* [in] */ __RPC__in BSTR bstrPropertyName,
            /* [retval][out] */ __RPC__out VARIANT *pvProperty);
        
        DECLSPEC_XFGVIRT(IWebWizardHost, SetWizardButtons)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetWizardButtons )( 
            __RPC__in IWebWizardHost2 * This,
            /* [in] */ VARIANT_BOOL vfEnableBack,
            /* [in] */ VARIANT_BOOL vfEnableNext,
            /* [in] */ VARIANT_BOOL vfLastPage);
        
        DECLSPEC_XFGVIRT(IWebWizardHost, SetHeaderText)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetHeaderText )( 
            __RPC__in IWebWizardHost2 * This,
            /* [in] */ __RPC__in BSTR bstrHeaderTitle,
            /* [in] */ __RPC__in BSTR bstrHeaderSubtitle);
        
        DECLSPEC_XFGVIRT(IWebWizardHost2, SignString)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SignString )( 
            __RPC__in IWebWizardHost2 * This,
            /* [in] */ __RPC__in BSTR value,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *signedValue);
        
        END_INTERFACE
    } IWebWizardHost2Vtbl;

    interface IWebWizardHost2
    {
        CONST_VTBL struct IWebWizardHost2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWebWizardHost2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWebWizardHost2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWebWizardHost2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWebWizardHost2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IWebWizardHost2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IWebWizardHost2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IWebWizardHost2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IWebWizardHost2_FinalBack(This)	\
    ( (This)->lpVtbl -> FinalBack(This) ) 

#define IWebWizardHost2_FinalNext(This)	\
    ( (This)->lpVtbl -> FinalNext(This) ) 

#define IWebWizardHost2_Cancel(This)	\
    ( (This)->lpVtbl -> Cancel(This) ) 

#define IWebWizardHost2_put_Caption(This,bstrCaption)	\
    ( (This)->lpVtbl -> put_Caption(This,bstrCaption) ) 

#define IWebWizardHost2_get_Caption(This,pbstrCaption)	\
    ( (This)->lpVtbl -> get_Caption(This,pbstrCaption) ) 

#define IWebWizardHost2_put_Property(This,bstrPropertyName,pvProperty)	\
    ( (This)->lpVtbl -> put_Property(This,bstrPropertyName,pvProperty) ) 

#define IWebWizardHost2_get_Property(This,bstrPropertyName,pvProperty)	\
    ( (This)->lpVtbl -> get_Property(This,bstrPropertyName,pvProperty) ) 

#define IWebWizardHost2_SetWizardButtons(This,vfEnableBack,vfEnableNext,vfLastPage)	\
    ( (This)->lpVtbl -> SetWizardButtons(This,vfEnableBack,vfEnableNext,vfLastPage) ) 

#define IWebWizardHost2_SetHeaderText(This,bstrHeaderTitle,bstrHeaderSubtitle)	\
    ( (This)->lpVtbl -> SetHeaderText(This,bstrHeaderTitle,bstrHeaderSubtitle) ) 


#define IWebWizardHost2_SignString(This,value,signedValue)	\
    ( (This)->lpVtbl -> SignString(This,value,signedValue) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWebWizardHost2_INTERFACE_DEFINED__ */


#ifndef __INewWDEvents_INTERFACE_DEFINED__
#define __INewWDEvents_INTERFACE_DEFINED__

/* interface INewWDEvents */
/* [helpstring][dual][object][uuid] */ 


EXTERN_C const IID IID_INewWDEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0751c551-7568-41c9-8e5b-e22e38919236")
    INewWDEvents : public IWebWizardHost
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE PassportAuthenticate( 
            /* [in] */ __RPC__in BSTR bstrSignInUrl,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pvfAuthenitcated) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct INewWDEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in INewWDEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in INewWDEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in INewWDEvents * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in INewWDEvents * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in INewWDEvents * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in INewWDEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            INewWDEvents * This,
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
        
        DECLSPEC_XFGVIRT(IWebWizardHost, FinalBack)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *FinalBack )( 
            __RPC__in INewWDEvents * This);
        
        DECLSPEC_XFGVIRT(IWebWizardHost, FinalNext)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *FinalNext )( 
            __RPC__in INewWDEvents * This);
        
        DECLSPEC_XFGVIRT(IWebWizardHost, Cancel)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Cancel )( 
            __RPC__in INewWDEvents * This);
        
        DECLSPEC_XFGVIRT(IWebWizardHost, put_Caption)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Caption )( 
            __RPC__in INewWDEvents * This,
            /* [in] */ __RPC__in BSTR bstrCaption);
        
        DECLSPEC_XFGVIRT(IWebWizardHost, get_Caption)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Caption )( 
            __RPC__in INewWDEvents * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrCaption);
        
        DECLSPEC_XFGVIRT(IWebWizardHost, put_Property)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Property )( 
            __RPC__in INewWDEvents * This,
            /* [in] */ __RPC__in BSTR bstrPropertyName,
            /* [in] */ __RPC__in VARIANT *pvProperty);
        
        DECLSPEC_XFGVIRT(IWebWizardHost, get_Property)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Property )( 
            __RPC__in INewWDEvents * This,
            /* [in] */ __RPC__in BSTR bstrPropertyName,
            /* [retval][out] */ __RPC__out VARIANT *pvProperty);
        
        DECLSPEC_XFGVIRT(IWebWizardHost, SetWizardButtons)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetWizardButtons )( 
            __RPC__in INewWDEvents * This,
            /* [in] */ VARIANT_BOOL vfEnableBack,
            /* [in] */ VARIANT_BOOL vfEnableNext,
            /* [in] */ VARIANT_BOOL vfLastPage);
        
        DECLSPEC_XFGVIRT(IWebWizardHost, SetHeaderText)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetHeaderText )( 
            __RPC__in INewWDEvents * This,
            /* [in] */ __RPC__in BSTR bstrHeaderTitle,
            /* [in] */ __RPC__in BSTR bstrHeaderSubtitle);
        
        DECLSPEC_XFGVIRT(INewWDEvents, PassportAuthenticate)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *PassportAuthenticate )( 
            __RPC__in INewWDEvents * This,
            /* [in] */ __RPC__in BSTR bstrSignInUrl,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pvfAuthenitcated);
        
        END_INTERFACE
    } INewWDEventsVtbl;

    interface INewWDEvents
    {
        CONST_VTBL struct INewWDEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define INewWDEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define INewWDEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define INewWDEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define INewWDEvents_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define INewWDEvents_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define INewWDEvents_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define INewWDEvents_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define INewWDEvents_FinalBack(This)	\
    ( (This)->lpVtbl -> FinalBack(This) ) 

#define INewWDEvents_FinalNext(This)	\
    ( (This)->lpVtbl -> FinalNext(This) ) 

#define INewWDEvents_Cancel(This)	\
    ( (This)->lpVtbl -> Cancel(This) ) 

#define INewWDEvents_put_Caption(This,bstrCaption)	\
    ( (This)->lpVtbl -> put_Caption(This,bstrCaption) ) 

#define INewWDEvents_get_Caption(This,pbstrCaption)	\
    ( (This)->lpVtbl -> get_Caption(This,pbstrCaption) ) 

#define INewWDEvents_put_Property(This,bstrPropertyName,pvProperty)	\
    ( (This)->lpVtbl -> put_Property(This,bstrPropertyName,pvProperty) ) 

#define INewWDEvents_get_Property(This,bstrPropertyName,pvProperty)	\
    ( (This)->lpVtbl -> get_Property(This,bstrPropertyName,pvProperty) ) 

#define INewWDEvents_SetWizardButtons(This,vfEnableBack,vfEnableNext,vfLastPage)	\
    ( (This)->lpVtbl -> SetWizardButtons(This,vfEnableBack,vfEnableNext,vfLastPage) ) 

#define INewWDEvents_SetHeaderText(This,bstrHeaderTitle,bstrHeaderSubtitle)	\
    ( (This)->lpVtbl -> SetHeaderText(This,bstrHeaderTitle,bstrHeaderSubtitle) ) 


#define INewWDEvents_PassportAuthenticate(This,bstrSignInUrl,pvfAuthenitcated)	\
    ( (This)->lpVtbl -> PassportAuthenticate(This,bstrSignInUrl,pvfAuthenitcated) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __INewWDEvents_INTERFACE_DEFINED__ */

#endif /* __Shell32_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_shldisp_0000_0001 */
/* [local] */ 


//-------------------------------------------------------------------------
//
// IAutoComplete interface
//
//
// [Member functions]
//
// IAutoComplete::Init(hwndEdit, punkACL, pwszRegKeyPath, pwszQuickComplete)
//   This function initializes an AutoComplete object, telling it
//   what control to subclass, and what list of strings to process.
//
// IAutoComplete::Enable(fEnable)
//   This function enables or disables the AutoComplete functionality.
//
//-------------------------------------------------------------------------


extern RPC_IF_HANDLE __MIDL_itf_shldisp_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_shldisp_0000_0001_v0_0_s_ifspec;

#ifndef __IAutoComplete_INTERFACE_DEFINED__
#define __IAutoComplete_INTERFACE_DEFINED__

/* interface IAutoComplete */
/* [unique][uuid][object][local][helpstring] */ 

typedef /* [unique] */ IAutoComplete *LPAUTOCOMPLETE;


EXTERN_C const IID IID_IAutoComplete;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00bb2762-6a77-11d0-a535-00c04fd7d062")
    IAutoComplete : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Init( 
            /* [annotation][in] */ 
            _In_  HWND hwndEdit,
            /* [annotation][unique][in] */ 
            _In_  IUnknown *punkACL,
            /* [annotation][unique][in] */ 
            _In_opt_  LPCWSTR pwszRegKeyPath,
            /* [annotation][in] */ 
            _In_opt_  LPCWSTR pwszQuickComplete) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Enable( 
            /* [in] */ BOOL fEnable) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAutoCompleteVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAutoComplete * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAutoComplete * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAutoComplete * This);
        
        DECLSPEC_XFGVIRT(IAutoComplete, Init)
        HRESULT ( STDMETHODCALLTYPE *Init )( 
            IAutoComplete * This,
            /* [annotation][in] */ 
            _In_  HWND hwndEdit,
            /* [annotation][unique][in] */ 
            _In_  IUnknown *punkACL,
            /* [annotation][unique][in] */ 
            _In_opt_  LPCWSTR pwszRegKeyPath,
            /* [annotation][in] */ 
            _In_opt_  LPCWSTR pwszQuickComplete);
        
        DECLSPEC_XFGVIRT(IAutoComplete, Enable)
        HRESULT ( STDMETHODCALLTYPE *Enable )( 
            IAutoComplete * This,
            /* [in] */ BOOL fEnable);
        
        END_INTERFACE
    } IAutoCompleteVtbl;

    interface IAutoComplete
    {
        CONST_VTBL struct IAutoCompleteVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAutoComplete_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAutoComplete_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAutoComplete_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAutoComplete_Init(This,hwndEdit,punkACL,pwszRegKeyPath,pwszQuickComplete)	\
    ( (This)->lpVtbl -> Init(This,hwndEdit,punkACL,pwszRegKeyPath,pwszQuickComplete) ) 

#define IAutoComplete_Enable(This,fEnable)	\
    ( (This)->lpVtbl -> Enable(This,fEnable) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAutoComplete_INTERFACE_DEFINED__ */


#ifndef __IAutoComplete2_INTERFACE_DEFINED__
#define __IAutoComplete2_INTERFACE_DEFINED__

/* interface IAutoComplete2 */
/* [unique][uuid][object][local][helpstring] */ 

typedef /* [unique] */ IAutoComplete2 *LPAUTOCOMPLETE2;

typedef 
enum _tagAUTOCOMPLETEOPTIONS
    {
        ACO_NONE	= 0,
        ACO_AUTOSUGGEST	= 0x1,
        ACO_AUTOAPPEND	= 0x2,
        ACO_SEARCH	= 0x4,
        ACO_FILTERPREFIXES	= 0x8,
        ACO_USETAB	= 0x10,
        ACO_UPDOWNKEYDROPSLIST	= 0x20,
        ACO_RTLREADING	= 0x40,
        ACO_WORD_FILTER	= 0x80,
        ACO_NOPREFIXFILTERING	= 0x100
    } 	AUTOCOMPLETEOPTIONS;


EXTERN_C const IID IID_IAutoComplete2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("EAC04BC0-3791-11d2-BB95-0060977B464C")
    IAutoComplete2 : public IAutoComplete
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetOptions( 
            /* [in] */ DWORD dwFlag) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOptions( 
            /* [annotation][out] */ 
            _Out_  DWORD *pdwFlag) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAutoComplete2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAutoComplete2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAutoComplete2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAutoComplete2 * This);
        
        DECLSPEC_XFGVIRT(IAutoComplete, Init)
        HRESULT ( STDMETHODCALLTYPE *Init )( 
            IAutoComplete2 * This,
            /* [annotation][in] */ 
            _In_  HWND hwndEdit,
            /* [annotation][unique][in] */ 
            _In_  IUnknown *punkACL,
            /* [annotation][unique][in] */ 
            _In_opt_  LPCWSTR pwszRegKeyPath,
            /* [annotation][in] */ 
            _In_opt_  LPCWSTR pwszQuickComplete);
        
        DECLSPEC_XFGVIRT(IAutoComplete, Enable)
        HRESULT ( STDMETHODCALLTYPE *Enable )( 
            IAutoComplete2 * This,
            /* [in] */ BOOL fEnable);
        
        DECLSPEC_XFGVIRT(IAutoComplete2, SetOptions)
        HRESULT ( STDMETHODCALLTYPE *SetOptions )( 
            IAutoComplete2 * This,
            /* [in] */ DWORD dwFlag);
        
        DECLSPEC_XFGVIRT(IAutoComplete2, GetOptions)
        HRESULT ( STDMETHODCALLTYPE *GetOptions )( 
            IAutoComplete2 * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwFlag);
        
        END_INTERFACE
    } IAutoComplete2Vtbl;

    interface IAutoComplete2
    {
        CONST_VTBL struct IAutoComplete2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAutoComplete2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAutoComplete2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAutoComplete2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAutoComplete2_Init(This,hwndEdit,punkACL,pwszRegKeyPath,pwszQuickComplete)	\
    ( (This)->lpVtbl -> Init(This,hwndEdit,punkACL,pwszRegKeyPath,pwszQuickComplete) ) 

#define IAutoComplete2_Enable(This,fEnable)	\
    ( (This)->lpVtbl -> Enable(This,fEnable) ) 


#define IAutoComplete2_SetOptions(This,dwFlag)	\
    ( (This)->lpVtbl -> SetOptions(This,dwFlag) ) 

#define IAutoComplete2_GetOptions(This,pdwFlag)	\
    ( (This)->lpVtbl -> GetOptions(This,pdwFlag) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAutoComplete2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_shldisp_0000_0003 */
/* [local] */ 

// INTERFACE: IEnumACString
//
// This interface was implemented to return autocomplete strings
// into the caller's buffer (to reduce the number of memory allocations).
// A sort index is also returned to control the order of items displayed.
// by autocomplete.  The sort index should be set to zero if unused.
//
// The NextItem method increments the current index by one (similar to Next
// when one item is requested).
//


extern RPC_IF_HANDLE __MIDL_itf_shldisp_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_shldisp_0000_0003_v0_0_s_ifspec;

#ifndef __IEnumACString_INTERFACE_DEFINED__
#define __IEnumACString_INTERFACE_DEFINED__

/* interface IEnumACString */
/* [unique][uuid][object][local][helpstring] */ 

typedef /* [unique] */ IEnumACString *PENUMACSTRING;

typedef /* [unique] */ IEnumACString *LPENUMACSTRING;

typedef 
enum _tagACENUMOPTION
    {
        ACEO_NONE	= 0,
        ACEO_MOSTRECENTFIRST	= 0x1,
        ACEO_FIRSTUNUSED	= 0x10000
    } 	ACENUMOPTION;


EXTERN_C const IID IID_IEnumACString;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8E74C210-CF9D-4eaf-A403-7356428F0A5A")
    IEnumACString : public IEnumString
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE NextItem( 
            /* [annotation][size_is][unique][string][out] */ 
            _Out_writes_opt_(cchMax)  LPWSTR pszUrl,
            /* [in] */ ULONG cchMax,
            /* [annotation][out] */ 
            _Out_  ULONG *pulSortIndex) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetEnumOptions( 
            /* [in] */ DWORD dwOptions) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetEnumOptions( 
            /* [annotation][out] */ 
            _Out_  DWORD *pdwOptions) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumACStringVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IEnumACString * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IEnumACString * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IEnumACString * This);
        
        DECLSPEC_XFGVIRT(IEnumString, Next)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Next )( 
            IEnumACString * This,
            /* [in] */ ULONG celt,
            /* [annotation] */ 
            _Out_writes_to_(celt,*pceltFetched)  LPOLESTR *rgelt,
            /* [annotation] */ 
            _Out_opt_  ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IEnumString, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            IEnumACString * This,
            /* [in] */ ULONG celt);
        
        DECLSPEC_XFGVIRT(IEnumString, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            IEnumACString * This);
        
        DECLSPEC_XFGVIRT(IEnumString, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            IEnumACString * This,
            /* [out] */ IEnumString **ppenum);
        
        DECLSPEC_XFGVIRT(IEnumACString, NextItem)
        HRESULT ( STDMETHODCALLTYPE *NextItem )( 
            IEnumACString * This,
            /* [annotation][size_is][unique][string][out] */ 
            _Out_writes_opt_(cchMax)  LPWSTR pszUrl,
            /* [in] */ ULONG cchMax,
            /* [annotation][out] */ 
            _Out_  ULONG *pulSortIndex);
        
        DECLSPEC_XFGVIRT(IEnumACString, SetEnumOptions)
        HRESULT ( STDMETHODCALLTYPE *SetEnumOptions )( 
            IEnumACString * This,
            /* [in] */ DWORD dwOptions);
        
        DECLSPEC_XFGVIRT(IEnumACString, GetEnumOptions)
        HRESULT ( STDMETHODCALLTYPE *GetEnumOptions )( 
            IEnumACString * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwOptions);
        
        END_INTERFACE
    } IEnumACStringVtbl;

    interface IEnumACString
    {
        CONST_VTBL struct IEnumACStringVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumACString_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumACString_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumACString_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumACString_Next(This,celt,rgelt,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,rgelt,pceltFetched) ) 

#define IEnumACString_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#define IEnumACString_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumACString_Clone(This,ppenum)	\
    ( (This)->lpVtbl -> Clone(This,ppenum) ) 


#define IEnumACString_NextItem(This,pszUrl,cchMax,pulSortIndex)	\
    ( (This)->lpVtbl -> NextItem(This,pszUrl,cchMax,pulSortIndex) ) 

#define IEnumACString_SetEnumOptions(This,dwOptions)	\
    ( (This)->lpVtbl -> SetEnumOptions(This,dwOptions) ) 

#define IEnumACString_GetEnumOptions(This,pdwOptions)	\
    ( (This)->lpVtbl -> GetEnumOptions(This,pdwOptions) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumACString_INTERFACE_DEFINED__ */


#ifndef __IDataObjectAsyncCapability_INTERFACE_DEFINED__
#define __IDataObjectAsyncCapability_INTERFACE_DEFINED__

/* interface IDataObjectAsyncCapability */
/* [object][uuid] */ 


EXTERN_C const IID IID_IDataObjectAsyncCapability;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3D8B0590-F691-11d2-8EA9-006097DF5BD4")
    IDataObjectAsyncCapability : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetAsyncMode( 
            /* [in] */ BOOL fDoOpAsync) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAsyncMode( 
            /* [out] */ __RPC__out BOOL *pfIsOpAsync) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE StartOperation( 
            /* [optional][unique][in] */ __RPC__in_opt IBindCtx *pbcReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InOperation( 
            /* [out] */ __RPC__out BOOL *pfInAsyncOp) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EndOperation( 
            /* [in] */ HRESULT hResult,
            /* [unique][in] */ __RPC__in_opt IBindCtx *pbcReserved,
            /* [in] */ DWORD dwEffects) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDataObjectAsyncCapabilityVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDataObjectAsyncCapability * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDataObjectAsyncCapability * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDataObjectAsyncCapability * This);
        
        DECLSPEC_XFGVIRT(IDataObjectAsyncCapability, SetAsyncMode)
        HRESULT ( STDMETHODCALLTYPE *SetAsyncMode )( 
            __RPC__in IDataObjectAsyncCapability * This,
            /* [in] */ BOOL fDoOpAsync);
        
        DECLSPEC_XFGVIRT(IDataObjectAsyncCapability, GetAsyncMode)
        HRESULT ( STDMETHODCALLTYPE *GetAsyncMode )( 
            __RPC__in IDataObjectAsyncCapability * This,
            /* [out] */ __RPC__out BOOL *pfIsOpAsync);
        
        DECLSPEC_XFGVIRT(IDataObjectAsyncCapability, StartOperation)
        HRESULT ( STDMETHODCALLTYPE *StartOperation )( 
            __RPC__in IDataObjectAsyncCapability * This,
            /* [optional][unique][in] */ __RPC__in_opt IBindCtx *pbcReserved);
        
        DECLSPEC_XFGVIRT(IDataObjectAsyncCapability, InOperation)
        HRESULT ( STDMETHODCALLTYPE *InOperation )( 
            __RPC__in IDataObjectAsyncCapability * This,
            /* [out] */ __RPC__out BOOL *pfInAsyncOp);
        
        DECLSPEC_XFGVIRT(IDataObjectAsyncCapability, EndOperation)
        HRESULT ( STDMETHODCALLTYPE *EndOperation )( 
            __RPC__in IDataObjectAsyncCapability * This,
            /* [in] */ HRESULT hResult,
            /* [unique][in] */ __RPC__in_opt IBindCtx *pbcReserved,
            /* [in] */ DWORD dwEffects);
        
        END_INTERFACE
    } IDataObjectAsyncCapabilityVtbl;

    interface IDataObjectAsyncCapability
    {
        CONST_VTBL struct IDataObjectAsyncCapabilityVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDataObjectAsyncCapability_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDataObjectAsyncCapability_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDataObjectAsyncCapability_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDataObjectAsyncCapability_SetAsyncMode(This,fDoOpAsync)	\
    ( (This)->lpVtbl -> SetAsyncMode(This,fDoOpAsync) ) 

#define IDataObjectAsyncCapability_GetAsyncMode(This,pfIsOpAsync)	\
    ( (This)->lpVtbl -> GetAsyncMode(This,pfIsOpAsync) ) 

#define IDataObjectAsyncCapability_StartOperation(This,pbcReserved)	\
    ( (This)->lpVtbl -> StartOperation(This,pbcReserved) ) 

#define IDataObjectAsyncCapability_InOperation(This,pfInAsyncOp)	\
    ( (This)->lpVtbl -> InOperation(This,pfInAsyncOp) ) 

#define IDataObjectAsyncCapability_EndOperation(This,hResult,pbcReserved,dwEffects)	\
    ( (This)->lpVtbl -> EndOperation(This,hResult,pbcReserved,dwEffects) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDataObjectAsyncCapability_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_shldisp_0000_0005 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_shldisp_0000_0005_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_shldisp_0000_0005_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


