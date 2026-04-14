

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


#ifndef __isysmon_h__
#define __isysmon_h__

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

#ifndef __ICounterItem_FWD_DEFINED__
#define __ICounterItem_FWD_DEFINED__
typedef interface ICounterItem ICounterItem;

#endif 	/* __ICounterItem_FWD_DEFINED__ */


#ifndef __ICounterItem2_FWD_DEFINED__
#define __ICounterItem2_FWD_DEFINED__
typedef interface ICounterItem2 ICounterItem2;

#endif 	/* __ICounterItem2_FWD_DEFINED__ */


#ifndef ___ICounterItemUnion_FWD_DEFINED__
#define ___ICounterItemUnion_FWD_DEFINED__
typedef interface _ICounterItemUnion _ICounterItemUnion;

#endif 	/* ___ICounterItemUnion_FWD_DEFINED__ */


#ifndef __DICounterItem_FWD_DEFINED__
#define __DICounterItem_FWD_DEFINED__
typedef interface DICounterItem DICounterItem;

#endif 	/* __DICounterItem_FWD_DEFINED__ */


#ifndef __ICounters_FWD_DEFINED__
#define __ICounters_FWD_DEFINED__
typedef interface ICounters ICounters;

#endif 	/* __ICounters_FWD_DEFINED__ */


#ifndef __ILogFileItem_FWD_DEFINED__
#define __ILogFileItem_FWD_DEFINED__
typedef interface ILogFileItem ILogFileItem;

#endif 	/* __ILogFileItem_FWD_DEFINED__ */


#ifndef __DILogFileItem_FWD_DEFINED__
#define __DILogFileItem_FWD_DEFINED__
typedef interface DILogFileItem DILogFileItem;

#endif 	/* __DILogFileItem_FWD_DEFINED__ */


#ifndef __ILogFiles_FWD_DEFINED__
#define __ILogFiles_FWD_DEFINED__
typedef interface ILogFiles ILogFiles;

#endif 	/* __ILogFiles_FWD_DEFINED__ */


#ifndef __ISystemMonitor_FWD_DEFINED__
#define __ISystemMonitor_FWD_DEFINED__
typedef interface ISystemMonitor ISystemMonitor;

#endif 	/* __ISystemMonitor_FWD_DEFINED__ */


#ifndef __ISystemMonitor2_FWD_DEFINED__
#define __ISystemMonitor2_FWD_DEFINED__
typedef interface ISystemMonitor2 ISystemMonitor2;

#endif 	/* __ISystemMonitor2_FWD_DEFINED__ */


#ifndef ___ISystemMonitorUnion_FWD_DEFINED__
#define ___ISystemMonitorUnion_FWD_DEFINED__
typedef interface _ISystemMonitorUnion _ISystemMonitorUnion;

#endif 	/* ___ISystemMonitorUnion_FWD_DEFINED__ */


#ifndef __DISystemMonitor_FWD_DEFINED__
#define __DISystemMonitor_FWD_DEFINED__
typedef interface DISystemMonitor DISystemMonitor;

#endif 	/* __DISystemMonitor_FWD_DEFINED__ */


#ifndef __DISystemMonitorInternal_FWD_DEFINED__
#define __DISystemMonitorInternal_FWD_DEFINED__
typedef interface DISystemMonitorInternal DISystemMonitorInternal;

#endif 	/* __DISystemMonitorInternal_FWD_DEFINED__ */


#ifndef __ISystemMonitorEvents_FWD_DEFINED__
#define __ISystemMonitorEvents_FWD_DEFINED__
typedef interface ISystemMonitorEvents ISystemMonitorEvents;

#endif 	/* __ISystemMonitorEvents_FWD_DEFINED__ */


#ifndef __DISystemMonitorEvents_FWD_DEFINED__
#define __DISystemMonitorEvents_FWD_DEFINED__
typedef interface DISystemMonitorEvents DISystemMonitorEvents;

#endif 	/* __DISystemMonitorEvents_FWD_DEFINED__ */


#ifndef __SystemMonitor_FWD_DEFINED__
#define __SystemMonitor_FWD_DEFINED__

#ifdef __cplusplus
typedef class SystemMonitor SystemMonitor;
#else
typedef struct SystemMonitor SystemMonitor;
#endif /* __cplusplus */

#endif 	/* __SystemMonitor_FWD_DEFINED__ */


#ifndef __CounterItem_FWD_DEFINED__
#define __CounterItem_FWD_DEFINED__

#ifdef __cplusplus
typedef class CounterItem CounterItem;
#else
typedef struct CounterItem CounterItem;
#endif /* __cplusplus */

#endif 	/* __CounterItem_FWD_DEFINED__ */


#ifndef __Counters_FWD_DEFINED__
#define __Counters_FWD_DEFINED__

#ifdef __cplusplus
typedef class Counters Counters;
#else
typedef struct Counters Counters;
#endif /* __cplusplus */

#endif 	/* __Counters_FWD_DEFINED__ */


#ifndef __LogFileItem_FWD_DEFINED__
#define __LogFileItem_FWD_DEFINED__

#ifdef __cplusplus
typedef class LogFileItem LogFileItem;
#else
typedef struct LogFileItem LogFileItem;
#endif /* __cplusplus */

#endif 	/* __LogFileItem_FWD_DEFINED__ */


#ifndef __LogFiles_FWD_DEFINED__
#define __LogFiles_FWD_DEFINED__

#ifdef __cplusplus
typedef class LogFiles LogFiles;
#else
typedef struct LogFiles LogFiles;
#endif /* __cplusplus */

#endif 	/* __LogFiles_FWD_DEFINED__ */


#ifndef __CounterItem2_FWD_DEFINED__
#define __CounterItem2_FWD_DEFINED__

#ifdef __cplusplus
typedef class CounterItem2 CounterItem2;
#else
typedef struct CounterItem2 CounterItem2;
#endif /* __cplusplus */

#endif 	/* __CounterItem2_FWD_DEFINED__ */


#ifndef __SystemMonitor2_FWD_DEFINED__
#define __SystemMonitor2_FWD_DEFINED__

#ifdef __cplusplus
typedef class SystemMonitor2 SystemMonitor2;
#else
typedef struct SystemMonitor2 SystemMonitor2;
#endif /* __cplusplus */

#endif 	/* __SystemMonitor2_FWD_DEFINED__ */


#ifndef __AppearPropPage_FWD_DEFINED__
#define __AppearPropPage_FWD_DEFINED__

#ifdef __cplusplus
typedef class AppearPropPage AppearPropPage;
#else
typedef struct AppearPropPage AppearPropPage;
#endif /* __cplusplus */

#endif 	/* __AppearPropPage_FWD_DEFINED__ */


#ifndef __GeneralPropPage_FWD_DEFINED__
#define __GeneralPropPage_FWD_DEFINED__

#ifdef __cplusplus
typedef class GeneralPropPage GeneralPropPage;
#else
typedef struct GeneralPropPage GeneralPropPage;
#endif /* __cplusplus */

#endif 	/* __GeneralPropPage_FWD_DEFINED__ */


#ifndef __GraphPropPage_FWD_DEFINED__
#define __GraphPropPage_FWD_DEFINED__

#ifdef __cplusplus
typedef class GraphPropPage GraphPropPage;
#else
typedef struct GraphPropPage GraphPropPage;
#endif /* __cplusplus */

#endif 	/* __GraphPropPage_FWD_DEFINED__ */


#ifndef __SourcePropPage_FWD_DEFINED__
#define __SourcePropPage_FWD_DEFINED__

#ifdef __cplusplus
typedef class SourcePropPage SourcePropPage;
#else
typedef struct SourcePropPage SourcePropPage;
#endif /* __cplusplus */

#endif 	/* __SourcePropPage_FWD_DEFINED__ */


#ifndef __CounterPropPage_FWD_DEFINED__
#define __CounterPropPage_FWD_DEFINED__

#ifdef __cplusplus
typedef class CounterPropPage CounterPropPage;
#else
typedef struct CounterPropPage CounterPropPage;
#endif /* __cplusplus */

#endif 	/* __CounterPropPage_FWD_DEFINED__ */


/* header files for imported files */
#include "ocidl.h"

