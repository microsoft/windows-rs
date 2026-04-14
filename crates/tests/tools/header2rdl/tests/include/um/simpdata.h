

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


#ifndef __simpdata_h__
#define __simpdata_h__

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

#ifndef __OLEDBSimpleProviderListener_FWD_DEFINED__
#define __OLEDBSimpleProviderListener_FWD_DEFINED__
typedef interface OLEDBSimpleProviderListener OLEDBSimpleProviderListener;

#endif 	/* __OLEDBSimpleProviderListener_FWD_DEFINED__ */


#ifndef __OLEDBSimpleProvider_FWD_DEFINED__
#define __OLEDBSimpleProvider_FWD_DEFINED__
typedef interface OLEDBSimpleProvider OLEDBSimpleProvider;

#endif 	/* __OLEDBSimpleProvider_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_simpdata_0000_0000 */
/* [local] */ 

//=--------------------------------------------------------------------------=
// simpdata.h
//=--------------------------------------------------------------------------=
// (C) Copyright 1995-1996 Microsoft Corporation.  All Rights Reserved.
//
// THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF
// ANY KIND, EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// THE IMPLIED WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A
// PARTICULAR PURPOSE.
//=--------------------------------------------------------------------------=

#pragma comment(lib,"uuid.lib")

//--------------------------------------------------------------------------
// OLE DB Simple Provider Toolkit

#ifndef SIMPDATA_H
#define SIMPDATA_H
#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#ifdef _WIN64

typedef LONGLONG DBROWCOUNT;

typedef LONGLONG DB_LORDINAL;


#else

typedef LONG DBROWCOUNT;

typedef LONG DB_LORDINAL;

#endif	// _WIN64
#define OSP_IndexLabel      (0)
#define OSP_IndexAll        (~0)
#define OSP_IndexUnknown    (~0)



extern RPC_IF_HANDLE __MIDL_itf_simpdata_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_simpdata_0000_0000_v0_0_s_ifspec;


#ifndef __MSDAOSP_LIBRARY_DEFINED__
#define __MSDAOSP_LIBRARY_DEFINED__

/* library MSDAOSP */
/* [version][lcid][helpstring][uuid] */ 

typedef 
enum OSPFORMAT
    {
        OSPFORMAT_RAW	= 0,
        OSPFORMAT_DEFAULT	= 0,
        OSPFORMAT_FORMATTED	= 1,
        OSPFORMAT_HTML	= 2
    } 	OSPFORMAT;

typedef 
enum OSPRW
    {
        OSPRW_DEFAULT	= 1,
        OSPRW_READONLY	= 0,
        OSPRW_READWRITE	= 1,
        OSPRW_MIXED	= 2
    } 	OSPRW;

typedef 
enum OSPFIND
    {
        OSPFIND_DEFAULT	= 0,
        OSPFIND_UP	= 1,
        OSPFIND_CASESENSITIVE	= 2,
        OSPFIND_UPCASESENSITIVE	= 3
    } 	OSPFIND;

typedef 
enum OSPCOMP
    {
        OSPCOMP_EQ	= 1,
        OSPCOMP_DEFAULT	= 1,
        OSPCOMP_LT	= 2,
        OSPCOMP_LE	= 3,
        OSPCOMP_GE	= 4,
        OSPCOMP_GT	= 5,
        OSPCOMP_NE	= 6
    } 	OSPCOMP;

typedef 
enum OSPXFER
    {
        OSPXFER_COMPLETE	= 0,
        OSPXFER_ABORT	= 1,
        OSPXFER_ERROR	= 2
    } 	OSPXFER;

typedef OLEDBSimpleProvider *LPOLEDBSimpleProvider;

EXTERN_C const IID LIBID_MSDAOSP;

#ifndef __OLEDBSimpleProviderListener_INTERFACE_DEFINED__
#define __OLEDBSimpleProviderListener_INTERFACE_DEFINED__

/* interface OLEDBSimpleProviderListener */
/* [version][oleautomation][unique][uuid][object] */ 


