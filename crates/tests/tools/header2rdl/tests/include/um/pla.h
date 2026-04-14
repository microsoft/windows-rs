

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


#ifndef __pla_h__
#define __pla_h__

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

#ifndef __IDataCollectorSet_FWD_DEFINED__
#define __IDataCollectorSet_FWD_DEFINED__
typedef interface IDataCollectorSet IDataCollectorSet;

#endif 	/* __IDataCollectorSet_FWD_DEFINED__ */


#ifndef __IDataManager_FWD_DEFINED__
#define __IDataManager_FWD_DEFINED__
typedef interface IDataManager IDataManager;

#endif 	/* __IDataManager_FWD_DEFINED__ */


#ifndef __IFolderAction_FWD_DEFINED__
#define __IFolderAction_FWD_DEFINED__
typedef interface IFolderAction IFolderAction;

#endif 	/* __IFolderAction_FWD_DEFINED__ */


#ifndef __IFolderActionCollection_FWD_DEFINED__
#define __IFolderActionCollection_FWD_DEFINED__
typedef interface IFolderActionCollection IFolderActionCollection;

#endif 	/* __IFolderActionCollection_FWD_DEFINED__ */


#ifndef __IDataCollector_FWD_DEFINED__
#define __IDataCollector_FWD_DEFINED__
typedef interface IDataCollector IDataCollector;

#endif 	/* __IDataCollector_FWD_DEFINED__ */


#ifndef __IPerformanceCounterDataCollector_FWD_DEFINED__
#define __IPerformanceCounterDataCollector_FWD_DEFINED__
typedef interface IPerformanceCounterDataCollector IPerformanceCounterDataCollector;

#endif 	/* __IPerformanceCounterDataCollector_FWD_DEFINED__ */


#ifndef __ITraceDataCollector_FWD_DEFINED__
#define __ITraceDataCollector_FWD_DEFINED__
typedef interface ITraceDataCollector ITraceDataCollector;

#endif 	/* __ITraceDataCollector_FWD_DEFINED__ */


#ifndef __IConfigurationDataCollector_FWD_DEFINED__
#define __IConfigurationDataCollector_FWD_DEFINED__
typedef interface IConfigurationDataCollector IConfigurationDataCollector;

#endif 	/* __IConfigurationDataCollector_FWD_DEFINED__ */


#ifndef __IAlertDataCollector_FWD_DEFINED__
#define __IAlertDataCollector_FWD_DEFINED__
typedef interface IAlertDataCollector IAlertDataCollector;

#endif 	/* __IAlertDataCollector_FWD_DEFINED__ */


#ifndef __IApiTracingDataCollector_FWD_DEFINED__
#define __IApiTracingDataCollector_FWD_DEFINED__
typedef interface IApiTracingDataCollector IApiTracingDataCollector;

#endif 	/* __IApiTracingDataCollector_FWD_DEFINED__ */


#ifndef __IDataCollectorCollection_FWD_DEFINED__
#define __IDataCollectorCollection_FWD_DEFINED__
typedef interface IDataCollectorCollection IDataCollectorCollection;

#endif 	/* __IDataCollectorCollection_FWD_DEFINED__ */


#ifndef __IDataCollectorSetCollection_FWD_DEFINED__
#define __IDataCollectorSetCollection_FWD_DEFINED__
typedef interface IDataCollectorSetCollection IDataCollectorSetCollection;

#endif 	/* __IDataCollectorSetCollection_FWD_DEFINED__ */


#ifndef __ITraceDataProvider_FWD_DEFINED__
#define __ITraceDataProvider_FWD_DEFINED__
typedef interface ITraceDataProvider ITraceDataProvider;

#endif 	/* __ITraceDataProvider_FWD_DEFINED__ */


#ifndef __ITraceDataProviderCollection_FWD_DEFINED__
#define __ITraceDataProviderCollection_FWD_DEFINED__
typedef interface ITraceDataProviderCollection ITraceDataProviderCollection;

#endif 	/* __ITraceDataProviderCollection_FWD_DEFINED__ */


#ifndef __ISchedule_FWD_DEFINED__
#define __ISchedule_FWD_DEFINED__
typedef interface ISchedule ISchedule;

#endif 	/* __ISchedule_FWD_DEFINED__ */


#ifndef __IScheduleCollection_FWD_DEFINED__
#define __IScheduleCollection_FWD_DEFINED__
typedef interface IScheduleCollection IScheduleCollection;

#endif 	/* __IScheduleCollection_FWD_DEFINED__ */


#ifndef __IValueMapItem_FWD_DEFINED__
#define __IValueMapItem_FWD_DEFINED__
typedef interface IValueMapItem IValueMapItem;

#endif 	/* __IValueMapItem_FWD_DEFINED__ */


#ifndef __IValueMap_FWD_DEFINED__
#define __IValueMap_FWD_DEFINED__
typedef interface IValueMap IValueMap;

#endif 	/* __IValueMap_FWD_DEFINED__ */


#ifndef __DataCollectorSet_FWD_DEFINED__
#define __DataCollectorSet_FWD_DEFINED__

#ifdef __cplusplus
typedef class DataCollectorSet DataCollectorSet;
#else
typedef struct DataCollectorSet DataCollectorSet;
#endif /* __cplusplus */

#endif 	/* __DataCollectorSet_FWD_DEFINED__ */


#ifndef __TraceSession_FWD_DEFINED__
#define __TraceSession_FWD_DEFINED__

#ifdef __cplusplus
typedef class TraceSession TraceSession;
#else
typedef struct TraceSession TraceSession;
#endif /* __cplusplus */

#endif 	/* __TraceSession_FWD_DEFINED__ */


#ifndef __TraceSessionCollection_FWD_DEFINED__
#define __TraceSessionCollection_FWD_DEFINED__

#ifdef __cplusplus
typedef class TraceSessionCollection TraceSessionCollection;
#else
typedef struct TraceSessionCollection TraceSessionCollection;
#endif /* __cplusplus */

#endif 	/* __TraceSessionCollection_FWD_DEFINED__ */


#ifndef __TraceDataProvider_FWD_DEFINED__
#define __TraceDataProvider_FWD_DEFINED__

#ifdef __cplusplus
typedef class TraceDataProvider TraceDataProvider;
#else
typedef struct TraceDataProvider TraceDataProvider;
#endif /* __cplusplus */

#endif 	/* __TraceDataProvider_FWD_DEFINED__ */


#ifndef __TraceDataProviderCollection_FWD_DEFINED__
#define __TraceDataProviderCollection_FWD_DEFINED__

#ifdef __cplusplus
typedef class TraceDataProviderCollection TraceDataProviderCollection;
#else
typedef struct TraceDataProviderCollection TraceDataProviderCollection;
#endif /* __cplusplus */

#endif 	/* __TraceDataProviderCollection_FWD_DEFINED__ */


#ifndef __DataCollectorSetCollection_FWD_DEFINED__
#define __DataCollectorSetCollection_FWD_DEFINED__

#ifdef __cplusplus
typedef class DataCollectorSetCollection DataCollectorSetCollection;
#else
typedef struct DataCollectorSetCollection DataCollectorSetCollection;
#endif /* __cplusplus */

#endif 	/* __DataCollectorSetCollection_FWD_DEFINED__ */


#ifndef __LegacyDataCollectorSet_FWD_DEFINED__
#define __LegacyDataCollectorSet_FWD_DEFINED__

#ifdef __cplusplus
typedef class LegacyDataCollectorSet LegacyDataCollectorSet;
#else
typedef struct LegacyDataCollectorSet LegacyDataCollectorSet;
#endif /* __cplusplus */

#endif 	/* __LegacyDataCollectorSet_FWD_DEFINED__ */


#ifndef __LegacyDataCollectorSetCollection_FWD_DEFINED__
#define __LegacyDataCollectorSetCollection_FWD_DEFINED__

#ifdef __cplusplus
typedef class LegacyDataCollectorSetCollection LegacyDataCollectorSetCollection;
#else
typedef struct LegacyDataCollectorSetCollection LegacyDataCollectorSetCollection;
#endif /* __cplusplus */

#endif 	/* __LegacyDataCollectorSetCollection_FWD_DEFINED__ */


#ifndef __LegacyTraceSession_FWD_DEFINED__
#define __LegacyTraceSession_FWD_DEFINED__

#ifdef __cplusplus
typedef class LegacyTraceSession LegacyTraceSession;
#else
typedef struct LegacyTraceSession LegacyTraceSession;
#endif /* __cplusplus */

#endif 	/* __LegacyTraceSession_FWD_DEFINED__ */


#ifndef __LegacyTraceSessionCollection_FWD_DEFINED__
#define __LegacyTraceSessionCollection_FWD_DEFINED__

#ifdef __cplusplus
typedef class LegacyTraceSessionCollection LegacyTraceSessionCollection;
#else
typedef struct LegacyTraceSessionCollection LegacyTraceSessionCollection;
#endif /* __cplusplus */

#endif 	/* __LegacyTraceSessionCollection_FWD_DEFINED__ */


#ifndef __ServerDataCollectorSet_FWD_DEFINED__
#define __ServerDataCollectorSet_FWD_DEFINED__

#ifdef __cplusplus
typedef class ServerDataCollectorSet ServerDataCollectorSet;
#else
typedef struct ServerDataCollectorSet ServerDataCollectorSet;
#endif /* __cplusplus */

#endif 	/* __ServerDataCollectorSet_FWD_DEFINED__ */


#ifndef __ServerDataCollectorSetCollection_FWD_DEFINED__
#define __ServerDataCollectorSetCollection_FWD_DEFINED__

#ifdef __cplusplus
typedef class ServerDataCollectorSetCollection ServerDataCollectorSetCollection;
#else
typedef struct ServerDataCollectorSetCollection ServerDataCollectorSetCollection;
#endif /* __cplusplus */

#endif 	/* __ServerDataCollectorSetCollection_FWD_DEFINED__ */


#ifndef __SystemDataCollectorSet_FWD_DEFINED__
#define __SystemDataCollectorSet_FWD_DEFINED__

#ifdef __cplusplus
typedef class SystemDataCollectorSet SystemDataCollectorSet;
#else
typedef struct SystemDataCollectorSet SystemDataCollectorSet;
#endif /* __cplusplus */

#endif 	/* __SystemDataCollectorSet_FWD_DEFINED__ */


#ifndef __SystemDataCollectorSetCollection_FWD_DEFINED__
#define __SystemDataCollectorSetCollection_FWD_DEFINED__

#ifdef __cplusplus
typedef class SystemDataCollectorSetCollection SystemDataCollectorSetCollection;
#else
typedef struct SystemDataCollectorSetCollection SystemDataCollectorSetCollection;
#endif /* __cplusplus */

#endif 	/* __SystemDataCollectorSetCollection_FWD_DEFINED__ */


#ifndef __BootTraceSession_FWD_DEFINED__
#define __BootTraceSession_FWD_DEFINED__

#ifdef __cplusplus
typedef class BootTraceSession BootTraceSession;
#else
typedef struct BootTraceSession BootTraceSession;
#endif /* __cplusplus */

#endif 	/* __BootTraceSession_FWD_DEFINED__ */


#ifndef __BootTraceSessionCollection_FWD_DEFINED__
#define __BootTraceSessionCollection_FWD_DEFINED__

#ifdef __cplusplus
typedef class BootTraceSessionCollection BootTraceSessionCollection;
#else
typedef struct BootTraceSessionCollection BootTraceSessionCollection;
#endif /* __cplusplus */

#endif 	/* __BootTraceSessionCollection_FWD_DEFINED__ */


/* header files for imported files */
#include "unknwn.h"
#include "oaidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_pla_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_pla_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_pla_0000_0000_v0_0_s_ifspec;


#ifndef __PlaLibrary_LIBRARY_DEFINED__
#define __PlaLibrary_LIBRARY_DEFINED__

/* library PlaLibrary */
/* [control][helpstring][version][uuid] */ 



















typedef /* [public][public][public][uuid] */  DECLSPEC_UUID("03837504-098b-11d8-9414-505054503030") 
enum __MIDL___MIDL_itf_pla_0001_0043_0001
    {
        plaPerformanceCounter	= 0,
        plaTrace	= 1,
        plaConfiguration	= 2,
        plaAlert	= 3,
        plaApiTrace	= 4
    } 	DataCollectorType;

typedef /* [public][public][public][uuid] */  DECLSPEC_UUID("03837507-098b-11d8-9414-505054503030") 
enum __MIDL___MIDL_itf_pla_0001_0043_0002
    {
        plaCommaSeparated	= 0,
        plaTabSeparated	= 1,
        plaSql	= 2,
        plaBinary	= 3
    } 	FileFormat;

typedef /* [public][public][public][public][public][uuid] */  DECLSPEC_UUID("03837508-098b-11d8-9414-505054503030") 
enum __MIDL___MIDL_itf_pla_0001_0043_0003
    {
        plaNone	= 0,
        plaPattern	= 0x1,
        plaComputer	= 0x2,
        plaMonthDayHour	= 0x100,
        plaSerialNumber	= 0x200,
        plaYearDayOfYear	= 0x400,
        plaYearMonth	= 0x800,
        plaYearMonthDay	= 0x1000,
        plaYearMonthDayHour	= 0x2000,
        plaMonthDayHourMinute	= 0x4000
    } 	AutoPathFormat;

typedef /* [public][public][uuid] */  DECLSPEC_UUID("0383750a-098b-11d8-9414-505054503030") 
enum __MIDL___MIDL_itf_pla_0001_0043_0004
    {
        plaStopped	= 0,
        plaRunning	= 1,
        plaCompiling	= 2,
        plaPending	= 3,
        plaUndefined	= 4
    } 	DataCollectorSetStatus;

typedef /* [public][public][public][uuid] */  DECLSPEC_UUID("0383750d-098b-11d8-9414-505054503030") 
enum __MIDL___MIDL_itf_pla_0001_0043_0005
    {
        plaTimeStamp	= 0,
        plaPerformance	= 1,
        plaSystem	= 2,
        plaCycle	= 3
    } 	ClockType;

typedef /* [public][public][public][uuid] */  DECLSPEC_UUID("0383750e-098b-11d8-9414-505054503030") 
enum __MIDL___MIDL_itf_pla_0001_0043_0006
    {
        plaFile	= 0x1,
        plaRealTime	= 0x2,
        plaBoth	= 0x3,
        plaBuffering	= 0x4
    } 	StreamMode;

typedef /* [public][public][uuid] */  DECLSPEC_UUID("0383751f-098b-11d8-9414-505054503030") 
enum __MIDL___MIDL_itf_pla_0001_0043_0007
    {
        plaCreateNew	= 0x1,
        plaModify	= 0x2,
        plaCreateOrModify	= 0x3,
        plaUpdateRunningInstance	= 0x10,
        plaFlushTrace	= 0x20,
        plaValidateOnly	= 0x1000
    } 	CommitMode;

typedef /* [public][public][public][public][public][uuid] */  DECLSPEC_UUID("03837535-098b-11d8-9414-505054503030") 
enum __MIDL___MIDL_itf_pla_0001_0043_0008
    {
        plaIndex	= 1,
        plaFlag	= 2,
        plaFlagArray	= 3,
        plaValidation	= 4
    } 	ValueMapType;

typedef /* [public][public][public][uuid] */  DECLSPEC_UUID("0383753b-098b-11d8-9414-505054503030") 
enum __MIDL___MIDL_itf_pla_0001_0043_0009
    {
        plaRunOnce	= 0,
        plaSunday	= 0x1,
        plaMonday	= 0x2,
        plaTuesday	= 0x4,
        plaWednesday	= 0x8,
        plaThursday	= 0x10,
        plaFriday	= 0x20,
        plaSaturday	= 0x40,
        plaEveryday	= 0x7f
    } 	WeekDays;

typedef /* [public][public][public][uuid] */  DECLSPEC_UUID("0383753f-098b-11d8-9414-505054503030") 
enum __MIDL___MIDL_itf_pla_0001_0043_0010
    {
        plaDeleteLargest	= 0,
        plaDeleteOldest	= 1
    } 	ResourcePolicy;

typedef /* [public][public][uuid] */  DECLSPEC_UUID("03837540-098b-11d8-9414-505054503030") 
enum __MIDL___MIDL_itf_pla_0001_0043_0011
    {
        plaCreateReport	= 0x1,
        plaRunRules	= 0x2,
        plaCreateHtml	= 0x4,
        plaFolderActions	= 0x8,
        plaResourceFreeing	= 0x10
    } 	DataManagerSteps;

typedef /* [public][public][public][uuid] */  DECLSPEC_UUID("03837542-098b-11d8-9414-505054503030") 
enum __MIDL___MIDL_itf_pla_0001_0043_0012
    {
        plaCreateCab	= 0x1,
        plaDeleteData	= 0x2,
        plaSendCab	= 0x4,
        plaDeleteCab	= 0x8,
        plaDeleteReport	= 0x10
    } 	FolderActionSteps;

#define PLA_FUNCTION    HRESULT __stdcall

PLA_FUNCTION
PlaExpandTaskArguments(
    VARIANT vDataSet,
    _Out_ BSTR* args
    );

#define PLA_CAPABILITY_LOCAL                0x10000000
#define PLA_CAPABILITY_V1_SVC               0x00000001
#define PLA_CAPABILITY_V1_SESSION           0x00000002
#define PLA_CAPABILITY_V1_SYSTEM            0x00000004
#define PLA_CAPABILITY_LEGACY_SESSION       0x00000008
#define PLA_CAPABILITY_LEGACY_SVC           0x00000010
#define PLA_CAPABILITY_AUTOLOGGER           0x00000020

#define PLAL_ALERT_CMD_LINE_SINGLE    ((DWORD)0x00000100)
#define PLAL_ALERT_CMD_LINE_A_NAME    ((DWORD)0x00000200)
#define PLAL_ALERT_CMD_LINE_C_NAME    ((DWORD)0x00000400)
#define PLAL_ALERT_CMD_LINE_D_TIME    ((DWORD)0x00000800)
#define PLAL_ALERT_CMD_LINE_L_VAL     ((DWORD)0x00001000)
#define PLAL_ALERT_CMD_LINE_M_VAL     ((DWORD)0x00002000)
#define PLAL_ALERT_CMD_LINE_U_TEXT    ((DWORD)0x00004000)
#define PLAL_ALERT_CMD_LINE_MASK      ((DWORD)0x00007F00)

PLA_FUNCTION
PlaGetServerCapabilities(
    _In_opt_ BSTR   Server,
    _Out_    PDWORD Capabilites
    );

PLA_FUNCTION
PlaGetLegacyAlertActionsStringFromFlags(
    _In_  DWORD  dwFlags,
    _Out_ BSTR   *pbstrAlertStr
    );

PLA_FUNCTION
PlaGetLegacyAlertActionsFlagsFromString(
    _In_  PCWSTR  pszArguments,
    _Out_ LPDWORD pdwFlags
    );

typedef VOID (*PLA_CABEXTRACT_CALLBACK)(PCWSTR FileName, PVOID Context);

HRESULT
PlaExtractCabinet(
    _In_     PCWSTR                  CabFileName,
    _In_     PCWSTR                  DestPath,
    _In_opt_ PLA_CABEXTRACT_CALLBACK Callback,
    _In_opt_ PVOID                   Context
    );

HRESULT
PlaDeleteReport(
    _In_ PCWSTR Folder
    );


EXTERN_C const IID LIBID_PlaLibrary;

#ifndef __IDataCollectorSet_INTERFACE_DEFINED__
#define __IDataCollectorSet_INTERFACE_DEFINED__

/* interface IDataCollectorSet */
/* [oleautomation][dual][uuid][object] */ 


