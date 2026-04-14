/********************************************************
*                                                       *
*   Copyright (C) Microsoft. All rights reserved.       *
*                                                       *
********************************************************/

//--------------------------------------------------------------------
// File:		ADOCTINT.h
//
// Contents:	ADO Interface header
//
//--------------------------------------------------------------------
#ifndef _ADOCTINT_H_
#define _ADOCTINT_H_

#ifndef _INC_TCHAR
#include <tchar.h>
#endif

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
#ifndef __adocat_h__
#define __adocat_h__
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
#ifndef ___ADOCollection_FWD_DEFINED__
#define ___ADOCollection_FWD_DEFINED__
typedef interface _ADOADOCollection _ADOCollection;
#endif 	/* ___ADOCollection_FWD_DEFINED__ */
#ifndef ___ADODynaCollection_FWD_DEFINED__
#define ___ADODynaCollection_FWD_DEFINED__
typedef interface _ADODynaADOCollection _ADODynaCollection;
#endif 	/* ___ADODynaCollection_FWD_DEFINED__ */
#ifndef ___Catalog_FWD_DEFINED__
#define ___Catalog_FWD_DEFINED__
typedef interface _ADOCatalog _Catalog;
#endif 	/* ___Catalog_FWD_DEFINED__ */
#ifndef ___Table_FWD_DEFINED__
#define ___Table_FWD_DEFINED__
typedef interface _ADOTable _Table;
#endif 	/* ___Table_FWD_DEFINED__ */
#ifndef ___Group25_FWD_DEFINED__
#define ___Group25_FWD_DEFINED__
typedef interface _Group25 _Group25;
#endif 	/* ___Group25_FWD_DEFINED__ */
#ifndef ___Group_FWD_DEFINED__
#define ___Group_FWD_DEFINED__
typedef interface _ADOGroup _Group;
#endif 	/* ___Group_FWD_DEFINED__ */
#ifndef ___User25_FWD_DEFINED__
#define ___User25_FWD_DEFINED__
typedef interface _User25 _User25;
#endif 	/* ___User25_FWD_DEFINED__ */
#ifndef ___User_FWD_DEFINED__
#define ___User_FWD_DEFINED__
typedef interface _ADOUser _User;
#endif 	/* ___User_FWD_DEFINED__ */
#ifndef ___Column_FWD_DEFINED__
#define ___Column_FWD_DEFINED__
typedef interface _ADOColumn _Column;
#endif 	/* ___Column_FWD_DEFINED__ */
#ifndef ___Index_FWD_DEFINED__
#define ___Index_FWD_DEFINED__
typedef interface _ADOIndex _Index;
#endif 	/* ___Index_FWD_DEFINED__ */
#ifndef ___Key_FWD_DEFINED__
#define ___Key_FWD_DEFINED__
typedef interface _ADOKey _Key;
#endif 	/* ___Key_FWD_DEFINED__ */
#ifndef __View_FWD_DEFINED__
#define __View_FWD_DEFINED__
typedef interface ADOView View;
#endif 	/* __View_FWD_DEFINED__ */
#ifndef __Procedure_FWD_DEFINED__
#define __Procedure_FWD_DEFINED__
typedef interface ADOProcedure Procedure;
#endif 	/* __Procedure_FWD_DEFINED__ */
#ifndef __Catalog_FWD_DEFINED__
#define __Catalog_FWD_DEFINED__
#ifdef __cplusplus
typedef class ADOCatalog Catalog;
#else
typedef struct ADOCatalog Catalog;
#endif /* __cplusplus */
#endif 	/* __Catalog_FWD_DEFINED__ */
#ifndef __Table_FWD_DEFINED__
#define __Table_FWD_DEFINED__
#ifdef __cplusplus
typedef class ADOTable Table;
#else
typedef struct ADOTable Table;
#endif /* __cplusplus */
#endif 	/* __Table_FWD_DEFINED__ */
#ifndef __Property_FWD_DEFINED__
#define __Property_FWD_DEFINED__
typedef interface ADOProperty Property;
#endif 	/* __Property_FWD_DEFINED__ */
#ifndef __Group_FWD_DEFINED__
#define __Group_FWD_DEFINED__
#ifdef __cplusplus
typedef class ADOGroup Group;
#else
typedef struct ADOGroup Group;
#endif /* __cplusplus */
#endif 	/* __Group_FWD_DEFINED__ */
#ifndef __User_FWD_DEFINED__
#define __User_FWD_DEFINED__
#ifdef __cplusplus
typedef class ADOUser User;
#else
typedef struct ADOUser User;
#endif /* __cplusplus */
#endif 	/* __User_FWD_DEFINED__ */
#ifndef __Column_FWD_DEFINED__
#define __Column_FWD_DEFINED__
#ifdef __cplusplus
typedef class ADOColumn Column;
#else
typedef struct ADOColumn Column;
#endif /* __cplusplus */
#endif 	/* __Column_FWD_DEFINED__ */
#ifndef __Index_FWD_DEFINED__
#define __Index_FWD_DEFINED__
#ifdef __cplusplus
typedef class ADOIndex Index;
#else
typedef struct ADOIndex Index;
#endif /* __cplusplus */
#endif 	/* __Index_FWD_DEFINED__ */
#ifndef __Key_FWD_DEFINED__
#define __Key_FWD_DEFINED__
#ifdef __cplusplus
typedef class ADOKey Key;
#else
typedef struct ADOKey Key;
#endif /* __cplusplus */
#endif 	/* __Key_FWD_DEFINED__ */
#ifndef __Tables_FWD_DEFINED__
#define __Tables_FWD_DEFINED__
typedef interface ADOTables Tables;
#endif 	/* __Tables_FWD_DEFINED__ */
#ifndef __Columns_FWD_DEFINED__
#define __Columns_FWD_DEFINED__
typedef interface ADOColumns Columns;
#endif 	/* __Columns_FWD_DEFINED__ */
#ifndef __Procedures_FWD_DEFINED__
#define __Procedures_FWD_DEFINED__
typedef interface ADOProcedures Procedures;
#endif 	/* __Procedures_FWD_DEFINED__ */
#ifndef __Views_FWD_DEFINED__
#define __Views_FWD_DEFINED__
typedef interface ADOViews Views;
#endif 	/* __Views_FWD_DEFINED__ */
#ifndef __Indexes_FWD_DEFINED__
#define __Indexes_FWD_DEFINED__
typedef interface ADOIndexes Indexes;
#endif 	/* __Indexes_FWD_DEFINED__ */
#ifndef __Keys_FWD_DEFINED__
#define __Keys_FWD_DEFINED__
typedef interface ADOKeys Keys;
#endif 	/* __Keys_FWD_DEFINED__ */
#ifndef __Users_FWD_DEFINED__
#define __Users_FWD_DEFINED__
typedef interface ADOUsers Users;
#endif 	/* __Users_FWD_DEFINED__ */
#ifndef __Groups_FWD_DEFINED__
#define __Groups_FWD_DEFINED__
typedef interface ADOGroups Groups;
#endif 	/* __Groups_FWD_DEFINED__ */
#ifndef __Properties_FWD_DEFINED__
#define __Properties_FWD_DEFINED__
typedef interface ADOProperties Properties;
#endif 	/* __Properties_FWD_DEFINED__ */
/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"
#ifdef __cplusplus
extern "C"{
#endif 
/* interface __MIDL_itf_adocat_0000_0000 */
/* [local] */ 
#pragma warning(push) 
#pragma warning(disable:4091) 
typedef /* [helpcontext] */ 
enum RuleEnum
    {
        adRINone	= 0,
        adRICascade	= 1,
        adRISetNull	= 2,
        adRISetDefault	= 3
    } 	RuleEnum;
typedef /* [helpcontext] */ 
enum KeyTypeEnum
    {
        adKeyPrimary	= 1,
        adKeyForeign	= 2,
        adKeyUnique	= 3
    } 	KeyTypeEnum;
typedef /* [helpcontext] */ 
enum ActionEnum
    {
        adAccessGrant	= 1,
        adAccessSet	= 2,
        adAccessDeny	= 3,
        adAccessRevoke	= 4
    } 	ActionEnum;
typedef /* [helpcontext] */ 
enum ColumnAttributesEnum
    {
        adColFixed	= 1,
        adColNullable	= 2
    } 	ColumnAttributesEnum;
typedef /* [helpcontext] */ 
enum SortOrderEnum
    {
        adSortAscending	= 1,
        adSortDescending	= 2
    } 	SortOrderEnum;
typedef /* [helpcontext] */ 
enum RightsEnum
    {
        adRightNone	= 0L,
        adRightDrop	= 0x100L,
        adRightExclusive	= 0x200L,
        adRightReadDesign	= 0x400L,
        adRightWriteDesign	= 0x800L,
        adRightWithGrant	= 0x1000L,
        adRightReference	= 0x2000L,
        adRightCreate	= 0x4000L,
        adRightInsert	= 0x8000L,
        adRightDelete	= 0x10000L,
        adRightReadPermissions	= 0x20000L,
        adRightWritePermissions	= 0x40000L,
        adRightWriteOwner	= 0x80000L,
        adRightMaximumAllowed	= 0x2000000L,
        adRightFull	= 0x10000000L,
        adRightExecute	= 0x20000000L,
        adRightUpdate	= 0x40000000L,
        adRightRead	= 0x80000000L
    } 	RightsEnum;
typedef /* [helpcontext] */ 
#ifdef _ADOINT_H_  //Avoid conflicting with ADO def
    class dummy dummy;
#else
enum DataTypeEnum
    {
        adEmpty	= 0,
        adTinyInt	= 16,
        adSmallInt	= 2,
        adInteger	= 3,
        adBigInt	= 20,
        adUnsignedTinyInt	= 17,
        adUnsignedSmallInt	= 18,
        adUnsignedInt	= 19,
        adUnsignedBigInt	= 21,
        adSingle	= 4,
        adDouble	= 5,
        adCurrency	= 6,
        adDecimal	= 14,
        adNumeric	= 131,
        adBoolean	= 11,
        adError	= 10,
        adUserDefined	= 132,
        adVariant	= 12,
        adIDispatch	= 9,
        adIUnknown	= 13,
        adGUID	= 72,
        adDate	= 7,
        adDBDate	= 133,
        adDBTime	= 134,
        adDBTimeStamp	= 135,
        adBSTR	= 8,
        adChar	= 129,
        adVarChar	= 200,
        adLongVarChar	= 201,
        adWChar	= 130,
        adVarWChar	= 202,
        adLongVarWChar	= 203,
        adBinary	= 128,
        adVarBinary	= 204,
        adLongVarBinary	= 205,
        adChapter	= 136,
        adFileTime	= 64,
        adPropVariant	= 138,
        adVarNumeric	= 139
    } 	DataTypeEnum;
#endif //ifdef _ADOINT.H_
typedef /* [helpcontext] */ 
enum AllowNullsEnum
    {
        adIndexNullsAllow	= 0,
        adIndexNullsDisallow	= 1,
        adIndexNullsIgnore	= 2,
        adIndexNullsIgnoreAny	= 4
    } 	AllowNullsEnum;
typedef /* [helpcontext] */ 
enum ObjectTypeEnum
    {
        adPermObjProviderSpecific	= -1,
        adPermObjTable	= 1,
        adPermObjColumn	= 2,
        adPermObjDatabase	= 3,
        adPermObjProcedure	= 4,
        adPermObjView	= 5
    } 	ObjectTypeEnum;
typedef /* [helpcontext] */ 
enum InheritTypeEnum
    {
        adInheritNone	= 0,
        adInheritObjects	= 1,
        adInheritContainers	= 2,
        adInheritBoth	= 3,
        adInheritNoPropogate	= 4
    } 	InheritTypeEnum;
extern RPC_IF_HANDLE __MIDL_itf_adocat_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_adocat_0000_0000_v0_0_s_ifspec;
#ifndef __ADOX_LIBRARY_DEFINED__
#define __ADOX_LIBRARY_DEFINED__
/* library ADOX */
/* [helpstring][version][uuid] */ 
EXTERN_C const IID LIBID_ADOX;
#ifndef ___ADOCollection_INTERFACE_DEFINED__
#define ___ADOCollection_INTERFACE_DEFINED__
/* interface _ADOADOCollection */
/* [object][uuid][nonextensible][dual] */ 
EXTERN_C const IID IID__ADOCollection;
#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000512-0000-0010-8000-00AA006D2EA4")
    _ADOADOCollection : public IDispatch
    {
    public:
        virtual /* [helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *c) = 0;
        
        virtual /* [id][restricted] */ HRESULT STDMETHODCALLTYPE _NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppvObject) = 0;
        
        virtual /* [helpcontext] */ HRESULT STDMETHODCALLTYPE Refresh( void) = 0;
        
    };
    
    
#else 	/* C style interface */
    typedef struct _ADOCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in _ADOADOCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in _ADOADOCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in _ADOADOCollection * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in _ADOADOCollection * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in _ADOADOCollection * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in _ADOADOCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            _ADOADOCollection * This,
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
        
        DECLSPEC_XFGVIRT(_Collection, get_Count)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in _ADOADOCollection * This,
            /* [retval][out] */ __RPC__out long *c);
        
        DECLSPEC_XFGVIRT(_ADOCollection, _NewEnum)
        /* [id][restricted] */ HRESULT ( STDMETHODCALLTYPE *_NewEnum )( 
            __RPC__in _ADOADOCollection * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppvObject);
        
        DECLSPEC_XFGVIRT(_ADOCollection, Refresh)
        /* [helpcontext] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in _ADOADOCollection * This);
        
        END_INTERFACE
    } _ADOCollectionVtbl;
    interface _ADOCollection
    {
        CONST_VTBL struct _ADOCollectionVtbl *lpVtbl;
    };
    
#ifdef COBJMACROS
#define _ADOCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define _ADOCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define _ADOCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 
#define _ADOCollection_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 
#define _ADOCollection_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 
#define _ADOCollection_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 
#define _ADOCollection_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 
#define _Collection_get_Count(This,c)	\
    ( (This)->lpVtbl -> get_Count(This,c) ) 
#define _ADOCollection__NewEnum(This,ppvObject)	\
    ( (This)->lpVtbl -> _NewEnum(This,ppvObject) ) 
#define _ADOCollection_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 
#endif /* COBJMACROS */
#endif 	/* C style interface */
#endif 	/* ___ADOCollection_INTERFACE_DEFINED__ */
#ifndef ___ADODynaCollection_INTERFACE_DEFINED__
#define ___ADODynaCollection_INTERFACE_DEFINED__
/* interface _ADODynaADOCollection */
/* [object][uuid][nonextensible][dual] */ 
EXTERN_C const IID IID__ADODynaCollection;
#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000513-0000-0010-8000-00AA006D2EA4")
    _ADODynaADOCollection : public _ADOCollection
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Append( 
            /* [in] */ __RPC__in_opt IDispatch *Object) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Delete( 
            /* [in] */ VARIANT Item) = 0;
        
    };
    
    