#ifdef __cplusplus
extern "C"{
#endif 



#ifndef __SystemMonitor_LIBRARY_DEFINED__
#define __SystemMonitor_LIBRARY_DEFINED__

/* library SystemMonitor */
/* [version][lcid][helpstring][uuid] */ 

typedef /* [helpstring] */ 
enum eDisplayTypeConstant
    {
        sysmonLineGraph	= 1,
        sysmonHistogram	= 2,
        sysmonReport	= 3,
        sysmonChartArea	= 4,
        sysmonChartStackedArea	= 5
    } 	DisplayTypeConstants;

typedef /* [helpstring] */ 
enum eReportValueTypeConstant
    {
        sysmonDefaultValue	= 0,
        sysmonCurrentValue	= 0x1,
        sysmonAverage	= 0x2,
        sysmonMinimum	= 0x3,
        sysmonMaximum	= 0x4
    } 	ReportValueTypeConstants;

typedef /* [helpstring] */ 
enum eDataSourceTypeConstant
    {
        sysmonNullDataSource	= 0xffffffff,
        sysmonCurrentActivity	= 0x1,
        sysmonLogFiles	= 0x2,
        sysmonSqlLog	= 0x3
    } 	DataSourceTypeConstants;

typedef /* [helpstring] */ 
enum __MIDL___MIDL_itf_sysmon_0000_0000_0001
    {
        sysmonFileHtml	= 1,
        sysmonFileReport	= 2,
        sysmonFileCsv	= 3,
        sysmonFileTsv	= 4,
        sysmonFileBlg	= 5,
        sysmonFileRetiredBlg	= 6,
        sysmonFileGif	= 7
    } 	SysmonFileType;

typedef /* [helpstring] */ 
enum __MIDL___MIDL_itf_sysmon_0000_0000_0002
    {
        sysmonDataAvg	= 1,
        sysmonDataMin	= 2,
        sysmonDataMax	= 3,
        sysmonDataTime	= 4,
        sysmonDataCount	= 5
    } 	SysmonDataType;

typedef /* [helpstring] */ 
enum __MIDL___MIDL_itf_sysmon_0000_0000_0003
    {
        sysmonBatchNone	= 0,
        sysmonBatchAddFiles	= 1,
        sysmonBatchAddCounters	= 2,
        sysmonBatchAddFilesAutoCounters	= 3
    } 	SysmonBatchReason;


DEFINE_GUID(LIBID_SystemMonitor,0x1B773E42,0x2509,0x11cf,0x94,0x2F,0x00,0x80,0x29,0x00,0x43,0x47);

#ifndef __ICounterItem_INTERFACE_DEFINED__
#define __ICounterItem_INTERFACE_DEFINED__

/* interface ICounterItem */
/* [object][hidden][helpstring][uuid] */ 


DEFINE_GUID(IID_ICounterItem,0x771A9520,0xEE28,0x11ce,0x94,0x1E,0x00,0x80,0x29,0x00,0x43,0x47);

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("771A9520-EE28-11ce-941E-008029004347")
    ICounterItem : public IUnknown
    {
    public:
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Value( 
            /* [retval][out] */ __RPC__out double *pdblValue) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Color( 
            /* [in] */ OLE_COLOR Color) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_Color( 
            /* [retval][out] */ __RPC__out OLE_COLOR *pColor) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Width( 
            /* [in] */ INT iWidth) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_Width( 
            /* [retval][out] */ __RPC__out INT *piValue) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_LineStyle( 
            /* [in] */ INT iLineStyle) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_LineStyle( 
            /* [retval][out] */ __RPC__out INT *piValue) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_ScaleFactor( 
            /* [in] */ INT iScale) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_ScaleFactor( 
            /* [retval][out] */ __RPC__out INT *piValue) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_Path( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetValue( 
            /* [out] */ __RPC__out double *Value,
            /* [out] */ __RPC__out long *Status) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStatistics( 
            /* [out] */ __RPC__out double *Max,
            /* [out] */ __RPC__out double *Min,
            /* [out] */ __RPC__out double *Avg,
            /* [out] */ __RPC__out long *Status) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICounterItemVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICounterItem * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICounterItem * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICounterItem * This);
        
        DECLSPEC_XFGVIRT(ICounterItem, get_Value)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Value )( 
            __RPC__in ICounterItem * This,
            /* [retval][out] */ __RPC__out double *pdblValue);
        
        DECLSPEC_XFGVIRT(ICounterItem, put_Color)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Color )( 
            __RPC__in ICounterItem * This,
            /* [in] */ OLE_COLOR Color);
        
        DECLSPEC_XFGVIRT(ICounterItem, get_Color)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Color )( 
            __RPC__in ICounterItem * This,
            /* [retval][out] */ __RPC__out OLE_COLOR *pColor);
        
        DECLSPEC_XFGVIRT(ICounterItem, put_Width)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Width )( 
            __RPC__in ICounterItem * This,
            /* [in] */ INT iWidth);
        
        DECLSPEC_XFGVIRT(ICounterItem, get_Width)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Width )( 
            __RPC__in ICounterItem * This,
            /* [retval][out] */ __RPC__out INT *piValue);
        
        DECLSPEC_XFGVIRT(ICounterItem, put_LineStyle)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_LineStyle )( 
            __RPC__in ICounterItem * This,
            /* [in] */ INT iLineStyle);
        
        DECLSPEC_XFGVIRT(ICounterItem, get_LineStyle)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_LineStyle )( 
            __RPC__in ICounterItem * This,
            /* [retval][out] */ __RPC__out INT *piValue);
        
        DECLSPEC_XFGVIRT(ICounterItem, put_ScaleFactor)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ScaleFactor )( 
            __RPC__in ICounterItem * This,
            /* [in] */ INT iScale);
        
        DECLSPEC_XFGVIRT(ICounterItem, get_ScaleFactor)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ScaleFactor )( 
            __RPC__in ICounterItem * This,
            /* [retval][out] */ __RPC__out INT *piValue);
        
        DECLSPEC_XFGVIRT(ICounterItem, get_Path)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Path )( 
            __RPC__in ICounterItem * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrValue);
        
        DECLSPEC_XFGVIRT(ICounterItem, GetValue)
        HRESULT ( STDMETHODCALLTYPE *GetValue )( 
            __RPC__in ICounterItem * This,
            /* [out] */ __RPC__out double *Value,
            /* [out] */ __RPC__out long *Status);
        
        DECLSPEC_XFGVIRT(ICounterItem, GetStatistics)
        HRESULT ( STDMETHODCALLTYPE *GetStatistics )( 
            __RPC__in ICounterItem * This,
            /* [out] */ __RPC__out double *Max,
            /* [out] */ __RPC__out double *Min,
            /* [out] */ __RPC__out double *Avg,
            /* [out] */ __RPC__out long *Status);
        
        END_INTERFACE
    } ICounterItemVtbl;

    interface ICounterItem
    {
        CONST_VTBL struct ICounterItemVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICounterItem_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICounterItem_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICounterItem_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICounterItem_get_Value(This,pdblValue)	\
    ( (This)->lpVtbl -> get_Value(This,pdblValue) ) 

#define ICounterItem_put_Color(This,Color)	\
    ( (This)->lpVtbl -> put_Color(This,Color) ) 

#define ICounterItem_get_Color(This,pColor)	\
    ( (This)->lpVtbl -> get_Color(This,pColor) ) 

#define ICounterItem_put_Width(This,iWidth)	\
    ( (This)->lpVtbl -> put_Width(This,iWidth) ) 

#define ICounterItem_get_Width(This,piValue)	\
    ( (This)->lpVtbl -> get_Width(This,piValue) ) 

#define ICounterItem_put_LineStyle(This,iLineStyle)	\
    ( (This)->lpVtbl -> put_LineStyle(This,iLineStyle) ) 

#define ICounterItem_get_LineStyle(This,piValue)	\
    ( (This)->lpVtbl -> get_LineStyle(This,piValue) ) 

#define ICounterItem_put_ScaleFactor(This,iScale)	\
    ( (This)->lpVtbl -> put_ScaleFactor(This,iScale) ) 

#define ICounterItem_get_ScaleFactor(This,piValue)	\
    ( (This)->lpVtbl -> get_ScaleFactor(This,piValue) ) 

#define ICounterItem_get_Path(This,pstrValue)	\
    ( (This)->lpVtbl -> get_Path(This,pstrValue) ) 

#define ICounterItem_GetValue(This,Value,Status)	\
    ( (This)->lpVtbl -> GetValue(This,Value,Status) ) 

#define ICounterItem_GetStatistics(This,Max,Min,Avg,Status)	\
    ( (This)->lpVtbl -> GetStatistics(This,Max,Min,Avg,Status) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICounterItem_INTERFACE_DEFINED__ */


#ifndef __ICounterItem2_INTERFACE_DEFINED__
#define __ICounterItem2_INTERFACE_DEFINED__

/* interface ICounterItem2 */
/* [object][helpstring][uuid] */ 


DEFINE_GUID(IID_ICounterItem2,0xeefcd4e1,0xea1c,0x4435,0xb7,0xf4,0xe3,0x41,0xba,0x03,0xb4,0xf9);

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("eefcd4e1-ea1c-4435-b7f4-e341ba03b4f9")
    ICounterItem2 : public ICounterItem
    {
    public:
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Selected( 
            /* [in] */ VARIANT_BOOL bState) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_Selected( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbState) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Visible( 
            /* [in] */ VARIANT_BOOL bState) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_Visible( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbState) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetDataAt( 
            /* [in] */ INT iIndex,
            /* [in] */ SysmonDataType iWhich,
            /* [out] */ __RPC__out VARIANT *pVariant) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICounterItem2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICounterItem2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICounterItem2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICounterItem2 * This);
        
        DECLSPEC_XFGVIRT(ICounterItem, get_Value)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Value )( 
            __RPC__in ICounterItem2 * This,
            /* [retval][out] */ __RPC__out double *pdblValue);
        
        DECLSPEC_XFGVIRT(ICounterItem, put_Color)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Color )( 
            __RPC__in ICounterItem2 * This,
            /* [in] */ OLE_COLOR Color);
        
        DECLSPEC_XFGVIRT(ICounterItem, get_Color)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Color )( 
            __RPC__in ICounterItem2 * This,
            /* [retval][out] */ __RPC__out OLE_COLOR *pColor);
        
        DECLSPEC_XFGVIRT(ICounterItem, put_Width)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Width )( 
            __RPC__in ICounterItem2 * This,
            /* [in] */ INT iWidth);
        
        DECLSPEC_XFGVIRT(ICounterItem, get_Width)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Width )( 
            __RPC__in ICounterItem2 * This,
            /* [retval][out] */ __RPC__out INT *piValue);
        
        DECLSPEC_XFGVIRT(ICounterItem, put_LineStyle)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_LineStyle )( 
            __RPC__in ICounterItem2 * This,
            /* [in] */ INT iLineStyle);
        
        DECLSPEC_XFGVIRT(ICounterItem, get_LineStyle)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_LineStyle )( 
            __RPC__in ICounterItem2 * This,
            /* [retval][out] */ __RPC__out INT *piValue);
        
        DECLSPEC_XFGVIRT(ICounterItem, put_ScaleFactor)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ScaleFactor )( 
            __RPC__in ICounterItem2 * This,
            /* [in] */ INT iScale);
        
        DECLSPEC_XFGVIRT(ICounterItem, get_ScaleFactor)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ScaleFactor )( 
            __RPC__in ICounterItem2 * This,
            /* [retval][out] */ __RPC__out INT *piValue);
        
        DECLSPEC_XFGVIRT(ICounterItem, get_Path)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Path )( 
            __RPC__in ICounterItem2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrValue);
        
        DECLSPEC_XFGVIRT(ICounterItem, GetValue)
        HRESULT ( STDMETHODCALLTYPE *GetValue )( 
            __RPC__in ICounterItem2 * This,
            /* [out] */ __RPC__out double *Value,
            /* [out] */ __RPC__out long *Status);
        
        DECLSPEC_XFGVIRT(ICounterItem, GetStatistics)
        HRESULT ( STDMETHODCALLTYPE *GetStatistics )( 
            __RPC__in ICounterItem2 * This,
            /* [out] */ __RPC__out double *Max,
            /* [out] */ __RPC__out double *Min,
            /* [out] */ __RPC__out double *Avg,
            /* [out] */ __RPC__out long *Status);
        
        DECLSPEC_XFGVIRT(ICounterItem2, put_Selected)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Selected )( 
            __RPC__in ICounterItem2 * This,
            /* [in] */ VARIANT_BOOL bState);
        
        DECLSPEC_XFGVIRT(ICounterItem2, get_Selected)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Selected )( 
            __RPC__in ICounterItem2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbState);
        
        DECLSPEC_XFGVIRT(ICounterItem2, put_Visible)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Visible )( 
            __RPC__in ICounterItem2 * This,
            /* [in] */ VARIANT_BOOL bState);
        
        DECLSPEC_XFGVIRT(ICounterItem2, get_Visible)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Visible )( 
            __RPC__in ICounterItem2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbState);
        
        DECLSPEC_XFGVIRT(ICounterItem2, GetDataAt)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetDataAt )( 
            __RPC__in ICounterItem2 * This,
            /* [in] */ INT iIndex,
            /* [in] */ SysmonDataType iWhich,
            /* [out] */ __RPC__out VARIANT *pVariant);
        
        END_INTERFACE
    } ICounterItem2Vtbl;

    interface ICounterItem2
    {
        CONST_VTBL struct ICounterItem2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICounterItem2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICounterItem2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICounterItem2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICounterItem2_get_Value(This,pdblValue)	\
    ( (This)->lpVtbl -> get_Value(This,pdblValue) ) 

#define ICounterItem2_put_Color(This,Color)	\
    ( (This)->lpVtbl -> put_Color(This,Color) ) 

#define ICounterItem2_get_Color(This,pColor)	\
    ( (This)->lpVtbl -> get_Color(This,pColor) ) 

#define ICounterItem2_put_Width(This,iWidth)	\
    ( (This)->lpVtbl -> put_Width(This,iWidth) ) 

#define ICounterItem2_get_Width(This,piValue)	\
    ( (This)->lpVtbl -> get_Width(This,piValue) ) 

#define ICounterItem2_put_LineStyle(This,iLineStyle)	\
    ( (This)->lpVtbl -> put_LineStyle(This,iLineStyle) ) 

#define ICounterItem2_get_LineStyle(This,piValue)	\
    ( (This)->lpVtbl -> get_LineStyle(This,piValue) ) 

#define ICounterItem2_put_ScaleFactor(This,iScale)	\
    ( (This)->lpVtbl -> put_ScaleFactor(This,iScale) ) 

#define ICounterItem2_get_ScaleFactor(This,piValue)	\
    ( (This)->lpVtbl -> get_ScaleFactor(This,piValue) ) 

#define ICounterItem2_get_Path(This,pstrValue)	\
    ( (This)->lpVtbl -> get_Path(This,pstrValue) ) 

#define ICounterItem2_GetValue(This,Value,Status)	\
    ( (This)->lpVtbl -> GetValue(This,Value,Status) ) 

#define ICounterItem2_GetStatistics(This,Max,Min,Avg,Status)	\
    ( (This)->lpVtbl -> GetStatistics(This,Max,Min,Avg,Status) ) 


#define ICounterItem2_put_Selected(This,bState)	\
    ( (This)->lpVtbl -> put_Selected(This,bState) ) 

#define ICounterItem2_get_Selected(This,pbState)	\
    ( (This)->lpVtbl -> get_Selected(This,pbState) ) 

#define ICounterItem2_put_Visible(This,bState)	\
    ( (This)->lpVtbl -> put_Visible(This,bState) ) 

#define ICounterItem2_get_Visible(This,pbState)	\
    ( (This)->lpVtbl -> get_Visible(This,pbState) ) 

#define ICounterItem2_GetDataAt(This,iIndex,iWhich,pVariant)	\
    ( (This)->lpVtbl -> GetDataAt(This,iIndex,iWhich,pVariant) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICounterItem2_INTERFACE_DEFINED__ */


#ifndef ___ICounterItemUnion_INTERFACE_DEFINED__
#define ___ICounterItemUnion_INTERFACE_DEFINED__

/* interface _ICounterItemUnion */
/* [object][hidden][uuid] */ 


DEFINE_GUID(IID__ICounterItemUnion,0xde1a6b74,0x9182,0x4c41,0x8e,0x2c,0x24,0xc2,0xcd,0x30,0xee,0x83);

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("de1a6b74-9182-4c41-8e2c-24c2cd30ee83")
    _ICounterItemUnion : public IUnknown
    {
    public:
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Value( 
            /* [retval][out] */ __RPC__out double *pdblValue) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Color( 
            /* [in] */ OLE_COLOR Color) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_Color( 
            /* [retval][out] */ __RPC__out OLE_COLOR *pColor) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Width( 
            /* [in] */ INT iWidth) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_Width( 
            /* [retval][out] */ __RPC__out INT *piValue) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_LineStyle( 
            /* [in] */ INT iLineStyle) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_LineStyle( 
            /* [retval][out] */ __RPC__out INT *piValue) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_ScaleFactor( 
            /* [in] */ INT iScale) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_ScaleFactor( 
            /* [retval][out] */ __RPC__out INT *piValue) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_Path( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetValue( 
            /* [out] */ __RPC__out double *Value,
            /* [out] */ __RPC__out long *Status) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStatistics( 
            /* [out] */ __RPC__out double *Max,
            /* [out] */ __RPC__out double *Min,
            /* [out] */ __RPC__out double *Avg,
            /* [out] */ __RPC__out long *Status) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Selected( 
            /* [in] */ VARIANT_BOOL bState) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_Selected( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbState) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Visible( 
            /* [in] */ VARIANT_BOOL bState) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_Visible( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbState) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetDataAt( 
            /* [in] */ INT iIndex,
            /* [in] */ SysmonDataType iWhich,
            /* [out] */ __RPC__out VARIANT *pVariant) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct _ICounterItemUnionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in _ICounterItemUnion * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in _ICounterItemUnion * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in _ICounterItemUnion * This);
        
        DECLSPEC_XFGVIRT(_ICounterItemUnion, get_Value)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Value )( 
            __RPC__in _ICounterItemUnion * This,
            /* [retval][out] */ __RPC__out double *pdblValue);
        
        DECLSPEC_XFGVIRT(_ICounterItemUnion, put_Color)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Color )( 
            __RPC__in _ICounterItemUnion * This,
            /* [in] */ OLE_COLOR Color);
        
        DECLSPEC_XFGVIRT(_ICounterItemUnion, get_Color)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Color )( 
            __RPC__in _ICounterItemUnion * This,
            /* [retval][out] */ __RPC__out OLE_COLOR *pColor);
        
        DECLSPEC_XFGVIRT(_ICounterItemUnion, put_Width)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Width )( 
            __RPC__in _ICounterItemUnion * This,
            /* [in] */ INT iWidth);
        
        DECLSPEC_XFGVIRT(_ICounterItemUnion, get_Width)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Width )( 
            __RPC__in _ICounterItemUnion * This,
            /* [retval][out] */ __RPC__out INT *piValue);
        
        DECLSPEC_XFGVIRT(_ICounterItemUnion, put_LineStyle)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_LineStyle )( 
            __RPC__in _ICounterItemUnion * This,
            /* [in] */ INT iLineStyle);
        
        DECLSPEC_XFGVIRT(_ICounterItemUnion, get_LineStyle)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_LineStyle )( 
            __RPC__in _ICounterItemUnion * This,
            /* [retval][out] */ __RPC__out INT *piValue);
        
        DECLSPEC_XFGVIRT(_ICounterItemUnion, put_ScaleFactor)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ScaleFactor )( 
            __RPC__in _ICounterItemUnion * This,
            /* [in] */ INT iScale);
        
        DECLSPEC_XFGVIRT(_ICounterItemUnion, get_ScaleFactor)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ScaleFactor )( 
            __RPC__in _ICounterItemUnion * This,
            /* [retval][out] */ __RPC__out INT *piValue);
        
        DECLSPEC_XFGVIRT(_ICounterItemUnion, get_Path)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Path )( 
            __RPC__in _ICounterItemUnion * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrValue);
        
        DECLSPEC_XFGVIRT(_ICounterItemUnion, GetValue)
        HRESULT ( STDMETHODCALLTYPE *GetValue )( 
            __RPC__in _ICounterItemUnion * This,
            /* [out] */ __RPC__out double *Value,
            /* [out] */ __RPC__out long *Status);
        
        DECLSPEC_XFGVIRT(_ICounterItemUnion, GetStatistics)
        HRESULT ( STDMETHODCALLTYPE *GetStatistics )( 
            __RPC__in _ICounterItemUnion * This,
            /* [out] */ __RPC__out double *Max,
            /* [out] */ __RPC__out double *Min,
            /* [out] */ __RPC__out double *Avg,
            /* [out] */ __RPC__out long *Status);
        
        DECLSPEC_XFGVIRT(_ICounterItemUnion, put_Selected)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Selected )( 
            __RPC__in _ICounterItemUnion * This,
            /* [in] */ VARIANT_BOOL bState);
        
        DECLSPEC_XFGVIRT(_ICounterItemUnion, get_Selected)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Selected )( 
            __RPC__in _ICounterItemUnion * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbState);
        
        DECLSPEC_XFGVIRT(_ICounterItemUnion, put_Visible)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Visible )( 
            __RPC__in _ICounterItemUnion * This,
            /* [in] */ VARIANT_BOOL bState);
        
        DECLSPEC_XFGVIRT(_ICounterItemUnion, get_Visible)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Visible )( 
            __RPC__in _ICounterItemUnion * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbState);
        
        DECLSPEC_XFGVIRT(_ICounterItemUnion, GetDataAt)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetDataAt )( 
            __RPC__in _ICounterItemUnion * This,
            /* [in] */ INT iIndex,
            /* [in] */ SysmonDataType iWhich,
            /* [out] */ __RPC__out VARIANT *pVariant);
        
        END_INTERFACE
    } _ICounterItemUnionVtbl;

    interface _ICounterItemUnion
    {
        CONST_VTBL struct _ICounterItemUnionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define _ICounterItemUnion_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define _ICounterItemUnion_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define _ICounterItemUnion_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define _ICounterItemUnion_get_Value(This,pdblValue)	\
    ( (This)->lpVtbl -> get_Value(This,pdblValue) ) 

#define _ICounterItemUnion_put_Color(This,Color)	\
    ( (This)->lpVtbl -> put_Color(This,Color) ) 

#define _ICounterItemUnion_get_Color(This,pColor)	\
    ( (This)->lpVtbl -> get_Color(This,pColor) ) 

#define _ICounterItemUnion_put_Width(This,iWidth)	\
    ( (This)->lpVtbl -> put_Width(This,iWidth) ) 

#define _ICounterItemUnion_get_Width(This,piValue)	\
    ( (This)->lpVtbl -> get_Width(This,piValue) ) 

#define _ICounterItemUnion_put_LineStyle(This,iLineStyle)	\
    ( (This)->lpVtbl -> put_LineStyle(This,iLineStyle) ) 

#define _ICounterItemUnion_get_LineStyle(This,piValue)	\
    ( (This)->lpVtbl -> get_LineStyle(This,piValue) ) 

#define _ICounterItemUnion_put_ScaleFactor(This,iScale)	\
    ( (This)->lpVtbl -> put_ScaleFactor(This,iScale) ) 

#define _ICounterItemUnion_get_ScaleFactor(This,piValue)	\
    ( (This)->lpVtbl -> get_ScaleFactor(This,piValue) ) 

#define _ICounterItemUnion_get_Path(This,pstrValue)	\
    ( (This)->lpVtbl -> get_Path(This,pstrValue) ) 

#define _ICounterItemUnion_GetValue(This,Value,Status)	\
    ( (This)->lpVtbl -> GetValue(This,Value,Status) ) 

#define _ICounterItemUnion_GetStatistics(This,Max,Min,Avg,Status)	\
    ( (This)->lpVtbl -> GetStatistics(This,Max,Min,Avg,Status) ) 

#define _ICounterItemUnion_put_Selected(This,bState)	\
    ( (This)->lpVtbl -> put_Selected(This,bState) ) 

#define _ICounterItemUnion_get_Selected(This,pbState)	\
    ( (This)->lpVtbl -> get_Selected(This,pbState) ) 

#define _ICounterItemUnion_put_Visible(This,bState)	\
    ( (This)->lpVtbl -> put_Visible(This,bState) ) 

#define _ICounterItemUnion_get_Visible(This,pbState)	\
    ( (This)->lpVtbl -> get_Visible(This,pbState) ) 

#define _ICounterItemUnion_GetDataAt(This,iIndex,iWhich,pVariant)	\
    ( (This)->lpVtbl -> GetDataAt(This,iIndex,iWhich,pVariant) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* ___ICounterItemUnion_INTERFACE_DEFINED__ */


#ifndef __DICounterItem_DISPINTERFACE_DEFINED__
#define __DICounterItem_DISPINTERFACE_DEFINED__

/* dispinterface DICounterItem */
/* [helpstring][hidden][uuid] */ 


DEFINE_GUID(DIID_DICounterItem,0xC08C4FF2,0x0E2E,0x11cf,0x94,0x2C,0x00,0x80,0x29,0x00,0x43,0x47);

#if defined(__cplusplus) && !defined(CINTERFACE)

    MIDL_INTERFACE("C08C4FF2-0E2E-11cf-942C-008029004347")
    DICounterItem : public IDispatch
    {
    };
    
#else 	/* C style interface */

    typedef struct DICounterItemVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in DICounterItem * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in DICounterItem * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in DICounterItem * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in DICounterItem * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in DICounterItem * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in DICounterItem * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            DICounterItem * This,
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
    } DICounterItemVtbl;

    interface DICounterItem
    {
        CONST_VTBL struct DICounterItemVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define DICounterItem_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define DICounterItem_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define DICounterItem_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define DICounterItem_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define DICounterItem_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define DICounterItem_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define DICounterItem_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */


#endif 	/* __DICounterItem_DISPINTERFACE_DEFINED__ */


#ifndef __ICounters_INTERFACE_DEFINED__
#define __ICounters_INTERFACE_DEFINED__

/* interface ICounters */
/* [object][hidden][dual][helpstring][uuid] */ 


DEFINE_GUID(IID_ICounters,0x79167962,0x28FC,0x11cf,0x94,0x2F,0x00,0x80,0x29,0x00,0x43,0x47);

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("79167962-28FC-11cf-942F-008029004347")
    ICounters : public IDispatch
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *pLong) = 0;
        
        virtual /* [id][restricted][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppIunk) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT index,
            /* [retval][out] */ __RPC__deref_out_opt DICounterItem	**ppI) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Add( 
            /* [in] */ __RPC__in BSTR pathname,
            /* [retval][out] */ __RPC__deref_out_opt DICounterItem	**ppI) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Remove( 
            /* [in] */ VARIANT index) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICountersVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICounters * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICounters * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICounters * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ICounters * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ICounters * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ICounters * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ICounters * This,
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
        
        DECLSPEC_XFGVIRT(ICounters, get_Count)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in ICounters * This,
            /* [retval][out] */ __RPC__out long *pLong);
        
        DECLSPEC_XFGVIRT(ICounters, get__NewEnum)
        /* [id][restricted][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in ICounters * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppIunk);
        
        DECLSPEC_XFGVIRT(ICounters, get_Item)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in ICounters * This,
            /* [in] */ VARIANT index,
            /* [retval][out] */ __RPC__deref_out_opt DICounterItem	**ppI);
        
        DECLSPEC_XFGVIRT(ICounters, Add)
        HRESULT ( STDMETHODCALLTYPE *Add )( 
            __RPC__in ICounters * This,
            /* [in] */ __RPC__in BSTR pathname,
            /* [retval][out] */ __RPC__deref_out_opt DICounterItem	**ppI);
        
        DECLSPEC_XFGVIRT(ICounters, Remove)
        HRESULT ( STDMETHODCALLTYPE *Remove )( 
            __RPC__in ICounters * This,
            /* [in] */ VARIANT index);
        
        END_INTERFACE
    } ICountersVtbl;

    interface ICounters
    {
        CONST_VTBL struct ICountersVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICounters_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICounters_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICounters_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICounters_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ICounters_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ICounters_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ICounters_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ICounters_get_Count(This,pLong)	\
    ( (This)->lpVtbl -> get_Count(This,pLong) ) 

#define ICounters_get__NewEnum(This,ppIunk)	\
    ( (This)->lpVtbl -> get__NewEnum(This,ppIunk) ) 

#define ICounters_get_Item(This,index,ppI)	\
    ( (This)->lpVtbl -> get_Item(This,index,ppI) ) 

#define ICounters_Add(This,pathname,ppI)	\
    ( (This)->lpVtbl -> Add(This,pathname,ppI) ) 

#define ICounters_Remove(This,index)	\
    ( (This)->lpVtbl -> Remove(This,index) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICounters_INTERFACE_DEFINED__ */


#ifndef __ILogFileItem_INTERFACE_DEFINED__
#define __ILogFileItem_INTERFACE_DEFINED__

/* interface ILogFileItem */
/* [object][hidden][helpstring][uuid] */ 


DEFINE_GUID(IID_ILogFileItem,0xD6B518DD,0x05C7,0x418a,0x89,0xE6,0x4F,0x9C,0xE8,0xC6,0x84,0x1E);

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D6B518DD-05C7-418a-89E6-4F9CE8C6841E")
    ILogFileItem : public IUnknown
    {
    public:
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Path( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrValue) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ILogFileItemVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ILogFileItem * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ILogFileItem * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ILogFileItem * This);
        
        DECLSPEC_XFGVIRT(ILogFileItem, get_Path)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Path )( 
            __RPC__in ILogFileItem * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrValue);
        
        END_INTERFACE
    } ILogFileItemVtbl;

    interface ILogFileItem
    {
        CONST_VTBL struct ILogFileItemVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ILogFileItem_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ILogFileItem_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ILogFileItem_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ILogFileItem_get_Path(This,pstrValue)	\
    ( (This)->lpVtbl -> get_Path(This,pstrValue) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ILogFileItem_INTERFACE_DEFINED__ */


#ifndef __DILogFileItem_DISPINTERFACE_DEFINED__
#define __DILogFileItem_DISPINTERFACE_DEFINED__

/* dispinterface DILogFileItem */
/* [helpstring][hidden][uuid] */ 


DEFINE_GUID(DIID_DILogFileItem,0x8D093FFC,0xF777,0x4917,0x82,0xD1,0x83,0x3F,0xBC,0x54,0xC5,0x8F);

#if defined(__cplusplus) && !defined(CINTERFACE)

    MIDL_INTERFACE("8D093FFC-F777-4917-82D1-833FBC54C58F")
    DILogFileItem : public IDispatch
    {
    };
    
#else 	/* C style interface */

    typedef struct DILogFileItemVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in DILogFileItem * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in DILogFileItem * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in DILogFileItem * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in DILogFileItem * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in DILogFileItem * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in DILogFileItem * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            DILogFileItem * This,
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
    } DILogFileItemVtbl;

    interface DILogFileItem
    {
        CONST_VTBL struct DILogFileItemVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define DILogFileItem_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define DILogFileItem_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define DILogFileItem_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define DILogFileItem_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define DILogFileItem_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define DILogFileItem_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define DILogFileItem_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */


#endif 	/* __DILogFileItem_DISPINTERFACE_DEFINED__ */


#ifndef __ILogFiles_INTERFACE_DEFINED__
#define __ILogFiles_INTERFACE_DEFINED__

/* interface ILogFiles */
/* [object][hidden][dual][helpstring][uuid] */ 


DEFINE_GUID(IID_ILogFiles,0x6A2A97E6,0x6851,0x41ea,0x87,0xAD,0x2A,0x82,0x25,0x33,0x58,0x65);

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6A2A97E6-6851-41ea-87AD-2A8225335865")
    ILogFiles : public IDispatch
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *pLong) = 0;
        
        virtual /* [id][restricted][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppIunk) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT index,
            /* [retval][out] */ __RPC__deref_out_opt DILogFileItem	**ppI) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Add( 
            /* [in] */ __RPC__in BSTR pathname,
            /* [retval][out] */ __RPC__deref_out_opt DILogFileItem	**ppI) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Remove( 
            /* [in] */ VARIANT index) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ILogFilesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ILogFiles * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ILogFiles * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ILogFiles * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ILogFiles * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ILogFiles * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ILogFiles * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ILogFiles * This,
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
        
        DECLSPEC_XFGVIRT(ILogFiles, get_Count)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in ILogFiles * This,
            /* [retval][out] */ __RPC__out long *pLong);
        
        DECLSPEC_XFGVIRT(ILogFiles, get__NewEnum)
        /* [id][restricted][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in ILogFiles * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppIunk);
        
        DECLSPEC_XFGVIRT(ILogFiles, get_Item)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in ILogFiles * This,
            /* [in] */ VARIANT index,
            /* [retval][out] */ __RPC__deref_out_opt DILogFileItem	**ppI);
        
        DECLSPEC_XFGVIRT(ILogFiles, Add)
        HRESULT ( STDMETHODCALLTYPE *Add )( 
            __RPC__in ILogFiles * This,
            /* [in] */ __RPC__in BSTR pathname,
            /* [retval][out] */ __RPC__deref_out_opt DILogFileItem	**ppI);
        
        DECLSPEC_XFGVIRT(ILogFiles, Remove)
        HRESULT ( STDMETHODCALLTYPE *Remove )( 
            __RPC__in ILogFiles * This,
            /* [in] */ VARIANT index);
        
        END_INTERFACE
    } ILogFilesVtbl;

    interface ILogFiles
    {
        CONST_VTBL struct ILogFilesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ILogFiles_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ILogFiles_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ILogFiles_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ILogFiles_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ILogFiles_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ILogFiles_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ILogFiles_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ILogFiles_get_Count(This,pLong)	\
    ( (This)->lpVtbl -> get_Count(This,pLong) ) 

#define ILogFiles_get__NewEnum(This,ppIunk)	\
    ( (This)->lpVtbl -> get__NewEnum(This,ppIunk) ) 

#define ILogFiles_get_Item(This,index,ppI)	\
    ( (This)->lpVtbl -> get_Item(This,index,ppI) ) 

#define ILogFiles_Add(This,pathname,ppI)	\
    ( (This)->lpVtbl -> Add(This,pathname,ppI) ) 

#define ILogFiles_Remove(This,index)	\
    ( (This)->lpVtbl -> Remove(This,index) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ILogFiles_INTERFACE_DEFINED__ */


#ifndef __ISystemMonitor_INTERFACE_DEFINED__
#define __ISystemMonitor_INTERFACE_DEFINED__

/* interface ISystemMonitor */
/* [object][hidden][helpstring][uuid] */ 


DEFINE_GUID(IID_ISystemMonitor,0x194EB241,0xC32C,0x11cf,0x93,0x98,0x00,0xAA,0x00,0xA3,0xDD,0xEA);

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("194EB241-C32C-11cf-9398-00AA00A3DDEA")
    ISystemMonitor : public IUnknown
    {
    public:
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Appearance( 
            /* [retval][out] */ __RPC__out INT *iAppearance) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE put_Appearance( 
            /* [in] */ INT iAppearance) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_BackColor( 
            /* [retval][out] */ __RPC__out OLE_COLOR *pColor) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE put_BackColor( 
            /* [in] */ OLE_COLOR Color) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_BorderStyle( 
            /* [retval][out] */ __RPC__out INT *iBorderStyle) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE put_BorderStyle( 
            /* [in] */ INT iBorderStyle) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_ForeColor( 
            /* [retval][out] */ __RPC__out OLE_COLOR *pColor) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE put_ForeColor( 
            /* [in] */ OLE_COLOR Color) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Font( 
            /* [retval][out] */ __RPC__deref_out_opt IFontDisp **ppFont) = 0;
        
        virtual /* [propputref][id] */ HRESULT STDMETHODCALLTYPE putref_Font( 
            /* [in] */ __RPC__in_opt IFontDisp *pFont) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Counters( 
            /* [retval][out] */ __RPC__deref_out_opt ICounters **ppICounters) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_ShowVerticalGrid( 
            /* [in] */ VARIANT_BOOL bState) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_ShowVerticalGrid( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbState) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_ShowHorizontalGrid( 
            /* [in] */ VARIANT_BOOL bState) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_ShowHorizontalGrid( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbState) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_ShowLegend( 
            /* [in] */ VARIANT_BOOL bState) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_ShowLegend( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbState) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_ShowScaleLabels( 
            /* [in] */ VARIANT_BOOL bState) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_ShowScaleLabels( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbState) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_ShowValueBar( 
            /* [in] */ VARIANT_BOOL bState) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_ShowValueBar( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbState) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_MaximumScale( 
            /* [in] */ INT iValue) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_MaximumScale( 
            /* [retval][out] */ __RPC__out INT *piValue) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_MinimumScale( 
            /* [in] */ INT iValue) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_MinimumScale( 
            /* [retval][out] */ __RPC__out INT *piValue) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_UpdateInterval( 
            /* [in] */ FLOAT fValue) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_UpdateInterval( 
            /* [retval][out] */ __RPC__out FLOAT *pfValue) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_DisplayType( 
            /* [in] */ DisplayTypeConstants eDisplayType) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_DisplayType( 
            /* [retval][out] */ __RPC__out DisplayTypeConstants *peDisplayType) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_ManualUpdate( 
            /* [in] */ VARIANT_BOOL bState) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_ManualUpdate( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbState) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_GraphTitle( 
            /* [in] */ __RPC__in BSTR bsTitle) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_GraphTitle( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbsTitle) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_YAxisLabel( 
            /* [in] */ __RPC__in BSTR bsTitle) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_YAxisLabel( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbsTitle) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CollectSample( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE UpdateGraph( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE BrowseCounters( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE DisplayProperties( void) = 0;
        
        virtual /* [hidden][id] */ HRESULT STDMETHODCALLTYPE Counter( 
            /* [in] */ INT iIndex,
            /* [out] */ __RPC__deref_out_opt ICounterItem **ppICounter) = 0;
        
        virtual /* [hidden][id] */ HRESULT STDMETHODCALLTYPE AddCounter( 
            /* [in] */ __RPC__in BSTR bsPath,
            /* [out] */ __RPC__deref_out_opt ICounterItem **ppICounter) = 0;
        
        virtual /* [hidden][id] */ HRESULT STDMETHODCALLTYPE DeleteCounter( 
            /* [in] */ __RPC__in_opt ICounterItem *pCtr) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_BackColorCtl( 
            /* [retval][out] */ __RPC__out OLE_COLOR *pColor) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE put_BackColorCtl( 
            /* [in] */ OLE_COLOR Color) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_LogFileName( 
            /* [in] */ __RPC__in BSTR bsFileName) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_LogFileName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *bsFileName) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_LogViewStart( 
            /* [in] */ DATE StartTime) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_LogViewStart( 
            /* [retval][out] */ __RPC__out DATE *StartTime) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_LogViewStop( 
            /* [in] */ DATE StopTime) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_LogViewStop( 
            /* [retval][out] */ __RPC__out DATE *StopTime) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_GridColor( 
            /* [retval][out] */ __RPC__out OLE_COLOR *pColor) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE put_GridColor( 
            /* [in] */ OLE_COLOR Color) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_TimeBarColor( 
            /* [retval][out] */ __RPC__out OLE_COLOR *pColor) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE put_TimeBarColor( 
            /* [in] */ OLE_COLOR Color) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Highlight( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbState) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE put_Highlight( 
            /* [in] */ VARIANT_BOOL bState) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_ShowToolbar( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbState) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE put_ShowToolbar( 
            /* [in] */ VARIANT_BOOL bState) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Paste( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Copy( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_ReadOnly( 
            /* [in] */ VARIANT_BOOL bState) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_ReadOnly( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbState) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_ReportValueType( 
            /* [in] */ ReportValueTypeConstants eReportValueType) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_ReportValueType( 
            /* [retval][out] */ __RPC__out ReportValueTypeConstants *peReportValueType) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_MonitorDuplicateInstances( 
            /* [in] */ VARIANT_BOOL bState) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_MonitorDuplicateInstances( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbState) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_DisplayFilter( 
            /* [in] */ INT iValue) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_DisplayFilter( 
            /* [retval][out] */ __RPC__out INT *piValue) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_LogFiles( 
            /* [retval][out] */ __RPC__deref_out_opt ILogFiles **ppILogFiles) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_DataSourceType( 
            /* [in] */ DataSourceTypeConstants eDataSourceType) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_DataSourceType( 
            /* [retval][out] */ __RPC__out DataSourceTypeConstants *peDataSourceType) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_SqlDsnName( 
            /* [in] */ __RPC__in BSTR bsSqlDsnName) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_SqlDsnName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *bsSqlDsnName) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_SqlLogSetName( 
            /* [in] */ __RPC__in BSTR bsSqlLogSetName) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_SqlLogSetName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *bsSqlLogSetName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISystemMonitorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISystemMonitor * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISystemMonitor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISystemMonitor * This);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, get_Appearance)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Appearance )( 
            __RPC__in ISystemMonitor * This,
            /* [retval][out] */ __RPC__out INT *iAppearance);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, put_Appearance)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Appearance )( 
            __RPC__in ISystemMonitor * This,
            /* [in] */ INT iAppearance);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, get_BackColor)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_BackColor )( 
            __RPC__in ISystemMonitor * This,
            /* [retval][out] */ __RPC__out OLE_COLOR *pColor);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, put_BackColor)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_BackColor )( 
            __RPC__in ISystemMonitor * This,
            /* [in] */ OLE_COLOR Color);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, get_BorderStyle)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_BorderStyle )( 
            __RPC__in ISystemMonitor * This,
            /* [retval][out] */ __RPC__out INT *iBorderStyle);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, put_BorderStyle)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_BorderStyle )( 
            __RPC__in ISystemMonitor * This,
            /* [in] */ INT iBorderStyle);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, get_ForeColor)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ForeColor )( 
            __RPC__in ISystemMonitor * This,
            /* [retval][out] */ __RPC__out OLE_COLOR *pColor);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, put_ForeColor)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ForeColor )( 
            __RPC__in ISystemMonitor * This,
            /* [in] */ OLE_COLOR Color);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, get_Font)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Font )( 
            __RPC__in ISystemMonitor * This,
            /* [retval][out] */ __RPC__deref_out_opt IFontDisp **ppFont);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, putref_Font)
        /* [propputref][id] */ HRESULT ( STDMETHODCALLTYPE *putref_Font )( 
            __RPC__in ISystemMonitor * This,
            /* [in] */ __RPC__in_opt IFontDisp *pFont);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, get_Counters)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Counters )( 
            __RPC__in ISystemMonitor * This,
            /* [retval][out] */ __RPC__deref_out_opt ICounters **ppICounters);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, put_ShowVerticalGrid)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ShowVerticalGrid )( 
            __RPC__in ISystemMonitor * This,
            /* [in] */ VARIANT_BOOL bState);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, get_ShowVerticalGrid)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ShowVerticalGrid )( 
            __RPC__in ISystemMonitor * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbState);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, put_ShowHorizontalGrid)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ShowHorizontalGrid )( 
            __RPC__in ISystemMonitor * This,
            /* [in] */ VARIANT_BOOL bState);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, get_ShowHorizontalGrid)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ShowHorizontalGrid )( 
            __RPC__in ISystemMonitor * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbState);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, put_ShowLegend)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ShowLegend )( 
            __RPC__in ISystemMonitor * This,
            /* [in] */ VARIANT_BOOL bState);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, get_ShowLegend)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ShowLegend )( 
            __RPC__in ISystemMonitor * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbState);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, put_ShowScaleLabels)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ShowScaleLabels )( 
            __RPC__in ISystemMonitor * This,
            /* [in] */ VARIANT_BOOL bState);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, get_ShowScaleLabels)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ShowScaleLabels )( 
            __RPC__in ISystemMonitor * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbState);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, put_ShowValueBar)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ShowValueBar )( 
            __RPC__in ISystemMonitor * This,
            /* [in] */ VARIANT_BOOL bState);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, get_ShowValueBar)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ShowValueBar )( 
            __RPC__in ISystemMonitor * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbState);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, put_MaximumScale)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_MaximumScale )( 
            __RPC__in ISystemMonitor * This,
            /* [in] */ INT iValue);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, get_MaximumScale)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_MaximumScale )( 
            __RPC__in ISystemMonitor * This,
            /* [retval][out] */ __RPC__out INT *piValue);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, put_MinimumScale)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_MinimumScale )( 
            __RPC__in ISystemMonitor * This,
            /* [in] */ INT iValue);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, get_MinimumScale)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_MinimumScale )( 
            __RPC__in ISystemMonitor * This,
            /* [retval][out] */ __RPC__out INT *piValue);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, put_UpdateInterval)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_UpdateInterval )( 
            __RPC__in ISystemMonitor * This,
            /* [in] */ FLOAT fValue);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, get_UpdateInterval)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_UpdateInterval )( 
            __RPC__in ISystemMonitor * This,
            /* [retval][out] */ __RPC__out FLOAT *pfValue);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, put_DisplayType)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_DisplayType )( 
            __RPC__in ISystemMonitor * This,
            /* [in] */ DisplayTypeConstants eDisplayType);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, get_DisplayType)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_DisplayType )( 
            __RPC__in ISystemMonitor * This,
            /* [retval][out] */ __RPC__out DisplayTypeConstants *peDisplayType);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, put_ManualUpdate)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ManualUpdate )( 
            __RPC__in ISystemMonitor * This,
            /* [in] */ VARIANT_BOOL bState);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, get_ManualUpdate)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ManualUpdate )( 
            __RPC__in ISystemMonitor * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbState);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, put_GraphTitle)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_GraphTitle )( 
            __RPC__in ISystemMonitor * This,
            /* [in] */ __RPC__in BSTR bsTitle);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, get_GraphTitle)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_GraphTitle )( 
            __RPC__in ISystemMonitor * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbsTitle);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, put_YAxisLabel)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_YAxisLabel )( 
            __RPC__in ISystemMonitor * This,
            /* [in] */ __RPC__in BSTR bsTitle);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, get_YAxisLabel)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_YAxisLabel )( 
            __RPC__in ISystemMonitor * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbsTitle);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, CollectSample)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CollectSample )( 
            __RPC__in ISystemMonitor * This);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, UpdateGraph)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *UpdateGraph )( 
            __RPC__in ISystemMonitor * This);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, BrowseCounters)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *BrowseCounters )( 
            __RPC__in ISystemMonitor * This);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, DisplayProperties)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *DisplayProperties )( 
            __RPC__in ISystemMonitor * This);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, Counter)
        /* [hidden][id] */ HRESULT ( STDMETHODCALLTYPE *Counter )( 
            __RPC__in ISystemMonitor * This,
            /* [in] */ INT iIndex,
            /* [out] */ __RPC__deref_out_opt ICounterItem **ppICounter);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, AddCounter)
        /* [hidden][id] */ HRESULT ( STDMETHODCALLTYPE *AddCounter )( 
            __RPC__in ISystemMonitor * This,
            /* [in] */ __RPC__in BSTR bsPath,
            /* [out] */ __RPC__deref_out_opt ICounterItem **ppICounter);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, DeleteCounter)
        /* [hidden][id] */ HRESULT ( STDMETHODCALLTYPE *DeleteCounter )( 
            __RPC__in ISystemMonitor * This,
            /* [in] */ __RPC__in_opt ICounterItem *pCtr);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, get_BackColorCtl)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_BackColorCtl )( 
            __RPC__in ISystemMonitor * This,
            /* [retval][out] */ __RPC__out OLE_COLOR *pColor);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, put_BackColorCtl)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_BackColorCtl )( 
            __RPC__in ISystemMonitor * This,
            /* [in] */ OLE_COLOR Color);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, put_LogFileName)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_LogFileName )( 
            __RPC__in ISystemMonitor * This,
            /* [in] */ __RPC__in BSTR bsFileName);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, get_LogFileName)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_LogFileName )( 
            __RPC__in ISystemMonitor * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *bsFileName);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, put_LogViewStart)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_LogViewStart )( 
            __RPC__in ISystemMonitor * This,
            /* [in] */ DATE StartTime);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, get_LogViewStart)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_LogViewStart )( 
            __RPC__in ISystemMonitor * This,
            /* [retval][out] */ __RPC__out DATE *StartTime);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, put_LogViewStop)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_LogViewStop )( 
            __RPC__in ISystemMonitor * This,
            /* [in] */ DATE StopTime);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, get_LogViewStop)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_LogViewStop )( 
            __RPC__in ISystemMonitor * This,
            /* [retval][out] */ __RPC__out DATE *StopTime);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, get_GridColor)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_GridColor )( 
            __RPC__in ISystemMonitor * This,
            /* [retval][out] */ __RPC__out OLE_COLOR *pColor);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, put_GridColor)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_GridColor )( 
            __RPC__in ISystemMonitor * This,
            /* [in] */ OLE_COLOR Color);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, get_TimeBarColor)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_TimeBarColor )( 
            __RPC__in ISystemMonitor * This,
            /* [retval][out] */ __RPC__out OLE_COLOR *pColor);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, put_TimeBarColor)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_TimeBarColor )( 
            __RPC__in ISystemMonitor * This,
            /* [in] */ OLE_COLOR Color);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, get_Highlight)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Highlight )( 
            __RPC__in ISystemMonitor * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbState);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, put_Highlight)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Highlight )( 
            __RPC__in ISystemMonitor * This,
            /* [in] */ VARIANT_BOOL bState);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, get_ShowToolbar)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ShowToolbar )( 
            __RPC__in ISystemMonitor * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbState);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, put_ShowToolbar)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ShowToolbar )( 
            __RPC__in ISystemMonitor * This,
            /* [in] */ VARIANT_BOOL bState);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, Paste)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Paste )( 
            __RPC__in ISystemMonitor * This);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, Copy)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Copy )( 
            __RPC__in ISystemMonitor * This);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, Reset)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in ISystemMonitor * This);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, put_ReadOnly)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ReadOnly )( 
            __RPC__in ISystemMonitor * This,
            /* [in] */ VARIANT_BOOL bState);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, get_ReadOnly)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ReadOnly )( 
            __RPC__in ISystemMonitor * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbState);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, put_ReportValueType)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ReportValueType )( 
            __RPC__in ISystemMonitor * This,
            /* [in] */ ReportValueTypeConstants eReportValueType);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, get_ReportValueType)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ReportValueType )( 
            __RPC__in ISystemMonitor * This,
            /* [retval][out] */ __RPC__out ReportValueTypeConstants *peReportValueType);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, put_MonitorDuplicateInstances)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_MonitorDuplicateInstances )( 
            __RPC__in ISystemMonitor * This,
            /* [in] */ VARIANT_BOOL bState);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, get_MonitorDuplicateInstances)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_MonitorDuplicateInstances )( 
            __RPC__in ISystemMonitor * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbState);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, put_DisplayFilter)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_DisplayFilter )( 
            __RPC__in ISystemMonitor * This,
            /* [in] */ INT iValue);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, get_DisplayFilter)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_DisplayFilter )( 
            __RPC__in ISystemMonitor * This,
            /* [retval][out] */ __RPC__out INT *piValue);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, get_LogFiles)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_LogFiles )( 
            __RPC__in ISystemMonitor * This,
            /* [retval][out] */ __RPC__deref_out_opt ILogFiles **ppILogFiles);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, put_DataSourceType)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_DataSourceType )( 
            __RPC__in ISystemMonitor * This,
            /* [in] */ DataSourceTypeConstants eDataSourceType);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, get_DataSourceType)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_DataSourceType )( 
            __RPC__in ISystemMonitor * This,
            /* [retval][out] */ __RPC__out DataSourceTypeConstants *peDataSourceType);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, put_SqlDsnName)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_SqlDsnName )( 
            __RPC__in ISystemMonitor * This,
            /* [in] */ __RPC__in BSTR bsSqlDsnName);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, get_SqlDsnName)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_SqlDsnName )( 
            __RPC__in ISystemMonitor * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *bsSqlDsnName);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, put_SqlLogSetName)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_SqlLogSetName )( 
            __RPC__in ISystemMonitor * This,
            /* [in] */ __RPC__in BSTR bsSqlLogSetName);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, get_SqlLogSetName)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_SqlLogSetName )( 
            __RPC__in ISystemMonitor * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *bsSqlLogSetName);
        
        END_INTERFACE
    } ISystemMonitorVtbl;

    interface ISystemMonitor
    {
        CONST_VTBL struct ISystemMonitorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISystemMonitor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISystemMonitor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISystemMonitor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISystemMonitor_get_Appearance(This,iAppearance)	\
    ( (This)->lpVtbl -> get_Appearance(This,iAppearance) ) 

#define ISystemMonitor_put_Appearance(This,iAppearance)	\
    ( (This)->lpVtbl -> put_Appearance(This,iAppearance) ) 

#define ISystemMonitor_get_BackColor(This,pColor)	\
    ( (This)->lpVtbl -> get_BackColor(This,pColor) ) 

#define ISystemMonitor_put_BackColor(This,Color)	\
    ( (This)->lpVtbl -> put_BackColor(This,Color) ) 

#define ISystemMonitor_get_BorderStyle(This,iBorderStyle)	\
    ( (This)->lpVtbl -> get_BorderStyle(This,iBorderStyle) ) 

#define ISystemMonitor_put_BorderStyle(This,iBorderStyle)	\
    ( (This)->lpVtbl -> put_BorderStyle(This,iBorderStyle) ) 

#define ISystemMonitor_get_ForeColor(This,pColor)	\
    ( (This)->lpVtbl -> get_ForeColor(This,pColor) ) 

#define ISystemMonitor_put_ForeColor(This,Color)	\
    ( (This)->lpVtbl -> put_ForeColor(This,Color) ) 

#define ISystemMonitor_get_Font(This,ppFont)	\
    ( (This)->lpVtbl -> get_Font(This,ppFont) ) 

#define ISystemMonitor_putref_Font(This,pFont)	\
    ( (This)->lpVtbl -> putref_Font(This,pFont) ) 

#define ISystemMonitor_get_Counters(This,ppICounters)	\
    ( (This)->lpVtbl -> get_Counters(This,ppICounters) ) 

#define ISystemMonitor_put_ShowVerticalGrid(This,bState)	\
    ( (This)->lpVtbl -> put_ShowVerticalGrid(This,bState) ) 

#define ISystemMonitor_get_ShowVerticalGrid(This,pbState)	\
    ( (This)->lpVtbl -> get_ShowVerticalGrid(This,pbState) ) 

#define ISystemMonitor_put_ShowHorizontalGrid(This,bState)	\
    ( (This)->lpVtbl -> put_ShowHorizontalGrid(This,bState) ) 

#define ISystemMonitor_get_ShowHorizontalGrid(This,pbState)	\
    ( (This)->lpVtbl -> get_ShowHorizontalGrid(This,pbState) ) 

#define ISystemMonitor_put_ShowLegend(This,bState)	\
    ( (This)->lpVtbl -> put_ShowLegend(This,bState) ) 

#define ISystemMonitor_get_ShowLegend(This,pbState)	\
    ( (This)->lpVtbl -> get_ShowLegend(This,pbState) ) 

#define ISystemMonitor_put_ShowScaleLabels(This,bState)	\
    ( (This)->lpVtbl -> put_ShowScaleLabels(This,bState) ) 

#define ISystemMonitor_get_ShowScaleLabels(This,pbState)	\
    ( (This)->lpVtbl -> get_ShowScaleLabels(This,pbState) ) 

#define ISystemMonitor_put_ShowValueBar(This,bState)	\
    ( (This)->lpVtbl -> put_ShowValueBar(This,bState) ) 

#define ISystemMonitor_get_ShowValueBar(This,pbState)	\
    ( (This)->lpVtbl -> get_ShowValueBar(This,pbState) ) 

#define ISystemMonitor_put_MaximumScale(This,iValue)	\
    ( (This)->lpVtbl -> put_MaximumScale(This,iValue) ) 

#define ISystemMonitor_get_MaximumScale(This,piValue)	\
    ( (This)->lpVtbl -> get_MaximumScale(This,piValue) ) 

#define ISystemMonitor_put_MinimumScale(This,iValue)	\
    ( (This)->lpVtbl -> put_MinimumScale(This,iValue) ) 

#define ISystemMonitor_get_MinimumScale(This,piValue)	\
    ( (This)->lpVtbl -> get_MinimumScale(This,piValue) ) 

#define ISystemMonitor_put_UpdateInterval(This,fValue)	\
    ( (This)->lpVtbl -> put_UpdateInterval(This,fValue) ) 

#define ISystemMonitor_get_UpdateInterval(This,pfValue)	\
    ( (This)->lpVtbl -> get_UpdateInterval(This,pfValue) ) 

#define ISystemMonitor_put_DisplayType(This,eDisplayType)	\
    ( (This)->lpVtbl -> put_DisplayType(This,eDisplayType) ) 

#define ISystemMonitor_get_DisplayType(This,peDisplayType)	\
    ( (This)->lpVtbl -> get_DisplayType(This,peDisplayType) ) 

#define ISystemMonitor_put_ManualUpdate(This,bState)	\
    ( (This)->lpVtbl -> put_ManualUpdate(This,bState) ) 

#define ISystemMonitor_get_ManualUpdate(This,pbState)	\
    ( (This)->lpVtbl -> get_ManualUpdate(This,pbState) ) 

#define ISystemMonitor_put_GraphTitle(This,bsTitle)	\
    ( (This)->lpVtbl -> put_GraphTitle(This,bsTitle) ) 

#define ISystemMonitor_get_GraphTitle(This,pbsTitle)	\
    ( (This)->lpVtbl -> get_GraphTitle(This,pbsTitle) ) 

#define ISystemMonitor_put_YAxisLabel(This,bsTitle)	\
    ( (This)->lpVtbl -> put_YAxisLabel(This,bsTitle) ) 

#define ISystemMonitor_get_YAxisLabel(This,pbsTitle)	\
    ( (This)->lpVtbl -> get_YAxisLabel(This,pbsTitle) ) 

#define ISystemMonitor_CollectSample(This)	\
    ( (This)->lpVtbl -> CollectSample(This) ) 

#define ISystemMonitor_UpdateGraph(This)	\
    ( (This)->lpVtbl -> UpdateGraph(This) ) 

#define ISystemMonitor_BrowseCounters(This)	\
    ( (This)->lpVtbl -> BrowseCounters(This) ) 

#define ISystemMonitor_DisplayProperties(This)	\
    ( (This)->lpVtbl -> DisplayProperties(This) ) 

#define ISystemMonitor_Counter(This,iIndex,ppICounter)	\
    ( (This)->lpVtbl -> Counter(This,iIndex,ppICounter) ) 

#define ISystemMonitor_AddCounter(This,bsPath,ppICounter)	\
    ( (This)->lpVtbl -> AddCounter(This,bsPath,ppICounter) ) 

#define ISystemMonitor_DeleteCounter(This,pCtr)	\
    ( (This)->lpVtbl -> DeleteCounter(This,pCtr) ) 

#define ISystemMonitor_get_BackColorCtl(This,pColor)	\
    ( (This)->lpVtbl -> get_BackColorCtl(This,pColor) ) 

#define ISystemMonitor_put_BackColorCtl(This,Color)	\
    ( (This)->lpVtbl -> put_BackColorCtl(This,Color) ) 

#define ISystemMonitor_put_LogFileName(This,bsFileName)	\
    ( (This)->lpVtbl -> put_LogFileName(This,bsFileName) ) 

#define ISystemMonitor_get_LogFileName(This,bsFileName)	\
    ( (This)->lpVtbl -> get_LogFileName(This,bsFileName) ) 

#define ISystemMonitor_put_LogViewStart(This,StartTime)	\
    ( (This)->lpVtbl -> put_LogViewStart(This,StartTime) ) 

#define ISystemMonitor_get_LogViewStart(This,StartTime)	\
    ( (This)->lpVtbl -> get_LogViewStart(This,StartTime) ) 

#define ISystemMonitor_put_LogViewStop(This,StopTime)	\
    ( (This)->lpVtbl -> put_LogViewStop(This,StopTime) ) 

#define ISystemMonitor_get_LogViewStop(This,StopTime)	\
    ( (This)->lpVtbl -> get_LogViewStop(This,StopTime) ) 

#define ISystemMonitor_get_GridColor(This,pColor)	\
    ( (This)->lpVtbl -> get_GridColor(This,pColor) ) 

#define ISystemMonitor_put_GridColor(This,Color)	\
    ( (This)->lpVtbl -> put_GridColor(This,Color) ) 

#define ISystemMonitor_get_TimeBarColor(This,pColor)	\
    ( (This)->lpVtbl -> get_TimeBarColor(This,pColor) ) 

#define ISystemMonitor_put_TimeBarColor(This,Color)	\
    ( (This)->lpVtbl -> put_TimeBarColor(This,Color) ) 

#define ISystemMonitor_get_Highlight(This,pbState)	\
    ( (This)->lpVtbl -> get_Highlight(This,pbState) ) 

#define ISystemMonitor_put_Highlight(This,bState)	\
    ( (This)->lpVtbl -> put_Highlight(This,bState) ) 

#define ISystemMonitor_get_ShowToolbar(This,pbState)	\
    ( (This)->lpVtbl -> get_ShowToolbar(This,pbState) ) 

#define ISystemMonitor_put_ShowToolbar(This,bState)	\
    ( (This)->lpVtbl -> put_ShowToolbar(This,bState) ) 

#define ISystemMonitor_Paste(This)	\
    ( (This)->lpVtbl -> Paste(This) ) 

#define ISystemMonitor_Copy(This)	\
    ( (This)->lpVtbl -> Copy(This) ) 

#define ISystemMonitor_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define ISystemMonitor_put_ReadOnly(This,bState)	\
    ( (This)->lpVtbl -> put_ReadOnly(This,bState) ) 

#define ISystemMonitor_get_ReadOnly(This,pbState)	\
    ( (This)->lpVtbl -> get_ReadOnly(This,pbState) ) 

#define ISystemMonitor_put_ReportValueType(This,eReportValueType)	\
    ( (This)->lpVtbl -> put_ReportValueType(This,eReportValueType) ) 

#define ISystemMonitor_get_ReportValueType(This,peReportValueType)	\
    ( (This)->lpVtbl -> get_ReportValueType(This,peReportValueType) ) 

#define ISystemMonitor_put_MonitorDuplicateInstances(This,bState)	\
    ( (This)->lpVtbl -> put_MonitorDuplicateInstances(This,bState) ) 

#define ISystemMonitor_get_MonitorDuplicateInstances(This,pbState)	\
    ( (This)->lpVtbl -> get_MonitorDuplicateInstances(This,pbState) ) 

#define ISystemMonitor_put_DisplayFilter(This,iValue)	\
    ( (This)->lpVtbl -> put_DisplayFilter(This,iValue) ) 

#define ISystemMonitor_get_DisplayFilter(This,piValue)	\
    ( (This)->lpVtbl -> get_DisplayFilter(This,piValue) ) 

#define ISystemMonitor_get_LogFiles(This,ppILogFiles)	\
    ( (This)->lpVtbl -> get_LogFiles(This,ppILogFiles) ) 

#define ISystemMonitor_put_DataSourceType(This,eDataSourceType)	\
    ( (This)->lpVtbl -> put_DataSourceType(This,eDataSourceType) ) 

#define ISystemMonitor_get_DataSourceType(This,peDataSourceType)	\
    ( (This)->lpVtbl -> get_DataSourceType(This,peDataSourceType) ) 

#define ISystemMonitor_put_SqlDsnName(This,bsSqlDsnName)	\
    ( (This)->lpVtbl -> put_SqlDsnName(This,bsSqlDsnName) ) 

#define ISystemMonitor_get_SqlDsnName(This,bsSqlDsnName)	\
    ( (This)->lpVtbl -> get_SqlDsnName(This,bsSqlDsnName) ) 

#define ISystemMonitor_put_SqlLogSetName(This,bsSqlLogSetName)	\
    ( (This)->lpVtbl -> put_SqlLogSetName(This,bsSqlLogSetName) ) 

#define ISystemMonitor_get_SqlLogSetName(This,bsSqlLogSetName)	\
    ( (This)->lpVtbl -> get_SqlLogSetName(This,bsSqlLogSetName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISystemMonitor_INTERFACE_DEFINED__ */


#ifndef __ISystemMonitor2_INTERFACE_DEFINED__
#define __ISystemMonitor2_INTERFACE_DEFINED__

/* interface ISystemMonitor2 */
/* [object][helpstring][uuid] */ 


DEFINE_GUID(IID_ISystemMonitor2,0x08e3206a,0x5fd2,0x4fde,0xa8,0xa5,0x8c,0xb3,0xb6,0x3d,0x26,0x77);

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("08e3206a-5fd2-4fde-a8a5-8cb3b63d2677")
    ISystemMonitor2 : public ISystemMonitor
    {
    public:
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_EnableDigitGrouping( 
            /* [in] */ VARIANT_BOOL bState) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_EnableDigitGrouping( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbState) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_EnableToolTips( 
            /* [in] */ VARIANT_BOOL bState) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_EnableToolTips( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbState) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_ShowTimeAxisLabels( 
            /* [in] */ VARIANT_BOOL bState) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_ShowTimeAxisLabels( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbState) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_ChartScroll( 
            /* [in] */ VARIANT_BOOL bScroll) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_ChartScroll( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbScroll) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_DataPointCount( 
            /* [in] */ INT iNewCount) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_DataPointCount( 
            /* [retval][out] */ __RPC__out INT *piDataPointCount) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ScaleToFit( 
            VARIANT_BOOL bSelectedCountersOnly) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SaveAs( 
            __RPC__in BSTR bstrFileName,
            SysmonFileType eSysmonFileType) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Relog( 
            __RPC__in BSTR bstrFileName,
            SysmonFileType eSysmonFileType,
            INT iFilter) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ClearData( void) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_LogSourceStartTime( 
            /* [out] */ __RPC__out DATE *pDate) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_LogSourceStopTime( 
            /* [out] */ __RPC__out DATE *pDate) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetLogViewRange( 
            /* [in] */ DATE StartTime,
            /* [in] */ DATE StopTime) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetLogViewRange( 
            /* [out] */ __RPC__out DATE *StartTime,
            /* [out] */ __RPC__out DATE *StopTime) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE BatchingLock( 
            /* [in] */ VARIANT_BOOL fLock,
            /* [in] */ SysmonBatchReason eBatchReason) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE LoadSettings( 
            /* [in] */ __RPC__in BSTR bstrSettingFileName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISystemMonitor2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISystemMonitor2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISystemMonitor2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISystemMonitor2 * This);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, get_Appearance)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Appearance )( 
            __RPC__in ISystemMonitor2 * This,
            /* [retval][out] */ __RPC__out INT *iAppearance);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, put_Appearance)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Appearance )( 
            __RPC__in ISystemMonitor2 * This,
            /* [in] */ INT iAppearance);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, get_BackColor)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_BackColor )( 
            __RPC__in ISystemMonitor2 * This,
            /* [retval][out] */ __RPC__out OLE_COLOR *pColor);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, put_BackColor)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_BackColor )( 
            __RPC__in ISystemMonitor2 * This,
            /* [in] */ OLE_COLOR Color);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, get_BorderStyle)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_BorderStyle )( 
            __RPC__in ISystemMonitor2 * This,
            /* [retval][out] */ __RPC__out INT *iBorderStyle);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, put_BorderStyle)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_BorderStyle )( 
            __RPC__in ISystemMonitor2 * This,
            /* [in] */ INT iBorderStyle);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, get_ForeColor)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ForeColor )( 
            __RPC__in ISystemMonitor2 * This,
            /* [retval][out] */ __RPC__out OLE_COLOR *pColor);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, put_ForeColor)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ForeColor )( 
            __RPC__in ISystemMonitor2 * This,
            /* [in] */ OLE_COLOR Color);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, get_Font)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Font )( 
            __RPC__in ISystemMonitor2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IFontDisp **ppFont);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, putref_Font)
        /* [propputref][id] */ HRESULT ( STDMETHODCALLTYPE *putref_Font )( 
            __RPC__in ISystemMonitor2 * This,
            /* [in] */ __RPC__in_opt IFontDisp *pFont);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, get_Counters)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Counters )( 
            __RPC__in ISystemMonitor2 * This,
            /* [retval][out] */ __RPC__deref_out_opt ICounters **ppICounters);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, put_ShowVerticalGrid)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ShowVerticalGrid )( 
            __RPC__in ISystemMonitor2 * This,
            /* [in] */ VARIANT_BOOL bState);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, get_ShowVerticalGrid)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ShowVerticalGrid )( 
            __RPC__in ISystemMonitor2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbState);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, put_ShowHorizontalGrid)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ShowHorizontalGrid )( 
            __RPC__in ISystemMonitor2 * This,
            /* [in] */ VARIANT_BOOL bState);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, get_ShowHorizontalGrid)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ShowHorizontalGrid )( 
            __RPC__in ISystemMonitor2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbState);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, put_ShowLegend)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ShowLegend )( 
            __RPC__in ISystemMonitor2 * This,
            /* [in] */ VARIANT_BOOL bState);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, get_ShowLegend)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ShowLegend )( 
            __RPC__in ISystemMonitor2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbState);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, put_ShowScaleLabels)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ShowScaleLabels )( 
            __RPC__in ISystemMonitor2 * This,
            /* [in] */ VARIANT_BOOL bState);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, get_ShowScaleLabels)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ShowScaleLabels )( 
            __RPC__in ISystemMonitor2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbState);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, put_ShowValueBar)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ShowValueBar )( 
            __RPC__in ISystemMonitor2 * This,
            /* [in] */ VARIANT_BOOL bState);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, get_ShowValueBar)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ShowValueBar )( 
            __RPC__in ISystemMonitor2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbState);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, put_MaximumScale)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_MaximumScale )( 
            __RPC__in ISystemMonitor2 * This,
            /* [in] */ INT iValue);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, get_MaximumScale)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_MaximumScale )( 
            __RPC__in ISystemMonitor2 * This,
            /* [retval][out] */ __RPC__out INT *piValue);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, put_MinimumScale)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_MinimumScale )( 
            __RPC__in ISystemMonitor2 * This,
            /* [in] */ INT iValue);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, get_MinimumScale)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_MinimumScale )( 
            __RPC__in ISystemMonitor2 * This,
            /* [retval][out] */ __RPC__out INT *piValue);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, put_UpdateInterval)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_UpdateInterval )( 
            __RPC__in ISystemMonitor2 * This,
            /* [in] */ FLOAT fValue);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, get_UpdateInterval)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_UpdateInterval )( 
            __RPC__in ISystemMonitor2 * This,
            /* [retval][out] */ __RPC__out FLOAT *pfValue);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, put_DisplayType)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_DisplayType )( 
            __RPC__in ISystemMonitor2 * This,
            /* [in] */ DisplayTypeConstants eDisplayType);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, get_DisplayType)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_DisplayType )( 
            __RPC__in ISystemMonitor2 * This,
            /* [retval][out] */ __RPC__out DisplayTypeConstants *peDisplayType);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, put_ManualUpdate)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ManualUpdate )( 
            __RPC__in ISystemMonitor2 * This,
            /* [in] */ VARIANT_BOOL bState);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, get_ManualUpdate)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ManualUpdate )( 
            __RPC__in ISystemMonitor2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbState);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, put_GraphTitle)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_GraphTitle )( 
            __RPC__in ISystemMonitor2 * This,
            /* [in] */ __RPC__in BSTR bsTitle);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, get_GraphTitle)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_GraphTitle )( 
            __RPC__in ISystemMonitor2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbsTitle);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, put_YAxisLabel)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_YAxisLabel )( 
            __RPC__in ISystemMonitor2 * This,
            /* [in] */ __RPC__in BSTR bsTitle);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, get_YAxisLabel)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_YAxisLabel )( 
            __RPC__in ISystemMonitor2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbsTitle);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, CollectSample)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CollectSample )( 
            __RPC__in ISystemMonitor2 * This);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, UpdateGraph)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *UpdateGraph )( 
            __RPC__in ISystemMonitor2 * This);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, BrowseCounters)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *BrowseCounters )( 
            __RPC__in ISystemMonitor2 * This);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, DisplayProperties)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *DisplayProperties )( 
            __RPC__in ISystemMonitor2 * This);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, Counter)
        /* [hidden][id] */ HRESULT ( STDMETHODCALLTYPE *Counter )( 
            __RPC__in ISystemMonitor2 * This,
            /* [in] */ INT iIndex,
            /* [out] */ __RPC__deref_out_opt ICounterItem **ppICounter);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, AddCounter)
        /* [hidden][id] */ HRESULT ( STDMETHODCALLTYPE *AddCounter )( 
            __RPC__in ISystemMonitor2 * This,
            /* [in] */ __RPC__in BSTR bsPath,
            /* [out] */ __RPC__deref_out_opt ICounterItem **ppICounter);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, DeleteCounter)
        /* [hidden][id] */ HRESULT ( STDMETHODCALLTYPE *DeleteCounter )( 
            __RPC__in ISystemMonitor2 * This,
            /* [in] */ __RPC__in_opt ICounterItem *pCtr);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, get_BackColorCtl)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_BackColorCtl )( 
            __RPC__in ISystemMonitor2 * This,
            /* [retval][out] */ __RPC__out OLE_COLOR *pColor);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, put_BackColorCtl)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_BackColorCtl )( 
            __RPC__in ISystemMonitor2 * This,
            /* [in] */ OLE_COLOR Color);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, put_LogFileName)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_LogFileName )( 
            __RPC__in ISystemMonitor2 * This,
            /* [in] */ __RPC__in BSTR bsFileName);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, get_LogFileName)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_LogFileName )( 
            __RPC__in ISystemMonitor2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *bsFileName);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, put_LogViewStart)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_LogViewStart )( 
            __RPC__in ISystemMonitor2 * This,
            /* [in] */ DATE StartTime);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, get_LogViewStart)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_LogViewStart )( 
            __RPC__in ISystemMonitor2 * This,
            /* [retval][out] */ __RPC__out DATE *StartTime);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, put_LogViewStop)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_LogViewStop )( 
            __RPC__in ISystemMonitor2 * This,
            /* [in] */ DATE StopTime);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, get_LogViewStop)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_LogViewStop )( 
            __RPC__in ISystemMonitor2 * This,
            /* [retval][out] */ __RPC__out DATE *StopTime);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, get_GridColor)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_GridColor )( 
            __RPC__in ISystemMonitor2 * This,
            /* [retval][out] */ __RPC__out OLE_COLOR *pColor);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, put_GridColor)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_GridColor )( 
            __RPC__in ISystemMonitor2 * This,
            /* [in] */ OLE_COLOR Color);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, get_TimeBarColor)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_TimeBarColor )( 
            __RPC__in ISystemMonitor2 * This,
            /* [retval][out] */ __RPC__out OLE_COLOR *pColor);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, put_TimeBarColor)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_TimeBarColor )( 
            __RPC__in ISystemMonitor2 * This,
            /* [in] */ OLE_COLOR Color);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, get_Highlight)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Highlight )( 
            __RPC__in ISystemMonitor2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbState);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, put_Highlight)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Highlight )( 
            __RPC__in ISystemMonitor2 * This,
            /* [in] */ VARIANT_BOOL bState);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, get_ShowToolbar)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ShowToolbar )( 
            __RPC__in ISystemMonitor2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbState);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, put_ShowToolbar)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ShowToolbar )( 
            __RPC__in ISystemMonitor2 * This,
            /* [in] */ VARIANT_BOOL bState);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, Paste)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Paste )( 
            __RPC__in ISystemMonitor2 * This);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, Copy)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Copy )( 
            __RPC__in ISystemMonitor2 * This);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, Reset)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in ISystemMonitor2 * This);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, put_ReadOnly)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ReadOnly )( 
            __RPC__in ISystemMonitor2 * This,
            /* [in] */ VARIANT_BOOL bState);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, get_ReadOnly)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ReadOnly )( 
            __RPC__in ISystemMonitor2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbState);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, put_ReportValueType)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ReportValueType )( 
            __RPC__in ISystemMonitor2 * This,
            /* [in] */ ReportValueTypeConstants eReportValueType);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, get_ReportValueType)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ReportValueType )( 
            __RPC__in ISystemMonitor2 * This,
            /* [retval][out] */ __RPC__out ReportValueTypeConstants *peReportValueType);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, put_MonitorDuplicateInstances)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_MonitorDuplicateInstances )( 
            __RPC__in ISystemMonitor2 * This,
            /* [in] */ VARIANT_BOOL bState);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, get_MonitorDuplicateInstances)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_MonitorDuplicateInstances )( 
            __RPC__in ISystemMonitor2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbState);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, put_DisplayFilter)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_DisplayFilter )( 
            __RPC__in ISystemMonitor2 * This,
            /* [in] */ INT iValue);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, get_DisplayFilter)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_DisplayFilter )( 
            __RPC__in ISystemMonitor2 * This,
            /* [retval][out] */ __RPC__out INT *piValue);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, get_LogFiles)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_LogFiles )( 
            __RPC__in ISystemMonitor2 * This,
            /* [retval][out] */ __RPC__deref_out_opt ILogFiles **ppILogFiles);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, put_DataSourceType)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_DataSourceType )( 
            __RPC__in ISystemMonitor2 * This,
            /* [in] */ DataSourceTypeConstants eDataSourceType);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, get_DataSourceType)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_DataSourceType )( 
            __RPC__in ISystemMonitor2 * This,
            /* [retval][out] */ __RPC__out DataSourceTypeConstants *peDataSourceType);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, put_SqlDsnName)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_SqlDsnName )( 
            __RPC__in ISystemMonitor2 * This,
            /* [in] */ __RPC__in BSTR bsSqlDsnName);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, get_SqlDsnName)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_SqlDsnName )( 
            __RPC__in ISystemMonitor2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *bsSqlDsnName);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, put_SqlLogSetName)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_SqlLogSetName )( 
            __RPC__in ISystemMonitor2 * This,
            /* [in] */ __RPC__in BSTR bsSqlLogSetName);
        
        DECLSPEC_XFGVIRT(ISystemMonitor, get_SqlLogSetName)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_SqlLogSetName )( 
            __RPC__in ISystemMonitor2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *bsSqlLogSetName);
        
        DECLSPEC_XFGVIRT(ISystemMonitor2, put_EnableDigitGrouping)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_EnableDigitGrouping )( 
            __RPC__in ISystemMonitor2 * This,
            /* [in] */ VARIANT_BOOL bState);
        
        DECLSPEC_XFGVIRT(ISystemMonitor2, get_EnableDigitGrouping)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_EnableDigitGrouping )( 
            __RPC__in ISystemMonitor2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbState);
        
        DECLSPEC_XFGVIRT(ISystemMonitor2, put_EnableToolTips)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_EnableToolTips )( 
            __RPC__in ISystemMonitor2 * This,
            /* [in] */ VARIANT_BOOL bState);
        
        DECLSPEC_XFGVIRT(ISystemMonitor2, get_EnableToolTips)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_EnableToolTips )( 
            __RPC__in ISystemMonitor2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbState);
        
        DECLSPEC_XFGVIRT(ISystemMonitor2, put_ShowTimeAxisLabels)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ShowTimeAxisLabels )( 
            __RPC__in ISystemMonitor2 * This,
            /* [in] */ VARIANT_BOOL bState);
        
        DECLSPEC_XFGVIRT(ISystemMonitor2, get_ShowTimeAxisLabels)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ShowTimeAxisLabels )( 
            __RPC__in ISystemMonitor2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbState);
        
        DECLSPEC_XFGVIRT(ISystemMonitor2, put_ChartScroll)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ChartScroll )( 
            __RPC__in ISystemMonitor2 * This,
            /* [in] */ VARIANT_BOOL bScroll);
        
        DECLSPEC_XFGVIRT(ISystemMonitor2, get_ChartScroll)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ChartScroll )( 
            __RPC__in ISystemMonitor2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbScroll);
        
        DECLSPEC_XFGVIRT(ISystemMonitor2, put_DataPointCount)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_DataPointCount )( 
            __RPC__in ISystemMonitor2 * This,
            /* [in] */ INT iNewCount);
        
        DECLSPEC_XFGVIRT(ISystemMonitor2, get_DataPointCount)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_DataPointCount )( 
            __RPC__in ISystemMonitor2 * This,
            /* [retval][out] */ __RPC__out INT *piDataPointCount);
        
        DECLSPEC_XFGVIRT(ISystemMonitor2, ScaleToFit)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ScaleToFit )( 
            __RPC__in ISystemMonitor2 * This,
            VARIANT_BOOL bSelectedCountersOnly);
        
        DECLSPEC_XFGVIRT(ISystemMonitor2, SaveAs)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SaveAs )( 
            __RPC__in ISystemMonitor2 * This,
            __RPC__in BSTR bstrFileName,
            SysmonFileType eSysmonFileType);
        
        DECLSPEC_XFGVIRT(ISystemMonitor2, Relog)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Relog )( 
            __RPC__in ISystemMonitor2 * This,
            __RPC__in BSTR bstrFileName,
            SysmonFileType eSysmonFileType,
            INT iFilter);
        
        DECLSPEC_XFGVIRT(ISystemMonitor2, ClearData)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ClearData )( 
            __RPC__in ISystemMonitor2 * This);
        
        DECLSPEC_XFGVIRT(ISystemMonitor2, get_LogSourceStartTime)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_LogSourceStartTime )( 
            __RPC__in ISystemMonitor2 * This,
            /* [out] */ __RPC__out DATE *pDate);
        
        DECLSPEC_XFGVIRT(ISystemMonitor2, get_LogSourceStopTime)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_LogSourceStopTime )( 
            __RPC__in ISystemMonitor2 * This,
            /* [out] */ __RPC__out DATE *pDate);
        
        DECLSPEC_XFGVIRT(ISystemMonitor2, SetLogViewRange)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetLogViewRange )( 
            __RPC__in ISystemMonitor2 * This,
            /* [in] */ DATE StartTime,
            /* [in] */ DATE StopTime);
        
        DECLSPEC_XFGVIRT(ISystemMonitor2, GetLogViewRange)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetLogViewRange )( 
            __RPC__in ISystemMonitor2 * This,
            /* [out] */ __RPC__out DATE *StartTime,
            /* [out] */ __RPC__out DATE *StopTime);
        
        DECLSPEC_XFGVIRT(ISystemMonitor2, BatchingLock)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *BatchingLock )( 
            __RPC__in ISystemMonitor2 * This,
            /* [in] */ VARIANT_BOOL fLock,
            /* [in] */ SysmonBatchReason eBatchReason);
        
        DECLSPEC_XFGVIRT(ISystemMonitor2, LoadSettings)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *LoadSettings )( 
            __RPC__in ISystemMonitor2 * This,
            /* [in] */ __RPC__in BSTR bstrSettingFileName);
        
        END_INTERFACE
    } ISystemMonitor2Vtbl;

    interface ISystemMonitor2
    {
        CONST_VTBL struct ISystemMonitor2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISystemMonitor2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISystemMonitor2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISystemMonitor2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISystemMonitor2_get_Appearance(This,iAppearance)	\
    ( (This)->lpVtbl -> get_Appearance(This,iAppearance) ) 

#define ISystemMonitor2_put_Appearance(This,iAppearance)	\
    ( (This)->lpVtbl -> put_Appearance(This,iAppearance) ) 

#define ISystemMonitor2_get_BackColor(This,pColor)	\
    ( (This)->lpVtbl -> get_BackColor(This,pColor) ) 

#define ISystemMonitor2_put_BackColor(This,Color)	\
    ( (This)->lpVtbl -> put_BackColor(This,Color) ) 

#define ISystemMonitor2_get_BorderStyle(This,iBorderStyle)	\
    ( (This)->lpVtbl -> get_BorderStyle(This,iBorderStyle) ) 

#define ISystemMonitor2_put_BorderStyle(This,iBorderStyle)	\
    ( (This)->lpVtbl -> put_BorderStyle(This,iBorderStyle) ) 

#define ISystemMonitor2_get_ForeColor(This,pColor)	\
    ( (This)->lpVtbl -> get_ForeColor(This,pColor) ) 

#define ISystemMonitor2_put_ForeColor(This,Color)	\
    ( (This)->lpVtbl -> put_ForeColor(This,Color) ) 

#define ISystemMonitor2_get_Font(This,ppFont)	\
    ( (This)->lpVtbl -> get_Font(This,ppFont) ) 

#define ISystemMonitor2_putref_Font(This,pFont)	\
    ( (This)->lpVtbl -> putref_Font(This,pFont) ) 

#define ISystemMonitor2_get_Counters(This,ppICounters)	\
    ( (This)->lpVtbl -> get_Counters(This,ppICounters) ) 

#define ISystemMonitor2_put_ShowVerticalGrid(This,bState)	\
    ( (This)->lpVtbl -> put_ShowVerticalGrid(This,bState) ) 

#define ISystemMonitor2_get_ShowVerticalGrid(This,pbState)	\
    ( (This)->lpVtbl -> get_ShowVerticalGrid(This,pbState) ) 

#define ISystemMonitor2_put_ShowHorizontalGrid(This,bState)	\
    ( (This)->lpVtbl -> put_ShowHorizontalGrid(This,bState) ) 

#define ISystemMonitor2_get_ShowHorizontalGrid(This,pbState)	\
    ( (This)->lpVtbl -> get_ShowHorizontalGrid(This,pbState) ) 

#define ISystemMonitor2_put_ShowLegend(This,bState)	\
    ( (This)->lpVtbl -> put_ShowLegend(This,bState) ) 

#define ISystemMonitor2_get_ShowLegend(This,pbState)	\
    ( (This)->lpVtbl -> get_ShowLegend(This,pbState) ) 

#define ISystemMonitor2_put_ShowScaleLabels(This,bState)	\
    ( (This)->lpVtbl -> put_ShowScaleLabels(This,bState) ) 

#define ISystemMonitor2_get_ShowScaleLabels(This,pbState)	\
    ( (This)->lpVtbl -> get_ShowScaleLabels(This,pbState) ) 

#define ISystemMonitor2_put_ShowValueBar(This,bState)	\
    ( (This)->lpVtbl -> put_ShowValueBar(This,bState) ) 

#define ISystemMonitor2_get_ShowValueBar(This,pbState)	\
    ( (This)->lpVtbl -> get_ShowValueBar(This,pbState) ) 

#define ISystemMonitor2_put_MaximumScale(This,iValue)	\
    ( (This)->lpVtbl -> put_MaximumScale(This,iValue) ) 

#define ISystemMonitor2_get_MaximumScale(This,piValue)	\
    ( (This)->lpVtbl -> get_MaximumScale(This,piValue) ) 

#define ISystemMonitor2_put_MinimumScale(This,iValue)	\
    ( (This)->lpVtbl -> put_MinimumScale(This,iValue) ) 

#define ISystemMonitor2_get_MinimumScale(This,piValue)	\
    ( (This)->lpVtbl -> get_MinimumScale(This,piValue) ) 

#define ISystemMonitor2_put_UpdateInterval(This,fValue)	\
    ( (This)->lpVtbl -> put_UpdateInterval(This,fValue) ) 

#define ISystemMonitor2_get_UpdateInterval(This,pfValue)	\
    ( (This)->lpVtbl -> get_UpdateInterval(This,pfValue) ) 

#define ISystemMonitor2_put_DisplayType(This,eDisplayType)	\
    ( (This)->lpVtbl -> put_DisplayType(This,eDisplayType) ) 

#define ISystemMonitor2_get_DisplayType(This,peDisplayType)	\
    ( (This)->lpVtbl -> get_DisplayType(This,peDisplayType) ) 

#define ISystemMonitor2_put_ManualUpdate(This,bState)	\
    ( (This)->lpVtbl -> put_ManualUpdate(This,bState) ) 

#define ISystemMonitor2_get_ManualUpdate(This,pbState)	\
    ( (This)->lpVtbl -> get_ManualUpdate(This,pbState) ) 

#define ISystemMonitor2_put_GraphTitle(This,bsTitle)	\
    ( (This)->lpVtbl -> put_GraphTitle(This,bsTitle) ) 

#define ISystemMonitor2_get_GraphTitle(This,pbsTitle)	\
    ( (This)->lpVtbl -> get_GraphTitle(This,pbsTitle) ) 

#define ISystemMonitor2_put_YAxisLabel(This,bsTitle)	\
    ( (This)->lpVtbl -> put_YAxisLabel(This,bsTitle) ) 

#define ISystemMonitor2_get_YAxisLabel(This,pbsTitle)	\
    ( (This)->lpVtbl -> get_YAxisLabel(This,pbsTitle) ) 

#define ISystemMonitor2_CollectSample(This)	\
    ( (This)->lpVtbl -> CollectSample(This) ) 

#define ISystemMonitor2_UpdateGraph(This)	\
    ( (This)->lpVtbl -> UpdateGraph(This) ) 

#define ISystemMonitor2_BrowseCounters(This)	\
    ( (This)->lpVtbl -> BrowseCounters(This) ) 

#define ISystemMonitor2_DisplayProperties(This)	\
    ( (This)->lpVtbl -> DisplayProperties(This) ) 

#define ISystemMonitor2_Counter(This,iIndex,ppICounter)	\
    ( (This)->lpVtbl -> Counter(This,iIndex,ppICounter) ) 

#define ISystemMonitor2_AddCounter(This,bsPath,ppICounter)	\
    ( (This)->lpVtbl -> AddCounter(This,bsPath,ppICounter) ) 

#define ISystemMonitor2_DeleteCounter(This,pCtr)	\
    ( (This)->lpVtbl -> DeleteCounter(This,pCtr) ) 

#define ISystemMonitor2_get_BackColorCtl(This,pColor)	\
    ( (This)->lpVtbl -> get_BackColorCtl(This,pColor) ) 

#define ISystemMonitor2_put_BackColorCtl(This,Color)	\
    ( (This)->lpVtbl -> put_BackColorCtl(This,Color) ) 

#define ISystemMonitor2_put_LogFileName(This,bsFileName)	\
    ( (This)->lpVtbl -> put_LogFileName(This,bsFileName) ) 

#define ISystemMonitor2_get_LogFileName(This,bsFileName)	\
    ( (This)->lpVtbl -> get_LogFileName(This,bsFileName) ) 

#define ISystemMonitor2_put_LogViewStart(This,StartTime)	\
    ( (This)->lpVtbl -> put_LogViewStart(This,StartTime) ) 

#define ISystemMonitor2_get_LogViewStart(This,StartTime)	\
    ( (This)->lpVtbl -> get_LogViewStart(This,StartTime) ) 

#define ISystemMonitor2_put_LogViewStop(This,StopTime)	\
    ( (This)->lpVtbl -> put_LogViewStop(This,StopTime) ) 

#define ISystemMonitor2_get_LogViewStop(This,StopTime)	\
    ( (This)->lpVtbl -> get_LogViewStop(This,StopTime) ) 

#define ISystemMonitor2_get_GridColor(This,pColor)	\
    ( (This)->lpVtbl -> get_GridColor(This,pColor) ) 

#define ISystemMonitor2_put_GridColor(This,Color)	\
    ( (This)->lpVtbl -> put_GridColor(This,Color) ) 

#define ISystemMonitor2_get_TimeBarColor(This,pColor)	\
    ( (This)->lpVtbl -> get_TimeBarColor(This,pColor) ) 

#define ISystemMonitor2_put_TimeBarColor(This,Color)	\
    ( (This)->lpVtbl -> put_TimeBarColor(This,Color) ) 

#define ISystemMonitor2_get_Highlight(This,pbState)	\
    ( (This)->lpVtbl -> get_Highlight(This,pbState) ) 

#define ISystemMonitor2_put_Highlight(This,bState)	\
    ( (This)->lpVtbl -> put_Highlight(This,bState) ) 

#define ISystemMonitor2_get_ShowToolbar(This,pbState)	\
    ( (This)->lpVtbl -> get_ShowToolbar(This,pbState) ) 

#define ISystemMonitor2_put_ShowToolbar(This,bState)	\
    ( (This)->lpVtbl -> put_ShowToolbar(This,bState) ) 

#define ISystemMonitor2_Paste(This)	\
    ( (This)->lpVtbl -> Paste(This) ) 

#define ISystemMonitor2_Copy(This)	\
    ( (This)->lpVtbl -> Copy(This) ) 

#define ISystemMonitor2_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define ISystemMonitor2_put_ReadOnly(This,bState)	\
    ( (This)->lpVtbl -> put_ReadOnly(This,bState) ) 

#define ISystemMonitor2_get_ReadOnly(This,pbState)	\
    ( (This)->lpVtbl -> get_ReadOnly(This,pbState) ) 

#define ISystemMonitor2_put_ReportValueType(This,eReportValueType)	\
    ( (This)->lpVtbl -> put_ReportValueType(This,eReportValueType) ) 

#define ISystemMonitor2_get_ReportValueType(This,peReportValueType)	\
    ( (This)->lpVtbl -> get_ReportValueType(This,peReportValueType) ) 

#define ISystemMonitor2_put_MonitorDuplicateInstances(This,bState)	\
    ( (This)->lpVtbl -> put_MonitorDuplicateInstances(This,bState) ) 

#define ISystemMonitor2_get_MonitorDuplicateInstances(This,pbState)	\
    ( (This)->lpVtbl -> get_MonitorDuplicateInstances(This,pbState) ) 

#define ISystemMonitor2_put_DisplayFilter(This,iValue)	\
    ( (This)->lpVtbl -> put_DisplayFilter(This,iValue) ) 

#define ISystemMonitor2_get_DisplayFilter(This,piValue)	\
    ( (This)->lpVtbl -> get_DisplayFilter(This,piValue) ) 

#define ISystemMonitor2_get_LogFiles(This,ppILogFiles)	\
    ( (This)->lpVtbl -> get_LogFiles(This,ppILogFiles) ) 

#define ISystemMonitor2_put_DataSourceType(This,eDataSourceType)	\
    ( (This)->lpVtbl -> put_DataSourceType(This,eDataSourceType) ) 

#define ISystemMonitor2_get_DataSourceType(This,peDataSourceType)	\
    ( (This)->lpVtbl -> get_DataSourceType(This,peDataSourceType) ) 

#define ISystemMonitor2_put_SqlDsnName(This,bsSqlDsnName)	\
    ( (This)->lpVtbl -> put_SqlDsnName(This,bsSqlDsnName) ) 

#define ISystemMonitor2_get_SqlDsnName(This,bsSqlDsnName)	\
    ( (This)->lpVtbl -> get_SqlDsnName(This,bsSqlDsnName) ) 

#define ISystemMonitor2_put_SqlLogSetName(This,bsSqlLogSetName)	\
    ( (This)->lpVtbl -> put_SqlLogSetName(This,bsSqlLogSetName) ) 

#define ISystemMonitor2_get_SqlLogSetName(This,bsSqlLogSetName)	\
    ( (This)->lpVtbl -> get_SqlLogSetName(This,bsSqlLogSetName) ) 


#define ISystemMonitor2_put_EnableDigitGrouping(This,bState)	\
    ( (This)->lpVtbl -> put_EnableDigitGrouping(This,bState) ) 

#define ISystemMonitor2_get_EnableDigitGrouping(This,pbState)	\
    ( (This)->lpVtbl -> get_EnableDigitGrouping(This,pbState) ) 

#define ISystemMonitor2_put_EnableToolTips(This,bState)	\
    ( (This)->lpVtbl -> put_EnableToolTips(This,bState) ) 

#define ISystemMonitor2_get_EnableToolTips(This,pbState)	\
    ( (This)->lpVtbl -> get_EnableToolTips(This,pbState) ) 

#define ISystemMonitor2_put_ShowTimeAxisLabels(This,bState)	\
    ( (This)->lpVtbl -> put_ShowTimeAxisLabels(This,bState) ) 

#define ISystemMonitor2_get_ShowTimeAxisLabels(This,pbState)	\
    ( (This)->lpVtbl -> get_ShowTimeAxisLabels(This,pbState) ) 

#define ISystemMonitor2_put_ChartScroll(This,bScroll)	\
    ( (This)->lpVtbl -> put_ChartScroll(This,bScroll) ) 

#define ISystemMonitor2_get_ChartScroll(This,pbScroll)	\
    ( (This)->lpVtbl -> get_ChartScroll(This,pbScroll) ) 

#define ISystemMonitor2_put_DataPointCount(This,iNewCount)	\
    ( (This)->lpVtbl -> put_DataPointCount(This,iNewCount) ) 

#define ISystemMonitor2_get_DataPointCount(This,piDataPointCount)	\
    ( (This)->lpVtbl -> get_DataPointCount(This,piDataPointCount) ) 

#define ISystemMonitor2_ScaleToFit(This,bSelectedCountersOnly)	\
    ( (This)->lpVtbl -> ScaleToFit(This,bSelectedCountersOnly) ) 

#define ISystemMonitor2_SaveAs(This,bstrFileName,eSysmonFileType)	\
    ( (This)->lpVtbl -> SaveAs(This,bstrFileName,eSysmonFileType) ) 

#define ISystemMonitor2_Relog(This,bstrFileName,eSysmonFileType,iFilter)	\
    ( (This)->lpVtbl -> Relog(This,bstrFileName,eSysmonFileType,iFilter) ) 

#define ISystemMonitor2_ClearData(This)	\
    ( (This)->lpVtbl -> ClearData(This) ) 

#define ISystemMonitor2_get_LogSourceStartTime(This,pDate)	\
    ( (This)->lpVtbl -> get_LogSourceStartTime(This,pDate) ) 

#define ISystemMonitor2_get_LogSourceStopTime(This,pDate)	\
    ( (This)->lpVtbl -> get_LogSourceStopTime(This,pDate) ) 

#define ISystemMonitor2_SetLogViewRange(This,StartTime,StopTime)	\
    ( (This)->lpVtbl -> SetLogViewRange(This,StartTime,StopTime) ) 

#define ISystemMonitor2_GetLogViewRange(This,StartTime,StopTime)	\
    ( (This)->lpVtbl -> GetLogViewRange(This,StartTime,StopTime) ) 

#define ISystemMonitor2_BatchingLock(This,fLock,eBatchReason)	\
    ( (This)->lpVtbl -> BatchingLock(This,fLock,eBatchReason) ) 

#define ISystemMonitor2_LoadSettings(This,bstrSettingFileName)	\
    ( (This)->lpVtbl -> LoadSettings(This,bstrSettingFileName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISystemMonitor2_INTERFACE_DEFINED__ */


#ifndef ___ISystemMonitorUnion_INTERFACE_DEFINED__
#define ___ISystemMonitorUnion_INTERFACE_DEFINED__

/* interface _ISystemMonitorUnion */
/* [object][hidden][uuid] */ 


DEFINE_GUID(IID__ISystemMonitorUnion,0xc8a77338,0x265f,0x4de5,0xaa,0x25,0xc7,0xda,0x1c,0xe5,0xa8,0xf4);

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("c8a77338-265f-4de5-aa25-c7da1ce5a8f4")
    _ISystemMonitorUnion : public IUnknown
    {
    public:
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Appearance( 
            /* [retval][out] */ __RPC__out INT *iAppearance) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE put_Appearance( 
            /* [in] */ INT iAppearance) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_BackColor( 
            /* [retval][out] */ __RPC__out OLE_COLOR *pColor) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE put_BackColor( 
            /* [in] */ OLE_COLOR Color) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_BorderStyle( 
            /* [retval][out] */ __RPC__out INT *iBorderStyle) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE put_BorderStyle( 
            /* [in] */ INT iBorderStyle) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_ForeColor( 
            /* [retval][out] */ __RPC__out OLE_COLOR *pColor) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE put_ForeColor( 
            /* [in] */ OLE_COLOR Color) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Font( 
            /* [retval][out] */ __RPC__deref_out_opt IFontDisp **ppFont) = 0;
        
        virtual /* [propputref][id] */ HRESULT STDMETHODCALLTYPE putref_Font( 
            /* [in] */ __RPC__in_opt IFontDisp *pFont) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Counters( 
            /* [retval][out] */ __RPC__deref_out_opt ICounters **ppICounters) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_ShowVerticalGrid( 
            /* [in] */ VARIANT_BOOL bState) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_ShowVerticalGrid( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbState) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_ShowHorizontalGrid( 
            /* [in] */ VARIANT_BOOL bState) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_ShowHorizontalGrid( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbState) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_ShowLegend( 
            /* [in] */ VARIANT_BOOL bState) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_ShowLegend( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbState) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_ShowScaleLabels( 
            /* [in] */ VARIANT_BOOL bState) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_ShowScaleLabels( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbState) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_ShowValueBar( 
            /* [in] */ VARIANT_BOOL bState) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_ShowValueBar( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbState) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_MaximumScale( 
            /* [in] */ INT iValue) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_MaximumScale( 
            /* [retval][out] */ __RPC__out INT *piValue) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_MinimumScale( 
            /* [in] */ INT iValue) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_MinimumScale( 
            /* [retval][out] */ __RPC__out INT *piValue) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_UpdateInterval( 
            /* [in] */ FLOAT fValue) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_UpdateInterval( 
            /* [retval][out] */ __RPC__out FLOAT *pfValue) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_DisplayType( 
            /* [in] */ DisplayTypeConstants eDisplayType) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_DisplayType( 
            /* [retval][out] */ __RPC__out DisplayTypeConstants *peDisplayType) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_ManualUpdate( 
            /* [in] */ VARIANT_BOOL bState) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_ManualUpdate( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbState) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_GraphTitle( 
            /* [in] */ __RPC__in BSTR bsTitle) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_GraphTitle( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbsTitle) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_YAxisLabel( 
            /* [in] */ __RPC__in BSTR bsTitle) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_YAxisLabel( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbsTitle) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CollectSample( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE UpdateGraph( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE BrowseCounters( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE DisplayProperties( void) = 0;
        
        virtual /* [hidden][id] */ HRESULT STDMETHODCALLTYPE Counter( 
            /* [in] */ INT iIndex,
            /* [out] */ __RPC__deref_out_opt ICounterItem **ppICounter) = 0;
        
        virtual /* [hidden][id] */ HRESULT STDMETHODCALLTYPE AddCounter( 
            /* [in] */ __RPC__in BSTR bsPath,
            /* [out] */ __RPC__deref_out_opt ICounterItem **ppICounter) = 0;
        
        virtual /* [hidden][id] */ HRESULT STDMETHODCALLTYPE DeleteCounter( 
            /* [in] */ __RPC__in_opt ICounterItem *pCtr) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_BackColorCtl( 
            /* [retval][out] */ __RPC__out OLE_COLOR *pColor) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE put_BackColorCtl( 
            /* [in] */ OLE_COLOR Color) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_LogFileName( 
            /* [in] */ __RPC__in BSTR bsFileName) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_LogFileName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *bsFileName) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_LogViewStart( 
            /* [in] */ DATE StartTime) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_LogViewStart( 
            /* [retval][out] */ __RPC__out DATE *StartTime) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_LogViewStop( 
            /* [in] */ DATE StopTime) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_LogViewStop( 
            /* [retval][out] */ __RPC__out DATE *StopTime) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_GridColor( 
            /* [retval][out] */ __RPC__out OLE_COLOR *pColor) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE put_GridColor( 
            /* [in] */ OLE_COLOR Color) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_TimeBarColor( 
            /* [retval][out] */ __RPC__out OLE_COLOR *pColor) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE put_TimeBarColor( 
            /* [in] */ OLE_COLOR Color) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Highlight( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbState) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE put_Highlight( 
            /* [in] */ VARIANT_BOOL bState) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_ShowToolbar( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbState) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE put_ShowToolbar( 
            /* [in] */ VARIANT_BOOL bState) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Paste( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Copy( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_ReadOnly( 
            /* [in] */ VARIANT_BOOL bState) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_ReadOnly( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbState) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_ReportValueType( 
            /* [in] */ ReportValueTypeConstants eReportValueType) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_ReportValueType( 
            /* [retval][out] */ __RPC__out ReportValueTypeConstants *peReportValueType) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_MonitorDuplicateInstances( 
            /* [in] */ VARIANT_BOOL bState) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_MonitorDuplicateInstances( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbState) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_DisplayFilter( 
            /* [in] */ INT iValue) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_DisplayFilter( 
            /* [retval][out] */ __RPC__out INT *piValue) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_LogFiles( 
            /* [retval][out] */ __RPC__deref_out_opt ILogFiles **ppILogFiles) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_DataSourceType( 
            /* [in] */ DataSourceTypeConstants eDataSourceType) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_DataSourceType( 
            /* [retval][out] */ __RPC__out DataSourceTypeConstants *peDataSourceType) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_SqlDsnName( 
            /* [in] */ __RPC__in BSTR bsSqlDsnName) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_SqlDsnName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *bsSqlDsnName) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_SqlLogSetName( 
            /* [in] */ __RPC__in BSTR bsSqlLogSetName) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_SqlLogSetName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *bsSqlLogSetName) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_EnableDigitGrouping( 
            /* [in] */ VARIANT_BOOL bState) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_EnableDigitGrouping( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbState) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_EnableToolTips( 
            /* [in] */ VARIANT_BOOL bState) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_EnableToolTips( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbState) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_ShowTimeAxisLabels( 
            /* [in] */ VARIANT_BOOL bState) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_ShowTimeAxisLabels( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbState) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_ChartScroll( 
            /* [in] */ VARIANT_BOOL bScroll) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_ChartScroll( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbScroll) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_DataPointCount( 
            /* [in] */ INT iNewCount) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_DataPointCount( 
            /* [retval][out] */ __RPC__out INT *piDataPointCount) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ScaleToFit( 
            VARIANT_BOOL bSelectedCountersOnly) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SaveAs( 
            __RPC__in BSTR bstrFileName,
            SysmonFileType eSysmonFileType) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Relog( 
            __RPC__in BSTR bstrFileName,
            SysmonFileType eSysmonFileType,
            INT iFilter) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ClearData( void) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_LogSourceStartTime( 
            /* [out] */ __RPC__out DATE *pDate) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_LogSourceStopTime( 
            /* [out] */ __RPC__out DATE *pDate) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetLogViewRange( 
            /* [in] */ DATE StartTime,
            /* [in] */ DATE StopTime) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetLogViewRange( 
            /* [out] */ __RPC__out DATE *StartTime,
            /* [out] */ __RPC__out DATE *StopTime) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE BatchingLock( 
            /* [in] */ VARIANT_BOOL fLock,
            /* [in] */ SysmonBatchReason eBatchReason) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE LoadSettings( 
            /* [in] */ __RPC__in BSTR bstrSettingFileName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct _ISystemMonitorUnionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in _ISystemMonitorUnion * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in _ISystemMonitorUnion * This);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, get_Appearance)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Appearance )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [retval][out] */ __RPC__out INT *iAppearance);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, put_Appearance)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Appearance )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [in] */ INT iAppearance);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, get_BackColor)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_BackColor )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [retval][out] */ __RPC__out OLE_COLOR *pColor);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, put_BackColor)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_BackColor )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [in] */ OLE_COLOR Color);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, get_BorderStyle)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_BorderStyle )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [retval][out] */ __RPC__out INT *iBorderStyle);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, put_BorderStyle)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_BorderStyle )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [in] */ INT iBorderStyle);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, get_ForeColor)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ForeColor )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [retval][out] */ __RPC__out OLE_COLOR *pColor);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, put_ForeColor)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ForeColor )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [in] */ OLE_COLOR Color);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, get_Font)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Font )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [retval][out] */ __RPC__deref_out_opt IFontDisp **ppFont);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, putref_Font)
        /* [propputref][id] */ HRESULT ( STDMETHODCALLTYPE *putref_Font )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [in] */ __RPC__in_opt IFontDisp *pFont);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, get_Counters)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Counters )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [retval][out] */ __RPC__deref_out_opt ICounters **ppICounters);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, put_ShowVerticalGrid)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ShowVerticalGrid )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [in] */ VARIANT_BOOL bState);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, get_ShowVerticalGrid)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ShowVerticalGrid )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbState);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, put_ShowHorizontalGrid)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ShowHorizontalGrid )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [in] */ VARIANT_BOOL bState);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, get_ShowHorizontalGrid)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ShowHorizontalGrid )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbState);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, put_ShowLegend)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ShowLegend )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [in] */ VARIANT_BOOL bState);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, get_ShowLegend)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ShowLegend )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbState);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, put_ShowScaleLabels)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ShowScaleLabels )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [in] */ VARIANT_BOOL bState);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, get_ShowScaleLabels)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ShowScaleLabels )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbState);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, put_ShowValueBar)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ShowValueBar )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [in] */ VARIANT_BOOL bState);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, get_ShowValueBar)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ShowValueBar )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbState);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, put_MaximumScale)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_MaximumScale )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [in] */ INT iValue);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, get_MaximumScale)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_MaximumScale )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [retval][out] */ __RPC__out INT *piValue);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, put_MinimumScale)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_MinimumScale )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [in] */ INT iValue);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, get_MinimumScale)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_MinimumScale )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [retval][out] */ __RPC__out INT *piValue);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, put_UpdateInterval)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_UpdateInterval )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [in] */ FLOAT fValue);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, get_UpdateInterval)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_UpdateInterval )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [retval][out] */ __RPC__out FLOAT *pfValue);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, put_DisplayType)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_DisplayType )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [in] */ DisplayTypeConstants eDisplayType);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, get_DisplayType)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_DisplayType )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [retval][out] */ __RPC__out DisplayTypeConstants *peDisplayType);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, put_ManualUpdate)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ManualUpdate )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [in] */ VARIANT_BOOL bState);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, get_ManualUpdate)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ManualUpdate )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbState);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, put_GraphTitle)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_GraphTitle )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [in] */ __RPC__in BSTR bsTitle);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, get_GraphTitle)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_GraphTitle )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbsTitle);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, put_YAxisLabel)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_YAxisLabel )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [in] */ __RPC__in BSTR bsTitle);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, get_YAxisLabel)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_YAxisLabel )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbsTitle);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, CollectSample)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CollectSample )( 
            __RPC__in _ISystemMonitorUnion * This);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, UpdateGraph)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *UpdateGraph )( 
            __RPC__in _ISystemMonitorUnion * This);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, BrowseCounters)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *BrowseCounters )( 
            __RPC__in _ISystemMonitorUnion * This);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, DisplayProperties)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *DisplayProperties )( 
            __RPC__in _ISystemMonitorUnion * This);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, Counter)
        /* [hidden][id] */ HRESULT ( STDMETHODCALLTYPE *Counter )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [in] */ INT iIndex,
            /* [out] */ __RPC__deref_out_opt ICounterItem **ppICounter);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, AddCounter)
        /* [hidden][id] */ HRESULT ( STDMETHODCALLTYPE *AddCounter )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [in] */ __RPC__in BSTR bsPath,
            /* [out] */ __RPC__deref_out_opt ICounterItem **ppICounter);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, DeleteCounter)
        /* [hidden][id] */ HRESULT ( STDMETHODCALLTYPE *DeleteCounter )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [in] */ __RPC__in_opt ICounterItem *pCtr);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, get_BackColorCtl)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_BackColorCtl )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [retval][out] */ __RPC__out OLE_COLOR *pColor);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, put_BackColorCtl)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_BackColorCtl )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [in] */ OLE_COLOR Color);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, put_LogFileName)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_LogFileName )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [in] */ __RPC__in BSTR bsFileName);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, get_LogFileName)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_LogFileName )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *bsFileName);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, put_LogViewStart)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_LogViewStart )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [in] */ DATE StartTime);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, get_LogViewStart)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_LogViewStart )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [retval][out] */ __RPC__out DATE *StartTime);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, put_LogViewStop)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_LogViewStop )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [in] */ DATE StopTime);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, get_LogViewStop)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_LogViewStop )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [retval][out] */ __RPC__out DATE *StopTime);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, get_GridColor)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_GridColor )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [retval][out] */ __RPC__out OLE_COLOR *pColor);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, put_GridColor)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_GridColor )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [in] */ OLE_COLOR Color);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, get_TimeBarColor)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_TimeBarColor )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [retval][out] */ __RPC__out OLE_COLOR *pColor);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, put_TimeBarColor)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_TimeBarColor )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [in] */ OLE_COLOR Color);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, get_Highlight)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Highlight )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbState);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, put_Highlight)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Highlight )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [in] */ VARIANT_BOOL bState);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, get_ShowToolbar)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ShowToolbar )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbState);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, put_ShowToolbar)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ShowToolbar )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [in] */ VARIANT_BOOL bState);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, Paste)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Paste )( 
            __RPC__in _ISystemMonitorUnion * This);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, Copy)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Copy )( 
            __RPC__in _ISystemMonitorUnion * This);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, Reset)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in _ISystemMonitorUnion * This);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, put_ReadOnly)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ReadOnly )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [in] */ VARIANT_BOOL bState);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, get_ReadOnly)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ReadOnly )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbState);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, put_ReportValueType)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ReportValueType )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [in] */ ReportValueTypeConstants eReportValueType);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, get_ReportValueType)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ReportValueType )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [retval][out] */ __RPC__out ReportValueTypeConstants *peReportValueType);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, put_MonitorDuplicateInstances)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_MonitorDuplicateInstances )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [in] */ VARIANT_BOOL bState);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, get_MonitorDuplicateInstances)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_MonitorDuplicateInstances )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbState);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, put_DisplayFilter)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_DisplayFilter )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [in] */ INT iValue);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, get_DisplayFilter)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_DisplayFilter )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [retval][out] */ __RPC__out INT *piValue);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, get_LogFiles)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_LogFiles )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [retval][out] */ __RPC__deref_out_opt ILogFiles **ppILogFiles);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, put_DataSourceType)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_DataSourceType )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [in] */ DataSourceTypeConstants eDataSourceType);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, get_DataSourceType)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_DataSourceType )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [retval][out] */ __RPC__out DataSourceTypeConstants *peDataSourceType);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, put_SqlDsnName)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_SqlDsnName )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [in] */ __RPC__in BSTR bsSqlDsnName);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, get_SqlDsnName)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_SqlDsnName )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *bsSqlDsnName);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, put_SqlLogSetName)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_SqlLogSetName )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [in] */ __RPC__in BSTR bsSqlLogSetName);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, get_SqlLogSetName)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_SqlLogSetName )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *bsSqlLogSetName);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, put_EnableDigitGrouping)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_EnableDigitGrouping )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [in] */ VARIANT_BOOL bState);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, get_EnableDigitGrouping)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_EnableDigitGrouping )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbState);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, put_EnableToolTips)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_EnableToolTips )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [in] */ VARIANT_BOOL bState);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, get_EnableToolTips)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_EnableToolTips )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbState);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, put_ShowTimeAxisLabels)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ShowTimeAxisLabels )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [in] */ VARIANT_BOOL bState);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, get_ShowTimeAxisLabels)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ShowTimeAxisLabels )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbState);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, put_ChartScroll)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ChartScroll )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [in] */ VARIANT_BOOL bScroll);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, get_ChartScroll)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ChartScroll )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbScroll);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, put_DataPointCount)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_DataPointCount )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [in] */ INT iNewCount);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, get_DataPointCount)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_DataPointCount )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [retval][out] */ __RPC__out INT *piDataPointCount);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, ScaleToFit)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ScaleToFit )( 
            __RPC__in _ISystemMonitorUnion * This,
            VARIANT_BOOL bSelectedCountersOnly);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, SaveAs)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SaveAs )( 
            __RPC__in _ISystemMonitorUnion * This,
            __RPC__in BSTR bstrFileName,
            SysmonFileType eSysmonFileType);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, Relog)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Relog )( 
            __RPC__in _ISystemMonitorUnion * This,
            __RPC__in BSTR bstrFileName,
            SysmonFileType eSysmonFileType,
            INT iFilter);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, ClearData)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ClearData )( 
            __RPC__in _ISystemMonitorUnion * This);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, get_LogSourceStartTime)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_LogSourceStartTime )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [out] */ __RPC__out DATE *pDate);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, get_LogSourceStopTime)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_LogSourceStopTime )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [out] */ __RPC__out DATE *pDate);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, SetLogViewRange)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetLogViewRange )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [in] */ DATE StartTime,
            /* [in] */ DATE StopTime);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, GetLogViewRange)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetLogViewRange )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [out] */ __RPC__out DATE *StartTime,
            /* [out] */ __RPC__out DATE *StopTime);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, BatchingLock)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *BatchingLock )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [in] */ VARIANT_BOOL fLock,
            /* [in] */ SysmonBatchReason eBatchReason);
        
        DECLSPEC_XFGVIRT(_ISystemMonitorUnion, LoadSettings)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *LoadSettings )( 
            __RPC__in _ISystemMonitorUnion * This,
            /* [in] */ __RPC__in BSTR bstrSettingFileName);
        
        END_INTERFACE
    } _ISystemMonitorUnionVtbl;

    interface _ISystemMonitorUnion
    {
        CONST_VTBL struct _ISystemMonitorUnionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define _ISystemMonitorUnion_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define _ISystemMonitorUnion_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define _ISystemMonitorUnion_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define _ISystemMonitorUnion_get_Appearance(This,iAppearance)	\
    ( (This)->lpVtbl -> get_Appearance(This,iAppearance) ) 

#define _ISystemMonitorUnion_put_Appearance(This,iAppearance)	\
    ( (This)->lpVtbl -> put_Appearance(This,iAppearance) ) 

#define _ISystemMonitorUnion_get_BackColor(This,pColor)	\
    ( (This)->lpVtbl -> get_BackColor(This,pColor) ) 

#define _ISystemMonitorUnion_put_BackColor(This,Color)	\
    ( (This)->lpVtbl -> put_BackColor(This,Color) ) 

#define _ISystemMonitorUnion_get_BorderStyle(This,iBorderStyle)	\
    ( (This)->lpVtbl -> get_BorderStyle(This,iBorderStyle) ) 

#define _ISystemMonitorUnion_put_BorderStyle(This,iBorderStyle)	\
    ( (This)->lpVtbl -> put_BorderStyle(This,iBorderStyle) ) 

#define _ISystemMonitorUnion_get_ForeColor(This,pColor)	\
    ( (This)->lpVtbl -> get_ForeColor(This,pColor) ) 

#define _ISystemMonitorUnion_put_ForeColor(This,Color)	\
    ( (This)->lpVtbl -> put_ForeColor(This,Color) ) 

#define _ISystemMonitorUnion_get_Font(This,ppFont)	\
    ( (This)->lpVtbl -> get_Font(This,ppFont) ) 

#define _ISystemMonitorUnion_putref_Font(This,pFont)	\
    ( (This)->lpVtbl -> putref_Font(This,pFont) ) 

#define _ISystemMonitorUnion_get_Counters(This,ppICounters)	\
    ( (This)->lpVtbl -> get_Counters(This,ppICounters) ) 

#define _ISystemMonitorUnion_put_ShowVerticalGrid(This,bState)	\
    ( (This)->lpVtbl -> put_ShowVerticalGrid(This,bState) ) 

#define _ISystemMonitorUnion_get_ShowVerticalGrid(This,pbState)	\
    ( (This)->lpVtbl -> get_ShowVerticalGrid(This,pbState) ) 

#define _ISystemMonitorUnion_put_ShowHorizontalGrid(This,bState)	\
    ( (This)->lpVtbl -> put_ShowHorizontalGrid(This,bState) ) 

#define _ISystemMonitorUnion_get_ShowHorizontalGrid(This,pbState)	\
    ( (This)->lpVtbl -> get_ShowHorizontalGrid(This,pbState) ) 

#define _ISystemMonitorUnion_put_ShowLegend(This,bState)	\
    ( (This)->lpVtbl -> put_ShowLegend(This,bState) ) 

#define _ISystemMonitorUnion_get_ShowLegend(This,pbState)	\
    ( (This)->lpVtbl -> get_ShowLegend(This,pbState) ) 

#define _ISystemMonitorUnion_put_ShowScaleLabels(This,bState)	\
    ( (This)->lpVtbl -> put_ShowScaleLabels(This,bState) ) 

#define _ISystemMonitorUnion_get_ShowScaleLabels(This,pbState)	\
    ( (This)->lpVtbl -> get_ShowScaleLabels(This,pbState) ) 

#define _ISystemMonitorUnion_put_ShowValueBar(This,bState)	\
    ( (This)->lpVtbl -> put_ShowValueBar(This,bState) ) 

#define _ISystemMonitorUnion_get_ShowValueBar(This,pbState)	\
    ( (This)->lpVtbl -> get_ShowValueBar(This,pbState) ) 

#define _ISystemMonitorUnion_put_MaximumScale(This,iValue)	\
    ( (This)->lpVtbl -> put_MaximumScale(This,iValue) ) 

#define _ISystemMonitorUnion_get_MaximumScale(This,piValue)	\
    ( (This)->lpVtbl -> get_MaximumScale(This,piValue) ) 

#define _ISystemMonitorUnion_put_MinimumScale(This,iValue)	\
    ( (This)->lpVtbl -> put_MinimumScale(This,iValue) ) 

#define _ISystemMonitorUnion_get_MinimumScale(This,piValue)	\
    ( (This)->lpVtbl -> get_MinimumScale(This,piValue) ) 

#define _ISystemMonitorUnion_put_UpdateInterval(This,fValue)	\
    ( (This)->lpVtbl -> put_UpdateInterval(This,fValue) ) 

#define _ISystemMonitorUnion_get_UpdateInterval(This,pfValue)	\
    ( (This)->lpVtbl -> get_UpdateInterval(This,pfValue) ) 

#define _ISystemMonitorUnion_put_DisplayType(This,eDisplayType)	\
    ( (This)->lpVtbl -> put_DisplayType(This,eDisplayType) ) 

#define _ISystemMonitorUnion_get_DisplayType(This,peDisplayType)	\
    ( (This)->lpVtbl -> get_DisplayType(This,peDisplayType) ) 

#define _ISystemMonitorUnion_put_ManualUpdate(This,bState)	\
    ( (This)->lpVtbl -> put_ManualUpdate(This,bState) ) 

#define _ISystemMonitorUnion_get_ManualUpdate(This,pbState)	\
    ( (This)->lpVtbl -> get_ManualUpdate(This,pbState) ) 

#define _ISystemMonitorUnion_put_GraphTitle(This,bsTitle)	\
    ( (This)->lpVtbl -> put_GraphTitle(This,bsTitle) ) 

#define _ISystemMonitorUnion_get_GraphTitle(This,pbsTitle)	\
    ( (This)->lpVtbl -> get_GraphTitle(This,pbsTitle) ) 

#define _ISystemMonitorUnion_put_YAxisLabel(This,bsTitle)	\
    ( (This)->lpVtbl -> put_YAxisLabel(This,bsTitle) ) 

#define _ISystemMonitorUnion_get_YAxisLabel(This,pbsTitle)	\
    ( (This)->lpVtbl -> get_YAxisLabel(This,pbsTitle) ) 

#define _ISystemMonitorUnion_CollectSample(This)	\
    ( (This)->lpVtbl -> CollectSample(This) ) 

#define _ISystemMonitorUnion_UpdateGraph(This)	\
    ( (This)->lpVtbl -> UpdateGraph(This) ) 

#define _ISystemMonitorUnion_BrowseCounters(This)	\
    ( (This)->lpVtbl -> BrowseCounters(This) ) 

#define _ISystemMonitorUnion_DisplayProperties(This)	\
    ( (This)->lpVtbl -> DisplayProperties(This) ) 

#define _ISystemMonitorUnion_Counter(This,iIndex,ppICounter)	\
    ( (This)->lpVtbl -> Counter(This,iIndex,ppICounter) ) 

#define _ISystemMonitorUnion_AddCounter(This,bsPath,ppICounter)	\
    ( (This)->lpVtbl -> AddCounter(This,bsPath,ppICounter) ) 

#define _ISystemMonitorUnion_DeleteCounter(This,pCtr)	\
    ( (This)->lpVtbl -> DeleteCounter(This,pCtr) ) 

#define _ISystemMonitorUnion_get_BackColorCtl(This,pColor)	\
    ( (This)->lpVtbl -> get_BackColorCtl(This,pColor) ) 

#define _ISystemMonitorUnion_put_BackColorCtl(This,Color)	\
    ( (This)->lpVtbl -> put_BackColorCtl(This,Color) ) 

#define _ISystemMonitorUnion_put_LogFileName(This,bsFileName)	\
    ( (This)->lpVtbl -> put_LogFileName(This,bsFileName) ) 

#define _ISystemMonitorUnion_get_LogFileName(This,bsFileName)	\
    ( (This)->lpVtbl -> get_LogFileName(This,bsFileName) ) 

#define _ISystemMonitorUnion_put_LogViewStart(This,StartTime)	\
    ( (This)->lpVtbl -> put_LogViewStart(This,StartTime) ) 

#define _ISystemMonitorUnion_get_LogViewStart(This,StartTime)	\
    ( (This)->lpVtbl -> get_LogViewStart(This,StartTime) ) 

#define _ISystemMonitorUnion_put_LogViewStop(This,StopTime)	\
    ( (This)->lpVtbl -> put_LogViewStop(This,StopTime) ) 

#define _ISystemMonitorUnion_get_LogViewStop(This,StopTime)	\
    ( (This)->lpVtbl -> get_LogViewStop(This,StopTime) ) 

#define _ISystemMonitorUnion_get_GridColor(This,pColor)	\
    ( (This)->lpVtbl -> get_GridColor(This,pColor) ) 

#define _ISystemMonitorUnion_put_GridColor(This,Color)	\
    ( (This)->lpVtbl -> put_GridColor(This,Color) ) 

#define _ISystemMonitorUnion_get_TimeBarColor(This,pColor)	\
    ( (This)->lpVtbl -> get_TimeBarColor(This,pColor) ) 

#define _ISystemMonitorUnion_put_TimeBarColor(This,Color)	\
    ( (This)->lpVtbl -> put_TimeBarColor(This,Color) ) 

#define _ISystemMonitorUnion_get_Highlight(This,pbState)	\
    ( (This)->lpVtbl -> get_Highlight(This,pbState) ) 

#define _ISystemMonitorUnion_put_Highlight(This,bState)	\
    ( (This)->lpVtbl -> put_Highlight(This,bState) ) 

#define _ISystemMonitorUnion_get_ShowToolbar(This,pbState)	\
    ( (This)->lpVtbl -> get_ShowToolbar(This,pbState) ) 

#define _ISystemMonitorUnion_put_ShowToolbar(This,bState)	\
    ( (This)->lpVtbl -> put_ShowToolbar(This,bState) ) 

#define _ISystemMonitorUnion_Paste(This)	\
    ( (This)->lpVtbl -> Paste(This) ) 

#define _ISystemMonitorUnion_Copy(This)	\
    ( (This)->lpVtbl -> Copy(This) ) 

#define _ISystemMonitorUnion_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define _ISystemMonitorUnion_put_ReadOnly(This,bState)	\
    ( (This)->lpVtbl -> put_ReadOnly(This,bState) ) 

#define _ISystemMonitorUnion_get_ReadOnly(This,pbState)	\
    ( (This)->lpVtbl -> get_ReadOnly(This,pbState) ) 

#define _ISystemMonitorUnion_put_ReportValueType(This,eReportValueType)	\
    ( (This)->lpVtbl -> put_ReportValueType(This,eReportValueType) ) 

#define _ISystemMonitorUnion_get_ReportValueType(This,peReportValueType)	\
    ( (This)->lpVtbl -> get_ReportValueType(This,peReportValueType) ) 

#define _ISystemMonitorUnion_put_MonitorDuplicateInstances(This,bState)	\
    ( (This)->lpVtbl -> put_MonitorDuplicateInstances(This,bState) ) 

#define _ISystemMonitorUnion_get_MonitorDuplicateInstances(This,pbState)	\
    ( (This)->lpVtbl -> get_MonitorDuplicateInstances(This,pbState) ) 

#define _ISystemMonitorUnion_put_DisplayFilter(This,iValue)	\
    ( (This)->lpVtbl -> put_DisplayFilter(This,iValue) ) 

#define _ISystemMonitorUnion_get_DisplayFilter(This,piValue)	\
    ( (This)->lpVtbl -> get_DisplayFilter(This,piValue) ) 

#define _ISystemMonitorUnion_get_LogFiles(This,ppILogFiles)	\
    ( (This)->lpVtbl -> get_LogFiles(This,ppILogFiles) ) 

#define _ISystemMonitorUnion_put_DataSourceType(This,eDataSourceType)	\
    ( (This)->lpVtbl -> put_DataSourceType(This,eDataSourceType) ) 

#define _ISystemMonitorUnion_get_DataSourceType(This,peDataSourceType)	\
    ( (This)->lpVtbl -> get_DataSourceType(This,peDataSourceType) ) 

#define _ISystemMonitorUnion_put_SqlDsnName(This,bsSqlDsnName)	\
    ( (This)->lpVtbl -> put_SqlDsnName(This,bsSqlDsnName) ) 

#define _ISystemMonitorUnion_get_SqlDsnName(This,bsSqlDsnName)	\
    ( (This)->lpVtbl -> get_SqlDsnName(This,bsSqlDsnName) ) 

#define _ISystemMonitorUnion_put_SqlLogSetName(This,bsSqlLogSetName)	\
    ( (This)->lpVtbl -> put_SqlLogSetName(This,bsSqlLogSetName) ) 

#define _ISystemMonitorUnion_get_SqlLogSetName(This,bsSqlLogSetName)	\
    ( (This)->lpVtbl -> get_SqlLogSetName(This,bsSqlLogSetName) ) 

#define _ISystemMonitorUnion_put_EnableDigitGrouping(This,bState)	\
    ( (This)->lpVtbl -> put_EnableDigitGrouping(This,bState) ) 

#define _ISystemMonitorUnion_get_EnableDigitGrouping(This,pbState)	\
    ( (This)->lpVtbl -> get_EnableDigitGrouping(This,pbState) ) 

#define _ISystemMonitorUnion_put_EnableToolTips(This,bState)	\
    ( (This)->lpVtbl -> put_EnableToolTips(This,bState) ) 

#define _ISystemMonitorUnion_get_EnableToolTips(This,pbState)	\
    ( (This)->lpVtbl -> get_EnableToolTips(This,pbState) ) 

#define _ISystemMonitorUnion_put_ShowTimeAxisLabels(This,bState)	\
    ( (This)->lpVtbl -> put_ShowTimeAxisLabels(This,bState) ) 

#define _ISystemMonitorUnion_get_ShowTimeAxisLabels(This,pbState)	\
    ( (This)->lpVtbl -> get_ShowTimeAxisLabels(This,pbState) ) 

#define _ISystemMonitorUnion_put_ChartScroll(This,bScroll)	\
    ( (This)->lpVtbl -> put_ChartScroll(This,bScroll) ) 

#define _ISystemMonitorUnion_get_ChartScroll(This,pbScroll)	\
    ( (This)->lpVtbl -> get_ChartScroll(This,pbScroll) ) 

#define _ISystemMonitorUnion_put_DataPointCount(This,iNewCount)	\
    ( (This)->lpVtbl -> put_DataPointCount(This,iNewCount) ) 

#define _ISystemMonitorUnion_get_DataPointCount(This,piDataPointCount)	\
    ( (This)->lpVtbl -> get_DataPointCount(This,piDataPointCount) ) 

#define _ISystemMonitorUnion_ScaleToFit(This,bSelectedCountersOnly)	\
    ( (This)->lpVtbl -> ScaleToFit(This,bSelectedCountersOnly) ) 

#define _ISystemMonitorUnion_SaveAs(This,bstrFileName,eSysmonFileType)	\
    ( (This)->lpVtbl -> SaveAs(This,bstrFileName,eSysmonFileType) ) 

#define _ISystemMonitorUnion_Relog(This,bstrFileName,eSysmonFileType,iFilter)	\
    ( (This)->lpVtbl -> Relog(This,bstrFileName,eSysmonFileType,iFilter) ) 

#define _ISystemMonitorUnion_ClearData(This)	\
    ( (This)->lpVtbl -> ClearData(This) ) 

#define _ISystemMonitorUnion_get_LogSourceStartTime(This,pDate)	\
    ( (This)->lpVtbl -> get_LogSourceStartTime(This,pDate) ) 

#define _ISystemMonitorUnion_get_LogSourceStopTime(This,pDate)	\
    ( (This)->lpVtbl -> get_LogSourceStopTime(This,pDate) ) 

#define _ISystemMonitorUnion_SetLogViewRange(This,StartTime,StopTime)	\
    ( (This)->lpVtbl -> SetLogViewRange(This,StartTime,StopTime) ) 

#define _ISystemMonitorUnion_GetLogViewRange(This,StartTime,StopTime)	\
    ( (This)->lpVtbl -> GetLogViewRange(This,StartTime,StopTime) ) 

#define _ISystemMonitorUnion_BatchingLock(This,fLock,eBatchReason)	\
    ( (This)->lpVtbl -> BatchingLock(This,fLock,eBatchReason) ) 

#define _ISystemMonitorUnion_LoadSettings(This,bstrSettingFileName)	\
    ( (This)->lpVtbl -> LoadSettings(This,bstrSettingFileName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* ___ISystemMonitorUnion_INTERFACE_DEFINED__ */


#ifndef __DISystemMonitor_DISPINTERFACE_DEFINED__
#define __DISystemMonitor_DISPINTERFACE_DEFINED__

/* dispinterface DISystemMonitor */
/* [helpstring][hidden][uuid] */ 


DEFINE_GUID(DIID_DISystemMonitor,0x13D73D81,0xC32E,0x11cf,0x93,0x98,0x00,0xAA,0x00,0xA3,0xDD,0xEA);

#if defined(__cplusplus) && !defined(CINTERFACE)

    MIDL_INTERFACE("13D73D81-C32E-11cf-9398-00AA00A3DDEA")
    DISystemMonitor : public IDispatch
    {
    };
    
#else 	/* C style interface */

    typedef struct DISystemMonitorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in DISystemMonitor * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in DISystemMonitor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in DISystemMonitor * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in DISystemMonitor * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in DISystemMonitor * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in DISystemMonitor * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            DISystemMonitor * This,
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
    } DISystemMonitorVtbl;

    interface DISystemMonitor
    {
        CONST_VTBL struct DISystemMonitorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define DISystemMonitor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define DISystemMonitor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define DISystemMonitor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define DISystemMonitor_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define DISystemMonitor_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define DISystemMonitor_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define DISystemMonitor_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */


#endif 	/* __DISystemMonitor_DISPINTERFACE_DEFINED__ */


#ifndef __DISystemMonitorInternal_DISPINTERFACE_DEFINED__
#define __DISystemMonitorInternal_DISPINTERFACE_DEFINED__

/* dispinterface DISystemMonitorInternal */
/* [helpstring][hidden][uuid] */ 


DEFINE_GUID(DIID_DISystemMonitorInternal,0x194EB242,0xC32C,0x11cf,0x93,0x98,0x00,0xAA,0x00,0xA3,0xDD,0xEA);

#if defined(__cplusplus) && !defined(CINTERFACE)

    MIDL_INTERFACE("194EB242-C32C-11cf-9398-00AA00A3DDEA")
    DISystemMonitorInternal : public IDispatch
    {
    };
    
#else 	/* C style interface */

    typedef struct DISystemMonitorInternalVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in DISystemMonitorInternal * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in DISystemMonitorInternal * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in DISystemMonitorInternal * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in DISystemMonitorInternal * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in DISystemMonitorInternal * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in DISystemMonitorInternal * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            DISystemMonitorInternal * This,
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
    } DISystemMonitorInternalVtbl;

    interface DISystemMonitorInternal
    {
        CONST_VTBL struct DISystemMonitorInternalVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define DISystemMonitorInternal_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define DISystemMonitorInternal_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define DISystemMonitorInternal_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define DISystemMonitorInternal_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define DISystemMonitorInternal_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define DISystemMonitorInternal_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define DISystemMonitorInternal_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */


#endif 	/* __DISystemMonitorInternal_DISPINTERFACE_DEFINED__ */


#ifndef __ISystemMonitorEvents_INTERFACE_DEFINED__
#define __ISystemMonitorEvents_INTERFACE_DEFINED__

/* interface ISystemMonitorEvents */
/* [object][helpstring][uuid] */ 


DEFINE_GUID(IID_ISystemMonitorEvents,0xEE660EA0,0x4ABD,0x11cf,0x94,0x3A,0x00,0x80,0x29,0x00,0x43,0x47);

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("EE660EA0-4ABD-11cf-943A-008029004347")
    ISystemMonitorEvents : public IUnknown
    {
    public:
        virtual /* [helpstring][id] */ void STDMETHODCALLTYPE OnCounterSelected( 
            /* [in] */ INT Index) = 0;
        
        virtual /* [helpstring][id] */ void STDMETHODCALLTYPE OnCounterAdded( 
            /* [in] */ INT Index) = 0;
        
        virtual /* [helpstring][id] */ void STDMETHODCALLTYPE OnCounterDeleted( 
            /* [in] */ INT Index) = 0;
        
        virtual /* [helpstring][id] */ void STDMETHODCALLTYPE OnSampleCollected( void) = 0;
        
        virtual /* [helpstring][id] */ void STDMETHODCALLTYPE OnDblClick( 
            /* [in] */ INT Index) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISystemMonitorEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISystemMonitorEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISystemMonitorEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISystemMonitorEvents * This);
        
        DECLSPEC_XFGVIRT(ISystemMonitorEvents, OnCounterSelected)
        /* [helpstring][id] */ void ( STDMETHODCALLTYPE *OnCounterSelected )( 
            __RPC__in ISystemMonitorEvents * This,
            /* [in] */ INT Index);
        
        DECLSPEC_XFGVIRT(ISystemMonitorEvents, OnCounterAdded)
        /* [helpstring][id] */ void ( STDMETHODCALLTYPE *OnCounterAdded )( 
            __RPC__in ISystemMonitorEvents * This,
            /* [in] */ INT Index);
        
        DECLSPEC_XFGVIRT(ISystemMonitorEvents, OnCounterDeleted)
        /* [helpstring][id] */ void ( STDMETHODCALLTYPE *OnCounterDeleted )( 
            __RPC__in ISystemMonitorEvents * This,
            /* [in] */ INT Index);
        
        DECLSPEC_XFGVIRT(ISystemMonitorEvents, OnSampleCollected)
        /* [helpstring][id] */ void ( STDMETHODCALLTYPE *OnSampleCollected )( 
            __RPC__in ISystemMonitorEvents * This);
        
        DECLSPEC_XFGVIRT(ISystemMonitorEvents, OnDblClick)
        /* [helpstring][id] */ void ( STDMETHODCALLTYPE *OnDblClick )( 
            __RPC__in ISystemMonitorEvents * This,
            /* [in] */ INT Index);
        
        END_INTERFACE
    } ISystemMonitorEventsVtbl;

    interface ISystemMonitorEvents
    {
        CONST_VTBL struct ISystemMonitorEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISystemMonitorEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISystemMonitorEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISystemMonitorEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISystemMonitorEvents_OnCounterSelected(This,Index)	\
    ( (This)->lpVtbl -> OnCounterSelected(This,Index) ) 

#define ISystemMonitorEvents_OnCounterAdded(This,Index)	\
    ( (This)->lpVtbl -> OnCounterAdded(This,Index) ) 

#define ISystemMonitorEvents_OnCounterDeleted(This,Index)	\
    ( (This)->lpVtbl -> OnCounterDeleted(This,Index) ) 

#define ISystemMonitorEvents_OnSampleCollected(This)	\
    ( (This)->lpVtbl -> OnSampleCollected(This) ) 

#define ISystemMonitorEvents_OnDblClick(This,Index)	\
    ( (This)->lpVtbl -> OnDblClick(This,Index) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISystemMonitorEvents_INTERFACE_DEFINED__ */


#ifndef __DISystemMonitorEvents_DISPINTERFACE_DEFINED__
#define __DISystemMonitorEvents_DISPINTERFACE_DEFINED__

/* dispinterface DISystemMonitorEvents */
/* [helpstring][uuid] */ 


DEFINE_GUID(DIID_DISystemMonitorEvents,0x84979930,0x4AB3,0x11cf,0x94,0x3A,0x00,0x80,0x29,0x00,0x43,0x47);

#if defined(__cplusplus) && !defined(CINTERFACE)

    MIDL_INTERFACE("84979930-4AB3-11cf-943A-008029004347")
    DISystemMonitorEvents : public IDispatch
    {
    };
    
#else 	/* C style interface */

    typedef struct DISystemMonitorEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in DISystemMonitorEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in DISystemMonitorEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in DISystemMonitorEvents * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in DISystemMonitorEvents * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in DISystemMonitorEvents * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in DISystemMonitorEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            DISystemMonitorEvents * This,
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
    } DISystemMonitorEventsVtbl;

    interface DISystemMonitorEvents
    {
        CONST_VTBL struct DISystemMonitorEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define DISystemMonitorEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define DISystemMonitorEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define DISystemMonitorEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define DISystemMonitorEvents_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define DISystemMonitorEvents_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define DISystemMonitorEvents_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define DISystemMonitorEvents_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */


#endif 	/* __DISystemMonitorEvents_DISPINTERFACE_DEFINED__ */


DEFINE_GUID(CLSID_SystemMonitor,0xC4D2D8E0,0xD1DD,0x11ce,0x94,0x0F,0x00,0x80,0x29,0x00,0x43,0x47);

#ifdef __cplusplus

class DECLSPEC_UUID("C4D2D8E0-D1DD-11ce-940F-008029004347")
SystemMonitor;
#endif

DEFINE_GUID(CLSID_CounterItem,0xC4D2D8E0,0xD1DD,0x11ce,0x94,0x0F,0x00,0x80,0x29,0x00,0x43,0x48);

#ifdef __cplusplus

class DECLSPEC_UUID("C4D2D8E0-D1DD-11ce-940F-008029004348")
CounterItem;
#endif

DEFINE_GUID(CLSID_Counters,0xB2B066D2,0x2AAC,0x11cf,0x94,0x2F,0x00,0x80,0x29,0x00,0x43,0x47);

#ifdef __cplusplus

class DECLSPEC_UUID("B2B066D2-2AAC-11cf-942F-008029004347")
Counters;
#endif

DEFINE_GUID(CLSID_LogFileItem,0x16EC5BE8,0xDF93,0x4237,0x94,0xE4,0x9E,0xE9,0x18,0x11,0x1D,0x71);

#ifdef __cplusplus

class DECLSPEC_UUID("16EC5BE8-DF93-4237-94E4-9EE918111D71")
LogFileItem;
#endif

DEFINE_GUID(CLSID_LogFiles,0x2735D9FD,0xF6B9,0x4f19,0xA5,0xD9,0xE2,0xD0,0x68,0x58,0x4B,0xC5);

#ifdef __cplusplus

class DECLSPEC_UUID("2735D9FD-F6B9-4f19-A5D9-E2D068584BC5")
LogFiles;
#endif

DEFINE_GUID(CLSID_CounterItem2,0x43196c62,0xc31f,0x4ce3,0xa0,0x2e,0x79,0xef,0xe0,0xf6,0xa5,0x25);

#ifdef __cplusplus

class DECLSPEC_UUID("43196c62-c31f-4ce3-a02e-79efe0f6a525")
CounterItem2;
#endif

DEFINE_GUID(CLSID_SystemMonitor2,0x7f30578c,0x5f38,0x4612,0xac,0xfe,0x6e,0xd0,0x4c,0x7b,0x7a,0xf8);

#ifdef __cplusplus

class DECLSPEC_UUID("7f30578c-5f38-4612-acfe-6ed04c7b7af8")
SystemMonitor2;
#endif

DEFINE_GUID(CLSID_AppearPropPage,0xe49741e9,0x93a8,0x4ab1,0x8e,0x96,0xbf,0x44,0x82,0x28,0x2e,0x9c);

#ifdef __cplusplus

class DECLSPEC_UUID("e49741e9-93a8-4ab1-8e96-bf4482282e9c")
AppearPropPage;
#endif

DEFINE_GUID(CLSID_GeneralPropPage,0xC3E5D3D2,0x1A03,0x11CF,0x94,0x2D,0x00,0x80,0x29,0x00,0x43,0x47);

#ifdef __cplusplus

class DECLSPEC_UUID("C3E5D3D2-1A03-11CF-942D-008029004347")
GeneralPropPage;
#endif

DEFINE_GUID(CLSID_GraphPropPage,0xC3E5D3D3,0x1A03,0x11CF,0x94,0x2D,0x00,0x80,0x29,0x00,0x43,0x47);

#ifdef __cplusplus

class DECLSPEC_UUID("C3E5D3D3-1A03-11CF-942D-008029004347")
GraphPropPage;
#endif

DEFINE_GUID(CLSID_SourcePropPage,0x0CF32AA1,0x7571,0x11D0,0x93,0xC4,0x00,0xAA,0x00,0xA3,0xDD,0xEA);

#ifdef __cplusplus

class DECLSPEC_UUID("0CF32AA1-7571-11D0-93C4-00AA00A3DDEA")
SourcePropPage;
#endif

DEFINE_GUID(CLSID_CounterPropPage,0xCF948561,0xEDE8,0x11CE,0x94,0x1E,0x00,0x80,0x29,0x00,0x43,0x47);

#ifdef __cplusplus

class DECLSPEC_UUID("CF948561-EDE8-11CE-941E-008029004347")
CounterPropPage;
#endif
#endif /* __SystemMonitor_LIBRARY_DEFINED__ */

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


