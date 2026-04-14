

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

#ifndef __wbemcli_h__
#define __wbemcli_h__

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

#ifndef __IWbemClassObject_FWD_DEFINED__
#define __IWbemClassObject_FWD_DEFINED__
typedef interface IWbemClassObject IWbemClassObject;

#endif 	/* __IWbemClassObject_FWD_DEFINED__ */


#ifndef __IWbemObjectAccess_FWD_DEFINED__
#define __IWbemObjectAccess_FWD_DEFINED__
typedef interface IWbemObjectAccess IWbemObjectAccess;

#endif 	/* __IWbemObjectAccess_FWD_DEFINED__ */


#ifndef __IWbemQualifierSet_FWD_DEFINED__
#define __IWbemQualifierSet_FWD_DEFINED__
typedef interface IWbemQualifierSet IWbemQualifierSet;

#endif 	/* __IWbemQualifierSet_FWD_DEFINED__ */


#ifndef __IWbemServices_FWD_DEFINED__
#define __IWbemServices_FWD_DEFINED__
typedef interface IWbemServices IWbemServices;

#endif 	/* __IWbemServices_FWD_DEFINED__ */


#ifndef __IWbemLocator_FWD_DEFINED__
#define __IWbemLocator_FWD_DEFINED__
typedef interface IWbemLocator IWbemLocator;

#endif 	/* __IWbemLocator_FWD_DEFINED__ */


#ifndef __IWbemObjectSink_FWD_DEFINED__
#define __IWbemObjectSink_FWD_DEFINED__
typedef interface IWbemObjectSink IWbemObjectSink;

#endif 	/* __IWbemObjectSink_FWD_DEFINED__ */


#ifndef __IEnumWbemClassObject_FWD_DEFINED__
#define __IEnumWbemClassObject_FWD_DEFINED__
typedef interface IEnumWbemClassObject IEnumWbemClassObject;

#endif 	/* __IEnumWbemClassObject_FWD_DEFINED__ */


#ifndef __IWbemCallResult_FWD_DEFINED__
#define __IWbemCallResult_FWD_DEFINED__
typedef interface IWbemCallResult IWbemCallResult;

#endif 	/* __IWbemCallResult_FWD_DEFINED__ */


#ifndef __IWbemContext_FWD_DEFINED__
#define __IWbemContext_FWD_DEFINED__
typedef interface IWbemContext IWbemContext;

#endif 	/* __IWbemContext_FWD_DEFINED__ */


#ifndef __IUnsecuredApartment_FWD_DEFINED__
#define __IUnsecuredApartment_FWD_DEFINED__
typedef interface IUnsecuredApartment IUnsecuredApartment;

#endif 	/* __IUnsecuredApartment_FWD_DEFINED__ */


#ifndef __IWbemUnsecuredApartment_FWD_DEFINED__
#define __IWbemUnsecuredApartment_FWD_DEFINED__
typedef interface IWbemUnsecuredApartment IWbemUnsecuredApartment;

#endif 	/* __IWbemUnsecuredApartment_FWD_DEFINED__ */


#ifndef __IWbemStatusCodeText_FWD_DEFINED__
#define __IWbemStatusCodeText_FWD_DEFINED__
typedef interface IWbemStatusCodeText IWbemStatusCodeText;

#endif 	/* __IWbemStatusCodeText_FWD_DEFINED__ */


#ifndef __IWbemBackupRestore_FWD_DEFINED__
#define __IWbemBackupRestore_FWD_DEFINED__
typedef interface IWbemBackupRestore IWbemBackupRestore;

#endif 	/* __IWbemBackupRestore_FWD_DEFINED__ */


#ifndef __IWbemBackupRestoreEx_FWD_DEFINED__
#define __IWbemBackupRestoreEx_FWD_DEFINED__
typedef interface IWbemBackupRestoreEx IWbemBackupRestoreEx;

#endif 	/* __IWbemBackupRestoreEx_FWD_DEFINED__ */


#ifndef __IWbemRefresher_FWD_DEFINED__
#define __IWbemRefresher_FWD_DEFINED__
typedef interface IWbemRefresher IWbemRefresher;

#endif 	/* __IWbemRefresher_FWD_DEFINED__ */


#ifndef __IWbemHiPerfEnum_FWD_DEFINED__
#define __IWbemHiPerfEnum_FWD_DEFINED__
typedef interface IWbemHiPerfEnum IWbemHiPerfEnum;

#endif 	/* __IWbemHiPerfEnum_FWD_DEFINED__ */


#ifndef __IWbemConfigureRefresher_FWD_DEFINED__
#define __IWbemConfigureRefresher_FWD_DEFINED__
typedef interface IWbemConfigureRefresher IWbemConfigureRefresher;

#endif 	/* __IWbemConfigureRefresher_FWD_DEFINED__ */


#ifndef __WbemLocator_FWD_DEFINED__
#define __WbemLocator_FWD_DEFINED__

#ifdef __cplusplus
typedef class WbemLocator WbemLocator;
#else
typedef struct WbemLocator WbemLocator;
#endif /* __cplusplus */

#endif 	/* __WbemLocator_FWD_DEFINED__ */


#ifndef __WbemContext_FWD_DEFINED__
#define __WbemContext_FWD_DEFINED__

#ifdef __cplusplus
typedef class WbemContext WbemContext;
#else
typedef struct WbemContext WbemContext;
#endif /* __cplusplus */

#endif 	/* __WbemContext_FWD_DEFINED__ */


#ifndef __UnsecuredApartment_FWD_DEFINED__
#define __UnsecuredApartment_FWD_DEFINED__

#ifdef __cplusplus
typedef class UnsecuredApartment UnsecuredApartment;
#else
typedef struct UnsecuredApartment UnsecuredApartment;
#endif /* __cplusplus */

#endif 	/* __UnsecuredApartment_FWD_DEFINED__ */


#ifndef __WbemClassObject_FWD_DEFINED__
#define __WbemClassObject_FWD_DEFINED__

#ifdef __cplusplus
typedef class WbemClassObject WbemClassObject;
#else
typedef struct WbemClassObject WbemClassObject;
#endif /* __cplusplus */

#endif 	/* __WbemClassObject_FWD_DEFINED__ */


#ifndef __MofCompiler_FWD_DEFINED__
#define __MofCompiler_FWD_DEFINED__

#ifdef __cplusplus
typedef class MofCompiler MofCompiler;
#else
typedef struct MofCompiler MofCompiler;
#endif /* __cplusplus */

#endif 	/* __MofCompiler_FWD_DEFINED__ */


#ifndef __WbemStatusCodeText_FWD_DEFINED__
#define __WbemStatusCodeText_FWD_DEFINED__

#ifdef __cplusplus
typedef class WbemStatusCodeText WbemStatusCodeText;
#else
typedef struct WbemStatusCodeText WbemStatusCodeText;
#endif /* __cplusplus */

#endif 	/* __WbemStatusCodeText_FWD_DEFINED__ */


#ifndef __WbemBackupRestore_FWD_DEFINED__
#define __WbemBackupRestore_FWD_DEFINED__

#ifdef __cplusplus
typedef class WbemBackupRestore WbemBackupRestore;
#else
typedef struct WbemBackupRestore WbemBackupRestore;
#endif /* __cplusplus */

#endif 	/* __WbemBackupRestore_FWD_DEFINED__ */


#ifndef __WbemRefresher_FWD_DEFINED__
#define __WbemRefresher_FWD_DEFINED__

#ifdef __cplusplus
typedef class WbemRefresher WbemRefresher;
#else
typedef struct WbemRefresher WbemRefresher;
#endif /* __cplusplus */

#endif 	/* __WbemRefresher_FWD_DEFINED__ */


#ifndef __WbemObjectTextSrc_FWD_DEFINED__
#define __WbemObjectTextSrc_FWD_DEFINED__

#ifdef __cplusplus
typedef class WbemObjectTextSrc WbemObjectTextSrc;
#else
typedef struct WbemObjectTextSrc WbemObjectTextSrc;
#endif /* __cplusplus */

#endif 	/* __WbemObjectTextSrc_FWD_DEFINED__ */


#ifndef __IWbemClassObject_FWD_DEFINED__
#define __IWbemClassObject_FWD_DEFINED__
typedef interface IWbemClassObject IWbemClassObject;

#endif 	/* __IWbemClassObject_FWD_DEFINED__ */


#ifndef __IWbemQualifierSet_FWD_DEFINED__
#define __IWbemQualifierSet_FWD_DEFINED__
typedef interface IWbemQualifierSet IWbemQualifierSet;

#endif 	/* __IWbemQualifierSet_FWD_DEFINED__ */


#ifndef __IWbemLocator_FWD_DEFINED__
#define __IWbemLocator_FWD_DEFINED__
typedef interface IWbemLocator IWbemLocator;

#endif 	/* __IWbemLocator_FWD_DEFINED__ */


#ifndef __IWbemObjectSink_FWD_DEFINED__
#define __IWbemObjectSink_FWD_DEFINED__
typedef interface IWbemObjectSink IWbemObjectSink;

#endif 	/* __IWbemObjectSink_FWD_DEFINED__ */


#ifndef __IWbemObjectSinkEx_FWD_DEFINED__
#define __IWbemObjectSinkEx_FWD_DEFINED__
typedef interface IWbemObjectSinkEx IWbemObjectSinkEx;

#endif 	/* __IWbemObjectSinkEx_FWD_DEFINED__ */


#ifndef __IEnumWbemClassObject_FWD_DEFINED__
#define __IEnumWbemClassObject_FWD_DEFINED__
typedef interface IEnumWbemClassObject IEnumWbemClassObject;

#endif 	/* __IEnumWbemClassObject_FWD_DEFINED__ */


#ifndef __IWbemContext_FWD_DEFINED__
#define __IWbemContext_FWD_DEFINED__
typedef interface IWbemContext IWbemContext;

#endif 	/* __IWbemContext_FWD_DEFINED__ */


#ifndef __IWbemCallResult_FWD_DEFINED__
#define __IWbemCallResult_FWD_DEFINED__
typedef interface IWbemCallResult IWbemCallResult;

#endif 	/* __IWbemCallResult_FWD_DEFINED__ */


#ifndef __IWbemServices_FWD_DEFINED__
#define __IWbemServices_FWD_DEFINED__
typedef interface IWbemServices IWbemServices;

#endif 	/* __IWbemServices_FWD_DEFINED__ */


#ifndef __IWbemShutdown_FWD_DEFINED__
#define __IWbemShutdown_FWD_DEFINED__
typedef interface IWbemShutdown IWbemShutdown;

#endif 	/* __IWbemShutdown_FWD_DEFINED__ */


#ifndef __IWbemObjectTextSrc_FWD_DEFINED__
#define __IWbemObjectTextSrc_FWD_DEFINED__
typedef interface IWbemObjectTextSrc IWbemObjectTextSrc;

#endif 	/* __IWbemObjectTextSrc_FWD_DEFINED__ */


#ifndef __IWbemObjectAccess_FWD_DEFINED__
#define __IWbemObjectAccess_FWD_DEFINED__
typedef interface IWbemObjectAccess IWbemObjectAccess;

#endif 	/* __IWbemObjectAccess_FWD_DEFINED__ */


#ifndef __IMofCompiler_FWD_DEFINED__
#define __IMofCompiler_FWD_DEFINED__
typedef interface IMofCompiler IMofCompiler;

#endif 	/* __IMofCompiler_FWD_DEFINED__ */


#ifndef __IUnsecuredApartment_FWD_DEFINED__
#define __IUnsecuredApartment_FWD_DEFINED__
typedef interface IUnsecuredApartment IUnsecuredApartment;

#endif 	/* __IUnsecuredApartment_FWD_DEFINED__ */


#ifndef __IWbemUnsecuredApartment_FWD_DEFINED__
#define __IWbemUnsecuredApartment_FWD_DEFINED__
typedef interface IWbemUnsecuredApartment IWbemUnsecuredApartment;

#endif 	/* __IWbemUnsecuredApartment_FWD_DEFINED__ */


#ifndef __IWbemStatusCodeText_FWD_DEFINED__
#define __IWbemStatusCodeText_FWD_DEFINED__
typedef interface IWbemStatusCodeText IWbemStatusCodeText;

#endif 	/* __IWbemStatusCodeText_FWD_DEFINED__ */


#ifndef __IWbemBackupRestore_FWD_DEFINED__
#define __IWbemBackupRestore_FWD_DEFINED__
typedef interface IWbemBackupRestore IWbemBackupRestore;

#endif 	/* __IWbemBackupRestore_FWD_DEFINED__ */


#ifndef __IWbemBackupRestoreEx_FWD_DEFINED__
#define __IWbemBackupRestoreEx_FWD_DEFINED__
typedef interface IWbemBackupRestoreEx IWbemBackupRestoreEx;

#endif 	/* __IWbemBackupRestoreEx_FWD_DEFINED__ */


#ifndef __IWbemRefresher_FWD_DEFINED__
#define __IWbemRefresher_FWD_DEFINED__
typedef interface IWbemRefresher IWbemRefresher;

#endif 	/* __IWbemRefresher_FWD_DEFINED__ */


#ifndef __IWbemHiPerfEnum_FWD_DEFINED__
#define __IWbemHiPerfEnum_FWD_DEFINED__
typedef interface IWbemHiPerfEnum IWbemHiPerfEnum;

#endif 	/* __IWbemHiPerfEnum_FWD_DEFINED__ */


#ifndef __IWbemConfigureRefresher_FWD_DEFINED__
#define __IWbemConfigureRefresher_FWD_DEFINED__
typedef interface IWbemConfigureRefresher IWbemConfigureRefresher;

#endif 	/* __IWbemConfigureRefresher_FWD_DEFINED__ */


#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_wbemcli_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family or WinMgmt Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_WINMGMT)


extern RPC_IF_HANDLE __MIDL_itf_wbemcli_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wbemcli_0000_0000_v0_0_s_ifspec;


#ifndef __WbemClient_v1_LIBRARY_DEFINED__
#define __WbemClient_v1_LIBRARY_DEFINED__

/* library WbemClient_v1 */
/* [uuid] */ 





















typedef /* [v1_enum] */ 
enum tag_WBEM_GENUS_TYPE
    {
        WBEM_GENUS_CLASS	= 1,
        WBEM_GENUS_INSTANCE	= 2
    } 	WBEM_GENUS_TYPE;

typedef /* [v1_enum] */ 
enum tag_WBEM_CHANGE_FLAG_TYPE
    {
        WBEM_FLAG_CREATE_OR_UPDATE	= 0,
        WBEM_FLAG_UPDATE_ONLY	= 0x1,
        WBEM_FLAG_CREATE_ONLY	= 0x2,
        WBEM_FLAG_UPDATE_COMPATIBLE	= 0,
        WBEM_FLAG_UPDATE_SAFE_MODE	= 0x20,
        WBEM_FLAG_UPDATE_FORCE_MODE	= 0x40,
        WBEM_MASK_UPDATE_MODE	= 0x60,
        WBEM_FLAG_ADVISORY	= 0x10000
    } 	WBEM_CHANGE_FLAG_TYPE;

typedef /* [v1_enum] */ 
enum tag_WBEM_GENERIC_FLAG_TYPE
    {
        WBEM_FLAG_RETURN_IMMEDIATELY	= 0x10,
        WBEM_FLAG_RETURN_WBEM_COMPLETE	= 0,
        WBEM_FLAG_BIDIRECTIONAL	= 0,
        WBEM_FLAG_FORWARD_ONLY	= 0x20,
        WBEM_FLAG_NO_ERROR_OBJECT	= 0x40,
        WBEM_FLAG_RETURN_ERROR_OBJECT	= 0,
        WBEM_FLAG_SEND_STATUS	= 0x80,
        WBEM_FLAG_DONT_SEND_STATUS	= 0,
        WBEM_FLAG_ENSURE_LOCATABLE	= 0x100,
        WBEM_FLAG_DIRECT_READ	= 0x200,
        WBEM_FLAG_SEND_ONLY_SELECTED	= 0,
        WBEM_RETURN_WHEN_COMPLETE	= 0,
        WBEM_RETURN_IMMEDIATELY	= 0x10,
        WBEM_MASK_RESERVED_FLAGS	= 0x1f000,
        WBEM_FLAG_USE_AMENDED_QUALIFIERS	= 0x20000,
        WBEM_FLAG_STRONG_VALIDATION	= 0x100000
    } 	WBEM_GENERIC_FLAG_TYPE;

typedef 
enum tag_WBEM_STATUS_TYPE
    {
        WBEM_STATUS_COMPLETE	= 0,
        WBEM_STATUS_REQUIREMENTS	= 1,
        WBEM_STATUS_PROGRESS	= 2,
        WBEM_STATUS_LOGGING_INFORMATION	= 0x100,
        WBEM_STATUS_LOGGING_INFORMATION_PROVIDER	= 0x200,
        WBEM_STATUS_LOGGING_INFORMATION_HOST	= 0x400,
        WBEM_STATUS_LOGGING_INFORMATION_REPOSITORY	= 0x800,
        WBEM_STATUS_LOGGING_INFORMATION_ESS	= 0x1000
    } 	WBEM_STATUS_TYPE;

typedef /* [v1_enum] */ 
enum tag_WBEM_TIMEOUT_TYPE
    {
        WBEM_NO_WAIT	= 0,
        WBEM_INFINITE	= 0xffffffff
    } 	WBEM_TIMEOUT_TYPE;

typedef /* [v1_enum] */ 
enum tag_WBEM_CONDITION_FLAG_TYPE
    {
        WBEM_FLAG_ALWAYS	= 0,
        WBEM_FLAG_ONLY_IF_TRUE	= 0x1,
        WBEM_FLAG_ONLY_IF_FALSE	= 0x2,
        WBEM_FLAG_ONLY_IF_IDENTICAL	= 0x3,
        WBEM_MASK_PRIMARY_CONDITION	= 0x3,
        WBEM_FLAG_KEYS_ONLY	= 0x4,
        WBEM_FLAG_REFS_ONLY	= 0x8,
        WBEM_FLAG_LOCAL_ONLY	= 0x10,
        WBEM_FLAG_PROPAGATED_ONLY	= 0x20,
        WBEM_FLAG_SYSTEM_ONLY	= 0x30,
        WBEM_FLAG_NONSYSTEM_ONLY	= 0x40,
        WBEM_MASK_CONDITION_ORIGIN	= 0x70,
        WBEM_FLAG_CLASS_OVERRIDES_ONLY	= 0x100,
        WBEM_FLAG_CLASS_LOCAL_AND_OVERRIDES	= 0x200,
        WBEM_MASK_CLASS_CONDITION	= 0x300
    } 	WBEM_CONDITION_FLAG_TYPE;

typedef /* [v1_enum] */ 
enum tag_WBEM_FLAVOR_TYPE
    {
        WBEM_FLAVOR_DONT_PROPAGATE	= 0,
        WBEM_FLAVOR_FLAG_PROPAGATE_TO_INSTANCE	= 0x1,
        WBEM_FLAVOR_FLAG_PROPAGATE_TO_DERIVED_CLASS	= 0x2,
        WBEM_FLAVOR_MASK_PROPAGATION	= 0xf,
        WBEM_FLAVOR_OVERRIDABLE	= 0,
        WBEM_FLAVOR_NOT_OVERRIDABLE	= 0x10,
        WBEM_FLAVOR_MASK_PERMISSIONS	= 0x10,
        WBEM_FLAVOR_ORIGIN_LOCAL	= 0,
        WBEM_FLAVOR_ORIGIN_PROPAGATED	= 0x20,
        WBEM_FLAVOR_ORIGIN_SYSTEM	= 0x40,
        WBEM_FLAVOR_MASK_ORIGIN	= 0x60,
        WBEM_FLAVOR_NOT_AMENDED	= 0,
        WBEM_FLAVOR_AMENDED	= 0x80,
        WBEM_FLAVOR_MASK_AMENDED	= 0x80
    } 	WBEM_FLAVOR_TYPE;

typedef /* [v1_enum] */ 
enum tag_WBEM_QUERY_FLAG_TYPE
    {
        WBEM_FLAG_DEEP	= 0,
        WBEM_FLAG_SHALLOW	= 1,
        WBEM_FLAG_PROTOTYPE	= 2
    } 	WBEM_QUERY_FLAG_TYPE;

typedef /* [v1_enum] */ 
enum tag_WBEM_SECURITY_FLAGS
    {
        WBEM_ENABLE	= 1,
        WBEM_METHOD_EXECUTE	= 2,
        WBEM_FULL_WRITE_REP	= 4,
        WBEM_PARTIAL_WRITE_REP	= 8,
        WBEM_WRITE_PROVIDER	= 0x10,
        WBEM_REMOTE_ACCESS	= 0x20,
        WBEM_RIGHT_SUBSCRIBE	= 0x40,
        WBEM_RIGHT_PUBLISH	= 0x80
    } 	WBEM_SECURITY_FLAGS;

typedef /* [v1_enum] */ 
enum tag_WBEM_LIMITATION_FLAG_TYPE
    {
        WBEM_FLAG_EXCLUDE_OBJECT_QUALIFIERS	= 0x10,
        WBEM_FLAG_EXCLUDE_PROPERTY_QUALIFIERS	= 0x20
    } 	WBEM_LIMITATION_FLAG_TYPE;

