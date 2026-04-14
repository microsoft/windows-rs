

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


#ifndef __adomd_h__
#define __adomd_h__

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

#ifndef __ICatalog_FWD_DEFINED__
#define __ICatalog_FWD_DEFINED__
typedef interface ICatalog ICatalog;

#endif 	/* __ICatalog_FWD_DEFINED__ */


#ifndef __ICellset_FWD_DEFINED__
#define __ICellset_FWD_DEFINED__
typedef interface ICellset ICellset;

#endif 	/* __ICellset_FWD_DEFINED__ */


#ifndef __Cell_FWD_DEFINED__
#define __Cell_FWD_DEFINED__
typedef interface Cell Cell;

#endif 	/* __Cell_FWD_DEFINED__ */


#ifndef __Axis_FWD_DEFINED__
#define __Axis_FWD_DEFINED__
typedef interface Axis Axis;

#endif 	/* __Axis_FWD_DEFINED__ */


#ifndef __Position_FWD_DEFINED__
#define __Position_FWD_DEFINED__
typedef interface Position Position;

#endif 	/* __Position_FWD_DEFINED__ */


#ifndef __Member_FWD_DEFINED__
#define __Member_FWD_DEFINED__
typedef interface Member Member;

#endif 	/* __Member_FWD_DEFINED__ */


#ifndef __Level_FWD_DEFINED__
#define __Level_FWD_DEFINED__
typedef interface Level Level;

#endif 	/* __Level_FWD_DEFINED__ */


#ifndef __CubeDef25_FWD_DEFINED__
#define __CubeDef25_FWD_DEFINED__
typedef interface CubeDef25 CubeDef25;

#endif 	/* __CubeDef25_FWD_DEFINED__ */


#ifndef __CubeDef_FWD_DEFINED__
#define __CubeDef_FWD_DEFINED__
typedef interface CubeDef CubeDef;

#endif 	/* __CubeDef_FWD_DEFINED__ */


#ifndef __Dimension_FWD_DEFINED__
#define __Dimension_FWD_DEFINED__
typedef interface Dimension Dimension;

#endif 	/* __Dimension_FWD_DEFINED__ */


#ifndef __Hierarchy_FWD_DEFINED__
#define __Hierarchy_FWD_DEFINED__
typedef interface Hierarchy Hierarchy;

#endif 	/* __Hierarchy_FWD_DEFINED__ */


#ifndef __MD_Collection_FWD_DEFINED__
#define __MD_Collection_FWD_DEFINED__
typedef interface MD_Collection MD_Collection;

#endif 	/* __MD_Collection_FWD_DEFINED__ */


#ifndef __Members_FWD_DEFINED__
#define __Members_FWD_DEFINED__
typedef interface Members Members;

#endif 	/* __Members_FWD_DEFINED__ */


#ifndef __Levels_FWD_DEFINED__
#define __Levels_FWD_DEFINED__
typedef interface Levels Levels;

#endif 	/* __Levels_FWD_DEFINED__ */


#ifndef __Axes_FWD_DEFINED__
#define __Axes_FWD_DEFINED__
typedef interface Axes Axes;

#endif 	/* __Axes_FWD_DEFINED__ */


#ifndef __Positions_FWD_DEFINED__
#define __Positions_FWD_DEFINED__
typedef interface Positions Positions;

#endif 	/* __Positions_FWD_DEFINED__ */


#ifndef __Hierarchies_FWD_DEFINED__
#define __Hierarchies_FWD_DEFINED__
typedef interface Hierarchies Hierarchies;

#endif 	/* __Hierarchies_FWD_DEFINED__ */


#ifndef __Dimensions_FWD_DEFINED__
#define __Dimensions_FWD_DEFINED__
typedef interface Dimensions Dimensions;

#endif 	/* __Dimensions_FWD_DEFINED__ */


#ifndef __CubeDefs_FWD_DEFINED__
#define __CubeDefs_FWD_DEFINED__
typedef interface CubeDefs CubeDefs;

#endif 	/* __CubeDefs_FWD_DEFINED__ */


#ifndef __Catalog_FWD_DEFINED__
#define __Catalog_FWD_DEFINED__

#ifdef __cplusplus
typedef class Catalog Catalog;
#else
typedef struct Catalog Catalog;
#endif /* __cplusplus */

#endif 	/* __Catalog_FWD_DEFINED__ */


#ifndef __Cellset_FWD_DEFINED__
#define __Cellset_FWD_DEFINED__

#ifdef __cplusplus
typedef class Cellset Cellset;
#else
typedef struct Cellset Cellset;
#endif /* __cplusplus */

#endif 	/* __Cellset_FWD_DEFINED__ */


#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_adomd_0000_0000 */
/* [local] */ 




















#define TARGET_IS_NT40_OR_LATER   1


extern RPC_IF_HANDLE __MIDL_itf_adomd_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_adomd_0000_0000_v0_0_s_ifspec;


#ifndef __ADOMD_LIBRARY_DEFINED__
#define __ADOMD_LIBRARY_DEFINED__

/* library ADOMD */
/* [helpstring][version][uuid] */ 

typedef /* [uuid][helpcontext] */  DECLSPEC_UUID("000002AE-0000-0010-8000-00AA006D2EA4") 
enum MemberTypeEnum
    {
        adMemberUnknown	= 0,
        adMemberRegular	= 0x1,
        adMemberAll	= 0x2,
        adMemberMeasure	= 0x3,
        adMemberFormula	= 0x4
    } 	MemberTypeEnum;

typedef /* [uuid][helpcontext] */  DECLSPEC_UUID("C23BBD43-E494-4d00-B4D1-6C9A2CE17CE3") 
enum SchemaObjectTypeEnum
    {
        adObjectTypeDimension	= 1,
        adObjectTypeHierarchy	= 2,
        adObjectTypeLevel	= 3,
        adObjectTypeMember	= 4
    } 	SchemaObjectTypeEnum;


EXTERN_C const IID LIBID_ADOMD;

#ifndef __ICatalog_INTERFACE_DEFINED__
#define __ICatalog_INTERFACE_DEFINED__

