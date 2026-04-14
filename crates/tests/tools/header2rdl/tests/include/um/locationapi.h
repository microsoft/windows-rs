

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

#ifndef __locationapi_h__
#define __locationapi_h__

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

#ifndef __ILocationReport_FWD_DEFINED__
#define __ILocationReport_FWD_DEFINED__
typedef interface ILocationReport ILocationReport;

#endif 	/* __ILocationReport_FWD_DEFINED__ */


#ifndef __ILatLongReport_FWD_DEFINED__
#define __ILatLongReport_FWD_DEFINED__
typedef interface ILatLongReport ILatLongReport;

#endif 	/* __ILatLongReport_FWD_DEFINED__ */


#ifndef __ICivicAddressReport_FWD_DEFINED__
#define __ICivicAddressReport_FWD_DEFINED__
typedef interface ICivicAddressReport ICivicAddressReport;

#endif 	/* __ICivicAddressReport_FWD_DEFINED__ */


#ifndef __ILocation_FWD_DEFINED__
#define __ILocation_FWD_DEFINED__
typedef interface ILocation ILocation;

#endif 	/* __ILocation_FWD_DEFINED__ */


#ifndef __ILocationPower_FWD_DEFINED__
#define __ILocationPower_FWD_DEFINED__
typedef interface ILocationPower ILocationPower;

#endif 	/* __ILocationPower_FWD_DEFINED__ */


#ifndef __IDefaultLocation_FWD_DEFINED__
#define __IDefaultLocation_FWD_DEFINED__
typedef interface IDefaultLocation IDefaultLocation;

#endif 	/* __IDefaultLocation_FWD_DEFINED__ */


#ifndef __ILocationEvents_FWD_DEFINED__
#define __ILocationEvents_FWD_DEFINED__
typedef interface ILocationEvents ILocationEvents;

#endif 	/* __ILocationEvents_FWD_DEFINED__ */


#ifndef __IDispLatLongReport_FWD_DEFINED__
#define __IDispLatLongReport_FWD_DEFINED__
typedef interface IDispLatLongReport IDispLatLongReport;

#endif 	/* __IDispLatLongReport_FWD_DEFINED__ */


#ifndef __IDispCivicAddressReport_FWD_DEFINED__
#define __IDispCivicAddressReport_FWD_DEFINED__
typedef interface IDispCivicAddressReport IDispCivicAddressReport;

#endif 	/* __IDispCivicAddressReport_FWD_DEFINED__ */


#ifndef __ILocationReportFactory_FWD_DEFINED__
#define __ILocationReportFactory_FWD_DEFINED__
typedef interface ILocationReportFactory ILocationReportFactory;

#endif 	/* __ILocationReportFactory_FWD_DEFINED__ */


#ifndef __ILatLongReportFactory_FWD_DEFINED__
#define __ILatLongReportFactory_FWD_DEFINED__
typedef interface ILatLongReportFactory ILatLongReportFactory;

#endif 	/* __ILatLongReportFactory_FWD_DEFINED__ */


#ifndef __ICivicAddressReportFactory_FWD_DEFINED__
#define __ICivicAddressReportFactory_FWD_DEFINED__
typedef interface ICivicAddressReportFactory ICivicAddressReportFactory;

#endif 	/* __ICivicAddressReportFactory_FWD_DEFINED__ */


#ifndef __Location_FWD_DEFINED__
#define __Location_FWD_DEFINED__

#ifdef __cplusplus
typedef class Location Location;
#else
typedef struct Location Location;
#endif /* __cplusplus */

#endif 	/* __Location_FWD_DEFINED__ */


#ifndef __DefaultLocation_FWD_DEFINED__
#define __DefaultLocation_FWD_DEFINED__

#ifdef __cplusplus
typedef class DefaultLocation DefaultLocation;
#else
typedef struct DefaultLocation DefaultLocation;
#endif /* __cplusplus */

#endif 	/* __DefaultLocation_FWD_DEFINED__ */


#ifndef __LatLongReport_FWD_DEFINED__
#define __LatLongReport_FWD_DEFINED__

#ifdef __cplusplus
typedef class LatLongReport LatLongReport;
#else
typedef struct LatLongReport LatLongReport;
#endif /* __cplusplus */

#endif 	/* __LatLongReport_FWD_DEFINED__ */


#ifndef __CivicAddressReport_FWD_DEFINED__
#define __CivicAddressReport_FWD_DEFINED__

#ifdef __cplusplus
typedef class CivicAddressReport CivicAddressReport;
#else
typedef struct CivicAddressReport CivicAddressReport;
#endif /* __cplusplus */

#endif 	/* __CivicAddressReport_FWD_DEFINED__ */


#ifndef ___ILatLongReportFactoryEvents_FWD_DEFINED__
#define ___ILatLongReportFactoryEvents_FWD_DEFINED__
typedef interface _ILatLongReportFactoryEvents _ILatLongReportFactoryEvents;

#endif 	/* ___ILatLongReportFactoryEvents_FWD_DEFINED__ */


#ifndef ___ICivicAddressReportFactoryEvents_FWD_DEFINED__
#define ___ICivicAddressReportFactoryEvents_FWD_DEFINED__
typedef interface _ICivicAddressReportFactoryEvents _ICivicAddressReportFactoryEvents;

#endif 	/* ___ICivicAddressReportFactoryEvents_FWD_DEFINED__ */


#ifndef __LatLongReportFactory_FWD_DEFINED__
#define __LatLongReportFactory_FWD_DEFINED__

#ifdef __cplusplus
typedef class LatLongReportFactory LatLongReportFactory;
#else
typedef struct LatLongReportFactory LatLongReportFactory;
#endif /* __cplusplus */

#endif 	/* __LatLongReportFactory_FWD_DEFINED__ */


#ifndef __CivicAddressReportFactory_FWD_DEFINED__
#define __CivicAddressReportFactory_FWD_DEFINED__

#ifdef __cplusplus
typedef class CivicAddressReportFactory CivicAddressReportFactory;
#else
typedef struct CivicAddressReportFactory CivicAddressReportFactory;
#endif /* __cplusplus */

#endif 	/* __CivicAddressReportFactory_FWD_DEFINED__ */


#ifndef __DispLatLongReport_FWD_DEFINED__
#define __DispLatLongReport_FWD_DEFINED__

#ifdef __cplusplus
typedef class DispLatLongReport DispLatLongReport;
#else
typedef struct DispLatLongReport DispLatLongReport;
#endif /* __cplusplus */

#endif 	/* __DispLatLongReport_FWD_DEFINED__ */


#ifndef __DispCivicAddressReport_FWD_DEFINED__
#define __DispCivicAddressReport_FWD_DEFINED__

#ifdef __cplusplus
typedef class DispCivicAddressReport DispCivicAddressReport;
#else
typedef struct DispCivicAddressReport DispCivicAddressReport;
#endif /* __cplusplus */

#endif 	/* __DispCivicAddressReport_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"
#include "SensorsApi.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_locationapi_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#include <SensorsApi.h>
#include <initguid.h>
#include <propkeydef.h>
#define LOCATION_API_VERSION    1
typedef 
enum LOCATION_REPORT_STATUS
    {
        REPORT_NOT_SUPPORTED	= 0,
        REPORT_ERROR	= 1,
        REPORT_ACCESS_DENIED	= 2,
        REPORT_INITIALIZING	= 3,
        REPORT_RUNNING	= 4
    } 	LOCATION_REPORT_STATUS;






extern RPC_IF_HANDLE __MIDL_itf_locationapi_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_locationapi_0000_0000_v0_0_s_ifspec;

#ifndef __ILocationReport_INTERFACE_DEFINED__
#define __ILocationReport_INTERFACE_DEFINED__