#else 	/* C style interface */
    typedef struct _ADODynaCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in _ADODynaADOCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in _ADODynaADOCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in _ADODynaADOCollection * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in _ADODynaADOCollection * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in _ADODynaADOCollection * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in _ADODynaADOCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            _ADODynaADOCollection * This,
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
        
        DECLSPEC_XFGVIRT(_Collection, get_Count)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in _ADODynaADOCollection * This,
            /* [retval][out] */ __RPC__out long *c);
        
        DECLSPEC_XFGVIRT(_ADOCollection, _NewEnum)
        /* [id][restricted] */ HRESULT ( STDMETHODCALLTYPE *_NewEnum )( 
            __RPC__in _ADODynaADOCollection * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppvObject);
        
        DECLSPEC_XFGVIRT(_ADOCollection, Refresh)
        /* [helpcontext] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in _ADODynaADOCollection * This);
        
        DECLSPEC_XFGVIRT(_ADODynaCollection, Append)
        HRESULT ( STDMETHODCALLTYPE *Append )( 
            __RPC__in _ADODynaADOCollection * This,
            /* [in] */ __RPC__in_opt IDispatch *Object);
        
        DECLSPEC_XFGVIRT(_ADODynaCollection, Delete)
        HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in _ADODynaADOCollection * This,
            /* [in] */ VARIANT Item);
        
        END_INTERFACE
    } _ADODynaCollectionVtbl;
    interface _ADODynaCollection
    {
        CONST_VTBL struct _ADODynaCollectionVtbl *lpVtbl;
    };
    
#ifdef COBJMACROS
#define _ADODynaCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define _ADODynaCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define _ADODynaCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 
#define _ADODynaCollection_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 
#define _ADODynaCollection_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 
#define _ADODynaCollection_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 
#define _ADODynaCollection_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 
#define _DynaCollection_get_Count(This,c)	\
    ( (This)->lpVtbl -> get_Count(This,c) ) 
#define _ADODynaCollection__NewEnum(This,ppvObject)	\
    ( (This)->lpVtbl -> _NewEnum(This,ppvObject) ) 
#define _ADODynaCollection_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 
#define _ADODynaCollection_Append(This,Object)	\
    ( (This)->lpVtbl -> Append(This,Object) ) 
#define _ADODynaCollection_Delete(This,Item)	\
    ( (This)->lpVtbl -> Delete(This,Item) ) 
#endif /* COBJMACROS */
#endif 	/* C style interface */
#endif 	/* ___ADODynaCollection_INTERFACE_DEFINED__ */
#ifndef ___Catalog_INTERFACE_DEFINED__
#define ___Catalog_INTERFACE_DEFINED__
/* interface _ADOCatalog */
/* [helpcontext][unique][dual][uuid][nonextensible][object] */ 
EXTERN_C const IID IID__Catalog;
#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000603-0000-0010-8000-00AA006D2EA4")
    _ADOCatalog : public IDispatch
    {
    public:
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_Tables( 
            /* [retval][out] */ __RPC__deref_out_opt ADOTables **ppvObject) = 0;
        
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_ActiveConnection( 
            /* [retval][out] */ __RPC__out VARIANT *pVal) = 0;
        
        virtual /* [helpcontext][id][propput] */ HRESULT STDMETHODCALLTYPE put_ActiveConnection( 
            /* [in] */ VARIANT newVal) = 0;
        
        virtual /* [helpcontext][id][propputref] */ HRESULT STDMETHODCALLTYPE putref_ActiveConnection( 
            /* [in] */ __RPC__in_opt IDispatch *pCon) = 0;
        
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_Procedures( 
            /* [retval][out] */ __RPC__deref_out_opt ADOProcedures **ppvObject) = 0;
        
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_Views( 
            /* [retval][out] */ __RPC__deref_out_opt ADOViews **ppvObject) = 0;
        
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_Groups( 
            /* [retval][out] */ __RPC__deref_out_opt ADOGroups **ppvObject) = 0;
        
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_Users( 
            /* [retval][out] */ __RPC__deref_out_opt ADOUsers **ppvObject) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE Create( 
            /* [in] */ __RPC__in BSTR ConnectString,
            /* [retval][out] */ __RPC__out VARIANT *Connection) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE GetObjectOwner( 
            /* [in] */ __RPC__in BSTR ObjectName,
            /* [in] */ ObjectTypeEnum ObjectType,
            /* [optional][in] */ VARIANT ObjectTypeId,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *OwnerName) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE SetObjectOwner( 
            /* [in] */ __RPC__in BSTR ObjectName,
            /* [in] */ ObjectTypeEnum ObjectType,
            /* [in] */ __RPC__in BSTR UserName,
            /* [optional][in] */ VARIANT ObjectTypeId) = 0;
        
    };
    
    
#else 	/* C style interface */
    typedef struct _CatalogVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in _ADOCatalog * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in _ADOCatalog * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in _ADOCatalog * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in _ADOCatalog * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in _ADOCatalog * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in _ADOCatalog * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            _ADOCatalog * This,
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
        
        DECLSPEC_XFGVIRT(_Catalog, get_Tables)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Tables )( 
            __RPC__in _ADOCatalog * This,
            /* [retval][out] */ __RPC__deref_out_opt ADOTables **ppvObject);
        
        DECLSPEC_XFGVIRT(_Catalog, get_ActiveConnection)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ActiveConnection )( 
            __RPC__in _ADOCatalog * This,
            /* [retval][out] */ __RPC__out VARIANT *pVal);
        
        DECLSPEC_XFGVIRT(_Catalog, put_ActiveConnection)
        /* [helpcontext][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ActiveConnection )( 
            __RPC__in _ADOCatalog * This,
            /* [in] */ VARIANT newVal);
        
        DECLSPEC_XFGVIRT(_Catalog, putref_ActiveConnection)
        /* [helpcontext][id][propputref] */ HRESULT ( STDMETHODCALLTYPE *putref_ActiveConnection )( 
            __RPC__in _ADOCatalog * This,
            /* [in] */ __RPC__in_opt IDispatch *pCon);
        
        DECLSPEC_XFGVIRT(_Catalog, get_Procedures)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Procedures )( 
            __RPC__in _ADOCatalog * This,
            /* [retval][out] */ __RPC__deref_out_opt ADOProcedures **ppvObject);
        
        DECLSPEC_XFGVIRT(_Catalog, get_Views)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Views )( 
            __RPC__in _ADOCatalog * This,
            /* [retval][out] */ __RPC__deref_out_opt ADOViews **ppvObject);
        
        DECLSPEC_XFGVIRT(_Catalog, get_Groups)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Groups )( 
            __RPC__in _ADOCatalog * This,
            /* [retval][out] */ __RPC__deref_out_opt ADOGroups **ppvObject);
        
        DECLSPEC_XFGVIRT(_Catalog, get_Users)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Users )( 
            __RPC__in _ADOCatalog * This,
            /* [retval][out] */ __RPC__deref_out_opt ADOUsers **ppvObject);
        
        DECLSPEC_XFGVIRT(_Catalog, Create)
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *Create )( 
            __RPC__in _ADOCatalog * This,
            /* [in] */ __RPC__in BSTR ConnectString,
            /* [retval][out] */ __RPC__out VARIANT *Connection);
        
        DECLSPEC_XFGVIRT(_Catalog, GetObjectOwner)
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *GetObjectOwner )( 
            __RPC__in _ADOCatalog * This,
            /* [in] */ __RPC__in BSTR ObjectName,
            /* [in] */ ObjectTypeEnum ObjectType,
            /* [optional][in] */ VARIANT ObjectTypeId,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *OwnerName);
        
        DECLSPEC_XFGVIRT(_Catalog, SetObjectOwner)
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *SetObjectOwner )( 
            __RPC__in _ADOCatalog * This,
            /* [in] */ __RPC__in BSTR ObjectName,
            /* [in] */ ObjectTypeEnum ObjectType,
            /* [in] */ __RPC__in BSTR UserName,
            /* [optional][in] */ VARIANT ObjectTypeId);
        
        END_INTERFACE
    } _CatalogVtbl;
    interface _Catalog
    {
        CONST_VTBL struct _CatalogVtbl *lpVtbl;
    };
    
#ifdef COBJMACROS
#define _Catalog_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define _Catalog_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define _Catalog_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 
#define _Catalog_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 
#define _Catalog_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 
#define _Catalog_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 
#define _Catalog_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 
#define _Catalog_get_Tables(This,ppvObject)	\
    ( (This)->lpVtbl -> get_Tables(This,ppvObject) ) 
#define _Catalog_get_ActiveConnection(This,pVal)	\
    ( (This)->lpVtbl -> get_ActiveConnection(This,pVal) ) 
#define _Catalog_put_ActiveConnection(This,newVal)	\
    ( (This)->lpVtbl -> put_ActiveConnection(This,newVal) ) 
#define _Catalog_putref_ActiveConnection(This,pCon)	\
    ( (This)->lpVtbl -> putref_ActiveConnection(This,pCon) ) 
#define _Catalog_get_Procedures(This,ppvObject)	\
    ( (This)->lpVtbl -> get_Procedures(This,ppvObject) ) 
#define _Catalog_get_Views(This,ppvObject)	\
    ( (This)->lpVtbl -> get_Views(This,ppvObject) ) 
#define _Catalog_get_Groups(This,ppvObject)	\
    ( (This)->lpVtbl -> get_Groups(This,ppvObject) ) 
#define _Catalog_get_Users(This,ppvObject)	\
    ( (This)->lpVtbl -> get_Users(This,ppvObject) ) 
#define _Catalog_Create(This,ConnectString,Connection)	\
    ( (This)->lpVtbl -> Create(This,ConnectString,Connection) ) 
#define _Catalog_GetObjectOwner(This,ObjectName,ObjectType,ObjectTypeId,OwnerName)	\
    ( (This)->lpVtbl -> GetObjectOwner(This,ObjectName,ObjectType,ObjectTypeId,OwnerName) ) 
#define _Catalog_SetObjectOwner(This,ObjectName,ObjectType,UserName,ObjectTypeId)	\
    ( (This)->lpVtbl -> SetObjectOwner(This,ObjectName,ObjectType,UserName,ObjectTypeId) ) 
#endif /* COBJMACROS */
#endif 	/* C style interface */
#endif 	/* ___Catalog_INTERFACE_DEFINED__ */
#ifndef ___Table_INTERFACE_DEFINED__
#define ___Table_INTERFACE_DEFINED__
/* interface _ADOTable */
/* [helpcontext][unique][dual][uuid][nonextensible][object] */ 
EXTERN_C const IID IID__Table;
#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000610-0000-0010-8000-00AA006D2EA4")
    _ADOTable : public IDispatch
    {
    public:
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_Columns( 
            /* [retval][out] */ __RPC__deref_out_opt ADOColumns **ppvObject) = 0;
        
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpcontext][id][propput] */ HRESULT STDMETHODCALLTYPE put_Name( 
            /* [in] */ __RPC__in BSTR newVal) = 0;
        
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_Type( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_Indexes( 
            /* [retval][out] */ __RPC__deref_out_opt ADOIndexes **ppvObject) = 0;
        
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_Keys( 
            /* [retval][out] */ __RPC__deref_out_opt ADOKeys **ppvObject) = 0;
        
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_Properties( 
            /* [retval][out] */ __RPC__deref_out_opt ADOProperties **ppvObject) = 0;
        
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_DateCreated( 
            /* [retval][out] */ __RPC__out VARIANT *pVal) = 0;
        
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_DateModified( 
            /* [retval][out] */ __RPC__out VARIANT *pVal) = 0;
        
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_ParentCatalog( 
            /* [retval][out] */ __RPC__deref_out_opt _ADOCatalog **ppvObject) = 0;
        
        virtual /* [helpcontext][id][propput] */ HRESULT STDMETHODCALLTYPE put_ParentCatalog( 
            /* [in] */ __RPC__in_opt _ADOCatalog *ppvObject) = 0;
        
        virtual /* [helpcontext][id][propputref] */ HRESULT STDMETHODCALLTYPE putref_ParentCatalog( 
            /* [in] */ __RPC__in_opt _ADOCatalog *ppvObject) = 0;
        
    };
    
    
#else 	/* C style interface */
    typedef struct _TableVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in _ADOTable * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in _ADOTable * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in _ADOTable * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in _ADOTable * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in _ADOTable * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in _ADOTable * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            _ADOTable * This,
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
        
        DECLSPEC_XFGVIRT(_Table, get_Columns)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Columns )( 
            __RPC__in _ADOTable * This,
            /* [retval][out] */ __RPC__deref_out_opt ADOColumns **ppvObject);
        
        DECLSPEC_XFGVIRT(_Table, get_Name)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in _ADOTable * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(_Table, put_Name)
        /* [helpcontext][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Name )( 
            __RPC__in _ADOTable * This,
            /* [in] */ __RPC__in BSTR newVal);
        
        DECLSPEC_XFGVIRT(_Table, get_Type)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Type )( 
            __RPC__in _ADOTable * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(_Table, get_Indexes)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Indexes )( 
            __RPC__in _ADOTable * This,
            /* [retval][out] */ __RPC__deref_out_opt ADOIndexes **ppvObject);
        
        DECLSPEC_XFGVIRT(_Table, get_Keys)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Keys )( 
            __RPC__in _ADOTable * This,
            /* [retval][out] */ __RPC__deref_out_opt ADOKeys **ppvObject);
        
        DECLSPEC_XFGVIRT(_Table, get_Properties)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Properties )( 
            __RPC__in _ADOTable * This,
            /* [retval][out] */ __RPC__deref_out_opt ADOProperties **ppvObject);
        
        DECLSPEC_XFGVIRT(_Table, get_DateCreated)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DateCreated )( 
            __RPC__in _ADOTable * This,
            /* [retval][out] */ __RPC__out VARIANT *pVal);
        
        DECLSPEC_XFGVIRT(_Table, get_DateModified)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DateModified )( 
            __RPC__in _ADOTable * This,
            /* [retval][out] */ __RPC__out VARIANT *pVal);
        
        DECLSPEC_XFGVIRT(_Table, get_ParentCatalog)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ParentCatalog )( 
            __RPC__in _ADOTable * This,
            /* [retval][out] */ __RPC__deref_out_opt _ADOCatalog **ppvObject);
        
        DECLSPEC_XFGVIRT(_Table, put_ParentCatalog)
        /* [helpcontext][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ParentCatalog )( 
            __RPC__in _ADOTable * This,
            /* [in] */ __RPC__in_opt _ADOCatalog *ppvObject);
        
        DECLSPEC_XFGVIRT(_Table, putref_ParentCatalog)
        /* [helpcontext][id][propputref] */ HRESULT ( STDMETHODCALLTYPE *putref_ParentADOCatalog )( 
            __RPC__in _ADOTable * This,
            /* [in] */ __RPC__in_opt _ADOCatalog *ppvObject);
        
        END_INTERFACE
    } _TableVtbl;
    interface _Table
    {
        CONST_VTBL struct _TableVtbl *lpVtbl;
    };
    