typedef /* [v1_enum] */ 
enum tag_WBEM_TEXT_FLAG_TYPE
    {
        WBEM_FLAG_NO_FLAVORS	= 0x1
    } 	WBEM_TEXT_FLAG_TYPE;

typedef /* [v1_enum] */ 
enum tag_WBEM_COMPARISON_FLAG
    {
        WBEM_COMPARISON_INCLUDE_ALL	= 0,
        WBEM_FLAG_IGNORE_QUALIFIERS	= 0x1,
        WBEM_FLAG_IGNORE_OBJECT_SOURCE	= 0x2,
        WBEM_FLAG_IGNORE_DEFAULT_VALUES	= 0x4,
        WBEM_FLAG_IGNORE_CLASS	= 0x8,
        WBEM_FLAG_IGNORE_CASE	= 0x10,
        WBEM_FLAG_IGNORE_FLAVOR	= 0x20
    } 	WBEM_COMPARISON_FLAG;

typedef /* [v1_enum] */ 
enum tag_WBEM_LOCKING
    {
        WBEM_FLAG_ALLOW_READ	= 0x1
    } 	WBEM_LOCKING_FLAG_TYPE;

typedef /* [v1_enum] */ 
enum tag_CIMTYPE_ENUMERATION
    {
        CIM_ILLEGAL	= 0xfff,
        CIM_EMPTY	= 0,
        CIM_SINT8	= 16,
        CIM_UINT8	= 17,
        CIM_SINT16	= 2,
        CIM_UINT16	= 18,
        CIM_SINT32	= 3,
        CIM_UINT32	= 19,
        CIM_SINT64	= 20,
        CIM_UINT64	= 21,
        CIM_REAL32	= 4,
        CIM_REAL64	= 5,
        CIM_BOOLEAN	= 11,
        CIM_STRING	= 8,
        CIM_DATETIME	= 101,
        CIM_REFERENCE	= 102,
        CIM_CHAR16	= 103,
        CIM_OBJECT	= 13,
        CIM_FLAG_ARRAY	= 0x2000
    } 	CIMTYPE_ENUMERATION;

typedef /* [v1_enum] */ 
enum tag_WBEM_BACKUP_RESTORE_FLAGS
    {
        WBEM_FLAG_BACKUP_RESTORE_DEFAULT	= 0,
        WBEM_FLAG_BACKUP_RESTORE_FORCE_SHUTDOWN	= 1
    } 	WBEM_BACKUP_RESTORE_FLAGS;

typedef /* [v1_enum] */ 
enum tag_WBEM_REFRESHER_FLAGS
    {
        WBEM_FLAG_REFRESH_AUTO_RECONNECT	= 0,
        WBEM_FLAG_REFRESH_NO_AUTO_RECONNECT	= 1
    } 	WBEM_REFRESHER_FLAGS;

typedef 
enum tag_WBEM_SHUTDOWN_FLAGS
    {
        WBEM_SHUTDOWN_UNLOAD_COMPONENT	= 1,
        WBEM_SHUTDOWN_WMI	= 2,
        WBEM_SHUTDOWN_OS	= 3
    } 	WBEM_SHUTDOWN_FLAGS;

typedef long CIMTYPE;

typedef /* [v1_enum] */ 
enum tag_WBEMSTATUS_FORMAT
    {
        WBEMSTATUS_FORMAT_NEWLINE	= 0,
        WBEMSTATUS_FORMAT_NO_NEWLINE	= 1
    } 	WBEMSTATUS_FORMAT;

typedef /* [v1_enum] */ 
enum tag_WBEM_LIMITS
    {
        WBEM_MAX_IDENTIFIER	= 0x1000,
        WBEM_MAX_QUERY	= 0x4000,
        WBEM_MAX_PATH	= 0x2000,
        WBEM_MAX_OBJECT_NESTING	= 64,
        WBEM_MAX_USER_PROPERTIES	= 1024
    } 	WBEM_LIMITS;

typedef /* [v1_enum] */ 
enum tag_WBEMSTATUS
    {
        WBEM_NO_ERROR	= 0,
        WBEM_S_NO_ERROR	= 0,
        WBEM_S_SAME	= 0,
        WBEM_S_FALSE	= 1,
        WBEM_S_ALREADY_EXISTS	= 0x40001,
        WBEM_S_RESET_TO_DEFAULT	= 0x40002,
        WBEM_S_DIFFERENT	= 0x40003,
        WBEM_S_TIMEDOUT	= 0x40004,
        WBEM_S_NO_MORE_DATA	= 0x40005,
        WBEM_S_OPERATION_CANCELLED	= 0x40006,
        WBEM_S_PENDING	= 0x40007,
        WBEM_S_DUPLICATE_OBJECTS	= 0x40008,
        WBEM_S_ACCESS_DENIED	= 0x40009,
        WBEM_S_PARTIAL_RESULTS	= 0x40010,
        WBEM_S_SOURCE_NOT_AVAILABLE	= 0x40017,
        WBEM_E_FAILED	= 0x80041001,
        WBEM_E_NOT_FOUND	= 0x80041002,
        WBEM_E_ACCESS_DENIED	= 0x80041003,
        WBEM_E_PROVIDER_FAILURE	= 0x80041004,
        WBEM_E_TYPE_MISMATCH	= 0x80041005,
        WBEM_E_OUT_OF_MEMORY	= 0x80041006,
        WBEM_E_INVALID_CONTEXT	= 0x80041007,
        WBEM_E_INVALID_PARAMETER	= 0x80041008,
        WBEM_E_NOT_AVAILABLE	= 0x80041009,
        WBEM_E_CRITICAL_ERROR	= 0x8004100a,
        WBEM_E_INVALID_STREAM	= 0x8004100b,
        WBEM_E_NOT_SUPPORTED	= 0x8004100c,
        WBEM_E_INVALID_SUPERCLASS	= 0x8004100d,
        WBEM_E_INVALID_NAMESPACE	= 0x8004100e,
        WBEM_E_INVALID_OBJECT	= 0x8004100f,
        WBEM_E_INVALID_CLASS	= 0x80041010,
        WBEM_E_PROVIDER_NOT_FOUND	= 0x80041011,
        WBEM_E_INVALID_PROVIDER_REGISTRATION	= 0x80041012,
        WBEM_E_PROVIDER_LOAD_FAILURE	= 0x80041013,
        WBEM_E_INITIALIZATION_FAILURE	= 0x80041014,
        WBEM_E_TRANSPORT_FAILURE	= 0x80041015,
        WBEM_E_INVALID_OPERATION	= 0x80041016,
        WBEM_E_INVALID_QUERY	= 0x80041017,
        WBEM_E_INVALID_QUERY_TYPE	= 0x80041018,
        WBEM_E_ALREADY_EXISTS	= 0x80041019,
        WBEM_E_OVERRIDE_NOT_ALLOWED	= 0x8004101a,
        WBEM_E_PROPAGATED_QUALIFIER	= 0x8004101b,
        WBEM_E_PROPAGATED_PROPERTY	= 0x8004101c,
        WBEM_E_UNEXPECTED	= 0x8004101d,
        WBEM_E_ILLEGAL_OPERATION	= 0x8004101e,
        WBEM_E_CANNOT_BE_KEY	= 0x8004101f,
        WBEM_E_INCOMPLETE_CLASS	= 0x80041020,
        WBEM_E_INVALID_SYNTAX	= 0x80041021,
        WBEM_E_NONDECORATED_OBJECT	= 0x80041022,
        WBEM_E_READ_ONLY	= 0x80041023,
        WBEM_E_PROVIDER_NOT_CAPABLE	= 0x80041024,
        WBEM_E_CLASS_HAS_CHILDREN	= 0x80041025,
        WBEM_E_CLASS_HAS_INSTANCES	= 0x80041026,
        WBEM_E_QUERY_NOT_IMPLEMENTED	= 0x80041027,
        WBEM_E_ILLEGAL_NULL	= 0x80041028,
        WBEM_E_INVALID_QUALIFIER_TYPE	= 0x80041029,
        WBEM_E_INVALID_PROPERTY_TYPE	= 0x8004102a,
        WBEM_E_VALUE_OUT_OF_RANGE	= 0x8004102b,
        WBEM_E_CANNOT_BE_SINGLETON	= 0x8004102c,
        WBEM_E_INVALID_CIM_TYPE	= 0x8004102d,
        WBEM_E_INVALID_METHOD	= 0x8004102e,
        WBEM_E_INVALID_METHOD_PARAMETERS	= 0x8004102f,
        WBEM_E_SYSTEM_PROPERTY	= 0x80041030,
        WBEM_E_INVALID_PROPERTY	= 0x80041031,
        WBEM_E_CALL_CANCELLED	= 0x80041032,
        WBEM_E_SHUTTING_DOWN	= 0x80041033,
        WBEM_E_PROPAGATED_METHOD	= 0x80041034,
        WBEM_E_UNSUPPORTED_PARAMETER	= 0x80041035,
        WBEM_E_MISSING_PARAMETER_ID	= 0x80041036,
        WBEM_E_INVALID_PARAMETER_ID	= 0x80041037,
        WBEM_E_NONCONSECUTIVE_PARAMETER_IDS	= 0x80041038,
        WBEM_E_PARAMETER_ID_ON_RETVAL	= 0x80041039,
        WBEM_E_INVALID_OBJECT_PATH	= 0x8004103a,
        WBEM_E_OUT_OF_DISK_SPACE	= 0x8004103b,
        WBEM_E_BUFFER_TOO_SMALL	= 0x8004103c,
        WBEM_E_UNSUPPORTED_PUT_EXTENSION	= 0x8004103d,
        WBEM_E_UNKNOWN_OBJECT_TYPE	= 0x8004103e,
        WBEM_E_UNKNOWN_PACKET_TYPE	= 0x8004103f,
        WBEM_E_MARSHAL_VERSION_MISMATCH	= 0x80041040,
        WBEM_E_MARSHAL_INVALID_SIGNATURE	= 0x80041041,
        WBEM_E_INVALID_QUALIFIER	= 0x80041042,
        WBEM_E_INVALID_DUPLICATE_PARAMETER	= 0x80041043,
        WBEM_E_TOO_MUCH_DATA	= 0x80041044,
        WBEM_E_SERVER_TOO_BUSY	= 0x80041045,
        WBEM_E_INVALID_FLAVOR	= 0x80041046,
        WBEM_E_CIRCULAR_REFERENCE	= 0x80041047,
        WBEM_E_UNSUPPORTED_CLASS_UPDATE	= 0x80041048,
        WBEM_E_CANNOT_CHANGE_KEY_INHERITANCE	= 0x80041049,
        WBEM_E_CANNOT_CHANGE_INDEX_INHERITANCE	= 0x80041050,
        WBEM_E_TOO_MANY_PROPERTIES	= 0x80041051,
        WBEM_E_UPDATE_TYPE_MISMATCH	= 0x80041052,
        WBEM_E_UPDATE_OVERRIDE_NOT_ALLOWED	= 0x80041053,
        WBEM_E_UPDATE_PROPAGATED_METHOD	= 0x80041054,
        WBEM_E_METHOD_NOT_IMPLEMENTED	= 0x80041055,
        WBEM_E_METHOD_DISABLED	= 0x80041056,
        WBEM_E_REFRESHER_BUSY	= 0x80041057,
        WBEM_E_UNPARSABLE_QUERY	= 0x80041058,
        WBEM_E_NOT_EVENT_CLASS	= 0x80041059,
        WBEM_E_MISSING_GROUP_WITHIN	= 0x8004105a,
        WBEM_E_MISSING_AGGREGATION_LIST	= 0x8004105b,
        WBEM_E_PROPERTY_NOT_AN_OBJECT	= 0x8004105c,
        WBEM_E_AGGREGATING_BY_OBJECT	= 0x8004105d,
        WBEM_E_UNINTERPRETABLE_PROVIDER_QUERY	= 0x8004105f,
        WBEM_E_BACKUP_RESTORE_WINMGMT_RUNNING	= 0x80041060,
        WBEM_E_QUEUE_OVERFLOW	= 0x80041061,
        WBEM_E_PRIVILEGE_NOT_HELD	= 0x80041062,
        WBEM_E_INVALID_OPERATOR	= 0x80041063,
        WBEM_E_LOCAL_CREDENTIALS	= 0x80041064,
        WBEM_E_CANNOT_BE_ABSTRACT	= 0x80041065,
        WBEM_E_AMENDED_OBJECT	= 0x80041066,
        WBEM_E_CLIENT_TOO_SLOW	= 0x80041067,
        WBEM_E_NULL_SECURITY_DESCRIPTOR	= 0x80041068,
        WBEM_E_TIMED_OUT	= 0x80041069,
        WBEM_E_INVALID_ASSOCIATION	= 0x8004106a,
        WBEM_E_AMBIGUOUS_OPERATION	= 0x8004106b,
        WBEM_E_QUOTA_VIOLATION	= 0x8004106c,
        WBEM_E_RESERVED_001	= 0x8004106d,
        WBEM_E_RESERVED_002	= 0x8004106e,
        WBEM_E_UNSUPPORTED_LOCALE	= 0x8004106f,
        WBEM_E_HANDLE_OUT_OF_DATE	= 0x80041070,
        WBEM_E_CONNECTION_FAILED	= 0x80041071,
        WBEM_E_INVALID_HANDLE_REQUEST	= 0x80041072,
        WBEM_E_PROPERTY_NAME_TOO_WIDE	= 0x80041073,
        WBEM_E_CLASS_NAME_TOO_WIDE	= 0x80041074,
        WBEM_E_METHOD_NAME_TOO_WIDE	= 0x80041075,
        WBEM_E_QUALIFIER_NAME_TOO_WIDE	= 0x80041076,
        WBEM_E_RERUN_COMMAND	= 0x80041077,
        WBEM_E_DATABASE_VER_MISMATCH	= 0x80041078,
        WBEM_E_VETO_DELETE	= 0x80041079,
        WBEM_E_VETO_PUT	= 0x8004107a,
        WBEM_E_INVALID_LOCALE	= 0x80041080,
        WBEM_E_PROVIDER_SUSPENDED	= 0x80041081,
        WBEM_E_SYNCHRONIZATION_REQUIRED	= 0x80041082,
        WBEM_E_NO_SCHEMA	= 0x80041083,
        WBEM_E_PROVIDER_ALREADY_REGISTERED	= 0x80041084,
        WBEM_E_PROVIDER_NOT_REGISTERED	= 0x80041085,
        WBEM_E_FATAL_TRANSPORT_ERROR	= 0x80041086,
        WBEM_E_ENCRYPTED_CONNECTION_REQUIRED	= 0x80041087,
        WBEM_E_PROVIDER_TIMED_OUT	= 0x80041088,
        WBEM_E_NO_KEY	= 0x80041089,
        WBEM_E_PROVIDER_DISABLED	= 0x8004108a,
        WBEMESS_E_REGISTRATION_TOO_BROAD	= 0x80042001,
        WBEMESS_E_REGISTRATION_TOO_PRECISE	= 0x80042002,
        WBEMESS_E_AUTHZ_NOT_PRIVILEGED	= 0x80042003,
        WBEMMOF_E_EXPECTED_QUALIFIER_NAME	= 0x80044001,
        WBEMMOF_E_EXPECTED_SEMI	= 0x80044002,
        WBEMMOF_E_EXPECTED_OPEN_BRACE	= 0x80044003,
        WBEMMOF_E_EXPECTED_CLOSE_BRACE	= 0x80044004,
        WBEMMOF_E_EXPECTED_CLOSE_BRACKET	= 0x80044005,
        WBEMMOF_E_EXPECTED_CLOSE_PAREN	= 0x80044006,
        WBEMMOF_E_ILLEGAL_CONSTANT_VALUE	= 0x80044007,
        WBEMMOF_E_EXPECTED_TYPE_IDENTIFIER	= 0x80044008,
        WBEMMOF_E_EXPECTED_OPEN_PAREN	= 0x80044009,
        WBEMMOF_E_UNRECOGNIZED_TOKEN	= 0x8004400a,
        WBEMMOF_E_UNRECOGNIZED_TYPE	= 0x8004400b,
        WBEMMOF_E_EXPECTED_PROPERTY_NAME	= 0x8004400c,
        WBEMMOF_E_TYPEDEF_NOT_SUPPORTED	= 0x8004400d,
        WBEMMOF_E_UNEXPECTED_ALIAS	= 0x8004400e,
        WBEMMOF_E_UNEXPECTED_ARRAY_INIT	= 0x8004400f,
        WBEMMOF_E_INVALID_AMENDMENT_SYNTAX	= 0x80044010,
        WBEMMOF_E_INVALID_DUPLICATE_AMENDMENT	= 0x80044011,
        WBEMMOF_E_INVALID_PRAGMA	= 0x80044012,
        WBEMMOF_E_INVALID_NAMESPACE_SYNTAX	= 0x80044013,
        WBEMMOF_E_EXPECTED_CLASS_NAME	= 0x80044014,
        WBEMMOF_E_TYPE_MISMATCH	= 0x80044015,
        WBEMMOF_E_EXPECTED_ALIAS_NAME	= 0x80044016,
        WBEMMOF_E_INVALID_CLASS_DECLARATION	= 0x80044017,
        WBEMMOF_E_INVALID_INSTANCE_DECLARATION	= 0x80044018,
        WBEMMOF_E_EXPECTED_DOLLAR	= 0x80044019,
        WBEMMOF_E_CIMTYPE_QUALIFIER	= 0x8004401a,
        WBEMMOF_E_DUPLICATE_PROPERTY	= 0x8004401b,
        WBEMMOF_E_INVALID_NAMESPACE_SPECIFICATION	= 0x8004401c,
        WBEMMOF_E_OUT_OF_RANGE	= 0x8004401d,
        WBEMMOF_E_INVALID_FILE	= 0x8004401e,
        WBEMMOF_E_ALIASES_IN_EMBEDDED	= 0x8004401f,
        WBEMMOF_E_NULL_ARRAY_ELEM	= 0x80044020,
        WBEMMOF_E_DUPLICATE_QUALIFIER	= 0x80044021,
        WBEMMOF_E_EXPECTED_FLAVOR_TYPE	= 0x80044022,
        WBEMMOF_E_INCOMPATIBLE_FLAVOR_TYPES	= 0x80044023,
        WBEMMOF_E_MULTIPLE_ALIASES	= 0x80044024,
        WBEMMOF_E_INCOMPATIBLE_FLAVOR_TYPES2	= 0x80044025,
        WBEMMOF_E_NO_ARRAYS_RETURNED	= 0x80044026,
        WBEMMOF_E_MUST_BE_IN_OR_OUT	= 0x80044027,
        WBEMMOF_E_INVALID_FLAGS_SYNTAX	= 0x80044028,
        WBEMMOF_E_EXPECTED_BRACE_OR_BAD_TYPE	= 0x80044029,
        WBEMMOF_E_UNSUPPORTED_CIMV22_QUAL_VALUE	= 0x8004402a,
        WBEMMOF_E_UNSUPPORTED_CIMV22_DATA_TYPE	= 0x8004402b,
        WBEMMOF_E_INVALID_DELETEINSTANCE_SYNTAX	= 0x8004402c,
        WBEMMOF_E_INVALID_QUALIFIER_SYNTAX	= 0x8004402d,
        WBEMMOF_E_QUALIFIER_USED_OUTSIDE_SCOPE	= 0x8004402e,
        WBEMMOF_E_ERROR_CREATING_TEMP_FILE	= 0x8004402f,
        WBEMMOF_E_ERROR_INVALID_INCLUDE_FILE	= 0x80044030,
        WBEMMOF_E_INVALID_DELETECLASS_SYNTAX	= 0x80044031
    } 	WBEMSTATUS;


EXTERN_C const IID LIBID_WbemClient_v1;

#ifndef __IWbemClassObject_INTERFACE_DEFINED__
#define __IWbemClassObject_INTERFACE_DEFINED__

/* interface IWbemClassObject */
/* [uuid][object][restricted][local] */ 


