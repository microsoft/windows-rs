

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

#ifndef __sensorsapi_h__
#define __sensorsapi_h__

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

#ifndef __ISensorManager_FWD_DEFINED__
#define __ISensorManager_FWD_DEFINED__
typedef interface ISensorManager ISensorManager;

#endif 	/* __ISensorManager_FWD_DEFINED__ */


#ifndef __ILocationPermissions_FWD_DEFINED__
#define __ILocationPermissions_FWD_DEFINED__
typedef interface ILocationPermissions ILocationPermissions;

#endif 	/* __ILocationPermissions_FWD_DEFINED__ */


#ifndef __ISensorCollection_FWD_DEFINED__
#define __ISensorCollection_FWD_DEFINED__
typedef interface ISensorCollection ISensorCollection;

#endif 	/* __ISensorCollection_FWD_DEFINED__ */


#ifndef __ISensor_FWD_DEFINED__
#define __ISensor_FWD_DEFINED__
typedef interface ISensor ISensor;

#endif 	/* __ISensor_FWD_DEFINED__ */


#ifndef __ISensorDataReport_FWD_DEFINED__
#define __ISensorDataReport_FWD_DEFINED__
typedef interface ISensorDataReport ISensorDataReport;

#endif 	/* __ISensorDataReport_FWD_DEFINED__ */


#ifndef __ISensorManagerEvents_FWD_DEFINED__
#define __ISensorManagerEvents_FWD_DEFINED__
typedef interface ISensorManagerEvents ISensorManagerEvents;

#endif 	/* __ISensorManagerEvents_FWD_DEFINED__ */


#ifndef __ISensorEvents_FWD_DEFINED__
#define __ISensorEvents_FWD_DEFINED__
typedef interface ISensorEvents ISensorEvents;

#endif 	/* __ISensorEvents_FWD_DEFINED__ */


#ifndef __SensorManager_FWD_DEFINED__
#define __SensorManager_FWD_DEFINED__

#ifdef __cplusplus
typedef class SensorManager SensorManager;
#else
typedef struct SensorManager SensorManager;
#endif /* __cplusplus */

#endif 	/* __SensorManager_FWD_DEFINED__ */


#ifndef __SensorCollection_FWD_DEFINED__
#define __SensorCollection_FWD_DEFINED__

#ifdef __cplusplus
typedef class SensorCollection SensorCollection;
#else
typedef struct SensorCollection SensorCollection;
#endif /* __cplusplus */

#endif 	/* __SensorCollection_FWD_DEFINED__ */


#ifndef __Sensor_FWD_DEFINED__
#define __Sensor_FWD_DEFINED__

#ifdef __cplusplus
typedef class Sensor Sensor;
#else
typedef struct Sensor Sensor;
#endif /* __cplusplus */

#endif 	/* __Sensor_FWD_DEFINED__ */


#ifndef __SensorDataReport_FWD_DEFINED__
#define __SensorDataReport_FWD_DEFINED__

#ifdef __cplusplus
typedef class SensorDataReport SensorDataReport;
#else
typedef struct SensorDataReport SensorDataReport;
#endif /* __cplusplus */

#endif 	/* __SensorDataReport_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"
#include "propsys.h"
#include "PortableDeviceTypes.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_sensorsapi_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
typedef /* [public][public][public][public] */ 
enum __MIDL___MIDL_itf_sensorsapi_0000_0000_0001
    {
        SENSOR_STATE_MIN	= 0,
        SENSOR_STATE_READY	= SENSOR_STATE_MIN,
        SENSOR_STATE_NOT_AVAILABLE	= ( SENSOR_STATE_READY + 1 ) ,
        SENSOR_STATE_NO_DATA	= ( SENSOR_STATE_NOT_AVAILABLE + 1 ) ,
        SENSOR_STATE_INITIALIZING	= ( SENSOR_STATE_NO_DATA + 1 ) ,
        SENSOR_STATE_ACCESS_DENIED	= ( SENSOR_STATE_INITIALIZING + 1 ) ,
        SENSOR_STATE_ERROR	= ( SENSOR_STATE_ACCESS_DENIED + 1 ) ,
        SENSOR_STATE_MAX	= SENSOR_STATE_ERROR
    } 	SensorState;

