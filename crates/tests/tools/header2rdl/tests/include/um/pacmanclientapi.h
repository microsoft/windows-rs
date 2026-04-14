

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

#ifndef __pacmanclientapi_h__
#define __pacmanclientapi_h__

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

#ifndef __IPMApplicationInfo_FWD_DEFINED__
#define __IPMApplicationInfo_FWD_DEFINED__
typedef interface IPMApplicationInfo IPMApplicationInfo;

#endif 	/* __IPMApplicationInfo_FWD_DEFINED__ */


#ifndef __IPMTilePropertyInfo_FWD_DEFINED__
#define __IPMTilePropertyInfo_FWD_DEFINED__
typedef interface IPMTilePropertyInfo IPMTilePropertyInfo;

#endif 	/* __IPMTilePropertyInfo_FWD_DEFINED__ */


#ifndef __IPMTilePropertyEnumerator_FWD_DEFINED__
#define __IPMTilePropertyEnumerator_FWD_DEFINED__
typedef interface IPMTilePropertyEnumerator IPMTilePropertyEnumerator;

#endif 	/* __IPMTilePropertyEnumerator_FWD_DEFINED__ */


#ifndef __IPMTileInfo_FWD_DEFINED__
#define __IPMTileInfo_FWD_DEFINED__
typedef interface IPMTileInfo IPMTileInfo;

#endif 	/* __IPMTileInfo_FWD_DEFINED__ */


#ifndef __IPMTileInfoEnumerator_FWD_DEFINED__
#define __IPMTileInfoEnumerator_FWD_DEFINED__
typedef interface IPMTileInfoEnumerator IPMTileInfoEnumerator;

#endif 	/* __IPMTileInfoEnumerator_FWD_DEFINED__ */


#ifndef __IPMApplicationInfoEnumerator_FWD_DEFINED__
#define __IPMApplicationInfoEnumerator_FWD_DEFINED__
typedef interface IPMApplicationInfoEnumerator IPMApplicationInfoEnumerator;

#endif 	/* __IPMApplicationInfoEnumerator_FWD_DEFINED__ */


#ifndef __IPMLiveTileJobInfo_FWD_DEFINED__
#define __IPMLiveTileJobInfo_FWD_DEFINED__
typedef interface IPMLiveTileJobInfo IPMLiveTileJobInfo;

#endif 	/* __IPMLiveTileJobInfo_FWD_DEFINED__ */


#ifndef __IPMLiveTileJobInfoEnumerator_FWD_DEFINED__
#define __IPMLiveTileJobInfoEnumerator_FWD_DEFINED__
typedef interface IPMLiveTileJobInfoEnumerator IPMLiveTileJobInfoEnumerator;

#endif 	/* __IPMLiveTileJobInfoEnumerator_FWD_DEFINED__ */


#ifndef __IPMDeploymentManager_FWD_DEFINED__
#define __IPMDeploymentManager_FWD_DEFINED__
typedef interface IPMDeploymentManager IPMDeploymentManager;

#endif 	/* __IPMDeploymentManager_FWD_DEFINED__ */


#ifndef __IPMEnumerationManager_FWD_DEFINED__
#define __IPMEnumerationManager_FWD_DEFINED__
typedef interface IPMEnumerationManager IPMEnumerationManager;

#endif 	/* __IPMEnumerationManager_FWD_DEFINED__ */


#ifndef __IPMTaskInfo_FWD_DEFINED__
#define __IPMTaskInfo_FWD_DEFINED__
typedef interface IPMTaskInfo IPMTaskInfo;

#endif 	/* __IPMTaskInfo_FWD_DEFINED__ */


#ifndef __IPMTaskInfoEnumerator_FWD_DEFINED__
#define __IPMTaskInfoEnumerator_FWD_DEFINED__
typedef interface IPMTaskInfoEnumerator IPMTaskInfoEnumerator;

#endif 	/* __IPMTaskInfoEnumerator_FWD_DEFINED__ */


#ifndef __IPMExtensionInfo_FWD_DEFINED__
#define __IPMExtensionInfo_FWD_DEFINED__
typedef interface IPMExtensionInfo IPMExtensionInfo;

#endif 	/* __IPMExtensionInfo_FWD_DEFINED__ */


#ifndef __IPMExtensionFileExtensionInfo_FWD_DEFINED__
#define __IPMExtensionFileExtensionInfo_FWD_DEFINED__
typedef interface IPMExtensionFileExtensionInfo IPMExtensionFileExtensionInfo;

#endif 	/* __IPMExtensionFileExtensionInfo_FWD_DEFINED__ */


#ifndef __IPMExtensionProtocolInfo_FWD_DEFINED__
#define __IPMExtensionProtocolInfo_FWD_DEFINED__
typedef interface IPMExtensionProtocolInfo IPMExtensionProtocolInfo;

#endif 	/* __IPMExtensionProtocolInfo_FWD_DEFINED__ */


#ifndef __IPMExtensionShareTargetInfo_FWD_DEFINED__
#define __IPMExtensionShareTargetInfo_FWD_DEFINED__
typedef interface IPMExtensionShareTargetInfo IPMExtensionShareTargetInfo;

#endif 	/* __IPMExtensionShareTargetInfo_FWD_DEFINED__ */


#ifndef __IPMExtensionContractInfo_FWD_DEFINED__
#define __IPMExtensionContractInfo_FWD_DEFINED__
typedef interface IPMExtensionContractInfo IPMExtensionContractInfo;

#endif 	/* __IPMExtensionContractInfo_FWD_DEFINED__ */


#ifndef __IPMExtensionFileOpenPickerInfo_FWD_DEFINED__
#define __IPMExtensionFileOpenPickerInfo_FWD_DEFINED__
typedef interface IPMExtensionFileOpenPickerInfo IPMExtensionFileOpenPickerInfo;

#endif 	/* __IPMExtensionFileOpenPickerInfo_FWD_DEFINED__ */


#ifndef __IPMExtensionFileSavePickerInfo_FWD_DEFINED__
#define __IPMExtensionFileSavePickerInfo_FWD_DEFINED__
typedef interface IPMExtensionFileSavePickerInfo IPMExtensionFileSavePickerInfo;

#endif 	/* __IPMExtensionFileSavePickerInfo_FWD_DEFINED__ */


#ifndef __IPMExtensionCachedFileUpdaterInfo_FWD_DEFINED__
#define __IPMExtensionCachedFileUpdaterInfo_FWD_DEFINED__
typedef interface IPMExtensionCachedFileUpdaterInfo IPMExtensionCachedFileUpdaterInfo;

#endif 	/* __IPMExtensionCachedFileUpdaterInfo_FWD_DEFINED__ */


#ifndef __IPMExtensionInfoEnumerator_FWD_DEFINED__
#define __IPMExtensionInfoEnumerator_FWD_DEFINED__
typedef interface IPMExtensionInfoEnumerator IPMExtensionInfoEnumerator;

#endif 	/* __IPMExtensionInfoEnumerator_FWD_DEFINED__ */


#ifndef __IPMBackgroundServiceAgentInfo_FWD_DEFINED__
#define __IPMBackgroundServiceAgentInfo_FWD_DEFINED__
typedef interface IPMBackgroundServiceAgentInfo IPMBackgroundServiceAgentInfo;

#endif 	/* __IPMBackgroundServiceAgentInfo_FWD_DEFINED__ */


#ifndef __IPMBackgroundWorkerInfo_FWD_DEFINED__
#define __IPMBackgroundWorkerInfo_FWD_DEFINED__
typedef interface IPMBackgroundWorkerInfo IPMBackgroundWorkerInfo;

#endif 	/* __IPMBackgroundWorkerInfo_FWD_DEFINED__ */


#ifndef __IPMBackgroundServiceAgentInfoEnumerator_FWD_DEFINED__
#define __IPMBackgroundServiceAgentInfoEnumerator_FWD_DEFINED__
typedef interface IPMBackgroundServiceAgentInfoEnumerator IPMBackgroundServiceAgentInfoEnumerator;

#endif 	/* __IPMBackgroundServiceAgentInfoEnumerator_FWD_DEFINED__ */


#ifndef __IPMBackgroundWorkerInfoEnumerator_FWD_DEFINED__
#define __IPMBackgroundWorkerInfoEnumerator_FWD_DEFINED__
typedef interface IPMBackgroundWorkerInfoEnumerator IPMBackgroundWorkerInfoEnumerator;

#endif 	/* __IPMBackgroundWorkerInfoEnumerator_FWD_DEFINED__ */


#ifndef __PMSvc_FWD_DEFINED__
#define __PMSvc_FWD_DEFINED__

#ifdef __cplusplus
typedef class PMSvc PMSvc;
#else
typedef struct PMSvc PMSvc;
#endif /* __cplusplus */

#endif 	/* __PMSvc_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_pacmanclientapi_0000_0000 */
/* [local] */ 

//
// Copyright (c) Microsoft Corporation.  All rights reserved.
//
//
// Copyright (c) Microsoft Corporation.  All rights reserved.
//
//
#pragma once
typedef GUID PRODUCTID;

typedef /* [v1_enum] */ 
enum _tagTILE_TEMPLATE_TYPE
    {
        TILE_TEMPLATE_INVALID	= 0,
        TILE_TEMPLATE_FLIP	= 0x5,
        TILE_TEMPLATE_DEEPLINK	= 0xd,
        TILE_TEMPLATE_CYCLE	= 0xe,
        TILE_TEMPLATE_METROCOUNT	= 0x1,
        TILE_TEMPLATE_AGILESTORE	= 0x2,
        TILE_TEMPLATE_GAMES	= 0x3,
        TILE_TEMPLATE_CALENDAR	= 0x4,
        TILE_TEMPLATE_MUSICVIDEO	= 0x7,
        TILE_TEMPLATE_PEOPLE	= 0xa,
        TILE_TEMPLATE_CONTACT	= 0xb,
        TILE_TEMPLATE_GROUP	= 0xc,
        TILE_TEMPLATE_DEFAULT	= 0xf,
        TILE_TEMPLATE_BADGE	= 0x10,
        TILE_TEMPLATE_BLOCK	= 0x11,
        TILE_TEMPLATE_TEXT01	= 0x12,
        TILE_TEMPLATE_TEXT02	= 0x13,
        TILE_TEMPLATE_TEXT03	= 0x14,
        TILE_TEMPLATE_TEXT04	= 0x15,
        TILE_TEMPLATE_TEXT05	= 0x16,
        TILE_TEMPLATE_TEXT06	= 0x17,
        TILE_TEMPLATE_TEXT07	= 0x18,
        TILE_TEMPLATE_TEXT08	= 0x19,
        TILE_TEMPLATE_TEXT09	= 0x1a,
        TILE_TEMPLATE_TEXT10	= 0x1b,
        TILE_TEMPLATE_TEXT11	= 0x1c,
        TILE_TEMPLATE_IMAGE	= 0x1d,
        TILE_TEMPLATE_IMAGECOLLECTION	= 0x1e,
        TILE_TEMPLATE_IMAGEANDTEXT01	= 0x1f,
        TILE_TEMPLATE_IMAGEANDTEXT02	= 0x20,
        TILE_TEMPLATE_BLOCKANDTEXT01	= 0x21,
        TILE_TEMPLATE_BLOCKANDTEXT02	= 0x22,
        TILE_TEMPLATE_PEEKIMAGEANDTEXT01	= 0x23,
        TILE_TEMPLATE_PEEKIMAGEANDTEXT02	= 0x24,
        TILE_TEMPLATE_PEEKIMAGEANDTEXT03	= 0x25,
        TILE_TEMPLATE_PEEKIMAGEANDTEXT04	= 0x26,
        TILE_TEMPLATE_PEEKIMAGE01	= 0x27,
        TILE_TEMPLATE_PEEKIMAGE02	= 0x28,
        TILE_TEMPLATE_PEEKIMAGE03	= 0x29,
        TILE_TEMPLATE_PEEKIMAGE04	= 0x2a,
        TILE_TEMPLATE_PEEKIMAGE05	= 0x2b,
        TILE_TEMPLATE_PEEKIMAGE06	= 0x2c,
        TILE_TEMPLATE_PEEKIMAGECOLLECTION01	= 0x2d,
        TILE_TEMPLATE_PEEKIMAGECOLLECTION02	= 0x2e,
        TILE_TEMPLATE_PEEKIMAGECOLLECTION03	= 0x2f,
        TILE_TEMPLATE_PEEKIMAGECOLLECTION04	= 0x30,
        TILE_TEMPLATE_PEEKIMAGECOLLECTION05	= 0x31,
        TILE_TEMPLATE_PEEKIMAGECOLLECTION06	= 0x32,
        TILE_TEMPLATE_SMALLIMAGEANDTEXT01	= 0x33,
        TILE_TEMPLATE_SMALLIMAGEANDTEXT02	= 0x34,
        TILE_TEMPLATE_SMALLIMAGEANDTEXT03	= 0x35,
        TILE_TEMPLATE_SMALLIMAGEANDTEXT04	= 0x36,
        TILE_TEMPLATE_SMALLIMAGEANDTEXT05	= 0x37,
        TILE_TEMPLATE_METROCOUNTQUEUE	= 0x38,
        TILE_TEMPLATE_SEARCH	= 0x39,
        TILE_TEMPLATE_TILEFLYOUT01	= 0x3a,
        TILE_TEMPLATE_FOLDER	= 0x3b,
        TILE_TEMPLATE_ALL	= 0x64
    } 	TILE_TEMPLATE_TYPE;

typedef /* [v1_enum] */ 
enum PM_APP_GENRE
    {
        PM_APP_GENRE_GAMES	= 0,
        PM_APP_GENRE_OTHER	= 0x1,
        PM_APP_GENRE_INVALID	= ( PM_APP_GENRE_OTHER + 1 ) 
    } 	PM_APP_GENRE;

typedef /* [v1_enum] */ 
enum _PM_APPLICATION_INSTALL_TYPE
    {
        PM_APPLICATION_INSTALL_NORMAL	= 0,
        PM_APPLICATION_INSTALL_IN_ROM	= 0x1,
        PM_APPLICATION_INSTALL_PA	= 0x2,
        PM_APPLICATION_INSTALL_DEBUG	= 0x3,
        PM_APPLICATION_INSTALL_ENTERPRISE	= 0x4,
        PM_APPLICATION_INSTALL_INVALID	= ( PM_APPLICATION_INSTALL_ENTERPRISE + 1 ) 
    } 	PM_APPLICATION_INSTALL_TYPE;

typedef /* [v1_enum] */ 
enum _PM_APPLICATION_STATE
    {
        PM_APPLICATION_STATE_MIN	= 0,
        PM_APPLICATION_STATE_INSTALLED	= ( PM_APPLICATION_STATE_MIN + 1 ) ,
        PM_APPLICATION_STATE_INSTALLING	= ( PM_APPLICATION_STATE_INSTALLED + 1 ) ,
        PM_APPLICATION_STATE_UPDATING	= ( PM_APPLICATION_STATE_INSTALLING + 1 ) ,
        PM_APPLICATION_STATE_UNINSTALLING	= ( PM_APPLICATION_STATE_UPDATING + 1 ) ,
        PM_APPLICATION_STATE_LICENSE_UPDATING	= ( PM_APPLICATION_STATE_UNINSTALLING + 1 ) ,
        PM_APPLICATION_STATE_MOVING	= ( PM_APPLICATION_STATE_LICENSE_UPDATING + 1 ) ,
        PM_APPLICATION_STATE_DISABLED_SD_CARD	= ( PM_APPLICATION_STATE_MOVING + 1 ) ,
        PM_APPLICATION_STATE_DISABLED_ENTERPRISE	= ( PM_APPLICATION_STATE_DISABLED_SD_CARD + 1 ) ,
        PM_APPLICATION_STATE_DISABLED_BACKING_UP	= ( PM_APPLICATION_STATE_DISABLED_ENTERPRISE + 1 ) ,
        PM_APPLICATION_STATE_DISABLED_MDIL_BINDING	= ( PM_APPLICATION_STATE_DISABLED_BACKING_UP + 1 ) ,
        PM_APPLICATION_STATE_MAX	= PM_APPLICATION_STATE_DISABLED_MDIL_BINDING,
        PM_APPLICATION_STATE_INVALID	= ( PM_APPLICATION_STATE_MAX + 1 ) 
    } 	PM_APPLICATION_STATE;

typedef /* [v1_enum] */ 
enum _PM_APPLICATION_HUBTYPE
    {
        PM_APPLICATION_HUBTYPE_NONMUSIC	= 0,
        PM_APPLICATION_HUBTYPE_MUSIC	= 0x1,
        PM_APPLICATION_HUBTYPE_INVALID	= ( PM_APPLICATION_HUBTYPE_MUSIC + 1 ) 
    } 	PM_APPLICATION_HUBTYPE;

typedef /* [v1_enum] */ 
enum _PM_TILE_HUBTYPE
    {
        PM_TILE_HUBTYPE_MUSIC	= 0x1,
        PM_TILE_HUBTYPE_MOSETTINGS	= 0x10000000,
        PM_TILE_HUBTYPE_GAMES	= 0x20000000,
        PM_TILE_HUBTYPE_APPLIST	= 0x40000000,
        PM_TILE_HUBTYPE_STARTMENU	= 0x80000000,
        PM_TILE_HUBTYPE_LOCKSCREEN	= 0x1000000,
        PM_TILE_HUBTYPE_KIDZONE	= 0x2000000,
        PM_TILE_HUBTYPE_CACHED	= 0x4000000,
        PM_TILE_HUBTYPE_INVALID	= ( PM_TILE_HUBTYPE_CACHED + 1 ) 
    } 	PM_TILE_HUBTYPE;

typedef /* [v1_enum] */ 
enum _PM_STARTTILE_TYPE
    {
        PM_STARTTILE_TYPE_PRIMARY	= 0x1,
        PM_STARTTILE_TYPE_SECONDARY	= 0x2,
        PM_STARTTILE_TYPE_APPLIST	= 0x3,
        PM_STARTTILE_TYPE_APPLISTPRIMARY	= 0x4,
        PM_STARTTILE_TYPE_INVALID	= ( PM_STARTTILE_TYPE_APPLISTPRIMARY + 1 ) 
    } 	PM_STARTTILE_TYPE;

typedef /* [v1_enum] */ 
enum _tagPM_TASK_TYPE
    {
        PM_TASK_TYPE_NORMAL	= 0,
        PM_TASK_TYPE_DEFAULT	= ( PM_TASK_TYPE_NORMAL + 1 ) ,
        PM_TASK_TYPE_SETTINGS	= ( PM_TASK_TYPE_DEFAULT + 1 ) ,
        PM_TASK_TYPE_BACKGROUNDSERVICEAGENT	= ( PM_TASK_TYPE_SETTINGS + 1 ) ,
        PM_TASK_TYPE_BACKGROUNDWORKER	= ( PM_TASK_TYPE_BACKGROUNDSERVICEAGENT + 1 ) ,
        PM_TASK_TYPE_INVALID	= ( PM_TASK_TYPE_BACKGROUNDWORKER + 1 ) 
    } 	PM_TASK_TYPE;

typedef /* [v1_enum] */ 
enum _PACKMAN_RUNTIME
    {
        PACKMAN_RUNTIME_NATIVE	= 1,
        PACKMAN_RUNTIME_SILVERLIGHTMOBILE	= 2,
        PACKMAN_RUNTIME_XNA	= 3,
        PACKMAN_RUNTIME_MODERN_NATIVE	= 4,
        PACKMAN_RUNTIME_JUPITER	= 5,
        PACKMAN_RUNTIME_INVALID	= ( PACKMAN_RUNTIME_JUPITER + 1 ) 
    } 	PACKMAN_RUNTIME;

typedef /* [v1_enum] */ 
enum _PM_ACTIVATION_POLICY
    {
        PM_ACTIVATION_POLICY_RESUME	= 0,
        PM_ACTIVATION_POLICY_RESUMESAMEPARAMS	= 1,
        PM_ACTIVATION_POLICY_REPLACE	= 2,
        PM_ACTIVATION_POLICY_REPLACESAMEPARAMS	= 3,
        PM_ACTIVATION_POLICY_MULTISESSION	= 4,
        PM_ACTIVATION_POLICY_REPLACE_IGNOREFOREGROUND	= 5,
        PM_ACTIVATION_POLICY_UNKNOWN	= 6,
        PM_ACTIVATION_POLICY_INVALID	= ( PM_ACTIVATION_POLICY_UNKNOWN + 1 ) 
    } 	PM_ACTIVATION_POLICY;

typedef /* [v1_enum] */ 
enum _PM_TASK_TRANSITION
    {
        PM_TASK_TRANSITION_DEFAULT	= 0,
        PM_TASK_TRANSITION_NONE	= 1,
        PM_TASK_TRANSITION_TURNSTILE	= 2,
        PM_TASK_TRANSITION_SLIDE	= 3,
        PM_TASK_TRANSITION_SWIVEL	= 4,
        PM_TASK_TRANSITION_READERBOARD	= 5,
        PM_TASK_TRANSITION_CUSTOM	= 6,
        PM_TASK_TRANSITION_INVALID	= ( PM_TASK_TRANSITION_CUSTOM + 1 ) 
    } 	PM_TASK_TRANSITION;

typedef /* [v1_enum] */ 
enum _tagPM_ENUM_APP_FILTER
    {
        PM_APP_FILTER_ALL	= 0,
        PM_APP_FILTER_VISIBLE	= ( PM_APP_FILTER_ALL + 1 ) ,
        PM_APP_FILTER_GENRE	= ( PM_APP_FILTER_VISIBLE + 1 ) ,
        PM_APP_FILTER_NONGAMES	= ( PM_APP_FILTER_GENRE + 1 ) ,
        PM_APP_FILTER_HUBTYPE	= ( PM_APP_FILTER_NONGAMES + 1 ) ,
        PM_APP_FILTER_PINABLEONKIDZONE	= ( PM_APP_FILTER_HUBTYPE + 1 ) ,
        PM_APP_FILTER_ALL_INCLUDE_MODERN	= ( PM_APP_FILTER_PINABLEONKIDZONE + 1 ) ,
        PM_APP_FILTER_FRAMEWORK	= ( PM_APP_FILTER_ALL_INCLUDE_MODERN + 1 ) ,
        PM_APP_FILTER_MAX	= ( PM_APP_FILTER_FRAMEWORK + 1 ) 
    } 	PM_ENUM_APP_FILTER;

typedef /* [v1_enum] */ 
enum _tagPM_ENUM_TILE_FILTER
    {
        PM_TILE_FILTER_APPLIST	= PM_APP_FILTER_MAX,
        PM_TILE_FILTER_PINNED	= ( PM_TILE_FILTER_APPLIST + 1 ) ,
        PM_TILE_FILTER_HUBTYPE	= ( PM_TILE_FILTER_PINNED + 1 ) ,
        PM_TILE_FILTER_APP_ALL	= ( PM_TILE_FILTER_HUBTYPE + 1 ) ,
        PM_TILE_FILTER_MAX	= ( PM_TILE_FILTER_APP_ALL + 1 ) 
    } 	PM_ENUM_TILE_FILTER;

typedef /* [v1_enum] */ 
enum _tagPM_ENUM_TASK_FILTER
    {
        PM_TASK_FILTER_APP_ALL	= PM_TILE_FILTER_MAX,
        PM_TASK_FILTER_TASK_TYPE	= ( PM_TASK_FILTER_APP_ALL + 1 ) ,
        PM_TASK_FILTER_DEHYD_SUPRESSING	= ( PM_TASK_FILTER_TASK_TYPE + 1 ) ,
        PM_TASK_FILTER_APP_TASK_TYPE	= ( PM_TASK_FILTER_DEHYD_SUPRESSING + 1 ) ,
        PM_TASK_FILTER_BGEXECUTION	= ( PM_TASK_FILTER_APP_TASK_TYPE + 1 ) ,
        PM_TASK_FILTER_MAX	= ( PM_TASK_FILTER_BGEXECUTION + 1 ) 
    } 	PM_ENUM_TASK_FILTER;

typedef /* [v1_enum] */ 
enum _tagPM_ENUM_EXTENSION_FILTER
    {
        PM_ENUM_EXTENSION_FILTER_BY_CONSUMER	= PM_TASK_FILTER_MAX,
        PM_ENUM_EXTENSION_FILTER_APPCONNECT	= PM_ENUM_EXTENSION_FILTER_BY_CONSUMER,
        PM_ENUM_EXTENSION_FILTER_PROTOCOL_ALL	= ( PM_ENUM_EXTENSION_FILTER_APPCONNECT + 1 ) ,
        PM_ENUM_EXTENSION_FILTER_FTASSOC_FILETYPE_ALL	= ( PM_ENUM_EXTENSION_FILTER_PROTOCOL_ALL + 1 ) ,
        PM_ENUM_EXTENSION_FILTER_FTASSOC_CONTENTTYPE_ALL	= ( PM_ENUM_EXTENSION_FILTER_FTASSOC_FILETYPE_ALL + 1 ) ,
        PM_ENUM_EXTENSION_FILTER_FTASSOC_APPLICATION_ALL	= ( PM_ENUM_EXTENSION_FILTER_FTASSOC_CONTENTTYPE_ALL + 1 ) ,
        PM_ENUM_EXTENSION_FILTER_SHARETARGET_ALL	= ( PM_ENUM_EXTENSION_FILTER_FTASSOC_APPLICATION_ALL + 1 ) ,
        PM_ENUM_EXTENSION_FILTER_FILEOPENPICKER_ALL	= ( PM_ENUM_EXTENSION_FILTER_SHARETARGET_ALL + 1 ) ,
        PM_ENUM_EXTENSION_FILTER_FILESAVEPICKER_ALL	= ( PM_ENUM_EXTENSION_FILTER_FILEOPENPICKER_ALL + 1 ) ,
        PM_ENUM_EXTENSION_FILTER_CACHEDFILEUPDATER_ALL	= ( PM_ENUM_EXTENSION_FILTER_FILESAVEPICKER_ALL + 1 ) ,
        PM_ENUM_EXTENSION_FILTER_MAX	= ( PM_ENUM_EXTENSION_FILTER_CACHEDFILEUPDATER_ALL + 1 ) 
    } 	PM_ENUM_EXTENSION_FILTER;