EXTERN_C const IID IID_OLEDBSimpleProviderListener;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("E0E270C1-C0BE-11d0-8FE4-00A0C90A6341")
    OLEDBSimpleProviderListener : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE aboutToChangeCell( 
            /* [in] */ DBROWCOUNT iRow,
            /* [in] */ DB_LORDINAL iColumn) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE cellChanged( 
            /* [in] */ DBROWCOUNT iRow,
            /* [in] */ DB_LORDINAL iColumn) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE aboutToDeleteRows( 
            /* [in] */ DBROWCOUNT iRow,
            /* [in] */ DBROWCOUNT cRows) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE deletedRows( 
            /* [in] */ DBROWCOUNT iRow,
            /* [in] */ DBROWCOUNT cRows) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE aboutToInsertRows( 
            /* [in] */ DBROWCOUNT iRow,
            /* [in] */ DBROWCOUNT cRows) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE insertedRows( 
            /* [in] */ DBROWCOUNT iRow,
            /* [in] */ DBROWCOUNT cRows) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE rowsAvailable( 
            /* [in] */ DBROWCOUNT iRow,
            /* [in] */ DBROWCOUNT cRows) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE transferComplete( 
            /* [in] */ OSPXFER xfer) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct OLEDBSimpleProviderListenerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in OLEDBSimpleProviderListener * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in OLEDBSimpleProviderListener * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in OLEDBSimpleProviderListener * This);
        
        DECLSPEC_XFGVIRT(OLEDBSimpleProviderListener, aboutToChangeCell)
        HRESULT ( STDMETHODCALLTYPE *aboutToChangeCell )( 
            __RPC__in OLEDBSimpleProviderListener * This,
            /* [in] */ DBROWCOUNT iRow,
            /* [in] */ DB_LORDINAL iColumn);
        
        DECLSPEC_XFGVIRT(OLEDBSimpleProviderListener, cellChanged)
        HRESULT ( STDMETHODCALLTYPE *cellChanged )( 
            __RPC__in OLEDBSimpleProviderListener * This,
            /* [in] */ DBROWCOUNT iRow,
            /* [in] */ DB_LORDINAL iColumn);
        
        DECLSPEC_XFGVIRT(OLEDBSimpleProviderListener, aboutToDeleteRows)
        HRESULT ( STDMETHODCALLTYPE *aboutToDeleteRows )( 
            __RPC__in OLEDBSimpleProviderListener * This,
            /* [in] */ DBROWCOUNT iRow,
            /* [in] */ DBROWCOUNT cRows);
        
        DECLSPEC_XFGVIRT(OLEDBSimpleProviderListener, deletedRows)
        HRESULT ( STDMETHODCALLTYPE *deletedRows )( 
            __RPC__in OLEDBSimpleProviderListener * This,
            /* [in] */ DBROWCOUNT iRow,
            /* [in] */ DBROWCOUNT cRows);
        
        DECLSPEC_XFGVIRT(OLEDBSimpleProviderListener, aboutToInsertRows)
        HRESULT ( STDMETHODCALLTYPE *aboutToInsertRows )( 
            __RPC__in OLEDBSimpleProviderListener * This,
            /* [in] */ DBROWCOUNT iRow,
            /* [in] */ DBROWCOUNT cRows);
        
        DECLSPEC_XFGVIRT(OLEDBSimpleProviderListener, insertedRows)
        HRESULT ( STDMETHODCALLTYPE *insertedRows )( 
            __RPC__in OLEDBSimpleProviderListener * This,
            /* [in] */ DBROWCOUNT iRow,
            /* [in] */ DBROWCOUNT cRows);
        
        DECLSPEC_XFGVIRT(OLEDBSimpleProviderListener, rowsAvailable)
        HRESULT ( STDMETHODCALLTYPE *rowsAvailable )( 
            __RPC__in OLEDBSimpleProviderListener * This,
            /* [in] */ DBROWCOUNT iRow,
            /* [in] */ DBROWCOUNT cRows);
        
        DECLSPEC_XFGVIRT(OLEDBSimpleProviderListener, transferComplete)
        HRESULT ( STDMETHODCALLTYPE *transferComplete )( 
            __RPC__in OLEDBSimpleProviderListener * This,
            /* [in] */ OSPXFER xfer);
        
        END_INTERFACE
    } OLEDBSimpleProviderListenerVtbl;

    interface OLEDBSimpleProviderListener
    {
        CONST_VTBL struct OLEDBSimpleProviderListenerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define OLEDBSimpleProviderListener_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define OLEDBSimpleProviderListener_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define OLEDBSimpleProviderListener_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define OLEDBSimpleProviderListener_aboutToChangeCell(This,iRow,iColumn)	\
    ( (This)->lpVtbl -> aboutToChangeCell(This,iRow,iColumn) ) 

#define OLEDBSimpleProviderListener_cellChanged(This,iRow,iColumn)	\
    ( (This)->lpVtbl -> cellChanged(This,iRow,iColumn) ) 

#define OLEDBSimpleProviderListener_aboutToDeleteRows(This,iRow,cRows)	\
    ( (This)->lpVtbl -> aboutToDeleteRows(This,iRow,cRows) ) 

#define OLEDBSimpleProviderListener_deletedRows(This,iRow,cRows)	\
    ( (This)->lpVtbl -> deletedRows(This,iRow,cRows) ) 

#define OLEDBSimpleProviderListener_aboutToInsertRows(This,iRow,cRows)	\
    ( (This)->lpVtbl -> aboutToInsertRows(This,iRow,cRows) ) 

#define OLEDBSimpleProviderListener_insertedRows(This,iRow,cRows)	\
    ( (This)->lpVtbl -> insertedRows(This,iRow,cRows) ) 

#define OLEDBSimpleProviderListener_rowsAvailable(This,iRow,cRows)	\
    ( (This)->lpVtbl -> rowsAvailable(This,iRow,cRows) ) 

#define OLEDBSimpleProviderListener_transferComplete(This,xfer)	\
    ( (This)->lpVtbl -> transferComplete(This,xfer) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __OLEDBSimpleProviderListener_INTERFACE_DEFINED__ */


#ifndef __OLEDBSimpleProvider_INTERFACE_DEFINED__
#define __OLEDBSimpleProvider_INTERFACE_DEFINED__

/* interface OLEDBSimpleProvider */
/* [version][oleautomation][unique][uuid][object] */ 


EXTERN_C const IID IID_OLEDBSimpleProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("E0E270C0-C0BE-11d0-8FE4-00A0C90A6341")
    OLEDBSimpleProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE getRowCount( 
            /* [retval][out] */ __RPC__out DBROWCOUNT *pcRows) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE getColumnCount( 
            /* [retval][out] */ __RPC__out DB_LORDINAL *pcColumns) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE getRWStatus( 
            /* [in] */ DBROWCOUNT iRow,
            /* [in] */ DB_LORDINAL iColumn,
            /* [retval][out] */ __RPC__out OSPRW *prwStatus) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE getVariant( 
            /* [in] */ DBROWCOUNT iRow,
            /* [in] */ DB_LORDINAL iColumn,
            /* [in] */ OSPFORMAT format,
            /* [retval][out] */ __RPC__out VARIANT *pVar) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE setVariant( 
            /* [in] */ DBROWCOUNT iRow,
            /* [in] */ DB_LORDINAL iColumn,
            /* [in] */ OSPFORMAT format,
            /* [in] */ VARIANT Var) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE getLocale( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrLocale) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE deleteRows( 
            /* [in] */ DBROWCOUNT iRow,
            /* [in] */ DBROWCOUNT cRows,
            /* [retval][out] */ __RPC__out DBROWCOUNT *pcRowsDeleted) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE insertRows( 
            /* [in] */ DBROWCOUNT iRow,
            /* [in] */ DBROWCOUNT cRows,
            /* [retval][out] */ __RPC__out DBROWCOUNT *pcRowsInserted) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE find( 
            /* [in] */ DBROWCOUNT iRowStart,
            /* [in] */ DB_LORDINAL iColumn,
            /* [in] */ VARIANT val,
            /* [in] */ OSPFIND findFlags,
            /* [in] */ OSPCOMP compType,
            /* [retval][out] */ __RPC__out DBROWCOUNT *piRowFound) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE addOLEDBSimpleProviderListener( 
            /* [in] */ __RPC__in_opt OLEDBSimpleProviderListener *pospIListener) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE removeOLEDBSimpleProviderListener( 
            /* [in] */ __RPC__in_opt OLEDBSimpleProviderListener *pospIListener) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE isAsync( 
            /* [retval][out] */ __RPC__out BOOL *pbAsynch) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE getEstimatedRows( 
            /* [retval][out] */ __RPC__out DBROWCOUNT *piRows) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE stopTransfer( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct OLEDBSimpleProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in OLEDBSimpleProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in OLEDBSimpleProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in OLEDBSimpleProvider * This);
        
        DECLSPEC_XFGVIRT(OLEDBSimpleProvider, getRowCount)
        HRESULT ( STDMETHODCALLTYPE *getRowCount )( 
            __RPC__in OLEDBSimpleProvider * This,
            /* [retval][out] */ __RPC__out DBROWCOUNT *pcRows);
        
        DECLSPEC_XFGVIRT(OLEDBSimpleProvider, getColumnCount)
        HRESULT ( STDMETHODCALLTYPE *getColumnCount )( 
            __RPC__in OLEDBSimpleProvider * This,
            /* [retval][out] */ __RPC__out DB_LORDINAL *pcColumns);
        
        DECLSPEC_XFGVIRT(OLEDBSimpleProvider, getRWStatus)
        HRESULT ( STDMETHODCALLTYPE *getRWStatus )( 
            __RPC__in OLEDBSimpleProvider * This,
            /* [in] */ DBROWCOUNT iRow,
            /* [in] */ DB_LORDINAL iColumn,
            /* [retval][out] */ __RPC__out OSPRW *prwStatus);
        
        DECLSPEC_XFGVIRT(OLEDBSimpleProvider, getVariant)
        HRESULT ( STDMETHODCALLTYPE *getVariant )( 
            __RPC__in OLEDBSimpleProvider * This,
            /* [in] */ DBROWCOUNT iRow,
            /* [in] */ DB_LORDINAL iColumn,
            /* [in] */ OSPFORMAT format,
            /* [retval][out] */ __RPC__out VARIANT *pVar);
        
        DECLSPEC_XFGVIRT(OLEDBSimpleProvider, setVariant)
        HRESULT ( STDMETHODCALLTYPE *setVariant )( 
            __RPC__in OLEDBSimpleProvider * This,
            /* [in] */ DBROWCOUNT iRow,
            /* [in] */ DB_LORDINAL iColumn,
            /* [in] */ OSPFORMAT format,
            /* [in] */ VARIANT Var);
        
        DECLSPEC_XFGVIRT(OLEDBSimpleProvider, getLocale)
        HRESULT ( STDMETHODCALLTYPE *getLocale )( 
            __RPC__in OLEDBSimpleProvider * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrLocale);
        
        DECLSPEC_XFGVIRT(OLEDBSimpleProvider, deleteRows)
        HRESULT ( STDMETHODCALLTYPE *deleteRows )( 
            __RPC__in OLEDBSimpleProvider * This,
            /* [in] */ DBROWCOUNT iRow,
            /* [in] */ DBROWCOUNT cRows,
            /* [retval][out] */ __RPC__out DBROWCOUNT *pcRowsDeleted);
        
        DECLSPEC_XFGVIRT(OLEDBSimpleProvider, insertRows)
        HRESULT ( STDMETHODCALLTYPE *insertRows )( 
            __RPC__in OLEDBSimpleProvider * This,
            /* [in] */ DBROWCOUNT iRow,
            /* [in] */ DBROWCOUNT cRows,
            /* [retval][out] */ __RPC__out DBROWCOUNT *pcRowsInserted);
        
        DECLSPEC_XFGVIRT(OLEDBSimpleProvider, find)
        HRESULT ( STDMETHODCALLTYPE *find )( 
            __RPC__in OLEDBSimpleProvider * This,
            /* [in] */ DBROWCOUNT iRowStart,
            /* [in] */ DB_LORDINAL iColumn,
            /* [in] */ VARIANT val,
            /* [in] */ OSPFIND findFlags,
            /* [in] */ OSPCOMP compType,
            /* [retval][out] */ __RPC__out DBROWCOUNT *piRowFound);
        
        DECLSPEC_XFGVIRT(OLEDBSimpleProvider, addOLEDBSimpleProviderListener)
        HRESULT ( STDMETHODCALLTYPE *addOLEDBSimpleProviderListener )( 
            __RPC__in OLEDBSimpleProvider * This,
            /* [in] */ __RPC__in_opt OLEDBSimpleProviderListener *pospIListener);
        
        DECLSPEC_XFGVIRT(OLEDBSimpleProvider, removeOLEDBSimpleProviderListener)
        HRESULT ( STDMETHODCALLTYPE *removeOLEDBSimpleProviderListener )( 
            __RPC__in OLEDBSimpleProvider * This,
            /* [in] */ __RPC__in_opt OLEDBSimpleProviderListener *pospIListener);
        
        DECLSPEC_XFGVIRT(OLEDBSimpleProvider, isAsync)
        HRESULT ( STDMETHODCALLTYPE *isAsync )( 
            __RPC__in OLEDBSimpleProvider * This,
            /* [retval][out] */ __RPC__out BOOL *pbAsynch);
        
        DECLSPEC_XFGVIRT(OLEDBSimpleProvider, getEstimatedRows)
        HRESULT ( STDMETHODCALLTYPE *getEstimatedRows )( 
            __RPC__in OLEDBSimpleProvider * This,
            /* [retval][out] */ __RPC__out DBROWCOUNT *piRows);
        
        DECLSPEC_XFGVIRT(OLEDBSimpleProvider, stopTransfer)
        HRESULT ( STDMETHODCALLTYPE *stopTransfer )( 
            __RPC__in OLEDBSimpleProvider * This);
        
        END_INTERFACE
    } OLEDBSimpleProviderVtbl;

    interface OLEDBSimpleProvider
    {
        CONST_VTBL struct OLEDBSimpleProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define OLEDBSimpleProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define OLEDBSimpleProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define OLEDBSimpleProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define OLEDBSimpleProvider_getRowCount(This,pcRows)	\
    ( (This)->lpVtbl -> getRowCount(This,pcRows) ) 

#define OLEDBSimpleProvider_getColumnCount(This,pcColumns)	\
    ( (This)->lpVtbl -> getColumnCount(This,pcColumns) ) 

#define OLEDBSimpleProvider_getRWStatus(This,iRow,iColumn,prwStatus)	\
    ( (This)->lpVtbl -> getRWStatus(This,iRow,iColumn,prwStatus) ) 

#define OLEDBSimpleProvider_getVariant(This,iRow,iColumn,format,pVar)	\
    ( (This)->lpVtbl -> getVariant(This,iRow,iColumn,format,pVar) ) 

#define OLEDBSimpleProvider_setVariant(This,iRow,iColumn,format,Var)	\
    ( (This)->lpVtbl -> setVariant(This,iRow,iColumn,format,Var) ) 

#define OLEDBSimpleProvider_getLocale(This,pbstrLocale)	\
    ( (This)->lpVtbl -> getLocale(This,pbstrLocale) ) 

#define OLEDBSimpleProvider_deleteRows(This,iRow,cRows,pcRowsDeleted)	\
    ( (This)->lpVtbl -> deleteRows(This,iRow,cRows,pcRowsDeleted) ) 

#define OLEDBSimpleProvider_insertRows(This,iRow,cRows,pcRowsInserted)	\
    ( (This)->lpVtbl -> insertRows(This,iRow,cRows,pcRowsInserted) ) 

#define OLEDBSimpleProvider_find(This,iRowStart,iColumn,val,findFlags,compType,piRowFound)	\
    ( (This)->lpVtbl -> find(This,iRowStart,iColumn,val,findFlags,compType,piRowFound) ) 

#define OLEDBSimpleProvider_addOLEDBSimpleProviderListener(This,pospIListener)	\
    ( (This)->lpVtbl -> addOLEDBSimpleProviderListener(This,pospIListener) ) 

#define OLEDBSimpleProvider_removeOLEDBSimpleProviderListener(This,pospIListener)	\
    ( (This)->lpVtbl -> removeOLEDBSimpleProviderListener(This,pospIListener) ) 

#define OLEDBSimpleProvider_isAsync(This,pbAsynch)	\
    ( (This)->lpVtbl -> isAsync(This,pbAsynch) ) 

#define OLEDBSimpleProvider_getEstimatedRows(This,piRows)	\
    ( (This)->lpVtbl -> getEstimatedRows(This,piRows) ) 

#define OLEDBSimpleProvider_stopTransfer(This)	\
    ( (This)->lpVtbl -> stopTransfer(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __OLEDBSimpleProvider_INTERFACE_DEFINED__ */

#endif /* __MSDAOSP_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_simpdata_0000_0001 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#endif


extern RPC_IF_HANDLE __MIDL_itf_simpdata_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_simpdata_0000_0001_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


