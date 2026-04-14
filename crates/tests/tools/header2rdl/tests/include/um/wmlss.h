

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


#ifndef __wmlss_h__
#define __wmlss_h__

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

#ifndef __IWindowsMediaLibrarySharingDeviceProperty_FWD_DEFINED__
#define __IWindowsMediaLibrarySharingDeviceProperty_FWD_DEFINED__
typedef interface IWindowsMediaLibrarySharingDeviceProperty IWindowsMediaLibrarySharingDeviceProperty;

#endif 	/* __IWindowsMediaLibrarySharingDeviceProperty_FWD_DEFINED__ */


#ifndef __IWindowsMediaLibrarySharingDeviceProperties_FWD_DEFINED__
#define __IWindowsMediaLibrarySharingDeviceProperties_FWD_DEFINED__
typedef interface IWindowsMediaLibrarySharingDeviceProperties IWindowsMediaLibrarySharingDeviceProperties;

#endif 	/* __IWindowsMediaLibrarySharingDeviceProperties_FWD_DEFINED__ */


#ifndef __IWindowsMediaLibrarySharingDevice_FWD_DEFINED__
#define __IWindowsMediaLibrarySharingDevice_FWD_DEFINED__
typedef interface IWindowsMediaLibrarySharingDevice IWindowsMediaLibrarySharingDevice;

#endif 	/* __IWindowsMediaLibrarySharingDevice_FWD_DEFINED__ */


#ifndef __IWindowsMediaLibrarySharingDevices_FWD_DEFINED__
#define __IWindowsMediaLibrarySharingDevices_FWD_DEFINED__
typedef interface IWindowsMediaLibrarySharingDevices IWindowsMediaLibrarySharingDevices;

#endif 	/* __IWindowsMediaLibrarySharingDevices_FWD_DEFINED__ */


#ifndef __IWindowsMediaLibrarySharingServices_FWD_DEFINED__
#define __IWindowsMediaLibrarySharingServices_FWD_DEFINED__
typedef interface IWindowsMediaLibrarySharingServices IWindowsMediaLibrarySharingServices;

#endif 	/* __IWindowsMediaLibrarySharingServices_FWD_DEFINED__ */


#ifndef __WindowsMediaLibrarySharingServices_FWD_DEFINED__
#define __WindowsMediaLibrarySharingServices_FWD_DEFINED__

#ifdef __cplusplus
typedef class WindowsMediaLibrarySharingServices WindowsMediaLibrarySharingServices;
#else
typedef struct WindowsMediaLibrarySharingServices WindowsMediaLibrarySharingServices;
#endif /* __cplusplus */

#endif 	/* __WindowsMediaLibrarySharingServices_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_wmlss_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#if (WINVER >= _WIN32_WINNT_WIN7)


extern RPC_IF_HANDLE __MIDL_itf_wmlss_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wmlss_0000_0000_v0_0_s_ifspec;


#ifndef __WMLSS_LIBRARY_DEFINED__
#define __WMLSS_LIBRARY_DEFINED__

/* library WMLSS */
/* [helpstring][version][uuid] */ 

typedef /* [public][v1_enum][helpstring][public] */ 
enum WindowsMediaLibrarySharingDeviceAuthorizationStatus
    {
        DEVICE_AUTHORIZATION_UNKNOWN	= 0,
        DEVICE_AUTHORIZATION_ALLOWED	= 1,
        DEVICE_AUTHORIZATION_DENIED	= 2
    } 	WindowsMediaLibrarySharingDeviceAuthorizationStatus;


EXTERN_C const IID LIBID_WMLSS;

#ifndef __IWindowsMediaLibrarySharingDeviceProperty_INTERFACE_DEFINED__
#define __IWindowsMediaLibrarySharingDeviceProperty_INTERFACE_DEFINED__

