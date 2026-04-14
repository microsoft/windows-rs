

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

#ifndef __activprof_h__
#define __activprof_h__

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

#ifndef __IActiveScriptProfilerControl_FWD_DEFINED__
#define __IActiveScriptProfilerControl_FWD_DEFINED__
typedef interface IActiveScriptProfilerControl IActiveScriptProfilerControl;

#endif 	/* __IActiveScriptProfilerControl_FWD_DEFINED__ */


#ifndef __IActiveScriptProfilerControl2_FWD_DEFINED__
#define __IActiveScriptProfilerControl2_FWD_DEFINED__
typedef interface IActiveScriptProfilerControl2 IActiveScriptProfilerControl2;

#endif 	/* __IActiveScriptProfilerControl2_FWD_DEFINED__ */


#ifndef __IActiveScriptProfilerHeapEnum_FWD_DEFINED__
#define __IActiveScriptProfilerHeapEnum_FWD_DEFINED__
typedef interface IActiveScriptProfilerHeapEnum IActiveScriptProfilerHeapEnum;

#endif 	/* __IActiveScriptProfilerHeapEnum_FWD_DEFINED__ */


#ifndef __IActiveScriptProfilerControl3_FWD_DEFINED__
#define __IActiveScriptProfilerControl3_FWD_DEFINED__
typedef interface IActiveScriptProfilerControl3 IActiveScriptProfilerControl3;

#endif 	/* __IActiveScriptProfilerControl3_FWD_DEFINED__ */


#ifndef __IActiveScriptProfilerControl4_FWD_DEFINED__
#define __IActiveScriptProfilerControl4_FWD_DEFINED__
typedef interface IActiveScriptProfilerControl4 IActiveScriptProfilerControl4;

#endif 	/* __IActiveScriptProfilerControl4_FWD_DEFINED__ */


#ifndef __IActiveScriptProfilerControl5_FWD_DEFINED__
#define __IActiveScriptProfilerControl5_FWD_DEFINED__
typedef interface IActiveScriptProfilerControl5 IActiveScriptProfilerControl5;

#endif 	/* __IActiveScriptProfilerControl5_FWD_DEFINED__ */


#ifndef __IActiveScriptProfilerCallback_FWD_DEFINED__
#define __IActiveScriptProfilerCallback_FWD_DEFINED__
typedef interface IActiveScriptProfilerCallback IActiveScriptProfilerCallback;

#endif 	/* __IActiveScriptProfilerCallback_FWD_DEFINED__ */


#ifndef __IActiveScriptProfilerCallback2_FWD_DEFINED__
#define __IActiveScriptProfilerCallback2_FWD_DEFINED__
typedef interface IActiveScriptProfilerCallback2 IActiveScriptProfilerCallback2;

#endif 	/* __IActiveScriptProfilerCallback2_FWD_DEFINED__ */


#ifndef __IActiveScriptProfilerCallback3_FWD_DEFINED__
#define __IActiveScriptProfilerCallback3_FWD_DEFINED__
typedef interface IActiveScriptProfilerCallback3 IActiveScriptProfilerCallback3;

#endif 	/* __IActiveScriptProfilerCallback3_FWD_DEFINED__ */


/* header files for imported files */
#include "unknwn.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_activprof_0000_0000 */
/* [local] */ 

//=--------------------------------------------------------------------------=
// ActivProf.h
//=--------------------------------------------------------------------------=
// (C) Copyright 2000 Microsoft Corporation.  All Rights Reserved.
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
// Disable /W4 compiler warning C4201: nameless struct/union
#pragma warning(push)
#pragma warning(disable:4201)  // Disable C4201: nameless struct/union
  
#pragma comment(lib,"uuid.lib")
//
// Declarations for ActiveX Scripting profiling.
//

const HRESULT ACTIVPROF_E_PROFILER_PRESENT = MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0x0200);
const HRESULT ACTIVPROF_E_PROFILER_ABSENT = MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0x0201);
const HRESULT ACTIVPROF_E_UNABLE_TO_APPLY_ACTION = MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0x0202);
const ULONG PROFILER_HEAP_OBJECT_NAME_ID_UNAVAILABLE=(ULONG)-1;

#ifndef __ActivProf_h
#define __ActivProf_h

/* GUIDs
 ********/

#ifndef _NO_SCRIPT_GUIDS
// {32E4694E-0D37-419B-B93D-FA20DED6E8EA}
DEFINE_GUID(IID_IActiveScriptProfilerHeapEnum, 0x32e4694e, 0xd37, 0x419b, 0xb9, 0x3d, 0xfa, 0x20, 0xde, 0xd6, 0xe8, 0xea);
// {0B403015-F381-4023-A5D0-6FED076DE716}
DEFINE_GUID(IID_IActiveScriptProfilerControl3, 0xb403015, 0xf381, 0x4023, 0xa5, 0xd0, 0x6f, 0xed, 0x7, 0x6d, 0xe7, 0x16);
#endif // _NO_SCRIPT_GUIDS

typedef /* [public][public][public][public] */ 
enum __MIDL___MIDL_itf_activprof_0000_0000_0001
    {
        PROFILER_SCRIPT_TYPE_USER	= 0,
        PROFILER_SCRIPT_TYPE_DYNAMIC	= ( PROFILER_SCRIPT_TYPE_USER + 1 ) ,
        PROFILER_SCRIPT_TYPE_NATIVE	= ( PROFILER_SCRIPT_TYPE_DYNAMIC + 1 ) ,
        PROFILER_SCRIPT_TYPE_DOM	= ( PROFILER_SCRIPT_TYPE_NATIVE + 1 ) 
    } 	PROFILER_SCRIPT_TYPE;

typedef /* [public] */ 
enum __MIDL___MIDL_itf_activprof_0000_0000_0002
    {
        PROFILER_EVENT_MASK_TRACE_SCRIPT_FUNCTION_CALL	= 0x1,
        PROFILER_EVENT_MASK_TRACE_NATIVE_FUNCTION_CALL	= 0x2,
        PROFILER_EVENT_MASK_TRACE_DOM_FUNCTION_CALL	= 0x4,
        PROFILER_EVENT_MASK_TRACE_ALL	= ( PROFILER_EVENT_MASK_TRACE_SCRIPT_FUNCTION_CALL | PROFILER_EVENT_MASK_TRACE_NATIVE_FUNCTION_CALL ) ,
        PROFILER_EVENT_MASK_TRACE_ALL_WITH_DOM	= ( PROFILER_EVENT_MASK_TRACE_ALL | PROFILER_EVENT_MASK_TRACE_DOM_FUNCTION_CALL ) 
    } 	PROFILER_EVENT_MASK;

typedef LONG PROFILER_TOKEN;



extern RPC_IF_HANDLE __MIDL_itf_activprof_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_activprof_0000_0000_v0_0_s_ifspec;

#ifndef __IActiveScriptProfilerControl_INTERFACE_DEFINED__
#define __IActiveScriptProfilerControl_INTERFACE_DEFINED__