#ifdef COBJMACROS
#define _Table_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define _Table_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define _Table_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 
#define _Table_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 
#define _Table_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 
#define _Table_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 
#define _Table_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 
#define _Table_get_Columns(This,ppvObject)	\
    ( (This)->lpVtbl -> get_Columns(This,ppvObject) ) 
#define _Table_get_Name(This,pVal)	\
    ( (This)->lpVtbl -> get_Name(This,pVal) ) 
#define _Table_put_Name(This,newVal)	\
    ( (This)->lpVtbl -> put_Name(This,newVal) ) 
#define _Table_get_Type(This,pVal)	\
    ( (This)->lpVtbl -> get_Type(This,pVal) ) 
#define _Table_get_Indexes(This,ppvObject)	\
    ( (This)->lpVtbl -> get_Indexes(This,ppvObject) ) 
#define _Table_get_Keys(This,ppvObject)	\
    ( (This)->lpVtbl -> get_Keys(This,ppvObject) ) 
#define _Table_get_Properties(This,ppvObject)	\
    ( (This)->lpVtbl -> get_Properties(This,ppvObject) ) 
#define _Table_get_DateCreated(This,pVal)	\
    ( (This)->lpVtbl -> get_DateCreated(This,pVal) ) 
#define _Table_get_DateModified(This,pVal)	\
    ( (This)->lpVtbl -> get_DateModified(This,pVal) ) 
#define _Table_get_ParentCatalog(This,ppvObject)	\
    ( (This)->lpVtbl -> get_ParentCatalog(This,ppvObject) ) 
#define _Table_put_ParentCatalog(This,ppvObject)	\
    ( (This)->lpVtbl -> put_ParentCatalog(This,ppvObject) ) 
#define _Table_putref_ParentCatalog(This,ppvObject)	\
    ( (This)->lpVtbl -> putref_ParentCatalog(This,ppvObject) ) 
#endif /* COBJMACROS */
#endif 	/* C style interface */
#endif 	/* ___Table_INTERFACE_DEFINED__ */
#ifndef ___Group25_INTERFACE_DEFINED__
#define ___Group25_INTERFACE_DEFINED__
/* interface _Group25 */
/* [helpcontext][unique][dual][uuid][hidden][nonextensible][object] */ 
EXTERN_C const IID IID__Group25;
#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000616-0000-0010-8000-00AA006D2EA4")
    _Group25 : public IDispatch
    {
    public:
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpcontext][id][propput] */ HRESULT STDMETHODCALLTYPE put_Name( 
            /* [in] */ __RPC__in BSTR newVal) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE GetPermissions( 
            /* [in] */ VARIANT Name,
            /* [in] */ ObjectTypeEnum ObjectType,
            /* [optional][in] */ VARIANT ObjectTypeId,
            /* [retval][out] */ __RPC__out RightsEnum *Rights) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE SetPermissions( 
            /* [in] */ VARIANT Name,
            /* [in] */ ObjectTypeEnum ObjectType,
            /* [in] */ ActionEnum Action,
            /* [in] */ RightsEnum Rights,
            /* [defaultvalue][in] */ InheritTypeEnum Inherit,
            /* [optional][in] */ VARIANT ObjectTypeId) = 0;
        
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_Users( 
            /* [retval][out] */ __RPC__deref_out_opt ADOUsers **ppvObject) = 0;
        
    };
    
    
#else 	/* C style interface */
    typedef struct _Group25Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in _Group25 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in _Group25 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in _Group25 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in _Group25 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in _Group25 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in _Group25 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            _Group25 * This,
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
        
        DECLSPEC_XFGVIRT(_Group25, get_Name)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in _Group25 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(_Group25, put_Name)
        /* [helpcontext][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Name )( 
            __RPC__in _Group25 * This,
            /* [in] */ __RPC__in BSTR newVal);
        
        DECLSPEC_XFGVIRT(_Group25, GetPermissions)
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *GetPermissions )( 
            __RPC__in _Group25 * This,
            /* [in] */ VARIANT Name,
            /* [in] */ ObjectTypeEnum ObjectType,
            /* [optional][in] */ VARIANT ObjectTypeId,
            /* [retval][out] */ __RPC__out RightsEnum *Rights);
        
        DECLSPEC_XFGVIRT(_Group25, SetPermissions)
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *SetPermissions )( 
            __RPC__in _Group25 * This,
            /* [in] */ VARIANT Name,
            /* [in] */ ObjectTypeEnum ObjectType,
            /* [in] */ ActionEnum Action,
            /* [in] */ RightsEnum Rights,
            /* [defaultvalue][in] */ InheritTypeEnum Inherit,
            /* [optional][in] */ VARIANT ObjectTypeId);
        
        DECLSPEC_XFGVIRT(_Group25, get_Users)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Users )( 
            __RPC__in _Group25 * This,
            /* [retval][out] */ __RPC__deref_out_opt ADOUsers **ppvObject);
        
        END_INTERFACE
    } _Group25Vtbl;
    interface _Group25
    {
        CONST_VTBL struct _Group25Vtbl *lpVtbl;
    };
    
#ifdef COBJMACROS
#define _Group25_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define _Group25_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define _Group25_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 
#define _Group25_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 
#define _Group25_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 
#define _Group25_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 
#define _Group25_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 
#define _Group25_get_Name(This,pVal)	\
    ( (This)->lpVtbl -> get_Name(This,pVal) ) 
#define _Group25_put_Name(This,newVal)	\
    ( (This)->lpVtbl -> put_Name(This,newVal) ) 
#define _Group25_GetPermissions(This,Name,ObjectType,ObjectTypeId,Rights)	\
    ( (This)->lpVtbl -> GetPermissions(This,Name,ObjectType,ObjectTypeId,Rights) ) 
#define _Group25_SetPermissions(This,Name,ObjectType,Action,Rights,Inherit,ObjectTypeId)	\
    ( (This)->lpVtbl -> SetPermissions(This,Name,ObjectType,Action,Rights,Inherit,ObjectTypeId) ) 
#define _Group25_get_Users(This,ppvObject)	\
    ( (This)->lpVtbl -> get_Users(This,ppvObject) ) 
#endif /* COBJMACROS */
#endif 	/* C style interface */
#endif 	/* ___Group25_INTERFACE_DEFINED__ */
#ifndef ___Group_INTERFACE_DEFINED__
#define ___Group_INTERFACE_DEFINED__
/* interface _ADOGroup */
/* [helpcontext][unique][dual][uuid][nonextensible][object] */ 
EXTERN_C const IID IID__Group;
#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000628-0000-0010-8000-00AA006D2EA4")
    _ADOGroup : public _Group25
    {
    public:
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_Properties( 
            /* [retval][out] */ __RPC__deref_out_opt ADOProperties **ppvObject) = 0;
        
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_ParentCatalog( 
            /* [retval][out] */ __RPC__deref_out_opt _ADOCatalog **ppvObject) = 0;
        
        virtual /* [helpcontext][id][propput] */ HRESULT STDMETHODCALLTYPE put_ParentCatalog( 
            /* [in] */ __RPC__in_opt _ADOCatalog *ppvObject) = 0;
        
        virtual /* [helpcontext][id][propputref] */ HRESULT STDMETHODCALLTYPE putref_ParentCatalog( 
            /* [in] */ __RPC__in_opt _ADOCatalog *ppvObject) = 0;
        
    };
    
    
#else 	/* C style interface */
    typedef struct _GroupVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in _ADOGroup * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in _ADOGroup * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in _ADOGroup * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in _ADOGroup * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in _ADOGroup * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in _ADOGroup * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            _ADOGroup * This,
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
        
        DECLSPEC_XFGVIRT(_Group25, get_Name)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in _ADOGroup * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(_Group25, put_Name)
        /* [helpcontext][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Name )( 
            __RPC__in _ADOGroup * This,
            /* [in] */ __RPC__in BSTR newVal);
        
        DECLSPEC_XFGVIRT(_Group25, GetPermissions)
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *GetPermissions )( 
            __RPC__in _ADOGroup * This,
            /* [in] */ VARIANT Name,
            /* [in] */ ObjectTypeEnum ObjectType,
            /* [optional][in] */ VARIANT ObjectTypeId,
            /* [retval][out] */ __RPC__out RightsEnum *Rights);
        
        DECLSPEC_XFGVIRT(_Group25, SetPermissions)
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *SetPermissions )( 
            __RPC__in _ADOGroup * This,
            /* [in] */ VARIANT Name,
            /* [in] */ ObjectTypeEnum ObjectType,
            /* [in] */ ActionEnum Action,
            /* [in] */ RightsEnum Rights,
            /* [defaultvalue][in] */ InheritTypeEnum Inherit,
            /* [optional][in] */ VARIANT ObjectTypeId);
        
        DECLSPEC_XFGVIRT(_Group25, get_Users)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Users )( 
            __RPC__in _ADOGroup * This,
            /* [retval][out] */ __RPC__deref_out_opt ADOUsers **ppvObject);
        
        DECLSPEC_XFGVIRT(_Group, get_Properties)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Properties )( 
            __RPC__in _ADOGroup * This,
            /* [retval][out] */ __RPC__deref_out_opt ADOProperties **ppvObject);
        
        DECLSPEC_XFGVIRT(_Group, get_ParentCatalog)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ParentCatalog )( 
            __RPC__in _ADOGroup * This,
            /* [retval][out] */ __RPC__deref_out_opt _ADOCatalog **ppvObject);
        
        DECLSPEC_XFGVIRT(_Group, put_ParentCatalog)
        /* [helpcontext][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ParentCatalog )( 
            __RPC__in _ADOGroup * This,
            /* [in] */ __RPC__in_opt _ADOCatalog *ppvObject);
        
        DECLSPEC_XFGVIRT(_Group, putref_ParentCatalog)
        /* [helpcontext][id][propputref] */ HRESULT ( STDMETHODCALLTYPE *putref_ParentADOCatalog )( 
            __RPC__in _ADOGroup * This,
            /* [in] */ __RPC__in_opt _ADOCatalog *ppvObject);
        
        END_INTERFACE
    } _GroupVtbl;
    interface _Group
    {
        CONST_VTBL struct _GroupVtbl *lpVtbl;
    };
    
#ifdef COBJMACROS
#define _Group_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define _Group_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define _Group_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 
#define _Group_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 
#define _Group_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 
#define _Group_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 
#define _Group_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 
#define _Group_get_Name(This,pVal)	\
    ( (This)->lpVtbl -> get_Name(This,pVal) ) 
#define _Group_put_Name(This,newVal)	\
    ( (This)->lpVtbl -> put_Name(This,newVal) ) 
#define _Group_GetPermissions(This,Name,ObjectType,ObjectTypeId,Rights)	\
    ( (This)->lpVtbl -> GetPermissions(This,Name,ObjectType,ObjectTypeId,Rights) ) 
#define _Group_SetPermissions(This,Name,ObjectType,Action,Rights,Inherit,ObjectTypeId)	\
    ( (This)->lpVtbl -> SetPermissions(This,Name,ObjectType,Action,Rights,Inherit,ObjectTypeId) ) 
#define _Group_get_Users(This,ppvObject)	\
    ( (This)->lpVtbl -> get_Users(This,ppvObject) ) 
#define _Group_get_Properties(This,ppvObject)	\
    ( (This)->lpVtbl -> get_Properties(This,ppvObject) ) 
#define _Group_get_ParentCatalog(This,ppvObject)	\
    ( (This)->lpVtbl -> get_ParentCatalog(This,ppvObject) ) 
#define _Group_put_ParentCatalog(This,ppvObject)	\
    ( (This)->lpVtbl -> put_ParentCatalog(This,ppvObject) ) 
#define _Group_putref_ParentCatalog(This,ppvObject)	\
    ( (This)->lpVtbl -> putref_ParentCatalog(This,ppvObject) ) 
#endif /* COBJMACROS */
#endif 	/* C style interface */
#endif 	/* ___Group_INTERFACE_DEFINED__ */
#ifndef ___User25_INTERFACE_DEFINED__
#define ___User25_INTERFACE_DEFINED__
/* interface _User25 */
/* [helpcontext][unique][dual][uuid][nonextensible][object] */ 
EXTERN_C const IID IID__User25;
#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000619-0000-0010-8000-00AA006D2EA4")
    _User25 : public IDispatch
    {
    public:
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpcontext][id][propput] */ HRESULT STDMETHODCALLTYPE put_Name( 
            /* [in] */ __RPC__in BSTR newVal) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE GetPermissions( 
            /* [in] */ VARIANT Name,
            /* [in] */ ObjectTypeEnum ObjectType,
            /* [optional][in] */ VARIANT ObjectTypeId,
            /* [retval][out] */ __RPC__out RightsEnum *Rights) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE SetPermissions( 
            /* [in] */ VARIANT Name,
            /* [in] */ ObjectTypeEnum ObjectType,
            /* [in] */ ActionEnum Action,
            /* [in] */ RightsEnum Rights,
            /* [defaultvalue][in] */ InheritTypeEnum Inherit,
            /* [optional][in] */ VARIANT ObjectTypeId) = 0;
        
        virtual /* [helpcontext][id] */ HRESULT STDMETHODCALLTYPE ChangePassword( 
            /* [in] */ __RPC__in BSTR OldPassword,
            /* [in] */ __RPC__in BSTR NewPassword) = 0;
        
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_Groups( 
            /* [retval][out] */ __RPC__deref_out_opt ADOGroups **ppvObject) = 0;
        
    };
    
    
#else 	/* C style interface */
    typedef struct _User25Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in _User25 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in _User25 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in _User25 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in _User25 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in _User25 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in _User25 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            _User25 * This,
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
        
        DECLSPEC_XFGVIRT(_User25, get_Name)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in _User25 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(_User25, put_Name)
        /* [helpcontext][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Name )( 
            __RPC__in _User25 * This,
            /* [in] */ __RPC__in BSTR newVal);
        
        DECLSPEC_XFGVIRT(_User25, GetPermissions)
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *GetPermissions )( 
            __RPC__in _User25 * This,
            /* [in] */ VARIANT Name,
            /* [in] */ ObjectTypeEnum ObjectType,
            /* [optional][in] */ VARIANT ObjectTypeId,
            /* [retval][out] */ __RPC__out RightsEnum *Rights);
        
        DECLSPEC_XFGVIRT(_User25, SetPermissions)
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *SetPermissions )( 
            __RPC__in _User25 * This,
            /* [in] */ VARIANT Name,
            /* [in] */ ObjectTypeEnum ObjectType,
            /* [in] */ ActionEnum Action,
            /* [in] */ RightsEnum Rights,
            /* [defaultvalue][in] */ InheritTypeEnum Inherit,
            /* [optional][in] */ VARIANT ObjectTypeId);
        
        DECLSPEC_XFGVIRT(_User25, ChangePassword)
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *ChangePassword )( 
            __RPC__in _User25 * This,
            /* [in] */ __RPC__in BSTR OldPassword,
            /* [in] */ __RPC__in BSTR NewPassword);
        
        DECLSPEC_XFGVIRT(_User25, get_Groups)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Groups )( 
            __RPC__in _User25 * This,
            /* [retval][out] */ __RPC__deref_out_opt ADOGroups **ppvObject);
        
        END_INTERFACE
    } _User25Vtbl;
    interface _User25
    {
        CONST_VTBL struct _User25Vtbl *lpVtbl;
    };
    
