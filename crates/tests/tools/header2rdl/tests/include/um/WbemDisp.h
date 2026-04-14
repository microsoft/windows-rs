

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

#ifndef __wbemdisp_h__
#define __wbemdisp_h__

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

#ifndef __ISWbemServices_FWD_DEFINED__
#define __ISWbemServices_FWD_DEFINED__
typedef interface ISWbemServices ISWbemServices;

#endif 	/* __ISWbemServices_FWD_DEFINED__ */


#ifndef __ISWbemLocator_FWD_DEFINED__
#define __ISWbemLocator_FWD_DEFINED__
typedef interface ISWbemLocator ISWbemLocator;

#endif 	/* __ISWbemLocator_FWD_DEFINED__ */


#ifndef __ISWbemObject_FWD_DEFINED__
#define __ISWbemObject_FWD_DEFINED__
typedef interface ISWbemObject ISWbemObject;

#endif 	/* __ISWbemObject_FWD_DEFINED__ */


#ifndef __ISWbemObjectSet_FWD_DEFINED__
#define __ISWbemObjectSet_FWD_DEFINED__
typedef interface ISWbemObjectSet ISWbemObjectSet;

#endif 	/* __ISWbemObjectSet_FWD_DEFINED__ */


#ifndef __ISWbemNamedValue_FWD_DEFINED__
#define __ISWbemNamedValue_FWD_DEFINED__
typedef interface ISWbemNamedValue ISWbemNamedValue;

#endif 	/* __ISWbemNamedValue_FWD_DEFINED__ */


#ifndef __ISWbemNamedValueSet_FWD_DEFINED__
#define __ISWbemNamedValueSet_FWD_DEFINED__
typedef interface ISWbemNamedValueSet ISWbemNamedValueSet;

#endif 	/* __ISWbemNamedValueSet_FWD_DEFINED__ */


#ifndef __ISWbemQualifier_FWD_DEFINED__
#define __ISWbemQualifier_FWD_DEFINED__
typedef interface ISWbemQualifier ISWbemQualifier;

#endif 	/* __ISWbemQualifier_FWD_DEFINED__ */


#ifndef __ISWbemQualifierSet_FWD_DEFINED__
#define __ISWbemQualifierSet_FWD_DEFINED__
typedef interface ISWbemQualifierSet ISWbemQualifierSet;

#endif 	/* __ISWbemQualifierSet_FWD_DEFINED__ */


#ifndef __ISWbemProperty_FWD_DEFINED__
#define __ISWbemProperty_FWD_DEFINED__
typedef interface ISWbemProperty ISWbemProperty;

#endif 	/* __ISWbemProperty_FWD_DEFINED__ */


#ifndef __ISWbemPropertySet_FWD_DEFINED__
#define __ISWbemPropertySet_FWD_DEFINED__
typedef interface ISWbemPropertySet ISWbemPropertySet;

#endif 	/* __ISWbemPropertySet_FWD_DEFINED__ */


#ifndef __ISWbemMethod_FWD_DEFINED__
#define __ISWbemMethod_FWD_DEFINED__
typedef interface ISWbemMethod ISWbemMethod;

#endif 	/* __ISWbemMethod_FWD_DEFINED__ */


#ifndef __ISWbemMethodSet_FWD_DEFINED__
#define __ISWbemMethodSet_FWD_DEFINED__
typedef interface ISWbemMethodSet ISWbemMethodSet;

#endif 	/* __ISWbemMethodSet_FWD_DEFINED__ */


#ifndef __ISWbemEventSource_FWD_DEFINED__
#define __ISWbemEventSource_FWD_DEFINED__
typedef interface ISWbemEventSource ISWbemEventSource;

#endif 	/* __ISWbemEventSource_FWD_DEFINED__ */


#ifndef __ISWbemObjectPath_FWD_DEFINED__
#define __ISWbemObjectPath_FWD_DEFINED__
typedef interface ISWbemObjectPath ISWbemObjectPath;

#endif 	/* __ISWbemObjectPath_FWD_DEFINED__ */


#ifndef __ISWbemLastError_FWD_DEFINED__
#define __ISWbemLastError_FWD_DEFINED__
typedef interface ISWbemLastError ISWbemLastError;

#endif 	/* __ISWbemLastError_FWD_DEFINED__ */


#ifndef __ISWbemSinkEvents_FWD_DEFINED__
#define __ISWbemSinkEvents_FWD_DEFINED__
typedef interface ISWbemSinkEvents ISWbemSinkEvents;

#endif 	/* __ISWbemSinkEvents_FWD_DEFINED__ */


#ifndef __ISWbemSink_FWD_DEFINED__
#define __ISWbemSink_FWD_DEFINED__
typedef interface ISWbemSink ISWbemSink;

#endif 	/* __ISWbemSink_FWD_DEFINED__ */


#ifndef __ISWbemSecurity_FWD_DEFINED__
#define __ISWbemSecurity_FWD_DEFINED__
typedef interface ISWbemSecurity ISWbemSecurity;

#endif 	/* __ISWbemSecurity_FWD_DEFINED__ */


#ifndef __ISWbemPrivilege_FWD_DEFINED__
#define __ISWbemPrivilege_FWD_DEFINED__
typedef interface ISWbemPrivilege ISWbemPrivilege;

#endif 	/* __ISWbemPrivilege_FWD_DEFINED__ */


#ifndef __ISWbemPrivilegeSet_FWD_DEFINED__
#define __ISWbemPrivilegeSet_FWD_DEFINED__
typedef interface ISWbemPrivilegeSet ISWbemPrivilegeSet;

#endif 	/* __ISWbemPrivilegeSet_FWD_DEFINED__ */


#ifndef __ISWbemServicesEx_FWD_DEFINED__
#define __ISWbemServicesEx_FWD_DEFINED__
typedef interface ISWbemServicesEx ISWbemServicesEx;

#endif 	/* __ISWbemServicesEx_FWD_DEFINED__ */


#ifndef __ISWbemObjectEx_FWD_DEFINED__
#define __ISWbemObjectEx_FWD_DEFINED__
typedef interface ISWbemObjectEx ISWbemObjectEx;

#endif 	/* __ISWbemObjectEx_FWD_DEFINED__ */


#ifndef __ISWbemDateTime_FWD_DEFINED__
#define __ISWbemDateTime_FWD_DEFINED__
typedef interface ISWbemDateTime ISWbemDateTime;

#endif 	/* __ISWbemDateTime_FWD_DEFINED__ */


#ifndef __ISWbemRefresher_FWD_DEFINED__
#define __ISWbemRefresher_FWD_DEFINED__
typedef interface ISWbemRefresher ISWbemRefresher;

#endif 	/* __ISWbemRefresher_FWD_DEFINED__ */


#ifndef __ISWbemRefreshableItem_FWD_DEFINED__
#define __ISWbemRefreshableItem_FWD_DEFINED__
typedef interface ISWbemRefreshableItem ISWbemRefreshableItem;

#endif 	/* __ISWbemRefreshableItem_FWD_DEFINED__ */


#ifndef __SWbemLocator_FWD_DEFINED__
#define __SWbemLocator_FWD_DEFINED__

#ifdef __cplusplus
typedef class SWbemLocator SWbemLocator;
#else
typedef struct SWbemLocator SWbemLocator;
#endif /* __cplusplus */

#endif 	/* __SWbemLocator_FWD_DEFINED__ */


#ifndef __SWbemNamedValueSet_FWD_DEFINED__
#define __SWbemNamedValueSet_FWD_DEFINED__

#ifdef __cplusplus
typedef class SWbemNamedValueSet SWbemNamedValueSet;
#else
typedef struct SWbemNamedValueSet SWbemNamedValueSet;
#endif /* __cplusplus */

#endif 	/* __SWbemNamedValueSet_FWD_DEFINED__ */


#ifndef __SWbemObjectPath_FWD_DEFINED__
#define __SWbemObjectPath_FWD_DEFINED__

#ifdef __cplusplus
typedef class SWbemObjectPath SWbemObjectPath;
#else
typedef struct SWbemObjectPath SWbemObjectPath;
#endif /* __cplusplus */

#endif 	/* __SWbemObjectPath_FWD_DEFINED__ */


#ifndef __SWbemLastError_FWD_DEFINED__
#define __SWbemLastError_FWD_DEFINED__

#ifdef __cplusplus
typedef class SWbemLastError SWbemLastError;
#else
typedef struct SWbemLastError SWbemLastError;
#endif /* __cplusplus */

#endif 	/* __SWbemLastError_FWD_DEFINED__ */


#ifndef __SWbemSink_FWD_DEFINED__
#define __SWbemSink_FWD_DEFINED__

#ifdef __cplusplus
typedef class SWbemSink SWbemSink;
#else
typedef struct SWbemSink SWbemSink;
#endif /* __cplusplus */

#endif 	/* __SWbemSink_FWD_DEFINED__ */


#ifndef __SWbemDateTime_FWD_DEFINED__
#define __SWbemDateTime_FWD_DEFINED__

#ifdef __cplusplus
typedef class SWbemDateTime SWbemDateTime;
#else
typedef struct SWbemDateTime SWbemDateTime;
#endif /* __cplusplus */

#endif 	/* __SWbemDateTime_FWD_DEFINED__ */


#ifndef __SWbemRefresher_FWD_DEFINED__
#define __SWbemRefresher_FWD_DEFINED__

#ifdef __cplusplus
typedef class SWbemRefresher SWbemRefresher;
#else
typedef struct SWbemRefresher SWbemRefresher;
#endif /* __cplusplus */

#endif 	/* __SWbemRefresher_FWD_DEFINED__ */


#ifndef __SWbemServices_FWD_DEFINED__
#define __SWbemServices_FWD_DEFINED__

#ifdef __cplusplus
typedef class SWbemServices SWbemServices;
#else
typedef struct SWbemServices SWbemServices;
#endif /* __cplusplus */

#endif 	/* __SWbemServices_FWD_DEFINED__ */


#ifndef __SWbemServicesEx_FWD_DEFINED__
#define __SWbemServicesEx_FWD_DEFINED__

#ifdef __cplusplus
typedef class SWbemServicesEx SWbemServicesEx;
#else
typedef struct SWbemServicesEx SWbemServicesEx;
#endif /* __cplusplus */

#endif 	/* __SWbemServicesEx_FWD_DEFINED__ */


#ifndef __SWbemObject_FWD_DEFINED__
#define __SWbemObject_FWD_DEFINED__

#ifdef __cplusplus
typedef class SWbemObject SWbemObject;
#else
typedef struct SWbemObject SWbemObject;
#endif /* __cplusplus */

#endif 	/* __SWbemObject_FWD_DEFINED__ */


#ifndef __SWbemObjectEx_FWD_DEFINED__
#define __SWbemObjectEx_FWD_DEFINED__

#ifdef __cplusplus
typedef class SWbemObjectEx SWbemObjectEx;
#else
typedef struct SWbemObjectEx SWbemObjectEx;
#endif /* __cplusplus */

#endif 	/* __SWbemObjectEx_FWD_DEFINED__ */


#ifndef __SWbemObjectSet_FWD_DEFINED__
#define __SWbemObjectSet_FWD_DEFINED__

#ifdef __cplusplus
typedef class SWbemObjectSet SWbemObjectSet;
#else
typedef struct SWbemObjectSet SWbemObjectSet;
#endif /* __cplusplus */

#endif 	/* __SWbemObjectSet_FWD_DEFINED__ */


#ifndef __SWbemNamedValue_FWD_DEFINED__
#define __SWbemNamedValue_FWD_DEFINED__

#ifdef __cplusplus
typedef class SWbemNamedValue SWbemNamedValue;
#else
typedef struct SWbemNamedValue SWbemNamedValue;
#endif /* __cplusplus */

#endif 	/* __SWbemNamedValue_FWD_DEFINED__ */


#ifndef __SWbemQualifier_FWD_DEFINED__
#define __SWbemQualifier_FWD_DEFINED__

#ifdef __cplusplus
typedef class SWbemQualifier SWbemQualifier;
#else
typedef struct SWbemQualifier SWbemQualifier;
#endif /* __cplusplus */

#endif 	/* __SWbemQualifier_FWD_DEFINED__ */


#ifndef __SWbemQualifierSet_FWD_DEFINED__
#define __SWbemQualifierSet_FWD_DEFINED__

#ifdef __cplusplus
typedef class SWbemQualifierSet SWbemQualifierSet;
#else
typedef struct SWbemQualifierSet SWbemQualifierSet;
#endif /* __cplusplus */

#endif 	/* __SWbemQualifierSet_FWD_DEFINED__ */


#ifndef __SWbemProperty_FWD_DEFINED__
#define __SWbemProperty_FWD_DEFINED__

#ifdef __cplusplus
typedef class SWbemProperty SWbemProperty;
#else
typedef struct SWbemProperty SWbemProperty;
#endif /* __cplusplus */

#endif 	/* __SWbemProperty_FWD_DEFINED__ */


#ifndef __SWbemPropertySet_FWD_DEFINED__
#define __SWbemPropertySet_FWD_DEFINED__

#ifdef __cplusplus
typedef class SWbemPropertySet SWbemPropertySet;
#else
typedef struct SWbemPropertySet SWbemPropertySet;
#endif /* __cplusplus */

#endif 	/* __SWbemPropertySet_FWD_DEFINED__ */


#ifndef __SWbemMethod_FWD_DEFINED__
#define __SWbemMethod_FWD_DEFINED__

#ifdef __cplusplus
typedef class SWbemMethod SWbemMethod;
#else
typedef struct SWbemMethod SWbemMethod;
#endif /* __cplusplus */

#endif 	/* __SWbemMethod_FWD_DEFINED__ */


#ifndef __SWbemMethodSet_FWD_DEFINED__
#define __SWbemMethodSet_FWD_DEFINED__

#ifdef __cplusplus
typedef class SWbemMethodSet SWbemMethodSet;
#else
typedef struct SWbemMethodSet SWbemMethodSet;
#endif /* __cplusplus */

#endif 	/* __SWbemMethodSet_FWD_DEFINED__ */


#ifndef __SWbemEventSource_FWD_DEFINED__
#define __SWbemEventSource_FWD_DEFINED__

#ifdef __cplusplus
typedef class SWbemEventSource SWbemEventSource;
#else
typedef struct SWbemEventSource SWbemEventSource;
#endif /* __cplusplus */

#endif 	/* __SWbemEventSource_FWD_DEFINED__ */


#ifndef __SWbemSecurity_FWD_DEFINED__
#define __SWbemSecurity_FWD_DEFINED__

#ifdef __cplusplus
typedef class SWbemSecurity SWbemSecurity;
#else
typedef struct SWbemSecurity SWbemSecurity;
#endif /* __cplusplus */

#endif 	/* __SWbemSecurity_FWD_DEFINED__ */


#ifndef __SWbemPrivilege_FWD_DEFINED__
#define __SWbemPrivilege_FWD_DEFINED__

#ifdef __cplusplus
typedef class SWbemPrivilege SWbemPrivilege;
#else
typedef struct SWbemPrivilege SWbemPrivilege;
#endif /* __cplusplus */

#endif 	/* __SWbemPrivilege_FWD_DEFINED__ */


#ifndef __SWbemPrivilegeSet_FWD_DEFINED__
#define __SWbemPrivilegeSet_FWD_DEFINED__

#ifdef __cplusplus
typedef class SWbemPrivilegeSet SWbemPrivilegeSet;
#else
typedef struct SWbemPrivilegeSet SWbemPrivilegeSet;
#endif /* __cplusplus */

#endif 	/* __SWbemPrivilegeSet_FWD_DEFINED__ */


#ifndef __SWbemRefreshableItem_FWD_DEFINED__
#define __SWbemRefreshableItem_FWD_DEFINED__

#ifdef __cplusplus
typedef class SWbemRefreshableItem SWbemRefreshableItem;
#else
typedef struct SWbemRefreshableItem SWbemRefreshableItem;
#endif /* __cplusplus */

#endif 	/* __SWbemRefreshableItem_FWD_DEFINED__ */


#ifndef __ISWbemLocator_FWD_DEFINED__
#define __ISWbemLocator_FWD_DEFINED__
typedef interface ISWbemLocator ISWbemLocator;

#endif 	/* __ISWbemLocator_FWD_DEFINED__ */


#ifndef __ISWbemServices_FWD_DEFINED__
#define __ISWbemServices_FWD_DEFINED__
typedef interface ISWbemServices ISWbemServices;

#endif 	/* __ISWbemServices_FWD_DEFINED__ */


#ifndef __ISWbemServicesEx_FWD_DEFINED__
#define __ISWbemServicesEx_FWD_DEFINED__
typedef interface ISWbemServicesEx ISWbemServicesEx;

#endif 	/* __ISWbemServicesEx_FWD_DEFINED__ */


#ifndef __ISWbemObject_FWD_DEFINED__
#define __ISWbemObject_FWD_DEFINED__
typedef interface ISWbemObject ISWbemObject;

#endif 	/* __ISWbemObject_FWD_DEFINED__ */


#ifndef __ISWbemObjectEx_FWD_DEFINED__
#define __ISWbemObjectEx_FWD_DEFINED__
typedef interface ISWbemObjectEx ISWbemObjectEx;

#endif 	/* __ISWbemObjectEx_FWD_DEFINED__ */


#ifndef __ISWbemLastError_FWD_DEFINED__
#define __ISWbemLastError_FWD_DEFINED__
typedef interface ISWbemLastError ISWbemLastError;

#endif 	/* __ISWbemLastError_FWD_DEFINED__ */


#ifndef __ISWbemObjectSet_FWD_DEFINED__
#define __ISWbemObjectSet_FWD_DEFINED__
typedef interface ISWbemObjectSet ISWbemObjectSet;

#endif 	/* __ISWbemObjectSet_FWD_DEFINED__ */


#ifndef __ISWbemNamedValueSet_FWD_DEFINED__
#define __ISWbemNamedValueSet_FWD_DEFINED__
typedef interface ISWbemNamedValueSet ISWbemNamedValueSet;

#endif 	/* __ISWbemNamedValueSet_FWD_DEFINED__ */


#ifndef __ISWbemNamedValue_FWD_DEFINED__
#define __ISWbemNamedValue_FWD_DEFINED__
typedef interface ISWbemNamedValue ISWbemNamedValue;

#endif 	/* __ISWbemNamedValue_FWD_DEFINED__ */


#ifndef __ISWbemObjectPath_FWD_DEFINED__
#define __ISWbemObjectPath_FWD_DEFINED__
typedef interface ISWbemObjectPath ISWbemObjectPath;

#endif 	/* __ISWbemObjectPath_FWD_DEFINED__ */


#ifndef __ISWbemProperty_FWD_DEFINED__
#define __ISWbemProperty_FWD_DEFINED__
typedef interface ISWbemProperty ISWbemProperty;

#endif 	/* __ISWbemProperty_FWD_DEFINED__ */


#ifndef __ISWbemPropertySet_FWD_DEFINED__
#define __ISWbemPropertySet_FWD_DEFINED__
typedef interface ISWbemPropertySet ISWbemPropertySet;

#endif 	/* __ISWbemPropertySet_FWD_DEFINED__ */


#ifndef __ISWbemQualifier_FWD_DEFINED__
#define __ISWbemQualifier_FWD_DEFINED__
typedef interface ISWbemQualifier ISWbemQualifier;

#endif 	/* __ISWbemQualifier_FWD_DEFINED__ */


#ifndef __ISWbemQualifierSet_FWD_DEFINED__
#define __ISWbemQualifierSet_FWD_DEFINED__
typedef interface ISWbemQualifierSet ISWbemQualifierSet;

#endif 	/* __ISWbemQualifierSet_FWD_DEFINED__ */


#ifndef __ISWbemMethod_FWD_DEFINED__
#define __ISWbemMethod_FWD_DEFINED__
typedef interface ISWbemMethod ISWbemMethod;

#endif 	/* __ISWbemMethod_FWD_DEFINED__ */


#ifndef __ISWbemMethodSet_FWD_DEFINED__
#define __ISWbemMethodSet_FWD_DEFINED__
typedef interface ISWbemMethodSet ISWbemMethodSet;

#endif 	/* __ISWbemMethodSet_FWD_DEFINED__ */


#ifndef __ISWbemSink_FWD_DEFINED__
#define __ISWbemSink_FWD_DEFINED__
typedef interface ISWbemSink ISWbemSink;

#endif 	/* __ISWbemSink_FWD_DEFINED__ */


#ifndef __ISWbemSinkEvents_FWD_DEFINED__
#define __ISWbemSinkEvents_FWD_DEFINED__
typedef interface ISWbemSinkEvents ISWbemSinkEvents;

#endif 	/* __ISWbemSinkEvents_FWD_DEFINED__ */


#ifndef __ISWbemEventSource_FWD_DEFINED__
#define __ISWbemEventSource_FWD_DEFINED__
typedef interface ISWbemEventSource ISWbemEventSource;

#endif 	/* __ISWbemEventSource_FWD_DEFINED__ */


#ifndef __ISWbemSecurity_FWD_DEFINED__
#define __ISWbemSecurity_FWD_DEFINED__
typedef interface ISWbemSecurity ISWbemSecurity;

#endif 	/* __ISWbemSecurity_FWD_DEFINED__ */


#ifndef __ISWbemPrivilege_FWD_DEFINED__
#define __ISWbemPrivilege_FWD_DEFINED__
typedef interface ISWbemPrivilege ISWbemPrivilege;

#endif 	/* __ISWbemPrivilege_FWD_DEFINED__ */


#ifndef __ISWbemPrivilegeSet_FWD_DEFINED__
#define __ISWbemPrivilegeSet_FWD_DEFINED__
typedef interface ISWbemPrivilegeSet ISWbemPrivilegeSet;

#endif 	/* __ISWbemPrivilegeSet_FWD_DEFINED__ */


#ifndef __ISWbemDateTime_FWD_DEFINED__
#define __ISWbemDateTime_FWD_DEFINED__
typedef interface ISWbemDateTime ISWbemDateTime;

#endif 	/* __ISWbemDateTime_FWD_DEFINED__ */


#ifndef __ISWbemRefreshableItem_FWD_DEFINED__
#define __ISWbemRefreshableItem_FWD_DEFINED__
typedef interface ISWbemRefreshableItem ISWbemRefreshableItem;

#endif 	/* __ISWbemRefreshableItem_FWD_DEFINED__ */


#ifndef __ISWbemRefresher_FWD_DEFINED__
#define __ISWbemRefresher_FWD_DEFINED__
typedef interface ISWbemRefresher ISWbemRefresher;

#endif 	/* __ISWbemRefresher_FWD_DEFINED__ */


/* header files for imported files */
#include "dispex.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_wbemdisp_0000_0000 */
/* [local] */ 

/*******************************************************************************/
/*                                                                             */
/*    Copyright (c) Microsoft Corporation.  All rights reserved.               */
/*                                                                             */
/*    WBEM IDispatch Compatible Access for Automation                          */
/*                                                                             */
/*******************************************************************************/
#include <winapifamily.h>
#pragma region Desktop Family or WinMgmt Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_WINMGMT)


extern RPC_IF_HANDLE __MIDL_itf_wbemdisp_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wbemdisp_0000_0000_v0_0_s_ifspec;


#ifndef __WbemScripting_LIBRARY_DEFINED__
#define __WbemScripting_LIBRARY_DEFINED__

/* library WbemScripting */
/* [helpstring][version][lcid][uuid] */ 


























typedef /* [helpstring][uuid][v1_enum] */  DECLSPEC_UUID("4A249B72-FC9A-11d1-8B1E-00600806D9B6") 
enum WbemChangeFlagEnum
    {
        wbemChangeFlagCreateOrUpdate	= 0,
        wbemChangeFlagUpdateOnly	= 0x1,
        wbemChangeFlagCreateOnly	= 0x2,
        wbemChangeFlagUpdateCompatible	= 0,
        wbemChangeFlagUpdateSafeMode	= 0x20,
        wbemChangeFlagUpdateForceMode	= 0x40,
        wbemChangeFlagStrongValidation	= 0x80,
        wbemChangeFlagAdvisory	= 0x10000
    } 	WbemChangeFlagEnum;

typedef /* [helpstring][uuid][v1_enum] */  DECLSPEC_UUID("4A249B73-FC9A-11d1-8B1E-00600806D9B6") 
enum WbemFlagEnum
    {
        wbemFlagReturnImmediately	= 0x10,
        wbemFlagReturnWhenComplete	= 0,
        wbemFlagBidirectional	= 0,
        wbemFlagForwardOnly	= 0x20,
        wbemFlagNoErrorObject	= 0x40,
        wbemFlagReturnErrorObject	= 0,
        wbemFlagSendStatus	= 0x80,
        wbemFlagDontSendStatus	= 0,
        wbemFlagEnsureLocatable	= 0x100,
        wbemFlagDirectRead	= 0x200,
        wbemFlagSendOnlySelected	= 0,
        wbemFlagUseAmendedQualifiers	= 0x20000,
        wbemFlagGetDefault	= 0,
        wbemFlagSpawnInstance	= 0x1,
        wbemFlagUseCurrentTime	= 0x1
    } 	WbemFlagEnum;

typedef /* [helpstring][uuid][v1_enum] */  DECLSPEC_UUID("4A249B76-FC9A-11d1-8B1E-00600806D9B6") 
enum WbemQueryFlagEnum
    {
        wbemQueryFlagDeep	= 0,
        wbemQueryFlagShallow	= 1,
        wbemQueryFlagPrototype	= 2
    } 	WbemQueryFlagEnum;

typedef /* [helpstring][uuid][v1_enum] */  DECLSPEC_UUID("4A249B78-FC9A-11d1-8B1E-00600806D9B6") 
enum WbemTextFlagEnum
    {
        wbemTextFlagNoFlavors	= 0x1
    } 	WbemTextFlagEnum;

typedef /* [helpstring][uuid][v1_enum] */  DECLSPEC_UUID("BF078C2A-07D9-11d2-8B21-00600806D9B6") 
enum WbemTimeout
    {
        wbemTimeoutInfinite	= 0xffffffff
    } 	WbemTimeout;

typedef /* [helpstring][uuid][v1_enum] */  DECLSPEC_UUID("4A249B79-FC9A-11d1-8B1E-00600806D9B6") 
enum WbemComparisonFlagEnum
    {
        wbemComparisonFlagIncludeAll	= 0,
        wbemComparisonFlagIgnoreQualifiers	= 0x1,
        wbemComparisonFlagIgnoreObjectSource	= 0x2,
        wbemComparisonFlagIgnoreDefaultValues	= 0x4,
        wbemComparisonFlagIgnoreClass	= 0x8,
        wbemComparisonFlagIgnoreCase	= 0x10,
        wbemComparisonFlagIgnoreFlavor	= 0x20
    } 	WbemComparisonFlagEnum;

typedef /* [helpstring][uuid][v1_enum] */  DECLSPEC_UUID("4A249B7B-FC9A-11d1-8B1E-00600806D9B6") 
enum WbemCimtypeEnum
    {
        wbemCimtypeSint8	= 16,
        wbemCimtypeUint8	= 17,
        wbemCimtypeSint16	= 2,
        wbemCimtypeUint16	= 18,
        wbemCimtypeSint32	= 3,
        wbemCimtypeUint32	= 19,
        wbemCimtypeSint64	= 20,
        wbemCimtypeUint64	= 21,
        wbemCimtypeReal32	= 4,
        wbemCimtypeReal64	= 5,
        wbemCimtypeBoolean	= 11,
        wbemCimtypeString	= 8,
        wbemCimtypeDatetime	= 101,
        wbemCimtypeReference	= 102,
        wbemCimtypeChar16	= 103,
        wbemCimtypeObject	= 13
    } 	WbemCimtypeEnum;