/* interface IActiveScriptProfilerControl */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IActiveScriptProfilerControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("784b5ff0-69b0-47d1-a7dc-2518f4230e90")
    IActiveScriptProfilerControl : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE StartProfiling( 
            /* [in] */ __RPC__in REFCLSID clsidProfilerObject,
            /* [in] */ DWORD dwEventMask,
            /* [in] */ DWORD dwContext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetProfilerEventMask( 
            /* [in] */ DWORD dwEventMask) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE StopProfiling( 
            /* [in] */ HRESULT hrShutdownReason) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IActiveScriptProfilerControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IActiveScriptProfilerControl * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IActiveScriptProfilerControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IActiveScriptProfilerControl * This);
        
        DECLSPEC_XFGVIRT(IActiveScriptProfilerControl, StartProfiling)
        HRESULT ( STDMETHODCALLTYPE *StartProfiling )( 
            __RPC__in IActiveScriptProfilerControl * This,
            /* [in] */ __RPC__in REFCLSID clsidProfilerObject,
            /* [in] */ DWORD dwEventMask,
            /* [in] */ DWORD dwContext);
        
        DECLSPEC_XFGVIRT(IActiveScriptProfilerControl, SetProfilerEventMask)
        HRESULT ( STDMETHODCALLTYPE *SetProfilerEventMask )( 
            __RPC__in IActiveScriptProfilerControl * This,
            /* [in] */ DWORD dwEventMask);
        
        DECLSPEC_XFGVIRT(IActiveScriptProfilerControl, StopProfiling)
        HRESULT ( STDMETHODCALLTYPE *StopProfiling )( 
            __RPC__in IActiveScriptProfilerControl * This,
            /* [in] */ HRESULT hrShutdownReason);
        
        END_INTERFACE
    } IActiveScriptProfilerControlVtbl;

    interface IActiveScriptProfilerControl
    {
        CONST_VTBL struct IActiveScriptProfilerControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IActiveScriptProfilerControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IActiveScriptProfilerControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IActiveScriptProfilerControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IActiveScriptProfilerControl_StartProfiling(This,clsidProfilerObject,dwEventMask,dwContext)	\
    ( (This)->lpVtbl -> StartProfiling(This,clsidProfilerObject,dwEventMask,dwContext) ) 

#define IActiveScriptProfilerControl_SetProfilerEventMask(This,dwEventMask)	\
    ( (This)->lpVtbl -> SetProfilerEventMask(This,dwEventMask) ) 

#define IActiveScriptProfilerControl_StopProfiling(This,hrShutdownReason)	\
    ( (This)->lpVtbl -> StopProfiling(This,hrShutdownReason) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IActiveScriptProfilerControl_INTERFACE_DEFINED__ */


#ifndef __IActiveScriptProfilerControl2_INTERFACE_DEFINED__
#define __IActiveScriptProfilerControl2_INTERFACE_DEFINED__

/* interface IActiveScriptProfilerControl2 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IActiveScriptProfilerControl2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("47810165-498F-40be-94F1-653557E9E7DA")
    IActiveScriptProfilerControl2 : public IActiveScriptProfilerControl
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CompleteProfilerStart( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE PrepareProfilerStop( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IActiveScriptProfilerControl2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IActiveScriptProfilerControl2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IActiveScriptProfilerControl2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IActiveScriptProfilerControl2 * This);
        
        DECLSPEC_XFGVIRT(IActiveScriptProfilerControl, StartProfiling)
        HRESULT ( STDMETHODCALLTYPE *StartProfiling )( 
            __RPC__in IActiveScriptProfilerControl2 * This,
            /* [in] */ __RPC__in REFCLSID clsidProfilerObject,
            /* [in] */ DWORD dwEventMask,
            /* [in] */ DWORD dwContext);
        
        DECLSPEC_XFGVIRT(IActiveScriptProfilerControl, SetProfilerEventMask)
        HRESULT ( STDMETHODCALLTYPE *SetProfilerEventMask )( 
            __RPC__in IActiveScriptProfilerControl2 * This,
            /* [in] */ DWORD dwEventMask);
        
        DECLSPEC_XFGVIRT(IActiveScriptProfilerControl, StopProfiling)
        HRESULT ( STDMETHODCALLTYPE *StopProfiling )( 
            __RPC__in IActiveScriptProfilerControl2 * This,
            /* [in] */ HRESULT hrShutdownReason);
        
        DECLSPEC_XFGVIRT(IActiveScriptProfilerControl2, CompleteProfilerStart)
        HRESULT ( STDMETHODCALLTYPE *CompleteProfilerStart )( 
            __RPC__in IActiveScriptProfilerControl2 * This);
        
        DECLSPEC_XFGVIRT(IActiveScriptProfilerControl2, PrepareProfilerStop)
        HRESULT ( STDMETHODCALLTYPE *PrepareProfilerStop )( 
            __RPC__in IActiveScriptProfilerControl2 * This);
        
        END_INTERFACE
    } IActiveScriptProfilerControl2Vtbl;

    interface IActiveScriptProfilerControl2
    {
        CONST_VTBL struct IActiveScriptProfilerControl2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IActiveScriptProfilerControl2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IActiveScriptProfilerControl2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IActiveScriptProfilerControl2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IActiveScriptProfilerControl2_StartProfiling(This,clsidProfilerObject,dwEventMask,dwContext)	\
    ( (This)->lpVtbl -> StartProfiling(This,clsidProfilerObject,dwEventMask,dwContext) ) 

#define IActiveScriptProfilerControl2_SetProfilerEventMask(This,dwEventMask)	\
    ( (This)->lpVtbl -> SetProfilerEventMask(This,dwEventMask) ) 

#define IActiveScriptProfilerControl2_StopProfiling(This,hrShutdownReason)	\
    ( (This)->lpVtbl -> StopProfiling(This,hrShutdownReason) ) 


#define IActiveScriptProfilerControl2_CompleteProfilerStart(This)	\
    ( (This)->lpVtbl -> CompleteProfilerStart(This) ) 

#define IActiveScriptProfilerControl2_PrepareProfilerStop(This)	\
    ( (This)->lpVtbl -> PrepareProfilerStop(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IActiveScriptProfilerControl2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_activprof_0000_0002 */
/* [local] */ 

typedef DWORD_PTR PROFILER_HEAP_OBJECT_ID;

typedef UINT PROFILER_HEAP_OBJECT_NAME_ID;

typedef void *PROFILER_EXTERNAL_OBJECT_ADDRESS;

typedef /* [public][v1_enum] */ 
enum __MIDL___MIDL_itf_activprof_0000_0002_0001
    {
        PROFILER_HEAP_OBJECT_FLAGS_NEW_OBJECT	= 0x1,
        PROFILER_HEAP_OBJECT_FLAGS_IS_ROOT	= 0x2,
        PROFILER_HEAP_OBJECT_FLAGS_SITE_CLOSED	= 0x4,
        PROFILER_HEAP_OBJECT_FLAGS_EXTERNAL	= 0x8,
        PROFILER_HEAP_OBJECT_FLAGS_EXTERNAL_UNKNOWN	= 0x10,
        PROFILER_HEAP_OBJECT_FLAGS_EXTERNAL_DISPATCH	= 0x20,
        PROFILER_HEAP_OBJECT_FLAGS_SIZE_APPROXIMATE	= 0x40,
        PROFILER_HEAP_OBJECT_FLAGS_SIZE_UNAVAILABLE	= 0x80,
        PROFILER_HEAP_OBJECT_FLAGS_NEW_STATE_UNAVAILABLE	= 0x100,
        PROFILER_HEAP_OBJECT_FLAGS_WINRT_INSTANCE	= 0x200,
        PROFILER_HEAP_OBJECT_FLAGS_WINRT_RUNTIMECLASS	= 0x400,
        PROFILER_HEAP_OBJECT_FLAGS_WINRT_DELEGATE	= 0x800,
        PROFILER_HEAP_OBJECT_FLAGS_WINRT_NAMESPACE	= 0x1000
    } 	PROFILER_HEAP_OBJECT_FLAGS;

typedef /* [public][public][public][v1_enum] */ 
enum __MIDL___MIDL_itf_activprof_0000_0002_0002
    {
        PROFILER_HEAP_OBJECT_OPTIONAL_INFO_PROTOTYPE	= 0x1,
        PROFILER_HEAP_OBJECT_OPTIONAL_INFO_FUNCTION_NAME	= 0x2,
        PROFILER_HEAP_OBJECT_OPTIONAL_INFO_SCOPE_LIST	= 0x3,
        PROFILER_HEAP_OBJECT_OPTIONAL_INFO_INTERNAL_PROPERTY	= 0x4,
        PROFILER_HEAP_OBJECT_OPTIONAL_INFO_NAME_PROPERTIES	= 0x5,
        PROFILER_HEAP_OBJECT_OPTIONAL_INFO_INDEX_PROPERTIES	= 0x6,
        PROFILER_HEAP_OBJECT_OPTIONAL_INFO_ELEMENT_ATTRIBUTES_SIZE	= 0x7,
        PROFILER_HEAP_OBJECT_OPTIONAL_INFO_ELEMENT_TEXT_CHILDREN_SIZE	= 0x8,
        PROFILER_HEAP_OBJECT_OPTIONAL_INFO_RELATIONSHIPS	= 0x9,
        PROFILER_HEAP_OBJECT_OPTIONAL_INFO_WINRTEVENTS	= 0xa,
        PROFILER_HEAP_OBJECT_OPTIONAL_INFO_WEAKMAP_COLLECTION_LIST	= 0xb,
        PROFILER_HEAP_OBJECT_OPTIONAL_INFO_MAP_COLLECTION_LIST	= 0xc,
        PROFILER_HEAP_OBJECT_OPTIONAL_INFO_SET_COLLECTION_LIST	= 0xd,
        PROFILER_HEAP_OBJECT_OPTIONAL_INFO_MAX_VALUE	= PROFILER_HEAP_OBJECT_OPTIONAL_INFO_SET_COLLECTION_LIST
    } 	PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE;

typedef /* [public][v1_enum] */ 
enum __MIDL___MIDL_itf_activprof_0000_0002_0003
    {
        PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS_NONE	= 0,
        PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS_IS_GET_ACCESSOR	= 0x10000,
        PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS_IS_SET_ACCESSOR	= 0x20000,
        PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS_LET_VARIABLE	= 0x40000,
        PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS_CONST_VARIABLE	= 0x80000
    } 	PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS;

typedef /* [public][public][v1_enum] */ 
enum __MIDL___MIDL_itf_activprof_0000_0002_0004
    {
        PROFILER_HEAP_ENUM_FLAGS_NONE	= 0,
        PROFILER_HEAP_ENUM_FLAGS_STORE_RELATIONSHIP_FLAGS	= 0x1,
        PROFILER_HEAP_ENUM_FLAGS_SUBSTRINGS	= 0x2,
        PROFILER_HEAP_ENUM_FLAGS_RELATIONSHIP_SUBSTRINGS	= ( PROFILER_HEAP_ENUM_FLAGS_STORE_RELATIONSHIP_FLAGS | PROFILER_HEAP_ENUM_FLAGS_SUBSTRINGS ) 
    } 	PROFILER_HEAP_ENUM_FLAGS;

typedef struct _PROFILER_HEAP_OBJECT_SCOPE_LIST
    {
    UINT count;
    /* [size_is] */ PROFILER_HEAP_OBJECT_ID scopes[ 1 ];
    } 	PROFILER_HEAP_OBJECT_SCOPE_LIST;

typedef /* [public][public][public][public][public][public][public][v1_enum] */ 
enum __MIDL___MIDL_itf_activprof_0000_0002_0005
    {
        PROFILER_PROPERTY_TYPE_NUMBER	= 0x1,
        PROFILER_PROPERTY_TYPE_STRING	= 0x2,
        PROFILER_PROPERTY_TYPE_HEAP_OBJECT	= 0x3,
        PROFILER_PROPERTY_TYPE_EXTERNAL_OBJECT	= 0x4,
        PROFILER_PROPERTY_TYPE_BSTR	= 0x5,
        PROFILER_PROPERTY_TYPE_SUBSTRING	= 0x6
    } 	PROFILER_RELATIONSHIP_INFO;

typedef struct _PROFILER_PROPERTY_TYPE_SUBSTRING_INFO
    {
    UINT length;
    LPCWSTR value;
    } 	PROFILER_PROPERTY_TYPE_SUBSTRING_INFO;

typedef struct _PROFILER_HEAP_OBJECT_RELATIONSHIP
    {
    PROFILER_HEAP_OBJECT_NAME_ID relationshipId;
    PROFILER_RELATIONSHIP_INFO relationshipInfo;
    /* [switch_is][switch_type] */ union 
        {
        /* [case()] */ double numberValue;
        /* [case()] */ LPCWSTR stringValue;
        /* [case()] */ BSTR bstrValue;
        /* [case()] */ PROFILER_HEAP_OBJECT_ID objectId;
        /* [case()] */ PROFILER_EXTERNAL_OBJECT_ADDRESS externalObjectAddress;
        /* [case()] */ PROFILER_PROPERTY_TYPE_SUBSTRING_INFO *subString;
        } 	;
    } 	PROFILER_HEAP_OBJECT_RELATIONSHIP;

typedef struct _PROFILER_HEAP_OBJECT_RELATIONSHIP_LIST
    {
    UINT count;
    /* [size_is] */ PROFILER_HEAP_OBJECT_RELATIONSHIP elements[ 1 ];
    } 	PROFILER_HEAP_OBJECT_RELATIONSHIP_LIST;

typedef struct _PROFILER_HEAP_OBJECT_OPTIONAL_INFO
    {
    PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE infoType;
    /* [switch_is][switch_type] */ union 
        {
        /* [case()] */ PROFILER_HEAP_OBJECT_ID prototype;
        /* [case()] */ LPCWSTR functionName;
        /* [case()] */ UINT elementAttributesSize;
        /* [case()] */ UINT elementTextChildrenSize;
        /* [case()] */ PROFILER_HEAP_OBJECT_SCOPE_LIST *scopeList;
        /* [case()] */ PROFILER_HEAP_OBJECT_RELATIONSHIP *internalProperty;
        /* [case()] */ PROFILER_HEAP_OBJECT_RELATIONSHIP_LIST *namePropertyList;
        /* [case()] */ PROFILER_HEAP_OBJECT_RELATIONSHIP_LIST *indexPropertyList;
        /* [case()] */ PROFILER_HEAP_OBJECT_RELATIONSHIP_LIST *relationshipList;
        /* [case()] */ PROFILER_HEAP_OBJECT_RELATIONSHIP_LIST *eventList;
        /* [case()] */ PROFILER_HEAP_OBJECT_RELATIONSHIP_LIST *weakMapCollectionList;
        /* [case()] */ PROFILER_HEAP_OBJECT_RELATIONSHIP_LIST *mapCollectionList;
        /* [case()] */ PROFILER_HEAP_OBJECT_RELATIONSHIP_LIST *setCollectionList;
        } 	;
    } 	PROFILER_HEAP_OBJECT_OPTIONAL_INFO;

typedef struct _PROFILER_HEAP_OBJECT
    {
    UINT size;
    union 
        {
        PROFILER_HEAP_OBJECT_ID objectId;
        PROFILER_EXTERNAL_OBJECT_ADDRESS externalObjectAddress;
        } 	;
    PROFILER_HEAP_OBJECT_NAME_ID typeNameId;
    ULONG flags;
    USHORT unused;
    USHORT optionalInfoCount;
    } 	PROFILER_HEAP_OBJECT;



extern RPC_IF_HANDLE __MIDL_itf_activprof_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_activprof_0000_0002_v0_0_s_ifspec;

#ifndef __IActiveScriptProfilerHeapEnum_INTERFACE_DEFINED__
#define __IActiveScriptProfilerHeapEnum_INTERFACE_DEFINED__

/* interface IActiveScriptProfilerHeapEnum */
/* [unique][local][uuid][object] */ 


EXTERN_C const IID IID_IActiveScriptProfilerHeapEnum;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("32E4694E-0D37-419B-B93D-FA20DED6E8EA")
    IActiveScriptProfilerHeapEnum : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG celt,
            /* [length_is][size_is][out] */ PROFILER_HEAP_OBJECT **heapObjects,
            /* [out] */ ULONG *pceltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOptionalInfo( 
            /* [in] */ PROFILER_HEAP_OBJECT *heapObject,
            /* [in] */ ULONG celt,
            /* [size_is][out] */ PROFILER_HEAP_OBJECT_OPTIONAL_INFO *optionalInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FreeObjectAndOptionalInfo( 
            /* [in] */ ULONG celt,
            /* [size_is][in] */ PROFILER_HEAP_OBJECT **heapObjects) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNameIdMap( 
            /* [size_is][size_is][out] */ LPCWSTR *pNameList[  ],
            /* [out] */ UINT *pcelt) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IActiveScriptProfilerHeapEnumVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IActiveScriptProfilerHeapEnum * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IActiveScriptProfilerHeapEnum * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IActiveScriptProfilerHeapEnum * This);
        
        DECLSPEC_XFGVIRT(IActiveScriptProfilerHeapEnum, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            IActiveScriptProfilerHeapEnum * This,
            /* [in] */ ULONG celt,
            /* [length_is][size_is][out] */ PROFILER_HEAP_OBJECT **heapObjects,
            /* [out] */ ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IActiveScriptProfilerHeapEnum, GetOptionalInfo)
        HRESULT ( STDMETHODCALLTYPE *GetOptionalInfo )( 
            IActiveScriptProfilerHeapEnum * This,
            /* [in] */ PROFILER_HEAP_OBJECT *heapObject,
            /* [in] */ ULONG celt,
            /* [size_is][out] */ PROFILER_HEAP_OBJECT_OPTIONAL_INFO *optionalInfo);
        
        DECLSPEC_XFGVIRT(IActiveScriptProfilerHeapEnum, FreeObjectAndOptionalInfo)
        HRESULT ( STDMETHODCALLTYPE *FreeObjectAndOptionalInfo )( 
            IActiveScriptProfilerHeapEnum * This,
            /* [in] */ ULONG celt,
            /* [size_is][in] */ PROFILER_HEAP_OBJECT **heapObjects);
        
        DECLSPEC_XFGVIRT(IActiveScriptProfilerHeapEnum, GetNameIdMap)
        HRESULT ( STDMETHODCALLTYPE *GetNameIdMap )( 
            IActiveScriptProfilerHeapEnum * This,
            /* [size_is][size_is][out] */ LPCWSTR *pNameList[  ],
            /* [out] */ UINT *pcelt);
        
        END_INTERFACE
    } IActiveScriptProfilerHeapEnumVtbl;

    interface IActiveScriptProfilerHeapEnum
    {
        CONST_VTBL struct IActiveScriptProfilerHeapEnumVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IActiveScriptProfilerHeapEnum_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IActiveScriptProfilerHeapEnum_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IActiveScriptProfilerHeapEnum_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IActiveScriptProfilerHeapEnum_Next(This,celt,heapObjects,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,heapObjects,pceltFetched) ) 

#define IActiveScriptProfilerHeapEnum_GetOptionalInfo(This,heapObject,celt,optionalInfo)	\
    ( (This)->lpVtbl -> GetOptionalInfo(This,heapObject,celt,optionalInfo) ) 

#define IActiveScriptProfilerHeapEnum_FreeObjectAndOptionalInfo(This,celt,heapObjects)	\
    ( (This)->lpVtbl -> FreeObjectAndOptionalInfo(This,celt,heapObjects) ) 

#define IActiveScriptProfilerHeapEnum_GetNameIdMap(This,pNameList,pcelt)	\
    ( (This)->lpVtbl -> GetNameIdMap(This,pNameList,pcelt) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IActiveScriptProfilerHeapEnum_INTERFACE_DEFINED__ */


#ifndef __IActiveScriptProfilerControl3_INTERFACE_DEFINED__
#define __IActiveScriptProfilerControl3_INTERFACE_DEFINED__

/* interface IActiveScriptProfilerControl3 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IActiveScriptProfilerControl3;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0B403015-F381-4023-A5D0-6FED076DE716")
    IActiveScriptProfilerControl3 : public IActiveScriptProfilerControl2
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE EnumHeap( 
            /* [out] */ __RPC__deref_out_opt IActiveScriptProfilerHeapEnum **ppEnum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IActiveScriptProfilerControl3Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IActiveScriptProfilerControl3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IActiveScriptProfilerControl3 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IActiveScriptProfilerControl3 * This);
        
        DECLSPEC_XFGVIRT(IActiveScriptProfilerControl, StartProfiling)
        HRESULT ( STDMETHODCALLTYPE *StartProfiling )( 
            __RPC__in IActiveScriptProfilerControl3 * This,
            /* [in] */ __RPC__in REFCLSID clsidProfilerObject,
            /* [in] */ DWORD dwEventMask,
            /* [in] */ DWORD dwContext);
        
        DECLSPEC_XFGVIRT(IActiveScriptProfilerControl, SetProfilerEventMask)
        HRESULT ( STDMETHODCALLTYPE *SetProfilerEventMask )( 
            __RPC__in IActiveScriptProfilerControl3 * This,
            /* [in] */ DWORD dwEventMask);
        
        DECLSPEC_XFGVIRT(IActiveScriptProfilerControl, StopProfiling)
        HRESULT ( STDMETHODCALLTYPE *StopProfiling )( 
            __RPC__in IActiveScriptProfilerControl3 * This,
            /* [in] */ HRESULT hrShutdownReason);
        
        DECLSPEC_XFGVIRT(IActiveScriptProfilerControl2, CompleteProfilerStart)
        HRESULT ( STDMETHODCALLTYPE *CompleteProfilerStart )( 
            __RPC__in IActiveScriptProfilerControl3 * This);
        
        DECLSPEC_XFGVIRT(IActiveScriptProfilerControl2, PrepareProfilerStop)
        HRESULT ( STDMETHODCALLTYPE *PrepareProfilerStop )( 
            __RPC__in IActiveScriptProfilerControl3 * This);
        
        DECLSPEC_XFGVIRT(IActiveScriptProfilerControl3, EnumHeap)
        HRESULT ( STDMETHODCALLTYPE *EnumHeap )( 
            __RPC__in IActiveScriptProfilerControl3 * This,
            /* [out] */ __RPC__deref_out_opt IActiveScriptProfilerHeapEnum **ppEnum);
        
        END_INTERFACE
    } IActiveScriptProfilerControl3Vtbl;

    interface IActiveScriptProfilerControl3
    {
        CONST_VTBL struct IActiveScriptProfilerControl3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IActiveScriptProfilerControl3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IActiveScriptProfilerControl3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IActiveScriptProfilerControl3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IActiveScriptProfilerControl3_StartProfiling(This,clsidProfilerObject,dwEventMask,dwContext)	\
    ( (This)->lpVtbl -> StartProfiling(This,clsidProfilerObject,dwEventMask,dwContext) ) 

#define IActiveScriptProfilerControl3_SetProfilerEventMask(This,dwEventMask)	\
    ( (This)->lpVtbl -> SetProfilerEventMask(This,dwEventMask) ) 

#define IActiveScriptProfilerControl3_StopProfiling(This,hrShutdownReason)	\
    ( (This)->lpVtbl -> StopProfiling(This,hrShutdownReason) ) 


#define IActiveScriptProfilerControl3_CompleteProfilerStart(This)	\
    ( (This)->lpVtbl -> CompleteProfilerStart(This) ) 

#define IActiveScriptProfilerControl3_PrepareProfilerStop(This)	\
    ( (This)->lpVtbl -> PrepareProfilerStop(This) ) 


#define IActiveScriptProfilerControl3_EnumHeap(This,ppEnum)	\
    ( (This)->lpVtbl -> EnumHeap(This,ppEnum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IActiveScriptProfilerControl3_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_activprof_0000_0004 */
/* [local] */ 

typedef /* [public][public][public][v1_enum] */ 
enum __MIDL___MIDL_itf_activprof_0000_0004_0001
    {
        PROFILER_HEAP_SUMMARY_VERSION_1	= 0x1
    } 	PROFILER_HEAP_SUMMARY_VERSION;

typedef struct _PROFILER_HEAP_SUMMARY
    {
    PROFILER_HEAP_SUMMARY_VERSION version;
    UINT totalHeapSize;
    } 	PROFILER_HEAP_SUMMARY;



extern RPC_IF_HANDLE __MIDL_itf_activprof_0000_0004_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_activprof_0000_0004_v0_0_s_ifspec;

#ifndef __IActiveScriptProfilerControl4_INTERFACE_DEFINED__
#define __IActiveScriptProfilerControl4_INTERFACE_DEFINED__

/* interface IActiveScriptProfilerControl4 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IActiveScriptProfilerControl4;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("160F94FD-9DBC-40D4-9EAC-2B71DB3132F4")
    IActiveScriptProfilerControl4 : public IActiveScriptProfilerControl3
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SummarizeHeap( 
            /* [out][in] */ __RPC__inout PROFILER_HEAP_SUMMARY *heapSummary) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IActiveScriptProfilerControl4Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IActiveScriptProfilerControl4 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IActiveScriptProfilerControl4 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IActiveScriptProfilerControl4 * This);
        
        DECLSPEC_XFGVIRT(IActiveScriptProfilerControl, StartProfiling)
        HRESULT ( STDMETHODCALLTYPE *StartProfiling )( 
            __RPC__in IActiveScriptProfilerControl4 * This,
            /* [in] */ __RPC__in REFCLSID clsidProfilerObject,
            /* [in] */ DWORD dwEventMask,
            /* [in] */ DWORD dwContext);
        
        DECLSPEC_XFGVIRT(IActiveScriptProfilerControl, SetProfilerEventMask)
        HRESULT ( STDMETHODCALLTYPE *SetProfilerEventMask )( 
            __RPC__in IActiveScriptProfilerControl4 * This,
            /* [in] */ DWORD dwEventMask);
        
        DECLSPEC_XFGVIRT(IActiveScriptProfilerControl, StopProfiling)
        HRESULT ( STDMETHODCALLTYPE *StopProfiling )( 
            __RPC__in IActiveScriptProfilerControl4 * This,
            /* [in] */ HRESULT hrShutdownReason);
        
        DECLSPEC_XFGVIRT(IActiveScriptProfilerControl2, CompleteProfilerStart)
        HRESULT ( STDMETHODCALLTYPE *CompleteProfilerStart )( 
            __RPC__in IActiveScriptProfilerControl4 * This);
        
        DECLSPEC_XFGVIRT(IActiveScriptProfilerControl2, PrepareProfilerStop)
        HRESULT ( STDMETHODCALLTYPE *PrepareProfilerStop )( 
            __RPC__in IActiveScriptProfilerControl4 * This);
        
        DECLSPEC_XFGVIRT(IActiveScriptProfilerControl3, EnumHeap)
        HRESULT ( STDMETHODCALLTYPE *EnumHeap )( 
            __RPC__in IActiveScriptProfilerControl4 * This,
            /* [out] */ __RPC__deref_out_opt IActiveScriptProfilerHeapEnum **ppEnum);
        
        DECLSPEC_XFGVIRT(IActiveScriptProfilerControl4, SummarizeHeap)
        HRESULT ( STDMETHODCALLTYPE *SummarizeHeap )( 
            __RPC__in IActiveScriptProfilerControl4 * This,
            /* [out][in] */ __RPC__inout PROFILER_HEAP_SUMMARY *heapSummary);
        
        END_INTERFACE
    } IActiveScriptProfilerControl4Vtbl;

    interface IActiveScriptProfilerControl4
    {
        CONST_VTBL struct IActiveScriptProfilerControl4Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IActiveScriptProfilerControl4_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IActiveScriptProfilerControl4_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IActiveScriptProfilerControl4_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IActiveScriptProfilerControl4_StartProfiling(This,clsidProfilerObject,dwEventMask,dwContext)	\
    ( (This)->lpVtbl -> StartProfiling(This,clsidProfilerObject,dwEventMask,dwContext) ) 

#define IActiveScriptProfilerControl4_SetProfilerEventMask(This,dwEventMask)	\
    ( (This)->lpVtbl -> SetProfilerEventMask(This,dwEventMask) ) 

#define IActiveScriptProfilerControl4_StopProfiling(This,hrShutdownReason)	\
    ( (This)->lpVtbl -> StopProfiling(This,hrShutdownReason) ) 


#define IActiveScriptProfilerControl4_CompleteProfilerStart(This)	\
    ( (This)->lpVtbl -> CompleteProfilerStart(This) ) 

#define IActiveScriptProfilerControl4_PrepareProfilerStop(This)	\
    ( (This)->lpVtbl -> PrepareProfilerStop(This) ) 


#define IActiveScriptProfilerControl4_EnumHeap(This,ppEnum)	\
    ( (This)->lpVtbl -> EnumHeap(This,ppEnum) ) 


#define IActiveScriptProfilerControl4_SummarizeHeap(This,heapSummary)	\
    ( (This)->lpVtbl -> SummarizeHeap(This,heapSummary) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IActiveScriptProfilerControl4_INTERFACE_DEFINED__ */


#ifndef __IActiveScriptProfilerControl5_INTERFACE_DEFINED__
#define __IActiveScriptProfilerControl5_INTERFACE_DEFINED__

/* interface IActiveScriptProfilerControl5 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IActiveScriptProfilerControl5;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1C01A2D1-8F0F-46A5-9720-0D7ED2C62F0A")
    IActiveScriptProfilerControl5 : public IActiveScriptProfilerControl4
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE EnumHeap2( 
            /* [in] */ PROFILER_HEAP_ENUM_FLAGS enumFlags,
            /* [out] */ __RPC__deref_out_opt IActiveScriptProfilerHeapEnum **ppEnum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IActiveScriptProfilerControl5Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IActiveScriptProfilerControl5 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IActiveScriptProfilerControl5 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IActiveScriptProfilerControl5 * This);
        
        DECLSPEC_XFGVIRT(IActiveScriptProfilerControl, StartProfiling)
        HRESULT ( STDMETHODCALLTYPE *StartProfiling )( 
            __RPC__in IActiveScriptProfilerControl5 * This,
            /* [in] */ __RPC__in REFCLSID clsidProfilerObject,
            /* [in] */ DWORD dwEventMask,
            /* [in] */ DWORD dwContext);
        
        DECLSPEC_XFGVIRT(IActiveScriptProfilerControl, SetProfilerEventMask)
        HRESULT ( STDMETHODCALLTYPE *SetProfilerEventMask )( 
            __RPC__in IActiveScriptProfilerControl5 * This,
            /* [in] */ DWORD dwEventMask);
        
        DECLSPEC_XFGVIRT(IActiveScriptProfilerControl, StopProfiling)
        HRESULT ( STDMETHODCALLTYPE *StopProfiling )( 
            __RPC__in IActiveScriptProfilerControl5 * This,
            /* [in] */ HRESULT hrShutdownReason);
        
        DECLSPEC_XFGVIRT(IActiveScriptProfilerControl2, CompleteProfilerStart)
        HRESULT ( STDMETHODCALLTYPE *CompleteProfilerStart )( 
            __RPC__in IActiveScriptProfilerControl5 * This);
        
        DECLSPEC_XFGVIRT(IActiveScriptProfilerControl2, PrepareProfilerStop)
        HRESULT ( STDMETHODCALLTYPE *PrepareProfilerStop )( 
            __RPC__in IActiveScriptProfilerControl5 * This);
        
        DECLSPEC_XFGVIRT(IActiveScriptProfilerControl3, EnumHeap)
        HRESULT ( STDMETHODCALLTYPE *EnumHeap )( 
            __RPC__in IActiveScriptProfilerControl5 * This,
            /* [out] */ __RPC__deref_out_opt IActiveScriptProfilerHeapEnum **ppEnum);
        
        DECLSPEC_XFGVIRT(IActiveScriptProfilerControl4, SummarizeHeap)
        HRESULT ( STDMETHODCALLTYPE *SummarizeHeap )( 
            __RPC__in IActiveScriptProfilerControl5 * This,
            /* [out][in] */ __RPC__inout PROFILER_HEAP_SUMMARY *heapSummary);
        
        DECLSPEC_XFGVIRT(IActiveScriptProfilerControl5, EnumHeap2)
        HRESULT ( STDMETHODCALLTYPE *EnumHeap2 )( 
            __RPC__in IActiveScriptProfilerControl5 * This,
            /* [in] */ PROFILER_HEAP_ENUM_FLAGS enumFlags,
            /* [out] */ __RPC__deref_out_opt IActiveScriptProfilerHeapEnum **ppEnum);
        
        END_INTERFACE
    } IActiveScriptProfilerControl5Vtbl;

    interface IActiveScriptProfilerControl5
    {
        CONST_VTBL struct IActiveScriptProfilerControl5Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IActiveScriptProfilerControl5_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IActiveScriptProfilerControl5_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IActiveScriptProfilerControl5_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IActiveScriptProfilerControl5_StartProfiling(This,clsidProfilerObject,dwEventMask,dwContext)	\
    ( (This)->lpVtbl -> StartProfiling(This,clsidProfilerObject,dwEventMask,dwContext) ) 

#define IActiveScriptProfilerControl5_SetProfilerEventMask(This,dwEventMask)	\
    ( (This)->lpVtbl -> SetProfilerEventMask(This,dwEventMask) ) 

#define IActiveScriptProfilerControl5_StopProfiling(This,hrShutdownReason)	\
    ( (This)->lpVtbl -> StopProfiling(This,hrShutdownReason) ) 


#define IActiveScriptProfilerControl5_CompleteProfilerStart(This)	\
    ( (This)->lpVtbl -> CompleteProfilerStart(This) ) 

#define IActiveScriptProfilerControl5_PrepareProfilerStop(This)	\
    ( (This)->lpVtbl -> PrepareProfilerStop(This) ) 


#define IActiveScriptProfilerControl5_EnumHeap(This,ppEnum)	\
    ( (This)->lpVtbl -> EnumHeap(This,ppEnum) ) 


#define IActiveScriptProfilerControl5_SummarizeHeap(This,heapSummary)	\
    ( (This)->lpVtbl -> SummarizeHeap(This,heapSummary) ) 


#define IActiveScriptProfilerControl5_EnumHeap2(This,enumFlags,ppEnum)	\
    ( (This)->lpVtbl -> EnumHeap2(This,enumFlags,ppEnum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IActiveScriptProfilerControl5_INTERFACE_DEFINED__ */


#ifndef __IActiveScriptProfilerCallback_INTERFACE_DEFINED__
#define __IActiveScriptProfilerCallback_INTERFACE_DEFINED__

/* interface IActiveScriptProfilerCallback */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IActiveScriptProfilerCallback;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("740eca23-7d9d-42e5-ba9d-f8b24b1c7a9b")
    IActiveScriptProfilerCallback : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [in] */ DWORD dwContext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Shutdown( 
            /* [in] */ HRESULT hrReason) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ScriptCompiled( 
            /* [in] */ PROFILER_TOKEN scriptId,
            /* [in] */ PROFILER_SCRIPT_TYPE type,
            /* [in] */ __RPC__in_opt IUnknown *pIDebugDocumentContext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FunctionCompiled( 
            /* [in] */ PROFILER_TOKEN functionId,
            /* [in] */ PROFILER_TOKEN scriptId,
            /* [string][in] */ __RPC__in_string const WCHAR *pwszFunctionName,
            /* [string][in] */ __RPC__in_string const WCHAR *pwszFunctionNameHint,
            /* [in] */ __RPC__in_opt IUnknown *pIDebugDocumentContext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnFunctionEnter( 
            /* [in] */ PROFILER_TOKEN scriptId,
            /* [in] */ PROFILER_TOKEN functionId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnFunctionExit( 
            /* [in] */ PROFILER_TOKEN scriptId,
            /* [in] */ PROFILER_TOKEN functionId) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IActiveScriptProfilerCallbackVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IActiveScriptProfilerCallback * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IActiveScriptProfilerCallback * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IActiveScriptProfilerCallback * This);
        
        DECLSPEC_XFGVIRT(IActiveScriptProfilerCallback, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            __RPC__in IActiveScriptProfilerCallback * This,
            /* [in] */ DWORD dwContext);
        
        DECLSPEC_XFGVIRT(IActiveScriptProfilerCallback, Shutdown)
        HRESULT ( STDMETHODCALLTYPE *Shutdown )( 
            __RPC__in IActiveScriptProfilerCallback * This,
            /* [in] */ HRESULT hrReason);
        
        DECLSPEC_XFGVIRT(IActiveScriptProfilerCallback, ScriptCompiled)
        HRESULT ( STDMETHODCALLTYPE *ScriptCompiled )( 
            __RPC__in IActiveScriptProfilerCallback * This,
            /* [in] */ PROFILER_TOKEN scriptId,
            /* [in] */ PROFILER_SCRIPT_TYPE type,
            /* [in] */ __RPC__in_opt IUnknown *pIDebugDocumentContext);
        
        DECLSPEC_XFGVIRT(IActiveScriptProfilerCallback, FunctionCompiled)
        HRESULT ( STDMETHODCALLTYPE *FunctionCompiled )( 
            __RPC__in IActiveScriptProfilerCallback * This,
            /* [in] */ PROFILER_TOKEN functionId,
            /* [in] */ PROFILER_TOKEN scriptId,
            /* [string][in] */ __RPC__in_string const WCHAR *pwszFunctionName,
            /* [string][in] */ __RPC__in_string const WCHAR *pwszFunctionNameHint,
            /* [in] */ __RPC__in_opt IUnknown *pIDebugDocumentContext);
        
        DECLSPEC_XFGVIRT(IActiveScriptProfilerCallback, OnFunctionEnter)
        HRESULT ( STDMETHODCALLTYPE *OnFunctionEnter )( 
            __RPC__in IActiveScriptProfilerCallback * This,
            /* [in] */ PROFILER_TOKEN scriptId,
            /* [in] */ PROFILER_TOKEN functionId);
        
        DECLSPEC_XFGVIRT(IActiveScriptProfilerCallback, OnFunctionExit)
        HRESULT ( STDMETHODCALLTYPE *OnFunctionExit )( 
            __RPC__in IActiveScriptProfilerCallback * This,
            /* [in] */ PROFILER_TOKEN scriptId,
            /* [in] */ PROFILER_TOKEN functionId);
        
        END_INTERFACE
    } IActiveScriptProfilerCallbackVtbl;

    interface IActiveScriptProfilerCallback
    {
        CONST_VTBL struct IActiveScriptProfilerCallbackVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IActiveScriptProfilerCallback_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IActiveScriptProfilerCallback_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IActiveScriptProfilerCallback_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IActiveScriptProfilerCallback_Initialize(This,dwContext)	\
    ( (This)->lpVtbl -> Initialize(This,dwContext) ) 

#define IActiveScriptProfilerCallback_Shutdown(This,hrReason)	\
    ( (This)->lpVtbl -> Shutdown(This,hrReason) ) 

#define IActiveScriptProfilerCallback_ScriptCompiled(This,scriptId,type,pIDebugDocumentContext)	\
    ( (This)->lpVtbl -> ScriptCompiled(This,scriptId,type,pIDebugDocumentContext) ) 

#define IActiveScriptProfilerCallback_FunctionCompiled(This,functionId,scriptId,pwszFunctionName,pwszFunctionNameHint,pIDebugDocumentContext)	\
    ( (This)->lpVtbl -> FunctionCompiled(This,functionId,scriptId,pwszFunctionName,pwszFunctionNameHint,pIDebugDocumentContext) ) 

#define IActiveScriptProfilerCallback_OnFunctionEnter(This,scriptId,functionId)	\
    ( (This)->lpVtbl -> OnFunctionEnter(This,scriptId,functionId) ) 

#define IActiveScriptProfilerCallback_OnFunctionExit(This,scriptId,functionId)	\
    ( (This)->lpVtbl -> OnFunctionExit(This,scriptId,functionId) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IActiveScriptProfilerCallback_INTERFACE_DEFINED__ */


#ifndef __IActiveScriptProfilerCallback2_INTERFACE_DEFINED__
#define __IActiveScriptProfilerCallback2_INTERFACE_DEFINED__

/* interface IActiveScriptProfilerCallback2 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IActiveScriptProfilerCallback2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("31B7F8AD-A637-409C-B22F-040995B6103D")
    IActiveScriptProfilerCallback2 : public IActiveScriptProfilerCallback
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnFunctionEnterByName( 
            /* [string][in] */ __RPC__in_string const WCHAR *pwszFunctionName,
            /* [in] */ PROFILER_SCRIPT_TYPE type) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnFunctionExitByName( 
            /* [string][in] */ __RPC__in_string const WCHAR *pwszFunctionName,
            /* [in] */ PROFILER_SCRIPT_TYPE type) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IActiveScriptProfilerCallback2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IActiveScriptProfilerCallback2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IActiveScriptProfilerCallback2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IActiveScriptProfilerCallback2 * This);
        
        DECLSPEC_XFGVIRT(IActiveScriptProfilerCallback, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            __RPC__in IActiveScriptProfilerCallback2 * This,
            /* [in] */ DWORD dwContext);
        
        DECLSPEC_XFGVIRT(IActiveScriptProfilerCallback, Shutdown)
        HRESULT ( STDMETHODCALLTYPE *Shutdown )( 
            __RPC__in IActiveScriptProfilerCallback2 * This,
            /* [in] */ HRESULT hrReason);
        
        DECLSPEC_XFGVIRT(IActiveScriptProfilerCallback, ScriptCompiled)
        HRESULT ( STDMETHODCALLTYPE *ScriptCompiled )( 
            __RPC__in IActiveScriptProfilerCallback2 * This,
            /* [in] */ PROFILER_TOKEN scriptId,
            /* [in] */ PROFILER_SCRIPT_TYPE type,
            /* [in] */ __RPC__in_opt IUnknown *pIDebugDocumentContext);
        
        DECLSPEC_XFGVIRT(IActiveScriptProfilerCallback, FunctionCompiled)
        HRESULT ( STDMETHODCALLTYPE *FunctionCompiled )( 
            __RPC__in IActiveScriptProfilerCallback2 * This,
            /* [in] */ PROFILER_TOKEN functionId,
            /* [in] */ PROFILER_TOKEN scriptId,
            /* [string][in] */ __RPC__in_string const WCHAR *pwszFunctionName,
            /* [string][in] */ __RPC__in_string const WCHAR *pwszFunctionNameHint,
            /* [in] */ __RPC__in_opt IUnknown *pIDebugDocumentContext);
        
        DECLSPEC_XFGVIRT(IActiveScriptProfilerCallback, OnFunctionEnter)
        HRESULT ( STDMETHODCALLTYPE *OnFunctionEnter )( 
            __RPC__in IActiveScriptProfilerCallback2 * This,
            /* [in] */ PROFILER_TOKEN scriptId,
            /* [in] */ PROFILER_TOKEN functionId);
        
        DECLSPEC_XFGVIRT(IActiveScriptProfilerCallback, OnFunctionExit)
        HRESULT ( STDMETHODCALLTYPE *OnFunctionExit )( 
            __RPC__in IActiveScriptProfilerCallback2 * This,
            /* [in] */ PROFILER_TOKEN scriptId,
            /* [in] */ PROFILER_TOKEN functionId);
        
        DECLSPEC_XFGVIRT(IActiveScriptProfilerCallback2, OnFunctionEnterByName)
        HRESULT ( STDMETHODCALLTYPE *OnFunctionEnterByName )( 
            __RPC__in IActiveScriptProfilerCallback2 * This,
            /* [string][in] */ __RPC__in_string const WCHAR *pwszFunctionName,
            /* [in] */ PROFILER_SCRIPT_TYPE type);
        
        DECLSPEC_XFGVIRT(IActiveScriptProfilerCallback2, OnFunctionExitByName)
        HRESULT ( STDMETHODCALLTYPE *OnFunctionExitByName )( 
            __RPC__in IActiveScriptProfilerCallback2 * This,
            /* [string][in] */ __RPC__in_string const WCHAR *pwszFunctionName,
            /* [in] */ PROFILER_SCRIPT_TYPE type);
        
        END_INTERFACE
    } IActiveScriptProfilerCallback2Vtbl;

    interface IActiveScriptProfilerCallback2
    {
        CONST_VTBL struct IActiveScriptProfilerCallback2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IActiveScriptProfilerCallback2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IActiveScriptProfilerCallback2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IActiveScriptProfilerCallback2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IActiveScriptProfilerCallback2_Initialize(This,dwContext)	\
    ( (This)->lpVtbl -> Initialize(This,dwContext) ) 

#define IActiveScriptProfilerCallback2_Shutdown(This,hrReason)	\
    ( (This)->lpVtbl -> Shutdown(This,hrReason) ) 

#define IActiveScriptProfilerCallback2_ScriptCompiled(This,scriptId,type,pIDebugDocumentContext)	\
    ( (This)->lpVtbl -> ScriptCompiled(This,scriptId,type,pIDebugDocumentContext) ) 

#define IActiveScriptProfilerCallback2_FunctionCompiled(This,functionId,scriptId,pwszFunctionName,pwszFunctionNameHint,pIDebugDocumentContext)	\
    ( (This)->lpVtbl -> FunctionCompiled(This,functionId,scriptId,pwszFunctionName,pwszFunctionNameHint,pIDebugDocumentContext) ) 

#define IActiveScriptProfilerCallback2_OnFunctionEnter(This,scriptId,functionId)	\
    ( (This)->lpVtbl -> OnFunctionEnter(This,scriptId,functionId) ) 

#define IActiveScriptProfilerCallback2_OnFunctionExit(This,scriptId,functionId)	\
    ( (This)->lpVtbl -> OnFunctionExit(This,scriptId,functionId) ) 


#define IActiveScriptProfilerCallback2_OnFunctionEnterByName(This,pwszFunctionName,type)	\
    ( (This)->lpVtbl -> OnFunctionEnterByName(This,pwszFunctionName,type) ) 

#define IActiveScriptProfilerCallback2_OnFunctionExitByName(This,pwszFunctionName,type)	\
    ( (This)->lpVtbl -> OnFunctionExitByName(This,pwszFunctionName,type) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IActiveScriptProfilerCallback2_INTERFACE_DEFINED__ */


#ifndef __IActiveScriptProfilerCallback3_INTERFACE_DEFINED__
#define __IActiveScriptProfilerCallback3_INTERFACE_DEFINED__

/* interface IActiveScriptProfilerCallback3 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IActiveScriptProfilerCallback3;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6ac5ad25-2037-4687-91df-b59979d93d73")
    IActiveScriptProfilerCallback3 : public IActiveScriptProfilerCallback2
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetWebWorkerId( 
            /* [in] */ DWORD webWorkerId) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IActiveScriptProfilerCallback3Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IActiveScriptProfilerCallback3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IActiveScriptProfilerCallback3 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IActiveScriptProfilerCallback3 * This);
        
        DECLSPEC_XFGVIRT(IActiveScriptProfilerCallback, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            __RPC__in IActiveScriptProfilerCallback3 * This,
            /* [in] */ DWORD dwContext);
        
        DECLSPEC_XFGVIRT(IActiveScriptProfilerCallback, Shutdown)
        HRESULT ( STDMETHODCALLTYPE *Shutdown )( 
            __RPC__in IActiveScriptProfilerCallback3 * This,
            /* [in] */ HRESULT hrReason);
        
        DECLSPEC_XFGVIRT(IActiveScriptProfilerCallback, ScriptCompiled)
        HRESULT ( STDMETHODCALLTYPE *ScriptCompiled )( 
            __RPC__in IActiveScriptProfilerCallback3 * This,
            /* [in] */ PROFILER_TOKEN scriptId,
            /* [in] */ PROFILER_SCRIPT_TYPE type,
            /* [in] */ __RPC__in_opt IUnknown *pIDebugDocumentContext);
        
        DECLSPEC_XFGVIRT(IActiveScriptProfilerCallback, FunctionCompiled)
        HRESULT ( STDMETHODCALLTYPE *FunctionCompiled )( 
            __RPC__in IActiveScriptProfilerCallback3 * This,
            /* [in] */ PROFILER_TOKEN functionId,
            /* [in] */ PROFILER_TOKEN scriptId,
            /* [string][in] */ __RPC__in_string const WCHAR *pwszFunctionName,
            /* [string][in] */ __RPC__in_string const WCHAR *pwszFunctionNameHint,
            /* [in] */ __RPC__in_opt IUnknown *pIDebugDocumentContext);
        
        DECLSPEC_XFGVIRT(IActiveScriptProfilerCallback, OnFunctionEnter)
        HRESULT ( STDMETHODCALLTYPE *OnFunctionEnter )( 
            __RPC__in IActiveScriptProfilerCallback3 * This,
            /* [in] */ PROFILER_TOKEN scriptId,
            /* [in] */ PROFILER_TOKEN functionId);
        
        DECLSPEC_XFGVIRT(IActiveScriptProfilerCallback, OnFunctionExit)
        HRESULT ( STDMETHODCALLTYPE *OnFunctionExit )( 
            __RPC__in IActiveScriptProfilerCallback3 * This,
            /* [in] */ PROFILER_TOKEN scriptId,
            /* [in] */ PROFILER_TOKEN functionId);
        
        DECLSPEC_XFGVIRT(IActiveScriptProfilerCallback2, OnFunctionEnterByName)
        HRESULT ( STDMETHODCALLTYPE *OnFunctionEnterByName )( 
            __RPC__in IActiveScriptProfilerCallback3 * This,
            /* [string][in] */ __RPC__in_string const WCHAR *pwszFunctionName,
            /* [in] */ PROFILER_SCRIPT_TYPE type);
        
        DECLSPEC_XFGVIRT(IActiveScriptProfilerCallback2, OnFunctionExitByName)
        HRESULT ( STDMETHODCALLTYPE *OnFunctionExitByName )( 
            __RPC__in IActiveScriptProfilerCallback3 * This,
            /* [string][in] */ __RPC__in_string const WCHAR *pwszFunctionName,
            /* [in] */ PROFILER_SCRIPT_TYPE type);
        
        DECLSPEC_XFGVIRT(IActiveScriptProfilerCallback3, SetWebWorkerId)
        HRESULT ( STDMETHODCALLTYPE *SetWebWorkerId )( 
            __RPC__in IActiveScriptProfilerCallback3 * This,
            /* [in] */ DWORD webWorkerId);
        
        END_INTERFACE
    } IActiveScriptProfilerCallback3Vtbl;

    interface IActiveScriptProfilerCallback3
    {
        CONST_VTBL struct IActiveScriptProfilerCallback3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IActiveScriptProfilerCallback3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IActiveScriptProfilerCallback3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IActiveScriptProfilerCallback3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IActiveScriptProfilerCallback3_Initialize(This,dwContext)	\
    ( (This)->lpVtbl -> Initialize(This,dwContext) ) 

#define IActiveScriptProfilerCallback3_Shutdown(This,hrReason)	\
    ( (This)->lpVtbl -> Shutdown(This,hrReason) ) 

#define IActiveScriptProfilerCallback3_ScriptCompiled(This,scriptId,type,pIDebugDocumentContext)	\
    ( (This)->lpVtbl -> ScriptCompiled(This,scriptId,type,pIDebugDocumentContext) ) 

#define IActiveScriptProfilerCallback3_FunctionCompiled(This,functionId,scriptId,pwszFunctionName,pwszFunctionNameHint,pIDebugDocumentContext)	\
    ( (This)->lpVtbl -> FunctionCompiled(This,functionId,scriptId,pwszFunctionName,pwszFunctionNameHint,pIDebugDocumentContext) ) 

#define IActiveScriptProfilerCallback3_OnFunctionEnter(This,scriptId,functionId)	\
    ( (This)->lpVtbl -> OnFunctionEnter(This,scriptId,functionId) ) 

#define IActiveScriptProfilerCallback3_OnFunctionExit(This,scriptId,functionId)	\
    ( (This)->lpVtbl -> OnFunctionExit(This,scriptId,functionId) ) 


#define IActiveScriptProfilerCallback3_OnFunctionEnterByName(This,pwszFunctionName,type)	\
    ( (This)->lpVtbl -> OnFunctionEnterByName(This,pwszFunctionName,type) ) 

#define IActiveScriptProfilerCallback3_OnFunctionExitByName(This,pwszFunctionName,type)	\
    ( (This)->lpVtbl -> OnFunctionExitByName(This,pwszFunctionName,type) ) 


#define IActiveScriptProfilerCallback3_SetWebWorkerId(This,webWorkerId)	\
    ( (This)->lpVtbl -> SetWebWorkerId(This,webWorkerId) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IActiveScriptProfilerCallback3_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_activprof_0000_0009 */
/* [local] */ 


#endif  // __ActivProf_h

// Restore the previous setting for C4201 compiler warning
#pragma warning(pop)
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_activprof_0000_0009_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_activprof_0000_0009_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