#ifdef COBJMACROS
#define _User25_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define _User25_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define _User25_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 
#define _User25_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 
#define _User25_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 
#define _User25_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 
#define _User25_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 
#define _User25_get_Name(This,pVal)	\
    ( (This)->lpVtbl -> get_Name(This,pVal) ) 
#define _User25_put_Name(This,newVal)	\
    ( (This)->lpVtbl -> put_Name(This,newVal) ) 
#define _User25_GetPermissions(This,Name,ObjectType,ObjectTypeId,Rights)	\
    ( (This)->lpVtbl -> GetPermissions(This,Name,ObjectType,ObjectTypeId,Rights) ) 
#define _User25_SetPermissions(This,Name,ObjectType,Action,Rights,Inherit,ObjectTypeId)	\
    ( (This)->lpVtbl -> SetPermissions(This,Name,ObjectType,Action,Rights,Inherit,ObjectTypeId) ) 
#define _User25_ChangePassword(This,OldPassword,NewPassword)	\
    ( (This)->lpVtbl -> ChangePassword(This,OldPassword,NewPassword) ) 
#define _User25_get_Groups(This,ppvObject)	\
    ( (This)->lpVtbl -> get_Groups(This,ppvObject) ) 
#endif /* COBJMACROS */
#endif 	/* C style interface */
#endif 	/* ___User25_INTERFACE_DEFINED__ */
#ifndef ___User_INTERFACE_DEFINED__
#define ___User_INTERFACE_DEFINED__
/* interface _ADOUser */
/* [helpcontext][unique][dual][uuid][nonextensible][object] */ 
EXTERN_C const IID IID__User;
#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000627-0000-0010-8000-00AA006D2EA4")
    _ADOUser : public _User25
    {
    public:
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_Properties( 
            /* [retval][out] */ __RPC__deref_out_opt ADOProperties **ppvObject) = 0;
        
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_ParentCatalog( 
            /* [retval][out] */ __RPC__deref_out_opt _ADOCatalog **ppvObject) = 0;
        
        virtual /* [helpcontext][id][propput] */ HRESULT STDMETHODCALLTYPE put_ParentCatalog( 
            /* [in] */ __RPC__in_opt _ADOCatalog *ppvObject) = 0;
        
        virtual /* [helpcontext][id][propputref] */ HRESULT STDMETHODCALLTYPE putref_ParentCatalog( 
            /* [in] */ __RPC__in_opt _ADOCatalog *ppvObject) = 0;
        
    };
    
    
#else 	/* C style interface */
    typedef struct _UserVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in _ADOUser * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in _ADOUser * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in _ADOUser * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in _ADOUser * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in _ADOUser * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in _ADOUser * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            _ADOUser * This,
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
        
        DECLSPEC_XFGVIRT(_User25, get_Name)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in _ADOUser * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(_User25, put_Name)
        /* [helpcontext][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Name )( 
            __RPC__in _ADOUser * This,
            /* [in] */ __RPC__in BSTR newVal);
        
        DECLSPEC_XFGVIRT(_User25, GetPermissions)
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *GetPermissions )( 
            __RPC__in _ADOUser * This,
            /* [in] */ VARIANT Name,
            /* [in] */ ObjectTypeEnum ObjectType,
            /* [optional][in] */ VARIANT ObjectTypeId,
            /* [retval][out] */ __RPC__out RightsEnum *Rights);
        
        DECLSPEC_XFGVIRT(_User25, SetPermissions)
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *SetPermissions )( 
            __RPC__in _ADOUser * This,
            /* [in] */ VARIANT Name,
            /* [in] */ ObjectTypeEnum ObjectType,
            /* [in] */ ActionEnum Action,
            /* [in] */ RightsEnum Rights,
            /* [defaultvalue][in] */ InheritTypeEnum Inherit,
            /* [optional][in] */ VARIANT ObjectTypeId);
        
        DECLSPEC_XFGVIRT(_User25, ChangePassword)
        /* [helpcontext][id] */ HRESULT ( STDMETHODCALLTYPE *ChangePassword )( 
            __RPC__in _ADOUser * This,
            /* [in] */ __RPC__in BSTR OldPassword,
            /* [in] */ __RPC__in BSTR NewPassword);
        
        DECLSPEC_XFGVIRT(_User25, get_Groups)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Groups )( 
            __RPC__in _ADOUser * This,
            /* [retval][out] */ __RPC__deref_out_opt ADOGroups **ppvObject);
        
        DECLSPEC_XFGVIRT(_User, get_Properties)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Properties )( 
            __RPC__in _ADOUser * This,
            /* [retval][out] */ __RPC__deref_out_opt ADOProperties **ppvObject);
        
        DECLSPEC_XFGVIRT(_User, get_ParentCatalog)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ParentCatalog )( 
            __RPC__in _ADOUser * This,
            /* [retval][out] */ __RPC__deref_out_opt _ADOCatalog **ppvObject);
        
        DECLSPEC_XFGVIRT(_User, put_ParentCatalog)
        /* [helpcontext][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ParentCatalog )( 
            __RPC__in _ADOUser * This,
            /* [in] */ __RPC__in_opt _ADOCatalog *ppvObject);
        
        DECLSPEC_XFGVIRT(_User, putref_ParentCatalog)
        /* [helpcontext][id][propputref] */ HRESULT ( STDMETHODCALLTYPE *putref_ParentADOCatalog )( 
            __RPC__in _ADOUser * This,
            /* [in] */ __RPC__in_opt _ADOCatalog *ppvObject);
        
        END_INTERFACE
    } _UserVtbl;
    interface _User
    {
        CONST_VTBL struct _UserVtbl *lpVtbl;
    };
    
#ifdef COBJMACROS
#define _User_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define _User_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define _User_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 
#define _User_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 
#define _User_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 
#define _User_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 
#define _User_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 
#define _User_get_Name(This,pVal)	\
    ( (This)->lpVtbl -> get_Name(This,pVal) ) 
#define _User_put_Name(This,newVal)	\
    ( (This)->lpVtbl -> put_Name(This,newVal) ) 
#define _User_GetPermissions(This,Name,ObjectType,ObjectTypeId,Rights)	\
    ( (This)->lpVtbl -> GetPermissions(This,Name,ObjectType,ObjectTypeId,Rights) ) 
#define _User_SetPermissions(This,Name,ObjectType,Action,Rights,Inherit,ObjectTypeId)	\
    ( (This)->lpVtbl -> SetPermissions(This,Name,ObjectType,Action,Rights,Inherit,ObjectTypeId) ) 
#define _User_ChangePassword(This,OldPassword,NewPassword)	\
    ( (This)->lpVtbl -> ChangePassword(This,OldPassword,NewPassword) ) 
#define _User_get_Groups(This,ppvObject)	\
    ( (This)->lpVtbl -> get_Groups(This,ppvObject) ) 
#define _User_get_Properties(This,ppvObject)	\
    ( (This)->lpVtbl -> get_Properties(This,ppvObject) ) 
#define _User_get_ParentCatalog(This,ppvObject)	\
    ( (This)->lpVtbl -> get_ParentCatalog(This,ppvObject) ) 
#define _User_put_ParentCatalog(This,ppvObject)	\
    ( (This)->lpVtbl -> put_ParentCatalog(This,ppvObject) ) 
#define _User_putref_ParentCatalog(This,ppvObject)	\
    ( (This)->lpVtbl -> putref_ParentCatalog(This,ppvObject) ) 
#endif /* COBJMACROS */
#endif 	/* C style interface */
#endif 	/* ___User_INTERFACE_DEFINED__ */
#ifndef ___Column_INTERFACE_DEFINED__
#define ___Column_INTERFACE_DEFINED__
/* interface _ADOColumn */
/* [helpcontext][unique][dual][uuid][nonextensible][object] */ 
EXTERN_C const IID IID__Column;
#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0000061C-0000-0010-8000-00AA006D2EA4")
    _ADOColumn : public IDispatch
    {
    public:
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpcontext][id][propput] */ HRESULT STDMETHODCALLTYPE put_Name( 
            /* [in] */ __RPC__in BSTR newVal) = 0;
        
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_Attributes( 
            /* [retval][out] */ __RPC__out ColumnAttributesEnum *pVal) = 0;
        
        virtual /* [helpcontext][id][propput] */ HRESULT STDMETHODCALLTYPE put_Attributes( 
            /* [in] */ ColumnAttributesEnum newVal) = 0;
        
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_DefinedSize( 
            /* [retval][out] */ __RPC__out long *pVal) = 0;
        
        virtual /* [helpcontext][id][propput] */ HRESULT STDMETHODCALLTYPE put_DefinedSize( 
            /* [in] */ long DefinedSize) = 0;
        
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_NumericScale( 
            /* [retval][out] */ __RPC__out BYTE *pVal) = 0;
        
        virtual /* [helpcontext][id][propput] */ HRESULT STDMETHODCALLTYPE put_NumericScale( 
            /* [in] */ BYTE newVal) = 0;
        
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_Precision( 
            /* [retval][out] */ __RPC__out long *pVal) = 0;
        
        virtual /* [helpcontext][id][propput] */ HRESULT STDMETHODCALLTYPE put_Precision( 
            /* [in] */ long newVal) = 0;
        
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_RelatedColumn( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpcontext][id][propput] */ HRESULT STDMETHODCALLTYPE put_RelatedColumn( 
            /* [in] */ __RPC__in BSTR newVal) = 0;
        
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_SortOrder( 
            /* [retval][out] */ __RPC__out SortOrderEnum *pVal) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_SortOrder( 
            /* [in] */ SortOrderEnum newVal) = 0;
        
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_Type( 
            /* [retval][out] */ __RPC__out DataTypeEnum *pVal) = 0;
        
        virtual /* [helpcontext][id][propput] */ HRESULT STDMETHODCALLTYPE put_Type( 
            /* [in] */ DataTypeEnum newVal) = 0;
        
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_Properties( 
            /* [retval][out] */ __RPC__deref_out_opt ADOProperties **ppvObject) = 0;
        
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_ParentCatalog( 
            /* [retval][out] */ __RPC__deref_out_opt _ADOCatalog **ppvObject) = 0;
        
        virtual /* [helpcontext][id][propput] */ HRESULT STDMETHODCALLTYPE put_ParentCatalog( 
            /* [in] */ __RPC__in_opt _ADOCatalog *ppvObject) = 0;
        
        virtual /* [helpcontext][id][propputref] */ HRESULT STDMETHODCALLTYPE putref_ParentCatalog( 
            /* [in] */ __RPC__in_opt _ADOCatalog *ppvObject) = 0;
        
    };
    
    
#else 	/* C style interface */
    typedef struct _ColumnVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in _ADOColumn * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in _ADOColumn * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in _ADOColumn * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in _ADOColumn * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in _ADOColumn * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in _ADOColumn * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            _ADOColumn * This,
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
        
        DECLSPEC_XFGVIRT(_Column, get_Name)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in _ADOColumn * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(_Column, put_Name)
        /* [helpcontext][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Name )( 
            __RPC__in _ADOColumn * This,
            /* [in] */ __RPC__in BSTR newVal);
        
        DECLSPEC_XFGVIRT(_Column, get_Attributes)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Attributes )( 
            __RPC__in _ADOColumn * This,
            /* [retval][out] */ __RPC__out ColumnAttributesEnum *pVal);
        
        DECLSPEC_XFGVIRT(_Column, put_Attributes)
        /* [helpcontext][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Attributes )( 
            __RPC__in _ADOColumn * This,
            /* [in] */ ColumnAttributesEnum newVal);
        
        DECLSPEC_XFGVIRT(_Column, get_DefinedSize)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DefinedSize )( 
            __RPC__in _ADOColumn * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(_Column, put_DefinedSize)
        /* [helpcontext][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DefinedSize )( 
            __RPC__in _ADOColumn * This,
            /* [in] */ long DefinedSize);
        
        DECLSPEC_XFGVIRT(_Column, get_NumericScale)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_NumericScale )( 
            __RPC__in _ADOColumn * This,
            /* [retval][out] */ __RPC__out BYTE *pVal);
        
        DECLSPEC_XFGVIRT(_Column, put_NumericScale)
        /* [helpcontext][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_NumericScale )( 
            __RPC__in _ADOColumn * This,
            /* [in] */ BYTE newVal);
        
        DECLSPEC_XFGVIRT(_Column, get_Precision)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Precision )( 
            __RPC__in _ADOColumn * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(_Column, put_Precision)
        /* [helpcontext][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Precision )( 
            __RPC__in _ADOColumn * This,
            /* [in] */ long newVal);
        
        DECLSPEC_XFGVIRT(_Column, get_RelatedColumn)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RelatedColumn )( 
            __RPC__in _ADOColumn * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(_Column, put_RelatedColumn)
        /* [helpcontext][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_RelatedColumn )( 
            __RPC__in _ADOColumn * This,
            /* [in] */ __RPC__in BSTR newVal);
        
        DECLSPEC_XFGVIRT(_Column, get_SortOrder)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SortOrder )( 
            __RPC__in _ADOColumn * This,
            /* [retval][out] */ __RPC__out SortOrderEnum *pVal);
        
        DECLSPEC_XFGVIRT(_Column, put_SortOrder)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SortOrder )( 
            __RPC__in _ADOColumn * This,
            /* [in] */ SortOrderEnum newVal);
        
        DECLSPEC_XFGVIRT(_Column, get_Type)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Type )( 
            __RPC__in _ADOColumn * This,
            /* [retval][out] */ __RPC__out DataTypeEnum *pVal);
        
        DECLSPEC_XFGVIRT(_Column, put_Type)
        /* [helpcontext][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Type )( 
            __RPC__in _ADOColumn * This,
            /* [in] */ DataTypeEnum newVal);
        
        DECLSPEC_XFGVIRT(_Column, get_Properties)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Properties )( 
            __RPC__in _ADOColumn * This,
            /* [retval][out] */ __RPC__deref_out_opt ADOProperties **ppvObject);
        
        DECLSPEC_XFGVIRT(_Column, get_ParentCatalog)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ParentCatalog )( 
            __RPC__in _ADOColumn * This,
            /* [retval][out] */ __RPC__deref_out_opt _ADOCatalog **ppvObject);
        
        DECLSPEC_XFGVIRT(_Column, put_ParentCatalog)
        /* [helpcontext][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ParentCatalog )( 
            __RPC__in _ADOColumn * This,
            /* [in] */ __RPC__in_opt _ADOCatalog *ppvObject);
        
        DECLSPEC_XFGVIRT(_Column, putref_ParentCatalog)
        /* [helpcontext][id][propputref] */ HRESULT ( STDMETHODCALLTYPE *putref_ParentADOCatalog )( 
            __RPC__in _ADOColumn * This,
            /* [in] */ __RPC__in_opt _ADOCatalog *ppvObject);
        
        END_INTERFACE
    } _ColumnVtbl;
    interface _Column
    {
        CONST_VTBL struct _ColumnVtbl *lpVtbl;
    };
    
