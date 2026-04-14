

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

#ifndef __wcmconfig_h__
#define __wcmconfig_h__

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

#ifndef __IItemEnumerator_FWD_DEFINED__
#define __IItemEnumerator_FWD_DEFINED__
typedef interface IItemEnumerator IItemEnumerator;

#endif 	/* __IItemEnumerator_FWD_DEFINED__ */


#ifndef __ISettingsIdentity_FWD_DEFINED__
#define __ISettingsIdentity_FWD_DEFINED__
typedef interface ISettingsIdentity ISettingsIdentity;

#endif 	/* __ISettingsIdentity_FWD_DEFINED__ */


#ifndef __ITargetInfo_FWD_DEFINED__
#define __ITargetInfo_FWD_DEFINED__
typedef interface ITargetInfo ITargetInfo;

#endif 	/* __ITargetInfo_FWD_DEFINED__ */


#ifndef __ISettingsEngine_FWD_DEFINED__
#define __ISettingsEngine_FWD_DEFINED__
typedef interface ISettingsEngine ISettingsEngine;

#endif 	/* __ISettingsEngine_FWD_DEFINED__ */


#ifndef __ISettingsItem_FWD_DEFINED__
#define __ISettingsItem_FWD_DEFINED__
typedef interface ISettingsItem ISettingsItem;

#endif 	/* __ISettingsItem_FWD_DEFINED__ */


#ifndef __ISettingsNamespace_FWD_DEFINED__
#define __ISettingsNamespace_FWD_DEFINED__
typedef interface ISettingsNamespace ISettingsNamespace;

#endif 	/* __ISettingsNamespace_FWD_DEFINED__ */


#ifndef __ISettingsResult_FWD_DEFINED__
#define __ISettingsResult_FWD_DEFINED__
typedef interface ISettingsResult ISettingsResult;

#endif 	/* __ISettingsResult_FWD_DEFINED__ */


#ifndef __ISettingsContext_FWD_DEFINED__
#define __ISettingsContext_FWD_DEFINED__
typedef interface ISettingsContext ISettingsContext;

#endif 	/* __ISettingsContext_FWD_DEFINED__ */


#ifndef __SettingsEngine_FWD_DEFINED__
#define __SettingsEngine_FWD_DEFINED__

#ifdef __cplusplus
typedef class SettingsEngine SettingsEngine;
#else
typedef struct SettingsEngine SettingsEngine;
#endif /* __cplusplus */

#endif 	/* __SettingsEngine_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_wcmconfig_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)








typedef /* [public][public][public] */ 
enum __MIDL___MIDL_itf_wcmconfig_0000_0000_0001
    {
        OfflineMode	= 1,
        OnlineMode	= 2
    } 	WcmTargetMode;

typedef /* [public][public] */ 
enum __MIDL___MIDL_itf_wcmconfig_0000_0000_0002
    {
        SharedEnumeration	= 1,
        UserEnumeration	= 2,
        AllEnumeration	= ( SharedEnumeration | UserEnumeration ) 
    } 	WcmNamespaceEnumerationFlags;

typedef /* [public][public][public] */ 
enum __MIDL___MIDL_itf_wcmconfig_0000_0000_0003
    {
        dataTypeByte	= 1,
        dataTypeSByte	= 2,
        dataTypeUInt16	= 3,
        dataTypeInt16	= 4,
        dataTypeUInt32	= 5,
        dataTypeInt32	= 6,
        dataTypeUInt64	= 7,
        dataTypeInt64	= 8,
        dataTypeBoolean	= 11,
        dataTypeString	= 12,
        dataTypeFlagArray	= 0x8000
    } 	WcmDataType;

typedef /* [public][public] */ 
enum __MIDL___MIDL_itf_wcmconfig_0000_0000_0004
    {
        settingTypeScalar	= 1,
        settingTypeComplex	= 2,
        settingTypeList	= 3
    } 	WcmSettingType;

typedef /* [public][public][public] */ 
enum __MIDL___MIDL_itf_wcmconfig_0000_0000_0005
    {
        restrictionFacetMaxLength	= 0x1,
        restrictionFacetEnumeration	= 0x2,
        restrictionFacetMaxInclusive	= 0x4,
        restrictionFacetMinInclusive	= 0x8
    } 	WcmRestrictionFacets;

typedef /* [public][public] */ 
enum __MIDL___MIDL_itf_wcmconfig_0000_0000_0006
    {
        UnknownStatus	= 0,
        UserRegistered	= 1,
        UserUnregistered	= 2,
        UserLoaded	= 3,
        UserUnloaded	= 4
    } 	WcmUserStatus;

typedef /* [public][public] */ 
enum __MIDL___MIDL_itf_wcmconfig_0000_0000_0007
    {
        ReadOnlyAccess	= 1,
        ReadWriteAccess	= 2
    } 	WcmNamespaceAccess;



extern RPC_IF_HANDLE __MIDL_itf_wcmconfig_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wcmconfig_0000_0000_v0_0_s_ifspec;

#ifndef __IItemEnumerator_INTERFACE_DEFINED__
#define __IItemEnumerator_INTERFACE_DEFINED__

/* interface IItemEnumerator */
/* [helpstring][uuid][unique][nonextensible][object][local] */ 


