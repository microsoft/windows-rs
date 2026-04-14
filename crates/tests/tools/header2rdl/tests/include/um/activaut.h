

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

#ifndef __activaut_h__
#define __activaut_h__

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

#ifndef __IScriptNode_FWD_DEFINED__
#define __IScriptNode_FWD_DEFINED__
typedef interface IScriptNode IScriptNode;

#endif 	/* __IScriptNode_FWD_DEFINED__ */


#ifndef __IScriptEntry_FWD_DEFINED__
#define __IScriptEntry_FWD_DEFINED__
typedef interface IScriptEntry IScriptEntry;

#endif 	/* __IScriptEntry_FWD_DEFINED__ */


#ifndef __IScriptScriptlet_FWD_DEFINED__
#define __IScriptScriptlet_FWD_DEFINED__
typedef interface IScriptScriptlet IScriptScriptlet;

#endif 	/* __IScriptScriptlet_FWD_DEFINED__ */


#ifndef __IActiveScriptAuthor_FWD_DEFINED__
#define __IActiveScriptAuthor_FWD_DEFINED__
typedef interface IActiveScriptAuthor IActiveScriptAuthor;

#endif 	/* __IActiveScriptAuthor_FWD_DEFINED__ */


#ifndef __IActiveScriptAuthorProcedure_FWD_DEFINED__
#define __IActiveScriptAuthorProcedure_FWD_DEFINED__
typedef interface IActiveScriptAuthorProcedure IActiveScriptAuthorProcedure;

#endif 	/* __IActiveScriptAuthorProcedure_FWD_DEFINED__ */


/* header files for imported files */
#include "ocidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_activaut_0000_0000 */
/* [local] */ 

//=--------------------------------------------------------------------------=
// ActivAut.h
//=--------------------------------------------------------------------------=
// (C) Copyright 1997 Microsoft Corporation.  All Rights Reserved.
//
// THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF
// ANY KIND, EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// THE IMPLIED WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A
// PARTICULAR PURPOSE.
//=--------------------------------------------------------------------------=
//
#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#pragma comment(lib,"uuid.lib")
//
// Declarations for ActiveX Script Authoring.
//

#ifndef __ActivAut_h
#define __ActivAut_h

/* GUIDs
 ********/

#ifndef _NO_AUTHOR_GUIDS
// {0AEE2A92-BCBB-11d0-8C72-00C04FC2B085}
DEFINE_GUID(CATID_ActiveScriptAuthor, 0xaee2a92, 0xbcbb, 0x11d0, 0x8c, 0x72, 0x0, 0xc0, 0x4f, 0xc2, 0xb0, 0x85);

// {9C109DA0-7006-11d1-B36C-00A0C911E8B2}
DEFINE_GUID(IID_IActiveScriptAuthor, 0x9c109da0, 0x7006, 0x11d1, 0xb3, 0x6c, 0x00, 0xa0, 0xc9, 0x11, 0xe8, 0xb2);

// {0AEE2A94-BCBB-11d0-8C72-00C04FC2B085}
DEFINE_GUID(IID_IScriptNode, 0xaee2a94, 0xbcbb, 0x11d0, 0x8c, 0x72, 0x0, 0xc0, 0x4f, 0xc2, 0xb0, 0x85);

// {0AEE2A95-BCBB-11d0-8C72-00C04FC2B085}
DEFINE_GUID(IID_IScriptEntry, 0xaee2a95, 0xbcbb, 0x11d0, 0x8c, 0x72, 0x0, 0xc0, 0x4f, 0xc2, 0xb0, 0x85);

// {0AEE2A96-BCBB-11d0-8C72-00C04FC2B085}
DEFINE_GUID(IID_IScriptScriptlet, 0xaee2a96, 0xbcbb, 0x11d0, 0x8c, 0x72, 0x0, 0xc0, 0x4f, 0xc2, 0xb0, 0x85);

// {7E2D4B70-BD9A-11d0-9336-00A0C90DCAA9}
DEFINE_GUID(IID_IActiveScriptAuthorProcedure, 0x7e2d4b70, 0xbd9a, 0x11d0, 0x93, 0x36, 0x0, 0xa0, 0xc9, 0xd, 0xca, 0xa9);

#endif // _NO_AUTHOR_GUIDS

/* Interfaces
 *************/


// Output flags for GetLanguageFlags
const DWORD fasaPreferInternalHandler = 0x0001;
const DWORD fasaSupportInternalHandler = 0x0002;
const DWORD fasaCaseSensitive = 0x0004;

// Flags for statement completion
const DWORD SCRIPT_CMPL_NOLIST     = 0x0000;
const DWORD SCRIPT_CMPL_MEMBERLIST = 0x0001;
const DWORD SCRIPT_CMPL_ENUMLIST   = 0x0002;
const DWORD SCRIPT_CMPL_PARAMTIP   = 0x0004;
const DWORD SCRIPT_CMPL_GLOBALLIST = 0x0008;

const DWORD SCRIPT_CMPL_ENUM_TRIGGER   = 0x0001;
const DWORD SCRIPT_CMPL_MEMBER_TRIGGER = 0x0002;
const DWORD SCRIPT_CMPL_PARAM_TRIGGER  = 0x0003;
const DWORD SCRIPT_CMPL_COMMIT         = 0x0004;

// Flags for getting attributes
const DWORD GETATTRTYPE_NORMAL    = 0x0000;
const DWORD GETATTRTYPE_DEPSCAN   = 0x0001;

// GETATTRFLAG_THIS can be used with either GETATTRTYPE_NORMAL or GETATTRTYPE_DEPSCAN
const DWORD GETATTRFLAG_THIS	     = 0x0100;
const DWORD GETATTRFLAG_HUMANTEXT = 0x8000;

// When the GETATTR_LOCALIZE flag is set the following bit will be
// set for source attributes that should be considered localized
const DWORD SOURCETEXT_ATTR_HUMANTEXT = 0x8000;

// Valid attributes returned for GETATTR_DEPSCAN
const DWORD SOURCETEXT_ATTR_IDENTIFIER     = 0x0100;
const DWORD SOURCETEXT_ATTR_MEMBERLOOKUP   = 0x0200;

// When the GETATTRFLAG_THIS flag is set the following bit will be
// set for the this pointer (JS) or me pointer (VBS)
const DWORD SOURCETEXT_ATTR_THIS           = 0x0400;




typedef WORD SOURCE_TEXT_ATTR;



extern RPC_IF_HANDLE __MIDL_itf_activaut_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_activaut_0000_0000_v0_0_s_ifspec;

#ifndef __IScriptNode_INTERFACE_DEFINED__
#define __IScriptNode_INTERFACE_DEFINED__

/* interface IScriptNode */
/* [object][unique][uuid] */ 