typedef /* [public] */ 
enum __MIDL___MIDL_itf_sensorsapi_0000_0000_0002
    {
        SENSOR_CONNECTION_TYPE_PC_INTEGRATED	= 0,
        SENSOR_CONNECTION_TYPE_PC_ATTACHED	= ( SENSOR_CONNECTION_TYPE_PC_INTEGRATED + 1 ) ,
        SENSOR_CONNECTION_TYPE_PC_EXTERNAL	= ( SENSOR_CONNECTION_TYPE_PC_ATTACHED + 1 ) 
    } 	SensorConnectionType;

typedef 
enum LOCATION_DESIRED_ACCURACY
    {
        LOCATION_DESIRED_ACCURACY_DEFAULT	= 0,
        LOCATION_DESIRED_ACCURACY_HIGH	= ( LOCATION_DESIRED_ACCURACY_DEFAULT + 1 ) 
    } 	LOCATION_DESIRED_ACCURACY;

typedef 
enum LOCATION_POSITION_SOURCE
    {
        LOCATION_POSITION_SOURCE_CELLULAR	= 0,
        LOCATION_POSITION_SOURCE_SATELLITE	= ( LOCATION_POSITION_SOURCE_CELLULAR + 1 ) ,
        LOCATION_POSITION_SOURCE_WIFI	= ( LOCATION_POSITION_SOURCE_SATELLITE + 1 ) ,
        LOCATION_POSITION_SOURCE_IPADDRESS	= ( LOCATION_POSITION_SOURCE_WIFI + 1 ) ,
        LOCATION_POSITION_SOURCE_UNKNOWN	= ( LOCATION_POSITION_SOURCE_IPADDRESS + 1 ) 
    } 	LOCATION_POSITION_SOURCE;

typedef 
enum SimpleDeviceOrientation
    {
        SIMPLE_DEVICE_ORIENTATION_NOT_ROTATED	= 0,
        SIMPLE_DEVICE_ORIENTATION_ROTATED_90	= ( SIMPLE_DEVICE_ORIENTATION_NOT_ROTATED + 1 ) ,
        SIMPLE_DEVICE_ORIENTATION_ROTATED_180	= ( SIMPLE_DEVICE_ORIENTATION_ROTATED_90 + 1 ) ,
        SIMPLE_DEVICE_ORIENTATION_ROTATED_270	= ( SIMPLE_DEVICE_ORIENTATION_ROTATED_180 + 1 ) ,
        SIMPLE_DEVICE_ORIENTATION_ROTATED_FACE_UP	= ( SIMPLE_DEVICE_ORIENTATION_ROTATED_270 + 1 ) ,
        SIMPLE_DEVICE_ORIENTATION_ROTATED_FACE_DOWN	= ( SIMPLE_DEVICE_ORIENTATION_ROTATED_FACE_UP + 1 ) 
    } 	SimpleDeviceOrientation;

typedef 
enum MagnetometerAccuracy
    {
        MAGNETOMETER_ACCURACY_UNKNOWN	= 0,
        MAGNETOMETER_ACCURACY_UNRELIABLE	= ( MAGNETOMETER_ACCURACY_UNKNOWN + 1 ) ,
        MAGNETOMETER_ACCURACY_APPROXIMATE	= ( MAGNETOMETER_ACCURACY_UNRELIABLE + 1 ) ,
        MAGNETOMETER_ACCURACY_HIGH	= ( MAGNETOMETER_ACCURACY_APPROXIMATE + 1 ) 
    } 	MagnetometerAccuracy;

typedef GUID SENSOR_CATEGORY_ID;

typedef REFGUID REFSENSOR_CATEGORY_ID;

typedef GUID SENSOR_TYPE_ID;

typedef REFGUID REFSENSOR_TYPE_ID;

typedef GUID SENSOR_ID;

typedef REFGUID REFSENSOR_ID;









extern RPC_IF_HANDLE __MIDL_itf_sensorsapi_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_sensorsapi_0000_0000_v0_0_s_ifspec;

#ifndef __ISensorManager_INTERFACE_DEFINED__
#define __ISensorManager_INTERFACE_DEFINED__