typedef /* [v1_enum] */ 
enum _tagPM_ENUM_BSA_FILTER
    {
        PM_ENUM_BSA_FILTER_ALL	= PM_ENUM_EXTENSION_FILTER_MAX,
        PM_ENUM_BSA_FILTER_BY_TASKID	= ( PM_ENUM_BSA_FILTER_ALL + 1 ) ,
        PM_ENUM_BSA_FILTER_BY_PRODUCTID	= ( PM_ENUM_BSA_FILTER_BY_TASKID + 1 ) ,
        PM_ENUM_BSA_FILTER_BY_PERIODIC	= ( PM_ENUM_BSA_FILTER_BY_PRODUCTID + 1 ) ,
        PM_ENUM_BSA_FILTER_BY_ALL_LAUNCHONBOOT	= ( PM_ENUM_BSA_FILTER_BY_PERIODIC + 1 ) ,
        PM_ENUM_BSA_FILTER_MAX	= ( PM_ENUM_BSA_FILTER_BY_ALL_LAUNCHONBOOT + 1 ) 
    } 	PM_ENUM_BSA_FILTER;

typedef /* [v1_enum] */ 
enum _tagPM_ENUM_BW_FILTER
    {
        PM_ENUM_BW_FILTER_BOOTWORKER_ALL	= PM_ENUM_BSA_FILTER_MAX,
        PM_ENUM_BW_FILTER_BY_TASKID	= ( PM_ENUM_BW_FILTER_BOOTWORKER_ALL + 1 ) ,
        PM_ENUM_BW_FILTER_MAX	= ( PM_ENUM_BW_FILTER_BY_TASKID + 1 ) 
    } 	PM_ENUM_BW_FILTER;

typedef struct _tagAPPTASKTYPE
    {
    PRODUCTID ProductID;
    PM_TASK_TYPE TaskType;
    } 	PM_APPTASKTYPE;

typedef struct _tagPM_EXTENSIONCONSUMER
    {
    PRODUCTID ConsumerPID;
    /* [string] */ BSTR ExtensionID;
    } 	PM_EXTENSIONCONSUMER;

typedef struct _tagPM_BSATASKID
    {
    PRODUCTID ProductID;
    /* [string] */ BSTR TaskID;
    } 	PM_BSATASKID;

typedef struct _tagPM_BWTASKID
    {
    PRODUCTID ProductID;
    /* [string] */ BSTR TaskID;
    } 	PM_BWTASKID;

typedef struct _PM_ENUM_FILTER
    {
    int FilterType;
    /* [switch_is][switch_type] */ union 
        {
        /* [default] */ int Dummy;
        /* [case()] */ PM_APP_GENRE Genre;
        /* [case()] */ PM_APPLICATION_HUBTYPE AppHubType;
        /* [case()] */ PM_TILE_HUBTYPE HubType;
        /* [case()] */ PM_TASK_TYPE Tasktype;
        /* [case()] */ PRODUCTID TaskProductID;
        /* [case()] */ PRODUCTID TileProductID;
        /* [case()] */ PM_APPTASKTYPE AppTaskType;
        /* [case()] */ PM_EXTENSIONCONSUMER Consumer;
        /* [case()] */ PM_BSATASKID BSATask;
        /* [case()] */ PRODUCTID BSAProductID;
        /* [case()] */ PM_BWTASKID BWTask;
        /* [case()][string] */ BSTR ProtocolName;
        /* [case()][string] */ BSTR FileType;
        /* [case()][string] */ BSTR ContentType;
        /* [case()] */ PRODUCTID AppSupportedFileExtPID;
        /* [case()][string] */ BSTR ShareTargetFileType;
        } 	FilterParameter;
    } 	PM_ENUM_FILTER;

typedef /* [v1_enum] */ 
enum _tagPM_LIVETILE_RECURRENCE_TYPE
    {
        PM_LIVETILE_RECURRENCE_TYPE_INSTANT	= 0,
        PM_LIVETILE_RECURRENCE_TYPE_ONETIME	= ( PM_LIVETILE_RECURRENCE_TYPE_INSTANT + 1 ) ,
        PM_LIVETILE_RECURRENCE_TYPE_INTERVAL	= ( PM_LIVETILE_RECURRENCE_TYPE_ONETIME + 1 ) ,
        PM_LIVETILE_RECURRENCE_TYPE_MAX	= PM_LIVETILE_RECURRENCE_TYPE_INTERVAL
    } 	PM_LIVETILE_RECURRENCE_TYPE;

typedef /* [v1_enum] */ 
enum _tagPM_TILE_SIZE
    {
        PM_TILE_SIZE_SMALL	= 0,
        PM_TILE_SIZE_MEDIUM	= ( PM_TILE_SIZE_SMALL + 1 ) ,
        PM_TILE_SIZE_LARGE	= ( PM_TILE_SIZE_MEDIUM + 1 ) ,
        PM_TILE_SIZE_SQUARE310X310	= ( PM_TILE_SIZE_LARGE + 1 ) ,
        PM_TILE_SIZE_TALL150X310	= ( PM_TILE_SIZE_SQUARE310X310 + 1 ) ,
        PM_TILE_SIZE_INVALID	= ( PM_TILE_SIZE_TALL150X310 + 1 ) 
    } 	PM_TILE_SIZE;

typedef /* [v1_enum] */ 
enum _tagPM_LOGO_SIZE
    {
        PM_LOGO_SIZE_SMALL	= 0,
        PM_LOGO_SIZE_MEDIUM	= ( PM_LOGO_SIZE_SMALL + 1 ) ,
        PM_LOGO_SIZE_LARGE	= ( PM_LOGO_SIZE_MEDIUM + 1 ) ,
        PM_LOGO_SIZE_INVALID	= ( PM_LOGO_SIZE_LARGE + 1 ) 
    } 	PM_LOGO_SIZE;

typedef struct _tagPM_STARTAPPBLOB
    {
    ULONG cbSize;
    GUID ProductID;
    /* [string] */ BSTR AppTitle;
    /* [string] */ BSTR IconPath;
    BOOL IsUninstallable;
    PM_APPLICATION_INSTALL_TYPE AppInstallType;
    GUID InstanceID;
    PM_APPLICATION_STATE State;
    BOOL IsModern;
    BOOL IsModernLightUp;
    USHORT LightUpSupportMask;
    } 	PM_STARTAPPBLOB;

typedef struct _tagPM_INVOCATIONINFO
    {
    /* [string] */ BSTR URIBaseOrAUMID;
    /* [string] */ BSTR URIFragmentOrArgs;
    } 	PM_INVOCATIONINFO;

typedef struct _tagPM_STARTTILEBLOB
    {
    ULONG cbSize;
    PRODUCTID ProductID;
    /* [string] */ BSTR TileID;
    TILE_TEMPLATE_TYPE TemplateType;
    ULONG HubPosition[ 32 ];
    ULONG HubVisibilityBitmask;
    BOOL IsDefault;
    PM_STARTTILE_TYPE TileType;
    /* [unique][size_is] */ BYTE *pbPropBlob;
    DWORD cbPropBlob;
    BOOL IsRestoring;
    BOOL IsModern;
    PM_INVOCATIONINFO InvocationInfo;
    } 	PM_STARTTILEBLOB;

typedef struct _tagPM_INSTALLINFO
    {
    PRODUCTID ProductID;
    /* [string] */ BSTR PackagePath;
    GUID InstanceID;
    /* [unique][size_is] */ BYTE *pbLicense;
    DWORD cbLicense;
    BOOL IsUninstallDisabled;
    DWORD DeploymentOptions;
    GUID OfferID;
    /* [unique][string] */ BSTR MarketplaceAppVersion;
    } 	PM_INSTALLINFO;

typedef struct _tagPM_UPDATEINFO_LEGACY
    {
    PRODUCTID ProductID;
    /* [string] */ BSTR PackagePath;
    GUID InstanceID;
    /* [unique][size_is] */ BYTE *pbLicense;
    DWORD cbLicense;
    /* [unique][string] */ BSTR MarketplaceAppVersion;
    } 	PM_UPDATEINFO_LEGACY;

typedef struct _tagPM_UPDATEINFO
    {
    PRODUCTID ProductID;
    /* [string] */ BSTR PackagePath;
    GUID InstanceID;
    /* [unique][size_is] */ BYTE *pbLicense;
    DWORD cbLicense;
    /* [unique][string] */ BSTR MarketplaceAppVersion;
    DWORD DeploymentOptions;
    } 	PM_UPDATEINFO;















extern RPC_IF_HANDLE __MIDL_itf_pacmanclientapi_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_pacmanclientapi_0000_0000_v0_0_s_ifspec;

#ifndef __IPMApplicationInfo_INTERFACE_DEFINED__
#define __IPMApplicationInfo_INTERFACE_DEFINED__