typedef /* [helpstring][uuid][v1_enum] */  DECLSPEC_UUID("4A249B7C-FC9A-11d1-8B1E-00600806D9B6") 
enum WbemErrorEnum
    {
        wbemNoErr	= 0,
        wbemErrFailed	= 0x80041001,
        wbemErrNotFound	= 0x80041002,
        wbemErrAccessDenied	= 0x80041003,
        wbemErrProviderFailure	= 0x80041004,
        wbemErrTypeMismatch	= 0x80041005,
        wbemErrOutOfMemory	= 0x80041006,
        wbemErrInvalidContext	= 0x80041007,
        wbemErrInvalidParameter	= 0x80041008,
        wbemErrNotAvailable	= 0x80041009,
        wbemErrCriticalError	= 0x8004100a,
        wbemErrInvalidStream	= 0x8004100b,
        wbemErrNotSupported	= 0x8004100c,
        wbemErrInvalidSuperclass	= 0x8004100d,
        wbemErrInvalidNamespace	= 0x8004100e,
        wbemErrInvalidObject	= 0x8004100f,
        wbemErrInvalidClass	= 0x80041010,
        wbemErrProviderNotFound	= 0x80041011,
        wbemErrInvalidProviderRegistration	= 0x80041012,
        wbemErrProviderLoadFailure	= 0x80041013,
        wbemErrInitializationFailure	= 0x80041014,
        wbemErrTransportFailure	= 0x80041015,
        wbemErrInvalidOperation	= 0x80041016,
        wbemErrInvalidQuery	= 0x80041017,
        wbemErrInvalidQueryType	= 0x80041018,
        wbemErrAlreadyExists	= 0x80041019,
        wbemErrOverrideNotAllowed	= 0x8004101a,
        wbemErrPropagatedQualifier	= 0x8004101b,
        wbemErrPropagatedProperty	= 0x8004101c,
        wbemErrUnexpected	= 0x8004101d,
        wbemErrIllegalOperation	= 0x8004101e,
        wbemErrCannotBeKey	= 0x8004101f,
        wbemErrIncompleteClass	= 0x80041020,
        wbemErrInvalidSyntax	= 0x80041021,
        wbemErrNondecoratedObject	= 0x80041022,
        wbemErrReadOnly	= 0x80041023,
        wbemErrProviderNotCapable	= 0x80041024,
        wbemErrClassHasChildren	= 0x80041025,
        wbemErrClassHasInstances	= 0x80041026,
        wbemErrQueryNotImplemented	= 0x80041027,
        wbemErrIllegalNull	= 0x80041028,
        wbemErrInvalidQualifierType	= 0x80041029,
        wbemErrInvalidPropertyType	= 0x8004102a,
        wbemErrValueOutOfRange	= 0x8004102b,
        wbemErrCannotBeSingleton	= 0x8004102c,
        wbemErrInvalidCimType	= 0x8004102d,
        wbemErrInvalidMethod	= 0x8004102e,
        wbemErrInvalidMethodParameters	= 0x8004102f,
        wbemErrSystemProperty	= 0x80041030,
        wbemErrInvalidProperty	= 0x80041031,
        wbemErrCallCancelled	= 0x80041032,
        wbemErrShuttingDown	= 0x80041033,
        wbemErrPropagatedMethod	= 0x80041034,
        wbemErrUnsupportedParameter	= 0x80041035,
        wbemErrMissingParameter	= 0x80041036,
        wbemErrInvalidParameterId	= 0x80041037,
        wbemErrNonConsecutiveParameterIds	= 0x80041038,
        wbemErrParameterIdOnRetval	= 0x80041039,
        wbemErrInvalidObjectPath	= 0x8004103a,
        wbemErrOutOfDiskSpace	= 0x8004103b,
        wbemErrBufferTooSmall	= 0x8004103c,
        wbemErrUnsupportedPutExtension	= 0x8004103d,
        wbemErrUnknownObjectType	= 0x8004103e,
        wbemErrUnknownPacketType	= 0x8004103f,
        wbemErrMarshalVersionMismatch	= 0x80041040,
        wbemErrMarshalInvalidSignature	= 0x80041041,
        wbemErrInvalidQualifier	= 0x80041042,
        wbemErrInvalidDuplicateParameter	= 0x80041043,
        wbemErrTooMuchData	= 0x80041044,
        wbemErrServerTooBusy	= 0x80041045,
        wbemErrInvalidFlavor	= 0x80041046,
        wbemErrCircularReference	= 0x80041047,
        wbemErrUnsupportedClassUpdate	= 0x80041048,
        wbemErrCannotChangeKeyInheritance	= 0x80041049,
        wbemErrCannotChangeIndexInheritance	= 0x80041050,
        wbemErrTooManyProperties	= 0x80041051,
        wbemErrUpdateTypeMismatch	= 0x80041052,
        wbemErrUpdateOverrideNotAllowed	= 0x80041053,
        wbemErrUpdatePropagatedMethod	= 0x80041054,
        wbemErrMethodNotImplemented	= 0x80041055,
        wbemErrMethodDisabled	= 0x80041056,
        wbemErrRefresherBusy	= 0x80041057,
        wbemErrUnparsableQuery	= 0x80041058,
        wbemErrNotEventClass	= 0x80041059,
        wbemErrMissingGroupWithin	= 0x8004105a,
        wbemErrMissingAggregationList	= 0x8004105b,
        wbemErrPropertyNotAnObject	= 0x8004105c,
        wbemErrAggregatingByObject	= 0x8004105d,
        wbemErrUninterpretableProviderQuery	= 0x8004105f,
        wbemErrBackupRestoreWinmgmtRunning	= 0x80041060,
        wbemErrQueueOverflow	= 0x80041061,
        wbemErrPrivilegeNotHeld	= 0x80041062,
        wbemErrInvalidOperator	= 0x80041063,
        wbemErrLocalCredentials	= 0x80041064,
        wbemErrCannotBeAbstract	= 0x80041065,
        wbemErrAmendedObject	= 0x80041066,
        wbemErrClientTooSlow	= 0x80041067,
        wbemErrNullSecurityDescriptor	= 0x80041068,
        wbemErrTimeout	= 0x80041069,
        wbemErrInvalidAssociation	= 0x8004106a,
        wbemErrAmbiguousOperation	= 0x8004106b,
        wbemErrQuotaViolation	= 0x8004106c,
        wbemErrTransactionConflict	= 0x8004106d,
        wbemErrForcedRollback	= 0x8004106e,
        wbemErrUnsupportedLocale	= 0x8004106f,
        wbemErrHandleOutOfDate	= 0x80041070,
        wbemErrConnectionFailed	= 0x80041071,
        wbemErrInvalidHandleRequest	= 0x80041072,
        wbemErrPropertyNameTooWide	= 0x80041073,
        wbemErrClassNameTooWide	= 0x80041074,
        wbemErrMethodNameTooWide	= 0x80041075,
        wbemErrQualifierNameTooWide	= 0x80041076,
        wbemErrRerunCommand	= 0x80041077,
        wbemErrDatabaseVerMismatch	= 0x80041078,
        wbemErrVetoPut	= 0x80041079,
        wbemErrVetoDelete	= 0x8004107a,
        wbemErrInvalidLocale	= 0x80041080,
        wbemErrProviderSuspended	= 0x80041081,
        wbemErrSynchronizationRequired	= 0x80041082,
        wbemErrNoSchema	= 0x80041083,
        wbemErrProviderAlreadyRegistered	= 0x80041084,
        wbemErrProviderNotRegistered	= 0x80041085,
        wbemErrFatalTransportError	= 0x80041086,
        wbemErrEncryptedConnectionRequired	= 0x80041087,
        wbemErrRegistrationTooBroad	= 0x80042001,
        wbemErrRegistrationTooPrecise	= 0x80042002,
        wbemErrTimedout	= 0x80043001,
        wbemErrResetToDefault	= 0x80043002
    } 	WbemErrorEnum;

typedef /* [helpstring][uuid][v1_enum] */  DECLSPEC_UUID("B54D66E7-2287-11d2-8B33-00600806D9B6") 
enum WbemAuthenticationLevelEnum
    {
        wbemAuthenticationLevelDefault	= 0,
        wbemAuthenticationLevelNone	= 1,
        wbemAuthenticationLevelConnect	= 2,
        wbemAuthenticationLevelCall	= 3,
        wbemAuthenticationLevelPkt	= 4,
        wbemAuthenticationLevelPktIntegrity	= 5,
        wbemAuthenticationLevelPktPrivacy	= 6
    } 	WbemAuthenticationLevelEnum;

typedef /* [helpstring][uuid][v1_enum] */  DECLSPEC_UUID("B54D66E8-2287-11d2-8B33-00600806D9B6") 
enum WbemImpersonationLevelEnum
    {
        wbemImpersonationLevelAnonymous	= 1,
        wbemImpersonationLevelIdentify	= 2,
        wbemImpersonationLevelImpersonate	= 3,
        wbemImpersonationLevelDelegate	= 4
    } 	WbemImpersonationLevelEnum;

typedef /* [helpstring][uuid][v1_enum] */  DECLSPEC_UUID("176D2F70-5AF3-11d2-8B4A-00600806D9B6") 
enum WbemPrivilegeEnum
    {
        wbemPrivilegeCreateToken	= 1,
        wbemPrivilegePrimaryToken	= 2,
        wbemPrivilegeLockMemory	= 3,
        wbemPrivilegeIncreaseQuota	= 4,
        wbemPrivilegeMachineAccount	= 5,
        wbemPrivilegeTcb	= 6,
        wbemPrivilegeSecurity	= 7,
        wbemPrivilegeTakeOwnership	= 8,
        wbemPrivilegeLoadDriver	= 9,
        wbemPrivilegeSystemProfile	= 10,
        wbemPrivilegeSystemtime	= 11,
        wbemPrivilegeProfileSingleProcess	= 12,
        wbemPrivilegeIncreaseBasePriority	= 13,
        wbemPrivilegeCreatePagefile	= 14,
        wbemPrivilegeCreatePermanent	= 15,
        wbemPrivilegeBackup	= 16,
        wbemPrivilegeRestore	= 17,
        wbemPrivilegeShutdown	= 18,
        wbemPrivilegeDebug	= 19,
        wbemPrivilegeAudit	= 20,
        wbemPrivilegeSystemEnvironment	= 21,
        wbemPrivilegeChangeNotify	= 22,
        wbemPrivilegeRemoteShutdown	= 23,
        wbemPrivilegeUndock	= 24,
        wbemPrivilegeSyncAgent	= 25,
        wbemPrivilegeEnableDelegation	= 26,
        wbemPrivilegeManageVolume	= 27
    } 	WbemPrivilegeEnum;

typedef /* [helpstring][uuid][v1_enum] */  DECLSPEC_UUID("09FF1992-EA0E-11d3-B391-00105A1F473A") 
enum WbemObjectTextFormatEnum
    {
        wbemObjectTextFormatCIMDTD20	= 1,
        wbemObjectTextFormatWMIDTD20	= 2
    } 	WbemObjectTextFormatEnum;

typedef /* [helpstring][v1_enum] */ 
enum WbemConnectOptionsEnum
    {
        wbemConnectFlagUseMaxWait	= 0x80
    } 	WbemConnectOptionsEnum;


EXTERN_C const IID LIBID_WbemScripting;

#ifndef __ISWbemServices_INTERFACE_DEFINED__
#define __ISWbemServices_INTERFACE_DEFINED__

/* interface ISWbemServices */
/* [helpstring][hidden][unique][dual][oleautomation][uuid][object][local] */ 