EXTERN_C const IID IID_IDataCollectorSet;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("03837520-098b-11d8-9414-505054503030")
    IDataCollectorSet : public IDispatch
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_DataCollectors( 
            /* [retval][out] */ __RPC__deref_out_opt IDataCollectorCollection **collectors) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Duration( 
            /* [retval][out] */ __RPC__out unsigned long *seconds) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_Duration( 
            /* [in] */ unsigned long seconds) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Description( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *description) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_Description( 
            /* [in] */ __RPC__in BSTR description) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_DescriptionUnresolved( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Descr) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_DisplayName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *DisplayName) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_DisplayName( 
            /* [in] */ __RPC__in BSTR DisplayName) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_DisplayNameUnresolved( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *name) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Keywords( 
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *keywords) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_Keywords( 
            /* [in] */ __RPC__in SAFEARRAY * keywords) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_LatestOutputLocation( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *path) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_LatestOutputLocation( 
            /* [in] */ __RPC__in BSTR path) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *name) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_OutputLocation( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *path) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_RootPath( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *folder) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_RootPath( 
            /* [in] */ __RPC__in BSTR folder) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Segment( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *segment) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_Segment( 
            /* [in] */ VARIANT_BOOL segment) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_SegmentMaxDuration( 
            /* [retval][out] */ __RPC__out unsigned long *seconds) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_SegmentMaxDuration( 
            /* [in] */ unsigned long seconds) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_SegmentMaxSize( 
            /* [retval][out] */ __RPC__out unsigned long *size) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_SegmentMaxSize( 
            /* [in] */ unsigned long size) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_SerialNumber( 
            /* [retval][out] */ __RPC__out unsigned long *index) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_SerialNumber( 
            /* [in] */ unsigned long index) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Server( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *server) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Status( 
            /* [retval][out] */ __RPC__out DataCollectorSetStatus *status) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Subdirectory( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *folder) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_Subdirectory( 
            /* [in] */ __RPC__in BSTR folder) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_SubdirectoryFormat( 
            /* [retval][out] */ __RPC__out AutoPathFormat *format) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_SubdirectoryFormat( 
            /* [in] */ AutoPathFormat format) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_SubdirectoryFormatPattern( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pattern) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_SubdirectoryFormatPattern( 
            /* [in] */ __RPC__in BSTR pattern) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Task( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *task) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_Task( 
            /* [in] */ __RPC__in BSTR task) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_TaskRunAsSelf( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *RunAsSelf) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_TaskRunAsSelf( 
            /* [in] */ VARIANT_BOOL RunAsSelf) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_TaskArguments( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *task) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_TaskArguments( 
            /* [in] */ __RPC__in BSTR task) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_TaskUserTextArguments( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *UserText) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_TaskUserTextArguments( 
            /* [in] */ __RPC__in BSTR UserText) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Schedules( 
            /* [retval][out] */ __RPC__deref_out_opt IScheduleCollection **ppSchedules) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_SchedulesEnabled( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *enabled) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_SchedulesEnabled( 
            /* [in] */ VARIANT_BOOL enabled) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_UserAccount( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *user) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Xml( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *xml) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Security( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrSecurity) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_Security( 
            /* [in] */ __RPC__in BSTR bstrSecurity) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_StopOnCompletion( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Stop) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_StopOnCompletion( 
            /* [in] */ VARIANT_BOOL Stop) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_DataManager( 
            /* [retval][out] */ __RPC__deref_out_opt IDataManager **DataManager) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetCredentials( 
            __RPC__in BSTR user,
            __RPC__in BSTR password) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Query( 
            /* [in] */ __RPC__in BSTR name,
            /* [unique][in] */ __RPC__in_opt BSTR server) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Commit( 
            /* [in] */ __RPC__in BSTR name,
            /* [unique][in] */ __RPC__in_opt BSTR server,
            CommitMode mode,
            /* [retval][out] */ __RPC__deref_out_opt IValueMap **validation) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Delete( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Start( 
            /* [in] */ VARIANT_BOOL Synchronous) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Stop( 
            /* [in] */ VARIANT_BOOL Synchronous) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetXml( 
            /* [in] */ __RPC__in BSTR xml,
            /* [retval][out] */ __RPC__deref_out_opt IValueMap **validation) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetValue( 
            __RPC__in BSTR key,
            __RPC__in BSTR value) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetValue( 
            __RPC__in BSTR key,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *value) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDataCollectorSetVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDataCollectorSet * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDataCollectorSet * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDataCollectorSet * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IDataCollectorSet * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IDataCollectorSet * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IDataCollectorSet * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IDataCollectorSet * This,
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
        
        DECLSPEC_XFGVIRT(IDataCollectorSet, get_DataCollectors)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_DataCollectors )( 
            __RPC__in IDataCollectorSet * This,
            /* [retval][out] */ __RPC__deref_out_opt IDataCollectorCollection **collectors);
        
        DECLSPEC_XFGVIRT(IDataCollectorSet, get_Duration)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Duration )( 
            __RPC__in IDataCollectorSet * This,
            /* [retval][out] */ __RPC__out unsigned long *seconds);
        
        DECLSPEC_XFGVIRT(IDataCollectorSet, put_Duration)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Duration )( 
            __RPC__in IDataCollectorSet * This,
            /* [in] */ unsigned long seconds);
        
        DECLSPEC_XFGVIRT(IDataCollectorSet, get_Description)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in IDataCollectorSet * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *description);
        
        DECLSPEC_XFGVIRT(IDataCollectorSet, put_Description)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Description )( 
            __RPC__in IDataCollectorSet * This,
            /* [in] */ __RPC__in BSTR description);
        
        DECLSPEC_XFGVIRT(IDataCollectorSet, get_DescriptionUnresolved)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_DescriptionUnresolved )( 
            __RPC__in IDataCollectorSet * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Descr);
        
        DECLSPEC_XFGVIRT(IDataCollectorSet, get_DisplayName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_DisplayName )( 
            __RPC__in IDataCollectorSet * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *DisplayName);
        
        DECLSPEC_XFGVIRT(IDataCollectorSet, put_DisplayName)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_DisplayName )( 
            __RPC__in IDataCollectorSet * This,
            /* [in] */ __RPC__in BSTR DisplayName);
        
        DECLSPEC_XFGVIRT(IDataCollectorSet, get_DisplayNameUnresolved)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_DisplayNameUnresolved )( 
            __RPC__in IDataCollectorSet * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *name);
        
        DECLSPEC_XFGVIRT(IDataCollectorSet, get_Keywords)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Keywords )( 
            __RPC__in IDataCollectorSet * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *keywords);
        
        DECLSPEC_XFGVIRT(IDataCollectorSet, put_Keywords)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Keywords )( 
            __RPC__in IDataCollectorSet * This,
            /* [in] */ __RPC__in SAFEARRAY * keywords);
        
        DECLSPEC_XFGVIRT(IDataCollectorSet, get_LatestOutputLocation)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_LatestOutputLocation )( 
            __RPC__in IDataCollectorSet * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *path);
        
        DECLSPEC_XFGVIRT(IDataCollectorSet, put_LatestOutputLocation)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_LatestOutputLocation )( 
            __RPC__in IDataCollectorSet * This,
            /* [in] */ __RPC__in BSTR path);
        
        DECLSPEC_XFGVIRT(IDataCollectorSet, get_Name)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IDataCollectorSet * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *name);
        
        DECLSPEC_XFGVIRT(IDataCollectorSet, get_OutputLocation)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_OutputLocation )( 
            __RPC__in IDataCollectorSet * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *path);
        
        DECLSPEC_XFGVIRT(IDataCollectorSet, get_RootPath)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RootPath )( 
            __RPC__in IDataCollectorSet * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *folder);
        
        DECLSPEC_XFGVIRT(IDataCollectorSet, put_RootPath)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_RootPath )( 
            __RPC__in IDataCollectorSet * This,
            /* [in] */ __RPC__in BSTR folder);
        
        DECLSPEC_XFGVIRT(IDataCollectorSet, get_Segment)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Segment )( 
            __RPC__in IDataCollectorSet * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *segment);
        
        DECLSPEC_XFGVIRT(IDataCollectorSet, put_Segment)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Segment )( 
            __RPC__in IDataCollectorSet * This,
            /* [in] */ VARIANT_BOOL segment);
        
        DECLSPEC_XFGVIRT(IDataCollectorSet, get_SegmentMaxDuration)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_SegmentMaxDuration )( 
            __RPC__in IDataCollectorSet * This,
            /* [retval][out] */ __RPC__out unsigned long *seconds);
        
        DECLSPEC_XFGVIRT(IDataCollectorSet, put_SegmentMaxDuration)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_SegmentMaxDuration )( 
            __RPC__in IDataCollectorSet * This,
            /* [in] */ unsigned long seconds);
        
        DECLSPEC_XFGVIRT(IDataCollectorSet, get_SegmentMaxSize)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_SegmentMaxSize )( 
            __RPC__in IDataCollectorSet * This,
            /* [retval][out] */ __RPC__out unsigned long *size);
        
        DECLSPEC_XFGVIRT(IDataCollectorSet, put_SegmentMaxSize)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_SegmentMaxSize )( 
            __RPC__in IDataCollectorSet * This,
            /* [in] */ unsigned long size);
        
        DECLSPEC_XFGVIRT(IDataCollectorSet, get_SerialNumber)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_SerialNumber )( 
            __RPC__in IDataCollectorSet * This,
            /* [retval][out] */ __RPC__out unsigned long *index);
        
        DECLSPEC_XFGVIRT(IDataCollectorSet, put_SerialNumber)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_SerialNumber )( 
            __RPC__in IDataCollectorSet * This,
            /* [in] */ unsigned long index);
        
        DECLSPEC_XFGVIRT(IDataCollectorSet, get_Server)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Server )( 
            __RPC__in IDataCollectorSet * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *server);
        
        DECLSPEC_XFGVIRT(IDataCollectorSet, get_Status)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Status )( 
            __RPC__in IDataCollectorSet * This,
            /* [retval][out] */ __RPC__out DataCollectorSetStatus *status);
        
        DECLSPEC_XFGVIRT(IDataCollectorSet, get_Subdirectory)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Subdirectory )( 
            __RPC__in IDataCollectorSet * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *folder);
        
        DECLSPEC_XFGVIRT(IDataCollectorSet, put_Subdirectory)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Subdirectory )( 
            __RPC__in IDataCollectorSet * This,
            /* [in] */ __RPC__in BSTR folder);
        
        DECLSPEC_XFGVIRT(IDataCollectorSet, get_SubdirectoryFormat)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_SubdirectoryFormat )( 
            __RPC__in IDataCollectorSet * This,
            /* [retval][out] */ __RPC__out AutoPathFormat *format);
        
        DECLSPEC_XFGVIRT(IDataCollectorSet, put_SubdirectoryFormat)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_SubdirectoryFormat )( 
            __RPC__in IDataCollectorSet * This,
            /* [in] */ AutoPathFormat format);
        
        DECLSPEC_XFGVIRT(IDataCollectorSet, get_SubdirectoryFormatPattern)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_SubdirectoryFormatPattern )( 
            __RPC__in IDataCollectorSet * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pattern);
        
        DECLSPEC_XFGVIRT(IDataCollectorSet, put_SubdirectoryFormatPattern)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_SubdirectoryFormatPattern )( 
            __RPC__in IDataCollectorSet * This,
            /* [in] */ __RPC__in BSTR pattern);
        
        DECLSPEC_XFGVIRT(IDataCollectorSet, get_Task)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Task )( 
            __RPC__in IDataCollectorSet * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *task);
        
        DECLSPEC_XFGVIRT(IDataCollectorSet, put_Task)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Task )( 
            __RPC__in IDataCollectorSet * This,
            /* [in] */ __RPC__in BSTR task);
        
        DECLSPEC_XFGVIRT(IDataCollectorSet, get_TaskRunAsSelf)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_TaskRunAsSelf )( 
            __RPC__in IDataCollectorSet * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *RunAsSelf);
        
        DECLSPEC_XFGVIRT(IDataCollectorSet, put_TaskRunAsSelf)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_TaskRunAsSelf )( 
            __RPC__in IDataCollectorSet * This,
            /* [in] */ VARIANT_BOOL RunAsSelf);
        
        DECLSPEC_XFGVIRT(IDataCollectorSet, get_TaskArguments)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_TaskArguments )( 
            __RPC__in IDataCollectorSet * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *task);
        
        DECLSPEC_XFGVIRT(IDataCollectorSet, put_TaskArguments)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_TaskArguments )( 
            __RPC__in IDataCollectorSet * This,
            /* [in] */ __RPC__in BSTR task);
        
        DECLSPEC_XFGVIRT(IDataCollectorSet, get_TaskUserTextArguments)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_TaskUserTextArguments )( 
            __RPC__in IDataCollectorSet * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *UserText);
        
        DECLSPEC_XFGVIRT(IDataCollectorSet, put_TaskUserTextArguments)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_TaskUserTextArguments )( 
            __RPC__in IDataCollectorSet * This,
            /* [in] */ __RPC__in BSTR UserText);
        
        DECLSPEC_XFGVIRT(IDataCollectorSet, get_Schedules)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Schedules )( 
            __RPC__in IDataCollectorSet * This,
            /* [retval][out] */ __RPC__deref_out_opt IScheduleCollection **ppSchedules);
        
        DECLSPEC_XFGVIRT(IDataCollectorSet, get_SchedulesEnabled)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_SchedulesEnabled )( 
            __RPC__in IDataCollectorSet * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *enabled);
        
        DECLSPEC_XFGVIRT(IDataCollectorSet, put_SchedulesEnabled)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_SchedulesEnabled )( 
            __RPC__in IDataCollectorSet * This,
            /* [in] */ VARIANT_BOOL enabled);
        
        DECLSPEC_XFGVIRT(IDataCollectorSet, get_UserAccount)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_UserAccount )( 
            __RPC__in IDataCollectorSet * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *user);
        
        DECLSPEC_XFGVIRT(IDataCollectorSet, get_Xml)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Xml )( 
            __RPC__in IDataCollectorSet * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *xml);
        
        DECLSPEC_XFGVIRT(IDataCollectorSet, get_Security)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Security )( 
            __RPC__in IDataCollectorSet * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrSecurity);
        
        DECLSPEC_XFGVIRT(IDataCollectorSet, put_Security)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Security )( 
            __RPC__in IDataCollectorSet * This,
            /* [in] */ __RPC__in BSTR bstrSecurity);
        
        DECLSPEC_XFGVIRT(IDataCollectorSet, get_StopOnCompletion)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_StopOnCompletion )( 
            __RPC__in IDataCollectorSet * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Stop);
        
        DECLSPEC_XFGVIRT(IDataCollectorSet, put_StopOnCompletion)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_StopOnCompletion )( 
            __RPC__in IDataCollectorSet * This,
            /* [in] */ VARIANT_BOOL Stop);
        
        DECLSPEC_XFGVIRT(IDataCollectorSet, get_DataManager)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_DataManager )( 
            __RPC__in IDataCollectorSet * This,
            /* [retval][out] */ __RPC__deref_out_opt IDataManager **DataManager);
        
        DECLSPEC_XFGVIRT(IDataCollectorSet, SetCredentials)
        HRESULT ( STDMETHODCALLTYPE *SetCredentials )( 
            __RPC__in IDataCollectorSet * This,
            __RPC__in BSTR user,
            __RPC__in BSTR password);
        
        DECLSPEC_XFGVIRT(IDataCollectorSet, Query)
        HRESULT ( STDMETHODCALLTYPE *Query )( 
            __RPC__in IDataCollectorSet * This,
            /* [in] */ __RPC__in BSTR name,
            /* [unique][in] */ __RPC__in_opt BSTR server);
        
        DECLSPEC_XFGVIRT(IDataCollectorSet, Commit)
        HRESULT ( STDMETHODCALLTYPE *Commit )( 
            __RPC__in IDataCollectorSet * This,
            /* [in] */ __RPC__in BSTR name,
            /* [unique][in] */ __RPC__in_opt BSTR server,
            CommitMode mode,
            /* [retval][out] */ __RPC__deref_out_opt IValueMap **validation);
        
        DECLSPEC_XFGVIRT(IDataCollectorSet, Delete)
        HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IDataCollectorSet * This);
        
        DECLSPEC_XFGVIRT(IDataCollectorSet, Start)
        HRESULT ( STDMETHODCALLTYPE *Start )( 
            __RPC__in IDataCollectorSet * This,
            /* [in] */ VARIANT_BOOL Synchronous);
        
        DECLSPEC_XFGVIRT(IDataCollectorSet, Stop)
        HRESULT ( STDMETHODCALLTYPE *Stop )( 
            __RPC__in IDataCollectorSet * This,
            /* [in] */ VARIANT_BOOL Synchronous);
        
        DECLSPEC_XFGVIRT(IDataCollectorSet, SetXml)
        HRESULT ( STDMETHODCALLTYPE *SetXml )( 
            __RPC__in IDataCollectorSet * This,
            /* [in] */ __RPC__in BSTR xml,
            /* [retval][out] */ __RPC__deref_out_opt IValueMap **validation);
        
        DECLSPEC_XFGVIRT(IDataCollectorSet, SetValue)
        HRESULT ( STDMETHODCALLTYPE *SetValue )( 
            __RPC__in IDataCollectorSet * This,
            __RPC__in BSTR key,
            __RPC__in BSTR value);
        
        DECLSPEC_XFGVIRT(IDataCollectorSet, GetValue)
        HRESULT ( STDMETHODCALLTYPE *GetValue )( 
            __RPC__in IDataCollectorSet * This,
            __RPC__in BSTR key,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *value);
        
        END_INTERFACE
    } IDataCollectorSetVtbl;

    interface IDataCollectorSet
    {
        CONST_VTBL struct IDataCollectorSetVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDataCollectorSet_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDataCollectorSet_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDataCollectorSet_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDataCollectorSet_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IDataCollectorSet_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IDataCollectorSet_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IDataCollectorSet_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IDataCollectorSet_get_DataCollectors(This,collectors)	\
    ( (This)->lpVtbl -> get_DataCollectors(This,collectors) ) 

#define IDataCollectorSet_get_Duration(This,seconds)	\
    ( (This)->lpVtbl -> get_Duration(This,seconds) ) 

#define IDataCollectorSet_put_Duration(This,seconds)	\
    ( (This)->lpVtbl -> put_Duration(This,seconds) ) 

#define IDataCollectorSet_get_Description(This,description)	\
    ( (This)->lpVtbl -> get_Description(This,description) ) 

#define IDataCollectorSet_put_Description(This,description)	\
    ( (This)->lpVtbl -> put_Description(This,description) ) 

#define IDataCollectorSet_get_DescriptionUnresolved(This,Descr)	\
    ( (This)->lpVtbl -> get_DescriptionUnresolved(This,Descr) ) 

#define IDataCollectorSet_get_DisplayName(This,DisplayName)	\
    ( (This)->lpVtbl -> get_DisplayName(This,DisplayName) ) 

#define IDataCollectorSet_put_DisplayName(This,DisplayName)	\
    ( (This)->lpVtbl -> put_DisplayName(This,DisplayName) ) 

#define IDataCollectorSet_get_DisplayNameUnresolved(This,name)	\
    ( (This)->lpVtbl -> get_DisplayNameUnresolved(This,name) ) 

#define IDataCollectorSet_get_Keywords(This,keywords)	\
    ( (This)->lpVtbl -> get_Keywords(This,keywords) ) 

#define IDataCollectorSet_put_Keywords(This,keywords)	\
    ( (This)->lpVtbl -> put_Keywords(This,keywords) ) 

#define IDataCollectorSet_get_LatestOutputLocation(This,path)	\
    ( (This)->lpVtbl -> get_LatestOutputLocation(This,path) ) 

#define IDataCollectorSet_put_LatestOutputLocation(This,path)	\
    ( (This)->lpVtbl -> put_LatestOutputLocation(This,path) ) 

#define IDataCollectorSet_get_Name(This,name)	\
    ( (This)->lpVtbl -> get_Name(This,name) ) 

#define IDataCollectorSet_get_OutputLocation(This,path)	\
    ( (This)->lpVtbl -> get_OutputLocation(This,path) ) 

#define IDataCollectorSet_get_RootPath(This,folder)	\
    ( (This)->lpVtbl -> get_RootPath(This,folder) ) 

#define IDataCollectorSet_put_RootPath(This,folder)	\
    ( (This)->lpVtbl -> put_RootPath(This,folder) ) 

#define IDataCollectorSet_get_Segment(This,segment)	\
    ( (This)->lpVtbl -> get_Segment(This,segment) ) 

#define IDataCollectorSet_put_Segment(This,segment)	\
    ( (This)->lpVtbl -> put_Segment(This,segment) ) 

#define IDataCollectorSet_get_SegmentMaxDuration(This,seconds)	\
    ( (This)->lpVtbl -> get_SegmentMaxDuration(This,seconds) ) 

#define IDataCollectorSet_put_SegmentMaxDuration(This,seconds)	\
    ( (This)->lpVtbl -> put_SegmentMaxDuration(This,seconds) ) 

#define IDataCollectorSet_get_SegmentMaxSize(This,size)	\
    ( (This)->lpVtbl -> get_SegmentMaxSize(This,size) ) 

#define IDataCollectorSet_put_SegmentMaxSize(This,size)	\
    ( (This)->lpVtbl -> put_SegmentMaxSize(This,size) ) 

#define IDataCollectorSet_get_SerialNumber(This,index)	\
    ( (This)->lpVtbl -> get_SerialNumber(This,index) ) 

#define IDataCollectorSet_put_SerialNumber(This,index)	\
    ( (This)->lpVtbl -> put_SerialNumber(This,index) ) 

#define IDataCollectorSet_get_Server(This,server)	\
    ( (This)->lpVtbl -> get_Server(This,server) ) 

#define IDataCollectorSet_get_Status(This,status)	\
    ( (This)->lpVtbl -> get_Status(This,status) ) 

#define IDataCollectorSet_get_Subdirectory(This,folder)	\
    ( (This)->lpVtbl -> get_Subdirectory(This,folder) ) 

#define IDataCollectorSet_put_Subdirectory(This,folder)	\
    ( (This)->lpVtbl -> put_Subdirectory(This,folder) ) 

#define IDataCollectorSet_get_SubdirectoryFormat(This,format)	\
    ( (This)->lpVtbl -> get_SubdirectoryFormat(This,format) ) 

#define IDataCollectorSet_put_SubdirectoryFormat(This,format)	\
    ( (This)->lpVtbl -> put_SubdirectoryFormat(This,format) ) 

#define IDataCollectorSet_get_SubdirectoryFormatPattern(This,pattern)	\
    ( (This)->lpVtbl -> get_SubdirectoryFormatPattern(This,pattern) ) 

#define IDataCollectorSet_put_SubdirectoryFormatPattern(This,pattern)	\
    ( (This)->lpVtbl -> put_SubdirectoryFormatPattern(This,pattern) ) 

#define IDataCollectorSet_get_Task(This,task)	\
    ( (This)->lpVtbl -> get_Task(This,task) ) 

#define IDataCollectorSet_put_Task(This,task)	\
    ( (This)->lpVtbl -> put_Task(This,task) ) 

#define IDataCollectorSet_get_TaskRunAsSelf(This,RunAsSelf)	\
    ( (This)->lpVtbl -> get_TaskRunAsSelf(This,RunAsSelf) ) 

#define IDataCollectorSet_put_TaskRunAsSelf(This,RunAsSelf)	\
    ( (This)->lpVtbl -> put_TaskRunAsSelf(This,RunAsSelf) ) 

#define IDataCollectorSet_get_TaskArguments(This,task)	\
    ( (This)->lpVtbl -> get_TaskArguments(This,task) ) 

#define IDataCollectorSet_put_TaskArguments(This,task)	\
    ( (This)->lpVtbl -> put_TaskArguments(This,task) ) 

#define IDataCollectorSet_get_TaskUserTextArguments(This,UserText)	\
    ( (This)->lpVtbl -> get_TaskUserTextArguments(This,UserText) ) 

#define IDataCollectorSet_put_TaskUserTextArguments(This,UserText)	\
    ( (This)->lpVtbl -> put_TaskUserTextArguments(This,UserText) ) 

#define IDataCollectorSet_get_Schedules(This,ppSchedules)	\
    ( (This)->lpVtbl -> get_Schedules(This,ppSchedules) ) 

#define IDataCollectorSet_get_SchedulesEnabled(This,enabled)	\
    ( (This)->lpVtbl -> get_SchedulesEnabled(This,enabled) ) 

#define IDataCollectorSet_put_SchedulesEnabled(This,enabled)	\
    ( (This)->lpVtbl -> put_SchedulesEnabled(This,enabled) ) 

#define IDataCollectorSet_get_UserAccount(This,user)	\
    ( (This)->lpVtbl -> get_UserAccount(This,user) ) 

#define IDataCollectorSet_get_Xml(This,xml)	\
    ( (This)->lpVtbl -> get_Xml(This,xml) ) 

#define IDataCollectorSet_get_Security(This,pbstrSecurity)	\
    ( (This)->lpVtbl -> get_Security(This,pbstrSecurity) ) 

#define IDataCollectorSet_put_Security(This,bstrSecurity)	\
    ( (This)->lpVtbl -> put_Security(This,bstrSecurity) ) 

#define IDataCollectorSet_get_StopOnCompletion(This,Stop)	\
    ( (This)->lpVtbl -> get_StopOnCompletion(This,Stop) ) 

#define IDataCollectorSet_put_StopOnCompletion(This,Stop)	\
    ( (This)->lpVtbl -> put_StopOnCompletion(This,Stop) ) 

#define IDataCollectorSet_get_DataManager(This,DataManager)	\
    ( (This)->lpVtbl -> get_DataManager(This,DataManager) ) 

#define IDataCollectorSet_SetCredentials(This,user,password)	\
    ( (This)->lpVtbl -> SetCredentials(This,user,password) ) 

#define IDataCollectorSet_Query(This,name,server)	\
    ( (This)->lpVtbl -> Query(This,name,server) ) 

#define IDataCollectorSet_Commit(This,name,server,mode,validation)	\
    ( (This)->lpVtbl -> Commit(This,name,server,mode,validation) ) 

#define IDataCollectorSet_Delete(This)	\
    ( (This)->lpVtbl -> Delete(This) ) 

#define IDataCollectorSet_Start(This,Synchronous)	\
    ( (This)->lpVtbl -> Start(This,Synchronous) ) 

#define IDataCollectorSet_Stop(This,Synchronous)	\
    ( (This)->lpVtbl -> Stop(This,Synchronous) ) 

#define IDataCollectorSet_SetXml(This,xml,validation)	\
    ( (This)->lpVtbl -> SetXml(This,xml,validation) ) 

#define IDataCollectorSet_SetValue(This,key,value)	\
    ( (This)->lpVtbl -> SetValue(This,key,value) ) 

#define IDataCollectorSet_GetValue(This,key,value)	\
    ( (This)->lpVtbl -> GetValue(This,key,value) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDataCollectorSet_INTERFACE_DEFINED__ */


#ifndef __IDataManager_INTERFACE_DEFINED__
#define __IDataManager_INTERFACE_DEFINED__

/* interface IDataManager */
/* [oleautomation][dual][uuid][object] */ 


EXTERN_C const IID IID_IDataManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("03837541-098b-11d8-9414-505054503030")
    IDataManager : public IDispatch
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Enabled( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfEnabled) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_Enabled( 
            /* [in] */ VARIANT_BOOL fEnabled) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_CheckBeforeRunning( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfCheck) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_CheckBeforeRunning( 
            /* [in] */ VARIANT_BOOL fCheck) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_MinFreeDisk( 
            /* [retval][out] */ __RPC__out ULONG *MinFreeDisk) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_MinFreeDisk( 
            /* [in] */ ULONG MinFreeDisk) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_MaxSize( 
            /* [retval][out] */ __RPC__out ULONG *pulMaxSize) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_MaxSize( 
            /* [in] */ ULONG ulMaxSize) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_MaxFolderCount( 
            /* [retval][out] */ __RPC__out ULONG *pulMaxFolderCount) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_MaxFolderCount( 
            /* [in] */ ULONG ulMaxFolderCount) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ResourcePolicy( 
            /* [retval][out] */ __RPC__out ResourcePolicy *pPolicy) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_ResourcePolicy( 
            /* [in] */ ResourcePolicy Policy) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_FolderActions( 
            /* [retval][out] */ __RPC__deref_out_opt IFolderActionCollection **Actions) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ReportSchema( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ReportSchema) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_ReportSchema( 
            /* [in] */ __RPC__in BSTR ReportSchema) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ReportFileName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrFilename) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_ReportFileName( 
            /* [in] */ __RPC__in BSTR pbstrFilename) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_RuleTargetFileName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Filename) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_RuleTargetFileName( 
            /* [in] */ __RPC__in BSTR Filename) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_EventsFileName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrFilename) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_EventsFileName( 
            /* [in] */ __RPC__in BSTR pbstrFilename) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Rules( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrXml) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_Rules( 
            /* [in] */ __RPC__in BSTR bstrXml) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Run( 
            /* [in] */ DataManagerSteps Steps,
            /* [in] */ __RPC__in BSTR bstrFolder,
            /* [retval][out] */ __RPC__deref_out_opt IValueMap **Errors) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Extract( 
            /* [in] */ __RPC__in BSTR CabFilename,
            /* [in] */ __RPC__in BSTR DestinationPath) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDataManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDataManager * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDataManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDataManager * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IDataManager * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IDataManager * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IDataManager * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IDataManager * This,
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
        
        DECLSPEC_XFGVIRT(IDataManager, get_Enabled)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Enabled )( 
            __RPC__in IDataManager * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfEnabled);
        
        DECLSPEC_XFGVIRT(IDataManager, put_Enabled)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Enabled )( 
            __RPC__in IDataManager * This,
            /* [in] */ VARIANT_BOOL fEnabled);
        
        DECLSPEC_XFGVIRT(IDataManager, get_CheckBeforeRunning)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_CheckBeforeRunning )( 
            __RPC__in IDataManager * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfCheck);
        
        DECLSPEC_XFGVIRT(IDataManager, put_CheckBeforeRunning)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_CheckBeforeRunning )( 
            __RPC__in IDataManager * This,
            /* [in] */ VARIANT_BOOL fCheck);
        
        DECLSPEC_XFGVIRT(IDataManager, get_MinFreeDisk)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_MinFreeDisk )( 
            __RPC__in IDataManager * This,
            /* [retval][out] */ __RPC__out ULONG *MinFreeDisk);
        
        DECLSPEC_XFGVIRT(IDataManager, put_MinFreeDisk)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_MinFreeDisk )( 
            __RPC__in IDataManager * This,
            /* [in] */ ULONG MinFreeDisk);
        
        DECLSPEC_XFGVIRT(IDataManager, get_MaxSize)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_MaxSize )( 
            __RPC__in IDataManager * This,
            /* [retval][out] */ __RPC__out ULONG *pulMaxSize);
        
        DECLSPEC_XFGVIRT(IDataManager, put_MaxSize)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_MaxSize )( 
            __RPC__in IDataManager * This,
            /* [in] */ ULONG ulMaxSize);
        
        DECLSPEC_XFGVIRT(IDataManager, get_MaxFolderCount)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_MaxFolderCount )( 
            __RPC__in IDataManager * This,
            /* [retval][out] */ __RPC__out ULONG *pulMaxFolderCount);
        
        DECLSPEC_XFGVIRT(IDataManager, put_MaxFolderCount)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_MaxFolderCount )( 
            __RPC__in IDataManager * This,
            /* [in] */ ULONG ulMaxFolderCount);
        
        DECLSPEC_XFGVIRT(IDataManager, get_ResourcePolicy)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ResourcePolicy )( 
            __RPC__in IDataManager * This,
            /* [retval][out] */ __RPC__out ResourcePolicy *pPolicy);
        
        DECLSPEC_XFGVIRT(IDataManager, put_ResourcePolicy)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ResourcePolicy )( 
            __RPC__in IDataManager * This,
            /* [in] */ ResourcePolicy Policy);
        
        DECLSPEC_XFGVIRT(IDataManager, get_FolderActions)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_FolderActions )( 
            __RPC__in IDataManager * This,
            /* [retval][out] */ __RPC__deref_out_opt IFolderActionCollection **Actions);
        
        DECLSPEC_XFGVIRT(IDataManager, get_ReportSchema)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ReportSchema )( 
            __RPC__in IDataManager * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ReportSchema);
        
        DECLSPEC_XFGVIRT(IDataManager, put_ReportSchema)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ReportSchema )( 
            __RPC__in IDataManager * This,
            /* [in] */ __RPC__in BSTR ReportSchema);
        
        DECLSPEC_XFGVIRT(IDataManager, get_ReportFileName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ReportFileName )( 
            __RPC__in IDataManager * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrFilename);
        
        DECLSPEC_XFGVIRT(IDataManager, put_ReportFileName)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ReportFileName )( 
            __RPC__in IDataManager * This,
            /* [in] */ __RPC__in BSTR pbstrFilename);
        
        DECLSPEC_XFGVIRT(IDataManager, get_RuleTargetFileName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RuleTargetFileName )( 
            __RPC__in IDataManager * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Filename);
        
        DECLSPEC_XFGVIRT(IDataManager, put_RuleTargetFileName)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_RuleTargetFileName )( 
            __RPC__in IDataManager * This,
            /* [in] */ __RPC__in BSTR Filename);
        
        DECLSPEC_XFGVIRT(IDataManager, get_EventsFileName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_EventsFileName )( 
            __RPC__in IDataManager * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrFilename);
        
        DECLSPEC_XFGVIRT(IDataManager, put_EventsFileName)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_EventsFileName )( 
            __RPC__in IDataManager * This,
            /* [in] */ __RPC__in BSTR pbstrFilename);
        
        DECLSPEC_XFGVIRT(IDataManager, get_Rules)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Rules )( 
            __RPC__in IDataManager * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrXml);
        
        DECLSPEC_XFGVIRT(IDataManager, put_Rules)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Rules )( 
            __RPC__in IDataManager * This,
            /* [in] */ __RPC__in BSTR bstrXml);
        
        DECLSPEC_XFGVIRT(IDataManager, Run)
        HRESULT ( STDMETHODCALLTYPE *Run )( 
            __RPC__in IDataManager * This,
            /* [in] */ DataManagerSteps Steps,
            /* [in] */ __RPC__in BSTR bstrFolder,
            /* [retval][out] */ __RPC__deref_out_opt IValueMap **Errors);
        
        DECLSPEC_XFGVIRT(IDataManager, Extract)
        HRESULT ( STDMETHODCALLTYPE *Extract )( 
            __RPC__in IDataManager * This,
            /* [in] */ __RPC__in BSTR CabFilename,
            /* [in] */ __RPC__in BSTR DestinationPath);
        
        END_INTERFACE
    } IDataManagerVtbl;

    interface IDataManager
    {
        CONST_VTBL struct IDataManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDataManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDataManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDataManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDataManager_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IDataManager_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IDataManager_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IDataManager_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IDataManager_get_Enabled(This,pfEnabled)	\
    ( (This)->lpVtbl -> get_Enabled(This,pfEnabled) ) 

#define IDataManager_put_Enabled(This,fEnabled)	\
    ( (This)->lpVtbl -> put_Enabled(This,fEnabled) ) 

#define IDataManager_get_CheckBeforeRunning(This,pfCheck)	\
    ( (This)->lpVtbl -> get_CheckBeforeRunning(This,pfCheck) ) 

#define IDataManager_put_CheckBeforeRunning(This,fCheck)	\
    ( (This)->lpVtbl -> put_CheckBeforeRunning(This,fCheck) ) 

#define IDataManager_get_MinFreeDisk(This,MinFreeDisk)	\
    ( (This)->lpVtbl -> get_MinFreeDisk(This,MinFreeDisk) ) 

#define IDataManager_put_MinFreeDisk(This,MinFreeDisk)	\
    ( (This)->lpVtbl -> put_MinFreeDisk(This,MinFreeDisk) ) 

#define IDataManager_get_MaxSize(This,pulMaxSize)	\
    ( (This)->lpVtbl -> get_MaxSize(This,pulMaxSize) ) 

#define IDataManager_put_MaxSize(This,ulMaxSize)	\
    ( (This)->lpVtbl -> put_MaxSize(This,ulMaxSize) ) 

#define IDataManager_get_MaxFolderCount(This,pulMaxFolderCount)	\
    ( (This)->lpVtbl -> get_MaxFolderCount(This,pulMaxFolderCount) ) 

#define IDataManager_put_MaxFolderCount(This,ulMaxFolderCount)	\
    ( (This)->lpVtbl -> put_MaxFolderCount(This,ulMaxFolderCount) ) 

#define IDataManager_get_ResourcePolicy(This,pPolicy)	\
    ( (This)->lpVtbl -> get_ResourcePolicy(This,pPolicy) ) 

#define IDataManager_put_ResourcePolicy(This,Policy)	\
    ( (This)->lpVtbl -> put_ResourcePolicy(This,Policy) ) 

#define IDataManager_get_FolderActions(This,Actions)	\
    ( (This)->lpVtbl -> get_FolderActions(This,Actions) ) 

#define IDataManager_get_ReportSchema(This,ReportSchema)	\
    ( (This)->lpVtbl -> get_ReportSchema(This,ReportSchema) ) 

#define IDataManager_put_ReportSchema(This,ReportSchema)	\
    ( (This)->lpVtbl -> put_ReportSchema(This,ReportSchema) ) 

#define IDataManager_get_ReportFileName(This,pbstrFilename)	\
    ( (This)->lpVtbl -> get_ReportFileName(This,pbstrFilename) ) 

#define IDataManager_put_ReportFileName(This,pbstrFilename)	\
    ( (This)->lpVtbl -> put_ReportFileName(This,pbstrFilename) ) 

#define IDataManager_get_RuleTargetFileName(This,Filename)	\
    ( (This)->lpVtbl -> get_RuleTargetFileName(This,Filename) ) 

#define IDataManager_put_RuleTargetFileName(This,Filename)	\
    ( (This)->lpVtbl -> put_RuleTargetFileName(This,Filename) ) 

#define IDataManager_get_EventsFileName(This,pbstrFilename)	\
    ( (This)->lpVtbl -> get_EventsFileName(This,pbstrFilename) ) 

#define IDataManager_put_EventsFileName(This,pbstrFilename)	\
    ( (This)->lpVtbl -> put_EventsFileName(This,pbstrFilename) ) 

#define IDataManager_get_Rules(This,pbstrXml)	\
    ( (This)->lpVtbl -> get_Rules(This,pbstrXml) ) 

#define IDataManager_put_Rules(This,bstrXml)	\
    ( (This)->lpVtbl -> put_Rules(This,bstrXml) ) 

#define IDataManager_Run(This,Steps,bstrFolder,Errors)	\
    ( (This)->lpVtbl -> Run(This,Steps,bstrFolder,Errors) ) 

#define IDataManager_Extract(This,CabFilename,DestinationPath)	\
    ( (This)->lpVtbl -> Extract(This,CabFilename,DestinationPath) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDataManager_INTERFACE_DEFINED__ */


#ifndef __IFolderAction_INTERFACE_DEFINED__
#define __IFolderAction_INTERFACE_DEFINED__

/* interface IFolderAction */
/* [oleautomation][dual][uuid][object] */ 


EXTERN_C const IID IID_IFolderAction;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("03837543-098b-11d8-9414-505054503030")
    IFolderAction : public IDispatch
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Age( 
            /* [retval][out] */ __RPC__out ULONG *pulAge) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_Age( 
            /* [in] */ ULONG ulAge) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Size( 
            /* [retval][out] */ __RPC__out ULONG *pulAge) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_Size( 
            /* [in] */ ULONG ulAge) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Actions( 
            /* [retval][out] */ __RPC__out FolderActionSteps *Steps) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_Actions( 
            /* [in] */ FolderActionSteps Steps) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_SendCabTo( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDestination) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_SendCabTo( 
            /* [in] */ __RPC__in BSTR bstrDestination) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFolderActionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFolderAction * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFolderAction * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFolderAction * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFolderAction * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFolderAction * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFolderAction * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFolderAction * This,
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
        
        DECLSPEC_XFGVIRT(IFolderAction, get_Age)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Age )( 
            __RPC__in IFolderAction * This,
            /* [retval][out] */ __RPC__out ULONG *pulAge);
        
        DECLSPEC_XFGVIRT(IFolderAction, put_Age)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Age )( 
            __RPC__in IFolderAction * This,
            /* [in] */ ULONG ulAge);
        
        DECLSPEC_XFGVIRT(IFolderAction, get_Size)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Size )( 
            __RPC__in IFolderAction * This,
            /* [retval][out] */ __RPC__out ULONG *pulAge);
        
        DECLSPEC_XFGVIRT(IFolderAction, put_Size)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Size )( 
            __RPC__in IFolderAction * This,
            /* [in] */ ULONG ulAge);
        
        DECLSPEC_XFGVIRT(IFolderAction, get_Actions)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Actions )( 
            __RPC__in IFolderAction * This,
            /* [retval][out] */ __RPC__out FolderActionSteps *Steps);
        
        DECLSPEC_XFGVIRT(IFolderAction, put_Actions)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Actions )( 
            __RPC__in IFolderAction * This,
            /* [in] */ FolderActionSteps Steps);
        
        DECLSPEC_XFGVIRT(IFolderAction, get_SendCabTo)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_SendCabTo )( 
            __RPC__in IFolderAction * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDestination);
        
        DECLSPEC_XFGVIRT(IFolderAction, put_SendCabTo)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_SendCabTo )( 
            __RPC__in IFolderAction * This,
            /* [in] */ __RPC__in BSTR bstrDestination);
        
        END_INTERFACE
    } IFolderActionVtbl;

    interface IFolderAction
    {
        CONST_VTBL struct IFolderActionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFolderAction_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFolderAction_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFolderAction_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFolderAction_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFolderAction_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFolderAction_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFolderAction_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFolderAction_get_Age(This,pulAge)	\
    ( (This)->lpVtbl -> get_Age(This,pulAge) ) 

#define IFolderAction_put_Age(This,ulAge)	\
    ( (This)->lpVtbl -> put_Age(This,ulAge) ) 

#define IFolderAction_get_Size(This,pulAge)	\
    ( (This)->lpVtbl -> get_Size(This,pulAge) ) 

#define IFolderAction_put_Size(This,ulAge)	\
    ( (This)->lpVtbl -> put_Size(This,ulAge) ) 

#define IFolderAction_get_Actions(This,Steps)	\
    ( (This)->lpVtbl -> get_Actions(This,Steps) ) 

#define IFolderAction_put_Actions(This,Steps)	\
    ( (This)->lpVtbl -> put_Actions(This,Steps) ) 

#define IFolderAction_get_SendCabTo(This,pbstrDestination)	\
    ( (This)->lpVtbl -> get_SendCabTo(This,pbstrDestination) ) 

#define IFolderAction_put_SendCabTo(This,bstrDestination)	\
    ( (This)->lpVtbl -> put_SendCabTo(This,bstrDestination) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFolderAction_INTERFACE_DEFINED__ */


#ifndef __IFolderActionCollection_INTERFACE_DEFINED__
#define __IFolderActionCollection_INTERFACE_DEFINED__

/* interface IFolderActionCollection */
/* [nonextensible][oleautomation][dual][uuid][object] */ 


EXTERN_C const IID IID_IFolderActionCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("03837544-098b-11d8-9414-505054503030")
    IFolderActionCollection : public IDispatch
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out ULONG *Count) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT Index,
            /* [retval][out] */ __RPC__deref_out_opt IFolderAction **Action) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **Enum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Add( 
            __RPC__in_opt IFolderAction *Action) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Remove( 
            VARIANT Index) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clear( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddRange( 
            __RPC__in_opt IFolderActionCollection *Actions) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateFolderAction( 
            /* [retval][out] */ __RPC__deref_out_opt IFolderAction **FolderAction) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFolderActionCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFolderActionCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFolderActionCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFolderActionCollection * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFolderActionCollection * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFolderActionCollection * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFolderActionCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFolderActionCollection * This,
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
        
        DECLSPEC_XFGVIRT(IFolderActionCollection, get_Count)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IFolderActionCollection * This,
            /* [retval][out] */ __RPC__out ULONG *Count);
        
        DECLSPEC_XFGVIRT(IFolderActionCollection, get_Item)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IFolderActionCollection * This,
            /* [in] */ VARIANT Index,
            /* [retval][out] */ __RPC__deref_out_opt IFolderAction **Action);
        
        DECLSPEC_XFGVIRT(IFolderActionCollection, get__NewEnum)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IFolderActionCollection * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **Enum);
        
        DECLSPEC_XFGVIRT(IFolderActionCollection, Add)
        HRESULT ( STDMETHODCALLTYPE *Add )( 
            __RPC__in IFolderActionCollection * This,
            __RPC__in_opt IFolderAction *Action);
        
        DECLSPEC_XFGVIRT(IFolderActionCollection, Remove)
        HRESULT ( STDMETHODCALLTYPE *Remove )( 
            __RPC__in IFolderActionCollection * This,
            VARIANT Index);
        
        DECLSPEC_XFGVIRT(IFolderActionCollection, Clear)
        HRESULT ( STDMETHODCALLTYPE *Clear )( 
            __RPC__in IFolderActionCollection * This);
        
        DECLSPEC_XFGVIRT(IFolderActionCollection, AddRange)
        HRESULT ( STDMETHODCALLTYPE *AddRange )( 
            __RPC__in IFolderActionCollection * This,
            __RPC__in_opt IFolderActionCollection *Actions);
        
        DECLSPEC_XFGVIRT(IFolderActionCollection, CreateFolderAction)
        HRESULT ( STDMETHODCALLTYPE *CreateFolderAction )( 
            __RPC__in IFolderActionCollection * This,
            /* [retval][out] */ __RPC__deref_out_opt IFolderAction **FolderAction);
        
        END_INTERFACE
    } IFolderActionCollectionVtbl;

    interface IFolderActionCollection
    {
        CONST_VTBL struct IFolderActionCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFolderActionCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFolderActionCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFolderActionCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFolderActionCollection_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFolderActionCollection_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFolderActionCollection_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFolderActionCollection_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFolderActionCollection_get_Count(This,Count)	\
    ( (This)->lpVtbl -> get_Count(This,Count) ) 

#define IFolderActionCollection_get_Item(This,Index,Action)	\
    ( (This)->lpVtbl -> get_Item(This,Index,Action) ) 

#define IFolderActionCollection_get__NewEnum(This,Enum)	\
    ( (This)->lpVtbl -> get__NewEnum(This,Enum) ) 

#define IFolderActionCollection_Add(This,Action)	\
    ( (This)->lpVtbl -> Add(This,Action) ) 

#define IFolderActionCollection_Remove(This,Index)	\
    ( (This)->lpVtbl -> Remove(This,Index) ) 

#define IFolderActionCollection_Clear(This)	\
    ( (This)->lpVtbl -> Clear(This) ) 

#define IFolderActionCollection_AddRange(This,Actions)	\
    ( (This)->lpVtbl -> AddRange(This,Actions) ) 

#define IFolderActionCollection_CreateFolderAction(This,FolderAction)	\
    ( (This)->lpVtbl -> CreateFolderAction(This,FolderAction) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFolderActionCollection_INTERFACE_DEFINED__ */


#ifndef __IDataCollector_INTERFACE_DEFINED__
#define __IDataCollector_INTERFACE_DEFINED__

/* interface IDataCollector */
/* [dual][uuid][object] */ 


EXTERN_C const IID IID_IDataCollector;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("038374ff-098b-11d8-9414-505054503030")
    IDataCollector : public IDispatch
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_DataCollectorSet( 
            /* [retval][out] */ __RPC__deref_out_opt IDataCollectorSet **group) = 0;
        
        virtual /* [restricted][hidden][propput] */ HRESULT STDMETHODCALLTYPE put_DataCollectorSet( 
            /* [in] */ __RPC__in_opt IDataCollectorSet *group) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_DataCollectorType( 
            /* [retval][out] */ __RPC__out DataCollectorType *type) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_FileName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *name) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_FileName( 
            /* [in] */ __RPC__in BSTR name) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_FileNameFormat( 
            /* [retval][out] */ __RPC__out AutoPathFormat *format) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_FileNameFormat( 
            /* [in] */ AutoPathFormat format) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_FileNameFormatPattern( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pattern) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_FileNameFormatPattern( 
            /* [in] */ __RPC__in BSTR pattern) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_LatestOutputLocation( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *path) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_LatestOutputLocation( 
            /* [in] */ __RPC__in BSTR path) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_LogAppend( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *append) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_LogAppend( 
            /* [in] */ VARIANT_BOOL append) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_LogCircular( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *circular) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_LogCircular( 
            /* [in] */ VARIANT_BOOL circular) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_LogOverwrite( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *overwrite) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_LogOverwrite( 
            /* [in] */ VARIANT_BOOL overwrite) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *name) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_Name( 
            /* [in] */ __RPC__in BSTR name) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_OutputLocation( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *path) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Index( 
            /* [retval][out] */ __RPC__out long *index) = 0;
        
        virtual /* [restricted][hidden][propput] */ HRESULT STDMETHODCALLTYPE put_Index( 
            /* [in] */ long index) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Xml( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Xml) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetXml( 
            /* [in] */ __RPC__in BSTR Xml,
            /* [retval][out] */ __RPC__deref_out_opt IValueMap **Validation) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateOutputLocation( 
            /* [in] */ VARIANT_BOOL Latest,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Location) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDataCollectorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDataCollector * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDataCollector * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDataCollector * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IDataCollector * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IDataCollector * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IDataCollector * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IDataCollector * This,
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
        
        DECLSPEC_XFGVIRT(IDataCollector, get_DataCollectorSet)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_DataCollectorSet )( 
            __RPC__in IDataCollector * This,
            /* [retval][out] */ __RPC__deref_out_opt IDataCollectorSet **group);
        
        DECLSPEC_XFGVIRT(IDataCollector, put_DataCollectorSet)
        /* [restricted][hidden][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DataCollectorSet )( 
            __RPC__in IDataCollector * This,
            /* [in] */ __RPC__in_opt IDataCollectorSet *group);
        
        DECLSPEC_XFGVIRT(IDataCollector, get_DataCollectorType)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_DataCollectorType )( 
            __RPC__in IDataCollector * This,
            /* [retval][out] */ __RPC__out DataCollectorType *type);
        
        DECLSPEC_XFGVIRT(IDataCollector, get_FileName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_FileName )( 
            __RPC__in IDataCollector * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *name);
        
        DECLSPEC_XFGVIRT(IDataCollector, put_FileName)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_FileName )( 
            __RPC__in IDataCollector * This,
            /* [in] */ __RPC__in BSTR name);
        
        DECLSPEC_XFGVIRT(IDataCollector, get_FileNameFormat)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_FileNameFormat )( 
            __RPC__in IDataCollector * This,
            /* [retval][out] */ __RPC__out AutoPathFormat *format);
        
        DECLSPEC_XFGVIRT(IDataCollector, put_FileNameFormat)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_FileNameFormat )( 
            __RPC__in IDataCollector * This,
            /* [in] */ AutoPathFormat format);
        
        DECLSPEC_XFGVIRT(IDataCollector, get_FileNameFormatPattern)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_FileNameFormatPattern )( 
            __RPC__in IDataCollector * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pattern);
        
        DECLSPEC_XFGVIRT(IDataCollector, put_FileNameFormatPattern)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_FileNameFormatPattern )( 
            __RPC__in IDataCollector * This,
            /* [in] */ __RPC__in BSTR pattern);
        
        DECLSPEC_XFGVIRT(IDataCollector, get_LatestOutputLocation)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_LatestOutputLocation )( 
            __RPC__in IDataCollector * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *path);
        
        DECLSPEC_XFGVIRT(IDataCollector, put_LatestOutputLocation)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_LatestOutputLocation )( 
            __RPC__in IDataCollector * This,
            /* [in] */ __RPC__in BSTR path);
        
        DECLSPEC_XFGVIRT(IDataCollector, get_LogAppend)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_LogAppend )( 
            __RPC__in IDataCollector * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *append);
        
        DECLSPEC_XFGVIRT(IDataCollector, put_LogAppend)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_LogAppend )( 
            __RPC__in IDataCollector * This,
            /* [in] */ VARIANT_BOOL append);
        
        DECLSPEC_XFGVIRT(IDataCollector, get_LogCircular)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_LogCircular )( 
            __RPC__in IDataCollector * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *circular);
        
        DECLSPEC_XFGVIRT(IDataCollector, put_LogCircular)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_LogCircular )( 
            __RPC__in IDataCollector * This,
            /* [in] */ VARIANT_BOOL circular);
        
        DECLSPEC_XFGVIRT(IDataCollector, get_LogOverwrite)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_LogOverwrite )( 
            __RPC__in IDataCollector * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *overwrite);
        
        DECLSPEC_XFGVIRT(IDataCollector, put_LogOverwrite)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_LogOverwrite )( 
            __RPC__in IDataCollector * This,
            /* [in] */ VARIANT_BOOL overwrite);
        
        DECLSPEC_XFGVIRT(IDataCollector, get_Name)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IDataCollector * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *name);
        
        DECLSPEC_XFGVIRT(IDataCollector, put_Name)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Name )( 
            __RPC__in IDataCollector * This,
            /* [in] */ __RPC__in BSTR name);
        
        DECLSPEC_XFGVIRT(IDataCollector, get_OutputLocation)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_OutputLocation )( 
            __RPC__in IDataCollector * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *path);
        
        DECLSPEC_XFGVIRT(IDataCollector, get_Index)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Index )( 
            __RPC__in IDataCollector * This,
            /* [retval][out] */ __RPC__out long *index);
        
        DECLSPEC_XFGVIRT(IDataCollector, put_Index)
        /* [restricted][hidden][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Index )( 
            __RPC__in IDataCollector * This,
            /* [in] */ long index);
        
        DECLSPEC_XFGVIRT(IDataCollector, get_Xml)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Xml )( 
            __RPC__in IDataCollector * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Xml);
        
        DECLSPEC_XFGVIRT(IDataCollector, SetXml)
        HRESULT ( STDMETHODCALLTYPE *SetXml )( 
            __RPC__in IDataCollector * This,
            /* [in] */ __RPC__in BSTR Xml,
            /* [retval][out] */ __RPC__deref_out_opt IValueMap **Validation);
        
        DECLSPEC_XFGVIRT(IDataCollector, CreateOutputLocation)
        HRESULT ( STDMETHODCALLTYPE *CreateOutputLocation )( 
            __RPC__in IDataCollector * This,
            /* [in] */ VARIANT_BOOL Latest,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Location);
        
        END_INTERFACE
    } IDataCollectorVtbl;

    interface IDataCollector
    {
        CONST_VTBL struct IDataCollectorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDataCollector_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDataCollector_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDataCollector_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDataCollector_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IDataCollector_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IDataCollector_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IDataCollector_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IDataCollector_get_DataCollectorSet(This,group)	\
    ( (This)->lpVtbl -> get_DataCollectorSet(This,group) ) 

#define IDataCollector_put_DataCollectorSet(This,group)	\
    ( (This)->lpVtbl -> put_DataCollectorSet(This,group) ) 

#define IDataCollector_get_DataCollectorType(This,type)	\
    ( (This)->lpVtbl -> get_DataCollectorType(This,type) ) 

#define IDataCollector_get_FileName(This,name)	\
    ( (This)->lpVtbl -> get_FileName(This,name) ) 

#define IDataCollector_put_FileName(This,name)	\
    ( (This)->lpVtbl -> put_FileName(This,name) ) 

#define IDataCollector_get_FileNameFormat(This,format)	\
    ( (This)->lpVtbl -> get_FileNameFormat(This,format) ) 

#define IDataCollector_put_FileNameFormat(This,format)	\
    ( (This)->lpVtbl -> put_FileNameFormat(This,format) ) 

#define IDataCollector_get_FileNameFormatPattern(This,pattern)	\
    ( (This)->lpVtbl -> get_FileNameFormatPattern(This,pattern) ) 

#define IDataCollector_put_FileNameFormatPattern(This,pattern)	\
    ( (This)->lpVtbl -> put_FileNameFormatPattern(This,pattern) ) 

#define IDataCollector_get_LatestOutputLocation(This,path)	\
    ( (This)->lpVtbl -> get_LatestOutputLocation(This,path) ) 

#define IDataCollector_put_LatestOutputLocation(This,path)	\
    ( (This)->lpVtbl -> put_LatestOutputLocation(This,path) ) 

#define IDataCollector_get_LogAppend(This,append)	\
    ( (This)->lpVtbl -> get_LogAppend(This,append) ) 

#define IDataCollector_put_LogAppend(This,append)	\
    ( (This)->lpVtbl -> put_LogAppend(This,append) ) 

#define IDataCollector_get_LogCircular(This,circular)	\
    ( (This)->lpVtbl -> get_LogCircular(This,circular) ) 

#define IDataCollector_put_LogCircular(This,circular)	\
    ( (This)->lpVtbl -> put_LogCircular(This,circular) ) 

#define IDataCollector_get_LogOverwrite(This,overwrite)	\
    ( (This)->lpVtbl -> get_LogOverwrite(This,overwrite) ) 

#define IDataCollector_put_LogOverwrite(This,overwrite)	\
    ( (This)->lpVtbl -> put_LogOverwrite(This,overwrite) ) 

#define IDataCollector_get_Name(This,name)	\
    ( (This)->lpVtbl -> get_Name(This,name) ) 

#define IDataCollector_put_Name(This,name)	\
    ( (This)->lpVtbl -> put_Name(This,name) ) 

#define IDataCollector_get_OutputLocation(This,path)	\
    ( (This)->lpVtbl -> get_OutputLocation(This,path) ) 

#define IDataCollector_get_Index(This,index)	\
    ( (This)->lpVtbl -> get_Index(This,index) ) 

#define IDataCollector_put_Index(This,index)	\
    ( (This)->lpVtbl -> put_Index(This,index) ) 

#define IDataCollector_get_Xml(This,Xml)	\
    ( (This)->lpVtbl -> get_Xml(This,Xml) ) 

#define IDataCollector_SetXml(This,Xml,Validation)	\
    ( (This)->lpVtbl -> SetXml(This,Xml,Validation) ) 

#define IDataCollector_CreateOutputLocation(This,Latest,Location)	\
    ( (This)->lpVtbl -> CreateOutputLocation(This,Latest,Location) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDataCollector_INTERFACE_DEFINED__ */


#ifndef __IPerformanceCounterDataCollector_INTERFACE_DEFINED__
#define __IPerformanceCounterDataCollector_INTERFACE_DEFINED__

/* interface IPerformanceCounterDataCollector */
/* [dual][uuid][object] */ 


EXTERN_C const IID IID_IPerformanceCounterDataCollector;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("03837506-098b-11d8-9414-505054503030")
    IPerformanceCounterDataCollector : public IDataCollector
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_DataSourceName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *dsn) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_DataSourceName( 
            /* [in] */ __RPC__in BSTR dsn) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_PerformanceCounters( 
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *counters) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_PerformanceCounters( 
            /* [in] */ __RPC__in SAFEARRAY * counters) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_LogFileFormat( 
            /* [retval][out] */ __RPC__out FileFormat *format) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_LogFileFormat( 
            /* [in] */ FileFormat format) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_SampleInterval( 
            /* [retval][out] */ __RPC__out unsigned long *interval) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_SampleInterval( 
            /* [in] */ unsigned long interval) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_SegmentMaxRecords( 
            /* [retval][out] */ __RPC__out unsigned long *records) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_SegmentMaxRecords( 
            /* [in] */ unsigned long records) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPerformanceCounterDataCollectorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPerformanceCounterDataCollector * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPerformanceCounterDataCollector * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPerformanceCounterDataCollector * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IPerformanceCounterDataCollector * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IPerformanceCounterDataCollector * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IPerformanceCounterDataCollector * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IPerformanceCounterDataCollector * This,
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
        
        DECLSPEC_XFGVIRT(IDataCollector, get_DataCollectorSet)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_DataCollectorSet )( 
            __RPC__in IPerformanceCounterDataCollector * This,
            /* [retval][out] */ __RPC__deref_out_opt IDataCollectorSet **group);
        
        DECLSPEC_XFGVIRT(IDataCollector, put_DataCollectorSet)
        /* [restricted][hidden][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DataCollectorSet )( 
            __RPC__in IPerformanceCounterDataCollector * This,
            /* [in] */ __RPC__in_opt IDataCollectorSet *group);
        
        DECLSPEC_XFGVIRT(IDataCollector, get_DataCollectorType)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_DataCollectorType )( 
            __RPC__in IPerformanceCounterDataCollector * This,
            /* [retval][out] */ __RPC__out DataCollectorType *type);
        
        DECLSPEC_XFGVIRT(IDataCollector, get_FileName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_FileName )( 
            __RPC__in IPerformanceCounterDataCollector * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *name);
        
        DECLSPEC_XFGVIRT(IDataCollector, put_FileName)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_FileName )( 
            __RPC__in IPerformanceCounterDataCollector * This,
            /* [in] */ __RPC__in BSTR name);
        
        DECLSPEC_XFGVIRT(IDataCollector, get_FileNameFormat)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_FileNameFormat )( 
            __RPC__in IPerformanceCounterDataCollector * This,
            /* [retval][out] */ __RPC__out AutoPathFormat *format);
        
        DECLSPEC_XFGVIRT(IDataCollector, put_FileNameFormat)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_FileNameFormat )( 
            __RPC__in IPerformanceCounterDataCollector * This,
            /* [in] */ AutoPathFormat format);
        
        DECLSPEC_XFGVIRT(IDataCollector, get_FileNameFormatPattern)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_FileNameFormatPattern )( 
            __RPC__in IPerformanceCounterDataCollector * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pattern);
        
        DECLSPEC_XFGVIRT(IDataCollector, put_FileNameFormatPattern)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_FileNameFormatPattern )( 
            __RPC__in IPerformanceCounterDataCollector * This,
            /* [in] */ __RPC__in BSTR pattern);
        
        DECLSPEC_XFGVIRT(IDataCollector, get_LatestOutputLocation)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_LatestOutputLocation )( 
            __RPC__in IPerformanceCounterDataCollector * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *path);
        
        DECLSPEC_XFGVIRT(IDataCollector, put_LatestOutputLocation)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_LatestOutputLocation )( 
            __RPC__in IPerformanceCounterDataCollector * This,
            /* [in] */ __RPC__in BSTR path);
        
        DECLSPEC_XFGVIRT(IDataCollector, get_LogAppend)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_LogAppend )( 
            __RPC__in IPerformanceCounterDataCollector * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *append);
        
        DECLSPEC_XFGVIRT(IDataCollector, put_LogAppend)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_LogAppend )( 
            __RPC__in IPerformanceCounterDataCollector * This,
            /* [in] */ VARIANT_BOOL append);
        
        DECLSPEC_XFGVIRT(IDataCollector, get_LogCircular)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_LogCircular )( 
            __RPC__in IPerformanceCounterDataCollector * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *circular);
        
        DECLSPEC_XFGVIRT(IDataCollector, put_LogCircular)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_LogCircular )( 
            __RPC__in IPerformanceCounterDataCollector * This,
            /* [in] */ VARIANT_BOOL circular);
        
        DECLSPEC_XFGVIRT(IDataCollector, get_LogOverwrite)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_LogOverwrite )( 
            __RPC__in IPerformanceCounterDataCollector * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *overwrite);
        
        DECLSPEC_XFGVIRT(IDataCollector, put_LogOverwrite)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_LogOverwrite )( 
            __RPC__in IPerformanceCounterDataCollector * This,
            /* [in] */ VARIANT_BOOL overwrite);
        
        DECLSPEC_XFGVIRT(IDataCollector, get_Name)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IPerformanceCounterDataCollector * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *name);
        
        DECLSPEC_XFGVIRT(IDataCollector, put_Name)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Name )( 
            __RPC__in IPerformanceCounterDataCollector * This,
            /* [in] */ __RPC__in BSTR name);
        
        DECLSPEC_XFGVIRT(IDataCollector, get_OutputLocation)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_OutputLocation )( 
            __RPC__in IPerformanceCounterDataCollector * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *path);
        
        DECLSPEC_XFGVIRT(IDataCollector, get_Index)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Index )( 
            __RPC__in IPerformanceCounterDataCollector * This,
            /* [retval][out] */ __RPC__out long *index);
        
        DECLSPEC_XFGVIRT(IDataCollector, put_Index)
        /* [restricted][hidden][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Index )( 
            __RPC__in IPerformanceCounterDataCollector * This,
            /* [in] */ long index);
        
        DECLSPEC_XFGVIRT(IDataCollector, get_Xml)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Xml )( 
            __RPC__in IPerformanceCounterDataCollector * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Xml);
        
        DECLSPEC_XFGVIRT(IDataCollector, SetXml)
        HRESULT ( STDMETHODCALLTYPE *SetXml )( 
            __RPC__in IPerformanceCounterDataCollector * This,
            /* [in] */ __RPC__in BSTR Xml,
            /* [retval][out] */ __RPC__deref_out_opt IValueMap **Validation);
        
        DECLSPEC_XFGVIRT(IDataCollector, CreateOutputLocation)
        HRESULT ( STDMETHODCALLTYPE *CreateOutputLocation )( 
            __RPC__in IPerformanceCounterDataCollector * This,
            /* [in] */ VARIANT_BOOL Latest,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Location);
        
        DECLSPEC_XFGVIRT(IPerformanceCounterDataCollector, get_DataSourceName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_DataSourceName )( 
            __RPC__in IPerformanceCounterDataCollector * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *dsn);
        
        DECLSPEC_XFGVIRT(IPerformanceCounterDataCollector, put_DataSourceName)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_DataSourceName )( 
            __RPC__in IPerformanceCounterDataCollector * This,
            /* [in] */ __RPC__in BSTR dsn);
        
        DECLSPEC_XFGVIRT(IPerformanceCounterDataCollector, get_PerformanceCounters)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_PerformanceCounters )( 
            __RPC__in IPerformanceCounterDataCollector * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *counters);
        
        DECLSPEC_XFGVIRT(IPerformanceCounterDataCollector, put_PerformanceCounters)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_PerformanceCounters )( 
            __RPC__in IPerformanceCounterDataCollector * This,
            /* [in] */ __RPC__in SAFEARRAY * counters);
        
        DECLSPEC_XFGVIRT(IPerformanceCounterDataCollector, get_LogFileFormat)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_LogFileFormat )( 
            __RPC__in IPerformanceCounterDataCollector * This,
            /* [retval][out] */ __RPC__out FileFormat *format);
        
        DECLSPEC_XFGVIRT(IPerformanceCounterDataCollector, put_LogFileFormat)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_LogFileFormat )( 
            __RPC__in IPerformanceCounterDataCollector * This,
            /* [in] */ FileFormat format);
        
        DECLSPEC_XFGVIRT(IPerformanceCounterDataCollector, get_SampleInterval)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_SampleInterval )( 
            __RPC__in IPerformanceCounterDataCollector * This,
            /* [retval][out] */ __RPC__out unsigned long *interval);
        
        DECLSPEC_XFGVIRT(IPerformanceCounterDataCollector, put_SampleInterval)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_SampleInterval )( 
            __RPC__in IPerformanceCounterDataCollector * This,
            /* [in] */ unsigned long interval);
        
        DECLSPEC_XFGVIRT(IPerformanceCounterDataCollector, get_SegmentMaxRecords)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_SegmentMaxRecords )( 
            __RPC__in IPerformanceCounterDataCollector * This,
            /* [retval][out] */ __RPC__out unsigned long *records);
        
        DECLSPEC_XFGVIRT(IPerformanceCounterDataCollector, put_SegmentMaxRecords)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_SegmentMaxRecords )( 
            __RPC__in IPerformanceCounterDataCollector * This,
            /* [in] */ unsigned long records);
        
        END_INTERFACE
    } IPerformanceCounterDataCollectorVtbl;

    interface IPerformanceCounterDataCollector
    {
        CONST_VTBL struct IPerformanceCounterDataCollectorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPerformanceCounterDataCollector_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPerformanceCounterDataCollector_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPerformanceCounterDataCollector_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPerformanceCounterDataCollector_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IPerformanceCounterDataCollector_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IPerformanceCounterDataCollector_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IPerformanceCounterDataCollector_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IPerformanceCounterDataCollector_get_DataCollectorSet(This,group)	\
    ( (This)->lpVtbl -> get_DataCollectorSet(This,group) ) 

#define IPerformanceCounterDataCollector_put_DataCollectorSet(This,group)	\
    ( (This)->lpVtbl -> put_DataCollectorSet(This,group) ) 

#define IPerformanceCounterDataCollector_get_DataCollectorType(This,type)	\
    ( (This)->lpVtbl -> get_DataCollectorType(This,type) ) 

#define IPerformanceCounterDataCollector_get_FileName(This,name)	\
    ( (This)->lpVtbl -> get_FileName(This,name) ) 

#define IPerformanceCounterDataCollector_put_FileName(This,name)	\
    ( (This)->lpVtbl -> put_FileName(This,name) ) 

#define IPerformanceCounterDataCollector_get_FileNameFormat(This,format)	\
    ( (This)->lpVtbl -> get_FileNameFormat(This,format) ) 

#define IPerformanceCounterDataCollector_put_FileNameFormat(This,format)	\
    ( (This)->lpVtbl -> put_FileNameFormat(This,format) ) 

#define IPerformanceCounterDataCollector_get_FileNameFormatPattern(This,pattern)	\
    ( (This)->lpVtbl -> get_FileNameFormatPattern(This,pattern) ) 

#define IPerformanceCounterDataCollector_put_FileNameFormatPattern(This,pattern)	\
    ( (This)->lpVtbl -> put_FileNameFormatPattern(This,pattern) ) 

#define IPerformanceCounterDataCollector_get_LatestOutputLocation(This,path)	\
    ( (This)->lpVtbl -> get_LatestOutputLocation(This,path) ) 

#define IPerformanceCounterDataCollector_put_LatestOutputLocation(This,path)	\
    ( (This)->lpVtbl -> put_LatestOutputLocation(This,path) ) 

#define IPerformanceCounterDataCollector_get_LogAppend(This,append)	\
    ( (This)->lpVtbl -> get_LogAppend(This,append) ) 

#define IPerformanceCounterDataCollector_put_LogAppend(This,append)	\
    ( (This)->lpVtbl -> put_LogAppend(This,append) ) 

#define IPerformanceCounterDataCollector_get_LogCircular(This,circular)	\
    ( (This)->lpVtbl -> get_LogCircular(This,circular) ) 

#define IPerformanceCounterDataCollector_put_LogCircular(This,circular)	\
    ( (This)->lpVtbl -> put_LogCircular(This,circular) ) 

#define IPerformanceCounterDataCollector_get_LogOverwrite(This,overwrite)	\
    ( (This)->lpVtbl -> get_LogOverwrite(This,overwrite) ) 

#define IPerformanceCounterDataCollector_put_LogOverwrite(This,overwrite)	\
    ( (This)->lpVtbl -> put_LogOverwrite(This,overwrite) ) 

#define IPerformanceCounterDataCollector_get_Name(This,name)	\
    ( (This)->lpVtbl -> get_Name(This,name) ) 

#define IPerformanceCounterDataCollector_put_Name(This,name)	\
    ( (This)->lpVtbl -> put_Name(This,name) ) 

#define IPerformanceCounterDataCollector_get_OutputLocation(This,path)	\
    ( (This)->lpVtbl -> get_OutputLocation(This,path) ) 

#define IPerformanceCounterDataCollector_get_Index(This,index)	\
    ( (This)->lpVtbl -> get_Index(This,index) ) 

#define IPerformanceCounterDataCollector_put_Index(This,index)	\
    ( (This)->lpVtbl -> put_Index(This,index) ) 

#define IPerformanceCounterDataCollector_get_Xml(This,Xml)	\
    ( (This)->lpVtbl -> get_Xml(This,Xml) ) 

#define IPerformanceCounterDataCollector_SetXml(This,Xml,Validation)	\
    ( (This)->lpVtbl -> SetXml(This,Xml,Validation) ) 

#define IPerformanceCounterDataCollector_CreateOutputLocation(This,Latest,Location)	\
    ( (This)->lpVtbl -> CreateOutputLocation(This,Latest,Location) ) 


#define IPerformanceCounterDataCollector_get_DataSourceName(This,dsn)	\
    ( (This)->lpVtbl -> get_DataSourceName(This,dsn) ) 

#define IPerformanceCounterDataCollector_put_DataSourceName(This,dsn)	\
    ( (This)->lpVtbl -> put_DataSourceName(This,dsn) ) 

#define IPerformanceCounterDataCollector_get_PerformanceCounters(This,counters)	\
    ( (This)->lpVtbl -> get_PerformanceCounters(This,counters) ) 

#define IPerformanceCounterDataCollector_put_PerformanceCounters(This,counters)	\
    ( (This)->lpVtbl -> put_PerformanceCounters(This,counters) ) 

#define IPerformanceCounterDataCollector_get_LogFileFormat(This,format)	\
    ( (This)->lpVtbl -> get_LogFileFormat(This,format) ) 

#define IPerformanceCounterDataCollector_put_LogFileFormat(This,format)	\
    ( (This)->lpVtbl -> put_LogFileFormat(This,format) ) 

#define IPerformanceCounterDataCollector_get_SampleInterval(This,interval)	\
    ( (This)->lpVtbl -> get_SampleInterval(This,interval) ) 

#define IPerformanceCounterDataCollector_put_SampleInterval(This,interval)	\
    ( (This)->lpVtbl -> put_SampleInterval(This,interval) ) 

#define IPerformanceCounterDataCollector_get_SegmentMaxRecords(This,records)	\
    ( (This)->lpVtbl -> get_SegmentMaxRecords(This,records) ) 

#define IPerformanceCounterDataCollector_put_SegmentMaxRecords(This,records)	\
    ( (This)->lpVtbl -> put_SegmentMaxRecords(This,records) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPerformanceCounterDataCollector_INTERFACE_DEFINED__ */


#ifndef __ITraceDataCollector_INTERFACE_DEFINED__
#define __ITraceDataCollector_INTERFACE_DEFINED__

/* interface ITraceDataCollector */
/* [dual][uuid][object] */ 


EXTERN_C const IID IID_ITraceDataCollector;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0383750b-098b-11d8-9414-505054503030")
    ITraceDataCollector : public IDataCollector
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_BufferSize( 
            /* [retval][out] */ __RPC__out unsigned long *size) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_BufferSize( 
            /* [in] */ unsigned long size) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_BuffersLost( 
            /* [retval][out] */ __RPC__out unsigned long *buffers) = 0;
        
        virtual /* [restricted][hidden][propput] */ HRESULT STDMETHODCALLTYPE put_BuffersLost( 
            /* [in] */ unsigned long buffers) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_BuffersWritten( 
            /* [retval][out] */ __RPC__out unsigned long *buffers) = 0;
        
        virtual /* [restricted][hidden][propput] */ HRESULT STDMETHODCALLTYPE put_BuffersWritten( 
            /* [in] */ unsigned long buffers) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ClockType( 
            /* [retval][out] */ __RPC__out ClockType *clock) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_ClockType( 
            /* [in] */ ClockType clock) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_EventsLost( 
            /* [retval][out] */ __RPC__out unsigned long *events) = 0;
        
        virtual /* [restricted][hidden][propput] */ HRESULT STDMETHODCALLTYPE put_EventsLost( 
            /* [in] */ unsigned long events) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ExtendedModes( 
            /* [retval][out] */ __RPC__out unsigned long *mode) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_ExtendedModes( 
            /* [in] */ unsigned long mode) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_FlushTimer( 
            /* [retval][out] */ __RPC__out unsigned long *seconds) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_FlushTimer( 
            /* [in] */ unsigned long seconds) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_FreeBuffers( 
            /* [retval][out] */ __RPC__out unsigned long *buffers) = 0;
        
        virtual /* [restricted][hidden][propput] */ HRESULT STDMETHODCALLTYPE put_FreeBuffers( 
            /* [in] */ unsigned long buffers) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Guid( 
            /* [retval][out] */ __RPC__out GUID *guid) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_Guid( 
            /* [in] */ GUID guid) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_IsKernelTrace( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *kernel) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_MaximumBuffers( 
            /* [retval][out] */ __RPC__out unsigned long *buffers) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_MaximumBuffers( 
            /* [in] */ unsigned long buffers) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_MinimumBuffers( 
            /* [retval][out] */ __RPC__out unsigned long *buffers) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_MinimumBuffers( 
            /* [in] */ unsigned long buffers) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_NumberOfBuffers( 
            /* [retval][out] */ __RPC__out unsigned long *buffers) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_NumberOfBuffers( 
            /* [in] */ unsigned long buffers) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_PreallocateFile( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *allocate) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_PreallocateFile( 
            /* [in] */ VARIANT_BOOL allocate) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ProcessMode( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *process) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_ProcessMode( 
            /* [in] */ VARIANT_BOOL process) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_RealTimeBuffersLost( 
            /* [retval][out] */ __RPC__out unsigned long *buffers) = 0;
        
        virtual /* [restricted][hidden][propput] */ HRESULT STDMETHODCALLTYPE put_RealTimeBuffersLost( 
            /* [in] */ unsigned long buffers) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_SessionId( 
            /* [retval][out] */ __RPC__out ULONG64 *id) = 0;
        
        virtual /* [restricted][hidden][propput] */ HRESULT STDMETHODCALLTYPE put_SessionId( 
            /* [in] */ ULONG64 id) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_SessionName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *name) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_SessionName( 
            /* [in] */ __RPC__in BSTR name) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_SessionThreadId( 
            /* [retval][out] */ __RPC__out unsigned long *tid) = 0;
        
        virtual /* [restricted][hidden][propput] */ HRESULT STDMETHODCALLTYPE put_SessionThreadId( 
            /* [in] */ unsigned long tid) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_StreamMode( 
            /* [retval][out] */ __RPC__out StreamMode *mode) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_StreamMode( 
            /* [in] */ StreamMode mode) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_TraceDataProviders( 
            /* [retval][out] */ __RPC__deref_out_opt ITraceDataProviderCollection **providers) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITraceDataCollectorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITraceDataCollector * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITraceDataCollector * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITraceDataCollector * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITraceDataCollector * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITraceDataCollector * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITraceDataCollector * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITraceDataCollector * This,
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
        
        DECLSPEC_XFGVIRT(IDataCollector, get_DataCollectorSet)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_DataCollectorSet )( 
            __RPC__in ITraceDataCollector * This,
            /* [retval][out] */ __RPC__deref_out_opt IDataCollectorSet **group);
        
        DECLSPEC_XFGVIRT(IDataCollector, put_DataCollectorSet)
        /* [restricted][hidden][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DataCollectorSet )( 
            __RPC__in ITraceDataCollector * This,
            /* [in] */ __RPC__in_opt IDataCollectorSet *group);
        
        DECLSPEC_XFGVIRT(IDataCollector, get_DataCollectorType)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_DataCollectorType )( 
            __RPC__in ITraceDataCollector * This,
            /* [retval][out] */ __RPC__out DataCollectorType *type);
        
        DECLSPEC_XFGVIRT(IDataCollector, get_FileName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_FileName )( 
            __RPC__in ITraceDataCollector * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *name);
        
        DECLSPEC_XFGVIRT(IDataCollector, put_FileName)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_FileName )( 
            __RPC__in ITraceDataCollector * This,
            /* [in] */ __RPC__in BSTR name);
        
        DECLSPEC_XFGVIRT(IDataCollector, get_FileNameFormat)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_FileNameFormat )( 
            __RPC__in ITraceDataCollector * This,
            /* [retval][out] */ __RPC__out AutoPathFormat *format);
        
        DECLSPEC_XFGVIRT(IDataCollector, put_FileNameFormat)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_FileNameFormat )( 
            __RPC__in ITraceDataCollector * This,
            /* [in] */ AutoPathFormat format);
        
        DECLSPEC_XFGVIRT(IDataCollector, get_FileNameFormatPattern)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_FileNameFormatPattern )( 
            __RPC__in ITraceDataCollector * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pattern);
        
        DECLSPEC_XFGVIRT(IDataCollector, put_FileNameFormatPattern)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_FileNameFormatPattern )( 
            __RPC__in ITraceDataCollector * This,
            /* [in] */ __RPC__in BSTR pattern);
        
        DECLSPEC_XFGVIRT(IDataCollector, get_LatestOutputLocation)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_LatestOutputLocation )( 
            __RPC__in ITraceDataCollector * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *path);
        
        DECLSPEC_XFGVIRT(IDataCollector, put_LatestOutputLocation)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_LatestOutputLocation )( 
            __RPC__in ITraceDataCollector * This,
            /* [in] */ __RPC__in BSTR path);
        
        DECLSPEC_XFGVIRT(IDataCollector, get_LogAppend)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_LogAppend )( 
            __RPC__in ITraceDataCollector * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *append);
        
        DECLSPEC_XFGVIRT(IDataCollector, put_LogAppend)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_LogAppend )( 
            __RPC__in ITraceDataCollector * This,
            /* [in] */ VARIANT_BOOL append);
        
        DECLSPEC_XFGVIRT(IDataCollector, get_LogCircular)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_LogCircular )( 
            __RPC__in ITraceDataCollector * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *circular);
        
        DECLSPEC_XFGVIRT(IDataCollector, put_LogCircular)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_LogCircular )( 
            __RPC__in ITraceDataCollector * This,
            /* [in] */ VARIANT_BOOL circular);
        
        DECLSPEC_XFGVIRT(IDataCollector, get_LogOverwrite)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_LogOverwrite )( 
            __RPC__in ITraceDataCollector * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *overwrite);
        
        DECLSPEC_XFGVIRT(IDataCollector, put_LogOverwrite)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_LogOverwrite )( 
            __RPC__in ITraceDataCollector * This,
            /* [in] */ VARIANT_BOOL overwrite);
        
        DECLSPEC_XFGVIRT(IDataCollector, get_Name)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in ITraceDataCollector * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *name);
        
        DECLSPEC_XFGVIRT(IDataCollector, put_Name)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Name )( 
            __RPC__in ITraceDataCollector * This,
            /* [in] */ __RPC__in BSTR name);
        
        DECLSPEC_XFGVIRT(IDataCollector, get_OutputLocation)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_OutputLocation )( 
            __RPC__in ITraceDataCollector * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *path);
        
        DECLSPEC_XFGVIRT(IDataCollector, get_Index)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Index )( 
            __RPC__in ITraceDataCollector * This,
            /* [retval][out] */ __RPC__out long *index);
        
        DECLSPEC_XFGVIRT(IDataCollector, put_Index)
        /* [restricted][hidden][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Index )( 
            __RPC__in ITraceDataCollector * This,
            /* [in] */ long index);
        
        DECLSPEC_XFGVIRT(IDataCollector, get_Xml)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Xml )( 
            __RPC__in ITraceDataCollector * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Xml);
        
        DECLSPEC_XFGVIRT(IDataCollector, SetXml)
        HRESULT ( STDMETHODCALLTYPE *SetXml )( 
            __RPC__in ITraceDataCollector * This,
            /* [in] */ __RPC__in BSTR Xml,
            /* [retval][out] */ __RPC__deref_out_opt IValueMap **Validation);
        
        DECLSPEC_XFGVIRT(IDataCollector, CreateOutputLocation)
        HRESULT ( STDMETHODCALLTYPE *CreateOutputLocation )( 
            __RPC__in ITraceDataCollector * This,
            /* [in] */ VARIANT_BOOL Latest,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Location);
        
        DECLSPEC_XFGVIRT(ITraceDataCollector, get_BufferSize)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_BufferSize )( 
            __RPC__in ITraceDataCollector * This,
            /* [retval][out] */ __RPC__out unsigned long *size);
        
        DECLSPEC_XFGVIRT(ITraceDataCollector, put_BufferSize)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_BufferSize )( 
            __RPC__in ITraceDataCollector * This,
            /* [in] */ unsigned long size);
        
        DECLSPEC_XFGVIRT(ITraceDataCollector, get_BuffersLost)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_BuffersLost )( 
            __RPC__in ITraceDataCollector * This,
            /* [retval][out] */ __RPC__out unsigned long *buffers);
        
        DECLSPEC_XFGVIRT(ITraceDataCollector, put_BuffersLost)
        /* [restricted][hidden][propput] */ HRESULT ( STDMETHODCALLTYPE *put_BuffersLost )( 
            __RPC__in ITraceDataCollector * This,
            /* [in] */ unsigned long buffers);
        
        DECLSPEC_XFGVIRT(ITraceDataCollector, get_BuffersWritten)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_BuffersWritten )( 
            __RPC__in ITraceDataCollector * This,
            /* [retval][out] */ __RPC__out unsigned long *buffers);
        
        DECLSPEC_XFGVIRT(ITraceDataCollector, put_BuffersWritten)
        /* [restricted][hidden][propput] */ HRESULT ( STDMETHODCALLTYPE *put_BuffersWritten )( 
            __RPC__in ITraceDataCollector * This,
            /* [in] */ unsigned long buffers);
        
        DECLSPEC_XFGVIRT(ITraceDataCollector, get_ClockType)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ClockType )( 
            __RPC__in ITraceDataCollector * This,
            /* [retval][out] */ __RPC__out ClockType *clock);
        
        DECLSPEC_XFGVIRT(ITraceDataCollector, put_ClockType)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ClockType )( 
            __RPC__in ITraceDataCollector * This,
            /* [in] */ ClockType clock);
        
        DECLSPEC_XFGVIRT(ITraceDataCollector, get_EventsLost)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_EventsLost )( 
            __RPC__in ITraceDataCollector * This,
            /* [retval][out] */ __RPC__out unsigned long *events);
        
        DECLSPEC_XFGVIRT(ITraceDataCollector, put_EventsLost)
        /* [restricted][hidden][propput] */ HRESULT ( STDMETHODCALLTYPE *put_EventsLost )( 
            __RPC__in ITraceDataCollector * This,
            /* [in] */ unsigned long events);
        
        DECLSPEC_XFGVIRT(ITraceDataCollector, get_ExtendedModes)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ExtendedModes )( 
            __RPC__in ITraceDataCollector * This,
            /* [retval][out] */ __RPC__out unsigned long *mode);
        
        DECLSPEC_XFGVIRT(ITraceDataCollector, put_ExtendedModes)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ExtendedModes )( 
            __RPC__in ITraceDataCollector * This,
            /* [in] */ unsigned long mode);
        
        DECLSPEC_XFGVIRT(ITraceDataCollector, get_FlushTimer)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_FlushTimer )( 
            __RPC__in ITraceDataCollector * This,
            /* [retval][out] */ __RPC__out unsigned long *seconds);
        
        DECLSPEC_XFGVIRT(ITraceDataCollector, put_FlushTimer)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_FlushTimer )( 
            __RPC__in ITraceDataCollector * This,
            /* [in] */ unsigned long seconds);
        
        DECLSPEC_XFGVIRT(ITraceDataCollector, get_FreeBuffers)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_FreeBuffers )( 
            __RPC__in ITraceDataCollector * This,
            /* [retval][out] */ __RPC__out unsigned long *buffers);
        
        DECLSPEC_XFGVIRT(ITraceDataCollector, put_FreeBuffers)
        /* [restricted][hidden][propput] */ HRESULT ( STDMETHODCALLTYPE *put_FreeBuffers )( 
            __RPC__in ITraceDataCollector * This,
            /* [in] */ unsigned long buffers);
        
        DECLSPEC_XFGVIRT(ITraceDataCollector, get_Guid)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Guid )( 
            __RPC__in ITraceDataCollector * This,
            /* [retval][out] */ __RPC__out GUID *guid);
        
        DECLSPEC_XFGVIRT(ITraceDataCollector, put_Guid)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Guid )( 
            __RPC__in ITraceDataCollector * This,
            /* [in] */ GUID guid);
        
        DECLSPEC_XFGVIRT(ITraceDataCollector, get_IsKernelTrace)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_IsKernelTrace )( 
            __RPC__in ITraceDataCollector * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *kernel);
        
        DECLSPEC_XFGVIRT(ITraceDataCollector, get_MaximumBuffers)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_MaximumBuffers )( 
            __RPC__in ITraceDataCollector * This,
            /* [retval][out] */ __RPC__out unsigned long *buffers);
        
        DECLSPEC_XFGVIRT(ITraceDataCollector, put_MaximumBuffers)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_MaximumBuffers )( 
            __RPC__in ITraceDataCollector * This,
            /* [in] */ unsigned long buffers);
        
        DECLSPEC_XFGVIRT(ITraceDataCollector, get_MinimumBuffers)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_MinimumBuffers )( 
            __RPC__in ITraceDataCollector * This,
            /* [retval][out] */ __RPC__out unsigned long *buffers);
        
        DECLSPEC_XFGVIRT(ITraceDataCollector, put_MinimumBuffers)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_MinimumBuffers )( 
            __RPC__in ITraceDataCollector * This,
            /* [in] */ unsigned long buffers);
        
        DECLSPEC_XFGVIRT(ITraceDataCollector, get_NumberOfBuffers)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_NumberOfBuffers )( 
            __RPC__in ITraceDataCollector * This,
            /* [retval][out] */ __RPC__out unsigned long *buffers);
        
        DECLSPEC_XFGVIRT(ITraceDataCollector, put_NumberOfBuffers)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_NumberOfBuffers )( 
            __RPC__in ITraceDataCollector * This,
            /* [in] */ unsigned long buffers);
        
        DECLSPEC_XFGVIRT(ITraceDataCollector, get_PreallocateFile)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_PreallocateFile )( 
            __RPC__in ITraceDataCollector * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *allocate);
        
        DECLSPEC_XFGVIRT(ITraceDataCollector, put_PreallocateFile)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_PreallocateFile )( 
            __RPC__in ITraceDataCollector * This,
            /* [in] */ VARIANT_BOOL allocate);
        
        DECLSPEC_XFGVIRT(ITraceDataCollector, get_ProcessMode)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProcessMode )( 
            __RPC__in ITraceDataCollector * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *process);
        
        DECLSPEC_XFGVIRT(ITraceDataCollector, put_ProcessMode)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ProcessMode )( 
            __RPC__in ITraceDataCollector * This,
            /* [in] */ VARIANT_BOOL process);
        
        DECLSPEC_XFGVIRT(ITraceDataCollector, get_RealTimeBuffersLost)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RealTimeBuffersLost )( 
            __RPC__in ITraceDataCollector * This,
            /* [retval][out] */ __RPC__out unsigned long *buffers);
        
        DECLSPEC_XFGVIRT(ITraceDataCollector, put_RealTimeBuffersLost)
        /* [restricted][hidden][propput] */ HRESULT ( STDMETHODCALLTYPE *put_RealTimeBuffersLost )( 
            __RPC__in ITraceDataCollector * This,
            /* [in] */ unsigned long buffers);
        
        DECLSPEC_XFGVIRT(ITraceDataCollector, get_SessionId)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_SessionId )( 
            __RPC__in ITraceDataCollector * This,
            /* [retval][out] */ __RPC__out ULONG64 *id);
        
        DECLSPEC_XFGVIRT(ITraceDataCollector, put_SessionId)
        /* [restricted][hidden][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SessionId )( 
            __RPC__in ITraceDataCollector * This,
            /* [in] */ ULONG64 id);
        
        DECLSPEC_XFGVIRT(ITraceDataCollector, get_SessionName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_SessionName )( 
            __RPC__in ITraceDataCollector * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *name);
        
        DECLSPEC_XFGVIRT(ITraceDataCollector, put_SessionName)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_SessionName )( 
            __RPC__in ITraceDataCollector * This,
            /* [in] */ __RPC__in BSTR name);
        
        DECLSPEC_XFGVIRT(ITraceDataCollector, get_SessionThreadId)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_SessionThreadId )( 
            __RPC__in ITraceDataCollector * This,
            /* [retval][out] */ __RPC__out unsigned long *tid);
        
        DECLSPEC_XFGVIRT(ITraceDataCollector, put_SessionThreadId)
        /* [restricted][hidden][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SessionThreadId )( 
            __RPC__in ITraceDataCollector * This,
            /* [in] */ unsigned long tid);
        
        DECLSPEC_XFGVIRT(ITraceDataCollector, get_StreamMode)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_StreamMode )( 
            __RPC__in ITraceDataCollector * This,
            /* [retval][out] */ __RPC__out StreamMode *mode);
        
        DECLSPEC_XFGVIRT(ITraceDataCollector, put_StreamMode)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_StreamMode )( 
            __RPC__in ITraceDataCollector * This,
            /* [in] */ StreamMode mode);
        
        DECLSPEC_XFGVIRT(ITraceDataCollector, get_TraceDataProviders)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_TraceDataProviders )( 
            __RPC__in ITraceDataCollector * This,
            /* [retval][out] */ __RPC__deref_out_opt ITraceDataProviderCollection **providers);
        
        END_INTERFACE
    } ITraceDataCollectorVtbl;

    interface ITraceDataCollector
    {
        CONST_VTBL struct ITraceDataCollectorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITraceDataCollector_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITraceDataCollector_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITraceDataCollector_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITraceDataCollector_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITraceDataCollector_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITraceDataCollector_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITraceDataCollector_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITraceDataCollector_get_DataCollectorSet(This,group)	\
    ( (This)->lpVtbl -> get_DataCollectorSet(This,group) ) 

#define ITraceDataCollector_put_DataCollectorSet(This,group)	\
    ( (This)->lpVtbl -> put_DataCollectorSet(This,group) ) 

#define ITraceDataCollector_get_DataCollectorType(This,type)	\
    ( (This)->lpVtbl -> get_DataCollectorType(This,type) ) 

#define ITraceDataCollector_get_FileName(This,name)	\
    ( (This)->lpVtbl -> get_FileName(This,name) ) 

#define ITraceDataCollector_put_FileName(This,name)	\
    ( (This)->lpVtbl -> put_FileName(This,name) ) 

#define ITraceDataCollector_get_FileNameFormat(This,format)	\
    ( (This)->lpVtbl -> get_FileNameFormat(This,format) ) 

#define ITraceDataCollector_put_FileNameFormat(This,format)	\
    ( (This)->lpVtbl -> put_FileNameFormat(This,format) ) 

#define ITraceDataCollector_get_FileNameFormatPattern(This,pattern)	\
    ( (This)->lpVtbl -> get_FileNameFormatPattern(This,pattern) ) 

#define ITraceDataCollector_put_FileNameFormatPattern(This,pattern)	\
    ( (This)->lpVtbl -> put_FileNameFormatPattern(This,pattern) ) 

#define ITraceDataCollector_get_LatestOutputLocation(This,path)	\
    ( (This)->lpVtbl -> get_LatestOutputLocation(This,path) ) 

#define ITraceDataCollector_put_LatestOutputLocation(This,path)	\
    ( (This)->lpVtbl -> put_LatestOutputLocation(This,path) ) 

#define ITraceDataCollector_get_LogAppend(This,append)	\
    ( (This)->lpVtbl -> get_LogAppend(This,append) ) 

#define ITraceDataCollector_put_LogAppend(This,append)	\
    ( (This)->lpVtbl -> put_LogAppend(This,append) ) 

#define ITraceDataCollector_get_LogCircular(This,circular)	\
    ( (This)->lpVtbl -> get_LogCircular(This,circular) ) 

#define ITraceDataCollector_put_LogCircular(This,circular)	\
    ( (This)->lpVtbl -> put_LogCircular(This,circular) ) 

#define ITraceDataCollector_get_LogOverwrite(This,overwrite)	\
    ( (This)->lpVtbl -> get_LogOverwrite(This,overwrite) ) 

#define ITraceDataCollector_put_LogOverwrite(This,overwrite)	\
    ( (This)->lpVtbl -> put_LogOverwrite(This,overwrite) ) 

#define ITraceDataCollector_get_Name(This,name)	\
    ( (This)->lpVtbl -> get_Name(This,name) ) 

#define ITraceDataCollector_put_Name(This,name)	\
    ( (This)->lpVtbl -> put_Name(This,name) ) 

#define ITraceDataCollector_get_OutputLocation(This,path)	\
    ( (This)->lpVtbl -> get_OutputLocation(This,path) ) 

#define ITraceDataCollector_get_Index(This,index)	\
    ( (This)->lpVtbl -> get_Index(This,index) ) 

#define ITraceDataCollector_put_Index(This,index)	\
    ( (This)->lpVtbl -> put_Index(This,index) ) 

#define ITraceDataCollector_get_Xml(This,Xml)	\
    ( (This)->lpVtbl -> get_Xml(This,Xml) ) 

#define ITraceDataCollector_SetXml(This,Xml,Validation)	\
    ( (This)->lpVtbl -> SetXml(This,Xml,Validation) ) 

#define ITraceDataCollector_CreateOutputLocation(This,Latest,Location)	\
    ( (This)->lpVtbl -> CreateOutputLocation(This,Latest,Location) ) 


#define ITraceDataCollector_get_BufferSize(This,size)	\
    ( (This)->lpVtbl -> get_BufferSize(This,size) ) 

#define ITraceDataCollector_put_BufferSize(This,size)	\
    ( (This)->lpVtbl -> put_BufferSize(This,size) ) 

#define ITraceDataCollector_get_BuffersLost(This,buffers)	\
    ( (This)->lpVtbl -> get_BuffersLost(This,buffers) ) 

#define ITraceDataCollector_put_BuffersLost(This,buffers)	\
    ( (This)->lpVtbl -> put_BuffersLost(This,buffers) ) 

#define ITraceDataCollector_get_BuffersWritten(This,buffers)	\
    ( (This)->lpVtbl -> get_BuffersWritten(This,buffers) ) 

#define ITraceDataCollector_put_BuffersWritten(This,buffers)	\
    ( (This)->lpVtbl -> put_BuffersWritten(This,buffers) ) 

#define ITraceDataCollector_get_ClockType(This,clock)	\
    ( (This)->lpVtbl -> get_ClockType(This,clock) ) 

#define ITraceDataCollector_put_ClockType(This,clock)	\
    ( (This)->lpVtbl -> put_ClockType(This,clock) ) 

#define ITraceDataCollector_get_EventsLost(This,events)	\
    ( (This)->lpVtbl -> get_EventsLost(This,events) ) 

#define ITraceDataCollector_put_EventsLost(This,events)	\
    ( (This)->lpVtbl -> put_EventsLost(This,events) ) 

#define ITraceDataCollector_get_ExtendedModes(This,mode)	\
    ( (This)->lpVtbl -> get_ExtendedModes(This,mode) ) 

#define ITraceDataCollector_put_ExtendedModes(This,mode)	\
    ( (This)->lpVtbl -> put_ExtendedModes(This,mode) ) 

#define ITraceDataCollector_get_FlushTimer(This,seconds)	\
    ( (This)->lpVtbl -> get_FlushTimer(This,seconds) ) 

#define ITraceDataCollector_put_FlushTimer(This,seconds)	\
    ( (This)->lpVtbl -> put_FlushTimer(This,seconds) ) 

#define ITraceDataCollector_get_FreeBuffers(This,buffers)	\
    ( (This)->lpVtbl -> get_FreeBuffers(This,buffers) ) 

#define ITraceDataCollector_put_FreeBuffers(This,buffers)	\
    ( (This)->lpVtbl -> put_FreeBuffers(This,buffers) ) 

#define ITraceDataCollector_get_Guid(This,guid)	\
    ( (This)->lpVtbl -> get_Guid(This,guid) ) 

#define ITraceDataCollector_put_Guid(This,guid)	\
    ( (This)->lpVtbl -> put_Guid(This,guid) ) 

#define ITraceDataCollector_get_IsKernelTrace(This,kernel)	\
    ( (This)->lpVtbl -> get_IsKernelTrace(This,kernel) ) 

#define ITraceDataCollector_get_MaximumBuffers(This,buffers)	\
    ( (This)->lpVtbl -> get_MaximumBuffers(This,buffers) ) 

#define ITraceDataCollector_put_MaximumBuffers(This,buffers)	\
    ( (This)->lpVtbl -> put_MaximumBuffers(This,buffers) ) 

#define ITraceDataCollector_get_MinimumBuffers(This,buffers)	\
    ( (This)->lpVtbl -> get_MinimumBuffers(This,buffers) ) 

#define ITraceDataCollector_put_MinimumBuffers(This,buffers)	\
    ( (This)->lpVtbl -> put_MinimumBuffers(This,buffers) ) 

#define ITraceDataCollector_get_NumberOfBuffers(This,buffers)	\
    ( (This)->lpVtbl -> get_NumberOfBuffers(This,buffers) ) 

#define ITraceDataCollector_put_NumberOfBuffers(This,buffers)	\
    ( (This)->lpVtbl -> put_NumberOfBuffers(This,buffers) ) 

#define ITraceDataCollector_get_PreallocateFile(This,allocate)	\
    ( (This)->lpVtbl -> get_PreallocateFile(This,allocate) ) 

#define ITraceDataCollector_put_PreallocateFile(This,allocate)	\
    ( (This)->lpVtbl -> put_PreallocateFile(This,allocate) ) 

#define ITraceDataCollector_get_ProcessMode(This,process)	\
    ( (This)->lpVtbl -> get_ProcessMode(This,process) ) 

#define ITraceDataCollector_put_ProcessMode(This,process)	\
    ( (This)->lpVtbl -> put_ProcessMode(This,process) ) 

#define ITraceDataCollector_get_RealTimeBuffersLost(This,buffers)	\
    ( (This)->lpVtbl -> get_RealTimeBuffersLost(This,buffers) ) 

#define ITraceDataCollector_put_RealTimeBuffersLost(This,buffers)	\
    ( (This)->lpVtbl -> put_RealTimeBuffersLost(This,buffers) ) 

#define ITraceDataCollector_get_SessionId(This,id)	\
    ( (This)->lpVtbl -> get_SessionId(This,id) ) 

#define ITraceDataCollector_put_SessionId(This,id)	\
    ( (This)->lpVtbl -> put_SessionId(This,id) ) 

#define ITraceDataCollector_get_SessionName(This,name)	\
    ( (This)->lpVtbl -> get_SessionName(This,name) ) 

#define ITraceDataCollector_put_SessionName(This,name)	\
    ( (This)->lpVtbl -> put_SessionName(This,name) ) 

#define ITraceDataCollector_get_SessionThreadId(This,tid)	\
    ( (This)->lpVtbl -> get_SessionThreadId(This,tid) ) 

#define ITraceDataCollector_put_SessionThreadId(This,tid)	\
    ( (This)->lpVtbl -> put_SessionThreadId(This,tid) ) 

#define ITraceDataCollector_get_StreamMode(This,mode)	\
    ( (This)->lpVtbl -> get_StreamMode(This,mode) ) 

#define ITraceDataCollector_put_StreamMode(This,mode)	\
    ( (This)->lpVtbl -> put_StreamMode(This,mode) ) 

#define ITraceDataCollector_get_TraceDataProviders(This,providers)	\
    ( (This)->lpVtbl -> get_TraceDataProviders(This,providers) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITraceDataCollector_INTERFACE_DEFINED__ */


#ifndef __IConfigurationDataCollector_INTERFACE_DEFINED__
#define __IConfigurationDataCollector_INTERFACE_DEFINED__

/* interface IConfigurationDataCollector */
/* [dual][uuid][object] */ 


EXTERN_C const IID IID_IConfigurationDataCollector;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("03837514-098b-11d8-9414-505054503030")
    IConfigurationDataCollector : public IDataCollector
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_FileMaxCount( 
            /* [retval][out] */ __RPC__out unsigned long *count) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_FileMaxCount( 
            /* [in] */ unsigned long count) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_FileMaxRecursiveDepth( 
            /* [retval][out] */ __RPC__out unsigned long *depth) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_FileMaxRecursiveDepth( 
            /* [in] */ unsigned long depth) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_FileMaxTotalSize( 
            /* [retval][out] */ __RPC__out unsigned long *size) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_FileMaxTotalSize( 
            /* [in] */ unsigned long size) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Files( 
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *Files) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_Files( 
            /* [in] */ __RPC__in SAFEARRAY * Files) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ManagementQueries( 
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *Queries) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_ManagementQueries( 
            /* [in] */ __RPC__in SAFEARRAY * Queries) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_QueryNetworkAdapters( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *network) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_QueryNetworkAdapters( 
            /* [in] */ VARIANT_BOOL network) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_RegistryKeys( 
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *query) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_RegistryKeys( 
            /* [in] */ __RPC__in SAFEARRAY * query) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_RegistryMaxRecursiveDepth( 
            /* [retval][out] */ __RPC__out unsigned long *depth) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_RegistryMaxRecursiveDepth( 
            /* [in] */ unsigned long depth) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_SystemStateFile( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *FileName) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_SystemStateFile( 
            /* [in] */ __RPC__in BSTR FileName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IConfigurationDataCollectorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IConfigurationDataCollector * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IConfigurationDataCollector * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IConfigurationDataCollector * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IConfigurationDataCollector * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IConfigurationDataCollector * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IConfigurationDataCollector * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IConfigurationDataCollector * This,
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
        
        DECLSPEC_XFGVIRT(IDataCollector, get_DataCollectorSet)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_DataCollectorSet )( 
            __RPC__in IConfigurationDataCollector * This,
            /* [retval][out] */ __RPC__deref_out_opt IDataCollectorSet **group);
        
        DECLSPEC_XFGVIRT(IDataCollector, put_DataCollectorSet)
        /* [restricted][hidden][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DataCollectorSet )( 
            __RPC__in IConfigurationDataCollector * This,
            /* [in] */ __RPC__in_opt IDataCollectorSet *group);
        
        DECLSPEC_XFGVIRT(IDataCollector, get_DataCollectorType)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_DataCollectorType )( 
            __RPC__in IConfigurationDataCollector * This,
            /* [retval][out] */ __RPC__out DataCollectorType *type);
        
        DECLSPEC_XFGVIRT(IDataCollector, get_FileName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_FileName )( 
            __RPC__in IConfigurationDataCollector * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *name);
        
        DECLSPEC_XFGVIRT(IDataCollector, put_FileName)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_FileName )( 
            __RPC__in IConfigurationDataCollector * This,
            /* [in] */ __RPC__in BSTR name);
        
        DECLSPEC_XFGVIRT(IDataCollector, get_FileNameFormat)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_FileNameFormat )( 
            __RPC__in IConfigurationDataCollector * This,
            /* [retval][out] */ __RPC__out AutoPathFormat *format);
        
        DECLSPEC_XFGVIRT(IDataCollector, put_FileNameFormat)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_FileNameFormat )( 
            __RPC__in IConfigurationDataCollector * This,
            /* [in] */ AutoPathFormat format);
        
        DECLSPEC_XFGVIRT(IDataCollector, get_FileNameFormatPattern)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_FileNameFormatPattern )( 
            __RPC__in IConfigurationDataCollector * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pattern);
        
        DECLSPEC_XFGVIRT(IDataCollector, put_FileNameFormatPattern)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_FileNameFormatPattern )( 
            __RPC__in IConfigurationDataCollector * This,
            /* [in] */ __RPC__in BSTR pattern);
        
        DECLSPEC_XFGVIRT(IDataCollector, get_LatestOutputLocation)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_LatestOutputLocation )( 
            __RPC__in IConfigurationDataCollector * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *path);
        
        DECLSPEC_XFGVIRT(IDataCollector, put_LatestOutputLocation)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_LatestOutputLocation )( 
            __RPC__in IConfigurationDataCollector * This,
            /* [in] */ __RPC__in BSTR path);
        
        DECLSPEC_XFGVIRT(IDataCollector, get_LogAppend)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_LogAppend )( 
            __RPC__in IConfigurationDataCollector * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *append);
        
        DECLSPEC_XFGVIRT(IDataCollector, put_LogAppend)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_LogAppend )( 
            __RPC__in IConfigurationDataCollector * This,
            /* [in] */ VARIANT_BOOL append);
        
        DECLSPEC_XFGVIRT(IDataCollector, get_LogCircular)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_LogCircular )( 
            __RPC__in IConfigurationDataCollector * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *circular);
        
        DECLSPEC_XFGVIRT(IDataCollector, put_LogCircular)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_LogCircular )( 
            __RPC__in IConfigurationDataCollector * This,
            /* [in] */ VARIANT_BOOL circular);
        
        DECLSPEC_XFGVIRT(IDataCollector, get_LogOverwrite)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_LogOverwrite )( 
            __RPC__in IConfigurationDataCollector * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *overwrite);
        
        DECLSPEC_XFGVIRT(IDataCollector, put_LogOverwrite)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_LogOverwrite )( 
            __RPC__in IConfigurationDataCollector * This,
            /* [in] */ VARIANT_BOOL overwrite);
        
        DECLSPEC_XFGVIRT(IDataCollector, get_Name)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IConfigurationDataCollector * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *name);
        
        DECLSPEC_XFGVIRT(IDataCollector, put_Name)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Name )( 
            __RPC__in IConfigurationDataCollector * This,
            /* [in] */ __RPC__in BSTR name);
        
        DECLSPEC_XFGVIRT(IDataCollector, get_OutputLocation)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_OutputLocation )( 
            __RPC__in IConfigurationDataCollector * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *path);
        
        DECLSPEC_XFGVIRT(IDataCollector, get_Index)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Index )( 
            __RPC__in IConfigurationDataCollector * This,
            /* [retval][out] */ __RPC__out long *index);
        
        DECLSPEC_XFGVIRT(IDataCollector, put_Index)
        /* [restricted][hidden][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Index )( 
            __RPC__in IConfigurationDataCollector * This,
            /* [in] */ long index);
        
        DECLSPEC_XFGVIRT(IDataCollector, get_Xml)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Xml )( 
            __RPC__in IConfigurationDataCollector * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Xml);
        
        DECLSPEC_XFGVIRT(IDataCollector, SetXml)
        HRESULT ( STDMETHODCALLTYPE *SetXml )( 
            __RPC__in IConfigurationDataCollector * This,
            /* [in] */ __RPC__in BSTR Xml,
            /* [retval][out] */ __RPC__deref_out_opt IValueMap **Validation);
        
        DECLSPEC_XFGVIRT(IDataCollector, CreateOutputLocation)
        HRESULT ( STDMETHODCALLTYPE *CreateOutputLocation )( 
            __RPC__in IConfigurationDataCollector * This,
            /* [in] */ VARIANT_BOOL Latest,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Location);
        
        DECLSPEC_XFGVIRT(IConfigurationDataCollector, get_FileMaxCount)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_FileMaxCount )( 
            __RPC__in IConfigurationDataCollector * This,
            /* [retval][out] */ __RPC__out unsigned long *count);
        
        DECLSPEC_XFGVIRT(IConfigurationDataCollector, put_FileMaxCount)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_FileMaxCount )( 
            __RPC__in IConfigurationDataCollector * This,
            /* [in] */ unsigned long count);
        
        DECLSPEC_XFGVIRT(IConfigurationDataCollector, get_FileMaxRecursiveDepth)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_FileMaxRecursiveDepth )( 
            __RPC__in IConfigurationDataCollector * This,
            /* [retval][out] */ __RPC__out unsigned long *depth);
        
        DECLSPEC_XFGVIRT(IConfigurationDataCollector, put_FileMaxRecursiveDepth)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_FileMaxRecursiveDepth )( 
            __RPC__in IConfigurationDataCollector * This,
            /* [in] */ unsigned long depth);
        
        DECLSPEC_XFGVIRT(IConfigurationDataCollector, get_FileMaxTotalSize)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_FileMaxTotalSize )( 
            __RPC__in IConfigurationDataCollector * This,
            /* [retval][out] */ __RPC__out unsigned long *size);
        
        DECLSPEC_XFGVIRT(IConfigurationDataCollector, put_FileMaxTotalSize)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_FileMaxTotalSize )( 
            __RPC__in IConfigurationDataCollector * This,
            /* [in] */ unsigned long size);
        
        DECLSPEC_XFGVIRT(IConfigurationDataCollector, get_Files)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Files )( 
            __RPC__in IConfigurationDataCollector * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *Files);
        
        DECLSPEC_XFGVIRT(IConfigurationDataCollector, put_Files)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Files )( 
            __RPC__in IConfigurationDataCollector * This,
            /* [in] */ __RPC__in SAFEARRAY * Files);
        
        DECLSPEC_XFGVIRT(IConfigurationDataCollector, get_ManagementQueries)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ManagementQueries )( 
            __RPC__in IConfigurationDataCollector * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *Queries);
        
        DECLSPEC_XFGVIRT(IConfigurationDataCollector, put_ManagementQueries)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ManagementQueries )( 
            __RPC__in IConfigurationDataCollector * This,
            /* [in] */ __RPC__in SAFEARRAY * Queries);
        
        DECLSPEC_XFGVIRT(IConfigurationDataCollector, get_QueryNetworkAdapters)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_QueryNetworkAdapters )( 
            __RPC__in IConfigurationDataCollector * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *network);
        
        DECLSPEC_XFGVIRT(IConfigurationDataCollector, put_QueryNetworkAdapters)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_QueryNetworkAdapters )( 
            __RPC__in IConfigurationDataCollector * This,
            /* [in] */ VARIANT_BOOL network);
        
        DECLSPEC_XFGVIRT(IConfigurationDataCollector, get_RegistryKeys)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RegistryKeys )( 
            __RPC__in IConfigurationDataCollector * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *query);
        
        DECLSPEC_XFGVIRT(IConfigurationDataCollector, put_RegistryKeys)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_RegistryKeys )( 
            __RPC__in IConfigurationDataCollector * This,
            /* [in] */ __RPC__in SAFEARRAY * query);
        
        DECLSPEC_XFGVIRT(IConfigurationDataCollector, get_RegistryMaxRecursiveDepth)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RegistryMaxRecursiveDepth )( 
            __RPC__in IConfigurationDataCollector * This,
            /* [retval][out] */ __RPC__out unsigned long *depth);
        
        DECLSPEC_XFGVIRT(IConfigurationDataCollector, put_RegistryMaxRecursiveDepth)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_RegistryMaxRecursiveDepth )( 
            __RPC__in IConfigurationDataCollector * This,
            /* [in] */ unsigned long depth);
        
        DECLSPEC_XFGVIRT(IConfigurationDataCollector, get_SystemStateFile)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_SystemStateFile )( 
            __RPC__in IConfigurationDataCollector * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *FileName);
        
        DECLSPEC_XFGVIRT(IConfigurationDataCollector, put_SystemStateFile)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_SystemStateFile )( 
            __RPC__in IConfigurationDataCollector * This,
            /* [in] */ __RPC__in BSTR FileName);
        
        END_INTERFACE
    } IConfigurationDataCollectorVtbl;

    interface IConfigurationDataCollector
    {
        CONST_VTBL struct IConfigurationDataCollectorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IConfigurationDataCollector_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IConfigurationDataCollector_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IConfigurationDataCollector_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IConfigurationDataCollector_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IConfigurationDataCollector_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IConfigurationDataCollector_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IConfigurationDataCollector_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IConfigurationDataCollector_get_DataCollectorSet(This,group)	\
    ( (This)->lpVtbl -> get_DataCollectorSet(This,group) ) 

#define IConfigurationDataCollector_put_DataCollectorSet(This,group)	\
    ( (This)->lpVtbl -> put_DataCollectorSet(This,group) ) 

#define IConfigurationDataCollector_get_DataCollectorType(This,type)	\
    ( (This)->lpVtbl -> get_DataCollectorType(This,type) ) 

#define IConfigurationDataCollector_get_FileName(This,name)	\
    ( (This)->lpVtbl -> get_FileName(This,name) ) 

#define IConfigurationDataCollector_put_FileName(This,name)	\
    ( (This)->lpVtbl -> put_FileName(This,name) ) 

#define IConfigurationDataCollector_get_FileNameFormat(This,format)	\
    ( (This)->lpVtbl -> get_FileNameFormat(This,format) ) 

#define IConfigurationDataCollector_put_FileNameFormat(This,format)	\
    ( (This)->lpVtbl -> put_FileNameFormat(This,format) ) 

#define IConfigurationDataCollector_get_FileNameFormatPattern(This,pattern)	\
    ( (This)->lpVtbl -> get_FileNameFormatPattern(This,pattern) ) 

#define IConfigurationDataCollector_put_FileNameFormatPattern(This,pattern)	\
    ( (This)->lpVtbl -> put_FileNameFormatPattern(This,pattern) ) 

#define IConfigurationDataCollector_get_LatestOutputLocation(This,path)	\
    ( (This)->lpVtbl -> get_LatestOutputLocation(This,path) ) 

#define IConfigurationDataCollector_put_LatestOutputLocation(This,path)	\
    ( (This)->lpVtbl -> put_LatestOutputLocation(This,path) ) 

#define IConfigurationDataCollector_get_LogAppend(This,append)	\
    ( (This)->lpVtbl -> get_LogAppend(This,append) ) 

#define IConfigurationDataCollector_put_LogAppend(This,append)	\
    ( (This)->lpVtbl -> put_LogAppend(This,append) ) 

#define IConfigurationDataCollector_get_LogCircular(This,circular)	\
    ( (This)->lpVtbl -> get_LogCircular(This,circular) ) 

#define IConfigurationDataCollector_put_LogCircular(This,circular)	\
    ( (This)->lpVtbl -> put_LogCircular(This,circular) ) 

#define IConfigurationDataCollector_get_LogOverwrite(This,overwrite)	\
    ( (This)->lpVtbl -> get_LogOverwrite(This,overwrite) ) 

#define IConfigurationDataCollector_put_LogOverwrite(This,overwrite)	\
    ( (This)->lpVtbl -> put_LogOverwrite(This,overwrite) ) 

#define IConfigurationDataCollector_get_Name(This,name)	\
    ( (This)->lpVtbl -> get_Name(This,name) ) 

#define IConfigurationDataCollector_put_Name(This,name)	\
    ( (This)->lpVtbl -> put_Name(This,name) ) 

#define IConfigurationDataCollector_get_OutputLocation(This,path)	\
    ( (This)->lpVtbl -> get_OutputLocation(This,path) ) 

#define IConfigurationDataCollector_get_Index(This,index)	\
    ( (This)->lpVtbl -> get_Index(This,index) ) 

#define IConfigurationDataCollector_put_Index(This,index)	\
    ( (This)->lpVtbl -> put_Index(This,index) ) 

#define IConfigurationDataCollector_get_Xml(This,Xml)	\
    ( (This)->lpVtbl -> get_Xml(This,Xml) ) 

#define IConfigurationDataCollector_SetXml(This,Xml,Validation)	\
    ( (This)->lpVtbl -> SetXml(This,Xml,Validation) ) 

#define IConfigurationDataCollector_CreateOutputLocation(This,Latest,Location)	\
    ( (This)->lpVtbl -> CreateOutputLocation(This,Latest,Location) ) 


#define IConfigurationDataCollector_get_FileMaxCount(This,count)	\
    ( (This)->lpVtbl -> get_FileMaxCount(This,count) ) 

#define IConfigurationDataCollector_put_FileMaxCount(This,count)	\
    ( (This)->lpVtbl -> put_FileMaxCount(This,count) ) 

#define IConfigurationDataCollector_get_FileMaxRecursiveDepth(This,depth)	\
    ( (This)->lpVtbl -> get_FileMaxRecursiveDepth(This,depth) ) 

#define IConfigurationDataCollector_put_FileMaxRecursiveDepth(This,depth)	\
    ( (This)->lpVtbl -> put_FileMaxRecursiveDepth(This,depth) ) 

#define IConfigurationDataCollector_get_FileMaxTotalSize(This,size)	\
    ( (This)->lpVtbl -> get_FileMaxTotalSize(This,size) ) 

#define IConfigurationDataCollector_put_FileMaxTotalSize(This,size)	\
    ( (This)->lpVtbl -> put_FileMaxTotalSize(This,size) ) 

#define IConfigurationDataCollector_get_Files(This,Files)	\
    ( (This)->lpVtbl -> get_Files(This,Files) ) 

#define IConfigurationDataCollector_put_Files(This,Files)	\
    ( (This)->lpVtbl -> put_Files(This,Files) ) 

#define IConfigurationDataCollector_get_ManagementQueries(This,Queries)	\
    ( (This)->lpVtbl -> get_ManagementQueries(This,Queries) ) 

#define IConfigurationDataCollector_put_ManagementQueries(This,Queries)	\
    ( (This)->lpVtbl -> put_ManagementQueries(This,Queries) ) 

#define IConfigurationDataCollector_get_QueryNetworkAdapters(This,network)	\
    ( (This)->lpVtbl -> get_QueryNetworkAdapters(This,network) ) 

#define IConfigurationDataCollector_put_QueryNetworkAdapters(This,network)	\
    ( (This)->lpVtbl -> put_QueryNetworkAdapters(This,network) ) 

#define IConfigurationDataCollector_get_RegistryKeys(This,query)	\
    ( (This)->lpVtbl -> get_RegistryKeys(This,query) ) 

#define IConfigurationDataCollector_put_RegistryKeys(This,query)	\
    ( (This)->lpVtbl -> put_RegistryKeys(This,query) ) 

#define IConfigurationDataCollector_get_RegistryMaxRecursiveDepth(This,depth)	\
    ( (This)->lpVtbl -> get_RegistryMaxRecursiveDepth(This,depth) ) 

#define IConfigurationDataCollector_put_RegistryMaxRecursiveDepth(This,depth)	\
    ( (This)->lpVtbl -> put_RegistryMaxRecursiveDepth(This,depth) ) 

#define IConfigurationDataCollector_get_SystemStateFile(This,FileName)	\
    ( (This)->lpVtbl -> get_SystemStateFile(This,FileName) ) 

#define IConfigurationDataCollector_put_SystemStateFile(This,FileName)	\
    ( (This)->lpVtbl -> put_SystemStateFile(This,FileName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IConfigurationDataCollector_INTERFACE_DEFINED__ */


#ifndef __IAlertDataCollector_INTERFACE_DEFINED__
#define __IAlertDataCollector_INTERFACE_DEFINED__

/* interface IAlertDataCollector */
/* [dual][uuid][object] */ 


EXTERN_C const IID IID_IAlertDataCollector;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("03837516-098b-11d8-9414-505054503030")
    IAlertDataCollector : public IDataCollector
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_AlertThresholds( 
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *alerts) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_AlertThresholds( 
            /* [in] */ __RPC__in SAFEARRAY * alerts) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_EventLog( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *log) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_EventLog( 
            /* [in] */ VARIANT_BOOL log) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_SampleInterval( 
            /* [retval][out] */ __RPC__out unsigned long *interval) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_SampleInterval( 
            /* [in] */ unsigned long interval) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Task( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *task) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_Task( 
            /* [in] */ __RPC__in BSTR task) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_TaskRunAsSelf( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *RunAsSelf) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_TaskRunAsSelf( 
            /* [in] */ VARIANT_BOOL RunAsSelf) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_TaskArguments( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *task) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_TaskArguments( 
            /* [in] */ __RPC__in BSTR task) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_TaskUserTextArguments( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *task) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_TaskUserTextArguments( 
            /* [in] */ __RPC__in BSTR task) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_TriggerDataCollectorSet( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *name) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_TriggerDataCollectorSet( 
            /* [in] */ __RPC__in BSTR name) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAlertDataCollectorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAlertDataCollector * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAlertDataCollector * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAlertDataCollector * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IAlertDataCollector * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IAlertDataCollector * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IAlertDataCollector * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IAlertDataCollector * This,
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
        
        DECLSPEC_XFGVIRT(IDataCollector, get_DataCollectorSet)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_DataCollectorSet )( 
            __RPC__in IAlertDataCollector * This,
            /* [retval][out] */ __RPC__deref_out_opt IDataCollectorSet **group);
        
        DECLSPEC_XFGVIRT(IDataCollector, put_DataCollectorSet)
        /* [restricted][hidden][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DataCollectorSet )( 
            __RPC__in IAlertDataCollector * This,
            /* [in] */ __RPC__in_opt IDataCollectorSet *group);
        
        DECLSPEC_XFGVIRT(IDataCollector, get_DataCollectorType)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_DataCollectorType )( 
            __RPC__in IAlertDataCollector * This,
            /* [retval][out] */ __RPC__out DataCollectorType *type);
        
        DECLSPEC_XFGVIRT(IDataCollector, get_FileName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_FileName )( 
            __RPC__in IAlertDataCollector * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *name);
        
        DECLSPEC_XFGVIRT(IDataCollector, put_FileName)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_FileName )( 
            __RPC__in IAlertDataCollector * This,
            /* [in] */ __RPC__in BSTR name);
        
        DECLSPEC_XFGVIRT(IDataCollector, get_FileNameFormat)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_FileNameFormat )( 
            __RPC__in IAlertDataCollector * This,
            /* [retval][out] */ __RPC__out AutoPathFormat *format);
        
        DECLSPEC_XFGVIRT(IDataCollector, put_FileNameFormat)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_FileNameFormat )( 
            __RPC__in IAlertDataCollector * This,
            /* [in] */ AutoPathFormat format);
        
        DECLSPEC_XFGVIRT(IDataCollector, get_FileNameFormatPattern)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_FileNameFormatPattern )( 
            __RPC__in IAlertDataCollector * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pattern);
        
        DECLSPEC_XFGVIRT(IDataCollector, put_FileNameFormatPattern)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_FileNameFormatPattern )( 
            __RPC__in IAlertDataCollector * This,
            /* [in] */ __RPC__in BSTR pattern);
        
        DECLSPEC_XFGVIRT(IDataCollector, get_LatestOutputLocation)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_LatestOutputLocation )( 
            __RPC__in IAlertDataCollector * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *path);
        
        DECLSPEC_XFGVIRT(IDataCollector, put_LatestOutputLocation)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_LatestOutputLocation )( 
            __RPC__in IAlertDataCollector * This,
            /* [in] */ __RPC__in BSTR path);
        
        DECLSPEC_XFGVIRT(IDataCollector, get_LogAppend)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_LogAppend )( 
            __RPC__in IAlertDataCollector * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *append);
        
        DECLSPEC_XFGVIRT(IDataCollector, put_LogAppend)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_LogAppend )( 
            __RPC__in IAlertDataCollector * This,
            /* [in] */ VARIANT_BOOL append);
        
        DECLSPEC_XFGVIRT(IDataCollector, get_LogCircular)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_LogCircular )( 
            __RPC__in IAlertDataCollector * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *circular);
        
        DECLSPEC_XFGVIRT(IDataCollector, put_LogCircular)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_LogCircular )( 
            __RPC__in IAlertDataCollector * This,
            /* [in] */ VARIANT_BOOL circular);
        
        DECLSPEC_XFGVIRT(IDataCollector, get_LogOverwrite)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_LogOverwrite )( 
            __RPC__in IAlertDataCollector * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *overwrite);
        
        DECLSPEC_XFGVIRT(IDataCollector, put_LogOverwrite)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_LogOverwrite )( 
            __RPC__in IAlertDataCollector * This,
            /* [in] */ VARIANT_BOOL overwrite);
        
        DECLSPEC_XFGVIRT(IDataCollector, get_Name)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IAlertDataCollector * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *name);
        
        DECLSPEC_XFGVIRT(IDataCollector, put_Name)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Name )( 
            __RPC__in IAlertDataCollector * This,
            /* [in] */ __RPC__in BSTR name);
        
        DECLSPEC_XFGVIRT(IDataCollector, get_OutputLocation)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_OutputLocation )( 
            __RPC__in IAlertDataCollector * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *path);
        
        DECLSPEC_XFGVIRT(IDataCollector, get_Index)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Index )( 
            __RPC__in IAlertDataCollector * This,
            /* [retval][out] */ __RPC__out long *index);
        
        DECLSPEC_XFGVIRT(IDataCollector, put_Index)
        /* [restricted][hidden][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Index )( 
            __RPC__in IAlertDataCollector * This,
            /* [in] */ long index);
        
        DECLSPEC_XFGVIRT(IDataCollector, get_Xml)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Xml )( 
            __RPC__in IAlertDataCollector * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Xml);
        
        DECLSPEC_XFGVIRT(IDataCollector, SetXml)
        HRESULT ( STDMETHODCALLTYPE *SetXml )( 
            __RPC__in IAlertDataCollector * This,
            /* [in] */ __RPC__in BSTR Xml,
            /* [retval][out] */ __RPC__deref_out_opt IValueMap **Validation);
        
        DECLSPEC_XFGVIRT(IDataCollector, CreateOutputLocation)
        HRESULT ( STDMETHODCALLTYPE *CreateOutputLocation )( 
            __RPC__in IAlertDataCollector * This,
            /* [in] */ VARIANT_BOOL Latest,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Location);
        
        DECLSPEC_XFGVIRT(IAlertDataCollector, get_AlertThresholds)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_AlertThresholds )( 
            __RPC__in IAlertDataCollector * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *alerts);
        
        DECLSPEC_XFGVIRT(IAlertDataCollector, put_AlertThresholds)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_AlertThresholds )( 
            __RPC__in IAlertDataCollector * This,
            /* [in] */ __RPC__in SAFEARRAY * alerts);
        
        DECLSPEC_XFGVIRT(IAlertDataCollector, get_EventLog)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_EventLog )( 
            __RPC__in IAlertDataCollector * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *log);
        
        DECLSPEC_XFGVIRT(IAlertDataCollector, put_EventLog)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_EventLog )( 
            __RPC__in IAlertDataCollector * This,
            /* [in] */ VARIANT_BOOL log);
        
        DECLSPEC_XFGVIRT(IAlertDataCollector, get_SampleInterval)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_SampleInterval )( 
            __RPC__in IAlertDataCollector * This,
            /* [retval][out] */ __RPC__out unsigned long *interval);
        
        DECLSPEC_XFGVIRT(IAlertDataCollector, put_SampleInterval)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_SampleInterval )( 
            __RPC__in IAlertDataCollector * This,
            /* [in] */ unsigned long interval);
        
        DECLSPEC_XFGVIRT(IAlertDataCollector, get_Task)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Task )( 
            __RPC__in IAlertDataCollector * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *task);
        
        DECLSPEC_XFGVIRT(IAlertDataCollector, put_Task)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Task )( 
            __RPC__in IAlertDataCollector * This,
            /* [in] */ __RPC__in BSTR task);
        
        DECLSPEC_XFGVIRT(IAlertDataCollector, get_TaskRunAsSelf)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_TaskRunAsSelf )( 
            __RPC__in IAlertDataCollector * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *RunAsSelf);
        
        DECLSPEC_XFGVIRT(IAlertDataCollector, put_TaskRunAsSelf)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_TaskRunAsSelf )( 
            __RPC__in IAlertDataCollector * This,
            /* [in] */ VARIANT_BOOL RunAsSelf);
        
        DECLSPEC_XFGVIRT(IAlertDataCollector, get_TaskArguments)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_TaskArguments )( 
            __RPC__in IAlertDataCollector * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *task);
        
        DECLSPEC_XFGVIRT(IAlertDataCollector, put_TaskArguments)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_TaskArguments )( 
            __RPC__in IAlertDataCollector * This,
            /* [in] */ __RPC__in BSTR task);
        
        DECLSPEC_XFGVIRT(IAlertDataCollector, get_TaskUserTextArguments)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_TaskUserTextArguments )( 
            __RPC__in IAlertDataCollector * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *task);
        
        DECLSPEC_XFGVIRT(IAlertDataCollector, put_TaskUserTextArguments)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_TaskUserTextArguments )( 
            __RPC__in IAlertDataCollector * This,
            /* [in] */ __RPC__in BSTR task);
        
        DECLSPEC_XFGVIRT(IAlertDataCollector, get_TriggerDataCollectorSet)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_TriggerDataCollectorSet )( 
            __RPC__in IAlertDataCollector * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *name);
        
        DECLSPEC_XFGVIRT(IAlertDataCollector, put_TriggerDataCollectorSet)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_TriggerDataCollectorSet )( 
            __RPC__in IAlertDataCollector * This,
            /* [in] */ __RPC__in BSTR name);
        
        END_INTERFACE
    } IAlertDataCollectorVtbl;

    interface IAlertDataCollector
    {
        CONST_VTBL struct IAlertDataCollectorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAlertDataCollector_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAlertDataCollector_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAlertDataCollector_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAlertDataCollector_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IAlertDataCollector_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IAlertDataCollector_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IAlertDataCollector_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IAlertDataCollector_get_DataCollectorSet(This,group)	\
    ( (This)->lpVtbl -> get_DataCollectorSet(This,group) ) 

#define IAlertDataCollector_put_DataCollectorSet(This,group)	\
    ( (This)->lpVtbl -> put_DataCollectorSet(This,group) ) 

#define IAlertDataCollector_get_DataCollectorType(This,type)	\
    ( (This)->lpVtbl -> get_DataCollectorType(This,type) ) 

#define IAlertDataCollector_get_FileName(This,name)	\
    ( (This)->lpVtbl -> get_FileName(This,name) ) 

#define IAlertDataCollector_put_FileName(This,name)	\
    ( (This)->lpVtbl -> put_FileName(This,name) ) 

#define IAlertDataCollector_get_FileNameFormat(This,format)	\
    ( (This)->lpVtbl -> get_FileNameFormat(This,format) ) 

#define IAlertDataCollector_put_FileNameFormat(This,format)	\
    ( (This)->lpVtbl -> put_FileNameFormat(This,format) ) 

#define IAlertDataCollector_get_FileNameFormatPattern(This,pattern)	\
    ( (This)->lpVtbl -> get_FileNameFormatPattern(This,pattern) ) 

#define IAlertDataCollector_put_FileNameFormatPattern(This,pattern)	\
    ( (This)->lpVtbl -> put_FileNameFormatPattern(This,pattern) ) 

#define IAlertDataCollector_get_LatestOutputLocation(This,path)	\
    ( (This)->lpVtbl -> get_LatestOutputLocation(This,path) ) 

#define IAlertDataCollector_put_LatestOutputLocation(This,path)	\
    ( (This)->lpVtbl -> put_LatestOutputLocation(This,path) ) 

#define IAlertDataCollector_get_LogAppend(This,append)	\
    ( (This)->lpVtbl -> get_LogAppend(This,append) ) 

#define IAlertDataCollector_put_LogAppend(This,append)	\
    ( (This)->lpVtbl -> put_LogAppend(This,append) ) 

#define IAlertDataCollector_get_LogCircular(This,circular)	\
    ( (This)->lpVtbl -> get_LogCircular(This,circular) ) 

#define IAlertDataCollector_put_LogCircular(This,circular)	\
    ( (This)->lpVtbl -> put_LogCircular(This,circular) ) 

#define IAlertDataCollector_get_LogOverwrite(This,overwrite)	\
    ( (This)->lpVtbl -> get_LogOverwrite(This,overwrite) ) 

#define IAlertDataCollector_put_LogOverwrite(This,overwrite)	\
    ( (This)->lpVtbl -> put_LogOverwrite(This,overwrite) ) 

#define IAlertDataCollector_get_Name(This,name)	\
    ( (This)->lpVtbl -> get_Name(This,name) ) 

#define IAlertDataCollector_put_Name(This,name)	\
    ( (This)->lpVtbl -> put_Name(This,name) ) 

#define IAlertDataCollector_get_OutputLocation(This,path)	\
    ( (This)->lpVtbl -> get_OutputLocation(This,path) ) 

#define IAlertDataCollector_get_Index(This,index)	\
    ( (This)->lpVtbl -> get_Index(This,index) ) 

#define IAlertDataCollector_put_Index(This,index)	\
    ( (This)->lpVtbl -> put_Index(This,index) ) 

#define IAlertDataCollector_get_Xml(This,Xml)	\
    ( (This)->lpVtbl -> get_Xml(This,Xml) ) 

#define IAlertDataCollector_SetXml(This,Xml,Validation)	\
    ( (This)->lpVtbl -> SetXml(This,Xml,Validation) ) 

#define IAlertDataCollector_CreateOutputLocation(This,Latest,Location)	\
    ( (This)->lpVtbl -> CreateOutputLocation(This,Latest,Location) ) 


#define IAlertDataCollector_get_AlertThresholds(This,alerts)	\
    ( (This)->lpVtbl -> get_AlertThresholds(This,alerts) ) 

#define IAlertDataCollector_put_AlertThresholds(This,alerts)	\
    ( (This)->lpVtbl -> put_AlertThresholds(This,alerts) ) 

#define IAlertDataCollector_get_EventLog(This,log)	\
    ( (This)->lpVtbl -> get_EventLog(This,log) ) 

#define IAlertDataCollector_put_EventLog(This,log)	\
    ( (This)->lpVtbl -> put_EventLog(This,log) ) 

#define IAlertDataCollector_get_SampleInterval(This,interval)	\
    ( (This)->lpVtbl -> get_SampleInterval(This,interval) ) 

#define IAlertDataCollector_put_SampleInterval(This,interval)	\
    ( (This)->lpVtbl -> put_SampleInterval(This,interval) ) 

#define IAlertDataCollector_get_Task(This,task)	\
    ( (This)->lpVtbl -> get_Task(This,task) ) 

#define IAlertDataCollector_put_Task(This,task)	\
    ( (This)->lpVtbl -> put_Task(This,task) ) 

#define IAlertDataCollector_get_TaskRunAsSelf(This,RunAsSelf)	\
    ( (This)->lpVtbl -> get_TaskRunAsSelf(This,RunAsSelf) ) 

#define IAlertDataCollector_put_TaskRunAsSelf(This,RunAsSelf)	\
    ( (This)->lpVtbl -> put_TaskRunAsSelf(This,RunAsSelf) ) 

#define IAlertDataCollector_get_TaskArguments(This,task)	\
    ( (This)->lpVtbl -> get_TaskArguments(This,task) ) 

#define IAlertDataCollector_put_TaskArguments(This,task)	\
    ( (This)->lpVtbl -> put_TaskArguments(This,task) ) 

#define IAlertDataCollector_get_TaskUserTextArguments(This,task)	\
    ( (This)->lpVtbl -> get_TaskUserTextArguments(This,task) ) 

#define IAlertDataCollector_put_TaskUserTextArguments(This,task)	\
    ( (This)->lpVtbl -> put_TaskUserTextArguments(This,task) ) 

#define IAlertDataCollector_get_TriggerDataCollectorSet(This,name)	\
    ( (This)->lpVtbl -> get_TriggerDataCollectorSet(This,name) ) 

#define IAlertDataCollector_put_TriggerDataCollectorSet(This,name)	\
    ( (This)->lpVtbl -> put_TriggerDataCollectorSet(This,name) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAlertDataCollector_INTERFACE_DEFINED__ */


#ifndef __IApiTracingDataCollector_INTERFACE_DEFINED__
#define __IApiTracingDataCollector_INTERFACE_DEFINED__

/* interface IApiTracingDataCollector */
/* [dual][uuid][object] */ 


EXTERN_C const IID IID_IApiTracingDataCollector;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0383751a-098b-11d8-9414-505054503030")
    IApiTracingDataCollector : public IDataCollector
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_LogApiNamesOnly( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *logapinames) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_LogApiNamesOnly( 
            /* [in] */ VARIANT_BOOL logapinames) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_LogApisRecursively( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *logrecursively) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_LogApisRecursively( 
            /* [in] */ VARIANT_BOOL logrecursively) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ExePath( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *exepath) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_ExePath( 
            /* [in] */ __RPC__in BSTR exepath) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_LogFilePath( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *logfilepath) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_LogFilePath( 
            /* [in] */ __RPC__in BSTR logfilepath) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_IncludeModules( 
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *includemodules) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_IncludeModules( 
            /* [in] */ __RPC__in SAFEARRAY * includemodules) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_IncludeApis( 
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *includeapis) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_IncludeApis( 
            /* [in] */ __RPC__in SAFEARRAY * includeapis) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ExcludeApis( 
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *excludeapis) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_ExcludeApis( 
            /* [in] */ __RPC__in SAFEARRAY * excludeapis) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IApiTracingDataCollectorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IApiTracingDataCollector * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IApiTracingDataCollector * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IApiTracingDataCollector * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IApiTracingDataCollector * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IApiTracingDataCollector * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IApiTracingDataCollector * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IApiTracingDataCollector * This,
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
        
        DECLSPEC_XFGVIRT(IDataCollector, get_DataCollectorSet)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_DataCollectorSet )( 
            __RPC__in IApiTracingDataCollector * This,
            /* [retval][out] */ __RPC__deref_out_opt IDataCollectorSet **group);
        
        DECLSPEC_XFGVIRT(IDataCollector, put_DataCollectorSet)
        /* [restricted][hidden][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DataCollectorSet )( 
            __RPC__in IApiTracingDataCollector * This,
            /* [in] */ __RPC__in_opt IDataCollectorSet *group);
        
        DECLSPEC_XFGVIRT(IDataCollector, get_DataCollectorType)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_DataCollectorType )( 
            __RPC__in IApiTracingDataCollector * This,
            /* [retval][out] */ __RPC__out DataCollectorType *type);
        
        DECLSPEC_XFGVIRT(IDataCollector, get_FileName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_FileName )( 
            __RPC__in IApiTracingDataCollector * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *name);
        
        DECLSPEC_XFGVIRT(IDataCollector, put_FileName)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_FileName )( 
            __RPC__in IApiTracingDataCollector * This,
            /* [in] */ __RPC__in BSTR name);
        
        DECLSPEC_XFGVIRT(IDataCollector, get_FileNameFormat)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_FileNameFormat )( 
            __RPC__in IApiTracingDataCollector * This,
            /* [retval][out] */ __RPC__out AutoPathFormat *format);
        
        DECLSPEC_XFGVIRT(IDataCollector, put_FileNameFormat)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_FileNameFormat )( 
            __RPC__in IApiTracingDataCollector * This,
            /* [in] */ AutoPathFormat format);
        
        DECLSPEC_XFGVIRT(IDataCollector, get_FileNameFormatPattern)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_FileNameFormatPattern )( 
            __RPC__in IApiTracingDataCollector * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pattern);
        
        DECLSPEC_XFGVIRT(IDataCollector, put_FileNameFormatPattern)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_FileNameFormatPattern )( 
            __RPC__in IApiTracingDataCollector * This,
            /* [in] */ __RPC__in BSTR pattern);
        
        DECLSPEC_XFGVIRT(IDataCollector, get_LatestOutputLocation)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_LatestOutputLocation )( 
            __RPC__in IApiTracingDataCollector * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *path);
        
        DECLSPEC_XFGVIRT(IDataCollector, put_LatestOutputLocation)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_LatestOutputLocation )( 
            __RPC__in IApiTracingDataCollector * This,
            /* [in] */ __RPC__in BSTR path);
        
        DECLSPEC_XFGVIRT(IDataCollector, get_LogAppend)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_LogAppend )( 
            __RPC__in IApiTracingDataCollector * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *append);
        
        DECLSPEC_XFGVIRT(IDataCollector, put_LogAppend)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_LogAppend )( 
            __RPC__in IApiTracingDataCollector * This,
            /* [in] */ VARIANT_BOOL append);
        
        DECLSPEC_XFGVIRT(IDataCollector, get_LogCircular)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_LogCircular )( 
            __RPC__in IApiTracingDataCollector * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *circular);
        
        DECLSPEC_XFGVIRT(IDataCollector, put_LogCircular)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_LogCircular )( 
            __RPC__in IApiTracingDataCollector * This,
            /* [in] */ VARIANT_BOOL circular);
        
        DECLSPEC_XFGVIRT(IDataCollector, get_LogOverwrite)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_LogOverwrite )( 
            __RPC__in IApiTracingDataCollector * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *overwrite);
        
        DECLSPEC_XFGVIRT(IDataCollector, put_LogOverwrite)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_LogOverwrite )( 
            __RPC__in IApiTracingDataCollector * This,
            /* [in] */ VARIANT_BOOL overwrite);
        
        DECLSPEC_XFGVIRT(IDataCollector, get_Name)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IApiTracingDataCollector * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *name);
        
        DECLSPEC_XFGVIRT(IDataCollector, put_Name)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Name )( 
            __RPC__in IApiTracingDataCollector * This,
            /* [in] */ __RPC__in BSTR name);
        
        DECLSPEC_XFGVIRT(IDataCollector, get_OutputLocation)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_OutputLocation )( 
            __RPC__in IApiTracingDataCollector * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *path);
        
        DECLSPEC_XFGVIRT(IDataCollector, get_Index)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Index )( 
            __RPC__in IApiTracingDataCollector * This,
            /* [retval][out] */ __RPC__out long *index);
        
        DECLSPEC_XFGVIRT(IDataCollector, put_Index)
        /* [restricted][hidden][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Index )( 
            __RPC__in IApiTracingDataCollector * This,
            /* [in] */ long index);
        
        DECLSPEC_XFGVIRT(IDataCollector, get_Xml)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Xml )( 
            __RPC__in IApiTracingDataCollector * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Xml);
        
        DECLSPEC_XFGVIRT(IDataCollector, SetXml)
        HRESULT ( STDMETHODCALLTYPE *SetXml )( 
            __RPC__in IApiTracingDataCollector * This,
            /* [in] */ __RPC__in BSTR Xml,
            /* [retval][out] */ __RPC__deref_out_opt IValueMap **Validation);
        
        DECLSPEC_XFGVIRT(IDataCollector, CreateOutputLocation)
        HRESULT ( STDMETHODCALLTYPE *CreateOutputLocation )( 
            __RPC__in IApiTracingDataCollector * This,
            /* [in] */ VARIANT_BOOL Latest,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Location);
        
        DECLSPEC_XFGVIRT(IApiTracingDataCollector, get_LogApiNamesOnly)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_LogApiNamesOnly )( 
            __RPC__in IApiTracingDataCollector * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *logapinames);
        
        DECLSPEC_XFGVIRT(IApiTracingDataCollector, put_LogApiNamesOnly)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_LogApiNamesOnly )( 
            __RPC__in IApiTracingDataCollector * This,
            /* [in] */ VARIANT_BOOL logapinames);
        
        DECLSPEC_XFGVIRT(IApiTracingDataCollector, get_LogApisRecursively)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_LogApisRecursively )( 
            __RPC__in IApiTracingDataCollector * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *logrecursively);
        
        DECLSPEC_XFGVIRT(IApiTracingDataCollector, put_LogApisRecursively)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_LogApisRecursively )( 
            __RPC__in IApiTracingDataCollector * This,
            /* [in] */ VARIANT_BOOL logrecursively);
        
        DECLSPEC_XFGVIRT(IApiTracingDataCollector, get_ExePath)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ExePath )( 
            __RPC__in IApiTracingDataCollector * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *exepath);
        
        DECLSPEC_XFGVIRT(IApiTracingDataCollector, put_ExePath)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ExePath )( 
            __RPC__in IApiTracingDataCollector * This,
            /* [in] */ __RPC__in BSTR exepath);
        
        DECLSPEC_XFGVIRT(IApiTracingDataCollector, get_LogFilePath)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_LogFilePath )( 
            __RPC__in IApiTracingDataCollector * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *logfilepath);
        
        DECLSPEC_XFGVIRT(IApiTracingDataCollector, put_LogFilePath)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_LogFilePath )( 
            __RPC__in IApiTracingDataCollector * This,
            /* [in] */ __RPC__in BSTR logfilepath);
        
        DECLSPEC_XFGVIRT(IApiTracingDataCollector, get_IncludeModules)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_IncludeModules )( 
            __RPC__in IApiTracingDataCollector * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *includemodules);
        
        DECLSPEC_XFGVIRT(IApiTracingDataCollector, put_IncludeModules)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_IncludeModules )( 
            __RPC__in IApiTracingDataCollector * This,
            /* [in] */ __RPC__in SAFEARRAY * includemodules);
        
        DECLSPEC_XFGVIRT(IApiTracingDataCollector, get_IncludeApis)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_IncludeApis )( 
            __RPC__in IApiTracingDataCollector * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *includeapis);
        
        DECLSPEC_XFGVIRT(IApiTracingDataCollector, put_IncludeApis)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_IncludeApis )( 
            __RPC__in IApiTracingDataCollector * This,
            /* [in] */ __RPC__in SAFEARRAY * includeapis);
        
        DECLSPEC_XFGVIRT(IApiTracingDataCollector, get_ExcludeApis)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ExcludeApis )( 
            __RPC__in IApiTracingDataCollector * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *excludeapis);
        
        DECLSPEC_XFGVIRT(IApiTracingDataCollector, put_ExcludeApis)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ExcludeApis )( 
            __RPC__in IApiTracingDataCollector * This,
            /* [in] */ __RPC__in SAFEARRAY * excludeapis);
        
        END_INTERFACE
    } IApiTracingDataCollectorVtbl;

    interface IApiTracingDataCollector
    {
        CONST_VTBL struct IApiTracingDataCollectorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IApiTracingDataCollector_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IApiTracingDataCollector_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IApiTracingDataCollector_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IApiTracingDataCollector_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IApiTracingDataCollector_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IApiTracingDataCollector_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IApiTracingDataCollector_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IApiTracingDataCollector_get_DataCollectorSet(This,group)	\
    ( (This)->lpVtbl -> get_DataCollectorSet(This,group) ) 

#define IApiTracingDataCollector_put_DataCollectorSet(This,group)	\
    ( (This)->lpVtbl -> put_DataCollectorSet(This,group) ) 

#define IApiTracingDataCollector_get_DataCollectorType(This,type)	\
    ( (This)->lpVtbl -> get_DataCollectorType(This,type) ) 

#define IApiTracingDataCollector_get_FileName(This,name)	\
    ( (This)->lpVtbl -> get_FileName(This,name) ) 

#define IApiTracingDataCollector_put_FileName(This,name)	\
    ( (This)->lpVtbl -> put_FileName(This,name) ) 

#define IApiTracingDataCollector_get_FileNameFormat(This,format)	\
    ( (This)->lpVtbl -> get_FileNameFormat(This,format) ) 

#define IApiTracingDataCollector_put_FileNameFormat(This,format)	\
    ( (This)->lpVtbl -> put_FileNameFormat(This,format) ) 

#define IApiTracingDataCollector_get_FileNameFormatPattern(This,pattern)	\
    ( (This)->lpVtbl -> get_FileNameFormatPattern(This,pattern) ) 

#define IApiTracingDataCollector_put_FileNameFormatPattern(This,pattern)	\
    ( (This)->lpVtbl -> put_FileNameFormatPattern(This,pattern) ) 

#define IApiTracingDataCollector_get_LatestOutputLocation(This,path)	\
    ( (This)->lpVtbl -> get_LatestOutputLocation(This,path) ) 

#define IApiTracingDataCollector_put_LatestOutputLocation(This,path)	\
    ( (This)->lpVtbl -> put_LatestOutputLocation(This,path) ) 

#define IApiTracingDataCollector_get_LogAppend(This,append)	\
    ( (This)->lpVtbl -> get_LogAppend(This,append) ) 

#define IApiTracingDataCollector_put_LogAppend(This,append)	\
    ( (This)->lpVtbl -> put_LogAppend(This,append) ) 

#define IApiTracingDataCollector_get_LogCircular(This,circular)	\
    ( (This)->lpVtbl -> get_LogCircular(This,circular) ) 

#define IApiTracingDataCollector_put_LogCircular(This,circular)	\
    ( (This)->lpVtbl -> put_LogCircular(This,circular) ) 

#define IApiTracingDataCollector_get_LogOverwrite(This,overwrite)	\
    ( (This)->lpVtbl -> get_LogOverwrite(This,overwrite) ) 

#define IApiTracingDataCollector_put_LogOverwrite(This,overwrite)	\
    ( (This)->lpVtbl -> put_LogOverwrite(This,overwrite) ) 

#define IApiTracingDataCollector_get_Name(This,name)	\
    ( (This)->lpVtbl -> get_Name(This,name) ) 

#define IApiTracingDataCollector_put_Name(This,name)	\
    ( (This)->lpVtbl -> put_Name(This,name) ) 

#define IApiTracingDataCollector_get_OutputLocation(This,path)	\
    ( (This)->lpVtbl -> get_OutputLocation(This,path) ) 

#define IApiTracingDataCollector_get_Index(This,index)	\
    ( (This)->lpVtbl -> get_Index(This,index) ) 

#define IApiTracingDataCollector_put_Index(This,index)	\
    ( (This)->lpVtbl -> put_Index(This,index) ) 

#define IApiTracingDataCollector_get_Xml(This,Xml)	\
    ( (This)->lpVtbl -> get_Xml(This,Xml) ) 

#define IApiTracingDataCollector_SetXml(This,Xml,Validation)	\
    ( (This)->lpVtbl -> SetXml(This,Xml,Validation) ) 

#define IApiTracingDataCollector_CreateOutputLocation(This,Latest,Location)	\
    ( (This)->lpVtbl -> CreateOutputLocation(This,Latest,Location) ) 


#define IApiTracingDataCollector_get_LogApiNamesOnly(This,logapinames)	\
    ( (This)->lpVtbl -> get_LogApiNamesOnly(This,logapinames) ) 

#define IApiTracingDataCollector_put_LogApiNamesOnly(This,logapinames)	\
    ( (This)->lpVtbl -> put_LogApiNamesOnly(This,logapinames) ) 

#define IApiTracingDataCollector_get_LogApisRecursively(This,logrecursively)	\
    ( (This)->lpVtbl -> get_LogApisRecursively(This,logrecursively) ) 

#define IApiTracingDataCollector_put_LogApisRecursively(This,logrecursively)	\
    ( (This)->lpVtbl -> put_LogApisRecursively(This,logrecursively) ) 

#define IApiTracingDataCollector_get_ExePath(This,exepath)	\
    ( (This)->lpVtbl -> get_ExePath(This,exepath) ) 

#define IApiTracingDataCollector_put_ExePath(This,exepath)	\
    ( (This)->lpVtbl -> put_ExePath(This,exepath) ) 

#define IApiTracingDataCollector_get_LogFilePath(This,logfilepath)	\
    ( (This)->lpVtbl -> get_LogFilePath(This,logfilepath) ) 

#define IApiTracingDataCollector_put_LogFilePath(This,logfilepath)	\
    ( (This)->lpVtbl -> put_LogFilePath(This,logfilepath) ) 

#define IApiTracingDataCollector_get_IncludeModules(This,includemodules)	\
    ( (This)->lpVtbl -> get_IncludeModules(This,includemodules) ) 

#define IApiTracingDataCollector_put_IncludeModules(This,includemodules)	\
    ( (This)->lpVtbl -> put_IncludeModules(This,includemodules) ) 

#define IApiTracingDataCollector_get_IncludeApis(This,includeapis)	\
    ( (This)->lpVtbl -> get_IncludeApis(This,includeapis) ) 

#define IApiTracingDataCollector_put_IncludeApis(This,includeapis)	\
    ( (This)->lpVtbl -> put_IncludeApis(This,includeapis) ) 

#define IApiTracingDataCollector_get_ExcludeApis(This,excludeapis)	\
    ( (This)->lpVtbl -> get_ExcludeApis(This,excludeapis) ) 

#define IApiTracingDataCollector_put_ExcludeApis(This,excludeapis)	\
    ( (This)->lpVtbl -> put_ExcludeApis(This,excludeapis) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IApiTracingDataCollector_INTERFACE_DEFINED__ */


#ifndef __IDataCollectorCollection_INTERFACE_DEFINED__
#define __IDataCollectorCollection_INTERFACE_DEFINED__

/* interface IDataCollectorCollection */
/* [nonextensible][oleautomation][dual][uuid][object] */ 


EXTERN_C const IID IID_IDataCollectorCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("03837502-098b-11d8-9414-505054503030")
    IDataCollectorCollection : public IDispatch
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *retVal) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT index,
            /* [retval][out] */ __RPC__deref_out_opt IDataCollector **collector) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Add( 
            __RPC__in_opt IDataCollector *collector) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Remove( 
            VARIANT collector) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clear( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddRange( 
            __RPC__in_opt IDataCollectorCollection *collectors) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateDataCollectorFromXml( 
            /* [in] */ __RPC__in BSTR bstrXml,
            /* [out] */ __RPC__deref_out_opt IValueMap **pValidation,
            /* [retval][out] */ __RPC__deref_out_opt IDataCollector **pCollector) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateDataCollector( 
            /* [in] */ DataCollectorType Type,
            /* [retval][out] */ __RPC__deref_out_opt IDataCollector **Collector) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDataCollectorCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDataCollectorCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDataCollectorCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDataCollectorCollection * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IDataCollectorCollection * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IDataCollectorCollection * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IDataCollectorCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IDataCollectorCollection * This,
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
        
        DECLSPEC_XFGVIRT(IDataCollectorCollection, get_Count)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IDataCollectorCollection * This,
            /* [retval][out] */ __RPC__out long *retVal);
        
        DECLSPEC_XFGVIRT(IDataCollectorCollection, get_Item)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IDataCollectorCollection * This,
            /* [in] */ VARIANT index,
            /* [retval][out] */ __RPC__deref_out_opt IDataCollector **collector);
        
        DECLSPEC_XFGVIRT(IDataCollectorCollection, get__NewEnum)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IDataCollectorCollection * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retVal);
        
        DECLSPEC_XFGVIRT(IDataCollectorCollection, Add)
        HRESULT ( STDMETHODCALLTYPE *Add )( 
            __RPC__in IDataCollectorCollection * This,
            __RPC__in_opt IDataCollector *collector);
        
        DECLSPEC_XFGVIRT(IDataCollectorCollection, Remove)
        HRESULT ( STDMETHODCALLTYPE *Remove )( 
            __RPC__in IDataCollectorCollection * This,
            VARIANT collector);
        
        DECLSPEC_XFGVIRT(IDataCollectorCollection, Clear)
        HRESULT ( STDMETHODCALLTYPE *Clear )( 
            __RPC__in IDataCollectorCollection * This);
        
        DECLSPEC_XFGVIRT(IDataCollectorCollection, AddRange)
        HRESULT ( STDMETHODCALLTYPE *AddRange )( 
            __RPC__in IDataCollectorCollection * This,
            __RPC__in_opt IDataCollectorCollection *collectors);
        
        DECLSPEC_XFGVIRT(IDataCollectorCollection, CreateDataCollectorFromXml)
        HRESULT ( STDMETHODCALLTYPE *CreateDataCollectorFromXml )( 
            __RPC__in IDataCollectorCollection * This,
            /* [in] */ __RPC__in BSTR bstrXml,
            /* [out] */ __RPC__deref_out_opt IValueMap **pValidation,
            /* [retval][out] */ __RPC__deref_out_opt IDataCollector **pCollector);
        
        DECLSPEC_XFGVIRT(IDataCollectorCollection, CreateDataCollector)
        HRESULT ( STDMETHODCALLTYPE *CreateDataCollector )( 
            __RPC__in IDataCollectorCollection * This,
            /* [in] */ DataCollectorType Type,
            /* [retval][out] */ __RPC__deref_out_opt IDataCollector **Collector);
        
        END_INTERFACE
    } IDataCollectorCollectionVtbl;

    interface IDataCollectorCollection
    {
        CONST_VTBL struct IDataCollectorCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDataCollectorCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDataCollectorCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDataCollectorCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDataCollectorCollection_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IDataCollectorCollection_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IDataCollectorCollection_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IDataCollectorCollection_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IDataCollectorCollection_get_Count(This,retVal)	\
    ( (This)->lpVtbl -> get_Count(This,retVal) ) 

#define IDataCollectorCollection_get_Item(This,index,collector)	\
    ( (This)->lpVtbl -> get_Item(This,index,collector) ) 

#define IDataCollectorCollection_get__NewEnum(This,retVal)	\
    ( (This)->lpVtbl -> get__NewEnum(This,retVal) ) 

#define IDataCollectorCollection_Add(This,collector)	\
    ( (This)->lpVtbl -> Add(This,collector) ) 

#define IDataCollectorCollection_Remove(This,collector)	\
    ( (This)->lpVtbl -> Remove(This,collector) ) 

#define IDataCollectorCollection_Clear(This)	\
    ( (This)->lpVtbl -> Clear(This) ) 

#define IDataCollectorCollection_AddRange(This,collectors)	\
    ( (This)->lpVtbl -> AddRange(This,collectors) ) 

#define IDataCollectorCollection_CreateDataCollectorFromXml(This,bstrXml,pValidation,pCollector)	\
    ( (This)->lpVtbl -> CreateDataCollectorFromXml(This,bstrXml,pValidation,pCollector) ) 

#define IDataCollectorCollection_CreateDataCollector(This,Type,Collector)	\
    ( (This)->lpVtbl -> CreateDataCollector(This,Type,Collector) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDataCollectorCollection_INTERFACE_DEFINED__ */


#ifndef __IDataCollectorSetCollection_INTERFACE_DEFINED__
#define __IDataCollectorSetCollection_INTERFACE_DEFINED__

/* interface IDataCollectorSetCollection */
/* [nonextensible][oleautomation][dual][uuid][object] */ 


EXTERN_C const IID IID_IDataCollectorSetCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("03837524-098b-11d8-9414-505054503030")
    IDataCollectorSetCollection : public IDispatch
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *retVal) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT index,
            /* [retval][out] */ __RPC__deref_out_opt IDataCollectorSet **set) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Add( 
            __RPC__in_opt IDataCollectorSet *set) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Remove( 
            VARIANT set) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clear( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddRange( 
            __RPC__in_opt IDataCollectorSetCollection *sets) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDataCollectorSets( 
            /* [unique][in] */ __RPC__in_opt BSTR server,
            /* [unique][in] */ __RPC__in_opt BSTR filter) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDataCollectorSetCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDataCollectorSetCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDataCollectorSetCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDataCollectorSetCollection * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IDataCollectorSetCollection * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IDataCollectorSetCollection * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IDataCollectorSetCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IDataCollectorSetCollection * This,
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
        
        DECLSPEC_XFGVIRT(IDataCollectorSetCollection, get_Count)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IDataCollectorSetCollection * This,
            /* [retval][out] */ __RPC__out long *retVal);
        
        DECLSPEC_XFGVIRT(IDataCollectorSetCollection, get_Item)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IDataCollectorSetCollection * This,
            /* [in] */ VARIANT index,
            /* [retval][out] */ __RPC__deref_out_opt IDataCollectorSet **set);
        
        DECLSPEC_XFGVIRT(IDataCollectorSetCollection, get__NewEnum)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IDataCollectorSetCollection * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retVal);
        
        DECLSPEC_XFGVIRT(IDataCollectorSetCollection, Add)
        HRESULT ( STDMETHODCALLTYPE *Add )( 
            __RPC__in IDataCollectorSetCollection * This,
            __RPC__in_opt IDataCollectorSet *set);
        
        DECLSPEC_XFGVIRT(IDataCollectorSetCollection, Remove)
        HRESULT ( STDMETHODCALLTYPE *Remove )( 
            __RPC__in IDataCollectorSetCollection * This,
            VARIANT set);
        
        DECLSPEC_XFGVIRT(IDataCollectorSetCollection, Clear)
        HRESULT ( STDMETHODCALLTYPE *Clear )( 
            __RPC__in IDataCollectorSetCollection * This);
        
        DECLSPEC_XFGVIRT(IDataCollectorSetCollection, AddRange)
        HRESULT ( STDMETHODCALLTYPE *AddRange )( 
            __RPC__in IDataCollectorSetCollection * This,
            __RPC__in_opt IDataCollectorSetCollection *sets);
        
        DECLSPEC_XFGVIRT(IDataCollectorSetCollection, GetDataCollectorSets)
        HRESULT ( STDMETHODCALLTYPE *GetDataCollectorSets )( 
            __RPC__in IDataCollectorSetCollection * This,
            /* [unique][in] */ __RPC__in_opt BSTR server,
            /* [unique][in] */ __RPC__in_opt BSTR filter);
        
        END_INTERFACE
    } IDataCollectorSetCollectionVtbl;

    interface IDataCollectorSetCollection
    {
        CONST_VTBL struct IDataCollectorSetCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDataCollectorSetCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDataCollectorSetCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDataCollectorSetCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDataCollectorSetCollection_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IDataCollectorSetCollection_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IDataCollectorSetCollection_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IDataCollectorSetCollection_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IDataCollectorSetCollection_get_Count(This,retVal)	\
    ( (This)->lpVtbl -> get_Count(This,retVal) ) 

#define IDataCollectorSetCollection_get_Item(This,index,set)	\
    ( (This)->lpVtbl -> get_Item(This,index,set) ) 

#define IDataCollectorSetCollection_get__NewEnum(This,retVal)	\
    ( (This)->lpVtbl -> get__NewEnum(This,retVal) ) 

#define IDataCollectorSetCollection_Add(This,set)	\
    ( (This)->lpVtbl -> Add(This,set) ) 

#define IDataCollectorSetCollection_Remove(This,set)	\
    ( (This)->lpVtbl -> Remove(This,set) ) 

#define IDataCollectorSetCollection_Clear(This)	\
    ( (This)->lpVtbl -> Clear(This) ) 

#define IDataCollectorSetCollection_AddRange(This,sets)	\
    ( (This)->lpVtbl -> AddRange(This,sets) ) 

#define IDataCollectorSetCollection_GetDataCollectorSets(This,server,filter)	\
    ( (This)->lpVtbl -> GetDataCollectorSets(This,server,filter) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDataCollectorSetCollection_INTERFACE_DEFINED__ */


#ifndef __ITraceDataProvider_INTERFACE_DEFINED__
#define __ITraceDataProvider_INTERFACE_DEFINED__

/* interface ITraceDataProvider */
/* [dual][uuid][object] */ 


EXTERN_C const IID IID_ITraceDataProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("03837512-098b-11d8-9414-505054503030")
    ITraceDataProvider : public IDispatch
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_DisplayName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *name) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_DisplayName( 
            /* [in] */ __RPC__in BSTR name) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Guid( 
            /* [retval][out] */ __RPC__out GUID *guid) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_Guid( 
            /* [in] */ GUID guid) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Level( 
            /* [retval][out] */ __RPC__deref_out_opt IValueMap **ppLevel) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_KeywordsAny( 
            /* [retval][out] */ __RPC__deref_out_opt IValueMap **ppKeywords) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_KeywordsAll( 
            /* [retval][out] */ __RPC__deref_out_opt IValueMap **ppKeywords) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Properties( 
            /* [retval][out] */ __RPC__deref_out_opt IValueMap **ppProperties) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_FilterEnabled( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *FilterEnabled) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_FilterEnabled( 
            /* [in] */ VARIANT_BOOL FilterEnabled) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_FilterType( 
            /* [retval][out] */ __RPC__out ULONG *pulType) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_FilterType( 
            /* [in] */ ULONG ulType) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_FilterData( 
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *ppData) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_FilterData( 
            /* [in] */ __RPC__in SAFEARRAY * pData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Query( 
            /* [in] */ __RPC__in BSTR bstrName,
            /* [unique][in] */ __RPC__in_opt BSTR bstrServer) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Resolve( 
            /* [in] */ __RPC__in_opt IDispatch *pFrom) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetSecurity( 
            /* [in] */ __RPC__in BSTR Sddl) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSecurity( 
            /* [in] */ ULONG SecurityInfo,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Sddl) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRegisteredProcesses( 
            /* [out] */ __RPC__deref_out_opt IValueMap **Processes) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITraceDataProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITraceDataProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITraceDataProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITraceDataProvider * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITraceDataProvider * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITraceDataProvider * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITraceDataProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITraceDataProvider * This,
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
        
        DECLSPEC_XFGVIRT(ITraceDataProvider, get_DisplayName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_DisplayName )( 
            __RPC__in ITraceDataProvider * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *name);
        
        DECLSPEC_XFGVIRT(ITraceDataProvider, put_DisplayName)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_DisplayName )( 
            __RPC__in ITraceDataProvider * This,
            /* [in] */ __RPC__in BSTR name);
        
        DECLSPEC_XFGVIRT(ITraceDataProvider, get_Guid)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Guid )( 
            __RPC__in ITraceDataProvider * This,
            /* [retval][out] */ __RPC__out GUID *guid);
        
        DECLSPEC_XFGVIRT(ITraceDataProvider, put_Guid)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Guid )( 
            __RPC__in ITraceDataProvider * This,
            /* [in] */ GUID guid);
        
        DECLSPEC_XFGVIRT(ITraceDataProvider, get_Level)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Level )( 
            __RPC__in ITraceDataProvider * This,
            /* [retval][out] */ __RPC__deref_out_opt IValueMap **ppLevel);
        
        DECLSPEC_XFGVIRT(ITraceDataProvider, get_KeywordsAny)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_KeywordsAny )( 
            __RPC__in ITraceDataProvider * This,
            /* [retval][out] */ __RPC__deref_out_opt IValueMap **ppKeywords);
        
        DECLSPEC_XFGVIRT(ITraceDataProvider, get_KeywordsAll)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_KeywordsAll )( 
            __RPC__in ITraceDataProvider * This,
            /* [retval][out] */ __RPC__deref_out_opt IValueMap **ppKeywords);
        
        DECLSPEC_XFGVIRT(ITraceDataProvider, get_Properties)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Properties )( 
            __RPC__in ITraceDataProvider * This,
            /* [retval][out] */ __RPC__deref_out_opt IValueMap **ppProperties);
        
        DECLSPEC_XFGVIRT(ITraceDataProvider, get_FilterEnabled)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_FilterEnabled )( 
            __RPC__in ITraceDataProvider * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *FilterEnabled);
        
        DECLSPEC_XFGVIRT(ITraceDataProvider, put_FilterEnabled)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_FilterEnabled )( 
            __RPC__in ITraceDataProvider * This,
            /* [in] */ VARIANT_BOOL FilterEnabled);
        
        DECLSPEC_XFGVIRT(ITraceDataProvider, get_FilterType)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_FilterType )( 
            __RPC__in ITraceDataProvider * This,
            /* [retval][out] */ __RPC__out ULONG *pulType);
        
        DECLSPEC_XFGVIRT(ITraceDataProvider, put_FilterType)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_FilterType )( 
            __RPC__in ITraceDataProvider * This,
            /* [in] */ ULONG ulType);
        
        DECLSPEC_XFGVIRT(ITraceDataProvider, get_FilterData)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_FilterData )( 
            __RPC__in ITraceDataProvider * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *ppData);
        
        DECLSPEC_XFGVIRT(ITraceDataProvider, put_FilterData)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_FilterData )( 
            __RPC__in ITraceDataProvider * This,
            /* [in] */ __RPC__in SAFEARRAY * pData);
        
        DECLSPEC_XFGVIRT(ITraceDataProvider, Query)
        HRESULT ( STDMETHODCALLTYPE *Query )( 
            __RPC__in ITraceDataProvider * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [unique][in] */ __RPC__in_opt BSTR bstrServer);
        
        DECLSPEC_XFGVIRT(ITraceDataProvider, Resolve)
        HRESULT ( STDMETHODCALLTYPE *Resolve )( 
            __RPC__in ITraceDataProvider * This,
            /* [in] */ __RPC__in_opt IDispatch *pFrom);
        
        DECLSPEC_XFGVIRT(ITraceDataProvider, SetSecurity)
        HRESULT ( STDMETHODCALLTYPE *SetSecurity )( 
            __RPC__in ITraceDataProvider * This,
            /* [in] */ __RPC__in BSTR Sddl);
        
        DECLSPEC_XFGVIRT(ITraceDataProvider, GetSecurity)
        HRESULT ( STDMETHODCALLTYPE *GetSecurity )( 
            __RPC__in ITraceDataProvider * This,
            /* [in] */ ULONG SecurityInfo,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Sddl);
        
        DECLSPEC_XFGVIRT(ITraceDataProvider, GetRegisteredProcesses)
        HRESULT ( STDMETHODCALLTYPE *GetRegisteredProcesses )( 
            __RPC__in ITraceDataProvider * This,
            /* [out] */ __RPC__deref_out_opt IValueMap **Processes);
        
        END_INTERFACE
    } ITraceDataProviderVtbl;

    interface ITraceDataProvider
    {
        CONST_VTBL struct ITraceDataProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITraceDataProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITraceDataProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITraceDataProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITraceDataProvider_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITraceDataProvider_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITraceDataProvider_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITraceDataProvider_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITraceDataProvider_get_DisplayName(This,name)	\
    ( (This)->lpVtbl -> get_DisplayName(This,name) ) 

#define ITraceDataProvider_put_DisplayName(This,name)	\
    ( (This)->lpVtbl -> put_DisplayName(This,name) ) 

#define ITraceDataProvider_get_Guid(This,guid)	\
    ( (This)->lpVtbl -> get_Guid(This,guid) ) 

#define ITraceDataProvider_put_Guid(This,guid)	\
    ( (This)->lpVtbl -> put_Guid(This,guid) ) 

#define ITraceDataProvider_get_Level(This,ppLevel)	\
    ( (This)->lpVtbl -> get_Level(This,ppLevel) ) 

#define ITraceDataProvider_get_KeywordsAny(This,ppKeywords)	\
    ( (This)->lpVtbl -> get_KeywordsAny(This,ppKeywords) ) 

#define ITraceDataProvider_get_KeywordsAll(This,ppKeywords)	\
    ( (This)->lpVtbl -> get_KeywordsAll(This,ppKeywords) ) 

#define ITraceDataProvider_get_Properties(This,ppProperties)	\
    ( (This)->lpVtbl -> get_Properties(This,ppProperties) ) 

#define ITraceDataProvider_get_FilterEnabled(This,FilterEnabled)	\
    ( (This)->lpVtbl -> get_FilterEnabled(This,FilterEnabled) ) 

#define ITraceDataProvider_put_FilterEnabled(This,FilterEnabled)	\
    ( (This)->lpVtbl -> put_FilterEnabled(This,FilterEnabled) ) 

#define ITraceDataProvider_get_FilterType(This,pulType)	\
    ( (This)->lpVtbl -> get_FilterType(This,pulType) ) 

#define ITraceDataProvider_put_FilterType(This,ulType)	\
    ( (This)->lpVtbl -> put_FilterType(This,ulType) ) 

#define ITraceDataProvider_get_FilterData(This,ppData)	\
    ( (This)->lpVtbl -> get_FilterData(This,ppData) ) 

#define ITraceDataProvider_put_FilterData(This,pData)	\
    ( (This)->lpVtbl -> put_FilterData(This,pData) ) 

#define ITraceDataProvider_Query(This,bstrName,bstrServer)	\
    ( (This)->lpVtbl -> Query(This,bstrName,bstrServer) ) 

#define ITraceDataProvider_Resolve(This,pFrom)	\
    ( (This)->lpVtbl -> Resolve(This,pFrom) ) 

#define ITraceDataProvider_SetSecurity(This,Sddl)	\
    ( (This)->lpVtbl -> SetSecurity(This,Sddl) ) 

#define ITraceDataProvider_GetSecurity(This,SecurityInfo,Sddl)	\
    ( (This)->lpVtbl -> GetSecurity(This,SecurityInfo,Sddl) ) 

#define ITraceDataProvider_GetRegisteredProcesses(This,Processes)	\
    ( (This)->lpVtbl -> GetRegisteredProcesses(This,Processes) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITraceDataProvider_INTERFACE_DEFINED__ */


#ifndef __ITraceDataProviderCollection_INTERFACE_DEFINED__
#define __ITraceDataProviderCollection_INTERFACE_DEFINED__

/* interface ITraceDataProviderCollection */
/* [nonextensible][oleautomation][dual][uuid][object] */ 


EXTERN_C const IID IID_ITraceDataProviderCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("03837510-098b-11d8-9414-505054503030")
    ITraceDataProviderCollection : public IDispatch
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *retVal) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT index,
            /* [retval][out] */ __RPC__deref_out_opt ITraceDataProvider **ppProvider) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Add( 
            __RPC__in_opt ITraceDataProvider *pProvider) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Remove( 
            VARIANT vProvider) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clear( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddRange( 
            __RPC__in_opt ITraceDataProviderCollection *providers) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateTraceDataProvider( 
            /* [retval][out] */ __RPC__deref_out_opt ITraceDataProvider **Provider) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTraceDataProviders( 
            /* [unique][in] */ __RPC__in_opt BSTR server) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTraceDataProvidersByProcess( 
            /* [unique][in] */ __RPC__in_opt BSTR Server,
            /* [in] */ ULONG Pid) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITraceDataProviderCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITraceDataProviderCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITraceDataProviderCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITraceDataProviderCollection * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITraceDataProviderCollection * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITraceDataProviderCollection * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITraceDataProviderCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITraceDataProviderCollection * This,
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
        
        DECLSPEC_XFGVIRT(ITraceDataProviderCollection, get_Count)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in ITraceDataProviderCollection * This,
            /* [retval][out] */ __RPC__out long *retVal);
        
        DECLSPEC_XFGVIRT(ITraceDataProviderCollection, get_Item)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in ITraceDataProviderCollection * This,
            /* [in] */ VARIANT index,
            /* [retval][out] */ __RPC__deref_out_opt ITraceDataProvider **ppProvider);
        
        DECLSPEC_XFGVIRT(ITraceDataProviderCollection, get__NewEnum)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in ITraceDataProviderCollection * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retVal);
        
        DECLSPEC_XFGVIRT(ITraceDataProviderCollection, Add)
        HRESULT ( STDMETHODCALLTYPE *Add )( 
            __RPC__in ITraceDataProviderCollection * This,
            __RPC__in_opt ITraceDataProvider *pProvider);
        
        DECLSPEC_XFGVIRT(ITraceDataProviderCollection, Remove)
        HRESULT ( STDMETHODCALLTYPE *Remove )( 
            __RPC__in ITraceDataProviderCollection * This,
            VARIANT vProvider);
        
        DECLSPEC_XFGVIRT(ITraceDataProviderCollection, Clear)
        HRESULT ( STDMETHODCALLTYPE *Clear )( 
            __RPC__in ITraceDataProviderCollection * This);
        
        DECLSPEC_XFGVIRT(ITraceDataProviderCollection, AddRange)
        HRESULT ( STDMETHODCALLTYPE *AddRange )( 
            __RPC__in ITraceDataProviderCollection * This,
            __RPC__in_opt ITraceDataProviderCollection *providers);
        
        DECLSPEC_XFGVIRT(ITraceDataProviderCollection, CreateTraceDataProvider)
        HRESULT ( STDMETHODCALLTYPE *CreateTraceDataProvider )( 
            __RPC__in ITraceDataProviderCollection * This,
            /* [retval][out] */ __RPC__deref_out_opt ITraceDataProvider **Provider);
        
        DECLSPEC_XFGVIRT(ITraceDataProviderCollection, GetTraceDataProviders)
        HRESULT ( STDMETHODCALLTYPE *GetTraceDataProviders )( 
            __RPC__in ITraceDataProviderCollection * This,
            /* [unique][in] */ __RPC__in_opt BSTR server);
        
        DECLSPEC_XFGVIRT(ITraceDataProviderCollection, GetTraceDataProvidersByProcess)
        HRESULT ( STDMETHODCALLTYPE *GetTraceDataProvidersByProcess )( 
            __RPC__in ITraceDataProviderCollection * This,
            /* [unique][in] */ __RPC__in_opt BSTR Server,
            /* [in] */ ULONG Pid);
        
        END_INTERFACE
    } ITraceDataProviderCollectionVtbl;

    interface ITraceDataProviderCollection
    {
        CONST_VTBL struct ITraceDataProviderCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITraceDataProviderCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITraceDataProviderCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITraceDataProviderCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITraceDataProviderCollection_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITraceDataProviderCollection_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITraceDataProviderCollection_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITraceDataProviderCollection_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITraceDataProviderCollection_get_Count(This,retVal)	\
    ( (This)->lpVtbl -> get_Count(This,retVal) ) 

#define ITraceDataProviderCollection_get_Item(This,index,ppProvider)	\
    ( (This)->lpVtbl -> get_Item(This,index,ppProvider) ) 

#define ITraceDataProviderCollection_get__NewEnum(This,retVal)	\
    ( (This)->lpVtbl -> get__NewEnum(This,retVal) ) 

#define ITraceDataProviderCollection_Add(This,pProvider)	\
    ( (This)->lpVtbl -> Add(This,pProvider) ) 

#define ITraceDataProviderCollection_Remove(This,vProvider)	\
    ( (This)->lpVtbl -> Remove(This,vProvider) ) 

#define ITraceDataProviderCollection_Clear(This)	\
    ( (This)->lpVtbl -> Clear(This) ) 

#define ITraceDataProviderCollection_AddRange(This,providers)	\
    ( (This)->lpVtbl -> AddRange(This,providers) ) 

#define ITraceDataProviderCollection_CreateTraceDataProvider(This,Provider)	\
    ( (This)->lpVtbl -> CreateTraceDataProvider(This,Provider) ) 

#define ITraceDataProviderCollection_GetTraceDataProviders(This,server)	\
    ( (This)->lpVtbl -> GetTraceDataProviders(This,server) ) 

#define ITraceDataProviderCollection_GetTraceDataProvidersByProcess(This,Server,Pid)	\
    ( (This)->lpVtbl -> GetTraceDataProvidersByProcess(This,Server,Pid) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITraceDataProviderCollection_INTERFACE_DEFINED__ */


#ifndef __ISchedule_INTERFACE_DEFINED__
#define __ISchedule_INTERFACE_DEFINED__

/* interface ISchedule */
/* [dual][uuid][object] */ 


EXTERN_C const IID IID_ISchedule;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0383753a-098b-11d8-9414-505054503030")
    ISchedule : public IDispatch
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_StartDate( 
            /* [retval][out] */ __RPC__out VARIANT *start) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_StartDate( 
            /* [in] */ VARIANT start) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_EndDate( 
            /* [retval][out] */ __RPC__out VARIANT *end) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_EndDate( 
            /* [in] */ VARIANT end) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_StartTime( 
            /* [retval][out] */ __RPC__out VARIANT *start) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_StartTime( 
            /* [in] */ VARIANT start) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Days( 
            /* [retval][out] */ __RPC__out WeekDays *days) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_Days( 
            /* [in] */ WeekDays days) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IScheduleVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISchedule * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISchedule * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISchedule * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ISchedule * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ISchedule * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ISchedule * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISchedule * This,
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
        
        DECLSPEC_XFGVIRT(ISchedule, get_StartDate)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_StartDate )( 
            __RPC__in ISchedule * This,
            /* [retval][out] */ __RPC__out VARIANT *start);
        
        DECLSPEC_XFGVIRT(ISchedule, put_StartDate)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_StartDate )( 
            __RPC__in ISchedule * This,
            /* [in] */ VARIANT start);
        
        DECLSPEC_XFGVIRT(ISchedule, get_EndDate)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_EndDate )( 
            __RPC__in ISchedule * This,
            /* [retval][out] */ __RPC__out VARIANT *end);
        
        DECLSPEC_XFGVIRT(ISchedule, put_EndDate)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_EndDate )( 
            __RPC__in ISchedule * This,
            /* [in] */ VARIANT end);
        
        DECLSPEC_XFGVIRT(ISchedule, get_StartTime)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_StartTime )( 
            __RPC__in ISchedule * This,
            /* [retval][out] */ __RPC__out VARIANT *start);
        
        DECLSPEC_XFGVIRT(ISchedule, put_StartTime)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_StartTime )( 
            __RPC__in ISchedule * This,
            /* [in] */ VARIANT start);
        
        DECLSPEC_XFGVIRT(ISchedule, get_Days)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Days )( 
            __RPC__in ISchedule * This,
            /* [retval][out] */ __RPC__out WeekDays *days);
        
        DECLSPEC_XFGVIRT(ISchedule, put_Days)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Days )( 
            __RPC__in ISchedule * This,
            /* [in] */ WeekDays days);
        
        END_INTERFACE
    } IScheduleVtbl;

    interface ISchedule
    {
        CONST_VTBL struct IScheduleVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISchedule_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISchedule_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISchedule_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISchedule_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISchedule_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISchedule_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISchedule_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISchedule_get_StartDate(This,start)	\
    ( (This)->lpVtbl -> get_StartDate(This,start) ) 

#define ISchedule_put_StartDate(This,start)	\
    ( (This)->lpVtbl -> put_StartDate(This,start) ) 

#define ISchedule_get_EndDate(This,end)	\
    ( (This)->lpVtbl -> get_EndDate(This,end) ) 

#define ISchedule_put_EndDate(This,end)	\
    ( (This)->lpVtbl -> put_EndDate(This,end) ) 

#define ISchedule_get_StartTime(This,start)	\
    ( (This)->lpVtbl -> get_StartTime(This,start) ) 

#define ISchedule_put_StartTime(This,start)	\
    ( (This)->lpVtbl -> put_StartTime(This,start) ) 

#define ISchedule_get_Days(This,days)	\
    ( (This)->lpVtbl -> get_Days(This,days) ) 

#define ISchedule_put_Days(This,days)	\
    ( (This)->lpVtbl -> put_Days(This,days) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISchedule_INTERFACE_DEFINED__ */


#ifndef __IScheduleCollection_INTERFACE_DEFINED__
#define __IScheduleCollection_INTERFACE_DEFINED__

/* interface IScheduleCollection */
/* [nonextensible][oleautomation][dual][uuid][object] */ 


EXTERN_C const IID IID_IScheduleCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0383753d-098b-11d8-9414-505054503030")
    IScheduleCollection : public IDispatch
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *retVal) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT index,
            /* [retval][out] */ __RPC__deref_out_opt ISchedule **ppSchedule) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ienum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Add( 
            __RPC__in_opt ISchedule *pSchedule) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Remove( 
            VARIANT vSchedule) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clear( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddRange( 
            __RPC__in_opt IScheduleCollection *pSchedules) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateSchedule( 
            /* [retval][out] */ __RPC__deref_out_opt ISchedule **Schedule) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IScheduleCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IScheduleCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IScheduleCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IScheduleCollection * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IScheduleCollection * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IScheduleCollection * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IScheduleCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IScheduleCollection * This,
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
        
        DECLSPEC_XFGVIRT(IScheduleCollection, get_Count)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IScheduleCollection * This,
            /* [retval][out] */ __RPC__out long *retVal);
        
        DECLSPEC_XFGVIRT(IScheduleCollection, get_Item)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IScheduleCollection * This,
            /* [in] */ VARIANT index,
            /* [retval][out] */ __RPC__deref_out_opt ISchedule **ppSchedule);
        
        DECLSPEC_XFGVIRT(IScheduleCollection, get__NewEnum)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IScheduleCollection * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ienum);
        
        DECLSPEC_XFGVIRT(IScheduleCollection, Add)
        HRESULT ( STDMETHODCALLTYPE *Add )( 
            __RPC__in IScheduleCollection * This,
            __RPC__in_opt ISchedule *pSchedule);
        
        DECLSPEC_XFGVIRT(IScheduleCollection, Remove)
        HRESULT ( STDMETHODCALLTYPE *Remove )( 
            __RPC__in IScheduleCollection * This,
            VARIANT vSchedule);
        
        DECLSPEC_XFGVIRT(IScheduleCollection, Clear)
        HRESULT ( STDMETHODCALLTYPE *Clear )( 
            __RPC__in IScheduleCollection * This);
        
        DECLSPEC_XFGVIRT(IScheduleCollection, AddRange)
        HRESULT ( STDMETHODCALLTYPE *AddRange )( 
            __RPC__in IScheduleCollection * This,
            __RPC__in_opt IScheduleCollection *pSchedules);
        
        DECLSPEC_XFGVIRT(IScheduleCollection, CreateSchedule)
        HRESULT ( STDMETHODCALLTYPE *CreateSchedule )( 
            __RPC__in IScheduleCollection * This,
            /* [retval][out] */ __RPC__deref_out_opt ISchedule **Schedule);
        
        END_INTERFACE
    } IScheduleCollectionVtbl;

    interface IScheduleCollection
    {
        CONST_VTBL struct IScheduleCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IScheduleCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IScheduleCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IScheduleCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IScheduleCollection_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IScheduleCollection_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IScheduleCollection_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IScheduleCollection_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IScheduleCollection_get_Count(This,retVal)	\
    ( (This)->lpVtbl -> get_Count(This,retVal) ) 

#define IScheduleCollection_get_Item(This,index,ppSchedule)	\
    ( (This)->lpVtbl -> get_Item(This,index,ppSchedule) ) 

#define IScheduleCollection_get__NewEnum(This,ienum)	\
    ( (This)->lpVtbl -> get__NewEnum(This,ienum) ) 

#define IScheduleCollection_Add(This,pSchedule)	\
    ( (This)->lpVtbl -> Add(This,pSchedule) ) 

#define IScheduleCollection_Remove(This,vSchedule)	\
    ( (This)->lpVtbl -> Remove(This,vSchedule) ) 

#define IScheduleCollection_Clear(This)	\
    ( (This)->lpVtbl -> Clear(This) ) 

#define IScheduleCollection_AddRange(This,pSchedules)	\
    ( (This)->lpVtbl -> AddRange(This,pSchedules) ) 

#define IScheduleCollection_CreateSchedule(This,Schedule)	\
    ( (This)->lpVtbl -> CreateSchedule(This,Schedule) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IScheduleCollection_INTERFACE_DEFINED__ */


#ifndef __IValueMapItem_INTERFACE_DEFINED__
#define __IValueMapItem_INTERFACE_DEFINED__

/* interface IValueMapItem */
/* [nonextensible][oleautomation][dual][uuid][object] */ 


EXTERN_C const IID IID_IValueMapItem;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("03837533-098b-11d8-9414-505054503030")
    IValueMapItem : public IDispatch
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Description( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *description) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_Description( 
            /* [in] */ __RPC__in BSTR description) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Enabled( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *enabled) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_Enabled( 
            /* [in] */ VARIANT_BOOL enabled) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Key( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *key) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_Key( 
            /* [in] */ __RPC__in BSTR key) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Value( 
            /* [retval][out] */ __RPC__out VARIANT *Value) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_Value( 
            /* [in] */ VARIANT Value) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ValueMapType( 
            /* [retval][out] */ __RPC__out ValueMapType *type) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_ValueMapType( 
            /* [in] */ ValueMapType type) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IValueMapItemVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IValueMapItem * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IValueMapItem * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IValueMapItem * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IValueMapItem * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IValueMapItem * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IValueMapItem * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IValueMapItem * This,
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
        
        DECLSPEC_XFGVIRT(IValueMapItem, get_Description)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in IValueMapItem * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *description);
        
        DECLSPEC_XFGVIRT(IValueMapItem, put_Description)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Description )( 
            __RPC__in IValueMapItem * This,
            /* [in] */ __RPC__in BSTR description);
        
        DECLSPEC_XFGVIRT(IValueMapItem, get_Enabled)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Enabled )( 
            __RPC__in IValueMapItem * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *enabled);
        
        DECLSPEC_XFGVIRT(IValueMapItem, put_Enabled)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Enabled )( 
            __RPC__in IValueMapItem * This,
            /* [in] */ VARIANT_BOOL enabled);
        
        DECLSPEC_XFGVIRT(IValueMapItem, get_Key)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Key )( 
            __RPC__in IValueMapItem * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *key);
        
        DECLSPEC_XFGVIRT(IValueMapItem, put_Key)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Key )( 
            __RPC__in IValueMapItem * This,
            /* [in] */ __RPC__in BSTR key);
        
        DECLSPEC_XFGVIRT(IValueMapItem, get_Value)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Value )( 
            __RPC__in IValueMapItem * This,
            /* [retval][out] */ __RPC__out VARIANT *Value);
        
        DECLSPEC_XFGVIRT(IValueMapItem, put_Value)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Value )( 
            __RPC__in IValueMapItem * This,
            /* [in] */ VARIANT Value);
        
        DECLSPEC_XFGVIRT(IValueMapItem, get_ValueMapType)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ValueMapType )( 
            __RPC__in IValueMapItem * This,
            /* [retval][out] */ __RPC__out ValueMapType *type);
        
        DECLSPEC_XFGVIRT(IValueMapItem, put_ValueMapType)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ValueMapType )( 
            __RPC__in IValueMapItem * This,
            /* [in] */ ValueMapType type);
        
        END_INTERFACE
    } IValueMapItemVtbl;

    interface IValueMapItem
    {
        CONST_VTBL struct IValueMapItemVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IValueMapItem_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IValueMapItem_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IValueMapItem_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IValueMapItem_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IValueMapItem_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IValueMapItem_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IValueMapItem_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IValueMapItem_get_Description(This,description)	\
    ( (This)->lpVtbl -> get_Description(This,description) ) 

#define IValueMapItem_put_Description(This,description)	\
    ( (This)->lpVtbl -> put_Description(This,description) ) 

#define IValueMapItem_get_Enabled(This,enabled)	\
    ( (This)->lpVtbl -> get_Enabled(This,enabled) ) 

#define IValueMapItem_put_Enabled(This,enabled)	\
    ( (This)->lpVtbl -> put_Enabled(This,enabled) ) 

#define IValueMapItem_get_Key(This,key)	\
    ( (This)->lpVtbl -> get_Key(This,key) ) 

#define IValueMapItem_put_Key(This,key)	\
    ( (This)->lpVtbl -> put_Key(This,key) ) 

#define IValueMapItem_get_Value(This,Value)	\
    ( (This)->lpVtbl -> get_Value(This,Value) ) 

#define IValueMapItem_put_Value(This,Value)	\
    ( (This)->lpVtbl -> put_Value(This,Value) ) 

#define IValueMapItem_get_ValueMapType(This,type)	\
    ( (This)->lpVtbl -> get_ValueMapType(This,type) ) 

#define IValueMapItem_put_ValueMapType(This,type)	\
    ( (This)->lpVtbl -> put_ValueMapType(This,type) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IValueMapItem_INTERFACE_DEFINED__ */


#ifndef __IValueMap_INTERFACE_DEFINED__
#define __IValueMap_INTERFACE_DEFINED__

/* interface IValueMap */
/* [nonextensible][oleautomation][dual][uuid][object] */ 


EXTERN_C const IID IID_IValueMap;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("03837534-098b-11d8-9414-505054503030")
    IValueMap : public IDispatch
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *retVal) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT index,
            /* [retval][out] */ __RPC__deref_out_opt IValueMapItem **value) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retVal) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Description( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *description) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_Description( 
            /* [in] */ __RPC__in BSTR description) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Value( 
            /* [retval][out] */ __RPC__out VARIANT *Value) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_Value( 
            /* [in] */ VARIANT Value) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ValueMapType( 
            /* [retval][out] */ __RPC__out ValueMapType *type) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_ValueMapType( 
            /* [in] */ ValueMapType type) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Add( 
            VARIANT value) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Remove( 
            VARIANT value) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clear( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddRange( 
            __RPC__in_opt IValueMap *map) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateValueMapItem( 
            /* [retval][out] */ __RPC__deref_out_opt IValueMapItem **Item) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IValueMapVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IValueMap * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IValueMap * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IValueMap * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IValueMap * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IValueMap * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IValueMap * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IValueMap * This,
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
        
        DECLSPEC_XFGVIRT(IValueMap, get_Count)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IValueMap * This,
            /* [retval][out] */ __RPC__out long *retVal);
        
        DECLSPEC_XFGVIRT(IValueMap, get_Item)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IValueMap * This,
            /* [in] */ VARIANT index,
            /* [retval][out] */ __RPC__deref_out_opt IValueMapItem **value);
        
        DECLSPEC_XFGVIRT(IValueMap, get__NewEnum)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IValueMap * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retVal);
        
        DECLSPEC_XFGVIRT(IValueMap, get_Description)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in IValueMap * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *description);
        
        DECLSPEC_XFGVIRT(IValueMap, put_Description)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Description )( 
            __RPC__in IValueMap * This,
            /* [in] */ __RPC__in BSTR description);
        
        DECLSPEC_XFGVIRT(IValueMap, get_Value)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Value )( 
            __RPC__in IValueMap * This,
            /* [retval][out] */ __RPC__out VARIANT *Value);
        
        DECLSPEC_XFGVIRT(IValueMap, put_Value)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Value )( 
            __RPC__in IValueMap * This,
            /* [in] */ VARIANT Value);
        
        DECLSPEC_XFGVIRT(IValueMap, get_ValueMapType)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ValueMapType )( 
            __RPC__in IValueMap * This,
            /* [retval][out] */ __RPC__out ValueMapType *type);
        
        DECLSPEC_XFGVIRT(IValueMap, put_ValueMapType)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ValueMapType )( 
            __RPC__in IValueMap * This,
            /* [in] */ ValueMapType type);
        
        DECLSPEC_XFGVIRT(IValueMap, Add)
        HRESULT ( STDMETHODCALLTYPE *Add )( 
            __RPC__in IValueMap * This,
            VARIANT value);
        
        DECLSPEC_XFGVIRT(IValueMap, Remove)
        HRESULT ( STDMETHODCALLTYPE *Remove )( 
            __RPC__in IValueMap * This,
            VARIANT value);
        
        DECLSPEC_XFGVIRT(IValueMap, Clear)
        HRESULT ( STDMETHODCALLTYPE *Clear )( 
            __RPC__in IValueMap * This);
        
        DECLSPEC_XFGVIRT(IValueMap, AddRange)
        HRESULT ( STDMETHODCALLTYPE *AddRange )( 
            __RPC__in IValueMap * This,
            __RPC__in_opt IValueMap *map);
        
        DECLSPEC_XFGVIRT(IValueMap, CreateValueMapItem)
        HRESULT ( STDMETHODCALLTYPE *CreateValueMapItem )( 
            __RPC__in IValueMap * This,
            /* [retval][out] */ __RPC__deref_out_opt IValueMapItem **Item);
        
        END_INTERFACE
    } IValueMapVtbl;

    interface IValueMap
    {
        CONST_VTBL struct IValueMapVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IValueMap_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IValueMap_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IValueMap_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IValueMap_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IValueMap_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IValueMap_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IValueMap_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IValueMap_get_Count(This,retVal)	\
    ( (This)->lpVtbl -> get_Count(This,retVal) ) 

#define IValueMap_get_Item(This,index,value)	\
    ( (This)->lpVtbl -> get_Item(This,index,value) ) 

#define IValueMap_get__NewEnum(This,retVal)	\
    ( (This)->lpVtbl -> get__NewEnum(This,retVal) ) 

#define IValueMap_get_Description(This,description)	\
    ( (This)->lpVtbl -> get_Description(This,description) ) 

#define IValueMap_put_Description(This,description)	\
    ( (This)->lpVtbl -> put_Description(This,description) ) 

#define IValueMap_get_Value(This,Value)	\
    ( (This)->lpVtbl -> get_Value(This,Value) ) 

#define IValueMap_put_Value(This,Value)	\
    ( (This)->lpVtbl -> put_Value(This,Value) ) 

#define IValueMap_get_ValueMapType(This,type)	\
    ( (This)->lpVtbl -> get_ValueMapType(This,type) ) 

#define IValueMap_put_ValueMapType(This,type)	\
    ( (This)->lpVtbl -> put_ValueMapType(This,type) ) 

#define IValueMap_Add(This,value)	\
    ( (This)->lpVtbl -> Add(This,value) ) 

#define IValueMap_Remove(This,value)	\
    ( (This)->lpVtbl -> Remove(This,value) ) 

#define IValueMap_Clear(This)	\
    ( (This)->lpVtbl -> Clear(This) ) 

#define IValueMap_AddRange(This,map)	\
    ( (This)->lpVtbl -> AddRange(This,map) ) 

#define IValueMap_CreateValueMapItem(This,Item)	\
    ( (This)->lpVtbl -> CreateValueMapItem(This,Item) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IValueMap_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_DataCollectorSet;

#ifdef __cplusplus

class DECLSPEC_UUID("03837521-098b-11d8-9414-505054503030")
DataCollectorSet;
#endif

EXTERN_C const CLSID CLSID_TraceSession;

#ifdef __cplusplus

class DECLSPEC_UUID("0383751c-098b-11d8-9414-505054503030")
TraceSession;
#endif

EXTERN_C const CLSID CLSID_TraceSessionCollection;

#ifdef __cplusplus

class DECLSPEC_UUID("03837530-098b-11d8-9414-505054503030")
TraceSessionCollection;
#endif

EXTERN_C const CLSID CLSID_TraceDataProvider;

#ifdef __cplusplus

class DECLSPEC_UUID("03837513-098b-11d8-9414-505054503030")
TraceDataProvider;
#endif

EXTERN_C const CLSID CLSID_TraceDataProviderCollection;

#ifdef __cplusplus

class DECLSPEC_UUID("03837511-098b-11d8-9414-505054503030")
TraceDataProviderCollection;
#endif

EXTERN_C const CLSID CLSID_DataCollectorSetCollection;

#ifdef __cplusplus

class DECLSPEC_UUID("03837525-098b-11d8-9414-505054503030")
DataCollectorSetCollection;
#endif

EXTERN_C const CLSID CLSID_LegacyDataCollectorSet;

#ifdef __cplusplus

class DECLSPEC_UUID("03837526-098b-11d8-9414-505054503030")
LegacyDataCollectorSet;
#endif

EXTERN_C const CLSID CLSID_LegacyDataCollectorSetCollection;

#ifdef __cplusplus

class DECLSPEC_UUID("03837527-098b-11d8-9414-505054503030")
LegacyDataCollectorSetCollection;
#endif

EXTERN_C const CLSID CLSID_LegacyTraceSession;

#ifdef __cplusplus

class DECLSPEC_UUID("03837528-098b-11d8-9414-505054503030")
LegacyTraceSession;
#endif

EXTERN_C const CLSID CLSID_LegacyTraceSessionCollection;

#ifdef __cplusplus

class DECLSPEC_UUID("03837529-098b-11d8-9414-505054503030")
LegacyTraceSessionCollection;
#endif

EXTERN_C const CLSID CLSID_ServerDataCollectorSet;

#ifdef __cplusplus

class DECLSPEC_UUID("03837531-098b-11d8-9414-505054503030")
ServerDataCollectorSet;
#endif

EXTERN_C const CLSID CLSID_ServerDataCollectorSetCollection;

#ifdef __cplusplus

class DECLSPEC_UUID("03837532-098b-11d8-9414-505054503030")
ServerDataCollectorSetCollection;
#endif

EXTERN_C const CLSID CLSID_SystemDataCollectorSet;

#ifdef __cplusplus

class DECLSPEC_UUID("03837546-098b-11d8-9414-505054503030")
SystemDataCollectorSet;
#endif

EXTERN_C const CLSID CLSID_SystemDataCollectorSetCollection;

#ifdef __cplusplus

class DECLSPEC_UUID("03837547-098b-11d8-9414-505054503030")
SystemDataCollectorSetCollection;
#endif

EXTERN_C const CLSID CLSID_BootTraceSession;

#ifdef __cplusplus

class DECLSPEC_UUID("03837538-098b-11d8-9414-505054503030")
BootTraceSession;
#endif

EXTERN_C const CLSID CLSID_BootTraceSessionCollection;

#ifdef __cplusplus

class DECLSPEC_UUID("03837539-098b-11d8-9414-505054503030")
BootTraceSessionCollection;
#endif
#endif /* __PlaLibrary_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_pla_0000_0001 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_pla_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_pla_0000_0001_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