/* interface IWindowsMediaLibrarySharingDeviceProperty */
/* [oleautomation][local][unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWindowsMediaLibrarySharingDeviceProperty;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("81E26927-7A7D-40A7-81D4-BDDC02960E3E")
    IWindowsMediaLibrarySharingDeviceProperty : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ BSTR *name) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Value( 
            /* [retval][out] */ VARIANT *value) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWindowsMediaLibrarySharingDevicePropertyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWindowsMediaLibrarySharingDeviceProperty * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWindowsMediaLibrarySharingDeviceProperty * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWindowsMediaLibrarySharingDeviceProperty * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IWindowsMediaLibrarySharingDeviceProperty * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IWindowsMediaLibrarySharingDeviceProperty * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IWindowsMediaLibrarySharingDeviceProperty * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IWindowsMediaLibrarySharingDeviceProperty * This,
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
        
        DECLSPEC_XFGVIRT(IWindowsMediaLibrarySharingDeviceProperty, get_Name)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            IWindowsMediaLibrarySharingDeviceProperty * This,
            /* [retval][out] */ BSTR *name);
        
        DECLSPEC_XFGVIRT(IWindowsMediaLibrarySharingDeviceProperty, get_Value)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Value )( 
            IWindowsMediaLibrarySharingDeviceProperty * This,
            /* [retval][out] */ VARIANT *value);
        
        END_INTERFACE
    } IWindowsMediaLibrarySharingDevicePropertyVtbl;

    interface IWindowsMediaLibrarySharingDeviceProperty
    {
        CONST_VTBL struct IWindowsMediaLibrarySharingDevicePropertyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWindowsMediaLibrarySharingDeviceProperty_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWindowsMediaLibrarySharingDeviceProperty_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWindowsMediaLibrarySharingDeviceProperty_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWindowsMediaLibrarySharingDeviceProperty_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IWindowsMediaLibrarySharingDeviceProperty_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IWindowsMediaLibrarySharingDeviceProperty_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IWindowsMediaLibrarySharingDeviceProperty_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IWindowsMediaLibrarySharingDeviceProperty_get_Name(This,name)	\
    ( (This)->lpVtbl -> get_Name(This,name) ) 

#define IWindowsMediaLibrarySharingDeviceProperty_get_Value(This,value)	\
    ( (This)->lpVtbl -> get_Value(This,value) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWindowsMediaLibrarySharingDeviceProperty_INTERFACE_DEFINED__ */


#ifndef __IWindowsMediaLibrarySharingDeviceProperties_INTERFACE_DEFINED__
#define __IWindowsMediaLibrarySharingDeviceProperties_INTERFACE_DEFINED__

/* interface IWindowsMediaLibrarySharingDeviceProperties */
/* [oleautomation][local][unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWindowsMediaLibrarySharingDeviceProperties;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C4623214-6B06-40C5-A623-B2FF4C076BFD")
    IWindowsMediaLibrarySharingDeviceProperties : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ LONG index,
            /* [retval][out] */ IWindowsMediaLibrarySharingDeviceProperty **property) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ LONG *count) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetProperty( 
            /* [in] */ BSTR name,
            /* [retval][out] */ IWindowsMediaLibrarySharingDeviceProperty **property) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWindowsMediaLibrarySharingDevicePropertiesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWindowsMediaLibrarySharingDeviceProperties * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWindowsMediaLibrarySharingDeviceProperties * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWindowsMediaLibrarySharingDeviceProperties * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IWindowsMediaLibrarySharingDeviceProperties * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IWindowsMediaLibrarySharingDeviceProperties * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IWindowsMediaLibrarySharingDeviceProperties * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IWindowsMediaLibrarySharingDeviceProperties * This,
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
        
        DECLSPEC_XFGVIRT(IWindowsMediaLibrarySharingDeviceProperties, get_Item)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            IWindowsMediaLibrarySharingDeviceProperties * This,
            /* [in] */ LONG index,
            /* [retval][out] */ IWindowsMediaLibrarySharingDeviceProperty **property);
        
        DECLSPEC_XFGVIRT(IWindowsMediaLibrarySharingDeviceProperties, get_Count)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            IWindowsMediaLibrarySharingDeviceProperties * This,
            /* [retval][out] */ LONG *count);
        
        DECLSPEC_XFGVIRT(IWindowsMediaLibrarySharingDeviceProperties, GetProperty)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetProperty )( 
            IWindowsMediaLibrarySharingDeviceProperties * This,
            /* [in] */ BSTR name,
            /* [retval][out] */ IWindowsMediaLibrarySharingDeviceProperty **property);
        
        END_INTERFACE
    } IWindowsMediaLibrarySharingDevicePropertiesVtbl;

    interface IWindowsMediaLibrarySharingDeviceProperties
    {
        CONST_VTBL struct IWindowsMediaLibrarySharingDevicePropertiesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWindowsMediaLibrarySharingDeviceProperties_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWindowsMediaLibrarySharingDeviceProperties_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWindowsMediaLibrarySharingDeviceProperties_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWindowsMediaLibrarySharingDeviceProperties_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IWindowsMediaLibrarySharingDeviceProperties_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IWindowsMediaLibrarySharingDeviceProperties_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IWindowsMediaLibrarySharingDeviceProperties_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IWindowsMediaLibrarySharingDeviceProperties_get_Item(This,index,property)	\
    ( (This)->lpVtbl -> get_Item(This,index,property) ) 

#define IWindowsMediaLibrarySharingDeviceProperties_get_Count(This,count)	\
    ( (This)->lpVtbl -> get_Count(This,count) ) 

#define IWindowsMediaLibrarySharingDeviceProperties_GetProperty(This,name,property)	\
    ( (This)->lpVtbl -> GetProperty(This,name,property) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWindowsMediaLibrarySharingDeviceProperties_INTERFACE_DEFINED__ */


#ifndef __IWindowsMediaLibrarySharingDevice_INTERFACE_DEFINED__
#define __IWindowsMediaLibrarySharingDevice_INTERFACE_DEFINED__

/* interface IWindowsMediaLibrarySharingDevice */
/* [oleautomation][local][unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWindowsMediaLibrarySharingDevice;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3DCCC293-4FD9-4191-A25B-8E57C5D27BD4")
    IWindowsMediaLibrarySharingDevice : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DeviceID( 
            /* [retval][out] */ BSTR *deviceID) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Authorization( 
            /* [retval][out] */ WindowsMediaLibrarySharingDeviceAuthorizationStatus *authorization) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Authorization( 
            /* [in] */ WindowsMediaLibrarySharingDeviceAuthorizationStatus authorization) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Properties( 
            /* [retval][out] */ IWindowsMediaLibrarySharingDeviceProperties **deviceProperties) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWindowsMediaLibrarySharingDeviceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWindowsMediaLibrarySharingDevice * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWindowsMediaLibrarySharingDevice * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWindowsMediaLibrarySharingDevice * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IWindowsMediaLibrarySharingDevice * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IWindowsMediaLibrarySharingDevice * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IWindowsMediaLibrarySharingDevice * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IWindowsMediaLibrarySharingDevice * This,
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
        
        DECLSPEC_XFGVIRT(IWindowsMediaLibrarySharingDevice, get_DeviceID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DeviceID )( 
            IWindowsMediaLibrarySharingDevice * This,
            /* [retval][out] */ BSTR *deviceID);
        
        DECLSPEC_XFGVIRT(IWindowsMediaLibrarySharingDevice, get_Authorization)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Authorization )( 
            IWindowsMediaLibrarySharingDevice * This,
            /* [retval][out] */ WindowsMediaLibrarySharingDeviceAuthorizationStatus *authorization);
        
        DECLSPEC_XFGVIRT(IWindowsMediaLibrarySharingDevice, put_Authorization)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Authorization )( 
            IWindowsMediaLibrarySharingDevice * This,
            /* [in] */ WindowsMediaLibrarySharingDeviceAuthorizationStatus authorization);
        
        DECLSPEC_XFGVIRT(IWindowsMediaLibrarySharingDevice, get_Properties)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Properties )( 
            IWindowsMediaLibrarySharingDevice * This,
            /* [retval][out] */ IWindowsMediaLibrarySharingDeviceProperties **deviceProperties);
        
        END_INTERFACE
    } IWindowsMediaLibrarySharingDeviceVtbl;

    interface IWindowsMediaLibrarySharingDevice
    {
        CONST_VTBL struct IWindowsMediaLibrarySharingDeviceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWindowsMediaLibrarySharingDevice_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWindowsMediaLibrarySharingDevice_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWindowsMediaLibrarySharingDevice_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWindowsMediaLibrarySharingDevice_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IWindowsMediaLibrarySharingDevice_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IWindowsMediaLibrarySharingDevice_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IWindowsMediaLibrarySharingDevice_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IWindowsMediaLibrarySharingDevice_get_DeviceID(This,deviceID)	\
    ( (This)->lpVtbl -> get_DeviceID(This,deviceID) ) 

#define IWindowsMediaLibrarySharingDevice_get_Authorization(This,authorization)	\
    ( (This)->lpVtbl -> get_Authorization(This,authorization) ) 

#define IWindowsMediaLibrarySharingDevice_put_Authorization(This,authorization)	\
    ( (This)->lpVtbl -> put_Authorization(This,authorization) ) 

#define IWindowsMediaLibrarySharingDevice_get_Properties(This,deviceProperties)	\
    ( (This)->lpVtbl -> get_Properties(This,deviceProperties) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWindowsMediaLibrarySharingDevice_INTERFACE_DEFINED__ */


#ifndef __IWindowsMediaLibrarySharingDevices_INTERFACE_DEFINED__
#define __IWindowsMediaLibrarySharingDevices_INTERFACE_DEFINED__

/* interface IWindowsMediaLibrarySharingDevices */
/* [oleautomation][local][unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWindowsMediaLibrarySharingDevices;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1803F9D6-FE6D-4546-BF5B-992FE8EC12D1")
    IWindowsMediaLibrarySharingDevices : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ LONG index,
            /* [retval][out] */ IWindowsMediaLibrarySharingDevice **device) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ LONG *count) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetDevice( 
            /* [in] */ BSTR deviceID,
            /* [retval][out] */ IWindowsMediaLibrarySharingDevice **device) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWindowsMediaLibrarySharingDevicesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWindowsMediaLibrarySharingDevices * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWindowsMediaLibrarySharingDevices * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWindowsMediaLibrarySharingDevices * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IWindowsMediaLibrarySharingDevices * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IWindowsMediaLibrarySharingDevices * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IWindowsMediaLibrarySharingDevices * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IWindowsMediaLibrarySharingDevices * This,
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
        
        DECLSPEC_XFGVIRT(IWindowsMediaLibrarySharingDevices, get_Item)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            IWindowsMediaLibrarySharingDevices * This,
            /* [in] */ LONG index,
            /* [retval][out] */ IWindowsMediaLibrarySharingDevice **device);
        
        DECLSPEC_XFGVIRT(IWindowsMediaLibrarySharingDevices, get_Count)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            IWindowsMediaLibrarySharingDevices * This,
            /* [retval][out] */ LONG *count);
        
        DECLSPEC_XFGVIRT(IWindowsMediaLibrarySharingDevices, GetDevice)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetDevice )( 
            IWindowsMediaLibrarySharingDevices * This,
            /* [in] */ BSTR deviceID,
            /* [retval][out] */ IWindowsMediaLibrarySharingDevice **device);
        
        END_INTERFACE
    } IWindowsMediaLibrarySharingDevicesVtbl;

    interface IWindowsMediaLibrarySharingDevices
    {
        CONST_VTBL struct IWindowsMediaLibrarySharingDevicesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWindowsMediaLibrarySharingDevices_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWindowsMediaLibrarySharingDevices_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWindowsMediaLibrarySharingDevices_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWindowsMediaLibrarySharingDevices_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IWindowsMediaLibrarySharingDevices_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IWindowsMediaLibrarySharingDevices_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IWindowsMediaLibrarySharingDevices_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IWindowsMediaLibrarySharingDevices_get_Item(This,index,device)	\
    ( (This)->lpVtbl -> get_Item(This,index,device) ) 

#define IWindowsMediaLibrarySharingDevices_get_Count(This,count)	\
    ( (This)->lpVtbl -> get_Count(This,count) ) 

#define IWindowsMediaLibrarySharingDevices_GetDevice(This,deviceID,device)	\
    ( (This)->lpVtbl -> GetDevice(This,deviceID,device) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWindowsMediaLibrarySharingDevices_INTERFACE_DEFINED__ */


#ifndef __IWindowsMediaLibrarySharingServices_INTERFACE_DEFINED__
#define __IWindowsMediaLibrarySharingServices_INTERFACE_DEFINED__

/* interface IWindowsMediaLibrarySharingServices */
/* [oleautomation][local][unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWindowsMediaLibrarySharingServices;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("01F5F85E-0A81-40DA-A7C8-21EF3AF8440C")
    IWindowsMediaLibrarySharingServices : public IDispatch
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE showShareMediaCPL( 
            /* [in] */ BSTR device) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_userHomeMediaSharingState( 
            /* [retval][out] */ VARIANT_BOOL *sharingEnabled) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_userHomeMediaSharingState( 
            /* [in] */ VARIANT_BOOL sharingEnabled) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_userHomeMediaSharingLibraryName( 
            /* [retval][out] */ BSTR *libraryName) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_userHomeMediaSharingLibraryName( 
            /* [in] */ BSTR libraryName) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_computerHomeMediaSharingAllowedState( 
            /* [retval][out] */ VARIANT_BOOL *sharingAllowed) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_computerHomeMediaSharingAllowedState( 
            /* [in] */ VARIANT_BOOL sharingAllowed) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_userInternetMediaSharingState( 
            /* [retval][out] */ VARIANT_BOOL *sharingEnabled) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_userInternetMediaSharingState( 
            /* [in] */ VARIANT_BOOL sharingEnabled) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_computerInternetMediaSharingAllowedState( 
            /* [retval][out] */ VARIANT_BOOL *sharingAllowed) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_computerInternetMediaSharingAllowedState( 
            /* [in] */ VARIANT_BOOL sharingAllowed) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_internetMediaSharingSecurityGroup( 
            /* [retval][out] */ BSTR *securityGroup) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_internetMediaSharingSecurityGroup( 
            /* [in] */ BSTR securityGroup) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_allowSharingToAllDevices( 
            /* [retval][out] */ VARIANT_BOOL *sharingEnabled) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_allowSharingToAllDevices( 
            /* [in] */ VARIANT_BOOL sharingEnabled) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE setDefaultAuthorization( 
            /* [in] */ BSTR MACAddresses,
            /* [in] */ BSTR friendlyName,
            /* [in] */ VARIANT_BOOL authorization) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE setAuthorizationState( 
            /* [in] */ BSTR MACAddress,
            /* [in] */ VARIANT_BOOL authorizationState) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE getAllDevices( 
            /* [retval][out] */ IWindowsMediaLibrarySharingDevices **devices) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_customSettingsApplied( 
            /* [retval][out] */ VARIANT_BOOL *customSettingsApplied) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWindowsMediaLibrarySharingServicesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWindowsMediaLibrarySharingServices * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWindowsMediaLibrarySharingServices * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWindowsMediaLibrarySharingServices * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IWindowsMediaLibrarySharingServices * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IWindowsMediaLibrarySharingServices * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IWindowsMediaLibrarySharingServices * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IWindowsMediaLibrarySharingServices * This,
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
        
        DECLSPEC_XFGVIRT(IWindowsMediaLibrarySharingServices, showShareMediaCPL)
        HRESULT ( STDMETHODCALLTYPE *showShareMediaCPL )( 
            IWindowsMediaLibrarySharingServices * This,
            /* [in] */ BSTR device);
        
        DECLSPEC_XFGVIRT(IWindowsMediaLibrarySharingServices, get_userHomeMediaSharingState)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_userHomeMediaSharingState )( 
            IWindowsMediaLibrarySharingServices * This,
            /* [retval][out] */ VARIANT_BOOL *sharingEnabled);
        
        DECLSPEC_XFGVIRT(IWindowsMediaLibrarySharingServices, put_userHomeMediaSharingState)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_userHomeMediaSharingState )( 
            IWindowsMediaLibrarySharingServices * This,
            /* [in] */ VARIANT_BOOL sharingEnabled);
        
        DECLSPEC_XFGVIRT(IWindowsMediaLibrarySharingServices, get_userHomeMediaSharingLibraryName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_userHomeMediaSharingLibraryName )( 
            IWindowsMediaLibrarySharingServices * This,
            /* [retval][out] */ BSTR *libraryName);
        
        DECLSPEC_XFGVIRT(IWindowsMediaLibrarySharingServices, put_userHomeMediaSharingLibraryName)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_userHomeMediaSharingLibraryName )( 
            IWindowsMediaLibrarySharingServices * This,
            /* [in] */ BSTR libraryName);
        
        DECLSPEC_XFGVIRT(IWindowsMediaLibrarySharingServices, get_computerHomeMediaSharingAllowedState)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_computerHomeMediaSharingAllowedState )( 
            IWindowsMediaLibrarySharingServices * This,
            /* [retval][out] */ VARIANT_BOOL *sharingAllowed);
        
        DECLSPEC_XFGVIRT(IWindowsMediaLibrarySharingServices, put_computerHomeMediaSharingAllowedState)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_computerHomeMediaSharingAllowedState )( 
            IWindowsMediaLibrarySharingServices * This,
            /* [in] */ VARIANT_BOOL sharingAllowed);
        
        DECLSPEC_XFGVIRT(IWindowsMediaLibrarySharingServices, get_userInternetMediaSharingState)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_userInternetMediaSharingState )( 
            IWindowsMediaLibrarySharingServices * This,
            /* [retval][out] */ VARIANT_BOOL *sharingEnabled);
        
        DECLSPEC_XFGVIRT(IWindowsMediaLibrarySharingServices, put_userInternetMediaSharingState)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_userInternetMediaSharingState )( 
            IWindowsMediaLibrarySharingServices * This,
            /* [in] */ VARIANT_BOOL sharingEnabled);
        
        DECLSPEC_XFGVIRT(IWindowsMediaLibrarySharingServices, get_computerInternetMediaSharingAllowedState)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_computerInternetMediaSharingAllowedState )( 
            IWindowsMediaLibrarySharingServices * This,
            /* [retval][out] */ VARIANT_BOOL *sharingAllowed);
        
        DECLSPEC_XFGVIRT(IWindowsMediaLibrarySharingServices, put_computerInternetMediaSharingAllowedState)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_computerInternetMediaSharingAllowedState )( 
            IWindowsMediaLibrarySharingServices * This,
            /* [in] */ VARIANT_BOOL sharingAllowed);
        
        DECLSPEC_XFGVIRT(IWindowsMediaLibrarySharingServices, get_internetMediaSharingSecurityGroup)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_internetMediaSharingSecurityGroup )( 
            IWindowsMediaLibrarySharingServices * This,
            /* [retval][out] */ BSTR *securityGroup);
        
        DECLSPEC_XFGVIRT(IWindowsMediaLibrarySharingServices, put_internetMediaSharingSecurityGroup)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_internetMediaSharingSecurityGroup )( 
            IWindowsMediaLibrarySharingServices * This,
            /* [in] */ BSTR securityGroup);
        
        DECLSPEC_XFGVIRT(IWindowsMediaLibrarySharingServices, get_allowSharingToAllDevices)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_allowSharingToAllDevices )( 
            IWindowsMediaLibrarySharingServices * This,
            /* [retval][out] */ VARIANT_BOOL *sharingEnabled);
        
        DECLSPEC_XFGVIRT(IWindowsMediaLibrarySharingServices, put_allowSharingToAllDevices)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_allowSharingToAllDevices )( 
            IWindowsMediaLibrarySharingServices * This,
            /* [in] */ VARIANT_BOOL sharingEnabled);
        
        DECLSPEC_XFGVIRT(IWindowsMediaLibrarySharingServices, setDefaultAuthorization)
        HRESULT ( STDMETHODCALLTYPE *setDefaultAuthorization )( 
            IWindowsMediaLibrarySharingServices * This,
            /* [in] */ BSTR MACAddresses,
            /* [in] */ BSTR friendlyName,
            /* [in] */ VARIANT_BOOL authorization);
        
        DECLSPEC_XFGVIRT(IWindowsMediaLibrarySharingServices, setAuthorizationState)
        HRESULT ( STDMETHODCALLTYPE *setAuthorizationState )( 
            IWindowsMediaLibrarySharingServices * This,
            /* [in] */ BSTR MACAddress,
            /* [in] */ VARIANT_BOOL authorizationState);
        
        DECLSPEC_XFGVIRT(IWindowsMediaLibrarySharingServices, getAllDevices)
        HRESULT ( STDMETHODCALLTYPE *getAllDevices )( 
            IWindowsMediaLibrarySharingServices * This,
            /* [retval][out] */ IWindowsMediaLibrarySharingDevices **devices);
        
        DECLSPEC_XFGVIRT(IWindowsMediaLibrarySharingServices, get_customSettingsApplied)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_customSettingsApplied )( 
            IWindowsMediaLibrarySharingServices * This,
            /* [retval][out] */ VARIANT_BOOL *customSettingsApplied);
        
        END_INTERFACE
    } IWindowsMediaLibrarySharingServicesVtbl;

    interface IWindowsMediaLibrarySharingServices
    {
        CONST_VTBL struct IWindowsMediaLibrarySharingServicesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWindowsMediaLibrarySharingServices_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWindowsMediaLibrarySharingServices_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWindowsMediaLibrarySharingServices_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWindowsMediaLibrarySharingServices_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IWindowsMediaLibrarySharingServices_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IWindowsMediaLibrarySharingServices_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IWindowsMediaLibrarySharingServices_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IWindowsMediaLibrarySharingServices_showShareMediaCPL(This,device)	\
    ( (This)->lpVtbl -> showShareMediaCPL(This,device) ) 

#define IWindowsMediaLibrarySharingServices_get_userHomeMediaSharingState(This,sharingEnabled)	\
    ( (This)->lpVtbl -> get_userHomeMediaSharingState(This,sharingEnabled) ) 

#define IWindowsMediaLibrarySharingServices_put_userHomeMediaSharingState(This,sharingEnabled)	\
    ( (This)->lpVtbl -> put_userHomeMediaSharingState(This,sharingEnabled) ) 

#define IWindowsMediaLibrarySharingServices_get_userHomeMediaSharingLibraryName(This,libraryName)	\
    ( (This)->lpVtbl -> get_userHomeMediaSharingLibraryName(This,libraryName) ) 

#define IWindowsMediaLibrarySharingServices_put_userHomeMediaSharingLibraryName(This,libraryName)	\
    ( (This)->lpVtbl -> put_userHomeMediaSharingLibraryName(This,libraryName) ) 

#define IWindowsMediaLibrarySharingServices_get_computerHomeMediaSharingAllowedState(This,sharingAllowed)	\
    ( (This)->lpVtbl -> get_computerHomeMediaSharingAllowedState(This,sharingAllowed) ) 

#define IWindowsMediaLibrarySharingServices_put_computerHomeMediaSharingAllowedState(This,sharingAllowed)	\
    ( (This)->lpVtbl -> put_computerHomeMediaSharingAllowedState(This,sharingAllowed) ) 

#define IWindowsMediaLibrarySharingServices_get_userInternetMediaSharingState(This,sharingEnabled)	\
    ( (This)->lpVtbl -> get_userInternetMediaSharingState(This,sharingEnabled) ) 

#define IWindowsMediaLibrarySharingServices_put_userInternetMediaSharingState(This,sharingEnabled)	\
    ( (This)->lpVtbl -> put_userInternetMediaSharingState(This,sharingEnabled) ) 

#define IWindowsMediaLibrarySharingServices_get_computerInternetMediaSharingAllowedState(This,sharingAllowed)	\
    ( (This)->lpVtbl -> get_computerInternetMediaSharingAllowedState(This,sharingAllowed) ) 

#define IWindowsMediaLibrarySharingServices_put_computerInternetMediaSharingAllowedState(This,sharingAllowed)	\
    ( (This)->lpVtbl -> put_computerInternetMediaSharingAllowedState(This,sharingAllowed) ) 

#define IWindowsMediaLibrarySharingServices_get_internetMediaSharingSecurityGroup(This,securityGroup)	\
    ( (This)->lpVtbl -> get_internetMediaSharingSecurityGroup(This,securityGroup) ) 

#define IWindowsMediaLibrarySharingServices_put_internetMediaSharingSecurityGroup(This,securityGroup)	\
    ( (This)->lpVtbl -> put_internetMediaSharingSecurityGroup(This,securityGroup) ) 

#define IWindowsMediaLibrarySharingServices_get_allowSharingToAllDevices(This,sharingEnabled)	\
    ( (This)->lpVtbl -> get_allowSharingToAllDevices(This,sharingEnabled) ) 

#define IWindowsMediaLibrarySharingServices_put_allowSharingToAllDevices(This,sharingEnabled)	\
    ( (This)->lpVtbl -> put_allowSharingToAllDevices(This,sharingEnabled) ) 

#define IWindowsMediaLibrarySharingServices_setDefaultAuthorization(This,MACAddresses,friendlyName,authorization)	\
    ( (This)->lpVtbl -> setDefaultAuthorization(This,MACAddresses,friendlyName,authorization) ) 

#define IWindowsMediaLibrarySharingServices_setAuthorizationState(This,MACAddress,authorizationState)	\
    ( (This)->lpVtbl -> setAuthorizationState(This,MACAddress,authorizationState) ) 

#define IWindowsMediaLibrarySharingServices_getAllDevices(This,devices)	\
    ( (This)->lpVtbl -> getAllDevices(This,devices) ) 

#define IWindowsMediaLibrarySharingServices_get_customSettingsApplied(This,customSettingsApplied)	\
    ( (This)->lpVtbl -> get_customSettingsApplied(This,customSettingsApplied) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWindowsMediaLibrarySharingServices_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_WindowsMediaLibrarySharingServices;

#ifdef __cplusplus

class DECLSPEC_UUID("AD581B00-7B64-4E59-A38D-D2C5BF51DDB3")
WindowsMediaLibrarySharingServices;
#endif
#endif /* __WMLSS_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_wmlss_0000_0001 */
/* [local] */ 

#endif
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_wmlss_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wmlss_0000_0001_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