EXTERN_C const IID IID_IWbemClassObject;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("dc12a681-737f-11cf-884d-00aa004b2e24")
    IWbemClassObject : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetQualifierSet( 
            /* [out] */ IWbemQualifierSet **ppQualSet) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Get( 
            /* [string][in] */ LPCWSTR wszName,
            /* [in] */ long lFlags,
            /* [unique][in][out] */ VARIANT *pVal,
            /* [unique][in][out] */ CIMTYPE *pType,
            /* [unique][in][out] */ long *plFlavor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Put( 
            /* [string][in] */ LPCWSTR wszName,
            /* [in] */ long lFlags,
            /* [in] */ VARIANT *pVal,
            /* [in] */ CIMTYPE Type) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Delete( 
            /* [string][in] */ LPCWSTR wszName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNames( 
            /* [string][in] */ LPCWSTR wszQualifierName,
            /* [in] */ long lFlags,
            /* [in] */ VARIANT *pQualifierVal,
            /* [out] */ SAFEARRAY * *pNames) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE BeginEnumeration( 
            /* [in] */ long lEnumFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ long lFlags,
            /* [unique][in][out] */ BSTR *strName,
            /* [unique][in][out] */ VARIANT *pVal,
            /* [unique][in][out] */ CIMTYPE *pType,
            /* [unique][in][out] */ long *plFlavor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EndEnumeration( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPropertyQualifierSet( 
            /* [string][in] */ LPCWSTR wszProperty,
            /* [out] */ IWbemQualifierSet **ppQualSet) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ IWbemClassObject **ppCopy) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetObjectText( 
            /* [in] */ long lFlags,
            /* [out] */ BSTR *pstrObjectText) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SpawnDerivedClass( 
            /* [in] */ long lFlags,
            /* [out] */ IWbemClassObject **ppNewClass) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SpawnInstance( 
            /* [in] */ long lFlags,
            /* [out] */ IWbemClassObject **ppNewInstance) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CompareTo( 
            /* [in] */ long lFlags,
            /* [in] */ IWbemClassObject *pCompareTo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPropertyOrigin( 
            /* [string][in] */ LPCWSTR wszName,
            /* [out] */ BSTR *pstrClassName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InheritsFrom( 
            /* [in] */ LPCWSTR strAncestor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMethod( 
            /* [string][in] */ LPCWSTR wszName,
            /* [in] */ long lFlags,
            /* [out] */ IWbemClassObject **ppInSignature,
            /* [out] */ IWbemClassObject **ppOutSignature) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE PutMethod( 
            /* [string][in] */ LPCWSTR wszName,
            /* [in] */ long lFlags,
            /* [in] */ IWbemClassObject *pInSignature,
            /* [in] */ IWbemClassObject *pOutSignature) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteMethod( 
            /* [string][in] */ LPCWSTR wszName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE BeginMethodEnumeration( 
            /* [in] */ long lEnumFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE NextMethod( 
            /* [in] */ long lFlags,
            /* [unique][in][out] */ BSTR *pstrName,
            /* [unique][in][out] */ IWbemClassObject **ppInSignature,
            /* [unique][in][out] */ IWbemClassObject **ppOutSignature) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EndMethodEnumeration( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMethodQualifierSet( 
            /* [string][in] */ LPCWSTR wszMethod,
            /* [out] */ IWbemQualifierSet **ppQualSet) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMethodOrigin( 
            /* [string][in] */ LPCWSTR wszMethodName,
            /* [out] */ BSTR *pstrClassName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWbemClassObjectVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWbemClassObject * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWbemClassObject * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWbemClassObject * This);
        
        DECLSPEC_XFGVIRT(IWbemClassObject, GetQualifierSet)
        HRESULT ( STDMETHODCALLTYPE *GetQualifierSet )( 
            IWbemClassObject * This,
            /* [out] */ IWbemQualifierSet **ppQualSet);
        
        DECLSPEC_XFGVIRT(IWbemClassObject, Get)
        HRESULT ( STDMETHODCALLTYPE *Get )( 
            IWbemClassObject * This,
            /* [string][in] */ LPCWSTR wszName,
            /* [in] */ long lFlags,
            /* [unique][in][out] */ VARIANT *pVal,
            /* [unique][in][out] */ CIMTYPE *pType,
            /* [unique][in][out] */ long *plFlavor);
        
        DECLSPEC_XFGVIRT(IWbemClassObject, Put)
        HRESULT ( STDMETHODCALLTYPE *Put )( 
            IWbemClassObject * This,
            /* [string][in] */ LPCWSTR wszName,
            /* [in] */ long lFlags,
            /* [in] */ VARIANT *pVal,
            /* [in] */ CIMTYPE Type);
        
        DECLSPEC_XFGVIRT(IWbemClassObject, Delete)
        HRESULT ( STDMETHODCALLTYPE *Delete )( 
            IWbemClassObject * This,
            /* [string][in] */ LPCWSTR wszName);
        
        DECLSPEC_XFGVIRT(IWbemClassObject, GetNames)
        HRESULT ( STDMETHODCALLTYPE *GetNames )( 
            IWbemClassObject * This,
            /* [string][in] */ LPCWSTR wszQualifierName,
            /* [in] */ long lFlags,
            /* [in] */ VARIANT *pQualifierVal,
            /* [out] */ SAFEARRAY * *pNames);
        
        DECLSPEC_XFGVIRT(IWbemClassObject, BeginEnumeration)
        HRESULT ( STDMETHODCALLTYPE *BeginEnumeration )( 
            IWbemClassObject * This,
            /* [in] */ long lEnumFlags);
        
        DECLSPEC_XFGVIRT(IWbemClassObject, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            IWbemClassObject * This,
            /* [in] */ long lFlags,
            /* [unique][in][out] */ BSTR *strName,
            /* [unique][in][out] */ VARIANT *pVal,
            /* [unique][in][out] */ CIMTYPE *pType,
            /* [unique][in][out] */ long *plFlavor);
        
        DECLSPEC_XFGVIRT(IWbemClassObject, EndEnumeration)
        HRESULT ( STDMETHODCALLTYPE *EndEnumeration )( 
            IWbemClassObject * This);
        
        DECLSPEC_XFGVIRT(IWbemClassObject, GetPropertyQualifierSet)
        HRESULT ( STDMETHODCALLTYPE *GetPropertyQualifierSet )( 
            IWbemClassObject * This,
            /* [string][in] */ LPCWSTR wszProperty,
            /* [out] */ IWbemQualifierSet **ppQualSet);
        
        DECLSPEC_XFGVIRT(IWbemClassObject, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            IWbemClassObject * This,
            /* [out] */ IWbemClassObject **ppCopy);
        
        DECLSPEC_XFGVIRT(IWbemClassObject, GetObjectText)
        HRESULT ( STDMETHODCALLTYPE *GetObjectText )( 
            IWbemClassObject * This,
            /* [in] */ long lFlags,
            /* [out] */ BSTR *pstrObjectText);
        
        DECLSPEC_XFGVIRT(IWbemClassObject, SpawnDerivedClass)
        HRESULT ( STDMETHODCALLTYPE *SpawnDerivedClass )( 
            IWbemClassObject * This,
            /* [in] */ long lFlags,
            /* [out] */ IWbemClassObject **ppNewClass);
        
        DECLSPEC_XFGVIRT(IWbemClassObject, SpawnInstance)
        HRESULT ( STDMETHODCALLTYPE *SpawnInstance )( 
            IWbemClassObject * This,
            /* [in] */ long lFlags,
            /* [out] */ IWbemClassObject **ppNewInstance);
        
        DECLSPEC_XFGVIRT(IWbemClassObject, CompareTo)
        HRESULT ( STDMETHODCALLTYPE *CompareTo )( 
            IWbemClassObject * This,
            /* [in] */ long lFlags,
            /* [in] */ IWbemClassObject *pCompareTo);
        
        DECLSPEC_XFGVIRT(IWbemClassObject, GetPropertyOrigin)
        HRESULT ( STDMETHODCALLTYPE *GetPropertyOrigin )( 
            IWbemClassObject * This,
            /* [string][in] */ LPCWSTR wszName,
            /* [out] */ BSTR *pstrClassName);
        
        DECLSPEC_XFGVIRT(IWbemClassObject, InheritsFrom)
        HRESULT ( STDMETHODCALLTYPE *InheritsFrom )( 
            IWbemClassObject * This,
            /* [in] */ LPCWSTR strAncestor);
        
        DECLSPEC_XFGVIRT(IWbemClassObject, GetMethod)
        HRESULT ( STDMETHODCALLTYPE *GetMethod )( 
            IWbemClassObject * This,
            /* [string][in] */ LPCWSTR wszName,
            /* [in] */ long lFlags,
            /* [out] */ IWbemClassObject **ppInSignature,
            /* [out] */ IWbemClassObject **ppOutSignature);
        
        DECLSPEC_XFGVIRT(IWbemClassObject, PutMethod)
        HRESULT ( STDMETHODCALLTYPE *PutMethod )( 
            IWbemClassObject * This,
            /* [string][in] */ LPCWSTR wszName,
            /* [in] */ long lFlags,
            /* [in] */ IWbemClassObject *pInSignature,
            /* [in] */ IWbemClassObject *pOutSignature);
        
        DECLSPEC_XFGVIRT(IWbemClassObject, DeleteMethod)
        HRESULT ( STDMETHODCALLTYPE *DeleteMethod )( 
            IWbemClassObject * This,
            /* [string][in] */ LPCWSTR wszName);
        
        DECLSPEC_XFGVIRT(IWbemClassObject, BeginMethodEnumeration)
        HRESULT ( STDMETHODCALLTYPE *BeginMethodEnumeration )( 
            IWbemClassObject * This,
            /* [in] */ long lEnumFlags);
        
        DECLSPEC_XFGVIRT(IWbemClassObject, NextMethod)
        HRESULT ( STDMETHODCALLTYPE *NextMethod )( 
            IWbemClassObject * This,
            /* [in] */ long lFlags,
            /* [unique][in][out] */ BSTR *pstrName,
            /* [unique][in][out] */ IWbemClassObject **ppInSignature,
            /* [unique][in][out] */ IWbemClassObject **ppOutSignature);
        
        DECLSPEC_XFGVIRT(IWbemClassObject, EndMethodEnumeration)
        HRESULT ( STDMETHODCALLTYPE *EndMethodEnumeration )( 
            IWbemClassObject * This);
        
        DECLSPEC_XFGVIRT(IWbemClassObject, GetMethodQualifierSet)
        HRESULT ( STDMETHODCALLTYPE *GetMethodQualifierSet )( 
            IWbemClassObject * This,
            /* [string][in] */ LPCWSTR wszMethod,
            /* [out] */ IWbemQualifierSet **ppQualSet);
        
        DECLSPEC_XFGVIRT(IWbemClassObject, GetMethodOrigin)
        HRESULT ( STDMETHODCALLTYPE *GetMethodOrigin )( 
            IWbemClassObject * This,
            /* [string][in] */ LPCWSTR wszMethodName,
            /* [out] */ BSTR *pstrClassName);
        
        END_INTERFACE
    } IWbemClassObjectVtbl;

    interface IWbemClassObject
    {
        CONST_VTBL struct IWbemClassObjectVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWbemClassObject_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWbemClassObject_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWbemClassObject_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWbemClassObject_GetQualifierSet(This,ppQualSet)	\
    ( (This)->lpVtbl -> GetQualifierSet(This,ppQualSet) ) 

#define IWbemClassObject_Get(This,wszName,lFlags,pVal,pType,plFlavor)	\
    ( (This)->lpVtbl -> Get(This,wszName,lFlags,pVal,pType,plFlavor) ) 

#define IWbemClassObject_Put(This,wszName,lFlags,pVal,Type)	\
    ( (This)->lpVtbl -> Put(This,wszName,lFlags,pVal,Type) ) 

#define IWbemClassObject_Delete(This,wszName)	\
    ( (This)->lpVtbl -> Delete(This,wszName) ) 

#define IWbemClassObject_GetNames(This,wszQualifierName,lFlags,pQualifierVal,pNames)	\
    ( (This)->lpVtbl -> GetNames(This,wszQualifierName,lFlags,pQualifierVal,pNames) ) 

#define IWbemClassObject_BeginEnumeration(This,lEnumFlags)	\
    ( (This)->lpVtbl -> BeginEnumeration(This,lEnumFlags) ) 

#define IWbemClassObject_Next(This,lFlags,strName,pVal,pType,plFlavor)	\
    ( (This)->lpVtbl -> Next(This,lFlags,strName,pVal,pType,plFlavor) ) 

#define IWbemClassObject_EndEnumeration(This)	\
    ( (This)->lpVtbl -> EndEnumeration(This) ) 

#define IWbemClassObject_GetPropertyQualifierSet(This,wszProperty,ppQualSet)	\
    ( (This)->lpVtbl -> GetPropertyQualifierSet(This,wszProperty,ppQualSet) ) 

#define IWbemClassObject_Clone(This,ppCopy)	\
    ( (This)->lpVtbl -> Clone(This,ppCopy) ) 

#define IWbemClassObject_GetObjectText(This,lFlags,pstrObjectText)	\
    ( (This)->lpVtbl -> GetObjectText(This,lFlags,pstrObjectText) ) 

#define IWbemClassObject_SpawnDerivedClass(This,lFlags,ppNewClass)	\
    ( (This)->lpVtbl -> SpawnDerivedClass(This,lFlags,ppNewClass) ) 

#define IWbemClassObject_SpawnInstance(This,lFlags,ppNewInstance)	\
    ( (This)->lpVtbl -> SpawnInstance(This,lFlags,ppNewInstance) ) 

#define IWbemClassObject_CompareTo(This,lFlags,pCompareTo)	\
    ( (This)->lpVtbl -> CompareTo(This,lFlags,pCompareTo) ) 

#define IWbemClassObject_GetPropertyOrigin(This,wszName,pstrClassName)	\
    ( (This)->lpVtbl -> GetPropertyOrigin(This,wszName,pstrClassName) ) 

#define IWbemClassObject_InheritsFrom(This,strAncestor)	\
    ( (This)->lpVtbl -> InheritsFrom(This,strAncestor) ) 

#define IWbemClassObject_GetMethod(This,wszName,lFlags,ppInSignature,ppOutSignature)	\
    ( (This)->lpVtbl -> GetMethod(This,wszName,lFlags,ppInSignature,ppOutSignature) ) 

#define IWbemClassObject_PutMethod(This,wszName,lFlags,pInSignature,pOutSignature)	\
    ( (This)->lpVtbl -> PutMethod(This,wszName,lFlags,pInSignature,pOutSignature) ) 

#define IWbemClassObject_DeleteMethod(This,wszName)	\
    ( (This)->lpVtbl -> DeleteMethod(This,wszName) ) 

#define IWbemClassObject_BeginMethodEnumeration(This,lEnumFlags)	\
    ( (This)->lpVtbl -> BeginMethodEnumeration(This,lEnumFlags) ) 

#define IWbemClassObject_NextMethod(This,lFlags,pstrName,ppInSignature,ppOutSignature)	\
    ( (This)->lpVtbl -> NextMethod(This,lFlags,pstrName,ppInSignature,ppOutSignature) ) 

#define IWbemClassObject_EndMethodEnumeration(This)	\
    ( (This)->lpVtbl -> EndMethodEnumeration(This) ) 

#define IWbemClassObject_GetMethodQualifierSet(This,wszMethod,ppQualSet)	\
    ( (This)->lpVtbl -> GetMethodQualifierSet(This,wszMethod,ppQualSet) ) 

#define IWbemClassObject_GetMethodOrigin(This,wszMethodName,pstrClassName)	\
    ( (This)->lpVtbl -> GetMethodOrigin(This,wszMethodName,pstrClassName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWbemClassObject_INTERFACE_DEFINED__ */


#ifndef __IWbemObjectAccess_INTERFACE_DEFINED__
#define __IWbemObjectAccess_INTERFACE_DEFINED__

/* interface IWbemObjectAccess */
/* [uuid][object][restricted][local] */ 


EXTERN_C const IID IID_IWbemObjectAccess;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("49353c9a-516b-11d1-aea6-00c04fb68820")
    IWbemObjectAccess : public IWbemClassObject
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetPropertyHandle( 
            /* [string][in] */ LPCWSTR wszPropertyName,
            /* [out] */ CIMTYPE *pType,
            /* [out] */ long *plHandle) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WritePropertyValue( 
            /* [in] */ long lHandle,
            /* [in] */ long lNumBytes,
            /* [size_is][in] */ const byte *aData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ReadPropertyValue( 
            /* [in] */ long lHandle,
            /* [in] */ long lBufferSize,
            /* [out] */ long *plNumBytes,
            /* [length_is][size_is][out] */ byte *aData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ReadDWORD( 
            /* [in] */ long lHandle,
            /* [out] */ DWORD *pdw) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WriteDWORD( 
            /* [in] */ long lHandle,
            /* [in] */ DWORD dw) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ReadQWORD( 
            /* [in] */ long lHandle,
            /* [out] */ unsigned __int64 *pqw) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WriteQWORD( 
            /* [in] */ long lHandle,
            /* [in] */ unsigned __int64 pw) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPropertyInfoByHandle( 
            /* [in] */ long lHandle,
            /* [out] */ BSTR *pstrName,
            /* [out] */ CIMTYPE *pType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Lock( 
            /* [in] */ long lFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Unlock( 
            /* [in] */ long lFlags) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWbemObjectAccessVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWbemObjectAccess * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWbemObjectAccess * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWbemObjectAccess * This);
        
        DECLSPEC_XFGVIRT(IWbemClassObject, GetQualifierSet)
        HRESULT ( STDMETHODCALLTYPE *GetQualifierSet )( 
            IWbemObjectAccess * This,
            /* [out] */ IWbemQualifierSet **ppQualSet);
        
        DECLSPEC_XFGVIRT(IWbemClassObject, Get)
        HRESULT ( STDMETHODCALLTYPE *Get )( 
            IWbemObjectAccess * This,
            /* [string][in] */ LPCWSTR wszName,
            /* [in] */ long lFlags,
            /* [unique][in][out] */ VARIANT *pVal,
            /* [unique][in][out] */ CIMTYPE *pType,
            /* [unique][in][out] */ long *plFlavor);
        
        DECLSPEC_XFGVIRT(IWbemClassObject, Put)
        HRESULT ( STDMETHODCALLTYPE *Put )( 
            IWbemObjectAccess * This,
            /* [string][in] */ LPCWSTR wszName,
            /* [in] */ long lFlags,
            /* [in] */ VARIANT *pVal,
            /* [in] */ CIMTYPE Type);
        
        DECLSPEC_XFGVIRT(IWbemClassObject, Delete)
        HRESULT ( STDMETHODCALLTYPE *Delete )( 
            IWbemObjectAccess * This,
            /* [string][in] */ LPCWSTR wszName);
        
        DECLSPEC_XFGVIRT(IWbemClassObject, GetNames)
        HRESULT ( STDMETHODCALLTYPE *GetNames )( 
            IWbemObjectAccess * This,
            /* [string][in] */ LPCWSTR wszQualifierName,
            /* [in] */ long lFlags,
            /* [in] */ VARIANT *pQualifierVal,
            /* [out] */ SAFEARRAY * *pNames);
        
        DECLSPEC_XFGVIRT(IWbemClassObject, BeginEnumeration)
        HRESULT ( STDMETHODCALLTYPE *BeginEnumeration )( 
            IWbemObjectAccess * This,
            /* [in] */ long lEnumFlags);
        
        DECLSPEC_XFGVIRT(IWbemClassObject, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            IWbemObjectAccess * This,
            /* [in] */ long lFlags,
            /* [unique][in][out] */ BSTR *strName,
            /* [unique][in][out] */ VARIANT *pVal,
            /* [unique][in][out] */ CIMTYPE *pType,
            /* [unique][in][out] */ long *plFlavor);
        
        DECLSPEC_XFGVIRT(IWbemClassObject, EndEnumeration)
        HRESULT ( STDMETHODCALLTYPE *EndEnumeration )( 
            IWbemObjectAccess * This);
        
        DECLSPEC_XFGVIRT(IWbemClassObject, GetPropertyQualifierSet)
        HRESULT ( STDMETHODCALLTYPE *GetPropertyQualifierSet )( 
            IWbemObjectAccess * This,
            /* [string][in] */ LPCWSTR wszProperty,
            /* [out] */ IWbemQualifierSet **ppQualSet);
        
        DECLSPEC_XFGVIRT(IWbemClassObject, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            IWbemObjectAccess * This,
            /* [out] */ IWbemClassObject **ppCopy);
        
        DECLSPEC_XFGVIRT(IWbemClassObject, GetObjectText)
        HRESULT ( STDMETHODCALLTYPE *GetObjectText )( 
            IWbemObjectAccess * This,
            /* [in] */ long lFlags,
            /* [out] */ BSTR *pstrObjectText);
        
        DECLSPEC_XFGVIRT(IWbemClassObject, SpawnDerivedClass)
        HRESULT ( STDMETHODCALLTYPE *SpawnDerivedClass )( 
            IWbemObjectAccess * This,
            /* [in] */ long lFlags,
            /* [out] */ IWbemClassObject **ppNewClass);
        
        DECLSPEC_XFGVIRT(IWbemClassObject, SpawnInstance)
        HRESULT ( STDMETHODCALLTYPE *SpawnInstance )( 
            IWbemObjectAccess * This,
            /* [in] */ long lFlags,
            /* [out] */ IWbemClassObject **ppNewInstance);
        
        DECLSPEC_XFGVIRT(IWbemClassObject, CompareTo)
        HRESULT ( STDMETHODCALLTYPE *CompareTo )( 
            IWbemObjectAccess * This,
            /* [in] */ long lFlags,
            /* [in] */ IWbemClassObject *pCompareTo);
        
        DECLSPEC_XFGVIRT(IWbemClassObject, GetPropertyOrigin)
        HRESULT ( STDMETHODCALLTYPE *GetPropertyOrigin )( 
            IWbemObjectAccess * This,
            /* [string][in] */ LPCWSTR wszName,
            /* [out] */ BSTR *pstrClassName);
        
        DECLSPEC_XFGVIRT(IWbemClassObject, InheritsFrom)
        HRESULT ( STDMETHODCALLTYPE *InheritsFrom )( 
            IWbemObjectAccess * This,
            /* [in] */ LPCWSTR strAncestor);
        
        DECLSPEC_XFGVIRT(IWbemClassObject, GetMethod)
        HRESULT ( STDMETHODCALLTYPE *GetMethod )( 
            IWbemObjectAccess * This,
            /* [string][in] */ LPCWSTR wszName,
            /* [in] */ long lFlags,
            /* [out] */ IWbemClassObject **ppInSignature,
            /* [out] */ IWbemClassObject **ppOutSignature);
        
        DECLSPEC_XFGVIRT(IWbemClassObject, PutMethod)
        HRESULT ( STDMETHODCALLTYPE *PutMethod )( 
            IWbemObjectAccess * This,
            /* [string][in] */ LPCWSTR wszName,
            /* [in] */ long lFlags,
            /* [in] */ IWbemClassObject *pInSignature,
            /* [in] */ IWbemClassObject *pOutSignature);
        
        DECLSPEC_XFGVIRT(IWbemClassObject, DeleteMethod)
        HRESULT ( STDMETHODCALLTYPE *DeleteMethod )( 
            IWbemObjectAccess * This,
            /* [string][in] */ LPCWSTR wszName);
        
        DECLSPEC_XFGVIRT(IWbemClassObject, BeginMethodEnumeration)
        HRESULT ( STDMETHODCALLTYPE *BeginMethodEnumeration )( 
            IWbemObjectAccess * This,
            /* [in] */ long lEnumFlags);
        
        DECLSPEC_XFGVIRT(IWbemClassObject, NextMethod)
        HRESULT ( STDMETHODCALLTYPE *NextMethod )( 
            IWbemObjectAccess * This,
            /* [in] */ long lFlags,
            /* [unique][in][out] */ BSTR *pstrName,
            /* [unique][in][out] */ IWbemClassObject **ppInSignature,
            /* [unique][in][out] */ IWbemClassObject **ppOutSignature);
        
        DECLSPEC_XFGVIRT(IWbemClassObject, EndMethodEnumeration)
        HRESULT ( STDMETHODCALLTYPE *EndMethodEnumeration )( 
            IWbemObjectAccess * This);
        
        DECLSPEC_XFGVIRT(IWbemClassObject, GetMethodQualifierSet)
        HRESULT ( STDMETHODCALLTYPE *GetMethodQualifierSet )( 
            IWbemObjectAccess * This,
            /* [string][in] */ LPCWSTR wszMethod,
            /* [out] */ IWbemQualifierSet **ppQualSet);
        
        DECLSPEC_XFGVIRT(IWbemClassObject, GetMethodOrigin)
        HRESULT ( STDMETHODCALLTYPE *GetMethodOrigin )( 
            IWbemObjectAccess * This,
            /* [string][in] */ LPCWSTR wszMethodName,
            /* [out] */ BSTR *pstrClassName);
        
        DECLSPEC_XFGVIRT(IWbemObjectAccess, GetPropertyHandle)
        HRESULT ( STDMETHODCALLTYPE *GetPropertyHandle )( 
            IWbemObjectAccess * This,
            /* [string][in] */ LPCWSTR wszPropertyName,
            /* [out] */ CIMTYPE *pType,
            /* [out] */ long *plHandle);
        
        DECLSPEC_XFGVIRT(IWbemObjectAccess, WritePropertyValue)
        HRESULT ( STDMETHODCALLTYPE *WritePropertyValue )( 
            IWbemObjectAccess * This,
            /* [in] */ long lHandle,
            /* [in] */ long lNumBytes,
            /* [size_is][in] */ const byte *aData);
        
        DECLSPEC_XFGVIRT(IWbemObjectAccess, ReadPropertyValue)
        HRESULT ( STDMETHODCALLTYPE *ReadPropertyValue )( 
            IWbemObjectAccess * This,
            /* [in] */ long lHandle,
            /* [in] */ long lBufferSize,
            /* [out] */ long *plNumBytes,
            /* [length_is][size_is][out] */ byte *aData);
        
        DECLSPEC_XFGVIRT(IWbemObjectAccess, ReadDWORD)
        HRESULT ( STDMETHODCALLTYPE *ReadDWORD )( 
            IWbemObjectAccess * This,
            /* [in] */ long lHandle,
            /* [out] */ DWORD *pdw);
        
        DECLSPEC_XFGVIRT(IWbemObjectAccess, WriteDWORD)
        HRESULT ( STDMETHODCALLTYPE *WriteDWORD )( 
            IWbemObjectAccess * This,
            /* [in] */ long lHandle,
            /* [in] */ DWORD dw);
        
        DECLSPEC_XFGVIRT(IWbemObjectAccess, ReadQWORD)
        HRESULT ( STDMETHODCALLTYPE *ReadQWORD )( 
            IWbemObjectAccess * This,
            /* [in] */ long lHandle,
            /* [out] */ unsigned __int64 *pqw);
        
        DECLSPEC_XFGVIRT(IWbemObjectAccess, WriteQWORD)
        HRESULT ( STDMETHODCALLTYPE *WriteQWORD )( 
            IWbemObjectAccess * This,
            /* [in] */ long lHandle,
            /* [in] */ unsigned __int64 pw);
        
        DECLSPEC_XFGVIRT(IWbemObjectAccess, GetPropertyInfoByHandle)
        HRESULT ( STDMETHODCALLTYPE *GetPropertyInfoByHandle )( 
            IWbemObjectAccess * This,
            /* [in] */ long lHandle,
            /* [out] */ BSTR *pstrName,
            /* [out] */ CIMTYPE *pType);
        
        DECLSPEC_XFGVIRT(IWbemObjectAccess, Lock)
        HRESULT ( STDMETHODCALLTYPE *Lock )( 
            IWbemObjectAccess * This,
            /* [in] */ long lFlags);
        
        DECLSPEC_XFGVIRT(IWbemObjectAccess, Unlock)
        HRESULT ( STDMETHODCALLTYPE *Unlock )( 
            IWbemObjectAccess * This,
            /* [in] */ long lFlags);
        
        END_INTERFACE
    } IWbemObjectAccessVtbl;

    interface IWbemObjectAccess
    {
        CONST_VTBL struct IWbemObjectAccessVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWbemObjectAccess_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWbemObjectAccess_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWbemObjectAccess_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWbemObjectAccess_GetQualifierSet(This,ppQualSet)	\
    ( (This)->lpVtbl -> GetQualifierSet(This,ppQualSet) ) 

#define IWbemObjectAccess_Get(This,wszName,lFlags,pVal,pType,plFlavor)	\
    ( (This)->lpVtbl -> Get(This,wszName,lFlags,pVal,pType,plFlavor) ) 

#define IWbemObjectAccess_Put(This,wszName,lFlags,pVal,Type)	\
    ( (This)->lpVtbl -> Put(This,wszName,lFlags,pVal,Type) ) 

#define IWbemObjectAccess_Delete(This,wszName)	\
    ( (This)->lpVtbl -> Delete(This,wszName) ) 

#define IWbemObjectAccess_GetNames(This,wszQualifierName,lFlags,pQualifierVal,pNames)	\
    ( (This)->lpVtbl -> GetNames(This,wszQualifierName,lFlags,pQualifierVal,pNames) ) 

#define IWbemObjectAccess_BeginEnumeration(This,lEnumFlags)	\
    ( (This)->lpVtbl -> BeginEnumeration(This,lEnumFlags) ) 

#define IWbemObjectAccess_Next(This,lFlags,strName,pVal,pType,plFlavor)	\
    ( (This)->lpVtbl -> Next(This,lFlags,strName,pVal,pType,plFlavor) ) 

#define IWbemObjectAccess_EndEnumeration(This)	\
    ( (This)->lpVtbl -> EndEnumeration(This) ) 

#define IWbemObjectAccess_GetPropertyQualifierSet(This,wszProperty,ppQualSet)	\
    ( (This)->lpVtbl -> GetPropertyQualifierSet(This,wszProperty,ppQualSet) ) 

#define IWbemObjectAccess_Clone(This,ppCopy)	\
    ( (This)->lpVtbl -> Clone(This,ppCopy) ) 

#define IWbemObjectAccess_GetObjectText(This,lFlags,pstrObjectText)	\
    ( (This)->lpVtbl -> GetObjectText(This,lFlags,pstrObjectText) ) 

#define IWbemObjectAccess_SpawnDerivedClass(This,lFlags,ppNewClass)	\
    ( (This)->lpVtbl -> SpawnDerivedClass(This,lFlags,ppNewClass) ) 

#define IWbemObjectAccess_SpawnInstance(This,lFlags,ppNewInstance)	\
    ( (This)->lpVtbl -> SpawnInstance(This,lFlags,ppNewInstance) ) 

#define IWbemObjectAccess_CompareTo(This,lFlags,pCompareTo)	\
    ( (This)->lpVtbl -> CompareTo(This,lFlags,pCompareTo) ) 

#define IWbemObjectAccess_GetPropertyOrigin(This,wszName,pstrClassName)	\
    ( (This)->lpVtbl -> GetPropertyOrigin(This,wszName,pstrClassName) ) 

#define IWbemObjectAccess_InheritsFrom(This,strAncestor)	\
    ( (This)->lpVtbl -> InheritsFrom(This,strAncestor) ) 

#define IWbemObjectAccess_GetMethod(This,wszName,lFlags,ppInSignature,ppOutSignature)	\
    ( (This)->lpVtbl -> GetMethod(This,wszName,lFlags,ppInSignature,ppOutSignature) ) 

#define IWbemObjectAccess_PutMethod(This,wszName,lFlags,pInSignature,pOutSignature)	\
    ( (This)->lpVtbl -> PutMethod(This,wszName,lFlags,pInSignature,pOutSignature) ) 

#define IWbemObjectAccess_DeleteMethod(This,wszName)	\
    ( (This)->lpVtbl -> DeleteMethod(This,wszName) ) 

#define IWbemObjectAccess_BeginMethodEnumeration(This,lEnumFlags)	\
    ( (This)->lpVtbl -> BeginMethodEnumeration(This,lEnumFlags) ) 

#define IWbemObjectAccess_NextMethod(This,lFlags,pstrName,ppInSignature,ppOutSignature)	\
    ( (This)->lpVtbl -> NextMethod(This,lFlags,pstrName,ppInSignature,ppOutSignature) ) 

#define IWbemObjectAccess_EndMethodEnumeration(This)	\
    ( (This)->lpVtbl -> EndMethodEnumeration(This) ) 

#define IWbemObjectAccess_GetMethodQualifierSet(This,wszMethod,ppQualSet)	\
    ( (This)->lpVtbl -> GetMethodQualifierSet(This,wszMethod,ppQualSet) ) 

#define IWbemObjectAccess_GetMethodOrigin(This,wszMethodName,pstrClassName)	\
    ( (This)->lpVtbl -> GetMethodOrigin(This,wszMethodName,pstrClassName) ) 


#define IWbemObjectAccess_GetPropertyHandle(This,wszPropertyName,pType,plHandle)	\
    ( (This)->lpVtbl -> GetPropertyHandle(This,wszPropertyName,pType,plHandle) ) 

#define IWbemObjectAccess_WritePropertyValue(This,lHandle,lNumBytes,aData)	\
    ( (This)->lpVtbl -> WritePropertyValue(This,lHandle,lNumBytes,aData) ) 

#define IWbemObjectAccess_ReadPropertyValue(This,lHandle,lBufferSize,plNumBytes,aData)	\
    ( (This)->lpVtbl -> ReadPropertyValue(This,lHandle,lBufferSize,plNumBytes,aData) ) 

#define IWbemObjectAccess_ReadDWORD(This,lHandle,pdw)	\
    ( (This)->lpVtbl -> ReadDWORD(This,lHandle,pdw) ) 

#define IWbemObjectAccess_WriteDWORD(This,lHandle,dw)	\
    ( (This)->lpVtbl -> WriteDWORD(This,lHandle,dw) ) 

#define IWbemObjectAccess_ReadQWORD(This,lHandle,pqw)	\
    ( (This)->lpVtbl -> ReadQWORD(This,lHandle,pqw) ) 

#define IWbemObjectAccess_WriteQWORD(This,lHandle,pw)	\
    ( (This)->lpVtbl -> WriteQWORD(This,lHandle,pw) ) 

#define IWbemObjectAccess_GetPropertyInfoByHandle(This,lHandle,pstrName,pType)	\
    ( (This)->lpVtbl -> GetPropertyInfoByHandle(This,lHandle,pstrName,pType) ) 

#define IWbemObjectAccess_Lock(This,lFlags)	\
    ( (This)->lpVtbl -> Lock(This,lFlags) ) 

#define IWbemObjectAccess_Unlock(This,lFlags)	\
    ( (This)->lpVtbl -> Unlock(This,lFlags) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWbemObjectAccess_INTERFACE_DEFINED__ */


#ifndef __IWbemQualifierSet_INTERFACE_DEFINED__
#define __IWbemQualifierSet_INTERFACE_DEFINED__

/* interface IWbemQualifierSet */
/* [uuid][local][restricted][object] */ 


EXTERN_C const IID IID_IWbemQualifierSet;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("dc12a680-737f-11cf-884d-00aa004b2e24")
    IWbemQualifierSet : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Get( 
            /* [string][in] */ LPCWSTR wszName,
            /* [in] */ long lFlags,
            /* [unique][in][out] */ VARIANT *pVal,
            /* [unique][in][out] */ long *plFlavor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Put( 
            /* [string][in] */ LPCWSTR wszName,
            /* [in] */ VARIANT *pVal,
            /* [in] */ long lFlavor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Delete( 
            /* [string][in] */ LPCWSTR wszName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNames( 
            /* [in] */ long lFlags,
            /* [out] */ SAFEARRAY * *pNames) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE BeginEnumeration( 
            /* [in] */ long lFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ long lFlags,
            /* [unique][in][out] */ BSTR *pstrName,
            /* [unique][in][out] */ VARIANT *pVal,
            /* [unique][in][out] */ long *plFlavor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EndEnumeration( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWbemQualifierSetVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWbemQualifierSet * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWbemQualifierSet * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWbemQualifierSet * This);
        
        DECLSPEC_XFGVIRT(IWbemQualifierSet, Get)
        HRESULT ( STDMETHODCALLTYPE *Get )( 
            IWbemQualifierSet * This,
            /* [string][in] */ LPCWSTR wszName,
            /* [in] */ long lFlags,
            /* [unique][in][out] */ VARIANT *pVal,
            /* [unique][in][out] */ long *plFlavor);
        
        DECLSPEC_XFGVIRT(IWbemQualifierSet, Put)
        HRESULT ( STDMETHODCALLTYPE *Put )( 
            IWbemQualifierSet * This,
            /* [string][in] */ LPCWSTR wszName,
            /* [in] */ VARIANT *pVal,
            /* [in] */ long lFlavor);
        
        DECLSPEC_XFGVIRT(IWbemQualifierSet, Delete)
        HRESULT ( STDMETHODCALLTYPE *Delete )( 
            IWbemQualifierSet * This,
            /* [string][in] */ LPCWSTR wszName);
        
        DECLSPEC_XFGVIRT(IWbemQualifierSet, GetNames)
        HRESULT ( STDMETHODCALLTYPE *GetNames )( 
            IWbemQualifierSet * This,
            /* [in] */ long lFlags,
            /* [out] */ SAFEARRAY * *pNames);
        
        DECLSPEC_XFGVIRT(IWbemQualifierSet, BeginEnumeration)
        HRESULT ( STDMETHODCALLTYPE *BeginEnumeration )( 
            IWbemQualifierSet * This,
            /* [in] */ long lFlags);
        
        DECLSPEC_XFGVIRT(IWbemQualifierSet, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            IWbemQualifierSet * This,
            /* [in] */ long lFlags,
            /* [unique][in][out] */ BSTR *pstrName,
            /* [unique][in][out] */ VARIANT *pVal,
            /* [unique][in][out] */ long *plFlavor);
        
        DECLSPEC_XFGVIRT(IWbemQualifierSet, EndEnumeration)
        HRESULT ( STDMETHODCALLTYPE *EndEnumeration )( 
            IWbemQualifierSet * This);
        
        END_INTERFACE
    } IWbemQualifierSetVtbl;

    interface IWbemQualifierSet
    {
        CONST_VTBL struct IWbemQualifierSetVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWbemQualifierSet_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWbemQualifierSet_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWbemQualifierSet_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWbemQualifierSet_Get(This,wszName,lFlags,pVal,plFlavor)	\
    ( (This)->lpVtbl -> Get(This,wszName,lFlags,pVal,plFlavor) ) 

#define IWbemQualifierSet_Put(This,wszName,pVal,lFlavor)	\
    ( (This)->lpVtbl -> Put(This,wszName,pVal,lFlavor) ) 

#define IWbemQualifierSet_Delete(This,wszName)	\
    ( (This)->lpVtbl -> Delete(This,wszName) ) 

#define IWbemQualifierSet_GetNames(This,lFlags,pNames)	\
    ( (This)->lpVtbl -> GetNames(This,lFlags,pNames) ) 

#define IWbemQualifierSet_BeginEnumeration(This,lFlags)	\
    ( (This)->lpVtbl -> BeginEnumeration(This,lFlags) ) 

#define IWbemQualifierSet_Next(This,lFlags,pstrName,pVal,plFlavor)	\
    ( (This)->lpVtbl -> Next(This,lFlags,pstrName,pVal,plFlavor) ) 

#define IWbemQualifierSet_EndEnumeration(This)	\
    ( (This)->lpVtbl -> EndEnumeration(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWbemQualifierSet_INTERFACE_DEFINED__ */


#ifndef __IWbemServices_INTERFACE_DEFINED__
#define __IWbemServices_INTERFACE_DEFINED__

/* interface IWbemServices */
/* [unique][uuid][restricted][object] */ 


EXTERN_C const IID IID_IWbemServices;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9556dc99-828c-11cf-a37e-00aa003240c7")
    IWbemServices : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OpenNamespace( 
            /* [in] */ __RPC__in const BSTR strNamespace,
            /* [in] */ long lFlags,
            /* [in] */ __RPC__in_opt IWbemContext *pCtx,
            /* [unique][in][out] */ __RPC__deref_opt_inout_opt IWbemServices **ppWorkingNamespace,
            /* [unique][in][out] */ __RPC__deref_opt_inout_opt IWbemCallResult **ppResult) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CancelAsyncCall( 
            /* [in] */ __RPC__in_opt IWbemObjectSink *pSink) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE QueryObjectSink( 
            /* [in] */ long lFlags,
            /* [out] */ __RPC__deref_out_opt IWbemObjectSink **ppResponseHandler) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetObject( 
            /* [in] */ __RPC__in const BSTR strObjectPath,
            /* [in] */ long lFlags,
            /* [in] */ __RPC__in_opt IWbemContext *pCtx,
            /* [unique][in][out] */ __RPC__deref_opt_inout_opt IWbemClassObject **ppObject,
            /* [unique][in][out] */ __RPC__deref_opt_inout_opt IWbemCallResult **ppCallResult) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetObjectAsync( 
            /* [in] */ __RPC__in const BSTR strObjectPath,
            /* [in] */ long lFlags,
            /* [in] */ __RPC__in_opt IWbemContext *pCtx,
            /* [in] */ __RPC__in_opt IWbemObjectSink *pResponseHandler) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE PutClass( 
            /* [in] */ __RPC__in_opt IWbemClassObject *pObject,
            /* [in] */ long lFlags,
            /* [in] */ __RPC__in_opt IWbemContext *pCtx,
            /* [unique][in][out] */ __RPC__deref_opt_inout_opt IWbemCallResult **ppCallResult) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE PutClassAsync( 
            /* [in] */ __RPC__in_opt IWbemClassObject *pObject,
            /* [in] */ long lFlags,
            /* [in] */ __RPC__in_opt IWbemContext *pCtx,
            /* [in] */ __RPC__in_opt IWbemObjectSink *pResponseHandler) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteClass( 
            /* [in] */ __RPC__in const BSTR strClass,
            /* [in] */ long lFlags,
            /* [in] */ __RPC__in_opt IWbemContext *pCtx,
            /* [unique][in][out] */ __RPC__deref_opt_inout_opt IWbemCallResult **ppCallResult) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteClassAsync( 
            /* [in] */ __RPC__in const BSTR strClass,
            /* [in] */ long lFlags,
            /* [in] */ __RPC__in_opt IWbemContext *pCtx,
            /* [in] */ __RPC__in_opt IWbemObjectSink *pResponseHandler) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateClassEnum( 
            /* [in] */ __RPC__in const BSTR strSuperclass,
            /* [in] */ long lFlags,
            /* [in] */ __RPC__in_opt IWbemContext *pCtx,
            /* [out] */ __RPC__deref_out_opt IEnumWbemClassObject **ppEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateClassEnumAsync( 
            /* [in] */ __RPC__in const BSTR strSuperclass,
            /* [in] */ long lFlags,
            /* [in] */ __RPC__in_opt IWbemContext *pCtx,
            /* [in] */ __RPC__in_opt IWbemObjectSink *pResponseHandler) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE PutInstance( 
            /* [in] */ __RPC__in_opt IWbemClassObject *pInst,
            /* [in] */ long lFlags,
            /* [in] */ __RPC__in_opt IWbemContext *pCtx,
            /* [unique][in][out] */ __RPC__deref_opt_inout_opt IWbemCallResult **ppCallResult) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE PutInstanceAsync( 
            /* [in] */ __RPC__in_opt IWbemClassObject *pInst,
            /* [in] */ long lFlags,
            /* [in] */ __RPC__in_opt IWbemContext *pCtx,
            /* [in] */ __RPC__in_opt IWbemObjectSink *pResponseHandler) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteInstance( 
            /* [in] */ __RPC__in const BSTR strObjectPath,
            /* [in] */ long lFlags,
            /* [in] */ __RPC__in_opt IWbemContext *pCtx,
            /* [unique][in][out] */ __RPC__deref_opt_inout_opt IWbemCallResult **ppCallResult) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteInstanceAsync( 
            /* [in] */ __RPC__in const BSTR strObjectPath,
            /* [in] */ long lFlags,
            /* [in] */ __RPC__in_opt IWbemContext *pCtx,
            /* [in] */ __RPC__in_opt IWbemObjectSink *pResponseHandler) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateInstanceEnum( 
            /* [in] */ __RPC__in const BSTR strFilter,
            /* [in] */ long lFlags,
            /* [in] */ __RPC__in_opt IWbemContext *pCtx,
            /* [out] */ __RPC__deref_out_opt IEnumWbemClassObject **ppEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateInstanceEnumAsync( 
            /* [in] */ __RPC__in const BSTR strFilter,
            /* [in] */ long lFlags,
            /* [in] */ __RPC__in_opt IWbemContext *pCtx,
            /* [in] */ __RPC__in_opt IWbemObjectSink *pResponseHandler) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ExecQuery( 
            /* [in] */ __RPC__in const BSTR strQueryLanguage,
            /* [in] */ __RPC__in const BSTR strQuery,
            /* [in] */ long lFlags,
            /* [in] */ __RPC__in_opt IWbemContext *pCtx,
            /* [out] */ __RPC__deref_out_opt IEnumWbemClassObject **ppEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ExecQueryAsync( 
            /* [in] */ __RPC__in const BSTR strQueryLanguage,
            /* [in] */ __RPC__in const BSTR strQuery,
            /* [in] */ long lFlags,
            /* [in] */ __RPC__in_opt IWbemContext *pCtx,
            /* [in] */ __RPC__in_opt IWbemObjectSink *pResponseHandler) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ExecNotificationQuery( 
            /* [in] */ __RPC__in const BSTR strQueryLanguage,
            /* [in] */ __RPC__in const BSTR strQuery,
            /* [in] */ long lFlags,
            /* [in] */ __RPC__in_opt IWbemContext *pCtx,
            /* [out] */ __RPC__deref_out_opt IEnumWbemClassObject **ppEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ExecNotificationQueryAsync( 
            /* [in] */ __RPC__in const BSTR strQueryLanguage,
            /* [in] */ __RPC__in const BSTR strQuery,
            /* [in] */ long lFlags,
            /* [in] */ __RPC__in_opt IWbemContext *pCtx,
            /* [in] */ __RPC__in_opt IWbemObjectSink *pResponseHandler) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ExecMethod( 
            /* [in] */ __RPC__in const BSTR strObjectPath,
            /* [in] */ __RPC__in const BSTR strMethodName,
            /* [in] */ long lFlags,
            /* [in] */ __RPC__in_opt IWbemContext *pCtx,
            /* [in] */ __RPC__in_opt IWbemClassObject *pInParams,
            /* [unique][in][out] */ __RPC__deref_opt_inout_opt IWbemClassObject **ppOutParams,
            /* [unique][in][out] */ __RPC__deref_opt_inout_opt IWbemCallResult **ppCallResult) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ExecMethodAsync( 
            /* [in] */ __RPC__in const BSTR strObjectPath,
            /* [in] */ __RPC__in const BSTR strMethodName,
            /* [in] */ long lFlags,
            /* [in] */ __RPC__in_opt IWbemContext *pCtx,
            /* [in] */ __RPC__in_opt IWbemClassObject *pInParams,
            /* [in] */ __RPC__in_opt IWbemObjectSink *pResponseHandler) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWbemServicesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWbemServices * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWbemServices * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWbemServices * This);
        
        DECLSPEC_XFGVIRT(IWbemServices, OpenNamespace)
        HRESULT ( STDMETHODCALLTYPE *OpenNamespace )( 
            __RPC__in IWbemServices * This,
            /* [in] */ __RPC__in const BSTR strNamespace,
            /* [in] */ long lFlags,
            /* [in] */ __RPC__in_opt IWbemContext *pCtx,
            /* [unique][in][out] */ __RPC__deref_opt_inout_opt IWbemServices **ppWorkingNamespace,
            /* [unique][in][out] */ __RPC__deref_opt_inout_opt IWbemCallResult **ppResult);
        
        DECLSPEC_XFGVIRT(IWbemServices, CancelAsyncCall)
        HRESULT ( STDMETHODCALLTYPE *CancelAsyncCall )( 
            __RPC__in IWbemServices * This,
            /* [in] */ __RPC__in_opt IWbemObjectSink *pSink);
        
        DECLSPEC_XFGVIRT(IWbemServices, QueryObjectSink)
        HRESULT ( STDMETHODCALLTYPE *QueryObjectSink )( 
            __RPC__in IWbemServices * This,
            /* [in] */ long lFlags,
            /* [out] */ __RPC__deref_out_opt IWbemObjectSink **ppResponseHandler);
        
        DECLSPEC_XFGVIRT(IWbemServices, GetObject)
        HRESULT ( STDMETHODCALLTYPE *GetObject )( 
            __RPC__in IWbemServices * This,
            /* [in] */ __RPC__in const BSTR strObjectPath,
            /* [in] */ long lFlags,
            /* [in] */ __RPC__in_opt IWbemContext *pCtx,
            /* [unique][in][out] */ __RPC__deref_opt_inout_opt IWbemClassObject **ppObject,
            /* [unique][in][out] */ __RPC__deref_opt_inout_opt IWbemCallResult **ppCallResult);
        
        DECLSPEC_XFGVIRT(IWbemServices, GetObjectAsync)
        HRESULT ( STDMETHODCALLTYPE *GetObjectAsync )( 
            __RPC__in IWbemServices * This,
            /* [in] */ __RPC__in const BSTR strObjectPath,
            /* [in] */ long lFlags,
            /* [in] */ __RPC__in_opt IWbemContext *pCtx,
            /* [in] */ __RPC__in_opt IWbemObjectSink *pResponseHandler);
        
        DECLSPEC_XFGVIRT(IWbemServices, PutClass)
        HRESULT ( STDMETHODCALLTYPE *PutClass )( 
            __RPC__in IWbemServices * This,
            /* [in] */ __RPC__in_opt IWbemClassObject *pObject,
            /* [in] */ long lFlags,
            /* [in] */ __RPC__in_opt IWbemContext *pCtx,
            /* [unique][in][out] */ __RPC__deref_opt_inout_opt IWbemCallResult **ppCallResult);
        
        DECLSPEC_XFGVIRT(IWbemServices, PutClassAsync)
        HRESULT ( STDMETHODCALLTYPE *PutClassAsync )( 
            __RPC__in IWbemServices * This,
            /* [in] */ __RPC__in_opt IWbemClassObject *pObject,
            /* [in] */ long lFlags,
            /* [in] */ __RPC__in_opt IWbemContext *pCtx,
            /* [in] */ __RPC__in_opt IWbemObjectSink *pResponseHandler);
        
        DECLSPEC_XFGVIRT(IWbemServices, DeleteClass)
        HRESULT ( STDMETHODCALLTYPE *DeleteClass )( 
            __RPC__in IWbemServices * This,
            /* [in] */ __RPC__in const BSTR strClass,
            /* [in] */ long lFlags,
            /* [in] */ __RPC__in_opt IWbemContext *pCtx,
            /* [unique][in][out] */ __RPC__deref_opt_inout_opt IWbemCallResult **ppCallResult);
        
        DECLSPEC_XFGVIRT(IWbemServices, DeleteClassAsync)
        HRESULT ( STDMETHODCALLTYPE *DeleteClassAsync )( 
            __RPC__in IWbemServices * This,
            /* [in] */ __RPC__in const BSTR strClass,
            /* [in] */ long lFlags,
            /* [in] */ __RPC__in_opt IWbemContext *pCtx,
            /* [in] */ __RPC__in_opt IWbemObjectSink *pResponseHandler);
        
        DECLSPEC_XFGVIRT(IWbemServices, CreateClassEnum)
        HRESULT ( STDMETHODCALLTYPE *CreateClassEnum )( 
            __RPC__in IWbemServices * This,
            /* [in] */ __RPC__in const BSTR strSuperclass,
            /* [in] */ long lFlags,
            /* [in] */ __RPC__in_opt IWbemContext *pCtx,
            /* [out] */ __RPC__deref_out_opt IEnumWbemClassObject **ppEnum);
        
        DECLSPEC_XFGVIRT(IWbemServices, CreateClassEnumAsync)
        HRESULT ( STDMETHODCALLTYPE *CreateClassEnumAsync )( 
            __RPC__in IWbemServices * This,
            /* [in] */ __RPC__in const BSTR strSuperclass,
            /* [in] */ long lFlags,
            /* [in] */ __RPC__in_opt IWbemContext *pCtx,
            /* [in] */ __RPC__in_opt IWbemObjectSink *pResponseHandler);
        
        DECLSPEC_XFGVIRT(IWbemServices, PutInstance)
        HRESULT ( STDMETHODCALLTYPE *PutInstance )( 
            __RPC__in IWbemServices * This,
            /* [in] */ __RPC__in_opt IWbemClassObject *pInst,
            /* [in] */ long lFlags,
            /* [in] */ __RPC__in_opt IWbemContext *pCtx,
            /* [unique][in][out] */ __RPC__deref_opt_inout_opt IWbemCallResult **ppCallResult);
        
        DECLSPEC_XFGVIRT(IWbemServices, PutInstanceAsync)
        HRESULT ( STDMETHODCALLTYPE *PutInstanceAsync )( 
            __RPC__in IWbemServices * This,
            /* [in] */ __RPC__in_opt IWbemClassObject *pInst,
            /* [in] */ long lFlags,
            /* [in] */ __RPC__in_opt IWbemContext *pCtx,
            /* [in] */ __RPC__in_opt IWbemObjectSink *pResponseHandler);
        
        DECLSPEC_XFGVIRT(IWbemServices, DeleteInstance)
        HRESULT ( STDMETHODCALLTYPE *DeleteInstance )( 
            __RPC__in IWbemServices * This,
            /* [in] */ __RPC__in const BSTR strObjectPath,
            /* [in] */ long lFlags,
            /* [in] */ __RPC__in_opt IWbemContext *pCtx,
            /* [unique][in][out] */ __RPC__deref_opt_inout_opt IWbemCallResult **ppCallResult);
        
        DECLSPEC_XFGVIRT(IWbemServices, DeleteInstanceAsync)
        HRESULT ( STDMETHODCALLTYPE *DeleteInstanceAsync )( 
            __RPC__in IWbemServices * This,
            /* [in] */ __RPC__in const BSTR strObjectPath,
            /* [in] */ long lFlags,
            /* [in] */ __RPC__in_opt IWbemContext *pCtx,
            /* [in] */ __RPC__in_opt IWbemObjectSink *pResponseHandler);
        
        DECLSPEC_XFGVIRT(IWbemServices, CreateInstanceEnum)
        HRESULT ( STDMETHODCALLTYPE *CreateInstanceEnum )( 
            __RPC__in IWbemServices * This,
            /* [in] */ __RPC__in const BSTR strFilter,
            /* [in] */ long lFlags,
            /* [in] */ __RPC__in_opt IWbemContext *pCtx,
            /* [out] */ __RPC__deref_out_opt IEnumWbemClassObject **ppEnum);
        
        DECLSPEC_XFGVIRT(IWbemServices, CreateInstanceEnumAsync)
        HRESULT ( STDMETHODCALLTYPE *CreateInstanceEnumAsync )( 
            __RPC__in IWbemServices * This,
            /* [in] */ __RPC__in const BSTR strFilter,
            /* [in] */ long lFlags,
            /* [in] */ __RPC__in_opt IWbemContext *pCtx,
            /* [in] */ __RPC__in_opt IWbemObjectSink *pResponseHandler);
        
        DECLSPEC_XFGVIRT(IWbemServices, ExecQuery)
        HRESULT ( STDMETHODCALLTYPE *ExecQuery )( 
            __RPC__in IWbemServices * This,
            /* [in] */ __RPC__in const BSTR strQueryLanguage,
            /* [in] */ __RPC__in const BSTR strQuery,
            /* [in] */ long lFlags,
            /* [in] */ __RPC__in_opt IWbemContext *pCtx,
            /* [out] */ __RPC__deref_out_opt IEnumWbemClassObject **ppEnum);
        
        DECLSPEC_XFGVIRT(IWbemServices, ExecQueryAsync)
        HRESULT ( STDMETHODCALLTYPE *ExecQueryAsync )( 
            __RPC__in IWbemServices * This,
            /* [in] */ __RPC__in const BSTR strQueryLanguage,
            /* [in] */ __RPC__in const BSTR strQuery,
            /* [in] */ long lFlags,
            /* [in] */ __RPC__in_opt IWbemContext *pCtx,
            /* [in] */ __RPC__in_opt IWbemObjectSink *pResponseHandler);
        
        DECLSPEC_XFGVIRT(IWbemServices, ExecNotificationQuery)
        HRESULT ( STDMETHODCALLTYPE *ExecNotificationQuery )( 
            __RPC__in IWbemServices * This,
            /* [in] */ __RPC__in const BSTR strQueryLanguage,
            /* [in] */ __RPC__in const BSTR strQuery,
            /* [in] */ long lFlags,
            /* [in] */ __RPC__in_opt IWbemContext *pCtx,
            /* [out] */ __RPC__deref_out_opt IEnumWbemClassObject **ppEnum);
        
        DECLSPEC_XFGVIRT(IWbemServices, ExecNotificationQueryAsync)
        HRESULT ( STDMETHODCALLTYPE *ExecNotificationQueryAsync )( 
            __RPC__in IWbemServices * This,
            /* [in] */ __RPC__in const BSTR strQueryLanguage,
            /* [in] */ __RPC__in const BSTR strQuery,
            /* [in] */ long lFlags,
            /* [in] */ __RPC__in_opt IWbemContext *pCtx,
            /* [in] */ __RPC__in_opt IWbemObjectSink *pResponseHandler);
        
        DECLSPEC_XFGVIRT(IWbemServices, ExecMethod)
        HRESULT ( STDMETHODCALLTYPE *ExecMethod )( 
            __RPC__in IWbemServices * This,
            /* [in] */ __RPC__in const BSTR strObjectPath,
            /* [in] */ __RPC__in const BSTR strMethodName,
            /* [in] */ long lFlags,
            /* [in] */ __RPC__in_opt IWbemContext *pCtx,
            /* [in] */ __RPC__in_opt IWbemClassObject *pInParams,
            /* [unique][in][out] */ __RPC__deref_opt_inout_opt IWbemClassObject **ppOutParams,
            /* [unique][in][out] */ __RPC__deref_opt_inout_opt IWbemCallResult **ppCallResult);
        
        DECLSPEC_XFGVIRT(IWbemServices, ExecMethodAsync)
        HRESULT ( STDMETHODCALLTYPE *ExecMethodAsync )( 
            __RPC__in IWbemServices * This,
            /* [in] */ __RPC__in const BSTR strObjectPath,
            /* [in] */ __RPC__in const BSTR strMethodName,
            /* [in] */ long lFlags,
            /* [in] */ __RPC__in_opt IWbemContext *pCtx,
            /* [in] */ __RPC__in_opt IWbemClassObject *pInParams,
            /* [in] */ __RPC__in_opt IWbemObjectSink *pResponseHandler);
        
        END_INTERFACE
    } IWbemServicesVtbl;

    interface IWbemServices
    {
        CONST_VTBL struct IWbemServicesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWbemServices_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWbemServices_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWbemServices_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWbemServices_OpenNamespace(This,strNamespace,lFlags,pCtx,ppWorkingNamespace,ppResult)	\
    ( (This)->lpVtbl -> OpenNamespace(This,strNamespace,lFlags,pCtx,ppWorkingNamespace,ppResult) ) 

#define IWbemServices_CancelAsyncCall(This,pSink)	\
    ( (This)->lpVtbl -> CancelAsyncCall(This,pSink) ) 

#define IWbemServices_QueryObjectSink(This,lFlags,ppResponseHandler)	\
    ( (This)->lpVtbl -> QueryObjectSink(This,lFlags,ppResponseHandler) ) 

#define IWbemServices_GetObject(This,strObjectPath,lFlags,pCtx,ppObject,ppCallResult)	\
    ( (This)->lpVtbl -> GetObject(This,strObjectPath,lFlags,pCtx,ppObject,ppCallResult) ) 

#define IWbemServices_GetObjectAsync(This,strObjectPath,lFlags,pCtx,pResponseHandler)	\
    ( (This)->lpVtbl -> GetObjectAsync(This,strObjectPath,lFlags,pCtx,pResponseHandler) ) 

#define IWbemServices_PutClass(This,pObject,lFlags,pCtx,ppCallResult)	\
    ( (This)->lpVtbl -> PutClass(This,pObject,lFlags,pCtx,ppCallResult) ) 

#define IWbemServices_PutClassAsync(This,pObject,lFlags,pCtx,pResponseHandler)	\
    ( (This)->lpVtbl -> PutClassAsync(This,pObject,lFlags,pCtx,pResponseHandler) ) 

#define IWbemServices_DeleteClass(This,strClass,lFlags,pCtx,ppCallResult)	\
    ( (This)->lpVtbl -> DeleteClass(This,strClass,lFlags,pCtx,ppCallResult) ) 

#define IWbemServices_DeleteClassAsync(This,strClass,lFlags,pCtx,pResponseHandler)	\
    ( (This)->lpVtbl -> DeleteClassAsync(This,strClass,lFlags,pCtx,pResponseHandler) ) 

#define IWbemServices_CreateClassEnum(This,strSuperclass,lFlags,pCtx,ppEnum)	\
    ( (This)->lpVtbl -> CreateClassEnum(This,strSuperclass,lFlags,pCtx,ppEnum) ) 

#define IWbemServices_CreateClassEnumAsync(This,strSuperclass,lFlags,pCtx,pResponseHandler)	\
    ( (This)->lpVtbl -> CreateClassEnumAsync(This,strSuperclass,lFlags,pCtx,pResponseHandler) ) 

#define IWbemServices_PutInstance(This,pInst,lFlags,pCtx,ppCallResult)	\
    ( (This)->lpVtbl -> PutInstance(This,pInst,lFlags,pCtx,ppCallResult) ) 

#define IWbemServices_PutInstanceAsync(This,pInst,lFlags,pCtx,pResponseHandler)	\
    ( (This)->lpVtbl -> PutInstanceAsync(This,pInst,lFlags,pCtx,pResponseHandler) ) 

#define IWbemServices_DeleteInstance(This,strObjectPath,lFlags,pCtx,ppCallResult)	\
    ( (This)->lpVtbl -> DeleteInstance(This,strObjectPath,lFlags,pCtx,ppCallResult) ) 

#define IWbemServices_DeleteInstanceAsync(This,strObjectPath,lFlags,pCtx,pResponseHandler)	\
    ( (This)->lpVtbl -> DeleteInstanceAsync(This,strObjectPath,lFlags,pCtx,pResponseHandler) ) 

#define IWbemServices_CreateInstanceEnum(This,strFilter,lFlags,pCtx,ppEnum)	\
    ( (This)->lpVtbl -> CreateInstanceEnum(This,strFilter,lFlags,pCtx,ppEnum) ) 

#define IWbemServices_CreateInstanceEnumAsync(This,strFilter,lFlags,pCtx,pResponseHandler)	\
    ( (This)->lpVtbl -> CreateInstanceEnumAsync(This,strFilter,lFlags,pCtx,pResponseHandler) ) 

#define IWbemServices_ExecQuery(This,strQueryLanguage,strQuery,lFlags,pCtx,ppEnum)	\
    ( (This)->lpVtbl -> ExecQuery(This,strQueryLanguage,strQuery,lFlags,pCtx,ppEnum) ) 

#define IWbemServices_ExecQueryAsync(This,strQueryLanguage,strQuery,lFlags,pCtx,pResponseHandler)	\
    ( (This)->lpVtbl -> ExecQueryAsync(This,strQueryLanguage,strQuery,lFlags,pCtx,pResponseHandler) ) 

#define IWbemServices_ExecNotificationQuery(This,strQueryLanguage,strQuery,lFlags,pCtx,ppEnum)	\
    ( (This)->lpVtbl -> ExecNotificationQuery(This,strQueryLanguage,strQuery,lFlags,pCtx,ppEnum) ) 

#define IWbemServices_ExecNotificationQueryAsync(This,strQueryLanguage,strQuery,lFlags,pCtx,pResponseHandler)	\
    ( (This)->lpVtbl -> ExecNotificationQueryAsync(This,strQueryLanguage,strQuery,lFlags,pCtx,pResponseHandler) ) 

#define IWbemServices_ExecMethod(This,strObjectPath,strMethodName,lFlags,pCtx,pInParams,ppOutParams,ppCallResult)	\
    ( (This)->lpVtbl -> ExecMethod(This,strObjectPath,strMethodName,lFlags,pCtx,pInParams,ppOutParams,ppCallResult) ) 

#define IWbemServices_ExecMethodAsync(This,strObjectPath,strMethodName,lFlags,pCtx,pInParams,pResponseHandler)	\
    ( (This)->lpVtbl -> ExecMethodAsync(This,strObjectPath,strMethodName,lFlags,pCtx,pInParams,pResponseHandler) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWbemServices_INTERFACE_DEFINED__ */


#ifndef __IWbemLocator_INTERFACE_DEFINED__
#define __IWbemLocator_INTERFACE_DEFINED__

/* interface IWbemLocator */
/* [unique][uuid][local][restricted][object] */ 


EXTERN_C const IID IID_IWbemLocator;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("dc12a687-737f-11cf-884d-00aa004b2e24")
    IWbemLocator : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ConnectServer( 
            /* [in] */ const BSTR strNetworkResource,
            /* [in] */ const BSTR strUser,
            /* [in] */ const BSTR strPassword,
            /* [in] */ const BSTR strLocale,
            /* [in] */ long lSecurityFlags,
            /* [in] */ const BSTR strAuthority,
            /* [in] */ IWbemContext *pCtx,
            /* [out] */ IWbemServices **ppNamespace) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWbemLocatorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWbemLocator * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWbemLocator * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWbemLocator * This);
        
        DECLSPEC_XFGVIRT(IWbemLocator, ConnectServer)
        HRESULT ( STDMETHODCALLTYPE *ConnectServer )( 
            IWbemLocator * This,
            /* [in] */ const BSTR strNetworkResource,
            /* [in] */ const BSTR strUser,
            /* [in] */ const BSTR strPassword,
            /* [in] */ const BSTR strLocale,
            /* [in] */ long lSecurityFlags,
            /* [in] */ const BSTR strAuthority,
            /* [in] */ IWbemContext *pCtx,
            /* [out] */ IWbemServices **ppNamespace);
        
        END_INTERFACE
    } IWbemLocatorVtbl;

    interface IWbemLocator
    {
        CONST_VTBL struct IWbemLocatorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWbemLocator_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWbemLocator_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWbemLocator_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWbemLocator_ConnectServer(This,strNetworkResource,strUser,strPassword,strLocale,lSecurityFlags,strAuthority,pCtx,ppNamespace)	\
    ( (This)->lpVtbl -> ConnectServer(This,strNetworkResource,strUser,strPassword,strLocale,lSecurityFlags,strAuthority,pCtx,ppNamespace) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWbemLocator_INTERFACE_DEFINED__ */


#ifndef __IWbemObjectSink_INTERFACE_DEFINED__
#define __IWbemObjectSink_INTERFACE_DEFINED__

/* interface IWbemObjectSink */
/* [uuid][restricted][object] */ 


EXTERN_C const IID IID_IWbemObjectSink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7c857801-7381-11cf-884d-00aa004b2e24")
    IWbemObjectSink : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Indicate( 
            /* [in] */ long lObjectCount,
            /* [size_is][in] */ __RPC__in_ecount_full(lObjectCount) IWbemClassObject **apObjArray) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetStatus( 
            /* [in] */ long lFlags,
            /* [in] */ HRESULT hResult,
            /* [unique][in] */ __RPC__in_opt BSTR strParam,
            /* [unique][in] */ __RPC__in_opt IWbemClassObject *pObjParam) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWbemObjectSinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWbemObjectSink * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWbemObjectSink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWbemObjectSink * This);
        
        DECLSPEC_XFGVIRT(IWbemObjectSink, Indicate)
        HRESULT ( STDMETHODCALLTYPE *Indicate )( 
            __RPC__in IWbemObjectSink * This,
            /* [in] */ long lObjectCount,
            /* [size_is][in] */ __RPC__in_ecount_full(lObjectCount) IWbemClassObject **apObjArray);
        
        DECLSPEC_XFGVIRT(IWbemObjectSink, SetStatus)
        HRESULT ( STDMETHODCALLTYPE *SetStatus )( 
            __RPC__in IWbemObjectSink * This,
            /* [in] */ long lFlags,
            /* [in] */ HRESULT hResult,
            /* [unique][in] */ __RPC__in_opt BSTR strParam,
            /* [unique][in] */ __RPC__in_opt IWbemClassObject *pObjParam);
        
        END_INTERFACE
    } IWbemObjectSinkVtbl;

    interface IWbemObjectSink
    {
        CONST_VTBL struct IWbemObjectSinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWbemObjectSink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWbemObjectSink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWbemObjectSink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWbemObjectSink_Indicate(This,lObjectCount,apObjArray)	\
    ( (This)->lpVtbl -> Indicate(This,lObjectCount,apObjArray) ) 

#define IWbemObjectSink_SetStatus(This,lFlags,hResult,strParam,pObjParam)	\
    ( (This)->lpVtbl -> SetStatus(This,lFlags,hResult,strParam,pObjParam) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWbemObjectSink_INTERFACE_DEFINED__ */


#ifndef __IEnumWbemClassObject_INTERFACE_DEFINED__
#define __IEnumWbemClassObject_INTERFACE_DEFINED__

/* interface IEnumWbemClassObject */
/* [uuid][restricted][object] */ 


EXTERN_C const IID IID_IEnumWbemClassObject;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("027947e1-d731-11ce-a357-000000000001")
    IEnumWbemClassObject : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ long lTimeout,
            /* [in] */ ULONG uCount,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(uCount, *puReturned) IWbemClassObject **apObjects,
            /* [out] */ __RPC__out ULONG *puReturned) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE NextAsync( 
            /* [in] */ ULONG uCount,
            /* [in] */ __RPC__in_opt IWbemObjectSink *pSink) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ __RPC__deref_out_opt IEnumWbemClassObject **ppEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ long lTimeout,
            /* [in] */ ULONG nCount) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumWbemClassObjectVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumWbemClassObject * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumWbemClassObject * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumWbemClassObject * This);
        
        DECLSPEC_XFGVIRT(IEnumWbemClassObject, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumWbemClassObject * This);
        
        DECLSPEC_XFGVIRT(IEnumWbemClassObject, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IEnumWbemClassObject * This,
            /* [in] */ long lTimeout,
            /* [in] */ ULONG uCount,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(uCount, *puReturned) IWbemClassObject **apObjects,
            /* [out] */ __RPC__out ULONG *puReturned);
        
        DECLSPEC_XFGVIRT(IEnumWbemClassObject, NextAsync)
        HRESULT ( STDMETHODCALLTYPE *NextAsync )( 
            __RPC__in IEnumWbemClassObject * This,
            /* [in] */ ULONG uCount,
            /* [in] */ __RPC__in_opt IWbemObjectSink *pSink);
        
        DECLSPEC_XFGVIRT(IEnumWbemClassObject, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumWbemClassObject * This,
            /* [out] */ __RPC__deref_out_opt IEnumWbemClassObject **ppEnum);
        
        DECLSPEC_XFGVIRT(IEnumWbemClassObject, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumWbemClassObject * This,
            /* [in] */ long lTimeout,
            /* [in] */ ULONG nCount);
        
        END_INTERFACE
    } IEnumWbemClassObjectVtbl;

    interface IEnumWbemClassObject
    {
        CONST_VTBL struct IEnumWbemClassObjectVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumWbemClassObject_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumWbemClassObject_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumWbemClassObject_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumWbemClassObject_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumWbemClassObject_Next(This,lTimeout,uCount,apObjects,puReturned)	\
    ( (This)->lpVtbl -> Next(This,lTimeout,uCount,apObjects,puReturned) ) 

#define IEnumWbemClassObject_NextAsync(This,uCount,pSink)	\
    ( (This)->lpVtbl -> NextAsync(This,uCount,pSink) ) 

#define IEnumWbemClassObject_Clone(This,ppEnum)	\
    ( (This)->lpVtbl -> Clone(This,ppEnum) ) 

#define IEnumWbemClassObject_Skip(This,lTimeout,nCount)	\
    ( (This)->lpVtbl -> Skip(This,lTimeout,nCount) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumWbemClassObject_INTERFACE_DEFINED__ */


#ifndef __IWbemCallResult_INTERFACE_DEFINED__
#define __IWbemCallResult_INTERFACE_DEFINED__

/* interface IWbemCallResult */
/* [uuid][restricted][object] */ 


EXTERN_C const IID IID_IWbemCallResult;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("44aca675-e8fc-11d0-a07c-00c04fb68820")
    IWbemCallResult : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetResultObject( 
            /* [in] */ long lTimeout,
            /* [out] */ __RPC__deref_out_opt IWbemClassObject **ppResultObject) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetResultString( 
            /* [in] */ long lTimeout,
            /* [out] */ __RPC__deref_out_opt BSTR *pstrResultString) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetResultServices( 
            /* [in] */ long lTimeout,
            /* [out] */ __RPC__deref_out_opt IWbemServices **ppServices) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCallStatus( 
            /* [in] */ long lTimeout,
            /* [out] */ __RPC__out long *plStatus) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWbemCallResultVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWbemCallResult * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWbemCallResult * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWbemCallResult * This);
        
        DECLSPEC_XFGVIRT(IWbemCallResult, GetResultObject)
        HRESULT ( STDMETHODCALLTYPE *GetResultObject )( 
            __RPC__in IWbemCallResult * This,
            /* [in] */ long lTimeout,
            /* [out] */ __RPC__deref_out_opt IWbemClassObject **ppResultObject);
        
        DECLSPEC_XFGVIRT(IWbemCallResult, GetResultString)
        HRESULT ( STDMETHODCALLTYPE *GetResultString )( 
            __RPC__in IWbemCallResult * This,
            /* [in] */ long lTimeout,
            /* [out] */ __RPC__deref_out_opt BSTR *pstrResultString);
        
        DECLSPEC_XFGVIRT(IWbemCallResult, GetResultServices)
        HRESULT ( STDMETHODCALLTYPE *GetResultServices )( 
            __RPC__in IWbemCallResult * This,
            /* [in] */ long lTimeout,
            /* [out] */ __RPC__deref_out_opt IWbemServices **ppServices);
        
        DECLSPEC_XFGVIRT(IWbemCallResult, GetCallStatus)
        HRESULT ( STDMETHODCALLTYPE *GetCallStatus )( 
            __RPC__in IWbemCallResult * This,
            /* [in] */ long lTimeout,
            /* [out] */ __RPC__out long *plStatus);
        
        END_INTERFACE
    } IWbemCallResultVtbl;

    interface IWbemCallResult
    {
        CONST_VTBL struct IWbemCallResultVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWbemCallResult_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWbemCallResult_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWbemCallResult_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWbemCallResult_GetResultObject(This,lTimeout,ppResultObject)	\
    ( (This)->lpVtbl -> GetResultObject(This,lTimeout,ppResultObject) ) 

#define IWbemCallResult_GetResultString(This,lTimeout,pstrResultString)	\
    ( (This)->lpVtbl -> GetResultString(This,lTimeout,pstrResultString) ) 

#define IWbemCallResult_GetResultServices(This,lTimeout,ppServices)	\
    ( (This)->lpVtbl -> GetResultServices(This,lTimeout,ppServices) ) 

#define IWbemCallResult_GetCallStatus(This,lTimeout,plStatus)	\
    ( (This)->lpVtbl -> GetCallStatus(This,lTimeout,plStatus) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWbemCallResult_INTERFACE_DEFINED__ */


#ifndef __IWbemContext_INTERFACE_DEFINED__
#define __IWbemContext_INTERFACE_DEFINED__

/* interface IWbemContext */
/* [uuid][local][restricted][object] */ 


EXTERN_C const IID IID_IWbemContext;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("44aca674-e8fc-11d0-a07c-00c04fb68820")
    IWbemContext : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ IWbemContext **ppNewCopy) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNames( 
            /* [in] */ long lFlags,
            /* [out] */ SAFEARRAY * *pNames) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE BeginEnumeration( 
            /* [in] */ long lFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ long lFlags,
            /* [out] */ BSTR *pstrName,
            /* [out] */ VARIANT *pValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EndEnumeration( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetValue( 
            /* [string][in] */ LPCWSTR wszName,
            /* [in] */ long lFlags,
            /* [in] */ VARIANT *pValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetValue( 
            /* [string][in] */ LPCWSTR wszName,
            /* [in] */ long lFlags,
            /* [out] */ VARIANT *pValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteValue( 
            /* [string][in] */ LPCWSTR wszName,
            /* [in] */ long lFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteAll( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWbemContextVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWbemContext * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWbemContext * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWbemContext * This);
        
        DECLSPEC_XFGVIRT(IWbemContext, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            IWbemContext * This,
            /* [out] */ IWbemContext **ppNewCopy);
        
        DECLSPEC_XFGVIRT(IWbemContext, GetNames)
        HRESULT ( STDMETHODCALLTYPE *GetNames )( 
            IWbemContext * This,
            /* [in] */ long lFlags,
            /* [out] */ SAFEARRAY * *pNames);
        
        DECLSPEC_XFGVIRT(IWbemContext, BeginEnumeration)
        HRESULT ( STDMETHODCALLTYPE *BeginEnumeration )( 
            IWbemContext * This,
            /* [in] */ long lFlags);
        
        DECLSPEC_XFGVIRT(IWbemContext, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            IWbemContext * This,
            /* [in] */ long lFlags,
            /* [out] */ BSTR *pstrName,
            /* [out] */ VARIANT *pValue);
        
        DECLSPEC_XFGVIRT(IWbemContext, EndEnumeration)
        HRESULT ( STDMETHODCALLTYPE *EndEnumeration )( 
            IWbemContext * This);
        
        DECLSPEC_XFGVIRT(IWbemContext, SetValue)
        HRESULT ( STDMETHODCALLTYPE *SetValue )( 
            IWbemContext * This,
            /* [string][in] */ LPCWSTR wszName,
            /* [in] */ long lFlags,
            /* [in] */ VARIANT *pValue);
        
        DECLSPEC_XFGVIRT(IWbemContext, GetValue)
        HRESULT ( STDMETHODCALLTYPE *GetValue )( 
            IWbemContext * This,
            /* [string][in] */ LPCWSTR wszName,
            /* [in] */ long lFlags,
            /* [out] */ VARIANT *pValue);
        
        DECLSPEC_XFGVIRT(IWbemContext, DeleteValue)
        HRESULT ( STDMETHODCALLTYPE *DeleteValue )( 
            IWbemContext * This,
            /* [string][in] */ LPCWSTR wszName,
            /* [in] */ long lFlags);
        
        DECLSPEC_XFGVIRT(IWbemContext, DeleteAll)
        HRESULT ( STDMETHODCALLTYPE *DeleteAll )( 
            IWbemContext * This);
        
        END_INTERFACE
    } IWbemContextVtbl;

    interface IWbemContext
    {
        CONST_VTBL struct IWbemContextVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWbemContext_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWbemContext_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWbemContext_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWbemContext_Clone(This,ppNewCopy)	\
    ( (This)->lpVtbl -> Clone(This,ppNewCopy) ) 

#define IWbemContext_GetNames(This,lFlags,pNames)	\
    ( (This)->lpVtbl -> GetNames(This,lFlags,pNames) ) 

#define IWbemContext_BeginEnumeration(This,lFlags)	\
    ( (This)->lpVtbl -> BeginEnumeration(This,lFlags) ) 

#define IWbemContext_Next(This,lFlags,pstrName,pValue)	\
    ( (This)->lpVtbl -> Next(This,lFlags,pstrName,pValue) ) 

#define IWbemContext_EndEnumeration(This)	\
    ( (This)->lpVtbl -> EndEnumeration(This) ) 

#define IWbemContext_SetValue(This,wszName,lFlags,pValue)	\
    ( (This)->lpVtbl -> SetValue(This,wszName,lFlags,pValue) ) 

#define IWbemContext_GetValue(This,wszName,lFlags,pValue)	\
    ( (This)->lpVtbl -> GetValue(This,wszName,lFlags,pValue) ) 

#define IWbemContext_DeleteValue(This,wszName,lFlags)	\
    ( (This)->lpVtbl -> DeleteValue(This,wszName,lFlags) ) 

#define IWbemContext_DeleteAll(This)	\
    ( (This)->lpVtbl -> DeleteAll(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWbemContext_INTERFACE_DEFINED__ */


#ifndef __IUnsecuredApartment_INTERFACE_DEFINED__
#define __IUnsecuredApartment_INTERFACE_DEFINED__

/* interface IUnsecuredApartment */
/* [object][uuid][restricted] */ 


EXTERN_C const IID IID_IUnsecuredApartment;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1cfaba8c-1523-11d1-ad79-00c04fd8fdff")
    IUnsecuredApartment : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateObjectStub( 
            /* [in] */ __RPC__in_opt IUnknown *pObject,
            /* [out] */ __RPC__deref_out_opt IUnknown **ppStub) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUnsecuredApartmentVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IUnsecuredApartment * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IUnsecuredApartment * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IUnsecuredApartment * This);
        
        DECLSPEC_XFGVIRT(IUnsecuredApartment, CreateObjectStub)
        HRESULT ( STDMETHODCALLTYPE *CreateObjectStub )( 
            __RPC__in IUnsecuredApartment * This,
            /* [in] */ __RPC__in_opt IUnknown *pObject,
            /* [out] */ __RPC__deref_out_opt IUnknown **ppStub);
        
        END_INTERFACE
    } IUnsecuredApartmentVtbl;

    interface IUnsecuredApartment
    {
        CONST_VTBL struct IUnsecuredApartmentVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUnsecuredApartment_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUnsecuredApartment_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUnsecuredApartment_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUnsecuredApartment_CreateObjectStub(This,pObject,ppStub)	\
    ( (This)->lpVtbl -> CreateObjectStub(This,pObject,ppStub) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUnsecuredApartment_INTERFACE_DEFINED__ */


#ifndef __IWbemUnsecuredApartment_INTERFACE_DEFINED__
#define __IWbemUnsecuredApartment_INTERFACE_DEFINED__

/* interface IWbemUnsecuredApartment */
/* [object][uuid][restricted] */ 


EXTERN_C const IID IID_IWbemUnsecuredApartment;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("31739d04-3471-4cf4-9a7c-57a44ae71956")
    IWbemUnsecuredApartment : public IUnsecuredApartment
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateSinkStub( 
            /* [in] */ __RPC__in_opt IWbemObjectSink *pSink,
            /* [in] */ DWORD dwFlags,
            /* [unique][in] */ __RPC__in_opt LPCWSTR wszReserved,
            /* [out] */ __RPC__deref_out_opt IWbemObjectSink **ppStub) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWbemUnsecuredApartmentVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWbemUnsecuredApartment * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWbemUnsecuredApartment * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWbemUnsecuredApartment * This);
        
        DECLSPEC_XFGVIRT(IUnsecuredApartment, CreateObjectStub)
        HRESULT ( STDMETHODCALLTYPE *CreateObjectStub )( 
            __RPC__in IWbemUnsecuredApartment * This,
            /* [in] */ __RPC__in_opt IUnknown *pObject,
            /* [out] */ __RPC__deref_out_opt IUnknown **ppStub);
        
        DECLSPEC_XFGVIRT(IWbemUnsecuredApartment, CreateSinkStub)
        HRESULT ( STDMETHODCALLTYPE *CreateSinkStub )( 
            __RPC__in IWbemUnsecuredApartment * This,
            /* [in] */ __RPC__in_opt IWbemObjectSink *pSink,
            /* [in] */ DWORD dwFlags,
            /* [unique][in] */ __RPC__in_opt LPCWSTR wszReserved,
            /* [out] */ __RPC__deref_out_opt IWbemObjectSink **ppStub);
        
        END_INTERFACE
    } IWbemUnsecuredApartmentVtbl;

    interface IWbemUnsecuredApartment
    {
        CONST_VTBL struct IWbemUnsecuredApartmentVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWbemUnsecuredApartment_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWbemUnsecuredApartment_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWbemUnsecuredApartment_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWbemUnsecuredApartment_CreateObjectStub(This,pObject,ppStub)	\
    ( (This)->lpVtbl -> CreateObjectStub(This,pObject,ppStub) ) 


#define IWbemUnsecuredApartment_CreateSinkStub(This,pSink,dwFlags,wszReserved,ppStub)	\
    ( (This)->lpVtbl -> CreateSinkStub(This,pSink,dwFlags,wszReserved,ppStub) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWbemUnsecuredApartment_INTERFACE_DEFINED__ */


#ifndef __IWbemStatusCodeText_INTERFACE_DEFINED__
#define __IWbemStatusCodeText_INTERFACE_DEFINED__

/* interface IWbemStatusCodeText */
/* [uuid][object][local] */ 


EXTERN_C const IID IID_IWbemStatusCodeText;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("eb87e1bc-3233-11d2-aec9-00c04fb68820")
    IWbemStatusCodeText : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetErrorCodeText( 
            /* [in] */ HRESULT hRes,
            /* [in] */ LCID LocaleId,
            /* [in] */ long lFlags,
            /* [out] */ BSTR *MessageText) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFacilityCodeText( 
            /* [in] */ HRESULT hRes,
            /* [in] */ LCID LocaleId,
            /* [in] */ long lFlags,
            /* [out] */ BSTR *MessageText) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWbemStatusCodeTextVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWbemStatusCodeText * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWbemStatusCodeText * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWbemStatusCodeText * This);
        
        DECLSPEC_XFGVIRT(IWbemStatusCodeText, GetErrorCodeText)
        HRESULT ( STDMETHODCALLTYPE *GetErrorCodeText )( 
            IWbemStatusCodeText * This,
            /* [in] */ HRESULT hRes,
            /* [in] */ LCID LocaleId,
            /* [in] */ long lFlags,
            /* [out] */ BSTR *MessageText);
        
        DECLSPEC_XFGVIRT(IWbemStatusCodeText, GetFacilityCodeText)
        HRESULT ( STDMETHODCALLTYPE *GetFacilityCodeText )( 
            IWbemStatusCodeText * This,
            /* [in] */ HRESULT hRes,
            /* [in] */ LCID LocaleId,
            /* [in] */ long lFlags,
            /* [out] */ BSTR *MessageText);
        
        END_INTERFACE
    } IWbemStatusCodeTextVtbl;

    interface IWbemStatusCodeText
    {
        CONST_VTBL struct IWbemStatusCodeTextVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWbemStatusCodeText_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWbemStatusCodeText_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWbemStatusCodeText_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWbemStatusCodeText_GetErrorCodeText(This,hRes,LocaleId,lFlags,MessageText)	\
    ( (This)->lpVtbl -> GetErrorCodeText(This,hRes,LocaleId,lFlags,MessageText) ) 

#define IWbemStatusCodeText_GetFacilityCodeText(This,hRes,LocaleId,lFlags,MessageText)	\
    ( (This)->lpVtbl -> GetFacilityCodeText(This,hRes,LocaleId,lFlags,MessageText) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWbemStatusCodeText_INTERFACE_DEFINED__ */


#ifndef __IWbemBackupRestore_INTERFACE_DEFINED__
#define __IWbemBackupRestore_INTERFACE_DEFINED__

/* interface IWbemBackupRestore */
/* [uuid][restricted][object] */ 


EXTERN_C const IID IID_IWbemBackupRestore;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C49E32C7-BC8B-11d2-85D4-00105A1F8304")
    IWbemBackupRestore : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Backup( 
            /* [string][in] */ __RPC__in_string LPCWSTR strBackupToFile,
            /* [in] */ long lFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Restore( 
            /* [string][in] */ __RPC__in_string LPCWSTR strRestoreFromFile,
            /* [in] */ long lFlags) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWbemBackupRestoreVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWbemBackupRestore * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWbemBackupRestore * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWbemBackupRestore * This);
        
        DECLSPEC_XFGVIRT(IWbemBackupRestore, Backup)
        HRESULT ( STDMETHODCALLTYPE *Backup )( 
            __RPC__in IWbemBackupRestore * This,
            /* [string][in] */ __RPC__in_string LPCWSTR strBackupToFile,
            /* [in] */ long lFlags);
        
        DECLSPEC_XFGVIRT(IWbemBackupRestore, Restore)
        HRESULT ( STDMETHODCALLTYPE *Restore )( 
            __RPC__in IWbemBackupRestore * This,
            /* [string][in] */ __RPC__in_string LPCWSTR strRestoreFromFile,
            /* [in] */ long lFlags);
        
        END_INTERFACE
    } IWbemBackupRestoreVtbl;

    interface IWbemBackupRestore
    {
        CONST_VTBL struct IWbemBackupRestoreVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWbemBackupRestore_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWbemBackupRestore_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWbemBackupRestore_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWbemBackupRestore_Backup(This,strBackupToFile,lFlags)	\
    ( (This)->lpVtbl -> Backup(This,strBackupToFile,lFlags) ) 

#define IWbemBackupRestore_Restore(This,strRestoreFromFile,lFlags)	\
    ( (This)->lpVtbl -> Restore(This,strRestoreFromFile,lFlags) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWbemBackupRestore_INTERFACE_DEFINED__ */


#ifndef __IWbemBackupRestoreEx_INTERFACE_DEFINED__
#define __IWbemBackupRestoreEx_INTERFACE_DEFINED__

/* interface IWbemBackupRestoreEx */
/* [uuid][restricted][object] */ 


EXTERN_C const IID IID_IWbemBackupRestoreEx;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A359DEC5-E813-4834-8A2A-BA7F1D777D76")
    IWbemBackupRestoreEx : public IWbemBackupRestore
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Pause( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Resume( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWbemBackupRestoreExVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWbemBackupRestoreEx * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWbemBackupRestoreEx * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWbemBackupRestoreEx * This);
        
        DECLSPEC_XFGVIRT(IWbemBackupRestore, Backup)
        HRESULT ( STDMETHODCALLTYPE *Backup )( 
            __RPC__in IWbemBackupRestoreEx * This,
            /* [string][in] */ __RPC__in_string LPCWSTR strBackupToFile,
            /* [in] */ long lFlags);
        
        DECLSPEC_XFGVIRT(IWbemBackupRestore, Restore)
        HRESULT ( STDMETHODCALLTYPE *Restore )( 
            __RPC__in IWbemBackupRestoreEx * This,
            /* [string][in] */ __RPC__in_string LPCWSTR strRestoreFromFile,
            /* [in] */ long lFlags);
        
        DECLSPEC_XFGVIRT(IWbemBackupRestoreEx, Pause)
        HRESULT ( STDMETHODCALLTYPE *Pause )( 
            __RPC__in IWbemBackupRestoreEx * This);
        
        DECLSPEC_XFGVIRT(IWbemBackupRestoreEx, Resume)
        HRESULT ( STDMETHODCALLTYPE *Resume )( 
            __RPC__in IWbemBackupRestoreEx * This);
        
        END_INTERFACE
    } IWbemBackupRestoreExVtbl;

    interface IWbemBackupRestoreEx
    {
        CONST_VTBL struct IWbemBackupRestoreExVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWbemBackupRestoreEx_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWbemBackupRestoreEx_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWbemBackupRestoreEx_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWbemBackupRestoreEx_Backup(This,strBackupToFile,lFlags)	\
    ( (This)->lpVtbl -> Backup(This,strBackupToFile,lFlags) ) 

#define IWbemBackupRestoreEx_Restore(This,strRestoreFromFile,lFlags)	\
    ( (This)->lpVtbl -> Restore(This,strRestoreFromFile,lFlags) ) 


#define IWbemBackupRestoreEx_Pause(This)	\
    ( (This)->lpVtbl -> Pause(This) ) 

#define IWbemBackupRestoreEx_Resume(This)	\
    ( (This)->lpVtbl -> Resume(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWbemBackupRestoreEx_INTERFACE_DEFINED__ */


#ifndef __IWbemRefresher_INTERFACE_DEFINED__
#define __IWbemRefresher_INTERFACE_DEFINED__

/* interface IWbemRefresher */
/* [uuid][object][restricted][local] */ 


EXTERN_C const IID IID_IWbemRefresher;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("49353c99-516b-11d1-aea6-00c04fb68820")
    IWbemRefresher : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Refresh( 
            /* [in] */ long lFlags) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWbemRefresherVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWbemRefresher * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWbemRefresher * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWbemRefresher * This);
        
        DECLSPEC_XFGVIRT(IWbemRefresher, Refresh)
        HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            IWbemRefresher * This,
            /* [in] */ long lFlags);
        
        END_INTERFACE
    } IWbemRefresherVtbl;

    interface IWbemRefresher
    {
        CONST_VTBL struct IWbemRefresherVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWbemRefresher_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWbemRefresher_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWbemRefresher_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWbemRefresher_Refresh(This,lFlags)	\
    ( (This)->lpVtbl -> Refresh(This,lFlags) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWbemRefresher_INTERFACE_DEFINED__ */


#ifndef __IWbemHiPerfEnum_INTERFACE_DEFINED__
#define __IWbemHiPerfEnum_INTERFACE_DEFINED__

/* interface IWbemHiPerfEnum */
/* [uuid][object][restricted][local] */ 


EXTERN_C const IID IID_IWbemHiPerfEnum;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2705C288-79AE-11d2-B348-00105A1F8177")
    IWbemHiPerfEnum : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AddObjects( 
            /* [in] */ long lFlags,
            /* [in] */ ULONG uNumObjects,
            /* [size_is][in] */ long *apIds,
            /* [size_is][in] */ IWbemObjectAccess **apObj) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveObjects( 
            /* [in] */ long lFlags,
            /* [in] */ ULONG uNumObjects,
            /* [size_is][in] */ long *apIds) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetObjects( 
            /* [in] */ long lFlags,
            /* [in] */ ULONG uNumObjects,
            /* [length_is][size_is][out] */ IWbemObjectAccess **apObj,
            /* [out] */ ULONG *puReturned) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveAll( 
            /* [in] */ long lFlags) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWbemHiPerfEnumVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWbemHiPerfEnum * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWbemHiPerfEnum * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWbemHiPerfEnum * This);
        
        DECLSPEC_XFGVIRT(IWbemHiPerfEnum, AddObjects)
        HRESULT ( STDMETHODCALLTYPE *AddObjects )( 
            IWbemHiPerfEnum * This,
            /* [in] */ long lFlags,
            /* [in] */ ULONG uNumObjects,
            /* [size_is][in] */ long *apIds,
            /* [size_is][in] */ IWbemObjectAccess **apObj);
        
        DECLSPEC_XFGVIRT(IWbemHiPerfEnum, RemoveObjects)
        HRESULT ( STDMETHODCALLTYPE *RemoveObjects )( 
            IWbemHiPerfEnum * This,
            /* [in] */ long lFlags,
            /* [in] */ ULONG uNumObjects,
            /* [size_is][in] */ long *apIds);
        
        DECLSPEC_XFGVIRT(IWbemHiPerfEnum, GetObjects)
        HRESULT ( STDMETHODCALLTYPE *GetObjects )( 
            IWbemHiPerfEnum * This,
            /* [in] */ long lFlags,
            /* [in] */ ULONG uNumObjects,
            /* [length_is][size_is][out] */ IWbemObjectAccess **apObj,
            /* [out] */ ULONG *puReturned);
        
        DECLSPEC_XFGVIRT(IWbemHiPerfEnum, RemoveAll)
        HRESULT ( STDMETHODCALLTYPE *RemoveAll )( 
            IWbemHiPerfEnum * This,
            /* [in] */ long lFlags);
        
        END_INTERFACE
    } IWbemHiPerfEnumVtbl;

    interface IWbemHiPerfEnum
    {
        CONST_VTBL struct IWbemHiPerfEnumVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWbemHiPerfEnum_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWbemHiPerfEnum_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWbemHiPerfEnum_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWbemHiPerfEnum_AddObjects(This,lFlags,uNumObjects,apIds,apObj)	\
    ( (This)->lpVtbl -> AddObjects(This,lFlags,uNumObjects,apIds,apObj) ) 

#define IWbemHiPerfEnum_RemoveObjects(This,lFlags,uNumObjects,apIds)	\
    ( (This)->lpVtbl -> RemoveObjects(This,lFlags,uNumObjects,apIds) ) 

#define IWbemHiPerfEnum_GetObjects(This,lFlags,uNumObjects,apObj,puReturned)	\
    ( (This)->lpVtbl -> GetObjects(This,lFlags,uNumObjects,apObj,puReturned) ) 

#define IWbemHiPerfEnum_RemoveAll(This,lFlags)	\
    ( (This)->lpVtbl -> RemoveAll(This,lFlags) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWbemHiPerfEnum_INTERFACE_DEFINED__ */


#ifndef __IWbemConfigureRefresher_INTERFACE_DEFINED__
#define __IWbemConfigureRefresher_INTERFACE_DEFINED__

/* interface IWbemConfigureRefresher */
/* [uuid][object][restricted][local] */ 


EXTERN_C const IID IID_IWbemConfigureRefresher;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("49353c92-516b-11d1-aea6-00c04fb68820")
    IWbemConfigureRefresher : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AddObjectByPath( 
            /* [in] */ IWbemServices *pNamespace,
            /* [string][in] */ LPCWSTR wszPath,
            /* [in] */ long lFlags,
            /* [in] */ IWbemContext *pContext,
            /* [out] */ IWbemClassObject **ppRefreshable,
            /* [unique][in][out] */ long *plId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddObjectByTemplate( 
            /* [in] */ IWbemServices *pNamespace,
            /* [in] */ IWbemClassObject *pTemplate,
            /* [in] */ long lFlags,
            /* [in] */ IWbemContext *pContext,
            /* [out] */ IWbemClassObject **ppRefreshable,
            /* [unique][in][out] */ long *plId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddRefresher( 
            /* [in] */ IWbemRefresher *pRefresher,
            /* [in] */ long lFlags,
            /* [unique][in][out] */ long *plId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Remove( 
            /* [in] */ long lId,
            /* [in] */ long lFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddEnum( 
            /* [in] */ IWbemServices *pNamespace,
            /* [string][in] */ LPCWSTR wszClassName,
            /* [in] */ long lFlags,
            /* [in] */ IWbemContext *pContext,
            /* [out] */ IWbemHiPerfEnum **ppEnum,
            /* [unique][in][out] */ long *plId) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWbemConfigureRefresherVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWbemConfigureRefresher * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWbemConfigureRefresher * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWbemConfigureRefresher * This);
        
        DECLSPEC_XFGVIRT(IWbemConfigureRefresher, AddObjectByPath)
        HRESULT ( STDMETHODCALLTYPE *AddObjectByPath )( 
            IWbemConfigureRefresher * This,
            /* [in] */ IWbemServices *pNamespace,
            /* [string][in] */ LPCWSTR wszPath,
            /* [in] */ long lFlags,
            /* [in] */ IWbemContext *pContext,
            /* [out] */ IWbemClassObject **ppRefreshable,
            /* [unique][in][out] */ long *plId);
        
        DECLSPEC_XFGVIRT(IWbemConfigureRefresher, AddObjectByTemplate)
        HRESULT ( STDMETHODCALLTYPE *AddObjectByTemplate )( 
            IWbemConfigureRefresher * This,
            /* [in] */ IWbemServices *pNamespace,
            /* [in] */ IWbemClassObject *pTemplate,
            /* [in] */ long lFlags,
            /* [in] */ IWbemContext *pContext,
            /* [out] */ IWbemClassObject **ppRefreshable,
            /* [unique][in][out] */ long *plId);
        
        DECLSPEC_XFGVIRT(IWbemConfigureRefresher, AddRefresher)
        HRESULT ( STDMETHODCALLTYPE *AddRefresher )( 
            IWbemConfigureRefresher * This,
            /* [in] */ IWbemRefresher *pRefresher,
            /* [in] */ long lFlags,
            /* [unique][in][out] */ long *plId);
        
        DECLSPEC_XFGVIRT(IWbemConfigureRefresher, Remove)
        HRESULT ( STDMETHODCALLTYPE *Remove )( 
            IWbemConfigureRefresher * This,
            /* [in] */ long lId,
            /* [in] */ long lFlags);
        
        DECLSPEC_XFGVIRT(IWbemConfigureRefresher, AddEnum)
        HRESULT ( STDMETHODCALLTYPE *AddEnum )( 
            IWbemConfigureRefresher * This,
            /* [in] */ IWbemServices *pNamespace,
            /* [string][in] */ LPCWSTR wszClassName,
            /* [in] */ long lFlags,
            /* [in] */ IWbemContext *pContext,
            /* [out] */ IWbemHiPerfEnum **ppEnum,
            /* [unique][in][out] */ long *plId);
        
        END_INTERFACE
    } IWbemConfigureRefresherVtbl;

    interface IWbemConfigureRefresher
    {
        CONST_VTBL struct IWbemConfigureRefresherVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWbemConfigureRefresher_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWbemConfigureRefresher_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWbemConfigureRefresher_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWbemConfigureRefresher_AddObjectByPath(This,pNamespace,wszPath,lFlags,pContext,ppRefreshable,plId)	\
    ( (This)->lpVtbl -> AddObjectByPath(This,pNamespace,wszPath,lFlags,pContext,ppRefreshable,plId) ) 

#define IWbemConfigureRefresher_AddObjectByTemplate(This,pNamespace,pTemplate,lFlags,pContext,ppRefreshable,plId)	\
    ( (This)->lpVtbl -> AddObjectByTemplate(This,pNamespace,pTemplate,lFlags,pContext,ppRefreshable,plId) ) 

#define IWbemConfigureRefresher_AddRefresher(This,pRefresher,lFlags,plId)	\
    ( (This)->lpVtbl -> AddRefresher(This,pRefresher,lFlags,plId) ) 

#define IWbemConfigureRefresher_Remove(This,lId,lFlags)	\
    ( (This)->lpVtbl -> Remove(This,lId,lFlags) ) 

#define IWbemConfigureRefresher_AddEnum(This,pNamespace,wszClassName,lFlags,pContext,ppEnum,plId)	\
    ( (This)->lpVtbl -> AddEnum(This,pNamespace,wszClassName,lFlags,pContext,ppEnum,plId) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWbemConfigureRefresher_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_WbemLocator;

#ifdef __cplusplus

class DECLSPEC_UUID("4590f811-1d3a-11d0-891f-00aa004b2e24")
WbemLocator;
#endif

EXTERN_C const CLSID CLSID_WbemContext;

#ifdef __cplusplus

class DECLSPEC_UUID("674B6698-EE92-11d0-AD71-00C04FD8FDFF")
WbemContext;
#endif

EXTERN_C const CLSID CLSID_UnsecuredApartment;

#ifdef __cplusplus

class DECLSPEC_UUID("49bd2028-1523-11d1-ad79-00c04fd8fdff")
UnsecuredApartment;
#endif

EXTERN_C const CLSID CLSID_WbemClassObject;

#ifdef __cplusplus

class DECLSPEC_UUID("9A653086-174F-11d2-B5F9-00104B703EFD")
WbemClassObject;
#endif

EXTERN_C const CLSID CLSID_MofCompiler;

#ifdef __cplusplus

class DECLSPEC_UUID("6daf9757-2e37-11d2-aec9-00c04fb68820")
MofCompiler;
#endif

EXTERN_C const CLSID CLSID_WbemStatusCodeText;

#ifdef __cplusplus

class DECLSPEC_UUID("eb87e1bd-3233-11d2-aec9-00c04fb68820")
WbemStatusCodeText;
#endif

EXTERN_C const CLSID CLSID_WbemBackupRestore;

#ifdef __cplusplus

class DECLSPEC_UUID("C49E32C6-BC8B-11d2-85D4-00105A1F8304")
WbemBackupRestore;
#endif

EXTERN_C const CLSID CLSID_WbemRefresher;

#ifdef __cplusplus

class DECLSPEC_UUID("c71566f2-561e-11d1-ad87-00c04fd8fdff")
WbemRefresher;
#endif

EXTERN_C const CLSID CLSID_WbemObjectTextSrc;

#ifdef __cplusplus

class DECLSPEC_UUID("8D1C559D-84F0-4bb3-A7D5-56A7435A9BA6")
WbemObjectTextSrc;
#endif
#endif /* __WbemClient_v1_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_wbemcli_0000_0001 */
/* [local] */ 

/*******************************************************************************/
/*                                                                             */
/*    Copyright (c) Microsoft Corporation.  All rights reserved.               */
/*                                                                             */
/*    Interfaces for WBEM clients. These interfaces are implemented by CIMOM   */
/*    and used by clients.  Additional interfaces needed by WBEM providers can */
/*    be found in WBEMPROV.IDL                                                 */
/*                                                                             */
/*******************************************************************************/



extern RPC_IF_HANDLE __MIDL_itf_wbemcli_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wbemcli_0000_0001_v0_0_s_ifspec;

/* interface __MIDL_itf_wbemcli_0000_0003 */
/* [local] */ 




extern RPC_IF_HANDLE __MIDL_itf_wbemcli_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wbemcli_0000_0003_v0_0_s_ifspec;

#ifndef __IWbemObjectSinkEx_INTERFACE_DEFINED__
#define __IWbemObjectSinkEx_INTERFACE_DEFINED__

/* interface IWbemObjectSinkEx */
/* [uuid][restricted][object] */ 


EXTERN_C const IID IID_IWbemObjectSinkEx;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("e7d35cfa-348b-485e-b524-252725d697ca")
    IWbemObjectSinkEx : public IWbemObjectSink
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE WriteMessage( 
            /* [in] */ ULONG uChannel,
            /* [in] */ __RPC__in const BSTR strMessage) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WriteError( 
            /* [unique][in] */ __RPC__in_opt IWbemClassObject *pObjError,
            /* [out] */ __RPC__out unsigned char *puReturned) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE PromptUser( 
            /* [in] */ __RPC__in const BSTR strMessage,
            /* [in] */ unsigned char uPromptType,
            /* [out] */ __RPC__out unsigned char *puReturned) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WriteProgress( 
            /* [in] */ __RPC__in const BSTR strActivity,
            /* [in] */ __RPC__in const BSTR strCurrentOperation,
            /* [in] */ __RPC__in const BSTR strStatusDescription,
            /* [in] */ ULONG uPercentComplete,
            /* [in] */ ULONG uSecondsRemaining) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WriteStreamParameter( 
            /* [in] */ __RPC__in const BSTR strName,
            /* [in] */ __RPC__in VARIANT *vtValue,
            /* [in] */ ULONG ulType,
            /* [in] */ ULONG ulFlags) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWbemObjectSinkExVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWbemObjectSinkEx * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWbemObjectSinkEx * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWbemObjectSinkEx * This);
        
        DECLSPEC_XFGVIRT(IWbemObjectSink, Indicate)
        HRESULT ( STDMETHODCALLTYPE *Indicate )( 
            __RPC__in IWbemObjectSinkEx * This,
            /* [in] */ long lObjectCount,
            /* [size_is][in] */ __RPC__in_ecount_full(lObjectCount) IWbemClassObject **apObjArray);
        
        DECLSPEC_XFGVIRT(IWbemObjectSink, SetStatus)
        HRESULT ( STDMETHODCALLTYPE *SetStatus )( 
            __RPC__in IWbemObjectSinkEx * This,
            /* [in] */ long lFlags,
            /* [in] */ HRESULT hResult,
            /* [unique][in] */ __RPC__in_opt BSTR strParam,
            /* [unique][in] */ __RPC__in_opt IWbemClassObject *pObjParam);
        
        DECLSPEC_XFGVIRT(IWbemObjectSinkEx, WriteMessage)
        HRESULT ( STDMETHODCALLTYPE *WriteMessage )( 
            __RPC__in IWbemObjectSinkEx * This,
            /* [in] */ ULONG uChannel,
            /* [in] */ __RPC__in const BSTR strMessage);
        
        DECLSPEC_XFGVIRT(IWbemObjectSinkEx, WriteError)
        HRESULT ( STDMETHODCALLTYPE *WriteError )( 
            __RPC__in IWbemObjectSinkEx * This,
            /* [unique][in] */ __RPC__in_opt IWbemClassObject *pObjError,
            /* [out] */ __RPC__out unsigned char *puReturned);
        
        DECLSPEC_XFGVIRT(IWbemObjectSinkEx, PromptUser)
        HRESULT ( STDMETHODCALLTYPE *PromptUser )( 
            __RPC__in IWbemObjectSinkEx * This,
            /* [in] */ __RPC__in const BSTR strMessage,
            /* [in] */ unsigned char uPromptType,
            /* [out] */ __RPC__out unsigned char *puReturned);
        
        DECLSPEC_XFGVIRT(IWbemObjectSinkEx, WriteProgress)
        HRESULT ( STDMETHODCALLTYPE *WriteProgress )( 
            __RPC__in IWbemObjectSinkEx * This,
            /* [in] */ __RPC__in const BSTR strActivity,
            /* [in] */ __RPC__in const BSTR strCurrentOperation,
            /* [in] */ __RPC__in const BSTR strStatusDescription,
            /* [in] */ ULONG uPercentComplete,
            /* [in] */ ULONG uSecondsRemaining);
        
        DECLSPEC_XFGVIRT(IWbemObjectSinkEx, WriteStreamParameter)
        HRESULT ( STDMETHODCALLTYPE *WriteStreamParameter )( 
            __RPC__in IWbemObjectSinkEx * This,
            /* [in] */ __RPC__in const BSTR strName,
            /* [in] */ __RPC__in VARIANT *vtValue,
            /* [in] */ ULONG ulType,
            /* [in] */ ULONG ulFlags);
        
        END_INTERFACE
    } IWbemObjectSinkExVtbl;

    interface IWbemObjectSinkEx
    {
        CONST_VTBL struct IWbemObjectSinkExVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWbemObjectSinkEx_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWbemObjectSinkEx_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWbemObjectSinkEx_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWbemObjectSinkEx_Indicate(This,lObjectCount,apObjArray)	\
    ( (This)->lpVtbl -> Indicate(This,lObjectCount,apObjArray) ) 

#define IWbemObjectSinkEx_SetStatus(This,lFlags,hResult,strParam,pObjParam)	\
    ( (This)->lpVtbl -> SetStatus(This,lFlags,hResult,strParam,pObjParam) ) 


#define IWbemObjectSinkEx_WriteMessage(This,uChannel,strMessage)	\
    ( (This)->lpVtbl -> WriteMessage(This,uChannel,strMessage) ) 

#define IWbemObjectSinkEx_WriteError(This,pObjError,puReturned)	\
    ( (This)->lpVtbl -> WriteError(This,pObjError,puReturned) ) 

#define IWbemObjectSinkEx_PromptUser(This,strMessage,uPromptType,puReturned)	\
    ( (This)->lpVtbl -> PromptUser(This,strMessage,uPromptType,puReturned) ) 

#define IWbemObjectSinkEx_WriteProgress(This,strActivity,strCurrentOperation,strStatusDescription,uPercentComplete,uSecondsRemaining)	\
    ( (This)->lpVtbl -> WriteProgress(This,strActivity,strCurrentOperation,strStatusDescription,uPercentComplete,uSecondsRemaining) ) 

#define IWbemObjectSinkEx_WriteStreamParameter(This,strName,vtValue,ulType,ulFlags)	\
    ( (This)->lpVtbl -> WriteStreamParameter(This,strName,vtValue,ulType,ulFlags) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWbemObjectSinkEx_INTERFACE_DEFINED__ */


#ifndef __IWbemShutdown_INTERFACE_DEFINED__
#define __IWbemShutdown_INTERFACE_DEFINED__

/* interface IWbemShutdown */
/* [uuid][object] */ 


EXTERN_C const IID IID_IWbemShutdown;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("b7b31df9-d515-11d3-a11c-00105a1f515a")
    IWbemShutdown : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Shutdown( 
            /* [in] */ LONG uReason,
            /* [in] */ ULONG uMaxMilliseconds,
            /* [in] */ __RPC__in_opt IWbemContext *pCtx) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWbemShutdownVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWbemShutdown * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWbemShutdown * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWbemShutdown * This);
        
        DECLSPEC_XFGVIRT(IWbemShutdown, Shutdown)
        HRESULT ( STDMETHODCALLTYPE *Shutdown )( 
            __RPC__in IWbemShutdown * This,
            /* [in] */ LONG uReason,
            /* [in] */ ULONG uMaxMilliseconds,
            /* [in] */ __RPC__in_opt IWbemContext *pCtx);
        
        END_INTERFACE
    } IWbemShutdownVtbl;

    interface IWbemShutdown
    {
        CONST_VTBL struct IWbemShutdownVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWbemShutdown_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWbemShutdown_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWbemShutdown_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWbemShutdown_Shutdown(This,uReason,uMaxMilliseconds,pCtx)	\
    ( (This)->lpVtbl -> Shutdown(This,uReason,uMaxMilliseconds,pCtx) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWbemShutdown_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_wbemcli_0000_0011 */
/* [local] */ 

typedef /* [v1_enum] */ 
enum tag_WMI_OBJ_TEXT
    {
        WMI_OBJ_TEXT_CIM_DTD_2_0	= 1,
        WMI_OBJ_TEXT_WMI_DTD_2_0	= 2,
        WMI_OBJ_TEXT_WMI_EXT1	= 3,
        WMI_OBJ_TEXT_WMI_EXT2	= 4,
        WMI_OBJ_TEXT_WMI_EXT3	= 5,
        WMI_OBJ_TEXT_WMI_EXT4	= 6,
        WMI_OBJ_TEXT_WMI_EXT5	= 7,
        WMI_OBJ_TEXT_WMI_EXT6	= 8,
        WMI_OBJ_TEXT_WMI_EXT7	= 9,
        WMI_OBJ_TEXT_WMI_EXT8	= 10,
        WMI_OBJ_TEXT_WMI_EXT9	= 11,
        WMI_OBJ_TEXT_WMI_EXT10	= 12,
        WMI_OBJ_TEXT_LAST	= 13
    } 	WMI_OBJ_TEXT;



extern RPC_IF_HANDLE __MIDL_itf_wbemcli_0000_0011_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wbemcli_0000_0011_v0_0_s_ifspec;

#ifndef __IWbemObjectTextSrc_INTERFACE_DEFINED__
#define __IWbemObjectTextSrc_INTERFACE_DEFINED__

/* interface IWbemObjectTextSrc */
/* [uuid][local][restricted][object] */ 


EXTERN_C const IID IID_IWbemObjectTextSrc;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("bfbf883a-cad7-11d3-a11b-00105a1f515a")
    IWbemObjectTextSrc : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetText( 
            /* [in] */ long lFlags,
            /* [in] */ IWbemClassObject *pObj,
            /* [in] */ ULONG uObjTextFormat,
            /* [in] */ IWbemContext *pCtx,
            /* [out] */ BSTR *strText) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateFromText( 
            /* [in] */ long lFlags,
            /* [in] */ BSTR strText,
            /* [in] */ ULONG uObjTextFormat,
            /* [in] */ IWbemContext *pCtx,
            /* [out] */ IWbemClassObject **pNewObj) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWbemObjectTextSrcVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWbemObjectTextSrc * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWbemObjectTextSrc * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWbemObjectTextSrc * This);
        
        DECLSPEC_XFGVIRT(IWbemObjectTextSrc, GetText)
        HRESULT ( STDMETHODCALLTYPE *GetText )( 
            IWbemObjectTextSrc * This,
            /* [in] */ long lFlags,
            /* [in] */ IWbemClassObject *pObj,
            /* [in] */ ULONG uObjTextFormat,
            /* [in] */ IWbemContext *pCtx,
            /* [out] */ BSTR *strText);
        
        DECLSPEC_XFGVIRT(IWbemObjectTextSrc, CreateFromText)
        HRESULT ( STDMETHODCALLTYPE *CreateFromText )( 
            IWbemObjectTextSrc * This,
            /* [in] */ long lFlags,
            /* [in] */ BSTR strText,
            /* [in] */ ULONG uObjTextFormat,
            /* [in] */ IWbemContext *pCtx,
            /* [out] */ IWbemClassObject **pNewObj);
        
        END_INTERFACE
    } IWbemObjectTextSrcVtbl;

    interface IWbemObjectTextSrc
    {
        CONST_VTBL struct IWbemObjectTextSrcVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWbemObjectTextSrc_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWbemObjectTextSrc_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWbemObjectTextSrc_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWbemObjectTextSrc_GetText(This,lFlags,pObj,uObjTextFormat,pCtx,strText)	\
    ( (This)->lpVtbl -> GetText(This,lFlags,pObj,uObjTextFormat,pCtx,strText) ) 

#define IWbemObjectTextSrc_CreateFromText(This,lFlags,strText,uObjTextFormat,pCtx,pNewObj)	\
    ( (This)->lpVtbl -> CreateFromText(This,lFlags,strText,uObjTextFormat,pCtx,pNewObj) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWbemObjectTextSrc_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_wbemcli_0000_0013 */
/* [local] */ 

typedef struct tag_CompileStatusInfo
    {
    long lPhaseError;
    HRESULT hRes;
    long ObjectNum;
    long FirstLine;
    long LastLine;
    DWORD dwOutFlags;
    } 	WBEM_COMPILE_STATUS_INFO;

typedef /* [v1_enum] */ 
enum tag_WBEM_COMPILER_OPTIONS
    {
        WBEM_FLAG_CHECK_ONLY	= 0x1,
        WBEM_FLAG_AUTORECOVER	= 0x2,
        WBEM_FLAG_WMI_CHECK	= 0x4,
        WBEM_FLAG_CONSOLE_PRINT	= 0x8,
        WBEM_FLAG_DONT_ADD_TO_LIST	= 0x10,
        WBEM_FLAG_SPLIT_FILES	= 0x20,
        WBEM_FLAG_STORE_FILE	= 0x100
    } 	WBEM_COMPILER_OPTIONS;

typedef /* [v1_enum] */ 
enum tag_WBEM_CONNECT_OPTIONS
    {
        WBEM_FLAG_CONNECT_REPOSITORY_ONLY	= 0x40,
        WBEM_FLAG_CONNECT_USE_MAX_WAIT	= 0x80,
        WBEM_FLAG_CONNECT_PROVIDERS	= 0x100
    } 	WBEM_CONNECT_OPTIONS;



extern RPC_IF_HANDLE __MIDL_itf_wbemcli_0000_0013_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wbemcli_0000_0013_v0_0_s_ifspec;

#ifndef __IMofCompiler_INTERFACE_DEFINED__
#define __IMofCompiler_INTERFACE_DEFINED__

/* interface IMofCompiler */
/* [uuid][object][local] */ 


EXTERN_C const IID IID_IMofCompiler;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6daf974e-2e37-11d2-aec9-00c04fb68820")
    IMofCompiler : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CompileFile( 
            /* [annotation][string][in] */ 
            _In_  LPWSTR FileName,
            /* [annotation][string][in] */ 
            _In_  LPWSTR ServerAndNamespace,
            /* [annotation][string][in] */ 
            _In_  LPWSTR User,
            /* [annotation][string][in] */ 
            _In_  LPWSTR Authority,
            /* [annotation][string][in] */ 
            _In_  LPWSTR Password,
            /* [annotation][in] */ 
            _In_  LONG lOptionFlags,
            /* [annotation][in] */ 
            _In_  LONG lClassFlags,
            /* [annotation][in] */ 
            _In_  LONG lInstanceFlags,
            /* [annotation][out][in] */ 
            _Inout_  WBEM_COMPILE_STATUS_INFO *pInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CompileBuffer( 
            /* [annotation][in] */ 
            _In_  long BuffSize,
            /* [annotation][size_is][in] */ 
            _In_reads_bytes_(BuffSize)  BYTE *pBuffer,
            /* [annotation][string][in] */ 
            _In_  LPWSTR ServerAndNamespace,
            /* [annotation][string][in] */ 
            _In_  LPWSTR User,
            /* [annotation][string][in] */ 
            _In_  LPWSTR Authority,
            /* [annotation][string][in] */ 
            _In_  LPWSTR Password,
            /* [annotation][in] */ 
            _In_  LONG lOptionFlags,
            /* [annotation][in] */ 
            _In_  LONG lClassFlags,
            /* [annotation][in] */ 
            _In_  LONG lInstanceFlags,
            /* [annotation][out][in] */ 
            _Inout_  WBEM_COMPILE_STATUS_INFO *pInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateBMOF( 
            /* [annotation][string][in] */ 
            _In_  LPWSTR TextFileName,
            /* [annotation][string][in] */ 
            _In_  LPWSTR BMOFFileName,
            /* [annotation][string][in] */ 
            _In_  LPWSTR ServerAndNamespace,
            /* [annotation][in] */ 
            _In_  LONG lOptionFlags,
            /* [annotation][in] */ 
            _In_  LONG lClassFlags,
            /* [annotation][in] */ 
            _In_  LONG lInstanceFlags,
            /* [annotation][out][in] */ 
            _Inout_  WBEM_COMPILE_STATUS_INFO *pInfo) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMofCompilerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMofCompiler * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMofCompiler * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMofCompiler * This);
        
        DECLSPEC_XFGVIRT(IMofCompiler, CompileFile)
        HRESULT ( STDMETHODCALLTYPE *CompileFile )( 
            IMofCompiler * This,
            /* [annotation][string][in] */ 
            _In_  LPWSTR FileName,
            /* [annotation][string][in] */ 
            _In_  LPWSTR ServerAndNamespace,
            /* [annotation][string][in] */ 
            _In_  LPWSTR User,
            /* [annotation][string][in] */ 
            _In_  LPWSTR Authority,
            /* [annotation][string][in] */ 
            _In_  LPWSTR Password,
            /* [annotation][in] */ 
            _In_  LONG lOptionFlags,
            /* [annotation][in] */ 
            _In_  LONG lClassFlags,
            /* [annotation][in] */ 
            _In_  LONG lInstanceFlags,
            /* [annotation][out][in] */ 
            _Inout_  WBEM_COMPILE_STATUS_INFO *pInfo);
        
        DECLSPEC_XFGVIRT(IMofCompiler, CompileBuffer)
        HRESULT ( STDMETHODCALLTYPE *CompileBuffer )( 
            IMofCompiler * This,
            /* [annotation][in] */ 
            _In_  long BuffSize,
            /* [annotation][size_is][in] */ 
            _In_reads_bytes_(BuffSize)  BYTE *pBuffer,
            /* [annotation][string][in] */ 
            _In_  LPWSTR ServerAndNamespace,
            /* [annotation][string][in] */ 
            _In_  LPWSTR User,
            /* [annotation][string][in] */ 
            _In_  LPWSTR Authority,
            /* [annotation][string][in] */ 
            _In_  LPWSTR Password,
            /* [annotation][in] */ 
            _In_  LONG lOptionFlags,
            /* [annotation][in] */ 
            _In_  LONG lClassFlags,
            /* [annotation][in] */ 
            _In_  LONG lInstanceFlags,
            /* [annotation][out][in] */ 
            _Inout_  WBEM_COMPILE_STATUS_INFO *pInfo);
        
        DECLSPEC_XFGVIRT(IMofCompiler, CreateBMOF)
        HRESULT ( STDMETHODCALLTYPE *CreateBMOF )( 
            IMofCompiler * This,
            /* [annotation][string][in] */ 
            _In_  LPWSTR TextFileName,
            /* [annotation][string][in] */ 
            _In_  LPWSTR BMOFFileName,
            /* [annotation][string][in] */ 
            _In_  LPWSTR ServerAndNamespace,
            /* [annotation][in] */ 
            _In_  LONG lOptionFlags,
            /* [annotation][in] */ 
            _In_  LONG lClassFlags,
            /* [annotation][in] */ 
            _In_  LONG lInstanceFlags,
            /* [annotation][out][in] */ 
            _Inout_  WBEM_COMPILE_STATUS_INFO *pInfo);
        
        END_INTERFACE
    } IMofCompilerVtbl;

    interface IMofCompiler
    {
        CONST_VTBL struct IMofCompilerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMofCompiler_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMofCompiler_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMofCompiler_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMofCompiler_CompileFile(This,FileName,ServerAndNamespace,User,Authority,Password,lOptionFlags,lClassFlags,lInstanceFlags,pInfo)	\
    ( (This)->lpVtbl -> CompileFile(This,FileName,ServerAndNamespace,User,Authority,Password,lOptionFlags,lClassFlags,lInstanceFlags,pInfo) ) 

#define IMofCompiler_CompileBuffer(This,BuffSize,pBuffer,ServerAndNamespace,User,Authority,Password,lOptionFlags,lClassFlags,lInstanceFlags,pInfo)	\
    ( (This)->lpVtbl -> CompileBuffer(This,BuffSize,pBuffer,ServerAndNamespace,User,Authority,Password,lOptionFlags,lClassFlags,lInstanceFlags,pInfo) ) 

#define IMofCompiler_CreateBMOF(This,TextFileName,BMOFFileName,ServerAndNamespace,lOptionFlags,lClassFlags,lInstanceFlags,pInfo)	\
    ( (This)->lpVtbl -> CreateBMOF(This,TextFileName,BMOFFileName,ServerAndNamespace,lOptionFlags,lClassFlags,lInstanceFlags,pInfo) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMofCompiler_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_wbemcli_0000_0015 */
/* [local] */ 

typedef /* [v1_enum] */ 
enum tag_WBEM_UNSECAPP_FLAG_TYPE
    {
        WBEM_FLAG_UNSECAPP_DEFAULT_CHECK_ACCESS	= 0,
        WBEM_FLAG_UNSECAPP_CHECK_ACCESS	= 1,
        WBEM_FLAG_UNSECAPP_DONT_CHECK_ACCESS	= 2
    } 	WBEM_UNSECAPP_FLAG_TYPE;



extern RPC_IF_HANDLE __MIDL_itf_wbemcli_0000_0015_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wbemcli_0000_0015_v0_0_s_ifspec;

/* interface __MIDL_itf_wbemcli_0000_0016 */
/* [local] */ 

typedef /* [v1_enum] */ 
enum tag_WBEM_INFORMATION_FLAG_TYPE
    {
        WBEM_FLAG_SHORT_NAME	= 0x1,
        WBEM_FLAG_LONG_NAME	= 0x2
    } 	WBEM_INFORMATION_FLAG_TYPE;



extern RPC_IF_HANDLE __MIDL_itf_wbemcli_0000_0016_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wbemcli_0000_0016_v0_0_s_ifspec;

/* interface __MIDL_itf_wbemcli_0000_0022 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_WINMGMT) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_wbemcli_0000_0022_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wbemcli_0000_0022_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  BSTR_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  VARIANT_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

unsigned long             __RPC_USER  BSTR_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree64(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  VARIANT_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree64(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