#ifdef COBJMACROS
#define _Column_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define _Column_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define _Column_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 
#define _Column_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 
#define _Column_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 
#define _Column_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 
#define _Column_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 
#define _Column_get_Name(This,pVal)	\
    ( (This)->lpVtbl -> get_Name(This,pVal) ) 
#define _Column_put_Name(This,newVal)	\
    ( (This)->lpVtbl -> put_Name(This,newVal) ) 
#define _Column_get_Attributes(This,pVal)	\
    ( (This)->lpVtbl -> get_Attributes(This,pVal) ) 
#define _Column_put_Attributes(This,newVal)	\
    ( (This)->lpVtbl -> put_Attributes(This,newVal) ) 
#define _Column_get_DefinedSize(This,pVal)	\
    ( (This)->lpVtbl -> get_DefinedSize(This,pVal) ) 
#define _Column_put_DefinedSize(This,DefinedSize)	\
    ( (This)->lpVtbl -> put_DefinedSize(This,DefinedSize) ) 
#define _Column_get_NumericScale(This,pVal)	\
    ( (This)->lpVtbl -> get_NumericScale(This,pVal) ) 
#define _Column_put_NumericScale(This,newVal)	\
    ( (This)->lpVtbl -> put_NumericScale(This,newVal) ) 
#define _Column_get_Precision(This,pVal)	\
    ( (This)->lpVtbl -> get_Precision(This,pVal) ) 
#define _Column_put_Precision(This,newVal)	\
    ( (This)->lpVtbl -> put_Precision(This,newVal) ) 
#define _Column_get_RelatedColumn(This,pVal)	\
    ( (This)->lpVtbl -> get_RelatedColumn(This,pVal) ) 
#define _Column_put_RelatedColumn(This,newVal)	\
    ( (This)->lpVtbl -> put_RelatedColumn(This,newVal) ) 
#define _Column_get_SortOrder(This,pVal)	\
    ( (This)->lpVtbl -> get_SortOrder(This,pVal) ) 
#define _Column_put_SortOrder(This,newVal)	\
    ( (This)->lpVtbl -> put_SortOrder(This,newVal) ) 
#define _Column_get_Type(This,pVal)	\
    ( (This)->lpVtbl -> get_Type(This,pVal) ) 
#define _Column_put_Type(This,newVal)	\
    ( (This)->lpVtbl -> put_Type(This,newVal) ) 
#define _Column_get_Properties(This,ppvObject)	\
    ( (This)->lpVtbl -> get_Properties(This,ppvObject) ) 
#define _Column_get_ParentCatalog(This,ppvObject)	\
    ( (This)->lpVtbl -> get_ParentCatalog(This,ppvObject) ) 
#define _Column_put_ParentCatalog(This,ppvObject)	\
    ( (This)->lpVtbl -> put_ParentCatalog(This,ppvObject) ) 
#define _Column_putref_ParentCatalog(This,ppvObject)	\
    ( (This)->lpVtbl -> putref_ParentCatalog(This,ppvObject) ) 
#endif /* COBJMACROS */
#endif 	/* C style interface */
#endif 	/* ___Column_INTERFACE_DEFINED__ */
#ifndef ___Index_INTERFACE_DEFINED__
#define ___Index_INTERFACE_DEFINED__
/* interface _ADOIndex */
/* [helpcontext][unique][dual][uuid][nonextensible][object] */ 
EXTERN_C const IID IID__Index;
#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0000061F-0000-0010-8000-00AA006D2EA4")
    _ADOIndex : public IDispatch
    {
    public:
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpcontext][id][propput] */ HRESULT STDMETHODCALLTYPE put_Name( 
            /* [in] */ __RPC__in BSTR newVal) = 0;
        
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_Clustered( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pVal) = 0;
        
        virtual /* [helpcontext][id][propput] */ HRESULT STDMETHODCALLTYPE put_Clustered( 
            /* [in] */ VARIANT_BOOL newVal) = 0;
        
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_IndexNulls( 
            /* [retval][out] */ __RPC__out AllowNullsEnum *pVal) = 0;
        
        virtual /* [helpcontext][id][propput] */ HRESULT STDMETHODCALLTYPE put_IndexNulls( 
            /* [in] */ AllowNullsEnum newVal) = 0;
        
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_PrimaryKey( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pVal) = 0;
        
        virtual /* [helpcontext][id][propput] */ HRESULT STDMETHODCALLTYPE put_PrimaryKey( 
            /* [in] */ VARIANT_BOOL newVal) = 0;
        
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_Unique( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pVal) = 0;
        
        virtual /* [helpcontext][id][propput] */ HRESULT STDMETHODCALLTYPE put_Unique( 
            /* [in] */ VARIANT_BOOL newVal) = 0;
        
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_Columns( 
            /* [retval][out] */ __RPC__deref_out_opt ADOColumns **ppvObject) = 0;
        
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_Properties( 
            /* [retval][out] */ __RPC__deref_out_opt ADOProperties **ppvObject) = 0;
        
    };
    
    
#else 	/* C style interface */
    typedef struct _IndexVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in _ADOIndex * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in _ADOIndex * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in _ADOIndex * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in _ADOIndex * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in _ADOIndex * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in _ADOIndex * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            _ADOIndex * This,
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
        
        DECLSPEC_XFGVIRT(_Index, get_Name)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in _ADOIndex * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(_Index, put_Name)
        /* [helpcontext][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Name )( 
            __RPC__in _ADOIndex * This,
            /* [in] */ __RPC__in BSTR newVal);
        
        DECLSPEC_XFGVIRT(_Index, get_Clustered)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Clustered )( 
            __RPC__in _ADOIndex * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pVal);
        
        DECLSPEC_XFGVIRT(_Index, put_Clustered)
        /* [helpcontext][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Clustered )( 
            __RPC__in _ADOIndex * This,
            /* [in] */ VARIANT_BOOL newVal);
        
        DECLSPEC_XFGVIRT(_Index, get_IndexNulls)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_IndexNulls )( 
            __RPC__in _ADOIndex * This,
            /* [retval][out] */ __RPC__out AllowNullsEnum *pVal);
        
        DECLSPEC_XFGVIRT(_Index, put_IndexNulls)
        /* [helpcontext][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_IndexNulls )( 
            __RPC__in _ADOIndex * This,
            /* [in] */ AllowNullsEnum newVal);
        
        DECLSPEC_XFGVIRT(_Index, get_PrimaryKey)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PrimaryKey )( 
            __RPC__in _ADOIndex * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pVal);
        
        DECLSPEC_XFGVIRT(_Index, put_PrimaryKey)
        /* [helpcontext][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PrimaryKey )( 
            __RPC__in _ADOIndex * This,
            /* [in] */ VARIANT_BOOL newVal);
        
        DECLSPEC_XFGVIRT(_Index, get_Unique)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Unique )( 
            __RPC__in _ADOIndex * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pVal);
        
        DECLSPEC_XFGVIRT(_Index, put_Unique)
        /* [helpcontext][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Unique )( 
            __RPC__in _ADOIndex * This,
            /* [in] */ VARIANT_BOOL newVal);
        
        DECLSPEC_XFGVIRT(_Index, get_Columns)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Columns )( 
            __RPC__in _ADOIndex * This,
            /* [retval][out] */ __RPC__deref_out_opt ADOColumns **ppvObject);
        
        DECLSPEC_XFGVIRT(_Index, get_Properties)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Properties )( 
            __RPC__in _ADOIndex * This,
            /* [retval][out] */ __RPC__deref_out_opt ADOProperties **ppvObject);
        
        END_INTERFACE
    } _IndexVtbl;
    interface _Index
    {
        CONST_VTBL struct _IndexVtbl *lpVtbl;
    };
    
#ifdef COBJMACROS
#define _Index_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define _Index_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define _Index_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 
#define _Index_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 
#define _Index_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 
#define _Index_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 
#define _Index_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 
#define _Index_get_Name(This,pVal)	\
    ( (This)->lpVtbl -> get_Name(This,pVal) ) 
#define _Index_put_Name(This,newVal)	\
    ( (This)->lpVtbl -> put_Name(This,newVal) ) 
#define _Index_get_Clustered(This,pVal)	\
    ( (This)->lpVtbl -> get_Clustered(This,pVal) ) 
#define _Index_put_Clustered(This,newVal)	\
    ( (This)->lpVtbl -> put_Clustered(This,newVal) ) 
#define _Index_get_IndexNulls(This,pVal)	\
    ( (This)->lpVtbl -> get_IndexNulls(This,pVal) ) 
#define _Index_put_IndexNulls(This,newVal)	\
    ( (This)->lpVtbl -> put_IndexNulls(This,newVal) ) 
#define _Index_get_PrimaryKey(This,pVal)	\
    ( (This)->lpVtbl -> get_PrimaryKey(This,pVal) ) 
#define _Index_put_PrimaryKey(This,newVal)	\
    ( (This)->lpVtbl -> put_PrimaryKey(This,newVal) ) 
#define _Index_get_Unique(This,pVal)	\
    ( (This)->lpVtbl -> get_Unique(This,pVal) ) 
#define _Index_put_Unique(This,newVal)	\
    ( (This)->lpVtbl -> put_Unique(This,newVal) ) 
#define _Index_get_Columns(This,ppvObject)	\
    ( (This)->lpVtbl -> get_Columns(This,ppvObject) ) 
#define _Index_get_Properties(This,ppvObject)	\
    ( (This)->lpVtbl -> get_Properties(This,ppvObject) ) 
#endif /* COBJMACROS */
#endif 	/* C style interface */
#endif 	/* ___Index_INTERFACE_DEFINED__ */
#ifndef ___Key_INTERFACE_DEFINED__
#define ___Key_INTERFACE_DEFINED__
/* interface _ADOKey */
/* [helpcontext][unique][dual][uuid][nonextensible][object] */ 
EXTERN_C const IID IID__Key;
#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000622-0000-0010-8000-00AA006D2EA4")
    _ADOKey : public IDispatch
    {
    public:
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpcontext][id][propput] */ HRESULT STDMETHODCALLTYPE put_Name( 
            /* [in] */ __RPC__in BSTR newVal) = 0;
        
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_DeleteRule( 
            /* [retval][out] */ __RPC__out RuleEnum *pVal) = 0;
        
        virtual /* [helpcontext][id][propput] */ HRESULT STDMETHODCALLTYPE put_DeleteRule( 
            /* [in] */ RuleEnum newVal) = 0;
        
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_Type( 
            /* [retval][out] */ __RPC__out KeyTypeEnum *pVal) = 0;
        
        virtual /* [helpcontext][id][propput] */ HRESULT STDMETHODCALLTYPE put_Type( 
            /* [in] */ KeyTypeEnum newVal) = 0;
        
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_RelatedTable( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpcontext][id][propput] */ HRESULT STDMETHODCALLTYPE put_RelatedTable( 
            /* [in] */ __RPC__in BSTR newVal) = 0;
        
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_UpdateRule( 
            /* [retval][out] */ __RPC__out RuleEnum *pVal) = 0;
        
        virtual /* [helpcontext][id][propput] */ HRESULT STDMETHODCALLTYPE put_UpdateRule( 
            /* [in] */ RuleEnum newVal) = 0;
        
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_Columns( 
            /* [retval][out] */ __RPC__deref_out_opt ADOColumns **ppvObject) = 0;
        
    };
    
    
#else 	/* C style interface */
    typedef struct _KeyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in _ADOKey * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in _ADOKey * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in _ADOKey * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in _ADOKey * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in _ADOKey * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in _ADOKey * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            _ADOKey * This,
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
        
        DECLSPEC_XFGVIRT(_Key, get_Name)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in _ADOKey * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(_Key, put_Name)
        /* [helpcontext][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Name )( 
            __RPC__in _ADOKey * This,
            /* [in] */ __RPC__in BSTR newVal);
        
        DECLSPEC_XFGVIRT(_Key, get_DeleteRule)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DeleteRule )( 
            __RPC__in _ADOKey * This,
            /* [retval][out] */ __RPC__out RuleEnum *pVal);
        
        DECLSPEC_XFGVIRT(_Key, put_DeleteRule)
        /* [helpcontext][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DeleteRule )( 
            __RPC__in _ADOKey * This,
            /* [in] */ RuleEnum newVal);
        
        DECLSPEC_XFGVIRT(_Key, get_Type)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Type )( 
            __RPC__in _ADOKey * This,
            /* [retval][out] */ __RPC__out KeyTypeEnum *pVal);
        
        DECLSPEC_XFGVIRT(_Key, put_Type)
        /* [helpcontext][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Type )( 
            __RPC__in _ADOKey * This,
            /* [in] */ KeyTypeEnum newVal);
        
        DECLSPEC_XFGVIRT(_Key, get_RelatedTable)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RelatedTable )( 
            __RPC__in _ADOKey * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(_Key, put_RelatedTable)
        /* [helpcontext][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_RelatedTable )( 
            __RPC__in _ADOKey * This,
            /* [in] */ __RPC__in BSTR newVal);
        
        DECLSPEC_XFGVIRT(_Key, get_UpdateRule)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UpdateRule )( 
            __RPC__in _ADOKey * This,
            /* [retval][out] */ __RPC__out RuleEnum *pVal);
        
        DECLSPEC_XFGVIRT(_Key, put_UpdateRule)
        /* [helpcontext][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_UpdateRule )( 
            __RPC__in _ADOKey * This,
            /* [in] */ RuleEnum newVal);
        
        DECLSPEC_XFGVIRT(_Key, get_Columns)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Columns )( 
            __RPC__in _ADOKey * This,
            /* [retval][out] */ __RPC__deref_out_opt ADOColumns **ppvObject);
        
        END_INTERFACE
    } _KeyVtbl;
    interface _Key
    {
        CONST_VTBL struct _KeyVtbl *lpVtbl;
    };
    
#ifdef COBJMACROS
#define _Key_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define _Key_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define _Key_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 
#define _Key_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 
#define _Key_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 
#define _Key_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 
#define _Key_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 
#define _Key_get_Name(This,pVal)	\
    ( (This)->lpVtbl -> get_Name(This,pVal) ) 