/* interface ILocationReport */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ILocationReport;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C8B7F7EE-75D0-4db9-B62D-7A0F369CA456")
    ILocationReport : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetSensorID( 
            /* [retval][out] */ __RPC__out SENSOR_ID *pSensorID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTimestamp( 
            /* [retval][out] */ __RPC__out SYSTEMTIME *pCreationTime) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetValue( 
            /* [in] */ __RPC__in REFPROPERTYKEY pKey,
            /* [retval][out] */ __RPC__out PROPVARIANT *pValue) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ILocationReportVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ILocationReport * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ILocationReport * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ILocationReport * This);
        
        DECLSPEC_XFGVIRT(ILocationReport, GetSensorID)
        HRESULT ( STDMETHODCALLTYPE *GetSensorID )( 
            __RPC__in ILocationReport * This,
            /* [retval][out] */ __RPC__out SENSOR_ID *pSensorID);
        
        DECLSPEC_XFGVIRT(ILocationReport, GetTimestamp)
        HRESULT ( STDMETHODCALLTYPE *GetTimestamp )( 
            __RPC__in ILocationReport * This,
            /* [retval][out] */ __RPC__out SYSTEMTIME *pCreationTime);
        
        DECLSPEC_XFGVIRT(ILocationReport, GetValue)
        HRESULT ( STDMETHODCALLTYPE *GetValue )( 
            __RPC__in ILocationReport * This,
            /* [in] */ __RPC__in REFPROPERTYKEY pKey,
            /* [retval][out] */ __RPC__out PROPVARIANT *pValue);
        
        END_INTERFACE
    } ILocationReportVtbl;

    interface ILocationReport
    {
        CONST_VTBL struct ILocationReportVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ILocationReport_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ILocationReport_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ILocationReport_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ILocationReport_GetSensorID(This,pSensorID)	\
    ( (This)->lpVtbl -> GetSensorID(This,pSensorID) ) 

#define ILocationReport_GetTimestamp(This,pCreationTime)	\
    ( (This)->lpVtbl -> GetTimestamp(This,pCreationTime) ) 

#define ILocationReport_GetValue(This,pKey,pValue)	\
    ( (This)->lpVtbl -> GetValue(This,pKey,pValue) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ILocationReport_INTERFACE_DEFINED__ */


#ifndef __ILatLongReport_INTERFACE_DEFINED__
#define __ILatLongReport_INTERFACE_DEFINED__

/* interface ILatLongReport */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ILatLongReport;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7FED806D-0EF8-4f07-80AC-36A0BEAE3134")
    ILatLongReport : public ILocationReport
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetLatitude( 
            /* [retval][out] */ __RPC__out DOUBLE *pLatitude) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLongitude( 
            /* [retval][out] */ __RPC__out DOUBLE *pLongitude) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetErrorRadius( 
            /* [retval][out] */ __RPC__out DOUBLE *pErrorRadius) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAltitude( 
            /* [retval][out] */ __RPC__out DOUBLE *pAltitude) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAltitudeError( 
            /* [retval][out] */ __RPC__out DOUBLE *pAltitudeError) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ILatLongReportVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ILatLongReport * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ILatLongReport * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ILatLongReport * This);
        
        DECLSPEC_XFGVIRT(ILocationReport, GetSensorID)
        HRESULT ( STDMETHODCALLTYPE *GetSensorID )( 
            __RPC__in ILatLongReport * This,
            /* [retval][out] */ __RPC__out SENSOR_ID *pSensorID);
        
        DECLSPEC_XFGVIRT(ILocationReport, GetTimestamp)
        HRESULT ( STDMETHODCALLTYPE *GetTimestamp )( 
            __RPC__in ILatLongReport * This,
            /* [retval][out] */ __RPC__out SYSTEMTIME *pCreationTime);
        
        DECLSPEC_XFGVIRT(ILocationReport, GetValue)
        HRESULT ( STDMETHODCALLTYPE *GetValue )( 
            __RPC__in ILatLongReport * This,
            /* [in] */ __RPC__in REFPROPERTYKEY pKey,
            /* [retval][out] */ __RPC__out PROPVARIANT *pValue);
        
        DECLSPEC_XFGVIRT(ILatLongReport, GetLatitude)
        HRESULT ( STDMETHODCALLTYPE *GetLatitude )( 
            __RPC__in ILatLongReport * This,
            /* [retval][out] */ __RPC__out DOUBLE *pLatitude);
        
        DECLSPEC_XFGVIRT(ILatLongReport, GetLongitude)
        HRESULT ( STDMETHODCALLTYPE *GetLongitude )( 
            __RPC__in ILatLongReport * This,
            /* [retval][out] */ __RPC__out DOUBLE *pLongitude);
        
        DECLSPEC_XFGVIRT(ILatLongReport, GetErrorRadius)
        HRESULT ( STDMETHODCALLTYPE *GetErrorRadius )( 
            __RPC__in ILatLongReport * This,
            /* [retval][out] */ __RPC__out DOUBLE *pErrorRadius);
        
        DECLSPEC_XFGVIRT(ILatLongReport, GetAltitude)
        HRESULT ( STDMETHODCALLTYPE *GetAltitude )( 
            __RPC__in ILatLongReport * This,
            /* [retval][out] */ __RPC__out DOUBLE *pAltitude);
        
        DECLSPEC_XFGVIRT(ILatLongReport, GetAltitudeError)
        HRESULT ( STDMETHODCALLTYPE *GetAltitudeError )( 
            __RPC__in ILatLongReport * This,
            /* [retval][out] */ __RPC__out DOUBLE *pAltitudeError);
        
        END_INTERFACE
    } ILatLongReportVtbl;

    interface ILatLongReport
    {
        CONST_VTBL struct ILatLongReportVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ILatLongReport_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ILatLongReport_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ILatLongReport_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ILatLongReport_GetSensorID(This,pSensorID)	\
    ( (This)->lpVtbl -> GetSensorID(This,pSensorID) ) 

#define ILatLongReport_GetTimestamp(This,pCreationTime)	\
    ( (This)->lpVtbl -> GetTimestamp(This,pCreationTime) ) 

#define ILatLongReport_GetValue(This,pKey,pValue)	\
    ( (This)->lpVtbl -> GetValue(This,pKey,pValue) ) 


#define ILatLongReport_GetLatitude(This,pLatitude)	\
    ( (This)->lpVtbl -> GetLatitude(This,pLatitude) ) 

#define ILatLongReport_GetLongitude(This,pLongitude)	\
    ( (This)->lpVtbl -> GetLongitude(This,pLongitude) ) 

#define ILatLongReport_GetErrorRadius(This,pErrorRadius)	\
    ( (This)->lpVtbl -> GetErrorRadius(This,pErrorRadius) ) 

#define ILatLongReport_GetAltitude(This,pAltitude)	\
    ( (This)->lpVtbl -> GetAltitude(This,pAltitude) ) 

#define ILatLongReport_GetAltitudeError(This,pAltitudeError)	\
    ( (This)->lpVtbl -> GetAltitudeError(This,pAltitudeError) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ILatLongReport_INTERFACE_DEFINED__ */


#ifndef __ICivicAddressReport_INTERFACE_DEFINED__
#define __ICivicAddressReport_INTERFACE_DEFINED__

/* interface ICivicAddressReport */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ICivicAddressReport;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C0B19F70-4ADF-445d-87F2-CAD8FD711792")
    ICivicAddressReport : public ILocationReport
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetAddressLine1( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrAddress1) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAddressLine2( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrAddress2) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCity( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrCity) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStateProvince( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrStateProvince) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPostalCode( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrPostalCode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCountryRegion( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrCountryRegion) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDetailLevel( 
            /* [retval][out] */ __RPC__out DWORD *pDetailLevel) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICivicAddressReportVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICivicAddressReport * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICivicAddressReport * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICivicAddressReport * This);
        
        DECLSPEC_XFGVIRT(ILocationReport, GetSensorID)
        HRESULT ( STDMETHODCALLTYPE *GetSensorID )( 
            __RPC__in ICivicAddressReport * This,
            /* [retval][out] */ __RPC__out SENSOR_ID *pSensorID);
        
        DECLSPEC_XFGVIRT(ILocationReport, GetTimestamp)
        HRESULT ( STDMETHODCALLTYPE *GetTimestamp )( 
            __RPC__in ICivicAddressReport * This,
            /* [retval][out] */ __RPC__out SYSTEMTIME *pCreationTime);
        
        DECLSPEC_XFGVIRT(ILocationReport, GetValue)
        HRESULT ( STDMETHODCALLTYPE *GetValue )( 
            __RPC__in ICivicAddressReport * This,
            /* [in] */ __RPC__in REFPROPERTYKEY pKey,
            /* [retval][out] */ __RPC__out PROPVARIANT *pValue);
        
        DECLSPEC_XFGVIRT(ICivicAddressReport, GetAddressLine1)
        HRESULT ( STDMETHODCALLTYPE *GetAddressLine1 )( 
            __RPC__in ICivicAddressReport * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrAddress1);
        
        DECLSPEC_XFGVIRT(ICivicAddressReport, GetAddressLine2)
        HRESULT ( STDMETHODCALLTYPE *GetAddressLine2 )( 
            __RPC__in ICivicAddressReport * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrAddress2);
        
        DECLSPEC_XFGVIRT(ICivicAddressReport, GetCity)
        HRESULT ( STDMETHODCALLTYPE *GetCity )( 
            __RPC__in ICivicAddressReport * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrCity);
        
        DECLSPEC_XFGVIRT(ICivicAddressReport, GetStateProvince)
        HRESULT ( STDMETHODCALLTYPE *GetStateProvince )( 
            __RPC__in ICivicAddressReport * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrStateProvince);
        
        DECLSPEC_XFGVIRT(ICivicAddressReport, GetPostalCode)
        HRESULT ( STDMETHODCALLTYPE *GetPostalCode )( 
            __RPC__in ICivicAddressReport * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrPostalCode);
        
        DECLSPEC_XFGVIRT(ICivicAddressReport, GetCountryRegion)
        HRESULT ( STDMETHODCALLTYPE *GetCountryRegion )( 
            __RPC__in ICivicAddressReport * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrCountryRegion);
        
        DECLSPEC_XFGVIRT(ICivicAddressReport, GetDetailLevel)
        HRESULT ( STDMETHODCALLTYPE *GetDetailLevel )( 
            __RPC__in ICivicAddressReport * This,
            /* [retval][out] */ __RPC__out DWORD *pDetailLevel);
        
        END_INTERFACE
    } ICivicAddressReportVtbl;

    interface ICivicAddressReport
    {
        CONST_VTBL struct ICivicAddressReportVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICivicAddressReport_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICivicAddressReport_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICivicAddressReport_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICivicAddressReport_GetSensorID(This,pSensorID)	\
    ( (This)->lpVtbl -> GetSensorID(This,pSensorID) ) 

#define ICivicAddressReport_GetTimestamp(This,pCreationTime)	\
    ( (This)->lpVtbl -> GetTimestamp(This,pCreationTime) ) 

#define ICivicAddressReport_GetValue(This,pKey,pValue)	\
    ( (This)->lpVtbl -> GetValue(This,pKey,pValue) ) 


#define ICivicAddressReport_GetAddressLine1(This,pbstrAddress1)	\
    ( (This)->lpVtbl -> GetAddressLine1(This,pbstrAddress1) ) 

#define ICivicAddressReport_GetAddressLine2(This,pbstrAddress2)	\
    ( (This)->lpVtbl -> GetAddressLine2(This,pbstrAddress2) ) 

#define ICivicAddressReport_GetCity(This,pbstrCity)	\
    ( (This)->lpVtbl -> GetCity(This,pbstrCity) ) 

#define ICivicAddressReport_GetStateProvince(This,pbstrStateProvince)	\
    ( (This)->lpVtbl -> GetStateProvince(This,pbstrStateProvince) ) 

#define ICivicAddressReport_GetPostalCode(This,pbstrPostalCode)	\
    ( (This)->lpVtbl -> GetPostalCode(This,pbstrPostalCode) ) 

#define ICivicAddressReport_GetCountryRegion(This,pbstrCountryRegion)	\
    ( (This)->lpVtbl -> GetCountryRegion(This,pbstrCountryRegion) ) 

#define ICivicAddressReport_GetDetailLevel(This,pDetailLevel)	\
    ( (This)->lpVtbl -> GetDetailLevel(This,pDetailLevel) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICivicAddressReport_INTERFACE_DEFINED__ */


#ifndef __ILocation_INTERFACE_DEFINED__
#define __ILocation_INTERFACE_DEFINED__

/* interface ILocation */
/* [unique][uuid][object] */ 



EXTERN_C const IID IID_ILocation;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("AB2ECE69-56D9-4F28-B525-DE1B0EE44237")
    ILocation : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE RegisterForReport( 
            /* [in] */ __RPC__in_opt ILocationEvents *pEvents,
            /* [in] */ __RPC__in REFIID reportType,
            /* [in] */ DWORD dwRequestedReportInterval) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnregisterForReport( 
            /* [in] */ __RPC__in REFIID reportType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetReport( 
            /* [in] */ __RPC__in REFIID reportType,
            /* [retval][out] */ __RPC__deref_out_opt ILocationReport **ppLocationReport) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetReportStatus( 
            /* [in] */ __RPC__in REFIID reportType,
            /* [retval][out] */ __RPC__out enum LOCATION_REPORT_STATUS *pStatus) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetReportInterval( 
            /* [in] */ __RPC__in REFIID reportType,
            /* [retval][out] */ __RPC__out DWORD *pMilliseconds) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetReportInterval( 
            /* [in] */ __RPC__in REFIID reportType,
            /* [in] */ DWORD millisecondsRequested) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDesiredAccuracy( 
            /* [in] */ __RPC__in REFIID reportType,
            /* [retval][out] */ __RPC__out enum LOCATION_DESIRED_ACCURACY *pDesiredAccuracy) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetDesiredAccuracy( 
            /* [in] */ __RPC__in REFIID reportType,
            /* [in] */ enum LOCATION_DESIRED_ACCURACY desiredAccuracy) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RequestPermissions( 
            /* [unique][in] */ __RPC__in_opt HWND hParent,
            /* [size_is][in] */ __RPC__in_ecount_full(count) IID *pReportTypes,
            /* [in] */ ULONG count,
            /* [in] */ BOOL fModal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ILocationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ILocation * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ILocation * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ILocation * This);
        
        DECLSPEC_XFGVIRT(ILocation, RegisterForReport)
        HRESULT ( STDMETHODCALLTYPE *RegisterForReport )( 
            __RPC__in ILocation * This,
            /* [in] */ __RPC__in_opt ILocationEvents *pEvents,
            /* [in] */ __RPC__in REFIID reportType,
            /* [in] */ DWORD dwRequestedReportInterval);
        
        DECLSPEC_XFGVIRT(ILocation, UnregisterForReport)
        HRESULT ( STDMETHODCALLTYPE *UnregisterForReport )( 
            __RPC__in ILocation * This,
            /* [in] */ __RPC__in REFIID reportType);
        
        DECLSPEC_XFGVIRT(ILocation, GetReport)
        HRESULT ( STDMETHODCALLTYPE *GetReport )( 
            __RPC__in ILocation * This,
            /* [in] */ __RPC__in REFIID reportType,
            /* [retval][out] */ __RPC__deref_out_opt ILocationReport **ppLocationReport);
        
        DECLSPEC_XFGVIRT(ILocation, GetReportStatus)
        HRESULT ( STDMETHODCALLTYPE *GetReportStatus )( 
            __RPC__in ILocation * This,
            /* [in] */ __RPC__in REFIID reportType,
            /* [retval][out] */ __RPC__out enum LOCATION_REPORT_STATUS *pStatus);
        
        DECLSPEC_XFGVIRT(ILocation, GetReportInterval)
        HRESULT ( STDMETHODCALLTYPE *GetReportInterval )( 
            __RPC__in ILocation * This,
            /* [in] */ __RPC__in REFIID reportType,
            /* [retval][out] */ __RPC__out DWORD *pMilliseconds);
        
        DECLSPEC_XFGVIRT(ILocation, SetReportInterval)
        HRESULT ( STDMETHODCALLTYPE *SetReportInterval )( 
            __RPC__in ILocation * This,
            /* [in] */ __RPC__in REFIID reportType,
            /* [in] */ DWORD millisecondsRequested);
        
        DECLSPEC_XFGVIRT(ILocation, GetDesiredAccuracy)
        HRESULT ( STDMETHODCALLTYPE *GetDesiredAccuracy )( 
            __RPC__in ILocation * This,
            /* [in] */ __RPC__in REFIID reportType,
            /* [retval][out] */ __RPC__out enum LOCATION_DESIRED_ACCURACY *pDesiredAccuracy);
        
        DECLSPEC_XFGVIRT(ILocation, SetDesiredAccuracy)
        HRESULT ( STDMETHODCALLTYPE *SetDesiredAccuracy )( 
            __RPC__in ILocation * This,
            /* [in] */ __RPC__in REFIID reportType,
            /* [in] */ enum LOCATION_DESIRED_ACCURACY desiredAccuracy);
        
        DECLSPEC_XFGVIRT(ILocation, RequestPermissions)
        HRESULT ( STDMETHODCALLTYPE *RequestPermissions )( 
            __RPC__in ILocation * This,
            /* [unique][in] */ __RPC__in_opt HWND hParent,
            /* [size_is][in] */ __RPC__in_ecount_full(count) IID *pReportTypes,
            /* [in] */ ULONG count,
            /* [in] */ BOOL fModal);
        
        END_INTERFACE
    } ILocationVtbl;

    interface ILocation
    {
        CONST_VTBL struct ILocationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ILocation_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ILocation_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ILocation_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ILocation_RegisterForReport(This,pEvents,reportType,dwRequestedReportInterval)	\
    ( (This)->lpVtbl -> RegisterForReport(This,pEvents,reportType,dwRequestedReportInterval) ) 

#define ILocation_UnregisterForReport(This,reportType)	\
    ( (This)->lpVtbl -> UnregisterForReport(This,reportType) ) 

#define ILocation_GetReport(This,reportType,ppLocationReport)	\
    ( (This)->lpVtbl -> GetReport(This,reportType,ppLocationReport) ) 

#define ILocation_GetReportStatus(This,reportType,pStatus)	\
    ( (This)->lpVtbl -> GetReportStatus(This,reportType,pStatus) ) 

#define ILocation_GetReportInterval(This,reportType,pMilliseconds)	\
    ( (This)->lpVtbl -> GetReportInterval(This,reportType,pMilliseconds) ) 

#define ILocation_SetReportInterval(This,reportType,millisecondsRequested)	\
    ( (This)->lpVtbl -> SetReportInterval(This,reportType,millisecondsRequested) ) 

#define ILocation_GetDesiredAccuracy(This,reportType,pDesiredAccuracy)	\
    ( (This)->lpVtbl -> GetDesiredAccuracy(This,reportType,pDesiredAccuracy) ) 

#define ILocation_SetDesiredAccuracy(This,reportType,desiredAccuracy)	\
    ( (This)->lpVtbl -> SetDesiredAccuracy(This,reportType,desiredAccuracy) ) 

#define ILocation_RequestPermissions(This,hParent,pReportTypes,count,fModal)	\
    ( (This)->lpVtbl -> RequestPermissions(This,hParent,pReportTypes,count,fModal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ILocation_INTERFACE_DEFINED__ */


#ifndef __ILocationPower_INTERFACE_DEFINED__
#define __ILocationPower_INTERFACE_DEFINED__

/* interface ILocationPower */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ILocationPower;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("193E7729-AB6B-4b12-8617-7596E1BB191C")
    ILocationPower : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Connect( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Disconnect( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ILocationPowerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ILocationPower * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ILocationPower * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ILocationPower * This);
        
        DECLSPEC_XFGVIRT(ILocationPower, Connect)
        HRESULT ( STDMETHODCALLTYPE *Connect )( 
            __RPC__in ILocationPower * This);
        
        DECLSPEC_XFGVIRT(ILocationPower, Disconnect)
        HRESULT ( STDMETHODCALLTYPE *Disconnect )( 
            __RPC__in ILocationPower * This);
        
        END_INTERFACE
    } ILocationPowerVtbl;

    interface ILocationPower
    {
        CONST_VTBL struct ILocationPowerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ILocationPower_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ILocationPower_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ILocationPower_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ILocationPower_Connect(This)	\
    ( (This)->lpVtbl -> Connect(This) ) 

#define ILocationPower_Disconnect(This)	\
    ( (This)->lpVtbl -> Disconnect(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ILocationPower_INTERFACE_DEFINED__ */


#ifndef __IDefaultLocation_INTERFACE_DEFINED__
#define __IDefaultLocation_INTERFACE_DEFINED__

/* interface IDefaultLocation */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IDefaultLocation;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A65AF77E-969A-4a2e-8ACA-33BB7CBB1235")
    IDefaultLocation : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetReport( 
            /* [in] */ __RPC__in REFIID reportType,
            /* [in] */ __RPC__in_opt ILocationReport *pLocationReport) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetReport( 
            /* [in] */ __RPC__in REFIID reportType,
            /* [retval][out] */ __RPC__deref_out_opt ILocationReport **ppLocationReport) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDefaultLocationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDefaultLocation * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDefaultLocation * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDefaultLocation * This);
        
        DECLSPEC_XFGVIRT(IDefaultLocation, SetReport)
        HRESULT ( STDMETHODCALLTYPE *SetReport )( 
            __RPC__in IDefaultLocation * This,
            /* [in] */ __RPC__in REFIID reportType,
            /* [in] */ __RPC__in_opt ILocationReport *pLocationReport);
        
        DECLSPEC_XFGVIRT(IDefaultLocation, GetReport)
        HRESULT ( STDMETHODCALLTYPE *GetReport )( 
            __RPC__in IDefaultLocation * This,
            /* [in] */ __RPC__in REFIID reportType,
            /* [retval][out] */ __RPC__deref_out_opt ILocationReport **ppLocationReport);
        
        END_INTERFACE
    } IDefaultLocationVtbl;

    interface IDefaultLocation
    {
        CONST_VTBL struct IDefaultLocationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDefaultLocation_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDefaultLocation_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDefaultLocation_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDefaultLocation_SetReport(This,reportType,pLocationReport)	\
    ( (This)->lpVtbl -> SetReport(This,reportType,pLocationReport) ) 

#define IDefaultLocation_GetReport(This,reportType,ppLocationReport)	\
    ( (This)->lpVtbl -> GetReport(This,reportType,ppLocationReport) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDefaultLocation_INTERFACE_DEFINED__ */


#ifndef __ILocationEvents_INTERFACE_DEFINED__
#define __ILocationEvents_INTERFACE_DEFINED__

/* interface ILocationEvents */
/* [object][uuid] */ 


EXTERN_C const IID IID_ILocationEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("CAE02BBF-798B-4508-A207-35A7906DC73D")
    ILocationEvents : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnLocationChanged( 
            /* [in] */ __RPC__in REFIID reportType,
            /* [in] */ __RPC__in_opt ILocationReport *pLocationReport) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnStatusChanged( 
            /* [in] */ __RPC__in REFIID reportType,
            /* [in] */ LOCATION_REPORT_STATUS newStatus) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ILocationEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ILocationEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ILocationEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ILocationEvents * This);
        
        DECLSPEC_XFGVIRT(ILocationEvents, OnLocationChanged)
        HRESULT ( STDMETHODCALLTYPE *OnLocationChanged )( 
            __RPC__in ILocationEvents * This,
            /* [in] */ __RPC__in REFIID reportType,
            /* [in] */ __RPC__in_opt ILocationReport *pLocationReport);
        
        DECLSPEC_XFGVIRT(ILocationEvents, OnStatusChanged)
        HRESULT ( STDMETHODCALLTYPE *OnStatusChanged )( 
            __RPC__in ILocationEvents * This,
            /* [in] */ __RPC__in REFIID reportType,
            /* [in] */ LOCATION_REPORT_STATUS newStatus);
        
        END_INTERFACE
    } ILocationEventsVtbl;

    interface ILocationEvents
    {
        CONST_VTBL struct ILocationEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ILocationEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ILocationEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ILocationEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ILocationEvents_OnLocationChanged(This,reportType,pLocationReport)	\
    ( (This)->lpVtbl -> OnLocationChanged(This,reportType,pLocationReport) ) 

#define ILocationEvents_OnStatusChanged(This,reportType,newStatus)	\
    ( (This)->lpVtbl -> OnStatusChanged(This,reportType,newStatus) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ILocationEvents_INTERFACE_DEFINED__ */


#ifndef __IDispLatLongReport_INTERFACE_DEFINED__
#define __IDispLatLongReport_INTERFACE_DEFINED__

/* interface IDispLatLongReport */
/* [unique][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_IDispLatLongReport;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8AE32723-389B-4A11-9957-5BDD48FC9617")
    IDispLatLongReport : public IDispatch
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Latitude( 
            /* [retval][out] */ __RPC__out DOUBLE *pVal) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Longitude( 
            /* [retval][out] */ __RPC__out DOUBLE *pVal) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_ErrorRadius( 
            /* [retval][out] */ __RPC__out DOUBLE *pVal) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Altitude( 
            /* [retval][out] */ __RPC__out DOUBLE *pVal) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_AltitudeError( 
            /* [retval][out] */ __RPC__out DOUBLE *pVal) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Timestamp( 
            /* [retval][out] */ __RPC__out DATE *pVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDispLatLongReportVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDispLatLongReport * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDispLatLongReport * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDispLatLongReport * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IDispLatLongReport * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IDispLatLongReport * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IDispLatLongReport * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IDispLatLongReport * This,
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
        
        DECLSPEC_XFGVIRT(IDispLatLongReport, get_Latitude)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Latitude )( 
            __RPC__in IDispLatLongReport * This,
            /* [retval][out] */ __RPC__out DOUBLE *pVal);
        
        DECLSPEC_XFGVIRT(IDispLatLongReport, get_Longitude)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Longitude )( 
            __RPC__in IDispLatLongReport * This,
            /* [retval][out] */ __RPC__out DOUBLE *pVal);
        
        DECLSPEC_XFGVIRT(IDispLatLongReport, get_ErrorRadius)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ErrorRadius )( 
            __RPC__in IDispLatLongReport * This,
            /* [retval][out] */ __RPC__out DOUBLE *pVal);
        
        DECLSPEC_XFGVIRT(IDispLatLongReport, get_Altitude)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Altitude )( 
            __RPC__in IDispLatLongReport * This,
            /* [retval][out] */ __RPC__out DOUBLE *pVal);
        
        DECLSPEC_XFGVIRT(IDispLatLongReport, get_AltitudeError)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AltitudeError )( 
            __RPC__in IDispLatLongReport * This,
            /* [retval][out] */ __RPC__out DOUBLE *pVal);
        
        DECLSPEC_XFGVIRT(IDispLatLongReport, get_Timestamp)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Timestamp )( 
            __RPC__in IDispLatLongReport * This,
            /* [retval][out] */ __RPC__out DATE *pVal);
        
        END_INTERFACE
    } IDispLatLongReportVtbl;

    interface IDispLatLongReport
    {
        CONST_VTBL struct IDispLatLongReportVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDispLatLongReport_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDispLatLongReport_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDispLatLongReport_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDispLatLongReport_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IDispLatLongReport_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IDispLatLongReport_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IDispLatLongReport_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IDispLatLongReport_get_Latitude(This,pVal)	\
    ( (This)->lpVtbl -> get_Latitude(This,pVal) ) 

#define IDispLatLongReport_get_Longitude(This,pVal)	\
    ( (This)->lpVtbl -> get_Longitude(This,pVal) ) 

#define IDispLatLongReport_get_ErrorRadius(This,pVal)	\
    ( (This)->lpVtbl -> get_ErrorRadius(This,pVal) ) 

#define IDispLatLongReport_get_Altitude(This,pVal)	\
    ( (This)->lpVtbl -> get_Altitude(This,pVal) ) 

#define IDispLatLongReport_get_AltitudeError(This,pVal)	\
    ( (This)->lpVtbl -> get_AltitudeError(This,pVal) ) 

#define IDispLatLongReport_get_Timestamp(This,pVal)	\
    ( (This)->lpVtbl -> get_Timestamp(This,pVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDispLatLongReport_INTERFACE_DEFINED__ */


#ifndef __IDispCivicAddressReport_INTERFACE_DEFINED__
#define __IDispCivicAddressReport_INTERFACE_DEFINED__

/* interface IDispCivicAddressReport */
/* [unique][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_IDispCivicAddressReport;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("16FF1A34-9E30-42c3-B44D-E22513B5767A")
    IDispCivicAddressReport : public IDispatch
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_AddressLine1( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pAddress1) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_AddressLine2( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pAddress2) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_City( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pCity) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_StateProvince( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pStateProvince) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_PostalCode( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pPostalCode) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_CountryRegion( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pCountryRegion) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_DetailLevel( 
            /* [retval][out] */ __RPC__out ULONG *pDetailLevel) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Timestamp( 
            /* [retval][out] */ __RPC__out DATE *pVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDispCivicAddressReportVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDispCivicAddressReport * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDispCivicAddressReport * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDispCivicAddressReport * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IDispCivicAddressReport * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IDispCivicAddressReport * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IDispCivicAddressReport * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IDispCivicAddressReport * This,
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
        
        DECLSPEC_XFGVIRT(IDispCivicAddressReport, get_AddressLine1)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AddressLine1 )( 
            __RPC__in IDispCivicAddressReport * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pAddress1);
        
        DECLSPEC_XFGVIRT(IDispCivicAddressReport, get_AddressLine2)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AddressLine2 )( 
            __RPC__in IDispCivicAddressReport * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pAddress2);
        
        DECLSPEC_XFGVIRT(IDispCivicAddressReport, get_City)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_City )( 
            __RPC__in IDispCivicAddressReport * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pCity);
        
        DECLSPEC_XFGVIRT(IDispCivicAddressReport, get_StateProvince)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StateProvince )( 
            __RPC__in IDispCivicAddressReport * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pStateProvince);
        
        DECLSPEC_XFGVIRT(IDispCivicAddressReport, get_PostalCode)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PostalCode )( 
            __RPC__in IDispCivicAddressReport * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pPostalCode);
        
        DECLSPEC_XFGVIRT(IDispCivicAddressReport, get_CountryRegion)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CountryRegion )( 
            __RPC__in IDispCivicAddressReport * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pCountryRegion);
        
        DECLSPEC_XFGVIRT(IDispCivicAddressReport, get_DetailLevel)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DetailLevel )( 
            __RPC__in IDispCivicAddressReport * This,
            /* [retval][out] */ __RPC__out ULONG *pDetailLevel);
        
        DECLSPEC_XFGVIRT(IDispCivicAddressReport, get_Timestamp)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Timestamp )( 
            __RPC__in IDispCivicAddressReport * This,
            /* [retval][out] */ __RPC__out DATE *pVal);
        
        END_INTERFACE
    } IDispCivicAddressReportVtbl;

    interface IDispCivicAddressReport
    {
        CONST_VTBL struct IDispCivicAddressReportVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDispCivicAddressReport_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDispCivicAddressReport_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDispCivicAddressReport_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDispCivicAddressReport_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IDispCivicAddressReport_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IDispCivicAddressReport_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IDispCivicAddressReport_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IDispCivicAddressReport_get_AddressLine1(This,pAddress1)	\
    ( (This)->lpVtbl -> get_AddressLine1(This,pAddress1) ) 

#define IDispCivicAddressReport_get_AddressLine2(This,pAddress2)	\
    ( (This)->lpVtbl -> get_AddressLine2(This,pAddress2) ) 

#define IDispCivicAddressReport_get_City(This,pCity)	\
    ( (This)->lpVtbl -> get_City(This,pCity) ) 

#define IDispCivicAddressReport_get_StateProvince(This,pStateProvince)	\
    ( (This)->lpVtbl -> get_StateProvince(This,pStateProvince) ) 

#define IDispCivicAddressReport_get_PostalCode(This,pPostalCode)	\
    ( (This)->lpVtbl -> get_PostalCode(This,pPostalCode) ) 

#define IDispCivicAddressReport_get_CountryRegion(This,pCountryRegion)	\
    ( (This)->lpVtbl -> get_CountryRegion(This,pCountryRegion) ) 

#define IDispCivicAddressReport_get_DetailLevel(This,pDetailLevel)	\
    ( (This)->lpVtbl -> get_DetailLevel(This,pDetailLevel) ) 

#define IDispCivicAddressReport_get_Timestamp(This,pVal)	\
    ( (This)->lpVtbl -> get_Timestamp(This,pVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDispCivicAddressReport_INTERFACE_DEFINED__ */


#ifndef __ILocationReportFactory_INTERFACE_DEFINED__
#define __ILocationReportFactory_INTERFACE_DEFINED__

/* interface ILocationReportFactory */
/* [unique][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_ILocationReportFactory;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2DAEC322-90B2-47e4-BB08-0DA841935A6B")
    ILocationReportFactory : public IDispatch
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE ListenForReports( 
            /* [defaultvalue][in] */ ULONG requestedReportInterval = 0) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE StopListeningForReports( void) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Status( 
            /* [retval][out] */ __RPC__out ULONG *pVal) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_ReportInterval( 
            /* [retval][out] */ __RPC__out ULONG *pMilliseconds) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_ReportInterval( 
            /* [in] */ ULONG millisecondsRequested) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_DesiredAccuracy( 
            /* [retval][out] */ __RPC__out ULONG *pDesiredAccuracy) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_DesiredAccuracy( 
            /* [in] */ ULONG desiredAccuracy) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE RequestPermissions( 
            /* [in] */ __RPC__in ULONG *hWnd) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ILocationReportFactoryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ILocationReportFactory * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ILocationReportFactory * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ILocationReportFactory * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ILocationReportFactory * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ILocationReportFactory * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ILocationReportFactory * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ILocationReportFactory * This,
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
        
        DECLSPEC_XFGVIRT(ILocationReportFactory, ListenForReports)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ListenForReports )( 
            __RPC__in ILocationReportFactory * This,
            /* [defaultvalue][in] */ ULONG requestedReportInterval);
        
        DECLSPEC_XFGVIRT(ILocationReportFactory, StopListeningForReports)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *StopListeningForReports )( 
            __RPC__in ILocationReportFactory * This);
        
        DECLSPEC_XFGVIRT(ILocationReportFactory, get_Status)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Status )( 
            __RPC__in ILocationReportFactory * This,
            /* [retval][out] */ __RPC__out ULONG *pVal);
        
        DECLSPEC_XFGVIRT(ILocationReportFactory, get_ReportInterval)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ReportInterval )( 
            __RPC__in ILocationReportFactory * This,
            /* [retval][out] */ __RPC__out ULONG *pMilliseconds);
        
        DECLSPEC_XFGVIRT(ILocationReportFactory, put_ReportInterval)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ReportInterval )( 
            __RPC__in ILocationReportFactory * This,
            /* [in] */ ULONG millisecondsRequested);
        
        DECLSPEC_XFGVIRT(ILocationReportFactory, get_DesiredAccuracy)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DesiredAccuracy )( 
            __RPC__in ILocationReportFactory * This,
            /* [retval][out] */ __RPC__out ULONG *pDesiredAccuracy);
        
        DECLSPEC_XFGVIRT(ILocationReportFactory, put_DesiredAccuracy)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DesiredAccuracy )( 
            __RPC__in ILocationReportFactory * This,
            /* [in] */ ULONG desiredAccuracy);
        
        DECLSPEC_XFGVIRT(ILocationReportFactory, RequestPermissions)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *RequestPermissions )( 
            __RPC__in ILocationReportFactory * This,
            /* [in] */ __RPC__in ULONG *hWnd);
        
        END_INTERFACE
    } ILocationReportFactoryVtbl;

    interface ILocationReportFactory
    {
        CONST_VTBL struct ILocationReportFactoryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ILocationReportFactory_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ILocationReportFactory_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ILocationReportFactory_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ILocationReportFactory_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ILocationReportFactory_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ILocationReportFactory_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ILocationReportFactory_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ILocationReportFactory_ListenForReports(This,requestedReportInterval)	\
    ( (This)->lpVtbl -> ListenForReports(This,requestedReportInterval) ) 

#define ILocationReportFactory_StopListeningForReports(This)	\
    ( (This)->lpVtbl -> StopListeningForReports(This) ) 

#define ILocationReportFactory_get_Status(This,pVal)	\
    ( (This)->lpVtbl -> get_Status(This,pVal) ) 

#define ILocationReportFactory_get_ReportInterval(This,pMilliseconds)	\
    ( (This)->lpVtbl -> get_ReportInterval(This,pMilliseconds) ) 

#define ILocationReportFactory_put_ReportInterval(This,millisecondsRequested)	\
    ( (This)->lpVtbl -> put_ReportInterval(This,millisecondsRequested) ) 

#define ILocationReportFactory_get_DesiredAccuracy(This,pDesiredAccuracy)	\
    ( (This)->lpVtbl -> get_DesiredAccuracy(This,pDesiredAccuracy) ) 

#define ILocationReportFactory_put_DesiredAccuracy(This,desiredAccuracy)	\
    ( (This)->lpVtbl -> put_DesiredAccuracy(This,desiredAccuracy) ) 

#define ILocationReportFactory_RequestPermissions(This,hWnd)	\
    ( (This)->lpVtbl -> RequestPermissions(This,hWnd) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ILocationReportFactory_INTERFACE_DEFINED__ */


#ifndef __ILatLongReportFactory_INTERFACE_DEFINED__
#define __ILatLongReportFactory_INTERFACE_DEFINED__

/* interface ILatLongReportFactory */
/* [unique][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_ILatLongReportFactory;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3F0804CB-B114-447D-83DD-390174EBB082")
    ILatLongReportFactory : public ILocationReportFactory
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_LatLongReport( 
            /* [retval][out] */ __RPC__deref_out_opt IDispLatLongReport **pVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ILatLongReportFactoryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ILatLongReportFactory * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ILatLongReportFactory * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ILatLongReportFactory * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ILatLongReportFactory * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ILatLongReportFactory * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ILatLongReportFactory * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ILatLongReportFactory * This,
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
        
        DECLSPEC_XFGVIRT(ILocationReportFactory, ListenForReports)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ListenForReports )( 
            __RPC__in ILatLongReportFactory * This,
            /* [defaultvalue][in] */ ULONG requestedReportInterval);
        
        DECLSPEC_XFGVIRT(ILocationReportFactory, StopListeningForReports)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *StopListeningForReports )( 
            __RPC__in ILatLongReportFactory * This);
        
        DECLSPEC_XFGVIRT(ILocationReportFactory, get_Status)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Status )( 
            __RPC__in ILatLongReportFactory * This,
            /* [retval][out] */ __RPC__out ULONG *pVal);
        
        DECLSPEC_XFGVIRT(ILocationReportFactory, get_ReportInterval)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ReportInterval )( 
            __RPC__in ILatLongReportFactory * This,
            /* [retval][out] */ __RPC__out ULONG *pMilliseconds);
        
        DECLSPEC_XFGVIRT(ILocationReportFactory, put_ReportInterval)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ReportInterval )( 
            __RPC__in ILatLongReportFactory * This,
            /* [in] */ ULONG millisecondsRequested);
        
        DECLSPEC_XFGVIRT(ILocationReportFactory, get_DesiredAccuracy)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DesiredAccuracy )( 
            __RPC__in ILatLongReportFactory * This,
            /* [retval][out] */ __RPC__out ULONG *pDesiredAccuracy);
        
        DECLSPEC_XFGVIRT(ILocationReportFactory, put_DesiredAccuracy)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DesiredAccuracy )( 
            __RPC__in ILatLongReportFactory * This,
            /* [in] */ ULONG desiredAccuracy);
        
        DECLSPEC_XFGVIRT(ILocationReportFactory, RequestPermissions)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *RequestPermissions )( 
            __RPC__in ILatLongReportFactory * This,
            /* [in] */ __RPC__in ULONG *hWnd);
        
        DECLSPEC_XFGVIRT(ILatLongReportFactory, get_LatLongReport)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LatLongReport )( 
            __RPC__in ILatLongReportFactory * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispLatLongReport **pVal);
        
        END_INTERFACE
    } ILatLongReportFactoryVtbl;

    interface ILatLongReportFactory
    {
        CONST_VTBL struct ILatLongReportFactoryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ILatLongReportFactory_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ILatLongReportFactory_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ILatLongReportFactory_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ILatLongReportFactory_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ILatLongReportFactory_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ILatLongReportFactory_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ILatLongReportFactory_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ILatLongReportFactory_ListenForReports(This,requestedReportInterval)	\
    ( (This)->lpVtbl -> ListenForReports(This,requestedReportInterval) ) 

#define ILatLongReportFactory_StopListeningForReports(This)	\
    ( (This)->lpVtbl -> StopListeningForReports(This) ) 

#define ILatLongReportFactory_get_Status(This,pVal)	\
    ( (This)->lpVtbl -> get_Status(This,pVal) ) 

#define ILatLongReportFactory_get_ReportInterval(This,pMilliseconds)	\
    ( (This)->lpVtbl -> get_ReportInterval(This,pMilliseconds) ) 

#define ILatLongReportFactory_put_ReportInterval(This,millisecondsRequested)	\
    ( (This)->lpVtbl -> put_ReportInterval(This,millisecondsRequested) ) 

#define ILatLongReportFactory_get_DesiredAccuracy(This,pDesiredAccuracy)	\
    ( (This)->lpVtbl -> get_DesiredAccuracy(This,pDesiredAccuracy) ) 

#define ILatLongReportFactory_put_DesiredAccuracy(This,desiredAccuracy)	\
    ( (This)->lpVtbl -> put_DesiredAccuracy(This,desiredAccuracy) ) 

#define ILatLongReportFactory_RequestPermissions(This,hWnd)	\
    ( (This)->lpVtbl -> RequestPermissions(This,hWnd) ) 


#define ILatLongReportFactory_get_LatLongReport(This,pVal)	\
    ( (This)->lpVtbl -> get_LatLongReport(This,pVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ILatLongReportFactory_INTERFACE_DEFINED__ */


#ifndef __ICivicAddressReportFactory_INTERFACE_DEFINED__
#define __ICivicAddressReportFactory_INTERFACE_DEFINED__

/* interface ICivicAddressReportFactory */
/* [unique][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_ICivicAddressReportFactory;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("BF773B93-C64F-4bee-BEB2-67C0B8DF66E0")
    ICivicAddressReportFactory : public ILocationReportFactory
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_CivicAddressReport( 
            /* [retval][out] */ __RPC__deref_out_opt IDispCivicAddressReport **pVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICivicAddressReportFactoryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICivicAddressReportFactory * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICivicAddressReportFactory * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICivicAddressReportFactory * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ICivicAddressReportFactory * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ICivicAddressReportFactory * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ICivicAddressReportFactory * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ICivicAddressReportFactory * This,
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
        
        DECLSPEC_XFGVIRT(ILocationReportFactory, ListenForReports)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ListenForReports )( 
            __RPC__in ICivicAddressReportFactory * This,
            /* [defaultvalue][in] */ ULONG requestedReportInterval);
        
        DECLSPEC_XFGVIRT(ILocationReportFactory, StopListeningForReports)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *StopListeningForReports )( 
            __RPC__in ICivicAddressReportFactory * This);
        
        DECLSPEC_XFGVIRT(ILocationReportFactory, get_Status)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Status )( 
            __RPC__in ICivicAddressReportFactory * This,
            /* [retval][out] */ __RPC__out ULONG *pVal);
        
        DECLSPEC_XFGVIRT(ILocationReportFactory, get_ReportInterval)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ReportInterval )( 
            __RPC__in ICivicAddressReportFactory * This,
            /* [retval][out] */ __RPC__out ULONG *pMilliseconds);
        
        DECLSPEC_XFGVIRT(ILocationReportFactory, put_ReportInterval)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ReportInterval )( 
            __RPC__in ICivicAddressReportFactory * This,
            /* [in] */ ULONG millisecondsRequested);
        
        DECLSPEC_XFGVIRT(ILocationReportFactory, get_DesiredAccuracy)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DesiredAccuracy )( 
            __RPC__in ICivicAddressReportFactory * This,
            /* [retval][out] */ __RPC__out ULONG *pDesiredAccuracy);
        
        DECLSPEC_XFGVIRT(ILocationReportFactory, put_DesiredAccuracy)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DesiredAccuracy )( 
            __RPC__in ICivicAddressReportFactory * This,
            /* [in] */ ULONG desiredAccuracy);
        
        DECLSPEC_XFGVIRT(ILocationReportFactory, RequestPermissions)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *RequestPermissions )( 
            __RPC__in ICivicAddressReportFactory * This,
            /* [in] */ __RPC__in ULONG *hWnd);
        
        DECLSPEC_XFGVIRT(ICivicAddressReportFactory, get_CivicAddressReport)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CivicAddressReport )( 
            __RPC__in ICivicAddressReportFactory * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispCivicAddressReport **pVal);
        
        END_INTERFACE
    } ICivicAddressReportFactoryVtbl;

    interface ICivicAddressReportFactory
    {
        CONST_VTBL struct ICivicAddressReportFactoryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICivicAddressReportFactory_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICivicAddressReportFactory_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICivicAddressReportFactory_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICivicAddressReportFactory_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ICivicAddressReportFactory_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ICivicAddressReportFactory_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ICivicAddressReportFactory_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ICivicAddressReportFactory_ListenForReports(This,requestedReportInterval)	\
    ( (This)->lpVtbl -> ListenForReports(This,requestedReportInterval) ) 

#define ICivicAddressReportFactory_StopListeningForReports(This)	\
    ( (This)->lpVtbl -> StopListeningForReports(This) ) 

#define ICivicAddressReportFactory_get_Status(This,pVal)	\
    ( (This)->lpVtbl -> get_Status(This,pVal) ) 

#define ICivicAddressReportFactory_get_ReportInterval(This,pMilliseconds)	\
    ( (This)->lpVtbl -> get_ReportInterval(This,pMilliseconds) ) 

#define ICivicAddressReportFactory_put_ReportInterval(This,millisecondsRequested)	\
    ( (This)->lpVtbl -> put_ReportInterval(This,millisecondsRequested) ) 

#define ICivicAddressReportFactory_get_DesiredAccuracy(This,pDesiredAccuracy)	\
    ( (This)->lpVtbl -> get_DesiredAccuracy(This,pDesiredAccuracy) ) 

#define ICivicAddressReportFactory_put_DesiredAccuracy(This,desiredAccuracy)	\
    ( (This)->lpVtbl -> put_DesiredAccuracy(This,desiredAccuracy) ) 

#define ICivicAddressReportFactory_RequestPermissions(This,hWnd)	\
    ( (This)->lpVtbl -> RequestPermissions(This,hWnd) ) 


#define ICivicAddressReportFactory_get_CivicAddressReport(This,pVal)	\
    ( (This)->lpVtbl -> get_CivicAddressReport(This,pVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICivicAddressReportFactory_INTERFACE_DEFINED__ */



#ifndef __LocationApiLib_LIBRARY_DEFINED__
#define __LocationApiLib_LIBRARY_DEFINED__

/* library LocationApiLib */
/* [version][uuid] */ 


EXTERN_C const IID LIBID_LocationApiLib;

EXTERN_C const CLSID CLSID_Location;

#ifdef __cplusplus

class DECLSPEC_UUID("E5B8E079-EE6D-4E33-A438-C87F2E959254")
Location;
#endif

EXTERN_C const CLSID CLSID_DefaultLocation;

#ifdef __cplusplus

class DECLSPEC_UUID("8B7FBFE0-5CD7-494a-AF8C-283A65707506")
DefaultLocation;
#endif

EXTERN_C const CLSID CLSID_LatLongReport;

#ifdef __cplusplus

class DECLSPEC_UUID("ED81C073-1F84-4ca8-A161-183C776BC651")
LatLongReport;
#endif

EXTERN_C const CLSID CLSID_CivicAddressReport;

#ifdef __cplusplus

class DECLSPEC_UUID("D39E7BDD-7D05-46b8-8721-80CF035F57D7")
CivicAddressReport;
#endif

#ifndef ___ILatLongReportFactoryEvents_DISPINTERFACE_DEFINED__
#define ___ILatLongReportFactoryEvents_DISPINTERFACE_DEFINED__

/* dispinterface _ILatLongReportFactoryEvents */
/* [uuid] */ 


EXTERN_C const IID DIID__ILatLongReportFactoryEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)

    MIDL_INTERFACE("16EE6CB7-AB3C-424B-849F-269BE551FCBC")
    _ILatLongReportFactoryEvents : public IDispatch
    {
    };
    
#else 	/* C style interface */

    typedef struct _ILatLongReportFactoryEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in _ILatLongReportFactoryEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in _ILatLongReportFactoryEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in _ILatLongReportFactoryEvents * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in _ILatLongReportFactoryEvents * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in _ILatLongReportFactoryEvents * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in _ILatLongReportFactoryEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            _ILatLongReportFactoryEvents * This,
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
    } _ILatLongReportFactoryEventsVtbl;

    interface _ILatLongReportFactoryEvents
    {
        CONST_VTBL struct _ILatLongReportFactoryEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define _ILatLongReportFactoryEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define _ILatLongReportFactoryEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define _ILatLongReportFactoryEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define _ILatLongReportFactoryEvents_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define _ILatLongReportFactoryEvents_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define _ILatLongReportFactoryEvents_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define _ILatLongReportFactoryEvents_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */


#endif 	/* ___ILatLongReportFactoryEvents_DISPINTERFACE_DEFINED__ */


#ifndef ___ICivicAddressReportFactoryEvents_DISPINTERFACE_DEFINED__
#define ___ICivicAddressReportFactoryEvents_DISPINTERFACE_DEFINED__

/* dispinterface _ICivicAddressReportFactoryEvents */
/* [uuid] */ 


EXTERN_C const IID DIID__ICivicAddressReportFactoryEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)

    MIDL_INTERFACE("C96039FF-72EC-4617-89BD-84D88BEDC722")
    _ICivicAddressReportFactoryEvents : public IDispatch
    {
    };
    
#else 	/* C style interface */

    typedef struct _ICivicAddressReportFactoryEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in _ICivicAddressReportFactoryEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in _ICivicAddressReportFactoryEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in _ICivicAddressReportFactoryEvents * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in _ICivicAddressReportFactoryEvents * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in _ICivicAddressReportFactoryEvents * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in _ICivicAddressReportFactoryEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            _ICivicAddressReportFactoryEvents * This,
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
    } _ICivicAddressReportFactoryEventsVtbl;

    interface _ICivicAddressReportFactoryEvents
    {
        CONST_VTBL struct _ICivicAddressReportFactoryEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define _ICivicAddressReportFactoryEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define _ICivicAddressReportFactoryEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define _ICivicAddressReportFactoryEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define _ICivicAddressReportFactoryEvents_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define _ICivicAddressReportFactoryEvents_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define _ICivicAddressReportFactoryEvents_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define _ICivicAddressReportFactoryEvents_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */


#endif 	/* ___ICivicAddressReportFactoryEvents_DISPINTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_LatLongReportFactory;

#ifdef __cplusplus

class DECLSPEC_UUID("9DCC3CC8-8609-4863-BAD4-03601F4C65E8")
LatLongReportFactory;
#endif

EXTERN_C const CLSID CLSID_CivicAddressReportFactory;

#ifdef __cplusplus

class DECLSPEC_UUID("2A11F42C-3E81-4ad4-9CBE-45579D89671A")
CivicAddressReportFactory;
#endif

EXTERN_C const CLSID CLSID_DispLatLongReport;

#ifdef __cplusplus

class DECLSPEC_UUID("7A7C3277-8F84-4636-95B2-EBB5507FF77E")
DispLatLongReport;
#endif

EXTERN_C const CLSID CLSID_DispCivicAddressReport;

#ifdef __cplusplus

class DECLSPEC_UUID("4C596AEC-8544-4082-BA9F-EB0A7D8E65C6")
DispCivicAddressReport;
#endif
#endif /* __LocationApiLib_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_locationapi_0000_0013 */
/* [local] */ 

#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)
#pragma deprecated(ICivicAddressReport)      // Please do not use ICivicAddressReport, use Geolocation WinRT APIs instead.
#pragma deprecated(IDefaultLocation)         // Please do not use IDefaultLocation, use Geolocation WinRT APIs instead.
#pragma deprecated(IDispLatLongReport)       // Please do not use IDispLatLongReport, use Geolocation WinRT APIs instead.
#pragma deprecated(IDispCivicAddressReport)  // Please do not use IDispCivicAddressReport, use Geolocation WinRT APIs instead.
#pragma deprecated(ILatLongReport)           // Please do not use ILatLongReport, use Geolocation WinRT APIs instead.
#pragma deprecated(ILocation)                // Please do not use ILocation, use Geolocation WinRT APIs instead.
#pragma deprecated(ILocationEvents)          // Please do not use ILocationEvents, use Geolocation WinRT APIs instead.
#pragma deprecated(ILocationPower)           // Please do not use ILocationPower, use Geolocation WinRT APIs instead.
#pragma deprecated(ILocationReport)          // Please do not use ILocationReport, use Geolocation WinRT APIs instead.
#endif // (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_locationapi_0000_0013_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_locationapi_0000_0013_v0_0_s_ifspec;

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


