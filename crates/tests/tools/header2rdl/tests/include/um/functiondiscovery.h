

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


#ifndef __functiondiscovery_h__
#define __functiondiscovery_h__

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

#ifndef __FunctionDiscovery_FWD_DEFINED__
#define __FunctionDiscovery_FWD_DEFINED__

#ifdef __cplusplus
typedef class FunctionDiscovery FunctionDiscovery;
#else
typedef struct FunctionDiscovery FunctionDiscovery;
#endif /* __cplusplus */

#endif 	/* __FunctionDiscovery_FWD_DEFINED__ */


#ifndef __PropertyStore_FWD_DEFINED__
#define __PropertyStore_FWD_DEFINED__

#ifdef __cplusplus
typedef class PropertyStore PropertyStore;
#else
typedef struct PropertyStore PropertyStore;
#endif /* __cplusplus */

#endif 	/* __PropertyStore_FWD_DEFINED__ */


#ifndef __FunctionInstanceCollection_FWD_DEFINED__
#define __FunctionInstanceCollection_FWD_DEFINED__

#ifdef __cplusplus
typedef class FunctionInstanceCollection FunctionInstanceCollection;
#else
typedef struct FunctionInstanceCollection FunctionInstanceCollection;
#endif /* __cplusplus */

#endif 	/* __FunctionInstanceCollection_FWD_DEFINED__ */


#ifndef __PropertyStoreCollection_FWD_DEFINED__
#define __PropertyStoreCollection_FWD_DEFINED__

#ifdef __cplusplus
typedef class PropertyStoreCollection PropertyStoreCollection;
#else
typedef struct PropertyStoreCollection PropertyStoreCollection;
#endif /* __cplusplus */

#endif 	/* __PropertyStoreCollection_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"
#include "propsys.h"
#include "FunctionDiscoveryAPI.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_functiondiscovery_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_functiondiscovery_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_functiondiscovery_0000_0000_v0_0_s_ifspec;


#ifndef __FctnDiscLib_LIBRARY_DEFINED__
#define __FctnDiscLib_LIBRARY_DEFINED__

/* library FctnDiscLib */
/* [helpstring][version][uuid] */ 


EXTERN_C const IID LIBID_FctnDiscLib;

EXTERN_C const CLSID CLSID_FunctionDiscovery;

#ifdef __cplusplus

class DECLSPEC_UUID("C72BE2EC-8E90-452c-B29A-AB8FF1C071FC")
FunctionDiscovery;
#endif

EXTERN_C const CLSID CLSID_PropertyStore;

#ifdef __cplusplus

class DECLSPEC_UUID("e4796550-df61-448b-9193-13fc1341b163")
PropertyStore;
#endif

EXTERN_C const CLSID CLSID_FunctionInstanceCollection;

#ifdef __cplusplus

class DECLSPEC_UUID("ba818ce5-b55f-443f-ad39-2fe89be6191f")
FunctionInstanceCollection;
#endif

EXTERN_C const CLSID CLSID_PropertyStoreCollection;

#ifdef __cplusplus

class DECLSPEC_UUID("EDD36029-D753-4862-AA5B-5BCCAD2A4D29")
PropertyStoreCollection;
#endif
#endif /* __FctnDiscLib_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_functiondiscovery_0000_0001 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_functiondiscovery_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_functiondiscovery_0000_0001_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