#define _Key_put_Name(This,newVal)	\
    ( (This)->lpVtbl -> put_Name(This,newVal) ) 
#define _Key_get_DeleteRule(This,pVal)	\
    ( (This)->lpVtbl -> get_DeleteRule(This,pVal) ) 
#define _Key_put_DeleteRule(This,newVal)	\
    ( (This)->lpVtbl -> put_DeleteRule(This,newVal) ) 
#define _Key_get_Type(This,pVal)	\
    ( (This)->lpVtbl -> get_Type(This,pVal) ) 
#define _Key_put_Type(This,newVal)	\
    ( (This)->lpVtbl -> put_Type(This,newVal) ) 
#define _Key_get_RelatedTable(This,pVal)	\
    ( (This)->lpVtbl -> get_RelatedTable(This,pVal) ) 
#define _Key_put_RelatedTable(This,newVal)	\
    ( (This)->lpVtbl -> put_RelatedTable(This,newVal) ) 
#define _Key_get_UpdateRule(This,pVal)	\
    ( (This)->lpVtbl -> get_UpdateRule(This,pVal) ) 
#define _Key_put_UpdateRule(This,newVal)	\
    ( (This)->lpVtbl -> put_UpdateRule(This,newVal) ) 
#define _Key_get_Columns(This,ppvObject)	\
    ( (This)->lpVtbl -> get_Columns(This,ppvObject) ) 
#endif /* COBJMACROS */
#endif 	/* C style interface */
#endif 	/* ___Key_INTERFACE_DEFINED__ */
#ifndef __View_INTERFACE_DEFINED__
#define __View_INTERFACE_DEFINED__
/* interface ADOView */
/* [helpcontext][unique][dual][uuid][nonextensible][object] */ 
EXTERN_C const IID IID_View;
#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000613-0000-0010-8000-00AA006D2EA4")
    ADOView : public IDispatch
    {
    public:
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_Command( 
            /* [retval][out] */ __RPC__out VARIANT *pVal) = 0;
        
        virtual /* [helpcontext][id][propput] */ HRESULT STDMETHODCALLTYPE put_Command( 
            /* [in] */ VARIANT newVal) = 0;
        
        virtual /* [helpcontext][id][propputref] */ HRESULT STDMETHODCALLTYPE putref_Command( 
            /* [in] */ __RPC__in_opt IDispatch *pComm) = 0;
        
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_DateCreated( 
            /* [retval][out] */ __RPC__out VARIANT *pVal) = 0;
        
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_DateModified( 
            /* [retval][out] */ __RPC__out VARIANT *pVal) = 0;
        
    };
    
    
#else 	/* C style interface */
    typedef struct ViewVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ADOView * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ADOView * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ADOView * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ADOView * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ADOView * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ADOView * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ADOView * This,
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
        
        DECLSPEC_XFGVIRT(View, get_Command)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Command )( 
            __RPC__in ADOView * This,
            /* [retval][out] */ __RPC__out VARIANT *pVal);
        
        DECLSPEC_XFGVIRT(View, put_Command)
        /* [helpcontext][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Command )( 
            __RPC__in ADOView * This,
            /* [in] */ VARIANT newVal);
        
        DECLSPEC_XFGVIRT(View, putref_Command)
        /* [helpcontext][id][propputref] */ HRESULT ( STDMETHODCALLTYPE *putref_Command )( 
            __RPC__in ADOView * This,
            /* [in] */ __RPC__in_opt IDispatch *pComm);
        
        DECLSPEC_XFGVIRT(View, get_Name)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in ADOView * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(View, get_DateCreated)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DateCreated )( 
            __RPC__in ADOView * This,
            /* [retval][out] */ __RPC__out VARIANT *pVal);
        
        DECLSPEC_XFGVIRT(View, get_DateModified)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DateModified )( 
            __RPC__in ADOView * This,
            /* [retval][out] */ __RPC__out VARIANT *pVal);
        
        END_INTERFACE
    } ViewVtbl;
    interface View
    {
        CONST_VTBL struct ViewVtbl *lpVtbl;
    };
    
#ifdef COBJMACROS
#define View_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define View_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define View_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 
#define View_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 
#define View_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 
#define View_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 
#define View_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 
#define View_get_Command(This,pVal)	\
    ( (This)->lpVtbl -> get_Command(This,pVal) ) 
#define View_put_Command(This,newVal)	\
    ( (This)->lpVtbl -> put_Command(This,newVal) ) 
#define View_putref_Command(This,pComm)	\
    ( (This)->lpVtbl -> putref_Command(This,pComm) ) 
#define View_get_Name(This,pVal)	\
    ( (This)->lpVtbl -> get_Name(This,pVal) ) 
#define View_get_DateCreated(This,pVal)	\
    ( (This)->lpVtbl -> get_DateCreated(This,pVal) ) 
#define View_get_DateModified(This,pVal)	\
    ( (This)->lpVtbl -> get_DateModified(This,pVal) ) 
#endif /* COBJMACROS */
#endif 	/* C style interface */
#endif 	/* __View_INTERFACE_DEFINED__ */
#ifndef __Procedure_INTERFACE_DEFINED__
#define __Procedure_INTERFACE_DEFINED__
/* interface ADOProcedure */
/* [helpcontext][unique][dual][uuid][nonextensible][object] */ 
EXTERN_C const IID IID_Procedure;
#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000625-0000-0010-8000-00AA006D2EA4")
    ADOProcedure : public IDispatch
    {
    public:
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_Command( 
            /* [retval][out] */ __RPC__out VARIANT *pVar) = 0;
        
        virtual /* [helpcontext][id][propput] */ HRESULT STDMETHODCALLTYPE put_Command( 
            /* [in] */ VARIANT newVal) = 0;
        
        virtual /* [helpcontext][id][propputref] */ HRESULT STDMETHODCALLTYPE putref_Command( 
            /* [in] */ __RPC__in_opt IDispatch *pComm) = 0;
        
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_DateCreated( 
            /* [retval][out] */ __RPC__out VARIANT *pVal) = 0;
        
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_DateModified( 
            /* [retval][out] */ __RPC__out VARIANT *pVal) = 0;
        
    };
    
    
#else 	/* C style interface */
    typedef struct ProcedureVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ADOProcedure * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ADOProcedure * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ADOProcedure * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ADOProcedure * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ADOProcedure * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ADOProcedure * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ADOProcedure * This,
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
        
        DECLSPEC_XFGVIRT(Procedure, get_Command)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Command )( 
            __RPC__in ADOProcedure * This,
            /* [retval][out] */ __RPC__out VARIANT *pVar);
        
        DECLSPEC_XFGVIRT(Procedure, put_Command)
        /* [helpcontext][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Command )( 
            __RPC__in ADOProcedure * This,
            /* [in] */ VARIANT newVal);
        
        DECLSPEC_XFGVIRT(Procedure, putref_Command)
        /* [helpcontext][id][propputref] */ HRESULT ( STDMETHODCALLTYPE *putref_Command )( 
            __RPC__in ADOProcedure * This,
            /* [in] */ __RPC__in_opt IDispatch *pComm);
        
        DECLSPEC_XFGVIRT(Procedure, get_Name)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in ADOProcedure * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(Procedure, get_DateCreated)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DateCreated )( 
            __RPC__in ADOProcedure * This,
            /* [retval][out] */ __RPC__out VARIANT *pVal);
        
        DECLSPEC_XFGVIRT(Procedure, get_DateModified)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DateModified )( 
            __RPC__in ADOProcedure * This,
            /* [retval][out] */ __RPC__out VARIANT *pVal);
        
        END_INTERFACE
    } ProcedureVtbl;
    interface Procedure
    {
        CONST_VTBL struct ProcedureVtbl *lpVtbl;
    };
    
#ifdef COBJMACROS
#define Procedure_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define Procedure_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define Procedure_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 
#define Procedure_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 
#define Procedure_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 
#define Procedure_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 
#define Procedure_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 
#define Procedure_get_Command(This,pVar)	\
    ( (This)->lpVtbl -> get_Command(This,pVar) ) 
#define Procedure_put_Command(This,newVal)	\
    ( (This)->lpVtbl -> put_Command(This,newVal) ) 
#define Procedure_putref_Command(This,pComm)	\
    ( (This)->lpVtbl -> putref_Command(This,pComm) ) 
#define Procedure_get_Name(This,pVal)	\
    ( (This)->lpVtbl -> get_Name(This,pVal) ) 
#define Procedure_get_DateCreated(This,pVal)	\
    ( (This)->lpVtbl -> get_DateCreated(This,pVal) ) 
#define Procedure_get_DateModified(This,pVal)	\
    ( (This)->lpVtbl -> get_DateModified(This,pVal) ) 
#endif /* COBJMACROS */
#endif 	/* C style interface */
#endif 	/* __Procedure_INTERFACE_DEFINED__ */
EXTERN_C const CLSID CLSID_Catalog;
#ifdef __cplusplus
Catalog;
#endif
EXTERN_C const CLSID CLSID_Table;
#ifdef __cplusplus
Table;
#endif
#ifndef __Property_INTERFACE_DEFINED__
#define __Property_INTERFACE_DEFINED__
/* interface ADOProperty */
/* [object][helpcontext][uuid][nonextensible][dual] */ 
EXTERN_C const IID IID_Property;
#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000503-0000-0010-8000-00AA006D2EA4")
    ADOProperty : public IDispatch
    {
    public:
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_Value( 
            /* [retval][out] */ __RPC__out VARIANT *pval) = 0;
        
        virtual /* [helpcontext][id][propput] */ HRESULT STDMETHODCALLTYPE put_Value( 
            /* [in] */ VARIANT val) = 0;
        
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr) = 0;
        
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_Type( 
            /* [retval][out] */ __RPC__out DataTypeEnum *ptype) = 0;
        
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_Attributes( 
            /* [retval][out] */ __RPC__out long *plAttributes) = 0;
        
        virtual /* [helpcontext][id][propput] */ HRESULT STDMETHODCALLTYPE put_Attributes( 
            /* [in] */ long lAttributes) = 0;
        
    };
    
    
#else 	/* C style interface */
    typedef struct PropertyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ADOProperty * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ADOProperty * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ADOProperty * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ADOProperty * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ADOProperty * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ADOProperty * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ADOProperty * This,
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
        
        DECLSPEC_XFGVIRT(Property, get_Value)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Value )( 
            __RPC__in ADOProperty * This,
            /* [retval][out] */ __RPC__out VARIANT *pval);
        
        DECLSPEC_XFGVIRT(Property, put_Value)
        /* [helpcontext][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Value )( 
            __RPC__in ADOProperty * This,
            /* [in] */ VARIANT val);
        
        DECLSPEC_XFGVIRT(Property, get_Name)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in ADOProperty * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        DECLSPEC_XFGVIRT(Property, get_Type)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Type )( 
            __RPC__in ADOProperty * This,
            /* [retval][out] */ __RPC__out DataTypeEnum *ptype);
        
        DECLSPEC_XFGVIRT(Property, get_Attributes)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Attributes )( 
            __RPC__in ADOProperty * This,
            /* [retval][out] */ __RPC__out long *plAttributes);
        
        DECLSPEC_XFGVIRT(Property, put_Attributes)
        /* [helpcontext][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Attributes )( 
            __RPC__in ADOProperty * This,
            /* [in] */ long lAttributes);
        
        END_INTERFACE
    } PropertyVtbl;
    interface Property
    {
        CONST_VTBL struct PropertyVtbl *lpVtbl;
    };
    
#ifdef COBJMACROS
#define Property_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define Property_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define Property_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 
#define Property_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 
#define Property_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 
#define Property_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 
#define Property_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 
#define Property_get_Value(This,pval)	\
    ( (This)->lpVtbl -> get_Value(This,pval) ) 
#define Property_put_Value(This,val)	\
    ( (This)->lpVtbl -> put_Value(This,val) ) 
#define Property_get_Name(This,pbstr)	\
    ( (This)->lpVtbl -> get_Name(This,pbstr) ) 
#define Property_get_Type(This,ptype)	\
    ( (This)->lpVtbl -> get_Type(This,ptype) ) 
#define Property_get_Attributes(This,plAttributes)	\
    ( (This)->lpVtbl -> get_Attributes(This,plAttributes) ) 
#define Property_put_Attributes(This,lAttributes)	\
    ( (This)->lpVtbl -> put_Attributes(This,lAttributes) ) 
#endif /* COBJMACROS */
#endif 	/* C style interface */
#endif 	/* __Property_INTERFACE_DEFINED__ */
EXTERN_C const CLSID CLSID_Group;
#ifdef __cplusplus
Group;
#endif
EXTERN_C const CLSID CLSID_User;
#ifdef __cplusplus
User;
#endif
EXTERN_C const CLSID CLSID_Column;
#ifdef __cplusplus
Column;
#endif
EXTERN_C const CLSID CLSID_Index;
#ifdef __cplusplus
Index;
#endif
EXTERN_C const CLSID CLSID_Key;
#ifdef __cplusplus
Key;
#endif
#ifndef __Tables_INTERFACE_DEFINED__
#define __Tables_INTERFACE_DEFINED__
/* interface ADOTables */
/* [object][uuid][helpcontext][nonextensible][dual] */ 
EXTERN_C const IID IID_Tables;
#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000611-0000-0010-8000-00AA006D2EA4")
    ADOTables : public _ADOCollection
    {
    public:
        virtual /* [id][helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT Item,
            /* [retval][out] */ __RPC__deref_out_opt Table	**ppvObject) = 0;
        
        virtual /* [helpcontext] */ HRESULT STDMETHODCALLTYPE Append( 
            /* [in] */ VARIANT Item) = 0;
        
        virtual /* [helpcontext] */ HRESULT STDMETHODCALLTYPE Delete( 
            /* [in] */ VARIANT Item) = 0;
        
    };
    
    
#else 	/* C style interface */
    typedef struct TablesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ADOTables * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ADOTables * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ADOTables * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ADOTables * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ADOTables * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ADOTables * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ADOTables * This,
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
        
        DECLSPEC_XFGVIRT(_Collection, get_Count)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in ADOTables * This,
            /* [retval][out] */ __RPC__out long *c);
        
        DECLSPEC_XFGVIRT(_ADOCollection, _NewEnum)
        /* [id][restricted] */ HRESULT ( STDMETHODCALLTYPE *_NewEnum )( 
            __RPC__in ADOTables * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppvObject);
        
        DECLSPEC_XFGVIRT(_ADOCollection, Refresh)
        /* [helpcontext] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in ADOTables * This);
        
        DECLSPEC_XFGVIRT(Tables, get_Item)
        /* [id][helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in ADOTables * This,
            /* [in] */ VARIANT Item,
            /* [retval][out] */ __RPC__deref_out_opt Table	**ppvObject);
        
        DECLSPEC_XFGVIRT(Tables, Append)
        /* [helpcontext] */ HRESULT ( STDMETHODCALLTYPE *Append )( 
            __RPC__in ADOTables * This,
            /* [in] */ VARIANT Item);
        
        DECLSPEC_XFGVIRT(Tables, Delete)
        /* [helpcontext] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in ADOTables * This,
            /* [in] */ VARIANT Item);
        
        END_INTERFACE
    } TablesVtbl;
    interface Tables
    {
        CONST_VTBL struct TablesVtbl *lpVtbl;
    };
    