EXTERN_C const IID IID_ISWbemServices;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("76A6415C-CB41-11d1-8B02-00600806D9B6")
    ISWbemServices : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Get( 
            /* [defaultvalue][in] */ BSTR strObjectPath,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [retval][out] */ ISWbemObject **objWbemObject) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetAsync( 
            /* [in] */ IDispatch *objWbemSink,
            /* [defaultvalue][in] */ BSTR strObjectPath = (BSTR)L"",
            /* [defaultvalue][in] */ long iFlags = 0,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet = 0,
            /* [defaultvalue][in] */ IDispatch *objWbemAsyncContext = 0) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Delete( 
            /* [in] */ BSTR strObjectPath,
            /* [defaultvalue][in] */ long iFlags = 0,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet = 0) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE DeleteAsync( 
            /* [in] */ IDispatch *objWbemSink,
            /* [in] */ BSTR strObjectPath,
            /* [defaultvalue][in] */ long iFlags = 0,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet = 0,
            /* [defaultvalue][in] */ IDispatch *objWbemAsyncContext = 0) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE InstancesOf( 
            /* [in] */ BSTR strClass,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [retval][out] */ ISWbemObjectSet **objWbemObjectSet) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE InstancesOfAsync( 
            /* [in] */ IDispatch *objWbemSink,
            /* [in] */ BSTR strClass,
            /* [defaultvalue][in] */ long iFlags = 0,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet = 0,
            /* [defaultvalue][in] */ IDispatch *objWbemAsyncContext = 0) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SubclassesOf( 
            /* [defaultvalue][in] */ BSTR strSuperclass,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [retval][out] */ ISWbemObjectSet **objWbemObjectSet) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SubclassesOfAsync( 
            /* [in] */ IDispatch *objWbemSink,
            /* [defaultvalue][in] */ BSTR strSuperclass = (BSTR)L"",
            /* [defaultvalue][in] */ long iFlags = wbemQueryFlagDeep,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet = 0,
            /* [defaultvalue][in] */ IDispatch *objWbemAsyncContext = 0) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ExecQuery( 
            /* [in] */ BSTR strQuery,
            /* [defaultvalue][in] */ BSTR strQueryLanguage,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [retval][out] */ ISWbemObjectSet **objWbemObjectSet) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ExecQueryAsync( 
            /* [in] */ IDispatch *objWbemSink,
            /* [in] */ BSTR strQuery,
            /* [defaultvalue][in] */ BSTR strQueryLanguage = (BSTR)L"WQL",
            /* [defaultvalue][in] */ long lFlags = 0,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet = 0,
            /* [defaultvalue][in] */ IDispatch *objWbemAsyncContext = 0) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE AssociatorsOf( 
            /* [in] */ BSTR strObjectPath,
            /* [defaultvalue][in] */ BSTR strAssocClass,
            /* [defaultvalue][in] */ BSTR strResultClass,
            /* [defaultvalue][in] */ BSTR strResultRole,
            /* [defaultvalue][in] */ BSTR strRole,
            /* [defaultvalue][in] */ VARIANT_BOOL bClassesOnly,
            /* [defaultvalue][in] */ VARIANT_BOOL bSchemaOnly,
            /* [defaultvalue][in] */ BSTR strRequiredAssocQualifier,
            /* [defaultvalue][in] */ BSTR strRequiredQualifier,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [retval][out] */ ISWbemObjectSet **objWbemObjectSet) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE AssociatorsOfAsync( 
            /* [in] */ IDispatch *objWbemSink,
            /* [in] */ BSTR strObjectPath,
            /* [defaultvalue][in] */ BSTR strAssocClass = (BSTR)L"",
            /* [defaultvalue][in] */ BSTR strResultClass = (BSTR)L"",
            /* [defaultvalue][in] */ BSTR strResultRole = (BSTR)L"",
            /* [defaultvalue][in] */ BSTR strRole = (BSTR)L"",
            /* [defaultvalue][in] */ VARIANT_BOOL bClassesOnly = FALSE,
            /* [defaultvalue][in] */ VARIANT_BOOL bSchemaOnly = FALSE,
            /* [defaultvalue][in] */ BSTR strRequiredAssocQualifier = (BSTR)L"",
            /* [defaultvalue][in] */ BSTR strRequiredQualifier = (BSTR)L"",
            /* [defaultvalue][in] */ long iFlags = 0,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet = 0,
            /* [defaultvalue][in] */ IDispatch *objWbemAsyncContext = 0) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ReferencesTo( 
            /* [in] */ BSTR strObjectPath,
            /* [defaultvalue][in] */ BSTR strResultClass,
            /* [defaultvalue][in] */ BSTR strRole,
            /* [defaultvalue][in] */ VARIANT_BOOL bClassesOnly,
            /* [defaultvalue][in] */ VARIANT_BOOL bSchemaOnly,
            /* [defaultvalue][in] */ BSTR strRequiredQualifier,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [retval][out] */ ISWbemObjectSet **objWbemObjectSet) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ReferencesToAsync( 
            /* [in] */ IDispatch *objWbemSink,
            /* [in] */ BSTR strObjectPath,
            /* [defaultvalue][in] */ BSTR strResultClass = (BSTR)L"",
            /* [defaultvalue][in] */ BSTR strRole = (BSTR)L"",
            /* [defaultvalue][in] */ VARIANT_BOOL bClassesOnly = FALSE,
            /* [defaultvalue][in] */ VARIANT_BOOL bSchemaOnly = FALSE,
            /* [defaultvalue][in] */ BSTR strRequiredQualifier = (BSTR)L"",
            /* [defaultvalue][in] */ long iFlags = 0,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet = 0,
            /* [defaultvalue][in] */ IDispatch *objWbemAsyncContext = 0) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ExecNotificationQuery( 
            /* [in] */ BSTR strQuery,
            /* [defaultvalue][in] */ BSTR strQueryLanguage,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [retval][out] */ ISWbemEventSource **objWbemEventSource) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ExecNotificationQueryAsync( 
            /* [in] */ IDispatch *objWbemSink,
            /* [in] */ BSTR strQuery,
            /* [defaultvalue][in] */ BSTR strQueryLanguage = (BSTR)L"WQL",
            /* [defaultvalue][in] */ long iFlags = 0,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet = 0,
            /* [defaultvalue][in] */ IDispatch *objWbemAsyncContext = 0) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ExecMethod( 
            /* [in] */ BSTR strObjectPath,
            /* [in] */ BSTR strMethodName,
            /* [defaultvalue][in] */ IDispatch *objWbemInParameters,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [retval][out] */ ISWbemObject **objWbemOutParameters) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ExecMethodAsync( 
            /* [in] */ IDispatch *objWbemSink,
            /* [in] */ BSTR strObjectPath,
            /* [in] */ BSTR strMethodName,
            /* [defaultvalue][in] */ IDispatch *objWbemInParameters = 0,
            /* [defaultvalue][in] */ long iFlags = 0,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet = 0,
            /* [defaultvalue][in] */ IDispatch *objWbemAsyncContext = 0) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Security_( 
            /* [retval][out] */ ISWbemSecurity **objWbemSecurity) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISWbemServicesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISWbemServices * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISWbemServices * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISWbemServices * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            ISWbemServices * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            ISWbemServices * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            ISWbemServices * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISWbemServices * This,
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
        
        DECLSPEC_XFGVIRT(ISWbemServices, Get)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Get )( 
            ISWbemServices * This,
            /* [defaultvalue][in] */ BSTR strObjectPath,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [retval][out] */ ISWbemObject **objWbemObject);
        
        DECLSPEC_XFGVIRT(ISWbemServices, GetAsync)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetAsync )( 
            ISWbemServices * This,
            /* [in] */ IDispatch *objWbemSink,
            /* [defaultvalue][in] */ BSTR strObjectPath,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [defaultvalue][in] */ IDispatch *objWbemAsyncContext);
        
        DECLSPEC_XFGVIRT(ISWbemServices, Delete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            ISWbemServices * This,
            /* [in] */ BSTR strObjectPath,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet);
        
        DECLSPEC_XFGVIRT(ISWbemServices, DeleteAsync)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *DeleteAsync )( 
            ISWbemServices * This,
            /* [in] */ IDispatch *objWbemSink,
            /* [in] */ BSTR strObjectPath,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [defaultvalue][in] */ IDispatch *objWbemAsyncContext);
        
        DECLSPEC_XFGVIRT(ISWbemServices, InstancesOf)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *InstancesOf )( 
            ISWbemServices * This,
            /* [in] */ BSTR strClass,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [retval][out] */ ISWbemObjectSet **objWbemObjectSet);
        
        DECLSPEC_XFGVIRT(ISWbemServices, InstancesOfAsync)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *InstancesOfAsync )( 
            ISWbemServices * This,
            /* [in] */ IDispatch *objWbemSink,
            /* [in] */ BSTR strClass,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [defaultvalue][in] */ IDispatch *objWbemAsyncContext);
        
        DECLSPEC_XFGVIRT(ISWbemServices, SubclassesOf)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SubclassesOf )( 
            ISWbemServices * This,
            /* [defaultvalue][in] */ BSTR strSuperclass,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [retval][out] */ ISWbemObjectSet **objWbemObjectSet);
        
        DECLSPEC_XFGVIRT(ISWbemServices, SubclassesOfAsync)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SubclassesOfAsync )( 
            ISWbemServices * This,
            /* [in] */ IDispatch *objWbemSink,
            /* [defaultvalue][in] */ BSTR strSuperclass,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [defaultvalue][in] */ IDispatch *objWbemAsyncContext);
        
        DECLSPEC_XFGVIRT(ISWbemServices, ExecQuery)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ExecQuery )( 
            ISWbemServices * This,
            /* [in] */ BSTR strQuery,
            /* [defaultvalue][in] */ BSTR strQueryLanguage,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [retval][out] */ ISWbemObjectSet **objWbemObjectSet);
        
        DECLSPEC_XFGVIRT(ISWbemServices, ExecQueryAsync)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ExecQueryAsync )( 
            ISWbemServices * This,
            /* [in] */ IDispatch *objWbemSink,
            /* [in] */ BSTR strQuery,
            /* [defaultvalue][in] */ BSTR strQueryLanguage,
            /* [defaultvalue][in] */ long lFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [defaultvalue][in] */ IDispatch *objWbemAsyncContext);
        
        DECLSPEC_XFGVIRT(ISWbemServices, AssociatorsOf)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *AssociatorsOf )( 
            ISWbemServices * This,
            /* [in] */ BSTR strObjectPath,
            /* [defaultvalue][in] */ BSTR strAssocClass,
            /* [defaultvalue][in] */ BSTR strResultClass,
            /* [defaultvalue][in] */ BSTR strResultRole,
            /* [defaultvalue][in] */ BSTR strRole,
            /* [defaultvalue][in] */ VARIANT_BOOL bClassesOnly,
            /* [defaultvalue][in] */ VARIANT_BOOL bSchemaOnly,
            /* [defaultvalue][in] */ BSTR strRequiredAssocQualifier,
            /* [defaultvalue][in] */ BSTR strRequiredQualifier,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [retval][out] */ ISWbemObjectSet **objWbemObjectSet);
        
        DECLSPEC_XFGVIRT(ISWbemServices, AssociatorsOfAsync)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *AssociatorsOfAsync )( 
            ISWbemServices * This,
            /* [in] */ IDispatch *objWbemSink,
            /* [in] */ BSTR strObjectPath,
            /* [defaultvalue][in] */ BSTR strAssocClass,
            /* [defaultvalue][in] */ BSTR strResultClass,
            /* [defaultvalue][in] */ BSTR strResultRole,
            /* [defaultvalue][in] */ BSTR strRole,
            /* [defaultvalue][in] */ VARIANT_BOOL bClassesOnly,
            /* [defaultvalue][in] */ VARIANT_BOOL bSchemaOnly,
            /* [defaultvalue][in] */ BSTR strRequiredAssocQualifier,
            /* [defaultvalue][in] */ BSTR strRequiredQualifier,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [defaultvalue][in] */ IDispatch *objWbemAsyncContext);
        
        DECLSPEC_XFGVIRT(ISWbemServices, ReferencesTo)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ReferencesTo )( 
            ISWbemServices * This,
            /* [in] */ BSTR strObjectPath,
            /* [defaultvalue][in] */ BSTR strResultClass,
            /* [defaultvalue][in] */ BSTR strRole,
            /* [defaultvalue][in] */ VARIANT_BOOL bClassesOnly,
            /* [defaultvalue][in] */ VARIANT_BOOL bSchemaOnly,
            /* [defaultvalue][in] */ BSTR strRequiredQualifier,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [retval][out] */ ISWbemObjectSet **objWbemObjectSet);
        
        DECLSPEC_XFGVIRT(ISWbemServices, ReferencesToAsync)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ReferencesToAsync )( 
            ISWbemServices * This,
            /* [in] */ IDispatch *objWbemSink,
            /* [in] */ BSTR strObjectPath,
            /* [defaultvalue][in] */ BSTR strResultClass,
            /* [defaultvalue][in] */ BSTR strRole,
            /* [defaultvalue][in] */ VARIANT_BOOL bClassesOnly,
            /* [defaultvalue][in] */ VARIANT_BOOL bSchemaOnly,
            /* [defaultvalue][in] */ BSTR strRequiredQualifier,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [defaultvalue][in] */ IDispatch *objWbemAsyncContext);
        
        DECLSPEC_XFGVIRT(ISWbemServices, ExecNotificationQuery)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ExecNotificationQuery )( 
            ISWbemServices * This,
            /* [in] */ BSTR strQuery,
            /* [defaultvalue][in] */ BSTR strQueryLanguage,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [retval][out] */ ISWbemEventSource **objWbemEventSource);
        
        DECLSPEC_XFGVIRT(ISWbemServices, ExecNotificationQueryAsync)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ExecNotificationQueryAsync )( 
            ISWbemServices * This,
            /* [in] */ IDispatch *objWbemSink,
            /* [in] */ BSTR strQuery,
            /* [defaultvalue][in] */ BSTR strQueryLanguage,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [defaultvalue][in] */ IDispatch *objWbemAsyncContext);
        
        DECLSPEC_XFGVIRT(ISWbemServices, ExecMethod)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ExecMethod )( 
            ISWbemServices * This,
            /* [in] */ BSTR strObjectPath,
            /* [in] */ BSTR strMethodName,
            /* [defaultvalue][in] */ IDispatch *objWbemInParameters,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [retval][out] */ ISWbemObject **objWbemOutParameters);
        
        DECLSPEC_XFGVIRT(ISWbemServices, ExecMethodAsync)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ExecMethodAsync )( 
            ISWbemServices * This,
            /* [in] */ IDispatch *objWbemSink,
            /* [in] */ BSTR strObjectPath,
            /* [in] */ BSTR strMethodName,
            /* [defaultvalue][in] */ IDispatch *objWbemInParameters,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [defaultvalue][in] */ IDispatch *objWbemAsyncContext);
        
        DECLSPEC_XFGVIRT(ISWbemServices, get_Security_)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Security_ )( 
            ISWbemServices * This,
            /* [retval][out] */ ISWbemSecurity **objWbemSecurity);
        
        END_INTERFACE
    } ISWbemServicesVtbl;

    interface ISWbemServices
    {
        CONST_VTBL struct ISWbemServicesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISWbemServices_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISWbemServices_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISWbemServices_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISWbemServices_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISWbemServices_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISWbemServices_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISWbemServices_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISWbemServices_Get(This,strObjectPath,iFlags,objWbemNamedValueSet,objWbemObject)	\
    ( (This)->lpVtbl -> Get(This,strObjectPath,iFlags,objWbemNamedValueSet,objWbemObject) ) 

#define ISWbemServices_GetAsync(This,objWbemSink,strObjectPath,iFlags,objWbemNamedValueSet,objWbemAsyncContext)	\
    ( (This)->lpVtbl -> GetAsync(This,objWbemSink,strObjectPath,iFlags,objWbemNamedValueSet,objWbemAsyncContext) ) 

#define ISWbemServices_Delete(This,strObjectPath,iFlags,objWbemNamedValueSet)	\
    ( (This)->lpVtbl -> Delete(This,strObjectPath,iFlags,objWbemNamedValueSet) ) 

#define ISWbemServices_DeleteAsync(This,objWbemSink,strObjectPath,iFlags,objWbemNamedValueSet,objWbemAsyncContext)	\
    ( (This)->lpVtbl -> DeleteAsync(This,objWbemSink,strObjectPath,iFlags,objWbemNamedValueSet,objWbemAsyncContext) ) 

#define ISWbemServices_InstancesOf(This,strClass,iFlags,objWbemNamedValueSet,objWbemObjectSet)	\
    ( (This)->lpVtbl -> InstancesOf(This,strClass,iFlags,objWbemNamedValueSet,objWbemObjectSet) ) 

#define ISWbemServices_InstancesOfAsync(This,objWbemSink,strClass,iFlags,objWbemNamedValueSet,objWbemAsyncContext)	\
    ( (This)->lpVtbl -> InstancesOfAsync(This,objWbemSink,strClass,iFlags,objWbemNamedValueSet,objWbemAsyncContext) ) 

#define ISWbemServices_SubclassesOf(This,strSuperclass,iFlags,objWbemNamedValueSet,objWbemObjectSet)	\
    ( (This)->lpVtbl -> SubclassesOf(This,strSuperclass,iFlags,objWbemNamedValueSet,objWbemObjectSet) ) 

#define ISWbemServices_SubclassesOfAsync(This,objWbemSink,strSuperclass,iFlags,objWbemNamedValueSet,objWbemAsyncContext)	\
    ( (This)->lpVtbl -> SubclassesOfAsync(This,objWbemSink,strSuperclass,iFlags,objWbemNamedValueSet,objWbemAsyncContext) ) 

#define ISWbemServices_ExecQuery(This,strQuery,strQueryLanguage,iFlags,objWbemNamedValueSet,objWbemObjectSet)	\
    ( (This)->lpVtbl -> ExecQuery(This,strQuery,strQueryLanguage,iFlags,objWbemNamedValueSet,objWbemObjectSet) ) 

#define ISWbemServices_ExecQueryAsync(This,objWbemSink,strQuery,strQueryLanguage,lFlags,objWbemNamedValueSet,objWbemAsyncContext)	\
    ( (This)->lpVtbl -> ExecQueryAsync(This,objWbemSink,strQuery,strQueryLanguage,lFlags,objWbemNamedValueSet,objWbemAsyncContext) ) 

#define ISWbemServices_AssociatorsOf(This,strObjectPath,strAssocClass,strResultClass,strResultRole,strRole,bClassesOnly,bSchemaOnly,strRequiredAssocQualifier,strRequiredQualifier,iFlags,objWbemNamedValueSet,objWbemObjectSet)	\
    ( (This)->lpVtbl -> AssociatorsOf(This,strObjectPath,strAssocClass,strResultClass,strResultRole,strRole,bClassesOnly,bSchemaOnly,strRequiredAssocQualifier,strRequiredQualifier,iFlags,objWbemNamedValueSet,objWbemObjectSet) ) 

#define ISWbemServices_AssociatorsOfAsync(This,objWbemSink,strObjectPath,strAssocClass,strResultClass,strResultRole,strRole,bClassesOnly,bSchemaOnly,strRequiredAssocQualifier,strRequiredQualifier,iFlags,objWbemNamedValueSet,objWbemAsyncContext)	\
    ( (This)->lpVtbl -> AssociatorsOfAsync(This,objWbemSink,strObjectPath,strAssocClass,strResultClass,strResultRole,strRole,bClassesOnly,bSchemaOnly,strRequiredAssocQualifier,strRequiredQualifier,iFlags,objWbemNamedValueSet,objWbemAsyncContext) ) 

#define ISWbemServices_ReferencesTo(This,strObjectPath,strResultClass,strRole,bClassesOnly,bSchemaOnly,strRequiredQualifier,iFlags,objWbemNamedValueSet,objWbemObjectSet)	\
    ( (This)->lpVtbl -> ReferencesTo(This,strObjectPath,strResultClass,strRole,bClassesOnly,bSchemaOnly,strRequiredQualifier,iFlags,objWbemNamedValueSet,objWbemObjectSet) ) 

#define ISWbemServices_ReferencesToAsync(This,objWbemSink,strObjectPath,strResultClass,strRole,bClassesOnly,bSchemaOnly,strRequiredQualifier,iFlags,objWbemNamedValueSet,objWbemAsyncContext)	\
    ( (This)->lpVtbl -> ReferencesToAsync(This,objWbemSink,strObjectPath,strResultClass,strRole,bClassesOnly,bSchemaOnly,strRequiredQualifier,iFlags,objWbemNamedValueSet,objWbemAsyncContext) ) 

#define ISWbemServices_ExecNotificationQuery(This,strQuery,strQueryLanguage,iFlags,objWbemNamedValueSet,objWbemEventSource)	\
    ( (This)->lpVtbl -> ExecNotificationQuery(This,strQuery,strQueryLanguage,iFlags,objWbemNamedValueSet,objWbemEventSource) ) 

#define ISWbemServices_ExecNotificationQueryAsync(This,objWbemSink,strQuery,strQueryLanguage,iFlags,objWbemNamedValueSet,objWbemAsyncContext)	\
    ( (This)->lpVtbl -> ExecNotificationQueryAsync(This,objWbemSink,strQuery,strQueryLanguage,iFlags,objWbemNamedValueSet,objWbemAsyncContext) ) 

#define ISWbemServices_ExecMethod(This,strObjectPath,strMethodName,objWbemInParameters,iFlags,objWbemNamedValueSet,objWbemOutParameters)	\
    ( (This)->lpVtbl -> ExecMethod(This,strObjectPath,strMethodName,objWbemInParameters,iFlags,objWbemNamedValueSet,objWbemOutParameters) ) 

#define ISWbemServices_ExecMethodAsync(This,objWbemSink,strObjectPath,strMethodName,objWbemInParameters,iFlags,objWbemNamedValueSet,objWbemAsyncContext)	\
    ( (This)->lpVtbl -> ExecMethodAsync(This,objWbemSink,strObjectPath,strMethodName,objWbemInParameters,iFlags,objWbemNamedValueSet,objWbemAsyncContext) ) 

#define ISWbemServices_get_Security_(This,objWbemSecurity)	\
    ( (This)->lpVtbl -> get_Security_(This,objWbemSecurity) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISWbemServices_INTERFACE_DEFINED__ */


#ifndef __ISWbemLocator_INTERFACE_DEFINED__
#define __ISWbemLocator_INTERFACE_DEFINED__

/* interface ISWbemLocator */
/* [helpstring][unique][oleautomation][hidden][dual][uuid][local][object] */ 


EXTERN_C const IID IID_ISWbemLocator;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("76A6415B-CB41-11d1-8B02-00600806D9B6")
    ISWbemLocator : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ConnectServer( 
            /* [defaultvalue][in] */ BSTR strServer,
            /* [defaultvalue][in] */ BSTR strNamespace,
            /* [defaultvalue][in] */ BSTR strUser,
            /* [defaultvalue][in] */ BSTR strPassword,
            /* [defaultvalue][in] */ BSTR strLocale,
            /* [defaultvalue][in] */ BSTR strAuthority,
            /* [defaultvalue][in] */ long iSecurityFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [retval][out] */ ISWbemServices **objWbemServices) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Security_( 
            /* [retval][out] */ ISWbemSecurity **objWbemSecurity) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISWbemLocatorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISWbemLocator * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISWbemLocator * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISWbemLocator * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            ISWbemLocator * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            ISWbemLocator * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            ISWbemLocator * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISWbemLocator * This,
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
        
        DECLSPEC_XFGVIRT(ISWbemLocator, ConnectServer)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ConnectServer )( 
            ISWbemLocator * This,
            /* [defaultvalue][in] */ BSTR strServer,
            /* [defaultvalue][in] */ BSTR strNamespace,
            /* [defaultvalue][in] */ BSTR strUser,
            /* [defaultvalue][in] */ BSTR strPassword,
            /* [defaultvalue][in] */ BSTR strLocale,
            /* [defaultvalue][in] */ BSTR strAuthority,
            /* [defaultvalue][in] */ long iSecurityFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [retval][out] */ ISWbemServices **objWbemServices);
        
        DECLSPEC_XFGVIRT(ISWbemLocator, get_Security_)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Security_ )( 
            ISWbemLocator * This,
            /* [retval][out] */ ISWbemSecurity **objWbemSecurity);
        
        END_INTERFACE
    } ISWbemLocatorVtbl;

    interface ISWbemLocator
    {
        CONST_VTBL struct ISWbemLocatorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISWbemLocator_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISWbemLocator_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISWbemLocator_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISWbemLocator_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISWbemLocator_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISWbemLocator_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISWbemLocator_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISWbemLocator_ConnectServer(This,strServer,strNamespace,strUser,strPassword,strLocale,strAuthority,iSecurityFlags,objWbemNamedValueSet,objWbemServices)	\
    ( (This)->lpVtbl -> ConnectServer(This,strServer,strNamespace,strUser,strPassword,strLocale,strAuthority,iSecurityFlags,objWbemNamedValueSet,objWbemServices) ) 

#define ISWbemLocator_get_Security_(This,objWbemSecurity)	\
    ( (This)->lpVtbl -> get_Security_(This,objWbemSecurity) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISWbemLocator_INTERFACE_DEFINED__ */


#ifndef __ISWbemObject_INTERFACE_DEFINED__
#define __ISWbemObject_INTERFACE_DEFINED__

/* interface ISWbemObject */
/* [helpstring][hidden][oleautomation][dual][uuid][object][local] */ 

#define	WBEMS_DISPID_DERIVATION	( 23 )


EXTERN_C const IID IID_ISWbemObject;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("76A6415A-CB41-11d1-8B02-00600806D9B6")
    ISWbemObject : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Put_( 
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [retval][out] */ ISWbemObjectPath **objWbemObjectPath) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE PutAsync_( 
            /* [in] */ IDispatch *objWbemSink,
            /* [defaultvalue][in] */ long iFlags = wbemChangeFlagCreateOrUpdate,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet = 0,
            /* [defaultvalue][in] */ IDispatch *objWbemAsyncContext = 0) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Delete_( 
            /* [defaultvalue][in] */ long iFlags = 0,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet = 0) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE DeleteAsync_( 
            /* [in] */ IDispatch *objWbemSink,
            /* [defaultvalue][in] */ long iFlags = 0,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet = 0,
            /* [defaultvalue][in] */ IDispatch *objWbemAsyncContext = 0) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Instances_( 
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [retval][out] */ ISWbemObjectSet **objWbemObjectSet) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE InstancesAsync_( 
            /* [in] */ IDispatch *objWbemSink,
            /* [defaultvalue][in] */ long iFlags = 0,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet = 0,
            /* [defaultvalue][in] */ IDispatch *objWbemAsyncContext = 0) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Subclasses_( 
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [retval][out] */ ISWbemObjectSet **objWbemObjectSet) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SubclassesAsync_( 
            /* [in] */ IDispatch *objWbemSink,
            /* [defaultvalue][in] */ long iFlags = wbemQueryFlagDeep,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet = 0,
            /* [defaultvalue][in] */ IDispatch *objWbemAsyncContext = 0) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Associators_( 
            /* [defaultvalue][in] */ BSTR strAssocClass,
            /* [defaultvalue][in] */ BSTR strResultClass,
            /* [defaultvalue][in] */ BSTR strResultRole,
            /* [defaultvalue][in] */ BSTR strRole,
            /* [defaultvalue][in] */ VARIANT_BOOL bClassesOnly,
            /* [defaultvalue][in] */ VARIANT_BOOL bSchemaOnly,
            /* [defaultvalue][in] */ BSTR strRequiredAssocQualifier,
            /* [defaultvalue][in] */ BSTR strRequiredQualifier,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [retval][out] */ ISWbemObjectSet **objWbemObjectSet) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE AssociatorsAsync_( 
            /* [in] */ IDispatch *objWbemSink,
            /* [defaultvalue][in] */ BSTR strAssocClass = (BSTR)L"",
            /* [defaultvalue][in] */ BSTR strResultClass = (BSTR)L"",
            /* [defaultvalue][in] */ BSTR strResultRole = (BSTR)L"",
            /* [defaultvalue][in] */ BSTR strRole = (BSTR)L"",
            /* [defaultvalue][in] */ VARIANT_BOOL bClassesOnly = FALSE,
            /* [defaultvalue][in] */ VARIANT_BOOL bSchemaOnly = FALSE,
            /* [defaultvalue][in] */ BSTR strRequiredAssocQualifier = (BSTR)L"",
            /* [defaultvalue][in] */ BSTR strRequiredQualifier = (BSTR)L"",
            /* [defaultvalue][in] */ long iFlags = 0,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet = 0,
            /* [defaultvalue][in] */ IDispatch *objWbemAsyncContext = 0) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE References_( 
            /* [defaultvalue][in] */ BSTR strResultClass,
            /* [defaultvalue][in] */ BSTR strRole,
            /* [defaultvalue][in] */ VARIANT_BOOL bClassesOnly,
            /* [defaultvalue][in] */ VARIANT_BOOL bSchemaOnly,
            /* [defaultvalue][in] */ BSTR strRequiredQualifier,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [retval][out] */ ISWbemObjectSet **objWbemObjectSet) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ReferencesAsync_( 
            /* [in] */ IDispatch *objWbemSink,
            /* [defaultvalue][in] */ BSTR strResultClass = (BSTR)L"",
            /* [defaultvalue][in] */ BSTR strRole = (BSTR)L"",
            /* [defaultvalue][in] */ VARIANT_BOOL bClassesOnly = FALSE,
            /* [defaultvalue][in] */ VARIANT_BOOL bSchemaOnly = FALSE,
            /* [defaultvalue][in] */ BSTR strRequiredQualifier = (BSTR)L"",
            /* [defaultvalue][in] */ long iFlags = 0,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet = 0,
            /* [defaultvalue][in] */ IDispatch *objWbemAsyncContext = 0) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ExecMethod_( 
            /* [in] */ BSTR strMethodName,
            /* [defaultvalue][in] */ IDispatch *objWbemInParameters,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [retval][out] */ ISWbemObject **objWbemOutParameters) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ExecMethodAsync_( 
            /* [in] */ IDispatch *objWbemSink,
            /* [in] */ BSTR strMethodName,
            /* [defaultvalue][in] */ IDispatch *objWbemInParameters = 0,
            /* [defaultvalue][in] */ long iFlags = 0,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet = 0,
            /* [defaultvalue][in] */ IDispatch *objWbemAsyncContext = 0) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Clone_( 
            /* [retval][out] */ ISWbemObject **objWbemObject) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetObjectText_( 
            /* [defaultvalue][in] */ long iFlags,
            /* [retval][out] */ BSTR *strObjectText) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SpawnDerivedClass_( 
            /* [defaultvalue][in] */ long iFlags,
            /* [retval][out] */ ISWbemObject **objWbemObject) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SpawnInstance_( 
            /* [defaultvalue][in] */ long iFlags,
            /* [retval][out] */ ISWbemObject **objWbemObject) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CompareTo_( 
            /* [in] */ IDispatch *objWbemObject,
            /* [defaultvalue][in] */ long iFlags,
            /* [retval][out] */ VARIANT_BOOL *bResult) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Qualifiers_( 
            /* [retval][out] */ ISWbemQualifierSet **objWbemQualifierSet) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Properties_( 
            /* [retval][out] */ ISWbemPropertySet **objWbemPropertySet) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Methods_( 
            /* [retval][out] */ ISWbemMethodSet **objWbemMethodSet) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Derivation_( 
            /* [retval][out] */ VARIANT *strClassNameArray) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Path_( 
            /* [retval][out] */ ISWbemObjectPath **objWbemObjectPath) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Security_( 
            /* [retval][out] */ ISWbemSecurity **objWbemSecurity) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISWbemObjectVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISWbemObject * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISWbemObject * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISWbemObject * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            ISWbemObject * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            ISWbemObject * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            ISWbemObject * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISWbemObject * This,
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
        
        DECLSPEC_XFGVIRT(ISWbemObject, Put_)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Put_ )( 
            ISWbemObject * This,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [retval][out] */ ISWbemObjectPath **objWbemObjectPath);
        
        DECLSPEC_XFGVIRT(ISWbemObject, PutAsync_)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *PutAsync_ )( 
            ISWbemObject * This,
            /* [in] */ IDispatch *objWbemSink,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [defaultvalue][in] */ IDispatch *objWbemAsyncContext);
        
        DECLSPEC_XFGVIRT(ISWbemObject, Delete_)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Delete_ )( 
            ISWbemObject * This,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet);
        
        DECLSPEC_XFGVIRT(ISWbemObject, DeleteAsync_)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *DeleteAsync_ )( 
            ISWbemObject * This,
            /* [in] */ IDispatch *objWbemSink,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [defaultvalue][in] */ IDispatch *objWbemAsyncContext);
        
        DECLSPEC_XFGVIRT(ISWbemObject, Instances_)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Instances_ )( 
            ISWbemObject * This,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [retval][out] */ ISWbemObjectSet **objWbemObjectSet);
        
        DECLSPEC_XFGVIRT(ISWbemObject, InstancesAsync_)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *InstancesAsync_ )( 
            ISWbemObject * This,
            /* [in] */ IDispatch *objWbemSink,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [defaultvalue][in] */ IDispatch *objWbemAsyncContext);
        
        DECLSPEC_XFGVIRT(ISWbemObject, Subclasses_)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Subclasses_ )( 
            ISWbemObject * This,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [retval][out] */ ISWbemObjectSet **objWbemObjectSet);
        
        DECLSPEC_XFGVIRT(ISWbemObject, SubclassesAsync_)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SubclassesAsync_ )( 
            ISWbemObject * This,
            /* [in] */ IDispatch *objWbemSink,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [defaultvalue][in] */ IDispatch *objWbemAsyncContext);
        
        DECLSPEC_XFGVIRT(ISWbemObject, Associators_)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Associators_ )( 
            ISWbemObject * This,
            /* [defaultvalue][in] */ BSTR strAssocClass,
            /* [defaultvalue][in] */ BSTR strResultClass,
            /* [defaultvalue][in] */ BSTR strResultRole,
            /* [defaultvalue][in] */ BSTR strRole,
            /* [defaultvalue][in] */ VARIANT_BOOL bClassesOnly,
            /* [defaultvalue][in] */ VARIANT_BOOL bSchemaOnly,
            /* [defaultvalue][in] */ BSTR strRequiredAssocQualifier,
            /* [defaultvalue][in] */ BSTR strRequiredQualifier,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [retval][out] */ ISWbemObjectSet **objWbemObjectSet);
        
        DECLSPEC_XFGVIRT(ISWbemObject, AssociatorsAsync_)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *AssociatorsAsync_ )( 
            ISWbemObject * This,
            /* [in] */ IDispatch *objWbemSink,
            /* [defaultvalue][in] */ BSTR strAssocClass,
            /* [defaultvalue][in] */ BSTR strResultClass,
            /* [defaultvalue][in] */ BSTR strResultRole,
            /* [defaultvalue][in] */ BSTR strRole,
            /* [defaultvalue][in] */ VARIANT_BOOL bClassesOnly,
            /* [defaultvalue][in] */ VARIANT_BOOL bSchemaOnly,
            /* [defaultvalue][in] */ BSTR strRequiredAssocQualifier,
            /* [defaultvalue][in] */ BSTR strRequiredQualifier,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [defaultvalue][in] */ IDispatch *objWbemAsyncContext);
        
        DECLSPEC_XFGVIRT(ISWbemObject, References_)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *References_ )( 
            ISWbemObject * This,
            /* [defaultvalue][in] */ BSTR strResultClass,
            /* [defaultvalue][in] */ BSTR strRole,
            /* [defaultvalue][in] */ VARIANT_BOOL bClassesOnly,
            /* [defaultvalue][in] */ VARIANT_BOOL bSchemaOnly,
            /* [defaultvalue][in] */ BSTR strRequiredQualifier,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [retval][out] */ ISWbemObjectSet **objWbemObjectSet);
        
        DECLSPEC_XFGVIRT(ISWbemObject, ReferencesAsync_)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ReferencesAsync_ )( 
            ISWbemObject * This,
            /* [in] */ IDispatch *objWbemSink,
            /* [defaultvalue][in] */ BSTR strResultClass,
            /* [defaultvalue][in] */ BSTR strRole,
            /* [defaultvalue][in] */ VARIANT_BOOL bClassesOnly,
            /* [defaultvalue][in] */ VARIANT_BOOL bSchemaOnly,
            /* [defaultvalue][in] */ BSTR strRequiredQualifier,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [defaultvalue][in] */ IDispatch *objWbemAsyncContext);
        
        DECLSPEC_XFGVIRT(ISWbemObject, ExecMethod_)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ExecMethod_ )( 
            ISWbemObject * This,
            /* [in] */ BSTR strMethodName,
            /* [defaultvalue][in] */ IDispatch *objWbemInParameters,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [retval][out] */ ISWbemObject **objWbemOutParameters);
        
        DECLSPEC_XFGVIRT(ISWbemObject, ExecMethodAsync_)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ExecMethodAsync_ )( 
            ISWbemObject * This,
            /* [in] */ IDispatch *objWbemSink,
            /* [in] */ BSTR strMethodName,
            /* [defaultvalue][in] */ IDispatch *objWbemInParameters,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [defaultvalue][in] */ IDispatch *objWbemAsyncContext);
        
        DECLSPEC_XFGVIRT(ISWbemObject, Clone_)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Clone_ )( 
            ISWbemObject * This,
            /* [retval][out] */ ISWbemObject **objWbemObject);
        
        DECLSPEC_XFGVIRT(ISWbemObject, GetObjectText_)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetObjectText_ )( 
            ISWbemObject * This,
            /* [defaultvalue][in] */ long iFlags,
            /* [retval][out] */ BSTR *strObjectText);
        
        DECLSPEC_XFGVIRT(ISWbemObject, SpawnDerivedClass_)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SpawnDerivedClass_ )( 
            ISWbemObject * This,
            /* [defaultvalue][in] */ long iFlags,
            /* [retval][out] */ ISWbemObject **objWbemObject);
        
        DECLSPEC_XFGVIRT(ISWbemObject, SpawnInstance_)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SpawnInstance_ )( 
            ISWbemObject * This,
            /* [defaultvalue][in] */ long iFlags,
            /* [retval][out] */ ISWbemObject **objWbemObject);
        
        DECLSPEC_XFGVIRT(ISWbemObject, CompareTo_)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CompareTo_ )( 
            ISWbemObject * This,
            /* [in] */ IDispatch *objWbemObject,
            /* [defaultvalue][in] */ long iFlags,
            /* [retval][out] */ VARIANT_BOOL *bResult);
        
        DECLSPEC_XFGVIRT(ISWbemObject, get_Qualifiers_)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Qualifiers_ )( 
            ISWbemObject * This,
            /* [retval][out] */ ISWbemQualifierSet **objWbemQualifierSet);
        
        DECLSPEC_XFGVIRT(ISWbemObject, get_Properties_)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Properties_ )( 
            ISWbemObject * This,
            /* [retval][out] */ ISWbemPropertySet **objWbemPropertySet);
        
        DECLSPEC_XFGVIRT(ISWbemObject, get_Methods_)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Methods_ )( 
            ISWbemObject * This,
            /* [retval][out] */ ISWbemMethodSet **objWbemMethodSet);
        
        DECLSPEC_XFGVIRT(ISWbemObject, get_Derivation_)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Derivation_ )( 
            ISWbemObject * This,
            /* [retval][out] */ VARIANT *strClassNameArray);
        
        DECLSPEC_XFGVIRT(ISWbemObject, get_Path_)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Path_ )( 
            ISWbemObject * This,
            /* [retval][out] */ ISWbemObjectPath **objWbemObjectPath);
        
        DECLSPEC_XFGVIRT(ISWbemObject, get_Security_)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Security_ )( 
            ISWbemObject * This,
            /* [retval][out] */ ISWbemSecurity **objWbemSecurity);
        
        END_INTERFACE
    } ISWbemObjectVtbl;

    interface ISWbemObject
    {
        CONST_VTBL struct ISWbemObjectVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISWbemObject_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISWbemObject_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISWbemObject_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISWbemObject_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISWbemObject_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISWbemObject_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISWbemObject_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISWbemObject_Put_(This,iFlags,objWbemNamedValueSet,objWbemObjectPath)	\
    ( (This)->lpVtbl -> Put_(This,iFlags,objWbemNamedValueSet,objWbemObjectPath) ) 

#define ISWbemObject_PutAsync_(This,objWbemSink,iFlags,objWbemNamedValueSet,objWbemAsyncContext)	\
    ( (This)->lpVtbl -> PutAsync_(This,objWbemSink,iFlags,objWbemNamedValueSet,objWbemAsyncContext) ) 

#define ISWbemObject_Delete_(This,iFlags,objWbemNamedValueSet)	\
    ( (This)->lpVtbl -> Delete_(This,iFlags,objWbemNamedValueSet) ) 

#define ISWbemObject_DeleteAsync_(This,objWbemSink,iFlags,objWbemNamedValueSet,objWbemAsyncContext)	\
    ( (This)->lpVtbl -> DeleteAsync_(This,objWbemSink,iFlags,objWbemNamedValueSet,objWbemAsyncContext) ) 

#define ISWbemObject_Instances_(This,iFlags,objWbemNamedValueSet,objWbemObjectSet)	\
    ( (This)->lpVtbl -> Instances_(This,iFlags,objWbemNamedValueSet,objWbemObjectSet) ) 

#define ISWbemObject_InstancesAsync_(This,objWbemSink,iFlags,objWbemNamedValueSet,objWbemAsyncContext)	\
    ( (This)->lpVtbl -> InstancesAsync_(This,objWbemSink,iFlags,objWbemNamedValueSet,objWbemAsyncContext) ) 

#define ISWbemObject_Subclasses_(This,iFlags,objWbemNamedValueSet,objWbemObjectSet)	\
    ( (This)->lpVtbl -> Subclasses_(This,iFlags,objWbemNamedValueSet,objWbemObjectSet) ) 

#define ISWbemObject_SubclassesAsync_(This,objWbemSink,iFlags,objWbemNamedValueSet,objWbemAsyncContext)	\
    ( (This)->lpVtbl -> SubclassesAsync_(This,objWbemSink,iFlags,objWbemNamedValueSet,objWbemAsyncContext) ) 

#define ISWbemObject_Associators_(This,strAssocClass,strResultClass,strResultRole,strRole,bClassesOnly,bSchemaOnly,strRequiredAssocQualifier,strRequiredQualifier,iFlags,objWbemNamedValueSet,objWbemObjectSet)	\
    ( (This)->lpVtbl -> Associators_(This,strAssocClass,strResultClass,strResultRole,strRole,bClassesOnly,bSchemaOnly,strRequiredAssocQualifier,strRequiredQualifier,iFlags,objWbemNamedValueSet,objWbemObjectSet) ) 

#define ISWbemObject_AssociatorsAsync_(This,objWbemSink,strAssocClass,strResultClass,strResultRole,strRole,bClassesOnly,bSchemaOnly,strRequiredAssocQualifier,strRequiredQualifier,iFlags,objWbemNamedValueSet,objWbemAsyncContext)	\
    ( (This)->lpVtbl -> AssociatorsAsync_(This,objWbemSink,strAssocClass,strResultClass,strResultRole,strRole,bClassesOnly,bSchemaOnly,strRequiredAssocQualifier,strRequiredQualifier,iFlags,objWbemNamedValueSet,objWbemAsyncContext) ) 

#define ISWbemObject_References_(This,strResultClass,strRole,bClassesOnly,bSchemaOnly,strRequiredQualifier,iFlags,objWbemNamedValueSet,objWbemObjectSet)	\
    ( (This)->lpVtbl -> References_(This,strResultClass,strRole,bClassesOnly,bSchemaOnly,strRequiredQualifier,iFlags,objWbemNamedValueSet,objWbemObjectSet) ) 

#define ISWbemObject_ReferencesAsync_(This,objWbemSink,strResultClass,strRole,bClassesOnly,bSchemaOnly,strRequiredQualifier,iFlags,objWbemNamedValueSet,objWbemAsyncContext)	\
    ( (This)->lpVtbl -> ReferencesAsync_(This,objWbemSink,strResultClass,strRole,bClassesOnly,bSchemaOnly,strRequiredQualifier,iFlags,objWbemNamedValueSet,objWbemAsyncContext) ) 

#define ISWbemObject_ExecMethod_(This,strMethodName,objWbemInParameters,iFlags,objWbemNamedValueSet,objWbemOutParameters)	\
    ( (This)->lpVtbl -> ExecMethod_(This,strMethodName,objWbemInParameters,iFlags,objWbemNamedValueSet,objWbemOutParameters) ) 

#define ISWbemObject_ExecMethodAsync_(This,objWbemSink,strMethodName,objWbemInParameters,iFlags,objWbemNamedValueSet,objWbemAsyncContext)	\
    ( (This)->lpVtbl -> ExecMethodAsync_(This,objWbemSink,strMethodName,objWbemInParameters,iFlags,objWbemNamedValueSet,objWbemAsyncContext) ) 

#define ISWbemObject_Clone_(This,objWbemObject)	\
    ( (This)->lpVtbl -> Clone_(This,objWbemObject) ) 

#define ISWbemObject_GetObjectText_(This,iFlags,strObjectText)	\
    ( (This)->lpVtbl -> GetObjectText_(This,iFlags,strObjectText) ) 

#define ISWbemObject_SpawnDerivedClass_(This,iFlags,objWbemObject)	\
    ( (This)->lpVtbl -> SpawnDerivedClass_(This,iFlags,objWbemObject) ) 

#define ISWbemObject_SpawnInstance_(This,iFlags,objWbemObject)	\
    ( (This)->lpVtbl -> SpawnInstance_(This,iFlags,objWbemObject) ) 

#define ISWbemObject_CompareTo_(This,objWbemObject,iFlags,bResult)	\
    ( (This)->lpVtbl -> CompareTo_(This,objWbemObject,iFlags,bResult) ) 

#define ISWbemObject_get_Qualifiers_(This,objWbemQualifierSet)	\
    ( (This)->lpVtbl -> get_Qualifiers_(This,objWbemQualifierSet) ) 

#define ISWbemObject_get_Properties_(This,objWbemPropertySet)	\
    ( (This)->lpVtbl -> get_Properties_(This,objWbemPropertySet) ) 

#define ISWbemObject_get_Methods_(This,objWbemMethodSet)	\
    ( (This)->lpVtbl -> get_Methods_(This,objWbemMethodSet) ) 

#define ISWbemObject_get_Derivation_(This,strClassNameArray)	\
    ( (This)->lpVtbl -> get_Derivation_(This,strClassNameArray) ) 

#define ISWbemObject_get_Path_(This,objWbemObjectPath)	\
    ( (This)->lpVtbl -> get_Path_(This,objWbemObjectPath) ) 

#define ISWbemObject_get_Security_(This,objWbemSecurity)	\
    ( (This)->lpVtbl -> get_Security_(This,objWbemSecurity) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISWbemObject_INTERFACE_DEFINED__ */


#ifndef __ISWbemObjectSet_INTERFACE_DEFINED__
#define __ISWbemObjectSet_INTERFACE_DEFINED__

/* interface ISWbemObjectSet */
/* [helpstring][nonextensible][hidden][dual][oleautomation][uuid][object][local] */ 


EXTERN_C const IID IID_ISWbemObjectSet;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("76A6415F-CB41-11d1-8B02-00600806D9B6")
    ISWbemObjectSet : public IDispatch
    {
    public:
        virtual /* [restricted][propget][id] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ IUnknown **pUnk) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Item( 
            /* [in] */ BSTR strObjectPath,
            /* [defaultvalue][in] */ long iFlags,
            /* [retval][out] */ ISWbemObject **objWbemObject) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ long *iCount) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Security_( 
            /* [retval][out] */ ISWbemSecurity **objWbemSecurity) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ItemIndex( 
            /* [in] */ long lIndex,
            /* [retval][out] */ ISWbemObject **objWbemObject) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISWbemObjectSetVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISWbemObjectSet * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISWbemObjectSet * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISWbemObjectSet * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            ISWbemObjectSet * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            ISWbemObjectSet * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            ISWbemObjectSet * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISWbemObjectSet * This,
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
        
        DECLSPEC_XFGVIRT(ISWbemObjectSet, get__NewEnum)
        /* [restricted][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            ISWbemObjectSet * This,
            /* [retval][out] */ IUnknown **pUnk);
        
        DECLSPEC_XFGVIRT(ISWbemObjectSet, Item)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Item )( 
            ISWbemObjectSet * This,
            /* [in] */ BSTR strObjectPath,
            /* [defaultvalue][in] */ long iFlags,
            /* [retval][out] */ ISWbemObject **objWbemObject);
        
        DECLSPEC_XFGVIRT(ISWbemObjectSet, get_Count)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            ISWbemObjectSet * This,
            /* [retval][out] */ long *iCount);
        
        DECLSPEC_XFGVIRT(ISWbemObjectSet, get_Security_)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Security_ )( 
            ISWbemObjectSet * This,
            /* [retval][out] */ ISWbemSecurity **objWbemSecurity);
        
        DECLSPEC_XFGVIRT(ISWbemObjectSet, ItemIndex)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ItemIndex )( 
            ISWbemObjectSet * This,
            /* [in] */ long lIndex,
            /* [retval][out] */ ISWbemObject **objWbemObject);
        
        END_INTERFACE
    } ISWbemObjectSetVtbl;

    interface ISWbemObjectSet
    {
        CONST_VTBL struct ISWbemObjectSetVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISWbemObjectSet_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISWbemObjectSet_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISWbemObjectSet_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISWbemObjectSet_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISWbemObjectSet_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISWbemObjectSet_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISWbemObjectSet_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISWbemObjectSet_get__NewEnum(This,pUnk)	\
    ( (This)->lpVtbl -> get__NewEnum(This,pUnk) ) 

#define ISWbemObjectSet_Item(This,strObjectPath,iFlags,objWbemObject)	\
    ( (This)->lpVtbl -> Item(This,strObjectPath,iFlags,objWbemObject) ) 

#define ISWbemObjectSet_get_Count(This,iCount)	\
    ( (This)->lpVtbl -> get_Count(This,iCount) ) 

#define ISWbemObjectSet_get_Security_(This,objWbemSecurity)	\
    ( (This)->lpVtbl -> get_Security_(This,objWbemSecurity) ) 

#define ISWbemObjectSet_ItemIndex(This,lIndex,objWbemObject)	\
    ( (This)->lpVtbl -> ItemIndex(This,lIndex,objWbemObject) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISWbemObjectSet_INTERFACE_DEFINED__ */


#ifndef __ISWbemNamedValue_INTERFACE_DEFINED__
#define __ISWbemNamedValue_INTERFACE_DEFINED__

/* interface ISWbemNamedValue */
/* [helpstring][hidden][oleautomation][dual][uuid][object][local] */ 


EXTERN_C const IID IID_ISWbemNamedValue;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("76A64164-CB41-11d1-8B02-00600806D9B6")
    ISWbemNamedValue : public IDispatch
    {
    public:
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Value( 
            /* [retval][out] */ VARIANT *varValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE put_Value( 
            /* [in] */ VARIANT *varValue) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ BSTR *strName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISWbemNamedValueVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISWbemNamedValue * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISWbemNamedValue * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISWbemNamedValue * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            ISWbemNamedValue * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            ISWbemNamedValue * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            ISWbemNamedValue * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISWbemNamedValue * This,
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
        
        DECLSPEC_XFGVIRT(ISWbemNamedValue, get_Value)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Value )( 
            ISWbemNamedValue * This,
            /* [retval][out] */ VARIANT *varValue);
        
        DECLSPEC_XFGVIRT(ISWbemNamedValue, put_Value)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Value )( 
            ISWbemNamedValue * This,
            /* [in] */ VARIANT *varValue);
        
        DECLSPEC_XFGVIRT(ISWbemNamedValue, get_Name)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            ISWbemNamedValue * This,
            /* [retval][out] */ BSTR *strName);
        
        END_INTERFACE
    } ISWbemNamedValueVtbl;

    interface ISWbemNamedValue
    {
        CONST_VTBL struct ISWbemNamedValueVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISWbemNamedValue_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISWbemNamedValue_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISWbemNamedValue_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISWbemNamedValue_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISWbemNamedValue_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISWbemNamedValue_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISWbemNamedValue_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISWbemNamedValue_get_Value(This,varValue)	\
    ( (This)->lpVtbl -> get_Value(This,varValue) ) 

#define ISWbemNamedValue_put_Value(This,varValue)	\
    ( (This)->lpVtbl -> put_Value(This,varValue) ) 

#define ISWbemNamedValue_get_Name(This,strName)	\
    ( (This)->lpVtbl -> get_Name(This,strName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISWbemNamedValue_INTERFACE_DEFINED__ */


#ifndef __ISWbemNamedValueSet_INTERFACE_DEFINED__
#define __ISWbemNamedValueSet_INTERFACE_DEFINED__

/* interface ISWbemNamedValueSet */
/* [helpstring][hidden][dual][oleautomation][uuid][object][local] */ 


EXTERN_C const IID IID_ISWbemNamedValueSet;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("CF2376EA-CE8C-11d1-8B05-00600806D9B6")
    ISWbemNamedValueSet : public IDispatch
    {
    public:
        virtual /* [restricted][propget][id] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ IUnknown **pUnk) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Item( 
            /* [in] */ BSTR strName,
            /* [defaultvalue][in] */ long iFlags,
            /* [retval][out] */ ISWbemNamedValue **objWbemNamedValue) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ long *iCount) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Add( 
            /* [in] */ BSTR strName,
            /* [in] */ VARIANT *varValue,
            /* [defaultvalue][in] */ long iFlags,
            /* [retval][out] */ ISWbemNamedValue **objWbemNamedValue) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Remove( 
            /* [in] */ BSTR strName,
            /* [defaultvalue][in] */ long iFlags = 0) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ ISWbemNamedValueSet **objWbemNamedValueSet) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE DeleteAll( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISWbemNamedValueSetVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISWbemNamedValueSet * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISWbemNamedValueSet * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISWbemNamedValueSet * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            ISWbemNamedValueSet * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            ISWbemNamedValueSet * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            ISWbemNamedValueSet * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISWbemNamedValueSet * This,
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
        
        DECLSPEC_XFGVIRT(ISWbemNamedValueSet, get__NewEnum)
        /* [restricted][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            ISWbemNamedValueSet * This,
            /* [retval][out] */ IUnknown **pUnk);
        
        DECLSPEC_XFGVIRT(ISWbemNamedValueSet, Item)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Item )( 
            ISWbemNamedValueSet * This,
            /* [in] */ BSTR strName,
            /* [defaultvalue][in] */ long iFlags,
            /* [retval][out] */ ISWbemNamedValue **objWbemNamedValue);
        
        DECLSPEC_XFGVIRT(ISWbemNamedValueSet, get_Count)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            ISWbemNamedValueSet * This,
            /* [retval][out] */ long *iCount);
        
        DECLSPEC_XFGVIRT(ISWbemNamedValueSet, Add)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Add )( 
            ISWbemNamedValueSet * This,
            /* [in] */ BSTR strName,
            /* [in] */ VARIANT *varValue,
            /* [defaultvalue][in] */ long iFlags,
            /* [retval][out] */ ISWbemNamedValue **objWbemNamedValue);
        
        DECLSPEC_XFGVIRT(ISWbemNamedValueSet, Remove)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Remove )( 
            ISWbemNamedValueSet * This,
            /* [in] */ BSTR strName,
            /* [defaultvalue][in] */ long iFlags);
        
        DECLSPEC_XFGVIRT(ISWbemNamedValueSet, Clone)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Clone )( 
            ISWbemNamedValueSet * This,
            /* [retval][out] */ ISWbemNamedValueSet **objWbemNamedValueSet);
        
        DECLSPEC_XFGVIRT(ISWbemNamedValueSet, DeleteAll)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *DeleteAll )( 
            ISWbemNamedValueSet * This);
        
        END_INTERFACE
    } ISWbemNamedValueSetVtbl;

    interface ISWbemNamedValueSet
    {
        CONST_VTBL struct ISWbemNamedValueSetVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISWbemNamedValueSet_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISWbemNamedValueSet_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISWbemNamedValueSet_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISWbemNamedValueSet_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISWbemNamedValueSet_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISWbemNamedValueSet_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISWbemNamedValueSet_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISWbemNamedValueSet_get__NewEnum(This,pUnk)	\
    ( (This)->lpVtbl -> get__NewEnum(This,pUnk) ) 

#define ISWbemNamedValueSet_Item(This,strName,iFlags,objWbemNamedValue)	\
    ( (This)->lpVtbl -> Item(This,strName,iFlags,objWbemNamedValue) ) 

#define ISWbemNamedValueSet_get_Count(This,iCount)	\
    ( (This)->lpVtbl -> get_Count(This,iCount) ) 

#define ISWbemNamedValueSet_Add(This,strName,varValue,iFlags,objWbemNamedValue)	\
    ( (This)->lpVtbl -> Add(This,strName,varValue,iFlags,objWbemNamedValue) ) 

#define ISWbemNamedValueSet_Remove(This,strName,iFlags)	\
    ( (This)->lpVtbl -> Remove(This,strName,iFlags) ) 

#define ISWbemNamedValueSet_Clone(This,objWbemNamedValueSet)	\
    ( (This)->lpVtbl -> Clone(This,objWbemNamedValueSet) ) 

#define ISWbemNamedValueSet_DeleteAll(This)	\
    ( (This)->lpVtbl -> DeleteAll(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISWbemNamedValueSet_INTERFACE_DEFINED__ */


#ifndef __ISWbemQualifier_INTERFACE_DEFINED__
#define __ISWbemQualifier_INTERFACE_DEFINED__

/* interface ISWbemQualifier */
/* [helpstring][unique][nonextensible][hidden][oleautomation][dual][uuid][local][object] */ 


EXTERN_C const IID IID_ISWbemQualifier;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("79B05932-D3B7-11d1-8B06-00600806D9B6")
    ISWbemQualifier : public IDispatch
    {
    public:
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Value( 
            /* [retval][out] */ VARIANT *varValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE put_Value( 
            /* [in] */ VARIANT *varValue) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ BSTR *strName) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_IsLocal( 
            /* [retval][out] */ VARIANT_BOOL *bIsLocal) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_PropagatesToSubclass( 
            /* [retval][out] */ VARIANT_BOOL *bPropagatesToSubclass) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE put_PropagatesToSubclass( 
            /* [in] */ VARIANT_BOOL bPropagatesToSubclass) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_PropagatesToInstance( 
            /* [retval][out] */ VARIANT_BOOL *bPropagatesToInstance) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE put_PropagatesToInstance( 
            /* [in] */ VARIANT_BOOL bPropagatesToInstance) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_IsOverridable( 
            /* [retval][out] */ VARIANT_BOOL *bIsOverridable) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE put_IsOverridable( 
            /* [in] */ VARIANT_BOOL bIsOverridable) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_IsAmended( 
            /* [retval][out] */ VARIANT_BOOL *bIsAmended) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISWbemQualifierVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISWbemQualifier * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISWbemQualifier * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISWbemQualifier * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            ISWbemQualifier * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            ISWbemQualifier * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            ISWbemQualifier * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISWbemQualifier * This,
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
        
        DECLSPEC_XFGVIRT(ISWbemQualifier, get_Value)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Value )( 
            ISWbemQualifier * This,
            /* [retval][out] */ VARIANT *varValue);
        
        DECLSPEC_XFGVIRT(ISWbemQualifier, put_Value)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Value )( 
            ISWbemQualifier * This,
            /* [in] */ VARIANT *varValue);
        
        DECLSPEC_XFGVIRT(ISWbemQualifier, get_Name)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            ISWbemQualifier * This,
            /* [retval][out] */ BSTR *strName);
        
        DECLSPEC_XFGVIRT(ISWbemQualifier, get_IsLocal)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_IsLocal )( 
            ISWbemQualifier * This,
            /* [retval][out] */ VARIANT_BOOL *bIsLocal);
        
        DECLSPEC_XFGVIRT(ISWbemQualifier, get_PropagatesToSubclass)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_PropagatesToSubclass )( 
            ISWbemQualifier * This,
            /* [retval][out] */ VARIANT_BOOL *bPropagatesToSubclass);
        
        DECLSPEC_XFGVIRT(ISWbemQualifier, put_PropagatesToSubclass)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_PropagatesToSubclass )( 
            ISWbemQualifier * This,
            /* [in] */ VARIANT_BOOL bPropagatesToSubclass);
        
        DECLSPEC_XFGVIRT(ISWbemQualifier, get_PropagatesToInstance)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_PropagatesToInstance )( 
            ISWbemQualifier * This,
            /* [retval][out] */ VARIANT_BOOL *bPropagatesToInstance);
        
        DECLSPEC_XFGVIRT(ISWbemQualifier, put_PropagatesToInstance)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_PropagatesToInstance )( 
            ISWbemQualifier * This,
            /* [in] */ VARIANT_BOOL bPropagatesToInstance);
        
        DECLSPEC_XFGVIRT(ISWbemQualifier, get_IsOverridable)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_IsOverridable )( 
            ISWbemQualifier * This,
            /* [retval][out] */ VARIANT_BOOL *bIsOverridable);
        
        DECLSPEC_XFGVIRT(ISWbemQualifier, put_IsOverridable)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_IsOverridable )( 
            ISWbemQualifier * This,
            /* [in] */ VARIANT_BOOL bIsOverridable);
        
        DECLSPEC_XFGVIRT(ISWbemQualifier, get_IsAmended)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_IsAmended )( 
            ISWbemQualifier * This,
            /* [retval][out] */ VARIANT_BOOL *bIsAmended);
        
        END_INTERFACE
    } ISWbemQualifierVtbl;

    interface ISWbemQualifier
    {
        CONST_VTBL struct ISWbemQualifierVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISWbemQualifier_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISWbemQualifier_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISWbemQualifier_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISWbemQualifier_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISWbemQualifier_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISWbemQualifier_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISWbemQualifier_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISWbemQualifier_get_Value(This,varValue)	\
    ( (This)->lpVtbl -> get_Value(This,varValue) ) 

#define ISWbemQualifier_put_Value(This,varValue)	\
    ( (This)->lpVtbl -> put_Value(This,varValue) ) 

#define ISWbemQualifier_get_Name(This,strName)	\
    ( (This)->lpVtbl -> get_Name(This,strName) ) 

#define ISWbemQualifier_get_IsLocal(This,bIsLocal)	\
    ( (This)->lpVtbl -> get_IsLocal(This,bIsLocal) ) 

#define ISWbemQualifier_get_PropagatesToSubclass(This,bPropagatesToSubclass)	\
    ( (This)->lpVtbl -> get_PropagatesToSubclass(This,bPropagatesToSubclass) ) 

#define ISWbemQualifier_put_PropagatesToSubclass(This,bPropagatesToSubclass)	\
    ( (This)->lpVtbl -> put_PropagatesToSubclass(This,bPropagatesToSubclass) ) 

#define ISWbemQualifier_get_PropagatesToInstance(This,bPropagatesToInstance)	\
    ( (This)->lpVtbl -> get_PropagatesToInstance(This,bPropagatesToInstance) ) 

#define ISWbemQualifier_put_PropagatesToInstance(This,bPropagatesToInstance)	\
    ( (This)->lpVtbl -> put_PropagatesToInstance(This,bPropagatesToInstance) ) 

#define ISWbemQualifier_get_IsOverridable(This,bIsOverridable)	\
    ( (This)->lpVtbl -> get_IsOverridable(This,bIsOverridable) ) 

#define ISWbemQualifier_put_IsOverridable(This,bIsOverridable)	\
    ( (This)->lpVtbl -> put_IsOverridable(This,bIsOverridable) ) 

#define ISWbemQualifier_get_IsAmended(This,bIsAmended)	\
    ( (This)->lpVtbl -> get_IsAmended(This,bIsAmended) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISWbemQualifier_INTERFACE_DEFINED__ */


#ifndef __ISWbemQualifierSet_INTERFACE_DEFINED__
#define __ISWbemQualifierSet_INTERFACE_DEFINED__

/* interface ISWbemQualifierSet */
/* [helpstring][nonextensible][hidden][dual][oleautomation][uuid][local][object] */ 


EXTERN_C const IID IID_ISWbemQualifierSet;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9B16ED16-D3DF-11d1-8B08-00600806D9B6")
    ISWbemQualifierSet : public IDispatch
    {
    public:
        virtual /* [restricted][propget][id] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ IUnknown **pUnk) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Item( 
            /* [in] */ BSTR name,
            /* [defaultvalue][in] */ long iFlags,
            /* [retval][out] */ ISWbemQualifier **objWbemQualifier) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ long *iCount) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Add( 
            /* [in] */ BSTR strName,
            /* [in] */ VARIANT *varVal,
            /* [defaultvalue][in] */ VARIANT_BOOL bPropagatesToSubclass,
            /* [defaultvalue][in] */ VARIANT_BOOL bPropagatesToInstance,
            /* [defaultvalue][in] */ VARIANT_BOOL bIsOverridable,
            /* [defaultvalue][in] */ long iFlags,
            /* [retval][out] */ ISWbemQualifier **objWbemQualifier) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Remove( 
            /* [in] */ BSTR strName,
            /* [defaultvalue][in] */ long iFlags = 0) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISWbemQualifierSetVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISWbemQualifierSet * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISWbemQualifierSet * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISWbemQualifierSet * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            ISWbemQualifierSet * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            ISWbemQualifierSet * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            ISWbemQualifierSet * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISWbemQualifierSet * This,
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
        
        DECLSPEC_XFGVIRT(ISWbemQualifierSet, get__NewEnum)
        /* [restricted][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            ISWbemQualifierSet * This,
            /* [retval][out] */ IUnknown **pUnk);
        
        DECLSPEC_XFGVIRT(ISWbemQualifierSet, Item)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Item )( 
            ISWbemQualifierSet * This,
            /* [in] */ BSTR name,
            /* [defaultvalue][in] */ long iFlags,
            /* [retval][out] */ ISWbemQualifier **objWbemQualifier);
        
        DECLSPEC_XFGVIRT(ISWbemQualifierSet, get_Count)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            ISWbemQualifierSet * This,
            /* [retval][out] */ long *iCount);
        
        DECLSPEC_XFGVIRT(ISWbemQualifierSet, Add)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Add )( 
            ISWbemQualifierSet * This,
            /* [in] */ BSTR strName,
            /* [in] */ VARIANT *varVal,
            /* [defaultvalue][in] */ VARIANT_BOOL bPropagatesToSubclass,
            /* [defaultvalue][in] */ VARIANT_BOOL bPropagatesToInstance,
            /* [defaultvalue][in] */ VARIANT_BOOL bIsOverridable,
            /* [defaultvalue][in] */ long iFlags,
            /* [retval][out] */ ISWbemQualifier **objWbemQualifier);
        
        DECLSPEC_XFGVIRT(ISWbemQualifierSet, Remove)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Remove )( 
            ISWbemQualifierSet * This,
            /* [in] */ BSTR strName,
            /* [defaultvalue][in] */ long iFlags);
        
        END_INTERFACE
    } ISWbemQualifierSetVtbl;

    interface ISWbemQualifierSet
    {
        CONST_VTBL struct ISWbemQualifierSetVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISWbemQualifierSet_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISWbemQualifierSet_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISWbemQualifierSet_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISWbemQualifierSet_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISWbemQualifierSet_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISWbemQualifierSet_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISWbemQualifierSet_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISWbemQualifierSet_get__NewEnum(This,pUnk)	\
    ( (This)->lpVtbl -> get__NewEnum(This,pUnk) ) 

#define ISWbemQualifierSet_Item(This,name,iFlags,objWbemQualifier)	\
    ( (This)->lpVtbl -> Item(This,name,iFlags,objWbemQualifier) ) 

#define ISWbemQualifierSet_get_Count(This,iCount)	\
    ( (This)->lpVtbl -> get_Count(This,iCount) ) 

#define ISWbemQualifierSet_Add(This,strName,varVal,bPropagatesToSubclass,bPropagatesToInstance,bIsOverridable,iFlags,objWbemQualifier)	\
    ( (This)->lpVtbl -> Add(This,strName,varVal,bPropagatesToSubclass,bPropagatesToInstance,bIsOverridable,iFlags,objWbemQualifier) ) 

#define ISWbemQualifierSet_Remove(This,strName,iFlags)	\
    ( (This)->lpVtbl -> Remove(This,strName,iFlags) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISWbemQualifierSet_INTERFACE_DEFINED__ */


#ifndef __ISWbemProperty_INTERFACE_DEFINED__
#define __ISWbemProperty_INTERFACE_DEFINED__

/* interface ISWbemProperty */
/* [helpstring][unique][hidden][oleautomation][dual][uuid][local][object] */ 


EXTERN_C const IID IID_ISWbemProperty;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1A388F98-D4BA-11d1-8B09-00600806D9B6")
    ISWbemProperty : public IDispatch
    {
    public:
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Value( 
            /* [retval][out] */ VARIANT *varValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE put_Value( 
            /* [in] */ VARIANT *varValue) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ BSTR *strName) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_IsLocal( 
            /* [retval][out] */ VARIANT_BOOL *bIsLocal) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Origin( 
            /* [retval][out] */ BSTR *strOrigin) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_CIMType( 
            /* [retval][out] */ WbemCimtypeEnum *iCimType) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Qualifiers_( 
            /* [retval][out] */ ISWbemQualifierSet **objWbemQualifierSet) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_IsArray( 
            /* [retval][out] */ VARIANT_BOOL *bIsArray) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISWbemPropertyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISWbemProperty * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISWbemProperty * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISWbemProperty * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            ISWbemProperty * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            ISWbemProperty * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            ISWbemProperty * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISWbemProperty * This,
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
        
        DECLSPEC_XFGVIRT(ISWbemProperty, get_Value)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Value )( 
            ISWbemProperty * This,
            /* [retval][out] */ VARIANT *varValue);
        
        DECLSPEC_XFGVIRT(ISWbemProperty, put_Value)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Value )( 
            ISWbemProperty * This,
            /* [in] */ VARIANT *varValue);
        
        DECLSPEC_XFGVIRT(ISWbemProperty, get_Name)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            ISWbemProperty * This,
            /* [retval][out] */ BSTR *strName);
        
        DECLSPEC_XFGVIRT(ISWbemProperty, get_IsLocal)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_IsLocal )( 
            ISWbemProperty * This,
            /* [retval][out] */ VARIANT_BOOL *bIsLocal);
        
        DECLSPEC_XFGVIRT(ISWbemProperty, get_Origin)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Origin )( 
            ISWbemProperty * This,
            /* [retval][out] */ BSTR *strOrigin);
        
        DECLSPEC_XFGVIRT(ISWbemProperty, get_CIMType)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_CIMType )( 
            ISWbemProperty * This,
            /* [retval][out] */ WbemCimtypeEnum *iCimType);
        
        DECLSPEC_XFGVIRT(ISWbemProperty, get_Qualifiers_)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Qualifiers_ )( 
            ISWbemProperty * This,
            /* [retval][out] */ ISWbemQualifierSet **objWbemQualifierSet);
        
        DECLSPEC_XFGVIRT(ISWbemProperty, get_IsArray)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_IsArray )( 
            ISWbemProperty * This,
            /* [retval][out] */ VARIANT_BOOL *bIsArray);
        
        END_INTERFACE
    } ISWbemPropertyVtbl;

    interface ISWbemProperty
    {
        CONST_VTBL struct ISWbemPropertyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISWbemProperty_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISWbemProperty_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISWbemProperty_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISWbemProperty_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISWbemProperty_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISWbemProperty_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISWbemProperty_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISWbemProperty_get_Value(This,varValue)	\
    ( (This)->lpVtbl -> get_Value(This,varValue) ) 

#define ISWbemProperty_put_Value(This,varValue)	\
    ( (This)->lpVtbl -> put_Value(This,varValue) ) 

#define ISWbemProperty_get_Name(This,strName)	\
    ( (This)->lpVtbl -> get_Name(This,strName) ) 

#define ISWbemProperty_get_IsLocal(This,bIsLocal)	\
    ( (This)->lpVtbl -> get_IsLocal(This,bIsLocal) ) 

#define ISWbemProperty_get_Origin(This,strOrigin)	\
    ( (This)->lpVtbl -> get_Origin(This,strOrigin) ) 

#define ISWbemProperty_get_CIMType(This,iCimType)	\
    ( (This)->lpVtbl -> get_CIMType(This,iCimType) ) 

#define ISWbemProperty_get_Qualifiers_(This,objWbemQualifierSet)	\
    ( (This)->lpVtbl -> get_Qualifiers_(This,objWbemQualifierSet) ) 

#define ISWbemProperty_get_IsArray(This,bIsArray)	\
    ( (This)->lpVtbl -> get_IsArray(This,bIsArray) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISWbemProperty_INTERFACE_DEFINED__ */


#ifndef __ISWbemPropertySet_INTERFACE_DEFINED__
#define __ISWbemPropertySet_INTERFACE_DEFINED__

/* interface ISWbemPropertySet */
/* [helpstring][hidden][dual][oleautomation][uuid][local][object] */ 


EXTERN_C const IID IID_ISWbemPropertySet;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DEA0A7B2-D4BA-11d1-8B09-00600806D9B6")
    ISWbemPropertySet : public IDispatch
    {
    public:
        virtual /* [restricted][propget][id] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ IUnknown **pUnk) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Item( 
            /* [in] */ BSTR strName,
            /* [defaultvalue][in] */ long iFlags,
            /* [retval][out] */ ISWbemProperty **objWbemProperty) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ long *iCount) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Add( 
            /* [in] */ BSTR strName,
            /* [in] */ WbemCimtypeEnum iCIMType,
            /* [defaultvalue][in] */ VARIANT_BOOL bIsArray,
            /* [defaultvalue][in] */ long iFlags,
            /* [retval][out] */ ISWbemProperty **objWbemProperty) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Remove( 
            /* [in] */ BSTR strName,
            /* [defaultvalue][in] */ long iFlags = 0) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISWbemPropertySetVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISWbemPropertySet * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISWbemPropertySet * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISWbemPropertySet * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            ISWbemPropertySet * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            ISWbemPropertySet * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            ISWbemPropertySet * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISWbemPropertySet * This,
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
        
        DECLSPEC_XFGVIRT(ISWbemPropertySet, get__NewEnum)
        /* [restricted][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            ISWbemPropertySet * This,
            /* [retval][out] */ IUnknown **pUnk);
        
        DECLSPEC_XFGVIRT(ISWbemPropertySet, Item)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Item )( 
            ISWbemPropertySet * This,
            /* [in] */ BSTR strName,
            /* [defaultvalue][in] */ long iFlags,
            /* [retval][out] */ ISWbemProperty **objWbemProperty);
        
        DECLSPEC_XFGVIRT(ISWbemPropertySet, get_Count)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            ISWbemPropertySet * This,
            /* [retval][out] */ long *iCount);
        
        DECLSPEC_XFGVIRT(ISWbemPropertySet, Add)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Add )( 
            ISWbemPropertySet * This,
            /* [in] */ BSTR strName,
            /* [in] */ WbemCimtypeEnum iCIMType,
            /* [defaultvalue][in] */ VARIANT_BOOL bIsArray,
            /* [defaultvalue][in] */ long iFlags,
            /* [retval][out] */ ISWbemProperty **objWbemProperty);
        
        DECLSPEC_XFGVIRT(ISWbemPropertySet, Remove)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Remove )( 
            ISWbemPropertySet * This,
            /* [in] */ BSTR strName,
            /* [defaultvalue][in] */ long iFlags);
        
        END_INTERFACE
    } ISWbemPropertySetVtbl;

    interface ISWbemPropertySet
    {
        CONST_VTBL struct ISWbemPropertySetVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISWbemPropertySet_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISWbemPropertySet_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISWbemPropertySet_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISWbemPropertySet_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISWbemPropertySet_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISWbemPropertySet_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISWbemPropertySet_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISWbemPropertySet_get__NewEnum(This,pUnk)	\
    ( (This)->lpVtbl -> get__NewEnum(This,pUnk) ) 

#define ISWbemPropertySet_Item(This,strName,iFlags,objWbemProperty)	\
    ( (This)->lpVtbl -> Item(This,strName,iFlags,objWbemProperty) ) 

#define ISWbemPropertySet_get_Count(This,iCount)	\
    ( (This)->lpVtbl -> get_Count(This,iCount) ) 

#define ISWbemPropertySet_Add(This,strName,iCIMType,bIsArray,iFlags,objWbemProperty)	\
    ( (This)->lpVtbl -> Add(This,strName,iCIMType,bIsArray,iFlags,objWbemProperty) ) 

#define ISWbemPropertySet_Remove(This,strName,iFlags)	\
    ( (This)->lpVtbl -> Remove(This,strName,iFlags) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISWbemPropertySet_INTERFACE_DEFINED__ */


#ifndef __ISWbemMethod_INTERFACE_DEFINED__
#define __ISWbemMethod_INTERFACE_DEFINED__

/* interface ISWbemMethod */
/* [helpstring][hidden][nonextensible][unique][oleautomation][dual][uuid][local][object] */ 


EXTERN_C const IID IID_ISWbemMethod;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("422E8E90-D955-11d1-8B09-00600806D9B6")
    ISWbemMethod : public IDispatch
    {
    public:
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ BSTR *strName) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Origin( 
            /* [retval][out] */ BSTR *strOrigin) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_InParameters( 
            /* [retval][out] */ ISWbemObject **objWbemInParameters) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_OutParameters( 
            /* [retval][out] */ ISWbemObject **objWbemOutParameters) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Qualifiers_( 
            /* [retval][out] */ ISWbemQualifierSet **objWbemQualifierSet) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISWbemMethodVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISWbemMethod * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISWbemMethod * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISWbemMethod * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            ISWbemMethod * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            ISWbemMethod * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            ISWbemMethod * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISWbemMethod * This,
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
        
        DECLSPEC_XFGVIRT(ISWbemMethod, get_Name)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            ISWbemMethod * This,
            /* [retval][out] */ BSTR *strName);
        
        DECLSPEC_XFGVIRT(ISWbemMethod, get_Origin)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Origin )( 
            ISWbemMethod * This,
            /* [retval][out] */ BSTR *strOrigin);
        
        DECLSPEC_XFGVIRT(ISWbemMethod, get_InParameters)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_InParameters )( 
            ISWbemMethod * This,
            /* [retval][out] */ ISWbemObject **objWbemInParameters);
        
        DECLSPEC_XFGVIRT(ISWbemMethod, get_OutParameters)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_OutParameters )( 
            ISWbemMethod * This,
            /* [retval][out] */ ISWbemObject **objWbemOutParameters);
        
        DECLSPEC_XFGVIRT(ISWbemMethod, get_Qualifiers_)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Qualifiers_ )( 
            ISWbemMethod * This,
            /* [retval][out] */ ISWbemQualifierSet **objWbemQualifierSet);
        
        END_INTERFACE
    } ISWbemMethodVtbl;

    interface ISWbemMethod
    {
        CONST_VTBL struct ISWbemMethodVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISWbemMethod_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISWbemMethod_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISWbemMethod_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISWbemMethod_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISWbemMethod_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISWbemMethod_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISWbemMethod_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISWbemMethod_get_Name(This,strName)	\
    ( (This)->lpVtbl -> get_Name(This,strName) ) 

#define ISWbemMethod_get_Origin(This,strOrigin)	\
    ( (This)->lpVtbl -> get_Origin(This,strOrigin) ) 

#define ISWbemMethod_get_InParameters(This,objWbemInParameters)	\
    ( (This)->lpVtbl -> get_InParameters(This,objWbemInParameters) ) 

#define ISWbemMethod_get_OutParameters(This,objWbemOutParameters)	\
    ( (This)->lpVtbl -> get_OutParameters(This,objWbemOutParameters) ) 

#define ISWbemMethod_get_Qualifiers_(This,objWbemQualifierSet)	\
    ( (This)->lpVtbl -> get_Qualifiers_(This,objWbemQualifierSet) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISWbemMethod_INTERFACE_DEFINED__ */


#ifndef __ISWbemMethodSet_INTERFACE_DEFINED__
#define __ISWbemMethodSet_INTERFACE_DEFINED__

/* interface ISWbemMethodSet */
/* [helpstring][hidden][nonextensible][dual][oleautomation][uuid][object][local] */ 


EXTERN_C const IID IID_ISWbemMethodSet;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C93BA292-D955-11d1-8B09-00600806D9B6")
    ISWbemMethodSet : public IDispatch
    {
    public:
        virtual /* [restricted][propget][id] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ IUnknown **pUnk) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Item( 
            /* [in] */ BSTR strName,
            /* [defaultvalue][in] */ long iFlags,
            /* [retval][out] */ ISWbemMethod **objWbemMethod) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ long *iCount) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISWbemMethodSetVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISWbemMethodSet * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISWbemMethodSet * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISWbemMethodSet * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            ISWbemMethodSet * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            ISWbemMethodSet * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            ISWbemMethodSet * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISWbemMethodSet * This,
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
        
        DECLSPEC_XFGVIRT(ISWbemMethodSet, get__NewEnum)
        /* [restricted][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            ISWbemMethodSet * This,
            /* [retval][out] */ IUnknown **pUnk);
        
        DECLSPEC_XFGVIRT(ISWbemMethodSet, Item)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Item )( 
            ISWbemMethodSet * This,
            /* [in] */ BSTR strName,
            /* [defaultvalue][in] */ long iFlags,
            /* [retval][out] */ ISWbemMethod **objWbemMethod);
        
        DECLSPEC_XFGVIRT(ISWbemMethodSet, get_Count)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            ISWbemMethodSet * This,
            /* [retval][out] */ long *iCount);
        
        END_INTERFACE
    } ISWbemMethodSetVtbl;

    interface ISWbemMethodSet
    {
        CONST_VTBL struct ISWbemMethodSetVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISWbemMethodSet_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISWbemMethodSet_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISWbemMethodSet_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISWbemMethodSet_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISWbemMethodSet_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISWbemMethodSet_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISWbemMethodSet_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISWbemMethodSet_get__NewEnum(This,pUnk)	\
    ( (This)->lpVtbl -> get__NewEnum(This,pUnk) ) 

#define ISWbemMethodSet_Item(This,strName,iFlags,objWbemMethod)	\
    ( (This)->lpVtbl -> Item(This,strName,iFlags,objWbemMethod) ) 

#define ISWbemMethodSet_get_Count(This,iCount)	\
    ( (This)->lpVtbl -> get_Count(This,iCount) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISWbemMethodSet_INTERFACE_DEFINED__ */


#ifndef __ISWbemEventSource_INTERFACE_DEFINED__
#define __ISWbemEventSource_INTERFACE_DEFINED__

/* interface ISWbemEventSource */
/* [helpstring][hidden][nonextensible][dual][oleautomation][uuid][local][object] */ 


EXTERN_C const IID IID_ISWbemEventSource;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("27D54D92-0EBE-11d2-8B22-00600806D9B6")
    ISWbemEventSource : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE NextEvent( 
            /* [defaultvalue][in] */ long iTimeoutMs,
            /* [retval][out] */ ISWbemObject **objWbemObject) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Security_( 
            /* [retval][out] */ ISWbemSecurity **objWbemSecurity) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISWbemEventSourceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISWbemEventSource * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISWbemEventSource * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISWbemEventSource * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            ISWbemEventSource * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            ISWbemEventSource * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            ISWbemEventSource * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISWbemEventSource * This,
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
        
        DECLSPEC_XFGVIRT(ISWbemEventSource, NextEvent)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *NextEvent )( 
            ISWbemEventSource * This,
            /* [defaultvalue][in] */ long iTimeoutMs,
            /* [retval][out] */ ISWbemObject **objWbemObject);
        
        DECLSPEC_XFGVIRT(ISWbemEventSource, get_Security_)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Security_ )( 
            ISWbemEventSource * This,
            /* [retval][out] */ ISWbemSecurity **objWbemSecurity);
        
        END_INTERFACE
    } ISWbemEventSourceVtbl;

    interface ISWbemEventSource
    {
        CONST_VTBL struct ISWbemEventSourceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISWbemEventSource_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISWbemEventSource_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISWbemEventSource_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISWbemEventSource_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISWbemEventSource_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISWbemEventSource_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISWbemEventSource_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISWbemEventSource_NextEvent(This,iTimeoutMs,objWbemObject)	\
    ( (This)->lpVtbl -> NextEvent(This,iTimeoutMs,objWbemObject) ) 

#define ISWbemEventSource_get_Security_(This,objWbemSecurity)	\
    ( (This)->lpVtbl -> get_Security_(This,objWbemSecurity) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISWbemEventSource_INTERFACE_DEFINED__ */


#ifndef __ISWbemObjectPath_INTERFACE_DEFINED__
#define __ISWbemObjectPath_INTERFACE_DEFINED__

/* interface ISWbemObjectPath */
/* [helpstring][unique][hidden][oleautomation][dual][uuid][local][object] */ 


EXTERN_C const IID IID_ISWbemObjectPath;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5791BC27-CE9C-11d1-97BF-0000F81E849C")
    ISWbemObjectPath : public IDispatch
    {
    public:
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Path( 
            /* [retval][out] */ BSTR *strPath) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE put_Path( 
            /* [in] */ BSTR strPath) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_RelPath( 
            /* [retval][out] */ BSTR *strRelPath) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE put_RelPath( 
            /* [in] */ BSTR strRelPath) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Server( 
            /* [retval][out] */ BSTR *strServer) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE put_Server( 
            /* [in] */ BSTR strServer) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Namespace( 
            /* [retval][out] */ BSTR *strNamespace) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE put_Namespace( 
            /* [in] */ BSTR strNamespace) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_ParentNamespace( 
            /* [retval][out] */ BSTR *strParentNamespace) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_DisplayName( 
            /* [retval][out] */ BSTR *strDisplayName) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE put_DisplayName( 
            /* [in] */ BSTR strDisplayName) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Class( 
            /* [retval][out] */ BSTR *strClass) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE put_Class( 
            /* [in] */ BSTR strClass) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_IsClass( 
            /* [retval][out] */ VARIANT_BOOL *bIsClass) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetAsClass( void) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_IsSingleton( 
            /* [retval][out] */ VARIANT_BOOL *bIsSingleton) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetAsSingleton( void) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Keys( 
            /* [retval][out] */ ISWbemNamedValueSet **objWbemNamedValueSet) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Security_( 
            /* [retval][out] */ ISWbemSecurity **objWbemSecurity) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Locale( 
            /* [retval][out] */ BSTR *strLocale) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE put_Locale( 
            /* [in] */ BSTR strLocale) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Authority( 
            /* [retval][out] */ BSTR *strAuthority) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE put_Authority( 
            /* [in] */ BSTR strAuthority) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISWbemObjectPathVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISWbemObjectPath * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISWbemObjectPath * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISWbemObjectPath * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            ISWbemObjectPath * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            ISWbemObjectPath * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            ISWbemObjectPath * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISWbemObjectPath * This,
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
        
        DECLSPEC_XFGVIRT(ISWbemObjectPath, get_Path)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Path )( 
            ISWbemObjectPath * This,
            /* [retval][out] */ BSTR *strPath);
        
        DECLSPEC_XFGVIRT(ISWbemObjectPath, put_Path)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Path )( 
            ISWbemObjectPath * This,
            /* [in] */ BSTR strPath);
        
        DECLSPEC_XFGVIRT(ISWbemObjectPath, get_RelPath)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_RelPath )( 
            ISWbemObjectPath * This,
            /* [retval][out] */ BSTR *strRelPath);
        
        DECLSPEC_XFGVIRT(ISWbemObjectPath, put_RelPath)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_RelPath )( 
            ISWbemObjectPath * This,
            /* [in] */ BSTR strRelPath);
        
        DECLSPEC_XFGVIRT(ISWbemObjectPath, get_Server)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Server )( 
            ISWbemObjectPath * This,
            /* [retval][out] */ BSTR *strServer);
        
        DECLSPEC_XFGVIRT(ISWbemObjectPath, put_Server)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Server )( 
            ISWbemObjectPath * This,
            /* [in] */ BSTR strServer);
        
        DECLSPEC_XFGVIRT(ISWbemObjectPath, get_Namespace)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Namespace )( 
            ISWbemObjectPath * This,
            /* [retval][out] */ BSTR *strNamespace);
        
        DECLSPEC_XFGVIRT(ISWbemObjectPath, put_Namespace)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Namespace )( 
            ISWbemObjectPath * This,
            /* [in] */ BSTR strNamespace);
        
        DECLSPEC_XFGVIRT(ISWbemObjectPath, get_ParentNamespace)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ParentNamespace )( 
            ISWbemObjectPath * This,
            /* [retval][out] */ BSTR *strParentNamespace);
        
        DECLSPEC_XFGVIRT(ISWbemObjectPath, get_DisplayName)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_DisplayName )( 
            ISWbemObjectPath * This,
            /* [retval][out] */ BSTR *strDisplayName);
        
        DECLSPEC_XFGVIRT(ISWbemObjectPath, put_DisplayName)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_DisplayName )( 
            ISWbemObjectPath * This,
            /* [in] */ BSTR strDisplayName);
        
        DECLSPEC_XFGVIRT(ISWbemObjectPath, get_Class)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Class )( 
            ISWbemObjectPath * This,
            /* [retval][out] */ BSTR *strClass);
        
        DECLSPEC_XFGVIRT(ISWbemObjectPath, put_Class)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Class )( 
            ISWbemObjectPath * This,
            /* [in] */ BSTR strClass);
        
        DECLSPEC_XFGVIRT(ISWbemObjectPath, get_IsClass)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_IsClass )( 
            ISWbemObjectPath * This,
            /* [retval][out] */ VARIANT_BOOL *bIsClass);
        
        DECLSPEC_XFGVIRT(ISWbemObjectPath, SetAsClass)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetAsClass )( 
            ISWbemObjectPath * This);
        
        DECLSPEC_XFGVIRT(ISWbemObjectPath, get_IsSingleton)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_IsSingleton )( 
            ISWbemObjectPath * This,
            /* [retval][out] */ VARIANT_BOOL *bIsSingleton);
        
        DECLSPEC_XFGVIRT(ISWbemObjectPath, SetAsSingleton)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetAsSingleton )( 
            ISWbemObjectPath * This);
        
        DECLSPEC_XFGVIRT(ISWbemObjectPath, get_Keys)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Keys )( 
            ISWbemObjectPath * This,
            /* [retval][out] */ ISWbemNamedValueSet **objWbemNamedValueSet);
        
        DECLSPEC_XFGVIRT(ISWbemObjectPath, get_Security_)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Security_ )( 
            ISWbemObjectPath * This,
            /* [retval][out] */ ISWbemSecurity **objWbemSecurity);
        
        DECLSPEC_XFGVIRT(ISWbemObjectPath, get_Locale)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Locale )( 
            ISWbemObjectPath * This,
            /* [retval][out] */ BSTR *strLocale);
        
        DECLSPEC_XFGVIRT(ISWbemObjectPath, put_Locale)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Locale )( 
            ISWbemObjectPath * This,
            /* [in] */ BSTR strLocale);
        
        DECLSPEC_XFGVIRT(ISWbemObjectPath, get_Authority)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Authority )( 
            ISWbemObjectPath * This,
            /* [retval][out] */ BSTR *strAuthority);
        
        DECLSPEC_XFGVIRT(ISWbemObjectPath, put_Authority)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Authority )( 
            ISWbemObjectPath * This,
            /* [in] */ BSTR strAuthority);
        
        END_INTERFACE
    } ISWbemObjectPathVtbl;

    interface ISWbemObjectPath
    {
        CONST_VTBL struct ISWbemObjectPathVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISWbemObjectPath_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISWbemObjectPath_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISWbemObjectPath_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISWbemObjectPath_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISWbemObjectPath_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISWbemObjectPath_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISWbemObjectPath_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISWbemObjectPath_get_Path(This,strPath)	\
    ( (This)->lpVtbl -> get_Path(This,strPath) ) 

#define ISWbemObjectPath_put_Path(This,strPath)	\
    ( (This)->lpVtbl -> put_Path(This,strPath) ) 

#define ISWbemObjectPath_get_RelPath(This,strRelPath)	\
    ( (This)->lpVtbl -> get_RelPath(This,strRelPath) ) 

#define ISWbemObjectPath_put_RelPath(This,strRelPath)	\
    ( (This)->lpVtbl -> put_RelPath(This,strRelPath) ) 

#define ISWbemObjectPath_get_Server(This,strServer)	\
    ( (This)->lpVtbl -> get_Server(This,strServer) ) 

#define ISWbemObjectPath_put_Server(This,strServer)	\
    ( (This)->lpVtbl -> put_Server(This,strServer) ) 

#define ISWbemObjectPath_get_Namespace(This,strNamespace)	\
    ( (This)->lpVtbl -> get_Namespace(This,strNamespace) ) 

#define ISWbemObjectPath_put_Namespace(This,strNamespace)	\
    ( (This)->lpVtbl -> put_Namespace(This,strNamespace) ) 

#define ISWbemObjectPath_get_ParentNamespace(This,strParentNamespace)	\
    ( (This)->lpVtbl -> get_ParentNamespace(This,strParentNamespace) ) 

#define ISWbemObjectPath_get_DisplayName(This,strDisplayName)	\
    ( (This)->lpVtbl -> get_DisplayName(This,strDisplayName) ) 

#define ISWbemObjectPath_put_DisplayName(This,strDisplayName)	\
    ( (This)->lpVtbl -> put_DisplayName(This,strDisplayName) ) 

#define ISWbemObjectPath_get_Class(This,strClass)	\
    ( (This)->lpVtbl -> get_Class(This,strClass) ) 

#define ISWbemObjectPath_put_Class(This,strClass)	\
    ( (This)->lpVtbl -> put_Class(This,strClass) ) 

#define ISWbemObjectPath_get_IsClass(This,bIsClass)	\
    ( (This)->lpVtbl -> get_IsClass(This,bIsClass) ) 

#define ISWbemObjectPath_SetAsClass(This)	\
    ( (This)->lpVtbl -> SetAsClass(This) ) 

#define ISWbemObjectPath_get_IsSingleton(This,bIsSingleton)	\
    ( (This)->lpVtbl -> get_IsSingleton(This,bIsSingleton) ) 

#define ISWbemObjectPath_SetAsSingleton(This)	\
    ( (This)->lpVtbl -> SetAsSingleton(This) ) 

#define ISWbemObjectPath_get_Keys(This,objWbemNamedValueSet)	\
    ( (This)->lpVtbl -> get_Keys(This,objWbemNamedValueSet) ) 

#define ISWbemObjectPath_get_Security_(This,objWbemSecurity)	\
    ( (This)->lpVtbl -> get_Security_(This,objWbemSecurity) ) 

#define ISWbemObjectPath_get_Locale(This,strLocale)	\
    ( (This)->lpVtbl -> get_Locale(This,strLocale) ) 

#define ISWbemObjectPath_put_Locale(This,strLocale)	\
    ( (This)->lpVtbl -> put_Locale(This,strLocale) ) 

#define ISWbemObjectPath_get_Authority(This,strAuthority)	\
    ( (This)->lpVtbl -> get_Authority(This,strAuthority) ) 

#define ISWbemObjectPath_put_Authority(This,strAuthority)	\
    ( (This)->lpVtbl -> put_Authority(This,strAuthority) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISWbemObjectPath_INTERFACE_DEFINED__ */


#ifndef __ISWbemLastError_INTERFACE_DEFINED__
#define __ISWbemLastError_INTERFACE_DEFINED__

/* interface ISWbemLastError */
/* [helpstring][hidden][oleautomation][dual][uuid][object][local] */ 


EXTERN_C const IID IID_ISWbemLastError;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D962DB84-D4BB-11d1-8B09-00600806D9B6")
    ISWbemLastError : public ISWbemObject
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct ISWbemLastErrorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISWbemLastError * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISWbemLastError * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISWbemLastError * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            ISWbemLastError * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            ISWbemLastError * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            ISWbemLastError * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISWbemLastError * This,
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
        
        DECLSPEC_XFGVIRT(ISWbemObject, Put_)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Put_ )( 
            ISWbemLastError * This,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [retval][out] */ ISWbemObjectPath **objWbemObjectPath);
        
        DECLSPEC_XFGVIRT(ISWbemObject, PutAsync_)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *PutAsync_ )( 
            ISWbemLastError * This,
            /* [in] */ IDispatch *objWbemSink,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [defaultvalue][in] */ IDispatch *objWbemAsyncContext);
        
        DECLSPEC_XFGVIRT(ISWbemObject, Delete_)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Delete_ )( 
            ISWbemLastError * This,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet);
        
        DECLSPEC_XFGVIRT(ISWbemObject, DeleteAsync_)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *DeleteAsync_ )( 
            ISWbemLastError * This,
            /* [in] */ IDispatch *objWbemSink,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [defaultvalue][in] */ IDispatch *objWbemAsyncContext);
        
        DECLSPEC_XFGVIRT(ISWbemObject, Instances_)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Instances_ )( 
            ISWbemLastError * This,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [retval][out] */ ISWbemObjectSet **objWbemObjectSet);
        
        DECLSPEC_XFGVIRT(ISWbemObject, InstancesAsync_)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *InstancesAsync_ )( 
            ISWbemLastError * This,
            /* [in] */ IDispatch *objWbemSink,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [defaultvalue][in] */ IDispatch *objWbemAsyncContext);
        
        DECLSPEC_XFGVIRT(ISWbemObject, Subclasses_)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Subclasses_ )( 
            ISWbemLastError * This,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [retval][out] */ ISWbemObjectSet **objWbemObjectSet);
        
        DECLSPEC_XFGVIRT(ISWbemObject, SubclassesAsync_)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SubclassesAsync_ )( 
            ISWbemLastError * This,
            /* [in] */ IDispatch *objWbemSink,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [defaultvalue][in] */ IDispatch *objWbemAsyncContext);
        
        DECLSPEC_XFGVIRT(ISWbemObject, Associators_)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Associators_ )( 
            ISWbemLastError * This,
            /* [defaultvalue][in] */ BSTR strAssocClass,
            /* [defaultvalue][in] */ BSTR strResultClass,
            /* [defaultvalue][in] */ BSTR strResultRole,
            /* [defaultvalue][in] */ BSTR strRole,
            /* [defaultvalue][in] */ VARIANT_BOOL bClassesOnly,
            /* [defaultvalue][in] */ VARIANT_BOOL bSchemaOnly,
            /* [defaultvalue][in] */ BSTR strRequiredAssocQualifier,
            /* [defaultvalue][in] */ BSTR strRequiredQualifier,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [retval][out] */ ISWbemObjectSet **objWbemObjectSet);
        
        DECLSPEC_XFGVIRT(ISWbemObject, AssociatorsAsync_)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *AssociatorsAsync_ )( 
            ISWbemLastError * This,
            /* [in] */ IDispatch *objWbemSink,
            /* [defaultvalue][in] */ BSTR strAssocClass,
            /* [defaultvalue][in] */ BSTR strResultClass,
            /* [defaultvalue][in] */ BSTR strResultRole,
            /* [defaultvalue][in] */ BSTR strRole,
            /* [defaultvalue][in] */ VARIANT_BOOL bClassesOnly,
            /* [defaultvalue][in] */ VARIANT_BOOL bSchemaOnly,
            /* [defaultvalue][in] */ BSTR strRequiredAssocQualifier,
            /* [defaultvalue][in] */ BSTR strRequiredQualifier,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [defaultvalue][in] */ IDispatch *objWbemAsyncContext);
        
        DECLSPEC_XFGVIRT(ISWbemObject, References_)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *References_ )( 
            ISWbemLastError * This,
            /* [defaultvalue][in] */ BSTR strResultClass,
            /* [defaultvalue][in] */ BSTR strRole,
            /* [defaultvalue][in] */ VARIANT_BOOL bClassesOnly,
            /* [defaultvalue][in] */ VARIANT_BOOL bSchemaOnly,
            /* [defaultvalue][in] */ BSTR strRequiredQualifier,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [retval][out] */ ISWbemObjectSet **objWbemObjectSet);
        
        DECLSPEC_XFGVIRT(ISWbemObject, ReferencesAsync_)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ReferencesAsync_ )( 
            ISWbemLastError * This,
            /* [in] */ IDispatch *objWbemSink,
            /* [defaultvalue][in] */ BSTR strResultClass,
            /* [defaultvalue][in] */ BSTR strRole,
            /* [defaultvalue][in] */ VARIANT_BOOL bClassesOnly,
            /* [defaultvalue][in] */ VARIANT_BOOL bSchemaOnly,
            /* [defaultvalue][in] */ BSTR strRequiredQualifier,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [defaultvalue][in] */ IDispatch *objWbemAsyncContext);
        
        DECLSPEC_XFGVIRT(ISWbemObject, ExecMethod_)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ExecMethod_ )( 
            ISWbemLastError * This,
            /* [in] */ BSTR strMethodName,
            /* [defaultvalue][in] */ IDispatch *objWbemInParameters,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [retval][out] */ ISWbemObject **objWbemOutParameters);
        
        DECLSPEC_XFGVIRT(ISWbemObject, ExecMethodAsync_)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ExecMethodAsync_ )( 
            ISWbemLastError * This,
            /* [in] */ IDispatch *objWbemSink,
            /* [in] */ BSTR strMethodName,
            /* [defaultvalue][in] */ IDispatch *objWbemInParameters,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [defaultvalue][in] */ IDispatch *objWbemAsyncContext);
        
        DECLSPEC_XFGVIRT(ISWbemObject, Clone_)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Clone_ )( 
            ISWbemLastError * This,
            /* [retval][out] */ ISWbemObject **objWbemObject);
        
        DECLSPEC_XFGVIRT(ISWbemObject, GetObjectText_)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetObjectText_ )( 
            ISWbemLastError * This,
            /* [defaultvalue][in] */ long iFlags,
            /* [retval][out] */ BSTR *strObjectText);
        
        DECLSPEC_XFGVIRT(ISWbemObject, SpawnDerivedClass_)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SpawnDerivedClass_ )( 
            ISWbemLastError * This,
            /* [defaultvalue][in] */ long iFlags,
            /* [retval][out] */ ISWbemObject **objWbemObject);
        
        DECLSPEC_XFGVIRT(ISWbemObject, SpawnInstance_)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SpawnInstance_ )( 
            ISWbemLastError * This,
            /* [defaultvalue][in] */ long iFlags,
            /* [retval][out] */ ISWbemObject **objWbemObject);
        
        DECLSPEC_XFGVIRT(ISWbemObject, CompareTo_)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CompareTo_ )( 
            ISWbemLastError * This,
            /* [in] */ IDispatch *objWbemObject,
            /* [defaultvalue][in] */ long iFlags,
            /* [retval][out] */ VARIANT_BOOL *bResult);
        
        DECLSPEC_XFGVIRT(ISWbemObject, get_Qualifiers_)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Qualifiers_ )( 
            ISWbemLastError * This,
            /* [retval][out] */ ISWbemQualifierSet **objWbemQualifierSet);
        
        DECLSPEC_XFGVIRT(ISWbemObject, get_Properties_)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Properties_ )( 
            ISWbemLastError * This,
            /* [retval][out] */ ISWbemPropertySet **objWbemPropertySet);
        
        DECLSPEC_XFGVIRT(ISWbemObject, get_Methods_)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Methods_ )( 
            ISWbemLastError * This,
            /* [retval][out] */ ISWbemMethodSet **objWbemMethodSet);
        
        DECLSPEC_XFGVIRT(ISWbemObject, get_Derivation_)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Derivation_ )( 
            ISWbemLastError * This,
            /* [retval][out] */ VARIANT *strClassNameArray);
        
        DECLSPEC_XFGVIRT(ISWbemObject, get_Path_)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Path_ )( 
            ISWbemLastError * This,
            /* [retval][out] */ ISWbemObjectPath **objWbemObjectPath);
        
        DECLSPEC_XFGVIRT(ISWbemObject, get_Security_)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Security_ )( 
            ISWbemLastError * This,
            /* [retval][out] */ ISWbemSecurity **objWbemSecurity);
        
        END_INTERFACE
    } ISWbemLastErrorVtbl;

    interface ISWbemLastError
    {
        CONST_VTBL struct ISWbemLastErrorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISWbemLastError_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISWbemLastError_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISWbemLastError_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISWbemLastError_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISWbemLastError_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISWbemLastError_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISWbemLastError_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISWbemLastError_Put_(This,iFlags,objWbemNamedValueSet,objWbemObjectPath)	\
    ( (This)->lpVtbl -> Put_(This,iFlags,objWbemNamedValueSet,objWbemObjectPath) ) 

#define ISWbemLastError_PutAsync_(This,objWbemSink,iFlags,objWbemNamedValueSet,objWbemAsyncContext)	\
    ( (This)->lpVtbl -> PutAsync_(This,objWbemSink,iFlags,objWbemNamedValueSet,objWbemAsyncContext) ) 

#define ISWbemLastError_Delete_(This,iFlags,objWbemNamedValueSet)	\
    ( (This)->lpVtbl -> Delete_(This,iFlags,objWbemNamedValueSet) ) 

#define ISWbemLastError_DeleteAsync_(This,objWbemSink,iFlags,objWbemNamedValueSet,objWbemAsyncContext)	\
    ( (This)->lpVtbl -> DeleteAsync_(This,objWbemSink,iFlags,objWbemNamedValueSet,objWbemAsyncContext) ) 

#define ISWbemLastError_Instances_(This,iFlags,objWbemNamedValueSet,objWbemObjectSet)	\
    ( (This)->lpVtbl -> Instances_(This,iFlags,objWbemNamedValueSet,objWbemObjectSet) ) 

#define ISWbemLastError_InstancesAsync_(This,objWbemSink,iFlags,objWbemNamedValueSet,objWbemAsyncContext)	\
    ( (This)->lpVtbl -> InstancesAsync_(This,objWbemSink,iFlags,objWbemNamedValueSet,objWbemAsyncContext) ) 

#define ISWbemLastError_Subclasses_(This,iFlags,objWbemNamedValueSet,objWbemObjectSet)	\
    ( (This)->lpVtbl -> Subclasses_(This,iFlags,objWbemNamedValueSet,objWbemObjectSet) ) 

#define ISWbemLastError_SubclassesAsync_(This,objWbemSink,iFlags,objWbemNamedValueSet,objWbemAsyncContext)	\
    ( (This)->lpVtbl -> SubclassesAsync_(This,objWbemSink,iFlags,objWbemNamedValueSet,objWbemAsyncContext) ) 

#define ISWbemLastError_Associators_(This,strAssocClass,strResultClass,strResultRole,strRole,bClassesOnly,bSchemaOnly,strRequiredAssocQualifier,strRequiredQualifier,iFlags,objWbemNamedValueSet,objWbemObjectSet)	\
    ( (This)->lpVtbl -> Associators_(This,strAssocClass,strResultClass,strResultRole,strRole,bClassesOnly,bSchemaOnly,strRequiredAssocQualifier,strRequiredQualifier,iFlags,objWbemNamedValueSet,objWbemObjectSet) ) 

#define ISWbemLastError_AssociatorsAsync_(This,objWbemSink,strAssocClass,strResultClass,strResultRole,strRole,bClassesOnly,bSchemaOnly,strRequiredAssocQualifier,strRequiredQualifier,iFlags,objWbemNamedValueSet,objWbemAsyncContext)	\
    ( (This)->lpVtbl -> AssociatorsAsync_(This,objWbemSink,strAssocClass,strResultClass,strResultRole,strRole,bClassesOnly,bSchemaOnly,strRequiredAssocQualifier,strRequiredQualifier,iFlags,objWbemNamedValueSet,objWbemAsyncContext) ) 

#define ISWbemLastError_References_(This,strResultClass,strRole,bClassesOnly,bSchemaOnly,strRequiredQualifier,iFlags,objWbemNamedValueSet,objWbemObjectSet)	\
    ( (This)->lpVtbl -> References_(This,strResultClass,strRole,bClassesOnly,bSchemaOnly,strRequiredQualifier,iFlags,objWbemNamedValueSet,objWbemObjectSet) ) 

#define ISWbemLastError_ReferencesAsync_(This,objWbemSink,strResultClass,strRole,bClassesOnly,bSchemaOnly,strRequiredQualifier,iFlags,objWbemNamedValueSet,objWbemAsyncContext)	\
    ( (This)->lpVtbl -> ReferencesAsync_(This,objWbemSink,strResultClass,strRole,bClassesOnly,bSchemaOnly,strRequiredQualifier,iFlags,objWbemNamedValueSet,objWbemAsyncContext) ) 

#define ISWbemLastError_ExecMethod_(This,strMethodName,objWbemInParameters,iFlags,objWbemNamedValueSet,objWbemOutParameters)	\
    ( (This)->lpVtbl -> ExecMethod_(This,strMethodName,objWbemInParameters,iFlags,objWbemNamedValueSet,objWbemOutParameters) ) 

#define ISWbemLastError_ExecMethodAsync_(This,objWbemSink,strMethodName,objWbemInParameters,iFlags,objWbemNamedValueSet,objWbemAsyncContext)	\
    ( (This)->lpVtbl -> ExecMethodAsync_(This,objWbemSink,strMethodName,objWbemInParameters,iFlags,objWbemNamedValueSet,objWbemAsyncContext) ) 

#define ISWbemLastError_Clone_(This,objWbemObject)	\
    ( (This)->lpVtbl -> Clone_(This,objWbemObject) ) 

#define ISWbemLastError_GetObjectText_(This,iFlags,strObjectText)	\
    ( (This)->lpVtbl -> GetObjectText_(This,iFlags,strObjectText) ) 

#define ISWbemLastError_SpawnDerivedClass_(This,iFlags,objWbemObject)	\
    ( (This)->lpVtbl -> SpawnDerivedClass_(This,iFlags,objWbemObject) ) 

#define ISWbemLastError_SpawnInstance_(This,iFlags,objWbemObject)	\
    ( (This)->lpVtbl -> SpawnInstance_(This,iFlags,objWbemObject) ) 

#define ISWbemLastError_CompareTo_(This,objWbemObject,iFlags,bResult)	\
    ( (This)->lpVtbl -> CompareTo_(This,objWbemObject,iFlags,bResult) ) 

#define ISWbemLastError_get_Qualifiers_(This,objWbemQualifierSet)	\
    ( (This)->lpVtbl -> get_Qualifiers_(This,objWbemQualifierSet) ) 

#define ISWbemLastError_get_Properties_(This,objWbemPropertySet)	\
    ( (This)->lpVtbl -> get_Properties_(This,objWbemPropertySet) ) 

#define ISWbemLastError_get_Methods_(This,objWbemMethodSet)	\
    ( (This)->lpVtbl -> get_Methods_(This,objWbemMethodSet) ) 

#define ISWbemLastError_get_Derivation_(This,strClassNameArray)	\
    ( (This)->lpVtbl -> get_Derivation_(This,strClassNameArray) ) 

#define ISWbemLastError_get_Path_(This,objWbemObjectPath)	\
    ( (This)->lpVtbl -> get_Path_(This,objWbemObjectPath) ) 

#define ISWbemLastError_get_Security_(This,objWbemSecurity)	\
    ( (This)->lpVtbl -> get_Security_(This,objWbemSecurity) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISWbemLastError_INTERFACE_DEFINED__ */


#ifndef __ISWbemSinkEvents_DISPINTERFACE_DEFINED__
#define __ISWbemSinkEvents_DISPINTERFACE_DEFINED__

/* dispinterface ISWbemSinkEvents */
/* [hidden][nonextensible][helpstring][uuid] */ 


EXTERN_C const IID DIID_ISWbemSinkEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)

    MIDL_INTERFACE("75718CA0-F029-11d1-A1AC-00C04FB6C223")
    ISWbemSinkEvents : public IDispatch
    {
    };
    
#else 	/* C style interface */

    typedef struct ISWbemSinkEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISWbemSinkEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISWbemSinkEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISWbemSinkEvents * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ISWbemSinkEvents * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ISWbemSinkEvents * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ISWbemSinkEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISWbemSinkEvents * This,
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
    } ISWbemSinkEventsVtbl;

    interface ISWbemSinkEvents
    {
        CONST_VTBL struct ISWbemSinkEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISWbemSinkEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISWbemSinkEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISWbemSinkEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISWbemSinkEvents_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISWbemSinkEvents_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISWbemSinkEvents_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISWbemSinkEvents_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */


#endif 	/* __ISWbemSinkEvents_DISPINTERFACE_DEFINED__ */


#ifndef __ISWbemSink_INTERFACE_DEFINED__
#define __ISWbemSink_INTERFACE_DEFINED__

/* interface ISWbemSink */
/* [helpstring][nonextensible][hidden][dual][oleautomation][uuid][object] */ 


EXTERN_C const IID IID_ISWbemSink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("75718C9F-F029-11d1-A1AC-00C04FB6C223")
    ISWbemSink : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Cancel( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISWbemSinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISWbemSink * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISWbemSink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISWbemSink * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ISWbemSink * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ISWbemSink * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ISWbemSink * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISWbemSink * This,
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
        
        DECLSPEC_XFGVIRT(ISWbemSink, Cancel)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Cancel )( 
            __RPC__in ISWbemSink * This);
        
        END_INTERFACE
    } ISWbemSinkVtbl;

    interface ISWbemSink
    {
        CONST_VTBL struct ISWbemSinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISWbemSink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISWbemSink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISWbemSink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISWbemSink_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISWbemSink_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISWbemSink_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISWbemSink_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISWbemSink_Cancel(This)	\
    ( (This)->lpVtbl -> Cancel(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISWbemSink_INTERFACE_DEFINED__ */


#ifndef __ISWbemSecurity_INTERFACE_DEFINED__
#define __ISWbemSecurity_INTERFACE_DEFINED__

/* interface ISWbemSecurity */
/* [helpstring][hidden][nonextensible][dual][oleautomation][uuid][local][object] */ 


EXTERN_C const IID IID_ISWbemSecurity;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B54D66E6-2287-11d2-8B33-00600806D9B6")
    ISWbemSecurity : public IDispatch
    {
    public:
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_ImpersonationLevel( 
            /* [retval][out] */ WbemImpersonationLevelEnum *iImpersonationLevel) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE put_ImpersonationLevel( 
            /* [in] */ WbemImpersonationLevelEnum iImpersonationLevel) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_AuthenticationLevel( 
            /* [retval][out] */ WbemAuthenticationLevelEnum *iAuthenticationLevel) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE put_AuthenticationLevel( 
            /* [in] */ WbemAuthenticationLevelEnum iAuthenticationLevel) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Privileges( 
            /* [retval][out] */ ISWbemPrivilegeSet **objWbemPrivilegeSet) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISWbemSecurityVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISWbemSecurity * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISWbemSecurity * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISWbemSecurity * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            ISWbemSecurity * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            ISWbemSecurity * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            ISWbemSecurity * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISWbemSecurity * This,
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
        
        DECLSPEC_XFGVIRT(ISWbemSecurity, get_ImpersonationLevel)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ImpersonationLevel )( 
            ISWbemSecurity * This,
            /* [retval][out] */ WbemImpersonationLevelEnum *iImpersonationLevel);
        
        DECLSPEC_XFGVIRT(ISWbemSecurity, put_ImpersonationLevel)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ImpersonationLevel )( 
            ISWbemSecurity * This,
            /* [in] */ WbemImpersonationLevelEnum iImpersonationLevel);
        
        DECLSPEC_XFGVIRT(ISWbemSecurity, get_AuthenticationLevel)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_AuthenticationLevel )( 
            ISWbemSecurity * This,
            /* [retval][out] */ WbemAuthenticationLevelEnum *iAuthenticationLevel);
        
        DECLSPEC_XFGVIRT(ISWbemSecurity, put_AuthenticationLevel)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_AuthenticationLevel )( 
            ISWbemSecurity * This,
            /* [in] */ WbemAuthenticationLevelEnum iAuthenticationLevel);
        
        DECLSPEC_XFGVIRT(ISWbemSecurity, get_Privileges)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Privileges )( 
            ISWbemSecurity * This,
            /* [retval][out] */ ISWbemPrivilegeSet **objWbemPrivilegeSet);
        
        END_INTERFACE
    } ISWbemSecurityVtbl;

    interface ISWbemSecurity
    {
        CONST_VTBL struct ISWbemSecurityVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISWbemSecurity_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISWbemSecurity_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISWbemSecurity_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISWbemSecurity_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISWbemSecurity_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISWbemSecurity_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISWbemSecurity_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISWbemSecurity_get_ImpersonationLevel(This,iImpersonationLevel)	\
    ( (This)->lpVtbl -> get_ImpersonationLevel(This,iImpersonationLevel) ) 

#define ISWbemSecurity_put_ImpersonationLevel(This,iImpersonationLevel)	\
    ( (This)->lpVtbl -> put_ImpersonationLevel(This,iImpersonationLevel) ) 

#define ISWbemSecurity_get_AuthenticationLevel(This,iAuthenticationLevel)	\
    ( (This)->lpVtbl -> get_AuthenticationLevel(This,iAuthenticationLevel) ) 

#define ISWbemSecurity_put_AuthenticationLevel(This,iAuthenticationLevel)	\
    ( (This)->lpVtbl -> put_AuthenticationLevel(This,iAuthenticationLevel) ) 

#define ISWbemSecurity_get_Privileges(This,objWbemPrivilegeSet)	\
    ( (This)->lpVtbl -> get_Privileges(This,objWbemPrivilegeSet) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISWbemSecurity_INTERFACE_DEFINED__ */


#ifndef __ISWbemPrivilege_INTERFACE_DEFINED__
#define __ISWbemPrivilege_INTERFACE_DEFINED__

/* interface ISWbemPrivilege */
/* [helpstring][hidden][nonextensible][dual][oleautomation][uuid][local][object] */ 


EXTERN_C const IID IID_ISWbemPrivilege;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("26EE67BD-5804-11d2-8B4A-00600806D9B6")
    ISWbemPrivilege : public IDispatch
    {
    public:
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_IsEnabled( 
            /* [retval][out] */ VARIANT_BOOL *bIsEnabled) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE put_IsEnabled( 
            /* [in] */ VARIANT_BOOL bIsEnabled) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ BSTR *strDisplayName) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_DisplayName( 
            /* [retval][out] */ BSTR *strDisplayName) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Identifier( 
            /* [retval][out] */ WbemPrivilegeEnum *iPrivilege) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISWbemPrivilegeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISWbemPrivilege * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISWbemPrivilege * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISWbemPrivilege * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            ISWbemPrivilege * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            ISWbemPrivilege * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            ISWbemPrivilege * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISWbemPrivilege * This,
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
        
        DECLSPEC_XFGVIRT(ISWbemPrivilege, get_IsEnabled)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_IsEnabled )( 
            ISWbemPrivilege * This,
            /* [retval][out] */ VARIANT_BOOL *bIsEnabled);
        
        DECLSPEC_XFGVIRT(ISWbemPrivilege, put_IsEnabled)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_IsEnabled )( 
            ISWbemPrivilege * This,
            /* [in] */ VARIANT_BOOL bIsEnabled);
        
        DECLSPEC_XFGVIRT(ISWbemPrivilege, get_Name)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            ISWbemPrivilege * This,
            /* [retval][out] */ BSTR *strDisplayName);
        
        DECLSPEC_XFGVIRT(ISWbemPrivilege, get_DisplayName)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_DisplayName )( 
            ISWbemPrivilege * This,
            /* [retval][out] */ BSTR *strDisplayName);
        
        DECLSPEC_XFGVIRT(ISWbemPrivilege, get_Identifier)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Identifier )( 
            ISWbemPrivilege * This,
            /* [retval][out] */ WbemPrivilegeEnum *iPrivilege);
        
        END_INTERFACE
    } ISWbemPrivilegeVtbl;

    interface ISWbemPrivilege
    {
        CONST_VTBL struct ISWbemPrivilegeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISWbemPrivilege_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISWbemPrivilege_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISWbemPrivilege_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISWbemPrivilege_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISWbemPrivilege_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISWbemPrivilege_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISWbemPrivilege_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISWbemPrivilege_get_IsEnabled(This,bIsEnabled)	\
    ( (This)->lpVtbl -> get_IsEnabled(This,bIsEnabled) ) 

#define ISWbemPrivilege_put_IsEnabled(This,bIsEnabled)	\
    ( (This)->lpVtbl -> put_IsEnabled(This,bIsEnabled) ) 

#define ISWbemPrivilege_get_Name(This,strDisplayName)	\
    ( (This)->lpVtbl -> get_Name(This,strDisplayName) ) 

#define ISWbemPrivilege_get_DisplayName(This,strDisplayName)	\
    ( (This)->lpVtbl -> get_DisplayName(This,strDisplayName) ) 

#define ISWbemPrivilege_get_Identifier(This,iPrivilege)	\
    ( (This)->lpVtbl -> get_Identifier(This,iPrivilege) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISWbemPrivilege_INTERFACE_DEFINED__ */


#ifndef __ISWbemPrivilegeSet_INTERFACE_DEFINED__
#define __ISWbemPrivilegeSet_INTERFACE_DEFINED__

/* interface ISWbemPrivilegeSet */
/* [helpstring][nonextensible][hidden][dual][oleautomation][uuid][local][object] */ 


EXTERN_C const IID IID_ISWbemPrivilegeSet;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("26EE67BF-5804-11d2-8B4A-00600806D9B6")
    ISWbemPrivilegeSet : public IDispatch
    {
    public:
        virtual /* [restricted][propget][id] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ IUnknown **pUnk) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Item( 
            /* [in] */ WbemPrivilegeEnum iPrivilege,
            /* [retval][out] */ ISWbemPrivilege **objWbemPrivilege) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ long *iCount) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Add( 
            /* [in] */ WbemPrivilegeEnum iPrivilege,
            /* [defaultvalue][in] */ VARIANT_BOOL bIsEnabled,
            /* [retval][out] */ ISWbemPrivilege **objWbemPrivilege) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Remove( 
            /* [in] */ WbemPrivilegeEnum iPrivilege) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE DeleteAll( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE AddAsString( 
            /* [in] */ BSTR strPrivilege,
            /* [defaultvalue][in] */ VARIANT_BOOL bIsEnabled,
            /* [retval][out] */ ISWbemPrivilege **objWbemPrivilege) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISWbemPrivilegeSetVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISWbemPrivilegeSet * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISWbemPrivilegeSet * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISWbemPrivilegeSet * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            ISWbemPrivilegeSet * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            ISWbemPrivilegeSet * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            ISWbemPrivilegeSet * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISWbemPrivilegeSet * This,
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
        
        DECLSPEC_XFGVIRT(ISWbemPrivilegeSet, get__NewEnum)
        /* [restricted][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            ISWbemPrivilegeSet * This,
            /* [retval][out] */ IUnknown **pUnk);
        
        DECLSPEC_XFGVIRT(ISWbemPrivilegeSet, Item)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Item )( 
            ISWbemPrivilegeSet * This,
            /* [in] */ WbemPrivilegeEnum iPrivilege,
            /* [retval][out] */ ISWbemPrivilege **objWbemPrivilege);
        
        DECLSPEC_XFGVIRT(ISWbemPrivilegeSet, get_Count)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            ISWbemPrivilegeSet * This,
            /* [retval][out] */ long *iCount);
        
        DECLSPEC_XFGVIRT(ISWbemPrivilegeSet, Add)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Add )( 
            ISWbemPrivilegeSet * This,
            /* [in] */ WbemPrivilegeEnum iPrivilege,
            /* [defaultvalue][in] */ VARIANT_BOOL bIsEnabled,
            /* [retval][out] */ ISWbemPrivilege **objWbemPrivilege);
        
        DECLSPEC_XFGVIRT(ISWbemPrivilegeSet, Remove)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Remove )( 
            ISWbemPrivilegeSet * This,
            /* [in] */ WbemPrivilegeEnum iPrivilege);
        
        DECLSPEC_XFGVIRT(ISWbemPrivilegeSet, DeleteAll)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *DeleteAll )( 
            ISWbemPrivilegeSet * This);
        
        DECLSPEC_XFGVIRT(ISWbemPrivilegeSet, AddAsString)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *AddAsString )( 
            ISWbemPrivilegeSet * This,
            /* [in] */ BSTR strPrivilege,
            /* [defaultvalue][in] */ VARIANT_BOOL bIsEnabled,
            /* [retval][out] */ ISWbemPrivilege **objWbemPrivilege);
        
        END_INTERFACE
    } ISWbemPrivilegeSetVtbl;

    interface ISWbemPrivilegeSet
    {
        CONST_VTBL struct ISWbemPrivilegeSetVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISWbemPrivilegeSet_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISWbemPrivilegeSet_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISWbemPrivilegeSet_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISWbemPrivilegeSet_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISWbemPrivilegeSet_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISWbemPrivilegeSet_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISWbemPrivilegeSet_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISWbemPrivilegeSet_get__NewEnum(This,pUnk)	\
    ( (This)->lpVtbl -> get__NewEnum(This,pUnk) ) 

#define ISWbemPrivilegeSet_Item(This,iPrivilege,objWbemPrivilege)	\
    ( (This)->lpVtbl -> Item(This,iPrivilege,objWbemPrivilege) ) 

#define ISWbemPrivilegeSet_get_Count(This,iCount)	\
    ( (This)->lpVtbl -> get_Count(This,iCount) ) 

#define ISWbemPrivilegeSet_Add(This,iPrivilege,bIsEnabled,objWbemPrivilege)	\
    ( (This)->lpVtbl -> Add(This,iPrivilege,bIsEnabled,objWbemPrivilege) ) 

#define ISWbemPrivilegeSet_Remove(This,iPrivilege)	\
    ( (This)->lpVtbl -> Remove(This,iPrivilege) ) 

#define ISWbemPrivilegeSet_DeleteAll(This)	\
    ( (This)->lpVtbl -> DeleteAll(This) ) 

#define ISWbemPrivilegeSet_AddAsString(This,strPrivilege,bIsEnabled,objWbemPrivilege)	\
    ( (This)->lpVtbl -> AddAsString(This,strPrivilege,bIsEnabled,objWbemPrivilege) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISWbemPrivilegeSet_INTERFACE_DEFINED__ */


#ifndef __ISWbemServicesEx_INTERFACE_DEFINED__
#define __ISWbemServicesEx_INTERFACE_DEFINED__

/* interface ISWbemServicesEx */
/* [helpstring][hidden][unique][nonextensible][dual][oleautomation][uuid][object][local] */ 


EXTERN_C const IID IID_ISWbemServicesEx;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D2F68443-85DC-427e-91D8-366554CC754C")
    ISWbemServicesEx : public ISWbemServices
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Put( 
            /* [in] */ ISWbemObjectEx *objWbemObject,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [retval][out] */ ISWbemObjectPath **objWbemObjectPath) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE PutAsync( 
            /* [in] */ ISWbemSink *objWbemSink,
            /* [in] */ ISWbemObjectEx *objWbemObject,
            /* [defaultvalue][in] */ long iFlags = wbemChangeFlagCreateOrUpdate,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet = 0,
            /* [defaultvalue][in] */ IDispatch *objWbemAsyncContext = 0) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISWbemServicesExVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISWbemServicesEx * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISWbemServicesEx * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISWbemServicesEx * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            ISWbemServicesEx * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            ISWbemServicesEx * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            ISWbemServicesEx * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISWbemServicesEx * This,
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
        
        DECLSPEC_XFGVIRT(ISWbemServices, Get)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Get )( 
            ISWbemServicesEx * This,
            /* [defaultvalue][in] */ BSTR strObjectPath,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [retval][out] */ ISWbemObject **objWbemObject);
        
        DECLSPEC_XFGVIRT(ISWbemServices, GetAsync)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetAsync )( 
            ISWbemServicesEx * This,
            /* [in] */ IDispatch *objWbemSink,
            /* [defaultvalue][in] */ BSTR strObjectPath,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [defaultvalue][in] */ IDispatch *objWbemAsyncContext);
        
        DECLSPEC_XFGVIRT(ISWbemServices, Delete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            ISWbemServicesEx * This,
            /* [in] */ BSTR strObjectPath,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet);
        
        DECLSPEC_XFGVIRT(ISWbemServices, DeleteAsync)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *DeleteAsync )( 
            ISWbemServicesEx * This,
            /* [in] */ IDispatch *objWbemSink,
            /* [in] */ BSTR strObjectPath,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [defaultvalue][in] */ IDispatch *objWbemAsyncContext);
        
        DECLSPEC_XFGVIRT(ISWbemServices, InstancesOf)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *InstancesOf )( 
            ISWbemServicesEx * This,
            /* [in] */ BSTR strClass,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [retval][out] */ ISWbemObjectSet **objWbemObjectSet);
        
        DECLSPEC_XFGVIRT(ISWbemServices, InstancesOfAsync)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *InstancesOfAsync )( 
            ISWbemServicesEx * This,
            /* [in] */ IDispatch *objWbemSink,
            /* [in] */ BSTR strClass,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [defaultvalue][in] */ IDispatch *objWbemAsyncContext);
        
        DECLSPEC_XFGVIRT(ISWbemServices, SubclassesOf)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SubclassesOf )( 
            ISWbemServicesEx * This,
            /* [defaultvalue][in] */ BSTR strSuperclass,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [retval][out] */ ISWbemObjectSet **objWbemObjectSet);
        
        DECLSPEC_XFGVIRT(ISWbemServices, SubclassesOfAsync)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SubclassesOfAsync )( 
            ISWbemServicesEx * This,
            /* [in] */ IDispatch *objWbemSink,
            /* [defaultvalue][in] */ BSTR strSuperclass,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [defaultvalue][in] */ IDispatch *objWbemAsyncContext);
        
        DECLSPEC_XFGVIRT(ISWbemServices, ExecQuery)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ExecQuery )( 
            ISWbemServicesEx * This,
            /* [in] */ BSTR strQuery,
            /* [defaultvalue][in] */ BSTR strQueryLanguage,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [retval][out] */ ISWbemObjectSet **objWbemObjectSet);
        
        DECLSPEC_XFGVIRT(ISWbemServices, ExecQueryAsync)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ExecQueryAsync )( 
            ISWbemServicesEx * This,
            /* [in] */ IDispatch *objWbemSink,
            /* [in] */ BSTR strQuery,
            /* [defaultvalue][in] */ BSTR strQueryLanguage,
            /* [defaultvalue][in] */ long lFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [defaultvalue][in] */ IDispatch *objWbemAsyncContext);
        
        DECLSPEC_XFGVIRT(ISWbemServices, AssociatorsOf)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *AssociatorsOf )( 
            ISWbemServicesEx * This,
            /* [in] */ BSTR strObjectPath,
            /* [defaultvalue][in] */ BSTR strAssocClass,
            /* [defaultvalue][in] */ BSTR strResultClass,
            /* [defaultvalue][in] */ BSTR strResultRole,
            /* [defaultvalue][in] */ BSTR strRole,
            /* [defaultvalue][in] */ VARIANT_BOOL bClassesOnly,
            /* [defaultvalue][in] */ VARIANT_BOOL bSchemaOnly,
            /* [defaultvalue][in] */ BSTR strRequiredAssocQualifier,
            /* [defaultvalue][in] */ BSTR strRequiredQualifier,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [retval][out] */ ISWbemObjectSet **objWbemObjectSet);
        
        DECLSPEC_XFGVIRT(ISWbemServices, AssociatorsOfAsync)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *AssociatorsOfAsync )( 
            ISWbemServicesEx * This,
            /* [in] */ IDispatch *objWbemSink,
            /* [in] */ BSTR strObjectPath,
            /* [defaultvalue][in] */ BSTR strAssocClass,
            /* [defaultvalue][in] */ BSTR strResultClass,
            /* [defaultvalue][in] */ BSTR strResultRole,
            /* [defaultvalue][in] */ BSTR strRole,
            /* [defaultvalue][in] */ VARIANT_BOOL bClassesOnly,
            /* [defaultvalue][in] */ VARIANT_BOOL bSchemaOnly,
            /* [defaultvalue][in] */ BSTR strRequiredAssocQualifier,
            /* [defaultvalue][in] */ BSTR strRequiredQualifier,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [defaultvalue][in] */ IDispatch *objWbemAsyncContext);
        
        DECLSPEC_XFGVIRT(ISWbemServices, ReferencesTo)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ReferencesTo )( 
            ISWbemServicesEx * This,
            /* [in] */ BSTR strObjectPath,
            /* [defaultvalue][in] */ BSTR strResultClass,
            /* [defaultvalue][in] */ BSTR strRole,
            /* [defaultvalue][in] */ VARIANT_BOOL bClassesOnly,
            /* [defaultvalue][in] */ VARIANT_BOOL bSchemaOnly,
            /* [defaultvalue][in] */ BSTR strRequiredQualifier,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [retval][out] */ ISWbemObjectSet **objWbemObjectSet);
        
        DECLSPEC_XFGVIRT(ISWbemServices, ReferencesToAsync)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ReferencesToAsync )( 
            ISWbemServicesEx * This,
            /* [in] */ IDispatch *objWbemSink,
            /* [in] */ BSTR strObjectPath,
            /* [defaultvalue][in] */ BSTR strResultClass,
            /* [defaultvalue][in] */ BSTR strRole,
            /* [defaultvalue][in] */ VARIANT_BOOL bClassesOnly,
            /* [defaultvalue][in] */ VARIANT_BOOL bSchemaOnly,
            /* [defaultvalue][in] */ BSTR strRequiredQualifier,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [defaultvalue][in] */ IDispatch *objWbemAsyncContext);
        
        DECLSPEC_XFGVIRT(ISWbemServices, ExecNotificationQuery)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ExecNotificationQuery )( 
            ISWbemServicesEx * This,
            /* [in] */ BSTR strQuery,
            /* [defaultvalue][in] */ BSTR strQueryLanguage,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [retval][out] */ ISWbemEventSource **objWbemEventSource);
        
        DECLSPEC_XFGVIRT(ISWbemServices, ExecNotificationQueryAsync)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ExecNotificationQueryAsync )( 
            ISWbemServicesEx * This,
            /* [in] */ IDispatch *objWbemSink,
            /* [in] */ BSTR strQuery,
            /* [defaultvalue][in] */ BSTR strQueryLanguage,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [defaultvalue][in] */ IDispatch *objWbemAsyncContext);
        
        DECLSPEC_XFGVIRT(ISWbemServices, ExecMethod)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ExecMethod )( 
            ISWbemServicesEx * This,
            /* [in] */ BSTR strObjectPath,
            /* [in] */ BSTR strMethodName,
            /* [defaultvalue][in] */ IDispatch *objWbemInParameters,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [retval][out] */ ISWbemObject **objWbemOutParameters);
        
        DECLSPEC_XFGVIRT(ISWbemServices, ExecMethodAsync)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ExecMethodAsync )( 
            ISWbemServicesEx * This,
            /* [in] */ IDispatch *objWbemSink,
            /* [in] */ BSTR strObjectPath,
            /* [in] */ BSTR strMethodName,
            /* [defaultvalue][in] */ IDispatch *objWbemInParameters,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [defaultvalue][in] */ IDispatch *objWbemAsyncContext);
        
        DECLSPEC_XFGVIRT(ISWbemServices, get_Security_)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Security_ )( 
            ISWbemServicesEx * This,
            /* [retval][out] */ ISWbemSecurity **objWbemSecurity);
        
        DECLSPEC_XFGVIRT(ISWbemServicesEx, Put)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Put )( 
            ISWbemServicesEx * This,
            /* [in] */ ISWbemObjectEx *objWbemObject,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [retval][out] */ ISWbemObjectPath **objWbemObjectPath);
        
        DECLSPEC_XFGVIRT(ISWbemServicesEx, PutAsync)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *PutAsync )( 
            ISWbemServicesEx * This,
            /* [in] */ ISWbemSink *objWbemSink,
            /* [in] */ ISWbemObjectEx *objWbemObject,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [defaultvalue][in] */ IDispatch *objWbemAsyncContext);
        
        END_INTERFACE
    } ISWbemServicesExVtbl;

    interface ISWbemServicesEx
    {
        CONST_VTBL struct ISWbemServicesExVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISWbemServicesEx_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISWbemServicesEx_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISWbemServicesEx_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISWbemServicesEx_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISWbemServicesEx_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISWbemServicesEx_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISWbemServicesEx_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISWbemServicesEx_Get(This,strObjectPath,iFlags,objWbemNamedValueSet,objWbemObject)	\
    ( (This)->lpVtbl -> Get(This,strObjectPath,iFlags,objWbemNamedValueSet,objWbemObject) ) 

#define ISWbemServicesEx_GetAsync(This,objWbemSink,strObjectPath,iFlags,objWbemNamedValueSet,objWbemAsyncContext)	\
    ( (This)->lpVtbl -> GetAsync(This,objWbemSink,strObjectPath,iFlags,objWbemNamedValueSet,objWbemAsyncContext) ) 

#define ISWbemServicesEx_Delete(This,strObjectPath,iFlags,objWbemNamedValueSet)	\
    ( (This)->lpVtbl -> Delete(This,strObjectPath,iFlags,objWbemNamedValueSet) ) 

#define ISWbemServicesEx_DeleteAsync(This,objWbemSink,strObjectPath,iFlags,objWbemNamedValueSet,objWbemAsyncContext)	\
    ( (This)->lpVtbl -> DeleteAsync(This,objWbemSink,strObjectPath,iFlags,objWbemNamedValueSet,objWbemAsyncContext) ) 

#define ISWbemServicesEx_InstancesOf(This,strClass,iFlags,objWbemNamedValueSet,objWbemObjectSet)	\
    ( (This)->lpVtbl -> InstancesOf(This,strClass,iFlags,objWbemNamedValueSet,objWbemObjectSet) ) 

#define ISWbemServicesEx_InstancesOfAsync(This,objWbemSink,strClass,iFlags,objWbemNamedValueSet,objWbemAsyncContext)	\
    ( (This)->lpVtbl -> InstancesOfAsync(This,objWbemSink,strClass,iFlags,objWbemNamedValueSet,objWbemAsyncContext) ) 

#define ISWbemServicesEx_SubclassesOf(This,strSuperclass,iFlags,objWbemNamedValueSet,objWbemObjectSet)	\
    ( (This)->lpVtbl -> SubclassesOf(This,strSuperclass,iFlags,objWbemNamedValueSet,objWbemObjectSet) ) 

#define ISWbemServicesEx_SubclassesOfAsync(This,objWbemSink,strSuperclass,iFlags,objWbemNamedValueSet,objWbemAsyncContext)	\
    ( (This)->lpVtbl -> SubclassesOfAsync(This,objWbemSink,strSuperclass,iFlags,objWbemNamedValueSet,objWbemAsyncContext) ) 

#define ISWbemServicesEx_ExecQuery(This,strQuery,strQueryLanguage,iFlags,objWbemNamedValueSet,objWbemObjectSet)	\
    ( (This)->lpVtbl -> ExecQuery(This,strQuery,strQueryLanguage,iFlags,objWbemNamedValueSet,objWbemObjectSet) ) 

#define ISWbemServicesEx_ExecQueryAsync(This,objWbemSink,strQuery,strQueryLanguage,lFlags,objWbemNamedValueSet,objWbemAsyncContext)	\
    ( (This)->lpVtbl -> ExecQueryAsync(This,objWbemSink,strQuery,strQueryLanguage,lFlags,objWbemNamedValueSet,objWbemAsyncContext) ) 

#define ISWbemServicesEx_AssociatorsOf(This,strObjectPath,strAssocClass,strResultClass,strResultRole,strRole,bClassesOnly,bSchemaOnly,strRequiredAssocQualifier,strRequiredQualifier,iFlags,objWbemNamedValueSet,objWbemObjectSet)	\
    ( (This)->lpVtbl -> AssociatorsOf(This,strObjectPath,strAssocClass,strResultClass,strResultRole,strRole,bClassesOnly,bSchemaOnly,strRequiredAssocQualifier,strRequiredQualifier,iFlags,objWbemNamedValueSet,objWbemObjectSet) ) 

#define ISWbemServicesEx_AssociatorsOfAsync(This,objWbemSink,strObjectPath,strAssocClass,strResultClass,strResultRole,strRole,bClassesOnly,bSchemaOnly,strRequiredAssocQualifier,strRequiredQualifier,iFlags,objWbemNamedValueSet,objWbemAsyncContext)	\
    ( (This)->lpVtbl -> AssociatorsOfAsync(This,objWbemSink,strObjectPath,strAssocClass,strResultClass,strResultRole,strRole,bClassesOnly,bSchemaOnly,strRequiredAssocQualifier,strRequiredQualifier,iFlags,objWbemNamedValueSet,objWbemAsyncContext) ) 

#define ISWbemServicesEx_ReferencesTo(This,strObjectPath,strResultClass,strRole,bClassesOnly,bSchemaOnly,strRequiredQualifier,iFlags,objWbemNamedValueSet,objWbemObjectSet)	\
    ( (This)->lpVtbl -> ReferencesTo(This,strObjectPath,strResultClass,strRole,bClassesOnly,bSchemaOnly,strRequiredQualifier,iFlags,objWbemNamedValueSet,objWbemObjectSet) ) 

#define ISWbemServicesEx_ReferencesToAsync(This,objWbemSink,strObjectPath,strResultClass,strRole,bClassesOnly,bSchemaOnly,strRequiredQualifier,iFlags,objWbemNamedValueSet,objWbemAsyncContext)	\
    ( (This)->lpVtbl -> ReferencesToAsync(This,objWbemSink,strObjectPath,strResultClass,strRole,bClassesOnly,bSchemaOnly,strRequiredQualifier,iFlags,objWbemNamedValueSet,objWbemAsyncContext) ) 

#define ISWbemServicesEx_ExecNotificationQuery(This,strQuery,strQueryLanguage,iFlags,objWbemNamedValueSet,objWbemEventSource)	\
    ( (This)->lpVtbl -> ExecNotificationQuery(This,strQuery,strQueryLanguage,iFlags,objWbemNamedValueSet,objWbemEventSource) ) 

#define ISWbemServicesEx_ExecNotificationQueryAsync(This,objWbemSink,strQuery,strQueryLanguage,iFlags,objWbemNamedValueSet,objWbemAsyncContext)	\
    ( (This)->lpVtbl -> ExecNotificationQueryAsync(This,objWbemSink,strQuery,strQueryLanguage,iFlags,objWbemNamedValueSet,objWbemAsyncContext) ) 

#define ISWbemServicesEx_ExecMethod(This,strObjectPath,strMethodName,objWbemInParameters,iFlags,objWbemNamedValueSet,objWbemOutParameters)	\
    ( (This)->lpVtbl -> ExecMethod(This,strObjectPath,strMethodName,objWbemInParameters,iFlags,objWbemNamedValueSet,objWbemOutParameters) ) 

#define ISWbemServicesEx_ExecMethodAsync(This,objWbemSink,strObjectPath,strMethodName,objWbemInParameters,iFlags,objWbemNamedValueSet,objWbemAsyncContext)	\
    ( (This)->lpVtbl -> ExecMethodAsync(This,objWbemSink,strObjectPath,strMethodName,objWbemInParameters,iFlags,objWbemNamedValueSet,objWbemAsyncContext) ) 

#define ISWbemServicesEx_get_Security_(This,objWbemSecurity)	\
    ( (This)->lpVtbl -> get_Security_(This,objWbemSecurity) ) 


#define ISWbemServicesEx_Put(This,objWbemObject,iFlags,objWbemNamedValueSet,objWbemObjectPath)	\
    ( (This)->lpVtbl -> Put(This,objWbemObject,iFlags,objWbemNamedValueSet,objWbemObjectPath) ) 

#define ISWbemServicesEx_PutAsync(This,objWbemSink,objWbemObject,iFlags,objWbemNamedValueSet,objWbemAsyncContext)	\
    ( (This)->lpVtbl -> PutAsync(This,objWbemSink,objWbemObject,iFlags,objWbemNamedValueSet,objWbemAsyncContext) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISWbemServicesEx_INTERFACE_DEFINED__ */


#ifndef __ISWbemObjectEx_INTERFACE_DEFINED__
#define __ISWbemObjectEx_INTERFACE_DEFINED__

/* interface ISWbemObjectEx */
/* [helpstring][hidden][oleautomation][dual][uuid][object][local] */ 


EXTERN_C const IID IID_ISWbemObjectEx;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("269AD56A-8A67-4129-BC8C-0506DCFE9880")
    ISWbemObjectEx : public ISWbemObject
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Refresh_( 
            /* [defaultvalue][in] */ long iFlags = 0,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet = 0) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_SystemProperties_( 
            /* [retval][out] */ ISWbemPropertySet **objWbemPropertySet) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetText_( 
            /* [in] */ WbemObjectTextFormatEnum iObjectTextFormat,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [retval][out] */ BSTR *bsText) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetFromText_( 
            /* [in] */ BSTR bsText,
            /* [in] */ WbemObjectTextFormatEnum iObjectTextFormat,
            /* [defaultvalue][in] */ long iFlags = 0,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet = 0) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISWbemObjectExVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISWbemObjectEx * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISWbemObjectEx * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISWbemObjectEx * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            ISWbemObjectEx * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            ISWbemObjectEx * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            ISWbemObjectEx * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISWbemObjectEx * This,
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
        
        DECLSPEC_XFGVIRT(ISWbemObject, Put_)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Put_ )( 
            ISWbemObjectEx * This,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [retval][out] */ ISWbemObjectPath **objWbemObjectPath);
        
        DECLSPEC_XFGVIRT(ISWbemObject, PutAsync_)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *PutAsync_ )( 
            ISWbemObjectEx * This,
            /* [in] */ IDispatch *objWbemSink,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [defaultvalue][in] */ IDispatch *objWbemAsyncContext);
        
        DECLSPEC_XFGVIRT(ISWbemObject, Delete_)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Delete_ )( 
            ISWbemObjectEx * This,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet);
        
        DECLSPEC_XFGVIRT(ISWbemObject, DeleteAsync_)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *DeleteAsync_ )( 
            ISWbemObjectEx * This,
            /* [in] */ IDispatch *objWbemSink,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [defaultvalue][in] */ IDispatch *objWbemAsyncContext);
        
        DECLSPEC_XFGVIRT(ISWbemObject, Instances_)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Instances_ )( 
            ISWbemObjectEx * This,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [retval][out] */ ISWbemObjectSet **objWbemObjectSet);
        
        DECLSPEC_XFGVIRT(ISWbemObject, InstancesAsync_)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *InstancesAsync_ )( 
            ISWbemObjectEx * This,
            /* [in] */ IDispatch *objWbemSink,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [defaultvalue][in] */ IDispatch *objWbemAsyncContext);
        
        DECLSPEC_XFGVIRT(ISWbemObject, Subclasses_)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Subclasses_ )( 
            ISWbemObjectEx * This,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [retval][out] */ ISWbemObjectSet **objWbemObjectSet);
        
        DECLSPEC_XFGVIRT(ISWbemObject, SubclassesAsync_)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SubclassesAsync_ )( 
            ISWbemObjectEx * This,
            /* [in] */ IDispatch *objWbemSink,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [defaultvalue][in] */ IDispatch *objWbemAsyncContext);
        
        DECLSPEC_XFGVIRT(ISWbemObject, Associators_)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Associators_ )( 
            ISWbemObjectEx * This,
            /* [defaultvalue][in] */ BSTR strAssocClass,
            /* [defaultvalue][in] */ BSTR strResultClass,
            /* [defaultvalue][in] */ BSTR strResultRole,
            /* [defaultvalue][in] */ BSTR strRole,
            /* [defaultvalue][in] */ VARIANT_BOOL bClassesOnly,
            /* [defaultvalue][in] */ VARIANT_BOOL bSchemaOnly,
            /* [defaultvalue][in] */ BSTR strRequiredAssocQualifier,
            /* [defaultvalue][in] */ BSTR strRequiredQualifier,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [retval][out] */ ISWbemObjectSet **objWbemObjectSet);
        
        DECLSPEC_XFGVIRT(ISWbemObject, AssociatorsAsync_)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *AssociatorsAsync_ )( 
            ISWbemObjectEx * This,
            /* [in] */ IDispatch *objWbemSink,
            /* [defaultvalue][in] */ BSTR strAssocClass,
            /* [defaultvalue][in] */ BSTR strResultClass,
            /* [defaultvalue][in] */ BSTR strResultRole,
            /* [defaultvalue][in] */ BSTR strRole,
            /* [defaultvalue][in] */ VARIANT_BOOL bClassesOnly,
            /* [defaultvalue][in] */ VARIANT_BOOL bSchemaOnly,
            /* [defaultvalue][in] */ BSTR strRequiredAssocQualifier,
            /* [defaultvalue][in] */ BSTR strRequiredQualifier,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [defaultvalue][in] */ IDispatch *objWbemAsyncContext);
        
        DECLSPEC_XFGVIRT(ISWbemObject, References_)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *References_ )( 
            ISWbemObjectEx * This,
            /* [defaultvalue][in] */ BSTR strResultClass,
            /* [defaultvalue][in] */ BSTR strRole,
            /* [defaultvalue][in] */ VARIANT_BOOL bClassesOnly,
            /* [defaultvalue][in] */ VARIANT_BOOL bSchemaOnly,
            /* [defaultvalue][in] */ BSTR strRequiredQualifier,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [retval][out] */ ISWbemObjectSet **objWbemObjectSet);
        
        DECLSPEC_XFGVIRT(ISWbemObject, ReferencesAsync_)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ReferencesAsync_ )( 
            ISWbemObjectEx * This,
            /* [in] */ IDispatch *objWbemSink,
            /* [defaultvalue][in] */ BSTR strResultClass,
            /* [defaultvalue][in] */ BSTR strRole,
            /* [defaultvalue][in] */ VARIANT_BOOL bClassesOnly,
            /* [defaultvalue][in] */ VARIANT_BOOL bSchemaOnly,
            /* [defaultvalue][in] */ BSTR strRequiredQualifier,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [defaultvalue][in] */ IDispatch *objWbemAsyncContext);
        
        DECLSPEC_XFGVIRT(ISWbemObject, ExecMethod_)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ExecMethod_ )( 
            ISWbemObjectEx * This,
            /* [in] */ BSTR strMethodName,
            /* [defaultvalue][in] */ IDispatch *objWbemInParameters,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [retval][out] */ ISWbemObject **objWbemOutParameters);
        
        DECLSPEC_XFGVIRT(ISWbemObject, ExecMethodAsync_)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ExecMethodAsync_ )( 
            ISWbemObjectEx * This,
            /* [in] */ IDispatch *objWbemSink,
            /* [in] */ BSTR strMethodName,
            /* [defaultvalue][in] */ IDispatch *objWbemInParameters,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [defaultvalue][in] */ IDispatch *objWbemAsyncContext);
        
        DECLSPEC_XFGVIRT(ISWbemObject, Clone_)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Clone_ )( 
            ISWbemObjectEx * This,
            /* [retval][out] */ ISWbemObject **objWbemObject);
        
        DECLSPEC_XFGVIRT(ISWbemObject, GetObjectText_)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetObjectText_ )( 
            ISWbemObjectEx * This,
            /* [defaultvalue][in] */ long iFlags,
            /* [retval][out] */ BSTR *strObjectText);
        
        DECLSPEC_XFGVIRT(ISWbemObject, SpawnDerivedClass_)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SpawnDerivedClass_ )( 
            ISWbemObjectEx * This,
            /* [defaultvalue][in] */ long iFlags,
            /* [retval][out] */ ISWbemObject **objWbemObject);
        
        DECLSPEC_XFGVIRT(ISWbemObject, SpawnInstance_)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SpawnInstance_ )( 
            ISWbemObjectEx * This,
            /* [defaultvalue][in] */ long iFlags,
            /* [retval][out] */ ISWbemObject **objWbemObject);
        
        DECLSPEC_XFGVIRT(ISWbemObject, CompareTo_)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CompareTo_ )( 
            ISWbemObjectEx * This,
            /* [in] */ IDispatch *objWbemObject,
            /* [defaultvalue][in] */ long iFlags,
            /* [retval][out] */ VARIANT_BOOL *bResult);
        
        DECLSPEC_XFGVIRT(ISWbemObject, get_Qualifiers_)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Qualifiers_ )( 
            ISWbemObjectEx * This,
            /* [retval][out] */ ISWbemQualifierSet **objWbemQualifierSet);
        
        DECLSPEC_XFGVIRT(ISWbemObject, get_Properties_)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Properties_ )( 
            ISWbemObjectEx * This,
            /* [retval][out] */ ISWbemPropertySet **objWbemPropertySet);
        
        DECLSPEC_XFGVIRT(ISWbemObject, get_Methods_)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Methods_ )( 
            ISWbemObjectEx * This,
            /* [retval][out] */ ISWbemMethodSet **objWbemMethodSet);
        
        DECLSPEC_XFGVIRT(ISWbemObject, get_Derivation_)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Derivation_ )( 
            ISWbemObjectEx * This,
            /* [retval][out] */ VARIANT *strClassNameArray);
        
        DECLSPEC_XFGVIRT(ISWbemObject, get_Path_)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Path_ )( 
            ISWbemObjectEx * This,
            /* [retval][out] */ ISWbemObjectPath **objWbemObjectPath);
        
        DECLSPEC_XFGVIRT(ISWbemObject, get_Security_)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Security_ )( 
            ISWbemObjectEx * This,
            /* [retval][out] */ ISWbemSecurity **objWbemSecurity);
        
        DECLSPEC_XFGVIRT(ISWbemObjectEx, Refresh_)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Refresh_ )( 
            ISWbemObjectEx * This,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet);
        
        DECLSPEC_XFGVIRT(ISWbemObjectEx, get_SystemProperties_)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_SystemProperties_ )( 
            ISWbemObjectEx * This,
            /* [retval][out] */ ISWbemPropertySet **objWbemPropertySet);
        
        DECLSPEC_XFGVIRT(ISWbemObjectEx, GetText_)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetText_ )( 
            ISWbemObjectEx * This,
            /* [in] */ WbemObjectTextFormatEnum iObjectTextFormat,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [retval][out] */ BSTR *bsText);
        
        DECLSPEC_XFGVIRT(ISWbemObjectEx, SetFromText_)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetFromText_ )( 
            ISWbemObjectEx * This,
            /* [in] */ BSTR bsText,
            /* [in] */ WbemObjectTextFormatEnum iObjectTextFormat,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet);
        
        END_INTERFACE
    } ISWbemObjectExVtbl;

    interface ISWbemObjectEx
    {
        CONST_VTBL struct ISWbemObjectExVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISWbemObjectEx_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISWbemObjectEx_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISWbemObjectEx_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISWbemObjectEx_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISWbemObjectEx_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISWbemObjectEx_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISWbemObjectEx_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISWbemObjectEx_Put_(This,iFlags,objWbemNamedValueSet,objWbemObjectPath)	\
    ( (This)->lpVtbl -> Put_(This,iFlags,objWbemNamedValueSet,objWbemObjectPath) ) 

#define ISWbemObjectEx_PutAsync_(This,objWbemSink,iFlags,objWbemNamedValueSet,objWbemAsyncContext)	\
    ( (This)->lpVtbl -> PutAsync_(This,objWbemSink,iFlags,objWbemNamedValueSet,objWbemAsyncContext) ) 

#define ISWbemObjectEx_Delete_(This,iFlags,objWbemNamedValueSet)	\
    ( (This)->lpVtbl -> Delete_(This,iFlags,objWbemNamedValueSet) ) 

#define ISWbemObjectEx_DeleteAsync_(This,objWbemSink,iFlags,objWbemNamedValueSet,objWbemAsyncContext)	\
    ( (This)->lpVtbl -> DeleteAsync_(This,objWbemSink,iFlags,objWbemNamedValueSet,objWbemAsyncContext) ) 

#define ISWbemObjectEx_Instances_(This,iFlags,objWbemNamedValueSet,objWbemObjectSet)	\
    ( (This)->lpVtbl -> Instances_(This,iFlags,objWbemNamedValueSet,objWbemObjectSet) ) 

#define ISWbemObjectEx_InstancesAsync_(This,objWbemSink,iFlags,objWbemNamedValueSet,objWbemAsyncContext)	\
    ( (This)->lpVtbl -> InstancesAsync_(This,objWbemSink,iFlags,objWbemNamedValueSet,objWbemAsyncContext) ) 

#define ISWbemObjectEx_Subclasses_(This,iFlags,objWbemNamedValueSet,objWbemObjectSet)	\
    ( (This)->lpVtbl -> Subclasses_(This,iFlags,objWbemNamedValueSet,objWbemObjectSet) ) 

#define ISWbemObjectEx_SubclassesAsync_(This,objWbemSink,iFlags,objWbemNamedValueSet,objWbemAsyncContext)	\
    ( (This)->lpVtbl -> SubclassesAsync_(This,objWbemSink,iFlags,objWbemNamedValueSet,objWbemAsyncContext) ) 

#define ISWbemObjectEx_Associators_(This,strAssocClass,strResultClass,strResultRole,strRole,bClassesOnly,bSchemaOnly,strRequiredAssocQualifier,strRequiredQualifier,iFlags,objWbemNamedValueSet,objWbemObjectSet)	\
    ( (This)->lpVtbl -> Associators_(This,strAssocClass,strResultClass,strResultRole,strRole,bClassesOnly,bSchemaOnly,strRequiredAssocQualifier,strRequiredQualifier,iFlags,objWbemNamedValueSet,objWbemObjectSet) ) 

#define ISWbemObjectEx_AssociatorsAsync_(This,objWbemSink,strAssocClass,strResultClass,strResultRole,strRole,bClassesOnly,bSchemaOnly,strRequiredAssocQualifier,strRequiredQualifier,iFlags,objWbemNamedValueSet,objWbemAsyncContext)	\
    ( (This)->lpVtbl -> AssociatorsAsync_(This,objWbemSink,strAssocClass,strResultClass,strResultRole,strRole,bClassesOnly,bSchemaOnly,strRequiredAssocQualifier,strRequiredQualifier,iFlags,objWbemNamedValueSet,objWbemAsyncContext) ) 

#define ISWbemObjectEx_References_(This,strResultClass,strRole,bClassesOnly,bSchemaOnly,strRequiredQualifier,iFlags,objWbemNamedValueSet,objWbemObjectSet)	\
    ( (This)->lpVtbl -> References_(This,strResultClass,strRole,bClassesOnly,bSchemaOnly,strRequiredQualifier,iFlags,objWbemNamedValueSet,objWbemObjectSet) ) 

#define ISWbemObjectEx_ReferencesAsync_(This,objWbemSink,strResultClass,strRole,bClassesOnly,bSchemaOnly,strRequiredQualifier,iFlags,objWbemNamedValueSet,objWbemAsyncContext)	\
    ( (This)->lpVtbl -> ReferencesAsync_(This,objWbemSink,strResultClass,strRole,bClassesOnly,bSchemaOnly,strRequiredQualifier,iFlags,objWbemNamedValueSet,objWbemAsyncContext) ) 

#define ISWbemObjectEx_ExecMethod_(This,strMethodName,objWbemInParameters,iFlags,objWbemNamedValueSet,objWbemOutParameters)	\
    ( (This)->lpVtbl -> ExecMethod_(This,strMethodName,objWbemInParameters,iFlags,objWbemNamedValueSet,objWbemOutParameters) ) 

#define ISWbemObjectEx_ExecMethodAsync_(This,objWbemSink,strMethodName,objWbemInParameters,iFlags,objWbemNamedValueSet,objWbemAsyncContext)	\
    ( (This)->lpVtbl -> ExecMethodAsync_(This,objWbemSink,strMethodName,objWbemInParameters,iFlags,objWbemNamedValueSet,objWbemAsyncContext) ) 

#define ISWbemObjectEx_Clone_(This,objWbemObject)	\
    ( (This)->lpVtbl -> Clone_(This,objWbemObject) ) 

#define ISWbemObjectEx_GetObjectText_(This,iFlags,strObjectText)	\
    ( (This)->lpVtbl -> GetObjectText_(This,iFlags,strObjectText) ) 

#define ISWbemObjectEx_SpawnDerivedClass_(This,iFlags,objWbemObject)	\
    ( (This)->lpVtbl -> SpawnDerivedClass_(This,iFlags,objWbemObject) ) 

#define ISWbemObjectEx_SpawnInstance_(This,iFlags,objWbemObject)	\
    ( (This)->lpVtbl -> SpawnInstance_(This,iFlags,objWbemObject) ) 

#define ISWbemObjectEx_CompareTo_(This,objWbemObject,iFlags,bResult)	\
    ( (This)->lpVtbl -> CompareTo_(This,objWbemObject,iFlags,bResult) ) 

#define ISWbemObjectEx_get_Qualifiers_(This,objWbemQualifierSet)	\
    ( (This)->lpVtbl -> get_Qualifiers_(This,objWbemQualifierSet) ) 

#define ISWbemObjectEx_get_Properties_(This,objWbemPropertySet)	\
    ( (This)->lpVtbl -> get_Properties_(This,objWbemPropertySet) ) 

#define ISWbemObjectEx_get_Methods_(This,objWbemMethodSet)	\
    ( (This)->lpVtbl -> get_Methods_(This,objWbemMethodSet) ) 

#define ISWbemObjectEx_get_Derivation_(This,strClassNameArray)	\
    ( (This)->lpVtbl -> get_Derivation_(This,strClassNameArray) ) 

#define ISWbemObjectEx_get_Path_(This,objWbemObjectPath)	\
    ( (This)->lpVtbl -> get_Path_(This,objWbemObjectPath) ) 

#define ISWbemObjectEx_get_Security_(This,objWbemSecurity)	\
    ( (This)->lpVtbl -> get_Security_(This,objWbemSecurity) ) 


#define ISWbemObjectEx_Refresh_(This,iFlags,objWbemNamedValueSet)	\
    ( (This)->lpVtbl -> Refresh_(This,iFlags,objWbemNamedValueSet) ) 

#define ISWbemObjectEx_get_SystemProperties_(This,objWbemPropertySet)	\
    ( (This)->lpVtbl -> get_SystemProperties_(This,objWbemPropertySet) ) 

#define ISWbemObjectEx_GetText_(This,iObjectTextFormat,iFlags,objWbemNamedValueSet,bsText)	\
    ( (This)->lpVtbl -> GetText_(This,iObjectTextFormat,iFlags,objWbemNamedValueSet,bsText) ) 

#define ISWbemObjectEx_SetFromText_(This,bsText,iObjectTextFormat,iFlags,objWbemNamedValueSet)	\
    ( (This)->lpVtbl -> SetFromText_(This,bsText,iObjectTextFormat,iFlags,objWbemNamedValueSet) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISWbemObjectEx_INTERFACE_DEFINED__ */


#ifndef __ISWbemDateTime_INTERFACE_DEFINED__
#define __ISWbemDateTime_INTERFACE_DEFINED__

/* interface ISWbemDateTime */
/* [helpstring][nonextensible][dual][oleautomation][uuid][local][object] */ 


EXTERN_C const IID IID_ISWbemDateTime;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5E97458A-CF77-11d3-B38F-00105A1F473A")
    ISWbemDateTime : public IDispatch
    {
    public:
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Value( 
            /* [retval][out] */ BSTR *strValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE put_Value( 
            /* [in] */ BSTR strValue) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Year( 
            /* [retval][out] */ long *iYear) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE put_Year( 
            /* [in] */ long iYear) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_YearSpecified( 
            /* [retval][out] */ VARIANT_BOOL *bYearSpecified) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE put_YearSpecified( 
            /* [in] */ VARIANT_BOOL bYearSpecified) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Month( 
            /* [retval][out] */ long *iMonth) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE put_Month( 
            /* [in] */ long iMonth) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_MonthSpecified( 
            /* [retval][out] */ VARIANT_BOOL *bMonthSpecified) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE put_MonthSpecified( 
            /* [in] */ VARIANT_BOOL bMonthSpecified) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Day( 
            /* [retval][out] */ long *iDay) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE put_Day( 
            /* [in] */ long iDay) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_DaySpecified( 
            /* [retval][out] */ VARIANT_BOOL *bDaySpecified) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE put_DaySpecified( 
            /* [in] */ VARIANT_BOOL bDaySpecified) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Hours( 
            /* [retval][out] */ long *iHours) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE put_Hours( 
            /* [in] */ long iHours) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_HoursSpecified( 
            /* [retval][out] */ VARIANT_BOOL *bHoursSpecified) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE put_HoursSpecified( 
            /* [in] */ VARIANT_BOOL bHoursSpecified) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Minutes( 
            /* [retval][out] */ long *iMinutes) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE put_Minutes( 
            /* [in] */ long iMinutes) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_MinutesSpecified( 
            /* [retval][out] */ VARIANT_BOOL *bMinutesSpecified) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE put_MinutesSpecified( 
            /* [in] */ VARIANT_BOOL bMinutesSpecified) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Seconds( 
            /* [retval][out] */ long *iSeconds) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE put_Seconds( 
            /* [in] */ long iSeconds) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_SecondsSpecified( 
            /* [retval][out] */ VARIANT_BOOL *bSecondsSpecified) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE put_SecondsSpecified( 
            /* [in] */ VARIANT_BOOL bSecondsSpecified) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Microseconds( 
            /* [retval][out] */ long *iMicroseconds) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE put_Microseconds( 
            /* [in] */ long iMicroseconds) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_MicrosecondsSpecified( 
            /* [retval][out] */ VARIANT_BOOL *bMicrosecondsSpecified) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE put_MicrosecondsSpecified( 
            /* [in] */ VARIANT_BOOL bMicrosecondsSpecified) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_UTC( 
            /* [retval][out] */ long *iUTC) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE put_UTC( 
            /* [in] */ long iUTC) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_UTCSpecified( 
            /* [retval][out] */ VARIANT_BOOL *bUTCSpecified) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE put_UTCSpecified( 
            /* [in] */ VARIANT_BOOL bUTCSpecified) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_IsInterval( 
            /* [retval][out] */ VARIANT_BOOL *bIsInterval) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE put_IsInterval( 
            /* [in] */ VARIANT_BOOL bIsInterval) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetVarDate( 
            /* [defaultvalue][in] */ VARIANT_BOOL bIsLocal,
            /* [retval][out] */ DATE *dVarDate) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetVarDate( 
            /* [in] */ DATE dVarDate,
            /* [defaultvalue][in] */ VARIANT_BOOL bIsLocal = TRUE) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetFileTime( 
            /* [defaultvalue][in] */ VARIANT_BOOL bIsLocal,
            /* [retval][out] */ BSTR *strFileTime) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetFileTime( 
            /* [in] */ BSTR strFileTime,
            /* [defaultvalue][in] */ VARIANT_BOOL bIsLocal = TRUE) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISWbemDateTimeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISWbemDateTime * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISWbemDateTime * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISWbemDateTime * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            ISWbemDateTime * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            ISWbemDateTime * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            ISWbemDateTime * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISWbemDateTime * This,
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
        
        DECLSPEC_XFGVIRT(ISWbemDateTime, get_Value)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Value )( 
            ISWbemDateTime * This,
            /* [retval][out] */ BSTR *strValue);
        
        DECLSPEC_XFGVIRT(ISWbemDateTime, put_Value)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Value )( 
            ISWbemDateTime * This,
            /* [in] */ BSTR strValue);
        
        DECLSPEC_XFGVIRT(ISWbemDateTime, get_Year)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Year )( 
            ISWbemDateTime * This,
            /* [retval][out] */ long *iYear);
        
        DECLSPEC_XFGVIRT(ISWbemDateTime, put_Year)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Year )( 
            ISWbemDateTime * This,
            /* [in] */ long iYear);
        
        DECLSPEC_XFGVIRT(ISWbemDateTime, get_YearSpecified)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_YearSpecified )( 
            ISWbemDateTime * This,
            /* [retval][out] */ VARIANT_BOOL *bYearSpecified);
        
        DECLSPEC_XFGVIRT(ISWbemDateTime, put_YearSpecified)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_YearSpecified )( 
            ISWbemDateTime * This,
            /* [in] */ VARIANT_BOOL bYearSpecified);
        
        DECLSPEC_XFGVIRT(ISWbemDateTime, get_Month)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Month )( 
            ISWbemDateTime * This,
            /* [retval][out] */ long *iMonth);
        
        DECLSPEC_XFGVIRT(ISWbemDateTime, put_Month)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Month )( 
            ISWbemDateTime * This,
            /* [in] */ long iMonth);
        
        DECLSPEC_XFGVIRT(ISWbemDateTime, get_MonthSpecified)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_MonthSpecified )( 
            ISWbemDateTime * This,
            /* [retval][out] */ VARIANT_BOOL *bMonthSpecified);
        
        DECLSPEC_XFGVIRT(ISWbemDateTime, put_MonthSpecified)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_MonthSpecified )( 
            ISWbemDateTime * This,
            /* [in] */ VARIANT_BOOL bMonthSpecified);
        
        DECLSPEC_XFGVIRT(ISWbemDateTime, get_Day)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Day )( 
            ISWbemDateTime * This,
            /* [retval][out] */ long *iDay);
        
        DECLSPEC_XFGVIRT(ISWbemDateTime, put_Day)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Day )( 
            ISWbemDateTime * This,
            /* [in] */ long iDay);
        
        DECLSPEC_XFGVIRT(ISWbemDateTime, get_DaySpecified)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_DaySpecified )( 
            ISWbemDateTime * This,
            /* [retval][out] */ VARIANT_BOOL *bDaySpecified);
        
        DECLSPEC_XFGVIRT(ISWbemDateTime, put_DaySpecified)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_DaySpecified )( 
            ISWbemDateTime * This,
            /* [in] */ VARIANT_BOOL bDaySpecified);
        
        DECLSPEC_XFGVIRT(ISWbemDateTime, get_Hours)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Hours )( 
            ISWbemDateTime * This,
            /* [retval][out] */ long *iHours);
        
        DECLSPEC_XFGVIRT(ISWbemDateTime, put_Hours)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Hours )( 
            ISWbemDateTime * This,
            /* [in] */ long iHours);
        
        DECLSPEC_XFGVIRT(ISWbemDateTime, get_HoursSpecified)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_HoursSpecified )( 
            ISWbemDateTime * This,
            /* [retval][out] */ VARIANT_BOOL *bHoursSpecified);
        
        DECLSPEC_XFGVIRT(ISWbemDateTime, put_HoursSpecified)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_HoursSpecified )( 
            ISWbemDateTime * This,
            /* [in] */ VARIANT_BOOL bHoursSpecified);
        
        DECLSPEC_XFGVIRT(ISWbemDateTime, get_Minutes)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Minutes )( 
            ISWbemDateTime * This,
            /* [retval][out] */ long *iMinutes);
        
        DECLSPEC_XFGVIRT(ISWbemDateTime, put_Minutes)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Minutes )( 
            ISWbemDateTime * This,
            /* [in] */ long iMinutes);
        
        DECLSPEC_XFGVIRT(ISWbemDateTime, get_MinutesSpecified)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_MinutesSpecified )( 
            ISWbemDateTime * This,
            /* [retval][out] */ VARIANT_BOOL *bMinutesSpecified);
        
        DECLSPEC_XFGVIRT(ISWbemDateTime, put_MinutesSpecified)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_MinutesSpecified )( 
            ISWbemDateTime * This,
            /* [in] */ VARIANT_BOOL bMinutesSpecified);
        
        DECLSPEC_XFGVIRT(ISWbemDateTime, get_Seconds)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Seconds )( 
            ISWbemDateTime * This,
            /* [retval][out] */ long *iSeconds);
        
        DECLSPEC_XFGVIRT(ISWbemDateTime, put_Seconds)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Seconds )( 
            ISWbemDateTime * This,
            /* [in] */ long iSeconds);
        
        DECLSPEC_XFGVIRT(ISWbemDateTime, get_SecondsSpecified)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_SecondsSpecified )( 
            ISWbemDateTime * This,
            /* [retval][out] */ VARIANT_BOOL *bSecondsSpecified);
        
        DECLSPEC_XFGVIRT(ISWbemDateTime, put_SecondsSpecified)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_SecondsSpecified )( 
            ISWbemDateTime * This,
            /* [in] */ VARIANT_BOOL bSecondsSpecified);
        
        DECLSPEC_XFGVIRT(ISWbemDateTime, get_Microseconds)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Microseconds )( 
            ISWbemDateTime * This,
            /* [retval][out] */ long *iMicroseconds);
        
        DECLSPEC_XFGVIRT(ISWbemDateTime, put_Microseconds)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Microseconds )( 
            ISWbemDateTime * This,
            /* [in] */ long iMicroseconds);
        
        DECLSPEC_XFGVIRT(ISWbemDateTime, get_MicrosecondsSpecified)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_MicrosecondsSpecified )( 
            ISWbemDateTime * This,
            /* [retval][out] */ VARIANT_BOOL *bMicrosecondsSpecified);
        
        DECLSPEC_XFGVIRT(ISWbemDateTime, put_MicrosecondsSpecified)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_MicrosecondsSpecified )( 
            ISWbemDateTime * This,
            /* [in] */ VARIANT_BOOL bMicrosecondsSpecified);
        
        DECLSPEC_XFGVIRT(ISWbemDateTime, get_UTC)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_UTC )( 
            ISWbemDateTime * This,
            /* [retval][out] */ long *iUTC);
        
        DECLSPEC_XFGVIRT(ISWbemDateTime, put_UTC)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_UTC )( 
            ISWbemDateTime * This,
            /* [in] */ long iUTC);
        
        DECLSPEC_XFGVIRT(ISWbemDateTime, get_UTCSpecified)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_UTCSpecified )( 
            ISWbemDateTime * This,
            /* [retval][out] */ VARIANT_BOOL *bUTCSpecified);
        
        DECLSPEC_XFGVIRT(ISWbemDateTime, put_UTCSpecified)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_UTCSpecified )( 
            ISWbemDateTime * This,
            /* [in] */ VARIANT_BOOL bUTCSpecified);
        
        DECLSPEC_XFGVIRT(ISWbemDateTime, get_IsInterval)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_IsInterval )( 
            ISWbemDateTime * This,
            /* [retval][out] */ VARIANT_BOOL *bIsInterval);
        
        DECLSPEC_XFGVIRT(ISWbemDateTime, put_IsInterval)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_IsInterval )( 
            ISWbemDateTime * This,
            /* [in] */ VARIANT_BOOL bIsInterval);
        
        DECLSPEC_XFGVIRT(ISWbemDateTime, GetVarDate)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetVarDate )( 
            ISWbemDateTime * This,
            /* [defaultvalue][in] */ VARIANT_BOOL bIsLocal,
            /* [retval][out] */ DATE *dVarDate);
        
        DECLSPEC_XFGVIRT(ISWbemDateTime, SetVarDate)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetVarDate )( 
            ISWbemDateTime * This,
            /* [in] */ DATE dVarDate,
            /* [defaultvalue][in] */ VARIANT_BOOL bIsLocal);
        
        DECLSPEC_XFGVIRT(ISWbemDateTime, GetFileTime)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetFileTime )( 
            ISWbemDateTime * This,
            /* [defaultvalue][in] */ VARIANT_BOOL bIsLocal,
            /* [retval][out] */ BSTR *strFileTime);
        
        DECLSPEC_XFGVIRT(ISWbemDateTime, SetFileTime)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetFileTime )( 
            ISWbemDateTime * This,
            /* [in] */ BSTR strFileTime,
            /* [defaultvalue][in] */ VARIANT_BOOL bIsLocal);
        
        END_INTERFACE
    } ISWbemDateTimeVtbl;

    interface ISWbemDateTime
    {
        CONST_VTBL struct ISWbemDateTimeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISWbemDateTime_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISWbemDateTime_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISWbemDateTime_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISWbemDateTime_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISWbemDateTime_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISWbemDateTime_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISWbemDateTime_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISWbemDateTime_get_Value(This,strValue)	\
    ( (This)->lpVtbl -> get_Value(This,strValue) ) 

#define ISWbemDateTime_put_Value(This,strValue)	\
    ( (This)->lpVtbl -> put_Value(This,strValue) ) 

#define ISWbemDateTime_get_Year(This,iYear)	\
    ( (This)->lpVtbl -> get_Year(This,iYear) ) 

#define ISWbemDateTime_put_Year(This,iYear)	\
    ( (This)->lpVtbl -> put_Year(This,iYear) ) 

#define ISWbemDateTime_get_YearSpecified(This,bYearSpecified)	\
    ( (This)->lpVtbl -> get_YearSpecified(This,bYearSpecified) ) 

#define ISWbemDateTime_put_YearSpecified(This,bYearSpecified)	\
    ( (This)->lpVtbl -> put_YearSpecified(This,bYearSpecified) ) 

#define ISWbemDateTime_get_Month(This,iMonth)	\
    ( (This)->lpVtbl -> get_Month(This,iMonth) ) 

#define ISWbemDateTime_put_Month(This,iMonth)	\
    ( (This)->lpVtbl -> put_Month(This,iMonth) ) 

#define ISWbemDateTime_get_MonthSpecified(This,bMonthSpecified)	\
    ( (This)->lpVtbl -> get_MonthSpecified(This,bMonthSpecified) ) 

#define ISWbemDateTime_put_MonthSpecified(This,bMonthSpecified)	\
    ( (This)->lpVtbl -> put_MonthSpecified(This,bMonthSpecified) ) 

#define ISWbemDateTime_get_Day(This,iDay)	\
    ( (This)->lpVtbl -> get_Day(This,iDay) ) 

#define ISWbemDateTime_put_Day(This,iDay)	\
    ( (This)->lpVtbl -> put_Day(This,iDay) ) 

#define ISWbemDateTime_get_DaySpecified(This,bDaySpecified)	\
    ( (This)->lpVtbl -> get_DaySpecified(This,bDaySpecified) ) 

#define ISWbemDateTime_put_DaySpecified(This,bDaySpecified)	\
    ( (This)->lpVtbl -> put_DaySpecified(This,bDaySpecified) ) 

#define ISWbemDateTime_get_Hours(This,iHours)	\
    ( (This)->lpVtbl -> get_Hours(This,iHours) ) 

#define ISWbemDateTime_put_Hours(This,iHours)	\
    ( (This)->lpVtbl -> put_Hours(This,iHours) ) 

#define ISWbemDateTime_get_HoursSpecified(This,bHoursSpecified)	\
    ( (This)->lpVtbl -> get_HoursSpecified(This,bHoursSpecified) ) 

#define ISWbemDateTime_put_HoursSpecified(This,bHoursSpecified)	\
    ( (This)->lpVtbl -> put_HoursSpecified(This,bHoursSpecified) ) 

#define ISWbemDateTime_get_Minutes(This,iMinutes)	\
    ( (This)->lpVtbl -> get_Minutes(This,iMinutes) ) 

#define ISWbemDateTime_put_Minutes(This,iMinutes)	\
    ( (This)->lpVtbl -> put_Minutes(This,iMinutes) ) 

#define ISWbemDateTime_get_MinutesSpecified(This,bMinutesSpecified)	\
    ( (This)->lpVtbl -> get_MinutesSpecified(This,bMinutesSpecified) ) 

#define ISWbemDateTime_put_MinutesSpecified(This,bMinutesSpecified)	\
    ( (This)->lpVtbl -> put_MinutesSpecified(This,bMinutesSpecified) ) 

#define ISWbemDateTime_get_Seconds(This,iSeconds)	\
    ( (This)->lpVtbl -> get_Seconds(This,iSeconds) ) 

#define ISWbemDateTime_put_Seconds(This,iSeconds)	\
    ( (This)->lpVtbl -> put_Seconds(This,iSeconds) ) 

#define ISWbemDateTime_get_SecondsSpecified(This,bSecondsSpecified)	\
    ( (This)->lpVtbl -> get_SecondsSpecified(This,bSecondsSpecified) ) 

#define ISWbemDateTime_put_SecondsSpecified(This,bSecondsSpecified)	\
    ( (This)->lpVtbl -> put_SecondsSpecified(This,bSecondsSpecified) ) 

#define ISWbemDateTime_get_Microseconds(This,iMicroseconds)	\
    ( (This)->lpVtbl -> get_Microseconds(This,iMicroseconds) ) 

#define ISWbemDateTime_put_Microseconds(This,iMicroseconds)	\
    ( (This)->lpVtbl -> put_Microseconds(This,iMicroseconds) ) 

#define ISWbemDateTime_get_MicrosecondsSpecified(This,bMicrosecondsSpecified)	\
    ( (This)->lpVtbl -> get_MicrosecondsSpecified(This,bMicrosecondsSpecified) ) 

#define ISWbemDateTime_put_MicrosecondsSpecified(This,bMicrosecondsSpecified)	\
    ( (This)->lpVtbl -> put_MicrosecondsSpecified(This,bMicrosecondsSpecified) ) 

#define ISWbemDateTime_get_UTC(This,iUTC)	\
    ( (This)->lpVtbl -> get_UTC(This,iUTC) ) 

#define ISWbemDateTime_put_UTC(This,iUTC)	\
    ( (This)->lpVtbl -> put_UTC(This,iUTC) ) 

#define ISWbemDateTime_get_UTCSpecified(This,bUTCSpecified)	\
    ( (This)->lpVtbl -> get_UTCSpecified(This,bUTCSpecified) ) 

#define ISWbemDateTime_put_UTCSpecified(This,bUTCSpecified)	\
    ( (This)->lpVtbl -> put_UTCSpecified(This,bUTCSpecified) ) 

#define ISWbemDateTime_get_IsInterval(This,bIsInterval)	\
    ( (This)->lpVtbl -> get_IsInterval(This,bIsInterval) ) 

#define ISWbemDateTime_put_IsInterval(This,bIsInterval)	\
    ( (This)->lpVtbl -> put_IsInterval(This,bIsInterval) ) 

#define ISWbemDateTime_GetVarDate(This,bIsLocal,dVarDate)	\
    ( (This)->lpVtbl -> GetVarDate(This,bIsLocal,dVarDate) ) 

#define ISWbemDateTime_SetVarDate(This,dVarDate,bIsLocal)	\
    ( (This)->lpVtbl -> SetVarDate(This,dVarDate,bIsLocal) ) 

#define ISWbemDateTime_GetFileTime(This,bIsLocal,strFileTime)	\
    ( (This)->lpVtbl -> GetFileTime(This,bIsLocal,strFileTime) ) 

#define ISWbemDateTime_SetFileTime(This,strFileTime,bIsLocal)	\
    ( (This)->lpVtbl -> SetFileTime(This,strFileTime,bIsLocal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISWbemDateTime_INTERFACE_DEFINED__ */


#ifndef __ISWbemRefresher_INTERFACE_DEFINED__
#define __ISWbemRefresher_INTERFACE_DEFINED__

/* interface ISWbemRefresher */
/* [helpstring][nonextensible][dual][oleautomation][uuid][local][object] */ 


EXTERN_C const IID IID_ISWbemRefresher;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("14D8250E-D9C2-11d3-B38F-00105A1F473A")
    ISWbemRefresher : public IDispatch
    {
    public:
        virtual /* [restricted][propget][id] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ IUnknown **pUnk) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Item( 
            /* [in] */ long iIndex,
            /* [retval][out] */ ISWbemRefreshableItem **objWbemRefreshableItem) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ long *iCount) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Add( 
            /* [in] */ ISWbemServicesEx *objWbemServices,
            /* [in] */ BSTR bsInstancePath,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [retval][out] */ ISWbemRefreshableItem **objWbemRefreshableItem) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE AddEnum( 
            /* [in] */ ISWbemServicesEx *objWbemServices,
            /* [in] */ BSTR bsClassName,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [retval][out] */ ISWbemRefreshableItem **objWbemRefreshableItem) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Remove( 
            /* [in] */ long iIndex,
            /* [defaultvalue][in] */ long iFlags = 0) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Refresh( 
            /* [defaultvalue][in] */ long iFlags = 0) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_AutoReconnect( 
            /* [retval][out] */ VARIANT_BOOL *bCount) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE put_AutoReconnect( 
            /* [in] */ VARIANT_BOOL bCount) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE DeleteAll( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISWbemRefresherVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISWbemRefresher * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISWbemRefresher * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISWbemRefresher * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            ISWbemRefresher * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            ISWbemRefresher * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            ISWbemRefresher * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISWbemRefresher * This,
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
        
        DECLSPEC_XFGVIRT(ISWbemRefresher, get__NewEnum)
        /* [restricted][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            ISWbemRefresher * This,
            /* [retval][out] */ IUnknown **pUnk);
        
        DECLSPEC_XFGVIRT(ISWbemRefresher, Item)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Item )( 
            ISWbemRefresher * This,
            /* [in] */ long iIndex,
            /* [retval][out] */ ISWbemRefreshableItem **objWbemRefreshableItem);
        
        DECLSPEC_XFGVIRT(ISWbemRefresher, get_Count)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            ISWbemRefresher * This,
            /* [retval][out] */ long *iCount);
        
        DECLSPEC_XFGVIRT(ISWbemRefresher, Add)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Add )( 
            ISWbemRefresher * This,
            /* [in] */ ISWbemServicesEx *objWbemServices,
            /* [in] */ BSTR bsInstancePath,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [retval][out] */ ISWbemRefreshableItem **objWbemRefreshableItem);
        
        DECLSPEC_XFGVIRT(ISWbemRefresher, AddEnum)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *AddEnum )( 
            ISWbemRefresher * This,
            /* [in] */ ISWbemServicesEx *objWbemServices,
            /* [in] */ BSTR bsClassName,
            /* [defaultvalue][in] */ long iFlags,
            /* [defaultvalue][in] */ IDispatch *objWbemNamedValueSet,
            /* [retval][out] */ ISWbemRefreshableItem **objWbemRefreshableItem);
        
        DECLSPEC_XFGVIRT(ISWbemRefresher, Remove)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Remove )( 
            ISWbemRefresher * This,
            /* [in] */ long iIndex,
            /* [defaultvalue][in] */ long iFlags);
        
        DECLSPEC_XFGVIRT(ISWbemRefresher, Refresh)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            ISWbemRefresher * This,
            /* [defaultvalue][in] */ long iFlags);
        
        DECLSPEC_XFGVIRT(ISWbemRefresher, get_AutoReconnect)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_AutoReconnect )( 
            ISWbemRefresher * This,
            /* [retval][out] */ VARIANT_BOOL *bCount);
        
        DECLSPEC_XFGVIRT(ISWbemRefresher, put_AutoReconnect)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_AutoReconnect )( 
            ISWbemRefresher * This,
            /* [in] */ VARIANT_BOOL bCount);
        
        DECLSPEC_XFGVIRT(ISWbemRefresher, DeleteAll)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *DeleteAll )( 
            ISWbemRefresher * This);
        
        END_INTERFACE
    } ISWbemRefresherVtbl;

    interface ISWbemRefresher
    {
        CONST_VTBL struct ISWbemRefresherVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISWbemRefresher_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISWbemRefresher_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISWbemRefresher_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISWbemRefresher_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISWbemRefresher_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISWbemRefresher_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISWbemRefresher_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISWbemRefresher_get__NewEnum(This,pUnk)	\
    ( (This)->lpVtbl -> get__NewEnum(This,pUnk) ) 

#define ISWbemRefresher_Item(This,iIndex,objWbemRefreshableItem)	\
    ( (This)->lpVtbl -> Item(This,iIndex,objWbemRefreshableItem) ) 

#define ISWbemRefresher_get_Count(This,iCount)	\
    ( (This)->lpVtbl -> get_Count(This,iCount) ) 

#define ISWbemRefresher_Add(This,objWbemServices,bsInstancePath,iFlags,objWbemNamedValueSet,objWbemRefreshableItem)	\
    ( (This)->lpVtbl -> Add(This,objWbemServices,bsInstancePath,iFlags,objWbemNamedValueSet,objWbemRefreshableItem) ) 

#define ISWbemRefresher_AddEnum(This,objWbemServices,bsClassName,iFlags,objWbemNamedValueSet,objWbemRefreshableItem)	\
    ( (This)->lpVtbl -> AddEnum(This,objWbemServices,bsClassName,iFlags,objWbemNamedValueSet,objWbemRefreshableItem) ) 

#define ISWbemRefresher_Remove(This,iIndex,iFlags)	\
    ( (This)->lpVtbl -> Remove(This,iIndex,iFlags) ) 

#define ISWbemRefresher_Refresh(This,iFlags)	\
    ( (This)->lpVtbl -> Refresh(This,iFlags) ) 

#define ISWbemRefresher_get_AutoReconnect(This,bCount)	\
    ( (This)->lpVtbl -> get_AutoReconnect(This,bCount) ) 

#define ISWbemRefresher_put_AutoReconnect(This,bCount)	\
    ( (This)->lpVtbl -> put_AutoReconnect(This,bCount) ) 

#define ISWbemRefresher_DeleteAll(This)	\
    ( (This)->lpVtbl -> DeleteAll(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISWbemRefresher_INTERFACE_DEFINED__ */


#ifndef __ISWbemRefreshableItem_INTERFACE_DEFINED__
#define __ISWbemRefreshableItem_INTERFACE_DEFINED__

/* interface ISWbemRefreshableItem */
/* [helpstring][nonextensible][dual][oleautomation][uuid][local][object] */ 


EXTERN_C const IID IID_ISWbemRefreshableItem;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5AD4BF92-DAAB-11d3-B38F-00105A1F473A")
    ISWbemRefreshableItem : public IDispatch
    {
    public:
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Index( 
            /* [retval][out] */ long *iIndex) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Refresher( 
            /* [retval][out] */ ISWbemRefresher **objWbemRefresher) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_IsSet( 
            /* [retval][out] */ VARIANT_BOOL *bIsSet) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Object( 
            /* [retval][out] */ ISWbemObjectEx **objWbemObject) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_ObjectSet( 
            /* [retval][out] */ ISWbemObjectSet **objWbemObjectSet) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Remove( 
            /* [defaultvalue][in] */ long iFlags = 0) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISWbemRefreshableItemVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISWbemRefreshableItem * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISWbemRefreshableItem * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISWbemRefreshableItem * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            ISWbemRefreshableItem * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            ISWbemRefreshableItem * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            ISWbemRefreshableItem * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISWbemRefreshableItem * This,
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
        
        DECLSPEC_XFGVIRT(ISWbemRefreshableItem, get_Index)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Index )( 
            ISWbemRefreshableItem * This,
            /* [retval][out] */ long *iIndex);
        
        DECLSPEC_XFGVIRT(ISWbemRefreshableItem, get_Refresher)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Refresher )( 
            ISWbemRefreshableItem * This,
            /* [retval][out] */ ISWbemRefresher **objWbemRefresher);
        
        DECLSPEC_XFGVIRT(ISWbemRefreshableItem, get_IsSet)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_IsSet )( 
            ISWbemRefreshableItem * This,
            /* [retval][out] */ VARIANT_BOOL *bIsSet);
        
        DECLSPEC_XFGVIRT(ISWbemRefreshableItem, get_Object)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Object )( 
            ISWbemRefreshableItem * This,
            /* [retval][out] */ ISWbemObjectEx **objWbemObject);
        
        DECLSPEC_XFGVIRT(ISWbemRefreshableItem, get_ObjectSet)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ObjectSet )( 
            ISWbemRefreshableItem * This,
            /* [retval][out] */ ISWbemObjectSet **objWbemObjectSet);
        
        DECLSPEC_XFGVIRT(ISWbemRefreshableItem, Remove)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Remove )( 
            ISWbemRefreshableItem * This,
            /* [defaultvalue][in] */ long iFlags);
        
        END_INTERFACE
    } ISWbemRefreshableItemVtbl;

    interface ISWbemRefreshableItem
    {
        CONST_VTBL struct ISWbemRefreshableItemVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISWbemRefreshableItem_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISWbemRefreshableItem_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISWbemRefreshableItem_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISWbemRefreshableItem_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISWbemRefreshableItem_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISWbemRefreshableItem_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISWbemRefreshableItem_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISWbemRefreshableItem_get_Index(This,iIndex)	\
    ( (This)->lpVtbl -> get_Index(This,iIndex) ) 

#define ISWbemRefreshableItem_get_Refresher(This,objWbemRefresher)	\
    ( (This)->lpVtbl -> get_Refresher(This,objWbemRefresher) ) 

#define ISWbemRefreshableItem_get_IsSet(This,bIsSet)	\
    ( (This)->lpVtbl -> get_IsSet(This,bIsSet) ) 

#define ISWbemRefreshableItem_get_Object(This,objWbemObject)	\
    ( (This)->lpVtbl -> get_Object(This,objWbemObject) ) 

#define ISWbemRefreshableItem_get_ObjectSet(This,objWbemObjectSet)	\
    ( (This)->lpVtbl -> get_ObjectSet(This,objWbemObjectSet) ) 

#define ISWbemRefreshableItem_Remove(This,iFlags)	\
    ( (This)->lpVtbl -> Remove(This,iFlags) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISWbemRefreshableItem_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_SWbemLocator;

#ifdef __cplusplus

class DECLSPEC_UUID("76A64158-CB41-11d1-8B02-00600806D9B6")
SWbemLocator;
#endif

EXTERN_C const CLSID CLSID_SWbemNamedValueSet;

#ifdef __cplusplus

class DECLSPEC_UUID("9AED384E-CE8B-11d1-8B05-00600806D9B6")
SWbemNamedValueSet;
#endif

EXTERN_C const CLSID CLSID_SWbemObjectPath;

#ifdef __cplusplus

class DECLSPEC_UUID("5791BC26-CE9C-11d1-97BF-0000F81E849C")
SWbemObjectPath;
#endif

EXTERN_C const CLSID CLSID_SWbemLastError;

#ifdef __cplusplus

class DECLSPEC_UUID("C2FEEEAC-CFCD-11d1-8B05-00600806D9B6")
SWbemLastError;
#endif

EXTERN_C const CLSID CLSID_SWbemSink;

#ifdef __cplusplus

class DECLSPEC_UUID("75718C9A-F029-11d1-A1AC-00C04FB6C223")
SWbemSink;
#endif

EXTERN_C const CLSID CLSID_SWbemDateTime;

#ifdef __cplusplus

class DECLSPEC_UUID("47DFBE54-CF76-11d3-B38F-00105A1F473A")
SWbemDateTime;
#endif

EXTERN_C const CLSID CLSID_SWbemRefresher;

#ifdef __cplusplus

class DECLSPEC_UUID("D269BF5C-D9C1-11d3-B38F-00105A1F473A")
SWbemRefresher;
#endif

EXTERN_C const CLSID CLSID_SWbemServices;

#ifdef __cplusplus

class DECLSPEC_UUID("04B83D63-21AE-11d2-8B33-00600806D9B6")
SWbemServices;
#endif

EXTERN_C const CLSID CLSID_SWbemServicesEx;

#ifdef __cplusplus

class DECLSPEC_UUID("62E522DC-8CF3-40a8-8B2E-37D595651E40")
SWbemServicesEx;
#endif

EXTERN_C const CLSID CLSID_SWbemObject;

#ifdef __cplusplus

class DECLSPEC_UUID("04B83D62-21AE-11d2-8B33-00600806D9B6")
SWbemObject;
#endif

EXTERN_C const CLSID CLSID_SWbemObjectEx;

#ifdef __cplusplus

class DECLSPEC_UUID("D6BDAFB2-9435-491f-BB87-6AA0F0BC31A2")
SWbemObjectEx;
#endif

EXTERN_C const CLSID CLSID_SWbemObjectSet;

#ifdef __cplusplus

class DECLSPEC_UUID("04B83D61-21AE-11d2-8B33-00600806D9B6")
SWbemObjectSet;
#endif

EXTERN_C const CLSID CLSID_SWbemNamedValue;

#ifdef __cplusplus

class DECLSPEC_UUID("04B83D60-21AE-11d2-8B33-00600806D9B6")
SWbemNamedValue;
#endif

EXTERN_C const CLSID CLSID_SWbemQualifier;

#ifdef __cplusplus

class DECLSPEC_UUID("04B83D5F-21AE-11d2-8B33-00600806D9B6")
SWbemQualifier;
#endif

EXTERN_C const CLSID CLSID_SWbemQualifierSet;

#ifdef __cplusplus

class DECLSPEC_UUID("04B83D5E-21AE-11d2-8B33-00600806D9B6")
SWbemQualifierSet;
#endif

EXTERN_C const CLSID CLSID_SWbemProperty;

#ifdef __cplusplus

class DECLSPEC_UUID("04B83D5D-21AE-11d2-8B33-00600806D9B6")
SWbemProperty;
#endif

EXTERN_C const CLSID CLSID_SWbemPropertySet;

#ifdef __cplusplus

class DECLSPEC_UUID("04B83D5C-21AE-11d2-8B33-00600806D9B6")
SWbemPropertySet;
#endif

EXTERN_C const CLSID CLSID_SWbemMethod;

#ifdef __cplusplus

class DECLSPEC_UUID("04B83D5B-21AE-11d2-8B33-00600806D9B6")
SWbemMethod;
#endif

EXTERN_C const CLSID CLSID_SWbemMethodSet;

#ifdef __cplusplus

class DECLSPEC_UUID("04B83D5A-21AE-11d2-8B33-00600806D9B6")
SWbemMethodSet;
#endif

EXTERN_C const CLSID CLSID_SWbemEventSource;

#ifdef __cplusplus

class DECLSPEC_UUID("04B83D58-21AE-11d2-8B33-00600806D9B6")
SWbemEventSource;
#endif

EXTERN_C const CLSID CLSID_SWbemSecurity;

#ifdef __cplusplus

class DECLSPEC_UUID("B54D66E9-2287-11d2-8B33-00600806D9B6")
SWbemSecurity;
#endif

EXTERN_C const CLSID CLSID_SWbemPrivilege;

#ifdef __cplusplus

class DECLSPEC_UUID("26EE67BC-5804-11d2-8B4A-00600806D9B6")
SWbemPrivilege;
#endif

EXTERN_C const CLSID CLSID_SWbemPrivilegeSet;

#ifdef __cplusplus

class DECLSPEC_UUID("26EE67BE-5804-11d2-8B4A-00600806D9B6")
SWbemPrivilegeSet;
#endif

EXTERN_C const CLSID CLSID_SWbemRefreshableItem;

#ifdef __cplusplus

class DECLSPEC_UUID("8C6854BC-DE4B-11d3-B390-00105A1F473A")
SWbemRefreshableItem;
#endif
#endif /* __WbemScripting_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_wbemdisp_0000_0018 */
/* [local] */ 

#define	WBEMS_DISPID_OBJECT_READY	( 1 )

#define	WBEMS_DISPID_COMPLETED	( 2 )

#define	WBEMS_DISPID_PROGRESS	( 3 )

#define	WBEMS_DISPID_OBJECT_PUT	( 4 )

#define	WBEMS_DISPID_CONNECTION_READY	( 5 )



extern RPC_IF_HANDLE __MIDL_itf_wbemdisp_0000_0018_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wbemdisp_0000_0018_v0_0_s_ifspec;

/* interface __MIDL_itf_wbemdisp_0000_0026 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_WINMGMT) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_wbemdisp_0000_0026_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wbemdisp_0000_0026_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