/* interface ICatalog */
/* [unique][helpcontext][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_ICatalog;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("228136B1-8BD3-11D0-B4EF-00A0C9138CA4")
    ICatalog : public IDispatch
    {
    public:
        virtual /* [helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr) = 0;
        
        virtual /* [helpcontext][propputref] */ HRESULT STDMETHODCALLTYPE putref_ActiveConnection( 
            /* [in] */ __RPC__in_opt IDispatch *pconn) = 0;
        
        virtual /* [helpcontext][propput] */ HRESULT STDMETHODCALLTYPE put_ActiveConnection( 
            /* [in] */ __RPC__in BSTR bstrConn) = 0;
        
        virtual /* [helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_ActiveConnection( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppConn) = 0;
        
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_CubeDefs( 
            /* [retval][out] */ __RPC__deref_out_opt CubeDefs **ppvObject) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICatalogVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICatalog * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICatalog * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICatalog * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ICatalog * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ICatalog * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ICatalog * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ICatalog * This,
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
        
        DECLSPEC_XFGVIRT(ICatalog, get_Name)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in ICatalog * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        DECLSPEC_XFGVIRT(ICatalog, putref_ActiveConnection)
        /* [helpcontext][propputref] */ HRESULT ( STDMETHODCALLTYPE *putref_ActiveConnection )( 
            __RPC__in ICatalog * This,
            /* [in] */ __RPC__in_opt IDispatch *pconn);
        
        DECLSPEC_XFGVIRT(ICatalog, put_ActiveConnection)
        /* [helpcontext][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ActiveConnection )( 
            __RPC__in ICatalog * This,
            /* [in] */ __RPC__in BSTR bstrConn);
        
        DECLSPEC_XFGVIRT(ICatalog, get_ActiveConnection)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ActiveConnection )( 
            __RPC__in ICatalog * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppConn);
        
        DECLSPEC_XFGVIRT(ICatalog, get_CubeDefs)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CubeDefs )( 
            __RPC__in ICatalog * This,
            /* [retval][out] */ __RPC__deref_out_opt CubeDefs **ppvObject);
        
        END_INTERFACE
    } ICatalogVtbl;

    interface ICatalog
    {
        CONST_VTBL struct ICatalogVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICatalog_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICatalog_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICatalog_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICatalog_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ICatalog_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ICatalog_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ICatalog_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ICatalog_get_Name(This,pbstr)	\
    ( (This)->lpVtbl -> get_Name(This,pbstr) ) 

#define ICatalog_putref_ActiveConnection(This,pconn)	\
    ( (This)->lpVtbl -> putref_ActiveConnection(This,pconn) ) 

#define ICatalog_put_ActiveConnection(This,bstrConn)	\
    ( (This)->lpVtbl -> put_ActiveConnection(This,bstrConn) ) 

#define ICatalog_get_ActiveConnection(This,ppConn)	\
    ( (This)->lpVtbl -> get_ActiveConnection(This,ppConn) ) 

#define ICatalog_get_CubeDefs(This,ppvObject)	\
    ( (This)->lpVtbl -> get_CubeDefs(This,ppvObject) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICatalog_INTERFACE_DEFINED__ */


#ifndef __ICellset_INTERFACE_DEFINED__
#define __ICellset_INTERFACE_DEFINED__

/* interface ICellset */
/* [unique][helpcontext][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_ICellset;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2281372A-8BD3-11D0-B4EF-00A0C9138CA4")
    ICellset : public IDispatch
    {
    public:
        virtual /* [helpcontext][id][vararg][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ __RPC__deref_in_opt SAFEARRAY * *idx,
            /* [retval][out] */ __RPC__deref_out_opt Cell **ppvObject) = 0;
        
        virtual /* [helpcontext] */ HRESULT STDMETHODCALLTYPE Open( 
            /* [optional][in] */ VARIANT DataSource,
            /* [optional][in] */ VARIANT ActiveConnection) = 0;
        
        virtual /* [helpcontext] */ HRESULT STDMETHODCALLTYPE Close( void) = 0;
        
        virtual /* [helpcontext][propputref] */ HRESULT STDMETHODCALLTYPE putref_Source( 
            /* [in] */ __RPC__in_opt IDispatch *pcmd) = 0;
        
        virtual /* [helpcontext][propput] */ HRESULT STDMETHODCALLTYPE put_Source( 
            /* [in] */ __RPC__in BSTR bstrCmd) = 0;
        
        virtual /* [helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_Source( 
            /* [retval][out] */ __RPC__out VARIANT *pvSource) = 0;
        
        virtual /* [helpcontext][propputref] */ HRESULT STDMETHODCALLTYPE putref_ActiveConnection( 
            /* [in] */ __RPC__in_opt IDispatch *pconn) = 0;
        
        virtual /* [helpcontext][propput] */ HRESULT STDMETHODCALLTYPE put_ActiveConnection( 
            /* [in] */ __RPC__in BSTR bstrConn) = 0;
        
        virtual /* [helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_ActiveConnection( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppConn) = 0;
        
        virtual /* [helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_State( 
            /* [retval][out] */ __RPC__out LONG *plState) = 0;
        
        virtual /* [helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_Axes( 
            /* [retval][out] */ __RPC__deref_out_opt Axes **ppvObject) = 0;
        
        virtual /* [helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_FilterAxis( 
            /* [retval][out] */ __RPC__deref_out_opt Axis **ppvObject) = 0;
        
        virtual /* [helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_Properties( 
            /* [retval][out] */ __RPC__deref_out_opt /* external definition not present */ Properties **ppvObject) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICellsetVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICellset * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICellset * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICellset * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ICellset * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ICellset * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ICellset * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ICellset * This,
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
        
        DECLSPEC_XFGVIRT(ICellset, get_Item)
        /* [helpcontext][id][vararg][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in ICellset * This,
            /* [in] */ __RPC__deref_in_opt SAFEARRAY * *idx,
            /* [retval][out] */ __RPC__deref_out_opt Cell **ppvObject);
        
        DECLSPEC_XFGVIRT(ICellset, Open)
        /* [helpcontext] */ HRESULT ( STDMETHODCALLTYPE *Open )( 
            __RPC__in ICellset * This,
            /* [optional][in] */ VARIANT DataSource,
            /* [optional][in] */ VARIANT ActiveConnection);
        
        DECLSPEC_XFGVIRT(ICellset, Close)
        /* [helpcontext] */ HRESULT ( STDMETHODCALLTYPE *Close )( 
            __RPC__in ICellset * This);
        
        DECLSPEC_XFGVIRT(ICellset, putref_Source)
        /* [helpcontext][propputref] */ HRESULT ( STDMETHODCALLTYPE *putref_Source )( 
            __RPC__in ICellset * This,
            /* [in] */ __RPC__in_opt IDispatch *pcmd);
        
        DECLSPEC_XFGVIRT(ICellset, put_Source)
        /* [helpcontext][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Source )( 
            __RPC__in ICellset * This,
            /* [in] */ __RPC__in BSTR bstrCmd);
        
        DECLSPEC_XFGVIRT(ICellset, get_Source)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Source )( 
            __RPC__in ICellset * This,
            /* [retval][out] */ __RPC__out VARIANT *pvSource);
        
        DECLSPEC_XFGVIRT(ICellset, putref_ActiveConnection)
        /* [helpcontext][propputref] */ HRESULT ( STDMETHODCALLTYPE *putref_ActiveConnection )( 
            __RPC__in ICellset * This,
            /* [in] */ __RPC__in_opt IDispatch *pconn);
        
        DECLSPEC_XFGVIRT(ICellset, put_ActiveConnection)
        /* [helpcontext][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ActiveConnection )( 
            __RPC__in ICellset * This,
            /* [in] */ __RPC__in BSTR bstrConn);
        
        DECLSPEC_XFGVIRT(ICellset, get_ActiveConnection)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ActiveConnection )( 
            __RPC__in ICellset * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppConn);
        
        DECLSPEC_XFGVIRT(ICellset, get_State)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_State )( 
            __RPC__in ICellset * This,
            /* [retval][out] */ __RPC__out LONG *plState);
        
        DECLSPEC_XFGVIRT(ICellset, get_Axes)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Axes )( 
            __RPC__in ICellset * This,
            /* [retval][out] */ __RPC__deref_out_opt Axes **ppvObject);
        
        DECLSPEC_XFGVIRT(ICellset, get_FilterAxis)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FilterAxis )( 
            __RPC__in ICellset * This,
            /* [retval][out] */ __RPC__deref_out_opt Axis **ppvObject);
        
        DECLSPEC_XFGVIRT(ICellset, get_Properties)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Properties )( 
            __RPC__in ICellset * This,
            /* [retval][out] */ __RPC__deref_out_opt /* external definition not present */ Properties **ppvObject);
        
        END_INTERFACE
    } ICellsetVtbl;

    interface ICellset
    {
        CONST_VTBL struct ICellsetVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICellset_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICellset_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICellset_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICellset_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ICellset_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ICellset_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ICellset_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ICellset_get_Item(This,idx,ppvObject)	\
    ( (This)->lpVtbl -> get_Item(This,idx,ppvObject) ) 

#define ICellset_Open(This,DataSource,ActiveConnection)	\
    ( (This)->lpVtbl -> Open(This,DataSource,ActiveConnection) ) 

#define ICellset_Close(This)	\
    ( (This)->lpVtbl -> Close(This) ) 

#define ICellset_putref_Source(This,pcmd)	\
    ( (This)->lpVtbl -> putref_Source(This,pcmd) ) 

#define ICellset_put_Source(This,bstrCmd)	\
    ( (This)->lpVtbl -> put_Source(This,bstrCmd) ) 

#define ICellset_get_Source(This,pvSource)	\
    ( (This)->lpVtbl -> get_Source(This,pvSource) ) 

#define ICellset_putref_ActiveConnection(This,pconn)	\
    ( (This)->lpVtbl -> putref_ActiveConnection(This,pconn) ) 

#define ICellset_put_ActiveConnection(This,bstrConn)	\
    ( (This)->lpVtbl -> put_ActiveConnection(This,bstrConn) ) 

#define ICellset_get_ActiveConnection(This,ppConn)	\
    ( (This)->lpVtbl -> get_ActiveConnection(This,ppConn) ) 

#define ICellset_get_State(This,plState)	\
    ( (This)->lpVtbl -> get_State(This,plState) ) 

#define ICellset_get_Axes(This,ppvObject)	\
    ( (This)->lpVtbl -> get_Axes(This,ppvObject) ) 

#define ICellset_get_FilterAxis(This,ppvObject)	\
    ( (This)->lpVtbl -> get_FilterAxis(This,ppvObject) ) 

#define ICellset_get_Properties(This,ppvObject)	\
    ( (This)->lpVtbl -> get_Properties(This,ppvObject) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICellset_INTERFACE_DEFINED__ */


#ifndef __Cell_INTERFACE_DEFINED__
#define __Cell_INTERFACE_DEFINED__

/* interface Cell */
/* [unique][helpcontext][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_Cell;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2281372E-8BD3-11D0-B4EF-00A0C9138CA4")
    Cell : public IDispatch
    {
    public:
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_Value( 
            /* [retval][out] */ __RPC__out VARIANT *pvar) = 0;
        
        virtual /* [helpcontext][propput] */ HRESULT STDMETHODCALLTYPE put_Value( 
            /* [in] */ VARIANT var) = 0;
        
        virtual /* [helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_Positions( 
            /* [retval][out] */ __RPC__deref_out_opt Positions **ppvObject) = 0;
        
        virtual /* [helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_Properties( 
            /* [retval][out] */ __RPC__deref_out_opt /* external definition not present */ Properties **ppvObject) = 0;
        
        virtual /* [helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_FormattedValue( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr) = 0;
        
        virtual /* [helpcontext][propput] */ HRESULT STDMETHODCALLTYPE put_FormattedValue( 
            /* [in] */ __RPC__in BSTR bstr) = 0;
        
        virtual /* [helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_Ordinal( 
            /* [retval][out] */ __RPC__out long *pl) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct CellVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in Cell * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in Cell * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in Cell * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in Cell * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in Cell * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in Cell * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            Cell * This,
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
        
        DECLSPEC_XFGVIRT(Cell, get_Value)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Value )( 
            __RPC__in Cell * This,
            /* [retval][out] */ __RPC__out VARIANT *pvar);
        
        DECLSPEC_XFGVIRT(Cell, put_Value)
        /* [helpcontext][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Value )( 
            __RPC__in Cell * This,
            /* [in] */ VARIANT var);
        
        DECLSPEC_XFGVIRT(Cell, get_Positions)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Positions )( 
            __RPC__in Cell * This,
            /* [retval][out] */ __RPC__deref_out_opt Positions **ppvObject);
        
        DECLSPEC_XFGVIRT(Cell, get_Properties)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Properties )( 
            __RPC__in Cell * This,
            /* [retval][out] */ __RPC__deref_out_opt /* external definition not present */ Properties **ppvObject);
        
        DECLSPEC_XFGVIRT(Cell, get_FormattedValue)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FormattedValue )( 
            __RPC__in Cell * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        DECLSPEC_XFGVIRT(Cell, put_FormattedValue)
        /* [helpcontext][propput] */ HRESULT ( STDMETHODCALLTYPE *put_FormattedValue )( 
            __RPC__in Cell * This,
            /* [in] */ __RPC__in BSTR bstr);
        
        DECLSPEC_XFGVIRT(Cell, get_Ordinal)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Ordinal )( 
            __RPC__in Cell * This,
            /* [retval][out] */ __RPC__out long *pl);
        
        END_INTERFACE
    } CellVtbl;

    interface Cell
    {
        CONST_VTBL struct CellVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define Cell_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define Cell_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define Cell_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define Cell_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define Cell_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define Cell_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define Cell_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define Cell_get_Value(This,pvar)	\
    ( (This)->lpVtbl -> get_Value(This,pvar) ) 

#define Cell_put_Value(This,var)	\
    ( (This)->lpVtbl -> put_Value(This,var) ) 

#define Cell_get_Positions(This,ppvObject)	\
    ( (This)->lpVtbl -> get_Positions(This,ppvObject) ) 

#define Cell_get_Properties(This,ppvObject)	\
    ( (This)->lpVtbl -> get_Properties(This,ppvObject) ) 

#define Cell_get_FormattedValue(This,pbstr)	\
    ( (This)->lpVtbl -> get_FormattedValue(This,pbstr) ) 

#define Cell_put_FormattedValue(This,bstr)	\
    ( (This)->lpVtbl -> put_FormattedValue(This,bstr) ) 

#define Cell_get_Ordinal(This,pl)	\
    ( (This)->lpVtbl -> get_Ordinal(This,pl) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __Cell_INTERFACE_DEFINED__ */


#ifndef __Axis_INTERFACE_DEFINED__
#define __Axis_INTERFACE_DEFINED__

/* interface Axis */
/* [unique][helpcontext][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_Axis;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("22813732-8BD3-11D0-B4EF-00A0C9138CA4")
    Axis : public IDispatch
    {
    public:
        virtual /* [helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr) = 0;
        
        virtual /* [helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_DimensionCount( 
            /* [retval][out] */ __RPC__out long *pl) = 0;
        
        virtual /* [id][helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_Positions( 
            /* [retval][out] */ __RPC__deref_out_opt Positions **ppvObject) = 0;
        
        virtual /* [helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_Properties( 
            /* [retval][out] */ __RPC__deref_out_opt /* external definition not present */ Properties **ppvObject) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct AxisVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in Axis * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in Axis * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in Axis * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in Axis * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in Axis * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in Axis * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            Axis * This,
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
        
        DECLSPEC_XFGVIRT(Axis, get_Name)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in Axis * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        DECLSPEC_XFGVIRT(Axis, get_DimensionCount)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DimensionCount )( 
            __RPC__in Axis * This,
            /* [retval][out] */ __RPC__out long *pl);
        
        DECLSPEC_XFGVIRT(Axis, get_Positions)
        /* [id][helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Positions )( 
            __RPC__in Axis * This,
            /* [retval][out] */ __RPC__deref_out_opt Positions **ppvObject);
        
        DECLSPEC_XFGVIRT(Axis, get_Properties)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Properties )( 
            __RPC__in Axis * This,
            /* [retval][out] */ __RPC__deref_out_opt /* external definition not present */ Properties **ppvObject);
        
        END_INTERFACE
    } AxisVtbl;

    interface Axis
    {
        CONST_VTBL struct AxisVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define Axis_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define Axis_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define Axis_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define Axis_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define Axis_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define Axis_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define Axis_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define Axis_get_Name(This,pbstr)	\
    ( (This)->lpVtbl -> get_Name(This,pbstr) ) 

#define Axis_get_DimensionCount(This,pl)	\
    ( (This)->lpVtbl -> get_DimensionCount(This,pl) ) 

#define Axis_get_Positions(This,ppvObject)	\
    ( (This)->lpVtbl -> get_Positions(This,ppvObject) ) 

#define Axis_get_Properties(This,ppvObject)	\
    ( (This)->lpVtbl -> get_Properties(This,ppvObject) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __Axis_INTERFACE_DEFINED__ */


#ifndef __Position_INTERFACE_DEFINED__
#define __Position_INTERFACE_DEFINED__

/* interface Position */
/* [unique][helpcontext][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_Position;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("22813734-8BD3-11D0-B4EF-00A0C9138CA4")
    Position : public IDispatch
    {
    public:
        virtual /* [helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_Ordinal( 
            /* [retval][out] */ __RPC__out long *pl) = 0;
        
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_Members( 
            /* [retval][out] */ __RPC__deref_out_opt Members **ppvObject) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct PositionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in Position * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in Position * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in Position * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in Position * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in Position * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in Position * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            Position * This,
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
        
        DECLSPEC_XFGVIRT(Position, get_Ordinal)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Ordinal )( 
            __RPC__in Position * This,
            /* [retval][out] */ __RPC__out long *pl);
        
        DECLSPEC_XFGVIRT(Position, get_Members)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Members )( 
            __RPC__in Position * This,
            /* [retval][out] */ __RPC__deref_out_opt Members **ppvObject);
        
        END_INTERFACE
    } PositionVtbl;

    interface Position
    {
        CONST_VTBL struct PositionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define Position_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define Position_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define Position_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define Position_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define Position_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define Position_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define Position_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define Position_get_Ordinal(This,pl)	\
    ( (This)->lpVtbl -> get_Ordinal(This,pl) ) 

#define Position_get_Members(This,ppvObject)	\
    ( (This)->lpVtbl -> get_Members(This,ppvObject) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __Position_INTERFACE_DEFINED__ */


#ifndef __Member_INTERFACE_DEFINED__
#define __Member_INTERFACE_DEFINED__

/* interface Member */
/* [unique][helpcontext][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_Member;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("22813736-8BD3-11D0-B4EF-00A0C9138CA4")
    Member : public IDispatch
    {
    public:
        virtual /* [helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr) = 0;
        
        virtual /* [helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_UniqueName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr) = 0;
        
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_Caption( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr) = 0;
        
        virtual /* [helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_Description( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr) = 0;
        
        virtual /* [helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_Parent( 
            /* [retval][out] */ __RPC__deref_out_opt Member **ppvObject) = 0;
        
        virtual /* [helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_LevelDepth( 
            /* [retval][out] */ __RPC__out long *pl) = 0;
        
        virtual /* [helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_LevelName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr) = 0;
        
        virtual /* [helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_Properties( 
            /* [retval][out] */ __RPC__deref_out_opt /* external definition not present */ Properties **ppvObject) = 0;
        
        virtual /* [helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_Type( 
            /* [retval][out] */ __RPC__out MemberTypeEnum *ptype) = 0;
        
        virtual /* [helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_ChildCount( 
            /* [retval][out] */ __RPC__out long *pl) = 0;
        
        virtual /* [helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_DrilledDown( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pf) = 0;
        
        virtual /* [helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_ParentSameAsPrev( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pf) = 0;
        
        virtual /* [helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_Children( 
            /* [retval][out] */ __RPC__deref_out_opt Members **ppvObject) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct MemberVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in Member * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in Member * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in Member * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in Member * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in Member * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in Member * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            Member * This,
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
        
        DECLSPEC_XFGVIRT(Member, get_Name)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in Member * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        DECLSPEC_XFGVIRT(Member, get_UniqueName)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UniqueName )( 
            __RPC__in Member * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        DECLSPEC_XFGVIRT(Member, get_Caption)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Caption )( 
            __RPC__in Member * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        DECLSPEC_XFGVIRT(Member, get_Description)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in Member * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        DECLSPEC_XFGVIRT(Member, get_Parent)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Parent )( 
            __RPC__in Member * This,
            /* [retval][out] */ __RPC__deref_out_opt Member **ppvObject);
        
        DECLSPEC_XFGVIRT(Member, get_LevelDepth)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LevelDepth )( 
            __RPC__in Member * This,
            /* [retval][out] */ __RPC__out long *pl);
        
        DECLSPEC_XFGVIRT(Member, get_LevelName)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LevelName )( 
            __RPC__in Member * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        DECLSPEC_XFGVIRT(Member, get_Properties)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Properties )( 
            __RPC__in Member * This,
            /* [retval][out] */ __RPC__deref_out_opt /* external definition not present */ Properties **ppvObject);
        
        DECLSPEC_XFGVIRT(Member, get_Type)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Type )( 
            __RPC__in Member * This,
            /* [retval][out] */ __RPC__out MemberTypeEnum *ptype);
        
        DECLSPEC_XFGVIRT(Member, get_ChildCount)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ChildCount )( 
            __RPC__in Member * This,
            /* [retval][out] */ __RPC__out long *pl);
        
        DECLSPEC_XFGVIRT(Member, get_DrilledDown)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DrilledDown )( 
            __RPC__in Member * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pf);
        
        DECLSPEC_XFGVIRT(Member, get_ParentSameAsPrev)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ParentSameAsPrev )( 
            __RPC__in Member * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pf);
        
        DECLSPEC_XFGVIRT(Member, get_Children)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Children )( 
            __RPC__in Member * This,
            /* [retval][out] */ __RPC__deref_out_opt Members **ppvObject);
        
        END_INTERFACE
    } MemberVtbl;

    interface Member
    {
        CONST_VTBL struct MemberVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define Member_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define Member_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define Member_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define Member_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define Member_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define Member_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define Member_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define Member_get_Name(This,pbstr)	\
    ( (This)->lpVtbl -> get_Name(This,pbstr) ) 

#define Member_get_UniqueName(This,pbstr)	\
    ( (This)->lpVtbl -> get_UniqueName(This,pbstr) ) 

#define Member_get_Caption(This,pbstr)	\
    ( (This)->lpVtbl -> get_Caption(This,pbstr) ) 

#define Member_get_Description(This,pbstr)	\
    ( (This)->lpVtbl -> get_Description(This,pbstr) ) 

#define Member_get_Parent(This,ppvObject)	\
    ( (This)->lpVtbl -> get_Parent(This,ppvObject) ) 

#define Member_get_LevelDepth(This,pl)	\
    ( (This)->lpVtbl -> get_LevelDepth(This,pl) ) 

#define Member_get_LevelName(This,pbstr)	\
    ( (This)->lpVtbl -> get_LevelName(This,pbstr) ) 

#define Member_get_Properties(This,ppvObject)	\
    ( (This)->lpVtbl -> get_Properties(This,ppvObject) ) 

#define Member_get_Type(This,ptype)	\
    ( (This)->lpVtbl -> get_Type(This,ptype) ) 

#define Member_get_ChildCount(This,pl)	\
    ( (This)->lpVtbl -> get_ChildCount(This,pl) ) 

#define Member_get_DrilledDown(This,pf)	\
    ( (This)->lpVtbl -> get_DrilledDown(This,pf) ) 

#define Member_get_ParentSameAsPrev(This,pf)	\
    ( (This)->lpVtbl -> get_ParentSameAsPrev(This,pf) ) 

#define Member_get_Children(This,ppvObject)	\
    ( (This)->lpVtbl -> get_Children(This,ppvObject) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __Member_INTERFACE_DEFINED__ */


#ifndef __Level_INTERFACE_DEFINED__
#define __Level_INTERFACE_DEFINED__

/* interface Level */
/* [unique][helpcontext][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_Level;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2281373A-8BD3-11D0-B4EF-00A0C9138CA4")
    Level : public IDispatch
    {
    public:
        virtual /* [helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr) = 0;
        
        virtual /* [helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_UniqueName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr) = 0;
        
        virtual /* [helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_Caption( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr) = 0;
        
        virtual /* [helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_Description( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr) = 0;
        
        virtual /* [helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_Depth( 
            /* [retval][out] */ __RPC__out short *pw) = 0;
        
        virtual /* [helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_Properties( 
            /* [retval][out] */ __RPC__deref_out_opt /* external definition not present */ Properties **ppvObject) = 0;
        
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_Members( 
            /* [retval][out] */ __RPC__deref_out_opt Members **ppvObject) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct LevelVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in Level * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in Level * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in Level * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in Level * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in Level * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in Level * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            Level * This,
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
        
        DECLSPEC_XFGVIRT(Level, get_Name)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in Level * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        DECLSPEC_XFGVIRT(Level, get_UniqueName)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UniqueName )( 
            __RPC__in Level * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        DECLSPEC_XFGVIRT(Level, get_Caption)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Caption )( 
            __RPC__in Level * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        DECLSPEC_XFGVIRT(Level, get_Description)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in Level * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        DECLSPEC_XFGVIRT(Level, get_Depth)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Depth )( 
            __RPC__in Level * This,
            /* [retval][out] */ __RPC__out short *pw);
        
        DECLSPEC_XFGVIRT(Level, get_Properties)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Properties )( 
            __RPC__in Level * This,
            /* [retval][out] */ __RPC__deref_out_opt /* external definition not present */ Properties **ppvObject);
        
        DECLSPEC_XFGVIRT(Level, get_Members)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Members )( 
            __RPC__in Level * This,
            /* [retval][out] */ __RPC__deref_out_opt Members **ppvObject);
        
        END_INTERFACE
    } LevelVtbl;

    interface Level
    {
        CONST_VTBL struct LevelVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define Level_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define Level_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define Level_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define Level_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define Level_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define Level_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define Level_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define Level_get_Name(This,pbstr)	\
    ( (This)->lpVtbl -> get_Name(This,pbstr) ) 

#define Level_get_UniqueName(This,pbstr)	\
    ( (This)->lpVtbl -> get_UniqueName(This,pbstr) ) 

#define Level_get_Caption(This,pbstr)	\
    ( (This)->lpVtbl -> get_Caption(This,pbstr) ) 

#define Level_get_Description(This,pbstr)	\
    ( (This)->lpVtbl -> get_Description(This,pbstr) ) 

#define Level_get_Depth(This,pw)	\
    ( (This)->lpVtbl -> get_Depth(This,pw) ) 

#define Level_get_Properties(This,ppvObject)	\
    ( (This)->lpVtbl -> get_Properties(This,ppvObject) ) 

#define Level_get_Members(This,ppvObject)	\
    ( (This)->lpVtbl -> get_Members(This,ppvObject) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __Level_INTERFACE_DEFINED__ */


#ifndef __CubeDef25_INTERFACE_DEFINED__
#define __CubeDef25_INTERFACE_DEFINED__

/* interface CubeDef25 */
/* [unique][helpcontext][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_CubeDef25;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2281373E-8BD3-11D0-B4EF-00A0C9138CA4")
    CubeDef25 : public IDispatch
    {
    public:
        virtual /* [helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr) = 0;
        
        virtual /* [helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_Description( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr) = 0;
        
        virtual /* [helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_Properties( 
            /* [retval][out] */ __RPC__deref_out_opt /* external definition not present */ Properties **ppvObject) = 0;
        
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_Dimensions( 
            /* [retval][out] */ __RPC__deref_out_opt Dimensions **ppvObject) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct CubeDef25Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in CubeDef25 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in CubeDef25 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in CubeDef25 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in CubeDef25 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in CubeDef25 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in CubeDef25 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            CubeDef25 * This,
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
        
        DECLSPEC_XFGVIRT(CubeDef25, get_Name)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in CubeDef25 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        DECLSPEC_XFGVIRT(CubeDef25, get_Description)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in CubeDef25 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        DECLSPEC_XFGVIRT(CubeDef25, get_Properties)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Properties )( 
            __RPC__in CubeDef25 * This,
            /* [retval][out] */ __RPC__deref_out_opt /* external definition not present */ Properties **ppvObject);
        
        DECLSPEC_XFGVIRT(CubeDef25, get_Dimensions)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Dimensions )( 
            __RPC__in CubeDef25 * This,
            /* [retval][out] */ __RPC__deref_out_opt Dimensions **ppvObject);
        
        END_INTERFACE
    } CubeDef25Vtbl;

    interface CubeDef25
    {
        CONST_VTBL struct CubeDef25Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define CubeDef25_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define CubeDef25_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define CubeDef25_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define CubeDef25_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define CubeDef25_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define CubeDef25_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define CubeDef25_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define CubeDef25_get_Name(This,pbstr)	\
    ( (This)->lpVtbl -> get_Name(This,pbstr) ) 

#define CubeDef25_get_Description(This,pbstr)	\
    ( (This)->lpVtbl -> get_Description(This,pbstr) ) 

#define CubeDef25_get_Properties(This,ppvObject)	\
    ( (This)->lpVtbl -> get_Properties(This,ppvObject) ) 

#define CubeDef25_get_Dimensions(This,ppvObject)	\
    ( (This)->lpVtbl -> get_Dimensions(This,ppvObject) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __CubeDef25_INTERFACE_DEFINED__ */


#ifndef __CubeDef_INTERFACE_DEFINED__
#define __CubeDef_INTERFACE_DEFINED__

/* interface CubeDef */
/* [unique][helpcontext][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_CubeDef;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DA16A34A-7B7A-46fd-AD9D-66DF1E699FA1")
    CubeDef : public CubeDef25
    {
    public:
        virtual /* [helpcontext] */ HRESULT STDMETHODCALLTYPE GetSchemaObject( 
            /* [in] */ SchemaObjectTypeEnum eObjType,
            /* [in] */ __RPC__in BSTR bsUniqueName,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppObj) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct CubeDefVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in CubeDef * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in CubeDef * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in CubeDef * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in CubeDef * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in CubeDef * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in CubeDef * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            CubeDef * This,
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
        
        DECLSPEC_XFGVIRT(CubeDef25, get_Name)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in CubeDef * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        DECLSPEC_XFGVIRT(CubeDef25, get_Description)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in CubeDef * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        DECLSPEC_XFGVIRT(CubeDef25, get_Properties)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Properties )( 
            __RPC__in CubeDef * This,
            /* [retval][out] */ __RPC__deref_out_opt /* external definition not present */ Properties **ppvObject);
        
        DECLSPEC_XFGVIRT(CubeDef25, get_Dimensions)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Dimensions )( 
            __RPC__in CubeDef * This,
            /* [retval][out] */ __RPC__deref_out_opt Dimensions **ppvObject);
        
        DECLSPEC_XFGVIRT(CubeDef, GetSchemaObject)
        /* [helpcontext] */ HRESULT ( STDMETHODCALLTYPE *GetSchemaObject )( 
            __RPC__in CubeDef * This,
            /* [in] */ SchemaObjectTypeEnum eObjType,
            /* [in] */ __RPC__in BSTR bsUniqueName,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppObj);
        
        END_INTERFACE
    } CubeDefVtbl;

    interface CubeDef
    {
        CONST_VTBL struct CubeDefVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define CubeDef_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define CubeDef_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define CubeDef_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define CubeDef_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define CubeDef_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define CubeDef_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define CubeDef_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define CubeDef_get_Name(This,pbstr)	\
    ( (This)->lpVtbl -> get_Name(This,pbstr) ) 

#define CubeDef_get_Description(This,pbstr)	\
    ( (This)->lpVtbl -> get_Description(This,pbstr) ) 

#define CubeDef_get_Properties(This,ppvObject)	\
    ( (This)->lpVtbl -> get_Properties(This,ppvObject) ) 

#define CubeDef_get_Dimensions(This,ppvObject)	\
    ( (This)->lpVtbl -> get_Dimensions(This,ppvObject) ) 


#define CubeDef_GetSchemaObject(This,eObjType,bsUniqueName,ppObj)	\
    ( (This)->lpVtbl -> GetSchemaObject(This,eObjType,bsUniqueName,ppObj) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __CubeDef_INTERFACE_DEFINED__ */


#ifndef __Dimension_INTERFACE_DEFINED__
#define __Dimension_INTERFACE_DEFINED__

/* interface Dimension */
/* [unique][helpcontext][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_Dimension;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("22813742-8BD3-11D0-B4EF-00A0C9138CA4")
    Dimension : public IDispatch
    {
    public:
        virtual /* [helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr) = 0;
        
        virtual /* [helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_UniqueName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr) = 0;
        
        virtual /* [helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_Description( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr) = 0;
        
        virtual /* [helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_Properties( 
            /* [retval][out] */ __RPC__deref_out_opt /* external definition not present */ Properties **ppvObject) = 0;
        
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_Hierarchies( 
            /* [retval][out] */ __RPC__deref_out_opt Hierarchies **ppvObject) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct DimensionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in Dimension * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in Dimension * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in Dimension * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in Dimension * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in Dimension * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in Dimension * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            Dimension * This,
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
        
        DECLSPEC_XFGVIRT(Dimension, get_Name)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in Dimension * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        DECLSPEC_XFGVIRT(Dimension, get_UniqueName)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UniqueName )( 
            __RPC__in Dimension * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        DECLSPEC_XFGVIRT(Dimension, get_Description)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in Dimension * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        DECLSPEC_XFGVIRT(Dimension, get_Properties)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Properties )( 
            __RPC__in Dimension * This,
            /* [retval][out] */ __RPC__deref_out_opt /* external definition not present */ Properties **ppvObject);
        
        DECLSPEC_XFGVIRT(Dimension, get_Hierarchies)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Hierarchies )( 
            __RPC__in Dimension * This,
            /* [retval][out] */ __RPC__deref_out_opt Hierarchies **ppvObject);
        
        END_INTERFACE
    } DimensionVtbl;

    interface Dimension
    {
        CONST_VTBL struct DimensionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define Dimension_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define Dimension_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define Dimension_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define Dimension_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define Dimension_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define Dimension_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define Dimension_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define Dimension_get_Name(This,pbstr)	\
    ( (This)->lpVtbl -> get_Name(This,pbstr) ) 

#define Dimension_get_UniqueName(This,pbstr)	\
    ( (This)->lpVtbl -> get_UniqueName(This,pbstr) ) 

#define Dimension_get_Description(This,pbstr)	\
    ( (This)->lpVtbl -> get_Description(This,pbstr) ) 

#define Dimension_get_Properties(This,ppvObject)	\
    ( (This)->lpVtbl -> get_Properties(This,ppvObject) ) 

#define Dimension_get_Hierarchies(This,ppvObject)	\
    ( (This)->lpVtbl -> get_Hierarchies(This,ppvObject) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __Dimension_INTERFACE_DEFINED__ */


#ifndef __Hierarchy_INTERFACE_DEFINED__
#define __Hierarchy_INTERFACE_DEFINED__

/* interface Hierarchy */
/* [unique][helpcontext][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_Hierarchy;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("22813746-8BD3-11D0-B4EF-00A0C9138CA4")
    Hierarchy : public IDispatch
    {
    public:
        virtual /* [helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr) = 0;
        
        virtual /* [helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_UniqueName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr) = 0;
        
        virtual /* [helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_Description( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr) = 0;
        
        virtual /* [helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_Properties( 
            /* [retval][out] */ __RPC__deref_out_opt /* external definition not present */ Properties **ppvObject) = 0;
        
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_Levels( 
            /* [retval][out] */ __RPC__deref_out_opt Levels **ppvObject) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct HierarchyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in Hierarchy * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in Hierarchy * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in Hierarchy * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in Hierarchy * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in Hierarchy * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in Hierarchy * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            Hierarchy * This,
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
        
        DECLSPEC_XFGVIRT(Hierarchy, get_Name)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in Hierarchy * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        DECLSPEC_XFGVIRT(Hierarchy, get_UniqueName)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UniqueName )( 
            __RPC__in Hierarchy * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        DECLSPEC_XFGVIRT(Hierarchy, get_Description)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in Hierarchy * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        DECLSPEC_XFGVIRT(Hierarchy, get_Properties)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Properties )( 
            __RPC__in Hierarchy * This,
            /* [retval][out] */ __RPC__deref_out_opt /* external definition not present */ Properties **ppvObject);
        
        DECLSPEC_XFGVIRT(Hierarchy, get_Levels)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Levels )( 
            __RPC__in Hierarchy * This,
            /* [retval][out] */ __RPC__deref_out_opt Levels **ppvObject);
        
        END_INTERFACE
    } HierarchyVtbl;

    interface Hierarchy
    {
        CONST_VTBL struct HierarchyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define Hierarchy_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define Hierarchy_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define Hierarchy_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define Hierarchy_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define Hierarchy_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define Hierarchy_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define Hierarchy_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define Hierarchy_get_Name(This,pbstr)	\
    ( (This)->lpVtbl -> get_Name(This,pbstr) ) 

#define Hierarchy_get_UniqueName(This,pbstr)	\
    ( (This)->lpVtbl -> get_UniqueName(This,pbstr) ) 

#define Hierarchy_get_Description(This,pbstr)	\
    ( (This)->lpVtbl -> get_Description(This,pbstr) ) 

#define Hierarchy_get_Properties(This,ppvObject)	\
    ( (This)->lpVtbl -> get_Properties(This,ppvObject) ) 

#define Hierarchy_get_Levels(This,ppvObject)	\
    ( (This)->lpVtbl -> get_Levels(This,ppvObject) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __Hierarchy_INTERFACE_DEFINED__ */


#ifndef __MD_Collection_INTERFACE_DEFINED__
#define __MD_Collection_INTERFACE_DEFINED__

/* interface MD_Collection */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_MD_Collection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("22813751-8BD3-11D0-B4EF-00A0C9138CA4")
    MD_Collection : public IDispatch
    {
    public:
        virtual /* [helpcontext] */ HRESULT STDMETHODCALLTYPE Refresh( void) = 0;
        
        virtual /* [id][restricted] */ HRESULT STDMETHODCALLTYPE _NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppvObject) = 0;
        
        virtual /* [helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *c) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct MD_CollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in MD_Collection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in MD_Collection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in MD_Collection * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in MD_Collection * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in MD_Collection * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in MD_Collection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            MD_Collection * This,
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
        
        DECLSPEC_XFGVIRT(MD_Collection, Refresh)
        /* [helpcontext] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in MD_Collection * This);
        
        DECLSPEC_XFGVIRT(MD_Collection, _NewEnum)
        /* [id][restricted] */ HRESULT ( STDMETHODCALLTYPE *_NewEnum )( 
            __RPC__in MD_Collection * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppvObject);
        
        DECLSPEC_XFGVIRT(MD_Collection, get_Count)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in MD_Collection * This,
            /* [retval][out] */ __RPC__out long *c);
        
        END_INTERFACE
    } MD_CollectionVtbl;

    interface MD_Collection
    {
        CONST_VTBL struct MD_CollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define MD_Collection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define MD_Collection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define MD_Collection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define MD_Collection_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define MD_Collection_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define MD_Collection_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define MD_Collection_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define MD_Collection_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#define MD_Collection__NewEnum(This,ppvObject)	\
    ( (This)->lpVtbl -> _NewEnum(This,ppvObject) ) 

#define MD_Collection_get_Count(This,c)	\
    ( (This)->lpVtbl -> get_Count(This,c) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __MD_Collection_INTERFACE_DEFINED__ */


#ifndef __Members_INTERFACE_DEFINED__
#define __Members_INTERFACE_DEFINED__

/* interface Members */
/* [unique][helpcontext][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_Members;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("22813757-8BD3-11D0-B4EF-00A0C9138CA4")
    Members : public MD_Collection
    {
    public:
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT Index,
            /* [retval][out] */ __RPC__deref_out_opt Member **ppvObject) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct MembersVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in Members * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in Members * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in Members * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in Members * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in Members * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in Members * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            Members * This,
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
        
        DECLSPEC_XFGVIRT(MD_Collection, Refresh)
        /* [helpcontext] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in Members * This);
        
        DECLSPEC_XFGVIRT(MD_Collection, _NewEnum)
        /* [id][restricted] */ HRESULT ( STDMETHODCALLTYPE *_NewEnum )( 
            __RPC__in Members * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppvObject);
        
        DECLSPEC_XFGVIRT(MD_Collection, get_Count)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in Members * This,
            /* [retval][out] */ __RPC__out long *c);
        
        DECLSPEC_XFGVIRT(Members, get_Item)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in Members * This,
            /* [in] */ VARIANT Index,
            /* [retval][out] */ __RPC__deref_out_opt Member **ppvObject);
        
        END_INTERFACE
    } MembersVtbl;

    interface Members
    {
        CONST_VTBL struct MembersVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define Members_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define Members_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define Members_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define Members_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define Members_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define Members_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define Members_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define Members_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#define Members__NewEnum(This,ppvObject)	\
    ( (This)->lpVtbl -> _NewEnum(This,ppvObject) ) 

#define Members_get_Count(This,c)	\
    ( (This)->lpVtbl -> get_Count(This,c) ) 


#define Members_get_Item(This,Index,ppvObject)	\
    ( (This)->lpVtbl -> get_Item(This,Index,ppvObject) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __Members_INTERFACE_DEFINED__ */


#ifndef __Levels_INTERFACE_DEFINED__
#define __Levels_INTERFACE_DEFINED__

/* interface Levels */
/* [unique][helpcontext][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_Levels;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("22813758-8BD3-11D0-B4EF-00A0C9138CA4")
    Levels : public MD_Collection
    {
    public:
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT Index,
            /* [retval][out] */ __RPC__deref_out_opt Level **ppvObject) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct LevelsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in Levels * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in Levels * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in Levels * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in Levels * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in Levels * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in Levels * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            Levels * This,
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
        
        DECLSPEC_XFGVIRT(MD_Collection, Refresh)
        /* [helpcontext] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in Levels * This);
        
        DECLSPEC_XFGVIRT(MD_Collection, _NewEnum)
        /* [id][restricted] */ HRESULT ( STDMETHODCALLTYPE *_NewEnum )( 
            __RPC__in Levels * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppvObject);
        
        DECLSPEC_XFGVIRT(MD_Collection, get_Count)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in Levels * This,
            /* [retval][out] */ __RPC__out long *c);
        
        DECLSPEC_XFGVIRT(Levels, get_Item)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in Levels * This,
            /* [in] */ VARIANT Index,
            /* [retval][out] */ __RPC__deref_out_opt Level **ppvObject);
        
        END_INTERFACE
    } LevelsVtbl;

    interface Levels
    {
        CONST_VTBL struct LevelsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define Levels_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define Levels_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define Levels_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define Levels_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define Levels_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define Levels_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define Levels_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define Levels_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#define Levels__NewEnum(This,ppvObject)	\
    ( (This)->lpVtbl -> _NewEnum(This,ppvObject) ) 

#define Levels_get_Count(This,c)	\
    ( (This)->lpVtbl -> get_Count(This,c) ) 


#define Levels_get_Item(This,Index,ppvObject)	\
    ( (This)->lpVtbl -> get_Item(This,Index,ppvObject) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __Levels_INTERFACE_DEFINED__ */


#ifndef __Axes_INTERFACE_DEFINED__
#define __Axes_INTERFACE_DEFINED__

/* interface Axes */
/* [unique][helpcontext][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_Axes;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("22813759-8BD3-11D0-B4EF-00A0C9138CA4")
    Axes : public MD_Collection
    {
    public:
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT Index,
            /* [retval][out] */ __RPC__deref_out_opt Axis **ppvObject) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct AxesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in Axes * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in Axes * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in Axes * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in Axes * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in Axes * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in Axes * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            Axes * This,
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
        
        DECLSPEC_XFGVIRT(MD_Collection, Refresh)
        /* [helpcontext] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in Axes * This);
        
        DECLSPEC_XFGVIRT(MD_Collection, _NewEnum)
        /* [id][restricted] */ HRESULT ( STDMETHODCALLTYPE *_NewEnum )( 
            __RPC__in Axes * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppvObject);
        
        DECLSPEC_XFGVIRT(MD_Collection, get_Count)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in Axes * This,
            /* [retval][out] */ __RPC__out long *c);
        
        DECLSPEC_XFGVIRT(Axes, get_Item)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in Axes * This,
            /* [in] */ VARIANT Index,
            /* [retval][out] */ __RPC__deref_out_opt Axis **ppvObject);
        
        END_INTERFACE
    } AxesVtbl;

    interface Axes
    {
        CONST_VTBL struct AxesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define Axes_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define Axes_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define Axes_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define Axes_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define Axes_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define Axes_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define Axes_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define Axes_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#define Axes__NewEnum(This,ppvObject)	\
    ( (This)->lpVtbl -> _NewEnum(This,ppvObject) ) 

#define Axes_get_Count(This,c)	\
    ( (This)->lpVtbl -> get_Count(This,c) ) 


#define Axes_get_Item(This,Index,ppvObject)	\
    ( (This)->lpVtbl -> get_Item(This,Index,ppvObject) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __Axes_INTERFACE_DEFINED__ */


#ifndef __Positions_INTERFACE_DEFINED__
#define __Positions_INTERFACE_DEFINED__

/* interface Positions */
/* [unique][helpcontext][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_Positions;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2281375A-8BD3-11D0-B4EF-00A0C9138CA4")
    Positions : public MD_Collection
    {
    public:
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT Index,
            /* [retval][out] */ __RPC__deref_out_opt Position **ppvObject) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct PositionsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in Positions * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in Positions * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in Positions * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in Positions * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in Positions * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in Positions * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            Positions * This,
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
        
        DECLSPEC_XFGVIRT(MD_Collection, Refresh)
        /* [helpcontext] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in Positions * This);
        
        DECLSPEC_XFGVIRT(MD_Collection, _NewEnum)
        /* [id][restricted] */ HRESULT ( STDMETHODCALLTYPE *_NewEnum )( 
            __RPC__in Positions * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppvObject);
        
        DECLSPEC_XFGVIRT(MD_Collection, get_Count)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in Positions * This,
            /* [retval][out] */ __RPC__out long *c);
        
        DECLSPEC_XFGVIRT(Positions, get_Item)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in Positions * This,
            /* [in] */ VARIANT Index,
            /* [retval][out] */ __RPC__deref_out_opt Position **ppvObject);
        
        END_INTERFACE
    } PositionsVtbl;

    interface Positions
    {
        CONST_VTBL struct PositionsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define Positions_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define Positions_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define Positions_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define Positions_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define Positions_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define Positions_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define Positions_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define Positions_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#define Positions__NewEnum(This,ppvObject)	\
    ( (This)->lpVtbl -> _NewEnum(This,ppvObject) ) 

#define Positions_get_Count(This,c)	\
    ( (This)->lpVtbl -> get_Count(This,c) ) 


#define Positions_get_Item(This,Index,ppvObject)	\
    ( (This)->lpVtbl -> get_Item(This,Index,ppvObject) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __Positions_INTERFACE_DEFINED__ */


#ifndef __Hierarchies_INTERFACE_DEFINED__
#define __Hierarchies_INTERFACE_DEFINED__

/* interface Hierarchies */
/* [unique][helpcontext][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_Hierarchies;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2281375B-8BD3-11D0-B4EF-00A0C9138CA4")
    Hierarchies : public MD_Collection
    {
    public:
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT Index,
            /* [retval][out] */ __RPC__deref_out_opt Hierarchy **ppvObject) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct HierarchiesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in Hierarchies * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in Hierarchies * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in Hierarchies * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in Hierarchies * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in Hierarchies * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in Hierarchies * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            Hierarchies * This,
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
        
        DECLSPEC_XFGVIRT(MD_Collection, Refresh)
        /* [helpcontext] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in Hierarchies * This);
        
        DECLSPEC_XFGVIRT(MD_Collection, _NewEnum)
        /* [id][restricted] */ HRESULT ( STDMETHODCALLTYPE *_NewEnum )( 
            __RPC__in Hierarchies * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppvObject);
        
        DECLSPEC_XFGVIRT(MD_Collection, get_Count)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in Hierarchies * This,
            /* [retval][out] */ __RPC__out long *c);
        
        DECLSPEC_XFGVIRT(Hierarchies, get_Item)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in Hierarchies * This,
            /* [in] */ VARIANT Index,
            /* [retval][out] */ __RPC__deref_out_opt Hierarchy **ppvObject);
        
        END_INTERFACE
    } HierarchiesVtbl;

    interface Hierarchies
    {
        CONST_VTBL struct HierarchiesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define Hierarchies_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define Hierarchies_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define Hierarchies_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define Hierarchies_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define Hierarchies_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define Hierarchies_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define Hierarchies_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define Hierarchies_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#define Hierarchies__NewEnum(This,ppvObject)	\
    ( (This)->lpVtbl -> _NewEnum(This,ppvObject) ) 

#define Hierarchies_get_Count(This,c)	\
    ( (This)->lpVtbl -> get_Count(This,c) ) 


#define Hierarchies_get_Item(This,Index,ppvObject)	\
    ( (This)->lpVtbl -> get_Item(This,Index,ppvObject) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __Hierarchies_INTERFACE_DEFINED__ */


#ifndef __Dimensions_INTERFACE_DEFINED__
#define __Dimensions_INTERFACE_DEFINED__

/* interface Dimensions */
/* [unique][helpcontext][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_Dimensions;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2281375C-8BD3-11D0-B4EF-00A0C9138CA4")
    Dimensions : public MD_Collection
    {
    public:
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT Index,
            /* [retval][out] */ __RPC__deref_out_opt Dimension **ppvObject) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct DimensionsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in Dimensions * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in Dimensions * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in Dimensions * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in Dimensions * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in Dimensions * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in Dimensions * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            Dimensions * This,
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
        
        DECLSPEC_XFGVIRT(MD_Collection, Refresh)
        /* [helpcontext] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in Dimensions * This);
        
        DECLSPEC_XFGVIRT(MD_Collection, _NewEnum)
        /* [id][restricted] */ HRESULT ( STDMETHODCALLTYPE *_NewEnum )( 
            __RPC__in Dimensions * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppvObject);
        
        DECLSPEC_XFGVIRT(MD_Collection, get_Count)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in Dimensions * This,
            /* [retval][out] */ __RPC__out long *c);
        
        DECLSPEC_XFGVIRT(Dimensions, get_Item)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in Dimensions * This,
            /* [in] */ VARIANT Index,
            /* [retval][out] */ __RPC__deref_out_opt Dimension **ppvObject);
        
        END_INTERFACE
    } DimensionsVtbl;

    interface Dimensions
    {
        CONST_VTBL struct DimensionsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define Dimensions_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define Dimensions_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define Dimensions_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define Dimensions_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define Dimensions_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define Dimensions_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define Dimensions_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define Dimensions_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#define Dimensions__NewEnum(This,ppvObject)	\
    ( (This)->lpVtbl -> _NewEnum(This,ppvObject) ) 

#define Dimensions_get_Count(This,c)	\
    ( (This)->lpVtbl -> get_Count(This,c) ) 


#define Dimensions_get_Item(This,Index,ppvObject)	\
    ( (This)->lpVtbl -> get_Item(This,Index,ppvObject) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __Dimensions_INTERFACE_DEFINED__ */


#ifndef __CubeDefs_INTERFACE_DEFINED__
#define __CubeDefs_INTERFACE_DEFINED__

/* interface CubeDefs */
/* [unique][helpcontext][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_CubeDefs;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2281375D-8BD3-11D0-B4EF-00A0C9138CA4")
    CubeDefs : public MD_Collection
    {
    public:
        virtual /* [helpcontext][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT Index,
            /* [retval][out] */ __RPC__deref_out_opt CubeDef **ppvObject) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct CubeDefsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in CubeDefs * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in CubeDefs * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in CubeDefs * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in CubeDefs * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in CubeDefs * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in CubeDefs * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            CubeDefs * This,
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
        
        DECLSPEC_XFGVIRT(MD_Collection, Refresh)
        /* [helpcontext] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in CubeDefs * This);
        
        DECLSPEC_XFGVIRT(MD_Collection, _NewEnum)
        /* [id][restricted] */ HRESULT ( STDMETHODCALLTYPE *_NewEnum )( 
            __RPC__in CubeDefs * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppvObject);
        
        DECLSPEC_XFGVIRT(MD_Collection, get_Count)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in CubeDefs * This,
            /* [retval][out] */ __RPC__out long *c);
        
        DECLSPEC_XFGVIRT(CubeDefs, get_Item)
        /* [helpcontext][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in CubeDefs * This,
            /* [in] */ VARIANT Index,
            /* [retval][out] */ __RPC__deref_out_opt CubeDef **ppvObject);
        
        END_INTERFACE
    } CubeDefsVtbl;

    interface CubeDefs
    {
        CONST_VTBL struct CubeDefsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define CubeDefs_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define CubeDefs_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define CubeDefs_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define CubeDefs_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define CubeDefs_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define CubeDefs_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define CubeDefs_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define CubeDefs_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#define CubeDefs__NewEnum(This,ppvObject)	\
    ( (This)->lpVtbl -> _NewEnum(This,ppvObject) ) 

#define CubeDefs_get_Count(This,c)	\
    ( (This)->lpVtbl -> get_Count(This,c) ) 


#define CubeDefs_get_Item(This,Index,ppvObject)	\
    ( (This)->lpVtbl -> get_Item(This,Index,ppvObject) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __CubeDefs_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_Catalog;

#ifdef __cplusplus

class DECLSPEC_UUID("228136B0-8BD3-11D0-B4EF-00A0C9138CA4")
Catalog;
#endif

EXTERN_C const CLSID CLSID_Cellset;

#ifdef __cplusplus

class DECLSPEC_UUID("228136B8-8BD3-11D0-B4EF-00A0C9138CA4")
Cellset;
#endif
#endif /* __ADOMD_LIBRARY_DEFINED__ */

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