#ifdef COBJMACROS
#define Tables_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define Tables_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define Tables_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 
#define Tables_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 
#define Tables_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 
#define Tables_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 
#define Tables_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 
#define Tables_get_Count(This,c)	\
    ( (This)->lpVtbl -> get_Count(This,c) ) 
#define Tables__NewEnum(This,ppvObject)	\
    ( (This)->lpVtbl -> _NewEnum(This,ppvObject) ) 
#define Tables_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 
#define Tables_get_Item(This,Item,ppvObject)	\
    ( (This)->lpVtbl -> get_Item(This,Item,ppvObject) ) 
#define Tables_Append(This,Item)	\
    ( (This)->lpVtbl -> Append(This,Item) ) 
#define Tables_Delete(This,Item)	\
    ( (This)->lpVtbl -> Delete(This,Item) ) 
#endif /* COBJMACROS */
#endif 	/* C style interface */
#endif 	/* __Tables_INTERFACE_DEFINED__ */
#ifndef __Columns_INTERFACE_DEFINED__
#define __Columns_INTERFACE_DEFINED__
/* interface ADOColumns */
/* [object][uuid][helpcontext][nonextensible][dual] */ 
EXTERN_C const IID IID_Columns;
#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0000061D-0000-0010-8000-00AA006D2EA4")
    ADOColumns : public _ADOCollection
    {
    public:
        virtual /* [id][helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT Item,
            /* [retval][out] */ __RPC__deref_out_opt Column	**ppvObject) = 0;
        
        virtual /* [helpcontext] */ HRESULT STDMETHODCALLTYPE Append( 
            /* [in] */ VARIANT Item,
            /* [defaultvalue][in] */ DataTypeEnum Type = adVarWChar,
            /* [defaultvalue][in] */ long DefinedSize = 0) = 0;
        
        virtual /* [helpcontext] */ HRESULT STDMETHODCALLTYPE Delete( 
            /* [in] */ VARIANT Item) = 0;
        
    };
    
    
#else 	/* C style interface */
    typedef struct ColumnsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ADOColumns * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ADOColumns * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ADOColumns * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ADOColumns * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ADOColumns * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ADOColumns * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ADOColumns * This,
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
        
        DECLSPEC_XFGVIRT(_Collection, get_Count)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in ADOColumns * This,
            /* [retval][out] */ __RPC__out long *c);
        
        DECLSPEC_XFGVIRT(_ADOCollection, _NewEnum)
        /* [id][restricted] */ HRESULT ( STDMETHODCALLTYPE *_NewEnum )( 
            __RPC__in ADOColumns * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppvObject);
        
        DECLSPEC_XFGVIRT(_ADOCollection, Refresh)
        /* [helpcontext] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in ADOColumns * This);
        
        DECLSPEC_XFGVIRT(Columns, get_Item)
        /* [id][helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in ADOColumns * This,
            /* [in] */ VARIANT Item,
            /* [retval][out] */ __RPC__deref_out_opt Column	**ppvObject);
        
        DECLSPEC_XFGVIRT(Columns, Append)
        /* [helpcontext] */ HRESULT ( STDMETHODCALLTYPE *Append )( 
            __RPC__in ADOColumns * This,
            /* [in] */ VARIANT Item,
            /* [defaultvalue][in] */ DataTypeEnum Type,
            /* [defaultvalue][in] */ long DefinedSize);
        
        DECLSPEC_XFGVIRT(Columns, Delete)
        /* [helpcontext] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in ADOColumns * This,
            /* [in] */ VARIANT Item);
        
        END_INTERFACE
    } ColumnsVtbl;
    interface Columns
    {
        CONST_VTBL struct ColumnsVtbl *lpVtbl;
    };
    
#ifdef COBJMACROS
#define Columns_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define Columns_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define Columns_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 
#define Columns_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 
#define Columns_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 
#define Columns_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 
#define Columns_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 
#define Columns_get_Count(This,c)	\
    ( (This)->lpVtbl -> get_Count(This,c) ) 
#define Columns__NewEnum(This,ppvObject)	\
    ( (This)->lpVtbl -> _NewEnum(This,ppvObject) ) 
#define Columns_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 
#define Columns_get_Item(This,Item,ppvObject)	\
    ( (This)->lpVtbl -> get_Item(This,Item,ppvObject) ) 
#define Columns_Append(This,Item,Type,DefinedSize)	\
    ( (This)->lpVtbl -> Append(This,Item,Type,DefinedSize) ) 
#define Columns_Delete(This,Item)	\
    ( (This)->lpVtbl -> Delete(This,Item) ) 
#endif /* COBJMACROS */
#endif 	/* C style interface */
#endif 	/* __Columns_INTERFACE_DEFINED__ */
#ifndef __Procedures_INTERFACE_DEFINED__
#define __Procedures_INTERFACE_DEFINED__
/* interface ADOProcedures */
/* [object][uuid][helpcontext][nonextensible][dual] */ 
EXTERN_C const IID IID_Procedures;
#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000626-0000-0010-8000-00AA006D2EA4")
    ADOProcedures : public _ADOCollection
    {
    public:
        virtual /* [id][helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT Item,
            /* [retval][out] */ __RPC__deref_out_opt ADOProcedure **ppvObject) = 0;
        
        virtual /* [helpcontext] */ HRESULT STDMETHODCALLTYPE Append( 
            /* [in] */ __RPC__in BSTR Name,
            /* [in] */ __RPC__in_opt IDispatch *Command) = 0;
        
        virtual /* [helpcontext] */ HRESULT STDMETHODCALLTYPE Delete( 
            /* [in] */ VARIANT Item) = 0;
        
    };
    
    
#else 	/* C style interface */
    typedef struct ProceduresVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ADOProcedures * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ADOProcedures * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ADOProcedures * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ADOProcedures * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ADOProcedures * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ADOProcedures * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ADOProcedures * This,
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
        
        DECLSPEC_XFGVIRT(_Collection, get_Count)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in ADOProcedures * This,
            /* [retval][out] */ __RPC__out long *c);
        
        DECLSPEC_XFGVIRT(_ADOCollection, _NewEnum)
        /* [id][restricted] */ HRESULT ( STDMETHODCALLTYPE *_NewEnum )( 
            __RPC__in ADOProcedures * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppvObject);
        
        DECLSPEC_XFGVIRT(_ADOCollection, Refresh)
        /* [helpcontext] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in ADOProcedures * This);
        
        DECLSPEC_XFGVIRT(Procedures, get_Item)
        /* [id][helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in ADOProcedures * This,
            /* [in] */ VARIANT Item,
            /* [retval][out] */ __RPC__deref_out_opt ADOProcedure **ppvObject);
        
        DECLSPEC_XFGVIRT(Procedures, Append)
        /* [helpcontext] */ HRESULT ( STDMETHODCALLTYPE *Append )( 
            __RPC__in ADOProcedures * This,
            /* [in] */ __RPC__in BSTR Name,
            /* [in] */ __RPC__in_opt IDispatch *Command);
        
        DECLSPEC_XFGVIRT(Procedures, Delete)
        /* [helpcontext] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in ADOProcedures * This,
            /* [in] */ VARIANT Item);
        
        END_INTERFACE
    } ProceduresVtbl;
    interface Procedures
    {
        CONST_VTBL struct ProceduresVtbl *lpVtbl;
    };
    
#ifdef COBJMACROS
#define Procedures_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define Procedures_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define Procedures_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 
#define Procedures_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 
#define Procedures_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 
#define Procedures_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 
#define Procedures_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 
#define Procedures_get_Count(This,c)	\
    ( (This)->lpVtbl -> get_Count(This,c) ) 
#define Procedures__NewEnum(This,ppvObject)	\
    ( (This)->lpVtbl -> _NewEnum(This,ppvObject) ) 
#define Procedures_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 
#define Procedures_get_Item(This,Item,ppvObject)	\
    ( (This)->lpVtbl -> get_Item(This,Item,ppvObject) ) 
#define Procedures_Append(This,Name,Command)	\
    ( (This)->lpVtbl -> Append(This,Name,Command) ) 
#define Procedures_Delete(This,Item)	\
    ( (This)->lpVtbl -> Delete(This,Item) ) 
#endif /* COBJMACROS */
#endif 	/* C style interface */
#endif 	/* __Procedures_INTERFACE_DEFINED__ */
#ifndef __Views_INTERFACE_DEFINED__
#define __Views_INTERFACE_DEFINED__
/* interface ADOViews */
/* [object][uuid][helpcontext][nonextensible][dual] */ 
EXTERN_C const IID IID_Views;
#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000614-0000-0010-8000-00AA006D2EA4")
    ADOViews : public _ADOCollection
    {
    public:
        virtual /* [id][helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT Item,
            /* [retval][out] */ __RPC__deref_out_opt ADOView **ppvObject) = 0;
        
        virtual /* [helpcontext] */ HRESULT STDMETHODCALLTYPE Append( 
            /* [in] */ __RPC__in BSTR Name,
            /* [in] */ __RPC__in_opt IDispatch *Command) = 0;
        
        virtual /* [helpcontext] */ HRESULT STDMETHODCALLTYPE Delete( 
            /* [in] */ VARIANT Item) = 0;
        
    };
    
    
#else 	/* C style interface */
    typedef struct ViewsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ADOViews * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ADOViews * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ADOViews * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ADOViews * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ADOViews * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ADOViews * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ADOViews * This,
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
        
        DECLSPEC_XFGVIRT(_Collection, get_Count)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in ADOViews * This,
            /* [retval][out] */ __RPC__out long *c);
        
        DECLSPEC_XFGVIRT(_ADOCollection, _NewEnum)
        /* [id][restricted] */ HRESULT ( STDMETHODCALLTYPE *_NewEnum )( 
            __RPC__in ADOViews * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppvObject);
        
        DECLSPEC_XFGVIRT(_ADOCollection, Refresh)
        /* [helpcontext] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in ADOViews * This);
        
        DECLSPEC_XFGVIRT(Views, get_Item)
        /* [id][helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in ADOViews * This,
            /* [in] */ VARIANT Item,
            /* [retval][out] */ __RPC__deref_out_opt ADOView **ppvObject);
        
        DECLSPEC_XFGVIRT(Views, Append)
        /* [helpcontext] */ HRESULT ( STDMETHODCALLTYPE *Append )( 
            __RPC__in ADOViews * This,
            /* [in] */ __RPC__in BSTR Name,
            /* [in] */ __RPC__in_opt IDispatch *Command);
        
        DECLSPEC_XFGVIRT(Views, Delete)
        /* [helpcontext] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in ADOViews * This,
            /* [in] */ VARIANT Item);
        
        END_INTERFACE
    } ViewsVtbl;
    interface Views
    {
        CONST_VTBL struct ViewsVtbl *lpVtbl;
    };
    
#ifdef COBJMACROS
#define Views_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define Views_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define Views_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 
#define Views_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 
#define Views_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 
#define Views_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 
#define Views_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 
#define Views_get_Count(This,c)	\
    ( (This)->lpVtbl -> get_Count(This,c) ) 
#define Views__NewEnum(This,ppvObject)	\
    ( (This)->lpVtbl -> _NewEnum(This,ppvObject) ) 
#define Views_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 
#define Views_get_Item(This,Item,ppvObject)	\
    ( (This)->lpVtbl -> get_Item(This,Item,ppvObject) ) 
#define Views_Append(This,Name,Command)	\
    ( (This)->lpVtbl -> Append(This,Name,Command) ) 
#define Views_Delete(This,Item)	\
    ( (This)->lpVtbl -> Delete(This,Item) ) 
#endif /* COBJMACROS */
#endif 	/* C style interface */
#endif 	/* __Views_INTERFACE_DEFINED__ */
#ifndef __Indexes_INTERFACE_DEFINED__
#define __Indexes_INTERFACE_DEFINED__
/* interface ADOIndexes */
/* [object][uuid][helpcontext][nonextensible][dual] */ 
EXTERN_C const IID IID_Indexes;
#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000620-0000-0010-8000-00AA006D2EA4")
    ADOIndexes : public _ADOCollection
    {
    public:
        virtual /* [id][helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT Item,
            /* [retval][out] */ __RPC__deref_out_opt Index	**ppvObject) = 0;
        
        virtual /* [helpcontext] */ HRESULT STDMETHODCALLTYPE Append( 
            /* [in] */ VARIANT Item,
            /* [optional][in] */ VARIANT columns) = 0;
        
        virtual /* [helpcontext] */ HRESULT STDMETHODCALLTYPE Delete( 
            /* [in] */ VARIANT Item) = 0;
        
    };
    
    
#else 	/* C style interface */
    typedef struct IndexesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ADOIndexes * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ADOIndexes * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ADOIndexes * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ADOIndexes * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ADOIndexes * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ADOIndexes * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ADOIndexes * This,
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
        
        DECLSPEC_XFGVIRT(_Collection, get_Count)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in ADOIndexes * This,
            /* [retval][out] */ __RPC__out long *c);
        
        DECLSPEC_XFGVIRT(_ADOCollection, _NewEnum)
        /* [id][restricted] */ HRESULT ( STDMETHODCALLTYPE *_NewEnum )( 
            __RPC__in ADOIndexes * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppvObject);
        
        DECLSPEC_XFGVIRT(_ADOCollection, Refresh)
        /* [helpcontext] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in ADOIndexes * This);
        
        DECLSPEC_XFGVIRT(Indexes, get_Item)
        /* [id][helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in ADOIndexes * This,
            /* [in] */ VARIANT Item,
            /* [retval][out] */ __RPC__deref_out_opt Index	**ppvObject);
        
        DECLSPEC_XFGVIRT(Indexes, Append)
        /* [helpcontext] */ HRESULT ( STDMETHODCALLTYPE *Append )( 
            __RPC__in ADOIndexes * This,
            /* [in] */ VARIANT Item,
            /* [optional][in] */ VARIANT columns);
        
        DECLSPEC_XFGVIRT(Indexes, Delete)
        /* [helpcontext] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in ADOIndexes * This,
            /* [in] */ VARIANT Item);
        
        END_INTERFACE
    } IndexesVtbl;
    interface Indexes
    {
        CONST_VTBL struct IndexesVtbl *lpVtbl;
    };
    
#ifdef COBJMACROS
#define Indexes_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define Indexes_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define Indexes_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 
#define Indexes_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 
#define Indexes_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 
#define Indexes_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 
#define Indexes_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 
#define Indexes_get_Count(This,c)	\
    ( (This)->lpVtbl -> get_Count(This,c) ) 