/* interface ISensorManager */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ISensorManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("BD77DB67-45A8-42DC-8D00-6DCF15F8377A")
    ISensorManager : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetSensorsByCategory( 
            /* [in] */ __RPC__in REFSENSOR_CATEGORY_ID sensorCategory,
            /* [out] */ __RPC__deref_out_opt ISensorCollection **ppSensorsFound) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetSensorsByType( 
            /* [in] */ __RPC__in REFSENSOR_TYPE_ID sensorType,
            /* [out] */ __RPC__deref_out_opt ISensorCollection **ppSensorsFound) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetSensorByID( 
            /* [in] */ __RPC__in REFSENSOR_ID sensorID,
            /* [out] */ __RPC__deref_out_opt ISensor **ppSensor) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetEventSink( 
            /* [in] */ __RPC__in_opt ISensorManagerEvents *pEvents) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE RequestPermissions( 
            /* [in] */ __RPC__in HWND hParent,
            /* [in] */ __RPC__in_opt ISensorCollection *pSensors,
            /* [in] */ BOOL fModal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISensorManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISensorManager * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISensorManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISensorManager * This);
        
        DECLSPEC_XFGVIRT(ISensorManager, GetSensorsByCategory)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetSensorsByCategory )( 
            __RPC__in ISensorManager * This,
            /* [in] */ __RPC__in REFSENSOR_CATEGORY_ID sensorCategory,
            /* [out] */ __RPC__deref_out_opt ISensorCollection **ppSensorsFound);
        
        DECLSPEC_XFGVIRT(ISensorManager, GetSensorsByType)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetSensorsByType )( 
            __RPC__in ISensorManager * This,
            /* [in] */ __RPC__in REFSENSOR_TYPE_ID sensorType,
            /* [out] */ __RPC__deref_out_opt ISensorCollection **ppSensorsFound);
        
        DECLSPEC_XFGVIRT(ISensorManager, GetSensorByID)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetSensorByID )( 
            __RPC__in ISensorManager * This,
            /* [in] */ __RPC__in REFSENSOR_ID sensorID,
            /* [out] */ __RPC__deref_out_opt ISensor **ppSensor);
        
        DECLSPEC_XFGVIRT(ISensorManager, SetEventSink)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetEventSink )( 
            __RPC__in ISensorManager * This,
            /* [in] */ __RPC__in_opt ISensorManagerEvents *pEvents);
        
        DECLSPEC_XFGVIRT(ISensorManager, RequestPermissions)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *RequestPermissions )( 
            __RPC__in ISensorManager * This,
            /* [in] */ __RPC__in HWND hParent,
            /* [in] */ __RPC__in_opt ISensorCollection *pSensors,
            /* [in] */ BOOL fModal);
        
        END_INTERFACE
    } ISensorManagerVtbl;

    interface ISensorManager
    {
        CONST_VTBL struct ISensorManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISensorManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISensorManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISensorManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISensorManager_GetSensorsByCategory(This,sensorCategory,ppSensorsFound)	\
    ( (This)->lpVtbl -> GetSensorsByCategory(This,sensorCategory,ppSensorsFound) ) 

#define ISensorManager_GetSensorsByType(This,sensorType,ppSensorsFound)	\
    ( (This)->lpVtbl -> GetSensorsByType(This,sensorType,ppSensorsFound) ) 

#define ISensorManager_GetSensorByID(This,sensorID,ppSensor)	\
    ( (This)->lpVtbl -> GetSensorByID(This,sensorID,ppSensor) ) 

#define ISensorManager_SetEventSink(This,pEvents)	\
    ( (This)->lpVtbl -> SetEventSink(This,pEvents) ) 

#define ISensorManager_RequestPermissions(This,hParent,pSensors,fModal)	\
    ( (This)->lpVtbl -> RequestPermissions(This,hParent,pSensors,fModal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISensorManager_INTERFACE_DEFINED__ */


#ifndef __ILocationPermissions_INTERFACE_DEFINED__
#define __ILocationPermissions_INTERFACE_DEFINED__

/* interface ILocationPermissions */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ILocationPermissions;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D5FB0A7F-E74E-44f5-8E02-4806863A274F")
    ILocationPermissions : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetGlobalLocationPermission( 
            /* [out] */ __RPC__out BOOL *pfEnabled) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE CheckLocationCapability( 
            /* [in] */ DWORD dwClientThreadId) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ILocationPermissionsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ILocationPermissions * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ILocationPermissions * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ILocationPermissions * This);
        
        DECLSPEC_XFGVIRT(ILocationPermissions, GetGlobalLocationPermission)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetGlobalLocationPermission )( 
            __RPC__in ILocationPermissions * This,
            /* [out] */ __RPC__out BOOL *pfEnabled);
        
        DECLSPEC_XFGVIRT(ILocationPermissions, CheckLocationCapability)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *CheckLocationCapability )( 
            __RPC__in ILocationPermissions * This,
            /* [in] */ DWORD dwClientThreadId);
        
        END_INTERFACE
    } ILocationPermissionsVtbl;

    interface ILocationPermissions
    {
        CONST_VTBL struct ILocationPermissionsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ILocationPermissions_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ILocationPermissions_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ILocationPermissions_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ILocationPermissions_GetGlobalLocationPermission(This,pfEnabled)	\
    ( (This)->lpVtbl -> GetGlobalLocationPermission(This,pfEnabled) ) 

#define ILocationPermissions_CheckLocationCapability(This,dwClientThreadId)	\
    ( (This)->lpVtbl -> CheckLocationCapability(This,dwClientThreadId) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ILocationPermissions_INTERFACE_DEFINED__ */


#ifndef __ISensorCollection_INTERFACE_DEFINED__
#define __ISensorCollection_INTERFACE_DEFINED__

/* interface ISensorCollection */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ISensorCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("23571E11-E545-4DD8-A337-B89BF44B10DF")
    ISensorCollection : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetAt( 
            /* [in] */ ULONG ulIndex,
            /* [out] */ __RPC__deref_out_opt ISensor **ppSensor) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetCount( 
            /* [out] */ __RPC__out ULONG *pCount) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Add( 
            /* [in] */ __RPC__in_opt ISensor *pSensor) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Remove( 
            /* [in] */ __RPC__in_opt ISensor *pSensor) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE RemoveByID( 
            /* [in] */ __RPC__in REFSENSOR_ID sensorID) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Clear( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISensorCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISensorCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISensorCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISensorCollection * This);
        
        DECLSPEC_XFGVIRT(ISensorCollection, GetAt)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetAt )( 
            __RPC__in ISensorCollection * This,
            /* [in] */ ULONG ulIndex,
            /* [out] */ __RPC__deref_out_opt ISensor **ppSensor);
        
        DECLSPEC_XFGVIRT(ISensorCollection, GetCount)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            __RPC__in ISensorCollection * This,
            /* [out] */ __RPC__out ULONG *pCount);
        
        DECLSPEC_XFGVIRT(ISensorCollection, Add)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Add )( 
            __RPC__in ISensorCollection * This,
            /* [in] */ __RPC__in_opt ISensor *pSensor);
        
        DECLSPEC_XFGVIRT(ISensorCollection, Remove)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Remove )( 
            __RPC__in ISensorCollection * This,
            /* [in] */ __RPC__in_opt ISensor *pSensor);
        
        DECLSPEC_XFGVIRT(ISensorCollection, RemoveByID)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *RemoveByID )( 
            __RPC__in ISensorCollection * This,
            /* [in] */ __RPC__in REFSENSOR_ID sensorID);
        
        DECLSPEC_XFGVIRT(ISensorCollection, Clear)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Clear )( 
            __RPC__in ISensorCollection * This);
        
        END_INTERFACE
    } ISensorCollectionVtbl;

    interface ISensorCollection
    {
        CONST_VTBL struct ISensorCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISensorCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISensorCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISensorCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISensorCollection_GetAt(This,ulIndex,ppSensor)	\
    ( (This)->lpVtbl -> GetAt(This,ulIndex,ppSensor) ) 

#define ISensorCollection_GetCount(This,pCount)	\
    ( (This)->lpVtbl -> GetCount(This,pCount) ) 

#define ISensorCollection_Add(This,pSensor)	\
    ( (This)->lpVtbl -> Add(This,pSensor) ) 

#define ISensorCollection_Remove(This,pSensor)	\
    ( (This)->lpVtbl -> Remove(This,pSensor) ) 

#define ISensorCollection_RemoveByID(This,sensorID)	\
    ( (This)->lpVtbl -> RemoveByID(This,sensorID) ) 

#define ISensorCollection_Clear(This)	\
    ( (This)->lpVtbl -> Clear(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISensorCollection_INTERFACE_DEFINED__ */


#ifndef __ISensor_INTERFACE_DEFINED__
#define __ISensor_INTERFACE_DEFINED__

/* interface ISensor */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ISensor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5FA08F80-2657-458E-AF75-46F73FA6AC5C")
    ISensor : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetID( 
            /* [out] */ __RPC__out SENSOR_ID *pID) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetCategory( 
            /* [out] */ __RPC__out SENSOR_CATEGORY_ID *pSensorCategory) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetType( 
            /* [out] */ __RPC__out SENSOR_TYPE_ID *pSensorType) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetFriendlyName( 
            /* [out] */ __RPC__deref_out_opt BSTR *pFriendlyName) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetProperty( 
            /* [in] */ __RPC__in REFPROPERTYKEY key,
            /* [out] */ __RPC__out PROPVARIANT *pProperty) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetProperties( 
            /* [in] */ __RPC__in_opt IPortableDeviceKeyCollection *pKeys,
            /* [out] */ __RPC__deref_out_opt IPortableDeviceValues **ppProperties) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetSupportedDataFields( 
            /* [out] */ __RPC__deref_out_opt IPortableDeviceKeyCollection **ppDataFields) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetProperties( 
            /* [in] */ __RPC__in_opt IPortableDeviceValues *pProperties,
            /* [out] */ __RPC__deref_out_opt IPortableDeviceValues **ppResults) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SupportsDataField( 
            /* [in] */ __RPC__in REFPROPERTYKEY key,
            /* [out] */ __RPC__out VARIANT_BOOL *pIsSupported) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetState( 
            /* [out] */ __RPC__out SensorState *pState) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetData( 
            /* [out] */ __RPC__deref_out_opt ISensorDataReport **ppDataReport) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SupportsEvent( 
            /* [in] */ __RPC__in REFGUID eventGuid,
            /* [out] */ __RPC__out VARIANT_BOOL *pIsSupported) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetEventInterest( 
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pCount) GUID **ppValues,
            /* [out] */ __RPC__out ULONG *pCount) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetEventInterest( 
            /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(count) GUID *pValues,
            /* [in] */ ULONG count) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetEventSink( 
            /* [in] */ __RPC__in_opt ISensorEvents *pEvents) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISensorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISensor * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISensor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISensor * This);
        
        DECLSPEC_XFGVIRT(ISensor, GetID)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetID )( 
            __RPC__in ISensor * This,
            /* [out] */ __RPC__out SENSOR_ID *pID);
        
        DECLSPEC_XFGVIRT(ISensor, GetCategory)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetCategory )( 
            __RPC__in ISensor * This,
            /* [out] */ __RPC__out SENSOR_CATEGORY_ID *pSensorCategory);
        
        DECLSPEC_XFGVIRT(ISensor, GetType)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetType )( 
            __RPC__in ISensor * This,
            /* [out] */ __RPC__out SENSOR_TYPE_ID *pSensorType);
        
        DECLSPEC_XFGVIRT(ISensor, GetFriendlyName)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetFriendlyName )( 
            __RPC__in ISensor * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pFriendlyName);
        
        DECLSPEC_XFGVIRT(ISensor, GetProperty)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetProperty )( 
            __RPC__in ISensor * This,
            /* [in] */ __RPC__in REFPROPERTYKEY key,
            /* [out] */ __RPC__out PROPVARIANT *pProperty);
        
        DECLSPEC_XFGVIRT(ISensor, GetProperties)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetProperties )( 
            __RPC__in ISensor * This,
            /* [in] */ __RPC__in_opt IPortableDeviceKeyCollection *pKeys,
            /* [out] */ __RPC__deref_out_opt IPortableDeviceValues **ppProperties);
        
        DECLSPEC_XFGVIRT(ISensor, GetSupportedDataFields)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetSupportedDataFields )( 
            __RPC__in ISensor * This,
            /* [out] */ __RPC__deref_out_opt IPortableDeviceKeyCollection **ppDataFields);
        
        DECLSPEC_XFGVIRT(ISensor, SetProperties)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetProperties )( 
            __RPC__in ISensor * This,
            /* [in] */ __RPC__in_opt IPortableDeviceValues *pProperties,
            /* [out] */ __RPC__deref_out_opt IPortableDeviceValues **ppResults);
        
        DECLSPEC_XFGVIRT(ISensor, SupportsDataField)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SupportsDataField )( 
            __RPC__in ISensor * This,
            /* [in] */ __RPC__in REFPROPERTYKEY key,
            /* [out] */ __RPC__out VARIANT_BOOL *pIsSupported);
        
        DECLSPEC_XFGVIRT(ISensor, GetState)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetState )( 
            __RPC__in ISensor * This,
            /* [out] */ __RPC__out SensorState *pState);
        
        DECLSPEC_XFGVIRT(ISensor, GetData)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetData )( 
            __RPC__in ISensor * This,
            /* [out] */ __RPC__deref_out_opt ISensorDataReport **ppDataReport);
        
        DECLSPEC_XFGVIRT(ISensor, SupportsEvent)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SupportsEvent )( 
            __RPC__in ISensor * This,
            /* [in] */ __RPC__in REFGUID eventGuid,
            /* [out] */ __RPC__out VARIANT_BOOL *pIsSupported);
        
        DECLSPEC_XFGVIRT(ISensor, GetEventInterest)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetEventInterest )( 
            __RPC__in ISensor * This,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pCount) GUID **ppValues,
            /* [out] */ __RPC__out ULONG *pCount);
        
        DECLSPEC_XFGVIRT(ISensor, SetEventInterest)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetEventInterest )( 
            __RPC__in ISensor * This,
            /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(count) GUID *pValues,
            /* [in] */ ULONG count);
        
        DECLSPEC_XFGVIRT(ISensor, SetEventSink)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetEventSink )( 
            __RPC__in ISensor * This,
            /* [in] */ __RPC__in_opt ISensorEvents *pEvents);
        
        END_INTERFACE
    } ISensorVtbl;

    interface ISensor
    {
        CONST_VTBL struct ISensorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISensor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISensor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISensor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISensor_GetID(This,pID)	\
    ( (This)->lpVtbl -> GetID(This,pID) ) 

#define ISensor_GetCategory(This,pSensorCategory)	\
    ( (This)->lpVtbl -> GetCategory(This,pSensorCategory) ) 

#define ISensor_GetType(This,pSensorType)	\
    ( (This)->lpVtbl -> GetType(This,pSensorType) ) 

#define ISensor_GetFriendlyName(This,pFriendlyName)	\
    ( (This)->lpVtbl -> GetFriendlyName(This,pFriendlyName) ) 

#define ISensor_GetProperty(This,key,pProperty)	\
    ( (This)->lpVtbl -> GetProperty(This,key,pProperty) ) 

#define ISensor_GetProperties(This,pKeys,ppProperties)	\
    ( (This)->lpVtbl -> GetProperties(This,pKeys,ppProperties) ) 

#define ISensor_GetSupportedDataFields(This,ppDataFields)	\
    ( (This)->lpVtbl -> GetSupportedDataFields(This,ppDataFields) ) 

#define ISensor_SetProperties(This,pProperties,ppResults)	\
    ( (This)->lpVtbl -> SetProperties(This,pProperties,ppResults) ) 

#define ISensor_SupportsDataField(This,key,pIsSupported)	\
    ( (This)->lpVtbl -> SupportsDataField(This,key,pIsSupported) ) 

#define ISensor_GetState(This,pState)	\
    ( (This)->lpVtbl -> GetState(This,pState) ) 

#define ISensor_GetData(This,ppDataReport)	\
    ( (This)->lpVtbl -> GetData(This,ppDataReport) ) 

#define ISensor_SupportsEvent(This,eventGuid,pIsSupported)	\
    ( (This)->lpVtbl -> SupportsEvent(This,eventGuid,pIsSupported) ) 

#define ISensor_GetEventInterest(This,ppValues,pCount)	\
    ( (This)->lpVtbl -> GetEventInterest(This,ppValues,pCount) ) 

#define ISensor_SetEventInterest(This,pValues,count)	\
    ( (This)->lpVtbl -> SetEventInterest(This,pValues,count) ) 

#define ISensor_SetEventSink(This,pEvents)	\
    ( (This)->lpVtbl -> SetEventSink(This,pEvents) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISensor_INTERFACE_DEFINED__ */


#ifndef __ISensorDataReport_INTERFACE_DEFINED__
#define __ISensorDataReport_INTERFACE_DEFINED__

/* interface ISensorDataReport */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ISensorDataReport;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0AB9DF9B-C4B5-4796-8898-0470706A2E1D")
    ISensorDataReport : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetTimestamp( 
            /* [out] */ __RPC__out SYSTEMTIME *pTimeStamp) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetSensorValue( 
            /* [in] */ __RPC__in REFPROPERTYKEY pKey,
            /* [out] */ __RPC__out PROPVARIANT *pValue) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetSensorValues( 
            /* [in] */ __RPC__in_opt IPortableDeviceKeyCollection *pKeys,
            /* [out] */ __RPC__deref_out_opt IPortableDeviceValues **ppValues) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISensorDataReportVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISensorDataReport * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISensorDataReport * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISensorDataReport * This);
        
        DECLSPEC_XFGVIRT(ISensorDataReport, GetTimestamp)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetTimestamp )( 
            __RPC__in ISensorDataReport * This,
            /* [out] */ __RPC__out SYSTEMTIME *pTimeStamp);
        
        DECLSPEC_XFGVIRT(ISensorDataReport, GetSensorValue)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetSensorValue )( 
            __RPC__in ISensorDataReport * This,
            /* [in] */ __RPC__in REFPROPERTYKEY pKey,
            /* [out] */ __RPC__out PROPVARIANT *pValue);
        
        DECLSPEC_XFGVIRT(ISensorDataReport, GetSensorValues)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetSensorValues )( 
            __RPC__in ISensorDataReport * This,
            /* [in] */ __RPC__in_opt IPortableDeviceKeyCollection *pKeys,
            /* [out] */ __RPC__deref_out_opt IPortableDeviceValues **ppValues);
        
        END_INTERFACE
    } ISensorDataReportVtbl;

    interface ISensorDataReport
    {
        CONST_VTBL struct ISensorDataReportVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISensorDataReport_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISensorDataReport_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISensorDataReport_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISensorDataReport_GetTimestamp(This,pTimeStamp)	\
    ( (This)->lpVtbl -> GetTimestamp(This,pTimeStamp) ) 

#define ISensorDataReport_GetSensorValue(This,pKey,pValue)	\
    ( (This)->lpVtbl -> GetSensorValue(This,pKey,pValue) ) 

#define ISensorDataReport_GetSensorValues(This,pKeys,ppValues)	\
    ( (This)->lpVtbl -> GetSensorValues(This,pKeys,ppValues) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISensorDataReport_INTERFACE_DEFINED__ */


#ifndef __ISensorManagerEvents_INTERFACE_DEFINED__
#define __ISensorManagerEvents_INTERFACE_DEFINED__

/* interface ISensorManagerEvents */
/* [object][helpstring][uuid] */ 


EXTERN_C const IID IID_ISensorManagerEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9B3B0B86-266A-4AAD-B21F-FDE5501001B7")
    ISensorManagerEvents : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE OnSensorEnter( 
            /* [in] */ __RPC__in_opt ISensor *pSensor,
            /* [in] */ SensorState state) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISensorManagerEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISensorManagerEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISensorManagerEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISensorManagerEvents * This);
        
        DECLSPEC_XFGVIRT(ISensorManagerEvents, OnSensorEnter)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *OnSensorEnter )( 
            __RPC__in ISensorManagerEvents * This,
            /* [in] */ __RPC__in_opt ISensor *pSensor,
            /* [in] */ SensorState state);
        
        END_INTERFACE
    } ISensorManagerEventsVtbl;

    interface ISensorManagerEvents
    {
        CONST_VTBL struct ISensorManagerEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISensorManagerEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISensorManagerEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISensorManagerEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISensorManagerEvents_OnSensorEnter(This,pSensor,state)	\
    ( (This)->lpVtbl -> OnSensorEnter(This,pSensor,state) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISensorManagerEvents_INTERFACE_DEFINED__ */


#ifndef __ISensorEvents_INTERFACE_DEFINED__
#define __ISensorEvents_INTERFACE_DEFINED__

/* interface ISensorEvents */
/* [object][helpstring][uuid] */ 


EXTERN_C const IID IID_ISensorEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5D8DCC91-4641-47E7-B7C3-B74F48A6C391")
    ISensorEvents : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE OnStateChanged( 
            /* [in] */ __RPC__in_opt ISensor *pSensor,
            /* [in] */ SensorState state) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE OnDataUpdated( 
            /* [in] */ __RPC__in_opt ISensor *pSensor,
            /* [in] */ __RPC__in_opt ISensorDataReport *pNewData) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE OnEvent( 
            /* [in] */ __RPC__in_opt ISensor *pSensor,
            /* [in] */ __RPC__in REFGUID eventID,
            /* [in] */ __RPC__in_opt IPortableDeviceValues *pEventData) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE OnLeave( 
            /* [in] */ __RPC__in REFSENSOR_ID ID) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISensorEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISensorEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISensorEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISensorEvents * This);
        
        DECLSPEC_XFGVIRT(ISensorEvents, OnStateChanged)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *OnStateChanged )( 
            __RPC__in ISensorEvents * This,
            /* [in] */ __RPC__in_opt ISensor *pSensor,
            /* [in] */ SensorState state);
        
        DECLSPEC_XFGVIRT(ISensorEvents, OnDataUpdated)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *OnDataUpdated )( 
            __RPC__in ISensorEvents * This,
            /* [in] */ __RPC__in_opt ISensor *pSensor,
            /* [in] */ __RPC__in_opt ISensorDataReport *pNewData);
        
        DECLSPEC_XFGVIRT(ISensorEvents, OnEvent)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *OnEvent )( 
            __RPC__in ISensorEvents * This,
            /* [in] */ __RPC__in_opt ISensor *pSensor,
            /* [in] */ __RPC__in REFGUID eventID,
            /* [in] */ __RPC__in_opt IPortableDeviceValues *pEventData);
        
        DECLSPEC_XFGVIRT(ISensorEvents, OnLeave)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *OnLeave )( 
            __RPC__in ISensorEvents * This,
            /* [in] */ __RPC__in REFSENSOR_ID ID);
        
        END_INTERFACE
    } ISensorEventsVtbl;

    interface ISensorEvents
    {
        CONST_VTBL struct ISensorEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISensorEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISensorEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISensorEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISensorEvents_OnStateChanged(This,pSensor,state)	\
    ( (This)->lpVtbl -> OnStateChanged(This,pSensor,state) ) 

#define ISensorEvents_OnDataUpdated(This,pSensor,pNewData)	\
    ( (This)->lpVtbl -> OnDataUpdated(This,pSensor,pNewData) ) 

#define ISensorEvents_OnEvent(This,pSensor,eventID,pEventData)	\
    ( (This)->lpVtbl -> OnEvent(This,pSensor,eventID,pEventData) ) 

#define ISensorEvents_OnLeave(This,ID)	\
    ( (This)->lpVtbl -> OnLeave(This,ID) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISensorEvents_INTERFACE_DEFINED__ */



#ifndef __SensorsApiLib_LIBRARY_DEFINED__
#define __SensorsApiLib_LIBRARY_DEFINED__

/* library SensorsApiLib */
/* [helpstring][version][uuid] */ 


EXTERN_C const IID LIBID_SensorsApiLib;

EXTERN_C const CLSID CLSID_SensorManager;

#ifdef __cplusplus

class DECLSPEC_UUID("77A1C827-FCD2-4689-8915-9D613CC5FA3E")
SensorManager;
#endif

EXTERN_C const CLSID CLSID_SensorCollection;

#ifdef __cplusplus

class DECLSPEC_UUID("79C43ADB-A429-469F-AA39-2F2B74B75937")
SensorCollection;
#endif

EXTERN_C const CLSID CLSID_Sensor;

#ifdef __cplusplus

class DECLSPEC_UUID("E97CED00-523A-4133-BF6F-D3A2DAE7F6BA")
Sensor;
#endif

EXTERN_C const CLSID CLSID_SensorDataReport;

#ifdef __cplusplus

class DECLSPEC_UUID("4EA9D6EF-694B-4218-8816-CCDA8DA74BBA")
SensorDataReport;
#endif
#endif /* __SensorsApiLib_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_sensorsapi_0000_0008 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_sensorsapi_0000_0008_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_sensorsapi_0000_0008_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  BSTR_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  HWND_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HWND * ); 
void                      __RPC_USER  HWND_UserFree(     __RPC__in unsigned long *, __RPC__in HWND * ); 

unsigned long             __RPC_USER  LPSAFEARRAY_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out LPSAFEARRAY * ); 
void                      __RPC_USER  LPSAFEARRAY_UserFree(     __RPC__in unsigned long *, __RPC__in LPSAFEARRAY * ); 

unsigned long             __RPC_USER  BSTR_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree64(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  HWND_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HWND * ); 
void                      __RPC_USER  HWND_UserFree64(     __RPC__in unsigned long *, __RPC__in HWND * ); 

unsigned long             __RPC_USER  LPSAFEARRAY_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out LPSAFEARRAY * ); 
void                      __RPC_USER  LPSAFEARRAY_UserFree64(     __RPC__in unsigned long *, __RPC__in LPSAFEARRAY * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