EXTERN_C const IID IID_IScriptNode;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0AEE2A94-BCBB-11d0-8C72-00C04FC2B085")
    IScriptNode : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Alive( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Delete( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetParent( 
            /* [out] */ __RPC__deref_out_opt IScriptNode **ppsnParent) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetIndexInParent( 
            /* [out] */ __RPC__out ULONG *pisn) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCookie( 
            /* [out] */ __RPC__out DWORD *pdwCookie) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNumberOfChildren( 
            /* [out] */ __RPC__out ULONG *pcsn) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetChild( 
            /* [in] */ ULONG isn,
            /* [out] */ __RPC__deref_out_opt IScriptNode **ppsn) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLanguage( 
            /* [out] */ __RPC__deref_out_opt BSTR *pbstr) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateChildEntry( 
            /* [in] */ ULONG isn,
            /* [in] */ DWORD dwCookie,
            /* [in] */ __RPC__in LPCOLESTR pszDelimiter,
            /* [out] */ __RPC__deref_out_opt IScriptEntry **ppse) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateChildHandler( 
            /* [in] */ __RPC__in LPCOLESTR pszDefaultName,
            /* [size_is][in] */ __RPC__in_ecount_full(cpszNames) LPCOLESTR *prgpszNames,
            /* [in] */ ULONG cpszNames,
            /* [in] */ __RPC__in LPCOLESTR pszEvent,
            /* [in] */ __RPC__in LPCOLESTR pszDelimiter,
            /* [in] */ __RPC__in_opt ITypeInfo *ptiSignature,
            /* [in] */ ULONG iMethodSignature,
            /* [in] */ ULONG isn,
            /* [in] */ DWORD dwCookie,
            /* [out] */ __RPC__deref_out_opt IScriptEntry **ppse) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IScriptNodeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IScriptNode * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IScriptNode * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IScriptNode * This);
        
        DECLSPEC_XFGVIRT(IScriptNode, Alive)
        HRESULT ( STDMETHODCALLTYPE *Alive )( 
            __RPC__in IScriptNode * This);
        
        DECLSPEC_XFGVIRT(IScriptNode, Delete)
        HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IScriptNode * This);
        
        DECLSPEC_XFGVIRT(IScriptNode, GetParent)
        HRESULT ( STDMETHODCALLTYPE *GetParent )( 
            __RPC__in IScriptNode * This,
            /* [out] */ __RPC__deref_out_opt IScriptNode **ppsnParent);
        
        DECLSPEC_XFGVIRT(IScriptNode, GetIndexInParent)
        HRESULT ( STDMETHODCALLTYPE *GetIndexInParent )( 
            __RPC__in IScriptNode * This,
            /* [out] */ __RPC__out ULONG *pisn);
        
        DECLSPEC_XFGVIRT(IScriptNode, GetCookie)
        HRESULT ( STDMETHODCALLTYPE *GetCookie )( 
            __RPC__in IScriptNode * This,
            /* [out] */ __RPC__out DWORD *pdwCookie);
        
        DECLSPEC_XFGVIRT(IScriptNode, GetNumberOfChildren)
        HRESULT ( STDMETHODCALLTYPE *GetNumberOfChildren )( 
            __RPC__in IScriptNode * This,
            /* [out] */ __RPC__out ULONG *pcsn);
        
        DECLSPEC_XFGVIRT(IScriptNode, GetChild)
        HRESULT ( STDMETHODCALLTYPE *GetChild )( 
            __RPC__in IScriptNode * This,
            /* [in] */ ULONG isn,
            /* [out] */ __RPC__deref_out_opt IScriptNode **ppsn);
        
        DECLSPEC_XFGVIRT(IScriptNode, GetLanguage)
        HRESULT ( STDMETHODCALLTYPE *GetLanguage )( 
            __RPC__in IScriptNode * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        DECLSPEC_XFGVIRT(IScriptNode, CreateChildEntry)
        HRESULT ( STDMETHODCALLTYPE *CreateChildEntry )( 
            __RPC__in IScriptNode * This,
            /* [in] */ ULONG isn,
            /* [in] */ DWORD dwCookie,
            /* [in] */ __RPC__in LPCOLESTR pszDelimiter,
            /* [out] */ __RPC__deref_out_opt IScriptEntry **ppse);
        
        DECLSPEC_XFGVIRT(IScriptNode, CreateChildHandler)
        HRESULT ( STDMETHODCALLTYPE *CreateChildHandler )( 
            __RPC__in IScriptNode * This,
            /* [in] */ __RPC__in LPCOLESTR pszDefaultName,
            /* [size_is][in] */ __RPC__in_ecount_full(cpszNames) LPCOLESTR *prgpszNames,
            /* [in] */ ULONG cpszNames,
            /* [in] */ __RPC__in LPCOLESTR pszEvent,
            /* [in] */ __RPC__in LPCOLESTR pszDelimiter,
            /* [in] */ __RPC__in_opt ITypeInfo *ptiSignature,
            /* [in] */ ULONG iMethodSignature,
            /* [in] */ ULONG isn,
            /* [in] */ DWORD dwCookie,
            /* [out] */ __RPC__deref_out_opt IScriptEntry **ppse);
        
        END_INTERFACE
    } IScriptNodeVtbl;

    interface IScriptNode
    {
        CONST_VTBL struct IScriptNodeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IScriptNode_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IScriptNode_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IScriptNode_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IScriptNode_Alive(This)	\
    ( (This)->lpVtbl -> Alive(This) ) 

#define IScriptNode_Delete(This)	\
    ( (This)->lpVtbl -> Delete(This) ) 

#define IScriptNode_GetParent(This,ppsnParent)	\
    ( (This)->lpVtbl -> GetParent(This,ppsnParent) ) 

#define IScriptNode_GetIndexInParent(This,pisn)	\
    ( (This)->lpVtbl -> GetIndexInParent(This,pisn) ) 

#define IScriptNode_GetCookie(This,pdwCookie)	\
    ( (This)->lpVtbl -> GetCookie(This,pdwCookie) ) 

#define IScriptNode_GetNumberOfChildren(This,pcsn)	\
    ( (This)->lpVtbl -> GetNumberOfChildren(This,pcsn) ) 

#define IScriptNode_GetChild(This,isn,ppsn)	\
    ( (This)->lpVtbl -> GetChild(This,isn,ppsn) ) 

#define IScriptNode_GetLanguage(This,pbstr)	\
    ( (This)->lpVtbl -> GetLanguage(This,pbstr) ) 

#define IScriptNode_CreateChildEntry(This,isn,dwCookie,pszDelimiter,ppse)	\
    ( (This)->lpVtbl -> CreateChildEntry(This,isn,dwCookie,pszDelimiter,ppse) ) 

#define IScriptNode_CreateChildHandler(This,pszDefaultName,prgpszNames,cpszNames,pszEvent,pszDelimiter,ptiSignature,iMethodSignature,isn,dwCookie,ppse)	\
    ( (This)->lpVtbl -> CreateChildHandler(This,pszDefaultName,prgpszNames,cpszNames,pszEvent,pszDelimiter,ptiSignature,iMethodSignature,isn,dwCookie,ppse) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IScriptNode_INTERFACE_DEFINED__ */


#ifndef __IScriptEntry_INTERFACE_DEFINED__
#define __IScriptEntry_INTERFACE_DEFINED__

/* interface IScriptEntry */
/* [object][unique][uuid] */ 


EXTERN_C const IID IID_IScriptEntry;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0AEE2A95-BCBB-11d0-8C72-00C04FC2B085")
    IScriptEntry : public IScriptNode
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetText( 
            /* [out] */ __RPC__deref_out_opt BSTR *pbstr) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetText( 
            /* [in] */ __RPC__in LPCOLESTR psz) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetBody( 
            /* [out] */ __RPC__deref_out_opt BSTR *pbstr) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetBody( 
            /* [in] */ __RPC__in LPCOLESTR psz) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetName( 
            /* [out] */ __RPC__deref_out_opt BSTR *pbstr) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetName( 
            /* [in] */ __RPC__in LPCOLESTR psz) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetItemName( 
            /* [out] */ __RPC__deref_out_opt BSTR *pbstr) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetItemName( 
            /* [in] */ __RPC__in LPCOLESTR psz) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSignature( 
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppti,
            /* [out] */ __RPC__out ULONG *piMethod) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetSignature( 
            /* [in] */ __RPC__in_opt ITypeInfo *pti,
            /* [in] */ ULONG iMethod) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRange( 
            /* [out] */ __RPC__out ULONG *pichMin,
            /* [out] */ __RPC__out ULONG *pcch) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IScriptEntryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IScriptEntry * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IScriptEntry * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IScriptEntry * This);
        
        DECLSPEC_XFGVIRT(IScriptNode, Alive)
        HRESULT ( STDMETHODCALLTYPE *Alive )( 
            __RPC__in IScriptEntry * This);
        
        DECLSPEC_XFGVIRT(IScriptNode, Delete)
        HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IScriptEntry * This);
        
        DECLSPEC_XFGVIRT(IScriptNode, GetParent)
        HRESULT ( STDMETHODCALLTYPE *GetParent )( 
            __RPC__in IScriptEntry * This,
            /* [out] */ __RPC__deref_out_opt IScriptNode **ppsnParent);
        
        DECLSPEC_XFGVIRT(IScriptNode, GetIndexInParent)
        HRESULT ( STDMETHODCALLTYPE *GetIndexInParent )( 
            __RPC__in IScriptEntry * This,
            /* [out] */ __RPC__out ULONG *pisn);
        
        DECLSPEC_XFGVIRT(IScriptNode, GetCookie)
        HRESULT ( STDMETHODCALLTYPE *GetCookie )( 
            __RPC__in IScriptEntry * This,
            /* [out] */ __RPC__out DWORD *pdwCookie);
        
        DECLSPEC_XFGVIRT(IScriptNode, GetNumberOfChildren)
        HRESULT ( STDMETHODCALLTYPE *GetNumberOfChildren )( 
            __RPC__in IScriptEntry * This,
            /* [out] */ __RPC__out ULONG *pcsn);
        
        DECLSPEC_XFGVIRT(IScriptNode, GetChild)
        HRESULT ( STDMETHODCALLTYPE *GetChild )( 
            __RPC__in IScriptEntry * This,
            /* [in] */ ULONG isn,
            /* [out] */ __RPC__deref_out_opt IScriptNode **ppsn);
        
        DECLSPEC_XFGVIRT(IScriptNode, GetLanguage)
        HRESULT ( STDMETHODCALLTYPE *GetLanguage )( 
            __RPC__in IScriptEntry * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        DECLSPEC_XFGVIRT(IScriptNode, CreateChildEntry)
        HRESULT ( STDMETHODCALLTYPE *CreateChildEntry )( 
            __RPC__in IScriptEntry * This,
            /* [in] */ ULONG isn,
            /* [in] */ DWORD dwCookie,
            /* [in] */ __RPC__in LPCOLESTR pszDelimiter,
            /* [out] */ __RPC__deref_out_opt IScriptEntry **ppse);
        
        DECLSPEC_XFGVIRT(IScriptNode, CreateChildHandler)
        HRESULT ( STDMETHODCALLTYPE *CreateChildHandler )( 
            __RPC__in IScriptEntry * This,
            /* [in] */ __RPC__in LPCOLESTR pszDefaultName,
            /* [size_is][in] */ __RPC__in_ecount_full(cpszNames) LPCOLESTR *prgpszNames,
            /* [in] */ ULONG cpszNames,
            /* [in] */ __RPC__in LPCOLESTR pszEvent,
            /* [in] */ __RPC__in LPCOLESTR pszDelimiter,
            /* [in] */ __RPC__in_opt ITypeInfo *ptiSignature,
            /* [in] */ ULONG iMethodSignature,
            /* [in] */ ULONG isn,
            /* [in] */ DWORD dwCookie,
            /* [out] */ __RPC__deref_out_opt IScriptEntry **ppse);
        
        DECLSPEC_XFGVIRT(IScriptEntry, GetText)
        HRESULT ( STDMETHODCALLTYPE *GetText )( 
            __RPC__in IScriptEntry * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        DECLSPEC_XFGVIRT(IScriptEntry, SetText)
        HRESULT ( STDMETHODCALLTYPE *SetText )( 
            __RPC__in IScriptEntry * This,
            /* [in] */ __RPC__in LPCOLESTR psz);
        
        DECLSPEC_XFGVIRT(IScriptEntry, GetBody)
        HRESULT ( STDMETHODCALLTYPE *GetBody )( 
            __RPC__in IScriptEntry * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        DECLSPEC_XFGVIRT(IScriptEntry, SetBody)
        HRESULT ( STDMETHODCALLTYPE *SetBody )( 
            __RPC__in IScriptEntry * This,
            /* [in] */ __RPC__in LPCOLESTR psz);
        
        DECLSPEC_XFGVIRT(IScriptEntry, GetName)
        HRESULT ( STDMETHODCALLTYPE *GetName )( 
            __RPC__in IScriptEntry * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        DECLSPEC_XFGVIRT(IScriptEntry, SetName)
        HRESULT ( STDMETHODCALLTYPE *SetName )( 
            __RPC__in IScriptEntry * This,
            /* [in] */ __RPC__in LPCOLESTR psz);
        
        DECLSPEC_XFGVIRT(IScriptEntry, GetItemName)
        HRESULT ( STDMETHODCALLTYPE *GetItemName )( 
            __RPC__in IScriptEntry * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        DECLSPEC_XFGVIRT(IScriptEntry, SetItemName)
        HRESULT ( STDMETHODCALLTYPE *SetItemName )( 
            __RPC__in IScriptEntry * This,
            /* [in] */ __RPC__in LPCOLESTR psz);
        
        DECLSPEC_XFGVIRT(IScriptEntry, GetSignature)
        HRESULT ( STDMETHODCALLTYPE *GetSignature )( 
            __RPC__in IScriptEntry * This,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppti,
            /* [out] */ __RPC__out ULONG *piMethod);
        
        DECLSPEC_XFGVIRT(IScriptEntry, SetSignature)
        HRESULT ( STDMETHODCALLTYPE *SetSignature )( 
            __RPC__in IScriptEntry * This,
            /* [in] */ __RPC__in_opt ITypeInfo *pti,
            /* [in] */ ULONG iMethod);
        
        DECLSPEC_XFGVIRT(IScriptEntry, GetRange)
        HRESULT ( STDMETHODCALLTYPE *GetRange )( 
            __RPC__in IScriptEntry * This,
            /* [out] */ __RPC__out ULONG *pichMin,
            /* [out] */ __RPC__out ULONG *pcch);
        
        END_INTERFACE
    } IScriptEntryVtbl;

    interface IScriptEntry
    {
        CONST_VTBL struct IScriptEntryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IScriptEntry_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IScriptEntry_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IScriptEntry_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IScriptEntry_Alive(This)	\
    ( (This)->lpVtbl -> Alive(This) ) 

#define IScriptEntry_Delete(This)	\
    ( (This)->lpVtbl -> Delete(This) ) 

#define IScriptEntry_GetParent(This,ppsnParent)	\
    ( (This)->lpVtbl -> GetParent(This,ppsnParent) ) 

#define IScriptEntry_GetIndexInParent(This,pisn)	\
    ( (This)->lpVtbl -> GetIndexInParent(This,pisn) ) 

#define IScriptEntry_GetCookie(This,pdwCookie)	\
    ( (This)->lpVtbl -> GetCookie(This,pdwCookie) ) 

#define IScriptEntry_GetNumberOfChildren(This,pcsn)	\
    ( (This)->lpVtbl -> GetNumberOfChildren(This,pcsn) ) 

#define IScriptEntry_GetChild(This,isn,ppsn)	\
    ( (This)->lpVtbl -> GetChild(This,isn,ppsn) ) 

#define IScriptEntry_GetLanguage(This,pbstr)	\
    ( (This)->lpVtbl -> GetLanguage(This,pbstr) ) 

#define IScriptEntry_CreateChildEntry(This,isn,dwCookie,pszDelimiter,ppse)	\
    ( (This)->lpVtbl -> CreateChildEntry(This,isn,dwCookie,pszDelimiter,ppse) ) 

#define IScriptEntry_CreateChildHandler(This,pszDefaultName,prgpszNames,cpszNames,pszEvent,pszDelimiter,ptiSignature,iMethodSignature,isn,dwCookie,ppse)	\
    ( (This)->lpVtbl -> CreateChildHandler(This,pszDefaultName,prgpszNames,cpszNames,pszEvent,pszDelimiter,ptiSignature,iMethodSignature,isn,dwCookie,ppse) ) 


#define IScriptEntry_GetText(This,pbstr)	\
    ( (This)->lpVtbl -> GetText(This,pbstr) ) 

#define IScriptEntry_SetText(This,psz)	\
    ( (This)->lpVtbl -> SetText(This,psz) ) 

#define IScriptEntry_GetBody(This,pbstr)	\
    ( (This)->lpVtbl -> GetBody(This,pbstr) ) 

#define IScriptEntry_SetBody(This,psz)	\
    ( (This)->lpVtbl -> SetBody(This,psz) ) 

#define IScriptEntry_GetName(This,pbstr)	\
    ( (This)->lpVtbl -> GetName(This,pbstr) ) 

#define IScriptEntry_SetName(This,psz)	\
    ( (This)->lpVtbl -> SetName(This,psz) ) 

#define IScriptEntry_GetItemName(This,pbstr)	\
    ( (This)->lpVtbl -> GetItemName(This,pbstr) ) 

#define IScriptEntry_SetItemName(This,psz)	\
    ( (This)->lpVtbl -> SetItemName(This,psz) ) 

#define IScriptEntry_GetSignature(This,ppti,piMethod)	\
    ( (This)->lpVtbl -> GetSignature(This,ppti,piMethod) ) 

#define IScriptEntry_SetSignature(This,pti,iMethod)	\
    ( (This)->lpVtbl -> SetSignature(This,pti,iMethod) ) 

#define IScriptEntry_GetRange(This,pichMin,pcch)	\
    ( (This)->lpVtbl -> GetRange(This,pichMin,pcch) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IScriptEntry_INTERFACE_DEFINED__ */


#ifndef __IScriptScriptlet_INTERFACE_DEFINED__
#define __IScriptScriptlet_INTERFACE_DEFINED__

/* interface IScriptScriptlet */
/* [object][unique][uuid] */ 


EXTERN_C const IID IID_IScriptScriptlet;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0AEE2A96-BCBB-11d0-8C72-00C04FC2B085")
    IScriptScriptlet : public IScriptEntry
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetSubItemName( 
            /* [out] */ __RPC__deref_out_opt BSTR *pbstr) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetSubItemName( 
            /* [in] */ __RPC__in LPCOLESTR psz) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetEventName( 
            /* [out] */ __RPC__deref_out_opt BSTR *pbstr) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetEventName( 
            /* [in] */ __RPC__in LPCOLESTR psz) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSimpleEventName( 
            /* [out] */ __RPC__deref_out_opt BSTR *pbstr) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetSimpleEventName( 
            /* [in] */ __RPC__in LPCOLESTR psz) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IScriptScriptletVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IScriptScriptlet * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IScriptScriptlet * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IScriptScriptlet * This);
        
        DECLSPEC_XFGVIRT(IScriptNode, Alive)
        HRESULT ( STDMETHODCALLTYPE *Alive )( 
            __RPC__in IScriptScriptlet * This);
        
        DECLSPEC_XFGVIRT(IScriptNode, Delete)
        HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IScriptScriptlet * This);
        
        DECLSPEC_XFGVIRT(IScriptNode, GetParent)
        HRESULT ( STDMETHODCALLTYPE *GetParent )( 
            __RPC__in IScriptScriptlet * This,
            /* [out] */ __RPC__deref_out_opt IScriptNode **ppsnParent);
        
        DECLSPEC_XFGVIRT(IScriptNode, GetIndexInParent)
        HRESULT ( STDMETHODCALLTYPE *GetIndexInParent )( 
            __RPC__in IScriptScriptlet * This,
            /* [out] */ __RPC__out ULONG *pisn);
        
        DECLSPEC_XFGVIRT(IScriptNode, GetCookie)
        HRESULT ( STDMETHODCALLTYPE *GetCookie )( 
            __RPC__in IScriptScriptlet * This,
            /* [out] */ __RPC__out DWORD *pdwCookie);
        
        DECLSPEC_XFGVIRT(IScriptNode, GetNumberOfChildren)
        HRESULT ( STDMETHODCALLTYPE *GetNumberOfChildren )( 
            __RPC__in IScriptScriptlet * This,
            /* [out] */ __RPC__out ULONG *pcsn);
        
        DECLSPEC_XFGVIRT(IScriptNode, GetChild)
        HRESULT ( STDMETHODCALLTYPE *GetChild )( 
            __RPC__in IScriptScriptlet * This,
            /* [in] */ ULONG isn,
            /* [out] */ __RPC__deref_out_opt IScriptNode **ppsn);
        
        DECLSPEC_XFGVIRT(IScriptNode, GetLanguage)
        HRESULT ( STDMETHODCALLTYPE *GetLanguage )( 
            __RPC__in IScriptScriptlet * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        DECLSPEC_XFGVIRT(IScriptNode, CreateChildEntry)
        HRESULT ( STDMETHODCALLTYPE *CreateChildEntry )( 
            __RPC__in IScriptScriptlet * This,
            /* [in] */ ULONG isn,
            /* [in] */ DWORD dwCookie,
            /* [in] */ __RPC__in LPCOLESTR pszDelimiter,
            /* [out] */ __RPC__deref_out_opt IScriptEntry **ppse);
        
        DECLSPEC_XFGVIRT(IScriptNode, CreateChildHandler)
        HRESULT ( STDMETHODCALLTYPE *CreateChildHandler )( 
            __RPC__in IScriptScriptlet * This,
            /* [in] */ __RPC__in LPCOLESTR pszDefaultName,
            /* [size_is][in] */ __RPC__in_ecount_full(cpszNames) LPCOLESTR *prgpszNames,
            /* [in] */ ULONG cpszNames,
            /* [in] */ __RPC__in LPCOLESTR pszEvent,
            /* [in] */ __RPC__in LPCOLESTR pszDelimiter,
            /* [in] */ __RPC__in_opt ITypeInfo *ptiSignature,
            /* [in] */ ULONG iMethodSignature,
            /* [in] */ ULONG isn,
            /* [in] */ DWORD dwCookie,
            /* [out] */ __RPC__deref_out_opt IScriptEntry **ppse);
        
        DECLSPEC_XFGVIRT(IScriptEntry, GetText)
        HRESULT ( STDMETHODCALLTYPE *GetText )( 
            __RPC__in IScriptScriptlet * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        DECLSPEC_XFGVIRT(IScriptEntry, SetText)
        HRESULT ( STDMETHODCALLTYPE *SetText )( 
            __RPC__in IScriptScriptlet * This,
            /* [in] */ __RPC__in LPCOLESTR psz);
        
        DECLSPEC_XFGVIRT(IScriptEntry, GetBody)
        HRESULT ( STDMETHODCALLTYPE *GetBody )( 
            __RPC__in IScriptScriptlet * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        DECLSPEC_XFGVIRT(IScriptEntry, SetBody)
        HRESULT ( STDMETHODCALLTYPE *SetBody )( 
            __RPC__in IScriptScriptlet * This,
            /* [in] */ __RPC__in LPCOLESTR psz);
        
        DECLSPEC_XFGVIRT(IScriptEntry, GetName)
        HRESULT ( STDMETHODCALLTYPE *GetName )( 
            __RPC__in IScriptScriptlet * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        DECLSPEC_XFGVIRT(IScriptEntry, SetName)
        HRESULT ( STDMETHODCALLTYPE *SetName )( 
            __RPC__in IScriptScriptlet * This,
            /* [in] */ __RPC__in LPCOLESTR psz);
        
        DECLSPEC_XFGVIRT(IScriptEntry, GetItemName)
        HRESULT ( STDMETHODCALLTYPE *GetItemName )( 
            __RPC__in IScriptScriptlet * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        DECLSPEC_XFGVIRT(IScriptEntry, SetItemName)
        HRESULT ( STDMETHODCALLTYPE *SetItemName )( 
            __RPC__in IScriptScriptlet * This,
            /* [in] */ __RPC__in LPCOLESTR psz);
        
        DECLSPEC_XFGVIRT(IScriptEntry, GetSignature)
        HRESULT ( STDMETHODCALLTYPE *GetSignature )( 
            __RPC__in IScriptScriptlet * This,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppti,
            /* [out] */ __RPC__out ULONG *piMethod);
        
        DECLSPEC_XFGVIRT(IScriptEntry, SetSignature)
        HRESULT ( STDMETHODCALLTYPE *SetSignature )( 
            __RPC__in IScriptScriptlet * This,
            /* [in] */ __RPC__in_opt ITypeInfo *pti,
            /* [in] */ ULONG iMethod);
        
        DECLSPEC_XFGVIRT(IScriptEntry, GetRange)
        HRESULT ( STDMETHODCALLTYPE *GetRange )( 
            __RPC__in IScriptScriptlet * This,
            /* [out] */ __RPC__out ULONG *pichMin,
            /* [out] */ __RPC__out ULONG *pcch);
        
        DECLSPEC_XFGVIRT(IScriptScriptlet, GetSubItemName)
        HRESULT ( STDMETHODCALLTYPE *GetSubItemName )( 
            __RPC__in IScriptScriptlet * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        DECLSPEC_XFGVIRT(IScriptScriptlet, SetSubItemName)
        HRESULT ( STDMETHODCALLTYPE *SetSubItemName )( 
            __RPC__in IScriptScriptlet * This,
            /* [in] */ __RPC__in LPCOLESTR psz);
        
        DECLSPEC_XFGVIRT(IScriptScriptlet, GetEventName)
        HRESULT ( STDMETHODCALLTYPE *GetEventName )( 
            __RPC__in IScriptScriptlet * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        DECLSPEC_XFGVIRT(IScriptScriptlet, SetEventName)
        HRESULT ( STDMETHODCALLTYPE *SetEventName )( 
            __RPC__in IScriptScriptlet * This,
            /* [in] */ __RPC__in LPCOLESTR psz);
        
        DECLSPEC_XFGVIRT(IScriptScriptlet, GetSimpleEventName)
        HRESULT ( STDMETHODCALLTYPE *GetSimpleEventName )( 
            __RPC__in IScriptScriptlet * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        DECLSPEC_XFGVIRT(IScriptScriptlet, SetSimpleEventName)
        HRESULT ( STDMETHODCALLTYPE *SetSimpleEventName )( 
            __RPC__in IScriptScriptlet * This,
            /* [in] */ __RPC__in LPCOLESTR psz);
        
        END_INTERFACE
    } IScriptScriptletVtbl;

    interface IScriptScriptlet
    {
        CONST_VTBL struct IScriptScriptletVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IScriptScriptlet_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IScriptScriptlet_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IScriptScriptlet_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IScriptScriptlet_Alive(This)	\
    ( (This)->lpVtbl -> Alive(This) ) 

#define IScriptScriptlet_Delete(This)	\
    ( (This)->lpVtbl -> Delete(This) ) 

#define IScriptScriptlet_GetParent(This,ppsnParent)	\
    ( (This)->lpVtbl -> GetParent(This,ppsnParent) ) 

#define IScriptScriptlet_GetIndexInParent(This,pisn)	\
    ( (This)->lpVtbl -> GetIndexInParent(This,pisn) ) 

#define IScriptScriptlet_GetCookie(This,pdwCookie)	\
    ( (This)->lpVtbl -> GetCookie(This,pdwCookie) ) 

#define IScriptScriptlet_GetNumberOfChildren(This,pcsn)	\
    ( (This)->lpVtbl -> GetNumberOfChildren(This,pcsn) ) 

#define IScriptScriptlet_GetChild(This,isn,ppsn)	\
    ( (This)->lpVtbl -> GetChild(This,isn,ppsn) ) 

#define IScriptScriptlet_GetLanguage(This,pbstr)	\
    ( (This)->lpVtbl -> GetLanguage(This,pbstr) ) 

#define IScriptScriptlet_CreateChildEntry(This,isn,dwCookie,pszDelimiter,ppse)	\
    ( (This)->lpVtbl -> CreateChildEntry(This,isn,dwCookie,pszDelimiter,ppse) ) 

#define IScriptScriptlet_CreateChildHandler(This,pszDefaultName,prgpszNames,cpszNames,pszEvent,pszDelimiter,ptiSignature,iMethodSignature,isn,dwCookie,ppse)	\
    ( (This)->lpVtbl -> CreateChildHandler(This,pszDefaultName,prgpszNames,cpszNames,pszEvent,pszDelimiter,ptiSignature,iMethodSignature,isn,dwCookie,ppse) ) 


#define IScriptScriptlet_GetText(This,pbstr)	\
    ( (This)->lpVtbl -> GetText(This,pbstr) ) 

#define IScriptScriptlet_SetText(This,psz)	\
    ( (This)->lpVtbl -> SetText(This,psz) ) 

#define IScriptScriptlet_GetBody(This,pbstr)	\
    ( (This)->lpVtbl -> GetBody(This,pbstr) ) 

#define IScriptScriptlet_SetBody(This,psz)	\
    ( (This)->lpVtbl -> SetBody(This,psz) ) 

#define IScriptScriptlet_GetName(This,pbstr)	\
    ( (This)->lpVtbl -> GetName(This,pbstr) ) 

#define IScriptScriptlet_SetName(This,psz)	\
    ( (This)->lpVtbl -> SetName(This,psz) ) 

#define IScriptScriptlet_GetItemName(This,pbstr)	\
    ( (This)->lpVtbl -> GetItemName(This,pbstr) ) 

#define IScriptScriptlet_SetItemName(This,psz)	\
    ( (This)->lpVtbl -> SetItemName(This,psz) ) 

#define IScriptScriptlet_GetSignature(This,ppti,piMethod)	\
    ( (This)->lpVtbl -> GetSignature(This,ppti,piMethod) ) 

#define IScriptScriptlet_SetSignature(This,pti,iMethod)	\
    ( (This)->lpVtbl -> SetSignature(This,pti,iMethod) ) 

#define IScriptScriptlet_GetRange(This,pichMin,pcch)	\
    ( (This)->lpVtbl -> GetRange(This,pichMin,pcch) ) 


#define IScriptScriptlet_GetSubItemName(This,pbstr)	\
    ( (This)->lpVtbl -> GetSubItemName(This,pbstr) ) 

#define IScriptScriptlet_SetSubItemName(This,psz)	\
    ( (This)->lpVtbl -> SetSubItemName(This,psz) ) 

#define IScriptScriptlet_GetEventName(This,pbstr)	\
    ( (This)->lpVtbl -> GetEventName(This,pbstr) ) 

#define IScriptScriptlet_SetEventName(This,psz)	\
    ( (This)->lpVtbl -> SetEventName(This,psz) ) 

#define IScriptScriptlet_GetSimpleEventName(This,pbstr)	\
    ( (This)->lpVtbl -> GetSimpleEventName(This,pbstr) ) 

#define IScriptScriptlet_SetSimpleEventName(This,psz)	\
    ( (This)->lpVtbl -> SetSimpleEventName(This,psz) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IScriptScriptlet_INTERFACE_DEFINED__ */


#ifndef __IActiveScriptAuthor_INTERFACE_DEFINED__
#define __IActiveScriptAuthor_INTERFACE_DEFINED__

/* interface IActiveScriptAuthor */
/* [object][unique][uuid] */ 


EXTERN_C const IID IID_IActiveScriptAuthor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9C109DA0-7006-11d1-B36C-00A0C911E8B2")
    IActiveScriptAuthor : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AddNamedItem( 
            /* [in] */ __RPC__in LPCOLESTR pszName,
            /* [in] */ DWORD dwFlags,
            /* [in] */ __RPC__in_opt IDispatch *pdisp) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddScriptlet( 
            /* [in] */ __RPC__in LPCOLESTR pszDefaultName,
            /* [in] */ __RPC__in LPCOLESTR pszCode,
            /* [in] */ __RPC__in LPCOLESTR pszItemName,
            /* [in] */ __RPC__in LPCOLESTR pszSubItemName,
            /* [in] */ __RPC__in LPCOLESTR pszEventName,
            /* [in] */ __RPC__in LPCOLESTR pszDelimiter,
            /* [in] */ DWORD dwCookie,
            /* [in] */ DWORD dwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ParseScriptText( 
            /* [in] */ __RPC__in LPCOLESTR pszCode,
            /* [in] */ __RPC__in LPCOLESTR pszItemName,
            /* [in] */ __RPC__in LPCOLESTR pszDelimiter,
            /* [in] */ DWORD dwCookie,
            /* [in] */ DWORD dwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetScriptTextAttributes( 
            /* [size_is][in] */ __RPC__in_ecount_full(cch) LPCOLESTR pszCode,
            /* [in] */ ULONG cch,
            /* [in] */ __RPC__in LPCOLESTR pszDelimiter,
            /* [in] */ DWORD dwFlags,
            /* [size_is][out][in] */ __RPC__inout_ecount_full(cch) SOURCE_TEXT_ATTR *pattr) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetScriptletTextAttributes( 
            /* [size_is][in] */ __RPC__in_ecount_full(cch) LPCOLESTR pszCode,
            /* [in] */ ULONG cch,
            /* [in] */ __RPC__in LPCOLESTR pszDelimiter,
            /* [in] */ DWORD dwFlags,
            /* [size_is][out][in] */ __RPC__inout_ecount_full(cch) SOURCE_TEXT_ATTR *pattr) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRoot( 
            /* [out] */ __RPC__deref_out_opt IScriptNode **ppsp) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLanguageFlags( 
            /* [out] */ __RPC__out DWORD *pgrfasa) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetEventHandler( 
            /* [in] */ __RPC__in_opt IDispatch *pdisp,
            /* [in] */ __RPC__in LPCOLESTR pszItem,
            /* [in] */ __RPC__in LPCOLESTR pszSubItem,
            /* [in] */ __RPC__in LPCOLESTR pszEvent,
            /* [out] */ __RPC__deref_out_opt IScriptEntry **ppse) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveNamedItem( 
            /* [in] */ __RPC__in LPCOLESTR pszName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddTypeLib( 
            /* [in] */ __RPC__in REFGUID rguidTypeLib,
            /* [in] */ DWORD dwMajor,
            /* [in] */ DWORD dwMinor,
            /* [in] */ DWORD dwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveTypeLib( 
            /* [in] */ __RPC__in REFGUID rguidTypeLib,
            /* [in] */ DWORD dwMajor,
            /* [in] */ DWORD dwMinor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetChars( 
            /* [in] */ DWORD fRequestedList,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrChars) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetInfoFromContext( 
            /* [in] */ __RPC__in LPCOLESTR pszCode,
            /* [in] */ ULONG cchCode,
            /* [in] */ ULONG ichCurrentPosition,
            /* [in] */ DWORD dwListTypesRequested,
            /* [out] */ __RPC__out DWORD *pdwListTypesProvided,
            /* [out] */ __RPC__out ULONG *pichListAnchorPosition,
            /* [out] */ __RPC__out ULONG *pichFuncAnchorPosition,
            /* [out] */ __RPC__out MEMBERID *pmemid,
            /* [out] */ __RPC__out LONG *piCurrentParameter,
            /* [out] */ __RPC__deref_out_opt IUnknown **ppunk) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsCommitChar( 
            /* [in] */ OLECHAR ch,
            /* [out] */ __RPC__out BOOL *pfcommit) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IActiveScriptAuthorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IActiveScriptAuthor * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IActiveScriptAuthor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IActiveScriptAuthor * This);
        
        DECLSPEC_XFGVIRT(IActiveScriptAuthor, AddNamedItem)
        HRESULT ( STDMETHODCALLTYPE *AddNamedItem )( 
            __RPC__in IActiveScriptAuthor * This,
            /* [in] */ __RPC__in LPCOLESTR pszName,
            /* [in] */ DWORD dwFlags,
            /* [in] */ __RPC__in_opt IDispatch *pdisp);
        
        DECLSPEC_XFGVIRT(IActiveScriptAuthor, AddScriptlet)
        HRESULT ( STDMETHODCALLTYPE *AddScriptlet )( 
            __RPC__in IActiveScriptAuthor * This,
            /* [in] */ __RPC__in LPCOLESTR pszDefaultName,
            /* [in] */ __RPC__in LPCOLESTR pszCode,
            /* [in] */ __RPC__in LPCOLESTR pszItemName,
            /* [in] */ __RPC__in LPCOLESTR pszSubItemName,
            /* [in] */ __RPC__in LPCOLESTR pszEventName,
            /* [in] */ __RPC__in LPCOLESTR pszDelimiter,
            /* [in] */ DWORD dwCookie,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IActiveScriptAuthor, ParseScriptText)
        HRESULT ( STDMETHODCALLTYPE *ParseScriptText )( 
            __RPC__in IActiveScriptAuthor * This,
            /* [in] */ __RPC__in LPCOLESTR pszCode,
            /* [in] */ __RPC__in LPCOLESTR pszItemName,
            /* [in] */ __RPC__in LPCOLESTR pszDelimiter,
            /* [in] */ DWORD dwCookie,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IActiveScriptAuthor, GetScriptTextAttributes)
        HRESULT ( STDMETHODCALLTYPE *GetScriptTextAttributes )( 
            __RPC__in IActiveScriptAuthor * This,
            /* [size_is][in] */ __RPC__in_ecount_full(cch) LPCOLESTR pszCode,
            /* [in] */ ULONG cch,
            /* [in] */ __RPC__in LPCOLESTR pszDelimiter,
            /* [in] */ DWORD dwFlags,
            /* [size_is][out][in] */ __RPC__inout_ecount_full(cch) SOURCE_TEXT_ATTR *pattr);
        
        DECLSPEC_XFGVIRT(IActiveScriptAuthor, GetScriptletTextAttributes)
        HRESULT ( STDMETHODCALLTYPE *GetScriptletTextAttributes )( 
            __RPC__in IActiveScriptAuthor * This,
            /* [size_is][in] */ __RPC__in_ecount_full(cch) LPCOLESTR pszCode,
            /* [in] */ ULONG cch,
            /* [in] */ __RPC__in LPCOLESTR pszDelimiter,
            /* [in] */ DWORD dwFlags,
            /* [size_is][out][in] */ __RPC__inout_ecount_full(cch) SOURCE_TEXT_ATTR *pattr);
        
        DECLSPEC_XFGVIRT(IActiveScriptAuthor, GetRoot)
        HRESULT ( STDMETHODCALLTYPE *GetRoot )( 
            __RPC__in IActiveScriptAuthor * This,
            /* [out] */ __RPC__deref_out_opt IScriptNode **ppsp);
        
        DECLSPEC_XFGVIRT(IActiveScriptAuthor, GetLanguageFlags)
        HRESULT ( STDMETHODCALLTYPE *GetLanguageFlags )( 
            __RPC__in IActiveScriptAuthor * This,
            /* [out] */ __RPC__out DWORD *pgrfasa);
        
        DECLSPEC_XFGVIRT(IActiveScriptAuthor, GetEventHandler)
        HRESULT ( STDMETHODCALLTYPE *GetEventHandler )( 
            __RPC__in IActiveScriptAuthor * This,
            /* [in] */ __RPC__in_opt IDispatch *pdisp,
            /* [in] */ __RPC__in LPCOLESTR pszItem,
            /* [in] */ __RPC__in LPCOLESTR pszSubItem,
            /* [in] */ __RPC__in LPCOLESTR pszEvent,
            /* [out] */ __RPC__deref_out_opt IScriptEntry **ppse);
        
        DECLSPEC_XFGVIRT(IActiveScriptAuthor, RemoveNamedItem)
        HRESULT ( STDMETHODCALLTYPE *RemoveNamedItem )( 
            __RPC__in IActiveScriptAuthor * This,
            /* [in] */ __RPC__in LPCOLESTR pszName);
        
        DECLSPEC_XFGVIRT(IActiveScriptAuthor, AddTypeLib)
        HRESULT ( STDMETHODCALLTYPE *AddTypeLib )( 
            __RPC__in IActiveScriptAuthor * This,
            /* [in] */ __RPC__in REFGUID rguidTypeLib,
            /* [in] */ DWORD dwMajor,
            /* [in] */ DWORD dwMinor,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IActiveScriptAuthor, RemoveTypeLib)
        HRESULT ( STDMETHODCALLTYPE *RemoveTypeLib )( 
            __RPC__in IActiveScriptAuthor * This,
            /* [in] */ __RPC__in REFGUID rguidTypeLib,
            /* [in] */ DWORD dwMajor,
            /* [in] */ DWORD dwMinor);
        
        DECLSPEC_XFGVIRT(IActiveScriptAuthor, GetChars)
        HRESULT ( STDMETHODCALLTYPE *GetChars )( 
            __RPC__in IActiveScriptAuthor * This,
            /* [in] */ DWORD fRequestedList,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrChars);
        
        DECLSPEC_XFGVIRT(IActiveScriptAuthor, GetInfoFromContext)
        HRESULT ( STDMETHODCALLTYPE *GetInfoFromContext )( 
            __RPC__in IActiveScriptAuthor * This,
            /* [in] */ __RPC__in LPCOLESTR pszCode,
            /* [in] */ ULONG cchCode,
            /* [in] */ ULONG ichCurrentPosition,
            /* [in] */ DWORD dwListTypesRequested,
            /* [out] */ __RPC__out DWORD *pdwListTypesProvided,
            /* [out] */ __RPC__out ULONG *pichListAnchorPosition,
            /* [out] */ __RPC__out ULONG *pichFuncAnchorPosition,
            /* [out] */ __RPC__out MEMBERID *pmemid,
            /* [out] */ __RPC__out LONG *piCurrentParameter,
            /* [out] */ __RPC__deref_out_opt IUnknown **ppunk);
        
        DECLSPEC_XFGVIRT(IActiveScriptAuthor, IsCommitChar)
        HRESULT ( STDMETHODCALLTYPE *IsCommitChar )( 
            __RPC__in IActiveScriptAuthor * This,
            /* [in] */ OLECHAR ch,
            /* [out] */ __RPC__out BOOL *pfcommit);
        
        END_INTERFACE
    } IActiveScriptAuthorVtbl;

    interface IActiveScriptAuthor
    {
        CONST_VTBL struct IActiveScriptAuthorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IActiveScriptAuthor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IActiveScriptAuthor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IActiveScriptAuthor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IActiveScriptAuthor_AddNamedItem(This,pszName,dwFlags,pdisp)	\
    ( (This)->lpVtbl -> AddNamedItem(This,pszName,dwFlags,pdisp) ) 

#define IActiveScriptAuthor_AddScriptlet(This,pszDefaultName,pszCode,pszItemName,pszSubItemName,pszEventName,pszDelimiter,dwCookie,dwFlags)	\
    ( (This)->lpVtbl -> AddScriptlet(This,pszDefaultName,pszCode,pszItemName,pszSubItemName,pszEventName,pszDelimiter,dwCookie,dwFlags) ) 

#define IActiveScriptAuthor_ParseScriptText(This,pszCode,pszItemName,pszDelimiter,dwCookie,dwFlags)	\
    ( (This)->lpVtbl -> ParseScriptText(This,pszCode,pszItemName,pszDelimiter,dwCookie,dwFlags) ) 

#define IActiveScriptAuthor_GetScriptTextAttributes(This,pszCode,cch,pszDelimiter,dwFlags,pattr)	\
    ( (This)->lpVtbl -> GetScriptTextAttributes(This,pszCode,cch,pszDelimiter,dwFlags,pattr) ) 

#define IActiveScriptAuthor_GetScriptletTextAttributes(This,pszCode,cch,pszDelimiter,dwFlags,pattr)	\
    ( (This)->lpVtbl -> GetScriptletTextAttributes(This,pszCode,cch,pszDelimiter,dwFlags,pattr) ) 

#define IActiveScriptAuthor_GetRoot(This,ppsp)	\
    ( (This)->lpVtbl -> GetRoot(This,ppsp) ) 

#define IActiveScriptAuthor_GetLanguageFlags(This,pgrfasa)	\
    ( (This)->lpVtbl -> GetLanguageFlags(This,pgrfasa) ) 

#define IActiveScriptAuthor_GetEventHandler(This,pdisp,pszItem,pszSubItem,pszEvent,ppse)	\
    ( (This)->lpVtbl -> GetEventHandler(This,pdisp,pszItem,pszSubItem,pszEvent,ppse) ) 

#define IActiveScriptAuthor_RemoveNamedItem(This,pszName)	\
    ( (This)->lpVtbl -> RemoveNamedItem(This,pszName) ) 

#define IActiveScriptAuthor_AddTypeLib(This,rguidTypeLib,dwMajor,dwMinor,dwFlags)	\
    ( (This)->lpVtbl -> AddTypeLib(This,rguidTypeLib,dwMajor,dwMinor,dwFlags) ) 

#define IActiveScriptAuthor_RemoveTypeLib(This,rguidTypeLib,dwMajor,dwMinor)	\
    ( (This)->lpVtbl -> RemoveTypeLib(This,rguidTypeLib,dwMajor,dwMinor) ) 

#define IActiveScriptAuthor_GetChars(This,fRequestedList,pbstrChars)	\
    ( (This)->lpVtbl -> GetChars(This,fRequestedList,pbstrChars) ) 

#define IActiveScriptAuthor_GetInfoFromContext(This,pszCode,cchCode,ichCurrentPosition,dwListTypesRequested,pdwListTypesProvided,pichListAnchorPosition,pichFuncAnchorPosition,pmemid,piCurrentParameter,ppunk)	\
    ( (This)->lpVtbl -> GetInfoFromContext(This,pszCode,cchCode,ichCurrentPosition,dwListTypesRequested,pdwListTypesProvided,pichListAnchorPosition,pichFuncAnchorPosition,pmemid,piCurrentParameter,ppunk) ) 

#define IActiveScriptAuthor_IsCommitChar(This,ch,pfcommit)	\
    ( (This)->lpVtbl -> IsCommitChar(This,ch,pfcommit) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IActiveScriptAuthor_INTERFACE_DEFINED__ */


#ifndef __IActiveScriptAuthorProcedure_INTERFACE_DEFINED__
#define __IActiveScriptAuthorProcedure_INTERFACE_DEFINED__

/* interface IActiveScriptAuthorProcedure */
/* [object][unique][uuid] */ 


EXTERN_C const IID IID_IActiveScriptAuthorProcedure;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7E2D4B70-BD9A-11d0-9336-00A0C90DCAA9")
    IActiveScriptAuthorProcedure : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ParseProcedureText( 
            /* [in] */ __RPC__in LPCOLESTR pszCode,
            /* [in] */ __RPC__in LPCOLESTR pszFormalParams,
            /* [in] */ __RPC__in LPCOLESTR pszProcedureName,
            /* [in] */ __RPC__in LPCOLESTR pszItemName,
            /* [in] */ __RPC__in LPCOLESTR pszDelimiter,
            /* [in] */ DWORD dwCookie,
            /* [in] */ DWORD dwFlags,
            /* [in] */ __RPC__in_opt IDispatch *pdispFor) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IActiveScriptAuthorProcedureVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IActiveScriptAuthorProcedure * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IActiveScriptAuthorProcedure * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IActiveScriptAuthorProcedure * This);
        
        DECLSPEC_XFGVIRT(IActiveScriptAuthorProcedure, ParseProcedureText)
        HRESULT ( STDMETHODCALLTYPE *ParseProcedureText )( 
            __RPC__in IActiveScriptAuthorProcedure * This,
            /* [in] */ __RPC__in LPCOLESTR pszCode,
            /* [in] */ __RPC__in LPCOLESTR pszFormalParams,
            /* [in] */ __RPC__in LPCOLESTR pszProcedureName,
            /* [in] */ __RPC__in LPCOLESTR pszItemName,
            /* [in] */ __RPC__in LPCOLESTR pszDelimiter,
            /* [in] */ DWORD dwCookie,
            /* [in] */ DWORD dwFlags,
            /* [in] */ __RPC__in_opt IDispatch *pdispFor);
        
        END_INTERFACE
    } IActiveScriptAuthorProcedureVtbl;

    interface IActiveScriptAuthorProcedure
    {
        CONST_VTBL struct IActiveScriptAuthorProcedureVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IActiveScriptAuthorProcedure_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IActiveScriptAuthorProcedure_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IActiveScriptAuthorProcedure_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IActiveScriptAuthorProcedure_ParseProcedureText(This,pszCode,pszFormalParams,pszProcedureName,pszItemName,pszDelimiter,dwCookie,dwFlags,pdispFor)	\
    ( (This)->lpVtbl -> ParseProcedureText(This,pszCode,pszFormalParams,pszProcedureName,pszItemName,pszDelimiter,dwCookie,dwFlags,pdispFor) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IActiveScriptAuthorProcedure_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_activaut_0000_0005 */
/* [local] */ 


#endif  // __ActivAut_h

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_activaut_0000_0005_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_activaut_0000_0005_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  BSTR_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  BSTR_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree64(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