/* interface IPMApplicationInfo */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IPMApplicationInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("50AFB58A-438C-4088-9789-F8C4899829C7")
    IPMApplicationInfo : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE get_ProductID( 
            /* [out] */ __RPC__out PRODUCTID *pProductID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_InstanceID( 
            /* [out] */ __RPC__out GUID *pInstanceID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_OfferID( 
            /* [out] */ __RPC__out GUID *pOfferID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_DefaultTask( 
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pDefaultTask) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_AppTitle( 
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pAppTitle) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_IconPath( 
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pAppIconPath) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_NotificationState( 
            /* [out] */ __RPC__out BOOL *pIsNotified) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_AppInstallType( 
            /* [out] */ __RPC__out PM_APPLICATION_INSTALL_TYPE *pAppInstallType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_State( 
            /* [out] */ __RPC__out PM_APPLICATION_STATE *pState) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_IsRevoked( 
            /* [out] */ __RPC__out BOOL *pIsRevoked) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_UpdateAvailable( 
            /* [out] */ __RPC__out BOOL *pIsUpdateAvailable) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_InstallDate( 
            /* [out] */ __RPC__out FILETIME *pInstallDate) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_IsUninstallable( 
            /* [out] */ __RPC__out BOOL *pIsUninstallable) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_IsThemable( 
            /* [out] */ __RPC__out BOOL *pIsThemable) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_IsTrial( 
            /* [out] */ __RPC__out BOOL *pIsTrial) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_InstallPath( 
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pInstallPath) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_DataRoot( 
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pDataRoot) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_Genre( 
            /* [out] */ __RPC__out PM_APP_GENRE *pGenre) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_Publisher( 
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pPublisher) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_Author( 
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pAuthor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_Description( 
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pDescription) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_Version( 
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pVersion) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_InvocationInfo( 
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pImageUrn,
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pParameters) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_AppPlatMajorVersion( 
            /* [out] */ __RPC__out BYTE *pMajorVer) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_AppPlatMinorVersion( 
            /* [out] */ __RPC__out BYTE *pMinorVer) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_PublisherID( 
            /* [out] */ __RPC__out GUID *pPublisherID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_IsMultiCore( 
            /* [out] */ __RPC__out BOOL *pIsMultiCore) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_SID( 
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pSID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_AppPlatMajorVersionLightUp( 
            /* [out] */ __RPC__out BYTE *pMajorVer) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_AppPlatMinorVersionLightUp( 
            /* [out] */ __RPC__out BYTE *pMinorVer) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE set_UpdateAvailable( 
            /* [in] */ BOOL IsUpdateAvailable) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE set_NotificationState( 
            /* [in] */ BOOL IsNotified) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE set_IconPath( 
            /* [string][in] */ __RPC__in_string BSTR AppIconPath) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE set_UninstallableState( 
            /* [in] */ BOOL IsUninstallable) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_IsPinableOnKidZone( 
            /* [out] */ __RPC__out BOOL *pIsPinable) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_IsOriginallyPreInstalled( 
            /* [out] */ __RPC__out BOOL *pIsPreinstalled) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_IsInstallOnSD( 
            /* [out] */ __RPC__out BOOL *pIsInstallOnSD) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_IsOptoutOnSD( 
            /* [out] */ __RPC__out BOOL *pIsOptoutOnSD) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_IsOptoutBackupRestore( 
            /* [out] */ __RPC__out BOOL *pIsOptoutBackupRestore) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE set_EnterpriseDisabled( 
            /* [in] */ BOOL IsDisabled) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE set_EnterpriseUninstallable( 
            /* [in] */ BOOL IsUninstallable) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_EnterpriseDisabled( 
            /* [out] */ __RPC__out BOOL *IsDisabled) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_EnterpriseUninstallable( 
            /* [out] */ __RPC__out BOOL *IsUninstallable) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_IsVisibleOnAppList( 
            /* [out] */ __RPC__out BOOL *pIsVisible) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_IsInboxApp( 
            /* [out] */ __RPC__out BOOL *pIsInboxApp) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_StorageID( 
            /* [out] */ __RPC__out GUID *pStorageID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_StartAppBlob( 
            /* [out][in] */ __RPC__inout PM_STARTAPPBLOB *pBlob) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_IsMovable( 
            /* [out] */ __RPC__out BOOL *pIsMovable) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_DeploymentAppEnumerationHubFilter( 
            /* [out] */ __RPC__out PM_TILE_HUBTYPE *HubType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_ModifiedDate( 
            /* [out] */ __RPC__out FILETIME *pModifiedDate) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_IsOriginallyRestored( 
            /* [out] */ __RPC__out BOOL *pIsRestored) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_ShouldDeferMdilBind( 
            /* [out] */ __RPC__out BOOL *pfDeferMdilBind) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_IsFullyPreInstall( 
            /* [out] */ __RPC__out BOOL *pfIsFullyPreInstall) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE set_IsMdilMaintenanceNeeded( 
            /* [in] */ BOOL fIsMdilMaintenanceNeeded) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE set_Title( 
            /* [string][in] */ __RPC__in_string BSTR AppTitle) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPMApplicationInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPMApplicationInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPMApplicationInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPMApplicationInfo * This);
        
        DECLSPEC_XFGVIRT(IPMApplicationInfo, get_ProductID)
        HRESULT ( STDMETHODCALLTYPE *get_ProductID )( 
            __RPC__in IPMApplicationInfo * This,
            /* [out] */ __RPC__out PRODUCTID *pProductID);
        
        DECLSPEC_XFGVIRT(IPMApplicationInfo, get_InstanceID)
        HRESULT ( STDMETHODCALLTYPE *get_InstanceID )( 
            __RPC__in IPMApplicationInfo * This,
            /* [out] */ __RPC__out GUID *pInstanceID);
        
        DECLSPEC_XFGVIRT(IPMApplicationInfo, get_OfferID)
        HRESULT ( STDMETHODCALLTYPE *get_OfferID )( 
            __RPC__in IPMApplicationInfo * This,
            /* [out] */ __RPC__out GUID *pOfferID);
        
        DECLSPEC_XFGVIRT(IPMApplicationInfo, get_DefaultTask)
        HRESULT ( STDMETHODCALLTYPE *get_DefaultTask )( 
            __RPC__in IPMApplicationInfo * This,
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pDefaultTask);
        
        DECLSPEC_XFGVIRT(IPMApplicationInfo, get_AppTitle)
        HRESULT ( STDMETHODCALLTYPE *get_AppTitle )( 
            __RPC__in IPMApplicationInfo * This,
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pAppTitle);
        
        DECLSPEC_XFGVIRT(IPMApplicationInfo, get_IconPath)
        HRESULT ( STDMETHODCALLTYPE *get_IconPath )( 
            __RPC__in IPMApplicationInfo * This,
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pAppIconPath);
        
        DECLSPEC_XFGVIRT(IPMApplicationInfo, get_NotificationState)
        HRESULT ( STDMETHODCALLTYPE *get_NotificationState )( 
            __RPC__in IPMApplicationInfo * This,
            /* [out] */ __RPC__out BOOL *pIsNotified);
        
        DECLSPEC_XFGVIRT(IPMApplicationInfo, get_AppInstallType)
        HRESULT ( STDMETHODCALLTYPE *get_AppInstallType )( 
            __RPC__in IPMApplicationInfo * This,
            /* [out] */ __RPC__out PM_APPLICATION_INSTALL_TYPE *pAppInstallType);
        
        DECLSPEC_XFGVIRT(IPMApplicationInfo, get_State)
        HRESULT ( STDMETHODCALLTYPE *get_State )( 
            __RPC__in IPMApplicationInfo * This,
            /* [out] */ __RPC__out PM_APPLICATION_STATE *pState);
        
        DECLSPEC_XFGVIRT(IPMApplicationInfo, get_IsRevoked)
        HRESULT ( STDMETHODCALLTYPE *get_IsRevoked )( 
            __RPC__in IPMApplicationInfo * This,
            /* [out] */ __RPC__out BOOL *pIsRevoked);
        
        DECLSPEC_XFGVIRT(IPMApplicationInfo, get_UpdateAvailable)
        HRESULT ( STDMETHODCALLTYPE *get_UpdateAvailable )( 
            __RPC__in IPMApplicationInfo * This,
            /* [out] */ __RPC__out BOOL *pIsUpdateAvailable);
        
        DECLSPEC_XFGVIRT(IPMApplicationInfo, get_InstallDate)
        HRESULT ( STDMETHODCALLTYPE *get_InstallDate )( 
            __RPC__in IPMApplicationInfo * This,
            /* [out] */ __RPC__out FILETIME *pInstallDate);
        
        DECLSPEC_XFGVIRT(IPMApplicationInfo, get_IsUninstallable)
        HRESULT ( STDMETHODCALLTYPE *get_IsUninstallable )( 
            __RPC__in IPMApplicationInfo * This,
            /* [out] */ __RPC__out BOOL *pIsUninstallable);
        
        DECLSPEC_XFGVIRT(IPMApplicationInfo, get_IsThemable)
        HRESULT ( STDMETHODCALLTYPE *get_IsThemable )( 
            __RPC__in IPMApplicationInfo * This,
            /* [out] */ __RPC__out BOOL *pIsThemable);
        
        DECLSPEC_XFGVIRT(IPMApplicationInfo, get_IsTrial)
        HRESULT ( STDMETHODCALLTYPE *get_IsTrial )( 
            __RPC__in IPMApplicationInfo * This,
            /* [out] */ __RPC__out BOOL *pIsTrial);
        
        DECLSPEC_XFGVIRT(IPMApplicationInfo, get_InstallPath)
        HRESULT ( STDMETHODCALLTYPE *get_InstallPath )( 
            __RPC__in IPMApplicationInfo * This,
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pInstallPath);
        
        DECLSPEC_XFGVIRT(IPMApplicationInfo, get_DataRoot)
        HRESULT ( STDMETHODCALLTYPE *get_DataRoot )( 
            __RPC__in IPMApplicationInfo * This,
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pDataRoot);
        
        DECLSPEC_XFGVIRT(IPMApplicationInfo, get_Genre)
        HRESULT ( STDMETHODCALLTYPE *get_Genre )( 
            __RPC__in IPMApplicationInfo * This,
            /* [out] */ __RPC__out PM_APP_GENRE *pGenre);
        
        DECLSPEC_XFGVIRT(IPMApplicationInfo, get_Publisher)
        HRESULT ( STDMETHODCALLTYPE *get_Publisher )( 
            __RPC__in IPMApplicationInfo * This,
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pPublisher);
        
        DECLSPEC_XFGVIRT(IPMApplicationInfo, get_Author)
        HRESULT ( STDMETHODCALLTYPE *get_Author )( 
            __RPC__in IPMApplicationInfo * This,
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pAuthor);
        
        DECLSPEC_XFGVIRT(IPMApplicationInfo, get_Description)
        HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in IPMApplicationInfo * This,
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pDescription);
        
        DECLSPEC_XFGVIRT(IPMApplicationInfo, get_Version)
        HRESULT ( STDMETHODCALLTYPE *get_Version )( 
            __RPC__in IPMApplicationInfo * This,
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pVersion);
        
        DECLSPEC_XFGVIRT(IPMApplicationInfo, get_InvocationInfo)
        HRESULT ( STDMETHODCALLTYPE *get_InvocationInfo )( 
            __RPC__in IPMApplicationInfo * This,
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pImageUrn,
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pParameters);
        
        DECLSPEC_XFGVIRT(IPMApplicationInfo, get_AppPlatMajorVersion)
        HRESULT ( STDMETHODCALLTYPE *get_AppPlatMajorVersion )( 
            __RPC__in IPMApplicationInfo * This,
            /* [out] */ __RPC__out BYTE *pMajorVer);
        
        DECLSPEC_XFGVIRT(IPMApplicationInfo, get_AppPlatMinorVersion)
        HRESULT ( STDMETHODCALLTYPE *get_AppPlatMinorVersion )( 
            __RPC__in IPMApplicationInfo * This,
            /* [out] */ __RPC__out BYTE *pMinorVer);
        
        DECLSPEC_XFGVIRT(IPMApplicationInfo, get_PublisherID)
        HRESULT ( STDMETHODCALLTYPE *get_PublisherID )( 
            __RPC__in IPMApplicationInfo * This,
            /* [out] */ __RPC__out GUID *pPublisherID);
        
        DECLSPEC_XFGVIRT(IPMApplicationInfo, get_IsMultiCore)
        HRESULT ( STDMETHODCALLTYPE *get_IsMultiCore )( 
            __RPC__in IPMApplicationInfo * This,
            /* [out] */ __RPC__out BOOL *pIsMultiCore);
        
        DECLSPEC_XFGVIRT(IPMApplicationInfo, get_SID)
        HRESULT ( STDMETHODCALLTYPE *get_SID )( 
            __RPC__in IPMApplicationInfo * This,
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pSID);
        
        DECLSPEC_XFGVIRT(IPMApplicationInfo, get_AppPlatMajorVersionLightUp)
        HRESULT ( STDMETHODCALLTYPE *get_AppPlatMajorVersionLightUp )( 
            __RPC__in IPMApplicationInfo * This,
            /* [out] */ __RPC__out BYTE *pMajorVer);
        
        DECLSPEC_XFGVIRT(IPMApplicationInfo, get_AppPlatMinorVersionLightUp)
        HRESULT ( STDMETHODCALLTYPE *get_AppPlatMinorVersionLightUp )( 
            __RPC__in IPMApplicationInfo * This,
            /* [out] */ __RPC__out BYTE *pMinorVer);
        
        DECLSPEC_XFGVIRT(IPMApplicationInfo, set_UpdateAvailable)
        HRESULT ( STDMETHODCALLTYPE *set_UpdateAvailable )( 
            __RPC__in IPMApplicationInfo * This,
            /* [in] */ BOOL IsUpdateAvailable);
        
        DECLSPEC_XFGVIRT(IPMApplicationInfo, set_NotificationState)
        HRESULT ( STDMETHODCALLTYPE *set_NotificationState )( 
            __RPC__in IPMApplicationInfo * This,
            /* [in] */ BOOL IsNotified);
        
        DECLSPEC_XFGVIRT(IPMApplicationInfo, set_IconPath)
        HRESULT ( STDMETHODCALLTYPE *set_IconPath )( 
            __RPC__in IPMApplicationInfo * This,
            /* [string][in] */ __RPC__in_string BSTR AppIconPath);
        
        DECLSPEC_XFGVIRT(IPMApplicationInfo, set_UninstallableState)
        HRESULT ( STDMETHODCALLTYPE *set_UninstallableState )( 
            __RPC__in IPMApplicationInfo * This,
            /* [in] */ BOOL IsUninstallable);
        
        DECLSPEC_XFGVIRT(IPMApplicationInfo, get_IsPinableOnKidZone)
        HRESULT ( STDMETHODCALLTYPE *get_IsPinableOnKidZone )( 
            __RPC__in IPMApplicationInfo * This,
            /* [out] */ __RPC__out BOOL *pIsPinable);
        
        DECLSPEC_XFGVIRT(IPMApplicationInfo, get_IsOriginallyPreInstalled)
        HRESULT ( STDMETHODCALLTYPE *get_IsOriginallyPreInstalled )( 
            __RPC__in IPMApplicationInfo * This,
            /* [out] */ __RPC__out BOOL *pIsPreinstalled);
        
        DECLSPEC_XFGVIRT(IPMApplicationInfo, get_IsInstallOnSD)
        HRESULT ( STDMETHODCALLTYPE *get_IsInstallOnSD )( 
            __RPC__in IPMApplicationInfo * This,
            /* [out] */ __RPC__out BOOL *pIsInstallOnSD);
        
        DECLSPEC_XFGVIRT(IPMApplicationInfo, get_IsOptoutOnSD)
        HRESULT ( STDMETHODCALLTYPE *get_IsOptoutOnSD )( 
            __RPC__in IPMApplicationInfo * This,
            /* [out] */ __RPC__out BOOL *pIsOptoutOnSD);
        
        DECLSPEC_XFGVIRT(IPMApplicationInfo, get_IsOptoutBackupRestore)
        HRESULT ( STDMETHODCALLTYPE *get_IsOptoutBackupRestore )( 
            __RPC__in IPMApplicationInfo * This,
            /* [out] */ __RPC__out BOOL *pIsOptoutBackupRestore);
        
        DECLSPEC_XFGVIRT(IPMApplicationInfo, set_EnterpriseDisabled)
        HRESULT ( STDMETHODCALLTYPE *set_EnterpriseDisabled )( 
            __RPC__in IPMApplicationInfo * This,
            /* [in] */ BOOL IsDisabled);
        
        DECLSPEC_XFGVIRT(IPMApplicationInfo, set_EnterpriseUninstallable)
        HRESULT ( STDMETHODCALLTYPE *set_EnterpriseUninstallable )( 
            __RPC__in IPMApplicationInfo * This,
            /* [in] */ BOOL IsUninstallable);
        
        DECLSPEC_XFGVIRT(IPMApplicationInfo, get_EnterpriseDisabled)
        HRESULT ( STDMETHODCALLTYPE *get_EnterpriseDisabled )( 
            __RPC__in IPMApplicationInfo * This,
            /* [out] */ __RPC__out BOOL *IsDisabled);
        
        DECLSPEC_XFGVIRT(IPMApplicationInfo, get_EnterpriseUninstallable)
        HRESULT ( STDMETHODCALLTYPE *get_EnterpriseUninstallable )( 
            __RPC__in IPMApplicationInfo * This,
            /* [out] */ __RPC__out BOOL *IsUninstallable);
        
        DECLSPEC_XFGVIRT(IPMApplicationInfo, get_IsVisibleOnAppList)
        HRESULT ( STDMETHODCALLTYPE *get_IsVisibleOnAppList )( 
            __RPC__in IPMApplicationInfo * This,
            /* [out] */ __RPC__out BOOL *pIsVisible);
        
        DECLSPEC_XFGVIRT(IPMApplicationInfo, get_IsInboxApp)
        HRESULT ( STDMETHODCALLTYPE *get_IsInboxApp )( 
            __RPC__in IPMApplicationInfo * This,
            /* [out] */ __RPC__out BOOL *pIsInboxApp);
        
        DECLSPEC_XFGVIRT(IPMApplicationInfo, get_StorageID)
        HRESULT ( STDMETHODCALLTYPE *get_StorageID )( 
            __RPC__in IPMApplicationInfo * This,
            /* [out] */ __RPC__out GUID *pStorageID);
        
        DECLSPEC_XFGVIRT(IPMApplicationInfo, get_StartAppBlob)
        HRESULT ( STDMETHODCALLTYPE *get_StartAppBlob )( 
            __RPC__in IPMApplicationInfo * This,
            /* [out][in] */ __RPC__inout PM_STARTAPPBLOB *pBlob);
        
        DECLSPEC_XFGVIRT(IPMApplicationInfo, get_IsMovable)
        HRESULT ( STDMETHODCALLTYPE *get_IsMovable )( 
            __RPC__in IPMApplicationInfo * This,
            /* [out] */ __RPC__out BOOL *pIsMovable);
        
        DECLSPEC_XFGVIRT(IPMApplicationInfo, get_DeploymentAppEnumerationHubFilter)
        HRESULT ( STDMETHODCALLTYPE *get_DeploymentAppEnumerationHubFilter )( 
            __RPC__in IPMApplicationInfo * This,
            /* [out] */ __RPC__out PM_TILE_HUBTYPE *HubType);
        
        DECLSPEC_XFGVIRT(IPMApplicationInfo, get_ModifiedDate)
        HRESULT ( STDMETHODCALLTYPE *get_ModifiedDate )( 
            __RPC__in IPMApplicationInfo * This,
            /* [out] */ __RPC__out FILETIME *pModifiedDate);
        
        DECLSPEC_XFGVIRT(IPMApplicationInfo, get_IsOriginallyRestored)
        HRESULT ( STDMETHODCALLTYPE *get_IsOriginallyRestored )( 
            __RPC__in IPMApplicationInfo * This,
            /* [out] */ __RPC__out BOOL *pIsRestored);
        
        DECLSPEC_XFGVIRT(IPMApplicationInfo, get_ShouldDeferMdilBind)
        HRESULT ( STDMETHODCALLTYPE *get_ShouldDeferMdilBind )( 
            __RPC__in IPMApplicationInfo * This,
            /* [out] */ __RPC__out BOOL *pfDeferMdilBind);
        
        DECLSPEC_XFGVIRT(IPMApplicationInfo, get_IsFullyPreInstall)
        HRESULT ( STDMETHODCALLTYPE *get_IsFullyPreInstall )( 
            __RPC__in IPMApplicationInfo * This,
            /* [out] */ __RPC__out BOOL *pfIsFullyPreInstall);
        
        DECLSPEC_XFGVIRT(IPMApplicationInfo, set_IsMdilMaintenanceNeeded)
        HRESULT ( STDMETHODCALLTYPE *set_IsMdilMaintenanceNeeded )( 
            __RPC__in IPMApplicationInfo * This,
            /* [in] */ BOOL fIsMdilMaintenanceNeeded);
        
        DECLSPEC_XFGVIRT(IPMApplicationInfo, set_Title)
        HRESULT ( STDMETHODCALLTYPE *set_Title )( 
            __RPC__in IPMApplicationInfo * This,
            /* [string][in] */ __RPC__in_string BSTR AppTitle);
        
        END_INTERFACE
    } IPMApplicationInfoVtbl;

    interface IPMApplicationInfo
    {
        CONST_VTBL struct IPMApplicationInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPMApplicationInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPMApplicationInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPMApplicationInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPMApplicationInfo_get_ProductID(This,pProductID)	\
    ( (This)->lpVtbl -> get_ProductID(This,pProductID) ) 

#define IPMApplicationInfo_get_InstanceID(This,pInstanceID)	\
    ( (This)->lpVtbl -> get_InstanceID(This,pInstanceID) ) 

#define IPMApplicationInfo_get_OfferID(This,pOfferID)	\
    ( (This)->lpVtbl -> get_OfferID(This,pOfferID) ) 

#define IPMApplicationInfo_get_DefaultTask(This,pDefaultTask)	\
    ( (This)->lpVtbl -> get_DefaultTask(This,pDefaultTask) ) 

#define IPMApplicationInfo_get_AppTitle(This,pAppTitle)	\
    ( (This)->lpVtbl -> get_AppTitle(This,pAppTitle) ) 

#define IPMApplicationInfo_get_IconPath(This,pAppIconPath)	\
    ( (This)->lpVtbl -> get_IconPath(This,pAppIconPath) ) 

#define IPMApplicationInfo_get_NotificationState(This,pIsNotified)	\
    ( (This)->lpVtbl -> get_NotificationState(This,pIsNotified) ) 

#define IPMApplicationInfo_get_AppInstallType(This,pAppInstallType)	\
    ( (This)->lpVtbl -> get_AppInstallType(This,pAppInstallType) ) 

#define IPMApplicationInfo_get_State(This,pState)	\
    ( (This)->lpVtbl -> get_State(This,pState) ) 

#define IPMApplicationInfo_get_IsRevoked(This,pIsRevoked)	\
    ( (This)->lpVtbl -> get_IsRevoked(This,pIsRevoked) ) 

#define IPMApplicationInfo_get_UpdateAvailable(This,pIsUpdateAvailable)	\
    ( (This)->lpVtbl -> get_UpdateAvailable(This,pIsUpdateAvailable) ) 

#define IPMApplicationInfo_get_InstallDate(This,pInstallDate)	\
    ( (This)->lpVtbl -> get_InstallDate(This,pInstallDate) ) 

#define IPMApplicationInfo_get_IsUninstallable(This,pIsUninstallable)	\
    ( (This)->lpVtbl -> get_IsUninstallable(This,pIsUninstallable) ) 

#define IPMApplicationInfo_get_IsThemable(This,pIsThemable)	\
    ( (This)->lpVtbl -> get_IsThemable(This,pIsThemable) ) 

#define IPMApplicationInfo_get_IsTrial(This,pIsTrial)	\
    ( (This)->lpVtbl -> get_IsTrial(This,pIsTrial) ) 

#define IPMApplicationInfo_get_InstallPath(This,pInstallPath)	\
    ( (This)->lpVtbl -> get_InstallPath(This,pInstallPath) ) 

#define IPMApplicationInfo_get_DataRoot(This,pDataRoot)	\
    ( (This)->lpVtbl -> get_DataRoot(This,pDataRoot) ) 

#define IPMApplicationInfo_get_Genre(This,pGenre)	\
    ( (This)->lpVtbl -> get_Genre(This,pGenre) ) 

#define IPMApplicationInfo_get_Publisher(This,pPublisher)	\
    ( (This)->lpVtbl -> get_Publisher(This,pPublisher) ) 

#define IPMApplicationInfo_get_Author(This,pAuthor)	\
    ( (This)->lpVtbl -> get_Author(This,pAuthor) ) 

#define IPMApplicationInfo_get_Description(This,pDescription)	\
    ( (This)->lpVtbl -> get_Description(This,pDescription) ) 

#define IPMApplicationInfo_get_Version(This,pVersion)	\
    ( (This)->lpVtbl -> get_Version(This,pVersion) ) 

#define IPMApplicationInfo_get_InvocationInfo(This,pImageUrn,pParameters)	\
    ( (This)->lpVtbl -> get_InvocationInfo(This,pImageUrn,pParameters) ) 

#define IPMApplicationInfo_get_AppPlatMajorVersion(This,pMajorVer)	\
    ( (This)->lpVtbl -> get_AppPlatMajorVersion(This,pMajorVer) ) 

#define IPMApplicationInfo_get_AppPlatMinorVersion(This,pMinorVer)	\
    ( (This)->lpVtbl -> get_AppPlatMinorVersion(This,pMinorVer) ) 

#define IPMApplicationInfo_get_PublisherID(This,pPublisherID)	\
    ( (This)->lpVtbl -> get_PublisherID(This,pPublisherID) ) 

#define IPMApplicationInfo_get_IsMultiCore(This,pIsMultiCore)	\
    ( (This)->lpVtbl -> get_IsMultiCore(This,pIsMultiCore) ) 

#define IPMApplicationInfo_get_SID(This,pSID)	\
    ( (This)->lpVtbl -> get_SID(This,pSID) ) 

#define IPMApplicationInfo_get_AppPlatMajorVersionLightUp(This,pMajorVer)	\
    ( (This)->lpVtbl -> get_AppPlatMajorVersionLightUp(This,pMajorVer) ) 

#define IPMApplicationInfo_get_AppPlatMinorVersionLightUp(This,pMinorVer)	\
    ( (This)->lpVtbl -> get_AppPlatMinorVersionLightUp(This,pMinorVer) ) 

#define IPMApplicationInfo_set_UpdateAvailable(This,IsUpdateAvailable)	\
    ( (This)->lpVtbl -> set_UpdateAvailable(This,IsUpdateAvailable) ) 

#define IPMApplicationInfo_set_NotificationState(This,IsNotified)	\
    ( (This)->lpVtbl -> set_NotificationState(This,IsNotified) ) 

#define IPMApplicationInfo_set_IconPath(This,AppIconPath)	\
    ( (This)->lpVtbl -> set_IconPath(This,AppIconPath) ) 

#define IPMApplicationInfo_set_UninstallableState(This,IsUninstallable)	\
    ( (This)->lpVtbl -> set_UninstallableState(This,IsUninstallable) ) 

#define IPMApplicationInfo_get_IsPinableOnKidZone(This,pIsPinable)	\
    ( (This)->lpVtbl -> get_IsPinableOnKidZone(This,pIsPinable) ) 

#define IPMApplicationInfo_get_IsOriginallyPreInstalled(This,pIsPreinstalled)	\
    ( (This)->lpVtbl -> get_IsOriginallyPreInstalled(This,pIsPreinstalled) ) 

#define IPMApplicationInfo_get_IsInstallOnSD(This,pIsInstallOnSD)	\
    ( (This)->lpVtbl -> get_IsInstallOnSD(This,pIsInstallOnSD) ) 

#define IPMApplicationInfo_get_IsOptoutOnSD(This,pIsOptoutOnSD)	\
    ( (This)->lpVtbl -> get_IsOptoutOnSD(This,pIsOptoutOnSD) ) 

#define IPMApplicationInfo_get_IsOptoutBackupRestore(This,pIsOptoutBackupRestore)	\
    ( (This)->lpVtbl -> get_IsOptoutBackupRestore(This,pIsOptoutBackupRestore) ) 

#define IPMApplicationInfo_set_EnterpriseDisabled(This,IsDisabled)	\
    ( (This)->lpVtbl -> set_EnterpriseDisabled(This,IsDisabled) ) 

#define IPMApplicationInfo_set_EnterpriseUninstallable(This,IsUninstallable)	\
    ( (This)->lpVtbl -> set_EnterpriseUninstallable(This,IsUninstallable) ) 

#define IPMApplicationInfo_get_EnterpriseDisabled(This,IsDisabled)	\
    ( (This)->lpVtbl -> get_EnterpriseDisabled(This,IsDisabled) ) 

#define IPMApplicationInfo_get_EnterpriseUninstallable(This,IsUninstallable)	\
    ( (This)->lpVtbl -> get_EnterpriseUninstallable(This,IsUninstallable) ) 

#define IPMApplicationInfo_get_IsVisibleOnAppList(This,pIsVisible)	\
    ( (This)->lpVtbl -> get_IsVisibleOnAppList(This,pIsVisible) ) 

#define IPMApplicationInfo_get_IsInboxApp(This,pIsInboxApp)	\
    ( (This)->lpVtbl -> get_IsInboxApp(This,pIsInboxApp) ) 

#define IPMApplicationInfo_get_StorageID(This,pStorageID)	\
    ( (This)->lpVtbl -> get_StorageID(This,pStorageID) ) 

#define IPMApplicationInfo_get_StartAppBlob(This,pBlob)	\
    ( (This)->lpVtbl -> get_StartAppBlob(This,pBlob) ) 

#define IPMApplicationInfo_get_IsMovable(This,pIsMovable)	\
    ( (This)->lpVtbl -> get_IsMovable(This,pIsMovable) ) 

#define IPMApplicationInfo_get_DeploymentAppEnumerationHubFilter(This,HubType)	\
    ( (This)->lpVtbl -> get_DeploymentAppEnumerationHubFilter(This,HubType) ) 

#define IPMApplicationInfo_get_ModifiedDate(This,pModifiedDate)	\
    ( (This)->lpVtbl -> get_ModifiedDate(This,pModifiedDate) ) 

#define IPMApplicationInfo_get_IsOriginallyRestored(This,pIsRestored)	\
    ( (This)->lpVtbl -> get_IsOriginallyRestored(This,pIsRestored) ) 

#define IPMApplicationInfo_get_ShouldDeferMdilBind(This,pfDeferMdilBind)	\
    ( (This)->lpVtbl -> get_ShouldDeferMdilBind(This,pfDeferMdilBind) ) 

#define IPMApplicationInfo_get_IsFullyPreInstall(This,pfIsFullyPreInstall)	\
    ( (This)->lpVtbl -> get_IsFullyPreInstall(This,pfIsFullyPreInstall) ) 

#define IPMApplicationInfo_set_IsMdilMaintenanceNeeded(This,fIsMdilMaintenanceNeeded)	\
    ( (This)->lpVtbl -> set_IsMdilMaintenanceNeeded(This,fIsMdilMaintenanceNeeded) ) 

#define IPMApplicationInfo_set_Title(This,AppTitle)	\
    ( (This)->lpVtbl -> set_Title(This,AppTitle) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPMApplicationInfo_INTERFACE_DEFINED__ */


#ifndef __IPMTilePropertyInfo_INTERFACE_DEFINED__
#define __IPMTilePropertyInfo_INTERFACE_DEFINED__

/* interface IPMTilePropertyInfo */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IPMTilePropertyInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6C2B8017-1EFA-42a7-86C0-6D4B640BF528")
    IPMTilePropertyInfo : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE get_PropertyID( 
            /* [out] */ __RPC__out ULONG *pPropID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_PropertyValue( 
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pPropValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE set_Property( 
            /* [string][in] */ __RPC__in_string BSTR PropValue) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPMTilePropertyInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPMTilePropertyInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPMTilePropertyInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPMTilePropertyInfo * This);
        
        DECLSPEC_XFGVIRT(IPMTilePropertyInfo, get_PropertyID)
        HRESULT ( STDMETHODCALLTYPE *get_PropertyID )( 
            __RPC__in IPMTilePropertyInfo * This,
            /* [out] */ __RPC__out ULONG *pPropID);
        
        DECLSPEC_XFGVIRT(IPMTilePropertyInfo, get_PropertyValue)
        HRESULT ( STDMETHODCALLTYPE *get_PropertyValue )( 
            __RPC__in IPMTilePropertyInfo * This,
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pPropValue);
        
        DECLSPEC_XFGVIRT(IPMTilePropertyInfo, set_Property)
        HRESULT ( STDMETHODCALLTYPE *set_Property )( 
            __RPC__in IPMTilePropertyInfo * This,
            /* [string][in] */ __RPC__in_string BSTR PropValue);
        
        END_INTERFACE
    } IPMTilePropertyInfoVtbl;

    interface IPMTilePropertyInfo
    {
        CONST_VTBL struct IPMTilePropertyInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPMTilePropertyInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPMTilePropertyInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPMTilePropertyInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPMTilePropertyInfo_get_PropertyID(This,pPropID)	\
    ( (This)->lpVtbl -> get_PropertyID(This,pPropID) ) 

#define IPMTilePropertyInfo_get_PropertyValue(This,pPropValue)	\
    ( (This)->lpVtbl -> get_PropertyValue(This,pPropValue) ) 

#define IPMTilePropertyInfo_set_Property(This,PropValue)	\
    ( (This)->lpVtbl -> set_Property(This,PropValue) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPMTilePropertyInfo_INTERFACE_DEFINED__ */


#ifndef __IPMTilePropertyEnumerator_INTERFACE_DEFINED__
#define __IPMTilePropertyEnumerator_INTERFACE_DEFINED__

/* interface IPMTilePropertyEnumerator */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IPMTilePropertyEnumerator;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("CC4CD629-9047-4250-AAC8-930E47812421")
    IPMTilePropertyEnumerator : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE get_Next( 
            /* [out] */ __RPC__deref_out_opt IPMTilePropertyInfo **ppPropInfo) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPMTilePropertyEnumeratorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPMTilePropertyEnumerator * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPMTilePropertyEnumerator * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPMTilePropertyEnumerator * This);
        
        DECLSPEC_XFGVIRT(IPMTilePropertyEnumerator, get_Next)
        HRESULT ( STDMETHODCALLTYPE *get_Next )( 
            __RPC__in IPMTilePropertyEnumerator * This,
            /* [out] */ __RPC__deref_out_opt IPMTilePropertyInfo **ppPropInfo);
        
        END_INTERFACE
    } IPMTilePropertyEnumeratorVtbl;

    interface IPMTilePropertyEnumerator
    {
        CONST_VTBL struct IPMTilePropertyEnumeratorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPMTilePropertyEnumerator_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPMTilePropertyEnumerator_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPMTilePropertyEnumerator_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPMTilePropertyEnumerator_get_Next(This,ppPropInfo)	\
    ( (This)->lpVtbl -> get_Next(This,ppPropInfo) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPMTilePropertyEnumerator_INTERFACE_DEFINED__ */


#ifndef __IPMTileInfo_INTERFACE_DEFINED__
#define __IPMTileInfo_INTERFACE_DEFINED__

/* interface IPMTileInfo */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IPMTileInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D1604833-2B08-4001-82CD-183AD734F752")
    IPMTileInfo : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE get_ProductID( 
            /* [out] */ __RPC__out PRODUCTID *pProductID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_TileID( 
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pTileID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_TemplateType( 
            /* [out] */ __RPC__out TILE_TEMPLATE_TYPE *pTemplateType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_HubPinnedState( 
            /* [in] */ PM_TILE_HUBTYPE HubType,
            /* [out] */ __RPC__out BOOL *pPinned) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_HubPosition( 
            /* [in] */ PM_TILE_HUBTYPE HubType,
            /* [out] */ __RPC__out ULONG *pPosition) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_IsNotified( 
            /* [out] */ __RPC__out BOOL *pIsNotified) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_IsDefault( 
            /* [out] */ __RPC__out BOOL *pIsDefault) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_TaskID( 
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pTaskID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_TileType( 
            /* [out] */ __RPC__out PM_STARTTILE_TYPE *pStartTileType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_IsThemable( 
            /* [out] */ __RPC__out BOOL *pIsThemable) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_PropertyById( 
            /* [in] */ ULONG PropID,
            /* [out] */ __RPC__deref_out_opt IPMTilePropertyInfo **ppPropInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_InvocationInfo( 
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pImageUrn,
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pParameters) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_PropertyEnum( 
            /* [out] */ __RPC__deref_out_opt IPMTilePropertyEnumerator **ppTilePropEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_HubTileSize( 
            /* [in] */ PM_TILE_HUBTYPE HubType,
            /* [out] */ __RPC__out PM_TILE_SIZE *pSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE set_HubPosition( 
            /* [in] */ PM_TILE_HUBTYPE HubType,
            /* [in] */ ULONG Position) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE set_NotifiedState( 
            /* [in] */ BOOL Notified) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE set_HubPinnedState( 
            /* [in] */ PM_TILE_HUBTYPE HubType,
            /* [in] */ BOOL Pinned) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE set_HubTileSize( 
            /* [in] */ PM_TILE_HUBTYPE HubType,
            /* [in] */ PM_TILE_SIZE Size) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE set_InvocationInfo( 
            /* [string][in] */ __RPC__in_string BSTR TaskName,
            /* [string][in] */ __RPC__in_string BSTR TaskParameters) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_StartTileBlob( 
            /* [out][in] */ __RPC__inout PM_STARTTILEBLOB *pBlob) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_IsRestoring( 
            /* [out] */ __RPC__out BOOL *pIsRestoring) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_IsAutoRestoreDisabled( 
            /* [out] */ __RPC__out BOOL *pIsAutoRestoreDisabled) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE set_IsRestoring( 
            /* [in] */ BOOL Restoring) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE set_IsAutoRestoreDisabled( 
            /* [in] */ BOOL AutoRestoreDisabled) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPMTileInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPMTileInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPMTileInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPMTileInfo * This);
        
        DECLSPEC_XFGVIRT(IPMTileInfo, get_ProductID)
        HRESULT ( STDMETHODCALLTYPE *get_ProductID )( 
            __RPC__in IPMTileInfo * This,
            /* [out] */ __RPC__out PRODUCTID *pProductID);
        
        DECLSPEC_XFGVIRT(IPMTileInfo, get_TileID)
        HRESULT ( STDMETHODCALLTYPE *get_TileID )( 
            __RPC__in IPMTileInfo * This,
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pTileID);
        
        DECLSPEC_XFGVIRT(IPMTileInfo, get_TemplateType)
        HRESULT ( STDMETHODCALLTYPE *get_TemplateType )( 
            __RPC__in IPMTileInfo * This,
            /* [out] */ __RPC__out TILE_TEMPLATE_TYPE *pTemplateType);
        
        DECLSPEC_XFGVIRT(IPMTileInfo, get_HubPinnedState)
        HRESULT ( STDMETHODCALLTYPE *get_HubPinnedState )( 
            __RPC__in IPMTileInfo * This,
            /* [in] */ PM_TILE_HUBTYPE HubType,
            /* [out] */ __RPC__out BOOL *pPinned);
        
        DECLSPEC_XFGVIRT(IPMTileInfo, get_HubPosition)
        HRESULT ( STDMETHODCALLTYPE *get_HubPosition )( 
            __RPC__in IPMTileInfo * This,
            /* [in] */ PM_TILE_HUBTYPE HubType,
            /* [out] */ __RPC__out ULONG *pPosition);
        
        DECLSPEC_XFGVIRT(IPMTileInfo, get_IsNotified)
        HRESULT ( STDMETHODCALLTYPE *get_IsNotified )( 
            __RPC__in IPMTileInfo * This,
            /* [out] */ __RPC__out BOOL *pIsNotified);
        
        DECLSPEC_XFGVIRT(IPMTileInfo, get_IsDefault)
        HRESULT ( STDMETHODCALLTYPE *get_IsDefault )( 
            __RPC__in IPMTileInfo * This,
            /* [out] */ __RPC__out BOOL *pIsDefault);
        
        DECLSPEC_XFGVIRT(IPMTileInfo, get_TaskID)
        HRESULT ( STDMETHODCALLTYPE *get_TaskID )( 
            __RPC__in IPMTileInfo * This,
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pTaskID);
        
        DECLSPEC_XFGVIRT(IPMTileInfo, get_TileType)
        HRESULT ( STDMETHODCALLTYPE *get_TileType )( 
            __RPC__in IPMTileInfo * This,
            /* [out] */ __RPC__out PM_STARTTILE_TYPE *pStartTileType);
        
        DECLSPEC_XFGVIRT(IPMTileInfo, get_IsThemable)
        HRESULT ( STDMETHODCALLTYPE *get_IsThemable )( 
            __RPC__in IPMTileInfo * This,
            /* [out] */ __RPC__out BOOL *pIsThemable);
        
        DECLSPEC_XFGVIRT(IPMTileInfo, get_PropertyById)
        HRESULT ( STDMETHODCALLTYPE *get_PropertyById )( 
            __RPC__in IPMTileInfo * This,
            /* [in] */ ULONG PropID,
            /* [out] */ __RPC__deref_out_opt IPMTilePropertyInfo **ppPropInfo);
        
        DECLSPEC_XFGVIRT(IPMTileInfo, get_InvocationInfo)
        HRESULT ( STDMETHODCALLTYPE *get_InvocationInfo )( 
            __RPC__in IPMTileInfo * This,
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pImageUrn,
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pParameters);
        
        DECLSPEC_XFGVIRT(IPMTileInfo, get_PropertyEnum)
        HRESULT ( STDMETHODCALLTYPE *get_PropertyEnum )( 
            __RPC__in IPMTileInfo * This,
            /* [out] */ __RPC__deref_out_opt IPMTilePropertyEnumerator **ppTilePropEnum);
        
        DECLSPEC_XFGVIRT(IPMTileInfo, get_HubTileSize)
        HRESULT ( STDMETHODCALLTYPE *get_HubTileSize )( 
            __RPC__in IPMTileInfo * This,
            /* [in] */ PM_TILE_HUBTYPE HubType,
            /* [out] */ __RPC__out PM_TILE_SIZE *pSize);
        
        DECLSPEC_XFGVIRT(IPMTileInfo, set_HubPosition)
        HRESULT ( STDMETHODCALLTYPE *set_HubPosition )( 
            __RPC__in IPMTileInfo * This,
            /* [in] */ PM_TILE_HUBTYPE HubType,
            /* [in] */ ULONG Position);
        
        DECLSPEC_XFGVIRT(IPMTileInfo, set_NotifiedState)
        HRESULT ( STDMETHODCALLTYPE *set_NotifiedState )( 
            __RPC__in IPMTileInfo * This,
            /* [in] */ BOOL Notified);
        
        DECLSPEC_XFGVIRT(IPMTileInfo, set_HubPinnedState)
        HRESULT ( STDMETHODCALLTYPE *set_HubPinnedState )( 
            __RPC__in IPMTileInfo * This,
            /* [in] */ PM_TILE_HUBTYPE HubType,
            /* [in] */ BOOL Pinned);
        
        DECLSPEC_XFGVIRT(IPMTileInfo, set_HubTileSize)
        HRESULT ( STDMETHODCALLTYPE *set_HubTileSize )( 
            __RPC__in IPMTileInfo * This,
            /* [in] */ PM_TILE_HUBTYPE HubType,
            /* [in] */ PM_TILE_SIZE Size);
        
        DECLSPEC_XFGVIRT(IPMTileInfo, set_InvocationInfo)
        HRESULT ( STDMETHODCALLTYPE *set_InvocationInfo )( 
            __RPC__in IPMTileInfo * This,
            /* [string][in] */ __RPC__in_string BSTR TaskName,
            /* [string][in] */ __RPC__in_string BSTR TaskParameters);
        
        DECLSPEC_XFGVIRT(IPMTileInfo, get_StartTileBlob)
        HRESULT ( STDMETHODCALLTYPE *get_StartTileBlob )( 
            __RPC__in IPMTileInfo * This,
            /* [out][in] */ __RPC__inout PM_STARTTILEBLOB *pBlob);
        
        DECLSPEC_XFGVIRT(IPMTileInfo, get_IsRestoring)
        HRESULT ( STDMETHODCALLTYPE *get_IsRestoring )( 
            __RPC__in IPMTileInfo * This,
            /* [out] */ __RPC__out BOOL *pIsRestoring);
        
        DECLSPEC_XFGVIRT(IPMTileInfo, get_IsAutoRestoreDisabled)
        HRESULT ( STDMETHODCALLTYPE *get_IsAutoRestoreDisabled )( 
            __RPC__in IPMTileInfo * This,
            /* [out] */ __RPC__out BOOL *pIsAutoRestoreDisabled);
        
        DECLSPEC_XFGVIRT(IPMTileInfo, set_IsRestoring)
        HRESULT ( STDMETHODCALLTYPE *set_IsRestoring )( 
            __RPC__in IPMTileInfo * This,
            /* [in] */ BOOL Restoring);
        
        DECLSPEC_XFGVIRT(IPMTileInfo, set_IsAutoRestoreDisabled)
        HRESULT ( STDMETHODCALLTYPE *set_IsAutoRestoreDisabled )( 
            __RPC__in IPMTileInfo * This,
            /* [in] */ BOOL AutoRestoreDisabled);
        
        END_INTERFACE
    } IPMTileInfoVtbl;

    interface IPMTileInfo
    {
        CONST_VTBL struct IPMTileInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPMTileInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPMTileInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPMTileInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPMTileInfo_get_ProductID(This,pProductID)	\
    ( (This)->lpVtbl -> get_ProductID(This,pProductID) ) 

#define IPMTileInfo_get_TileID(This,pTileID)	\
    ( (This)->lpVtbl -> get_TileID(This,pTileID) ) 

#define IPMTileInfo_get_TemplateType(This,pTemplateType)	\
    ( (This)->lpVtbl -> get_TemplateType(This,pTemplateType) ) 

#define IPMTileInfo_get_HubPinnedState(This,HubType,pPinned)	\
    ( (This)->lpVtbl -> get_HubPinnedState(This,HubType,pPinned) ) 

#define IPMTileInfo_get_HubPosition(This,HubType,pPosition)	\
    ( (This)->lpVtbl -> get_HubPosition(This,HubType,pPosition) ) 

#define IPMTileInfo_get_IsNotified(This,pIsNotified)	\
    ( (This)->lpVtbl -> get_IsNotified(This,pIsNotified) ) 

#define IPMTileInfo_get_IsDefault(This,pIsDefault)	\
    ( (This)->lpVtbl -> get_IsDefault(This,pIsDefault) ) 

#define IPMTileInfo_get_TaskID(This,pTaskID)	\
    ( (This)->lpVtbl -> get_TaskID(This,pTaskID) ) 

#define IPMTileInfo_get_TileType(This,pStartTileType)	\
    ( (This)->lpVtbl -> get_TileType(This,pStartTileType) ) 

#define IPMTileInfo_get_IsThemable(This,pIsThemable)	\
    ( (This)->lpVtbl -> get_IsThemable(This,pIsThemable) ) 

#define IPMTileInfo_get_PropertyById(This,PropID,ppPropInfo)	\
    ( (This)->lpVtbl -> get_PropertyById(This,PropID,ppPropInfo) ) 

#define IPMTileInfo_get_InvocationInfo(This,pImageUrn,pParameters)	\
    ( (This)->lpVtbl -> get_InvocationInfo(This,pImageUrn,pParameters) ) 

#define IPMTileInfo_get_PropertyEnum(This,ppTilePropEnum)	\
    ( (This)->lpVtbl -> get_PropertyEnum(This,ppTilePropEnum) ) 

#define IPMTileInfo_get_HubTileSize(This,HubType,pSize)	\
    ( (This)->lpVtbl -> get_HubTileSize(This,HubType,pSize) ) 

#define IPMTileInfo_set_HubPosition(This,HubType,Position)	\
    ( (This)->lpVtbl -> set_HubPosition(This,HubType,Position) ) 

#define IPMTileInfo_set_NotifiedState(This,Notified)	\
    ( (This)->lpVtbl -> set_NotifiedState(This,Notified) ) 

#define IPMTileInfo_set_HubPinnedState(This,HubType,Pinned)	\
    ( (This)->lpVtbl -> set_HubPinnedState(This,HubType,Pinned) ) 

#define IPMTileInfo_set_HubTileSize(This,HubType,Size)	\
    ( (This)->lpVtbl -> set_HubTileSize(This,HubType,Size) ) 

#define IPMTileInfo_set_InvocationInfo(This,TaskName,TaskParameters)	\
    ( (This)->lpVtbl -> set_InvocationInfo(This,TaskName,TaskParameters) ) 

#define IPMTileInfo_get_StartTileBlob(This,pBlob)	\
    ( (This)->lpVtbl -> get_StartTileBlob(This,pBlob) ) 

#define IPMTileInfo_get_IsRestoring(This,pIsRestoring)	\
    ( (This)->lpVtbl -> get_IsRestoring(This,pIsRestoring) ) 

#define IPMTileInfo_get_IsAutoRestoreDisabled(This,pIsAutoRestoreDisabled)	\
    ( (This)->lpVtbl -> get_IsAutoRestoreDisabled(This,pIsAutoRestoreDisabled) ) 

#define IPMTileInfo_set_IsRestoring(This,Restoring)	\
    ( (This)->lpVtbl -> set_IsRestoring(This,Restoring) ) 

#define IPMTileInfo_set_IsAutoRestoreDisabled(This,AutoRestoreDisabled)	\
    ( (This)->lpVtbl -> set_IsAutoRestoreDisabled(This,AutoRestoreDisabled) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPMTileInfo_INTERFACE_DEFINED__ */


#ifndef __IPMTileInfoEnumerator_INTERFACE_DEFINED__
#define __IPMTileInfoEnumerator_INTERFACE_DEFINED__

/* interface IPMTileInfoEnumerator */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IPMTileInfoEnumerator;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DED83065-E462-4b2c-ACB5-E39CEA61C874")
    IPMTileInfoEnumerator : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE get_Next( 
            /* [out] */ __RPC__deref_out_opt IPMTileInfo **ppTileInfo) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPMTileInfoEnumeratorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPMTileInfoEnumerator * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPMTileInfoEnumerator * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPMTileInfoEnumerator * This);
        
        DECLSPEC_XFGVIRT(IPMTileInfoEnumerator, get_Next)
        HRESULT ( STDMETHODCALLTYPE *get_Next )( 
            __RPC__in IPMTileInfoEnumerator * This,
            /* [out] */ __RPC__deref_out_opt IPMTileInfo **ppTileInfo);
        
        END_INTERFACE
    } IPMTileInfoEnumeratorVtbl;

    interface IPMTileInfoEnumerator
    {
        CONST_VTBL struct IPMTileInfoEnumeratorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPMTileInfoEnumerator_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPMTileInfoEnumerator_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPMTileInfoEnumerator_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPMTileInfoEnumerator_get_Next(This,ppTileInfo)	\
    ( (This)->lpVtbl -> get_Next(This,ppTileInfo) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPMTileInfoEnumerator_INTERFACE_DEFINED__ */


#ifndef __IPMApplicationInfoEnumerator_INTERFACE_DEFINED__
#define __IPMApplicationInfoEnumerator_INTERFACE_DEFINED__

/* interface IPMApplicationInfoEnumerator */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IPMApplicationInfoEnumerator;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0EC42A96-4D46-4dc6-A3D9-A7ACAAC0F5FA")
    IPMApplicationInfoEnumerator : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE get_Next( 
            /* [out] */ __RPC__deref_out_opt IPMApplicationInfo **ppAppInfo) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPMApplicationInfoEnumeratorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPMApplicationInfoEnumerator * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPMApplicationInfoEnumerator * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPMApplicationInfoEnumerator * This);
        
        DECLSPEC_XFGVIRT(IPMApplicationInfoEnumerator, get_Next)
        HRESULT ( STDMETHODCALLTYPE *get_Next )( 
            __RPC__in IPMApplicationInfoEnumerator * This,
            /* [out] */ __RPC__deref_out_opt IPMApplicationInfo **ppAppInfo);
        
        END_INTERFACE
    } IPMApplicationInfoEnumeratorVtbl;

    interface IPMApplicationInfoEnumerator
    {
        CONST_VTBL struct IPMApplicationInfoEnumeratorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPMApplicationInfoEnumerator_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPMApplicationInfoEnumerator_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPMApplicationInfoEnumerator_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPMApplicationInfoEnumerator_get_Next(This,ppAppInfo)	\
    ( (This)->lpVtbl -> get_Next(This,ppAppInfo) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPMApplicationInfoEnumerator_INTERFACE_DEFINED__ */


#ifndef __IPMLiveTileJobInfo_INTERFACE_DEFINED__
#define __IPMLiveTileJobInfo_INTERFACE_DEFINED__

/* interface IPMLiveTileJobInfo */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IPMLiveTileJobInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6009A81F-4710-4697-B5F6-2208F6057B8E")
    IPMLiveTileJobInfo : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE get_ProductID( 
            /* [out] */ __RPC__out PRODUCTID *pProductID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_TileID( 
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pTileID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_NextSchedule( 
            /* [out] */ __RPC__out FILETIME *pNextSchedule) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE set_NextSchedule( 
            /* [in] */ FILETIME ftNextSchedule) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_StartSchedule( 
            /* [out] */ __RPC__out FILETIME *pStartSchedule) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE set_StartSchedule( 
            /* [in] */ FILETIME ftStartSchedule) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_IntervalDuration( 
            /* [out] */ __RPC__out ULONG *pIntervalDuration) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE set_IntervalDuration( 
            /* [in] */ ULONG ulIntervalDuration) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_RunForever( 
            /* [out] */ __RPC__out BOOL *IsRunForever) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE set_RunForever( 
            /* [in] */ BOOL fRunForever) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_MaxRunCount( 
            /* [out] */ __RPC__out ULONG *pMaxRunCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE set_MaxRunCount( 
            /* [in] */ ULONG ulMaxRunCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_RunCount( 
            /* [out] */ __RPC__out ULONG *pRunCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE set_RunCount( 
            /* [in] */ ULONG ulRunCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_RecurrenceType( 
            /* [out] */ __RPC__out ULONG *pRecurrenceType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE set_RecurrenceType( 
            /* [in] */ ULONG ulRecurrenceType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_TileXML( 
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcbTileXml) BYTE **pTileXml,
            /* [out] */ __RPC__out ULONG *pcbTileXml) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE set_TileXML( 
            /* [size_is][in] */ __RPC__in_ecount_full(cbTileXml) BYTE *pTileXml,
            /* [in] */ ULONG cbTileXml) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_UrlXML( 
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcbUrlXML) BYTE **pUrlXML,
            /* [out] */ __RPC__out ULONG *pcbUrlXML) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE set_UrlXML( 
            /* [size_is][in] */ __RPC__in_ecount_full(cbUrlXML) BYTE *pUrlXML,
            /* [in] */ ULONG cbUrlXML) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_AttemptCount( 
            /* [out] */ __RPC__out ULONG *pAttemptCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE set_AttemptCount( 
            /* [in] */ ULONG ulAttemptCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_DownloadState( 
            /* [out] */ __RPC__out ULONG *pDownloadState) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE set_DownloadState( 
            /* [in] */ ULONG ulDownloadState) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPMLiveTileJobInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPMLiveTileJobInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPMLiveTileJobInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPMLiveTileJobInfo * This);
        
        DECLSPEC_XFGVIRT(IPMLiveTileJobInfo, get_ProductID)
        HRESULT ( STDMETHODCALLTYPE *get_ProductID )( 
            __RPC__in IPMLiveTileJobInfo * This,
            /* [out] */ __RPC__out PRODUCTID *pProductID);
        
        DECLSPEC_XFGVIRT(IPMLiveTileJobInfo, get_TileID)
        HRESULT ( STDMETHODCALLTYPE *get_TileID )( 
            __RPC__in IPMLiveTileJobInfo * This,
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pTileID);
        
        DECLSPEC_XFGVIRT(IPMLiveTileJobInfo, get_NextSchedule)
        HRESULT ( STDMETHODCALLTYPE *get_NextSchedule )( 
            __RPC__in IPMLiveTileJobInfo * This,
            /* [out] */ __RPC__out FILETIME *pNextSchedule);
        
        DECLSPEC_XFGVIRT(IPMLiveTileJobInfo, set_NextSchedule)
        HRESULT ( STDMETHODCALLTYPE *set_NextSchedule )( 
            __RPC__in IPMLiveTileJobInfo * This,
            /* [in] */ FILETIME ftNextSchedule);
        
        DECLSPEC_XFGVIRT(IPMLiveTileJobInfo, get_StartSchedule)
        HRESULT ( STDMETHODCALLTYPE *get_StartSchedule )( 
            __RPC__in IPMLiveTileJobInfo * This,
            /* [out] */ __RPC__out FILETIME *pStartSchedule);
        
        DECLSPEC_XFGVIRT(IPMLiveTileJobInfo, set_StartSchedule)
        HRESULT ( STDMETHODCALLTYPE *set_StartSchedule )( 
            __RPC__in IPMLiveTileJobInfo * This,
            /* [in] */ FILETIME ftStartSchedule);
        
        DECLSPEC_XFGVIRT(IPMLiveTileJobInfo, get_IntervalDuration)
        HRESULT ( STDMETHODCALLTYPE *get_IntervalDuration )( 
            __RPC__in IPMLiveTileJobInfo * This,
            /* [out] */ __RPC__out ULONG *pIntervalDuration);
        
        DECLSPEC_XFGVIRT(IPMLiveTileJobInfo, set_IntervalDuration)
        HRESULT ( STDMETHODCALLTYPE *set_IntervalDuration )( 
            __RPC__in IPMLiveTileJobInfo * This,
            /* [in] */ ULONG ulIntervalDuration);
        
        DECLSPEC_XFGVIRT(IPMLiveTileJobInfo, get_RunForever)
        HRESULT ( STDMETHODCALLTYPE *get_RunForever )( 
            __RPC__in IPMLiveTileJobInfo * This,
            /* [out] */ __RPC__out BOOL *IsRunForever);
        
        DECLSPEC_XFGVIRT(IPMLiveTileJobInfo, set_RunForever)
        HRESULT ( STDMETHODCALLTYPE *set_RunForever )( 
            __RPC__in IPMLiveTileJobInfo * This,
            /* [in] */ BOOL fRunForever);
        
        DECLSPEC_XFGVIRT(IPMLiveTileJobInfo, get_MaxRunCount)
        HRESULT ( STDMETHODCALLTYPE *get_MaxRunCount )( 
            __RPC__in IPMLiveTileJobInfo * This,
            /* [out] */ __RPC__out ULONG *pMaxRunCount);
        
        DECLSPEC_XFGVIRT(IPMLiveTileJobInfo, set_MaxRunCount)
        HRESULT ( STDMETHODCALLTYPE *set_MaxRunCount )( 
            __RPC__in IPMLiveTileJobInfo * This,
            /* [in] */ ULONG ulMaxRunCount);
        
        DECLSPEC_XFGVIRT(IPMLiveTileJobInfo, get_RunCount)
        HRESULT ( STDMETHODCALLTYPE *get_RunCount )( 
            __RPC__in IPMLiveTileJobInfo * This,
            /* [out] */ __RPC__out ULONG *pRunCount);
        
        DECLSPEC_XFGVIRT(IPMLiveTileJobInfo, set_RunCount)
        HRESULT ( STDMETHODCALLTYPE *set_RunCount )( 
            __RPC__in IPMLiveTileJobInfo * This,
            /* [in] */ ULONG ulRunCount);
        
        DECLSPEC_XFGVIRT(IPMLiveTileJobInfo, get_RecurrenceType)
        HRESULT ( STDMETHODCALLTYPE *get_RecurrenceType )( 
            __RPC__in IPMLiveTileJobInfo * This,
            /* [out] */ __RPC__out ULONG *pRecurrenceType);
        
        DECLSPEC_XFGVIRT(IPMLiveTileJobInfo, set_RecurrenceType)
        HRESULT ( STDMETHODCALLTYPE *set_RecurrenceType )( 
            __RPC__in IPMLiveTileJobInfo * This,
            /* [in] */ ULONG ulRecurrenceType);
        
        DECLSPEC_XFGVIRT(IPMLiveTileJobInfo, get_TileXML)
        HRESULT ( STDMETHODCALLTYPE *get_TileXML )( 
            __RPC__in IPMLiveTileJobInfo * This,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcbTileXml) BYTE **pTileXml,
            /* [out] */ __RPC__out ULONG *pcbTileXml);
        
        DECLSPEC_XFGVIRT(IPMLiveTileJobInfo, set_TileXML)
        HRESULT ( STDMETHODCALLTYPE *set_TileXML )( 
            __RPC__in IPMLiveTileJobInfo * This,
            /* [size_is][in] */ __RPC__in_ecount_full(cbTileXml) BYTE *pTileXml,
            /* [in] */ ULONG cbTileXml);
        
        DECLSPEC_XFGVIRT(IPMLiveTileJobInfo, get_UrlXML)
        HRESULT ( STDMETHODCALLTYPE *get_UrlXML )( 
            __RPC__in IPMLiveTileJobInfo * This,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcbUrlXML) BYTE **pUrlXML,
            /* [out] */ __RPC__out ULONG *pcbUrlXML);
        
        DECLSPEC_XFGVIRT(IPMLiveTileJobInfo, set_UrlXML)
        HRESULT ( STDMETHODCALLTYPE *set_UrlXML )( 
            __RPC__in IPMLiveTileJobInfo * This,
            /* [size_is][in] */ __RPC__in_ecount_full(cbUrlXML) BYTE *pUrlXML,
            /* [in] */ ULONG cbUrlXML);
        
        DECLSPEC_XFGVIRT(IPMLiveTileJobInfo, get_AttemptCount)
        HRESULT ( STDMETHODCALLTYPE *get_AttemptCount )( 
            __RPC__in IPMLiveTileJobInfo * This,
            /* [out] */ __RPC__out ULONG *pAttemptCount);
        
        DECLSPEC_XFGVIRT(IPMLiveTileJobInfo, set_AttemptCount)
        HRESULT ( STDMETHODCALLTYPE *set_AttemptCount )( 
            __RPC__in IPMLiveTileJobInfo * This,
            /* [in] */ ULONG ulAttemptCount);
        
        DECLSPEC_XFGVIRT(IPMLiveTileJobInfo, get_DownloadState)
        HRESULT ( STDMETHODCALLTYPE *get_DownloadState )( 
            __RPC__in IPMLiveTileJobInfo * This,
            /* [out] */ __RPC__out ULONG *pDownloadState);
        
        DECLSPEC_XFGVIRT(IPMLiveTileJobInfo, set_DownloadState)
        HRESULT ( STDMETHODCALLTYPE *set_DownloadState )( 
            __RPC__in IPMLiveTileJobInfo * This,
            /* [in] */ ULONG ulDownloadState);
        
        END_INTERFACE
    } IPMLiveTileJobInfoVtbl;

    interface IPMLiveTileJobInfo
    {
        CONST_VTBL struct IPMLiveTileJobInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPMLiveTileJobInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPMLiveTileJobInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPMLiveTileJobInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPMLiveTileJobInfo_get_ProductID(This,pProductID)	\
    ( (This)->lpVtbl -> get_ProductID(This,pProductID) ) 

#define IPMLiveTileJobInfo_get_TileID(This,pTileID)	\
    ( (This)->lpVtbl -> get_TileID(This,pTileID) ) 

#define IPMLiveTileJobInfo_get_NextSchedule(This,pNextSchedule)	\
    ( (This)->lpVtbl -> get_NextSchedule(This,pNextSchedule) ) 

#define IPMLiveTileJobInfo_set_NextSchedule(This,ftNextSchedule)	\
    ( (This)->lpVtbl -> set_NextSchedule(This,ftNextSchedule) ) 

#define IPMLiveTileJobInfo_get_StartSchedule(This,pStartSchedule)	\
    ( (This)->lpVtbl -> get_StartSchedule(This,pStartSchedule) ) 

#define IPMLiveTileJobInfo_set_StartSchedule(This,ftStartSchedule)	\
    ( (This)->lpVtbl -> set_StartSchedule(This,ftStartSchedule) ) 

#define IPMLiveTileJobInfo_get_IntervalDuration(This,pIntervalDuration)	\
    ( (This)->lpVtbl -> get_IntervalDuration(This,pIntervalDuration) ) 

#define IPMLiveTileJobInfo_set_IntervalDuration(This,ulIntervalDuration)	\
    ( (This)->lpVtbl -> set_IntervalDuration(This,ulIntervalDuration) ) 

#define IPMLiveTileJobInfo_get_RunForever(This,IsRunForever)	\
    ( (This)->lpVtbl -> get_RunForever(This,IsRunForever) ) 

#define IPMLiveTileJobInfo_set_RunForever(This,fRunForever)	\
    ( (This)->lpVtbl -> set_RunForever(This,fRunForever) ) 

#define IPMLiveTileJobInfo_get_MaxRunCount(This,pMaxRunCount)	\
    ( (This)->lpVtbl -> get_MaxRunCount(This,pMaxRunCount) ) 

#define IPMLiveTileJobInfo_set_MaxRunCount(This,ulMaxRunCount)	\
    ( (This)->lpVtbl -> set_MaxRunCount(This,ulMaxRunCount) ) 

#define IPMLiveTileJobInfo_get_RunCount(This,pRunCount)	\
    ( (This)->lpVtbl -> get_RunCount(This,pRunCount) ) 

#define IPMLiveTileJobInfo_set_RunCount(This,ulRunCount)	\
    ( (This)->lpVtbl -> set_RunCount(This,ulRunCount) ) 

#define IPMLiveTileJobInfo_get_RecurrenceType(This,pRecurrenceType)	\
    ( (This)->lpVtbl -> get_RecurrenceType(This,pRecurrenceType) ) 

#define IPMLiveTileJobInfo_set_RecurrenceType(This,ulRecurrenceType)	\
    ( (This)->lpVtbl -> set_RecurrenceType(This,ulRecurrenceType) ) 

#define IPMLiveTileJobInfo_get_TileXML(This,pTileXml,pcbTileXml)	\
    ( (This)->lpVtbl -> get_TileXML(This,pTileXml,pcbTileXml) ) 

#define IPMLiveTileJobInfo_set_TileXML(This,pTileXml,cbTileXml)	\
    ( (This)->lpVtbl -> set_TileXML(This,pTileXml,cbTileXml) ) 

#define IPMLiveTileJobInfo_get_UrlXML(This,pUrlXML,pcbUrlXML)	\
    ( (This)->lpVtbl -> get_UrlXML(This,pUrlXML,pcbUrlXML) ) 

#define IPMLiveTileJobInfo_set_UrlXML(This,pUrlXML,cbUrlXML)	\
    ( (This)->lpVtbl -> set_UrlXML(This,pUrlXML,cbUrlXML) ) 

#define IPMLiveTileJobInfo_get_AttemptCount(This,pAttemptCount)	\
    ( (This)->lpVtbl -> get_AttemptCount(This,pAttemptCount) ) 

#define IPMLiveTileJobInfo_set_AttemptCount(This,ulAttemptCount)	\
    ( (This)->lpVtbl -> set_AttemptCount(This,ulAttemptCount) ) 

#define IPMLiveTileJobInfo_get_DownloadState(This,pDownloadState)	\
    ( (This)->lpVtbl -> get_DownloadState(This,pDownloadState) ) 

#define IPMLiveTileJobInfo_set_DownloadState(This,ulDownloadState)	\
    ( (This)->lpVtbl -> set_DownloadState(This,ulDownloadState) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPMLiveTileJobInfo_INTERFACE_DEFINED__ */


#ifndef __IPMLiveTileJobInfoEnumerator_INTERFACE_DEFINED__
#define __IPMLiveTileJobInfoEnumerator_INTERFACE_DEFINED__

/* interface IPMLiveTileJobInfoEnumerator */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IPMLiveTileJobInfoEnumerator;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("BC042582-9415-4f36-9F99-06F104C07C03")
    IPMLiveTileJobInfoEnumerator : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE get_Next( 
            /* [out] */ __RPC__deref_out_opt IPMLiveTileJobInfo **ppLiveTileJobInfo) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPMLiveTileJobInfoEnumeratorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPMLiveTileJobInfoEnumerator * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPMLiveTileJobInfoEnumerator * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPMLiveTileJobInfoEnumerator * This);
        
        DECLSPEC_XFGVIRT(IPMLiveTileJobInfoEnumerator, get_Next)
        HRESULT ( STDMETHODCALLTYPE *get_Next )( 
            __RPC__in IPMLiveTileJobInfoEnumerator * This,
            /* [out] */ __RPC__deref_out_opt IPMLiveTileJobInfo **ppLiveTileJobInfo);
        
        END_INTERFACE
    } IPMLiveTileJobInfoEnumeratorVtbl;

    interface IPMLiveTileJobInfoEnumerator
    {
        CONST_VTBL struct IPMLiveTileJobInfoEnumeratorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPMLiveTileJobInfoEnumerator_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPMLiveTileJobInfoEnumerator_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPMLiveTileJobInfoEnumerator_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPMLiveTileJobInfoEnumerator_get_Next(This,ppLiveTileJobInfo)	\
    ( (This)->lpVtbl -> get_Next(This,ppLiveTileJobInfo) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPMLiveTileJobInfoEnumerator_INTERFACE_DEFINED__ */


#ifndef __IPMDeploymentManager_INTERFACE_DEFINED__
#define __IPMDeploymentManager_INTERFACE_DEFINED__

/* interface IPMDeploymentManager */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IPMDeploymentManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("35F785FA-1979-4A8B-BC8F-FD70EB0D1544")
    IPMDeploymentManager : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ReportDownloadBegin( 
            /* [in] */ PRODUCTID productID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ReportDownloadProgress( 
            /* [in] */ PRODUCTID productID,
            /* [in] */ USHORT usProgress) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ReportDownloadComplete( 
            /* [in] */ PRODUCTID productID,
            /* [in] */ HRESULT hrResult) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE BeginInstall( 
            /* [in] */ __RPC__in PM_INSTALLINFO *pInstallInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE BeginUpdate( 
            /* [in] */ __RPC__in PM_UPDATEINFO *pUpdateInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE BeginDeployPackage( 
            /* [in] */ __RPC__in PM_INSTALLINFO *pInstallInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE BeginUpdateDeployedPackageLegacy( 
            /* [in] */ __RPC__in PM_UPDATEINFO_LEGACY *pUpdateInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE BeginUninstall( 
            /* [in] */ PRODUCTID productID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE BeginEnterpriseAppInstall( 
            /* [in] */ __RPC__in PM_INSTALLINFO *pInstallInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE BeginEnterpriseAppUpdate( 
            /* [in] */ __RPC__in PM_UPDATEINFO *pUpdateInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE BeginUpdateLicense( 
            /* [in] */ PRODUCTID productID,
            /* [in] */ GUID offerID,
            /* [size_is][in] */ __RPC__in_ecount_full(cbLicense) BYTE *pbLicense,
            /* [in] */ DWORD cbLicense) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLicenseChallenge( 
            /* [string][in] */ __RPC__in_string BSTR PackagePath,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcbChallenge) BYTE **ppbChallenge,
            /* [out] */ __RPC__out DWORD *pcbChallenge,
            /* [unique][size_is][size_is][out][in] */ __RPC__deref_opt_inout_ecount_full_opt(*pcbKID) BYTE **ppbKID,
            /* [unique][out][in] */ __RPC__inout_opt DWORD *pcbKID,
            /* [unique][size_is][size_is][out][in] */ __RPC__deref_opt_inout_ecount_full_opt(*pcbDeviceID) BYTE **ppbDeviceID,
            /* [unique][out][in] */ __RPC__inout_opt DWORD *pcbDeviceID,
            /* [unique][size_is][size_is][out][in] */ __RPC__deref_opt_inout_ecount_full_opt(*pcbSaltValue) BYTE **ppbSaltValue,
            /* [unique][out][in] */ __RPC__inout_opt DWORD *pcbSaltValue,
            /* [unique][size_is][size_is][out][in] */ __RPC__deref_opt_inout_ecount_full_opt(*pcbKGVValue) BYTE **ppbKGVValue,
            /* [unique][out][in] */ __RPC__inout_opt DWORD *pcbKGVValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLicenseChallengeByProductID( 
            /* [in] */ PRODUCTID ProductID,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcbLicense) BYTE **ppbChallenge,
            /* [out] */ __RPC__out DWORD *pcbLicense) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLicenseChallengeByProductID2( 
            /* [in] */ PRODUCTID ProductID,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcbLicense) BYTE **ppbChallenge,
            /* [out] */ __RPC__out DWORD *pcbLicense,
            /* [unique][size_is][size_is][out][in] */ __RPC__deref_opt_inout_ecount_full_opt(*pcbKID) BYTE **ppbKID,
            /* [unique][out][in] */ __RPC__inout_opt DWORD *pcbKID,
            /* [unique][size_is][size_is][out][in] */ __RPC__deref_opt_inout_ecount_full_opt(*pcbDeviceID) BYTE **ppbDeviceID,
            /* [unique][out][in] */ __RPC__inout_opt DWORD *pcbDeviceID,
            /* [unique][size_is][size_is][out][in] */ __RPC__deref_opt_inout_ecount_full_opt(*pcbSaltValue) BYTE **ppbSaltValue,
            /* [unique][out][in] */ __RPC__inout_opt DWORD *pcbSaltValue,
            /* [unique][size_is][size_is][out][in] */ __RPC__deref_opt_inout_ecount_full_opt(*pcbKGVValue) BYTE **ppbKGVValue,
            /* [unique][out][in] */ __RPC__inout_opt DWORD *pcbKGVValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RevokeLicense( 
            /* [in] */ PRODUCTID productID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RebindMdilBinaries( 
            /* [in] */ PRODUCTID ProductID,
            /* [in] */ __RPC__in SAFEARRAY * FileNames) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RebindAllMdilBinaries( 
            /* [in] */ PRODUCTID ProductID,
            /* [in] */ GUID InstanceID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RegenerateXbf( 
            /* [in] */ PRODUCTID ProductID,
            /* [in] */ __RPC__in SAFEARRAY * AssemblyPaths) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GenerateXbfForCurrentLocale( 
            /* [in] */ PRODUCTID ProductID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE BeginProvision( 
            /* [in] */ PRODUCTID ProductID,
            /* [string][in] */ __RPC__in_string BSTR XMLpath) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE BeginDeprovision( 
            /* [in] */ PRODUCTID ProductID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ReindexSQLCEDatabases( 
            /* [in] */ PRODUCTID ProductID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetApplicationsNeedMaintenance( 
            /* [in] */ DWORD RequiredMaintenanceOperations,
            /* [out] */ __RPC__out DWORD *pcApplications) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UpdateChamberProfile( 
            /* [in] */ PRODUCTID ProductID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnterprisePolicyIsApplicationAllowed( 
            /* [in] */ PRODUCTID productId,
            /* [in] */ __RPC__in LPCWSTR publisherName,
            /* [out] */ __RPC__out BOOL *pIsAllowed) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE BeginUpdateDeployedPackage( 
            /* [in] */ __RPC__in PM_UPDATEINFO *pUpdateInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ReportRestoreCancelled( 
            /* [in] */ PRODUCTID productID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ResolveResourceString( 
            /* [in] */ __RPC__in LPCWSTR resourceString,
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pResolvedResourceString) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UpdateCapabilitiesForModernApps( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ReportDownloadStatusUpdate( 
            /* [in] */ PRODUCTID productId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE BeginUninstallWithOptions( 
            /* [in] */ PRODUCTID productID,
            /* [in] */ DWORD removalOptions) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE BindDeferredMdilBinaries( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GenerateXamlLightupXbfForCurrentLocale( 
            /* [string][in] */ __RPC__in_string BSTR PackageFamilyName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddLicenseForAppx( 
            /* [in] */ PRODUCTID productID,
            /* [size_is][in] */ __RPC__in_ecount_full(cbLicense) BYTE *pbLicense,
            /* [in] */ DWORD cbLicense,
            /* [unique][size_is][in] */ __RPC__in_ecount_full_opt(cbPlayReadyHeader) BYTE *pbPlayReadyHeader,
            /* [in] */ DWORD cbPlayReadyHeader) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FixJunctionsForAppsOnSDCard( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPMDeploymentManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPMDeploymentManager * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPMDeploymentManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPMDeploymentManager * This);
        
        DECLSPEC_XFGVIRT(IPMDeploymentManager, ReportDownloadBegin)
        HRESULT ( STDMETHODCALLTYPE *ReportDownloadBegin )( 
            __RPC__in IPMDeploymentManager * This,
            /* [in] */ PRODUCTID productID);
        
        DECLSPEC_XFGVIRT(IPMDeploymentManager, ReportDownloadProgress)
        HRESULT ( STDMETHODCALLTYPE *ReportDownloadProgress )( 
            __RPC__in IPMDeploymentManager * This,
            /* [in] */ PRODUCTID productID,
            /* [in] */ USHORT usProgress);
        
        DECLSPEC_XFGVIRT(IPMDeploymentManager, ReportDownloadComplete)
        HRESULT ( STDMETHODCALLTYPE *ReportDownloadComplete )( 
            __RPC__in IPMDeploymentManager * This,
            /* [in] */ PRODUCTID productID,
            /* [in] */ HRESULT hrResult);
        
        DECLSPEC_XFGVIRT(IPMDeploymentManager, BeginInstall)
        HRESULT ( STDMETHODCALLTYPE *BeginInstall )( 
            __RPC__in IPMDeploymentManager * This,
            /* [in] */ __RPC__in PM_INSTALLINFO *pInstallInfo);
        
        DECLSPEC_XFGVIRT(IPMDeploymentManager, BeginUpdate)
        HRESULT ( STDMETHODCALLTYPE *BeginUpdate )( 
            __RPC__in IPMDeploymentManager * This,
            /* [in] */ __RPC__in PM_UPDATEINFO *pUpdateInfo);
        
        DECLSPEC_XFGVIRT(IPMDeploymentManager, BeginDeployPackage)
        HRESULT ( STDMETHODCALLTYPE *BeginDeployPackage )( 
            __RPC__in IPMDeploymentManager * This,
            /* [in] */ __RPC__in PM_INSTALLINFO *pInstallInfo);
        
        DECLSPEC_XFGVIRT(IPMDeploymentManager, BeginUpdateDeployedPackageLegacy)
        HRESULT ( STDMETHODCALLTYPE *BeginUpdateDeployedPackageLegacy )( 
            __RPC__in IPMDeploymentManager * This,
            /* [in] */ __RPC__in PM_UPDATEINFO_LEGACY *pUpdateInfo);
        
        DECLSPEC_XFGVIRT(IPMDeploymentManager, BeginUninstall)
        HRESULT ( STDMETHODCALLTYPE *BeginUninstall )( 
            __RPC__in IPMDeploymentManager * This,
            /* [in] */ PRODUCTID productID);
        
        DECLSPEC_XFGVIRT(IPMDeploymentManager, BeginEnterpriseAppInstall)
        HRESULT ( STDMETHODCALLTYPE *BeginEnterpriseAppInstall )( 
            __RPC__in IPMDeploymentManager * This,
            /* [in] */ __RPC__in PM_INSTALLINFO *pInstallInfo);
        
        DECLSPEC_XFGVIRT(IPMDeploymentManager, BeginEnterpriseAppUpdate)
        HRESULT ( STDMETHODCALLTYPE *BeginEnterpriseAppUpdate )( 
            __RPC__in IPMDeploymentManager * This,
            /* [in] */ __RPC__in PM_UPDATEINFO *pUpdateInfo);
        
        DECLSPEC_XFGVIRT(IPMDeploymentManager, BeginUpdateLicense)
        HRESULT ( STDMETHODCALLTYPE *BeginUpdateLicense )( 
            __RPC__in IPMDeploymentManager * This,
            /* [in] */ PRODUCTID productID,
            /* [in] */ GUID offerID,
            /* [size_is][in] */ __RPC__in_ecount_full(cbLicense) BYTE *pbLicense,
            /* [in] */ DWORD cbLicense);
        
        DECLSPEC_XFGVIRT(IPMDeploymentManager, GetLicenseChallenge)
        HRESULT ( STDMETHODCALLTYPE *GetLicenseChallenge )( 
            __RPC__in IPMDeploymentManager * This,
            /* [string][in] */ __RPC__in_string BSTR PackagePath,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcbChallenge) BYTE **ppbChallenge,
            /* [out] */ __RPC__out DWORD *pcbChallenge,
            /* [unique][size_is][size_is][out][in] */ __RPC__deref_opt_inout_ecount_full_opt(*pcbKID) BYTE **ppbKID,
            /* [unique][out][in] */ __RPC__inout_opt DWORD *pcbKID,
            /* [unique][size_is][size_is][out][in] */ __RPC__deref_opt_inout_ecount_full_opt(*pcbDeviceID) BYTE **ppbDeviceID,
            /* [unique][out][in] */ __RPC__inout_opt DWORD *pcbDeviceID,
            /* [unique][size_is][size_is][out][in] */ __RPC__deref_opt_inout_ecount_full_opt(*pcbSaltValue) BYTE **ppbSaltValue,
            /* [unique][out][in] */ __RPC__inout_opt DWORD *pcbSaltValue,
            /* [unique][size_is][size_is][out][in] */ __RPC__deref_opt_inout_ecount_full_opt(*pcbKGVValue) BYTE **ppbKGVValue,
            /* [unique][out][in] */ __RPC__inout_opt DWORD *pcbKGVValue);
        
        DECLSPEC_XFGVIRT(IPMDeploymentManager, GetLicenseChallengeByProductID)
        HRESULT ( STDMETHODCALLTYPE *GetLicenseChallengeByProductID )( 
            __RPC__in IPMDeploymentManager * This,
            /* [in] */ PRODUCTID ProductID,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcbLicense) BYTE **ppbChallenge,
            /* [out] */ __RPC__out DWORD *pcbLicense);
        
        DECLSPEC_XFGVIRT(IPMDeploymentManager, GetLicenseChallengeByProductID2)
        HRESULT ( STDMETHODCALLTYPE *GetLicenseChallengeByProductID2 )( 
            __RPC__in IPMDeploymentManager * This,
            /* [in] */ PRODUCTID ProductID,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcbLicense) BYTE **ppbChallenge,
            /* [out] */ __RPC__out DWORD *pcbLicense,
            /* [unique][size_is][size_is][out][in] */ __RPC__deref_opt_inout_ecount_full_opt(*pcbKID) BYTE **ppbKID,
            /* [unique][out][in] */ __RPC__inout_opt DWORD *pcbKID,
            /* [unique][size_is][size_is][out][in] */ __RPC__deref_opt_inout_ecount_full_opt(*pcbDeviceID) BYTE **ppbDeviceID,
            /* [unique][out][in] */ __RPC__inout_opt DWORD *pcbDeviceID,
            /* [unique][size_is][size_is][out][in] */ __RPC__deref_opt_inout_ecount_full_opt(*pcbSaltValue) BYTE **ppbSaltValue,
            /* [unique][out][in] */ __RPC__inout_opt DWORD *pcbSaltValue,
            /* [unique][size_is][size_is][out][in] */ __RPC__deref_opt_inout_ecount_full_opt(*pcbKGVValue) BYTE **ppbKGVValue,
            /* [unique][out][in] */ __RPC__inout_opt DWORD *pcbKGVValue);
        
        DECLSPEC_XFGVIRT(IPMDeploymentManager, RevokeLicense)
        HRESULT ( STDMETHODCALLTYPE *RevokeLicense )( 
            __RPC__in IPMDeploymentManager * This,
            /* [in] */ PRODUCTID productID);
        
        DECLSPEC_XFGVIRT(IPMDeploymentManager, RebindMdilBinaries)
        HRESULT ( STDMETHODCALLTYPE *RebindMdilBinaries )( 
            __RPC__in IPMDeploymentManager * This,
            /* [in] */ PRODUCTID ProductID,
            /* [in] */ __RPC__in SAFEARRAY * FileNames);
        
        DECLSPEC_XFGVIRT(IPMDeploymentManager, RebindAllMdilBinaries)
        HRESULT ( STDMETHODCALLTYPE *RebindAllMdilBinaries )( 
            __RPC__in IPMDeploymentManager * This,
            /* [in] */ PRODUCTID ProductID,
            /* [in] */ GUID InstanceID);
        
        DECLSPEC_XFGVIRT(IPMDeploymentManager, RegenerateXbf)
        HRESULT ( STDMETHODCALLTYPE *RegenerateXbf )( 
            __RPC__in IPMDeploymentManager * This,
            /* [in] */ PRODUCTID ProductID,
            /* [in] */ __RPC__in SAFEARRAY * AssemblyPaths);
        
        DECLSPEC_XFGVIRT(IPMDeploymentManager, GenerateXbfForCurrentLocale)
        HRESULT ( STDMETHODCALLTYPE *GenerateXbfForCurrentLocale )( 
            __RPC__in IPMDeploymentManager * This,
            /* [in] */ PRODUCTID ProductID);
        
        DECLSPEC_XFGVIRT(IPMDeploymentManager, BeginProvision)
        HRESULT ( STDMETHODCALLTYPE *BeginProvision )( 
            __RPC__in IPMDeploymentManager * This,
            /* [in] */ PRODUCTID ProductID,
            /* [string][in] */ __RPC__in_string BSTR XMLpath);
        
        DECLSPEC_XFGVIRT(IPMDeploymentManager, BeginDeprovision)
        HRESULT ( STDMETHODCALLTYPE *BeginDeprovision )( 
            __RPC__in IPMDeploymentManager * This,
            /* [in] */ PRODUCTID ProductID);
        
        DECLSPEC_XFGVIRT(IPMDeploymentManager, ReindexSQLCEDatabases)
        HRESULT ( STDMETHODCALLTYPE *ReindexSQLCEDatabases )( 
            __RPC__in IPMDeploymentManager * This,
            /* [in] */ PRODUCTID ProductID);
        
        DECLSPEC_XFGVIRT(IPMDeploymentManager, SetApplicationsNeedMaintenance)
        HRESULT ( STDMETHODCALLTYPE *SetApplicationsNeedMaintenance )( 
            __RPC__in IPMDeploymentManager * This,
            /* [in] */ DWORD RequiredMaintenanceOperations,
            /* [out] */ __RPC__out DWORD *pcApplications);
        
        DECLSPEC_XFGVIRT(IPMDeploymentManager, UpdateChamberProfile)
        HRESULT ( STDMETHODCALLTYPE *UpdateChamberProfile )( 
            __RPC__in IPMDeploymentManager * This,
            /* [in] */ PRODUCTID ProductID);
        
        DECLSPEC_XFGVIRT(IPMDeploymentManager, EnterprisePolicyIsApplicationAllowed)
        HRESULT ( STDMETHODCALLTYPE *EnterprisePolicyIsApplicationAllowed )( 
            __RPC__in IPMDeploymentManager * This,
            /* [in] */ PRODUCTID productId,
            /* [in] */ __RPC__in LPCWSTR publisherName,
            /* [out] */ __RPC__out BOOL *pIsAllowed);
        
        DECLSPEC_XFGVIRT(IPMDeploymentManager, BeginUpdateDeployedPackage)
        HRESULT ( STDMETHODCALLTYPE *BeginUpdateDeployedPackage )( 
            __RPC__in IPMDeploymentManager * This,
            /* [in] */ __RPC__in PM_UPDATEINFO *pUpdateInfo);
        
        DECLSPEC_XFGVIRT(IPMDeploymentManager, ReportRestoreCancelled)
        HRESULT ( STDMETHODCALLTYPE *ReportRestoreCancelled )( 
            __RPC__in IPMDeploymentManager * This,
            /* [in] */ PRODUCTID productID);
        
        DECLSPEC_XFGVIRT(IPMDeploymentManager, ResolveResourceString)
        HRESULT ( STDMETHODCALLTYPE *ResolveResourceString )( 
            __RPC__in IPMDeploymentManager * This,
            /* [in] */ __RPC__in LPCWSTR resourceString,
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pResolvedResourceString);
        
        DECLSPEC_XFGVIRT(IPMDeploymentManager, UpdateCapabilitiesForModernApps)
        HRESULT ( STDMETHODCALLTYPE *UpdateCapabilitiesForModernApps )( 
            __RPC__in IPMDeploymentManager * This);
        
        DECLSPEC_XFGVIRT(IPMDeploymentManager, ReportDownloadStatusUpdate)
        HRESULT ( STDMETHODCALLTYPE *ReportDownloadStatusUpdate )( 
            __RPC__in IPMDeploymentManager * This,
            /* [in] */ PRODUCTID productId);
        
        DECLSPEC_XFGVIRT(IPMDeploymentManager, BeginUninstallWithOptions)
        HRESULT ( STDMETHODCALLTYPE *BeginUninstallWithOptions )( 
            __RPC__in IPMDeploymentManager * This,
            /* [in] */ PRODUCTID productID,
            /* [in] */ DWORD removalOptions);
        
        DECLSPEC_XFGVIRT(IPMDeploymentManager, BindDeferredMdilBinaries)
        HRESULT ( STDMETHODCALLTYPE *BindDeferredMdilBinaries )( 
            __RPC__in IPMDeploymentManager * This);
        
        DECLSPEC_XFGVIRT(IPMDeploymentManager, GenerateXamlLightupXbfForCurrentLocale)
        HRESULT ( STDMETHODCALLTYPE *GenerateXamlLightupXbfForCurrentLocale )( 
            __RPC__in IPMDeploymentManager * This,
            /* [string][in] */ __RPC__in_string BSTR PackageFamilyName);
        
        DECLSPEC_XFGVIRT(IPMDeploymentManager, AddLicenseForAppx)
        HRESULT ( STDMETHODCALLTYPE *AddLicenseForAppx )( 
            __RPC__in IPMDeploymentManager * This,
            /* [in] */ PRODUCTID productID,
            /* [size_is][in] */ __RPC__in_ecount_full(cbLicense) BYTE *pbLicense,
            /* [in] */ DWORD cbLicense,
            /* [unique][size_is][in] */ __RPC__in_ecount_full_opt(cbPlayReadyHeader) BYTE *pbPlayReadyHeader,
            /* [in] */ DWORD cbPlayReadyHeader);
        
        DECLSPEC_XFGVIRT(IPMDeploymentManager, FixJunctionsForAppsOnSDCard)
        HRESULT ( STDMETHODCALLTYPE *FixJunctionsForAppsOnSDCard )( 
            __RPC__in IPMDeploymentManager * This);
        
        END_INTERFACE
    } IPMDeploymentManagerVtbl;

    interface IPMDeploymentManager
    {
        CONST_VTBL struct IPMDeploymentManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPMDeploymentManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPMDeploymentManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPMDeploymentManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPMDeploymentManager_ReportDownloadBegin(This,productID)	\
    ( (This)->lpVtbl -> ReportDownloadBegin(This,productID) ) 

#define IPMDeploymentManager_ReportDownloadProgress(This,productID,usProgress)	\
    ( (This)->lpVtbl -> ReportDownloadProgress(This,productID,usProgress) ) 

#define IPMDeploymentManager_ReportDownloadComplete(This,productID,hrResult)	\
    ( (This)->lpVtbl -> ReportDownloadComplete(This,productID,hrResult) ) 

#define IPMDeploymentManager_BeginInstall(This,pInstallInfo)	\
    ( (This)->lpVtbl -> BeginInstall(This,pInstallInfo) ) 

#define IPMDeploymentManager_BeginUpdate(This,pUpdateInfo)	\
    ( (This)->lpVtbl -> BeginUpdate(This,pUpdateInfo) ) 

#define IPMDeploymentManager_BeginDeployPackage(This,pInstallInfo)	\
    ( (This)->lpVtbl -> BeginDeployPackage(This,pInstallInfo) ) 

#define IPMDeploymentManager_BeginUpdateDeployedPackageLegacy(This,pUpdateInfo)	\
    ( (This)->lpVtbl -> BeginUpdateDeployedPackageLegacy(This,pUpdateInfo) ) 

#define IPMDeploymentManager_BeginUninstall(This,productID)	\
    ( (This)->lpVtbl -> BeginUninstall(This,productID) ) 

#define IPMDeploymentManager_BeginEnterpriseAppInstall(This,pInstallInfo)	\
    ( (This)->lpVtbl -> BeginEnterpriseAppInstall(This,pInstallInfo) ) 

#define IPMDeploymentManager_BeginEnterpriseAppUpdate(This,pUpdateInfo)	\
    ( (This)->lpVtbl -> BeginEnterpriseAppUpdate(This,pUpdateInfo) ) 

#define IPMDeploymentManager_BeginUpdateLicense(This,productID,offerID,pbLicense,cbLicense)	\
    ( (This)->lpVtbl -> BeginUpdateLicense(This,productID,offerID,pbLicense,cbLicense) ) 

#define IPMDeploymentManager_GetLicenseChallenge(This,PackagePath,ppbChallenge,pcbChallenge,ppbKID,pcbKID,ppbDeviceID,pcbDeviceID,ppbSaltValue,pcbSaltValue,ppbKGVValue,pcbKGVValue)	\
    ( (This)->lpVtbl -> GetLicenseChallenge(This,PackagePath,ppbChallenge,pcbChallenge,ppbKID,pcbKID,ppbDeviceID,pcbDeviceID,ppbSaltValue,pcbSaltValue,ppbKGVValue,pcbKGVValue) ) 

#define IPMDeploymentManager_GetLicenseChallengeByProductID(This,ProductID,ppbChallenge,pcbLicense)	\
    ( (This)->lpVtbl -> GetLicenseChallengeByProductID(This,ProductID,ppbChallenge,pcbLicense) ) 

#define IPMDeploymentManager_GetLicenseChallengeByProductID2(This,ProductID,ppbChallenge,pcbLicense,ppbKID,pcbKID,ppbDeviceID,pcbDeviceID,ppbSaltValue,pcbSaltValue,ppbKGVValue,pcbKGVValue)	\
    ( (This)->lpVtbl -> GetLicenseChallengeByProductID2(This,ProductID,ppbChallenge,pcbLicense,ppbKID,pcbKID,ppbDeviceID,pcbDeviceID,ppbSaltValue,pcbSaltValue,ppbKGVValue,pcbKGVValue) ) 

#define IPMDeploymentManager_RevokeLicense(This,productID)	\
    ( (This)->lpVtbl -> RevokeLicense(This,productID) ) 

#define IPMDeploymentManager_RebindMdilBinaries(This,ProductID,FileNames)	\
    ( (This)->lpVtbl -> RebindMdilBinaries(This,ProductID,FileNames) ) 

#define IPMDeploymentManager_RebindAllMdilBinaries(This,ProductID,InstanceID)	\
    ( (This)->lpVtbl -> RebindAllMdilBinaries(This,ProductID,InstanceID) ) 

#define IPMDeploymentManager_RegenerateXbf(This,ProductID,AssemblyPaths)	\
    ( (This)->lpVtbl -> RegenerateXbf(This,ProductID,AssemblyPaths) ) 

#define IPMDeploymentManager_GenerateXbfForCurrentLocale(This,ProductID)	\
    ( (This)->lpVtbl -> GenerateXbfForCurrentLocale(This,ProductID) ) 

#define IPMDeploymentManager_BeginProvision(This,ProductID,XMLpath)	\
    ( (This)->lpVtbl -> BeginProvision(This,ProductID,XMLpath) ) 

#define IPMDeploymentManager_BeginDeprovision(This,ProductID)	\
    ( (This)->lpVtbl -> BeginDeprovision(This,ProductID) ) 

#define IPMDeploymentManager_ReindexSQLCEDatabases(This,ProductID)	\
    ( (This)->lpVtbl -> ReindexSQLCEDatabases(This,ProductID) ) 

#define IPMDeploymentManager_SetApplicationsNeedMaintenance(This,RequiredMaintenanceOperations,pcApplications)	\
    ( (This)->lpVtbl -> SetApplicationsNeedMaintenance(This,RequiredMaintenanceOperations,pcApplications) ) 

#define IPMDeploymentManager_UpdateChamberProfile(This,ProductID)	\
    ( (This)->lpVtbl -> UpdateChamberProfile(This,ProductID) ) 

#define IPMDeploymentManager_EnterprisePolicyIsApplicationAllowed(This,productId,publisherName,pIsAllowed)	\
    ( (This)->lpVtbl -> EnterprisePolicyIsApplicationAllowed(This,productId,publisherName,pIsAllowed) ) 

#define IPMDeploymentManager_BeginUpdateDeployedPackage(This,pUpdateInfo)	\
    ( (This)->lpVtbl -> BeginUpdateDeployedPackage(This,pUpdateInfo) ) 

#define IPMDeploymentManager_ReportRestoreCancelled(This,productID)	\
    ( (This)->lpVtbl -> ReportRestoreCancelled(This,productID) ) 

#define IPMDeploymentManager_ResolveResourceString(This,resourceString,pResolvedResourceString)	\
    ( (This)->lpVtbl -> ResolveResourceString(This,resourceString,pResolvedResourceString) ) 

#define IPMDeploymentManager_UpdateCapabilitiesForModernApps(This)	\
    ( (This)->lpVtbl -> UpdateCapabilitiesForModernApps(This) ) 

#define IPMDeploymentManager_ReportDownloadStatusUpdate(This,productId)	\
    ( (This)->lpVtbl -> ReportDownloadStatusUpdate(This,productId) ) 

#define IPMDeploymentManager_BeginUninstallWithOptions(This,productID,removalOptions)	\
    ( (This)->lpVtbl -> BeginUninstallWithOptions(This,productID,removalOptions) ) 

#define IPMDeploymentManager_BindDeferredMdilBinaries(This)	\
    ( (This)->lpVtbl -> BindDeferredMdilBinaries(This) ) 

#define IPMDeploymentManager_GenerateXamlLightupXbfForCurrentLocale(This,PackageFamilyName)	\
    ( (This)->lpVtbl -> GenerateXamlLightupXbfForCurrentLocale(This,PackageFamilyName) ) 

#define IPMDeploymentManager_AddLicenseForAppx(This,productID,pbLicense,cbLicense,pbPlayReadyHeader,cbPlayReadyHeader)	\
    ( (This)->lpVtbl -> AddLicenseForAppx(This,productID,pbLicense,cbLicense,pbPlayReadyHeader,cbPlayReadyHeader) ) 

#define IPMDeploymentManager_FixJunctionsForAppsOnSDCard(This)	\
    ( (This)->lpVtbl -> FixJunctionsForAppsOnSDCard(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPMDeploymentManager_INTERFACE_DEFINED__ */


#ifndef __IPMEnumerationManager_INTERFACE_DEFINED__
#define __IPMEnumerationManager_INTERFACE_DEFINED__

/* interface IPMEnumerationManager */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IPMEnumerationManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("698D57C2-292D-4CF3-B73C-D95A6922ED9A")
    IPMEnumerationManager : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE get_AllApplications( 
            /* [out] */ __RPC__deref_out_opt IPMApplicationInfoEnumerator **ppAppEnum,
            /* [in] */ PM_ENUM_FILTER Filter) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_AllTiles( 
            /* [out] */ __RPC__deref_out_opt IPMTileInfoEnumerator **ppTileEnum,
            /* [in] */ PM_ENUM_FILTER Filter) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_AllTasks( 
            /* [out] */ __RPC__deref_out_opt IPMTaskInfoEnumerator **ppTaskEnum,
            /* [in] */ PM_ENUM_FILTER Filter) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_AllExtensions( 
            /* [out] */ __RPC__deref_out_opt IPMExtensionInfoEnumerator **ppExtensionEnum,
            /* [in] */ PM_ENUM_FILTER Filter) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_AllBackgroundServiceAgents( 
            /* [out] */ __RPC__deref_out_opt IPMBackgroundServiceAgentInfoEnumerator **ppBSAEnum,
            /* [in] */ PM_ENUM_FILTER Filter) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_AllBackgroundWorkers( 
            /* [out] */ __RPC__deref_out_opt IPMBackgroundWorkerInfoEnumerator **ppBSWEnum,
            /* [in] */ PM_ENUM_FILTER Filter) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_ApplicationInfo( 
            /* [in] */ PRODUCTID ProductID,
            /* [out] */ __RPC__deref_out_opt IPMApplicationInfo **ppAppInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_TileInfo( 
            /* [in] */ PRODUCTID ProductID,
            /* [string][in] */ __RPC__in_string BSTR TileID,
            /* [out] */ __RPC__deref_out_opt IPMTileInfo **ppTileInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_TaskInfo( 
            /* [in] */ PRODUCTID ProductID,
            /* [string][in] */ __RPC__in_string BSTR TaskID,
            /* [out] */ __RPC__deref_out_opt IPMTaskInfo **ppTaskInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_TaskInfoEx( 
            /* [in] */ PRODUCTID ProductID,
            /* [string][in] */ __RPC__in_string LPCWSTR TaskID,
            /* [out] */ __RPC__deref_out_opt IPMTaskInfo **ppTaskInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_BackgroundServiceAgentInfo( 
            /* [in] */ DWORD BSAID,
            /* [out] */ __RPC__deref_out_opt IPMBackgroundServiceAgentInfo **ppTaskInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_AllLiveTileJobs( 
            /* [out] */ __RPC__deref_out_opt IPMLiveTileJobInfoEnumerator **ppLiveTileJobEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_LiveTileJob( 
            /* [in] */ PRODUCTID ProductID,
            /* [string][in] */ __RPC__in_string BSTR TileID,
            /* [in] */ PM_LIVETILE_RECURRENCE_TYPE RecurrenceType,
            /* [out] */ __RPC__deref_out_opt IPMLiveTileJobInfo **ppLiveTileJobInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_ApplicationInfoExternal( 
            /* [in] */ PRODUCTID ProductID,
            /* [out] */ __RPC__deref_out_opt IPMApplicationInfo **ppAppInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_FileHandlerGenericLogo( 
            /* [string][in] */ __RPC__in_string BSTR FileType,
            /* [in] */ PM_LOGO_SIZE LogoSize,
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pLogo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_ApplicationInfoFromAccessClaims( 
            /* [string][in] */ __RPC__in_string BSTR SysAppID0,
            /* [string][in] */ __RPC__in_string BSTR SysAppID1,
            /* [out] */ __RPC__deref_out_opt IPMApplicationInfo **ppAppInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_StartTileEnumeratorBlob( 
            /* [in] */ PM_ENUM_FILTER Filter,
            /* [out] */ __RPC__out DWORD *pcTiles,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcTiles) PM_STARTTILEBLOB **ppTileBlobs) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_StartAppEnumeratorBlob( 
            /* [in] */ PM_ENUM_FILTER Filter,
            /* [out] */ __RPC__out DWORD *pcApps,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcApps) PM_STARTAPPBLOB **ppAppBlobs) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPMEnumerationManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPMEnumerationManager * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPMEnumerationManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPMEnumerationManager * This);
        
        DECLSPEC_XFGVIRT(IPMEnumerationManager, get_AllApplications)
        HRESULT ( STDMETHODCALLTYPE *get_AllApplications )( 
            __RPC__in IPMEnumerationManager * This,
            /* [out] */ __RPC__deref_out_opt IPMApplicationInfoEnumerator **ppAppEnum,
            /* [in] */ PM_ENUM_FILTER Filter);
        
        DECLSPEC_XFGVIRT(IPMEnumerationManager, get_AllTiles)
        HRESULT ( STDMETHODCALLTYPE *get_AllTiles )( 
            __RPC__in IPMEnumerationManager * This,
            /* [out] */ __RPC__deref_out_opt IPMTileInfoEnumerator **ppTileEnum,
            /* [in] */ PM_ENUM_FILTER Filter);
        
        DECLSPEC_XFGVIRT(IPMEnumerationManager, get_AllTasks)
        HRESULT ( STDMETHODCALLTYPE *get_AllTasks )( 
            __RPC__in IPMEnumerationManager * This,
            /* [out] */ __RPC__deref_out_opt IPMTaskInfoEnumerator **ppTaskEnum,
            /* [in] */ PM_ENUM_FILTER Filter);
        
        DECLSPEC_XFGVIRT(IPMEnumerationManager, get_AllExtensions)
        HRESULT ( STDMETHODCALLTYPE *get_AllExtensions )( 
            __RPC__in IPMEnumerationManager * This,
            /* [out] */ __RPC__deref_out_opt IPMExtensionInfoEnumerator **ppExtensionEnum,
            /* [in] */ PM_ENUM_FILTER Filter);
        
        DECLSPEC_XFGVIRT(IPMEnumerationManager, get_AllBackgroundServiceAgents)
        HRESULT ( STDMETHODCALLTYPE *get_AllBackgroundServiceAgents )( 
            __RPC__in IPMEnumerationManager * This,
            /* [out] */ __RPC__deref_out_opt IPMBackgroundServiceAgentInfoEnumerator **ppBSAEnum,
            /* [in] */ PM_ENUM_FILTER Filter);
        
        DECLSPEC_XFGVIRT(IPMEnumerationManager, get_AllBackgroundWorkers)
        HRESULT ( STDMETHODCALLTYPE *get_AllBackgroundWorkers )( 
            __RPC__in IPMEnumerationManager * This,
            /* [out] */ __RPC__deref_out_opt IPMBackgroundWorkerInfoEnumerator **ppBSWEnum,
            /* [in] */ PM_ENUM_FILTER Filter);
        
        DECLSPEC_XFGVIRT(IPMEnumerationManager, get_ApplicationInfo)
        HRESULT ( STDMETHODCALLTYPE *get_ApplicationInfo )( 
            __RPC__in IPMEnumerationManager * This,
            /* [in] */ PRODUCTID ProductID,
            /* [out] */ __RPC__deref_out_opt IPMApplicationInfo **ppAppInfo);
        
        DECLSPEC_XFGVIRT(IPMEnumerationManager, get_TileInfo)
        HRESULT ( STDMETHODCALLTYPE *get_TileInfo )( 
            __RPC__in IPMEnumerationManager * This,
            /* [in] */ PRODUCTID ProductID,
            /* [string][in] */ __RPC__in_string BSTR TileID,
            /* [out] */ __RPC__deref_out_opt IPMTileInfo **ppTileInfo);
        
        DECLSPEC_XFGVIRT(IPMEnumerationManager, get_TaskInfo)
        HRESULT ( STDMETHODCALLTYPE *get_TaskInfo )( 
            __RPC__in IPMEnumerationManager * This,
            /* [in] */ PRODUCTID ProductID,
            /* [string][in] */ __RPC__in_string BSTR TaskID,
            /* [out] */ __RPC__deref_out_opt IPMTaskInfo **ppTaskInfo);
        
        DECLSPEC_XFGVIRT(IPMEnumerationManager, get_TaskInfoEx)
        HRESULT ( STDMETHODCALLTYPE *get_TaskInfoEx )( 
            __RPC__in IPMEnumerationManager * This,
            /* [in] */ PRODUCTID ProductID,
            /* [string][in] */ __RPC__in_string LPCWSTR TaskID,
            /* [out] */ __RPC__deref_out_opt IPMTaskInfo **ppTaskInfo);
        
        DECLSPEC_XFGVIRT(IPMEnumerationManager, get_BackgroundServiceAgentInfo)
        HRESULT ( STDMETHODCALLTYPE *get_BackgroundServiceAgentInfo )( 
            __RPC__in IPMEnumerationManager * This,
            /* [in] */ DWORD BSAID,
            /* [out] */ __RPC__deref_out_opt IPMBackgroundServiceAgentInfo **ppTaskInfo);
        
        DECLSPEC_XFGVIRT(IPMEnumerationManager, get_AllLiveTileJobs)
        HRESULT ( STDMETHODCALLTYPE *get_AllLiveTileJobs )( 
            __RPC__in IPMEnumerationManager * This,
            /* [out] */ __RPC__deref_out_opt IPMLiveTileJobInfoEnumerator **ppLiveTileJobEnum);
        
        DECLSPEC_XFGVIRT(IPMEnumerationManager, get_LiveTileJob)
        HRESULT ( STDMETHODCALLTYPE *get_LiveTileJob )( 
            __RPC__in IPMEnumerationManager * This,
            /* [in] */ PRODUCTID ProductID,
            /* [string][in] */ __RPC__in_string BSTR TileID,
            /* [in] */ PM_LIVETILE_RECURRENCE_TYPE RecurrenceType,
            /* [out] */ __RPC__deref_out_opt IPMLiveTileJobInfo **ppLiveTileJobInfo);
        
        DECLSPEC_XFGVIRT(IPMEnumerationManager, get_ApplicationInfoExternal)
        HRESULT ( STDMETHODCALLTYPE *get_ApplicationInfoExternal )( 
            __RPC__in IPMEnumerationManager * This,
            /* [in] */ PRODUCTID ProductID,
            /* [out] */ __RPC__deref_out_opt IPMApplicationInfo **ppAppInfo);
        
        DECLSPEC_XFGVIRT(IPMEnumerationManager, get_FileHandlerGenericLogo)
        HRESULT ( STDMETHODCALLTYPE *get_FileHandlerGenericLogo )( 
            __RPC__in IPMEnumerationManager * This,
            /* [string][in] */ __RPC__in_string BSTR FileType,
            /* [in] */ PM_LOGO_SIZE LogoSize,
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pLogo);
        
        DECLSPEC_XFGVIRT(IPMEnumerationManager, get_ApplicationInfoFromAccessClaims)
        HRESULT ( STDMETHODCALLTYPE *get_ApplicationInfoFromAccessClaims )( 
            __RPC__in IPMEnumerationManager * This,
            /* [string][in] */ __RPC__in_string BSTR SysAppID0,
            /* [string][in] */ __RPC__in_string BSTR SysAppID1,
            /* [out] */ __RPC__deref_out_opt IPMApplicationInfo **ppAppInfo);
        
        DECLSPEC_XFGVIRT(IPMEnumerationManager, get_StartTileEnumeratorBlob)
        HRESULT ( STDMETHODCALLTYPE *get_StartTileEnumeratorBlob )( 
            __RPC__in IPMEnumerationManager * This,
            /* [in] */ PM_ENUM_FILTER Filter,
            /* [out] */ __RPC__out DWORD *pcTiles,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcTiles) PM_STARTTILEBLOB **ppTileBlobs);
        
        DECLSPEC_XFGVIRT(IPMEnumerationManager, get_StartAppEnumeratorBlob)
        HRESULT ( STDMETHODCALLTYPE *get_StartAppEnumeratorBlob )( 
            __RPC__in IPMEnumerationManager * This,
            /* [in] */ PM_ENUM_FILTER Filter,
            /* [out] */ __RPC__out DWORD *pcApps,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcApps) PM_STARTAPPBLOB **ppAppBlobs);
        
        END_INTERFACE
    } IPMEnumerationManagerVtbl;

    interface IPMEnumerationManager
    {
        CONST_VTBL struct IPMEnumerationManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPMEnumerationManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPMEnumerationManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPMEnumerationManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPMEnumerationManager_get_AllApplications(This,ppAppEnum,Filter)	\
    ( (This)->lpVtbl -> get_AllApplications(This,ppAppEnum,Filter) ) 

#define IPMEnumerationManager_get_AllTiles(This,ppTileEnum,Filter)	\
    ( (This)->lpVtbl -> get_AllTiles(This,ppTileEnum,Filter) ) 

#define IPMEnumerationManager_get_AllTasks(This,ppTaskEnum,Filter)	\
    ( (This)->lpVtbl -> get_AllTasks(This,ppTaskEnum,Filter) ) 

#define IPMEnumerationManager_get_AllExtensions(This,ppExtensionEnum,Filter)	\
    ( (This)->lpVtbl -> get_AllExtensions(This,ppExtensionEnum,Filter) ) 

#define IPMEnumerationManager_get_AllBackgroundServiceAgents(This,ppBSAEnum,Filter)	\
    ( (This)->lpVtbl -> get_AllBackgroundServiceAgents(This,ppBSAEnum,Filter) ) 

#define IPMEnumerationManager_get_AllBackgroundWorkers(This,ppBSWEnum,Filter)	\
    ( (This)->lpVtbl -> get_AllBackgroundWorkers(This,ppBSWEnum,Filter) ) 

#define IPMEnumerationManager_get_ApplicationInfo(This,ProductID,ppAppInfo)	\
    ( (This)->lpVtbl -> get_ApplicationInfo(This,ProductID,ppAppInfo) ) 

#define IPMEnumerationManager_get_TileInfo(This,ProductID,TileID,ppTileInfo)	\
    ( (This)->lpVtbl -> get_TileInfo(This,ProductID,TileID,ppTileInfo) ) 

#define IPMEnumerationManager_get_TaskInfo(This,ProductID,TaskID,ppTaskInfo)	\
    ( (This)->lpVtbl -> get_TaskInfo(This,ProductID,TaskID,ppTaskInfo) ) 

#define IPMEnumerationManager_get_TaskInfoEx(This,ProductID,TaskID,ppTaskInfo)	\
    ( (This)->lpVtbl -> get_TaskInfoEx(This,ProductID,TaskID,ppTaskInfo) ) 

#define IPMEnumerationManager_get_BackgroundServiceAgentInfo(This,BSAID,ppTaskInfo)	\
    ( (This)->lpVtbl -> get_BackgroundServiceAgentInfo(This,BSAID,ppTaskInfo) ) 

#define IPMEnumerationManager_get_AllLiveTileJobs(This,ppLiveTileJobEnum)	\
    ( (This)->lpVtbl -> get_AllLiveTileJobs(This,ppLiveTileJobEnum) ) 

#define IPMEnumerationManager_get_LiveTileJob(This,ProductID,TileID,RecurrenceType,ppLiveTileJobInfo)	\
    ( (This)->lpVtbl -> get_LiveTileJob(This,ProductID,TileID,RecurrenceType,ppLiveTileJobInfo) ) 

#define IPMEnumerationManager_get_ApplicationInfoExternal(This,ProductID,ppAppInfo)	\
    ( (This)->lpVtbl -> get_ApplicationInfoExternal(This,ProductID,ppAppInfo) ) 

#define IPMEnumerationManager_get_FileHandlerGenericLogo(This,FileType,LogoSize,pLogo)	\
    ( (This)->lpVtbl -> get_FileHandlerGenericLogo(This,FileType,LogoSize,pLogo) ) 

#define IPMEnumerationManager_get_ApplicationInfoFromAccessClaims(This,SysAppID0,SysAppID1,ppAppInfo)	\
    ( (This)->lpVtbl -> get_ApplicationInfoFromAccessClaims(This,SysAppID0,SysAppID1,ppAppInfo) ) 

#define IPMEnumerationManager_get_StartTileEnumeratorBlob(This,Filter,pcTiles,ppTileBlobs)	\
    ( (This)->lpVtbl -> get_StartTileEnumeratorBlob(This,Filter,pcTiles,ppTileBlobs) ) 

#define IPMEnumerationManager_get_StartAppEnumeratorBlob(This,Filter,pcApps,ppAppBlobs)	\
    ( (This)->lpVtbl -> get_StartAppEnumeratorBlob(This,Filter,pcApps,ppAppBlobs) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPMEnumerationManager_INTERFACE_DEFINED__ */


#ifndef __IPMTaskInfo_INTERFACE_DEFINED__
#define __IPMTaskInfo_INTERFACE_DEFINED__

/* interface IPMTaskInfo */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IPMTaskInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("BF1D8C33-1BF5-4ee0-B549-6B9DD3834942")
    IPMTaskInfo : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE get_ProductID( 
            /* [out] */ __RPC__out PRODUCTID *pProductID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_TaskID( 
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pTaskID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_NavigationPage( 
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pNavigationPage) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_TaskTransition( 
            /* [out] */ __RPC__out PM_TASK_TRANSITION *pTaskTransition) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_RuntimeType( 
            /* [out] */ __RPC__out PACKMAN_RUNTIME *pRuntimetype) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_ActivationPolicy( 
            /* [out] */ __RPC__out PM_ACTIVATION_POLICY *pActivationPolicy) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_TaskType( 
            /* [out] */ __RPC__out PM_TASK_TYPE *pTaskType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_InvocationInfo( 
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pImageUrn,
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pParameters) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_ImagePath( 
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pImagePath) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_ImageParams( 
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pImageParams) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_InstallRootFolder( 
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pInstallRootFolder) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_DataRootFolder( 
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pDataRootFolder) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_IsSingleInstanceHost( 
            /* [out] */ __RPC__out BOOL *pIsSingleInstanceHost) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_IsInteropEnabled( 
            /* [out] */ __RPC__out BOOL *pIsInteropEnabled) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_ApplicationState( 
            /* [out] */ __RPC__out PM_APPLICATION_STATE *pApplicationState) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_InstallType( 
            /* [out] */ __RPC__out PM_APPLICATION_INSTALL_TYPE *pInstallType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_Version( 
            /* [out] */ __RPC__out BYTE *pTargetMajorVersion,
            /* [out] */ __RPC__out BYTE *pTargetMinorVersion) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_BitsPerPixel( 
            /* [out] */ __RPC__out USHORT *pBitsPerPixel) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_SuppressesDehydration( 
            /* [out] */ __RPC__out BOOL *pSuppressesDehydration) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_BackgroundExecutionAbilities( 
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pBackgroundExecutionAbilities) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_IsOptedForExtendedMem( 
            /* [out] */ __RPC__out BOOL *pIsOptedIn) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPMTaskInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPMTaskInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPMTaskInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPMTaskInfo * This);
        
        DECLSPEC_XFGVIRT(IPMTaskInfo, get_ProductID)
        HRESULT ( STDMETHODCALLTYPE *get_ProductID )( 
            __RPC__in IPMTaskInfo * This,
            /* [out] */ __RPC__out PRODUCTID *pProductID);
        
        DECLSPEC_XFGVIRT(IPMTaskInfo, get_TaskID)
        HRESULT ( STDMETHODCALLTYPE *get_TaskID )( 
            __RPC__in IPMTaskInfo * This,
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pTaskID);
        
        DECLSPEC_XFGVIRT(IPMTaskInfo, get_NavigationPage)
        HRESULT ( STDMETHODCALLTYPE *get_NavigationPage )( 
            __RPC__in IPMTaskInfo * This,
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pNavigationPage);
        
        DECLSPEC_XFGVIRT(IPMTaskInfo, get_TaskTransition)
        HRESULT ( STDMETHODCALLTYPE *get_TaskTransition )( 
            __RPC__in IPMTaskInfo * This,
            /* [out] */ __RPC__out PM_TASK_TRANSITION *pTaskTransition);
        
        DECLSPEC_XFGVIRT(IPMTaskInfo, get_RuntimeType)
        HRESULT ( STDMETHODCALLTYPE *get_RuntimeType )( 
            __RPC__in IPMTaskInfo * This,
            /* [out] */ __RPC__out PACKMAN_RUNTIME *pRuntimetype);
        
        DECLSPEC_XFGVIRT(IPMTaskInfo, get_ActivationPolicy)
        HRESULT ( STDMETHODCALLTYPE *get_ActivationPolicy )( 
            __RPC__in IPMTaskInfo * This,
            /* [out] */ __RPC__out PM_ACTIVATION_POLICY *pActivationPolicy);
        
        DECLSPEC_XFGVIRT(IPMTaskInfo, get_TaskType)
        HRESULT ( STDMETHODCALLTYPE *get_TaskType )( 
            __RPC__in IPMTaskInfo * This,
            /* [out] */ __RPC__out PM_TASK_TYPE *pTaskType);
        
        DECLSPEC_XFGVIRT(IPMTaskInfo, get_InvocationInfo)
        HRESULT ( STDMETHODCALLTYPE *get_InvocationInfo )( 
            __RPC__in IPMTaskInfo * This,
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pImageUrn,
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pParameters);
        
        DECLSPEC_XFGVIRT(IPMTaskInfo, get_ImagePath)
        HRESULT ( STDMETHODCALLTYPE *get_ImagePath )( 
            __RPC__in IPMTaskInfo * This,
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pImagePath);
        
        DECLSPEC_XFGVIRT(IPMTaskInfo, get_ImageParams)
        HRESULT ( STDMETHODCALLTYPE *get_ImageParams )( 
            __RPC__in IPMTaskInfo * This,
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pImageParams);
        
        DECLSPEC_XFGVIRT(IPMTaskInfo, get_InstallRootFolder)
        HRESULT ( STDMETHODCALLTYPE *get_InstallRootFolder )( 
            __RPC__in IPMTaskInfo * This,
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pInstallRootFolder);
        
        DECLSPEC_XFGVIRT(IPMTaskInfo, get_DataRootFolder)
        HRESULT ( STDMETHODCALLTYPE *get_DataRootFolder )( 
            __RPC__in IPMTaskInfo * This,
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pDataRootFolder);
        
        DECLSPEC_XFGVIRT(IPMTaskInfo, get_IsSingleInstanceHost)
        HRESULT ( STDMETHODCALLTYPE *get_IsSingleInstanceHost )( 
            __RPC__in IPMTaskInfo * This,
            /* [out] */ __RPC__out BOOL *pIsSingleInstanceHost);
        
        DECLSPEC_XFGVIRT(IPMTaskInfo, get_IsInteropEnabled)
        HRESULT ( STDMETHODCALLTYPE *get_IsInteropEnabled )( 
            __RPC__in IPMTaskInfo * This,
            /* [out] */ __RPC__out BOOL *pIsInteropEnabled);
        
        DECLSPEC_XFGVIRT(IPMTaskInfo, get_ApplicationState)
        HRESULT ( STDMETHODCALLTYPE *get_ApplicationState )( 
            __RPC__in IPMTaskInfo * This,
            /* [out] */ __RPC__out PM_APPLICATION_STATE *pApplicationState);
        
        DECLSPEC_XFGVIRT(IPMTaskInfo, get_InstallType)
        HRESULT ( STDMETHODCALLTYPE *get_InstallType )( 
            __RPC__in IPMTaskInfo * This,
            /* [out] */ __RPC__out PM_APPLICATION_INSTALL_TYPE *pInstallType);
        
        DECLSPEC_XFGVIRT(IPMTaskInfo, get_Version)
        HRESULT ( STDMETHODCALLTYPE *get_Version )( 
            __RPC__in IPMTaskInfo * This,
            /* [out] */ __RPC__out BYTE *pTargetMajorVersion,
            /* [out] */ __RPC__out BYTE *pTargetMinorVersion);
        
        DECLSPEC_XFGVIRT(IPMTaskInfo, get_BitsPerPixel)
        HRESULT ( STDMETHODCALLTYPE *get_BitsPerPixel )( 
            __RPC__in IPMTaskInfo * This,
            /* [out] */ __RPC__out USHORT *pBitsPerPixel);
        
        DECLSPEC_XFGVIRT(IPMTaskInfo, get_SuppressesDehydration)
        HRESULT ( STDMETHODCALLTYPE *get_SuppressesDehydration )( 
            __RPC__in IPMTaskInfo * This,
            /* [out] */ __RPC__out BOOL *pSuppressesDehydration);
        
        DECLSPEC_XFGVIRT(IPMTaskInfo, get_BackgroundExecutionAbilities)
        HRESULT ( STDMETHODCALLTYPE *get_BackgroundExecutionAbilities )( 
            __RPC__in IPMTaskInfo * This,
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pBackgroundExecutionAbilities);
        
        DECLSPEC_XFGVIRT(IPMTaskInfo, get_IsOptedForExtendedMem)
        HRESULT ( STDMETHODCALLTYPE *get_IsOptedForExtendedMem )( 
            __RPC__in IPMTaskInfo * This,
            /* [out] */ __RPC__out BOOL *pIsOptedIn);
        
        END_INTERFACE
    } IPMTaskInfoVtbl;

    interface IPMTaskInfo
    {
        CONST_VTBL struct IPMTaskInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPMTaskInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPMTaskInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPMTaskInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPMTaskInfo_get_ProductID(This,pProductID)	\
    ( (This)->lpVtbl -> get_ProductID(This,pProductID) ) 

#define IPMTaskInfo_get_TaskID(This,pTaskID)	\
    ( (This)->lpVtbl -> get_TaskID(This,pTaskID) ) 

#define IPMTaskInfo_get_NavigationPage(This,pNavigationPage)	\
    ( (This)->lpVtbl -> get_NavigationPage(This,pNavigationPage) ) 

#define IPMTaskInfo_get_TaskTransition(This,pTaskTransition)	\
    ( (This)->lpVtbl -> get_TaskTransition(This,pTaskTransition) ) 

#define IPMTaskInfo_get_RuntimeType(This,pRuntimetype)	\
    ( (This)->lpVtbl -> get_RuntimeType(This,pRuntimetype) ) 

#define IPMTaskInfo_get_ActivationPolicy(This,pActivationPolicy)	\
    ( (This)->lpVtbl -> get_ActivationPolicy(This,pActivationPolicy) ) 

#define IPMTaskInfo_get_TaskType(This,pTaskType)	\
    ( (This)->lpVtbl -> get_TaskType(This,pTaskType) ) 

#define IPMTaskInfo_get_InvocationInfo(This,pImageUrn,pParameters)	\
    ( (This)->lpVtbl -> get_InvocationInfo(This,pImageUrn,pParameters) ) 

#define IPMTaskInfo_get_ImagePath(This,pImagePath)	\
    ( (This)->lpVtbl -> get_ImagePath(This,pImagePath) ) 

#define IPMTaskInfo_get_ImageParams(This,pImageParams)	\
    ( (This)->lpVtbl -> get_ImageParams(This,pImageParams) ) 

#define IPMTaskInfo_get_InstallRootFolder(This,pInstallRootFolder)	\
    ( (This)->lpVtbl -> get_InstallRootFolder(This,pInstallRootFolder) ) 

#define IPMTaskInfo_get_DataRootFolder(This,pDataRootFolder)	\
    ( (This)->lpVtbl -> get_DataRootFolder(This,pDataRootFolder) ) 

#define IPMTaskInfo_get_IsSingleInstanceHost(This,pIsSingleInstanceHost)	\
    ( (This)->lpVtbl -> get_IsSingleInstanceHost(This,pIsSingleInstanceHost) ) 

#define IPMTaskInfo_get_IsInteropEnabled(This,pIsInteropEnabled)	\
    ( (This)->lpVtbl -> get_IsInteropEnabled(This,pIsInteropEnabled) ) 

#define IPMTaskInfo_get_ApplicationState(This,pApplicationState)	\
    ( (This)->lpVtbl -> get_ApplicationState(This,pApplicationState) ) 

#define IPMTaskInfo_get_InstallType(This,pInstallType)	\
    ( (This)->lpVtbl -> get_InstallType(This,pInstallType) ) 

#define IPMTaskInfo_get_Version(This,pTargetMajorVersion,pTargetMinorVersion)	\
    ( (This)->lpVtbl -> get_Version(This,pTargetMajorVersion,pTargetMinorVersion) ) 

#define IPMTaskInfo_get_BitsPerPixel(This,pBitsPerPixel)	\
    ( (This)->lpVtbl -> get_BitsPerPixel(This,pBitsPerPixel) ) 

#define IPMTaskInfo_get_SuppressesDehydration(This,pSuppressesDehydration)	\
    ( (This)->lpVtbl -> get_SuppressesDehydration(This,pSuppressesDehydration) ) 

#define IPMTaskInfo_get_BackgroundExecutionAbilities(This,pBackgroundExecutionAbilities)	\
    ( (This)->lpVtbl -> get_BackgroundExecutionAbilities(This,pBackgroundExecutionAbilities) ) 

#define IPMTaskInfo_get_IsOptedForExtendedMem(This,pIsOptedIn)	\
    ( (This)->lpVtbl -> get_IsOptedForExtendedMem(This,pIsOptedIn) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPMTaskInfo_INTERFACE_DEFINED__ */


#ifndef __IPMTaskInfoEnumerator_INTERFACE_DEFINED__
#define __IPMTaskInfoEnumerator_INTERFACE_DEFINED__

/* interface IPMTaskInfoEnumerator */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IPMTaskInfoEnumerator;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0630B0F8-0BBC-4821-BE74-C7995166ED2A")
    IPMTaskInfoEnumerator : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE get_Next( 
            /* [out] */ __RPC__deref_out_opt IPMTaskInfo **ppTaskInfo) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPMTaskInfoEnumeratorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPMTaskInfoEnumerator * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPMTaskInfoEnumerator * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPMTaskInfoEnumerator * This);
        
        DECLSPEC_XFGVIRT(IPMTaskInfoEnumerator, get_Next)
        HRESULT ( STDMETHODCALLTYPE *get_Next )( 
            __RPC__in IPMTaskInfoEnumerator * This,
            /* [out] */ __RPC__deref_out_opt IPMTaskInfo **ppTaskInfo);
        
        END_INTERFACE
    } IPMTaskInfoEnumeratorVtbl;

    interface IPMTaskInfoEnumerator
    {
        CONST_VTBL struct IPMTaskInfoEnumeratorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPMTaskInfoEnumerator_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPMTaskInfoEnumerator_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPMTaskInfoEnumerator_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPMTaskInfoEnumerator_get_Next(This,ppTaskInfo)	\
    ( (This)->lpVtbl -> get_Next(This,ppTaskInfo) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPMTaskInfoEnumerator_INTERFACE_DEFINED__ */


#ifndef __IPMExtensionInfo_INTERFACE_DEFINED__
#define __IPMExtensionInfo_INTERFACE_DEFINED__

/* interface IPMExtensionInfo */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IPMExtensionInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("49ACDE79-9788-4d0a-8AA0-1746AFDB9E9D")
    IPMExtensionInfo : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE get_SupplierPID( 
            /* [out] */ __RPC__out PRODUCTID *pSupplierPID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_SupplierTaskID( 
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pSupplierTID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_Title( 
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pTitle) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_IconPath( 
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pIconPath) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_ExtraFile( 
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pFilePath) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_InvocationInfo( 
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pImageUrn,
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pParameters) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPMExtensionInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPMExtensionInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPMExtensionInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPMExtensionInfo * This);
        
        DECLSPEC_XFGVIRT(IPMExtensionInfo, get_SupplierPID)
        HRESULT ( STDMETHODCALLTYPE *get_SupplierPID )( 
            __RPC__in IPMExtensionInfo * This,
            /* [out] */ __RPC__out PRODUCTID *pSupplierPID);
        
        DECLSPEC_XFGVIRT(IPMExtensionInfo, get_SupplierTaskID)
        HRESULT ( STDMETHODCALLTYPE *get_SupplierTaskID )( 
            __RPC__in IPMExtensionInfo * This,
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pSupplierTID);
        
        DECLSPEC_XFGVIRT(IPMExtensionInfo, get_Title)
        HRESULT ( STDMETHODCALLTYPE *get_Title )( 
            __RPC__in IPMExtensionInfo * This,
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pTitle);
        
        DECLSPEC_XFGVIRT(IPMExtensionInfo, get_IconPath)
        HRESULT ( STDMETHODCALLTYPE *get_IconPath )( 
            __RPC__in IPMExtensionInfo * This,
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pIconPath);
        
        DECLSPEC_XFGVIRT(IPMExtensionInfo, get_ExtraFile)
        HRESULT ( STDMETHODCALLTYPE *get_ExtraFile )( 
            __RPC__in IPMExtensionInfo * This,
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pFilePath);
        
        DECLSPEC_XFGVIRT(IPMExtensionInfo, get_InvocationInfo)
        HRESULT ( STDMETHODCALLTYPE *get_InvocationInfo )( 
            __RPC__in IPMExtensionInfo * This,
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pImageUrn,
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pParameters);
        
        END_INTERFACE
    } IPMExtensionInfoVtbl;

    interface IPMExtensionInfo
    {
        CONST_VTBL struct IPMExtensionInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPMExtensionInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPMExtensionInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPMExtensionInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPMExtensionInfo_get_SupplierPID(This,pSupplierPID)	\
    ( (This)->lpVtbl -> get_SupplierPID(This,pSupplierPID) ) 

#define IPMExtensionInfo_get_SupplierTaskID(This,pSupplierTID)	\
    ( (This)->lpVtbl -> get_SupplierTaskID(This,pSupplierTID) ) 

#define IPMExtensionInfo_get_Title(This,pTitle)	\
    ( (This)->lpVtbl -> get_Title(This,pTitle) ) 

#define IPMExtensionInfo_get_IconPath(This,pIconPath)	\
    ( (This)->lpVtbl -> get_IconPath(This,pIconPath) ) 

#define IPMExtensionInfo_get_ExtraFile(This,pFilePath)	\
    ( (This)->lpVtbl -> get_ExtraFile(This,pFilePath) ) 

#define IPMExtensionInfo_get_InvocationInfo(This,pImageUrn,pParameters)	\
    ( (This)->lpVtbl -> get_InvocationInfo(This,pImageUrn,pParameters) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPMExtensionInfo_INTERFACE_DEFINED__ */


#ifndef __IPMExtensionFileExtensionInfo_INTERFACE_DEFINED__
#define __IPMExtensionFileExtensionInfo_INTERFACE_DEFINED__

/* interface IPMExtensionFileExtensionInfo */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IPMExtensionFileExtensionInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6B87CB6C-0B88-4989-A4EC-033714F710D4")
    IPMExtensionFileExtensionInfo : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE get_Name( 
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_DisplayName( 
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pDisplayName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_Logo( 
            /* [in] */ PM_LOGO_SIZE LogoSize,
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pLogo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_ContentType( 
            /* [string][ref][in] */ __RPC__in_string BSTR FileType,
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pContentType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_FileType( 
            /* [string][in] */ __RPC__in_string BSTR ContentType,
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pFileType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_InvocationInfo( 
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pImageUrn,
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pParameters) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_AllFileTypes( 
            /* [out] */ __RPC__out DWORD *pcbTypes,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcbTypes) BSTR **ppTypes) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPMExtensionFileExtensionInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPMExtensionFileExtensionInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPMExtensionFileExtensionInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPMExtensionFileExtensionInfo * This);
        
        DECLSPEC_XFGVIRT(IPMExtensionFileExtensionInfo, get_Name)
        HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IPMExtensionFileExtensionInfo * This,
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pName);
        
        DECLSPEC_XFGVIRT(IPMExtensionFileExtensionInfo, get_DisplayName)
        HRESULT ( STDMETHODCALLTYPE *get_DisplayName )( 
            __RPC__in IPMExtensionFileExtensionInfo * This,
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pDisplayName);
        
        DECLSPEC_XFGVIRT(IPMExtensionFileExtensionInfo, get_Logo)
        HRESULT ( STDMETHODCALLTYPE *get_Logo )( 
            __RPC__in IPMExtensionFileExtensionInfo * This,
            /* [in] */ PM_LOGO_SIZE LogoSize,
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pLogo);
        
        DECLSPEC_XFGVIRT(IPMExtensionFileExtensionInfo, get_ContentType)
        HRESULT ( STDMETHODCALLTYPE *get_ContentType )( 
            __RPC__in IPMExtensionFileExtensionInfo * This,
            /* [string][ref][in] */ __RPC__in_string BSTR FileType,
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pContentType);
        
        DECLSPEC_XFGVIRT(IPMExtensionFileExtensionInfo, get_FileType)
        HRESULT ( STDMETHODCALLTYPE *get_FileType )( 
            __RPC__in IPMExtensionFileExtensionInfo * This,
            /* [string][in] */ __RPC__in_string BSTR ContentType,
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pFileType);
        
        DECLSPEC_XFGVIRT(IPMExtensionFileExtensionInfo, get_InvocationInfo)
        HRESULT ( STDMETHODCALLTYPE *get_InvocationInfo )( 
            __RPC__in IPMExtensionFileExtensionInfo * This,
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pImageUrn,
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pParameters);
        
        DECLSPEC_XFGVIRT(IPMExtensionFileExtensionInfo, get_AllFileTypes)
        HRESULT ( STDMETHODCALLTYPE *get_AllFileTypes )( 
            __RPC__in IPMExtensionFileExtensionInfo * This,
            /* [out] */ __RPC__out DWORD *pcbTypes,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcbTypes) BSTR **ppTypes);
        
        END_INTERFACE
    } IPMExtensionFileExtensionInfoVtbl;

    interface IPMExtensionFileExtensionInfo
    {
        CONST_VTBL struct IPMExtensionFileExtensionInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPMExtensionFileExtensionInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPMExtensionFileExtensionInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPMExtensionFileExtensionInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPMExtensionFileExtensionInfo_get_Name(This,pName)	\
    ( (This)->lpVtbl -> get_Name(This,pName) ) 

#define IPMExtensionFileExtensionInfo_get_DisplayName(This,pDisplayName)	\
    ( (This)->lpVtbl -> get_DisplayName(This,pDisplayName) ) 

#define IPMExtensionFileExtensionInfo_get_Logo(This,LogoSize,pLogo)	\
    ( (This)->lpVtbl -> get_Logo(This,LogoSize,pLogo) ) 

#define IPMExtensionFileExtensionInfo_get_ContentType(This,FileType,pContentType)	\
    ( (This)->lpVtbl -> get_ContentType(This,FileType,pContentType) ) 

#define IPMExtensionFileExtensionInfo_get_FileType(This,ContentType,pFileType)	\
    ( (This)->lpVtbl -> get_FileType(This,ContentType,pFileType) ) 

#define IPMExtensionFileExtensionInfo_get_InvocationInfo(This,pImageUrn,pParameters)	\
    ( (This)->lpVtbl -> get_InvocationInfo(This,pImageUrn,pParameters) ) 

#define IPMExtensionFileExtensionInfo_get_AllFileTypes(This,pcbTypes,ppTypes)	\
    ( (This)->lpVtbl -> get_AllFileTypes(This,pcbTypes,ppTypes) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPMExtensionFileExtensionInfo_INTERFACE_DEFINED__ */


#ifndef __IPMExtensionProtocolInfo_INTERFACE_DEFINED__
#define __IPMExtensionProtocolInfo_INTERFACE_DEFINED__

/* interface IPMExtensionProtocolInfo */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IPMExtensionProtocolInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1E3FA036-51EB-4453-BAFF-B8D8E4B46C8E")
    IPMExtensionProtocolInfo : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE get_Protocol( 
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pProtocol) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_InvocationInfo( 
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pImageUrn,
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pParameters) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPMExtensionProtocolInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPMExtensionProtocolInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPMExtensionProtocolInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPMExtensionProtocolInfo * This);
        
        DECLSPEC_XFGVIRT(IPMExtensionProtocolInfo, get_Protocol)
        HRESULT ( STDMETHODCALLTYPE *get_Protocol )( 
            __RPC__in IPMExtensionProtocolInfo * This,
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pProtocol);
        
        DECLSPEC_XFGVIRT(IPMExtensionProtocolInfo, get_InvocationInfo)
        HRESULT ( STDMETHODCALLTYPE *get_InvocationInfo )( 
            __RPC__in IPMExtensionProtocolInfo * This,
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pImageUrn,
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pParameters);
        
        END_INTERFACE
    } IPMExtensionProtocolInfoVtbl;

    interface IPMExtensionProtocolInfo
    {
        CONST_VTBL struct IPMExtensionProtocolInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPMExtensionProtocolInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPMExtensionProtocolInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPMExtensionProtocolInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPMExtensionProtocolInfo_get_Protocol(This,pProtocol)	\
    ( (This)->lpVtbl -> get_Protocol(This,pProtocol) ) 

#define IPMExtensionProtocolInfo_get_InvocationInfo(This,pImageUrn,pParameters)	\
    ( (This)->lpVtbl -> get_InvocationInfo(This,pImageUrn,pParameters) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPMExtensionProtocolInfo_INTERFACE_DEFINED__ */


#ifndef __IPMExtensionShareTargetInfo_INTERFACE_DEFINED__
#define __IPMExtensionShareTargetInfo_INTERFACE_DEFINED__

/* interface IPMExtensionShareTargetInfo */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IPMExtensionShareTargetInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5471F48B-C65C-4656-8C70-242E31195FEA")
    IPMExtensionShareTargetInfo : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE get_AllFileTypes( 
            /* [out] */ __RPC__out DWORD *pcTypes,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcTypes) BSTR **ppTypes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_AllDataFormats( 
            /* [out] */ __RPC__out DWORD *pcDataFormats,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcDataFormats) BSTR **ppDataFormats) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_SupportsAllFileTypes( 
            /* [out] */ __RPC__out BOOL *pSupportsAllTypes) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPMExtensionShareTargetInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPMExtensionShareTargetInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPMExtensionShareTargetInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPMExtensionShareTargetInfo * This);
        
        DECLSPEC_XFGVIRT(IPMExtensionShareTargetInfo, get_AllFileTypes)
        HRESULT ( STDMETHODCALLTYPE *get_AllFileTypes )( 
            __RPC__in IPMExtensionShareTargetInfo * This,
            /* [out] */ __RPC__out DWORD *pcTypes,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcTypes) BSTR **ppTypes);
        
        DECLSPEC_XFGVIRT(IPMExtensionShareTargetInfo, get_AllDataFormats)
        HRESULT ( STDMETHODCALLTYPE *get_AllDataFormats )( 
            __RPC__in IPMExtensionShareTargetInfo * This,
            /* [out] */ __RPC__out DWORD *pcDataFormats,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcDataFormats) BSTR **ppDataFormats);
        
        DECLSPEC_XFGVIRT(IPMExtensionShareTargetInfo, get_SupportsAllFileTypes)
        HRESULT ( STDMETHODCALLTYPE *get_SupportsAllFileTypes )( 
            __RPC__in IPMExtensionShareTargetInfo * This,
            /* [out] */ __RPC__out BOOL *pSupportsAllTypes);
        
        END_INTERFACE
    } IPMExtensionShareTargetInfoVtbl;

    interface IPMExtensionShareTargetInfo
    {
        CONST_VTBL struct IPMExtensionShareTargetInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPMExtensionShareTargetInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPMExtensionShareTargetInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPMExtensionShareTargetInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPMExtensionShareTargetInfo_get_AllFileTypes(This,pcTypes,ppTypes)	\
    ( (This)->lpVtbl -> get_AllFileTypes(This,pcTypes,ppTypes) ) 

#define IPMExtensionShareTargetInfo_get_AllDataFormats(This,pcDataFormats,ppDataFormats)	\
    ( (This)->lpVtbl -> get_AllDataFormats(This,pcDataFormats,ppDataFormats) ) 

#define IPMExtensionShareTargetInfo_get_SupportsAllFileTypes(This,pSupportsAllTypes)	\
    ( (This)->lpVtbl -> get_SupportsAllFileTypes(This,pSupportsAllTypes) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPMExtensionShareTargetInfo_INTERFACE_DEFINED__ */


#ifndef __IPMExtensionContractInfo_INTERFACE_DEFINED__
#define __IPMExtensionContractInfo_INTERFACE_DEFINED__

/* interface IPMExtensionContractInfo */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IPMExtensionContractInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("E5666373-7BA1-467C-B819-B175DB1C295B")
    IPMExtensionContractInfo : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE get_InvocationInfo( 
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pAUMID,
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pArgs) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPMExtensionContractInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPMExtensionContractInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPMExtensionContractInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPMExtensionContractInfo * This);
        
        DECLSPEC_XFGVIRT(IPMExtensionContractInfo, get_InvocationInfo)
        HRESULT ( STDMETHODCALLTYPE *get_InvocationInfo )( 
            __RPC__in IPMExtensionContractInfo * This,
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pAUMID,
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pArgs);
        
        END_INTERFACE
    } IPMExtensionContractInfoVtbl;

    interface IPMExtensionContractInfo
    {
        CONST_VTBL struct IPMExtensionContractInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPMExtensionContractInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPMExtensionContractInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPMExtensionContractInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPMExtensionContractInfo_get_InvocationInfo(This,pAUMID,pArgs)	\
    ( (This)->lpVtbl -> get_InvocationInfo(This,pAUMID,pArgs) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPMExtensionContractInfo_INTERFACE_DEFINED__ */


#ifndef __IPMExtensionFileOpenPickerInfo_INTERFACE_DEFINED__
#define __IPMExtensionFileOpenPickerInfo_INTERFACE_DEFINED__

/* interface IPMExtensionFileOpenPickerInfo */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IPMExtensionFileOpenPickerInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6DC91D25-9606-420C-9A78-E034A3418345")
    IPMExtensionFileOpenPickerInfo : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE get_AllFileTypes( 
            /* [out] */ __RPC__out DWORD *pcTypes,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcTypes) BSTR **ppTypes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_SupportsAllFileTypes( 
            /* [out] */ __RPC__out BOOL *pSupportsAllTypes) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPMExtensionFileOpenPickerInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPMExtensionFileOpenPickerInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPMExtensionFileOpenPickerInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPMExtensionFileOpenPickerInfo * This);
        
        DECLSPEC_XFGVIRT(IPMExtensionFileOpenPickerInfo, get_AllFileTypes)
        HRESULT ( STDMETHODCALLTYPE *get_AllFileTypes )( 
            __RPC__in IPMExtensionFileOpenPickerInfo * This,
            /* [out] */ __RPC__out DWORD *pcTypes,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcTypes) BSTR **ppTypes);
        
        DECLSPEC_XFGVIRT(IPMExtensionFileOpenPickerInfo, get_SupportsAllFileTypes)
        HRESULT ( STDMETHODCALLTYPE *get_SupportsAllFileTypes )( 
            __RPC__in IPMExtensionFileOpenPickerInfo * This,
            /* [out] */ __RPC__out BOOL *pSupportsAllTypes);
        
        END_INTERFACE
    } IPMExtensionFileOpenPickerInfoVtbl;

    interface IPMExtensionFileOpenPickerInfo
    {
        CONST_VTBL struct IPMExtensionFileOpenPickerInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPMExtensionFileOpenPickerInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPMExtensionFileOpenPickerInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPMExtensionFileOpenPickerInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPMExtensionFileOpenPickerInfo_get_AllFileTypes(This,pcTypes,ppTypes)	\
    ( (This)->lpVtbl -> get_AllFileTypes(This,pcTypes,ppTypes) ) 

#define IPMExtensionFileOpenPickerInfo_get_SupportsAllFileTypes(This,pSupportsAllTypes)	\
    ( (This)->lpVtbl -> get_SupportsAllFileTypes(This,pSupportsAllTypes) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPMExtensionFileOpenPickerInfo_INTERFACE_DEFINED__ */


#ifndef __IPMExtensionFileSavePickerInfo_INTERFACE_DEFINED__
#define __IPMExtensionFileSavePickerInfo_INTERFACE_DEFINED__

/* interface IPMExtensionFileSavePickerInfo */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IPMExtensionFileSavePickerInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("38005CBA-F81A-493E-A0F8-922C8680DA43")
    IPMExtensionFileSavePickerInfo : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE get_AllFileTypes( 
            /* [out] */ __RPC__out DWORD *pcTypes,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcTypes) BSTR **ppTypes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_SupportsAllFileTypes( 
            /* [out] */ __RPC__out BOOL *pSupportsAllTypes) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPMExtensionFileSavePickerInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPMExtensionFileSavePickerInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPMExtensionFileSavePickerInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPMExtensionFileSavePickerInfo * This);
        
        DECLSPEC_XFGVIRT(IPMExtensionFileSavePickerInfo, get_AllFileTypes)
        HRESULT ( STDMETHODCALLTYPE *get_AllFileTypes )( 
            __RPC__in IPMExtensionFileSavePickerInfo * This,
            /* [out] */ __RPC__out DWORD *pcTypes,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcTypes) BSTR **ppTypes);
        
        DECLSPEC_XFGVIRT(IPMExtensionFileSavePickerInfo, get_SupportsAllFileTypes)
        HRESULT ( STDMETHODCALLTYPE *get_SupportsAllFileTypes )( 
            __RPC__in IPMExtensionFileSavePickerInfo * This,
            /* [out] */ __RPC__out BOOL *pSupportsAllTypes);
        
        END_INTERFACE
    } IPMExtensionFileSavePickerInfoVtbl;

    interface IPMExtensionFileSavePickerInfo
    {
        CONST_VTBL struct IPMExtensionFileSavePickerInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPMExtensionFileSavePickerInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPMExtensionFileSavePickerInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPMExtensionFileSavePickerInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPMExtensionFileSavePickerInfo_get_AllFileTypes(This,pcTypes,ppTypes)	\
    ( (This)->lpVtbl -> get_AllFileTypes(This,pcTypes,ppTypes) ) 

#define IPMExtensionFileSavePickerInfo_get_SupportsAllFileTypes(This,pSupportsAllTypes)	\
    ( (This)->lpVtbl -> get_SupportsAllFileTypes(This,pSupportsAllTypes) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPMExtensionFileSavePickerInfo_INTERFACE_DEFINED__ */


#ifndef __IPMExtensionCachedFileUpdaterInfo_INTERFACE_DEFINED__
#define __IPMExtensionCachedFileUpdaterInfo_INTERFACE_DEFINED__

/* interface IPMExtensionCachedFileUpdaterInfo */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IPMExtensionCachedFileUpdaterInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("E2D77509-4E58-4BA9-AF7E-B642E370E1B0")
    IPMExtensionCachedFileUpdaterInfo : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE get_SupportsUpdates( 
            /* [out] */ __RPC__out BOOL *pSupportsUpdates) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPMExtensionCachedFileUpdaterInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPMExtensionCachedFileUpdaterInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPMExtensionCachedFileUpdaterInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPMExtensionCachedFileUpdaterInfo * This);
        
        DECLSPEC_XFGVIRT(IPMExtensionCachedFileUpdaterInfo, get_SupportsUpdates)
        HRESULT ( STDMETHODCALLTYPE *get_SupportsUpdates )( 
            __RPC__in IPMExtensionCachedFileUpdaterInfo * This,
            /* [out] */ __RPC__out BOOL *pSupportsUpdates);
        
        END_INTERFACE
    } IPMExtensionCachedFileUpdaterInfoVtbl;

    interface IPMExtensionCachedFileUpdaterInfo
    {
        CONST_VTBL struct IPMExtensionCachedFileUpdaterInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPMExtensionCachedFileUpdaterInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPMExtensionCachedFileUpdaterInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPMExtensionCachedFileUpdaterInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPMExtensionCachedFileUpdaterInfo_get_SupportsUpdates(This,pSupportsUpdates)	\
    ( (This)->lpVtbl -> get_SupportsUpdates(This,pSupportsUpdates) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPMExtensionCachedFileUpdaterInfo_INTERFACE_DEFINED__ */


#ifndef __IPMExtensionInfoEnumerator_INTERFACE_DEFINED__
#define __IPMExtensionInfoEnumerator_INTERFACE_DEFINED__

/* interface IPMExtensionInfoEnumerator */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IPMExtensionInfoEnumerator;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("403B9E82-1171-4573-8E6F-6F33F39B83DD")
    IPMExtensionInfoEnumerator : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE get_Next( 
            /* [out] */ __RPC__deref_out_opt IPMExtensionInfo **ppExtensionInfo) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPMExtensionInfoEnumeratorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPMExtensionInfoEnumerator * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPMExtensionInfoEnumerator * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPMExtensionInfoEnumerator * This);
        
        DECLSPEC_XFGVIRT(IPMExtensionInfoEnumerator, get_Next)
        HRESULT ( STDMETHODCALLTYPE *get_Next )( 
            __RPC__in IPMExtensionInfoEnumerator * This,
            /* [out] */ __RPC__deref_out_opt IPMExtensionInfo **ppExtensionInfo);
        
        END_INTERFACE
    } IPMExtensionInfoEnumeratorVtbl;

    interface IPMExtensionInfoEnumerator
    {
        CONST_VTBL struct IPMExtensionInfoEnumeratorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPMExtensionInfoEnumerator_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPMExtensionInfoEnumerator_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPMExtensionInfoEnumerator_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPMExtensionInfoEnumerator_get_Next(This,ppExtensionInfo)	\
    ( (This)->lpVtbl -> get_Next(This,ppExtensionInfo) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPMExtensionInfoEnumerator_INTERFACE_DEFINED__ */


#ifndef __IPMBackgroundServiceAgentInfo_INTERFACE_DEFINED__
#define __IPMBackgroundServiceAgentInfo_INTERFACE_DEFINED__

/* interface IPMBackgroundServiceAgentInfo */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IPMBackgroundServiceAgentInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3A8B46DA-928C-4879-998C-09DC96F3D490")
    IPMBackgroundServiceAgentInfo : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE get_ProductID( 
            /* [out] */ __RPC__out PRODUCTID *pProductID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_TaskID( 
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pTaskID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_BSAID( 
            /* [out] */ __RPC__out DWORD *pBSAID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_BGSpecifier( 
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pBGSpecifier) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_BGName( 
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pBGName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_BGSource( 
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pBGSource) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_BGType( 
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pBGType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_IsPeriodic( 
            /* [out] */ __RPC__out BOOL *pIsPeriodic) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_IsScheduled( 
            /* [out] */ __RPC__out BOOL *pIsScheduled) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_IsScheduleAllowed( 
            /* [out] */ __RPC__out BOOL *pIsScheduleAllowed) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_Description( 
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pDescription) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_IsLaunchOnBoot( 
            /* [out] */ __RPC__out BOOL *pLaunchOnBoot) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE set_IsScheduled( 
            /* [in] */ BOOL IsScheduled) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE set_IsScheduleAllowed( 
            /* [in] */ BOOL IsScheduleAllowed) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPMBackgroundServiceAgentInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPMBackgroundServiceAgentInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPMBackgroundServiceAgentInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPMBackgroundServiceAgentInfo * This);
        
        DECLSPEC_XFGVIRT(IPMBackgroundServiceAgentInfo, get_ProductID)
        HRESULT ( STDMETHODCALLTYPE *get_ProductID )( 
            __RPC__in IPMBackgroundServiceAgentInfo * This,
            /* [out] */ __RPC__out PRODUCTID *pProductID);
        
        DECLSPEC_XFGVIRT(IPMBackgroundServiceAgentInfo, get_TaskID)
        HRESULT ( STDMETHODCALLTYPE *get_TaskID )( 
            __RPC__in IPMBackgroundServiceAgentInfo * This,
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pTaskID);
        
        DECLSPEC_XFGVIRT(IPMBackgroundServiceAgentInfo, get_BSAID)
        HRESULT ( STDMETHODCALLTYPE *get_BSAID )( 
            __RPC__in IPMBackgroundServiceAgentInfo * This,
            /* [out] */ __RPC__out DWORD *pBSAID);
        
        DECLSPEC_XFGVIRT(IPMBackgroundServiceAgentInfo, get_BGSpecifier)
        HRESULT ( STDMETHODCALLTYPE *get_BGSpecifier )( 
            __RPC__in IPMBackgroundServiceAgentInfo * This,
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pBGSpecifier);
        
        DECLSPEC_XFGVIRT(IPMBackgroundServiceAgentInfo, get_BGName)
        HRESULT ( STDMETHODCALLTYPE *get_BGName )( 
            __RPC__in IPMBackgroundServiceAgentInfo * This,
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pBGName);
        
        DECLSPEC_XFGVIRT(IPMBackgroundServiceAgentInfo, get_BGSource)
        HRESULT ( STDMETHODCALLTYPE *get_BGSource )( 
            __RPC__in IPMBackgroundServiceAgentInfo * This,
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pBGSource);
        
        DECLSPEC_XFGVIRT(IPMBackgroundServiceAgentInfo, get_BGType)
        HRESULT ( STDMETHODCALLTYPE *get_BGType )( 
            __RPC__in IPMBackgroundServiceAgentInfo * This,
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pBGType);
        
        DECLSPEC_XFGVIRT(IPMBackgroundServiceAgentInfo, get_IsPeriodic)
        HRESULT ( STDMETHODCALLTYPE *get_IsPeriodic )( 
            __RPC__in IPMBackgroundServiceAgentInfo * This,
            /* [out] */ __RPC__out BOOL *pIsPeriodic);
        
        DECLSPEC_XFGVIRT(IPMBackgroundServiceAgentInfo, get_IsScheduled)
        HRESULT ( STDMETHODCALLTYPE *get_IsScheduled )( 
            __RPC__in IPMBackgroundServiceAgentInfo * This,
            /* [out] */ __RPC__out BOOL *pIsScheduled);
        
        DECLSPEC_XFGVIRT(IPMBackgroundServiceAgentInfo, get_IsScheduleAllowed)
        HRESULT ( STDMETHODCALLTYPE *get_IsScheduleAllowed )( 
            __RPC__in IPMBackgroundServiceAgentInfo * This,
            /* [out] */ __RPC__out BOOL *pIsScheduleAllowed);
        
        DECLSPEC_XFGVIRT(IPMBackgroundServiceAgentInfo, get_Description)
        HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in IPMBackgroundServiceAgentInfo * This,
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pDescription);
        
        DECLSPEC_XFGVIRT(IPMBackgroundServiceAgentInfo, get_IsLaunchOnBoot)
        HRESULT ( STDMETHODCALLTYPE *get_IsLaunchOnBoot )( 
            __RPC__in IPMBackgroundServiceAgentInfo * This,
            /* [out] */ __RPC__out BOOL *pLaunchOnBoot);
        
        DECLSPEC_XFGVIRT(IPMBackgroundServiceAgentInfo, set_IsScheduled)
        HRESULT ( STDMETHODCALLTYPE *set_IsScheduled )( 
            __RPC__in IPMBackgroundServiceAgentInfo * This,
            /* [in] */ BOOL IsScheduled);
        
        DECLSPEC_XFGVIRT(IPMBackgroundServiceAgentInfo, set_IsScheduleAllowed)
        HRESULT ( STDMETHODCALLTYPE *set_IsScheduleAllowed )( 
            __RPC__in IPMBackgroundServiceAgentInfo * This,
            /* [in] */ BOOL IsScheduleAllowed);
        
        END_INTERFACE
    } IPMBackgroundServiceAgentInfoVtbl;

    interface IPMBackgroundServiceAgentInfo
    {
        CONST_VTBL struct IPMBackgroundServiceAgentInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPMBackgroundServiceAgentInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPMBackgroundServiceAgentInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPMBackgroundServiceAgentInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPMBackgroundServiceAgentInfo_get_ProductID(This,pProductID)	\
    ( (This)->lpVtbl -> get_ProductID(This,pProductID) ) 

#define IPMBackgroundServiceAgentInfo_get_TaskID(This,pTaskID)	\
    ( (This)->lpVtbl -> get_TaskID(This,pTaskID) ) 

#define IPMBackgroundServiceAgentInfo_get_BSAID(This,pBSAID)	\
    ( (This)->lpVtbl -> get_BSAID(This,pBSAID) ) 

#define IPMBackgroundServiceAgentInfo_get_BGSpecifier(This,pBGSpecifier)	\
    ( (This)->lpVtbl -> get_BGSpecifier(This,pBGSpecifier) ) 

#define IPMBackgroundServiceAgentInfo_get_BGName(This,pBGName)	\
    ( (This)->lpVtbl -> get_BGName(This,pBGName) ) 

#define IPMBackgroundServiceAgentInfo_get_BGSource(This,pBGSource)	\
    ( (This)->lpVtbl -> get_BGSource(This,pBGSource) ) 

#define IPMBackgroundServiceAgentInfo_get_BGType(This,pBGType)	\
    ( (This)->lpVtbl -> get_BGType(This,pBGType) ) 

#define IPMBackgroundServiceAgentInfo_get_IsPeriodic(This,pIsPeriodic)	\
    ( (This)->lpVtbl -> get_IsPeriodic(This,pIsPeriodic) ) 

#define IPMBackgroundServiceAgentInfo_get_IsScheduled(This,pIsScheduled)	\
    ( (This)->lpVtbl -> get_IsScheduled(This,pIsScheduled) ) 

#define IPMBackgroundServiceAgentInfo_get_IsScheduleAllowed(This,pIsScheduleAllowed)	\
    ( (This)->lpVtbl -> get_IsScheduleAllowed(This,pIsScheduleAllowed) ) 

#define IPMBackgroundServiceAgentInfo_get_Description(This,pDescription)	\
    ( (This)->lpVtbl -> get_Description(This,pDescription) ) 

#define IPMBackgroundServiceAgentInfo_get_IsLaunchOnBoot(This,pLaunchOnBoot)	\
    ( (This)->lpVtbl -> get_IsLaunchOnBoot(This,pLaunchOnBoot) ) 

#define IPMBackgroundServiceAgentInfo_set_IsScheduled(This,IsScheduled)	\
    ( (This)->lpVtbl -> set_IsScheduled(This,IsScheduled) ) 

#define IPMBackgroundServiceAgentInfo_set_IsScheduleAllowed(This,IsScheduleAllowed)	\
    ( (This)->lpVtbl -> set_IsScheduleAllowed(This,IsScheduleAllowed) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPMBackgroundServiceAgentInfo_INTERFACE_DEFINED__ */


#ifndef __IPMBackgroundWorkerInfo_INTERFACE_DEFINED__
#define __IPMBackgroundWorkerInfo_INTERFACE_DEFINED__

/* interface IPMBackgroundWorkerInfo */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IPMBackgroundWorkerInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7DD4531B-D3BF-4B6B-94F3-69C098B1497D")
    IPMBackgroundWorkerInfo : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE get_ProductID( 
            /* [out] */ __RPC__out PRODUCTID *pProductID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_TaskID( 
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pTaskID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_BGName( 
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pBGName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_MaxStartupLatency( 
            /* [out] */ __RPC__out DWORD *pMaxStartupLatency) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_ExpectedRuntime( 
            /* [out] */ __RPC__out DWORD *pExpectedRuntime) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_IsBootWorker( 
            /* [out] */ __RPC__out BOOL *pIsBootWorker) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPMBackgroundWorkerInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPMBackgroundWorkerInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPMBackgroundWorkerInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPMBackgroundWorkerInfo * This);
        
        DECLSPEC_XFGVIRT(IPMBackgroundWorkerInfo, get_ProductID)
        HRESULT ( STDMETHODCALLTYPE *get_ProductID )( 
            __RPC__in IPMBackgroundWorkerInfo * This,
            /* [out] */ __RPC__out PRODUCTID *pProductID);
        
        DECLSPEC_XFGVIRT(IPMBackgroundWorkerInfo, get_TaskID)
        HRESULT ( STDMETHODCALLTYPE *get_TaskID )( 
            __RPC__in IPMBackgroundWorkerInfo * This,
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pTaskID);
        
        DECLSPEC_XFGVIRT(IPMBackgroundWorkerInfo, get_BGName)
        HRESULT ( STDMETHODCALLTYPE *get_BGName )( 
            __RPC__in IPMBackgroundWorkerInfo * This,
            /* [string][out][in] */ __RPC__deref_inout_opt_string BSTR *pBGName);
        
        DECLSPEC_XFGVIRT(IPMBackgroundWorkerInfo, get_MaxStartupLatency)
        HRESULT ( STDMETHODCALLTYPE *get_MaxStartupLatency )( 
            __RPC__in IPMBackgroundWorkerInfo * This,
            /* [out] */ __RPC__out DWORD *pMaxStartupLatency);
        
        DECLSPEC_XFGVIRT(IPMBackgroundWorkerInfo, get_ExpectedRuntime)
        HRESULT ( STDMETHODCALLTYPE *get_ExpectedRuntime )( 
            __RPC__in IPMBackgroundWorkerInfo * This,
            /* [out] */ __RPC__out DWORD *pExpectedRuntime);
        
        DECLSPEC_XFGVIRT(IPMBackgroundWorkerInfo, get_IsBootWorker)
        HRESULT ( STDMETHODCALLTYPE *get_IsBootWorker )( 
            __RPC__in IPMBackgroundWorkerInfo * This,
            /* [out] */ __RPC__out BOOL *pIsBootWorker);
        
        END_INTERFACE
    } IPMBackgroundWorkerInfoVtbl;

    interface IPMBackgroundWorkerInfo
    {
        CONST_VTBL struct IPMBackgroundWorkerInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPMBackgroundWorkerInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPMBackgroundWorkerInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPMBackgroundWorkerInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPMBackgroundWorkerInfo_get_ProductID(This,pProductID)	\
    ( (This)->lpVtbl -> get_ProductID(This,pProductID) ) 

#define IPMBackgroundWorkerInfo_get_TaskID(This,pTaskID)	\
    ( (This)->lpVtbl -> get_TaskID(This,pTaskID) ) 

#define IPMBackgroundWorkerInfo_get_BGName(This,pBGName)	\
    ( (This)->lpVtbl -> get_BGName(This,pBGName) ) 

#define IPMBackgroundWorkerInfo_get_MaxStartupLatency(This,pMaxStartupLatency)	\
    ( (This)->lpVtbl -> get_MaxStartupLatency(This,pMaxStartupLatency) ) 

#define IPMBackgroundWorkerInfo_get_ExpectedRuntime(This,pExpectedRuntime)	\
    ( (This)->lpVtbl -> get_ExpectedRuntime(This,pExpectedRuntime) ) 

#define IPMBackgroundWorkerInfo_get_IsBootWorker(This,pIsBootWorker)	\
    ( (This)->lpVtbl -> get_IsBootWorker(This,pIsBootWorker) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPMBackgroundWorkerInfo_INTERFACE_DEFINED__ */


#ifndef __IPMBackgroundServiceAgentInfoEnumerator_INTERFACE_DEFINED__
#define __IPMBackgroundServiceAgentInfoEnumerator_INTERFACE_DEFINED__

/* interface IPMBackgroundServiceAgentInfoEnumerator */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IPMBackgroundServiceAgentInfoEnumerator;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("18EB2072-AB56-43B3-872C-BEAFB7A6B391")
    IPMBackgroundServiceAgentInfoEnumerator : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE get_Next( 
            /* [out] */ __RPC__deref_out_opt IPMBackgroundServiceAgentInfo **ppBSAInfo) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPMBackgroundServiceAgentInfoEnumeratorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPMBackgroundServiceAgentInfoEnumerator * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPMBackgroundServiceAgentInfoEnumerator * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPMBackgroundServiceAgentInfoEnumerator * This);
        
        DECLSPEC_XFGVIRT(IPMBackgroundServiceAgentInfoEnumerator, get_Next)
        HRESULT ( STDMETHODCALLTYPE *get_Next )( 
            __RPC__in IPMBackgroundServiceAgentInfoEnumerator * This,
            /* [out] */ __RPC__deref_out_opt IPMBackgroundServiceAgentInfo **ppBSAInfo);
        
        END_INTERFACE
    } IPMBackgroundServiceAgentInfoEnumeratorVtbl;

    interface IPMBackgroundServiceAgentInfoEnumerator
    {
        CONST_VTBL struct IPMBackgroundServiceAgentInfoEnumeratorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPMBackgroundServiceAgentInfoEnumerator_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPMBackgroundServiceAgentInfoEnumerator_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPMBackgroundServiceAgentInfoEnumerator_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPMBackgroundServiceAgentInfoEnumerator_get_Next(This,ppBSAInfo)	\
    ( (This)->lpVtbl -> get_Next(This,ppBSAInfo) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPMBackgroundServiceAgentInfoEnumerator_INTERFACE_DEFINED__ */


#ifndef __IPMBackgroundWorkerInfoEnumerator_INTERFACE_DEFINED__
#define __IPMBackgroundWorkerInfoEnumerator_INTERFACE_DEFINED__

/* interface IPMBackgroundWorkerInfoEnumerator */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IPMBackgroundWorkerInfoEnumerator;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("87F479F8-90D8-4EC7-92B9-72787E2F636B")
    IPMBackgroundWorkerInfoEnumerator : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE get_Next( 
            /* [out] */ __RPC__deref_out_opt IPMBackgroundWorkerInfo **ppBWInfo) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPMBackgroundWorkerInfoEnumeratorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPMBackgroundWorkerInfoEnumerator * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPMBackgroundWorkerInfoEnumerator * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPMBackgroundWorkerInfoEnumerator * This);
        
        DECLSPEC_XFGVIRT(IPMBackgroundWorkerInfoEnumerator, get_Next)
        HRESULT ( STDMETHODCALLTYPE *get_Next )( 
            __RPC__in IPMBackgroundWorkerInfoEnumerator * This,
            /* [out] */ __RPC__deref_out_opt IPMBackgroundWorkerInfo **ppBWInfo);
        
        END_INTERFACE
    } IPMBackgroundWorkerInfoEnumeratorVtbl;

    interface IPMBackgroundWorkerInfoEnumerator
    {
        CONST_VTBL struct IPMBackgroundWorkerInfoEnumeratorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPMBackgroundWorkerInfoEnumerator_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPMBackgroundWorkerInfoEnumerator_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPMBackgroundWorkerInfoEnumerator_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPMBackgroundWorkerInfoEnumerator_get_Next(This,ppBWInfo)	\
    ( (This)->lpVtbl -> get_Next(This,ppBWInfo) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPMBackgroundWorkerInfoEnumerator_INTERFACE_DEFINED__ */



#ifndef __pacmanservicelib_LIBRARY_DEFINED__
#define __pacmanservicelib_LIBRARY_DEFINED__

/* library pacmanservicelib */
/* [version][uuid] */ 


EXTERN_C const IID LIBID_pacmanservicelib;

EXTERN_C const CLSID CLSID_PMSvc;

#ifdef __cplusplus

class DECLSPEC_UUID("B9E511FC-E364-497A-A121-B7B3612CEDCE")
PMSvc;
#endif
#endif /* __pacmanservicelib_LIBRARY_DEFINED__ */

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  BSTR_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  LPSAFEARRAY_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out LPSAFEARRAY * ); 
void                      __RPC_USER  LPSAFEARRAY_UserFree(     __RPC__in unsigned long *, __RPC__in LPSAFEARRAY * ); 

unsigned long             __RPC_USER  BSTR_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree64(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  LPSAFEARRAY_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out LPSAFEARRAY * ); 
void                      __RPC_USER  LPSAFEARRAY_UserFree64(     __RPC__in unsigned long *, __RPC__in LPSAFEARRAY * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