EXTERN_C const IID IID_IItemEnumerator;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9F7D7BB7-20B3-11DA-81A5-0030F1642E3C")
    IItemEnumerator : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Current( 
            /* [retval][out] */ VARIANT *Item) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE MoveNext( 
            /* [retval][out] */ BOOL *ItemValid) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IItemEnumeratorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IItemEnumerator * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IItemEnumerator * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IItemEnumerator * This);
        
        DECLSPEC_XFGVIRT(IItemEnumerator, Current)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Current )( 
            IItemEnumerator * This,
            /* [retval][out] */ VARIANT *Item);
        
        DECLSPEC_XFGVIRT(IItemEnumerator, MoveNext)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *MoveNext )( 
            IItemEnumerator * This,
            /* [retval][out] */ BOOL *ItemValid);
        
        DECLSPEC_XFGVIRT(IItemEnumerator, Reset)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Reset )( 
            IItemEnumerator * This);
        
        END_INTERFACE
    } IItemEnumeratorVtbl;

    interface IItemEnumerator
    {
        CONST_VTBL struct IItemEnumeratorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IItemEnumerator_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IItemEnumerator_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IItemEnumerator_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IItemEnumerator_Current(This,Item)	\
    ( (This)->lpVtbl -> Current(This,Item) ) 

#define IItemEnumerator_MoveNext(This,ItemValid)	\
    ( (This)->lpVtbl -> MoveNext(This,ItemValid) ) 

#define IItemEnumerator_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IItemEnumerator_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_wcmconfig_0000_0001 */
/* [local] */ 

#define WCM_SETTINGS_ID_NAME                L"name"
#define WCM_SETTINGS_ID_VERSION             L"version"
#define WCM_SETTINGS_ID_LANGUAGE            L"language"
#define WCM_SETTINGS_ID_ARCHITECTURE        L"architecture"
#define WCM_SETTINGS_ID_TOKEN               L"token"
#define WCM_SETTINGS_ID_URI                 L"uri"
#define WCM_SETTINGS_ID_VERSION_SCOPE       L"versionScope"
#define WCM_SETTINGS_ID_FLAG_REFERENCE     0x00000000
#define WCM_SETTINGS_ID_FLAG_DEFINITION    0x00000001


extern RPC_IF_HANDLE __MIDL_itf_wcmconfig_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wcmconfig_0000_0001_v0_0_s_ifspec;

#ifndef __ISettingsIdentity_INTERFACE_DEFINED__
#define __ISettingsIdentity_INTERFACE_DEFINED__

/* interface ISettingsIdentity */
/* [helpstring][uuid][unique][nonextensible][object][local] */ 


EXTERN_C const IID IID_ISettingsIdentity;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9F7D7BB6-20B3-11DA-81A5-0030F1642E3C")
    ISettingsIdentity : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetAttribute( 
            /* [in] */ void *Reserved,
            /* [string][in] */ const WCHAR *Name,
            /* [retval][out] */ BSTR *Value) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetAttribute( 
            /* [in] */ void *Reserved,
            /* [string][in] */ const WCHAR *Name,
            /* [string][in] */ const WCHAR *Value) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetFlags( 
            /* [retval][out] */ DWORD *Flags) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetFlags( 
            /* [in] */ DWORD Flags) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISettingsIdentityVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISettingsIdentity * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISettingsIdentity * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISettingsIdentity * This);
        
        DECLSPEC_XFGVIRT(ISettingsIdentity, GetAttribute)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetAttribute )( 
            ISettingsIdentity * This,
            /* [in] */ void *Reserved,
            /* [string][in] */ const WCHAR *Name,
            /* [retval][out] */ BSTR *Value);
        
        DECLSPEC_XFGVIRT(ISettingsIdentity, SetAttribute)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetAttribute )( 
            ISettingsIdentity * This,
            /* [in] */ void *Reserved,
            /* [string][in] */ const WCHAR *Name,
            /* [string][in] */ const WCHAR *Value);
        
        DECLSPEC_XFGVIRT(ISettingsIdentity, GetFlags)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetFlags )( 
            ISettingsIdentity * This,
            /* [retval][out] */ DWORD *Flags);
        
        DECLSPEC_XFGVIRT(ISettingsIdentity, SetFlags)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetFlags )( 
            ISettingsIdentity * This,
            /* [in] */ DWORD Flags);
        
        END_INTERFACE
    } ISettingsIdentityVtbl;

    interface ISettingsIdentity
    {
        CONST_VTBL struct ISettingsIdentityVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISettingsIdentity_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISettingsIdentity_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISettingsIdentity_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISettingsIdentity_GetAttribute(This,Reserved,Name,Value)	\
    ( (This)->lpVtbl -> GetAttribute(This,Reserved,Name,Value) ) 

#define ISettingsIdentity_SetAttribute(This,Reserved,Name,Value)	\
    ( (This)->lpVtbl -> SetAttribute(This,Reserved,Name,Value) ) 

#define ISettingsIdentity_GetFlags(This,Flags)	\
    ( (This)->lpVtbl -> GetFlags(This,Flags) ) 

#define ISettingsIdentity_SetFlags(This,Flags)	\
    ( (This)->lpVtbl -> SetFlags(This,Flags) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISettingsIdentity_INTERFACE_DEFINED__ */


#ifndef __ITargetInfo_INTERFACE_DEFINED__
#define __ITargetInfo_INTERFACE_DEFINED__

/* interface ITargetInfo */
/* [helpstring][uuid][unique][nonextensible][object][local] */ 


EXTERN_C const IID IID_ITargetInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9F7D7BB8-20B3-11DA-81A5-0030F1642E3C")
    ITargetInfo : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetTargetMode( 
            /* [retval][out] */ WcmTargetMode *TargetMode) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetTargetMode( 
            /* [in] */ WcmTargetMode TargetMode) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetTemporaryStoreLocation( 
            /* [retval][out] */ BSTR *TemporaryStoreLocation) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetTemporaryStoreLocation( 
            /* [string][in] */ const WCHAR *TemporaryStoreLocation) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetTargetID( 
            /* [retval][out] */ BSTR *TargetID) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetTargetID( 
            /* [in] */ GUID TargetID) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetTargetProcessorArchitecture( 
            /* [retval][out] */ BSTR *ProcessorArchitecture) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetTargetProcessorArchitecture( 
            /* [string][in] */ const WCHAR *ProcessorArchitecture) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetProperty( 
            /* [in] */ BOOL Offline,
            /* [string][in] */ const WCHAR *Property,
            /* [retval][out] */ BSTR *Value) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetProperty( 
            /* [in] */ BOOL Offline,
            /* [string][in] */ const WCHAR *Property,
            /* [string][in] */ const WCHAR *Value) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetEnumerator( 
            /* [retval][out] */ IItemEnumerator **Enumerator) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE ExpandTarget( 
            /* [in] */ BOOL Offline,
            /* [string][in] */ const WCHAR *Location,
            /* [retval][out] */ BSTR *ExpandedLocation) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE ExpandTargetPath( 
            /* [in] */ BOOL Offline,
            /* [string][in] */ const WCHAR *Location,
            /* [retval][out] */ BSTR *ExpandedLocation) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetModulePath( 
            /* [string][in] */ const WCHAR *Module,
            /* [string][in] */ const WCHAR *Path) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE LoadModule( 
            /* [string][in] */ const WCHAR *Module,
            /* [retval][out] */ HMODULE *ModuleHandle) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetWow64Context( 
            /* [string][in] */ const WCHAR *InstallerModule,
            /* [in] */ BYTE *Wow64Context) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE TranslateWow64( 
            /* [string][in] */ const WCHAR *ClientArchitecture,
            /* [string][in] */ const WCHAR *Value,
            /* [retval][out] */ BSTR *TranslatedValue) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetSchemaHiveLocation( 
            /* [in] */ LPCWSTR pwzHiveDir) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetSchemaHiveLocation( 
            /* [retval][out] */ BSTR *pHiveLocation) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetSchemaHiveMountName( 
            /* [in] */ LPCWSTR pwzMountName) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetSchemaHiveMountName( 
            /* [retval][out] */ BSTR *pMountName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITargetInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ITargetInfo * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ITargetInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ITargetInfo * This);
        
        DECLSPEC_XFGVIRT(ITargetInfo, GetTargetMode)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetTargetMode )( 
            ITargetInfo * This,
            /* [retval][out] */ WcmTargetMode *TargetMode);
        
        DECLSPEC_XFGVIRT(ITargetInfo, SetTargetMode)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetTargetMode )( 
            ITargetInfo * This,
            /* [in] */ WcmTargetMode TargetMode);
        
        DECLSPEC_XFGVIRT(ITargetInfo, GetTemporaryStoreLocation)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetTemporaryStoreLocation )( 
            ITargetInfo * This,
            /* [retval][out] */ BSTR *TemporaryStoreLocation);
        
        DECLSPEC_XFGVIRT(ITargetInfo, SetTemporaryStoreLocation)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetTemporaryStoreLocation )( 
            ITargetInfo * This,
            /* [string][in] */ const WCHAR *TemporaryStoreLocation);
        
        DECLSPEC_XFGVIRT(ITargetInfo, GetTargetID)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetTargetID )( 
            ITargetInfo * This,
            /* [retval][out] */ BSTR *TargetID);
        
        DECLSPEC_XFGVIRT(ITargetInfo, SetTargetID)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetTargetID )( 
            ITargetInfo * This,
            /* [in] */ GUID TargetID);
        
        DECLSPEC_XFGVIRT(ITargetInfo, GetTargetProcessorArchitecture)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetTargetProcessorArchitecture )( 
            ITargetInfo * This,
            /* [retval][out] */ BSTR *ProcessorArchitecture);
        
        DECLSPEC_XFGVIRT(ITargetInfo, SetTargetProcessorArchitecture)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetTargetProcessorArchitecture )( 
            ITargetInfo * This,
            /* [string][in] */ const WCHAR *ProcessorArchitecture);
        
        DECLSPEC_XFGVIRT(ITargetInfo, GetProperty)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetProperty )( 
            ITargetInfo * This,
            /* [in] */ BOOL Offline,
            /* [string][in] */ const WCHAR *Property,
            /* [retval][out] */ BSTR *Value);
        
        DECLSPEC_XFGVIRT(ITargetInfo, SetProperty)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetProperty )( 
            ITargetInfo * This,
            /* [in] */ BOOL Offline,
            /* [string][in] */ const WCHAR *Property,
            /* [string][in] */ const WCHAR *Value);
        
        DECLSPEC_XFGVIRT(ITargetInfo, GetEnumerator)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetEnumerator )( 
            ITargetInfo * This,
            /* [retval][out] */ IItemEnumerator **Enumerator);
        
        DECLSPEC_XFGVIRT(ITargetInfo, ExpandTarget)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ExpandTarget )( 
            ITargetInfo * This,
            /* [in] */ BOOL Offline,
            /* [string][in] */ const WCHAR *Location,
            /* [retval][out] */ BSTR *ExpandedLocation);
        
        DECLSPEC_XFGVIRT(ITargetInfo, ExpandTargetPath)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ExpandTargetPath )( 
            ITargetInfo * This,
            /* [in] */ BOOL Offline,
            /* [string][in] */ const WCHAR *Location,
            /* [retval][out] */ BSTR *ExpandedLocation);
        
        DECLSPEC_XFGVIRT(ITargetInfo, SetModulePath)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetModulePath )( 
            ITargetInfo * This,
            /* [string][in] */ const WCHAR *Module,
            /* [string][in] */ const WCHAR *Path);
        
        DECLSPEC_XFGVIRT(ITargetInfo, LoadModule)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *LoadModule )( 
            ITargetInfo * This,
            /* [string][in] */ const WCHAR *Module,
            /* [retval][out] */ HMODULE *ModuleHandle);
        
        DECLSPEC_XFGVIRT(ITargetInfo, SetWow64Context)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetWow64Context )( 
            ITargetInfo * This,
            /* [string][in] */ const WCHAR *InstallerModule,
            /* [in] */ BYTE *Wow64Context);
        
        DECLSPEC_XFGVIRT(ITargetInfo, TranslateWow64)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *TranslateWow64 )( 
            ITargetInfo * This,
            /* [string][in] */ const WCHAR *ClientArchitecture,
            /* [string][in] */ const WCHAR *Value,
            /* [retval][out] */ BSTR *TranslatedValue);
        
        DECLSPEC_XFGVIRT(ITargetInfo, SetSchemaHiveLocation)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetSchemaHiveLocation )( 
            ITargetInfo * This,
            /* [in] */ LPCWSTR pwzHiveDir);
        
        DECLSPEC_XFGVIRT(ITargetInfo, GetSchemaHiveLocation)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetSchemaHiveLocation )( 
            ITargetInfo * This,
            /* [retval][out] */ BSTR *pHiveLocation);
        
        DECLSPEC_XFGVIRT(ITargetInfo, SetSchemaHiveMountName)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetSchemaHiveMountName )( 
            ITargetInfo * This,
            /* [in] */ LPCWSTR pwzMountName);
        
        DECLSPEC_XFGVIRT(ITargetInfo, GetSchemaHiveMountName)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetSchemaHiveMountName )( 
            ITargetInfo * This,
            /* [retval][out] */ BSTR *pMountName);
        
        END_INTERFACE
    } ITargetInfoVtbl;

    interface ITargetInfo
    {
        CONST_VTBL struct ITargetInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITargetInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITargetInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITargetInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITargetInfo_GetTargetMode(This,TargetMode)	\
    ( (This)->lpVtbl -> GetTargetMode(This,TargetMode) ) 

#define ITargetInfo_SetTargetMode(This,TargetMode)	\
    ( (This)->lpVtbl -> SetTargetMode(This,TargetMode) ) 

#define ITargetInfo_GetTemporaryStoreLocation(This,TemporaryStoreLocation)	\
    ( (This)->lpVtbl -> GetTemporaryStoreLocation(This,TemporaryStoreLocation) ) 

#define ITargetInfo_SetTemporaryStoreLocation(This,TemporaryStoreLocation)	\
    ( (This)->lpVtbl -> SetTemporaryStoreLocation(This,TemporaryStoreLocation) ) 

#define ITargetInfo_GetTargetID(This,TargetID)	\
    ( (This)->lpVtbl -> GetTargetID(This,TargetID) ) 

#define ITargetInfo_SetTargetID(This,TargetID)	\
    ( (This)->lpVtbl -> SetTargetID(This,TargetID) ) 

#define ITargetInfo_GetTargetProcessorArchitecture(This,ProcessorArchitecture)	\
    ( (This)->lpVtbl -> GetTargetProcessorArchitecture(This,ProcessorArchitecture) ) 

#define ITargetInfo_SetTargetProcessorArchitecture(This,ProcessorArchitecture)	\
    ( (This)->lpVtbl -> SetTargetProcessorArchitecture(This,ProcessorArchitecture) ) 

#define ITargetInfo_GetProperty(This,Offline,Property,Value)	\
    ( (This)->lpVtbl -> GetProperty(This,Offline,Property,Value) ) 

#define ITargetInfo_SetProperty(This,Offline,Property,Value)	\
    ( (This)->lpVtbl -> SetProperty(This,Offline,Property,Value) ) 

#define ITargetInfo_GetEnumerator(This,Enumerator)	\
    ( (This)->lpVtbl -> GetEnumerator(This,Enumerator) ) 

#define ITargetInfo_ExpandTarget(This,Offline,Location,ExpandedLocation)	\
    ( (This)->lpVtbl -> ExpandTarget(This,Offline,Location,ExpandedLocation) ) 

#define ITargetInfo_ExpandTargetPath(This,Offline,Location,ExpandedLocation)	\
    ( (This)->lpVtbl -> ExpandTargetPath(This,Offline,Location,ExpandedLocation) ) 

#define ITargetInfo_SetModulePath(This,Module,Path)	\
    ( (This)->lpVtbl -> SetModulePath(This,Module,Path) ) 

#define ITargetInfo_LoadModule(This,Module,ModuleHandle)	\
    ( (This)->lpVtbl -> LoadModule(This,Module,ModuleHandle) ) 

#define ITargetInfo_SetWow64Context(This,InstallerModule,Wow64Context)	\
    ( (This)->lpVtbl -> SetWow64Context(This,InstallerModule,Wow64Context) ) 

#define ITargetInfo_TranslateWow64(This,ClientArchitecture,Value,TranslatedValue)	\
    ( (This)->lpVtbl -> TranslateWow64(This,ClientArchitecture,Value,TranslatedValue) ) 

#define ITargetInfo_SetSchemaHiveLocation(This,pwzHiveDir)	\
    ( (This)->lpVtbl -> SetSchemaHiveLocation(This,pwzHiveDir) ) 

#define ITargetInfo_GetSchemaHiveLocation(This,pHiveLocation)	\
    ( (This)->lpVtbl -> GetSchemaHiveLocation(This,pHiveLocation) ) 

#define ITargetInfo_SetSchemaHiveMountName(This,pwzMountName)	\
    ( (This)->lpVtbl -> SetSchemaHiveMountName(This,pwzMountName) ) 

#define ITargetInfo_GetSchemaHiveMountName(This,pMountName)	\
    ( (This)->lpVtbl -> GetSchemaHiveMountName(This,pMountName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITargetInfo_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_wcmconfig_0000_0003 */
/* [local] */ 

#define LINK_STORE_TO_ENGINE_INSTANCE    0x00000001


extern RPC_IF_HANDLE __MIDL_itf_wcmconfig_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wcmconfig_0000_0003_v0_0_s_ifspec;

#ifndef __ISettingsEngine_INTERFACE_DEFINED__
#define __ISettingsEngine_INTERFACE_DEFINED__

/* interface ISettingsEngine */
/* [helpstring][uuid][unique][nonextensible][object][local] */ 


EXTERN_C const IID IID_ISettingsEngine;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9F7D7BB9-20B3-11DA-81A5-0030F1642E3C")
    ISettingsEngine : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetNamespaces( 
            /* [in] */ WcmNamespaceEnumerationFlags Flags,
            /* [in] */ void *Reserved,
            /* [retval][out] */ IItemEnumerator **Namespaces) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetNamespace( 
            /* [in] */ ISettingsIdentity *SettingsID,
            /* [in] */ WcmNamespaceAccess Access,
            /* [in] */ void *Reserved,
            /* [retval][out] */ ISettingsNamespace **NamespaceItem) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetErrorDescription( 
            /* [in] */ LONG HResult,
            /* [retval][out] */ BSTR *Message) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE CreateSettingsIdentity( 
            /* [retval][out] */ ISettingsIdentity **SettingsID) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetStoreStatus( 
            /* [in] */ void *Reserved,
            /* [out] */ WcmUserStatus *Status) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE LoadStore( 
            /* [in] */ DWORD Flags) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE UnloadStore( 
            /* [in] */ void *Reserved) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE RegisterNamespace( 
            /* [in] */ ISettingsIdentity *SettingsID,
            /* [in] */ IStream *Stream,
            /* [in] */ BOOL PushSettings,
            /* [retval][out] */ VARIANT *Results) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE UnregisterNamespace( 
            /* [in] */ ISettingsIdentity *SettingsID,
            /* [in] */ BOOL RemoveSettings) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE CreateTargetInfo( 
            /* [retval][out] */ ITargetInfo **Target) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetTargetInfo( 
            /* [retval][out] */ ITargetInfo **Target) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetTargetInfo( 
            /* [in] */ ITargetInfo *Target) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE CreateSettingsContext( 
            /* [in] */ DWORD Flags,
            /* [in] */ void *Reserved,
            /* [retval][out] */ ISettingsContext **SettingsContext) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetSettingsContext( 
            /* [in] */ ISettingsContext *SettingsContext) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE ApplySettingsContext( 
            /* [in] */ ISettingsContext *SettingsContext,
            /* [annotation][size_is][size_is][out] */ 
            _Outptr_result_buffer_(*pcIdentities)  LPWSTR **pppwzIdentities,
            /* [retval][out] */ SIZE_T *pcIdentities) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetSettingsContext( 
            /* [retval][out] */ ISettingsContext **SettingsContext) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISettingsEngineVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISettingsEngine * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISettingsEngine * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISettingsEngine * This);
        
        DECLSPEC_XFGVIRT(ISettingsEngine, GetNamespaces)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetNamespaces )( 
            ISettingsEngine * This,
            /* [in] */ WcmNamespaceEnumerationFlags Flags,
            /* [in] */ void *Reserved,
            /* [retval][out] */ IItemEnumerator **Namespaces);
        
        DECLSPEC_XFGVIRT(ISettingsEngine, GetNamespace)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetNamespace )( 
            ISettingsEngine * This,
            /* [in] */ ISettingsIdentity *SettingsID,
            /* [in] */ WcmNamespaceAccess Access,
            /* [in] */ void *Reserved,
            /* [retval][out] */ ISettingsNamespace **NamespaceItem);
        
        DECLSPEC_XFGVIRT(ISettingsEngine, GetErrorDescription)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetErrorDescription )( 
            ISettingsEngine * This,
            /* [in] */ LONG HResult,
            /* [retval][out] */ BSTR *Message);
        
        DECLSPEC_XFGVIRT(ISettingsEngine, CreateSettingsIdentity)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *CreateSettingsIdentity )( 
            ISettingsEngine * This,
            /* [retval][out] */ ISettingsIdentity **SettingsID);
        
        DECLSPEC_XFGVIRT(ISettingsEngine, GetStoreStatus)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetStoreStatus )( 
            ISettingsEngine * This,
            /* [in] */ void *Reserved,
            /* [out] */ WcmUserStatus *Status);
        
        DECLSPEC_XFGVIRT(ISettingsEngine, LoadStore)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *LoadStore )( 
            ISettingsEngine * This,
            /* [in] */ DWORD Flags);
        
        DECLSPEC_XFGVIRT(ISettingsEngine, UnloadStore)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *UnloadStore )( 
            ISettingsEngine * This,
            /* [in] */ void *Reserved);
        
        DECLSPEC_XFGVIRT(ISettingsEngine, RegisterNamespace)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *RegisterNamespace )( 
            ISettingsEngine * This,
            /* [in] */ ISettingsIdentity *SettingsID,
            /* [in] */ IStream *Stream,
            /* [in] */ BOOL PushSettings,
            /* [retval][out] */ VARIANT *Results);
        
        DECLSPEC_XFGVIRT(ISettingsEngine, UnregisterNamespace)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *UnregisterNamespace )( 
            ISettingsEngine * This,
            /* [in] */ ISettingsIdentity *SettingsID,
            /* [in] */ BOOL RemoveSettings);
        
        DECLSPEC_XFGVIRT(ISettingsEngine, CreateTargetInfo)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *CreateTargetInfo )( 
            ISettingsEngine * This,
            /* [retval][out] */ ITargetInfo **Target);
        
        DECLSPEC_XFGVIRT(ISettingsEngine, GetTargetInfo)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetTargetInfo )( 
            ISettingsEngine * This,
            /* [retval][out] */ ITargetInfo **Target);
        
        DECLSPEC_XFGVIRT(ISettingsEngine, SetTargetInfo)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetTargetInfo )( 
            ISettingsEngine * This,
            /* [in] */ ITargetInfo *Target);
        
        DECLSPEC_XFGVIRT(ISettingsEngine, CreateSettingsContext)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *CreateSettingsContext )( 
            ISettingsEngine * This,
            /* [in] */ DWORD Flags,
            /* [in] */ void *Reserved,
            /* [retval][out] */ ISettingsContext **SettingsContext);
        
        DECLSPEC_XFGVIRT(ISettingsEngine, SetSettingsContext)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetSettingsContext )( 
            ISettingsEngine * This,
            /* [in] */ ISettingsContext *SettingsContext);
        
        DECLSPEC_XFGVIRT(ISettingsEngine, ApplySettingsContext)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ApplySettingsContext )( 
            ISettingsEngine * This,
            /* [in] */ ISettingsContext *SettingsContext,
            /* [annotation][size_is][size_is][out] */ 
            _Outptr_result_buffer_(*pcIdentities)  LPWSTR **pppwzIdentities,
            /* [retval][out] */ SIZE_T *pcIdentities);
        
        DECLSPEC_XFGVIRT(ISettingsEngine, GetSettingsContext)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetSettingsContext )( 
            ISettingsEngine * This,
            /* [retval][out] */ ISettingsContext **SettingsContext);
        
        END_INTERFACE
    } ISettingsEngineVtbl;

    interface ISettingsEngine
    {
        CONST_VTBL struct ISettingsEngineVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISettingsEngine_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISettingsEngine_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISettingsEngine_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISettingsEngine_GetNamespaces(This,Flags,Reserved,Namespaces)	\
    ( (This)->lpVtbl -> GetNamespaces(This,Flags,Reserved,Namespaces) ) 

#define ISettingsEngine_GetNamespace(This,SettingsID,Access,Reserved,NamespaceItem)	\
    ( (This)->lpVtbl -> GetNamespace(This,SettingsID,Access,Reserved,NamespaceItem) ) 

#define ISettingsEngine_GetErrorDescription(This,HResult,Message)	\
    ( (This)->lpVtbl -> GetErrorDescription(This,HResult,Message) ) 

#define ISettingsEngine_CreateSettingsIdentity(This,SettingsID)	\
    ( (This)->lpVtbl -> CreateSettingsIdentity(This,SettingsID) ) 

#define ISettingsEngine_GetStoreStatus(This,Reserved,Status)	\
    ( (This)->lpVtbl -> GetStoreStatus(This,Reserved,Status) ) 

#define ISettingsEngine_LoadStore(This,Flags)	\
    ( (This)->lpVtbl -> LoadStore(This,Flags) ) 

#define ISettingsEngine_UnloadStore(This,Reserved)	\
    ( (This)->lpVtbl -> UnloadStore(This,Reserved) ) 

#define ISettingsEngine_RegisterNamespace(This,SettingsID,Stream,PushSettings,Results)	\
    ( (This)->lpVtbl -> RegisterNamespace(This,SettingsID,Stream,PushSettings,Results) ) 

#define ISettingsEngine_UnregisterNamespace(This,SettingsID,RemoveSettings)	\
    ( (This)->lpVtbl -> UnregisterNamespace(This,SettingsID,RemoveSettings) ) 

#define ISettingsEngine_CreateTargetInfo(This,Target)	\
    ( (This)->lpVtbl -> CreateTargetInfo(This,Target) ) 

#define ISettingsEngine_GetTargetInfo(This,Target)	\
    ( (This)->lpVtbl -> GetTargetInfo(This,Target) ) 

#define ISettingsEngine_SetTargetInfo(This,Target)	\
    ( (This)->lpVtbl -> SetTargetInfo(This,Target) ) 

#define ISettingsEngine_CreateSettingsContext(This,Flags,Reserved,SettingsContext)	\
    ( (This)->lpVtbl -> CreateSettingsContext(This,Flags,Reserved,SettingsContext) ) 

#define ISettingsEngine_SetSettingsContext(This,SettingsContext)	\
    ( (This)->lpVtbl -> SetSettingsContext(This,SettingsContext) ) 

#define ISettingsEngine_ApplySettingsContext(This,SettingsContext,pppwzIdentities,pcIdentities)	\
    ( (This)->lpVtbl -> ApplySettingsContext(This,SettingsContext,pppwzIdentities,pcIdentities) ) 

#define ISettingsEngine_GetSettingsContext(This,SettingsContext)	\
    ( (This)->lpVtbl -> GetSettingsContext(This,SettingsContext) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISettingsEngine_INTERFACE_DEFINED__ */


#ifndef __ISettingsItem_INTERFACE_DEFINED__
#define __ISettingsItem_INTERFACE_DEFINED__

/* interface ISettingsItem */
/* [helpstring][uuid][unique][nonextensible][object][local] */ 


EXTERN_C const IID IID_ISettingsItem;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9F7D7BBB-20B3-11DA-81A5-0030F1642E3C")
    ISettingsItem : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetName( 
            /* [retval][out] */ BSTR *Name) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetValue( 
            /* [retval][out] */ VARIANT *Value) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetValue( 
            /* [in] */ const VARIANT *Value) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetSettingType( 
            /* [retval][out] */ WcmSettingType *Type) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetDataType( 
            /* [retval][out] */ WcmDataType *Type) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetValueRaw( 
            /* [size_is][size_is][out] */ BYTE **Data,
            /* [retval][out] */ ULONG *DataSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetValueRaw( 
            /* [in] */ LONG DataType,
            /* [size_is][in] */ const BYTE *Data,
            /* [in] */ ULONG DataSize) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE HasChild( 
            /* [retval][out] */ BOOL *ItemHasChild) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Children( 
            /* [retval][out] */ IItemEnumerator **Children) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetChild( 
            /* [string][in] */ const WCHAR *Name,
            /* [retval][out] */ ISettingsItem **Child) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetSettingByPath( 
            /* [string][in] */ const WCHAR *Path,
            /* [retval][out] */ ISettingsItem **Setting) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE CreateSettingByPath( 
            /* [string][in] */ const WCHAR *Path,
            /* [retval][out] */ ISettingsItem **Setting) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE RemoveSettingByPath( 
            /* [string][in] */ const WCHAR *Path) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetListKeyInformation( 
            /* [out] */ BSTR *KeyName,
            /* [retval][out] */ WcmDataType *DataType) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE CreateListElement( 
            /* [in] */ const VARIANT *KeyData,
            /* [retval][out] */ ISettingsItem **Child) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE RemoveListElement( 
            /* [string][in] */ LPCWSTR ElementName) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Attributes( 
            /* [retval][out] */ IItemEnumerator **Attributes) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetAttribute( 
            /* [string][in] */ const WCHAR *Name,
            /* [retval][out] */ VARIANT *Value) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetPath( 
            /* [retval][out] */ BSTR *Path) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetRestrictionFacets( 
            /* [retval][out] */ WcmRestrictionFacets *RestrictionFacets) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetRestriction( 
            /* [in] */ WcmRestrictionFacets RestrictionFacet,
            /* [retval][out] */ VARIANT *FacetData) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetKeyValue( 
            /* [retval][out] */ VARIANT *Value) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISettingsItemVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISettingsItem * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISettingsItem * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISettingsItem * This);
        
        DECLSPEC_XFGVIRT(ISettingsItem, GetName)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetName )( 
            ISettingsItem * This,
            /* [retval][out] */ BSTR *Name);
        
        DECLSPEC_XFGVIRT(ISettingsItem, GetValue)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetValue )( 
            ISettingsItem * This,
            /* [retval][out] */ VARIANT *Value);
        
        DECLSPEC_XFGVIRT(ISettingsItem, SetValue)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetValue )( 
            ISettingsItem * This,
            /* [in] */ const VARIANT *Value);
        
        DECLSPEC_XFGVIRT(ISettingsItem, GetSettingType)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetSettingType )( 
            ISettingsItem * This,
            /* [retval][out] */ WcmSettingType *Type);
        
        DECLSPEC_XFGVIRT(ISettingsItem, GetDataType)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetDataType )( 
            ISettingsItem * This,
            /* [retval][out] */ WcmDataType *Type);
        
        DECLSPEC_XFGVIRT(ISettingsItem, GetValueRaw)
        HRESULT ( STDMETHODCALLTYPE *GetValueRaw )( 
            ISettingsItem * This,
            /* [size_is][size_is][out] */ BYTE **Data,
            /* [retval][out] */ ULONG *DataSize);
        
        DECLSPEC_XFGVIRT(ISettingsItem, SetValueRaw)
        HRESULT ( STDMETHODCALLTYPE *SetValueRaw )( 
            ISettingsItem * This,
            /* [in] */ LONG DataType,
            /* [size_is][in] */ const BYTE *Data,
            /* [in] */ ULONG DataSize);
        
        DECLSPEC_XFGVIRT(ISettingsItem, HasChild)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *HasChild )( 
            ISettingsItem * This,
            /* [retval][out] */ BOOL *ItemHasChild);
        
        DECLSPEC_XFGVIRT(ISettingsItem, Children)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Children )( 
            ISettingsItem * This,
            /* [retval][out] */ IItemEnumerator **Children);
        
        DECLSPEC_XFGVIRT(ISettingsItem, GetChild)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetChild )( 
            ISettingsItem * This,
            /* [string][in] */ const WCHAR *Name,
            /* [retval][out] */ ISettingsItem **Child);
        
        DECLSPEC_XFGVIRT(ISettingsItem, GetSettingByPath)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetSettingByPath )( 
            ISettingsItem * This,
            /* [string][in] */ const WCHAR *Path,
            /* [retval][out] */ ISettingsItem **Setting);
        
        DECLSPEC_XFGVIRT(ISettingsItem, CreateSettingByPath)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *CreateSettingByPath )( 
            ISettingsItem * This,
            /* [string][in] */ const WCHAR *Path,
            /* [retval][out] */ ISettingsItem **Setting);
        
        DECLSPEC_XFGVIRT(ISettingsItem, RemoveSettingByPath)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *RemoveSettingByPath )( 
            ISettingsItem * This,
            /* [string][in] */ const WCHAR *Path);
        
        DECLSPEC_XFGVIRT(ISettingsItem, GetListKeyInformation)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetListKeyInformation )( 
            ISettingsItem * This,
            /* [out] */ BSTR *KeyName,
            /* [retval][out] */ WcmDataType *DataType);
        
        DECLSPEC_XFGVIRT(ISettingsItem, CreateListElement)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *CreateListElement )( 
            ISettingsItem * This,
            /* [in] */ const VARIANT *KeyData,
            /* [retval][out] */ ISettingsItem **Child);
        
        DECLSPEC_XFGVIRT(ISettingsItem, RemoveListElement)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *RemoveListElement )( 
            ISettingsItem * This,
            /* [string][in] */ LPCWSTR ElementName);
        
        DECLSPEC_XFGVIRT(ISettingsItem, Attributes)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Attributes )( 
            ISettingsItem * This,
            /* [retval][out] */ IItemEnumerator **Attributes);
        
        DECLSPEC_XFGVIRT(ISettingsItem, GetAttribute)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetAttribute )( 
            ISettingsItem * This,
            /* [string][in] */ const WCHAR *Name,
            /* [retval][out] */ VARIANT *Value);
        
        DECLSPEC_XFGVIRT(ISettingsItem, GetPath)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetPath )( 
            ISettingsItem * This,
            /* [retval][out] */ BSTR *Path);
        
        DECLSPEC_XFGVIRT(ISettingsItem, GetRestrictionFacets)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetRestrictionFacets )( 
            ISettingsItem * This,
            /* [retval][out] */ WcmRestrictionFacets *RestrictionFacets);
        
        DECLSPEC_XFGVIRT(ISettingsItem, GetRestriction)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetRestriction )( 
            ISettingsItem * This,
            /* [in] */ WcmRestrictionFacets RestrictionFacet,
            /* [retval][out] */ VARIANT *FacetData);
        
        DECLSPEC_XFGVIRT(ISettingsItem, GetKeyValue)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetKeyValue )( 
            ISettingsItem * This,
            /* [retval][out] */ VARIANT *Value);
        
        END_INTERFACE
    } ISettingsItemVtbl;

    interface ISettingsItem
    {
        CONST_VTBL struct ISettingsItemVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISettingsItem_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISettingsItem_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISettingsItem_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISettingsItem_GetName(This,Name)	\
    ( (This)->lpVtbl -> GetName(This,Name) ) 

#define ISettingsItem_GetValue(This,Value)	\
    ( (This)->lpVtbl -> GetValue(This,Value) ) 

#define ISettingsItem_SetValue(This,Value)	\
    ( (This)->lpVtbl -> SetValue(This,Value) ) 

#define ISettingsItem_GetSettingType(This,Type)	\
    ( (This)->lpVtbl -> GetSettingType(This,Type) ) 

#define ISettingsItem_GetDataType(This,Type)	\
    ( (This)->lpVtbl -> GetDataType(This,Type) ) 

#define ISettingsItem_GetValueRaw(This,Data,DataSize)	\
    ( (This)->lpVtbl -> GetValueRaw(This,Data,DataSize) ) 

#define ISettingsItem_SetValueRaw(This,DataType,Data,DataSize)	\
    ( (This)->lpVtbl -> SetValueRaw(This,DataType,Data,DataSize) ) 

#define ISettingsItem_HasChild(This,ItemHasChild)	\
    ( (This)->lpVtbl -> HasChild(This,ItemHasChild) ) 

#define ISettingsItem_Children(This,Children)	\
    ( (This)->lpVtbl -> Children(This,Children) ) 

#define ISettingsItem_GetChild(This,Name,Child)	\
    ( (This)->lpVtbl -> GetChild(This,Name,Child) ) 

#define ISettingsItem_GetSettingByPath(This,Path,Setting)	\
    ( (This)->lpVtbl -> GetSettingByPath(This,Path,Setting) ) 

#define ISettingsItem_CreateSettingByPath(This,Path,Setting)	\
    ( (This)->lpVtbl -> CreateSettingByPath(This,Path,Setting) ) 

#define ISettingsItem_RemoveSettingByPath(This,Path)	\
    ( (This)->lpVtbl -> RemoveSettingByPath(This,Path) ) 

#define ISettingsItem_GetListKeyInformation(This,KeyName,DataType)	\
    ( (This)->lpVtbl -> GetListKeyInformation(This,KeyName,DataType) ) 

#define ISettingsItem_CreateListElement(This,KeyData,Child)	\
    ( (This)->lpVtbl -> CreateListElement(This,KeyData,Child) ) 

#define ISettingsItem_RemoveListElement(This,ElementName)	\
    ( (This)->lpVtbl -> RemoveListElement(This,ElementName) ) 

#define ISettingsItem_Attributes(This,Attributes)	\
    ( (This)->lpVtbl -> Attributes(This,Attributes) ) 

#define ISettingsItem_GetAttribute(This,Name,Value)	\
    ( (This)->lpVtbl -> GetAttribute(This,Name,Value) ) 

#define ISettingsItem_GetPath(This,Path)	\
    ( (This)->lpVtbl -> GetPath(This,Path) ) 

#define ISettingsItem_GetRestrictionFacets(This,RestrictionFacets)	\
    ( (This)->lpVtbl -> GetRestrictionFacets(This,RestrictionFacets) ) 

#define ISettingsItem_GetRestriction(This,RestrictionFacet,FacetData)	\
    ( (This)->lpVtbl -> GetRestriction(This,RestrictionFacet,FacetData) ) 

#define ISettingsItem_GetKeyValue(This,Value)	\
    ( (This)->lpVtbl -> GetKeyValue(This,Value) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISettingsItem_INTERFACE_DEFINED__ */


#ifndef __ISettingsNamespace_INTERFACE_DEFINED__
#define __ISettingsNamespace_INTERFACE_DEFINED__

/* interface ISettingsNamespace */
/* [helpstring][uuid][unique][nonextensible][object][local] */ 


EXTERN_C const IID IID_ISettingsNamespace;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9F7D7BBA-20B3-11DA-81A5-0030F1642E3C")
    ISettingsNamespace : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetIdentity( 
            /* [retval][out] */ ISettingsIdentity **SettingsID) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Settings( 
            /* [retval][out] */ IItemEnumerator **Settings) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Save( 
            /* [in] */ BOOL PushSettings,
            /* [retval][out] */ ISettingsResult **Result) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetSettingByPath( 
            /* [string][in] */ const WCHAR *Path,
            /* [retval][out] */ ISettingsItem **Setting) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE CreateSettingByPath( 
            /* [string][in] */ const WCHAR *Path,
            /* [retval][out] */ ISettingsItem **Setting) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE RemoveSettingByPath( 
            /* [string][in] */ const WCHAR *Path) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetAttribute( 
            /* [string][in] */ const WCHAR *Name,
            /* [retval][out] */ VARIANT *Value) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISettingsNamespaceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISettingsNamespace * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISettingsNamespace * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISettingsNamespace * This);
        
        DECLSPEC_XFGVIRT(ISettingsNamespace, GetIdentity)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetIdentity )( 
            ISettingsNamespace * This,
            /* [retval][out] */ ISettingsIdentity **SettingsID);
        
        DECLSPEC_XFGVIRT(ISettingsNamespace, Settings)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Settings )( 
            ISettingsNamespace * This,
            /* [retval][out] */ IItemEnumerator **Settings);
        
        DECLSPEC_XFGVIRT(ISettingsNamespace, Save)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Save )( 
            ISettingsNamespace * This,
            /* [in] */ BOOL PushSettings,
            /* [retval][out] */ ISettingsResult **Result);
        
        DECLSPEC_XFGVIRT(ISettingsNamespace, GetSettingByPath)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetSettingByPath )( 
            ISettingsNamespace * This,
            /* [string][in] */ const WCHAR *Path,
            /* [retval][out] */ ISettingsItem **Setting);
        
        DECLSPEC_XFGVIRT(ISettingsNamespace, CreateSettingByPath)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *CreateSettingByPath )( 
            ISettingsNamespace * This,
            /* [string][in] */ const WCHAR *Path,
            /* [retval][out] */ ISettingsItem **Setting);
        
        DECLSPEC_XFGVIRT(ISettingsNamespace, RemoveSettingByPath)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *RemoveSettingByPath )( 
            ISettingsNamespace * This,
            /* [string][in] */ const WCHAR *Path);
        
        DECLSPEC_XFGVIRT(ISettingsNamespace, GetAttribute)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetAttribute )( 
            ISettingsNamespace * This,
            /* [string][in] */ const WCHAR *Name,
            /* [retval][out] */ VARIANT *Value);
        
        END_INTERFACE
    } ISettingsNamespaceVtbl;

    interface ISettingsNamespace
    {
        CONST_VTBL struct ISettingsNamespaceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISettingsNamespace_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISettingsNamespace_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISettingsNamespace_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISettingsNamespace_GetIdentity(This,SettingsID)	\
    ( (This)->lpVtbl -> GetIdentity(This,SettingsID) ) 

#define ISettingsNamespace_Settings(This,Settings)	\
    ( (This)->lpVtbl -> Settings(This,Settings) ) 

#define ISettingsNamespace_Save(This,PushSettings,Result)	\
    ( (This)->lpVtbl -> Save(This,PushSettings,Result) ) 

#define ISettingsNamespace_GetSettingByPath(This,Path,Setting)	\
    ( (This)->lpVtbl -> GetSettingByPath(This,Path,Setting) ) 

#define ISettingsNamespace_CreateSettingByPath(This,Path,Setting)	\
    ( (This)->lpVtbl -> CreateSettingByPath(This,Path,Setting) ) 

#define ISettingsNamespace_RemoveSettingByPath(This,Path)	\
    ( (This)->lpVtbl -> RemoveSettingByPath(This,Path) ) 

#define ISettingsNamespace_GetAttribute(This,Name,Value)	\
    ( (This)->lpVtbl -> GetAttribute(This,Name,Value) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISettingsNamespace_INTERFACE_DEFINED__ */


#ifndef __ISettingsResult_INTERFACE_DEFINED__
#define __ISettingsResult_INTERFACE_DEFINED__

/* interface ISettingsResult */
/* [helpstring][uuid][unique][nonextensible][object][local] */ 


EXTERN_C const IID IID_ISettingsResult;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9F7D7BBC-20B3-11DA-81A5-0030F1642E3C")
    ISettingsResult : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetDescription( 
            /* [retval][out] */ BSTR *description) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetErrorCode( 
            /* [retval][out] */ HRESULT *hrOut) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetContextDescription( 
            /* [retval][out] */ BSTR *description) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetLine( 
            /* [retval][out] */ DWORD *dwLine) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetColumn( 
            /* [retval][out] */ DWORD *dwColumn) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetSource( 
            /* [retval][out] */ BSTR *file) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISettingsResultVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISettingsResult * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISettingsResult * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISettingsResult * This);
        
        DECLSPEC_XFGVIRT(ISettingsResult, GetDescription)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetDescription )( 
            ISettingsResult * This,
            /* [retval][out] */ BSTR *description);
        
        DECLSPEC_XFGVIRT(ISettingsResult, GetErrorCode)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetErrorCode )( 
            ISettingsResult * This,
            /* [retval][out] */ HRESULT *hrOut);
        
        DECLSPEC_XFGVIRT(ISettingsResult, GetContextDescription)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetContextDescription )( 
            ISettingsResult * This,
            /* [retval][out] */ BSTR *description);
        
        DECLSPEC_XFGVIRT(ISettingsResult, GetLine)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetLine )( 
            ISettingsResult * This,
            /* [retval][out] */ DWORD *dwLine);
        
        DECLSPEC_XFGVIRT(ISettingsResult, GetColumn)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetColumn )( 
            ISettingsResult * This,
            /* [retval][out] */ DWORD *dwColumn);
        
        DECLSPEC_XFGVIRT(ISettingsResult, GetSource)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetSource )( 
            ISettingsResult * This,
            /* [retval][out] */ BSTR *file);
        
        END_INTERFACE
    } ISettingsResultVtbl;

    interface ISettingsResult
    {
        CONST_VTBL struct ISettingsResultVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISettingsResult_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISettingsResult_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISettingsResult_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISettingsResult_GetDescription(This,description)	\
    ( (This)->lpVtbl -> GetDescription(This,description) ) 

#define ISettingsResult_GetErrorCode(This,hrOut)	\
    ( (This)->lpVtbl -> GetErrorCode(This,hrOut) ) 

#define ISettingsResult_GetContextDescription(This,description)	\
    ( (This)->lpVtbl -> GetContextDescription(This,description) ) 

#define ISettingsResult_GetLine(This,dwLine)	\
    ( (This)->lpVtbl -> GetLine(This,dwLine) ) 

#define ISettingsResult_GetColumn(This,dwColumn)	\
    ( (This)->lpVtbl -> GetColumn(This,dwColumn) ) 

#define ISettingsResult_GetSource(This,file)	\
    ( (This)->lpVtbl -> GetSource(This,file) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISettingsResult_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_wcmconfig_0000_0007 */
/* [local] */ 

#define LIMITED_VALIDATION_MODE    0x00000001


extern RPC_IF_HANDLE __MIDL_itf_wcmconfig_0000_0007_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wcmconfig_0000_0007_v0_0_s_ifspec;

#ifndef __ISettingsContext_INTERFACE_DEFINED__
#define __ISettingsContext_INTERFACE_DEFINED__

/* interface ISettingsContext */
/* [helpstring][uuid][unique][nonextensible][object][local] */ 


EXTERN_C const IID IID_ISettingsContext;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9F7D7BBD-20B3-11DA-81A5-0030F1642E3C")
    ISettingsContext : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Serialize( 
            /* [in] */ IStream *pStream,
            /* [in] */ ITargetInfo *pTarget) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Deserialize( 
            /* [in] */ IStream *pStream,
            /* [in] */ ITargetInfo *pTarget,
            /* [size_is][size_is][out] */ ISettingsResult ***pppResults,
            /* [retval][out] */ SIZE_T *pcResultCount) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetUserData( 
            /* [in] */ void *pUserData) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetUserData( 
            /* [retval][out] */ void **pUserData) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetNamespaces( 
            /* [retval][out] */ IItemEnumerator **ppNamespaceIds) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetStoredSettings( 
            /* [in] */ ISettingsIdentity *pIdentity,
            /* [out] */ IItemEnumerator **ppAddedSettings,
            /* [out] */ IItemEnumerator **ppModifiedSettings,
            /* [out] */ IItemEnumerator **ppDeletedSettings) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE RevertSetting( 
            /* [in] */ ISettingsIdentity *pIdentity,
            /* [in] */ LPCWSTR pwzSetting) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISettingsContextVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISettingsContext * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISettingsContext * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISettingsContext * This);
        
        DECLSPEC_XFGVIRT(ISettingsContext, Serialize)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Serialize )( 
            ISettingsContext * This,
            /* [in] */ IStream *pStream,
            /* [in] */ ITargetInfo *pTarget);
        
        DECLSPEC_XFGVIRT(ISettingsContext, Deserialize)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Deserialize )( 
            ISettingsContext * This,
            /* [in] */ IStream *pStream,
            /* [in] */ ITargetInfo *pTarget,
            /* [size_is][size_is][out] */ ISettingsResult ***pppResults,
            /* [retval][out] */ SIZE_T *pcResultCount);
        
        DECLSPEC_XFGVIRT(ISettingsContext, SetUserData)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetUserData )( 
            ISettingsContext * This,
            /* [in] */ void *pUserData);
        
        DECLSPEC_XFGVIRT(ISettingsContext, GetUserData)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetUserData )( 
            ISettingsContext * This,
            /* [retval][out] */ void **pUserData);
        
        DECLSPEC_XFGVIRT(ISettingsContext, GetNamespaces)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetNamespaces )( 
            ISettingsContext * This,
            /* [retval][out] */ IItemEnumerator **ppNamespaceIds);
        
        DECLSPEC_XFGVIRT(ISettingsContext, GetStoredSettings)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetStoredSettings )( 
            ISettingsContext * This,
            /* [in] */ ISettingsIdentity *pIdentity,
            /* [out] */ IItemEnumerator **ppAddedSettings,
            /* [out] */ IItemEnumerator **ppModifiedSettings,
            /* [out] */ IItemEnumerator **ppDeletedSettings);
        
        DECLSPEC_XFGVIRT(ISettingsContext, RevertSetting)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *RevertSetting )( 
            ISettingsContext * This,
            /* [in] */ ISettingsIdentity *pIdentity,
            /* [in] */ LPCWSTR pwzSetting);
        
        END_INTERFACE
    } ISettingsContextVtbl;

    interface ISettingsContext
    {
        CONST_VTBL struct ISettingsContextVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISettingsContext_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISettingsContext_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISettingsContext_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISettingsContext_Serialize(This,pStream,pTarget)	\
    ( (This)->lpVtbl -> Serialize(This,pStream,pTarget) ) 

#define ISettingsContext_Deserialize(This,pStream,pTarget,pppResults,pcResultCount)	\
    ( (This)->lpVtbl -> Deserialize(This,pStream,pTarget,pppResults,pcResultCount) ) 

#define ISettingsContext_SetUserData(This,pUserData)	\
    ( (This)->lpVtbl -> SetUserData(This,pUserData) ) 

#define ISettingsContext_GetUserData(This,pUserData)	\
    ( (This)->lpVtbl -> GetUserData(This,pUserData) ) 

#define ISettingsContext_GetNamespaces(This,ppNamespaceIds)	\
    ( (This)->lpVtbl -> GetNamespaces(This,ppNamespaceIds) ) 

#define ISettingsContext_GetStoredSettings(This,pIdentity,ppAddedSettings,ppModifiedSettings,ppDeletedSettings)	\
    ( (This)->lpVtbl -> GetStoredSettings(This,pIdentity,ppAddedSettings,ppModifiedSettings,ppDeletedSettings) ) 

#define ISettingsContext_RevertSetting(This,pIdentity,pwzSetting)	\
    ( (This)->lpVtbl -> RevertSetting(This,pIdentity,pwzSetting) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISettingsContext_INTERFACE_DEFINED__ */



#ifndef __Wcm_LIBRARY_DEFINED__
#define __Wcm_LIBRARY_DEFINED__

/* library Wcm */
/* [version][lcid][helpstring][helpfile][uuid] */ 


EXTERN_C const IID LIBID_Wcm;

EXTERN_C const CLSID CLSID_SettingsEngine;

#ifdef __cplusplus

class DECLSPEC_UUID("9F7D7BB5-20B3-11DA-81A5-0030F1642E3C")
SettingsEngine;
#endif
#endif /* __Wcm_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_wcmconfig_0000_0009 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_wcmconfig_0000_0009_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wcmconfig_0000_0009_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