#define Indexes__NewEnum(This,ppvObject)	\
    ( (This)->lpVtbl -> _NewEnum(This,ppvObject) ) 
#define Indexes_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 
#define Indexes_get_Item(This,Item,ppvObject)	\
    ( (This)->lpVtbl -> get_Item(This,Item,ppvObject) ) 
#define Indexes_Append(This,Item,columns)	\
    ( (This)->lpVtbl -> Append(This,Item,columns) ) 
#define Indexes_Delete(This,Item)	\
    ( (This)->lpVtbl -> Delete(This,Item) ) 
#endif /* COBJMACROS */
#endif 	/* C style interface */
#endif 	/* __Indexes_INTERFACE_DEFINED__ */
#ifndef __Keys_INTERFACE_DEFINED__
#define __Keys_INTERFACE_DEFINED__
/* interface ADOKeys */
/* [object][uuid][helpcontext][nonextensible][dual] */ 
EXTERN_C const IID IID_Keys;
#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000623-0000-0010-8000-00AA006D2EA4")
    ADOKeys : public _ADOCollection
    {
    public:
        virtual /* [id][helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT Item,
            /* [retval][out] */ __RPC__deref_out_opt Key	**ppvObject) = 0;
        
        virtual /* [helpcontext] */ HRESULT STDMETHODCALLTYPE Append( 
            /* [in] */ VARIANT Item,
            /* [defaultvalue][in] */ KeyTypeEnum Type,
            /* [optional][in] */ VARIANT Column,
            /* [defaultvalue][in] */ __RPC__in BSTR RelatedADOTable = (BSTR)L"",
            /* [defaultvalue][in] */ __RPC__in BSTR RelatedADOColumn = (BSTR)L"") = 0;
        
        virtual /* [helpcontext] */ HRESULT STDMETHODCALLTYPE Delete( 
            /* [in] */ VARIANT Item) = 0;
        
    };
    
    
#else 	/* C style interface */
    typedef struct KeysVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ADOKeys * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ADOKeys * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ADOKeys * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ADOKeys * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ADOKeys * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ADOKeys * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ADOKeys * This,
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
        
        DECLSPEC_XFGVIRT(_Collection, get_Count)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in ADOKeys * This,
            /* [retval][out] */ __RPC__out long *c);
        
        DECLSPEC_XFGVIRT(_ADOCollection, _NewEnum)
        /* [id][restricted] */ HRESULT ( STDMETHODCALLTYPE *_NewEnum )( 
            __RPC__in ADOKeys * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppvObject);
        
        DECLSPEC_XFGVIRT(_ADOCollection, Refresh)
        /* [helpcontext] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in ADOKeys * This);
        
        DECLSPEC_XFGVIRT(Keys, get_Item)
        /* [id][helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in ADOKeys * This,
            /* [in] */ VARIANT Item,
            /* [retval][out] */ __RPC__deref_out_opt Key	**ppvObject);
        
        DECLSPEC_XFGVIRT(Keys, Append)
        /* [helpcontext] */ HRESULT ( STDMETHODCALLTYPE *Append )( 
            __RPC__in ADOKeys * This,
            /* [in] */ VARIANT Item,
            /* [defaultvalue][in] */ KeyTypeEnum Type,
            /* [optional][in] */ VARIANT Column,
            /* [defaultvalue][in] */ __RPC__in BSTR RelatedTable,
            /* [defaultvalue][in] */ __RPC__in BSTR RelatedColumn);
        
        DECLSPEC_XFGVIRT(Keys, Delete)
        /* [helpcontext] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in ADOKeys * This,
            /* [in] */ VARIANT Item);
        
        END_INTERFACE
    } KeysVtbl;
    interface Keys
    {
        CONST_VTBL struct KeysVtbl *lpVtbl;
    };
    
#ifdef COBJMACROS
#define Keys_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define Keys_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define Keys_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 
#define Keys_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 
#define Keys_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 
#define Keys_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 
#define Keys_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 
#define Keys_get_Count(This,c)	\
    ( (This)->lpVtbl -> get_Count(This,c) ) 
#define Keys__NewEnum(This,ppvObject)	\
    ( (This)->lpVtbl -> _NewEnum(This,ppvObject) ) 
#define Keys_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 
#define Keys_get_Item(This,Item,ppvObject)	\
    ( (This)->lpVtbl -> get_Item(This,Item,ppvObject) ) 
#define Keys_Append(This,Item,Type,Column,RelatedTable,RelatedColumn)	\
    ( (This)->lpVtbl -> Append(This,Item,Type,Column,RelatedTable,RelatedColumn) ) 
#define Keys_Delete(This,Item)	\
    ( (This)->lpVtbl -> Delete(This,Item) ) 
#endif /* COBJMACROS */
#endif 	/* C style interface */
#endif 	/* __Keys_INTERFACE_DEFINED__ */
#ifndef __Users_INTERFACE_DEFINED__
#define __Users_INTERFACE_DEFINED__
/* interface ADOUsers */
/* [object][uuid][helpcontext][nonextensible][dual] */ 
EXTERN_C const IID IID_Users;
#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0000061A-0000-0010-8000-00AA006D2EA4")
    ADOUsers : public _ADOCollection
    {
    public:
        virtual /* [id][helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT Item,
            /* [retval][out] */ __RPC__deref_out_opt User	**ppvObject) = 0;
        
        virtual /* [helpcontext] */ HRESULT STDMETHODCALLTYPE Append( 
            /* [in] */ VARIANT Item,
            /* [defaultvalue][in] */ __RPC__in BSTR Password = (BSTR)L"") = 0;
        
        virtual /* [helpcontext] */ HRESULT STDMETHODCALLTYPE Delete( 
            /* [in] */ VARIANT Item) = 0;
        
    };
    
    
#else 	/* C style interface */
    typedef struct UsersVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ADOUsers * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ADOUsers * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ADOUsers * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ADOUsers * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ADOUsers * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ADOUsers * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ADOUsers * This,
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
        
        DECLSPEC_XFGVIRT(_Collection, get_Count)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in ADOUsers * This,
            /* [retval][out] */ __RPC__out long *c);
        
        DECLSPEC_XFGVIRT(_ADOCollection, _NewEnum)
        /* [id][restricted] */ HRESULT ( STDMETHODCALLTYPE *_NewEnum )( 
            __RPC__in ADOUsers * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppvObject);
        
        DECLSPEC_XFGVIRT(_ADOCollection, Refresh)
        /* [helpcontext] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in ADOUsers * This);
        
        DECLSPEC_XFGVIRT(Users, get_Item)
        /* [id][helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in ADOUsers * This,
            /* [in] */ VARIANT Item,
            /* [retval][out] */ __RPC__deref_out_opt User	**ppvObject);
        
        DECLSPEC_XFGVIRT(Users, Append)
        /* [helpcontext] */ HRESULT ( STDMETHODCALLTYPE *Append )( 
            __RPC__in ADOUsers * This,
            /* [in] */ VARIANT Item,
            /* [defaultvalue][in] */ __RPC__in BSTR Password);
        
        DECLSPEC_XFGVIRT(Users, Delete)
        /* [helpcontext] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in ADOUsers * This,
            /* [in] */ VARIANT Item);
        
        END_INTERFACE
    } UsersVtbl;
    interface Users
    {
        CONST_VTBL struct UsersVtbl *lpVtbl;
    };
    
#ifdef COBJMACROS
#define Users_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define Users_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define Users_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 
#define Users_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 
#define Users_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 
#define Users_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 
#define Users_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 
#define Users_get_Count(This,c)	\
    ( (This)->lpVtbl -> get_Count(This,c) ) 
#define Users__NewEnum(This,ppvObject)	\
    ( (This)->lpVtbl -> _NewEnum(This,ppvObject) ) 
#define Users_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 
#define Users_get_Item(This,Item,ppvObject)	\
    ( (This)->lpVtbl -> get_Item(This,Item,ppvObject) ) 
#define Users_Append(This,Item,Password)	\
    ( (This)->lpVtbl -> Append(This,Item,Password) ) 
#define Users_Delete(This,Item)	\
    ( (This)->lpVtbl -> Delete(This,Item) ) 
#endif /* COBJMACROS */
#endif 	/* C style interface */
#endif 	/* __Users_INTERFACE_DEFINED__ */
#ifndef __Groups_INTERFACE_DEFINED__
#define __Groups_INTERFACE_DEFINED__
/* interface ADOGroups */
/* [object][uuid][helpcontext][nonextensible][dual] */ 
EXTERN_C const IID IID_Groups;
#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000617-0000-0010-8000-00AA006D2EA4")
    ADOGroups : public _ADOCollection
    {
    public:
        virtual /* [id][helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT Item,
            /* [retval][out] */ __RPC__deref_out_opt Group	**ppvObject) = 0;
        
        virtual /* [helpcontext] */ HRESULT STDMETHODCALLTYPE Append( 
            /* [in] */ VARIANT Item) = 0;
        
        virtual /* [helpcontext] */ HRESULT STDMETHODCALLTYPE Delete( 
            /* [in] */ VARIANT Item) = 0;
        
    };
    
    
#else 	/* C style interface */
    typedef struct GroupsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ADOGroups * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ADOGroups * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ADOGroups * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ADOGroups * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ADOGroups * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ADOGroups * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ADOGroups * This,
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
        
        DECLSPEC_XFGVIRT(_Collection, get_Count)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in ADOGroups * This,
            /* [retval][out] */ __RPC__out long *c);
        
        DECLSPEC_XFGVIRT(_ADOCollection, _NewEnum)
        /* [id][restricted] */ HRESULT ( STDMETHODCALLTYPE *_NewEnum )( 
            __RPC__in ADOGroups * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppvObject);
        
        DECLSPEC_XFGVIRT(_ADOCollection, Refresh)
        /* [helpcontext] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in ADOGroups * This);
        
        DECLSPEC_XFGVIRT(Groups, get_Item)
        /* [id][helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in ADOGroups * This,
            /* [in] */ VARIANT Item,
            /* [retval][out] */ __RPC__deref_out_opt Group	**ppvObject);
        
        DECLSPEC_XFGVIRT(Groups, Append)
        /* [helpcontext] */ HRESULT ( STDMETHODCALLTYPE *Append )( 
            __RPC__in ADOGroups * This,
            /* [in] */ VARIANT Item);
        
        DECLSPEC_XFGVIRT(Groups, Delete)
        /* [helpcontext] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in ADOGroups * This,
            /* [in] */ VARIANT Item);
        
        END_INTERFACE
    } GroupsVtbl;
    interface Groups
    {
        CONST_VTBL struct GroupsVtbl *lpVtbl;
    };
    
#ifdef COBJMACROS
#define Groups_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define Groups_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define Groups_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 
#define Groups_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 
#define Groups_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 
#define Groups_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 
#define Groups_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 
#define Groups_get_Count(This,c)	\
    ( (This)->lpVtbl -> get_Count(This,c) ) 
#define Groups__NewEnum(This,ppvObject)	\
    ( (This)->lpVtbl -> _NewEnum(This,ppvObject) ) 
#define Groups_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 
#define Groups_get_Item(This,Item,ppvObject)	\
    ( (This)->lpVtbl -> get_Item(This,Item,ppvObject) ) 
#define Groups_Append(This,Item)	\
    ( (This)->lpVtbl -> Append(This,Item) ) 
#define Groups_Delete(This,Item)	\
    ( (This)->lpVtbl -> Delete(This,Item) ) 
#endif /* COBJMACROS */
#endif 	/* C style interface */
#endif 	/* __Groups_INTERFACE_DEFINED__ */
#ifndef __Properties_INTERFACE_DEFINED__
#define __Properties_INTERFACE_DEFINED__
/* interface ADOProperties */
/* [object][uuid][helpcontext][nonextensible][dual] */ 
EXTERN_C const IID IID_Properties;
#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000504-0000-0010-8000-00AA006D2EA4")
    ADOProperties : public _ADOCollection
    {
    public:
        virtual /* [id][helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT Item,
            /* [retval][out] */ __RPC__deref_out_opt ADOProperty **ppvObject) = 0;
        
    };
    
    
#else 	/* C style interface */
    typedef struct PropertiesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ADOProperties * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ADOProperties * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ADOProperties * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ADOProperties * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ADOProperties * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ADOProperties * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ADOProperties * This,
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
        
        DECLSPEC_XFGVIRT(_Collection, get_Count)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in ADOProperties * This,
            /* [retval][out] */ __RPC__out long *c);
        
        DECLSPEC_XFGVIRT(_ADOCollection, _NewEnum)
        /* [id][restricted] */ HRESULT ( STDMETHODCALLTYPE *_NewEnum )( 
            __RPC__in ADOProperties * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppvObject);
        
        DECLSPEC_XFGVIRT(_ADOCollection, Refresh)
        /* [helpcontext] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in ADOProperties * This);
        
        DECLSPEC_XFGVIRT(Properties, get_Item)
        /* [id][helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in ADOProperties * This,
            /* [in] */ VARIANT Item,
            /* [retval][out] */ __RPC__deref_out_opt ADOProperty **ppvObject);
        
        END_INTERFACE
    } PropertiesVtbl;
    interface Properties
    {
        CONST_VTBL struct PropertiesVtbl *lpVtbl;
    };
    
#ifdef COBJMACROS
#define Properties_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define Properties_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define Properties_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 
#define Properties_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 
#define Properties_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 
#define Properties_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 
#define Properties_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 
#define Properties_get_Count(This,c)	\
    ( (This)->lpVtbl -> get_Count(This,c) ) 
#define Properties__NewEnum(This,ppvObject)	\
    ( (This)->lpVtbl -> _NewEnum(This,ppvObject) ) 
#define Properties_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 
#define Properties_get_Item(This,Item,ppvObject)	\
    ( (This)->lpVtbl -> get_Item(This,Item,ppvObject) ) 
#endif /* COBJMACROS */
#endif 	/* C style interface */
#endif 	/* __Properties_INTERFACE_DEFINED__ */
#endif /* __ADOX_LIBRARY_DEFINED__ */
/* interface __MIDL_itf_adocat_0000_0001 */
/* [local] */ 
#pragma warning(pop) 
extern RPC_IF_HANDLE __MIDL_itf_adocat_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_adocat_0000_0001_v0_0_s_ifspec;
/* Additional Prototypes for ALL interfaces */
/* end of Additional Prototypes */
#ifdef __cplusplus
}
#endif
#endif
/***********************************
Forwards
*/
#define ADOCatalog _ADOCatalog
#define ADOTable _ADOTable
#define ADOGroup _ADOGroup
#define ADOUser _ADOUser
#define ADOIndex _ADOIndex
#define ADOColumn _ADOColumn
#define ADOKey _ADOKey
#define ADOParameter _ADOParameter
#define ADOCollection _ADOCollection
#define ADODynaCollection _ADODynaCollection


#endif // _ADOCTINT_H_
